# Result CENSUS4-PKG-29 (prefix `c4p29`): plugin-family, stdio pdf (1.7 base) lower/lift/export/graph-projection/diff/lib root and docx base test oracle

Executor Sonnet 5.5, foreground, no sub-agents, no modifying git command, no ticket tool. `F` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`. Tools: `F/c4p29-tools/` (copied from `c4p25-tools`, re-pointed and extended). Logs: `F/🗑️generated/c4p29-*` (nothing deleted). Stage tree (regenerable from the snapshots): `scratchpad/c4p29-work/stage2`.

## 0. Verdict

DONE. All seven files are split and installed (29 children, 7 rewritten roots), lossless, order-preserving, formatted, with the complete-output oracle differential identical to the pre-edit tree (pdf crate lib: privacy lines 22 = 22, non-stub rows 461 = 461, error histogram identical; docx test oracle lib + test rows identical), the `--check-applied` plugin gate at `onlyCombined` 0 for pdf and docx, the reader probe byte-identical (1,312 failures, same hash), the ratchet and composition laws at their baselines. No cargo compile was possible (compiler ingress gate); the exact command for the compile queue is in section 6.

## 1. Files (`F/MONOLITH-CENSUS-4.md` section 4, Package 29, 7 files, 10,615 lines)

`PB` = `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base`, `PDF` = `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf`, `DB` = `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base`.

| key | file | lines before | root after | children (lines) |
| --- | --- | ---: | ---: | --- |
| lower | `PB/🔨️modules/⬆️lower/🦀️.rs` | 1,501 | 36 | `🎛️options-and-lowering` 361, `🔤️fonts-and-resources` 360, `🧭️pages-and-navigation` 484, `📚️catalog-and-info` 286 |
| lift | `PB/🔨️modules/⬇️lift/🦀️.rs` | 1,435 | 40 | `🧩️lifter-and-document` 325, `🪟️resources-and-fonts` 441, `🧭️navigation-and-annotations` 363, `📚️catalog-and-page-mode` 294 |
| exp | `PB/🚪️io/📤️export/🦀️.rs` | 1,156 | 62 | `🧬️encode-and-reconcile` 294, `🪡️graft` 545, `🌊️streaming-and-builders` 280 |
| gp | `PB/🧬️schema/🔎️graph-projection/🦀️.rs` | 1,400 | 54 | `🧩️lifter-and-document` 274, `🪟️resources-and-fonts` 443, `🧭️navigation-and-annotations` 363, `📚️catalog-and-page-mode` 294 |
| diff | `PB/🧬️schema/🔺️diff/🦀️.rs` | 2,286 | 66 | `🔀️tri-state-indexed-and-keyed` 369, `📄️page-info-and-pages` 411, `🌳️value-trees` 597, `🧭️path-addressing-and-validation` 205, `🧾️pdf-diff-aggregate` 325, `🛠️mutation-diff-builders` 353 |
| root | `PDF/🦀️.rs` (lib root of `semio-s-artifact-stdio-pdf`) | 1,410 | 89 | `🧾️artifact-declaration` 273, `🌳️standards-tree` 359, `✏️editor-tree` 346, `👁️viewer-tree` 342 |
| ora | `DB/🔮️oracles/🦀️.rs` | 1,427 | 115 | `🌳️xml-tree-and-opc` 402, `📝️wordprocessing-and-package` 310, `🧮️json-values-and-addressing` 234, `🔁️forward-inverse-and-routing` 392 |

Totals: 10,615 lines in 7 files become 7 roots (462 lines) plus 29 children; the largest file is 597 lines (`diff/🌳️value-trees`), the largest path is 201 UTF-8 bytes (limit 240).

Crates that mount them (`c4p29-tools/whocrate.py`): `semio-s-artifact-stdio-pdf` (lib root `PDF/🦀️.rs`; mounts root, lift, lower, gp, diff, exp) and `semio-s-artifact-stdio-docx-test-oracle` (mounts ora).

How the split is built:

- Mode `mod`: each child is a `#[path]` module (`//!` emoji docstring, `use super::*;`, the moved lines verbatim), mounted from its root with `mod x;` and, when the child exports a name another file uses, `pub use x::*;` (visibility derived from the items). Private definitions another file names became `pub(super)` (structs, fields, tuple fields, inherent-impl members; widened lines: lower 28, lift 50, exp 42, gp 46, diff 29, ora 73, root 0). `pub use super::lift::Category;` in `lower` is now the absolute path `crate::standards::v1_7::subsets::base::modules::lift::Category`, because `super` moved one level.
- `exp`: region `🔖️Encode` spans two children, so it is repeated balanced (`//#endregion` closes it in `🧬️encode-and-reconcile`, `//#region` reopens it in `🪡️graft`).
- `root`: three inline `#[path = "."]` mount trees (`standards`, `editor`, `viewer`) became files; the child holds the body of the inline module (dedented) with every relative `#[path]` re-based by `../` (a `#[path]` file is a mod-rs file, so its nested mounts resolve from its own directory); the root keeps `#[path = "<folder>/🦀️.rs"] pub mod standards;`. The declaration functions and the derived io registry moved to `🧾️artifact-declaration` (`include_str!("../📜️artifact-definition.json")` re-based). The root is 89 lines, so the pin in the lib-wiring ratchet (150-line budget) is gone.
- `ora`: the 1,337-line inline `mod oracles` is a container: `#[path = "."]` was added in front of it, its four depth-1 slices became children mounted inside it (`mod x; use x::*;`, the one with the entry points `pub use`), dedented.

