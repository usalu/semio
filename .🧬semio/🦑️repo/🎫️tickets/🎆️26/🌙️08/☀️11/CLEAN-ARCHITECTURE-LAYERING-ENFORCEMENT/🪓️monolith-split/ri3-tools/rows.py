"""📋 RI3 row printer: reads a pos result JSON and prints the NEW privacy and resolve rows of a configuration, stub-flagged ones included, grouped by (code, crate, file, message)."""
import json, re, sys, collections

res = json.load(open(sys.argv[1]))
cfg = sys.argv[2]
cats = sys.argv[3].split(",") if len(sys.argv) > 3 else ["privacy", "resolve"]
skip = re.compile(sys.argv[4]) if len(sys.argv) > 4 else None
rows = res["summary"][cfg]["new_list"]
g = collections.OrderedDict()
for d in rows:
    if d["cat"] not in cats:
        continue
    m = re.match(r"^(__m_[A-Za-z0-9_]+)/(.*)$", d["file"])
    crate, rel = (m.group(1)[4:].replace("semio_framework_os_renderer_wgpu", "rw"), m.group(2)) if m else ("", d["file"])
    key = (d["code"], crate, rel[-62:], re.sub(r"\s+", " ", d["msg"])[:110], d["stub"])
    g.setdefault(key, []).append(d["line"])
n = 0
for (code, crate, rel, msg, stub), lines in g.items():
    line = f"{code} [{crate}] ..{rel} x{len(lines)} {sorted(set(lines))[:4]}: {msg}{' (stub-flagged)' if stub else ''}"
    if skip and skip.search(line):
        continue
    print(line)
    n += 1
print("rows", n, "of", sum(1 for d in rows if d["cat"] in cats), "total in new_list (cap: result keeps all)")
