"""🪚 FB carve, stage 1: cuts the engine sources into the kernel-contract, mesh-io, shape-value, shape-exchange, test and engine files.
Reads only fb-tools/orig/engine (the snapshot taken before the first edit) and writes the live tree after asserting the live files still equal the snapshot.
usage: carve.py [--dry]   (--dry writes into /Users/ueli/.cache/fb/dry instead of the repo)"""
import os, re, sys, hashlib
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
from items import split_items

R = "/Users/ueli/Documents/semio"
E_REL = "🧰️framework/🔨️modules/🧊️3d/📐️brep/⚙️engine"
LIVE = f"{R}/{E_REL}"
ORIG = f"{HERE}/orig/engine"
DRY = "--dry" in sys.argv
OUT = "/Users/ueli/.cache/fb/dry/engine" if DRY else LIVE


def rd(rel, base=ORIG):
    with open(f"{base}/{rel}", encoding="utf-8") as f:
        return f.read()


import json as _j
STATE_PATH = f"{HERE}/carve-state.json"
STATE = _j.load(open(STATE_PATH, encoding="utf-8")) if os.path.exists(STATE_PATH) else {}


def sha(b):
    return hashlib.sha256(b).hexdigest()


def guard(rel):
    """a live file may be overwritten only when it still equals the snapshot or the text this tool wrote last (nobody else touched it)."""
    p = f"{LIVE}/{rel}"
    if not os.path.exists(p):
        return
    cur = sha(open(p, "rb").read())
    o = f"{ORIG}/{rel}"
    ok = {STATE.get(rel)}
    if os.path.exists(o):
        ok.add(sha(open(o, "rb").read()))
    assert cur in ok, f"live file changed by someone else: {rel}"


def wr(rel, text):
    if not DRY:
        guard(rel)
    p = f"{OUT}/{rel}"
    rp = os.path.realpath(os.path.dirname(p)) if os.path.exists(os.path.dirname(p)) else None
    os.makedirs(os.path.dirname(p), exist_ok=True)
    real = os.path.realpath(os.path.dirname(p))
    assert real.startswith(os.path.realpath(OUT)), (p, real)
    with open(p, "w", encoding="utf-8") as f:
        f.write(text)
    if not DRY:
        STATE[rel] = sha(text.encode())
        with open(STATE_PATH, "w", encoding="utf-8") as f:
            _j.dump(STATE, f, ensure_ascii=False, indent=1)
    print("wrote", rel, len(text.encode()))


def once(text, anchor):
    assert text.count(anchor) == 1, f"anchor count {text.count(anchor)}: {anchor[:80]!r}"
    return anchor


def cut(text, start_anchor, end_anchor, keep_end=True):
    """removes [start_anchor, end_anchor) or through end_anchor when keep_end is False; returns (new_text, removed_text)."""
    once(text, start_anchor)
    once(text, end_anchor)
    a, b = text.index(start_anchor), text.index(end_anchor)
    assert a < b
    if not keep_end:
        b += len(end_anchor)
    return text[:a] + text[b:], text[a:b]


if not DRY and not STATE:
    for rel in ["🦀️.rs", "📦️mesh-io/🦀️.rs", "🪅️shape-value/🦀️.rs", "🔎️analysis-session/🦀️.rs", "⏱️operation-jobs/🦀️.rs", "🧪️tests/🔬️unit/🦀️.rs", "⏱️operation-jobs/🧪️tests/🔬️unit/🦀️.rs", "🪅️shape-value/🧪️tests/🔬️unit/🦀️.rs", "🔎️analysis-session/🧪️tests/🔬️unit/🦀️.rs", "📦️mesh-io/🧪️tests/🔬️unit/🦀️.rs"]:
        assert rd(rel, LIVE) == rd(rel), f"live file changed since the snapshot: {rel}"

root = rd("🦀️.rs")

HDR_DOC = root[: root.index("extern crate")]

