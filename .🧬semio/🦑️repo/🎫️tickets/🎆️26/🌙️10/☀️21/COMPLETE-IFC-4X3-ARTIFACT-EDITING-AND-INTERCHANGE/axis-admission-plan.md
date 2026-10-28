# Cartesian Placement Admission

The independent native source audit confirmed missing IFC4X3_ADD2 AxisAndRefDirProvision enforcement. The pinned EXPRESS requires both optional Axis and RefDirection or neither; Location must be a CartesianPoint with matching dimension, and supplied directions must have matching dimension and nonparallel nonzero ratios. The primary IfcFirstProjAxis fallback is only relevant within these rules; one-sided negative-X examples are invalid and must not be blessed by inventing a different fallback.

Schema-first axis-cases.json supplies 18 positive/negative cases shared by the real TypeScript graph evaluator and independent tools. The implementation will add exact coordinate-role/dimension/provision checks and max-component-scaled normalization for all finite Binary64 ratios. Native currently already scales normalization; its paired-provision admission and original-source regression must be added. Existing Rust compiler ingress remains unavailable; syntax parsing cannot establish native execution.

Primary source: [IfcAxis2Placement3D](https://standards.buildingsmart.org/IFC/RELEASE/IFC4_3/HTML/lexical/IfcAxis2Placement3D.htm) and [IfcFirstProjAxis](https://standards.buildingsmart.org/IFC/RELEASE/IFC4_3/HTML/lexical/IfcFirstProjAxis.htm), corroborated by pinned IFC4X3_ADD2.exp. Run tests before editing, then use IfcOpenShell for ordinary matrices/schema refusals and mpmath for finite extreme normalization. Preserve exact official fixtures.

