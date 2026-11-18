# R179: is a task built once per fleet? (2026-11-18, read 03:56 to 04:16 UTC)

Answer in one line: no. Three machines compiled the same Rust task (identical task key) for 8m42s, 22m13s and 23m36s, because the Nx single-flight claim of a task is capped at 120 s by the claim table (the front sends no `expectedMilliseconds`). Staged fix with a law that is red on the live code and green on the staged code: `🧰️scripts/R179/`. Nothing below `🧰️framework` was landed.

## 1. Facts (store `/Users/Shared/semio-store/v1/origins/*/ready*.json|txt`, cache server `.cache/f/cache-server/v1`, read-only)

Probe build `nx run-many --projects=norm-din4108,norm-din4108-rs,repo-mcp --targets=build` (7 tasks). Times UTC, start = end minus Nx "Run duration".

| machine | run start | run end | Run duration | Cache line | Rust task `norm-din4108-rs:build:prototype` | other tasks | outcome |
|---|---|---|---|---|---|---|---|
| g1-m08 | 03:39:52 | 04:04:35 | 24m43s | 0/7 hit | 23m36s | 7 s non-critical | ready 04:04:35 |
| g1-m07 | 03:43:38 | 04:06:22 | 22m44s | 0/7 hit | 22m13s | 7.6 s recoverable | ready 04:06:22 |
| g1-m03 | 03:49:08 | 03:58:30 | 9m22s | 0/7 hit | 8m42s | 7.9 s recoverable | ready 03:58:53 |
| g1-m10 | about 04:03:00 | 04:04:33 | 45.6s | 0/4 hit | not started (3 skipped) | `dsl-derive-rs:generate-schema-index` 45.6 s = whole run | FAILED `Original nested cache custody refused` at `norm-din4108:build:prototype`, then "Original process admission closed"; exit 1, backoff |
| g1-m01 | - | - | - | - | - | - | still in the graph read ("waiting for the fleet to infer s-dev-composition 1190 of 1800", then a new read at 04:15) |

Nx prints per-task seconds only for the critical path; the other six tasks have no per-task seconds in any log (see 6, instrumentation).

Was anything a hit after g1-m03 published (03:58)? g1-m08 and g1-m07 were mid-compile; g1-m10 ran its tasks at 04:03 to 04:04, after the cache held three entries, and still got 0/4. No later cycle exists yet, so no machine has been served a task from the fleet cache.

Cache server (`main/nx/`, entries are `{hash,size}` pointers into the host CAS):
- Nx task entries now: `7427260498132517366` (1.22 MB, stored 03:58 by g1-m03, the Rust task), `15698478355623763392` (1843 B, 03:49:55, slot `7692e8…`). A third (`15132650495004308041`, 1711 B, 03:43, no slot) was removed between 04:03 and 04:09 (slot supersession or collection; not examined).
- Task outputs ARE uploaded (three cacheable tasks per run; PUTs are accepted), the nx-graph is a separate publication (`ac` project-inference records).
- Evidence that the Rust task key is identical on three machines: the slot meta `7427…json` is rewritten at 04:04:30 (storedAt `1794974670715`, g1-m08's run end) and again at 04:06:02 (`1794974762386`, g1-m07's run end), while the entry file itself dates from 03:58. `publish` refreshes `storedAt` only for an existing key (claim "claimed" or PUT "exists"), so g1-m08 and g1-m07 registered the SAME key g1-m03 had stored. Same hash, same machine class (macOS arm64), three different machines: the key is machine independent for this task.
- Nx single-flight claims seen in `claims.jsonl` (journal starts 03:56): only g1-m10 claimed an Nx task, hash `15170646253472068348`, 04:03:32 to 04:04:19, renewed every 10 s (the 45.6 s task), then dropped. No claim exists for the Rust task at any time in 03:56 to 04:08 although three machines were compiling it. (g1-m05 `inst-…` is an install claim.)
- Different small-task hashes: g1-m10's claimed hash (`15170…`) is not any stored entry. Whether that is the same task as `15698…`/`15132…` cannot be shown from the store (no hash-to-task mapping is logged). Do not read it as proven drift.

