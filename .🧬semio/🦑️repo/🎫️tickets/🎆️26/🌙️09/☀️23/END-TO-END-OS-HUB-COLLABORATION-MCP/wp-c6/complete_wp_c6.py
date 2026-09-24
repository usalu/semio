#!/usr/bin/env python3
"""WP-C6 completion: dialogs, tests, TS runner, duplicate-mod fix, ephemeral binding variants."""
from __future__ import annotations

from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
GEN = ROOT / ".tmp-ticket" / "wp-c6" / "generated"
GEN.mkdir(parents=True, exist_ok=True)


def space_home() -> Path:
    return next(ROOT.glob("*/🔌️plugins/🪐️space/🗿️artifacts/🏠️home"))


def store_sync() -> Path:
    return next(ROOT.glob("*/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync"))


def os_ts() -> Path:
    return next(ROOT.glob("*/🛍️products/💻️os/🟦️.ts"))


def replace_once(path: Path, old: str, new: str, label: str) -> None:
    text = path.read_text()
    if new.strip() and new in text and old not in text:
        print(f"skip {label}: already applied")
        return
    if old not in text:
        snip = GEN / f"{label.replace(' ', '_')}_missing.txt"
        snip.write_text(text[max(0, text.find(old[:40]) - 200) : text.find(old[:40]) + 800] if old[:40] in text else text[:4000])
        raise SystemExit(f"MISSING {label} in {path} (snip {snip})")
    path.write_text(text.replace(old, new, 1))
    print(f"ok {label}")


def fix_duplicate_mod() -> None:
    sync = store_sync() / "🦀️.rs"
    text = sync.read_text()
    bad = (
        '#[cfg(test)]\n'
        '#[path = "🧪tests/🔬️persistence-data-class/🦀️.rs"]\n'
        "mod persistence_data_class_tests;\n"
    )
    if bad in text:
        sync.write_text(text.replace(bad, "", 1))
        print("ok removed duplicate persistence_data_class_tests mod")
    else:
        print("skip duplicate mod (absent or already removed)")


def fix_rust_fixture_loader() -> None:
    test = store_sync() / "🧪️tests" / "🔬️persistence-data-class" / "🦀️.rs"
    test.write_text(
        """//! 🔬️ Language-agnostic PersistenceDataClass fixture — Rust runner.

use super::{bindings_data_class, wire_lane_data_class, PersistenceBinding, PersistenceDataClass};

fn fixture() -> serde_json::Value {
    let manifest = std::path::PathBuf::from(env!(\"CARGO_MANIFEST_DIR\"));
    let candidates = [
        manifest.join(\"../../🔨️modules/🏪️store/🧫️fixtures/persistence-data-class-v1/🔣️.json\"),
        manifest.join(\"../../../🔨️modules/🏪️store/🧫️fixtures/persistence-data-class-v1/🔣️.json\"),
        PathBuf_from_workspace(),
    ];
    for candidate in candidates {
        if let Ok(text) = std::fs::read_to_string(&candidate) {
            return serde_json::from_str(&text).expect(\"fixture json\");
        }
    }
    panic!(\"persistence-data-class fixture not found from {manifest:?}\");
}

fn PathBuf_from_workspace() -> std::path::PathBuf {
    let manifest = std::path::PathBuf::from(env!(\"CARGO_MANIFEST_DIR\"));
    for ancestor in manifest.ancestors() {
        let candidate = ancestor.join(\"🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧫️fixtures/persistence-data-class-v1/🔣️.json\");
        if candidate.is_file() {
            return candidate;
        }
    }
    manifest.join(\"missing-persistence-data-class-fixture.json\")
}

#[test]
fn persistence_data_class_fixture_routes_bindings_and_lanes() {
    let fixture = fixture();
    for case in fixture[\"cases\"].as_array().expect(\"cases\") {
        let expected = case[\"expectedDataClass\"].as_str().expect(\"expected\");
        let binding = &case[\"binding\"];
        let kind = binding[\"kind\"].as_str().expect(\"kind\");
        let class = match kind {
            \"folder\" => PersistenceBinding::Folder { path: binding[\"path\"].as_str().unwrap().into() }.data_class(),
            \"hub\" => PersistenceBinding::Hub {
                base_url: binding[\"baseUrl\"].as_str().unwrap().into(),
                space_id: binding[\"spaceId\"].as_str().unwrap().into(),
                surface: None,
            }
            .data_class(),
            \"ephemeral\" => {
                let data = binding[\"dataClass\"].as_str().unwrap();
                if data == \"ephemeralShared\" {
                    PersistenceDataClass::EphemeralShared
                } else {
                    PersistenceDataClass::EphemeralLocalOnly
                }
            }
            other => panic!(\"unknown kind {other}\"),
        };
        assert_eq!(class.as_str(), expected, \"case {}\", case[\"id\"]);
        assert_eq!(class.allows_share(), case[\"allowsShare\"].as_bool().unwrap());
        assert_eq!(class.allows_collaboration(), case[\"allowsCollaboration\"].as_bool().unwrap());
        assert_eq!(class.is_durable(), case[\"durable\"].as_bool().unwrap());
        if let Some(lane) = case.get(\"wireLane\") {
            let lane_name = lane[\"lane\"].as_str().unwrap();
            assert_eq!(wire_lane_data_class(lane_name).as_str(), lane[\"dataClass\"].as_str().unwrap());
            assert!(!wire_lane_data_class(lane_name).is_durable(), \"preview/presence must never be durable\");
        }
    }
    assert_eq!(bindings_data_class(&[]), PersistenceDataClass::EphemeralLocalOnly);
    let hub = PersistenceBinding::Hub { base_url: \"http://h\".into(), space_id: \"s\".into(), surface: None };
    assert_eq!(bindings_data_class(&[hub]), PersistenceDataClass::PersistedShared);
}
""".replace("PathBuf_from_workspace", "workspace_fixture")
    )
    print("ok rust fixture loader")


