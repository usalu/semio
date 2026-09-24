# Distribution Naming Authority Review

Scope is the two remaining OS developer output mirrors. No directory moves, builds, regeneration, deletion or payload rewrites have been performed in either mirror at this checkpoint.

| Existing Path Relative to OS Developer Root | Files | Directories Below Root | Payload Bytes | Observed Purpose |
| --- | ---: | ---: | ---: | --- |
| `dist` | 1,864 | 71 | 323,609,977 | Existing Vite site with index/404 HTML, hashed assets, copied assets, extension and plugin modules. |
| `📦️packages/🟦️typescript/fixture/dist` | 1,875 | 71 | 290,462,284 | Older copied-asset mirror; no index HTML exists. |

The first tree was created in August and has some September HTML changes. Despite its age, its root cannot be treated as unrelated junk: the current Vite configuration has no explicit `outDir` unless a brand supplies one, so Vite still selects this default `dist`. The second tree has no incoming exact `fixture/dist` reference found in the inspected OS, root script, editor or CI configuration and lacks a site entry point; its ownership still needs confirmation before any recovery move.

The current `staticDirVitePlugin` uses each public `spec.route` as its copied physical output directory. Thus unchanged public `/plugin-modules` and `/extensions` routes still create unprefixed distribution directories even though the runtime's private physical module directories and children are repaired. Its configurable build filenames and routes must receive explicit current layout authority and aligned readers; a broad generated-output exemption would leave this recurrence unfixed.

The retained Vite `assets` directory contains 59 KaTeX font files plus application/library chunks, worker code, CSS and WASM (79 files altogether). Their filename hashes are cache identities, not handpicked semantic emoji. The copied `asset` tree additionally contains source-format examples and older font, mesh and icon arrangements. Blindly rebuilding would discard those old copies and still use the current inadequate physical-output naming defaults, so neither action has been taken.

The current source-side asset/font authorities were independently repaired by root and the renderer lane; those owners must remain coordinated with any forthcoming distribution layout choices. Existing raw public IDs are not silently rewritten merely because an internal physical directory was renamed.

## Approved Public Route Authority

Root approved two explicit current public route labels, `/🔌️plugin-modules` and `/🧩️extension-modules`, matching the already-handpicked private filesystem collection basenames. Public plugin/extension IDs remain unchanged. This supersedes the earlier narrower decision to preserve raw route labels: the later distribution review proved that unchanged labels recreate unprefixed physical output directories on every static build.

Before any consumer cutover, the new two-field `🛣️routes.json` authority and a strict definition in the existing deployment schema were added with language-neutral positive/hostile cases. The first registry run failed exactly on the missing route parser while 12 prior tests passed. After implementation, all 13 registry tests passed. Ajv independently checks exact allowed route values/keys; whatwg-url independently confirms the decoded path for Unicode and encoded HTTP request cases. The production parser rejects old raw routes, similar-prefix impostors, dot traversal, encoded separators, control bytes, malformed escapes and doubled separators. Current route consumers have not been changed at this checkpoint.

The exact incoming inventory is coordinated: this lane owns developer router/Vite, extension-store endpoints, registry generator, current installation records and their fixtures; kernel owns its watcher constants, framework/renderer fixtures and ShellHost install endpoints; hub owns its two Axum routes and real encoded-request verification; root owns the 26 mirror component import prefixes, Actor worker URL and four styling prefix/fixture changes. The frozen nested-Cargo projection witness's old source coordinate remains unchanged.

A broader OS physical audit reached 5,003 entries / 2,476 governed entries while excluding renderer and the two identified distribution mirrors. Its only findings outside the unresolved raw `fixture` parent were pre-existing misplaced reserved ticket/build-cache trees under the OS, shell and host Rust package roots. Those trees were not deleted or renamed. The successful focused current module-output audit remains the compliance evidence for the repaired deployed trees; no claim is made that the unresolved distributions are compliant.

