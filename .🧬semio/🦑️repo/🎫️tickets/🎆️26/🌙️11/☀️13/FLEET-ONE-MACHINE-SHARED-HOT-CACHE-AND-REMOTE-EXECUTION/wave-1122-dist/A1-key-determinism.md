# 🔑️ A1: Why a Rust unit compiled on one machine is not reused on another

Lane A1, read-only audit. No code or ticket edits. No sub-agents. Live numbers are a snapshot taken 2026-11-22 around 20:55 local time; the live files keep moving.

UC = `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache`

## Summary

- Fleet-wide, about 149,100 units were compiled but not shared (the sum of the per-machine `cacheSkipped`, `repo fleet status`). About 85 % of them (126,971) are `spool-full`. That is upload back-pressure, not a key problem. Key-related reasons (`key-incomplete`, `path-dependent`, `no-compiler-identity`, `publish-interface`, `diverged`) together are about 9,000 (about 6 %), and the rest are upload, bypass or legacy-counter reasons.
- The coordinator itself is healthy: 42 skipped against 16,310 local-plus-fleet hits and 24,235 misses. Its only key-side skips are 20 `key-incomplete` and 2 `path-dependent`.
- The key is portable only for the checkout root (`cwd` and workspace). Everything else that is machine-bound in the key is a real divergence risk: the raw `CARGO_MANIFEST_DIR` of registry crates (CARGO_HOME is not a portable root), the raw compiler and wrapper paths in build-script keys, and `-C incremental` (correctly excluded). The evidence points at CARGO_HOME as the main cross-machine splitter for dependency crates; this is a hypothesis to confirm with one traced run, see section 5.
- Build-script RUN outputs are never fleet-shared (local store only). Proc-macros and dylibs are shared only among machines with the same host triple and Darwin linker major. Incremental units are not shared by design.

## 1. What goes into the fleet key

Entry: `wrapRustcV1` in `UC/📜️script.ts:70`. Per unit it computes a local key (line 131, `localKeyV1`) and a fleet key (`prepareFleetV1`, `UC/🛰️fleet/🟦️.ts:51`, which calls `remoteKeyV1`, `UC/🛰️fleet/🪪️identity/🟦️.ts:131`).

Fleet key (`remoteKeyV1`, identity:131-133):
`sha256([policy namespace, fleet namespace, platform, portableKeyV1(input, roots), optional "real-paths" list for sensitive units])`

`portableKeyV1` (identity:125-128) is `unitKeyV1` (`UC/🔏️key/🟦️.ts:142-144`) over:

| Part | Source | Where |
|---|---|---|
| namespace | `semio.cargo.unit-cache/v1` | schema Policy.namespace |
| toolchain | `unitToolchainV1` (`🛰️fleet/🟦️.ts:36-43`): full `rustc -vV` text (own) or the `-vV` text without its `host:` line for host-independent units (wasm lib/rlib, `hostIndependentV1`, identity:52-57) | `🔏️key/🟦️.ts:176-183` for the own text |
| normalized args | `normalizeRustcArgumentsV1` (`🔏️key/🟦️.ts:84-106`) | see list below |
| source digest | `unitSourcesV1` (`🔏️key/🔗️sources/🟦️.ts:27-39`): entry file, every module/include the scanner reaches, plus `OUT_DIR` files as `⟨out⟩/…` rows (max 256) | sources:31, 34 |
| externs | `name` plus dependency digest: interface identity (rmeta tables) or extern identity, else file digest | script.ts:130 |
| environment | `keyEnvironmentV1` (`🔏️key/🟦️.ts:135-139`) and env names read by `env!`/`option_env!` (`sourceEnvironmentV1`, sources:42-44) | |

