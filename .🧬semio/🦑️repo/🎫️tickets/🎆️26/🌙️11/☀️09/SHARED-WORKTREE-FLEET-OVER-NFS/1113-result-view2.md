# Result: view convergence, daemon memory, latency window (2026-11-13) — LANDED

Landing: `1113-landings.md` (2026-11-13T20:03Z, 11 files). Scripts: `🧰️scripts/1113-view2/` (`land.py`, `runtests.sh`, `conv2.py`, `profile-*.ts`, `fold-equal2.ts`, `refuse-equal.ts`). The previous agent's scratch copies were gone (scratchpad wiped); its design notes in the contract matched, the code was rebuilt and verified here.

## 1. Working-tree view converges in seconds

Cause: the full `git status` (25–35 s on 273 000 files) ran at 5 % duty, so after one pass the next was due 10 minutes later (`nextDelayV1(120 s, cost, 5 %)`), and the pre-existing `stale` flag only covered a restart seed (and then even exempted the view from the comparison), so a changed tree was reported `diverged` until that slow pass.

Built:
- `gitViewDeltaV1` (git): re-classifies only the changed paths with `git status` under literal pathspecs; after a stage or HEAD move also the paths that differ between the two index trees (`diff-tree`) and whose staged status changed (`diff --cached --name-status`). Refused (full pass) for a `.gitignore`, more than 20 000 paths, git failure. `GitViewV1` gains `stage`; `assembleViewV1` derives counts and digest.
- Replica notes the paths of every publish and apply (`viewDirt`, overflow beyond 4000); the attester drains them: delta after 1 s quiet (10 % duty), complete pass for overflow (>1500 paths), `.gitignore`, failure (25 % duty, 5 s quiet), complete verification only after other activity at 5 % duty as before.
- Attested view is `stale: true` while changes are known and not reflected (pending paths, moved stage, pass in flight, restart seed); a stale origin is `lagging`, not settled and not compared, so `fleet --wait` waits and never reports `diverged` for a merely unrefreshed view.

| Measure (this Mac, 273 000 files) | Before | After |
|---|---|---|
| 158 untracked paths deleted -> view reflects it | up to 11–12 min | 20 files created: 2.5 s; deleted: 2.5 s (live, `conv2.py`) |
| one changed path | 25–35 s full pass | 0.26 s |
| 10 / 60 / 260 paths | 25–35 s | 0.36 / 0.84 / 2.9 s |
| stage change only | full pass | 55 ms |
| delta result vs full pass | - | identical digest in all checks (real repo, and a 13-step real-git sequence incl. commit and mixed reset) |

Tests: git +1 (equivalence after each kind of change), attest +1 (scheduling, stale, overflow, restage, verification) and the stale verdict test inverted (lagging), replica `🌿️git.ts` +1 (three real worktrees, 40 files created and deleted, no `diverged` verdict observed, converged in about 4 s with verification set to one hour).

## 2. Daemon memory

Method: scratch process, COPY of `state.ckp`+`state.wal` (`profile-mem.ts`), real git calls read-only against the live tree. Open state: 393 MiB RSS (171 MiB heap). Transient peaks, not state, were the problem:

| Step | Before (RSS peak) | After |
|---|---|---|
| open + one full view + tracked list + one guards pass | 1906 MiB | view, guards, view, guards: 1076 MiB plateau |
| guards alone (without state) | ~1.0 GB over base | 752 MiB total |

Culprits (measured per step): `refusePathV1` allocated per path part (`split`, `toUpperCase`, regex), +667 MiB for 273 000 paths, now an index scan with a first-letter guard (+105 MiB, 3x faster); `foldNameV1` built strings character by character (+545 MiB), now a table-driven `replace` (+41 MiB, 3x faster); `spellingV1` kept every directory listing (+223 MiB), now drops listings outside the current path chain (+33 MiB); `trackedEntriesV1` kept a `Set` and 273 000 object ids (now neither). Equivalence: `foldNameV1` identical on all 1.1 M code points and 273 000 tracked paths (`fold-equal2.ts`), `refusePathV1` identical on 674 805 paths and fuzz cases (`refuse-equal.ts`); five new language-agnostic fixture rows, Python oracle 281 543 vectors 0 mismatches. Delta passes need none of the tracked-list work.
Live after restart: RSS 1.33 GB after 12 min (including a first 48 s full view under test load), before 1.96 to 2.4 GB after hours. Not yet observed over hours.

## 3. Publish latency outlier

The 491 421 ms was a leftover: `LatencyV1` kept the newest 100 samples forever, so one sample from a blocked period stayed in `p95` of a quiet machine. Samples now leave the report one hour after they were taken (`LatencyV1(…, retainMilliseconds, now)`, test added, README). Live after restart: publish p50 73 ms, p95 126 ms. The real slow path (a new file in a cold directory waits for the directory sweep, 5 to 6 s spacing at 10 % duty, median about 3 s) is unchanged: shortening it needs a higher duty or more CPU; the hint path and warm directories stay at 50 to 600 ms.

## Tests and live check
All 34 test files green on the live tree after landing (replica 94, attest 77, git 45, tiers 58, boundary 69, topology 65, journal 129+18 …); 20-seed chaos sweep 20/20 (on the copy). Daemon restarted onto code 35240fac3f31 within about 1 minute; `attest.json` advances (age under 2 s), `health.json` ok, `beat.json` fresh. `status`/README already name `.cache/f/boot/beat.json`.

## Not verified
Linux and Windows runs; behaviour over many hours (RSS plateau, verification passes); other machines' views (they run the old code until they restart, so their attestations do not yet carry the new `stale` semantics); the first view after a restart still takes 30 to 50 s (shown as stale/lagging meanwhile).