# ---------------------------------------------------------------------------------------------------------------- K
TYPE_DOC = "/// 🧱 Vertex positions with outer and inner vertex loops for each face.\npub type SolidFaceLoops = (Vec<[f32; 3]>, Vec<(Vec<u32>, Vec<Vec<u32>>)>);\n"
once(root, TYPE_DOC)
_, contract_regions = cut(root, "// #region 🔖️ContractTypes", "// #region 🔖️Kernel", True)
_, kernel_region = cut(root, "// #region 🔖️Kernel", "// #endregion 🔖️Kernel\n", False)
MAP_ERR = "// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9\nfn map_err(e: &KernelError) -> BrepError {\n    BrepError::Operation(e.to_string())\n}\n"
once(root, MAP_ERR)
k_map_err = "/// 🧭️ Reports a kernel error the way every one-shot operation of the engine always has: as [`BrepError::Operation`] carrying its message.\n" + MAP_ERR.replace("fn map_err", "pub fn map_err")

K_DOC = (
    "//! 🤝 Contract of the 3D kernel's B-Rep interface: the geometry handle and error vocabulary and the model-free synchronous\n"
    "//! [`BrepKernel`] trait that the arena-backed engine implements.\n"
    "//!\n"
    "//! Crate cut of `semio-framework-3d-brep-engine` (carve FB, `🪓️monolith-split/RESULT-CRATE-CARVE-FB.md`).\n\n"
)
k_text = (
    K_DOC
    + "extern crate semio_framework_value_derive as value_derive;\n"
    + TYPE_DOC
    + "\nuse semio_framework_3d_brep_representation::contract::*;\nuse semio_framework_3d_brep_representation::error::KernelError;\n\n"
    + contract_regions.rstrip("\n") + "\n\n"
    + kernel_region.rstrip("\n") + "\n\n"
    + "// #region 🧮Convert\n\n" + k_map_err + "\n// #endregion 🧮Convert\n\n"
    + '#[cfg(test)]\n#[path = "🧪️tests/🔬️unit/🦀️.rs"]\nmod tests;\n'
)

# ---------------------------------------------------------------------------------------------------------------- M
mesh = rd("📦️mesh-io/🦀️.rs")
once(mesh, "    pub(super) active: bool,\n")
mesh = mesh.replace("    pub(super) active: bool,\n", "    active: bool,\n")
once(mesh, "    pub(super) fn cleanup_complete(&self)->bool {self.retired}\n")
mesh = mesh.replace(
    "    pub(super) fn cleanup_complete(&self)->bool {self.retired}\n",
    "    pub fn cleanup_complete(&self)->bool {self.retired}\n"
    "    /// 🔒️ True while a session has registered this import as an open pending mutation.\n"
    "    pub fn is_active(&self)->bool {self.active}\n"
    "    /// 🔒️ Records that a session registered or released this import as an open pending mutation.\n"
    "    pub fn set_active(&mut self,active:bool) {self.active=active;}\n",
)
MESH_FN_START = "/// 📦 Converts a tessellation [`MeshTransfer`] into `semio-framework-mesh-engine`'s `MeshData`.\n"
_, mesh_fn = cut(root, MESH_FN_START, "// #endregion 🧮Convert\n", True)
mesh_fn = mesh_fn.rstrip("\n") + "\n"
mesh_fn_clean = mesh_fn
once(mesh, "    TriangleMesh { positions, normals, indices: data.indices.clone() }\n}\n\n// #endregion 🔖️Convert\n")
mesh = mesh.replace("    TriangleMesh { positions, normals, indices: data.indices.clone() }\n}\n\n// #endregion 🔖️Convert\n",
                    "    TriangleMesh { positions, normals, indices: data.indices.clone() }\n}\n\n" + mesh_fn_clean + "\n// #endregion 🔖️Convert\n")
once(mesh, "use semio_framework_3d_brep_construction::euler::{add_shell, add_solid};\n")
mesh = mesh.replace("use semio_framework_3d_brep_construction::euler::{add_shell, add_solid};\n",
                    "use semio_framework_3d_brep_construction::euler::{add_shell, add_solid};\nuse semio_framework_3d_brep_kernel_contract::BrepError;\n")
once(mesh, '#[cfg(test)]\n#[path = "🧪️tests/🔬️unit/🦀️.rs"]\nmod tests;\n')
mesh = mesh.replace('#[cfg(test)]\n#[path = "🧪️tests/🔬️unit/🦀️.rs"]\nmod tests;\n',
                    '#[cfg(test)]\n#[path = "🧪️tests/🔬️unit/🦀️.rs"]\nmod tests;\n\n#[cfg(test)]\n#[path = "🧪️tests/🖼️preview/🦀️.rs"]\nmod preview_tests;\n')

