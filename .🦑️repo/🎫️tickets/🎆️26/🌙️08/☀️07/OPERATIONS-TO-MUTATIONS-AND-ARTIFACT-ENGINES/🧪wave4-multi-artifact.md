# Wave 4 Report — Multi-Artifact Fan-Out (Assigned Plugins)

Scope: puzzle, block, fem, gis, procedural, trinity, reasoning (`semio-s-plugin-reasoning-mindmap`), playbook, space (home), sourcing (curate), demonstrator (playground), energy (model). Reference: `📜️normative-spec.md`, `🧪wave3-report.md`, lowpoly `🧬️mutations/` layout.

`DEVELOPER_DIR=/Library/Developer/CommandLineTools` for all gates. Logs: `🧪wave4-semio-s-plugin-<crate>-check.txt`.

## Final gate table (`cargo check -p <crate>`)

| Crate | Result | Notes |
|-------|--------|-------|
| `semio-s-plugin-energy` | **PASS** | Stub artifact + plugin engine; mutations facet present |
| `semio-s-plugin-block` | **PASS** | `🧬️mutations/` on ◻2d/🖐️5d/🧊️3d; `register_block_exports`; `DocumentApp` → `Mutation` |
| `semio-s-plugin-fem` | **PASS** | `🧬️mutations/` on fem2d/fem3d; `crate::analyses` import fix |
| `semio-s-plugin-sourcing` | **PASS** | Curate `🧬️mutations/` + `SourcingDocument` alias; `vcs` in glue |
| `semio-s-plugin-procedural` | **FAIL** | `🧬️mutations/` added; op/spr OpText deduped; `Procedural*Mutation` needs full `DslEnum` field attrs / generation variant wiring (~E0277) |
| `semio-s-plugin-puzzle` | **FAIL** | `🧬️mutations/` + slim `🔧️op`; 3d/5d play apps still old `&self` `DocumentApp`; `infinite_canvas` graph exports missing |
| `semio-s-plugin-trinity` | **FAIL** | `🧬️mutations/` on jack/rewrite; glue doc order fixed; `infinite_canvas` + duplicate imports in jack mutations root |
| `semio-s-plugin-space` | **FAIL** | Home app syntax repaired; `SHome` mutations wired; blocked on `semio_framework_os` backbone API + missing example DSL includes + `SHomeDiff` path |
| `semio-s-plugin-playbook` | **FAIL** | Kernel `playbook` mod path-added; `ui_wgpu` inside kernel `builder_kit` (no dep in plugin crate) |
| `semio-s-plugin-reasoning-mindmap` | **FAIL** | `🧬️mutations/`; same `infinite_canvas` / `graph::GraphExtension` surface as puzzle |
| `semio-s-plugin-gis` | **FAIL** | Transitive `semio-framework-surface` → `E0432` `dsl_core` (pre-wave4) |
| `semio-s-plugin-demonstrator` | **FAIL** | Transitive compile (process / playground wiring), not re-verified green |

## Wave 4 deliverables landed

- Ran `🧪wave4-migrate-assigned.py` + `🧪wave4-fix-post.py` (ticket folder): mutations roots, variant triads (where enum size allowed), slim op re-exports, grammar copies, plugin-wide `*Operation` → `*Mutation` (shielded), stub engines for playground/model/home where scripted.
- **PASS crates** have `🧬️mutations/`, `ArtifactEngine` on artifact engines, `protocol::Mutation`, app `Emit::mutations` / associated types aligned with framework B1.
- Puzzle ◻2d play app already on static `DocumentApp::handle`; 🧊️3d/🖐️5d restored from history but not migrated off `&self` API.
- Space `🏠️home`: `catalog_port_concrete` brace fix; `DocumentApp` impl header repair; `🧬️mutations/` + glue.

## Remaining work (ordered)

1. **Puzzle 3d/5d** — port play apps to static `DocumentApp` (mirror ◻2d); fix `BoardHost` / example JSON exports.
2. **Procedural 2d/3d** — restore `#[derive(dsl::DslEnum)]` parity with pre-migration op (generation nested variant, `#[dsl(...)]` on fields).
3. **Canvas plugins** (puzzle, trinity, reasoning) — reconcile `extern crate infinite_canvas as …` with current `graph`/`BoardHost` API.
4. **Space** — OS host re-exports (`semio_framework_os::*` backbone/space/workflow) + `SHomeDiff` facet path; trim or restore missing cross-plugin example DSL includes.
5. **Playbook** — either add `ui_wgpu` (or cfg-gate `builder_kit`) or split kernel UI from document mutations facet.
6. **GIS** — fix `semio-framework-surface` / `dsl_core` (blocks gis map + terrain).
7. **Demonstrator** — after process/playground green, re-check playground `⚙️engine` + minimal `SetDocument` mutation.

## Scripts (ticket folder)

| File | Role |
|------|------|
| `🧪wave4-migrate-assigned.py` | Bulk mutations facet + rename |
| `🧪wave4-fix-post.py` | Glue doc order, grammar copy, op slim, block/sourcing glue |
| `🧪wave4-migrate-remodel.py` | Full remodel reference (cad/draw/process) |

## Collateral

Kernel / shared issues from wave3 (store receipt, CAD gates) unchanged — re-run lowpoly tests if touching kernel.
