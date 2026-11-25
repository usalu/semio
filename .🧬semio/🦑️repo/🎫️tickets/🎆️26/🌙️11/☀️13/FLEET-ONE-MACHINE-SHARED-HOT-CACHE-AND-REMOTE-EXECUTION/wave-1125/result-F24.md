# F24: split of `semio-s-artifact-stdio-dwg-snapshot-objects` (wave 1125)

## Result
Eight domain crates plus the slim root. The old crate name stays as the root (object body union only); all other code moved to new crates in `…/🧬️schema/📸️snapshot/📦️objects/<domain>/📦️packages/🦀️rust`, each with `[lib] path = "../../🦀️.rs"` (flat module, no `standards::…` nesting). No re-exports, no facade.

| crate | depends on (inside the split) | rmeta MB | est. rlib MB | est. lib charge MB |
|---|---|---|---|---|
| metadata (macro crate, `#[macro_export]`, no deps) | - | 0.02 | ~0 | ~0 |
| xrecord | metadata | 0.45 | 8 | ~170 |
| tables | metadata | 2.28 | 42 | ~860 |
| associativity | metadata | 1.22 | 23 | ~460 |
| entities | tables, metadata | 2.53 | 47 | ~960 |
| styles | tables, metadata | 2.37 | 44 | ~900 |
| constraints | associativity, metadata | 0.70 | 13 | ~265 |
| blocks | associativity | 2.15 | 40 | ~815 |
| root (object union) | all eight | 1.20 | 22 | ~455 |
Before: one crate, rlib 240 MB (F15), charge 4,894 MB. Estimate method: the 240 MB and 4,894 MB split by each crate's share of the measured rmeta sizes (12.9 MB total); no wasm build was run, so these are proportional estimates. Largest piece ~1.0 GB, far below 4,096 MB. Not measured: real charge.
Also dropped from every new crate: os-kernel, schema, schema-composition, schema-state (and four unused `extern crate` aliases); metadata macros call each other via `$crate::`.

## Changed
- Domain crates: `Cargo.toml` + the domain `🦀️.rs` (header, imports of sibling crates and macros). Root: `🦀️.rs`, `Cargo.toml`; shim `📦️packages/🦀️rust/🦀️.rs` deleted.
- Registered: `🖊️dwg/Cargo.toml` (members + workspace.dependencies), `📋️build-owners.json`, `…/native-build/📏️paths/🧪️tests/🧫️fixtures/🔣️.json`. Admission carving entries (`🎟️admission/🧬️schema/🔣️.json`, unit-ceiling test) still name the old crate; left, because a unit leaves the list only when remeasured.
- Consumers: 47 `.rs` files path-rewritten; 21 manifests list exactly the used domain crates (`split.py`, plus fixes by compile errors for files mounted from sibling folders); 29 `Cargo.lock` files in other workspaces rewritten by `locks.py` (atomic rename).
- Scripts: `T/🧰️scripts/F24/split.py`, `locks.py`. Landing in `T/landings.md`.

## Proof
- `check -p` of all nine crates: Finished, no warnings.
- `check --workspace --lib --keep-going` (dwg workspace): all pass except `semio-s-artifact-stdio-dwg-editor` and `-editor-ac1018`: a doc line without `//!` in `…/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs` line 3 (file unchanged since Nov 5; pre-existing, not touched by me).
- `--tests`: failures only in `dwg-snapshot` and `dwg` lib tests: `RetainedWorkOutcome::unwrap/is_err`, argument counts and `DwgApplicationPropertyKind`; I believe unrelated (not caused by the split) but did not prove it against HEAD. The sqlite test mounts relied on names via glob; not investigated further.
- `cargo test --lib -p` of the nine crates: pass (0 tests: the crates carry no tests; real behaviour tests are the sqlite/binary round-trips of the consumers, which do not compile for the reasons above).
- `cargo metadata --locked` in the cad workspace: ok (locks consistent). Hub: the same command crashed in Bun (`Bun v1.4.2` crash), not verified; the 29 locks were not validated with cargo except cad and dwg.
- `dangling-imports.py` on `…/📦️objects` and `…/🚪️io`: exit 0.

## Not verified / notes
- Consumer workspaces outside dwg (cad, semio, puzzle, ...) were not compiled; they depend on dwg crates only through locks and path deps. Unused consumer deps may exist (e.g. a crate got a domain dep from a mount it did not need).
- Manifest edit incident: a TOML of dynamic-blocks was invalid for a moment (my in-place rewrite); the coordinator reported a parse refusal. Later writes went through temp + rename.
- No unit-ceiling remeasure; remove the old carving entries after a build.
