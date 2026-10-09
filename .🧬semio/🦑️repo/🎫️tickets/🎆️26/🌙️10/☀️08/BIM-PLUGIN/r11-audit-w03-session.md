# 🔎️ R11 Audit — w03-session (WP-03 + z-consumers) and z-incremental

Status: w03-session **NOT STARTED**; z-incremental **NOT STARTED**.

## WP-03 — consumers still on separate inference paths
| Consumer | Path today | Location |
|---|---|---|
| Graph session `ModelInferenceSession` | exists, test-only | `🕸️model-graph/📡️session/🦀️.rs` (update, refresh, inference, report) |
| Editor | own `ModelInferenceSession` (same name!), per-field `compute_*` table, thread_local `SESSIONS`, `ModelDiff::between` per refresh | `✏️editor/🔮️inference/🦀️.rs:50-89`; `with_inference` callers `🖱️canvas-pointer-down:41`, `✏️editor/🦀️.rs:605,799,803`, `🧵️gestures/🦀️.rs:329`; `🧩️area:107,133` (`rooms_of`) |
| Viewer | thread_local `MEMO` + snapshot clone + deep compare | `🖌️render/🦀️.rs:116-141`; callers `👁️viewer/…/🗺️plan:48`, `🧊️world:55` |
| IFC export | `ModelInference::infer` | `🚪️io/📤️export/🏗️ifc/🦀️.rs:215-236` |
| CSV export | `ModelInference::infer` | `📊️csv/🦀️.rs:111` |
| glTF / SVG export | fresh uncached graph per call | `🧊️gltf/🌳️scene:111`, `🎨️svg:59`, `📏️projection:71` |
| IFC import | thin projection | `📥️import/🏗️ifc:94` |
| Oracle helper | `ModelInference::infer` | `🚪️io/📝️text/📸️snapshot:61` |

Progress/cancel: framework has `InferenceCursor` (`fraction`, `cancel`, `is_cancelled`, `infer_field_step`,
`🧰️framework/🛍️products/💻️os/🔨️modules/💡️inference/🦀️.rs:364-448`) and `ArtifactCommandWorkStep::Progress` +
`cx.is_cancelled()` (`🔌️plugin/🧵️retained-command:85-91,455`). The graph session uses unbounded `resolve_ready(…)` and
panics on error. No BIM code emits progress. `Serializer::serialize` has no progress/cancel.

Leftovers: `[DEBUG]` timers in `🕸️model-graph/🧮️compute/🦀️.rs:213-220` (`DEBUG_TIMES`); live test `zz_debug_timing` in
`🕸️model-graph/🧪️tests/📈️incremental/🦀️.rs:47-77`.

## z-incremental
Target < 15 ms (release) for one wall move in 511 walls / 64 windows; measured 353 ms (88/2385 nodes computed, hashing
dominates). Engine `infer_field_after_diff` gates field-level only; `run_to_end` walks the whole plan with hashing, clones
per node step, string-based `result_root`, full values-map clones. `ModelDiff::touches` already emits per-entity paths;
`TouchedPaths` supports prefix intersection (`📡️spr/🎮️command:41-80`); unused per key. `bench-base.txt` is a failed build
log with no timings.

## Coordinator decisions
- Cancellation design: (a) a stepped, cancellable session update built on `infer_field_step` + `InferenceCursor`
  (progress fraction, cancel keeps finished values, errors are values not panics). Render/export/recompute run as
  `ArtifactCommandWork` jobs emitting `ArtifactCommandWorkStep::Progress` and honouring `cx.is_cancelled()`. Exports take
  the session's inference as input. Do not change the framework `Serializer` trait.
- One name: the graph session is `ModelInferenceSession`; the editor's own type is deleted.
- Ownership split: z-incremental owns the framework engine (`💡️inference/🦀️.rs`) and `🕸️model-graph/🧮️compute` per-key
  dependency mapping + the `[DEBUG]` cleanup; w03-session owns `📡️session` API (stepped/cancellable) and all consumers.
