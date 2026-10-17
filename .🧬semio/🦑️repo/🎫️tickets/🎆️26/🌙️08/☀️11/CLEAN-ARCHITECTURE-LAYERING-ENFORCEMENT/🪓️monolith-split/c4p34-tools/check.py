import importlib, os, re, sys, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import c4p34 as P
tags = sys.argv[1:] or 'png jpg avi pptx dxfbin dxfschema zlib'.split()
V = '️'
ok = True
maxlines = 0
maxpath = 0
maxabs = 0
total_children = 0
RG = re.compile(r'^\s*//\s?#(end)?region\b', re.M)


def read(p):
    return open(p, encoding='utf-8').read()


def balance(text, what):
    o, cl = len(re.findall(r'^\s*//\s?#region\b', text, re.M)), len(re.findall(r'^\s*//\s?#endregion\b', text, re.M))
    if o != cl:
        print('REGION IMBALANCE', what, o, cl)
        return False
    return True


def emojis_of(line):
    return {c for c in line if ord(c) > 0x2000 and c != V and c != '‍'}


def layer(t, parent_path, base_rel, kids, root_line):
    global ok, maxlines, maxpath, maxabs, total_children
    ptxt = read(parent_path)
    mounts = re.findall(r'#\[path = "([^"]+)"\]\s*(?:#\[cfg[^\]]*\]\s*)?(?:pub(?:\([^)]*\))? )?mod (\w+);', ptxt)
    got = collections.Counter(m[0] for m in mounts)
    seen = set()
    root_em = emojis_of(root_line)
    for k in kids:
        folder, mod, doc_emoji_expected = k
        rel = folder + '/🦀️.rs'
        if got.get(rel, 0) != 1:
            print(t, 'MOUNT COUNT', rel, got.get(rel, 0)); ok = False
        fp = os.path.join(os.path.dirname(parent_path), rel)
        if not os.path.isfile(fp):
            print(t, 'MISSING FILE', fp); ok = False; continue
        total_children += 1
        text = read(fp)
        m = re.match(r'^//! (\S+) ', text.split('\n')[0])
        if not m:
            print(t, 'NO DOC', folder); ok = False; continue
        e = m.group(1)
        if e != folder.split(V)[0] + V:
            print(t, 'DOC EMOJI != FOLDER', folder, e); ok = False
        if e in seen:
            print(t, 'DUP DOC EMOJI', e); ok = False
        seen.add(e)
        if e.replace(V, '') in {x for x in root_em}:
            print(t, 'CHILD EMOJI == ROOT EMOJI', e); ok = False
        ok &= balance(text, folder)
        n = text.count('\n'); maxlines = max(maxlines, n)
        r = os.path.relpath(fp, P.REPO)
        maxpath = max(maxpath, len(r.encode('utf-8')))
        maxabs = max(maxabs, len(P.REPO + '/' + r))
        if '[DEBUG]' in text:
            print(t, 'DEBUG LOG', folder); ok = False
        if not re.search(r'^(?:use super::\*;|use )', text, re.M) and 'use super' not in text:
            pass
    ok &= balance(ptxt, parent_path)
    if '[DEBUG]' in ptxt:
        print(t, 'DEBUG LOG in parent'); ok = False
    print(t, os.path.basename(os.path.dirname(parent_path)), 'lines', ptxt.count('\n'), 'children', len(kids))


for t in tags:
    sp = importlib.import_module('specs.' + t).SPEC
    live = os.path.dirname(os.path.join(P.REPO, sp['file']))
    outer = os.path.join(P.REPO, sp['file'])
    root_line = read(outer).split('\n')[0]
    inn = sp.get('inner')
    kids = [(c['folder'], c['mod'], None) for c in sp['children']]
    if inn:
        kids.append((inn['folder'], inn['mod'], None))
    layer(t, outer, '', kids, root_line)
    if inn:
        shell = os.path.join(live, inn['folder'], '🦀️.rs')
        layer(t, shell, inn['folder'], [(c['folder'], c['mod'], None) for c in inn['children']], read(shell).split('\n')[0])
print('children', total_children, 'longest child lines', maxlines, 'longest repo-relative path bytes', maxpath, 'longest absolute path chars', maxabs)
print('CHECKS OK' if ok else 'CHECKS FAILED')
sys.exit(0 if ok else 1)
