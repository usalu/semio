# EN 1998 diagrams and grips (wave 2)

Nothing compiled or run (no cargo/rustc/`repo run`); checked by reading. Gates: `handle-paths-check.py en1998` 0 uncovered; every new handle/marker path has a `SetFieldRule` (assessments.{sectionWidthM,effectiveDepthM,barDiameterMm,tensionSteelAreaM2,chordRotationDemandRad}, accelerograms.scaleFactor, piers.momentRdNm, tanks/silos.fillingRatio, towers.heightM); source-boundary scan: no en1998 crate over budget. No scene-law/diagram-law script exists; the unit/live tests below run `Diagram2d::validate` and the handle laws.

## Verified first (audit was stale)
Already present before this change: buildings (spectra, drift, modes, modal participation, force profile, storey shear/drift, pushover, q-chart, base shear, time-history set vs elastic spectrum), foundations (4 profiles), bridges (deck modes, pier moments), tanks (wall pressure), silo-demand, tower modes/forces, diaphragm, chord-rotation ladder, plateau chart. Placements already have move grips (`move_handles`).

## New diagrams (pure projections; all in `diagrams()` via existing modules)
- `pier-capacity:<bridge>` (scene-structures): per pier elastic stiffness (support pier stiffness, governing direction) to hinge force M_Rd/H then plateau, V_Rd limit, demand point; yield marker draggable onto `piers[].momentRdNm` (k = H). No ductility values invented: plateau only extends the axis.
- `silo-fill:<id>`: N_Ed and V_Ed at design a_g over filling ratio vs N_Rd/V_Rd; two markers draggable onto `silos[].fillingRatio`. Silo wall pressure still not modelled (no normative kernel, see v15).
- `wall-pressure:<tank>`: added `fill-level` marker (draggable onto `tanks[].fillingRatio`, k = 1/H) and shell-height limit.
- `tower-shear:<id>` / `tower-drift:<id>`: CQC level shear (new `tower::level_profile`; node 0 equals `Response.base_shear_n`), design displacement q*F/(m w^2) and inter-level drift; tip marker draggable onto `towers[].heightM`. No drift limit drawn (none specified for towers).
- `accelerogram:<id>` (horizontal-elements): scaled time history, peak marker draggable onto `accelerograms[].scaleFactor` (k = 1/raw peak). The response-spectrum-vs-code-spectrum diagram (`time-history:<building>`) already existed.
- `chord-rotation:<assessment>`: added `demand-<id>` marker draggable onto `chordRotationDemandRad`.
- Moment-curvature not drawn: no yield-moment kernel exists (only yield curvature); not invented.

## New grips
- Assessed member: `:width` (sectionWidthM), `:effective-depth` (effectiveDepthM), `:bar-diameter` (barDiameterMm, 100 mm per m), `:tension-steel` (tensionSteelAreaM2), anchored on the section glyph.
- Accelerogram trace: `<id>:scale` (scaleFactor) at the scaled peak; trace entity now lists its handles (`trace_base` and `TRACE_METRES_PER_MS2` made `pub(crate)` in horizontal-elements).

## Files
`🎬️scene-structures/🦀️.rs` (TowerView.q, four fns), `🎬️scene/🧱️horizontal-elements/🦀️.rs`, `🎬️scene/🌋️capacity/🦀️.rs`, `🎬️scene/🕹️handles/🦀️.rs`, `🧬️schema/🗼️tower/🦀️.rs` (`level_profile`), tests `🎬️scene/🧪️tests/🔬️unit/🦀️.rs` (4 tests + helper) and `🔬️live/🦀️.rs` (incremental == full after drags of record scale, silo fill, section, tower, piers).

## Open / unverified
- Unverified until compile; grip anchors and k values (bar diameter 100, steel area 0.004) are UX choices.
- Pier displacement uses the pier-only stiffness `3EI/H^3` with the support force of the governing direction.
- Bridge capacity curve, tank/silo hydrodynamic pressure beyond existing kernels, moment-curvature, remaining unbound check families (gap 7) untouched.
