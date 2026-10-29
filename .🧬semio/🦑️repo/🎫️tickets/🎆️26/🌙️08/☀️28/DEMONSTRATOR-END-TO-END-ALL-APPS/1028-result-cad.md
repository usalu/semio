# 1028 result: cad

Status: RED, now reaching rustc; frontier = foreign `semio-framework-os-store-replay` (see Update 2). (older status follows) Frontier = pre-compiler Rust-crate source-boundary gate on shared framework crates (not cad code). `semio-hub-cad-component` (wasm32-wasip2, `🌎️hub/Cargo.toml`) has not reached the compiler. Extensions (workspace `✏️s/Cargo.toml`: aec-building, aec-building-energy, aec-building-structure, spatial-shape) NOT checked yet (one queue slot ~10 min each; stopped at the shared blocker).

## Files changed
- ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/Cargo.toml: added `[workspace.dependencies]` for `semio-framework-io-ports`, `semio-framework-value-snapshot-read`, `semio-framework-os-spr`.

## Errors fixed
- hub workspace did not load (CargoLockRefused / cargo metadata): draw artifact crates `drawing-model` and `drawing-editor-store-preparation` inherit those three framework crates from the drawing workspace root, which did not declare them -> declared them. After this the hub Cargo.lock refresh and source admission proceed. This blocked EVERY `🌎️hub/Cargo.toml` check (all hub-workspace fleets).

## Remaining blocker (diagnosis)
Source admission (`admitCargoSourceBoundariesV1`) enforces limits = min(global policy, crate `[rust-source-boundary]`). The global policy `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔣️policy.json` `sourceBoundary` is 262144 bytes / 32 files / 4000 lines and its schema `maximum` is the same, so raising a crate's own `rust-source-boundary` (as peers did for store-engine to 1 MiB, os-vcs to 64 files, plugin-decl-views to 512 KiB/48) has NO effect: the closure must be split. Refusals seen in order (each class is the same):
- semio-framework-os-store-engine: closure over 256 KiB (assigned to puzzlehub per fem note).
- semio-framework-os-vcs: file count exhausted at `🌿️vcs/🚪️io/📝️text/🌱️genesis/🦀️.rs` (remainingFiles 0 of 32).
- semio-framework-plugin-decl-views: bytes exhausted at `🔌️plugin/📝️draft/🚫️none/♻️retirement/🦀️.rs` (825 bytes left; its `#[path]` closure spans ../../ sibling dirs).
More crates will likely follow after each is split (units are admitted in order; ~873 units). Needs a coordinated split (or a deliberate policy+schema raise) of these shared crates before any hub component can reach rustc.

