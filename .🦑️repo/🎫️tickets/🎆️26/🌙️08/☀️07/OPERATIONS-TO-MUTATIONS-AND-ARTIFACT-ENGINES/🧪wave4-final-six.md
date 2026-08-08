# Wave 4 Final Six — Mutations Facets

Scope: last six artifacts missing `🧬️mutations/` (present, sequence, layout, playbook, imperative, raster).
Reference: `📜️normative-spec.md`, `🧪wave3-report.md`, `🧪wave4-singles-a.md`.
Gate: `DEVELOPER_DIR=/Library/Developer/CommandLineTools` · `cargo check -p <crate> --lib`.
Op brand kept (`🔧️op`, `*.op.semio`, `OpText` / `OpBinary`).

## Gate table

| Artifact | Crate | Mutations | Engine | `cargo check` | Notes |
|----------|-------|-----------|--------|---------------|-------|
| 🎬️present | `semio-s-plugin-animate` | ✅ 4 triads | ✅ `PresentEngine` | **FAIL** | Pre-existing `engine::animate::*` / `TYPST_FONTS` — not mutations facet |
| 🎬️sequence | `semio-s-plugin-sequence` | ✅ 8 triads | ✅ `SequenceEngine` | **FAIL** | Pre-existing dag / `SequenceFixtureDsl` P6 helpers — not mutations facet |
| 📏️layout | `semio-s-plugin-layout` | ✅ 7 triads | ✅ `LayoutEngine` | **PASS** | Font include path repaired (`🖼️canvas/🖼️assets`) |
| 📖️playbook | `semio-s-plugin-playbook` | ✅ 9 triads (kernel re-export) | ✅ `PlaybookEngine` | **PASS** | Facet wraps `flow::playbook::PlaybookMutation`; P6 DocumentDsl/Op codecs in kernel |
| 📜️imperative | `semio-s-plugin-imperative` | ✅ 1 triad | ✅ `ImperativeEngine` | **PASS** | Removed dead `imperative_engine::bootstrap_linked_modules` |
| 🖨️raster | `semio-s-plugin-raster` | ✅ 5 triads | ✅ `RasterEngine` | **PASS** | Owned enum in mutations; OpText stays in `🔧️op` |

## Per-artifact deliverables

### 1. `🎞️animate` / `🎬️present`
- `🧬️mutations/`: `PresentMutation` + apply/inverse; triads `🎞tiles`, `📎set-source`, `📋set-tiles`, `🃏set-deck`
- Slim `🔧️op`: `PresentMutationDsl` + OpText/OpBinary bridge (Op brand kept)
- Glue `artifacts::present::mutations` + TS `present_mutations`
- Grammar/protocol already `start mutation` / `schema …mutation`

### 2. `🎬️sequence` / `🎬️sequence`
- `🧬️mutations/`: full enum + `sequence_fixture_mutations` + 8 step/edge triads
- Slim `🔧️op` re-exports; OpText remains in `📡️spr`
- Glue + TS `sequence_mutations`

### 3. `📏️layout` / `📏️layout`
- `🧬️mutations/`: enum + `apply_layout_mutation` / `inverse_layout_mutation` + 7 triads
- `LayoutDiff.mutations` field rename; OpText stays in `📡️spr`
- Glue + TS `layout_mutations`

### 4. `📖️playbook` / `📖️playbook`
- `🧬️mutations/`: re-exports kernel `PlaybookMutation` (+ `apply_playbook_edit_mutation`) with 9 triad stubs
- Plugin uses `pub use flow::playbook` (forms pattern) + `flow` / `ui_wgpu` deps
- Kernel: serde tag `mutation`; P6 handcrafted `DocumentDsl` / `DocumentPack` / `OpText` / `OpBinary`
- Glue + TS `playbook_mutations`

### 5. `📜️imperative` / `📜️imperative`
- `🧬️mutations/`: `ImperativeMutation` struct + `✂️step-collection` triad
- OpText stays in `📡️spr`; slim `🔧️op` re-exports
- Glue + TS `imperative_mutations`

### 6. `🖨️raster` / `🖨️raster`
- `🧬️mutations/`: enum + helpers + 5 layer triads
- Handcrafted OpText/OpBinary retained in `🔧️op`
- Glue + TS `raster_mutations`

## Kept Op brand

`🔧️op`, `*.op.semio`, `grammar <x>.op`, `OpText`, `OpBinary`, `print_op` / `parse_op` / `encode_op` / `decode_op`, `LanguageRole::Ops`.

## Helpers / logs (ticket folder)

- Installer: `🧪wave4-final-six-install.py`
- Per-crate logs: `🧪wave4-{animate,sequence,layout,playbook,imperative,raster}-check.txt`
- This report: `🧪wave4-final-six.md`

## Unrelated remaining blockers (outside mutations facet)

### animate
- Missing `crate::artifacts::present::engine::animate::{AnimateConfig, Scene, …}` surface
- Missing `TYPST_FONTS` in text engine

### sequence
- `infinite_board_port_directed_dag::DagLayoutOrientation` / dag wire helpers
- `SequenceFixtureDsl::__DSL_EXTENSION` / `__DSL_ENVELOPE_ID` (P6 DocumentDsl wiring on fixture DSL)

## Verdict

All six artifacts have `🧬️mutations/` on disk with triad layout, `ArtifactEngine`, Operation→Mutation renames in-plugin, grammar `start mutation`, protocol `.mutation`, glue wiring, and TS `*_mutations` exports.

Green `cargo check`: **layout, playbook, imperative, raster**. Animate/sequence still fail on pre-existing unrelated engine/dep surfaces; their mutations facets are present and wired.
