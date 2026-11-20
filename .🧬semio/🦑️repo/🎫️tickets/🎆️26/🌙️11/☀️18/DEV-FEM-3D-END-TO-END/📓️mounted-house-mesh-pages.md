# Mounted House mesh pages

The shipped House raft spans 6 m × 8 m and specifies a 0.7071067811865476 m maximum edge. The mounted mesh child previously exceeded its 4 KiB contiguous preparation owner before reading the domain. The configured logical ceilings were used as the allocation sizes. Triangulation faces, insertion scratch, the edge authority, and the published mesh also had contiguous backing assumptions.

The mounted path uses the same preparation, triangulation, constraint recovery, and publication stages with paged backing for working rows. Preparation primes one page per lane and grows as rows are used. Insertion compacts retained faces in place and reuses its paged cavity scratch. The edge authority and output reserve from the completed preparation geometry. Each page admission receives its own exact copy, capacity, depth, and Work receipt. Retirement releases each page under its own quote. Cold callers still return the existing `TriMesh2` value.

The language-neutral raft fixture fixes the shipped polygon and mesh size. The native test first failed at `PreparationFailed`. After paging, it reached the published result under a genuine Source supplier with 221 points, 384 triangles, and 48 m² total area, then completed the close ladder. An independent NumPy calculation generated the rectangle grid and used determinants to measure the two triangles in each cell. It matched all three values. [NumPy oracle](./mounted-raft-oracle.py)

The preparation fixtures now check first-page admission when logical ceilings exceed 4 KiB, and preserve the point, edge, and lookup owner identities at handoff. The cancellation cases cut partial owner allocations and compare admitted bytes with released bytes. The TypeScript oracle passed these fixture cases through `fast-json-patch`.

The complete native mesh library suite passed all 17 tests. The mesh library also checked successfully for `wasm32-wasip2`.
