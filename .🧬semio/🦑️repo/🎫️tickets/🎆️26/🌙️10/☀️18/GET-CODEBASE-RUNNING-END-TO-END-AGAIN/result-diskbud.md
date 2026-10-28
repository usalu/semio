# result-diskbud (wave 1027b, lane diskbud): steady-state disk under a declared budget, lock-aware GC

`C` = this ticket. Tools `C/diskbud-tools/` (`sample.py` size sampler, `written.py` bytes-written-by-age walker, `make.py` batch builder, `src/` sources, `staged/diskbud/`, `mirror-gc.py`, `tests/`), samples `C/🗑️generated/diskbud/[DEBUG] samples.jsonl`. Mirror `dbud` was the only one and is dropped (re-create with `mirror.py create dbud` to re-verify). Everything measured read-only and streaming; nothing deleted, nothing applied live, no official run. Not closed: the ticket.

## FOR coordinator
- batch ready `C/diskbud-tools/staged/diskbud` (10 files: 8 new + 2 edits, flat `files/` + `base/` + `manifest.json`, `stage.py` keeps modes (template of xplat5), base = the live files of 12:10, `stage.py check` = all `base`).
  - apply `python3 -I C/diskbud-tools/staged/diskbud/stage.py apply` · revert `... stage.py revert` · laws `sh C/diskbud-tools/staged/diskbud/verify.sh <root>` (the 2 new suites plus the memo, memo-slice, memo-carry, journal lock/directory/world suites and the suite-membership law; GREEN on mirror `dbud`) · `... verify.sh <root> before` = base of the two edited files restored, only the new laws run, they are RED (memo 1 fail = byte budget not enforced, journal 1 fail = no sweep at start), the batch is applied again afterwards.
  - drift: `python3 -I C/diskbud-tools/make.py` re-derives both edits from the CURRENT live files by exact anchors (exit 2 when an anchor is gone, never guesses); `memo/🟦️.ts` is hot (memolist, memoout, printhot).
  - edits: `🧠️memo/🟦️.ts` (import, the two `collectGraphMemoV1(directory,8)` call sites lose the literal, the collect function is replaced by the planner-driven one) and `🏡️resident/📜️journal/🟦️.ts` (one import, one line after `takeLock`). Independent of C1 (no shared file with jobsrv, disk-unitcache, disk-rootguard, disk-sweep, disk-ledger, dashlock, sweep2).
  - product proof: after landing, `course:build` hot twice must still say `plan reused`/served from the memo (the collect function runs in every sealing child); `ls ~/Library/Caches/semio/repo/*/journal/graph-memo | wc -l` stays <= 8 records + their files; no sweep line appears unless a dead worktree state exists (today none does).
- NOT staged, user decisions (section 5): `.🧬semio/🌐hub` 100 GiB, rust `cs/v` 3.9 GiB funded child, pre-jdir journal dirs 139 MiB, stale `components/runs` 0.58 GiB, mirror retention (tool `mirror-gc.py` is read-only until `--apply --prefix`).
- FOR disk-ledger/unit-cache owners: the declared in-image budgets of the rust image overcommit its own watermarks (section 4, finding F1); decide before landing C1 `ledger`.

## 1. Measurements (11:20-12:15, host 926 GiB, 155 GiB free = 83 % used; APFS container 827 GB used / 167 GB unallocated)

Physical vs apparent: the live images are DENSE (`du` of `fixed.dmg` = capacity: the high-water mark of everything ever written, APFS inside never trims), while every tree measured inside an image or inside a mirror with `du` counts clones in full (areas sum to 37.8 GiB apparent on 16.1 GiB physical in the products image; a mirror is 1.77 GiB apparent and `mirror.py create` measures a physical free-space delta of 41 MiB). Physical rows use `df` of the mount, apparent rows `du -sk`.

Idle sample pair (33 min, no build running): products +0.04 GiB/h, tools +0.06, rust +0.03, graph -0.03, dependencies/diagnostics/leases 0, journal +32 MiB/h (one run), tickets +55 MiB/h, host free -0.67 GiB over 33 min (peers). Under load the growth is bursty; rows below give the observed churn (bytes with mtime in the last 3/24/72 h, `written.py`) and, for the rust image, the 71 h fill series (`u/fill.jsonl`, 5 703 samples).

## 2. Per-area table

Budget column = what is declared TODAY (live ledger/policy) -> what is staged (C1 `disk-ledger`/`disk-unitcache`/`disk-sweep` or this batch). GC owner column = who actually removes bytes today -> after C1 -> after `diskbud`.

