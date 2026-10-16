"""🏗️ Orchestrates the whole ui-scene carve into the staged tree: math crates, scene crates, re-homed pack/modelling/surface edges, tests and test kits."""
import os, re, sys
from lib import *
from model import *
from synth import *
from rsitems import split_items, regions, region_text
import carve_math, carve_scenes
from carve_scenes import unqual
from carve_math import header_names, join_items

PRODUCT = ["world3d", "node-graph", "plane", "views"]

TEST_EMOJI = {"doc": "🔬️", "world3d": "🌐️", "node-graph": "🔀️", "plane": "🪧️", "views": "📑️", "surface": "🪪️", "pack": "🧳️", "modelling": "🏷️"}
RT_EMOJI = {"world3d": "🌀️", "node-graph": "🔁️", "plane": "🧿️", "views": "🧾️", "plane-x": "🪆️"}


def snapshot_names(rel):
    return sorted(api_names(base(rel)))


def pub_use(mod, names):
    return "pub use " + mod + "::{" + ", ".join(names) + "};"


def assign_tests(titems, defs, local, rules=None):
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
    out = {}
    for it in titems:
        if it.kind in ("use", "marker"):
            continue
        out[id(it)] = crates_of(it)
    return out


def emit_items(items, assigned, key, all_items):
    """Items of `items` owned by `key`, with the region markers of every region that holds at least one of them."""
    stack = []
    keep = []
    regs = {}
    for idx, it in enumerate(items):
        if it.kind == "marker":
            if it.name.startswith("//#region"):
                stack.append(idx)
            else:
                s = stack.pop()
                regs[s] = idx
            continue
    chosen = set()
    for idx, it in enumerate(items):
        if it.kind != "marker" and assigned.get(id(it)) == key:
            chosen.add(idx)
    for s, e in regs.items():
        if any(s < i < e for i in chosen):
            chosen.add(s)
            chosen.add(e)
    ordered = [items[i] for i in sorted(chosen)]
    out = ""
    prev = None
    for it in ordered:
        if prev is not None:
            out += "\n" if it.start == prev.end + 1 else "\n\n"
        out += it.text
        prev = it
    return out, bool(chosen)


