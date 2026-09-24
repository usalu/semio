#!/usr/bin/env python3
"""⚖️ T12 / S15 stdio ×9: appends one live kit-verb law per stdio editor (dispatched as the shells send
it, settled through the host's publication loop, read back from the snapshot) and gives the five crates
without it the `artifact-app-testing` dev feature. Usage: stdio-kit-laws.py [--write]"""
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts")
DEV_FEATURE = 'semio-framework-plugin = { workspace = true, features = ["artifact-app-testing"] }\n'

HELPERS = '''
//#region 🪟️KitVerbLaws
type KitFixtureApp = semio_framework_plugin::VcsArtifactApp<EditorApp<{editor}>>;

/// 🧪️ One registered fixture app holding `document`, loaded exactly as a host applies the example
/// switch's `Effect::LoadDocument`.
async fn kit_fixture_holding(document: &{snapshot}) -> KitFixtureApp {{
    use semio_framework_plugin::PluginApp;
    let mut app = semio_framework_plugin::artifact_app_laws::new_registered_app::<EditorApp<{editor}>, _>(async {{ semio_framework_plugin::App {{ definition: {create}(), examples: Vec::new() }} }}).await;
    let semio_framework_plugin::Effect::LoadDocument {{ pack, spr }} = semio_s_artifact_stdio_contract::load_example_effect(document, {schema}) else {{
        panic!("the example switch hands the host one whole document")
    }};
    app.load_document_pack(&store::ArtifactPackFiles {{ pack, spr, ops: String::new() }}).await.expect("the host loads the example document");
    app
}}

/// 🕹️ Dispatches `action` with text-staged `args`, exactly as a rail sends it, and settles it through
/// the host's bounded publication loop.
async fn dispatch_settled(app: &mut KitFixtureApp, action: &str, args: &[(&str, &str)]) -> Result<(), Fault> {{
    use semio_framework_plugin::PluginApp;
    let meta = semio_framework_plugin::artifact_app_laws::meta("local");
    let args = dsl::DslValue::object(args.iter().map(|(key, value)| ((*key).to_string(), dsl::DslValue::String((*value).to_string()))).collect::<Vec<_>>());
    app.handle_action(action, Some(&args), &meta).await?;
    semio_framework_plugin::artifact_app_laws::settle_registered_typed_operation(app, meta.instance_id).await.map(|_| ())
}}

'''

LAW_HEAD = '''/// ⚖️ LAW: `{verb}` — the verb {kit} mints for `🪟️main` — reaches the document through this
/// editor's exact retained factory. Unregistered, the reactor refused it inside `s` with
/// `interactive-job.missing-factory` (S15, session 11).
#[semio_framework_async_macros::async_test]
async fn the_kit_verb_edits_the_document_through_its_exact_retained_factory() {{
'''
TAIL = '''    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut app);
}
'''
INVALID_TEXT = '''
/// ⚖️ LAW: `replace-text` with text that is not this artifact's DSL is refused loudly with
/// `stdio.{fn}.invalid-text` and leaves the document untouched — it used to answer an empty emit, which
/// read as an accepted edit that moved nothing.
#[semio_framework_async_macros::async_test]
async fn replace_text_refuses_text_that_is_not_the_artifacts_dsl() {{
    let example = {example}({example_id});
    let mut app = kit_fixture_holding(&example).await;
    let fault = dispatch_settled(&mut app, "replace-text", &[("text", "not a dsl document")]).await.expect_err("invalid text is refused");
    assert!(format!("{{fault:?}}").contains("stdio.{fn}.invalid-text"), "{{fault:?}}");
    assert_eq!(app.snapshot().expect("{fn} snapshot"), example);
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut app);
}}
'''

