# Final Build-System Audit

Read-only snapshot taken during concurrent implementation on 2026-10-14. Findings below can change as owners finish edits. No build or runtime verification was performed by this audit.

## Actionable Public Boundary Findings

- Root `package.json:113` still invokes `nx run` for `test-reserved-producer-native`.
- `.vscode/launch.json:10–51` has two newly added direct `bun nx` commands, Nx environment variables, and Nx-shaped serialized capabilities. Remaining launches were largely migrated already.
- Hub Rust `📜️script.ts:3331` calls root script with `nx`; lines 3334, 4912, 5543, 6256, 6322, 12885, 12897–12898, 13243–13260, 13691, 15717 use `--skip-nx-cache`. Neutral selections reject this flag.
- Puzzle retained script line 23, Space testing script line 25, GIS services Rust script lines 113/119, GIS catalog publisher line 9, and repo test script line 36 also retain `--skip-nx-cache` in `bun repo` invocations.
- Hub actors line 78 and discovery runtime verification lines 405/456/463 directly execute `node_modules/nx/dist/bin/nx.js`, bypassing backend selection. Main owns actors; Bazel agent owns discovery verification.
- Dev stdio artifact graph lines 12/115 directly requests Nx graph and Nx environment. This is already assigned to the Bazel agent with its graph shape migration.
- Play fresh-build helper lines 20/25/30 exports Nx CLI flags, NX environment variables, and NX cache directory selection. Its tests and fixture encode the same contract and need coordinated updates.
- Play E2E invocation identity lines 10–14 requires `NX_INVOCATION_ROOT_PID` and throws without it. A neutral shared invocation identity must be issued by both backends.
- Play project lines 148–150 and Pets React project lines 67–69 publish NX runner environment settings.
- Acceptance orchestration lines 439/546/598 and shard execution line 99 set NX environment variables in generic task consumers.
- Rebuild contracts in Hub, Dev, and plugin registry still encode command arrays starting with `nx`. Demonstrator shell and GIS schema `localHub.owner.args` do likewise. Acceptance config lines 1624/1651 embeds `bun nx` subprocesses.
- Demonstrator E2E config line 23 tells callers to run through Nx; its comments and script documentation also describe Nx ownership. The assertion itself checks PLAYWRIGHT_BASE_URL and can describe the neutral owner.

`📥️audit.json` lists exact repository-relative paths grouped for targeted edits. Complex files are explicitly assigned elsewhere or require semantic changes; do not apply blind token replacement to them.

## Intentional Implementation Details

- Nx package dependencies, patches, nx.json, Nx metadata suppression in package manifests, .nxignore, and private bootstrap/adapter executor translations are expected while Nx remains a supported backend.
- Private Nx native selection seals/capability issuance and Nx graph cache inventory are backend internals, excluded from authored edit input.
- Dependency truth enforcing installed Nx packages and cleanup recognizing Nx processes support the installed backend.
- Nx-specific test diagnostics/oracles can name Nx. They should not be confused with live generic process callers.
- Cargo comments, root script comments, and Storybook comments still mention old direct Nx entrypoints; these are documentation residue with no execution effect.

## Audit Method and Limits

Used rg over hidden active sources, excluding ticket history, generated/cache output, node_modules, dist, target, and unrelated mesh normal identifiers. Narrowed command/API patterns after the broad identifier scan. No expensive graph generation or tests were run. Results cover public command/manifests/environment contracts; they do not claim backend behavioral equivalence.


## Final Integration Recheck

Later snapshot after concurrent corrections supersedes the earlier actionable inventory. No current blocker was found in the integration scope.

- Parsed `.vscode/launch.json` successfully and inspected all 7,859 configurations: none contains direct bun nx, --skip-nx-cache, NX environment keys, or node_modules/nx executable paths. The remaining nx input option intentionally selects the supported implementation.
- Root package now routes its last reserved-producer script through bun repo. CI workflows and devcontainer lifecycle commands expose neutral routes.
- Active TypeScript/JSON/module command and environment scan, excluding private backend/bootstrap/cache and test fixtures, now finds only Nx dependency truth, Nx inference observation internals, and the dashboard normalization implementation/rejection schema. Earlier generic process consumers were corrected.
- Root .bazelrc, .bazelversion, MODULE.bazel each have exact repository-root taxonomy exceptions with Bazel authority, permanent expiry, and bun repo tools setup bazel verification. Pinned version is 8.4.2; module name is semio.
- Dashboard launch check compares original current document with projected document directly; it does not normalize current before comparison. Thus stale public launch commands remain detectable. Generation writes normalized projection.
- Checked 75 scoped source paths (all audit input paths plus current build module files): no absolute source path exceeds 256 characters. An attempted broad Git diff path listing emitted daemon errors, so this result deliberately uses the explicit scoped filesystem inventory and does not claim a repository-wide changed-file check.

Verification here is static inspection plus JSON parsing/path measurements only. Main/core own runtime graph publication and backend tests. No source was changed by this final recheck.


## Exact Graph Authority Review

Focused read-only snapshot of graph authority and observation helper; core is concurrently finishing retries/tests.

Confirmed statically: graph membership hashes admitted files plus discovered directories using path, physical node type and full mode (graph line 124). Stamp-only changes are not membership differences. Observed file validation admits identical bytes after restamping by comparing SHA-256. Captured observations include inherited valid inference-cache facts and initial module-load facts. Pinned graph reuse checks mandatory SHA-256 against the exact file bytes before parsing (line 144). Stable inference errors are rethrown when membership and failed read facts still hold; changed membership/facts cause bounded retry. No graph regeneration or runtime tests were run for this review.

Concrete actual-observation blockers sent directly to core/root:

1. Observation helper line 143 records a directory listing via facts.readdir, then returns a separate fs.readdirSync result. A concurrent listing change can make recorded and consumed values differ. Record the returned listing itself.
2. TypeScript host line 170 records file bytes before calling sys.readFile. The second returned text can differ from the captured digest. Record the actual returned text/bytes with its matching read stamp.
3. record (lines 97–101) and retainSourceFactsV1 (line 57) retain only the first fact for kind/path. A second differing consumed read can disappear from authority. Preserve conflicting facts or mark the inference inconsistent so it retries.
4. Initial inventory/before membership (graph line 150), error-path inventory, and post-computation inventory (line 160) can fail outside the current inference retry catch. Core was informed and already owns transient retry completion.

These concerns are about interleavings during active concurrent writes. They do not negate the confirmed digest, mode, membership, or identical-content logic. No source edits were made by this reviewer.


## Observation Fix Verification

Read-only source recheck after the observation implementation agent completed its tests. All three earlier observation blockers are resolved in the inspected source: directory reads fingerprint the exact returned listing; each TypeScript host port fingerprints its own returned value; observations preserve distinct kind/path/fact tuples and explicitly reject contradictory values while permitting identical file bytes with different stamps. Cached project inference inherits rows through the same conflict-preserving function. Both synchronous current validation and asynchronous observationsHold reject mixed images. The latter checks AbortSignal before validation and between worker checks; graph passes its invocation signal.