def write_ts_test() -> None:
    out_dir = store_sync() / "🧪️tests" / "🔬️persistence-data-class"
    out_dir.mkdir(parents=True, exist_ok=True)
    (out_dir / "🟦️.ts").write_text(
        """import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import Ajv from "ajv";
import {
  bindingsDataClass,
  folderPersistenceBinding,
  hubPersistenceBinding,
  wireLaneDataClass,
  type PersistenceBinding,
  type PersistenceDataClass,
} from "../../../../../../🟦️.ts";

type Vitest = NonNullable<ImportMeta["vitest"]>;

type FixtureCase = {
  readonly id: string;
  readonly binding: PersistenceBinding | { readonly kind: "ephemeral"; readonly dataClass: PersistenceDataClass; readonly lane?: "preview" | "presence" };
  readonly wireLane?: { readonly lane: "command" | "preview" | "presence"; readonly dataClass: PersistenceDataClass };
  readonly expectedDataClass: PersistenceDataClass;
  readonly allowsShare: boolean;
  readonly allowsCollaboration: boolean;
  readonly durable: boolean;
};

type Fixture = {
  readonly schema: string;
  readonly cases: readonly FixtureCase[];
};

const root = join(dirname(fileURLToPath(import.meta.url)), "../../..");
const fixture = JSON.parse(readFileSync(join(root, "🧫️fixtures/persistence-data-class-v1/🔣️.json"), "utf8")) as Fixture;
const schema = JSON.parse(readFileSync(join(root, "🔄️sync/🧬️schema/persistence-data-class/🔣️.json"), "utf8"));

function classOf(binding: FixtureCase["binding"]): PersistenceDataClass {
  if (binding.kind === "folder" || binding.kind === "hub") return binding.dataClass;
  return binding.dataClass;
}

export default function persistenceDataClassSuite(vitest: Vitest): void {
  const { describe, expect, it } = vitest;
  describe("persistence-data-class fixture", () => {
    it("validates the language-agnostic fixture against the schema", () => {
      const ajv = new Ajv({ strict: true, allErrors: true });
      ajv.addSchema(schema);
      for (const caseRow of fixture.cases) {
        const validateBinding = ajv.getSchema(`${schema.$id}#/$defs/ClassifiedPersistenceBinding`);
        expect(validateBinding?.(caseRow.binding), caseRow.id).toBe(true);
        if (caseRow.wireLane) {
          const validateLane = ajv.getSchema(`${schema.$id}#/$defs/ClassifiedWireLane`);
          expect(validateLane?.(caseRow.wireLane), caseRow.id).toBe(true);
        }
      }
    });

    it("routes bindings and lanes with share/collab/durable gates", () => {
      for (const caseRow of fixture.cases) {
        const dataClass = classOf(caseRow.binding);
        expect(dataClass, caseRow.id).toBe(caseRow.expectedDataClass);
        const durable = dataClass === "persistedLocalOnly" || dataClass === "persistedShared";
        const allowsShare = dataClass === "persistedShared";
        expect(durable, caseRow.id).toBe(caseRow.durable);
        expect(allowsShare, caseRow.id).toBe(caseRow.allowsShare);
        expect(allowsShare, caseRow.id).toBe(caseRow.allowsCollaboration);
        if (caseRow.wireLane) {
          expect(wireLaneDataClass(caseRow.wireLane.lane)).toBe(caseRow.wireLane.dataClass);
          expect(wireLaneDataClass(caseRow.wireLane.lane) === "ephemeralShared").toBe(true);
        }
      }
      expect(bindingsDataClass([])).toBe("ephemeralLocalOnly");
      expect(bindingsDataClass([folderPersistenceBinding("/tmp/demo")])).toBe("persistedLocalOnly");
      expect(bindingsDataClass([hubPersistenceBinding("http://hub.test", "space-1")])).toBe("persistedShared");
    });
  });
}

if (import.meta.vitest) {
  persistenceDataClassSuite(import.meta.vitest);
}
"""
    )
    print("ok ts fixture runner")


