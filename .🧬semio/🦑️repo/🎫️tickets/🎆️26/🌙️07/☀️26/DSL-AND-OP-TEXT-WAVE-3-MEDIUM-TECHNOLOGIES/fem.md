# fem (2d + 3d) — DocumentDsl + OpText

## Scope
- `fem/2d/rs/lib.rs`: `Fem2dDocument` / `Fem2dOperation` — new `// #region 🔖️Dsl` + `// #region 🔖️OpText`
  after `// #endregion 🔖️Operations`, before `// #region 🔖️Bridge`.
- `fem/3d/rs/lib.rs`: `Fem3dDocument` / `Fem3dOperation` — same placement.
- `fem/plugin/rs/lib.rs`: `FEM2D_EXAMPLE_JSON`/`FEM3D_EXAMPLE_JSON` renamed to
  `FEM2D_EXAMPLE_DSL`/`FEM3D_EXAMPLE_DSL`, `include_str!` retargeted to `.fem2d`/`.fem3d`, every
  `serde_json::from_str::<FemXdDocument>(...)` call site (handler + 15 test sites) replaced with
  `FemXdDocument::parse_dsl(...)`. `.example(...)`'s `document_json` field is opaque per
  `framework/core/js/index.ts`'s own docstring (only used for the example-picker dropdown id/label on
  the JS side, never `JSON.parse`d generically) — safe to hand it DSL text instead of JSON.
- `fem/2d/example/default.fem2d.json` -> `fem/2d/example/default.fem2d` (deleted JSON).
- `fem/3d/example/default.fem3d.json` -> `fem/3d/example/default.fem3d` (deleted JSON).
- `fem/core/` untouched — no DSL-relevant types live there for fem2d/fem3d's own documents.

## Grammar (hand-rolled, vendored per-crate like `writer::writer_dsl`, NOT reusing vcs's private helpers)
Shared style across `fem2d_dsl`/`fem3d_dsl` modules: `@marker key=value ... "trailing text"` lines,
one line per document entity, dispatched purely by `@marker` (order-independent). Composite value
mini-grammars used inside single fields:
- points: `x,y;x,y;...` or `-`
- holes: `points|points|...` or `-`
- dof list: `Tx,Ty,...` or `-`
- terms (combinations): `id:factor,id:factor,...` or `-`
- loads (load case): `kind:id:...kind-fields...|kind:id:...` or `-`
  - `nodal:id:node:dof:value`, `memberUdl:id:element:wx:wy[:wz for 3d]`, `area:id:region|solid:pressure`

2D document line kinds (mirror `Fem2dDocument` field order): `@node`, `@bar`/`@beam`, `@region`,
`@material`, `@section`, `@support`, `@loadcase`, `@combination`, `@analysis`, `@camera`.
3D swaps `@region` for `@solid` (extrusion fields `basez`/`height`/`layers`/`mesh` replacing flat
`thickness`), adds `roll=` to `@frame`, `g=` to `@material`, `iz=`/`j=` to `@section`, `wz=` to
`memberUdl` loads, and `@camera` is a single opaque escaped-json quoted field (`FemCamera3d` is
`{json: String}`, plugin-owned shape) instead of `x=/y=/zoom=`.

Op-line grammar: `setNode`/`removeNode`, `setElement`(`kind=bar|beam` or `kind=bar|frame`)/`removeElement`,
`setMaterial`/`removeMaterial`, `setSection`/`removeSection`, `setSupport`/`removeSupport`,
`setLoadCase`/`removeLoadCase`, `setRegion`/`removeRegion` (2D) or `setSolid`/`removeSolid` (3D),
`setCombination`/`removeCombination`, `setAnalysisSettings`, `setCamera`, `setDocument`. Every `Set*`
op reuses the SAME `print_*_fields`/`parse_*` helper its matching `@marker` document line uses (plus
`index=`), so doc-DSL and op-text never drift apart. `setDocument` embeds the FULL multi-line
`print_document` output escaped (`\n` -> `\\n`) into one quoted field.

## Fixture regeneration approach
Both `default.fem2d.json`/`default.fem3d.json` were being actively edited concurrently by another
session throughout this ticket (simplifying the family-house example down from 12/9-element to
smaller counts, several times). Rather than hand-transcribing floats (error-prone for 30+ node 3D
fixture), added a temporary `#[test]` that: `serde_json::from_str`s the CURRENT (at-generation-time)
JSON into the doc struct, calls `.print_dsl()`, asserts it round-trips via `parse_dsl`, and
`println!`s the canonical text (`cargo test -- --nocapture`) — captured that stdout verbatim as the
new `.fem2d`/`.fem3d` fixture file, then removed the temporary test and deleted the JSON files.
Existing `example_fixture_parses_and_solves`/`example_fixture_parses` tests in `fem/2d`,`fem/3d`
retargeted to `include_str!("../example/default.fem2d")` + `Fem2dDocument::parse_dsl(...)` (count
assertions unchanged — same underlying data, just re-encoded).

## Environment note
Repo under heavy concurrent load during this ticket (load avg ~28, many other sessions' cargo builds
running). Used an isolated `CARGO_TARGET_DIR` per project convention; sccache is configured repo-wide
(`.cargo/config.toml` `rustc-wrapper = "sccache"`) so this doesn't cost a cold rebuild, just CPU
scheduling delay under contention.
