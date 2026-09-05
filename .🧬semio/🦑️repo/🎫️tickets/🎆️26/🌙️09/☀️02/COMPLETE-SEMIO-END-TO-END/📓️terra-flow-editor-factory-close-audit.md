# Flow editor factory full-close audit

Status: source review only, 2026-09-04. No Cargo/Nx command was run; the native slot remains owned elsewhere.

## Superseded initial gap: no editor factory law

`child-identity-check` currently exact-selects six laws. All names are current and the sixth is the real viewer factory close. The first five are, respectively, Flow content identity/projection, isolated durable-store owners, isolated presence owners, and isolated transient ownership. None constructs `crate::plugin()`, selects `FlowApps::FlowEditor`, or drives that actual editor app to terminal close.

Consequently, the then-six-law gate did **not** prove the editor's full lifecycle. It must not be described as a Flow editor factory pass. This is historical: the live tree now contains a seventh registered production-factory law described below.

Source anchors:

- Exact current six-FQN selection: `✏️s/🔌️plugins/🌊️flow/📦️packages/🦀️rust/📜️script.ts:27-41`.
- The only current factory-close law, for the viewer: `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:124-155`.
- Real plugin editor registration: `✏️s/🔌️plugins/🌊️flow/🦀️.rs:24-35`.

## Current source wiring

The editor source itself supplies all currently required owner hooks:

- document: shared Flow snapshot/mutation owners and `ArtifactDocumentStoreDisposer`;
- config: Flow retained config owners plus bounded config disposer;
- draft: bounded `NoDraft` owners/disposer;
- presence: Flow local/peer retirement factories and the Flow presence-store disposer;
- transient: `NoTransientStoreDisposer`;
- app-instance state: `FlowInstanceOperationOwner`, which starts with a real `FlowEvalSession` and removes it only after its terminal close witness.

These are not five interchangeable lanes: `VcsArtifactApp` closes document, config, draft, presence, transient, and framework interaction as six ordered stages. It also drains the instance-operation owner before the six store stages can finish. `close_terminal_is_empty` requires every disposer slot, tool/operation registry, returned snapshot-read pump, interaction state, and instance owner to be empty.

Source anchors:

