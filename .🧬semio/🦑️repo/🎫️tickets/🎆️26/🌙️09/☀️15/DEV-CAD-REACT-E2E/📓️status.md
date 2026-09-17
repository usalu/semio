# Status: Dev Cad React E2E

**Status:** closed 2026-09-17 (phase 3: hover latency)
**Goal:** Get dev cad react working end to end (🛠️dev📐️cad⚛️react, port 6020), including every model-definition interaction JSON running with its preview.
**Bookkeeping:** manual on disk — repo MCP failed to connect (connection closed) in every session.

## Result
- Phase 1 ([cad-react-e2e-2026-09-16.md](./📓️cad-react-e2e-2026-09-16.md)): build → activate → serve → boot → real Concrete Forest geometry + reference image in all four panes → `cad` domain picks (viewport, Artifact tree) → inspection → reference mutations with undo → gumball arming.
- Phase 2 ([cad-interactions-e2e-2026-09-16.md](./📓️cad-interactions-e2e-2026-09-16.md)): all 60 `🕹️interactions/*.json` assets are the only statechart (legacy building statechart retired, 11 `aec.building` assets embedded and given `display` blocks), `engagementSubmit`/`worldPointerDown`/`engagementPossibleSelect` migrated to the retained artifact route, canonical session JSON, Enter = `confirm`, one history item per engagement (framework config-lane coalesce fix), echo-safe Action line (framework shell fix). Browser battery: Box, Sphere, Cylinder, Line, Circle, Arc, Polyline, ControlPointCurve, InterpCrv, Plane, Move, Rotate, Scale3D, Copy, Mirror, Place Column/Wall/Beam/Slab, Construct External Wall/Roof, Reinforced Concrete Column / One Way Slab all commit with their preview lane painted, ⌘Z removes the committed object, 0 console faults.
- Phase 3 ([hover-latency-2026-09-17.md](./📓️hover-latency-2026-09-17.md)): hover highlight 450–590 ms → 5–40 ms painted (local raycast paint in `World3dHost`, `worldHoverPaintIdV1`), guest echo 715 → ~330 ms (`shellReducer` identity bail-out, cad mesh-lane cache per materialized scene, cad `interaction_scope`); the remaining echo cost is framework-structural (reserved-job turns, whole-surface republish, redundant post-action refresh) and itemised in the report.
- Native: `semio-s-artifact-cad-cad --lib` 331 passed / 19 failed (phase-1 harness-debt families + one peer in-flight `setContributions` wire test), `semio-s-plugin-cad` 5/5; 57/60 assets drive to a commit outcome natively.

## Open
- Hover echo structural items (framework): the reserved-job shape costs three guest turns per pure interaction write; `paged_text_carrier` re-chunks unchanged lanes and the host re-admits the whole surface; the host's post-action refresh re-renders every scoped surface the guest's dirty pass already published. Native warm cad scene build is 1.7 ms — the 70–80 ms guest turns are reactor/patch overhead.
- Framework-wide 64-edit ledger per store (`ARTIFACT_HISTORY_LEDGER_CAPACITY`, no compaction) bounds a document session to ~30 committed interactions; a coalesced engagement keeps every amended snapshot in one edit.
- `interaction.call` mode choosers, `kernel.query`, `assignExtrusionDistance` and the 17 `Unsupported` commit families (booleans, edits, lofts/sweeps, features, measures, anchor).
- Retained-audit TS oracle needs `$defs.CadRetainedJobs` back in `🧬️schema/🔣️.json` (pre-existing).
- Document-tree paging navigation (`setPanelPage` exists now; the tree still closes with a plain `+N` row).

## Notes
- 2026-09-15: [dev-cad-react-e2e-2026-09-15.md](./📓️dev-cad-react-e2e-2026-09-15.md), [cad-ui-surface-capacity-2026-09-15.md](./📓️cad-ui-surface-capacity-2026-09-15.md) (superseded).
- Probes: [🐍️console-dump-probe.mjs](./🐍️console-dump-probe.mjs), [🐍️cad-interact-probe.mjs](./🐍️cad-interact-probe.mjs), [🐍️cad-tree-reference-probe.mjs](./🐍️cad-tree-reference-probe.mjs), [🐍️cad-utility-probe.mjs](./🐍️cad-utility-probe.mjs), [🐍️cad-interaction-run-probe.mjs](./🐍️cad-interaction-run-probe.mjs), [🐍️cad-hover-latency-probe.mjs](./🐍️cad-hover-latency-probe.mjs) (`SEMIO_PROBE_RUNS` JSON battery: label, `preSelect`, ordered `steps` of `pick`/`entry`/`undo`/`option`); serve/restage: [📜️serve-cad-react.sh](./📜️serve-cad-react.sh), [📜️activate-cad-react.sh](./📜️activate-cad-react.sh).