## Coordinated Route Cutover

The OS authority remains in OS. Neutral framework `createDevPluginSource` and `createExtensionSource` now accept explicit watcher URLs from their owning OS adapter; no framework-to-product JSON dependency was introduced. The kernel lane proved the old hardcoded watcher behavior red, then its 89-test framework suite passed with explicit URLs.

This lane updated the developer watcher/router, Vite aliases and static output routes, extension-store watcher/install matching, registry generator, three neutral module-URL expectations, vendor-rebase fixtures, and 26 exact current installation metadata URLs. The metadata before/after comparison proves every byte outside the single route prefix unchanged. Normal registry generation succeeded for 59 plugin crates, 60 playgrounds and 45 framework packages. The generated public catalog now imports the same route authority.

The new nine-case language-neutral HTTP fixture drives the actual developer and extension-store adapter middleware functions, extracted with the TypeScript AST and supplied inert filesystem/watch dependencies. Encoded Unicode GET watcher and POST installation requests succeed; old routes, wrong methods, encoded traversal and separator impostors fall through. The independent whatwg-url parser agrees with each accepted path. All nine cases passed and emitted a runtime verification log without mutating real output directories.

The first concurrent full developer run had 82 passing tests and one expected-in-flight transport failure: a mirror still contained its old public prefix while root was applying that exact batch. Root subsequently confirmed all 26 mirror files / 108 imports cut over with non-import body hashes preserved. The fresh full developer rerun is pending. The first registry rerun exceeded its default 15-second process budget before results; its long-level rerun is pending. Neither unfinished or budget-stopped run is counted as a pass.

The old distributions remain untouched. Root retained coordination of the broad shared `/asset` consumer boundary, while this lane owns an explicit OS Rollup/output authority. A bounded shared-asset consumer inventory precedes any further public-route cutover.

The registry long-level rerun completed with 13 passing tests. After root's mirror all-clear, the full developer suite had 82 passing tests and a transport test timeout at its unchanged 20-second limit amid concurrent compiler work. The exact focused transport rerun passed both tests: nine actual middleware request cases and 12 byte-identical static assets resolving 345 imports across 56 plugin roots / 26 mirrors. Kernel received this all-clear for its canonical/direct worker refresh. No post-cutover transport assertion remains failing.

## Read-Only Build Inspection Safety

The complete styling finalization inventory is seven hooks, not five: mesh collection, favicon, deployment markers, HTML aliases, UI assets, bundled map tiles and static-directory copies. The latter marker and HTML writers also need to honor the producer's `build.write=false` contract. Root authorized this lane to repair the seven bounded hooks and the owned resolved-build interface without touching its ongoing asset transport prefix changes.

A new neutral fixture names all seven hooks and three modes (`false`, `true`, absent/default). The test snapshots all output bytes with an independent fast-glob roster and SHA-256 before/after, then checks positive expected output for true/default. The initial test setup omitted the mesh catalog's required `$schema` field and was corrected narrowly; that setup failure is not claimed as the behavioral red. The next attempt stopped in Nx project-graph processing before execution; a daemon-independent retry is pending. Production hook guards have not yet been added at this checkpoint.

The daemon-independent retry captured the actual behavioral red: all seven hooks changed output with `write=false`. Each hook now records `config.build.write !== false` and returns before any finalization write when disabled; `OwnedResolvedBuildConfig.build` explicitly owns the optional boolean. The full styling suite then passed 35 tests / 1,026 assertions, including unchanged positive writes for true and absent/default and preservation of retained witness bytes. No copy/delete behavior was weakened for actual write builds.

An in-memory Vite output inspection is now running through Nx with `write:false`, `emptyOutDir:false`, the runner config loader and ticket-only cache/output paths. It bypasses the OS native/plugin prebuild workflow entirely. A full 3,739-file SHA-256 snapshot of both retained distribution trees was taken first; neither tree will be considered preserved without the after comparison. Root owns the forthcoming shared `/🖼️assets` transport authority and will update the single corresponding positive path in the new seven-hook test at its coordinated cutover.

