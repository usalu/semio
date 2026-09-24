from pathlib import Path
import json
import shutil
import textwrap

space = next(Path("/Users/ueli/Documents/semio").glob("*/🔌️plugins/🪐️space"))
osroot = next(Path("/Users/ueli/Documents/semio").glob("*/🛍️products/💻️os"))
ticket = next(Path("/Users/ueli/Documents/semio/.🧬semio").rglob("END-TO-END-OS-HUB-COLLABORATION-MCP"))
gen = ticket / "🗑️generated" / "wp-c6"
gen.mkdir(parents=True, exist_ok=True)

# 1) Fix icons Cloud + Save
main = next(p for p in space.rglob("🦀️.rs") if "explore" in str(p) and p.parent.name.endswith("main") and "tests" not in p.parts)
mt = main.read_text()
mt = mt.replace("IconName::CloudUpload", "IconName::Cloud").replace("IconName::HardDrive", "IconName::Save")
main.write_text(mt)
print("icons fixed")

# 2) Create promote-to-hub-space and persist-locally command modules
cmds = next(p for p in space.rglob("*") if p.is_dir() and p.name.endswith("commands") and "🏠️home" in str(p) and "create-studio" in [c.name for c in p.iterdir()])
# find by sibling create-studio
cmds = next(p.parent for p in space.rglob("*create-studio*") if p.is_dir())
print("cmds dir", cmds)

promote_dir = cmds / "☁️promote-to-hub-space"
persist_dir = cmds / "💾️persist-locally"
for d in (promote_dir, persist_dir):
    d.mkdir(parents=True, exist_ok=True)
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
//! Event-sourced persistence of an ephemeral local-only studio onto the local catalog
//! (folder/file backbone). Share/collaboration stay blocked; hub promotion is a separate command.

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

# 3) Wire modules in home artifact crate + editor
home_crate = next(p for p in space.rglob("*/🏠️home/🦀️.rs") if p.name.endswith(".rs"))
hc = home_crate.read_text()
if "promote_to_hub_space" not in hc:
    # find create_studio path include pattern
    needle = '#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🏗️create-studio/🦀️.rs"]\n            pub mod create_studio;'
    if needle in hc:
        hc = hc.replace(
            needle,
            needle
            + '\n            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/☁️promote-to-hub-space/🦀️.rs"]\n            pub mod promote_to_hub_space;'
            + '\n            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/💾️persist-locally/🦀️.rs"]\n            pub mod persist_locally;',
            1,
        )
        home_crate.write_text(hc)
        print("home crate mods wired")
    else:
        print("WARN home crate needle missing")
        (gen / "home-crate-snip.txt").write_text(hc[hc.find("create_studio")-200:hc.find("create_studio")+400] if "create_studio" in hc else "no create_studio")

editor = next(p for p in space.rglob("*/🏠️home/**/✏️editor/🦀️.rs") if "tests" not in p.parts and p.parent.name.endswith("editor") or False)
editors = [p for p in space.rglob("🦀️.rs") if str(p).endswith("✳️any/✏️editor/🦀️.rs") and "🏠️home" in str(p)]
print("editors", editors)
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
    editor.write_text(et)
    print("editor commands wired")
else:
    print("editor already wired")

# 4) share-space blocks ephemeral
share = next(p for p in space.rglob("*/🔗️share-space/🦀️.rs") if "tests" not in p.parts)
st = share.read_text()
if "ephemeralLocalOnly" not in st:
    old = '''pub fn handle(payload: &ShareSpace, _doc: &ArtifactView<'_, SHomeSnapshot>, _cfg: &ConfigView<'_, HomeConfig>) -> Result<Emit<SHomeMutation, HomeConfigMutation>, Fault> {
    if payload.email.trim().is_empty() {
        let args = Some(pack::json_to_dsl_value(&pack::json!({ "spaceId": payload.space_id.clone() })));
        return Ok(Emit::effect(Effect::OpenDialog { req: semio_framework_plugin::RequestId(126), dialog_id: "shareSpace".into(), args }));
    }'''
    new = '''pub fn handle(payload: &ShareSpace, _doc: &ArtifactView<'_, SHomeSnapshot>, cfg: &ConfigView<'_, HomeConfig>) -> Result<Emit<SHomeMutation, HomeConfigMutation>, Fault> {
    let directory = cfg.snapshot.directory().unwrap_or_default();
    let is_hub = directory.spaces.contains_key(&payload.space_id);
    if !is_hub {
        let args = Some(pack::json_to_dsl_value(&pack::json!({
            "spaceId": payload.space_id.clone(),
            "dataClass": "ephemeralLocalOnly",
            "reason": "ephemeral-local-only"
        })));
        return Ok(Emit::effect(Effect::OpenDialog { req: semio_framework_plugin::RequestId(128), dialog_id: "ephemeralShareBlocked".into(), args }));
    }
    if payload.email.trim().is_empty() {
        let args = Some(pack::json_to_dsl_value(&pack::json!({ "spaceId": payload.space_id.clone() })));
        return Ok(Emit::effect(Effect::OpenDialog { req: semio_framework_plugin::RequestId(126), dialog_id: "shareSpace".into(), args }));
    }'''
    if old not in st:
        print("WARN share handle needle missing")
        (gen / "share-snip.txt").write_text(st)
    else:
        share.write_text(st.replace(old, new, 1))
        print("share-space gated")
