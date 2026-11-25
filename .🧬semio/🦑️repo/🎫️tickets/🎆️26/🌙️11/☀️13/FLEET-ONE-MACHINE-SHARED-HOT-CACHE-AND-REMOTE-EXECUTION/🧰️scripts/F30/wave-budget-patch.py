import json, pathlib, re, sys, hashlib
G = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations")
SCHEMA = G / "🧬️schema" / "🔣️.json"
TEST = G / "🧪️tests" / "📈️adaptive-waves" / "🟦️.ts"
FIX = G / "🧪️tests" / "📈️adaptive-waves" / "🔣️.json"
OLD, NEW = 300000, 900000
sha = lambda p: hashlib.sha256(p.read_bytes()).hexdigest()[:12]
before = {p.name + str(i): sha(p) for i, p in enumerate((SCHEMA, TEST, FIX))}

def rule(size, elapsed, inferred=None, bp=None, base=48, maximum=960, growth=4, budget=NEW):
    def scaled(v):
        return max(1, v * bp // 10000) if bp and 0 < bp < 10000 else max(1, v)
    b = scaled(base)
    if inferred is None or inferred < 0 or inferred * 4 > size: return b
    m = max(b, scaled(maximum))
    fitting = size * budget // max(1, elapsed)
    return max(b, min(m, size * growth if inferred == 0 else size, fitting))

s = SCHEMA.read_text(encoding="utf8")
if f'"ownerWaveBudgetMilliseconds": {OLD},' in s:
    SCHEMA.write_text(s.replace(f'"ownerWaveBudgetMilliseconds": {OLD},', f'"ownerWaveBudgetMilliseconds": {NEW},'), encoding="utf8")
elif f'"ownerWaveBudgetMilliseconds": {NEW},' not in s: sys.exit("schema anchor missing")

t = TEST.read_text(encoding="utf8")
t = t.replace(f"ownerWaveBudgetMilliseconds).toBe({OLD})", f"ownerWaveBudgetMilliseconds).toBe({NEW})").replace(f"growth=4,budget={OLD}", f"growth=4,budget={NEW}")
TEST.write_text(t, encoding="utf8")

lines = FIX.read_text(encoding="utf8").split("\n")
out = []
for line in lines:
    m = re.search(r'\{.*\}', line)
    if m and '"expected"' in line:
        case = json.loads(m.group(0).rstrip(","))
        if case["elapsedMilliseconds"] in (100000, 600000):
            line = line.replace(f'"elapsedMilliseconds": {case["elapsedMilliseconds"]}', f'"elapsedMilliseconds": {case["elapsedMilliseconds"] * 3}')
            case["elapsedMilliseconds"] *= 3
        exp = rule(case["size"], case["elapsedMilliseconds"], case.get("inferred"), case.get("basisPoints"))
        line = re.sub(r'"expected": \d+', f'"expected": {exp}', line)
        line = line.replace("300 s over 100 s is three times", "900 s over 300 s is three times")
    out.append(line)
FIX.write_text("\n".join(out), encoding="utf8")
for i, p in enumerate((SCHEMA, TEST, FIX)): print(p.name, before[p.name + str(i)], "->", sha(p))
