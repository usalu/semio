# One-shot source refactor of the gltf inferences package (r5c): leaf contract moved down, encode_result over the measure, per-leaf registry descriptors.
import re, os, sys, glob, shutil
REPO = "/Users/ueli/Documents/semio"
F = REPO + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
BACKUP = F + "/🗑️generated/r5c-orig/inferences"
ANY = REPO + "/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any"
INF = ANY + "/🧬️schema/💡️inferences"
CONTRACTS = ANY + "/🔨️modules/🧾️measurement-contracts/🦀️.rs"
DRY = "--dry" in sys.argv

def reindent(body):
    rows = body.split(chr(10))
    out = [rows[0]]
    for row in rows[1:]:
        stripped = row.strip()
        out.append(("        " if stripped.startswith("}") else "            ") + stripped)
    return chr(10).join(out)

def read(p): return open(p, encoding="utf8").read()
def write(p, t):
    if DRY: return
    rel = os.path.relpath(p, REPO)
    b = os.path.join(BACKUP, rel)
    if not os.path.exists(b):
        os.makedirs(os.path.dirname(b), exist_ok=True)
        shutil.copy2(p, b)
    open(p, "w", encoding="utf8").write(t)

comp_path = INF + "/🦀️.rs"
comp = read(comp_path)
assert "pub fn inference_descriptor" not in comp

leaves = []
for fam in sorted(glob.glob(INF + "/*/🦀️.rs")):
    d = os.path.dirname(fam)
    if os.path.basename(d) in ("🔨️geometry-kernel", "🏗️dag-assembly", "🧪️tests", "📦️packages"): continue
    t = read(fam)
    for m in re.finditer(r'#\[path = "([^"]+)/🦀️\.rs"\]\s*pub mod (\w+);', t):
        leaves.append((os.path.basename(d), m.group(1), m.group(2)))
assert len(leaves) == 67, len(leaves)

ENC = re.compile(r'pub fn encode_result\(indicators: &GltfEntityIndicators\) -> semio_framework_value::DslValue \{\s*semio_framework_value::ToValue::to_value\(&indicators\.(\w+)\.(\w+)\)\s*\}')
UNAV = re.compile(r'pub fn unavailable_measure\([^)]*\)\s*->\s*([^{]+)\{')
mod_info = {}
for fam, folder, mod in leaves:
    p = f"{INF}/{fam}/{folder}/🦀️.rs"
    t = read(p)
    e = ENC.search(t); u = UNAV.search(t)
    assert e and u, p
    assert mod not in mod_info, mod
    mod_info[mod] = dict(path=p, fam=fam, folder=folder, field=(e.group(1), e.group(2)), ty=u.group(1).strip())

# component: descriptor entries
i = comp.index("pub fn gltf_artifact_inference_descriptors()")
start_fn = comp.rfind("// 🚫️async", 0, i)
j = comp.index("\n}\n", i) + 3
fn_text = comp[start_fn:j]
entries = re.findall(r'semio_framework_schema_registry::ArtifactInferenceDescriptor \{\s*id: "([^"]+)",\s*inference: (semio_framework_schema_registry::FacetLeaves \{.*?\n\s*\}),\n\s*\},', fn_text, re.S)
assert len(entries) == 67, len(entries)
by_leaf = {}
for id_, body in entries:
    paths = re.findall(r'include_str!\("([^"]+)"\)', body)
    folders = {p.rsplit("/", 1)[0] for p in paths}
    assert len(folders) == 1, (id_, folders)
    folder = folders.pop()
    by_leaf[folder] = (id_, body)

order = []
for fam, folder, mod in leaves:
    key = f"{fam}/{folder}"
    assert key in by_leaf, key
    order.append((mod, key))

