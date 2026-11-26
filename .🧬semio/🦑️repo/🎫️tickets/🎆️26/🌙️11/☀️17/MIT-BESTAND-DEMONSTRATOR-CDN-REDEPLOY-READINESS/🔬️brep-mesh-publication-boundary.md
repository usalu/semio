# BREP Mesh Publication Source Boundary

Procedural run `w2WMq3K` failed during source admission before rustc. The BREP engine's selected closure was 4,006 active lines against the existing 4,000-line limit. The ticket's independent `check-brep-mesh-publication-boundary.ts` reproduced that violation with the repository source-closure reader for both Darwin and WASI.

The mesh I/O crate now owns `MeshImportDraft`, which keeps the import cursor, draft key, preparation receipt and retirement phase. It receives the existing primitive handle generator from the engine. The engine keeps the public `MeshPublication` surface and its private fields that bind a draft to the correct registry and revision. Only the engine can place a prepared root in that registry, update unpublished accounting, advance the revision and commit the draft. A stale revision still blocks active preparation, and the same owner can close a draft after its revision changes. The existing API, grant calculations, error order and live registration sequence remain intact.

The same Darwin and WASI check now passes with 3,981 active lines, 237,397 bytes and 23 files for the engine, and 3,035 active lines, 162,939 bytes and 14 files for mesh I/O. The independent 11-root selected closure census passed all 1,824 rows after this change and the separate Solid Transaction repair. No manifests, lockfiles or policy limits changed here.

The changed source files are:

- `🧰️framework/🔨️modules/🧊️3d/📐️brep/⚙️engine/📥️mesh-publication/🦀️.rs`
- `🧰️framework/🔨️modules/🧊️3d/📐️brep/⚙️engine/📦️mesh-io/🦀️.rs`
- `🧰️framework/🔨️modules/🧊️3d/📐️brep/⚙️engine/📦️mesh-io/📥️mesh-publication/🦀️.rs`

No exact warm compiler externs survived the previous pair's lane retirement. The canonical Procedural retry owns the next integrated WASI compile. The existing engine mesh import fixture checks pending-mutation retirement and one-time publication.