## Preserved Orphan Fixture Output

Root approved the exact move of `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/fixture` to this ticket's `🧪️distribution-fixture-recovery`. The source held only its old `dist` child, with no index page and no incoming exact reference in the inspected authored framework/root/editor/CI scopes. The no-overwrite move retained the whole tree. The post-move independent roster and full SHA-256 comparison passed for all 1,875 files / 290,462,284 bytes. Nothing was deleted or merged; these are retained recovery inputs, not disposable generated output.

The primary developer `dist` remains in place: renderer's WGPU Trunk HTML still has an actual `copy-dir` reader for `dev/dist/asset`, which the renderer lane reserved for the eventual coherent cutover. No new primary output is claimed.

## Output Inspection Limitations

The initial shell stopped with status 143 after the snapshot and before a Vite log was created. The first Nx exec attempt then encountered the repository package-script lifecycle name `nx`; calling the existing root script router directly avoided that routing issue. The Vite runner attempt still ended 143 without a plan. A native Bun config import avoided the runner's task-router transformation; its parent shell also ended 143 while orphan Nx PID 27675 and Bun child 27704 continued briefly. Those processes later disappeared, with browser externalization warnings but no plan/error trailer. No further blind retry was started, and no success is inferred from partial logs.

One concrete build-graph observation is recorded separately from the unknown termination cause: the OS production config lacks an explicit `import.meta.vitest` definition, so its current graph traverses UiDocumentStore's test-only Node imports. This is not proof of the signal origin. Root has been asked to coordinate any narrow production-test-exclusion repair before further output planning.

The retained primary output also passed its after-inspection roster/SHA check: 1,864 files / 323,609,977 bytes unchanged. Together with the intact fixture recovery, every original distribution payload byte remains accounted for.

Root approved the narrow production definition repair. The new neutral `🧹️production-tests.json` fixture is compiled by actual Vite/Rollup entirely in memory in a separate Node process, then the resulting ES module is executed. A resolver counter independently observes whether the test-only `node:fs` import reaches the graph. The fixture explicitly preserves its entry exports so runtime witnesses remain observable. Behavioral red then isolated exactly one mismatch: production resolved one Node test import instead of zero, while runtime and test-mode witnesses were correct. The OS Vite config now defines `import.meta.vitest` as `undefined`; the separate Vitest configuration is unchanged. Its green transport/boundary rerun is pending, and this finding is still not attributed as the cause of status 143.

That focused rerun passed all three transport/boundary tests. The runtime module remains executable; test-mode still resolves and executes its test witness, while production resolves zero test-only Node imports. Inspection of the repository timeout wrapper confirms it uses SIGKILL and emits a budget diagnostic, unlike the unexplained 143 terminations; the relevant source SIGTERM routines target explicitly selected development listeners and were not invoked here. One post-fix full in-memory probe is now bounded to 180 seconds, with a PTY and explicit signal/deadline diagnostics. No parallel inspection attempts are running.

## Test-Runner-Owned Inspection

The PTY probe returned status 1 with an empty log, before any signal/deadline marker. It produced no output plan. Root then authorized a bounded inspection harness in the existing developer task router, invoked only through the existing Nx test target. No output authority is inferred from the old distribution's hashed names.

The new `🔎️build-inspection.json` neutral specimen and `📐️build-inspection.schema.json` define the no-write settings and exact metadata record shape. Ajv validates positive and hostile records, while Node crypto and WebCrypto independently agree on the specimen byte length/digest. The neutral inspection and existing executable production/test-mode witness checks passed: 2 tests passed, 84 skipped.

