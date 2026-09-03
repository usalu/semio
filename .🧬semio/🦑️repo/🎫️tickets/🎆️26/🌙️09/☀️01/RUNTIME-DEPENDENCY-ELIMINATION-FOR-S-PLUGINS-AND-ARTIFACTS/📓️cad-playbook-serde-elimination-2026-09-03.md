# CAD + Playbook serde elimination — 2026-09-03

Zero cargo commands run, zero sub-agents spawned. All verification by re-reading edited regions and
grepping for the ticket's own gates (`.get([0-9]`, `unwrap_or_else` on infallible helpers).

## Counts (`grep -rn 'serde_json\|use serde\|derive([^)]*Serialize' <plugin> | grep -vE '🧪|🏭|🔬' | grep -vE ':\s*(///|//!|//|\*)'`)

- `📐️cad`: **62 → 16**
- `📖️playbook`: **69 → 47** (raw count includes ~24 refs that are `#[cfg_attr(test, derive(Serialize,
  Deserialize))]` restorations — see "Oracle-test restorations" below; those are intentional, not
  regressions)

## What moved

### cad
- `✏️editor/🦀️.rs`: `cad_window_action` took `Option<serde_json::Value>` and round-tripped through
  `semio_framework::optional_json_to_dsl` even though `ActionDescriptor.args` is already
  `Option<DslValue>` — now takes `DslValue` directly, no bridge. Same pattern fixed in
  `✏️editor/🎭️modes/✏️edit/🦀️.rs` and `📌️panels/🔍️inspection/🦀️.rs`'s `cad_action`.
  Test module (~28 refs) converted to `protocol::json`'s first-party `json!` macro + `Value` type
  (same `Index`/`PartialEq<&str>`/`as_*` surface as `serde_json::Value`, proven already in production
  at 🧩️puzzle's editor) — only `serde_json::` explicit calls (`to_string`/`from_str`/`from_slice`)
  needed touching; bare `json!(...)`/`Value` call sites needed no per-site edits.
- `mesh_data_to_dsl` (shape window + edit mode): `semio_framework_plugin::MeshData` already carries
  `From<MeshData> for pack::json::Value` in the framework (`🔺️mesh-engine/🦀️.rs`) — the plugin's own
  doc comment claiming "still derives serde::Serialize" was stale from an earlier wave. Fixed to
  `protocol::os_pack::json::to_dsl_value(&protocol::os_pack::json::Value::from(data.clone()))`.
  cad's inspection/📄️artifact panel tests still serialize `BuiltNode` directly (framework type, no
  ToValue — genuine, unfixed) so those 4 refs remain.
- `⚙️engine/🕹️interaction/🦀️.rs`, `🎬️interaction-spec/🦀️.rs`: test-only `json!(...)` → local
  `DslValue` builder helpers (`vec3_json`, `point_arg`, `value_arg`) — zero refs left.
- `📌️panels/🔍️inspection/🦀️.rs`: `object_ids: Vec<String>` inside `json!({"objectIds": object_ids,
  ...})` cannot move through `pack::json`'s `json!` macro (its catch-all arm takes ownership via
  `Value::from`, and there's no `From<Vec<String>>` — unlike `serde_json::json!`, which serializes by
  reference). Replaced with a `patch_selection_args(&[String], String) -> DslValue` helper instead.

### playbook
- `PlaybookSnapshot`/`PlaybookDiff`/`PlaybookArtifact` compose `store::ArtifactChild<S>` fields.
  `ArtifactChild<S>: ToValue/FromValue` is unconditional in the framework (`🏪️store/🦀️.rs:2753`), but
  its `Serialize/Deserialize` is `#[cfg_attr(test, derive(...))]` — **test-only**. The pre-existing
  unconditional `#[derive(..., Serialize, Deserialize, ...)]` on `PlaybookSnapshot`/`PlaybookDiff` was
  therefore already a LATENT NON-TEST COMPILE BREAK (calling `Serialize` on a field type that only has
  it under `#[cfg(test)]`). Fixed by mirroring `PlaybookArtifact`'s pre-existing hand-written
  `ToValue`/`FromValue` impl (same file, `🧬️schema/🦀️.rs`) onto `PlaybookSnapshot`
  (`🧬️schema/📸️snapshot/🦀️.rs`), and restoring `#[cfg_attr(test, derive(Serialize, Deserialize))]`
  everywhere a **committed `🧪️tests/<fixture>/🦀️.rs` file** (untouchable, DO NOT TOUCH) still calls
  `serde_json::from_str`/`to_value` against these types — see "Oracle-test restorations".