- Editor hooks and retained tool registration: `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1576-1705`.
- Instance owner and its evaluation-session close transition: `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1523-1569`.
- Evaluation session has an explicit-drop guard and bounded retirement state machine: `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs:2287-2311,2437-2516`.
- Framework close stages and final witness: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:23678-23725`.

I found no current source/API inconsistency in these hook signatures. The source is nevertheless unexecuted in combination.

## Bounded factory law required

Add a seventh exact law beside the editor, then exact-list it in the same `child-identity-check` script after the source fixture/oracle import. It should:

1. Call `crate::plugin()` and obtain the editor definition id from `create_flow_app()`; use `Plugin::create_app`, not `VcsArtifactApp::new`.
2. Require `FlowApps::FlowEditor`, editor role, and the expected document write capability. This keeps the test at the registered production factory boundary.
3. Drive `PluginApp::close_step(1, 4096)` to `Complete`, reject an unexpected `Blocked`, and check every `Pending` report is bounded by that same grant.
4. Require `close_terminal_is_empty` after `Complete`. This is what proves the default `FlowEvalSession` (neural cache, default status string and collections), six store lanes, and interaction store were all actually retired rather than silently dropped.

The default Flow parent is a material fixture, not an empty placeholder: it carries a slider, neuron, preview and two synapses. It is sufficient for this fresh-factory close law; a separate composed-reload law must add the Semio child member and its full envelope lifecycle.

Source anchor: default fixture contents at `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifact/🦀️.rs:318-334`.

## Live seven-law update: source-sound, native terminal pending

The live `child-identity-check` now exact-selects the seventh law
`flow_actual_surface_factories_close_all_owners_under_neutral_grants` after the six pre-existing unique FQNs at `✏️s/🔌️plugins/🌊️flow/📦️packages/🦀️rust/📜️script.ts:27-43`. The law calls the actual `crate::plugin()` builder, checks the ordered two-entry manifest, creates every registered definition through `Plugin::create_app`, requires the matching `FlowApps::{FlowEditor,FlowViewer}` variant, and drives each fresh app under all three neutral byte grants (1, 64, 4096): `✏️s/🔌️plugins/🌊️flow/🦀️.rs:47-77`.

The initial zero-grant concern is source-closed: after reaching terminal empty, the law correctly expects `Pending { released_items: 0, released_bytes: 0 }` for `close_step(0, 0)` at line 72, matching `VcsArtifactApp::close_step`'s required zero-item no-progress rule at `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:23204-23207`; it then supplies the normal grant and requires `Complete` at line 74. The fresh app is not reused across grants, and unexpected `Blocked`, over-budget progress, incomplete close, or a false terminal witness all fail the law.

The language-neutral `surface-owners` JSON/AJV oracle is also no longer orphaned from the Semio member identity. It requires the exact package, ordered editor/viewer roles, `s.stdio.semio@v1/flow`, grants, and terminal result (`✏️s/🔌️plugins/🌊️flow/🧪️fixtures/🧹️surface-owners/{🔣️.json,🧬️.schema.json}`); its source oracle separately requires both exact `editor_with_members::<FlowPlayApp, SemioMembers>` and `viewer_with_members::<FlowViewer, SemioMembers>` bindings and rejects altered member identity (`✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️fixtures/📜️script.ts:361-376`).

No Cargo command was run by this audit. The active registered native run is still the only runtime evidence candidate, so this is **source-sound / runtime pending**, not a pass. The law validates fresh default factories and their six store stages plus instance closure; it intentionally does not establish retained child reload, an externally held document/interaction reader, or a populated Flow tool/host operation. Those require the separate public retained `MemberOpenRequest` integration and operation/reader laws described below.

## Retained and reader boundaries

The proposed fresh-editor law proves the instance operation owner is closed, but it does not prove every exceptional retained route:

- Flow's direct and host-only tool factories own paged input/decoders and have their own close/terminal implementations. They are only populated after a real owned tool operation, so a fresh editor test cannot cover them.
- The framework closes document snapshot-read returns through a bounded pump and deliberately returns `Blocked` if an outside lease remains live. Existing Flow presence-owner coverage exercises a reader in isolation; no current Flow editor factory law holds a document or interaction read across close.
- A busy `ArtifactInstanceOperationOwnerHandle` returns a fail-closed framework fault through `try_lock`; a correct test must not assume this is a successful close.

Source anchors:

- Direct retained factory and bounded raw input admission: `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1183-1256`.
- Host-only retained job close/terminal state: `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1292-1398`.
- Bounded document snapshot-read close and live-lease failure: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:23678-23689`.
- Owner handle's nonblocking/fail-closed lock: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:12719-12744`.

Follow the fresh editor-factory law with one focused retained-operation/reader law only once the host exposes a public production invocation that actually admits a Flow direct or host-only job. It must hold the reader/operation through the first close attempt, assert the documented blocked-or-fault outcome without false terminal success, release or cancel it through its owner, and then prove exact terminal empty. A synthetic hand-built job would not validate the app runtime boundary.

## 2026-09-04 compiler-repair reread: current RED

The historical registered `flow-child-identity-exact` capture at
`🗑️generated/flow-child-identity-exact/exact-cargo-laws-7xS1B3/00/build.stdout`
ended with 211 Rust compile errors, so none of the seven selected lifecycle laws
executed. Its old broken `protocol::testkit` import and flat leaf paths are now
source-closed: the current mutation suite imports the asynchronous
`protocol::os_spr::testkit` helpers and current direct leaves no longer use the
nonexistent `create_widget::mutation`, `move_widgets::mutation`, etc. paths.
The external-`Widget` serde derives that caused the captured `E0277`s were also
removed; the only remaining Flow `cfg_attr(test, serde...)` is the local
`FlowStringList { values: Vec<String> }`, whose fields are first-party scalar
data.

There is, however, a new current compile blocker in the attempted async cleanup.
The Flow testkit made its VCS-facing helpers synchronous:

