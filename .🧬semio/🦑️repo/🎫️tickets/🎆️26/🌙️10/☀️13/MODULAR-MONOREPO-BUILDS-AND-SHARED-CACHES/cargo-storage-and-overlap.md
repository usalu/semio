# Cargo Storage and Actual Overlap Evidence

## Actual Storage Precedence Repair

Actual `cargo-overlap-prototype2.log` Cargo compiler-artifact paths escaped the env-only private leased build directory when `.cargo/config.toml` was supplied through `--config`. The actual build script landed in canonical `cache/cargo/build/prototype/build/shared/...`, while the requested private build root held no `.cargo-build-lock`. Private uplifted target outputs were honored. This was a physical storage/lease identity defect, not merely an argument formatting issue.

The canonical composer now appends `--config build.build-dir=<JSON-quoted original absolute path>` after the authored configuration and before the Rust compiler `--` delimiter. Required `buildDirectory` comes from the original artifact policy. Both component-pass compiler/unit-graph invocations and noncompiling relay invocations forward it; there is no new alias or renewed budget. Actual private artifact paths and physical profile lock in prototype3 and later match that explicit original leased store.

The source golden law was actually red before this repair (`cargo-storage-source-red.log`) and green afterward (`cargo-storage-source-green1.log`). Expanded source qualification `cargo-storage-source-green2.log` passed one test, 16 assertions, Nx 131ms, including prior conflicting storage, existing configuration, compiler delimiter, spaces, quotes and backslash paths, plus independent `@iarna/toml` agreement with Bun TOML. Latest `cargo-storage-source-green3.log` passed 2 tests / 24 assertions, Nx 215ms, including both JSONC launch inventories. SDK agent was informed that production precedence sources are ready before freezing.

## Actual Pinned Cargo Probe

Both modes use actual `cargo 1.99.0-nightly (3efb1f477 2026-07-17)`, pinned `nightly-2026-07-20`, repository fine-grain locking/new build directory layout, std-only authored workspace packages, original finite 180000ms control, original cancellation/progress, canonical capacity/shared-store/profile admission and private uplifted targets. Build scripts report actual `NUM_JOBS=1`. The strict language-neutral corpus agrees with Ajv and independent SQLite readers. Independent raw Cargo run outputs agree with captured binaries (`cohort_a:137:1`, `cohort_b:137:1`). No project-wide native Cargo build was run by this probe.

| Actual receipt | Nx duration | Tests/assertions | Cold builders ms | Same target warm ms | Warm rustc | Logical/physical bytes |
|---|---:|---:|---|---|---|---|
| `cargo-overlap-prototype7.log`, `cargo-overlap-prototype.json` | 4.6s | 2 / 97 | 710, 1057 | 30, 29 | 0, 0 | 12756515 / 13824000 |
| `cargo-overlap-release1.log`, `cargo-overlap-release.json` | 21.2s | 2 / 97 | 3809, 7546 | 29, 30 | 0, 0 | 6588025 / 7024640 |

Both actual same-profile admissions overlap (maximum active 2). Both warm original-target invocations emit all units fresh and no Rust compiler execution. Each invocation retains its own final target output; the whole tiny workspace, intermediates and oracle outputs remain below the 100MB authored bound and are removed in the owned finalizer.

Both modes physically prove live Cargo blocks exclusive `std::fs::File::try_lock()` and Python `fcntl.flock(LOCK_EX|LOCK_NB)` on the actual private `b/<profile>/.cargo-build-lock`. An exclusive std holder blocks Python, and an exclusive Python holder blocks std. After Cargo stops, std exclusive acquisition succeeds. These are actual macOS observations; Linux/Windows interoperation is not qualified here. Prototype4 was the first primary std-versus-live-Cargo receipt; prototype6 first qualified both Python directions. Parent received these physical results before its pruner guard work. Root lease/pruner APIs were not edited.

## Material Limits

