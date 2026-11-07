# November 5 Source Admission Independent Review

Read-only current-source inspection; no heavy test or new CLI observer. Focused results and public Rust verification are reported in nov05-admission-capture.md and were not independently rerun. Full Ajv and loader identity suites remain pending.

## Finding

Node interactive transport validates only maximumSnapshotBytes in buildSourceClosureNodeArgumentsV1 and the generated loader. The loader also compares against maximumFiles, maximumFileBytes and maximumSourceBytes from the supplied policy without validating their presence, type or upper bounds. A forged missing field makes the greater-than comparison false; inflated fields relax individual admission ceilings up to the 32 MiB snapshot ceiling. The retained forged-policy test mutates only maximumSnapshotBytes. Validate the entire closed policy against the schema ceiling (or exact declared policy), both at producer and loader boundary, and test missing, mistyped, additional and oversized fields. This observation concerns the transport policy boundary; it does not itself show an issued Source capture can be forged or a normal caller supplies such a policy.

## Other Observations

The census returns a separate frozen metadata object before the normal branch attaches private WeakMap byte/import authority; captured-byte and import access require the actual captured object. Traversal uses the same source-read and discovery machinery, records first origin before queueing, and attributes root and workspace manifests to the importer that first causes capture. The census cannot itself provide bytes to the loader.

The snapshot formula starts with the UTF-8 byte length of the empty files-array JSON, adds each serialized object with empty base64, four bytes per three source bytes rounded up, and commas between objects. JSON.stringify and Buffer.byteLength account for Unicode and escaping in root, paths and imports. It correctly measures snapshot JSON without the separate Node newline. The retained small actual-capture check compares it with serialized output, but no explicit escaped or Unicode path/import fixture was observed in that focused harness.

Census capacity is separately 8192 files/64 MiB; normal helper uses schema 2048 files/20 MiB source/32 MiB snapshot, and per-file read still caps 4 MiB. Capture checks cancellation around reads, projection checks on entries and specifiers, with phase progress at projection start/end and each 16 projected units. These checks do not yield during synchronous projection, so externally delivered cancellation and progress depend on event-loop scheduling; the focused test aborts synchronously from its callback. Sorting and final mapping have no cooperative yield. No unrelated ceiling relaxation was established in the inspected paths.

Transport permits snapshot size at the exact bound plus one newline, and child strips that newline before the snapshot check. General execution, producer/consumer schema parity, and loader identity need the pending full suites. No conclusion here replaces real course and native runtime gates.

## Transport Fix Re-review

The Node producer and both generated loader forms embed the same predicate and schema-derived six-field canonical object. It requires an ordinary or null prototype, exactly the canonical own keys including symbols in the count, safe integer values and exact equality. The loader receives a JSON-parsed or JSON-embedded policy and rechecks it, so a producer-side getter cannot make a forged serialized policy pass the loader; JSON cannot carry getters, custom prototypes or symbols. A caller-supplied accessor may still execute during producer validation/serialization, but no authority bypass was established. The previous missing/inflated-field gap is closed in current source.

Projection now yields before each 16-source batch and tests cancellation after the yield. Each batch projects against the complete contents map, preserving cross-batch import lookup. The report records actual Node 19 policy refusals, valid image, Unicode/escaped size parity, cross-batch parity, timer cancellation and current-source Rust verification; this review inspected source without repeating those executions. Census returns before captured WeakMap registration and no new authority leak was found. Full Ajv and loader-identity suites remain pending.
