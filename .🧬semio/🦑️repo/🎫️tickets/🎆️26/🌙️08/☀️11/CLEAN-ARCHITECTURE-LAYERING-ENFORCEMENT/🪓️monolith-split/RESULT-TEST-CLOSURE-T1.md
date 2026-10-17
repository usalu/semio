# 🧪️ Result TEST-CLOSURE-T1 (slice `tc1`, 2026-10-17)

Executor: Sonnet 5.5, foreground, no sub-agents, no ticket tool, no modifying git command. `F` = this folder. Tools: `F/tc1-tools/`. Logs and census outputs: `F/🗑️generated/tc1-*`.

## 0. Verdict

**DONE (static), UNCOMPILED.** All nine target crates are inside the source gate for their test closure on all three labels (native, wasm32-unknown-unknown, wasm32-wasip2), by the scoped `tc1-census.ts` run (`tc1-census-after3`) and by a scoped copy of `cgc7-census.ts` (`tc1-tools/tc1-cgc7.ts`, `ONLY=` the touched crates; result: 0 oversized, 0 reader-refused, `tc1-cgc7-crate-census.csv`). Test closure of the worst crate fell from 66.9x to 0.97x. `-pdf-test-oracle` is no longer reader-refused. Nothing was compiled (`COMPILE ADMITTED` is not in `📓️FLEET.md`); the scoped rustc differential (7 oracle crates, pptx facade, ply-inferences) shows 0 new rows. Open: `ply-inferences` stays at 31/32 files because its lib alone is 30 files (section 4); the new companion crate is unverified by any compile (section 5).

## 1. Baseline (before the first edit, `tc1-census-baseline`, native; wasm labels identical)

`F/tc1-tools/census.sh <tag>` runs `tc1-census.ts` (copy of `rtb-census.ts`, outputs `tc1-census-*`) over the nine crates (`F/tc1-tools/manifests.txt`).

| crate | files | bytes | lines | ratio |
|---|---:|---:|---:|---:|
| tiff-test-oracle | 12 | 17,545,228 | 1,069 | 66.93 |
| ifc-test-oracle | 19 | 3,483,962 | 3,032 | 13.29 |
| mp4-test-oracle | 3 | 2,746,784 | 504 | 10.48 |
| jpg-test-oracle | 5 | 520,017 | 715 | 1.98 |
| pptx-test-oracle | 19 | 504,458 | 1,645 | 1.92 |
| docx-test-oracle | 16 | 324,036 | 2,023 | 1.24 |
| pptx (integration `model_laws`) | 30 | 292,611 | 855 | 1.12 |
| ply-inferences | 33 | 209,841 | 3,062 | 1.03 (files) |
| pdf-test-oracle | refused | | | |

Compile: `COMPILE ADMITTED` is not in `📓️FLEET.md`; nothing is compiled.

## 2. Approach

No generic Rust test-support reader exists for the stdio packages (the repo-text reader lives in the renderer test-support crate, and `semio-repo-test-host` is scanned by many TS laws, so it was not touched). The stdio precedent is `runtime_fixture_text` (`CARGO_MANIFEST_DIR` join, e.g. zip/opc, epw). Each oracle artifact root `<artifact>/🔮️oracles/🦀️.rs` got a `#[cfg(test)] pub(crate) mod runtime_fixture` (`bytes`, `text`, paths relative to the artifact folder, manifest dir plus `../../..`); the pptx facade `model-laws` integration target got the same module inline (`../..`). Test units replace `include_bytes!`/`include_str!` by `crate::runtime_fixture::*` reads (consts became cached `fn fixture() -> &'static [u8]` over `OnceLock`). Fixture files are untouched. Originals: `F/tc1-tools/orig/` (`index.tsv`), site inventory before/after: `F/🗑️generated/tc1-sites-before.tsv` (50 sites, with sha256) and `tc1-sites-after.tsv` (0 sites).

pdf-test-oracle refusal: `include_str!(concat!(..., $mutation, ...))` in the 4.1.4 base unit (computed include path, the reader refuses it). That path also lacked a `/` before `$mutation`, so it could never have compiled; the run-time read uses the correct path (all 25 files exist).

## 3. Census after (native; `tc1-census-after2`)

