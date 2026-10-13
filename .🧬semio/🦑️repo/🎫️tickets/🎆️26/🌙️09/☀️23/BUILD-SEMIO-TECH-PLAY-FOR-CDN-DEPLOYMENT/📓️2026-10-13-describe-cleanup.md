# 🧹️ Describe Cleanup: Retired Owned-Describe API, Host-Tool Profile, Hub Identity Schema, Inventory Runner (2026-10-13)

This report follows up the open items of `📓️2026-10-13-describe-tool-fix.md` §5 and the critical-path finding of `📓️2026-10-13-wave-n1-n2.md` (the describe tool is the slowest cold chain, 1,072 s).

Rules kept:
- No git-modifying command, no worktree, no sub-agent, no ticket operation.
- `🗑️generated` was not touched.
- Every cargo invocation ran in the foreground through `🚦️cargo-slot.sh` (wrapper `describe-cleanup/🏗️cargo.sh`), with private `CARGO_TARGET_DIR` and `CARGO_BUILD_BUILD_DIR` under `⚡️cache/dd/<lane>`. All five lanes (`base`, `p1`, `final`, `describe`, `job`; 5.3 GB) are deleted.
- One exception, stopped: an `nx run @semio-tech/os-plugin-describe-rs:build` started under the wrapper turned out to compile in the shared build dir, because the nx owner-command does not pass `CARGO_BUILD_BUILD_DIR` through. It was killed with `📜️kill-tree.py` after about 15 minutes, and its partial capture dir under `dist/` was removed. Any `host-tool` units it finished stay in the shared `⚡️cache/cargo/build`.

## 1. Outcome

| Item | Before | After |
|---|---|---|
| Retired owned-describe API (`SemioActorArtifact`, `SemioDescribeSession` and their kin) | about 400 lines with no production caller | deleted, together with its tests and the dead code it was the last user of |
| `long` differential test | panicked in `SemioActorArtifact::parse` on every current component | drives the production `execute_describe_owned`, with the Wasmtime describe as oracle. **1/1 green on 2 shipped components**, byte-identical descriptors (171,964 B and 324,293 B) |
| Describe crate unit tests | 13/13 | 13/13, plus `long` 1/1 |
| Interpreter unit tests (`plugin-host --lib interpreter::`) | – | 11/11 |
| Describe tool, cold build, private dirs | **945 s** wall (release profile, load 37→59) | **392 s** (cgu 16 / thin-local, load 32→57), **−58.5 %**. The committed `host-tool` profile took 570 s at load 82→118, roughly twice the baseline load, with the same critical-path shape |
| Describe tool, warm no-op rebuild | – | 0.86 s (cargo, shared build dir) |
| Describe tool binary | 4,523,872 B | 5,498,240 B (+0.97 MB) |
| Describe runtime (owned interpreter), same machine, interleaved | animate 14.3 / 19.5 s, stdio-image 35.4 / 34.4 s | animate 14.4 / 15.7 s, stdio-image 35.4 / 35.0 s. Equal within load noise; descriptor sha unchanged (`6c48b426…` = receipt) |
| Wasmtime in the describe tool's normal dependencies | yes, unused outside `cfg(test)` | dev-dependency only |
| reasoning identity schema | absent at the hub owner. The only one was in `✏️s/🔌️plugins/💡️reasoning` and refused `semio-hub-` | `🌎️hub/🧩️compositions/💡️reasoning/🧬️schema/🪪️plugin-identity/🔣️.json`. The legacy schema is deleted, along with its index and catalog rows |
| plugin-identity suite (vitest) | 1/3 | 3/3 against the registry the L1 component split generates (projected, see §4). Against the committed registry, which is stale since 10-12 23:42, it fails until `generate` can run (§5) |
| Inventory suite under its declared runner (vitest, `test-deployment-contract`) | 12/13 (`Bun is not defined`) | **13/13** |

## 2. Task 1: Retired Owned-Describe API

### Evidence it was dead in production

- **Callers.** Before the change, `rg` over every `*.rs`, `*.ts` and `*.json` found `SemioActorArtifact`, `SemioActorExport`, `SemioActorInstance`, `SemioDescribeHost`, `SemioDescribeSession`, `SemioDescribeStepOutcome` and `SemioDescribeReply` only in `🧠️interpreter/🦀️.rs`, the interpreter's unit tests and the describe `long` test.
- **ABI.** The API validated the 7-export `[async-lift]semio:framework/…` canonical ABI plus the `[task-return]describe` import. Current components carry the owned 24-export ABI. The describe-tool-fix run showed the parse fails on every current component.
- **Production path.** Production describe is `OwnedRuntime::compile` + `describe_observed` (`execute_describe_owned`), over `OwnedSemioArtifact`.
- **Checkpoint, restore and cancel.** These belong to the production `OwnedRuntime` (`🖥️host/🧪️tests/🔬️owned-runtime`). They never belonged to a describe-only session.

