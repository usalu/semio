# Artifact Verification R4

## JavaScript Grid3d Coverage

The registered `@semio-tech/wfc-js:test` selector discovers schema suites at `🗿️artifacts/**/🧬️schema/🧪️tests/🧩️suite/🟦️.ts`. Grid3d previously had no matching suite. Added its suite at `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🧩️suite/🟦️.ts`.

It discovers all 14 declared mutation kinds and their language-neutral before/after vectors; Ajv independently validates each of 28 snapshots against the published schema and framework reference schemas. The suite also checks axis lengths, cell bounds and tile references. Targeted `repo tools exec --light bun --lang=en -- test ./<suite>` passed 15 tests and 281 assertions. The registered project gate remains separately required.

## Current Gates

Started the registered WFC JavaScript test with cache disabled and all five distinct workspace `repo cargo` native `--workspace --lib --locked` tests through repository MCP operations. They await terminal results; no project or native pass is yet claimed. Selectors are recorded in `run-r2-native.md`. Shared framework and core gates are owned separately by native_verify.

Bitmap's first native attempt failed during lock preparation: its pin-solution manifest inherited nonexistent `semio-framework-plugin` and `semio-framework-os-kernel` workspace dependencies. Aligned that component's manifest, library aliases, implementation and unit tests with the current workspace names `semio-plugin` and `semio-os-kernel`. A fresh bitmap gate was started after the first returned; its result is pending. No bitmap test ran on the first attempt.

Registered JS MCP call timed out at its 300-second transport limit; it provided no test verdict. Subsequent current fleet and process command-line scan found no surviving WFC JS process. Restarted once as `repo run @semio-tech/wfc-js:test --skip-cache --lang=en`, persistent handle 91749; capacity queued, no verdict.

Current observation handles: bitmap CLI session `68977`; four workspace calls for grid2d, 2d, grid3d and 3d share MCP cell `g31:38` (results returned together); registered JavaScript CLI session `91749`. Each is pending. The first bitmap MCP cell `g31:34` returned the manifest failure above, and original JS MCP cell `g31:35` returned the timeout above. Native verifier subsequently fixed shared Rust module documentation ordering; a native gate that captured earlier shared source may need recapture if it reports that error.

An independent workspace manifest audit found the same stale inherited names in the bitmap root package. Removed obsolete duplicate entries from `🖼️bitmap/📦️packages/🦀️rust/Cargo.toml` and changed seven bitmap Rust implementation/test files still referring to the old crate names. No runtime semantic changes were made.

Bitmap retry `g31:47` returned exit 1 during lock preparation because the shared WFC engine oracle manifest inherited the obsolete `semio-framework-os-kernel` name. Native verifier removed that stale duplicate. Started a fresh bitmap CLI session `68977` after the repairs; pending. No bitmap compiler or tests ran on either failed attempt.

Contract audit identified the same stale identifiers in 17 grid2d, 2d, grid3d and 3d Rust files. Aligned references to `semio_plugin` and `semio_os_kernel`; verified the plugin package's implicit library name and the kernel's explicit `[lib].name`, and confirmed artifact manifests use the new dependency names. Actual API typechecking remains pending. In-flight native workspace snapshots may precede these changes and must be interpreted accordingly.

Latest process scan: bitmap CLI session `68977`, PID 3245002, active; 3d MCP agent `artifact-3`, PID 3250809, queued for capacity. No grid2d, 2d or grid3d command process found. Their individual MCP outcomes remain hidden until cell `g31:38` completes all four calls, so no duplicate replacements were started. Registered JS CLI `91749` remains pending.

JS session `91749` subsequently admitted and verified 449957 graph facts, then entered input closure verification (0/2141 at the observed point); no test verdict yet. Bitmap `68977` remains capacity queued.

MCP cell `g31:38` returned: grid2d and 2d failed during lock preparation due to the shared oracle's obsolete workspace dependency (now repaired); grid3d and 3d hit the MCP 300-second observation timeout without verdict. No compiler or tests ran on the two manifest failures. A subsequent process scan showed no surviving four workspace commands. Restarted through persistent CLI sessions grid2d `17226`, 2d `31990`, grid3d `45702`, 3d `58150`. All remain pending through normal admission. Bitmap `68977` and registered JS `91749` are also pending. These session IDs supersede the earlier MCP observation cells.

