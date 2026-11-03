# 🗂️ Work Board (coordinator-owned)

Coordinator: the main conversation (Opus 5.5, medium effort).

**Launch:** 2026-11-02. All 20 workers were dispatched in a single message with 20 parallel Agent calls; all were accepted as async background tasks. Each worker uses its configured agent type, so its model and effort come from its definition (Sonnet 5.5 or Haiku 5.5, medium). There were no per-call model overrides and no forks.

**Concurrency:** 20 live workers at launch. The runtime limit is 20 concurrent subagents.

**Observed model:** the runtime did not report a per-agent model. The recorded model is the one in the definition each type was launched with.

| # | Type | Agent id | Assignment | Depends on | State | Verification |
|---|---|---|---|---|---|---|
| 1 | sonnet-contracts | a5251977094d899a6 | schema, contracts, resolver, availability, conventions | — | running | — |
| 2 | sonnet-rigs | a77318ba2671924a8 | 3D evaluator, cross-view landmark audit of all 149 | #1 (3D state types) | running | — |
| 3 | sonnet-humans | ac199a52e1c87faaa | 👤️humans 28 | #1 | running | — |
| 4 | sonnet-animals | add2bc5e610ad16e6 | 🐕️animals 14 + 🔌️appliances B 12 (laptop … kitchen-extractor-hood) | #1 | running | — |
| 5 | sonnet-vegetation | a73126a9c9331e5fa | 🌳️vegetation 16 | #1 | running | — |
| 6 | sonnet-objects | aacfcb47177dba5c0 | 🪑️objects 15 + 🛋️furniture 12 | #1 | running | — |
| 7 | sonnet-machines | adec58333c92c1d7c | 🏗️machinery 16 + 🚜️vehicles 9 | #1 | running | — |
| 8 | sonnet-buildings | a43ffc2dc02e74f89 | 🧱️components 15 + 🔌️appliances A 12 (ceiling-diffuser … hot-water-cylinder) | #1, #10 | running | — |
| 9 | sonnet-thermal | a7104637db5e351e2 | (1) 3D transport, the single environment, view-consistent thermal: done (`📝️sonnet-thermal.md`); (2) fixes T9 and T10 plus an all-asset thermal sweep | #1, #2 | running (task 2) | Task 1: 85/85 targeted vitest pass and `tsc` is clean for its files, per the worker. In the wider run 299/305 pass; the 6 failures are viewpoint call sites, routed to exports. A numpy oracle is included. |
| 10 | sonnet-structural | ae9dd96c529da0c3a | 3D structure solver, structural views, out-of-plane symbols | #1, #2 | running | — |
| 11 | sonnet-viewer | afa34ba58c8c8c8a6 | React target + site: selector, three-view sync | #1 | running | — |
| 12 | sonnet-exports | abfe396e72eaaa0ba | exports, multiview sheet, dimension proof, cache keys | #1 | running | — |
| 13 | sonnet-course | a16b0c064c3b70db4 | course casts: explicit viewpoint, guard | #12 | running | — |
| 14 | sonnet-validation | af942be794f1365c0 | coverage matrix, reference cases, proof sheets | #1 | running | — |
| 15 | haiku-inventory | ab9ba6d274106f452 | (1) inventory: done (`📝️haiku-inventory.md`); (2) audit of the 62 unavailable decisions: done (`📝️haiku-inventory-unavailable.md`); (3) pose and clip ids against the manifest | — | running (task 3) | static |
| 16 | haiku-spatial-audit | ada8f81aef66704ad | (1) conventions audit and 10 spot checks: done (`📝️haiku-spatial-audit.md`); (2) 14 more spot checks | — | running (task 2) | static |
| 17 | haiku-analysis-audit | ac5506a902e1e2f3a | (1) thermal and structure audit: done (`📝️haiku-analysis-audit.md`); (2) classification of extent failures | — | running (task 2) | static |
| 18 | haiku-consumer-audit | aedbcd016a3d706a2 | (1) consumer audit: done (`📝️haiku-consumer-audit.md`); (2) course call sites: handedness, mirroring and sizing | — | running (task 2) | static |
| 19 | haiku-registry | a134881f1c39ec670 | (1) manifest and cast-viewpoint prep: done, see `📝️haiku-registry.md` (149/149 rows, 534 bands with all three views, 88/94 casts lack a viewpoint, two casts at 1:20); (2) derivative media inventory | — | running (task 2) | task 1: 149 folders and rows cross-checked |
| 20 | haiku-docs | ab53580012b3255e9 | (1) view-doc inventory and handoff skeleton: done (`📖️view-documentation-inventory.md`, `📦️handoff.md`, `📝️haiku-docs.md`); (2) `📖️terminology.md` | — | running (task 2) | task 1: findings confirmed by contracts |

