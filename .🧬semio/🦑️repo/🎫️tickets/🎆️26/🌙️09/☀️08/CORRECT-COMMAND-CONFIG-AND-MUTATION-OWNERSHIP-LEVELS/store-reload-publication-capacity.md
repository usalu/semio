# Store Reload Publication Capacity

The Wires pointer runtime regression loaded a zero-history pack and then tried one durable MoveNode. It reached Store commit preflight and rejected because the applied-id, cursor-id and revision arrays had no preinstalled capacity. Evidence: `🗑️generated/wires-pointer-move-stack-diagnostic.log`, test `wires_pointer_move_uses_only_the_captured_canvas_and_publishes_document_positions`.

The call uses `load_document_pack → ArtifactStore::reset → set_state`, rather than the separate retained initializer. `ArtifactStore::new` and `ArtifactStoreInitializationRuntime` already installed fixed history catalogs. `set_state` instead adopted incoming empty Vecs, created an empty revision accumulator, and `bump` copied those zero capacities into the persisted cursor. Shared Store replacement of applied/redo ids had the same capacity loss.

The fix makes Store initialization catalog capacity an invariant of replacement id owners and new revision accumulators. Allocation occurs before root replacement, and the durable publication capacity check remains intact. A neutral two-case regression reloads empty history, publishes one retained batch and compares the resulting scalar with serde_json. Runtime validation is pending.
