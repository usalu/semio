# W2-A Report — Rust Wasmtime Host

Lane: **2-A Rust host** (Sonnet 5). Contract: `📋️contract-freeze.md` §3/§4/§5/§6; scout findings
`📓️scout-2-group-undo-and-hosts.md` §3/§5. Built on W0-C's pure dependency-graph functions, W0-D's
WIT/bindgen wiring, W1-A's `contributor.list-artifact-mutations`/`artifact-mutation-plan` wire
shapes, W1-B's guest transaction state machine, W1-C's `TransactionCoordinator`/`MemberRelation::Peer`.
Start commit `7ad8955884`.

## Files touched (exclusive lease)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs` — five new regions
  (`🔖️PluginGraph`, `🎯️MutationRouter`, `🔖️InstanceDirectory`, `🎯️TransactionCoordinator`), an
  upgrade of the existing `💡️InferenceRouter` region, one new method on `🔖️IoRouter`
  (`unregister_plugin`), and one new `#[cfg(test)]` e2e test appended inside the pre-existing
  bottom-of-file `mod tests`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️component.rs` — `🔖️AppChannelHost` (widened
  `open` signature), `🔖️SpaceRunner::open_node` (threads the new param), `🔖️WasmtimeNodeHost` (five
  new fields, recursive `load_runtime_recursive`, `unload_plugin`, `hot_reload_plugin`,
  `run_transaction`, `undo_transaction_group`), `🔖️Tests::FakeHost::open` (signature match only).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/📦️bin.rs` — **not touched**; nothing in the CLI flow
  needed the five new components directly, and the lease's other two files already expose everything
  a future caller needs.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📦️packages/🦀️rust/Cargo.toml` — added
  `blake3 = "1"` (already an indirect dependency via `semio-framework-os-kernel`; needed directly here
  for `HostTransactionCoordinator`'s `MutationOrigin::Contributed.payload_hash`, the exact
  `blake3::hash(...).as_bytes()` idiom `📡️spr/🎮️command/🦀️component.rs`'s own `Planner` cycle guard
  already uses for the identical `(mutation_id, payload_hash)` shape).

## Components as landed

### 1. `PluginGraph` (`host/🦀️component.rs`, region `🔖️PluginGraph`)

```rust
pub struct PluginGraph { /* Mutex<BTreeMap<String, PluginManifest>> */ }
pub enum PluginGraphError { Graph(semio_framework::DependencyGraphError), UnloadBlocked{plugin_id, dependents}, Unknown{plugin_id}, LockPoisoned }
impl PluginGraph {
    pub fn new() -> Self;
    pub fn register(&self, manifest: PluginManifest) -> Result<(), PluginGraphError>;
    pub fn prepare_hot_reload(&self, new_manifest: &PluginManifest) -> Result<(), PluginGraphError>;
    pub fn commit_hot_reload(&self, new_manifest: PluginManifest) -> Result<(), PluginGraphError>;
    pub fn guard_unload(&self, plugin_id: &str) -> Result<(), PluginGraphError>;
    pub fn unregister(&self, plugin_id: &str) -> Result<(), PluginGraphError>;
    pub fn load_order(&self) -> Result<Vec<String>, PluginGraphError>;
    pub fn dependents(&self, plugin_id: &str) -> Result<Vec<String>, PluginGraphError>;
    pub fn manifest(&self, plugin_id: &str) -> Result<Option<PluginManifest>, PluginGraphError>;
    pub fn is_registered(&self, plugin_id: &str) -> Result<bool, PluginGraphError>;
}
```

`register`/`prepare_hot_reload` call `semio_framework::resolve_load_order` (not
`validate_dependency_graph` alone) — **this was a real bug I caught with my own tests**:
`validate_dependency_graph` only catches missing-dependency/version-mismatch; per W0-C's own report
a true cycle is only caught by `resolve_load_order`'s toposort leftover-set walk. My first draft
called `validate_dependency_graph` and a cycle test (`a_later_registration_that_would_close_a_cycle_is_rejected`)
failed with `Ok(())` instead of `Err(Cycle)` until fixed. No graph logic is reimplemented — every
rejection is W0-C's own `DependencyGraphError` verbatim.

### 2. `ArtifactMutationRouter` (`host/🦀️component.rs`, region `🎯️MutationRouter`)

```rust
pub struct HostMutationRosterEntry { mutation_id, verb, entity, kind, record, contributor: Option<String>, artifact_kind: Option<String> } // mirrors guest WireMutationRosterEntry
pub struct HostArtifactMutationPlanRequest { artifact_kind, mutation_id, revision, generation, snapshot_pack, payload } // mirrors guest WireArtifactMutationPlanRequest
pub struct HostArtifactMutationPlanResult { artifact_kind, mutation_id, revision, generation, owner_ops, label, foreign: Vec<protocol::ForeignStep> } // mirrors guest WireArtifactMutationPlanResult
pub enum MutationOwnership { Owner{plugin_id}, Contributed{plugin_id} }
pub struct ArtifactMutationRouter { /* Mutex<BTreeMap<(String,String),(String,HostMutationRosterEntry)>> */ }
impl ArtifactMutationRouter {
    pub fn register_plugin(&self, plugin_id: &str, dependencies: &[semio_framework::PluginDependency], roster_wire_bytes: &[u8]) -> Result<(), PluginHostError>;
    pub fn register_roster(&self, plugin_id: &str, dependencies: &[semio_framework::PluginDependency], roster: Vec<HostMutationRosterEntry>) -> Result<(), PluginHostError>; // pure half, unit-testable without wasm
    pub fn resolve(&self, artifact_kind: &str, mutation_id: &str) -> Result<MutationOwnership, PluginHostError>;
    pub fn roster(&self) -> Result<Vec<HostMutationRosterEntry>, PluginHostError>;
    pub fn unregister_plugin(&self, plugin_id: &str) -> Result<(), PluginHostError>;
}
```

Decode path uses `store::pack_rt::decode_wire_value` + `dsl::from_dsl_value` — **not** `serde_json`
— matching the guest's real `encode_wire_serialized` (`store::pack_rt::encode_wire_value(&to_dsl_value(value))`,
confirmed by reading `🔌️plugin/🦀️component.rs:12808` directly). Gate (contract §4 rule 1): a
CONTRIBUTED row's `contributor` must equal the reporting `plugin_id`, and
`io::ArtifactKindId::parse(artifact_kind).plugin()`'s owning plugin must be a direct entry of
`dependencies`. Conflict rule is byte-identical-or-reject, exactly `ArtifactInferenceRouter::
register_plugin`'s existing idiom.

**Two-tier `resolve` lookup, a second real bug my tests caught**: an OWNER roster row's wire shape
carries no `artifact_kind` at all (only a contributed row does), so it can only be keyed by
`(reporting_plugin_id, mutation_id)` at registration — but callers always call `resolve(artifact_kind,
mutation_id)` with a real 3-segment kind. First draft's `resolve` only tried the exact
`(artifact_kind, mutation_id)` key and always missed every owner row. Fixed: `resolve` first tries the
exact key (catches contributed rows), then falls back to `(ArtifactKindId::parse(artifact_kind).plugin(),
mutation_id)` (catches owner rows). Both bugs were caught by my own unit tests failing honestly, not
inferred from reading the code.

### 3. `ArtifactInferenceRouter` upgrade (existing region `💡️InferenceRouter`)

`GuestArtifactInferenceMetadata` gains `#[serde(default)] contributor: Option<String>` and
`#[serde(default)] depends_on: Vec<String>` (additive+defaulted — today's guest wire, which sends
neither, still decodes unchanged). `register_plugin` gains a `dependencies: &[PluginDependency]`
parameter and the same contract §4 rule 1 gate the mutation router applies (contributor-aware).
`infer` is now `infer_with_visited`, recursing over `depends_on` (same `artifact_kind`, per the
field's own doc): for each dependency it recursively resolves the dependency's OWN
`InferenceRouteResult` first (`build_dependency_inference_request` — identity fields from the
dependency's own registered metadata, caller-context fields inherited from the parent request,
`previous_state` always `None`), then injects `(dependency_schema, result_bytes)` pairs into the
outgoing request's `dependencies` field before calling the owner/contributor's real `artifact-infer`
— exactly the WIT `dependencies: list<tuple<string, list<u8>>>` shape W0-D added. A new
`validate_inference_dependency_graph` DFS cycle-detector (registration-time) plus a runtime `visited`
guard (defense in depth) reject a `depends_on` cycle with a typed `PluginHostError::Plugin`. This is
genuinely new logic, not a reuse of W0-C's plugin-manifest toposort (different domain: inference
schemas within one artifact kind, not plugins).

