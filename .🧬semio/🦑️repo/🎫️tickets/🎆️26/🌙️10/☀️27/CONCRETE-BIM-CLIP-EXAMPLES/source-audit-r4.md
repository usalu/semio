# Source Audit R4

Read-only audit of current remodel drawing integration. Source was changing during audit; line references are the inspected snapshot. No source edits or shell commands were used.

## Confirmed Defects

### Axon Omits Authored Stair Landings

`🎨️visuals/🏛️savoye/🧊️axon/🐍️.py:452` iterates only `item.get("steps", ())` for stairs. Model `_stair_geometry` (`🧱️model/🐍️.py:308`) separately returns `landings`. Current `ST-GF` and `ST-FR` have authored landing_parts at z 3.38–3.50 and 6.58–6.70. Plan line 302 and section line 404 concatenate steps and landings; axon never renders either landing. Add landings to the axon parts iteration and assert these solids in a language-agnostic fixture.

### Radiator Section Is 0.30 m Lower Than Axon

`🎨️visuals/🏛️savoye/✂️section/🐍️.py:49–52` centres equipment height around mean(points.z). Current RAD-1 points both have z=3.6 with size.z=0.6, producing 3.3–3.9. Axon `_service_prims` lines 303–306 treats the same authored z as the base and produces 3.6–4.2. BL-1 happens to agree because its points span the entire 0–1.2 height. Choose one equipment origin convention, normalize through one geometry interface, and validate both views against the same bounds fixture.

## Checks Without New Defects

- Nullable filling tags are guarded in plan and axon; OP-SC-FRAME is currently unfilled and does not generate a missing-tag lookup there.
- General straight wall cells now subtract all authored door/window openings, and axon cells emit closed six-face solids.
- Shared wall_frame now offsets authored centre lines by half thickness, matching the plan centre-line rectangle convention.
- Current source already includes upper storey columns; the parent handoff count of 31 columns is stale.

## Scope Limits

No Manim or Python runtime was launched because repository launcher was reported unavailable. This report contains deterministic source/data contradictions only, not speculative visual findings.
