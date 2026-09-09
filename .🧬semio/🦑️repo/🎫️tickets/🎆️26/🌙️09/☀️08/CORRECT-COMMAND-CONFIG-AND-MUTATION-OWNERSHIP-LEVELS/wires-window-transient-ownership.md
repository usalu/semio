# Wires Window Transient Ownership

## Confirmed trace

Pointer down/up publish drag coordinates through `WiresConfigMutation::SetDrag`; pointer move reads that global config to compute a genuine document node move. The three drag fields also remain in the full artifact and sparse-diff schemas, while the actual document snapshot already excludes them. No active production reader/publisher was found for the duplicate Wires presence drag fields.

## Implementation scope

Register a concrete canvas `WindowTransientOwner`, retain the prior drag snapshot in the framework operation context, and route pointer lifecycle writes through the exact captured window authority. Preserve node movement as a document mutation. Remove drag from persisted app config, artifact/diff contracts, and unused presence projection. Update every schema representation and committed-diff fixture affected by removing artifact-only copies.

## Validation

Pending. The existing ownership gate needs these three newly traced exclusions; its earlier zero result did not cover drag field names incorrectly annotated as artifact state. Native tests must exercise two instances of one canvas kind, release/reset behavior, and unchanged config/document streams for pointer-only state.

## Artifact regression result

The added three drag-field exclusions made the live ownership gate fail on six Wires artifact/diff declarations. Removing the Rust/default/apply/absorb copies, all five schema projections, and six committed diff fixture copies made `abstraction-ownership-validation:enforce` pass (11 direct vectors, four nested vectors, three command vectors, 104 artifact/diff contracts, zero live breaches). The Wires contract pair is now explicitly included in the native artifact-law package set, expanding it from 51 to 52 crates. The move-node fixture now requires drag keys to be absent rather than present with null values.

The Wires pointer-move app-config migration remains pending; this gate result does not claim that migration is complete. The unused presence contract has been removed. Non-Rust Wires artifact/snapshot/diff projections now use the actual document fields and shared artifact child identity contracts; see `wires-document-contract-ownership.md`.

## Current implementation checkpoint

The concrete canvas now registers `WiresCanvasTransientOwner` under its own window taxonomy node, with all five schema formats, typed mutation descriptor/codecs, and migrated neutral mutation vectors. Retained pointer down/up publish `WindowTransient`; pointer-down hit lookup advances one node field per work step rather than materializing the whole board. They read the exact captured transient snapshot from the retained job context. Their direct unscoped handlers reject dispatch. Native run 6 passed all five selected tests through Nx in 3 minutes 34 seconds. The production lifecycle test binds the actual app instance, exercises two concrete canvases with generations 2/0 after down/up, checks empty terminal drag projections and unchanged document/config packs, and completes bounded close. This trace does not yet cover a positive node hit or pointer move.

Remaining required Wires work: `canvasPointerMove` still reads/publishes the old app config and is classified batch-only. Move it to the captured window transient input while keeping document reposition as a genuine document mutation, supply the retained document preparation/progress path, then delete the old `WiresConfig`. Update example/document replacement to clear gesture state under the correct lifecycle. Existing pointer tests assume unregistered synchronous dispatch and need conversion to real retained dispatch. The pointer-down/up work does not complete the whole drag migration.

The retained route fixture is nine actual commands (previous schema said ten), with two window-transient routes. Host effects are implicit; the window-transient publication lane cannot also advertise the exclusive host-only lane. The descriptor now lives under the editor command schema.
# Command Metadata Ownership

The retained command-route contract was physically misplaced under the artifact's document schema `$defs`. It now lives under `✏️editor/🎮️commands/🧬️schema/🔣️.json`; the existing editor fixture remains an editor-level test input. Document projections reference the canonical OS Store child and framework IO addressing contracts.
