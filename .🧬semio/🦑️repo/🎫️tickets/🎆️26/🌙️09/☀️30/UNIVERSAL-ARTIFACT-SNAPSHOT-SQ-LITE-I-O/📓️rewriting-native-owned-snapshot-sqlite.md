# Rewriting Owned Snapshot SQLite Staging

The actual Rewriting snapshot owns five persisted fields: `before_fixture_json`, `lhs_json`, `rhs_json`, `parameter_bindings`, and `rule_layout`. This report stages a handcrafted relational contract and meaningful Native baselines. The production SQLite provider, capability hook, controlled input/output hooks, and JSON exact-word repair remain unmounted until the main coordinator obtains genuine runtime absence or fidelity failures. No Cargo invocation was started by this worker.

## Actual ownership

Artifact root: `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting`. All snapshot paths below are relative to `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot`.

The three body fields are authored strings. Their names and the RHS language annotation do not convert their persisted type into a parsed JSON tree. Invalid JSON-looking content, exact whitespace, NUL, reserved delimiters, and Unicode must survive. SQL stores each as its named TEXT column.

`parameter_bindings` is an actual Native `BTreeMap<String, semio_framework_graph::manifest::PropertyValue>`. Its six variants are Null, Bool, Number(f64), String, Array, and Object(BTreeMap). This domain is narrower than the intrinsic nine-variant Value owner; there is no UInt, Int, or Bytes variant to invent here. `rule_layout` is an actual BTreeMap of `LayoutPoint { x: f64, y: f64 }`. Neither f64 owner imposes a finite-only constructor restriction. No child handle is persisted in this parent. The editor's materialized working graphs are separate local ownership.

The native root derives its typed DslRecord. Its handwritten ordinary Pack/Text codec uses the actual root record. Recursive graph PropertyValue controlled metadata/construction/output may refuse until its concrete owner is implemented; the staged controlled-record law measures the real seam. The current declared JSON leaves convert ordinary ToValue output to JSON and therefore cannot be assumed to preserve non-finite states or payload bits.

## Source audit

The actual Source Snapshot delegates to `parseRewritingArtifact` in its owning schema. That schema aliases framework JSON-projection `DslValue` as PropertyValue and validates the complete input through `parseDslValue`; its layout fields are `number` and explicitly finite. It consequently cannot represent every legal Native numeric word. This is a source inspection finding, not a measured Source feature RED. The TypeScript executor owns the eventual canonical Source repair after Jack. No parallel DTO, numeric union, source compatibility branch, or inferred provider was introduced here.

## Handwritten relational contract

`🪶️sqlite/🗄️.sql` contains nine independently authored tables. `rewriting_document` stores the three strings. `rewriting_binding` associates each literal binding key with a typed `rewriting_value`. Separate boolean, number, and string tables hold scalar values; ordered array-element and object-member tables retain relationships. `rewriting_layout` stores each literal map key and both scalar coordinates.

Every f64 scalar has a named nullable REAL query value, signed INTEGER exact raw word, and numeric class. The raw word uses exact reinterpretation through signed i64 storage; it does not cast the float numerically. Finite/infinity query values remain readable; NaN query values are NULL. Companion validation must distinguish positive/negative infinity, finite, and NaN, preserving signed zero, subnormal values, maximum finite, quiet NaN and signaling NaN payloads.

Structural IDs are positive SQL surrogates. Literal keys are independent strings, including empty and NUL-containing identities. Eventual providers must verify BTreeMap key uniqueness/order, array/object ordinals, variant cardinality, exclusive ownership, cycles and orphan rows. No semantic identity inference or global reference registry is part of the SQL contract.

## Neutral corpus and baselines

`🧫️fixtures/🪶️sqlite/🔣️.json` contains a full five-field case, all six property variants, nested/empty arrays and objects, both booleans, independent literal keys, all three body strings and nine binary64 word vectors. The full case has 28 rows across all nine tables. The fixture explicitly declares table widths and row counts.

Eight snapshot Native laws are mounted in `🧪️tests/🪶️sqlite/🦀️.rs`: actual bare capability; complete ordinary Binary and Text roundtrips; genuine cumulative controlled record construction; declared JSON closed-word transport; independent Bun SQLite schema/width/word/string checks; real interior cancellation while constructing a 128KiB authored string; and actual erased Binary/Text relational I/O.

A ninth declaration law is mounted as a descendant of the actual private Any component, at `🧪️tests/🪶️sqlite/🚪️io/🦀️.rs`, and calls its real `io_declaration()`. It does not guess a flattened module API. Native fixture guards use the canonical owned retirement factory and run the explicit cold cursor to terminal emptiness. Rustfmt parsed both test files; compilation and runtime results remain pending in the coordinator's sole Native lane.