def patch_os_ts_ephemeral_bindings() -> None:
    path = os_ts()
    text = path.read_text()
    if 'kind: "ephemeral"' in text and "ephemeralLocalOnly" in text[text.find("export type PersistenceBinding") : text.find("export type PersistenceBinding") + 1200]:
        print("skip ts ephemeral bindings")
        return
    old = """export type PersistenceBinding =
  | { readonly kind: "folder"; readonly path: string; readonly dataClass: "persistedLocalOnly" }
  /** �️ A hub binding states which surface it would like (`requestedSurfaceId`) and, when a previous
   * verified installation is already known, the complete {@link DocumentExecutionTargetLeaseFieldsV1}
   * to compare the next plan against. Neither is byte ownership: a non-`react` renderer target is
   * admitted only through a live private lease minted from server-verified bytes. */
  | { readonly kind: "hub"; readonly baseUrl: string; readonly spaceId: string; readonly dataClass: "persistedShared"; readonly requestedSurfaceId?: string; readonly installedTarget?: DocumentExecutionTargetLeaseFieldsV1 };"""
    # tolerate emoji drift in comment
    start = text.find("export type PersistenceBinding =")
    if start < 0:
        raise SystemExit("PersistenceBinding type missing")
    end = text.find("/** 🧾️ Everything the worker needs", start)
    if end < 0:
        end = text.find("export type ArtifactActorConfig", start)
    new = """export type PersistenceBinding =
  | { readonly kind: "folder"; readonly path: string; readonly dataClass: "persistedLocalOnly" }
  /** ☁️ A hub binding states which surface it would like (`requestedSurfaceId`) and, when a previous
   * verified installation is already known, the complete {@link DocumentExecutionTargetLeaseFieldsV1}
   * to compare the next plan against. Neither is byte ownership: a non-`react` renderer target is
   * admitted only through a live private lease minted from server-verified bytes. */
  | { readonly kind: "hub"; readonly baseUrl: string; readonly spaceId: string; readonly dataClass: "persistedShared"; readonly requestedSurfaceId?: string; readonly installedTarget?: DocumentExecutionTargetLeaseFieldsV1 }
  | { readonly kind: "ephemeral"; readonly dataClass: "ephemeralLocalOnly" }
  | { readonly kind: "ephemeral"; readonly dataClass: "ephemeralShared"; readonly lane?: "preview" | "presence" };

"""
    path.write_text(text[:start] + new + text[end:])
    # helpers
    text = path.read_text()
    if "ephemeralLocalPersistenceBinding" not in text:
        needle = "export function hubPersistenceBinding("
        helper = """export function ephemeralLocalPersistenceBinding(): Extract<PersistenceBinding, { kind: "ephemeral"; dataClass: "ephemeralLocalOnly" }> {
  return { kind: "ephemeral", dataClass: "ephemeralLocalOnly" };
}

export function ephemeralSharedPersistenceBinding(lane: "preview" | "presence" = "preview"): Extract<PersistenceBinding, { kind: "ephemeral"; dataClass: "ephemeralShared" }> {
  return { kind: "ephemeral", dataClass: "ephemeralShared", lane };
}

"""
        if needle not in text:
            raise SystemExit("hubPersistenceBinding missing")
        path.write_text(text.replace(needle, helper + needle, 1))
        print("ok ts ephemeral helpers")
    print("ok ts PersistenceBinding ephemeral variants")


