"""📝️ Writes the tables of `U/silhouette-SV-1.md` from the logs of 🗑️generated/silhouette/SV-1 (before, after, proofs, ops) between the markers `<!-- tables -->` and `<!-- /tables -->` of the report: `python3 report.py`."""
import json, os, re, glob
W = os.path.dirname(os.path.abspath(__file__))
U = os.path.dirname(os.path.dirname(W))
GEN = os.path.join(U, "🗑️generated", "silhouette", "SV-1")
SCOPE = "agave apple-tree bamboo-clump black-alder black-locust common-ash crab-apple date-palm english-oak european-beech field-maple fig-tree ginkgo hawthorn honey-locust hornbeam-hedge horse-chestnut hosta-clump italian-cypress".split()
def rows(path):
    out = []
    if not os.path.exists(path): return out
    for line in open(path):
        m = re.match(r"\S+ 1:(\d+) \(f [\d.]+ mm\) bbox (.*)", line)
        if m: out.append((m.group(1), m.group(2).strip()))
    return out
def failing(row): return row.count("!")
def proof(name, i):
    for f in sorted(glob.glob(os.path.join(GEN, "ship-*.txt")) + [os.path.join(GEN, "proof.txt")]):
        pass
    return ""
lines = []
manifest = json.load(open(os.path.join(W, "manifest-changes.json")))
proofs = {}
for f in [os.path.join(GEN, "proof.txt")]:
    current = None
    if os.path.exists(f):
        for line in open(f):
            m = re.match(r"(\S+): \d+ ops", line)
            if m: current = m.group(1); proofs.setdefault(current, {})
            m = re.match(r"🧊️catalog-silhouette exit (\d+)\s+(.*)", line)
            if m and current: proofs[current]["test"] = ("passes" if m.group(1) == "0" else "FAILS") + " (" + m.group(2).strip().rstrip(";") + ")"
            m = re.match(r"(AssertionError|Error): (.*)", line)
            if m and current: proofs[current]["first"] = m.group(2).strip()[:200]
            m = re.match(r"(\S+)\t(\d+ issues)", line)
            if m and m.group(1) in SCOPE: proofs.setdefault(m.group(1), {})["validate"] = m.group(2)
            m = re.match(r"check CLI exit (\d+): (.*)", line)
            if m and current: proofs[current]["check"] = m.group(2).strip()
rerun = os.path.join(GEN, "final-test.txt")
if os.path.exists(rerun):
    current = None
    for line in open(rerun):
        m = re.match(r"## (\S+)", line)
        if m: current = m.group(1); continue
        m = re.match(r"🧊️catalog-silhouette exit (\d+)\s+(.*)", line)
        if m and current: proofs.setdefault(current, {})["test"] = ("passes" if m.group(1) == "0" else "FAILS") + " (" + m.group(2).strip().rstrip(";") + ")"; proofs[current].pop("first", None)
        m = re.match(r"(AssertionError|Error): (.*)", line)
        if m and current: proofs[current]["first"] = m.group(2).strip()[:200]
        m = re.match(r"(\S+)\t(\d+ issues)", line)
        if m and current: proofs[current]["validate"] = m.group(2)
        m = re.match(r"check CLI exit (\d+): (.*)", line)
        if m and current: proofs[current]["check"] = m.group(2).strip()
for a in SCOPE:
    before, after = rows(os.path.join(GEN, "before", a + ".log")), rows(os.path.join(GEN, "after", a + ".log"))
    ops = json.load(open(os.path.join(W, "ops", a + ".json"))) if os.path.exists(os.path.join(W, "ops", a + ".json")) else []
    kinds = {}
    for o in ops:
        k = {"shape-add": "patches added", "shape": "shapes cut back or rebound", "shape-remove": "shapes removed", "solid-set": "solids moved or stretched", "dims": "dimension op"}.get(o["op"], o["op"])
        if o["op"] == "shape" and str(o.get("id", "")).startswith("sv1-"): k = "patches reshaped"
        kinds[k] = kinds.get(k, 0) + 1
    p = proofs.get(a, {})
    lines.append(f"### {a}\n")
    lines.append(f"- Failing checks (bounding box axes, IoU and largest boundary distance over all scales and views): **{sum(failing(r) for _, r in before)} before, {sum(failing(r) for _, r in after)} after**.")
    lines.append(f"- Real `🧊️catalog-silhouette` test: {p.get('test', 'not run')}" + (f"; first failing expectation: `{p['first']}`" if p.get("first") else "") + f". `validateAsset`: {p.get('validate', '?')}. Check CLI: `{p.get('check', '?')}`.")
    lines.append(f"- Ops (`🧰️silhouette/SV-1/ops/{a}.json`): " + (", ".join(f"{n} {k}" for k, n in kinds.items()) if kinds else "none") + (f". Declared dimensions for the manifest: `{json.dumps(manifest[a])}`" if a in manifest else "") + ".")
    lines.append("")
    lines.append("| scale | before (width height depth: body/declared; view IoU/largest distance; `!` fails) | after |")
    lines.append("|---|---|---|")
    am = dict(after)
    for s, r in before: lines.append(f"| 1:{s} | {r} | {am.get(s, '')} |")
    for s, r in after:
        if s not in dict(before): lines.append(f"| 1:{s} | (not measured before: no drawing of this scale) | {r} |")
    lines.append("")
path = os.path.join(U, "silhouette-SV-1.md")
text = open(path).read()
a, b = text.index("<!-- tables -->") + len("<!-- tables -->"), text.index("<!-- /tables -->")
open(path, "w").write(text[:a] + "\n\n" + "\n".join(lines) + "\n" + text[b:])
print(f"{len(SCOPE)} plants, {sum(1 for a in SCOPE if proofs.get(a, {}).get('test', '').startswith('passes'))} pass the real test")
