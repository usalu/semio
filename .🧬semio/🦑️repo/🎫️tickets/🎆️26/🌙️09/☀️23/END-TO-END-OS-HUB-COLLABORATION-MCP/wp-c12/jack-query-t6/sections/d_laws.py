# 🧪️ Section D of `c12-jack-query-document-patch.py`: committed vectors, the demo asset and the laws that measured the old lane.
import glob as _glob

VECTORS = ANY + "🧫️fixtures/🧬️mutations/"
for path in sorted(_glob.glob(os.path.join(REPO, VECTORS, "*", "*", "📸️snapshot", "*", "🔣️.json"))):
    rel = os.path.relpath(path, REPO)
    if "🔎️set-query" in rel:
        continue
    json_fixture(rel, with_key("query", DEFAULT_QUERY), "vector snapshot " + rel.split("🧬️mutations/")[1].split("/")[0] + " " + rel.split("/")[-2])
for path in sorted(_glob.glob(os.path.join(REPO, VECTORS, "*", "*", "🔺️diff", "🔣️.json"))):
    rel = os.path.relpath(path, REPO)
    if "🔎️set-query" in rel:
        continue
    json_fixture(rel, with_key("query", None), "vector diff " + rel.split("🧬️mutations/")[1].split("/")[0])


for path in sorted(_glob.glob(os.path.join(REPO, MUT, "*", "🧪️tests", "*", "🦀️.rs"))):
    rel = os.path.relpath(path, REPO)
    leaf = rel.split("🧬️mutations/")[1].split("/")[0]
    replace_every(rel, "seven document slots", "eight document slots", f"law {leaf} slot count prose")
    replace_every(rel, "assert_eq!(slots.len(), 7,", "assert_eq!(slots.len(), 8,", f"law {leaf} slot count")

ASSET = ANY + "🖼️assets/🎬️demo/🗣️.dsl.semio"
edit(ASSET, 'root-node-id="7dc5b737-3b6b-4068-b315-b7bacc91c2e1" camera={', 'root-node-id="7dc5b737-3b6b-4068-b315-b7bacc91c2e1" query="' + DEFAULT_QUERY + '" camera={', "demo asset query")

OWNERSHIP = EDITOR + "🎭️modes/✏️edit/🪟️windows/🌐️graph/🎚️config/"
edit(OWNERSHIP + "🧪️tests/🔬️window-config-ownership/🟦️.ts", 'import { readFileSync } from "node:fs";', 'import { existsSync, readFileSync } from "node:fs";', "ownership ts import")
edit(OWNERSHIP + "🧪️tests/🔬️window-config-ownership/🟦️.ts", """  const query = fixture.queryOwnership;
  const editorSchema = JSON.parse(readFileSync(new URL("../../../../📝️editor/🎚️config/🧬️schema/🔣️.json", import.meta.url), "utf8"));
  const editorMutationSchema = JSON.parse(readFileSync(new URL("../../../../📝️editor/🎚️config/🧬️schema/🧬️mutations/🔣️.json", import.meta.url), "utf8"));
  const resultsSchema""", """  const query = fixture.resultsOwnership;
  const documentSchema = JSON.parse(readFileSync(new URL("../../../../../../../../🧬️schema/📸️snapshot/🔣️.json", import.meta.url), "utf8"));
  assert(documentSchema.required.includes("query") && documentSchema.properties.query["x-semio-state"] === "artifact", "the Jack query is document content");
  assert(!existsSync(new URL("../../../../📝️editor/🎚️config", import.meta.url)), "the query editor window carries no config lane of its own");
  const resultsSchema""", "ownership ts document query")
