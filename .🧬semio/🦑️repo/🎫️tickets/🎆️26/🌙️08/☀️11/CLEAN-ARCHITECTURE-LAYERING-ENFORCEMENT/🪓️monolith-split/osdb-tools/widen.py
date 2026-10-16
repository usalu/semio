"""🔓 Visibility widening and cfg(test) exposure across the crate boundaries (sound over-approximation by identifier use).

Runs on a staged tree:  widen.py <stage db dir> <crates.json>   (edits the stage in place, prints a report).
"""
import collections
import json
import os
import re
import sys

from lib import *
from model import parse_items
from rs import strip, idents
from rewrite import find_uses

MEMBER_FN = re.compile(r'^(?P<ind>\s{4})(?P<vis>pub(?:\([^)]*\))?\s+)?(?P<mods>(?:default\s+|const\s+|async\s+|unsafe\s+)*)(?P<kind>fn|const|type)\s+(?P<name>[A-Za-z_][A-Za-z0-9_]*)')
FIELD = re.compile(r'^(?P<ind>\s{4})(?P<vis>pub(?:\([^)]*\))?\s+)?(?P<name>[A-Za-z_][A-Za-z0-9_]*)\s*:\s')
HEAD_VIS = re.compile(r'^(?P<ind>\s*)(?P<vis>pub(?:\([^)]*\))?\s+)?')


class Crate:
    def __init__(self, stage, lib, root, files):
        self.lib = lib
        self.root = root
        self.files = files
        self.stage = stage
        self.text = {f: read(os.path.join(stage, f)) for f in files}
        self.dirty = set()
        self.parsed = {f: parse_items(self.text[f]) for f in files}
        self.stripped = {f: strip(self.text[f]) for f in files}

    def lines(self, f):
        return self.text[f].split('\n')

    def items(self, f):
        return self.parsed[f][0]

    def mounts(self, f):
        out = {}
        for m in re.finditer(r'#\[path = "([^"]+)"\]\s*(?:pub(?:\([^)]*\))?\s+)?mod\s+([A-Za-z_][A-Za-z0-9_]*)\s*;', self.text[f]):
            p = os.path.normpath(os.path.join(os.path.dirname(os.path.join(self.stage, f)), m.group(1)))
            rel = os.path.relpath(p, self.stage)
            if rel in self.text:
                out[m.group(2)] = rel
        return out

    def set_line(self, f, idx, new):
        ls = self.lines(f)
        if ls[idx] != new:
            ls[idx] = new
            self.text[f] = '\n'.join(ls)
            self.dirty.add(f)

    def make_pub(self, f, idx):
        ls = self.lines(f)
        if re.match(r'\s*(?:std::)?thread_local!\s*\{', ls[idx]):
            for k in range(idx, min(idx + 6, len(ls))):
                mm = re.match(r'(\s*)(?:pub(?:\([^)]*\))?\s+)?static\s', ls[k])
                if mm:
                    if 'pub ' in ls[k][:mm.end()]:
                        return False
                    self.set_line(f, k, mm.group(1) + 'pub ' + ls[k][len(mm.group(1)):].lstrip() if not ls[k].lstrip().startswith('pub') else re.sub(r'pub(?:\([^)]*\))?\s+', 'pub ', ls[k], 1))
                    return True
            return False
        m = HEAD_VIS.match(ls[idx])
        if (m.group('vis') or '').strip() == 'pub':
            return False
        self.set_line(f, idx, m.group('ind') + 'pub ' + ls[idx][m.end():])
        return True


def refs_of(x_lib, y, f):
    """Rest paths referring into crate x_lib in file f of crate y."""
    out = set()
    text = y.text[f]
    st = y.stripped[f]
    if x_lib not in st:
        return out
    uses = find_uses(text)
    for u in uses:
        for path, alias, glob in parse_use(u['tree']):
            if path and path[0] == x_lib:
                out.add(tuple(path[1:]) + (('*',) if glob else ()))
    spans = [(u['start'], u['end']) for u in uses]
    for m in re.finditer(r'(?<![A-Za-z0-9_:])' + re.escape(x_lib) + r'((?:::[A-Za-z_][A-Za-z0-9_]*)+)', st):
        if any(a <= m.start() < b for a, b in spans):
            continue
        out.add(tuple(m.group(1).split('::')[1:]))
    return out


