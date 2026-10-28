# IFC Opening Subtraction Plan

The current sharedengine diagnoses and omits hosts with IfcRelVoidsElement because opening subtraction is not implemented. Full real-building display requires actual closed-solid subtraction, with original placements, nested mappings and source length units. Do not display opening volumes as solids or silently publish uncut hosts.

## Retained Official Input

Unchanged official IFC4 ADD2TC1 sample: https://raw.githubusercontent.com/buildingSMART/Standards-builds/a8664407b63d42651e48a78a252c76d09dd6274c/IFC/RELEASE/IFC4/ADD2_TC1/HTML/annex/annex-e/slab-openings.ifc

Pinned Standards-builds commit a8664407b63d42651e48a78a252c76d09dd6274c. Bytes 4348; SHA256 736a4aad0f03ac6512d486e7d1e5e94e68cfe8b5916c7e9e6d14b0215e26ac62. Saved as official-slab-openings.ifc in this ticket. Per-file final-IFC release license CC BY-ND4.0, attribution buildingSMARTInternationalLtd and original FILE_NAME producer retained. No sourcefixture registration or conformance claim yet.

## Required Design

First-party closed-solid meshBoolean interface shared by native and TypeScript, schema-first neutral fixtures and independent existing test-only manifold/Three/IfcOpenShell oracle. Preserve control at every plane split/traversal/emission stage and never publish partial products. Cover disjoint, contained, crossing, touching and coplanar solids; manifold edge incidence and signed volume, multiple openings, negative/reflected placements, cancellation/budget exhaustion and invalid open/nonmanifold operands. Do not hide numeric or topology failure with fabricated bounding solids. Distinguish general CSG from clipping halfspaces and preserve explicit unsupported diagnostics until each is implemented.