Normalization (`normalizeRustcArgumentsV1`, `🔏️key/🟦️.ts:84-106`) drops or rewrites:
- `--out-dir` (line 89), `-C incremental=` in all three spellings (90-91), `-L dependency=` (92, 101), `--diagnostic-width` (93-94), `--color` (95-96), the `diagnostic-rendered-ansi` and `diagnostic-short` JSON words (97-100).
- `--extern path` becomes `--extern name=#<digest>` (102), so extern paths never enter the key.
- Every other argument goes through `machineNeutralRemapV1(laneNeutralV1(arg, laneRoot))` (103). The lane root becomes `@@semio-lane@@` (70-75).
- Then the whole argument list is tokenised by the portable roots in `portableKeyV1` (`portableTextV1`, identity:107-115).

Environment (`keyEnvironmentV1`, `🔏️key/🟦️.ts:135-139`), names from schema `key.environmentNames` and prefix `CARGO_PKG_`:
- `CARGO_CRATE_NAME`, `CARGO_BIN_NAME`, `CARGO_MANIFEST_DIR`, `CARGO_ENCODED_RUSTFLAGS`, `RUSTC_BOOTSTRAP`, `SDKROOT`, `MACOSX_DEPLOYMENT_TARGET`, `IPHONEOS_DEPLOYMENT_TARGET`, plus `CARGO_PKG_*`.
- `CARGO_ENCODED_RUSTFLAGS` goes through `neutralRustflagsV1` (`🔏️key/🟦️.ts:115-118`). `SDKROOT` is reduced to the SDK directory name (`sdkIdentityV1`, 109-112).
- Not in the key: `HOME`, `CARGO_HOME`, `RUSTUP_HOME`, `PWD`, `TMPDIR`, `OUT_DIR`, timestamps, hostname and pid. Only `fleetOwnerV1` (`🛰️fleet/🟦️.ts:22`) uses hostname and pid, for claims.

Platform (`platformOfV1`, `🛰️fleet/🪪️identity/🟦️.ts:63-68`): os, arch, host triple, digest of the full `-vV` text, toolchain digest, linker `darwin-<major>` (only for bin, cdylib, dylib, staticlib), and CPU identity only for `target-cpu=native`. Host-independent wasm units collapse to `any` (line 65).

Machine-specific things checked, and what they do to the key:

| Machine-specific value | Keyed as | Effect across two same-platform machines |
|---|---|---|
| Checkout root (`cwd`, workspace) | portable token `@@semio-cwd@@`, `@@semio-ws@@` (`portableRootsV1`, identity:98-104) | Neutral. Tested in `🧪️tests/🧳️portable/🟦️.ts:37-43`. |
| Lane / target dir (`--out-dir`, `⟨cargo-build-dir⟩`, `-L dependency`) | removed or remapped (`🔏️key/🟦️.ts:89, 92`, `machineNeutralRemapV1`) | Neutral. |
| `CARGO_HOME` / registry path, when it is NOT the cwd of a crate | not a portable root; raw in `CARGO_MANIFEST_DIR` (`keyEnvironmentV1` line 137 applies `laneNeutralV1` and portable tokens only where a root matches) and in `--remap-path-prefix` values (token only when `machineNeutralRemapV1` knows the role) | DIVERGES if CARGO_HOME or the registry path differs between machines. See section 4, cause A. |
| `RUSTUP_HOME`, rust-src, sysroot | remap roles (`⟨rust-src⟩`, `script.ts:80-88`) | Neutral in args; not portable when they leak into bytes (see path-dependent). |
| SDK location | name only (`sdkIdentityV1`) | Neutral. |
| Compiler binary path | not in unit key; the host `-vV` text is the toolchain | Neutral. |
| Timestamps, pid, hostname | not in unit key | Neutral. |
| `semio-compiler-resources/<epoch-ms>-<pid>-0/…` files read by a crate | read by the crate, appear as dep-info gap | Never shareable (per-run file). See section 2, key-incomplete. |
| `-C incremental=<dir>` | removed from the key (`🔏️key/🟦️.ts:90-91`) | Key-neutral, but bytes differ, so the unit is kept out of the fleet (script.ts:214-215). |
| `-C metadata` | kept (`argumentClassOf` line 226) | Cargo computes it from the package id; equal on equal trees. Not tested cross-machine here. |

