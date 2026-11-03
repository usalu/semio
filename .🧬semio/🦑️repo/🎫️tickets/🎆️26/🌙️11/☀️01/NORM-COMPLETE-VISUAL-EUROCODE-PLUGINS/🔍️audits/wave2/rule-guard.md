# Rule Guard (wave 2)

Scope: `✏️s/🔌️plugins/📕️norm` for en1990 to en1999 and the shared crates; din4108, vdi3805, iso16757, din18599, din16798 excluded. Scanner: `🗑️generated/scan.py`, outputs in `🗑️generated/rule-guard/`. Scan rules are the brief's list.

## Pass 1 (2026-11-02 15:19)

Counts (in scope):
- long paths (> 256 bytes from `/home/deck/semio`): 26 (en1994: 3 fixture folders, en1999: 14 fixture folders)
- `[DEBUG]`: 0
- prose `//` comments or `//#region` inside bodies (.rs/.ts): 0 (the 458 scanner hits are single-line `/** @state … */` JSDoc on generated members, which start with an emoji)
- docstrings without emoji: 6, all false positives (keycap emoji `#️⃣` starts with `#`, the scanner does not recognise it)
- forbidden wording: `TODO`, `FIXME`, `todo!`, `unimplemented!`, `SDK GAP`, `legacy`, `compat layer`: 0
- `placeholder`: 2 fixed (oracles python), 1 kept (`execution` enum value `"placeholder"` in `🧬️schema/🔣️.json`, mirror of the framework job classification), 2 kept (guard test strings in `⚖️compliance` test)
- `migrat*`: all hits are `InteractiveJobClassification::Migrated`, `admission: "migrated"` (framework mirror) or test wording "(migrated)"; kept, see note below
- `-2` names: 7 crates in en1999 mutation groups (routed)

Fixes:
- `🔮️oracles/🏃️execution/🐍️.py` line 38: "placeholder" to "rule"
- `🔮️oracles/🏃️execution/📝️carrier/🐍️.py` line 86: "placeholder" to "rule"

Routed (requests.md, `[open] guard → leaf-kinds`):
- 26 long mutation fixture paths (en1994, en1999)
- 7 `-2` mutation group crates in en1999

Notes:
- `strain compatibility`, `compatible units`, `q-compatible detailing` are engineering or API terms, not the `compat layer` wording; not flagged.
- `"migrated"` and `InteractiveJobClassification::Migrated` mirror the framework enum; `retained-disposition-oracle` error text ("not fully migrated") sits in app-surface, shared, not touched.
- `parts-2-6` (en1993 clause folder) is a clause name, not a `-2` suffix; not flagged.
- Scanner fixes wanted: keycap emoji (`#️⃣`) are emoji; `-2` check is not in `scan.py`.

## Pass 2 (2026-11-02 15:34)

Counts (in scope): long paths 4 (was 26; en1999 fixtures shortened, en1994 `change-beam-steel-plastic-modulus-m3`, `change-beam-hogging-reinforcement-m2-per-m` (2 files) and `change-beam-steel-flange-thickness-m` still over, routed); `[DEBUG]` 0; prose `//` and `//#region` in bodies 0; docstrings without emoji 6 (keycap false positives); forbidden `legacy`/`SDK GAP`/`TODO`/`FIXME`/`todo!`/`unimplemented!` 0; `placeholder` 5 (kept, see pass 1); `compat` 42 (new hits are unit and engineering terms, e.g. "compatible unit", "strain compatibility"); `-2` names: 7 en1999 mutation group dirs still present (routed).

Fixes: none this pass.

Still open: en1994 long fixture paths and en1999 `-2` mutation groups (routed to leaf-kinds in requests.md).

## Pass 3 (2026-11-02 15:39)

Counts (in scope): long paths 0 (en1994 fixtures now short); `[DEBUG]` 0; prose `//` and `//#region` in bodies 0; docstrings without emoji 0 (keycap `#️⃣` starts are accepted); `legacy`, `SDK GAP`, `TODO`, `FIXME`, `todo!`, `unimplemented!`, `migration` 0; `placeholder` 5 (kept, framework mirror and guard tests); `migrated` wording only as framework enum mirror; `-2` names 7 (en1999 mutation group dirs, still routed to leaf-kinds).

Fixes: none this pass.

## Pass 4 (2026-11-02 15:42)

Counts (in scope): long paths 0; `[DEBUG]` 0; prose `//` and `//#region` in bodies 0; docstrings without emoji 0; `legacy`, `SDK GAP`, `TODO`, `FIXME`, `todo!`, `unimplemented!`, `migration` 0; `placeholder` 5 new-looking hits are framework API (`ui::input(...).placeholder(...)` in `🪟️visual-windows/🧾️properties/🎛️controls`, kept) plus the earlier kept ones; `-2` names 7 (still routed).

Fixes: none this pass.

## Pass 5 (2026-11-02 15:45)

Counts (in scope): long paths 0; `[DEBUG]` 0; prose `//` and `//#region` in bodies 0; docstrings without emoji 0; `legacy`, `SDK GAP`, `TODO`, `FIXME`, `todo!`, `unimplemented!`, `migration` 0; `placeholder` 5 (kept); `-2` names 7 (routed, unchanged since pass 1).

Fixes: none this pass.

## Pass 6 (2026-11-02 15:48)

Counts (in scope): long paths 0; `[DEBUG]` 0; prose `//` and `//#region` in bodies 0; docstrings without emoji 0; `legacy`, `SDK GAP`, `TODO`, `FIXME`, `todo!`, `unimplemented!`, `migration` 0; `placeholder` 5 (kept); `compat` 42 (engineering and unit terms); `-2` names 7 (routed, unchanged).

Fixes: none this pass.

## Pass 7 (2026-11-02 15:50)

Counts (in scope): long paths 0; `[DEBUG]` 0; prose `//` and `//#region` in bodies 0; docstrings without emoji 0; `legacy`, `SDK GAP`, `TODO`, `FIXME`, `todo!`, `unimplemented!`, `migration` 0; `placeholder` 5 (kept); `compat` 43 (one new hit, `strain compatibility` engineering term, not flagged); `-2` names 7 (routed, unchanged).

Fixes: none this pass.

## Pass 8 (2026-11-02 15:53)

Counts (in scope): long paths 0; `[DEBUG]` 0; prose `//` and `//#region` in bodies 0; docstrings without emoji 0; `legacy`, `SDK GAP`, `TODO`, `FIXME`, `todo!`, `unimplemented!`, `migration` 0; `placeholder` 5 (kept); `-2` names 7 (routed, unchanged since 15:19).

Fixes: none this pass.

## Pass 9 (2026-11-02 15:54)

Counts (in scope): as pass 8 (long paths 0, `[DEBUG]` 0, prose comments 0, docstrings without emoji 0, forbidden wording 0 apart from kept `placeholder` 5; `-2` names 7 routed).

Fixes: none this pass.

## Pass 10 (2026-11-02 15:55)

Counts (in scope): as pass 9 (long paths 0, `[DEBUG]` 0, prose comments 0, docstrings without emoji 0, forbidden wording 0 apart from kept `placeholder` 5; `-2` names 7 routed, still open in requests.md).

Fixes: none this pass.

## Pass 11 (2026-11-02 15:57)

Counts (in scope): unchanged from pass 10 (long paths 0, `[DEBUG]` 0, prose comments 0, docstrings without emoji 0, forbidden wording 0 apart from kept `placeholder` 5; `-2` names 7 routed).

