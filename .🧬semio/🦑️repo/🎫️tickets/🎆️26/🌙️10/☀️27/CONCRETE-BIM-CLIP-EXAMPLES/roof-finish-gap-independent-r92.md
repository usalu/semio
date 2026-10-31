# Roof Finish Coverage Residual Classification

The reported0.0255200000271m² missing roof area is not a missing walking finish. The sole substantive polygon isX4.64–4.86,Y5.89–6.006, area0.0255200000000002m². It lies entirely within actual column10-FIRST, whose material continues fromZ3.14 to9.03. The witness[4.75,5.945,6.59] therefore belongs to occupied column material at the queried roof plane. The ground10 footprint also matches in plan, but its Z0–2.91 does not own this roof occupancy.

The other66 components sum2.7143e-11m². A localized per-support subtraction gives0.0255200000293m²; four diagnostic precision grids1e-9 through1e-12 retain the substantive area. This differs from the earlier global-union numerical false loss: here the major area is a real intentional column reservation.

The coverage oracle accounts for slab weathering and finish-void owners but omits native column/joint occupancy when defining walkable support. It should exclude actual occupied structure at the queried height, rather than filling the column or introducing a hardcoded permitted gap. Preserve source geometry. Typed controls: roof-finish-gap-cases-r92.json.

The narrow two-slab/finish-owner query took45.50s. Source31e02b6b changed to7bb88ea3 elsewhere during the frozen-input query; APIb7eccced remained unchanged. Exact67component geometries and owner intersections are retained in generated/roof-finish-gap-independent-r92.json. No full-building capture or source edit was performed.
