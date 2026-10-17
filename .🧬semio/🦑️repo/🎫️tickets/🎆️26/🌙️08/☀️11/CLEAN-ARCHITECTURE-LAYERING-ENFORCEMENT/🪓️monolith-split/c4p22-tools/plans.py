"""🗺️ c4p22 split plans for Package 22: the seven stdio artifact Rust modules (obj diff, gltf oracle, gltf binary and text diff wires, gltf geometry kernel, gltf diff, ply diff).
`mode` "mod": children are `#[path]` modules (`use super::*;`, root `<vis> use <mod>::*;` when the child exports a name another file uses). `ranges` are 1-based inclusive line ranges of the pre-edit file.
Boundaries are computed from item names and region names of the pre-edit blob, never typed as raw line numbers: `S(key, name)` is the first line of the doc/attribute block of the item `name`, `T(key, text)` of the first line containing `text`, `R(key, region)` the span of a region.
`pre` / `post` lines wrap a child body (a region that spans two children is repeated, balanced, in each, as in `RESULT-REGION-RESTORE.md`); `macro_use` mounts the child with `#[macro_use]` (it defines a `macro_rules!` another child expands)."""
import re
from c4plib import *
import splitter as S
from rslex import outline


def _ls(key):
    return lines_of(read(f"{ORIG}/{key}.rs"))


def S(key, name, nth=1):
    ls = _ls(key)
    its = [it for it in outline("\n".join(ls) + "\n") if it["name"] == name and it["kind"] not in ("use", "other")]
    assert len(its) >= nth, (key, name)
    return _block_start(ls, its[nth - 1]["start"] + 1)


def T(key, text, nth=1):
    ls = _ls(key)
    hits = [i + 1 for i, l in enumerate(ls) if text in l]
    assert len(hits) >= nth, (key, text)
    return _block_start(ls, hits[nth - 1])


def _is_c(l):
    s = l.strip()
    return (s.startswith(("///", "//", "#[")) and not s.startswith(("//#region", "//#endregion", "// #region", "// #endregion")))


def _block_start(ls, line):
    i = line - 1
    while i - 1 >= 0:
        prev = ls[i - 1]
        if _is_c(prev):
            i -= 1
            continue
        if prev.strip() == "":
            j = i - 2
            while j >= 0 and ls[j].strip() == "":
                j -= 1
            if j >= 0 and _is_c(ls[j]) and ls[j].strip().startswith("///"):
                i = j + 1
                continue
        break
    return i + 1


def R(key, name, which=1):
    ls = _ls(key)
    stack, out = [], []
    for i, l in enumerate(ls, 1):
        m = re.match(r"^\s*//\s?#region\s*(.*)$", l)
        if m:
            stack.append((i, m.group(1).strip()))
            continue
        m = re.match(r"^\s*//\s?#endregion\b", l)
        if m and stack:
            s, n = stack.pop()
            if n == name:
                out.append((s, i))
    out.sort()
    return out[which - 1]


def OPEN(key, name):
    for i, l in enumerate(_ls(key), 1):
        m = re.match(r"^\s*//\s?#region\s*(.*)$", l)
        if m and m.group(1).strip() == name:
            return i
    raise KeyError((key, name))


def END(key):
    return len(_ls(key))


def cuts(key, starts, end):
    out = []
    for i, s in enumerate(starts):
        out.append((s, (starts[i + 1] - 1) if i + 1 < len(starts) else end))
    return out


GLTF_MODEL_LIB = f"{GLTF}/🧬️schema/📦️packages/🦀️rust/🦀️.rs"
GLTF_INF_LIB = f"{GLTF}/🧬️schema/💡️inferences/📦️packages/🦀️rust/🦀️.rs"
GLTF_ORA_LIB = f"{ART}/🧊️gltf/🔮️oracles/🦀️.rs"
OBJ_DIFF_LIB = f"{OBJD}/📦️packages/🦀️rust/🦀️.rs"
PLY_INF_LIB = f"{ART}/🧱️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/💡️inferences/📦️packages/🦀️rust/🦀️.rs"


def _kids(key, specs):
    out = []
    for folder, mod, doc, ranges, extra in specs:
        d = dict(folder=folder, mod=mod, ranges=ranges, doc=doc)
        d.update(extra)
        out.append(d)
    return out


def _plan(key, rel, lib, specs):
    return dict(rel=rel, mode="mod", lib=lib, children=_kids(key, specs))


