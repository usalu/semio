# Native Query Refusal Audit 24

Read-only source audit. No tests, builds, runtime acceptance, git mutations, or source edits were performed. Source observations are bounded to three full-file before/after captures in `🗑️generated/root-native-query-refusal-audit-24-source-receipt.json`; receipt hashes qualify simultaneous edits. This is not an all-platform or whole-OS acceptance claim.

## Confirmed Receiving Behavior

Math `ray_pick_instance` (2170), `ray_pick_mesh_detail` (2195), and `interpolate_mesh_uv` (2245) return repository-owned `Result<Option<...>, Mesh3dFault>`. Mesh access faults propagate through `?`. Geometric misses and absent UVs explicitly return `Ok(None)`. There is no `.ok()?` fault collapse in these observed bodies. The retained authority lock exposes Busy separately from semantic Closing; source enum and gate mappings preserve this distinction.

World production Paint receiver at 5089 maps Busy to `WorldInteractionStep::Pending`, all other mesh errors to Fault, and missing UV to Fault. Authority Pick receiving branch at 7298 stores the original cursor, consumes one context fuel, and returns Pending. The UV finish path makes no blocking spin/retry loop. The resumed cursor uses `context.should_yield()` at 4864. Cancellation and deadline acceptance beyond this visible StepContext transport remain outside this source-only acceptance.

Repository-wide Rust call search found the one production direct UV call above; additional direct ray/UV callers at 15421, 15449, 15450, 15530, 15552 are in individually `#[cfg(test)]` functions. They use finite test query admission. They do not constitute overlooked production receivers. Public signatures use repository geometry reexports and Mesh3dFault, not direct foreign runtime types.

## Actionable Adjacent Producer Finding

WorldTerrainMeshJob `step_live` at 9123–9179 does not preserve the new refusal distinction: begin errors are all mapped to ByteCapacity (9133); allocate demand, allocate execution, writes, and seal errors are all mapped to Closing (9140–9179). Therefore actual Mesh3dFault::Busy is converted into a permanent semantic fault by this production producer rather than retained Pending. The phase/item cursors only advance after successful writes, so retaining the producer on Busy should be feasible without replaying accepted writes. Root and HighNative were notified; no concurrent code was changed by this auditor.

The same observed producer creates constructor and allocation grants directly from `mesh3d_begin_capacity_byte_demand()` and `mesh3d_allocate_capacity_byte_demand(token)` (9132, 9140). No independent capacity allowance is visible in `step_live(&mut self)`. This is a source-level internally demand-funded grant finding, not a measured allocation receipt failure. Independent authority could exist elsewhere; no claim that the entire producer chain was exhaustively proved.

## Limits

The Scene file is retained in the receipt but was not exhaustively analyzed. EngineCanvas typed DraftChangesJson publication is ongoing by another agent and was not frozen or mutated. No legacy alias or foreign public type leak was identified in these three query signatures; this does not establish repository-wide absence. No unbounded query retry was found in the specific actual UV receiving path. The existing 246-law and expanded producer journal results are parent-provided context, not rerun verification by this auditor.
