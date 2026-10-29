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
