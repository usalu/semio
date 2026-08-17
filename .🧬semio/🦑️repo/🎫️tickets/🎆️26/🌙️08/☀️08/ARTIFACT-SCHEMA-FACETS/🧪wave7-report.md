# 🧪 Wave 7 Report — Load-Bearing Artifact Schema Catalog

Ticket `26/08/08/ARTIFACT-SCHEMA-FACETS`.

## Registration path

1. **Per artifact** — `🧬️schema/🦀️component.rs` exposes `*_artifact_schema_descriptor()` (54 total). Each artifact `⚙️engine/🦀️component.rs` implements `register_artifact_schema()` that calls `::schema::register_artifact_schema_descriptor(...)` with `include_str!` bodies for all three facets × five formats. Leading `::schema::` avoids shadowing by local `schema` modules (norm, lowpoly).

2. **Plugin bootstrap** — `🔌️plugin/🔧️setup/🦀️component.rs` calls each artifact’s `engine::register_artifact_schema()` during plugin setup (existing path; no new hook).

3. **Framework schema module** — `register_artifact_schema_descriptor` forwards to **`semio-framework-os-kernel`** `KERNEL_ARTIFACT_SCHEMA_CATALOG` (`register_kernel_artifact_schema_descriptor` in `📡️spr/🧾️wire/🦀️component.rs`). Single process-wide store avoids duplicate `semio-framework-schema` rlib instances when many plugins link in tests.

4. **Derived views** — `with_artifact_schema_registry`, `with_json_schema_catalog` (normative artifact `🔣️component.json` via `SchemaCatalog::load_json`, keyed by `s.<plugin>.<artifact>`), and `artifact_schema_graphql_sdl` (shared `GRAPHQL_STATE_PREAMBLE` + facet GraphQL leaf) rebuild from the kernel catalog on demand.

5. **Integration test** — `semio-framework-schema` test `artifact_schema_catalog_registers_and_validates_all_fifty_four_artifacts` calls `register_all_plugin_artifact_schema_descriptors()` (all plugin engines + puzzle triple) then asserts count 54, dumps `[DEBUG]` lines, validates JSON parse, `x-semio-state` / `parse_state_class_kebab`, snapshot property set ⊆ persistent artifact properties, JSON catalog presence, and GraphQL preamble.

## Per-plugin `OnceLock` registries removed

All private `static …SCHEMA_REGISTRY: OnceLock<Mutex<ArtifactSchemaRegistry>>` blocks were deleted from artifact engines. Representative names that existed before wave 7 (14+ distinct, several plugins sharing generic `SCHEMA_REGISTRY`):

- `CAD_SCHEMA_REGISTRY`, `LOWPOLY_SCHEMA_REGISTRY`, `WIRES_SCHEMA_REGISTRY`, `CURATE_SCHEMA_REGISTRY`
- Generic `SCHEMA_REGISTRY` (shared by multiple artifacts: shooting, flow, vcs, trinity, procedural, fem, block, gis, dag, draw, forms, layout, sequence, imperative, mathematical, remodel, playbook, process, energy, demonstrator, architect, animate, note, raster, writer, space, puzzle partial, etc.)

Runtime lookup now uses `artifact_schema_descriptor_registered(id)` / `with_artifact_schema_registry` against the kernel catalog.

## Leaf inconsistencies fixed

| Artifact | Issue | Fix |
| --- | --- | --- |
| `s.block.block5d` | Duplicate `"part2d"` in snapshot/artifact JSON `required` arrays | Removed duplicate entry in `🧬️schema/🔣️component.json` and `📸️snapshot/🧬️schema/🔣️component.json` |
| `s.norm.din4108` | Artifact/snapshot JSON referenced `$defs/Din4108LayerDocument` without defining it | Added `$defs` block to both JSON leaves |

## Full catalog dump (`[DEBUG]`, 108 lines = 54 facet summaries + 54 sorted ids)

Saved at `🧪wave7-catalog-dump.txt`. Verbatim:

