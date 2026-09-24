from pathlib import Path

space = next(Path("/Users/ueli/Documents/semio").glob("*/🔌️plugins/🪐️space"))
osroot = next(Path("/Users/ueli/Documents/semio").glob("*/🛍️products/💻️os"))
ticket = next(Path("/Users/ueli/Documents/semio/.🧬semio").rglob("END-TO-END-OS-HUB-COLLABORATION-MCP"))
gen = ticket / "🗑️generated" / "wp-c6"
gen.mkdir(parents=True, exist_ok=True)

cmds = next(p.parent for p in space.rglob("*") if p.is_dir() and "create-studio" in p.name and "🏠️home" in str(p))
print("cmds", cmds)

promote_dir = cmds / "☁️promote-to-hub-space"
persist_dir = cmds / "💾️persist-locally"
for d in (promote_dir, persist_dir):
    (d / "🧪️tests" / "🔬️unit").mkdir(parents=True, exist_ok=True)

(promote_dir / "🦀️.rs").write_text('''//! ☁️ S Home launcher app command — `promote-to-hub-space`.
//! Event-sourced promotion of an ephemeral local-only studio onto the hub directory
//! (`os.directory.create-space`). Share/collaboration stay blocked until the hub fold lands.

use crate::standards::v1::subsets::any::schema::mutations::text::SHomeMutation;
use crate::SHomeSnapshot;
use crate::editor::home::config::{HomeConfig, HomeConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, Emit, Fault};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[dsl(keyword = "promote-to-hub-space")]
pub struct PromoteToHubSpace {
    pub space_id: String,
    pub name: String,
}

pub fn handle(payload: &PromoteToHubSpace, _doc: &ArtifactView<'_, SHomeSnapshot>, _cfg: &ConfigView<'_, HomeConfig>) -> Result<Emit<SHomeMutation, HomeConfigMutation>, Fault> {
    let name = if payload.name.trim().is_empty() { payload.space_id.clone() } else { payload.name.clone() };
    let args = Some(pack::json_to_dsl_value(&pack::json!({ "name": name, "spaceKind": "atelier", "visibility": "private", "sourceSpaceId": payload.space_id.clone() })));
    Ok(Emit::effect(Effect::ReplayShellCommand { action_id: "os.directory.create-space".into(), args }))
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
''')
(promote_dir / "🧪️tests" / "🔬️unit" / "🦀️.rs").write_text('''use super::*;
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, HistoryView};

#[test]
fn promote_relays_create_space_with_source_id() {
    let projection = SHomeSnapshot { schema: "s.home".into(), catalog_generation: 0 };
    let history = HistoryView::empty();
    let doc = ArtifactView::new(&projection, &history);
    let config = HomeConfig::default();
    let cfg = ConfigView { snapshot: &config, window: None };
    let emit = handle(&PromoteToHubSpace { space_id: "draft-1".into(), name: "Promoted".into() }, &doc, &cfg).expect("handle");
    assert!(emit.effects.iter().any(|effect| matches!(effect, Effect::ReplayShellCommand { action_id, .. } if action_id == "os.directory.create-space")));
}
''')
(persist_dir / "🦀️.rs").write_text('''//! 💾 S Home launcher app command — `persist-locally`.
//! Event-sourced persistence of an ephemeral local-only studio onto the local catalog.

use crate::standards::v1::subsets::any::schema::mutations::change_catalog_generation;
use crate::standards::v1::subsets::any::schema::mutations::text::SHomeMutation;
use crate::SHomeSnapshot;
use crate::editor::home::config::{HomeConfig, HomeConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, Emit, Fault};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[dsl(keyword = "persist-locally")]
pub struct PersistLocally {
    pub space_id: String,
    pub folder_path: Option<String>,
}

pub fn handle(payload: &PersistLocally, doc: &ArtifactView<'_, SHomeSnapshot>, _cfg: &ConfigView<'_, HomeConfig>) -> Result<Emit<SHomeMutation, HomeConfigMutation>, Fault> {
    if payload.folder_path.as_ref().map(|path| path.trim().is_empty()).unwrap_or(true) {
        let args = Some(pack::json_to_dsl_value(&pack::json!({ "spaceId": payload.space_id.clone() })));
        return Ok(Emit::effect(Effect::OpenDialog { req: semio_framework_plugin::RequestId(127), dialog_id: "persistLocally".into(), args }));
    }
    let folder_path = payload.folder_path.clone().unwrap_or_default();
    let args = Some(pack::json_to_dsl_value(&pack::json!({ "spaceId": payload.space_id.clone(), "folderPath": folder_path })));
    Ok(Emit {
        artifact_mutations: vec![change_catalog_generation(doc.snapshot.catalog_generation + 1)],
        effects: vec![Effect::ReplayShellCommand { action_id: "os.space.persist-locally".into(), args }],
        ..Default::default()
    })
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
''')
(persist_dir / "🧪️tests" / "🔬️unit" / "🦀️.rs").write_text('''use super::*;
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, HistoryView};

#[test]
fn empty_folder_opens_persist_dialog() {
    let projection = SHomeSnapshot { schema: "s.home".into(), catalog_generation: 3 };
    let history = HistoryView::empty();
    let doc = ArtifactView::new(&projection, &history);
    let config = HomeConfig::default();
    let cfg = ConfigView { snapshot: &config, window: None };
    let emit = handle(&PersistLocally { space_id: "draft-1".into(), folder_path: None }, &doc, &cfg).expect("handle");
    assert!(emit.effects.iter().any(|effect| matches!(effect, Effect::OpenDialog { dialog_id, .. } if dialog_id == "persistLocally")));
}
''')
print("commands written")

