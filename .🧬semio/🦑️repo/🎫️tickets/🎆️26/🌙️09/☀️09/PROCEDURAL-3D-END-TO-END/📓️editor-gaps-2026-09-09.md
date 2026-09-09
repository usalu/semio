# ✏️ Generation3d editor — the four audited gaps, closed (2026-09-09)

Packet: `✏️editor/**` of `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any`,
limited to the four items the coordinator briefed: (a) the blind context menu, (b) the two no-op
pointer-down routes, (c) the seven provisional config mutation leaf descriptors, (d) the
`👥️set-contributions` command directory. The tessellation/preview functions in the same
`✏️editor/🦀️.rs` are another lane's and were not touched.

## 0. TL;DR

| gap | before | after |
|---|---|---|
| (a) context menu | `let selected: Vec<String> = Vec::new();` hardcoded — transform trio, removal group and the destructive delete row could never appear | `context_menu_with_request_context` implemented; the menu reads the **framework-owned `graph` selection** through `PreviewInteractionMarks::from_interaction`, split into the `node`/`edge` domains a menu addresses. Proven by a live-dispatch test. |
| (b) `worldPointerDown` / `graphPointerDown` | `Ok(Emit::default())` no-ops, both declared `Migrated`, both retained tool ids, both with `HostOnly` publication contracts and bounded execution contracts | **deleted** — payload dirs, enum rows, action definitions, classifications, tool ids, contracts, `command_from_action` arms and module mounts. Selection is framework-injected on the declared `graph` domain; neither route is reachable in this repo (evidence in §2). A regression test forbids their return. |
| (c) 7 config leaf descriptors | hand-written `DESCRIPTORS` with `⚠️ PROVISIONAL` owners naming directories that did not exist | `dsl::Mutations`-derived over **7 authored leaf directories**, each with `🦀️.rs` + `🔣️.json` + `🧬️schema/🔣️.json`, per-leaf real `inverse`, per-leaf text opcode and binary tag. Owner-directory-exists test added, exactly like the viewer's. |
| (d) `👥️set-contributions/` | empty directory | already removed by the viewer lane (`📓️viewer-2026-09-09.md` §2.7); re-verified — no `setContributions` reference survives anywhere under `🌀️procedural` outside generated `🔣️.json` mirrors. |

---

## 1. (a) The context menu now reads the authoritative selection

### 1.1 What the framework actually offers

The `ArtifactEditor`/`ArtifactApp` trait already carries a context-carrying variant; the audit's
premise ("`context_menu` carries no `InteractionView`") was true of the method the app implemented,
not of the trait:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:10937` — `async fn context_menu_with_request_context(request, doc, cfg, view_state, interaction, registry)`, defaulting to `context_menu` and documented as *"`context_menu`'s interaction-aware twin. `ContextMenuRequest.surface.selection` is CLIENT-supplied and only ever carries what that surface itself painted … `interaction` is the authoritative framework-owned selection for every declared domain."*
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:24851-24866` — `VcsArtifactApp::context_menu` is what the runtime calls, and it builds an `InteractionView` from the live interaction store + ephemeral hover + peer presence and calls **`A::context_menu_with_request_context`**, never `context_menu` directly.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:6925-6934` / `:26114` / `:26439` — the `EditorApp`/`ViewerApp` forwarders already pass it through.

So no framework change was needed. Repo-wide there was exactly **one** implementor before this
packet — `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/…/✏️editor/🦀️.rs:7259` — and its shape (one
`context_menu_body`, two thin delegates) is the pattern this packet copied.

### 1.2 Design

Three edits in `✏️editor/🦀️.rs`:

- `:1415-1426` — `context_menu` becomes a delegate against `PreviewInteractionMarks::default()`
  (the marks-free entry point the framework still offers).
- `:1430-1445` — **new** `context_menu_with_request_context`, delegating with
  `PreviewInteractionMarks::from_interaction(interaction)` — the same resolver
  `render_with_request_context` (`:1390`) already used, so the menu and the painted geometry agree
  by construction rather than by convention.
- `:1449-1483` — `Generation3dPlayApp::context_menu_body`, the ONE implementation. Unchanged
  disclosure shape (`reorganize` top-level, transform trio conditioned on a selection, `create`/
  `targets`/`methods` groups, destructive `delete-selection` last); the only substantive change is
  that `selection_domains_from_surface`'s fallback is now real:

```rust
let (selected_nodes, selected_edges) = marks.graph_selection_domains(&doc.snapshot.fixture);
let (nodes, edges) = selection_domains_from_surface(request.surface.as_ref(), &selected_nodes, &selected_edges);
```

`selection_domains_from_surface` (`🔌️plugin/🦀️.rs:11968`) prefers the clicked surface's own painted
selection and falls back to the supplied ids, so a right-click on an unselected node still targets
what was clicked, while a menu opened where the surface reports nothing (`surface: None` — every
world-3d menu, since `World3dHost` puts only its own painted object ids there) now sees the real
selection.

- `✏️editor/🦀️.rs:1782-1796` — **new** `PreviewInteractionMarks::graph_selection_domains(fixture)`.
  The `graph` domain is one `DomainSelection` carrying node ids, `{w}@{c}` handles, `{w}@{c}#{i}`
  preview instances **and** synapse ids; a context menu addresses two domains. The split is by
  fixture membership: an id that is a synapse id is an `edge`, everything else projects onto its
  owning widget via the existing three-level `widget_of`. A preview instance selected in the world
  therefore targets its node exactly as a click in the graph does.