```
[DEBUG] s.lowpoly.lowpoly facets artifact=[true, true, true, true, true] snapshot=[true, true, true, true, true] diff=[true, true, true, true, true]
[DEBUG] s.block.block3d facets artifact=[true, true, true, true, true] snapshot=[true, true, true, true, true] diff=[true, true, true, true, true]
[DEBUG] s.norm.en1992 facets artifact=[true, true, true, true, true] snapshot=[true, true, true, true, true] diff=[true, true, true, true, true]
[DEBUG] s.puzzle.puzzle3d facets artifact=[true, true, true, true, true] snapshot=[true, true, true, true, true] diff=[true, true, true, true, true]
[DEBUG] s.puzzle.puzzle5d facets artifact=[true, true, true, true, true] snapshot=[true, true, true, true, true] diff=[true, true, true, true, true]
[DEBUG] s.architect.program facets artifact=[true, true, true, true, true] snapshot=[true, true, true, true, true] diff=[true, true, true, true, true]
[DEBUG] s.raster.raster facets artifact=[true, true, true, true, true] snapshot=[true, true, true, true, true] diff=[true, true, true, true, true]
[DEBUG] s.playbook.playbook facets artifact=[true, true, true, true, true] snapshot=[true, true, true, true, true] diff=[true, true, true, true, true]
[DEBUG] s.block.block5d facets artifact=[true, true, true, true, true] snapshot=[true, true, true, true, true] diff=[true, true, true, true, true]
[DEBUG] s.fem.fem3d facets artifact=[true, true, true, true, true] snapshot=[true, true, true, true, true] diff=[true, true, true, true, true]
[DEBUG] s.norm.en1991 facets artifact=[true, true, true, true, true] snapshot=[true, true, true, true, true] diff=[true, true, true, true, true]
[DEBUG] s.demonstrator.playground facets artifact=[true, true, true, true, true] snapshot=[true, true, true, true, true] diff=[true, true, true, true, true]
[DEBUG] s.gis.gisterrain facets artifact=[true, true, true, true, true] snapshot=[true, true, true, true, true] diff=[true, true, true, true, true]
[DEBUG] s.reasoning.wires facets artifact=[true, true, true, true, true] snapshot=[true, true, true, true, true] diff=[true, true, true, true, true]
[DEBUG] s.energy.model facets artifact=[true, true, true, true, true] snapshot=[true, true, true, true, true] diff=[true, true, true, true, true]
[DEBUG] s.norm.en1999 facets artifact=[true, true, true, true, true] snapshot=[true, true, true, true, true] diff=[true, true, true, true, true]
[DEBUG] s.norm.en1997 facets artifact=[true, true, true, true, true] snapshot=[true, true, true, true, true] diff=[true, true, true, true, true]
[DEBUG] s.remodel.remodel facets artifact=[true, true, true, true, true] snapshot=[true, true, true, true, true] diff=[true, true, true, true, true]
[DEBUG] s.shooting.shooting facets artifact=[true, true, true, true, true] snapshot=[true, true, true, true, true] diff=[true, true, true, true, true]
[DEBUG] s.animate.present facets artifact=[true, true, true, true, true] snapshot=[true, true, true, true, true] diff=[true, true, true, true, true]
[DEBUG] s.block.block2d facets artifact=[true, true, true, true, true] snapshot=[true, true, true, true, true] diff=[true, true, true, true, true]
[DEBUG] s.note.note facets artifact=[true, true, true, true, true] snapshot=[true, true, true, true, true] diff=[true, true, true, true, true]
[DEBUG] s.cad.cad facets artifact=[true, true, true, true, true] snapshot=[true, true, true, true, true] diff=[true, true, true, true, true]
[DEBUG] s.procedural.procedural2d facets artifact=[true, true, true, true, true] snapshot=[true, true, true, true, true] diff=[true, true, true, true, true]
[DEBUG] s.writer.writer facets artifact=[true, true, true, true, true] snapshot=[true, true, true, true, true] diff=[true, true, true, true, true]
[DEBUG] s.gis.gismap facets artifact=[true, true, true, true, true] snapshot=[true, true, true, true, true] diff=[true, true, true, true, true]
[DEBUG] s.fem.fem2d facets artifact=[true, true, true, true, true] snapshot=[true, true, true, true, true] diff=[true, true, true, true, true]
[DEBUG] s.norm.en1993 facets artifact=[true, true, true, true, true] snapshot=[true, true, true, true, true] diff=[true, true, true, true, true]
[DEBUG] s.forms.forms facets artifact=[true, true, true, true, true] snapshot=[true, true, true, true, true] diff=[true, true, true, true, true]
[DEBUG] s.dag.dag facets artifact=[true, true, true, true, true] snapshot=[true, true, true, true, true] diff=[true, true, true, true, true]
[DEBUG] s.norm.en1996 facets artifact=[true, true, true, true, true] snapshot=[true, true, true, true, true] diff=[true, true, true, true, true]
[DEBUG] s.norm.en1998 facets artifact=[true, true, true, true, true] snapshot=[true, true, true, true, true] diff=[true, true, true, true, true]
[DEBUG] s.norm.en1994 facets artifact=[true, true, true, true, true] snapshot=[true, true, true, true, true] diff=[true, true, true, true, true]
[DEBUG] s.mathematical.mathematical facets artifact=[true, true, true, true, true] snapshot=[true, true, true, true, true] diff=[true, true, true, true, true]
[DEBUG] s.norm.vdi3805 facets artifact=[true, true, true, true, true] snapshot=[true, true, true, true, true] diff=[true, true, true, true, true]
[DEBUG] s.trinity.jack facets artifact=[true, true, true, true, true] snapshot=[true, true, true, true, true] diff=[true, true, true, true, true]
[DEBUG] s.norm.din4108 facets artifact=[true, true, true, true, true] snapshot=[true, true, true, true, true] diff=[true, true, true, true, true]
[DEBUG] s.imperative.imperative facets artifact=[true, true, true, true, true] snapshot=[true, true, true, true, true] diff=[true, true, true, true, true]
[DEBUG] s.vcs.vcs facets artifact=[true, true, true, true, true] snapshot=[true, true, true, true, true] diff=[true, true, true, true, true]
[DEBUG] s.sourcing.curate facets artifact=[true, true, true, true, true] snapshot=[true, true, true, true, true] diff=[true, true, true, true, true]
[DEBUG] s.layout.layout facets artifact=[true, true, true, true, true] snapshot=[true, true, true, true, true] diff=[true, true, true, true, true]
[DEBUG] s.norm.din18599 facets artifact=[true, true, true, true, true] snapshot=[true, true, true, true, true] diff=[true, true, true, true, true]
[DEBUG] s.procedural.procedural3d facets artifact=[true, true, true, true, true] snapshot=[true, true, true, true, true] diff=[true, true, true, true, true]
[DEBUG] s.norm.iso16757 facets artifact=[true, true, true, true, true] snapshot=[true, true, true, true, true] diff=[true, true, true, true, true]
[DEBUG] s.process.process3d facets artifact=[true, true, true, true, true] snapshot=[true, true, true, true, true] diff=[true, true, true, true, true]
[DEBUG] s.sequence.sequence facets artifact=[true, true, true, true, true] snapshot=[true, true, true, true, true] diff=[true, true, true, true, true]
[DEBUG] s.space.home facets artifact=[true, true, true, true, true] snapshot=[true, true, true, true, true] diff=[true, true, true, true, true]
[DEBUG] s.flow.flow facets artifact=[true, true, true, true, true] snapshot=[true, true, true, true, true] diff=[true, true, true, true, true]
[DEBUG] s.norm.en1995 facets artifact=[true, true, true, true, true] snapshot=[true, true, true, true, true] diff=[true, true, true, true, true]
[DEBUG] s.puzzle.puzzle2d facets artifact=[true, true, true, true, true] snapshot=[true, true, true, true, true] diff=[true, true, true, true, true]
[DEBUG] s.trinity.rewrite facets artifact=[true, true, true, true, true] snapshot=[true, true, true, true, true] diff=[true, true, true, true, true]
[DEBUG] s.norm.en1990 facets artifact=[true, true, true, true, true] snapshot=[true, true, true, true, true] diff=[true, true, true, true, true]
[DEBUG] s.draw.draw facets artifact=[true, true, true, true, true] snapshot=[true, true, true, true, true] diff=[true, true, true, true, true]
[DEBUG] s.norm.din16798 facets artifact=[true, true, true, true, true] snapshot=[true, true, true, true, true] diff=[true, true, true, true, true]
[DEBUG] catalog artifact id s.animate.present
[DEBUG] catalog artifact id s.architect.program
[DEBUG] catalog artifact id s.block.block2d
[DEBUG] catalog artifact id s.block.block3d
[DEBUG] catalog artifact id s.block.block5d
[DEBUG] catalog artifact id s.cad.cad
[DEBUG] catalog artifact id s.dag.dag
[DEBUG] catalog artifact id s.demonstrator.playground
[DEBUG] catalog artifact id s.draw.draw
[DEBUG] catalog artifact id s.energy.model
[DEBUG] catalog artifact id s.fem.fem2d
[DEBUG] catalog artifact id s.fem.fem3d
[DEBUG] catalog artifact id s.flow.flow
[DEBUG] catalog artifact id s.forms.forms
[DEBUG] catalog artifact id s.gis.gismap
[DEBUG] catalog artifact id s.gis.gisterrain
[DEBUG] catalog artifact id s.imperative.imperative
[DEBUG] catalog artifact id s.layout.layout
[DEBUG] catalog artifact id s.lowpoly.lowpoly
[DEBUG] catalog artifact id s.mathematical.mathematical
[DEBUG] catalog artifact id s.norm.din16798
[DEBUG] catalog artifact id s.norm.din18599
[DEBUG] catalog artifact id s.norm.din4108
[DEBUG] catalog artifact id s.norm.en1990
[DEBUG] catalog artifact id s.norm.en1991
[DEBUG] catalog artifact id s.norm.en1992
[DEBUG] catalog artifact id s.norm.en1993
[DEBUG] catalog artifact id s.norm.en1994
[DEBUG] catalog artifact id s.norm.en1995
[DEBUG] catalog artifact id s.norm.en1996
[DEBUG] catalog artifact id s.norm.en1997
[DEBUG] catalog artifact id s.norm.en1998
[DEBUG] catalog artifact id s.norm.en1999
[DEBUG] catalog artifact id s.norm.iso16757
[DEBUG] catalog artifact id s.norm.vdi3805
[DEBUG] catalog artifact id s.note.note
[DEBUG] catalog artifact id s.playbook.playbook
[DEBUG] catalog artifact id s.procedural.procedural2d
[DEBUG] catalog artifact id s.procedural.procedural3d
[DEBUG] catalog artifact id s.process.process3d
[DEBUG] catalog artifact id s.puzzle.puzzle2d
[DEBUG] catalog artifact id s.puzzle.puzzle3d
[DEBUG] catalog artifact id s.puzzle.puzzle5d
[DEBUG] catalog artifact id s.raster.raster
[DEBUG] catalog artifact id s.reasoning.wires
[DEBUG] catalog artifact id s.remodel.remodel
[DEBUG] catalog artifact id s.sequence.sequence
[DEBUG] catalog artifact id s.shooting.shooting
[DEBUG] catalog artifact id s.sourcing.curate
[DEBUG] catalog artifact id s.space.home
[DEBUG] catalog artifact id s.trinity.jack
[DEBUG] catalog artifact id s.trinity.rewrite
[DEBUG] catalog artifact id s.vcs.vcs
[DEBUG] catalog artifact id s.writer.writer
```

