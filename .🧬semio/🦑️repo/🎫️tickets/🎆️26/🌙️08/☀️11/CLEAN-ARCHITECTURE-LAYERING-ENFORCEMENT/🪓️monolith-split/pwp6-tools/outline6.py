import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
os.environ.setdefault('CLOSURE', '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated/pwp0-closure.json')
import plan_tools as pt
P = pt.ROOT + pt.P
for f in sys.argv[1:]:
    its, L, R = pt.outline(P + f)
    print('==', f, len(R))
    for it in its:
        print(f"  {it['s']:5d}-{it['e']:5d} {it['kind']:8s} {it['head'][:110]}")
