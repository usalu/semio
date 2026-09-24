#!/usr/bin/env python3
"""🏠️ U5 6b — one-off: retire Home's unauthenticated `foldDirectoryEvents` writer.

Home's directory projection has exactly one writer after this: the receipt-sealed page (`applyDirectoryEventPage` →
`HomeConfigMutation::ReplaceDirectoryProjection`). A raw fold advanced `directory.cursor` past the sealed receipt, so the
next page (`after == receipt cursor`) answered the non-retryable `s.home.directory-event-page-frontier-race`.
Every replacement is exact and must match once; the script refuses to write anything otherwise."""
import json
import pathlib
import shutil
import sys

ROOT = pathlib.Path("/Users/ueli/Documents/semio")
HOME = ROOT / "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home"
ANY = HOME / "🏅️standards/🔖️1/🪆️subsets/✳️any"
EDITOR = ANY / "✏️editor"

edits: list[tuple[pathlib.Path, str, str]] = []


def edit(path: pathlib.Path, old: str, new: str) -> None:
    edits.append((path, old, new))


edit(HOME / "🦀️.rs",
     '            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📇️fold-directory-events/🦀️.rs"]\n            pub mod fold_directory_events;\n',
     "")

e = EDITOR / "🦀️.rs"
edit(e, "use crate::editor::home::commands::{copy_invite_link, create_space, delete_space, fold_directory_events, manage_space,",
     "use crate::editor::home::commands::{copy_invite_link, create_space, delete_space, manage_space,")
edit(e, '        "foldDirectoryEvents" as "fold-directory-events" => fold_directory_events::FoldDirectoryEvents,\n', "")
edit(e, "invisible while `applyDirectoryEventPage` and `foldDirectoryEvents`\n/// were `BatchOnlyPendingRewrite` and therefore never reached this lane at all",
     "invisible while `applyDirectoryEventPage` was\n/// `BatchOnlyPendingRewrite` and therefore never reached this lane at all")
edit(e, "        | HomeCommand::RenameSpace(_)\n        | HomeCommand::FoldDirectoryEvents(_) => return None,",
     "        | HomeCommand::RenameSpace(_) => return None,")
edit(e, "            HomeConfigMutation::FoldDirectoryEvent { event_json } => (event_json.len(), HOME_CONFIG_BASE_BYTES),\n            _ => return Err(\"Space Home config preparation rejects non-retained mutations\".into()),",
     "            _ => return Err(\"Space Home config preparation rejects non-retained mutations\".into()),")
edit(e, "            HomeConfigMutation::FoldDirectoryEvent { event_json } => (event_json.len(), HOME_CONFIG_BASE_BYTES),\n            _ => return Err(request),",
     "            _ => return Err(request),")
edit(e, """                // ⚙️ The fold is not a field replacement — its post state is the mutation's own diff,
                // and its declared inverse is the exact pre-fold snapshot.
                HomeConfigMutation::FoldDirectoryEvent { .. } => {
                    let diff = ::protocol::Mutation::diff(&mutation, base).into_parts().0;
                    post = ::protocol::MutationDiff::apply(&diff, base).map_err(|_| "Space Home config fold could not apply its own diff".to_string())?;
                    HomeConfigMutation::Snapshot { config: base.clone() }
                }
""", "")
edit(e, """            "foldDirectoryEvents" => {
                Ok(HomeCommand::FoldDirectoryEvents(fold_directory_events::FoldDirectoryEvents { events_json: args.and_then(|value| value.get("eventsJson")).and_then(DslValue::as_str).map_or_else(|| "[]".into(), str::to_string) }))
            }
""", "")
edit(e, '        .view_action("foldDirectoryEvents", LocalizedLabel::native("Fold Directory Events", "Verzeichnisereignisse einspielen"))\n', "")
edit(e, '        .action_interactive_job("foldDirectoryEvents", InteractiveJobClassification::BatchOnlyPendingRewrite)\n', "")

