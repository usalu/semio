# 📦 Result ENGINE-CANVAS-PACKET-DRIFT (slice `ecp`, 2026-10-16)

Executor: Sonnet 5.5, foreground, no sub-agents, no modifying git command, no ticket tool. `F` = this folder. `EC` = `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas`. Logs: `F/🗑️generated/ecp-*` (oracle runs write `ecp-rii-<label>-*`). Input: `F/ecp-tools/ecp.json` (4-crate family for the E0451 scanner). Pre-edit copy of the law file: scratchpad `ecp-standalone-pre.rs`.

## 0. Verdict

**GREEN against the oracle, UNCOMPILED.** The `scene_retirement_faulted` field was removed on purpose, so the fix is the removal from the one remaining reader, not a new field.

* Oracle `E0560 struct EngineCanvasPacket has no field named scene_retirement_faulted` (`EC/🧪️tests/🧊️wgpu-standalone/🦀️.rs:110`, crate `engine_attach_tests`, test cfg): **1 -> 0**, and **0 new rows from my change** (section 4.1). Canary E0308, E0616, E0624 reached in both runs, in lib and test.
* No code outside `F` mentions `scene_retirement_faulted` any more (`git grep -F`, section 4.2).

## 1. Found state

* Definition: `EC/🎯️targets/🧊️wgpu/📦️prepared-engine-canvas/🦀️.rs:21` (`pub struct EngineCanvasPacket`, mounted by the `engine-registry` crate through `#[path]`). Its fields: `surface`, `document_generation`, `scene_revision`, `metrics_generation`, `scene: ManuallyDrop<canvas::Scene>`, `scene_retirement: ManuallyDrop<canvas::CanvasSceneRetirement>`, `clear`, `width`, `height`. No fault field.
* Constructors: two in the same file (`publish_staged` line 227, test-only `publish_reserved` line 277, both already `scene_retirement: Default::default()`), one in the law (`raster_lifecycle_packet`, `EC/🧪️tests/🧊️wgpu-standalone/🦀️.rs`). Readers of the field: only the law fixture.
* History (read-only `git log -S`, `git show`): the field arrived in `0a0bb743800` / `f39d4b0db3b` (2026-09-08/10) together with `scene_retirement: Option<canvas::OpaqueSceneRetirementToken>`; commit `0c3e78944af` (2026-10-14, the original-retirement migration) **deleted both** from the struct, replaced the token by the `CanvasSceneRetirement` cursor, made `close_step(cx)` return `Result<InteractiveJobCloseStep, OriginalTurnFailure>` and moved fault reporting into that `Err`. It updated both production constructors and left the law fixture (a third constructor) behind: a one-sided removal, not a rename.
* Intended end state: no flag. A faulted retirement is an `Err` from `close_step` / `next_close_demands`; terminality is `surface.id.terminal_is_empty() && scene_retirement.terminal_is_empty(&scene)`. Adding a dummy field would resurrect a state the owner deliberately retired, so none was added.
* A second reader of the old shape existed in TypeScript: `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🧹️retirement/🧪️tests/🧪️source-contract/🟦️.ts` (the language-agnostic source contract for `fixture.scene.retainedConsumers`) required `scene_retirement: Option<canvas::OpaqueSceneRetirementToken>` and `advance_opaque_scene_retirement` in the packet source. The packet does not cross the wire (no serde, `ManuallyDrop` fields), so this source contract is its TS twin.

## 2. Edits

1. `EC/🧪️tests/🧊️wgpu-standalone/🦀️.rs` `raster_lifecycle_packet`: `scene: ManuallyDrop::new(canvas::Scene::new())`, `scene_retirement: ManuallyDrop::new(canvas::CanvasSceneRetirement::default())`, the `scene_retirement_faulted: false` line removed. (`scene` and `scene_retirement` had the pre-`0c3e` types as well; the oracle hid them behind the stub layer, the E0560 was the visible symptom.)
2. Same file, new Rust law `engine_packet_retirement_is_the_scene_cursor_and_terminal_only_after_the_identity_closes`: a fresh fixture packet is not terminal; `next_close_demands(4096)` is `Ok` with `depth == 1`; after `surface.id.close_step()` the packet is terminal (so its `Drop` assertion is satisfied) and `next_close_demands` is `Ok` with depth, copy and release bytes all 0. It needs no `StepContext`.
3. TS twin `🟦️.ts`: the `engineCanvasPacket` consumer reads `EC/🎯️targets/🧊️wgpu/📦️prepared-engine-canvas/🦀️.rs` directly and requires `scene_retirement: ManuallyDrop<canvas::CanvasSceneRetirement>` and `self.scene_retirement.close_step(&mut self.scene,grant)`; the loop takes the advance marker from a fourth tuple element (the two token-based consumers keep `advance_opaque_scene_retirement`). I also corrected the canvas source path of line 18 to `🔨️modules/🖼️canvas/🎬️scene/🖌️renderer/🦀️.rs` (where `retire_vello_fragment` lives now), because the file died there before it ever reached the consumer rows.

No manifest, fixture JSON, registry or launch.json change was needed.

## 3. Not done on purpose (hand-offs)

