# Original Text Splice Composition

Package: semio-framework-text-splice-native. Owner: framework/ui/scene/text-splice/native, with its own packages/rust Cargo manifest and framework owning workspace. The package depends only on first-party job and value interfaces; no dependency on the scene facade or external runtime library.

Public views, owned result types and composer API follow the coordinator contract. Implementation and native tests are in progress and no source-ready or native-pass claim is made. The input owner must retain the exact immutable pair until the composer is physically closed; the composer validates original source, decoder, pointers, lengths, starts and a bounded capture fingerprint.

## Source Readiness

The complete API and bounded implementation are now authored. Normal steps use an exact fixed metadata quote plus actual scalar/output allocation extents, with one scalar or frontier decision per step. Eight scalar buffers are counted, reserved, filled and fingerprinted; all window/original/typed/rotation access is virtual. Feasible intervals select the same nearest run then nearest next placement as the old ordered placement enumeration. Output UTF-8 bytes are counted before exact reserve and one-scalar copy. Every capture is revalidated before ready; the provider retains the pair immutable throughout. Physical buffers, partial output and shell use controlled original retirement. Static admit explicitly requires original_source_parent before birth.

Independent actual TS then + fast-check + JSON Patch comparison passed 35,775 fixture/run/generated pairs, including 24,000 arbitrary scalar/context pairs. The native tests reuse the existing composition and typing-run fixtures, compare 128 deterministic generated cases against actual old Rust then outside funded callbacks, and emit actual inputs/results for cross-language comparison. Three actual-caller/decoder/HeapWitness laws are authored: contract/denial/foreign-source equivalence; seven cancellation cuts; and expired source zero-effects plus paid closure without renewal. Native execution remains pending the coordinator's joint ready gate.

No cold then, collection of a complete virtual text, placement vector clone, external runtime dependency, or duplicate native allocation charge is used in production. The scene facade is dev-only for the old Rust algebra oracle.

The actual-record oracle now derives and compares the entire ordered expected record tape: every indexed fixture, every actual typing-run composition step, and all 128 deterministic generated pairs. This prevents missing or duplicated source/index witnesses from replacing one another. A hostile-log checker first validates a real native baseline, then requires five duplicate/missing-record corpora to fail. It has not been run without actual native output. Rust source and the active native snapshot remain untouched by this oracle correction.

## Actual Composer Native Pass

The second joint native request compiled and executed the composer suite: 3 passed, 0 failed, 0 ignored, 0 filtered. Actual record counts: {"fixture": 15, "generated": 128, "typing": 20}. All seven cancellation cuts and the expired original-source closure record were emitted. The strict independent TS/cold-then/fast-check/JSON Patch oracle passed against this actual log (35775 reference comparisons, nativeCompared:true). Five hostile corpora derived from the passing actual baseline were all rejected, including missing typing, duplicate fixtures/generated/typing records and missing expiry.

The same batch's pure core passed all 10 tests; its actual 13-scenario/43-step, five-cancel/four-relocation Scrub oracle passed. Pure EDIT executed both laws and failed its source-identity invariant before records (0 passed, 2 failed), so the whole wrapper exited 1. The defining owner is repairing that exact runtime issue. Full terminal output remains generated/typing-joint-native-2.log; no aggregate or end-to-end success is claimed.
