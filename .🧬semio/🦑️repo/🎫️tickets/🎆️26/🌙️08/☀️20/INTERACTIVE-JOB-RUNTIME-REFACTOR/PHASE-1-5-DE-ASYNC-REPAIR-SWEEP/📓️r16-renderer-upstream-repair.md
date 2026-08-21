# R16 Renderer Upstream Repair

## Scope

This continuation repaired the Phase 1.5 de-async fallout that prevented the mounted native renderer crate from reaching its own compiler gate. It reused `🔧️r13-deasync-codemod.ts`; no additional script was created. Edits were selected from rustc JSON primary spans or from audited, pure codec/transform files. The Phase 5 UI/WGPU transaction migration was not started.

## Baseline blockers

- `semio-framework-plugin` contained `pub async async fn`, `awaitue_to_ui_value`, and three `.await.expe.awaitct` corruptions. The first targeted plugin library check reached zero errors after those exact repairs.
- `semio-framework-os-infinite --lib` initially reported 820 compiler errors.
- The first mounted renderer attempt was blocked before renderer compilation by the same upstream graph.

## Codemod evidence

The ticket-local codemod was extended, rather than replaced, with compiler-span handling for E0728, a `deasync-pure-file` mode, `--lib`, and test attribute synchronization. Its original 17 self-tests passed after the extension.

Fresh compiler-guided/pure-file repair counts recorded during the sweep:

- Infinite E0728: 239 physical illegal `.await` tokens removed across six files; diagnostic baseline 230 to 0.
- Geometry engine: 395 edits (129 async definitions, 239 awaits, 27 async-test attributes); library check reached 0 errors.
- Graph pure files: 1,588 edits total (`engine` 753, `drawing` 112, `dsl` 551, `manifest` 62, `algorithms` 110); graph library check reached 0 errors after compiler-span cleanup of stale `Box::pin`/`resolve_ready` bridges.
- Replication pure files: mutation 66, codec 159, causal 194, wire 276, ids 4, followed by compiler-guided callsite repairs; replication library check reached 0 errors.
- OS pack/store/surface pure files: pack value 314, pack facade 26, terrain 44, tiled-map 312, paint 135, node-graph 111, editor 234.
- Renderer-mounted structured runs: `r13-emz5fdpv` kept 481 edits; `r13-47498ulw` kept 2; `r13-h0era15s` kept 160. The compiler error count then fell 30 → 9 → 2 → the directory-client-only blocker described below.

The generic product-neutral pack format remains async where it performs real `PackSource`/`PackSink` I/O. Only audited in-memory OS pack/store codec helpers were made synchronous; real generic calls retain explicit `crate::os_io::resolve_ready` boundaries.

## Source repairs

- Corrected the malformed plugin tokens listed above.
- De-async'd pure geometry, graph, replication, OS pack/store, surface, editor, schema, workflow, and config mutation paths only where compiler evidence required synchronous use.
- Removed stale `resolve_ready` wrappers after `ArtifactPack`, timestamps, and pure pack helpers became synchronous.
- Made `pack_value_to_base64`, `pack_value_from_base64`, `decode_scene_pack_field`, and `scene_field_json_text` synchronous pure transforms and cleared their exact stale awaits.
- Rewired opening-config dispatch to its pure leaf diff/inverse functions and removed the obsolete `block_on` inverse bridge.
- Preserved async boundaries for real host/runtime I/O.

## Checkpoint schema coordination

Phase 2 supplied the clean actor wire contract. This sweep updated:

- kernel `TurnStatus::CheckpointReady { checkpoint: semio_framework_actor::JobCheckpoint }`;
- WIT `checkpoint-ready(job-checkpoint)` with `state` and `applied-progress`;
- guest reactor kernel-to-WIT conversion;
- root plugin-host WIT-to-kernel conversion;
- the renderer actor lifecycle callsites to construct stable `JobOperation` values and real `JobCheckpoint` objects for Suspend/Resume.

No layer fabricates an empty checkpoint or drops `applied_progress`. The shard mapping/tests remain owned by the Phase 2 agent and were coordinated directly.

## Commands and results

```text
bun .🧬semio/.../PHASE-1-5-DE-ASYNC-REPAIR-SWEEP/🔧️r13-deasync-codemod.ts selftest
PASS — 17 original tests after the E0728 extension

cargo check -p semio-framework-plugin --lib --message-format=json
PASS at the initial malformed-token gate — exit 0, 0 errors

cargo check -p semio-framework-os-infinite --lib --message-format=json
PASS after the repair cascade — exit 0, 0 errors

cargo check -p semio-framework-geometry-engine --lib --message-format=json
PASS — 0 errors

cargo check -p semio-framework-graph --lib --message-format=json
PASS — 0 errors

cargo check -p semio-framework-replication --lib --message-format=json
PASS — 0 errors

cargo check -p semio-framework-os-renderer-wgpu --lib --message-format=json
REACHED renderer dependency graph. Structured counts during the final cascade: 30, 9, 2, then 7 directory-client errors while P1i was actively rewriting that owned file.

bun nx run @semio-tech/framework-renderer-wgpu:test-quick
REACHED native Cargo compilation; final verdict pending the concurrent P1i directory-client contract repair.
```

The latest rechecks of `semio-framework-plugin`, `semio-framework-os-infinite`, and `semio-framework` each currently stop on the same single concurrent directory-client `T: Clone` error at line 277, not on plugin/infinite de-async residue.

## Concurrent blocker

`/root/p1_runtime_gate` confirmed active ownership of `📇️directory/🔌️client/🦀️component.rs`. Its final contract is native `DirectoryTransport: Send + Sync`, async `http`, synchronous bounded `open_ws(ctx, url, timeout_ms)`, and synchronous `DirectoryWsConnection::{send_text, try_recv_text, close}` with a finite owned `DirectoryStream` state machine. This sweep deliberately did not edit that file while its owner is active.

The mounted renderer verdict and the direct compile proof for `assert_send::<AppRuntime>()` will be appended after that owned blocker clears.

## Workspace hygiene

An earlier report attempt accidentally created a literal-backslash duplicate tree. The canonical Phase 3 report was confirmed present, then only the accidental duplicate tree was removed. Canonical `.🧬semio` content was not touched by that cleanup.
