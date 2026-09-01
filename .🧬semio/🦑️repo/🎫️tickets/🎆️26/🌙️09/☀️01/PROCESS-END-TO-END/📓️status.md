# 🏭️ Process End to End — status

## Target
`bun run dev:process:3d` → `bun ./📜️script.ts dev process 3d` → nx `@semio-tech/framework-os-dev:dev -- process3d`,
react renderer on **port 6022** (playground metadata: `✏️s/🔌️plugins/🏭️process/📦️packages/🦀️rust/Cargo.toml:18-22`,
app `s.process.process3d@1/*#editor`).

## Confirmed findings (read from source, not assumed)

### F1 — the shipped wasm is 5 days stale
`🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/🔌️plugin-modules/process/semio_s_plugin_process_component.core.wasm`
is dated **Aug 27 16:02** (50 MB); `🛂️descriptor.semio` was regenerated Sep 1 14:13. Same failure shape the
sourcing ticket hit: the browser loads a plugin built against a different framework. A `wasm32-wasip2` build is
the truth gate and is running.

### F2 — both example documents load with an EMPTY scene  ← the "empty window" root cause
`Process3dSnapshot` (`🧬️schema/📸️snapshot/🦀️component.rs:22-52`) carries, since ticket
`26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave 4, two **inline, authoritative** payload fields beside the
composed child handles:

| field | role |
| --- | --- |
| `stock_payload: Stock` | the real workpiece solid — **the only thing the renderer reads** |
| `step_payloads: Vec<ProcessStep>` | the real timeline — **the only thing the stepper reads** |
| `stock_solid` / `steps` / `tool_solids` | `ArtifactChild<…>` handles; composition identity only, never resolved (no `LinkResolver` exists) |

The DSL printer emits ten lines including `stockPayload=` and `stepPayloads=`
(`🧬️schema/📸️snapshot/🦀️component.rs:140-153`). **Both shipped example fixtures emit only eight** — they predate
wave 4 and were never regenerated:

- `📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` (the `timber-beam-joinery` / `default_document()` fixture) —
  fields present: `workshop, stockId, stockLabel, stockPose, stockSolid, steps, toolSolids, resolvedUpTo`.
- `PROCESS_3D_PLATE_EXAMPLE_TEXT` (`🧬️schema/📸️snapshot/📝️text/🦀️component.rs:14-25`) — same eight.

`parse_process3d_snapshot_body` (`…/📸️snapshot/🦀️component.rs:155-190`) starts from
`empty_process3d_snapshot()` and only overwrites the lines it sees, so the missing lines silently fall back to
`ProcessWorkingScene::default()` → **stock = a 1×1×1 unit box, steps = `[]`**.

Consequence in the app (`✏️editor/🦀️component.rs:997` boots `default_document()`):
- `🪚️workpiece` renders `processed_mesh(scene, resolved_up_to)` over that scene
  (`…/🪟️windows/🪚️workpiece/🦀️component.rs:92-107`) → a unit cube labelled "Timber Beam", not a beam.
- the engagement stepper (`…:151-162`) reports **0 steps**, so the timeline is dead.
- the existing tests never catch it: `default_document_parses_timber_example`
  (`🧬️schema/🦀️component.rs:954-958`) asserts only `steps.child_id` is non-empty, and
  `render_world_scene_contains_processed_mesh` (`✏️editor/🦀️component.rs:1760`) only greps the string
  `"processed"` — which the fallback mesh also produces.

### F3 — the seven step-scoped mutations are declared no-ops
`🧬️schema/🧬️mutations/{🌱create-step,🗑️delete-step,🏷️rename-step,🔘change-step-enabled,🧷change-step-origin,📐replace-step-measure,🔀reorder-steps}`
each return `MutationOutcome::empty().warn("mutation.no-op", "…pending a link resolver for the composed steps child.")`,
and `🧪️tests/mutate-process3d-1/🦀️.rs:61-69` lists all seven under `UNOBSERVABLE`. That reasoning is now stale:
`step_payloads` is the authoritative durable record and is inline, so these verbs can be implemented against it
(re-minting `steps`/`tool_solids` exactly as `process_working_scene_to_snapshot`
(`🗿️artifacts/🧊️process3d/🦀️component.rs:786-820`) already does) without any `LinkResolver`.
**Today the app cannot add, remove, rename, reorder, enable or re-measure a single process step.**

### F4 — not ours
`semio-s-plugin-stdio` still has no `🔣️descriptor.json` in `🔌️plugin-modules/stdio/` (last successful build Aug 18;
its component link overruns the 1 000 000-function ceiling). Process depends on stdio only for **types at compile
time** — it does not link stdio's wasm — so this is boot noise, not a process blocker. Peer-owned.

## Plan
1. **P0** rebuild `semio-s-plugin-process` for `wasm32-wasip2`; fix whatever no longer compiles.
2. **P1** regenerate both example fixtures with real `stockPayload` + `stepPayloads` via the documented
   fixture-regeneration technique (real `process_working_scene_to_snapshot` + `print_dsl()`, never hand-transcribed),
   and add assertions that bite (non-degenerate stock solid, non-empty timeline).
3. **P2** implement the seven step-scoped mutations against `step_payloads` + re-minted children; retire the
   `UNOBSERVABLE` list and regenerate the committed mutation vectors.
4. **P3** audit the four panels and the viewer for empty renders.
5. **P4** boot :6022, drive the browser, confirm every window is non-empty and the commands round-trip.

## Machine note
The shared cargo target dir is heavily contended (peer sessions running `cargo build -p semio-s-plugin-cad`,
workspace `cargo check`, two `cargo test` runs). Builds block on the target-dir lock; poll, do not kill.
