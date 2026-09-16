# 🕹️ Interaction-view threading — findings + fix (2026-09-16)

Ticket: `26/08/28/DEMONSTRATOR-END-TO-END-ALL-APPS`.
Scope: thread the framework-owned interaction state (selection + hover per domain) into every
render/chrome path uniformly, and consume it in the six demonstrator apps.

## 1. Findings — the audits are LARGELY STALE

All paths below are relative to `/Users/ueli/Documents/semio`.
Framework plugin SDK file (consolidated, 39 083 lines):
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` — referred to below as **SDK**.

### 1.1 The mechanism already exists for `render` and `context_menu`

- **SDK:9439** `pub struct InteractionView<'a> { state, hover, peers }` — read-only combined view over
  the persisted-local `protocol::InteractionState` (selection / active mode / active granularity) and
  the ephemeral-local `InteractionHoverState`, plus the peer-presence roster.
  Accessors **SDK:9450** `selection(domain)`, **SDK:9470** `leftover_selected_ids()`,
  **SDK:9478** `hover(domain, channel)`, **SDK:9487** `active_granularity`, **SDK:9493** `active_mode`,
  **SDK:9508/9520** `peers_selecting`/`peers_hovering`.
- **SDK:11554** `ArtifactApp::render_with_request_context(owner, body_key, doc, cfg, view_state, transient, interaction)`
  — default body discards `transient`/`interaction` and falls through to `render`.
- **SDK:11615** `ArtifactApp::context_menu_with_request_context(request, doc, cfg, view_state, interaction, registry)`.
- **SDK:11586** `ArtifactApp::window_measures_with_request_context(doc, cfg, view_state, interaction)`.
- The host **always** calls the `_with_request_context` twins, never the plain ones:
  - **SDK:29473** and **SDK:29487** — `A::render_with_request_context(...)` (both the
    snapshot-override branch and the normal branch), with the view built at **SDK:29462**.
  - **SDK:29531** — `InteractionView` built for `window_measures`; used at **SDK:29537**.
  - **SDK:29622** — `InteractionView` built for `context_menu`; used at **SDK:29632**.

So the audits' claim *"`ArtifactApp::render()` / `context_menu()` are invoked by the framework without
an `InteractionView`"* is **no longer true** for `render`, `context_menu` and `window_measures`.

### 1.2 The ONE remaining framework hole: `window_engagements`

`window_engagements_with_request_context` is the only `_with_request_context` twin that carries
`transient` but **no `interaction`**:

- **SDK:11573** (`ArtifactApp` trait), **SDK:30928** (`ArtifactEditor`), **SDK:31278** (`ArtifactViewer`),
- blanket impls **SDK:31641** (Editor→App) and **SDK:31904** (Viewer→App),
- builder-macro trampoline **SDK:7356**,
- host call site **SDK:29516**, inside `VcsArtifactApp::window_engagements` (**SDK:29494**), which —
  unlike `render`/`window_measures`/`context_menu` — never builds an `InteractionView` at all.

That is what keeps the **cad engagement HUD "N selected" reading 0**.

### 1.3 Per-app state BEFORE this fix

| app | crate / editor file | render selection | chrome / menu |
|---|---|---|---|
| generator `generation3d` | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/…/✏️editor/🦀️.rs` | ✅ **:2210** `render_with_request_context` → `PreviewInteractionMarks::from_interaction` → node graph + world previews + inspection | ✅ **:2260** `context_menu_with_request_context` |
| aggregator `puzzle3d` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/…/✏️editor/🦀️.rs` | ✅ **:8217** → `Puzzle3dInteractionSnapshot::from_interaction` → `render_body` **:8354** `inspection::render(&envelope, interaction, labels)` | ✅ **:8235** context menu; ⚠️ **:8256** engagements had no interaction |
| aussuchen `curation` | `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/…/✏️editor/🦀️.rs` | ✅ **:1042** → `interaction.selection(SOURCING_ROWS_DOMAIN).ids` → `preview::render(snapshot, &selected_ids, labels)` (the hardcoded `&[]` at **:1033** is only the interaction-less `render` delegate) | n/a |
| bearbeiten `process3d` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/…/✏️editor/🦀️.rs` | ✅ **:1637** → `interaction.selection(PROCESS3D_INTERACTION_DOMAIN)` → inspection | ❌ **:1671** `context_menu` NOT selection-gated (documented gap) |
| koordinator `cad` | `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/…/✏️editor/🦀️.rs` | ✅ **:1968** → `CadInteractionSnapshot::from_interaction` → scenes + inspection + tree | ❌ **:1979** `window_engagements` builds `CadInteractionSnapshot::default()`, so the HUD at `…/✏️editor/🎭️modes/✏️edit/🦀️.rs:299` counts `0`; ❌ **:2013** `context_menu` not selection-gated |
| verfolgen `gismap` | `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/…/✏️editor/🦀️.rs` | ❌ **no** `render_with_request_context` at all — inspection panel `…/📌️panels/🔍️inspection/🦀️.rs:31` renders a map-wide summary only, with a stale "SDK gap" doc comment | ❌ `context_menu` **:985** not interaction-aware |

So the real work is: (a) close the framework `window_engagements` hole, (b) wire **gis**, (c) wire the
**cad** HUD + menu, (d) gate the **process3d** menu.

## 2. Design

One mechanism, no per-app hacks. The framework already threads `InteractionView` through three of the
four per-request projections; the fix is to complete the set rather than invent anything new:

1. **Framework** — `window_engagements_with_request_context` gains `interaction: &InteractionView<'_>`
   as its last parameter, on all three traits (`ArtifactApp`/`ArtifactEditor`/`ArtifactViewer`), both
   blanket impls and the builder-macro trampoline. Its default body keeps discarding it
   (`let _ = (transient, interaction);`) and falling through to `window_engagements`, so no app is
   forced to care. The host's `VcsArtifactApp::window_engagements` builds the view with the SAME
   materialize-owned-before-field-wise-destructure shape the three existing paths use.
