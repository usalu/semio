# 🧩 Result: IconRenderHost Generated Input Removed From `semio-framework-os-renderer-wgpu`

The compiler-ingress refusal `Generated Rust input requires compiler ownership` for this crate is gone. The icon bindings are now a checked-in, schema-first generated source file owned by the General Assets bundle. `build.rs` no longer generates anything and no `OUT_DIR` include remains in the crate.

## What was generated, from what

Old producer: `🧊️wgpu/🏗️builder/🦀️.rs`, reached through `📦️packages/🦀️rust/build.rs` -> `🏗️builder/🦀️.rs` -> `🧊️wgpu/🏗️builder/🦀️.rs`.

- Input: the 250 handpicked source SVGs under `🧰️framework/🔨️modules/🖼️assets/🔣️icons/` (public icon id = file stem minus the leading emoji) plus the 682 byte engine logo `🧑‍🎨engine/🔣️.svg`.
- Output in `OUT_DIR`: a byte copy of every SVG and of the logo, plus `🧩️icons.rs` with `pub const SEMIO_LOGO_SVG` and `pub static ICON_SVGS: &[(&str, &str)]` (sorted by id) built from `include_str!` of those copies.
- It also wrote `semio-runtime-resource-inputs.jsonl` (build resource observations). The native-build provenance reader treats a missing file as "no resources" (`continue` at `native-build/🟦️.ts:637`), so removing the producer is safe.

Decision: this is an index of `include_str!` paths over already checked-in SVG sources, not embedded data. Total embedded SVG payload is 82,574 bytes (250 icons plus logo). It is therefore a source-file case (not the runtime asset channel of `RESULT-CARVE-COMPILER.md`). The SVG bytes keep being embedded by `include_str!` straight from the checked-in sources, so no copy, no new asset channel and no behaviour change.

## Changes

1. Generator (the existing assets codegen, `@semio-tech/assets`):
   - `🔣️icons/🏗️builder/📽️projection/🟦️.ts`, `renderRust`: emits one more artifact `🔣️icons/🤖️generated/🧩️icon-embeds/🦀️.rs` (`pub static ICON_SVGS`, ids sorted, `include_str!("../../<source path>")` of the RAW sources, not the normalised `🖼️icon_svgs` copies, so the bytes the renderer rasterises are unchanged).
   - `🏗️builder/📦️publication/🟦️.ts`: `🧩️icon-embeds/🦀️.rs` added to `assetOutputRoots()`, so it is part of the output manifest, membership check, pruning and `preview-generated`.
   - `📦️packages/🟦️typescript/📋️project.json`: the file is a declared Nx output of the `build` target.
   - Drift is caught by the existing `check-generated` command (`checkAssetArtifacts`: exact content and exact membership of every owned root). Before the file was written the check reported `files=["🔣️icons/🤖️generated/🧩️icon-embeds/🦀️.rs"]` stale with `membership=true`; after writing it passes (`check ok`, 261 deterministic outputs).
2. Generated file: `🧰️framework/🔨️modules/🖼️assets/🔣️icons/🤖️generated/🧩️icon-embeds/🦀️.rs` (254 lines, 20,119 bytes, rustfmt-clean, repo-relative path 105 bytes). Docstring opens with `//! 🧩️`.
3. Consumer: `🧱️elements/🖼️IconRenderHost/🎯️targets/🧊️wgpu/🦀️.rs`: the `include!(concat!(env!("OUT_DIR"), …))` line is replaced by
   - `pub const SEMIO_LOGO_SVG: &str = include_str!("../../../../🔣️.svg");` (manifest-relative include of the checked-in logo; the logo is not generated), and
   - `#[path = "../../../../../../../../../🔨️modules/🖼️assets/🔣️icons/🤖️generated/🧩️icon-embeds/🦀️.rs"] mod icon_embeds; pub use icon_embeds::ICON_SVGS;`.
   The public surface of `icon_atlas` is identical (`ICON_SVGS`, `SEMIO_LOGO_SVG`).
4. Build step: `🧊️wgpu/🏗️builder/🦀️.rs` (included by `build.rs` and the catalog-declared adapter) is reduced to the Windows 8 MiB stack link-arg for `semio-wgpu-native`. `build.rs` itself stays: it still carries that link-arg, and the package catalog (`🪪️package-catalog.json`, `cargoBuild: "build.rs"`) and the adapter chain are unchanged. The `ResourceObservationsV1` mount, `OUT_DIR` copies and `cargo:rerun-if-changed` lines are gone (cargo tracks `include_str!` files through dep-info).
5. Old builder preserved for reference: `🗑️generated/icon-old-builder.rs.txt`.

## Proof

