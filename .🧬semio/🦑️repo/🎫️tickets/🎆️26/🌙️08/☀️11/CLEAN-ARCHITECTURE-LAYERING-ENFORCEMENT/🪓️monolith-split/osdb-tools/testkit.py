"""🧰 cfg(test) support code used across crate boundaries becomes `cfg(any(test, feature = "testkit"))`.

Runs on a staged tree after `widen.py`:  testkit.py <stage db dir> <crates.json>  (edits in place; writes testkit.json beside crates.json).
"""
import collections
import json
import os
import re
import sys

from lib import *
from rs import strip, idents
from model import parse_items
from widen import Crate, refs_of, MEMBER_FN, FIELD

STOP = {'new', 'finish', 'close', 'next', 'empty', 'owner', 'seal', 'acquire', 'encode', 'decode', 'prepare_platform', 'resume', 'fault', 'request', 'cell', 'lock', 'get', 'set', 'len', 'key', 'id', 'name', 'state', 'value', 'read', 'write', 'open', 'run', 'step', 'poll', 'wake'}
CFG_TEST = re.compile(r'^(\s*)#\[cfg\(\s*test\s*\)\]\s*$')
NEW_RE = re.compile(r'^(\s*)#\[cfg\(any\(test, feature = "testkit"\)\)\]\s*$')
NEW = '#[cfg(any(test, feature = "testkit"))]'


def gated_members(x, f):
    lines = x.lines(f)
    out = []
    for i, l in enumerate(lines):
        m = CFG_TEST.match(l)
        if not m or m.group(1) != '    ':
            continue
        j = i + 1
        while j < len(lines) and lines[j].strip().startswith('#['):
            j += 1
        if j >= len(lines):
            continue
        mm = MEMBER_FN.match(lines[j])
        if mm:
            out.append((i, j, mm.group('name'), 'fn'))
            continue
        fm = FIELD.match(lines[j])
        if fm:
            out.append((i, j, fm.group('name'), 'field'))
    return out


def construct_end(st_lines, j):
    depth = 0
    for k in range(j, len(st_lines)):
        for ch in st_lines[k]:
            if ch in '([{':
                depth += 1
            elif ch in ')]}':
                depth -= 1
        if depth <= 0:
            tail = st_lines[k].rstrip()
            if tail.endswith((';', ',', '}')) or depth < 0:
                return k
    return j


def new_gated_ids(x, f):
    lines = x.lines(f)
    st = strip(x.text[f]).split('\n')
    ids = set()
    for i, l in enumerate(lines):
        if NEW_RE.match(l):
            j = i + 1
            while j < len(lines) and lines[j].strip().startswith('#['):
                j += 1
            e = construct_end(st, j)
            ids |= idents('\n'.join(st[j:e + 1]))
    return ids