# ---------------------------------------------------------------------------------------------------------------- S and shape exchange
shape = rd("🪅️shape-value/🦀️.rs")
SV_USE = "use super::{map_err, entity_roots, label_of_entity, Brep, BrepError, Entity, GeometryHandle, GeometryKind, MeshTransfer};\n"
once(shape, SV_USE)
LABEL_HW_DOC = "/// 🎯️ Highest label carried by a body's entities or the root, plus one.\n"
once(shape, LABEL_HW_DOC)
head, tail = shape.split(LABEL_HW_DOC)
TESTS_MOUNT = '#[cfg(test)]\n#[path = "🧪️tests/🔬️unit/🦀️.rs"]\nmod tests;\n'
once(tail, TESTS_MOUNT)
exchange_body = tail.replace("\n" + TESTS_MOUNT, "").rstrip("\n") + "\n"
assert TESTS_MOUNT not in exchange_body
s_text = head.replace(SV_USE,
    "use semio_framework_3d_brep_kernel_contract::{map_err, BrepError, GeometryHandle, GeometryKind};\n"
    "use semio_framework_3d_brep_representation::contract::MeshTransfer;\n")
s_text = s_text.rstrip("\n") + "\n"
assert s_text.startswith("//! 🪅️")
once(s_text, "use semio_framework_3d_brep_representation::topology::Wire;\n")
s_text = s_text.replace("use semio_framework_3d_brep_representation::topology::Wire;\n", "extern crate semio_framework_value_derive as value_derive;\nuse semio_framework_3d_brep_representation::topology::Wire;\n", 1)

x_text = (
    "//! 🚚 Session side of the standalone shape value: [`Brep::export_shape`] captures the closure of one live handle as an owned\n"
    "//! [`ShapeValue`] and [`Brep::import_shape`] / [`Brep::import_shape_mapped`] registers one in a session again.\n\n"
    "use super::{entity_roots, label_of_entity, Brep, BrepError, Entity, GeometryHandle, GeometryKind};\n"
    "use semio_framework_3d_brep_representation::topology::history::{LabelSource, PersistentLabel};\n"
    "use semio_framework_3d_brep_representation::topology::{Body, Wire};\n"
    "use semio_framework_3d_brep_shape_value::{ImportedShape, ShapeRoot, ShapeValue, ShapeWire};\n\n"
    + LABEL_HW_DOC + exchange_body
    + '\n#[cfg(test)]\n#[path = "🧪️tests/🔬️unit/🦀️.rs"]\nmod tests;\n'
)

# ---------------------------------------------------------------------------------------------------------------- engine root
new_root = root
new_root = new_root.replace("extern crate semio_framework_value_derive as value_derive;\n", "", 1)
new_root = new_root.replace(TYPE_DOC, "", 1)
old_mesh_mount = '#[path = "📦️mesh-io/🦀️.rs"]\nmod mesh_io;\npub use mesh_io::{MeshImportCursor, MeshImportStep};\n'
once(new_root, old_mesh_mount)
new_root = new_root.replace(old_mesh_mount, "pub use semio_framework_3d_brep_mesh_io::{mesh_data_from_mesh_transfer, MeshImportCursor, MeshImportStep};\n")
old_shape_mount = '#[path = "🪅️shape-value/🦀️.rs"]\nmod shape_value;\npub use shape_value::{ImportedShape, ShapeComponent, ShapeRoot, ShapeTessellationJob, ShapeValue, ShapeWire};\n'
once(new_root, old_shape_mount)
new_root = new_root.replace(old_shape_mount,
    '#[path = "🚚️shape-exchange/🦀️.rs"]\nmod shape_exchange;\npub use semio_framework_3d_brep_shape_value::{ImportedShape, ShapeComponent, ShapeRoot, ShapeTessellationJob, ShapeValue, ShapeWire};\n'
    "pub use semio_framework_3d_brep_kernel_contract::{BrepError, BrepKernel, BrepTopology, ClosestPoint, GeometryHandle, GeometryKind, SolidFaceLoops};\n"
    "use semio_framework_3d_brep_kernel_contract::map_err;\n")
