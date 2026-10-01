# Semio TypeScript SQLite Snapshot Implementation

## Scope and current evidence

All nineteen native Semio providers already have handwritten DDL and native laws recorded in `🦀️semio-report.md`. All nineteen TypeScript providers are now implemented and the fresh registered source gate passed 62 laws and 517 assertions across nineteen files. The final public declaration-consumer gate and default source gate are now green. The fresh native gate is now green: 108 tests passed. Earlier evidence below is retained with its actual scope.

The newest uncached `@semio-tech/stdio-semio:test --skip-nx-cache` gate passed all thirteen suites in 1m7s: 41 built outputs and 40 tested runtime exports, including independent declaration-consumer compilation. Model's separately registered uncached source gate passed two laws and 23 assertions. The old ten-suite result below is retained as earlier evidence, not the latest coverage boundary. While the shared Nx artifact generator's resolved cache inputs are under repair, validation explicitly skips Nx cache despite correct authored facet inputs.

The public `@semio-tech/stdio-semio:test` package gate passed the first ten suites in an uncached 58.5-second execution. This includes actual package build, exported runtime functions, independent declaration-consumer compilation, owned suite checks and runtime SQLite oracles. Kit subsequently passed `@semio-tech/stdio-semio:check` in 53.2 seconds and the artifact-owned `@semio-tech/stdio-semio-rs:test-snapshot-sqlite-source kit` target: two laws and 23 assertions, including independent SQL edits of an unsigned 64-bit blob size.

The existing `@semio-tech/stdio-semio-rs:verify-stdio-document-contract` route freshly passed in 5.9 seconds after the shared geometry change. It exercises Object and Kit committed snapshots, diffs and mutations against independent Ajv/JSON Patch oracles and performs strict TypeScript compilation. Geometry fixtures retain native primitive numeric wire shapes; explicit, named test constructors construct the corresponding owned binary words. No runtime compatibility union or reflective conversion was introduced.

## Owned scalar semantics

Shared native geometry uses binary64 coordinates, quaternions and transforms and binary32 colors. The actual TypeScript owner now uses first-party `Binary64` and `Binary32` words, with strict per-field parsers. This preserves positive and negative zero, subnormals, infinities and quiet/signaling NaN payloads. Physical SQLite rows expose named numeric query fields plus authored exact bit/class companion columns. Audio's native binary32 sample field retains its individually authored bits-only schema. Video retains signed64 rational fields and unsigned64 timestamp words; Kit retains unsigned64 blob descriptor size as canonical decimal text in SQLite and bigint in the owned model.

Kit's native `schema: String` has no fixed-schema validator. Its TypeScript owner and neutral JSON schemas now preserve all strings consistently. Object retains its fixed schema guard because the native Object validator explicitly enforces it.

## Domain mappings and integrity

Each domain owns a SQL literal equal to its handwritten native `.sql` source. Providers explicitly insert named scalar and relationship rows. Genuinely native byte buffers alone use BLOB cells. Ordered collections retain duplicate keys where the native model allows them. Typed SemioValue sharing is limited to actual owned SemioValue fields in Value, Table and Graph; it is not an artifact-wide fallback.

Kit's eleven tables distinguish catalog types, designs, pieces, piece connections, external artifact references, each child collection, optional value child, representation pins and blob descriptors. It rejects dangling references, cross-design piece endpoints, duplicate native IDs, multiply owned reference/blob descriptors, malformed pin shapes, invalid child coordinates and noncanonical unsigned sizes. Its query oracle checks real joins and foreign-key consistency, then edits SQLite directly and reconstructs the exact typed result.

Mesh's ten tables distinguish mesh/primitive topology, every ordered vertex attribute, indexed position relationships, binary32 materials and genuine texture bytes. The shared neutral fixture's formerly ambiguous `primitive_id` join was qualified after independent SQLite rejected it.

## Controls and registration

Shared TypeScript `ArtifactSqliteProjection` now exposes explicit additional-row and additional-value-byte preflight checks. Ordered reconstruction has cancellation before result allocation and at 256 rows. Large owned byte copies checkpoint before allocation; nested values use iterative reconstruction, including a 2,000-level law.