def build_all():
    carve_math.fresh_new()
    roots_m, tests_m, kits_m, reg_, out_m, assign_m, titems_m, kit_names, body_m = carve_math.emit()
    files = {}
    for k, t in roots_m.items():
        files[CRATES[k]["node"] + "/🦀️.rs"] = t
    for k, t in tests_m.items():
        files["🧪️tests/🔬️" + k + "-unit/🦀️.rs"] = t
    for k, t in kits_m.items():
        files["🧪️tests/🧰️" + k + "-testkit/🦀️.rs"] = t
    bodies, views_body, bl, mod_new, S = carve_scenes.scenes_bodies()
    pack_src = base("📦️pack/🦀️.rs")
    reg_.add_crate("pack", pack_src, [])
    reg_.add_crate("modelling", mod_new, [])
    reg_.add_crate("doc", bodies["doc"], ["pack"])
    up_prod = {"world3d": ["doc", "pack", "modelling", "pass"], "node-graph": ["doc", "pack"], "plane": ["doc", "pack"], "views": ["doc", "pack"]}
    dev_prod = ["camera", "algebra", "mesh", "pass", "modelling", "picking"]
    plane_extra = base("📷️framing/🦀️.rs") + base("🖼️canvas2d-snapshot/🦀️.rs")
    snap_names = {
        "world3d": snapshot_names("🌍️world3d-snapshot/🦀️.rs"),
        "plane_snap": snapshot_names("🖼️canvas2d-snapshot/🦀️.rs"),
        "block_list": snapshot_names("🧬️schema/🧩️block-list/🦀️.rs"),
    }
    reg_.defined.setdefault("world3d", set())
    bodies["views"] = unqual_views = views_body
    bl2 = bl.replace("pub use block_list::*;", pub_use("block_list", snap_names["block_list"]))
    bl2 = bl2.replace('#[path = "../🧬️schema/🧩️block-list/🦀️.rs"]', '#[path = "../../🧬️schema/🧩️block-list/🦀️.rs"]')
    assert "../../🧬️schema" in bl2
    bodies["views"] = views_body + "\n\n" + bl2
    for k in PRODUCT:
        extra = ""
        if k == "world3d":
            extra = base("🌍️world3d-snapshot/🦀️.rs")
        if k == "plane":
            extra = plane_extra
        if k == "views":
            extra = base("✂️text-splice/🦀️.rs") + base("🧬️schema/🧩️block-list/🦀️.rs")
        reg_.add_crate(k, bodies[k] + "\n" + extra, up_prod[k], dev=dev_prod + (["modelling"] if k != "world3d" else []))
    reg_.add_crate("surface", base("🌉️surface/🦀️.rs"), ["doc", "pack"], dev=["world3d", "plane", "views", "node-graph"])
    reg_.catalog.update(parse_catalog(carve_scenes.SCENES_HEADER))
    root_text = {}
    provided = {}
    mounts_hdr = {
        "world3d": ['#[path = "../../🌍️world3d-snapshot/🦀️.rs"]\nmod world3d_snapshot;\n' + pub_use("world3d_snapshot", snap_names["world3d"])],
        "plane": ['#[path = "../../📷️framing/🦀️.rs"]\nmod framing;\npub use framing::{Canvas2dFrameCamera, Canvas2dFraming};', '#[path = "../../🖼️canvas2d-snapshot/🦀️.rs"]\nmod canvas2d_snapshot;\n' + pub_use("canvas2d_snapshot", snap_names["plane_snap"])],
        "views": ['#[path = "../../✂️text-splice/🦀️.rs"]\nmod text_splice;\n' + [l for l in base("📦️packages/🦀️rust/🦀️.rs").split("\n") if l.startswith("pub use text_splice::")][0]],
    }
    plane_tail = """
impl ToValue for Canvas2dFraming {
    fn to_value(&self) -> DslValue {
        DslValue::Object(vec![("revision".into(),self.revision.to_value()),("bounds".into(),self.bounds.to_vec().to_value()),("padding".into(),self.padding.to_value())])
    }
}

impl FromValue for Canvas2dFraming {
    fn from_value(value: DslValue) -> Result<Self,ValueError> {
        let entries = value.into_object()?;
        let bounds: Vec<f64> = value_decode(&entries,"bounds")?;
        Ok(Self { revision: value_decode(&entries,"revision")?,bounds: bounds.try_into().map_err(|_| ValueError::new(protocol::value::ValueRefusalKind::InvalidValue, "canvas framing requires four bounds"))?,padding: value_decode(&entries,"padding")? })
    }
}
"""
    S_tail = S[S.index("impl ToValue for crate::Canvas2dFraming"):]
    assert S_tail.replace("crate::", "").strip() == plane_tail.strip(), "framing impl drifted"
    bodies["plane"] = bodies["plane"] + "\n\n" + plane_tail.strip("\n")
    reg_.pub["doc"].update({"scene_pack_wire": "macro"})
    for k in ["doc"] + PRODUCT:
        text = bodies[k]
        lines, ext, sib = reg_.import_lines(k, text + "\n" + (plane_extra if k == "plane" else ""), pub_names=set())
        if k == "doc":
            sib_text = ""
        extra_hdr = "\n".join(mounts_hdr.get(k, []))
        hist = []
        if k == "doc":
            hist = [""] + [l[4:] if l.startswith("//! ") else "" for l in S.split("\n")[:22] if l.startswith("//!")]
            hist[1] = re.sub(r"^🎬️ ", "", hist[1])
        d = doc(carve_scenes.ROOT_EMOJI2[k], carve_scenes.DOCS[k], ["", "Carved out of the former `semio-framework-ui-scene` scenes module by ticket `26/08/11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT`."] + hist)
        hdr = fmt_header(d + "\n" + lines + "\n" + extra_hdr)
        provided[k] = header_names(hdr) | reg_.defined[k]
        tail = ""
        files_for_tests = k
        root_text[k] = [hdr, bodies[k], tail]
    return files, reg_, bodies, root_text, provided, snap_names, S, mod_new, titems_m