def objd():
    k = "objd"
    return _plan(k, f"{OBJD}/🦀️.rs", OBJ_DIFF_LIB, [
        ("🧮️indexed-collections", "indexed_collections", "🧮️ The shared index-collection algebra and the vertex, texture coordinate, normal and face diffs built on it.", [(R(k, "IndexCollectionCore")[0], R(k, "🔖️FaceDiff")[1])], {}),
        ("🏷️named-collections-and-diff", "named_collections_and_diff", "🏷️ The name-keyed group and object diffs and the aggregate `ObjDiff` with its validation, apply, inverse and absorb.", [(R(k, "🔖️GroupDiff")[0], R(k, "🔖️Diff")[1])], {}),
        ("🧵️codec-builders-and-demos", "codec_builders_and_demos", "🧵️ The hand-rolled `ObjDiff` text and binary codec, the mutation diff builders and the demo diff cases.", [(R(k, "🔖️HandcraftedDiffCodec")[0], R(k, "🔖️DemoCases")[1])], {}),
    ])


def gor():
    k = "gor"
    half = S(k, "asset_object")
    return _plan(k, f"{GLTF}/🔮️oracles/🦀️.rs", GLTF_ORA_LIB, [
        ("🧰️glb-container-and-json-helpers", "glb_container_and_json_helpers", "🧰️ The independent GLB container codec, the JSON tree and parameter helpers and the host JSON bridge.", [(R(k, "🔖️Container")[0], R(k, "🔖️JsonBridge")[1])], {}),
        ("🎬️scene-node-and-collection-kinds", "scene_node_and_collection_kinds", "🎬️ Oracle kinds for scene roots, node children, material flags and the camera, skin, animation and extension collections.", [(R(k, "🔖️Kinds")[0], half - 1)], dict(post=["//#endregion 🔖️Kinds"])),
        ("🎨️asset-and-material-resource-kinds", "asset_and_material_resource_kinds", "🎨️ Oracle kinds for the asset block, reference remapping and the material, image, sampler and texture collections.", [(half, R(k, "🔖️Kinds")[1])], dict(pre=["//#region 🔖️Kinds"])),
        ("🏗️structure-kinds", "structure_kinds", "🏗️ The structure kinds of the scene, buffer and mesh catalogs and their shared reference remapping.", [R(k, "🔖️StructureKinds")], {}),
        ("🧭️dispatch-and-projection", "dispatch_and_projection", "🧭️ The oracle kind dispatch, the round trip and the normalized `project_gltf` projection.", [(R(k, "🔖️Dispatch")[0], R(k, "🔖️Projection")[1])], {}),
    ])


def gbw():
    k = "gbw"
    return _plan(k, f"{GLTF}/🚪️io/💾️binary/🔺️diff/🧵️wire/🦀️.rs", GLTF_MODEL_LIB, [
        ("🧱️scalar-and-enum-codecs", "scalar_and_enum_codecs", "🧱️ Binary primitives, option, tri-state and vector framing, JSON blobs and the glTF enum codecs.", [(S(k, "read_bin_blob"), S(k, "write_bin_asset_diff") - 1)], {}),
        ("🧊️scene-mesh-and-accessor-codecs", "scene_mesh_and_accessor_codecs", "🧊️ Binary codecs of the asset, scene, node, mesh, sparse and accessor elements and their diffs.", [(S(k, "write_bin_asset_diff"), S(k, "read_bin_texture_info") - 1)], {}),
        ("🎨️material-and-resource-codecs", "material_and_resource_codecs", "🎨️ Binary codecs of the texture infos, PBR material, buffer, buffer view, texture, image, sampler and skin elements.", [(S(k, "read_bin_texture_info"), S(k, "write_bin_animation_channel_target") - 1)], {}),
        ("🎬️animation-and-camera-codecs", "animation_and_camera_codecs", "🎬️ Binary codecs of the animation, camera and attribute elements and the generic collection triple.", [(S(k, "write_bin_animation_channel_target"), T(k, "macro_rules! delta_bin_codec") - 1)], {}),
        ("🔺️delta-diff-and-writer-codecs", "delta_diff_and_writer_codecs", "🔺️ The list delta codec macro and its instances, the per-element diff codecs and the late writers.", [(T(k, "macro_rules! delta_bin_codec"), END(k))], {}),
    ])


def gtw():
    k = "gtw"
    return _plan(k, f"{GLTF}/🚪️io/📝️text/🔺️diff/🧵️wire/🦀️.rs", GLTF_MODEL_LIB, [
        ("🧱️scalar-and-enum-codecs", "scalar_and_enum_codecs", "🧱️ Text primitives, option framing, hex, JSON and f64 helpers and the glTF enum codecs.", [(S(k, "hex_encode"), S(k, "enc_asset_diff") - 1)], {}),
        ("🧊️scene-mesh-and-accessor-codecs", "scene_mesh_and_accessor_codecs", "🧊️ Text codecs of the asset, scene, node, mesh, sparse and accessor elements and their diffs.", [(S(k, "enc_asset_diff"), S(k, "enc_texture_info") - 1)], {}),
        ("🎨️material-and-resource-codecs", "material_and_resource_codecs", "🎨️ Text codecs of the texture infos, PBR material, buffer, buffer view, texture, image, sampler and skin elements.", [(S(k, "enc_texture_info"), S(k, "enc_animation_channel_target") - 1)], {}),
        ("🎬️animation-and-camera-codecs", "animation_and_camera_codecs", "🎬️ Text codecs of the animation and camera elements.", [(S(k, "enc_animation_channel_target"), T(k, "Generic index-keyed collection triple codec") - 1)], {}),
        ("🔺️delta-diff-and-print-codecs", "delta_diff_and_print_codecs", "🔺️ The list delta codec macro and its instances, the per-element diff codecs, the generic collection triple and the `GltfDiff` print and parse.", [(T(k, "Generic index-keyed collection triple codec"), END(k))], {}),
    ])


