# Build Mode Integration

Exact vocabulary changes remove dev/ship build-mode aliases and wasm-iterate compiler-profile identity. No compatibility adapters remain. Cargo manifests and native compiler policy are owned by the native agent.

## Changed Source Owners

- `📜️script.ts`
- `🌎️hub/📦️packages/🦀️rust/📜️script.ts`
- `🌎️hub/🧫️fixtures/🌳️unit-census/🔣️.json`
- `🌎️hub/🧪️tests/🤝️dev-collaboration/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧫️fixtures/🧾️cargo-provenance/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/🧭️document-open-plan-v1.json`
- `🧰️framework/🛍️products/💻️os/🧪️testing/⚖️scale/📤️publication/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧪️tests/✅️catalog-complete/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🔄️refresh/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🚀️commands/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🌐️browser-host/🏗️staging/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🧰️preparation/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🏃️execution/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🩺️readiness/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🌐️serve/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧫️fixtures/👁️watch-policy.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧫️fixtures/🦀️wasm-profile-policy/🧬️v1/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/📦️materialization/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🧩️driver-stages/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🏃️execution/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/📋️plan/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/👁️watch/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🛂️descriptor/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/⚖️parity/🏃️execution/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/⚖️parity/🌐️server-pool/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🏃️execution/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧹️export-path-policy/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧪️ticket-owned-browser-host-staging/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/📇️canonical-bootstrap-folder-mirror/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧹️host-handle-policy/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧹️capability-policy/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/✅️verification/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚚️distribution/🏃️execution/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📊️benchmarks/🔌️plugins/🏃️execution/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📊️benchmarks/🔌️plugins/📋️plan/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📊️size/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🛂️descriptor-emission/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🧩️component-stage/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏷️component-receipt/🧫️fixtures/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏷️component-receipt/🧬️schema/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏷️component-receipt/🧬️schema/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏷️component-receipt/🧬️schema/🧪️tests/🔬️unit/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏷️component-receipt/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏷️component-receipt/🧪️tests/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧫️fixtures/🔏️document-execution-target-lease-v1/🔣️.json`

## Environment Bindings

- `📋️project.json`
- `🌎️hub/🧪️tests/🤝️dev-collaboration/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧫️fixtures/🎮️registry/🏗️workspace.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧊️browser-serving/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧊️native-runtime/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🌐️server/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🟦️.ts`

## Coordinator Runtime Validation

- Added schema and language-neutral mode/policy vectors before implementation. The first Nx run exposed a missing explicit test artifact directory; the properly owned red run then failed on the absent implementation module, as intended.
- `bun nx run @semio-tech/framework-process:test-build-mode --configuration=prototype` executed the owned implementation against Ajv and TypeScript transpilation: 3 tests passed, 68 assertions, no failures. Revalidated after shared infrastructure integration: 3 passed; task time 5.0 s under parallel fleet load. This is task runtime, not total launcher latency.
- Extracted workspace BuildScript and Storybook discovery into direct command owners, with root routing loaded lazily. Root build/build-storybook/build-canvas-font-assets targets invoke the dedicated canonical script and no longer load the root audit monolith.
- Added language-neutral workspace build ownership vectors before extraction. Red Nx execution failed on the absent owner entrypoint. After correcting the independent bundler test's wildcard and literal filename matching, the prototype Nx suite passed 2 tests with esbuild import-boundary and actual mode dispatch observations. Task runtime 1.8 s; no root audit or interactivity test implementation is included by the independent bundler.
- New prototype/release workspace build and all added test commands are registered in launch.json. Parsed launch.json, root project metadata, and nx.json successfully.
- Real release execution and root prototype graph probes exposed two integration defects: native original-task selection ignored --configuration, and generic OS build aggregators depended on fixed release targets. Both have been assigned to and corrected by the Nx workstream; independent revalidation remains required before declaring acceptance.
- New process-build, process-build-mode, repository build, Storybook build, and CMake owner taxonomy declarations are explicit.

