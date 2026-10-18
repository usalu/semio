"""📑 C6FIN inner item lister: innerlist.py <tag> <inline-mod-name> [indent] prints the items of the body of an inline module as modsplit sees them (line numbers are body-relative, 1-based). Applies the spec pre-edit if a spec exists."""
import os, sys, importlib
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import modsplit as M
tag, mod = sys.argv[1], sys.argv[2]
indent = int(sys.argv[3]) if len(sys.argv) > 3 else 4
try:
    sp = importlib.import_module('specs.' + tag).SPEC
except ModuleNotFoundError:
    sp = dict(orig=tag + '.rs')
txt = open(os.path.join(HERE, 'orig', sp['orig']), encoding='utf-8').read()
if sp.get('pre'):
    txt = sp['pre'](txt)
doc = M.Doc(txt)
c = [i for i, it in enumerate(doc.items) if it['kind'] == 'mod' and it['name'] == mod and not doc.lines[it['e']].rstrip().endswith(';')]
assert len(c) == 1, c
pre, head, body, hdr = M.inline_body(doc.lines, doc.items[c[0]], indent, tag)
inner = M.Doc('\n'.join(body))
print('header at file line', doc.items[c[0]]['s'] + 1 + len(pre), 'body lines', len(body))
M.listing(inner)
