# Slider invalidation fix

## Symptom
Moving a slider did not update the 3D preview and did not show waiting/processing status on flow nodes.

## Cause
After FlowEvalSession, throttled refreshUi during (and at the end of) a slider gesture still ran syncFlowSessionEvalFromScene, applying stale plugin evalJson onto the live WASM session. That installed a fake converged baseline (new slider seeds + old channel outputs), cleared computing chrome, and left pending_eval_widget_ids empty so flowEvalTick never re-armed.

## Fix
1. React FlowGraphCanvasHost: skip all scene resync while the gesture is active; on pointer-up sync structure only (skipFixture) and never apply stale eval/status — commitFixture drives the plugin tick chain.
2. FlowHost::apply_eval_outputs_json: probe convergence before establishing baseline; stale outputs only refresh computing chrome.
3. WGPU sync_flow_host: refresh computing chrome when fixture semantics change without a fresh status payload.

## Tests
- apply_eval_outputs_json_skips_baseline_when_outputs_stale_for_seeds
- flow_eval_session_*
- procedural-3d-ui sphere_cut_example_computing_chrome_clears_once_ticks_converge
