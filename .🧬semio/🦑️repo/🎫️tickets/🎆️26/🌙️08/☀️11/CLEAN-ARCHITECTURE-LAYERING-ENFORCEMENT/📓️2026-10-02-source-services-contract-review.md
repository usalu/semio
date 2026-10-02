# Source Services Contract Review

Read-only inspection; no jobs executed. Root's six providers are forthcoming, so no runtime extraction success is asserted.

## Actual contract gaps

Current `normalization/🧪️tests/🏗️source-services/🟦️.ts` correctly uses TypeScript AST for direct edges and esbuild for transitive owned-input closure. Fixture lists six canonical leaves and excludes umbrella/mutation runtime capture. Three concrete limitations remain:

* Schema only closes row shape/cardinality, not roster: any six arbitrary distinct nonempty paths pass it, and dependencies may reference any roster row. Fix owner path enum plus per-owner dependency tuple/const, or close the entire ordered roster as a const. Add a same-cardinality replaced-path rejection and a changed-edge rejection; current shortened-array rejection only proves cardinality.
* Imports outside the six-owner roster disappear from the direct edge comparison. esbuild packages:"external" excludes foreign packages from metafile.inputs, so adding an external runtime import could pass both checks. Inspect metafile.outputs[*].imports external entries against owned system node: modules; retain legitimate first-party discovery/serialization imports as captured owned dependencies. Check dynamic imports/requires through the same external output facts. Type-only third-party imports require separately bounded public-contract policy if introduced.
* Rejecting ExportDeclaration catches reexports, but not an exported wrapper body calling a forbidden facade or nonrelative package alias. Transitive bundling catches statically resolved relative wrappers, but an external alias remains invisible under packages external. Close external import sources and enforce actual callable exports/owned implementation declarations rather than calling every non-reexport an implementation.

Fixture root is normalization, workspace resolve ascends seven directories and matches current repo layout. Denied prefix test uses startsWith("🟦️.ts"), technically broader than exact umbrella path; use equality for exact leaf and slash-delimited prefixes for trees to keep domain boundaries explicit.

## Registration state at inspection

Package 📜️script.ts623–627 now explicitly runs source-services, source-admission and source-admission-io together, fixing the prior runtime inclusion gap. Searches found no source-services target/package-script/launch registration yet; those are pending concrete registration obligations, not passing route evidence. Default runRepositoryTestCommand budget is retained. Add targeted owner source inputs for all six leaves and fixtures/schema; existing tsconfig recursive ../../**/*.ts already covers types.

## Closed fresh-session/cache fixture matrix

Extend the existing taxonomy-pattern-compiler-reuse integration contract rather than creating a permissive cached snapshot callback. Proposed language-neutral rows each declare ordered actions and expected authority/output:

1. same-bytes-two-loads: reads2, distinct input receipts and session views; same content-derived facts. Every load performs current physical admission. Define matcher session sharing/isolation explicitly; current original law requires distinct matcher.
2. mutate-returned-schema: modify one returned fixedFilenameContracts reason; subsequent same-byte load and prior independent session remain authored-byte values. Either frozen mutation refuses or detached session view isolates. Do not expose shared mutable parse facts.
3. same-path-changed-bytes: fresh receipt hash changes and reason reflects current bytes; old session remains unchanged. Same hash from equal bytes at a second physical path must attach that path's current receipt.
4. valid→invalid→valid: invalid current schema throws, then valid bytes return original facts; failure never inserts partial cache entry or evicts authority into an accepted partial result.
5. syntax/lossy→valid: failures preserve correct parse/validation boundary and successful later output. Cache keys derive only from captured lossless bytes, never supplied filename/stat counters.
6. valid→physical deletion/link/root-link/linked ancestor: subsequent load refuses even with cached content; known hash is not a read receipt. Use actual owned input physical oracle paths, including raw missing-prefix/.. when the input contract rejects it.
7. opaque path/root: refuse before snapshot capture; original compose assertion remains. Root ancestry must be checked by the physical owner, not inferred from taxonomy content cache.

Original matcher law181–244 already covers changed/invalid/lossy/syntax/opaque and exact counters. Its [2reads,2parses,2validations], distinct schema and matcher and per-session compilation counts are actual assertions currently contradicted by cache. Fresh physical reads/failure behavior are authority; parser count and duplicate pattern compilation are implementation. Preserve original failure receipt, handcraft explicit revised fixture counts for pure parse-facts caching, and add isolation rows before changing that implementation expectation. No silent deletion of the failed law.

Canonical split: immutable content-derived parsed facts cache inside taxonomy owner; fresh load session containing current physical input/path, detached or frozen schema view, explicitly owned matcher session. Inputs are never cached. IO source admission directly calls actual options-aware loader; no facade callback or alias adapter. No new external runtime dependency is needed.
