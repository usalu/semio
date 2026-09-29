#!/usr/bin/env python3
"""🧪️ U6 set A2 — stdio lib-test RUNTIME drift after set A compiled (test-only, rule 22): bcf, txt, wav.

* bcf  the topic table renders as a `Table` of `topic-<row>` rows with `cell-<col>` `Input`s bound to `set-cell` (no `TableScene`
       surface) → the address/revision law reads the cell input and its binding; set A's own hunk drops two needless qualifications.
* txt  mutation provenance names the derive's authority projection (`MUTATION_AUTHORITY_LOCATOR`, the derive's own law), not the
       taxonomy; the draft text travels in scene lanes (`SceneDoc::merge_lane` over the carrier children, the details-panel idiom);
       set A's `MutationDiff::apply(Mutation::diff(..))` translation now refuses an outcome that carries messages — the old
       `Mutation::apply` failed on them, applying the empty diff silently would hide a refusal.
* wav  snapshot edits validate against the REGISTERED document schema; a fixture editor runs without the plugin assembly that
       publishes it → the tests register `wav_artifact_schema_descriptor()` (the json editor tests' idiom).

Usage: u6-test-drift-2.py [--dry-run | --write | --revert] [--root <repo root>]"""
import hashlib
import sys
from pathlib import Path

