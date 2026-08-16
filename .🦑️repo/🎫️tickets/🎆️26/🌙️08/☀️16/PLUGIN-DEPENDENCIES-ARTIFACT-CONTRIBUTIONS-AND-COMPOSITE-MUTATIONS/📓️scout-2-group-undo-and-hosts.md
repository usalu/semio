# Scout 2 — Group undo, coordinator surface, host loading

Read-only survey (Haiku), 2026-08-16. Binding constraints for lanes W1-C, W2-A, W2-B.

## 1. Group undo already exists — but only over member TAILS

`🏪️store/🦀️component.rs:6035` `CompositionCoordinator::undo_group(members, group_id) -> GroupUndoReport` undoes a member **iff `member.tail_group_id() == Some(group_id)`**, skipping (never aborting on) foreign or failing members; `redo_group` at :6056. There is **no** walk that reverses every edit in a store sharing a `group_id` — only the tail.

**Consequence:** this is sufficient for our protocol exactly because each transaction member commits **one** `Edit` (contract §5.6). W1-C therefore *extends* rather than *invents* group undo, and must keep the one-edit-per-member invariant explicit.

## 2. CompositionCoordinator is already byte-opaque — ownership is the only blocker

`dispatch_group(parent_ref, parent: &mut dyn SpaceMember, children: &mut [(&mut dyn SpaceMember, ChildDispatch)], parent_ops: Vec<Vec<u8>>, genesis, meta) -> Result<GroupReceipt, VcsError>` (:5924). `ChildDispatch { child: ArtifactRef, ops: Vec<Vec<u8>>, op_schema, labels }` (:5572) — ops are never decoded, only hashed and framed. `GroupReceipt { invocation_id, member_edits, created_children }` (:5598). Compensation (:5878) undoes parent then children in reverse.

The hard blocker for peer-to-peer members is phase-1 validation (:5938-5941):

```rust
match self.graph.owner_of(&dispatch.child.artifact_id) {
    Some(owner_id) if owner_id == parent_ref.artifact_id => {}
    _ => return Err(VcsError::OwnershipViolation(...)),
}
```

**Consequence (binding on W1-C):** generalize to `TransactionCoordinator` with a member-relation mode — `Owned` keeps today's `owner_of` check verbatim (existing tests must stay green), `Peer` skips the ownership check while keeping cycle detection and the shared `invocation_id`/compensation machinery. Do not delete or weaken the owned path.

## 3. Rust host loads plugins lazily; there is no instance directory

`🏃️run/🦀️component.rs:1209` `runtime_for(plugin_id)` loads on first use, registers the blob store and the shared `IoRouter`, then caches in `self.runtimes: HashMap<String, Arc<WasmPluginRuntime>>`. `WasmtimeNodeHost::open()` (:1224) calls it before `create_app`. Plugin→artifact-instance coupling exists only as `WorkflowNode.plugin_id`. `ArtifactInferenceRouter` is **not** wired in `🏃️run` today (only `IoRouter` is).

**Consequence (binding on W2-A):** `InstanceDirectory` is genuinely new; `PluginGraph` must sit in front of `runtime_for` so a dependency is loaded before its dependent; the inference router needs wiring alongside the io router.

## 4. Browser host: `contributes`/`consumes` is the only dependency notion, loading is lazy

`🎠️kernel/🟦️component.ts:140-146` `expandPluginRegistry` keeps the primary plugin plus any entry whose `contributes` intersects the primary's `consumes`; `resolvePlaygroundBoot` (:1215) builds that list; `acquirePluginModule` (:965) leases one module at a time through a worker-backed pool (main-thread `import()` fallback). No eager batch load, no ordering guarantees beyond array order.

**Consequence (binding on W2-B):** `dependsOn` must join `contributes`/`consumes` in the registry entry, and boot must walk the dependency order from `PluginGraph` instead of relying on array order.

## 5. Nothing considers dependents on reload/unload

`WasmPluginRuntime::hot_reload` (:634) re-reads the binary, manifest, store and bindings in isolation; `ExtensionRuntime::unload` (:1184) deactivates and drops the instance; TS `evictPluginModule` (:975) drops the pooled lease. No path asks who depends on the plugin being replaced.

**Consequence:** the "hot reload re-validates dependents / unload refused while dependents are loaded" requirement is entirely new code in W2-A and W2-B.
