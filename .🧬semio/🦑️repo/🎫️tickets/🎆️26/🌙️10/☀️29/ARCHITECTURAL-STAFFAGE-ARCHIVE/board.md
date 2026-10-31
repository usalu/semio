# Task Board

Coordinator: Opus 5.5. Contracts: [contracts.md](contracts.md). Exploration: [exploration.md](exploration.md).

| # | Task | Worker | Status | Gate |
|---|---|---|---|---|
| 0 | Exploration (course site, pets, exports, conventions) | 4× haiku-reader | done | – |
| 1 | Schema TS twin + runtime contracts | coordinator | done | G1 |
| 2 | Scaffold packages, registration, test/dev/build targets | sonnet-worker | in progress | G1 |
| 3 | Core engine: JSON schema, units, rig, animation, evaluation, catalog validation | sonnet-worker | in progress | G2 |
| 4 | Thermal solver + reference cases | sonnet-worker | in progress | G2 |
| 5 | Structural solver + anastruct oracle | sonnet-worker | in progress | G2 |
| 6 | Depiction (7 views) + SVG + preview CLI | sonnet-worker | in progress | G2 |
| 7 | Exports PDF/DXF/DWG/video/frames | sonnet-worker | in progress | G5 |
| 8 | Manifest `🗂️catalog/🔣️.json` | haiku-helper | done (48/48) | G3 |
| 8b | Catalog check CLI `🔨️modules/🗂️catalog/🩺️check` | coordinator | done | G3 |
| 9a | Website app (`📚️app`, `🔨️modules/*`) | sonnet-worker | in progress | G4 |
| 9b | Figure component, clock, custom element, browser exports | sonnet-worker | in progress | G4/G5 |
| 10 | Content: humans A (→ catalog file), humans B (→ T/🧰️scripts/humans-b.json, merge), vegetation, animals, vehicles | 5× sonnet-worker | in progress | G3 |
| 11 | Content: objects, components | 2× haiku-helper | in progress | G3 |
| 12 | Audits: content, usability/a11y, integration | haiku-reader ×3 | pending | G6 |

## Checkpoints

- Content launched with T/authoring.md; walker clip defect (contact gaps, kinematic spikes) sent to core + structure workers.
- 2026-10-29: ticket opened via `repo ticket open` (MCP ticket/test tools denied in this session; `repo` CLI used). Contracts written.
- Objects (8) and components (8) authored, check ✓. Line-weight table lightened (staffage is subordinate view detail, not cut lines):
  body 0.18/0.25/0.35/0.50, detail 0.13/0.13/0.18/0.25, texture –/0.13/0.13/0.13, ground 0.25/0.35/0.50/0.70.
- Decisions: tilt-window stays bottom-hinged (tilt of tilt-turn sash); tied-canopy tie becomes a bilateral tube strut (roller along
  the rod) so wind uplift is carried in compression; manifest dimensions/ids are synced from authored assets (coordinator script).
- Pending refinement pass (sonnet "art director") for haiku-authored categories: café chair sit clip needs an occupant load, desk
  monitor drawn, parasol all ribs + fabric scallops, bench slat detail at lod 2, all 7 views reviewed at 1:50/1:200.
- Audit plan additions: (a) provenance audit — verify every cited reference exists and supports the parameter (web), downgrade to
  authored-estimate where unverifiable; (b) stance-foot drift audit (≤ 1 cm tolerated = 0.2 mm at 1:50); (c) id/manifest sync.
- Validator additions (coordinator): ballistic phases, permanent supports must never be switched off (catches hoof-as-support).
- Refinement items (animals): roe-deer far-fore hoof plant jitter ~2 cm, swing hooves 2 cm below ground, bounding poses must state
  "dynamic snapshot (moment not balanced statically)" in descriptions; cat sitting tail; horse hind thigh boxy at 1:50.
- Refinement: dairy-cow lying udder 0.1 m below ground; hoof geometry 6 mm below ground in walk; vegetation cap lines at bone joints.
- Checkpoint: humans 12/12 merged (humans-b.json merged by sync-manifest.ts --merge), vegetation 8/8, objects 8/8, components 8/8 →
  36/48 authored, manifest synced (dims, named poses, clips, completion). Pending animals assembly (6 authored as fragments), vehicles.
- Content audit must reconcile manifest `distinction` texts with final authoring (e.g. vegetation "winter-bare" replaced by storm/snow).
- Refinement: humans-b toe-off sinks ≤ 4 cm (rigid foot) → add toe bone or lift; mittens → hands with thumb.
- G3 reached: 48/48 assets authored, full pipeline check ✓ for all (T/🗑️generated/full-check.txt), validateManifest 0 issues
  (validator compares named poses), 181 named poses, 117 clips. Website reported complete (T/website.md, 99 tests, browser shots).