All generated command logs remain below the ticket's generated directory during ongoing work. No full monorepo build, Windows/macOS/Linux parity, or goal completion is claimed by these focused checks.

## Final Integration In Progress

- Workspace build entrypoint and selected graph admission ran through Nx with both prototype and release configurations and excluded prerequisite execution: both succeeded. These are admission/entrypoint checks, not full workspace compiler builds. Current task durations were 9.8s and 10.8s under concurrent graph/compiler load; launcher elapsed time is measured separately by tooling.
- Go binary resolvers and coordinator serving use exclusive mode/owner directories `cache/🗃️bin/<mode>/<owner>/<binary>`. The owned staging API replaces an output directory, so a shared parent binary directory would discard sibling tools. Coordinator dev consumes its declared Nx build dependency and does not rebuild inline.
- Schema-first driver jobs now declare jobserver or reserved worker protocol. Jobserver clients share the parent token pool. Other compiler children reserve available tokens, inherit the reserved parallelism, and receive no jobserver flags. Nested driver pools are capped to their admitted capacity. New fixture vectors and actual child-process budget/slot-release checks are integrated with independent Ajv admission and existing real Cargo/jobserver checks.
- Red was confirmed through Nx with the missing worker-budget export. The first full green attempt failed because a standalone Nx exec suite had no artifact parent directory; suite setup now creates its supplied ticket-owned parent. Rerun pending.
- Added Go mode contract launch registration; updated coordinator command documentation to the actual dev handler.

Worker-budget rerun through Nx succeeded: 14 tests, 155 assertions, actual Cargo/WASM pass and runtime reserved-worker log. Four held workers were observed; the next jobserver child and Cargo completed, establishing token release rather than merely testing the budget helper. Launch seed now preserves all new mode/build contract entries, workspace prototype/release build entries, and declaration-worker debugging targets.

## Representative Build Measurements

Real inferred `@semio-tech/framework-io-base64-rs:build:prototype` compiled successfully through canonical Nx routing and the owned artifact driver. Actual Cargo compilation was 0.51s, while task critical path was 75s. Its trace exposed duplicate native owner-command wrapping and an unconditional synchronization costing roughly 50s before Cargo. This is an unresolved performance failure, delegated to native/tooling integration; it is not evidence of a fast warm iteration loop.

Full workspace prerequisite graph generation failed for both modes with Nx native matcher `error building NFA`. The declaration debug target's graph launch also failed with undefined project-graph errors. Tooling is diagnosing these independently from successful excluded-dependency entrypoint checks.

## Exact Whole-Pool Compiler Reservation

Bun's internal bundler exposes no confirmed worker limit in the installed runtime. Simple TypeScript artifact bundling is being moved behind an owned esbuild child interface. Essential virtual-source Bun bundling enters full-host admission and refuses inherited partial capacity. The Flow WASM pipeline job is explicitly exclusive because its post-build browser publication requires that bundler.

The neutral jobserver now supports acquireAll without holding an initial token. A whole-pool request accumulates available slots at the queue head while already running compilers finish, and later requests wait. Cancellation returns partially reserved tokens. This avoids the deadlock where two exclusive jobs each hold one slot while waiting for the rest. A stage's explicit environment carries its actual held width and mode; it does not rely on mutating ambient process environment.

TDD: initial tool startup attempt failed due a concurrent immutable-tool workspace dependency change; a second Nx attempt observed the expected missing acquireAll method. Integrated rerun passed 21 tests, skipped 1 unavailable GNU make ≥ 4.4 oracle, with 224 assertions. The Rust jobserver crate oracle verified restored pool capacity after cancellation. Runtime recorded two whole-pool jobs and two whole-pool stages with four workers, serialization, and successful Cargo progress.

## Compiler Budget Coverage

