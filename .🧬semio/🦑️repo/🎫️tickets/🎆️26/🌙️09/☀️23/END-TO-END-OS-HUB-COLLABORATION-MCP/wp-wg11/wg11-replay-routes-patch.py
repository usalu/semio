#!/usr/bin/env python3
"""📣️ WG11 session 14c — window-3 T3 set: the wgpu shell never drops a guest `replayShellCommand` silently.

Measured (SH2, 2026-09-28; re-read by WG11 on the live tree): the wgpu shell's `handle_replay_shell_command` served only
`os.directory.*` and `os.open-artifact*` — `os.create-space-artifact` (the Space index's "New document") and SH2's route-B
`os.local-catalog.*` (keep a studio on this device) fell through a documented no-op, and on wasm32 (the browser wgpu shell)
BOTH replay arms were compiled out, so every replay reached `queue_host_effects` and was dropped with a debug line. React's
ShellHost routes create through the worker's creation job and refuses every unserved gate out loud (`📣️replay-refusal`).

Set:
  1. `🏛️ShellHost/📣️replay-refusal/🔣️.json` (NEW) — the schema-first vocabulary both shells read (reasons, en + de texts, the
     fault-code prefix, shared vectors); the TS module derives its closed reason type, labels and codes from it (API unchanged)
     and gains `local-catalog-unavailable`.
  2. wgpu shell: `ReplayRefusalReasonV1` + `REPLAY_REFUSAL_V1` (the same JSON, `include_str!`), `refuse_replay` (localized
     warning notice with its fault code), `os.create-space-artifact` → the shell's ONE creation door
     (`space_artifact_creation_replay_choice` = ShellHost's `spaceArtifactCreationRequestFromAction` over this shell's catalog:
     exact `{kindChoice, name}`, a canonical choice equal to one kind the space's ready catalog offers; then the door seals,
     submits and opens the ready artifact), `os.local-catalog.*` → `local-catalog-unavailable` (the native half of route B
     follows SH2's B1 vocabulary), any other unserved `os.*` → `unrouted-command` / `invalid-request`; the replay arms serve
     wasm32 too.
  3. Laws: Rust (hub-projection-workspace) — vocabulary keys == reasons + shared vectors; unserved replays refused out loud in
     en + de; the creation choice gate (canonical, offered, exact keys, right space). TS (engine-contract) — the same vectors.

Dry run by default; `--write` backs every edited file up byte-for-byte under `wp-wg11/w3-backup/replay-routes/` and applies
(every anchor asserted exactly once; refuses when a new file exists); `--revert` restores the backups and removes the new file.
"""

import difflib
import json
import shutil
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
ENGINE = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine"
SHELL = ENGINE / "🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs"
SHELL_LAWS = ENGINE / "🧱️elements/🐚️Shell/🧪️tests/🔗️hub-projection-workspace/🦀️.rs"
REFUSAL_TS = ENGINE / "🧱️elements/🏛️ShellHost/📣️replay-refusal/🟦️.ts"
REFUSAL_JSON = ENGINE / "🧱️elements/🏛️ShellHost/📣️replay-refusal/🔣️.json"
CONTRACT_TS = ENGINE / "🧪️tests/🔬️engine-contract/🟦️.ts"
BACKUP = Path(__file__).resolve().parent / "w3-backup/replay-routes"