The opt-in actual inspection requires a new diagnostic JSON path under a ticket's `🗑️generated` directory. It imports the actual Vite configuration, keeps every production plugin, explicitly disables write/public copying/output clearing, and places any cache/output proposal under that diagnostic directory. Its child has a 180-second maximum and no native/plugin prebuild. Every retained primary-distribution file is hashed before and in `finally` after the child; successful metadata is emitted with exclusive creation, never overwriting an earlier result. Asset/chunk contents are not rewritten or stored by this inspection.

The first invocation used the exhaustive test profile, which selects Node for coverage. It failed before collecting the in-source suite because Vite cannot bundle the existing lazy `bun:sqlite` import under that profile. No inspection ran. The ordinary long profile uses Bun and permits a 300-second task budget; its one bounded actual inspection is now running. This profile limitation is not reported as a product naming failure or as a passing test.

The renderer owner has decoupled Trunk's asset copy source from old `dev/dist/asset`: it now reads the canonical framework assets directory and writes the explicitly named `🖼️assets` destination. That removes this incoming dependency but does not make the old primary distribution compliant or authorize deleting it.

## Contextual Physical Census

The corrected read-only census supplies actual package ecosystems, parent directory roles, fixed-parent IDs and sibling filename IDs to the fixed-name resolver. It inspected 4,985 entries, including 2,461 governed names, and found zero missing, stacked, generic or repeated sibling emoji in that inspected scope. Earlier diagnostic output without ecosystem context incorrectly treated Cargo/Node manifests and dependency internals as authored violations; it is not valid evidence for renaming them.

The following remain explicitly pending repair, not compliant exemptions: the 1,864-file primary `dev/dist` and the three misplaced nested `.🧬semio` evidence roots under OS, shell and host Rust package roots. They remain untouched. Renderer belongs to its parallel owner; installed dependencies and the independently generated async-probe Cargo target are outside this authored census. A separate semantic-role diagnostic still has unresolved directory contexts to distinguish from schema-owned mutation identities; no complete central-registry claim is made.

The actual long-profile inspection passed in 91.84 seconds overall (78.85 seconds in the test): 89 Rollup output records / 163,770,277 bytes, independently hashed, with all 1,864 primary-distribution files unchanged and no output directory created. The exact facade/module/source metadata is retained in `🗑️generated/os/actual-rollup-inspection.json`. These are output observations, not permission to write guessed replacement names.

## Dead Test Asset Emission

Ten inspected assets were source/fixture payloads reached only from in-source tests, including Actor, kernel, UI, plugin and ShellHost source files. Installed Vite code establishes the mechanism: its definition pass retains `if (void 0) { new URL(...) }`, then its asset-URL plugin emits those files before Rollup removes the dead branch. The prior definition fix excluded test-only imports but could not prevent this earlier asset emission.

Root authorized a bounded pre-build transform only for JavaScript/TypeScript modules containing `import.meta.vitest`. The new neutral actual-Rollup fixture executes both production and test-mode modules, proves the runtime asset URL resolves to emitted bytes, and requires test-mode to retain both assets. Initial harness setup failures (URL decoding, callback arity and expected sort order) were corrected before the behavioral red was isolated. The final red showed exactly one extra production test asset while runtime execution and both test-mode assets remained correct.

`semioProductionTestBoundaryVitePlugin` now applies the existing Vite/esbuild transform with the exact test definition and syntax dead-code elimination before normal and worker asset collection. It does not modify authored source files or the separate Vitest configuration. Its first verification exposed esbuild's default ASCII escaping of emoji URL literals; explicit UTF-8 output is required and was independently confirmed against the actual transformer. The next focused green verification is pending. No accidental test source is assigned a deployment filename.

The UTF-8 focused verification passed all three tests. The full no-write production inspection then passed with 79 outputs / 162,395,172 bytes, exactly ten fewer assets than the prior observation. All ten test-source/fixture payloads are absent, and every primary-distribution byte remains unchanged.

## Hand-Authored Bundle Layout