- `dispatch` / `dispatch_with_registry` at
  `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:2120-2126`
  invoke `VcsArtifactApp::dispatch_typed`, which returns `impl Future`, not a
  `Result` (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:22785-22794`).
- `render`, `main_window_measures`, `select_graph`, and the context-menu helper
  call the async `PluginApp` VCS facade. Its concrete `render`,
  `window_measures`, `context_menu`, and `handle_action` implementations are
  async at `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:24730-24860`.
  Their currently direct `.expect(...)` / serialization calls cannot typecheck.

Only `AppActionRegistry::from_definition` and `Plugin::create_app` are
synchronous (`…/🔌️plugin/🦀️.rs:11989,25118-25120`). The clean correction is to
retain synchronous assembly/factory use, but restore async helpers and `.await`
at every VCS-facing Flow test caller. This is not a lifecycle-law semantic
change and must not be papered over with a blocking bridge. Until that lands and
a fresh registered terminal is available, the seven-law lifecycle scope remains
**RED / no native assertion evidence**.

### Post-repair API residual

The VCS helpers and sampled downstream callers have since been restored to
`async`, which matches the concrete framework signatures. One separate current
type error remains: `PluginApp::handle_action` accepts `Option<&DslValue>`, not
`serde_json::Value` (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:11708,24314`).
`select_graph` still passes `Some(&serde_json::json!(…))` at Flow editor
`🦀️.rs:2148`; the public duplicate-widget action tests do the same at their
starts/continuations in
`✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📋️duplicate-widget/🦀️.rs:363,396,401,403,409,432,434,449,451,459`.
They must create a named first-party `DslValue` before the await (the framework
testkit itself uses `DslValue::from(&serde_json::Value)` at
`…/🔌️plugin/🦀️.rs:34456-34463`). This preserves the public action-bus test:
the live `dispatch_action` explicitly routes both framework-reserved interaction
verbs and registered command JSON through their validated admissions
(`…/🔌️plugin/🦀️.rs:22013-22043`). It is a type repair, not a reason to replace
the test with a typed-command shortcut.

### Current reread after the targeted repairs

The testkit now makes precisely the VCS-facing helpers async and awaits them at
their downstream test call sites; pure `every_command`, selection-data
construction, schema fixtures, `AppActionRegistry::from_definition`, and
`Plugin::create_app` remain synchronous. The expected generic Flow VCS facade
split is therefore restored.

The action-argument repair is also source-valid. `DslValue` deliberately
implements both `From<&serde_json::Value>` and `From<serde_json::Value>` at
`🧰️framework/🔨️modules/🌱️value/🦀️.rs:247-276`. Current `select_graph` gives
its converted argument a name before the await; the duplicate-widget public
action tests use the owned conversion in an immediate awaited statement. No
Flow `handle_action` call now passes a raw `serde_json::Value`. This does not
restore a serde derive on Flow app types, and the action calls still pass the
first-party `DslValue` required by the production API.

I found no additional concrete API/type mismatch in the seven exact lifecycle
laws on this reread. This is **source-qualified only**: the in-progress
metadata/build work has no terminal result and must not be represented as a
native lifecycle pass.

One non-semantic cleanup remains in the surface law: `AppRole` is currently
exhaustive over `Editor` and `Viewer`, so the wildcard arm in the role mapping
at `✏️s/🔌️plugins/🌊️flow/🦀️.rs:52-54` is an `unreachable_patterns` warning
(also recorded in the historical capture). It does not alter the factory law,
but should be removed before a warnings-as-errors target is used.

## 2026-09-04 metadata-frontier rereread — current test-helper RED

Coordinator evidence says metadata run `96604` stopped at Flow with twelve
diagnostics in four compile-shape families: serializing the actual render root,
implicit `UiFixedList<BuiltNode>`, mixing the window declaration surface enum
with the UI contract scene enum, and passing an `Arc` snapshot where
`ArtifactView` needs the snapshot referent. It is a compile RED, not a Stdio
failure and not a native lifecycle result.

The current tree source-closes three of those shapes:

- render-time scene construction uses UI-contract `ContractSurfaceKind`, while
  the manifest keeps plugin `SurfaceKind`
  (`…/windows/🌊️main/🦀️.rs:9-10,23-39,83-108`);
- the heterogeneous form children have explicit `UiFixedList::<BuiltNode>`
  (`…/windows/📝️form/🦀️.rs:122-143`);
- the retained host path passes `payload.snapshot.as_ref()` to `ArtifactView`
  (`…/editor/🦀️.rs:1336-1344`).

The purported serialization shape repair is still a deterministic test/runtime
RED. `editor::flow::testkit::render` serializes `tree.root` directly
(`…/editor/🦀️.rs:2130-2132`). That avoids needing `ComponentTree: Serialize`,
but `BuiltChildren::Serialize` intentionally rejects every nonempty child page
(`🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🏗️builder.rs:325-332`).
Flow's document and catalogue bodies produce section children
(`…/panels/🗿️artifact/🦀️.rs:37-47`, `…/panels/🛍️catalogue/🦀️.rs:65-89`), so
the current tests calling this helper cannot prove those populated bodies and
will leave their retained child pages to the thread-local retirement authority
without a test drain.

