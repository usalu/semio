# Wave S — Render-time selection/hover reads + app-initiated selection writes (2026-09-09)

Scope: tasks 1–6 of the W-S brief (render-time interaction reads, click-to-select binding, `Emit`
selection-write channel, presence verification, tests, this report). All claims below are grounded in a
run whose exact command and tail are quoted; anything unverified is called out in
[§9 Not verified](#9-not-verified).

---

## 1. TL;DR

- **Render-time reads are live.** `Puzzle3dPlayApp` now overrides `ArtifactEditor::render_with_request_context`
  and threads one `Puzzle3dInteractionSnapshot` (selection + `"pointer"` hover of the framework-owned
  `vortex` domain) into a single `render_body`. `gumball_active`, `world_selection_json`,
  `world_vortices_json`, `world_interaction_json`, `world_brush_preview_json`,
  `puzzle3d_brush_target_vortex`, `object_vortices_visible` and the inspection panel all read it.
- **Click-to-select is bound.** `scene.domain_id = Some("vortex")` **plus**
  `scene.domain_granularity_id = Some("object")` — the granularity is mandatory, because the host's own
  default is `"handle"`, which puzzle3d does not declare and `validate_state` would prune on every pick.
- **`Emit.interaction_writes` exists and actually reaches puzzle3d.** The brief assumed `dispatch_emit`
  was puzzle3d's publication path; it is **not** — every puzzle3d action is a retained tool job whose
  `Emit` is drained by `publish_mounted_typed_operation_unit`. Both paths are wired (see [§3](#3-framework-additions-exact-signatures)).
- **Two audit-visible corrections to the brief's design:** `selectionJson` gets **no** `vortexIds` field
  (the host's `WorldSelectionRecord` has none — vortex marks travel on `vorticesJson.selected/.hovered`),
  and `window_measures` needed its own additive interaction-aware variant or the Brush "Placement" picker
  could not be fixed at all.
- **Presence needed no fields** — only a corrected docstring. Verified: `PresencePeer.interaction` is
  assembled generically.
- **`cargo check` is green (0 errors) for the lib and for `--tests`.** `cargo test` **cannot be used as a
  gate in this crate right now** — the test binary aborts on two pre-existing faults, proven not to be
  this wave's by an A/B probe. Details and evidence in [§7](#7-test-runs).

---

## 2. Files changed

### Framework (additive only)
| File | Change |
| --- | --- |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` | `InteractionWrite` + `Emit.interaction_writes`; `apply_interaction_writes`; 3 apply sites in `dispatch_emit`; `ArtifactToolPublicationLane::Interaction`; `TypedOperationResultLane::Interaction`; typed-operation interaction publication unit; `window_measures_with_request_context` + `context_menu_with_request_context` on `ArtifactApp`/`ArtifactEditor`/`ArtifactViewer` + both adapters + both wrapper call sites; root re-export of `InteractionView`/`InteractionWrite` |
| `🧰️framework/🔨️modules/🖱️ui/🧬️schema/🔣️.json` | `retainedCommandLane` enum gains `"interaction"` (the schema's own `$comment` requires one member per runtime enum variant) |

### puzzle3d
| File | Change |
| --- | --- |
| `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` | `Puzzle3dInteractionSnapshot` (new `🔖️InteractionSnapshot` region); `PUZZLE3D_HOVER_CHANNEL`; `puzzle3d_brush_target_vortex` implemented; `Puzzle3dActionCtx.interaction`/`.interaction_writes`/`replace_selection`; `handle_action_impl` takes the snapshot and returns `interaction_writes`; `render_with_request_context`/`window_measures_with_request_context`/`context_menu_with_request_context` overrides + `render_body`/`window_measures_body`/`context_menu_body`; `Puzzle3dContextSelection::fill_from_interaction`; `PUBLICATION_CONTRACTS` for 4 routes; `extent`'s unused `command` warning fixed |
| `…/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️main/🦀️.rs` | `gumball_active`, `world_selection_json`, `world_vortices_json`, `world_interaction_json`, `world_brush_preview_json`, `object_vortices_visible`, `window_measures`, `render` all take the snapshot; `hovered_kind_id`; `scene.domain_id`/`domain_granularity_id` bound |
| `…/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️main/🪛️utilities/🖌️brush/🦀️.rs` | `options(..)` takes the snapshot; Placement picker reachable |
| `…/✏️editor/📌️panels/🔍️inspection/🦀️.rs` | rewritten: per-granularity field groups (object/vortex/attraction/targetVolume/reference) with `patchInspector` `hidden`/`locked` rows; document summary as the empty state |
| `…/✏️editor/🎮️commands/🧬️select-same-kind/🦀️.rs` | emits a real replace-select of every same-kind object |
| `…/✏️editor/🎮️commands/👯️duplicate-selection/🦀️.rs` | re-selects the clones |
| `…/✏️editor/🎮️commands/✅️accept-suggestion/🦀️.rs` | re-selects the placed object; hover fallback restored |
| `…/✏️editor/🎮️commands/🖌️add-brush-object/🦀️.rs` | re-selects the placed object |
| `…/✏️editor/🎮️commands/🔁️cycle-candidate/🦀️.rs` | new `puzzle3d_brush_target_vortex` arity |
| `…/✏️editor/👥️presence/🦀️.rs` | docstring corrected (no field change — see [§5](#5-presence)) |
| `…/✏️editor/🧪️tests/🔬️unit/🦀️.rs` | 4 stale known-gap tests rewritten, 8 tests added, 4 call sites re-arity'd |
| `✏️s/🔌️plugins/🧩️puzzle/🔏️publication-authority/🔣️.json` | 4 routes moved into two new `interaction`-lane groups |
| `✏️s/🔌️plugins/🧩️puzzle/🧬️schema/🔣️.json` | fixture lane enum gains `"interaction"` (Ajv gate) |
| `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/📜️script.ts` | lane union type + `ArtifactToolPublicationLane::(…|Interaction|…)` regex |

### Deliberate exception — two lines in `⏳️precompute/**` (W-F's area)
The brief said not to touch `⏳️precompute/**`. The crate's **lib-test target did not compile at HEAD**
for reasons unrelated to either wave, which made it impossible to run *any* test in this crate. Two
strictly-additive lines were changed to unblock compilation; both are visibility/import only, no logic:

- `…/✏️editor/⏳️precompute/🦀️.rs:44-45` — `const` → `pub(crate) const` for `FILL_ENVELOPE_MAX_BYTES` and
  `FILL_ENVELOPE_MAX_ITEMS`. Its own test module (`🪣️fill/🧪️tests/🔬️unit/🦀️.rs:4`) imports them, but
  `precompute` re-exports `component::*`, so a private const in `component` is **not** reachable as
  `precompute::X` from the sibling `precompute::fill` subtree.
- `…/✏️editor/⏳️precompute/🪣️fill/🧪️tests/🔬️unit/🦀️.rs:3` — added `DOCUMENT_CELL_SLOTS` to the existing
  `precompute::geometry::{…}` import (used at `:1097` and `:1104`).

Proof this was pre-existing, not W-F's uncommitted edit and not mine:
`git show HEAD:…/⏳️precompute/🦀️.rs | grep -n 'FILL_ENVELOPE_MAX_BYTES: usize'` → `44:const FILL_ENVELOPE_MAX_BYTES…`
(private at HEAD), and `git diff --stat` showed the fill test file itself unmodified. **Coordinator: hand
these two lines to W-F.**

---

## 3. Framework additions (exact signatures)

```rust
// 🕹️ One app-initiated selection write riding alongside an ordinary action's own mutations.
#[derive(Clone, Debug, PartialEq)]
pub struct InteractionWrite {
    pub domain: String,
    pub targets: Vec<protocol::InteractionTarget>,
    pub merge: protocol::MergeMode,
}
impl InteractionWrite {
    pub fn replace(domain: impl Into<String>, granularity: &str, ids: impl IntoIterator<Item = String>) -> Self;
}

// on Emit<Mutation, ConfigMutation, DraftMutation>, next to `child_emits`, defaults to `Vec::new()`
pub interaction_writes: Vec<InteractionWrite>,

// VcsArtifactApp<A>
async fn apply_interaction_writes(&mut self, writes: &[InteractionWrite], meta: &ActionMeta) -> Result<(), Fault>;

// VcsArtifactApp<A> — retained-tool twin of the above
fn mounted_typed_interaction_writes_are_next(mounted: &MountedTypedCommandFullOperation<A>) -> bool;
async fn publish_mounted_typed_interaction_unit(&mut self, mounted: &mut MountedTypedCommandFullOperation<A>) -> Result<(), Fault>;

// ArtifactToolPublicationLane
Interaction,          // new variant, after `Child`
// TypedOperationResultLane
Interaction,          // new variant, after `Child`

// ArtifactApp (async), ArtifactEditor / ArtifactViewer (sync) — all three default to the plain method
async fn window_measures_with_request_context(
    doc: &ArtifactView<'_, Self::Snapshot>,
    cfg: &ConfigView<'_, Self::Config>,
    view_state: &ViewModel,
    interaction: &InteractionView<'_>,
) -> HashMap<String, Vec<WindowMeasure>>;

async fn context_menu_with_request_context(
    request: &ContextMenuRequest,
    doc: &ArtifactView<'_, Self::Snapshot>,
    cfg: &ConfigView<'_, Self::Config>,
    view_state: &ViewModel,
    interaction: &InteractionView<'_>,
    registry: &AppActionRegistry,
) -> Vec<ContextMenuItemSpec>;
```

Wrapper call sites switched (one each): `VcsArtifactApp::window_measures` and
`VcsArtifactApp::context_menu` now build the same `InteractionView` `render` already built
(`interaction_state()` + `interaction_hover` + `peer_presence`) and call the `_with_request_context`
variant. **Blast radius: 0 forced plugin changes** — verified by compiling `semio-framework-plugin`,
`semio-s-artifact-process-process3d` and `semio-s-artifact-block-3d` clean ([§7](#7-test-runs)).

### 3.1 Why `dispatch_emit` alone was not enough (correction to the brief)

The brief specified applying `interaction_writes` in `VcsArtifactApp::dispatch_emit`. Every puzzle3d
action is a **retained tool job** (`Puzzle3dRetainedCommandJobFactory`,
`InteractiveJobClassification::Migrated`) whose `Emit` is drained field-by-field by the *sync*
`publish_mounted_typed_operation_unit` — it never reaches `dispatch_emit`. Writes are therefore applied
on both paths:

1. **`dispatch_emit`** — `apply_interaction_writes` runs after the mutation lanes land, at all three exits
   (`child_emits` group path, artifact-mutations-empty path, and the ordinary path right after
   `revalidate_interaction_state_after_document_change`). Serves `import_media`, `task-resume-emit`, the
   framework clipboard routes, and any app that is not on the retained path.
2. **Retained tool jobs** — `advance_typed_operation_publication_one` (async) branches to
   `publish_mounted_typed_interaction_unit` when `mounted_typed_interaction_writes_are_next`, i.e. once
   every durable lane of that emit is drained and writes remain. Ordering is therefore
   artifact → config → draft → presence → transient → child → **interaction** → effect → event → ui →
   terminal, which is what "after the action's own mutations" has to mean.
   No new field on `MountedTypedCommandFullOperation` — the writes stay in the retained emit until taken.
   That was deliberate: three framework test files construct that struct as a literal, and a new field
   would have forced edits into them.

### 3.2 Publication lane — decision

`interaction_writes` **does** need a lane: the retained publication gate refuses any store lane absent
from a factory's exact `PUBLICATION_CONTRACTS`, and `interaction_store` is a real durable
(persisted-local, `HistoryLane::Interaction`) store — unlike `effects`/`events`, which are host outboxes.
So `ArtifactToolPublicationLane::Interaction` was added, gated in the same boolean chain, and declared on
the four puzzle3d routes that now write:

| route | lanes before | lanes after |
| --- | --- | --- |
| `selectSameKindSelection` | `config` | `config`, `interaction` |
| `duplicateSelection` | `artifact`, `config` | `artifact`, `config`, `interaction` |
| `acceptSuggestion` | `artifact`, `config` | `artifact`, `config`, `interaction` |
| `addBrushObject` | `artifact`, `config` | `artifact`, `config`, `interaction` |

Three fixtures/schemas had to learn the spelling for the audit to pass: the framework `framework.ui`
lane enum, puzzle's own `🧬️schema/🔣️.json` lane enum (Ajv), and the `📜️script.ts` regex + union type.

---

## 4. Render-time reads — what each consumer now sees

`Puzzle3dInteractionSnapshot { granularity, selected, hovered }` mirrors `Puzzle3dActionCtx`'s grouping
exactly (a `DomainSelection` carries one live granularity, so `selected_object_ids()` and friends return
`&[]` unless the granularity matches). Hover ids arrive **without** a granularity
(`protocol::DomainHover` has none), so `hovered_object_id`/`hovered_vortex_full_id`/`hovered_reference_id`
resolve against the live fixture rather than guessing from id spelling.

| consumer | before | now |
| --- | --- | --- |
| `gumball_active` | hardcoded `false` | transform utility **and** ≥1 handle flag (`transform_move`/`transform_rotate`) **and** ≥1 selected object or target volume |
| `world_selection_json` | `ids: []`, `vortexIds: []`, `targetVolumeIds: []`, no hover | `ids` (objects), `activeObjectId`, `targetVolumeIds`, `referenceSelectedId`, `hoveredId` (object, or `reference:{id}`), `hoveredKindId`, `gumballActive` |
| `world_vortices_json` | no marks | per-record `selected`/`hovered` |
| `world_interaction_json` | no hover | `hoveredVortexFullId` |
| `object_vortices_visible` | `Always` only | `Always`, or `Selected` when the object or one of its markers is selected/hovered |
| `puzzle3d_brush_target_vortex` | `None` | selected vortex → hovered vortex → first vortex of hovered object |
| `world_brush_preview_json` | suggestion-menu target only | suggestion-menu target, else the live brush target |
| `brush::options` | picker unreachable | picker renders for a live brush target |
| `panels::inspection::render` | static document summary | per-granularity field group, summary as empty state |
| `context_menu` | client `surface.selection` only (object + feature) | plus the authoritative domain read for vortex/attraction/targetVolume/reference (surface hits keep priority) |

### 4.1 `vortexIds` is not a host field (correction to the brief)

The brief asked for `vortexIds` in `selectionJson`. `World3dHost/🟦️.tsx`'s `WorldSelectionRecord`
(`:174-197`) has **no** vortex field, and `grep -n vortexIds` in that file finds only
`:4200`, an action-arg literal for the legacy `setSelection` verb. Vortex selection/hover paint is read
off each `WorldVortexRecord` (`:246-257` `selected`/`hovered`; consumed at `:2743-2745`
`vortex.selected ? palette.selected : vortex.hovered ? palette.hovered : null`). So the marks were put
on `vorticesJson`, and `world_selection_json`'s doc comment records why the field is absent. A test
asserts `selection.get("vortexIds").is_none()` so this cannot silently regress.

### 4.2 `window_measures` needed its own variant (extension of the brief)

The brief listed the Brush "Placement" picker under task 1, but that picker is a `WindowMeasure`, built by
`main::window_measures` → `brush::options`, reached through `ArtifactApp::window_measures` — a trait method
with **no** interaction parameter and no `_with_request_context` twin anywhere in the framework (grepped).
Without an additive variant the picker was unfixable, so `window_measures_with_request_context` was added
in the same shape as `render_with_request_context`. `window_engagements` (object/attraction counts + a
command input) and `tool_measures` (fill count/voxel/distribution) read no selection and were left alone.

---

## 5. Presence

Verified — **no fields added**, docstring corrected instead:

- `protocol::PresencePeer.interaction: Option<protocol::PresenceInteraction>` (`🔌️plugin/🦀️.rs:7585`).
- Populated generically by the heartbeat: `protocol::assemble_presence_interaction(self.app_id().await,
  &interaction_state, &hover_specs, &selection_specs)` then `encode_presence_interaction`
  (`:22213-22221`) — zero per-app code.
- Read back generically: `InteractionView::peers_selecting`/`peers_hovering` walk
  `presence.interaction.domains[].selected/.hovered` (`:8228`, `:8242`).

`Puzzle3dPresence`'s doc comment claimed it was the "shareable live subset … (selection, hover, …)" while
the struct carries only camera + active utility/tool. It now states that selection/hover are deliberately
absent because they are framework-owned and already broadcast on wire bit 7, and that an app-owned copy
would be a second authority over the same state.

---

## 6. Click-to-select — verification of the legacy verbs

- `grep -rn "worldVortexSelect|worldVortexHover|worldPick|worldSelect\b|setHover"` across
  `✏️s/🔌️plugins/🧩️puzzle` (excluding `🗄️stdio`) returns **only doc comments and test prose** — no handler.
  The stale comment claiming "bespoke vortex-fit pick logic elsewhere in this crate" was deleted.
- Host instance picks: `handleInstancePointerDown` (`World3dHost/🟦️.tsx:~4386`) dispatches
  `interactionSelect` via `world3dSelectionActionArgs(interactionDomainId, interactionGranularity, …)`
  when `scene.domainId` is set; likewise `dispatchInstanceHover` → `interactionHover`, marquee
  (`:~4816`) and background-clear (`:~4906`). All four now take the domain path.
- Granularity: `interactionGranularity = scene?.domainGranularityId ?? WORLD3D_DEFAULT_INTERACTION_GRANULARITY`
  where the default is `"handle"` (`:3832`) — **not** one of puzzle3d's six granularities, so leaving
  `domain_granularity_id` unset would have every pick pruned by `validate_state`. Hence
  `scene.domain_granularity_id = Some(PUZZLE3D_GRANULARITY_OBJECT)`.
- `interaction_topology` already emits every granularity the host can send (`object` for instances,
  `vortex`/`attraction`/`targetVolume`/`reference`/`kind` for the tree/catalogue rows) — unchanged.
- **Open host-side gap (not fixed, out of this wave's remit):** `handleVortexSelect`/`dispatchVortexHover`
  (`World3dHost/🟦️.tsx:4416-4455`) dispatch `worldVortexSelect`/`worldVortexHover` **unconditionally** —
  they are not gated on `interactionDomainId` the way instance picks are. So clicking a vortex MARKER in
  the viewport still dispatches a verb nothing handles. Selecting a vortex from the document/catalogue
  tree (which uses `puzzle3d_interaction_select`) works, and everything downstream of a vortex selection
  (Placement picker, brush target, `Selected` marker mode, inspection panel) is now correct once the
  selection exists. Fixing the marker path means routing those two host callbacks through
  `world3dSelectionActionArgs(domainId, "vortex", …)` / `world3dHoverActionArgs(domainId, "vortex", …)`
  — a `World3dHost` change affecting every `domainId`-bound world plugin, flagged rather than taken.

---

## 7. Test runs

Every command below was run from the repo root with
`CARGO_TARGET_DIR=…/scratchpad/target-p3d RUSTC_WRAPPER=""`.

### 7.1 `cargo check` (lib) — 0 errors

```
cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly -j 4 --message-format=short
…
warning: `semio-s-artifact-puzzle-3d` (lib) generated 44 warnings (run `cargo fix …`)
    Finished `dev` profile [unoptimized] target(s) in 49.80s
```

Zero errors anywhere, `⏳️precompute/` included — the two pre-existing E0308s the brief warned about at
`⏳️precompute/🪣️fill/🦀️.rs:2614` were gone by the time this wave built. Remaining warnings are all
outside this wave: `🗣️terminology/🦀️.rs:5` unused `Terminology`, `👁️viewer/🦀️.rs:65` unused `view_state`,
~11 dead-code warnings in `⏳️precompute/`, and ~30 `unnecessary qualification` warnings from a peer's
import-unqualify sweep. The one warning the brief asked for — `unused variable: command` in
`Puzzle3dScalarConfigWork::extent` — is fixed (`_command`, with a docstring saying why the extent is
command-independent).

### 7.2 `cargo check --tests` — 0 errors

```
cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly --tests -j 4 --message-format=short
…
warning: `semio-s-artifact-puzzle-3d` (lib test) generated 36 warnings
    Finished `dev` profile [unoptimized] target(s) in 49.93s
```

(Green only after the two `⏳️precompute` visibility/import lines in [§2](#2-files-changed); before them,
3 pre-existing E0432/E0425 errors in `⏳️precompute/🪣️fill/🧪️tests/🔬️unit/🦀️.rs` blocked the whole target.)

### 7.3 Blast-radius checks — 0 errors

```
cargo check -p semio-framework-plugin -j 4                                    → Finished in 1m59s
cargo check -p semio-s-artifact-process-process3d -p semio-s-artifact-block-3d → Finished in 1m10s
```

`cargo check -p semio-framework-plugin --tests` fails with **one** error —
`📡️backbone/🔗️binding/🧪️tests/🔬️unit-standalone/🦀️.rs:31: no method named to_value on
DocumentBackboneBindingWireV1`. That is a **peer's uncommitted change**: `git diff --stat` shows 4
modified files under `🔌️plugin/📡️backbone/🔗️binding/` (schema, fixture, `🦀️.rs`, that test) which this
wave never touched.

### 7.4 `cargo test --lib` — **could not be used as a gate** ⚠️

```
cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib -j 4
running 561 tests
… 12 ok, 3 FAILED …
thread '…::context_menu_at_selects_object_groups_flags_and_keeps_delete_last' has overflowed its stack
fatal runtime error: stack overflow, aborting
```

The binary aborts, so no suite summary is reachable. Two independent **pre-existing** faults:

**(a) registry-less `testkit::app()` is unusable for this plugin.** With `RUST_MIN_STACK=134217728`, a
pure-chrome test (`fill_and_brush_params_are_tagged_utility_options_not_engagement_controls`, which builds
`app()`) panics inside `testkit::new_app` at `🔌️plugin/🦀️.rs:16611`:

```
tool proof catalog must exactly join migrated generated declarations to live concrete factories:
Fault { code: FaultCode("interactive-job.catalog-authority"),
  message: "tool factory proof rejected tool 'openAddObjectDialog': … generated_migrated=false … migrated={}" }
```

i.e. the bare `new_app` has no migrated declarations, so a plugin declaring
`bounded_first_step_tool_proofs!` must use `new_app_with_registry`. Nothing in this wave touches app
construction.

**(b) `resolve_ready` non-ready first poll on the reserved interaction verbs.** Every
`app_with_registry()` test that dispatches `interactionSelect`/`clearSelection` panics at
`🧰️framework/🔨️modules/🚪️io/🦀️.rs:898`:

```
resolve_ready: future was not ready on first poll — io-async-signatures requires every artifact-IO body
to complete without real suspension
stack backtrace:
   2: semio_framework::io::resolve_ready::<VcsArtifactApp<EditorApp<Puzzle3dPlayApp>>::handle_action::{closure#0}>
   3: …::testkit::dispatch
   4: …::testkit::select_id
   5: …::tests::world_pick_null_clears_without_reselecting_first_object::{closure#0}
```

`ArtifactStoreCursorDisposer::drop` then panics with *"artifact store cursor disposer reached Drop before
terminal-empty ownership"* during unwinding, which turns it into a non-unwinding `SIGABRT` and hides the
suite summary (a known trap in this repo).

**Proof (b) is not this wave's:** an A/B probe replaced all three
`self.apply_interaction_writes(&interaction_writes, meta).await?;` call sites with
`let _ = &interaction_writes;`, rebuilt, and re-ran
`world_pick_null_clears_without_reselecting_first_object` (a test this wave never edited):
`grep -c "resolve_ready: future was not ready"` → `1`, i.e. **identical failure**. The probe was then
reverted (verified: 3 call sites restored). Independently, `dispatch_interaction_action` is reached from
`handle_action` **without** passing through `dispatch_emit`, and `git diff -U0` on
`🔌️plugin/🦀️.rs` shows every uncommitted hunk in that file belongs to this wave and none of them sit on
the reserved-verb path.

**Also failing, also pre-existing:** `add_brush_object_hostile_static_law_rejects_engine_run_to_completion`
and `add_object_kind_hostile_static_law_rejects_whole_catalog_conversion`. Their hostile-mutation step is
`source.replacen(marker, "cursor-removed", 1)`, which cannot flip the predicate when a marker occurs
twice. Verified identical at HEAD:

```
grep -c 'Puzzle3dAddBrushObjectStage::Decode'   worktree → 2   git show HEAD:… → 2
grep -c 'PUZZLE3D_RELOCATE_VORTICES_PER_OBJECT' worktree → 14  git show HEAD:… → 14
```

`command_envelope_round_trip_holds_for_an_applied_operation` also FAILED; not investigated (uses `app()`,
so most likely fault (a)).

**Consequence, stated plainly: the 8 tests added and the 4 rewritten by this wave COMPILE but were NOT
executed.** They are listed in [§8](#8-tests-added-and-rewritten) so whoever unblocks the suite can run
them first.

### 7.5 `bun ✏️s/…/🧩️puzzle/📦️packages/🟦️typescript/📜️script.ts publication-authority-audit Puzzle3dPlayApp` — **green**

```
validated Puzzle publication authority; owners=Puzzle3dPlayApp;
admitted=openAddObjectDialog,transformBegin,…,selectSameKindSelection,acceptSuggestion,addBrushObject,duplicateSelection;
schema=Ajv; oracle=independent
```

The brief said this audit was red; it passes for `Puzzle3dPlayApp` after the lane additions. It first
failed on Ajv (`allowedValues: [artifact, config, draft, presence, transient, child, host-only]`), which
is what surfaced the puzzle-local lane enum in `✏️s/🔌️plugins/🧩️puzzle/🧬️schema/🔣️.json`.

### 7.6 `bun ./📜️script.ts verify interactivity` — **red, for a reason outside this wave**

```
error: [verify interactivity] Puzzle fill envelope baseline was falsely rejected:
Puzzle fill admission does not advance one fixed nested allocation/entry backed by the exact credited
slot pages before reservation
    at interactivityPuzzleFillEnvelopeSelfTests (✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🔬️interactivity-puzzle-fill-envelope/🟦️.ts:77)
```

`interactivityPuzzleFillEnvelopeFailures(precompute, fill, geometry, action)` reads only
`⏳️precompute/🦀️.rs`, `⏳️precompute/🪣️fill/🦀️.rs`, `⏳️precompute/📐️geometry/🦀️.rs` and
`🎮️commands/🪣️fill-build-tick/🦀️.rs` — none of which this wave edited (the two-line visibility change
touches neither literal). The run also reports pre-existing `.vscode/🧩️launch.seed.jsonc` gate-registration
findings and 757 all-app discovery failures, all from other peers' in-flight work.

The checks in that audit that *do* read a file this wave edited were verified by hand to still match
byte-for-byte (`interactivityPuzzleFillPreviewJsonFailures`' three literals on
`🪟️windows/🧊️main/🦀️.rs`):

```
grep -c 'session.fill_preview_json_page(&color, labels.fill_progress.as_str())'                        → 1
grep -c 'world_fill_preview_json(precompute, envelope, labels).or_else(|| world_brush_preview_json'    → 1
grep -c 'serde_json::to_value(build)'                                                                  → 0   (must be absent)
```

---

## 8. Tests added and rewritten

Written and compiling; **not executed** (see [§7.4](#74-cargo-test---lib--could-not-be-used-as-a-gate-)).
All in `…/✏️editor/🧪️tests/🔬️unit/🦀️.rs`.

Added:
1. `gumball_inactive_when_every_handle_flag_is_off`
2. `world_selection_json_carries_the_host_field_names_per_granularity` (incl. `vortexIds` absence)
3. `world_scene_binds_the_vortex_interaction_domain_and_its_granularity`
4. `world_vortices_carry_their_own_selected_and_hovered_flags` (incl. `hoveredVortexFullId`)
5. `select_same_kind_widens_the_selection_to_every_object_of_that_kind` (asserts both
   `interaction_state()` and the painted `selectionJson.ids`)
6. `select_same_kind_with_no_selection_leaves_the_selection_untouched`
7. `duplicate_selection_reselects_the_created_clones`
8. `brush_placement_picker_appears_only_for_a_live_brush_target` (drives the real
   `app.window_measures(..)`, i.e. the `_with_request_context` seam)
9. `selected_vortex_inspector_renders_the_vortex_field_group`

Rewritten from "known gap / degraded floor" to real behaviour:
- `gumball_active_only_for_transform_utilities_with_object_selection` — now asserts `gumballActive: true`
- `world_vortices_stay_hidden_in_selected_mode_pending_the_render_interaction_gap` →
  `world_vortices_reveal_in_selected_mode_only_for_the_selected_object`
- `selected_object_inspector_nests_origin_into_x_y_z_steppers` →
  `selected_object_inspector_renders_that_object_field_group`
- `world_pick_keeps_instances_geometry_json_stable` — now also asserts `selectionJson.ids`
- `transform_utility_is_local_to_the_window_instance_…` — doc comment de-staled

Re-arity'd call sites: 4 `main::window_measures(…)` calls now pass
`&Puzzle3dInteractionSnapshot::default()`.

---

## 9. Not verified

1. **No test in this crate was executed.** Both faults blocking the suite are pre-existing and proven not
   to be this wave's, but the consequence stands: nothing here is runtime-confirmed. Highest-priority
   follow-up.
2. **No runtime/browser confirmation.** No `dev` boot, no console log, no screenshot. The gumball actually
   appearing, an instance highlight painting, the Placement picker showing, and the inspection panel
   switching are all *derived from* the host source that consumes these fields — not observed.
3. **Vortex-marker click is still dead** end-to-end, by the host gap in [§6](#6-click-to-select--verification-of-the-legacy-verbs).
   Vortex selection from the tree/catalogue works; from the 3D marker it does not.
4. **Two-peer presence smoke test not performed** (the 26/08/14 ticket's own open item). The mechanism was
   verified by reading the assembly/read-back code, not by running two clients.
5. **`Emit.interaction_writes` on the `dispatch_emit` path is untested at runtime.** puzzle3d exercises
   only the retained-tool path; the `dispatch_emit` sites (`import_media`, `task-resume-emit`, clipboard
   routes) have no app declaring writes today.
6. **`ensure_reserved_emit_bounded` does not count `interaction_writes`** toward
   `max_output_bytes`. It counts artifact/config/draft mutations and effects. Only the framework clipboard
   routes (`copy`/`cut`/`paste`) go through it and none of them writes selection, so nothing is unbounded
   today — but a future reserved route that does would escape the cap.
7. **`puzzle5d` / `puzzle2d` / `block3d` / `block5d` / `block2d` were not migrated.** They have the same
   `scene.domain_id = None` + plain-`render` shape. The pattern is now proven compilable on puzzle3d and
   the framework seams are in place; the mechanical follow-up is per-crate.
8. **The reducer-side hover fallback for `accept_suggestion` is thinner than render's.**
   `handle_action_impl` receives a full snapshot including hover on the retained path
   (`Puzzle3dInteractionSnapshot::from_state(interaction, hover)`), so this is wired — but it was not
   exercised, and the `Puzzle3dActionCtx` hover fields are only reachable through
   `puzzle3d_brush_target_vortex`, not through named `hovered_*` ctx helpers.
9. **`interaction_writes` are not lane-gated on the `dispatch_emit` path** (only on the retained
   publication path, which is where the gate lives). Consistent with how `child_emits` behaves there.
10. **`verify interactivity` was not brought green** — it is red on `⏳️precompute` literals owned by W-F.
    Only the three literals it reads from a file this wave edited were hand-verified.
