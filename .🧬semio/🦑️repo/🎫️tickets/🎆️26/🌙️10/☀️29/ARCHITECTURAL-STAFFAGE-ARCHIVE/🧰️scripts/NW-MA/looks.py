import sys, importlib
from render import render
from lib import T
name, mod, den, pose = sys.argv[1], sys.argv[2], int(sys.argv[3]), (sys.argv[4] if len(sys.argv) > 4 else None)
m = importlib.import_module(mod)
a = m.asset()
render(a, m.rig, den, T + f"🗑️generated/NW-MA/{name}-{den}" + (f"-{pose}" if pose else ""), pose_id=pose, pxmm=int(sys.argv[5]) if len(sys.argv) > 5 else 40)