All standalone WASM pipeline jobs now reserve the whole pool: wasm-pack can run non-jobserver-aware optimization after Cargo, and Flow's post-build browser publication requires virtual-source Bun bundling. Controlled component optimization continues borrowing only available slots. Stage environments carry exact worker count and selected mode into Node V8/libuv, Binaryen, descriptor emission, and probe operations. The actor/virtual-source bundler is being isolated into a cancellable child compiler.

Private source closure and Cargo lock hashing are being batched inside Nx's hasher environment map, preserving scoped ownership without launching a subprocess per closure or polluting compiler child environments. New canonical digest debugging commands and launch seed registrations cover source lists, inventory manifests, and reachable Cargo lock resolution. Ticket input source-closure.json is handcrafted configuration and is retained.

## Coordinator Revalidation

The workspace build ownership suite was executed concurrently through Nx with prototype and release on the shared checkout. Both finished successfully: 2 tests, 867 assertions per configuration. Independent esbuild closure inspection excludes the root audit monolith, and actual script mode dispatch emitted the selected graph mode. Task duration was 2.3s each under fleet load. This remains ownership/runtime dispatch validation, not a full compiler graph build.

Added permanent launch and seed registrations for native queued cancellation and bounded compiler capacity. Direct source-closure, inventory and reachable Cargo lock digest debugging commands each completed through their canonical Nx targets.

## Bounded Single-Core Phases

The neutral capacity API now accepts an explicit single-core request. Its omitted request remains a full-host compiler claim. The execution workstream observed overlapping real children, saturation at host capacity, priority for a queued whole-host writer, queued cancellation and SIGKILL owner recovery. A Python SQLite transaction independently observed that two shared children block an exclusive holder. Three targeted tests passed. Web compilation is integrating separate bundle, declaration and checking phase claims and a common host lease directory. Direct declaration and virtual Bun debugging launches are being executed in both modes.

## Disk Pressure During Acceptance

Full prototype graph admission succeeded for all 1,213 tasks in prototype mode. The initial debug export was 371,581,710 bytes and repeated 743,791 hash-plan instructions with only 7,606 unique instruction strings. The tooling owner is replacing repeated export arrays with a schema-first dictionary and task indices. The underlying graph had been reduced to 31.1MB compact. This export establishes graph admission, not compiler success.

Both direct declaration and virtual Bun debugging launches succeeded through Nx in each mode. Declaration output count was 1; virtual actor responses loaded both captured runtime fragments and emitted 145-byte prototype versus 47-byte release JavaScript. Total launch measurements under continuous fleet graph changes were 57.085s/56.781s for declarations and 68.128s/68.489s for Bun prototype/release. These are successful runtime checks and poor contended launcher measurements, not evidence of an acceptable warm loop.

ENOSPC interrupted later graph writes and package provisioning. Obsolete ticket diagnostics nx-plans.json (1,273,621,662 bytes, prior giant failed hash-plan export) and nx-full-prototype-lock.json (371,581,710 bytes) were removed after retaining their measured findings here. Compiler ownership and current state must be respected during space recovery.

## Executable Artifact Probe

An initial inline Bun eval through Nx exec failed because Nx shell quoting did not preserve nested JavaScript quotes. No artifact-runtime success was inferred from that attempt. A retained canonical ticket input script now generates both worker requests from the existing handcrafted JSON inputs, then imports each actual generated artifact and asserts its result. Through Nx, observed console output was answer=5 for prototype (145 bytes) and release (47 bytes); both runtime snapshots compiled and the command exited successfully. The script and its permanent launch seed entry are retained for independent re-execution after generated-folder cleanup.

Added Go executable acceptance launch registration. The native workstream observed the zero-touch canonical rebuild of six owner/mode Go binaries and validated their compiler flags, CLI help, MCP initialization and isolated coordinator HTTP health in both modes: 1 test, 30 assertions, 41.05s runtime. Targeted profile admission and queued/metadata-child cancellation passed 4 tests, 48 assertions, 3.1s.

