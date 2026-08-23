# P8yw Raster Retained Ingress Independent Final Audit

## Verdict

RED. The packet removes Raster from the raw whole-buffer caller census and introduces retained ingress, cancellation, polling, and close surfaces. It does not yet satisfy the bounded-opportunity, exact-ownership, or terminal-retirement gates required for source acceptance.

## Scope

- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs`
- Raster editor and Wasm ingress changes named by the implementation handback
- Phase 8 structural verifier evidence and ledgers

No Cargo, Nx, Wasm, or browser runtime gate was run in this audit because overlapping Rust source packets remain active and the plan requires one serialized build owner after source convergence.

## Accepted Evidence

- The raw structural census is now 12 matches: one shared definition and 11 live callers. Raster contributes no remaining raw caller match.
- The Wasm bridge exposes page ingress, poll, cancel, and close instead of a whole-buffer constructor.
- The packet's narrow verifier self-tests and deterministic ledgers are useful regression evidence.

These facts are preserved, but they do not prove bounded work inside the retained job.

## Blocking Findings

### 1. Recursive Raster owners are still cloned and encoded in one opportunity

`RasterSnapshotCloneAuthority::clone_layer` serializes the entire recursive `RasterLayerNode` before checking its encoded length, then calls `source.clone()` on the whole node (`component.rs:688-694`). `clone_asset_entry` does the same for an entire asset entry (`component.rs:696-701`). The surrounding step invokes those helpers once per top-level layer or asset and consumes fuel only after the work and allocation have completed (`component.rs:704-748`).

A hostile but structurally valid nested group can therefore perform arbitrarily deep traversal, allocation, serialization, and cloning in one scheduler opportunity. The 4 KiB check is post-allocation and is not a pre-admission bound. Low remaining fuel or a near deadline cannot prevent this work; only the zero-budget precheck is discriminated.

Required repair: use an explicit resumable cursor for recursive layer/asset census and construction, perform exact pre-admission before ownership transfer, and consume scheduler budget before each bounded unit. Do not use whole-value `serde_json::to_vec` or `Clone` as the admitted unit.

### 2. Applied history still clones and traverses the whole current snapshot per mutation

`ApplyForward` encodes a complete operation, computes its diff, and applies it in a single call before consuming fuel (`component.rs:1087-1119`). `RasterDiff::apply` begins by cloning the complete snapshot (`text/component.rs:332-349`) and its layer helpers perform further whole-vector/tree work. Snapshot size is not bounded by the size of one encoded mutation; it grows with accepted history. Hashing inverse and redo operations likewise encodes an entire operation before the size check.

Thus “one mutation per step” is not a bounded step. A small mutation against a large current snapshot can clone or traverse the full recursive snapshot in one opportunity.

Required repair: make mutation validation, hashing, semantic application, replacement construction, and previous-owner retirement cursorized operations with explicit item/byte/depth bounds. Add fixtures where small forward mutations target large/deep snapshots under low nonzero fuel and expired/near-expired deadlines.

### 3. A completed candidate is not part of cancellation/staleness retirement

`BuildCandidate` moves the envelope and runtime into `self.candidate` and returns `Complete` (`component.rs:1220-1230`). The job can subsequently transition from `Complete` to a cancellation/fault retirement phase, but the terminal retirement pump does not include `self.candidate`. The normal framework path takes the candidate immediately, so this is primarily a trait/lifecycle correctness hole; direct or adversarial scheduling can leave the candidate retained while terminal handoff is declared.

Required repair: include the completed candidate in an explicit bounded disposer/retirement cursor before terminal-empty acknowledgement, and add cancel/stale-after-complete fixtures.

### 4. Retirement bytes are not exact allocation ownership

String retirement compares and reports `String::len()` (`component.rs:70-78`), and byte-vector retirement compares and reports `Vec::len()` (`component.rs:336-342`). Dropping either owner releases its allocation capacity, not merely its logical length. A value with spare capacity can therefore release more bytes than granted or reported.

Required repair: account the exact owned allocation capacity used by the admitted owner model, and make the corresponding admission/retirement evidence symmetric.

### 5. Nested retirement retains a recursive boxed call chain

`RasterOwnedRetirement` stores `active: Option<Box<RasterOwnedRetirement>>`, and `close_step` recursively calls the child cursor (`component.rs:348-358`). Hostile nested Raster values can create an arbitrarily deep call chain during each retirement grant unless depth is proven bounded before admission.

Required repair: use a fixed-capacity iterative retirement stack or establish and enforce an admitted maximum depth before retaining the owner. Add deep hostile retirement fixtures with one-item/small-byte grants.

## Verifier Gaps

- The current success case proves removal of the raw Raster caller, not absence of whole recursive work inside its replacement.
- Add discriminating mutations for post-allocation caps, whole recursive clone/encode, fuel consumed after work, complete-candidate cancellation, length-versus-capacity retirement, and recursive retirement depth.
- Keep the existing 12-match structural expectation only as a census gate; it is not a semantic acceptance gate.

## Acceptance Conditions

The packet may be re-audited after all five blockers are repaired, its narrow verifier passes with the new discriminating fixtures, the global verifier remains free of Raster failures, and the eventual serialized native/Wasm/browser timing matrix exercises the live Raster ingress lifecycle.