## Session 2 roster (2026-11-02 23:35)

The session-1 fleet had ended; no agents were reachable (`ListAgents` empty). All 20 session-2 workers were dispatched in one message with 20 parallel Agent calls, each with its configured type and no model override or fork. They were accepted as async background tasks, so 20 workers were live at launch. The runtime did not report a per-agent model; the definition models are Sonnet 5.5 and Haiku 5.5, medium effort. Brief: `📋️session-2.md`. The ticket was reopened by hand (`ticket_reopen` not permitted).

| # | Type | Agent id | Assignment | State |
|---|---|---|---|---|
| 1 | sonnet-contracts | a25749927d6f06e7a | full suite and validator; D42/D46/D47 schema; paper-symbol minimum; captions; `--ranges`; backlog | running |
| 2 | sonnet-rigs | a198ebe8cc995ef1d | animation drops 3D loads; member-on-head bug; udder depth order (D40); solid3 D42/D46/D47; D44; scrub = playback; landmark audit | running |
| 3 | sonnet-humans | a31100dc84049d5eb | humans 28: lateral-ambiguous, D45 anchors, handedness, proofs | running |
| 4 | sonnet-animals | a1d413d6a902be964 | animals 14 + appliances B 12: followTip limbs, cow drift, D45, emitter z, proofs | running |
| 5 | sonnet-vegetation | a8230803a8c98e6a2 | vegetation 16: D35/D42/D47 agave, palm, hosta; crowns; proofs | running |
| 6 | sonnet-objects | ae0f89baaa5a535ab | objects 15 + furniture 12: D45 447 off-geometry anchors, grill emitter, proofs | running |
| 7 | sonnet-machines | a0f6c8b74f4c68fef | machinery 16 + vehicles 9: D45, generator exhaust, asymmetry, proofs | running |
| 8 | sonnet-buildings | afbb36d6dc9f007d7 | components 15 + appliances A 12: radiator plume, D45 supports, lateral load case, proofs | running |
| 9 | sonnet-thermal | aa6dd35d09a8f742e | all-asset thermal sweep; moving emitter; persistent flow across view switches | running |
| 10 | sonnet-structural | abe426392164593df | all-asset structural sweep; out-of-plane oracle; lateral deflection and torsion; D45 in solver | running |
| 11 | sonnet-viewer | a060deb3e0e9e7486 | mzNm in the detail model; D38 playground; selector sync; browser tests | running |
| 12 | sonnet-exports | ab1278c4078a22d16 | ISO projection symbol; dimension proof in real files; frames = live; DWG status; cache | running |
| 13 | sonnet-course | acd876bbb03a329d6 | render key; magic sizing; Python tests; regenerate 94 casts; sample clip renders | running |
| 14 | sonnet-validation | aafa5c316feea2131 | report; matrix v3 and reruns; regression test; reference cases; proof and contact sheets | running |
| 15 | haiku-inventory | aad331de76e0618a2 | re-inventory and scope-shrink check | running |
| 16 | haiku-spatial-audit | a06c353400331f731 | asymmetry and D45 anchor audit | running |
| 17 | haiku-analysis-audit | a9f25e7802334df19 | thermal and structural code audit, emitter directions | running |
| 18 | haiku-consumer-audit | ae136234aefb8027c | consumer propagation and stale media audit | running |
| 19 | haiku-registry | a8692f3aaf4a968bd | pending manifest changes, D39 renames, manifest vs asset | running |
| 20 | haiku-docs | a8e34b1383fc09904 | handoff restructure, README conventions, W1 list | running |

