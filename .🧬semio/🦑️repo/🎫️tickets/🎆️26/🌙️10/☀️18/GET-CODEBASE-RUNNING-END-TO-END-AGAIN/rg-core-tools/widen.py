import json, os, re, sys, collections
sys.path.insert(0, os.path.dirname(__file__))
import rgl
R = rgl.ROOT
CODES = {"E0603", "E0616", "E0624", "E0451", "E0624"}
def targets(log):
    out = collections.OrderedDict()
    for line in open(log, encoding="utf-8", errors="replace"):
        line = line.strip()
        if not line.startswith("{"):
            continue
        try:
            d = json.loads(line)
        except Exception:
            continue
        if d.get("reason") != "compiler-message":
            continue
        m = d["message"]
        code = (m.get("code") or {}).get("code")
        if code not in CODES:
            continue
        for ch in m.get("children", []):
            for sp in ch.get("spans", []):
                if sp.get("is_primary") or True:
                    f = sp["file_name"]
                    if f.startswith("/"):
                        f = os.path.relpath(f, R)
                    out[(f, sp["line_start"], sp["column_start"])] = (code, ch.get("message", "")[:70])
    return out
def apply(tg, dry=False):
    byfile = collections.defaultdict(list)
    for (f, l, c), v in tg.items():
        byfile[f].append((l, c))
    n = 0
    for f, locs in byfile.items():
        p = os.path.join(R, f)
        if not os.path.exists(p):
            continue
        lines = rgl.read(p).split("\n")
        for l, c in sorted(set(locs), reverse=True):
            s = lines[l - 1]
            head, tail = s[:c - 1], s[c - 1:]
            m = re.match(r'pub\((?:super|crate|self|in [^)]*)\)\s*', tail)
            if m:
                tail = "pub " + tail[m.end():]
            elif tail.startswith("pub "):
                continue
            else:
                tail = "pub " + tail
            lines[l - 1] = head + tail
            n += 1
        if not dry:
            rgl.write(p, "\n".join(lines))
    return n
if __name__ == "__main__":
    tg = targets(sys.argv[1])
    for k, v in tg.items():
        print(k[0][-60:], k[1], k[2], v)
    if len(sys.argv) > 2 and sys.argv[2] == "apply":
        print("applied", apply(tg))