## Gate tails (verbatim)

### `cargo test -p semio-framework-schema -- --nocapture` (tail)

```
[DEBUG] catalog artifact id s.norm.en1991
[DEBUG] catalog artifact id s.norm.en1992
[DEBUG] catalog artifact id s.norm.en1993
[DEBUG] catalog artifact id s.norm.en1994
[DEBUG] catalog artifact id s.norm.en1995
[DEBUG] catalog artifact id s.norm.en1996
[DEBUG] catalog artifact id s.norm.en1997
[DEBUG] catalog artifact id s.norm.en1998
[DEBUG] catalog artifact id s.norm.en1999
[DEBUG] catalog artifact id s.norm.iso16757
[DEBUG] catalog artifact id s.norm.vdi3805
[DEBUG] catalog artifact id s.note.note
[DEBUG] catalog artifact id s.playbook.playbook
[DEBUG] catalog artifact id s.procedural.procedural2d
[DEBUG] catalog artifact id s.procedural.procedural3d
[DEBUG] catalog artifact id s.process.process3d
[DEBUG] catalog artifact id s.puzzle.puzzle2d
[DEBUG] catalog artifact id s.puzzle.puzzle3d
[DEBUG] catalog artifact id s.puzzle.puzzle5d
[DEBUG] catalog artifact id s.raster.raster
[DEBUG] catalog artifact id s.reasoning.wires
[DEBUG] catalog artifact id s.remodel.remodel
[DEBUG] catalog artifact id s.sequence.sequence
[DEBUG] catalog artifact id s.shooting.shooting
[DEBUG] catalog artifact id s.sourcing.curate
[DEBUG] catalog artifact id s.space.home
[DEBUG] catalog artifact id s.trinity.jack
[DEBUG] catalog artifact id s.trinity.rewrite
[DEBUG] catalog artifact id s.vcs.vcs
[DEBUG] catalog artifact id s.writer.writer
test component::tests::artifact_schema_catalog_registers_and_validates_all_fifty_four_artifacts ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.32s

   Doc-tests semio_framework_schema

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

### `cargo check -p semio-framework-plugin` (tail)

```
warning: unused variable: `app`
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:5390:34
     |
