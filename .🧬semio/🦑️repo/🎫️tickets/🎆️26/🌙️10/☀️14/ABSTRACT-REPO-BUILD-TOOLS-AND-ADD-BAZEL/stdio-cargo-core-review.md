# Stdio Cargo Metadata: Core Read-Only Review

Root assigned Core to review High's managed metadata route and final delta while Root owned ALL15. Core runs no runtime/test/producer and edits no production Source. Final verdict remains pending High's actual source-ready handoff.

## Proven Current Bypass and Callers

The baseline artifact-graph module owns `runCaptured`: direct `spawn(command,args)` using ambient PATH/environment, independent signal handlers, a new per-command 120-second timer, and its own output accumulation. `stdioCargoMetadata` invokes it as `cargo metadata --locked --no-deps --format-version 1 --manifest-path <actual workspace manifest>` for every distinct workspace, then deduplicates packages by actual package ID. These are real metadata operations, not compiler executions.

Exactly two helper callers were found: `testStdioArtifactPackageContract` and `testStdioArtifactPackageGraph`. Their permanent entry owners are the two existing `artifact-commands` BundleScript classes, each already owning `this.invocation`. Thus a required original invocation can be threaded through both existing helper functions and metadata without creating another process envelope, ambient fallback or compatibility overload.

## Managed Route Constraints Sent to High

High proposes retaining the genuine tooling store and its sealed `RepositoryRustToolchainV1` generation, running the generation's explicit Cargo program through existing separated bounded owned capture, and consuming JSON within that lifetime. This is a managed sealed-SDK metadata route; it should not be described as executing the fuller repository Cargo launcher's dependency/storage policy. The fuller `launchRepositoryCargoV1` request/capture API is also available if that policy is intended.

The following constraints are grounded in the current public owners:

- Preserve one original invocation from the command through all manifests; no new per-manifest process deadline or envelope.
- Retain generation custody through the complete loop and output parsing; recheck the real generation at boundaries.
- `captureOwnedCommandResultV1` returns status/signal outcomes, so require status zero and signal null before parsing stdout. Keep stderr separate from JSON and finite byte/line limits.
- Tooling `store.control` is generic original admission. `nativeCallerPhaseV1` additionally intersects original remaining time with the original native child maximum and supplies awaited original progress. Use one such phase for the metadata operation, not a new phase per manifest, if the direct SDK route is chosen.
- The original native authority also owns network/artifact projection; do not silently obtain those from an ambient Cargo environment. The phase's existing options project `CARGO_NET_OFFLINE` and actual native artifact directory. Preserve the generation's explicit program/PATH/Rustup/CargoHome projection.
- Preserve the existing metadata argument vector, actual workspace manifest resolution and package-ID deduplication. No fake manifest, renamed package or unowned target output is needed.

These are review constraints, not a claim that High's final implementation or runtime has passed. Core waits for the actual settled delta before a final verdict.

## Landed Helper and Caller Review

The settled helper requires the invocation, creates a single `nativeCallerPhaseV1` at function entry, and awaits Stdio progress before tooling admission and before each manifest. It retains the real tooling store and sealed generation through the complete loop, checks generation custody, invokes the explicit generation Cargo program through separated owned capture, requires zero status/null signal before parsing, checks packages shape, and rechecks generation/phase/original before returning package-ID-deduplicated data. Capture uses `phase.leaseControl`, the phase's original network/artifact environment projection combined with generation SDK environment, and byte/line limits clamped by the actual native transport grant. No fresh original envelope or per-manifest timer remains.

Both permanent command classes pass `this.invocation` into the two helper callers, and the retained Stdio adapter receives its existing original wire before calling the graph helper. No omitted required caller was found in the current census. Source and caller route are coherent for those boundaries.

High reports authentic focused RED zero pass/two fail/five assertions/248 ms before phase refinement (`stdio-managed-runtime-red.log`), then final **two passed/14 Bun assertions/375 ms** with additional Node ownership assertions (`stdio-managed-runtime-final.log`). The final runtime evidence covers real Original cancellation, requested expiry, awaited first publisher refusal and finite owned child output against SQLite/Ajv oracles. Core did not run those tests; actual positive full Cargo metadata remains a later acceptance operation, not inferred from this focus.

## Precise Acquisition-Deadline Boundary

At Root's request, Core traced acquisition separately from capture. `withRepositoryCacheStoreV1` constructs `scriptCompilerAdmissionControlV1(invocation, ..., 0)`: that control intersects the supplied invocation remaining time only. It does not inspect `native.child`. The Rust generation owner creates another native phase after tooling admission; generation custody uses the store control. Therefore the current Stdio entry phase continuously bounds the actual capture and checks around acquisition, but its own start-relative native child ceiling is not automatically imposed on the entire tools/generation acquisition wait. Those waits **do** retain the genuine original signal/deadline. No rebased original process clock is created.

