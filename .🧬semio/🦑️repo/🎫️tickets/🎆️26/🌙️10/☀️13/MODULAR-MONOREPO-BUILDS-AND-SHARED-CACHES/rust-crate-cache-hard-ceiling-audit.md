# Rust Crate Artifacts and a Physical Cache Ceiling

Read-only audit on 2026-10-14. Source/configuration and compiler execution were untouched. The only authored output is this report. Read AGENTS.md, architecture.md, cache-headroom.md, storage-pressure-headroom.md, and cargo-storage-and-overlap.md. No build or test was run; prior qualification is attributed to those reports, not independently repeated here.

## Current Physical Evidence

Bounded `du -sk` inventories used 8–15-second per-root timeouts. The first whole Cargo scan timed out and establishes no total; separate build and target scans completed. Counts reflect concurrent activity and are observations, not an atomic repository snapshot. macOS `du` reports allocated blocks in KiB; APFS shared extents, clones and snapshots prevent interpreting their sum as exclusive disk reclaimability.

| Root | Allocated KiB | Approximate GiB |
| --- | ---: | ---: |
| `.🧬semio/🦑️repo/⚡️cache/cargo/build` | 162,050,900 | 154.54 |
| `.🧬semio/🦑️repo/⚡️cache/cargo/target` | 7,426,856 | 7.08 |
| `.🧬semio/🦑️repo/⚡️cache/nx` | 6,666,520 | 6.36 |
| `/Users/ueli/.cargo/registry` | 1,949,204 | 1.86 |
| `/Users/ueli/.cargo/git` | 6,120 | 0.006 |
| Cargo build `prototype` | 44,989,096 | 42.90 |
| Cargo build `release` | 1,221,284 | 1.16 |
| Cargo build `semio-cargo-logs` | 1,820 | 0.002 |
| Cargo build `semio-cargo-provenance` | 39,828 | 0.038 |

`df -k .` observed 27,514,892 KiB available (26.24 GiB), filesystem 98% full. Canonical `cargo/home` does not exist; Cargo defaults to the user home unless a caller supplies CARGO_HOME. Canonical build `t` did not exist at inspection; this does not mean recipe slots are unused, because actual policies can place buildDirectory below another root.

Top-level Cargo build inventory contains debug, prototype, release, wasm-prototype, wasm-release, wasm-dev, wasm32-unknown-unknown, wasm32-wasip2, tmp, semio-compiler-resources, semio-cargo-provenance/logs, and semio-trunk-provenance. Target contains debug/prototype/release plus resource/provenance directories. Cache root also contains e1/e1b/e2/e2b/e3b/e4, pe, edfix, compiler, cargo-preparation, build, trunk, hub, plug, at, arttest, tests, agents and many other domains. These are candidates for complete schema registration, not proven inactive/deletable roots. Historical reports identify private Cargo stores and copied baselines within several of them. This audit did not recursively census all private roots or millions of entries.

Additional mutable Rust locations include Cargo registry cache/source/index/git under the actual CARGO_HOME, toolchain state under actual RUSTUP_HOME, build-script OUT_DIR within compilation units, finalized/working incremental sessions, persistent recipe target slots, private invocation captures and immutable executable pins, product `dist/build/<mode>` outputs, and the Nx cached copies of those outputs. Only roots measured above have current physical numbers.

## Giant Crates Still Exist

Bun TOML parsed 587 non-ticket authored Cargo package manifests found by `rg --files --hidden` with node_modules, .git and ticket infrastructure excluded. A lexical source-module traversal followed explicit path attributes and simple external module declarations from each lib target. The table is an all-cfg textual closure estimate, including tests, not Cargo metadata or a compiler-validated active target closure. Inline module nesting, generated includes and Rust path rules need a real unit/source oracle before enforcing these counts; they nevertheless expose large source ownership rather than merely large directories.

