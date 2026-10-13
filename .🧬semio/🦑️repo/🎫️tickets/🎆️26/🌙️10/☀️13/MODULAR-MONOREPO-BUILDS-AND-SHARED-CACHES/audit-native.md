# Native Build Audit

Read-only source audit on 2026-10-13. No production files changed and no builds or tests executed. Read root AGENTS.md, products AGENTS.md, and repository AGENTS.md. Findings below are source observations; performance recommendations require measurements.

## Concrete Gaps

| Owner | Evidence | Proposed Change |
| --- | --- | --- |
| Root `📜️script.ts`, CppScript | `resolvePreset` chooses implicit Windows/macOS/Linux presets; `cppEnv` hard defaults `CMAKE_BUILD_PARALLEL_LEVEL` to 4 | Require prototype or release at build ingress, map mode to an explicit host preset, and derive concurrency from the shared resource policy |
| `CMakePresets.json` | Linux/macOS use Debug implicitly; Windows build/test preset uses Release; experimental cxx26 uses Debug | Define portable prototype/release configure, build, and test presets consistently for all supported hosts; include mode, architecture, and toolchain in mutable store identity |
| Root `📜️script.ts`, CppScript | Configure/build/test share the preset directory; `purgeStaleCmakeCache` recursively deletes it without a visible lease | Hold an exclusive resource lease through configure/build artifact capture; refuse or rebuild incompatible cache while holding the same lease |
| Root `CMakeLists.txt` | Only project options and CTest are defined; no native subdirectories or targets | Grow through domain-owned target contributions rather than treating a successful empty CMake build as feature validation |
| `Cargo.toml`, `🧰️framework/Cargo.toml`, `🌎️hub/Cargo.toml` | Native dev profiles disable incremental compilation; only wasm-iterate has an explicitly iteration-oriented profile | Add consistent native prototype profiles with incremental compilation and low-cost code generation; retain measured hot dependency optimization where required |
| Repository CLI `🔨️modules/⌨️cli/📦️packages/🦀️rust/📜️script.ts` | Build, Repo, and Mcp run `cargo build` without profile and execute from `debug` | Require explicit mode for artifact-producing build; map runtime bootstrap to declared prototype contract and locate the corresponding profile deliverable |
| UI styling `🏗️builder/🔷️dotnet/📜️script.ts` | Build rejects all arguments and forces Release; shared mutable state is keyed by dotnet/ui-styling only | Require mode and map prototype to Debug and release to Release; qualify shared state by compiler/platform/architecture; publish outputs with mode ownership |
| UI styling `🏗️builder/🐍️python/📜️script.ts` | Wheel build rejects all arguments and has one contract | Require declared mode even if wheel bytes are identical; keep interpreter execution and wheel publication policy separate; avoid inventing optimization that provides no runtime benefit |

## Existing Architecture to Preserve

- `.cargo/config.toml` already provides shared target/build stores, nightly fine-grained locking, content checksum freshness, and per-target flags. Separate deliverables from shared compiler intermediates.
- Native release driver at framework/process/artifacts/native-build/component-pass already uses every available core through a GNU make jobserver, rustc memory admission, telemetry, critical-path scheduling, cached component stages, and cancellable child processes.
- Release passes are grouped by profile, with workspace passes in a lane executed sequentially to reduce shared-unit contention. Preserve this when introducing modes.
- UI styling Python and .NET builders already lease mutable state, capture outputs in unique staging directories, and publish artifact copies through the shared publication service.
- Root native release profile uses opt-level 3, thin LTO, one codegen unit, and disabled incremental compilation. Wasm release deliberately uses opt-level s and 16 codegen units, with documented prior timing/size measurements. Avoid replacing measured policies with blanket knobs.

## Cache and Resource Boundaries

The mode must enter the Nx task configuration/hash and the artifact ownership identity. Toolchain identity, target architecture, platform, feature set, locked inputs, and mode must participate where they affect outputs. Mutable compiler state should remain shared within those safe boundaries; task-output caching should restore final artifacts rather than mutable compiler stores.

`canonicalArchitectureEnvironment` in repo/library/caching/cargo sets both `CARGO_TARGET_DIR` and `CARGO_BUILD_BUILD_DIR` to a canonical-architecture store. Ordinary Cargo uses the shared root configured in `.cargo/config.toml`; canonical architecture work therefore forms a separate compiler-store partition. This may be intentional for canonical correctness, but should be explicitly modeled and tested rather than attributed to the ordinary shared-store contract.

The release driver creates a per-generation all-core jobserver. Simultaneous release drivers can each allocate all cores. Existing host-wide lease infrastructure should coordinate CPU/memory admission across generations and ordinary builds; simply setting every subprocess to all cores would oversubscribe a collaborative workspace.

`.cargo/config.toml` fixes Rust compiler frontend threads at eight. CPU budgeting must account for internal compiler threads as well as Cargo job counts. Target-scoped rustflags replace global flags, so preserving those flags matters; wasm32-unknown-unknown currently does not repeat the global threads flag.

## Validation to Add Before Implementation

1. Neutral fixtures for prototype/release mode admission, tool mapping, mutable-store identity, and output identity; compare JSON validation with an existing independent schema library.
2. Cargo metadata/profile argv fixtures proving native prototype, native release, wasm prototype, and wasm release are selected explicitly and output lookup follows the selected profile.
3. CMake preset fixture validation across Windows, macOS, and Linux, plus a real tiny native target smoke test when targets are introduced.
4. Concurrent lease tests proving configure/cache purge cannot delete a running builder's mutable state, and mode-specific output publication cannot overwrite another mode.
5. Actual cold, warm, and one-source-edit measurements before asserting build latency improvements; record runtime smoke output for built native binaries.

## Suggested Ownership for Execution

Parent owns schema-first build-mode policy and shared process helpers. A native implementation agent can own root Cargo profile blocks and the three nested workspace profile blocks, CMakePresets.json, CppScript, styling .NET/Python builder scripts, and repository CLI Rust script. Nx dependency/cache projection changes belong with the Nx audit owner. Keep launch registration coordinated with the parent so a shared file does not receive conflicting command edits.