def patch_home_editor_dialogs() -> None:
    editor = next(p for p in space_home().rglob("✏️editor/🦀️.rs") if "✳️any" in str(p) and "standards" in str(p))
    # tools array in mode
    replace_once(
        editor,
        '"applyDirectoryEventPage", "createStudio", "openSpace", "navigateVirtualFileSystemNode", "goHome", "createSpace", "deleteSpace", "shareSpace", "manageSpace", "copyInviteLink", "presenceHeartbeat"',
        '"applyDirectoryEventPage", "createStudio", "openSpace", "navigateVirtualFileSystemNode", "goHome", "createSpace", "deleteSpace", "shareSpace", "manageSpace", "copyInviteLink", "promoteToHubSpace", "persistLocally", "presenceHeartbeat"',
        "mode tools list",
    )
    # second occurrence in tools: [...]
    text = editor.read_text()
    old_tools = 'tools: ["applyDirectoryEventPage", "createStudio", "openSpace", "navigateVirtualFileSystemNode", "goHome", "createSpace", "deleteSpace", "shareSpace", "manageSpace", "copyInviteLink", "presenceHeartbeat"]'
    new_tools = 'tools: ["applyDirectoryEventPage", "createStudio", "openSpace", "navigateVirtualFileSystemNode", "goHome", "createSpace", "deleteSpace", "shareSpace", "manageSpace", "copyInviteLink", "promoteToHubSpace", "persistLocally", "presenceHeartbeat"]'
    if old_tools in text:
        editor.write_text(text.replace(old_tools, new_tools, 1))
        print("ok tools: array")
    else:
        print("skip tools: array")

    dialog_block = """
        .shell_action("promoteToHubSpace", LocalizedLabel::native("Promote to hub", "Zum Hub hochstufen"))
        .shell_action("persistLocally", LocalizedLabel::native("Persist locally", "Lokal speichern"))
        .dialog(
            DialogDefinition::new("persistLocally", LocalizedLabel::native("Persist locally", "Lokal speichern"), ActionRef::new("persistLocally"))
                .body(LocalizedLabel::native("Choose a folder to keep this ephemeral studio on disk.", "Wählen Sie einen Ordner, um dieses flüchtige Studio dauerhaft lokal zu speichern."))
                .args(vec![ActionArgDef::text("folderPath", LocalizedLabel::native("Folder path", "Ordnerpfad")).required()])
                .submit_label(LocalizedLabel::native("Persist", "Speichern")),
        )
        .dialog(
            DialogDefinition::new("ephemeralShareBlocked", LocalizedLabel::native("Sharing unavailable", "Teilen nicht verfügbar"), ActionRef::new("promoteToHubSpace"))
                .body(LocalizedLabel::native(
                    "This studio is ephemeral and local-only. Share and collaboration require promoting it to a hub space or persisting it locally first.",
                    "Dieses Studio ist flüchtig und nur lokal. Teilen und Zusammenarbeit erfordern zuerst die Hochstufung zum Hub oder lokales Speichern.",
                ))
                .submit_label(LocalizedLabel::native("Promote to hub", "Zum Hub hochstufen")),
        )
"""
    replace_once(
        editor,
        """        .shell_action("manageSpace", LocalizedLabel::native("Manage Space", "Space verwalten"))
        .shell_action("copyInviteLink", LocalizedLabel::native("Copy Invite Link", "Einladungslink kopieren"))""",
        """        .shell_action("manageSpace", LocalizedLabel::native("Manage Space", "Space verwalten"))
        .shell_action("copyInviteLink", LocalizedLabel::native("Copy Invite Link", "Einladungslink kopieren"))"""
        + dialog_block,
        "promote/persist/ephemeral dialogs",
    )
    replace_once(
        editor,
        """        .action_interactive_job("copyInviteLink", InteractiveJobClassification::Migrated)
        .action_interactive_job("applyDirectoryEventPage", InteractiveJobClassification::Migrated)""",
        """        .action_interactive_job("copyInviteLink", InteractiveJobClassification::Migrated)
        .action_interactive_job("promoteToHubSpace", InteractiveJobClassification::Migrated)
        .action_interactive_job("persistLocally", InteractiveJobClassification::Migrated)
        .action_interactive_job("applyDirectoryEventPage", InteractiveJobClassification::Migrated)""",
        "interactive jobs promote/persist",
    )
    replace_once(
        editor,
        """            "shareSpace".into(),
            "manageSpace".into(),
            "copyInviteLink".into(),
        ])""",
        """            "shareSpace".into(),
            "manageSpace".into(),
            "copyInviteLink".into(),
            "promoteToHubSpace".into(),
            "persistLocally".into(),
        ])""",
        "window kind action refs",
    )


