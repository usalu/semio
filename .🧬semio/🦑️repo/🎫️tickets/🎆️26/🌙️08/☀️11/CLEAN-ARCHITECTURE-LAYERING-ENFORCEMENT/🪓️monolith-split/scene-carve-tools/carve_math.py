"""🧮 Carves the math module of ui-scene into the algebra, camera, mesh, pass and picking crates (staged under the scratchpad)."""
import re, sys
from lib import *
from model import *
from synth import *
from rsitems import split_items, regions, region_text

MATH_HEADER = """pub use semio_framework_geometry::{Mat4, Vec3};
use serde::{Deserialize, Serialize};
pub use semio_framework_mesh_engine::ComponentReferenceTable;
use protocol::value::{RetirementDemand,retained_clone::{RetainedCloneGrant,RetainedCloneProgress},retirement::controlled::ControlledRetirement};
use semio_framework_ui_viewport::{
    Viewport3dAxonometricHemisphere, Viewport3dAxonometricQuadrant,
    Viewport3dAxonometricVariant, Viewport3dObliqueVariant,
    Viewport3dOrthographicView, Viewport3dProjectionMode,
    Viewport3dProjectionOrientation, Viewport3dProjectionSpec,
};
use std::collections::{BTreeMap, BTreeSet};
"""

ALGEBRA_PRIVATE = ["vec3_new_m", "vec3_from_array_m", "mat4_identity_m", "mat4_perspective_m", "mat4_orthographic_m", "mat4_look_at_m", "mat4_translation_m", "mat4_scale_vec_m", "mat4_from_quat_m"]

TESTKIT_MESH = ["ownership_test_grant", "admit_mesh", "allocate_mesh", "close_mesh", "mesh_test_attempt_controlled", "mesh_test_attempt", "test_box_mesh", "LegacyMeshOracleData", "paged_mesh_fixture", "ObservedAllocator", "retirement_allocation_event", "observe_retirement_allocations"]
TESTKIT_CAMERA = ["concrete_forest_camera", "puzzle3d_top_camera", "puzzle3d_perspective_camera"]

ALLOC_STATIC = "#[global_allocator]\nstatic OBSERVED_ALLOCATOR:ObservedAllocator=ObservedAllocator;"

DOCS = {
    "algebra": "🧮️ Synchronous `Vec3`/`Mat4` algebra shared by every scene math crate — the plain `fn` mirror of `semio_framework_geometry`'s async inherent methods (ruling E6), verified against that crate's own algebra tests.",
    "camera": "🎥️ Orbit and parallel cameras, projection-spec matrices, framing, gumball and ray math, and the level-of-detail grid — the renderer-neutral camera half of the scene math.",
    "mesh": "🕸️ Paged mesh ownership: the exclusive authority, write tokens, leases, native resource custody and the instance and component source records drawn from them.",
    "pass": "🎞️ Scene draw descriptors: materials, lighting, shadow pass, procedural grid, line and textured draws, and frustum culling.",
    "picking": "🎯️ Screen picking: ray and marquee selection of instances, components and vertices over leased meshes.",
}


def split_pieces(text):
    items = split_items(text)
    pieces = []
    for it in items:
        if it.kind == "marker":
            continue
        pieces.append(it)
    return pieces


def gap_before(text, it, items):
    lines = text.split("\n")
    prev = [x for x in items if x.end < it.start]
    if not prev:
        return "\n\n"
    p = prev[-1]
    return "\n" if it.start == p.end + 1 else "\n\n"


