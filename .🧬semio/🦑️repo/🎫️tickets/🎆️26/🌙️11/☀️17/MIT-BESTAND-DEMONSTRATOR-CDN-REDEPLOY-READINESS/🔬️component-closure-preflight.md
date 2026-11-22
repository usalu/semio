# Component closure preflight (demonstrator, process)

Target `wasm32-wasip2`, profile `release`, workspace `🌎️hub/Cargo.toml`, `-p semio-hub-demonstrator-component -p semio-hub-process-component --lib`, nightly-2026-07-20, `--locked --offline --keep-going -j2`.

## Commands

1. Gate route: `repo cargo --store=shared --configuration=release --parallelism=2 -- check --locked --offline --keep-going -p ... --lib --target wasm32-wasip2 --manifest-path 🌎️hub/Cargo.toml`. It first freed 3.3 + 3.0 + 7.2 GiB of derivable caches to reach its 16 GiB reserve (about 10 min), then failed in source admission twice, before the compiler:
   - `semio-plugin-instance-app` exceeded `maxSourceBytes` (267663 of 262144, 19 of 32 files). Another agent is splitting it (`natural-publication`, `docs`, `staging/*`); it later passed.
   - `semio-s-artifact-stdio-pdf-page-editor` exceeded it (266064 of 262144, 22 files). Same agent split it into `authoring/`, `view/` and a new page-editor-base crate; it later passed.
   The retried release builds go through the same admission, so they can die on such an overage if started mid-split.
2. Compiler route (no gate wrapper, private dirs in the scratchpad, deleted afterwards): `repo tools exec --light rustup -- run nightly-2026-07-20 cargo check --locked --offline --keep-going -j2 --manifest-path 🌎️hub/Cargo.toml -p ... --lib --target wasm32-wasip2 --profile release --message-format=short --config build.rustc="rustc" --config build.rustc-wrapper="" ...` with `CARGO_HOME=.cache/b/r/m/cargo/home`, `CARGO_TARGET_DIR`/`CARGO_BUILD_BUILD_DIR` in the scratchpad (about 1 GiB).

## Errors found

| Crate | Error | Status |
|---|---|---|
| `semio-s-artifact-stdio-pdf-graph-projection` | `xref/🌐️file-structure/🦀️.rs` (new 15:37, mirrored into this crate) `use semio_framework_value::{ByteReadRange, ByteReadSource}` unresolved: crate had no such dependency | Fixed: dependency added to its `Cargo.toml`; `🌎️hub/Cargo.lock` entry updated by cargo (also picked up other agents' pending new crates) |
| `semio-plugin-instance-app` | `natural-publication/🦀️.rs:37` E0599 `terminal_is_empty` is an associated function, not a method | Another agent's live refactor; no longer reproduced in later run |
| `semio-s-artifact-stdio-pdf-page-editor-authoring` (new, untracked) | E0432 `mut_base_4::...::io::{export, import, mutation_bridge}` do not exist; mut-base-4 only carves binary mutations | OPEN, not fixed (see below) |

## Open item: page-editor-authoring

The needed items live in `semio-s-artifact-stdio-pdf-schema-4` (`io::export`), `semio-s-artifact-stdio-pdf-mut-family-11` (`io::import`) and `semio-s-artifact-stdio-pdf-mut-base-6` (`io::mutation_bridge`). Depending on schema-4 creates a package cycle (`schema-4 -> tests-2 -> page-editor -> authoring -> schema-4`) that breaks cargo resolution for the whole workspace; I tried it and reverted at once. The fix needs a layering decision in the agent's page-editor split (the `export` module has to come from a crate below `tests-2`, or be moved down). Left to the owner.

## Status

- `semio-hub-process-component`: no compile error in its closure in the run that reached it (crate checked last, clean); it does not depend on the page-editor.
- `semio-hub-demonstrator-component`: NOT clean. Blocked by page-editor-authoring; crates downstream of it (page-editor, pdf, the demonstrator) are unchecked. Earlier fixes (process3d-editor macro arms, xlsx-base-mutations import) hold: no error from them.