edit(OWNERSHIP + "🧪️tests/🔬️window-config-ownership/🟦️.ts", """    ["../../../../📝️editor/🎚️config/🧬️schema", "JackEditorWindowConfig"],
    ["../../../../📝️editor/🎚️config/🧬️schema/🧬️mutations", "JackEditorWindowConfigMutation"],
""", "", "ownership ts editor leaves")
edit(OWNERSHIP + "🧪️tests/🔬️window-config-ownership/🟦️.ts", """  const validateEditor = ajv.compile(editorSchema);
  const validateEditorMutation = ajv.compile(editorMutationSchema);
""", "", "ownership ts editor validators")
edit(OWNERSHIP + "🧪️tests/🔬️window-config-ownership/🟦️.ts", """  let editors = Object.fromEntries(query.pairs.map((pair: { editorWindowId: string }) => [pair.editorWindowId, structuredClone(query.editorBase)]));
""", "", "ownership ts editor state")
edit(OWNERSHIP + "🧪️tests/🔬️window-config-ownership/🟦️.ts", """  const editorGenerations: Record<string, number> = Object.fromEntries(Object.keys(editors).map((id) => [id, 0]));
""", "", "ownership ts editor generations")
edit(OWNERSHIP + "🧪️tests/🔬️window-config-ownership/🟦️.ts", """    assert(validateEditorMutation(pair.queryMutation), JSON.stringify(validateEditorMutation.errors));
""", "", "ownership ts editor mutation check")
edit(OWNERSHIP + "🧪️tests/🔬️window-config-ownership/🟦️.ts", """    editors = applyPatch(editors, [{ op: "replace", path: `/${pair.editorWindowId}/jackQuery`, value: pair.queryMutation.value }], true, false).newDocument;
""", "", "ownership ts editor patch")
edit(OWNERSHIP + "🧪️tests/🔬️window-config-ownership/🟦️.ts", """    editorGenerations[pair.editorWindowId] += 1;
""", "", "ownership ts editor generation step")
edit(OWNERSHIP + "🧪️tests/🔬️window-config-ownership/🟦️.ts", """  for (const state of Object.values(editors)) assert(validateEditor(state), JSON.stringify(validateEditor.errors));
""", "", "ownership ts editor validation")
edit(OWNERSHIP + "🧪️tests/🔬️window-config-ownership/🟦️.ts", """  assert.deepEqual(editorGenerations, query.expectedEditorGenerations);
""", "", "ownership ts editor generations check")
edit(OWNERSHIP + "🧪️tests/🔬️window-config-ownership/🟦️.ts", """  const reloadedEditors = structuredClone(editors);
""", "", "ownership ts editor reload")
edit(OWNERSHIP + "🧪️tests/🔬️window-config-ownership/🟦️.ts", """  assert.deepEqual(reloadedEditors, editors);
""", "", "ownership ts editor reload check")


OWNERSHIP_FIXTURE = OWNERSHIP + "🧫️fixtures/🔬️window-config-ownership/🔣️.json"
edit(OWNERSHIP_FIXTURE, """  "queryOwnership": {
    "appConfig": {},
    "editorBase": {
      "jackQuery": "MATCH (a:Piece) RETURN a.name"
    },
    "resultsBase\"""", """  "resultsOwnership": {
    "appConfig": {},
    "resultsBase\"""", "ownership fixture results only")
edit(OWNERSHIP_FIXTURE, """        "editorWindowId": "editor-left",
        "resultsWindowId": "results-left",
        "queryMutation": {
          "kind": "set-query",
          "value": "MATCH (a:Piece) WHERE a.name = 'b' RETURN a.name"
        },
""", """        "resultsWindowId": "results-left",
""", "ownership fixture left pair")
edit(OWNERSHIP_FIXTURE, """        "editorWindowId": "editor-right",
        "resultsWindowId": "results-right",
        "queryMutation": {
          "kind": "set-query",
          "value": "MATCH (a:Piece) WHERE a.name != 'b' RETURN a.name"
        },
""", """        "resultsWindowId": "results-right",
""", "ownership fixture right pair")
edit(OWNERSHIP_FIXTURE, """    "expectedEditorGenerations": {
      "editor-left": 1,
      "editor-right": 1
    },
""", "", "ownership fixture editor generations")

