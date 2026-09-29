#!/usr/bin/env python3
"""💬️ S19 set `catalog-audits` (guest, T6 round 4+; coordinator 20:1x): two os-mcp `search::long` audit findings over the
installed catalog (`s14-s19-logs/mcp-search-long-1.txt`, G12's os-mcp test binary):
- gis viewer `setCamera` was published to agents with no description. A map camera is window view state the React
  `TiledMapHost` dispatches after every pan/zoom — chrome, exactly like the gis EDITOR's own `setCamera`
  (`action_audience(.., Chrome)`) and the law's "camera poses are chrome and never published". The viewer's verb is now
  Chrome audience, so it never reaches the agent catalog.
- raster `exportPng` writes a user path (download) with `effects.destructive = false` → `ApprovalMode::WhenDestructive`
  never asked a human. It is destructive now, like every other export.
Laws: gis viewer `the_viewer_camera_verb_is_chrome`, raster `export_png_asks_a_human_before_writing_a_user_path`.
usage: s19-catalog-audits.py [--dry-run|--write|--revert]"""
import importlib.util
import os
import sys

spec = importlib.util.spec_from_file_location("s19_setlib", os.path.join(os.path.dirname(os.path.abspath(__file__)), "s19_setlib.py"))
lib = importlib.util.module_from_spec(spec)
spec.loader.exec_module(lib)

VIEWER = "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer"
MAP = f"{VIEWER}/🎭️modes/👁️view/🪟️windows/🗺️map/🦀️.rs"
VIEWER_TESTS = f"{VIEWER}/🧪️tests/🔬️unit/🦀️.rs"
RASTER = "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"
RASTER_TESTS = "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs"

CAMERA_OLD = '''/// 🧭️ The one verb a read-only map window owns. `ActionKind::View` and a window-config publication: a
/// camera is never document data, so it publishes no document mutation — which is exactly why a
/// VIEWER may own it. It declares its `camera` argument so the host's `{surfaceId, camera:{x,y,zoom}}`
/// survives `effective_action_args`, which keeps ONLY declared arg ids once an action declares any.
pub fn set_camera_action() -> ActionDefinition {
    let mut action = ActionDefinition::bounded_catalog(SET_CAMERA_ACTION_ID, LocalizedLabel::native("Set camera", "Kamera setzen"), ActionKind::View)
        .with_args(vec![ActionArgDef::text("camera", LocalizedLabel::native("Map camera", "Kartenkamera")).required()]);
    action.semantics.execution.interactive_job = InteractiveJobClassification::Migrated;
    action
}
'''
CAMERA_NEW = '''/// 🧭️ The one verb a read-only map window owns. `ActionKind::View` and a window-config publication: a
/// camera is never document data, so it publishes no document mutation — which is exactly why a
/// VIEWER may own it. It declares its `camera` argument so the host's `{surfaceId, camera:{x,y,zoom}}`
/// survives `effective_action_args`, which keeps ONLY declared arg ids once an action declares any. The
/// pose is window chrome the host dispatches after each pan/zoom — never an agent capability, like the
/// editor's own `setCamera`.
pub fn set_camera_action() -> ActionDefinition {
    let mut action = ActionDefinition::bounded_catalog(SET_CAMERA_ACTION_ID, LocalizedLabel::native("Set camera", "Kamera setzen"), ActionKind::View)
        .with_args(vec![ActionArgDef::text("camera", LocalizedLabel::native("Map camera", "Kartenkamera")).required()]);
    action.semantics.execution.interactive_job = InteractiveJobClassification::Migrated;
    action.semantics.audience = Some(semio_framework_plugin::CapabilityAudience::Chrome);
    action
}
'''
VIEWER_LAW = '''
//#region 🧭️CameraAudience
/// 🧭️ LAW: a map camera is window view state — chrome, never an agent capability (os-mcp `search::long`: the viewer
/// published `setCamera` to agents with no description while the editor's own `setCamera` is already chrome).
#[test]
fn the_viewer_camera_verb_is_chrome() {
    let definition = create_gismap_viewer();
    let camera = definition.window_kinds.iter().flat_map(|window| window.actions.iter()).find(|action| action.id == map::SET_CAMERA_ACTION_ID).expect("the map window declares setCamera");
    assert_eq!(semio_framework::resolve_audience(camera), semio_framework_plugin::CapabilityAudience::Chrome);
}
//#endregion 🧭️CameraAudience
'''
EXPORT_OLD = '            .action_destructive("setActiveExample")\n            // 🧵️ Phase-8 dispositions.'
EXPORT_NEW = '            .action_destructive("setActiveExample")\n            .action_destructive("exportPng")\n            // 🧵️ Phase-8 dispositions.'
RASTER_LAW = '''
//#region 💾️ExportApproval
/// 💾️ LAW: `exportPng` writes a user path (a download), so it is destructive and `ApprovalMode::WhenDestructive` asks a
/// human before an agent commits it (os-mcp `search::long`: it was published with `effects.destructive = false`).
#[test]
fn export_png_asks_a_human_before_writing_a_user_path() {
    let definition = create_raster_app();
    let export = definition.actions.iter().chain(definition.window_kinds.iter().flat_map(|window| window.actions.iter())).find(|action| action.id == "exportPng").expect("raster declares exportPng");
    assert!(export.semantics.effects.destructive, "exportPng must be destructive");
}
//#endregion 💾️ExportApproval
'''


def append(marker, law):
    def transform(text):
        if marker in text:
            return text
        return text.rstrip("\n") + "\n" + law
    return transform


lib.run("catalog-audits", [
    (MAP, lib.replace_once(CAMERA_OLD, CAMERA_NEW)),
    (VIEWER_TESTS, append("fn the_viewer_camera_verb_is_chrome", VIEWER_LAW)),
    (RASTER, lib.replace_once(EXPORT_OLD, EXPORT_NEW)),
    (RASTER_TESTS, append("fn export_png_asks_a_human_before_writing_a_user_path", RASTER_LAW)),
], sys.argv[1:])
