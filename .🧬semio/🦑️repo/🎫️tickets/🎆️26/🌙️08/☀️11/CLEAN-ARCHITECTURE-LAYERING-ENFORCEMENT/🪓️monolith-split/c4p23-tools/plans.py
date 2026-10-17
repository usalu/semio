"""🗺️ c4p23 split plans for Package 23: the raster artifact (schema root, diff, io, editor unit tests) and the stdio contract editing family (editing, patch, details, details unit tests).
`mode` "mod": children are `#[path]` modules (`use super::*;`, root `<vis> use <mod>::*;` when the child exports a name another file uses). `mode` "include": test fragments spliced back with `include!` (module identity and test names unchanged; relative `include_str!` paths gain `../`).
`ranges` are 1-based inclusive line ranges of the pre-edit file; each starts at an item start (docs and attributes included) and ends at an item end or its trailing region marker.
`subs` are exact one-occurrence text substitutions applied to the root after the cut (the descriptor facet leaves read the whole module tree)."""
from c4plib import *

LIB_RASTER = f"{ART}/🦀️.rs"
LIB_ED = f"{ED}/🦀️.rs"
LIB_PATCH = f"{ED}/🩹️patch/🦀️.rs"
LIB_DETAILS = f"{ED}/🪟️details/🦀️.rs"

SCHEMA_KIDS = ["🌳️layer-tree-helpers", "🗺️owned-map", "📐️layer-model", "🖼️asset-content-and-patch"]
DIFF_KIDS = ["🌲️layer-tree-apply", "🧮️inverse-and-absorb", "🖌️repaint-canvas"]


def concat_leaf(folder_prefix, kids):
    parts = [f'include_str!("{folder_prefix}🦀️.rs")'] + [f'include_str!("{folder_prefix}{k}/🦀️.rs")' for k in kids]
    return "concat!(" + ", ".join(parts) + ")"


