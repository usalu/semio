# Wave 2 — `flow`/`flow`/`1`/`any` facet report

Facet: `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations`
Crate: `semio-s-plugin-flow`

## Vocabulary derived (9 semantic mutations, 0 generic left)

| Old (generic) | New semantic mutation | Verb | Entity | Record | Notes |
|---|---|---|---|---|---|
| `Widgets(CollectionMutation::Add{index,item})` | `CreateWidget{index,widget}` | `create` | `widget` | `CreatedWidget` | id-keyed create |
| `Widgets(CollectionMutation::Remove{id})` | `DeleteWidget{id}` | `delete` | `widget` | `DeletedWidget` | captures cascade (severed synapses + layout entry) |
| `Widgets(CollectionMutation::Move{id,to_index})` | `ReorderWidgets{id,to_index}` | `reorder` | `widget` | `ReorderedWidgets` | list position, never spatial |
| `Widgets(CollectionMutation::Patch{id,patch})` | `ReplaceWidget{id,widget}` | `replace` | `widget` | `ReplacedWidget` | widgets are a heterogeneous tagged enum — whole-value swap, matches `flow::Widget`'s own `Patchable` impl |
| `Synapses(CollectionMutation::Add{index,item})` | `ConnectWidgets{index,id,from,from_port,to,to_port}` | `connect` | `synapse` | `ConnectedWidgets` | edge-collection verb (derivation-rules §4), not generic create |
| `Synapses(CollectionMutation::Remove{id})` | `DisconnectWidgets{id}` | `disconnect` | `synapse` | `DisconnectedWidgets` | edge-collection verb |
| `Synapses(CollectionMutation::Move{id,to_index})` | `ReorderSynapses{id,to_index}` | `reorder` | `synapse` | `ReorderedSynapses` | list position |
| `Synapses(CollectionMutation::Patch{id,patch})` | `UpdateSynapseEndpoints{id,from,from_port,to,to_port}` | `update` | `synapse` | `UpdatedSynapseEndpoints` | inseparable 4-field facet, never set one endpoint at a time |
| `SetLayout{entries}` | `MoveWidgets{entries}` | `move` | `widgets` | `MovedWidgets` | plural — mirrors `flow::flow_fixture_operations`'s own batched-per-gesture `SetLayout` output 1:1 (real multi-widget drag), `entries: Vec<flow::FlowLayoutEntry>` (addr + optional position, not a generic option-bag) |
| `SetSnapshot{snapshot}` | **deleted, no replacement mutation** | — | — | — | banned vocabulary; whole-document replace has no in-history mutation per taxonomy |

