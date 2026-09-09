# Block and GIS Composition

## GIS

The GIS plugin manifest now mounts its taxonomy-root composition source. The package-local Rust implementation mount tree was removed. The plugin and native-codec receipt registry refer directly to the map and terrain artifact crates, each with its app assembly feature enabled. The native stdio catalog remains an explicit plugin-level contribution because it assembles the complete codec catalog. Unused artifact-specific runtime dependencies were removed from this composition manifest.

The existing language-neutral artifact identity fixture, independent serde_json oracle, native-codec receipt tests, and component cold-map-patch integration test remain owned by the plugin integration boundary. Artifact examples and document tests are being mounted by the artifact execution owner in each leaf.

Validation pending: Cargo resolution, GIS component compile, fixture-based plugin assembly runtime test.

## Block

The Block plugin now mounts its taxonomy-root app enum, registration and surface tests. Its three artifact dependencies activate their app assembly features; the plugin invokes each generic artifact declaration with the parent `BlockApps` enum. Shared kind, metadata, author, representation and camera records are canonically owned by the block-2d artifact. The package-local monolithic Rust mount tree was removed. The optional entry symbol is default-enabled through `plugin-entry`. Component compiler and runtime surface checks remain pending.

## Files

- `✏️s/🔌️plugins/🌍️gis/🦀️.rs`
- `✏️s/🔌️plugins/🌍️gis/📇️native-codecs/🦀️.rs`
- `✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/Cargo.toml`
- Removed `✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/🦀️.rs`

Additional Block files: `✏️s/🔌️plugins/🧱️block/🦀️.rs`, `✏️s/🔌️plugins/🧱️block/📦️packages/🦀️rust/Cargo.toml`, removed `✏️s/🔌️plugins/🧱️block/📦️packages/🦀️rust/🦀️.rs`.

## GIS Leaf Integration Corrections

The component compiler reached Block and GIS after compiling the selected full stdio catalog, then found five Block schema-alias errors and fourteen GIS source-boundary errors. The final dependency spelling is `semio-framework-schema`, matching the derive macro’s emitted `::semio_framework_schema::` paths. Handwritten framework references use that canonical absolute crate path; `crate::schema` remains the artifact domain module. No duplicate external-crate alias item is declared.

The coordinator repaired both GIS leaf duplicate snapshot exports, three mechanically corrupted Semio base-geometry import paths, and the map artifact's missing Surface dependency. Surface is optional for map app assembly and uses its prior no-session-bindgen selection in both GIS leaves, preventing duplicate component session exports. Unused direct framework geometry dependencies were removed. GIS check 3 isolated the canonical derive dependency spelling, which was repaired in both leaf manifests and sixteen mounted source files. Check 4 failed in shared PNG/glTF mutation derives while reading an empty authority JSON file; the current project and taxonomy documents both parse successfully. The current combined Block/GIS retry is `block-gis-composition-check-5.txt`; its result is pending.

Additional files are both GIS artifact roots and manifests plus gismap `🏅️standards/🔖️1/🪆️subsets/✳️any/{🧬️schema,🚪️io}/🦀️.rs`.

## Example Ownership

Removing the old GIS parent mount tree must preserve its two editor demo-session implementations. Both now have canonical mounts in the corresponding artifact editor examples module, with their existing tests. The two existing test asset includes were corrected to reach the adjacent example asset directory. The plugin no longer mounts those artifact implementations independently. Additional files are the gismap and gisterrain `✏️editor/📚️examples/🎬️demo-session/🧪️tests/🦀️.rs` files, plus both artifact roots.

The current composition check also restored the `protocol`/`store` aliases required by the closed-app dispatch macro. Terrain selects its editor framework dependency only through component app assembly. Map's retained initializer now names the canonical kernel fault types, allowing its framework dependency to be optional and selected only for its editor/viewer feature. Existing schema use of Surface terrain tiles remains a real default dependency.

