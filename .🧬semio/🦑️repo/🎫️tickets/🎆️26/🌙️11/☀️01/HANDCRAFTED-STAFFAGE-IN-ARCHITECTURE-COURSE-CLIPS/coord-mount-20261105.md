# Mounting Decision — 2026-11-05

Read-only design review of mount-audit-20261105.md, current heating/calculation _plant_house, primary-energy _furnish/_boiler/installation/panel call sites, and canonical gas-boiler and panel-radiator rig, anchor and pose data. No source edits or tests executed.

## Decision

Repair two demonstrated placement inconsistencies: the calculation side radiators' visible bracket-to-wall gaps and the primary-energy wall-hung boiler's floor-level connection stubs. Keep the already raised calculation boiler and the corrected thermostat at their current horizontal positions. A cutaway may show equipment on an implied rear wall without drawing that wall behind every object. Do not create new wall glyphs, partitions, pedestals or mounting assets merely to make every support plane explicit.

The useful distinction is whether the drawing contradicts the selected asset's existing physical relationship. In a side view with a visible receiving wall, a bracket stopping short of that wall contradicts the mounting. A face-on appliance on a wall omitted by the room cutaway is a legitimate projection convention. Placing a wall-hung boiler's bottom connection stubs on the drawn floor falsely reads as standing equipment, even if a rear mounting wall is implied.

## Calculation Side Radiators: Fix

The scene draws actual side walls. The panel-radiator's upper support is explicitly titled fixed to the wall of the scene. Its local bracket-top (0.47, 0.035) is transformed by the root (0.035, 0.11, 90 degrees) to world x=0, y=0.58 m. The existing authored 0.035 m stand-off is therefore already encoded; adding a scene gap duplicates it.

Set the two cast-origin x positions to the actual inner wall faces. With present wall centres (-6.65, -1.35), room_h=1.6 and wall=0.35*1.6/2.5=0.224, those faces are -6.538 and -1.462. Derive them from the wall geometry variables instead of storing rounded replacement constants. Preserve the existing unmirrored/mirrored orientation and origin floor y=-0.15 before _HOUSE_DY, which preserves the authored 0.11 m radiator underside clearance.

Keep pipe attachment, pipe spots and any explicit wall-highlight geometry derived from the resulting radiator positions. Do not stretch brackets or insert extra lines to bridge the gap.

## Calculation Boiler and Thermostat: Keep

Calculation already positions boiler origin 0.7 m above cellar floor at one shared scene metre. Its wall-bracket therefore sits 1.34 m above cellar floor (0.7 + 0.64). This is internally coherent with the selected wall-hung asset. x=-2.85 inside the cellar is compatible with an implied rear wall in the existing cutaway, as is the face-on thermostat after its corrected floor-origin placement.

Moving the boiler to either drawn side wall would conflate the face-on equipment plane with the section cut plane and unnecessarily relocate the chimney, labels, pipe links and thermal staging. No evidence requires that redesign. The chimney is not a support, but it need not be one when the appliance is on the implied rear wall.

The cylinder is already correct: canonical feet are at y=0 and the cast origin is the cellar floor. Preserve it.

## Primary-Energy Boiler: Fix Height Only

The boiler helper expressly selects a wall-hung asset, but both its installation and gas comparison panel anchor the stub origin at ground+0.01 scene units. Replace the arbitrary near-ground anchor with a common physical mounting clearance of 0.7 m above the respective floor, multiplied by that scene's existing metre. This reuses the calculation scene's authored installation convention without claiming a building-code minimum.

Prefer a shared physical clearance value in the existing primary-energy scene helper or fixture. Give _boiler an unambiguous floor datum contract, or pass the lifted origin explicitly at both call sites; do not offset in both places. Keep the heat pump anchored at the floor, as it is a different asset with a ground support. Preserve both devices' common scene metre.

Update the immediately connected pipe and flow endpoints only where their current endpoint intentionally denotes a boiler connection. Leave abstract upstream carrier paths, system-boundary rings and quantity bars alone. Derive those changed endpoints from the placed cast rather than duplicating an old G+0.1 assumption. The existing annex outline may stay; it represents a room/cutaway, not a mount. The comparison panel may omit a physical rear wall for the same reason.

## Primary-Energy Front Radiator

A front-view radiator may be on an implied rear wall; do not snap it to a sidewall in screen x. Its extra floor_y+0.02 offset is not evidence of a missing wall. If touching this helper for the mounting correction, remove the arbitrary vertical nudge and use floor_y as the canonical floor datum so the catalog's 0.11 m underside clearance remains exact. This is a small consistency adjustment, not grounds for a separate mounting framework.

## Schema-First Verification Contract

Extend the existing language-neutral placement vectors before source edits. No production schema expansion is warranted: the asset schema already declares dimensions, rig transform and support anchors.

Vectors should state:
- section wall centres/thickness and expected inner faces;
- radiator canonical bracket location and floor-origin mapping, including mirror;
- boiler connection-origin clearance 0.7 m and bracket height 1.34 m above floor;
- cylinder foot-to-floor equality;
- front radiator floor-origin equality when removing the nudge.

Check transformed support positions with the existing independent NumPy/Manim geometry oracle. Keep a positive vector demonstrating that face-on wall-mounted equipment can use an implied rear wall; do not create a global rule requiring every projected support anchor to coincide with a drawn line. Visually inspect the changed actual scene and pipe contacts. Full catalog or video rebuilds are unnecessary for these placement-only changes.

## Scope Boundary

These changes correct use of existing canonical staffage and authored physical dimensions. They require no new glyph, asset, solver, support schema, general scene graph, wall-snapping engine or automatic collision system. Actual building compliance and detailed installation design are outside this educational scene correction.
