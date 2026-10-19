# OS DB And Renderer Stash Preservation — 2026-10-16

This finite source-preservation pass completes the assigned DB storage/artifact and renderer residue from the active ticket. It preserves authored incoming mechanisms within current HEAD's extracted source owners. It does not claim compilation, test success, or runtime behavior.

## Three-Way Authority And Scope

Verified current HEAD: `1bae6115bc56bf5e1b01212774dd76877a5f64fe`; actual latest stash: `ec7b3361858df86599da79e5b6dde70ccbbc3625`; stash parent/base: `f856dff081373e131ebf72618120cb73e303a912`. Historical stashes were not restoration authority. All Git operations were read-only.

Exclusive module prefixes agreed directly with the OS executor: `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db` and `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer`. The shared `os-preservation-outcomes.json` was read as locator data and not modified. Base and incoming source spans were re-read with `git show`; current owner source was inspected before each port.

The upstream executor had already restored exact-base additions and nonoverlapping changes. This pass resolved the remaining 100 incoming hunk records plus two deleted-old-owner cases. The full base/incoming hunk evidence is retained in [the hunk ledger](preserve-os-db-renderer-hunks-1016.md). The owner table below records every assigned residue, independently of generated locator data.

## Preserved Mechanisms

DB storage now carries authored original fault custody, fault/poll/cleanup slot custody, operation custody, fair lost-owner maintenance cursor, fixture actor identity construction, and the authored fault-custody source tests. Those mechanisms are located in current `io-core`, `io-registry`, `io-driver`, storage integration-test, and sync-test owners. `DbIoFaultCustody` and its reserve function cross the current extracted crate boundary explicitly. The removed monolithic DB package root was not restored: its authored allocation fixture is mounted from the existing framework trace test allocator by the current storage and sync roots.

Artifact contract changes preserve canonical serialization bounds, `DiagnosticProgress` and retained source accounting, and the split `ColdMutationDiff<Counter>` fixture implementation. Current extracted ownership and existing diagnostics remain.

Renderer preservation includes immutable launch admission, original native I/O source/invocation custody, bounded CreateApp compiler source ownership, original supplied native paths and contiguous component bytes, renderer decode source closure, modified-file observation receipts, frame completion checkout/return, paid original frame recovery and cursor close, and localized source refusal display. Native launch and native I/O are mounted once in current `host-plumbing`; the existing kernel protocol, asset decode and renderer facade consume those authored interfaces. Existing source APIs were widened only where the current extraction crosses crates. The deleted monolithic kernel-runtime root was not restored: its `read_native_component` delta now lives in current `kernel-protocol/📥️lifted/📦️kernel-runtime/🦀️.rs`.

Current HEAD's generic `MailboxRuntime`/`RuntimeHandle<R>` methods and getter-based dispatch are retained. The newly authored AppRuntime-specific queue/presenter orchestration lives in a concrete `RuntimeMailbox<AppRuntime>` implementation in the existing mailbox owner; original recovery remains generic and uses HEAD getters. HEAD's `PrefsState`, explicit `ChromeState` parameter and `PluginsState.plugins` composition are retained while adding authored refusal and modification handling. HEAD's shared extracted renderer worker pool remains: stash deleted its old monolithic copy, not the productive current shared owner.

No new command route was introduced. The existing native launch command now includes authored admission while retaining HEAD's repository runtime-assets/control environment. Source fixture dependencies retain HEAD's removed-font decision.

## Exact Owner Resolution