`Dev/🚚️distribution/📇️layout.json` now declares the explicit source-owner-to-output mappings. Initial neutral schema/resolver verification passed: unknown or ambiguous identities, generic/stacked/missing emoji, path traversal, undeclared filename placeholders, duplicate output paths and shared sibling emoji are rejected. An independent emoji-regex census checks every declared output path, and the source roster covers all 60 installed KaTeX font files exactly (59 are currently emitted; Size3 WOFF2 is inlined).

The output root is `📤️distribution`, containing `🧶️bundles`. Meaningful bundle owners are bootstrap, shared shell runtime, backbone worker, Puzzle, PDF, Surface, Flow, Editor and math typesetting. WASM engines and their JavaScript bridges have distinct leaf emojis. Fonts are grouped by actual family, then weight/style or delimiter size, then explicit compressed-web/web/outline format filenames. Every choice is literal catalog data, not a filename heuristic or fallback.

The first strict full preview rejected the transient KaTeX CSS-import JavaScript loader. This source identity appears in the Rollup graph but Vite removes its empty chunk from the final inventory. It now has its own explicit `🧮️math-typesetting/📥️style-loader-[hash].js` owner, separate from renderer and stylesheet. The catalog has 12 chunk owners and 68 asset owners. A fresh no-write preview is pending; neither canonical output nor the old primary distribution has been written or cleared.

Root directed separate ownership for bundle/HTML/manifest outputs and streamed static-copy verification. The future generator contract will not pretend that its bounded Rollup preview covers the roughly 3 GB of copied plugin/extension/static-source payloads. No blanket generated subtree exemption is being added.

## Preserved Misplaced Cache Evidence

Complete inspection of the three nested `.🧬semio` roots found only six Cargo-created `CACHEDIR.TAG` files, two per tree, with old August 20 job-runtime ticket-shaped ancestors. No source, binaries, logs or current ticket data remained. Each tag is 177 bytes with SHA-256 `6d9d1d216e0f83abc5e5662ca62c92b4f23009466b54fa27321a69acdb778bb2`.

The following whole-tree no-overwrite moves preserved every descendant and every byte:

- `OS/📦️packages/🦀️rust/.🧬semio` → ticket `🕰️misplaced-cache-evidence/💻️os`.
- `OS/🔨️modules/🖥️shell/📦️packages/🦀️rust/.🧬semio` → ticket `🕰️misplaced-cache-evidence/🐚️shell`.
- `OS/🖥️host/📦️packages/🦀️rust/.🧬semio` → ticket `🕰️misplaced-cache-evidence/🖥️host`.

All six post-move SHA checks passed. Nothing was deleted or merged; these are retained recovery inputs outside generated output. Their previous authored-tree locations are no longer pending. The primary distribution remains untouched and pending.
# Bounded Manifest Follow-up

Further status: the first direct producer preview and its first isolated retry both stalled and were stopped by targeting only their own PIDs. Neither is a passing preview. A phase-instrumented retry has an explicit 240-second build budget and has reached 1,500 recorded input witnesses plus the worker's 52-file watch closure. The cause is not yet established. The initial circular-load hypothesis was not sufficient: the neutral direct cycle completes under Bun but exits with unsettled top-level await (13) under Node. Detached configuration imports complete with the same witness under both, and missing imports are rejected; this corrected neutral regression passes.

Generic preview registration is now bounded by explicit `previewArguments` when a project owns multiple generators. Existing `preview-generated` routes retain their exact established invocation. An alternate route must remain in the same project, have an explicit `preview-*` target, use safe literal command segments ending in `preview`, and match its physical Nx command exactly. Cross-project, empty, shell-injecting and mutating invocations remain rejected. The three source/metadata/workspace preview regressions passed with 61 assertions. The new distribution generator itself has not yet been added to the taxonomy because real producer completion and dependency closure remain pending.

