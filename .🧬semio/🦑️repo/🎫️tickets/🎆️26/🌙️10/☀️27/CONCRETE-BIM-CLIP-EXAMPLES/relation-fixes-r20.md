# Scoped Relation Repairs R20

Fresh source374: added32-FIRST to REL-TYPE-TY-CIRC22; added OP-GARAGE and GARAGE-SLIDING to REL-CON-ground. Source products and geometry are unchanged. Every IfcRelAssociatesMaterial related target was independently checked against the current element/type registry; no retired or missing target remained at this fresh read, so valid material links were preserved. No source-generation or migration script was used.

The baseline subsequently clarified the material omission: SL-1 directly owns MAT-HOLLOW-SLAB but lacked its relation. Added REL-MAT-MAT-HOLLOW-SLAB→SL-1. The21 RC pads and21 lime preparations were also joined to their existing direct material groups. CW-1 correctly inherits its glass material from its native curtain-wall type; no redundant occurrence link was fabricated.