Both full graph exports now succeed with 1,213 tasks in the respective selected mode. Dictionary encoding reduced exported files from 354MiB to about 70.6MB; planner timings expose remaining native matcher repeated workspace scans and are being corrected. Full compiler graph execution remains outstanding. A sweep found 270 independent Cargo workspaces lacking prototype; repository-owned native profile configuration is being centralized instead of duplicating profile tables across those manifests.

## Mode-Neutral Driver API

The neutral native component driver now uses NativeBuild type and function names, including NativeBuildPlanV1 and runNativeBuildDriverV1. Warm/fresh generation selection uses SEMIO_NATIVE_BUILD_GENERATION, and mutable run reports live under the shorter runs cache namespace. All authored consumers, first-party facades, language-neutral fixtures and schema documentation asset references were updated; no compatibility exports or old generation environment aliases remain. The root release workflow now merges its original invocation cancellation signal with its local termination controller. The actual release workflow name and optimization profiles retain their delivery meaning. Regression validation is pending.

Touched files for this cleanup:
- `README.md`
- `📜️script.ts`
- `🏢️semio-tech/🎡️play/🔨️modules/📦️site/🆕️fresh-build/🟦️.ts`
- `🏢️semio-tech/🎡️play/🔨️modules/📦️site/📜️script.ts`
- `🏢️semio-tech/🎡️play/🧪️tests/🧪️playfreshbuild/🧫️fixtures/🔣️.json`
- `🧰️framework/📦️packages/🟦️typescript/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/📖️readme.md`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧪️tests/🧫️fixtures/🧪️ooxml-readme-document/🏷️.xml`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧩️native-prerequisites/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🗂️schema-document-index.json`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🟦️.ts`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🧬️schema/🔣️.json`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🧪️tests/🟦️.ts`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🧪️tests/🧫️fixtures/🔣️.json`
- `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/🧩️component-pass/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🧩️driver-stages/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🧩️component-stage/🟦️.ts`

## Cache And Lock Correctness Regressions

An actual opposite-arrival compiler-state fixture reproduced a host/state lock cycle: an independent caller held mutable state while awaiting host capacity, while an inherited host holder awaited that state. Web corrected the neutral helper to acquire host capacity before state, removed unused outer state ownership from stateless artifact/composition compilation and moved test consumers into private snapshots. The same runtime regression then completed both graphs; permanent web fixture passed 22.4s. Concurrent CSV package tests passed in prototype and release, 18 tests/191 assertions per mode (11.9s/9.8s).

Native profile fixture now verifies config-only profiles in an independent manifest, actual debug assertions true/false in prototype/release, and accepted automatic rustc frontend thread selection. Installed @iarna/toml independently agrees with profile configuration. Actual native mode suite passed 11 tests after central configuration. The pinned Rust source establishes that frontend workers obey inherited jobserver tokens, so automatic thread hints do not create an independent CPU allowance.

Tooling found an actual empty native source inventory for the representative base64 crate and repaired finite positive selection plus per-exclusion matching; presence validation is green. Independent real rustc dep-info validation remains in progress. Shared Bun download caching exposed a second correctness defect: a new tooling installation could contain an older patch under the same patch filename despite matching copied recipe bytes. Content-addressed patch paths and applied-byte readiness validation are being implemented; inherited release routing has not yet been accepted after this correction.

The schema documentation index now points to the neutral NativeBuildPlan schema identity. Two unrelated static README parser payloads were restored after incidental progress-tag changes so their archived literal content remains intact. Current README documents the new progress tag.

## Compiler Admission Integration and State Ordering

The current native driver now receives complete original caller control rather than a bare cancellation signal. The Cargo-state regression admits an unlaned same-profile job and pass through the neutral JSON schema/Ajv oracle. With one token, the original state-before-token pass acquisition stalls until the caller cancels; the controlled physical red run failed after 5.13 seconds. The implementation now acquires the jobserver token before the Cargo mutable-state lease and releases them in reverse order. The job independently probes the held SQLite lease using Python sqlite3. The green runtime run is pending graph construction.

