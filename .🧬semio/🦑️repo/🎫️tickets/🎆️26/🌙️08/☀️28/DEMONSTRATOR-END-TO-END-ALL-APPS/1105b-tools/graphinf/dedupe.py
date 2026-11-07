import re, sys
for path in sys.argv[1:]:
    lines = open(path, encoding="utf8").read().split("\n")
    out, seen, removed = [], {}, []
    for l in lines:
        if re.match(r"^\[.*\]\s*$", l): seen = {}
        k = re.match(r"^([A-Za-z0-9_\-]+)\s*=", l)
        if k and not l.startswith("["):
            key = k.group(1)
            if key in seen:
                if seen[key].strip() != l.strip(): print("KEEP-DIFFERENT", path, l)
                else: removed.append(l); continue
            else: seen[key] = l
        out.append(l)
    if removed:
        open(path, "w", encoding="utf8").write("\n".join(out)); print("deduped", len(removed), path)