## Persistent GIS Law Routing

The two GIS native law groups now name the map artifact directly; the editor law explicitly enables component-app-assembly. Their exact test names include the taxonomy mounting component module. The map group oracle now inspects its extracted test source for neutral membership coverage, and the durable assembly oracle checks the current owned optional-sink expression. The map group oracle passed 26 checks; the durable assembly oracle passed AJV plus Node/WebCrypto hash agreement, three roles, four cancellation cases and three rejection cases. Native law execution is still pending the old Cargo queue.

Additional coordinator-owned test routing and fixture-reference files:

- `✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/📜️script.ts`
- `✏️s/🔌️plugins/🌍️gis/🧪️tests/🌉️component-cold-map-patch/🦀️.rs`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🗺️mutate-gismap-1/🦀️.rs`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🦀️.rs`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🏔️mutate-gisterrain-1/🦀️.rs`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🦀️.rs`
## Current Native Acceptance Sequence

The previous GIS default runtime attempt reached the extracted map unit tests and failed because a default-build test called the component-only declaration API. Its metadata and neutral inference checks remain enabled by default; the declaration assertion now has its matching component-app-assembly gate. Retry 8 (`gis-default-runtime-8-jobs2.txt`, session 43362) is pending.

After this combined default runtime gate, each GIS leaf still requires an independent default check so Cargo feature unification cannot mask a missing default dependency. Both leaves then require their component-app-assembly tests, followed by the Block and GIS parent library tests and GIS native-codec integration. These checks use the ticket Cargo target with two build jobs and incremental compilation disabled. The component cold-map-patch suite additionally requires built WASM/receipt assets; its include paths have been audited but its runtime has not been claimed.

The fresh full host integration graph is running in session 64108 (`framework-host-integration-7-current-jobs2.txt`). Forms completed its ordinary Nx check and four prerequisites before this host command acquired the shared Cargo lock. Native results above remain historical until their listed follow-up gates finish.

## Block Test Router

The Block parent test router discarded all arguments and did not await the common asynchronous Cargo test helper. It now follows the existing GIS router: resolve the test level, forward remaining Cargo/libtest arguments, and await the helper. This preserves explicit `--lib` selection for the parent integration gate and lets the router observe its terminal status. Runtime confirmation remains part of the pending ordinary Nx parent test.

Additional file: `✏️s/🔌️plugins/🧱️block/📦️packages/🦀️rust/📜️script.ts`.

Bun parsed the corrected Block router successfully (exit 0, `block-parent-router-parse-current.txt`). This is syntax validation only; native execution and forwarded library selection are still pending. The artifact executor made the corresponding scoped correction in its five composed parent routers.

The fresh full-host session64108 subsequently passed with exit0; see `📓️integration-review.md`. GIS and Block runtime/default/component acceptance remains separately pending.

## Independent Default Nx Retry

GIS retry8 ended with exit101 in the shared plugin prerequisite: the pre-worker cancelled `MountedTypedCommandFullOperation` initializer lacked `window_transient_authority`. The artifact executor added the semantically empty `None` field. This failure occurred before GIS assertions and does not count as a test result.

Retry9 is now session38894, `gis-independent-default-nx-9.txt`: ordinary Nx `run-many --targets=check,test --projects=@semio-tech/gis-gismap-rs,@semio-tech/gis-gisterrain-rs --parallel=1 -- --no-default-features --lib`. Each leaf gets a separate normal check and a separate runtime test invocation. This replaces the combined default retry and explicitly covers the previously pending independence check, preventing Cargo feature unification or test-only dependencies from masking a default library defect. No app-assembly feature is selected here; the component and parent gates still follow.

Retry9 map check failed on incomplete shared WindowConfig fields, map test failed on the subsequently remaining shared future Sync bounds, and terrain check exited 101 because the lock file changed during manifest repairs. Terrain test remains active against subsequently repaired source. These failures precede GIS assertions; its independent failed check/test targets require focused retries after the current operation terminates.