Cold concurrent shared library compilation invokes rustc twice, even though both artifact messages name the identical shared library output. This is not cold compilation reuse. A new empty private uplifted target for `cohort_a` causes another binary rustc invocation: 153ms prototype / 2887ms release; dependencies are fresh. `cohort_b` happened to reuse all units in the subsequent empty target (34ms both). Accordingly, zero-rustc warm evidence is specifically for the same original private target, not an arbitrary new target or full product warm build. Prototype5/6 earlier red warm expectations remain retained as evidence.

## Exact Changed Paths

- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🟦️.ts`: canonical explicit build directory composer.
- Same owner `🧩️component-pass/🟦️.ts` and `🌊️relay/🟦️.ts`: all actual composer callers forward original storage.
- `🧰️framework/🔨️modules/🏃️process/🏗️build/🧪️tests/🏗️native/🟦️.ts` and `🧫️fixtures/🏗️native/🔣️.json`: source golden cases and TOML oracle.
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🔒️lease/🧪️tests/🏎️overlap/🟦️.ts`, `🧬️schema/🔣️.json`, `🧫️fixtures/🔣️.json`: actual Cargo overlap, locks, correctness and warm measurements.
- Ticket `headroom/📜️script.ts`, `headroom/project.json`: canonical `cargo-overlap` and `cargo-storage-source` routes/configurations.
- `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc`: actual overlap prototype/release and storage source acceptance entries.

Permanent scripts remain in existing 📜️script.ts owners. No runtime external dependencies or Git/worktree mutations were introduced. The existing full-build goal remains open.


## Persistent Compiler Recipe Target Reuse

The next feature is installed in the genuine native artifact owner. A recipe identifies reusable Cargo target state, not source-equivalent final artifact bytes. It includes actual Cargo/rustc identities, manifest/cwd, command and all compiler arguments, profile/storage, observed config/manifest/lock/toolchain files and declared compiler environment. Ephemeral original caller envelopes and remaining budgets stay forwarded to every child and do not name the target slot. Every invocation still runs actual Cargo; Cargo validates changed sources and declared build-script environment.

The first-party `🎯️recipe` owner retains the canonical shared build-store/profile lease plus exclusive exact recipe target admission. The slot lives under original `buildDirectory/t/<32 hexadecimal digest characters>`; its receipt retains the complete canonical recipe, and mismatched receipt/collisions are refused. Actual compiler build-dir arguments must physically match the original leased store. Input observation rechecks before compile. The artifact producer retains target/store admission through primary/dependency copies, source record, staging and completed provenance; compiler capacity is released at compiler stop. Thus a later same-recipe producer waits until current capture completes. Captured/pinned outputs remain invocation-owned and immutable after the next target mutation. No parent lease/pruner files changed.

TDD initial canonical gate `cargo-target-recipe-red3.log` actually failed because the recipe implementation did not exist. Earlier red1/red2 were registration/path setup failures and do not count as semantic evidence. Helper/custody green1 passed 2 tests / 17 assertions in 402ms. Its independent SQLite/FS oracle verifies waiting→capture→release→next-producer ordering and that a later target overwrite preserves previously captured bytes. Latest tests also refuse exhausted original admission before slot birth. Strict defining TypeScript compilation and both launch inventories actually passed in prototype3.

| Actual receipt | Scope | Nx duration | Tests/assertions | Warm reuse | Physical bytes |
|---|---|---:|---:|---|---:|
| `cargo-target-recipe-actual-prototype1.log` | Existing two-builder real Cargo probe plus persistent recipes | 5.3s | 2 / 126 | New same-recipe requests use retained target, all fresh/zero rustc | <100MB |
| `cargo-target-recipe-actual-release1.log` | Same real Cargo probe, explicit release | 22.1s | 2 / 129 | New same-recipe requests 32ms / 28ms, all fresh/zero rustc | 7901184 |
| `cargo-target-production-prototype2.log` | Actual canonical artifact producer | 2.4s | 3 / 29 | 195ms, same target, zero rustc | 2342912 |
| `cargo-target-production-release2.log` | Actual canonical artifact producer, explicit release | 8.6s | 3 / 29 | 400ms, same target, zero rustc | 1830912 |
| `cargo-target-production-prototype3.log` | Above producer plus strict defining types and both launch inventories | 3.9s | 4 / 37 | 300ms, same target, zero rustc | 2342912 |
| `cargo-target-fixtures2.log` | Existing selected capture/cancel/nonzero/empty/raw-component/expired original remainder laws | 1.2s | 6 / 20 | All actually run | No actual Cargo |