## SHARED CHANGE
- None kept (a temporary decl-views boundary bump was reverted to the peer's 524288/48/6000). draw workspace root edit above is artifact-local.
Logs: C/🗑️generated/1028/cad-chk{1..9}.txt

## Update (chk11)
- os-vcs: coordinator ordered a split, then cancelled it: gatecensus measures it PASSING (261954 B / 31 files) after a peer's in-place module inlining. No split done by me.
- After vcs, store-engine and decl-views passed the gate, chk11 now stops at `semio-framework-plugin-job` (`🔌️plugin/👷️job/🏛️registration/🦀️.rs` 14642 B, 1395 B budget left; 16 files left): shared crate over the 256 KiB cap, owned elsewhere. Logs: cad-chk11.txt.
- cad slice still has not reached rustc; extensions unchecked.

## Update 2 (chk13 hub + y1..y4 extensions)
Extensions are four separate workspaces (`<ext>/Cargo.toml`), checked individually: packages semio-s-plugin-cad-{aec-building,aec-energy,aec-structure,spatial-shape}-component. All four plus `semio-hub-cad-component` now stop at the SAME single foreign crate; no cad-owned errors seen.

### Fixed (shared)
- `semio-framework-os-store-canonical-json` (`🏪️store/🧵️canonical-edit/🦀️.rs`): the `ArtifactCanonicalTreeSource` block (struct + 2 impls) was duplicated verbatim (E0428/E0119/E0592/E0034, 8 errors) -> removed the first copy.

### Remaining (foreign)
- `semio-framework-os-store-replay` (`🏪️store/🔁️replay/⏩️replay-owned-state-operations/🦀️.rs`): L503 `settlement.step(..)?` now returns `(bool, RetainedCloneProgress)` (expected bool); L600 `OperationMessageSettlement::new` is private. Cause: peer mid-split of `🔁️replay/🎮️operation/🦀️.rs` (uncommitted, edited 19:21; also reported by demobundle). Owner: whoever runs the replay operation split; consumer must destructure the tuple (account the progress) and `new` must be `pub`/constructed by the owner.
- Unknown beyond that (downstream crates not yet compiled).

## SHARED CHANGE
- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit/🦀️.rs: removed duplicate ArtifactCanonicalTreeSource block (reason above).
- ✏️s/…/🖍️draw/🗿️artifacts/🖍️drawing/Cargo.toml: workspace deps (artifact-local).
Logs: C/🗑️generated/1028/cad-chk13.txt, cad-y1..y4.txt

## Update 3 (free-fix mode, runs r1..r3, 20:00-21:00)
Re-run tool: `C/1028-tools/cad/run.sh N` (launches the 5 detached checks tagged N, waits <=570 s per call; call again with same N until all rc files exist). Logs `C/🗑️generated/1028/cad-rN-{hub,b,e,s,p}.txt`.
- r1 (20:00): engine-core `take_derived_report_result` bounds (`P: Clone, Mutation: ColdMutation<P>`) added uncommitted by a peer at 19:54, not satisfied by callers (finish_derived_report_replay / delegating accessor / time-travel store-state). Peer still editing (changed again 20:05), so not touched.
- r2 (20:30): engine-live / engine-bundle / lifted_store imports in flux (each of the 5 checks saw a different tree) - active store-engine split sweep.
- r3 (21:00): all 5 checks now fail identically in `semio-framework-os-store-engine-command` (`🏪️store/🕰️history/🎮️command`), 151 errors: unresolved `ArtifactDerivedOperation(Mode)` import from os-store-replay (moved to `replay_preparation`), `ArtifactOriginalVcsFailure.source` private, `ArtifactNativeVcsFrame` lost `caller_generation`/`generation`/`registry` fields. Files in that crate changed within the last 10 min (peer mid-migration of the store/replay split), so per the 10-minute rule not touched.
No cad-owned error has appeared in any run. Next step when the store split settles: run `run.sh 4`.

## CHECKPOINT (22:55) - final status
STATUS: RED (no cad-owned error ever seen; blocked by foreign plugin-layer crates in flux). Files changed by me: draw `Cargo.toml` (workspace deps), `store/🧵️canonical-edit/🦀️.rs` (duplicate block removed). No edits in progress.
r4 (22:15) per-crate frontier, all foreign: `semio-framework-plugin-instance-stores` (🧵️instance/📥️staging/📨️dispatch L188/191: `Vec<u8>` where `PagedBytes<MAX>` expected; non-bound, files edited <10 min), `semio-framework-plugin-runtime-ops` (6-arg method called with 4; `StagedWindowContextCapture: Copy` unsatisfied; ext checks also `NativeCloseKey: RetireOwned` unsatisfied = bound class), `semio-framework-os-kernel-neural-engine-model` (missing dsl-record deps; peer added them to Cargo.toml 22:25, likely fixed).
r5 checks (5 detached, pids not recorded) were still queued/running at checkpoint; results land in `C/🗑️generated/1028/cad-r5-{hub,b,e,s,p}.{txt,rc}`.
NEXT: `C/1028-tools/cad/run.sh 5` (re-attaches to r5 without relaunch; repeat until all rc exist), read `grep -n "^error\|could not compile"` per log; fix non-bound errors (instance-stores PagedBytes at dispatch L188/191 first), then `run.sh 6`.