Retry9 ended exit1, with all four leaf check/test targets failing before GIS assertions. Retry10 is session32710 in `gis-independent-default-nx-10.txt`, using the same independent four-target sequence with a fresh private Nx workspace directory. Its compilation now follows the current shared WindowConfig field and loader-return repairs.

## Block Parent Runtime

Session5941 runs the ordinary `bun x nx run @semio-tech/block-plugin:test --output-style=stream -- --lib` gate with two Cargo jobs, incremental compilation disabled and the shared ticket target. All four declared generator prerequisites completed, and the parent library test target is active. Receipt: `block-parent-native-nx-1.txt`. The explicitly forwarded `--lib` reaches the parent router. This runs independently of GIS default acceptance and does not replace the individual Block leaf checks.

Block parent session5941 failed before assertions on shared window configuration pack imports. Those imports subsequently appeared in current source without coordinator changes. Focused retry36694 runs only Collection and Block parent; Block’s required generator prerequisites remain included. GIS session32710 remains active after its first Map check failed on the earlier missing shared channel encode arms.

### GIS Native SVG Export Regression

The tenth independent Nx matrix compiled Map and discovered 150 tests. Nextest stopped after 13 tests: 11 passed and two existing SVG export tests failed because the drawing bridge snapshot was printed as artifact DSL. The production renderer now calls the standalone SVG snapshot’s native `export_utf8` codec and decodes its UTF-8 bytes. The registry conversion, dimensions, and test assertions remain intact. Runtime validation of this correction is pending; the same matrix continues with Terrain. Evidence: `🗑️generated/gis-independent-default-nx-10.txt`.

### Complete Pre-Fix Map Runtime Inventory

The retained binary metadata allowed an execution-only Nextest run without another Cargo build. Exact exit 100: all 150 tests ran in 2.967s, 132 passed and 18 failed. This binary predates the SVG fix. Twelve stale fixture suites expected content-derived child identities; all 24 neutral snapshots now carry the stable identities directly, their normalization wrappers are removed, and the suites assert identity preservation alongside unchanged exact snapshot/diff/inverse oracles. Repeated replacement patches exposed a real absorb defect: duplicate patch entries violated the apply validator. Absorption now combines payload replacements for the same feature, preserving an earlier payload across an empty patch. A four-case neutral fixture, strict schema, independent AJV/replacement check and serde_json runtime regression cover this behavior. Three remaining failures concern exact store mutation retirement setup and a retained fault payload; the registry executor owns those repairs. Evidence: `gis-map-existing-binary-full-red.txt` and `gis-map-patch-neutral-oracle.txt`. The queued ordinary Nx rerun will validate current source.

### Component Feature Runtime Gate

The normal Nx component test matrix is now queued for both Map and Terrain, selecting `component-app-assembly`, library tests and `--no-fail-fast`. It uses a private Nx workspace directory, the shared ticket Cargo target and two jobs with incremental compilation disabled. Receipt: `gis-component-native-nx-1.txt`. This gate remains distinct from the default-feature independence checks and runtime suites.

The GIS parent library test gate is queued separately (`gis-parent-native-nx-1.txt`), with `--lib --no-fail-fast`. Its full Stdio catalog feature is intentional: the parent publishes the existing exact 36-definition/26-codec catalog commitment. This is a composition-layer dependency; neither independent GIS artifact depends on that parent.

The stable-member audit also found an unused content-key computation: every handle refresh cloned and JSON-serialized all feature collections, then passed that string to two functions that ignored it. All four callers are now argument-free and the unused private content-key function is removed. Stable member identities remain covered by the exact neutral snapshot, inverse, and typed group tests; the pending native rerun includes this correction.

### Terrain Independent Default Check Pass

The tenth ordinary Nx matrix completed Terrain’s separate `check --no-default-features --lib` successfully. Cargo reported a finished dev profile after 44m39s including shared-target wait. The same process now runs Terrain’s default library tests. Map’s focused corrected-source check has acquired the shared target. No Terrain runtime pass is claimed yet.

