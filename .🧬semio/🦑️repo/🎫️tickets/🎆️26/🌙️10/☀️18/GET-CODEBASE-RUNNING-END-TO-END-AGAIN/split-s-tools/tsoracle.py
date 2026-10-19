"""📏 split-s TS line-multiset oracle: tsoracle.py <orig> <new files...> — every original non-blank line must appear (as a multiset, trimmed) across the parts; extras are listed (imports/exports are expected)."""
import sys, collections
orig = open(sys.argv[1], encoding='utf-8').read().split('\n')
O = collections.Counter(l.strip() for l in orig if l.strip())
N = collections.Counter()
for f in sys.argv[2:]:
    N.update(l.strip() for l in open(f, encoding='utf-8').read().split('\n') if l.strip())
miss = O - N
extra = N - O
print('orig', sum(O.values()), 'new', sum(N.values()), 'missing', sum(miss.values()), 'extra', sum(extra.values()))
for l, c in sorted(miss.items()):
    print('  - %d %s' % (c, l[:170]))
for l, c in sorted(extra.items()):
    print('  + %d %s' % (c, l[:120]))
sys.exit(1 if miss else 0)