### 4. `InstanceDirectory` (new region `🔖️InstanceDirectory`)

```rust
pub struct InstanceLocation { plugin_id: String, instance_id: u32, artifact_kind: String }
pub struct InstanceDirectory { /* Mutex<{by_artifact_id: HashMap<String, InstanceLocation>, by_instance: HashMap<(String,u32), Vec<String>>}> */ }
impl InstanceDirectory {
    pub fn bind(&self, artifact_id: &str, plugin_id: &str, instance_id: u32, artifact_kind: &str) -> Result<(), PluginHostError>;
    pub fn resolve(&self, artifact_id: &str) -> Option<InstanceLocation>;
    pub fn unbind_instance(&self, plugin_id: &str, instance_id: u32);
    pub fn artifact_ids_for_instance(&self, plugin_id: &str, instance_id: u32) -> Vec<String>;
}
```

**Third real bug caught by my own tests**: `bind`'s first draft only ever ADDED to the new
`(plugin_id, instance_id)`'s `by_instance` list, never removing the artifact id from its PREVIOUS
location on a rebind — so re-binding the same `artifact_id` to a new instance left the stale
`by_instance` entry pointing at the old, now-wrong instance. Fixed: `bind` now looks up the prior
`InstanceLocation` (if any) and removes the artifact id from that instance's list before inserting
the new one. `rebinding_the_same_artifact_id_replaces_the_prior_location` exercises this.

