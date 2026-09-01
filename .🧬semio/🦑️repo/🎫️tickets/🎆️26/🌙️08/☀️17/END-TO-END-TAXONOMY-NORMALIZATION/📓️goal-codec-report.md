# note-oracle-codec Rust Restructure — Report

Moved all 6 `src/*.rs` to `<emoji><name>/🦀️.rs` module dirs, deleted `src/`: `dxf_codec.rs`→🖊️dxf-codec (fileKind `drawing-2d-model`=🖊️ for .dxf/.dwg), `svg_codec.rs`→🔣️svg-codec (fileKind `svg`=🔣️, the corrected emoji per this ticket's own fix), `pdf_codec.rs`→📕️pdf-codec (fileKind `pdf`=📕️), `recipes.rs`→🧫️recipes (reuses `fixtures` kind's emoji — recipes.rs IS the fixture/before-after manifest module), `cli.rs`→⌨️cli (matches the 32 existing on-disk `⌨️cli/` uses and the `members-of-modules` allowlist exactly). `main.rs`→crate-root `🦀️.rs` (bare, per `rust-binary-entry`), mirroring the already-migrated `🦀️rust/🦀️.rs` pattern where dir+leaf share 🦀️.

Registered 5 new `semanticDirectoryKinds` in taxonomy.json, each `parentKindIds: ["generator-crate"]` — scoping avoids the exact hazard in `goal-final-status.md` §4 (generator-crate's catch-all `^[a-z0-9]+(?:-[a-z0-9]+)*$` would otherwise ambiguously match these same slugs under the sibling `generator` parent). `validateTaxonomy` = 0 problems after the edit.

Cargo.toml `[[bin]] path` → `"🦀️.rs"`. Crate-root `🦀️.rs` module decls now use `#[path = "<dir>/🦀️.rs"] mod <name>;` (precedent: `📡️replication/📡️wire/🏠️local-interaction/🦀️.rs`) — all `crate::`-qualified `use` statements in the 5 submodules untouched, no `include_str!`/path literals existed to fix.

`cargo build`: **Finished `dev` profile [unoptimized + debuginfo] target(s) in 56.71s** — clean, zero errors/warnings. Cargo.toml/Cargo.lock left in place per fixedFilenameContracts.