use_mesh = re.search(r"^use mesh_io::\{([^}]*)\};\n", new_root, re.M)
assert use_mesh
names = [n.strip() for n in use_mesh.group(1).split(",")]
for gone in ("mesh_to_mesh_data", "triangle_mesh_from_transfer"):
    names.remove(gone)
new_root = new_root.replace(use_mesh.group(0), "use semio_framework_3d_brep_mesh_io::{" + ", ".join(names) + "};\n")
new_root, _ = cut(new_root, "// #region 🔖️ContractTypes", "#[path = \"🧹️retirement/🦀️.rs\"]", True)
once(new_root, MAP_ERR)
new_root = new_root.replace(MAP_ERR, "")
new_root, _ = cut(new_root, MESH_FN_START, "// #endregion 🧮Convert\n", True)
for old, new in [
    ("if budget>0 && !cursor.active && !cursor.retirement_complete() {cursor.active=true;self.pending_mutations+=1;}",
     "if budget>0 && !cursor.is_active() && !cursor.retirement_complete() {cursor.set_active(true);self.pending_mutations+=1;}"),
]:
    once(new_root, old)
    new_root = new_root.replace(old, new)
old_close = "if cursor.cleanup_complete() && cursor.active {cursor.active=false;self.pending_mutations=self.pending_mutations.checked_sub(1).expect(\"owned import mutation\");}"
assert new_root.count(old_close) == 2
new_root = new_root.replace(old_close, "if cursor.cleanup_complete() && cursor.is_active() {cursor.set_active(false);self.pending_mutations=self.pending_mutations.checked_sub(1).expect(\"owned import mutation\");}")

# ---------------------------------------------------------------------------------------------------------------- tests
def parts_of(rel):
    parts = split_items(rd(rel))
    assert "".join(p[2] for p in parts).rstrip("\n") == rd(rel).rstrip("\n")
    return parts


def pick(parts, names):
    got = []
    for kind, name, text in parts:
        if kind == "item" and name in names:
            got.append(text)
    missing = [n for n in names if n not in {p[1] for p in parts if p[0] == "item"}]
    assert not missing, missing
    return got


def prefix_match(parts, prefix):
    hits = [p[1] for p in parts if p[0] == "item" and p[1].startswith(prefix)]
    assert len(hits) == 1, (prefix, hits)
    return hits[0]


unit = parts_of("🧪️tests/🔬️unit/🦀️.rs")
WB = [prefix_match(unit, p) for p in (
    "brep_selected_solid_operations_require_scoped_components_and_positive_",
    "brep_deconstruction_covers_every_topological_shape_kind",
    "brep_closest_point_honors_curved_face_trims",
    "dispose_reclaims_unreferenced_topology",
    "the_validate_gate_judges_the_shape_it_was_asked_about_and_not_the_aren",
)]
K_TESTS = ["brep_error_contract_is_owned_and_stable"]
M_TESTS = [prefix_match(unit, p) for p in (
    "brep_preview_component_reference_tables_refuse_missing_picking_buffers",
    "brep_preview_component_ranges_refuse_incomplete_or_overlapping_groups",
    "brep_merged_preview_preserves_original_component_ranges",
    "brep_original_vertex_ranges_refuse_malformed_coverage_and_labels",
)]
moved = set(WB) | set(K_TESTS) | set(M_TESTS)


def join_items(texts):
    return "\n".join(t.rstrip("\n") + "\n" for t in texts)


def rebuild(parts, keep):
    out = []
    for kind, name, text in parts:
        if kind == "item" and name in keep:
            out.append(text)
        elif kind in ("marker",):
            out.append(text)
    return out


wb_unit = join_items(["use super::*;\n"] + pick(unit, WB))
k_test = join_items(["use super::*;\n"] + pick(unit, K_TESTS))
m_test = join_items(["use super::*;\n"] + pick(unit, M_TESTS))
t_api_items = [text for kind, name, text in unit if (kind == "item" and name not in moved and name != "use super::*;") or kind == "marker"]
t_api = join_items(t_api_items)

ops = parts_of("⏱️operation-jobs/🧪️tests/🔬️unit/🦀️.rs")
ops_wb_name = prefix_match(ops, "extract_and_absorb_isolate_a_working_copy")
SNAP_PRIV = ("/// 📸️ Everything observable about a session: the whole body, the live handle set and the pending mutations. Two equal\n"
             "/// snapshots mean the session is byte-identical.\n"
             "fn snapshot(kernel: &Brep) -> String {\n"
             "    format!(\"{:?}|{:?}|{}\", kernel.body, kernel.live.keys().collect::<Vec<_>>(), kernel.pending_mutations)\n"
             "}\n")
