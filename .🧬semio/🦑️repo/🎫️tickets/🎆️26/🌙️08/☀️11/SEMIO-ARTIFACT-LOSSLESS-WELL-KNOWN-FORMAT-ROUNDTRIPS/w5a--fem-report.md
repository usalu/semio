# W5a — 🏗️fem (ad-hoc codec extraction)

## Scope

Write scope: `✏️s/🔌️plugins/🏗️fem/**` only. `✏️s/🔌️plugins/🗄️stdio/**` read-only (never edited).
Target per master-plan extraction map + w0-recon-report §3: the 16 JsonCodec-under-format-name
leaf trees under `🗿️artifacts/{◻2d,🧊️3d}/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/{📥️import/🧩️deserializers,
📤️export/🧵️serializers}/🗿️artifacts/{🧊️obj,🎒️zip,📷️png,🟪️stl}/.../🦀️component.rs` (2 shapes × 2
directions × 4 formats = 16, exact recon match, re-verified before editing).

## What was deleted (all 16, outright, no fallback/flag)

Every leaf wrote raw JSON bytes (via `JsonCodec`) into a file claiming to be `.obj`/`.stl`/`.zip`/
`.png` — fabricated formats. Deleted whole directories (component.rs + component.ts stub each):

| Shape | Direction | Format | LOC (.rs) |
|---|---|---|---|
| ◻2d | import | zip/png/stl/obj | 23×4 = 92 |
| ◻2d | export | zip/png/stl/obj | 12×4 = 48 |
| 🧊️3d | import | zip/png/stl/obj | 23×4 = 92 |
| 🧊️3d | export | zip/png/stl/obj | 12×4 = 48 |
| **Total** | | | **280 LOC deleted** |

Verified via `wc -l` on all 16 files before deletion (see command output in the exit checklist
below); confirmed post-deletion no `.rs`/`.ts` files remain under any of the 16 paths, empty
parent dirs (leftover `🔖️2.0`/`🔖️1.2` version dirs) also removed.

## Per-pair judgment (not mechanical — judged each on its merits)

- **obj, stl (both shapes, EXPORT direction): real geometric mapping exists → rewrote honestly.**
  `FemRegion` (2D) and `FemSolid` (3D) are genuinely meshed continuum bodies: real Delaunay
  triangulation (`crate::mesh::triangulate`, the SAME call `build_nodes_and_elements`/
  `resolve_geometry` already make for their own `Tri3Cst`/`Tet4` FEM elements) extruded by a real
  physical dimension the artifact actually carries (`FemRegion.thickness`, `FemSolid.height` +
  `base_z`), reduced to its outward-oriented boundary surface via `crate::mesh::boundary_faces`
  (the SAME tested helper `fem_3d`'s own solid meshing already relies on for volume/area
  invariants). New shared bridge fn `engine::meshing::build_semio_mesh_snapshot` (added to the
  EXISTING `⚙️engine/🕸️meshing/🦀️component.rs` file, new `#region 🔖️SemioMeshBridge`, both
  shapes) builds a real `SemioMeshSnapshot` (Triangles-topology primitives) from this geometry,
  then the two new leaf files hand it to stdio's real, tested `SemioMeshToObj`/`SemioMeshToStl`
  bridge (`ArtifactSerializer::serialize`) and stdio's real grammar encoders
  (`obj::standards::v3_0::engine::encode_obj`, `stl::standards::v_ascii::engine::encode_stl_ascii`)
  — zero hand-rolled byte encoding in fem itself.

  `FemElement::Bar`/`Beam`/`Frame` line members are explicitly EXCLUDED from this mesh: they carry
  no real cross-section PROFILE in the persisted data (only scalar `area`/`iy`/`iz`/`j`), so no
  honest 3D solid can be derived from them without fabricating a cross-section shape — and the real
  stdio mesh↔obj/stl bridges are hard-Triangles-only anyway (a non-Triangles primitive is a hard
  `Err` in both `SemioMeshToObj`/`SemioMeshToStl`, confirmed by reading their source), so a Lines
  primitive couldn't even round-trip through them. A pure bar/beam/frame model (no `regions`/
  `solids`) exports a structurally valid, empty `.obj`/`.stl` — documented, not fabricated.

- **obj, stl IMPORT direction: no honest mapping → deleted, not replaced.** Reconstructing a
  `Fem2dSnapshot`/`Fem3dSnapshot` from an arbitrary imported mesh would require fabricating
  `FemMaterial`/`FemSection`/`FemSupport`/`FemLoadCase` (engineering data no mesh format carries) —
  exactly the "leaving a capability gap is more honest than a fabricated codec" case. The dead
  (never-registered) import leaves are gone; `import_stdio_kinds()` no longer lists `stdio.obj`/
  `stdio.stl`.

