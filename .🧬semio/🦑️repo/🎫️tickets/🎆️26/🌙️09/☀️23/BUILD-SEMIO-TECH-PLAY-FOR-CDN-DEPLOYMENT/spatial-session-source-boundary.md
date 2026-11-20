# Spatial session source boundary

The queued Play preflight reported 13 Rust files and 264,249 bytes for the spatial session crate against the unchanged ceiling of 32 files and 262,144 bytes. The session's GLB bridge already used the B-Rep kernel contract, mesh transfer conversion and shared mesh codecs. The existing B-Rep mesh I/O library owns these operations and already has their dependencies.

The bridge now lives in B-Rep mesh I/O. Export still tessellates shapes in order, appends positions and normals, offsets the merged indices, and uses the same GLB encoder. Import still decodes GLB, generates the same OBJ payload and invokes the same kernel importer. The two public session functions retain their signatures and convert the bridge errors into their original session variants. The existing B-Rep engine facade provides the library functions. The session still owns locks, geometry claims and JSON responses. The change adds no dependencies and needs no manifest or lockfile edits.

## Verification

The release source-boundary scanner exited successfully with zero over-budget rows and zero errors across the live spatial kernel, B-Rep engine and Flow B-Rep trees. These are the observed live measurements after the edits.

| Rust source owner | Files | Bytes |
| --- | ---: | ---: |
| Spatial session | 12 | 254,803 |
| B-Rep mesh I/O | 3 | 75,167 |
| B-Rep engine facade | 20 | 217,550 |
| Flow B-Rep extension | 15 | 161,579 |

The targeted Bun source scanner suite passed 28 tests and 49 assertions, including the Ajv corpus and independent SQLite condition evaluation. The selection excludes compiler-backed tests so the live release owns Rust compilation.

## Changed files

- [B-Rep mesh I/O bridge](../../../../../../../🧰️framework/🔨️modules/🧊️3d/📐️brep/⚙️engine/📦️mesh-io/🦀️.rs)
- [B-Rep engine facade](../../../../../../../🧰️framework/🔨️modules/🧊️3d/📐️brep/⚙️engine/🦀️.rs)
- [Spatial session public wrappers](../../../../../../../✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/🦀️.rs)
- [Spatial session error mapping](../../../../../../../✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/⚠️errors/🦀️.rs)
- Removed `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/📤️media-export/🦀️.rs` after moving its implementation.