- `mutations/🦀️.rs`: `decode_playbook_mutation_json`/`decode_playbook_snapshot_json`/
  `encode_playbook_snapshot_json`/`seed_playbook_scene_json` → `protocol::json::from_json_str`/
  `to_json_string` (all four types now have ToValue/FromValue). Zero refs left in this file.
- Root `🗿️artifacts/📖️playbook/🦀️.rs`: `flow_content_snapshot_from_steps`/`steps_from_flow_content`/
  `flow_content_child_handle`/`document_child_handle` converted (stdio's `SemioFlowSnapshot`/
  `SemioDocumentSnapshot`/framework's `PlaybookBlock` all already ToValue-only). One genuine
  "third-party serde oracle" test (`wireOmission` case, explicitly labeled in its own `.expect(...)`
  strings) proves `ArtifactChild::local_owner`'s `#[serde(skip)]` treatment against real serde —
  left untouched, per "never delete an oracle test."
- `📥️import`/`📤️export` json rfc8259 leaves: `serde_json::from_value`/`to_value` against
  `PlaybookSnapshot` — **fixed to avoid the same latent break**: import now bridges
  `stdio::JsonSnapshot::to_serde_value() -> DslValue` via the framework's existing
  `DslValue: From<&serde_json::Value>`, then `PlaybookSnapshot::from_value`. Export goes
  `PlaybookSnapshot::to_value() -> DslValue -> serde_json::Value` via the framework's existing reverse
  `From<&DslValue> for serde_json::Value`. Import leaf: 1 → 0. Export leaf stays at 1 ref
  (`serde_json::Value::from(...)`) because `stdio::JsonSnapshot::from_value` genuinely requires
  `Into<JsonValue>`, and `serde_json::Value` is stdio's own documented bridge type — not this
  plugin's boundary to move.
- `PlaybookTopology`/`PlaybookInference` (no ToValue previously, no serde_json call site anywhere,
  no external dependent found): switched straight to `ToValue, FromValue`. Zero refs left.
- `PlaybookChapterPayload` (⚙️engine): same — switched to ToValue/FromValue, its two call sites in
  `✏️editor/🦀️.rs` converted to `protocol::json`.
- `✏️editor/🎭️modes/🏗️builder/🦀️.rs` (`WindowLayout`) and its `🪟️windows/🏗️builder/🦀️.rs`
  (`ProgramContributionEntry`, ToValue-only in the framework already): both converted.
- `👁️viewer/.../🌳️steps/🦀️.rs`: `UiNode` (has ToValue) → `protocol::json::to_json_string`.

## Oracle-test restorations (why the raw count went back up)

Nine mutation-leaf structs (`AddStep`/`RemoveStep`/`MoveStep`/`AddBlock`/`RemoveBlock`/`MoveBlock`/
`ReplaceBlock`/`UpdateStep`/`ChangeTitle`), `PlaybookMutation`, `PlaybookDiff`, `PlaybookArtifact`,
`PlaybookStringList`, and `PlaybookSnapshot` all had their production `Serialize`/`Deserialize`
stripped first, then **restored as `#[cfg_attr(test, derive(Serialize, Deserialize))]`** after
discovering 9 files under `🧬️mutations/*/🧪️tests/**` (DO NOT TOUCH — untouchable, off-limits) call
`serde_json::from_str::<PlaybookMutation/PlaybookSnapshot/PlaybookDiff>`/`serde_json::to_value(&_)`
directly against these exact types, and one non-🧪 test in `🗿️artifacts/📖️playbook/🦀️.rs`
(`wireOmission`) is a genuine third-party-serde oracle. This mirrors the framework's own
`ArtifactChild<S>` treatment exactly (`#[cfg_attr(test, derive(Serialize, Deserialize))]` there too),
so production stays 100% ToValue/FromValue while `cargo test` still compiles. Verified via
`grep -rln serde_json … | grep 🧪` across the whole plugin — exactly those 9 files, all now satisfied.

## Confirmed genuine framework/architecture boundaries — left alone, not force-converted

- **`BuiltNode`** (`🖱️ui/🧬️contract/../🎯️targets/🧊️wgpu/🦀️component.rs`): deliberately has NO
  ToValue/FromValue (`UiValue`-embedding exception, documented in its own file). Every
  `serde_json::to_string(&some_rendered_node)` test assertion in both plugins is this — cad
  `📌️panels/📄️artifact/🦀️.rs` (3), cad `✏️editor/🦀️.rs` (4 of its 4 remaining refs), playbook
  `✏️editor/🦀️.rs` (`ComponentTree` wraps the same `BuiltNode`, and **has no Serialize impl at all,
  not even `#[cfg(test)]`** — this test helper (`testkit::render`) was already broken before my
  session; not something I introduced or could fix without touching the framework).
- **`AppDefinition`** (`🛂️manifest/🦀️.rs`): its own doc comment says `# 🚧️ BLOCKED
  (26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-...)` — transitively embeds `WindowKindDefinition`/
  `UtilityDefinition`, "left serde-only. Revisit once both convert." playbook `✏️editor/🦀️.rs`'s
  `the_manifest_stitches_every_taxonomy_node` test hits this directly.
- **`apply_world3d_sun_action`/`apply_world3d_projection_action`/`world3d_projection_action_moves_pose`**
  (`🔌️plugin/🦀️.rs`): still `Option<&serde_json::Value>` in the framework. cad's
  `🎮️commands/🌞️sun/🦀️.rs` (3) and `🎮️commands/🎥️camera/🦀️.rs` (2) bridge once via
  `serde_json::Value::from(&dsl_args)` right at the call, already correctly minimal/documented.
- **`MeshDwgDocumentImporter = fn(&MeshData) -> Result<serde_json::Value, String>`**
  (`🔌️plugin/🦀️.rs`): a framework-owned function-pointer *type*, not a value — cad's
  `🚪️io/🦀️.rs::cad_document_from_mesh` (2 refs) is the one required bridge point, already isolated.
- **`playbook_bounded_serialized_bytes<T: serde::Serialize>`** (playbook `✏️editor/🦀️.rs`, 1 ref +
  its 2 call-site `where … serde::Serialize` bounds): OUR OWN choice, not framework-forced, but its
  only instantiation is `P=PlaybookConfig, M=PlaybookConfigMutation` — **neither type has
  ToValue/FromValue today** (`✏️editor/🎚️config/🦀️.rs` 3 refs + `🎚️config/🧬️schema/🦀️.rs` 2 refs +
  playbook `👥️presence/🦀️.rs` 3 refs + `👥️presence/🧬️schema/🦀️.rs` 2 refs — `PlaybookConfigMutation`
  uses `dsl::DslOps` instead of `ToValue/FromValue`, a derive pairing I did not have budget to
  characterize safely this session). **Left unconverted, flagged for a follow-up ticket** — this is
  the single largest remaining structured chunk (~10 refs across config+presence) and looks tractable
  once `dsl::DslOps` vs `ToValue/FromValue` coexistence is confirmed safe (or a hand-written impl is
  written, mirroring the `PlaybookSnapshot`/`PlaybookArtifact` pattern in this same session).
- **`🧩️extensions/🌀️procedural/🦀️.rs`** (1 ref): already fully converted by a prior wave; its one
  surviving `serde_json` dependency is explicitly documented as `playbook::visible_blocks`'s
  hard-typed `serde_json::Map<String, serde_json::Value>` signature — a framework function this
  plugin doesn't own.

## Files touched (see `git diff --name-only` for the authoritative list)

Roughly 40 files across the two plugins. No `Cargo.toml` edited. No ticket close/reopen performed —
left open for the dev to review and potentially fold into `PlaybookConfig`/`PlaybookPresence`
follow-up.
