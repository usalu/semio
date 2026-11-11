# Framework batch B, text crates

All three LIB targets pass `check --lib` (logs: 🗑️generated/batch-b-*-r5.log).

## Changes
- 2d text-font `🦀️.rs`, `🧵️outline/🦀️.rs`, `🤝️kerning/🦀️.rs`: RetireOwned impls moved to `RetirementCursor` + `retirement_seed`; the owner structs became pub (E0446). Outline `append/step/advance` now take `&mut RetainedAllocationEffect` and a local `slot` helper maps the effect-based `work.slot` result. `GlyphOutlineCursor::advance` gained an `effect` parameter, so callers in font run/shape (other crates) must pass one.
- 2d text-font-testing `🦀️.rs` + Cargo.toml: `close_with_items` now drives `ControlledRetirement::step(body,effect)` through `EffectBank` (new dependency `semio-framework-value-testing-effect-bank`); added `WORK`.
- text-splice-native `🦀️.rs` + Cargo.toml: constructions use `admit_original_construction_settled` (new dependency `semio-framework-action-bus`), decoder identity is `NativeAllocationIdentity`, close uses `close_original_effect_step`.

## Shared modules
None touched.


## Sibling font crates (second pass)
Lib checks pass for text-font-catalog, text-font-shape-unicode, text-font-shape, text-font-run (logs batch-b-*-L*.log, s*.log).
- All font `RetireOwned` impls migrated to `RetirementCursor` + `retirement_seed`; owner structs made pub.
- `work::slot(work,list,effect)` helper added in font `🎟️work/🦀️.rs`; every step/advance in catalog, shape, unicode and run takes `effect:&mut RetainedAllocationEffect`; `contiguous` receives `effect.actual()`.
- Primitive cursors without allocation (kerning, probes, mapper) keep their old signatures.


## Third pass
- Test targets (`check -p <crate> --tests`) pass for text-font, catalog, shape-unicode, shape, run and testing. The testing crate gained `with_work` and `with_effect` helpers (EffectBank-backed); test call sites use them. The workspace test `close` delegates to the testing crate.
- Out-of-family callers migrated but NOT verifiable: ✏️s draw `🎬️scene/🔤️text/🦀️.rs` (DocumentTextJob step/advance take an effect) and layout `🎬️scene/📄️paragraph/🦀️.rs` (same threading). `check` of semio-s-artifact-draw-drawing-scene (122 errors) and semio-s-artifact-layout-layout-engine (193) stops in upstream crates owned by others (plugin document-media, builder program/reactor surfaces native.rs, media intrinsic RetireOwned); no error in my two files was reached.
- Callers of the draw `advance`/layout paragraph step further up (prepare/…) will need an effect once the upstream crates compile.