LABELS = {
    "sign-in-required": ("Sign in to run that command.", "Zum Ausführen dieses Befehls bitte anmelden."),
    "space-required": ("Open a space first — that command needs one.", "Zuerst einen Space öffnen — dieser Befehl braucht einen."),
    "space-index-required": ("Open the space itself to create a document there.", "Zum Anlegen eines Dokuments den Space selbst öffnen."),
    "invalid-request": ("That command was incomplete and was not run.", "Dieser Befehl war unvollständig und wurde nicht ausgeführt."),
    "router-not-ready": ("The workspace is still loading — try again in a moment.", "Der Arbeitsbereich lädt noch — bitte gleich erneut versuchen."),
    "open-rejected": ("That document could not be opened.", "Dieses Dokument konnte nicht geöffnet werden."),
    "view-only-access": ("You can only view documents in this space — editing needs the Author role.", "In diesem Space können Sie Dokumente nur ansehen — zum Bearbeiten ist die Rolle Autor nötig."),
    "unrouted-command": ("This shell has no route for that command.", "Diese Shell kennt keinen Weg für diesen Befehl."),
    "local-catalog-unavailable": ("This shell cannot keep a studio on this device — open it in the web shell or keep it in a hub space.", "Diese Shell kann kein Studio auf diesem Gerät behalten — in der Web-Shell öffnen oder in einem Hub-Space behalten."),
}
PREFIX = "shell.replayShellCommand."
VOCABULARY = {
    "schema": "semio.os.shell-replay-refusal/v1",
    "description": "Every way a shell refuses a guest's replayShellCommand — a durable gesture the user made — with the notice text each reason shows (en, de; any other locale reads en) and the fault code a probe, a law and the console name it by (codePrefix + reason). One reason per gate, so a notice names what the user can act on. Read by React's ShellHost (📣️replay-refusal/🟦️.ts) and by the wgpu shell (🐚️Shell/🎯️targets/🧊️wgpu); vectors are answered by both implementations' laws.",
    "codePrefix": PREFIX,
    "reasons": {reason: {"en": en, "de": de} for reason, (en, de) in LABELS.items()},
    "vectors": [
        {"reason": "local-catalog-unavailable", "locale": "de", "text": LABELS["local-catalog-unavailable"][1], "code": PREFIX + "local-catalog-unavailable"},
        {"reason": "local-catalog-unavailable", "locale": "en", "text": LABELS["local-catalog-unavailable"][0], "code": PREFIX + "local-catalog-unavailable"},
        {"reason": "sign-in-required", "locale": "de", "text": LABELS["sign-in-required"][1], "code": PREFIX + "sign-in-required"},
        {"reason": "space-index-required", "locale": "en", "text": LABELS["space-index-required"][0], "code": PREFIX + "space-index-required"},
        {"reason": "unrouted-command", "locale": "fr", "text": LABELS["unrouted-command"][0], "code": PREFIX + "unrouted-command"},
        {"reason": "invalid-request", "locale": "de", "text": LABELS["invalid-request"][1], "code": PREFIX + "invalid-request"},
    ],
}

TS_BODY_OLD = '''/** 👁️✏️ The closed set of reasons the shell's own `replayShellCommand` route drops a guest gesture.
 * One reason per gate, so a notice names the thing the user can act on and never the internal id. */
export type ReplayRefusalReasonV1 =
  | "sign-in-required"
  | "space-required"
  | "space-index-required"
  | "invalid-request"
  | "router-not-ready"
  | "open-rejected"
  | "view-only-access"
  | "unrouted-command";

export type ReplayRefusalLabelV1 = { readonly en: string; readonly de: string };

/** 🗣️ Notice text per reason. `locale` picks; only an unknown locale falls back to English. */
export const REPLAY_REFUSAL_LABELS_V1: Readonly<Record<ReplayRefusalReasonV1, ReplayRefusalLabelV1>> = {
  "sign-in-required": { en: "Sign in to run that command.", de: "Zum Ausführen dieses Befehls bitte anmelden." },
  "space-required": { en: "Open a space first — that command needs one.", de: "Zuerst einen Space öffnen — dieser Befehl braucht einen." },
  "space-index-required": { en: "Open the space itself to create a document there.", de: "Zum Anlegen eines Dokuments den Space selbst öffnen." },
  "invalid-request": { en: "That command was incomplete and was not run.", de: "Dieser Befehl war unvollständig und wurde nicht ausgeführt." },
  "router-not-ready": { en: "The workspace is still loading — try again in a moment.", de: "Der Arbeitsbereich lädt noch — bitte gleich erneut versuchen." },
  "open-rejected": { en: "That document could not be opened.", de: "Dieses Dokument konnte nicht geöffnet werden." },
  "view-only-access": { en: "You can only view documents in this space — editing needs the Author role.", de: "In diesem Space können Sie Dokumente nur ansehen — zum Bearbeiten ist die Rolle Autor nötig." },
  "unrouted-command": { en: "This shell has no route for that command.", de: "Diese Shell kennt keinen Weg für diesen Befehl." },
};
'''
TS_BODY_NEW = '''import vocabulary from "./🔣️.json" with { type: "json" };

/** 👁️✏️ The closed set of reasons a shell's own `replayShellCommand` route drops a guest gesture — the keys of the
 * schema-first vocabulary `🔣️.json` the wgpu shell reads too, so both shells refuse in the same words. One reason per gate,
 * so a notice names the thing the user can act on and never the internal id. */
export type ReplayRefusalReasonV1 = keyof typeof vocabulary.reasons;

export type ReplayRefusalLabelV1 = { readonly en: string; readonly de: string };

/** 🗣️ Notice text per reason (`🔣️.json`). `locale` picks; only an unknown locale falls back to English. */
export const REPLAY_REFUSAL_LABELS_V1: Readonly<Record<ReplayRefusalReasonV1, ReplayRefusalLabelV1>> = vocabulary.reasons;
'''
TS_CODE_OLD = '''export function replayRefusalCodeV1(reason: ReplayRefusalReasonV1): string {
  return `shell.replayShellCommand.${reason}`;
}'''
TS_CODE_NEW = '''export function replayRefusalCodeV1(reason: ReplayRefusalReasonV1): string {
  return `${vocabulary.codePrefix}${reason}`;
}'''

