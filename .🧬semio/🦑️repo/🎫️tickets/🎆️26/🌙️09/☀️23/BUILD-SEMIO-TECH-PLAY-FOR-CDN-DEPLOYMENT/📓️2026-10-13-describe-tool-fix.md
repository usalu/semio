# 🛂️ Describe Tool Fix, Red Text-Scan Tests, Move-Broken Registry Tests (2026-10-13)

These are M0 prerequisites that follow waves A2, E1 and E2.

Rules kept:
- No git-modifying command, no worktree, no sub-agent, no ticket operation.
- `🗑️generated` was not touched.
- Every cargo invocation ran in the foreground through `🚦️cargo-slot.sh`, with private `CARGO_TARGET_DIR` and `CARGO_BUILD_BUILD_DIR` under `⚡️cache/fixd/describe`. That directory (2.7 GB) is deleted.

The ticket input is `describe-tool-fix/🏗️cargo.sh`, the slot-gated cargo wrapper with the private dirs.

## 1. Outcome

| Item | Before | After |
|---|---|---|
| Describe crate `--lib --bins --tests` check | 13 errors in the lib-test target | 0 errors, 0 warnings in the crate |
| Describe tool, driver host-tool pass (`bun nx run @semio-tech/os-plugin-describe-rs:build`, the release job `describe-tool`) | – | green: cargo `release` 5 m 53 s cold in private dirs, nx 7 m 04 s; binary staged in `dist/build` |
| Describe crate unit tests (`os-plugin-describe-rs:test`, nextest `fundamental`) | did not compile | **13/13 pass**, 1 skipped (the `long` differential) |
| Real `describe` on the shipped animate bytes | – | succeeds in 8 s; the pack and JSON are **byte-identical** to the committed pair; the sha equals `receipt.semanticDescriptorSha256` (`6c48b426…`) |
| `framework-plugin:test-retained-raw-source` | 17 pass / 3 fail | **20/20**; the nx target is green, including its strict `tsc` pass |
| Registry and descriptor suites | see §4 | see §4 |

## 2. Describe Tool: Root Cause and Fix

### Root cause: a stale emitter host, not a changed WIT world

The WIT world is unchanged and remains the only source of truth. `🔌️plugin/🧬️schema/📜️.wit` declares the following in `interface pure`:

- `log`, `now-ms` and `trace-span` as sync functions;
- `operation-begin`, `-progress`, `-allocation`, `-reserve-return` and `-finish` as `async func`.

wasmtime 47 `bindgen!` places every `async func` on `pure::HostWithStore<T> for HasSelf<T>`, with an `Accessor`, never on `Host`. The plugin host (`🖥️host/🦀️.rs`) already implements them there.

The emitter's `cfg(test)` wasmtime host (`🛂️descriptor-emission/🦀️.rs`) had the five methods on `pure::Host`. That produced:

- 5 × E0407;
- 5 × E0277 (`DescribeHostState: HasData`);
- an unsatisfied `HostWithStore` bound at `Actor::add_to_linker`.

All of these are in the lib-test target only. The release `--bin` build compiled.

The second fault in the same target came from mounting the interpreter twice:

- The emitter mounted `#[path = "../../🧠️interpreter/🦀️.rs"] pub mod interpreter;` itself, so the whole interpreter compiled a second time in the describe crate. The host crate already compiles it and exports it as `semio_framework_plugin_host::interpreter`.
- The interpreter's own `cfg(test)` checkpoint test then compiled in the describe crate as well, which needs `semio_framework_trace`: 2 × E0433.
- Outside `cfg(test)` the duplicate was dead code. Only the Wasmtime oracle and the parity test use it.

### Fix

- The five `operation_*` doors moved to `impl pure::HostWithStore<DescribeHostState> for HasSelf<DescribeHostState>`. They still answer refusal code 6, because `describe()` owns no receiving operation.
- The duplicate mount was removed. The one interpreter is imported with `#[cfg(test)] use semio_framework_plugin_host::interpreter;`. Effects:
  - its tests stay in their owning crate;
  - the trace error is gone;
  - the release tool no longer compiles a second, dead copy of the interpreter (its root file alone is 3.5 k lines).
- The now-unused `extern crate semio_framework_os_kernel as dsl;` alias was removed. The Cargo.toml note was corrected to match.

### Unit tests made current

| Test | Fault | Change |
|---|---|---|
| `full_catalog_describe_retains_finite_wall_and_fuel_bounds` | pinned fuel `8_000_000_000`; the constant was raised on 10-12 to the measured `32_000_000_000` (architect, 22.66 G) | assert the current constant |
| `owned_core_exports_are_defined_once_and_invoked_by_both_owners` | (1) pinned the literal arity `18`, but `OwnedSemioExport::ALL` is `[Self; 24]` (SQLite exports). (2) Matched `extension_exports!` arms by their exact matcher text; the arms gained `$mounted_policy:expr` | (1) The arity is owned by the type of `ALL`; the test now asserts the names are unique, plus the existing exact-set equality. (2) A brace-matched `macro_arms` walks **every** arm of `plugin_exports!` and `extension_exports!`, whatever its matcher spells, and requires each to reach exactly `ALL`. This is the same contract and still covers the extension-only arm regression. |