| crate | files | bytes | lines | ratio |
|---|---:|---:|---:|---:|
| tiff-test-oracle | 6 | 68,648 | 1,091 | 0.27 |
| ifc-test-oracle | 12 | 168,345 | 3,066 | 0.77 |
| mp4-test-oracle | 2 | 28,533 | 518 | 0.13 |
| jpg-test-oracle | 4 | 37,302 | 729 | 0.18 |
| pptx-test-oracle | 14 | 94,153 | 1,676 | 0.44 |
| docx-test-oracle | 11 | 107,930 | 2,054 | 0.51 |
| pdf-test-oracle | 21 | 180,451 | 3,049 | 0.76 (was refused) |
| pptx `model_laws` | 8 | 64,466 | 873 | 0.25 |
| ply-inferences | 33 | 209,841 | 3,062 | 1.03 (open) |

## 4. Per crate: before and after (native; wasm labels identical)

| crate | before (files / bytes / lines, ratio) | after | what changed |
|---|---|---|---|
| tiff-test-oracle | 12 / 17,545,228 / 1,069 (66.93) | 6 / 68,648 / 1,091 (0.27) | 17.4 MB scan + 4 small tiff/json now read at run time |
| ifc-test-oracle | 19 / 3,483,962 / 3,032 (13.29) | 12 / 168,345 / 3,066 (0.77) | 5 units: 4 ifc fixtures (2.5 MB + 3 x 194 KB), 2 `.feature` files |
| mp4-test-oracle | 3 / 2,746,784 / 504 (10.48) | 2 / 28,533 / 518 (0.13) | 2.7 MB mp4 |
| jpg-test-oracle | 5 / 520,017 / 715 (1.98) | 4 / 37,302 / 729 (0.18) | 483 KB scan |
| pptx-test-oracle | 19 / 504,458 / 1,645 (1.92) | 14 / 94,153 / 1,676 (0.44) | 3 x 112 KB pptx, 60 KB catalog json, feature |
| docx-test-oracle | 16 / 324,036 / 2,023 (1.24) | 11 / 107,930 / 2,054 (0.51) | 3 docx, 136 KB catalog json, feature |
| pdf-test-oracle | refused | 21 / 180,451 / 3,049 (0.76) | 6.3 MB thesis pdf (11 units), 11 features, 30 vector jsons; the computed `include_str!(concat!(..))` path is gone |
| pptx (integration `model_laws`, worst unit) | 30 / 292,611 / 855 (1.12) | 8 / 64,466 / 873 (0.25) | 4 test units of the base schema: 112 KB pptx, 16 facet texts, demo assets, 3 jsons |
| ply-inferences | 33 / 209,841 / 3,062 (1.03) | 31 / 206,248 / 2,989 (0.97) | `bounds` and `inferences` unit laws moved to the new companion crate `semio-s-artifact-stdio-ply-inferences-tests` (3 / 3,900 / 78, 0.09) |

Own sources of the touched units are unchanged except the include sites; fixtures are byte-identical (sha256 check: 45 fixture sites, 0 differing, `tc1-tools/verify.py`).

## 5. What changed, exactly