else:
    print("share already gated")

# 5) Native live foldDirectoryEvents
wgpu = next((osroot / "🔨️modules" / "📺️renderer").rglob("*/🐚️Shell/🎯️targets/*/🦀️.rs"))
wt = wgpu.read_text()
if "fold_live_directory_events" not in wt:
    old_match = '''                    DirectoryStreamMessage::Event { .. } | DirectoryStreamMessage::Heartbeat { .. } => dirty = true,'''
    new_match = '''                    DirectoryStreamMessage::Event { event } => {
                        live_events.push((*event).clone());
                        dirty = true;
                    }
                    DirectoryStreamMessage::Heartbeat { .. } => dirty = true,'''
    if old_match not in wt:
        print("WARN stream match missing")
    else:
        # add live_events vec before loop
        old_loop = '''            let mut dirty = false;
            let mut rebootstrap = false;
            for message in runner.drain() {
                match message {
                    DirectoryStreamMessage::Event { event } => {
                        live_events.push((*event).clone());
                        dirty = true;
                    }'''
        # first replace match arm
        wt = wt.replace(old_match, new_match, 1)
        wt = wt.replace(
            '''            let mut dirty = false;
            let mut rebootstrap = false;
            for message in runner.drain() {''',
            '''            let mut dirty = false;
            let mut rebootstrap = false;
            let mut live_events: Vec<store::os_directory::DirectoryEvent> = Vec::new();
            for message in runner.drain() {''',
            1,
        )
        # after the for loop / dirty handling, dispatch fold
        anchor = '''            if dirty || rebootstrap {
                if let Some(home) = self.directory_home.as_mut() {
                    match home.wake(rebootstrap) {
                        Ok(Some(_)) => changed = true,
                        Ok(None) => {}
                        Err(error) => {
                            home.close();
                            self.error = Some(error.to_string());
                        }
                    }
                }
            }'''
        replacement = '''            if !live_events.is_empty() {
                self.dispatch_fold_directory_events(&live_events);
                changed = true;
            }
            if dirty || rebootstrap {
                if let Some(home) = self.directory_home.as_mut() {
                    match home.wake(rebootstrap) {
                        Ok(Some(_)) => changed = true,
                        Ok(None) => {}
                        Err(error) => {
                            home.close();
                            self.error = Some(error.to_string());
                        }
                    }
                }
            }'''
        if anchor not in wt:
            print("WARN dirty anchor missing after match patch")
        else:
            wt = wt.replace(anchor, replacement, 1)
        # add helper method near start_directory_home_publication
        helper = '''
    /// 📔️ Live directory WS events fold through the same `foldDirectoryEvents` command the browser shell uses,
    /// so hub membership stays consistent without waiting for a full event-page rebootstrap.
    #[cfg(not(target_arch = "wasm32"))]
    fn dispatch_fold_directory_events(&mut self, events: &[store::os_directory::DirectoryEvent]) {
        let Some(session) = self.session.clone() else { return };
        let Some(program) = self.plugins.iter().find(|entry| entry.plugin_id == session.plugin_id).cloned() else { return };
        let events_json = pack::to_json_string(events);
        let live_view_state = self.live_view_state(&session);
        let window_kind_id = session.app.window_kinds.first().map(|kind| kind.id.clone()).unwrap_or_default();
        let window_instance_id = live_view_state.window_id.clone().unwrap_or_else(|| window_kind_id.clone());
        let mode_id = live_view_state.active_mode_id.clone().unwrap_or_else(|| session.app.default_mode_id.clone());
        let invocation = semio_framework::manifest::ActionInvocation {
            address: semio_framework::manifest::ActionAddress {
                plugin_id: session.plugin_id.clone(),
                app_id: session.app.id.clone(),
                mode_id,
                window_kind_id,
                window_instance_id,
                action_id: "foldDirectoryEvents".into(),
            },
            arguments: BTreeMap::from([("eventsJson".into(), DslValue::String(events_json))]),
        };
        let action_json = dsl::os_pack::json::to_json_string(&invocation);
        let pool = crate::renderer_worker_pool();
        let instance_id = session.instance_id;
        let _ = ShellPoolFuture::spawn(pool, Lane::Io, async move {
            let _ = program.handle_action(instance_id, &action_json, &live_view_state).await;
        });
    }
'''
        mark = '    /// 🏠️ Begins one bounded canonical-page fetch owned by the retained Home bootstrap epoch.'
        if mark in wt and "dispatch_fold_directory_events" not in wt:
            wt = wt.replace(mark, helper + "\n" + mark, 1)
        wgpu.write_text(wt)
        print("native foldDirectoryEvents wired")
else:
    print("native fold already present")

print("finish script phase1 done")