SHELL_VOCABULARY = r'''
//#region 📣️ReplayRefusal
/// 📣️ Every way this shell refuses a guest's `replayShellCommand` — a durable gesture the user made — named by the ONE
/// schema-first vocabulary React's ShellHost reads too (`🏛️ShellHost/📣️replay-refusal/🔣️.json`): a refusal is a warning notice
/// in the shell's tongue carrying its fault code, never a silent drop.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum ReplayRefusalReasonV1 {
    SignInRequired,
    SpaceRequired,
    SpaceIndexRequired,
    InvalidRequest,
    RouterNotReady,
    OpenRejected,
    ViewOnlyAccess,
    UnroutedCommand,
    LocalCatalogUnavailable,
}

/// 🗣️ One reason's notice text per supported locale.
#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ReplayRefusalLabelV1 {
    pub en: String,
    pub de: String,
}

/// 🧪️ One shared vector both shells' laws answer: a reason in a locale reads `text` under `code`.
#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ReplayRefusalVectorV1 {
    pub reason: ReplayRefusalReasonV1,
    pub locale: String,
    pub text: String,
    pub code: String,
}

/// 📜️ The shared vocabulary file.
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct ReplayRefusalVocabularyV1 {
    pub schema: String,
    pub description: String,
    pub code_prefix: String,
    pub reasons: BTreeMap<String, ReplayRefusalLabelV1>,
    pub vectors: Vec<ReplayRefusalVectorV1>,
}

/// 📜️ The shared replay-refusal vocabulary, parsed once from the file both shells read.
pub(crate) static REPLAY_REFUSAL_V1: std::sync::LazyLock<ReplayRefusalVocabularyV1> =
    std::sync::LazyLock::new(|| serde_json::from_str(include_str!("../../../🏛️ShellHost/📣️replay-refusal/🔣️.json")).expect("the shared replay-refusal vocabulary parses"));

impl ReplayRefusalReasonV1 {
    pub(crate) const ALL: [Self; 9] =
        [Self::SignInRequired, Self::SpaceRequired, Self::SpaceIndexRequired, Self::InvalidRequest, Self::RouterNotReady, Self::OpenRejected, Self::ViewOnlyAccess, Self::UnroutedCommand, Self::LocalCatalogUnavailable];

    /// 🔑️ The reason's vocabulary key — its own kebab-case wire name.
    pub(crate) fn key(self) -> String {
        serde_json::to_value(self).ok().and_then(|value| value.as_str().map(str::to_owned)).unwrap_or_default()
    }

    /// 🗣️ The notice text in `locale`; only an unknown locale reads English.
    pub(crate) fn notice_text(self, locale: &str) -> String {
        REPLAY_REFUSAL_V1.reasons.get(&self.key()).map(|label| if locale == "de" { label.de.clone() } else { label.en.clone() }).unwrap_or_default()
    }

    /// 🩺️ The fault code the notice carries, so a probe, a law and the console name the same gate.
    pub(crate) fn code(self) -> String {
        format!("{}{}", REPLAY_REFUSAL_V1.code_prefix, self.key())
    }
}

/// 🌱️ The one kind an `os.create-space-artifact` replay may create — ShellHost's `spaceArtifactCreationRequestFromAction`
/// over this shell's creation catalog: exactly `{kindChoice, name}`, the choice a canonical (decode ∘ encode)
/// `ArtifactKindChoice` equal to one kind the ready catalog of `space_id` offers. Answers `(kindId, name)`.
pub(crate) async fn space_artifact_creation_replay_choice(args: Option<&Value>, catalog: Option<&SpaceArtifactCreationCatalogV1>, space_id: &str) -> Result<(String, String), ReplayRefusalReasonV1> {
    let invalid = ReplayRefusalReasonV1::InvalidRequest;
    let args = args.and_then(Value::as_object).filter(|args| args.len() == 2).ok_or(invalid)?;
    let (Some(choice), Some(name)) = (args.get("kindChoice").and_then(Value::as_str), args.get("name").and_then(Value::as_str)) else { return Err(invalid) };
    let decoded = semio_framework::manifest::decode_artifact_kind_choice(choice).await.map_err(|_| invalid)?;
    if semio_framework::manifest::encode_artifact_kind_choice(&decoded).await != choice {
        return Err(invalid);
    }
    let catalog = catalog.filter(|catalog| catalog.space_id == space_id).ok_or(ReplayRefusalReasonV1::RouterNotReady)?;
    for kind in &catalog.kinds {
        let offered = semio_framework::manifest::ArtifactKindChoice {
            kind_id: kind.kind_id.clone(),
            schema: kind.schema.clone(),
            dialect: semio_framework::ArtifactDialect { artifact_kind: kind.dialect.artifact_kind.clone(), standard: kind.dialect.standard.clone(), subset: kind.dialect.subset.clone() },
            label: LocalizedLabel::native(&kind.label.en, &kind.label.de),
        };
        if semio_framework::manifest::encode_artifact_kind_choice(&offered).await == choice {
            return Ok((kind.kind_id.clone(), name.to_string()));
        }
    }
    Err(invalid)
}
//#endregion 📣️ReplayRefusal
'''