c = EDITOR / "🎚️config/🦀️.rs"
edit(c, """    /// 📇️ JSON-serialized `DirectoryReadModel` (see `🔖️DirectoryJson` above) — folded here by
    /// `HomeConfigMutation::FoldDirectoryEvent` as `/directory/ws` events arrive; read via `directory()`.
    /// No optimistic mutation (contract §C6): the ONLY writer is the fold over hub-confirmed events.""",
     """    /// 📇️ JSON-serialized `DirectoryReadModel` (see `🔖️DirectoryJson` above), read via `directory()`.
    /// No optimistic mutation (contract §C6): the ONLY writer is `HomeConfigMutation::ReplaceDirectoryProjection`,
    /// sealed from one authenticated `DirectoryEventPageV1` together with the three receipt fields below.""")
edit(c, """    /// 📇️ Folds one hub-confirmed `DirectoryEvent` (JSON-encoded, contract §C1) into `directory_json`
    /// — the SOLE writer of the directory read model (contract §C6: no optimistic mutation).
    #[dsl(key = "fold-directory-event")]
    FoldDirectoryEvent { event_json: String },
""", "")
edit(c, """        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/⚙️fold-directory-event", semantic_kind: "fold-directory-event", display_name: "Fold Directory Event", emoji: "⚙️", aggregate_variant: "FoldDirectoryEvent", payload_schema: "🧬️schema/🔣️.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
""", "")
edit(c, """            HomeConfigMutation::FoldDirectoryEvent { .. } => &Self::DESCRIPTORS[1],
            HomeConfigMutation::ReplaceDirectoryProjection { .. } => &Self::DESCRIPTORS[2],""",
     """            HomeConfigMutation::ReplaceDirectoryProjection { .. } => &Self::DESCRIPTORS[1],""")
edit(c, """            HomeConfigMutation::FoldDirectoryEvent { event_json } => {
                if let (Ok(event), Ok(directory)) = (pack::from_json_str::<store::os_directory::DirectoryEvent>(event_json), next.directory()) {
                    next.directory_json = directory_to_json(&store::os_directory::fold(directory, &event));
                }
            }
""", "")

ct = EDITOR / "🎚️config/🧪️tests/🔬️unit/🦀️.rs"
edit(ct, '    store::os_store::test_support::assert_op_line_round_trip(&HomeConfigMutation::FoldDirectoryEvent { event_json: "{}".into() });\n', "")
edit(ct, """#[semio_framework_async_macros::async_test]
async fn fold_directory_event_updates_the_read_model() {
    let config = HomeConfig::default();
    let event_json = pack::json!({
        "seq": 1,
        "id": "evt-1",
        "hlc": { "physicalMs": 0, "logical": 0 },
        "actor": { "kind": "user", "id": "user:u1#s1" },
        "spaceId": "sp-1",
        "body": { "kind": "space.created", "spaceId": "sp-1", "name": "Atelier", "spaceKind": "atelier", "visibility": "private", "ownerUserId": "u1" },
        "recordedAtMs": 1000
    })
    .to_string();
    let next = HomeConfigMutation::FoldDirectoryEvent { event_json }.diff(&config).diff().clone();
    let model = next.directory().expect("folded directory projection");
    assert_eq!(model.cursor, 1);
    let space = model.spaces.get("sp-1").expect("space folded");
    assert_eq!(space.view.name, "Atelier");
}

#[semio_framework_async_macros::async_test]
async fn fold_directory_event_ignores_malformed_json() {
    let config = HomeConfig::default();
    let next = HomeConfigMutation::FoldDirectoryEvent { event_json: "not json".into() }.diff(&config).diff().clone();
    assert_eq!(next.directory_json, config.directory_json, "malformed events never panic and never change the model");
}

""", "")

et = EDITOR / "🧪️tests/🔬️unit/🦀️.rs"
edit(et, """    let base = HomeConfig::default();
    let created = protocol::Mutation::diff(&HomeConfigMutation::FoldDirectoryEvent { event_json: created }, &base).diff().clone();
    protocol::Mutation::diff(&HomeConfigMutation::FoldDirectoryEvent { event_json: member }, &created).diff().clone()
}""", """    let directory = [created, member].iter().map(|event| pack::from_json_str::<store::os_directory::DirectoryEvent>(event).expect("fixture directory event")).fold(store::os_directory::DirectoryReadModel::default(), |model, event| store::os_directory::fold(model, &event));
    HomeConfig { directory_json: crate::editor::home::config::directory_to_json(&directory), ..HomeConfig::default() }
}""")