def owner_scenes_unit(it):
    n = it.name
    low = n.lower()
    if n == "text_editor_buffer_lanes_preserve_complete_unicode_documents":
        return "surface"
    if n in ("a_scene_without_lanes_still_publishes_its_whole_doc",):
        return "plane"
    if n.startswith(("WORLD3D", "world3d", "scene_lane_hash", "a_nakagin_scale_world3d")):
        return "world3d"
    if n.startswith(("CANVAS2D", "canvas2d", "BOARD2D", "board2d", "TILEDMAP", "tiledmap", "ink_canvas", "PAINT2D", "paint2d")):
        return "plane"
    if n.startswith(("node_graph", "NODE_GRAPH")):
        return "node-graph"
    if n.startswith(("table_lanes",)):
        return "views"
    raise SystemExit("unassigned scenes-unit item " + n)


def split_mixed(it):
    n = it.name
    if n == "table_scene_and_tiled_map_scene_round_trip":
        t = it.text
        a = t.index("    let tiled = ")
        table = t[:a].rstrip() + "\n}"
        table = table.replace("fn table_scene_and_tiled_map_scene_round_trip", "fn table_scene_round_trips")
        tiled = "#[test]\nfn tiled_map_scene_round_trips() {\n" + t[a:]
        return [("views", table), ("plane", tiled)]
    if n == "board2d_scene_and_block_list_scene_round_trip":
        t = it.text
        a = t.index("    let blocks = ")
        board = t[:a].rstrip() + "\n}"
        board = board.replace("fn board2d_scene_and_block_list_scene_round_trip", "fn board2d_scene_round_trips")
        blocks = "#[test]\nfn block_list_scene_round_trips() {\n" + t[a:]
        return [("plane", board), ("views", blocks)]
    return None


def owner_round_trip(it):
    n = it.name
    if n.startswith("canvas2d"):
        return "plane"
    if n.startswith("world3d"):
        return "world3d"
    if n.startswith("node_graph"):
        return "node-graph"
    if n in ("missing_required_field_reports_the_field_name", "typed_block_list_shared_fixture_round_trips_without_nested_text"):
        return "views"
    raise SystemExit("unassigned round-trip item " + n)


MODELLING_SCENE_TESTS = {"modelling_scene", "the_three_modelling_lanes_split_out_of_the_spine_and_merge_back_as_typed_values", "a_typed_lane_refuses_an_invalid_payload_and_leaves_the_scene_unchanged", "only_a_changed_modelling_lane_changes_the_spine_manifest", "the_scene_value_codec_and_serde_shape_carry_the_typed_fields", "the_typed_lanes_never_ride_the_pack_and_the_split_spine_does", "the_builder_validates_what_it_sets"}


def group_items(items, owner_fn, extra=None):
    """Returns {crate: text} keeping region markers where a region holds items of that crate."""
    assigned = {}
    pieces = {}
    for it in items:
        if it.kind in ("use", "marker"):
            continue
        sp = split_mixed(it) if extra is None else None
        if sp:
            pieces[id(it)] = sp
            continue
        assigned[id(it)] = owner_fn(it)
    stack, regs = [], {}
    for idx, it in enumerate(items):
        if it.kind == "marker":
            if it.name.startswith("//#region"):
                stack.append(idx)
            else:
                regs[stack.pop()] = idx
    out = {}
    crates = set(assigned.values()) | {c for v in pieces.values() for c, _ in v}
    for c in crates:
        chosen = []
        for idx, it in enumerate(items):
            if id(it) in pieces:
                for cc, t in pieces[id(it)]:
                    if cc == c:
                        chosen.append((idx, it, t))
            elif assigned.get(id(it)) == c:
                chosen.append((idx, it, it.text))
        idxs = {i for i, _, _ in chosen}
        for s_, e_ in regs.items():
            if any(s_ < i < e_ for i in idxs):
                chosen.append((s_, items[s_], items[s_].text))
                chosen.append((e_, items[e_], items[e_].text))
        chosen.sort(key=lambda x: (x[0], 0))
        text = ""
        prev = None
        for idx, it, t in chosen:
            if prev is not None:
                text += "\n" if (it.start == prev.end + 1) else "\n\n"
            text += t
            prev = it
        out[c] = text
    return out