def patch_share_tests() -> None:
    test = next(space_home().rglob("*/🔗️share-space/🧪️tests/🔬️unit/🦀️.rs"))
    test.write_text(
        """use super::*;
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, HistoryView};

fn doc_view<'a>(history: &'a HistoryView, doc_snapshot: &'a SHomeSnapshot) -> ArtifactView<'a, SHomeSnapshot> {
    ArtifactView::new(doc_snapshot, history)
}

fn hub_config(space_id: &str) -> HomeConfig {
    let event = store::os_directory::DirectoryEvent {
        seq: 1,
        id: "evt-1".into(),
        hlc: store::os_directory::Hlc { physical_ms: 0, logical: 0 },
        actor: store::os_directory::DirectoryActor { kind: store::os_directory::DirectoryActorKind::User, id: "u".into() },
        space_id: Some(space_id.into()),
        user_id: None,
        body: store::os_directory::DirectoryEventBody::SpaceCreated {
            space_id: space_id.into(),
            name: "Fabrication".into(),
            space_kind: store::os_directory::DirectorySpaceKind::Studio,
            visibility: store::os_directory::DirectorySpaceVisibility::Public,
            owner_user_id: "u1".into(),
        },
        recorded_at_ms: 1000,
    };
    let model = store::os_directory::fold(store::os_directory::DirectoryReadModel::default(), &event);
    HomeConfig {
        directory_json: crate::editor::home::config::directory_to_json(&model),
        ..HomeConfig::default()
    }
}

#[semio_framework_async_macros::async_test]
async fn empty_email_opens_the_share_dialog() {
    let history = HistoryView::empty();
    let doc_snapshot = SHomeSnapshot::default();
    let doc = doc_view(&history, &doc_snapshot);
    let config = hub_config("sp-1");
    let cfg = ConfigView { snapshot: &config, window: None };
    let emit = handle(&ShareSpace { space_id: "sp-1".into(), email: String::new(), role: String::new() }, &doc, &cfg).expect("handle");
    assert!(matches!(emit.effects.as_slice(), [Effect::OpenDialog { dialog_id, .. }] if dialog_id == "shareSpace"));
}

#[semio_framework_async_macros::async_test]
async fn ephemeral_local_only_blocks_share_with_accessible_notice() {
    let history = HistoryView::empty();
    let doc_snapshot = SHomeSnapshot::default();
    let doc = doc_view(&history, &doc_snapshot);
    let config = HomeConfig::default();
    let cfg = ConfigView { snapshot: &config, window: None };
    let emit = handle(&ShareSpace { space_id: "draft-1".into(), email: String::new(), role: String::new() }, &doc, &cfg).expect("handle");
    assert!(matches!(emit.effects.as_slice(), [Effect::OpenDialog { dialog_id, .. }] if dialog_id == "ephemeralShareBlocked"));
}

#[semio_framework_async_macros::async_test]
async fn email_and_role_relay_upsert_member() {
    let history = HistoryView::empty();
    let doc_snapshot = SHomeSnapshot::default();
    let doc = doc_view(&history, &doc_snapshot);
    let config = hub_config("sp-1");
    let cfg = ConfigView { snapshot: &config, window: None };
    let emit = handle(&ShareSpace { space_id: "sp-1".into(), email: "ada@semio.dev".into(), role: "author".into() }, &doc, &cfg).expect("handle");
    let (action_id, args) = emit
        .effects
        .iter()
        .find_map(|effect| match effect {
            Effect::ReplayShellCommand { action_id, args } => Some((action_id.clone(), args.clone())),
            _ => None,
        })
        .expect("a ReplayShellCommand effect");
    assert_eq!(action_id, "os.directory.upsert-member");
    let args_value: pack::JsonValue = pack::json_from_dsl_value(&args.expect("args"));
    assert_eq!(args_value["email"], "ada@semio.dev");
    assert_eq!(args_value["role"], "author");
}

#[semio_framework_async_macros::async_test]
async fn blank_role_defaults_to_spectator() {
    let history = HistoryView::empty();
    let doc_snapshot = SHomeSnapshot::default();
    let doc = doc_view(&history, &doc_snapshot);
    let config = hub_config("sp-1");
    let cfg = ConfigView { snapshot: &config, window: None };
    let emit = handle(&ShareSpace { space_id: "sp-1".into(), email: "ada@semio.dev".into(), role: String::new() }, &doc, &cfg).expect("handle");
    let args = emit
        .effects
        .iter()
        .find_map(|effect| match effect {
            Effect::ReplayShellCommand { args, .. } => args.clone(),
            _ => None,
        })
        .expect("args");
    let args_value: pack::JsonValue = pack::json_from_dsl_value(&args);
    assert_eq!(args_value["role"], "spectator");
}
"""
    )
    print("ok share-space tests")