def main():
    fresh_new()
    M = base("📐️math/🦀️.rs")
    R = regions(M)
    cat = parse_catalog(MATH_HEADER)
    reg = lambda n: region_text(M, n)
    body = {}
    alg = reg("🔖️SyncAlgebra")
    for n in ALGEBRA_PRIVATE:
        assert len(re.findall(r"^fn " + n + r"\b", alg, re.M)) == 1, n
        alg = re.sub(r"^fn " + n + r"\b", "pub fn " + n, alg, flags=re.M)
    body["algebra"] = alg
    body["camera"] = "\n\n".join(reg(n) for n in ["Camera", "GumballMath", "LodGrid"])
    mesh = reg("Mesh")
    old = '#[path = "📦️resource/🦀️.rs"]\nmod mesh3d_resource;'
    assert mesh.count(old) == 1
    mesh = mesh.replace(old, '#[path = "../📦️resource/🦀️.rs"]\nmod mesh3d_resource;')
    body["mesh"] = mesh
    body["pass"] = "\n\n".join(reg(n) for n in ["ScenePass", "ProceduralGrid", "LineDraw", "Culling"])
    body["picking"] = reg("Picking")
    children = {"mesh": base("📐️math/♻️retirement/🦀️.rs") + "\n" + base("📐️math/📦️resource/🦀️.rs") + "\n" + base("📐️math/📦️resource/📥️receiving/🦀️.rs")}
    reg_ = Registry(cat)
    UP = {"algebra": [], "camera": ["algebra"], "mesh": ["algebra"], "pass": ["algebra", "mesh"], "picking": ["algebra", "camera", "mesh", "pass"]}
    DEV = {"algebra": [], "camera": [], "mesh": [], "pass": ["camera"], "picking": []}
    for k in ["algebra", "camera", "mesh", "pass", "picking"]:
        reg_.add_crate(k, body[k], UP[k], dev=DEV[k])
    reg_.pub["mesh"].update({"ComponentReferenceTable": "use"})
    for n in ["Mesh3dResourceIdentity", "Mesh3dResourceLoan", "Mesh3dResourceOwner", "Mesh3dResourceReceiving", "Mesh3dReceivingStep", "MESH3D_RESOURCE_COMPONENT_COUNT_CAPACITY"]:
        reg_.pub["mesh"][n] = "use"
    out = {}
    for k in ["algebra", "camera", "mesh", "pass", "picking"]:
        text = body[k] + "\n" + children.get(k, "")
        pubn = set()
        provided = set()
        if k == "algebra":
            pubn = {"Mat4", "Vec3"}
        if k == "mesh":
            pubn = {"ComponentReferenceTable"}
        lines, ext, sib = reg_.import_lines(k, text, pub_names=pubn)
        if k == "mesh":
            lines = "#[path = \"../♻️retirement/🦀️.rs\"]\nmod mesh3d_retirement;\nuse mesh3d_retirement::{IneligibleGroups, NumericSeen};\n" + lines
        out[k] = (lines, ext, sib)
    return body, out, reg_, children



TEST_DOC = {
    "algebra": ("➕️", "Unit proofs of the synchronous `Vec3`/`Mat4` algebra."),
    "camera": ("🔭️", "Unit proofs of the orbit and parallel cameras, projection matrices, gumball and level-of-detail grid math."),
    "mesh": ("🧪️", "Unit proofs of the paged mesh authority, native resource custody and exact physical retirement receipts."),
    "pass": ("🪟️", "Unit proofs of frustum culling over the scene draw descriptors."),
    "picking": ("🧲️", "Unit proofs of ray, marquee and component screen picking."),
}
KIT_DOC = {
    "mesh": ("🧰️", "Shared test kit of the mesh crate: the allocation observer, bounded retry helpers and paged mesh fixtures every mesh-dependent suite reuses."),
    "camera": ("🎦️", "Shared test kit of the camera crate: the verbatim wire cameras reused by the culling and picking suites."),
}
ROOT_EMOJI = {"algebra": "🧮️", "camera": "🎥️", "mesh": "🕸️", "pass": "🎞️", "picking": "🎯️"}


