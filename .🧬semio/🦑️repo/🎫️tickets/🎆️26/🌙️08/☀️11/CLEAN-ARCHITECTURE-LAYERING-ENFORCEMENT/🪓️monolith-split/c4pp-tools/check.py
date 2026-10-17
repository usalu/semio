import importlib, os, re, sys, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import c4pp as P
tags = sys.argv[1:] or 'sfm sfmunit surface mesh repair editor session algebra'.split()
ok = True
maxlines = 0
maxpath = 0
total_children = 0
RG = re.compile(r'^\s*//\s?#(end)?region\b', re.M)
for t in tags:
    sp = importlib.import_module('specs.' + t).SPEC
    live = os.path.join(P.REPO, os.path.dirname(sp['file']))
    parent = open(os.path.join(P.REPO, sp['file']), encoding='utf-8').read()
    mounts = re.findall(r'#\[path = "([^"]+)"\]\s*(?:#\[cfg[^\]]*\]\s*)?(?:pub(?:\([^)]*\))? )?mod (\w+);', parent)
    want = {c['folder'] + '/🦀️.rs': c['mod'] for c in sp['children']}
    got = collections.Counter(m[0] for m in mounts)
    for rel, mod in want.items():
        if got.get(rel, 0) != 1:
            print(t, 'MOUNT COUNT', rel, got.get(rel, 0)); ok = False
        if not os.path.isfile(os.path.join(live, rel)):
            print(t, 'MISSING FILE', rel); ok = False
    root_doc = re.search(r'^//! (\S+)', open(os.path.join(P.REPO, sp['file']), encoding='utf-8').read(), re.M)
    root_emoji = root_doc.group(1).replace('️', '') if root_doc else None
    seen = set()
    for c in sp['children']:
        total_children += 1
        text = open(os.path.join(live, c['folder'], '🦀️.rs'), encoding='utf-8').read()
        first = text.split('\n')[0]
        m = re.match(r'^//! (\S+) ', first)
        if not m: print(t, 'NO DOC', c['folder']); ok = False; continue
        e = m.group(1)
        if e != c['folder'].split('️')[0] + '️': print(t, 'DOC EMOJI != FOLDER', c['folder'], e); ok = False
        if e in seen: print(t, 'DUP DOC EMOJI', e); ok = False
        seen.add(e)
        if root_emoji and e.replace('️', '') == root_emoji: print(t, 'CHILD EMOJI == ROOT EMOJI', e); ok = False
        o, cl = len(re.findall(r'^\s*//\s?#region\b', text, re.M)), len(re.findall(r'^\s*//\s?#endregion\b', text, re.M))
        if o != cl: print(t, 'REGION IMBALANCE', c['folder'], o, cl); ok = False
        n = text.count('\n'); maxlines = max(maxlines, n)
        rel = os.path.relpath(os.path.join(live, c['folder'], '🦀️.rs'), P.REPO)
        maxpath = max(maxpath, len(rel.encode('utf-8')))
        if '[DEBUG]' in text: print(t, 'DEBUG LOG', c['folder']); ok = False
    ptxt = open(os.path.join(P.REPO, sp['file']), encoding='utf-8').read()
    if '[DEBUG]' in ptxt: print(t, 'DEBUG LOG in parent'); ok = False
    o, cl = len(re.findall(r'^\s*//\s?#region\b', ptxt, re.M)), len(re.findall(r'^\s*//\s?#endregion\b', ptxt, re.M))
    if o != cl: print(t, 'REGION IMBALANCE parent', o, cl); ok = False
    print(t, 'parent lines', ptxt.count('\n'), 'children', len(sp['children']))
print('children', total_children, 'longest child lines', maxlines, 'longest repo-relative path bytes', maxpath)
print('CHECKS OK' if ok else 'CHECKS FAILED')
sys.exit(0 if ok else 1)