| Package | Estimated files | UTF-8 bytes | Lines |
| --- | ---: | ---: | ---: |
| semio-s-artifact-bim-model | 2,905 | 10,307,460 | 187,329 |
| semio-framework-os-kernel | 505 | 10,119,327 | 150,402 |
| semio-framework-plugin | 359 | 8,357,258 | 117,425 |
| semio-s-artifact-energy-model | 1,741 | 6,943,937 | 132,208 |
| semio-s-artifact-architect-program | 1,274 | 6,414,568 | 90,390 |
| semio-framework-ui | 166 | 4,807,629 | 91,664 |
| semio-framework-os-infinite | 64 | 3,082,487 | 56,709 |
| semio-framework-os-kernel-db | 60 | 3,028,168 | 59,290 |

Exact manifest locations are respectively `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/📦️packages/🦀️rust/Cargo.toml`, `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/Cargo.toml`, `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/Cargo.toml`, `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/📦️packages/🦀️rust/Cargo.toml`, `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/📦️packages/🦀️rust/Cargo.toml`, `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/Cargo.toml`, `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/Cargo.toml`, and `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📦️packages/🦀️rust/Cargo.toml`.

OS kernel lib `🦀️.rs` directly mounts ../../🔨️modules/🗣️dsl/🦀️.rs and declares former standalone crate aliases (`store`, `spr`, `vcs`, `dsl`, `protocol` styles). Folder modules are therefore not independently reusable Cargo artifacts. A changed included source marks the enclosing Rust crate stale; incremental rustc may reuse internal queries, but cannot make that changed crate an Nx artifact hit. Dependents can need recompilation/relinking. Prototype has incremental=true, debug=1, 256 codegen units; release has incremental=false, thin LTO and one codegen unit (`.cargo/config.toml:69–83`). Final LTO/link artifacts remain separately expensive even after leaf splitting.

## The Current Policy Is a Soft Budget

`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔣️policy.json:466–485` sets Cargo budget 85,899,345,920 bytes (80 GiB), incremental budget 25,769,803,776 bytes (24 GiB), shared recent guard two days, incremental guard one hour, and pressure below 8 GiB aiming at 16 GiB with a 24-hour guard. Those are logical planning values and retention preferences, not a hard upper bound.

The defining `⚡️caching/🧹️pruning/🟦️.ts:48–74` explicitly preserves recent/locked units above budget and reports guardedOverBudgetBytes. Its scanners sum stat sizes; target-file scanner uses stat.size. Compiler writes are admitted without reserving disk allocation. Pressure deletes afterwards and may refuse while foreign compilers/readers hold state. The present physical Cargo build alone is nearly twice the nominal 80-GiB budget. No finite never-exceeded maximum follows from age-based pruning, free-space sampling or post-write eviction.

`nx.json` supplies cacheDirectory but no maxCacheSize. A pinned Nx default, if any, is not an authored fixed policy here. Public product outputs, private captures/pins, registry sources, toolchains and ticket outputs need their own allocations in the overall repository/storage envelope. A fixed cache ceiling cannot bound an unrestricted source checkout, Git history, arbitrary user files or externally configured Cargo homes; define each scope honestly.

## Recipe Targets and Duplication

`🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎯️recipe/🟦️.ts` hashes manifest/config/toolchain/command/environment into `<buildDirectory>/t/<32-hex>/recipe.json` slots and leases custody through capture. `🏗️native-build/🟦️.ts:205–210` hashes ancestor Cargo manifests/lock/config/toolchain and user Cargo config; source is intentionally freshness-checked by Cargo on each invocation. Schema excludes scheduling-only jobs and owned storage env overrides. Configuration/lockfile changes birth new persistent slots; schema has no per-slot bytes/count/eviction limit. Prototype/release and targets/features legitimately multiply compilation identities.

Shared build-dir remains explicitly authoritative while CARGO_TARGET_DIR selects recipe uplifted outputs (`🏗️native-build/🟦️.ts:756`). Thus making every crate use a completely isolated build-dir would duplicate its entire dependency intermediates unless the new artifact protocol imports dependency artifacts correctly. Existing overlap report also documents cold concurrent shared-library duplication: a shared directory is not itself proof that all cold producers coalesce. Replacing Cargo targets with Nx task outputs must not blindly cache a whole shared mutable target directory: that mixes ownership, races writes and restores unrelated state.

## Required Architecture