PLANS = {
    "rsc": dict(
        rel=f"{RA}/🧬️schema/🦀️.rs", mode="mod", lib=f"{RA}/🧬️schema/🦀️.rs",
        children=[
            dict(folder="🌳️layer-tree-helpers", mod="layer_tree_helpers", ranges=[(109, 213), (216, 239), (249, 292), (323, 325)], doc="🌳️ Layer tree helpers: ids, names, protection, lookup, flattening, creation and cloning of layer nodes."),
            dict(folder="🗺️owned-map", mod="owned_map", noglob=True, ranges=[(329, 833)], doc="🗺️ The bounded page-backed owned map behind the raster assets and adjustment parameters."),
            dict(folder="📐️layer-model", mod="layer_model", ranges=[(835, 1012)], doc="📐️ Viewport, camera, transform, mask and the layer node tree of the raster document."),
            dict(folder="🖼️asset-content-and-patch", mod="asset_content_and_patch", ranges=[(1014, 1251)], doc="🖼️ Image assets, pixel and mask content, adjustment parameters, layer patches, ids and cold retirement."),
        ],
        subs=[
            ('artifact: semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript:', 'artifact: semio_framework_schema_registry::FacetLeaves { rust: ' + concat_leaf("", SCHEMA_KIDS) + ', typescript:'),
            ('rust: include_str!("🔺️diff/🦀️.rs"), typescript: include_str!("🔺️diff/🟦️.ts")', 'rust: ' + concat_leaf("🔺️diff/", DIFF_KIDS) + ', typescript: include_str!("🔺️diff/🟦️.ts")'),
        ]),
    "rdf": dict(
        rel=f"{RA}/🧬️schema/🔺️diff/🦀️.rs", mode="mod", lib=f"{RA}/🧬️schema/🔺️diff/🦀️.rs",
        children=[
            dict(folder="🌲️layer-tree-apply", mod="layer_tree_apply", ranges=[(134, 652)], doc="🌲️ Validation and application of layer patches, layer deltas, asset deltas and pixel regions onto a snapshot."),
            dict(folder="🧮️inverse-and-absorb", mod="inverse_and_absorb", ranges=[(654, 1033)], doc="🧮️ The delta algebra: exact inverses of layer deltas and absorption of a delta into an earlier one."),
            dict(folder="🖌️repaint-canvas", mod="repaint_canvas", ranges=[(1089, 1264)], doc="🖌️ Repaint support: the canvas a stroke or fill paints into, its refusals and the painted diff."),
        ]),
    "rio": dict(
        rel=f"{RA}/🚪️io/🦀️.rs", mode="mod", lib=f"{RA}/🚪️io/🦀️.rs",
        children=[
            dict(folder="🌉️semio-bridge", mod="semio_bridge", ranges=[(31, 261)], doc="🌉️ The semio image and drawing bridge: dialects, io keys, format conversions and png canonicalization."),
            dict(folder="🧩️composite-and-media", mod="composite_and_media", ranges=[(263, 509)], doc="🧩️ Composite rendering, the gif87a bridge and the media export and import ports."),
            dict(folder="🎹️derived-composition-and-registry", mod="derived_composition_and_registry", noglob=True, ranges=[(516, 757)], doc="🎹️ The derived composer composition and the derived io registry entries."),
            dict(folder="🏗️derived-construction-and-analysis", mod="derived_construction_and_analysis", ranges=[(774, 935)], doc="🏗️ The derived builder construction and analysis, asset child minting and the artifact kind."),
        ]),
    "rtu": dict(
        rel=f"{RA}/✏️editor/🧪️tests/🔬️unit/🦀️.rs", mode="include", lib=LIB_RASTER,
        children=[
            dict(folder="📥️envelope-and-scene-laws", mod="envelope_and_scene_laws", ranges=[(135, 362)], doc="📥️ Retained envelope ingress, io ports, window measures and scene rendering laws."),
            dict(folder="🧭️layer-and-camera-laws", mod="layer_and_camera_laws", ranges=[(364, 636)], doc="🧭️ Layer, camera, convergence, utility and import laws of the raster editor."),
            dict(folder="🔁️wire-and-action-bridge-laws", mod="wire_and_action_bridge_laws", ranges=[(638, 898)], doc="🔁️ Command wire, retained route and host action bridge laws."),
            dict(folder="🏔️mounted-boot-laws", mod="mounted_boot_laws", ranges=[(900, 1332)], doc="🏔️ The mounted app boot, history, selection and close laws."),
        ]),
    "edr": dict(
        rel=f"{ED}/🦀️.rs", mode="mod", lib=LIB_ED,
        children=[
            dict(folder="📜️edit-contract", mod="edit_contract", ranges=[(32, 132), (151, 240)], doc="📜️ The snapshot edit contract: action ids, events, commands, their text and binary wire and the edit error."),
            dict(folder="🪡️value-operations", mod="value_operations", ranges=[(242, 581)], doc="🪡️ Pointer operations on snapshot values, source parsing and schema validation."),
            dict(folder="📨️action-bridge", mod="action_bridge", ranges=[(583, 732)], doc="📨️ Action definitions, argument parsing and the bridge from host actions to edit events."),
            dict(folder="🧰️editor-traits-and-retained-copies", mod="editor_traits_and_retained_copies", ranges=[(734, 1024)], doc="🧰️ The snapshot editing editor traits, retained copies, close turns and native preparation routes."),
            dict(folder="🛠️tool-job-factories", mod="tool_job_factories", ranges=[(1075, 1429)], doc="🛠️ The bounded native and snapshot edit tool job factories, admission and first-step proofs."),
        ]),
    "edp": dict(
        rel=f"{ED}/🩹️patch/🦀️.rs", mode="mod", lib=LIB_PATCH,
        children=[
            dict(folder="📜️patch-wire", mod="patch_wire", ranges=[(36, 189), (194, 205)], doc="📜️ The snapshot patch value, its labels and text, hex and binary wire."),
            dict(folder="🪜️patch-steps", mod="patch_steps", ranges=[(207, 563)], doc="🪜️ Lowering a patch to primitive steps, splices and their application and validation."),
            dict(folder="🧭️patch-edits", mod="patch_edits", ranges=[(565, 821)], doc="🧭️ Preparing, applying and inverting snapshot patches against schema and dialect."),
            dict(folder="🧩️inverse-parts", mod="inverse_parts", ranges=[(823, 1120)], doc="🧩️ The bounded inverse planner: JSON length measures, shells and inverse parts."),
            dict(folder="🧬️input-schema", mod="input_schema", ranges=[(1122, 1328)], doc="🧬️ The patch input schema typed by the snapshot sub-schema a pointer addresses."),
        ]),
    "edd": dict(
        rel=f"{ED}/🪟️details/🦀️.rs", mode="mod", lib=LIB_DETAILS,
        children=[
            dict(folder="📐️detail-model", mod="detail_model", ranges=[(50, 142)], doc="📐️ The detail model: path segments, values, presentations, variants, capabilities and the provider trait."),
            dict(folder="🧬️schema-resolution", mod="schema_resolution", ranges=[(144, 622)], doc="🧬️ Schema bundling, normalization, union selection and schema navigation by path."),
            dict(folder="🧩️template-synthesis", mod="template_synthesis", ranges=[(624, 959)], doc="🧩️ Template candidates synthesized from schemas, validity checks and schema annotations."),
            dict(folder="🗂️dsl-provider", mod="dsl_provider", ranges=[(961, 1299)], doc="🗂️ The DSL snapshot details provider over a typed snapshot value."),
            dict(folder="🏷️labels-and-controls", mod="labels_and_controls", ranges=[(1301, 1632)], doc="🏷️ Localized labels and the scalar, enum, template, variant and text controls."),
            dict(folder="🌲️detail-tree-and-window", mod="detail_tree_and_window", ranges=[(1634, 2028)], doc="🌲️ Collection and item controls, the detail tree, the details window and its rendering."),
        ]),
    "edu": dict(
        rel=f"{ED}/🪟️details/🧪️tests/🔬️unit/🦀️.rs", mode="include", lib=LIB_DETAILS,
        children=[
            dict(folder="🧱️lazy-and-bounded-laws", mod="lazy_and_bounded_laws", ranges=[(6, 206)], doc="🧱️ Bounded leaf, lazy native snapshot and lazy collection details laws."),
            dict(folder="🌳️collection-and-table-laws", mod="collection_and_table_laws", ranges=[(208, 502)], doc="🌳️ Collection rows, table first paint, paging, derived schema controls and revisioned inputs."),
            dict(folder="🧬️schema-template-laws", mod="schema_template_laws", ranges=[(504, 734)], doc="🧬️ Schema templates, annotations and capabilities laws."),
            dict(folder="🔀️union-source-and-window-laws", mod="union_source_and_window_laws", ranges=[(736, 1063)], doc="🔀️ Tagged union, enum, long path, source draft and nested window laws."),
        ]),
}
ORDER = ["rsc", "rdf", "rio", "rtu", "edr", "edp", "edd", "edu"]
