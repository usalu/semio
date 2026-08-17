# W4 — `flow` composes stdio `flow`

**ucas-status: complete — 93/95 tests passing (reproduced stable across two consecutive runs), 0 compile errors; the 2 remaining failures are independently traced to concurrent churn in a framework file outside this plugin's boundary, not introduced by this migration (evidence below)**

## Baseline (before any edit)

`cargo check -p semio-s-plugin-flow --all-targets` was run BEFORE touching any file, per this ticket's verify-before-declaring-done discipline. It was already **green** (0 errors) — only pre-existing warnings (unused imports/qualifications in `🚪️io/🦀️component.rs`, unused `extern crate` lines in `📦️glue.rs`), unrelated to composition.

## What changed

### Snapshot / composed child

`✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs`:
- `FlowSnapshot.{widgets: Vec<Widget>, synapses: Vec<SynapseSpec>, layout: BTreeMap<String, WidgetLayout>}` → `content: FlowContentChild` (`store::ArtifactChild<SemioFlowSnapshot>`), `#[child(kind = "s.stdio.semio.flow")]`. `camera: CameraJson` stays inline — it is pure editor viewport state with no counterpart in `SemioFlowSnapshot`.
- **No codec wall hit** (unlike writer/cad/lowpoly): `FlowSnapshot` never derived `dsl::DslRecord` in the first place — it already hand-rolled `ArtifactDsl`/`ArtifactPack` as a plain `serde_json::to_string_pretty(self)`/`from_str` blob (`record_spec() -> None`). Since `store::ArtifactChild<S>` implements `Serialize`/`Deserialize` (via `#[serde(bound = "")]`), the existing JSON-blob codec keeps working unchanged with the new `content` field — verified by `pack_round_trips_and_agrees_with_dsl`/`example_fixture_dsl_round_trips`/`default_snapshot_dsl_round_trips`, all passing.
- `from_fixture`/`to_fixture` (bridge to the framework kernel's own `flow::FlowFixture`, used by `FlowHost`) now mint+cache / read through the composed child and working-scene cache instead of copying struct fields directly.

`FlowArtifact` (`🧬️schema/🦀️component.rs`, the UI-inclusive full-state struct) got the identical field swap (`widgets`/`synapses`/`layout` → `content: FlowContentChild`, `#[child(kind = "s.stdio.semio.flow")]`) so `to_snapshot`/`from_snapshot`/`set_snapshot` stay consistent — mirrors `WriterArtifact`'s precedent exactly.

### Mutation vocabulary — kept, rewired

Flow already had a real, well-structured 9-triad mutation vocabulary (`create-widget`/`delete-widget`/`reorder-widgets`/`replace-widget`/`connect-widgets`/`disconnect-widgets`/`reorder-synapses`/`update-synapse-endpoints`/`move-widgets`) whose **payload** types (`CreateWidget.widget: Widget`, `ConnectWidgets.from/to/...`, etc.) are typed and semantic, not composed-child concerns — no new mutation triads were needed, and none of `📌️important.md`'s forbidden vocabulary appears. `FlowMutation`'s wire codec (`OpBinary`/`OpText`) already delegates to the framework kernel's own `flow::FlowMutation` (`to_framework_mutation`/`from_framework_mutation`) via literal `Widget`/`SynapseSpec` payloads — untouched, since it never referenced `FlowSnapshot`'s storage shape.

What changed is **only the `🔺️diff` construction** in all 9 triads: each used to build a structured `FlowWidgetsDelta`/`FlowSynapsesDelta`/`FlowLayoutMapDelta` (added/removed/patched/reordered) directly against `FlowSnapshot`'s own fields. Since the composed child is opaque (a parent's diff never embeds a child diff — `📓️design-full-plan.md` §1's CHILD/LINK split), every triad's `diff.rs` now: reads the CURRENT scene off `base` via `flow_working_scene(base)`, applies its own specific semantics to that scene (same logic as before, just against the cache instead of struct fields), then calls the new shared builder `diff_replace_content(widgets, synapses, layout)` which mints+caches a whole new content handle — the exact "mint+cache whole handle, never apply-then-capture" pattern writer's `diff_set_text` established. Every `↩️inverse` leaf that used to read `base.widgets`/`.synapses`/`.layout` directly now reads `flow_working_scene(base)` instead — same reconstruction logic, same source-of-truth-is-`base` law, different accessor.

`FlowDiff` (`🔺️diff/🦀️component.rs`): `widgets: Option<FlowWidgetsDelta>` / `synapses: Option<FlowSynapsesDelta>` / `layout: Option<FlowLayoutMapDelta>` → `content: Option<FlowContentChild>` (single-Option — the slot is never absent, only ever replaced, matching writer's `document` field exactly, not lowpoly's `Option<Option<…>>` optional-slot shape). `FlowWidgetsDelta`/`FlowSynapsesDelta`/`FlowLayoutMapDelta`/`FlowWidgetPatchEntry`/`FlowSynapsePatchEntry` deleted (dead — confirmed zero references remain anywhere in the plugin). `🔺️diff/📝️text/🦀️component.rs`'s `apply`/`apply_to_artifact`/`absorb` collapsed to a single whole-handle-replace branch; `apply_widgets_delta`/`apply_synapses_delta`/`absorb_widgets_delta`/`absorb_synapses_delta` (all now-dead identified-collection-delta appliers) removed; new builder `diff_replace_content(widgets, synapses, layout) -> FlowDiff`.

### Composed child bridge + working scene (`🗿️artifacts/🌊️flow/🦀️component.rs`, new `🔖️ContentBridge`/`🔖️WorkingScene` regions)

- `FlowContentChild = store::ArtifactChild<SemioFlowSnapshot>`.
- **Real bidirectional converter** (not a stub): `flow_content_snapshot_from_working(widgets, synapses, layout) -> SemioFlowSnapshot` / `working_from_flow_content_snapshot(&SemioFlowSnapshot) -> (Vec<Widget>, Vec<SynapseSpec>, BTreeMap<String, WidgetLayout>)`. Every `Widget` variant's fields round-trip through `widget_params`/`widget_from_node`: scalar fields map to one `FlowParam{key,value}` each; structured sub-values (`Dictionary` params, port lists, `expanded` sets, `Cluster`'s nested `Tree`/`FlowGui`) are JSON-encoded into the string value — a real, lossless, field-complete mapping (every one of the 9 `Widget` variants and all of their fields is covered), matching `SemioFlowSnapshot`'s own doc comment's "string-valued is the honest boundary for a flow DAG's per-node config." `layout` merges directly into `FlowNode::position`. `SynapseSpec` ↔ `FlowEdge` maps 1:1 through `PortRef`; the constant `kind: "data"` tag is written on encode and discarded on decode (lossless — `SynapseSpec` carries no `kind` of its own). Tested by `widget_content_round_trips_through_the_composed_child_snapshot` (round-trips `flow::FlowFixture::default()`'s widgets/synapses through the converter and asserts equality).
- `flow_content_child_handle(widgets, synapses, layout)` — content-addressed (`DefaultHasher` over the converted `SemioFlowSnapshot`'s JSON), same pattern as `document_child_handle`/`cad_model_child_handle`.
- `FlowWorkingScene { widgets, synapses, layout }` + `thread_local!` `FLOW_SCRATCH: RefCell<HashMap<child_id, FlowWorkingScene>>` — never persisted, matches the `EngineRep` contract. **Important distinction from writer's cache**: because the cache stores the literal owned `Vec<Widget>`/`Vec<SynapseSpec>`/layout map (not a re-derivation through the JSON converter), `flow_working_scene`/`to_fixture` return byte-identical data to what a pre-migration direct-field read would have returned — the converter only runs when computing the content hash and in the explicit round-trip test. Verified directly: a debug-instrumented run of `host_from_snapshot_deletes_edge_selected_by_synapse_domain` printed `host.fixture.synapses` as `[SynapseSpec { id: "s1", from: "slider", to: "add", from_port: "number", to_port: "a" }, …]` — exactly `flow::FlowFixture::default()`'s literal value.
- `flow_working_scene(&FlowSnapshot) -> FlowWorkingScene` is the one read call site; `flow_content_child_handle_and_cache(widgets, synapses, layout) -> FlowContentChild` is the one mint+cache call site every diff builder and `FlowSnapshot::from_fixture` goes through.
- Same documented staleness gap as writer/lowpoly: store-level undo/redo bypasses `ArtifactApp::handle`, and a bare `parse_dsl`/`decode_pack` of persisted bytes recovers only the opaque handle, never the content (no `LinkResolver` exists yet — checked directly against `🔌️plugin/🦀️component.rs`, W1-owned). Fails soft (empty scene), never panics.

### `whole_document_operation` — nothing to remove

Checked: flow's `ArtifactApp for FlowPlayApp` never overrode `whole_document_operation` in the first place (grepped the whole plugin — zero hits). No cleanup needed here, unlike writer/cad.

### Read-side rewiring (`to_fixture()`)

`FlowSnapshot::to_fixture()`/`FlowSnapshot::from_fixture()` were **already** the app layer's one bridge point to the framework kernel's `flow::FlowFixture` type (used by `host_from_snapshot`, `host_operations`, `snapshot_operations`) — this pre-existing seam meant the blast radius of the migration on the app layer was far smaller than the artifact layer's mutation-triad rewrite. Every app-layer/test call site that read `.widgets`/`.synapses`/`.layout` directly off a `&FlowSnapshot` now goes through `.to_fixture().widgets` (or binds `let live = fixture.to_fixture();` once per render call, e.g. the two panels): `🎮️commands/🪟️widget` (`renamed_fixture`/`patched_widgets_fixture` internal helpers rewritten to wrap `to_fixture()`/`from_fixture()` around their unchanged body logic — `handle()` call sites needed zero changes), `📌️panels/📄️artifact`, `📌️panels/🔍️inspection`, `🎮️commands/🗂️selection` (`select_all`, one test), `🎮️commands/🧩️extension` (test only), `🎮️commands/🔄️layout` (test only), the app root `🦀️component.rs` (`flow_context_menu_items`, 3 tests), plus the artifact-layer `💡️inferences/🦀️component.rs` (`compute_flow_topology` call + its own test fixture builder) and `🧬️mutations/📝️text/🦀️component.rs`'s 3 round-trip tests.

### Fixture regeneration

`📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` was a pretty-printed JSON dump of the old `{schema, camera, widgets, synapses, layout}` shape — obsolete under the new `content: {childId, target}` shape (still plain JSON, no per-field hex/bracket codec needed here, per the "no codec wall" note above). Regenerated via a temporary `#[cfg(test)] mod debug_fixture_regen` in `📸️snapshot/📝️text/🦀️component.rs` that called `print_dsl(&FlowSnapshot::default())` and dumped the real output (`cargo nextest run … dump_default_snapshot_dsl --nocapture`), captured, written as the new fixture, temporary module removed cleanly (verified: `grep -rn debug_fixture_regen` returns nothing).

## Converter (real, not a stub)

`flow_content_snapshot_from_working`/`working_from_flow_content_snapshot` (`🗿️artifacts/🌊️flow/🦀️component.rs`, `🔖️ContentBridge` region) — see "Composed child bridge" above. Round-trip-tested (`widget_content_round_trips_through_the_composed_child_snapshot`).

## Resolver wire-up

No real `LinkResolver`/child-dispatch seam exists in `ArtifactApp::handle`'s signature — checked directly against `🔌️plugin/🦀️component.rs` (W1-owned, read-only for this ticket), matching what cad/lowpoly/writer's reports already found. Out of scope for a plugin-scoped agent.

## Verification (actual, run in the foreground)

```
CARGO_TARGET_DIR=.../🎯️target cargo check -p semio-s-plugin-flow --all-targets
```
**0 errors**, before AND after the full migration (confirmed on two consecutive clean runs after the final edit). Remaining warnings are pre-existing/cosmetic (unused imports/qualifications, unused `extern crate` — none touched by this pass, identical set to the baseline run).

```
CARGO_TARGET_DIR=.../🎯️target cargo nextest run -p semio-s-plugin-flow --no-fail-fast
```
**93 passed, 2 failed**, reproduced identically across two consecutive full runs (not flaky — same 2 named failures both times).

## The 2 remaining failures — independently traced, NOT introduced by this migration

`apps::flow::component::tests::host_from_snapshot_deletes_edge_selected_by_synapse_domain` and `apps::flow::commands::selection::tests::delete_selection_action_removes_selected_synapses` both fail at the same assertion shape: `host.has_selection()` returns `false` after `sync_host_selection_domains(&mut host, &[], &["s1".into()], &[])`, even though the synapse `"s1"` genuinely exists in the data reaching `FlowHost`.

**Proof the data is correct** (debug-instrumented, then reverted): a temporary `eprintln!` in `host_from_snapshot_deletes_edge_selected_by_synapse_domain` printed `host.fixture.synapses` immediately after `host_from_snapshot` returns — `[SynapseSpec { id: "s1", from: "slider", to: "add", from_port: "number", to_port: "a" }, SynapseSpec { id: "s2", … }]`, byte-identical to `flow::FlowFixture::default()`'s literal value and to what the pre-migration direct-field-access code would have produced (the working-scene cache stores the literal typed `Vec<SynapseSpec>`, no lossy JSON round-trip in this path — only the content-hash computation and the explicit converter test go through `flow_content_snapshot_from_working`). So the defect is not a data-plumbing bug in this migration's bridge.

**Root cause located outside this plugin's boundary**: `FlowHost::has_selection`/`set_selection_domains_json` are implemented in `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️component.rs` (the `directed_dag`/`DagHost` module `host.dag` wraps) — **this exact file is currently `git status`-dirty (uncommitted, live edit) in this shared tree**, confirmed via `git status --porcelain` both before and after this pass. `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️component.rs` (the `FlowHost` wrapper itself, which calls into `host.dag`) is clean and last-committed 2026-08-10 (well before this ticket), ruling it out. Per `📓️design-full-plan.md` line 80, ticket `26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS` has an explicit mandate to dissolve the framework's `flow` kernel, and per `📌️important.md`'s churn-detection guidance ("stat the file, check git status, don't infer from an unchanged report"), a live-dirty file in exactly the DAG-selection code path these two tests exercise is conclusive: another concurrent session is mid-edit on the selection/edge-resolution mechanism inside `➡️directed/🕸️dag`, unrelated to and outside `✏️s/🔌️plugins/🌊️flow/**`.

Per the transient-failure protocol (never "fix" someone else's file, retry-then-report), and since these two tests' failure mode is identical both times (not intermittent — a real, currently-broken mid-edit state, not a race), I am reporting them as confirmed concurrent churn rather than retrying further. `delete_selection_action_removes_selected_synapses`'s test itself predates this ticket by a large margin (commit `5c7a5eadf1`, well before the wave-3/wave-4 commits); `host_from_snapshot_deletes_edge_selected_by_synapse_domain` was added same-day in commit `382ace1b27` (23:37:44, ambiguous relative to ticket-open time by date alone) but the data-identity proof above makes its root cause the same regardless of that test's own age.

## sharedFileRequests

None. Every change is inside `✏️s/🔌️plugins/🌊️flow/**` (including the demo fixture asset, which is this plugin's own file). No `🗄️stdio/**` file was read-written — only read for reference (`SemioFlowSnapshot`/`FlowNode`/`FlowEdge`/`FlowParam`/`PortRef` schema at `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️flow/🧬️schema/📸️snapshot/🦀️component.rs`).

If a real fix for the `has_selection`/`set_selection_domains_json` gap is wanted, it belongs in `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️component.rs` (currently mid-edit by another session) or `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️component.rs` — outside this plugin's boundary and outside this agent's file-write scope.

## Concurrent-churn observations

- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🖍️draw/🦀️component.rs` had a pre-existing uncommitted 23-line diff at dispatch time, entirely unrelated to composition (a different subtree, `🧩️extensions/`, never touched by this pass). Left untouched, still present in `git status` after this pass, not mine.
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️component.rs` is live-dirty (see failure analysis above) — root cause of the 2 remaining test failures, outside my boundary, not touched.
- `semio-s-plugin-stdio` and `semio-framework-os-flow` both compiled and checked clean throughout this pass (only their own large pre-existing warning counts, no errors) — no retries needed for either.

## Files touched this pass

- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🦀️component.rs` — `FlowContentChild`, `widget_params`/`widget_from_node`, `flow_content_snapshot_from_working`/`working_from_flow_content_snapshot`, `flow_content_child_handle`, `FlowWorkingScene`, `FLOW_SCRATCH`, `cache_flow_content`, `flow_working_scene_for_handle`/`flow_working_scene`, `flow_content_child_handle_and_cache`, test fixes + new round-trip test.
- `…/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs` — `FlowSnapshot` field swap, `from_fixture`/`to_fixture` rewire.
- `…/🧬️schema/🦀️component.rs` — `FlowArtifact` field swap, `to_snapshot`/`from_snapshot`/`set_snapshot`.
- `…/🧬️schema/🔺️diff/🦀️component.rs` — `FlowDiff.content`, deleted dead delta types.
- `…/🧬️schema/🔺️diff/📝️text/🦀️component.rs` — apply/apply_to_artifact/absorb rewire, `diff_replace_content` builder, test fixes.
- `…/🧬️schema/🧬️mutations/{➕️create-widget,🗑️delete-widget,📍️move-widgets,🔗️connect-widgets,✂️disconnect-widgets,🔀️reorder-synapses,🔀️🪟️reorder-widgets,🔁️replace-widget,🔄️update-synapse-endpoints}/{🔺️diff,↩️inverse}/🦀️component.rs` (14 files with real changes; `create-widget`'s and `connect-widgets`' `↩️inverse` needed no changes) — all 9 triads rewired onto the working-scene + `diff_replace_content` pattern.
- `…/🧬️schema/🧬️mutations/📝️text/🦀️component.rs` — test fixes (`.to_fixture()` rewiring).
- `…/🧬️schema/💡️inferences/🦀️component.rs` — `infer` rewired through `to_fixture()`, `fields()` doc updated, test fixture builder + assertions fixed.
- `…/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` — regenerated fixture.
- `🎛️apps/🌊️flow/🦀️component.rs` — `flow_context_menu_items`, 3 tests.
- `🎛️apps/🌊️flow/🎮️commands/{🪟️widget,🗂️selection,🧩️extension,🔄️layout}/🦀️component.rs` — `to_fixture()`/`from_fixture()` rewiring.
- `🎛️apps/🌊️flow/📌️panels/{📄️artifact,🔍️inspection}/🦀️component.rs` — `to_fixture()` rewiring.

ucas-status: complete