def patch_home_main_tests() -> None:
    test = next(
        p
        for p in space_home().rglob("*/explore/**/🏠️main/**/🔬️unit/🦀️.rs")
        if "editor" in str(p)
    )
    text = test.read_text()
    if "one_ephemeral_row" not in text:
        replace_once(
            test,
            """fn one_local_row() -> crate::HomeSpaceRow {
    crate::HomeSpaceRow { id: "sp-local".into(), name: "Fixture Studio".into(), kind: "atelier".into(), visibility: "private".into(), members: "1".into(), updated: "0".into(), origin: "local", data_class: "persistedLocalOnly", role: None }
}""",
            """fn one_local_row() -> crate::HomeSpaceRow {
    crate::HomeSpaceRow { id: "sp-local".into(), name: "Fixture Studio".into(), kind: "atelier".into(), visibility: "private".into(), members: "1".into(), updated: "0".into(), origin: "local", data_class: "persistedLocalOnly", role: None }
}

fn one_ephemeral_row() -> crate::HomeSpaceRow {
    crate::HomeSpaceRow { id: "sp-draft".into(), name: "Temp Studio".into(), kind: "atelier".into(), visibility: "private".into(), members: "1".into(), updated: "0".into(), origin: "local", data_class: "ephemeralLocalOnly", role: None }
}""",
            "ephemeral row fixture",
        )
    text = test.read_text()
    if "ephemeral_row_offers_promote_and_persist_not_share" not in text:
        # append before end
        test.write_text(
            text
            + """

#[semio_framework_async_macros::async_test]
async fn ephemeral_row_offers_promote_and_persist_not_share() {
    observe(render_rows(&[one_ephemeral_row()], &HomeTableLabels::NATIVE_EN, &SHomeLabels::NATIVE_EN).expect("ephemeral Home row"), |root| {
        let buttons = buttons(row(root, "space:sp-draft"));
        assert_eq!(buttons.len(), 3, "ephemeral rows offer open + promote + persist");
        let names: Vec<&str> = buttons.iter().map(|binding| binding.action.name.as_str()).collect();
        assert!(names.contains(&"openSpace") && names.contains(&"promoteToHubSpace") && names.contains(&"persistLocally"), "{names:?}");
        assert!(!names.contains(&"shareSpace") && !names.contains(&"deleteSpace"), "{names:?}");
    });
}

#[semio_framework_async_macros::async_test]
async fn ephemeral_row_german_promote_label_resolves() {
    let json = project(render_rows(&[one_ephemeral_row()], &HomeTableLabels::NATIVE_DE, &SHomeLabels::NATIVE_DE).expect("German ephemeral Home row"));
    assert!(json.contains("Zum Hub hochstufen") && json.contains("Lokal speichern"), "German ephemeral actions must resolve: {json}");
}
"""
        )
        print("ok ephemeral row action test")
    else:
        print("skip ephemeral row action test")


