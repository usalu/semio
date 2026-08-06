# W2 DocumentApp fixups (plugins)

`DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo check -p semio-s-plugin-<name> --lib` on 2026-08-06.

## Summary

| Category | Count |
| --- | ---: |
| Play apps: `Self::whole_document_operation` (was invalid `self.` in assoc fn) | 8 files |
| Play apps: removed `LazyLock<Mutex<…>>` session cheats | 4 apps (draw, flow, procedural 2d/3d) |
| Play apps: `command_id` → `&'static str` (trait match) | ~47 component files |
| Plugin `📦️glue.rs`: `#![allow]` → `#[allow]` (crate root) | all affected plugin glues |
| Kernel: `impl_whole_record_config!` (`$self` → `$crate`, `::protocol::OperationDiff`) | 1 |
| Kernel: `pub extern crate self as semio_format` (dsl derive) | 1 |
| **Priority crates green (0 rustc errors)** | **norm, fem, mathematical** |

## Per-crate `cargo check --lib`

| Crate | Exit | Errors | First error (if any) |
| --- | ---: | ---: | --- |
| norm | 0 | 0 | — |
| fem | 0 | 0 | — |
| mathematical | 0 | 0 | — |
| draw | 101 | 2 | `semio-framework-os`: duplicate `Value` / `Mutex` imports (`store_sync` wiring) |
| note | 101 | 2 | same `semio-framework-os` |
| forms | 101 | 8 | unresolved `playbook` (kernel module not linked without `ui_wgpu` DAG) |
| flow | 101 | 189 | missing `semio-framework-os-flow` extension file `📃️list/🦀️component.rs` |
| sequence | 101 | 190 | `semio-s-imperative` duplicate `Identified`/`Patchable` on `engine::Step` |
| cad | 101 | 40 | unresolved `workflow_kernel::*` in framework-os glue |
| puzzle | 101 | 227 | missing infinite/canvas font assets + transitive deps |
| space | 101 | 227 | same font / infinite canvas assets |

## DocumentApp migration fixes applied

### Invalid `self.` in associated fns

- `🏗️fem` ◻2d/🧊️3d `import_media` → `Self::whole_document_operation`
- `🏭️process` 🧊️3d, `🌍️gis` ◻2d/🧊️3d, `💠️lowpoly`, `🔱️trinity` ♻️rewrite, `📐️cad` — same pattern

### OS-state session cheats removed (per-call `Default`)

- `🖍️draw` `DrawPlayApp`: removed `DRAWPLAYAPP_SESSION`; `handle`/`render` use `DrawSession::default()`
- `🌊️flow` `FlowPlayApp`: removed `FLOWPLAYAPP_EVAL_SESSION`
- `🌀️procedural` ◻2d/🧊️3d: removed `PROCEDURAL*EVAL_SESSION`

### Trait surface

- Batch: `fn command_id(...) -> &'static str` across play `🦀️component.rs` files (matches `DocumentApp::command_id`).
- No remaining `fn document_schema()` / `fn app_id()` / `self.document_schema()` under `✏️s/🔌️plugins/**/🎛️apps/**`.
- No remaining `LazyLock` session locks under play app components (verified `rg`).

### Registration

- `🪐️space` glue already uses `register_document_app::<HomeApp>(…)` / `::<SpaceApp>(…)` (factory closure form not present).

## Remaining (not DocumentApp-local)

1. **forms / playbook**: need `playbook` domain crate wired without pulling `ui_wgpu` into `semio-framework-os-kernel` (attempt to `pub mod playbook` in kernel glue failed on `ui_wgpu` / `dsl_value_to_json`).
2. **draw / note**: `semio-framework-os` host glue incomplete (`store_sync`, duplicate imports).
3. **flow / puzzle / space / cad**: missing framework assets, unwired modules, or `workflow_kernel` exports — environment/DAG, not play-app trait bodies.

## Files touched (high level)

- `✏️s/🔌️plugins`: draw, flow, procedural (2d/3d), fem, process, gis, lowpoly, trinity rewrite, cad; ~47 play apps `command_id`; plugin `📦️glue.rs` allow attrs.
- `🧰️framework/…/🏪️store/🦀️component.rs`: `impl_whole_record_config!`
- `🧰️framework/…/os-kernel/📦️glue.rs`: `pub extern crate self as semio_format`