Latest poll: all five native sessions wait for capacity; JS verification advanced to 32/2141 closures. No native or project test verdict. A scoped `repo tools exec --light git -- diff --check` for WFC artifacts and this report passed.

## Bridge Selection

Five artifact mutation bridges are independent Cargo workspaces and excluded by the artifact workspace `--lib` gates. Each declares a binary target and includes two bridge-owned unit tests. After the corresponding library gates, select each with `repo cargo --store=shared --configuration=prototype --parallelism=1 --lang=en -- test --manifest-path '<artifact>/🏭️bridge/📦️packages/🦀️rust/Cargo.toml' --bin semio-wfc-<bitmap|grid2d|2d|grid3d|3d>-mutation-bridge --locked -- --test-threads=1`. These commands are prepared, not yet run. The plugin-level `🏭️bridge/Cargo.toml` is a separate binary with no unit tests; its runtime inventory is another independent check. Current WFC JSON scan finds no NUL-containing JSON files; the prior bitmap preview NUL report is stale.

The bridge tests compare production descriptor inventories against portable coordinate fixtures using serde_json and cover sibling-coordinate refusal and cancellation; these are binary unit tests rather than an end-to-end subprocess invocation. The five exact binary names and manifest locations were re-read from their current Cargo manifests.

Latest session poll: five native workspace commands remain in capacity admission; JavaScript input closure verification reached 82/2141. No further verdict.

Next poll: all five original native sessions remain in capacity admission; JS input closures reached 110/2141. Parent cancelled its own pre-UI browser run to free its grant; browser acceptance remains required separately, as recorded in `run-r5-coordination.md`. No WFC test verdict has appeared.

Later poll: the five original native sessions remain queued; registered JS session `91749` progressed to 171/2141 closures. No test verdict.

Most recent poll: all original native sessions remain capacity queued, bitmap at the front; JS verification reached 208/2141 closures. Parent also deferred its own hub gate before tests to prioritize native feedback. Neither deferred browser nor hub is an acceptance pass.

Subsequently bitmap `68977` and 2d `31990` admitted and entered source preparation; bitmap reached cargo build-entry resource retirement inventory. Grid2d `17226`, grid3d `45702` and 3d `58150` still wait on capacity. JS `91749` advanced to 228/2141 closures. No compiler/test verdict.

Bitmap `68977` and 2d `31990` subsequently terminated exit 1 during build entry. The polling response truncated the diagnostic before it could be retained; the cause is unknown and no test pass is claimed. Restarted those two terminal commands with full ticket-generated logs: bitmap `76442`, 2d `30712`. These supersede the former sessions. Remaining three native sessions and JS continue unchanged.

Grid2d `17226` and 3d `58150` subsequently entered source preparation; grid3d `45702` and the bitmap/2d logged retries wait on capacity. JS verification reached 272/2141 closures. No compiler/test verdict.

Grid2d `17226` and 3d `58150` both returned exit 1 during native Source admission, before compilation, at shared `semio-plugin-instance-app` unit 397. The shared `📥️staging/🧬️composer-api/🦀️.rs` had 13986 bytes against 13872 remaining in the crate allowance. Reported the shared failure to parent and native owner. Grid2d lock refresh completed before this refusal. Do not rerun these two until the shared source owner resolves the admission failure; neither artifact's tests ran.

Grid3d `45702` returned the same shared Source admission failure before compilation. Investigation confirms the crate declares 262144 bytes and the per-unit reader counts distinct source files cumulatively; the current root plus recursively referenced source files originally totaled approximately 266258 bytes. Extracted host-event original scope, cancellation and route handling into field-borrowed mutable and read-only contexts owned by the existing dispatch component. The app retains pending-work aggregation and borrows the same owners; no limits changed. A portable JSON decoder-identity scope regression and serde_json test were authored before extraction. Recursive `#[path]` scan after extraction finds 17 app source files totaling 257943 bytes, subject to native admission authority and concurrent source changes. Dispatch library test session `42282` is pending. Parent handles app-wide current dependency-identifier alignment. Native checks must still verify the extraction and all five artifacts; the source count is not a compile/test pass.

Parent aligned app source dependency identifiers and removed an unintended `#[cfg(test)]` on `VcsArtifactApp`. After that source change, restarted the four terminal artifact workspace checks with retained generated logs: grid2d `77291`, 2d `42422`, grid3d `73940`, 3d `9469`. Bitmap `76442` remains the original live retry. JS `91749` and dispatch `42282` remain live. These six native sessions and the JS project test have no verdict yet. Dispatch test setup now explicitly enables job testkit for the original caller/decoder regression; the regression retires both decoder owners and the caller.