# 1. contract move
blk_start = comp.index("pub const GLTF_GEOMETRY_READS")
blk_end = comp.index("#[derive(Clone, Copy)]\npub struct GltfInferenceLeafServiceDescriptor")
moved = comp[blk_start:blk_end]
assert "pub trait GltfInferenceLeaf" in moved and "pub struct GltfInferenceLeafDescriptor" in moved
comp = comp[:blk_start] + comp[blk_end:]
contracts = read(CONTRACTS)
anchor = "//#endregion 🧾️MeasurementContracts"
assert contracts.count(anchor) == 1
contracts = contracts.replace(anchor, "//#region 🧭️LeafContract\n" + moved.rstrip() + "\n//#endregion 🧭️LeafContract\n" + anchor)
write(CONTRACTS, contracts)

# 2. service table
def table_sub(m):
    mod = m.group(1)
    fam, field = mod_info[mod]["field"]
    return f"encode: |indicators| {mod}::encode_result(&indicators.{fam}.{field})"
comp, n = re.subn(r'encode: (\w+)::encode_result', table_sub, comp)
assert n == 67, n

# 3. descriptors list
calls = "\n".join(f"        {mod}::inference_descriptor()," for mod, _ in order)
new_fn = "// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9\npub fn gltf_artifact_inference_descriptors() -> Vec<semio_framework_schema_registry::ArtifactInferenceDescriptor> {\n    vec![\n" + calls + "\n    ]\n}\n"
comp = comp.replace(fn_text, new_fn)
write(comp_path, comp)

# 4. leaves
DROP = {"GltfEntityIndicators", "GltfInferenceLeaf", "GltfInferenceLeafDescriptor", "GLTF_GEOMETRY_READS"}
def split_top(body):
    items, depth, cur = [], 0, ""
    for ch in body:
        if ch == "{": depth += 1
        if ch == "}": depth -= 1
        if ch == "," and depth == 0:
            items.append(cur.strip()); cur = ""
        else: cur += ch
    if cur.strip(): items.append(cur.strip())
    return items
def rewrite_imports(t):
    out, at = "", 0
    marker = "use super::super::{"
    while True:
        i = t.find(marker, at)
        if i < 0: break
        j, depth = i + len(marker), 1
        while depth:
            depth += {"{": 1, "}": -1}.get(t[j], 0); j += 1
        end = t.index(";", j)
        items = split_top(t[i + len(marker): j - 1].replace(chr(10), " "))
        kept = [re.sub(r"\s+", " ", x) for x in items if x not in DROP]
        out += t[at:i]
        if len(kept) == len(items): out += t[i:end + 1]
        elif not kept: out += ""
        elif len(kept) == 1: out += "use super::super::" + kept[0] + ";"
        else: out += "use super::super::{" + ", ".join(kept) + "};"
        at = end + 1
    return out + t[at:]
for mod, key in order:
    info = mod_info[mod]
    t = read(info["path"])
    t = rewrite_imports(t)
    t = re.sub(r'^\n(?=use )', '', t, flags=re.M) if False else t
    t, n = ENC.subn(f"pub fn encode_result(measure: &{info['ty']}) -> semio_framework_value::DslValue {{\n    semio_framework_value::ToValue::to_value(measure)\n}}", t)
    assert n == 1
    t = t.replace("pub(crate) fn infer(", "pub fn infer(")
    id_, body = by_leaf[key]
    body = re.sub(r'include_str!\("(?:[^"/]+/)*([^"/]+)"\)', lambda m: f'include_str!("{m.group(1)}")', body)
    region = ("//#region 🧾️InferenceDescriptor\n"
              "/// 🧾️ Registry descriptor of this inference leaf; its companion documents are embedded from this leaf's own folder.\n"
              "pub fn inference_descriptor() -> semio_framework_schema_registry::ArtifactInferenceDescriptor {\n"
              "    semio_framework_schema_registry::ArtifactInferenceDescriptor {\n"
              f"        id: \"{id_}\",\n"
              f"        inference: {reindent(body)},\n"
              "    }\n}\n"
              "//#endregion 🧾️InferenceDescriptor\n\n")
    at = t.find("#[cfg(test)]")
    t = (t[:at] + region + t[at:]) if at >= 0 else (t.rstrip() + "\n\n" + region.rstrip() + "\n")
    write(info["path"], t)
print("ok", len(order))