### 1.3 Tests

- `✏️editor/🧪️tests/🔬️unit/🦀️.rs:551` `context_menu_reads_the_framework_owned_graph_selection` —
  drives the **real** registered app: asserts the empty-selection menu offers neither
  `translateSelection` nor `removeWidget`, dispatches the framework's own
  `semio_framework::INTERACTION_SELECT_ACTION_ID` on domain `graph`, then asserts the menu unfolds
  `translateSelection`/`rotateSelection`/`scaleSelection`/`removeWidget`/`removeGeneration` and a
  destructive delete row. `request.surface` is `None` throughout, so the only possible source of the
  selection is the `InteractionView`.
- `:576` `graph_selection_splits_into_node_and_edge_domains` — pure unit over
  `graph_selection_domains`: `"note@out#2"` → node `"note"`, `"wire"` → edge `"wire"`.
- `:535` `context_menu_grouped_disclosure_stays_within_budget` — kept; its docstring no longer
  claims the selection rows are unreachable.

---

## 2. (b) The two pointer-down routes are deleted, not stubbed

### 2.1 What the framework does with them — the evidence

**`worldPointerDown`.** Exactly one dispatcher exists in the whole repo outside generated
`🔣️.json` descriptor mirrors and bundled `dist/` JS:

`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx:4819-4831`

```tsx
if (selection.engagementSessionActive && hostRef.current && cameraRef.current) {
  const point = raycastGroundPoint(event.clientX, event.clientY, rect, cameraRef.current);
  if (point) {
    dispatch("worldPointerDown", { pane: …, position: point, shiftKey, ctrlKey, metaKey });
```

It carries a **ground-plane point**, not a pick id, and it fires only while an engagement session is
armed. `engagementSessionActive` is a field the app itself publishes into its world-3d selection
JSON — the only two producers in the repo are
`✏️s/🔌️plugins/🏭️process/…/🪚️workpiece/🦀️.rs:74` (`active_utility != "select"`) and
`✏️s/🔌️plugins/📐️cad/…/🎭️modes/✏️edit/🦀️.rs:174` (`runtime.engagement_session.is_some()`).
generation3d's `preview_selection_json` (`✏️editor/🦀️.rs:1763-1780`) publishes no such field, and
its three utilities are `move`/`rotate`/`scale` — all gumball transforms, no placement gesture. The
route was therefore **unreachable in generation3d**. Picking in the world does not go through it at
all: `World3dHost/🟦️.tsx:4447` dispatches `interactionSelect`, the framework-injected verb, with
`world3dSelectionActionArgs(interactionDomainId, granularity, [record?.interactionId ?? id], merge)`.

**`graphPointerDown`.** Zero dispatchers anywhere — the string appears only in generated plugin
descriptor JSON and in three plugins' own declarations. Flow and trinity already deleted theirs on
ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM and say so in place:

> `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/…/✏️editor/🦀️.rs:2264-2266` — *"Selection/hover verbs (`setSelection`/`clearSelection`/`selectAll`/`selectNode`/`nodeGraphSelect`/`nodeGraphHover`/`graphPointerDown`) are no longer declared here: framework-owned, injected via `.interaction(...)` below"*

generation3d already declares that same `.interaction("graph")` domain with `Pick`/`Rectangle`
selection methods and binds it to all three of its windows (`✏️editor/🦀️.rs`, `.interaction(…)` +
three `.window_kind_interactions(…)` rows), so the six framework verbs
(`interactionSelect`/`interactionHover`/`clearSelection`/`selectAll`/`setSelectionMode`/
`setInteractionGranularity`) are auto-injected and are what a pick actually travels through.