### Corrected Map Independent Default Check Pass

The focused ordinary Nx check completed successfully against the corrected Map source: `check --no-default-features --lib`, five of five tasks, zero cache hits, Cargo dev profile 43m14s and Nx 43m42s including the shared-target queue. This validates the SVG export, stable handle signatures and patch absorption code as an independent default library. The same session35992 now runs the full default library tests with `--no-fail-fast`; the expected new neutral composition test and the three ownership-test repairs remain runtime-pending. Receipt: `gis-map-native-nx-11-svg.txt`. Both independent GIS default library checks have now passed.

### Current Block Parent Runtime Result

Ordinary Nx session36694 ended with exact exit1 after all seven Block parent library tests ran: six passed, one failed, zero skipped. `descriptor_is_fresh` rejected the Block3D viewer declaration: `app s.block.block3d@1/*#viewer document must contain non-empty segments`. The component-enabled Block2D/3D/5D crates and parent all compiled successfully before this assertion. The artifact executor owns the scoped declaration repair; the coordinator will retry the parent test after its source/neutral oracle is settled. The earlier Collection target in this same matrix passed all four tests and does not require repetition. Receipt: `collection-block-native-nx-1.txt`; total matrix101m06s includes the shared-target queue.

### Full Component Runtime Inventory And Fixture Repairs

The first ordinary Map component gate compiled and discovered241 tests. Several fixtures overflowed the default native test stack; other fixtures reached strict app authority validation and failed. Its fundamental15s execution budget killed the failed run before a complete summary, and the matrix continued with Terrain. A separate execution-only replay of the retained binary used a256MiB native stack, two test threads and no fail-fast: exact exit100, all241 tests ran in39.110s,207passed34failed, zero skipped. This replay required no Cargo rebuild. Receipts: `gis-map-component-full-pre-fixture-repair.txt` and `gis-map-component-failure-classification.json`.

- Missing real fixture registry: 24 failures.
- Missing fixture instance binding: 4 failures.
- Fixture terminal cleanup: 1 failures.
- Config serialization/neutral contract: 3 failures.
- Store reader retirement: 2 failures.

The artifact executor owns the29 app-fixture repairs: canonical action registry, exact live instance binding, explicit terminal close, and correctly registered paired fixtures. The Nx executor owns three direct config mutation contract failures. The registry executor owns two retained-reader close failures in the existing document Store tests. No runtime validation or retirement invariant is weakened. The isolated catalog law also failed with a larger stack (exit100, one test), proving registration is a separate issue; four selected inference/shell laws likewise failed on missing registry or live instance binding (`gis-map-component-catalog-isolated-red.txt`, `gis-map-component-registered-isolated-red.txt`).

The Block3D viewer now declares its canonical `semio/block/3d` document segments and has a typed plus serde_json descriptor assertion. Focused ordinary Block parent retry5194 is active (`block-parent-native-nx-2-document.txt`); Collection is not repeated.

The two Store close failures were traced beyond fixture setup to a production ownership cycle: `tail_undo_cache` and the displaced-snapshot queue retained the same current Arc, while the close state machine processed displaced snapshots before the tail owner. The registry executor is correcting the alias handoff and redo ordering, with external-reader retention still enforced. Its new native result remains pending.

## GIS Registry And Snapshot Ownership Repair Review

The coordinator reviewed the current Map testkit after the full 241-test diagnostic replay. Its app constructor now creates the production action registry, binds numeric instance 1, and exposes explicit terminal close. The GIS-local two-instance harness uses that same setup and closes both instances. These 11 test-source repairs are settled; they have not yet passed a current-source component run.

