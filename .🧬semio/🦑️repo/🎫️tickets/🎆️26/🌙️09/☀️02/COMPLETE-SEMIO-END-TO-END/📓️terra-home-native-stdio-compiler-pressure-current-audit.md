# Home Native Stdio Compiler Pressure: Current Audit

Status: read-only source and host-process audit, 2026-09-05. No build was started, no existing process was changed, and no product source was modified. This is a bounded preflight packet for the existing Home native gate, not a qualification receipt.

## Decision

Do not start another native compilation while the current fleet is occupying essentially all swap. For the next invocation of the existing Home gate, first scope Rustc to **one internal compilation thread** and keep the 256 MiB stack override out of the compiler. Then make the structural repair: the native OS host must select a narrow DWG codec surface rather than forcing Stdio's 36-artifact catalog into every native Space test.

`CARGO_BUILD_JOBS=1` is already present, but it only limits concurrent Cargo units. It does **not** override the active Rustc `-Z threads=8` setting or the inherited `RUST_MIN_STACK=268435456` setting.

## Observed live state

| Item | Current evidence | Consequence |
| --- | --- | --- |
| Exact Home entrypoint | [launch configuration](</Users/ueli/Documents/semio/.vscode/launch.json:5081>) runs `bun nx run @semio-tech/space-plugin:home-directory-identity-rows-native-check --skip-nx-cache`; it fixes the ticket target and sets Cargo jobs to one at [lines 5086–5093](</Users/ueli/Documents/semio/.vscode/launch.json:5086>). | This is the only native invocation to scope; it need not alter general builds. |
| Active compiler | `rustc` PID **31509**, child of Cargo 29957 and the Space script, had been alive 51 minutes. Its exact command compiles `semio_s_plugin_stdio` with both `full-artifact-catalog` and `home-io`, into `public-member-open-sol-target`, with `-Z threads=8`. | A narrowly selected Home row law is paying for native full-catalog codegen. |
| Rustc parallelism | A process-thread snapshot showed **11 Rustc thread rows**, one runnable and the remainder sleeping. The process had only ~18–31% CPU at successive samples. | Eight internal compiler workers are not delivering eight useful active cores here. |
| Fleet saturation | The snapshot contained **33 live rustc** and **32 Cargo** processes: ~4.70 GiB aggregate Rustc RSS and ~180% aggregate Rustc CPU, on a 10-CPU / 32 GiB machine. | This is fleet contention and background throttling, not a single Cargo job using every core. |
| VM state | `vm.swapusage`: **64.82 GiB used of 66.56 GiB**, only 1.74 GiB swap free. `vm_stat` showed 11.1 GiB active + inactive, 3.42 GiB wired, and 16.3 GiB physical compressor occupancy. The point-in-time `memory_pressure` report still said 35% memory free and zero throttled pages. | Treat this as severe accumulated swap/paging exposure, not proof that PID 31509 alone consumes all memory. New full compiles are the avoidable risk. |
| Scheduler | An independently running PID 5361 repeatedly invokes `taskpolicy -B` over every Cargo and Rustc process every 15 seconds. PID 31509 showed Darwin priority `31T`. | Low observed CPU is consistent with imposed background policy as well as codegen waits; this audit makes no priority/process recommendation. |
| Per-process stack input | The active Cargo/Rustc environment includes `RUST_MIN_STACK=268435456`. The Home script injects it at [line 203](</Users/ueli/Documents/semio/✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/📜️script.ts:203>), and the exact runner uses the same environment for build, list, and native stages at [lines 1943–1949](</Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts:1943>). | It is not just a law-process setting: Cargo and Rustc inherit it. |
| Target pressure | The pinned ticket target is 15 GiB, of which 11 GiB is `debug/incremental`. | A new flag fingerprint will rebuild into this target. Do not change the environment of the live run. |

The local Rust standard library states that `RUST_MIN_STACK` specifies the stack size of spawned Rust threads, while the main thread is excluded ([`thread/mod.rs:131–139`](</Users/ueli/.rustup/toolchains/nightly-2026-07-07-aarch64-apple-darwin/lib/rustlib/src/rust/library/std/src/thread/mod.rs:131>)). It is read from the process environment by the thread runtime ([`lifecycle.rs:37–40`](</Users/ueli/.rustup/toolchains/nightly-2026-07-07-aarch64-apple-darwin/lib/rustlib/src/rust/library/std/src/thread/lifecycle.rs:37>)). Thus the observed ten spawned Rustc-side threads can reserve up to roughly 2.5 GiB of stack address space at 256 MiB each. Reservation is **not** proof that all of it is resident, so it must not be counted as PID 31509 RSS; it is nevertheless an avoidable compiler-side virtual-memory multiplier.