REPLAY_OLD = '''    /// 📇️ ticket §C6/§3/§4 — dispatches a `ReplayShellCommand`'s `action_id`/`args` onto the
    /// directory funnel or the opening relay; every other action id is a documented no-op (see this
    /// region's callers' own comment).
    async fn handle_replay_shell_command(&mut self, action_id: &str, args: Option<&DslValue>) {
        let args_json = args.map(dsl_value_as_json);
        if action_id == "os.directory.open-administration" {
            let space_id = args_json.as_ref().and_then(|args| args.get("spaceId")).and_then(Value::as_str).unwrap_or_default().to_string();
            self.open_space_administration(&space_id);
            return;
        }
        if let Some(command) = directory_command_from_action(action_id, args_json.as_ref()) {
            self.dispatch_directory_command(command).await;
            return;
        }
        if action_id == "os.open-artifact" || action_id == "os.open-artifact-with" {
            self.handle_open_artifact_relay(action_id, args_json.as_ref()).await;
        }
    }
'''
REPLAY_NEW = '''    /// 📇️ ticket §C6/§3/§4 — dispatches a `ReplayShellCommand`'s `action_id`/`args` onto the directory funnel, the opening
    /// relay or the creation door; an `os.*` command this shell cannot serve is refused out loud ([`Self::refuse_replay`]) —
    /// the local studio catalog (`os.local-catalog.*`) until route B's native half lands, any unknown one as unrouted.
    async fn handle_replay_shell_command(&mut self, action_id: &str, args: Option<&DslValue>) {
        let args_json = args.map(dsl_value_as_json);
        if action_id == "os.directory.open-administration" {
            let space_id = args_json.as_ref().and_then(|args| args.get("spaceId")).and_then(Value::as_str).unwrap_or_default().to_string();
            self.open_space_administration(&space_id);
            return;
        }
        if let Some(command) = directory_command_from_action(action_id, args_json.as_ref()) {
            self.dispatch_directory_command(command).await;
            return;
        }
        if action_id == "os.open-artifact" || action_id == "os.open-artifact-with" {
            self.handle_open_artifact_relay(action_id, args_json.as_ref()).await;
            return;
        }
        let refusal = if action_id == "os.create-space-artifact" {
            self.replay_create_space_artifact(args_json.as_ref()).await.err()
        } else if action_id.starts_with("os.local-catalog.") {
            Some(ReplayRefusalReasonV1::LocalCatalogUnavailable)
        } else if action_id.starts_with("os.directory.") {
            Some(ReplayRefusalReasonV1::InvalidRequest)
        } else if action_id.starts_with("os.") {
            Some(ReplayRefusalReasonV1::UnroutedCommand)
        } else {
            None
        };
        if let Some(reason) = refusal {
            self.refuse_replay(reason);
        }
    }

    /// 📣️ Refuses one guest `replayShellCommand` out loud: the shared vocabulary's warning notice in this shell's tongue,
    /// carrying the reason's fault code — React's `refuseReplay`.
    fn refuse_replay(&mut self, reason: ReplayRefusalReasonV1) {
        let code = reason.code();
        self.show_transient_notice(reason.notice_text(&self.locale_id), semio_framework::Severity::Warning, Some(code.as_str()));
    }

    /// 🌱️ `os.create-space-artifact {kindChoice, name}` from the mounted Space index guest onto this shell's ONE creation door —
    /// ShellHost's route: a signed-in identity, the index's own space, a choice the space's ready catalog offers
    /// ([`space_artifact_creation_replay_choice`]); the door then seals the intent under its minted key, submits it and opens the
    /// ready artifact ([`Self::pump_hub_artifact_creation`]). A busy or not-yet-ready door asks the user to try again.
    async fn replay_create_space_artifact(&mut self, args: Option<&Value>) -> Result<(), ReplayRefusalReasonV1> {
        if self.identity.is_none() {
            return Err(ReplayRefusalReasonV1::SignInRequired);
        }
        let index_mounted = self.sync_channel.as_ref().is_some_and(|channel| channel.document_id == S_SPACE_INDEX_DOCUMENT_ID);
        let space_id = self.open_space_id.clone().filter(|space_id| index_mounted && !space_id.is_empty()).ok_or(ReplayRefusalReasonV1::SpaceIndexRequired)?;
        if self.hub_workspace.open_space_id.as_deref() != Some(space_id.as_str()) || self.hub_workspace.creation.catalog.as_ref().is_none_or(|catalog| catalog.space_id != space_id) {
            self.hub_workspace.open_space_id = Some(space_id.clone());
            self.open_hub_artifact_creation(&space_id).await;
        }
        let (kind_id, name) = space_artifact_creation_replay_choice(args, self.hub_workspace.creation.catalog.as_ref(), &space_id).await?;
        let creation = &mut self.hub_workspace.creation;
        creation.kind_id = Some(kind_id);
        creation.name_draft = name;
        if hub_artifact_creation_intent(&self.hub_workspace).is_none() {
            let busy = self.hub_workspace.creation.catalog_phase != HubArtifactCatalogPhase::Ready || self.hub_workspace.creation.operation.as_ref().is_some_and(|operation| !hub_artifact_creation_terminal(operation.phase));
            return Err(if busy { ReplayRefusalReasonV1::RouterNotReady } else { ReplayRefusalReasonV1::InvalidRequest });
        }
        self.begin_hub_artifact_creation();
        self.pump_hub_artifact_creation().await;
        Ok(())
    }
'''