The shared Store repair recognizes when the previous current Arc is already owned by the tail undo cache, retaining only one retirement owner. Redo establishes its pre-edit tail owner before replacing current. New focused tests cover exact Apply/Undo/Redo retirement counts and a live snapshot reader that blocks close until its owner returns. The registry executor owns their runtime gate. Existing Map strict close assertions remain in place.

Root native gates remain active: Map and Terrain defaults, Terrain component, GIS parent, repaired Block parent and PDF publication. The GIS parent holds the shared Cargo target lock and has advanced from Wasmtime compilation to stdio leaf crates; queue elapsed time is not test runtime or a compilation-speed benchmark.

## Fresh Map Component Acceptance

All three scoped repair groups are source-settled. Required nullable configuration keys use explicit first-party `#[value(required)]` derive support; explicit null remains valid while omission fails. Floating-point fixture literals preserve their wire number kind. The nonfinite regression retains the generic JSON writer’s established null encoding and verifies rejection without state change at mutation admission and the fallible config-diff persistence boundary. The Map-local test router sets its tested debug stack size only for test commands.

The coordinator started ordinary `bun x nx run @semio-tech/gis-gismap-rs:test --output-style=stream -- --features component-app-assembly --lib --no-fail-fast` with the shared ticket Cargo target, two jobs, no incremental compilation and a private Nx workspace-data directory. Session 18593 is active; receipt `gis-map-component-native-nx-2-registry-config-store.txt`. No runtime pass is claimed. Shared Store and value-derive owners are also running focused exact regression selectors.

## Registered GIS Native Route Acceptance

Session 74426 runs three ordinary GIS parent Nx targets in a continuing sequential batch: `durable-three-store-assembly-native-check`, `map-create-region-group-native-check`, then `native-codec-check`. They execute four kernel plus one Map component law, one default Map law, and two parent integration laws respectively. The durable target honors the explicit unlimited build budget so Cargo queue time does not discard its build; the later two retain their existing one-hour build budget. All use the shared target with two jobs and incremental compilation disabled, and a private Nx workspace-data directory. Individual logs use `gis-exact-<target>-1.txt`, and exact statuses are retained in `gis-exact-native-1.tsv`. These runs verify the registered routes in addition to broad leaf and parent library tests; no terminal result is claimed yet.

## GIS Parent Pre-Edit Derive Failure

Parent session 39276 ended with exact exit 1 after 98m48 including shared-target waiting and compilation. Its earlier compiled value-derive rejected two later `#[value(required)]` field attributes in Map. The current field parser explicitly accepts `required`; the observed unsupported-attribute branch cannot be produced for that key by the current parser. The long parent invocation began before this repair and reached its leaf source after the attribute was introduced. Receipt: `gis-parent-native-nx-1.txt`. This failure does not establish current-source native success.

The shared target lock was released. Fresh ordinary Nx parent session 98734 uses a new private workspace-data directory and current derive sources, with the same shared Cargo target, two jobs and no incremental compilation. Receipt: `gis-parent-native-nx-2-current-derive.txt`. Other native waiters remain attached.

## Terrain Default Runtime Repair

The independent Terrain default check completed successfully. Its first default test run compiled all 50 tests but stopped after 23 passes and two failures. Replaying the retained compiled binary with `--no-fail-fast` and two test threads completed all 50 tests in 0.683 seconds: 46 passed and four failed (exit 100). The failures are one Store test fixture lacking its exact mutation retirement factory, two canonical JSON fixtures using integer `1` for the typed floating-point exaggeration, and one structural catalog lookup still resolving from the former package owner. Raw evidence is `🗑️generated/gis-terrain-default-full-pre-fixture-repair.txt`. The registry executor owns these repairs; a fresh ordinary Nx full-suite run is required before claiming Terrain passes.

## Shared Decoder Source Epoch

