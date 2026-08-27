# Flow Extension Native Unblock

## Observed Compiler Boundary

The coordinator's shared Flow native run failed before executing its eleven selected tests. The captured log reports 171 errors in `semio-s-plugin-flow-extension-brep`; it does not contain full spans. These are compiler failures, not failing Flow lifecycle tests. The core package intentionally lists Brep, Primitive, Math, Text, Logic, Dictionary, and List as dev-dependencies, with component guests disabled. All seven real extensions must remain in the test graph.

## Source Diagnosis

Only Brep still declares synchronous `Operator::evaluate` implementations as `async fn`: 39 source declarations, including five macros that expand into multiple operators. The six other core-test extensions already implement synchronous evaluation. Brep's bodies use synchronous kernel closures and contain no `.await`.

The Brep extension imports `flow_extension_sdk::brep_geometry::*`, but that framework module privately imports `BrepKernel`. Consequently, the extension does not bring the existing kernel trait into scope. The actual trait and its concrete `Brep` implementation exist in the stdio Brep engine; `box_prim`, `import_step`, and related methods are synchronous and are not missing implementations. The extension already directly depends on stdio. The precise repair is an explicit import at the extension use boundary, preserving the real geometry implementation and feature graph.

The remaining headline E0308 diagnostics need a detailed warm compiler pass after the two certain source corrections; their spans are not present in the retained coordinator capture.

## Ownership Audit Across All Seven Extensions

The six already-synchronous extensions are not yet certified against strict Dictionary ownership. In particular, List `read_list` returns an owned dictionary that is abandoned by read-only operators and on errors; Get builds an owned partial output before fallible per-element selection. Dictionary Merge builds a partial dictionary before fallible subsequent input reads. Tests and component JSON entrypoints also create temporary registries and owned input/output dictionaries. These are explicit cold evaluation/codec boundaries and need `ColdOwner`/cold-builder ownership, including error paths. They must not weaken the Dictionary terminal guard or earn retained-job credit.

## Repair Sequence

Source checkpoint: the 39 declarations and explicit trait import are corrected. All seven extensions' temporary module-registry calls now use explicit cold scopes; shared JSON evaluation retires input/output/tree owners and cloned manifest catalogues. List borrows its input list and uses a cold partial-output builder; Dictionary Merge uses a cold partial-output builder. Extension test tails remain an explicit regression follow-up. No native result is implied.

1. Correct the 39 Brep trait declarations and explicitly import the existing stdio `BrepKernel` trait.
2. Check all seven production evaluation/JSON boundaries for owned Dictionary temporaries; use existing explicit cold scopes and preserve returned ownership.
3. Ask the coordinator for a detailed warm Flow-core check, then fix concrete remaining diagnostics. Do not disable geometry or remove dev-dependencies.
4. Rerun the eleven shared lifecycle tests and full relevant extension regressions. No native pass is claimed by this report.

## Independent App Packet Status

The app's four retained Artifact mutation recipes cover five existing live routes, use selected-item copying and targeted inverses, and feed the Store canonical sealer. The source oracle passed four cases and four hostile fixtures with a 4,800-byte semantic label; native recipe/publication tests remain unrun. Config preparation now admits one-byte grants through chunked edit metadata. The new scalar-record wire witness is source-only and awaits exact OpBinary fixtures and integration; the previous HostOnly JSON witness was not the actual command wire format.