The exact existing test-only repair is
`✏️s/🔌️plugins/📕️norm/🖥️app-surface/🧪️tests/🦀️.rs:31-56`: manually walk
`BuiltNode.children`, assert rejected children are empty and sibling keys are
unique, serialize only each typed `component`, enforce depth 64 and
`UI_BUILT_CHILD_RETIRE_SLOTS` bounds, drop the full tree even on panic, then
drain through the public
`semio_framework_ui_contract::close_built_node_page_one()` API
(`…/builder.rs:142-146`). Flow should reuse this test-only projection and a
neutral projected-shape fixture (body key, root/component, ordered child keys),
not add a generic production serializer or use `ComponentTreeProducer` merely
to inspect the tree. Layout/main/compiled/preview tests should then assert the
typed projected scene properties rather than JSON substring checks.

This is source-only diagnosis. No Cargo/Nx command was run by this audit and
there is no native acceptance.

## 2026-09-04 bounded Flow render projection map

The current-tree reusable API is deliberately small and test-facing:

- [`BuiltNode.children`](../../../../../../../../🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🏗️builder.rs:349) is iterable without serializing the retained backing page;
- [`close_built_node_page_one`](../../../../../../../../🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🏗️builder.rs:142) retires one queued child node and reports terminal emptiness;
- `BuiltChildren::Serialize` expressly rejects populated pages
  ([same builder](../../../../../../../../🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🏗️builder.rs:325)). There is therefore no safe generic production serde route for a complete component tree.

The existing Norm test helper is the correct exact pattern, not merely an
analogy: [`project_node` and `project_and_retire`](../../../../../../../../✏️s/🔌️plugins/📕️norm/🖥️app-surface/🧪️tests/🦀️.rs:31) take an owned `ComponentTree`, walk `BuiltNode.children`, assert no rejected children and unique sibling keys, project only the typed `component`, impose depth `< 64` plus the public `UI_BUILT_CHILD_RETIRE_SLOTS` node cap, and use `catch_unwind` to ensure the tree is dropped before replaying an original panic. The bounded loop ends as soon as the public close API reports terminal empty. Flow should use that pattern verbatim in a test-only helper; `ComponentTreeProducer` has no role after an already-built tree is returned.

The Flow helper remains unfixed in the current bytes: [`testkit::render`](../../../../../../../../✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:2130) still directly serializes `tree.root`. Tests that must change from a JSON substring to typed projection assertions are:

- editable main scene: [`main`](../../../../../../../../✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌊️main/🦀️.rs:124), currently `contains("node-graph")`;
- compiled DAG: [`compiled`](../../../../../../../../✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🗣️compiled/🦀️.rs:52), currently `contains("text-editor")`;
- generated preview: [`preview`](../../../../../../../../✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🧬️generate/🪟️windows/👁️preview/🦀️.rs:52), likewise a text-editor substring;
- three-surface app integration: [`editor tests`](../../../../../../../../✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:2392), which presently checks action and label text plus the preview substring.

The replacement fixture should constrain the public projected tree shape
(`bodyKey`, root key, typed component discriminant/surface kind, ordered child
keys), while direct unit tests retain their domain facts: main must be
`NodeGraph`, compiled/preview must be `TextEditor`, and their editable/read-only
flags and typed scene properties must be destructured from the component rather
than inferred from labels. That preserves layout and preview assertions without
turning localized labels or JSON formatting into a surface contract.

The exact typed surface assertion is available without inventing a projection
schema. `scene_surface` creates `Component::Surface`
([plugin assembly](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:347)); after matching that component, tests should assert
`SurfaceProps.kind` and call `semio_framework_ui_scene::decode::<T>(props)`.
That decoder checks `doc_schema` before its typed pack decode
([scene codec](../../../../../../../../🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust/🌉️surface.rs:74)).
The main test can therefore decode `ui_wgpu::wgpu::NodeGraphScene` and assert
`editable == Some(true)` plus its typed graph fields; compiled and preview can
decode `TextEditorScene` and assert their typed language/value semantics. The
production builders make those intended shapes explicit
([main](../../../../../../../../✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌊️main/🦀️.rs:95),
[compiled](../../../../../../../../✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🗣️compiled/🦀️.rs:40),
[preview](../../../../../../../../✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🧬️generate/🪟️windows/👁️preview/🦀️.rs:42)).
Perform that assertion before moving the owned tree into the bounded
project-and-retire helper.