### 2.2 Why deletion, not a transient observation hook

The brief allowed "record the observation into `🫧️transient` and prove it with a test" *if* the
routes were observation hooks. They are not: nothing can dispatch them into this app, so a transient
field fed by them would be permanently empty, and a test asserting a hand-dispatched payload lands
in transient would prove only that the test can dispatch it. Under CLAUDE.md's greenfield rules
("MUST NOT support legacy api", "MUST NOT add compatibility layers", "MUST aim for clean long term
solution") an unreachable route is dead API. The precedent inside the same domain (flow, trinity)
is deletion, and puzzle3d's surviving `worldPointerDown` is justified there by a real engagement
session — the condition generation3d does not meet.

If generation3d ever grows a placement utility, `worldPointerDown` comes back **with** the
`WindowEngagement` that makes it live, in one packet, rather than sitting as a stub that reads as
implemented.

### 2.3 What was removed

`✏️editor/🦀️.rs` — 14 lines, plus two names off the payload-module import list:

- `app_commands!` rows `"worldPointerDown" as "world-pointer-down"` and `"graphPointerDown" as "graph-pointer-down"` (was `:80-81`)
- `GENERATION3D_RETAINED_TOOL_IDS` entries (was `:267-268`)
- `ArtifactToolPublicationContract { tool_id: …, lanes: &[HostOnly] }` ×2 (was `:625-626`)
- `ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500)` arms ×2 (was `:1121-1122`)
- `command_from_action` arms ×2 (was `:1278-1279`)
- `ActionDefinition::new("worldPointerDown"/"graphPointerDown", …, ActionKind::View, "mouse-pointer")` ×2 (was `:1529-1530`)
- `.action_interactive_job(…, InteractiveJobClassification::Migrated)` ×2 (was `:1555-1556`)

Also removed: `🎮️commands/🌍️world-pointer-down/` and `🎮️commands/🖱️graph-pointer-down/` (directories),
their `#[path]` module mounts in `🗿️artifacts/🧊️generation3d/🦀️.rs` (was `:692`, `:736`), and the two
rows plus two wire keywords in `✏️editor/🧪️tests/🔬️unit/🦀️.rs`'s `every_command()` /
`expected_keywords` bijection.

The command enum is 27 rows now (was 29). Variant ordinals shift; that is a wire-format change and
is deliberate — greenfield, no legacy support.

### 2.4 Test

`✏️editor/🧪️tests/🔬️unit/🦀️.rs:589`
`no_pointer_down_route_survives_the_framework_owned_selection_domain` — serializes
`create_generation3d_app()` and asserts the manifest JSON contains no `PointerDown` at all, that no
retained tool id ends in `PointerDown`, and that the `graph` interaction domain (what replaced them)
is declared.

---

## 3. (c) Seven real config mutation leaves

### 3.1 Before

`✏️editor/🎚️config/🦀️.rs` hand-wrote `impl Mutation<Generation3dConfig>` with a `DESCRIPTORS` array
whose own docstring said it plainly:

> *"⚠️ PROVISIONAL: no variant below has an authored leaf directory on disk yet, so every `owner` names a path that does not exist"*

Seven descriptors, every `owner` pointing at `…/✏️editor/🎚️config/⚙️set-*` — seven directories that
never existed. Every `inverse` returned a whole-config `Snapshot`, so the config was invertible only
in bulk, and `textOpcode`/`binaryTag` were `None` on all seven.

### 3.2 After

The aggregate moved to `✏️editor/🎚️config/🧬️schema/🧬️mutations/🦀️.rs` (83 lines) and is now
`dsl::Mutations`-derived — the same shape the viewer lane authored
(`📓️viewer-2026-09-09.md` §2.1), so the `DESCRIPTORS` array is *generated from the on-disk
descriptors* rather than hand-asserted:

```rust
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslOps, dsl::Mutations)]
#[mutations(snapshot = Generation3dConfig, diff = Generation3dConfig, schema = "generation3dcfg")]
pub enum Generation3dConfigMutation {
    #[dsl(key = "snapshot")]            SetSnapshot(SetSnapshot),
    #[dsl(key = "lod-mode")]            SetLodMode(SetLodMode),
    #[dsl(key = "show-mode")]           SetShowMode(SetShowMode),
    #[dsl(key = "camera")]              SetCamera(SetCamera),
    #[dsl(key = "preview-camera")]      SetPreviewCamera(SetPreviewCamera),
    #[dsl(key = "sun")]                 SetSun(SetSun),
    #[dsl(key = "selected-generation")] SetSelectedGeneration(SetSelectedGeneration),
}
```

Seven authored leaf directories under `✏️editor/🎚️config/🧬️schema/🧬️mutations/`, each an immediate
child of the aggregate's own mutation root (so `provenance.mutation_root == scope.mutation_root`
holds by physical location, per the leaf-ownership contract in
`26/09/02/PUZZLE-3D-END-TO-END/📓️blocker-stdio-mutation-leaf-ownership.md`), each with `🦀️.rs`
(`dsl::MutationLeaf` + a real per-leaf `protocol::MutationKind` with its own `diff`/`inverse`/
`label`/`target`), `🔣️.json` (the 14-field descriptor the derive parses and validates —
`🗣️dsl/✨️derive/🦀️.rs:380-414`) and `🧬️schema/🔣️.json` (the draft-07 payload schema):