E2's source-path change (`PLUGIN_SDK_SOURCE` → `⚛️reactor/🏃️runtime/🦀️.rs`) is correct. The two laws that read it, the owned-export law and the unowned codec text, pass.

### Runtime verification

| Run | Result |
|---|---|
| `semio-framework-plugin-describe describe semio_hub_animate.wasm --out <scratch>` with the newly built release binary | exit 0, 188 M fuel, about 4 s execute, 8.3 s wall; `cmp` identical to `🌎️hub/🧩️compositions/🎞️animate/{🛂️.descriptor.semio,🔣️.json}`; sha `6c48b426e4f2589fbd01e02142c2e5dce31a24c6af4ebe9266c2e2e05e0864f2` = receipt |
| `os-plugin-describe-rs:test -- long` with `SEMIO_OWNED_DIFFERENTIAL_FIXTURES=<animate wasm>` | The fixed Wasmtime host links, instantiates and completes `describe()` on animate (the oracle step at test line 77 returned). The test then fails one line later in `interpreter::SemioActorArtifact::parse`: "component has no core module implementing the Semio actor export ABI". See §5. |
| `test-canonical-descriptor-pack` | `cases=2`, green |
| `test-fresh-component` | Green on the second run. The first run failed `queued-cargo-cancel` at load average 22–34: a 150 ms cancel landed during file observation, before the cargo queue. It is a timing law, not a code fault. |

## 3. `test-retained-raw-source` (3 red → 0)

At session start the suite was 17/20, not 16/20. The fourth test that had been reported red was already green.

| Test | Cause | Retarget |
|---|---|---|
| `mounted original lease …` (`♻️cancellation/📦️lease/🧪️tests/🟦️.ts`) | (1) `std::mem::size_of::<ToolCancellationLeaseRetirement>()` is spelled `size_of::<…>()` since warning-flood. (2) Masked behind (1): `if scope.claim.is_finished()` was in the plugin root `🦀️.rs` at HEAD, and E2 moved it to `🎮️command/📥️admission/🦀️.rs`. E2 had repointed `host` to the instance files, which do not own it. | (1) `sizeOfDemand(field, type)`: a regex accepting optional `std::`/`core::mem::` and any spacing; used for `capacity_bytes` and `release_bytes`. (2) The claim law reads its owner, `🎮️command/📥️admission/🦀️.rs`, with a spacing-insensitive regex. |
| `yielded window refresh …` (same file) | `copy_bytes: std::mem::size_of::<store::LaneItemReceipt>()` is now `size_of::<…>()` | `sizeOfDemand("copy_bytes","store::LaneItemReceipt")` over the same host files |
| `registered owned serializers …` (`♻️lifecycle/🏪️stores/🧪️tests/🟦️.ts`) | E2 moved `artifact_owned_tool_birth_grant` / `_handoff_grant` into `👷️job/🦀️.rs`. The test looked for their definitions and counted calls in the instance files. | Exactly one definition each in `👷️job/🦀️.rs` (regex on `fn name() -> RetainedCloneGrant`). No definition in the host files. Call counts from the host files: birth 3, handoff 1, which is the same contract as the old totals of 4 and 2 including the definition. The retired `retained_*_grant` names stay absent. |

Run: `bun test` over the five files gave 20 pass / 0 fail. `NX_FORCE_REUSE_CACHED_GRAPH=true bun nx run @semio-tech/framework-plugin:test-retained-raw-source --skip-nx-cache` succeeded.

## 4. Registry, Descriptor and Driver Suites (before → after)