This remains a current source repair map only. It does not alter the `96604`
compile RED or provide native render/lifecycle evidence.

## 2026-09-04 shared fixture projection — direct serde superseded; retirement-bound RED

The direct Flow `serde_json::to_value(&tree.root)` failure described above is
now source-closed. Flow's test helper delegates to the shared
`plugin::testkit::project_and_retire_fixture_tree`
([plugin testkit](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:6707)),
and the shared observer serializes a component at a time while observing only
the fixture key/component/child shape. It is intentionally not a production
tree codec, and it does **not** prove scene decoding, layout, accessibility,
bindings, menu semantics, or actual renderer output. The existing typed scene
recommendation above remains the required test for those facts.

Current source still has a deterministic fixture-lifetime RED. The observer
returns at the first `rejected_children` check or early duplicate-key check
([same testkit](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:6692-6701)),
then drops the entire tree but attempts only 8,192 global retirement turns
([same file](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:6712-6715)).
That is not a bound on retained structure: the UI authority allows 384 child
pages of 32 nodes each ([builder](../../../../../../../../🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🏗️builder.rs:36-37)), and
`close_built_node_page_one` needs one turn for each node plus one terminal
page-release turn ([same builder](../../../../../../../../🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🏗️builder.rs:107-145)). An early structural
rejection can therefore leave populated pages beyond the observation limit and
cannot be proven terminal by the old drain cap.

The source-correct repair now being staged is a fixed close bound
`UI_BUILT_CHILD_RETIRE_SLOTS * (UI_BUILT_CHILDREN_MAX + 1) = 12,672`, not an
unbounded cleanup loop. Its hostile fixture must reserve 383 full normal pages
and one rejected-child page: `383 × (32 nodes + 1 page release) + (1 node + 1
page release) = 12,641` close turns. The native law must first manually
establish that exact count (and that it exceeds 8,192), then recreate the same
tree, require the observer to return `rejected-children`, and immediately
observe terminal emptiness. That proves the observer cannot leak fixed global
retirement authority on an early rejection. The present two positive and two
small structural fixture rows do not establish this yet.

This is a source-level test-helper defect only. No native terminal for the new
projection law has been observed by this audit.

### Current-byte review of the bounded repair

The landed repair implements the required fixed bound exactly:
`FIXTURE_TREE_RETIRE_STEPS = 384 × (32 + 1) = 12,672`
([shared testkit](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:6688-6715)).
The Flow adversary is breadth-bounded rather than a misleading deep recursive
chain: one root normal page, 32 middle pages, 350 full grandchild pages, and
one rejected page. Its `32 + 1,024 + 11,200 + 1 = 12,257` child nodes plus 384
page releases require precisely 12,641 turns
([Flow fixture law](../../../../../../../../✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:2135-2193)).
The law first counts that full close, verifies it exceeds 8,192, recreates the
tree, then requires the early `rejected-children` result and terminal
emptiness. The neutral fixture pins every arithmetic value
([fixture](../../../../../../../../✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️fixtures/🖼️tree-projection/🔣️.json:2-6)). No boundedness or ownership defect remains in this exact construction.

Coordinator source evidence records the oracle/diff-hygiene source target
`86066` green. It is not a Rust/native terminal; the combined projection and
seven lifecycle laws remain unrun in the active metadata target. The exact
count law is deterministic when it owns the process-global child-retire
authority. The normal registered broad runner uses cargo-nextest: its
scheduler runs test cases in separate test processes, so each has an
independent `LazyLock` authority; no serialisation change is needed there.
`runExactCargoLaws` likewise invokes the one selected test with
`--test-threads=1`. The caveat applies only to the explicit no-nextest fallback
in `runCargoTestBudgeted`, which calls a single parallel `cargo test` libtest
process without those thread arguments. In that fallback a saturating test can
race unrelated tree tests and perturb its construction/count. Do not treat
that fallback as equivalent broad evidence unless it is made serial or all
global-authority users share a test boundary.