`🧪️tests/🪶️sqlite/📋️contract/🟦️.ts` independently inserts the complete neutral case using Bun SQLite, checks every table width and literal row count, verifies all variant and relationship collections, and serializes/deserializes the database for integrity/FK checks. Its nine word laws check every number and layout scalar against exact signed raw-word storage plus query/class values. This is the third-party SQL contract oracle, not a claim that the unmounted Native provider exists.

## Executed independent contract result

The actual registered uncached Source-contract command completed successfully: **10 tests passed, zero failed, 204 assertions, 20ms Bun test time, 9.9s Nx duration**. It checked the entire 28-row neutral case, every one of the nine authored table widths/counts, all six PropertyValue variants, literal strings/keys, both booleans, nine exact raw-word vectors, independent serialize/deserialize integrity and foreign keys. Log: `🗑️generated/rewriting-sql-contract-first.log`.

This proves the handcrafted SQL is independently interpretable and that the neutral scalar/relationship contract is executable. It does not establish Rewriting Source projection or Native SQLite capability. All nine Native baselines remain unexecuted and production remains unmounted pending the coordinator's sole lane.

## Executable registration

Owning package: `📦️packages/🦀️rust/{📜️script.ts,📋️project.json}`. Existing graph and document/map/window verification commands are retained. The canonical shared Rust artifact runner now receives the explicit SQLite contract suite and `component-app-assembly` feature necessary for the actual declared I/O test.

Registered targets are `@semio-tech/trinity-rewriting-rs:test-snapshot-sqlite`, `test-snapshot-sqlite-native`, and `test-snapshot-sqlite-source`. The source target currently answers only the independent SQL contract. Source production and public consumer tests will be appended by their owner after authentic runtime RED. Both launch catalogs contain the three commands at orders 408.737–408.739, with existing quick level and no deadline changes.

Native baseline to queue: `SEMIO_TEST_LEVEL=quick bun nx run @semio-tech/trinity-rewriting-rs:test-snapshot-sqlite-native --skip-nx-cache`. It is not included in the main coordinator's already-running sixteen-owner batch. No universal completion claim is made.

## Files authored or updated

- Snapshot `🪶️sqlite/🗄️.sql` and `🧫️fixtures/🪶️sqlite/🔣️.json`.
- Snapshot Native and independent Source-contract test files, plus the descendant declared-I/O law.
- Snapshot and Any Native roots: cfg(test) module mounts only.
- Rust package script and project registration.
- Both current and seed launch catalogs: narrow Rewriting entries only.
- This ticket report; generated registered-run log is under the ticket generated directory.

## Executed Generator Prerequisite Repair

The current parent batch stopped before Native compilation in Rewriting's `graph-generate`: its owning `🛂️manifest/📇️outputs.json` omitted the mandatory output-policy record. The canonical parser requires explicit `excludedInputPaths` and `genericEmojiIdentities`; no default policy is inferred. Added both as empty arrays, preserving the exact Rewriting input area and all authored output identities. No shared parser relaxation was made.

Fresh registered `@semio-tech/trinity-rewriting-rs:graph-generate --skip-nx-cache` completed successfully, exit zero, **4.7 seconds Nx**, and wrote its one admitted `rewrite-lhs` manifest through the existing owning generator. Log: `🗑️generated/rewriting-graph-explicit-policy-current.log`. This repairs the measured prerequisite and establishes generator execution only. Rewriting Native capability, typed fidelity and controlled I/O laws still require the sole coordinator's runtime baseline; the Native semantic provider remains unmounted.

## Unmounted Native Projection Draft

The adjacent `📸️snapshot/🪶️sqlite/🦀️.rs` now contains an individually handwritten Native projection draft for the nine authored tables. It retains all three literal authored body Strings directly, the full layout map, and each binding's actual six-variant Graph PropertyValue tree. Primitive bodies, ordered array edges and ordered object members are separate named entities; each numeric value and both layout coordinates use the existing REAL/null, signed exact-word and classification companions. No body String is parsed or substituted, and no property tree is carried as serialized JSON. Native BTreeMap order governs literal map keys.

The checked borrowed forecast admits row/schema/table/column limits; explicit iterative traversal avoids recursive projection. Source is syntax-inspected only. No module mount, capability, Native codec repair or compiled/runtime provider claim is made. True controlled Native transport remains pending the sole coordinator's authentic nine-law baseline.

The same unmounted source now includes iterative, field-by-field reconstruction. It indexes the nine declared tables, validates exact positive primary keys/widths, consumes each row once, checks contiguous array/object ordinals and canonical unique map order, and rejects cycles, shared descendants, orphans, alternative primitive bodies, invalid booleans and inconsistent IEEE queries. Completed descendants and partially built containers are guarded by the canonical Graph PropertyValue `RetireOwned` cursor, including cancellation while assembling deep trees. Long literal text uses bounded first-party Reconstruction copies; scans and frontier construction publish interior cancellation checkpoints. The completed Snapshot uses its already-authored actual retained retirement authority. Rustfmt parsed this draft only; no Native compilation or execution is claimed.