The current declared native child maximum is 3,600,000 ms; the intended standalone acceptance original is 120,000 ms and therefore stricter throughout. No actual 120-second deadline bypass was found or tested. A smaller native grant or longer/unbounded original would distinguish child capture-only semantics from whole-metadata phase semantics. Existing `NativeCallerPhaseV1` documentation mentions awaited queued custody, and the Cargo preparation owner passes its entry phase control into compiler storage explicitly; the generic tooling owner cannot be assumed to do the same automatically.

If the entry native phase is intended to bound all metadata acquisition, the minimal available mechanism is one narrowed invocation view whose `remainingMilliseconds` delegates to that existing phase budget while preserving the exact policy, capabilities, signal, publish and continuation; all SDK owners then see that same nonrenewable remaining bound. This requires no new public abstraction or original envelope. Core reported this concrete control-boundary distinction to Root and High and made no production patch; Root owns the semantic decision. It is not evidence of a currently observed metadata runtime failure.

## Final Read Identities

- `✏️s/🧑‍💻dev/🗄️stdio/🏘️composition/🕸️artifact-graph/🟦️.ts`: `76c0f1bc6ca52181f8056a38af77d595eefb109b59e75a8cc75fb7a15eca319e`.
- `✏️s/🧑‍💻dev/🗄️stdio/🏘️composition/🏃️artifact-commands/🟦️.ts`: `fe1224071f0b54c435131f3a76dd44e5d8f5ca1d69e0321f4fd21ba729679ffc`.
- `✏️s/🧑‍💻dev/🗄️stdio/🧪️tests/📦️artifact-package-graph/🟦️.ts`: `acbfb03b1b560e0d638f978d047043394fd0d93352cac6b79f54904a33417b54`.

## Closed Acquisition Review: Root's Whole-Metadata Contract

Root explicitly selected the whole-metadata native phase contract. High added exactly one narrowed invocation view after the existing entry phase:

`const bounded={...invocation,control:{...invocation.control,remainingMilliseconds:()=>phase.budget()}};`

Tooling admission receives that view. Policy and capabilities remain the identical objects; signal, publish and yieldContinuation remain the identical objects/functions. Only the remaining-budget function changes. It delegates to the already-created metadata entry phase, so it cannot renew the native ceiling, change the original envelope, or introduce another clock. The generic tooling owner now sees this shrinking bound, and SDK-owned later native phases are intersected with it through the stored invocation. The original caller itself remains unmodified.

Capture still uses the same entry phase control, original authority environment/network/artifact projection and transport clamps. The actual generation custody remains active through every manifest execution, zero-status/null-signal checks, JSON parsing, package accumulation and generation/phase/original rechecks. Current permanent caller threading remains complete; no compatibility overload or missing original fallback was added.

High's actual queued-custody RED was **two passed/one failed/15 assertions/5.68 seconds** (`stdio-queued-native-red.log`). The final authentic gate is **three passed/20 assertions/6.41 seconds** (`stdio-queued-native-green.log`), whose terminal tail Core independently read. The queued law holds the actual `repository-cache-charge:<tools-root>` lease, observes the actual matching acquisition publisher while forwarding it, and lets a schema-admitted narrower 5,000 ms native grant expire during that awaited progress. After the patch, the matching publish completes but continuation does not occur; the real deadline refusal propagates. The blocker is released in finally; a fresh real tooling callback subsequently enters and the original signal remains unaborted. The test does not fake a store/image or perform full Cargo metadata.

**Final Source/control/custody review: clean.** The acquisition distinction above is resolved for the chosen contract. Core made no permanent Source edit and ran no runtime; the only authorized code edit was the narrow ticket SDK selector. Full positive Cargo metadata, producer acceptance and final pin qualification remain Root-owned later checks. No further test or Source change was requested by Core.

### Post-Narrowing Source Identities

- `✏️s/🧑‍💻dev/🗄️stdio/🏘️composition/🕸️artifact-graph/🟦️.ts`: `50630f83374f75d72133db20d7a7e8f9bfaedd0d8e475f6707080ceff893ed41`.
- `✏️s/🧑‍💻dev/🗄️stdio/🧪️tests/🏃️command-ownership/🟦️.ts`: `1173befa613ba3e16490d136e30a6ed6fd4a1cfdb4e703728b5fe1fcc68b002c`.