SHELL_EDITS = [
    (
        "use semio_framework_os_kernel::os_directory::schema::space_artifact_creation::SpaceArtifactCreationPhaseV1;\n",
        "use semio_framework_os_kernel::os_directory::schema::space_artifact_creation::{SpaceArtifactCreationCatalogV1, SpaceArtifactCreationPhaseV1};\n",
    ),
    (
        '''fn space_index_dialect() -> semio_framework::ArtifactDialect {
    semio_framework::ArtifactDialect { artifact_kind: "s.space.space".to_string(), standard: "1".to_string(), subset: "*".to_string() }
}
''',
        '''fn space_index_dialect() -> semio_framework::ArtifactDialect {
    semio_framework::ArtifactDialect { artifact_kind: "s.space.space".to_string(), standard: "1".to_string(), subset: "*".to_string() }
}
''' + SHELL_VOCABULARY,
    ),
    (
        '''        // RESOLVE the uri, and the native replay relay — because both are `async` and that funnel is''',
        '''        // RESOLVE the uri, and the replay relay — because both are `async` and that funnel is''',
    ),
    (
        '''                    if let Err(error) = self.apply_shell_uri(&uri).await {
                        Self::debug_log(&format!("[DEBUG] wgpu shell navigate effect failed: {error}"));
                    }
                }
                #[cfg(not(target_arch = "wasm32"))]
                semio_framework::kernel::Effect::ReplayShellCommand { action_id, args } => {''',
        '''                    if let Err(error) = self.apply_shell_uri(&uri).await {
                        Self::debug_log(&format!("[DEBUG] wgpu shell navigate effect failed: {error}"));
                    }
                }
                semio_framework::kernel::Effect::ReplayShellCommand { action_id, args } => {''',
    ),
    (
        '''        // that must also RESOLVE the uri, and the native replay relay — stay here, because both are''',
        '''        // that must also RESOLVE the uri, and the replay relay — stay here, because both are''',
    ),
    (
        '''                    self.apply_shell_uri(&uri).await?;
                }
                #[cfg(not(target_arch = "wasm32"))]
                semio_framework::kernel::Effect::ReplayShellCommand { action_id, args } => {''',
        '''                    self.apply_shell_uri(&uri).await?;
                }
                semio_framework::kernel::Effect::ReplayShellCommand { action_id, args } => {''',
    ),
    (REPLAY_OLD, REPLAY_NEW),
]

