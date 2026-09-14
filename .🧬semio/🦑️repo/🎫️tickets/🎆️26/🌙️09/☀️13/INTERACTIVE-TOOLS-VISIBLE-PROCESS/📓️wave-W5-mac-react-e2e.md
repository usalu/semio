# 🪣️ Wave W5 — puzzle 3d fill end to end in the React shell (macOS, 2026-09-14)

Session ⚪a319b1ae (Opus 5). Runtime gate on `:6013` (`serve-puzzle3d-react-dev`), headless probe `🔍️w5-fill-run-probe.ts`
(init-script sampler of the perspective window's `data-tool-run-*`, vite HMR websocket routed to a mock, tour veil
cleared by Skip only). Diagnostics: `🔍️w5-pace-diagnose-probe.ts`, `🔍️w5-echo-map-probe.ts`.

## Result (probe run 12, `🗑️generated/W5-mac-react-e2e/fill-run-2026-09-14T19-36-17-920Z.*`)

18 PASS / 1 FAIL, 0 hard faults.

| Verdict | Evidence |
|---|---|
| ready group offers Start | panel "Ready to start" + Start with the active Fill tool |
| attempts appear one by one | 10 distinct record counts in 8 s, largest jump 3 per 200 ms sample |
| candidate under test visible | `testing` record on screen in 15 of 32 samples |
| danger + success + provisional | 99 success / 340 danger, 99 provisional pieces, committed document and history untouched |
| progress aria | `Choosing vortex and object (2/5): 99 of 100 objects (99 %)` |
| abort | provisional retired, document + history untouched, trace kept |
| pause / step | paused run holds; one step shows exactly one visible unit (the next candidate under test) |
| chords | `mod+enter` starts, `mod+.` aborts |
| finalize / undo | 12 of 12 placed, finalize commits 12, one undo removes all |
| **FAIL** every placed piece provisional at complete | the 100-object run reached 99/100 at the 900 s budget (≈ 2 s per attempt under load) |

Screenshot `…-complete-default.png`: collisions red, fitting pieces translucent green provisional, panel with status,
progress and Pause/Step/Abort/Finalize.

## Defects found and fixed

1. `DuplicateSiblingKey` on the ToolRun panel (trace key re-upserted testing → verdict) killed every refresh → `recent_trace` moves/retires keys; law `tool_run_panel_lists_a_re_upserted_trace_key_once_at_its_newest_position`.
2. No Start affordance → framework ready group `framework.toolRun.ready` (label `readyToStart` Rust/TS/schema/fixture); law `tool_run_panel_offers_start_for_the_active_run_tool_until_its_run_exists`.
3. Every ToolRun chord opened a staged form → `actionRequiresStagedForm` ignores hidden args; engine-contract case.
4. Provisional pieces rendered as committed → `Puzzle3dInstanceResidency::refresh(fixture, provisional_entities)` stamps `provisional`; finalize law asserts it.
5. Everything shown at once (dev request) → viewer pace `ViewModel.toolRunUnitsPerSecond` (React host default 20), fuel-1 paced stepping, one outstanding `toolRunPace` wake, presentation gate on the renderers' echoed trace cursors (wake view state carries them), the releasing wake shows the next unit and returns its refresh scope; fill job makes a constructed candidate its own visible unit (`deferred` observations). Laws: `a_paced_run_shows_one_unit_per_interval_and_waits_for_the_host_wake`, `a_paced_run_waits_until_its_renderer_presented_the_last_unit`, `fill_run_job_step_with_one_unit_of_fuel_shows_each_candidate_before_its_verdict`; view-context fixture rows; integer-carrier law now covers actor-ingress `FromValue`.

## Gates run

plugin `tool_run` 34/34 (2 ms overlay bench is load-sensitive, passes alone), `semio-framework-tool-run` 27/27, tool-run TS
conformance 21/21, puzzle 3d `fill_run` 16/16 and `-- fill` 51/52 (2 ms Nakagin ceiling under load), manifest view-context
12/12 + TS host-context law, engine-contract staged-form rule 4/4, framework-rs generate green.

## Open

- Real pace is ≈ 1 visible unit/s under this machine's load (presentation round trip), so 100 placements take ~15 min; a panel speed control (persisted local-only preference, incl. unpaced) is not built yet.
- ToolRun panel "Attempts" tree renders `NO DATA` in React (rows built by the guest are not shown).
- wgpu shell: ignores `DispatchAction.delay_ms`, sessions stay unpaced (`tool_run_units_per_second: None`); contract gates (e) count change mid-run, (f) second-tab rebase, (h) wgpu evidence not run.
- `d-finalize-reaches-history` probe verdict is weak (undo proves the history row).
