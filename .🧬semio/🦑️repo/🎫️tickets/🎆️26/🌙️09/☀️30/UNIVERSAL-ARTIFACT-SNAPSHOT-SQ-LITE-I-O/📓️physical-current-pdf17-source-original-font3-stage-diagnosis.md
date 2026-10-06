# Original PDF Source Route Timing

The registered original `@semio-tech/stdio-pdf-rs:test-snapshot-sqlite-source` route, unchanged fundamental grant 15000ms, genuinely completed **54/54 tests across 13 files in 14.16s**, exit 0. Log `🗑️generated/physical-current-pdf17-source-original-font3-stage-diagnosis.log`. Temporary `[DEBUG]` markers measured only the original font3 operations and were removed afterward, retaining its original fixture, traversals and assertions.

| Variant | Projection create | Domain write | Projection finish | File export | File import | Reader create | Domain read |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Type1 | 20.76ms | 24.24ms | 41.08ms | 18.52ms | 36.03ms | 25.87ms | 0.53ms |
| TrueType | 20.77ms | 0.37ms | 19.19ms | 57.34ms | 21.75ms | 49.91ms | 0.16ms |
| Type3 | 40.33ms | 0.47ms | 22.82ms | 10.88ms | 16.98ms | 26.12ms | 0.30ms |
| Type0 | 25.66ms | 1.96ms | 16.26ms | 21.23ms | 26.86ms | 13.56ms | 0.92ms |

Actual source shows duplicate operation-local authored grammar work: shared `ArtifactSqliteProjection.create` parses the complete SQL; `finish` uses `artifactSqliteDatabaseWithSchema` but calls `validateSqliteDatabaseSchemaControlled`, whose `validateSchemaWork` parses the same SQL again. The private parsed table definitions/CST are not exposed at the PDF composition seam. Root owns that shared interface. No global schema cache or grant mutation was mounted here. The data identifies repeated schema work, without a contention explanation or claim that font traversal is intrinsically slow.

Final no-diagnostic owning route is running in `🗑️generated/physical-current-pdf17-source-original54-final-no-diagnostics.log`; this entry makes no receipt claim for that pending run.

Fresh final original registered Source replay after removing every temporary diagnostic genuinely exits0: 54/54,0failed,13files,10.70s. Log: 🗑️generated/physical-current-pdf17-source-original54-final-no-diagnostics.log. The original15000ms grant is unchanged. No Source production/schema cache/grant was modified for this diagnostic.