# ── jack editor laws that measured the window-config lane ──
UNIT = EDITOR + "🧪️tests/🔬️unit/🦀️.rs"
replace_every(UNIT, "    assert_eq!(scene.buffer, TRINITY_JACK_DEFAULT_QUERY);\n", "    assert_eq!(scene.buffer, crate::TRINITY_JACK_DEFAULT_QUERY);\n", "editor law default query")
edit(UNIT, "    for query in [crate::editor::jack::TRINITY_JACK_DEFAULT_QUERY, ", "    for query in [crate::TRINITY_JACK_DEFAULT_QUERY, ", "editor law shipped queries")
edit(UNIT, """async fn text_edit_updates_query_without_operations() {""", """async fn text_edit_puts_the_query_into_the_document() {""", "editor law text edit name")
edit(UNIT, """    assert_eq!(scene.buffer, "MATCH (a:Piece) RETURN a.name");
}
""", """    assert_eq!(scene.buffer, "MATCH (a:Piece) RETURN a.name");
    assert_eq!(app.snapshot().expect("projection").query, "MATCH (a:Piece) RETURN a.name", "the typed query is document content, not a window's config");
}
""", "editor law text edit document")
edit(UNIT, """async fn set_active_example_swaps_fixture_without_changing_editor_query() {""", """async fn set_active_example_loads_the_example_with_its_preset_query() {""", "editor law example name")
edit(UNIT, """    assert!(!result.requested_effects.is_empty() || !receipt.effects.is_empty(), "setActiveExample must request the LoadDocument effect");
    let node = app.render(TRINITY_JACK_PLAY_BODY_EDITOR, None, &query_windows().for_window_instance("editor-main").unwrap()).await.expect("render");
    let json = artifact_app_laws::project_and_retire_fixture_tree(node).expect("project semantic UI test tree");
    let scene = artifact_app_laws::decode_fixture_scene_with_lanes::<semio_framework_plugin::TextEditorScene>(&json).expect("text-editor scene");
    assert_eq!(scene.buffer, crate::TRINITY_JACK_DEFAULT_QUERY);
}""", """    let loaded = result.requested_effects.iter().chain(receipt.effects.iter()).find_map(|effect| match effect {
        semio_framework_plugin::Effect::LoadDocument { pack, .. } => Some(<crate::JackSnapshot as store::ArtifactPack>::decode_pack(pack).expect("the loaded example decodes")),
        _ => None,
    });
    let loaded = loaded.expect("setActiveExample must request the LoadDocument effect");
    assert_eq!(loaded.query, crate::editor::jack::commands::preset_query("branch-chain"), "the example document carries its own preset query");
    assert!(!loaded.nodes().is_empty(), "the example document carries its graph");
}""", "editor law example preset query")

EXAMPLE_LAWS = ANY + "📚️examples/🎬️demo/🧪️tests/🧩️example/🦀️.rs"
edit(EXAMPLE_LAWS, """#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = include_str!("../../../../🖼️assets/🎬️demo/🗣️.dsl.semio");
    assert!(text.len() > 8);
}
""", """#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = include_str!("../../../../🖼️assets/🎬️demo/🗣️.dsl.semio");
    assert!(text.len() > 8);
}

/// 🔁️ The committed example is this codec's own output — byte for byte, its document query included — the fixpoint
/// `🔌️mutate-jack-1`'s round trip and the query editor's first frame both read.
#[semio_framework_async_macros::async_test]
async fn primary_asset_is_the_codec_s_own_output() {
    let text = include_str!("../../../../🖼️assets/🎬️demo/🗣️.dsl.semio");
    let parsed = crate::standards::v1::subsets::any::schema::snapshot::text::parse_dsl(text).expect("example dsl parses");
    assert_eq!(parsed.query, crate::TRINITY_JACK_DEFAULT_QUERY, "the example opens on the default query");
    assert_eq!(crate::standards::v1::subsets::any::schema::snapshot::text::print_dsl(&parsed), text, "the committed example is the codec's own output");
}
""", "example law codec fixpoint")

edit(UNIT, """    let result = app
        .dispatch_typed(TrinityJackCommand::TextEdit { text: "MATCH (a:Piece) RETURN a.name".into() }, &semio_framework_plugin::ActionMeta { view_state: Some(editor.clone()), ..meta("local") })
        .await
        .expect("edit");
    assert!(result.mutations.is_empty());
""", """    app.dispatch_typed(TrinityJackCommand::TextEdit { text: "MATCH (a:Piece) RETURN a.name".into() }, &semio_framework_plugin::ActionMeta { view_state: Some(editor.clone()), ..meta("local") })
        .await
        .expect("edit");
""", "editor law text edit publishes on the document")
edit(UNIT, """keeps saving (every keystroke amends the run's ONE coalesced
/// window-config edit), and ONE undo""", """keeps saving (every keystroke amends the run's ONE coalesced
/// document edit), and ONE undo""", "editor law typing run document edit prose")
edit(UNIT, """        if drive_query_ownership_operations(&mut app).await? != (0, 1, 1) {
            return Err("query operation did not produce one editor-config and one results-transient receipt".into());""", """        if drive_query_ownership_operations(&mut app).await? != (0, 0, 1) {
            return Err("query operation did not produce exactly one results-transient receipt".into());""", "editor law run query receipts")