## Decisions log

- **D1.** The side view is the existing `elevation`, the rig plane. World x is forward, y is up, z is left. View names stay `elevation | front | plan`. Handedness and viewing side were later fixed by D6: the frame is LEFT-handed.
- **D2.** Unified 3D state contract, from the rigs and structural proposals:
  - `Vec3 {x, y, z}`;
  - `EvaluatedBone.frame {o, u, v, w}` and `tip`;
  - `anchors.z`;
  - `root {x, y, z, angleDeg}`;
  - `segmentAccelerations: Vec3`;
  - WorldForce gets `z`, `fzN`, `mxNm`, `myNm`, and `momentNm` is renamed to `mzNm`;
  - AppliedLoad gets `z`, `fzN`, `momentXNm`, `momentYNm`, and `wzNPerM`.
  - contracts lands the types, rigs implements `evaluate()`, and structural implements `worldForces()` and the solver.
- **D3.** The structural solver becomes 3D. Approved.
- **D4.** Transport, emitters and condensation become Vec3, and `directionRad` is dropped. `windDirectionDeg` becomes a horizontal bearing from +x towards +z. Thermal edits the Transport region of contracts.
- **D5.** Every cast row requires `viewpoint`, and `mirror` is allowed only with elevation. Exports owns the schema and the exporter; course owns the manifest, Python and the guard.
- **D6.** The code is kept. The frame is LEFT-handed: x ahead, y up, z left. The brief's "right-handed" claim was wrong; haiku-docs and contracts found it independently.
  - The elevation is seen from −z, the asset's right, with ahead to the right on paper.
  - The front is seen from +x, with paper x = z.
  - The plan is seen from +y, with paper x = x and paper up = +z.
  - Terminology: Seitenansicht, Vorderansicht and Draufsicht. Grundriss is used only for a cut plan.
  - The 7 modes are called representation (Darstellung).
- **D7.** The sheet and three-view layout: the plan sits above the elevation and shares x; the front sits beside the elevation, shares y, and its paper x runs in the same direction as plan up (+z).
- **D8.** Unavailable scale decisions get a required `cause` field. Backlog: 53 of 53 symbols lack a front view and 7 lack a plan.
- **D9.** An `asset` alternative is valid only if its target is drawable at that scale and is the same kind of thing. The hedgehog → red-squirrel → domestic-cat chain is forbidden.
- **D10.** `validRange` is a declared support range. Coverage includes the ends of each band's range, which reach 1:1000.
- **D11.** `timeS` is the absolute time and `clipTimeS` the wrapped clip phase. Thermal history, environment timelines and transport use `timeS`.
- **D12.** There is one environment evaluator, in `🌡️thermal/🌤️environment`. Wind is blended as a vector, component by component.
- **D13.** The cast index `heightM` is always the physical height. Paper extents are in `boundsM`. Exports reports this as landed.
- **D14.** Every export, download and preview filename includes the viewpoint. Exports reports the CLI and preview parts as landed.
- **D15.** Missing views are never tolerated: no `?? []` on geometry and no default viewpoint.
- **D16.** Revolve `axisX` is an absolute rest-world coordinate. This is fragile but correct; it is deferred and not changed while the category batches are live.
- **D17.** An asset at least 3 mm overall on paper must be drawable at that scale. Below 3 mm it may be unavailable with a truthful cause. Sixteen bands must be authored; see `📝️haiku-inventory-unavailable.md`.
- **D18.** `dimensions` is the physical envelope of everything that belongs to the asset: held items, attachments, plates and lids. It excludes stroke width, plumes and annotations. Extent tolerance is max(2 %, 15 mm).
- **D19.** Course casts:
  - A mirror on a laterally asymmetric asset needs a reason in the guard's allowlist, and every mirror must be visible at the call site.
  - Casts are sized from their real dimensions, with no magic constants and no non-uniform stretch.
  - A plan rotation is allowed only as a documented yaw.
