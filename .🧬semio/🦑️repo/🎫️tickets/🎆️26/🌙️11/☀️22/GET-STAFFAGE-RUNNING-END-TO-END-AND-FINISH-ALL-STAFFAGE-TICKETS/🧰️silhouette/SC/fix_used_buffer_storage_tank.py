"""🛢️ Repair of `used-buffer-storage-tank` at 1:50: seen from ahead the round shell is drawn with a corner of its polygon on the bearers and tessellated with an edge there (30 segments, half a step apart), so the slivers between shell and bearer ended 18.8 mm apart in the two. The shell solid is turned by half a tessellation step (6 degrees) about its own axis, which changes nothing of the round body and puts its lowest corner where the drawing has it. Idempotent: python3 fix_used_buffer_storage_tank.py"""
import sys
sys.path.insert(0, __file__.rsplit("/", 1)[0])
import sc

ID = "used-buffer-storage-tank"
text, doc = sc.load(ID)
sc.solid(doc, 50, "tank-shell")["transform"] = {"rotation": [0.052336, 0.0, 0.0, 0.99863]}
sc.save(ID, text, doc)