UNIT_EMOJI = {"world3d": "🛰️", "node-graph": "🔀️", "plane": "🪧️", "views": "📑️"}
RT_EMOJI = {"world3d": "🌀️", "node-graph": "🔁️", "plane": "🧿️", "views": "🧾️"}
UNIT_TEXT = {
    "world3d": "Unit proofs of the world-3d scene: its lane contract, presentation defaults, lane split and merge, and pack and value codecs.",
    "node-graph": "Unit proofs of the node-graph scene: hover and interaction records, highlighted ids and the neutral lane contract.",
    "plane": "Unit proofs of the flat-surface scenes: canvas, board, tiled map, ink canvas and paint lane contracts and spine bounds.",
    "views": "Unit proofs of the document-style scenes: complete unicode table lanes.",
}
RT_TEXT = {
    "world3d": "Value-codec round trips of the world-3d scene, dense and bare.",
    "node-graph": "Value-codec round trips of the node-graph scene, its records and its viewport.",
    "plane": "Value-codec round trips of the canvas, board and tiled-map scenes.",
    "views": "Value-codec round trips of the table and block-list scenes and their refusals.",
}


def fix_test_text(text, k=None):
    if k == "plane":
        assert text.count("World3dSceneLane::ToolRunTrace.body_key()") == 2
        text = text.replace("World3dSceneLane::ToolRunTrace.body_key()", '"framework.scene.world3d.toolRunTrace"')
    text = text.replace("crate::encode(", "semio_framework_ui_scene_surface::encode(").replace("crate::decode(", "semio_framework_ui_scene_surface::decode(")
    return unqual(text)


def test_header(reg_, k, text, provided_k, emoji, doc_text, extra_lines=()):
    ext, sib = reg_.needs(k, text, provided=provided_k | {"tests"}, test=True)
    lines = []
    if ext:
        lines.append(render_use(list(ext.values())))
    for d_, names in sib.items():
        lines.append(render_use([ident(d_) + "::" + n for n in names]))
    lines += list(extra_lines)
    return fmt_header(doc(emoji, doc_text) + "\nuse super::*;\n" + "\n".join(lines))


def mounts_tail(k, names):
    out = ""
    for mod, f in names:
        out += '#[cfg(test)]\n#[path = "../../🧪️tests/' + f + '/🦀️.rs"]\nmod ' + mod + ';\n\n'
    return out.rstrip("\n") + "\n"


def finish():
    files, reg_, bodies, root_text, provided, snap_names, S, mod_new, titems_m = build_all()
    reg_.catalog["Value"] = "serde_json::Value"
    deletes = ["🎬️scenes/🦀️.rs", "📐️math/🦀️.rs", "🧪️tests/🔬️math-unit/🦀️.rs", "🧪️tests/🔬️scenes-unit/🦀️.rs", "🧪️tests/🔬️scenes-value-round-trip/🦀️.rs"]
    gu = group_items(split_items(base("🧪️tests/🔬️scenes-unit/🦀️.rs")), owner_scenes_unit)
    gr = group_items(split_items(base("🧪️tests/🔬️scenes-value-round-trip/🦀️.rs")), owner_round_trip)
    for k in PRODUCT:
        hdr, body, _ = root_text[k]
        mounts = []
        if k in gu:
            mounts.append(("tests", "🔬️" + k + "-unit"))
        if k in gr:
            mounts.append(("value_round_trip_tests", "🔬️" + k + "-value-round-trip"))
        if k == "world3d":
            mounts.append(("modelling_scene_tests", "🔬️world3d-modelling-scene-unit"))
        tail = mounts_tail(k, mounts)
        files[CRATES[k]["node"] + "/🦀️.rs"] = hdr + "\n" + body.rstrip("\n") + "\n\n" + tail
        if k in gu:
            t = fix_test_text(gu[k], k)
            files["🧪️tests/🔬️" + k + "-unit/🦀️.rs"] = test_header(reg_, k, t, provided[k], UNIT_EMOJI[k], UNIT_TEXT[k]) + "\n" + t + "\n"
        if k in gr:
            t = fix_test_text(gr[k])
            files["🧪️tests/🔬️" + k + "-value-round-trip/🦀️.rs"] = test_header(reg_, k, t, provided[k], RT_EMOJI[k], RT_TEXT[k]) + "\n" + t + "\n"
    hdr, body, _ = root_text["doc"]
    files[CRATES["doc"]["node"] + "/🦀️.rs"] = hdr + "\n" + body.rstrip("\n") + "\n\n" + mounts_tail("doc", [("tests", "🔬️doc-unit")])
    return files, reg_, bodies, root_text, provided, snap_names, S, mod_new, gu, gr, deletes



def one(text, old, new):
    assert text.count(old) == 1, (old, text.count(old))
    return text.replace(old, new)


