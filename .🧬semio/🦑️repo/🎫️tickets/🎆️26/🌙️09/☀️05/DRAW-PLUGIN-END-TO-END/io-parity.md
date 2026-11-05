# Drawing IO Declaration Parity

The TypeScript declaration now uses the shared first-party IoEntryDescriptor and nominal parseDialectCoordinate contract. Its eight entries follow native registration order exactly: Drawing → SVG/PDF/PNG/JSON, JSON → Drawing, SVG → Drawing, Drawing → DWG/DXF. SVG import is Semantic; JSON import is Exact. Both imports declare sniffs true because deserializer_entry installs a sniff function pointer, including the default Confidence::None implementation. The field describes callable registration, not detection confidence.

Removed the four nonexistent PDF/PNG/DWG/DXF imports and stale stub/type-availability comments. No runtime third-party dependencies were added. serde_json is a test-only dependency already available in the workspace.

## Contracts and Tests

The neutral descriptor fixture uses the existing shared IO and artifact-reference schemas. It records eight expected descriptors, six invalid wire values, four unsupported import pairs, and two valid-schema stale metadata mutations. The TypeScript regression compares the whole ordered declaration, checks every expected pair occurs exactly once, rejects unsupported pairs, checks each wire value with both the owned parser and independent Ajv, and rejects stale metadata parity. Expected check count: 45.

The native root test io_descriptors_match_neutral_fixture reads the same neutral JSON and compares metadata from actual io().entries, plus rejects every unsupported pair. This catches registration drift without relying on source-text extraction.

The declared draw-js:test selector io-descriptors runs parity and strict TypeScript checking; the full declared suite runs both too.

## Validation

- Before the production change, the tiny direct owned parity test run through repo tools exec --light bun failed with the expected full-table mismatch (12 primitive-string entries versus eight structured entries, plus stale fidelity/sniff metadata). This was a diagnostic run, not declared route validation.
- Declared repo run @semio-tech/draw-js:test -- io-descriptors is queued under session 67782 behind host memory capacity; no declared green result is claimed yet.
- Declared native owner validation repo run @semio-tech/draw-drawing-rs:test --lang=en -- --lib io_descriptors_match_neutral_fixture is queued under session 62542. At the latest check, focused JS waited over 10 minutes and native over 7 minutes behind critical host memory capacity; neither has executed, so neither has a green result.
- All six changed/new source paths have absolute UTF-16 lengths at most 147, below 256.

## Files

- ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🟦️.ts
- ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧪️tests/🔬️descriptor-parity/🟦️.ts
- ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧪️tests/🧫️fixtures/📇️descriptor-parity.json
- ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🦀️.rs
- ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/Cargo.toml
- ✏️s/🔌️plugins/🖍️draw/📦️packages/🟦️typescript/📜️script.ts

## Cache Input Coverage

The repository task model merges named input groups using last-write precedence (🏗️build/🧮️task-model/🟦️.ts groups(), and 🕸️graph/🚪️admission/🟦️.ts buildSourceContractGroupsV1). The project-specific default therefore replaced the workspace default instead of extending it. Draw previously listed only framework 2d/ui inputs. Added explicit projectRoot files, sharedGlobals, and Drawing plugin source/fixture/assets extensions including Rust and TOML publication-authority owners. Existing browser-acceptance target is preserved. This ensures declaration, fixture, script, and native source edits invalidate cached Draw tests.

Additional changed file: ✏️s/🔌️plugins/🖍️draw/📦️packages/🟦️typescript/📋️project.json.

Cache coverage excludes plugin dist, .🧾️store, target, .cache, node_modules, and 🗑️generated trees using explicit repository negative-glob conventions; the source glob omits wasm. Neutral authored binary fixtures remain covered by fixture/example globs.

## Current API Review

Rechecked current definitions while execution remains capacity-blocked: IoDeclaration.entries is the public static IoEntry slice; Dialect is Copy with artifact_kind and public StandardId/SubsetId tuple fields; SubsetId::ANY is exactly *; IoFidelity derives Debug with the four expected wire variant spellings; IoEntry.sniff is a public optional function pointer. The native comparison uses these current fields without removed constructors. The public framework/io/schema SDK exports schemaSchema as ioSchema and reexports IoEntryDescriptor, parseDialectCoordinate, and parseIoEntryDescriptor from the owned schema module. parseIoEntryDescriptor admits unknown wire input and returns the expected structured descriptors. Existing typecheckPathRaster takes repoRoot, a module directory, and optional owned source paths exactly as called. No grounded API repair was found.