def run(stage, crates_json, report):
    meta = json.load(open(crates_json))
    crates = {c['lib']: Crate(stage, c['lib'], c['root'], c['files']) for c in meta}
    # who uses what
    users = collections.defaultdict(lambda: collections.defaultdict(set))
    for y in crates.values():
        for f in y.files:
            for x in crates.values():
                if x is y:
                    continue
                for rest in refs_of(x.lib, y, f):
                    users[x.lib][rest].add(y.lib)
    ident_sets = {y.lib: set().union(*[idents(y.stripped[f]) for f in y.files]) for y in crates.values()}
    ident_nd = {}
    changed = collections.defaultdict(list)
    type_users = collections.defaultdict(set)
    for x in crates.values():
        for rest, ys in users[x.lib].items():
            if not rest:
                continue
            f = x.root
            for seg in rest:
                if seg == '*':
                    break
                found = None
                for i, it in enumerate(x.items(f)):
                    nm = it.name
                    if it.kind == 'use':
                        mm = re.search(r'(?:as\s+)?([A-Za-z_][A-Za-z0-9_]*)\s*[;}]?\s*$', nm)
                    if nm == seg and it.kind not in ('impl', 'use'):
                        found = (i, it)
                        break
                if found is None:
                    break
                i, it = found
                if it.vis != 'pub':
                    if x.make_pub(f, it.header):
                        changed[x.lib].append(f'{seg}')
                if it.kind == 'mod':
                    tgt = x.mounts(f).get(seg)
                    if tgt is None:
                        break
                    f = tgt
                    continue
                if it.kind in ('struct', 'enum', 'union', 'trait'):
                    type_users[(x.lib, f, seg)] |= ys
                break
    # members of exported types
    for (xl, f, tname), ys in type_users.items():
        x = crates[xl]
        ytext = set()
        for yl in ys:
            ytext |= ident_sets[yl]
        items = x.items(f)
        lines = x.lines(f)
        for it in items:
            if it.kind == 'struct' and it.name == tname:
                start = it.header
                while start < it.end and '{' not in lines[start]:
                    start += 1
                body_lines = range(start + 1, it.end)
                tuple_struct = re.match(r'\s*(?:pub(?:\([^)]*\))?\s+)?struct\s+' + re.escape(tname) + r'(?:<[^(){}]*>)?\s*\(', lines[it.header]) is not None
                for ln in body_lines:
                    m = FIELD.match(lines[ln])
                    if m and (m.group('vis') or '').strip() != 'pub' and m.group('name') in ytext:
                        if x.make_pub(f, ln):
                            changed[xl].append(f'{tname}.{m.group("name")}')
                if tuple_struct and not it.vis.startswith('pub('):
                    hdr = lines[it.header]
                    new = re.sub(r'(struct\s+' + re.escape(tname) + r'(?:<[^(){}]*>)?\s*)\(([^()]*)\)', lambda mm: mm.group(1) + '(' + ', '.join(('pub ' + p.strip()) if not p.strip().startswith('pub') else p.strip() for p in mm.group(2).split(',') if p.strip()) + ')', hdr, count=1)
                    if new != hdr and re.search(r'\b' + re.escape(tname) + r'\s*\(', ' '.join(y_text for y_text in [x.stripped.get(f, '')])) is not None:
                        x.set_line(f, it.header, new)
                        changed[xl].append(f'{tname}.0')
            if it.kind == 'impl' and it.self_type == tname and not it.trait:
                for ln in range(it.header + 1, it.end):
                    m = MEMBER_FN.match(lines[ln])
                    if m and (m.group('vis') or '').strip() != 'pub' and m.group('name') in ytext:
                        if x.make_pub(f, ln):
                            changed[xl].append(f'{tname}::{m.group("name")}')
    from plan import CHUNK_LIBS
    family_of = {lib: m for m, libs in CHUNK_LIBS.items() for lib in libs}
    for x in crates.values():
        f_all = x.files
        fam = family_of.get(x.lib)
        other_ids = set()
        other_fam_ids = set()
        for y in crates.values():
            if y is x:
                continue
            other_ids |= ident_sets[y.lib]
            if fam and family_of.get(y.lib) == fam:
                other_fam_ids |= ident_sets[y.lib]
        for f in f_all:
            lines = x.lines(f)
            for it in x.items(f):
                if it.kind == 'impl' and not it.trait:
                    for ln in range(it.header + 1, it.end):
                        m = MEMBER_FN.match(lines[ln])
                        if not m:
                            continue
                        vis = (m.group('vis') or '').strip()
                        if vis == 'pub':
                            continue
                        restricted = vis.startswith('pub(')
                        if (restricted and m.group('name') in other_ids) or (fam and m.group('name') in other_fam_ids):
                            if x.make_pub(f, ln):
                                changed[x.lib].append(f'{it.self_type}::{m.group("name")}')
                elif it.kind == 'struct':
                    start = it.header
                    while start < it.end and '{' not in lines[start]:
                        start += 1
                    for ln in range(start + 1, it.end):
                        m = FIELD.match(lines[ln])
                        if not m:
                            continue
                        vis = (m.group('vis') or '').strip()
                        if vis == 'pub':
                            continue
                        restricted = vis.startswith('pub(')
                        if (restricted and m.group('name') in other_ids) or (fam and m.group('name') in other_fam_ids):
                            if x.make_pub(f, ln):
                                changed[x.lib].append(f'{it.name}.{m.group("name")}')
    for x in crates.values():
        for f in x.dirty:
            write(os.path.join(stage, f), x.text[f])
    report['widened'] = {k: len(v) for k, v in changed.items()}
    report['widened_names'] = {k: v for k, v in changed.items()}
    return crates, users


if __name__ == '__main__':
    stage, cj = sys.argv[1], sys.argv[2]
    rep = {}
    run(stage, cj, rep)
    print(json.dumps(rep['widened']))