LAWS_ANCHOR = '''#[cfg(not(target_arch = "wasm32"))]
fn remote_of(shell: &ShellState) -> String {'''
LAWS_NEW = r'''/// 📣️ The shared replay-refusal vocabulary (`🏛️ShellHost/📣️replay-refusal/🔣️.json`) names exactly the reasons this shell can
/// raise, and its vectors — the same ones React's engine contract answers — pin each reason's text per locale and its code.
#[test]
fn the_shared_replay_refusal_vocabulary_names_every_reason_with_its_text_and_code() {
    let vocabulary = &*REPLAY_REFUSAL_V1;
    let reasons: std::collections::BTreeSet<String> = ReplayRefusalReasonV1::ALL.iter().map(|reason| reason.key()).collect();
    assert_eq!(vocabulary.reasons.keys().cloned().collect::<std::collections::BTreeSet<_>>(), reasons, "vocabulary keys == the shell's reasons");
    assert!(vocabulary.reasons.values().all(|label| !label.en.is_empty() && !label.de.is_empty() && label.en != label.de), "every reason speaks both tongues");
    assert!(!vocabulary.vectors.is_empty());
    for vector in &vocabulary.vectors {
        assert_eq!((vector.reason.notice_text(&vector.locale), vector.reason.code()), (vector.text.clone(), vector.code.clone()), "{vector:?}");
    }
}

/// 📣️ A guest replay this shell cannot serve is refused OUT LOUD — the vocabulary's warning notice in the shell's tongue with
/// its fault code — never dropped: the local studio catalog (route B has no native half yet), an unknown `os.*` command or
/// directory verb, and a space artifact creation nobody signed in for, or from no mounted Space index.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn an_unserved_guest_replay_is_refused_out_loud_in_the_shells_tongue() {
    use ReplayRefusalReasonV1::{InvalidRequest, LocalCatalogUnavailable, SignInRequired, SpaceIndexRequired, UnroutedCommand};
    let identity = || Identity { user_id: "user-a".into(), email: "a@example.test".into(), display_name: "Ada".into(), hub_base_url: "https://hub.example".into(), issued_at_ms: 1 };
    let cases: [(&str, &str, bool, &[(&str, &str)], ReplayRefusalReasonV1); 7] = [
        ("en", "os.local-catalog.admit", false, &[("documentId", "studio-1")], LocalCatalogUnavailable),
        ("de", "os.local-catalog.open", true, &[("documentId", "studio-1")], LocalCatalogUnavailable),
        ("de", "os.local-catalog.retire", true, &[("documentId", "studio-1")], LocalCatalogUnavailable),
        ("en", "os.unknown-verb", true, &[], UnroutedCommand),
        ("de", "os.directory.unknown-verb", true, &[], InvalidRequest),
        ("de", "os.create-space-artifact", false, &[("kindChoice", "{}"), ("name", "Plan")], SignInRequired),
        ("en", "os.create-space-artifact", true, &[("kindChoice", "{}"), ("name", "Plan")], SpaceIndexRequired),
    ];
    for (locale, action_id, signed_in, args, reason) in cases {
        let mut shell = shell();
        shell.locale_id = locale.into();
        if signed_in {
            shell.identity = Some(identity());
        }
        shell_command(&mut shell, action_id, args);
        let notice = shell.transient_notice().unwrap_or_else(|| panic!("{action_id} ({locale}) is refused out loud"));
        let label = &REPLAY_REFUSAL_V1.reasons[&reason.key()];
        let spoken = if locale == "de" { label.de.as_str() } else { label.en.as_str() };
        assert_eq!((notice.message.as_str(), notice.code.as_deref()), (spoken, Some(reason.code().as_str())), "{locale} {action_id}");
        assert!(matches!(notice.severity, semio_framework::Severity::Warning), "{action_id} warns");
    }
}

/// 🌱️ Only a canonical choice of one kind the space's ready catalog offers becomes a creation — ShellHost's
/// `spaceArtifactCreationRequestFromAction` gate: exact `{kindChoice, name}`, the choice byte-equal to the catalog kind's
/// encoding, the catalog of the index's own space.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_space_artifact_creation_replay_names_exactly_one_offered_kind() {
    let catalog: SpaceArtifactCreationCatalogV1 = serde_json::from_value(serde_json::json!({
        "schema": "semio.hub.space-artifact-creation-catalog/v1",
        "spaceId": "space-a",
        "catalogGenerationId": "d1099ba98c89995f462d00c109bc8fcdb5594260e9bdb584e3da628dabbe94ad",
        "kinds": [
            { "kindId": "2d.block", "schema": "block.2d", "dialect": { "artifactKind": "s.block.block2d", "standard": "1", "subset": "*" }, "label": { "en": "Block board", "de": "Blockbrett" } },
            { "kindId": "2d.note", "schema": "note.document", "dialect": { "artifactKind": "s.note.note", "standard": "1", "subset": "*" }, "label": { "en": "Note", "de": "Notiz" } }
        ]
    }))
    .expect("catalog fixture");
    let choice = |kind: &semio_framework_os_kernel::os_directory::schema::space_artifact_creation::SpaceArtifactCreationKindV1| {
        drive(semio_framework::manifest::encode_artifact_kind_choice(&semio_framework::manifest::ArtifactKindChoice {
            kind_id: kind.kind_id.clone(),
            schema: kind.schema.clone(),
            dialect: semio_framework::ArtifactDialect { artifact_kind: kind.dialect.artifact_kind.clone(), standard: kind.dialect.standard.clone(), subset: kind.dialect.subset.clone() },
            label: LocalizedLabel::native(&kind.label.en, &kind.label.de),
        }))
    };
    let note = choice(&catalog.kinds[1]);
    let judged = |args: Value, space: &str| drive(space_artifact_creation_replay_choice(Some(&args), Some(&catalog), space));
    assert_eq!(judged(serde_json::json!({ "kindChoice": note, "name": "Minutes" }), "space-a"), Ok(("2d.note".to_string(), "Minutes".to_string())));
    let mut forged = serde_json::from_str::<Value>(&note).expect("choice is JSON");
    forged["label"]["en"] = Value::from("Notes");
    let refused = [
        (serde_json::json!({ "kindChoice": note, "name": "Minutes", "spaceId": "space-b" }), "space-a", ReplayRefusalReasonV1::InvalidRequest),
        (serde_json::json!({ "kindChoice": note }), "space-a", ReplayRefusalReasonV1::InvalidRequest),
        (serde_json::json!({ "kindChoice": format!(" {note}"), "name": "Minutes" }), "space-a", ReplayRefusalReasonV1::InvalidRequest),
        (serde_json::json!({ "kindChoice": forged.to_string(), "name": "Minutes" }), "space-a", ReplayRefusalReasonV1::InvalidRequest),
        (serde_json::json!({ "kindChoice": note, "name": 7 }), "space-a", ReplayRefusalReasonV1::InvalidRequest),
        (serde_json::json!({ "kindChoice": note, "name": "Minutes" }), "space-b", ReplayRefusalReasonV1::RouterNotReady),
    ];
    for (args, space, reason) in refused {
        assert_eq!(judged(args.clone(), space), Err(reason), "{args} in {space}");
    }
    assert_eq!(drive(space_artifact_creation_replay_choice(Some(&serde_json::json!({ "kindChoice": note, "name": "Minutes" })), None, "space-a")), Err(ReplayRefusalReasonV1::RouterNotReady));
}

'''

