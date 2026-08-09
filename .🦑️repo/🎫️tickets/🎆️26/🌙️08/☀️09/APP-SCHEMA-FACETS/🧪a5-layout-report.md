# A5 Layout Report

## Summary

Implemented app-schema facets for owner `✏️s/🔌️plugins/📏️layout/🎛️apps/📏️layout/🎚️config` (`LayoutConfig` / `LayoutPresence`).

- Config schema leaf set mirrors runtime `LayoutConfig` exactly (nested `LayoutCamera` + `LayoutDropPreviewState`, all `local-ui`).
- Presence runtime + schema: shareable live subset — `activePageId`, `selectedIds`, `hoveredId`, `dropPreview`, `camera`, `previewCamera` (all `shared-ui`); excludes local-only `engagementInput` and `locale`.
- Wired `📦️glue.rs` with nested `config { component; schema }` and `presence { component; schema }`.
- `LayoutPlayApp` now binds `type Presence = LayoutPresence` / `type PresenceMutation = LayoutPresenceMutation`.

## Files touched

### Created
- `✏️s/🔌️plugins/📏️layout/🎛️apps/📏️layout/🎚️config/🧬️schema/🦀️component.rs`
- `✏️s/🔌️plugins/📏️layout/🎛️apps/📏️layout/🎚️config/🧬️schema/🟦️component.ts`
- `✏️s/🔌️plugins/📏️layout/🎛️apps/📏️layout/🎚️config/🧬️schema/🔗️component.graphql`
- `✏️s/🔌️plugins/📏️layout/🎛️apps/📏️layout/🎚️config/🧬️schema/🔣️component.json`
- `✏️s/🔌️plugins/📏️layout/🎛️apps/📏️layout/🎚️config/🧬️schema/🛰️component.proto`
- `✏️s/🔌️plugins/📏️layout/🎛️apps/📏️layout/👥️presence/🦀️component.rs`
- `✏️s/🔌️plugins/📏️layout/🎛️apps/📏️layout/👥️presence/🧬️schema/🦀️component.rs`
- `✏️s/🔌️plugins/📏️layout/🎛️apps/📏️layout/👥️presence/🧬️schema/🟦️component.ts`
- `✏️s/🔌️plugins/📏️layout/🎛️apps/📏️layout/👥️presence/🧬️schema/🔗️component.graphql`
- `✏️s/🔌️plugins/📏️layout/🎛️apps/📏️layout/👥️presence/🧬️schema/🔣️component.json`
- `✏️s/🔌️plugins/📏️layout/🎛️apps/📏️layout/👥️presence/🧬️schema/🛰️component.proto`

### Updated
- `✏️s/🔌️plugins/📏️layout/📦️packages/🦀️rust/📦️glue.rs`
- `✏️s/🔌️plugins/📏️layout/🎛️apps/📏️layout/🦀️component.rs`

### Report
- `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️09/APP-SCHEMA-FACETS/🧪a5-layout-report.md`

## Gate tails

### 1. Scoped policy (`policyAppSchemaBreaches` filtered to layout)

```
0
```

### 2. `cargo check -p semio-s-plugin-layout`

```
    |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
543 -     type Snapshot = crate::artifacts::layout::LayoutSnapshot;
543 +     type Snapshot = LayoutSnapshot;
    |

warning: `semio-s-plugin-layout` (lib) generated 22 warnings (run `cargo fix --lib -p semio-s-plugin-layout` to apply 14 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 3m 04s
warning: the following packages contain code that will be rejected by a future version of Rust: block v0.1.6
note: to see what the problems were, use the option `--future-incompat-report`, or run `cargo report future-incompatibilities --id 1`
```

### 3. `cargo test -p semio-s-plugin-layout --lib`

```
test artifacts::layout::spr::tests::op_text_round_trips_every_layout_operation_variant ... ok
test artifacts::layout::spr::tests::op_text_round_trips_full_page_and_frame_patch_fields ... ok
test artifacts::layout::spr::tests::parse_op_reports_engine_parser_errors ... ok
test artifacts::layout::engine::scene::tests::layout_story_in_frame_resolves_alignment_variants_and_detects_overset ... ok
test artifacts::layout::engine::scene::tests::marks_hovered_frame_rect ... ok
test artifacts::layout::engine::scene::tests::scene_and_hit_test_error_when_page_missing ... ok
test artifacts::layout::engine::scene::tests::pdf_export_writes_pdf_header ... ok
test artifacts::layout::engine::scene::tests::png_cpu_export_writes_valid_rgba_png ... ok
test artifacts::layout::engine::scene::tests::svg_export_contains_rect_and_wraps_a_valid_document ... ok
test artifacts::layout::engine::scene::tests::scene_png_from_display_list_writes_a_valid_png ... ok

test result: ok. 116 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.97s
```

## Unverified

- No end-to-end multi-peer presence UI wiring (schema + DocumentApp types only; peers still need runtime consumers).
- Did not run full-repo `bun ./📜️script.ts policy` (explicitly out of scope per fan-out brief).
