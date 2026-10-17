# Result CENSUS4-PKG-32 (prefix `c4p32`)

Executor: Sonnet 5.5, foreground, no sub-agents, no modifying git command, no ticket tool. `F` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`. Tools: `F/c4p32-tools/` (pre-edit snapshots `orig/<n>.rs.txt` with `orig/SHA256`, file list `files.txt`). Logs: `F/🗑️generated/c4p32-*`. Scratch: session scratchpad `c4p32/`.

## 0. Verdict

**DONE (static evidence; the real `bun repo cargo` check is queued).** Last update 2026-10-17 10:40.

- 8 files (10,278 lines) became 8 wiring roots (19 to 807 lines) plus 40 children (65 to 577 lines; largest child 577 lines, the fem 3d editor play app). Root 2 (shooting artifact root) keeps 807 lines because 524 of them are the inline `pub mod standards` wiring tree (nested `#[path = "."]` modules cannot move without changing module identity, as in `RESULT-CENSUS4-PKG-6.md` target 5).
- No compatibility re-export or alias: the only `pub use child::*` / `use child::*` lines are the parent wiring of the items the file already exposed (unused ones pruned, one kept deliberately, see section 5). No caller outside the eight files was touched.
- The real compile is not run: `COMPILE ADMITTED` is not in `📓️FLEET.md` (checked 10:38) and the launcher is not admitted. The queued commands are in section 6.
- Bugs found and fixed on the way: (1) the BIM `fault_notices!` invocation generates `pub fn bim_fault_notices`; the child lost its re-export when unused re-exports were pruned. The bim differential showed one new E0425; fixed by `pub use fault_notices::*;` in the bim editor root (`keep_reexport` in the tool), live differential clean afterwards (section 4).

## 1. Found state

- Fresh slice: no `c4p32-tools`, no `c4p32-*` logs existed. The 8 files were at their census line counts (1163, 1478, 1569, 1239, 1305, 1264, 1237, 1023 = 10,278).
- Snapshots of the 8 live files taken before any edit (`c4p32-tools/orig/1..8.rs.txt`, SHA256 in `orig/SHA256`). Files 3 (fem editor root), 1 (shooting editor root) and 6 (bim editor root) differ from `HEAD b84bf526c7b` by peer one-line edits (3 files, 9 insertions/9 deletions); the snapshots are the live files, not HEAD.
- Tooling: `c4p32.py` (descendant of `c4p19.py`; new: plain `//` comment lines trailing an item are attached to that item instead of aborting the parse).

## 2. File list (package 32, `plugin-family`, rust, 8 files)

| n | file | lines |
| ---: | --- | ---: |
| 1 | `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/…/✏️editor/🦀️.rs` | 1163 |
| 2 | `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🦀️.rs` | 1478 |
| 3 | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/…/🌐️any/✏️editor/🦀️.rs` | 1569 |
| 4 | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/…/🌐️any/✏️editor/🧵️session/🦀️.rs` | 1239 |
| 5 | `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/…/✏️editor/🗣️terminology/🦀️.rs` | 1305 |
| 6 | `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/…/✏️editor/🦀️.rs` | 1264 |
| 7 | `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/…/✏️editor/🧩️entities/🦀️.rs` | 1237 |
| 8 | `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/…/🧬️schema/🔺️diff/🩹️patches/🧱️building-elements/🦀️.rs` | 1023 |

(full list: `c4p32-tools/files.txt`)

## 3. Baselines (before any edit)

| Gate | Result |
| --- | --- |
| `--check-applied` (`new crates <artifact>/📦️packages/🦀️rust --check-applied --base=f856dff0813`, `c4p32-tools/checkapplied.sh before`) | shooting exit 0 (native 1755 -> 1760, `--test` 3944 -> 3944, onlyCombined 0); fem 3d exit 0 (21 crates, native 2262 -> 2267, `--test` 948 -> 948, onlyCombined 0); bim exit 0 (native 5681 -> 5856, `--test` 15263 -> 15152, onlyCombined 0) |
| tool-job proof catalog (`failures-dump.ts`, `c4p32-failures-before.txt`) | 1312 failures, `Bun.hash` of the sorted list **2980433518510787621** |
| tool-job evidence (`tooljob-probe.ts before`) | sources 17,020, rows 1,205, dispositions 1,116, proofs 640, app-owned 1,033, failures 1,312; the 8 files appear as separate sources |
| E0451 scan (`rpv-tools/lit.py c4p32.json`, `c4p32-lit-before.log`) | 7 candidate sites (3 bim `Point`, 2 `Area`, 1 `Select`, 1 `Binary`), all inherited |
| lib-wiring-ratchet | the shooting artifact root is pinned at 1479 lines (budget 150); no other root of the eight is a crate lib root |

