import os, re, json, sys, collections, subprocess
ROOT = "/Users/ueli/Documents/semio"
F = os.path.join(ROOT, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split")
G = os.path.join(F, "🗑️generated")
out = subprocess.run(["git", "-c", "core.fsmonitor=false", "-c", "core.quotepath=false", "grep", "-l", "-F", "semio_framework_test_fixtures", "--", "*.rs", ":!.🧬semio"], cwd=ROOT, capture_output=True, text=True).stdout.split("\n")
untracked = [l for l in open(os.path.join(G, "frd-untracked-rs.txt"), encoding="utf-8").read().split("\n") if l]
files = sorted({f for f in out if f} | set(untracked))
ex = json.load(open(os.path.join(F, "frd-tools/excluded.json"), encoding="utf-8"))["fragments"]
c = collections.Counter(); nfiles = 0
for rel in files:
    p = os.path.join(ROOT, rel)
    if not os.path.exists(p): continue
    t = open(p, encoding="utf-8").read()
    if "semio_framework_test_fixtures" not in t: continue
    if "/🧲️fixture-reader/" in rel: continue
    nfiles += 1
    for m in re.finditer(r'\b(fixture_text|fixture_bytes|fixture_path|manifest_dir|repo_root|repo_text|repo_bytes)!\(', t): c[m.group(1)] += 1
print("files using the reader:", nfiles); print(dict(c), sum(c.values()))
rest = collections.Counter()
for rel in json.load(open(os.path.join(G, "frd-files.json"), encoding="utf-8")):
    p = os.path.join(ROOT, rel)
    if not os.path.exists(p): continue
    t = open(p, encoding="utf-8").read()
    n = len(re.findall(r'fn runtime_fixture_(?:text|bytes)\b', t))
    if n:
        o = next((x["owner"] for x in ex if x["fragment"] in rel), "free-hot")
        rest[o] += n
print("helper copies left:", dict(rest), sum(rest.values()))
