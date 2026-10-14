# Generic Cargo Command Custody Audit

Static-only review; no tests, imports, issuer/factory calls, capture execution or native processes. Current findings sent immediately to Root and HighScope. Concurrent implementation can supersede these hashes.

General Process path/hash snapshots:

- `🧭️routing/🎛️command/🎟️custody/🟦️.ts`: `e62b0c0fdae743ed6c727b7836e4b5a1459ffbf9c7fdd3eab09e7c467ae46cc3`.
- `🧭️routing/🎛️command/⚙️configuration/🟦️.ts`: `0a79d758d99daa7804a0581f396fe57fce61b0de90cbfe295092b317c592aa11`.
- `🧭️routing/🎛️command/🟦️.ts`: `e5e4eb9a8b08df0ce5d5407f3d109369fac8ea9b44399a1001eaac999da11251`.
- `📥️capture/🟦️.ts`: `e1d3ca0e05024317be4bb2b041d8400a7bd7f56a67b90fc2ef81538c6008adb5`.

## Exact Child Selection Is Not Bound

Custody line11 verifies backend tests kind, absolute backend cwd, selected cwd/manifest and request argument suffix. It does not bind `request.command` or its fixed argv/action prefix. When selected test arguments are empty, the suffix condition is vacuous: any command/args with the selected cwd/manifest obtains inherited Cargo policy. Nonempty selection arguments still allow an arbitrary prefix. The command request schema allows arbitrary nonempty commands; policy admission does not add the missing binding.

`executeCommandV1` line41 admits that inherited policy with null wrapper Cargo artifact policy; line58 subsequently captures the requested arbitrary command. Backend endpoint validation happens only if that child eventually invokes the backend, so it cannot guard this earlier spawn. The seam must establish the exact original selected child program, script/action and fixed arguments as well as its forwarded tail. Retaining immutable snapshots of an initially unbound request does not supply that binding.

## Capture Execution Inputs Remain Mutable Across Publication

Capture lines28–29 retain invocation policy/capability/control/signal/port identity. However, the public capture API retains caller-owned `options` and `args` across awaited publication at line44 without snapshot/check. `remaining()` reads live `options.budgetMs`; spawn reads live args/cwd/env; output accounting reads live `options.maxOutputBytes`, and output path reads happen after publication. A caller publication callback holding these original objects can increase phase/output limits or change execution inputs before the first spawn. The original parent deadline still bounds total execution, but the original narrower phase/output grants can be widened.

`executeCommandV1` creates fresh private capture args/options, reducing exposure in that specific caller; this finding applies to the public capture API and any caller retaining its input objects. Snapshot/pin execution inputs and capture policy before awaited callbacks, then reject mutation before effect or consistently execute the immutable snapshot. No runtime exploit or test result is asserted.

## Preserved Behavior Observed

Configuration requires original invocation and recognizes an explicitly present backend rather than inventing one. Its inherited branch retains complete inherited Cargo policy, target/build environment and null wrapper artifact authority. Command execution retains request/policy/environment and original invocation identity across preparation, progress and capture. Capture uses the saved original signal and clock through its existing timeout/cancellation supervisor. These improvements do not close the two specific gaps above.

## Correction Follow-Up

Static snapshots: custody `8b04b8d8879afa45607e9b8f8ffc823c669878f39edd0f1a111e58797f2f93b3`; General execution contract `e6006fda02353d74ac97a979736f7257feaef00612f999f93bbb9992dde13e9a`; capture `89e2f612fdb91c721251d0bf3d1697f0a0384eda4db4ea9eaea2c3c4bbb5f8eb`.

The General declaration now requires explicit continuation command or null for every operation; the backend requires an exact resolved continuation or null. The selector carries only the authored continuation and declared forwarded tail. Repo issuer resolves its cwd under the original root and maps authored Bun to the actual executable. Generic custody now requires a non-null continuation and compares request program, cwd and full argv exactly, as well as selected manifest/cwd. This addresses the original arbitrary program/prefix gap. Closed schema objects require all continuation fields and reject extra fields. Direct operations declare null and cannot borrow Generic wrapper custody.

The public reader now returns the definition-keyed CargoExecutionContractsV1 type rather than caller-chosen arbitrary T. Capture snapshots argv, cwd, environment, phase/output limits and output paths before publication; all actual remaining-time, spawn, output and evidence reads use the captured execution image. Retention compares caller inputs with that image around awaited publication. This addresses the previously observed mutable capture-input gap.

Cancellation and cleanup branch review: already canceled original input returns a cancellation result; publication interruption catches CaptureInterrupted and returns canceled/timeout result before spawn. A phase exhausted after publication returns timeout. A signal canceled after output files open closes both descriptors before returning. For a running child, close clears timer/progress/listener and closes descriptors before awaiting/rejecting publication guard failure. Capture uses original saved signal and remaining-time callback throughout. Generic executeCommand keeps acquired preparation in its outer finally, so a retention guard failure after preparation or later awaited callbacks still releases it. No new confirmed correction regression was found in this bounded static review. These are branch observations, not runtime receipt/cleanup proof; no tests/imports/capture/issuer/native execution was performed.

## Final Freeze Review Pending

Runtime owner reported endpoint freeze: API `766b399ee9e56a03bd4e55a021f55e6ae516757c5b48dd63a432913cf9e77d33`, private script `3c33540e58484ffeabca0b9fa89c95cff923ff2147214f589fce8ffb20660c09`. Current lexical inspection confirms updated closed reader/operation API usage and exact original descriptor including continuation revalidation. The private command checks exact execute argv before input and delegates only to the bounded input reader/runner.

Scope source/metadata freeze has not yet been received. A pending catalog checkpoint was sent to Scope: execution scope row at catalog line7717 still records old schema hash `3d2b7db1...` and omits the new CargoExecutionContinuationV1 export, whereas current Runtime handshake schema observation is `62a23334...`. These are in-progress metadata fragments; no final current acceptance is asserted until the owner completes freeze and this reviewer checks current bytes. Historical audit hashes above are deliberately preserved as historical findings, not presented as final current source authority. No probes or native execution occurred.

## Native Receiver and Source Registration Integration Checkpoint

Specific NativeScript snapshot `4bfbf08426a70a4b4172a8aa8052478afff0ca7536c489bc186d1128f56178bc`; Specific caching policy `e7f1e6099b2cf6461c1eb96a94ad24cc47d29a66b4e9402bf76afd39a5ce9438`.

NativeScript owner-command branch retains exact original policy/capability/control/signal/port identities and serialized authority at line61, with requireRepositoryCargoExecutionV1 when a backend is present. It calls retain before selected-owner publication (63), after awaited publication (65), immediately before runOwnedCommand (83), in progress callbacks, and after execution (85). Between publication and spawn, preparation is only synchronous descriptor construction; no awaited actual preparation was observed requiring an additional bracket. The runtime API enters native import closure through this Specific import. No General-to-Specific edge was introduced by that import.

Specific caching policy toolchains.cargo.files explicitly lists the private execution endpoint script at line117. Inference nativeCommandInputs lines538–548 includes all cargo.files and the existing native router executable import closure, so both the API import and endpoint file have defining Source-input paths. This is a static registration trace, not successful actual Source capture evidence.

Canonical target script discovery remains under RuntimeHigh repair (outer command plus explicit continuation versus old all-argv script existence/outer-cwd heuristics). Scope confirms no overlapping edit there. Final catalog and actual Source-roster probe remain pending; this checkpoint does not assert final current acceptance or revive historical hashes as current proof. No imports, probes, issuer, Store, native or test execution occurred.
