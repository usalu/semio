# Course Runs Target Independent Audit

Observation: 2026-10-20T22:29:40.560Z. Bounded static text/metadata audit only. No Source execution, native commands, test collection, pass claim, Docker/private effect or process polling. No Source edits.

## Finding

No concrete target spelling or declaration mismatch was found. The current project declaration names `@semio-tech/course-tests-runs-rs` and declares no targets. Its adjacent Cargo manifest declares concrete package `semio-framework-course-tests-runs`, a `[lib]` with `path = "../../🦀️.rs"`, and no `test = false` override. The declared library file exists, is gated by `#![cfg(test)]`, and mounts lifecycle and views modules. The root workspace includes this exact companion-runs package path.

The current native inference implementation is stronger evidence than mere Cargo naming convention: `🧩️component-targets/🟨️.mjs:52–64` returns build/check/test leaves for an adjacent concrete Cargo package and constructs the test command from this manifest. `🔎️owner-discovery/🟨️.mjs:163–165` merges those inferred leaves into authored project declarations; `🧱️project-assembly/🟨️.mjs:297` also merges Cargo leaves before authored targets. Since this project has no authored targets, no override suppresses its inferred `test` leaf. The package name and project name need not match: the project provides the requested graph identity and the generated command selects the Cargo manifest. Thus current static declarations and inference imply `@semio-tech/course-tests-runs-rs:test`.

## Remaining Uncertainty

Static implication is not actual graph admission or execution. Current native owner/discovery admission, dependency/tool availability, compilation, actual collection and terminal results still require the existing authorized native run. This audit neither resolves that runtime requirement nor claims suite7 passed. The prior Current21 note is conservative but its target-existence inference can now cite the concrete current inference code above.

## SHA256 Observations

Paths below are relative to `/Users/ueli/Documents/semio`. Reads were sequential and are observations, not an atomic Source seal.

| Path | SHA256 |
| --- | --- |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/TEACHING-ARCHITECTURE-DEPLOY-READY/2026-10-21-current21-bindings.md` | `da711ddd68d0d5dbcc7300f60d70409b142625ac8e78d39ed3f8203ca4d261e8` |
| `🧰️framework/🛍️products/📚️course/🧪️tests/🎞️companion-runs/📦️packages/🦀️rust/Cargo.toml` | `020116a999975031a0aba6310eeba06adc22693774002b5ea49e545499671c35` |
| `🧰️framework/🛍️products/📚️course/🧪️tests/🎞️companion-runs/📦️packages/🦀️rust/📋️project.json` | `8abae8bb4b3b453770db876de1ea3d3839d10ef487961d02c4db6ac1f1eb27df` |
| `Cargo.toml` | `32adf4589be8502aa63dee245c500d5837d2495eaeaaec8a2a4c9f97f292f259` |
| `🧰️framework/🛍️products/📚️course/🧪️tests/🎞️companion-runs/🦀️.rs` | `a80735998c0dfc2ab5968bb18ae87e446576c9baf18e71c1caa8b8e1e9236dfc` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧩️component-targets/🟨️.mjs` | `59a4895bcb699cbb5b5862f38385b94b788e3b2d80fadfe5a0446b2b5352a9e5` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🔎️owner-discovery/🟨️.mjs` | `d443dbb08911673dd77d72c69535457690ef6660b9b472b616b2a604de2c6d1c` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧱️project-assembly/🟨️.mjs` | `76c1ba05406d8882f918558d6034b28737a87cbdfc90f977eb9af1399b89fb5b` |
