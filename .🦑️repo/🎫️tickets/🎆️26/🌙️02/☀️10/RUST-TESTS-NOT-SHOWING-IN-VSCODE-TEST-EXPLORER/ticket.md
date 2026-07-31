---
goal: AI-OPTIMIZED-REPO/SANDBOXED-REPO/ZERO-TOUCH-DEVCONTAINER
---

# Ticket

## Summary

Fixed two issues in `.devcontainer/devcontainer.json` preventing rust-analyzer from discovering Rust tests for the VSCode Test Explorer:

1. **Wrong path in `rust-analyzer.linkedProjects`**: Was `rs/compose/Cargo.toml`, corrected to `compose/rs/Cargo.toml`.
2. **Removed `rust-analyzer.cargo.target: wasm32-unknown-unknown`**: This caused rust-analyzer to compile for wasm, hiding all `#[cfg(not(target_arch = "wasm32"))]` test code and dev-dependencies (like `proptest`).

## Changes

- `.devcontainer/devcontainer.json`: Fixed `rust-analyzer.linkedProjects` path and removed wasm target override.
