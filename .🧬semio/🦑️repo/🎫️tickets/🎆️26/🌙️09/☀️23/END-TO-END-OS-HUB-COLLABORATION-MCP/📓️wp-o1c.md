# WP-O1c — Plugin Registry Check Green + Renderer-WGPU Build

Slice: O1c. Scratch: `.tmp-ticket/wp-o1c/`. Private cargo: `.tmp-ticket/wp-o1c/target`.

## Status

| Item | State | Evidence |
|------|-------|----------|
| plugin-registry taxonomy (~340 O1) | PASS | plugin-check.txt — 0 taxonomy findings (soft-require IO/examples) |
| wfc descriptor JSON/pack | PASS | fix-desc.txt — pack re-encoded; self-hash OK |
| norm catalog hash sync | BLOCKED | needs generate; catalog stale after wfc fix (plugin-check2.txt) |
| plugin-registry:generate | QUEUED | fleet wasm mutex; behind s14/o3b/gj3b (plugin-generate.txt) |
| plugin-registry:check green | FAIL pending generate | EXIT:1 stale plugins.json |
| Shell pack + WindowKinds.first | PASS | wgpu-check6.txt lib compile |
| handle_shell_hit InputState arity | PASS | wgpu-test-compile3.txt; dock/pane/context hit tests OK |
| m10b format string | PASS | store-sync Ack Rejected compile |
| framework-renderer-wgpu:test compile | PASS | wgpu-test-compile3.txt |
| framework-renderer-wgpu:test run | PARTIAL | 1282/1371 pass; ~89 peer UI fails (e.g. conflict rowCount 3!=1) unrelated to hit arity |
| wasm32 ui wgpu-engine check | QUEUED | same wasm mutex |

## Clusters (O1 gate-3 → fresh)

O1 listed ~341 taxonomy violations. Fresh `check` shows **0 taxonomy tree findings** (IO/examples soft-required in validateTaxonomyTree). Remaining hard gate: descriptor/catalog freshness.

## Fixes

1. Shell: `dsl::os_pack::json::to_json_string(&events.to_vec())`; `window_kinds.first().id.clone()`
2. store sync: `batch_id={batch_id} reason={reason}`
3. Six Shell tests: second arg `&InputState::<ActionDescriptor>::default()`
4. wfc source descriptor pack←JSON + descriptorSha256

## Gaps

- Wasm mutex fleet queue (s14 → o3b → gj3b → o1c); generate waiter pid still queued.
- ~89 wgpu runtime fails look like settings/WindowOptions peer drift (conflict TreeItem count), not our hit patches.
