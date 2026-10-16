#!/usr/bin/env python3
# 📑️ Prints the column-0 item outline (start-end, kind, head) of Rust files relative to the plugin folder.
import sys, os
sys.argv_saved = sys.argv; sys.argv = ['x']
import plan_tools as pt
from common import PLUG
for rel in sys.argv_saved[1:]:
    its, L, R = pt.outline(PLUG + '/' + rel)
    print('==', rel, len(R), 'lines')
    for it in its:
        print(f"{it['s']:5d}-{it['e']:<5d} {it['kind']:7s} {it['head'][:110]}")
