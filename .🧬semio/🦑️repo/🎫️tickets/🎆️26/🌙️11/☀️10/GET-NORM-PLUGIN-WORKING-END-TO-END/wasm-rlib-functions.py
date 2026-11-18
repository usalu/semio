"""🔢️ Counts defined wasm functions per rlib linked into a failed component, from its rustc diagnostic."""
import json, re, sys, glob, os
from collections import Counter

def leb(b, i):
    r = s = 0
    while True:
        x = b[i]; i += 1; r |= (x & 0x7f) << s; s += 7
        if x < 0x80: return r, i

def functions(obj):
    if obj[:4] != b"\0asm": return 0
    i, n = 8, 0
    while i < len(obj):
        sid = obj[i]; size, j = leb(obj, i + 1)
        if sid == 3: n += leb(obj, j)[0]
        i = j + size
    return n

def rlib_functions(path):
    data = open(path, "rb").read()
    if not data.startswith(b"!<arch>\n"): return 0
    i, total = 8, 0
    while i + 60 <= len(data):
        size = int(data[i + 48:i + 58].decode().strip()); body = data[i + 60:i + 60 + size]
        total += functions(body); i += 60 + size + (size & 1)
    return total

out = glob.glob(sys.argv[1] + "/fingerprint/output*")[0]
text = next(json.loads(l)["rendered"] for l in open(out) if '"level":"error"' in l and "linking" in l)
rlibs = re.findall(r'"([^"]+\.rlib)"', text)
counts = Counter({os.path.basename(p).split("-")[0][3:]: rlib_functions(p) for p in rlibs if os.path.exists(p)})
print("rlibs", len(rlibs), "total defined functions", sum(counts.values()))
for name, n in counts.most_common(int(sys.argv[2]) if len(sys.argv) > 2 else 40): print(n, name)