Semio owns its native/source SQLite commands in its Rust facet script and Nx targets; framework artifact-specific routes were concurrently retired and were not restored. The shared runner now uses the explicit fixed nineteen-suite `snapshotSqliteTests` option. Native, source and combined Nx targets and launch entries belong to Semio. The whole source suite exceeded the default 15-second fundamental budget before finishing the base branches; the existing `SEMIO_TEST_LEVEL=long` lane completed successfully in 21.4 seconds including fresh dependencies. Both Semio facets' cache inputs explicitly include artifact standards outside projectRoot, root TypeScript/native facades, shared SQLite core and generic artifact runners. Launch entries point at the artifact-owned targets.

## External runtime observation

Bun 1.3.14 `structuredClone` corrupted repeated aliased nested ArtifactRef objects in the original Kit expected-value fixture. A clone-only law reproduced this before any provider invocation. The fixture now constructs distinct references and transforms, matching Rust's owned clone semantics; no runtime codec workaround was added. The generated reproduction log is temporary ticket output. A native/provider defect was not inferred from the external clone failure.

## Files

The domain files reside under `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/<owner>/🧬️schema/📸️snapshot`: each authored `🪶️sqlite/🟦️.ts` and `🧪️tests/🪶️sqlite/🟦️.ts` mirrors its adjacent SQL and neutral corpus. Related owned scalar models/diffs/mutations are updated at their individual source owners. Package facade exports and TypeScript facet suite registrations are explicit. Object/Kit document-contract tests and the shared geometry fixture constructors now distinguish numeric native wire corpus from owned binary-word values.

## Continued Source Execution: CAD, Document, Presentation and Drawing

The independently authored CAD source suite passed its uncached owned Nx gate with both laws. Document then passed two laws and 23 assertions, including the complete native DocBlock algebra, independent image edits, 2,000-deep quote reconstruction and structural ownership errors. Presentation passed two laws and 22 assertions through independent SQLite. Its twenty-six handwritten tables preserve masters, layouts, slides, all shape and placeholder kinds, exact frame words, owned picture octets and separate notes/text/table-cell collections. Document and Presentation reuse only the actual shared DocBlock types through fifteen explicitly supplied table names; their root catalogs and table declarations remain individually authored.

Drawing's first uncached source execution passed two laws and 25 assertions. Its sixteen tables preserve all six path commands, style references, actual binary32 colors/opacity and binary64 geometry, every group/image/text/path field and a 2,000-deep iterative group tree. The previously malformed owned TypeScript parser flattened variants, declared group-nodes while the native type uses group, and accepted image bytes as text. Those defects were corrected in the owned snapshot model and parser. Matching drawing diff, inference and mutation consumers now use the actual owned geometry/word types; their additional typed consumer law is awaiting the seventeen-suite package test.

Actual public check evidence: fifteen suites passed uncached in 1m28s; sixteen suites passed uncached in 1m39s, including all emitted declaration consumers and owned suite typechecks. The seventeen-suite public package test is running. BRep and the base union remain unfinished at this checkpoint; no nineteen-provider completion is claimed.

The canonical central typed SQLite I/O and declaration registry remains Rust-owned. Source inspection of the TypeScript framework I/O seam found no corresponding artifact SQLite registration consumer. The owned TS API therefore exposes its explicit public to/from SqliteDatabase functions and tests real physical export/import using the shared engine, without introducing a parallel registry. Native exact declaration hooks and subset guards remain mandatory and separately covered by the existing native suite.

## Final Nineteen Source Evidence and Consumer Audit

`SEMIO_TEST_LEVEL=long NX_DAEMON=false bun nx run @semio-tech/stdio-semio-rs:test-snapshot-sqlite-source --skip-nx-cache` completed successfully: 62 laws, 517 assertions, nineteen files, 18.34 seconds test runtime and 21.4 seconds uncached Nx execution. All four registered dependencies executed freshly. The suite includes all eighteen base union branch files interpreted by independent Bun SQLite, edited query-visible entities, every dedicated domain's own complete nonempty model laws, exact IEEE companions, integrity_check, foreign_key_check, resource limits and cancellation.