| dir | variant | dsl key | textOpcode | binaryTag | diffParticipation |
|---|---|---|---|---|---|
| `⚙️set-snapshot` | `SetSnapshot` | `snapshot` | `set-snapshot` | 1 | `detect` |
| `🔬️set-lod-mode` | `SetLodMode` | `lod-mode` | `lod-mode` | 2 | `apply-only` |
| `👁️set-show-mode` | `SetShowMode` | `show-mode` | `show-mode` | 3 | `apply-only` |
| `🕸️set-camera` | `SetCamera` | `camera` | `set-camera` | 4 | `apply-only` |
| `📷️set-preview-camera` | `SetPreviewCamera` | `preview-camera` | `preview-camera` | 5 | `apply-only` |
| `🌞️set-sun` | `SetSun` | `sun` | `set-sun` | 6 | `apply-only` |
| `🧬️set-selected-generation` | `SetSelectedGeneration` | `selected-generation` | `selected-generation` | 7 | `apply-only` |

`textOpcode` is prefixed with the verb wherever the dsl key is a single word (`snapshot`, `camera`,
`sun`): `mutation_leaf_kebab` (`🗣️dsl/✨️derive/🦀️.rs:434`) requires at least one hyphen — the exact
trap the viewer lane hit on `🌞️set-sun`, recorded in its own report. Emoji are pairwise distinct so
the `folder/name/emoji-not-unique` policy stays clean (verified, §5.3).

Each leaf's `inverse` is now a **real per-leaf inverse** (`SetLodMode { value: base.lod_mode }`, …)
instead of a whole-config `Snapshot`, so an undo of a sun change no longer rewrites the camera. Only
`SetSnapshot`, which genuinely is the bulk replacement, inverts to another `SetSnapshot`.

`✏️editor/🎚️config/🦀️.rs` shrank from 402 to 163 lines: the whole `🔖️ConfigOperations` region
(enum, hand-written `OpText`/`OpBinary`, the 240-line `DESCRIPTORS` array, `descriptor`, `diff`,
`inverse`) is replaced by `:154-155`:

```rust
#[path = "🧬️schema/🧬️mutations/🦀️.rs"]
mod mutations;
pub use mutations::*;
```

so every existing `use crate::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation}`
import keeps working unchanged.

### 3.3 Call sites

21 construction sites across 11 files moved from struct-variant to newtype form and are written
fully qualified — `Generation3dConfigMutation::SetSun(crate::editor::generation3d::config::SetSun { … })` —
because two command payload modules own structs with the same names as their config leaves
(`🎮️commands/👁️set-show-mode`'s `SetShowMode`, `🎮️commands/🔬️set-lod-mode`'s `SetLodMode`), and an
unqualified import would be ambiguous in exactly those two files. Files touched:
`🎮️commands/{🔭️node-graph-viewport,📷️set-camera,🔬️set-lod-mode,👁️set-show-mode,🌞️toggle-sun,🧭️set-sun-azimuth,🌄️set-sun-elevation,🔆️set-sun-intensity,🧬️generation,🎨️set-active-example}/🦀️.rs`
and `🎚️config/🧪️tests/🔬️unit/🦀️.rs`.

### 3.4 Tests

`✏️editor/🎚️config/🧪️tests/🔬️unit/🦀️.rs`:

- `:63` `every_config_leaf_owner_directory_exists_on_disk` — the viewer's test verbatim in shape:
  every `DESCRIPTORS[i].owner` must be a real directory carrying both `🔣️.json` and
  `🧬️schema/🔣️.json`. This is the assertion the old provisional array could not have passed.
- `:75` `config_leaf_descriptors_cover_every_variant_in_tag_order` — the descriptor set is exactly
  the seven semantic kinds, in binary-tag order.
- The five pre-existing round-trip tests (per-variant `diff`/`inverse` restoration, text/binary op
  round trip for every variant, pack round trip, defaults) were retargeted to the newtype form and
  now exercise the real per-leaf inverses rather than the blanket snapshot.

---

## 4. (d) `👥️set-contributions`

Already removed by the viewer lane (`📓️viewer-2026-09-09.md` §2.7 — it was an empty directory
created 2026-08-16 and never populated). Re-verified this packet: the directory is absent from
`✏️editor/🎮️commands/`, and `grep -rn setContributions ✏️s/🔌️plugins/🌀️procedural/` returns nothing
outside generated `🔣️.json` descriptor mirrors. Nothing further owed. The name stays legal in
`🔣️taxonomy.json`'s repo-global `members-of-commands` allowlist, which is not a per-directory
set-equality, so no taxonomy edit was needed and none was made.

---

## 5. Gate outputs

Private target dir throughout: `CARGO_TARGET_DIR=$S/target-edit`, `RUSTC_WRAPPER=""`,
`--keep-going`, seeded by APFS clone `cp -Rc target/debug` (76 GB, free on APFS). Raw logs copied to
`🗑️generated/`.

### 5.1 Baseline (before any edit)

`cargo check -p semio-s-plugin-procedural --keep-going` → **0 errors**, `Finished dev profile in
1m 31s` (20:01). The tree this packet started from was green, so every error below is attributable.
⚠️ That log did not survive a scratchpad cleanup forced by an ENOSPC (§5.10); the result is reported
from the run, not from a retained file.

### 5.2 After the config-leaf refactor (§3)

`cargo check -p semio-s-artifact-procedural-generation3d --features component-app-assembly --keep-going`

```
rc=0
warning: unused import: `SpaceMember`   (🧰️framework/…/🌊️flow/🖥️host/🦀️.rs — not this packet's)
warning: `semio-framework-os-flow` (lib) generated 1 warning
```

**0 errors, 0 warnings owned by this packet.** Warnings are being emitted, so this is a real
type-check and not an aborted expansion. This is the run that proves the `dsl::Mutations` derive
accepted all seven on-disk descriptors (it parses and validates them at macro-expansion time and
const-evaluates the source authority).

### 5.3 Taxonomy policies