Publication safety now has a neutral behavioral regression: missing API RED, then 3 selected Nx tests GREEN. A new sandbox publication is byte-checked; unknown children and divergent prior-owned bytes both reject without mutation; one exact prior-manifest-owned hash leaf is retired only after staging and its previous bytes remain available in a recovery staging tree. Bun's physical roster agrees with independent fast-glob. No real Dev distribution was published or retired by this test.

Dedicated Dev `generate-distribution`, `preview-distribution`, and `check-distribution` Nx routes now call the existing `📜️script.ts distribution ...` router. Exact launch-seed entries were added; canonical launch regeneration is pending. The first real producer preview is running without native prebuild or static-copy hooks. Compiler inputs are collected for main and worker builds and rehashed after compilation to reject concurrent source changes; output names still require an exact literal layout owner.

The previously long-running external publisher PIDs 80692 and 80695 are absent. Current copied component JS/JSON searches found no raw `/plugin-modules/` or `/extension-modules/` URLs. Its old ticket script writes completion logs only to an unspecified `LOG_DIR`; no completion result was inferred from process absence.

The current full Dev Nx suite passed 87 regular tests with the one opt-in production inspection skipped (88 total). The strict manifest feature then received a language-neutral fixture and JSON Schema: its initial API test failed because `parseDistributionManifest` did not exist, and the implemented parser plus layout-owner tests passed together (2 selected tests; 87 excluded).

Manifest owner IDs are the literal handpicked output templates already present in the layout. They map uniquely to exact source-owner coordinates; they are not synthesized from filenames. The manifest rejects unknown paths, raw HTML aliases, mismatched owners, duplicate paths or owners, unsafe coordinates, malformed digests and invalid byte counts. It requires the declared HTML entry. Node crypto and WebCrypto agree on the neutral digest witness.

Installed Vite `dist/node/chunks/config.js` exposes pre/post preview middleware hooks around its internal `index.html` fallback. The shared plugin already installs the emoji entry rewrites in those hooks. Its separate `closeBundle` hook additionally writes both `index.html` and `404.html`; those are not fixed-name authority for Dev. Four actual consumers were identified (Dev, demonstrator, presentation, Hub admin), so shared alias removal is pending an actual no-alias preview HTTP test and consumer coordination. The old primary distribution remains untouched.

## Qualified HTML And Actual Byte Evidence

The phase-instrumented bounded producer preview completed through the dedicated Nx route: 79 output files and 3,053 input witnesses. Its main compiler reported 3,243 watch entries and its worker 52. This is a successful read-only preview, not canonical publication. The earlier stopped attempts remain unknown failures; no causal claim is made for Bun's configuration cycle.

The independent existing Dev test runner subsequently validated every compiled output's byte length and SHA with Node crypto and WebCrypto, validated the emitted manifest with Ajv, and checked all output path emojis. It also streamed all 1,864 old primary distribution files and compared the complete independent fast-glob roster with the retained baseline. All bytes and paths remain unchanged; the new canonical `📤️distribution` is still absent. Together with the source-closure neutral regression, 2 selected tests passed (91 skipped).

Static compiler source closure now has a strict schema and explicit module-entry authority. The neutral cyclic module/JSON/builtin graph agrees with independent TypeScript import preprocessing and resolution; unsafe entries and malformed authority are rejected. Authored imports are closed through the installed resolver. External package entry bytes and nearest package manifests are recorded at an explicit dependency boundary, with the root Bun lockfile included. A fresh real build must validate this expanded closure; the earlier 3,053-input preview does not yet establish that result.

The shared HTML alias writer was removed only after a real Vite build/preview RED demonstrated that all four neutral HTTP routes already served the declared emoji entry while unwanted aliases were still created. GREEN now passes the full Styling suite: 40 tests and 1,363 assertions, including the seven adapters' retained-byte behavior and independent Parse5 checks. The exact presentation workflow assertion now requires `🌐️.html`. The Hub lane patched its three current fallback/root/document coordinates and reported a passing actual production-router native test: six HTTP cases, MIME and bytes, traversal 400 and missing-build 503, with both aliases absent. No old primary distribution alias files were deleted.

