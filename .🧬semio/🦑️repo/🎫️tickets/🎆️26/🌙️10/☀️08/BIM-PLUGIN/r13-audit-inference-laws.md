# 🔎️ R13 Audit — Inference and No-State Laws (Haiku, read-only)

ART = `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any`.

## Summary
1. **Snapshot = authored only: PASS.** No stored derived values (storey elevation, wall height, solids, schedule rows,
   dimension text, sheet frames, evaluated formulas all inferred).
2. **Graph registration: 20/22.** `NodeKind` (26 kinds) covers all fields except `🎨️finishes` (computed inside
   `Quantity(space)`) and `📦️bodies` (helper recomputed inside `Plan` and `Diagnostics(Building)`). `ModelInference` has
   21 `#[derived]` fields; `model_graph::READS` covers every collection read.
3. **Dependency honesty: no stale-result mismatch** on 8 spot-checked fields; several undeclared reads covered only
   through parent chains (walls solids `wall.axis`, plan `window_types`/`stairs`, attach `edges`).
4. **Tests:** cache-transparency loop covers 10/21 projections; missing for view-linework, zones/schemes, finishes,
   bodies, diagnostic_index. Gating tests missing for storey-levels, wall-layout, opening-frames, curtain-layout,
   stair-runs, spaces, plan-linework, view-linework, zones, phase-visibility, effective-properties, finishes, bodies.
5. **No BIM state module:** no framework bim/aec module. Pre-existing `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🏗️bim`
   (stateless operators with simplified BIM schemas computing floor area / gross volume — divergent derivation) and
   `📐️cad/🧩️extensions/🏢️aec-building*` (profiles, storey count on CAD artifact; no duplication).
6. **Global state:** `🕸️model-graph/🗂️registry/🦀️.rs:19-24` thread_local `SESSIONS`/`PROBES` (per-instance inference
   sessions); `🧮️compute:34-49` thread-local counters; others are static tables only.

## Violations
| # | Sev | Where | Finding |
|---|---|---|---|
| H1 | High | `🕸️model-graph/🧮️compute/🦀️.rs:365` | `view_value` passes `annotations: None` → plan/ceiling-plan views, SVG and sheets omit dimensions/tags/notes/leaders |
| M1 | Med | `🖼️view-linework/🗺️plans:23-31`, `🧭️plan:390-425` | plan geometry recomputed per view (View node; accepted, see rulings) |
| M2 | Med | `🗂️registry:19-24` | BIM-side thread_local session map holds derived state per instance |
| M3 | Med | `🗂️registry:36-41` | engine fault → `ModelInference::default()` silently (empty model shown) |
| M4 | Med | `🧮️compute:380-383,425,335,439-448`, `➖️beams:119` | per-element full scans (openings per wall/space/curtain, spaces per zone, columns per beam) |
| M5 | Med | IFC `🏗️frame:96,102`, `⬜️horizontal:41`, `🔲️ceilings:27`, `🧭️frames:49`, `🏛️spatial:50`; glTF `🌳️scene:118` | exports re-derive tops/elevations instead of reading inference |
| M6 | Med | `🌊️flow/🧩️extensions/🏗️bim/🦀️.rs:256-272,399-425` | second, divergent BIM derivation (other plugin) |
| L1–L11 | Low | registry nested read, thread-local counters, snapshot clones per mutation, uncached `infer` wrappers (csv/json/diagnostics panel), dead `area`/`perimeter`, undeclared reads, dead `find_cycle`, defensive defaults, quantities `table_json` re-sum, doc drift | see transcript |

## Coordinator rulings (binding for `r13-inference-laws`)
- **H1**: fix — views receive the annotation index; plan-view SVG/sheet tests assert annotations.
- **M1**: accepted (a view is its own node with its own cut/range); share sub-results through registered nodes where
  identical (e.g. per-storey bodies).
- **M2**: the per-instance inference session is a domain-neutral concern → the framework owns "one inference session
  per mounted artifact instance" (lifetime bound to the instance, closed through the store/close ladder per
  `r11-exec-store.md`); BIM only provides the graph. Remove the BIM thread_local registry. No compat path.
- **M3**: engine faults surface as diagnostics + an editor notice (en+de), never an empty model.
- **M4**: add index nodes (host → openings, space → bounding openings, zone → spaces, storey spatial index for columns)
  so no node scans a whole collection per element.
- **M5**: exports read placements/elevations/tops from the session's inference only.
- **M6**: consolidate: the flow `🏗️bim` extension operates on the BIM model artifact (reads its inference: quantities,
  spaces) instead of simplified schemas; its own area/volume rules are removed. (Separate package `r14-extensions`,
  together with reviewing `📐️cad/🧩️extensions/🏢️aec-building*`.)
- Register `finishes` and `bodies` as nodes; declare all direct reads; remove dead code and uncached wrappers; fix doc
  drift; gating + cache-transparency tests for EVERY projection (one generic test over all projections + per-field
  gating cases).
