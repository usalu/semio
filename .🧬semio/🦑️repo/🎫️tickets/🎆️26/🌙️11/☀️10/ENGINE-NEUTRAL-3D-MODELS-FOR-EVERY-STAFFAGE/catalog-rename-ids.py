"""Renames ids that violate the kebab-case `Slug` to valid slugs inside one catalog asset, by exact string replacement (the file is never re-serialised).
Usage: catalog-rename-ids.py <category-folder/slug> [--apply] [--map old=new ...]
 - candidates: every identifier string of the asset (ids, bone, parent, axisBone, anchors, supports, loads, track targets, bone-keyed maps) that does not match the Slug pattern;
 - default new id: lower case, '.' -> '-'; explicit --map entries override;
 - a bone id (a rig bone) is renamed everywhere, including the "bone" members of the authored `volume` (never its solid ids); every other id is renamed outside `volume` only;
 - the same mapping is applied to ticket sources `volumes/<category>/<slug>.json` ("bone" members) when they exist.
Prints the rename table; without --apply nothing is written."""
import json, os, re, sys
SLUG = re.compile(r"^[a-z0-9]+(?:-[a-z0-9]+)*$")
S = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/"
T = os.path.dirname(os.path.abspath(__file__))
spec = sys.argv[1]
apply = "--apply" in sys.argv
explicit = dict(a.split("=", 1) for a in sys.argv[2:] if "=" in a and not a.startswith("--"))
path = S + spec + "/🔣️.json"
raw = open(path, encoding="utf-8").read()
doc = json.loads(raw)
asset = doc["asset"]
bones = {b["id"] for b in asset["rig"]["bones"]}
ID_KEYS = {"id", "bone", "parent", "axisBone", "region", "regions", "anchor", "anchors", "subject", "operatingKey", "material", "key", "reference", "to", "from", "target"}
found = set()
def parts(s):
    return s.split(":") if ":" in s else [s]
def walk(x, key=None, listed=False):
    if isinstance(x, dict):
        for k, v in x.items():
            if k == "volume" and key is None: continue
            if key in ("bones", "emitters", "operating") and not listed and isinstance(v, (dict, int, float)) and not SLUG.match(k) and re.match(r"^[A-Za-z0-9._-]+$", k): found.add(k)
            walk(v, k)
    elif isinstance(x, list):
        for v in x: walk(v, key, True)
    elif isinstance(x, str) and key in ID_KEYS:
        for p in parts(x):
            if not SLUG.match(p) and re.match(r"^[A-Za-z0-9._-]+$", p) and key != "target" or (key == "target" and not SLUG.match(p) and re.match(r"^[A-Za-z0-9._-]+$", p)): found.add(p)
walk(asset)
audit = json.load(open(os.path.join(T, "🗑️generated", "catalog-fix", "audit.json")))
offending = set()
for r in audit["rows"]:
    if r["source"].endswith("/" + spec.split("/")[1]) and not r["volume"]:
        if r["rule"] == "pattern":
            m = re.search(r'\(value (.*)\)$', r["message"])
            if m and m.group(1).startswith('"') and m.group(1).endswith('"'): offending.update(m.group(1)[1:-1].split(":"))
        if r["rule"] == "propertyNames":
            m = re.search(r'"propertyName":"([^"]*)"', r["message"])
            if m: offending.add(m.group(1))
found &= offending
found |= {k for k in explicit if '"' + k + '"' in raw}
mapping = {}
for old in sorted(found):
    mapping[old] = explicit.get(old, old.lower().replace(".", "-"))
taken = set(found) | bones
for old, new in list(mapping.items()):
    if not SLUG.match(new): print("INVALID", old, new); sys.exit(1)
news = list(mapping.values())
dups = {n for n in news if news.count(n) > 1}
if dups: print("COLLISION among new ids", dups); sys.exit(1)
def volume_span(text):
    i = text.find('"volume": {')
    if i < 0: return None
    j = text.index("{", i)
    depth, k, instr, esc = 0, j, False, False
    while True:
        c = text[k]
        if instr:
            if esc: esc = False
            elif c == "\\": esc = True
            elif c == '"': instr = False
        else:
            if c == '"': instr = True
            elif c == "{": depth += 1
            elif c == "}":
                depth -= 1
                if depth == 0: return j, k + 1
        k += 1
_span = volume_span(raw)
raw_nv = raw if _span is None else raw[:_span[0]] + raw[_span[1]:]
clash = sorted(n for n in news if '"' + n + '"' in raw_nv and n not in mapping)
if clash: print("NEW ID ALREADY IN FILE", clash[:10]); sys.exit(1)
print(f"{spec}: {len(mapping)} ids")
for old, new in mapping.items(): print(f"  {old} -> {new}" + ("  (bone)" if old in bones else ""))
if not apply: sys.exit(0)
def volume_refs(vol):
    """Follows the renamed ids in the references of the volume (bone, weights bone, region, regions, material), never in the solid ids."""
    for old, new in mapping.items():
        vol = re.sub(r'("(?:bone|region|material)":\s*)"' + re.escape(old) + '"', lambda m: m.group(1) + '"' + new + '"', vol)
        vol = re.sub(r'("regions":\s*\[)([^\]]*)(\])', lambda m: m.group(1) + re.sub('"' + re.escape(old) + '"', '"' + new + '"', m.group(2)) + m.group(3), vol)
    return vol
STR = re.compile(r'"((?:[^"\\]|\\.)*)"')
def rename_string(m, only):
    body = m.group(1)
    ps = body.split(":")
    out = []
    changed = False
    for p in ps:
        if p in mapping and (only is None or p in only):
            out.append(mapping[p]); changed = True
        else: out.append(p)
    return '"' + ":".join(out) + '"' if changed else m.group(0)
def transform(text):
    span = volume_span(text)
    before, vol, after = (text, None, "") if span is None else (text[:span[0]], text[span[0]:span[1]], text[span[1]:])
    before = STR.sub(lambda m: rename_string(m, None), before)
    after = STR.sub(lambda m: rename_string(m, None), after) if after else after
    if vol is not None:
        vol = volume_refs(vol)
    return before + (vol or "") + after
new_raw = transform(raw)
if open(path, encoding="utf-8").read() != raw: print("file changed meanwhile, rerun"); sys.exit(1)
open(path, "w", encoding="utf-8").write(new_raw)
json.loads(new_raw)
cat = spec.split("/")[0].lstrip("🌳️🏗️🐕️👤️🔌️🚜️🛋️🧱️🪑️")
src = os.path.join(T, "volumes", cat, spec.split("/")[1] + ".json")
if os.path.exists(src):
    t = open(src, encoding="utf-8").read()
    t = volume_refs(t)
    open(src, "w", encoding="utf-8").write(t)
    print("  synced", src)
print("written")
