# Snapshot fixes: animate, architect program presence, forms snapshot (2026-11-24)

Tree edited: `/Users/ueli/.sinc/play`. Nothing was compiled (no cargo/rustc); verified by reading only. Live repo had no fix for any of these (diffed: identical to the broken snapshot apart from the edits below).

## 1. semio-hub-animate
- Cause: `dyn_enum_close!` expands to `semio_framework_value::…` paths; the crate had no `semio-framework-value` dependency.
- Model: `🌎️hub/🧩️compositions/🖍️draw` (`semio-framework-value.workspace = true`; root workspace declares it at `Cargo.toml:975`).
- Changed: `/Users/ueli/.sinc/play/🌎️hub/🧩️compositions/🎞️animate/📦️packages/🦀️rust/Cargo.toml` (added the dependency after `semio-framework-job`).
- Hub crates are not in the root `Cargo.lock`, so no lock change expected.

## 2. semio-s-artifact-architect-program-presence
- Cause: not the removed `pub use`. `ArchitectPresence` derives `NativeReceive` and has `Option<AdjacencyKind>`; `AdjacencyKind` (registers kernel) lacked `NativeReceive`. All 7 errors (E0277 x5, E0599 x2) stem from that.
- Model: unit scalar enums in stdio semio snapshots (`DslScalar` + `semio_framework_value::NativeReceive`); the derive supports unit enums.
- Changed: `/Users/ueli/.sinc/play/✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🗄️registers/🧱️kernel/🦀️.rs` (`AdjacencyKind` derive list gains `semio_framework_value::NativeReceive`). Only definition of the enum; `semio_framework_value` already a dependency there.
- Rest of presence file compared to CAD presence: same `ArtifactPresenceSnapshot` shape. No other NativeReceive users in architect.

## 3. semio-s-artifact-forms-forms-snapshot
- (a) Binary module declares a local `pack` module, shadowing the extern `pack` crate; line 27 used `pack::snapshot::ArtifactDsl`. Replaced with `semio_os_kernel::os_store::ArtifactDsl` exactly like lines 16/21/26 of the same file.
- (b) `encode_sqlite_snapshot_native` computed `counts` inside a throwaway closure, then used it outside. Now `let counts=match admission::forecast(..){Ok(..)=>..,Err(error)=>return RetainedWorkOutcome{NotStarted(error),work:0}};` then the record encode, mirroring lowpoly/equation (forecast first, early NotStarted return). `record_controlled(self,&counts,native)` signature already matches.
- Changed:
  - `/Users/ueli/.sinc/play/✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/📸️snapshot/🦀️.rs`
  - `/Users/ueli/.sinc/play/✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs`
- Checked: other `pack::snapshot::ArtifactDsl` uses in the crate (bound, sqlite, text) sit in modules without a local `pack`; decode signature and `preflight` match lowpoly's.

## Unresolved / risks
- Unverified by compiler; later errors may surface after these (e.g. further native-receive derive bounds for architect presence, other forms decode drift).
- Animate: possible further drift behind the single reported error.
