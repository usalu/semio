# 🧠️ Owned Runtime Crates: Describe Tool Without plugin-host or Wasmtime (2026-10-13 / 2026-10-14)

This report covers the "next speed-up" from `📓️2026-10-13-describe-cleanup.md` §5. The owned interpreter and `OwnedRuntime` moved out of `semio-framework-plugin-host`. The describe tool no longer depends on `plugin-host`, so it no longer compiles Wasmtime, Cranelift, `os-config` or `os-services`.

Day 1 (2026-10-13) extracted one crate and measured it. Day 2 (2026-10-14, after the session restart and about 24 h of tree movement) split that crate into three acyclic owners to fit the Codex fleet's global source admission ceiling (`MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/crate-production-boundaries.md`: 4,000 active lines / 262,144 bytes / 32 files per unit).

**Rules kept.**

- No git-modifying command, no worktree, no sub-agent, no ticket operation.
- `🗑️generated` was not touched. No Codex-fleet package was edited.
- Every cargo command ran in the foreground through `🚦️cargo-slot.sh` (wrapper `owned-runtime-crate/🏗️cargo.sh`), with `CARGO_TARGET_DIR` and `CARGO_BUILD_BUILD_DIR` under `⚡️cache/ort/<lane>`. Those lanes are deleted (§7).
- The one exception: the harness moved two long commands to the background after its 600 s foreground cap: a day-1 test build and the day-2 unit-census gate.

## 1. Outcome

| Item | Before | After |
|---|---|---|
| Describe tool normal-dependency closure | `plugin-host` + Wasmtime/Cranelift (102 units) + `os-config` + `os-services` | 101 packages (`cargo tree -e normal`, day 2). No `wasmtime*`, `cranelift*`, `semio-framework-plugin-host`, `os-config` or `os-services` |
| Describe tool cold build, `--profile host-tool`, private dirs | **423.1 s** (cargo `7m 03s`), load 30→61, 401 units, unit-time sum 3,047 s | **361.3 s** (cargo `6m 01s`) at roughly double the load (88→116), 102 units, unit-time sum 1,331 s (−56 %) |
| Wasmtime/Cranelift family in the tool build | 102 units, 1,243 s | 0 |
| Describe binary | 5,498,240 B | 3,277,376 B |
| Real describe of animate (shipped `semio_hub_animate.wasm`) | committed pair, receipt `semanticDescriptorSha256 = 6c48b426…` | rc 0, 188,159,977 fuel, 75 s. `🛂️.descriptor.semio` and `🔣️.json` **byte-identical** to the committed pair; sha `6c48b426e4f2…` = receipt |
| Describe unit tests | 13/13 | 13/13 (day 1) |
| Describe `long` differential (owned vs Wasmtime) | 1/1 | 1/1, byte-identical on `semio_hub_stdio_binary` (171,964 B) and `semio_hub_stdio_media` (324,293 B), 158 s (day 1) |
| Source admission of the owned-runtime code | — | 3 crates, each **PASS** (§3) |

The cold-build numbers come from the day-1 single-crate extraction. Day 2's three-crate split leaves the tool's closure unchanged in substance (§3), but it could not be rebuilt cold: the tree does not compile today (§6).

## 2. Critical Path (cargo `--timings`, `⏱️timings-critical-path.py`)

**Before** (423.1 s):

```
syn → … → semio-framework-ui (121.6 s, start 187) → semio-framework-tool-run (54.3)
  → semio-framework (139.3) → semio-framework-os-config (40.3)
  → semio-framework-plugin-host (87.6) → describe bin (5.7)
```

Off the path, the Wasmtime family still took 1,243 s of CPU (`cranelift-codegen` 252 s, `wasmtime` 116 s, `wasmtime-environ` 101 s, `wasmparser` 93 s, `wast` 88 s, `wasmtime-wasi` 81 s).

**After** (361.3 s, at about twice the load):

```
value-derive (8.8) → value (27.4) → pack-json (16.8) → replication (59.5)
  → ui-contract (79.3) → ui-scene (41.6) → ui (71.2) → tool-run (44.8)
  → semio-framework (199.7) → describe bin (3.5)
```

The owned runtime (106 s at that load) ran in parallel with `semio-framework`'s codegen, because it pipelines on `semio-framework`'s rmeta. It is off the path.

What remains on the path:

- The `ui → semio-framework` chain. It is genuinely needed today: the kernel types the runtime and the emitter speak (`Budget`, `Event`, `TurnResult`, `PackageDescriptor`, `Fault`) live in `semio-framework`, which depends on `semio-framework-ui` with the `wgpu` feature.
- `os-kernel` (pack runtime).

