# K6: build-script RUN key is lane- and root-neutral (2026-11-23)

UC = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache`. Scratch: `/private/tmp/claude-501/-Users-ueli-Documents-semio/caf759d9-ccba-4796-9909-827125acd710/scratchpad/k6/` (tiny workspace with a build script, `root`, `lane1`, `lane2`, `probe.ts`). Lock `lock.unit-key-portable` taken and released. Nothing touched the live store or servers.

## Which component differs
- Executable identity: NOT the cause. `build_script_build*.real.semio-run` of R2's two target dirs hold the same `identity` (unit key of the compiling unit + sensitive paths) and the same `key`; the binary is keyed by unit key, not bytes, and the identity file is rewritten on every restore and compile (`📜️script.ts:158,243`).
- OUT_DIR, CARGO_TARGET_DIR, `__CARGO_DEFAULT_LIB_METADATA`, the metadata hash in the binary path: none is a key input (`buildKeyEnvironmentV1` takes only TARGET/HOST/PROFILE/OPT_LEVEL/DEBUG, tool names, CARGO_ENCODED_RUSTFLAGS, CARGO_MANIFEST_DIR/LINKS/PRIMARY_PACKAGE, CARGO_PKG_/FEATURE_/CFG_/DEP_ prefixes).
- Real gap found: `DEP_<LINKS>_<KEY>` variables carry the dependency's build directory (`<target>/build/<crate>-<unit hash>/out`, `<crate>/<hash>` under `build-dir-new-layout`). Only the lane root was neutralised, the unit hash stayed, so any package that consumes a `links` dependency got another local and fleet key per lane and per machine (probe: 3 environments, 3 keys; hash identical gives 1). This is `⟨build⟩` in the new key.
- R2's A/C difference for proc-macro2/quote/serde/serde_core (no `links`, no DEP_) does not reproduce with current code: two lanes (R1 target dirs, shared store) of a real build-script crate give one key `a006ec7c…` (lookups: lane1 miss, lane2 `local-hit`; OUT_DIR `diff -r` identical; `cargo:rustc-env` path translated to lane2's own OUT_DIR). The most likely cause is code generation drift: R2 sealed the wrapper per invocation and K2/K5/S1 landed key changes between its A (36 min) and C runs. Now diagnosable: see "Diagnostics".

## Changes
1. `UC/🔏️key/🟦️.ts`: `buildPathIdentityV1` (any `…/build/<crate>-<16hex>` or `…/build/<crate>/<16hex>` inside a value becomes `⟨build⟩<crate>`), applied to every build-script key value in `buildKeyEnvironmentV1`; `buildDependencyPathsV1` (the DEP_ variables whose value is such a path, with a token each, longest first).
2. `UC/💾️store/🟦️.ts`: `buildRunNeutralV1/buildRunRestoreV1` take those dependency directories: a published OUT_DIR file or stream that names a dependency's directory stores the token and a replay in another lane writes that lane's own directory (so the run key being hash-neutral never replays another lane's hash directory); `restoreBuildRunV1` takes `environment`; the foreign-lane check uses the translated text.
3. `UC/🏗️buildscript/🟦️.ts`: passes `environment` on restore; every run now records a lookup row (`crate: build-script-run:<package>`, outcome local-hit/fleet-hit/miss, named key components and the rustflags value) in `y/lookups.jsonl`, so `churn` explains a run key that differs between lanes the same way it does for units.
4. OUT_DIR itself is not a key input (it is where the output goes); `⟨out⟩` for the compile units' `env!("OUT_DIR")` already exists (`outDirectoryIdentityV1`, both layouts).
5. rerun-if-env-changed values were already part of the run facts (by value, lane-neutral); unchanged.

## Test
`UC/🏗️buildscript/🧪️tests/🛤️lanes/🟦️.ts` + `🔣️.json` (language-agnostic fixture: two lanes of machine one with different lane roots and unit hashes, machine two with other home, checkout and lane): one local key and one fleet key for the three; DEP directory forms; run recorded in lane 1 is found in lane 2, OUT_DIR text file equals the lane-2 spelling, the binary file byte-equal, streams name lane 2's dependency directory and no trace of lane 1; Python hashlib recomputation of the key from the portable inputs. The first assertion failed before the change (3 keys).
`repo test-file`, one at a time, `taskpolicy -b`, load 50: lanes 4/0, buildscript fleet-run 2/0, store contract 24/0, portable 36/0, key 8/0.

## Open
- Real-build confirmation of R2's A/C difference needs a fresh A/C on one code generation (set `SEMIO_UNIT_CACHE_TOOLCHAIN` as before; the run lookups now show the differing component). The running wrapper generation keeps the old code until the next seal.
- Build scripts whose binary has no `.semio-run` identity (bytes fallback) still key by bytes; the wrapper writes it on every compile/restore, so only hand-placed executables hit it.