- Wave 3 running: refinement humans+animals (sonnet), refinement objects/components/vegetation/vehicles (sonnet), manifest
  distinction texts (haiku-helper), provenance audit (haiku-reader), a11y/usability audit (haiku-reader), official test/typecheck
  gate via repo run (background), exports worker still running.
- Gates: `repo run @teaching/staffage:test` stalled in graph planning (shared-machine load, "no progress for 10 min"), typecheck
  timed out (124) — infrastructure, retry later. Direct vitest with the targets' configs: library 376/378 (2 open PDF tests, export
  worker), react 99/99. Scaffold and export workers resumed for missing deliverables (scaffold.md; dwg wrapper, frames, video, cli).

## Phase 2 (2026-10-29) — see phase2-plan.md, authoring-v2.md, content-worker-brief.md

| # | Task | Worker | Status |
|---|---|---|---|
| P0 | Audit, schema v2 twin, runtime contracts v2, per-asset split + furniture/machinery reclassification | coordinator | done |
| P1 | Core: JSON schema v2, loaders, evaluation w/ representations, validator, check/preview (M1); representation resolution, print profiles, suitability, depiction v2, proof sheets (M2) | sonnet core | running |
| P2 | Thermal v2 (layers, physiology, transient, quantities, shared ranges, environments, thermal view) | sonnet thermal | running |
| P3 | Transport (respiration, world-space parcels, pathlines/streamlines, condensation, transport view) | sonnet transport | running |
| P4 | Convert 48 existing assets (CV-H1/H2/H3, CV-A, CV-V, CV-FO, CV-VM, CV-C) | 8× sonnet | running |
| P5 | 59 new assets (NW-*) | 11× sonnet | after core M1 |
| P6 | Website integration, exports | 2× sonnet | after engine APIs |
| P7 | Audits, coverage matrix, proof sheets, motion review | haiku ×N + coordinator | last |

## Phase 2 restart (session 2, 2026-10-29 15:40)

| # | Task | Worker | Status |
|---|---|---|---|
| R1 | Core: check/preview authoritative, copied-representation check, resolution/derived/unavailable, transport hook, invariance tests, proof sheets, coverage matrix | sonnet core | running |
| R2 | Thermal v2 audit/finish, thermal-v2.md, third-party oracles, per-asset solve table | sonnet thermal | running |
| R3 | Transport verification, determinism, evidence clips, per-asset emitter table | sonnet transport | running |
| R4 | New assets NW-H1, NW-H2, NW-H3, NW-A1, NW-A2, NW-V1, NW-V2, NW-F (31 assets) | 8× sonnet | running |
| R5 | Representation depth audit (scripted) → repr-audit.md | haiku-helper | running |
| R6 | NW-O, NW-VE, NW-MA, NW-C (24 assets); CV fix-ups from official check; website; exports | wave 3 | pending |
| R7 | Audits, coverage matrix, proof sheets, motion review | wave 4 | pending |
- R5 done → [repr-audit.md](repr-audit.md). Coordinator reading: dairy-cow 1:200 == 1:100 (copied, must be redrawn); glass-balustrade,
  glulam-beam 200/100 and playground-swing 500/200 share id sets (review); humans' 1:50 is still the v1 drawing and has fewer points than
  the new 1:100 (1:50 must become the most articulated drawing: fingers, folds, seams, faces); animals/vehicles have almost no material
  hatching at 1:100/1:50; no asset offers plan geometry or symbols yet. → CV fix-up wave.
- 16:15 Core M1 landed (check-v2-results.md): 0/48 pass. 30 crashed on missing `thermal.passengers` → coordinator added `[]` to those
  30 files (pure field insertion, byte-identical round-trip otherwise). Remaining classes: valid-range-overlap, suitability-gap 0.00 mm
  (ruling sent to core: touching/overlapping outlines are joins, gaps are 0 < d < min only), representation-copied (parasol, cargo-bike,
  bus, excavator), default-scale-unavailable (vehicles/machinery). Wave 3 content launched: NW-O, NW-VE, NW-MA, NW-C (24 assets).
- 17:35 All 15 workers had been interrupted (wave 2 at 15:53, wave 3 + core at 16:20; transcripts end with "[Request interrupted
  by user]"), nothing reported. All resumed from their transcripts via SendMessage at 17:35. Monitor transcript mtimes for liveness.
- 18:30 Interruptions recur ~10 min after the coordinator turn goes idle and on rejected permission prompts; coordinator now stays
  in the foreground (T/🧰️scripts/coord/liveness.sh). Contracts: R1–R3 of thermal-v2.md applied (passengers in ThermalInputs and
  ThermalField, radiation split + contact in RegionField, all required). Core reported (core-v2-report.md): 756/767 library tests,
  71/73 assets failing on content (gaps 68, features 28, defaultScale 18, copied 10); proof sheets + coverage matrix tooling done.
- Fix-up wave launched: CVF-H1 (6 humans), CVF-H2 (6 humans), CVF-A (6 animals), CVF-V (8 vegetation), CVF-FO (8), CVF-VMC (14).
