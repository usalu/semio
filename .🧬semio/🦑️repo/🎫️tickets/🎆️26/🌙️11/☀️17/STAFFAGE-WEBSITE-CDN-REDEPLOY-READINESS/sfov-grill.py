import importlib.util
spec = importlib.util.spec_from_file_location('l', 'sfov-lib.py'); l = importlib.util.module_from_spec(spec); spec.loader.exec_module(l)
s = l.s
d = s.Doc('🪑️objects/charcoal-grill')
n = l.vol(d, 100, 'charcoal-bed', 'profile')
l.setv(d, n, [[-0.03, 0.0], [-0.03, 0.15], [0.0168, 0.22], [0.066, 0.1673], [0.066, 0.0], [-0.03, 0.0]])
d.commit()
