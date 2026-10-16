"""📜 Source-text laws follow the code they scan: a test that asserts on the text of a carved source file is homed in the crate that owns that text.

srcfix.py <report|apply>   (operates on the staged tree; run after testkit.py and before finalize.py)

Every scan test (an `include_str!` of a Rust source file, directly or through the `retained_submit_source` helper) is analysed against the
ORIGINAL source: its positive string-literal assertions that held in the original file must hold in the file the test now embeds. When they
moved, the test is rewritten to embed the single carved file that holds them and is homed in that file's crate. Tests that need more than
one carved file are listed for the explicit SPLITS table.
"""
import os
import re
import sys
import json

from lib import *
from model import parse_items
from rs import strip

MODE = sys.argv[1] if len(sys.argv) > 1 else 'report'
INC = re.compile(r'include_str!\(\s*"([^"]*🦀️\.rs)"\s*\)')
HELPER = 'retained_submit_source'
UNIT = '🧪️tests/🔬️unit/🦀️.rs'

crates = json.load(open(SP + '/crates.json'))
LIB_OF = {f: c['lib'] for c in crates for f in c['files']}
ROOT_OF = {c['lib']: c['root'] for c in crates}


def collect(base, only_rs=True):
    out = {}
    for root, ds, fs in os.walk(base):
        for f in fs:
            if f.endswith('.rs'):
                p = os.path.join(root, f)
                out[os.path.relpath(p, base)] = read(p)
    return out


def unesc(s):
    try:
        return bytes(s, 'utf-8').decode('unicode_escape').encode('latin-1').decode('utf-8')
    except Exception:
        return s


def literals(body):
    """String literals of a test body with their polarity: False when asserted ABSENT (`assert!(!x.contains(..))`, directly or through a `for` list)."""
    for_lists = []
    for m in re.finditer(r'for\s+\w+\s+in\s+\[', body):
        depth, i = 1, m.end()
        while i < len(body) and depth:
            depth += {'[': 1, ']': -1}.get(body[i], 0)
            i += 1
        after = body[i:i + 400]
        k = after.find('assert')
        neg = k >= 0 and after[k:].startswith(('assert!(!', 'assert!( !'))
        for_lists.append((m.end(), i, neg))
    out = []
    for m in re.finditer(r'"((?:[^"\\]|\\.)*)"', body):
        raw = m.group(1)
        s = unesc(raw)
        if len(s) <= 5 or s.startswith('../'):
            continue
        neg = None
        for a, b, n in for_lists:
            if a <= m.start() < b:
                neg = n
        if neg is None:
            before = body[max(0, m.start() - 160):m.start()]
            k = max(before.rfind('assert!('), before.rfind('assert_eq!('))
            neg = k >= 0 and before[k:].startswith('assert!(!')
        out.append((m.start(1), m.end(1), raw, s, not neg))
    return out


def scan_tests(files):
    """(file, item, lines, includes) for every fn that embeds a Rust source."""
    found = []
    for rel, text in files.items():
        if '🧪️tests/' not in rel or ('include_str!' not in text and HELPER not in text):
            continue
        items, trail, lines = parse_items(text)
        helper_target = None
        for it in items:
            if it.kind == 'fn' and it.name == HELPER:
                body = '\n'.join(lines[it.start:it.end + 1])
                m = INC.search(body)
                helper_target = m.group(1) if m else None
        for it in items:
            if it.kind != 'fn' or not it.name or it.name == HELPER:
                continue
            body = '\n'.join(lines[it.start:it.end + 1])
            incs = INC.findall(body)
            uses_helper = HELPER + '()' in body and helper_target
            if not incs and not uses_helper:
                continue
            targets = [os.path.normpath(os.path.join(os.path.dirname(rel), i)) for i in incs]
            if uses_helper:
                targets.append(os.path.normpath(os.path.join(os.path.dirname(rel), helper_target)))
            found.append(dict(file=rel, item=it, body=body, targets=targets, uses_helper=bool(uses_helper)))
    return found


