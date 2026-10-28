# Authored BIM Cartesian Frame Provision

The full registered sweep exposed one production failure after stricter Cartesian admission: bimSpatialToIfc4x3 emits Axis=$ plus RefDirection for every nonzero building rotation. The pinned AxisAndRefDirProvision rule rejects that authored frame. Add neutral negative/zero/positive/quarter/half-turn cases before fixing the actual builder. Nonzero horizontal rotation must author explicit Z and X directions together. Zero rotation may retain both unset. Independent full IfcOpenShell validation must check every emitted model, while actual reimport preserves rotation/origin/elevation. The High details worker owns equivalent native builder/query parity.

