# Current Slab Candidate Cardinality R74

Fresh isolated openValidator interpreter, current source SHA256`d2f3c0d1bb97c93836fb61b53adfe94756c59c31217585899decf09cda4f59f2`,452products. Ran only source load, SL-1 native geometry build, Shapely Polygon creation, STRtree build and bbox candidate counting. No intersection/difference/union oracle calls were executed by this diagnostic beyond the existing native producer internals. No test/source/producer changes. Interpreter disposed after completion; retained reproduction is slab-performance-r74.py. Output written once after completion.

| Measurement | Result |
|---|---:|
| SL-1 full/exclusive polygon count |1451/1451|
| Shared parts |0|
| STRtree bbox pairs j>i |13672|
| All bbox hits including self/both directions |28795|
| Maximum bbox hits for one polygon |199|
| Import/load model |0.335535s|
| Native SL-1 build |0.860993s|
| Shapely polygon construction |0.113380s|
| STRtree construction |0.003725s|
| Count bbox candidates |0.054829s|

Current source therefore has substantially more SL-1 fragments than the old448-product/R23 capture598. But neither candidate construction nor counting is slow:13672candidate overlays form a finite moderate workload. A completed source contract also performs1451validity/containment-difference checks for SL-1, plus all other slabs and the folded roof ownership fixture. These cardinalities cannot establish whether a specific later GEOS overlay is pathological. Earlier R69 instrumentation completed4999operations/120s with about three filesystem overwrites per binary operation; its wall average is not uninstrumented GEOS timing. The active corrected full method lasting>490s remains unresolved by this count and must not be labeled hung or passing. Bounded low-I/O overlay timing, if later required, should identify the actual slow phase/operand rather than extrapolate from write-heavy profiling.
