# R55: Rust unit cache and offload, duplicate work

Date: 2026-11-15 18:45 to 19:05 CET. Landed under `landing.lock` with `T/🧰️scripts/R55/land.py` (three phases: red, apply, verify; record in `T/landings.md`, section "Package R55 landing"). T = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION`.

## 1. Divergences: classification (read from `.cache/f/cache-server/fleet/divergences.json` and the entries in `v1/main/ac`)

| Class | Rows | Evidence |
|---|---|---|
| Graph owner records (`kind: project-inference`, document `record.br`) | 914 of 1000 (905 `main`, 9 `semio`) | the kept entry is present and its inline note says `project-inference` |
| Rust compile units | 0 shown | no kept entry carries a `unit.json` output |
| Kept entry gone (evicted or replaced), cannot be classified | 86 (82 `main`, 4 `semio`) | file absent in the server store |

- 1000 rows, 920 distinct keys (the map said 921), all dated 2026-11-15. 865 of the 1000 fall in 04:35 to 05:59 UTC (one burst), the other 135 are spread over 06:00 to 17:46 UTC.
- Pairs (kept to refused): g1-server to g1-m10 462, g1-m01 to g1-m10 147, g1-m07 to g1-m10 96, g1-server to g1-m03 87, g1-m01 to g1-m03 69, g1-m01 to g1-m05 36. 857 keys have one refusal, 63 have two to nine.
- So the map's premise ("the same Rust unit key was built on two origins") is not what the log shows. The divergence log is a log of graph records. The cap (1000 rows, newest kept) means older Rust rows could have been pushed out; the log cannot tell.
- Cause for the 914 records: the fingerprint compared is the digest set of the entry's documents (`record.br`, `closure-*`), so the record bytes really differ between two origins. The blobs of both sides are no longer in any store I can read (the server's blob store is the host CAS and the 04:35 blobs are evicted), so I could not diff a pair. Unverified reading of the code (`🪪️graph-state/📦️project-records/🟦️.ts`): the record "carries every observation it consumed" and a taker that cannot hold the first record "replaces it by its own" (`reinferred` counter); the server keeps the first writer, refuses the later one, so the stale first record stays and every other machine re-infers it. That is duplicate inference work by design of first-write-wins, not nondeterminism, but I did not prove it. It belongs to the graph exchange (R53's area); I did not change it.
- Found and fixed in my area: for Rust units the fingerprint included the digest of `unit.json`, whose bytes hold `createdMs` and `costMs`. Any honest second build of one unit key (same outputs) therefore differed, was answered 409, was counted as a divergence, made the uploader adopt the fleet copy over its own local entry (`uploadUnitV1` `"diverged"`) and bumped the `skipped` counter. Real divergences of Rust units would have drowned in these.

## 2. Fixes landed

1. `🔬️action-result`: `parseRemoteCacheStableResultV1(bytes, volatileNames)` returns `stable` digests (outputs except the volatile manifest) and `manifest`. `🧾️provenance`: `PROVENANCE_VOLATILE_OUTPUTS_V1 = ["unit.json"]`; the server compares fingerprints that leave the manifest out and recomputes the kept side from the held bytes under the same definition. A different output byte is still 409. Test: access test "two builds of one unit whose manifests differ only in their timestamps...". Language-agnostic fixture `🧾️provenance/🧪️tests/🧫️fixtures/🔣️.json` (6 cases) checked by the independent Python `🐍️.py` and by the TypeScript implementation; a second test ties the name to the unit-cache `manifestName`.
2. Consumer provenance (item 5): the server counts every successful `GET` of an action result as a read by the signed origin (`static`/`anonymous` otherwise) of the producer's entry (`unknown` for entries stored before provenance), per `producer>consumer` pair with the compiled units among them (`u` flag in `provenance.jsonl`), at most 1024 pairs (then `*>*`), persisted in `fleet/consumption.json` (schema `Consumption` in the identity schema) and returned as `consumption` in `GET /status`, which `repo remote-cache status` prints under `server` (schema `ServerStatus.consumption` added). Test: access test "consumption: ..." (pairs `g1-m01>g1-m02` hits 3 units 2, `g1-m01>g1-m01` hits 1; misses and HEAD not counted) and the provenance test (restart, bound).
3. Fail-open branches (item 2), `🛰️fleet/🟦️.ts` `obtainFleetUnitV1`: a claim, an await or a post-wait fetch that fails with `down` is retried `retries` = 2 times with growing pause (750 ms, 1500 ms, in the fleet policy schema) before the unit compiles locally; `denied` and `unsupported` are not retried. The wait on a live claim was already bounded (twice the holder's expected cost, 30 s to 20 min) and is unchanged. Every fall-back is now counted by reason in `<unit cache root>/fl/fb/<reason>` (one byte per event): the `obtainFleetUnitV1` reasons (`stored-unreadable`, `claims-full`, `claims-down`, `claims-denied`, `holder-slow`, `await-fault`, `fault`, `rounds`, `error`, `no-claims`), `breaker-down|denied|error`, `hit-unservable` and `path-dependent`. `--semio-unit-cache stats` prints them as `fallbacks`. Tests: `🛰️fleet/🧪️tests/🧯️fallback` (4: counters, transient claim retried, exhausted, denied not retried, await fault bounded).
4. The `skipped` counter (997) was a sum of breaker skips, path-dependent units and (now fixed) false divergences; the reason counters split it.

Red before, green after: red run recorded in `landings.md` (provenance and fallback files fail to import, both new access tests fail); after the edits (pass counts): provenance 2, action-result 4, server 6, access 8, identity 9 and 8, remote-cache `🛰️fleet` 11, remote-cache root 21, collector 7, fallback 4, unit-cache fleet subset 7, churn 9, uploader 2, preflight 2, heavy-questions 6, all 0 red.

## 3. Path-sensitive units (item 3): what is true

- The code is already past the map: units whose source scan finds a root path are keyed with that path (`sensitive`), shared among machines with the same path; only a leak the source scan did not announce skips the share (`path-dependent`).
- On this machine's event logs (stale, last written 2026-11-14 13:42) 69 `remote-skip` events of 2117 misses (3 percent) were path-dependent: 35 distinct crates, all `semio_s_artifact_stdio_*` rmeta files and the `wit_bindgen_rust_macro` dylib. Their metadata holds the string `/Users/Shared/semio/` followed by path segments (strings the crates' macros produce), checked with `strings` on `libsemio_s_artifact_stdio_semio_schema-982b886bea9179f0.rmeta`. A `--remap-path-prefix` does not touch string constants, so these cannot be made root independent by remapping; they can only be shared among machines at the same root, by keying the post-hoc leaked path in, which needs a second lookup per miss or a server hint. Not implemented. Kinds that truly cannot be shared across roots: crates whose macros embed `CARGO_MANIFEST_DIR` or the workspace root as a string, and linked outputs of a different OS or CPU (by design).

## 4. Offload (item 4): what is true

- `out 139, refused 560, fellBack 512` and `unmapped-path 601` are cumulative since 2026-11-14 morning. `x/offload.jsonl` shows all 601 `unmapped-path` declines carry the bare text "source outside CARGO_MANIFEST_DIR", written between 2026-11-14 13:36 and 13:42; the current planner (`🗺️paths/🟦️.ts`) emits a longer message and ships the module tree, and nothing has been appended since. The counters are therefore stale evidence, not the current decline rate. The other reasons in that log: `worker-policy` 202 (CARGO_PKG_DESCRIPTION value 185, `--check-cfg` 17), `unreadable-input` 21, `incomplete-inputs` 20, `no-idle-machine` 5, 84 offloaded runs (plan 3 to 106 ms, execute 1.1 to 6.5 s).
- Not changed. A current decline rate needs a fresh run on a machine that offloads; I did not start one at load 35 to 45.

## 5. Not verified

- The running cache server (sidecar `build-cache`) and every machine's wrapper still run the old code until restarted or rebuilt; I did not touch processes. The fixes take effect on the next server restart (`/status.consumption`, fingerprint) and for new wrapper invocations (retries, fall-back counters; if the wrapper runs as a built native executable it needs its rebuild).
- No before/after live numbers: before = no consumption record exists, `skipped` 997 unsplit; after = the fixtures above. The first live reading comes after the restart.
- Cause of the 914 record divergences (stale-first-record theory) and the 86 unclassifiable rows; whether Rust unit divergences occurred earlier than the cap shows.
- Linux (g1-m04) and Windows (g1-m05) cannot be run from here; nothing was changed that is platform specific, and no cross-platform reuse of native code was added (wasm32 library units stay shared through `hostIndependentV1`).
- The differential law (remapped build equals a clean local build) was not added: no path-remapping code was changed. The existing remap tests compare with rustc itself; I did not rerun them.
- The full two-machine cargo test of `🛰️fleet/🧪️tests` ("compiled exactly once") was not run (cargo at this load); the claim tests that do not need cargo ran.

---

# Follow-up (19:00 to 19:35 CET): activation, root-keyed sharing, two-machine proof

## A. Activation, and live numbers

- Cache server: the replica daemon's code-closure watch restarted the `build-cache` sidecar by itself (`.cache/f/sidecars/build-cache.log`: "cache code changed, restarting" 18:00:56 UTC, serving again 18:01:22 UTC, after my first landing at about 17:59 UTC; restarted once more at 18:13:41 UTC by someone else's landing). No manual step, no process of mine killed.
- Wrappers: no binary to rebuild. On macOS and Linux Cargo runs a shell stand-in whose name carries the digest of the sealed code (`unitCacheShimPathV1`, `sealModulesV1` in `unit-cache/🚪️launch`), so every new `repo cargo` launch seals and runs the new code. Only Windows has a native executable, which the launcher builds itself (`bun build --compile`). The old `.cache/b/r/m/u/bin/rustc-wrapper` still points at the vanished `/Users/Shared/semio` path; it is an unused leftover of 11-14, no longer what a launch uses.
- Graph read: `.cache/f/graph-server/read-record.json` runner 91760 and read 91761, alive before (etime 48:34) and after (1:09:22) all of this, same pids.
- Live `GET /status` (signed, from the server machine, 19:20): `consumption` exists, counting since 19:01 CET: after 4 303 reads in 19 pairs; 4 207 cross-origin, 96 self; `units` is 0 in every pair (nobody has built Rust units through the fleet since the restart, and entries stored before it carry no unit flag, so unit reuse is not visible yet). Before: no field. Divergences since the first restart: 23 new rows, 22 graph records, 1 not classifiable, 0 units (log still at its cap of 1000, the count is capped). `fallbacks` by reason: none on this machine yet (no `repo cargo` launch has run with the new wrapper; the counters live in `<unit cache root>/fl/fb/` and in `stats`); the fixture build below produced `root-keyed` 2.

## B. The 35 path-sensitive crates: by necessity or not (item 2)

Measured with rustc 1.99-nightly on tiny crates at two roots, `--remap-path-prefix=<root>=…`, scratch `…/R55/exp`:
- Root independent once the remap scope is `all` (the wrapper already forces it, `PATH_NEUTRAL_FLAGS_V1`): `file!()`, panic locations, `#[path]` modules outside the crate directory, `include_str!` with a relative path or with `concat!(env!("CARGO_MANIFEST_DIR"), …)`, a procedural macro's `tracked::path`, debug info. rmeta and rlib byte-identical across roots in every case. Without the remap they differ.
- Root dependent by necessity: a unit that uses the VALUE of `env!("CARGO_MANIFEST_DIR")` or `OUT_DIR` (a `const M: &str = env!(…)`, or a procedural macro that formats the caller's directory into the program): the string is in the bytes and no remap touches a string constant.
- The 35 crates on this machine (`semio_s_artifact_stdio_*`, `wit_bindgen_rust_macro`): their rmeta holds the root followed by the paths of `…/🧬️schema/📸️snapshot/🔣️.json`-style asset files. I could not reproduce that with a tiny crate (every file name case above came out remapped). The 69 events are from 2026-11-14 13:36 to 13:42; I could not tell whether they predate the effect of `--remap-path-scope=all` for these crates, and no fresh build exists to re-measure. The cause stays open; the mechanism below makes it harmless either way.

## C. Root-keyed sharing (landed, `landings.md` "Package R55 landing 2")

A unit whose compiled bytes hold a directory of its machine that the source scan did not announce is, after the first build, keyed by those directories and shared among machines whose directories are equal:
- The producer scans its outputs (`leakedRootKindsV1`: kinds `cwd`, `manifest`, `workspace`, `cargo-home`, `rustup-home`; a leak of the lane directory is never shared). It claims the root-keyed key, publishes a root hint (an action result at `sha256(root-hint, neutral key)` naming the kinds), releases the neutral claim and uploads the unit under the key that includes `kind=path` for each kind.
- A consumer asks for the hint before its first lookup; when it exists, it uses the root-keyed key for lookup, claim, wait and upload. A waiter on the neutral key follows the hint when the claim lapses (`obtainFleetUnitV1(…, redirect)`), so it does not compile a second time. A hint that names a kind the machine lacks, or a leak beyond the hinted kinds, ends in the old "path-dependent, not shared" outcome. A forged hint can only reduce sharing.
- Roots per origin: the store does not record them (`origins/*/home.json` has a random key only; unit manifests carry no working directory), so I cannot list them. The coordinator's statement that most Macs share one provisioned root is what this relies on; a machine at another root builds those units itself and shares them with machines at its own root.

Proof (test `🛰️fleet/🧪️tests/🌳️rooted`, real cargo, tiny workspace core, proc-macro `mac` that formats the caller's manifest directory, `leafr`, `app`; one private cache server; 3 pass; red before: the module did not export the API):
| machine | root | compiled by rustc (`core`, `mac`, `leafr`, `app`) | from the fleet |
|---|---|---|---|
| m1 cold | X | 4 | 0; uploads; `root-keyed` 2 (leafr, and app which inlines the string) |
| m2 cold cache | X (same path) | 0 | all, the two root-keyed units included |
| m3 cold cache | Y | 2 (`leafr`, `app`) | `core`, `mac` (path independent, shared across roots) |
| m4 cold cache | Y | 0 | all |
| clean local build | Z, no fleet | 4 | rlib of `core` equals m1's and m3's byte for byte (differential law: fleet-installed and remapped equals clean local) |

## D. The two-machine proof at different roots (item 3)

The existing `🛰️fleet/🧪️tests` cases, run again after both landings (9 pass, 0 red): "two machines at different paths" (m1 compiles at least 7 units and uploads them; m2 at another path compiles 0, serves every unit from the fleet, runs the same program, `claims.active` 0, ac puts equal m1's misses; editing one crate then recompiles exactly `leaf2` and `app`) and "two machines start the same cold build at the same time" (every unit compiled exactly once in total across both: `app`, 2 build scripts, `core`, `leaf1`, `leaf2`, `mac`; at least 4 waiting remote hits).

## E. Still unverified

- Whether the 35 crates still leak with today's flags (no fresh build of them); the fixture proves the mechanism, not their count.
- Live `units` consumption and live fallback reasons: need real wrapper launches after the next worker waves.
- Cross-root sharing of the root-keyed units cannot happen by design; same-root sharing between two real machines was shown with two cache directories on one machine at the same path, not on two hosts.
- Linux and Windows: not run. Item 4 (graph owner-record divergence) left to R49 as assigned.