## 4. Evidence on the installed tree

| Gate | Result |
| --- | --- |
| Lossless (line multiset, `c4p32.py lossless <tag> <live dir>`, all 8) | LOSSLESS OK, `missing 0`; extras are only wiring (`#[path]`, `mod`, re-export, `use super::*`, doc line) and region markers. File 5 (labels) declares its macro skeleton lines (see below) |
| Region markers | every file balanced (48 files, 0 unbalanced); orig/new begin counts: shooting editor 11/13, shooting root 6/7, fem editor 10/11, fem session 2/8, bim terminology 1/1, bim editor 10/11, bim entities 10/11, bim patches 0/0 (extra begins are the same region reopened in the children that hold its items) |
| rustfmt (`fmt.py live`, repo config, `skip_children`) | 48 files parse, 0 failures; `--check` hunks 104 (snapshots) -> 99 (live), none added |
| Scoped patched differential (`oracle.py`, native lib, `--test`, `wasm32-wasip2`, extern-crate and external-glob patches on both sandboxes) | shooting (both roots): lib 2006=2006, test 4253=4253, p2 2006=2006, warnings equal, CLEAN. fem editor: 523=523 / 590=590 / 523=523, CLEAN. fem session: 73=73 / 134=134 / 73=73, CLEAN. bim (four files): first stage run lib 9559 vs 9560 with **one new E0425** `bim_fault_notices` (the bug of section 0); after the fix the live-tree run is lib 9559=9559, test 21071=21071, p2 9559=9559, warnings 38=38 / 50=50 / 38=38, CLEAN. The stage runs applied to the live files because every live file equals its staged file modulo the sandbox neutralisation (checked by comparing the oracle-time sandbox with the live tree: 44 of 48 byte-identical, the 4 others differ only by `extern crate` removal / glob neutralisation / a folder emoji rename done before install) |
| Label table expansion (`labelsprobe.py live`) | the four chained row macros of the split BIM terminology expand to the same `app_labels!` token stream as the original table: 255,956 characters, 1,424 `;` rows, `EXPANSION IDENTICAL` (nightly `-Zunpretty=expanded`, `app_labels!` replaced by a `stringify!` probe in sandbox copies; a negative control mutating one row was detected) |
| Definition scan (`defscan.py <tag> <live dir>`, own files excluded from the external set) | 0 cross-file private leftovers for 7 families (46 private declarations scanned; the label file has none) |
| Source-text readers | `POLICY_SPLIT_SOURCE_ROOTS` (`📖️source-access/🟦️.ts`) now lists the shooting, fem 3d and bim editor roots, the fem session root, the bim terminology root and the bim patches root (6 lines); `tree-check.ts` shows `policyModuleTreeFiles` reaches all 40 children exactly once. The tool-job proof catalog failure list is byte-identical to the baseline: **1312 failures, hash 2980433518510787621** (after the 4 files, after all 8, and after the last registration); `tooljob-probe.ts` rows, dispositions, proofs, app-owned rows and failure lines identical, only the production source count changes (17,020 -> 17,038, the children of the roots that are not registered). The fem live-visual law `toolJobFemLiveVisualPublicationExact` takes the fem 3d session text as a parameter but nothing calls it; the session root is registered anyway |
| E0451 scan (`lit.py`) | 7 candidate sites before and after (all inherited, bim `Point` / `Area` and cross-crate `Select` / `Binary`) |
| Taxonomy | `tax.ts` over the 40 child directories: 0 unresolved (all `crate-module-children`); `sib.py`: no emoji repeats among siblings, every emoji is an existing folder prefix in the repo, longest repo-relative path 220 bytes (limit 240), longest absolute path 175 chars |
| `--check-applied` (`new crates <artifact>/📦️packages/🦀️rust --check-applied --base=f856dff0813`) | shooting exit 0 (native 1755 -> 1760, `--test` 3944 -> 3944, onlyCombined 0, same as before); fem 3d exit 0 (21 crates, native 2262 -> 2267, `--test` 948 -> 948, onlyCombined 0, same as before); bim: the run on the tree without the `fault_notices` re-export gave exit 1 (native onlyCombined 2: E0432 `crate::editor::bim::BimCommand` at `📊️export-schedule-csv:10` and `semio_framework_ui_contract` at `🔍️properties:26`, `--test` 0); the re-run on the final tree (`after3`, `--keep`) is exit 0, native 7657 -> 7662, `--test` 15785 -> 15650, onlyCombined 0 in both (the base column moves with peer edits) |
| Census (`census.sh after`, native / wasm32-unknown-unknown / wasm32-wasip2, lib and cfg(test); identical across targets) | fem editor lib 19 files / 151,378 bytes / 2,202 lines (test 24 / 187,911 / 2,416), session lib 9 / 63,369 / 1,326 (test 14 / 106,977 / 1,964): both under the gate (worst ratio 0.75). Derived before (after minus the new children and wiring): editor lib 14 files / ~2,160 lines, session lib 2 files / ~1,266 lines. The shooting and bim artifact crates mount their whole standards tree and are far over the gate before and after (shooting 228 files / 9,772 lines, bim 1,319 files / 88,656 lines; the split adds 9 resp. 19 files and about 68 resp. 155 lines of wiring); carving those crates is outside this slice |
| lib-wiring ratchet | the shooting artifact root pin lowered 1479 -> 808 in `📏️lib-wiring-ratchet/🔣️.json`; `bun test` of `lib-wiring-budget`: baseline 2 pass / 1 fail, now 1 pass / 2 fail; both failures are foreign rows (energy model root grew 165 -> 214, in the baseline too, and a stale pin of `🗄️stdio/🗿️artifacts/🗜️deflate/🦀️.rs`, created by the peer slice that split it); none of my roots is listed |

