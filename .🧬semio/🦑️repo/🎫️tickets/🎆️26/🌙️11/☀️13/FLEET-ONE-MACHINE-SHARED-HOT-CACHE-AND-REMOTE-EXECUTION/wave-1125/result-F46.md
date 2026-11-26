# F46: dwg editor failures, dwg-snapshot / dwg lib tests, hub lock

## 1. `dwg-editor` / `dwg-editor-ac1018` `check --lib`
- Real compiler error 1: `…/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs` line 3 was plain text after `//!` inner docs (both ac1024 and ac1018 copies). Fixed with `//!`.
- It hid the second class (resolution errors, not related to ArtifactPersistedOperation): both editors' `🦀️.rs` called `semio_s_artifact_stdio_dwg_model::…::io::export::encode_dwg` / `::io::import::decode_dwg`; `dwg-model` is the schema crate and has no `io`; the functions live in `dwg-write` (export) and `dwg-read` (import). Fixed in the clean way: call sites (2 per editor file, 1 in the ac1024 window test `🧪️tests/🔬️unit/🦀️.rs`) now name `semio_s_artifact_stdio_dwg_write` / `_read`; both editor `Cargo.toml` list the two crates; `Cargo.lock` of dwg and hub gained the two deps for both editor packages (atomic, 4 lines each).
- Proof: `check --lib` of `semio-s-artifact-stdio-dwg-editor` Finished (23m55s). `semio-s-artifact-stdio-dwg-editor-ac1018`: NOT verified; every attempt fails in dependency `semio-ui-contract` with a build-system verdict, not a code error: `compiler needs an estimated 1630 MiB but its parent memory grant allows 1536/1436 MiB` (unit has no measured peak, cold estimate 1630 MB above the 1536 MB library budget: "carve it"). The ac1024 editor passed with a larger grant. Its sources/manifest got the identical edits as the ac1024 editor.

## 2. `dwg-snapshot` and `dwg` lib tests
Two causes, both fixed in tests, no facade:
- Peer API change (not F24): the SQLite snapshot owner API now takes an `output` slot and returns `RetainedWorkOutcome` (arity and `unwrap`/`is_err` errors). Tests moved to the stdio contract testing helpers `encode_native` / `decode_native::<DwgSnapshot>` / `import_sqlite` / `export_sqlite` (same migration the en1995 tests already have). 7 + 2 sites in `…/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs`, 1 + 1 in `🧪️tests/🗂️declaration/🦀️.rs`.
- F24 split (names moved into the domain crates, missing in test imports): `DwgXRecordValue`, `DwgApplicationPropertyKind`, `DwgVisibilityEvaluationHistory`, `DwgMlineLinetype`, `DwgBlockMoveCoordinateMode`, `DwgAssocNetworkMemberKind`, `DwgBlockActionCoordinateMode`, `DwgBlockScaleMode`, `DwgAssociativeActionStatus`, `DwgEvaluationVariant` now imported from the exact crates (xrecord, document, blocks, styles, associativity) in the main tests file.
- Proof: `check --lib --tests` Finished for `semio-s-artifact-stdio-dwg-snapshot` (14m27s) and `semio-s-artifact-stdio-dwg` (12m24s). Tests were only type-checked, not run (`cargo test` not run; bun oracle tests need a build of the whole closure).

## 3. Hub Cargo.lock
- Static check of `🌎️hub/Cargo.lock` (4675 packages): every dependency reference resolves to an existing package/version; the nine F24 domain crates plus root, document, drawing carry exactly the deps their manifests declare (compared with dwg's lock, which cargo validated: only dev-dependency differences, expected for non-member path crates).
- `repo cargo -- metadata --locked --offline` in hub: first run ended "The stamps moved again after the restart: run the command again" (no Bun crash seen); second run: `metadata --locked --offline` exit 0, 26 MB JSON, parsed, no lock change needed (so hub's lock is consistent after the split and after the editor dep additions).

## Notes
- Landings and scripts: `T/landings.md` (Wave 1125 F46), `T/🧰️scripts/F46/editor-io.py`, `editor-locks.py`. The landing lock `landing.lock.product-closure` was taken and released three times; dangling-imports exit 0 for every touched folder.
- Builds were slow because peer edits to framework crates repeatedly invalidated the whole dependency check (12-24 min each).
- The `semio-ui-contract` carve (1630 MB vs 1536 MB) blocks small-grant builds of anything depending on it; belongs to the native-build lane.
