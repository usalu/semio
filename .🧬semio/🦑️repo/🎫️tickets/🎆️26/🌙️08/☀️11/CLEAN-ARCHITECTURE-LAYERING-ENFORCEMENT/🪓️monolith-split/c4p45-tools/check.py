"""✅ c4p45 structure checks over the live tree (or a stage root): mounts, docstring emoji (unique among siblings and the root, folder emoji, already used in the repo, not an existing sibling folder), region balance per file, test-name multiset, DEBUG counts, path lengths, sizes.
usage: check35.py [live|stage] [tag...]"""
import collections, importlib, os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import c4p45 as P

mode = sys.argv[1] if len(sys.argv) > 1 else 'live'
tags = sys.argv[2:] or 'pix gdt rnd dfl drv sch pnt'.split()
USED = {l.split('\t')[0] for l in open(os.path.join(P.SCR, 'folder-emojis.txt'), encoding='utf-8').read().split('\n') if l}
FE = '️'
RG = re.compile(r'^\s*//\s?#(end)?region\b', re.M)
TEST = re.compile(r'#\[test\]((?:\s*#\[[^\]]*(?:\][^\]]*)*?\])*)\s*(?:pub\s+)?(?:async\s+)?fn\s+(\w+)')
ok = True
maxlines = maxpath = nchildren = 0


def bad(*a):
    global ok
    ok = False
    print('  FAIL', *a)


def rd(p):
    return open(p, encoding='utf-8').read()


def regions_balanced(text):
    depth = 0
    for l in text.split('\n'):
        m = RG.match(l)
        if not m:
            continue
        depth += -1 if m.group(1) else 1
        if depth < 0:
            return False
    return depth == 0


for t in tags:
    sp = importlib.import_module('specs.' + t).SPEC
    base = os.path.join(P.REPO, os.path.dirname(sp['file'])) if mode == 'live' else os.path.join(P.SCR, 'stage', t)
    root_p = os.path.join(base, '🦀️.rs')
    root = rd(root_p)
    orig = rd(os.path.join(P.T, 'orig', sp['orig']))
    folders = P.rel_folders(sp)
    kids = {f: rd(os.path.join(base, f, '🦀️.rs')) for f in folders}
    nchildren += len(kids)
    print(t, 'root lines', root.count('\n'), 'children', {f[:2]: kids[f].count('\n') for f in folders})
    mounts = re.findall(r'#\[path\s*=\s*"([^"]+)"\]\s*(?:#\[cfg[^\]]*\]\s*)?(?:pub(?:\([^)]*\))? )?mod (\w+);|include!\("([^"]+)"\);', root)
    got = collections.Counter(a or b for a, _, b in [(m[0], m[1], m[2]) for m in mounts])
    for f in folders:
        if got.get(f + '/🦀️.rs', 0) != 1:
            bad(t, 'mount count', f, got.get(f + '/🦀️.rs', 0))
    rootdoc = re.match(r'^(?://!|///) (\S+)', root)
    root_emoji = rootdoc.group(1).replace(FE, '') if rootdoc else None
    siblings = set()
    if mode == 'live':
        for e in os.listdir(base):
            if e not in folders and os.path.isdir(os.path.join(base, e)):
                siblings.add(e.split(FE)[0] if FE in e else e[:1])
    seen = set()
    for f in folders:
        text = kids[f]
        first = text.split('\n')[0]
        m = re.match(r'^(?://!|///) (\S+)', first)
        if not m:
            bad(t, 'first line is no doc', f)
            continue
        e = m.group(1)
        folder_emoji = f.split(FE)[0]
        if e.replace(FE, '') != folder_emoji:
            bad(t, 'doc emoji != folder emoji', f, e)
        if e in seen:
            bad(t, 'duplicate doc emoji', e)
        seen.add(e)
        if root_emoji and e.replace(FE, '') == root_emoji:
            bad(t, 'child emoji == root emoji', e)
        if folder_emoji in siblings:
            bad(t, 'folder emoji clashes with an existing sibling folder', f)
        if folder_emoji + FE not in USED:
            bad(t, 'folder emoji is not used as a folder prefix anywhere in the repo', f)
        if not regions_balanced(text):
            bad(t, 'region markers unbalanced', f)
        n = text.count('\n')
        maxlines = max(maxlines, n)
        rel = os.path.relpath(os.path.join(base, f, '🦀️.rs'), P.REPO)
        maxpath = max(maxpath, len(rel.encode('utf-8')))
        if len(rel.encode('utf-8')) > 240 or len(os.path.join(P.REPO, rel)) > 256:
            bad(t, 'path too long', rel)
        if '[DEBUG]' in text and not '[DEBUG]' in orig:
            bad(t, 'new DEBUG log', f)
    if not regions_balanced(root):
        bad(t, 'region markers unbalanced in root')
    dbg_o, dbg_n = orig.count('[DEBUG]'), root.count('[DEBUG]') + sum(k.count('[DEBUG]') for k in kids.values())
    if dbg_o != dbg_n:
        bad(t, 'DEBUG count', dbg_o, dbg_n)
    a = sorted(m.group(2) for m in TEST.finditer(orig))
    b = sorted(m.group(2) for m in TEST.finditer(root + ''.join(kids.values())))
    if a != b:
        bad(t, 'test fn names differ', set(a) ^ set(b))
    ro, rc = len(RG.findall(root)), 0
    print('  test fns', len(a), '=', len(b), '| markers orig', len(RG.findall(orig)), 'new', len(RG.findall(root)) + sum(len(RG.findall(k)) for k in kids.values()))
print('children', nchildren, 'longest child lines', maxlines, 'longest repo-relative path bytes', maxpath)
print('CHECKS OK' if ok else 'CHECKS FAILED')
sys.exit(0 if ok else 1)