def build():
    body, out, reg_, children = main()
    T = base("🧪️tests/🔬️math-unit/🦀️.rs")
    titems = [it for it in split_items(T) if it.kind != "marker"]
    local = {it.name: it for it in titems if it.kind in ("fn", "struct", "const", "static")}
    defs = {}
    for k in ["algebra", "camera", "mesh", "pass", "picking"]:
        for n in list(reg_.defined[k]) + list(reg_.pub[k]):
            defs.setdefault(n, k)
    for n in ["NumericSeen", "IneligibleGroups"]:
        defs[n] = "mesh"
    for it in split_items(children["mesh"]):
        if it.kind in ("fn", "struct", "enum", "const", "type", "static"):
            defs.setdefault(it.name, "mesh")
    defs.pop("vertex", None)
    for n in VEC3_METHODS + MAT4_METHODS:
        defs[n] = "algebra"

    def crates_of(it, seen=None):
        seen = seen if seen is not None else set()
        key = it.name + str(it.start)
        if key in seen:
            return set()
        seen.add(key)
        cs = set()
        ids = code_idents(it.text)
        for i in ids:
            if i in defs and i != it.name:
                cs.add(defs[i])
        for i in ids:
            if i in local and local[i] is not it:
                cs |= crates_of(local[i], seen)
        return cs

    assign = {}
    kit_items = {"mesh": [], "camera": []}
    for it in titems:
        if it.kind == "use":
            continue
        name = it.name
        if name in TESTKIT_MESH or (it.kind == "impl" and ("LegacyMeshOracleData" in name or "ObservedAllocator" in name)) or (it.kind == "other" and "RETIREMENT_ALLOCATION_EVENTS" in name):
            kit_items["mesh"].append(it)
            continue
        if name in TESTKIT_CAMERA:
            kit_items["camera"].append(it)
            continue
        if name == "OBSERVED_ALLOCATOR":
            assign[id(it)] = "mesh"
            continue
        cs = crates_of(it)
        if not cs:
            assign[id(it)] = "mesh"
        elif "picking" in cs:
            assign[id(it)] = "picking"
        elif "pass" in cs:
            assign[id(it)] = "pass"
        elif "mesh" in cs:
            assign[id(it)] = "mesh"
        elif "camera" in cs:
            assign[id(it)] = "camera"
        else:
            assign[id(it)] = "algebra"
    return body, out, reg_, children, titems, assign, kit_items, T



def header_names(header_text):
    names = set()
    for m in re.finditer(r"^\s*(?:pub(?:\([^)]*\))?\s+)?use\s+([^;]+);", header_text, re.M):
        for n, full in use_names("use " + m.group(1)):
            names.add(n)
    for m in re.finditer(r"^\s*(?:pub\s+)?mod\s+(\w+)\s*;", header_text, re.M):
        names.add(m.group(1))
    return names


def join_items(items, T_items_all, text):
    out = ""
    for n, it in enumerate(items):
        if n:
            prev_all = [x for x in T_items_all if x.end < it.start]
            sep = "\n\n"
            if prev_all and it.start == prev_all[-1].end + 1:
                sep = "\n"
            out += sep
        out += it.text
    return out


def pubify(text):
    text = re.sub(r"^(\s*)(pub\(crate\)\s+)?fn ", lambda m: m.group(1) + "pub fn ", text, count=1, flags=re.M)
    return text