`schema: String` and `camera: CameraJson` (document-root scalars) were left with no mutation:
`schema` is a static envelope/version marker, never diffed by the framework's own
`flow_fixture_operations`; `camera` is view/UI state the framework never diffs either (confirmed by
reading `flow_fixture_operations`'s source — it only ever emits `Widgets`/`Synapses`/`SetLayout`) and
is managed separately by the app layer (`engine::focus_selection_camera`, `FlowHost::dag.set_viewport`).
Inventing a `move-camera` mutation was considered and rejected: `to_framework_mutation` (needed for
the `OpBinary`/`OpText` wire codecs, which still delegate to the framework's generic
`flow::FlowMutation` for actual encoding) has no way to construct a full `flow::FlowFixture` from a
bare camera-only payload without a base snapshot it doesn't receive — so it would be unrepresentable
on the wire, not just cosmetically unmodeled.

Every `SEMANTICS.kind` matches its variant's own kebab form and its triad-dir stem exactly, and every
`verb` (`create`/`delete`/`reorder`/`replace`/`connect`/`disconnect`/`update`/`move`) is in
`APPROVED_VERBS` (derive-enforced compile-time asserts on each variant — confirmed by manual review
per-mutation; couldn't run the actual assertion because of the `glue.rs` blocker below, see Verify).

## Real handcrafted diffs (no apply-then-capture)

Every `🔺️diff` leaf builds the artifact's sparse `FlowDiff` directly from the payload — never
apply-then-capture:

- `create-widget`/`delete-widget`/`reorder-widgets`/`replace-widget` and the four synapse
  equivalents construct the appropriate `protocol::CollectionMutation` (`Add`/`Remove`/`Move`/`Patch`)
  from the payload's own fields and delegate to the artifact's existing pure sparse-diff builders
  (`widgets_delta_from_collection_mutation`/`synapses_delta_from_collection_mutation` in
  `🔺️diff/📝️text/🦀️component.rs`) — `CollectionMutation` stays purely internal to the diff/inverse
  leaves, never appears in the public `FlowMutation` enum (taxonomy's `## Forbidden vocabulary`).
- `delete-widget`'s diff is the one genuinely new piece of logic: it cascades into every synapse
  whose `from`/`to` references the deleted widget id (`FlowSynapsesDelta::removed`) and clears the
  widget's `layout` entry if present — a real captured-cascade delete, not the old code's bare
  `CollectionMutation::Remove` passthrough.
- `move-widgets`' diff builds a `FlowLayoutMapDelta` directly from the payload's entries (id →
  `Option<WidgetLayout>`, `None` = clear).

## Real handcrafted inverses (computed from `base`, never by inverting the diff structurally)

- `create-widget` ↔ `delete-widget` are exact partners (taxonomy pair).
- `delete-widget`'s inverse is the interesting one: re-`create`s the widget at its base-state index,
  restores its `move-widgets` layout entry if it had one, then re-`connect`s every severed synapse
  (found in `base.synapses`) in reverse order — matching the taxonomy's "delete captures cascade...
  re-`connect`ed after `create` in reverse dependency order" rule.
- `connect-widgets` ↔ `disconnect-widgets` are exact partners; `disconnect-widgets`'s inverse
  reconstructs the full synapse (endpoints + base-state index) from `base`.
- `reorder-widgets`/`reorder-synapses` inverses look up the item's CURRENT index in `base` and
  reorder back to it (guarded: returns `Vec::new()` if the id is absent from `base`, never panics —
  I deliberately avoided `protocol::inverse_collection_mutation`'s panic-on-missing-target contract
  here in favor of the taxonomy's "missing target ⇒ `Vec::new()`" rule).
- `replace-widget`/`update-synapse-endpoints` look up the OLD value from `base` and re-emit the same
  verb with the old value; `Vec::new()` if the target is gone.
- `move-widgets`' inverse rebuilds each entry from `base.layout.get(id)` (restoring absence as
  `None`); returns `Vec::new()` for an empty batch.

## Files touched (all inside `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow`)

- `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs` — rewritten: tuple-variant
  `FlowMutation` enum + `#[derive(..., protocol::Mutations)]` +
  `#[mutations(snapshot = FlowSnapshot, diff = FlowDiff, schema = "flow.flow")]`; old hand-written
  `impl Mutation<FlowSnapshot> for FlowMutation` deleted (derive generates it now).
  `from_framework_mutation` now returns `Option<FlowMutation>` (was infallible `FlowMutation`):
  `flow::FlowMutation::SetFixture` (whole-fixture replace) has no semantic-mutation representation
  per the taxonomy's `set-snapshot` ban and returns `None` — this is unreachable on the live
  host-bridge path since the framework's own `flow::flow_fixture_operations` never emits `SetFixture`
  (verified by reading its source: only `Widgets`/`Synapses`/`SetLayout`), and only matters for a
  hand-authored/decoded `flow.op` line, which now correctly *fails* to decode instead of silently
  losing data. `to_framework_mutation` stays infallible/total (every semantic variant has an exact
  framework-generic counterpart). `OpBinary::decode_op`/`OpText::parse_op` updated to surface a real
  error (`protocol::ProtocolError::Malformed` / `store::TextError`) when `from_framework_mutation`
  returns `None`, instead of the old unconditional `.map(...)`.
- New triad leaf dirs (`.rs` only, one per mutation, matching kind exactly):
  `➕️create-widget`, `🗑️delete-widget`, `🔀️reorder-widgets`, `🔁️replace-widget`,
  `🔗️connect-widgets`, `✂️disconnect-widgets`, `🔀️reorder-synapses`,
  `🔄️update-synapse-endpoints`, `📍️move-widgets` — each `{🦠️mutation,🔺️diff,↩️inverse}/🦀️component.rs`.
- Deleted dirs (fully superseded, not merely emptied): `📄set-snapshot`, `📐set-layout`, `🔗synapses`,
  `🧩widgets` — these were pre-existing scaffolds 1:1 mirroring the four OLD generic variant names,
  not real semantic content.
- `🧬️mutations/📝️text/🦀️component.rs` — test module extended: `move_widgets_inverse_restores_base`
  (rewritten from the old `set_layout_inverse_restores_base`, now with real non-empty entries and an
  assertion on the applied value, not a trivial empty-batch no-op), plus two new round-trip tests,
  `create_widget_then_delete_widget_round_trips_to_base` (exercises the cascade-delete/re-create
  pair) and `connect_widgets_then_disconnect_widgets_round_trips_to_base`.
- `🧬️mutations/💾️binary/🦀️component.rs` — both existing tests rewritten to use a real
  `MoveWidgets` operation instead of an empty `SetLayout`, so the binary/text round trip actually
  exercises non-trivial payload bytes.
- `🧬️schema/🔺️diff/📝️text/🦀️component.rs` (outside my facet but inside my artifact dir; its one test
  directly constructed `FlowMutation::SetLayout` and broke) — test renamed/rewritten to
  `move_widgets_diff_touches_only_the_layout_slot` using the new `MoveWidgets` mutation; the
  `widgets_delta_from_collection_mutation`/`synapses_delta_from_collection_mutation`/
  `diff_set_snapshot` helper functions in this file are unchanged (still real, still used — the first
  two by every triad leaf's diff logic above; `diff_set_snapshot` is still exercised by this file's
  other, untouched test and is a generically useful whole-artifact-swap diff builder for the
  artifact's non-mutation reset path, not something owned by the deleted `SetSnapshot` mutation).
- `🏅️standards/🔖️1/⚙️engine/🦀️component.rs` (outside my facet but inside my artifact dir) —
  `snapshot_operations`/`host_operations` changed `.map(from_framework_mutation)` →
  `.filter_map(from_framework_mutation)` to match the new `Option`-returning signature (mechanical,
  2 call sites; both are dead-code-safe since `SetFixture` is never actually emitted here).

## Blocked-mechanism: `📦️glue.rs`'s hand-maintained `#[path]` module tree is off-limits and stale

`semio-s-plugin-flow` is `[lib] path = "📦️glue.rs"`; that file (explicitly off-limits per the task's
hard boundary) hand-lists every triad leaf file with its own `#[path = "..."]` attribute — no
glob/auto-discovery. Deleting the four old scaffold dirs and creating nine real per-mutation dirs
(the correct shape per `derivation-rules.md`'s triad layout, and consistent with this same wave's
`vcs`/`writer` facets, which hit and reported the identical blocker) leaves `glue.rs`'s
`pub mod mutations { ... }` block pointing at nonexistent paths. Confirmed by running
`cargo check -p semio-s-plugin-flow`, which fails immediately with:
`error: couldn't read '.../🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs': No such file or
directory` at `📦️glue.rs:89`.

I did not touch `📦️glue.rs`, per the hard boundary constraint. All in-boundary code is internally
consistent — every cross-file module path (`super::diff`/`super::inverse` within a leaf,
`crate::artifacts::flow::schema::mutations::<slug>::mutation::X` across leaves for inverse
cross-references like `delete-widget` → `create-widget`/`connect-widgets`/`move-widgets`) was written
against the exact same nesting pattern `glue.rs` already uses, and re-checked by hand since
`cargo check` cannot see past the `glue.rs` read error.

### Exact `📦️glue.rs` patch

Full ready-to-paste replacement for the `pub mod mutations { ... }` block
(`📦️packages/🦀️rust/📦️glue.rs`, lines 77–122) is saved at:
`/private/tmp/claude-501/-Users-ueli-Documents-semio/5170febb-8580-4df7-9a13-8950b45be8bd/scratchpad/new-mutations-block.txt`
(generated programmatically from the dir list — no hand-typed emoji paths — one `pub mod <slug> {
mutation; diff; inverse; }` sub-block per new mutation, replacing the four `set_snapshot`/
`set_layout`/`synapses`/`widgets` sub-blocks). The original block is saved alongside it as
`old-mutations-block.txt` for diffing.

### sharedFileRequests

None for `🎛️apps/🌊️flow/**` — grepped exhaustively for `FlowMutation::Widgets`/`Synapses`/`SetLayout`/
`SetSnapshot` and for any import of `artifacts::flow::{schema::mutations::FlowMutation, op::FlowMutation}`
outside my artifact directory: only `🏅️standards/🔖️1/⚙️engine/🦀️component.rs` (inside my artifact
dir, already fixed above) touches this type at all. No app command anywhere constructs a `FlowMutation`
variant directly — they all go through `snapshot_operations`/`host_operations`
(`from_framework_mutation`) or the `OpText`/`OpBinary` wire codecs, both already updated. Only
outstanding shared-file need is the `📦️glue.rs` patch above.

## Testkit law coverage (recipe step e)

Skipped: grepped `semio-s-plugin-flow`'s `Cargo.toml` and every file under
`✏️s/🔌️plugins/🌊️flow` for an existing `testkit`/`protocol::testkit` import — none found. Per the
recipe, no new Cargo dependency was added; extended the facet's own existing tests instead (see
Files touched above: `move_widgets_inverse_restores_base`, two new round-trip tests) which exercise
the same inverse/round-trip properties `assert_mutation_inverse_law` would, by hand.

## Deferred (not blocking, per the ticket's step f)

Grammar (`📝️text/📖️component.grammar.semio`) and binary protocol
(`💾️binary/📡️component.protocol.semio`) under `🧬️mutations/` still describe an unrelated placeholder
vocabulary (`add-node`/`set-port`/`wire-edge`/`patch-layout`, never matched `FlowMutation` even
before this migration); left untouched. No per-triad `.ts` mirror files were written for the nine new
leaves (only `.rs`) — the old leaves' `.ts` stub files (each just `export {};`) were deleted along
with their dirs.

## Verify

- `cargo check -p semio-s-plugin-flow` — **red**, but only for the `📦️glue.rs` structural reason
  documented above. No error originates from any file inside `🗿️artifacts/🌊️flow` itself; the one
  reported error is `glue.rs:89`'s stale `#[path]` reference, deterministic (re-derivable from the
  directory diff, not transient).
- Before restructuring into real per-mutation dirs, I also ran `cargo check` against an earlier,
  fully-compiling version of this same vocabulary (9 mutations grouped into the four pre-existing
  `widgets`/`synapses`/`set_layout`/`set_snapshot` scaffold module names, reusing files `glue.rs`
  already wired so no `glue.rs` edit was needed) — that version was **green** with zero warnings from
  my artifact directory (full log checked: zero `🗿️artifacts/🌊️flow` hits). I deliberately abandoned
  that shape in favor of the proper one-dir-per-mutation layout once I found this same wave's `vcs`/
  `writer` reports establishing `blocked-mechanism` + an exact `glue.rs` patch as this ticket's actual
  convention — the grouped shape would have also failed `policyMutationImplPresenceBreaches`'s
  triad-dir-stem-equals-kind check for 8 of 9 mutations, which the correct shape satisfies for all 9.
- Also observed, both before and independent of my restructuring: `cargo check -p semio-s-plugin-flow`
  intermittently fails at `📦️glue.rs:418` with `couldn't read
  '.../🎛️apps/🌊️flow/📌️panels/📄️document/🦀️component.rs'` — confirmed this directory has been
  renamed to `📌️panels/📄️artifact` (present on disk, modified today) by a different concurrent
  session; retried 3× at 60s intervals per the workspace-churn policy, error persisted identically
  each time (not obviously transient, but clearly unrelated to this ticket — `🎛️apps/**` is
  explicitly out of my boundary). This is a second, independent blocker in the same crate that the
  reconciliation pass (or the owning session) will need to resolve alongside the `glue.rs` patch
  above; not something I can or should fix.
- `cargo test` not run (blocked by the same compile errors).
