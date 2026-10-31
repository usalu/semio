import sys
from render import render
from mixer_asset import *
a = asset()
G_ = T + "🗑️generated/NW-MA/"
import os; os.makedirs(G_, exist_ok=True)
for den in [int(x) for x in sys.argv[1:]] or [500, 200, 100, 50]:
    render(a, rig, den, G_ + f"mixer-{den}", pxmm=40 if den >= 200 else 60)