## 2026-09-04 contract-owned tree retirement — fixed stack and exact-owner RED

The preceding 12,672-turn global-page drain is not a complete tree-lifecycle
proof. It drains only the global `BuiltChildren` handback queue after
`drop(tree)`; it does not retain and close the tree's ordinary and rejected
child pages itself, and it does not close `UiValue` descendants in component,
binding, or menu fields. This supersedes any reading of the earlier fixed
global drain as a full tree-owner acceptance.

The correct production seam is a contract-owned `BuiltTreeRetirement` which
owns one `BuiltNode`, one `UiTypedRetirementCursor`, and a fixed heap-backed
stack of `BuiltChildrenIntoIter` pages. The stack must be sized to
`UI_BUILT_CHILD_RETIRE_SLOTS` (384), not to the 64-node observation limit.
Each nonempty iterator still owns exactly one of the 384 reservations made by
`BuiltChildren::try_push`; a structurally rejected chain can keep all 384
pages live. A node with both ordinary and rejected children consumes two page
entries, so the same page bound remains sufficient. Allocate the stack with
the exact heap capacity—`Vec::with_capacity(384)`, `resize_with`, then
`into_boxed_slice`—rather than first materialising a large fixed array on the
native call stack or allowing a growing vector.

The close state must retain only page iterators, not parent nodes. It first
uses a private `UiTypedRetire for BuiltNode` over all nine nonstructural fields:
`key`, `component`, `layout`, `style`, `activity`, `disabled`,
`accessibility`, `bindings`, and `menu`. It then moves ordinary and rejected
children into the page stack, drops the typed-empty node, and processes the
ordinary iterator before rejected (push rejected first for LIFO order).
`UiTypedRetirementCursor` already reaches typed component, binding, and menu
`UiValue` descendants; the tree owner must not add bindings to the structural
projection JSON merely to close them.

There is a non-obvious iterator invariant. Calling `next()` once more after
the final child is mandatory: that final `None` releases the iterator's exact
handback. Testing `len() == 0` and dropping it instead publishes the remaining
page into the process-global authority
([builder](../../../../../../../../🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🏗️builder.rs:274-305)). The contract needs a private nonblocking
terminal-release operation—`try_lock` only when that final `None` must release
the handback—so contention returns no progress while retaining the exact
iterator. It must not call the current blocking mutex path from a close turn.

The nearest existing runtime cursor is deliberately not reusable:
`SurfaceTreeRetireCursor` has a 4,097-page heap stack, ignores
`rejected_children`, does not retire typed node fields, and overflows by
dropping an iterator into the global authority
([reconcile](../../../../../../../../🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/♻️reconcile.rs:2327-2383)). It is evidence for the exact iterator
hazard, not an implementation template.

Finally, this owner cannot honestly claim safe early `Drop` with the existing
cursor. A nonterminal `UiTypedRetirementCursor` may own a
`UiValueRetirement`, whose destructor hands back a root and then panics
unconditionally ([value retirement](../../../../../../../../🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/🦀️.rs:31-119)). The outer `UiPendingPatch`-style
`if !panicking()` guard does not prevent that inner panic during unwinding.
P0 must therefore be a strict terminal owner: observer error or caught panic
keeps driving this owner to terminal before returning/resuming, and a
contended turn simply retains it for retry. A future early-abandon route needs
its own private whole-tree handback and explicit `UiValue` handoff; ordinary
Drop is not such a route. No source implementation or native terminal for the
new owner has been observed.

### Staged neutral owner contract

The unmounted `retirement/🌲️built` fixture and schema correctly state 384
pages, 385 nodes, the nine typed nonstructural fields, LIFO staging of
`rejected_children` before `children`, no global queue advancement, and no
safe partial-drop/abandon claim. Its 192 ordinary plus 192 rejected-page chain
is a valid maximum-depth construction: only page iterators, rather than full
nodes, need reside in the fixed stack. The fixture's 30 extra payload bytes
also matches its key, binding action/capability/argument, menu, and
ordinary/rejected labels.

This is deliberately source-red until the owner exists; its script requires
`BuiltTreeRetirement`, `UiTypedRetirementCursor`, and
`try_next_or_release` in the missing Rust sibling. The forthcoming native law
must additionally prove three details not represented by a bare alternating
chain: cursor reset between child nodes, a parent with both ordinary and
rejected pages (ordinary completes first), and a 383-page owner running beside
one foreign retained page so that no global consumption can be hidden. The
owner stack should assert its exact heap capacity of 384 without using a
cross-platform `size_of` byte constant.