- **D20.** Manifest fixes approved:
  - three missing named poses;
  - three 1:500 bands flipped to authored after verification.
- **D21.** A solid's `axisBone` must be the shape's own bone or one of its ancestors. The charcoal-grill lid broke this rule.
- **D22.** Symbol `semantics` keeps its three values: true-extent, true-position and nominal. Calling a true-extent symbol a "conventional sign" is not a contradiction. Consumers must show the semantics to the reader.
- **D23.** A moment vector is drawn as an arrow with two heads pointing the same way. Its sense follows the left-hand rule, because the frame is left-handed, and the legend states the curl sense in each view.
- **D24.** The revolve axis is local to its bone: `axisX` and `axisZ` are offsets from the axis bone's rest origin. This fixes a bug where lateral bones grew their discs (umbrella 1.145 m instead of 1.05 m). Rigs fixes `solid3` and migrates every revolved solid in one coordinated pass. This supersedes the deferral in D16.
- **D25.** `view-twin-missing` is removed, because front and plan shapes take their depth from their own 3D carry. The rule `lateral-ambiguous` replaces it: shapes on spread or twisted bones must be solids.
- **D26.** Solids are allowed in every view. This withdraws the earlier instruction to reject solids on the elevation.
- **D27.** A true-extent symbol with all three views that passes the print minimum and the extent check satisfies D17. This covers city-bicycle and scissor-lift at 1:500.
- **D28.** `lateral-ambiguous` flags a flat shape only when its flattening error on paper, s · sin θ_max · (1/scale), exceeds the print minFeature (0.2 mm) at the band's scale. The issue reports θ_max and the error in mm. Shapes are not mass-converted to solids under the earlier rule. The `failures.json` header must match its map, and every issue must name its asset.
- **D29.** Vegetation crosswind sway gets no validator exemption. Each tree keeps its crosswind abduction only if D28 flags none of its shapes. Otherwise the crosswind keys of that tree are reverted to pure along-wind sway. Foliage is not mass-converted to solids.
- **D30.** The coverage matrix reports cells at a nominal scale separately from cells at the ends of `validRange` (D10). A print-profile violation is a failure only at a scale where the band is declared drawable.
- **D31.** A revolved solid declares its axis direction explicitly, as a bone-local axis (`u`, `v` or `w`) with a default of none. `axisBone` stays its own bone or an ancestor (D21), and helper axis bones that are children of the shape bone are deleted (charcoal-grill `lid-axis`, bar-stool `pad-axis`). Contracts adds the schema field, rigs implements it in `solid3` and migrates every revolved solid to `axis: "u"` explicitly, and objects then re-authors the lid and the pad.
- **D32.** The multiview sheet's `--projection first|third` is required and has no default. Third-angle is the D7 arrangement and the one the viewer uses. First-angle is the ISO 128 alternative; it mirrors the positions (plan below, front to the left) and is documented as such. The preview contact sheet uses the D7 arrangement too.
- **D33.** A 1:500 band that the validator flags as `representation-copied` stays `authored` in the manifest, but it counts as FAIL in the matrix until its owner redraws it.
- **D34.** Every check (validator, matrix, exports proof) uses the single D18 tolerance, max(2 %, 15 mm). A view reused unchanged across two bands counts as FAIL (`derived-view`), because the drawings must be scale-specific. It is reported separately from a view derived from another view within the same band.
- **Matrix v2** (validation, before the D24 pass reached it): 37,233 cells, of which 28,052 are ok, 1,113 declared unavailable and 8,068 FAIL. Of the print-profile pairs, 19 are at a nominal scale and 89 at a range end. Sync worst cases: 84 height and 91 forward fail D18, almost all authoring defects. Ground role was ruled out as the cause. A rerun (v3) is pending.
- **D35.** Thin parts that leave the drawing plane (leaves, fronds, blades, sheets, cloth) become solids with their physical thickness, tapered where the real part tapers:
  - agave leaf: about 40 mm at the base, down to 4 mm at the tip;
  - date-palm frond: the rachis tapers from about 60 to 15 mm, and the leaflet band is about 2 mm thick, sheet-like in the frond plane;
  - hosta blade: about 1.5 mm.

  Where the thickness is below the print minimum, the stroke supplies the visible width; geometry is never widened to fake it. The edge-on projection must still be true.
