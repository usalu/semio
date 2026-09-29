#!/usr/bin/env python3
"""🧹️ SH2 P2 stage edit (session 15): every registered Space surface in the space-space unit tests is a self-closing
`Registered<A>` (bound to live instance 1, retired through `close_registered_fixture_app` on drop — the store drop
witness), and the feed laws accept the settled lanes a transient feed publishes (its item + render refresh + terminal marker).
Idempotent: a replacement whose result is present is skipped; a missing anchor aborts before any write."""
import os, sys

STAGE = "/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-sh2-p2-stage/✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any"
TRANSIENT_TESTS = f"{STAGE}/👁️viewer/🫧️transient/🧪️tests/🔬️unit/🦀️.rs"
EDITOR_TESTS = f"{STAGE}/✏️editor/🧪️tests/🔬️unit/🦀️.rs"
VIEWER_TESTS = f"{STAGE}/👁️viewer/🧪️tests/🔬️unit/🦀️.rs"

REGISTERED = '''//#region 🧹️Registered
/// 🧹️ A registered Space surface bound to live instance `1` (the instance `artifact_app_laws::meta` stamps on every
/// dispatch) that retires through the bounded close protocol when it is dropped — every registry-backed app owns stores
/// that must close before drop (the store drop witness). A test that panics leaves the witness alone, so its first
/// failure stays the one reported.
pub(crate) struct Registered<A: semio_framework_plugin::PluginApp>(A);

impl<A: semio_framework_plugin::PluginApp> Registered<A> {
    /// 🧷️ Binds `app` to live instance `1` and takes over its close.
    pub(crate) async fn bound(mut app: A) -> Self {
        app.bind_instance_id(1).await;
        Self(app)
    }
}

impl<A: semio_framework_plugin::PluginApp> std::ops::Deref for Registered<A> {
    type Target = A;

    fn deref(&self) -> &A {
        &self.0
    }
}

impl<A: semio_framework_plugin::PluginApp> std::ops::DerefMut for Registered<A> {
    fn deref_mut(&mut self) -> &mut A {
        &mut self.0
    }
}

impl<A: semio_framework_plugin::PluginApp> Drop for Registered<A> {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut self.0);
        }
    }
}

/// 🫧️ Whether a settled Space feed published its transient item and nothing but that item's render refresh and terminal
/// marker — never a document, draft, config, window, presence or child lane.
pub(crate) fn publishes_only_transient_state(lanes: &[semio_framework_plugin::app::TypedOperationResultLane]) -> bool {
    use semio_framework_plugin::app::TypedOperationResultLane::{Terminal, Transient, Ui};
    lanes.iter().any(|lane| matches!(lane, Transient)) && lanes.iter().all(|lane| matches!(lane, Transient | Ui | Terminal))
}
//#endregion 🧹️Registered

'''