2. **Apps** — each app that wants the selection overrides the `_with_request_context` twin and funnels
   both entry points through one shared body (`*_body`), so the interaction-less twin is a delegate
   against an empty domain rather than a second implementation. This is the pattern cad/puzzle3d/
   procedural already established for `render`; it now also covers window chrome and context menus.

Apps read their own domain out of the view into a plain, app-owned snapshot struct
(`CadInteractionSnapshot`, `Puzzle3dInteractionSnapshot`, `PreviewInteractionMarks`, and the new
`Gis2dInteractionSnapshot`), because `InteractionView`'s fields are `pub(crate)` to the SDK crate and
an app's own tests cannot construct one.

## 3. Edits

### 3.1 Framework — `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`

| line | change |
|---|---|
| **7356-7364** | builder-macro viewer trampoline: `interaction` parameter added and forwarded to `V::window_engagements_with_request_context` |
| **11574-11583** | `ArtifactApp::window_engagements_with_request_context` — new `interaction` parameter, new doc comment naming it the fourth and last per-request projection, `let _ = (transient, interaction);` |
| **29494-29500** | `VcsArtifactApp::window_engagements` — builds `interaction_state`/`interaction_hover`/`interaction_peers` and the `InteractionView`, before the `&mut self` destructure |
| **29530** | call site now passes `&interaction` |
| **30942-30945** | `ArtifactEditor::window_engagements_with_request_context` — same parameter + default body |
| **31292-31295** | `ArtifactViewer::window_engagements_with_request_context` — same |
| **31655-31657** | Editor→App blanket impl forwards `interaction` to `E::…` |
| **31918-31920** | Viewer→App blanket impl forwards `interaction` to `V::…` |

### 3.2 koordinator `📐️cad` — `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`

| line | change |
|---|---|
| **1064-1075** | NEW `CadPlayApp::window_engagements_body(doc, cfg, view_state, interaction)` — the one chrome implementation both entry points share, building `CadPlayView { … interaction }` |
| **1995-2000** | `window_engagements` is now a delegate against an empty domain |
| **2002-2014** | NEW `window_engagements_with_request_context` — `CadInteractionSnapshot::from_interaction` → the HUD's `"N selected"` |
| **2028-2035** | `context_menu` body unchanged; its "⚠️ documented reduced-fidelity gap" comment replaced — the `InteractionView` is reachable now, so leaving the section ungated is a UX choice, not an SDK limitation (see §4.4) |

`…/✏️editor/🎭️modes/✏️edit/🦀️.rs:297-299` — the stale "`window_engagements` has no request context"
comment above `let selected_count = envelope.interaction.ids.len();` replaced; the line itself is
unchanged and now reads a live selection.

### 3.3 verfolgen `🌍️gis` — the app that had NO threading at all

`✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`