1. Author a language-neutral Rust unit descriptor and artifact receipt schema before implementations: package identity, exact source/generated/build-script inputs, host/target role, triple, mode/profile, compiler/toolchain/linker/sysroot, resolved feature set, cfg/rustflags/env, locked third-party closure and direct dependency receipt digests. Keep scheduling/storage identities outside semantic keys. Build scripts declare env/file inputs and generated outputs; undeclared inputs fail qualification.
2. Infer one Nx project/task per crate and resolved compilation variant, with exact source closure including files mounted outside package directory. Direct dependency tasks form a DAG; changed crate source invalidates that crate and affected consumers. Contract crates, implementations, proc-macros/build scripts, test harnesses, application assembly and linking remain separate. Root Cargo workspace and product scripts schedule completion only. Do not use a giant workspace wildcard as every crate input.
3. Split existing giant root-mounted domains into real Cargo crates around schema contracts and domain ownership: OS store/codec/snapshot, protocol/model, DSL grammar/derive, replication/sync, host/session; plugin contracts/protocol, descriptor/interpreter, host/runtime; renderer/UI layout/text/paint/backend; artifact model standards/subsets/codecs. Replace cross-mounted implementation modules with explicit dependencies and small public contract APIs. Introduce schema limits on source closure bytes/files and measured output size, with actionable build refusal when a crate exceeds its authored boundary. Numerical size limits require qualification against active compiler closures, not this textual census.
4. Nx artifacts own immutable per-unit outputs/receipt, not the entire Cargo target tree. A compiler adapter must preserve compatible Cargo metadata/fingerprints/build-script outputs and crate dependency ABI so cached dependency outputs actually prevent recompilation. Cargo unit-graph/dep-info and real compiler traces qualify exactness; simply running `cargo build -p` in a private target does not supply independently reusable leaf artifacts. Keep final application link/LTO as a separate task.
5. Retain one bounded shared mutable compiler scratch pool where practical, separate from immutable Nx artifacts. No per-agent/per-invocation dependency tree copies. Incremental history is optional scratch with a strict subquota; evict before admission and disable incremental for an oversized unit if its declared policy permits. Active state is never pruned to satisfy a cache preference.
6. Define a fixed physical allocation envelope covering immutable Nx artifacts, Cargo scratch/incremental, recipe uplifted state, registry/toolchains as applicable, restored/public outputs, captures/pins, logs/indexes, and worst-case staging/atomic replacement overlap. Atomically reserve each producer and each restore before writing. `used + committed reservations <= hard cap` is the ledger invariant; cancellation releases only after physical retirement. Full active working set means queue or refuse, never exceed the cap.
7. A strict never-exceeded guarantee for native Cargo/rustc requires a real filesystem quota or fixed-size volume/container covering every child write. Post-write stat polling can overshoot arbitrarily between observations. macOS, Windows and Linux quota/volume provisioning must be qualified individually; without an enforceable backing limit and accounted metadata, describe the result as a soft target. Compiler ENOSPC from the bounded volume must cancel and clean only owned partial outputs, without publishing an artifact. Keep minimum host headroom separately.
8. Disk admission must include cache-hit restore, archive extraction, publication copy/replacement, hardlink/clone accounting, SQLite/WAL/indexes, open-unlinked files and partial failed outputs. Charge allocated blocks conservatively and manage snapshots; logical bytes and clones do not imply physically reclaimed space. Avoid retaining full historical public outputs alongside cached duplicates unless charged.

Portable acceptance corpus should cover changed crate builds, unaffected sibling artifact reuse, changed direct/transitive dependency, generated/build-script input and environment changes, host/target proc-macro split, feature union changes, source paths outside manifest directory, two modes/triples, equal-to-cap and cap-plus-one, concurrent reservations, active-set exhaustion, cache restore staging, cancellation, oversized compiler output and crash recovery. Validate schema with Ajv, graph/reservation decisions with independent SQLite, output equivalence with real Cargo/rustc and third-party library oracles. Run tiny real builds for actual changed-source rustc execution and unchanged artifact reuse before any fleet build. No implementation or qualification of this architecture is claimed by this audit.