def inplace(files, reg_, S, mod_new, provided, snap_names):
    # pack
    pk = base("📦️pack/🦀️.rs")
    for n in ["TAG_NONE", "TAG_SOME", "TAG_SEQ", "TAG_MAP"]:
        pk = one(pk, "const " + n + ": u8", "pub const " + n + ": u8")
    pk = one(pk, "fn write_varint(", "pub fn write_varint(")
    files["📦️pack/🦀️.rs"] = pk
    pu = base("🧪️tests/🔬️pack-unit/🦀️.rs")
    items = split_items(pu)
    drop = [it for it in items if it.name == "typed_scene_neutral_catalog_matches_native_serde_contracts"]
    assert len(drop) == 1
    moved_catalog = drop[0].text
    lines = pu.split("\n")
    start, end = drop[0].start, drop[0].end
    rest = lines[:start] + lines[end + 1:]
    pu2 = "\n".join(rest)
    pu2 = one(pu2, "crate::math::Camera3d::default()", "semio_framework_ui_scene_camera::Camera3d::default()")
    old = "use crate::math::{frame_projection_orbit_to_spec_bounds, projection_spec_matrix, projection_spec_orientation_look, projection_spec_project_ndc, projection_spec_unproject_ndc, Camera3d, CameraProjection3d, Mat4Math, OrbitController, Vec3Math, WORLD_PROJECTION_FRAME_PADDING};"
    names = re.search(r"use crate::math::\{([^}]*)\};", pu2).group(1)
    assert old in pu2
    new = "use semio_framework_ui_scene_algebra::{Mat4Math, Vec3Math};\n    use semio_framework_ui_scene_camera::{frame_projection_orbit_to_spec_bounds, projection_spec_matrix, projection_spec_orientation_look, projection_spec_project_ndc, projection_spec_unproject_ndc, Camera3d, CameraProjection3d, OrbitController, WORLD_PROJECTION_FRAME_PADDING};"
    for n in ["Mat4Math", "Vec3Math"]:
        assert n in reg_.pub["algebra"]
    for n in ["frame_projection_orbit_to_spec_bounds", "projection_spec_matrix", "projection_spec_orientation_look", "projection_spec_project_ndc", "projection_spec_unproject_ndc", "Camera3d", "CameraProjection3d", "OrbitController", "WORLD_PROJECTION_FRAME_PADDING"]:
        assert n in reg_.pub["camera"], n
    pu2 = pu2.replace(old, new)
    files["🧪️tests/🔬️pack-unit/🦀️.rs"] = pu2
    # modelling
    m = mod_new
    m = one(m, "use serde::{Deserialize, Serialize};\n\nuse crate::World3dScene;\n", "use serde::{Deserialize, Serialize};\n")
    files["📏️world3d-modelling/🦀️.rs"] = m
    mu = base("🧪️tests/🔬️world3d-modelling-unit/🦀️.rs")
    mitems = split_items(mu)
    keep, moved = [], []
    mlines = mu.split("\n")
    dropset = set()
    for it in mitems:
        if it.name in MODELLING_SCENE_TESTS:
            moved.append(it)
            for i2 in range(it.start, it.end + 1):
                dropset.add(i2)
            if it.end + 1 < len(mlines) and not mlines[it.end + 1].strip():
                dropset.add(it.end + 1)
    mu2 = "\n".join(l for i2, l in enumerate(mlines) if i2 not in dropset)
    mu2 = one(mu2, "use crate::{scene_lane_hash, SceneDoc, World3dSceneLane};\n", "")
    assert mu2.count("use crate::math::tests::observe_retirement_allocations as observe;") == 2
    mu2 = mu2.replace("use crate::math::tests::observe_retirement_allocations as observe;", "use semio_framework_ui_scene_mesh::testkit::observe_retirement_allocations as observe;")
    mu2 = one(mu2, "    use crate::math::{Camera3d, CameraProjection3d, Mat4Math, Vec3};", "    use semio_framework_geometry::Vec3;\n    use semio_framework_ui_scene_algebra::Mat4Math;\n    use semio_framework_ui_scene_camera::{Camera3d, CameraProjection3d};")
    mu2 = mu2.rstrip("\n") + "\n\n#[global_allocator]\nstatic OBSERVED_ALLOCATOR: semio_framework_ui_scene_mesh::testkit::ObservedAllocator = semio_framework_ui_scene_mesh::testkit::ObservedAllocator;\n"
    files["🧪️tests/🔬️world3d-modelling-unit/🦀️.rs"] = mu2
    helper_src = [it for it in mitems if it.name in ("FIXTURE", "fixture", "decode")]
    helpers = "\n\n".join(it.text for it in helper_src)
    mtext = "\n\n".join(it.text for it in moved)
    ms_text = helpers + "\n\n" + mtext
    ms_text = unqual(ms_text.replace("crate::pack::PackError", "semio_framework_ui_scene_pack::PackError"))
    ext, sib = reg_.needs("world3d", ms_text, provided=provided["world3d"] | {"tests"}, test=True)
    lines_ = []
    if ext:
        lines_.append(render_use(list(ext.values())))
    for d_, nm in sib.items():
        lines_.append(render_use([ident(d_) + "::" + n for n in nm]))
    files["🧪️tests/🔬️world3d-modelling-scene-unit/🦀️.rs"] = fmt_header(doc("🏷️", "Unit proofs of the modelling lanes carried by the world-3d scene: split, merge, refusal and builder validation.") + "\nuse super::*;\n" + "\n".join(lines_)) + "\n" + ms_text + "\n"
    # surface
    sf = base("🌉️surface/🦀️.rs")
    sf = one(sf, "use crate::pack::PackError;\nuse crate::SceneDoc;\nuse ui_contract::{SurfaceDoc, SurfaceKind, SurfaceProps};", "use semio_framework_ui_scene_doc::SceneDoc;\nuse semio_framework_ui_scene_pack::PackError;\nuse ui_contract::{SurfaceDoc, SurfaceKind, SurfaceProps};")
    files["🌉️surface/🦀️.rs"] = sf
    su = base("🧪️tests/🔬️surface-unit/🦀️.rs")
    su = one(su, "use crate::{Board2dScene, TableScene, World3dScene};", "use semio_framework_ui_scene_node_graph_placeholder;")
    su = su.replace("use semio_framework_ui_scene_node_graph_placeholder;", "use semio_framework_ui_scene_plane::Board2dScene;\nuse semio_framework_ui_scene_views::{TableScene, TextEditorScene, TEXT_EDITOR_BUFFER_LANE_KEY};\nuse semio_framework_ui_scene_world3d::World3dScene;\nuse semio_framework_ui_scene_doc::SceneDoc;\nuse serde_json::Value;")
    su = su.replace("crate::TextEditorScene", "TextEditorScene")
    te_test = [it for it in split_items(base("🧪️tests/🔬️scenes-unit/🦀️.rs")) if it.name == "text_editor_buffer_lanes_preserve_complete_unicode_documents"][0].text
    te_test = te_test.replace("crate::encode(", "encode(").replace("crate::decode(", "decode(")
    su = su.rstrip("\n") + "\n\n" + te_test + "\n"
    files["🧪️tests/🔬️surface-unit/🦀️.rs"] = su
    # block-list
    bl = base("🧬️schema/🧩️block-list/🦀️.rs")
    bl = one(bl, "use super::{value_push,value_push_option,value_decode,SceneDoc};", "use semio_framework_ui_scene_doc::{value_decode, value_push, value_push_option, SceneDoc};")
    files["🧬️schema/🧩️block-list/🦀️.rs"] = bl
    return moved_catalog



