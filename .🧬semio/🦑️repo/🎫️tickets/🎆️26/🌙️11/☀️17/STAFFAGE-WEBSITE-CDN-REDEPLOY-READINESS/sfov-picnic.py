import importlib.util
spec = importlib.util.spec_from_file_location('l', 'sfov-lib.py'); l = importlib.util.module_from_spec(spec); spec.loader.exec_module(l)
s = l.s
d = s.Doc('🛋️furniture/picnic-table')
for sc in (50, 100):
    try:
        n = l.vol(d, sc, 'top-plank')
    except KeyError:
        continue
    print(sc, d.t[n['s']:n['e']][:200].replace('\n',' '))
    l.setv(d, s.walk(n, ['size', '0']), 0.14)
    l.setv(d, s.walk(n, ['transform', 'translation', '0']), 0.07)
l.setv(d, s.walk(l.shape(d, 50, 'front', 'hinge-0'), ['w']), 0.11)
l.setv(d, s.walk(l.shape(d, 50, 'front', 'hinge-1'), ['x']), -0.78)
l.setv(d, s.walk(l.shape(d, 50, 'front', 'hinge-1'), ['w']), 0.11)
d.commit()
