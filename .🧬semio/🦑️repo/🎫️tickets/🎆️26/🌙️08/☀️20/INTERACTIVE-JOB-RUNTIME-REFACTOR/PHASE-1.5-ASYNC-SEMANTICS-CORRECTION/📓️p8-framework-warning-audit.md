# Phase 8 Framework Warning Baseline

## Exact Gate

`CARGO_TARGET_DIR='…/EVERY-TOOL-INTERACTIVE-JOB-MIGRATION/🧪️target-p8-runtime' cargo clippy -p semio-framework-plugin --all-targets -- -D warnings`

Executed from `🧰️framework/📦️packages/🦀️rust`. The framework cohort contains 81 errors.

| Count | Lint | Primary file |
| ---: | --- | --- |
| 18 | `async_fn_in_trait` | `🔨️modules/🚪️io/🦀️component.rs` |
| 29 | `clippy::double_must_use` | `🔨️modules/🚪️io/🦀️component.rs` |
| 12 | `clippy::result_large_err` | `🔨️modules/🚪️io/🦀️component.rs` |
| 3 | `clippy::await_holding_lock` | `🔨️modules/🚪️io/🦀️component.rs` |
| 1 | `clippy::len_without_is_empty` | `🔨️modules/🚪️io/🦀️component.rs` |
| 1 | `clippy::type_complexity` | `🔨️modules/🚪️io/🦀️component.rs` |
| 4 | `async_fn_in_trait` | `🔨️modules/🎠️kernel/🦀️component.rs` |
| 3 | `async_fn_in_trait` | `🔨️modules/🛂️manifest/🦀️component.rs` |
| 1 | `clippy::large_enum_variant` | `🔨️modules/🛂️manifest/🦀️component.rs` |
| 2 | `clippy::map_unwrap_or` | `🔨️modules/🛂️manifest/🦀️component.rs` |
| 1 | `clippy::wrong_self_convention` | `🔨️modules/🛂️manifest/🦀️component.rs` |
| 6 | `clippy::map_unwrap_or` | `🛍️products/💻️os/🔨️modules/🔁️workflow/🦀️component.rs` |

## First Mechanical Repairs

Replaced all six workflow `map(...).unwrap_or...` sites and both manifest sites with the direct `map_or`/`map_or_else`/`is_some_and`/`is_ok_and` forms. The remaining work is concentrated in the public async-trait contracts and I/O registry error representations; those require coordinated contract changes rather than lint suppression.

## Framework Cohort Corrections

The takeover baseline was 73 live errors after the eight prior mechanical repairs and concurrent manifest corrections. The original 81-error census remains the historical starting point.

- Replaced all 25 public `async fn` trait declarations in framework IO, kernel broker hooks, and manifest media conversion with explicit `Future + Send` contracts. Existing implementations remain genuinely asynchronous and all framework targets prove their returned futures satisfy `Send`.
- Added `RandomAccessPayload::is_empty` as an asynchronous default which delegates to `len` and preserves codec diagnostics.
- Removed 29 redundant function-level `#[must_use]` attributes from `Result`-returning APIs.
- Boxed the owned large error payloads in `IoRegistryRegistrationError`, `ArtifactAssemblyRegistryError`, and the new IO mechanism's `IoRegistryError`; constructors, formatters, tests, and internal callers were updated together.
- Boxed `TutorialUiSample::Snapshot.state`, keeping the sparse delta representation small, and updated the owned composer and fixture callers.
- Cloned the IO mechanism's small registry index while each `RwLockReadGuard` is held, then released the guard before route, run, or identify futures suspend. No standard-library lock is held across an await.
- Introduced `FormatDescriptorIndexes` for the repeated paired catalog index type and renamed `MediaConverter::from_form` to `source_form`.
- Corrected the manifest interactive-job test imports/qualifications and removed six unnecessary mutable ActionBus test bindings.

## Narrow Test Exception

The routing-law `IoEntry` passthrough test double must implement the production fallible bare-function-pointer signature, but intentionally always succeeds. It has one item-level `clippy::unnecessary_wraps` allowance with an explicit reason. No production item or module has a lint allowance.

## Verification

All commands ran from `🧰️framework/📦️packages/🦀️rust` with the shared Phase 8 target directory.

| Command | Result |
| --- | --- |
| `cargo fmt -p semio-framework` followed by `cargo fmt -p semio-framework -- --check` | Passed. |
| `CARGO_TARGET_DIR=…/🧪️target-p8-runtime cargo clippy -p semio-framework --all-targets -- -D warnings` | Passed. |
| `CARGO_TARGET_DIR=…/🧪️target-p8-runtime cargo test -p semio-framework --all-targets` | Passed: 164 tests, 0 failed. |
| `CARGO_TARGET_DIR=…/🧪️target-p8-runtime cargo clippy -p semio-framework-plugin --all-targets -- -D warnings` | Framework cohort passed; gate advanced to the distinct OS plugin cohort. |

## Next Plugin-Gate Cohort

The exact final plugin-gate census contains 757 errors, all rooted under `🛍️products/💻️os/🔨️modules/🔌️plugin`: 623 in the plugin component, 56 in host, 29 in builder, and 49 across reactor/checkpoint/request/job leaves. The leading lint counts are 346 `result_large_err`, 199 `unused_qualifications`, 40 `map_unwrap_or`, 16 `needless_pass_by_value`, 16 `type_complexity`, and 15 `unused_imports`. This is a clearly different, unowned cohort; no Puzzle, FEM, Animate, renderer, or framework-2d source was edited. The renderer's tutorial recorder has one out-of-scope `TutorialUiSample::Snapshot` constructor which must wrap its snapshot in `Box::new` when the renderer cohort adopts the smaller enum representation.
