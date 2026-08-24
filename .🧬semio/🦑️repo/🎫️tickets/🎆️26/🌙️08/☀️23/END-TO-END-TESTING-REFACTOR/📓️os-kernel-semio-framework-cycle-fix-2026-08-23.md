# os-kernel ↔ semio-framework dependency-cycle fix

## Problem

`semio-framework-os-kernel` could not compile: two of its own mounted files —
`🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🦀️component.rs` (54 sites) and
`🏪️store/🦀️component.rs` (3 sites, later shown unrelated — see below) — referenced
`semio_framework::kernel::{FixedCommandPage, CommandPageSet, PagedCommand, PagedCommandReader,
COMMAND_PAGE_MAXIMUM_BYTES}` and `semio_framework::{Fault, FaultOrigin, FaultCode}`. `semio-framework`
depends on `semio-framework-os-kernel` (`🧰️framework/🔨️modules/🎠️kernel/🦀️component.rs:776` already
re-exports `PresencePeer` etc. from it), so `os-kernel` cannot depend back on `semio-framework` —
those five symbols, defined only in `🎠️kernel/🦀️component.rs`'s `//#region 🔖️PagedCommandIngress`
(a semio-framework-only file, mounted into `semio-framework`, `semio-framework-graph`, and
`semio-s-plugin-stdio` — confirmed by that file's own `ExtensionDescriptor` docstring), were
genuinely unreachable from `os-kernel`. `Fault`/`FaultOrigin`/`FaultCode`, by contrast, are NOT
semio-framework-only: they are mounted into `os-kernel` itself already, via
`semio-framework-replication` (Cargo `[lib] name = "protocol"`) → `os_dsl::diagnostic` →
`pub use crate::os_dsl::*` at `os-kernel`'s crate root. `crate::Fault` already worked in `os-kernel`
without any dependency change.

## Fix

1. **`Fault`/`FaultOrigin`/`FaultCode`** (channel/component.rs, ~26 sites): requalified
   `semio_framework::Fault` → `crate::Fault` (same for `FaultOrigin`/`FaultCode`). No move, no new
   dependency — the type was already reachable at `os-kernel`'s own crate root.

2. **`FixedCommandPage`/`CommandPageSet`/`PagedCommand`/`PagedCommandReader`/
   `COMMAND_PAGE_MAXIMUM_BYTES`** (and the rest of the self-contained `PagedCommandIngress` region —
   `CommandEnvelope`, `CommandBatch`, `CommandEnvelopeSet`, `RejectedCommandBuild(Registry)`,
   `CommandBatchProgress`, `CommandBatchDriver`, `CommandDriverRegistry`, `CommandPageCursor`,
   `CommandIngressStatus`, and the `COMMAND_*` consts): the whole `//#region 🔖️PagedCommandIngress`
   (`🎠️kernel/🦀️component.rs:880-1822`) was self-contained (only used `Fault`/`FaultOrigin`/
   `FaultCode` and `serde`, nothing else semio-framework-only), so it was moved wholesale into
   `channel/component.rs` (the one file that actually needs it as real code — inserted as its own
   `//#region 🔖️PagedCommandIngress`, right after `//#endregion 🔖️ChildPackEntry`), qualifying its
   internal `Fault`/`FaultOrigin`/`FaultCode` → `crate::*` and its `Serialize`/`Deserialize` →
   `serde::*` to match this file's existing zero-`use`-statement, fully-qualified style. `kernel/
   🦀️component.rs` keeps the region marker but its body is now a single re-export:
   `pub use semio_framework_os_kernel::channel::{...same 17 names...};` — the exact shape its own
   `PresencePeer` re-export at line 776 already uses. This was NOT optional (contrary to an earlier
   coordinator correction that assumed the five symbols only needed local-path fixes purely inside
   `channel/component.rs`): `kernel/component.rs`'s own `Event::CommandIngressPage` variant and its
   own `#[cfg(test)] mod extension_activation_tests` (`use super::*`) still reference these types, and
   they are consumed unqualified via the same `semio_framework::kernel::X` path from FIVE other
   os-kernel-external files (`🏃️run`, `🔌️plugin/🦀️component.rs`, `🔌️plugin/⚛️reactor/🦀️component.rs`,
   `🔌️plugin/🖥️host/🦀️component.rs`, `🌉️mcp/🏠️workspace/🦀️component.rs` — all in separate crates that
   legitimately depend on `semio-framework`). Deleting the qualification without leaving a re-export
   would have broken all six of those call sites.

3. **`UiPatch`** — NOT moved, NOT given a new dependency. It never caused a real error: in
   `channel/component.rs` it appears only in two doc comments and as an unrelated local enum-variant
   tag (`AppFrame::UiPatch { .. }`, a different enum, value namespace only). Verified against the
   pre-fix `cargo check` output: zero errors mention `UiPatch` (`grep -c UiPatch` on the captured
   117-error log = 0). It stays defined in `semio-framework-ui-contract` and re-exported from
   `🎠️kernel/🦀️component.rs`'s separate `//#region 🔖️UiPatch`, untouched.

## Verification (real `cargo check` output, repo root)

- Before: `cargo check -p semio-framework-os-kernel --lib` → **117 errors** (57× E0433, all in
  `channel/component.rs` + 3 in `store/component.rs`; 22× E0277, 5× E0308, 9× E0502, 24× E0599, all
  already in `store/component.rs` + `testkit/component.rs` + `replication/🔗️causal/component.rs`,
  confirmed present in the pre-fix log too — pre-existing, unrelated to this cycle).
- After: `cargo check -p semio-framework-os-kernel --lib` → **63 errors**, ALL in
  `store/🦀️component.rs` (peer's in-flight `MutationDagAppliedStep`/`drain_applied_envelopes`
  refactor — actively edited, mtime ~10 min before I started; NOT touched per explicit
  instruction), `testkit/🦀️component.rs` (2× `drain_applied_envelopes`, same cause), and one
  pre-existing dead-code warning in `replication/🔗️causal/component.rs`. **Zero errors remain in
  `channel/component.rs` or `kernel/component.rs`.**
- `cargo check -p semio-framework --lib` and `cargo check -p semio-s-plugin-stdio --lib`: both stop
  at the same upstream `semio-framework-os-kernel` compile step with the identical 63 errors (same
  file set, same breakdown) — cargo never reaches either crate's own code, so this is not evidence
  either crate's own source is broken, only that the dependency graph is blocked on `store.rs`.
  Because `semio-s-plugin-stdio` never compiles, **`subject`/`parity` for `mutate-pdf-1-7` could not
  be run** — blocked entirely by the peer's `store.rs` work, not by anything in this fix's scope.

## Files touched

- `🧰️framework/🔨️modules/🎠️kernel/🦀️component.rs` — region body replaced with a re-export.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🦀️component.rs` — region inserted, all
  `semio_framework::kernel::X` / `semio_framework::Fault|FaultOrigin|FaultCode` references
  requalified to local / `crate::`.
- `🏪️store/🦀️component.rs` — **not touched** (peer's active file, per explicit instruction; its 3
  original error sites were never the six symbols in scope here, and its current errors are a
  separate, pre-existing `MutationDagAppliedStep` issue).