Graph inventories now retry typed transient membership failures before inference, on inference-error recheck, and after inference. Stable inference failures still throw when original membership and facts hold. Provider native host readText and catalog host readText now read through observed readFileSync. Pinned consumer digest validation remains present.

One narrow remaining retry case was reported to core/root: membership lstat catch (graph line 123 in this snapshot) recognizes ENOENT only, while an ancestor directory replaced by a regular file can yield ENOTDIR for a captured descendant. Readdir already classifies both codes as BUILD_MEMBERSHIP_CHANGED. Treat lstat ENOTDIR equivalently so this consumed physical membership edit retries rather than failing immediately.

No source edits, graph regeneration, or test execution by this reviewer. Agent-reported regression counts are not independently claimed here.


## Final Inventory Audit

Snapshot: 1,239 unique combined entries; 31 absent paths, of which 30 are explicit workspace/core move or removal sources. No duplicate or erroneous bare ticket-relative entry in combined inventory. Root and bootstrap input inventories are fully covered; recursively interpreted workspace paths are fully covered. Bazel ticketInputs contains seven intentionally ticket-relative values and must be qualified against the ticket root during merge. Three such script values are absent as bare keys but correctly present as fully qualified combined paths.

Unexpected absent entry requiring correction or explicit removal attribution:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧬️schema/📋️project.json`

Definite missing final entries referenced by current dashboard inventory or core report:

- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️14/ABSTRACT-REPO-BUILD-TOOLS-AND-ADD-BAZEL/📝️bazel-control-review.md`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️14/ABSTRACT-REPO-BUILD-TOOLS-AND-ADD-BAZEL/📝️graph-read-authorities.md`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🕸️graph/🔏️seal/📥️membership/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧬️schema/📥️authority.json`

Extant authored ticket files absent from combined inventory:

- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️14/ABSTRACT-REPO-BUILD-TOOLS-AND-ADD-BAZEL/📝️graph-read-authorities.md`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️14/ABSTRACT-REPO-BUILD-TOOLS-AND-ADD-BAZEL/📝️bazel-control-review.md`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️14/ABSTRACT-REPO-BUILD-TOOLS-AND-ADD-BAZEL/cold/📜️script.ts`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️14/ABSTRACT-REPO-BUILD-TOOLS-AND-ADD-BAZEL/command-audit/📜️script.ts`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️14/ABSTRACT-REPO-BUILD-TOOLS-AND-ADD-BAZEL/modules/📜️script.ts`

Additional extant build-module files absent from combined inventory; root should decide task attribution before final merge (no whole-repo Git attribution performed):

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🏗️native/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🏗️native/🧫️fixtures/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🧫️fixtures/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧬️schema/📥️authority.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📖️storybook/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🛠️cmake/📜️script.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧬️schema/🛠️modules.json`

Measured absolute paths as UTF-16 code units, including current combined historical ticket scripts/configs and all current ticket non-generated authored files: none exceeds 256. This is scoped inventory verification, not a claim about every historical ticket in the repository. No source or inventory was edited by this audit.


## Structural Versus Execution Byte Authority

Read-only trace without duplicate graph discovery. Current neutral inference `🏗️build/🕸️graph/📥️inference/🟨️.mjs:1616–1630` parses every TS/JS module, computes its raw SHA-256, and publishes `metadata.semio.moduleGraph.files` rows containing file, hash, trackedHash, imports, productionImports. The dependency edge consumer on line 1619 uses productionImports plus current owner/package/lock resolution; it does not use hash or trackedHash. createMetadata publishes the complete diagnostic image on line 1630.

Literal-consumer inventory across active sources found only tests consuming this emitted moduleGraph: scheduling tests `⚡️caching/🕸️graph/🏗️scheduling/🧪️tests/🟦️.ts:40–53` verify schema, hashes, imports and defensive copies; observation project tests `🏗️build/👁️observation/🧪️tests/📦️projects/🟦️.ts:26–27` inspect changed hash/imports and read counts. Rust physical-reference fixture uses the same identifier for unrelated fixture cases. Production matches in Pets Vite builder, OS Vite plugins and library Vite caching refer to Vite server.moduleGraph, not metadata.semio.moduleGraph. No production consumer of emitted row.hash/trackedHash was found. Scheduling schema currently requires those fields and must be changed with its tests if metadata is decomposed.

Safe decomposition opportunities:

- Move diagnostic TS/JS full-byte hashes and trackedHash out of the structural graph to optional task/cache/source evidence. Preserve resolved production import edges, source provenance and physical membership.
- A hash-field deletion alone does not reduce concurrent-edit rejection: memoizedFileRecordAsyncV1 still records full byte file facts. Structural observation needs an explicit semantic parser fact identified by parser revision/path/options and its actual returned imports; validation must reparse changed bytes and compare complete consumed semantic output. Do not simply omit the observations.
- Rust cachedRustReferences/boundedRustReferences (inference lines 374–432) uses raw SHA as a private content-cache key. The graph consumes parsed modules/includes/error and physical candidate existence. A structural Rust observation can validate this complete reference result while leaving implementation-body byte changes to task hashing. Preserve #[path], nested modules, include/include_str/include_bytes expressions, path context and parser errors.
- Cargo manifests, authored project declarations and permanent scripts define targets, configurations, command strings, inputs, modes, dependencies and native/source closure behavior. Their raw facts cannot be removed wholesale. Any semantic admission must cover every consumed declaration, not only imports.
- Keep implementation/parser/schema revision hashes, lock/package resolution authority and content-addressed closure manifest digests. These identify semantics and exact inventory declarations, rather than arbitrary task-body bytes.

Execution byte authority already exists independently: `🏗️build/📥️inputs/🧮️digest/🟨️.mjs:7–23` hashes current exact source closure bytes, and lines 26–62 populate per-task source/lock environments. Bazel `🏗️build/🔧️tools/🏗️bazel/📜️script.ts:143–153` computes per-task source identities and runtime inputs; cached output snapshots also retain SHA validation. These stages are appropriate owners of full source-byte cache identity. The structural split must retain these checks rather than treating a semantic graph hit as a task-content hit.

No source changes or graph execution were performed for this trace. Consumer findings and conservative decomposition boundaries were sent to root/core.


## Iterative Loader Memory And Admission Review

Final read-only source review found no concrete blocker in the revised loader. loadBuildGraphAttemptV1 owns each candidate's inventory, captured graph, failed observations and publication locals. loadBuildGraphV1 awaits that attempt and retries iteratively at most nine times; it does not retain a recursive chain of earlier candidate frames. Module-level inference memoization/publication caches intentionally remain, so releasing candidate frames does not imply every cache allocation is discarded.

