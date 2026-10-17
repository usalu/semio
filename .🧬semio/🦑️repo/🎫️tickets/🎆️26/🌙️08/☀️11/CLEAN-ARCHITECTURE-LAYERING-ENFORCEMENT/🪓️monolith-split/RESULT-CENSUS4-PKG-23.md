# Result CENSUS4-PKG-23 (prefix `c4p23`): raster artifact and stdio registry editing family, eight over-1,000-line Rust files

Executor: Sonnet 5.5, foreground, no sub-agents, no modifying git command, no ticket tool. `F` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`. Tools: `F/c4p23-tools/`. Logs: `F/🗑️generated/c4p23-*`. Pre-edit snapshots (byte copies, taken before the first edit): `F/🗑️generated/c4p23-orig/<key>.rs`. Install journal: `F/🗑️generated/c4p23-installed.json`.

## 0. Verdict

**DONE (static evidence; real compile not admitted).** All eight files are split and installed (8 roots rewritten, 35 children created, 27 `#[path]` module children and 8 `include!` test fragments). Lossless, rustfmt-parse clean, region markers kept, taxonomy ok, the scoped patched differential is identical on all six groups and four configurations, the full-crate typeck oracle (`pos`) is identical, the `--check-applied` raster gate equals its baseline, the reader probe shows no change in any Package 23 row. Two follow-up repo edits outside the eight files were needed and are recorded in section 4. No `COMPILE ADMITTED` line existed in `📓️FLEET.md` at 08:07, so the real `bun repo cargo` check is queued (command in section 7).

## 1. Slice

`R` = `✏️s/🔌️plugins`. `RA` = `R/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any`. `ED` = `R/🗄️stdio/📇️registry/🧬️contract/✏️editing`.

| Key | File (relative to `R`) | Lines before | Root after | Children (lines) |
|---|---|---:|---:|---|
| `rsc` | `RA/🧬️schema/🦀️.rs` | 1,276 | 181 | 🌳️layer-tree-helpers 180, 🗺️owned-map 507, 📐️layer-model 182, 🖼️asset-content-and-patch 242 |
| `rdf` | `RA/🧬️schema/🔺️diff/🦀️.rs` | 1,274 | 208 | 🌲️layer-tree-apply 523, 🧮️inverse-and-absorb 384, 🖌️repaint-canvas 180 |
| `rio` | `RA/🚪️io/🦀️.rs` | 1,076 | 235 | 🌉️semio-bridge 212, 🧩️composite-and-media 248, 🎹️derived-composition-and-registry 244, 🏗️derived-construction-and-analysis 166 |
| `rtu` | `RA/✏️editor/🧪️tests/🔬️unit/🦀️.rs` | 1,332 | 140 | 📥️envelope-and-scene-laws 230, 🧭️layer-and-camera-laws 275, 🔁️wire-and-action-bridge-laws 263, 🏔️mounted-boot-laws 435 (include fragments) |
| `edr` | `ED/🦀️.rs` | 1,598 | 284 | 📜️edit-contract 195, 🪡️value-operations 344, 📨️action-bridge 154, 🧰️editor-traits-and-retained-copies 295, 🛠️tool-job-factories 359 |
| `edp` | `ED/🩹️patch/🦀️.rs` | 1,332 | 62 | 📜️patch-wire 170, 🪜️patch-steps 361, 🧭️patch-edits 261, 🧩️inverse-parts 302, 🧬️input-schema 211 |
| `edd` | `ED/🪟️details/🦀️.rs` | 2,032 | 76 | 📐️detail-model 97, 🧬️schema-resolution 483, 🧩️template-synthesis 340, 🗂️dsl-provider 343, 🏷️labels-and-controls 336, 🌲️detail-tree-and-window 399 |
| `edu` | `ED/🪟️details/🧪️tests/🔬️unit/🦀️.rs` | 1,063 | 12 | 🧱️lazy-and-bounded-laws 203, 🌳️collection-and-table-laws 297, 🧬️schema-template-laws 233, 🔀️union-source-and-window-laws 330 (include fragments) |

Largest child 523 lines (limit about 1,000). Children sit next to their parent as `<emoji>️<kebab>/🦀️.rs`; test fragments sit under the test folder (`🧪️tests/🔬️unit/<emoji>️<kebab>/🦀️.rs`). Longest new path 202 UTF-8 bytes (limit 240).

## 2. Found state

