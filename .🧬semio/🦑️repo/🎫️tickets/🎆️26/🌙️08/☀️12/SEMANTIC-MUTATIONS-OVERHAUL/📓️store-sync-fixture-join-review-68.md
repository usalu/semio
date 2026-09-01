# Store Sync Fixture Join Review 68

## Exact Assigned Boundary

The runtime coordinator assigned only 84 compiler diagnostics in the outer `#[cfg(test)] mod tests` of [Store Sync](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs). The native run was a compiler failure, not 84 failed tests: all six intended OS laws remained unexecuted.

Root read the complete [owner report](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/📓️os-kernel-six-native-r1-compiler-red-2026-08-28.md) and [rendered appendix](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/📓️os-kernel-six-r1-full-compiler-diagnostics-2026-08-28.md), including the referenced full type names. Two oversized tool displays were truncated; smaller overlapping reads recovered the missing passages before this review. No native command was run.

Root independently parsed all 161 original JSONL records. The exact file is 818114 bytes / SHA256 `654962ed8040bcc4fb3f693e5c827faca180e2f4a332f3532aa900476140f16e`. It contains 92 source error diagnostics, one error-level abort summary, 66 warnings, and two failure notes. Selecting primary outer-sync-test spans and excluding the reserved E0603 gives precisely 14 E0053, 68 E0277, and two E0599.

## Source/API Checks Before Any Edit

- DemoSnapshot implements synchronous ArtifactDsl/ArtifactPack methods as async; the six exact method signatures must match the actual traits.
- DemoDiff apply/absorb and DemoMutation OpText/OpBinary/diff/inverse likewise return concrete values under the current traits. Remove only proven stale async/await joins; preserve actual asynchronous Store/session/transport operations.
- The raw E0599 diagnostics do **not** prove that Backbone.send was removed. They explicitly point to an implemented trait method absent from scope. Current Store source still declares `Backbone::send` and implements it for ChannelBackbone. The fresh channel test must retain its same two ACK messages and FIFO one-pop assertions. An API plan must distinguish this scope join from any separately proposed retained-send authority.
- No production SyncSession, codec, backbone retirement, Fresh decoder, registry, Interaction, or lifecycle edit is included.
- The reserved fixture_runner_handle call and native_actor::retained_turn_fixtures/WorkerSubmitError sites remain runtime-owned.

Observed current sync endpoint: 268230 bytes / `37012443ee787d1a05e7826e8c3a8ac35ea0be6d43eba6918dcd7076c15e8d93`. Observed current Store endpoint: 1541032 bytes / `ed1d6b93b36a07f3c2aa914350c97f993613bc1a779e3b019a8f1329c7e19a37`. These are read-only current endpoints, not the earlier OS6 frozen source and not attributed to a particular writer.

## Separate Mandatory Metadata Finding

Reading the current base Mutation trait revealed default `DESCRIPTORS = &[]` and `descriptor()` returning `UNDECLARED_MUTATION_LEAF`, with comments explicitly describing migration compatibility. This is contrary to the explicit goal's mandatory leaf contract/no compatibility requirement. It also explains why an undeclared DemoMutation can lack the formerly mandatory members without this inventory reporting E0046.

This finding is separate from the assigned 84-diagnostic test-only packet. No base-trait restoration, default removal, fabricated descriptor, or native compile has been performed. Exact source ownership and a test-first follow-up must be coordinated; the broad goal is not complete while these defaults remain.

## Current State

The executor is preparing a bounded API/source plan with preserved values, wire semantics, and assertions; no production or test source edit is yet authorized by this review. Rejected-page-close Rust stays unmounted pending taxonomy's one-member release. Its two wrapper bodies remain unchanged for a future genuine semantic RED, after current compiler blockers are resolved.

