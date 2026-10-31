# Beam Void Authority Audit

This read-only audit binds the raw physical source 7f90ee18f0f9ef989083c4c1247d8fa40aa725a2b2dffd380cde7ca3a7eeb978. All 111 IfcBeam objects were inspected. Exactly four raw beam geometry.voids are authored, and none has a COLUMN reservation tag. The generated slab column reservations were initially conflated with beam passages; the executing agent acknowledged that distinction and has not removed or modified source voids.

| Beam | Authored Passage | Pipe Axis (m) | Opening Width (m) |
| --- | --- | --- | ---: |
| BEAM-PRIMARY-F-2-7 | V-DP-1-FLOOR | 14.25, 1.25 | 0.10000 |
| BEAM-PRIMARY-F-4-10 | V-DP-2-FLOOR | 4.75, 1.25 | 0.10000 |
| BEAM-PRIMARY-R-2-7 | V-DP-1-ROOF | 14.25, 1.25 | 0.10000 |
| BEAM-PRIMARY-R-4-10 | V-DP-2-ROOF | 4.75, 1.25 | 0.10000 |

Each passage has 96 authored vertices and an approximately 100 mm reserved profile around the corresponding original nominal 80 mm rainwater pipe. Beam Ownership evidence explicitly identifies the fixed conduit passage and qualifies its clearance/registration as inferred. DP-1 and DP-2 retain original evidence references and their registered floor/roof penetrations. These four genuine authored air passages must remain in both appropriate physical/Reference semantics; replacing derived column-occupancy polygons must not erase them.

Current _column_partition generates temporary occupancy/material cells and separate bore-air records. Those outputs are not independent raw source openings. The semantic constructor must preserve source rainwater voids and independently apply analytic column material and explicitly reserved bore-air operands. This audit does not certify reconstructed runtime bodies or native quantities; those remain separate tests. Source bytes were unchanged throughout.