## 2. Why the Rust task was computed three times (cause found, reproduced)

The front (`🧶️nx/🚦️single-flight`) claims a missed task hash with `ttlMilliseconds` 30 s and renews it every 10 s, but never sends `expectedMilliseconds`. The claim table's lifetime of a claim is `min(1800 s, max(120 s, ttl, 4 x expected))` (`🎟️claims/🟦️.ts` `lifetimeOf`, `minimumLifetimeMilliseconds` 120000). With no expectation the lifetime is 120 s: the next renewal after 120 s answers `lost`, the claim is dropped and cooled, and every machine that asks later claims it afresh and computes. Start gaps in the table above: 3m46s, 5m30s: both beyond 120 s, so no machine ever saw a held claim (no `[nx-flight] waiting` line in any log either).
Cargo units do not have this defect (`⚡️caching` callers pass `expectedMilliseconds`); the Nx front is the only claimer that does not.

Ruled out by measurement (task 2, hash inputs): the engine digest is independent of HOME, cwd and a foreign state directory (`🧮️engine-digest/🟨️.mjs`, generic `a5ac38a5…`, native `d936f2a6…`, three runs each with different HOME/`SEMIO_STATE_DIR`/cwd: identical); the `runtime` inputs `node -p platform+arch` and `bun --version` print `darwinarm64` and `1.4.2`, nothing else; `projectBuildTargetForNxV1` is a pure function of the target. No state root, HOME, host or toolchain path reaches the hash for these tasks. Hash-moving facts that do exist (see 7): the engine digest moves with every live edit of the keyed closure.

## 3. Cache switched off per task (why they run on every machine)

| task | authored | effective | reason |
|---|---|---|---|
| `ui-styling-tokens:generate` | `cache:false`, `outputs:[]` | not cached | writes generated sources into the workspace; a hit would restore nothing |
| `framework-schema:generate` | `cache:false`, `outputs:[]` | not cached | same |
| `dsl-derive-rs:generate-schema-index` | `cache:true`, `outputs:[]` | cached, 45.6 s on g1-m10 | latent defect: it writes `schema-index.json` under `🤖️generated/` (gitignored) from the whole workspace schema corpus, which is not among its inputs, and a hit restores nothing, so the file is missing on a machine that hit |
| `assets:build` | `cache:true`, outputs `README.md`, `🔣️icons/🤖️generated/*` | downgraded to `cache:false` by `nxOutputsCacheableV1` (`library/🟨️.mjs`) | outputs outside the rust/products cache partitions are "never restored by Nx" |
| `repo-mcp:build` | `cache:true`, output `.🧬semio/🦑️repo/⚡️cache/🗃️bin/{mode}/repo` | downgraded to `cache:false` | same: a workspace path |
| `norm-din4108:build`, `norm-din4108-rs:build` | `cache:true`, output `{projectRoot}/.🧾️store/build.json` | cached (receipt through the fleet content store) | admitted since package G landing 6 |

So of seven tasks only three are cacheable by design. The fix for the four is not "switch cache on" but a declared output kind that the fleet can restore: the receipt-through-fleet-content path (package G, `🧾️store-receipt/☁️fleet-content`) applied to the generated icon tree, the `repo` binary and the generated sources, each with a digest-verified restore. Not done here; the seconds of these tasks are small (non-critical path total 7 to 8 s on g1-m03), so the value is low next to the Rust task. The Go binary of `repo-mcp:build` is also covered by Go's own build cache.

## 4. Simultaneous execution: staged, red-first

