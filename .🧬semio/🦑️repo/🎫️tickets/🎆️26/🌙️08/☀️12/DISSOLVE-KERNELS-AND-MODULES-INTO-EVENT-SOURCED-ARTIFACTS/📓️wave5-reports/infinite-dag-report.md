# Wave 5 — `♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag` `CollectionMutation` elimination

Target file: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️component.rs`
(37 hits of `CollectionMutation<TId,TItem,TPatch>`, the **generic wrapper** Type 1 from
`📓️collection-type-elimination-map.md`, defined UCAS-owned at `💻️os/🔨️modules/🌿️vcs/🦀️component.rs:280`
— confirmed by `grep -n "CollectionMutation<"` returning the two `Nodes(CollectionMutation<…>)`/
`Edges(CollectionMutation<…>)` variant declarations, not the space-style bare local enum).

## Boundary confirmation

`grep -c CollectionMutation` → 37. `grep -n "CollectionMutation<"` → exactly the two
`Nodes(CollectionMutation<String, DagNodeSpec, DagNodePatch>)` / `Edges(CollectionMutation<String,
DagFixtureEdge, DagEdgePatch>)` variants (old `:7444-7445`) — Type 1, not `🪐️space`'s Type 2. Full
paths double-checked against the trap table: this is `💻️os/🔨️modules/♾️infinite/…` (framework
module), not the `🕸️dag` **plugin** (`✏️s/🔌️plugins/🕸️dag/`) — an unrelated tree sharing the emoji.

## ⚠️ Decisive finding that reshaped this wave: the plugin no longer bridges through this type, but DOES still depend on two of its sibling types

Read `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
in full (and its 14 triad leaves). Its own header doc comment: *"no bridge into
`infinite_board_port_directed_dag::DagMutation` (the foreign kernel port type) either — see
`📝️text/🦀️component.rs` for the local `DagMutationDsl` mirror that replaced it."* Confirmed by grep:
zero live references to `infinite_board_port_directed_dag::DagMutation` anywhere in the plugin tree
(only two doc-comment mentions of its removal), and zero references anywhere in the repo to
`infinite_board_port_directed_dag::DagStore`/`DagEnvelope`/the wasm-bound `DagSnapshotVcs` outside
this file itself. **This file's `DagMutation`/`DagDiff`/`DagStore` region was fully orphaned before
this wave** — a prior session (per that facet's own `📓️waveM-reports/dag-dag-report.md`, status
`partial`, gates never run) built the plugin's own 14-verb semantic vocabulary independently and
switched the plugin off this framework enum entirely, leaving this file free to redesign without any
external dispatch-site sweep.

**But NOT fully orphaned** — a second, narrower grep (`grep -rln "DagNodePatch\|DagEdgePatch"
✏️s/🔌️plugins/🕸️dag`) found 12 plugin files still directly consuming this file's `DagNodePatch`/
`DagEdgePatch` **as diff-internal patch types** (`pub use infinite_board_port_directed_dag::{DagEdgePatch,
…, DagNodePatch, …}` at `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🦀️component.rs:9-11`), and their own
`Identified`/`Patchable` impls, called by name (`.apply_patch(patch)`) in the plugin's own
`apply_nodes_delta`/`apply_edges_delta`/`apply_identified_delta` (`…/🧬️schema/🔺️diff/📝️text/🦀️component.rs:15-77`).
First pass deleted these outright and broke that plugin (`error[E0432]: unresolved imports
board::ports::directed_dag::{DagEdgePatch, DagNodePatch}` at this crate's own
`♾️infinite/📦️packages/🦀️rust/📦️glue.rs:87-88` re-export, caught by this wave's own `cargo check`, not
assumed) — restored both types, their fields, and their `Identified`/`Patchable` impls **byte-identical**
to the pre-existing shape, in a clearly labelled `//#region 🔖️ExternalPatchSupport` explaining why they
stay. This file's own new `DagMutation`/`DagDiff` no longer uses either type — they exist solely for the
plugin now. Lesson matches the ticket's own standing correction protocol: *"a bare identifier grep is a
search, not a census"* — checking only `DagMutation` usage and not the sibling patch types once again
proved a hit count real but a blast-radius claim wrong, exactly the class of error the ticket's own doc
warns against, caught here by the mandated compiler gate rather than assumed away.