The root Cargo configuration supplies `sccache` and `-Z threads=8` globally ([`.cargo/config.toml:1–3`](</Users/ueli/Documents/semio/.cargo/config.toml:1>)). This exact launch clears both wrapper variables ([`launch.json:5091–5093`](</Users/ueli/Documents/semio/.vscode/launch.json:5091>)), so no current Stdio compiler is using the configured cache wrapper. Re-enabling it is not the recommended fix for this cold, feature-heavy compile.

## Correctly scoped next-run settings

### 1. Cap Rustc internals through the build configuration slot

Add only this environment entry to the existing native launch, after the active run has terminated naturally:

```json
"CARGO_BUILD_RUSTFLAGS": "-Z threads=1"
```

This is safer than `RUSTFLAGS`/`CARGO_ENCODED_RUSTFLAGS`: Cargo documents that those two override *all* target flags before `target.<triple>.rustflags`, while `CARGO_BUILD_RUSTFLAGS` supplies the `[build]` configuration slot ([local Cargo config reference](</Users/ueli/.rustup/toolchains/nightly-2026-07-07-aarch64-apple-darwin/share/doc/rust/html/cargo/reference/config.html:714>), [precedence](</Users/ueli/.rustup/toolchains/nightly-2026-07-07-aarch64-apple-darwin/share/doc/rust/html/cargo/reference/config.html:722>)). Consequently:

- macOS and Windows native builds replace the root `build.rustflags = ["-Z", "threads=8"]` with one internal compiler thread;
- existing Linux target-specific `mold` flags stay authoritative rather than being accidentally erased;
- existing wasm32-wasip2 target flags (including its required memory cap and explicit eight threads) also stay authoritative.

Tradeoff: an otherwise idle machine may compile this one crate more slowly. Under the observed background scheduler and one-runnable-thread snapshot, it reduces contention/reserved stacks without removing a useful parallel execution path. It changes Cargo fingerprints, so it is for the **next** build only; never attach it to PID 31509's live target.

`CARGO_INCREMENTAL=0` is an optional recovery setting only for a newly provisioned ticket target if the reduced-thread build still causes paging. It prevents further multi-gigabyte incremental output, but makes clean builds slower and does not shrink the current 11 GiB directory. It should not be the default first change.

### 2. Give the 256 MiB stack only to the test executable

The three Space Home exact-law paths all currently inject the same compiler-inherited stack setting ([`space script:66`](</Users/ueli/Documents/semio/✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/📜️script.ts:66>), [`:159`](</Users/ueli/Documents/semio/✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/📜️script.ts:159>), [`:203`](</Users/ueli/Documents/semio/✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/📜️script.ts:203>)). Extend `ExactCargoLawOptions` with an explicit `nativeEnv`, then use:

```text
build/list environment = options.env + CARGO_TARGET_DIR
native-law environment = build/list environment + options.nativeEnv
```

Move `RUST_MIN_STACK=268435456` from each `env` object to its `nativeEnv`. `capture` already distinguishes `build`, `list`, and `native` at [lines 1943–1954](</Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts:1943>), so this is a small schema-level runner boundary: no shell wrapper, no ambient mutation, and no change to the native law's required stack.

Add a TypeScript retained-port test that records the process environment for one build/list/native sequence: build and list must not contain `RUST_MIN_STACK`; native must contain the exact explicit value. This prevents reintroducing compiler stack inflation elsewhere.

## Why the Home native graph is compiling all Stdio

The selected Space dependency is already narrow: `semio-s-plugin-space` selects Stdio `home-io` ([`Cargo.toml:16`](</Users/ueli/Documents/semio/✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/Cargo.toml:16>)), and the source gate defines that as CSV/JSON/XLSX/ZIP plus binary/deflate/txt/xml. The actual Home gate only builds the plugin-host law and three pure Home table-row rendering laws ([`space script:201–218`](</Users/ueli/Documents/semio/✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/📜️script.ts:201>)).

