# OS-Kernel Six-Law R1: Compiler RED

## Actual Terminal

The exact approved canonical invocation exited1. Rust reported semio-framework-os-kernel (lib test):92 errors and66 warnings. Zero of the six selected laws executed; no compiled six-test roster was produced. The run reached the real owner-crate test compile under --lib --features sync,ureq, not WGPU dependency compilation. No fix, feature change, retry, or extra native command was performed.

The source/compiler hold was released immediately after the terminal and postcapture. Subsequent read-only process inspection showed no cargo/rustc/nextest process. Root acknowledged Mutation's hold release. Later source work is not covered by this completed capture.

## Exact Attribution

Counts below are compiler diagnostics, not failing tests or distinct semantic defects. The two new Send assertions are actual compile-time RED evidence, but their async test bodies did not execute.

| Boundary | Diagnostics | Exact evidence and owner follow-up |
| --- | ---: | --- |
| Existing Directory library import | 1 E0432 | [Directory/client:481](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️component.rs:481) still imports TokioHostRuntime from async instead of its existing services owner. Reserved narrow import-identity repair; not changed here. |
| Existing SyncSession library detach | 1 E0277 | [SyncSession:900](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs:900) awaits synchronous Result<Option<Backbones>,VcsError>. This is not permission to discard the returned owner or just delete await; original-parent retained Store/session forwarding remains the design prerequisite. |
| Existing native actor library Send consumers | 2 uncoded Send errors | [Actor future:2245](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs:2245), with exact child notes pointing to compile_dsl1263 and print_mirror1277. Keep the actor future Send; repair only compiler-proven codec slots/thunks in the reserved packet. |
| New intended Send laws | 2 E0277 | [New Send leaf:19](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🧵️send/🧪️tests/🦀️.rs:19) and [line36](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🧵️send/🧪️tests/🦀️.rs:36) reject the actual two erased futures. No unsafe Send, blanket codec expansion or local executor substitution. |
| Sync cfg(test) trait implementations | 14 E0053 | Existing DemoSnapshot/DemoDiff/DemoMutation implementations at sync3727–3847 remain async where their actual traits now return concrete values. Exact signatures and existing wire/semantic assertions must be preserved by a separately assigned fixture join. |
| Sync cfg(test) stale await/cascades | 68 E0277 | 58 not-a-future diagnostics and10 unsized str/[u8] diagnostics in those fixture bodies/callers. The unsized diagnostics are recorded at the same stale-await expressions, not independently counted semantic defects. No speculative affected-test count. |
| Sync cfg(test) runner visibility | 1 E0603 | [Fixture call:4169](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs:4169) accesses fixture_runner_handle declared pub(super) at2541 under native_actor::retained_turn_fixtures. Its actual caller is a sibling outer test module; any visibility repair must stay test-only. |
| Sync cfg(test) channel calls | 2 E0599 | [Fixture:3708](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs:3708) and3709 call removed ChannelBackbone.send. A real current channel-owner API join must retain exact messages and assertions, not restore the removed API. |
| Sync cfg(test) Debug requirement | 1 E0277 | [Fixture:2600](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs:2600) uses Result::expect on WorkerSubmitError, which has no Debug. The returned closure is an owner; do not add a generic dropping formatter/error workaround. |

The last five rows total86 additional cfg(test) diagnostics. Actual cfg boundaries are sync2507/2508 for retained_turn_fixtures and3608/3609 for the outer tests. Thus92 = original4 library blockers + intended2 Send laws +86 test-compile diagnostics. The preserved JSON has93 level=error records because the additional record is the terminal “aborting due to92” summary; it is not a93rd source error.

Directory identity runtime observations and both backbone refusal assertions remain unexecuted. Absence of a diagnostic in a particular new leaf is not a pass. Existing test expectations, limits and production code were not weakened.

## Capture And Routing

Fresh pre/post capture contains586 rows. All SHA/byte/device/inode/mtime tuples are equal; every individual read was stable; selected domain re-enumeration has no added or removed members. All14 original native packet hashes match the retained R17 packet. Launch03eee150bb2be15d3a0afa78efddd739f22caa97cf0709accece2e18406d47a4 is the separately announced taxonomy publication and was stable during this gate; it is noncompiled provenance.

- [Exact prepared command/manifest](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️os-kernel-six-r1-capture-manifest-2026-08-28.json)
- [Complete before capture](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️os-kernel-six-r1-before-2026-08-28.json)
- [Complete after capture, empty drift/membership delta](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️os-kernel-six-r1-after-2026-08-28.json)
- [Actual command and all five tool-output chunks, exact escaped CR bytes](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️os-kernel-six-r1-tool-output-2026-08-28.json)
- [Readable raw tool output](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️member-os-kernel-six-r1-2026-08-28.md)

The readable Markdown normalizes two CRLF line endings from the initial command echo; comparison confirms that this is the only difference from the original five chunks. The JSON keeps their exact string bytes. No truncated process-inspection output is substituted for compiler evidence.

The actual warm invocation was cargo nextest list --list-type binaries-only --message-format json --profile exhaustive -p semio-framework-os-kernel --lib --features sync,ureq. Compilation failed before nextest run, so the requested no-fail-fast execution and anchored selection did not run. Same master target/jobs2/build-budget3600000/coverage0 remained in force. The newest observed artifact directory semio-nextest-eHzfQD is empty; no binary metadata or test stdout is claimed.

## Full Diagnostic Preservation

The terminal runner prints summaries; full diagnostics were recovered read-only from the exact fresh compiler fingerprint ending3dbb555fdf919f7c. The original and ticket JSONL copy independently hash to654962ed8040bcc4fb3f693e5c827faca180e2f4a332f3532aa900476140f16e and are818114bytes each.

- [All161 original JSON records](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️os-kernel-six-r1-compiler-diagnostics-2026-08-28.jsonl)
- [Complete rendered diagnostics,66 warnings and19 referenced full type files](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/📓️os-kernel-six-r1-full-compiler-diagnostics-2026-08-28.md)

One oversized read was truncated by the tool and was not used as evidence. Reading five complete JSON lines at a time recovered all161 records; JSON parsing and exact source/copy hashes verified the retained copy. No native rerun was needed.

Root's three earlier source oracles are separately documented in [coordinator source R1](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/📓️coordinator-os-source-r1-2026-08-28.md), which was read in full. Their Nx0 results are not evidence that these native laws ran.

Resident/Opening/Free→Refund/rejected-page2/Plugin/WGPU execution remains excluded. No native ownership, parent funding, timing or full-suite green follows from this compiler inventory.