Auditor identified three stale crate identifiers in the dispatch source closure; aligned only those to dependency keys and verified their package/library names in the manifests. Bitmap and dispatch native sessions remain in capacity admission, as do all four artifact retries. JS reached 454/2141 input closures. No native pass claimed.

Bitmap `76442` subsequently completed Source admission for `semio-plugin-instance-app` (unit 397/528) and entered actual Rust compilation. This establishes that the admitted closure fit its boundary for that run, not that it typechecks or passes tests. Dispatch and four other artifact tests still await their own results.

The independent dispatch manifest audit found no remaining undeclared old plugin or kernel identifiers in its non-test source. Latest JS progress is 539/2141 input closures; bitmap continues compiling foundational crates without a terminal verdict.

Bitmap `76442` then exited 1 in actual compilation of `semio-ui-contract`, before WFC tests: `NativeReceive` derives in `📃️document/🦀️.rs` lacked the facade attribute and generated references to the absent direct `semio_framework_value_native_owned` crate; patch receive also named `OriginalReceivingPrimary` at an obsolete facade path. Aligned the four derives with the existing UI facade convention and the receiver type with its actual `native_decoding` reexport, without a dependency. Started bitmap `97887` with complete `🗑️generated/bitmap-native-r5.log`; the repair and all WFC tests remain unverified until actual terminal results.

Independent static UI contract/presentation scan found no other actual `NativeReceive` derive missing the value facade and no remaining invalid direct `OriginalReceivingPrimary` path. This is source inspection only. Latest registered JS closure verification reached 636/2141; all six native commands remain pending in capacity admission.

Dispatch test `42282` returned exit 1 before compilation: `semio-os-vcs` Source admission reached its 262144-byte limit, with 220 bytes remaining for a 2536-byte text VCS source and two files remaining. The regression needs only the repository's existing `semio-framework-job-testing-caller`; replaced the broader job `testkit` dev feature with that exact test-only dependency while retaining the same test. No Source policy was changed or bypassed. Dispatch retry `79144` retains full output in `🗑️generated/host-routing-r5.log`; whether its dependency closure admits remains to be observed.

3d `9469` then failed before compilation because the new dispatch test dependency inherited the caller from a workspace that did not declare it. Corrected the dependency to the actual local caller package via its relative path; `serde_json` workspace inheritance was independently checked against the actual monorepo root. Restarted 3d as `42289` with `🗑️generated/3d-native-r6.log`. Other existing commands remain intact; any that captured the earlier manifest may require a retry after terminal failure.

Grid3d `73940` also returned exit 1 at the same `semio-os-vcs` byte-boundary refusal as dispatch `42282`, showing this VCS source issue affects artifact selection as well; no grid3d test ran. VCS component extraction is now owned by `vcs_boundary`; this slice made no VCS source edit. Grid3d must be rerun after that repair. Latest JS closure progress 822/2141; bitmap `97887`, dispatch `79144`, grid2d `77291`, 2d `42422`, 3d `42289` and JS `91749` remain active.

Grid2d `77291` and 2d `42422` subsequently completed Source admission for `semio-os-vcs` and entered Rust compilation; their source or feature closures may differ from grid3d's refused closure. 2d has reached compilation of UI contract; neither test run has a terminal verdict. JS reached 832/2141 closures.

Grid2d `77291` subsequently returned exit 1 compiling `semio-kernel-identity`: NativeReceive derives emitted unresolved direct `semio_framework_value_native_owned` references. The six affected type declarations lacked the existing value facade attribute; added it using the package's declared value dependency. No tests ran; grid2d must be rerun after shared VCS and kernel source settle. 2d remains active and may have captured older source.

2d `42422` returned exit 1 in actual `semio-os-vcs` compilation: `🗃️history-ledger/♻️retirement/🦀️.rs:26` calls `.map` on `RetainedWorkOutcome`, which is not an iterator or `Result`. Forwarded the exact diagnostic to `vcs_boundary`; no 2d tests ran. Full output is in `🗑️generated/2d-native-r5.log` until ticket cleanup.

