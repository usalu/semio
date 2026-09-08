
//! 🧪️ `testkit::assert_declared_actions_bridge_to_commands`'s signature is still
//! `fn(manifest: fn() -> App)` (framework testkit gap, `📓️w0-f-report.md` Gap 3) — `App { definition,
//! examples }` shape kept alive here purely to satisfy that call.
use super::create_playground_editor;
use semio_framework_plugin::App;

pub fn playground_editor_manifest_for_testkit() -> App {
    App { definition: create_playground_editor(), examples: Vec::new() }
}