TWO_EDITORS_START = "#[semio_framework_async_macros::async_test]\nasync fn jack_graph_window_config_query_ownership_isolates_two_editor_result_pairs_and_reloads_only_authored_sources() {\n"
TWO_EDITORS_LAW = '''/// 🔎️ The Jack query is DOCUMENT content (ticket 26/09/23 C12, decision (a)): every editor window shows the one query the
/// document holds, an edit from either editor is one undoable document edit and persists with the document's pack, and running
/// a query — from either editor, into either results window — reads the document without editing it. Only the results stay
/// per window, and they are never persisted: a reopened app starts with fresh results windows.
#[semio_framework_async_macros::async_test]
async fn jack_query_is_document_content_shared_by_every_editor_while_results_stay_per_window() {
    let mut app = new_app().await;
    let view = ViewModel {
        window_instances: vec![
            semio_framework_plugin::ViewWindowInstance { id: "editor-left".into(), window_kind_id: TRINITY_JACK_PLAY_WINDOW_EDITOR.into() },
            semio_framework_plugin::ViewWindowInstance { id: "editor-right".into(), window_kind_id: TRINITY_JACK_PLAY_WINDOW_EDITOR.into() },
            semio_framework_plugin::ViewWindowInstance { id: "results-left".into(), window_kind_id: TRINITY_JACK_PLAY_WINDOW_RESULTS.into() },
            semio_framework_plugin::ViewWindowInstance { id: "results-right".into(), window_kind_id: TRINITY_JACK_PLAY_WINDOW_RESULTS.into() },
        ],
        ..Default::default()
    };
    let editor_left = view.for_window_instance("editor-left").unwrap();
    let editor_right = view.for_window_instance("editor-right").unwrap();
    let results_left = view.for_window_instance("results-left").unwrap();
    let results_right = view.for_window_instance("results-right").unwrap();
    let left_query = "MATCH (a:Piece) WHERE a.name = 'b' RETURN a.name";
    let right_query = "MATCH (a:Piece) WHERE a.name != 'b' RETURN a.name";
    let app_config_before = app.config_pack().await.expect("app config pack");
    let denied = app
        .dispatch_typed(
            TrinityJackCommand::RunQuery { query: Some(left_query.into()), results_window_id: "editor-right".into() },
            &semio_framework_plugin::ActionMeta { view_state: Some(editor_left.clone()), ..meta("query-owner") },
        )
        .await;
    assert!(denied.is_err(), "an attached editor cannot be promoted to results mutation authority by command payload");

    app.dispatch_typed(TrinityJackCommand::TextEdit { text: left_query.into() }, &semio_framework_plugin::ActionMeta { view_state: Some(editor_left.clone()), ..meta("query-owner") })
        .await
        .expect("query edit");
    assert_eq!(drive_query_ownership_operations(&mut app).await.expect("query edit"), (0, 0, 0));
    let document = app.snapshot().expect("document after the query edit");
    assert_eq!(document.query, left_query, "the typed query is the document's");
    for context in [&editor_left, &editor_right] {
        let tree = app.render(TRINITY_JACK_PLAY_BODY_EDITOR, None, context).await.expect("editor render");
        let rendered = artifact_app_laws::project_and_retire_fixture_tree(tree).expect("editor projection");
        let scene = artifact_app_laws::decode_fixture_scene_with_lanes::<semio_framework_plugin::TextEditorScene>(&rendered).expect("editor scene");
        assert_eq!(scene.buffer, left_query, "every editor window shows the document's one query");
    }

    for (context, target, query) in [(&editor_left, "results-left", None), (&editor_right, "results-right", Some(right_query.to_string()))] {
        app.dispatch_typed(TrinityJackCommand::RunQuery { query, results_window_id: target.into() }, &semio_framework_plugin::ActionMeta { view_state: Some(context.clone()), ..meta("query-owner") })
            .await
            .expect("paired query");
    }
    assert_eq!(drive_query_ownership_operations(&mut app).await.expect("paired queries"), (0, 0, 2));
    assert_eq!(app.snapshot().expect("document after queries"), document, "running a read query never edits the document");
    let app_config_after = app.config_pack().await.expect("app config after");
    assert_eq!(app_config_after.pack, app_config_before.pack);
    assert_eq!(app_config_after.spr, app_config_before.spr);
    assert!(app.window_config_packs().await.expect("persisted window configs").is_empty(), "no editor window persists a query of its own");
    assert_eq!(app.ephemeral_snapshot().await.transient_generation, 0);
    assert_eq!(app.window_transient_generation(&results_left).expect("left result generation"), Some(1));
    assert_eq!(app.window_transient_generation(&results_right).expect("right result generation"), Some(1));
    let left_snapshot = app.window_transient_snapshot(&results_left).expect("left result snapshot").expect("left result owner");
    let right_snapshot = app.window_transient_snapshot(&results_right).expect("right result snapshot").expect("right result owner");
    let left_state = left_snapshot.get::<JackResultsWindowTransientOwner>().expect("left result state");
    let right_state = right_snapshot.get::<JackResultsWindowTransientOwner>().expect("right result state");
    assert!(left_state.query_error.is_none() && right_state.query_error.is_none());
    assert_ne!(left_state.result, right_state.result, "concurrent executions must keep distinct result payloads");
    drop(left_snapshot);
    drop(right_snapshot);
    for context in [&results_left, &results_right] {
        let tree = app.render(TRINITY_JACK_PLAY_BODY_RESULTS, None, context).await.expect("result render");
        assert!(artifact_app_laws::project_and_retire_fixture_tree(tree).expect("result projection").contains("table"));
    }

    let reloaded = <crate::JackSnapshot as store::ArtifactPack>::decode_pack(&store::ArtifactPack::encode_pack(&document)).expect("the document pack decodes");
    assert_eq!(reloaded.query, left_query, "the query persists with the document");
    let admitted = app.handle_action("undo", None, &semio_framework_plugin::ActionMeta { view_state: Some(editor_right.clone()), ..meta("query-owner") }).await.unwrap_or_else(|fault| panic!("undo admission: {fault:?}"));
    semio_framework_plugin::app::settle_framework_reserved_admission(&mut app.app, admitted).await.unwrap_or_else(|fault| panic!("undo settles: {fault:?}"));
    drive_query_ownership_operations(&mut app).await.expect("undo publishes");
    assert_eq!(app.snapshot().expect("document after undo").query, crate::TRINITY_JACK_DEFAULT_QUERY, "one undo from either editor reverts the query edit");
    app.close();

    let mut reopened = new_app().await;
    for context in [&results_left, &results_right] {
        assert_eq!(reopened.window_transient_generation(context).expect("fresh result generation"), Some(0));
        let snapshot = reopened.window_transient_snapshot(context).expect("fresh result snapshot").expect("fresh results owner");
        let state = snapshot.get::<JackResultsWindowTransientOwner>().expect("fresh results state");
        assert!(state.query_execution_id.is_none() && state.result.is_none() && state.query_error.is_none());
    }
    reopened.close();
}
'''
unit_text = read(UNIT)
if unit_text is not None and TWO_EDITORS_START in unit_text:
    start = unit_text.index(TWO_EDITORS_START)
    end = unit_text.index("    reopened.close();\n}\n", start) + len("    reopened.close();\n}\n")
    files[UNIT] = unit_text[:start] + TWO_EDITORS_LAW + unit_text[end:]
    plan.append("edit    editor law two editors share the document query")
