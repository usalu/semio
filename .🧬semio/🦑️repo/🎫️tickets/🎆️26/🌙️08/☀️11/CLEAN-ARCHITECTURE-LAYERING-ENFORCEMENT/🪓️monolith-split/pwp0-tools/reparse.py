import sys, oracle as o
tag, name = sys.argv[1], sys.argv[2]
sb = o.SCR + '/pwp0-sb-' + tag
a = o.parse(open(f'{o.GEN}/pwp0-oracle-{tag}-A-{name}.jsonl').read(), sb + '/A'); b = o.parse(open(f'{o.GEN}/pwp0-oracle-{tag}-B-{name}.jsonl').read(), sb + '/B')
new, old = a - b, b - a
print(name, 'A', sum(a.values()), 'B', sum(b.values()), 'onlyNew', sum(new.values()), 'onlyOld', sum(old.values()))
for k, v in new.items(): print(' +new', v, k)
for k, v in old.items(): print(' -old', v, k)
