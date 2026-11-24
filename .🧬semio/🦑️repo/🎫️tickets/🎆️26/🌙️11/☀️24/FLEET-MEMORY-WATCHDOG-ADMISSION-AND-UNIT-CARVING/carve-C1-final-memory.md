# C1: where a component final's memory goes, and which per-unit setting bounds it

Date 2026-11-24. Scripts in this folder: `carve-C1-run.sh` (guarded, sampled single run), `carve-C1-telemetry.py` (read-only analysis of the driver telemetry and the cargo closure). No source, manifest or config file was edited.

## 0. Result in one paragraph

**The cad final could not be measured on this machine, and no final can be rebuilt "final-only" here.** Main's Rust backing (`.cache/b/r/fixed.dmg`, 43 GB image, 16.5 GB physical) is not attached and refuses to attach (`diskutil image attach ... exited with status 1`, "Failed to mount the volume", reference d87380fb), so there is no warm dependency set for main's checkout path; play's backing (`/Users/ueli/.sinc/play`) holds no rlibs (its lane keeps only 3099 `.rcgu.o` leftovers and the content-addressed unit cache stores wall and peak numbers, not artifacts) and is off limits. A cold dependency build of cad is 1041 units (911 workspace crates plus 130 external), the smallest hub final is 552 units, so per the task rules I stopped and report the exact command and cost (section 4). What I could establish is the exact set of codegen settings in effect (section 1), the measured peak/wall/CPU behaviour of every final from the driver telemetry (section 2), and a tested harness that produces every number the task asks for as soon as a warm final exists (section 5). Section 6 ranks the settings by what the evidence supports; it is a hypothesis list, not a measurement, and is marked so.

## 1. Facts: what is in effect for the cad final today (read from the repo, not guessed)

