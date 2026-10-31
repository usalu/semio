# Admitted Source Byte Availability — 2026-10-29

Read-only architecture audit tied to35703's real stale build.closure.source-differs refusal. No Source implementation, guard, scope, deadline or capability was changed. No fresh invocation, capture job, Native work, service or memo mutation was started. Frontend97168 remains the normal fresh repro owner.

## Answer

Authenticated complete Source bytes already exist at the initial compiler and inside the captured stream loaders. They are not exported as a generic complete byte port to the received single-file SDK factory. The SDK carries a partial immutable text DATA interface in addition to hash/length metadata, but that interface cannot supply every executable helper-closure module.

Therefore an SDK factory cannot generally replace its physical reads with an already-available complete admitted-byte interface today. Re-reading changed physical Source and accepting a different digest would weaken the guard; constructing a fresh Original/capture or reaching into another loader's private maps would not preserve the existing admission. No such substitution is proposed.

## Existing Owners And Interfaces

1. Compiler capture: authorizers/TypeScript owns capturedBytes in a private WeakMap keyed by a genuine NxAuthorizerSources object. nxAuthorizerCapturedBytesV1 rejects an unissued object and returns copies of actual bytes. snapshotNxAuthorizerSourcesV1 serializes all those bytes as base64. That snapshot's reader enforces manifest identity, order, exact byte hashes, completeness, relative coordinates and existing size bounds. These are compiler-side objects/data, not an automatically inherited SDK byte callback.

2. Captured Bun stream: closure/Bun loader retains the complete images table inside its loader scope. Its redirected fs/readFileSync/readFile surfaces resolve declared paths to those exact bytes; currentImage checks the actual table binding. Source module reads do not reopen those physical files. The public global __semioCapturedSelectionV1 contains only the physical hash roster and import resolutions, not the images table. The private captured-module/builtin wrappers are loader machinery, not a general SDK byte service.

3. Captured Node stream: closure/Node loader retains complete paths/module bytes and registers fs wrappers/hooks. Its internal semio.captured.node.fs Symbol carries the virtual fs surfaces within that process. It is not a public authority to borrow from an unrelated SDK/worker. No port is gained merely by knowing its Symbol name or metadata.

4. Single-file SDK compiler: snapshot/TypeScript48 defines semioCapturedPhysicalSources and semioCapturedSourceResolutions from the authenticated snapshot and additionally defines semioCapturedPhysicalSourceImages. It compiles executable modules from the same byte table, but does not carry the whole table as an SDK callback or redirect every native node:fs read through a complete captured reader.

5. Partial revision DATA port: revision/MJS validates semioCapturedPhysicalSourceImages against the same admitted roster. Every row supplies a byteLength, optionally text whose UTF8 bytes/length/SHA match the admitted hash. capturePhysicalSourceTextV1 and captureBoundedSourceRevisionV1 can return that text without physical reread when present and within the reducing byte bound. When text is absent, retained-text reads fall back to physical bytes and enforce the admitted SHA. Hash-only observations may use length metadata; that is not recovery of missing Source bytes.

The SDK compiler's capturedSourceImagesV1 selection is deliberately selective: imported modules and the entry usually get length only; unimported UTF8 resources or explicitly named data reads may retain text. The literal named-data discovery is bounded at64KiB. It cannot be assumed to include all transitive Native helper modules.

## Actual Failed SDK Four Rows

Bounded inspection of the actual5,910,015byte semio-authorizers.mjs derivative named by the failed run found one image record per row:

| Changed admitted row | Embedded byte length | Text available |
|---|---:|---|
| runtime/node/native/TypeScript |1554|No|
| schema/validator/TypeScript |18177|Yes|
| caching/suites/JSON |49913|No|
| graph/generations/pin/schema/JSON |6477|No|

The evidence is failed-sdk-admitted-image-availability.json, derived from the exact bundle in failed-sdk-source-mismatches.json. This is a bounded metadata observation, not execution or a new authority. It does not print retained Source text. The partial revision port could cover the validator row, but cannot cover the other three from that bundle. Changing only sourceBytes to call that port would still encounter physical fallback for those rows.

## Current Admitted Factory Contract

closure/TypeScript19–22 selects an admitted hash row, reads via its imported fs surface, then rejects bytes differing from that exact digest. captureBuildSourceClosureV1 additionally requires the real issued Original, an exact complete parent resolution selection, reducing closure/static scope, path/import/bounds checks and the expected per-row hash. requireAdmittedBuildSourceClosureV1 validates the resulting child against the admitted parent.

Inside a stream loader, imported fs is the private immutable reader, so the same function reads original admitted bytes. The existing actual subclosure test explicitly rewrites a physical helper after admission and checks that another nested capture remains immutable. This audit read that test's Source; it did not rerun it.

Inside the ordinary single-file SDK, the native fs read is physical unless a specifically supported captured-reader context exists. Thus the observed edits correctly fail the present guard. The compiler-role factory can capture initial current Source before effects; the admitted-role factory must stay inside its existing parent's selection. Switching roles or recapturing on refusal would change that authority boundary.

A future complete immutable-byte capability would need the actual admitted byte ownership, full hash/root/import-map binding, Original/cancellation lifetime, reducing bounds and existing no-source-substitution rules carried coherently into the SDK. The existing partial DATA port is insufficient proof of that capability. This is an architecture boundary, not an implementation proposal or authorization to relax physical verification.

## Anchors

- closure/🟦️.ts19–22,31–42,55–63: admitted reads, Original and resolution/closure checks.
- authorizers/🟦️.ts19,23: private byte table and issued-object read.
- authorizers/snapshot/🟦️.ts20–29,40–50: all-byte snapshot validation, selective images and SDK compile.
- revision/🟨️.mjs16–31,47–49: captured image admission and physical fallback; public declarations in its adjacent d.mts.
- closure/Bun/🟦️.ts capturedRead/currentImage, capturedFs and __semioCapturedSelectionV1: private full-image reader versus global metadata.
- closure/Node/🟨️.mjs9–12: internal fs wrappers/hooks.
- process/capture/entry/🟦️.ts30–41: captured runtime command inputs and genuine descendant/cleanup tokens, a later command capability rather than an SDK-wide Source-byte reader.

No production files were edited for this report.
