"""🔄️ Resets every file a staged batch of this lane touches in a MIRROR to the live bytes (or removes it when live has none): python3 -I reset.py <mirror-root> <batch...>"""
import json, os, shutil, sys
HERE = os.path.dirname(os.path.abspath(__file__))
LIVE = os.path.abspath(os.path.join(HERE, *[".."] * 8))
mirror = os.path.abspath(sys.argv[1])
assert mirror != LIVE and ("/sm/" in mirror or "/tmp/m/" in mirror), "a mirror root is required"
for batch in sys.argv[2:]:
    for rel in json.load(open(os.path.join(HERE, "staged", batch, "manifest.json"), encoding="utf8")):
        source, target = os.path.join(LIVE, rel), os.path.join(mirror, rel)
        if os.path.exists(source): os.makedirs(os.path.dirname(target), exist_ok=True); shutil.copy2(source, target)
        elif os.path.exists(target): os.remove(target)
print("reset", sys.argv[2:])
