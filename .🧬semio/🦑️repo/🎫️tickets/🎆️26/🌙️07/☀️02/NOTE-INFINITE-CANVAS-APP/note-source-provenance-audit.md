# Note component source provenance audit

Attempt 36 (driver `wL46KEm`) linked, staged, transpiled and described the real Note WASI component. Provenance rejected the maintenance-ladder source before model or browser publication. The driver report records 878 compiled units and a successful compiler exit. The final provenance recheck did not run.

The source appeared in the actual pre-Cargo admission roster. Three distinct extents bracket the failure on 20 November 2026, CET.

| Observation | Time | Bytes | Digest |
| --- | --- | ---: | --- |
| Admission memo for the exact path | 23:41:37 | 81,918 | SHA-256 `46f78d5bc47b4e2472d0d2b496e3c9a9106a72901931f8f1393d42458970e9e5` |
| Rustc dep-info for `semio-plugin-instance-ladders` | 23:45:08–10 | 82,031 | BLAKE3 `0408f42d96f6dd7c061209507a058152f886f1770e2dde4e73619c60718b8b59` |
| Live source, confirmed by the next admission memo | 23:53:04 | 82,325 | SHA-256 `57083542525c01c4a853415085ba5c1613fbd24d32214300ed86843686a29939` |

Different lengths establish that the byte bodies differ without comparing hash algorithms. The admission memo survives in the repository unit cache with key `1c823d80ba745ea85760cee74aee4dc249990cbdee31d9d09ed4184c75809eb2`. The run's private source witness directory was removed by driver cleanup. The proof error means neither a retained body associated with the observed path nor current source bytes matched rustc's checksum and extent. The driver reused the same completed compiler result for five bounded provenance retries.

The existing `components iterate` route requires provenance before model and browser publication. Its transpile and describe stages do not publish a model or browser generation. The captured compiler output and stage cache therefore cannot complete this failed publication on their own.

The artifact compiler path already uses `watchCargoSourceDriftV1` to report live changes against the admitted roster and cancel a stale child early. The component pass has the same owned control and roster at Cargo launch but does not install the watcher. Its existing output prefetch records digests and does not retain a newly encountered source body when the compiler consumed an intermediate version. A verified per-unit source capture could use rustc's dep-info checksum and extent to admit that intermediate body while it is still present. Both checks preserve the final receipt requirement. The audit made no production edits and launched no build.