- **zip (both directions, both shapes): no honest mapping → deleted, not replaced.** zip is not a
  bridge target of any semio subset in the master plan's lattice (mesh↔gltf/stl/obj/ply/las; no
  zip). fem has no real archive-bundle capability (no multi-file export flow) to honestly back a
  `.zip`; a "bundle these exports into one zip" feature would be new functionality, not extraction,
  and out of this ticket's scope.

- **png (both directions, both shapes): no honest mapping → deleted, not replaced.** Checked
  per the task's own instruction ("check if it does"): grepped the whole fem plugin for
  rasterization/pixel-buffer code (`rasteriz`, `pixel`, `rgba8`, `framebuffer`) — the only hit is a
  UI camera-scale doc comment, not a raster generator. fem produces no visualization raster; a real
  PNG export doesn't exist to extract.

## Rewired composer/registration surface (both shapes)

- `🏅️standards/🔖️1/🎹️composer/🦀️component.rs` (both shapes): removed `EXPORT_ZIP_DIALECT`/
  `compose_export_zip` and `EXPORT_PNG_DIALECT`/`compose_export_png` (consts + fns + `entries()`
  rows) outright; `EXPORT_STL_DIALECT`/`compose_export_obj` call sites unchanged (same signature,
  now real bodies) with a new doc comment explaining the mesh-bridge honesty rationale.
- `✳️any/🚪️io/🦀️component.rs` (both shapes): `import_stdio_kinds()` dropped `stdio.obj`/
  `stdio.png`/`stdio.stl`/`stdio.zip` (now `csv, json, md, txt` only); `export_stdio_kinds()`
  dropped `stdio.png`/`stdio.zip` (now `csv, json, md, obj, stl, txt`).
- `✳️any/🎹️composer/🦀️component.rs` (both shapes): stale doc comment (`reads()` never actually
  included obj/png/stl/zip, only its own doc claimed it did) corrected to match reality.