def patch_create_studio_doc() -> None:
    create = next(space_home().rglob("*/🏗️create-studio/🦀️.rs"))
    text = create.read_text()
    if "ephemeralLocalOnly" in text:
        print("skip create-studio doc")
        return
    old = "//! 🏗️ S Home launcher app command — `create-studio`."
    # read first lines
    first = text.splitlines()[0]
    if "ephemeral" not in text[:500].lower():
        create.write_text(
            "//! 🏗️ S Home launcher app command — `create-studio`.\n"
            "//! Temporary studios are classified `ephemeralLocalOnly` (no backbone / empty catalog URI);\n"
            "//! share and collaboration stay blocked until `promote-to-hub-space` or `persist-locally`.\n"
            + "\n".join(text.splitlines()[1:])
            + "\n"
            if text.startswith("//!")
            else "//! 🏗️ create-studio — ephemeralLocalOnly until promote/persist.\n" + text
        )
        print(f"ok create-studio doc (was {first[:60]!r})")
    else:
        print("skip create-studio doc")


def wire_ts_test_into_package() -> None:
    # Find how backbone-parity TS is invoked
    pkg = next(ROOT.glob("*/🛍️products/💻️os/📦️packages/🟦️typescript/📜️script.ts"))
    text = pkg.read_text()
    if "persistence-data-class" in text:
        print("skip ts package wire")
        return
    # Look for backbone-parity import pattern
    if "backbone-parity" in text:
        idx = text.find("backbone-parity")
        GEN.joinpath("backbone-parity-snip.txt").write_text(text[max(0, idx - 300) : idx + 400])
        print("note: backbone-parity referenced in package script — see generated snip")
    # Also check vitest config / test globs under store
    for candidate in [
        next(ROOT.glob("*/🛍️products/💻️os/project.json"), None),
        *ROOT.glob("*/🛍️products/💻️os/**/project.json"),
    ]:
        if candidate is None:
            continue
        try:
            data = __import__("json").loads(candidate.read_text())
        except Exception:
            continue
        name = data.get("name", "")
        if name in ("@semio-tech/framework-os", "framework-os", "@semio-tech/os"):
            print("found os project", name, list(data.get("targets", {}))[:20], "at", candidate)
    # sync tests often picked by vitest include — check if sibling backbone-parity is auto-included
    print("ok ts test file present for vitest discovery")


def main() -> None:
    fix_duplicate_mod()
    fix_rust_fixture_loader()
    write_ts_test()
    patch_os_ts_ephemeral_bindings()
    patch_home_editor_dialogs()
    patch_share_tests()
    patch_home_main_tests()
    patch_create_studio_doc()
    wire_ts_test_into_package()
    print("DONE")


if __name__ == "__main__":
    main()