Ordinary cache admission first parses the small authority receipt, validates version-2 schema, membership and actual source observations. Only then reads canonical graph bytes; SHA-256 must match before JSON parsing and structural graph validation. Missing cache files, corrupt JSON and mismatched graph-byte digests rebuild. Explicit reuse keeps its stricter behavior: pinned source bytes require a present valid digest and exact match, and malformed explicit reuse refuses rather than silently replacing the requested snapshot.

Changed observations or transient membership failures return an absent candidate for bounded retry. Stable inference errors still throw after membership/fact recheck. Cancellation remains checked on entry to each attempt loop, inventory work and observation validation, including the validation AbortSignal. The previously reported lstat ENOTDIR gap is resolved. No empty graph or partial candidate fallback exists.

This review did not execute tests or graph discovery; runtime gates remain owned by main/core.


## Primary Inference Memory Retention Trace

Read-only response to measured pre-portableInputs RSS growth. No duplicate discovery, source edits or heap measurements were run. The following are concrete retained-reference paths; their contribution to the reported RSS needs counters or heap evidence rather than assumption.

Highest-priority multiplicative path is observation `observeInference`: rows combines the complete global loadObservations set with every individual project scope. `originalInferencePublications` retains that array per cachePath, and `observeProjectInferenceV1` retains the same publication plus observations and serialized project image in global projectImages. Cached-header hits parse full observation arrays independently. loadSealed flips only at the first observeInference call, while emojiProjectJsonNodes first performs catalog provider work, native owner traversal, nativeCommandInputs, taxonomy read and playground/staging discovery. All those initial observed reads can therefore become shared load facts copied into every project publication. With P projects and L load facts, the array storage is proportional to P*L even though the final capture deduplicates to about 101k unique facts. Cached header objects can multiply actual row/path objects too. Measure loadObservations.size, projectImages.size, total retained observation row counts, total serialized project image bytes and original publication counts. Factor immutable load facts once and retain per-project own facts separately; preserve union validation and publication admission.

A safe bounded release opportunity is to materialize a capture's final observations array and clear its Map in finally, including the failed-capture callback path. Global import/libraryBootstrap promises created under sourceCaptures.run may retain async context even after loader frames return; a cleared Map cannot keep all serialized fact keys and row references alive. The same principle can release observation scope Maps after their required rows have been materialized, inherited and published. Failure paths must transfer their facts before clearing. This does not authorize discarding source facts or weakening validation.

Other explicit roots: projectImages retains a JSON image plus observation/publication arrays for every cachePath without eviction; originalInferencePublications retains the latest publication per path; recordStores retains one open store/index/descriptors per path without a close/evict lifecycle. RecordStore values are read on demand rather than loading whole pack bytes; pending staged values are normally flushed after successful inference, but a compute error before flush retains pending values in the globally held store. An attempt-bound release API can close/flush stores and clear attempt memo/publication state after copying final authority, with a byte-bounded per-project warm cache if needed. It must run only after all awaited inference work settles and must preserve any SDK seal/publication consumers that still require the current attempt.

Revision-qualified importRevision URLs create distinct ESM namespace instances. If implementation revisions change during retries, the runtime module registry can retain older module-global cache universes even when reloadedImplementation points only to the newest revision. This is a conditional retention amplifier, not proven for retries caused solely by Rust/Cargo task-source edits.

Not identified as a primary unbounded root: RUST_SOURCE_CACHE has an explicit byte budget; RUST_INPUT_SNAPSHOTS and NATIVE_DIRECTORY_CLOSURES are WeakMaps; projectChecks is a WeakMap keyed by parent scope. The command parser is already byte-bounded per main. moduleGraphPublications is keyed by workspace and overwritten, and this reported phase precedes its dependency publication. Iterative loader frame release alone cannot free the global/ALS roots above.


## Current Authored Manifest And Schema Audit

Executed independent Ajv validation against the owned neutral project schema at `🏗️build/🧬️schema/📋️project/🔣️.json`. Current active authored inventory: 626 project declarations and 275 package manifests, excluding tickets, generated/cache output, vendor, dependencies, dist and target. All 626 project declarations parsed and validated successfully. No unresolved local $schema reference or external-schema reference was found. 146 declarations omit the optional $schema annotation; they were still independently validated and this is not a schema violation. No package manifest retains Nx metadata, and no package script directly calls the Nx CLI/bin under the checked command forms. No current blocker found.

Temporary exact machine output: `🗑️generated/manifest-audit.json`; main must delete generated output during ticket completion. No authored source was edited.


## Loaded Producer Staleness And Retry Lifecycle

Confirmed: this reviewer made no source changes in any audit, including no observationsHold/onStale callback changes. Only ticket reports and temporary generated audit output were written.

Concrete current instrumentation gap: graph buildGraphSourceFactsHoldV1 accepts onStale but its callback passed into observationsHold only logs profile diagnostics and never invokes the argument. Consequently the inference-error catch's stale array remains empty and moduleLoad classification cannot report the cause. This was sent to root/core immediately.

Persistent stale producer path: observation loadObservations is global, captureObservedSourceInputsV1 merges it into every attempt, and observeInference merges it into every result publication. No reset/replacement exists. invokeCurrentImplementation can import a revised producer URL, but that revised producer still imports the same canonical observation module. Thus a changed file retained as an original loaded-producer fact can invalidate every later attempt even when the filesystem becomes stable. Keeping old loaded-code facts is safe refusal; repeating full inference is wasteful and cannot heal it.

Safe behavior: distinguish immutable loaded producer/policy/schema facts from ordinary per-attempt authored source facts. When a loaded producer fact no longer holds, fail promptly with a typed producer-changed error identifying the file and requiring a fresh process, or reload the entire producer closure together with an isolated observation context whose recorded revision actually matches executing code. A fresh producer URL alone is insufficient. Never clear a stale producer fact and substitute current disk bytes while executing the previously loaded implementation. Continue bounded retries for ordinary consumed source/membership edits. Identical-byte producer restamps remain admitted by content validation.

Correction to earlier memory section: the complete invokeCurrentImplementation wrapper calls outer observeInference before entering emojiProjectJsonNodes, so loadSealed flips before primary catalog/nativeCommandInputs/project loops. The global load set contains module/bootstrap initialization facts, not all pre-project native discovery as that earlier hypothesis stated. P*L per-project copying remains a concrete path, but L and its memory impact must be measured; the overstated admission-window inference should not be used as a diagnosis.

No full graph execution or source edits were performed for this review.


## Semantic Fact Design Invariants

Read-only design audit sent to core and observation executor; no source edits or duplicate discovery.