EDITORS = [
    {
        "dir": "📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any", "crate": "📊️csv", "fn": "csv", "editor": "CsvEditor", "snapshot": "CsvSnapshot", "create": "create_csv_editor",
        "schema": "STDIO_CSV_DOCUMENT_SCHEMA", "example": "csv_example_snapshot", "example_id": "crate::examples::demo::ID", "verb": "set-cell", "kit": "the `TableWindowKit`",
        "body": '''    let mut app = kit_fixture_holding(&csv_example_snapshot(crate::examples::demo::ID)).await;
    dispatch_settled(&mut app, "set-cell", &[("row", "0"), ("column", "0"), ("value", "Zeta")]).await.expect("set-cell settles");
    let after = app.snapshot().expect("csv snapshot");
    assert_eq!(after.records[grid_row_to_record_index(after.has_header, 0)].fields[0].value, "Zeta");
''',
    },
    {
        "dir": "📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any", "crate": "📑️tsv", "fn": "tsv", "editor": "TsvEditor", "snapshot": "TsvSnapshot", "create": "create_tsv_editor",
        "schema": "STDIO_TSV_DOCUMENT_SCHEMA", "example": "tsv_example_snapshot", "example_id": "crate::examples::demo::ID", "verb": "set-cell", "kit": "the `TableWindowKit`",
        "body": '''    let mut app = kit_fixture_holding(&tsv_example_snapshot(crate::examples::demo::ID)).await;
    dispatch_settled(&mut app, "set-cell", &[("row", "1"), ("column", "1"), ("value", "Oak Board")]).await.expect("set-cell settles");
    assert_eq!(app.snapshot().expect("tsv snapshot").records[1][1], "Oak Board");
''',
    },
    {
        "dir": "🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any", "crate": "🔤️txt", "fn": "txt", "editor": "TxtEditor", "snapshot": "TxtSnapshot", "create": "create_txt_editor",
        "schema": "STDIO_TXT_DOCUMENT_SCHEMA", "example": "txt_example_snapshot", "example_id": "crate::examples::demo::ID", "verb": "replace-text", "kit": "the `TextWindowKit`",
        "body": '''    let mut app = kit_fixture_holding(&txt_example_snapshot(crate::examples::demo::ID)).await;
    dispatch_settled(&mut app, "replace-text", &[("text", "alpha\\nbeta\\n")]).await.expect("replace-text settles");
    let after = app.snapshot().expect("txt snapshot");
    assert_eq!(after.lines, vec!["alpha".to_string(), "beta".to_string()]);
    assert!(after.trailing_newline);
''',
    },
    {
        "dir": "📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any", "crate": "📝️md", "fn": "md", "editor": "MdEditor", "snapshot": "MdSnapshot", "create": "create_md_editor",
        "schema": "STDIO_MD_DOCUMENT_SCHEMA", "example": "md_example_snapshot", "example_id": "crate::examples::demo::ID", "verb": "replace-text", "kit": "the `TextWindowKit`", "invalid": True,
        "body": '''    let target = md_example_snapshot(crate::examples::demo::ID);
    let mut app = kit_fixture_holding(&MdSnapshot::default()).await;
    dispatch_settled(&mut app, "replace-text", &[("text", &<MdSnapshot as store::ArtifactDsl>::print_dsl(&target))]).await.expect("replace-text settles");
    assert_eq!(app.snapshot().expect("md snapshot"), target);
''',
    },
    {
        "dir": "🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any", "crate": "🌐️html", "fn": "html", "editor": "HtmlEditor", "snapshot": "HtmlSnapshot", "create": "create_html_editor",
        "schema": "STDIO_HTML_DOCUMENT_SCHEMA", "example": "html_example_snapshot", "example_id": "crate::examples::demo::ID", "verb": "replace-text", "kit": "the `TextWindowKit`", "invalid": True,
        "body": '''    let target = html_example_snapshot(crate::examples::demo::ID);
    let mut app = kit_fixture_holding(&HtmlSnapshot::default()).await;
    dispatch_settled(&mut app, "replace-text", &[("text", &<HtmlSnapshot as store::ArtifactDsl>::print_dsl(&target))]).await.expect("replace-text settles");
    assert_eq!(app.snapshot().expect("html snapshot"), target);
''',
    },
    {
        "dir": "🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base", "crate": "🧾️json", "fn": "json_any", "editor": "JsonAnyEditor", "snapshot": "JsonSnapshot", "create": "create_json_editor",
        "schema": "STDIO_JSON_DOCUMENT_SCHEMA", "example": "json_any_example_snapshot", "example_id": "crate::examples::demo::ID", "verb": "set-node", "kit": "the `TreeWindowKit`",
        "body": '''    let mut app = kit_fixture_holding(&json_any_example_snapshot(crate::examples::demo::ID)).await;
    dispatch_settled(&mut app, "set-node", &[("nodeId", &main::encode_path_id(&[main::member_segment("name")])), ("value", "semio-edited")]).await.expect("set-node settles");
    let printed = <JsonSnapshot as store::ArtifactDsl>::print_dsl(&app.snapshot().expect("json snapshot"));
    assert!(printed.contains("\\"semio-edited\\"") && !printed.contains("\\"semio\\","), "{printed}");
''',
    },
    {
        "dir": "🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🛜️i-json", "crate": None, "fn": "json_i_json", "editor": "JsonIJsonEditor", "snapshot": "JsonSnapshot", "create": "create_json_i_json_editor",
        "schema": "STDIO_JSON_DOCUMENT_SCHEMA", "example": "json_i_json_example_snapshot", "example_id": "crate::standards::v_rfc8259::subsets::i_json::examples::demo::ID", "verb": "set-node", "kit": "the `TreeWindowKit`",
        "body": '''    let mut app = kit_fixture_holding(&json_i_json_example_snapshot(crate::standards::v_rfc8259::subsets::i_json::examples::demo::ID)).await;
    dispatch_settled(&mut app, "set-node", &[("nodeId", &main::encode_path_id(&[main::member_segment("id")])), ("value", "semio.stdio.i-json.edited")]).await.expect("set-node settles");
    let printed = <JsonSnapshot as store::ArtifactDsl>::print_dsl(&app.snapshot().expect("json snapshot"));
    assert!(printed.contains("semio.stdio.i-json.edited") && !printed.contains("semio.stdio.i-json.demo"), "{printed}");
''',
    },
    {
        "dir": "📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base", "crate": "📰️xml", "fn": "xml_any", "editor": "XmlAnyEditor", "snapshot": "XmlSnapshot", "create": "create_xml_editor",
        "schema": "STDIO_XML_DOCUMENT_SCHEMA", "example": "xml_any_example_snapshot", "example_id": "crate::examples::demo::ID", "verb": "set-node", "kit": "the `TreeWindowKit`",
        "body": '''    let mut app = kit_fixture_holding(&xml_any_example_snapshot(crate::examples::demo::ID)).await;
    dispatch_settled(&mut app, "set-node", &[("nodeId", &main::encode_node_path(&[0, 0])), ("value", "Ada")]).await.expect("set-node settles");
    let printed = <XmlSnapshot as store::ArtifactDsl>::print_dsl(&app.snapshot().expect("xml snapshot"));
    assert!(printed.contains("Ada") && !printed.contains("Tove"), "{printed}");
''',
    },
    {
        "dir": "📰️xml/🏅️standards/🔖️1.0/🪆️subsets/✅️valid", "crate": None, "fn": "xml_valid", "editor": "XmlValidEditor", "snapshot": "XmlSnapshot", "create": "create_xml_valid_editor",
        "schema": "STDIO_XML_DOCUMENT_SCHEMA", "example": "xml_valid_example_snapshot", "example_id": "crate::standards::v1_0::subsets::valid::examples::demo::ID", "verb": "set-node", "kit": "the `TreeWindowKit`",
        "body": '''    let mut app = kit_fixture_holding(&xml_valid_example_snapshot(crate::standards::v1_0::subsets::valid::examples::demo::ID)).await;
    dispatch_settled(&mut app, "set-node", &[("nodeId", &main::encode_node_path(&[0, 0, 0])), ("value", "Stone Forest")]).await.expect("set-node settles");
    let printed = <XmlSnapshot as store::ArtifactDsl>::print_dsl(&app.snapshot().expect("xml snapshot"));
    assert!(printed.contains("Stone Forest") && !printed.contains("Concrete Forest"), "{printed}");
''',
    },
]


