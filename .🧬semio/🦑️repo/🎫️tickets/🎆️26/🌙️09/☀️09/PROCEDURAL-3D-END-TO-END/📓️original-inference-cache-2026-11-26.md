# Funded Geometry Inference Cache

The original inference driver retains typed evaluation rows across source runs. It publishes a dependency fingerprint with each cacheable row, reads prior results under the active source grant, and reuses the typed value through the field's existing funded copy operation. It counts a completed row once, as either a compute or a cache hit. Each cache row and backing page retires through the original resource owner.

Geometry hashes the canonical widget and its authored incoming wires with explicit record boundaries. The driver then adds the fingerprints of the actual planned parents. An edited widget and its downstream dependents recompute, while independent branches reuse their results. Cyclic or unsupported dependency inputs continue through the ordinary original compute path. The geometry cache keeps the catalogue and compute context fixed for its lifetime.

Geometry admits cache entries whose cost it can quote with bounded metadata reads. It recomputes list-valued and unusually wide records while retaining their valid dependency digests for downstream rows.

Incremental runs read and write the cache. Cold runs pay to clear the cache before evaluating. Bypass skips lookup and publication, and a zero budget disables both. Engine run close returns the current source loan while preserving reusable rows. The enclosing service or editor closes those rows with its own bounded retirement grants.

## Verification

- The native fixture drives original inference with one retained item per crossing and compares every actual backing allocation and free against the receipt. Four consecutive runs yield compute and hit counts of `4/0`, `0/4`, `2/2`, and `0/4`. The edit changes one branch and its dependent. A zero-item turn leaves the cache intact.
- The same fixture completed with a 144-byte accounting budget, exercised funded eviction, stayed within budget, and paid to release the final backing pages.
- Python's graph walker and SHA-256 implementation reproduced the neutral fixture's output values, counts and dependency digests using the program in [verify-original-inference-cache.py](verify-original-inference-cache.py).
- Ajv accepted the neutral fixture against its draft-07 schema.
- The broad inference library suite is currently blocked by stale shared-allocation retirement and recorded-work test APIs in existing test cases. The scoped native test builds the production inference library and evaluates it through the standalone `original-cache` target.