Resolution counts: **43** `exact-context-port`, **1** `head-extraction-retained`, **56** `semantic-owner-port`. No assigned hunk remains in `needs-semantic-port`.

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/📜️script.ts` — hunk IDs 1. Preserved HEAD runtime assets/control plus authored launch admission
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/🧪️tests/📇️catalog/🟦️.ts` — hunk IDs 2. Source evidence dependency fixture keeps HEAD additions
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🏛️host-base/🦀️.rs` — hunk IDs 29. HEAD extracted productive worker pool remains shared by child crates; deleted monolith not resurrected
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🏠️os-host/🦀️.rs` — hunk IDs 3, 4, 5. Exact stash block with HEAD public/interface names preserved; Preserved HEAD AppMailbox identity
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📐️surface-lane/🦀️.rs` — hunk IDs 6, 7, 8, 9, 10, 11. Exact stash block with HEAD public/interface names preserved; Original resize worker admission; Preserved public surface-lane owner interface
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🔧️host-plumbing/🦀️.rs` — hunk IDs 38, 39. Mount authored native launch and original native I/O once in HEAD host-plumbing, expose existing source APIs to extracted consumers
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/📦️kernel-runtime/🏊️kernel-pool/🦀️.rs` — hunk IDs 12. Exact stash block with HEAD public/interface names preserved
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/📦️kernel-runtime/📬️kernel-replay-client/🦀️.rs` — hunk IDs 13. Exact stash block with HEAD public/interface names preserved
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/📦️kernel-runtime/🧮️kernel-core/🦀️.rs` — hunk IDs 14, 15. Public extracted kernel protocol function retains original supplied path; Exact stash block with HEAD public/interface names preserved
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/📮️mailbox-impl/🦀️.rs` — hunk IDs 17, 18, 19, 20. Completion checkout ledger paid return; Original recovery retains current generic borrow; Paid close before release; Current generic getter and original context
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🔁️asset-decode-jobs/🦀️.rs` — hunk IDs 21, 22, 23, 24, 25, 26, 27, 28. Exact stash block with HEAD public/interface names preserved; Actual decoder session declaration/constructor with public extracted surface
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🖥️frame-presentation/🦀️.rs` — hunk IDs 16. Context through current presenter mailbox
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🛎️frame-actions/🔁️runtime-apply/🦀️.rs` — hunk IDs 30, 31, 32, 33, 34, 35, 36, 37. Function-scoped authored admission and close retain generic MailboxRuntime/RuntimeHandle and HEAD getter dispatch; 
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🧠️app-runtime/🦀️.rs` — hunk IDs 40. Original modified observation receipt mapped through HEAD PluginsState component
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🧾️frame-build/🧮️frame-transaction/🦀️.rs` — hunk IDs 41. 
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧵️frame-job/🦀️.rs` — hunk IDs 42. Exact stash block with HEAD public/interface names preserved
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️wgpu-frame-job-unit/🦀️.rs` — hunk IDs 43. Exact stash block with HEAD public/interface names preserved
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️wgpu-renderer-async-boundary/🦀️.rs` — hunk IDs 44, 45. Retain HEAD split world scene/terrain arguments and authored caller loan
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🎛️prefs/🗨️chrome-overlays-and-tour/🦀️.rs` — hunk IDs 46, 47, 48. Map authored localized source refusal through HEAD PrefsState and explicit ChromeState
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🧪️wgpu-fault-notices/🦀️.rs` — hunk IDs 49. Preserve new source fault field on current fixture
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⌨️cli/🦀️.rs` — hunk IDs 50. Retained db_conflict owner
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🧪️tests/🔬️unit/🦀️.rs` — hunk IDs 51, 52, 53, 54. Retained db_io_driver owner
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/📇️io-registry/🦀️.rs` — hunk IDs 61, 65, 66, 68, 69. ; Fairness cursor remains in current registry
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🚦️io-driver/🦀️.rs` — hunk IDs 62, 63, 64, 67. ; Original reservation in current task-operation owner
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🚦️io-driver/🧪️tests/🔬️db-io-retained-fixtures/🦀️.rs` — hunk IDs 70. 
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🧪️tests/🔬️unit/🦀️.rs` — hunk IDs 71. Preserved authored integration fixture tests; no test execution
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🧱️io-core/🦀️.rs` — hunk IDs 55, 56, 57, 58, 59, 60. ; Current public cross-crate owner fields
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🧬️contract/🦀️.rs` — hunk IDs 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99. 
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🧬️contract/🧪️tests/🔬️unit/🦀️.rs` — hunk IDs 100. Trait split in extracted contract Counter fixture

## Additional Current Owner Wiring

The new launch/I/O source files retained their authored physical locations under `⌨️native-entrypoint/🎟️launch` and `🧊️renderer/🚪️native-io`, mounted by current `🔧️host-plumbing/🦀️.rs`. Current `📡️kernel-protocol/🦀️.rs`, `🦴️asset-decode/🦀️.rs`, `🧊️renderer/🦀️.rs`, and `🧠️kernel-runtime/🦀️.rs` explicitly import or export the preserved source vocabulary. The current `kernel-core/🐣️create-app/🎟️source/🦀️.rs` public boundary reflects its extracted consumers.

Only existing first-party package edges were added to receiving manifests, without dependency resolution or lock regeneration:

| Receiving manifest under renderer `🧑‍🎨engine/🎯️targets/🧊️wgpu` | Added existing package edges |
| --- | --- |
| `🔧️host-plumbing/📦️packages/🦀️rust/Cargo.toml` | `semio-framework-os-services` |
| `📡️kernel-protocol/📦️packages/🦀️rust/Cargo.toml` | `semio-framework-os-renderer-wgpu-host-plumbing`, `semio-framework-job-worker` |
| `🦴️asset-decode/📦️packages/🦀️rust/Cargo.toml` | `semio-framework-os-renderer-wgpu-host-plumbing`, `semio-framework-plugin-ownership` |

Root owns the global Cargo lock reconciliation and launch registration; those edge changes were sent to root. No new runtime library was introduced.

## Validation And Explicit Qualifications

Independent source inventory found **9175** unique HEAD function names, **9259** unique stash function names and **9314** current function names; **no function name present on both sides is absent from current source**. This is a source-loss check, not a behavioral equivalence claim. All **135** current DB/renderer TOML manifests parse, and the Rust source census contains zero merge conflict markers.

All 877 current DB/renderer Rust files were scanned with a source delimiter lexer handling nested comments, escaped quoted strings, Rust raw strings and character literals: zero unmatched delimiters. Every edited source write used a fresh-byte comparison immediately before same-volume atomic replacement; no Git/index/stash mutation occurred. Function-scoped replacement corrected ambiguous partial overlap insertion in `RuntimeApply`; two early generic-context module placements were removed before mounting native ownership at its actual current owner.

No compiler, build, test, formatter that compiles, or runtime command was run. Source tests are preserved, not claimed passing. DB storage fault-custody test access across extracted modules has not been compiler-validated.

**Unresolved cross-owner integration qualification:** the authored structured `ProgramFault.create_app_source` field in current `🤝️contracts/📥️lifted/🌉️ProgramBridge/🦀️.rs` still references `crate::kernel_runtime::CreateAppSourceFailure`, while HEAD places that concrete source owner in kernel protocol above contracts. A direct package edge would create a dependency cycle. This pass preserves the authored failure custody and HEAD extraction, but does not invent an error/owner abstraction, add a wrapper, or attempt to make that reference compile. Its visibility, derive requirements and other cross-owner source consumers also have no compilation evidence. Root explicitly confirmed this boundary is outside the requested preservation-only work.

The exact-source fixture and canonical contract changes remain authoritative over broad assumptions about compilation. Existing unrelated compiler/test failures and pipeline state were deliberately not investigated.