- Session 1 (before the restart): nothing had been installed. Tools, plans and a staged tree existed; the typeck oracle had been run on the staged overlay (pre, post, negative control). The old section 0 of this file said "nothing edited", which was true.
- Wave 7 start, verified before the first edit: all eight live files were byte-identical to `F/🗑️generated/c4p23-orig/<key>.rs`, no child folder existed, `c4p23-installed.json` did not exist. Nothing to roll back.
- Session-1 staging defects found by the scoped differential (which session 1 had run but not closed), all fixed in the splitter and re-staged before the install:
  1. `rio` (child `🎹️derived-composition-and-registry`): the splitter hoisted `pub use derived_composition::*;` out of the child into the root while the module `derived_composition` stayed in the child, a real `E0432` in the split tree (`+new unresolved import derived_composition`; the typeck oracle had hidden it as stub noise). Fix in `c4psplit.py`: `use` items whose head names a module defined in the same child stay in the child, and `mod` items now count as exports so the root gets the `pub use <child>::*;`.
  2. `rsc` (`🗺️owned-map`) and `rio` (`🎹️derived-composition-and-registry`): a new `unused import: super::*` warning. These two children use nothing from the root, so the plan has `noglob=True` for them and they carry no `use super::*;`.
  3. `scoped.py` pointed at the wiped scratchpad of session 1; re-pointed to this session's scratchpad.
  4. The earlier `edd test` difference was only a `couldn't read` path text (include fragments gain one `../`); with all four configs it now reports identical counts and identical messages.

## 3. What changed (files)

Created (35 children, listed in section 1) and rewritten (the eight roots). Additional edits:

| File | Change |
|---|---|
| `RA/🧬️schema/🦀️.rs` | descriptor facet leaves (`rust:`) now `concat!(include_str!("🦀️.rs"), include_str!(child)…)` for the schema and, in `RA/🧬️schema/🔺️diff` terms, `concat!(include_str!("🔺️diff/🦀️.rs"), …)` for the diff, so the registered leaf text is the same text as before (line multiset) |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🏁️run/🟦️.ts` | `rasterEnvelopeCodec` reader: the schema root entry is followed by the four new schema children (child-specific read). The law checks `impl Serialize for RasterOwnedMap`, owned-map field sites and similar, which now live in `🗺️owned-map` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧫️fixtures/📏️lib-wiring-ratchet/🔣️.json` | `edp` (now 62 lines) and `edd` (76) comply with the 150-line wiring budget, so their pins are removed; the `edr` pin goes 1599 -> 285 (the lib root is 284 lines plus the final newline) |

Widened items (`pub(super)`): rsc 6, rdf 14, rio 7, edr 23, edp 28, edd 85 (rtu and edu: none, include fragments keep one module). Every removed line in the lossless proof has a twin: widened, re-based `include_str!`/`include!` path (`../` added: rtu 3, edu 10), or the declared descriptor substitution (rsc 2). Test names: rtu 49 = 49, edu 25 = 25.

## 4. Source-text readers

Readers searched with `git grep -F` over the eight file names and their folders (`F/🗑️generated/c4p23-readers-grep.txt`, 145 lines, plus a fresh search for relative `#[path]`/`include_str!` references). The only reader of moved text is the tool-job coverage law above (fixed). The remaining hits are directory-prefix matches (schema catalogs, schema-document index, grammar fixtures, build owners, dependencies), docstring mentions, and mounts that live in files this slice does not move (`rdf` still carries its two relative `#[path = "../../🚪️io/…/🔺️diff/🦀️.rs"]` mounts in the root).

No compatibility re-exports: the roots hold `#[path] mod x; <vis> use x::*;` wiring only (the glob re-export is the module wiring of a split, not an alias), no `pub use` of old names, no deprecation.

## 5. Verification (commands, real results)