Fixes: none this pass.

## Pass 12 (2026-11-02 16:26)

Counts (in scope, din4108/vdi3805/iso16757/din18599/din16798 excluded):
- long paths (≥256 bytes from `/home/deck/semio`): 0
- `[DEBUG]`: 0
- `//` prose and `//#region` inside bodies (indented, .rs/.ts/.js): 0. Column-0 `//#region` section markers in crate roots are not in bodies and were not counted. The scan's `/**` hits in `.ts` are generated `@state` docstrings, not prose.
- docstrings without emoji: 38 found, 38 fixed. Fixes: 34 Python docstrings (`"""` lines in `🔮️oracles` and `🧪️tests` `.py` files, which earlier passes did not scan) and 4 Rust docstrings where a non-emoji `▭` glyph led the line (`🎬️scene/🧮️facts`, `🧬️alloys-and-sections`, `🖼️canvas-lowering`, which is in shared app-surface). `𝜓` with FE0F in the en1995 oracle replaced by `🪵`. Six remaining hits start with an emoji plus variation selector (`⤴️`, `▶️`, `◻️`) and were accepted. One scanner hit (`table.rs:315`) came from the `/**` branch on a non-TS string and is a false positive.
- forbidden wording (brief list): `legacy`, `SDK GAP`, `TODO`, `FIXME`, `todo!`, `unimplemented!`: 0. `migrat*`: 142 hits, 139 are the framework enum or admission mirror (`InteractiveJobClassification::Migrated`, `"migrated"`), kept as in pass 1. 3 wording hits routed to shared (see requests.md).
- `-2` names: 0 in scope. The seven en1999 `-2` mutation crates from pass 1 are no longer in the tree. The three `-2` names left (`failing-2`, `products-2`, `elements-2`) are in excluded trees.
- source budget (`source-boundary-scan.py -v`, flagged at ≥30 files, ≥3700 lines or ≥240000 B): 12 in-scope crates. OVER the gate: `en1999-diff` (6281 lines). Warning band: `en1998-diff` 3919, `en1996-masonry` 3886, `⚖️compliance` 3818, `en1992-diff` 3990 (already routed by triage at requests.md line 477), and six 31-file mutation crates. `en1993-diff` dropped below 240000 B since the last pass. Excluded: `vdi3805-model`, `iso16757-model`.

Fixes: 38 docstring emoji (see above). No DEBUG lines to remove.

Routed (requests.md, appended under "from: rule-guard pass 12"): en1999 owner (OVER gate, diff crate), en1998 owner (diff crate), en1996 owner (masonry crate), shared (compliance crate, and migration wording in 3 strings), leaf-kinds (6 mutation crates at 31 files).

Still open from earlier passes: none in scope for paths, DEBUG, comments or `-2` names. Routed earlier: en1992-diff (line 477).

Note: the rule-guard pass ran on a live tree, and the boundary scan changed between two runs in this pass (en1993-diff under 240000 B, `-2` crates gone). The figures are for 16:26.

## Pass 13 (2026-11-02 21:36)

Scope: `✏️s/🔌️plugins/📕️norm` (din4108, vdi3805, iso16757, din18599, din16798 excluded). Scanner: `🗑️generated/rule-guard/pass13-scan.py` (output `pass13-scan.json`), boundary scan `pass13-source-boundary.txt`.

Counts (in scope):
- 1. paths ≥256 bytes from `/home/deck/semio/`: 2 (en1994 `change-beam-stud-transverse-spacing-m` before/after `🔣️.json`, 256 and 257 bytes). New since pass 12. Routed.
- 2. `[DEBUG]`: 0.
- 3. `//` prose and `//#region` inside bodies (indented, .rs/.ts/.js/.mjs): 0.
- 4. docstrings not starting with an emoji (Rust `///`/`//!`, Python `"""`): 11 found, 11 fixed (all Python, see fixes). Rust: 0.
- 5. wording (legacy, SDK GAP, TODO, FIXME, todo!, unimplemented!, compat layer, deprecat, migrat): 5 hits. 3 are migration wording already routed to shared (requests.md, "migration wording in three test/error strings", not re-routed). 2 are false positives: `let todo: Vec<EntityRef>` variable in `🎬️structural-scene/🎯️projection/🦀️.rs:213-214`. `migrated` framework enum mirror skipped. Zero `legacy`, `SDK GAP`, `TODO`, `FIXME`, `todo!`, `unimplemented!`.
- 6. `-2` names: 0 in scope. The three hits (`failing-2`, `products-2`, `elements-2`) are in excluded trees (vdi3805, iso16757, din4108).
- 7. per-crate budget (`source-boundary-scan.py -v`, scanner gate 32 files / 262144 B / 4000 lines; task band 30 files / 3700 lines / 240000 B). In scope, at or above band:
  - `en1999-checks` 17 files, 252363 B, 3966 lines: routed (en1999 owner).
  - `en1999-mutations` 6 files, 3720 lines: routed (en1999 owner).
  - `en1990-diff` 5 files, 3933 lines: routed (en1990 owner).
  - `en1996-diff` 5 files, 3715 lines: routed (en1996 owner).
  - `en1996-masonry` 19 files, 3830 lines: already routed (pass 12, line 519).
  - six 31-file mutation crates (`en1992-mut-member-fields`, `en1996-mut-loads`, `en1996-mut-piers-lintels`, `en1991-mut-gravity`, `en1991-mut-snow`, `en1996-mut-walls`): already routed (line 522).
- 8. duplicated top-level items in one file (Rust `struct/enum/fn/type/const/static/trait`, `pub XDelta {`, Python `def/class`): 0 after the scanner was fixed to skip `const fn`. The first run reported 6 false positives (`const fn` keys).
- 9. `list_delta!` call sites: 129 in scope. `key: id` appears 3 times, all in `⚖️compliance/🧰️design-case/📇️rows/🦀️.rs`, and the row types `CaseNumber`, `CaseText`, `CaseRow` are defined in that file, so no imported row type. 0 violations.

Fixes (11, all Python docstring openers, emoji prefix added, text unchanged):
- `🧩️en1994/…/🔮️oracles/🧮️catalogue/🐍️.py` lines 186 (🪶️), 262 (🔺️), 308 (🔥️)
- `🔩️en1993/…/🔮️oracles/🕸️frame-eigen/🐍️.py` line 38 (🧾️)
- `🌍️en1997/…/🔮️oracles/⚖️compliance/📇️catalogue.py` line 1 (🗂️) and `📚️geo.py` line 1 (📚️)
- `🏛️en1992/…/🎬️scene/🔮️oracles/🐍️.py` line 102 (🧱️)
- `🏛️en1992/…/🧬️schema/🧪️tests/⚖️compliance/🐍️.py` lines 346 (🔩️) and 357 (🪨️)
- `🔮️oracles/🏃️harness/🔌️adapters/sympy/🐍️.py` line 1: `∑` (not an emoji) replaced by 🧮️
- `📇️registry/…/🕹️direct-manipulation/🔮️oracles/🐍️.py` line 1 (🧲️)

Routed (requests.md, "from: rule-guard pass 13"): en1994 long path (leaf-kinds), en1999-checks, en1999-mutations, en1990-diff, en1996-diff (budget).

