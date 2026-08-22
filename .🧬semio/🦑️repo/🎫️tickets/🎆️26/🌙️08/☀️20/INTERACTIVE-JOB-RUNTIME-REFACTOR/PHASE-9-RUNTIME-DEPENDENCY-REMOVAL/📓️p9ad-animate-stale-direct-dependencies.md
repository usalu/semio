# P9ad — Animate Stale Direct Dependencies

Date: 2026-08-22  
State: dependency cleanup landed; Animate crate compilation gate remains open

## Scope

This packet audited the real Rust source roots mounted by `semio-s-plugin-animate` and removed only direct dependency rows whose APIs were absent from those roots or could be replaced with owned standard-library values.

## Changes

- Removed the unused direct `comemo` row. No Animate source referenced `comemo`.
- Replaced the public `ecow::EcoString` fields and constructor bounds in the text engine with owned `String` values, then removed the direct `ecow` row.
- Removed the direct `fontdb` row. The only Animate occurrence was documentation; font discovery is provided by Typst assets and no Animate code names a `fontdb` API.
- Removed the direct `base64` row. Animate consumes the already-encoded result of `semio_framework_os::rasterize_svg_to_png_base64` and never calls the external crate.
- Updated the text-renderer boundary documentation so it names only dependencies actually used by that module.

The concurrent removals of `thiserror` and `pollster` in the same manifest are outside this packet and are not attributed here.

## Verification

- `rustfmt --edition 2021 --check <animate text component>`: pass.
- Touched-source `[DEBUG]` census: zero.
- `cargo metadata --no-deps --format-version 1`: pass.
- `bun ./📜️script.ts verify dependencies`: pass, baseline 238 → current 205; the ratchet reports both `rust:comemo` and `rust:ecow` removed.
- `cargo check -p semio-s-plugin-animate --lib --message-format=short`: fail after reaching the Animate crate, with 1,296 diagnostics from its broader unresolved de-async/generated-dyn-enum backlog. Representative failures are the missing generated `Animations`/`Sobjects` types, missing `dyn_enum_close`, and calls treating non-suspending helpers as futures. The check reported no unresolved `comemo`, `ecow`, `fontdb`, or `base64` import and no error caused by the `String` constructor bounds.

## Honest gate state

The dependency manifest resolves and the repository dependency ratchet is tighter, but this packet does **not** claim that the Animate crate compiles or that its tests pass. Phase 9 remains open until a dedicated Animate de-async/generated-macro repair makes the real crate compile and its text tests execute. `fontdb` and `base64` also remain external identities elsewhere in the workspace; only Animate's stale direct rows were removed here.