def emit():
    body, out, reg_, children, titems, assign, kit_items, T = build()
    kit_names = {"mesh": set(), "camera": set()}
    kit_files = {}
    for k, its in kit_items.items():
        parts = []
        for it in its:
            t = it.text
            if it.kind == "fn":
                t = re.sub(r"^((?:\s*///[^\n]*\n|\s*#\[[^\n]*\]\n)*)\s*(?:pub\(crate\)\s+)?fn ", lambda m: m.group(1) + "pub fn ", t, count=1)
                if it.name != "retirement_allocation_event":
                    kit_names[k].add(it.name)
            elif it.kind == "struct":
                t = t.replace("struct " + it.name, "pub struct " + it.name, 1)
                if it.name == "LegacyMeshOracleData":
                    t = re.sub(r"^    (\w+: )", r"    pub \1", t, flags=re.M)
                kit_names[k].add(it.name)
            elif it.kind == "impl" and "LegacyMeshOracleData" in it.name:
                t = t.replace("    fn triangle", "    pub fn triangle")
            parts.append((it, t))
        kit_files[k] = parts
    roots = {}
    tests_files = {}
    kit_out = {}
    provided = {}
    for k in ["algebra", "camera", "mesh", "pass", "picking"]:
        lines, ext, sib = out[k]
        has_tests = any(v == k for v in assign.values())
        has_kit = k in kit_files
        d = doc(ROOT_EMOJI[k], DOCS[k], ["", "Carved out of the former `semio-framework-ui-scene` math module by ticket `26/08/11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT`."])
        header = fmt_header(d + "\n" + lines)
        provided[k] = header_names(header) | reg_.defined[k]
        tail = ""
        if has_kit:
            tail += '#[cfg(any(test, feature = "testkit"))]\n#[path = "../../🧪️tests/🧰️' + k + '-testkit/🦀️.rs"]\npub mod testkit;\n\n'
        if has_tests:
            tail += '#[cfg(test)]\n#[path = "../../🧪️tests/🔬️' + k + '-unit/🦀️.rs"]\nmod tests;\n'
        roots[k] = header + "\n" + body[k].rstrip("\n") + "\n\n" + tail.rstrip("\n") + "\n"
    for k in kit_files:
        text = "\n\n".join(t for it, t in kit_files[k])
        ext, sib = reg_.needs(k, text, provided=provided[k], test=True)
        pubn = {"ObservedAllocator"} if False else set()
        lines = []
        if ext:
            lines.append(render_use(list(ext.values())))
        for d_, names in sib.items():
            lines.append(render_use([ident(d_) + "::" + n for n in names]))
        e, t = KIT_DOC[k]
        header = fmt_header(doc(e, t) + "\nuse super::*;\n" + "\n".join(lines))
        kit_out[k] = header + "\n" + text + "\n"
    for k in ["algebra", "camera", "mesh", "pass", "picking"]:
        its = [it for it in titems if assign.get(id(it)) == k]
        if not its:
            continue
        text = join_items(its, titems, T)
        ext, sib = reg_.needs(k, text, provided=provided[k] | {"tests"}, test=True)
        lines = []
        if ext:
            lines.append(render_use(list(ext.values())))
        for d_, names in sib.items():
            lines.append(render_use([ident(d_) + "::" + n for n in names]))
        ids = code_idents(text)
        for kk in kit_names:
            used = sorted(n for n in kit_names[kk] if n in ids)
            if not used:
                continue
            if kk == k:
                lines.append("use super::testkit::{" + ", ".join(used) + "};")
            else:
                lines.append("use " + ident(kk) + "::testkit::{" + ", ".join(used) + "};")
        e, t = TEST_DOC[k]
        header = fmt_header(doc(e, t) + "\nuse super::*;\n" + "\n".join(lines))
        tests_files[k] = header + "\n" + text + "\n"
    return roots, tests_files, kit_out, reg_, out, assign, titems, kit_names, body


def write_math():
    roots, tests_files, kit_out, reg_, out, assign, titems, kit_names, body = emit()
    for k, t in roots.items():
        wr(NEW, CRATES[k]["node"] + "/🦀️.rs", t)
    for k, t in tests_files.items():
        wr(NEW, "🧪️tests/🔬️" + k + "-unit/🦀️.rs", t)
    for k, t in kit_out.items():
        wr(NEW, "🧪️tests/🧰️" + k + "-testkit/🦀️.rs", t)
    import os
    os.remove(NEW + "/🧪️tests/🔬️math-unit/🦀️.rs")
    os.remove(NEW + "/📐️math/🦀️.rs")
    return roots, tests_files, kit_out


if __name__ == "__main__":
    roots, tests_files, kit_out = write_math()
    for k, t in roots.items(): print("====", k, len(t.split("\n")))
    for k, t in kit_out.items(): print("---- kit", k); print(t[:1500])
    for k, t in tests_files.items(): print("---- tests", k); print(t[:900])
