import os, sys, importlib
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import c4p17 as P
from rsitems import items, name_kind

tag, first, last = sys.argv[1], int(sys.argv[2]), int(sys.argv[3])
sp = importlib.import_module('specs.' + tag).SPEC
txt = open(os.path.join(P.T, 'orig', sp['orig']), encoding='utf-8').read()
lines = txt.split('\n')
body = '\n'.join(lines[first - 1:last])
br = body.index('{')
head, inner = body[:br + 1], body[br + 1:]
base_line = first - 1 + head.count('\n')
for it in items(inner):
    v, k, n = name_kind(it['toks'])
    s = it['first_tok_line'] + base_line + 1
    e = it['end_line'] + base_line + 1
    a = it['attrs']
    print('%d-%d\t%d\t%s\t%s\t%s' % (s, e, e - s + 1, v, k, n), ' '.join(a)[:60])
