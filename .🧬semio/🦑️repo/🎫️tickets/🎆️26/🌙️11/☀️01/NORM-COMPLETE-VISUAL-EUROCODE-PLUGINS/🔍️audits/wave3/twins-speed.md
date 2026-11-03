# W3-TWINS-SPEED report

No cargo/rustc/`repo run`, no git writes, no queued job killed. Result: `schema twins generate/check` is NOT slow. The 30+ minutes are queue and lock time, and one of them is a self-deadlock in `heavy.sh`. No change to the twins tool was made (nothing to gain, see below); the real fix is staged and needs one `mv` by the coordinator.

## 1. What the tool does (read: `🧬️schema/🟦️.ts` `twins()`, `🧬️twin-projection/🟦️.ts`, `🧩️model/🟦️.ts`)
- One process, in-process, synchronous. `loadModules(--under)` walks only the `--under` folder (skips node_modules, target, tests, fixtures, packages), `JSON.parse` of each `🔣️.json`; `indexModules` is built from the same scope only (so `$ref` outside `--under` is "unresolved reference", a fast failure, see section 4). Each module renders its twins as strings; compare to the committed file; `generate` writes only changed ones. No project graph, no TS compiler, no child process per file, no whole-repo read. The only per-file cost is a few `readFileSync`. Scope-strictness and write-skipping (`readFileSync(target) !== text`) already exist.
- Startup is `repo` launcher + bun + the script router (~0.5 s).

## 2. Timings (measured now, `nice -n 19`, host load 15, swap full, direct `repo ./📜️script.ts schema twins check --strict --under <artifact>`)
| scope | modules (`🔣️.json`) | twins | wall |
|---|---|---|---|
| din18599, vdi3805 | 54 | 0 (no twin family) | 0.54-0.88 s |
| en1990 | 283 | (drift reported, evolve in flight) | 0.65 s |
| en1991 | 617 files / 304 modules | 921 identical | 0.87 s |
| en1993 (largest, 1280 `🔣️.json`) | 1280 | fails fast, see section 4 | 0.94 s |
| whole `📕️norm` (5713 `🔣️.json`) | 5713 | fails fast, see section 4 | 1.6 s |
Other post-apply steps of `evolve.py` (`post.py`): `schema-sync/check.py` 3.0 s, `handle-paths-check.py` <0.1 s, `check-cargo-authority.py` 0.1 s, `repo ./📜️script.ts verify build-owners sync` 28 s (the only noticeable one; run twice per apply: `owners_sync()` and again as a gate). Python `evolve.py` planning/verify is 4-19 s per plan (schema-tooling.md).
Before/after for the tool: identical (no code change), 0.5-1 s per artifact. A content-hash cache, a worker pool or a persistent process would save at most the sub-second render and cost correctness risk and memory; not implemented on purpose (the byte-identical requirement is trivially met, since the tool is untouched).

## 3. Where the 30+ minutes actually go
1. Self-deadlock (measured live). `post.py` runs `HEAVY repo ./📜️script.ts schema twins generate --under ...` for every twins/owners step; the agents also wrap the whole `evolve.py --apply` in `heavy.sh` (`en1990-wave3b-apply.sh`: `$H nice -n 19 python3 -I evolve.py add-leaf ... --apply`). The outer `heavy.sh` holds `heavy-slot.lock` (flock); the inner `heavy.sh` opens the same file again and `flock -w 3600` waits for its own parent: a deadlock for 3600 s, after which `post.py` aborts the apply (`check=True`) with the files written but no twins. Live right now: slot holder pid 3435601 (`evolve.py add-leaf en1990 testSeries specs/en1990-test-pairs.json --apply`, 1210 s old), its only child pid 3436258 is `flock -w 3600 heavy-slot.lock ... schema twins generate --under .../en1990` (1205 s old, parent = the holder). Every other waiting job (the ~19 in the queue, 8 of them `evolve.py ... --apply`, 7 `schema twins generate`) is blocked behind it; each queued `evolve.py --apply` will deadlock the same way when it gets the slot (60 min each), which is the whole serial queue.
2. Queue waits: the single flock slot plus the graph-plan wait (`heavy.sh` waits up to 45 min while any build is in graph-plan) and `build_active && low_mem`. The twins command itself is ~1 s of that.
3. `build-owners sync` 28 s x2 per apply (minor).

## 4. Note: a transient fast failure that looks like a hang
While `evolve.py` is mid-apply, `schema twins check` (scopes en1993 and the whole tree) stops with `unresolved reference https://json.schemas.assets.semio-tech.com/s/norm/en1993/mutation/change-plated-transverse-stiffener-inertia/schema.json` (`En1993Mutation` aggregate lists a mutation leaf schema that is not on disk: another agent's apply is in flight). It exits in 1 s with a stack trace and no `[schema twins]` summary line; a caller that greps the summary (law-sweep saw "exit 0 with only a wait line") should read the exit code.

## 5. Fix (staged, not applied)
`🗑️generated/build-runner/heavy.sh.new` (a copy of `heavy.sh` plus 9 lines): before the waits, `holds_slot` walks the PPid chain via `/proc/<pid>/status` and `/proc/<pid>/fd/*`; if an ancestor already has `heavy-slot.lock` open (i.e. holds the lease) the command is exec'd directly with `nice -n 15` (no wait, no second flock). It works for already-queued commands too (they were exec'd by the old script without an env marker), because it inspects the ancestor's fds, not the environment. `bash -n` OK. Replacing the shared runner script in place was denied by the permission classifier, so apply it with:
`mv .../build-runner/heavy.sh.new .../build-runner/heavy.sh` (atomic; running instances keep their open copy).
Regression test: `🧰️scripts/heavy-lease/test-reentrant.sh <heavy.sh>` (scratch runner dir, own lock, never the shared one): OLD `heavy.sh` -> nested call exit 124 (deadlock, killed by the 5 s timeout); `heavy.sh.new` -> nested call exit 0.
Before: nested heavy call 3600 s (flock timeout) then apply aborts. After: nested heavy call ~0 s (measured 0.03 s in scratch).
Not done (coordinator): the currently deadlocked holder (pid 3435601 / child 3436258) will only release by itself in ~40 min; killing child 3436258 releases the slot at once but makes that `evolve.py` abort after writing files (rerun it with the new `heavy.sh`: the plan is then empty except the twins/owners steps). I did not kill anything. Optional: drop the second `build-owners sync` gate in `post.py` (28 s per apply).
