"""🗑️ S20 window-3 prepared patch: retire the note and shooting `loadRequest` whole-document pickers (coordinator decision
28 13:5x) — the framework's Import Document is the one path that opens a document from a file.

Both pickers accepted `.dsl/.spk/.ops` yet dispatched `setFixtureJson` / `importSnapshotJson`, which decode a `json` argument
the host's file-open envelope (`{payload, name, chunk, chunkCount}`) never carries: neither ever read its own export
(io-matrix 27 19:5x `missing field json`). Removed in one set: the command modules and enum rows, decoders, retained tool ids,
publication and execution contracts, manifest rows (label, description, use-when, interactive-job class), their unit tests,
the note action-cohort fixture, the shooting retained-command-limits fixture + schema (route count 38 → 37), and the two
io-matrix import pins. `setFixtureJson` / `importSnapshotJson` stay as argument verbs (agents, tests).

Usage: python3 s20-patch-retire-load-request.py [--dry-run]   (idempotent)
"""
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
DRY = "--dry-run" in sys.argv
NOTE = "✏️s/🔌️plugins/🗒️note"
NOTE_ROOT = f"{NOTE}/🗿️artifacts/🗒️note/🦀️.rs"
NOTE_EDITOR = f"{NOTE}/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor"
SHOOT = "✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any"
SHOOT_EDITOR = f"{SHOOT}/✏️editor"
IO_MATRIX = "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧫️fixtures/🚪️io-matrix.json"
NOTE_LOAD_REQUEST = f"{NOTE_EDITOR}/🎮️commands/📥️load-request/🦀️.rs"

SHOOTING_DOCUMENT_REGION = """//#region 🔖️LoadRequest
pub mod load_request {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "load-request")]
    pub struct LoadRequest {}

    pub fn handle(_payload: &LoadRequest, _doc: &ArtifactView<'_, ShootingSnapshot>, _cfg: &ConfigView<'_, ShootingConfig>, _ctx: &mut ShootingDispatchCtx) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        Ok(Emit::effect(Effect::RequestFileOpen { req: semio_framework_plugin::RequestId(109), accept: ".ops,.dsl,.spk,application/octet-stream,text/plain".into(), read_as: None, import_action: "importSnapshotJson".into(), multiple: false }))
    }
}
//#endregion 🔖️LoadRequest

"""

SHOOTING_DOCUMENT_TEST = """
#[semio_framework_async_macros::async_test]
async fn load_request_declares_the_import_snapshot_json_import_action() {
    use semio_framework_plugin::Effect;
    let mut app = shooting_app().await;
    let result = dispatch(&mut app, ShootingCommand::LoadRequest(load_request::LoadRequest {})).await;
    match &result.requested_effects[0] {
        Effect::RequestFileOpen { import_action, .. } => assert_eq!(import_action, "importSnapshotJson"),
        other => panic!("expected RequestFileOpen, got {other:?}"),
    }
}
"""