elif unit_text is not None and "fn jack_query_is_document_content_shared_by_every_editor_while_results_stay_per_window()" in unit_text:
    plan.append("present editor law two editors share the document query")
else:
    problems.append(("editor law two editors share the document query", "anchor"))

EDITOR_SCOPE_OLD = """            // 📇️ Per-window action scoping. Every retained WINDOW-CONFIG verb reads the config of the
            // window it was dispatched from, so an unowned one is a live fault: `build_definition`
            // copies an unowned action onto EVERY window kind, the Actions pane of the graph pane then
            // offered `formatDocument`, and the reducer refused it with "Jack query formatting requires
            // the exact editor-window config snapshot" (`:422`). Text verbs belong to the query editor,
            // viewport/LOD to the graph. Document verbs, selection, history and the example picker stay
            // unscoped on purpose — they read the document, not a pane.
"""
EDITOR_SCOPE_NEW = """            // 📇️ Per-window action scoping: `build_definition` copies an unowned action onto EVERY window
            // kind. Viewport/LOD read the config of the graph window they were dispatched from, so they
            // belong to the graph; the text verbs edit the document's query where it is typed, so they
            // belong to the query editor. Document verbs, selection, history and the example picker stay
            // unscoped on purpose — they read the document, not a pane.
"""
edit(EDITOR + "🦀️.rs", EDITOR_SCOPE_OLD, EDITOR_SCOPE_NEW, "editor action scoping prose")
edit(SCHEMA + "🧪️tests/🪪️document-contract/🟦️.ts", "  assert.equal(snapshotFixtures.length, 16);\n", "  assert.equal(snapshotFixtures.length, 18);\n", "document contract nine vector snapshot pairs")
edit(SCHEMA + "⚙️operations/🧪️tests/🔬️unit/🦀️.rs", "    assert_eq!(<TrinityGraphMutation as protocol::SemanticMutation<JackSnapshot>>::kinds().len(), 8);\n", "    assert_eq!(<TrinityGraphMutation as protocol::SemanticMutation<JackSnapshot>>::kinds().len(), 9);\n", "operations law nine semantic kinds")
