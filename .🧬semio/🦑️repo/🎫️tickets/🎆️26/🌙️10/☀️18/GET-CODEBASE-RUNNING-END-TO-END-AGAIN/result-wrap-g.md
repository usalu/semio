FIXED 16:12

FOR gate-g (wrap-g 14:30): FIX-OF-RED for the cold-lane `failed to run rustc: EBADF ... posix_spawn` (lane2-g print-rs:build 13:40, integ-g dashboard test-journeys 13:25/13:34/13:51). Failing runs: `C/🗑️generated/lane2-g/official-print.log` line 11399, `C/🗑️generated/integ-g/c3{,b,c}.dash.log`. Files edited LIVE under `🗃️unit-cache` (3 files, one compile-atomic set, all unit-cache tests green): `📜️script.ts`, `🚪️launch/🟦️.ts`, `🧪️tests/🟦️.ts`. My one official run `bun repo --tool=bazel run @semio-tech/print-rs:build --skip-cache` ran 14:27-16:12 and is DONE (green); no need to wait.

# result-wrap-g (executor wrap-g, wave 1026g)

## Root cause (exact)
1. With the build-script cache on (`SEMIO_UNIT_CACHE_BUILD_SCRIPTS=on`), the wrapper replaces every compiled `build_script_build` by a Bun stand-in (`🏗️buildscript` `shimSourceV1`) that runs the real executable with `spawn(...,{stdio:"inherit"})`. The real build script therefore has ONLY descriptors 0-2; the jobserver pipe Cargo handed to the stand-in (`--jobserver-fds=3,5`) is not passed on. Its environment still carries `CARGO_MAKEFLAGS=-j --jobserver-fds=3,5 --jobserver-auth=3,5`.
2. libc's `build.rs` honours `RUSTC_WRAPPER`, so it starts the unit-cache wrapper (a new Bun process). Bun opens an internal pipe at the lowest free descriptor: FD 3. The wrapper's `run()` (`jobserverDescriptorsV1`) parsed `3,5` out of the stale flags, "validated" them with `fstat` (conc-cargo's earlier fix: fd 3 exists, fd 5 does not), and spawned the compiler with `stdio:["inherit","inherit","inherit",3]`, i.e. a `dup2(3,3)` of Bun's own internal pipe: `posix_spawn` returned EBADF.
3. Why only now: the build script of a crate that does NOT call `$RUSTC_WRAPPER` (proc-macro2 etc.) never reaches the wrapper; the old shared directory had all build scripts fresh; a WARM lane never runs a build script. Only a cold lane compiling a crate like `libc` shows it. Why integ-g could not reproduce it standalone: the numbers depend on how Cargo was started (a Cargo spawned by Bun/`bun test` gets jobserver fds 6,7, which do not collide with Bun's fd 3; a Cargo started from a shell gets 3,5).
4. An in-process `fstat` can never tell a real inherited descriptor from one the runtime opened itself at the same number (integ-g's Bun 1.3.14 failure with `3,4` is the same fault).

## Reproduction (graph-free, 2 s)
`C/wrap-g-tools/run-repro.sh <bun> [VAR=val]` (crate `repro` = `libc =0.2.190`, crate `repro2` = a build script that runs `$RUSTC_WRAPPER $RUSTC --version`; cold lane, cold cache, the pinned toolchain Cargo, the real wrapper script behind the real stand-in). Before the fix: `failed to run rustc: EBADF: bad file descriptor, posix_spawn '.../bin/rustc'` (libc 0.2.190 build.rs:390) with the pinned Bun 1.4.2. Descriptor census scripts: `fds.ts`, `fds-first.ts`, `spawn-try.ts` (variants of the stdio vector: only `[...,3,"ignore",5]` with 3 AND 5 open works, `[...,3]` with 5 closed gives EBADF).

## Fix (source of the fault: the wrapper trusted the environment)
- `🚪️launch/🟦️.ts` `unitCacheShimSourceV1`: the POSIX stand-in is a `/bin/sh` script, and it runs BEFORE the runtime opens any descriptor. It unsets `SEMIO_UNIT_CACHE_JOBSERVER_FDS`, parses the pair out of `CARGO_MAKEFLAGS` (else `MAKEFLAGS`; `--jobserver-auth` wins over `--jobserver-fds`, last declaration wins; `fifo:` paths, single numbers and triples are no descriptor pair) and exports `SEMIO_UNIT_CACHE_JOBSERVER_FDS=<r>,<w>` ONLY when both descriptors are open (`( : <&r ) 2>/dev/null && ( : >&w ) 2>/dev/null`).
- `📜️script.ts` `jobserverDescriptorsV1` reads only that variable; the `fstat` filter in `run()` is gone (and its import). Windows is unaffected (native executable, named semaphore, no descriptors).
- The build-script stand-in still does not hand the jobserver down to real build scripts (unchanged, pre-existing: scripts that spawn `make`/`cc` draw no tokens from Cargo's pool; the `jobserver` crate ignores descriptors that are not open). Not changed here.

## Laws (`🧪️tests/🟦️.ts`, +3, suite 26/26, windows suite 5/5)
1. real Cargo + real build script that runs `$RUSTC_WRAPPER $RUSTC --version|-vV` with stale jobserver numbers in its environment, cold lane (RED before: exit 101, GREEN now).
2. the stand-in survives stale numbers `3,5` / `3,4` / `4,5` / `3,3` / `fifo:` directly (RED before: EBADF; GREEN now).
3. the stand-in reports exactly the open pair: vectors checked against an independent Python `os.fstat` census of the same descriptors (both open, one end open, none open, other numbers, fifo, one number, three numbers, no flags, `MAKEFLAGS` fallback, a value inherited from a parent wrapper dropped).
Existing assertion adapted: the shim text test now checks the `#!/bin/sh` head and the quoted `exec` tail.

## Measurements
Graph-free repro, cold lane, libc: before = fail in 1.8 s (EBADF); after = `Finished` in 6.1 s. Unit-cache suite 62.8 s (26 tests).

## Official proof
`bun repo --tool=bazel run @semio-tech/print-rs:build --skip-cache` (wrap-g-tools/official-print.sh), started 14:27, exit 0, wall 6300 s of which about 2380 s capacity queue (3-4 peer runs ahead, 16 GB-style floor), graph generation 562 (90 projects refreshed in 616 s, my unit-cache edit), then `INFO: Build completed successfully, 5 total actions` (Bazel elapsed 944 s, critical path 936 s; the cold Rust lane compiled the workspace including the `libc` build script; 0 occurrences of EBADF in the log). Peak RSS of the single largest process 6.2 GB (`/usr/bin/time -l`), peak memory footprint 3.9 GB. Lane2-g's failing run was the same command (14 min queue + 15 min build, died in libc).

## Not done / residual
- Build scripts under the build-script cache get no jobserver tokens from Cargo (the stand-in drops the descriptors); correct but not pooled. A follow-up could hand the verified pair to `runReal` the same way.
- Not run on Linux/devcontainer/Windows: the stand-in is POSIX `sh` (dash on Debian: only `${##}`, `${%}`, `case`, subshell redirects are used), Windows uses the native executable with no descriptors.
