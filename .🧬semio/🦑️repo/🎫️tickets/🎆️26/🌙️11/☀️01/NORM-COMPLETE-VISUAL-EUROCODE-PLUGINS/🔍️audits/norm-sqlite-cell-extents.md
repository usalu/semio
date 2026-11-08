# Exact SQLite Cell Extents

The native SQLite database was inspected independently by cell type. Every integer/real contributes eight bytes; UTF-8 text contributes its encoded length, null contributes zero. No blob cells occur in these fixtures.

| Snapshot | Rows | Numeric cells | UTF-8 bytes | Total value bytes |
|---|---:|---:|---:|---:|
| EN1992 full |18|216|799|2527|
| EN1992 optional families absent |14|187|689|2185|
| EN1993 full |25|690|2347|7867|
| EN1993 empty |1|1|5|13|
| EN1995 full |12|192|825|2361|
| EN1995 empty |1|1|10|18|
| EN1997 full |20|326|1074|3682|
| EN1997 empty |1|6|46|94|
| EN1999 full |26|299|1421|3813|

EN1997's literal independent oracle was hand-updated against canonical fixture values: closed enums persistent/da1Str/sand/bored/active/free, cptNkt zero, elasticModulus 33000000000 (IEEE word 4764451958503243776), concreteClass c4050 and the authored new zero-valued pile/wall/lab/water fields. Four newly authored empty domain tables retain exact column-roster checks. The unchanged 20 populated rows now contain 495 cells instead of the stale 420. Both annex variants retain separate literal expectations and the original coherent nested edit.