## 2. Found state (fresh slice, 2026-10-17 07:34)

- No `c4p29-*` file, tool folder, log or child folder existed. The 7 files carried no staged or unstaged change in the Rust text (`git diff HEAD` for the pdf and docx trees only shows peer TypeScript and `Cargo.lock` edits).
- Pre-edit snapshots with SHA256: `F/c4p29-tools/orig/{lower,lift,exp,gp,diff,root,ora}.rs` plus `.path` and `SHA256SUMS`. Before the install each live file was compared with its hash again (`7 of 7 live == snapshot`):

```
62abdcb0978dfa5bcd536d7348eb792fa0160ca05c76abaf8e1086880a958219  diff.rs
01859c3204de8ff8eb72155719f0f095e16f26acc6a1f2a8d1d58a0998835916  exp.rs
5dad14e29356934b351600abf79f3fec8996705db7be954e58903efb0ae97aee  gp.rs
6c376fc3c3e6d19f8419537e68d02f8e8e9a86695fcc4021b1d9c1c019a93bdc  lift.rs
2f3f7fc6823692cb98e40a1d79bf877fed5179b3929cd421ba68884f54f45726  lower.rs
4ee780ac1ed3864b0315b8f27cc6aa51bbba5954edd5f60fd222e3dfe99054e9  ora.rs
7c8b23b203694177cc23d198cf44770e193884c9ae476ffa959429510d0274a0  root.rs
```

- Peers worked in the same trees at the same time (a `c4p30` slice, TypeScript `🔺️diff/🟦️.ts` and `inferences` edits under the pdf and docx artifacts); none of that was touched.

## 3. Baselines recorded before the first edit

