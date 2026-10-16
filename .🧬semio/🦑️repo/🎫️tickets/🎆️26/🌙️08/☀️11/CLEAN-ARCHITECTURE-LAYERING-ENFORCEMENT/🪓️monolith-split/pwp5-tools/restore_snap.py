"""♻️ Restores the lift sources and stubs from the pre-edit snapshot (own files only) and removes the lift files the run created, so lifts.py can run again."""
import os, shutil, sys
SCR = '/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad'
SNAP = SCR + '/pwp5-snap'
P = '/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/'
SKIP = ('🪝️mounts', '📦️packages', '🧷️companions', '⚛️reactor/🧪️tests', '🧪️tests')
n = 0
for dp, dn, fn in os.walk(SNAP):
    rel = os.path.relpath(dp, SNAP)
    if rel == '.' or any(rel == s or rel.startswith(s + '/') for s in SKIP): continue
    for f in fn:
        dst = os.path.join(P, rel, f)
        os.makedirs(os.path.dirname(dst), exist_ok=True)
        shutil.copy2(os.path.join(dp, f), dst); n += 1
for rel in ['🧵️retained-command/🛬️lifted/🏛️reserved/🦀️.rs', '👷️job/🛬️lifted/🔄️full-operation/🦀️.rs', '📜️declaration/📨️emission/🛬️lifted/📦️preparation/♻️retirement/🦀️.rs']:
    p = P + rel
    if os.path.exists(p): os.remove(p)
    d = os.path.dirname(p)
    while os.path.isdir(d) and not os.listdir(d) and d.startswith(P) and d != P: os.rmdir(d); d = os.path.dirname(d)
print('restored', n)
