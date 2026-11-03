# 📦️ Handoff: Staffage Multiview (Top, Front, Side)

Status: SKELETON. Every section is a TODO until the owning worker has finished and verified it. No result is claimed here.
Ticket (T): `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️02/STAFFAGE-MULTIVIEW-TOP-FRONT-SIDE-COVERAGE`.
Library (L): `🎓️teaching/🏛️architecture/🧍️staffage`. Brief: `T/📋️brief.md`. View inventory: `T/📖️view-documentation-inventory.md`.

## Coverage matrix summary

Source status (re-checked 2026-11-02 at about 17:36): `T/🗑️generated/validation/matrix.json` now EXISTS (10.1 MB, written 17:34, with a `summary.json` beside it; worker-reported by sonnet-validation, not opened for this handoff, not verified). An earlier check (before 17:34) found no matrix. `T/📝️sonnet-validation.md` still does not exist. The folder `T/🗑️generated/sonnet-validation/` holds `full/`, `full2/`, `probe/` and `sync/`. The table below remains an empty template, as the coordinator asked; it is not filled from the matrix.

Run: TODO (run id and timestamp of the matrix file, to confirm).

Matrix v2 note (board, worker-reported by validation before the D24 pass reached it; not verified here): 37,233 cells, of which 28,052 ok, 1,113 declared unavailable and 8,068 FAIL. Of the print-profile pairs, 19 are at a nominal scale and 89 at a range end. Sync worst cases: 84 height and 91 forward fail D18, almost all authoring defects; ground role ruled out as the cause. Rerun v3 pending (status: open). The table below stays an empty template and is not filled from v2.

| Category | Assets | Bands | Views (3) | Modes (7) | OK | Declared unavailable | FAIL |
|---|---|---|---|---|---|---|---|
| humans | 28 | TODO | TODO | TODO | TODO | TODO | TODO |
| animals | 14 | TODO | TODO | TODO | TODO | TODO | TODO |
| vegetation | 16 | TODO | TODO | TODO | TODO | TODO | TODO |
| objects | 15 | TODO | TODO | TODO | TODO | TODO | TODO |
| furniture | 12 | TODO | TODO | TODO | TODO | TODO | TODO |
| machinery | 16 | TODO | TODO | TODO | TODO | TODO | TODO |
| vehicles | 9 | TODO | TODO | TODO | TODO | TODO | TODO |
| components | 15 | TODO | TODO | TODO | TODO | TODO | TODO |
| appliances | 24 | TODO | TODO | TODO | TODO | TODO | TODO |
| **total** | 149 | TODO | TODO | TODO | TODO | TODO | TODO |

- TODO: named FAIL codes and their counts, from the matrix file (no silent substitution, no blank, no rotated or mirrored view).
- TODO: asset counts per category re-counted from `L/🗂️catalog/` (the 149 figure is from the brief and is not yet re-counted here).

### Sampled vs exhaustive

- TODO: which cells the matrix covers exhaustively (all poses, clips, transitions, bands, alternatives, viewpoints and modes) and which are sampled, with the sampling rule and seed.
- TODO: the cell count of each mode, and the cells not covered, with the reason.

## Conventions and schema changes

Status legend: "decided" = in the decisions log or an approved document; "landed (per worker report)" = the owner says it is in the code or schema; no test result is cited for any item here, so nothing is "verified".