| Check | Command | Result |
| --- | --- | --- |
| Typeck oracle, lib (pdf crate + docx test oracle) | `PYTHONHASHSEED=0 POS_PREFIX=c4p29 python3 c4p29-tools/posrun.py run c4p29-tools/c4p29.json --cfgs lib --label pre0 --no-base` (`c4p29-pos-pre0.log`; an earlier unseeded run `pre` is row-identical) | canary reached (`E0308, E0616, E0624`); 11,106 errors (10,645 stub-involved); 22 privacy lines, 461 non-stub rows |
| Typeck oracle, test cfg of the pdf crate | same with `--cfgs test` (`c4p29-pos-pre.log`) | 2 errors only: the union of the pdf crate does not parse in test cfg (`pub mod crate {` and a missing unicode-escaped `#[path]`); pre-existing, so the pdf test cfg is blind in this oracle |
| Typeck oracle, docx test oracle alone, lib + test | `... posrun.py run c4p29-tools/c4p29ora.json --cfgs lib,test --label orapre` | canary reached; lib 270, test 323 rows |
| E0451 literal scan | `POS_PREFIX=c4p29 python3 rpv-tools/lit.py c4p29-tools/c4p29.json pre` | 0 candidate sites; 243 brace structs, 345 definitions, 1,028 files in the module tree |
| Census (`measureRustCrate`, gate 32 files / 262,144 bytes / 4,000 lines) | `c4p29-tools/census.sh before` | pdf crate 722 files / 3,744,187 bytes / 52,805 lines (far over the gate before and after); docx test oracle 4 files / 86,710 bytes / 1,652 lines (inside) |
| `--check-applied` (docx) | `c4p29-tools/checkapplied.sh before docx` | exit 0, 8 crates, native 1,578 = 1,578, `--test` 1,931 = 1,931, `onlyCombined` 0, admission and shared empty |
| `--check-applied` (pdf) | `c4p29-tools/checkapplied.sh before pdf` | started 08:02 and was still running when the install happened; treated as not usable as a baseline (the `after` run is the gate: `onlyCombined` must be 0) |
| Reader probe (tool-job scan, completion oracle) | `bun c4p29-tools/probe.ts` (`c4p29-probe-before.json`) | 16,997 sources, 1,312 failures (`failuresHash` 2980433518510787621), 0 proofs/rows/app-owned rows for stdio pdf or docx, completion oracle 10; the 7 files are among the 842 scanned stdio sources |
| Ratchet law | `bun test .../📏️lib-wiring-budget/🟦️.ts` (`c4p29-ratchet-before.log`) | 2 pass, 1 fail (peer: `🔋️energy/.../🧬️schema/🦀️.rs` grew from 165 to 214) |
| Hub composition law | `bun test 🌎️hub/🧩️compositions/🗄️stdio/🔮️oracles/🧪️tests/🧩️composition/🟦️.ts` (`c4p29-composition-before.log`) | 60 pass, 3 fail (png oracle package, current Rust callers, lower plugin ownership surface; none mine) |

## 4. Evidence after the install (live tree)

