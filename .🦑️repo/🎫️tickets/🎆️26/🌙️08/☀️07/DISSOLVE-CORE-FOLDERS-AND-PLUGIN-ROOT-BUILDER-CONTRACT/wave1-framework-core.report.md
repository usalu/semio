# Wave 1 report — dissolve framework core

## Summary

Dissolved framework modules core (puzzle+core, no VS16) into concept siblings. Split the ~4090-line TypeScript monolith into per-module component.ts files. Renamed Rust crate semio-framework-core to semio-framework and TS package @semio-tech/framework-core to @semio-tech/framework. Deleted empty core folder.

## Created

- modules/manifest/ (Rust from former core/ui, TS split, generated/ moved under it)
- modules/kernel/ (lifted from core/ui/kernel + TS runtime/playground/lease modules)
- action-bus/component.ts (action arg / utility derivation helpers)
- platform/component.ts (element ids, presence, dock/pane stores, inspector helpers)
- mesh/component.ts (component scene protocol)
- ticket: deferred-framework-core.json
- ticket: original-component.ts (git recovery snapshot used for TS resplit)
- ticket: wave1-framework-cargo-check.log

## Updated

- packages/rust/glue.rs — #[path] to sibling modules; pub mod manifest (+ alias ui)
- packages/rust/Cargo.toml — name=semio-framework, metadata id=framework
- packages/rust/script.ts — typegen/clippy package name + generated output under manifest/
- packages/typescript/package.json — name=@semio-tech/framework
- packages/typescript/glue.ts — reexports all five modules + inline vitest region
- packages/typescript/vitest.config.ts — package name + glue entry
- Non-plugin Cargo.toml path deps: package/dep key/feature dep:semio-framework
- Plugin Cargo.toml path-dep rename only (semio-framework-core -> semio-framework) so workspace resolves
- Framework + consumer TS package.json/imports no longer reference @semio-tech/framework-core (scan=0)
- manifest/component.rs kernel #[path] -> ../kernel/component.rs

## Removed

- modules/core/ (entire folder, including temporary barrel and empty ui/)

## Deferred shared edits (see deferred-framework-core.json)

- Root Cargo.toml workspace alias: already observed as semio-framework; Wave 2 should confirm lock/aliases
- Root script.ts / eslint / storybook / dependency-cruiser may still mention old core path strings (Wave 2; policy region not touched)
- cargo check -p semio-framework blocked by parallel wave1 flow-ext rename (missing plugins/flow/extensions/core path referenced by flow plugin Cargo). Not caused by this dissolve.

## Layout after dissolve

modules/action-bus/
modules/platform/
modules/mesh/
modules/manifest/ (+ generated/)
modules/kernel/
(no modules/core)

## Notes

- Glue keeps `pub use manifest as ui` so existing `::ui::` / crate::ui paths keep resolving during Wave 2.
- TS tests live in packages/typescript/glue.ts (import.meta.vitest) to avoid circular imports across module files.
- Did not edit root script.ts policy region.
- Did not restructure plugin trees beyond Cargo package-name path-dep rename required by crate rename.

## Remaining 🧩core path strings (Wave 2)

- script.ts
- 📜️script.ts
- eslint.config.mjs
