import importlib.util
spec = importlib.util.spec_from_file_location('l', 'sfov-lib.py'); l = importlib.util.module_from_spec(spec); spec.loader.exec_module(l)
s = l.s
d = s.Doc('🪑️objects/tealight')
n = l.vol(d, 50, 'cup-shell', 'profile')
print(d.t[n['s']:n['e']][:300])
l.setv(d, n, [[0.0, 0.0], [0.0155, 0.0], [0.019, 0.016], [0.0182, 0.016], [0.0147, 0.001], [0.0, 0.001]])
d.commit()
