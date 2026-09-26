"""🔬️ D1: prints, per undescribed agent verb of one plugin, the payload fields and handler body of the command leaf named like it."""
import json, os, re, sys, glob, collections
ROOT = "/Users/ueli/Documents/semio"
plugin = sys.argv[1]
limit = int(sys.argv[2]) if len(sys.argv) > 2 else 18
rows = json.load(open(f"{ROOT}/.tmp-ticket/wp-d1/generated/census-before.json"))
REG = json.load(open(f"{ROOT}/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🔌️plugins.json"))
entry = next(e for e in REG if e["pluginId"] == plugin)
proot = os.path.join(ROOT, os.path.dirname(os.path.dirname(entry["cratePath"])))
kebab = lambda s: re.sub(r"(?<!^)(?=[A-Z])", "-", s).lower()
leaves = collections.defaultdict(list)
for path in glob.glob(os.path.join(proot, "**", "🎮️commands", "**", "🦀️.rs"), recursive=True):
    if "🧪️tests" in path:
        continue
    name = re.sub(r"^[^a-z0-9]+", "", os.path.basename(os.path.dirname(path)))
    leaves[name].append(path)
seen = set()
for r in rows:
    if r["plugin"] != plugin or r["audience"] != "agent" or (r["desc_en"] and r["desc_de"]):
        continue
    vid = r["id"].rsplit(".", 1)[-1]
    art = r["app"].split("@")[0].split(".")[-1] if r["app"] else ""
    if (vid, art) in seen:
        continue
    seen.add((vid, art))
    print(f"### {vid} [{art}] {r['kind']} {r['title_en']}")
    for p in sorted(leaves.get(kebab(vid), []), key=lambda p: (art not in p, len(p)))[:1]:
        text = open(p).read()
        keep = [l for l in text.splitlines() if l.strip() and not l.strip().startswith(("use ", "//", "#[", "#!")) and "mod tests" not in l]
        print("   " + "\n   ".join(keep[:limit]))