## Native Retry

Initial native focused session 62542 exited 1 before test execution during host-capacity admission. The error was filesystem EBUSY (errno -4082), names() at gate line 74, readGrants line 182, withHostCapacityV1 line 284. No IO test compilation or assertion executed. Retried the same declared native focused command as session 36952; again waiting for critical host memory admission. This is an infrastructure failure, not a passing or failing IO contract result.

## Admission Impasse and Resume Commands

On parent request after the third consecutive shared-host capacity blocker, canceled only this agents queued handles with Ctrl+C: TS session 67782 after 1650 seconds at door.capacity, and native retry 36952 after 241 seconds at door.capacity. Both exited 1 from cancellation before any owned test execution. Host capacity still reported critical memory (approximately -2.3 GB above the 4096 MB floor), six running peers, and four/nine queued requests ahead respectively. No peer process was canceled. No declared test green or source compilation failure has been observed. No independent IO source repair remains known from current API review.

Resume sequentially when host capacity is available:

```powershell
repo run @semio-tech/draw-js:test --lang=en -- io-descriptors
repo run @semio-tech/draw-drawing-rs:test --lang=en -- --lib io_descriptors_match_neutral_fixture
repo run @semio-tech/draw-drawing-rs:test --lang=en
repo run @semio-tech/draw-drawing-rs:check --lang=en
```

The root agents full declared draw-js:test validation is also required independently; its handle is outside this agents ownership. Keep ticket open until declared end-to-end checks finish.

## Resumed Native Preparation and Coordinated Release

Resumed focused declared native command as session39661. It passed host capacity admission (8 jobs,16607MiB reserved), waited for the shared dependency builder, then became the active dependency owner. It reached actual current immutable input capture/replay; no Rust compiler or test body ran. At approximately3200seconds route total (3080seconds repository command), parent requested deliberate cancellation of only this agents handle because it was executing the confirmed old defective capture implementation while reserving eight CPU jobs needed by the queued repair regression44842. Ctrl+C exited1 before any owned test body. No peer process was touched. This resource release is coordinated for the grounded bootstrap repair, not an arbitrary timeout restart.

After bootstrap capture/replay repair validates, restart focused named native IO test and require a positive executed test count; then full declared Drawing native test and check remain required. Neither focused nor full native green is claimed. Read-only supporting investigation is retained in capture-audit.md.

## Preliminary Cargo Ingress Audit

The first-party repo cargo launcher is a valid admitted compiler ingress: it requires the issued Original, compiler runtime entries, repository compiler store, configuration/profile injection, build leases, exact requested parallelism and host/compiler capacity. The --parallelism option is supported before the Cargo separator. It does not itself refresh graph-generated owner modules. A potential low-parallel preliminary check would target the actual artifact crate manifest with --lib --tests --locked; declared graph-generate/test/check routes would still remain mandatory.

Current generated graph inputs are demonstrably stale, so no preliminary Cargo check was started and the existing native13533 handle was retained. Drawing generated registry still imports semio_framework_graph::manifest::Manifest, while the current renderGraphArtifacts registry emitter writes semio_framework_graph_manifest::Manifest and the actual artifact crate directly depends on semio-framework-graph-manifest. Thus compiling the current generated registry would conflate stale generated imports with handwritten Draw errors. The current emitter also emits owning enum wire tests not guaranteed present in the old file. Drawing check/test dependOn graph-generate, which uses the owner outputs.json catalog, admits current manifest documents, renders exact bytes and publishes outputs. An explicit generator freshness admission must precede a meaningful independent compile. No raw Cargo, duplicate compilation, missing Original workaround, or stale-generated preliminary proof was run.

## Full Native Coverage and Level Audit

The current declared test router selects11 explicit packages: root artifact plus10 companion test crates. The artifact Cargo workspace has37 members. Dependency libraries compile without cfg(test) unless selected as test packages; selecting their companion does not run their own embedded cfg(test) modules. Concrete omissions include semio-s-artifact-draw-drawing-viewer (viewer and canvas unit laws mounted directly under cfg(test)), semio-s-artifact-draw-drawing-geometry (bounds/raster/framing/picking/selection/editing/handles/arrangement laws), and the read/write/io libraries (SVG import primitives, PDF writer and image-admission tests). These tests are not remounted by the selected companion roots inspected. The full native command as currently declared is therefore broader than the focused IO law but does not cover all Drawing workspace semantics.

