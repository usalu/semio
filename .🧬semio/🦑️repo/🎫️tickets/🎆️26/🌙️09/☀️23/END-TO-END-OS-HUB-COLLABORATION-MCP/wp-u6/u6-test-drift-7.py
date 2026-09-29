#!/usr/bin/env python3
"""🧪️ U6 set A7 — stdio lib-test drift behind the 55-red count of 2026-09-29 10:28 (test-only, rule 22).

* Scene lanes (09-27): `scene_surface` splits a scene's payload into lane carriers (`TextEditorScene.buffer`,
  `TableScene.columnsJson/rowsJson`), so a bare `semio_framework_ui_scene::decode` reads an empty spine → the laws read the
  ASSEMBLED scene (`artifact_app_laws::built_surface_scene`, or `decode_fixture_scene_with_lanes` over the projected
  fixture tree for the windowed pdf document window) — binary ×4, deflate ×3, csv/tsv/epw viewers + epw editor, pdf ×10.
* Paged children: a windowed table body refuses a direct serde walk ("BuiltChildren requires retained page transport") →
  the csv/tsv editor laws project the body through `project_and_retire_fixture_tree` (which also retires it).
* Registered document schema (09-27 `snapshot-edit.schema-unregistered`): a fixture editor runs without the plugin assembly
  that publishes its document schema → png/jpg/tiff laws register it first (the json/wav/mp4 idiom).
* Window-owned actions live on their window kind (json idiom) → png's action roster laws look there too.
* pdf 1.7 gained `set-snapshot` (09-27) → 61 kinds; gltf likewise → 121; pptx diffs are tagged `shapeKind` (09-28, the
  `kind` field of `Placeholder` would collide) → the law asserts that tag; the provenance taxonomy path is the mutation
  authority (dsl derive, 09-26; txt laws already updated).
* Fixtures made valid, never weakened: the mp4 base keeps its chunk table consistent with the pushed sample; the raw ZIP64
  central header declares version-needed 4.5 (APPNOTE 4.4.3.2); `λ` (not in CP437) is the unencodable name (`δ` is CP437
  0xEB); the shadow law inspects a snapshot WITH an entry (an empty one prints no entry header state); the pptx undo law
  compares against the document the host opened (a codec fixed point), asserting the authored presentation survives it.
* dev-dependencies: binary/deflate/epw gain `semio-framework-plugin` + `artifact-app-testing` (already a normal
  dependency → no lock change); pdf gains `semio-framework-ui-scene` (one Cargo.lock dependency line).

Usage: u6-test-drift-7.py [--dry-run | --write | --revert] [--root <repo root>]"""
import hashlib
import sys
from pathlib import Path