The next structural cut is to move the manifest/kernel vocabulary out of `semio-framework` into a crate without the ui chain. The Codex fleet is cutting the UI side now.

Comparing the two runs: unit durations inflate with load (`semio-framework` 139 s → 200 s), so the wall comparison is conservative. Under equal load the expected saving is the two removed path units (`os-config` 40 s + `plugin-host` 88 s), about 295 s against 423 s.

## 3. Crate Layout (Day 2, Three Acyclic Owners)

| Package (name ≤ 48) | Place | Owns | Normal deps | Admission (files / active lines / bytes) |
|---|---|---|---|---|
| `semio-framework-plugin-interpreter` | `🔌️plugin/🧠️interpreter/📦️packages/🦀️rust`, lib = `🧠️interpreter/🦀️.rs` | core interpreter: decode, fuel, cancel, checkpoint, instance opening. Exports `INTERPRETER_SOURCE` | `value`, `job`, `actor` | 3 / 3,293 / 175,356 PASS |
| `semio-framework-plugin-guest-contract` | `🔌️plugin/🧬️guest-contract/📦️packages/🦀️rust`, lib = `🧬️guest-contract/🦀️.rs` | vocabulary every guest runtime shares: `TurnFault`, `PluginHostError`, `IoRouteConflict`, `IoEntryRouteConflict`, `JobBudget`, `JobStep`, `GuestCallCancellation`, `GuestDocumentPair`, `GuestDocumentMirror`, `decode_guest_fault_bytes`, `identity_turn_fault`, `validate_cold_page`, `GUEST_*` bounds. Also mounts `operation_authority`, `original_actor_context` and `sqlite_wire` | `semio-framework`, `os-kernel`, `value(-derive)`, `actor`, `async`, `job`, `diagnostic`, `artifact-reference`, serde | 10 / 966 / 110,432 PASS |
| `semio-framework-plugin-interpreter-runtime` | `🔌️plugin/🧠️interpreter/🏃️runtime/📦️packages/🦀️rust`, lib = `🏃️runtime/🦀️.rs` | `OwnedRuntime`, `OwnedCompiledGuest`, `OwnedGuestInstance`, snapshot operations, the owned operation framing (`🎛️operation`), startup, call and jobs, `owned_engine_identity`, `decode_owned_result` | interpreter + guest-contract + the same neutral set, `hash`, `pack-json`, `trace` | 8 / 1,475 / 135,266 PASS |

The graph is acyclic: `interpreter ← interpreter-runtime → guest-contract`. `plugin-host` (Wasmtime) and the describe tool both depend on `guest-contract` and `interpreter-runtime`.

How the cuts work:

- **Measurement.** The admission numbers come from `owned-runtime-crate/📏️source-admission.ts`, which runs the canonical `readRustSourceClosureV1` with the same native aarch64/macOS cfg as the OS crate-boundaries gate.
- **Engine identity.** The single day-1 crate measured 21 files / 420,577 B and failed admission. Part of the reason: the runtime embedded the whole interpreter source (`include_str!`) for `owned_engine_identity`.
  - Now the core exports `INTERPRETER_SOURCE`, and the runtime hashes it. The digest algorithm is unchanged (magic + length-prefixed interpreter source + adapter source).
  - The adapter half is now the runtime file, not the 8.5 k-line host file. Identities change once, and the hub's verification cache re-verifies, which is the documented behaviour.
- **The `From` impl.** `impl From<CoreError> for TurnFault` would be an orphan impl after the cut. It became `fn core_fault` in the runtime crate.
- **`CompiledHandle` and `GuestInstance`.** Both stay in `plugin-host`, which owns backend dispatch. Each holds `Option<OwnedCompiledGuest>` / `GuestInstanceState::Owned(OwnedGuestInstance)`. `impl GuestRuntime for OwnedRuntime` stays in `plugin-host`, delegating to the inherent owned API (`compile_component`, `instantiate_actor`, `execute_actor_turn`, `start/step/cancel_actor_job`, `checkpoint_actor`, `restore_actor`, `admit_context`, `close_context`).
- **SQLite snapshot calls.** `plugin-host` drives these through a narrow owned API (`OwnedRuntime::codec_instance`, `OwnedGuestInstance::{pending_snapshot, begin_snapshot, resume_snapshot, poisoned, turn_in_flight}`, `OwnedSnapshotOperation`), never through owned internals.
- **Wasmtime.** Wasmtime is a dev-dependency only: of the describe crate, as its oracle, and of the core and runtime crates, as their interpreter oracle.
- **No shims.** No compat layer and no re-export shim exist. `plugin-host`'s former `pub mod interpreter`, `operation_authority`, `reactor` and `sqlite_wire` re-exports are gone. Every consumer imports from the owning crate.
- **Dev opt-level.** `[profile.dev.package.semio-framework-plugin-interpreter] opt-level = 3`, formerly on `plugin-host`, now sits on the core in the root, hub and all 137 nested workspace manifests. The hot loop (`execute_machine`) lives there.