def ggk():
    k = "ggk"
    a, b = S(k, "GltfPairGeometry"), S(k, "shell_material_metrics")
    return _plan(k, f"{GLTF}/🧬️schema/💡️inferences/🔨️geometry-kernel/🦀️.rs", GLTF_INF_LIB, [
        ("🧮️vector-math-and-part-decoding", "vector_math_and_part_decoding", "🧮️ Vector and matrix math, node transforms, fingerprints, triangle decoding, part collection and bounds.", [(OPEN(k, "🧮️Kernel"), a - 1)], dict(post=["//#endregion 🧮️Kernel"])),
        ("🕸️pair-geometry-hull-and-topology", "pair_geometry_hull_and_topology", "🕸️ Pair geometry sampling, convex hull metrics, thickness and roughness samples, mesh topology and distance queries.", [(a, b - 1)], dict(pre=["//#region 🧮️Kernel"], post=["//#endregion 🧮️Kernel"])),
        ("🧊️shell-metrics-and-geometry-context", "shell_metrics_and_geometry_context", "🧊️ Shell material metrics, statistics, the principal frame and the `GltfGeometryContext` aggregate.", [(b, T(k, "#[cfg(test)]") - 1)], dict(pre=["//#region 🧮️Kernel"], post=["//#endregion 🧮️Kernel"])),
    ])


def gdf():
    k = "gdf"
    return _plan(k, f"{GLTF}/🧬️schema/🔺️diff/🦀️.rs", GLTF_MODEL_LIB, [
        ("🧮️collection-algebra", "collection_algebra", "🧮️ Index transport, the generic collection algebra and the generic collection diff with its list delta macros.", [(R(k, "🔖️IndexTransport")[0], R(k, "🔖️GenericCollectionDiff")[1])], dict(macro_use=True)),
        ("📐️structural-element-diffs", "structural_element_diffs", "📐️ The asset, scene, node, mesh and accessor element diffs.", [(R(k, "🔖️AssetDiff")[0], R(k, "🔖️AccessorDiff")[1])], {}),
        ("🎨️resource-element-diffs", "resource_element_diffs", "🎨️ The material, buffer, primitive, texture, image, buffer view, skin and animation element diffs and the collection aliases.", [(R(k, "🔖️MaterialDiff")[0], R(k, "🔖️CollectionTypeAliases")[1])], {}),
        ("🧊️gltf-diff-aggregate", "gltf_diff_aggregate", "🧊️ The aggregate `GltfDiff` with its touched regions, validation, apply, inverse, absorb and demo cases.", [R(k, "🔖️Diff")], {}),
    ])


def plyd():
    k = "plyd"
    return _plan(k, f"{PLYD}/🦀️.rs", PLY_INF_LIB, [
        ("🧮️indexed-triple-algebra", "indexed_triple_algebra", "🧮️ The generic index-keyed removed, modified and added triple with its apply, validate, absorb and inverse algebra.", [R(k, "🔖️CommentsTriple")], {}),
        ("📊️row-and-element-diffs", "row_and_element_diffs", "📊️ The row field, row, element and element-collection diffs with their position-aware absorb.", [(R(k, "🔖️RowFieldDiff")[0], R(k, "🔖️ElementsTriple")[1])], {}),
        ("🧊️ply-diff-aggregate", "ply_diff_aggregate", "🧊️ The aggregate `PlyDiff` with its validation, apply, inverse and the mutation diff builders.", [(R(k, "🔖️ElementsTriple")[1] + 1, R(k, "🔖️HandcraftedDiffCodec")[0] - 1)], {}),
        ("🧵️codec-and-demo-cases", "codec_and_demo_cases", "🧵️ The hand-rolled `PlyDiff` text and binary codec and the demo diff cases.", [(R(k, "🔖️HandcraftedDiffCodec")[0], R(k, "🔖️DemoDiffCases")[1])], {}),
    ])


PLANS = {"objd": objd(), "gor": gor(), "gbw": gbw(), "gtw": gtw(), "ggk": ggk(), "gdf": gdf(), "plyd": plyd()}
ORDER = ["objd", "gor", "gbw", "gtw", "ggk", "gdf", "plyd"]