def original_api():
    S = base("🎬️scenes/🦀️.rs")
    flat = {}
    for f in ["🎬️scenes/🦀️.rs", "🖼️canvas2d-snapshot/🦀️.rs", "🌉️surface/🦀️.rs", "🌍️world3d-snapshot/🦀️.rs", "📏️world3d-modelling/🦀️.rs", "🧬️schema/🧩️block-list/🦀️.rs"]:
        for n, k in api_names(base(f)).items():
            if n == "block_list":
                continue
            flat[n] = f
    flat["Canvas2dFraming"] = "framing"
    flat["Canvas2dFrameCamera"] = "framing"
    root = base("📦️packages/🦀️rust/🦀️.rs")
    ts = [l for l in root.split("\n") if l.startswith("pub use text_splice::")][0]
    for n, _ in use_names(ts):
        flat[n] = "text-splice"
    math = {}
    M = base("📐️math/🦀️.rs")
    for n, k in api_names(M).items():
        if n in ("tests", "mesh3d_retirement", "mesh3d_resource"):
            continue
        math[n] = k
    return flat, math


def facade(reg_, flat, math):
    owner = {}
    for k in ["doc", "world3d", "node-graph", "plane", "views", "surface", "modelling"]:
        for n, kind in reg_.pub[k].items():
            if n in flat:
                assert n not in owner or owner[n] == k or True
                owner.setdefault(n, []).append(k) if isinstance(owner.get(n), list) else owner.__setitem__(n, [k])
    miss = sorted(n for n in flat if n not in owner)
    multi = {n: v for n, v in owner.items() if len(v) > 1}
    return owner, miss, multi



