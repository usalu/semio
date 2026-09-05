# UI WGPU Taxonomy Path Repair

Date: 2026-09-04

## Outcome

The framework UI WGPU root now mounts its physical component module at `🦀️🧩️component.rs`; the stale `🦀️component.rs` edge no longer blocks downstream library builds. The registered owner generator also refreshed the current two-locale/two-terminology Rust and TypeScript axis projections.

## Evidence

- `bun nx run @semio-tech/ui-rs:generate --skip-nx-cache`: session `22533`, exit `0`.
- `bun nx run @semio-tech/ui-rs:check --skip-nx-cache`: session `74671`, exit `0`; both generated axis files are fresh.
- `bun nx run @semio-tech/ui-rs:test-quick --skip-nx-cache`: session `82295`, exit `1` after reaching the crate on two existing test-only `E0308` mismatches. This is not a full owner-suite pass and is not represented as one.
- The public-directory agent's later isolated hub all-feature compilation passed `semio-framework-ui` as a library and advanced downstream, confirming that the missing module edge itself is removed.

## Files

- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🤖️generated.rs`
- `🧰️framework/🔨️modules/🛂️manifest/🤖️generated/🟦️ui-axes.ts`
