import re, sys, os, glob, json, hashlib, collections
import xml.etree.ElementTree as ET
R = "/Users/ueli/Documents/semio"
G = R + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated"
label, listfile = sys.argv[1], sys.argv[2]
files = [l for l in open(listfile, encoding="utf8").read().split("\n") if l]
out = f"{G}/tsl2-{label}"
msgs = collections.Counter(); byfile = {}
rows = []
for f in files:
    k = hashlib.sha1(f.encode()).hexdigest()[:12]
    p = f"{out}/{k}.xml"
    if not os.path.exists(p):
        rows.append((f, "NOXML", "")); continue
    try: t = ET.parse(p)
    except Exception as e:
        rows.append((f, "BADXML", str(e))); continue
    lg = open(f"{out}/{k}.log", encoding="utf8", errors="replace").read().split("\n")
    errs = [l.strip()[:200] for l in lg if l.startswith("error:") or l.startswith("error ")]
    fails = [l.strip()[:200] for l in lg if l.startswith("(fail)")]
    for e in errs[:6]: msgs[re.sub(r"/Users/ueli/Documents/semio", "", e)] += 1
    for fl in fails: rows.append((f, fl, errs[0] if errs else ""))
if len(sys.argv) > 3 and sys.argv[3] == "msgs":
    for m, n in msgs.most_common(60): print(n, m)
else:
    for r in rows: print("\t".join(r))