| line | change |
|---|---|
| **20** | `MapFeature` added to the `use crate::{…}` list |
| **44-45** | NEW `GIS2D_LAYER_GRANULARITY` |
| **47-95** | NEW `Gis2dInteractionSnapshot` + `from_interaction` / `selected_layer` / `feature_selection_json` / `feature_hover_json` (the `{positions,routes}` and `{kind,id}` shapes `TiledMapHost`'s `parseFeatureSelection`/`parseMapHoveredFeature` read) |
| **758-770** | NEW `Gis2dPlayApp::render_body(…, interaction)` — the one render implementation both entry points share |
| **1049-1068** | `render` delegates against an empty domain; NEW `render_with_request_context` |
| **1075-1101** | `context_menu` delegates; NEW `context_menu_with_request_context` gating `clearSelection` on the real selection |
| **735-737** | `gis2d_context_menu_items`' "🕳️ `selected_ids` is always empty for now" doc replaced |

⚠️ A concurrent peer worked the same gis seam during this session and converged on the same design —
they wired `map::render` to consume `Gis2dInteractionSnapshot::feature_selection_json`/
`feature_hover_json`, authored `render_body`/`render_with_request_context`, added two end-to-end laws
(`an_interaction_select_marks_the_feature_selected_in_the_rendered_scene`,
`an_interaction_hover_reaches_the_scene_as_the_popup_record`), and corrected my inspection test to
project through `artifact_app_laws::project_and_retire_fixture_tree`. Those edits are kept as-is.

`…/✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🦀️.rs:85-92` — `render` takes the interaction snapshot and
sets `scene.selection_json` / `scene.hover_json` (was: `TiledMapScene::base`'s empty defaults + a
"known SDK gap" comment).

`…/✏️editor/📌️panels/🔍️inspection/🦀️.rs:26-72` — `render(document, cfg, interaction, labels)`; the
summary now carries a `Selected` count, a `"layer"` pick adds an id/label/visible section, a
`"feature"` pick adds an id/kind section, with a new `feature_kind` helper.

`…/✏️editor/🗣️terminology/🦀️.rs:38-41` — four new label rows (`selected`, `visible`, `feature`,
`feature_kind`), native EN + DE.

### 3.4 bearbeiten `🏭️process` — `…/🧊️process3d/…/✏️editor/🦀️.rs`

| line | change |
|---|---|
| **261-272** | NEW `process3d_context_menu_items(registry, selected_ids)` — `removeSelectedStep` only for a non-empty `"geometry"` selection |
| **1667-1687** | `context_menu` delegates against an empty selection; NEW `context_menu_with_request_context`. Replaces the "no longer selection-gated" comment |

Also `…/✏️editor/🧪️tests/🔬️unit/🦀️.rs:192-198` — the shared `context::render` helper was serializing
`tree.root` directly, which now fails with `Error("BuiltChildren requires retained page transport")`
(a framework transport migration landed by a peer this session). Switched to
`artifact_app_laws::project_and_retire_fixture_tree`, the same route the gis peer used. This moves the
`panels::inspection` tests from a transport error to a real render; see §4.3 for why the raw failure
counts across the two runs are not directly comparable.

### 3.5 Mechanical parameter additions (no behaviour change)

Every other overrider of `window_engagements_with_request_context` takes the new parameter as
`_interaction`: `✒️writer` (**:1290**), `🗒️note` (**:369**), `📏️layout` (**:693**), `🧩️puzzle 🧊️3d`
(**:8261**), `🧩️puzzle 🖐️5d` (**:7970**), `🧩️puzzle ◻️2d` (**:3763**), `🖍️draw` (**:1483**).

### 3.6 Tests added

- `📐️cad …/✏️editor/🧪️tests/🔬️unit/🦀️.rs` — `the_engagement_hud_counts_the_threaded_cad_selection`:
  `shape::engagement` over a `CadPlayView` with an empty vs. two-id `CadInteractionSnapshot`; asserts
  the `cad-status` row text starts `0` / `2`.
- `🌍️gis …/📌️panels/🔍️inspection/🧪️tests/🔬️unit/🦀️.rs` —
  `the_inspector_detail_section_follows_the_features_selection`: empty domain → summary only; a
  `"layer"` pick → the layer detail section; a `"feature"` pick → the feature's id and document kind.
- `🏭️process …/✏️editor/🧪️tests/🔬️unit/🦀️.rs` —
  `the_context_menu_gates_the_destructive_row_on_the_geometry_selection`: `removeSelectedStep` absent
  for an empty selection, present for a non-empty one, `addStep` present either way.

## 4. Verification

### 4.1 Unrelated, PRE-EXISTING breakage (peer churn, not from this fix)

Two crates neither touched nor depended on by this change fail to compile in the current tree, and the
errors changed shape between two runs minutes apart — a peer's live refactor:

- `semio-s-artifact-trinity-jack` — first `couldn't read …/🎮️commands/🧫️set-snapshot-json/🦀️.rs: No such
  file or directory`, later 11 × `E0425: cannot find value 'snapshot'` in `🧬️schema/🧮️executor/🦀️.rs:43-45`.
- `semio-s-artifact-draw-drawing` — 8 × `E0433: cannot find module or crate 'serde_json'` in
  `…/🪟️windows/🖼️canvas/🫧️transient/🦀️.rs`.

This blocks `semio-s-plugin-draw` and `semio-s-artifact-writer-writer` (which depends on trinity-jack)
from being verified. Their edits here are the same single `_interaction` parameter that compiles
cleanly in note/layout/puzzle-2d/puzzle-5d/puzzle-3d.

### 4.2 Commands run

Output tee'd under `🗑️generated/check-interaction-view-threading-{1..5}.txt` and
`🗑️generated/test-interaction-view-threading-1.txt`. Every cargo invocation was prefixed
`DEVELOPER_DIR=/Library/Developer/CommandLineTools RUSTFLAGS=-Awarnings
CARGO_PROFILE_WASM_DEV_DEBUG=false CARGO_BUILD_JOBS=4` (the last per the coordinator's load-shedding
note; checks were batched into one invocation rather than one per crate).

```
cargo check --target wasm32-wasip2 --keep-going \
  -p semio-framework-plugin -p semio-s-plugin-gis -p semio-s-plugin-cad \
  -p semio-s-plugin-process -p semio-s-plugin-puzzle -p semio-s-plugin-sourcing \
  -p semio-s-plugin-procedural
# → Finished `dev` profile in 2m 30s — CLEAN
```

All seven — the framework SDK plus every one of the six demonstrator plugins — check clean on
`wasm32-wasip2`, including the transitively rebuilt artifact crates `semio-s-artifact-cad-cad`,
`semio-s-artifact-gis-gismap`, `semio-s-artifact-process-process3d`, `semio-s-artifact-puzzle-{2d,3d,5d}`,
`semio-s-artifact-procedural-{generation2d,generation3d,assembly}`, `semio-s-artifact-sourcing-curation`.

```
cargo check --target wasm32-wasip2 --keep-going \
  -p semio-s-plugin-writer -p semio-s-plugin-note -p semio-s-plugin-layout -p semio-s-plugin-draw
# → semio-s-artifact-note-note, semio-s-artifact-layout-layout, semio-s-plugin-note,
#   semio-s-plugin-layout: CLEAN. writer/draw blocked by §4.1.
```

A final re-check after the §4.4 revert:

```
cargo check --target wasm32-wasip2 --keep-going -p semio-framework-plugin \
  -p semio-s-plugin-gis -p semio-s-plugin-cad -p semio-s-plugin-process \
  -p semio-s-plugin-puzzle -p semio-s-plugin-procedural
# → Finished in 34.76s — CLEAN
```

`semio-s-plugin-sourcing` was clean in the earlier run and now fails on a change that is not mine: a
peer's namespace churn made `Trigger` in `…/🪵️sourcing/…/🪟️windows/🏊️pool/🦀️.rs:128` resolve to
`semio_framework::machine::Trigger` (`Event`/`Eventless`/`Done`/`Timer`), so `Trigger::Click` no longer
exists — `E0599`. Nothing in this fix touches that file, that import or that enum.

### 4.3 Native tests

Counts are what I measured; I did NOT establish a clean before-baseline for process3d or cad (no
`git stash` is permitted here), so attribution below rests on the failure messages and call sites,
which are quoted.

**`semio-s-artifact-gis-gismap` — GREEN, 244 passed / 0 failed** (`--features component-app-assembly
--lib -- --skip live_envelope`). Includes my
`the_inspector_detail_section_follows_the_features_selection` and the peer's two scene laws. An
earlier run of the same suite (10:42, before the peer corrected my test's transport) had 2 failures;
both are now green.

**`semio-s-artifact-process-process3d` — 299 passed / 32 failed / 3 ignored / 1 filtered**
(`--lib -- --skip vcs_artifact_app_production_maintenance_swap`; that one test hangs indefinitely —
18+ minutes in the unfiltered run before it was terminated). My own
`the_context_menu_gates_the_destructive_row_on_the_geometry_selection` and the pre-existing
`world_context_menu_exposes_process_commands` both pass.

⚠️ Precision note: the earlier unfiltered run (before my `context::render` harness fix) emitted **38
individual `FAILED` lines but never a summary line** — its harness was wedged on the hanging test and
terminated, so it had not finished enumerating. It also ran the whole crate, whereas the post-fix run
is `--lib` with one test skipped. So "38 → 32" is NOT a clean before/after delta and should not be
quoted as one. What IS directly measured: before the fix, the seven `panels::inspection` tests failed
with `Error("BuiltChildren requires retained page transport")` from `context::render`; after it, those
same tests reach a real render and fail (or pass) on their own content assertions instead. That is the
only improvement I can attribute to the harness change.

The 32 remaining failures are NOT from this change — their messages name framework store/registry
migrations in flight:
- `ValidationFailed("edit history insertion requires its exact mutation retirement factory")`
- `"typed-operation pending publication rejected a stale immutable document root"`
- `"processd-publication.saturated"`, `"unknown process export format kind 'step'"`

The first of these is also why the `selected_*_renders_*` inspection tests still fail: their `select`
helper dispatches a real `interactionSelect`, whose `HistoryLane::Interaction` write goes through the
same edit-history insertion path that is currently rejecting, so the `"geometry"` domain stays empty
and the panel renders `"No selection"`. The render wiring itself is correct and untouched by this
change (`process3d_render_body:1301` → `inspection::render(doc, selected_ids, labels)`, fed by the
`render_with_request_context` that already existed before this session).

**`semio-s-artifact-cad-cad` — 7 passed / 4 failed** on the `engagement`/`context_menu` filter. My
`the_engagement_hud_counts_the_threaded_cad_selection` passes, and both pre-existing context-menu
tests are green. The 4 failures all abort inside app CONSTRUCTION, before any code this change
touches: `🔌️plugin/🦀️.rs:21438` `interactive-job.catalog-authority` — *"tool factory proof rejected
tool 'addNode' … generated_migrated=false … migrated={}"* — plus one in `🏪️store/🦀️.rs:2697`. A peer is
working this exact registry seam (`🗑️generated/registry-check-generated.txt`,
`registry-catalog-complete.txt`, both 10:47).

### 4.4 One change made and then reverted

I first gated cad's `context_menu` on a non-empty `"cad"` selection (mirroring the process3d gating).
That broke two existing tests — `context_menu_is_grouped_and_keeps_delete_object_last` and
`context_menu_resolves_labels_from_the_registry`, both of which call the interaction-less twin and
assert `deleteObject` is present and trailing. Menu gating was not in this task's scope and "keep
existing tests green" is, so I reverted it; both tests are green again. The `InteractionView` is now
reachable from `context_menu_with_request_context` if cad ever wants that UX.

## 5. What remains

- **The audits this ticket cites are stale and should be marked so.** `📓️app-generator.md`,
  `📓️app-aggregator.md`, `📓️app-aussuchen.md`, `📓️app-koordinator.md` and
  `📓️explore-2026-09-16-page-and-tests.md` §4 all name "`render()`/`context_menu()` are invoked
  without an `InteractionView`" as the one cross-cutting root cause. That was true when they were
  written; it is not true now. Four of the six apps already threaded it. The residue was the
  `window_engagements` hole (§1.2) and gis (§3.3).
- **process3d's 32 remaining failures are a store/registry migration, not a UI-threading problem** —
  in particular `edit history insertion requires its exact mutation retirement factory` blocks the
  interaction lane from persisting at all, which is what actually keeps the process3d inspection
  panel empty. That is the next thing to fix for `bearbeiten`, and it is a different ticket.
- **cad's 4 failures are the `interactive-job.catalog-authority` tool-proof join** (`generated_migrated=false`),
  already being worked by a peer.
- **`vcs_artifact_app_production_maintenance_swap_is_authoritative_and_fail_closed` hangs** in
  `semio-s-artifact-process-process3d` — it wedged the unfiltered run for 18+ minutes until that run
  was terminated. Every measurement here skips it. It needs its own look; a hanging test is worse than
  a failing one because it makes the whole suite unmeasurable.
- Both cargo jobs I had left resident (pids 92106, 98331) have since exited on their own; nothing of
  mine is still running.

- `semio-s-plugin-writer` / `semio-s-plugin-draw` need a re-check once the peer's `trinity-jack` and
  `draw-drawing` refactors land — their edits here are mechanical parameter additions only.
- The gis map surface now publishes `selection_json`/`hover_json`, but nothing yet publishes a
  `"pointer"`-channel hover for the `"features"` domain from `TiledMapHost`, so `feature_hover_json`
  is exercised only by unit construction until that lane exists.
- `puzzle3d`'s `window_engagements_with_request_context` takes `_interaction` but does not use it —
  its HUD carries no selection-derived row today. If one is wanted, the parameter is now there.
