"""Instance dict sizes of lean faces by assignment style. Usage: xl4_dict.py"""
import io
import pickle
import sys

import numpy as np

import xl4_common as C

visuals = C.load()
shape = visuals._shape([np.array([[0, 0, 0], [1, 0, 0], [1, 1, 0], [0, 1, 0]], float)], "#ffffff", 1.0, "#000000", 0.45)
shape.world_vertices = np.zeros((4, 3))
shape.world_normal = np.zeros(3)
shape.world_depth = 1.5
shape.paint = ("shell", 0.5, 0.45, False)
shape.semantic_role, shape.semantic_key = "shell", "k"
shape.source_polygon = 3
state = dict(shape.__dict__)
print("original dict", sys.getsizeof(shape.__dict__), "keys", list(state))
blob = pickle.dumps(state, protocol=pickle.HIGHEST_PROTOCOL)
loaded = pickle.loads(blob)
print("pickle blob bytes", len(blob), "loaded dict", sys.getsizeof(loaded))
a = object.__new__(visuals._Facet)
a.__dict__ = loaded
b = object.__new__(visuals._Facet)
b.__dict__ = dict(loaded)
c = object.__new__(visuals._Facet)
for key, value in loaded.items():
    setattr(c, key, value)
d = object.__new__(visuals._Facet)
d.__dict__.update(loaded)
for name, obj in (("assign loaded", a), ("assign dict(loaded)", b), ("setattr loop", c), ("__dict__.update", d)):
    print(name, sys.getsizeof(obj.__dict__), sys.getsizeof(obj))
print("array bytes per shape: points", state["points"].nbytes + 112, "rgbas", 3 * (state["fill_rgbas"].nbytes + 112))