HUNKS: list[tuple[str, str, str]] = [
    (NOTE_ROOT, '            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📥️load-request/🦀️.rs"]\n            pub mod load_request;\n', ""),
    (f"{NOTE_EDITOR}/🧵️retained/🦀️.rs", '    "navigatorEngagementInput",\n    "saveDownload",\n    "loadRequest",\n];\n', '    "navigatorEngagementInput",\n    "saveDownload",\n];\n'),
    (f"{NOTE_EDITOR}/🧵️retained/🦀️.rs", '    ArtifactToolPublicationContract { tool_id: "loadRequest", lanes: &[ArtifactToolPublicationLane::HostOnly] },\n', ""),
    (f"{NOTE_EDITOR}/🧪️tests/🔬️unit/🦀️.rs", "        NoteCommand::LoadRequest(load_request::LoadRequest {}),\n", ""),
    (f"{NOTE_EDITOR}/🧪️tests/🔬️unit/🦀️.rs", 'assert_eq!(ids.len(), 34, "every NoteCommand row must be covered by every_command()");', 'assert_eq!(ids.len(), 33, "every NoteCommand row must be covered by every_command()");'),
    (f"{NOTE_EDITOR}/🦀️.rs", "use crate::editor::note::commands::{load_request, save_download};\n", "use crate::editor::note::commands::save_download;\n"),
    (f"{NOTE_EDITOR}/🦀️.rs", '        "loadRequest" as "load-request" => load_request::LoadRequest,\n', ""),
    (f"{NOTE_EDITOR}/🦀️.rs", '            "loadRequest" => NoteCommand::LoadRequest(decode(action, empty())?),\n', ""),
    (f"{NOTE_EDITOR}/🦀️.rs", '            "loadRequest" => semio_framework::ToolExecutionContract::resumable(65_536, 4_096, 1, 262_144, 7_500, 1, 1),\n', ""),
    (f"{NOTE_EDITOR}/🦀️.rs",
     '            // 🐚️ Import/export footer actions → panel Shell actions emitting host effects (S).\n            .shell_action("loadRequest", LocalizedLabel::native("Import", "Importieren"))\n',
     '            // 🐚️ Export footer action → panel Shell action emitting a host effect (S); opening a file is the framework Import Document.\n'),
    (f"{NOTE_EDITOR}/🦀️.rs", '            .action_describe("loadRequest", LocalizedLabel::native("Asks the host to open a file and import it into this note.", "Fordert den Host auf, eine Datei zu öffnen und in diese Notiz zu importieren."))\n            .action_use_when("loadRequest", vec!["import a document".into(), "open a file into the note".into()])\n', ""),
    (f"{NOTE_EDITOR}/🦀️.rs", '            .action_interactive_job("loadRequest", semio_framework_plugin::InteractiveJobClassification::Migrated)\n', ""),
    (f"{NOTE_EDITOR}/🎮️commands/💾️save-download/🧪️tests/🔬️unit/🦀️.rs", "use crate::editor::note::commands::load_request;\n", ""),
    (f"{NOTE_EDITOR}/🎮️commands/💾️save-download/🧪️tests/🔬️unit/🦀️.rs", "async fn save_download_and_load_request_effects() {", "async fn save_download_requests_a_media_export() {"),
    (f"{NOTE_EDITOR}/🎮️commands/💾️save-download/🧪️tests/🔬️unit/🦀️.rs",
     '\n    let load = dispatch(&mut app, NoteCommand::LoadRequest(load_request::LoadRequest {})).await;\n    assert!(matches!(load.requested_effects.first(), Some(Effect::RequestFileOpen { import_action, .. }) if import_action == "setFixtureJson"), "loadRequest must request a file open: {:?}", load.requested_effects);\n',
     "\n"),
    (f"{NOTE}/🧫️fixtures/🧪️action-cohort/🔣️.json", '  "routeCount": 35,\n', '  "routeCount": 33,\n'),
    (f"{NOTE}/🧫️fixtures/🧪️action-cohort/🔣️.json", '    "navigatorEngagementInput",\n    "loadRequest"\n  ],\n', '    "navigatorEngagementInput"\n  ],\n'),
    (f"{NOTE}/🧫️fixtures/🧪️action-cohort/🔣️.json", '        "navigatorEngagementInput",\n        "loadRequest"\n      ]\n', '        "navigatorEngagementInput"\n      ]\n'),
    (f"{SHOOT_EDITOR}/🦀️.rs", '        "loadRequest" as "load-request" => load_request::LoadRequest,\n', ""),
    (f"{SHOOT_EDITOR}/🦀️.rs", "use document::{import_snapshot_json, load_request, reset_snapshot, save_download, set_active_example};\n", "use document::{import_snapshot_json, reset_snapshot, save_download, set_active_example};\n"),
    (f"{SHOOT_EDITOR}/🦀️.rs", '            "loadRequest" => ShootingCommand::LoadRequest(decode(action, plain())?),\n', ""),
    (f"{SHOOT_EDITOR}/🦀️.rs", '    "saveDownload",\n    "loadRequest",\n    "importAssetRequest",\n', '    "saveDownload",\n    "importAssetRequest",\n'),
    (f"{SHOOT_EDITOR}/🦀️.rs", '        ArtifactToolPublicationContract { tool_id: "loadRequest", lanes: &[ArtifactToolPublicationLane::HostOnly] },\n', ""),
    (f"{SHOOT_EDITOR}/🦀️.rs", '            "loadRequest" => ToolExecutionContract::bounded_first_step(65_536, 64, 1, 262_144, 7_500),\n', ""),
    (f"{SHOOT_EDITOR}/🦀️.rs", '            .shell_action("loadRequest", LocalizedLabel::native("Load Request", "Ladeanfrage"))\n', ""),
    (f"{SHOOT_EDITOR}/🦀️.rs", '            .action_interactive_job("loadRequest", InteractiveJobClassification::Migrated)\n', ""),
    (f"{SHOOT_EDITOR}/🦀️.rs", '            .action_describe("loadRequest", LocalizedLabel::native("Opens the host\'s file picker for a saved shooting document (.ops, .dsl or .spk); the chosen file then replaces the current document.", "Öffnet die Dateiauswahl des Hosts für ein gespeichertes Shooting-Dokument (.ops, .dsl oder .spk); die gewählte Datei ersetzt dann das aktuelle Dokument."))\n', ""),
    (f"{SHOOT_EDITOR}/🎮️commands/📄️document/🦀️.rs", SHOOTING_DOCUMENT_REGION, ""),
    (f"{SHOOT_EDITOR}/🎮️commands/📄️document/🧪️tests/🔬️unit/🦀️.rs", SHOOTING_DOCUMENT_TEST, ""),
    (f"{SHOOT_EDITOR}/🧪️tests/🔬️unit/🦀️.rs", '"saveDownload", "loadRequest", "importAssetRequest", "exportActiveShot", "exportAllShots"]', '"saveDownload", "importAssetRequest", "exportActiveShot", "exportAllShots"]'),
    (f"{SHOOT_EDITOR}/🧪️tests/🔬️unit/🦀️.rs", 'let wrong_tool = fixture.replacen("\\"loadRequest\\"", "\\"forgedRequest\\"", 1);', 'let wrong_tool = fixture.replacen("\\"importAssetRequest\\"", "\\"forgedRequest\\"", 1);'),
    (f"{SHOOT_EDITOR}/🧪️tests/🔬️unit/🦀️.rs", "        ShootingCommand::LoadRequest(load_request::LoadRequest {}),\n", ""),
    (f"{SHOOT_EDITOR}/🧪️tests/🔬️unit/🦀️.rs", 'for command in ["loadRequest", "importAssetRequest", "saveDownload",', 'for command in ["importAssetRequest", "saveDownload",'),
    (f"{SHOOT_EDITOR}/🧪️tests/🔬️unit/🦀️.rs",
     """    let result = dispatch(&mut app, ShootingCommand::LoadRequest(load_request::LoadRequest {})).await;
    match &result.requested_effects[0] {
        Effect::RequestFileOpen { import_action, .. } => assert_eq!(import_action, "importSnapshotJson"),
        other => panic!("expected RequestFileOpen, got {other:?}"),
    }
    let result = dispatch(&mut app, ShootingCommand::SaveDownload(save_download::SaveDownload {})).await;""",
     "    let result = dispatch(&mut app, ShootingCommand::SaveDownload(save_download::SaveDownload {})).await;"),
    (f"{SHOOT_EDITOR}/🧪️tests/🔬️unit/🦀️.rs", 'assert_eq!(ids.len(), 37, "every ShootingCommand row must be covered by every_command()");', 'assert_eq!(ids.len(), 36, "every ShootingCommand row must be covered by every_command()");'),
    (f"{SHOOT_EDITOR}/🧪️tests/🔬️unit/🦀️.rs", "ShootingRetainedCatalogSummary { routes: 38, bounded: 37, resumable: 1, migrated: 37, fail_closed: 1, unique: true,", "ShootingRetainedCatalogSummary { routes: 37, bounded: 36, resumable: 1, migrated: 36, fail_closed: 1, unique: true,"),
    (f"{SHOOT_EDITOR}/🧫️fixtures/🧫️retained-command-limits/🔣️.json", '    {\n      "toolId": "loadRequest",\n      "lanes": [\n        "HostOnly"\n      ]\n    },\n', ""),
    (f"{SHOOT_EDITOR}/🧫️fixtures/🧫️retained-command-limits/🔣️.json", '    {\n      "id": "loadRequest",\n      "execution": "bounded",\n      "admission": "Migrated",\n      "feature": "emit exactly one fixed host file-open request"\n    },\n', ""),
    (f"{SHOOT}/🧬️schema/🔣️.json", '    "ShootingLoadRequestPublication": {\n      "type": "object",\n      "required": [\n        "toolId",\n        "lanes"\n      ],\n      "properties": {\n        "toolId": {\n          "const": "loadRequest"\n        },\n        "lanes": {\n          "const": [\n            "HostOnly"\n          ]\n        }\n      },\n      "additionalProperties": false\n    },\n', ""),
    (f"{SHOOT}/🧬️schema/🔣️.json", '          "items": [\n            {\n              "$ref": "#/$defs/ShootingLoadRequestPublication"\n            },\n            {\n              "$ref": "#/$defs/ShootingImportAssetRequestPublication"\n            }\n          ],\n          "additionalItems": false,\n          "minItems": 2,\n          "maxItems": 2\n', '          "items": [\n            {\n              "$ref": "#/$defs/ShootingImportAssetRequestPublication"\n            }\n          ],\n          "additionalItems": false,\n          "minItems": 1,\n          "maxItems": 1\n'),
    (f"{SHOOT}/🧬️schema/🔣️.json", '              "minItems": 38,\n              "maxItems": 38,\n', '              "minItems": 37,\n              "maxItems": 37,\n'),
    (IO_MATRIX, '      "imports": [{ "id": "note", "verb": "loadRequest", "via": "picker", "from": "note" }],\n', '      "imports": [],\n'),
    (IO_MATRIX, '      "imports": [{ "id": "document", "verb": "loadRequest", "via": "picker", "from": "document" }],\n', '      "imports": [],\n'),
]