ops_snapshot = [p for p in ops if p[0] == "item" and p[1] == "snapshot"][0][2]
once(ops_snapshot, "fn snapshot(kernel: &Brep) -> String {")
SNAP_PUB = ("/// 📸️ Everything observable about a session through its public surface: the whole body and the live handle set. Two equal\n"
            "/// snapshots mean the session is byte-identical.\n"
            "fn snapshot(kernel: &Brep) -> String {\n"
            "    let mut live = kernel.live_handles().into_iter().collect::<Vec<_>>();\n"
            "    live.sort();\n"
            "    format!(\"{:?}|{:?}\", kernel.representation(), live)\n"
            "}\n")

shp = parts_of("🪅️shape-value/🧪️tests/🔬️unit/🦀️.rs")
shp_wb_name = prefix_match(shp, "shape_value_import_refuses_dangling_references_without_mutation")
ana = parts_of("🔎️analysis-session/🧪️tests/🔬️unit/🦀️.rs")
ana_wb_name = prefix_match(ana, "validate_judges_the_selected_shape_not_the_whole_session")

if False:
    print("K", len(k_text.encode()), "M", len(mesh.encode()), "S", len(s_text.encode()), "X", len(x_text.encode()), "root", len(new_root.encode()))
    print("WB root tests", WB)
    print("moved to M", M_TESTS)


# ---------------------------------------------------------------------------------------------------------------- stage 2: tests
sys.path.insert(0, f"{HERE}/pos")
from posstub import expand_use, mask as _mask

CONTRACT_NAMES = ["Vec3", "Aabb", "ParamDomain", "FaceGroup", "EdgeGroup", "VertexGroup", "SurfaceKind", "CurveKind", "FaceInfo", "EdgeInfo", "MeshTransfer", "PointClassification", "OpQuality", "BREP_KERNEL_OPERATIONS", "operation_quality"]
ENGINE_API = ["Brep", "BrepBooleanAdmission", "BrepBooleanJob", "BrepBooleanStep", "BrepError", "BrepKernel", "BrepOperation", "BrepOperationAdmission", "BrepOperationJob", "BrepOperationProgress", "BrepOperationStep", "BrepRetentionJob", "BrepTopology", "ClosestPoint", "GeometryHandle", "GeometryKind", "ImportedShape", "MeshImportCursor", "MeshImportStep", "ShapeComponent", "ShapeRoot", "ShapeTessellationJob", "ShapeValue", "ShapeWire", "SolidExporter", "SolidImporter", "SolidFaceLoops", "StlSolidExporter", "StlSolidImporter", "ObjSolidExporter", "ObjSolidImporter", "GlbSolidExporter", "GlbSolidImporter", "mesh_data_from_mesh_transfer"]


def root_use_leaves():
    leaves = []
    for m in re.finditer(r"^(?:#\[cfg\(test\)\]\s*)?(use\s+[^;]+;)", root, re.M):
        stmt = m.group(1)
        if "mesh_io" in stmt or stmt.startswith("use super") or "testing_support" in stmt:
            continue
        tree = stmt[len("use"):].strip().rstrip(";").strip()
        for kind, segs in expand_use(tree):
            if kind == "glob":
                if segs[-1] == "contract" and segs[-2].endswith("representation"):
                    for n in CONTRACT_NAMES:
                        leaves.append(("semio_framework_3d_brep_representation::contract", n, n))
                continue
            if kind != "item":
                continue
            alias = segs[-1]
            m2 = re.search(r"\bas\s+(\w+)", tree)
            leaves.append(("::".join(segs[:-1]), segs[-1], alias))
    aliased = []
    for m in re.finditer(r"(\w+(?:::\w+)*)::(\w+) as (\w+)", root):
        aliased.append((m.group(1), m.group(2), m.group(3)))
    return leaves, aliased


