# Result audit13 (read-only audit of 19 staged batches)

Lane audit13, wave 1027b. Live root `/Users/ueli/Documents/semio`. Nothing live was edited, no git command was run, no mirror was created, no product build ran.

Method: `python3 -I <batch>/stage.py check --root /Users/ueli/Documents/semio` for each batch (raw output in `🗑️generated/audit13/check-01.txt` to `check-19.txt`, in the order of the table). The 128 manifest entries were compared by repo path across all 19 manifests (exact key match). Overlapping pairs were tested with `diff3 -m` (live base vs both new files; output in `🗑️generated/audit13/merge-*.txt`). FOR coordinator lines come from each batch's result file.

## FOR coordinator

- Two batches are drifted and must not be applied until re-cut: `quiet16/hold` (1 drifted entry, needs multiwriter first) and `quiet16/lastrun` (1 drifted entry, stale fixture base).
- Three file overlaps need a rebase of whichever batch lands second: `claimsctl` vs `multiwriter` (2 conflict hunks), `multiwriter` vs `factview` (1 conflict hunk), `nestedfix` vs `nxdeadline` (clean merge, but the second one still drifts).
- Apply order is in section 3. Run the cache relief before `cachegc/quota`.

## 1. Batch table

| # | Batch | Entries | check on live | Overlaps (shared repo path) | Required order (from result file) | Risks flagged by result file |
|---|---|---|---|---|---|---|
| 1 | cachegc/quota | 11 | 11 base | none | `relief.py` dry run, then `--apply` (live store, not a repo file) before landing; after landing: `cache gc --dry-run`, then course and print runs | Relief removes 2 sparse test leaves (878,636,250 fake bytes) from the live store. `repo cache gc` crashes today (`Bounded captured Source channel required`). Gc before relief would evict real records first (LRU by write time). |
| 2 | e2e/claimsctl | 2 | 2 base | multiwriter (`generations/🧩️claims/🟨️.mjs`, 2 conflict hunks) | e2e step 1, "LIVE DEFECT, land first" | Any command that waits for a graph claim while holding an issued control dies at graph-plan (19:31 and 19:34 e2e runs). |
| 3 | e2e/petsrace | 2 | 2 base | none | e2e step 2 (focused proof: 17/80 failed on base, expect 0/80) | none |
| 4 | e2e/runfix | 4 | 4 base | none (4 files under the ticket folder) | e2e step 3 | none |
| 5 | e2e/layoutfresh | 1 | 1 base | none | e2e step 3b, after runfix | none |
| 6 | builderlock/multiwriter | 25 | 25 base | claimsctl (claims, conflict), factview (`generations/🟨️.mjs`, conflict), hold (`upgrade-before-custody/🟦️.ts`, hold base depends on it) | Land after claimsctl and factview (rebase with `make.py` on drift); land before hold | `generations/🟨️.mjs` at 64,359 B against the 65,536 B consumer cap. 13 new files. Product proof needs the official gate set run sequentially. |
| 7 | audit12/sigexit | 1 | 1 base | none | any; the mirror verify needs `--no-seal` | Hot file (`🏃️process/…/🏃️process/🟦️.ts`). Fixes a HIGH regression from progress `sigint`. |
| 8 | audit12/farmexports | 1 | 1 base | none | any | none; fixes an observation declaration-roster regression from farmid `authority` |
| 9 | audit12/typedecl | 3 | 3 base | none within this set | any; before the winlaunch, progress and rusttests lanes edit the same 3 files | Also carries the tsc fixes for those lanes (session `.d.mts`, door, suites). Those batches are not in this audit and were not checked against it. |
| 10 | audit12/saytest | 1 | 1 base | none | any | none |
| 11 | audit12/privparent | 1 | 1 base | none | any | The result describes a bug in optcache `store`, but the manifest only touches `🧪️tests/🕸️graph/🧰️support/🟦️.ts` (+4/-3). Confirm that no store change is missing. |
| 12 | obsopen/nestedfix | 6 | 6 base | nxdeadline (`🏪️store/🟦️.ts`, clean diff3 merge) | either order against nxdeadline; the second one is rebased | Graph implementation input (`🏪️store`): the next graph product run re-infers. Mirror verify takes about 14.5 min. |
| 13 | obsopen/lostbirth | 8 | 8 base | none | land next to another process-module batch (sigexit is one; no shared file) | Edits the settlement schema and the native storage schema (process-module implementation inputs): next graph product run re-infers. |
| 14 | nxdeadline/release | 7 | 7 base | nestedfix (`🏪️store/🟦️.ts`, clean diff3 merge) | after nestedfix (rebase with `rebuild.sh`); official proof after pinpub `pinned` lands | The result says it is independent of other batches. That is wrong for the store file. The law births 4 namespaces with fixed identities in the live nx partition. |
| 15 | farmid/declined | 3 | 3 base | none | any; its product proof follows cachegc | Root cause of the 142 "declined leaves" lines is the full optional graph cache (cache-gc lane), not fixed by this batch. |
| 16 | mem2/factview | 2 | 2 base | builderlock (`generations/🟨️.mjs`, 1 conflict hunk) | before multiwriter (its base is the tree without multiwriter) | none |
| 17 | incfacts/libfacts | 9 | 9 base | none | any; land it last among product-changing batches so the re-inference happens once | One-time re-inference of every project (`source-analyses` stamp 1 to 2). Hot files: `project-assembly`, `script-inputs`. |
| 18 | quiet16/hold | 24 | 23 base, 1 drifted: `🕸️graph/🧪️tests/🔼️upgrade-before-custody/🟦️.ts` | builderlock (hold base = multiwriter new, `bc91…`; live = multiwriter base, `fd11…`) | after multiwriter; before lastrun | Edits the Bazel executor and schema: every Bazel action re-executes once (cold build per product). `docs_hold.py apply` and `revert` run outside the manifest and are not covered by check. |
| 19 | quiet16/lastrun | 17 | 16 base, 1 drifted: `🌡️host-capacity/🧪️tests/🤫️quiet/🧫️fixtures/🔣️.json` | hold (same fixture path) | after hold, and only after re-cut | Its base for the fixture is `04fe…` (43,850 B). Live is absent, and hold's new file is `1aa3…` (42,519 B, same bytes as lastrun's new). The entry stays drifted after hold lands, so re-cut with `make.py lastrun` or drop the entry. `docs_lastrun.py` runs outside the manifest. |