| Suite | Before | After | Note |
|---|---|---|---|
| `ComponentReceiptV1` (`🏷️component-receipt/🧪️tests`, bun) | 29/29 | 29/29 | |
| descriptor-verification (`🛂️descriptor-verification/🧪️tests`, bun) | 21/21 | 21/21 | |
| registry discovery (`🔎️discovery/🧪️tests`, bun, `SEMIO_TEST_ARTIFACT_DIR`) | 10/10 | 10/10 | |
| deployment inventory (`📦️deployment/🧪️tests/📇️inventory`, bun) | 12/13 | **13/13** | A2 fallout: the generated-catalog case fed `hashes` to `parseDeployedRegistryEntryV1`, which A2 made refuse it (`/hashes is not allowed`). The semantic entry is now built without `hashes`. |
| catalog-complete (vitest, `exhaustive`) | 11/12 | **12/12** | `refuses ambient roots …` assumed the shared target is `<tmp repo>/target`. Under the nx owner-command, `CARGO_TARGET_DIR` is set, and `cargoTargetDirectory` honours it, so the refusal never fired. Verified: `bun test` passes without `CARGO_TARGET_DIR` and fails with it. The test now refuses `cargoTargetDirectory(repoRoot)`, the path the verifier itself treats as shared. Passes with and without the variable. |
| host-activation + rebuild (vitest, `exhaustive`) | 6/6 | 6/6 | |
| generated-projection (vitest) | 5/6 | 5/6 | pre-existing: JSON import attribute (`📥️invocation/🧬️schema/🔣️.json`), already listed by A2 |
| inventory under vitest (`test-deployment-contract` runs it this way) | 12/13 | 12/13 | `Bun is not defined`: the test spawns through `Bun.spawnSync`, but vitest runs under node. Runner mismatch, not a move. |
| plugin-identity (bun 10:5x / vitest 11:3x) | 2/3 (bun) | 1/3 (vitest) | not move-caused, see §5 |
| `test-canonical-descriptor-pack` | – | green | |
| `test-fresh-component` | – | green (second run) | |
| A1/N1 driver: `🧩️component-pass/🧪️tests` | – | 12/12 | |
| native-build `🧪️tests` | – | 25/25 | |
| receipt provenance (`🧾️receipt/🧪️tests`) | – | 15/15 | |
| stage cache | – | 5/5 | |
| describe crate unit tests | did not compile | 13/13 (+1 `long`, skipped) | |

The vitest totals over the five files were 34/37 before and 36/40 after, with plugin-identity added. `@semio-tech/plugin-registry:test` was run with `--excludeTaskDependencies`. Its `generate` dependency refuses the committed pairs that are still bound (`Invalid registry descriptor: /hashes is not allowed`). That is the known A2 state until `workspace:release` and `workspace:components describe --all` regenerate them.

## 5. Not Fixed (with evidence)

- **The `long` differential test uses a retired owned API.**
  - `configured_real_component_suite_owned_descriptors_match_wasmtime` drives `interpreter::SemioActorArtifact` / `SemioDescribeSession`. These validate the 7-export async-lift `SemioActorExport` ABI, which current components no longer carry.
  - The production path is `OwnedRuntime::compile` + `describe_observed`, over `OwnedSemioArtifact` with 24 exports.
  - Nothing outside the interpreter and its tests uses those two types.
  - The test is env-gated, so it never runs in normal levels. It is a product-test drift, not a move.
  - Proposed retarget: compare the Wasmtime oracle with `execute_describe_owned`. Separately, decide whether `SemioDescribeSession` (checkpoint, restore and cancel of describe) is still owned, or is dead code to delete.
- **plugin-identity:**
  - (a) `🌎️hub/🧩️compositions/💡️reasoning` has a `🧫️fixtures/🧫️plugin-identity` but no `🧬️schema/🔣️.json`, and had none at HEAD either. The only schema with `semio.reasoning.plugin-identity/v1` is the old `✏️s/🔌️plugins/💡️reasoning/🧬️schema/🔣️.json`. Its `packageName` pattern `^semio-s-plugin-` refuses the hub name `semio-hub-reasoning`. This predates A2, E1 and E2, and needs a schema-ownership decision for the hub reasoning composition.
  - (b) The vitest run at 11:3x also failed "missing `[package.metadata.component].package`" for animate. A peer was rewriting `🌎️hub/🧩️compositions/🎞️animate/📦️packages/🦀️rust/Cargo.toml` at 11:28: component metadata, playground rows and `crate-type` were removed, and the `component-guest` feature was dropped. The same case passed under `bun test` at 10:5x. This is in flight and not mine to touch.
- **generated-projection** (JSON import attribute) and **inventory under vitest** (`Bun` global) fail for runner reasons that predate these waves.

## 6. Files

Paths are relative to `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/`.

- `🖨️describe/🛂️descriptor-emission/🦀️.rs`: the `pure::HostWithStore` impl, the interpreter import instead of the duplicate mount, the `dsl` alias dropped.
- `🖨️describe/📦️packages/🦀️rust/Cargo.toml`: the dependency note only.
- `🖨️describe/🧪️tests/🔬️unit/🦀️.rs`: the fuel bound, and the owned-export law (`macro_arms`, the arity from `ALL`).
- `♻️cancellation/📦️lease/🧪️tests/🟦️.ts`: `sizeOfDemand`, and the claim law reading `🎮️command/📥️admission/🦀️.rs`.
- `♻️lifecycle/🏪️stores/🧪️tests/🟦️.ts`: the grant laws reading `👷️job/🦀️.rs`.
- `📇️registry/📦️deployment/🧪️tests/📇️inventory/🟦️.ts`: the semantic entry without `hashes`.
- `📇️registry/🧪️tests/✅️catalog-complete/🟦️.ts`: the ambient target taken from `cargoTargetDirectory`.
- Ticket input: `describe-tool-fix/🏗️cargo.sh`.
