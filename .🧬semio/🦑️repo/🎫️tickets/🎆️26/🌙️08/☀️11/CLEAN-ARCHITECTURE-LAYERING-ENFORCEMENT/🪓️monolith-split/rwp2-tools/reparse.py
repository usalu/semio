import sys, os
sys.argv_saved = sys.argv
import oracle2 as o2
tag, cfg = sys.argv[1], sys.argv[2]
crates = sys.argv[3].split(',') if len(sys.argv) > 3 else []
sb = o2.SCR + '/rwp2/sb-' + tag
A, B = sb + '/A', sb + '/B'
ea = open('%s/rwp2-oracle-%s-A-%s.jsonl' % (o2.GEN, tag, cfg)).read(); eb = open('%s/rwp2-oracle-%s-B-%s.jsonl' % (o2.GEN, tag, cfg)).read()
a, b = o2.norm(o2.o.parse(ea, A), A, crates), o2.norm(o2.o.parse(eb, B), B, crates)
new, old = a - b, b - a
print('== %s: A=%d B=%d onlyNew=%d onlyOld=%d' % (cfg, sum(a.values()), sum(b.values()), sum(new.values()), sum(old.values())))
for k, v in sorted(new.items()): print('  +new', v, k)
for k, v in sorted(old.items()): print('  -old', v, k)