| Check | Command | Result |
|---|---|---|
| Lossless line multiset (live) | `python3 c4p23-tools/c4plossless.py live` | `LOSSLESS OK removed=178 added=327`; all removed lines have a twin (widened, rebased, descriptor) |
| Live-tree checks (paths, duplicate mounts, regions, docstring emoji, test names, sizes) | `python3 c4p23-tools/c4pcheck.py` (`c4p23-check-live.log`) | no missing mounts, no duplicate mounts, no duplicate first emoji, regions equal (rsc 11/11, rdf 5/5, rio 11/11, rtu 3/3, edp 5/5), no split region pair, test fns 49=49 and 25=25, max child 523 lines, max path 202 bytes. The tool prints `CHECKS FAILED` only because it cannot trace the `include!` fragments of `rtu` from the raster crate root (the editor tree is mounted through `#[path = "."]` module blocks); verified by hand: `rtu` root has four `include!` lines, all four files exist, `#[path = "🧪️tests/🔬️unit/🦀️.rs"] mod tests;` is mounted by the editor root |
| rustfmt parse | `rustfmt +nightly-2026-07-20 --edition 2021 --config skip_children=true --emit stdout <file>` over all 43 files | no `error` lines (a broken control file reports one) |
| Scoped patched differential, live | `python3 c4p23-tools/scoped.py live <rsc\|rdf\|rio\|edr\|edp\|edd> lib,test,p2,p2test` (`c4p23-scoped-live-*.log`) | all six `SCOPED … CLEAN`: error multisets identical (rsc 490/782, rdf 205/243, rio 194/247, edr 141/246, edp 89/228, edd 51/188 on lib/test), warnings identical, wasm32-wasip2 lib/test identical to native |
| Full-crate typeck oracle | `C4P23_OVERLAY=… POS_PREFIX=c4p23 python3 c4p23-tools/posrun.py run c4p23.json --cfgs lib,test --label post2 --no-base`, then `posdiff.py pre post2` | `POSDIFF IDENTICAL` (lib 1367 = 1367, test 2591 = 2591); the overlay is byte-identical to the live tree (43 files `cmp`); negative control `neg` shows the deliberate break as `+1 E0425` |
| Taxonomy | `bun c4p23-tools/c4ptaxcheck.ts c4p23-newdirs.txt` | `checked=35 unresolved=0` (`crate-module-children` or `members-of-tests`) |
| RGI emoji, sibling uniqueness, reuse | `node c4p5-tools/rgi.mjs`, sibling scan, git-tracked prefix scan | `bad=0`; no sibling clash; every emoji already prefixes other folders |
| Own-closure census (canonical `readRustSourceClosureV1`, lib / test) | `bun c4p23-tools/census.ts <root>` | editing 4 -> 9 files, 2,359 -> 2,397 lines / 13 -> 18 files, 3,070 -> 3,108; patch 3 -> 8 / 1,476 -> 1,516, 7 -> 12 / 2,125 -> 2,165; details 1 -> 7 / 2,031 -> 2,079 (test 21 files, 3,160 lines; no baseline because the sandbox lacks the shared fixture); raster stub crate unchanged (1 / 7 files). All below 32 files, 4,000 lines, 262,144 bytes |
| Lib-wiring budget | `bun test ./🧪️tests/📏️lib-wiring-budget/🟦️.ts` (library module) | 3 pass, 0 fail before and after the ratchet edit |
| Tool-job reader probe (`probe.ts`) | `bun c4p23-tools/probe.ts pre`, `post` | Package 23 rows identical: `proofsMine` 30 = 30, `rowsMine` 30, `appOwnedMine` 31, `failuresMine` 1 = 1 (the baseline failure about `RasterDownloadJobFactory`). `sourcesMine` 111 -> 138 (+27 children, as expected). Global `proofs` 640 -> 618 and `failuresTotal` 1312 -> 1277 moved because peers edited other plugins between the two runs (about 16 minutes apart); not mine |
| Raster envelope law text | `probe.ts pre lawonly` vs `post` | the re-pointed reader sees the same text: 163 needle counts and 23 regex counts equal; law result `false` before and after (it is `false` at the baseline too) |
| Layout gate | `LC_ALL=en_US.UTF-8 bun ./📜️script.ts new crates "R/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1" --check-applied --base=f856dff0813 --sandbox=…` (`c4p23-checkapplied-raster-pre.log`, `-post.log`) | exit 0 both times: native 43/43, `--test` 70/70, `onlyCombined` 0, admission/shared/sharedForeign empty. For the stdio editing directories the gate refuses (`no artifact package`, and `ENOENT … contract/Cargo.toml` for the patch folder), before and after, because they are registry contracts, not artifacts |

## 6. Foreign errors, hand-offs

- Foreign: none caused or fixed by this slice. Observed: tool-job global counts drift with peer edits (above); the first reader probe of session 1 failed on a peer's moved export (`policyReadOsStoreSource`), which the peer repaired before this session's runs.
- The raster standards subtree (`rsc`, `rdf`, `rio`, `rtu`) is not in the closure of the `semio-s-artifact-raster-raster` stub crate today (`🖨️raster/🦀️.rs` mounts only two tests); the typeck oracle and the scoped differential treat those files with the crate's name, so a real compile of those four files is only possible once the crate is wired. Hand-off for whoever wires it.

## 7. Compile queue (not admitted)

No `COMPILE ADMITTED` line in `📓️FLEET.md` at 08:07 (the mutex folder `🗑️generated/cargo-mutex.lock` is held by a peer). Exact command for the compile queue (hold the fleet mutex around it):

```
cd /Users/ueli/Documents/semio && SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}' perl -e 'alarm 3500; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- check -p semio-s-artifact-stdio-contract-editing -p semio-s-artifact-stdio-contract-editing-patch -p semio-s-artifact-stdio-contract-editing-details --lib --tests --keep-going --message-format=short
```

(add `-p semio-s-artifact-raster-raster` once the raster standards subtree is mounted by that crate).

## 8. Tools and logs (kept)

`F/c4p23-tools/`: `plans.py` (split plans), `c4psplit.py` (stager, session-2 fixes), `c4pinstall.py` (installer with pre-edit equality assertion), `c4plossless.py`, `c4pcheck.py`, `scoped.py`, `posrun.py`/`posdiff.py`/`overlay.py`/`injlib.py`, `probe.ts`, `census.ts`, `checkapplied.sh`, `c4ptaxcheck.ts`. Logs and stage: `F/🗑️generated/c4p23-*` (never deleted).