def leaf_candidates():
    leaves, aliased = root_use_leaves()
    out = {}
    for path, name, alias in leaves:
        out.setdefault(alias, (path, name))
    for path, name, alias in aliased:
        if path.startswith("semio_framework"):
            out[alias] = (path, name)
    for n in ENGINE_API:
        out[n] = ("semio_framework_3d_brep_engine", n)
    for n in ("Wire", "Body", "EntityRef", "OpRecorder", "PersistentLabel"):
        pass
    return out


def split_top_commas(text):
    parts, cur, depth = [], "", 0
    for ch in text:
        if ch == "{":
            depth += 1
        if ch == "}":
            depth -= 1
        if ch == "," and depth == 0:
            parts.append(cur)
            cur = ""
        else:
            cur += ch
    if cur.strip():
        parts.append(cur)
    return [p.strip() for p in parts if p.strip()]


def use_leaves(tree, prefix=()):
    """[(path_segments, name, alias)] of a use tree; `self` leaves become module leaves."""
    tree = tree.strip()
    m = re.match(r"^(.*?)::\s*\{(.*)\}$", tree, re.S)
    if m:
        out = []
        for part in split_top_commas(m.group(2)):
            out += use_leaves(part, prefix + tuple(x for x in m.group(1).strip().split("::") if x))
        return out
    if tree.startswith("{") and tree.endswith("}"):
        out = []
        for part in split_top_commas(tree[1:-1]):
            out += use_leaves(part, prefix)
        return out
    am = re.match(r"^(.*?)\s+as\s+(\w+)$", tree, re.S)
    alias = None
    if am:
        tree, alias = am.group(1).strip(), am.group(2)
    segs = prefix + tuple(x for x in tree.split("::") if x)
    if segs and segs[-1] == "*":
        return []
    if segs and segs[-1] == "self":
        segs = segs[:-1]
    if not segs:
        return []
    return [(segs[:-1], segs[-1], alias or segs[-1])]


def module_cands(rel):
    out = {}
    text = rd(rel)
    for m in re.finditer(r"^(?:#\[cfg\(test\)\]\s*)?use\s+([^;]+);", text, re.M):
        tree = m.group(1)
        if tree.strip().startswith("super::") or tree.strip().startswith("crate::"):
            continue
        for path, name, alias in use_leaves(tree):
            out[alias] = ("::".join(path), name)
    return out


CANDS = leaf_candidates()
CANDS.pop("mesh_to_mesh_data", None)
CANDS.pop("triangle_mesh_from_transfer", None)


def uses_name(body_masked, name):
    return re.search(r"(?<![A-Za-z0-9_:])" + re.escape(name) + r"\b", body_masked) is not None


import json as _json
EXTRA = _json.load(open(f"{HERE}/extra-imports.json", encoding="utf-8")) if os.path.exists(f"{HERE}/extra-imports.json") else {}


def build_header(body, extra_leaves=(), skip=(), cands=None, force=()):
    """explicit `use` lines for exactly the candidate names the body mentions."""
    bm = _mask(body)
    groups = {}
    merged = dict(CANDS)
    merged.update(cands or {})
    for alias, (path, name) in sorted(merged.items()):
        if alias in skip:
            continue
        if re.search(r"\bfn\s+" + re.escape(alias) + r"\b|\bstruct\s+" + re.escape(alias) + r"\b|\bconst\s+" + re.escape(alias) + r"\b", bm):
            continue
        if (uses_name(bm, alias) and (path or re.search(r"(?<![A-Za-z0-9_:])" + re.escape(alias) + r"::", bm))) or alias in force:
            groups.setdefault(path, []).append((name, alias))
    for path, name, alias in extra_leaves:
        if uses_name(bm, alias):
            groups.setdefault(path, []).append((name, alias))
    lines = []
    for path in sorted(groups):
        items = sorted(set(groups[path]))
        names = [n if n == a else f"{n} as {a}" for n, a in items]
        if not path:
            lines += [f"use {n};" for n in names]
        else:
            lines.append(f"use {path}::{names[0]};" if len(names) == 1 else f"use {path}::{{{', '.join(names)}}};")
    return "\n".join(lines) + ("\n" if lines else "")


