# 2D and 3D Hand Review

Scopes: framework modules `◻️2d` and `🧊️3d`. The 3D root and TypeScript package `AGENTS.md` were read completely; the root's historical Brep/Mesh/Scene links do not name current nested instruction files. No instructions were edited.

Each authored entry is reviewed by actual purpose and sibling context. The 2D tree contains the shared path-segment/error engine (`⚙️engine`), planar set operations (`🔀️booleans`), bitmap contour detection (`🔍️trace`), and Rust/TypeScript implementations. The 3D tree contains single-precision rotational/isometry algebra (`🌀️rigid`), half-edge mesh topology/editing (`🥽️mesh`), BVH overlap and containment queries (`🧿️collision`), and its scoped oracle tests. These existing names are meaningful and sibling-distinct; no arbitrary churn is required if the complete physical audit confirms this.

Literal AGENTS, Cargo/package manifests, TypeScript configuration, and Vitest configuration retain their reserved identities. The emoji-prefixed Nx manifest is the repository's explicit fixed tool contract. Dependency node_modules trees are not authored source and are recorded separately.

Final physical audit: 2D has 18 entries / 13 governed, and 3D has 24 entries / 16 governed. Neither tree has missing, multiple, generic, or repeated sibling emoji findings. The two existing names `🌀️rigid` and `🧿️collision` were absent from the semantic registry; exact entries were added to `members-of-members-of-modules`, after which both resolve and taxonomy loading succeeds. No physical source entry was renamed or removed.

The first TypeScript quick runs failed before tests because both routers relied on the shared default `🧪️tests/🟦️.ts` config path, which does not exist in either package. Each scoped router now explicitly selects its existing reserved `vitest.config.ts`; no shared default or config filename changed. Both reruns passed: 2D four tests, 3D one test. The native package pair also completed successfully: 23 2D tests and 84 3D tests, including the independent Parry3D/serde comparisons. Scoped whitespace checks passed.

A later bounded documentation follow-up corrected the 3D Rust router's stale claim that it owned a dedicated Criterion/Brep benchmark. The actual Cargo manifest declares no standalone benchmark target. Its existing `cargo bench -p semio-framework-3d` command remains untouched; the documentation now distinguishes Cargo benchmark mode from Stdio's dedicated Brep benchmark. No benchmark execution is claimed.