def run(stage, crates_json, report):
    meta = json.load(open(crates_json))
    crates = {c['lib']: Crate(stage, c['lib'], c['root'], c['files']) for c in meta}

    def refresh(x):
        for ff in list(x.dirty):
            write(os.path.join(stage, ff), x.text[ff])
        for ff in x.files:
            x.stripped[ff] = strip(x.text[ff])
            x.parsed[ff] = parse_items(x.text[ff])
        x.dirty.clear()

    def kit_use(y):
        s = set()
        for ff in y.files:
            s |= set(re.findall(r'(?:\.|\b)([A-Za-z_][A-Za-z0-9_]*)\s*(?:\(|:)', y.stripped[ff]))
        return s

    converted = collections.defaultdict(list)
    users = collections.defaultdict(set)
    candidates = collections.defaultdict(list)
    member_conv = collections.defaultdict(set)

    def convert(x, f, attr_idx, member_idx, name, ys):
        x.set_line(f, attr_idx, re.sub(r'#\[cfg\(\s*test\s*\)\]', NEW, x.lines(f)[attr_idx]))
        if member_idx is not None:
            x.make_pub(f, member_idx)
        converted[x.lib].append(name)
        users[x.lib] |= ys

    for rounds in range(24):
        changed = False
        ku = {y.lib: kit_use(y) for y in crates.values()}
        for x in crates.values():
            f = x.root
            for it in x.items(f):
                if not it.cfg_test or it.kind in ('mod', 'use', 'impl') or not it.name:
                    continue
                real = set()
                for y in crates.values():
                    if y is x:
                        continue
                    for ff in y.files:
                        for rest in refs_of(x.lib, y, ff):
                            if rest and rest[0] == it.name:
                                real.add(y.lib)
                if not real:
                    continue
                lines = x.lines(f)
                ai = next((k for k in range(it.start, it.header + 1) if CFG_TEST.match(lines[k])), None)
                if ai is not None:
                    convert(x, f, ai, it.header, it.name, real)
                    changed = True
            for ai, mi, name, kind in gated_members(x, f):
                real = {y.lib for y in crates.values() if y is not x and name in ku[y.lib]}
                if not real:
                    continue
                candidates[x.lib].append((name, kind, sorted(real)))
                if name in STOP or not CFG_TEST.match(x.lines(f)[ai]):
                    continue
                convert(x, f, ai, mi, name, real)
                member_conv[x.lib].add(name)
                changed = True
            lines_f = x.lines(f)
            if member_conv[x.lib]:
                for i, l in enumerate(lines_f[:-1]):
                    if CFG_TEST.match(l):
                        mm = re.match(r'\s*(?:let\s+(?:mut\s+)?)?([A-Za-z_][A-Za-z0-9_]*)\s*[:,=]', lines_f[i + 1])
                        if mm and mm.group(1) in member_conv[x.lib]:
                            x.set_line(f, i, re.sub(r'#\[cfg\(\s*test\s*\)\]', NEW, l))
                            changed = True
                for it in x.items(f):
                    if it.kind == 'impl' and it.cfg_test:
                        body = '\n'.join(lines_f[it.header:it.end + 1])
                        if any(re.search(r'\bfn\s+' + re.escape(n) + r'\b', body) for n in member_conv[x.lib]):
                            for k in range(it.start, it.header + 1):
                                if CFG_TEST.match(lines_f[k]):
                                    x.set_line(f, k, re.sub(r'#\[cfg\(\s*test\s*\)\]', NEW, lines_f[k]))
                                    changed = True
            kit_ids = new_gated_ids(x, f) | set(converted[x.lib])
            for it in x.items(f):
                if it.cfg_test and it.kind not in ('mod', 'use') and (it.name or it.self_type):
                    nm = it.self_type if it.kind == 'impl' else it.name
                    if nm in kit_ids:
                        lines = x.lines(f)
                        ai = next((k for k in range(it.start, it.header + 1) if CFG_TEST.match(lines[k])), None)
                        if ai is not None:
                            convert(x, f, ai, it.header if it.kind != 'impl' else None, nm, set())
                            changed = True
            lines_f = x.lines(f)
            for i, l in enumerate(lines_f[:-1]):
                if CFG_TEST.match(l) and re.match(r'use\b', lines_f[i + 1]):
                    leaf = set(re.findall(r'[A-Za-z_][A-Za-z0-9_]*', lines_f[i + 1]))
                    if leaf & kit_ids:
                        x.set_line(f, i, re.sub(r'#\[cfg\(\s*test\s*\)\]', NEW, l))
                        changed = True
        for x in crates.values():
            refresh(x)
        if not changed:
            break
    report['testkit'] = {k: sorted(set(v)) for k, v in converted.items()}
    report['testkit_users'] = {k: sorted(v) for k, v in users.items()}
    report['member_candidates'] = {k: v for k, v in candidates.items()}
    return report


if __name__ == '__main__':
    rep = {}
    run(sys.argv[1], sys.argv[2], rep)
    json.dump(rep, open(os.path.dirname(os.path.abspath(sys.argv[2])) + '/testkit.json', 'w'), ensure_ascii=False, indent=1)
    print('converted', {k: len(v) for k, v in rep['testkit'].items() if v})
    print('member candidates', {k: len(set(n for n, _, _ in v)) for k, v in rep['member_candidates'].items() if v})