def keep_use_items(parts, body, only=None):
    """original `use` items of a test file (super globs dropped); every leaf stays (trait imports are used without being named) unless `only` lists the leaves to keep."""
    bm = _mask(body)
    groups = {}
    for kind, name, text in parts:
        if kind != "item" or not text.lstrip().startswith("use "):
            continue
        tree = text.strip()[len("use"):].strip().rstrip(";").strip()
        if tree.startswith("super::") or tree.startswith("crate::"):
            continue
        for k, segs in expand_use(tree):
            if k != "item":
                continue
            alias = segs[-1]
            if only is None or alias in only:
                groups.setdefault("::".join(segs[:-1]), []).append(segs[-1])
    lines = []
    for path in sorted(groups):
        names = sorted(set(groups[path]))
        lines.append(f"use {path}::{names[0]};" if len(names) == 1 else f"use {path}::{{{', '.join(names)}}};")
    return "\n".join(lines) + ("\n" if lines else "")


def kept_aliases(text):
    out = set()
    for line in text.split("\n"):
        if line.startswith("use "):
            for k, segs in expand_use(line[4:].rstrip(";")):
                if k == "item":
                    out.add(segs[-1])
    return out


def with_header(parts, body, skip=(), only=None, cands=None, force=()):
    kept = keep_use_items(parts, body, only)
    return build_header(body, skip=tuple(skip) + tuple(kept_aliases(kept)), cands=cands, force=force) + kept


def body_items(parts, drop):
    out = []
    for kind, name, text in parts:
        if kind == "item" and (text.lstrip().startswith("use ") or name in drop):
            continue
        out.append(text)
    return out


# T: black-box root tests
api_body = join_items(body_items(unit, set(WB) | set(K_TESTS) | set(M_TESTS)))
t_api_file = build_header(api_body, force=EXTRA.get("api", [])) + "\n" + api_body.lstrip("\n")

# T: operation jobs
OPS_DROP = {ops_wb_name, "snapshot"}
ops_items = []
for kind, name, text in ops:
    if kind == "item" and (text.lstrip().startswith("use ") or name in OPS_DROP):
        continue
    ops_items.append(text)
ops_body = join_items(ops_items)
once(ops_body, '"../../../🧪️tests/🧫️fixtures/⏱️operation-jobs/🔣️.json"')
ops_body = ops_body.replace('"../../../🧪️tests/🧫️fixtures/⏱️operation-jobs/🔣️.json"', '"../🧫️fixtures/⏱️operation-jobs/🔣️.json"')
once(ops_body, "        let body = &scene.kernel.body;\n")
ops_body = ops_body.replace("        let body = &scene.kernel.body;\n", "        let body = scene.kernel.representation();\n")
# the public snapshot sits where the private one did: before `Scene`
SNAP_ANCHOR = "/// 🧱️ One built case: input handles by fixture key.\n"
once(ops_body, SNAP_ANCHOR)
ops_body = ops_body.replace(SNAP_ANCHOR, SNAP_PUB + "\n" + SNAP_ANCHOR)
t_ops_file = with_header(ops, ops_body, cands=module_cands("⏱️operation-jobs/🦀️.rs"), force=EXTRA.get("ops", [])) + "\n" + ops_body.lstrip("\n")

# T: shape values
SHP_DROP = {shp_wb_name}
shp_body = join_items(body_items(shp, SHP_DROP))
once(shp_body, '"../../../🧪️tests/🧫️fixtures/🪬️shape-values/🔣️.json"')
shp_body = shp_body.replace('"../../../🧪️tests/🧫️fixtures/🪬️shape-values/🔣️.json"', '"../🧫️fixtures/🪬️shape-values/🔣️.json"')
t_shp_file = with_header(shp, shp_body, cands=module_cands("🪅️shape-value/🦀️.rs"), force=EXTRA.get("shp", [])) + "\n" + shp_body.lstrip("\n")

# T: analysis
ANA_DROP = {ana_wb_name}
ana_body = join_items(body_items(ana, ANA_DROP))
t_ana_file = with_header(ana, ana_body, cands=module_cands("🔎️analysis-session/🦀️.rs"), force=EXTRA.get("ana", [])) + "\n" + ana_body.lstrip("\n")

# engine: white-box files
_ops_wb_body = join_items([SNAP_PRIV] + pick(ops, [ops_wb_name]))
ops_wb = "use super::super::*;\nuse super::*;\n" + keep_use_items(ops, _ops_wb_body, only={"BTreeSet"}) + "\n" + _ops_wb_body
shp_wb_body = join_items(pick(shp, [shp_wb_name]))
x_test_file = "use super::*;\n" + keep_use_items(shp, shp_wb_body, only={"ArenaId", "Curve3Id", "SolidId"}) + "\n" + shp_wb_body
ana_wb_body = join_items(pick(ana, [ana_wb_name]))
ana_wb_file = "use super::*;\n" + keep_use_items(ana, ana_wb_body, only={"Watertightness"}) + "\n" + ana_wb_body