## 4. Consumers Rewired

These now import from the owning crate; their manifests gained `guest-contract` and/or `interpreter-runtime`, with `interpreter` kept only as a dev-dependency where a test reads core types:

- `plugin-host`;
- the describe tool;
- mcp workspace, `run`, renderer-wgpu (runtime, native-caller, renderer);
- hub trusted catalog and its tests, the count profile, component-codecs and the draw guest test;
- the note guest tests;
- the shard child binary.

Integration tests that peers hung on the day-1 crate moved to their owners:

- `original_native_frame_caller` and `original_actor_poll_input` → guest-contract;
- `original_host_deadline` and `original_host_root` stay on the core.

`📜️script.ts` exact-law groups now name the right package. Examples: job exports split into core `tests::…`, runtime `owned_runtime_tests::…` and contract `original_actor_context::…`. The guest SDK's snapshot refusal law `operation::input::tests::…` → runtime.

Source-text gates were repointed:

- the host `guest-fault-check` reads the runtime and contract files;
- the host admission test reads the runtime and contract files. Its law passes under `bun test`; its two sibling laws fail on strings that no source in the tree contains, which is peer-owned.

Lockfiles were resynced with `cargo metadata --offline` per correction 93(e): root, hub and the 21 nested workspaces that resolve `plugin-host`, with no third-party version change. `🌎️hub/🧩️compositions/💠️lowpoly/📦️packages/🦀️rust/Cargo.lock` is a stray member lockfile that its workspace (hub) does not use.

Launch: `check` and `test` entries for both new projects follow the interpreter's entries (4_gate, 900.04423–900.04426, `bun repo run …`).

## 5. Verification

| Check | Result |
|---|---|
| `cargo check` interpreter + describe + plugin-host `--lib --bins --tests` (day 1, single crate) | green, with warnings as proof of type-check |
| describe unit tests (day 1) | 13/13 |
| describe `long` differential (day 1) | 1/1, byte-identical |
| real describe of animate, `host-tool` binary (day 1) | rc 0, byte-identical pair, sha = receipt |
| interpreter crate tests (day 1, before the heap witness) | 17 passed, 6 failed. All 6 were heap-observation laws that read 0 because the crate's test build had no `HeapWitness` global allocator. The witness was added (now one per crate); the rerun was cut by the session restart |
| plugin-host `cargo check --tests` (day 1) | green |
| mcp / run / renderer-wgpu `cargo check` (day 1) | red with errors that pre-date this change (`Budget` missing `retained`, `TurnResult` fields, …). None mentions a moved or renamed item except two pre-existing non-exhaustive matches whose type path now reads `guest_contract` |
| Day-2 source admission, three crates | PASS (§3) |
| Day-2 `rustfmt` parse of the three crate roots | rc 0 |
| Day-2 dependency resolution (`cargo tree -e normal`, `cargo metadata --offline` on 23 workspaces) | resolves; describe closure free of Wasmtime and plugin-host |
| Day-2 `cargo check` of the three crates + describe + plugin-host | **blocked**: `semio-framework-job` (E0502), `ui-theme`, `ui-layout`, `os-pack`, `os-command` and `os-store-message-ledger` do not compile. These are the Codex fleet's unqualified splits and a job edit at 16:01, all also red in the coordinator's `📓️2026-10-14-compile-baseline.md` run (16:49–16:57). None of my crates reports an error of its own |
| Day-2 describe cold build, unit tests, `long`, animate describe, interpreter tests | blocked by the same red dependencies |
| Hub wasm workspace check | not run; red in the coordinator baseline on the same peer crates |
| hub unit-census law (`🌎️hub/🧪️tests/🌳️unit-census/🟦️.ts`, the body of `os-hub:unit-census-check`) | **3/3 pass** (384 s) after a hub lockfile resync. `bun repo run os-hub:unit-census-check` itself sat 24 min in `build:project-assembly`, so I stopped it with `📜️kill-tree.py` and ran its law directly. Two earlier direct runs failed with `--locked`, because peers added crates mid-run (e.g. `semio-s-artifact-stdio-semio-base-examples`) |
| Taxonomy (`verify taxonomy report`, scoped) | `directory-kind-unresolved` and `root-script` findings, at parity with sibling packages (`🖥️host`, `🖨️describe`) |

