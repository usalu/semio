# Canonical Framework Cargo Path Repair

Date: 2026-09-04  
Ticket: `26/09/02/COMPLETE-SEMIO-END-TO-END`

## Boundary

Current Cargo metadata failed before browser/native document-open server laws because eight active Rust manifests still referenced the removed `🖱️ui/🎨️styling` owner after the canonical owner became `🖱️ui/🎨️🟠️styling`. The first reproducible terminal was `semio-framework-ui-contract` failing to read the old styling `Cargo.toml` with OS error 2. Once that frontier was repaired, metadata exposed two OS manifests still referencing removed `🔨️modules/🧮️math` instead of canonical `🔨️modules/🧮️🔢️math`.

## Repair

All eight active `ui_styling` path dependencies now resolve the canonical `🎨️🟠️styling/📦️packages/🦀️rust` manifest while retaining their existing relative depth and package identity. The two active OS `math` dependencies now resolve canonical `🧮️🔢️math`. No package feature, dependency version, workspace membership, source API, or generated lockfile was changed.

## Evidence

The pre-repair command and terminal were:

```text
cargo metadata --no-deps --format-version 1 --manifest-path ✏️s/🔌️plugins/🏗️fem/📦️packages/🦀️rust/Cargo.toml
failed to read .../🖱️ui/🎨️styling/📦️packages/🦀️rust/Cargo.toml (os error 2)
```

The exact post-repair command exits `0`. Active-Cargo stale-reference censuses find zero old `ui_styling` or framework-math paths, and `git diff --check` exits `0`. This packet only restores manifest graph resolution; downstream source diagnostics remain separate.