BRep's twenty-five tables explicitly preserve every curve, parameter curve and surface variant, all analytic parameters, NURBS control points/weights/knots, topology identities and oriented relationships. Its independent suite passed three laws and 25 assertions. Full signed64 integer and unsigned64 label domains remain distinct; nextLabel is bigint in the owned model and canonical decimal TEXT in SQLite. Duplicate ownership, geometry-shape inconsistencies, dangling topology and malformed ring links are refused.

Base's handwritten document table selects exactly one of eighteen dedicated document roots through individually declared foreign keys. The eighteen branches retain their own handcrafted tables and providers. All unselected entity tables must remain empty; the selected document root is mandatory and unique. There is no fallback payload. Its individual final direct source run passed twenty laws and 82 assertions. The final combined run additionally includes explicit all-nineteen artifact-consumer and scalar diff/mutation laws.

A Mesh preallocation regression first proved that 100,000 texture octets were copied before a value budget refusal. Projection now includes the full row overhead and UTF-8 fields before octet allocation, then checkpoints before large copies. The fresh repair suite passed three laws and 26 assertions, including zero indexed source reads before both budget refusal and cancellation.

The final typed consumer audit corrected actual owned artifacts still declaring Audio as opaque entries, Image ICC octets as text and BRep nextLabel as number. Table, Graph and Flow artifacts now expose their actual persisted snapshot types. Drawing's inference, diff and mutation owners use the same exact geometry types and correct group variant. Animation time, Document run/image extents, Presentation frame dimensions and Video unsigned timestamps now share their actual snapshot scalar domains. The language-neutral base corpus records signaling NaN32/NaN64 and u64 maximum words for the explicit diff/mutation consumer law. Unused generated parser scaffolds in Table/Graph artifacts, Document artifact and Animation diff were removed after bounded repository searches confirmed no consumer; they returned untyped property bags incompatible with the declared native-mirroring contracts. No replacement generic object traversal was introduced.

The first nineteen public run reached nineteen owned suite typechecks and exposed one test literal-union comparison; it was repaired. The next run, with explicit artifact consumers, exposed an unused Document property-bag parser and an Audio parser member-list widening; both were repaired and the fresh public rerun is pending. The native all-nineteen registered gate is also pending. This report does not substitute earlier native greens or direct source greens for those final checks.

## Final Public and Zero-Touch Source Gates

The actual uncached `NX_DAEMON=false bun nx run @semio-tech/stdio-semio:test --skip-nx-cache` is GREEN: 53 built outputs, 58 verified runtime exports, nineteen owned suites, independent declaration-consumer compilation without skipLibCheck and all owned suite typechecks. It completed in 4m42s under shared parallel build load. The corpus and tests include the explicit nineteen artifact consumers and exact scalar diff/mutation consumers; all 62 laws and 517 assertions passed across the nineteen individual files.

The shared Rust artifact runner now accepts an optional positive safe-integer snapshotSqliteTestBudgetMs and Semio explicitly declares 120000. This uses the existing budgetMs mechanism without changing any other owner's default. The actual default registered source command, without SEMIO_TEST_LEVEL or a CLI workaround, completed GREEN: 62 laws, 517 assertions, nineteen files, 41.51s Bun test runtime and 47.5s uncached Nx execution including four fresh dependencies. Default launch execution is therefore zero-touch. The earlier 15s budget failure is recorded as the red evidence, followed by the bounded owner-specific repair.

The final fresh native gate remains in shared Nextest/Cargo preparation at this checkpoint. No final fresh native success is claimed until that command completes.

## Fresh Native Completion

The actual default registered `SEMIO_TEST_ARTIFACT_DIR=<ticket>/🗑️generated NX_DAEMON=false bun nx run @semio-tech/stdio-semio-rs:test-snapshot-sqlite-native --skip-nx-cache` is GREEN. Nextest ran 108 tests: 108 passed, 2,136 unrelated tests skipped by the explicit sqlite_snapshot_ filter. Test runtime was 2.436 seconds; uncached Nx execution including fresh preparation and four dependencies took 15m8s under the concurrent shared Cargo workload. This completes the fresh public/source/native proof for all nineteen Semio providers. The broader universal artifact snapshot goal remains unfinished; STEP is the next assigned family.