Both real production modes then change the source file: actual Cargo reports nonfresh and executes rustc, the newly published binary prints `recipe-changed`, and the prior first-party pinned executable still prints `recipe-original`. This qualifies freshness and immutable acquired output, not a skipped-Cargo source cache. Complete product/native renderer compilation and its warm timing are still unqualified here. Cold concurrent shared-library duplication remains a separate material limitation.

`cargo-target-fixtures1.log` exposed queued lease sidecar paths exceeding the 256-character bound when fixture lease databases lived directly under long ticket output. The fixtures were hand-migrated to canonical `.cache/framework/leases` with their distinct original build identities, and their real pass is fixtures2. Custom controlled compiler fixtures now supply explicit authored compiler/toolchain identities instead of borrowing real Rust tools during synthetic cancellation. Temporary fixture directories remain ticket generated output; private real-probe roots are removed in their finalizers.

Additional exact paths:

- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎯️recipe/🟦️.ts`, `🧬️schema/🔣️.json`, `🧪️tests/🟦️.ts`, `🧪️tests/🧫️fixtures/🔣️.json`: recipe schema, original admission/custody implementation and portable/actual oracles.
- Genuine parent `🏗️native-build/🟦️.ts`: observed compiler recipe supplier, persistent target selection, original custody through complete invocation capture; custom compiler ports require explicit compiler/toolchain identities.
- Existing `🏗️native-build/🧪️tests/🟦️.ts`: every custom port caller migrated and authentic canonical fixture lease storage.
- Existing `🏗️native-build/🔒️lease/🧪️tests/🏎️overlap/🟦️.ts`: actual new-request recipe reuse, Cargo/rustc metadata and independent correct-output checks; live std lock assertion requires blocked.
- `🧰️framework/🔨️modules/🏃️process/📜️script.ts`: canonical `cargo-target-recipe` and `cargo-overlap` test suite registration.
- Ticket headroom script/project plus both launch files: original `cargo-target-recipe` prototype/release and selected fixture routes; launch orders 900.05748096/97/98.

Common production source readiness was sent to the SDK agent after actual both-mode production qualification. No broad native/release product compiler was launched in this scope. Persistent target state is deliberately retained; its future eviction remains part of the original store maintenance policy, with no eviction behavior claimed by this feature.

## Scheduling Does Not Fragment Persistent Targets

Root review found that the production artifact producer acquired CPU capacity before waiting for an identical compiler recipe and included its dynamically inserted CARGO_BUILD_JOBS in the target identity. A new actual Cargo regression first acquired a genuine one-core lease and then a genuine all-core lease. It failed on distinct physical target paths, proving the previous fixed-one-worker fixture missed the problem.

The producer now admits the exact recipe before reserving CPU capacity. Its schema-owned environment selection excludes owned storage overrides and scheduling-only jobs, while retaining semantic compiler flags, target/profile configuration, compiler wrappers and toolchain inputs. Scheduling values still reach the actual Cargo process, and Cargo always validates freshness. A second actual acceptance law holds the target recipe, observes the real producer queued there, and acquires a core while that producer waits. The original recipe/store remains held through artifact/provenance capture. Independent SQLite validates the environment partition.

Prototype final acceptance passed 5 tests/44 assertions in 4.0 seconds. The actual producer reused a target across changed admitted CPU counts, ran no Rust compiler on its warm build, rebuilt changed source, and preserved a previously pinned executable. An earlier prototype run measured warm capture at157ms. Release final acceptance also passed 5 tests/44 assertions, with actual warm capture192ms and zero Rust compiler execution. These measurements cover the tiny actual producer and do not qualify whole-workspace warm launch latency.