## 5. File list, line counts and layout

Wiring is `#[path = "<folder>/🦀️.rs"] mod x;` plus a `pub use` / `pub(crate) use` / `use x::*;` limited to what the file already exposed; the unused ones were pruned by the tool (`prune_unused_reexports`), `pub(super)` widening derived from identifier use across the split files and the whole crate (over-approximation by name, never narrower than before). Children use `use super::*;`, no qualified paths, no comments inside definitions, each docstring starts with the folder emoji.

| n | file | before | after (root + children) |
| ---: | --- | ---: | --- |
| 1 | shooting editor `✏️editor/🦀️.rs` | 1163 | 226 + `🧰️ui-helpers-and-io` 144, `🔀️args-bridge` 147 (inline `mod args_bridge` as file module), `🧵️retained-commands` 175, `🕹️play-app` 295, `📋️manifest` 214 |
| 2 | shooting artifact root `🎥️shooting/🦀️.rs` | 1478 | 807 + `🪪️declaration` 160, `🌍️scene-records` 253, `🖼️emblem-codec` 126, `🩹️scene-patches` 162 |
| 3 | fem 3d editor `✏️editor/🦀️.rs` | 1569 | 194 + `🗃️retained-commands` 313, `📬️store-preparation` 151, `🔌️json-io-and-arg-helpers` 125, `🖥️play-app` 577, `📋️manifest` 251 |
| 4 | fem 3d session `🧵️session/🦀️.rs` | 1239 | 99 + `🧷️identity-and-recovery-slot` 117, `💼️mounted-state` 283, `📸️snapshot-preflight` 160, `🗃️registry` 103 (`MOUNTED` thread-local widened `pub(super)`), `⚙️mounted-job` 164, `🚀️session-api` 182, `🧹️recovery-and-close` 191 |
| 5 | bim terminology `🗣️terminology/🦀️.rs` | 1305 | 66 + `🏠️model-and-window-labels` 296, `🎮️command-and-argument-labels` 236, `⚠️fault-and-utility-labels` 302, `🔍️review-and-schedule-labels` 455 |
| 6 | bim editor `✏️editor/🦀️.rs` | 1264 | 387 + `🔀️args-bridge` 228, `🗃️io-and-retained-commands` 132, `📢️fault-notices` 118, `🖥️play-app` 317, `📋️manifest` 115 |
| 7 | bim entities `🧩️entities/🦀️.rs` | 1237 | 204 + `🔤️field-codecs` 344, `🧮️field-tables` 282, `➕️create-rows` 211, `🗂️entity-table` 97, `🧾️properties` 78, `🔎️project-and-lookup` 65 |
| 8 | bim building-element patches `🧱️building-elements/🦀️.rs` | 1023 | 19 + `🏚️walls-and-curtain-walls` 228, `🏛️columns-beams-slabs` 228, `🔲️ceilings-roofs-openings` 240, `🪜️stairs-railings-ramps` 336 |