# Wire home crate
home_crate = next(p for p in space.rglob("🦀️.rs") if p.parent.name.endswith("home") and "🗿️artifacts" in str(p))
hc = home_crate.read_text()
if "promote_to_hub_space" not in hc:
    # find create_studio path line
    idx = hc.find("create-studio/🦀️.rs")
    if idx < 0:
        raise SystemExit("create-studio path missing in home crate")
    # insert after create_studio mod block
    marker = 'pub mod create_studio;'
    if marker not in hc:
        raise SystemExit("pub mod create_studio missing")
    hc = hc.replace(
        marker,
        marker
        + '\n            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/☁️promote-to-hub-space/🦀️.rs"]\n            pub mod promote_to_hub_space;'
        + '\n            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/💾️persist-locally/🦀️.rs"]\n            pub mod persist_locally;',
        1,
    )
    home_crate.write_text(hc)
    print("home crate wired", home_crate)
else:
    print("home crate already wired")

editors = [p for p in space.rglob("🦀️.rs") if str(p).endswith("✳️any/✏️editor/🦀️.rs") and "🏠️home" in str(p)]
editor = editors[0]
et = editor.read_text()
if "promote_to_hub_space" not in et:
    et = et.replace(
        "use crate::editor::home::commands::{bind_space_file, create_studio, import_space, open_space};",
        "use crate::editor::home::commands::{bind_space_file, create_studio, import_space, open_space, persist_locally, promote_to_hub_space};",
    )
    et = et.replace(
        '"createStudio" as "create-studio" => create_studio::CreateStudio,',
        '"createStudio" as "create-studio" => create_studio::CreateStudio,\n'
        '        "promoteToHubSpace" as "promote-to-hub-space" => promote_to_hub_space::PromoteToHubSpace,\n'
        '        "persistLocally" as "persist-locally" => persist_locally::PersistLocally,',
    )
    # match arms for retained extent may need wildcards - check later
    editor.write_text(et)
    print("editor wired")
else:
    print("editor already wired")

# share gate - simpler approach without directory().unwrap_or_default if that doesn't exist
share = next(p for p in space.rglob("*/🔗️share-space/🦀️.rs") if "tests" not in p.parts)
st = share.read_text()
if "ephemeralShareBlocked" not in st:
    # Use resolve_studio_document via resolve_ready to detect ephemeral (backbone none / draft)
    old = "pub fn handle(payload: &ShareSpace, _doc: &ArtifactView<'_, SHomeSnapshot>, _cfg: &ConfigView<'_, HomeConfig>) -> Result<Emit<SHomeMutation, HomeConfigMutation>, Fault> {\n    if payload.email.trim().is_empty() {"
    new = '''pub fn handle(payload: &ShareSpace, _doc: &ArtifactView<'_, SHomeSnapshot>, cfg: &ConfigView<'_, HomeConfig>) -> Result<Emit<SHomeMutation, HomeConfigMutation>, Fault> {
    let directory = cfg.snapshot.directory().ok();
    let is_hub = directory.as_ref().is_some_and(|model| model.spaces.contains_key(&payload.space_id));
    if !is_hub {
        let args = Some(pack::json_to_dsl_value(&pack::json!({
            "spaceId": payload.space_id.clone(),
            "dataClass": "ephemeralLocalOnly",
            "reason": "ephemeral-local-only"
        })));
        return Ok(Emit::effect(Effect::OpenDialog { req: semio_framework_plugin::RequestId(128), dialog_id: "ephemeralShareBlocked".into(), args }));
    }
    if payload.email.trim().is_empty() {'''
    if old not in st:
        (gen / "share-snip.txt").write_text(st)
        raise SystemExit("share needle missing")
    share.write_text(st.replace(old, new, 1))
    print("share gated")
else:
    print("share already gated")

print("phase b done")