rt = EDITOR / "🎮️commands/🏷️rename-space/🧪️tests/🔬️unit/🦀️.rs"
edit(rt, "    let config = protocol::Mutation::diff(&HomeConfigMutation::FoldDirectoryEvent { event_json }, &HomeConfig::default()).diff().clone();",
     "    let event = pack::from_json_str::<store::os_directory::DirectoryEvent>(&event_json).expect(\"fixture directory event\");\n    let config = HomeConfig { directory_json: crate::editor::home::config::directory_to_json(&store::os_directory::fold(store::os_directory::DirectoryReadModel::default(), &event)), ..HomeConfig::default() };")

cs = EDITOR / "🎮️commands/🌱create-space/🦀️.rs"
edit(cs, """//! the resulting `space.created` event returns over `/directory/ws` and is folded back in by
//! `fold-directory-events`. No optimistic mutation of the read model.""",
     """//! the resulting `space.created` event wakes the shell's acknowledged directory stream, whose next sealed
//! page reaches `apply-directory-event-page`. No optimistic mutation of the read model.""")

v = ANY / "👁️viewer/🦀️.rs"
edit(v, "use crate::editor::home::config::{HomeConfig, HomeConfigMutation};", "use crate::editor::home::config::{HomeConfig, HomeConfigMutation};")
edit(v, """/// 👁️ Ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS: the viewer's config now
/// carries the folded hub directory read model (`HomeConfig`, shared with the editor — see
/// `HomeViewer::Config` below), so this Home viewer session ALSO needs to receive folded directory
/// events when it is the currently-mounted session (contract §C6: the shell's directory lane folds into
/// whichever session is mounted, editor or viewer). `Noop` stays the `Default` variant
/// (`assert_viewer_never_mutates` requires `Command: Default` and only ever dispatches the default).
#[derive(Clone, Debug, PartialEq, Default)]
pub enum HomeViewCommand {
    #[default]
    Noop,
    /// 📇️ Mirrors the editor's `fold-directory-events` command — a config-only fold, never an
    /// artifact/draft mutation (structurally impossible here: `ViewEmit` has no such field).
    FoldDirectoryEvents { events_json: String },
}

impl protocol::OpBinary for HomeViewCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        match self {
            HomeViewCommand::Noop => Ok(vec![0]),
            HomeViewCommand::FoldDirectoryEvents { events_json } => {
                let mut out = vec![1u8];
                out.extend_from_slice(events_json.as_bytes());
                Ok(out)
            }
        }
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        match bytes.first() {
            Some(1) => Ok(HomeViewCommand::FoldDirectoryEvents { events_json: String::from_utf8_lossy(&bytes[1..]).into_owned() }),
            _ => Ok(HomeViewCommand::Noop),
        }
    }
}""", """/// 👁️ The read-only Home surface accepts no command: its directory projection is written only by the
/// editor's sealed-page lane (`applyDirectoryEventPage`), which the viewer renders but never advances.
#[derive(Clone, Debug, PartialEq, Default)]
pub enum HomeViewCommand {
    #[default]
    Noop,
}

impl protocol::OpBinary for HomeViewCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(HomeViewCommand::Noop)
    }
}""")
edit(v, """    /// 👁️ Structurally read-only: neither variant ever carries an artifact/draft mutation (`ViewEmit`
    /// has no such field to carry one in). `Noop` returns the empty emit; `FoldDirectoryEvents` folds
    /// each event into `HomeConfigMutation::FoldDirectoryEvent`, the SAME config-only writer the editor
    /// uses — never an optimistic mutation, never a document edit.
    fn handle(command: &Self::Command, _doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _interaction: &InteractionView<'_>, view_state: Option<&semio_framework_plugin::ViewModel>, _engines: &EngineHandles) -> Result<ViewEmit<Self::ConfigMutation>, Fault> {
        match command {
            HomeViewCommand::Noop => Ok(ViewEmit::default()),
            HomeViewCommand::FoldDirectoryEvents { events_json } => {
                view_state.and_then(crate::home_session_identity).ok_or_else(|| Fault::from("s.home.session-identity-required"))?;
                let events: Vec<store::os_directory::DirectoryEvent> = pack::from_json_str(events_json).unwrap_or_default();
                let config_mutations = events.iter().map(pack::to_json_string).map(|event_json| HomeConfigMutation::FoldDirectoryEvent { event_json }).collect();
                Ok(ViewEmit::config(config_mutations))
            }
        }
    }""", """    /// 👁️ Structurally read-only: the sole `HomeViewCommand::Noop` answers the empty emit — no config
    /// change, no effect, never a document edit.
    fn handle(_command: &Self::Command, _doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _interaction: &InteractionView<'_>, _view_state: Option<&semio_framework_plugin::ViewModel>, _engines: &EngineHandles) -> Result<ViewEmit<Self::ConfigMutation>, Fault> {
        Ok(ViewEmit::default())
    }""")