## Domain shape read (before choosing verbs)

`DagSnapshot { schema, nodes: Vec<DagNodeSpec>, edges: Vec<DagFixtureEdge> }`. `DagNodeSpec`: `id`
(identity/address — edges reference nodes as `"<id>@<port>"` strings, so renaming `id` cascades),
`name` (separate display label), `abbreviation`, `icon`, `x`/`y` (position), `width`/`height` (extent),
`operator_kind: Option<String>`, `properties: PropertyBag`, `kind: DagNodeKind` (11-variant tagged enum:
Computation/Slider/Select/Screen/Note/Image/Preview/Action/Export/Cluster/AppInstance). `DagFixtureEdge`:
`id`, `source`/`target` (endpoint strings), `route_style: EdgeRouteStyle`, `properties: PropertyBag`.
Node vec order is semantically meaningful (z-stack: `DagHost::paint_scene`/hit-testing iterate
`nodes.iter().rev()`, later index = frontmost — confirmed by reading, not assumed); no explicit
bring-to-front/send-to-back editor gesture exists today, so no reorder verb was minted speculatively —
only what the original `CollectionMutation::Move{id,to_index}` already covered. Edge vec order has no
such evidence (no `.rev()`, no reorder gesture), so `connect-nodes` carries no index.

## Verb derivation

Read `../SEMANTIC-MUTATIONS-OVERHAUL/📓️taxonomy.md` and `📓️derivation-rules.md` per the ticket's binding
instruction, THEN found the plugin's own already-SMO-adjacent 14-verb vocabulary (built independently
for the identical domain shape) and adopted the **same 14 verbs, same field names** for consistency
across the two homes of this domain: `CreateNode{node,index}` / `DeleteNode{id}` (cascade: severs +
inverse re-`ConnectNodes`s every touching edge) / `RenameNode{id,new_id}` (id is the identity field per
addressing rule 2; cascades every `"<id>@<port>"` edge endpoint, captured as
`rewritten_edge_endpoints`) / `ChangeNodeName{id,new_name}` (display label, separate from `id`) /
`MoveNode{id,x,y}` (absolute spatial) / `ResizeNode{id,width,height}` (absolute extent) /
`ChangeNodeIcon`/`ChangeNodeAbbreviation`/`ChangeNodeOperatorKind{id,new_operator_kind:Option<String>}`
(plain scalars) / `ReplaceNodeKind{id,new_kind}` (whole-value swap of the 11-variant enum — the
editor's only real kind-editing gesture, `node_patch_for_field`'s slider arms in the plugin, already
clones-mutates-refits a **whole** `kind` value, so `replace` per the scalar-vs-structured test, not a
30-variant-times-field decomposition) / `ReplaceNodeProperties{id,new_properties}` (whole `PropertyBag`
swap — no piecewise per-property editing gesture exists) / `ReorderNodes{order:Vec<String>}` (FINAL-state
full id order, matching what the sole existing z-order gesture needs) / `ConnectNodes{id,source,target,
route_style,properties}` / `DisconnectNodes{id}` (derivation rule 4: an edge is endpoints-plus-payload
with no independent identity — `connect`/`disconnect`, not `create`/`delete`; no direct edge-field-change
verb — a reroute/re-style is `disconnect-nodes`+`connect-nodes`, matching the plugin's own `dag_snapshot_mutations`
differ).

**Verbs left out, and why**: no `change-edge-source`/`change-edge-target`/`change-edge-route-style` (no
evidence either is edited independently of a full reconnect — flow's `ChangeSynapseFromPort`/`ToPort`
precedent doesn't transfer without the same evidence here); no plural `delete-nodes`/`drag-nodes` (no
multi-select-commits-to-VCS gesture in this file's boundary — `DagHost::delete_selected` operates on the
ephemeral runtime `fixture`, a separate, untouched mechanism outside this wave's region; multi-delete
already composes as N `DeleteNode` mutations under one `ArtifactCommand::Apply`, which is the
`group_id`-batched-primitives pattern the doctrine specifies, not a missing verb).