# M preview tests, K tests
m_prev_body = join_items(pick(unit, M_TESTS))
for fx in ("🎯️component-picking", "🎯️vertex-provenance", "🔗️mesh-transfer-merge"):
    m_prev_body = m_prev_body.replace(f'include_str!("../🧫️fixtures/{fx}/🔣️.json")', f'include_str!("../../../🧪️tests/🧫️fixtures/{fx}/🔣️.json")')
assert "../🧫️fixtures" not in m_prev_body.replace("../../../🧪️tests/🧫️fixtures", "")
m_prev_file = "use super::*;\n" + "\n" + m_prev_body
k_tests_file = "use super::*;\n\n" + join_items(pick(unit, K_TESTS))

T_ROOT_DOC = (
    "//! 🔭 Black-box laws of the arena-backed B-Rep engine through its public surface: the sync API, the operation jobs, the shape value and the\n"
    "//! session analysis. The white-box laws that need the engine's private session state stay in `semio-framework-3d-brep-engine`.\n\n"
)
t_root_file = (
    T_ROOT_DOC
    + '#[cfg(test)]\n#[path = "🔭️api/🦀️.rs"]\nmod api;\n\n'
    + '#[cfg(test)]\n#[path = "⏱️operation-jobs/🦀️.rs"]\nmod operation_jobs;\n\n'
    + '#[cfg(test)]\n#[path = "🪅️shape-values/🦀️.rs"]\nmod shape_values;\n\n'
    + '#[cfg(test)]\n#[path = "🔎️analysis/🦀️.rs"]\nmod analysis;\n'
)

if "--show" in sys.argv:
    for n, t in [("t_api", t_api_file), ("t_ops", t_ops_file), ("t_shp", t_shp_file), ("t_ana", t_ana_file), ("x_test", x_test_file), ("ana_wb", ana_wb_file), ("m_prev", m_prev_file), ("k_tests", k_tests_file), ("wb_unit", wb_unit), ("ops_wb", ops_wb)]:
        print(n, len(t.encode()), "\n" + "\n".join(t.split("\n")[:14]), "\n---")


# ---------------------------------------------------------------------------------------------------------------- stage 3: write
def remove_live(rel):
    p = f"{OUT}/{rel}"
    assert os.path.realpath(p).startswith(os.path.realpath(OUT))
    if not DRY:
        guard(rel)
    if os.path.exists(p):
        os.remove(p)
        print("removed", rel)


if "--write" in sys.argv:
    wr("🧬️kernel-contract/🦀️.rs", k_text)
    wr("🧬️kernel-contract/🧪️tests/🔬️unit/🦀️.rs", k_tests_file)
    wr("📦️mesh-io/🦀️.rs", mesh)
    wr("📦️mesh-io/🧪️tests/🖼️preview/🦀️.rs", m_prev_file)
    wr("🪅️shape-value/🦀️.rs", s_text)
    remove_live("🪅️shape-value/🧪️tests/🔬️unit/🦀️.rs")
    wr("🚚️shape-exchange/🦀️.rs", x_text)
    wr("🚚️shape-exchange/🧪️tests/🔬️unit/🦀️.rs", x_test_file)
    wr("🦀️.rs", new_root)
    wr("🧪️tests/🔬️unit/🦀️.rs", wb_unit)
    wr("🧪️tests/🦀️.rs", t_root_file)
    wr("🧪️tests/🔭️api/🦀️.rs", t_api_file)
    wr("🧪️tests/⏱️operation-jobs/🦀️.rs", t_ops_file)
    wr("⏱️operation-jobs/🧪️tests/🔬️unit/🦀️.rs", ops_wb)
    wr("🧪️tests/🪅️shape-values/🦀️.rs", t_shp_file)
    wr("🧪️tests/🔎️analysis/🦀️.rs", t_ana_file)
    wr("🔎️analysis-session/🧪️tests/🔬️unit/🦀️.rs", ana_wb_file)