Observed remaining disk space was approximately 1.3 GiB. Full compiler graph verification awaits capacity and safe profile-locked compaction. Broad age-based cache cleanup does not protect active builds and is not being used. The narrow superseded finalized-session compaction must use the same current lease namespace as Cargo builders.

The controlled Cargo state-order green run passed: one runtime test, four assertions, 1.84 seconds; Python sqlite3 independently reported the lease held. The finite-caller-budget red run failed after 4.21 seconds with the safety cancellation instead of the original 750 ms budget. The native driver now binds the remaining original budget to a timer before admission; its merged cancellation reaches unit-graph children, worker-token waits, mutable-state admission and compiler trees. Storybook child budgets now use the remaining admission grant after waiting. Focused runtime green and broader driver checks are pending.

## Build Helper Dependency Layering

Removed the unused framework-wide ephemeralBox import from the repository library. The actual playground port cache now imports the explicit neutral kernel transient-state facade; resolving a build helper no longer needs the unrelated framework UI/runtime barrel for one process-local box. New facade: `🧰️framework/🔨️modules/🎠️kernel/📦️packages/🟦️typescript/🫧️transient/🟦️.ts`; updated playground owner: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎮️playground/🟦️.ts`. Runtime/source-closure verification is delegated to the current actual leaf runs.

## Final Native Control and Ordering Runtime Gate

Actual canonical `@semio-tech/framework-process:test-native-queued-cancellation` passed on current sources: six tests, 56 assertions, 4.44 seconds, Nx task duration 6.6 seconds. It covers schema admission/Ajv, queued exclusive cancellation with no unhandled rejection, metadata child cancellation and successor admission, exact profile modes, finite original 750 ms caller deadline, and the same-profile unlaned job/pass ordering with one worker token and Python SQLite oracle. Deadline cancellation exposed an ignored derived provenance promise rejection; the driver now observes that rejection while retaining it for actual stage consumers. The original caller's signal stays alive when its phase deadline expires.

Current edits include the native component-pass provider/tests/neutral fixture, the existing process test owner (focused gate selects the new regressions), and Storybook's root owner clipping its post-admission child budget. No new executable command is added by these two regressions; their existing gate is already registered in both launch files.

The full current component-pass and jobserver suite also passed through Nx: 26 tests, one skipped GNU make >=4.4 fixture (system version unavailable), 254 assertions, 21.79 seconds. This includes real wasm32-wasip2 components in both profiles, streaming stages, cancellation, token sharing and the third-party Rust jobserver client oracle.

Registered repo:test-graph-seal in both launch registries using localized JSONC-aware insertion; existing formatting, trailing commas, and concurrent configurations preserved. Removed only the completed full prototype/release/pruned generated JSON graph exports after their owner confirmed no current consumer and durable counts were retained. Shared compiler stores are untouched. A status census found 3104 closed tickets; only one had a generated directory, 39 MiB, so this does not provide useful full compilation headroom.

## Development Rebuild Admission

The Next development compiler owner now groups overlapping client/server/edge compiler phases under one host claim using public Webpack beforeCompile/done/failed/watchClose hooks. It releases the claim between rebuilds and cancels queued closing phases without aborting the original caller. The Node child receives the actual original process envelope. A narrow process-envelope facade avoids importing executable Script classes into Node strip-types configuration. Neutral JSON/schema vectors and an installed Webpack compiler oracle are authored; the absent-provider red ran through Nx. First implementation run found the fixture schema URI and the queue path budget; these fixture issues are corrected. Runtime green is pending tool patch repair.

The first actual full workspace prototype attempt failed during pinned Nx provisioning because the concurrently edited patch had invalid hunk counts. No compiler task started and no full build success is claimed. The Nx agent is repairing/verifying the patch before the retry.

The Next development rebuild test is now a canonical package command: `bun ./📜️script.ts test next-rebuild`, Nx target `@semio-tech/repo-coordinator:test-next-rebuild`, registered in both launch registries. Its independent compiler oracle uses the installed pinned Next Webpack SDK. The narrow original-process-envelope facade is `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/📥️invocation/🏃️process/🟦️.ts`. Both the DevScript child environment and next.config.mjs retain actual original process control. The schema document index already includes the new admitted schema through concurrent owner generation; no duplicate index entry was added.

Next development rebuild canonical gate passed on current sources: three tests, eight assertions, 1.90 seconds; Nx task duration8.0seconds. Real pinned Next Webpack compiled two participants under the same claim and returned capacity to an idle successor. Queued phase closure preserved the original caller. Full workspace prototype retry now reached all1213tasks:540nativehash plans, planning3.742seconds,source4.863seconds,nativehash63.801seconds,receipt0.222seconds. Compilation is running; no full build success is claimed.

The inherited/prototype-port Next control fixture physically failed all grouped lifecycle checks when object spreading erased methods. Explicit forwarding now passes all three tests/eight assertions in2.05seconds. The native driver concurrently changed to keep the admission signal exactly original, but lost cancellation of its unit-graph child on a finite phase deadline; the existing physical deadline regression failed at4.069seconds with safety cancellation. The implementation now keeps original admission signal/ports and separately composes cancellation only for owned children/stages/jobserver work, so it can stop metadata without aborting the caller. Its current original-admission/owned-cancellation gate is running.

The full prototype run exposed domain PNG declaration errors (unhandled set-gamma/replace-samples mutations and duplicate PngRegion export), infrastructure manifest node_modules traversal, native shebang placement, stale WASM admission variable, and oversized Bun JSON output transport. Infrastructure owners are fixing these. The own full prototype coordinator was stopped before the canonical command-driver route flip, with shared Cargo warm state retained. The recorded run reached actual compiler leaves, and no full build success is claimed.

After the concurrent native admission rewrite, the unchanged finite-deadline regression again exposed a metadata child that outlived the original grant. The corrected design keeps original signal identity for resource admission and explicitly forwards prototype methods; a separate owned operation cancellation signal stops metadata/children/stages/tokens without aborting that caller. Current gate passed six tests/56assertions in5.70seconds. No original signal identity assertion is weakened.

## Full Build PNG Declaration Blocker

The full prototype compiler run exposed an incomplete declared PNG mutation union and an ambiguous PngRegion export. A language-neutral fixture now covers the existing set-gamma and replace-samples variants against independent RFC6902/fast-json-patch and pngjs codec witnesses. Its physical red returned undefined for an admitted mutation. The implementation now delegates these variants through the existing canonical PNG diff algebra with cancellation checkpoints, and operation/diff exports refer to the same canonical region type. Native schema, fixture and package declaration validation are running; no runtime library was added.

## PNG Full-Build Blocker Verification

The first real prototype workspace graph exposed two schema-declared PNG mutations with no executable branch and a duplicated public region type. The neutral mutation vectors now cover gamma and exact native sample replacement using the existing PNG diff algebra; the independent RFC6902 library and PNG codec agree on both outputs. The physical absent-branch red failed as expected. The current focused gate passed one test, twelve assertions, in 1.014 seconds, with actual debug witnesses for both mutation vectors. The real prototype PNG task then passed, emitting twenty-one declarations and twenty-five owner outputs; Nx reported 11.3 seconds of task execution. Graph preparation and concurrent graph-lock waiting occurred before that task timer, so this is not a launcher latency claim. Two earlier green attempts never reached tests or compilation because unrelated inferred job metadata had an invalid embedded workspaceRoot token; those attempts are recorded as failures rather than passes.

Exact owner paths changed:

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/⚙️operations/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🧬️owned-native-contract/🟦️.test.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🧫️fixtures/🧬️owned-native-samples/🔣️.json`

Command evidence before generated-output cleanup: `png-build-blocker-red.log`, `png-build-blocker-green2.log`, and `png-prototype-build-verified2.log`.