## 2. Skip reasons: where recorded, live counts, root causes

Recording: `countSkipV1` (`UC/🛰️fleet/📊️counters/🟦️.ts:26-29`) appends one byte to `<workspace>/.cache/f/unit-counters/skip/<reason>`. `skipped` is the sum of those files (`readBuildCountersV1`, counters:57-62). Fallbacks (local compile after a fleet attempt) are counted separately under `<root>/fl/fb/<reason>` (`countFallbackV1`, counters:44-46).

Live coordinator counters (`/Users/ueli/Documents/semio/.cache/f/build-counters.json`, `unit-counters/skip/`):

| Reason | Coordinator | Code site |
|---|---|---|
| `key-incomplete` | 20 | script.ts:216 (`keyGapV1`) |
| `bypass-no-artifact-messages` | 13 | script.ts:20 (`rustcBypassV1`, key-util line 58) |
| `upload-failed` | 5 | uploader:156 |
| `path-dependent` | 2 | script.ts:232 (`leakedRootKindsV1`) |
| `incremental` | 1 | script.ts:215 |
| `upload-expired` | 1 | uploader:139 |
| total `skipped` | 42 | build-counters.json |

Coordinator unit counts: localHit 12,889, remoteHit 3,421, miss 24,235, stored 14,153. Fallbacks (`fl/fb`): breaker-down 4,676, fault 748, root-keyed 147, hit-unservable 12, root-key-failed 29, path-dependent 16, holder-slow 2, claims-down 28. The coordinator's spool `fl/up` is empty (0 jobs).

Fleet-wide (`repo fleet status --json`, per-machine `cacheSkipReasons`, machines m01 to m13 and g1-server; m04, m05 and m09 offline). Summed by me:

