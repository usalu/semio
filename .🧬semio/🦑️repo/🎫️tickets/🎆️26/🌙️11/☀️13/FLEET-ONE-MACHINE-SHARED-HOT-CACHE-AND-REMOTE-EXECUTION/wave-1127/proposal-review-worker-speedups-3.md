# 🧭 Review: Consolidated Worker Development-Speed Feedback (18:15, 2026-11-27)

Five worker write-ups (GIS goal, hub/wasm journeys, generation3d, a ticket's 10 proposals, a 2-day lane report) agree: ~95 % of time goes to the plan/build/test cycle, not code. Mapped against wave 1127 state. A full build-system freeze is active until the proof read seals, so new lanes stage and test only.

## Themes → state → action

| Theme (sources) | State after wave 1127 | Action |
|---|---|---|
| Always-ready project graph, plans read a fixed snapshot, never restart on edits, single-project plan < 5 s | H38 overlay reads, H43/H56/H60/H61 latency, H75 budgets; whole read still restarts on any build-system edit (seal blocked all afternoon) | H84 seal; follow-up design lane after seal (resident incremental graph, per-owner publication) |
| Repo tool unbreakable by peers' edits (run a tested, fixed copy) | `repo` runs from the live tree; peers' half edits broke the tool ~8× | **H94**: repo CLI/MCP/sidecars run from a sealed tool snapshot promoted only after its gate passes |
| Shared tree always compiles; multi-file changes atomic | **`repo land`** (H74/H89) mandatory since 14:10; replica still publishes raw edits | **H95**: builds read a consistent source snapshot (no mid-build churn); published "broken crates" board with last editor |
| Smaller portable build units; remove host-probing build scripts; limits per unit | H6/H49/H52 memory, H81 keys; not-portable declines remain (484+) | **H99**: inventory host-probing build scripts → declared, portable inputs; unit time/memory limits enforced at plan |
| One machine scheduler (CPU/memory/disk) admitting by measured peak, pause not cancel, continuous cleanup | H13/H46/H57/H31/H82 admission + backfill + priority; warden pause ladder; H34/H47/H66 images; H93 budget design | Implementation of H93 budget after seal; measured-peak admission audit in H90 |
| Fast feedback tier, progress rate + ETA, compact structured verdicts, reuse test results for same inputs | test offload (H29/H44/H50), H92 affected tests (staged), H75 latency | **H96**: progress events with rate/ETA everywhere, compact verdict JSON for test/build, cross-agent test-result cache keyed by input digest |
| Self-repairing build cache (verify on read, atomic writes, cleanup before full, never force-unmount while writing, `doctor --repair`) | H63/H64 guards, H17/H41/H47 collection, H32 retention law | **H100**: verify-on-read for every cache record (corrupt = miss + quarantine), atomic writes audit, `repo doctor --repair` |
| Generated files scheduled up front; preflight in seconds (missing modules/generated/limits) before 20-80 min builds | H84 producer fix; H90 memory preflight (staged) | **H98**: `repo run` preflight (generated-file producers scheduled first, module resolution, size/memory limits) in seconds |
| Stale builds at activation (served module carries source digest) | not done (plugin activation) | Proposal card (product/plugin area) |
| Native test layer for plugin runtime; record/replay browser probes natively; wasm panic → typed fault; private dev server per agent; journey selectors by schema id | not done (product) | Proposal cards |
| Coordination: declared file claims, per-agent output dir, build queue with ETA + duplicate-rebuild merging; message-or-restart agents; rotate agents before huge context | landing locks/slots/claims files by convention; Nx flight dedupe (H33) | Part of H96 (queue ETA, dedupe); coordinator practice: retire lanes ~250k context (done this wave) |
| Faults that explain themselves | H71 fault detail, H91 failure registry (staged) | covered |
