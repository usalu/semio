import importlib.util
spec = importlib.util.spec_from_file_location('l', 'sfov-lib.py'); l = importlib.util.module_from_spec(spec); spec.loader.exec_module(l)
s = l.s
d = s.Doc('🛋️furniture/picnic-table')
for k in range(1, 6):
    l.setv(d, s.walk(l.shape(d, 50, 'plan', f'top-board-{k}'), ['w']), 0.14)
d.commit()