| Rank | Reason (as recorded) | Fleet count | Share | Machines (count) | Root cause in code |
|---|---|---|---|---|---|
| 1 | `spool-full` | 126,971 | 85.2 % | m02 52,586; m06 50,412; m07 12,845; m10 10,292; m12 836 | `enqueueUploadV1` (`UC/🛰️fleet/📤️uploader/🟦️.ts:38`) refuses once 4,096 jobs wait (2,048 for offers). `shareFleetUnitV1` then returns false and `unsharedReasonV1` (`🛰️fleet/🟦️.ts:204-206`) reports `spool-full` (script.ts:217). The unit is never re-offered later: offers only happen on a local hit (`offerFleetUnitV1`, fleet:196-201). Root cause is upload back-pressure while the fleet cache or breaker is down (see `upload-expired-down`, `breaker-down`). Not a key problem. |
| 2 | `unrecorded` | 8,936 | 6.0 % | m01 3,344; m12 5,592 | Reporter remainder, not a live reason. Each machine still has a legacy `unit-counters/skipped` file (the coordinator's is 5,687 bytes, last written 11:54), which `readSkipReasonsV1` ignores (counters:35-39). Fix: delete the legacy file (the greenfield rule allows it). |
| 3 | `key-incomplete` | 6,714 | 4.5 % | m02 1,479; m08 1,669; m06 1,297; m10 1,202; m07 1,025; server 20; others | `keyGapV1` (`UC/🔏️key/🔗️sources/🟦️.ts:47-52`): the dep-info of the finished compile lists a file that the source scan did not cover, or an environment name the key does not select. Coordinator examples: `serde_core-1.0.229/src/de/mod.rs` (8 events), `libc-0.2.191/src/new/apple/xnu/mach/…` (4 events), `typst-assets-0.14.2/src/../files/icu…`, a derive macro reading `mutation-authority.json`, and `semio-compiler-resources/<ms>-<pid>-0/observation.json` (per-run, never shareable). The scanner (`reachedFilesV1`, sources:30) misses paths reached through `..` or macro-driven reads. Fix: add dep-info files to the source rows on the next compile (store the dep-info per first-level key), and exclude per-run runtime files by design. |
| 4 | `upload-expired` / `upload-expired-down` | 1,975 + 1,142 = 3,117 | 2.1 % | m12 821; m07 512; m10 512; m02 756; m06 512 | Uploader drops a job older than `maximumJobAgeMilliseconds` (uploader:139), mostly while the breaker is down. Same root as rank 1. |
| 5 | `path-dependent` | 1,091 | 0.7 % | m08 140; m06 367; m02 500; m10 31; m07 29; m12 16; m13 5; m01 1; server 2 | `leakedRootKindsV1` / `leakedPathsV1` (script.ts:232): compiled bytes contain a machine root. Examples: `libwit_bindgen_rust_macro-*.dylib` embeds `/Users/ueli/Documents/semio` (8 events); `libsemio_s_artifact_stdio_*.rmeta` embeds `/Users/Shared/semio`, a second checkout root that is not a portable root (about 20 events). Fix: remap or tokenise proc-macro output, and add a `/Users/Shared/semio` style root as a portable root only where the checkout really is the same. |
| 6 | `no-compiler-identity` | 471 | 0.3 % | m06 only | `fleetUnidentifiedReasonV1` (fleet:46-48): `readFleetSettingsV1` fails on that machine. Machine-local configuration fault on m06, not a key issue. |
| 7 | `publish-interface` | 527 | 0.4 % | m06 303; m02 216; m08 6; m07 2 | script.ts:175+29: `writeDependenciesV1` returned no crate-hash tables for an rmeta (`tables=false`). The interface identity is then unknown and the unit is skipped. Root cause not yet traced. |
| 8 | `bypass-no-artifact-messages` | 601 | 0.4 % | m02 218; m06 162; m10 63; m12 53; m07 48; m08 40; server 13; others | `rustcBypassV1` (`🔏️key/🟦️.ts:58`) refuses any command line without `artifacts` in `--json`. Events name `proc_macro2`, `thiserror`, `anyhow` (81 events). Not cached at all, by design. Open: which Cargo invocation passes no artifacts message. |
| 9 | `upload-failed` | 303 | 0.2 % | m07 171; m12 45; m08 66; m02 12; m06 3; server 5 | uploader:156 after 4 attempts. Network or server errors. |
| 10 | `breaker-down` (fallback and skip) | 240 (skip) / 4,676 (fallback) | 0.2 % / n.a. | m12 238; m02 2 | Breaker open: the unit is compiled locally and waits in the spool (`🛰️fleet/🟦️.ts:180`). |
| 11 | `diverged` | 80 | 0.05 % | m10 65; m02 10; m12 5 | Same key, different output bytes: the server refuses the second writer. See section 3. |
| 12 | `bypass-native-link` | 38 | 0.03 % | m06 31; m02 7 | `rustcBypassV1` line 62: linked crate with `-l` or native `-L`. Not cached, by design. |
| 13 | `root-key-failed`, `publish-refused`, `incremental`, `upload-breaker` | 7 + 12 + 2 + 1 | <0.02 % | | Rare. `publish-refused` is `publishUnitV1` declining (script.ts:210, reason `refused`). |

Reading the fleet numbers: they are cumulative since each machine's counter file began, not for one build. The counters are a lower bound for the live state (the coordinator's own counter is the only one I could read directly).

Hits for comparison (per fleet-status): `cacheHit` 260,484; `cacheMiss` 435,668 across machines.

## 3. Output divergence, not key divergence

The fleet cache server holds 149 refused writes (`repo remote-cache status`, `divergences.count` 149). Among the 23 `sort: unit` evidence entries I read (the list was 119 entries; I did not read them all), each has the same key on g1-m07 (kept) and g1-m10 (refused), and exactly one differing file, an rlib such as `out/libsemio_framework_job-cd310d33bd29fe78.rlib`. The `.ac` action records are the same size but have different digests. So the key is equal across these machines and the rlib bytes are not reproducible. Cause not traced. Likely candidates: a path or timestamp embedded in the rlib, or a machine-specific codegen input. Open item.

