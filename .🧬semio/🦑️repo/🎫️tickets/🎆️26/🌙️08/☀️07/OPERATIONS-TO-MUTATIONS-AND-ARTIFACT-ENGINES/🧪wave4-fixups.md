# Wave 4 Fixups — Plugin `cargo check` Results

`DEVELOPER_DIR=/Library/Developer/CommandLineTools`. Per-crate logs: `🧪wave4-fixup-<plugin>-check.txt`. Summary TSV: `🧪wave4-fixups-summary.tsv`.

## Gate summary

| Result | Count |
|--------|------:|
| **PASS** | **22** |
| **FAIL** | **10** |
| Total plugin crates | **32** |

## PASS

| Crate | Notes |
|-------|-------|
| `semio-s-plugin-architect` | |
| `semio-s-plugin-block` | |
| `semio-s-plugin-cad` | |
| `semio-s-plugin-draw` | |
| `semio-s-plugin-energy` | |
| `semio-s-plugin-fem` | |
| `semio-s-plugin-forms` | `use crate::playbook::` (module re-export) |
| `semio-s-plugin-gis` | `dsl_core` → `dsl::os_dsl`; `register_gis_exports`; gismap OpText via spr |
| `semio-s-plugin-imperative` | |
| `semio-s-plugin-layout` | |
| `semio-s-plugin-lowpoly` | Wave 3 reference |
| `semio-s-plugin-mathematical` | |
| `semio-s-plugin-norm` | |
| `semio-s-plugin-note` | |
| `semio-s-plugin-playbook` | |
| `semio-s-plugin-procedural` | spr DSL mirror OpText; no `DslEnum` on mutation enum |
| `semio-s-plugin-process` | |
| `semio-s-plugin-raster` | |
| `semio-s-plugin-remodel` | |
| `semio-s-plugin-shooting` | |
| `semio-s-plugin-sourcing` | |
| `semio-s-plugin-vcs` | |

## FAIL (first error)

| Crate | First error | Likely root |
|-------|-------------|-------------|
| `semio-s-plugin-animate` | `present::engine::animate::{AnimateConfig, QualityPreset}` | engine path / module wiring |
| `semio-s-plugin-dag` | `DagDiff` missing `.apply` | MutationDiff vs Diff API |
| `semio-s-plugin-demonstrator` | `semio_framework_core` unresolved | wrong crate alias (transitive) |
| `semio-s-plugin-flow` | `default_contributions_json` missing | forms/playbook contributions helper |
| `semio-s-plugin-puzzle` | `semio_framework_core` unresolved | leftover import; DocumentApp partially migrated |
| `semio-s-plugin-reasoning-mindmap` | `DefaultWiresExtension: GraphExtension` | mindmap vs infinite `GraphExtension` |
| `semio-s-plugin-sequence` | `dag::computation_node_width` missing | more dag crate-root re-exports needed |
| `semio-s-plugin-space` | `crate::store_sync` in OS host | `os-host-full` modules not fully wired |
| `semio-s-plugin-trinity` | `lex` not in scope | lexer not re-exported; OpText dups; op vs mutations paths |
| `semio-s-plugin-writer` | same `lex` via trinity | blocked on trinity |

## Fixups landed

1. **Procedural** — drop `DslEnum` on mutation enums; OpText/OpBinary via spr DSL mirrors; keep `🔧️op` + grammar; `apply_generation_mutation` in playbook kernel.
2. **Puzzle** — static `DocumentApp` associated types / `handle`; TLS play sessions for 3d/5d; Mutation Value bridges; `vcs` + `blake3`.
3. **Infinite** — crate-root directed_normal + expanded directed_dag re-exports (`BoardHost`, layouts, `GraphExtension`, `dag_fixture_to_wire_literal`, `DagLayoutOrientation`, `EdgeRouteStyle`, `PortShape`).
4. **GIS** — surface `dsl` fix; `register_gis_exports`; gismap op slimmed.
5. **Space** — fixture includes → demo DSL files; `SHomeDiff` alias; enabled `os-host-full` (still blocked on `store_sync`).
6. **Trinity** — mutations glue; `TrinityGraphDocument` / `RewriteRuleDocument` aliases; `language_service as core`.
7. **Forms** — `playbook::` → `crate::playbook::`.
8. **Bulk** — `start mutation` in op/mutations grammars; document-mutation Operation→Mutation renames.

## Kept

- `🔧️op`, `OpText`, `OpBinary`, `*.op.semio`, `LanguageRole::Ops`
- Draw domain `SetBooleanOperation` / `boolean_operation`

## Next blockers (ordered)

1. Trinity: re-export `lex`, dedupe OpText, point store helpers at `mutations` (unblocks writer).
2. Puzzle: replace `semio_framework_core`; finish BoardHost/example gaps.
3. Space: wire `store_sync` under `os-host-full` (or split host features).
4. Flow contributions helper; sequence/dag infinite exports; animate engine paths; reasoning GraphExtension; demonstrator core alias.