| Setting | Value | Source |
|---|---|---|
| profile selected for a wasm final | `release-wasm` (the `wasm-component` role maps to `component`; prototype uses `prototype-component`, same numbers) | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🎚️mode/🦀️profile/🧬️schema/🔣️.json` lines 431-437, 453-458 |
| opt-level | `s` | same |
| codegen-units | 16 | same |
| lto | `thin` (an upstream-bitcode ThinLTO across the whole dependency closure, since the final is a `cdylib`) | same |
| debuginfo | 0 (nothing to drop) | same |
| incremental | false | same |
| strip | `symbols` for role component; trim-paths `object`; debug-assertions and overflow-checks off in release | `🎚️mode/🦀️profile/🟦️.ts` line 36 |
| panic | preserved from the manifests (`preserved` list) | schema line 413-420 |
| front-end threads | Cargo gets `-Z threads=0` (`.cargo/config.toml`, `[target.wasm32-wasip2]` repeats it); the wrapper rewrites it to `1` for every `cdylib` final whatever the grant | `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/🦀️.rs` `frontend_tokens` (line 456) |
| back end | `-Z no-parallel-backend` appended only when the run was admitted exactly ONE job (`serial_backend`, line 462); with a larger grant the CGU and ThinLTO workers take jobserver tokens | same |
| linker args | `--max-memory=536870912`, `-zstack-size=8388608` (shadow stack) | `.cargo/config.toml` `[target.wasm32-wasip2]` |
| share-generics | not set; rustc's default is on for opt-level `s` (to be confirmed with `-Z print-mono-items`, section 5) | rustc default |
| how the pass builds it | `cargo build --locked -Z trim-paths --offline --manifest-path 🌎️hub/Cargo.toml -p <crate> --lib --target wasm32-wasip2 --profile release --keep-going --message-format=json -j8 --config profile.release.<key>=<value>...` with `RUSTC_WRAPPER=semio-build-scheduler`, `CARGO_TARGET_DIR`/`CARGO_BUILD_BUILD_DIR` = the lane, `CARGO_UNSTABLE_FEATURE_UNIFICATION=true`, `SEMIO_BUILD_MODE=release`, `SEMIO_COMPILER_PARALLELISM=<grant>`. The effective `--config` set was read off a peer's live cargo argv (a scratch build of `semio-hub-stdio-binary-component`): opt-level "s", debug 0, codegen-units 16, incremental false, lto "thin", strip "symbols", trim-paths "object", build-override opt-level 0 / codegen-units 256 | `🧩️component-pass/🟦️.ts` lines 149-161, 517-523; profile rendering `🦀️profile/🟦️.ts` line 36-38 |

## 2. Measured behaviour of finals (play driver telemetry, read-only: `carve-C1-telemetry.py` on `/Users/ueli/.sinc/play/.cache/b/r/m/components/build-telemetry/units.jsonl`)

| final (crate, target) | builds | peak MiB min / median / max | wall s | CPU/wall | output MB |
|---|---|---|---|---|---|
| framework_editor (unknown-unknown) | 8 | 325 / 383 / 589 | 10-91 | 1.43 | 4 |
| framework_surface | 8 | 507 / 548 / 674 | 16-117 | 1.38 | 6 |
| os_node_graph | 8 | 1156 / 1229 / 1347 | 38-268 | 1.21 | 8 |
| os_flow | 9 | 1704 / 1857 / 1969 | 40-245 | 1.27 | 7 |
| hub_dag (wasip2) | 1 | 3291 | 2183 | 0.95 | 211 |
| hub_puzzle | 4 | 2869 / 4150 / 6148 | 1078-1287 | 0.90 | 93 |
| hub_cad (wasip2) | 1 | 6550 | 2277 | 0.97 | 224 |

Linear fit over the 39 rows: peak = 983 MiB + 23.1 MiB per output MB. Closure sizes from `cargo metadata --filter-platform wasm32-wasip2` (hub workspace): cad 1041 crates (911 workspace), puzzle 1192 (1062), flow 899 (769), smallest hub final stdio_media 552 (518).

What this says, with no measurement of mine mixed in:
1. cad and dag are single-threaded to the end (CPU/wall 0.95-0.97 over 36-38 minutes), i.e. the wrapper's one front-end thread and serial back end are in effect and the 2277 s is one thread working through the whole closure. More threads cannot lower this peak (the wrapper comment already measured 1 thread 2.9 GB against 4 threads 5.4 GB on a small crate).
2. Output size does not predict the peak among the two 200+ MB wasip2 finals: dag 211 MB peaks at 3.3 GB, cad 224 MB at 6.55 GB (2x). So it is not "bytes written" but something per-input (LLVM IR or bitcode of the closure that the final loads and keeps) that differs.
3. puzzle's own four builds range 2.9 to 6.1 GB at equal output (92.5 to 92.7 MB), all at CPU/wall about 0.9. Same source, same flags, 2x peak: a scheduling-dependent component (order in which ThinLTO imports are materialised, or concurrent module buffers) sits on top of a floor of about 2.9 GB. The cad and dag peaks are single samples, so the cad floor is unknown and could be well below 6.55 GB.

## 3. Where the memory goes (what the evidence supports; not measured by me)

The 2x variation at equal output and flags (puzzle) and the dag/cad 2x at equal output size both point at the ThinLTO stage of the final, not at wasm-ld and not at front-end monomorphization: with `lto=thin` on a `cdylib` rustc reads the embedded bitcode of every upstream rlib (1041 crates here, each compiled at 16 CGUs), builds the combined summary index and then optimises/imports module by module; all serialized upstream modules and the index are resident from the start of that stage and only the per-module working set varies with schedule. The final crate itself (a "holding only its entry point" crate per its manifest) has almost nothing to monomorphize, so the section 4 split question (final's own generic instantiations) is expected to be answered "no". Both statements are predictions to be confirmed by `-Z time-passes` (the pass table in section 5), and I did not confirm either.

## 4. Why nothing was measured, the exact blocked command and its cost

Facts found:
- `.cache/b/r/m` of main is an empty mount point. `repo cargo --store=shared --configuration=release --parallelism=1 -- tree ...` (the repo's ingress, which opens the declared backing) failed at the attach: `/usr/sbin/diskutil image attach --nobrowse --mountOptions owners --mountPoint .../.cache/b/r/m .../.cache/b/r/fixed.dmg exited with status 1`. The attach also moved the empty stranded entry `cargo/home` (created 02:14 by some peer) to `.cache/b/r/m-stranded-1795485957802`; nothing references it. Recovery of this image is the item of memory note "Native Cache Images APFS & Recovery"; not attempted.
- `hdiutil info` shows every other fixed image (d, g, l, n, o, p, t of main; d, l, o, p, r, t of play) attached; only main's `r` is missing.
- Play's lane (`u/l/p8eca84`, 17.4 GB) holds no rlibs; the unit cache `u/c/<crate>` holds `.peak` and wall numbers only. A copy of it would also exceed the 15 GiB scratch limit and touch play's volume (3 GB free, a `ditto` backup of it is running right now).
- A peer is running a cold `semio-hub-stdio-binary-component` release build in its own scratch at -j8 (started about 03:00 minus 30 min); its tree is the smallest-sized hub final and not mine to use.

The command that would produce the warm baseline (cold cost: 1041 units; the final alone took 2277 s single-threaded, and the cold dependency half is longer than the 30+ minutes the peer's 552-unit build has taken so far; plan about 60-90 minutes wall at -j8 on a loaded Mac and several GB of lane disk, with up to a dozen concurrent rustc at 0.4-1.9 GB each):

```
cd /Users/ueli/Documents/semio
repo cargo --store=shared --configuration=release --parallelism=8 -- build --locked -Z trim-paths --offline \
  --manifest-path "🌎️hub/Cargo.toml" -p semio-hub-cad-component --lib --target wasm32-wasip2 --profile release \
  --config profile.release.opt-level='"s"' --config profile.release.debug=0 --config profile.release.codegen-units=16 \
  --config profile.release.incremental=false --config profile.release.lto='"thin"' --config profile.release.strip='"symbols"'