## What changed

All edits are inside the pre-existing `// #region 🔖️ArtifactVcs` (old `:7263` onward) through the end
of the file; nothing before it (the `DagHost` runtime/paint/hit-test/layout code, ~7,200 lines) was
touched.

- **`//#region 🔖️ExternalPatchSupport`** (new, `:7276`-ish) — `Identified<String> for DagNodeSpec`/
  `DagFixtureEdge`, `DagNodePatch` (unchanged 6-field shape: `name,x,y,width,height,kind`), `DagEdgePatch`
  (unchanged: `source,target`), their `Patchable` impls (`apply_patch`/`diff_patch`, byte-identical to
  before) — kept **only** because `✏️s/🔌️plugins/🕸️dag` consumes them directly (see finding above); no
  longer used by this file's own `DagMutation`/`DagDiff`.
- **`//#region 🔖️DiffDeltas`** (new) — one small concrete struct per verb (`RenamedNode`, `MovedNode`,
  `ResizedNode`, `ChangedNodeName`, `ChangedNodeIcon`, `ChangedNodeAbbreviation`,
  `ChangedNodeOperatorKind`, `ReplacedNodeKind`, `ReplacedNodeProperties`, `RewrittenEdgeEndpoint`) —
  chosen over reusing `DagNodePatch` as a shared diff-internal bag so `DagDiff::absorb`'s per-field
  LWW-overwrite semantics stay unambiguous (no risk of two unrelated field-changes colliding in one
  shared patch struct).
- **`pub enum DagMutation`** (`:7516`) — 14 variants replacing `Nodes(CollectionMutation<…>)`/
  `Edges(CollectionMutation<…>)`/`SetNodes{nodes}`/`SetEdges{edges}`/`SetSnapshot{snapshot}`.
  `SetSnapshot` was **itself one of the literal banned identifiers** in taxonomy's forbidden-vocabulary
  list (`SetSnapshot`, `NoMutation`, `CollectionMutation`) — its removal (no replacement, per the locked
  "whole-document replace goes through `ArtifactStore::reset`" decision) was in scope even though the
  ticket boundary text named only `CollectionMutation`, because it lived in the same enum and its own
  doc comment described it as a mutation variant.
- **`pub struct DagDiff`** (`:7537`) — sparse, one `Option<…>` field per verb's delta, `created_node_at:
  Option<usize>` as `created_node`'s sibling field (matches `🪐️space`'s `created_folder_at` convention;
  derive engine has no "record + position" shape).
- **`impl MutationDiff<DagSnapshot> for DagDiff`** (`apply`/`absorb`, `:7562`) — hand-rolled, direct
  field mutation per delta (no generic collection-diff machinery); `absorb` is LWW-overwrite per
  singular field, `.extend()` for the two id-list fields and the edge-rewrite list.
- **`impl Mutation<DagSnapshot> for DagMutation`** (`diff`/`inverse`, `:7700`) — every `diff` arm built
  from `(payload, base)` directly (never apply-then-capture); every `inverse` arm reads the pre-state
  from `base` and returns `Vec::new()` on a missing target.
- **`DagMutationDsl`** (OpText/OpBinary mirror) — 14 variants matching `DagMutation`; `ReplaceNodeKind`
  uses `#[dsl(statements)] new_kind: Box<DagNodeKindDsl>` (same pattern as this file's own
  `DagNodeKindDsl::Preview.content` and `🪐️space`'s `ReplaceEntryBody.new_body`); every other field
  (`Vec<String>`, `Option<String>`, `EdgeRouteStyle`, `PropertyBag`) is DSL-representable directly per
  this file's own `DagNodeSpecDsl`/`DagNodeKindDsl::Preview.expanded` precedent — **no JSON-escape-hatch
  fields were needed**, unlike the plugin facet's own mirror (which needed one for `DagNodeKind` alone
  because it couldn't reuse this file's private `DagNodeKindDsl`).