VCS owner reports current retirement source already changed by a concurrent edit to use `tally`; its correctness remains pending their native check. Do not claim the earlier failed snapshot tested current source.

VCS owner subsequently landed identity extraction and started its own native gate `34450`; active bitmap `97887`, dispatch `79144` and 3d `42289` are still awaiting admission, so their source capture can assess the new closure. Grid2d, 2d and grid3d remain terminal failures requiring rerun after shared source verification. JS `91749` advanced to 897/2141 closures; no WFC test verdict yet.

Bitmap `97887` subsequently completed Source admission of the current `semio-os-vcs` unit 373/530, extracted `semio-os-vcs-entity-identity` unit 374/530, and `semio-plugin-instance-app` unit 398/530, then began actual Rust compilation. This confirms the selected closure meets Source limits, not that it typechecks or passes tests. Dispatch `79144` and 3d `42289` remain active; JS verification reached 980/2141.

Latest observed bitmap compilation reached UI contract dependencies without a terminal error; dispatch and 3d remain in capacity admission. JS verified 1052/2141 closures. No test bodies or final verdicts yet.

Bitmap `97887` subsequently returned exit 1 in `semio-os-channel` compilation, after VCS and app Source admission. `📦️archive/🛬️decode/🦀️.rs:19` produces a `NativeSnapshotFailure` outcome inside `owner.receive` instantiated for `ValueError`, causing two E0308 mismatches. Forwarded the diagnostic to native owner; no bitmap test ran. Full diagnostic is retained temporarily in `🗑️generated/bitmap-native-r5.log`.

Aligned archive decode's receiving error parameter with `NativeSnapshotFailure`, mapping the scoped value error through its existing `From<ValueError>` that retains its original cause and progress. The portable archive fixture has an existing independent protobufjs oracle and checks malformed input, allocation refusal and cancellation. Initial focused production library session `85821` does not own that test. The actual test is mounted by `semio-os-channel-tests` companion; started explicit companion selection `80043`, retaining both sessions and their generated logs. No validation verdict yet.

Production library session `85821` completed exit 0, confirming the channel compilation repair; it selected zero tests (two filtered out), so it is not the oracle verdict. Companion `80043` has admitted and is still pending.

Companion `80043` returned exit 1 at test compilation: archive test lacked two explicit failure/outcome imports, and adjacent command fixtures still initialized now-paged payloads as `Vec`. Added the imports and used the existing fallible `PagedBytes::try_from_slice` convention. Restarted the exact companion oracle as `90818`, full log `🗑️generated/channel-archive-companion-r6.log`; no test ran on the failed attempt.

Dispatch retry `79144` and 3d retry `42289` both returned exit 1 before compilation in shared `semio-framework-job-context` Source admission: `📦️owner/🦀️.rs` is 3702 bytes against 3089 remaining, with nine files still in the allowance. The user-requested shared job-context structural repair is assigned to `vcs_boundary`; no dispatch or 3d test ran. Exact temporary logs are `host-routing-r5.log` and `3d-native-r6.log`.

After `vcs_boundary` confirmed job-context clock extraction source-ready, started fresh normal-ingress checks for all terminal artifact and dispatch gates, with generated R7 logs: bitmap `78190`, grid2d `89658`, 2d `99897`, grid3d `75532`, 3d `29101`, dispatch `25558`. Archive companion `90818` and original registered JS `91749` remain live. No R7 verdict yet.

R7 update: 2d native handle 99897 exited 1 before tests on shared retirement `ErasedCursor` (`RetirementStep::Failure` expects `ValueError`, erased close now returns `OriginalRetirementFailure`). Bitmap 78190, grid2d 89658, grid3d 75532, 3d 29101 and host routing 25558 were still live at the subsequent poll.

All remaining R7 native handles 78190/89658/75532/29101/25558 exited 1 before tests on different captured states of the shared retirement error migration: bitmap hit the erased cursor mismatch; grid2d and 3d hit `RetirementStep::Failure` requiring the new typed error; grid3d and dispatch hit original-vector propagation. IO 72254 also exited 1 on the encode typed-failure mismatch. These are not artifact test verdicts.

Registered JS session 91749 exited 1 after graph/project inference. Its unredirected final poll was truncated before the underlying error, so there is no reliable test verdict or diagnosable cause. A fresh registered run with retained output remains required after native prerequisites settle. The targeted grid3d Bun run remains the only completed JS result for the new suite.
