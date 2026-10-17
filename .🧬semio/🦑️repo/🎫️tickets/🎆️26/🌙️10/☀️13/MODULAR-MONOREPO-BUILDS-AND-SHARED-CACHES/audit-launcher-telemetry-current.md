# Launcher and Telemetry Audit

No actionable finding in the reviewed compiler-to-consumer and telemetry lifetime cut.

Read-only audit of `compiler-capacity-launcher-consumption-current.md`, SHA256 `1d7ed9ab4dc8ef674e2a0cd792187c218aa0349f91b01c03ca25baca80a92189`. Root and Repo AGENTS were read. No test, compiler, Native, platform sampler, browser, service, cache, lease or modifying Git operation was run. Only this audit Markdown was written.

## Source Trace

- Core capacity/scope/consumption (`🏃️process/🏗️build/🎟️capacity/🪆️scope/🤲️consumption/🟦️.ts:4`) awaits the real capacity scope before invoking the continuation. The capacity owner (`🪆️scope/🟦️.ts:24-34`) drains accepted children before releasing its lease. The public capacity facade reexports that same scope at `🎟️capacity/🟦️.ts:81`; issued-operation facades resolve to the same private `controls` ownership map in `🧭️routing/📥️invocation/🏃️process/🟦️.ts:76-104`.
- Actual Repo Cargo launcher (`🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🚪️launcher/🟦️.ts:102-106`) snapshots the request, environment and capture callback before store entry. Lines 115 and 128-145 derive the real reservation namespace and retain writer/profile custody around the complete CPU-to-consumer operation. Lines 122-126 request the consumer from the retained input control; line 140 returns immutable terminal DATA plus the continuation. Lines 142-143 join the sampler and close broker/jobserver before CPU settlement. Input/store custody encloses that entire operation at lines 113 and 151-160. The consumer environment is the input environment, before compiler-only broker/jobserver additions.
- Actual sampling (`📦️artifacts/🏗️native-build/📈️telemetry/⏱️sampling/🟦️.ts:6-24`) has one pending operation, a finite sample ceiling, coalesced ticks and stop/join. Timer admission is inside try/catch, including synchronous refusal. Separate failed/failure state preserves undefined. Timeline awaits its onRow callback (`📈️telemetry/🟦️.ts:89-98`), and platform capture is bounded/owned at lines 65-77. Linux descriptors are reserved before open and reads use a fixed maximum+1 buffer.
- Native nonRustc capture is owned and bounded (`🧩️component-pass/🟦️.ts:316-321`). Timeline construction failure closes existing broker/jobserver at lines 390-396. Stall sampling requests actual CPU descendants and fixed bounded sample captures at lines 457-466. Disposal closes births, joins stall work and pending driver work, then joins timeline and closes broker/jobserver at lines 470-471. Native stop records a boolean plus exact unknown first reason at lines 362-364.

## Evidence and Limits

All 16 Source hashes (15 own plus shared wrapper) and all 22 retained log hashes matched at the start and again after source inspection. No Source drift occurred in this audit. The shared wrapper remains SHA256 `716a32f608138a8f179fb9656a02bbbee132eb5634de14e00fbd9840d8a9ecca`; its documented disjoint Rust warm-input selector coedit is preserved.

The actual `SEMIO_CAPACITY_INHERITANCE_ONLY` strict branch contains 51 explicit distinct roots plus the wrapper itself, exactly 52. Strict, noUncheckedIndexedAccess, ES2022 and react-jsx remain present. Both retained final Source logs contain three isolated status=0 files with 66+109+82=257 assertions and 12 tests. Both final strict logs are empty. Terminal exit codes and handle closure are collector provenance recorded in the handback; empty logs alone do not independently prove exit status.

The ordinary Node/SQLite evidence is genuine kernel evidence, not Native execution: retained output shows compile busy, consumer CPU free, input/output held through descendant drainage, retired CPU refusal, real reacquisition, exact bytes and final release. Negative child scripts actually fail their received Originals, emit bounded terminal DATA and set nonzero exitCode; they do not catch a failed Original and reuse it as successful authority. Sampler birth-close, cancellation, Error and undefined paths are present in the actual child script. Existing Native-positive fixtures remain source/type evidence only.

No physical platform command, actual Cargo compiler, Native build, product runtime or cross-platform execution was independently qualified. This audit does not broaden the existing persistent-cache/product HOLD. The separately prioritized HIGH Nx / Python producer work is outside this cut.