No Draw-specific long/quick/exhaustive compile-time cfg gate was found. The10 companion roots use cfg(test); own embedded unit modules use cfg(test), and testkit gates supply test-facing schema support. External feature-sut oracle adapters are separate excluded oracle/bridge crates, not tests enabled by selecting a longer level. A longer level cannot repair missing package selection.

The first-party resolveTestLevel consumes only a recognized FIRST forwarded selector: fundamental, quick, long, exhaustive. Without a caller environment it defaults to fundamental (not quick). repositoryCargoTestPolicyV1 selects that Nextest profile. cargoTestPlanV1 excludes higher-level namespace prefixes via --skip and chooses per-level assertion budgets; it does not change the selected package set or add a cfg flag. The inspected Drawing tests do not use higher-level namespace modules, so changing level primarily changes their permitted assertion budget here. long is appropriate for a complete native gate without exhaustive coverage overhead; exhaustive also auto-enables SEMIO_COVERAGE.

A supported full-level invocation is repo run @semio-tech/draw-drawing-rs:test --lang=en --parallel=2 -- long (FIRST forwarded argument, no guessed trailing Cargo test filter). This still only selects the current11 until the owned package catalog is repaired. validateNativeCargoArguments explicitly refuses --workspace, --package/-p, --manifest-path and --exclude overrides through this target; do not bypass the catalog via a trailing flag. A dedicated declared complete target/catalog should select all37 Drawing workspace member packages before claiming full semantic native proof. The focused current handle13533 is unchanged and remains valid for the exact named IO law. No extra queue or source edit was made during this coverage audit.

## Native Catalog Repair Red Baseline

Refactored the exact existing selection into the new owned native catalog without changing behavior:11 test packages and root-only check. Neutral coverage fixture expects all37 real Cargo workspace members and tests omissions, duplicates, unknown members, wrong manifests and separately disabled test/check selections. Independent Ajv validates the schema, the first-party schema validator must agree, Bun TOML reads actual current member package identities, and SQLite performs bidirectional complete-set comparison. The positive assertion compares the actual catalog/options with that current authoritative workspace membership, so the red proof must be actual11-vs37 coverage rather than missing imports.

The initial ordinary source-test queue86769 was replaced once by the documented normal one-core admission wrapper (not duplicated): repo capacity run --memory=2048 --cpu=1 -- repo tools exec bun -- test <owned native catalog test>. Current sole source proof handle41313 is waiting for capacity; no test execution/red/green yet. Valid focused native handle13533 remains retained. Reviewed resolver and Nextest partitioner confirm --lib remains a build selector and io_descriptors_match_neutral_fixture an assertion filter; omission of an explicit first level does not consume either, so that focused invocation is valid and was not canceled. No full37 catalog behavior is enabled yet pending this meaningful red result.

## Catalog Regression Resource Release

Revalidated native13533 at roughly8244s route total/7970s shared dependency wait: still no graph generation, Rust compiler or test body. Parent explicitly requested freeing only its idle2CPU reservation to fund the concrete normal-capacity catalog regression. Preserved raw output in generated/native-13533.log and canceled that own handle; terminal exit1 is cancellation, not test evidence. Complete fresh37-member test/check remain mandatory. The full run must show actual io_descriptors_match_neutral_fixture with a positive executed count; no narrowed substitute is accepted.

## Complete Native Catalog Repair

The owned catalog now selects all37 current Drawing Cargo workspace members for both check and test. Existing issued Cargo provider and package/feature override validation remain unchanged. A declared native-coverage prerequisite on test/check runs the neutral schema/cases with independent Bun TOML parsing of actual Cargo membership, Ajv schema validation and SQLite exact set comparison, followed by strict TypeScript noEmit. Graph generation remains mandatory.

Meaningful red executed against the behavior-preserving interim catalog: one test failed at the actual old11/root-only-check versus37 membership assertion. Two preceding setup failures (path selection and relative imports) are not counted as red. After completing all37 catalog rows, the same normally admitted1CPU2048MB source regression executed one test, one pass, zero failures. Strict TypeScript noEmit for the production catalog and regression completed exit0. No light test route was used. These source proofs do not establish native compilation or named IO-law execution.

Fresh complete native validation starts with repo run @semio-tech/draw-drawing-rs:test --lang=en --parallel=2 -- long. Its actual named io_descriptors_match_neutral_fixture execution must be nonzero. The separate declared check remains required after full test, and generated graph registry must reflect the current graph-manifest dependency before native proof.