### Deleted

- **`🧠️interpreter/🦀️.rs`:**
  - `SEMIO_DESCRIBE_EXPORT` / `_RETURN_MODULE` / `_RETURN_NAME` / `_CHECKPOINT_MAGIC` / `_CHECKPOINT_VERSION`;
  - `SemioActorExport`, `SemioActorArtifact`, `SemioActorInstance`, `SemioDescribeReply`, `SemioDescribeHost`, `argument_i32`, `SemioDescribeStepOutcome`, `SemioDescribeSession`;
  - the region `🔌️SemioActorBoundary`. Its surviving constants now open `🧠️OwnedSemioBoundary`.
- **Dead code exposed by the deletion:**
  - `CoreModule::imports()` (its only caller was `SemioActorArtifact`);
  - `wasm_execution_binary` / `normalize_wasm_binary` / `encode_unsigned_leb`. Their only caller was the Wasmtime oracle, and the oracle now runs the unmodified shipped bytes (Wasmtime accepts the custom sections; proven by the green `long` run).
- **Interpreter tests:**
  - `semio_describe_host_preserves_context_backpressure_and_bounded_result`;
  - `configured_component_fixture_is_owned_parseable`;
  - `long::configured_component_fixture_owned_describe_runs_bounded_fuel` (env-gated by `SEMIO_OWNED_COMPONENT_FIXTURE`, so vacuous in every level);
  - `execution_binary_recursively_removes_only_custom_sections`.
- **Schemas and fixtures.** The API had none: no schema, fixture or launch entry named it.

### Retargeted `long` test (`🖨️describe/🧪️tests/🔬️unit/🦀️.rs`)

- `shipped_components_owned_descriptors_match_wasmtime` compares the production owned describe with `execute_describe_wasmtime` (Wasmtime component host, raw shipped bytes) and asserts byte equality.
- **Suite.** `SEMIO_OWNED_DIFFERENTIAL_FIXTURES` when a caller names components. Otherwise the two smallest shipped components, from `🌎️hub/🧩️compositions/**/📦️packages/🦀️rust/dist/components/*.wasm` across compositions and extensions.
- **Fewer than two components.** The test fails with a message, so the `long` level can no longer pass vacuously.
- **Run** (cargo test, private dirs, 117 s): `semio_hub_stdio_binary.wasm` and `semio_hub_stdio_media.wasm` matched Wasmtime exactly; 1 passed.

## 3. Task 2: Describe-Tool Cold Build

### Where the 945 s went (cargo `--timings`, baseline release profile)

- **Critical path.** `os-kernel` 608 s (132 s frontend, 476 s single-unit LLVM codegen), then the describe link with thin LTO, 82 s.
- **Other heavy units:**

  | Unit | Time |
  |---|---|
  | `semio-framework` | 430 s |
  | `plugin-host` | 332 s |
  | `cranelift-codegen` | 442 s |
  | `wasmtime` | 270 s |

- **Unit-time sum, 5,264 s:**
  - the Wasmtime/Cranelift family is 1,807 s (34 %), linked into nothing: the release binary never references Wasmtime;
  - the `semio-framework-ui*` family is 490 s.

### Change