## 2. Overlaps (exact repo path in two manifests)

Total: 128 entries across 19 manifests. Five shared paths, no others.

| Shared path | Batches | Base vs live | diff3 merge test | Consequence |
|---|---|---|---|---|
| `…/🕰️generations/🧩️claims/🟨️.mjs` | claimsctl, multiwriter | both base = live (`098a…`) | 2 conflict hunks | claimsctl first; multiwriter rebased with `make.py` |
| `…/🕰️generations/🟨️.mjs` | multiwriter, factview | both base = live (`6e7e…`) | 1 conflict hunk | factview first (its base excludes multiwriter); multiwriter rebased. Estimate: 63,108 + 1,251 (multiwriter) + 261 (factview) = 64,620 B, under 65,536 B. This is additive arithmetic, so re-measure after the merge. |
| `…/🏪️store/🟦️.ts` (library `⚡️caching`) | nestedfix, nxdeadline | both base = live (`5576…`) | clean (0 conflicts) | either first; the second is rebased and re-verified |
| `…/🧪️tests/🔼️upgrade-before-custody/🟦️.ts` | multiwriter, hold | live = multiwriter base (`fd11…`); hold base = multiwriter new (`bc91…`) | not applicable | multiwriter strictly before hold |
| `…/🌡️host-capacity/🧪️tests/🤫️quiet/🧫️fixtures/🔣️.json` | hold, lastrun | live absent; hold new `1aa3…`; lastrun base `04fe…`, new `1aa3…` | not applicable | hold before lastrun; lastrun base must be re-cut |

## 3. Proposed apply order (respects every dependency)

Each step is `python3 -I <batch>/stage.py apply` with the verify and proof lines from the batch's result file. Rebase any batch whose check reports drifted before applying it.

1. cachegc relief (`relief.py` dry run, then `--apply`). Live store only.
2. cachegc/quota.
3. e2e/claimsctl (live defect; must precede multiwriter).
4. mem2/factview (must precede multiwriter).
5. builderlock/multiwriter, after `make.py` re-cut against live that has claimsctl and factview.
6. quiet16/hold (its base now equals multiwriter's new).
7. quiet16/lastrun, after re-cut of the fixture entry.
8. e2e/petsrace.
9. e2e/runfix.
10. e2e/layoutfresh.
11. obsopen/nestedfix.
12. audit12/sigexit (process module, before lostbirth).
13. obsopen/lostbirth.
14. nxdeadline/release, after the nestedfix rebase; its official proof waits for pinpub `pinned`.
15. audit12/farmexports.
16. audit12/typedecl (before the winlaunch, progress and rusttests batches touch its files).
17. audit12/saytest.
18. audit12/privparent.
19. farmid/declined (its proof follows step 2).
20. incfacts/libfacts (last, so the one-time re-inference covers all earlier graph and process changes).

Steps 8 to 10 and 15 to 18 have no file overlap with anything else and can go in any order. Only the rebase points above are hard dependencies.

## 4. Limits

- Check only confirms base state per entry. It does not run laws, verify.sh, mirrors, or the product proofs, and it does not prove that combined edits are semantically correct. The diff3 test is textual.
- Not covered by check: `relief.py`, `docs_hold.py`, `docs_lastrun.py`, and the `make.py` re-cuts.
- Batches named by the result files but not in this set: winlaunch, progress, rusttests (targets of typedecl), pinpub (nxdeadline proof), farmid `captured` (also bumps `project-assembly`, like incfacts).