Feature unification happens later. Space's native target selects `semio-framework-os/os-host-full` ([`space Cargo.toml:51–52`](</Users/ueli/Documents/semio/✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/Cargo.toml:51>)); the OS host then has an unconditional non-WASI Stdio dependency with `full-artifact-catalog` ([`host Cargo.toml:49–54`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/Cargo.toml:49>)). That makes the active Stdio command contain both features.

The host's direct Stdio uses are only DWG conversion types and functions—imports at [host lines 3043–3050](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🖥️host/🦀️.rs:3043>), SVG-to-DWG at [3087–3095](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🖥️host/🦀️.rs:3087>), and two mesh-DWG bridge functions at [3270–3288](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🖥️host/🦀️.rs:3270>). There is no source-backed basis for this Home-row gate to require the other 35 artifact trees.

The scale is structural, not a generic cache issue:

| Native Stdio surface | Current source scale |
| --- | --- |
| full catalog | 36 artifacts; 3,598 Rust files; 527,951 Rust source lines; root glue is 14,118 lines / 4,690 path mounts |
| DWG tree actually used by OS host | 64 Rust files / 20,359 source lines |
| especially expensive full-only examples | Semio artifact: 1,208 files / 173,347 lines; glTF: 382 files / 46,214 lines |

The full catalog also carries a known compile-time const-evaluation hotspot: the root explicitly records that glTF's `dsl::Mutations` descriptor roster has 120 leaves and exceeds the normal const-eval step budget ([`Stdio root:13–19`](</Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/🦀️.rs:13>)); its 120-variant derived mutation enum is at [`gltf mutations:128`](</Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🦀️.rs:128>). This is a real long-term full-catalog hotspot, but it is not the first repair: profiling it before the dependency fan-out is removed would measure work the Home gate should not request.

## Clean catalog-preserving repair

Introduce a Stdio feature named `native-dwg-codec` alongside `home-io` and `full-artifact-catalog` in [`Stdio Cargo.toml:31–35`](</Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/Cargo.toml:31>). Mount the existing DWG module when either `full-artifact-catalog` **or** `native-dwg-codec` is selected, replacing only the outer gate at [`Stdio root:3692–3696`](</Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/🦀️.rs:3692>). Leave `plugin-root = ["full-artifact-catalog"]` unchanged: Stdio's shipped plugin and all complete-catalog consumers retain the complete catalog.

Then change the OS host's native dependency from `full-artifact-catalog` to `native-dwg-codec`. This preserves every host DWG behavior while the Home-native graph becomes `home-io + native-dwg-codec`, rather than `home-io + full-artifact-catalog`. Do **not** make the feature a test-only alias or remove the host's real DWG behavior.

Update the existing Stdio `home-io-surface` oracle to declare this host-only narrow surface and assert all of the following:

1. Space still selects exactly `home-io`.
2. OS host selects exactly `native-dwg-codec`, not `full-artifact-catalog`.
3. only the DWG outer module admits `native-dwg-codec`; every other full-only module still requires `full-artifact-catalog`.
4. `plugin-root` still selects full catalog and the complete descriptor still has all 36 artifacts.
5. Hub and GIS retain their current complete-catalog selections where their actual execution surfaces require them.

Native acceptance must include both existing real host DWG laws—`media_export_raster::tests::svg_to_dwg_round_trip_produces_a_polyline` and `media_export_raster::tests::mesh_dwg_registrar_round_trips_a_box` ([host lines 4304–4320](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🖥️host/🦀️.rs:4304>))—plus the existing Home row laws. A source oracle alone cannot prove the new feature admits the actual DWG types.

Expected tradeoff: this changes feature fingerprints once and recompiles a cold target. It reduces the steady native Home closure by roughly the full-only 500k-line catalog, without hiding or deleting any Stdio artifact and without weakening native DWG behavior. Only after this graph separation is qualified should the glTF descriptor derive be redesigned as a generator-produced, schema-checked static roster; that second change must preserve its 120-leaf enumeration and exact mutation codec laws.

## Exact next step

After the current native processes finish, use the same registered launch entry at [`.vscode/launch.json:5081`](</Users/ueli/Documents/semio/.vscode/launch.json:5081>) with the scoped `CARGO_BUILD_RUSTFLAGS` setting. Do not start a parallel full-catalog compile. Implement and source-gate the `native-dwg-codec` boundary before the next cold Home native acceptance. No passing build is claimed by this audit.