1. **`[profile.host-tool]`** in the root `Cargo.toml`: `inherits = "release"`, `codegen-units = 16`, `lto = false` (thin-local LTO across each crate's units).
   - The describe build script (`DescriptorBuildScript`, `DESCRIBE_TOOL_PROFILE`) builds with `--profile host-tool`.
   - Both release pipelines declare the `describe-tool` job with `"profile": "host-tool"`, in the root `📋️project.json` and the play `📋️project.json`. The admission broker keys unit history by profile, so the declaration must name the profile cargo actually uses.
   - The artifact capture already resolves `--profile` (`native-build/🟦️.ts:521`).
2. **Wasmtime moved to describe's `[dev-dependencies]`.** It serves only the `cfg(test)` oracle.
   - This does not shrink the closure yet, because `plugin-host` still depends on Wasmtime unconditionally (§5).

### After (P1 = the same settings via `--config`; final = the committed profile)

- **Critical path (P1):** `ui-styling → ui-contract (98 s) → ui-scene → ui (99 s) → semio-framework (170 s) → os-config → plugin-host (107 s) → link (8 s)`, about 392 s.
- **Unit-time sum** 2,668 s, against 5,264 s.
- **Runtime.** cgu 1 for `plugin-host`, where the interpreter lives, was considered. It was not needed: describe throughput is equal (§1), so all crates use 16 units.

### Stage cache

The describe tool is deliberately **not** a stage-cache entry. A warm generation compiles into the shared Cargo build dir, and a no-op rebuild of the tool measured 0.86 s.

| Generation | Behaviour |
|---|---|
| Warm | Cargo's content-checksummed, unit-granular freshness is the cache. Both product releases (root `workspace:release` and play `build`) reuse it, so the tool is built once. |
| Fresh | Cold by contract, a from-scratch proof. |

A second, coarser key built from a prior build's dep-info could miss environment, rustflags or feature changes. It would buy less than one second.

## 4. Task 3: reasoning Identity Schema, Inventory Runner

### Schema location

- **Owner.** The identity tuple belongs to the owner of the component facet, `🌎️hub/🧩️compositions/💡️reasoning`, because L1 put the component crate in `💡️reasoning/🧩️component/📦️packages/🦀️rust`. The composition root `🦀️.rs` performs `builder("reasoning")` and `package_id("semio:reasoning")`, and `🧫️fixtures/🧫️plugin-identity` lives there.
- **Precedent.** `🪐️space` already set it: `🧬️schema/🪪️plugin-identity/🔣️.json` pairs with `🧫️fixtures/🧫️plugin-identity/🔣️.json`.
- **New schema.** `ReasoningPluginIdentity`, `$id` `https://semio.tech/schema/hub/reasoning/plugin-identity`, with `packageName` const `semio-hub-reasoning-component`.
- **Removed.** The legacy `✏️s/🔌️plugins/💡️reasoning/🧬️schema/🔣️.json` (`s.reasoning`, `packageName` `^semio-s-plugin-`) is deleted. It had no other definition.
- **Hand-applied derived rows.** The legacy schema's rows were removed by hand from three derived files: the schema document index, `🔣️schema-catalog.json` and `📓️schema-catalog.md` (count 3755 → 3754).
  - Both generators' projections were computed without writing (`describe-cleanup/🔎️schema-index-diff.ts`, `🔎️schema-catalog-scopes.ts`). They confirm that removal is exactly their delta for this change.
  - The only other index delta is a peer's new `nx-graph-seal` schema, left to its owner.
  - Neither generator projects a hub `🪪️plugin-identity` scope.

### Suite (`📇️registry/🧪️tests/🪪️plugin-identity/🟦️.ts`)

- **Owner.** `identityOwner(cratePath)` is the library owner of the component facet, resolved through the registry's own `componentLibraryManifestV1`, not `cratePath/../..`.
- **Lookups.** The root source, the fixture and `🧬️schema/🪪️plugin-identity/🔣️.json` are read from that owner.
- **Verification** (`describe-cleanup/🪪️identity-projection.py`, run under vitest):
  - It builds the registry `generate` will emit after L1: every row's `cratePath` becomes its `🧩️component` crate, and `packageName` comes from that crate's manifest. All 68 rows have one.
  - Result: **3/3 pass**, and the reasoning and space fixtures validate against their owner schemas.

### Inventory runner

- `test-deployment-contract` registers the suite under vitest (node). Its only Bun-specific call was `Bun.spawnSync`.
- It now uses `node:child_process.spawnSync`. The "in Bun and independent Node" law still spawns `bun run -` and `node` subprocesses.
- Result under vitest: **13/13**.

## 5. Not Fixed (with evidence)

- **The committed registry is stale, and `generate` refuses to run.**
  - `@semio-tech/plugin-registry:generate` fails with `Invalid registry descriptor: /hashes is not allowed`: 36 of 44 hub descriptor pairs still carry the A2 binding.
  - `🤖️generated/🔌️plugins.json` (10-12 23:42) therefore still names pre-L1 library crates.
  - Until `workspace:components describe --all`, or a write-mode release, regenerates the pairs, plugin-identity fails on the committed registry: animate's library manifest has no `[package.metadata.component]`, and `componentLibraryManifestV1` refuses a non-facet crate path.
  - This is A2 and L1 state, coordinator- or catalog-owned.
- **Structural follow-up for 10× growth (E2b, plugin host): an owned-runtime crate.**
  - The tool's critical path is now the `plugin-host` closure: ui chain → `semio-framework` → `os-config` → `plugin-host`.
  - The describe tool needs only the owned interpreter and `OwnedRuntime`, but `OwnedRuntime` lives inside `plugin-host`'s root with Wasmtime, ui (wgpu feature), `os-services`/tokio and job.
  - Extracting the owned runtime (interpreter + `OwnedRuntime` + `GuestRuntime` vocabulary), and splitting the manifest types out of `semio-framework`'s ui dependency, would remove:
    - the 1,807 s Wasmtime family;
    - the ui chain from the critical path.
  - This touches the E2b host internals, so it was left untouched.
- **Owned-runtime test failure (E2b).** `owned_runtime_tests::configured_component_executes_owned_describe_reactor_jobs_cancel_and_checkpoint_restore`, run once with `SEMIO_OWNED_COMPONENT_FIXTURE=animate`, fails at `checkpoint fuel-yielded owned turn` with `native encode has no available explicit retirement slot`.
  - The test is env-gated and never runs in normal levels. Its describe part passes.
  - The code it exercises is unrelated to the deleted API.
- **The registry vitest config also loads `bun:test` files** (7 files load with 0 tests and count as failed).
  - `catalog-publication` `valid-owner-descriptor` fails, 1 of 45 cases (trusted-catalog lane, in flight).
  - `generated-projection` has the known JSON import-attribute fault.
- **Other host tools.** `font-assets` still builds with `--release`, so moving it to `host-tool` is up to its owner.

## 6. Verification Run

| Check | Result |
|---|---|
| `cargo check -p plugin-host -p describe --lib --tests` | green, with warnings as proof of type-check |
| describe unit tests (built test binary, `--skip long`) | 13/13 |
| describe `long` (cargo test, 2 shipped components) | 1/1, byte-identical to Wasmtime |
| `plugin-host --lib interpreter::` | 11/11 |
| describe-tool cold builds (private dirs) | 945 s → 392 s; committed profile 570 s at 2× load |
| warm no-op | 0.86 s |
| real describe, `host-tool` binary, animate | rc 0, sha `6c48b426…` |
| registry: descriptor-verification + discovery (bun) + `ComponentReceiptV1` | 60/60 |
| registry vitest: catalog-complete, host-activation, rebuild | green (catalog-publication 44/45, not mine) |
| inventory (vitest) | 13/13 |
| plugin-identity (vitest), projected L1 registry | 3/3 |

Not run: the nx `os-plugin-describe-rs:build` job to completion (stopped, see "Rules kept"), and a full release.

## 7. Files

Paths are relative to `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/` unless absolute.

- `🧠️interpreter/🦀️.rs` and `🧠️interpreter/🧪️tests/🔬️unit/🦀️.rs`: deletions (§2).
- `🖨️describe/🛂️descriptor-emission/🦀️.rs`: the oracle runs raw bytes; doc of the interpreter import.
- `🖨️describe/🧪️tests/🔬️unit/🦀️.rs`: the retargeted `long` test.
- `🖨️describe/📦️packages/🦀️rust/Cargo.toml`: Wasmtime moved to dev-dependencies.
- `🖨️describe/🏗️component-build/🟦️.ts`: `DESCRIBE_TOOL_PROFILE`, `--profile host-tool`.
- `📇️registry/🧪️tests/🪪️plugin-identity/🟦️.ts`: `identityOwner`, owner fixture and schema.
- `📇️registry/📦️deployment/🧪️tests/📇️inventory/🟦️.ts`: `node:child_process.spawnSync`.
- `/Cargo.toml`: `[profile.host-tool]`.
- `/📋️project.json` and `/🏢️semio-tech/🎡️play/📋️project.json`: the `describe-tool` job profile.
- `/🌎️hub/🧩️compositions/💡️reasoning/🧬️schema/🪪️plugin-identity/🔣️.json`: new.
- `/✏️s/🔌️plugins/💡️reasoning/🧬️schema/🔣️.json`: deleted.
- `🗣️dsl/✨️derive/🗂️schema-document-index.json` (`💻️os/🔨️modules/`), and `📚️library/{🔣️schema-catalog.json, 📓️schema-catalog.md}` (`🦑️repo/🔨️modules/`): the `s.reasoning` rows removed.
- Ticket inputs: `describe-cleanup/{🏗️cargo.sh, 🔎️schema-index-diff.ts, 🔎️schema-catalog-scopes.ts, 🪪️identity-projection.py}`.