`🧰️scripts/R179/` (README.txt has the apply recipe and live sha256 of the base files):
- `schema.patch.txt`: policy `expectedMilliseconds` 450000 (lifetime 1800 s = `maximumHoldMilliseconds`) and `waitBudgetMilliseconds` 600000 -> 1800000 (a waiter must outwait the longest observed task, 23m36s, as long as the holder's lease is alive; a dead holder is taken over after one lease of 30 s, not after the budget).
- `flight.patch.txt`: the claim body carries `expectedMilliseconds`; type `NxFlightTimingV1` gains the key.
- `long-holder.test.ts.txt` for `🧶️nx/🚦️single-flight/🧪️tests/🕰️long-holder/🟦️.ts`: real claim table, real claim routes, real front; only clock (time warp x200) and transport are modelled. Law 1: a 15 minute task started by machine A is awaited by B (minute 3) and C (minute 9): exactly one computes, B and C `waited:1 served:1 duplicated:0`. Law 2: the holder dies mid-task (transport down): exactly one of the two waiters takes over (`taken` sum 1, `duplicated` 0), the other is served.
- Result under the bun preload that substitutes staged files (transpiler cache off, no cache entry names scratch): base files RED 2 of 2 (computed `[m08, m07, m03]`, i.e. the production defect reproduced); staged files GREEN 2 of 2. Existing single-flight test file: 14 pass on base; 13 pass 1 fail on staged on the first try and passes on the second try: the failing assertion is `received.milliseconds > 800` of the native-client test (771 against a 900 ms sleep), a timing flake that also fails on base under load (load average 32 to 46), not the change.
- Not landed: non-test code below `🧰️framework` is staged only (rule). Needs a landing window; the test file lands together with the fix so no red test enters the tree. After landing: workers need no restart protocol beyond the usual (the front is started per Nx run).

## 5. Rust build inside the task: crate-level sharing

Exists (packages B, T, E, O of this ticket): the rustc wrapper's unit cache has a fleet tier (`⚡️caching/🦀️cargo/🗃️unit-cache/🛰️fleet`): one rustc invocation is the unit, key = toolchain + normalised args + source and extern digests + role-keyed remaps (machine paths are not in the key; portability proven by byte-identical rlib/rmeta from two checkouts with different CARGO_HOME), read-through/write-through to the fleet cache (`ac`), claims per unit with `expectedMilliseconds`, a 409 divergence adopts the fleet's bytes, offload under pressure. The claim journal shows `ac` claims from every worker in the window (g1-m03 212, g1-m10 245, g1-m07 123, g1-m01 46, g1-m08 38, g1-m05 105 including `semio`), but `ac` also carries graph-inference and owner-pool records, so the number of shared crates of the probe build cannot be read from it. It is plausible, NOT verified, that g1-m03's 8m42s against 22m13s and 23m36s is crate sharing (it started after g1-m07 and g1-m08 had published units), since the three machines are of the same class.
Why it matters more than the Nx task cache: the Nx task key contains the engine digest of the keyed closure (generic 903 files, native 1285 files) and moves with every tooling edit; the unit key does not.
Missing for crate-level sharing, smallest provable slice (plan only):
1. Per-run unit ledger line: at the end of the owned Nx invocation (next to `settleNxSlotsV1` in `⚡️caching/🚀️bootstrap/📜️script.ts`) print `[units] machine hit-local hit-remote miss stored skipped diverged` from `.cache/f/unit-counters` (the files exist: `localHit miss remoteHit stored skipped since`). Test: fake counter dir, two runs, delta printed. This yields the missing "units compiled vs fetched per machine" column.
2. Proof on the smallest case with the existing `pets-contract` build (12 units, 10 s): machine A builds, machine B (other root and HOME) builds with the fleet tier on: `remoteHit 12, miss 0`; this is package B's recorded test; run it again on two real workers once the lab is free and quote the ledger.
3. A cross-machine concurrent start of one unit is already single-flight (claims); the law "three machines, one unit, one compile" exists in package B's tests; add the long-unit case (unit longer than 120 s with `expectedMilliseconds` from its previous duration) to prevent the same cap defect there: unit claims pass `expected`, but check what value a never-seen unit passes (`defaultExpectedMilliseconds` of `☁️stage-fleet/🎟️flight`).

## 6. Instrumentation to settle the open question (hashes per machine), staged as plan

`settleNxSlotsV1` (`🧶️nx/🪢️slot-settle`) already reads the Nx task rows (`hash`, `project`, `target`, `configuration`, created/accessed) of the run. Print them as one language-neutral line per row (`[nx-task] <project>:<target> <hash> <stored|hit>`) from the call in `📜️script.ts`: then the ready logs in the store carry hash plus task per machine and the table above gets its per-task columns and the hash comparison. Not staged as code (it touches the hot bootstrap script; one landing together with section 4 is cheaper than two).

## 7. Findings for others

1. g1-m10 failed with `Original nested cache custody refused` at `norm-din4108:build:prototype` after its quick tasks passed (04:04:33, "Waiting for access to native-storage … position 2 of 2, behind process 95879" just before): R175's custody for the receipt output; it is the second machine to hit a custody refusal on this task shape. Not investigated here.
2. Someone edited keyed engine files live at 04:13:08 to 04:13:31 UTC (`🧰️framework/🔨️modules/🏃️process/🧰️tools/🔣️.json` is in BOTH the generic and the native closure; `📦️artifacts/🏗️native-build/💾️storage/…` in the native one): the digests moved (generic `a5ac38a5…` -> `96394796…`, native `d936f2a6…` -> `efbb8030…`). Every Nx task hash of every machine changed with it: the fleet entry `7427…` (the Rust task) can no longer be hit until a machine re-stores the new key. Landing rule 22:40/01:40 is being broken or a window opened; the coordinator should know. Every cycle after such an edit recompiles the Rust task from the Nx side (units still hit).
3. g1-m01 restarted its graph read at 04:15 ("100 s") after `code-stale`.
4. Heavy `repo` commands from a diagnostic agent start a whole-source admission: I ran `repo remote-cache status` once by mistake (14 s, stopped by pipe close); no state left.

## Unverified
Claim lapse is inferred from the claim table code, the start gaps and the red test, not observed (the journal starts 03:56, the claims were granted at 03:40 to 03:49). Per-task seconds of the six small tasks. Whether `15170…` equals the stored small-task hash. Crate sharing numbers of the real probe. The fix on real workers (needs a landing window and a second cycle).

## Update 04:35 UTC: landed, engine edit attribution
- Landed 04:34:01 to 04:34:18 under `landing.lock` (waited 236 s behind R180's landing; the lock `owner` file text was stale "claude-R177 03:30"): long-holder 2/0, single-flight 14/0, transpiler cache off, live shas checked equal to the base copies before the lock. Entry in `landings.md`. Machines that start an Nx run after 04:34 use the fix; the next cycles' "Cache: N/M hit" and any `[nx-flight] waiting/served` lines are NOT yet observed and must be added to the table.
- The 04:13 edits (mtimes): `🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧬️schema/🔣️.json` 04:13:08.6; `…/💾️storage/🍎️image/👤️owners/🟦️.ts` and `…/🍎️image/🛠️hdiutil/🟦️.ts` 04:13:17.0; `…/💾️storage/🧹️cleanup/🎛️command/🟦️.ts` 04:13:22.9; `🔨️modules/🏃️process/🧰️tools/🔣️.json` 04:13:31.9. Content is the native image storage (hdiutil, image owners, cleanup command): this is the "disk-recovery landing" the coordinator named, not R179/R177/R161 work (R161's last landing 03:30:45 touched `🧰️tools/🔎️resolve`, which is a different file). No landings.md entry exists for it yet; the only other edit of the generic closure since 03:31 is `🏃️process/🦀️cargo/⏱️budget/🟦️.ts` at 03:31:48 (also before the freeze lifted). I cannot name the agent from mtimes alone.
