import importlib.util
spec = importlib.util.spec_from_file_location('l', 'sfov-lib.py'); l = importlib.util.module_from_spec(spec); spec.loader.exec_module(l)
s = l.s
d = s.Doc('🛋️furniture/picnic-table')
l.setv(d, s.walk(l.shape(d, 100, 'front', 'hinge-0'), ['w']), 0.11)
l.setv(d, s.walk(l.shape(d, 100, 'front', 'hinge-1'), ['x']), -0.78)
l.setv(d, s.walk(l.shape(d, 100, 'front', 'hinge-1'), ['w']), 0.11)
d.commit()