```

I did not run it: more than a hundred crates is the task's stop condition, and the backing it needs does not mount.

## 5. The measurement matrix, ready to run once a warm cad (or any hub final) exists

Each row is final-only: `cargo rustc` with the same lane, `--config` set and wrapper as the baseline command in section 4, so dependency fingerprints match (extra rustc flags after `--` apply to the final only and the wrapper still rewrites threads and adds the serial back end); `touch` is unnecessary because a changed rustc argument invalidates only the final.

```
BASE='repo cargo --store=shared --configuration=release --parallelism=1 -- rustc --locked --offline --manifest-path 🌎️hub/Cargo.toml -p semio-hub-cad-component --lib --target wasm32-wasip2 --profile release <the --config set above> --'
export CARVE_WASM=<lane>/wasm32-wasip2/release/semio_hub_cad_component.wasm     # sha256 and bytes are recorded per run
carve-C1-run.sh base   $OUT/base   -- ${=BASE}
carve-C1-run.sh tp     $OUT/tp     -- ${=BASE} -Z time-passes -Z time-llvm-passes     # which pass owns the peak; stderr in $OUT/tp.stderr
carve-C1-run.sh cgu1   $OUT/cgu1   -- ${=BASE} -C codegen-units=1                     # also 4, 16 (= base, control), 64
carve-C1-run.sh nopb   $OUT/nopb   -- ${=BASE} -Z no-parallel-backend
carve-C1-run.sh thr1   $OUT/thr1   -- ${=BASE} -Z threads=1
carve-C1-run.sh lto0   $OUT/lto0   -- ${=BASE} -C lto=off                              # changes output
carve-C1-run.sh lto0b  $OUT/lto0b  -- ${=BASE} -C lto=off -C embed-bitcode=no          # changes output
carve-C1-run.sh optz   $OUT/optz   -- ${=BASE} -C opt-level=z                          # information only, changes output (also 1, s as control)
carve-C1-run.sh sg     $OUT/sg     -- ${=BASE} -Z share-generics=yes                   # and =no
carve-C1-run.sh mono   $OUT/mono   -- ${=BASE} -Z print-mono-items=lazy                # then wc -l; -Z dump-mono-stats for CGU counts
```

Also record: `rustc --print cfg`-style facts of the baseline (`-C` flags are already printed by `cargo -v`), the dependency rlib count and total bytes (`ls -l` of the lane `deps` for the closure; 1041 units expected), and per-process peaks (`footprint` of `rustc` versus `wasm-component-ld`), which `carve-C1-run.sh` records. The harness refuses to start below 9 GB of free+inactive+speculative memory and kills its own tree below 3 GB (it logs `killed=1`). It was exercised only on a trivial rustc compile (4 s, rss 86 MB, footprint 26 MB), enough to prove the sampling, the guard arithmetic and the output row; its numbers for a real final are not available.

## 6. Recommendation (ranked by expected memory saved; every row is untested, so "expected" is reasoning plus the section 2 data)

| rank | change | memory effect expected | semantics / output | wall cost | where declared |
|---|---|---|---|---|---|
| 1 | Measure first: the matrix of section 5 on `semio_hub_dag_component` or `semio_hub_puzzle_component` (smaller closures that already behave like cad, 3.3 GB and 2.9-6.1 GB) if cad's cold build is not affordable | decides every row below | none | one cold pass | this ticket |
| 2 | `-C lto=off` (thin-local) on component finals only: the final then stops loading the whole upstream bitcode set; expected to remove the floor that scales with closure size | largest expected, to be measured | output bytes change (no cross-crate inlining of dependencies), behaviour not; cost: slower and larger wasm, contradicts "maximum performance" | wall shorter | a `release-wasm-final` row in the profile schema `🎚️mode/🦀️profile/🧬️schema/🔣️.json` `settings` (the `component` role already selects its own row) |
| 3 | Make the wrapper decide the final's LTO from the admitted grant: when the run's prediction exceeds the declared `unitCeilingMegabytes`, append `-C lto=thin -C codegen-units=<n>` unchanged but add `-Z no-parallel-backend` for any grant (today only for grant exactly 1) so ThinLTO workers cannot add concurrent module buffers (the 2x spread of puzzle at equal output) | removes the scheduling-dependent half (puzzle 6.1 to 2.9 GB, if the spread is the concurrency) | output identical (the wrapper documents the flag only schedules) | wall up for a lone final on idle cores | wrapper `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/🦀️.rs` `serial_backend` plus its test `🧪️tests/🧵️frontend-threads/🟦️.ts` and Python oracle |
| 4 | `-C codegen-units=1` or `4` on finals | CGU count mainly changes how many modules are optimised at once; with `no-parallel-backend` already serial the saving is expected small; 1 raises single-module IR size | output changes (inlining layout) | longer | profile row as rank 2 |
| 5 | `-Z threads=1`, `-C debuginfo=0`, `-C embed-bitcode=no` of the final | none expected: threads are already 1 and debuginfo already 0 for finals; embed-bitcode of the final only affects its own (absent) rlib | identical | none | already in effect, no change needed |
| 6 | Split the component by artifact family (design.md section 4, step 3) | divides the closure each final links; fixed by the closure counts above (cad 1041, puzzle 1192, flow 899 units against 552 for the smallest hub final) | changes the plugin structure | build-graph work | crate layout of `🌎️hub/🧩️compositions/📐️cad` |

What peak to expect: unknown until section 5 runs. The only bracketing numbers are measured ones: 3.3 GB for the dag final at the same output size as cad, and 2.9 GB as the lowest of four puzzle builds. If rank 3 removes the schedule-dependent half and rank 2 the closure-scaled floor, a 4 GB unit ceiling is plausible for dag/puzzle and uncertain for cad; I do not claim it for cad.

## 7. What could not be measured

Everything in sections 3, 5 and 6 beyond the telemetry: no baseline run, no `-Z time-passes` pass table, no variant, no sha256 or output-size comparison, no mono-item or CGU counts, no dependency rlib byte total. Cause: no warm dependency set for main's checkout (backing fails to attach, section 4) and a cold 1041-unit build is outside the task's stop rule.