- `📦️glue.rs` (fem's own, in-scope): removed the 4 dead import-side `pub mod {zip,png,stl,obj}`
  blocks and the 2 dead export-side `pub mod {zip,png}` blocks per shape (8 blocks total); kept
  export-side `stl`/`obj` mod declarations (same paths, new file contents). Brace-balance verified
  programmatically before compiling.

## Foreign/out-of-scope items found during verification (not touched)

1. **Lagging call sites, fixed (in-scope, fem/** only):** fem's pre-existing (untouched by the 16-
   file scope) CSV/MD/JSON export+import leaves (6+4=10 files, both shapes) failed to compile
   against stdio's CURRENT real `CsvSnapshot`/`MdSnapshot`/`JsonSnapshot` shapes (`has_header`+
   `records`/`blocks`/custom `JsonValue`, not the old `headers`/`rows`/`body`/`serde_json::Value`
   these leaves were written against). Since `cargo check -p semio-s-plugin-fem` cannot succeed at
   all otherwise, and the fix is a same-crate, mechanical field-shape update (no stdio edits, no
   behavior change beyond matching current real types), fixed all 10: CSV wraps the DSL blob in one
   `CsvRecord`; MD wraps it in one `MdBlock::CodeBlock` (verbatim, no markdown-escaping risk); JSON
   walks `serde_json::Value` ↔ stdio's real lexeme-preserving `JsonValue` tree (`serde_to_json_value`
   / `json_value_to_serde`, new small pure functions, one pair per direction) and now reads/writes
   through stdio's own real RFC 8259 text codec (`write_json_text`/`parse_json_text`) instead of a
   re-derived encoder. Not part of the assigned 16, but load-bearing for a green `cargo check`.
2. **Framework compile break, self-resolved (not touched, reported only):** first `cargo test`
   attempt failed to compile transitively — `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs`
   (3× E0063 missing `label`/`semantic_kind` in `MutationMeta` initializers). `git status` showed
   this exact file unstaged-modified (a live concurrent session mid-editing it); re-ran minutes
   later once its diff landed (fields added at all 5 call sites) and it compiled clean. Not fixed
   by this agent — confirmed foreign, self-resolved by the owning session.
3. **Pre-existing, unrelated test failures — reported, NOT fixed (genuinely out of this ticket's
   scope):** `cargo test -p semio-s-plugin-fem --lib` is 324 passed / **8 failed**. All 8 are
   `semio_protocol_conformance` tests in fem2d's and fem3d's `🧬️mutations/💾️binary` and
   `📸️snapshot/💾️binary` `component.rs` files (untouched by this ticket — confirmed clean via
   `git status`, never opened for editing). Root cause: each embeds a `📡️component.protocol.semio`
   grammar fixture (`include_str!`) whose line 2 is `protocol 2d.mutations` / `protocol 2d.snapshot`
   / `protocol 3d.mutations` / `protocol 3d.snapshot` — an identifier that starts with a digit
   (`2d`/`3d`), which `::dsl::parse_grammar` rejects (`expected Ident, found Int "2"`). This is a
   pre-migration protocol-fixture naming defect, orthogonal to codec extraction (obj/zip/png/stl),
   present before this session started, and not something this ticket's mandate covers — flagged
   here for the orchestrator/W8 to route to a proper fix (rename the protocol identifier, e.g.
   `fem2d.mutations`, in the 4 `📡️component.protocol.semio` fixtures) rather than silently patched
   mid-ticket.

## stdio_gaps

None. Every real geometric pair (mesh↔obj, mesh↔stl) fem needed already exists in stdio (W4 G2),
used as-is via `SemioMeshToObj`/`SemioMeshToStl` + `obj`/`stl`'s own real grammar encoders. zip and
png have no stdio gap either — they're simply not honest targets for fem's domain data (see
per-pair judgment above); nothing to request from stdio.

## Files touched (created/modified/removed)

**Removed (16 leaf dirs, 32 files: 16×.rs + 16×.ts):**
- `🗿️artifacts/{◻2d,🧊️3d}/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/{🎒️zip/🔖️2.0,📷️png/🔖️1.2,🟪️stl/🔖️ascii,🧊️obj/🔖️3.0}/✳️any/{🦀️component.rs,🟦️component.ts}`
- `🗿️artifacts/{◻2d,🧊️3d}/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/{🎒️zip/🔖️2.0,📷️png/🔖️1.2}/✳️any/{🦀️component.rs,🟦️component.ts}`

**Created (4 real leaf dirs, 8 files):**
- `🗿️artifacts/{◻2d,🧊️3d}/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/{🧊️obj/🔖️3.0,🟪️stl/🔖️ascii}/✳️any/{🦀️component.rs,🟦️component.ts}`

**Modified:**
- `🗿️artifacts/{◻2d,🧊️3d}/🏅️standards/🔖️1/⚙️engine/🕸️meshing/🦀️component.rs` (new `build_semio_mesh_snapshot` fn + region)
- `🗿️artifacts/{◻2d,🧊️3d}/🏅️standards/🔖️1/🎹️composer/🦀️component.rs` (removed zip/png entries; doc comment)
- `🗿️artifacts/{◻2d,🧊️3d}/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs` (kind lists)
- `🗿️artifacts/{◻2d,🧊️3d}/🏅️standards/🔖️1/🪆️subsets/✳️any/🎹️composer/🦀️component.rs` (doc comment only — no code change)
- `🗿️artifacts/{◻2d,🧊️3d}/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/{📤️export/🧵️serializers,📥️import/🧩️deserializers}/🗿️artifacts/{📊️csv,📝️md,🔣️json}/.../🦀️component.rs` (10 files, lagging call-site fix, item 1 above)
- `📦️packages/🦀️rust/📦️glue.rs` (8 dead mod blocks removed, per shape/direction)

## Exit checklist

`cargo check -p semio-s-plugin-fem` — **0 errors, 59 warnings** (pre-existing warning shapes:
unused imports, dead struct fields, hidden-lifetime deprecation — none introduced by this ticket's
edits; full output in `w5a--fem-cargo-check.txt`, isolated-`CARGO_TARGET_DIR` run to avoid the
~15-way shared-lock contention from concurrent sibling W5a/W5b agents; a second run against the
SHARED target dir also finished clean in parallel, confirming the result — see raw
`fem-check-2.txt`/`fem-check-isolated.txt` in the ticket's session scratch, byte-identical warning
count 59 in both):

```
warning: `semio-s-plugin-fem` (lib) generated 59 warnings (run `cargo fix --lib -p semio-s-plugin-fem` to apply 55 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 4m 43s
```

`cargo test -p semio-s-plugin-fem --lib` — **324 passed, 8 failed** (all 8 pre-existing/unrelated,
see "Foreign/out-of-scope items" #3 above; full output in `w5a--fem-cargo-test.txt`):

```
test result: FAILED. 324 passed; 8 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.19s
```

LOC deleted: **280** (16 files, `wc -l` verified before deletion, see per-pair table above).
No new test files created; no glue/catalog/script.ts edits outside fem's own glue.rs; no
`ticket_close` called.