ROOT = Path(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else Path("/Users/ueli/Documents/semio")
WRITE = "--write" in sys.argv
REVERT = "--revert" in sys.argv
BACKUP = Path("/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-u6-backup/test-drift-2")
ART = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/"
TXT = ART + "🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/"
WAV = ART + "🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs"
OLD_TAXONOMY = '    assert_eq!(provenance.taxonomy_path, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json");\n'
NEW_TAXONOMY = '    assert_eq!(provenance.taxonomy_path, "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🔣️mutation-authority.json");\n'
LANES = '''
/// 🚚️ Decodes the text surface and re-attaches every lane its carrier children hold (`SceneDoc::merge_lane`).
fn merged_scene(node: &BuiltNode) -> semio_framework_ui_scene::TextEditorScene {
    let Component::Surface(props) = &node.component else { panic!("expected a retained text surface") };
    let mut scene: semio_framework_ui_scene::TextEditorScene = semio_framework_ui_scene::decode(props).expect("decode text scene");
    for carrier in &node.children {
        let mut payload = String::new();
        let mut frontier = vec![carrier];
        while let Some(child) = frontier.pop() {
            if let Component::Text(text) = &child.component {
                payload.push_str(&text.packed_payload());
            }
            frontier.extend(child.children.iter().rev());
        }
        semio_framework_ui_scene::SceneDoc::merge_lane(&mut scene, carrier.key.as_str(), payload);
    }
    scene
}
'''
WAV_REGISTER = '''
/// 🧬️ Registers the document schema wav's declaration contributes — the registered contract every snapshot edit validates
/// against; a fixture editor runs without the plugin assembly that publishes it.
fn register_document_schema() {
    framework_schema::register_artifact_schema_descriptors(vec![crate::standards::riff_pcm::subsets::any::schema::wav_artifact_schema_descriptor()]).expect("the wav document schema registers");
}
'''

SETS = {
    ART + "💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/🖊️markup/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🧪️tests/🔬️unit/🦀️.rs": [
        ("    for locale in [semio_framework_plugin::Locale::En, semio_framework_plugin::Locale::De] {\n", "    for locale in [Locale::En, Locale::De] {\n", 1),
        (
            '''    use semio_framework_plugin::Component;
    let mut document = BcfSnapshot::default();
    document.topics.push(Default::default());
    document.topics[0].title = "Issue".into();
    let node = render_revisioned(&document, "store-revision", semio_framework_plugin::Locale::En, &semio_framework_plugin::TreeWindows::unhosted()).expect("render table");
    let Component::Surface(props) = node.component else { panic!("expected table surface") };
    let scene: semio_framework_ui_scene::TableScene = semio_framework_ui_scene::decode(&props).expect("decode scene");
    let rows: serde_json::Value = serde_json::from_str(&scene.rows_json).expect("rows JSON");
    assert_eq!(rows[0]["1"]["value"], "Issue");
    assert_eq!(rows[0]["1"]["action"]["args"]["row"], 0);
    assert_eq!(rows[0]["1"]["action"]["args"]["column"], 1);
    assert_eq!(rows[0]["1"]["action"]["args"]["revision"], "store-revision");
}
''',
            '''    use semio_framework_plugin::{Component, UiValue};
    fn node_by_key<'a>(node: &'a BuiltNode, key: &str) -> Option<&'a BuiltNode> {
        if node.key.as_str() == key {
            return Some(node);
        }
        node.children.iter().find_map(|child| node_by_key(child, key))
    }
    let mut document = BcfSnapshot::default();
    document.topics.push(Default::default());
    document.topics[0].title = "Issue".into();
    let node = render_revisioned(&document, "store-revision", Locale::En, &TreeWindows::unhosted()).expect("render table");
    let row = node_by_key(&node, "topic-0").expect("topic row");
    let cell = node_by_key(row, "cell-1").expect("title cell");
    let Component::Input(props) = &cell.component else { panic!("the title cell is an editable input") };
    assert_eq!(props.value.as_str(), "Issue");
    let binding = cell.bindings.iter().find(|binding| binding.action.name.as_str() == "set-cell").expect("set-cell binding");
    let Some(UiValue::Map(arguments)) = &binding.args else { panic!("set-cell carries its cell address") };
    let arguments: Vec<_> = arguments.iter().collect();
    let argument = |key: &str| arguments.iter().find(|(name, _)| name.as_str() == key).map(|(_, value)| value).unwrap_or_else(|| panic!("{key} argument"));
    assert_eq!(argument("row"), &UiValue::Number(0.0));
    assert_eq!(argument("column"), &UiValue::Number(1.0));
    assert!(matches!(argument("revision"), UiValue::Text(revision) if revision.as_str() == "store-revision"));
}
''',
            1,
        ),
    ],
    TXT + "✏️editor/🧪️tests/🔬️unit/🦀️.rs": [
        (
            '''        next = protocol::MutationDiff::apply(<TxtMutation as protocol::Mutation<TxtSnapshot>>::diff(mutation, &next).diff(), &next).expect("native mutation applies");
''',
            '''        let outcome = <TxtMutation as protocol::Mutation<TxtSnapshot>>::diff(mutation, &next);
        assert!(outcome.messages().is_empty(), "native mutation {mutation:?} refused: {:?}", outcome.messages());
        next = protocol::MutationDiff::apply(outcome.diff(), &next).expect("native mutation applies");
''',
            1,
        ),
    ],
    TXT + "✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🧪️tests/🔬️unit/🦀️.rs": [
        ("use semio_framework_plugin::Component;\n", "use semio_framework_plugin::Component;\n" + LANES, 1),
        ('''    let Component::Surface(props) = node.component else { panic!("expected a retained text surface") };
    let scene: semio_framework_ui_scene::TextEditorScene = semio_framework_ui_scene::decode(&props).expect("decode text scene");
    assert_eq!(scene.buffer, "a\\nb");
''', '''    let scene = merged_scene(&node);
    assert_eq!(scene.buffer, "a\\nb");
''', 1),
    ],
    TXT + "👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🧪️tests/🔬️unit/🦀️.rs": [
        ("use semio_framework_plugin::Component;\n", "use semio_framework_plugin::Component;\n" + LANES, 1),
        ('''    let Component::Surface(props) = node.component else { panic!("expected a retained text surface") };
    let scene: semio_framework_ui_scene::TextEditorScene = semio_framework_ui_scene::decode(&props).expect("decode text scene");
    assert_eq!(scene.buffer, "a\\nb");
''', '''    assert_eq!(merged_scene(&node).buffer, "a\\nb");
''', 1),
    ],
    **{TXT + f"🧬️schema/🧬️mutations/{leaf}/🧪️tests/🔬️unit/🦀️.rs": [(OLD_TAXONOMY, NEW_TAXONOMY, 1)] for leaf in ["📥️insert-line", "🗑️remove-line", "✏️set-line", "🔚️set-line-ending", "↩️set-trailing-newline"]},
    WAV: [
        ("use crate::standards::riff_pcm::subsets::any::schema::mutations::set_snapshot;\n", "use crate::standards::riff_pcm::subsets::any::schema::mutations::set_snapshot;\n" + WAV_REGISTER, 1),
        ("async fn one_mebibyte_sample_lane_edits_without_generic_value_expansion() {\n", "async fn one_mebibyte_sample_lane_edits_without_generic_value_expansion() {\n    register_document_schema();\n", 1),
        ("async fn data_kind_edit_publishes_the_requested_variant_and_reopens_natively() {\n", "async fn data_kind_edit_publishes_the_requested_variant_and_reopens_natively() {\n    register_document_schema();\n", 1),
        ("async fn chunk_layout_and_pad_bytes_are_visible_and_editable_details() {\n", "async fn chunk_layout_and_pad_bytes_are_visible_and_editable_details() {\n    register_document_schema();\n", 1),
        ("async fn large_sample_edit_publishes_cancels_undoes_redoes_and_preserves_metadata() {\n", "async fn large_sample_edit_publishes_cancels_undoes_redoes_and_preserves_metadata() {\n    register_document_schema();\n", 1),
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
        print(f"{'WRITE' if WRITE else 'DRY'} {rel}: {len(hunks)} hunks")
    if WRITE and problems == 0:
        BACKUP.mkdir(parents=True, exist_ok=True)
        for rel, path, text in planned:
            (BACKUP / key(rel)).write_bytes(path.read_bytes())
            path.write_text(text)
    print(f"{'write' if WRITE and problems == 0 else 'dry-run'}: {len(SETS)} files, {problems} problems")
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main())