DOC_UNIT = """use super::*;
use std::hash::Hasher;

#[test]
fn scene_lane_hash_matches_the_published_fnv1a_64_vectors_and_the_fnv_crate() {
    for (payload, expected) in [("", "cbf29ce484222325"), ("a", "af63dc4c8601ec8c"), ("foobar", "85944171f73967e8")] {
        assert_eq!(scene_lane_hash(payload), expected, "published vector {payload:?}");
    }
    for payload in ["", "x", r#"{"lane":"annotations"}"#, "grüße — 世界 🌍", &"0123456789abcdef".repeat(4096)] {
        let mut oracle = fnv::FnvHasher::default();
        oracle.write(payload.as_bytes());
        assert_eq!(scene_lane_hash(payload), format!("{:016x}", oracle.finish()), "fnv crate oracle");
    }
}

#[test]
fn scene_lane_ref_round_trips_through_the_value_codec_and_serde() {
    let reference = SceneLaneRef { lane: "annotations".into(), bytes: 42, hash: scene_lane_hash("payload") };
    assert_eq!(SceneLaneRef::from_value(reference.to_value()), Ok(reference.clone()));
    let json = serde_json::to_value(&reference).unwrap();
    assert_eq!(json, serde_json::json!({"lane": "annotations", "bytes": 42, "hash": reference.hash}));
    assert_eq!(serde_json::from_value::<SceneLaneRef>(json).unwrap(), reference);
}

#[test]
fn value_primitives_follow_serde_optionality() {
    let entries = vec![("present".to_string(), DslValue::String("yes".into())), ("nothing".to_string(), DslValue::Null)];
    assert_eq!(value_decode::<String>(&entries, "present"), Ok("yes".to_string()));
    assert_eq!(value_decode_option::<String>(&entries, "absent"), Ok(None));
    assert_eq!(value_decode_option::<String>(&entries, "nothing"), Ok(None));
    assert_eq!(value_decode_default(&entries, "absent", || 7u32), Ok(7));
    assert_eq!(value_decode::<String>(&entries, "absent"), Err(ValueError::new(protocol::value::ValueRefusalKind::InvalidValue, "missing field `absent`")));
    let mut out = Vec::new();
    value_push(&mut out, "k", &"v".to_string());
    value_push_option(&mut out, "none", &Option::<String>::None);
    value_push_if_nonempty::<String>(&mut out, "empty", &[]);
    assert_eq!(out, vec![("k".to_string(), DslValue::String("v".into()))]);
}
"""