CONTRACT_IMPORT_ANCHOR = '''import artifactCreationProgressFixture from "../../🧱️elements/🏛️ShellHost/🧫️fixtures/🌱️artifact-creation/🔣️.json";
'''
CONTRACT_IMPORT_NEW = '''import artifactCreationProgressFixture from "../../🧱️elements/🏛️ShellHost/🧫️fixtures/🌱️artifact-creation/🔣️.json";
import replayRefusalVocabulary from "../../🧱️elements/🏛️ShellHost/📣️replay-refusal/🔣️.json";
import { REPLAY_REFUSAL_LABELS_V1, replayRefusalCodeV1, replayRefusalNoticeTextV1, type ReplayRefusalReasonV1 } from "../../🧱️elements/🏛️ShellHost/📣️replay-refusal/🟦️.ts";
'''
CONTRACT_APPEND = '''
describe("📣️ replay refusal vocabulary", () => {
  it("names every reason in both tongues from the shared vocabulary the wgpu shell reads", () => {
    expect(Object.keys(REPLAY_REFUSAL_LABELS_V1).sort()).toEqual(Object.keys(replayRefusalVocabulary.reasons).sort());
    for (const label of Object.values(REPLAY_REFUSAL_LABELS_V1)) expect(label.en !== "" && label.de !== "" && label.en !== label.de).toBe(true);
  });

  it("answers the shared vectors the wgpu shell's law answers", () => {
    expect(replayRefusalVocabulary.vectors.length).toBeGreaterThan(0);
    for (const vector of replayRefusalVocabulary.vectors) {
      const reason = vector.reason as ReplayRefusalReasonV1;
      expect([replayRefusalNoticeTextV1(reason, vector.locale), replayRefusalCodeV1(reason)]).toEqual([vector.text, vector.code]);
    }
  });
});
'''


