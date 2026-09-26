# Audit S12 — Build + Rebuild Convergence and Zero-Touch

Auditor (Sonnet 5, read-only), session 12, 2026-09-25 23:0x–23:4x CEST. No edits, no builds/tests/servers run,
no git writes. Every timing figure below is either a literal line from `.🧬semio/🌐hub/w2-logs/*.txt` (the
`[w2-restage]` wrapper timestamps or Nx's own "Run duration"/"Cache" report), a literal line from
`.tmp-ticket/📓️wp-w2.md`'s timestamped log, or a `Cargo.toml`/`.cargo/config.toml` line with its own docstring
cited verbatim. Where a claim could not be independently re-measured this session (no builds allowed), it is
marked **UNVERIFIED**.

---

## 0. Summary

| # | Finding | Evidence | Verdict |
|---|---|---|---|
| S1 | One convergence cycle (describe→materialize→generate→check→activate→verify, 60 plugins) costs **~40 min when run back‑to‑back in one hold**, but the fleet took **13:23→18:56 (5 h 33 m) across 4 attempts** because peer edits and a lock queue landed between steps | `w2-logs/restage.txt` wrapper timestamps | Confirmed |
| S2 | Nx cache hit ratio is **0–3 % on every describe/materialize/check run measured this ticket** — no run got real incremental reuse | Every `w2-logs/*.txt` "Cache: N/M hit" line | Confirmed |
| S3 | `describe` and `component-dev` build the *literal same* cargo unit by design (so described bytes == shipped bytes), but only stay byte-identical if nothing in the shared dependency graph changes between the two invocations — with 8–11 concurrent slices editing the same untriaged tree, that window is routinely hours, not minutes | `🔌️plugin/🖨️describe/🏗️component-build/🟦️.ts:56-63` docstring + `pluginComponentRustcArgs`; restage3-verify (`consistent=0 diverged=60`, all `committed=shared≠dist=staged`) | Confirmed |
| S4 | A single global FIFO "wasm" fleet mutex (`.tmp-ticket/📜️fleet-mutex.sh`) serializes **every** wasm cargo invocation across **every** ticket slice — it exists because concurrent `cargo` under `fine-grain-locking` has deadlocked in this repo (`prebuild_lock_exclusive`, no rustc child, 59 min once) — so unrelated slices' turns land *inside* another slice's multi-hour chain and reintroduce drift | `wp-w2.md` 02:5x/13:3x entries, `work-packages.md` 142/173 | Confirmed |
| S5 | `wasm-dev` and `wasm-release` both force `codegen-units = 1` (single-threaded LLVM codegen per crate) — for `wasm-release` to work around an LLVM-22 linker crash and to maximize cross-crate dedup, for `wasm-dev` **only** so described/shipped bytes match. This is the dominant reason a from-scratch describe-all of 60 crates takes 31–47 min even with parallel Nx scheduling | `Cargo.toml:610-617,730-749` (cited comments) | Confirmed |
| S6 | Release (`component-release`) at 34 packages serially would cost **~15 min/package × 34 ≈ 8.5 h**; W2 landed a 3-parallel-per-hold batching fix that cut this materially, but publish still needed **4 attempts** (08:34→11:44) for three separate correctness bugs found only at full-catalog scale | `work-packages.md` 07:1x, `wp-w2.md` 08:34/10:39/11:11/11:44 | Confirmed |
| S7 | Non-build interruptions cost more wall-clock than the compiler did: **3 fleet-wide usage-limit freezes (~100 min each ≈ 5 h)**, **2 external low-disk SIGKILL sweeps that deleted already-built `generated/` output and a live hub data root**, and **1 desktop-app restart that killed every process fleet-wide (hub, holds, disk guard) at session end** | `work-packages.md` 144/166/174, `wp-w2.md` 08:34/12:19-12:35, `session-12-preamble.md` "Situation at 22:50" | Confirmed |
| S8 | Zero-touch has **never been proven end-to-end on any of the 4 required platforms in one timed run**. Closest: a Linux container proof green end-to-end with 3 generator-contract bugs (B1–B3) seeded and 1 Rust bug (B4, winit) patched **only in the container copy**, not landed at HEAD | `wp-z2.md` §1h/§2, `audit-s11-cross-platform.md` headline | Confirmed |

**Headline root cause:** this is not primarily a compiler-speed problem. The `.cargo/config.toml` design
(shared build-dir, fine-grain-locking, checksum-freshness) is explicitly built to let concurrent builds share
already-built units and rebuild only on real content change — but the fleet's own working model (AGENTS.md:
"work simultaneously with others on the same files," "MUST NOT use git worktrees") guarantees the shared tree
is edited continuously by 8–11 concurrent agents with no isolation. Two build-time correctness contracts
(byte-identical `describe`/`component-dev`, and Nx's own content-addressed task hash) both require "nothing
changed since the input was hashed" — a premise the fleet's collaboration model structurally violates on any
run longer than a few minutes. Every root fix below either shrinks the window in which that premise must hold,
or removes the requirement that it hold.

---

## 1. Measured cost table per chain step

Source: `.🧬semio/🌐hub/w2-logs/restage.txt` (`[w2-restage]` wrapper, wall-clock, authoritative) cross-checked
against each step's own Nx "Run duration"/"Cache" report in the matching `restage*-<step>.txt` file.

### 1.1 Staging chain (60 plugin guests), 4 attempts, 2026-09-25

| Attempt | Step | Start | End | Wall | Nx cache hit | Result |
|---|---|---|---|---|---|---|
| restage (1) | describe-all | 13:23:59 | 14:32:56 | **4137 s (68.9 m)** | 0/60 (0 %) | rc=0 |
| | generate | 14:32:56 | 14:33:32 | 36 s | 0/2 (0 %) | rc=0 |
| | check | 14:33:32 | 14:36:04 | 152 s | 0/1 (0 %) | rc=0 |
| | activate-s | 14:36:04 | 14:36:45 | 41 s | 1/14 (7 %) | rc=0 |
| | verify | 14:36:45 | 14:36:49 | 4 s | — | **rc=1**: `consistent=-12 diverged=72` (13 MISS `core.wasm`, 11 DIFF dist≠staged) |
| restage2 | materialize-all | 15:23:03 | 16:24:10 | **3667 s (61.1 m)** | 4/123 (3 %) | **rc=130**: demonstrator + vcs `component-dev` failed on transient build-dir file errors |
| restage3 | materialize-all (retry, queued behind WG7 16:24→17:11) | 16:25:30 | 18:04:34 | **5944 s (99.1 m)** | 1/127 (1 %) | rc=0, all 60 |
| | generate | 18:04:34 | 18:09:12 | 278 s | 0/2 (0 %) | rc=0 |
| | check | 18:09:12 | 18:13:04 | 232 s | 0/1 (0 %) | rc=0 |
| | activate-s | 18:13:04 | 18:15:25 | 141 s | 0/14 (0 %) | rc=0 |
| | verify | 18:15:25 | 18:15:34 | 9 s | — | **rc=1**: `consistent=0 diverged=60` — `committed=shared` (13:24–14:32 describe) ≠ `dist=staged` (16:25–18:04 materialize) for **every** component: peers edited shared crates in the ~2 h gap |
| restage4 | describe-all (re-run, immediately followed by materialize in the same hold) | 18:16:32 | 18:48:11 | **1899 s (31.6 m)** | 0/60 (0 %) | rc=0 |
| | materialize-all | 18:48:11 | 18:52:49 | **278 s (4.6 m)** | — | rc=0 (near‑instant: same tree as the describe that just ran, nothing moved in between) |
| | generate | 18:52:49 | 18:53:18 | 29 s | 0/2 (0 %) | rc=0 |
| | check | 18:53:18 | 18:55:23 | 125 s | 0/1 (0 %) | rc=0 |
| | activate-s | 18:55:23 | 18:56:00 | 37 s | 4/14 (29 %) | rc=0 |
| | verify | 18:56:00 | 18:56:05 | 5 s | — | **rc=0: `consistent=60 diverged=0`** |

**Reading this table:** the *only* attempt that converged on the first try was restage4 — the one run where
describe and materialize happened back-to-back inside one continuous hold (31.6 m + 4.6 m ≈ 36 m total for
both compile phases). Every attempt where a lock queue or a multi-hour gap separated describe from
materialize (restage→restage2→restage3) diverged and had to redo the describe step. **Total elapsed wall-clock
for this one staging convergence: 13:23:59→18:56:05 = 5 h 32 m across 4 attempts, vs. ~40 min for the chain
that actually converged.**

### 1.2 Release chain (`component-release`, 34 packages)

| Phase | Evidence | Cost |
|---|---|---|
| Naive serial cost (pre-fix) | `work-packages.md` 07:1x: "Release builds ~15 min/package serial (cgu=1, thin LTO)" | **~15 min × 34 ≈ 8.5 h** if run one at a time |
| Landed fix: 3-parallel inside one wasm-mutex hold | `wp-w2.md` 07:2x, `w2-release-par.sh` | batch B (5 packages: block, writer, draw, puzzle, wfc) **3342 s (55.7 m)** for 5 packages in one hold — ≈3× the naive per-package rate applied to 5 at once, not 5×15 min=75 min serial |
| Publish attempt 1 (catalog B) | `wp-w2.md` 08:34 | **FAILED**: gis closed actor 50 299 594 core bytes > 64 MiB bound (base64 embedding, 1.34× core) |
| Publish attempt 2 | `wp-w2.md` 10:39 | **FAILED**: block declares `kit.catalog` without owning its codec — `UNOWNED_ARTIFACT_CODEC_SCHEMA` not yet handled |
| Publish attempt 3 | `wp-w2.md` 11:11 | **FAILED**: gis answered 2 editor targets for one kind — hub creation-selection requires exactly 1 |
| Publish attempt 4 | `wp-w2.md` 11:44 | **SUCCESS**, 1774 s (29.6 m) — catalog B, 9 packages, 12 open targets, compressed actors 5.2–16.8 MB |
| Rest-warm (25 remaining packages) attempt 1 | `w2-logs/release-par-rest-1.txt`, "attempt=1 25 packages 2026-09-25 13:02:26" | cancelled at **19 m 7 s** (coordinator-directed stop for priority restage), 1/7 Nx cache hit |
| Earlier rest-warm attempts | `wp-w2.md` 12:16, 12:24 | **2× SIGKILLed (rc=137)** by an external low-disk sweep before even reaching cancellation |
| `--packages all` (all 34) | — | **Never completed in the visible log** (last state: catalog B live with 9/34 packages; rest-warm restarted a 3rd time at 13:02, then stopped for a priority restage at 13:23) |

### 1.3 Non-build interruption cost (same day, from `work-packages.md`)

| Event | Duration | Effect |
|---|---|---|
| Orphaned `dev mcp stdio os` process deadlocked against W2's describe in `prebuild_lock_exclusive` | 59 min, no rustc child | routed to G10 as a product defect (MCP server build outlives its client) |
| Network outage (API ENOTFOUND), all 11 executors cut | ~15 min (03:1x→03:26) | W2's detached chain survived (mutex held by a background process, not a live agent turn) |
| Usage-limit cut #1 | 03:40→05:20 (**100 min**) | fleet-wide freeze; W2's detached stage chain kept running unattended |
| Usage-limit cut #2 | 08:40→10:20 (**100 min**) | fleet-wide freeze |
| Usage-limit cut #3 | 13:4x→15:20 (**~100 min**) | fleet-wide freeze |
| External low-disk sweep #1 | 12:16, rc=137 | rest-warm SIGKILLed, no compiler error, disk 42 GiB free |
| External low-disk sweep #2 | 12:19–12:35 | **deleted `wp-w2/generated/` and `wp-w1/generated/` including hub 7800's live data root**; hub died, had to restart on a relocated data root outside the sweep's reach |
| Desktop app restart, session end | ~19:30 | **killed every fleet process**: hub 7800, all 10 slice hubs, all serves, all holds, the disk guard — nothing survived into session 12 |

Three usage-limit freezes alone account for **≈5 hours** of the ~18-hour session-11 day. Combined with the two
disk-sweep kills and the desktop restart, non-compiler interruptions plausibly cost as much wall-clock as the
compiler itself did.

---

## 2. Root causes of divergence

### 2.1 `describe` and `component-dev` are the same cargo unit, but only if run back-to-back (evidence: S1, S3)

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏗️component-build/🟦️.ts:56-58`:
```
/** @emoji 🧩 Builds the exact `component-dev` unit ({@link pluginComponentRustcArgs}) into the shared target
 * and returns cargo's uplifted output path, so the described bytes are the bytes `materialize-dev` ships. */
export function buildPluginComponent(...) {
  runCmd("cargo", ["rustc", ...pluginComponentRustcArgs(packageName, "wasm-dev")], ...);
```
and `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/🟦️.ts:11-16`:
```
/** 🧩️ The ONE cargo `rustc` argument list that links a plugin/extension component: `component-<profile>` and
 * the plugin's own `describe` both build exactly this unit, so the shared build-dir compiles it once and the
 * described bytes are the shipped bytes. */
export function pluginComponentRustcArgs(packageName, profile) {
  return ["-p", packageName, "--lib", "--crate-type", "cdylib", "--target", "wasm32-wasip2", "--profile", profile, ...];
}
```
This design is correct and, under `checksum-freshness = true` (`.cargo/config.toml`), should make the second
invocation a no-op if nothing changed. **Measured proof it works when nothing changes:** restage4's
`materialize-all` ran immediately after its own `describe-all` and finished in 278 s instead of another ~30
min — i.e. cargo recognized the freshly-built units and mostly skipped recompilation.

**Measured proof it fails when hours pass:** restage3's verify (`w2-logs/restage3-verify.txt`) shows, for all
60 components, `committed=shared` (from the 13:24–14:32 describe) differing from `dist=staged` (from the
16:25–18:04 materialize) — not because anything about the plugin itself changed, but because peers edited
shared framework crates in the ~2-hour gap between the two runs, which cargo's checksum-freshness correctly
detected as real input changes and rebuilt. The correctness contract ("described bytes == shipped bytes")
degrades into "…IF nothing else in the tree changed in between," and on this fleet something always does.

### 2.2 A single global "wasm" mutex forces multi-hour queueing, which is exactly the window that breaks 2.1

`.tmp-ticket/📜️fleet-mutex.sh` is one FIFO lock (`/tmp/semio-wasm-build.lock`) shared by name across **every**
slice's wasm cargo invocation, ticket-wide. It exists because naive concurrent cargo under
`fine-grain-locking` has deadlocked here before: `work-packages.md` 02:5x records a peer's orphaned
`cargo build --locked` sitting 59 minutes in `prebuild_lock_exclusive` with zero rustc children, blocking W2's
own describe. `work-packages.md` 173 records the inverse: the coordinator had to stop C10's zero-touch probe
because it had held the wasm mutex since 13:29 and W2's restage needed to go first. `wp-w2.md` 16:24 records
restage3's materialize queued behind WG7's turn for 47 minutes (16:24→17:11) — the queueing itself is small,
but it is exactly the kind of gap in which 2.1's premise breaks, because the mutex only serializes *builds*,
not *edits*: peers keep editing source files while they wait their turn, so every minute spent queued is a
minute the eventually-running build's "as of" snapshot drifts from what was described.

### 2.3 `codegen-units = 1` on both wasm profiles is the dominant single-crate cost (evidence: S5)

`Cargo.toml:608-617`:
```
# 🛡️ WASI component links alone select this mitigation for rust-lld's ElemSection crash.
# Native dev retains Cargo's parallel codegen policy; publication stays wasm-release.
[profile.wasm-dev]
inherits = "dev"
codegen-units = 1
# 🧾️ A component's described bytes must be its shipped bytes: `describe` and `component-dev` link the same
# `cargo rustc --crate-type cdylib` unit, and `incremental` is part of that unit's profile identity, so an
# inherited `incremental = true` split it by the caller's `CARGO_INCREMENTAL` into two compiles with two
# different wasm hashes (ticket 26/09/23 W1: descriptor `ce48…`/`2a5c…` vs staged `a740…`).
incremental = false
```
and `Cargo.toml:730-749` (`wasm-release`): `codegen-units = 1` again, justified there by both the same
ElemSection crash *and* cross-crate dedup for shipped size. `wasm-dev` pays the same single-threaded-codegen
cost for **zero size/dedup benefit** — it is there purely so `describe` and `component-dev` hash identically,
i.e. it is a second, independent mechanism defending the *same* correctness property 2.1 already targets via
"same cargo unit." `codegen-units = 1` is the primary reason a cold `describe-all` of 60 crates (many
single-purpose, but several large — GIS ~193 MiB unoptimized per the norm-package comment at `Cargo.toml:698`)
takes 31–47 minutes wall-clock even with Nx's own task-level parallelism (restage-describe 46m44s, restage4
31m17s, both at 0/60 Nx cache hit).

### 2.4 Nx's own cache never hits, because the fleet's collaboration model invalidates its content hash continuously

Every single Nx run captured across this ticket (`describe`, `generate`, `check`, `materialize`,
`component-release`) reports 0–3 % cache hit and Nx's own CLI recommends "share a cache across your team and
CI" in every one of them. There is no remote/shared Nx cache in this repo. With 8–11 concurrent agent
sessions editing the *same* checked-out tree with **no worktree isolation** (AGENTS.md: "You MUST NOT use git
worktrees" + "You MUST work simultaneously with others on the same files"), any task whose Nx input glob
touches shared framework code will see a different content hash on almost every invocation, because someone
is very likely mid-edit somewhere in that glob at any given moment. This is not a caching bug to fix in Nx
config — it is a direct, structural consequence of the fleet's own working rule, and it means **every slice
independently re-pays the full 30–60-minute cold-compile cost** for the same 60-plugin graph, even when their
own change touches one file.

### 2.5 Release-time correctness bugs surfaced only at full-catalog scale (one-time cost, not structural)

Three of the four publish failures (§1.2) — the actor byte-bound overrun from base64 embedding, the
unowned-codec-schema rejection, and the duplicate-open-target selection — were real product bugs that had
never been exercised because `--packages all` (34 packages) had never actually been run to completion before
(`wp-w2.md` §2: "No full catalog was ever published"). All three are now fixed at the source and covered by
laws (`wp-w2.md` §2.1, 10:2x/10:4x/11:2x). **This cost will not recur** on the next full-catalog rebuild unless
the fixes regress — it should not be counted against the *recurring* per-rebuild cost, only against the
one-time "first run at scale" bill this ticket already paid.

### 2.6 External interruptions compound everything above (evidence: S7)

Because 2.1's correctness window is measured in hours and 2.2's mutex queueing routinely spans tens of
minutes, any additional multi-hour interruption (a usage-limit freeze, an external low-disk sweep, a desktop
restart) does not just pause the clock — it guarantees the eventually-resumed build's premise ("nothing else
changed since I was described") is already false, because the fleet's other 7–10 slices kept editing during
the freeze. The clearest instance: session 12 opens with "Every process of session 11 died at ~19:30 (desktop
app restart) … Nothing holds `/tmp/semio-wasm-build.lock`" and catalog B — published live on hub 7800 at
11:44 — is now stale relative to the current tree and must be re-released from scratch before `--packages
all` can even resume (`session-12-preamble.md`, "Situation at 22:50"). None of restage4's 60/60-consistent
state or catalog B's data root survived the restart; there is no checkpoint/resume mechanism for build state
across a process-level fleet wipe, only for the wasm-mutex *lock file* itself (which the coordinator explicitly
re-verified was released cleanly).

---

## 3. Concrete root fixes, ranked by hours saved

### R1 — Make "describe immediately followed by materialize, same hold, no other slice's turn in between" the *only* way to run the chain (saves ~4–8 h per convergence cycle)

Evidence for the win: restage4 (back-to-back) converged in ~40 min; restage→restage2→restage3 (separated by a
lock queue and a multi-hour gap) took 5 h 32 m across 4 attempts for the identical outcome (§1.1).

**What to change:**
- `🔁️rebuild/🟦️.ts` (`RebuildAllScript`, the one AGENTS.md-compliant registered chain,
  `bun nx run @semio-tech/plugin-registry:rebuild-all`) currently shells each declared step out as its own
  independent `bun nx run …` process (`🔁️rebuild/🟦️.ts:48`, `runCmd("bun", [...step.command], …)`); it does
  **not** itself acquire the wasm fleet mutex, so nothing in the product code stops another slice's build from
  landing between its `descriptors`/`registry`/`guests` stages. Mutex discipline today is a *ticket-local
  convention* (`.tmp-ticket/📜️fleet-mutex.sh`), not something the canonical command enforces.
- Fix: have `RebuildAllScript` acquire one exclusive build-serialization lock (the same primitive the ticket's
  `fleet-mutex.sh` implements, moved into `🧰️framework/🛍️products/🦑️repo`'s own library so it stops being a
  ticket-folder script per AGENTS.md's "no runtime dependency on external libraries" + "close to each other"
  rules) for the *entire* `descriptors→registry→guests` span, not per-step. This guarantees the exact
  condition that made restage4 converge on the first try, every time, for anyone who runs the canonical
  command — not just when a slice happens to remember to run its own steps back-to-back under a hand-held
  mutex.
- Verify: run `bun nx run @semio-tech/plugin-registry:rebuild-all` end to end (currently explicitly **not**
  yet run end-to-end per `wp-w2.md` 05:32 — "The single command was not run end to end (it would duplicate
  this rebuild)") once no other slice needs the wasm mutex, and confirm the verify step reports `diverged=0`
  on the first attempt.
- Owner: **W2** (owns the chain and the `rebuild-all` command) with the fleet-mutex primitive relocated by
  whoever owns `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library` (infra-general, not plugin-specific).

### R2 — Stop killing already-converged build state; make the wasm-mutex lock and hub data roots survive a desktop restart or the disk guard (saves the 2 disk-sweep deletions + the session-12 full restart, each multi-hour)

Evidence: the 12:19–12:35 external sweep deleted `wp-w2/generated/` including the **live hub 7800 data root**
mid-run; the desktop-app restart at ~19:30 killed every hub, hold, and the disk guard fleet-wide, and catalog
B (published at 11:44) is now stale and must be re-released before `--packages all` can resume
(`session-12-preamble.md`).

**What to change:**
- The session-12 coordinator has already landed a better disk guard
  (`.tmp-ticket/…/📜️disk-guard.sh`: excludes actively-compiling crates via a live `ps | grep rustc` check,
  prunes only idle incremental (>60 min) and superseded units (>12 h) below explicit free-space thresholds)
  and a rule that durable data (hub data roots, catalog copies, hold state) must live under
  `.🧬semio/🌐hub/s12-<slice>-*/` outside every sweep's reach (`session-12-preamble.md` rule 8). This is the
  right shape of fix — **UNVERIFIED** whether it has yet been proven to survive a real low-disk event this
  session, since no such event has been observed post-fix in the logs read.
- Still open: nothing here survives a desktop-app-level restart, because the hub process, the hold, and the
  hold's data root are all child processes of the same desktop session. A durable catalog/hub-data root
  already exists as a *concept* (`.🧬semio/🌐hub/…`, gitignored, outside the ticket sweep) — the missing piece
  is a supervisor that can detect "the hub process is gone but its data root and catalog are still on disk"
  and restart the hub against the existing state instead of the next session re-deriving everything from a
  fresh `w2-restart-7800.sh` invocation. `w2-restart-7800.sh` already does the right thing *once told to run*
  (copies the catalog, provisions users, starts the hold) — the gap is that nothing runs it automatically on
  process death.
- Owner: **W2** for the hub-restart-on-death behavior; disk-guard hygiene is already coordinator-owned this
  session.

### R3 — Get a shared/remote build cache so slices stop independently re-paying the same 30–60-minute cold compile (saves the bulk of the 0–3 %-hit-ratio time, roughly proportional to slice count)

Evidence: every logged Nx run in this ticket is 0–3 % cache hit and Nx's own CLI output recommends exactly
this in every capture (§1.1, §1.2).

**What to change:** this repo already treats "one shared compiler cache" as a first-class design goal for
Cargo (`.cargo/config.toml`'s own header comment: "One shared compiler cache for every workspace, crate,
profile, target, agent and dev"). Nx has no equivalent today. Given AGENTS.md's "no new runtime dependency on
external libraries" and "use only system libraries," the fix that fits is **not** the SaaS Nx Cloud — it is
extending the same local-filesystem sharing model Cargo already uses to Nx's own cache directory, since every
slice in this fleet already shares one physical checkout: point every slice's Nx cache at the same shared
directory (as `build-dir`/`target-dir` already are) instead of each slice's ticket-local
`CARGO_TARGET_DIR=/…/wp-<slice>/target` pattern implying an isolated Nx cache too. **UNVERIFIED**: whether Nx's
task hashing would actually produce a shared, reusable hash across two slices editing unrelated files in the
same tree at different moments — worth a controlled two-slice experiment before committing to this as the fix,
since §2.4's structural argument (constant concurrent edits to shared glob paths) may still defeat it even with
a shared cache directory. If it does, the deeper fix is narrowing each target's Nx `inputs` so a plugin's own
`describe`/`component-dev` task hash does not transitively include every other plugin's and every framework
crate's source glob — currently a single peer edit anywhere invalidates the whole 60-project graph's cache
(0/60, 0/123, 0/127 hits across every run measured), which is disproportionate if most edits are scoped to one
crate.
- Owner: **flag to the coordinator / a NEW infra slice** — this is repo-wide Nx configuration, not
  plugin-registry-specific, and needs the two-slice experiment above before landing.

### R4 — Widen release parallelism beyond 3-at-a-time now that the batching primitive already exists (saves hours on the remaining 25-of-34-package rest-warm)

Evidence: the landed 3-parallel batching (`w2-release-par.sh`, `nx run-many -t component-release --parallel=3`)
already cut batch B's 5-package release to 3342 s instead of ~75 min serial equivalent (§1.2). Disk was
freed to 137 GiB by the coordinator mid-session, and load was measured at ~4 by session-12 start — headroom
for more parallelism was not the blocker; the FIFO wasm mutex means the *whole batch* runs inside one hold
regardless of `--parallel` value, so raising `--parallel` (e.g., to match available cores) costs nothing in
mutex contention and should shrink the 25-package rest-warm proportionally.
- Verify: rerun rest-warm with a higher `--parallel` and compare wall-clock to the 3-parallel batch-B baseline
  (3342 s / 5 packages ≈ 668 s/package effective) — **UNVERIFIED**, not tested this session (no builds
  allowed).
- Owner: **W2**.

### R5 — Revisit `codegen-units = 1` on `wasm-dev` once the LLVM ElemSection bug is fixed upstream (saves compile time on every future cold describe-all, but externally gated)

Evidence: `Cargo.toml:608-609` names the ElemSection crash as the reason `wasm-dev` needs `codegen-units = 1`
at all (the byte-identity requirement in §2.3 is the *second*, independent reason it stays there even after an
LLVM fix). This is a medium-term item: track the upstream Rust/LLVM issue and, once fixed, benchmark whether
removing `codegen-units = 1` from `wasm-dev` specifically (keeping it on `wasm-release` for dedup) shortens
cold `describe-all`/`materialize-all` wall-clock, since dev builds get no size benefit from CGU=1 today.
- **UNVERIFIED**: no evidence in this ticket of the current LLVM/rust-lld version or whether the ElemSection
  bug is already fixed in `nightly-2026-07-07` (`rust-toolchain.toml`) — worth checking before assuming this
  fix is still blocked.
- Owner: **W2** or a **NEW** toolchain-tracking slice.

---

## 4. Zero-touch status for a fresh clone

Sources: `.tmp-ticket/📓️audit-s11-cross-platform.md`, `.tmp-ticket/📓️wp-z2.md`.

| Platform | Status today | What breaks |
|---|---|---|
| **macOS (native)** | Only continuously-exercised platform; `signExecutableForDistribution`/`installExecutable` verified clean (no in-place Mach-O overwrite SIGKILL); no full *timed* zero-touch run from clean state captured anywhere in ticket history (audit-s11-build-health P0-3) | None found; **never timed end-to-end** |
| **Linux (native, non-container)** | Was **P0-broken** (unconditional `mold` linker requirement, apt-only install) — **LANDED+MEASURED fix**: every glibc Linux host now links with the pinned toolchain's own `rust-lld` (`.cargo/config.toml` `[target.'cfg(all(target_os="linux", target_env="gnu"))']`), measured 6.6 s vs mold 3.3 s vs GNU ld 37.8 s on the real `os-hub` binary — no distro dependency left | Blocked by 4 fresh-clone bugs found by Z2's own Linux-container run (B1–B4, below); no evidence of a run on **real** (non-container) Linux hardware |
| **Windows (native)** | 4 P0/P1 source fixes **LANDED** (long-path git config + guidance, PowerShell 5.1 BOM/encoding, destructive global cargo-config overwrite deleted, unguarded install line) — each backed by an oracle/law (`ps-lint.txt`: PSScriptAnalyzer + pwsh parser; path-budget law) | **Zero evidence of any execution on real Windows hardware anywhere in ticket history.** Unproven: the winget Node install chain, Bun/PowerShell handling of emoji paths, Git-for-Windows checkout of emoji+ZWJ names, MSVC with the shared build-dir (deepest measured build path is 246 UTF-16 units below the clone root, fitting only under a ≤13-unit clone root) |
| **Devcontainer** | Structurally intact and unregressed (`postCreateCommand`, `setup.dependsOn`, Rust-feature pinned off in favor of `rust-toolchain.toml`, `mold` no longer needed) | **ON HOLD**, not run this ticket: blocked by the same B1–B4 fresh-clone bugs (B4/winit stops `cargo check -p semio-hub` inside the container exactly as on native Linux) **and** a host-side blocker — Docker Desktop cannot bind-mount `~/Documents` on this Mac (macOS privacy permission), so a real run needs a clean copy outside `~/Documents`, not yet executed |

### B1–B4 (Z2's fresh-clone blockers, all found by an actual `ubuntu:24.04` container run, none Linux-specific except B4)

1. **B1** `assets:build` fails: `external-emoji-shortcodes` is declared an external generator input with no
   producer and `inclusion: ignored`, so it is simply absent on any fresh clone. Fix: track the pinned snapshot
   outside `🤖️generated`.
2. **B2** `framework-graph:generate` fails: the `wgpu-frame-worker` contract declares its generated output
   `inclusion: "tracked"`, but `.gitignore`'s `**/🤖️generated/` blanket-ignores it and it is genuinely not in
   git — the contract must say `"ignored"` and `prepare` must actually run the generator.
3. **B3** — same bug, different contract: `scale-fixture`'s output root is also `**/🧫️fixtures/**/🤖️generated/**`-ignored while declared tracked.
4. **B4** — Linux/devcontainer-specific: `winit` is pinned `default-features = false, features = ["rwh_06"]`
   with no Linux backend, so on any Linux (native or container) `winit`'s `compile_error!` stops every crate
   enabling `semio-framework-ui/wgpu-engine`, semio-hub included. A patch exists
   (`wp-z2/pending/winit-linux-backends.py`) and was **proven** in the container run (`cargo check -p semio-hub`
   green in 3 m 40 s, `cargo build --bin os-hub` green in 8 m 27 s after applying it) but is explicitly **gated
   on W2's Hub Handoff / `--packages all` publish** before landing at HEAD, per the ABI freeze in
   `session-12-preamble.md` rule 1.

B1–B3 are general zero-touch bugs (generator-contract vs. `.gitignore` mismatches) that would block a fresh
clone on **any** platform, not just Linux — Z2's audit happened to surface them via the Linux container run
because that was the first genuinely-fresh-clone execution anyone had done. **None of B1–B4 are landed at HEAD
as of this audit** (Z2 seeded/patched them only inside the disposable container copy to prove the rest of the
chain, explicitly deferring the real landing until after W2's publish to respect the ABI freeze).

**Bottom line:** zero-touch is not proven end-to-end on any platform as a single timed run at HEAD. The
Linux-container proof is the closest thing to a full run this ticket has, and it required 3 real bugs
worked around outside the tracked tree and 1 real bug patched in a disposable copy — none yet landed.

---

## 5. What I could not verify

- Could not run `bun nx run @semio-tech/plugin-registry:rebuild-all` end to end myself (no builds permitted;
  it also has never been run end-to-end by anyone per `wp-w2.md` 05:32) — R1's exact wall-clock saving is
  extrapolated from restage4's measured ~40-minute back-to-back chain, not independently re-measured.
- Could not confirm whether the session-12 disk guard (`📜️disk-guard.sh`) has actually been tested against a
  real low-disk event yet — no such event appears in the logs read this session.
- Could not confirm the current LLVM/rust-lld ElemSection bug status against `nightly-2026-07-07` (R5) — this
  would need either a changelog read or a build, neither done.
- Did not read `restage2-materialize.txt`'s or `restage3-materialize.txt`'s full multi-MB bodies (only tails
  and the `[w2-restage]` wrapper summary) — per-crate compile-time breakdowns within those runs are not
  captured here, only the aggregate wall-clock and Nx cache-hit ratio.
- `wp-r8.md` (the R8 gate-health slice) contains no material on the build/rebuild chain itself — it is cited
  here only for which downstream gates are blocked on W2's rebuild (§3/§4 owner mapping), consistent with the
  task's framing of R8 as "gates," not build infra.
- Did not independently verify Z2's Linux-container linker benchmark (§Zero-touch, `rust-lld` vs `mold` vs GNU
  ld) — reported as cited from `wp-z2/generated/linker-probe-*.txt`, not re-run.

Sources: `.🧬semio/🌐hub/w2-logs/{restage,restage2,restage3,restage4,release-par,release-par-rest-1}*.txt`;
`.tmp-ticket/📓️wp-w2.md`; `.tmp-ticket/📓️work-packages.md`; `.tmp-ticket/📓️audit-s11-build-health.md`;
`.tmp-ticket/📓️audit-s11-cross-platform.md`; `.tmp-ticket/📓️wp-z2.md`; `.tmp-ticket/📓️wp-r8.md`;
`.tmp-ticket/📜️fleet-mutex.sh`; `.🧬semio/🦑️repo/🎫️tickets/…/END-TO-END-OS-HUB-COLLABORATION-MCP/📜️disk-guard.sh`;
`.🧬semio/🦑️repo/🎫️tickets/…/END-TO-END-OS-HUB-COLLABORATION-MCP/📓️session-12-preamble.md`; `Cargo.toml`;
`.cargo/config.toml`; `rust-toolchain.toml`; `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏗️component-build/🟦️.ts`;
`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/🟦️.ts`;
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🔁️rebuild/🟦️.ts`.
