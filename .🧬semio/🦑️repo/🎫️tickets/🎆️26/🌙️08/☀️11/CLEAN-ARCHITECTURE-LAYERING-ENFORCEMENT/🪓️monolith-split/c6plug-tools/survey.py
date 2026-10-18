import sys, os, re, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from modsplit import Doc
from usetree import parse_use_item
from rsitems import lex, items
T = os.path.dirname(os.path.abspath(__file__))
for tag in sys.argv[1:]:
    text = open(T + '/orig/%s.rs' % tag, encoding='utf-8').read()
    doc = Doc(text)
    lines = doc.lines
    use_lines = set()
    binds = []
    for it in doc.items:
        if it['kind'] == 'use':
            for L in range(it['s'], it['e'] + 1):
                use_lines.add(L)
            toks = [t for t in lex('\n'.join(lines[it['s']:it['e'] + 1]))]
            for b in parse_use_item(toks):
                binds.append((b, it['s'] + 1, it['attrs']))
    # occurrences outside use lines
    cnt = collections.Counter()
    for k, x, sl, el, si in lex(text):
        if k == 'id' and sl not in use_lines:
            cnt[x] += 1
    print('==', tag)
    for (name, full, glob), ln, attrs in binds:
        print('  L%-5d %-28s %-70s occ=%d %s' % (ln, name, full[:70], cnt.get(name, 0), attrs))
