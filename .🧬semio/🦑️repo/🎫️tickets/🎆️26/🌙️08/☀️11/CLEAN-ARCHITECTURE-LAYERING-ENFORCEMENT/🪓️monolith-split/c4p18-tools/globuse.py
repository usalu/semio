import os, re, sys, importlib
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import c4p18 as P
from rsitems import items, name_kind

def imported_names(text):
    names = set()
    for it in items(text):
        v, k, n = name_kind(it['toks'])
        if k != 'use':
            continue
        src = '\n'.join(text.split('\n')[it['first_tok_line']:it['end_line'] + 1])
        src = re.sub(r'//[^\n]*', '', src)
        for m in re.finditer(r'\bas\s+([A-Za-z_]\w*)', src):
            names.add(m.group(1))
        body = re.sub(r'\bas\s+\w+', '', src)
        for m in re.finditer(r'([A-Za-z_]\w*)\s*(?=[,};])', body):
            names.add(m.group(1))
    return names

for tag in sys.argv[1:]:
    sp = importlib.import_module('specs.' + tag).SPEC
    root = os.path.join(P.SCR, 'stage', tag)
    parent = open(os.path.join(root, '🦀️.rs'), encoding='utf-8').read()
    texts = {c['folder']: open(os.path.join(root, c['folder'], '🦀️.rs'), encoding='utf-8').read() for c in sp['children']}
    defined = {}
    for f, t in [('__parent__', parent)] + list(texts.items()):
        for it in items(t):
            v, k, n = name_kind(it['toks'])
            if n and k not in ('use', 'impl', 'mod'):
                defined.setdefault(n, f)
            if k == 'mod' and n:
                defined.setdefault(n, f)
    imp = imported_names(parent)
    for f, t in texts.items():
        ids = P.ident_set(t)
        own = set()
        for it in items(t):
            v, k, n = name_kind(it['toks'])
            if n:
                own.add(n)
        from_parent = sorted(i for i in ids if i in imp and i not in own)
        from_other = sorted(i for i in ids if i in defined and defined[i] != f and i not in own)
        print(tag, f, 'parent-imports:', len(from_parent), 'other-defs:', len(from_other), '' if (from_parent or from_other) else '<<< NO GLOB NEEDED?')