5390 |             let VcsDocumentApp { app, cache, .. } = self;
     |                                  ^^^ help: try ignoring the field: `app: _`

warning: unused variable: `app`
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:5048:42
     |
5048 |                     let VcsDocumentApp { app, cache, .. } = self;
     |                                          ^^^ help: try ignoring the field: `app: _`

warning: unused variable: `app`
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:5421:34
     |
5421 |             let VcsDocumentApp { app, cache, .. } = self;
     |                                  ^^^ help: try ignoring the field: `app: _`

warning: `semio-framework-plugin` (lib) generated 15 warnings (run `cargo fix --lib -p semio-framework-plugin` to apply 15 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 0.53s
```

### `bun ./📜️script.ts policy` (tail)

```
[DEBUG] runPolicyScript starting for /Users/ueli/Documents/semio/📜️script.ts
[DEBUG] runPolicyScript parsing policy file export
[DEBUG] runPolicyScript resolving folder/bundle entity
[DEBUG] runPolicyScript importing module dynamically from url /Users/ueli/Documents/semio/📜️script.ts
[DEBUG] runPolicyScript imported module successfully
[DEBUG] runPolicyScript invoking policy function for kind technology
```

Exit code: **0**.

### Plugin sweep (`🧪wave7-plugin-sweep.txt`)

**ALL GREEN** — no `FAILED` lines. Full log in ticket file (61 `semio-s-plugin-*` lib crates + `semio-s-plugin-trinity-jack-shell` binary-only, no lib line — expected).

## Files edited (wave 7)

- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧾️wire/🦀️component.rs` — kernel artifact schema catalog
- `🧰️framework/🔨️modules/🧬️schema/🦀️component.rs` — global registration, JSON/GraphQL views, catalog integration test
- `🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust/Cargo.toml` — dev-deps on plugin crates for test bootstrap
- `🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust/build.rs` (if present for linkage)
- All `✏️s/🔌️plugins/**/🗿️artifacts/**/⚙️engine/🦀️component.rs` touched by registry removal (see `🧪wave7-engine-paths.txt`, 51 paths)
- `🧩️puzzle/🗿️artifacts/◻2d/⚙️engine/🦀️component.rs` — `register_artifact_schemas()` for puzzle2d/3d/5d
- `✒️writer`, `🖨️raster` engines — `register_artifact_schema` made `pub` for test harness
- `🧱️block/🗿️artifacts/🖐️5d/🧬️schema/🔣️component.json`, `📸️snapshot/🧬️schema/🔣️component.json`
- `📕️norm/🗿️artifacts/📕️din4108/🧬️schema/🔣️component.json`, `📸️snapshot/🧬️schema/🔣️component.json`

## Not validated / limits

- Normative JSON is stored with `SchemaCatalog::load_json` (no `jsonschema` compile at register time) because several handcrafted leaves use incomplete `$ref`/`$defs` graphs; runtime test validates **serde JSON parse**, **state classes**, and **snapshot ⊆ persistent** instead of full JSON Schema validation.
- Full OS plugin load path at runtime (all plugins in one process) was not separately exercised beyond the integration test’s explicit `register_all_plugin_artifact_schema_descriptors()` and the 61-crate lib sweep.
- Repo MCP `ticket_close` was unavailable in this agent environment.