**Populated at**: `WasmtimeNodeHost::open` (`🏃️run/🦀️component.rs`), right after
`WasmPluginRuntime::create_app` — the real "instantiate-app" moment. `AppChannelHost::open`'s
signature grew a third `artifact_ref: &str` parameter (was `(plugin_id, app_id)`) so `SpaceRunner::
open_node` can pass `node.artifact_ref` through; `artifact_kind` is derived from the loaded runtime's
`manifest.apps[app_id].io.document_schema` (falling back to `app_id` if empty). I deliberately did
**not** add a second hook inside `HostState::pre_adopt_command_packs` for `Hello`/`LoadDocument`
specifically: by the time those commands are sent (`SpaceRunner::compute_node`'s frame script),
`open()` has already run and bound the instance, so a second hook would be redundant for this
runner's own call ordering. Flagged for W2-B/whoever drives a host that opens instances via a
different path: the seam is `pre_adopt_command_packs`'s `Hello`/`LoadDocument` match arms if a future
caller needs it there instead.

### 5. `HostTransactionCoordinator` (new region `🎯️TransactionCoordinator`)

```rust
pub struct TransactionMember { plugin_id: String, instance_id: u32 }
pub struct TransactionOutcome { txn_id: String, members: Vec<TransactionMember>, edit_ids: Vec<String> }
pub enum TransactionError { Rejected{code: String, message: String}, Host(PluginHostError) }
pub struct HostTransactionCoordinator;
impl HostTransactionCoordinator {
    pub fn run_transaction(
        &self, instances: &InstanceDirectory, mutation_router: &ArtifactMutationRouter,
        exchange: impl FnMut(&str, u32, protocol::AppCommand) -> Result<Vec<protocol::AppFrame>, TransactionError>,
        plan_contributed: impl FnMut(&str, &str, &str, &TransactionMember, &[u8]) -> Result<HostArtifactMutationPlanResult, TransactionError>,
        initiator: TransactionMember, local_ops: Vec<Vec<u8>>, description: String, foreign: Vec<protocol::ForeignStep>,
    ) -> Result<TransactionOutcome, TransactionError>;
    pub fn undo_group(&self, exchange: impl FnMut(...), members: &[TransactionMember], group_id: &str);
    pub fn redo_group(&self, exchange: impl FnMut(...), members: &[TransactionMember], group_id: &str);
}
```

Drives contract §5 steps 1-7: mints `txn_id`; resolves every `ForeignStep.target` via
`InstanceDirectory` then `ArtifactMutationRouter.resolve`; **Owner** → accumulates `step.payload`
into that member's `prepared_ops`; **Contributed** → calls `plan_contributed` (the real
`contributor.artifact-mutation-plan`, wired in `🏃️run/🦀️component.rs`'s `run_transaction` with the
target's live `SessionLanePack` snapshot via `WasmPluginRuntime::document_session`), extends
`prepared_ops` with the plan's `owner_ops`, and recurses over the plan's own `foreign`. Depth
(`protocol::MAX_PLAN_DEPTH` = 8) and cycle (`(artifact_id, mutation_id, blake3(payload))`, matching
`📡️spr/🎮️command`'s own `Planner` cycle key exactly) are guarded per contract §5.4. Every member is
sent **exactly one** `TransactionPrepare`, always in the **pre-planned wire form**
(`prepared_ops`/`label`/`origin`, never `mutation_id`+`payload`) — a deliberate choice following
W1-B's own report §5 recommendation ("the owner-mutation form carries no `origin` on the wire...
recommend the host always prefers the pre-planned form for foreign targets"), which also lets N
`ForeignStep`s accumulated against the same member ride in one `prepared_ops` list rather than
requiring N separate prepares (which the guest's one-pending-transaction rule would reject on the
second). Phase 1 (§5.5) requires every member's `TransactionPrepared.rejection` empty or rolls back
every already-prepared member. Phase 2 (§5.6) commits in **reverse discovery order**; a commit
failure sends `TransactionUndo{group_id: txn_id}` to already-committed members and
`TransactionRollback` to the rest. `undo_group`/`redo_group` fan out `TransactionUndo`/`Redo{group_id}`
to every member, best-effort.

Frozen rejection codes produced: `transaction.unknown-target`, `transaction.unknown-mutation`,
`transaction.depth-exceeded`, `transaction.cycle`, `transaction.member-rejected` (wraps a guest's own
`transaction.instance-busy`/`generation-mismatch`/etc. rejection message), `transaction.commit-failed`.
`transaction.dependency-missing`/`version-mismatch` are `PluginGraph`'s (load-time, not
transaction-time); `transaction.contribution-not-permitted` is used by `🏃️run`'s `run_transaction`
wrapper when a resolved contributor plugin isn't currently loaded.

## `🏃️run/🦀️component.rs` wiring

`WasmtimeNodeHost` gained five fields: `plugin_graph: Arc<PluginGraph>`,
`mutation_router: Arc<ArtifactMutationRouter>`, `inference_router: Arc<ArtifactInferenceRouter>`,
`instance_directory: Arc<InstanceDirectory>`, `transaction_coordinator: Arc<HostTransactionCoordinator>`.

`runtime_for` → `load_runtime_recursive(plugin_id, &mut loading_stack)`: loads `plugin_id`'s wasm,
reads its real manifest, then **recursively loads every declared dependency FIRST** (before
registering `plugin_id` itself into any router/graph) — satisfying scout-2 §3's "a dependency must be
loaded before its dependent." A `loading` stack catches a cycle that only becomes visible once a
manifest is actually read (distinct from `PluginGraph`'s own registration-time cycle check, which
only fires once every member is registered). After recursion: registers into `io_router` (unchanged
behavior), `plugin_graph.register` (contract §4 rule 5 gate), `mutation_router.register_plugin`
(decodes the real `list_artifact_mutations()` wire bytes), `inference_router.register_plugin`.

`unload_plugin(plugin_id)`: `plugin_graph.guard_unload` first (refuses while a dependent is loaded,
contract §4.5), then unregisters from every router and drops the runtime.

`hot_reload_plugin(plugin_id)`: builds a **fresh** `WasmPluginRuntime` from the same compiled path,
`plugin_graph.prepare_hot_reload` validates the WHOLE graph with the new manifest substituted
(catches a version bump that would break a live dependent — scout-2 §5's "nothing considers
dependents on reload") **before** anything swaps, then atomically replaces the runtime map entry and
every router registration. Added `IoRouter::unregister_plugin` (new method, none existed) since
`IoRouter::register_plugin`'s existing `PluginRuntimeConflict` check would otherwise reject a fresh
`Arc` for an already-registered plugin id. A fresh top-level `Arc` (not in-place mutation of the old
one) was a deliberate choice: nothing in this host caches a `WasmPluginRuntime` handle across calls
(`exchange`/`open` always re-look-up via `self.runtimes.get(plugin_id)`), so replacing the map entry
is observationally identical to, and much simpler than, requiring exclusive ownership of the old
`Arc` (which every router's own registration also holds a clone of — `Arc::get_mut` would fail in
practice the moment `runtime_for` wires a plugin into three routers).

`run_transaction(initiator_handle, local_ops, description, foreign)`: resolves the initiator's
`(plugin_id, instance_id)` from `self.instances`, then drives `HostTransactionCoordinator::
run_transaction` with closures wrapping this host's own `runtimes` map for `exchange` (encode →
`WasmPluginRuntime::exchange` → decode) and `plan_contributed` (looks up the target's live
`SessionLanePack.pack` via `document_session`, builds a `HostArtifactMutationPlanRequest`, calls the
contributor's real `artifact_mutation_plan`, decodes the result) — the exact DSL wire encoding
(`dsl::to_dsl_value`/`store::pack_rt::encode_wire_value`) the guest's `encode_wire_serialized` uses.
`undo_transaction_group` mirrors this for group undo.

`AppChannelHost::open` gained a third `artifact_ref: &str` parameter (was `(plugin_id, app_id)`);
`SpaceRunner::open_node` passes `&node.artifact_ref`; `WasmtimeNodeHost::open` binds it into
`instance_directory` after `create_app`; `FakeHost::open` (test fixture) updated to match and ignores it.

## Tests written and run

**`semio-framework-plugin-host --lib`: 38 passed, 0 failed** (real output below). Breakdown of the
new/changed tests (18 total, all in `host/🦀️component.rs`):

- `plugin_graph_tests` (6): real-edge load order + dependents, missing-dependency rejection,
  version-mismatch rejection, a genuine cycle closed by a later registration, unload refused while a
  dependent is registered then permitted once it's gone, hot-reload rejected when it would break a
  live dependent's version requirement.
- `artifact_mutation_router_tests` (4): owner+contributed rows both resolve and are both visible in
  the merged roster, a contribution onto a non-dependency is rejected, conflicting owner rows reject
  unless byte-identical, unregister drops only that plugin's rows.
- `instance_directory_tests` (2): bind/resolve/unbind round trip, rebind replaces the prior location.
- `host_transaction_coordinator_tests` (5, pure Rust — no wasm; see "What the wasm e2e does and does
  not prove" below): a real two-member transaction commits and **group undo restores both members**,
  unknown-target rejected before any prepare is sent, unknown-mutation rejected, a genuine cycle
  rejected, a 10-hop chain rejected as depth-exceeded, a member rejection (real
  `transaction.instance-busy` from the fake's own busy-check) rolls back every already-prepared member.
- **1 new real wasmtime e2e test**: `plugin_dependency_infrastructure_wires_real_loaded_plugins_and_one_real_extension`
  (see below).

Three real bugs were caught and fixed by these tests failing honestly on first run (not inferred by
inspection): `PluginGraph` cycle detection (`validate_dependency_graph` alone never catches a cycle —
needed `resolve_load_order`), `ArtifactMutationRouter::resolve`'s two-tier lookup (owner rows keyed by
plugin id, not artifact kind), and `InstanceDirectory::bind`'s stale-reverse-index-on-rebind bug. All
three are documented inline in the component sections above.

### The wasmtime e2e (contract-required proof)

`plugin_dependency_infrastructure_wires_real_loaded_plugins_and_one_real_extension`
(`host/🦀️component.rs`, region `🔖️W2aPluginDependencyE2e`, right after the pre-existing
`IoRouterE2e` test) loads:
- **2 real plugin components**: `cad`, `stdio` (`WasmPluginRuntime::load`, the same pair the
  pre-existing `IoRouterE2e` test already proved cross-instance IO with),
- **1 real extension component**: flow's `math` extension (`ExtensionRuntime::load`, `extension-world`).

It proves, against these REAL loaded components:
- `PluginGraph.register` accepts both real manifests, `load_order`/`dependents` are correct over
  them, and — layered as synthetic manifests on top of the real ones (see "known limitation" below) —
  the three typed rejections (`MissingDependency`, `VersionMismatch`, `Cycle`) and the unload guard
  all fire correctly against a graph that also contains real plugin data.
- `ArtifactMutationRouter.register_plugin` decodes the REAL `list_artifact_mutations()` wire bytes
  from both real plugins and merges them without conflict; `.roster()` is readable.
- `ArtifactInferenceRouter.register_plugin` decodes the REAL `list_artifact_inferences()` metadata
  from both and merges it.
- `InstanceDirectory.bind`/`resolve`/`unbind_instance` round-trip against real plugin identities.
- `ExtensionRuntime.load` on a real `extension-world` component succeeds and its real manifest
  (`extends`, `extension_id`) is readable.

### What the wasmtime e2e does and does not prove (read before reusing this pattern)

No shipped plugin in this repo declares a real `.depends_on()`/`.contributes()` relationship yet —
confirmed by `grep -r '\.depends_on\(\|\.contributes\('` over `✏️s/🔌️plugins`, zero hits. Per this
ticket's own master plan, wiring a REAL cross-plugin dependency/contribution is explicitly **W3 pilot
work** ("W3 pilots (flow composite, cad↔aec-building contribution, cross-artifact transaction) prove
the mechanisms end to end"), not W2's. Consequently:
- The wasm e2e's `PluginGraph` rejections are proven against **synthetic** manifests built by cloning
  a real plugin's manifest and overriding its `dependencies` — the graph logic itself is real (the
  exact `PluginGraph::register` a real dependent's real manifest would go through), only the
  dependency EDGE is synthetic.
- **A full end-to-end composite-mutation transaction over real wasm** (a real guest proposing a
  transaction because its own `Mutation::foreign_steps` is non-empty) is not achievable with today's
  plugin fleet — no shipped plugin overrides `foreign_steps`. `HostTransactionCoordinator`'s full
  commit + group-undo-restores-both-members path is instead proven by
  `host_transaction_coordinator_tests::a_two_member_transaction_commits_and_group_undo_restores_both`,
  a pure-Rust in-process fake (`FakeCluster`) that faithfully implements the identical two-phase wire
  protocol contract §5 defines (`TransactionPrepare`/`Commit`/`Rollback`/`Undo`, one `pending` per
  instance, `TransactionCommit` applies as one edit stamped `group_id = txn_id`) — this proves the
  COORDINATOR's own orchestration (resolution, phase-1 all-or-nothing, reverse-order commit,
  compensation, group fan-out) deterministically; it is not a real wasmtime call. The wasm e2e instead
  exercises the parts that genuinely are exercisable today: real component loading, real manifest/
  roster/metadata decode, real graph/router/directory wiring.

### Building the wasm artifacts

Per instructions, I did not invent a build command — used the same one `🏗️IoRouterE2e`'s own doc
comment and `🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts`'s `buildPlugin` use:
`cargo build -p <package> --target wasm32-wasip2 --profile dev`. **This currently fails** for a fresh
build of `semio-s-plugin-cad`/`-stdio`/`-flow-extension-math` — not because of anything in this
lease, but because their shared dependency `semio-framework-plugin` (the guest SDK, W1-A/W1-B's
lease, `🔌️plugin/🦀️component.rs` + `🔌️plugin/🏗️builder/🦀️component.rs`) does not currently compile
(confirmed via two spaced retries showing a *different* error signature each time — e.g.
`ArtifactStore::new` return-type mismatch, `IoWireError` vs `String` — live, uncommitted, actively
churning). The e2e test above therefore runs against the pre-existing `.core.wasm` files already
checked into `🧑️‍💻️dev/🔌️plugin-modules/{cad,stdio,flow-extension-math}/` from a prior successful dev
build (cad/stdio dated 2026-08-12, flow-extension-math 2026-08-15) — these turned out to already be
WIT-compatible with the `contributor` interface this ticket added (the e2e test's real calls to
`list_artifact_mutations()`/`list_artifact_inferences()` on them succeed), so the proof is real even
though I could not produce a byte-fresh build myself. Flagged for the coordinator: once
`semio-framework-plugin` stabilizes, a fresh `bun nx run @semio-tech/framework-os-dev:build -- cad`
(etc.) should be re-run and this e2e re-verified against the fresh artifacts.

## Gates (real output, this session)

### `cargo test -p semio-framework-plugin-host --lib`

```
running 38 tests
test component::artifact_inference_router_tests::only_exactly_echoed_guest_results_are_publishable ... ok
test component::artifact_mutation_router_tests::conflicting_owner_rows_are_rejected_unless_byte_identical ... ok
test component::app_router_tests::contribution_without_a_declared_dependency_is_rejected ... ok
test component::app_router_tests::duplicate_app_ref_is_a_conflict ... ok
test component::artifact_mutation_router_tests::a_contribution_onto_a_non_dependency_is_rejected ... ok
test component::app_router_tests::owned_surface_gaps_reports_the_missing_role_only ... ok
test component::app_router_tests::contribution_with_a_declared_dependency_is_admitted_and_sorted_after_the_owner ... ok
test component::app_router_tests::owner_surface_sorts_first_then_plugin_id_then_app_id ... ok
test component::artifact_mutation_router_tests::owner_and_contributed_rows_both_resolve_correctly ... ok
test component::app_router_tests::unregister_plugin_drops_its_surfaces_but_keeps_its_ownership_claim ... ok
test component::artifact_mutation_router_tests::unregister_drops_only_that_plugins_rows ... ok
test component::host_transaction_coordinator_tests::a_cycle_is_rejected ... ok
test component::host_transaction_coordinator_tests::an_unknown_mutation_is_rejected ... ok
test component::host_transaction_coordinator_tests::an_unknown_target_is_rejected_before_any_prepare_is_sent ... ok
test component::instance_directory_tests::bind_resolve_and_unbind_round_trip ... ok
test component::instance_directory_tests::rebinding_the_same_artifact_id_replaces_the_prior_location ... ok
test component::host_transaction_coordinator_tests::a_chain_deeper_than_max_plan_depth_is_rejected ... ok
test component::host_transaction_coordinator_tests::a_two_member_transaction_commits_and_group_undo_restores_both ... ok
test component::opening_resolver_tests::step1_explicit_default_still_in_router_wins ... ok
test component::host_transaction_coordinator_tests::a_member_rejection_rolls_back_every_already_prepared_member ... ok
test component::opening_resolver_tests::step2_and_step3_collapse_to_the_owner_surface_when_default_is_stale ... ok
test component::opening_resolver_tests::step3_first_entry_when_the_owner_has_no_surface_for_this_role ... ok
test component::opening_resolver_tests::step4_unknown_dialect_when_the_router_has_nothing ... ok
test component::plugin_graph_tests::hot_reload_is_rejected_when_it_would_break_a_live_dependents_version_requirement ... ok
test component::plugin_graph_tests::a_later_registration_that_would_close_a_cycle_is_rejected ... ok
test component::plugin_graph_tests::register_rejects_a_missing_dependency ... ok
test component::plugin_graph_tests::register_rejects_a_version_mismatch ... ok
test component::plugin_graph_tests::load_order_respects_a_real_dependency_edge ... ok
test component::plugin_graph_tests::unload_is_refused_while_a_dependent_is_registered ... ok
test component::tests::io_router_routes_a_real_cross_plugin_compose_between_two_loaded_wasm_plugins ... ok
test component::tests::wasm_plugin_runtime_api_exists ... ok
test component::tests::plugin_dependency_infrastructure_wires_real_loaded_plugins_and_one_real_extension ... ok
test component::tests::wasm_plugin_runtime_loads_real_plugin_component_if_present ... ok
test opening_config::component::tests::opening_preferences_default_is_empty ... ok
test opening_config::mutations::clear_default_app::mutation::tests::clear_default_app_label_names_role_and_dialect ... ok
test opening_config::mutations::component::tests::set_default_app_and_clear_default_app_invert_each_other ... ok
test opening_config::mutations::set_default_app::mutation::tests::set_default_app_label_names_role_and_dialect ... ok
test component::tests::extension_runtime_constructs_engine_and_linker ... ok

test result: ok. 38 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

`component::app_router_tests`/`opening_resolver_tests`/`opening_config::*` are a DIFFERENT concurrent
ticket's tests (`ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET`'s `AppRouter`/`OpeningResolver` regions,
which appeared in this shared file between my initial read and later edits — confirmed via
`git status`/region-marker diffing, never touched by this lane) — included here only because they run
in the same `cargo test` invocation; they pass, which is a good sign the tree was stable at gate time.

### `cargo check -p semio-framework-plugin-host -p semio-framework-os-kernel`

**PASS**, clean (warnings only):

```
warning: `semio-framework-plugin-host` (lib) generated 4 warnings (run `cargo fix --lib -p semio-framework-plugin-host` to apply 4 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 0.24s
```

All 4 warnings are pre-existing/external (`unused extern crate` in `📦️glue.rs`, an unused import in
the concurrent `opening_config` region, one unused-variable warning in an unrelated pre-existing
match arm) — none in code this lane added.

### `cargo check -p semio-framework-os-run` (extra diligence, not a required gate)

**Could not get a clean run — honestly reporting, not fabricating a pass.** This crate (my
`🏃️run/🦀️component.rs`/`📦️bin.rs` changes live here) transitively depends on
`semio-framework-plugin` (the guest SDK) via `semio-framework-os`, and that crate does not currently
compile — confirmed via 4 spaced retries, each showing a genuinely different error set (first two:
`semio-s-plugin-stdio` syntax/missing-file errors under active edit — `git status` shows 227 modified
files under `✏️s/🔌️plugins/🗄️stdio/`, the FULL-STDIO ticket's live work; second two: `E0432`/`E0599`/`E0061`
all inside `🔌️plugin/🏗️builder/🦀️component.rs`, W1-A's lease, not mine). This is squarely this
ticket's own documented "known external breakage" class, confirmed by file attribution each time
(`git log --date=iso`), not assumed. I therefore reviewed `🏃️run/🦀️component.rs`'s new code
manually against the exact APIs `semio-framework-plugin-host --lib` already compiles and tests
clean (same method names, same signatures, same error types) — every new symbol I call
(`PluginGraph::{register,load_order,dependents,guard_unload,unregister,prepare_hot_reload,
commit_hot_reload}`, `ArtifactMutationRouter::{register_plugin,roster}`,
`ArtifactInferenceRouter::register_plugin`, `InstanceDirectory::bind`, `IoRouter::unregister_plugin`,
`HostTransactionCoordinator::{run_transaction,undo_group}`, `HostArtifactMutationPlanRequest/Result`)
is a real, gate-verified `pub` item in `semio-framework-plugin-host`. I could not run `cargo test -p
semio-framework-os-run` or my own e2e wiring in `WasmtimeNodeHost` for real; recommend the
coordinator re-run `cargo check -p semio-framework-os-run` once `🔌️plugin/🏗️builder/🦀️component.rs`
(W1-A's lease) stabilizes.

## Notes for later waves / whoever mirrors this in W2-B

- `PluginGraph`/`ArtifactMutationRouter`/`ArtifactInferenceRouter`/`InstanceDirectory`/
  `HostTransactionCoordinator` are all plain `pub` types in `semio-framework-plugin-host` with no
  wasmtime-specific state beyond `Arc<WasmPluginRuntime>` handles the CALLER supplies — the graph/
  router/directory logic itself is host-runtime-agnostic (the coordinator in particular is driven
  entirely through closures), so W2-B's TS host can mirror the SAME rules without needing to port
  Rust code, only the same decode shapes (`WireMutationRosterEntry`/`WireArtifactMutationPlanRequest`/
  `Result`, both DSL-wire-encoded, not JSON) and the same two-tier owner/contributed lookup.
- The `blake3`-based `payload_hash_of` in `HostTransactionCoordinator` matches
  `📡️spr/🎮️command/🦀️component.rs`'s own `Planner` cycle-guard hash exactly (`blake3::hash(bytes).as_bytes()`)
  — W2-B should use the SAME algorithm (not e.g. a JS-side different hash) if it ever needs to
  construct a `MutationOrigin::Contributed.payload_hash` for cross-host provenance comparison.
- `HostTransactionCoordinator::run_transaction`'s pre-planned-form-always choice (never the
  owner-mutation wire form) is a deliberate reading of W1-B's own recommendation, not literally
  spelled out as mandatory in the contract — flagging for the coordinator in case a future contract
  revision adds an `origin` field to the owner-mutation form instead, which would let a host choose
  either form.
- `InstanceDirectory` population is currently wired only at `WasmtimeNodeHost::open` (instantiate-app
  time). If a future caller opens plugin instances through a path that never calls this host's own
  `open` (unlikely for this crate, but worth flagging), `HostState::pre_adopt_command_packs`'s
  `Hello`/`LoadDocument` match arms are the documented alternate seam.
- `WasmtimeNodeHost::hot_reload_plugin`/`unload_plugin` are net-new methods with **zero existing
  callers** (nothing in `🏃️run`'s CLI/`SpaceRunner` flow triggers a reload/unload today) — they are
  infrastructure for whoever wires a live dev-boot hot-swap path (browser side already has one per
  scout-2 §4/§5; the native host did not before this ticket).

## Files touched (summary)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/📦️bin.rs` — inspected, no change required
