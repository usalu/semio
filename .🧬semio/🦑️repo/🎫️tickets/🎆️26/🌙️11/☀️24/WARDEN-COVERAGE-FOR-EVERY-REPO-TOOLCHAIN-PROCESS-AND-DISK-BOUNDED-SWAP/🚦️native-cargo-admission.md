# 🚦️ Native Cargo Admission (2026-11-24)

## Findings

- **An admitted entry already exists: `repo cargo --store=shared|isolated --configuration=prototype|release [--parallelism=<cores>] -- <cargo args>`.** The build entry gates it as the host-capacity operation `cargo` (`🏗️build/🔣️policy.json`, kind `build`: 1536 MB fixed + 2048 MB per job, 1 cpu per job). `withBuildCapacityV1` waits in the fair queue with progress lines and cancellation, rewrites `--parallelism` to the granted jobs (`grantedBuildArgumentsV1`), and `launchRepositoryCargoV1` then runs cargo in the issued compiler store with the unit-cache wrapper, a per-agent lane, the cargo jobserver and storage custody.
- **Why the scripts bypassed it.** `.cargo/config.toml` sets `build.rustc` to the ingress guard (`🔒️enforcement/🏗️compiler/🚪️ingress/📜️script.ts`, which throws "Original compiler ingress requires the repository launcher and authentic enforcing store") and `build.rustc-wrapper = "repo"`. A bare `cargo` in the checkout therefore dies at its first `rustc -vV` probe; only the launcher issues `RUSTC`/`RUSTC_WRAPPER` after it holds the storage reservation. The ticket scripts escaped with `CARGO_BUILD_RUSTC=rustc CARGO_BUILD_RUSTC_WRAPPER=` (or `--config build.rustc-wrapper=""`), a private `CARGO_TARGET_DIR`, `CARGO_PROFILE_*` environment (which the launcher refuses: profiles belong to issued consumption) and a fixed `-j`/`CARGO_BUILD_JOBS`. Each of these removes a different layer: gate, wrapper and unit admission, shared lane, grant. Their stated reason was "when the ingress gate stalls": a first cold run spends minutes in `build:source-admission` (about 2.5 min cold, 20 s warm for `semio-framework-value`).
- **The guard is correct and stays.** Letting a direct cargo with an explicit target directory pass it would reopen exactly the bypass; the supported way to keep the wrapper active is the launcher, which is what the scripts now call. No wrapper or guard code changed.
- **What was missing** was not an entry but (a) any use of it by the ticket scripts and (b) a language-agnostic proof of the grant→jobs decision of a cargo run with an independent oracle: `🦀️cargo-scaling` only bounds the jobs (`>= 1`, `< 8`).

## Design

No second runner. The scripts call `repo cargo`; the grant derives the width, so nothing in a script names a job count, target directory, compiler or wrapper. `repo capacity run --operation=cargo -- <cmd>` remains the way to admit arbitrary non-cargo heavy work (it exports `SEMIO_BUILD_PARALLEL`).

Scripts (`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/SOURCING-END-TO-END/`): `native-cargo.sh <label> <workspace> <cargo args…>` is the one implementation (log `<OUT>/<label>.txt`, default `🗑️generated`, prints `exit N -> file`, `CONFIGURATION` default `prototype`). `verify-cargo.sh`, `minav-cargo.sh`, `gesture-cargo.sh`, `plugintests-cargo.sh`, `native-cargo-root.sh`, `native-cargo-mem.sh`, `typed-refusal-cargo.sh`, `typed-refusal-wasm-cargo.sh` only choose the log directory and delegate. `native-wasm-build.sh` runs `build --manifest-path <m> -p <pkg> --lib --target wasm32-wasip2` per component (the prototype profile selectors are the `prototype` configuration of the launcher). The `JOBS=n` prefixes of `rust-rest.sh` and `empty-example-laws*.sh` were removed. All are POSIX `sh` (the old ones were zsh with hard-coded `/Users/ueli` paths).

Consequences to know: artifacts now live in the shared agent lane (`.cache/b/r/m/u/l/<lane>/t`), not in `🗑️generated/native*`; `native-stage-modules.ts` still reads `🗑️generated/native-wasm` and must read the lane target after the next component build.

## Tests

`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🧪️tests/🚦️native-cargo-grant/`: `🧫️fixtures/🔣️.json` (15 hosts, authored verdicts), `🐍️oracle/🐍️.py` (standard-library Python re-implementation of the cargo sizing, floor, budget, load ceiling, pressure/swap/files/attach stops), `🟦️.ts`. Four tests, 30 expectations, all pass (`repo test-file`, suite `build-host-capacity`):

1. authored verdicts hold per host (incident host with 5022 MB available and 1832 MB swap: `wait memory`; 8 asked on 8000 MB: 2; on 14000 MB: 5; 64 GB host: 4; critical/warn pressure, worked and saturated swap, files, attaches, load-bound 1 job);
2. the gate (`sizeHostRequestV1` + `decideHostCapacityV1`, shipped policy) equals the Python oracle on every host;
3. the matrix covers all hold-back reasons and at least four distinct granted widths;
4. `withBuildCapacityV1` on a 3000 MB host starts nothing, prints `[capacity] waiting … for cargo: memory …` and ends with a rejection when cancelled.

## Proof

`verify-cargo.sh proof-tiny . check -p semio-framework-value --lib` (rewritten script, warm cache): `[capacity] admitted cargo with 6 of 10 job(s): the host cannot carry more right now`, `Finished prototype profile in 3.25s`, exit 0, 20 s wall. The cold first run of the same command through `repo cargo` took 2:28 and was admitted with 2 of 10 jobs. `native-wasm-build.sh` with `ONLY=semio-s-plugin-sourcing-beams-component` was admitted (5 of 10 jobs) and accepted `--manifest-path`/`--target wasm32-wasip2`; I cancelled it after 100 s (cold source admission), and it released its leases on SIGTERM. The full component build was not run to completion.

## Open

- A cargo started by the old `native-wasm-build.sh` (`-j 2`, wrapper off) was still running during this work (pid 13672); it predates the rewrite and stays unadmitted until it ends. The warden ticket covers seeing such processes.
- Nothing prevents a future script from exporting `CARGO_BUILD_RUSTC=rustc` again; only the warden (subjects by toolchain path regardless of ancestry) can cover that.