Notes on the three non-mechanical shapes:
- **Macro tables stay with their uses.** File 6 keeps `bim_command_table!` and its four consumer macros in the root (they generate `BimCommand`, the tool roster and the manifest actions, items the tool cannot widen). File 7 keeps the `set!` / `delete!` / `renaming!` / `field!` / `inferred!` / `kind!` macros in the root and wires every table child after the macro definitions, so the textual macro scope reaches them. File 3 keeps the `app_commands!` invocation in the root.
- **File 5 (one `bim_labels!` table of 1,260 rows).** `app_labels!` defines one struct per invocation, so the rows cannot become several invocations. The rows moved verbatim into four `macro_rules! bim_label_rows_*` row providers (one per child, `#[macro_use]` mod) that pass their accumulated rows to the next provider and the last one to `bim_labels_emit!`, which calls the unchanged `bim_labels!`. `lossless` declares the macro skeleton lines as extras and the single removed line `bim_labels! {`; `labelsprobe.py` proves the identical expansion.
- **Macro-generated item kept reachable.** `📢️fault-notices` holds the `fault_notices!` macro and its invocation, which generates `pub fn bim_fault_notices`; the root re-exports it with `pub use fault_notices::*;` (spec key `keep_reexport`).

## 6. Commands run (real results)

All from `/Users/ueli/Documents/semio`, logs in `F/🗑️generated/c4p32-*`; tools in `F/c4p32-tools/`. No ENFILE or Bun segfault occurred in this session.

| command | result |
| --- | --- |
| `c4p32-tools/checkapplied.sh before` (three `new crates .. --check-applied --base=f856dff0813`) | shooting, fem3d, bim exit 0 (section 3) |
| `bun failures-dump.ts` before / after 4 files / after 8 files / final | 1312 failures, hash 2980433518510787621 each time |
| `bun tooljob-probe.ts before` / `after` | identical except the production source count 17,020 -> 17,038 |
| `python3 rpv-tools/lit.py c4p32.json` before / after | 7 candidate sites both times |
| `python3 oracle.py shooting lib,test,p2`; `run-diff.sh stage2 lib,test,p2 femeditor femsession bim` | shooting, femeditor, femsession CLEAN; bim lib showed the one E0425 (the bim `test`/`p2` stage runs were killed after the bug was known) |
| `C4P32_LIVE=1 C4P32_SKIP_ORIG=1 python3 oracle.py bim lib,test`; `C4P32_LIVE=1 python3 oracle.py bim p2` | CLEAN, CLEAN |
| `python3 c4p32.py install <tag>` for 8 specs (refuses unless the file equals its snapshot) | 7 installed, `labels.py` stage + install for the label file |
| `python3 c4p32.py lossless <tag> <live dir>` x 8; `fmt.py live`; `defscan.py` x 7; `sib.py`; `bun tax.ts newdirs.txt`; `bun tree-check.ts`; `python3 labelsprobe.py live` | all as in section 4 |
| `c4p32-tools/checkapplied-two.sh after1 shooting fem3d`; `after2 bim`; `after3 bim` (`C4P32_KEEP=1`) | exit 0, exit 0; bim `after2` exit 1 (before the fix), `after3` exit 0 |
| `census.sh after` | section 4 |
| `bun test lib-wiring-budget` before / after | 2 pass 1 fail / 1 pass 2 fail (foreign, section 4) |

Queued real check (not run: no `COMPILE ADMITTED` line, compile mutex untouched). One launcher call per artifact workspace under the fleet mutex, from `/Users/ueli/Documents/semio`:

```
SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}' perl -e 'alarm 3500; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- check -p <crate> --lib --tests --keep-going --message-format=short --manifest-path <artifact dir>/Cargo.toml
```

| artifact dir (under `✏️s/🔌️plugins/`) | crates for `-p` |
| --- | --- |
| `🎥️shooting/🗿️artifacts/🎥️shooting` | `semio-s-artifact-shooting-shooting` |
| `🏗️fem/🗿️artifacts/🧊️3d` | `semio-s-artifact-fem-3d-editor`, `semio-s-artifact-fem-3d-editor-session` |
| `🏙️bim/🗿️artifacts/🏢️model` | `semio-s-artifact-bim-model` |

Also queue the bim label table (the real `app_labels!` expansion with `$crate` paths and the `BimLabels::localized` consumers) and the fem session unit tests.

## 7. Foreign errors and hand-offs

- `lib-wiring-budget`: energy model root grew 165 -> 214 lines (baseline), `🗄️stdio/🗿️artifacts/🗜️deflate/🦀️.rs` pin is stale since the peer split of package 34. Not mine.
- Tool-job proof catalog: 1,312 inherited failures (identical before and after).
- The shooting and bim artifact crates are far over the compiler-ingress gate (they mount the entire standards tree); a crate carve of those two is not part of this slice.
- `newdirs.txt` in `c4p32-tools/` lists the 40 child directories.
