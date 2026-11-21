# EN 1993-1-9 Table 8.8 fatigue of closed stringers

The production assessment catalogue covers all eight Table 8.8 details. The selected geometry and longitudinal or local bending stress basis must be confirmed before applying a category.

| Details | Case | Nominal fatigue class and stress basis |
| --- | --- | --- |
| 1–2 | C93-21 | 80 MPa at cross-girder web thickness t ≤ 12 mm and 71 MPa above 12 mm. Check the longitudinal stress in the rib. |
| 3 | C93-21 | 36 MPa for separate stringers on either side of the cross girder. Check the longitudinal rib stress. |
| 4 | C93-21 | 71 MPa for a full-penetration backed splice. Confirm the backing tack welds are inside the final butt weld and check the longitudinal rib stress. |
| 5 | C93-23 | 112, 90 or 80 MPa according to the applicable Table 8.3 weld group. Confirm the double-sided full-penetration weld and its referenced fabrication requirements. The case applies kₛ = (25/t)⁰·² above 25 mm. |
| 6 | C93-22 | 71 MPa for the critical cross-girder web section including Vierendeel effects, or 112 MPa when the stress is determined by EN 1993-2 §9.4.2.2(3). |
| 7 | C93-24 | 71 MPa for the recessed partial-penetration weld with a ≥ t and both indicated fit-up dimensions ≤ 2 mm. Calculate section modulus using the rib thickness. |
| 8 | C93-24 | 50 MPa for the applicable fillet or other partial-penetration weld with the indicated gap ≤ 0.5 mm. Calculate section modulus using the weld throat. |

The corrected German text explicitly places the backing tack-weld requirement in detail 4 and marks the earlier text deleted from detail 5. The detail 7 and 8 geometry and section modulus rules follow the corrected drawings. C93-24 accepts bending moment ranges per unit width at the weld. It computes Ww = t²/6 or a²/6 per unit width, then divides the moment ranges by Ww. For details 1–6, the equivalent and frequent stresses are supplied from the documented structural analysis. [DIN EN 1993-1-9:2010-12 in the Niedersachsen gazette, Table 8.8](https://www.niedersachsen.de/download/80810/nds._mbl._nr._37_2012_vom_30.10.2012_anlagenband_37i.pdf#page=285) [Table 8.3](https://www.niedersachsen.de/download/80810/nds._mbl._nr._37_2012_vom_30.10.2012_anlagenband_37i.pdf#page=278) [EN 1993-2:2006](https://www.phd.eng.br/wp-content/uploads/2015/12/en.1993.2.2006.pdf)

The language-neutral fixture exercises all rows, both cutout stress methods, all three unbacked splice categories, the 12 and 25 mm boundaries, weld fit-up boundaries, changes in the governing section modulus, separate frequent-load failure, and unqualified or invalid inputs. The cases use §8(1)–(2). V-512–V-515 trace the four production entries. V-505 tracks Tables 8.1–8.7.

SQLite 3.54.0 reproduced the 25 fixture cases using an independent SQL expression. The production Rust evaluators passed all 25 fixture cases when compiled in a standalone harness. The checked-in assessment catalogue test consumes the same fixture. The shared workspace build was still running during verification.
