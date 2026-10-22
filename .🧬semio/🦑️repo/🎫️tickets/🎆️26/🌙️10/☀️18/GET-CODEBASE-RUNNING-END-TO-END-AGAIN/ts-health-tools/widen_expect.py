import re, sys, collections
os_root = "/Users/ueli/Documents/semio/"
log = sys.argv[1]
write = "--write" in sys.argv
head = re.compile(r"^(.+?)\((\d+),(\d+)\): error TS2769: No overload matches this call\.$")
errs = collections.defaultdict(list)
lines = open(log, encoding="utf-8").read().split("\n")
for i, l in enumerate(lines):
    m = head.match(l)
    if not m:
        continue
    nxt = lines[i + 1] if i + 1 < len(lines) else ""
    if "(expected:" not in nxt:
        continue
    errs[m.group(1)].append((int(m.group(2)), int(m.group(3))))

def skip_string(s, i):
    q = s[i]
    i += 1
    while i < len(s):
        c = s[i]
        if c == "\\":
            i += 2
            continue
        if c == q:
            return i + 1
        if q == "`" and c == "$" and s[i + 1: i + 2] == "{":
            depth = 1
            i += 2
            while i < len(s) and depth:
                if s[i] in "'\"`":
                    i = skip_string(s, i)
                    continue
                if s[i] == "{":
                    depth += 1
                elif s[i] == "}":
                    depth -= 1
                i += 1
            continue
        i += 1
    return i

def close_paren(s, i):
    depth = 0
    while i < len(s):
        c = s[i]
        if c in "'\"`":
            i = skip_string(s, i)
            continue
        if c == "(":
            depth += 1
        elif c == ")":
            depth -= 1
            if depth == 0:
                return i
        i += 1
    return -1

total = 0
skipped = []
for f, pos in errs.items():
    path = os_root + f
    text = open(path, encoding="utf-8").read()
    ls = text.split("\n")
    starts = [0]
    for l in ls:
        starts.append(starts[-1] + len(l) + 1)
    edits = set()
    for (ln, col) in pos:
        line = ls[ln - 1]
        u16 = line.encode("utf-16-le")
        prefix = u16[: 2 * (col - 1)].decode("utf-16-le")
        off = starts[ln - 1] + len(prefix)
        found = None
        for m in re.finditer(r"expect\(", text[max(0, off - 20000): off]):
            pass
        cands = [m.start() + max(0, off - 20000) for m in re.finditer(r"\bexpect\(", text[max(0, off - 20000): off])]
        for c in reversed(cands):
            e = close_paren(text, c + 6)
            if e < 0 or e >= off:
                continue
            m2 = re.match(r"(\.(not|resolves|rejects))*\.(toBe|toEqual|toStrictEqual|toContain|toBeCloseTo|toMatchObject|toContainEqual)\(", text[e + 1: e + 80])
            if not m2:
                continue
            p0 = e + 1 + m2.end() - 1
            q = close_paren(text, p0)
            if p0 < off <= q:
                found = c
                break
        if found is None:
            skipped.append((f, ln, col))
            continue
        edits.add(found + 6)
    new = text
    for p in sorted(edits, reverse=True):
        new = new[:p] + "<unknown>" + new[p:]
    total += len(edits)
    if write and edits:
        open(path, "w", encoding="utf-8").write(new)
print("files", len(errs), "edits", total, "skipped", len(skipped))
for s in skipped:
    print("SKIP", s)