def main() -> None:
    write = "--write" in sys.argv
    for spec in EDITORS:
        tests = ROOT / spec["dir"] / "✏️editor" / "🧪️tests" / "🔬️unit" / "🦀️.rs"
        text = tests.read_text()
        if "🪟️KitVerbLaws" in text:
            raise SystemExit(f"{tests}: already carries the kit-verb laws")
        block = HELPERS.format(**spec) + LAW_HEAD.format(**spec) + spec["body"] + TAIL
        if spec.get("invalid"):
            block += INVALID_TEXT.format(**spec)
        block += "//#endregion 🪟️KitVerbLaws\n"
        if write:
            tests.write_text(text.rstrip("\n") + "\n" + block)
            print(f"wrote {tests.relative_to(ROOT)}")
        else:
            print(f"== {tests.relative_to(ROOT)}\n{block}")
        if spec["crate"]:
            manifest = ROOT / spec["crate"] / "📦️packages" / "🦀️rust" / "Cargo.toml"
            cargo = manifest.read_text()
            if "artifact-app-testing" in cargo:
                continue
            anchor = "semio-framework-async-macros = { workspace = true }\n"
            if cargo.count(anchor) != 1 or "[dev-dependencies]\n" not in cargo:
                raise SystemExit(f"{manifest}: dev-dependency anchor")
            if write:
                manifest.write_text(cargo.replace(anchor, anchor + DEV_FEATURE))
                print(f"wrote {manifest.relative_to(ROOT)}")


if __name__ == "__main__":
    main()