Concrete consumer boundaries: command closure relativeScriptInputs (inference 465–487) consumes commandSourceImports and actual require/TypeScript resolution probes. genericTargetCommandInputs (539–551) additionally consumes all module import specifiers to derive external dependency identities. The full module dependency graph consumes imports and productionImports (1616–1619). These require distinct parser modes: command prunes type-only imports and import.meta.vitest branches; production excludes scripts/tests; all mode retains diagnostics/type imports. Rust closure consumes complete module/include/error references (374–432), including path context and missing candidate facts. Root command router extraction (1390 onward) consumes AST register calls, not only imports. Cargo/project/package/policy/schema and router sources therefore retain raw-byte authority unless separately given a complete semantic contract for every consumed property.

A semantic observation should identify registered provider ID, actual loaded implementation revision, parser mode/options, source path and stable JSON result. Registration must refuse ambiguous differing implementations for one identity, and persisted receipts must not select arbitrary code or module paths. Unknown provider or revision mismatch is a cache miss/refusal, never acceptance. Reloaded providers must be scoped to executing revision so old module facts cannot be relabeled as new code. Keep raw file observations when any independent consumer needs them; never globally suppress by path.

If source digest/stamp is carried as a fast validation hint, changed hint must trigger reparse and comparison of returned semantic result rather than rejection by raw hash alone. Mixed-read consistency compares provider/revision/options/result identity, not the mutable hint envelope; otherwise two body-only rewrites with equal import semantics still conflict. Resolution/existence/realpath probes made by a provider remain exact and validated, including error outcomes.

The owned authority receipt schema currently enumerates observation kinds explicitly; add semantic schema contract before producers. Inference cache hit inheritance, current validation, asynchronous validation, conflict detection and failure capture must all handle the new kind and preserve facts. Parser implementation/source schema revisions invalidate old records. Do not accept malformed/unknown semantic facts or non-JSON analyzer output. Cancellation must apply before reading and between revalidation work; stable parse failures still fail rather than becoming empty reference sets.

Independent oracles: TypeScript preProcessFile can validate broad imported-file scope; esbuild metafiles can validate runtime import closure without assuming identical type-only/test-branch treatment. Ajv should independently validate semantic receipt fixtures. Existing Rust physical-reference tests use actual rustc cases and can validate module/include target behavior. Language-neutral vectors should pair body-only equal-semantic changes with changed import/module/path/router declarations; task/cache tests must still prove changed body bytes change execution cache identity.


## Actual Graph-Free CLI Audit

Inspected dispatcher ordering before running commands: help returns before ensureBuildEnvironment/loadBuildGraph, and selected backend/task flag validation also occurs before environment or graph acquisition. Only tools setup is documented; there is no graph-free version or tools-inspection subcommand, so none was invented or executed. No setup, download, build, graph or show command was run.

Actual CLI results:

- bun repo --help: exit 0; prints neutral bun repo routes, selectable nx|bazel, run/run-many/affected/exec/graph/show/watch/reset/tools setup.
- bun repo --tool=bazel --help: exit 0; same neutral help, no Bazel preparation.
- bun repo --tool=unsupported --help: exit 1 with Unknown build tool: unsupported.
- bun repo run workspace:build --skip-nx-cache: exit 1 with Unsupported build option: --skip-nx-cache before task/environment acquisition.
- bun repo run workspace:build --output=unsupported: exit 1 with Unsupported build output style: unsupported before task/environment acquisition.

Default Nx is statically confirmed by owned build policy defaultTool and parseBuildInvocation fallback; help does not separately print the selected default. The actual strict errors are useful although Bun includes stack traces. No blocker found in this bounded audit. Logs retained temporarily under 🗑️generated/cli-audit-1.txt through cli-audit-5.txt and cli-audit.json for main cleanup.


## Implemented Semantic Authority Review

Read-only review of the implemented helper/schema/inference integration. No new concrete provenance blocker found beyond the provider-aware project validation memo already assigned to the observation executor.

Confirmed: immutable registered provider handles prevent arbitrary receipt code execution; registrations carry explicit 64-hex revision. prepareSourceAnalysesV1 binds command/module parsers to implementation revision, command parser bytes and resolved TypeScript compiler source SHA; Rust provider binds implementation revision and Rust parser namespace. Graph loader awaits producer bootstrap and registration before cached receipt checks. command, all/production module and Rust call sites now use semantic ports, and module metadata no longer embeds diagnostic body hashes.

Successful sync/async analysis records actual parser input hints and output SHA plus nested observations. Cache replay validates nested dependencies and JSON image, with a 16 MiB reusable image budget and no retained source buffers or parser trees. Failed analysis falls back to the actual raw source fact and transfers nested observations. Changed bytes reparse under registered revision and compare output plus nested semantic meanings; unchanged bytes retain matching output authority. Unknown provider/revision or malformed semantic contract refuses admission. Contradictory semantic results are distinguished from body-only hint changes. Observation schema adds semantic structure, ordinary authority schema admits analysis kind, and async/current/cache inheritance flows handle it. Capture Map finally clearing is present.

Independent raw consumers remain deliberately exact: inference goScript heuristic around 899, wasm tooling body check around 974, root router AST around 1411, native host/provider readText ports around 1272/1462, explicit provider owner.inputs around 1466, and package/TOML/project/policy/schema reads. If body churn touches those consumers it can still invalidate a candidate; semantic import admission must not erase their raw observations. Further performance work may give a complete derived boolean/router contract to specific consumers, never blanket suppression.

Provider-aware memo correctness remains important: a cached true validation must be keyed by registered current provider identity/revision as well as row fact, so replacing a provider during a parent computation cannot reuse an earlier true result. Dashboard owns this fix per main coordination. Loaded producer/source staleness still requires strict loaded-revision behavior rather than relabeling old code; this audit does not weaken prior producer-staleness guidance.

No full graph, duplicate Node project runtime, test execution or source edits were performed by this review. Earlier measured RSS remains a runtime concern for core; the bounded semantic JSON cache alone is not evidence that whole-process memory is bounded.


## Private Nx Configuration Projection Parity

Read-only source/installed-Nx review, excluding the env merge bug already owned by core. Current private projection chooses executor from base execution.kind/ownerArguments and then flattens configuration execution into executor options. Portable resolveBuildTargetV1 spreads configuration target properties and merges execution before task planning.

Concrete schema-admitted differing cases:

- Base noop plus configuration execution.kind=command still projects nx:noop; base command plus configuration noop still executes Nx commands. ownerArguments configuration changes likewise cannot switch the base-selected executor.
- Configuration inputs, outputs, dependsOn and cache become nested Nx configuration options, whereas installed Nx task input hashing (`dist/src/hasher/task-hasher.js:129`), dependency planning (`dist/src/tasks-runner/utils.js:47`), and output selection (`utils.js:220`) read base target fields. Portable/Bazel planning consumes resolved configuration target fields.
- Base command plus configuration commands leaves both execution properties after portable merge: Bazel chooses commands first (adapter line 224), but installed Nx run-commands implementation around line 94 chooses command first and replaces commands. A resolved projection or explicit mutually exclusive command contract is required for parity.