Three queued commands ended before their artifact tests could run: Map default test (117m24 including queue), Terrain component test (within the 141m10 component matrix), and Block parent test (78m53). Cargo reported E0277 at the shared replication map decoder line 139: an older source revision called `fields.get("kind")` on a slice of `(String, DslValue)` pairs. The current source already uses an iterator lookup, corrected by its concurrent owner; this ticket made no edit there. Current-source retries are required. Map's preceding default check remains a successful result, but this failed test attempt is not a pass.

Current-source root retries are attached: Map default uses `gis-map-default-native-nx-12-current.txt` (session 41462), and Block parent uses `block-parent-native-nx-3-current.txt` (session 24289). Both completed their four normal Nx generation prerequisites and entered native execution. The repaired Map component session 18593, registered GIS native-law batch 74426, GIS parent retry 98734 and native PDF build 90282 remain attached. No successful runtime result is inferred from progress alone.

## Terrain Retry and Cooperative Build Budget

The Terrain default and component fixture repairs are source-settled and reviewed in `📓️gis-terrain-fixture-repair.md`. Fresh ordinary Nx test runs are attached as sessions 51186 (default, `gis-terrain-default-native-nx-2-fixtures.txt`) and 66717 (component, `gis-terrain-component-native-nx-2-fixtures.txt`), with the shared ticket Cargo target, two build jobs, incremental compilation disabled and a 256 MiB test stack.

The pending region-group and native-codec routes now read `SEMIO_BUILD_BUDGET_MS` with their existing one-hour default, matching the durable route. This allows the explicit zero build budget of the existing native batch to survive Cargo queue contention; listing and test execution keep their existing positive deadlines. Bun transpilation and scoped whitespace validation passed. The running durable call is unchanged; subsequent commands in the existing batch will load the current router.

## Current Map Component Runtime

The current repaired component package compiled and ran all 241 tests through ordinary Nx. Nextest completed in 10.974 seconds with 212 passes, 29 failures and zero skips; Nx exited 1 after 70m38 including compilation and queue time. This supersedes the older 207/241 replay but is not acceptance.

The first-panic classification (`🗑️generated/gis-map-component-2-first-failures.json`) identifies 17 missing bounded draft-store disposer failures and 12 remaining command, render, streaming-envelope or convergence failures. The strict disposal assertions remain intact. The artifact executor owns the next Map repairs, while the registry executor checks the corresponding Terrain draft ownership seam before its queued component build. Raw runtime output is `🗑️generated/gis-map-component-native-nx-2-registry-config-store.txt`.

## Typed Command Admission Diagnosis

Read-only coordinator inspection found a common cause of the remaining command assertions. The current framework `dispatch_typed` admits and starts a retained asynchronous operation, returning an empty admission result whose output contains `operationId` and `generation`. Map's fixture returned that result directly and asserted immediate mutations or rendered state. The public `PluginApp` contract instead exposes bounded publication advancement, exact result pages and receiver/operation/generation/sequence/attempt acknowledgements, followed by effect/event/UI drains. The executor is updating fixture consumption against this protocol; the production admission and retirement guarantees must remain intact. Terrain's analogous fixture was flagged for the same integration correction.

## Durable Kernel Runtime and Map Retry

The registered durability route built the current kernel test binary and passed all four exact selected laws: exact three-store factories/one decision, late-member rejection, cancellation before journaling, and uncertain-journal retention. Each law ran one test with zero failures (0.14, 0.01, 0.01 and 0.08 seconds). The receipt records the binary SHA-256 `683f13500c678a7cd97e347cb27e191029b99fe63e4ef8b23979743e85312c1a`; exact logs and receipt are under `🗑️generated/gis-exact-native-1-artifacts/exact-cargo-laws-9kKVaA/00`. The fifth Map role-port law is still compiling/queued, so the whole registered route is not yet a pass.

The shared fixture completion helper and current Map lifecycle/window repairs were reviewed. Ordinary Nx Map component retry 3 is attached as session 67105, with `gis-map-component-native-nx-3-completion.txt` and private `nx-root-gis-map-components-3` workspace data. It runs all component library tests with `--no-fail-fast`.