def fix_literal(raw, pool, libs):
    """The original asserted text, rewritten the way the carve rewrote the code it quotes."""
    cands = [raw.replace('pub(crate) ', 'pub ')]
    for variant in list(cands):
        for lib in libs:
            cands.append(re.sub(r'(?<![A-Za-z0-9_])db_[a-z0-9_]+::', lib + '::', variant))
    for cand in cands:
        s = unesc(cand)
        if cand != raw and any(s in t for t in pool.values()):
            return cand
    return None


def analyse(stage, prist):
    pool = {k: t for k, t in stage.items() if '🧪️tests/' not in k}
    libs = sorted({c['lib'] for c in crates})
    results = []
    for t in scan_tests(stage):
        pu = '\n'.join(prist.get(x, '') for x in t['targets'])
        rewrites = {}
        lits = []
        for s, e, raw, text, positive in literals(t['body']):
            if not positive or text not in pu:
                continue
            if not any(text in v for v in pool.values()):
                fixed = fix_literal(raw, pool, libs)
                if fixed is not None:
                    rewrites[raw] = fixed
                    raw, text = fixed, unesc(fixed)
            lits.append((raw, text))
        missing = [text for raw, text in lits if not any(text in v for v in pool.values())]
        remaining = {text for raw, text in lits if any(text in v for v in pool.values())}
        own = LIB_OF.get(t['file'])
        cover = []
        while remaining:
            best = max(pool, key=lambda k: (sum(1 for l in remaining if l in pool[k]), k in t['targets'], LIB_OF.get(k) == own, -len(pool[k])))
            got = {l for l in remaining if l in pool[best]}
            if not got:
                break
            cover.append(best)
            remaining -= got
        results.append(dict(t, lits=lits, cover=cover, vanished=missing, rewrites=rewrites))
    return results


def host_of(path):
    return LIB_OF.get(path)


def classify(results):
    plan = []
    for r in results:
        k = LIB_OF.get(r['file'])
        cover = r['cover']
        if not cover:
            plan.append((r, 'keep', None))
        elif len(cover) == 1:
            only = cover[0]
            same = LIB_OF.get(only) == k and cover == r['targets'] and not r['rewrites']
            plan.append((r, 'keep' if same else 'host', only))
        else:
            plan.append((r, 'manual', None))
    return plan


PURE_WORDS = set(
    'let source engine artifact wal assert contains find unwrap include_str retained_submit_source fn for in required forbidden if return false true '
    'process_isolated_law db_io_driver tests str expect rfind rsplit_once split_once lines collect trim_start strip_prefix or_else and_then continue Some else '
    'as usize i64 mut matches Vec name index line depth body trimmed scrutinee rest ends_with starts_with split next marker production declarations replay '
    'history terminal close region runner panicked panic_close preflight publish stale handoff released result_drop missing format test frame probe retained future_drop'.split())
UNIT_BLOCK = '//#region 🔖️Source laws\n{}\n//#endregion 🔖️Source laws\n'


def is_pure(r):
    ids = idents(strip(r['body']))
    return not {i for i in ids if i not in PURE_WORDS and i != r['item'].name and not i.startswith('_')}


def unit_file(lib):
    cands = [f for f in next(c['files'] for c in crates if c['lib'] == lib) if f.endswith(UNIT)]
    if len(cands) != 1:
        raise SystemExit(f'{lib}: expected one unit test file, got {cands}')
    return cands[0]


def retarget(body, from_file, target):
    new = f'include_str!("{os.path.relpath(target, os.path.dirname(from_file))}")'
    return INC.sub(lambda m: new, body).replace(HELPER + '()', new)


def literal_fix(body, target_text):
    for s_, e_, raw, text, positive in sorted(literals(body), reverse=True):
        if positive and text not in target_text:
            base = raw.replace('pub(crate) ', 'pub ')
            cands = [base] + [re.sub(r'(?<![A-Za-z0-9_])db_[a-z0-9_]+::', lib + '::', base) for lib in sorted({c['lib'] for c in crates})]
            for cand in cands:
                if cand != raw and unesc(cand) in target_text:
                    body = body[:s_] + cand + body[e_:]
                    break
    return body