- `<artifact>/🔮️oracles/🦀️.rs` of tiff, ifc, mp4, jpg, pptx, docx, pdf: `#[cfg(test)] pub(crate) mod runtime_fixture` (`bytes`, `text`; mp4 and jpg only `bytes`), manifest dir plus `../../..`.
- 25 oracle test units (ifc 5, tiff 2, mp4 1 lib-embedded tests module, jpg 1, pptx 3, docx 3, pdf 10): every `include_bytes!`/`include_str!` (50 sites, 5 of them `concat!` in pdf 4.1.4 base) replaced; consts became cached `fn fixture()`/`feature()`/`scan()` over `OnceLock`; pdf 1.7 units moved the documented `FIXTURE` into the existing `fn fixture()`. Originals: `tc1-tools/orig/` (`index.tsv`).
- pptx facade: `🧪️tests/🔬️model-laws/🦀️.rs` carries `mod runtime_fixture` (`../..`); the four mounted base-schema units (`🧪️tests/🔬️unit`, `📸️snapshot/🧪️tests/🔬️shadow`, `🧭️transform/🧪️tests`, `💡️inferences/🧾outline/🧪️tests/🔬️unit`) read at run time.
- ply: new crate `semio-s-artifact-stdio-ply-inferences-tests` at `…/🧬️schema/💡️inferences/🧪️tests/🔬️unit/📦️packages/🦀️rust` (Cargo.toml, `🦀️.rs` mounting the two existing unit files with `#[path]`, root re-exports `schema`, `standards`, `PlySnapshot`, `STDIO_PLY_DOCUMENT_SCHEMA` so the unit files keep their `crate::` paths). Registered: `🧱️ply/Cargo.toml` members, `🧱️ply/📦️packages/🦀️rust/📜️script.ts` package list, `📋️build-owners.json` row (after `ply-inferences`), `🧱️ply/Cargo.lock` entry. The two `#[cfg(test)] mod tests;` mounts were removed from `💡️inferences/📦bounds/🦀️.rs` and `💡️inferences/🦀️.rs`. The facade does not depend on the companion (a dependency would change the lock of 10 consuming workspaces). No `.vscode/launch.json` change: stdio artifact launch routes are per artifact (`@semio-tech/stdio-ply-rs:test*`), and the companion runs under them through the script's package list.
- The diff-codec law stays in `ply-inferences`: it reads `pub(crate)` `cfg(test)` helpers (`demo_diff_cases`, `sweep_a`, `sweep_b`), so moving it would need a `testkit` feature and widening in carved lib files.

## 6. Evidence (commands run)

- Census: `F/tc1-tools/census.sh baseline|after1|after2|after3` (`bun tc1-census.ts`, native + both wasm labels, `DUMP_INPUTS`); `bun tc1-cgc7.ts` with `ONLY=` (exit 0, 0 oversized, 0 refused).
- `python3 tc1-tools/verify.py`: test-name multisets 39 files / 146 tests before and after, 0 differing; fixture sha256 45 sites, 0 differing; `rustfmt --edition 2021 --emit stdout` parse of the 39 touched files plus the companion root, 0 failing.
- Scoped differential (`tc1-tools/oracle.py`, rustc nightly-2026-07-20, `--test`, `CARGO_MANIFEST_DIR` set, baseline sandbox with originals restored, sandboxes under `/Users/ueli/.cache/tc1/`): tiff 35/35, mp4 18/18, jpg 21/21, ifc 35/35, pptx-oracle 38/38, docx 41/41, pptx model-laws 28/28 with 0 new and 0 gone; pdf 114 base / 89 now with 0 new and 25 gone (the 25 are exactly the 5 x 5 `couldn't read` errors of the `concat!` path, which lacked the `/` before `$mutation`: the base unit could never compile, so the pdf-test-oracle refusal was a real defect); ply-inferences (`PATCH=1`, extern crates removed) 475 base / 469 now, 0 new, 6 gone (the unmounted units). Negative control: renaming `runtime_fixture` in one tiff unit gave 8 new rows.
- Limits: the differential cuts off before type checking (missing external crates), so `Vec<u8>` vs `&[u8]` adaptations (for example `project_tiff(&expected)`, `&fixture_pack`, `for text in &vectors`) were reviewed by hand, not checked; the companion crate cannot be checked by the oracle (its dependency is another crate).

## 7. For the compile queue

```
cd /Users/ueli/Documents/semio && bun repo cargo --store=shared --configuration=prototype -- test -p semio-s-artifact-stdio-<tiff|ifc|mp4|jpg|pptx|docx|pdf>-test-oracle --no-run
cd /Users/ueli/Documents/semio && bun repo cargo --store=shared --configuration=prototype -- test -p semio-s-artifact-stdio-pptx --test model_laws --no-run
cd /Users/ueli/Documents/semio && bun repo cargo --store=shared --configuration=prototype -- test -p semio-s-artifact-stdio-ply-inferences -p semio-s-artifact-stdio-ply-inferences-tests --no-run
```
Expect to check first: ply companion (names and `protocol` alias), pdf 4.1.4 base `vector` closure types, ifc `assert_ne!(output, fixture(), ..)`. `Cargo.lock` of the ply workspace was edited by hand (one new package entry); refresh with `update --workspace --offline` when the launcher admits. The ply-inferences lib itself is 30/32 files (a path-preserving mega-crate); further headroom needs a re-carve by the generator, not a test move.
