# Result audit14 (read-only apply-readiness audit of the G48 queue)

Lane audit14, wave 1027b, after G47 landed (21:01). Live root `/Users/ueli/Documents/semio`. Nothing live was edited, no stage.py apply or revert ran, no git modifying command ran, no mirror was created, no sub-agent ran.

Method: each batch's `stage.py check --root <live>` (exit code and per-entry lines), plus my own sha256 comparison of every manifest row against live, base/ and files/ bytes. Overlaps are exact repo paths present in two or more manifests. Same-base pairs were 3-way merged with `diff3 -m` (mine = A.new, older = base, yours = B.new). Whole-file removals were counted with difflib (base to new). Raw data: `C/🗑️generated/audit14/` (report.json, summary.txt, pass2.txt, pass3.txt, batch-table.md, drift-classes.md, stack-deps.json, merge/). Scripts: `analyze.py`, `analyze2.py`, `analyze3.py` in the same folder.

## FOR coordinator

- Nothing in this queue can be applied as one sequence by stage.py alone. 51 of 661 in-scope rows are drifted on live, and 95 path pairs were cut from the same base (or from unrelated bases), so the second batch of each pair fails `check` once the first lands. Use the rebuild scripts in the order of section 6, not the raw `apply_all.sh` lines.
- Already live, do not apply: `edit30/freshmtime` (4 of 4 rows = new) and `edit30/prodclock` (1 of 1). `cbudget/clock` is half-landed (7 of 9 rows = new, 2 drifted); it is outside your list, but `cbudget/shard` was cut independently of it.
- Superseded, do not apply the lane copy: `nolegacy/reapi` (use `audit11-tools/staged/reapi`, which has the same 4 bases; audit11 reports the lane copy refuses REAPI 2.1 `output_paths` commands, which I did not re-run) and `edit30/warmderive` (use `audit11-tools/staged/warmderive`, the copy audit11 verified; its memo schema bytes differ from edit30's).
- Two real collisions need a decision, not just a rebuild: (1) `nolegacy/nobaseline` deletes the `📒️baseline` module, and the language-law allowlist row for it lives in `lang`; land `lang` and `nobaseline` in the same pass, `lang` first. (2) `compliance/libbuild` and `isolate/helper` both change `🧪️tests/🕸️graph/🧰️support/🟦️.ts` from different bases; compliance lands first, then `isolate-tools/rebuild.sh helper`.
- Scope notes: `goyaml-tools/staged` does not exist (skipped). `germanqa` (7 batches, queued for G48 in coordination.md 21:07) and `clock` are not in your list and were not audited beyond their effect on the listed batches.

## 1. Batch table

Columns: entries; check on live as base/applied/drifted (from `stage.py check`, matches my hash count); shared paths; pairs that need a rebase because the bases differ (same base or unrelated); pairs that are stacked (B.base = A.new, one linear order); whole-file lines removed and added; result file used.

| Batch | Entries | base/applied/drifted | Shared paths | Rebase pairs | Stacked pairs | -removed +added | Result file | Verdict |
|---|---|---|---|---|---|---|---|---|
| compliance/fwmodules | 3 | 3/0/0 | 1 | 1 | 0 | -9 +9 | result-compliance.md | READY (group B) |
| compliance/fwprocess | 13 | 12/0/1 | 4 | 6 | 0 | -22 +20 | result-compliance.md | REBUILD (rebuild_all.sh) |
| compliance/lang | 52 | 52/0/0 | 19 | 17 | 4 | -160 +411 | result-compliance.md | READY on check; group B; 3 pairs need merge (see 2) |
| compliance/libbuild | 22 | 21/0/1 | 11 | 11 | 0 | -108 +105 | result-compliance.md | REBUILD (BRANCH on isolate/helper base) |
| compliance/libgraph | 16 | 15/0/1 | 10 | 15 | 1 | -90 +84 | result-compliance.md | REBUILD (RE-CUT) |
| compliance/libother | 3 | 3/0/0 | 1 | 1 | 0 | -3 +2 | result-compliance.md | READY (group B) |
| compliance/libtools | 3 | 3/0/0 | 1 | 1 | 0 | -4 +4 | result-compliance.md | READY (group B) |
| compliance/names | 36 | 36/0/0 | 0 | 0 | 0 | -1737 +1737 (16 renames) | result-compliance.md | READY (group B) |
| nolegacy/lang | 37 | 28/0/9 | 10 | 8 | 3 | -580 +792 | result-nolegacy.md | REBUILD (rebuild.py), after group B; lands with nobaseline |
| nolegacy/lease | 23 | 14/0/9 | 11 | 12 | 0 | -186 +114 | result-nolegacy.md | REBUILD (rebuild.py), after group B |
| nolegacy/nobaseline | 17 | 16/0/1 | 2 | 2 | 0 | -809 +12 | result-nolegacy.md | REBUILD (1 file), with lang |
| nolegacy/reapi | 4 | 4/0/0 | 4 | 2 | 2 | -14 +16 | result-nolegacy.md | SUPERSEDED by audit11/reapi, do not apply |
| schemafirst/law | 3 | 3/0/0 | 0 | 0 | 0 | -0 +358 | result-schemafirst.md | first of group E |
| schemafirst/revision | 4 | 4/0/0 | 0 | 0 | 0 | -3 +49 | result-schemafirst.md | group E (rebuild_all.sh) |
| schemafirst/observation | 8 | 5/0/3 | 2 | 2 | 0 | -30 +146 | result-schemafirst.md | REBUILD (rebuild_all.sh) |
| schemafirst/generations | 13 | 11/0/2 | 4 | 4 | 0 | -28 +240 | result-schemafirst.md | REBUILD (rebuild_all.sh) |
| schemafirst/graphcore | 6 | 5/0/1 | 1 | 2 | 0 | -20 +87 | result-schemafirst.md | REBUILD (rebuild_all.sh) |
| schemafirst/memo | 6 | 5/0/1 | 4 | 6 | 0 | -13 +130 | result-schemafirst.md | REBUILD; before audit11/warmderive (memo conflicts) |
| schemafirst/arguments | 2 | 2/0/0 | 1 | 1 | 0 | -4 +16 | result-schemafirst.md | group E (rebuild_all.sh after B) |
| schemafirst/cachegc | 7 | 6/0/1 | 3 | 3 | 0 | -9 +90 | result-schemafirst.md | REBUILD (rebuild_all.sh) |
| schemafirst/hostcap | 7 | 2/0/5 | 2 | 3 | 0 | -13 +92 | result-schemafirst.md | REBUILD (rebuild_all.sh); 1 merge conflict with compliance/lang |
| schemafirst/inference | 8 | 8/0/0 | 4 | 9 | 0 | -10 +76 | result-schemafirst.md | group E (rebuild_all.sh after B; 1 merge conflict with compliance/lang) |
| schemafirst/bazelsessions | 4 | 4/0/0 | 2 | 2 | 0 | -5 +60 | result-schemafirst.md | group E (rebuild_all.sh after B) |
| schemafirst/bazelscript | 3 | 2/0/1 | 1 | 3 | 0 | -9 +66 | result-schemafirst.md | REBUILD (BRANCH: script.ts base is older) |
| schemafirst/buildmisc | 10 | 10/0/0 | 3 | 3 | 0 | -7 +133 | result-schemafirst.md | group E; 1 merge conflict with langpick/imports |
| schemafirst/readoracle | 6 | 6/0/0 | 1 | 1 | 0 | -19 +131 | result-schemafirst.md | group E (rebuild_all.sh after B) |
| schemafirst/storelimits | 3 | 3/0/0 | 0 | 0 | 0 | -3 +47 | result-schemafirst.md | group E |
| edit30/freshmtime | 4 | 0/4/0 | 0 | 0 | 0 | -2 +109 | result-edit30.md | ALREADY LIVE |
| edit30/prodclock | 1 | 0/1/0 | 0 | 0 | 0 | -3 +7 | result-edit30.md | ALREADY LIVE |
| edit30/nxbuild | 3 | 3/0/0 | 0 | 0 | 0 | -8 +44 | result-edit30.md | READY (no shared path) |
| edit30/witpub | 7 | 6/0/1 | 1 | 3 | 0 | -108 +143 | result-edit30.md | REBASE (edit.py): script.ts BRANCH |
| edit30/warmderive | 10 | 9/0/1 | 10 | 15 | 1 | -9 +341 | result-edit30.md | SUPERSEDED by audit11/warmderive |
| langpick/core | 27 | 27/0/0 | 5 | 2 | 4 | -249 +428 | result-langpick.md | REBUILD first (build.py), then land.sh step 1 |
| langpick/copies | 33 | 32/0/1 | 6 | 6 | 0 | -142 +137 | result-langpick.md | REBUILD (build.py) |
| langpick/dashboard | 16 | 16/0/0 | 0 | 0 | 0 | -161 +71 | result-langpick.md | READY (no shared path); fixture removal needs review (section 4) |
| langpick/imports | 35 | 33/0/2 | 7 | 10 | 1 | -38 +34 | result-langpick.md | STACKED on core; REBUILD (build.py) |
| langpick/launcher | 7 | 6/0/1 | 1 | 0 | 1 | -48 +92 | result-langpick.md | STACKED on core |
| langpick/launcher2 | 1 | 0/0/1 | 1 | 0 | 1 | -12 +0 | result-langpick.md | STACKED on core |
| langpick/routing | 9 | 7/0/2 | 5 | 5 | 1 | -93 +158 | result-langpick.md | STACKED on core; REBUILD (build.py) |
| benchci/acceptance | 14 | 14/0/0 | 1 | 1 | 0 | -50 +5332 | result-benchci.md | READY (group A); lease README rebases after it |
| cbudget/shard | 4 | 4/0/0 | 0 | 0 | 0 | -1 +131 | result-cbudget.md | READY (group A), optional |
| nxdeadline/release | 7 | 7/0/0 | 0 | 0 | 0 | -12 +104 | result-nxdeadline.md | READY (group A) |
| isolate/helper | 3 | 3/0/0 | 1 | 1 | 0 | -90 +117 | result-isolate.md | READY on check; after group B, rebuild.sh helper |
| isolate/budget | 17 | 17/0/0 | 1 | 1 | 0 | -49 +83 | result-isolate.md | READY after helper |
| isolate/buildb | 16 | 16/0/0 | 2 | 2 | 0 | -40 +72 | result-isolate.md | READY after helper |
| isolate/cachinga | 12 | 12/0/0 | 0 | 0 | 0 | -40 +63 | result-isolate.md | READY after helper |
| isolate/cachingb | 12 | 12/0/0 | 0 | 0 | 0 | -36 +58 | result-isolate.md | READY after helper |
| isolate/enforce | 13 | 13/0/0 | 0 | 0 | 0 | -35 +61 | result-isolate.md | READY after helper |
| isolate/miscb | 11 | 11/0/0 | 0 | 0 | 0 | -23 +45 | result-isolate.md | READY after helper |
| isolate/tools | 19 | 19/0/0 | 0 | 0 | 0 | -68 +103 | result-isolate.md | READY after helper |
| isolate/builda | 14 | 10/0/4 | 1 | 1 | 0 | -26 +54 | result-isolate.md | REBUILD (rebuild.sh builda) |
| isolate/misca | 11 | 10/0/1 | 0 | 0 | 0 | -16 +38 | result-isolate.md | REBUILD (rebuild.sh misca) |
| isolate/fence | 13 | 12/0/1 | 1 | 1 | 0 | -5 +351 | result-isolate.md | REBUILD (rebuild.sh fence), LAST |
| incfacts/libfacts | 9 | 9/0/0 | 4 | 8 | 0 | -42 +75 | result-incfacts.md | READY now, but drifts after B, E, F; apply LAST after make_batch.py |
| audit11/reapi | 4 | 4/0/0 | 4 | 2 | 2 | -14 +16 | result-audit11.md | READY; use INSTEAD of nolegacy/reapi (group A) |
| audit11/warmderive | 10 | 10/0/0 | 10 | 15 | 1 | -9 +457 | result-audit11.md | REBUILD (rebuild.py on fresh mirror) after B and E; INSTEAD of edit30 |

Totals (661 in-scope rows incl. audit11 copies; clock excluded): base 605, applied 5, drifted 51. Repo paths in two or more manifests: 77 (clock and audit11 copies included), 107 pairs.

## 2. Overlaps (same repo path in two batches)

Lane pairs with shared paths: compliance-schemafirst 23, compliance-nolegacy 13, audit11-edit30 10, compliance-langpick 8, langpick-schemafirst 6, langpick-nolegacy 6, compliance-isolate 4, edit30-schemafirst 4, compliance-incfacts 4, audit11-nolegacy 4, langpick-langpick 4, audit11-schemafirst 3, and smaller pairs.

Pairs are of three kinds:
- Linear (9): B.base = A.new, so the order is fixed and no merge is needed. compliance/lang then nolegacy/lang (2 paths), compliance/libgraph then nolegacy/lang (1), langpick/core then routing, imports, launcher2, launcher (4), cbudget/clock then compliance/lang (2; clock is already live, so that link is satisfied). Identical new (3): nolegacy/reapi = audit11/reapi on 2 paths, edit30/warmderive = audit11/warmderive on 1.
- Same base (73 pairs): 6 of them are creations of the same new file from both sides (no base, so no merge; all are edit30 vs audit11 warmderive new files), and 67 are testable with diff3: 54 clean, 13 with conflicts.
- Unrelated bases (22 pairs): no common ancestor, so no 3-way merge. These need the second batch rebuilt from live.

The 13 conflicting pairs (diff3 hunks in brackets):

1. compliance/fwprocess vs nolegacy/lease: `🧭️order/🗂️registry/🟦️.ts` [2]
2. compliance/lang vs schemafirst/hostcap: `🌡️host-capacity/📡️sampling/🟦️.ts` [1]
3. compliance/lang vs schemafirst/inference: `📦️project-nodes/🟨️.mjs` [1]
4. compliance/lang vs nolegacy/lease: `🚢️fleet-status/🟦️.ts` [2]
5. compliance/libbuild vs nolegacy/lease: `🚢️fleet-status/🧪️tests/🐍️.py` [1]
6. compliance/libgraph vs nolegacy/lang: `🕰️generations/✂️cutoff/🟨️.mjs` [1]
7. edit30/warmderive vs audit11/warmderive: `🧭️script-inputs/🟨️.mjs` [5]
8. edit30/warmderive vs audit11/warmderive: `🧠️memo/🟦️.ts` [1]
9. schemafirst/memo vs edit30/warmderive: `🧠️memo/🟦️.ts` [1]
10. schemafirst/memo vs audit11/warmderive: `🧠️memo/🟦️.ts` [1]
11. schemafirst/buildmisc vs langpick/imports: `⌨️command/⚙️runtime/🟦️.ts` [1]
12. nolegacy/reapi vs audit11/reapi: `👷️worker/🟦️.ts` [6] and `🧪️tests/🟦️.ts` [4]

Resolution: 7 and 8-10 are resolved by using audit11 warmderive instead of edit30 and by landing schemafirst memo before audit11 (the rebuild re-derives the memo schema json, which differs in all three cuts; see section 3). 12 is resolved by choosing audit11 reapi. Items 1-6 and 11 are resolved by the lanes' own rebuild scripts after the earlier batch lands. Nothing else needs a hand merge.

Remaining same-base pairs that merge cleanly (54) need no action beyond the rebuild of the later batch.

## 3. Drifted entries and what each one needs (51 rows)

Classes: STACKED = the row's base is the new of an unlanded batch, so land that batch first. BRANCH = live equals the base of another batch, so this batch was cut on an older tree; rebuild. RE-CUT = live matches no batch in the queue (a landing or peer edit after the cut); rebuild from live. Full per-row list: `C/🗑️generated/audit14/report.json` and `pass2.txt`.

| Batch | Rows | Class | Example files | Action |
|---|---|---|---|---|
| compliance/fwprocess | 1 | RE-CUT | `🧭️routing/…/🏃️process/🟦️.ts` | rebuild_all.sh |
| compliance/libbuild | 1 | BRANCH (isolate/helper base) | `🧪️tests/🕸️graph/🧰️support/🟦️.ts` | rebuild_all.sh; isolate/helper then rebuilt on top |
| compliance/libgraph | 1 | RE-CUT | `🕰️generations/🟨️.mjs` | rebuild_all.sh |
| nolegacy/lang | 9 | 3 STACKED (compliance/lang, libgraph), 6 BRANCH/RE-CUT | `📖️texts/🔣️.json` (7 catalogs), `native-manifests/🟨️.mjs` | rebuild.py after group B |
| nolegacy/lease | 9 | 5 RE-CUT, 4 BRANCH (langpick/copies, routing, benchci README) | `📈️standing/🧬️schema/🔣️.json`, `📈️progress.json`, `README.md` | rebuild.py after group B and benchci |
| nolegacy/nobaseline | 1 | RE-CUT | `🗂️schema-document-index.json` (shared with clock) | rebuild.py; decide clock first |
| schemafirst/bazelscript | 1 | BRANCH (compliance/lang base) | `🔧️tools/🏗️bazel/📜️script.ts` | rebuild_all.sh |
| schemafirst/cachegc | 1 | RE-CUT | `🧹️cache-gc/⌨️command/🟦️.ts` | rebuild_all.sh |
| schemafirst/generations | 2 | RE-CUT | `🕰️generations/🟨️.mjs`, `🧩️claims/🟨️.mjs` | rebuild_all.sh |
| schemafirst/graphcore | 1 | RE-CUT | `👁️observation/🧪️tests/📥️reuse/🟦️.ts` | rebuild_all.sh |
| schemafirst/hostcap | 5 | 4 RE-CUT, 1 BRANCH (compliance/fwprocess, langpick/core) | `🧬️schema/🔣️.json`, `📏️decision/🟦️.ts` | rebuild_all.sh |
| schemafirst/memo | 1 | BRANCH (audit11/warmderive base) | `🧬️schema/🔣️.json` | rebuild_all.sh; then audit11 rebuild |
| schemafirst/observation | 3 | 2 BRANCH (compliance/libbuild), 1 RE-CUT | `👁️observation/🟨️.mjs`, `🟨️.d.mts`, fixture | rebuild_all.sh |
| edit30/warmderive | 1 | BRANCH (audit11 base) | memo `🧬️schema/🔣️.json` | superseded |
| edit30/witpub | 1 | BRANCH (compliance/lang base) | `🔧️tools/🏗️bazel/📜️script.ts` | edit.py rebase, then apply |
| langpick/copies | 1 | RE-CUT | `🚪️door/🟦️.ts` | build.py |
| langpick/imports | 2 | 1 STACKED (langpick/core), 1 RE-CUT | `📦️packages/🟦️typescript/…/🗣️language/🟦️.ts`, `⌨️command/🟦️.ts` | build.py after core |
| langpick/launcher | 1 | STACKED (langpick/core) | `🧪️tests/🗣️language-contract/🐹️.go` | after core |
| langpick/launcher2 | 1 | STACKED (langpick/core) | `🗂️registry/🔣️.json` | after core |
| langpick/routing | 2 | 1 STACKED (langpick/core), 1 RE-CUT | `🗣️language/🧪️tests/🟦️.ts`, `🧭️routing/…/🟦️.ts` | build.py after core |
| isolate/builda | 4 | RE-CUT | `👁️observation/🧪️tests/🧱️blocks/…`, `🪪️presence/…` | rebuild.sh builda |
| isolate/misca | 1 | RE-CUT | `📦️artifacts/🏗️native-build/🧪️tests/🟦️.ts` | rebuild.sh misca |
| isolate/fence | 1 | RE-CUT | `⚡️caching/💾️storage/🧾️ledger/📖️texts/🔣️.json` | rebuild.sh fence |
| cbudget/clock (out of list) | 2 | RE-CUT | `🗂️schema-document-index.json`, `📓️build-guide.md` | rebuild.py (clock) |

Notes: `schemafirst/bazelscript` and `edit30/witpub` drifted on `script.ts` because `compliance/lang` and `langpick/imports` changed that file after their cut. `schemafirst/memo` and `edit30/warmderive` drifted on the memo schema JSON because live already holds the bytes audit11 cut from (`f4ccee8b`), which is neither their base (`b1e833c6`) nor their new; something outside the queue changed that file after schemafirst was cut. None of the three can be applied as is.

## 4. Whole-file removals (base to new)

Full table: `C/🗑️generated/audit14/summary.txt` (WHOLE-FILE LINE DELTA). 31 entries delete a file (new = null); 16 of those are the `compliance/names` renames, each with a matching creation (identical or 94-100 percent line overlap), so no code is lost there.

Flagged removals (removed lines of 15 or more, or a deletion without a creation):

- `nolegacy/nobaseline` -809 in 17 files (9 deleted, no creation). Intended: the unwired `📒️baseline` module. Verified against live with `git grep`: every reference outside this batch is either inside the batch's own new bytes (checked: 0 baseline references left in any new file of the batch) or is the language-law allowlist row (`🔒️allowlist/🔣️.json` key for `baseline/🟦️.ts`, line 154). The allowlist row is removed only by `nolegacy/lang` (its new has 0 references; `compliance/lang` new still has 1). So nobaseline must not land without lang.
- `nolegacy/lang` -358 and -61 in catalog JSON (`📖️texts/🔣️.json`), plus -35 and -34 in others. Intended: refusal texts moved into their own declared table and EN+DE catalogs. Keys removed are replaced in the same files. Not verified key by key.
- `langpick/core` -249 in 27 files (the protocol moves from `process/build/language` to `runtime/language`). Moved modules are rewritten, not byte-identical (line overlap 0.15 to 0.32, by set-of-lines Jaccard). Live importers of the deleted index `process/build/language/🟦️.ts`: none outside manifests. The one outside hit is a Bazel scope test fixture fragment `"/🗣️language/🟦️.ts"`, which is an ends-with match, so it still matches after the move. Not run.
- `langpick/imports` -3 (deletion of a 3-line file in `packages/typescript/build/language`). Intended, presumably the re-export that the move makes unnecessary; not checked against the file content.
- `langpick/dashboard` -111 in `🌐️language/🔣️.json` (test fixture, 486 to 375 lines). Removed rows are resolve cases with `environment` and `expected: null`. Plausible for the language-protocol change, but unverified. Review before apply.
- `langpick/launcher` -31 in `🐹️.go` (protocol struct fields `EnvironmentOrder`, `ExplicitVariable`, `PreferenceListVariable` removed). Intended by the protocol move; check the launcher build.
- `langpick/launcher2` -12 in the launcher registry JSON. Intended (registry path change in core).
- `isolate/helper` -81 in `🧰️support/🟦️.ts` (106 to 33 lines). Intended supersede of the G46 optcache helper, per result-isolate.md. The same file is also edited by `compliance/libbuild` (section 3).
- `nolegacy/lease` -83 (fixture) and -26 (schema), plus -186 overall. Intended: legacy holder state and `legacyAfterMilliseconds` removed.
- `edit30/witpub` -108 overall (-43, -35, -19 in witness tests and runner). Intended per result-edit30.md (runner moved into the witness module).
- `benchci/acceptance` -38 in the 600 KB help snapshot, +112. Intended; the result says never hand-merge it, rebase it.
- `compliance/libbuild` observation mjs -45 / +45 and `compliance/libgraph` script-inputs -41 / +41: docstring and comment rewrites (the emoji opener rule). Not re-checked here.
- `schemafirst/*`: removals are small (max -30, observation) and the result says the literals moved into schemas.

No removal found that drops live code from another landing without a re-add or a documented delete, except items that need an owner review: `langpick/dashboard` fixture rows, `nolegacy/lang` catalog keys, and the `langpick/imports` 3-line file.

## 5. Result-file rebuild requirements

- compliance: result-compliance.md says `rebuild_all.sh` after any landing in between; result-audit11.md says libgraph, lang and names land after `rebuild_all.sh`. Only 3 rows are drifted now, but a rebuild is needed for those three anyway.
- nolegacy: result-nolegacy.md says `python3 -I C/nolegacy-tools/rebuild.py` after compliance (its base is compliance's new). result-audit11.md says rebuild lang with rebuild.py. `reapi`: use audit11's copy.
- schemafirst: result-schemafirst.md says `sh C/schemafirst-tools/rebuild_all.sh` after compliance and nolegacy; law first.
- edit30: warmderive is superseded by audit11's copy, which needs `rebuild.py <fresh mirror>` (audit11 result). witpub needs `witpub/edit.py <fresh root> apply` after compliance lang and schemafirst bazelscript, then `make-stage.py`.
- langpick: result-langpick.md step 0: `python3 -I C/langpick-tools/build.py` rebuilds all batches from current live; then `sh C/langpick-tools/land.sh apply` (core, copies, imports, dashboard, routing, launcher, launcher2).
- isolate: result-isolate.md: helper first, fence last; drift by `sh C/isolate-tools/rebuild.sh <batch>`.
- nxdeadline: result-nxdeadline.md: `sh C/nxdeadline-tools/rebuild.sh` only on drift (none now; the batch was rebased 21:01).
- benchci: result-benchci.md: `sh C/benchci-tools/rebase.sh` only on drift (none now).
- cbudget/shard: `python3 -I C/cbudget-tools/staged/shard/rebuild.py` on drift (none now).
- incfacts: result-incfacts.md: `make_batch.py` re-cuts from anchors; ONE-TIME re-inference of every project on landing.
- audit11/reapi: no rebuild, use instead of nolegacy/reapi (verify with `VERIFY_ROOT=<mirror> sh C/audit11-tools/staged/reapi/verify.sh`, timeout flag included).

## 6. Recommended apply order

Re-run `python3 -I <batch>/stage.py check --root /Users/ueli/Documents/semio` before each group. Each group ends with its verify.sh. Batches within one group are file-disjoint unless noted.

- Group 0, no action: edit30/freshmtime and edit30/prodclock are live. Record them as done.
- Group A, independent, land first: `audit11/reapi` (instead of `nolegacy/reapi`), `benchci/acceptance`, `nxdeadline/release`, `edit30/nxbuild`, `cbudget/shard` (optional; clock is half-landed, so decide clock first). None of these needs a rebuild today. benchci must land before the nolegacy/lease rebuild, because they share README.md.
- Group B, compliance: `sh C/compliance-tools/rebuild_all.sh`, then `sh C/compliance-tools/apply_all.sh /Users/ueli/Documents/semio apply` (fwprocess, fwmodules, libother, libtools, libbuild, libgraph, lang, names). The 8 compliance batches are file-disjoint among themselves. Fix the 3 drifted rows first (section 3).
- Group C, isolate helper: after B, `sh C/isolate-tools/rebuild.sh helper`, then apply helper. Then `builda`, `misca` (rebuild.sh first), `budget`, `buildb`, `cachinga`, `cachingb`, `enforce`, `miscb`, `tools` in any order. `fence` last, after `rebuild.sh fence`.
- Group D, nolegacy: after B, `python3 -I C/nolegacy-tools/rebuild.py`. Apply `lease`, then `lang`, then `nobaseline` in the same pass. Never `reapi` (superseded). Never `nobaseline` without `lang`.
- Group E, schemafirst: after B and D, `sh C/schemafirst-tools/rebuild_all.sh`. Apply in `C/schemafirst-tools/order.txt` order: law, revision, observation, generations, graphcore, memo, arguments, cachegc, hostcap, bazelsessions, bazelscript, readoracle, buildmisc, inference, storelimits. `memo` must land before audit11 warmderive.
- Group F, edit30 and audit11 derive: witpub rebased with `edit.py` after B and E, then apply. `audit11/warmderive` rebuilt with `rebuild.py <fresh mirror>` after B and E, then apply (it replaces `edit30/warmderive`, which is not applied).
- Group G, langpick: after B and E, `python3 -I C/langpick-tools/build.py`, then `sh C/langpick-tools/land.sh apply`. Core gives one graph re-inference; the lane says so.
- Group H, incfacts: last. `make_batch.py` re-cut (its 9 rows share 4 paths, 8 pairs, with compliance, libgraph, schemafirst and edit30/audit11; all 8 pairs merge clean with diff3). It causes the one-time project re-inference, so it goes after all product-changing groups.

Group ordering dependencies, in short: A before D (benchci/README), B before C (libbuild support file), B and D before E (lang/libgraph/lease overlaps), B and E before F and G, all before H. Only the B-to-C and D-to-nobaseline links are hard. The rest are rebuilds the lane scripts already do.

## 7. Limits

- check confirms base state per row. It does not run laws, verify.sh, mirrors, or product proofs. The diff3 test is textual only.
- Not checked: `relief.py`, `docs_*` helpers, the rebuild scripts' output (none was run), and the verify.sh of any batch.
- Whole-file removal review is by sample (first lines of each removed block) plus a live `git grep` for the moved, deleted and renamed paths. No full semantic check.
- Not in scope and not audited: clock (except its drift state), germanqa (7 batches queued for G48), audit13 batches (quiet16, multiwriter, claimsctl, factview, nestedfix, lostbirth), and the G47 landings.
- Mirrors: audit11's warmderive rebuild.py and the lanes' rebuild.py need a fresh mirror; none was created here.
