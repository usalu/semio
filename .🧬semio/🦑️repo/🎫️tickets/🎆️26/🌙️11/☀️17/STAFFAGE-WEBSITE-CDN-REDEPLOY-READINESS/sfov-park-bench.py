import importlib.util
spec = importlib.util.spec_from_file_location('l', 'sfov-lib.py'); l = importlib.util.module_from_spec(spec); spec.loader.exec_module(l)
s = l.s
d = s.Doc('🛋️furniture/park-bench')
n = l.vol(d, 50, 'seat-slat')
l.setv(d, s.walk(n, ['size', '2']), 0.1)
l.setv(d, s.walk(n, ['transform', 'translation', '2']), -0.25)
l.setv(d, s.walk(n, ['repeat', 'translation', '2']), 0.13)
for k in range(1, 5):
    sh = l.shape(d, 50, 'front', f'seat-slat-{k}')
    l.setv(d, s.walk(sh, ['w']), 0.1)
    l.setv(d, s.walk(sh, ['x']), round(-0.3 + 0.13 * (k - 1), 5))
d.commit()
