# 📓️ UiIntent Kernel Event Bridge Report

**Date:** 2026-08-20  
**Ticket:** `26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME`  
**Related peer ticket:** `26/08/20/SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY` (`wit-flip` / `sdk-flip`)

## Problem

`cargo check -p semio-framework-plugin-host --lib` failed with a non-exhaustive match on
`kernel_event_to_wit` after a same-day peer edit added `Event::UiIntent { instance, intent }` to
`semio_framework::kernel::Event` without updating the host-side WIT bridge.

Separately, `semio-framework-plugin/⚛️reactor/🩹️patches/🦀️component.rs` had broken imports from the
`sdk-flip` migration (`semio_framework_semio_framework_ui_runtime` typo, missing `ui_contract` alias).

## Fix 1 — `kernel_event_to_wit` (`🖥️host/🦀️component.rs`)

Added the missing arm immediately after `AppCommandEvent`, mirroring the reverse mapping already
present in `⚛️reactor/🦀️component.rs`:

```rust
Event::UiIntent { instance, intent } => wit_events::Event::UiIntent(wit_events::UiIntentEvent {
    instance: instance.0.parse().unwrap_or(instance_id),
    intent: intent.clone(),
}),
```

WIT side was already ready (`ui-intent-event` record + `ui-intent(...)` variant in `📜️component.wit`).

## Fix 2 — `PatchTracker` imports (`⚛️reactor/🩹️patches/🦀️component.rs`)

- Corrected runtime import: `semio_framework_ui_runtime::{ComponentTree, SurfaceReconciler}`
- Added `use semio_framework_ui_contract as ui_contract;`
- Moved `TreeNode` import into `#[cfg(test)]` only

`UiPatch` already uses `SurfaceId` / `UiRevision` newtypes from the contract crate; the reconciler
API accepts `impl Into<SurfaceId>`, so the existing `&str` surface keys in `PatchTracker` remain valid.
`mark_ack` keeps `u64` because kernel `Event::PatchAck` still carries raw `revision: u64`.

## Verification

```text
CARGO_TARGET_DIR=/private/tmp/claude-501/cursor-scratchpad/target-ui-intent-fix \
  cargo check -p semio-framework-plugin-host --lib
→ EXIT 0 (1 warning: SharedThread visibility — pre-existing)

CARGO_TARGET_DIR=/private/tmp/claude-501/cursor-scratchpad/target-ui-intent-fix \
  cargo check -p semio-framework-plugin --lib
→ patches file: EXIT 0 (no errors)
→ full crate: EXIT 101, 109 errors (pre-existing sdk-flip residue in main `🔌️plugin/🦀️component.rs`:
  `Label::data` removal, `SurfaceKind`/`WindowLayout` type splits, `ComponentTree: Serialize`, etc.)
```

## Files touched

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🩹️patches/🦀️component.rs`

## Out of scope

The remaining ~109 errors in `semio-framework-plugin --lib` belong to the
`SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY` ticket's `sdk-flip` packet, not this bridge fix.