EDITS = [
    (TRANSIENT_TESTS, "//#region 🧫️Batches\n", REGISTERED + "//#region 🧫️Batches\n"),
    (EDITOR_TESTS, "    pub type SpaceIndexApp = semio_framework_plugin::VcsArtifactApp<EditorApp<SpaceIndexEditor>>;",
     "    pub type SpaceIndexApp = Registered<semio_framework_plugin::VcsArtifactApp<EditorApp<SpaceIndexEditor>>>;"),
    (EDITOR_TESTS, "    use semio_framework_plugin::artifact_app_laws::{meta, new_app_with_registry};\n",
     "    use semio_framework_plugin::artifact_app_laws::{meta, new_app_with_registry};\n    pub(crate) use crate::viewer::space_index::transient::tests::Registered;\n"),
    (EDITOR_TESTS, "    /// 🧪️ An app instance carrying the real `AppActionRegistry`.", "    /// 🧪️ A self-closing app instance carrying the real `AppActionRegistry`."),
    (EDITOR_TESTS, "anyway (`admit_command_wire_with_proof` refuses every verb with no manifest declaration). The app is\n    /// bound to live instance `1`, the instance `artifact_app_laws::meta` stamps on every dispatch — a typed\n    /// command addressed to any other instance is refused as not belonging to the mounted app.\n    pub async fn new_app() -> SpaceIndexApp {\n        use semio_framework_plugin::PluginApp;\n        let mut app = new_app_with_registry::<EditorApp<SpaceIndexEditor>>(space_index_manifest_for_tests).await;\n        app.bind_instance_id(1).await;\n        app\n    }",
     "anyway (`admit_command_wire_with_proof` refuses every verb with no manifest declaration). The app is\n    /// [`Registered`]: bound to live instance `1` — a typed command addressed to any other instance is refused as not\n    /// belonging to the mounted app — and closed when it is dropped.\n    pub async fn new_app() -> SpaceIndexApp {\n        Registered::bound(new_app_with_registry::<EditorApp<SpaceIndexEditor>>(space_index_manifest_for_tests).await).await\n    }"),
    (EDITOR_TESTS, "    pub async fn feed<A: semio_framework_plugin::PluginApp>(app: &mut A, after_seq_exclusive: u64,",
     "    pub async fn feed<A: semio_framework_plugin::PluginApp>(app: &mut Registered<A>, after_seq_exclusive: u64,"),
    (EDITOR_TESTS, "        semio_framework_plugin::artifact_app_laws::settle_registered_typed_operation(app, 1).await.expect(\"the directory batch settles\")",
     "        semio_framework_plugin::artifact_app_laws::settle_registered_typed_operation(&mut **app, 1).await.expect(\"the directory batch settles\")"),
    (EDITOR_TESTS, "        app.dispatch_typed(command, &meta(\"local\")).await?;\n        semio_framework_plugin::artifact_app_laws::settle_registered_typed_operation(app, 1).await",
     "        app.dispatch_typed(command, &meta(\"local\")).await?;\n        semio_framework_plugin::artifact_app_laws::settle_registered_typed_operation(&mut **app, 1).await"),
    (EDITOR_TESTS, "    pub async fn settle_action<A: semio_framework_plugin::PluginApp>(app: &mut A, action: &str,",
     "    pub async fn settle_action<A: semio_framework_plugin::PluginApp>(app: &mut Registered<A>, action: &str,"),
    (EDITOR_TESTS, "        app.handle_action(action, Some(&pack::json_to_dsl_value(&args)), &meta(\"local\")).await?;\n        semio_framework_plugin::artifact_app_laws::settle_registered_typed_operation(app, 1).await",
     "        app.handle_action(action, Some(&pack::json_to_dsl_value(&args)), &meta(\"local\")).await?;\n        semio_framework_plugin::artifact_app_laws::settle_registered_typed_operation(&mut **app, 1).await"),
    (EDITOR_TESTS, "async fn table<A: semio_framework_plugin::PluginApp>(app: &mut A, locale:",
     "async fn table<A: semio_framework_plugin::PluginApp>(app: &mut context::Registered<A>, locale:"),
    (EDITOR_TESTS, "    use semio_framework_plugin::app::TypedOperationResultLane;\n    use semio_framework_plugin::{Locale, PluginApp};\n    let (mut author, id) = context::new_app_with_indexed_artifact().await;\n    let mut spectator = semio_framework_plugin::VcsArtifactApp::<semio_framework_plugin::ViewerApp<crate::viewer::space_index::SpaceIndexViewer>>::with_registry(\n        Default::default(),\n        semio_framework_plugin::AppActionRegistry::from_definition(&crate::viewer::space_index::create_space_index_viewer()),\n    )\n    .await;\n    spectator.bind_instance_id(1).await;\n",
     "    use semio_framework_plugin::Locale;\n    let (mut author, id) = context::new_app_with_indexed_artifact().await;\n    let mut spectator = context::Registered::bound(\n        semio_framework_plugin::VcsArtifactApp::<semio_framework_plugin::ViewerApp<crate::viewer::space_index::SpaceIndexViewer>>::with_registry(\n            Default::default(),\n            semio_framework_plugin::AppActionRegistry::from_definition(&crate::viewer::space_index::create_space_index_viewer()),\n        )\n        .await,\n    )\n    .await;\n"),
    (EDITOR_TESTS, "        assert!(!settled.lanes.is_empty() && settled.lanes.iter().all(|lane| matches!(lane, TypedOperationResultLane::Transient)), \"the feed writes only the transient lane: {:?}\", settled.lanes);",
     "        assert!(crate::viewer::space_index::transient::tests::publishes_only_transient_state(&settled.lanes), \"the feed writes only the transient lane: {:?}\", settled.lanes);"),
    (EDITOR_TESTS, "    }\n    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut author);\n    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut spectator);\n}", "    }\n}"),
    (VIEWER_TESTS, "    use crate::viewer::space_index::transient::tests::{origin_of, SPACE};\n    use semio_framework_plugin::app::TypedOperationResultLane;\n    use semio_framework_plugin::PluginApp;\n    let mut app = semio_framework_plugin::VcsArtifactApp::<ViewerApp<SpaceIndexViewer>>::with_registry(Default::default(), semio_framework_plugin::AppActionRegistry::from_definition(&create_space_index_viewer())).await;\n    app.bind_instance_id(1).await;\n",
     "    use crate::viewer::space_index::transient::tests::{origin_of, publishes_only_transient_state, Registered, SPACE};\n    use semio_framework_plugin::PluginApp;\n    let mut app = Registered::bound(semio_framework_plugin::VcsArtifactApp::<ViewerApp<SpaceIndexViewer>>::with_registry(Default::default(), semio_framework_plugin::AppActionRegistry::from_definition(&create_space_index_viewer())).await).await;\n"),
    (VIEWER_TESTS, "        let settled = semio_framework_plugin::artifact_app_laws::settle_registered_typed_operation(&mut app, 1).await.expect(\"the feed settles\");\n        assert!(!settled.lanes.is_empty() && settled.lanes.iter().all(|lane| matches!(lane, TypedOperationResultLane::Transient)), \"{action} writes only the transient lane: {:?}\", settled.lanes);",
     "        let settled = semio_framework_plugin::artifact_app_laws::settle_registered_typed_operation(&mut *app, 1).await.expect(\"the feed settles\");\n        assert!(publishes_only_transient_state(&settled.lanes), \"{action} writes only the transient lane: {:?}\", settled.lanes);"),
    (VIEWER_TESTS, "    assert!(json.contains(\"user:u-2#s1\"), \"the viewer shows the row's presence: {json}\");\n    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut app);\n}",
     "    assert!(json.contains(\"user:u-2#s1\"), \"the viewer shows the row's presence: {json}\");\n}"),
]

texts = {path: open(path, encoding="utf-8").read() for path in {e[0] for e in EDITS}}
plan = []
for path, old, new in EDITS:
    text = texts[path]
    if new in text:
        plan.append(("applied", path, old[:60]))
        continue
    if text.count(old) != 1:
        sys.exit(f"anchor count {text.count(old)} in {os.path.basename(os.path.dirname(os.path.dirname(os.path.dirname(path))))}: {old[:90]!r}")
    texts[path] = text.replace(old, new)
    plan.append(("apply", path, old[:60]))
for state, path, head in plan:
    print(f"{state:>8}  {head!r}")
if "--write" in sys.argv:
    for path, text in texts.items():
        open(path, "w", encoding="utf-8").write(text)
    print("written")
