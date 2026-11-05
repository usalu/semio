# Original Surface View Read

## Source Contract

`semio_plugin_plugin_program::surface_context::OriginalSurfaceViewRead` now owns the genuine registry capture lease, original caller/decoder identity, selected-slot traversal, original RetainedCloneSource admission and exact source restoration. It contains no ViewModel clone cursor. The existing OriginalSurfaceViewCapture embeds the same pin in its stable Box and retains its typed clone cursor/output; the cached registry lock identifies the stable embedded pin address.

Public read API: `admit(cx,&mut Option<Box<Self>>)`, `birth_demand()`, `advance(registry,cx)`, `advance_surface(registry,surface,cx)`, `ready()`, `borrow(&self,&registry,&cx)`, `close_step(registry,cx)`, `close_demands(registry)`, `terminal_is_empty()`, `release_demand()`.

Borrow returns the actual `RetainedCloneRef<ViewModel>` tied to both the pin and registry lifetime. Pending source admission is a WorkLimit control refusal; it never becomes business absence. Only a ready, genuinely absent original returns None. Source moves during admission but no original field is cloned. The actual registry mutation lease remains held across ready frames. The original returns to its exact vacant full-cache or selected-slot destination before unlocking.

## Defining Factor

All source pin, selection, admission, return and restoration logic exists once in OriginalSurfaceViewRead. Capture delegates it and preserves its original public clone API. No public pending callback signatures changed.

The read construction and step adapters now forward every child's complete raw work independently of progress, success or failure. Existing original construction owns allocation accounting. The child event uses the actual available original item grant, preserving cursor item semantics, while metadata and Box births remain explicit single transitions. Cursor advance/take/close, admission advance/take/return, source take_authority and output retirement preserve exact raw work. No max(Err), copied-item CPU inference or child work1 wrapper remains in read.rs.

Pin and capture closure use original retirement predicates and actual same-source retirement loans, including an expired semantic deadline. SurfaceContexts.close_original_step now uses the same raw retirement forwarding so registry cleanup cannot discard underlying work. The unrelated refresh algorithm is unchanged. Both new read and factored capture have unconditional terminal Drop assertions.

## Schema and Independent Proof Before Production Edit

Authored read-pin schema/corpus:16 cases covering cache/selected originals, genuine absence, repeated ready borrow, busy lease, zero grant, canceled prefixes, pending-not-absence and foreign caller/decoder/registry/selector. Registered Bun test `1105-view-read-oracle.mjs` passed strict Ajv2020 validation and independent SQLite ownership-transfer/restoration transactions BEFORE read.rs production changes.

## Native Laws Source Ready, Not Run

Three laws are mounted in the existing semio-plugin-runtime-ops test owner under its existing original-actor-context-testing feature. They use real OriginalJobFixtureCaller + OriginalJobDecodeOwner, actual SurfaceContexts fixtures and actual ViewModel fields. No StepContext or RetainedCloneRef is constructed in tests.

Coverage includes original payload/roster/map pointer equality before, during and after the pin; pin birth allocation independence from a large Unicode panel payload; repeated no-clone ready borrows; busy second-pin isolation; pending refusal vs genuine None; actual same-label foreign caller, original decoder and registry substitution; selected string allocation substitution; canceled prefixes; constructor and close-axis denial; semantic-deadline-expired restoration; typed clone capture preservation; same-caller registry/typed output/source/decoder/Box/caller cleanup; exact per-turn physical heap receipts, unchanged original stock and deferred full200-call bound.

Native execution and typing are NOTRUN pending Runtime's sole coordinated native owner. No Cargo, native compiler, sync, manifest/dependency or lower Value/Job edit was performed. Source-only parser verification is not a compiler verdict.

## Source Census

Fresh unchanged production closure: 111,061 bytes across 8 files; cap262,144/32 leaves 151,083 bytes and 24 slots. This is a static literal source census, not native Source admission. Generated rows:1105-view-read-source-census.json.


## Files

- plugin/reactor/surfaces/read.rs: common read pin, capture factor and exact raw adapters.
- plugin/reactor/surfaces/native.rs: read reexport and exact registry retirement adapter.
- plugin/reactor/surfaces root: explicit public read export.
- plugin/reactor/surfaces/tests/read-pin/{schema,cases}.json: authored language-neutral contract and16 cases.
- plugin/reactor/runtime/ops root: native law mount using existing original-actor-context-testing.
- plugin/reactor/runtime/ops/tests/original-view-read.rs: three actual-registry/caller laws.
- Ticket1105-view-read-oracle.mjs: independent Ajv2020/SQLite test input retained.
- Ticket1105-original-view-read.md: this report.

Registered Rust syntax verification: read.rs, native.rs, native law3 files,0 errors; surface root was also parsed earlier with0 errors. All new paths remain within256 UTF16 units.