A scan of all current authored project configuration objects found zero overrides involving kind, ownerArguments, commands, inputs, outputs, dependsOn, cache or executor. These are valid public graph API cases rather than a demonstrated existing authored task failure; env remains the existing real-task issue separately owned by core. Ordinary scalar command/cwd and array args overwrite follows shallow option selection, subject to command-versus-commands conflict above. Target defaultConfiguration is preserved and both Nx task identity and Bazel identity consult it; project-level defaultConfiguration is not consumed by portable Bazel identity either.

Duplicate common nxBuildTargetV1 export remains at graph module line 43 in the inspected snapshot, with no production caller found; private projection nxTarget is the actual consumer. Core owns removal. Concrete cases were sent to core/root. No source changes, duplicate env test, native execution or graph discovery were performed.


## Authored Configuration Key Census

Current active authored project inventory grew to 632 during shared development. Enumerated every literal target configuration: 22 configurations total. Exact top-level key frequencies are args=22 and env=15; nested options keys=0. There are no authored command/commands, kind, ownerArguments, executor, inputs, outputs, dependsOn, cache or other target-field overrides. Every authored configuration therefore fits an execution-option-only args/env contract. This is a census of current literal project JSON, not dynamically inferred/generated configurations; those require separate producer contract tests and cannot be claimed covered by this census.

Exact temporary rows and counts: 🗑️generated/configuration-audit.json. Core/root received counts immediately. No source changes or graph generation. No tightening-schema validation is claimed while core is still implementing that schema.


## Final Direct Public Surface Gate

After core explicitly confirmed authoring/common schemas stable, ran only the two exact direct Bun test files: dashboard launch and library build-surfaces. Actual result: 13 passed, 1 failed, 27,053 assertions across 14 tests. Dashboard launch tests all passed. Blocker: build-surfaces test at line 44 compiles authoring schema with strict Ajv and fails strictTypes: missing type object for properties at #/allOf/0/anyOf/0/properties/options. Core/root notified immediately; reviewer made no repair. Remaining surface tests passed. Owned policy defaultTool still nx.

Exact log: 🗑️generated/surface-final-audit.txt. No Nx/Bazel, full graph, bootstrap/setup or native compilation was run by this gate.


## Final Green Surface And Tightened Schema Verification

After core confirmed the strict-schema correction stable, reran the exact two direct Bun files. Actual latest result: 14 passed, 0 failed, 27,057 assertions. This supersedes the earlier strictTypes failure. Independently compiled the current tightened authored project schema with Ajv strict:true and validated every currently enumerated authored project: 633 admitted, zero invalid/parse errors, zero broken local schema references. Of these, 480 have resolving local $schema references and 153 omit the optional annotation. Owned defaultTool remains nx.

Temporary evidence: 🗑️generated/surface-final-rerun.txt and manifest-tightened-audit.json. No source edits, SDK bootstrap, backend invocation, compilation or full graph discovery was performed for this verification.


## Final Report Consistency Review

Read-only review while final graph qualification runs exclusively. Completed validation rows have retained report evidence: build-core Configuration Identity And Node Qualification records 24/165 and actual Node 2/25; Bazel validation records final 8/123 in 35.80 seconds; workspace identity records all 22 native DSL tests; bootstrap identity records 4/187 and Bash vectors; dashboard report records final observation 44/185, installed CLI 1/16, compact expansion 5/26, and live watch qualification. Integration records actual MCP protocol/build/SDK and related qualification. The final-audit latest section records 14/27,057 and strict 633-project census.

Pending canonical graph publication, exhaustive Bazel planning and final default Nx execution are explicitly identified as pending in final-report. The report does not claim every monorepo action executed. macOS-only execution, unavailable PowerShell, unrun native Windows/Linux/devcontainer qualification, unrelated complete typecheck failures, and retained dashboard binary versus current source qualification remain accurately limited. Marker text distinguishes 97 proven repaired products from preserved unproven historical products/backend snapshots. Watch result matches actual Bun/Node plan equality, full source exclusions and live cancellation evidence.

One missing useful final contract caveat was sent to root: configurations override execution options only; executor/native owner identity, scheduling, input/output sets, cache and continuity are target invariants, and command/commands form changes are rejected. Current general configuration wording is not itself false, but readers should see this deliberate supported boundary. The final source inventory claim should be treated as provisional until root regenerates the final combined inventory, as already planned.

No final-report rewrite, production source edits, tests, backend activity or graph reads were performed for this consistency review.


## Targeted Raw Script Consumer Audit

Read-only trace after completed graph rejection; no source edits or graph execution.

Direct primary-inference script-body reads are three actual consumers: goManifest around line 899 consumes only the boolean regex match for literal go command or runGo function call plus root/existence/manifest selection; withWasmTooling call around 974 and predicate around 1069 consume only whether buildWasmWebV1/buildRepositoryWasmWebV1 call syntax is present, deriving SEMIO_WASM_BUILD_REQUIRED and workspace:deps-wasm-opt dependency; rootCommandTargets around 1411 parses router variable registration AST and must keep exact source authority unless a complete router AST projection is explicitly introduced. The first two are valid narrow structural-provider candidates already assigned to core; full imports and runtime bytes remain independent authorities.

nativeHostSourceFactsV1 reads script at line 63 but discards its text. The shared read helper checks every path ancestor and leaf via no-follow kind, then a UTF8 encoded-text budget of 1 MiB. New explicit script admission API can preserve only presence/ancestry/type/defined byte-budget result. Cargo and project text are actually parsed for package/bin identity, project root/name, native target executor/cwd/command, mode env and outputs, so those remain exact declaration authorities. declaredPlaygroundHostInputPathsV1 must still include the script in returned execution source inputs. A physical byte-size limit versus prior text decode/reencode size should be an explicit consistent contract, especially for invalid UTF8 boundary vectors; silently using stat size is not automatically identical.

No additional direct script read that only discards implementation bytes was found in the primary inference module. Provider readText returns text to arbitrary registered contribution logic; explicit owner.inputs are retained raw authority. Resolved tooling/bootstrap scripts and schema/policy implementation identities are also genuine loaded producer authority, not mere presence checks.

Additional concrete provider provenance concern sent to root/core: catalogBuildContributionsV1 load port around 1464 reads current provider source bytes and then imports the canonical URL. On later retries after a provider edit, the import registry may return old executing code while current reread bytes are recorded. Safe revision-qualified load or immutable original loaded-byte identity with typed changed-producer refusal is required; repeatedly rereading disk alone does not establish executing revision. This concern is separate from task script presence/semantic decomposition and must not be hidden by source observation suppression.


## Bounded Script Admission And Catalog Reload Recheck