vt = ANY / "👁️viewer/🧪️tests/🔬️unit/🦀️.rs"
edit(vt, "async fn fold_directory_events_command_never_touches_the_document_store() {", "async fn viewer_never_touches_the_document_store() {")

TAXONOMY = ROOT / "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"
edit(TAXONOMY, '        "📇️fold-directory-events",\n', "")

by_path: dict[pathlib.Path, str] = {}
for path, old, new in edits:
    text = by_path.get(path) or path.read_text(encoding="utf-8")
    if old == new:
        continue
    count = text.count(old)
    if count != 1:
        sys.exit(f"refused: {path.relative_to(ROOT)} expects one match, found {count}: {old[:90]!r}")
    by_path[path] = text.replace(old, new)

fixture = EDITOR / "🧫️fixtures/🧫️retained-command-limits/🔣️.json"
fixture_text = fixture.read_text(encoding="utf-8")
fixture_old = """    {
      "id": "foldDirectoryEvents",
      "disposition": "BatchOnlyPendingRewrite",
      "lanes": [],
      "blocker": "unbounded event-page parsing and full projection fold still execute in one direct handler"
    },
"""
if fixture_text.count(fixture_old) != 1:
    sys.exit("refused: fixture route row")
by_path[fixture] = fixture_text.replace(fixture_old, "")

schema = ANY / "🧬️schema/🔣️.json"
schema_text = schema.read_text(encoding="utf-8")
schema_old = """                    {
                      "type": "object",
                      "additionalProperties": false,
                      "required": [
                        "id",
                        "disposition",
                        "lanes",
                        "blocker"
                      ],
                      "properties": {
                        "id": {
                          "const": "foldDirectoryEvents"
                        },
                        "disposition": {
                          "const": "BatchOnlyPendingRewrite"
                        },
                        "lanes": {
                          "const": []
                        },
                        "blocker": {
                          "const": "unbounded event-page parsing and full projection fold still execute in one direct handler"
                        }
                      }
                    },
"""
for old, new in [(schema_old, ""), ('                  "minItems": 18,\n                  "maxItems": 18,', '                  "minItems": 17,\n                  "maxItems": 17,'), ("sixteen-route disposition census", "seventeen-route disposition census")]:
    if schema_text.count(old) != 1:
        sys.exit(f"refused: schema {old[:60]!r}")
    schema_text = schema_text.replace(old, new)
by_path[schema] = schema_text

for path in (fixture, schema):
    json.loads(by_path[path])
json.loads(by_path[TAXONOMY])
for path, text in by_path.items():
    path.write_text(text, encoding="utf-8")
    print(f"edited {path.relative_to(ROOT)}")
shutil.rmtree(EDITOR / "🎮️commands/📇️fold-directory-events")
print("removed 🎮️commands/📇️fold-directory-events")