ROOT = Path(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else Path("/Users/ueli/Documents/semio")
WRITE = "--write" in sys.argv
REVERT = "--revert" in sys.argv
BACKUP = Path("/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-u6-backup/test-drift-7")
ART = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/"
MAIN_EDIT = "✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🧪️tests/🔬️unit/🦀️.rs"
MAIN_VIEW = "👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🧪️tests/🔬️unit/🦀️.rs"
EDITOR = "✏️editor/🧪️tests/🔬️unit/🦀️.rs"
BINARY = ART + "💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/"
DEFLATE = ART + "🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/"
CSV = ART + "📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/"
TSV = ART + "📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/"
EPW = ART + "🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/"
PNG = ART + "📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/"
JPG = ART + "📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/"
TIFF = ART + "🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/"
PDF = ART + "📖️pdf/🏅️standards/"
PPTX = ART + "📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/"
GLTF = ART + "🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/"
MP4 = ART + "🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🧬️schema/🧬️mutations/"
ZIP = ART + "🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/"

IMPORT = ("use super::*;\nuse semio_framework_plugin::Component;\n", "use super::*;\n", 1)
TEXT_SCENE = (
    '    let Component::Surface(props) = node.component else { panic!("expected a retained text surface") };\n'
    '    let scene: semio_framework_ui_scene::TextEditorScene = semio_framework_ui_scene::decode(&props).expect("decode text scene");\n',
    '    let scene: semio_framework_ui_scene::TextEditorScene = semio_framework_plugin::artifact_app_laws::built_surface_scene(&node).expect("decode the text scene with its lanes");\n',
)
TABLE_SCENE = (
    '    let Component::Surface(props) = node.component else { panic!("expected a retained table surface") };\n'
    '    let scene: semio_framework_ui_scene::TableScene = semio_framework_ui_scene::decode(&props).expect("decode table scene");\n',
    '    let scene: semio_framework_ui_scene::TableScene = semio_framework_plugin::artifact_app_laws::built_surface_scene(&node).expect("decode the table scene with its lanes");\n',
    1,
)
PROJECTED_TABLE = (
    '    let json = serde_json::to_string(&node).expect("declarative table json");\n',
    '    let json = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("declarative table json");\n',
)
PLUGIN_DEV = (
    "[dev-dependencies]\nsemio-framework-async-macros = { workspace = true }\n",
    '[dev-dependencies]\nsemio-framework-async-macros = { workspace = true }\nsemio-framework-plugin = { workspace = true, features = ["artifact-app-testing"] }\n',
    1,
)


def register(descriptor: str, artifact: str) -> tuple:
    return (
        "use super::*;\n",
        "use super::*;\n\n"
        f"/// 🧬️ Registers the document schema {artifact}'s declaration contributes — the contract every snapshot edit validates against;\n"
        "/// a fixture editor runs without the plugin assembly that publishes it.\n"
        "fn register_document_schema() {\n"
        f'    framework_schema::register_artifact_schema_descriptors(vec![{descriptor}()]).expect("the {artifact} document schema registers");\n'
        "}\n",
        1,
    )


def first_statement(signature: str) -> tuple:
    return (signature + "\n", signature + "\n    register_document_schema();\n", 1)


JPG_DESCRIPTOR = "crate::standards::v_jfif_1_01::subsets::document::schema::jpg_artifact_schema_descriptor"
TIFF_DESCRIPTOR = "crate::standards::v6_0::subsets::document::schema::tiff_artifact_schema_descriptor"
PNG_DESCRIPTOR = "crate::standards::v1_2::subsets::any::schema::png_artifact_schema_descriptor"
PAYLOAD_LAW = "fn payload_detail_edits_publish_the_exact_requested_value() {"
PDF_VIEWERS = ["4️⃣1.4/🪆️subsets/🧱️base", "4️⃣1.4/🪆️subsets/🗄️a", "4️⃣1.4/🪆️subsets/🖨️x", "7️⃣1.7/🪆️subsets/🧱️base", "7️⃣1.7/🪆️subsets/🗄️a", "7️⃣1.7/🪆️subsets/📐️e", "7️⃣1.7/🪆️subsets/⚕️h", "7️⃣1.7/🪆️subsets/♿️ua", "7️⃣1.7/🪆️subsets/🧾️vt", "7️⃣1.7/🪆️subsets/🖨️x"]
PDF_PAGE = (
    '    let semio_framework_ui_contract::Component::Text(text_node) = &node.children[0].component else { panic!("expected Text") };\n'
    '    assert!(text_node.value.0.contains("MediaBox"));\n',
    '    let projection = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("project the rendered page window");\n'
    '    let page: semio_framework_ui_scene::TextEditorScene = semio_framework_plugin::artifact_app_laws::decode_fixture_scene_with_lanes(&projection).expect("the page renders as a read-only text scene with its lanes");\n'
    '    assert!(page.buffer.contains("MediaBox"));\n',
    1,
)
SETS = {
    ART + "💾️binary/📦️packages/🦀️rust/Cargo.toml": [PLUGIN_DEV],
    BINARY + MAIN_EDIT: [IMPORT, (*TEXT_SCENE, 2)],
    BINARY + EDITOR: [(*TEXT_SCENE, 1)],
    BINARY + MAIN_VIEW: [IMPORT, (*TEXT_SCENE, 1)],
    ART + "🗜️deflate/📦️packages/🦀️rust/Cargo.toml": [
        (
            'semio-framework-async-macros = { workspace = true }\nsemio-framework-ui-scene = { workspace = true }\n',
            'semio-framework-async-macros = { workspace = true }\nsemio-framework-plugin = { workspace = true, features = ["artifact-app-testing"] }\nsemio-framework-ui-scene = { workspace = true }\n',
            1,
        )
    ],
    DEFLATE + MAIN_EDIT: [IMPORT, (*TEXT_SCENE, 1)],
    DEFLATE + EDITOR: [(*TEXT_SCENE, 1)],
    DEFLATE + MAIN_VIEW: [IMPORT, (*TEXT_SCENE, 1)],
    CSV + MAIN_EDIT: [(*PROJECTED_TABLE, 1)],
    CSV + MAIN_VIEW: [IMPORT, TABLE_SCENE],
    TSV + MAIN_EDIT: [(*PROJECTED_TABLE, 2)],
    TSV + MAIN_VIEW: [IMPORT, TABLE_SCENE],
    ART + "🌦️epw/📦️packages/🦀️rust/Cargo.toml": [PLUGIN_DEV],
    EPW + MAIN_EDIT: [IMPORT, TABLE_SCENE],
    EPW + MAIN_VIEW: [IMPORT, TABLE_SCENE],
    PNG + EDITOR: [
        register(PNG_DESCRIPTOR, "png"),
        (
            '        let action = definition.actions.iter().find(|action| action.id == *action_id).expect("typed snapshot edit action");\n',
            '        let action = definition.actions.iter().chain(definition.window_kinds.iter().flat_map(|window| window.actions.iter())).find(|action| action.id == *action_id).expect("typed snapshot edit action (window-owned actions live on their window kind)");\n',
            1,
        ),
        (
            '    let action = definition.actions.iter().find(|action| action.id == patch_pixel_region::ACTION_ID).expect("pixel region app action");\n',
            '    let action = definition.actions.iter().chain(definition.window_kinds.iter().flat_map(|window| window.actions.iter())).find(|action| action.id == patch_pixel_region::ACTION_ID).expect("pixel region action (window-owned actions live on their window kind)");\n',
            1,
        ),
        first_statement("fn snapshot_detail_edit_round_trips_through_native_history_and_codecs() {"),
        first_statement("fn typed_snapshot_source_preserves_ancillary_and_unknown_chunk_details() {"),
        first_statement("async fn large_raster_metadata_and_pixel_edits_publish_and_replay_compactly() {"),
    ],
    JPG + "🧾️document/" + EDITOR: [register(JPG_DESCRIPTOR, "jpg"), first_statement("fn large_raster_quality_edit_uses_compact_native_event() {"), first_statement(PAYLOAD_LAW)],
    JPG + "🧱️baseline/" + EDITOR: [register(JPG_DESCRIPTOR, "jpg"), first_statement(PAYLOAD_LAW)],
    TIFF + "🧾️document/" + EDITOR: [register(TIFF_DESCRIPTOR, "tiff"), first_statement("fn large_raster_byte_order_edit_uses_compact_native_event() {"), first_statement(PAYLOAD_LAW)],
    TIFF + "🧱️baseline/" + EDITOR: [register(TIFF_DESCRIPTOR, "tiff"), first_statement(PAYLOAD_LAW)],
    ART + "📖️pdf/📦️packages/🦀️rust/Cargo.toml": [
        (
            'semio-framework-plugin = { workspace = true, features = ["artifact-app-testing"] }\n',
            'semio-framework-plugin = { workspace = true, features = ["artifact-app-testing"] }\nsemio-framework-ui-scene = { workspace = true }\n',
            1,
        )
    ],
    PDF + "7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs": [("    assert_eq!(kinds.len(), 60);\n", "    assert_eq!(kinds.len(), 61);\n", 1)],
    **{PDF + subset + "/" + MAIN_VIEW: [PDF_PAGE] for subset in PDF_VIEWERS},
    PPTX + EDITOR: [
        (
            "    let mut original = PptxSnapshot::default();\n"
            '    original.presentation.slides.push(PptxSlide { shapes: vec![PptxShape::TextBox { text_frame: vec![PptxParagraph::text("before")], position: Default::default() }] });\n',
            "    let mut authored = PptxSnapshot::default();\n"
            '    authored.presentation.slides.push(PptxSlide { shapes: vec![PptxShape::TextBox { text_frame: vec![PptxParagraph::text("before")], position: Default::default() }] });\n',
            1,
        ),
        (
            '    let semio_framework_plugin::Effect::LoadDocument { pack, spr } = semio_s_artifact_stdio_contract::load_example_effect(&original, STDIO_PPTX_DOCUMENT_SCHEMA) else { panic!("PPTX fixture produces a document load") };\n'
            "    app.load_document_pack(&store::ArtifactPackFiles { pack, spr, ops: String::new() }).await.unwrap();\n",
            '    let semio_framework_plugin::Effect::LoadDocument { pack, spr } = semio_s_artifact_stdio_contract::load_example_effect(&authored, STDIO_PPTX_DOCUMENT_SCHEMA) else { panic!("PPTX fixture produces a document load") };\n'
            "    app.load_document_pack(&store::ArtifactPackFiles { pack, spr, ops: String::new() }).await.unwrap();\n"
            "    let original = app.snapshot().unwrap().clone();\n"
            '    assert_eq!(original.presentation, authored.presentation, "the host opens the authored presentation as its canonical package");\n',
            1,
        ),
    ],
    PPTX + "🧬️schema/🧬️mutations/📸️set-snapshot/🧪️tests/🏷️retitles-and-lowers-the-title-placeholder/🦀️.rs": [
        (
            '.pointer("/presentation/slides/modified/0/diff/shapes/modified/0/diff/kind")',
            '.pointer("/presentation/slides/modified/0/diff/shapes/modified/0/diff/shapeKind")',
            1,
        ),
        (
            '"set-snapshot/retitles-and-lowers-the-title-placeholder: PptxShapeDiff is tagged `kind` — the SNAPSHOT enum\'s own tag is `shapeKind`, and mixing the two up is exactly the collision this artifact renamed around"',
            '"set-snapshot/retitles-and-lowers-the-title-placeholder: PptxShapeDiff is tagged `shapeKind` like the SNAPSHOT enum — `kind` is the Placeholder\'s own field, and an internal tag named `kind` would collide with it"',
            1,
        ),
    ],
    GLTF + "🌳️node/🏷️rename/🧪️tests/🔬️direct-leaf/🦀️.rs": [
        (
            '    assert_eq!(provenance.taxonomy_path, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json");\n',
            '    assert_eq!(provenance.taxonomy_path, "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🔣️mutation-authority.json");\n',
            1,
        )
    ],
    GLTF + "🧪️tests/🔬️unit/🦀️.rs": [
        ("    assert_eq!(GltfMutation::kinds().len(), 120);\n", "    assert_eq!(GltfMutation::kinds().len(), 121);\n", 1),
        (".collect::<std::collections::BTreeSet<_>>().len(), 120);\n", ".collect::<std::collections::BTreeSet<_>>().len(), 121);\n", 1),
    ],
    MP4 + "🧪️tests/🔬️unit/🦀️.rs": [
        (
            "    base.tracks[0].samples.push(Mp4Sample { data: vec![4, 5], duration: 33, cts_offset: 0, sync: false });\n"
            "    let m = Mp4Mutation::RemoveSample(",
            "    base.tracks[0].samples.push(Mp4Sample { data: vec![4, 5], duration: 33, cts_offset: 0, sync: false });\n"
            '    *base.tracks[0].chunk_sample_counts.last_mut().expect("the base track has a chunk") += 1;\n'
            "    let m = Mp4Mutation::RemoveSample(",
            1,
        )
    ],
    ZIP + "🚪️io/🧪️tests/🔬️codec/🦀️.rs": [
        (
            "        cen.extend_from_slice(&u32_le(SIG_CENTRAL));\n"
            "        cen.extend_from_slice(&u16_le(if e.force_zip64_sentinel { 45 } else { 20 }));\n"
            "        cen.extend_from_slice(&u16_le(20));\n",
            "        cen.extend_from_slice(&u32_le(SIG_CENTRAL));\n"
            "        cen.extend_from_slice(&u16_le(if e.force_zip64_sentinel { 45 } else { 20 }));\n"
            "        cen.extend_from_slice(&u16_le(if e.force_zip64_sentinel { 45 } else { 20 }));\n",
            1,
        ),
        ('    entry.name = "δ.txt".into();\n', '    entry.name = "λ.txt".into();\n', 1),
    ],
    ZIP + "🧬️schema/📸️snapshot/🧪️tests/🔬️shadow/🦀️.rs": [
        (
            '    let json = format!("{:?}", ZipSnapshot::default());\n',
            '    let json = format!("{:?}", ZipSnapshot { entries: vec![ZipEntry::default()], ..ZipSnapshot::default() });\n',
            1,
        )
    ],
}


def key(rel: str) -> str:
    return hashlib.sha256(rel.encode()).hexdigest()[:16]


def main() -> int:
    if REVERT:
        for rel in SETS:
            backup = BACKUP / key(rel)
            if backup.exists():
                (ROOT / rel).write_bytes(backup.read_bytes())
                backup.unlink()
                print(f"REVERTED {rel}")
        return 0
    problems, planned = 0, []
    for rel, hunks in SETS.items():
        path = ROOT / rel
        text = path.read_text()
        for old, new, count in hunks:
            found = text.count(old)
            if found != count:
                print(f"PROBLEM {rel}: {'already applied' if new in text else f'anchor count {found} != {count}'}: {old[:70]!r}")
                problems += 1
                continue
            text = text.replace(old, new)
        planned.append((rel, path, text))
        print(f"{'WRITE' if WRITE else 'DRY'} {rel.split('/🗿️artifacts/', 1)[-1][:90]}: {len(hunks)} hunks")
    if WRITE and problems == 0:
        BACKUP.mkdir(parents=True, exist_ok=True)
        for rel, path, text in planned:
            (BACKUP / key(rel)).write_bytes(path.read_bytes())
            path.write_text(text)
    print(f"{'write' if WRITE and problems == 0 else 'dry-run'}: {len(SETS)} files, {problems} problems")
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main())
