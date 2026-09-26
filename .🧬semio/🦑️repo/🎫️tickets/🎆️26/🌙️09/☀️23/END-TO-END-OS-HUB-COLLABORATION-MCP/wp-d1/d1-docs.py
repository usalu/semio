"""📚️ D1: for one plugin's undescribed agent verbs, prints the module docs of the mutation/command leaf named like the verb."""
import json, os, re, sys, glob, collections
ROOT = "/Users/ueli/Documents/semio"
plugin = sys.argv[1]
only = set(sys.argv[2].split(",")) if len(sys.argv) > 2 else None
rows = json.load(open(f"{ROOT}/.tmp-ticket/wp-d1/generated/census-before.json"))
REG = json.load(open(f"{ROOT}/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🔌️plugins.json"))
entry = next(e for e in REG if e["pluginId"] == plugin)
proot = os.path.join(ROOT, os.path.dirname(os.path.dirname(entry["cratePath"])))
kebab = lambda s: re.sub(r"(?<!^)(?=[A-Z])", "-", s).lower()
leaves = collections.defaultdict(list)
for path in glob.glob(os.path.join(proot, "**", "🦀️.rs"), recursive=True):
    if "🧪️tests" in path or "/target/" in path:
        continue
    folder = os.path.basename(os.path.dirname(path))
    name = re.sub(r"^[^a-z0-9]+", "", folder)
    if name:
        leaves[name].append(path)
seen = set()
for r in rows:
    if r["plugin"] != plugin or r["audience"] != "agent" or (r["desc_en"] and r["desc_de"]):
        continue
    vid = r["id"].rsplit(".", 1)[-1]
    art = r["app"].split("@")[0].split(".")[-1] if r["app"] else ""
    key = (vid, art)
    if key in seen or (only and vid not in only):
        continue
    seen.add(key)
    hits = [p for p in leaves.get(kebab(vid), []) if art in p or True]
    hits = sorted(hits, key=lambda p: (art not in p, len(p)))[:1]
    print(f"### {vid} [{art}] {r['title_en']}")
    for p in hits:
        doc = [l[4:].rstrip() for l in open(p).read().splitlines()[:14] if l.startswith("//!")]
        print("   " + " ".join(doc)[:700])