## Schema-Owned Compiler Preview

The remaining normalizer hardcode was repaired schema-first rather than special-casing Dev. Owned generators may declare an exact safe same-project argument vector and bounded preview resources. Established generators retain 128 MiB / 60 seconds; Dev declares exactly 256 MiB / 240 seconds. Unsafe arguments, foreign targets, unknown limit fields, fractional or out-of-range values are rejected. The normalizer now verifies the physical Nx command and invokes those same arguments and limits. Its protocol still rejects all stderr. Focused Bun/TypeScript invocation parity plus Ajv validation passed.

The generic `compiler-input-manifest-v1` authority declares one output manifest, one source schema, one static authority and a maximum file count. Its normalizer integration reads exact SHA/size rows from a current manifest to detect later input mutations, then reads the fresh preview manifest for initial generation and newly discovered dependencies. Opaque, self-produced, escaping, duplicate, unsorted, symlinked, missing or byte-divergent inputs reject. The regeneration freezes every fresh compiler witness; it cannot omit a preview-declared dependency. Six hostile authority shapes and four hostile language-neutral manifest variants passed against the schema/parser; the focused repo checks passed 4 tests / 87 assertions.

`dev-distribution-bundle` is now an owned contract with exactly three ignored outputs: semantic `🌐️.html`, `🧶️bundles`, and `🧾️manifest.json`. It names the 13 static authorities literally and uses the dedicated generate/preview/check Nx routes. The launch seed's three commands were materialized by the normal registry generator into `.vscode/launch.json`.

A fresh real preview after static closure passed: 3,080 exact input witnesses, 79 outputs, 3,249 main watch entries and 52 worker watch entries. The prior rerun rejected because the discovery implementation changed during compilation; that is expected source-race enforcement, not a producer failure. The direct machine-channel preview produced 216,416,095 stdout bytes within the declared limit. It also exposed one real 398-byte CSS optimizer warning from an invalid authored WebKit selector, so strict generic invocation has not yet been claimed green and the warning is not suppressed.

The invalid `*::-webkit-scrollbar:hover::-webkit-scrollbar-thumb` chain was corrected at its authored CSS authority to `*:hover::-webkit-scrollbar-thumb`, preserving the intended hovered scroll owner and a valid pseudo-element position. A focused assertion passed, followed by the full Styling suite at 41 tests / 1,365 assertions. The real machine preview then completed with 216,421,459 stdout bytes and exactly zero stderr bytes, qualifying the strict channel without suppression.

Canonical bounded publication is complete. After an explicit absent-root check, the generator published 79 compiler outputs plus one manifest, with no retirement. Its staging recovery is retained at ticket-generated `distribution-publication-E5mSuU`. The audit expectation was then advanced from pre-publication absence to exact canonical freshness; regeneration updated only the source-witness manifest and retained all output paths, with no retirement, under recovery `distribution-publication-sieEDL`. The final dedicated `check-distribution` route completed with 3,080 revalidated inputs and reports `distribution check: fresh`.

An independent final audit of the check's compiled bytes passed 2 selected Dev tests: every output has matching Node/WebCrypto SHA and size, the canonical root equals that plan, and the entire old 1,864-file primary `dist` roster still equals its retained path/size/SHA baseline. The new output partition contains 80 files. It does not claim ownership of copied plugin, extension, asset or other static namespaces.

The source taxonomy now declares exact meanings for the distribution's PDF/Puzzle/math bundles, font families, weights/styles/sizes, and dynamic outline-content leaves. A fresh scoped inventory covers 120 physical file/directory entries and reports zero violations, including zero missing, duplicate, generic or stacked emojis. A wider OS inventory covered 3,331 entries and likewise found zero path naming statute violations, while retaining 262 separate non-naming authority findings for further scoped work; those are recorded in `🗑️generated/os/os-final-taxonomy-inventory.json` rather than being misreported as clean.
