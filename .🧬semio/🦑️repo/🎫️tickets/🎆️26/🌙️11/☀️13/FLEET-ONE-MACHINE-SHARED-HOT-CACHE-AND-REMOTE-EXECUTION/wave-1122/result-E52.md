# E52 stray caches (2026-11-22 14:15 to 14:45)

## Result
No `.cache` directory exists in the checkout any more except the declared root `.cache` (the ticket tree `.🧬semio/` and `node_modules` are out of scope by rule, as in the law). `repo test-file` of `🧾️ledger/🧪️tests/⚖️law/🌐️native` now passes 4/4 (was 3 pass / 1 fail "stray cache roots").

## Inventory (before; read-only find/du; 11 stray roots below `🧰️framework`)
| root (below 🧰️framework/) | size | last write |
|---|---|---|
| `.cache` | 8.07 GiB (3 sparse 8 GiB fixed.dmg images, tools, dependencies partitions) | 11-13 |
| `🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/.cache` | 6.06 GiB | 11-11 |
| `…/📚️library/📦️packages/🟦️typescript/.cache` | 2.06 GiB | 11-11 |
| `…/🏗️build/🧪️tests/🕸️graph/🧊️optional-cache/.cache` | 71 MiB | 11-11 |
| `🔨️modules/🏃️process/.cache`, `…/🔌️plugin/📇️registry/.cache`, `…/🛠️tools/.cache`, `…/🏗️native/.cache` | 4 MiB each | 11-11 |
| `…/📝️derived-config/🧪️tests/📝️derived-config/.cache` (empty `test-sandbox`) | 0 | 11-22 01:17 |
| `…/🛰️fleet-fetch/🧪️tests/📣️claim-wait/.cache` (`f/fetch-tmp`) | 0 | 11-19 |
| `…/⚡️cache-contracts/🧾️command-inputs/.cache` (empty `test-sandbox`) | 0 | 11-15 |
Nothing was mounted below any of them (`mount | grep semio/🧰️` = 0), `lsof +D` empty. Tickets hold 52 more `.cache` in other tickets' `🗑️generated` (test scratch `io-XXXXXX/.cache`, `file-record-image-*/.cache`) and 10 `.nx`: not touched (rule 3, and the law excludes `.🧬semio/`).

## Who created them
- The 8 big ones (Oct 22, Nov 11, Nov 13): native cache storage (partition images `b/<p>/fixed.dmg`, `b/c` sqlite, `boot-recovery`, `framework/leases`) opened with a member directory as workspace by old code (a batch run on 11-11 04:26 to 04:27 over many members; `framework/leases` and `b/` layout are pre-partition). Current code does not recreate them: `repo cache status` and `repo test-file` started from `🧰️framework/🔨️modules/🏃️process/🏗️build` created no `.cache` (checked).
- Still live until today: (a) `fleet-fetch/🟦️.ts` staged downloads with `context.root??"."` (`.cache/f/fetch-tmp` and `.fleet-fetch-*.tmp` below the cwd when neither worktree root nor store is set); reproduced by the new test before the fix. (b) the `derived-config` test used `process.cwd()` as workspace and as output parent (`.cache/test-sandbox`, `.🧬semio/🦑️repo/⚡️cache/tests`), so a run with cwd in the test directory (the old rule 10 recipe) wrote there; the `command-inputs` one came from the same function.

## Changes
1. `⚡️caching/🚀️bootstrap/🛰️fleet-fetch/🟦️.ts`: staging in `os.tmpdir()` when neither root nor store (both places); new test `🧪️tests/🛖️staging/🟦️.ts` (cwd a temp dir, remote cache faked: nothing may appear in the cwd; failed before, passes after).
2. `⚡️caching/📦️artifacts/📝️derived-config/🧪️tests/📝️derived-config/🟦️.ts`: checkout found from `import.meta.dir`, output in a temp directory it removes.
3. Ledger (schema first): optional `strays` rule (`quietMs` 900000, `ignoredSegments` node_modules/.git, `excludedDirectories` `.🧬semio`, bilingual `reason`) in `🧾️ledger/🧬️schema`, `🔣️policy.json`, type `CacheStrayV1`, `cacheStrayNamesV1(ledger)` = single-segment root directory names (`.cache`, `.r`).
4. `🏗️build/🧹️cache-gc/🏃️runner/🟦️.ts`: `cacheStrayRootsV1` (git `ls-files --others --ignored --directory`, 300 s bound from the gc schema `strayScanMilliseconds`, cancellable, links never followed, workspace root itself never), `cacheMountPointsV1` (mount table: darwin `mount`, linux `/proc/self/mounts`, none on Windows), and `collectCacheV1` plans each stray root as a `retired` target with quiet custody (so SQLite locks and live queue tickets hold units, young units stay, the changed-since-scan re-check applies, an emptied root is `rmdir`ed) and skips any stray with a mount below it ("Keeping ... mount:<point>"). Selector: plain `repo cache gc` and `--area=retired`. Injectable ports `strays`, `mountPoints`. Strays are not recorded in the size ledger. Status does not walk for strays (it must not walk trees): only `gc` and `gc --dry-run` show them.
5. Tests `🧹️cache-gc/🧪️tests/🥀️strays/🟦️.ts` (5): rule schema closed; git discovery equals an independent `readdir` walk on a real temporary git repo (root `.cache`, node_modules, excluded dir, symlink named `.cache`, empty root); dry run removes nothing; real run removes quiet unheld units, empties and removes the root, keeps a SQLite-locked unit, a mounted root, a young unit, `--area=t1` leaves strays alone; the declared ledger carries the rule.

## Cleanup executed
`repo cache gc --dry-run --area=retired`: 11 targets, 17 entries, 16.28 GiB, 0 kept. `repo cache gc --area=retired`: "Removed 17 entries, 16.28 GiB freed, 0 kept". `df -k` available on the Data volume 47,751,204 KB before and 47,402,856 KB after (other writers consumed the gain meanwhile and APFS frees lazily; the logical 16.28 GiB is the gc figure). A fresh `find` finds no `.cache` below the checkout besides the root.

## Tests run (all via `repo test-file`)
ledger 8 files 59 pass (law/🌐️native 4/4 after cleanup), cache-gc 15 files all pass incl. strays 5, doctor probes 18, host-capacity disk-recovery 22, remote-cache collector 8, fleet-fetch 8 + claim-wait 4 + staging 1, derived-config 2. Landing recorded in `T/landings.md` ("Wave 1122 E52"). Scripts: `T/🧰️scripts/E52/`.

## Not done / not verified
- Other undeclared roots seen, not touched: `.venv` (580 MB root, 272 MB energy plugin, 2.6 MB styling), `__pycache__` (5), `.🧾️store` (15 per-package 4 KB stores), `.p6af50af4/p/.vite` and `.pbdb5915c`, 10 `.nx` fixtures in tickets. A python bytecode setting (`PYTHONDONTWRITEBYTECODE`) in the tool registry would stop `__pycache__`; not landed (no area lock taken for `tools`, pycache dates are only 11-11).
- The native-storage creator of the 11-11 batch was not identified in code (only shown not to reproduce from cwd-in-member for `repo cache status` and `repo test-file`); the strays rule is the safety net: any new one is removed by the next `repo cache gc`. A stray root is only seen by git when ignored: `*.[Cc]ache` in `.gitignore` covers `.cache`, `.r` is not matched by it (not ignored means git does not list it as ignored; none exists now).
- Windows/Linux paths of the mount probe not run (darwin only).
- Whether the scheduled idle gc includes strays (`repo cache gc` default does; the idle loop code was not read).