## 6. Not Done / Blocked

- **Day-2 compilation and the cold re-measurement of the split.** Blocked until the peer crates in §5 compile. The command to rerun is `ORT_LANE=after owned-runtime-crate/🏗️cargo.sh cargo build -p semio-framework-plugin-describe --bin semio-framework-plugin-describe --profile host-tool --timings`, then `⏱️timings-critical-path.py`.
- **`bun repo run` route of the census.** It did not get past project assembly in 24 min. The law itself passes (§5).
- **Fixture-dependent owned laws.** These are `owned_instance_open_tests` and the host count tests. They need the staged `semio_framework_plugin_host_test_component.wasm`, which is not present.
- **Concurrent visibility narrowing (day 1, 16:02).** An unidentified peer sweep turned `pub(crate)` into private on interpreter items the runtime uses (`checkpoint_layout`, `encode_json_input`, `operation::checkpoint`, …), across 🔌️plugin and os-kernel. I restored my files. A tool that maps files to crates by directory sees no users and narrows them. It must resolve `#[path]` mounts.

## 7. Cleanup

Private lanes `⚡️cache/ort/{before,after,check,test}` are deleted (the whole `⚡️cache/ort`). The ticket inputs under `owned-runtime-crate/` are kept:

- `🏗️cargo.sh`
- `⏱️timings-critical-path.py`
- `📏️source-admission.ts`
- `🧬️extract-runtime.py`
- `🔀️rewire-host.py`
- `🔀️rewire-consumers.py`
- `🪓️split-owned-runtime.py`
- `🗂️taxonomy-scopes*.txt`

## 8. Files

Paths are relative to `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/` unless absolute.

- **New.**
  - `🧬️guest-contract/{🦀️.rs, 📦️packages/🦀️rust/{Cargo.toml, 📋️project.json, 📜️script.ts}}`
  - `🧠️interpreter/🏃️runtime/{🦀️.rs, 📦️packages/🦀️rust/{Cargo.toml, 📋️project.json, 📜️script.ts}}`
  - `🧠️interpreter/🏃️runtime/🎛️operation/🦀️.rs`
  - `🧠️interpreter/🏃️runtime/🧪️tests/{🔬️owned-runtime, 🔬️owned-instance-open, 🔬️microsecond-clock}/🦀️.rs`
- **Moved.** `🖥️host/🚪️io/🎛️operation/{📥️input, 📤️checkpoint}` → `🧠️interpreter/🏃️runtime/🎛️operation/`, and `🖥️host/🧪️tests/{🔬️owned-runtime, 🔬️microsecond-clock}` → runtime tests.
- **Changed.**
  - `🧠️interpreter/🦀️.rs` (`INTERPRETER_SOURCE`, test heap witness)
  - `🧠️interpreter/📦️packages/🦀️rust/{Cargo.toml, 📜️script.ts}`. The facade `🦀️.rs` is deleted.
  - `🖥️host/{🦀️.rs, 📦️packages/🦀️rust/{🦀️.rs, Cargo.toml, 📜️script.ts}, 🚪️io/🪶️snapshot/🦀️.rs, 🧬️component-codec/🦀️.rs, 📥️cold-pair/🦀️.rs, ⏳️runtime/🦀️.rs, 🧵️shard/**, 🧪️tests/**, 🔁️lifecycle/🧪️tests/**, 🎟️context/🤝️admission/🧪️tests/🟦️.ts}`
  - `🖨️describe/{🛂️descriptor-emission/🦀️.rs, 🧪️tests/🔬️unit/🦀️.rs, 📦️packages/🦀️rust/Cargo.toml}`
  - `📦️packages/🦀️rust/📜️script.ts` (guest SDK law group)
- **Consumers.** `/🌎️hub/{Cargo.toml, 📦️packages/🦀️rust/Cargo.toml, 🗿️artifact-authority/🔏️trusted-catalog/**, 🧪️tests/{🔏️trusted-catalog-profile, 🧩️component-codecs}/**, 🧩️compositions/🖍️draw/**}`, mcp, run, renderer-wgpu sources and manifests, `/✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/**`.
- **Workspace.**
  - `/Cargo.toml` (members, workspace deps, dev profile)
  - 137 nested `Cargo.toml` dev-profile entries
  - 23 lockfiles
  - `/.vscode/launch.json`