def replaced(path: Path, source: str, edits) -> str:
    for old, new in edits:
        count = source.count(old)
        if count != 1:
            sys.exit(f"anchor occurs {count}x in {path.parent.name}/{path.name}: {old[:100]!r}")
        source = source.replace(old, new)
    return source


def plans():
    shell = SHELL.read_text(encoding="utf-8")
    laws = SHELL_LAWS.read_text(encoding="utf-8")
    refusal = REFUSAL_TS.read_text(encoding="utf-8")
    contract = CONTRACT_TS.read_text(encoding="utf-8")
    return [
        (SHELL, shell, replaced(SHELL, shell, SHELL_EDITS)),
        (SHELL_LAWS, laws, replaced(SHELL_LAWS, laws, [(LAWS_ANCHOR, LAWS_NEW + LAWS_ANCHOR)])),
        (REFUSAL_TS, refusal, replaced(REFUSAL_TS, refusal, [(TS_BODY_OLD, TS_BODY_NEW), (TS_CODE_OLD, TS_CODE_NEW)])),
        (CONTRACT_TS, contract, replaced(CONTRACT_TS, contract, [(CONTRACT_IMPORT_ANCHOR, CONTRACT_IMPORT_NEW)]).rstrip("\n") + "\n" + CONTRACT_APPEND),
    ]


def main():
    write = "--write" in sys.argv
    revert = "--revert" in sys.argv
    if revert:
        for path in [SHELL, SHELL_LAWS, REFUSAL_TS, CONTRACT_TS]:
            backup = BACKUP / path.relative_to(ROOT)
            if not backup.exists():
                sys.exit(f"no backup for {path.relative_to(ROOT)}")
            shutil.copyfile(backup, path)
        REFUSAL_JSON.unlink(missing_ok=True)
        print("REVERTED: 4 files restored from backups, vocabulary removed")
        return
    if REFUSAL_JSON.exists():
        sys.exit(f"{REFUSAL_JSON.relative_to(ROOT)} exists — landed already")
    planned = plans()
    for path, before, after in planned:
        sys.stdout.writelines(difflib.unified_diff(before.splitlines(True), after.splitlines(True), f"{path.parent.name}/{path.name}", f"{path.parent.name}/{path.name} (patched)", n=1))
    vocabulary = json.dumps(VOCABULARY, ensure_ascii=False, indent=2) + "\n"
    print(f"\n+++ NEW {REFUSAL_JSON.relative_to(ROOT)} ({len(vocabulary)} chars)")
    if write:
        for path, before, after in planned:
            backup = BACKUP / path.relative_to(ROOT)
            backup.parent.mkdir(parents=True, exist_ok=True)
            backup.write_text(before, encoding="utf-8")
        for path, _, after in planned:
            path.write_text(after, encoding="utf-8")
        REFUSAL_JSON.write_text(vocabulary, encoding="utf-8")
    print(f"\n{'WRITTEN' if write else 'DRY RUN'}: {len(planned)} files edited, 1 new (crates: semio-framework-os-renderer-wgpu native + wasm32; os TS: ShellHost, engine-contract vitest)")


if __name__ == "__main__":
    main()
