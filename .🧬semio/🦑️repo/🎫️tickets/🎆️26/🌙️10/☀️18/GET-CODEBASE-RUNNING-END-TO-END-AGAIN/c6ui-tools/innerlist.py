"""📋 C6DB2 inner module outline: innerlist.py <tag> <mod> [indent] prints the item listing of the dedented body of an inline `mod <mod> {` in the way modsplit.py addresses it (1-based inner line numbers)."""
import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import modsplit as M
tag, mod = sys.argv[1], sys.argv[2]
indent = int(sys.argv[3]) if len(sys.argv) > 3 else 4
text = open(os.path.join(M.T, 'orig', tag + '.rs'), encoding='utf-8').read()
import importlib
try:
    _sp = importlib.import_module('specs.' + tag).SPEC
    if _sp.get('pre'):
        text = _sp['pre'](text)
except ModuleNotFoundError:
    pass
doc = M.Doc(text)
c = [i for i, it in enumerate(doc.items) if it['kind'] == 'mod' and it['name'] == mod and not doc.lines[it['e']].rstrip().endswith(';')]
assert len(c) == 1, c
it = doc.items[c[0]]
pre, head, body, hdr = M.inline_body(doc.lines, it, indent, tag)
print('# pre', pre, 'head', head)
M.listing(M.Doc('\n'.join(body)))
open(os.path.join(M.SCR, tag + '.inner.rs'), 'w', encoding='utf-8').write('\n'.join(body))
