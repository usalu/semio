# Independent Visual Integration Audit R95

Read-only source/production inspection; no tests, model evaluation, rendering or production edits. Source SHA256: 029dca650a9e32e24b06ff5d339f6af6dc970c3a1c9658a3e3c63f16b1e13039. Reviewed all five requested handoffs.

## Confirmed Defect: Gravel Cell Boundaries Are Inked in Axon

Production visuals/🏛️savoye/🧊️axon/🧩️primitives.py:381–388 creates a separate Prim for every details/furnishings cell, without merge. _part_faces at109 emits each closed-cell facet. 🧊️axon/🐍️.py:254–275 culls backfaces but fills and strokes every remaining facet. Appearance smoothing at267–269 retains a0.5-width stroke. There is no cross-Prim exposed-surface union or internal-edge cancellation.

Pure source JSON inspection finds8658mesh cells for FIN-PILOTIS-GRAVEL and5582exact shared horizontal upper-facet edges. Exact witness: cell0 face1 and cell21 face1 both contain segment (0,0,-0.15) to (0.009161343320973554,1.1102249114179588,-0.15). Both upper facets face the axon viewer. The painter independently inks this interior coplanar decomposition edge twice. Another witness: cells1/22 face1 share (0.13977508858204127,20.240838656679028,-0.15) to (3.0749999999999997,21.4,-0.15). These are construction-cell boundaries, not physical gravel seams.

R93 removes this class of artifact from ramp plans but does not repair axon surfaces. Exposed material surface/ink boundaries need certification before final HD acceptance while preserving true grade, holes, native cells and material appearance. No pixel severity claim is made without rendering.

## Other Boundaries Reviewed

No opaque fake estate plane remains in inspected axon ground: 🧊️axon/🐍️.py:580–587 emits an unfilled bounded annotation; primitives.py:200–208 explicitly qualifies unsurveyed nonuniform context. Plan solids suppress per-cell strokes for FLOORING at 📋️plan/🐍️.py:468–469. Ramp plans use private exact owned material reference at44–45 and compound region boundaries at507–512. Floor-arrival selection remains geometric at139–148.

No additional verified omission was established. Elevations/sections retain per-part paths requiring final image inspection; no additional speculative defect is claimed. Root owns membership/source corrections and final native/lifecycle/movie gates. This audit does not certify those pending gates.