Read-only current source review found no blocker in the requested scope. nativeHostScriptAdmissionV1 returns the exact decoded/reencoded UTF8 ≤1 MiB boolean predicate. nativeHostSourceFactsV1 separately checks every no-follow ancestor and regular-file leaf, calls explicit admitScript, rejects malformed result/oversize/absence/errors, and retains genuine raw Cargo/project parsing. No fabricated script text is returned. Script remains part of declared execution source inputs. The text adapter bridges real readText callers, discovery uses it, and filesystem view preserves its existing fatal UTF8 decoding; graph semantic provider preserves the prior Buffer.toString decoding used by its raw text view.

Core's script requirement provider retains exact prior Go and WASM regex booleans, so ordinary body changes preserve the structural result while changed classifications invalidate. Native script semantic result similarly changes on oversize; independent ancestry/absence facts remain exact. Root router AST remains a distinct raw consumer.

Catalog load now imports a URL qualified by the actual provider source SHA, then rechecks physical source bytes after import and throws on change. This addresses the earlier top-level canonical-module cache finding. It is not a claim that every arbitrary transitive dynamic-provider dependency is independently reloaded; that broader contract was not exercised here.

No source changes, tests, generic Node cache probe or graph execution performed. Findings sent to root/core/dashboard.


## Final Current Public Surface Recheck

Lightweight read-only source inventory while core qualifies the fresh graph. No direct Nx command or NX environment caller remained in root package/editor/CI/devcontainer/client/MCP code or relevant agent Markdown. The broader active source scan outside private backend/bootstrap/tests/history found only intentional Nx schema/dependency truth and dashboard normalization/noise rejection references. All 275 active package manifests omit Nx project metadata.

The current native script API declaration, JSON schema and runtime agree on exact withinLimit boolean and explicit admitScript; raw declaration and no-follow checks remain separate. Tightened execution-option-only configuration contract is present; core owns current declaration preflight so no duplicate validation was run here.

Latest combined inventory contains 1,506 paths, no AGENTS.md entry, and no existing listed absolute path exceeds 256 UTF-16 code units. This statement is limited to the authored task inventory; it does not certify every historical/generated path in the shared repository or infer Git attribution for concurrently edited files.

No source changes, large graph reads, backend/setup invocation or redundant test runs were performed. No concrete blocker found in this scope.


## Backend Target Class Coverage Audit

Read-only current authored census: 635 projects, 3,389 literal targets; 3,252 use repo:script, one repo:noop, and 136 omit executor. All 136 omitted-executor targets have options.command or options.commands. No unsupported authored executor was found. There are 37 continuous targets and three parallelism:false targets. Dynamic inference targets are deliberately excluded from this literal census and remain core's exhaustive graph planning responsibility.

Bazel executes source/native/custom script ownership through the same portable command recipe and issued original caller environment; native ownership does not become noop. Only explicit execution.kind=noop exits early. Missing command recipes fail explicitly. Continuous targets use separate service readiness/termination handling, while ordinary actions honor dependency closure and output restoration; nonparallel ordinary actions acquire exclusive task schedule lease. This audit did not run those branches; retained actual backend tests supply runtime evidence. Public strict flags remain neutral, and SDK executor names occur only at private Nx projection.

One concrete schema-admitted API mismatch was sent to core/root: authored Task permits top-level command, but neutralBuildTargetV1 moves only options into execution and leaves top-level command in target rest. A valid minimal top-level command declaration consequently reaches command-kind execution without command/commands, and Bazel raises Build command required. No current authored declaration uses top-level command, so no existing task failure was established. Normalize this supported shortcut or reject it in the schema; do not silently admit an unexecutable recipe.

No canonical graph reads, backend invocation, heavy tests or source edits were performed.


## Closure Hygiene Preflight

Read-only inspection only; nothing closed or removed. Combined inventory has 1,514 unique paths and all 45 retained ticket files are included: 23 Markdown reports, 10 JSON input/state documents and 12 authored permanent harness scripts. Exactly 128 listed paths are absent: 97 previous marker names, 30 explicit ownership move/removal locations and the documented intermediate project-schema path. source-changes.md records this classification. No unexpected absent class was found.

Generated output contains logs, reports, sqlite/Nx runtime files and three generated TypeScript fixtures: two 64-byte synthetic import files and the synthetic Bazel integration receiver. These are tool outputs, not authored retained input harnesses or copied repository implementation files. Retained harnesses import actual repository implementations and remain outside generated. No copied tracked implementation source was identified in the inspected ticket inputs. Removing only ticket/🗑️generated preserves authored inputs and Markdown evidence.

finish/📜️script.ts checks ticket_close advertises no_management and explicitly passes true. Close requires an explicit summary, absence of generated root, a report without current pending qualification phrases, and a nonempty string-path inventory without AGENTS.md entries. It calls the repository ticket_close tool and verifies ticket.json status is closed afterward. Current pending report phrases correctly prevent premature closure. Full inventory completeness is established by this preflight and final inventory generation; the close script itself validates shape rather than independently reconstructing completeness. No management Git command is invoked by this harness.

No source edits, close action, deletion or graph read performed.

## Continuity Audit After Interruption

Read-only source review confirms authored top-level command shorthand is moved into options before inference command-source extraction and again at the portable boundary, with ambiguous forms rejected. The regression validates the authored schema, normalization, configured command/environment agreement with Nx's actual option combiner, and rejection cases. The selected Rust aggregate replaces the base operation with uncached noop execution and empty inputs/outputs; every retained selectable configuration has an empty execution override, preventing old command/environment overrides from restoring a compiler invocation. Original dependencies and selected compiler roots remain in base scheduling. Its fixture contains both a retained library dependency and a selected producer, and compares actual Nx task-graph construction against the Bazel planner, so the scheduling assertion is non-vacuous.

Remaining static contract concern: inference targetScriptClosure reads only target.options.command and base cwd, while the admitted authored configuration contract permits command and cwd overrides. genericTargetCommandInputs and nativeTargetCommandInputs receive that base-only closure. No configuration-source expansion is present in those functions or project input inference. A configuration selecting a script outside the base script import closure/project can therefore lack its executable source closure in inferred task inputs. The current shorthand regression changes arguments on the same script and does not exercise this case. This is a source-level authority gap requiring either unioned configured command closures or a stricter authored configuration contract; no cache-hit runtime reproduction was attempted by this audit.

No source edits, heavy tests or graph discovery were performed. Final canonical publication and runtime qualification remain owned by root/core. The closure harness still refuses reports containing pending-qualification language and requires generated output removal before ticket closure.

## Policy Facet Ownership Correction Review

Source-only narrow recheck found no remaining blocker in the corrected ownership path. Discovery supplies the already inferred project owners to the policy facet before merging contributions. The facet selects the owner by physical root and publishes its existing name/root; its lint target spreads the complete prior target (including execution options, configurations and metadata) and appends lint-policy to the deduplicated prior dependency list. It adds a noop lint only where no lint existed. A preexisting lint-policy target throws instead of being overwritten. The contribution merge preserves all other prior targets and project properties; dependency discovery then runs against the merged physical projects, so the policy facet no longer creates a competing owner that can absorb edges. Final graph admission independently rejects two names sharing one physical root.

