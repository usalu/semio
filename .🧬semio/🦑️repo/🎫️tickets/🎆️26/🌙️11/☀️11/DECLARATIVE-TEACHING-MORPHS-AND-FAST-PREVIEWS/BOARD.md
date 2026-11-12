# Coordinator Task Board

Configured capacity: 20 worker profiles (kit), cap env not observable from inside the session. Each worker below was dispatched through an actual background Agent call; the harness reports their completions. Their concurrency is inferred from overlapping lifetimes between dispatch and completion notifications. Their runtime model and effort come from the profile definitions; the coordinator cannot see what the runtime actually used.

| # | Profile | Task | Depends | Status | Result |
| --- | --- | --- | --- | --- | --- |
| W1 | haiku-inventory | INVENTORY.md: morph call sites, consumers | – | running | |
| W2 | haiku-timeline-audit | TIMELINE-AUDIT.md | – | running | |
| W3 | haiku-resource-audit | RESOURCE-AUDIT.md | – | running | |
| W4 | sonnet-staffage | STAFFAGE-LOD-DESIGN.md (phase 1, analysis) | – | running | |
| W5 | sonnet-foundations | env health, runner, HOST.md, FUNDAMENTAL-FIXES.md | – | done | 3 suites green; FD-001 relative venv path; run-tests.sh |
| W6 | sonnet-validation | QUEUE.md, BASELINE.md | – | running | |
| W7 | haiku-teaching-audit | TEACHING-INTENT.md | – | running | |
| W8 | sonnet-schema | schema, spec, stage, facade | CONTRACT | running | |
| W9 | sonnet-timeline | ⏱️timeline evaluator | CONTRACT | running | |
| W10 | sonnet-preview | 📍️placement, 🖼️preview | W9 iface | running | |
| W11 | sonnet-final-morph | 🎞️full executor + recipes | W9 iface | running | |
| W12 | sonnet-profiles | 🎚️profile, Job, CLI, instrument | CONTRACT | running | |
| W13 | sonnet-playback | PLAYBACK.md + player profile selection | W12 | running | |
| W14 | sonnet-exports | EXPORTS.md + export provenance/posters | W12 | running | |
| W15 | sonnet-authoring | morph authoring CLI | W8 | running | |
| – | sonnet-clips-a / clips-b | clip migration batches | W1, W8–W12 | pending | |
| – | sonnet-lifecycle | cancellation/leaks | W3 | pending | |
| – | haiku-registrations / haiku-documentation | registrations, docs | migrations | pending | |

## Updates

- W1, W2, W3, W7 (Haiku readers): done; read-only tools cannot write, coordinator transcribed INVENTORY.md, TIMELINE-AUDIT.md, RESOURCE-AUDIT.md, TEACHING-INTENT.md.
- W4 staffage: design done, decisions sent, resumed for implementation.
- W5 foundations: resumed with TIMELINE-AUDIT risks 1–3 (staffage global clock patch, dt updaters, play/wait drift).
- W8 schema: done (28/29; facade failure caused by 🎞️full ease import, relayed to W11). Resumed as clip batch E1 (physics, heating, cooling, shared energy visuals, optimization, BIM models).
- W9 timeline: done (36 tests). Resumed as batch E2a (ventilation).
- W15 authoring: done (30 tests). Resumed as batch E2b (demand).
- W16 sonnet-lifecycle, W17 sonnet-clips-a (Nakagin perf → Grasshopper), W18 sonnet-clips-b (BIM perf → Excel + BIM info): dispatched.
