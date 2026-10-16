"""🧹 Heuristic unused-import scan of the carved files (an imported name that appears nowhere else in its file, test modules included when the file is the module root)."""
import filecmp, os, re, sys
a, b = sys.argv[1], sys.argv[2]
USE = re.compile(r"^(\s*)(?:pub(?:\([^)]*\))?\s+)?use\s+([^;]+);", re.M | re.S)
def names(spec):
    spec = re.sub(r"\s+", " ", spec.strip())
    out = []
    def walk(prefix, body):
        body = body.strip()
        if "{" not in body:
            for part in body.split(","):
                part = part.strip()
                if not part: continue
                if " as " in part: out.append(part.split(" as ")[1].strip())
                elif part == "self": out.append(prefix.rstrip(":").split("::")[-1])
                elif part == "*": pass
                else: out.append(part.split("::")[-1])
            return
        depth, start, parts = 0, 0, []
        for i, ch in enumerate(body):
            if ch == "{": depth += 1
            elif ch == "}": depth -= 1
            elif ch == "," and depth == 0:
                parts.append(body[start:i]); start = i + 1
        parts.append(body[start:])
        for part in parts:
            part = part.strip()
            if not part: continue
            if "{" in part:
                head, inner = part.split("{", 1)
                walk(prefix + head, inner.rsplit("}", 1)[0])
            else:
                walk(prefix, part)
    walk("", spec)
    return [x for x in out if re.match(r"^\w+$", x)]
bad = 0
for dp, dn, fn in os.walk(b):
    if "node_modules" in dp or "🧭️journeys" in dp: continue
    for f in fn:
        if not f.endswith(".rs"): continue
        new = os.path.join(dp, f)
        old = os.path.join(a, os.path.relpath(new, b))
        if os.path.exists(old) and filecmp.cmp(old, new, shallow=False): continue
        text = open(new, encoding="utf-8").read()
        # a file that mounts test children keeps imports those tests use through `use super::*`
        for m in USE.finditer(text):
            for name in names(m.group(2)):
                count = len(re.findall(r"\b" + re.escape(name) + r"\b", text))
                if count <= 1 and name not in ("self", "super"):
                    bad += 1
                    print(os.path.relpath(new, b), "unused?", name)
print("flagged", bad)