The permanent regression exercises all six recorded physical owners, alternating existing/missing lint targets, preserves an existing executable lint and prepare dependency, compares ordinary lint scheduling through actual Nx task graph construction and Bazel planning, validates independent Ajv admission, checks Nx root merging and GraphBuilder dependency retention, and covers lint-policy collision plus duplicate-root rejection. Configuration preservation follows the complete lint spread in source; this particular fixture does not explicitly add a configured lint override. No tests or graph discovery were run by this reviewer, and reported focused-test counts remain the implementation owner's evidence.

## Configured Input Correction Source Review

The correction unions base plus effective configured command recipes, merging each override with base options. Every command-list member contributes sources or marks conservative fallback independently; discovering one script no longer suppresses fallback for another unclassified command. Explicit args select a script only for a bare Bun/Node launcher with forwarding enabled, so script-looking argument data on an existing entrypoint is not treated as another executable. Both cwd and script tokens expand workspaceRoot/projectRoot, and inference callers now pass the physical root. Native lock discovery unions the effective configured command/list recipes. The shared literal tokenizer is extracted into an exported helper while declaredNativeOwnerCommandV1 delegates to it and retains the same native contract validation.

One newly introduced source-level rejection was sent to root and the correction owner: parsing string options.args is unguarded, although command token parsing is guarded. An authored source recipe using command `bun` and args `$SCRIPT` is schema-admitted but throws the strict native literal error during source discovery. Unclassifiable source args should preserve fallback authority; strict native owner admission should retain its rejection. The existing eleven language-neutral cases cover literal args only. No runtime reproduction or tests were run by this reviewer; the implementation owner is resolving and testing this narrow case.

Follow-up source recheck: the options.args parse is now guarded; failed literal classification leaves no source path and therefore sets fallback. The permanent language-neutral fixture includes dynamicArguments with `$SCRIPT`, empty expected script sources and required fallback. This resolves the narrow rejection found above without changing strict native owner token admission. No remaining source blocker was found within this configured-input correction scope; runtime gates remain the correction owner's responsibility.

## Private Nx Literal Command Bridge Review

Narrow source review of the new source executor, bridge, source/command schemas and projector found no actionable defect. The executor writes a versioned source image keyed by its exact SHA256; the bridge checks file bytes against the sole 64-hex argument before parsing and closed source-schema validation. The source contains the SHA256 of the original process invocation wire and the bridge requires that exact binding before receiving the original envelope. The receiver validates its existing deadline and uses its cancellation control; runOwnedCommand receives the remaining budget and signal. This route creates no replacement policy or capabilities and issues no broader descendant grants.

The outer Nx shell command is fixed Bun runner syntax plus the digest. The executable runner path and source directory travel in reserved private environment keys, with overrides refused; command text is read from the validated source file and passed as a separate Bun exec argument. Declared args and original caller args are combined as literal quoted arguments only when forwarding applies, including each command-list member's explicit override. The projected ownerArguments route still uses the separate existing owner executor, including its native owner admission; generic execution does not rewrite native requests. The projector only changes the executor projection and retains canonical project/root identity.

Source review followed original envelope admission/control through its existing implementation. No heavy graph or tests were run by this reviewer. Actual Nx/Bazel argv parity and new negative source/digest/wire/deadline gates remain the implementation owners' runtime evidence.

## Literal Bridge Cache Identity Trace

The invocation wire and its source image are staged inside executeLiteralBuildArgumentsV1, after Nx's task hashing/cache lookup boundary; neither is added to projected target options or declared hash inputs. This preserves stable task identity across different invocation deadlines.

Existing graph/tooling namespace mechanisms do not by themselves bind new engine code to task artifact identity: nxFiniteEnvironment scopes workspace data/socket paths, and boundedNxNamespaceV1 places graph metadata under toolingIdentity but task artifacts in the shared nx/artifacts directory. sourceClosureTaskEnvironments fills only declared exact-source and Cargo-lock tokens. Shared globals contain platform, Bun version and build mode. A source target whose executable closure is a small independent script need not include the private bridge/executor files. Root/core were informed that these new execution implementation bytes therefore need explicit Nx task inputs; the proposed private projection binding must preserve effective default source inputs when absent and must exclude per-invocation source staging.

This is a source trace only; no task hash or graph was executed by this reviewer. Correction ownership remains with core.

## Final Engine Binding And Production-Stage Review

Source recheck confirms the bridge cache omission is corrected in private Nx projection. projectBuildTargetForNxV1 appends nxBuildEngineInputsV1 to explicit task inputs or the effective default/default-dependency pair, leaving noop projection unchanged. nxBuildEngineInputsV1 starts from the real projector, executor registry, source executor, bridge and command schema, directly calls the exported reference classifier/resolver while closing actual local imports and file URLs, and emits exact owned paths plus external package and Node/Bun runtime dependencies. Directory URLs are classified separately; directory-index module imports are resolved as real files and missing imports throw. Invocation wire/source staging never enters these inputs. The native hasher regression mutates engine bytes, package identity and independent staging bytes, requiring only the first two to change identity. The reference regression compares fixed language-neutral cases to independent Babel parsing and real directory-index resolution to Node.

Production emojiProjectJsonNodes directly calls discoverNativeBuildOwnersV1, retaining the exact original owner traversal and progress/cancellation yield every sixteen candidates. Its normal projectWithDefaults call keeps the default hash representation, including persistent compact-source authority. The read-only source query invokes that same projectWithDefaults normalization and alone selects source representation; it refuses Cargo owners, retains original invocation checks and uses the same recipe/group construction. The configured-source regression calls this shared production stage rather than a replacement fixture normalizer, then checks independent esbuild/TypeScript source closure and actual Nx input expansion, including configured-only mutation coverage. No vacuous or weakened test substitution was found in this narrow review.

No code/tests were edited, and no heavy graph or test was executed by this reviewer. The reported runtime gates remain the implementation owners' evidence.

## Current Report, Inventory And Operational Cache Review

Read-only inventory snapshot initially found 1,524 unique entries, 128 absent historical/removal entries, no AGENTS.md entries and no existing recorded absolute path above 256 UTF-16 units. The two retained provenance inputs were then added by root; root's subsequent review reports 1,526 entries (1,475 source and 51 retained ticket files), still 128 absent, zero overlength and zero whitespace errors. This audit performed no AGENTS.md or source edit; inventory absence alone does not independently prove that every concurrent contributor avoided AGENTS edits. Earlier source-changes.md totals were stale at the initial snapshot and root owns their refresh.

The narrow current public-interface scan of root package, editor launch, devcontainer and build policy found no direct bun nx, --skip-nx-cache or NX-prefixed public controls. defaultTool remains nx. Final-report correctly labels the first publication as historical and final policy-owner publication/default Nx qualification pending. A successful focused source gate does not replace the required current default Nx, exhaustive task planning, stdio and Bazel surface qualifications; none is claimed completed here.