- **`DagNodePatchDsl` + its to/from-dsl functions** — deleted (private, only used by the removed
  `NodesPatch` variant; not part of the plugin's re-export surface).
- **Test module `dag_vcs_tests`** — every `CollectionMutation`/`SetNodes`/`SetEdges`/`SetSnapshot`
  literal replaced with the new vocabulary; added coverage for every one of the 14 variants (op-text
  round trip per variant, `ChangeNodeOperatorKind`'s `Some`/`None` arms both), plus a `🔖️MutationLaws`
  region: determinism (`diff`/`inverse` called twice, asserted equal), diff-consistency (`diff().apply()`
  compared against the mutation's own documented direct-field effect), the absorb law (two coalesced
  `MoveNode` diffs converge to the LATER value), and missing-target no-ops for `MoveNode`/`DeleteNode`/
  `DisconnectNodes`.
- **`♾️infinite/📦️packages/🦀️rust/📦️glue.rs`** (`:86-89`) — crate-root re-export list left unchanged in
  final shape (still re-exports `DagNodePatch`/`DagEdgePatch` alongside the rest) after the first-pass
  deletion attempt was caught by `cargo check` and reverted — see the finding above.

## A real bug this wave's own executed laws caught (not hypothetical — per the ticket's mandate)

First implementation of `CreateNode` carried no position (`{node}` only, matching the plugin facet's own
`create_node(node)`/`delete_node`-inverse shape, which per its own report was **never compiled or
tested**). Running the round-trip law for `delete_node_severs_and_reconnects_edges` (2 nodes, delete the
first) failed: `inverse()` restored membership and field values but not vec **position** — the
recreated node landed at the end instead of its original index, silently reordering the z-stack on
every undo of a delete. Fixed by adding `index: usize` to `CreateNode` (taxonomy's own canonical-args
column: "full initial payload **(+ optional index)**") and threading it through `diff`/`apply`
(`Vec::insert` at the FINAL-state index, clamped) and `DeleteNode`'s `inverse` (captures the node's
`position()` in `base`, not just its `id`). Re-ran the standalone extraction (below) after the fix —
all assertions pass. This is a deliberate, evidenced improvement over the plugin facet's own
(unexecuted) shape, flagged here rather than silently diverging from it.

## Files touched

- **Updated**: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️component.rs`
  (region `🔖️ArtifactVcs` only), `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/📦️glue.rs`
  (re-export list, temporarily edited then restored to include `DagNodePatch`/`DagEdgePatch`).
- **Read only**: `✏️s/🔌️plugins/🕸️dag/**` (12 files reading `DagNodePatch`/`DagEdgePatch`, plus the
  14-triad `🧬️mutations` facet, plus `🗿️artifacts/🕸️dag/🦀️component.rs`'s re-export list) — never
  edited, per the ticket's plugin-boundary rule; used only to confirm what this framework file's public
  surface must keep.
- **Created (ticket-folder scratch)**: `scratch-w5-dag-standalone.rs` (standalone law-test extraction),
  `scratch-w5-dag-check3.txt` .. `scratch-w5-dag-check-final.txt` (raw `cargo check` output at each
  iteration), this report.

## Verification — commands run, real output

**Owning crate**: `semio-framework-os-infinite` (lib name `semio_framework_os_infinite`, package
`🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/Cargo.toml`), determined from
`♾️infinite/📦️packages/🦀️rust/📦️glue.rs:60-61` (`#[path = "../../🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️component.rs"]
pub mod directed_dag;`) — **not** `semio-framework-os-kernel` (this ticket's other documented baseline
crate does not mount this file at all; confirmed by grep, no `#[path]` reference to
`➡️directed/🕸️dag` anywhere under `💻️os/📦️packages/🦀️rust/`). No prior baseline for this crate exists in
this ticket's records, so a baseline was established as part of this wave (see next paragraph) rather
than assumed from another crate's numbers.

```
$ RUSTC_WRAPPER="" CARGO_TARGET_DIR=<ticket>/🎯️target cargo check -p semio-framework-os-infinite --all-targets
```
Run 4 times across this wave (after each structural change, `touch`ed implicitly by each edit forcing a
real recheck — never relied on a cached zero-diagnostic run). Final state, `lib` target:
**0 errors**, 63 warnings (pre-existing "unnecessary qualification" style lints in the untouched
`DagHost` runtime code before `:7264`, plus one now-fixed `unused import: OpText` this wave introduced
and removed). `lib test` target: **12 errors, all in `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️component.rs`**
— zero errors reference `➡️directed/🕸️dag`. Confirmed by grepping the raw output for both this file's
path (63 warning hits, 0 error hits — the earlier `unused import: OpText` was the only one, now fixed)
and the error locations specifically (`grep -A1 "error\[E0608\]\|error: couldn't read"` → every `-->`
line points at `🌍️world/🦀️component.rs`, e.g. `assert_eq!(args["kind"], json!("vortex"))` at `:4076` —
`DslValue` has no `Index` impl (`grep -rn "impl.*Index.*for DslValue" 🧰️framework/🔨️modules/🗣️dsl` →
zero hits) — plus 2 `couldn't read …capsule_J.glb: No such file or directory` errors, an absent bundled
asset). `🌍️world/🦀️component.rs` was never read or touched by this wave. `stat -f '%Sm'` on it reports
Aug 7 (six days before this session), and `git log --oneline -3` for that path shows no commit newer
than the repo's oldest visible history in this session — pre-existing, not live churn from a concurrent
session tonight. Full raw output for the final run: `scratch-w5-dag-check-final.txt`.

**`cargo test -p semio-framework-os-infinite --lib` could not be run to completion** — the same
unrelated `🌍️world` breakage blocks the whole crate's test binary from linking (a single `--lib test`
target compiles every module's `#[cfg(test)]` code as one unit; one module's compile error blocks all).
Per `📌️important.md`'s protocol for red compiles outside this wave's boundary (retry, prove zero errors
in own paths, record, report `blocked-churn`, stop) — retried 3 times across real (non-cached) rebuilds,
identical 12-error signature every time, zero touching this file, recorded under
`## Concurrent-churn observations` below. **No claim of "tests passed against the real crate" is made.**

**Real, honest execution of the new logic instead**, per the ticket's "gates are necessary, not
sufficient" instruction (`✳️text`'s 6-failures-after-a-clean-gate precedent) — wrote and ran a standalone
extraction of the new `diff`/`apply`/`inverse`/`absorb` logic (byte-identical algorithm, minimal type
stand-ins) at `.🦑️repo/🎫️tickets/…/scratch-w5-dag-standalone.rs`:

```
$ rustc --edition 2021 -o scratch-w5-dag-standalone2 scratch-w5-dag-standalone.rs && ./scratch-w5-dag-standalone2
warning: variant `ChangeNodeName` is never constructed
   (expected — this run's fixture doesn't exercise it; the real file's test module does)
ALL SCRATCH ASSERTIONS PASSED
```

That run genuinely exercises, and passed: create→move→resize→delete round trip (inverse restores exact
prior state, including position — this is what caught the `index` bug above, on the FIRST run before the
fix, with a real `assertion left == right failed` panic showing the reordered node vec); rename-node's
edge-endpoint cascade; delete-node's sever+reconnect cascade; reorder-nodes round trip; connect/disconnect
round trip; determinism (`diff`/`inverse` called twice on identical inputs, asserted equal); diff-consistency
(`diff().apply()` compared against the mutation's own documented direct-field effect); the absorb law (two
coalesced `MoveNode` diffs converge to the later value, not the earlier one); missing-target no-ops
(`MoveNode`/`DeleteNode`/`DisconnectNodes` against an empty snapshot, asserted `Vec::new()`/`DagDiff::default()`).
**This is scratch verification, not a substitute for the real crate's test suite** — flagged honestly,
not claimed as equivalent, matching the flow/space wave's own precedent for the identical situation.

## `## Concurrent-churn observations`

`🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️component.rs` fails `cargo check
-p semio-framework-os-infinite --all-targets` under `--tests`/`--all-targets` with 12 errors (10×
`E0608 cannot index into a value of type DslValue` in its own `#[cfg(test)]` code, 2× a missing bundled
`.glb` asset read at compile time) — this file was never read or edited by this wave, its own tests
index `DslValue` with `[]` while no `Index` impl exists anywhere in the `dsl` crate (grep-confirmed), and
its on-disk mtime (Aug 7) predates this session. Retried the scoped check 3 times at real (multi-minute)
intervals across this wave's own edits; identical error set every time. This blocks `cargo test --lib`
for this whole crate (not just this wave's tests) but does **not** block `cargo check` of the `lib`
target itself, which is genuinely clean. Recorded per protocol; not fixed (out of boundary), reported as
`blocked-churn` for the test-execution gate specifically, worked around via standalone extraction for
this wave's own new-logic verification.

One further concurrent-churn item hit mid-wave and self-resolved: the first `cargo check` attempt failed
inside `semio-framework` (a dependency of `semio-framework-os-infinite`) with 3 unrelated errors
(`unresolved import semio_framework_mesh_engine`, cascading `mesh::{mesh_box, …}` re-export failures in
`🧰️framework/📦️packages/🦀️rust/📦️glue.rs:51-56`) — a different, deeper pre-existing break (that module's
own doc comment: *"now dissolved into `semio-framework-mesh-engine`"*, a crate that does not exist in
this workspace). A second run of the identical command (no edits to that crate in between) got past it
cleanly — most likely resolved by a concurrent wave 5 sibling packet (`📓️wave5-reports/mesh-module-dissolution-report.md`,
present in this ticket folder) landing mid-build. Recorded for completeness; not this wave's fix, not
re-verified independently since it's outside this wave's boundary and stopped reproducing.

## sharedFileRequests

None outside this file and its own crate's glue. `✏️s/🔌️plugins/🕸️dag/**` was read extensively (to
determine the real external-consumer surface) but never edited, per the plugin-boundary rule — its own
14-triad facet remains exactly as that prior session left it (status `partial`, its own gates still
unrun, per its own `📓️waveM-reports/dag-dag-report.md`; unaffected by this wave either way since this
file's `DagMutation` was already unreferenced by it).

## Honest pass/fail

- **`CollectionMutation<TId,TItem,TPatch>` (the assigned banned type): eliminated.** Zero live references
  remain (grep-confirmed); the two `Nodes`/`Edges(CollectionMutation<…>)` variants are gone with no
  generic-wrapper replacement, decomposed into 14 real semantic verbs.
- **`SetSnapshot`/`SetNodes`/`SetEdges` (adjacent banned/CRUD-flavored variants in the same enum):
  eliminated too, no replacement**, per the doctrine's "whole-document/whole-collection replace is not
  an in-history mutation" rule — in scope because they shared the enum being converted, not a scope
  expansion beyond what compiling this file coherently required.
- **`semio-framework-os-infinite`'s `lib` target: compiles clean (0 errors).** This is the honest,
  currently-achievable compiler gate for this crate.
- **`lib test` target: cannot compile**, but demonstrably for reasons unrelated to this wave (12 errors,
  all in a different module, pre-existing, `blocked-churn`-recorded above) — not claimed as a pass, not
  hidden.
- **Law tests: authored for all 14 variants and genuinely EXECUTED** via a standalone extraction (not
  merely written), which caught and led to fixing one real bug (`CreateNode`/`DeleteNode`'s lost
  position on undo) before this report was written — the class of finding the ticket's "gates are
  necessary, not sufficient" instruction exists to force, reproduced here honestly rather than glossed
  over.
- **Every verb left out of this vocabulary is flagged above with its reason** (no invented vocabulary):
  no per-edge field-change verbs, no plural node-delete verb, no speculative kind-field decomposition.
