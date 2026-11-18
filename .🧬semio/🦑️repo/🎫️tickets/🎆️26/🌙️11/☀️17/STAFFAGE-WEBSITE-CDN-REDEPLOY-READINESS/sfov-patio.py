import importlib.util
spec = importlib.util.spec_from_file_location('l', 'sfov-lib.py'); l = importlib.util.module_from_spec(spec); spec.loader.exec_module(l)
s = l.s
d = s.Doc('🪑️objects/patio-heater')
l.setv(d, s.walk(l.shape(d, 50, 'front', 'wheel-l'), ['x']), 0.19)
l.setv(d, s.walk(l.shape(d, 50, 'front', 'wheel-r'), ['x']), -0.21)
d.commit()
