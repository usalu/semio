# Package R10: is "nothing is compiled twice" true for compiler actions? (2026-11-15)

Paths: `LIB` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/`, `UC` = `LIB/⚡️caching/🦀️cargo/🗃️unit-cache/`, `RX` = `LIB/🏗️build/🌐️remote-execution/`, `T` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/`. Scripts: `T/🧰️scripts/R10/` (`table.py`, `keyparts.py` = the numbers; `closure.py`, `build_payload.py`, `land.py` = landing).

## Verdict

Disproved as stated, proved for the part that has a mechanism. Measured on this machine (counters since 11-13 22:57, 15342 lookups): 54.8 % of unit lookups were served without compiling (7844 local hits + 566 fleet hits), 6932 compiled. The mechanism that stops two machines starting one unit (claim, await, takeover) works and is exercised by the fleet test ("two machines start the same cold build at the same time: every unit is compiled exactly once in total", 15/15 pass). What is still compiled twice is not the race but (a) the units that never reach the fleet (offload declined or refused, uploads lost), (b) units whose key is not machine-neutral (Windows spellings, host line of the compiler text for WebAssembly), (c) real variants (features, profile, test vs lib), which are different units and not duplicates.

## 1. Numbers (from `table.py`, `keyparts.py`)

Per action kind. Only rustc units have lookups; Nx tasks and wasm stage caches are counted elsewhere (result-AA), cargo build scripts count as rustc units here (`build_script_build`). tsc-like tasks: no per-action counters exist (Nx cache counters are not kept), so "tsc" is NOT measured.

| quantity | value | source |
|---|---|---|
| lookups (local hit + fleet hit + miss) | 15342 | `.cache/f/unit-counters` (one byte per decision) |
| local hits / fleet hits / misses | 7844 / 566 / 6932 | same |
| uploads (stored) / skipped (breaker, path-dependent) | 4488 / 997 | same |
| bypassed command lines (query, no artifacts) | 6 in the 23-minute event window | `…/m/u/x/*.jsonl` |
| offload out / refused / fell back | 139 / 560 / 512 | `.cache/f/offload-counters` |

Reason classes of misses and of units that stayed single-machine (counts in the same window: this machine, 11-14 13:19 to 13:42, 18672 events, 2104 compiles, 2117 fleet lookups all missing, 2072 uploaded):

| reason class | count | where seen |
|---|---|---|
| key differs between machines for equal sources | not measurable from the logs: the logged key parts (`toolchain args source externs environment`) are hashes; of 754 crate/source groups with more than one key for equal source the differing part is always `args` (483 together with `externs`, which follows), never toolchain, source or environment. Across lanes of the same worktree: 0 of 958 crates have one metadata hash per lane with lanes differing; the 192 crates with several hashes have them inside one lane (features, profile, test or host/wasm variants), so lanes are not a cause | `keyparts.py`, lane out dirs |
| action refused by worker policy | 202 fell back in the window; scheduler log: 36 `environment value of CARGO_PKG_DESCRIPTION` (last 3 on 11-15 01:15), 13 `-C linker-plugin-lto`, 2 `--check-cfg`, 1 `SEMIO_REL_CARGO_MANIFEST_DIR path` (all but the first group were seen only before 11-14 12:00 and no longer refuse, test proves it) | `build-scheduler.log` |
| declined before it left (unmapped path) | 601 of 606 declines, ALL with detail `source outside CARGO_MANIFEST_DIR`; 548 distinct crates, every one a workspace crate (`semio_*`), the 58 registry crates ran fine | `offload.jsonl` |
| entry evicted | not distinguishable: no event kind records an evicted entry; the cache index holds 256 + 256 result entries and a 66-line claim log, nothing was dropped by age in the window | `cache-server/v1` |
| upload failed (PUT) | `build-cache.log.1`: 3576 `PUT /main/cas/<sha> failed` lines for 2958 distinct blobs, every one with `process.disk.low-space` (server partition `/` at 97 to 100 % used, `repo cache cas gc` refused to run); the current `build-cache.log` has 0 failures. The client dropped these units for good ("a failed unit is dropped, never retried") | `build-cache.log.1`, uploader |
| never offered | 997 skipped (breaker down/denied or `path-dependent`: 69 events, the compiled bytes hold a machine path) | counters, events |
| in-flight on another machine | claims work (2117 claim `granted`, 0 waits because the window had one active machine); scheduler-level dedup of identical actions exists (`🗓️scheduler/🟦️.ts:151`, counter `deduplicated`) and is tested in the scheduler tests; across machines the fleet claim is the first gate and the scheduler dedup the second | code, fleet test |

