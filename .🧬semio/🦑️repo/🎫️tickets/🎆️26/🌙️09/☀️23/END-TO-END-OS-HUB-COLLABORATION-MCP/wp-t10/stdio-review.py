"""🗄️ Stdio review sheet: for every stdio leaf with no explicit rejection whose artifact diff `apply` can fail, lists the
payload fields and flags a KEYED leaf (it addresses an existing entry by id/index/key/path), whose diff the apply can
refuse (`mutation.apply.missing-target|invalid-index`) — the rejection production reaches through `apply_to`."""
import json, os, re, collections, sys
root = "/Users/ueli/Documents/semio/"
ev = json.load(open(root + ".tmp-ticket/wp-t10/generated/leaf-evidence.json"))
can = json.load(open(root + ".tmp-ticket/wp-t10/generated/apply-can-err.json"))
KEY = re.compile(r"^(?:pub(?:\([a-z]+\))?\s+)?(?:r#)?([a-z_0-9]*(?:id|guid|index|key|name|path|position|at|row|column|col|tag|segment|chunk|page|object|entry|member|element|node|ifd|table|slot|number|offset|frame|track|sheet|cell|slide|part|layer|point|vertex|face|member_path|pointer))\s*:", re.M)
def scope(rel):
    parts = rel.split("/"); i = parts.index("🗿️artifacts"); return "/".join(parts[:i + 2])
out, stats = [], collections.Counter()
for rel, r in sorted(ev.items()):
    if "🗄️stdio" not in rel or "🗿️artifacts" not in rel or "rejected" in r["evidence"] or not can.get(scope(rel)): continue
    text = ""
    for dp, ds, fs in os.walk(root + rel):
        ds[:] = [d for d in ds if d not in ("🧪️tests", "🧫️fixtures", "↩️inverse")]
        for f in fs:
            if f.endswith(".rs"): text += open(os.path.join(dp, f), encoding="utf-8").read()
    m = re.search(r"pub struct \w+\s*\{([^}]*)\}", text)
    fields = re.findall(r"(?:pub(?:\([a-z]+\))?\s+)?(?:r#)?([a-z_0-9]+)\s*:", m.group(1)) if m else []
    keyed = [f for f in fields if KEY.match(f + ":")]
    verb = r["kind"].split("-")[0]
    cls = "keyed" if keyed and verb not in ("insert", "add", "append", "embed") or verb in ("remove", "delete") else "unkeyed"
    if verb in ("insert", "add", "append", "embed") and keyed: cls = "keyed-insert"
    stats[(cls, verb)] += 1
    out.append((cls, r["kind"], fields, rel.split("/🗿️artifacts/")[1].split("/")[0]))
for k, v in sorted(stats.items()): print(v, k)
if "--list" in sys.argv:
    for o in out: print(o)
json.dump(out, open(root + ".tmp-ticket/wp-t10/generated/stdio-review.json", "w"), ensure_ascii=False, indent=1)
