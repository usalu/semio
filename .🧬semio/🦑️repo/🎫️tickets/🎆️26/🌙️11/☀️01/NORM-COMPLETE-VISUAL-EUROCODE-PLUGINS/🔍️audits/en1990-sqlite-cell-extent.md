# EN 1990 SQLite Cell Extent

The unchanged SQL schema was read by independent native SQLite. Every actual integer/real cell consumes eight contract bytes; UTF-8 text uses SQLite length(CAST(value AS BLOB)); NULL contributes zero. Blob lengths are measured directly. No projection-budget implementation was used.

|Table|Numeric Cells|UTF-8 Bytes|Blob Bytes|NULL Cells|Contract Bytes|
|---|---:|---:|---:|---:|---:|
|en1990_accidental_action|3|6|0|0|30|
|en1990_action|24|76|0|0|268|
|en1990_bridge_sls|63|181|0|1|685|
|en1990_case|0|0|0|0|0|
|en1990_case_number|0|0|0|0|0|
|en1990_case_text|0|0|0|0|0|
|en1990_document|11|58|0|0|146|
|en1990_effect|17|47|0|4|183|
|en1990_factor_override|0|0|0|0|0|
|en1990_fatigue_spectrum|28|78|0|2|302|
|en1990_member|189|582|0|0|2094|
|en1990_permanent_action|6|20|0|0|68|
|en1990_placement|0|0|0|0|0|
|en1990_reliability_analysis|11|47|0|0|135|
|en1990_seismic_action|12|32|0|0|128|
|en1990_test_series|32|110|0|0|366|
|en1990_traffic_group|0|0|0|0|0|
|en1990_variable_action|3|23|0|0|47|

Total: 399 numeric cells × 8 + 1260 UTF-8 bytes = 4452 value bytes. The complete case has 31 rows and 12697 schema bytes. The current canonical enum is normal; the former impossible gumbel 世界 assertion moved its Unicode coverage into the actual arbitrary reliability logical_id, R 世界, adding seven UTF-8 bytes. Closed enum validation and native word assertions remain intact.

## Complete Canonical Reliability and Test Scalars

The new schema-directed numeric occurrence law first failed genuinely: canonical fixture162 versus persisted158. The missing four occurrences were reliability secondActionMean/Sd and theoryMean in both test-series rows. SQL, TypeScript, Rust and native retained admission now persist these fields with exact IEEE companions. The previously defaulted secondActionDistribution is also stored as its canonical closed enum. Nonzero authored fixture values prevent accidental default-zero success.

The new extent is independently derived from the prior4452: four numeric values and four signed word cells add64 bytes, four finite class strings add24 bytes, and the newly persisted Weibull enum adds7 UTF-8 bytes, giving4547. Actual schema text plus table names totals13160 bytes; reliability/table widths become26/29. Empty-domain document extent stays146.
