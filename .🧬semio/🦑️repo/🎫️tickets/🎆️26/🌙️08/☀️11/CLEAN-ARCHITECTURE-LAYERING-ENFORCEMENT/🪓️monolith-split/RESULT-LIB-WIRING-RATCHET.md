# Result LIB-WIRING-RATCHET (slice `lwr`)

Status: DONE. Executor: Sonnet 5.5, foreground, no sub-agents, no modifying git command, no ticket tool, no cargo. `F` = this folder. Tools: `F/lwr-tools/`. Logs: `F/🗑️generated/lwr-*`.

## 0. Verdict

`lib-wiring-budget`: before 1 pass / 2 fail, after 3 pass / 0 fail (`lwr-wiring-before.log`, `lwr-wiring-after.log`). No pin was raised.

## 1. Failing rows before (`lwr-wiring-before.log`, `lwr-rows-before.log` from `lwr-tools/rows.ts`)

Counts are the test's `split("\n").length`.

| Root | Pin | Count | Cause (git log of the root is peer auto-commits only; `git diff HEAD` shows the real edit) |
| --- | ---: | ---: | --- |
| lowpoly `✏️editor/⚙️engine/🦀️.rs` | 495 | 496 | peer added the one-line `use semio_framework_3d::mesh::{MeshModelingOps};` (mesh-modeling trait migration) |
| procedural generation3d `✏️mesh-edit/🦀️.rs` | 153 | 154 | same peer migration, same import line |
| layout `📤️export/🔣️json/🦀️.rs` | 1879 | 1898 | slice `ldd` (drawing decouple): `is_drawing`, the cached `drawing` scene field and `drawing_scene` (+19 net) |

Stale pins (the third test: a pinned root that complies, vanished or is unmeasured): at my first run the root list was din18599, din16798, en1992 and iso16757 `⚖️checks`; a later scan also showed the generation3d host `✉️envelope` (83 lines) and en1998 `inferences` (a peer removed the en1998 entry meanwhile). The remaining roots in the task's "7 stale" were shrunk-below-pin rows (`LOWER`), which do not fail the law but are ratchet debt (process3d editor 1948 -> 479, etc.).

## 2. Changes

### 2.1 Slimmed roots (names unchanged, pins not raised)

| Root | Before (test count) | After | Child | Moved |
| --- | ---: | ---: | --- | --- |
| lowpoly engine `E/⚙️engine/🦀️.rs` | 496 | 451 | `⚠️errors/🦀️.rs` (new) | `//#region ⚠️ Errors` verbatim: `LowpolyCoreError`, `Display`, `Error`, `From<MeshKernelError>`; root keeps `#[path]` mount plus `pub use errors::LowpolyCoreError;` and drops the now unused `MeshKernelError` import |
| procedural mesh-edit `M/🦀️.rs` | 154 | 145 (complies, 150 budget) | `🎯️element-picks/🦀️.rs` (new) | the `edges`, `vertices`, `faces` selection helpers (`pub(super)`); root drops the `picked`, `EdgeId`, `VertexId` imports |
| layout export json `J/🦀️.rs` | 1898 | 1457 | `✅️json-validation/🦀️.rs` (new, 447 lines) | the self-contained JSON validation state machine: the nine private state enums, `JsonValidationCursor` and its impl, plus `JSON_INPUT_BYTES_PER_UNIT` and the `FromValue/ToValue` derive import; root keeps the MAX consts (child reads them via `use super::{..}`), `pub use json_validation::JsonValidationCursor;` in the existing `🪢️TaxonomyMounts` region |

The validation block had no reference from the rest of the json root (verified by the assertions in `carve_layoutjson.py`: none of the moved names occur in the remaining text), so no member needed widening. `JsonValidationCursor` is still exported at the crate root, so `semio_s_artifact_layout_layout_engine_json::JsonValidationCursor` is unchanged. Pre-existing and not mine: the export crate's unit test touches private `JsonContainer`/`.stack` and calls the private `JsonValidationCursor::new`; both were already cross-crate breaks before the move (the json crate was carved into its own package earlier).

### 2.2 Ratchet fixture `🧪️tests/🧫️fixtures/📏️lib-wiring-ratchet/🔣️.json` (library module)

Applied with `lwr-tools/ratchet.ts --apply` (read, compute, re-read guard, atomic rename; JSON round-trip is byte-identical, so no format churn):

- Removed (root now complies or vanished): generation3d mesh-edit (145), generation3d host `✉️envelope` (83), norm din18599 (41), din16798 (36), en1992 (43), iso16757 part1 (84).
- Lowered to the current count: process3d editor 1948 -> 479, lowpoly engine 495 -> 451, layout export json 1879 -> 1457, layout export 2531 -> 2525, layout editor 2006 -> 2000, layout io 749 -> 643, layout schema 1012 -> 997, layout root 184 -> 183, norm en1997 2312 -> 173, playbook 823 -> 231, procedure 737 -> 212, energy viewer 508 -> 394, energy session 1619 -> 319, stdio gif 509 -> 209, binary 375 -> 169, md 423 -> 159, png 463 -> 165, txt 363 -> 164, stl 427 -> 164, semio 413 -> 412, puzzle 2d 1491 -> 1488.
- 343 entries untouched, 0 grown rows left.

Paths: `E` = `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor`; `M` = `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/✏️mesh-edit`; `J` = `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🔣️json`.

## 3. Verification (all real runs; logs in `F/🗑️generated/lwr-*`)

