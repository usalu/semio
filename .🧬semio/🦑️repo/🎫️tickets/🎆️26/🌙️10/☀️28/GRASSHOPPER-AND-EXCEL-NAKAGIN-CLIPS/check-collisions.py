"""🔍 Inspect independent capsule bounding-box intersections."""
import importlib.util
import sys
from pathlib import Path
import numpy as np
p=Path(__file__).resolve().parents[7]/"🎓️teaching/🏛️architecture/🏢️nakagin/🐍️.py"
spec=importlib.util.spec_from_file_location("model",p)
m=importlib.util.module_from_spec(spec)
sys.modules["model"]=m
spec.loader.exec_module(m)
slots=m.slots()
boxes=[(np.array(m.slot_solid(s).vertices).min(axis=0),np.array(m.slot_solid(s).vertices).max(axis=0)) for s in slots]
for i,a in enumerate(boxes):
    for j,b in enumerate(boxes[i+1:],i+1):
        if np.all(np.minimum(a[1],b[1])-np.maximum(a[0],b[0])>1e-6):
            print("[DEBUG]", slots[i], slots[j])
