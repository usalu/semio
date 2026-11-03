# Round 3 task board (coordinator-owned)

## Coordinator

- **Configured:** claude-opus-5-5 at medium effort.
- **Verified at runtime:** the coordinator runs as claude-opus-5-5 (Opus 5.5).

## Workers

- Every worker is dispatched with `run_in_background` and uses its named subagent type, with no model override.
- Each worker's model and effort come from its `--agents` profile definition. The coordinator can read only the type name, so the model and effort columns below are the configured values, not verified ones.
- Wave 1: all 20 workers were dispatched in a single message on 2026-11-03.

| id | worker id | subagent type | configured model | assignment | depends on | status |
|---|---|---|---|---|---|---|
| W01 | inv | haiku-inventory | haiku 5.5 | inventory every course clip and build the migration matrix | none | running |
| W02 | prov | haiku-provenance | haiku 5.5 | audit for ad-hoc staffage, fallback paths and allowlist entries | none | running |
| W03 | media | haiku-media-audit | haiku 5.5 | audit consumers, stale media and cache keys | none | running |
| W04 | lesson | haiku-lesson-audit | haiku 5.5 | audit teaching objectives, mode and physics per lesson | none | running |
| W05 | contract | sonnet-contracts | sonnet 5.5 | provenance contract, schema, exporter and bridge, no fallbacks | requests from W06–W18 | running |
| W06 | thermal | sonnet-thermal | sonnet 5.5 | thermal model, emitters, world-space transport, legends | W05 | running |
| W07 | struct | sonnet-structural | sonnet 5.5 | structural force and moment mode, plus integration in structural clips | W05, W16 | running |
| W08 | people | sonnet-people | sonnet 5.5 | people and animals: poses, plan postures, emitter landmarks | W06 | running |
| W09 | veg | sonnet-vegetation | sonnet 5.5 | vegetation, transpiration (latent), tree cast memory (OI-10) | W06 | running |
| W10 | objects | sonnet-objects | sonnet 5.5 | objects and machines; OI-04, 23, 48, 87, 113, 114, 116, 123 | none | running |
| W11 | build | sonnet-buildings | sonnet 5.5 | building components; Nakagin furnishing (OI-46) | W10 | running |
| W12 | timing | sonnet-timing | sonnet 5.5 | deterministic shared scene time and narration sync; OI-121 | W05 | running |
| W13 | player | sonnet-player | sonnet 5.5 | player and routes, library browser modes and provenance; OI-128 | W05, W07 | running |
| W14 | clipsa | sonnet-clips-a | sonnet 5.5 | clips: heating and cooling | W06, W08–W11 | running |
| W15 | clipsb | sonnet-clips-b | sonnet 5.5 | clips: ventilation, demand, physics, intro, concepts, optimization comfort | W06, W08–W11 | running |
| W16 | clipsc | sonnet-clips-c | sonnet 5.5 | clips: bim, excel, grasshopper, nakagin, typology, morph, parametric, fit, course, optimization | W07–W11 | running |
| W17 | media | sonnet-media | sonnet 5.5 | media regeneration, manifests and cache keys | W05, W14–W16 | running |
| W18 | valid | sonnet-validation | sonnet 5.5 | baseline tests, course-wide validator, reruns | W05, W12 | running |
| W19 | reg | haiku-registry | haiku 5.5 | registry consistency; OI-102, 125, 127 | none | running |
| W20 | doc | haiku-documentation | haiku 5.5 | r3 baseline, summary corrections and skeleton | none | running |

## Notes

- W03 and W17 both use the short id `media`. W03's report is saved as `r3-media-audit-report.md`.
- `ticket_reopen` permission was not granted, so the ticket was reopened manually in `🎫️ticket.json`.

## Agent handles (wave 1)

| worker | handle |
|---|---|
| W01 | a5be… (task 2: a5c6…) |
| W02 | a960… |
| W03 | a640… |
| W04 | adbf… (task 2: a76c…) |
| W05 | ad2b… |
| W06 | ad3d… |
| W07 | a0db… |
| W08 | af42… |
| W09 | af43… |
| W10 | a1ff… |
| W11 | a5d4… |
| W12 | a253… |
| W13 | ab33… |
| W14 | ad98… |
| W15 | a559… |
| W16 | ad05… |
| W17 | a281… |
| W18 | a1b5… |
| W19 | a2c3… |
| W20 | ab0c… (task 2: adfb…, task 3: ab9a…) |

**Routing errors (coordinator):** the first rulings messages went to the wrong handles:
- the W14 and W15 messages were swapped;
- the W11 message went to W12;
- the W17 message went to W18;
- the W18 message went to W19.

Corrections were sent to all affected workers.

## Log

- 2026-11-03: wave 1 dispatched, 20 workers.
- W20 finished task 1 (`r3-baseline.md`, `r3-summary.md` skeleton, summary corrections). The coordinator wrote `r3-coordinator-rulings.md` and sent the extra items to W10, W14, W17 and W18.
- W20 was refilled with task 2, the asset ledger (`r3-asset-ledger.md`). 20 workers are running.
- W01 finished the inventory (`r3-inv-report.md`, 127 scenes). The coordinator issued rulings R1–R4. W01 was refilled with inv2 (untraced call sites, lazy loader, dead casts).
- W20 finished the ledger (149 assets, 94 casts resolve). W20 was refilled with doc3 (en/de catalog text).
- W04 finished the lesson audit (`r3-lesson-report.md`, F1–F13). The findings were routed to W06, W07, W14 and W15. W04 was refilled with lesson2 (the lessons not yet audited). 20 workers are running.
- W19 finished reg: the registry is clean (149 assets, 94 casts resolve) and the OI-127 notes are appended. W19 was refilled with reg2 (environment ids, reusable checker), handle aa75….
- W06 finished thermal:
  - Added the comfort module (ISO 7730 PMV and PPD, matching the pythermalcomfort oracle).
  - Added the breathing-catalog test and the legend wording for absolute vs comparative values.
  - Fixed F5. F6 was rejected: pythermalcomfort 4.8 uses c_dil = 120, so our value is kept.
  - Vitest: 317 of 318 pass. The failure is thermal-span, from a concurrent schema edit.
- W06 was refilled with thermal2 (scenario coherence), handle a530…. 20 workers are running.
