import importlib.util
spec = importlib.util.spec_from_file_location('l', 'sfov-lib.py'); l = importlib.util.module_from_spec(spec); spec.loader.exec_module(l)
s = l.s
d = s.Doc('🛋️furniture/park-bench')
for k in range(1, 5):
    sh = l.shape(d, 50, 'plan', f'seat-slat-{k}')
    l.setv(d, s.walk(sh, ['h']), 0.1)
    l.setv(d, s.walk(sh, ['y']), round(-0.3 + 0.13 * (k - 1), 5))
d.commit()