### Current staged owner reread — source-qualified pass, deferred mount

The formerly missing owner source is now staged at
`🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/🌲️built/🦀️.rs`.
It allocates exactly `UI_BUILT_CHILD_RETIRE_SLOTS` `Option` entries with
`Vec::with_capacity`, `resize_with`, and `into_boxed_slice` (lines 22–26),
so the 384-page traversal stack is heap backed and non-growing. It holds one
boxed current node plus the page stack, rather than retaining a recursive
parent chain. `close_step` drives the nine actual `BuiltNode` payload fields
through the same `UiTypedRetirementCursor` and resets that cursor whenever a
field completes (lines 35–51); that covers `key`, `component`, `layout`,
`style`, `activity`, `disabled`, `accessibility`, `bindings`, and `menu`
without placing bindings in the structural projection.

The page order and final-release mechanics are source-sound. The owner stages
`rejected_children` first and `children` second (lines 53–61), hence the
LIFO stack visits ordinary children first. The added `try_next_or_release`
uses `try_lock` only after its final child (the only point that needs handback
release), returning `Pending` without changing iterator state under
contention; after the exact key/epoch/reservation check it releases the page
locally and clears its backing (`…/🌲️built/📋️children/🦀️.rs:12-30`). The
staged native tests cover typed component cases under all three byte grants, a
node containing both ordinary and rejected pages with the asserted
ordinary-first order, a 384-page alternating chain, lock contention, and 383
owner pages beside one queued foreign page. The owner remains strict-terminal:
`ManuallyDrop` prevents a partially closed node/cursor from being dropped
after the outer contract violation, rather than falsely advertising safe
abandonment.

This is intentionally **not compiled or accepted yet**. The required module
wiring is deliberately deferred while the active native target is occupied:
the private iterator child module must be mounted inside `🏗️builder.rs` and
`pub(crate)`-reexport its enum there, because it reads builder-private backing,
cursor, handback, and authority state. The tree owner must then be mounted
under `🎬️action.rs`'s existing retirement module and publicly reexported.
Mounting the current child file directly as an action-retirement sibling would
rightly fail Rust privacy checks; that is a deferred wiring requirement, not a
claim of a current mounted-source defect. No Cargo/Nx/native terminal was run
by this audit.

One mount-time hardening item remains: the owner manually calls
`UiTypedRetirementCursor::advance` for each of its nine field types, but does
not yet carry the corresponding compile-time depth assertions. `advance`
returns an error only at runtime when a field's `UiTypedRetire::DEPTH` exceeds
the fixed 16-byte path (`…/🌳️typed/🧱️component.rs:25-29`), while existing
`UiSnapshot`, `UiPatch`, and `UiIntent` enforce their bounds at compile time
(lines 230-232). Add assertions for the nontrivial owned field types at the
mount seam. That keeps a future component/layout/binding schema expansion from
turning into an error followed by the intended strict-owner panic. This is a
source-hardening requirement; it does not invalidate the staged traversal
algorithm or constitute runtime evidence.

There is one portable-construction RED to clear before that mount:
`Vec::with_capacity(384)` promises capacity of **at least** 384, so an
allocator is allowed to expose a larger `Vec::capacity`. The staged
`assert_eq!(pages.capacity(), UI_BUILT_CHILD_RETIRE_SLOTS)` therefore turns a
valid over-allocation into a cross-platform panic. The fixed authority is the
post-`resize_with` logical length and the resulting boxed-slice length, both
exactly 384, not allocator spare capacity. Remove the equality assertion and
retain the page-count-versus-boxed-length guard in `close_step`.

### Immediate current-byte supersession

Both source hardening observations above are now closed in the still-unmounted
owner. `new` retains the fixed resized boxed slice but no longer asserts an
allocator-specific `Vec::capacity`; `close_step` remains guarded by the boxed
slice's exact logical length. The owner also now carries compile-time
`UiTypedRetire::DEPTH <= UI_TYPED_RETIREMENT_DEPTH` assertions for all nine
owned field types, and resets its typed cursor again when a child node is
handed off from a page. Those are source-qualified corrections only: the
private builder/action module wiring and any native execution remain pending.
