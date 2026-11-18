import importlib.util
spec = importlib.util.spec_from_file_location('l', 'sfov-lib.py'); l = importlib.util.module_from_spec(spec); spec.loader.exec_module(l)
s = l.s
d = s.Doc('🪑️objects/tealight')
n = l.vol(d, 50, 'cup-shell', 'profile')
l.setv(d, n, [[0.0, 0.0], [0.0, 0.0162], [0.016, 0.0195], [0.016, 0.017], [0.002, 0.0135], [0.002, 0.0]])
d.commit()