The retained qualify harness now obtains the canonical path from buildGraphPathV1, executes the real runOwnedBuildInvocationV1 under its original twenty-minute envelope, preserves the original control while counting progress, and verifies the published authority digest against pinned exact graph bytes. It does not bypass enforcing fact-store admission. Its path correction does not supply missing runtime publication evidence. The read-only source query and owner-prefix tests are bounded production-stage evidence, distinct from full operational fact-store admission.

Concrete reset integration gap sent to root: the build reset branch still recursively removes only historical `.🧬semio/🦑️repo/⚡️cache/build`. buildGraphPathV1 now names graph.json inside repositoryCachePartitionRootV1(root,"graph"), and loadBuildGraphV1 publishes there under the enforcing graph fact store and build-graph-state exclusive queued lease. Backend Nx reset does not target that neutral graph partition. The inspected enforcing partition/graph modules expose custody and snapshot retention but no reset/invalidation API. Consequently current reset leaves current canonical graph/authority intact. This is a reset behavior omission, not evidence of stale-source admission; correction should use the enforcing owner's custody rather than raw recursive deletion of the mounted partition.

## Expanded Public Interface Recheck

Scoped rg recheck covered CI workflows, devcontainer lifecycle files, editor controls, repo client/CLI/test/dashboard sources and agent-client templates, excluding generated output, fixtures/tests where appropriate, third-party installations and archival ticket reports. No active direct bun nx, skip-nx-cache, Nx executable path or generic NX daemon/parallel control was found. Dashboard matches in registry NOISE_ENV, its schema and README are explicit rejection/documentation that these private environment values are never stored, not public execution routes. A fresh parse of all 275 active package.json manifests found zero Nx task script callers and zero Nx authoring metadata. This is static public-surface evidence; current runtime publication and backend qualification remain pending.

## Controlled Neutral Reset Correction Review

Narrow source review finds the reported reset omission corrected. The public reset branch now calls invalidateBuildGraphV1. Both invalidation and graph publication enter withBuildGraphStateV1, which retains the original enforcing graph fact store and the identical exclusive queued build-graph-state resource/control. Invalidation checks store/root identity, inventories actual entries, selects only canonical.json, authority.json and bounded UUID staging through the reset schema, and refuses nonregular files, multiple links, oversized files and foreign devices. It rechecks storage, contained path and each current file fact before removal, with original cancellation/progress checks throughout. Immutable pins and source facts remain; no mounted partition root is recursively removed. The publication path and stale-staging cleanup now consistently use canonical.json.

The permanent selector test compares real filesystem inventory against fast-glob and independent schema validation, checks malformed ownership cases and verifies already-cancelled original control prevents mutation before store admission. That is meaningful selection/control evidence; it does not by itself prove successful deletion inside the mounted enforcing store. The implementation owner retains responsibility for any additional runtime gate. No source edit or test execution was performed by this reviewer.

## Updated Final Report Claim Review

Read-only comparison of the updated final report with build-core.md, configured-inputs.md, integration.md and graph-reset.md found no contradictory runtime completion claim in the reviewed update. The 42-test/715-assertion combined source gate and 10-test/27,323-assertion registered build-surfaces preflight are recorded in core's current qualification section. Configured-inputs.md supports the real native hasher's five-assertion engine/package/staging distinction and the read-only normalizer's 189 assertions, explicitly separating historical full-producer red evidence from current source-stage evidence. Integration records the three real enforcing artifact-store tests/19 assertions. Reset documentation and the final report both clearly limit its executed proof to real filesystem selection plus original cancellation before admission and expressly state that positive mounted reset was not run.

The final report retains current-store publication, corrected whole-graph ownership, exhaustive Bazel planning and actual final default Nx/Bazel execution as pending. The historical first graph digest is identified as diagnostic rather than current. Broader native platforms and unavailable whole-package compiler checks remain qualified. This review did not rerun runtime gates and does not promote implementation-owner evidence to independent execution evidence. No final qualification or ticket completion is claimed.

## Whole Active Canonical-Path Audit And Private Pin Review

Whole active-repository rg scans for graph.json, canonical.json and cache/build forms, excluding installations, caches, Git, generated products and archival ticket history, found no additional historical canonical graph consumers beyond the assigned dashboard runtime/tests/docs, build-mode fixture reader and private Nx fallback. Unrelated domain/runtime graph fixtures, exported scratch graphs and backend project-graph files were classified separately. Stdio artifact graph intentionally writes a scratch graph.json through `bun repo graph --file`, so it is not a historical canonical-path reader. Noncanonical historical cache locations remain separately identifiable in private command staging, shared test-inference fallback and compact-source fixture readers; this scan does not claim those are canonical graph consumers or that their operational ownership changed.

The corrected private Nx graphPublication now admits a closed required path/digest object, requires an absolute host path before reading, requires a lowercase 64-hex digest, and verifies exact file bytes before JSON parsing. All private nodes/dependencies/authority consumers share this helper. There is no historical or new common-canonical fallback and no optional digest admission. The new publication schema is itself captured by the engine source closure through its literal file URL. Source-only review found no remaining blocker in this private pin correction; implementation owners retain runtime test evidence. Dashboard shared-path correction is still awaiting its owner's ready notification for separate review.

## Final Current-Store Integration Review

Source-only review found no remaining blocker in the narrowly coordinated integration corrections. The common graph module captures SHA256 of its actual loaded implementation and shared publication declaration. Production checks that pair before enforcing custody, after the queued graph-state writer is acquired, when admitting cached authority, and immediately before canonical renames. Published authority is version 3 with required closed module fingerprints; buildGraphProducerAuthorityV1 validates complete receipt schema and the loaded pair, refusing version-2/unbound or mismatching producer claims. Permanent cases compare independent Bun/Node hashes and Ajv schema/equality admissions rather than accepting a merely renamed receipt.

Neutral explicit reuse and private Nx consumption now share the closed mandatory pin schema: absolute path plus valid exact lowercase SHA256, checked against actual bytes before parsing. Neither falls back to a canonical location or accepts an unbound digest. The actual private plugin negative cases and common reuse cases exercise their real consumers; no second public API was added.

The TypeScript publication path and authority name consume the shared neutral publication declaration. Dashboard registry and inventory consume the shared Rust graph_file helper, which derives the same path from publication, partition and mount declarations. The retained report limits Rust execution evidence to a freshly compiled actual helper and current independent fixture/oracle checks; it does not use older dashboard binaries to qualify these changes. Shared test node and dependency inference both invoke testBuildInferenceDirectoryV1 and therefore use the current declared fact-store path by default, retaining the explicit authored override. The production use is direct, not test-only substitution.

No source/tests were edited and no heavy graph or tests were executed by this reviewer. Current canonical publication, default Nx, exhaustive planning, stdio and final Bazel surface acceptance remain separate pending runtime gates.