| Check | Command | Result |
| --- | --- | --- |
| The law, before | `bun test ./🧪️tests/📏️lib-wiring-budget/🟦️.ts` in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library` | 1 pass, 2 fail (3 grown rows, 4 stale pins), `lwr-wiring-before.log` |
| The law, after (run twice, the second after all edits) | same | 3 pass, 0 fail, 5 expect() calls, `lwr-wiring-after.log`; `lwr-tools/rows.ts` lists 0 grown, 0 unpinned-over-budget, 0 stale, 0 lowerable rows (`lwr-rows-after.log` is empty) |
| Lossless line multiset (`lwr-tools/lossless.py`, originals in `lwr-tools/orig/`) | stripped non-empty lines, original against root plus child | lowpoly: 1 removed (the trimmed import), 7 added (docstring x2, mount x3, two import lines); mesh-edit: 5 removed (3 fn heads, 2 imports) and 12 added (3 `pub(super)` twins, mount x3, docstring, 5 imports); layout json: 0 removed, 5 added (docstring, mount x3, one `use super::{..}`). No body line is lost (`lwr-lossless.log`) |
| Typeck oracle (`pos-tools/pos.py`, `POS_PREFIX=lwr`, family `lwr-tools/lwr.json` = the three crates, lib and `--test`) | `pre` on the unedited tree, `post` after the edit, `lwr-tools/posdiff.py pre post lib test` | canary reached (`E0308, E0616, E0624`) in both; lib 438 = 438 rows, test 536 = 536 rows; **`POSDIFF IDENTICAL`, 0 new rows, 0 lost rows**; privacy rows 0. Pre-existing noise on both sides (not mine): `dictionary` child names the root-private `JsonStringWriteCursor` (4 resolve rows), mesh-edit `!`-typed iterator rows from the stubbed `MeshModelingOps` trait |
| Negative control | `neg` run with three sandbox mutations (`edges` un-widened, `JSON_INPUT_BYTES_PER_UNIT` renamed, `MeshKernelError` renamed in the lowpoly child), `posdiff.py post neg lib` | 6 new rows: E0603 `edges` is private, E0425 `JSON_INPUT_BYTES_PER_UNIT`, E0425 `MeshKernelErrorX`, plus E0599 rows from the error-typed receivers; the oracle sees all three children (`lwr-posdiff-neg.log`) |
| rustfmt | `rustfmt +nightly-2026-07-20 --edition 2021 --check` on the 3 children and the 3 roots | 0 parse errors on all six; formatting hunks: lowpoly `⚠️errors` 1 (the verbatim `Self::Managed(error)=>write!(..)` line, pre-existing text), the other two children 0 (`lwr-rustfmt.log`) |
| Paths and taxonomy | byte length of the 3 child files; `bun wbr-tools/taxcheck.ts` | 170, 224, 203 bytes (limit 240); folders are `<RGI emoji>` + U+FE0F + kebab, each emoji already a folder prefix in the repo and unique among siblings (`⚠️`, `🎯️`, `✅️`); 3 of 3 resolve to `crate-module-children`, 0 unresolved, 0 ambiguous (`lwr-taxcheck.log`); each child starts with a `//!` docstring whose first character is its own emoji; no comment inside any definition was added; 0 `[DEBUG]` markers; no `*.lwr-tmp` leftovers |
| Source-text readers | `git grep` over ts/tsx/json/py/toml outside tickets | no reader names the three children or depends on the old root text; the roots appear only in the ratchet fixture, `📋️build-owners.json`/`🔒️dependencies.json` (package-level rows) and the peer lists `📚️consumers.json`/`📥️callers.json` of the mesh-modeling trait migration (plain file lists, not read by any test; the mesh-edit and engine roots keep their paths) |

Not run: any cargo build (compiler admission is closed for this fleet). The type-level proof is the differential oracle above. Real-check command for the compile queue, one launcher call:

```
cd /Users/ueli/Documents/semio && perl -e 'alarm 3500; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype -- check -p semio-s-artifact-lowpoly-lowpoly-engine -p semio-s-artifact-procedural-3d-mesh-edit -p semio-s-artifact-layout-layout-engine-json --lib --tests --keep-going --message-format=short
```

## 4. Notes and hand-offs

1. The task text called the 7 failing stale pins "root shrank below its pin". The test's third case actually fails only for roots that comply (<= 150), vanished or unmeasured, so those entries had to be removed, not lowered. The shrunk-below-pin rows do not fail the law; they are lowered here because lowering is always allowed.
2. The peer lists `📚️consumers.json`/`📥️callers.json` name the mesh-edit root as a `MeshModelingOps` consumer; the new `🎯️element-picks` child does not use the trait, so no entry is needed.
3. The `dictionary` child of the layout json crate names the root-private `JsonStringWriteCursor` (pre-existing rows, unchanged by this slice). The export crate's unit test touches private `JsonContainer` and `JsonValidationCursor::new`, also pre-existing.
4. The pins are now tight (equal to the current counts) for the 21 lowered roots; any peer growth of those roots fails the law at once, which is the intent of the ratchet.

## 5. Files

Edited:
- `E/⚙️engine/🦀️.rs` (496 to 451), `M/🦀️.rs` (154 to 145), `J/🦀️.rs` (1898 to 1457)
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧫️fixtures/📏️lib-wiring-ratchet/🔣️.json` (6 entries removed, 21 lowered)

Created:
- `E/⚙️engine/⚠️errors/🦀️.rs`, `M/🎯️element-picks/🦀️.rs`, `J/✅️json-validation/🦀️.rs`
- this file and `F/lwr-tools/` (`rows.ts`, `ratchet.ts`, `carve_lowpoly.py`, `carve_meshedit.py`, `carve_layoutjson.py`, `lossless.py`, `posdiff.py`, `injlib.py`, `lwr.json`, `orig/`)
