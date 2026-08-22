# Phase 8 Schema and UI Warning Baseline

## Exact gate

```text
CARGO_TARGET_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/EVERY-TOOL-INTERACTIVE-JOB-MIGRATION/🧪️target-p8-runtime' cargo clippy -p semio-framework-plugin --all-targets -- -D warnings
```

Run from `🧰️framework/📦️packages/🦀️rust`; exit status: `101`.

## Failures

| Package | File | Line(s) | Lint |
| --- | --- | --- | --- |
| UI | `🎯️targets/🧊️wgpu/🦀️component.rs` | 2939 | `clippy::empty_line_after_doc_comments` |
| Schema | `🦀️component.rs` | 106, 107, 108, 148, 151 | `async_fn_in_trait` |
| Schema | `🦀️component.rs` | 272 | `clippy::len_without_is_empty` |
| Schema | `🦀️component.rs` | 299 | `clippy::needless_pass_by_value` |
| Schema | `🦀️component.rs` | 354, 377, 508, 531, 674, 698 | `clippy::double_must_use` |
| Schema | `🦀️validator.rs` | 286 | `clippy::map_unwrap_or` |
| UI | `🖼️assets/🔣️icons/🤖️generated/🦀️icon_name.rs` | 1012 | `clippy::should_implement_trait` |
| UI | `🎯️targets/🧊️wgpu/🦀️component.rs` | 334, 339, 342 | `clippy::map_unwrap_or` |
| UI | `🎯️targets/🧊️wgpu/🦀️component.rs` | 484 | `clippy::manual_map` |
| UI | `🎯️targets/🧊️wgpu/🦀️component.rs` | 1455, 1482 | `clippy::clone_on_copy` |

The gate reported 14 schema errors and 8 UI errors. No non-schema/UI source errors were emitted before compilation stopped.
