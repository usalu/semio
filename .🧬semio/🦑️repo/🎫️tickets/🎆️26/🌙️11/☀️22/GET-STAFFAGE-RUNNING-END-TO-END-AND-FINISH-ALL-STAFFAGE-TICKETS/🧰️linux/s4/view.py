#!/usr/bin/env python3
"""🔭️ Shortens a dump of dump.ts for reading: bone frames as origin and axes in millimetres, long lines cut. usage: view.py <dump file> [max line length]"""
import json, re, sys
limit = int(sys.argv[2]) if len(sys.argv) > 2 else 330
axis = lambda a: "".join(("+" if a[k] > 0 else "-") + k for k in "xyz" if abs(a[k]) > 0.5) or "(%.2f,%.2f,%.2f)" % (a["x"], a["y"], a["z"])
for line in open(sys.argv[1]):
    line = line.rstrip("\n")
    if line.startswith("bone "):
        head, frame = line.split(" frame ")
        f = json.loads(frame)
        tilt = any(0.01 < abs(f[n][k]) < 0.99 for n in "uvw" for k in "xyz")
        head = re.sub(r" rest\(.*\)", "", head)
        print(f"{head} o=({f['o']['x']*1000:.0f},{f['o']['y']*1000:.0f},{f['o']['z']*1000:.0f}) u={axis(f['u'])} v={axis(f['v'])} w={axis(f['w'])}{' TILTED ' + frame if tilt else ''}")
    elif line.startswith("decisions") or line.startswith("materials"):
        print(line[:700])
    elif line.startswith("refs:"):
        print(line[:1600])
    else:
        print(line[:limit])
