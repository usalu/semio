STATUS: GREEN for non-pdf, non-IFC stdio crates: `stdio3-hub-1` (C/🗑️generated/1101/stdio3-hub-1.txt, rc 1) shows docx-base-model, tiff-mutation-regressions and png-tests checked with zero errors; the only remaining stdio errors (14) are in `stdio-pdf-schema-4` (stdio2). Other red crates in that run: energy-model-editor/viewer (not stdio).

## Edits
- docx base `🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs` `retire_sqlite_snapshot`: `.result.unwrap_or_else`, `close_step(grant).result` (error -> panic, `.expect` removed).
- tiff `✏️editor/🎭️modes/✏️edit/🎮️commands/🎨️paint-region/🦀️.rs` close_step: closure returns `RetainedWorkOutcome{result, work}` (same as png patch-pixel-region).
- png-tests: `paint-native-region` and `patch-pixel-region` were already ported by a peer at 12:20; not touched.

## Why test/regression crates are in the closure
`stdio-png-tests` and `stdio-tiff-mutation-regressions` are not tests: they are path-preserving carves holding production editor/definition/io-registry code. `stdio-png` and `stdio-tiff` list them as normal `[dependencies]` and re-export from them (`semio_s_artifact_stdio_png_tests::{definition,...}`, `..tiff_mutation_regressions::{io_registry,editor,...}`). Misnomer, no dev-dependency leak; renaming would touch ~65 lockfiles (coordinator-only `dependencies lock`), so left as is.