`bun $T/🐍️viewer-policy-check.ts` — the viewer lane's ticket-local runner, unmodified: it already
filters on the whole `🌀️procedural` scope, so it covers the editor surface as-is (no second script
was added, per CLAUDE.md's one-`📜️script.ts` rule).

```
viewer-purity:                0 procedural breach(es) of 2 repo-wide
subset-surface-completeness:  0 procedural breach(es) of 66 repo-wide
contributed-surface-target:   0 procedural breach(es) of 0 repo-wide
app-schema:                   6 procedural breach(es) of 533 repo-wide
emoji-prefix:                12 procedural breach(es) of 1287 repo-wide
```

- **app-schema 12 → 6** against the viewer lane's capture: all six remaining are pre-existing
  surfaces this packet does not own (`🌀️generation2d/👁️viewer`, `🧩️assembly/✏️editor`,
  `🧩️assembly/👁️viewer` — each missing its config/presence schema facet entirely). The seven new
  `🧬️mutations/*/🧬️schema` leaves added none.
- **emoji-prefix 13 → 12**, and none of the twelve names a directory this packet created. The seven
  new leaf emoji (`⚙️ 🔬️ 👁️ 🕸️ 📷️ 🌞️ 🧬️`) are pairwise distinct within their parent.

Raw: `🗑️generated/editor-policy-check.txt`.

### 5.4 `cargo check -p semio-s-plugin-procedural --keep-going` (native, whole plugin)

```
Checking semio-s-artifact-procedural-generation3d v0.1.0 (…/🧊️generation3d/📦️packages/🦀️rust)
Checking semio-s-plugin-procedural v0.1.0 (…/🌀️procedural/📦️packages/🦀️rust)
Finished `dev` profile [unoptimized] target(s) in 1m 29s
```

**0 errors.** One warning in the whole closure, and it is not this packet's:
`🧰️framework/…/🌊️flow/🖥️host/🦀️.rs — unused import: SpaceMember`. Raw:
`🗑️generated/editor-native-check.txt`.

### 5.5 `cargo check -p semio-s-plugin-procedural --target wasm32-wasip2 --profile wasm-dev --keep-going`

With `CARGO_PROFILE_WASM_DEV_DEBUG=false` and a separate private `target-edit-wasm`:

```
Finished `wasm-dev` profile [unoptimized] target(s) in 2m 06s
```

**0 errors**, 98 crates compiled from cold, same single peer warning. Raw:
`🗑️generated/editor-wasm-check.txt`.

### 5.6 `cargo check … --lib --tests`

```
warning: `semio-s-artifact-procedural-generation3d` (lib test) generated 5 warnings
Finished `dev` profile [unoptimized] target(s) in 6.53s
```

**0 errors.** All five warnings are `unnecessary qualification` at lines this packet did not author
(`🧬️mutations/💾️binary/🧪️tests/🔬️retained-authority-laws/🦀️.rs:119`,
`✏️editor/🧪️tests/🔬️testkit/🦀️.rs:37,38,44`, `✏️editor/🧪️tests/🔬️unit/🦀️.rs:9`). Raw:
`🗑️generated/editor-lib-tests-check.txt`.

Getting here took several poll iterations against three concurrent lanes: 26 errors at 19:58 all in
`🧰️framework/…/🌊️flow/🖥️host/🦀️.rs` (`PreviewTessellate*`, `preview_mesh_pack_has_geometry` — the
resumable-tessellation lane), then a `host_from_fixture` removal that left dangling call sites in
`🕸️flow/🦀️.rs`, `👁️viewer/🦀️.rs` and `↔️translate-selection`'s test, then an `ArtifactView::new`
signature change, then a `PluginApp` bound change (`M: SpaceMember + MemberFactory`). Every one was
another session's in-flight refactor and every one was repaired by its author within minutes; none
was this packet's.

### 5.7 Unit tests — `cargo test … --lib -- <exact filters> --test-threads=1`

```
test editor::generation3d::component::tests::graph_selection_splits_into_node_and_edge_domains ... ok
test editor::generation3d::component::tests::no_pointer_down_route_survives_the_framework_owned_selection_domain ... ok
test editor::generation3d::config::component::tests::config_leaf_descriptors_cover_every_variant_in_tag_order ... ok
test editor::generation3d::config::component::tests::config_op_text_round_trips_every_variant ... ok
test editor::generation3d::config::component::tests::config_set_camera_and_preview_camera_round_trip ... ok
test editor::generation3d::config::component::tests::config_set_selected_generation_round_trips ... ok
test editor::generation3d::config::component::tests::config_set_sun_round_trip_as_raw_json ... ok
test editor::generation3d::config::component::tests::every_config_leaf_owner_directory_exists_on_disk ... ok
test editor::generation3d::config::component::tests::generation3d_config_default_matches_the_former_runtime_defaults ... ok
test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 284 filtered out
```

Run at 20:29 against the final `🌀️procedural` source. Raw: `🗑️generated/editor-unit-tests.txt` — a
verbatim transcript rather than the log file, because a 20:39–20:49 re-run overwrote the log and
never got past two further peer breaks: first `stdio-semio`'s `🎨️blend/🦀️.rs` (§5.9), then a
repo-wide rename of the editor's `testkit` module that broke ~10 `🎮️commands/*/🧪️tests/🔬️unit/🦀️.rs`
files at once (`cannot find module or crate testkit`). Neither touches this packet's files; the
blocked capture is kept at `🗑️generated/editor-unit-tests-peer-blocked-2049.txt` for attribution.

**`context_menu_reads_the_framework_owned_graph_selection` is written and type-checks but has NOT
executed green** — stated plainly rather than claimed. It aborts the test process inside
`app.handle_action(semio_framework::INTERACTION_SELECT_ACTION_ID, …)`:

```
[DEBUG] before empty-selection context_menu
[DEBUG] empty-selection context_menu returned ["reorganize", "menu.group.create", "addWidget",
        "addGeneration", "menu.group.methods", "renameGeneration", "updateGenerationValues",
        "patchFlowWidgets"]
thread '…::context_menu_reads_the_framework_owned_graph_selection' has overflowed its stack
fatal runtime error: stack overflow, aborting
```

So the *first half* of the test — the empty-selection menu, which is the branch that proves the rows
are correctly folded away — does run and returns exactly the expected eight rows. The abort is in the
framework's own interaction-select dispatch, and it is **not this packet's**: the untouched,
pre-existing `generation3d_interaction_selection_owns_its_persisted_history` (which this packet does
not modify beyond adding the same teardown call) fails identically, and so does the untouched
`preview_eval_exact_window_transient_isolates_and_resets_in_the_registered_app` (a different
assertion, then the same abort). Two further crate-wide regressions from other lanes are visible in
the same runs and were reproduced against untouched tests:

- `ordered-map root must be explicitly retired before drop`
  (`🧰️framework/🔨️modules/🌱️value/🗂️ordered/🦀️.rs:81`, invariant added 2026-09-08 23:25) fires on the
  **temporary returned by `VcsArtifactApp::snapshot()`** — it hands back an *owned* `A::Snapshot`,
  and `Generation3dSnapshot.fixture.layout` is an `OrderedMap<WidgetLayout>`. Backtrace frames 2→6:
  `OrderedMap<WidgetLayout>::drop` ← `FlowFixture` ← `Generation3dSnapshot` ← the test closure. Every
  registered-app test in this crate that calls `app.snapshot()` therefore aborts. This packet's own
  test was rewritten to use a literal default-fixture widget id instead, so it is immune.
- The abort cascades into `artifact store cursor disposer reached Drop before terminal-empty
  ownership` (`🏪️store/🦀️.rs:2117`) — a *consequence* of unwinding, not an independent defect.
  `semio_framework_plugin::testkit::close_registered_fixture_app` was added to the three
  registered-app tests in `✏️editor/🧪️tests/🔬️unit/🦀️.rs` that lacked it, which is correct on its own
  merits and does not by itself clear the abort.

### 5.8 `bun ./📜️script.ts verify interactivity`

Red repo-wide, and **zero of its findings name `generation3d`, `procedural` or `PointerDown`**
(grepped: 0 hits each). The run aborts inside a puzzle self-test
(`interactivityPuzzleFillEnvelopeSelfTests`, `✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🔬️interactivity-puzzle-fill-envelope/🟦️.ts:77`)
after reporting the launch-seed gate registrations, `19228 descriptors exceed fixed capacity 256`,
and 757+ all-app discovery failures under `✒️writer`. Raw: `🗑️generated/editor-interactivity.txt`.

`bun ./📜️script.ts verify interactivity apps`:

```
descriptors=3 extensions=0 apps=8 launchOnlyProducts=52 surfaces=60 actions=550
migratedActions=404 missingActions=146 launchCoveredApps=0 launchMissingApps=8
launches=151 failures=782 selfTests=25
```

Byte-identical to the viewer lane's capture, with **0 `generation3d` hits among the 782 failures**.
The counters are unchanged by this packet's two removed actions because this gate reads the
*generated* `🔣️.json` descriptor mirrors, not source — the same degraded-discovery mode the viewer
report documents, and the reason §7 records the descriptor regen as owed. Raw:
`🗑️generated/editor-interactivity-apps.txt`.

### 5.9 Re-run at 20:30 — a fourth peer break

Re-running §5.4/§5.5 at 20:30 returned 2 errors, both in
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/…/🪆️subsets/🧊️brep/🧬️schema/🔺️diff/🎨️blend/🦀️.rs:32,676` —
the boolean-kernel lane's file (`📓️boolean-kernel-2026-09-09.md`), reached through
`semio-s-artifact-stdio-semio`. Nothing in `🌀️procedural`. The clean §5.4/§5.5 captures above are
from 20:16 and 20:20 against a `🌀️procedural` source tree identical to the final one (only test
files changed afterwards, and neither check builds tests).

### 5.10 Environment note — the box ran out of disk mid-packet

At 20:04 `/System/Volumes/Data` hit 100% and **every** Bash call failed with
`ENOSPC … tasks/<id>.output` before executing, including `df` and `rm` (the harness writes its
output file first). Foreground tool calls were unusable; backgrounded ones (`run_in_background`)
still ran, which is what allowed the private target dir to be removed and the disk recovered. Worth
recording as a recovery path: **when every Bash call dies on ENOSPC, run the cleanup in the
background.** The private target dir was then re-seeded by APFS clone (free in blocks) and all
subsequent runs used it.

## 6. Files changed

### New

```
✏️editor/🎚️config/🧬️schema/🧬️mutations/🦀️.rs
✏️editor/🎚️config/🧬️schema/🧬️mutations/⚙️set-snapshot/{🦀️.rs,🔣️.json,🧬️schema/🔣️.json}
✏️editor/🎚️config/🧬️schema/🧬️mutations/🔬️set-lod-mode/{🦀️.rs,🔣️.json,🧬️schema/🔣️.json}
✏️editor/🎚️config/🧬️schema/🧬️mutations/👁️set-show-mode/{🦀️.rs,🔣️.json,🧬️schema/🔣️.json}
✏️editor/🎚️config/🧬️schema/🧬️mutations/🕸️set-camera/{🦀️.rs,🔣️.json,🧬️schema/🔣️.json}
✏️editor/🎚️config/🧬️schema/🧬️mutations/📷️set-preview-camera/{🦀️.rs,🔣️.json,🧬️schema/🔣️.json}
✏️editor/🎚️config/🧬️schema/🧬️mutations/🌞️set-sun/{🦀️.rs,🔣️.json,🧬️schema/🔣️.json}
✏️editor/🎚️config/🧬️schema/🧬️mutations/🧬️set-selected-generation/{🦀️.rs,🔣️.json,🧬️schema/🔣️.json}
```

### Deleted

```
✏️editor/🎮️commands/🌍️world-pointer-down/🦀️.rs   (directory)
✏️editor/🎮️commands/🖱️graph-pointer-down/🦀️.rs   (directory)
```

### Modified

| path | change |
|---|---|
| `✏️editor/🦀️.rs` | `context_menu` → delegate (`:1415`); new `context_menu_with_request_context` (`:1430`); new `context_menu_body` (`:1449`); new `PreviewInteractionMarks::graph_selection_domains` (`:1782`); both pointer-down routes removed from the command enum, retained tool ids, publication contracts, execution contracts, `command_from_action`, action definitions and interactive-job classifications |
| `✏️editor/🎚️config/🦀️.rs` | 402 → 163 lines: whole `🔖️ConfigOperations` region replaced by the `🧬️mutations` mount (`:154`); unused `use protocol::Mutation;` dropped |
| `✏️editor/🎚️config/🧪️tests/🔬️unit/🦀️.rs` | newtype call sites; `use protocol::Mutation;`; two new tests (`:63`, `:75`) |
| `✏️editor/🧪️tests/🔬️unit/🦀️.rs` | pointer-down rows + wire keywords removed from the command bijection; three new tests (`:551`, `:576`, `:589`); `:532` docstring corrected; `close_registered_fixture_app` added to the three registered-app tests that lacked it (`:530`, `:545`, `:573`) |
| `✏️editor/🎮️commands/{🔭️node-graph-viewport,📷️set-camera,🔬️set-lod-mode,👁️set-show-mode,🌞️toggle-sun,🧭️set-sun-azimuth,🌄️set-sun-elevation,🔆️set-sun-intensity,🧬️generation,🎨️set-active-example}/🦀️.rs` | config-mutation construction moved to the newtype form |
| `🗿️artifacts/🧊️generation3d/🦀️.rs` | two `#[path]` command module mounts removed |

## 7. Owed / follow-ups (not this packet's)

- The generated plugin descriptor mirrors (`✏️s/🔌️plugins/🌀️procedural/🔣️.json`,
  `🧰️framework/…/🧑‍💻dev/🔌️plugin-modules/🌀️procedural/🔣️.json`, the `dist/dev` and `dist/release`
  twins) still list `worldPointerDown`/`graphPointerDown`. They are regenerated from a served wasm
  build, which the ticket already tracks as owed (`📓️status.md`, 19:26 entry). Not hand-edited.
- The six remaining `app-schema/facet-completeness` breaches under `🌀️procedural` belong to
  `🌀️generation2d/👁️viewer` and both `🧩️assembly` surfaces.
- **`VcsArtifactApp::snapshot()` aborts the process on drop** for any app whose snapshot holds an
  `OrderedMap` (§5.7). This is a framework-level regression against the 2026-09-08 ordered-map
  retirement invariant and it currently kills every registered-app test in this crate that reads a
  snapshot; it needs either a retiring accessor or an explicit retire in the testkit. Worth its own
  ticket — it is far wider than generation3d.
- **`handle_action(INTERACTION_SELECT_ACTION_ID)` stack-overflows** in this crate (§5.7). Same
  attribution: the untouched pre-existing test reproduces it. Once it clears, the one written test
  that could not execute — `context_menu_reads_the_framework_owned_graph_selection` — should be run
  with `cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly
  --lib -- editor::generation3d::component::tests::context_menu_reads_the_framework --test-threads=1`.