## 4. Ranked causes of cross-machine key divergence (same platform, same synced tree)

Ranked by expected impact on sharing, not by the skip count above (the skip count is dominated by upload back-pressure).

A. `CARGO_HOME` and registry paths in keys for dependency crates (highest). Evidence: 449 distinct non-token `env:CARGO_MANIFEST_DIR` digests across the 2,525 lookup rows of the coordinator snapshot (the cwd token digest `e15a71c2efb9c063` = `sha256('"@@semio-cwd@@"')[:16]` appears on 1,513 rows). Coordinator's registry crates live under `/Users/ueli/Documents/semio/.cache/b/r/m/cargo/home/registry/…` (key-incomplete events), so CARGO_HOME is a per-checkout path. Registry crates are not portable roots (`portableRootsV1` gets only cwd and workspace; `ROOT_KINDS_V1` `cargo-home` is used only by the root-hint fallback, `🛰️fleet/🟦️.ts:97`). Two machines with different CARGO_HOME will therefore key every registry crate differently, unless the crate is compiled with cwd equal to its package dir (then the cwd token applies). I could not confirm the cwd Cargo gives registry crates from this code alone. Verify with one traced run (`SEMIO_UNIT_CACHE_TRACE_KEYS=1` on two machines, compare `env:CARGO_MANIFEST_DIR` of one registry crate).
   Fix: add `cargo-home` and `rustup-home` as portable roots (tokens `@@semio-cargo-home@@`, `@@semio-rustup-home@@`) in `portableRootsV1` and in the `hints`, so every spelling is tokenised in the key, not only in leak detection.

B. Build-script RUN keys are fully machine-bound (see section 5). Not shared at all.

C. `RUSTC_WRAPPER`, `RUSTC`, `RUSTC_LINKER` and un-neutralised `CARGO_ENCODED_RUSTFLAGS` in build-script keys (`buildKeyEnvironmentV1`, `🔏️key/🟦️.ts:206-210`). This function applies only `laneNeutralV1`, not `neutralRustflagsV1` or `portableTextV1`, unlike `keyEnvironmentV1` (`🔏️key/🟦️.ts:137`). Divergence for any build-script key on two machines.

D. Path leaks in bytes (`path-dependent`, 1,091 fleet-wide). Unit bytes embed a root that is not portable (proc-macro dylib with the workspace root, rmeta with a second checkout root). These units are kept local by design (`script.ts:232`). Shareable only after the leaking path is remapped or tokenised.

E. Source scan gaps (`key-incomplete`, 6,714). Dep-info files the scanner misses. Shareable after the dep-info is added to the key on the next compile.

F. Output non-determinism (`diverged`, 23 unit evidence entries). The key is equal but the rlib bytes are not. Not a key problem, but the same effect: no sharing.

G. Platform split by linker major and host triple (proc-macros, dylibs, bins). Only machines with the same `darwin-<major>` share linked units. All fleet machines report the same platform digest (`macos-arm64-312cc5b2932fbb31`), so this is currently not splitting the fleet, but I could not check every machine's Darwin major.

H. Interface identity failures (`publish-interface`, 527). Unit not shared.

## 5. The three specific questions