The fresh complete-long route is handle43703, normally admitted2CPU5233MB. Actual parser-source listing/state/frontier inspection completed and the route reached build dependencies; it now shares an existing producer without starting another. At route651s/shared380s there is no refusal, native-coverage prerequisite result, graph-generation result, Rust compilation or native assertion execution yet. Raw output is retained under generated/native-43703.log; this remains a pending full37 validation, not green proof. Independent parent review of catalog/provider selection is retained in native-catalog-review.md. All six new catalog/schema/fixture/test files have absolute paths <=108 UTF-16 characters.

## Fresh Full Native Route Resource Release

Revalidated actual handle43703 at route2572s/shared2312s: it remained explicitly waiting for the existing dependency build. No native-coverage prerequisite body, graph-generation body, Rust compiler or native test body had begun. Parent requested releasing only this idle2CPU5233MB reservation to admit concrete normally queued bootstrap resume repair regression25598. Preserved raw output in generated/native-43703.log, canceled only own43703, and observed terminal exit1. This is resource coordination for a confirmed preparation repair, not a timeout restart or a native failure/green proof. No peer processes were touched.

No own live native handles remain. Hold fresh scheduling until shared preparation is actually repaired and ready. Complete37-member long native test with nonzero named io_descriptors_match_neutral_fixture remains mandatory, followed by the separate declared check. Resume commands, after prerequisite readiness: repo run @semio-tech/draw-drawing-rs:test --lang=en --parallel=2 -- long; then repo run @semio-tech/draw-drawing-rs:check --lang=en --parallel=2. Retain actual native-coverage prerequisite and fresh graph-manifest registry generation evidence as well. The already executed meaningful catalog red, one-test source green, and strict TypeScript green remain valid source proofs only.

## Native Ownership Resumed Behind Preparation Readiness

Native ownership resumed with no live own process handles and no new queue. Parent reports old implementation handle76442 remains live in another publication walk after retries; final sealed-resume repair green is being recovered after an execution worker requirement-loader error. This is coordination status, not new native or preparation proof. Hold the existing explicit long37-member test/check commands until repaired shared preparation is actually ready. Do not repeat the unchanged catalog source tests, use a bypass, or count canceled43703 as proof. After readiness, require actual native-coverage prerequisite evidence, current graph-manifest registry generation, complete native assertions with a nonzero named io_descriptors_match_neutral_fixture execution, then separate declared check.

## Final Native and IO Source Revalidation While Preparation Repairs

Read-only revalidation found no new handwritten IO/catalog defect. The native root still registers the exact8 declared serializers/deserializers in the neutral fixture order; the shared nominal TypeScript table and both parity tests still match that registration, including4 absent import pairs and real fidelity/sniff metadata. Native fixture inclusion is relative to the actual root lib file, whose Cargo lib path points ../../🦀️.rs. The artifact-local lock entry contains serde_json, matching the existing root dev dependency. Current catalog JSON still has37 rows with37 check and37 test selections; the issued package script spreads those options, and both declared gates retain graph-generate/native-coverage prerequisites. No source tests were repeated.

The already known generation blocker remains observable: current generated registry imports semio_framework_graph::manifest::Manifest while the authoritative current emitter uses semio_framework_graph_manifest::Manifest. Do not manually alter this generated file or compile it as fresh proof. Actual owning graph generation must update it after repaired shared preparation becomes ready. Shared preparation readiness remains the scheduling blocker; no new native queue was created.

Concrete full gate remains repo run @semio-tech/draw-drawing-rs:test --lang=en --parallel=2 -- long, followed only after full test green by repo run @semio-tech/draw-drawing-rs:check --lang=en --parallel=2. Capture actual prerequisite success, graph registry freshness, selected member coverage and executed named IO law; a zero-count assertion result is insufficient.

## Fresh Native Gate After Qualified Generic Repair

Parent reports final25-root strict12627 terminal0 and all13 portable modes green, with qualified Generic source frozen. Started sole fresh declared full37 long Drawing native route: repo run @semio-tech/draw-drawing-rs:test --lang=en --parallel=2 -- long. Handle22389 was normally admitted with1 of requested2 CPU because current host cannot carry more, and entered actual source/implementation admission. Preserve full raw output under generated/native-22389.log. There is no focused or duplicate native queue. Require actual native-coverage and current graph generation, all37 selection and positive named io_descriptors_match_neutral_fixture execution; separate declared check follows only full test green. Prior cancellations remain non-proof.