- **Vegetation outcome of D29:** all 13 trees were reverted to along-wind sway. θ_max was 6.5–32.9° and the error 2.6–67.9 mm, all above 0.2 mm. The 110 `elevation-lateral` issues came from those keys and are now 0.
- **D36.** The asset file's `dimensions` is the single source of truth. The manifest's `heightM` and `widthM` must equal it within 1 mm; the contracts validator enforces this with `manifest-dimension-mismatch`. Owners confirm the asset value, then registry updates the manifest. Four rows are open: date-palm, pregnant-person, skateboarder and operating-table.
- **D37.** Slender diagonal members (lattice diagonals, slings, braces, spokes, open diagonal lines in front and plan) get a dedicated `member` solid. It is a prismatic bar between two bone-local points, with a physical section, and it projects to its true foreshortened outline in every view. Contracts owns the schema, rigs implements `solid3`, and machines authors the tower-crane and the flat diagonals left behind by its D28 pass. No exemption, and no bounding-rectangle solids.
- **D38.** In the playground, a placement has a 3D position: x along the line, z lateral, plus yaw. Each view projects it through the camera, so in the front view figures sit at their z and are depth-ordered by x. They are never laid out side by side. Catalog friezes are a presentation layout, not a scene, so the side-by-side arrangement stays there and the frieze is labelled as such.
- **D39.** Symbol ids are view-neutral; `*-plan` names are renamed. The manifest and every reference (course, fixtures, tests) change in the same pass, with no aliases.
- **Note.** Structural added `Anchor.tiltDeg` to the schema; contracts was told. The curtain-wall bay oracle passes, matching buildings' hand values.
- **Backlog** (`📝️sonnet-contracts-backlog.md`, broadcast): 24 assets fail. There are 147 `lateral-ambiguous`, 5 `unavailable-but-drawable`, 5 extent issues, 1 `representation-copied`, 3 `manifest-dimension-mismatch` and 1 `manifest-mismatch`. The `--ranges` section is pending.
- **D40.** Every view is a true orthographic view with hidden-line removal. There are no dashed hidden lines, and parts fully hidden in a view are not drawn. A cut plan (Grundriss) is only a named section, never the default plan. Glazing does not occlude.
- **D41.** Cross-view sync has two tiers.
  - Hard FAIL: per-bone extents for bones with a single shape per view, plus an anchor landmark point-in-shape test.
  - Reported, not FAIL: whole-group extents, as "group-drift".
  - The dry run found 1,117 per-bone cases over D18 in 88 assets before the split into tiers.
- **Incident.** A worker (animals) ran `taskkill /F /IM bun.exe`, which killed every bun process on the machine. All workers were told to restart their runs, and global process kills are forbidden.
- **D37 fields** (approved): `member` with `from` and `to` (`bone?`, `x`, `y`, `z` required) and `section` (`rect` w×h or `circle` d). A member shape has no `geometry` field.
- **D39 scheme:** `<asset>-symbol`, with semantic suffixes for assets with two symbols. Machines applied it to its 9 symbols, and registry applies the rest.
- **Coordinator-checked C1.** I inspected the skateboarder multiview sheets at 1:100, 1:200 and 1:500 in dark line, pose `cruising`, third-angle (`🗑️generated/buildings/skate-stack.png`).
  - The plan sits above the side elevation and the front sits to the right, as D7 requires.
  - The arms agree across views: forward and back in the side elevation, spread in the front, diagonal in the plan.
  - Overall dimensions agree: side 1.08 m, front 1.04 m, plan depth 1.04 m, height 1.76–1.77 m.
  - Open: the projection symbol has not been verified against ISO 5456-2, and the front ground line is asymmetric. Both are routed to exports.