def target_text(files, lib):
    return files[ROOT_OF[lib]]


def apply(files, results):
    from srcfix_splits import splits
    plan = classify(results)
    bodies = {(LIB_OF[r['file']], r['item'].name): r['body'] for r, _, _ in plan}
    manual = {(LIB_OF[r['file']], r['item'].name) for r, a, _ in plan if a == 'manual'}
    sp = splits(bodies)
    handled = set(sp['replace'])
    missing = manual - handled
    if missing:
        raise SystemExit(f'unhandled scan tests: {sorted(missing)}')
    deletes, replaces, appends = {}, {}, {}
    for r, action, only in plan:
        lib, name, f, it = LIB_OF[r['file']], r['item'].name, r['file'], r['item']
        key = (lib, name)
        if key in sp['replace']:
            new = sp['replace'][key]
            if new is None:
                deletes.setdefault(f, []).append((it.start, it.end))
            else:
                replaces.setdefault(f, []).append((it.start, it.end, literal_fix(new, files[ROOT_OF[lib]])))
            continue
        if action == 'keep':
            continue
        host = LIB_OF[only]
        new = literal_fix(retarget(r['body'], unit_file(host), only), files[only])
        if host != lib and is_pure(r):
            deletes.setdefault(f, []).append((it.start, it.end))
            appends.setdefault(unit_file(host), []).append(new)
        else:
            replaces.setdefault(f, []).append((it.start, it.end, literal_fix(retarget(r['body'], f, only), files[only])))
    for host, text in sp['add']:
        appends.setdefault(unit_file(host), []).append(literal_fix(text, files[ROOT_OF[host]]))
    for f in set(deletes) | set(replaces) | set(appends):
        lines = files[f].split('\n')
        edits = [(s, e, None) for s, e in deletes.get(f, [])] + [(s, e, t) for s, e, t in replaces.get(f, [])]
        for s, e, t in sorted(edits, key=lambda x: -x[0]):
            if t is None:
                tail = e + 2 if e + 1 < len(lines) and lines[e + 1].strip() == '' else e + 1
                del lines[s:tail]
            else:
                lines[s:e + 1] = t.split('\n')
        text = '\n'.join(lines)
        if f in appends:
            names = [re.search(r'fn (\w+)', t).group(1) for t in appends[f]]
            for n in names:
                if re.search(r'\bfn ' + n + r'\b', text):
                    raise SystemExit(f'{f}: would add a second {n}')
            text = text.rstrip('\n') + '\n\n' + UNIT_BLOCK.format('\n\n'.join(appends[f]))
        files[f] = text
    removed = 0
    for f, text in list(files.items()):
        if '🧪️tests/' in f and HELPER in text:
            items, trail, lines = parse_items(text)
            body_wo = text
            users = len(re.findall(r'\b' + HELPER + r'\(\)', text)) - 1
            helper = next((i for i in items if i.kind == 'fn' and i.name == HELPER), None)
            if helper and users == 0:
                end = helper.end + 2 if helper.end + 1 < len(lines) and lines[helper.end + 1].strip() == '' else helper.end + 1
                del lines[helper.start:end]
                files[f] = '\n'.join(lines)
                removed += 1
    return files, dict(deleted=sum(len(v) for v in deletes.values()), replaced=sum(len(v) for v in replaces.values()), appended=sum(len(v) for v in appends.values()), helpers_removed=removed)


def main():
    stage = collect(STAGE)
    prist = collect(PRISTINE)
    results = analyse(stage, prist)
    plan = classify(results)
    for r, action, only in plan:
        lib = LIB_OF.get(r['file'])
        if action == 'keep':
            continue
        print(f"{action:7} {lib:30} {r['item'].name[:80]:80} -> {host_of(only) if only else r['cover']} {('VANISHED ' + str(r['vanished'])[:120]) if r['vanished'] else ''}")
    if MODE == 'report':
        return
    before = dict(stage)
    files, stat = apply(stage, results)
    for f, t in files.items():
        if t != before[f]:
            write(os.path.join(STAGE, f), t)
    print('srcfix', stat)


if __name__ == '__main__':
    main()