Worker-policy reasons by environment value: `CARGO_PKG_DESCRIPTION` only. Cause: the policy accepted no newline in any environment value, but Cargo sets `CARGO_PKG_DESCRIPTION` (and `AUTHORS`, `README`, …) verbatim from the manifest, which is multi-line in the repository (`semio-framework-trace` has a 700-character, multi-line one).

## 2. Causes with file:line (before the change) and the fixes

| # | cause | before | fix | proof |
|---|---|---|---|---|
| 1 | worker policy refused every unit with a multi-line manifest description | `RX/🧱️actions/🟦️.ts:86` `SAFE_TEXT` (no `\n`) | `CARGO_PKG_*` values may hold any byte but NUL, up to 65536; all other names keep the strict rule (`RX/🧱️actions/🟦️.ts:87`) | `🧱️actions/🧪️tests`: new test, red before (`environment value of CARGO_PKG_DESCRIPTION`), green after; loader names stay refused next to a `CARGO_PKG_` variable |
| 2 | the planner declined every workspace crate whose shim lives above the manifest when Cargo compiles in the package directory (absolute source, working directory = manifest directory) or when the module directory is not below the working directory | `RX/🚀️offload/🗺️paths/🟦️.ts:91` | the module directory above the manifest travels as the virtual package whenever it is not under the working directory (`anchored`, `RX/🚀️offload/🗺️paths/🟦️.ts:92`); declined only when it is shallower than 3 path components, when the remaps do not cover it (the compiler would record the machine path), or when its files are not bounded | `🚀️offload/🧪️tests`: new test (same plan in two checkouts, no requester path in arguments, shallow and uncovered directories declined), red before with the exact production detail |
| 3 | a failed upload was dropped for good | `UC/🛰️fleet/📤️uploader/🟦️.ts:129` | `retryJobV1` queues the unit again as a settled offer without its claim, after 15 s doubling, at most 4 attempts, never older than 10 min (policy and `SpoolJob` schema extended, `UC/🛰️fleet/🧬️schema/🔣️.json`); the next drain asks the server first (`hasResult`), so a unit another machine stored is not uploaded twice; spill objects are kept until the last attempt | `📤️uploader/🧪️tests`: retry arithmetic and schema; server answering PUT 500 (the real log's failure), then healthy: uploaded on the second drain, a not-yet-due job is left alone, the last attempt is counted once |
| 4 | WebAssembly libraries were keyed per operating system and architecture, so the Linux and Windows machine share with nobody although the bytes cannot depend on the host | `UC/🛰️fleet/🪪️identity/🟦️.ts:47` (`platformOfV1` always names os, arch, host, linker), and the compiler text with its `host:` line in the key (`UC/🔏️key/🟦️.ts`, `toolchainIdentityV1`) | `hostIndependentV1` (`UC/🛰️fleet/🪪️identity/🟦️.ts:52`): a `lib`/`rlib` for a `wasm32-` target other than the host, not native-CPU, is keyed by the compiler release WITHOUT its host line (`neutralReleaseV1`); platform `os/arch/host = any`; the local key uses the same neutral toolchain (`unitToolchainV1`, `UC/🛰️fleet/🟦️.ts:34`); policy `hostIndependent` and `Platform.os` enum extended in the schema; proc-macros, bins, cdylib/dylib/staticlib, host units and native-CPU stay platform-bound | law test below |
| 5 | Windows spellings produced other keys than POSIX for the same unit: the registry/git/rust-src remap roots matched only `/`, so `C:\Users\x\.cargo\registry\src` stayed in the key; `crates\core\src\lib.rs` and `@@semio-cwd@@\crates\core` differ from the POSIX spelling; a Windows root was resolved with the host's path rules (a Windows root on a POSIX test machine was relative) | `UC/🧭️remap/🟦️.ts:32` (`MACHINE_ROOTS`), `UC/🛰️fleet/🪪️identity/🟦️.ts` (`spellings`, `portableTextV1`, `portableKeyV1`, `slotPartsV1`) | separators accepted in the three roots; Windows paths resolved by `win32` rules wherever they appear; the tail after a root token and the source path are spelled with `/` in the portable key (only text that holds a backslash after a token changes, so no POSIX key moves) | law test, 4 vectors against Python `PureWindowsPath` |

## 3. The law test (`UC/🛰️fleet/🪪️identity/🧪️tests/🌍️machine-law/🟦️.ts`)

Four simulated machines (macOS `/Users/ueli/Documents/semio`, Linux `/home/deck/semio`, Windows `C:\semio`, a second macOS at `/Volumes/Work/ci/checkout/semio`), each with its own home, host name, `CARGO_HOME`, `RUSTUP_HOME`, path separator, `HOME`/`USERPROFILE`/`HOSTNAME`/`COMPUTERNAME` in the environment, real `normalizeRustcArgumentsV1` + `keyEnvironmentV1` + `portableRootsV1` + `remoteKeyV1`:
- WebAssembly library: ONE key and ONE slot on all four (before: 4 keys on 4 machines, measured with the same harness; the Windows registry remap and the `host:` line were the two components that differed, found by dumping the portable key input per machine).
- Native library, proc-macro, wasm cdylib, wasm native-CPU: equal on the two macOS machines (different roots, homes, names), three different keys for macOS/Linux/Windows.
- Sensitivity: a changed source, another compiler commit, another target, `target-cpu=native`, `rlib` instead of `lib` all move the key; `cdylib`, `proc-macro`, `bin`, `staticlib` and the host's own target are never host-independent.
- Third-party cross-check: Python `pathlib.PureWindowsPath` computes the portable text of 6 Windows (and UNC) vectors; ours equals it.

## 4. Verification

`BUN_RUNTIME_TRANSPILER_CACHE_PATH=0`; scratch copy of the touched closure outside the worktree (no links), then the landing script on the live tree.
- scratch: actions 5 pass, offload 26 pass, determinism 5 pass, remap 6, key 8 + parts 1, identity 8, machine-law 4, uploader 2, extern 1, embedding 2, fleet tier (real Cargo workspaces through the wrapper, real cache server over sockets) 15 pass in 194 s. The two Windows-stand-in tests fail in the scratch copy only because it has no `node_modules` for the native bundler; the same file passes 7/7 on the live tree.
- landing (under the lock, 12 files, load check ok on all 7 sources): all 11 test files green on the live tree, 0 red before and after, including the fleet tier (real Cargo builds) and the offload cluster test. Recorded in `T/landings.md` "Package R10"; originals for `land.py --rollback` in `T/🗑️generated/R10-landing-*`.

## 5. NOT verified / what remains duplicated

- Output bytes of an offloaded unit in the new "package directory" shape against a local compile: the planner is proven path-independent and bounded, the rustc byte comparison of the existing determinism test covers the old shapes only. First live offload of a module-layout crate is the proof; the divergence detector (`remote-skip: diverged`) would show a mismatch.
- Host-independent keys across operating systems: proven as keys, NOT as bytes. Only macOS arm64 exists here; whether a Linux or Windows rustc gives byte-identical wasm32 rlibs is unproven. Safety net: first-upload-wins plus the existing divergence report (`fleet/divergences.json`); to turn it off, empty `hostIndependent.targetPrefixes` in the fleet schema. Every wasm32 unit key moves once (new toolchain component), so the first build after landing recompiles them once per machine.
- Why 745 of 976 crates show more than one miss key in the window: the logs hold only hashes, the component is `args`. Lane-independence is shown (0 cross-lane differences), feature/profile variants explain the 192 multi-hash crates on disk, but a per-argument attribution needs the key parts split by argument class; the `🧩️parts` format has a Python oracle and fixtures, so I did not change it. Suggested next step: add `meta:` (metadata, extra-filename), `cfg:` and `codegen:` digests with the oracle.
- Entry eviction as a miss cause: no event distinguishes it; counts impossible.
- tsc-like/Nx task duplicates: no counters; not measured.
- Still compiled twice by design: real variants (features/profile/test/lib), units with `path-dependent` output (69, shared only between machines with equal paths), incremental workspace units per lane, link steps and build-script runs outside the unit cache (`audit-wasm-and-still-twice-1114.md` section 5), units whose offload waits on a busy fleet and then run locally (no-idle-machine, 21).
- The disk-full cache server (97 to 100 % used) is an infrastructure cause outside the code: retry only helps once space returns; `process.disk.low-space` also refused `repo cache cas gc`, which would free the space (circular refusal, not touched: package R6/R8 area).

## 6. What the other packages must know

- `Platform.os` can now be `any`; every consumer that switches on it must accept it (searched: no other member reads it).
- `SpoolJob` has optional `attempts`, `notBeforeMs`; `drainUploadsV1` returns `retried`.
- `unitBasicsV1` hands a neutral release (`host: "any"`) to `slotPartsV1` for host-independent units, so their slot id is the same on all platforms.
- Nothing in `🏗️build/🕸️graph`, the dependency store, closure-missing or the plan memory guard was touched.
