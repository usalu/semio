# P1: end-to-end proof, numbers (2026-11-23, 01:08 to 02:20)

T = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/wave-1122-dist`. Scratch (stores, targets, logs): `/private/tmp/claude-501/-Users-ueli-Documents-semio/caf759d9-ccba-4796-9909-827125acd710/scratchpad/p1/` (A-root, B-root, C-root, D-root, `an.py`, `*.out`, `*.stamp`; first attempts under `old/`).

## Result

| Phase | Where / how | Units | Compiled local | Compiled remote | Local hits | Fleet hits | Misses | Build-script RUNs | Wall |
|---|---|---|---|---|---|---|---|---|---|
| A3 build | empty local store, fresh target, `pin:g1-m01` | 17 | 0 | 17 (all on g1-m01) | 0 | 0 | 17 | 4 run locally (declined: spawns-process), 4 published | 7 m 47 s (load 40-50) |
| B3 same tree, "other machine" | fresh EMPTY local store + fresh target, `pin:g1-m08` | 17 | 0 | 0 | 0 | 17 | 0 | 4 fleet hits (`script-hit`), 0 run | 35 s |
| C edit `p-leaf` (`sum * 31` to `* 37`) | fresh empty local store + fresh target, `pin:g1-m11` | 17 | 0 | 3 (p_leaf, p_mid, p_top on g1-m11) | 0 | 14 | 3 | 4 `script-hit` | 3 m 38 s (p_leaf waited 118 s in the worker queue) |

- 17 units = 13 crate units (unicode-ident, proc-macro2, quote, syn, serde_core, serde_derive, serde, itoa, smallvec, p-leaf, p-side, p-mid, p-top) + 4 `build_script_build` binaries; build-script RUNs counted separately. All keys independent of the target dir: B3 and C, with a different target and lane directory, hit what A3 published.
- A3 and B3 produced the byte-identical `p-top` binary (sha256 prefix 257fced9617c8fa7), output `42 1662 side-7`; C prints `42 1974 side-7`.
- C recompiled exactly p-leaf and its dependents p-mid and p-top; p-side, all 8 registry units, 4 build-script binaries and 4 RUNs were fleet hits. The dependents are recompiled because their `extern` key components changed.
- "Compiled" = lookup `miss`; where = `x/offload.jsonl` (`offload-run` rows carry the worker host); counts from `y/lookups.jsonl` and `x/offload.jsonl` of each empty root (`an.py <root>`). No `offload-local-fallback`, no declined rustc unit in A3, B3, C. 100 % of rustc work on a worker in A3 and C, 0 compiles anywhere in B3.

## Workspace and commands

- Proof workspace `T/proof/` (own `[workspace]`, 4 crates + serde 1.0.229 (derive), itoa 1.0.18, smallvec 1.16.3; 13 locked packages; longest path 162 chars; `Cargo.lock` handwritten from the root lock, offline). Files are checked-in, synced files; edit is the one `sed` in C.
- Driver `T/P1-run.sh <cacheRoot> <targetDir> <offload> build` (plus `P1-seal.ts`, `P1-retention.ts`): seals the CURRENT unit-cache code into `<cacheRoot>` (`ensureUnitCacheShimV1`, 7 s), then runs the pinned nightly cargo with the wrapper env of a live `repo cargo` build (copied from a live process: `SEMIO_UNIT_CACHE_*`, `SEMIO_REMOTE_CACHE_*`, `SEMIO_OFFLOAD`), `--release` with the repo's release profile selectors (`trim-paths="object"`) and `-Z trim-paths`. `SEMIO_UNIT_CACHE_TOOLCHAIN=p1-run3` (a key component, so this run's keys never collide with earlier runs; A3 is a genuine cold start).
  - A3 `P1_TOOLCHAIN=p1-run3 taskpolicy -b ./P1-run.sh $S/A-root $S/A-target pin:g1-m01 build`; B3 same with `B-root B-target pin:g1-m08`; C same with `C-root C-target pin:g1-m11`. Timestamps: A3 01:59:21-02:07:08, B3 02:07:17-02:07:52, C 02:08:06-02:11:53.
- Workers: Ready origins m01, m06, m08, m11, all code digest `74d4f0baf1…` (ready.json), 11 darwin-arm64 workers registered, all advertise `semio-pin`, `semio-sdk`, `semio-toolchain=312cc5b2932fbb31` (= sha256 prefix of `rustc -vV`). Pinned: X = g1-m01, Y = g1-m08, Z = g1-m11.
- Why not `repo cargo`: it ran, but both attempts (`generate-lockfile` and `build --offload=pin:g1-m01`, 12-14 min each, killed by me) never left `source.capture` (door `build:source-admission`), 0 % CPU, no lease held or awaited, with `repo show projects` holding the graph custody for 70+ min at 125/5220 owners. Not resolved, not my lane (report to the lane that owns door admission). Also `repo cargo` offers no way to start with an empty local unit cache (`SEMIO_UNIT_CACHE_ROOT` is always the store's `u`); the driver does. Local live `.cache/b/r/m/u` was never written by this proof (all roots under the scratchpad); the fleet cache (`:6094`, main instance) was written (about 21 new entries per distinct key set).

## Blockers found on the way (exact causes)

1. **OUT_DIR identity broke under `build-dir-new-layout` (fixed, tested).** First B run (B2, old code, identical A2 before): 13 fleet hits, 4 misses = serde_core, serde, p_mid, p_top. `components` diff A2 to B2: `env:OUT_DIR` for serde_core (`85474c79d343e23f` vs `6c4122eb9b3d5d64`), cascading through `extern:serde_core` to serde (and p_mid, p_top). Cause: `outDirectoryIdentityV1` (`unit-cache/🔏️key/🟦️.ts:182`) only recognised `build/<crate>-<16hex>/out`; the root `.cargo/config.toml` sets `build-dir-new-layout`, which gives `build/<crate>/<16hex>/out`, so the full lane-relative path with Cargo's unit hash entered the key, differing between targets and lanes, for every crate whose source reads OUT_DIR (serde, serde_core, ...). Fix: accept both layouts (regex alternation, 2 lines) plus 3 expectations in `🧪️tests/🌍️cross-root/🟦️.ts`; `repo test-file` cross-root 2 pass / 0 fail. Re-run A3/B3: 17/17 fleet hits. Landed in `T/landings.md` 02:20. Lock `lock.outdir-layout` taken and removed. NOT in the live play wrapper (the sealed generation 8126f99ff keeps the old regex; builds started after the next wrapper seal pick it up, one more re-key of OUT_DIR units); the pinned play clone `/Users/ueli/.sinc/play` has its own tree (unaffected, it still has the bug: any cross-lane/cross-machine reuse of OUT_DIR units there is lost, check its lookups for `env:OUT_DIR` component churn).
2. **SDK identity differs on the coordinator (open, not fixed).** `sdkIdentityV1` = basename of the realpath of `SDKROOT`. On the coordinator the Xcode SDK `MacOSX27.2.sdk` is a symlink to `MacOSX.sdk`, so identity `MacOSX.sdk`, while every worker advertises `semio-sdk=MacOSX27.2.sdk`. Effects: (a) every linked unit (bin, proc-macro, cdylib, build_script_build) offered to a worker fails with `pin-failed ... infrastructure 14 No other machine can take the action` (A1: 6 of 6 linked units local, 9 of 17 remote); (b) the identity is part of linked units' keys, so a coordinator-built bin/proc-macro never matches a worker-built one. Workaround used: `SDKROOT=/Library/Developer/CommandLineTools/SDKs/MacOSX27.2.sdk` (real directory, same SDK). Proper fix: derive the identity from `SDKSettings.json` (`Version`/`DisplayName`), not the directory name (area of O1/K1; lane locks free but not touched). Because the proof used the CLT path, the live play build (SDKROOT `.../Xcode.../MacOSX.sdk`) still cannot place linked units on workers and has coordinator-only keys for them.
3. **Slow wrapper probes under load.** `rustcVersionTextV1` times out after 20 s (`spawnSync`); under load 50-70 the first probe failed (`pin-failed g1-m01: toolchain-missing`, 2 units, 22 s and 31 s) before the memo (`offload-rustc.json`, TTL 10 min) was written. The driver pre-seeds the memo with the real `rustc -vV` text. Suggest a longer first-probe timeout or retrying once. A one-off `Content store unreachable ... 100 s` also forced one unit local (A1 itoa).
4. **Build-script RUNs are not remote.** 4 RUNs (proc-macro2, quote, serde_core, serde) are declined `not-portable build-script: spawns-process`; they run on the requester once (A3) and from then on come from the fleet (`script-hit` 4/4 in B3 and C). `build_script_build` binaries do offload (4 of 17 remote units).
5. `repo cargo`: the `SEMIO_OFFLOAD` environment variable is not forwarded (O1 already said: use `--offload=`); in this proof the wrapper env is set directly.

## History (for the record)

| Run | Change | Units | Remote | Local fallback | Fleet hit / miss in B-run |
|---|---|---|---|---|---|
| A1 (Xcode `MacOSX.sdk`, no remap flags) | first | 17 | 0 at first (`unmapped-path`: no `trim-paths`), then 9 after switching to the release selectors | 8 (6 linked, 1 probe timeout, 1 content-store) | not run |
| A2/B2 (CLT SDK, old key code) | SDK fixed | 17 | 17 (all on g1-m01), 2 m 03 s | 0 | 13 / 4 (OUT_DIR) |
| A3/B3 (fixed key code) | OUT_DIR fix | 17 | 17 | 0 | 17 / 0 |

## D. Retention dry-run (R1), scratch only

`P1-retention.ts` calls `retireStaleUnitsV1(root,{workspace:<checkout>,mode:"dry-run",graceMilliseconds:0})` of the R1 local mirror on scratch store copies (never on live; the live server still runs old code and was not touched). Entries carry `retention` declarations because the sealed wrapper includes R1 code.

- Store D-root = A3's entries (pre-edit) + C's entries (post-edit), judged against the edited tree: 24 scanned, 20 declared, 8 live, **stale: `edited` 1 (`e/42/420a3a92…` = pre-edit p_leaf, reason path `proof/p-leaf/src/lib.rs`), `extern` 2 (`e/fd/fdd9710a…` = pre-edit p_mid, `e/7a/7aebf961…` = pre-edit p_top)**, would remove 12 keys, removed 0 (dry-run, 0 bytes reclaimed). Exactly the edited crate plus its two dependents; the 4 new C entries, p-side and the registry units are not stale.
- 9 entries are `unreached (no live consumer)`: the proc-macro chain (proc_macro2, quote, syn, unicode_ident, serde_derive) and the 4 build_script_build binaries. They ARE live (C hit them). R1's edge resolver only follows plain rlib/rmeta/bin externs (R1 open item), so proc-macro and build-script edges are not seen. Under `enforce` this would evict live entries after the grace (12 keys of 24 here, 9 wrongly). Needs fixing before enforce is switched on: R1 owner.
- The live fleet store (`.cache/b/p/m/fleet/cache-server/v1`) was not read or modified; entries published by A3/B3/C exist there but carry declarations only for the units published by the sealed wrapper above.

## Reproduce

`an.py <root>` prints lookups (`local-hit|fleet-hit|miss` per crate) and offload events (`offload-run` origin, `offload-local-fallback` reason, `offload-declined`) of one root; the key component diff is `y/lookups.jsonl` `components` of the same crate in two roots (done for serde_core/serde: only `env:OUT_DIR` and the cascading `extern:serde_core` differ). Stores under the scratchpad are small (A 5.4 MB, C 5.3 MB); delete with the scratchpad.

## Not verified / open

- The live server counters (`repo remote-cache status`) before/after were not recorded (the status route needs the admin credential; `curl :6094/status` answers Unauthorized); evidence is the per-root logs and byte-identical outputs.
- A3 is 17 of 17 rustc units remote only with the coordinator-SDK workaround; with the stock Xcode SDK 6 linked units stay local (blocker 2).
- Not run on windows/linux workers (m04 linux and m05 windows not Ready).