* The same law file and `🕸️wgpu-node-graph` / `🧩️wgpu-engine-surfaces` still call `packet.close_step()` with no argument and treat it as `bool` (`wgpu-standalone:76,78`, `wgpu-node-graph:144`, `wgpu-engine-surfaces:877..997`). That is the other half of commit `0c3e78944af` (`close_step(cx)` -> `Result`), not the field. A port needs `with_engine_close_context`, whose body in `engine-registry/🧰️fixtures/🦀️.rs:18` still uses the old 6-argument `StepContext::new` and 2-argument `StepBudget::new` (current signatures: 7 and 3 arguments, `🧵️job/🦀️.rs:779,182`). The oracle's stub layer hides these rows, a compiler will not. Owner: the retained-step migration slice.
* Also from the same migration: most tests that create `raster_lifecycle_packet` drop a non-terminal packet, and `impl Drop for EngineCanvasPacket` asserts terminality (`prepared-engine-canvas/🦀️.rs:44`). The new law avoids that by closing the identity first; the older tests need the same treatment when they are ported.

## 4. Verification (every line names the command and its real result)

### 4.1 Oracle (`pos.py`, family `rii-tools/rii.json` = 104 crates incl. engine-registry, engine-attach, engine-attach-tests, engine-board; lib and `--test`; absolute, `--no-base`)

`cd F/pos-tools && POS_PREFIX=ecp python3 pos.py run ../rii-tools/rii.json --cfgs lib,test --label <L> --no-base`

| run | tree | row `E0560 ... scene_retirement_faulted` | canary (lib / test) | rows in the report |
|---|---|---|---|---|
| `b0` | before my first edit (union built at start; the row is at `wgpu-standalone/🦀️.rs` line 110, i.e. the pre-edit file) | **1** | E0308 E0616 E0624 / same | 421 |
| `a1` | final tree | **0** | E0308 E0616 E0624 / same | 421 |

Row multiset diff of the two reports (key: code, unit, file, lines, message): only in `b0` = that one E0560; only in `a1` = `E0599 [os_renderer_wgpu_native_probe] .../native-socket-probe/../🪶️sqlite/🦀️.rs:43 no method named encode_native_snapshot`. That row is in a crate and file I never touched (a peer edited the tree between the two runs): **0 new rows attributable to this slice**. The remaining rows of the engine-registry / engine-attach / engine-board units are the documented baseline classes (`element_contracts` / `asset_authority` / `map_tiles` extern-alias artifacts and stub-layer E0308/E0369, identical in `b0` and `a1`).

### 4.2 Static gates

| check | command | result |
|---|---|---|
| E0451 scan | `cd F/rpv-tools && POS_PREFIX=ecp python3 lit.py ../ecp-tools/ecp.json ecp-lit` (engine-registry, engine-attach-tests, engine-attach, engine-board) | `0 candidate E0451 sites; 35 brace structs; 35 definitions; 24 files in the module tree` |
| no stale reader | `git -c core.quotepath=false grep -n -F scene_retirement_faulted -- '🧰️framework' '✏️s' ':!*.md'` | 0 hits (before: 1, the law) |
| other packet constructors | `git grep -n -E 'EngineCanvasPacket \{|scene_retirement:'` over `🧰️framework` | 3 literals (2 production, 1 law), all with the same 8 fields now |
| rustfmt parse | `rustfmt +nightly-2026-07-20 --edition 2021 --config skip_children=true --emit stdout <law file>` | exit 0, no parse error |
| rustfmt style | `rustfmt +nightly-2026-07-20 --edition 2021 --style-edition 2024 --config-path rustfmt.toml --config skip_children=true --check <law file>` | no `Diff in` hunk |
| path length | `printf %s <path> \| wc -c` for the two edited files | 173 and 146 bytes (limit 240) |
| TS twin, consumer rows | scratch copy of the `sceneConsumers` block run with `bun` against the three real sources | `consumers ok` |
| TS twin, whole file | `perl -e 'alarm 100; exec @ARGV' -- bun '🧰️framework/.../🧪️source-contract/🟦️.ts'` | exit 0, no output. Before my edit the same command failed at line 19 (`AssertionError`, canvas source path), so the test discriminates. |

## 5. Real-check commands (for the compile queue; NOT run by me)

```
cd /Users/ueli/Documents/semio && bun repo cargo --store=shared --configuration=prototype -- check -p semio-framework-os-renderer-wgpu-engine-registry -p semio-framework-os-renderer-wgpu-engine-attach-tests --lib --tests --keep-going --message-format=short
cd /Users/ueli/Documents/semio && bun repo cargo --store=shared --configuration=prototype -- test -p semio-framework-os-renderer-wgpu-engine-attach-tests --lib engine_packet_retirement_is_the_scene_cursor_and_terminal_only_after_the_identity_closes
```

Expected first compile faults of the law file, all from section 3 (not from the field): `E0061` for `packet.close_step()` (lines 76 and 78), and `with_engine_close_context` not compiling until its `StepContext::new` / `StepBudget::new` calls are updated. The `Drop` of a non-terminal `raster_lifecycle_packet` in the GPU-candidate tests will panic at run time until they close the identity first.

## 6. Files

Edited: `EC/🧪️tests/🧊️wgpu-standalone/🦀️.rs`; `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🧹️retirement/🧪️tests/🧪️source-contract/🟦️.ts`.
Created: `F/RESULT-ENGINE-CANVAS-PACKET-DRIFT.md`, `F/ecp-tools/ecp.json`. Logs: `F/🗑️generated/ecp-*` (kept).
Deleted: none. Peer changes in the same files (the `crate::collect_fixture_actions` -> `semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions` rewrites in the law file, the `readModuleTreeSource` import in the TS file) were left as found.