| Gate | Command | Result |
| --- | --- | --- |
| Lossless (stripped non-empty line multiset; signed delta minus the plan's own wrapper, marker, rebase and widening lines) | `c4plossless.py live` (`c4p29-lossless-live.log`) | `LOSSLESS OK`: 0 unexplained removed lines; the added lines are only wiring (`#[path]`, `mod`, `use x::*;`), 29 child docstrings and `pub(super)` widenings |
| Order (children spliced back at the mount, stripped sequence equals the pre-edit blob) | `c4pseq.py live` | `SEQUENCE OK`, 7 of 7 identical |
| Mounts and reachability | `c4pmore.py mounts live`, `c4pcheck.py` | every `#[path]`/`include!` target exists, each of the 29 children is reached exactly once, 0 new duplicate mounts; the 4 missing unicode-escaped `#[path]` targets of the 4.1.4 editors exist before and after |
| Region markers | `c4pcheck.py` | balanced in every file; 0 regions split across files except the one repeated in `exp` (planned); test fn names equal (root: 2 = 2) |
| Public API | `c4pmore.py api live` | the `pub` and `pub(crate)` top-level item sets of all 7 roots are identical before and after (lost 0, gained 0) |
| rustfmt | `c4pparse.py live`, `c4pmore.py fmt live` | all 36 files parse; `--check` hunks 132 before, 129 after (no new debt) |
| Definition scan (independent of the widener; raw identifiers of every other file in the 1,046-file closure) | `c4pdefscan.py live` | 117 private declarations scanned, 5 cross-file hits, all false positives checked by hand: `Recording.inner`, `Recording.seen`, `CatalogLane::moved`, `Graft::catalog`, `Graft::put` match only local variables or same-file uses |
| Taxonomy | `bun c4p29-tools/c4ptaxcheck.ts 🗑️generated/c4p29-dirs.txt` | `checked=29 unresolved=0`, all `crate-module-children`; all emojis RGI (`rgi.mjs`) |
| Path length | `c4pcheck.py` | max 201 UTF-8 bytes (limit 240) |
| Typeck oracle on the live tree, pdf crate lib | `PYTHONHASHSEED=0 POS_PREFIX=c4p29 python3 c4p29-tools/posrun.py run c4p29-tools/c4p29.json --cfgs lib --label live0 --no-base`, then `python3 c4p29-tools/c4pfullcmp.py pre0 live0` | canary reached; 11,106 errors (same total); **privacy lines 22 = 22, non-stub rows 461 = 461, by-code histogram identical: `FULLCMP IDENTICAL`** |
| Typeck oracle on the live tree, docx test oracle lib + test | `... c4p29ora.json --cfgs lib,test --label oralive`, `C4P29_FAMILY=c4p29ora python3 c4pposline.py orapre oralive lib test` | lib 270 = 270, test 323 = 323, 0 rows only on either side: `POSLINE IDENTICAL` |
| E0451 literal scan | `lit.py c4p29.json post` | 0 candidate sites; 1,057 files in the module tree (+29 children) |
| Census after | `c4p29-tools/census.sh after` | pdf crate 747 files / 3,749,019 bytes / 52,976 lines (still far over the gate, +25 files, +171 lines of wiring and docstrings); docx test oracle 8 files / 83,626 bytes / 1,682 lines (inside the gate) |
| Ratchet law | `bun test .../📏️lib-wiring-budget/🟦️.ts` (`c4p29-ratchet-after.log`) | 2 pass, 1 fail: the same single peer breach as before (energy schema 165 to 214); the pdf lib root pin 1,411 was removed (root now 89 lines), and the test "ratchet lists no root that complies" passes |
| Hub composition law | same command (`c4p29-composition-after.log`) | 60 pass, 3 fail, the same three tests as before |
| `--check-applied`, reader probe | see 4b | pdf and docx `onlyCombined` 0; probe identical |

### 4a. Why the first differentials looked different, and how it was resolved

- The raw `*-final.err` of the oracle is cut at 30,000,000 characters (`pos.py` line 347). A split that lengthens diagnostic paths moves the cut, so a row-by-row comparison of the truncated file loses the last ~159 rows (all in the vt subset at the end of the module order) although rustc printed the same 11,106 errors. Proof: `pre0` versus a tree where only the three mount trees are split (`ovR_trees`) shows 150 phantom lost rows by `c4pposline.py`, but `c4pfullcmp.py` (privacy file and non-stub rows written from the complete output, plus the by-code histogram) is identical for the same pair. `c4pfullcmp.py` is the comparison used for the verdict.
- The first staged tree (`post1`) also had one real regression: a tuple struct `GraphView(&..)` whose `pub(super)` item widening was overwritten by its tuple-field widening on the same line (E0425 in `🪡️graft`). Fixed in the splitter (merge of the two edits), restaged, and the differential above is on the fixed tree.
- A negative control for the container mode: removing one `pub(super)` from the staged `ora` tree gave 4 new E0425 rows in the oracle (`C4P29_FAMILY=c4p29ora python3 c4pposline.py orapre oraneg lib`), so the differential does see a broken visibility.

### 4b. Layout gate and reader probe

| Gate | Command | Result |
| --- | --- | --- |
| `--check-applied` docx | `LC_ALL=en_US.UTF-8 c4p29-tools/checkapplied.sh after docx` (`c4p29-checkapplied-after-docx.log`) | exit 0; 8 crates; native 1,578 = 1,578, `--test` 1,931 = 1,931, `onlyCombined` 0 / 0; admission, shared, sharedForeign empty (identical to the baseline) |
| `--check-applied` pdf | `... checkapplied.sh after pdf` (`c4p29-checkapplied-after-pdf.log`, started 08:48, ended 09:48) | exit 0; native base 6,558 / combined 4,093, `--test` base 4,367 / combined 4,504; **`onlyCombined` 0 / 0**; admission, shared, sharedForeign empty. (The pdf `before` run ended 08:50 with the same `onlyCombined` 0, but it straddled the install at 08:45, so only the `after` run is used; the base counts move with the peer tree state.) |
| Reader probe | `bun c4p29-tools/probe.ts` (`c4p29-probe-after.json` against `c4p29-probe-before.json`) | identical: 1,312 failures, `failuresHash` 2980433518510787621, 0 stdio proofs / rows / app-owned rows, scan and completion oracle (10) equal. Only `sources` differs: the 29 new children are among the scanned sources (stdio sources 842 to 871; none removed) |

## 5. Source-text readers and consumers

- Re-pointed (`c4p29-tools/c4pconsumers.py`, journal `F/🗑️generated/c4p29-consumers.json`):
  1. `PB/🧬️schema/🦀️.rs` line 210: the `diff` facet registers its Rust text with `rust: include_str!("🔺️diff/🦀️.rs")`; it is now `rust: concat!(include_str!("🔺️diff/🦀️.rs"), include_str!("🔺️diff/<each of the six children>/🦀️.rs"))`, the pattern the obj, ply, las and norm diff facets already use. `concat!(include_str!(..), include_str!(..))` was checked with a one-file `rustc --emit=metadata` in the scratchpad. The stub layer of the oracle cannot read this expression (`FacetLeaves has no field named rust`), so the differentials above were run on a tree without this one-line edit.
  2. `🧰️framework/.../📚️library/🧪️tests/🧫️fixtures/📏️lib-wiring-ratchet/🔣️.json`: pin of `PDF/🦀️.rs` (1,411) removed.
- Checked and not affected (the root files keep their paths): `⚡️caching/🧪️tests/🧫️fixtures/artifact-packages/🔣️.json` (names the pdf root as package source), the hub composition fixture (names the docx base oracle root and its sha256; the composition law is unchanged), `POLICY_SPLIT_SOURCE_ROOTS` (lists none of these roots; the tool-job reader globs every `🦀️.rs`, so the children are scanned too), the `stream-roles` TypeScript test that parses `../../🔺️diff/🦀️.rs` with tree-sitter (it fails on its first path, `stream-roles/🦀️.rs`, not mine: `expect(law).toBeDefined()`; my roots are valid Rust by rustfmt).

## 6. Limits and foreign errors

- No cargo compile was run (compiler ingress gate, section "Compile mutex" of the wave brief). The type-level proof is the static `rustc --emit=metadata` differential above; the pdf crate is blind in test cfg (union does not parse), so the unit-test modules (`exp`/`diff` `🧪️tests`) are covered only by the definition scan and by the token-based widener. A real check for the queue: `cd /Users/ueli/Documents/semio && SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}' perl -e 'alarm 3500; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- check -p semio-s-artifact-stdio-pdf -p semio-s-artifact-stdio-docx-test-oracle --lib --tests --keep-going --message-format=short`.
- The pdf crate fails the source-boundary gate before and after (722 to 747 files, 52,805 to 52,976 lines); it cannot pass by splitting these files, so no compile of it is possible until the crate is carved.
- Foreign: `stream-roles` TypeScript test (1 fail at `./🦀️.rs`), the ratchet breach `🔋️energy/.../🧬️schema/🦀️.rs` (165 to 214), the three composition failures above; none involve a file of this slice.

## 7. Files

- Rewritten roots (7): the files of section 1.
- Created (29 children, each `🦀️.rs` in its folder next to the root): section 1.
- Edited consumers (2): `PB/🧬️schema/🦀️.rs`, `📏️lib-wiring-ratchet/🔣️.json`.
- Tools (kept): `F/c4p29-tools/` (`plans.py`, `c4psplit.py`, `c4plossless.py`, `c4pseq.py`, `c4pmore.py`, `c4pcheck.py`, `c4pparse.py`, `c4pdefscan.py`, `c4pfullcmp.py`, `c4pposline.py`, `c4pinstall.py`, `c4pconsumers.py`, `posrun.py`, `posdiff.py`, `injlib.py`, `census.sh`, `checkapplied.sh`, `probe.ts`, `c4ptaxcheck.ts`, `c4p29.json`, `c4p29ora.json`, `orig/`).
- Logs and journals (kept, `F/🗑️generated/`): `c4p29-installed.json`, `c4p29-consumers.json`, `c4p29-pos-*.log`, `c4p29-c4p29*-*.err`, `c4p29-fullcmp-*.txt`, `c4p29-*-live.log`, census, probe, ratchet, composition and check-applied logs.
