# Real Floor-Finish / Owned Beam Overlaps R92

Repair is required:17 actual horizontal first-floor covering bodies overlap exclusive occupied beam material. The unique occupied overlap is0.430371908918m³. This is not a bbox suspicion, not a hull substitute and not intentional shared slab ownership. The audit intersects actual floor polygons/heights with native beam.parts only; beam.shared_parts, embedded slab references and column-owned reservations are excluded.

Stable primary capture77f8a94f0215bfe2414b3e5c2698c3b368db7b63170d275d56e43486ca0b4a7e/API83a4dd3e6c502b803e82c0cfc5c7e7c44f03149a246d6581a99dfe9497db6cdb. It checks4502floorparts overZ3.08..3.17, screens all110beamidentities and resolves14height-active exclusive beam products.126positive pair records sum0.430371908918m³; independent per-floorpart beam-union integration gives the same value. The earlier smaller raw global-union result was a GEOS floating-precision artifact, not geometric double ownership between these intersection prisms.

| Floor | Material | Correct occupied overlap m³ | Strict interior witness XYZ |
|---|---|---:|---|
| FIN-FIRST-THRESHOLDS | MAT-MORTAR-PUTTY | 0.251108318182 | 19.000000, 3.625000, 3.147500 |
| FIN-FIRST-THRESHOLDS | MAT-FLOOR-FINISH | 0.103134463636 | 19.000000, 3.625000, 3.162500 |
| FIN-TERRACE-PAVING | MAT-PAVING | 0.008844300000 | 14.295000, 18.566000, 3.135000 |
| FIN-TERRACE-PAVING | MAT-MORTAR-PUTTY | 0.044221500000 | 14.295000, 18.566000, 3.105000 |
| FIN-TERRACE-GRASS | MAT-PLANTING-SUBSTRATE | 0.001755000000 | 14.295000, 18.193987, 3.110000 |
| FIN-BOUDOIR | MAT-MORTAR-PUTTY | 0.021308327100 | 14.205000, 17.481000, 3.110000 |

The west BEAM-PRIMARY-F-5-33 alone intersects FIN-FIRST-THRESHOLDS bedding.0492m³ and finish.0492m³, total.0984m³. Its actual X−.08..+.08/Z3.14..3.41 upstand occupies the nominal floor-layer space. Other affected members are BEAM-PRIMARY-F-1-6,F-6-29,F-24-30,F-13-25 andF-25-28. FIN-KITCHEN’s own bodies have no positive beam overlap in this scope; the west kitchen leg feet remain fully supported outside the beam by threshold material atX.16.. .185.

Every ordinary intersection uses actual polygon area excluding holes times the strict positive shared height interval. All126positive rows are prism intersections.124stored polygon exteriors reproduce their captured volume; two multipart intersections were recovered using the SAME source hash and a separately recorded current API5e5cad1e85a3fddeabd8660a989aa59c9700b9f23f89635315c5016504ae4c8a. Each reconstructed shape’s area×height agrees with the original recorded volume within1e−10m³. This preserves cavities/multipart topology rather than filling it with a hull. Separate recovery provenance is retained; it is not a claim that the entireproducer remained frozen through later work.

Correct the source-owned floor prisms by actual beam-material subtraction at the intersecting heights. Keep structural/native beam geometry and explicit slab/column ownership unchanged. The revised finish-coverage oracle must exclude raised nonwalkable beam-owned domains instead of repopulating them to cover a gross slab projection. Old positive witnesses should be empty in finish material while remaining occupied in the beam. A later physical weight table must use the repaired source. The intentional1.5kN/m² teaching load policy remains a separate assumed allowance and must not receive a second floor-mass sum.

## Performance and scope

The initial overbroad facade-ownership request was cancelled. The successful sparse candidate pipeline uses exact raw-height negative filtering before owned geometry, because ownership subtraction cannot create new material. Nevertheless this capture took665.573seconds: facade raw datums resolve soffit through full slab geometry, exposing a cold dependency cost. This is a measured audit duration, not a profile attributing all time to one function. It is additional static performance evidence beyond repeated SciPy hulls; no complete building SDK or master suite was run. The subsequent exact union recovery took only a few seconds.

Portable evidence and acceptance: floor-beam-owned-cases-r92.json. Reproducible inputs floor-beam-owned-intersections-r92.py and floor-beam-unique-overlap-r92.py. Detailed generated pair geometry, material witnesses, excluded ownership and source/producer hashes are retained. No production edits.

## Numerical correction

The initial raw global GEOS union returned.430018224708m³ and was mistakenly described as removing cross-beam duplication. That classification is withdrawn. Direct pairwise intersection of all126retained prisms finds no positive overlap above1e−12m³; per-floorpart beam-union sum is.430371908918m³. Diagnostic copies on1e−9,1e−10,1e−11 and1e−12m coordinate grids all give.430371908918m³ within1.7e−13m³. The unquantized union lost.011789473684m² in both15mm finish strata despite area-additive pairwise geometry. No source quantization, feature dropping or floor-part duplication is asserted. Parent/floor-owner RED value.430371908918m³ is correct. Portablecase retains the exact original126intersection prisms, including multipart geometry, so this diagnostic no longer depends on an unchanged production source.

## Repair verified

The source owner’s cb9b138f repair independently passes the targeted four-floor/six-exclusive-beam recheck with zero occupied volume; all six original group witnesses are empty in finish and remain positive in actual beam material. See floor-beam-repaired-review-r92.md. The red geometry above records the earlier77f8 snapshot and no longer describes current construction.