Frame and camera bases
- Frame: x ahead, y up, z left (+z = left of the asset). Left-handed. Decided (D6). D1 now points to D6 on handedness and viewing side. Source: `🔗️interfaces.md` §2. `📐️conventions.md` exists in T: worker-reported (sonnet-contracts reported writing it), exists. Not coordinator-checked. Interfaces §2 remains the source for this bullet.
- Side (`elevation`): seen from −z (the asset's right), paper x = world x, ahead to the right. Decided (D6, interfaces §2).
- Front (`front`): seen from +x looking back, paper x = world z, paper y = world y. Decided (D6, interfaces §2).
- Plan (`plan`): seen from +y, paper x = world x, paper up = +z. Decided (D6, interfaces §2).
- A view switch changes only the camera basis: pose, time, root, scale, seed and environment are unchanged. Decided (interfaces §2).

Three-view layout and time model
- Sheet: plan above the side view and sharing x; front beside the side view, sharing y, with its paper x in the direction of plan up (+z). Decided (D7).
- Time model: one `evaluate` per instant; the three viewpoints are projected from that one `EvaluatedState`, and the analysis is solved once. Decided (interfaces §5, `resolveViews`). D11: `timeS` is the absolute time and `clipTimeS` the wrapped clip phase; thermal history, environment timelines and transport use `timeS`. Decided (D11). D12: one environment evaluator in `🌡️thermal/🌤️environment`, wind blended as a vector component by component. Decided (D12).

3D state contract
- `Vec3 {x, y, z}`, `BoneFrame {o, u, v, w}`, `EvaluatedBone.frame` and `tip`, `EvaluatedState.root` and `anchors[id].z`, `Vec3` accelerations, `WorldForce` with `z`, `fzN`, `mxNm`, `myNm`, `mzNm` (`momentNm` renamed), schema `Anchor.z`, `AppliedLoad` with `z`, `fzN`, `momentXNm`, `momentYNm`, `distributed.wzNPerM`. Decided (D2, D4). Landed (per worker report) as types, interfaces §1; the follow-up work is open: `evaluate()` (rigs), `worldForces()` and the 3D solver (structural), Parcel, Pathline, Streamline, condensation and EmitterState as `Vec3` with `directionRad` dropped and `windDirectionDeg` as a bearing from +x towards +z (thermal, D4).
- Structural 3D solver is approved (D3). Decided.
- Old `momentNm` references on `WorldForce` are not fixed by contracts; they sit in the structural rewrite (interfaces §1).

Cast viewpoint requirement
- Every cast row requires `viewpoint`; `mirror` only with `elevation`. Decided (D5). Landed (per exports report): schema, exporter and README; cast-export suite 66 pass, 1 skip. The consumer audit found all 94 cast rows carry `viewpoint`, so the registry's count of 88 lacking one is stale. The course Python test re-run is still pending. Exports owns the schema and exporter, course owns the manifest, Python and guard.
- D13: the cast index `heightM` is always the physical height; paper extents are in `boundsM`. Decided (D13). Landed (per exports report).
- D14: every export, download and preview filename includes the viewpoint. Decided (D14). Landed for the CLI and preview parts (per exports report).
- D15: missing views are never tolerated: no `?? []` on geometry and no default viewpoint. Decided (D15). Landed status not reported.
- D16: superseded by D24 (the revolve axis is bone-local). Decided (D24).

Unavailable decisions and symbols
- Unavailable scale: required `cause` of `below-printable-feature | no-physical-depiction`; unavailable in all three viewpoints, never half-available. Decided (D8, interfaces §3). Landed (per worker report) in the schema, interfaces §3. Backlog per D8: 53 of 53 symbols lack a front view, 7 lack a plan.
- Symbols: `SymbolRepresentation.geometry` requires `elevation`, `front` and `plan`, each `minItems: 1`. Landed (per worker report), interfaces §3.1.
- Fidelity: `physical` (authored scale, or a `true-extent` symbol) and `conventional-symbol` (`true-position` or `nominal`, paper mm, size carries no dimension); fidelity is returned by the resolver, never stored. Decided (interfaces §3.3).
- Validator codes (`unavailable-cause-missing`, `symbol-*-missing`, `alternative-symbol-view-missing`, `extent-front-*`, `extent-plan-*`, `symbol-extent-*`, `symbol-conventional-note`) and the extent tolerance of max(2 %, 15 mm). Decided (interfaces §4). Landed (per contracts report) in `🔨️modules/🔭️multiview`; the failure backlog has been generated.
- No silent fallback: no other scale, no other view, no blank scene; `view-missing` and the other codes are named failures. Decided (interfaces §5).
- D9: an `asset` alternative is valid only if its target is drawable at that scale and is the same kind of thing; the hedgehog → red-squirrel → domestic-cat chain is forbidden. Decided (D9).
- D10: `validRange` is a declared support range; coverage includes the ends of each band's range, which reach 1:1000. Decided (D10).
- D17: an asset at least 3 mm overall on paper must be drawable at that scale; below 3 mm it may be unavailable with a truthful cause. Sixteen bands must be authored (`📝️haiku-inventory-unavailable.md`). Decided (D17).

### Schema changes (per source)

Interfaces (`🔗️interfaces.md` §1 to §6, draft 1 updated 2026-11-02; worker-reported by contracts, not verified by tests here):
- `Vec3 {x, y, z}`; `BoneFrame {o, u, v, w}`; `EvaluatedBone.frame` and `tip`; `EvaluatedState.root` and `anchors[id].z`; `kinematics.comVelocity` and `segmentAccelerations` as Vec3.
- `WorldForce` `{ …, z, fxN, fyN, fzN, mxNm, myNm, mzNm, … }`: `momentNm` renamed `mzNm`.
- Schema `Anchor.z?` (default 0); `AppliedLoad` `z?`, `fzN?`, `momentXNm?`, `momentYNm?` (default 0; `momentNm` stays as Mz); `AppliedLoad.distributed.wzNPerM?` (default 0).
- `SymbolRepresentation.geometry` requires `elevation`, `front` and `plan` (`minItems: 1`).
- `UnavailableRepresentation.cause`: `below-printable-feature | no-physical-depiction`.
- Fidelity `physical | conventional-symbol` carried by the resolver; `DrawingScene.metadata.fidelity` and `ResolvedDrawing.fidelity` carry the same value (interfaces §6).
- Validator codes in force (interfaces §6): `unavailable-but-drawable` (D17, D27; the symbol print-minimum part of D27 is reported as not implemented yet), `cause-missing-symbol`, `symbol-semantics-missing`, `alternative-asset-category`, `alternative-asset-chained`, `alternative-asset-undrawable` (D9), `valid-range-gap` (D10), `solid-axis-not-ancestor` (D21), `lateral-ambiguous` (D25, D28). `validateRanges(asset, profile)` is a separate, slow export (D10). Removed: `view-twin-missing` (D25), `solid-on-elevation` (D26).
- Stable shared API (interfaces §6): `CAMERAS`, `Camera`, `project`, `nearness`, `ViewMissingError` in `🔨️modules/🎨️depiction/🔭️views`, re-exported.

Conventions (`📐️conventions.md`, contracts, 2026-11-02; worker-reported, not verified by tests here):
- Frame, cameras, moments (`mzNm` counter-clockwise in the side elevation), sections, time units, dimension envelope with tolerance max(2 %, 15 mm), revolved solids (bone-local axis, `solid-axis-not-ancestor`), symbol semantics and fidelity (sections 1 to 9).

Exports (`📝️sonnet-exports.md`, worker-reported; tests as stated there):
- Cast schema, twin and exporter: `viewpoint` required; `mirror` requires `elevation`; `viewpointOf` removed; `heightM` is the physical figure height (D13); paper extents in `boundsM` and `figureBoundsM`.
- CLI: `--viewpoint` required, no default; sheet formats `sheet-svg|sheet-pdf|sheet-dxf|sheet-dwg` take `--projection first|third` and refuse `--viewpoint`; output stem `asset-when[-t<time>]-mode-<viewpoint|sheet-projection>-1to<denominator>`.
- SVG and PDF localized prose (`VIEW_TITLES`, `viewpointCaption`); machine ids stay in attributes.
- Preview: `--viewpoint` required unless `--views`; file names carry the viewpoint; `thermal.referenceEnvironment` honoured (D14).

Structure (`📝️sonnet-structural.md`, worker-reported; tests as stated there):
- `worldForces()` keeps `z`, `fzN`, `momentXNm`, `momentYNm`, `wzNPerM`; 3D solver; `🏗️Structure` region of contracts (Reaction, JointLoad, DiagramStation, StructuralResult residual and centre of mass); sign docstrings (D6, D23).

Not yet landed or open: D31 (explicit revolve axis direction; the coordinator will say when it lands); D24 rigs pass is reported finished by the coordinator (row 2 above).

Terminology
- en/de vocabulary: Side elevation / Seitenansicht, Front elevation / Vorderansicht, Plan (top view) / Draufsicht, Three views / Drei Ansichten; modes are representation / Darstellung; Grundriss only for a cut plan section; front cut "Section A-A" / "Schnitt A-A", plan cut "Plan section B-B" / "Grundriss-Schnitt B-B"; identifiers unchanged. Decided (D6 for the German terms; coordinator approval of `📖️terminology.md`, OQ1 and OQ3). Applied to the README (R-a to R-i, consumer-audit items). Not applied yet: the i18n, captions, validator messages, schema descriptions and tests listed in `📖️terminology.md` §4.

Scale rule
- `mm = m * 1000 / denominator`; real dimensions are checked on paper. Stated in the README Conventions (`mm = m · 1000 / denominator`). Decided (README convention; not a new decision).

## Proof sheets

- TODO: where the proof sheets are written (generated output under `T/🗑️generated/<agent>/`) and which of them are kept.
- TODO: labels used on each sheet (viewpoint captions, mode names, scale, pose or clip instant), with the language set checked.
- TODO: the same-instant, same-scale, same-ground rule checked per sheet.

## Example animations and exports

- TODO: list of example clips rendered per view and per mode (MP4, WebM, frames), with frame counts and fps.
- TODO: export results per format (SVG, PDF, DXF, DWG, frames, MP4, WebM) and per viewpoint, with any named failure.
- TODO: cast rows added or changed in `L/🎓️…/🎨️visuals/🧍️staffage/🔣️.json` (course consumer), if any.

## Consumer migration

Each item is tagged with its source and verification status. No consumer migration is coordinator-checked.

1. **Viewpoint required.**
   - Casts: every row requires `viewpoint` (exports, worker-reported, cast-export suite 66 passed, 1 skipped). The consumer audit found all 94 cast rows carry `viewpoint` (relayed by the coordinator); the registry's 88-of-94 count is stale. The course Python re-run is pending (not verified).
   - CLI and preview: `--viewpoint` is required, with no default (exports, worker-reported).
   - Cache keys: the Python `cast_key` now includes the viewpoint (exports finding 4, worker-reported). The site and React download caches were not changed by exports; the viewer owner must confirm the viewpoint is in their keys (open).
2. **`heightM` physical.** The cast index `heightM` is the physical figure height, with paper extents in `boundsM` and `figureBoundsM` (D13; exports, worker-reported; README updated, Task 11). Open: the course `staffage_height` must read the new meaning (exports finding 7, not verified).
3. **`mzNm` rename.** Contracts landed the `WorldForce` rename (worker-reported, interfaces §1). Open consumers, reported by structural (worker-reported, not re-checked here):
   - the React detail model (`🎯️targets/⚛️react/🔨️modules/🔬️detail/🧬️model/🟦️.tsx`, about lines 144 to 160) still reads `momentNm` (owner sonnet-viewer);
   - the animation module (`🔨️modules/🎞️animation/🟦️.ts`, about lines 295 to 305) drops `fzN`, `momentXNm`, `momentYNm`, `z` and `distributed.wzNPerM` in clips and transitions (owner sonnet-rigs).
4. **Fidelity.** `DrawingScene.metadata.fidelity` and `ResolvedDrawing.fidelity` carry `physical | conventional-symbol` (interfaces §6, contracts, worker-reported). D22 requires the viewer and exports to show the semantics to the reader. Their display status was not reported in T (not verified).
5. **`--projection`.** The sheet formats take `--projection first|third`, required, no default (D32; exports, worker-reported). Third angle is the D7 arrangement, which the viewer and the preview contact sheet use (D32). First angle is ISO 128 and is documented as such (D32).

Still TODO:
- TODO: i18n keys and German wording changed (`L/🎯️targets/⚛️react/🔨️modules/🌐️i18n/`), with the new viewpoints module wording aligned.
- TODO: URL keys (`vp`, `view`) and their defaults, if they changed.

## Fleet, model and concurrency record

Sources: `🗂️work-board.md` (fleet table, decisions, launch note) and the worker reports in T. Each claim is tagged: **worker-reported** (the named report or board row says it), **coordinator-checked** (none is tagged so in the sources read), or **not verified** (no evidence read here). Model and effort are the configured ones from the launch note: the runtime did not report a per-agent model, so they are not observed.

### Fleet record (20 rows)

| # | Type | Agent id | Configured model, effort | Assignment (board) | State (board, at read) | Report and verification |
|---|---|---|---|---|---|---|
| 1 | sonnet-contracts | a5251977094d899a6 | Sonnet 5.5, medium | schema, contracts, resolver, availability, conventions | running | no report in T; interfaces.md draft 1 (not verified) |
| 2 | sonnet-rigs | a77318ba2671924a8 | Sonnet 5.5, medium | 3D evaluator, cross-view landmark audit of all 149 | running | D24 pass reported finished (411 revolved solids in 18 assets, 0.000 mm rest difference), relayed by the coordinator: worker-reported; no rigs report in T |
| 3 | sonnet-humans | ac199a52e1c87faaa | Sonnet 5.5, medium | humans 28 | running | no report in T; not verified |
| 4 | sonnet-animals | add2bc5e610ad16e6 | Sonnet 5.5, medium | animals 14 + appliances B 12 | running (board); PAUSED at the D24 freeze (report) | `📝️sonnet-animals.md`: worker-reported, per-asset table at the pause; open extent and `lateral-ambiguous` items |
| 5 | sonnet-vegetation | a73126a9c9331e5fa | Sonnet 5.5, medium | vegetation 16 | running (board) | `📝️sonnet-vegetation.md`: worker-reported; crown fill verified by dry run only, write held for the D24 pause |
| 6 | sonnet-objects | aacfcb47177dba5c0 | Sonnet 5.5, medium | objects 15 + furniture 12 | running | no report in T; not verified |
| 7 | sonnet-machines | adec58333c92c1d7c | Sonnet 5.5, medium | machinery 16 + vehicles 9 | running | no report in T; not verified |
| 8 | sonnet-buildings | a43ffc2dc02e74f89 | Sonnet 5.5, medium | components 15 + appliances A 12 | running (board) | `📝️sonnet-buildings.md`: worker-reported; per-asset status, proof renders for 27 assets, new 1:500 bands (venetian blind, hot-water cylinder) |
| 9 | sonnet-thermal | a7104637db5e351e2 | Sonnet 5.5, medium | 3D transport, single environment, view-consistent thermal | running, task 2 (board) | `📝️sonnet-thermal.md`: worker-reported; 85 of 85 targeted vitest pass; wider run 299 of 305, the 6 failures are viewpoint call sites routed to exports; T9 and T10 open |
| 10 | sonnet-structural | ae9dd96c529da0c3a | Sonnet 5.5, medium | 3D structure solver, structural views, out-of-plane symbols | running (board) | `📝️sonnet-structural.md`: worker-reported; 204 passed, 1 failed in the structure set (the failure is a catalog validator check, not statics); `tsc` errors remain in other owners' files |
| 11 | sonnet-viewer | afa34ba58c8c8c8a6 | Sonnet 5.5, medium | React target and site: selector, three-view sync | running (board) | no report in T; not verified |
| 12 | sonnet-exports | abfe396e72eaaa0ba | Sonnet 5.5, medium | exports, multiview sheet, dimension proof, cache keys | running (board) | `📝️sonnet-exports.md`: worker-reported; multiview-sheet 16 of 16; cast-export 66 passed; DWG sheet unverified (`DwgUnavailableError`) |
| 13 | sonnet-course | a16b0c064c3b70db4 | Sonnet 5.5, medium | course casts: explicit viewpoint, guard | running (board) | no report in T; course Python re-run pending (relayed by the coordinator) |
| 14 | sonnet-validation | af942be794f1365c0 | Sonnet 5.5, medium | coverage matrix, reference cases, proof sheets | running (board) | no matrix or report in T at the last check (Task 10) |
| 15 | haiku-inventory | ab9ba6d274106f452 | Haiku 5.5, medium | inventory; unavailable-decision audit; pose and clip ids | running, task 3 (board) | `📝️haiku-inventory.md` and `📝️haiku-inventory-unavailable.md`: static reading, not verified by tests |
| 16 | haiku-spatial-audit | ada8f81aef66704ad | Haiku 5.5, medium | conventions audit, spot checks | running, task 2 (board) | `📝️haiku-spatial-audit.md`: static, not verified by tests |
| 17 | haiku-analysis-audit | ac5506a902e1e2f3a | Haiku 5.5, medium | thermal and structure audit; extent failure classes | running, task 2 (board) | `📝️haiku-analysis-audit.md`, `📝️haiku-analysis-extents.md`: static |
| 18 | haiku-consumer-audit | aedbcd016a3d706a2 | Haiku 5.5, medium | consumer audit; course call sites | running, task 2 (board) | `📝️haiku-consumer-audit.md`, `📝️haiku-consumer-callsites.md`: static; the "94 of 94 cast rows carry viewpoint" finding is relayed by the coordinator |
| 19 | haiku-registry | a134881f1c39ec670 | Haiku 5.5, medium | manifest and cast-viewpoint prep; media inventory | running, task 2 (board) | `📝️haiku-registry.md`, `📝️haiku-registry-media.md`: its 88 of 94 count is stale (coordinator) |
| 20 | haiku-docs | ab53580012b3255e9 | Haiku 5.5, medium | view-doc inventory, handoff, terminology, README drafts | running (board); terminology approved by the coordinator | this ticket's `📝️haiku-docs.md`: documentation only, no test run |

Summary of verification: no test result in this record is coordinator-checked. Worker-reported test results are those of rows 9, 10 and 12 above and the cast-export count of row 12.

### Concurrency

- 20 workers were dispatched in one message with 20 parallel Agent calls, all as async background tasks. The board records 20 live at launch against a runtime limit of 20 concurrent subagents (board, coordinator). Worker-reported in the board; not verified against the runtime.
- The runtime did not report a per-agent model. The recorded model is the one in each agent's definition (board launch note). Not verified.
- Dependencies on the board: row 8 depends on rows 1 and 10; rows 9 and 10 depend on rows 1 and 2; rows 2 to 7, 11, 12 and 14 depend on row 1; row 13 depends on row 12. Rows 15 to 20 have no dependency.
- Process rule on the board: a new validator rule adding more than about 50 issues goes to the coordinator before it lands.

### Decisions D1–D33 (from `🗂️work-board.md`, one line each)

- D1: the side view is the existing `elevation` (rig plane); view names stay `elevation | front | plan`; handedness fixed by D6.
- D2: unified 3D state contract (`Vec3`, `BoneFrame`, `tip`, `anchors.z`, `root`, Vec3 accelerations, WorldForce and AppliedLoad fields, `momentNm` to `mzNm`); contracts lands types, rigs `evaluate()`, structural `worldForces()` and solver.
- D3: the structural solver becomes 3D (approved).
- D4: transport, emitters and condensation become Vec3; `directionRad` dropped; `windDirectionDeg` a bearing from +x towards +z.
- D5: every cast row requires `viewpoint`; `mirror` only with elevation; exports owns schema and exporter, course owns manifest, Python and guard.
- D6: the frame is LEFT-handed (x ahead, y up, z left); cameras: elevation from −z, front from +x, plan from +y; Seitenansicht, Vorderansicht, Draufsicht; Grundriss only for a cut plan; modes are representation (Darstellung).
- D7: three-view layout: plan above the elevation sharing x; front beside it sharing y, paper x in the plan-up direction (+z).
- D8: unavailable scales need a required `cause`; backlog 53 of 53 symbols lack front, 7 lack plan.
- D9: an `asset` alternative is valid only if drawable at that scale and of the same kind; no chains.
- D10: `validRange` is a declared support range; coverage includes its ends, reaching 1:1000.
- D11: `timeS` absolute, `clipTimeS` the wrapped clip phase; thermal history, environment timelines and transport use `timeS`.
- D12: one environment evaluator in `🌡️thermal/🌤️environment`; wind blended as a vector.
- D13: cast index `heightM` is always the physical height; paper extents in `boundsM`.
- D14: every export, download and preview filename includes the viewpoint.
- D15: missing views are never tolerated: no `?? []` on geometry, no default viewpoint.
- D16: revolve `axisX` absolute rest-world (superseded by D24).
- D17: an asset at least 3 mm overall on paper must be drawable at that scale; below 3 mm it may be unavailable with a truthful cause; sixteen bands to author.
- D18: `dimensions` is the physical envelope of everything belonging to the asset; tolerance max(2 %, 15 mm).
- D19: course casts: a mirror of a laterally asymmetric asset needs an allowlist reason and must be visible at the call site; casts sized from real dimensions; plan rotation only as a documented yaw.
- D20: manifest fixes: three missing named poses; three 1:500 bands flipped to authored after verification.
- D21: a solid's `axisBone` is the shape's own bone or one of its ancestors.
- D22: symbol `semantics` keeps true-extent, true-position and nominal; consumers show the semantics to the reader.
- D23: a moment vector is an arrow with two heads; sense follows the left-hand rule; the legend states the curl sense per view.
- D24: the revolve axis is local to its bone (`axisX`, `axisZ` offsets from the axis bone's rest origin); fixes lateral disc growth; supersedes D16.
- D25: `view-twin-missing` removed; `lateral-ambiguous` added: shapes on spread or twisted bones must be solids.
- D26: solids are allowed in every view; the elevation rejection is withdrawn.
- D27: a true-extent symbol with all three views that passes the print minimum and the extent check satisfies D17.
- D28: `lateral-ambiguous` flags a flat shape only when its flattening error on paper exceeds the 0.2 mm print minimum at the band's scale; shapes are not mass-converted to solids; `failures.json` header must match its map.
- D29: vegetation crosswind sway gets no validator exemption; a tree keeps its crosswind abduction only if D28 flags none of its shapes; foliage is not mass-converted.
- D30: the coverage matrix reports nominal-scale cells separately from `validRange` end cells; a print-profile violation is a failure only where the band is declared drawable.
- D31: a revolved solid declares its bone-local axis explicitly (`u`, `v` or `w`, default none); helper axis bones are deleted; contracts adds the field, rigs implements it and migrates to `axis: "u"`, objects re-authors lid and pad. **Not yet landed: the coordinator will say when it does.**
- D32: `--projection first|third` is required with no default; third angle is the D7 arrangement and the viewer's; first angle is ISO 128 (plan below, front left); the preview contact sheet uses D7.
- D33: a 1:500 band flagged `representation-copied` stays `authored` but counts as FAIL in the matrix until redrawn.
- D34: every check (validator, matrix, exports proof) uses the single D18 tolerance, max(2 %, 15 mm). A view reused unchanged across two bands counts as FAIL (`derived-view`), because drawings must be scale-specific; it is reported separately from a view derived from another view within the same band. Board, decided; landing not reported.
- D35: thin parts that leave the drawing plane (leaves, fronds, blades, sheets, cloth) become solids with their physical thickness, tapered where the real part tapers (agave leaf about 40 mm to 4 mm; date-palm rachis about 60 to 15 mm, leaflet band about 2 mm; hosta blade about 1.5 mm). Below the print minimum the stroke supplies the visible width; geometry is never widened to fake it. Board, decided; the D29 outcome is reported in the board (all 13 trees reverted to along-wind sway, 110 `elevation-lateral` issues now 0; worker-reported by the board).
- D36: the asset file's `dimensions` is the single source of truth. The manifest's `heightM` and `widthM` must equal it within 1 mm, enforced by the contracts validator as `manifest-dimension-mismatch`. Owners confirm the asset value, then registry updates the manifest. Four rows are open: date-palm, pregnant-person, skateboarder and operating-table. Board, decided; open rows.
- W1 (wrap-up): every owner makes its ticket scripts cross-platform by resolving paths from the script location (`import.meta.dir`, `Path(__file__)`) or the repo root, never `C:/git/semio`. 110 files with 152 lines (`📑️evidence-index.md` §5). Deferred until wrap-up so that scripts in active use do not break. Status: open. Per-owner routing is in "Remaining work" below and in the index.

Landing status: the board gives no landing status for D1 to D33. Landing status is taken from the worker reports in the schema section below, and D31 is stated there as not landed.

- TODO: shared-file edits per worker (who wrote what, and when), and any merge or conflict resolution.

## Verification evidence

- TODO: test runs with the exact command, the exit code and the summary line, per suite (`repo run @teaching/staffage:test --lang=en` or the fallback `repo tools exec bun -- ...`, with the fallback stated).
- TODO: language-agnostic test vectors and the third-party oracle used for each new feature.
- TODO: guard results (`L/🧪️tests/`, the course guard `staffage_only.py`).
- TODO: real-browser or viewer checks, if run, with the tool and the result.
- Not yet run: none of the above. The inventory slot ran no tests.

## Commands to run and inspect

Listed from the files only. Nothing here has been run for this handoff. Paths are relative to L (`🎓️teaching/🏛️architecture/🧍️staffage`) unless marked.

Repo targets (from `📋️project.json` and the README "Run" section)
- `repo run @teaching/staffage:test --lang=en`: library test (`📦️packages/🟦️typescript/📋️project.json`, target `test`, runs `repo ./📜️script.ts test`).
- `repo run @teaching/staffage:typecheck --lang=en`: library typecheck (same file, target `typecheck`).
- `repo run @teaching/staffage-react:test --lang=en`: React target test (`🎯️targets/⚛️react/📦️packages/🟦️typescript/📋️project.json`).
- `repo run @teaching/staffage-react:typecheck --lang=en`: React target typecheck (same file).
- `repo run @teaching/staffage-site:dev --lang=en`: site dev server (`🌐️site/📦️packages/🟦️typescript/📋️project.json`, target `dev`; also `dev-site`, a second target that runs `repo ./📜️script.ts dev-site`; its purpose is not checked here).
- `repo run @teaching/staffage-site:build --lang=en`: site build (same file, target `build`).
- `repo run @teaching/staffage-site:test --lang=en` and `repo run @teaching/staffage-site:typecheck --lang=en`: site test and typecheck (same file; the site `package.json` scripts name these, the targets exist in the project file).

Cast exporter (README, "Course casts")
- `repo tools exec bun -- 🔨️modules/📤️export/🎭️cast/🟦️.ts --manifest <file> [--ids a,b] --out <dir> [--parent <pid>]`. Per the exporter header, `--catalog <dir>` is also accepted. Unknown flags are refused; progress goes to stderr.

Export and preview CLIs (headers of the files)
- Export: `repo tools exec bun -- 🔨️modules/📤️export/🎬️cli/🟦️.ts --asset <slug> --clip <slug> [--time s] | --pose <slug> --view dark-line --viewpoint elevation|front|plan --scale 100 --fps 25 --format svg|pdf|dxf|dwg|png-sequence|mp4|webm --lang en|de --out <dir>`.
- Aligned multiview sheet: the same command with `--format sheet-svg|sheet-pdf|sheet-dxf|sheet-dwg --projection first|third` and no `--viewpoint` (from the export CLI header; behaviour not checked).
- Preview: `repo tools exec bun -- 🔨️modules/🎨️depiction/🖨️preview/🟦️.ts --asset <id> [--catalog <root>] [--scale 100] [--pose id | --clip id --time s | --symbol id] [--view v] [--viewpoint elevation|front|plan] [--views] [--sheet] --language en|de --out <dir>` (usage line of the file).

Course Python tests (previous ticket `☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS`)
- `m9_tests.sh` runs `uv run --project <energy python package> --python 3.12 --group test python -m unittest "$@"` with `PYTHONUTF8=1`, the teaching output and cache roots, and `SEMIO_TEST_MANIM=1`. The script hard-codes `/c/git/semio` as the workspace (not cross-platform, which AGENTS.md requires), so it must be replaced before reuse. To confirm: the energy Python package path and the test module names for the staffage guard (`🎨️visuals/🧪️tests/🐍️staffage_only.py`).

Placeholders for commands the workers are building (to confirm; none exists as a documented command yet)
- TO CONFIRM: coverage matrix command (owner: sonnet-validation).
- TO CONFIRM: proof sheet command (owner: sonnet-validation; output under `T/🗑️generated/sonnet-validation/`).
- TO CONFIRM: multiview sheet export, the `sheet-*` formats above (owner: sonnet-exports).
- TO CONFIRM: failure backlog: `bun T/🛠️scripts/sonnet-contracts/failures.ts <out.json>`. The file exists, and its source reads the output path from the first argument (`process.argv[2]`). It imports absolute `C:/git/semio/...` paths, so it is not cross-platform yet. Not run.
- Also present, not in the handoff before: `T/🛠️scripts/course/tests.sh` (runs the course `unittest` modules through `uv`, with output under `T/🗑️generated/course`). It is a candidate replacement for `m9_tests.sh`; not run and not verified.

## Consumer audits

All audits below are static traces relayed by the coordinator; none ran a test. Every finding is status **open**. Each item names its routed owner. The earlier consumer audit (`📝️haiku-consumer-audit.md`) is the base report and is not repeated here.

### Modes × views (`📝️haiku-consumer-modes.md`, slot 18)
- Hatching is off by default, so thermal and forces read by colour only, against the README's colour-independent claim. Owners: viewer (defaults), contracts (option). Open.
- Member hatching applies only to axial force and stress; shear, moment and contact pressure are colour-only, and transport temperature parcels too (`D/💨️transport-view:73-77`). Owners: structural, thermal. Open.
- Transport numbers use `.toFixed` (`D/💨️transport-view:131`), so German shows a point decimal separator. Owner: thermal. Open.
- The inspector note "N further values in the inspector" (`D/🟦️.ts:191-193`) varies by view, so legends differ between views. Owner: contracts. Open.
- Front and plan occlusion under spread or twist is approximate (`D/🧊️depth:47,59`). Owner: rigs. Open.
- Test gaps: dark-line, dark-solid, light-solid and forces are not rendered at uniform or mid instants or at transitions; only English is rendered; seven-modes, view-synchrony and transport are tested only at elevation; no test checks a colour-independent cue. Owner: validation. Open.

### React and site (`📝️haiku-consumer-react.md`, slot 18)
Routed to sonnet-viewer for every finding. Already correct: compare, viewer, frames, sheet cells, placed figures, playground, browse and overview cards, custom element, export panel, playground export; URL state round-trips `vp`, `vl`, compare tokens and playground `vp`/`m`; download filenames and metadata carry the viewpoint.
- High: `R/🏠️overview/🟦️.tsx:91` hard-codes `viewpoint="elevation"` in `Frieze`, ignoring the preview preference. Open.
- Medium: `R/📤️downloads/🟦️.ts:105` and `R/🧮️pipeline/🟦️.ts:32,100`: `viewpoint` is optional; a missing one names the file `undefined` and silently draws the elevation (violates D15). Open.
- Medium: `R/🔬️detail/🟦️.tsx:75`: the compare link drops the pose or clip selection and the time. Open.
- Low: `R/🧍️figure:25,117` and `R/📍️placed:82-84`: optional viewpoint; an evaluation error leaves an empty slot with no message. Open.
- Low: `R/🗺️routing:200,287,321` and `R/💾️persistence:179`: an invalid `vp` or stored preference falls back to elevation silently unless the problem check runs. Open.
- Low: `R/🔬️detail/📤️export-panel:91`: the lede does not show the viewpoint. Open.
- Low: `R/🧪️playground:325-326`: a mirror on front or plan is kept in state but not drawn; it should be refused. Open.
- Gap: no React test covers the viewpoint paths. Owner sonnet-viewer. Open.

### Course call sites (`📝️haiku-consumer-callsites.md`, slot 18)
Covers 33 files and 169 call sites; asset checks were sampled. Routed to sonnet-course (manifest, Python and guard, D5 and D19).
- Mirrored casts with asymmetric content: the manifest's `mirror` flag is invisible at the call site, because Python `staffage()` defaults to `mirror=False`. Examples: `optimization-room-cooler` (split-ac-indoor-unit, one display, one wall rest, one jet outlet) and `ventilation-natural-radiator-on` / `-off` (panel-radiator, one valve head). Owner: sonnet-course. Open.
- Sampled low-risk cases (for example `shared-office-worker-pathlines`, `mirror=True` three times, and the person pathlines with alternating mirror) need a hand check. Owner: sonnet-course. Open.

### Export paths (`📝️sonnet-exports.md`)
- Absent. `📝️sonnet-exports.md` contains no export-path audit: a search for "export path", "paths" and "absolute" found no section. Owner: sonnet-exports to write it or state that it is not needed. Status: open.

## Remaining work

- W1 (wrap-up, open): each owner fixes its ticket scripts to resolve paths from the script location or the repo root; list in `📑️evidence-index.md` §5 (110 files, 152 lines). Routing: animals 27 files, buildings 14, humans 10, machines 18, objects 14, vegetation 14, thermal 1 (their owners); contracts 3, course 4, exports 3, viewer 2.

- TODO: open items per owner, from the brief's ownership table.
- TODO: unresolved issues from the view inventory (`T/📖️view-documentation-inventory.md`, section 4), each marked resolved or open.
- TODO: follow-ups for the ticket close (`ticket_close` summary, files created, updated or removed).
