# Test-file item splitter: top-level items (with their leading doc/attr lines) keyed by fn/const name.
import re
def split(text):
    lines = text.split("\n"); items = []; i = 0; n = len(lines)
    while i < n:
        if not lines[i].strip(): i += 1; continue
        start = i
        # leading attrs/docs/comments
        while i < n and (lines[i].lstrip().startswith(("#[", "///", "//")) ) and not lines[i].startswith("// #region") : i += 1
        if i >= n: items.append((None, start, n)); break
        head = lines[i]
        depth = 0; j = i; seen = False
        while j < n:
            l = lines[j]
            depth += l.count("{") - l.count("}")
            if "{" in l: seen = True
            if (seen and depth == 0) or (not seen and l.rstrip().endswith(";") and depth == 0): break
            j += 1
        m = re.match(r"\s*(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?(?:fn|const|static|use|struct|enum|mod|type)\s+(\w+)", head)
        name = m.group(1) if m else head.strip()[:40]
        if head.startswith("use "): name = "use:" + head.strip()
        items.append((name, start, j + 1)); i = j + 1
    return lines, items
def take(text, names):
    lines, items = split(text)
    out = []
    for name, a, b in items:
        if name in names: out.append("\n".join(lines[a:b]))
    return out
def drop(text, names):
    lines, items = split(text); keep = [True] * len(lines)
    for name, a, b in items:
        if name in names:
            for k in range(a, b): keep[k] = False
            if b < len(lines) and not lines[b].strip(): keep[b] = False
    return "\n".join(l for l, k in zip(lines, keep) if k)
if __name__ == "__main__":
    import sys
    t = open(sys.argv[1], encoding="utf-8").read()
    lines, items = split(t)
    for name, a, b in items: print(a + 1, b, name)