Stale routes (no action, status is the owner's): line 517 (`en1999-diff` 6281 lines) is no longer in band: the diff crate is now 1 file, 1498 lines, after the split at line 597. Line 518 (`en1998-diff`) is 1930 lines now. Line 520 (`⚖️compliance`) is 2668 lines, 17 files, below the band.

Notes: `-2` check and budget check now use the task thresholds. The scanner's own 4000-line gate is not reached by any in-scope crate except `en1999-checks` at 3966 (under the gate).

## Pass 14 (2026-11-02 23:10)

Scope: `✏️s/🔌️plugins/📕️norm` (din4108, vdi3805, iso16757, din18599, din16798 excluded). Scanner: `🗑️generated/rule-guard/pass14-scan.py` (output `pass14-scan.txt`, `pass14-scan.json`), boundary scan `pass14-source-boundary.txt`.

Counts (in scope):
- 1. paths ≥256 bytes from `/home/deck/semio/`: 0 (pass 13 en1994 long path `change-beam-stud-transverse-spacing-m` is no longer over; its route at requests.md line 782 can close).
- 2. `[DEBUG]`: 0.
- 3. `//` prose and `//#region` inside bodies: 0.
- 4. docstrings not starting with an emoji: 0.
- 5. wording: 5 hits, all known. 3 are migration wording in test and error strings (already routed to shared, not re-routed). 2 are the `let todo` variable in `🎬️structural-scene/🎯️projection/🦀️.rs:213-214` (false positive). No `legacy`, `SDK GAP`, `TODO`, `FIXME`, `todo!`, `unimplemented!`, `compat layer`.
- 6. `-2` names: 0 in scope. The three hits are in excluded trees (vdi3805, iso16757, din4108).
- 7. per-crate budget (task band: ≥30 files, ≥3700 lines or ≥240000 B), in scope:
  - `en1999-checks` 18 files, 259844 B, 4000 lines: at the 4000-line gate. crate-budget split in progress (requests.md line 825), not re-routed.
  - `en1991-scene` 17 files, 248115 B, 3991 lines: routed (en1991 owner).
  - `en1993-scene` 17 files, 240098 B: routed (en1993 owner).
  - `en1993-clauses` 28 files, 3771 lines: routed (en1993 owner).
  - `en1996-masonry` 19 files, 3858 lines: already routed (line 519, pass 12).
  - `en1996-mutations` 6 files, 3792 lines: routed (en1996 owner).
  - `en1996-diff` 5 files, 3769 lines: already routed (line 786, pass 13).
  - `en1999-mutations` 6 files, 3720 lines: already routed (line 784, pass 13).
  - `en1997-model` 5 files, 3700 lines: routed (en1997 owner, band edge).
  - `en1997-mut-pile` 31 files: routed (en1997 owner). The other five 31-file crates of pass 13 are gone (crate-budget split).
  - Band edge, not flagged: `en1998-mutations` 239670 B.
- 8. duplicated top-level items per file: 0.
- 9. `list_delta!` call sites: 130 in scope (+1 since pass 13). `key: id` appears only in `⚖️compliance/🧰️design-case/📇️rows/🦀️.rs` lines 77-101 (`CaseNumberDelta`, `CaseTextDelta`, `CaseRowDelta`), and the row types `CaseNumber`, `CaseText`, `CaseRow` are defined in that file. 0 violations.
- 10. (new) workspace membership and path. Every in-scope crate `📦️packages/🦀️rust/Cargo.toml` resolves its `workspace = "…"` to a directory with a `[workspace]` table, and is listed in that workspace's `members`.
  - 28 crates resolve to the norm root `✏️s/🔌️plugins/📕️norm` (the literal check).
  - 459 artifact crates resolve to their artifact workspace root (`🗿️artifacts/<artifact>`). This is by design: the norm root excludes `🗿️artifacts/**`, and each artifact root lists its own crates. They are not checked against the norm root.
  - 1 problem found and fixed: `semio-s-plugin-norm-ndp-model` (`🔖️ndp/🧱️model`) was not in the norm workspace `members`. Added to `✏️s/🔌️plugins/📕️norm/Cargo.toml`. Re-scan: 0 problems, 487 ok.

Fixes:
- `✏️s/🔌️plugins/📕️norm/Cargo.toml`: `🧱️model` ndp crate added to `members` (check 10).
- No docstring, comment, DEBUG, or path fixes needed this pass.

Routed (requests.md, "from: rule-guard pass 14"): en1991-scene, en1993-scene and en1993-clauses, en1996-mutations, en1997-mut-pile and en1997-model. Skipped as already routed: en1999-checks (gate, in progress), en1999-mutations, en1996-diff, en1996-masonry, the 3 migration-wording hits (shared).

Notes:
- The pass 14 scanner (`pass14-scan.py`) is the pass 13 scanner plus check 10. It now writes JSON before the workspace check, so the JSON has no workspace block; the check result is in `pass14-scan.txt`.
- No builds and no git writes were run.

## Pass 15 (2026-11-03 00:10)

Scope: `✏️s/🔌️plugins/📕️norm` (din4108, vdi3805, iso16757, din18599, din16798 excluded). Scanner: `🗑️generated/rule-guard/pass15-scan.py` (copy of pass 14 plus checks 11 to 13; output `pass15-scan.txt`, `pass15-scan.json`), boundary scan `pass15-source-boundary.txt` (`source-boundary-scan.py "✏️s/🔌️plugins/📕️norm" -v`, run from the repo root). No builds, no git writes.

Counts (in scope):
- 1. paths ≥256 bytes from `/home/deck/semio/`: 0.
- 2. `[DEBUG]`: 0.
- 3. `//` prose and `//#region` inside bodies: 0.
- 4. docstrings not starting with an emoji: 0.
- 5. wording: 5 hits, all known (unchanged from pass 14): 3 migration wording in test and error strings (shared, routed earlier), 2 false positives (`let todo` in `🎬️structural-scene/🎯️projection/🦀️.rs:213-214`).
- 6. `-2` names: 0 in scope (3 hits in excluded vdi3805, iso16757, din4108 trees).
- 7. per-crate budget (task band: ≥30 files, or ≥3700 lines, or ≥240000 B), in scope:
  - `en1991-scene` 17 files, 248344 B, 3994 lines: routed pass 14 (still open, see requests.md).
  - `en1993-scene` 17 files, 240098 B: routed pass 14 (still open).
  - `en1996-masonry` 19 files, 3865 lines: already routed (pass 12).
  - `en1999-model` 10 files, 3966 lines: new, routed.
  - `en1994-mut-beam-design` 31 files: new, routed.
  - Gone since pass 14 (no longer in band): `en1999-checks`, `en1993-clauses`, `en1996-mutations`, `en1996-diff`, `en1999-mutations`, `en1997-model`, `en1997-mut-pile`.
- 8. duplicated top-level items per file: 0.
- 9. `list_delta!` call sites: 141 in scope (+11 since pass 14). `key: id` appears only as the three struct-body lines in `⚖️compliance/🧰️design-case/📇️rows/🦀️.rs:77,89,101` (row types defined in that file). 0 violations.
- 10. workspace membership and path: 0 problems, 562 ok (scanner output).
- 11. (new) duplicate entries in any workspace `members` or `exclude`: 0 across 12 workspace roots.
- 12. (new) `python3 -I 🧰️scripts/check-cargo-authority.py` (run from the norm root): `checked 28 crates, 28 members, 0 errors`, exit 0.
- 13. (new) every on-disk crate listed in its workspace members and in its umbrella `📜️script.ts` packages list. Scanner reported 27 problems; 26 are real, 1 is a scanner false positive:
  - 19 crates with no umbrella `📜️script.ts` above them that has a `packages` list (12 ndp, 2 compliance, 2 config crates under results and viewport, 3 structural-scene). Routed (shared, infra, ndp, structural-scene).
  - 7 app-surface crates under `📇️registry/🧬️contract/🖥️app-surface/`: the nearest umbrella is the single-package contract umbrella, so they are not in any list. Routed (app-surface and contract owners).
  - 1 false positive: `semio-s-artifact-norm-en1991-mutations-proto` (`🏅️standards/…/🛰️proto-twin/📦️packages/🦀️rust`) is listed in `🗿️artifacts/🏋️en1991/Cargo.toml` members (checked with tomllib) and in the en1991 umbrella packages list. The scanner's workspace lookup missed it. Not routed.

Fixes: none. No trivial item existed: no duplicate members, no missing member, no emoji or comment issue. The umbrella gaps need new or extended script.ts files and are routed instead.

Routed (requests.md, "from: rule-guard pass 15"): en1999-model, en1994-mut-beam-design, the 19 umbrella gaps, the 7 app-surface umbrella gaps. Pass 14 routes for en1991-scene and en1993-scene remain open and were not re-routed.

Notes:
- `pass15-scan.py` check 13 uses the `wsok` list from check 10 to decide workspace listing; that lookup misses the `🏅️standards` proto-twin crate. Membership was verified directly, so the result stands. Scanner fix not made.
- The umbrella test in check 13 accepts either a `packages` list or a single-package `runArtifactRustPackageMain(import.meta.dir, "<name>", …)` umbrella with the same name.

## Pass 16 (2026-11-03)

Scope: `✏️s/🔌️plugins/📕️norm` (din4108, vdi3805, iso16757, din18599, din16798 excluded). Scanner: `🗑️generated/rule-guard/pass16-scan.py` (copy of pass 15 plus checks 14 and 15; output `pass16-scan.txt`). No builds, no git writes. No fixes, no routes: nothing new needed an owner.

Counts (in scope):
- 1. paths ≥256 bytes: 0.
- 2. `[DEBUG]`: 0.
- 3. `//` prose and `//#region`: 0.
- 4. docstrings not starting with an emoji: 0.
- 5. wording: 5, all known (3 migration wording in test and error strings, 2 `let todo` false positives).
- 6. `-2` names: 5 raw hits. 3 are in excluded trees (vdi3805, iso16757, din4108). 2 are in en1995 (`🏠️change-floor-psi-2` under `🧬️schema/🧬️mutations` and `🧪️tests/🧫️fixtures/🧬️mutations`). That id is intentional: it is referenced by `🧪️tests/🪵️mutate-en1995-1/🥒️.feature`, `🦀️.rs` and `🔮️oracles/🔣️.json`, and no `change-floor-psi` sibling exists, so it is not a copy. Scanner false positive, no route.
- 7. per-crate budget: not comparable with pass 15. Pass 15's crate unit is not in the scanner, and a tree-based re-measure gives different numbers. No new route made from this pass. Pass 15 routes stand.
- 8. duplicated top-level items per file: 0.
- 9. `list_delta!` call sites: 141 (unchanged). 0 violations.
- 10. workspace membership: 591 ok, 1 reported. The reported crate `semio-s-artifact-norm-en1991-scene-fields` (`🗿️artifacts/🏋️en1991/🏅️standards/…/🎬️scene/🌈️fields`) IS listed: tomllib shows it in `🗿️artifacts/🏋️en1991/Cargo.toml` members. Scanner false positive (same lookup defect as the pass 15 proto-twin note).
- 11. duplicate workspace members or excludes: 0 across 12 roots.
- 12. `check-cargo-authority.py`: `checked 28 crates, 28 members, 0 errors`, exit 0.
- 13. umbrella and workspace listing: 21 reported, 2 are the scene-fields false positive (it is in the en1991 umbrella `packages` list, verified directly). 19 real, all carried from pass 15 (12 ndp crates, 2 compliance, 2 config, 3 structural-scene). The 7 app-surface gaps from pass 15 no longer appear.
- 14. (new) stray generated outputs:
  - Untracked Cargo.toml in scope: 345. Each judged new and legitimate: unique package name (check 15), 344 of 345 listed in a workspace (check 10), umbrella status as in check 13. No stray duplicate found. Not deleted.
  - `🗃️table` directories in scope: exactly one, `⚖️compliance/📜️annex-and-tables/🔖️ndp/🗃️table/` (12 crates en1990 to en1999), with its module file `🔖️ndp/🗃️table.rs` beside it. This is the intended module layout, not a duplicate. No `🗃️table/` sits next to `🔖️ndp/`, and the repo-wide search found none in the norm tree. Nothing to delete.
  - Non-ignored untracked files in scope: 16550. This is index drift from concurrent work (the git index does not track most of the working tree). Informational, not a rule-guard violation.
- 15. (new) duplicate Cargo package names: 593 packages in scope, 0 duplicated names.

Fixes: none. Deletions: none. Routes: none new. Open routes from pass 15 (19 umbrella gaps, en1999-model, en1994-mut-beam-design, en1991-scene and en1993-scene from pass 14) unchanged.

Scanner note: the `members`/workspace lookup in `pass16-scan.py` misreports two crates under `🗿️artifacts/🏋️en1991` (proto-twin and scene-fields) as not listed, and check 13 inherits that. The scanner was not fixed in this pass.

## Pass 17 (2026-11-03)

Scope: `✏️s/🔌️plugins/📕️norm` (din4108, vdi3805, iso16757, din18599, din16798 excluded). Scanner: `🗑️generated/rule-guard/pass17-scan.py` (copy of pass 16 plus checks 16 to 18; output `pass17-scan.txt`). No builds, no git writes.

Counts (in scope):
- 1. paths ≥256 bytes: 0.
- 2. `[DEBUG]`: 2, both in untracked `📇️registry/🧬️contract/🖥️app-surface/🧪️tests/⏱️latency-bench/🦀️.rs:102,105`. Routed (app-surface and contract owners).
- 3. `//` prose and `//#region`: 0.
- 4. docstrings not starting with an emoji: 1, `🗿️artifacts/🔩️en1993/…/🔮️oracles/🏛️annex-blocks/🐍️.py:101` (`stiffened_plate`). Fixed: `"""📐️ Navier solution …`.
- 5. wording: 5, all known (3 migration wording in test and error strings, 2 `let todo` false positives).
- 6. `-2` names: 5 raw hits, same as pass 16 (3 in excluded trees, 2 intentional `change-floor-psi-2` in en1995). No route.
- 7. per-crate budget: not re-measured (pass 15 routes stand).
- 8. duplicated top-level items per file: 0.
- 9. `list_delta!` call sites: 143 (+2 since pass 16, new en1994 action-row crates). Key-id lines: only the three known compliance row structs (`⚖️compliance/🧰️design-case/📇️rows/🦀️.rs:77,89,101`). 0 violations.
- 10. workspace membership: 601 ok, 0 problems. The two pass 16 false positives (en1991 proto-twin and scene-fields) no longer report.
- 11. duplicate workspace members or excludes: 0 across 12 roots.
- 12. `check-cargo-authority.py`: `checked 28 crates, 28 members, 0 errors`, exit 0.
- 13. umbrella and workspace listing: 22 reported. 19 carried from pass 15 (12 ndp, 2 compliance, 2 config, 3 structural-scene). 3 new: `semio-s-artifact-norm-en1994-mut-beam-action-rows`, `-mut-column-action-rows`, `-mut-slab-action-rows` are listed only in the second of two `runArtifactRustPackageMain` calls in `🗿️artifacts/🧩️en1994/📦️packages/🦀️rust/📜️script.ts`. The first (stale, line 4) is a subset, so the scanner reads it. Routed to en1994 owner (delete line 4).
- 14. stray generated outputs: 354 untracked Cargo.toml in scope (345 in pass 16, all new crates; unique names per check 15). `🗃️table` entries: the intended `🔖️ndp/🗃️table/` directory and `🔖️ndp/🗃️table.rs` module file, same as pass 16. Untracked non-ignored files in scope: 16952 (index drift, informational). Nothing deleted.
- 15. duplicate Cargo package names: 602 packages, 0 duplicated.
- 16. (new) duplicate struct fields in `pub struct`: 0. The `pub placements` duplicate in `En1994Snapshot` is not present: `🗿️artifacts/🧩️en1994/…/🧬️schema/📸️snapshot/🦀️.rs` has one `pub placements` (line 42).
- 17. (new) duplicate enum variants and duplicate `KINDS` entries: 0 real. One scanner false positive: `🎬️structural-scene/🧪️tests/🧫️fixtures/🎭️mock-projection/🦀️.rs:31` (`KINDS: [EntityKindSpec; 2]` of struct literals, split on commas). The entries have distinct ids (`member`, `load`). Mutation root enums (`En1990Mutation` … `En1999MutationDsl`, `NormResults/ViewportWindowConfigMutation`): no duplicate variants.
- 18. (new) `#[dsl(unit = "…")]` symbols not in the framework unit table (`🧰️framework/🔨️modules/🗣️dsl/🔤️token/🦀️.rs`, `static UNITS`, 50 symbols): 0. Used units include `m, N, Pa, m2, s, N/m, m/s, K, J, m/s2, m4, m3, kg, deg, pct, N*m, mm, m2/m, kg/m3, h`, all present.

Fixes: 1 (check 4 docstring emoji, `🐍️.py:101`). No duplicate fields or variants existed, so none were needed.
Routes (requests.md, "from: rule-guard pass 17"): en1994 `script.ts` duplicate umbrella call (3 crates missing from the first call); app-surface `[DEBUG]` lines in `latency-bench` (untracked). Pass 15 routes unchanged (19 umbrella gaps, en1999-model, en1994-mut-beam-design, pass 14 en1991-scene and en1993-scene).

Scanner notes: check 16 and 17 are new in `pass17-scan.py`. The KINDS splitter (check 17) mis-splits struct-literal arrays; the mock-projection case is a known false positive. Check 13 reads only the first `packages` list in a `script.ts`, so a duplicated umbrella call hides later entries. The scanner was not changed for that.

## Pass 18 (2026-11-03)

Scope: `✏️s/🔌️plugins/📕️norm` (din4108, vdi3805, iso16757, din18599, din16798 excluded). Scanner: `🗑️generated/rule-guard/pass18-scan.py` (copy of pass 17 plus checks 19 to 22; output `pass18-scan.txt`, stderr `pass18-scan.err`). No builds, no git writes.

Counts, checks 1 to 18 (pass 17 in brackets):
- 1. paths ≥256 bytes: 14 [0]. All in en1998 mutation fixtures (7 folders, `…/🧪️tests/🧫️fixtures/🧬️mutations/`). New since pass 17. Routed.
- 2. `[DEBUG]`: 0 [2]. The `latency-bench` lines are gone. Resolved.
- 3. `//` prose and `//#region`: 0 [0].
- 4. docstrings not starting with an emoji: 1 [1]. `🗿️artifacts/🏋️en1991/…/🔮️oracles/⚖️compliance/🐍️.py:385` (`ev_more`). Fixed: `"""📚️ EN 1991-1-1 …`. Re-read before the edit.
- 5. wording: 4 [5]. `legacy` in two en1998 test messages (routed). `let todo` in `🎬️structural-scene/🎯️projection/🦀️.rs:213,214` (false positive). The three migration hits from pass 17 are not in the list now.
- 6. `-2` names: 2 [5] (intentional `change-floor-psi-2` in en1995, ψ₂). No route.
- 7. per-crate budget: not re-measured.
- 8. duplicated top-level items per file: 0 [0].
- 9. `list_delta!` call sites: 147 [143]. Not re-checked for key ids this pass.
- 10. workspace membership: 630 ok, 0 problems [601].
- 11. duplicate workspace members or excludes: 0.
- 12. `check-cargo-authority.py`: `checked 28 crates, 28 members, 0 errors`, exit 0.
- 13. umbrella and workspace listing: 24 reported [22]. 19 carried from pass 17. 5 new: `semio-s-artifact-norm-en1999-mut-rows-{sheet-action,connection-action,member-action,shell-action,section-element}` are not in the en1999 umbrella packages list. Routed.
- 14. stray generated outputs: 383 untracked Cargo.toml in scope [354]; 17806 untracked non-ignored files in scope (index drift, informational). Nothing deleted.
- 15. duplicate Cargo package names: 631 packages, 0 duplicated.
- 16. duplicate struct fields: 0 [0].
- 17. duplicate enum variants and KINDS entries: 1 [1], the known `mock-projection` KINDS false positive.
- 18. dsl unit symbols not in the framework table: 0 [0].

New checks (pass 18):
- 19. max/min against table or NDP values without a clause citation in the fn docstring (report only): 119. By artifact: en1992 56, en1991 17, en1996 11, en1993 7, en1998 5, en1995 4, en1994 2, en1997 2, en1990 1, en1999 1; 15 outside artifacts. Heuristic: a window of ±4 lines holds `ndp(`, `NDP`, `TABLE`, `table` or `entry.`; test paths are skipped. Many hits are numeric guards.
- 20. unverified wording vs NDP rows (report only). 20a: 38 explanation strings with "unverified" or "ungeprüft" in files that reference no unverified registry row. 20b: 62 artifact files use an unverified registry row in a string literal without an UnverifiedBasis path (plus 10 registry table crates, expected). Both heuristic, file-level. Registry: `⚖️compliance/📜️annex-and-tables/🔖️ndp/🔣️.json` (2920 rows, 2557 unverified, 363 verified).
- 21. String fields in `🧬️schema` structs whose fixture values form 2 to 5 distinct values: 521 raw hits (enum candidates). By artifact: en1992 150, en1999 124, en1998 71, en1993 46, en1991 30, en1994 29, en1997 24, en1990 22, en1995 16, en1996 2; 7 outside artifacts. Many are `Option<String>` with an empty value; the list is raw per copy, not per field.
- 22. spec ids in spec-trace JSON not in `📐️specs/*.md`: 0 across 11 `🧭️spec-trace/🔣️.json` files.

Fixes: 1 (check 4 docstring emoji, `🐍️.py:385`). No `[DEBUG]` lines remained.
Routes (requests.md, "from: rule-guard pass 18"): en1998 fixture paths (check 1), en1999 umbrella (check 13), en1998 `legacy` wording (check 5), and report-only lists for checks 19, 20a, 20b, 21.
Scanner notes: check 20b is file-level and does not follow helper calls; check 21 counts per copy of a field, so the 521 is not a field count; check 19 window is ±4 lines.

## Pass 19 (2026-11-03)

Scope: `✏️s/🔌️plugins/📕️norm` (din4108, vdi3805, iso16757, din18599, din16798 excluded). Scanner: `🗑️generated/rule-guard/pass19-scan.py` (copy of pass 18 plus checks 23 to 26; output `pass19-scan.txt`, stderr `pass19-scan.err`, empty). No builds, no git writes. Git was read-only (`git diff HEAD`, `git log`, `git ls-files`).

Counts, checks 1 to 22 (pass 18 in brackets):
- 1. paths ≥256 bytes: 0 [14]. The en1998 fixture paths are gone. Resolved.
- 2. `[DEBUG]`: 0 [0].
- 3. `//` prose and `//#region`: 0 [0].
- 4. docstrings not starting with an emoji: 11 [1]. New since pass 18: 7 in `🧪️tests/🧬️schema-sync/🐍️check.py`, 3 in `🧪️tests/🧬️schema-sync/🐍️sync.py`, 1 in `🔮️oracles/🚦️check-outcome/🐍️.py:34`. Fixed (see Fixes).
- 5. wording: 2 [4]. Both are `let todo` in `🎬️structural-scene/🎯️projection/🦀️.rs:213,214` (false positive). The en1998 `legacy` message is gone. Resolved.
- 6. `-2` names: 2 [2]. Intentional `change-floor-psi-2` (ψ₂, en1995). No route.
- 7. per-crate budget: not measured.
- 8. duplicated top-level items per file: 0 [0].
- 9. `list_delta!` call sites: 165 [147]. Informational.
- 10. workspace membership: 662 ok, 0 problems [630 ok].
- 11. duplicate workspace members or excludes: 0 [0].
- 12. `check-cargo-authority.py`: `checked 28 crates, 28 members, 0 errors`, exit 0.
- 13. umbrella and workspace listing: 21 [24]. The five en1999 mut-rows entries are resolved. New: `semio-s-artifact-norm-en1998-mut-site-ground` and `...-en1998-mut-members-wall-rows` not in the en1998 packages list. The 19 "no umbrella script.ts" items carry (NDP tables, compliance, structural-scene, viewport config). Routed.
- 14. stray generated outputs: 423 untracked Cargo.toml in scope [383]; 19242 untracked non-ignored files in scope [17806]. Index drift, informational. Nothing deleted.
- 15. duplicate Cargo package names: 663 packages, 0 duplicated.
- 16. duplicate struct fields: 0 [0].
- 17. duplicate enum variants and KINDS entries: 1 [1], the known `mock-projection` KINDS false positive.
- 18. dsl unit symbols not in the framework table: 0 [0].
- 19. max/min against table value without clause citation (report only): 115 [119].
- 20a. unverified wording without an unverified registry row (report only): 43 [38].
- 20b. unverified NDP row used without UnverifiedBasis path (report only): 74 [62].
- 21. String fields with 2 to 5 fixture values (report only): 512 [521]. Raw per copy, not per field.
- 22. spec ids in spec-trace JSON not in `📐️specs/*.md`: 0 [0].

New checks (pass 19):
- 23. tolerance literals above 1 % in test files, with a tolerance keyword on the line (report only): 73 candidates, 64 in files with an oracle reference. Heuristic: many hits are value literals (`1.0` 31, `0.5` 8, geometry values), not tolerances. Routed with the examples that matter (oracle harness `within(... 2e-2)` and `within(... 1.029)`, en1998 `< 0.08`).
- 24. test files with an expectation change and no oracle change (report only): 0 flagged. 56 test files with changed expectation lines (float literal in an assert/expect/approx line) have mtime within 3 h. Each lies in an artifact root where an oracle file with changed numeric literals also changed. Caveat: the check is root-level, not file-level, and `git log --since=3h` returns no commits. All changes are in the working tree, so the window is mtime-based. About 7674 test files have mtime ≤3 h, so the tree was largely rewritten in this window. Weak signal; the owners should confirm the oracle diffs for the 56.
- 25. EDIT_RULES and match blocks (report only): 1 hit "EDIT_RULES literal without move_item", a false positive (`impl NormEditRules {` in app-surface `✏️edit-rules/🦀️.rs:210`). Every EDIT_RULES literal has `move_item`. Match blocks: 10 hits. Six are false positives where the arms construct enum values (placement-drag:75 `PlacementOp`, edit-rules:485 `RemedyEdit`, canvas-lowering:53 `Tone`, live-lanes editor-wiring:190 `LiveCommand`, en1991 handles:42 `Round`). En1995 timber:43 is a false positive too: `other => other` is a binding wildcard the scanner does not count, and `flipped()` only changes AtLeast/AtMost. Live-lanes editor-wiring:267 and :289 are a tuple match and a nested match, not inspected. Matches needing a decision: app-surface edit-rules:400 (RemedyBound, `_` arm, which now has the new `Insert` variant) and en1994 edit-rules:160 (NormEdit, `_` arm). Routed.
- 26. normative literals in check code outside NDP tables (report only): 2003 hits with no NDP, table or entry reference within ±4 lines. Routed to GAMMA-UNIFY. Partial-factor and ψ literals are a minority; most are 1.0/0.5/1.1 values.

Fixes:
- 4: prefixed 11 docstrings with an emoji. Re-read before each edit, indentation kept, Python AST parse OK for all three files. `🧪️tests/🧬️schema-sync/🐍️check.py` (61, 85, 153, 173, 263, 358, 589), `🧪️tests/🧬️schema-sync/🐍️sync.py` (65, 107, 428), `🗿️artifacts/🏋️en1991/…/🔮️oracles/🚦️check-outcome/🐍️.py` (34).
- No `[DEBUG]` lines. No `move_item` literal needed a fix.

Routes (requests.md, "from: rule-guard pass 19"): en1998 umbrella (check 13), test tolerances (check 23), app-surface and en1994 match decisions (check 25), GAMMA-UNIFY normative literals (check 26). Checks 19, 20a, 20b, 21, 9 are report-only counts; the pass 18 routes stand.

Scanner notes: `pass19-scan.py` writes its JSON to the repo root when given an argument, so run it without one (the stray file made during this pass was removed). Check 25 is a heuristic; it reports arm-body construction as matches and misses binding wildcards (`other =>`), so the false positives and misses above are expected. Check 24 is root-level, see above.

## Pass 20 (2026-11-03)

Scope: `✏️s/🔌️plugins/📕️norm` (din4108, vdi3805, iso16757, din18599, din16798 excluded). Scanner: `🗑️generated/rule-guard/pass20-scan.py` (copy of pass 19 plus checks 27 to 29; output `pass20-scan.txt`, stderr `pass20-scan.err`, empty). Run once, niced, no JSON argument (nothing written to the repo root). Spec-trace verifier run with `.venv/bin/python -I` (no `--write`). No builds, no git writes.

Counts, checks 1 to 22 (pass 19 in brackets):
- 1. paths ≥256 bytes: 1 [0]. `🗿️artifacts/🧩️en1994/…/🧬️mutations/🧵️change-column-reinforcement-product/✅apply/📸️snapshot/⬅️before/🔣️.json` is exactly 256 bytes. New since pass 19. Routed.
- 2. `[DEBUG]`: 0 [0].
- 3. `//` prose and `//#region`: 0 [0].
- 4. docstrings not starting with an emoji: 3 [11]. All three fixed (see Fixes).
- 5. wording: 2 [2]. Both `let todo` in `🎬️structural-scene/🎯️projection/🦀️.rs:213,214` (false positive).
- 6. `-2` names: 2 [2] (intentional `change-floor-psi-2`, ψ₂). No route.
- 8. duplicated top-level items per file: 0 [0].
- 9. `list_delta!` call sites: 167 [165]. Informational.
- 10. workspace membership: 686 ok, 0 problems [662 ok].
- 11. duplicate workspace members or excludes: 0 [0] (12 workspace roots).
- 12. `check-cargo-authority.py`: `checked 28 crates, 28 members, 0 errors`, exit 0.
- 13. umbrella and workspace listing: 19 [21]. 667 umbrella ok. The 19 are the "no umbrella script.ts" items carried from pass 19 (NDP tables, compliance, structural-scene, viewport config). No route change.
- 14. stray generated outputs: 439 untracked Cargo.toml in scope [423]; 19966 untracked non-ignored files in scope [19242]. Index drift, informational. Nothing deleted.
- 15. duplicate Cargo package names: 687 packages, 0 duplicated.
- 16. duplicate struct fields: 0 [0].
- 17. duplicate enum variants and KINDS entries: 1 [1], the known `mock-projection` KINDS false positive.
- 18. dsl unit symbols not in the framework table: 0 [0] (50 table symbols).
- 19. max/min against table value without clause citation (report only): 120 [115].
- 20a. unverified wording without an unverified registry row (report only): 45 [43].
- 20b. unverified NDP row used without UnverifiedBasis path (report only): 80 [74].
- 21. String fields with 2 to 5 fixture values (report only): 484 [512]. Raw per copy, not per field.
- 22. spec ids in spec-trace JSON not in `📐️specs/*.md`: 0 [0].

Checks 23 to 26 (report only, pass 19 in brackets):
- 23. tolerance literals above 1 % in test files: 72 [73], 63 with an oracle reference [64].
- 24. test files with an expectation change and no oracle change: 0 [0]. Pass 19 caveats still hold (mtime window, root-level).
- 25. EDIT_RULES and match blocks: 8 hits [10]. One EDIT_RULES hit is a false positive (`impl NormEditRules {`). Seven match hits are listed in the requests file. Routed.
- 26. normative literals in check code outside NDP tables: 1987 [2003]. Routed to GAMMA-UNIFY as before.

New checks (pass 20):
- 27. `impl Default` and `derive(Default)` on norm types (report only): 339 hits, 51 `impl Default for`, 20 flagged `[choice-type]` by name. Lang and MemberKind have no Default outside tests. Hits that matter, all routed:
  - `ReliabilityClass` `impl Default` → `RC2` (`⚖️action-factors/🦀️.rs:23`). Normative choice.
  - `AnnexChoice::De` in the `impl Default` of `En1990Snapshot` (`en1990 …/📸️snapshot/🦀️.rs:94`) and `En1991Snapshot` (`en1991 …/📸️snapshot/🦀️.rs:309`). Normative choice.
  - `MasonryClass` `derive(Default)` → `Class3` (en1996 `📸️snapshot/🦀️.rs:384`). Normative choice.
  - `RebarProduct` `derive(Default)` → `HotRolled` (en1994 `column-fire/🦀️.rs:62`). Product choice.
  - `SectionMethod` `derive(Default)` → `FiniteElement` (`🧮️mechanics/🧬️schema/🦀️.rs:237`). Method choice.
  - `Tier` (→ `Basic`) and `InspectorOptions` (→ `Tier::Expert`, `UnitSystem::Engineering`) in app-surface. User-choice UI defaults, low.
  - `SectionOptions` (`mechanics …/🦀️.rs:250`, mesh option default). Low.
  - The other derives are scene `Scope::All`, `Layout`, `Out`, `NoConfig`, `NoPresence`, and schema structs with empty or zero defaults. Not flagged.
- 28. paths 247 to 256 bytes, +10 bytes would breach 256 (early warning): 1520 [new check]. 1517 are mutation fixture paths (`🧬️mutations/<change-…>/✅apply/📸️snapshot/⬅️before|➡️after/🔣️.json`) at 247 to 255 bytes. Three are `.rs` paths in en1998 (249, 248) and en1992 (247). The longest kind names are `change-column-reinforcement-product` (en1994) and `change-foundation-pile-moment-rd-nm` (en1998, 255). Routed.
- 29a. spec-trace `🦀️.rs` twins vs `🧭️spec-trace/🧰️codegen/🐍️.py`: exit 0, all 10 artifacts ok (676 ids, en1990 to en1999). No stale twins.
- 29b. schema twins check: not run. No `schema twins check` subcommand exists in any norm `📜️script.ts`; the norm typescript router registers `test` and `wire-twins` only. Routed as an open question to the shared owner.

Fixes:
- 4: prefixed 3 docstrings with an emoji. Re-read before each edit, parse check OK for all three files:
  - `🗿️artifacts/🔩️en1993/…/🔮️oracles/📏️stiffener-outline/🐍️.py:45` `🔬️`
  - `🗿️artifacts/🏛️en1992/…/🧬️schema/🧪️tests/⚖️compliance/🐍️.py:415` `📐️`
  - `🎬️structural-scene/🧰️projection-kit/🗂️design-case/🧊️forms/🔮️oracle/🐍️.py:22` `🔲️`
- No `[DEBUG]` lines. No other trivial items.

Routes (requests.md, "from: rule-guard pass 20"): check 1 (en1994 fixture path at 256), check 28 (1517 fixture paths and 3 `.rs` paths in 247 to 249 bytes), check 27 (AnnexChoice defaults in en1990 and en1991, ReliabilityClass, MasonryClass, RebarProduct, SectionMethod, app-surface UI defaults), check 25 (8 match hits), check 29b (no schema twins command).

Scanner notes: check 28 walks file and directory entries in scope; all 1520 hits are files. Check 27 excludes `🧪️tests` and `/tests/` paths; fixture snapshot defaults under `🧬️schema/📸️snapshot` are included. Check 29a runs the verifier with the venv python, niced, read-only.

## Pass 21 (2026-11-03)

Scope: `✏️s/🔌️plugins/📕️norm` (din4108, vdi3805, iso16757, din18599, din16798 excluded). Scanner: `🗑️generated/rule-guard/pass21-scan.py` (copy of pass 20 plus checks 29b, 30 to 32; output `pass21-scan.txt`, stderr `pass21-scan.err`, empty). Run once, niced, stdin closed, no JSON argument (nothing written to the repo root). Outputs of the extra runs: `pass21-twins.txt` and `pass21-twins.err` (29b), `pass21-schema-sync.json` and `pass21-schema-sync.txt` (31). No builds, no git writes.

Counts, checks 1 to 22 (pass 20 in brackets):
- 1. paths ≥256 bytes: 0 [1]. The en1994 fixture path is gone. Longest path in scope is 249 bytes (check 28). Resolved.
- 2. `[DEBUG]`: 0 [0].
- 3. `//` prose and `//#region`: 0 [0].
- 4. docstrings not starting with an emoji: 0 [3]. Pass 20 fixes hold.
- 5. wording: 2 [2]. Both `let todo` in `🎬️structural-scene/🎯️projection/🦀️.rs` (false positive).
- 6. `-2` names: 2 [2] (intentional `change-floor-psi-2`, ψ₂). No route.
- 8. duplicated top-level items per file: 0 [0].
- 9. `list_delta!` call sites: 167 [167]. Informational.
- 10. workspace membership: 699 ok, 0 problems [686 ok].
- 11. duplicate workspace members or excludes: 0 [0] (12 workspace roots).
- 12. `check-cargo-authority.py`: `checked 30 crates, 30 members, 0 errors`, exit 0 [28].
- 13. umbrella and workspace listing: 20 [19]. 679 umbrella ok. The new item is `semio-s-plugin-norm-results-config` (`🪟️results/🎚️config`, no umbrella `📜️script.ts`). The other 19 are carried from pass 20.
- 14. stray generated outputs: 452 untracked Cargo.toml in scope [439]; 23233 untracked non-ignored files in scope [19966]. Index drift, informational. Nothing deleted.
- 15. duplicate Cargo package names: 700 packages, 0 duplicated [687].
- 16. duplicate struct fields: 0 [0].
- 17. duplicate enum variants and KINDS entries: 1 [1], the known `mock-projection` KINDS false positive.
- 18. dsl unit symbols not in the framework table: 0 [0] (50 table symbols).
- 19. max/min against table value without clause citation (report only): 122 [120].
- 20a. unverified wording without an unverified registry row (report only): 47 [45].
- 20b. unverified NDP row used without UnverifiedBasis path (report only): 81 [80].
- 21. String fields with 2 to 5 fixture values (report only): 479 [484]. Raw per copy, not per field.
- 22. spec ids in spec-trace JSON not in `📐️specs/*.md`: 0 [0].

Checks 23 to 26 (report only, pass 20 in brackets):
- 23. tolerance literals above 1 % in test files: 74 [72], 65 with an oracle reference [63].
- 24. test files with an expectation change and no oracle change: 0 [0]. The mtime window is now empty (no test file with changed expectation lines in the last 3 h).
- 25. EDIT_RULES and match blocks: 9 [8]. The `impl NormEditRules {` false positive is still there. New hit against pass 20: en1994 `🧭️edit-rules/🦀️.rs:164` (NormEdit, lacks `InsertItem`, `MoveItem`, no wildcard). Others as in pass 20 (en1991 handles:43, grips:47, edit-rules:500, canvas-lowering:53, placement-drag:75, live-lanes editor-wiring:191 and :268). Routed.
- 26. normative literals in check code outside NDP tables: 1988 [1987]. Routed to GAMMA-UNIFY as before.

New checks (pass 21):
- 27. `impl Default` and `derive(Default)` on norm types (report only): 316 hits [339], 17 flagged `[choice-type]` [20]. The en1990 and en1991 `AnnexChoice` defaults and the `MasonryClass` default are gone from the list.
- 28. paths 247 to 256 bytes (early warning): 3 [1520]. All three are `.rs` paths: en1998 249 and 248 bytes, en1992 247 bytes. The 1517 mutation fixture paths are no longer in the band. Routed (pass 20 routes for the three `.rs` paths stand).
- 29a. spec-trace `🦀️.rs` twins vs `🧭️spec-trace/🧰️codegen/🐍️.py`: exit 0, all 10 artifacts ok (676 ids, en1990 to en1999; per-artifact counts 29, 109, 159, 142, 54, 68, 37, 42, 32, 42). No stale twins.
- 29b. `repo ./📜️script.ts schema twins check --strict` (niced, stdin closed): does not run. Exit 1 in `📦️packages/🟦️typescript/` with `error: Original parent Script process invocation required` (the `📜️script.ts` router rejects a non-framework invocation; the same error came from `🧮️mechanics/📦️packages/🦀️rust/` and `🏭️bridge/`). The norm typescript router registers only `test` and `wire-twins`, so no `schema twins` subcommand is reachable from these scripts. Not counted. Routed.
- 30. duplicate scenario `@id-…` tags within one `.feature` file: 0 (114 `.feature` files scanned).
- 31. hand edits in generated twins (fallback): `schema twins check --strict` is not runnable (29b), so the fallback is `🧪️tests/🧬️schema-sync/🐍️check.py --all` (schema JSON against Rust, TS, proto and GraphQL twins, read-only; its JSON report is `pass21-schema-sync.json`). Result: 1 mismatch [new], `en1998:🧬️schema/🔺️diff En1998Diff` (`joints`: JSON optional, Rust twin required). This is a schema-vs-twin mismatch, not proof of a hand edit; the generator output was not compared. Note: a first run at 03:43 showed 5 mismatches (`effectiveStiffnessRatio` on En1998Member and En1998Member snapshot, `storeyStiffnessRatio` on En1998Building, plus the `joints` one). The four field mismatches were gone by the scan at 03:50, so another owner changed the en1998 twins in between. Routed.
- 32. `impl Default` on norm snapshots or choice enums, after NO-DEFAULTS v1/v2 (from check 27): 16 hits [new check]. Breakdown:
  - en1994 `📸️snapshot/🦀️.rs:54`: `impl Default for En1994Snapshot`. Still present; NO-DEFAULTS v2 deferred it (`🔍️audits/wave2/no-defaults-v2.md`, en1994 section).
  - en1994 `📸️snapshot/🦀️.rs` derive(Default) on structs SteelSection (140), ProfiledSheeting (168, `[choice-type]`), HeadedStuds (185), CharacteristicAction (209), SlabAction (238), ColumnAction (270), CompositeBeam (289), CompositeColumn (346), CompositeSlab (396), Placement (420). Snapshot types, not choices, except ProfiledSheeting.
  - en1990 `📸️snapshot/🦀️.rs:366`: derive(Default) on BridgeSls.
  - Choice-type, not snapshot: `⚖️action-factors/🦀️.rs:23` `impl Default for ReliabilityClass` → `Self::RC2` (normative, still open from pass 20); `🧮️mechanics/🧬️schema/🦀️.rs:249` `impl Default for SectionOptions`; app-surface `🔖️field-meta/🦀️.rs:49` `derive(Default)` on enum `Tier` (default `Basic`); app-surface `🪟️visual-windows/🧾️properties/🦀️.rs:35` `impl Default for InspectorOptions` (`UnitSystem::Engineering`, `Tier::Expert`).
  - Not in check 32 any more: the en1991 `AnnexChoice` default, `MasonryClass`, `RebarProduct`, `SectionMethod`. Pass 20 items for them are closed as far as this scan shows.

Fixes:
- None. Checks 3, 4, 2 and the docstring check are at 0. The two `let todo` hits are false positives. No file edited, no `[DEBUG]` lines.

Routes (requests.md, "from: rule-guard pass 21"): check 13 (results-config umbrella), check 25 (en1994 edit-rules:164 match), check 29b (no reachable `schema twins check`, the `repo` invocation fails), check 31 (en1998 `En1998Diff.joints` optional mismatch), check 32 (en1994 snapshot `impl Default`, en1994 and en1990 snapshot `derive(Default)`, ReliabilityClass, SectionOptions, Tier, InspectorOptions). Check 28 `.rs` paths and check 27 open items from pass 20 stand.

Scanner notes: the first run hung for 15 min because the `repo` CLI call inside the scanner held the captured stdout pipe open. Check 29b in the scanner now reads the separately captured `pass21-twins.txt` and `.err` (both from a shell run with stdin closed, 2 s). Check 31 writes `pass21-schema-sync.json` next to the scanner.