def oracle_counts(text: str) -> str:
    return text.replace('"routes": 38,', '"routes": 37,').replace('"bounded": 37,', '"bounded": 36,').replace('"migrated": 37,', '"migrated": 36,')


def main() -> None:
    texts: dict[str, str] = {}
    notes = []
    for rel, old, new in HUNKS:
        text = texts.setdefault(rel, (ROOT / rel).read_text())
        count = text.count(old)
        if count == 1:
            texts[rel] = text.replace(old, new)
            notes.append(f"apply     {rel.split('/')[-3]}/…/{rel.split('/')[-1]}: {old.strip().splitlines()[0][:70]}")
        elif count == 0 and (new == "" or new in text):
            notes.append(f"applied   {rel.split('/')[-1]}: {old.strip().splitlines()[0][:70]}")
        else:
            notes.append(f"CONFLICT  {rel}: anchor found {count}× — {old.strip().splitlines()[0][:70]}")
    limits = f"{SHOOT_EDITOR}/🧫️fixtures/🧫️retained-command-limits/🔣️.json"
    before = texts[limits]
    texts[limits] = oracle_counts(before)
    notes.append("apply     retained-command-limits oracle counts 38/37/37 → 37/36/36" if texts[limits] != before else "applied   retained-command-limits oracle counts")
    load_request = ROOT / NOTE_LOAD_REQUEST
    notes.append(f"{'apply    ' if load_request.exists() else 'applied  '} delete {NOTE_LOAD_REQUEST}")
    print("\n".join(notes))
    if any(line.startswith("CONFLICT") for line in notes):
        raise SystemExit("conflict: nothing written")
    if not DRY:
        for rel, text in texts.items():
            (ROOT / rel).write_text(text)
        if load_request.exists():
            load_request.unlink()
            if not any(load_request.parent.iterdir()):
                load_request.parent.rmdir()
    print(f"{'dry-run' if DRY else 'applied'}: {len(texts)} files")


if __name__ == "__main__":
    main()