| Area | Size now (physical / apparent) | Growth | Budget | GC owner |
|---|---|---|---|---|
| **Fixed images (dense)** | p 24 + rust 48 + t 8 + d 8 + g 6 + o 2 + n 1.5 + l 0.06 = **97.6 GiB** physical, constant | none (fixed) | partitions in budget policy (97.56 GiB) -> machine default 121.8 GiB (C1 ledger) | n/a, images shrink only when recreated |
| rust image `⚡️cache/rust/fixed` | 30.8 of 47.8 GiB used (64 %, 33.8 apparent) | gross 10.9 GiB per active hour (mean over 56 active h, p90 24.8, max 47.3); 111 drops > 1 GiB in 71 h; peaks 45.8 GiB (95.8 %), 2.9 % of samples above the 41.8 GiB high-water, **92.7 % above the 35.9 GiB low-water** | partition 48 GiB; nested `unit-cache` 24 GiB | unit-cache watcher at 87.5 % -> + band maintenance every 10 min (C1 unitcache) |
| - `u/o` unit objects | 11.0 GiB | 6.7 GiB written/24 h, 2.3 GiB/3 h | share 25 % = 11.95 GiB | gc (casShare) |
| - `u/l` lanes (4 live) | 2.4 GiB (print lane 1.9, dashboard 0.4); sessions 0.04 | recreated: 2.6 GiB in 3 h; cargo2 `incremental` adds ~0.8 GiB per edited workspace crate per lane | share 20 % = 9.56 GiB, incrementalShare 10 % = 4.8 GiB | gc, incremental sets first (C1 + cargo2 reuse it) |
| - `u/a` admission memos | 2.4 GiB, 233 076 files, oldest 5 d | ~11 MiB/h (821 MiB / 72 h) | none -> 3 % = 1.43 GiB | none -> C1 `pruneAdmissionsV1` |
| - `nx/tmp` (node-compile-cache) | 1.6 GiB | 507 MiB / 24 h | `compile-cache-rust` 0.75 + `nx-tmp` 0.25 GiB (over by 0.6) | none -> C1 sweep (12 h) |
| - `cargo/prototype`, `cargo/home` | 3.2 + 1.7 GiB | 109 MiB / 24 h | `cargo-home` 3 GiB, rest = `cargo-store` remainder | cargo owner |
| - `b/p` compiler workspace | 5.0 GiB (debug 1.3, prototype 3.0, logs 0.13, provenance 0.19) | 45 MiB / 24 h | reservations 4 + 4 GiB | compiler owner |
| - `cs/v` funded child `compiler-state:prototype` | **3.9 GiB, dense image, not mounted since 10-25 21:26** | none | reservation 4 GiB, outside every ledger area | none (F2) |
| - `components/runs` | 0.58 GiB, 68 dirs, newest 10-24 15:43 | 0 | none | none (F3) |
| products image `.cache/b/p` | 16.1 of 24 GiB (67 %); apparent 37.8: `bazel/outputs` 20.0, `bazel/actions` 7.4, `bazel/workspaces` 7.7, `bazel/pool` 1.0, vite 1.1, browser 1.0, tmp 0.3 | churn 595 MiB / 24 h, 14.9 GiB / 72 h (outputs), 3.5 GiB actions, 6.9 GiB workspaces | `bazel-actions` 40 %, `workspaces` 10 %, `outputs` 8 %, `pool` 6 % of the image (apparent counting) | bazel session collection on the free-space trigger (soft 3 GiB free); C1 sweep excludes owner areas |
| - Bazel output bases per slot | s0 6.1, s1 7.3, s2 6.0 GiB (apparent), s3-s5 0.25 / 0.24 / 0.00, s6-s10 0.0-0.12, d3 0.17; 16 slot dirs | +7 MiB/h idle, bursts with a product run | `bazel-outputs` 8 % | the same free-space trigger: an idle big slot stays until the image has < 3 GiB free (physical 67 % today, trigger at ~87 %) |
| tools `.cache/b/t` | 6.1 of 8 GiB (77 %); `nx` 32 GiB apparent (clones), `dashboard-cli` 0.85 GiB | dashboard-cli +334 MiB / 24 h, nx +185 MiB / 24 h | `nx-tooling` 2 GiB (4.4 held) | nxret-g/lease-g (idle-image rule), unchanged |
| dependencies `.cache/b/d` | 4.5 of 8 GiB | 0 | image | image |
| graph `.cache/b/g` | 2.0 of 6 GiB | snapshots +62 MiB / 24 h, generations +54 MiB | `graph-inference` 2 GiB, closures 1.3 GiB | graph collector (free-space trigger) |
| diagnostics `.cache/b/o` | 1.07 of 2 GiB (54 %) | **`o/tests` 939 MiB in 24 h** | `test-runs` 1 GiB | test-evidence owner; at this rate the image is full in ~1 day of heavy testing (F4) |
| nx pool `.cache/b/n` / leases `b/l` / control `b/c` | 1.5 GiB unmounted / 0.01 / 4 MiB | 0 | 8 MiB control (C1 ledger) | n/a |
| native loose `.cache/{bazel,command,impl-exp,semio,boot-recovery,test-sandbox}` | 0.27 GiB (bazel 182 MiB retired, command 57 (budget 128), impl-exp 37) | +5 MiB/h | root 256 MiB (over) | none -> C1 sweep (12 h) |
| repository loose `⚡️cache/*` outside the rust image | 4.7 GiB: `tools/elan` 2.8 (budget 3), `go` 0.74 (1), `cargo` 0.27, `🗃️bin` 0.1, undeclared `next-services` 70 MiB, `edfix` 22, `nx-admission` 17, `3f08…` 13, `e2b` 6, `p5` 4 | ~0 | root 24 GiB (inside) | 14-day undeclared rule, never started -> C1 sweep |
| **journal state** `~/Library/Caches/semio/repo/<id>/journal` (new, jdir) | 119 MiB (`graph-memo` 104: plans 1.8 / 6.4 / 6.8 / 38.4 MB, 20 files; logs 0.3) | memo 104 MiB in 45 min, then bounded by the record count; **logs 1 MiB segments, 16 kept = 16 MiB max**; under churn 6 MiB in 18 s | none (count 8 only; a record may hold 640 MiB) -> memo 1 GiB + abandoned-state sweep | memo: count only -> **diskbud** byte budget; dead worktrees: none -> **diskbud** |
| journal state OLD (pre-jdir) `$TMPDIR/semio-repo-501` + products `tmp/semio-repo-501` | 101 + 38 MiB, owners (pids 53030, 2500) dead | 0 | none | none (user decision 3; greenfield: no legacy sweep) |
| `~/Library/Caches/semio/{execution-targets,lease-registry,runtimes}` | 0.85 GiB (37 wasm modules, newest 09-29) / 16 MiB (3 687 dirs) / 59 MiB | 0 | none | none (F5) |
| mirrors `$TMPDIR/sm` (11 + `.nm`) | apparent 25-28 GiB (1.77-2.0 GiB each, `.nm` 3.1), **physical ~41 MiB per clone**, more with images (`dhm` 2.3 GiB: 6 sparse dmgs, one attached 48 GiB-capacity rust image) | -3.1 GiB/h apparent (peers dropping), creates +1.8 GiB each | none | `mirror.py drop` by hand -> `mirror-gc.py` (report, `--apply --prefix`) |
| tickets `.🧬semio/🦑️repo/🎫️tickets` | 3.5 GiB; this ticket's `🗑️generated` 618 MiB (`coord` 297, `nx` 119, `cwd-g` 50), tool dirs 0.3 | +55 MiB/h | rule: delete at close | manual by rule (no automatic delete by design) |
| **`.🧬semio/🌐hub`** (outside the ledger roots) | **100 GiB apparent, 258 dirs, newest 10-19** (`s15-rs1-overlay` 27, `s15-ex1-overlay` 27, `s15-ex1-scratch` 22, `s14-w4-catalog-*` 4 + 4 ...) | 0 since 10-19 | none | none (user decision 1) |
| other host consumers seen | `~/.cache/uv` 22.4 GiB, `~/.cache/r4`/`r5a` 10 GiB each, `Documents/semio.worktrees` 29 GiB, `semio-kinan-wip` 28 GiB (not this repo's cache) | - | - | - |

Ledger-visible total (images 97.6 + loose 4.7 + native 0.27) = **102.6 GiB against the staged machine default 121.8 GiB** (inside). Everything outside the ledger roots that this repository causes (hub 100 GiB apparent, mirrors, journals, tickets, `~/Library`, `$TMPDIR` rest) is unbudgeted: the hub dirs alone are as large as all images.

## 3. Does C1 cover what grew today? (jobsrv, disk-unitcache, disk-rootguard, disk-sweep, disk-ledger, dashlock, sweep2)

| Grew today | Covered by C1? | Verdict |
|---|---|---|
| unit admission memos `u/a` 2.4 GiB | yes, `disk-unitcache` (age 7 d, 3 % share, single-flight in every collection) | covered |
| rust band 75-87.5 % never maintained | yes, `disk-unitcache` (watcher maintains at start and every 10 min) | covered (measured: 92.7 % of 71 h above the low-water) |
| rustc incremental sessions (cargo2, +0.8 GiB per edited workspace crate and lane) | the existing collector removes incremental sets first, `incrementalShare` 10 % = 4.8 GiB, live lanes skipped by their markers; `disk-unitcache` makes it run in the band | covered, but see F1 (budget sum) |
| graph memo plans (6.4-38 MB each, 8 records) | **no**: count-only retention, a record may hold 640 MiB, no byte bound | **gap -> `diskbud` memo budget** |
| journal logs (~1 MB/min under churn) | already bounded: 1 MiB segments, 16 kept (16 MiB) | covered by the journal itself (checked: 16 segments of the old dir) |
| journal state of dead worktrees/mirrors (each id keeps up to 1 GiB of memo for ever; the old location is dead since jdir) | **no** | **gap -> `diskbud` abandoned sweep** (new location only) |
| Bazel output bases per slot | no (sweep excludes owner areas); free-space trigger only, so idle big slots live until ~87 % physical | not a gap in bytes (image capacity is the hard bound, 67 % now); reported, owner bazel |
| new keyed-command memos (envleak), rowid identities | no new on-disk store: the keyed command module is an environment policy, identities live inside the plan witness (a few bytes per row, inside the memo plan) | nothing to cover |
| `compile-cache-rust`, retired `.cache/bazel`, undeclared lane scratch | yes, `disk-sweep` (12 h, over-budget areas every 30 min) | covered |
| mirrors `$TMPDIR/sm`, tickets, hub, `~/Library/Caches/semio/{execution-targets,lease-registry}` | no (outside the ledger roots) | reported; mirror tool + user decisions |
| `cs/v` child, `components/runs`, `o/tests` 940 MiB/day | no | findings F2-F4 |

## 4. Findings

- **F1 rust image budgets overcommit its own watermarks.** Declared inside the image: `unit-cache` 24 + `cargo-home` 3 + `nx-artifacts` 3 + `compile-cache-rust` 0.75 + `nx-tmp` 0.25 = 31.0 GiB, plus the three funded reservations (compiler prototype 4, release 4, compiler-state 4 = 12 GiB) = **43.0 GiB, above the 41.8 GiB high-water (87.5 % of 47.8) and the 35.9 GiB low-water**. C1's own law only checks `u` shares (48 % of 48 = 23.0 <= 24). A state where every area sits at its budget therefore starts the collector and still cannot get below the low-water mark; the measured series shows exactly that (92.7 % above low). Two consistent fixes: raise the rust partition (48 -> 56 GiB; images are dense and fixed, so a recreate; the machine default 121.8 GiB has room: ledger-visible 102.6 GiB) or lower the budgets (unit-cache <= 16.9 GiB means shares 0.20 / 0.12 / 0.03 and fewer hits). Not staged: it changes cache-hit economics and conflicts with the C1 share law.
- **F2** `rust/fixed/m/cs/v`: an unmounted dense 4.2 GiB funded child (`repo:compiler-state:prototype`, allocation c3c14de1…, born 10-25 21:26, `hdiutil info` has no attach); 3.9 GiB used of the rust image, no consumer found, no retirement path in the ledger.
- **F3** `components/runs` 68 run dirs (`w*`), 0.58 GiB, newest 10-24: stale compile-run scratch, no area, no collector.
- **F4** `o/tests` 939 MiB in 24 h on a 2 GiB diagnostics image (1 GiB `test-runs` budget): the declared owner (`test-evidence`) must retire runs faster than ~1 GiB/day or the image fills inside a day of heavy testing.
- **F5** `~/Library/Caches/semio/execution-targets/components` 0.85 GiB of 37 wasm modules written 09-26..09-29, never read since; `lease-registry` 3 687 dirs (16 MiB). Static; no owner/collector; small.
- **F6** `.cache/b/p/m/tr-*`: 13 empty per-test-run directories (0 bytes, leak of names only).
- **F7** `mirror.py drop` renames a root into `.trash` and `rm -rf`s it while an image mounted inside may still be attached (observed for `sfm`: leases image attached below `sm/.trash/…` for the length of the delete, detached afterwards). A deleted-but-attached dmg keeps its blocks until the detach; `mirror-gc.py --apply` detaches first and skips busy ones.

## 5. User decisions (nothing below was done)

1. `.🧬semio/🌐hub` (100 GiB, 258 dirs, untouched since 10-19; the big ones `s15-rs1-overlay`, `s15-ex1-overlay`, `s15-ex1-scratch`, `s14-*`): fleet scratch of the hub project (also `db*-pg`, `*-neo4j` data dirs). Decide keep/delete per dir; a ledger root or a retention rule for it is only sensible once the hub owner says which dirs are state.
2. Delete `rust/fixed/m/cs` (3.9 GiB) if nobody uses the compiler-state child (F2) and `components/runs/*` older than 3 days (0.58 GiB, F3).
3. Delete the pre-jdir journal dirs: `$TMPDIR/semio-repo-501` (101 MiB) and `.cache/b/p/m/tmp/semio-repo-501` (38 MiB); their service pids 53030 and 2500 are dead.
4. Drop idle mirrors (`python3 -I C/diskbud-tools/mirror-gc.py --older-than-hours 6 --prefix <lane>-` shows what; `rb1` 5.3 h, `hpcj` 4.3 h are the oldest; each owner drops its own).

## 6. What `diskbud` stages

| Piece | Rule | Law |
|---|---|---|
| `🧠️memo/🧹️retention` (schema + module) | the newest 8 records stay (as before), then the oldest kept record goes while records + the files they name (each once) exceed **1 GiB** and more than 2 stay (the newest always stays); every file no kept record names goes after 60 s (as before); a byte-driven removal prints ONE EN + DE line on stderr (`[repo graph] the graph memo exceeded its 1024 MiB budget … / Das Graph-Memo überschritt sein Budget …`) | Python model (`🐍️.py`) == TS plan on 400 seeded states; newest never removed; infinite budget == former count rule; real directory collection with a 900 kB budget removes exactly the oldest slice and its files; format strings exact in both languages; budget >= largest plan + relevance |
| `📜️journal/🧹️abandoned` (schema + module) + one call after `takeLock` | at every service start: `<container>/<id>/journal` siblings (container `repo` or `semio-repo-<uid>`, id 12 hex) whose lock no service holds (the service's own `changeLockHeldV1` rule) and where nothing changed for **7 d** (**10 min** when the recorded worktree root is gone) are removed after taking their journal lock exclusively; a layout other than the cache layout (tests, custom directories) is never swept; the line goes to the service log as EN / DE (`[repo journal] removed N abandoned change journal state director(ies) (X MiB) … / N verwaiste Änderungsjournal-Zustandsverzeichnis(se) …`) | Python model on 500 seeded facts + both age boundaries; a real container with 7 kinds (old, young, root gone and quiet, root gone and recent, dead lock, young live lock, old live lock) removes exactly the model's set and leaves non-id names, id-only dirs, the own id and files alone; a custom directory is never swept; a starting service sweeps and logs both languages |
| `mirror-gc.py` (ticket tool, read-only by default) | report: age, apparent size, attached images, processes inside, droppable; `--apply` needs `--prefix`, detaches first, skips busy | `tests/test_mirror_gc.py` 2/2 |

Verification (mirror `dbud`, guard clean after every run): new laws GREEN (memo 6/0, journal 5/0); regression suites memo 8/0 (+1 skip), slice 2/0, carry 2/0, journal lock 3/0, directory 4/0, world 0/0 (1 skip), suite-membership law 7/0; `before` run RED as expected; tsc strict over the 6 TS files: 0 errors; stage modes kept (template). Longest staged path 114 chars.

## 7. Not done / open
- No live after-numbers by rule (no gc, nothing applied); the first measurement is `ls ~/Library/Caches/semio/repo/*/journal/graph-memo` and the next `sample.py` pair after landing (`python3 -I C/diskbud-tools/sample.py <out.jsonl>`).
- The memo byte line prints on the planner child's stderr (no other channel exists in that function); it appears only when the byte budget (not the count) removes a slice.
- Docs (build guide EN/DE "Disk budget") not edited: README/guide are hot; add a row per piece above when C1 `disk-ledger` has landed (FOR docs).
- Rust overcommit (F1), hub (decision 1) and the other findings need an owner; none can be fixed by a pure code batch.