- **D37 landed.** Rigs reports that the evaluator passes its oracle at 1e-6 m. Contracts reports that the schema and validator rules are in: `member-end-not-ancestor`, `member-section`, `member-degenerate`, `member-solid` and `member-in-paper-symbol`.
- **D42.** Extruded solids get an optional linear taper toward the bone tip (field name to be agreed between rigs and contracts). It is needed for the D35 agave lamina.
- **D36 closed.** The four rows are confirmed: pregnant-person 0.69, operating-table 2.45, skateboarder 1.084 and date-palm 6.3.
- **Suite status (contracts, 2026-11-02, mid-edit):** 30 of 1,718 tests fail, in 15 of 76 files. A fresh run is pending.
- **D43.** An air-borne region (for example the shower-head spray) may have no drawn shape, provided two conditions hold:
  - its `hidden` note says it is air-borne, in en and de;
  - an emitter anchored to it gives the visible trace in the transport view.
- **Milestone.** Contracts reports the first whole-project clean `tsc` (`📦️packages/🟦️typescript`). The contracts backlog is down to 7 failing assets: 27 `lateral-ambiguous` (25 humans, 2 machinery) and 2 `unavailable-but-drawable`. The 31 `manifest-mismatch` issues are pending the D39 renames.
- **Open bug.** Animals reports that a `member` shape on the labrador head produced no geometry in front or plan, in both poses tried (evaluation `:307`), so animals kept revolved solids for the heads. Routed to rigs for an end-to-end fix through `depict()`.
- **Incident.** Animals reports a "wrongly scoped `rm`". I have asked for the exact command and what was affected.
- **D44.** A physical root yaw (`yawDeg`, rotation about +y) rotates the whole evaluated 3D state before projection. Rigs owns it; viewer (playground placements) and course (plan casts) consume it.
- **D45.** There is one anchor per physical contact or support (feet, wheels, castors, pins, rollers, track and skid sides), each at its real x, y and z. Lumped z-0 anchors are split, and the structural fixtures follow.
- **Status.** Machines (25 assets) and objects and furniture (27 assets) report 0 validator issues and 0 `lateral-ambiguous`. Viewer reports 163 of 165 tests passing; the one failure is `catalog-loading`, which shows 4 of 149 assets invalid.
- **Blocked.** `mcp__repo__doctor` was not granted to the coordinator. The missing link to `@semio-tech/framework-transient` (React and site) is unresolved and needs the owner's permission for the repo dependency tools.
- **D46.** Revolved solids take an optional `lateralScale` (> 0, default 1, which is a circle) for an elliptic cross-section, for example a horse head of 0.18 × 0.32 m. Rigs and contracts own it.
- **Status.** Animals and appliances B (26 assets) report 0 validator issues and 144 proof renders, of which 7 sheets were inspected. Open: the cow drift at 1:50 and 1:100, and the udder showing through the body in the front view (a depth-order bug, routed to rigs).
- **Suite triage (contracts, `vitest-full3`):** 23 of 1,716 tests fail in 17 of 77 files, every one routed to an owner. The `valid-range-gap` false positive has been fixed.
- **D47.** Extruded solids declare their sweep axis explicitly: `sweep: "v" | "w"`, required with no default. With `v`, the profile lies in the bone's (u, w) plane and the thickness runs along the bone normal. Rigs migrates every existing extruded solid to `sweep: "w"` in one pass, with a 0.000 mm proof, and the D42 `zTip` taper applies along the sweep axis. This is needed so that fronds and leaves are broad from above.
- **Wrap-up W1.** At wrap-up, every owner makes its ticket scripts cross-platform by resolving paths from the script location (`import.meta.dir`, `Path(__file__)`) or the repo root, never `C:/git/semio`. There are 110 files with 152 lines; the per-owner list is in `📑️evidence-index.md` §5. This is deferred so that scripts in active use don't break.
- **Process.** Any new validator rule that would add more than about 50 issues goes to the coordinator before it lands.
- **Note.** The course clip render key ignores catalog and exporter changes, so stale mp4s could be served. Routed to course.
- **Note.** Workers cannot always message each other directly (rigs could not reach contracts), so the coordinator relays.
- **Note.** `ticket_open` was not permitted in this session, so the ticket was created manually.