def facade_files(files, reg_, moved_catalog):
    flat, math = original_api()
    flat.pop("*", None)
    final_pub = {}
    for k in ["doc", "world3d", "node-graph", "plane", "views", "surface", "modelling"]:
        final_pub[k] = set(api_names(files[CRATES[k]["node"] + "/🦀️.rs"]))
    owner = {}
    for n in flat:
        cs = [k for k in final_pub if n in final_pub[k]]
        assert len(cs) == 1, (n, cs)
        owner[n] = cs[0]
    groups = {}
    for n, k in owner.items():
        groups.setdefault(k, []).append(n)
    lines = ["pub use semio_framework_ui_scene_pack as pack;"]
    for k in ["doc", "surface", "world3d", "modelling", "node-graph", "plane", "views"]:
        if k in groups:
            lines.append("pub use " + ident(k) + "::{" + ", ".join(sorted(groups[k])) + "};")
    d = "//! 🎬️ Product scene payloads for the semantic UI contract's `Component::Surface(SurfaceProps)` — the 15 `SceneDoc` structs each embedded product surface carries as an opaque pack-encoded `SurfaceDoc.bytes` blob, plus the generic 3D scene math shared by every mesh and world viewport.\n//!\n//! This crate is the thin facade over the scene producers `semio-framework-ui-scene-pack`, `-doc`, `-surface`, `-modelling`, `-world3d`, `-node-graph`, `-plane`, `-views` and the math producers `-algebra`, `-camera`, `-mesh`, `-pass` and `-picking`; it re-exports exactly the names its consumers use, bit for bit the surface of the former monolith. Wasm-safe by construction, product- and renderer-neutral, sync by decree (ruling E6).\n"
    fac = fmt_header(d + "\n#[path = \"../../📐️math/🦀️.rs\"]\npub mod math;\n" + "\n".join(lines) + "\n")
    fac = fac.rstrip("\n") + "\n\n#[cfg(test)]\n#[path = \"../../🧪️tests/🔬️scene-catalog-unit/🦀️.rs\"]\nmod tests;\n"
    files["📦️packages/🦀️rust/🦀️.rs"] = fac
    mo = {}
    for k in ["algebra", "camera", "mesh", "pass", "picking"]:
        fp = api_names(files[CRATES[k]["node"] + "/🦀️.rs"])
        for n in math:
            if n in fp:
                mo.setdefault(k, []).append(n)
    allm = sorted(n for v in mo.values() for n in v)
    assert allm == sorted(math), (set(math) - set(allm), set(allm) - set(math))
    ml = ["pub use " + ident(k) + "::{" + ", ".join(sorted(mo[k])) + "};" for k in ["algebra", "camera", "mesh", "pass", "picking"]]
    md = "//! 🌐️ Generic 3D scene math — camera, mesh instances, screen picking and draw descriptors — re-exported name for name from the math producers `-algebra`, `-camera`, `-mesh`, `-pass` and `-picking`.\n"
    mh = [l for l in base("📐️math/🦀️.rs").split("\n")[:13] if l.startswith("//!")]
    mh = [re.sub(r"^//! 🌐️ ", "//! ", l) for l in mh]
    md = md.rstrip("\n") + "\n//!\n" + "\n".join(mh) + "\n"
    files["📐️math/🦀️.rs"] = fmt_header(md + "\n" + "\n".join(ml) + "\n")
    cat = moved_catalog.replace("    use crate::*;\n", "    use semio_framework_ui_scene_pack::{to_bytes, write_varint, TAG_MAP, TAG_NONE, TAG_SEQ, TAG_SOME};\n")
    assert "use semio_framework_ui_scene_pack::{to_bytes" in cat
    cat = cat.replace("fn packet(", "fn packet(")
    files["🧪️tests/🔬️scene-catalog-unit/🦀️.rs"] = fmt_header(doc("🗃️", "Catalog proof: every typed scene of the facade decodes the language-neutral retained-scene catalog and refuses the hostile rows.") + "\nuse super::*;\n") + "\n" + cat + "\n"
    hdr = fmt_header(doc("🔬️", "Unit proofs of the scene doc contract: the lane hash against published FNV-1a vectors and the fnv crate, the lane reference codecs and the value primitives.") + "\nuse protocol::value::{DslValue, FromValue, ToValue, ValueError};\n")
    files["🧪️tests/🔬️doc-unit/🦀️.rs"] = hdr.replace("use protocol::value::{DslValue, FromValue, ToValue, ValueError};\n", "") + DOC_UNIT.replace("use super::*;\n", "use super::*;\nuse protocol::value::{DslValue, FromValue, ToValue, ValueError};\n", 1)
    return owner, mo


def stage():
    files, reg_, bodies, root_text, provided, snap_names, S, mod_new, gu, gr, deletes = finish()
    cat = inplace(files, reg_, S, mod_new, provided, snap_names)
    owner, mo = facade_files(files, reg_, cat)
    for rel in deletes:
        path = NEW + "/" + rel
        if os.path.exists(path):
            os.remove(path)
    for rel, text in files.items():
        wr(NEW, rel, text)
    return files, reg_, owner, mo


if __name__ == "__main__":
    files, reg_, owner, mo = stage()
    print(len(files), {k: len(v) for k, v in mo.items()})
    print(files["📦️packages/🦀️rust/🦀️.rs"])
    print(files["🧪️tests/🔬️doc-unit/🦀️.rs"][:600])