(a) Build-script RUN outputs: not shared.
- `runBuildScriptV1` (`UC/🏗️buildscript/🟦️.ts:52-83`) only calls the local `lookupBuildRunV1` and `publishBuildRunV1` (`💾️store`). No fleet client call exists in that path. Counts on the coordinator: `script-hit` 370, `script-miss` 709, `script-publish` 709 (event log). These are local only.
- Key (`🏗️buildscript/🟦️.ts:62`): namespace, `"build-script"`, the full `-vV` text (`toolchainIdentityV1`), the SHA-256 of the compiled build-script binary (`digest(real)`), the raw run arguments, and `buildKeyEnvironmentV1` (`🔏️key/🟦️.ts:206-210`).
- Machine-bound parts: the raw `CARGO_MANIFEST_DIR` (not tokenised), the raw `RUSTC`/`RUSTC_WRAPPER`/`RUSTC_LINKER` paths, the un-neutralised `CARGO_ENCODED_RUSTFLAGS`, and the compiled binary digest (its paths are not trimmed). Any two machines with different checkouts or CARGO_HOME get different keys.
- What would make it shareable: route the key through `portableKeyV1` with the roots, reuse `keyEnvironmentV1`'s neutralisation for the env, key the binary by its neutral digest, and publish to the fleet store like a unit. The stdout of a build script that embeds a path would still need the leak check.

(b) Units that depend on OUT_DIR.
- OUT_DIR itself is not in the key. `--out-dir` is removed (`🔏️key/🟦️.ts:89`), and the generated files are keyed by content (`unitSourcesV1` sources:31, 34, rows `⟨out⟩/…`). So such units are shareable when the generated content is path-free.
- Limits: the walk stops at 256 files (`OUTPUT_FILES`, sources:11, 19). Beyond that, generated files are not keyed and show up as dep-info gaps (`key-incomplete`). I inferred this from the code, not from a live count.
- Generated code with absolute paths (for instance `concat!(env!("OUT_DIR"), …)` embedded into bytes) leaks through `leakedRootKindsV1` and becomes `path-dependent`.

(c) Incremental, proc-macros and dylibs.
- `-C incremental`: not shared. The key ignores the incremental session (`🔏️key/🟦️.ts:90-91`), so the wrapper compiles shared non-focus units without it (`withoutIncrementalV1`, script.ts:114). Units that still compile with a session are kept out of the fleet with reason `incremental` (script.ts:215, 2 events fleet-wide). The slot identity also carries `inc=0/1` (identity:162), so the two spellings never share a slot.
- Proc-macros and dylibs: the cache treats them as cacheable (schema `cacheableCrateTypes`, includes proc-macro, dylib). They are host-bound: `hostIndependentV1` is false for host-target units (identity:52-57), so the platform is the host triple, the full compiler digest and, for dylib, the Darwin linker major (identity:66-67). Proc-macro is not in `linkedCrateTypes` (schema), so its linker is empty. They share only among machines with the same host triple and compiler build. The wit-bindgen proc-macro dylib embeds the workspace root (8 path-dependent events), so it is not shared at all today.

## 6. Open items

1. Verify cause A with one traced run: `SEMIO_UNIT_CACHE_TRACE_KEYS` on two machines (or the coordinator and g1-m06) for one registry crate. Confirm the cwd Cargo passes to rustc for registry crates.
2. Trace the `publish-interface` path (`writeDependenciesV1`, `🔭️interface`) for the rmeta that lacks tables.
3. Find the invocation behind the `no-artifact-messages` bypass (proc_macro2, thiserror, anyhow).
4. Compare two `libsemio_framework_job-*.rlib` from g1-m07 and g1-m10 (`repo remote-cache evidence show <id>`), to find what varies in the rlib bytes.
5. Delete the legacy `unit-counters/skipped` file on all machines so the `unrecorded` remainder disappears.
6. Verify the Darwin major on all fleet machines (linked units split by `darwin-<major>`).

Tests reviewed (not run): `UC/🧪️tests/🧳️portable/🟦️.ts` covers two synthetic machines with different home, checkout, target and SDK paths for a workspace member only. No test covers a registry crate under a different CARGO_HOME, nor a build-script key across machines. That gap is why cause A and B went unnoticed.

Tools run: `repo remote-cache status`, `repo fleet status --json`, `repo remote-cache evidence list` (read as a JSON file), `curl` on the cache server (`/healthz` 200; `/status` requires the signed fleet fetch, so I used `repo`), and read-only python over `UC` counter, event and lookup files. Nothing was written outside this report.