| Check | Command / method | Result |
|---|---|---|
| Old generator logic, run in isolation | `🗑️generated/icon-old-oracle.ts` is a line-by-line port of the old `catalog_sources` + `main` (byte-order `BTreeMap`, `trim_start_matches` id rule, same assertions) over the real icon tree. It writes the old `OUT_DIR/🧩️icons.rs` text to `🗑️generated/icon-old-OUT_DIR-icons.rs.txt`. | 250 entries |
| Byte identity | The same script parses the new checked-in file, resolves every `include_str!` against it and compares to the old output | ids and order equal; all 250 SVG byte contents equal; entry lines equal modulo the path prefix; logo bytes equal (682); `#[path]` resolves to the generated file; total embedded 82,574 bytes |
| Census | `cd 🗑️generated && ONLY=semio-framework-os-renderer-wgpu bun census.ts` (`icon-census.log`) | `error: ""`, `withErrors: 0` (was `Generated Rust input requires compiler ownership`). Still oversized: 397 files, 5,962,702 bytes, 102,088 active lines, `worstRatio` 25.5, binding = lines; `embeddedFiles` 261, `embeddedBytes` 164,569 |
| Generator freshness | `bun 🗑️generated/icon-gen-driver.ts check` (calls the real `checkAssetArtifacts()`) | `check ok 261` |
| Generator via launcher | `bun repo run @semio-tech/assets:check-generated` | not obtained, see "Open items"; the identical `checkAssetArtifacts()` passed via the driver |
| rustfmt parse | `rustfmt --edition 2021 --check --config skip_children=true` on the IconRenderHost file; `rustfmt --check` on the generated file and the slim builder | exit 0, no parse errors |
| Full-resolution differential, native lib | `🗑️generated/icon-oracle.py`: sandbox A = current engine tree (symlinked siblings, engine copied), sandbox B = identical copy with the three edits reverted to the old `include!(OUT_DIR…)` and a real `OUT_DIR` holding the old generator output (`🧩️icons.rs`, `🪧️semio_logo.svg`, `🔣️icons/…` copies). Both get the sandbox patches of `📓️RESOLUTION-ORACLE.md`. `rustc +nightly-2026-07-20 --edition 2021 --crate-type lib --emit=metadata -o /dev/null 📚️library/🦀️.rs` | A: 6155 errors, B: 6155 errors, `onlyNew=0 onlyOld=0` (errors are the unrelated missing external crates and `infinite_world::world` of the other fleet carves; no error mentions `icon_embeds`, `ICON_SVGS`, `SEMIO_LOGO_SVG`, E0583 or a missing file; the only textual difference in the two `.err` files is shifted line numbers) |
| Full-resolution differential, `--test` | same sandboxes, same command plus `--test` (`icon-oracle-run.py test A|B`, `icon-oracle-parse.py`) | A: 1650 errors, B: 1650 errors, `onlyNew=0 onlyOld=0` (sandbox-specific absolute paths inside messages are normalised; no error mentions `icon_embeds`, `ICON_SVGS` or `SEMIO_LOGO_SVG`) |
| Full-resolution differential, `--target wasm32-unknown-unknown` | same sandboxes, same command plus `--target wasm32-unknown-unknown` | A: 5695 errors, B: 5695 errors, `onlyNew=0 onlyOld=0` |

Logs in `🗑️generated/`: `icon-oracle-lib.log`, `icon-oracle-test.log`, `icon-oracle-wasm.log` (see open items), `icon-census.log`. Scripts: `icon-old-oracle.ts`, `icon-gen-driver.ts`, `icon-oracle.py`, `icon-oracle-run.py`, `icon-oracle-parse.py`, `icon-oracle-bg.sh`, `icon-checkgen-loop.sh`.

## Open items

- The `--test --target wasm32-unknown-unknown` differential configuration was not run (the host was at load average 50-65 for the whole session; each configuration took 3 to 35 minutes). Native lib, native `--test` and wasm32 lib are identical between old and new.
- `bun repo run @semio-tech/assets:check-generated` through the repo launcher: not obtained: 8 attempts over about 60 minutes (`icon-checkgen-loop.sh`) all failed in the repo graph layer before the target ran (the launcher failed repeatedly with `Repository target execution contract refused`, `Original selected owner registry changed` and `Active graph readers retain required Nx pool headroom` because of concurrent fleet churn on the build-graph owner registry; none of these are caused by this change).
- The crate stays oversized (about 25x the line limit). Option 1 of `AUDIT-REFUSED-AND-PLUGIN-GATE.md` only removes the refusal; the carves of `IconRenderHost`, `Scenes`, `NodeGraph` and shell sources remain. When `IconRenderHost` becomes its own crate, it needs only the generated file (82 KB of embeds, far below 262,144 bytes) and the logo.
- The peer working in the same projection file (runtime icon pack `🔤️shortcodes/🔣️.json`) was not touched; the two additions are independent and `check-generated` is green with both.
