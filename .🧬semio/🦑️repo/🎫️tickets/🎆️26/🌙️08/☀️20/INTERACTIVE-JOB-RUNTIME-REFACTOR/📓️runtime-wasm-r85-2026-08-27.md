# Runtime Wasm R85

Actual canonical `check-wasm` exited0. The existing script checks `semio-framework-ui-runtime` serially for `wasm32-wasip2` then `wasm32-unknown-unknown`; both Cargo completion footers are present (2.32s and2.13s). Jobs2 and retained target unchanged. Compilation only: no guest instantiation, clock runtime, Plugin WIT/Pending caller, or interactive timing claim.

The runtime/contract/job sources remained unchanged from selected pre-input census R84 through these checks.

## Exact Command

```sh
set -o pipefail
SEMIO_COVERAGE=0 NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad' SEMIO_TEST_ARTIFACT_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts' SEMIO_BUILD_BUDGET_MS=3600000 RUST_BACKTRACE=1 bun x nx run @semio-tech/ui-runtime-rs:check-wasm --skip-nx-cache 2>&1 | tee '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️member-runtime-wasm-r85-2026-08-27.txt'
```

## Full Output

```text

> nx run @semio-tech/ui-runtime-rs:check-wasm

> bun ./📜️script.ts check-wasm

[0m[33mWarning[0m[2m:[0m [1mThe 'NO_COLOR' env is ignored due to the 'FORCE_COLOR' env being set.[0m
[0m      [2mat [0m[0m[1m[3mwarnOnDeactivatedColors[0m[2m ([0m[0m[36minternal:tty[0m[2m:[0m[33m33[0m[2m:[33m24[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3mgetColorDepth[0m[2m ([0m[0m[36minternal:tty[0m[2m:[0m[33m42[0m[2m:[33m39[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3mshouldColorize[0m[2m ([0m[0m[36minternal:util/colors[0m[2m:[0m[33m14[0m[2m:[33m109[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3mrefresh[0m[2m ([0m[0m[36minternal:util/colors[0m[2m:[0m[33m18[0m[2m:[33m31[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3minternal:util/colors[0m[2m ([0m[0m[36minternal:util/colors[0m[2m:[0m[33m24[0m[2m:[33m16[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3minternal:assert/assertion_error[0m[2m ([0m[0m[36minternal:assert/assertion_error[0m[2m:[0m[33m2[0m[2m:[33m187[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3mloadAssertionError[0m[2m ([0m[0m[36mnode:assert[0m[2m:[0m[33m28[0m[2m:[33m96[0m[2m)[0m

warning: unused import: `OnceLock`
  --> 🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust/../../🦀️component.rs:41:22
   |
41 | use std::sync::{Arc, OnceLock};
   |                      ^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: unused import: `std::time::Instant`
  --> 🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust/../../🦀️component.rs:43:5
   |
43 | use std::time::Instant;
   |     ^^^^^^^^^^^^^^^^^^

    Checking semio-framework-ui-contract v0.1.0 (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust)
warning: `semio-framework-job` (lib) generated 2 warnings (run `cargo fix --lib -p semio-framework-job` to apply 2 suggestions)
warning: unnecessary qualification
  --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../♻️retirement/🌳️typed/🦀️component.rs:16:6
   |
16 | impl std::fmt::Debug for UiTypedRetirementCursor {
   |      ^^^^^^^^^^^^^^^
   |
   = note: requested on the command line with `-W unused-qualifications`
help: remove the unnecessary path segments
   |
16 - impl std::fmt::Debug for UiTypedRetirementCursor {
16 + impl fmt::Debug for UiTypedRetirementCursor {
   |

warning: unnecessary qualification
  --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../♻️retirement/🌳️typed/🦀️component.rs:17:35
   |
17 |     fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
   |                                   ^^^^^^^^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
17 -     fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
17 +     fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> std::fmt::Result {
   |

warning: unnecessary qualification
  --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../♻️retirement/🌳️typed/🦀️component.rs:17:63
   |
17 |     fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
   |                                                               ^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
17 -     fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
17 +     fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> fmt::Result {
   |

warning: unnecessary qualification
   --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🦀️document.rs:543:5
    |
543 |     std::mem::size_of::<Mutex<UiDocumentArena>>() + std::mem::size_of::<crate::UiArenaHandbacks<UI_DOCUMENT_LEASE_SLOTS, 1>>()
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
543 -     std::mem::size_of::<Mutex<UiDocumentArena>>() + std::mem::size_of::<crate::UiArenaHandbacks<UI_DOCUMENT_LEASE_SLOTS, 1>>()
543 +     size_of::<Mutex<UiDocumentArena>>() + std::mem::size_of::<crate::UiArenaHandbacks<UI_DOCUMENT_LEASE_SLOTS, 1>>()
    |

warning: unnecessary qualification
   --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🦀️document.rs:543:53
    |
543 |     std::mem::size_of::<Mutex<UiDocumentArena>>() + std::mem::size_of::<crate::UiArenaHandbacks<UI_DOCUMENT_LEASE_SLOTS, 1>>()
    |                                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
543 -     std::mem::size_of::<Mutex<UiDocumentArena>>() + std::mem::size_of::<crate::UiArenaHandbacks<UI_DOCUMENT_LEASE_SLOTS, 1>>()
543 +     std::mem::size_of::<Mutex<UiDocumentArena>>() + size_of::<crate::UiArenaHandbacks<UI_DOCUMENT_LEASE_SLOTS, 1>>()
    |

warning: unused variable: `byte_candidate`
  --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../📋️copy/🦀️component.rs:68:71
   |
68 |             fn copy_one(&self, candidate: &mut Self, _: &mut [usize], byte_candidate: &mut Vec<u8>, _: usize, work: usize) -> Result<UiComponentCopyProgress, UiFixedListAllocationError> {
   |                                                                       ^^^^^^^^^^^^^^
...
76 | scalar!(bool, u16, u64, f64, UiNodeId, UiRevision, Activity, TransitionHint, StyleSpec, Trigger, ContainerRole, InputKind, RowActionPlacement, SurfaceKind, Liveness, GridTrack, SpaceToken, Align, Justify, EdgeSpace, Axis, Anchor, ScrollAxes, Sizing);
   | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- in this macro invocation
   |
help: `byte_candidate` is captured in macro and introduced a unused variable
  --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../📋️copy/🦀️component.rs:68:71
   |
68 |             fn copy_one(&self, candidate: &mut Self, _: &mut [usize], byte_candidate: &mut Vec<u8>, _: usize, work: usize) -> Result<UiComponentCopyProgress, UiFixedListAllocationError> {
   |                                                                       ^^^^^^^^^^^^^^
...
76 | scalar!(bool, u16, u64, f64, UiNodeId, UiRevision, Activity, TransitionHint, StyleSpec, Trigger, ContainerRole, InputKind, RowActionPlacement, SurfaceKind, Liveness, GridTrack, SpaceToken, Align, Justify, EdgeSpace, Axis, Anchor, ScrollAxes, Sizing);
   | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- in this macro invocation
   = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default
   = note: this warning originates in the macro `scalar` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: unused variable: `byte_candidate`
  --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../📋️copy/🦀️component.rs:68:71
   |
68 |             fn copy_one(&self, candidate: &mut Self, _: &mut [usize], byte_candidate: &mut Vec<u8>, _: usize, work: usize) -> Result<UiComponentCopyProgress, UiFixedListAllocationError> {
   |                                                                       ^^^^^^^^^^^^^^
...
76 | scalar!(bool, u16, u64, f64, UiNodeId, UiRevision, Activity, TransitionHint, StyleSpec, Trigger, ContainerRole, InputKind, RowActionPlacement, SurfaceKind, Liveness, GridTrack, SpaceToken, Align, Justify, EdgeSpace, Axis, Anchor, ScrollAxes, Sizing);
   | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- in this macro invocation
   |
help: `byte_candidate` is captured in macro and introduced a unused variable
  --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../📋️copy/🦀️component.rs:68:71
   |
68 |             fn copy_one(&self, candidate: &mut Self, _: &mut [usize], byte_candidate: &mut Vec<u8>, _: usize, work: usize) -> Result<UiComponentCopyProgress, UiFixedListAllocationError> {
   |                                                                       ^^^^^^^^^^^^^^
...
76 | scalar!(bool, u16, u64, f64, UiNodeId, UiRevision, Activity, TransitionHint, StyleSpec, Trigger, ContainerRole, InputKind, RowActionPlacement, SurfaceKind, Liveness, GridTrack, SpaceToken, Align, Justify, EdgeSpace, Axis, Anchor, ScrollAxes, Sizing);
   | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- in this macro invocation
   = note: this warning originates in the macro `scalar` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: unused variable: `byte_candidate`
  --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../📋️copy/🦀️component.rs:81:63
   |
81 | ... _: &mut [usize], byte_candidate: &mut Vec<u8>, _: usize, work: usize) -> Result<UiComponentCopyProgress, UiFixedListAllocationEr...
   |                      ^^^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_byte_candidate`

warning: unused variable: `candidate`
   --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../📋️copy/🦀️component.rs:49:34
    |
 49 |             fn allocation(&self, candidate: &Self, path: &[usize]) -> Result<usize, &'static str> {
    |                                  ^^^^^^^^^
...
237 | ui_typed_field_catalog!(typed_fields);
    | ------------------------------------- in this macro invocation
    |
help: `candidate` is captured in macro and introduced a unused variable
   --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../📋️copy/🦀️component.rs:49:34
    |
 49 |             fn allocation(&self, candidate: &Self, path: &[usize]) -> Result<usize, &'static str> {
    |                                  ^^^^^^^^^
...
237 | ui_typed_field_catalog!(typed_fields);
    | ------------------------------------- in this macro invocation
    = note: this warning originates in the macro `typed_fields` which comes from the expansion of the macro `ui_typed_field_catalog` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: unused variable: `path`
   --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../📋️copy/🦀️component.rs:50:29
    |
 50 |                 let (index, path) = read_path(path)?;
    |                             ^^^^
...
237 | ui_typed_field_catalog!(typed_fields);
    | ------------------------------------- in this macro invocation
    |
help: `path` is captured in macro and introduced a unused variable
   --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../📋️copy/🦀️component.rs:50:29
    |
 50 |                 let (index, path) = read_path(path)?;
    |                             ^^^^
...
237 | ui_typed_field_catalog!(typed_fields);
    | ------------------------------------- in this macro invocation
    = note: this warning originates in the macro `typed_fields` which comes from the expansion of the macro `ui_typed_field_catalog` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: unused variable: `candidate`
   --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../📋️copy/🦀️component.rs:53:32
    |
 53 |             fn copy_one(&self, candidate: &mut Self, path: &mut [usize], byte_candidate: &mut Vec<u8>, allocation: usize, work: usi...
    |                                ^^^^^^^^^
...
237 | ui_typed_field_catalog!(typed_fields);
    | ------------------------------------- in this macro invocation
    |
help: `candidate` is captured in macro and introduced a unused variable
   --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../📋️copy/🦀️component.rs:53:32
    |
 53 |             fn copy_one(&self, candidate: &mut Self, path: &mut [usize], byte_candidate: &mut Vec<u8>, allocation: usize, work: usi...
    |                                ^^^^^^^^^
...
237 | ui_typed_field_catalog!(typed_fields);
    | ------------------------------------- in this macro invocation
    = note: this warning originates in the macro `typed_fields` which comes from the expansion of the macro `ui_typed_field_catalog` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: unused variable: `byte_candidate`
   --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../📋️copy/🦀️component.rs:53:74
    |
 53 |             fn copy_one(&self, candidate: &mut Self, path: &mut [usize], byte_candidate: &mut Vec<u8>, allocation: usize, work: usi...
    |                                                                          ^^^^^^^^^^^^^^
...
237 | ui_typed_field_catalog!(typed_fields);
    | ------------------------------------- in this macro invocation
    |
help: `byte_candidate` is captured in macro and introduced a unused variable
   --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../📋️copy/🦀️component.rs:53:74
    |
 53 |             fn copy_one(&self, candidate: &mut Self, path: &mut [usize], byte_candidate: &mut Vec<u8>, allocation: usize, work: usi...
    |                                                                          ^^^^^^^^^^^^^^
...
237 | ui_typed_field_catalog!(typed_fields);
    | ------------------------------------- in this macro invocation
    = note: this warning originates in the macro `typed_fields` which comes from the expansion of the macro `ui_typed_field_catalog` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: unused variable: `allocation`
   --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../📋️copy/🦀️component.rs:53:104
    |
 53 |             fn copy_one(&self, candidate: &mut Self, path: &mut [usize], byte_candidate: &mut Vec<u8>, allocation: usize, work: usi...
    |                                                                                                        ^^^^^^^^^^
...
237 | ui_typed_field_catalog!(typed_fields);
    | ------------------------------------- in this macro invocation
    |
help: `allocation` is captured in macro and introduced a unused variable
   --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../📋️copy/🦀️component.rs:53:104
    |
 53 |             fn copy_one(&self, candidate: &mut Self, path: &mut [usize], byte_candidate: &mut Vec<u8>, allocation: usize, work: usi...
    |                                                                                                        ^^^^^^^^^^
...
237 | ui_typed_field_catalog!(typed_fields);
    | ------------------------------------- in this macro invocation
    = note: this warning originates in the macro `typed_fields` which comes from the expansion of the macro `ui_typed_field_catalog` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: unused variable: `work`
   --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../📋️copy/🦀️component.rs:53:123
    |
 53 |             fn copy_one(&self, candidate: &mut Self, path: &mut [usize], byte_candidate: &mut Vec<u8>, allocation: usize, work: usi...
    |                                                                                                                           ^^^^
...
237 | ui_typed_field_catalog!(typed_fields);
    | ------------------------------------- in this macro invocation
    |
help: `work` is captured in macro and introduced a unused variable
   --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../📋️copy/🦀️component.rs:53:123
    |
 53 |             fn copy_one(&self, candidate: &mut Self, path: &mut [usize], byte_candidate: &mut Vec<u8>, allocation: usize, work: usi...
    |                                                                                                                           ^^^^
...
237 | ui_typed_field_catalog!(typed_fields);
    | ------------------------------------- in this macro invocation
    = note: this warning originates in the macro `typed_fields` which comes from the expansion of the macro `ui_typed_field_catalog` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: unused variable: `path`
   --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../📋️copy/🦀️component.rs:55:29
    |
 55 |                 let (index, path) = split(path)?;
    |                             ^^^^
...
237 | ui_typed_field_catalog!(typed_fields);
    | ------------------------------------- in this macro invocation
    |
help: `path` is captured in macro and introduced a unused variable
   --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../📋️copy/🦀️component.rs:55:29
    |
 55 |                 let (index, path) = split(path)?;
    |                             ^^^^
...
237 | ui_typed_field_catalog!(typed_fields);
    | ------------------------------------- in this macro invocation
    = note: this warning originates in the macro `typed_fields` which comes from the expansion of the macro `ui_typed_field_catalog` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: unused variable: `count`
   --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../📋️copy/🦀️component.rs:56:21
    |
 56 |                 let count = 0 $(+ { let _ = stringify!($field); 1 })*;
    |                     ^^^^^
...
237 | ui_typed_field_catalog!(typed_fields);
    | ------------------------------------- in this macro invocation
    |
help: `count` is captured in macro and introduced a unused variable
   --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../📋️copy/🦀️component.rs:56:21
    |
 56 |                 let count = 0 $(+ { let _ = stringify!($field); 1 })*;
    |                     ^^^^^
...
237 | ui_typed_field_catalog!(typed_fields);
    | ------------------------------------- in this macro invocation
    = note: this warning originates in the macro `typed_fields` which comes from the expansion of the macro `ui_typed_field_catalog` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: unused variable: `value`
   --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../♻️retirement/🌳️typed/🦀️component.rs:62:57
    |
 62 |             fn retire_typed(&mut self, path: &mut [u8], value: &mut Option<UiValueRetirement>, bytes: usize) -> Result<UiValueRetir...
    |                                                         ^^^^^
...
186 | ui_typed_field_catalog!(typed_fields);
    | ------------------------------------- in this macro invocation
    |
help: `value` is captured in macro and introduced a unused variable
   --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../♻️retirement/🌳️typed/🦀️component.rs:62:57
    |
 62 |             fn retire_typed(&mut self, path: &mut [u8], value: &mut Option<UiValueRetirement>, bytes: usize) -> Result<UiValueRetir...
    |                                                         ^^^^^
...
186 | ui_typed_field_catalog!(typed_fields);
    | ------------------------------------- in this macro invocation
    = note: this warning originates in the macro `typed_fields` which comes from the expansion of the macro `ui_typed_field_catalog` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: unused variable: `bytes`
   --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../♻️retirement/🌳️typed/🦀️component.rs:62:96
    |
 62 |             fn retire_typed(&mut self, path: &mut [u8], value: &mut Option<UiValueRetirement>, bytes: usize) -> Result<UiValueRetir...
    |                                                                                                ^^^^^
...
186 | ui_typed_field_catalog!(typed_fields);
    | ------------------------------------- in this macro invocation
    |
help: `bytes` is captured in macro and introduced a unused variable
   --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../♻️retirement/🌳️typed/🦀️component.rs:62:96
    |
 62 |             fn retire_typed(&mut self, path: &mut [u8], value: &mut Option<UiValueRetirement>, bytes: usize) -> Result<UiValueRetir...
    |                                                                                                ^^^^^
...
186 | ui_typed_field_catalog!(typed_fields);
    | ------------------------------------- in this macro invocation
    = note: this warning originates in the macro `typed_fields` which comes from the expansion of the macro `ui_typed_field_catalog` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: unused variable: `path`
   --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../♻️retirement/🌳️typed/🦀️component.rs:64:29
    |
 64 |                 let (index, path) = split(path)?;
    |                             ^^^^
...
186 | ui_typed_field_catalog!(typed_fields);
    | ------------------------------------- in this macro invocation
    |
help: `path` is captured in macro and introduced a unused variable
   --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../♻️retirement/🌳️typed/🦀️component.rs:64:29
    |
 64 |                 let (index, path) = split(path)?;
    |                             ^^^^
...
186 | ui_typed_field_catalog!(typed_fields);
    | ------------------------------------- in this macro invocation
    = note: this warning originates in the macro `typed_fields` which comes from the expansion of the macro `ui_typed_field_catalog` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: unused variable: `count`
   --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../♻️retirement/🌳️typed/🦀️component.rs:65:21
    |
 65 |                 let count = 0 $(+ { let _ = stringify!($field); 1 })*;
    |                     ^^^^^
...
186 | ui_typed_field_catalog!(typed_fields);
    | ------------------------------------- in this macro invocation
    |
help: `count` is captured in macro and introduced a unused variable
   --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../♻️retirement/🌳️typed/🦀️component.rs:65:21
    |
 65 |                 let count = 0 $(+ { let _ = stringify!($field); 1 })*;
    |                     ^^^^^
...
186 | ui_typed_field_catalog!(typed_fields);
    | ------------------------------------- in this macro invocation
    = note: this warning originates in the macro `typed_fields` which comes from the expansion of the macro `ui_typed_field_catalog` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: unused variable: `right`
   --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../⚖️compare/🦀️component.rs:60:35
    |
 60 |             fn compare_one(&self, right: &Self, path: &mut [usize], values: &mut ValueComparison, bytes: usize) -> Result<UiCompone...
    |                                   ^^^^^
...
128 | ui_typed_field_catalog!(typed_fields);
    | ------------------------------------- in this macro invocation
    |
help: `right` is captured in macro and introduced a unused variable
   --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../⚖️compare/🦀️component.rs:60:35
    |
 60 |             fn compare_one(&self, right: &Self, path: &mut [usize], values: &mut ValueComparison, bytes: usize) -> Result<UiCompone...
    |                                   ^^^^^
...
128 | ui_typed_field_catalog!(typed_fields);
    | ------------------------------------- in this macro invocation
    = note: this warning originates in the macro `typed_fields` which comes from the expansion of the macro `ui_typed_field_catalog` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: unused variable: `values`
   --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../⚖️compare/🦀️component.rs:60:69
    |
 60 |             fn compare_one(&self, right: &Self, path: &mut [usize], values: &mut ValueComparison, bytes: usize) -> Result<UiCompone...
    |                                                                     ^^^^^^
...
128 | ui_typed_field_catalog!(typed_fields);
    | ------------------------------------- in this macro invocation
    |
help: `values` is captured in macro and introduced a unused variable
   --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../⚖️compare/🦀️component.rs:60:69
    |
 60 |             fn compare_one(&self, right: &Self, path: &mut [usize], values: &mut ValueComparison, bytes: usize) -> Result<UiCompone...
    |                                                                     ^^^^^^
...
128 | ui_typed_field_catalog!(typed_fields);
    | ------------------------------------- in this macro invocation
    = note: this warning originates in the macro `typed_fields` which comes from the expansion of the macro `ui_typed_field_catalog` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: unused variable: `bytes`
   --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../⚖️compare/🦀️component.rs:60:99
    |
 60 |             fn compare_one(&self, right: &Self, path: &mut [usize], values: &mut ValueComparison, bytes: usize) -> Result<UiCompone...
    |                                                                                                   ^^^^^
...
128 | ui_typed_field_catalog!(typed_fields);
    | ------------------------------------- in this macro invocation
    |
help: `bytes` is captured in macro and introduced a unused variable
   --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../⚖️compare/🦀️component.rs:60:99
    |
 60 |             fn compare_one(&self, right: &Self, path: &mut [usize], values: &mut ValueComparison, bytes: usize) -> Result<UiCompone...
    |                                                                                                   ^^^^^
...
128 | ui_typed_field_catalog!(typed_fields);
    | ------------------------------------- in this macro invocation
    = note: this warning originates in the macro `typed_fields` which comes from the expansion of the macro `ui_typed_field_catalog` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: unused variable: `path`
   --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../⚖️compare/🦀️component.rs:62:29
    |
 62 |                 let (index, path) = split(path)?;
    |                             ^^^^
...
128 | ui_typed_field_catalog!(typed_fields);
    | ------------------------------------- in this macro invocation
    |
help: `path` is captured in macro and introduced a unused variable
   --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../⚖️compare/🦀️component.rs:62:29
    |
 62 |                 let (index, path) = split(path)?;
    |                             ^^^^
...
128 | ui_typed_field_catalog!(typed_fields);
    | ------------------------------------- in this macro invocation
    = note: this warning originates in the macro `typed_fields` which comes from the expansion of the macro `ui_typed_field_catalog` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: unused variable: `count`
   --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../⚖️compare/🦀️component.rs:63:21
    |
 63 |                 let count = 0 $(+ { let _ = stringify!($field); 1 })*;
    |                     ^^^^^
...
128 | ui_typed_field_catalog!(typed_fields);
    | ------------------------------------- in this macro invocation
    |
help: `count` is captured in macro and introduced a unused variable
   --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../⚖️compare/🦀️component.rs:63:21
    |
 63 |                 let count = 0 $(+ { let _ = stringify!($field); 1 })*;
    |                     ^^^^^
...
128 | ui_typed_field_catalog!(typed_fields);
    | ------------------------------------- in this macro invocation
    = note: this warning originates in the macro `typed_fields` which comes from the expansion of the macro `ui_typed_field_catalog` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: `semio-framework-ui-contract` (lib) generated 47 warnings (22 duplicates) (run `cargo fix --lib -p semio-framework-ui-contract` to apply 6 suggestions)
    Checking semio-framework-ui-runtime v0.1.0 (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust)
warning: unnecessary qualification
   --> 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:385:49
    |
385 | ...   let fixed = size_of::<Self>() + std::mem::size_of_val(self.ordinals.entries.entries.as_ref()) + std::mem::size_of_val(self.ke...
    |                                       ^^^^^^^^^^^^^^^^^^^^^
    |
    = note: requested on the command line with `-W unused-qualifications`
help: remove the unnecessary path segments
    |
385 -                 let fixed = size_of::<Self>() + std::mem::size_of_val(self.ordinals.entries.entries.as_ref()) + std::mem::size_of_val(self.key_index.entries.entries.as_ref()) + ui_contract::UiDocumentAssembly::required_open_bytes();
385 +                 let fixed = size_of::<Self>() + size_of_val(self.ordinals.entries.entries.as_ref()) + std::mem::size_of_val(self.key_index.entries.entries.as_ref()) + ui_contract::UiDocumentAssembly::required_open_bytes();
    |

warning: unnecessary qualification
   --> 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:385:113
    |
385 | ...   let fixed = size_of::<Self>() + std::mem::size_of_val(self.ordinals.entries.entries.as_ref()) + std::mem::size_of_val(self.ke...
    |                                                                                                       ^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
385 -                 let fixed = size_of::<Self>() + std::mem::size_of_val(self.ordinals.entries.entries.as_ref()) + std::mem::size_of_val(self.key_index.entries.entries.as_ref()) + ui_contract::UiDocumentAssembly::required_open_bytes();
385 +                 let fixed = size_of::<Self>() + std::mem::size_of_val(self.ordinals.entries.entries.as_ref()) + size_of_val(self.key_index.entries.entries.as_ref()) + ui_contract::UiDocumentAssembly::required_open_bytes();
    |

warning: unnecessary qualification
    --> 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:2053:95
     |
2053 | ...   diff.owned_copy = Some(RecordOwnedCopy::Bindings(ui_contract::UiBindingsCopy::new(std::mem::take(&mut diff.record.bindings))));
     |                                                                                         ^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
2053 -             diff.owned_copy = Some(RecordOwnedCopy::Bindings(ui_contract::UiBindingsCopy::new(std::mem::take(&mut diff.record.bindings))));
2053 +             diff.owned_copy = Some(RecordOwnedCopy::Bindings(ui_contract::UiBindingsCopy::new(take(&mut diff.record.bindings))));
     |

warning: unnecessary qualification
    --> 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:3508:95
     |
3508 | ...   *owned_copy = Some(RecordOwnedCopy::Bindings(ui_contract::UiBindingsCopy::new(std::mem::take(&mut record.bindings))));
     |                                                                                     ^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
3508 -                 *owned_copy = Some(RecordOwnedCopy::Bindings(ui_contract::UiBindingsCopy::new(std::mem::take(&mut record.bindings))));
3508 +                 *owned_copy = Some(RecordOwnedCopy::Bindings(ui_contract::UiBindingsCopy::new(take(&mut record.bindings))));
     |

warning: constant `SURFACE_RECONCILE_FIXED_OPS` is never used
  --> 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:36:7
   |
36 | const SURFACE_RECONCILE_FIXED_OPS: usize = SURFACE_RECONCILE_FIXED_NODES * 9 + 1;
   |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: methods `values` and `clear` are never used
   --> 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:172:8
    |
142 | impl<K: Eq, V, const N: usize> SurfaceLinearMap<K, V, N> {
    | -------------------------------------------------------- methods in this implementation
...
172 |     fn values(&self) -> impl Iterator<Item = &V> {
    |        ^^^^^^
...
176 |     fn clear(&mut self) {
    |        ^^^^^

warning: methods `iter`, `remove`, and `is_empty` are never used
   --> 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:208:8
    |
200 | impl<T: Eq, const N: usize> SurfaceLinearSet<T, N> {
    | -------------------------------------------------- methods in this implementation
...
208 |     fn iter(&self) -> impl Iterator<Item = &T> {
    |        ^^^^
...
212 |     fn remove(&mut self, value: &T) -> bool {
    |        ^^^^^^
...
221 |     fn is_empty(&self) -> bool {
    |        ^^^^^^^^

warning: method `get_index` is never used
   --> 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:540:8
    |
539 | impl<K, V, const N: usize> SurfaceLinearMap<K, V, N> {
    | ---------------------------------------------------- method in this implementation
540 |     fn get_index(&self, index: usize) -> Option<(&K, &V)> {
    |        ^^^^^^^^^

warning: method `bindings` is never used
   --> 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:678:8
    |
677 | impl RecordOwnedCopy {
    | -------------------- method in this implementation
678 |     fn bindings(&self) -> Option<&ui_contract::UiBindingsCopy> { if let Self::Bindings(value) = self { Some(value) } else { None } }
    |        ^^^^^^^^

warning: associated functions `new` and `new_with_limits` are never used
    --> 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:1309:19
     |
1308 | impl SurfaceReconcileCursor {
     | --------------------------- associated functions in this implementation
1309 |     pub(crate) fn new(tree: crate::ComponentTree, current: &SurfaceReconciler) -> Self {
     |                   ^^^
...
1313 |     pub(crate) fn new_with_limits(tree: crate::ComponentTree, current: &SurfaceReconciler, limits: SurfaceReconcileLimits) -> Self {
     |                   ^^^^^^^^^^^^^^^

warning: function `split_surface_reconcile` is never used
    --> 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:2242:4
     |
2242 | fn split_surface_reconcile(mut credit: ui_contract::UiResidentPermit) -> Result<(ui_contract::UiResidentPermit, ui_contract::UiRes...
     |    ^^^^^^^^^^^^^^^^^^^^^^^

warning: function `shrink_surface_reconcile` is never used
    --> 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:2249:4
     |
2249 | fn shrink_surface_reconcile(mut credit: ui_contract::UiResidentPermit, usage: SurfaceReconcileUsage) -> Result<ui_contract::UiResi...
     |    ^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `estimate_record_bytes` is never used
    --> 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:3454:4
     |
3454 | fn estimate_record_bytes(record: &ui_contract::UiNodeRecord) -> usize {
     |    ^^^^^^^^^^^^^^^^^^^^^

warning: `semio-framework-ui-runtime` (lib) generated 13 warnings (run `cargo fix --lib -p semio-framework-ui-runtime` to apply 4 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 2.32s
    Checking semio-framework-ui-contract v0.1.0 (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust)
warning: unused import: `OnceLock`
  --> 🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust/../../🦀️component.rs:41:22
   |
41 | use std::sync::{Arc, OnceLock};
   |                      ^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: unused import: `std::time::Instant`
  --> 🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust/../../🦀️component.rs:43:5
   |
43 | use std::time::Instant;
   |     ^^^^^^^^^^^^^^^^^^

warning: `semio-framework-job` (lib) generated 2 warnings (run `cargo fix --lib -p semio-framework-job` to apply 2 suggestions)
warning: unnecessary qualification
  --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../♻️retirement/🌳️typed/🦀️component.rs:16:6
   |
16 | impl std::fmt::Debug for UiTypedRetirementCursor {
   |      ^^^^^^^^^^^^^^^
   |
   = note: requested on the command line with `-W unused-qualifications`
help: remove the unnecessary path segments
   |
16 - impl std::fmt::Debug for UiTypedRetirementCursor {
16 + impl fmt::Debug for UiTypedRetirementCursor {
   |

warning: unnecessary qualification
  --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../♻️retirement/🌳️typed/🦀️component.rs:17:35
   |
17 |     fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
   |                                   ^^^^^^^^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
17 -     fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
17 +     fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> std::fmt::Result {
   |

warning: unnecessary qualification
  --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../♻️retirement/🌳️typed/🦀️component.rs:17:63
   |
17 |     fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
   |                                                               ^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
17 -     fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
17 +     fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> fmt::Result {
   |

warning: unnecessary qualification
   --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🦀️document.rs:543:5
    |
543 |     std::mem::size_of::<Mutex<UiDocumentArena>>() + std::mem::size_of::<crate::UiArenaHandbacks<UI_DOCUMENT_LEASE_SLOTS, 1>>()
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
543 -     std::mem::size_of::<Mutex<UiDocumentArena>>() + std::mem::size_of::<crate::UiArenaHandbacks<UI_DOCUMENT_LEASE_SLOTS, 1>>()
543 +     size_of::<Mutex<UiDocumentArena>>() + std::mem::size_of::<crate::UiArenaHandbacks<UI_DOCUMENT_LEASE_SLOTS, 1>>()
    |

warning: unnecessary qualification
   --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🦀️document.rs:543:53
    |
543 |     std::mem::size_of::<Mutex<UiDocumentArena>>() + std::mem::size_of::<crate::UiArenaHandbacks<UI_DOCUMENT_LEASE_SLOTS, 1>>()
    |                                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
543 -     std::mem::size_of::<Mutex<UiDocumentArena>>() + std::mem::size_of::<crate::UiArenaHandbacks<UI_DOCUMENT_LEASE_SLOTS, 1>>()
543 +     std::mem::size_of::<Mutex<UiDocumentArena>>() + size_of::<crate::UiArenaHandbacks<UI_DOCUMENT_LEASE_SLOTS, 1>>()
    |

warning: unused variable: `byte_candidate`
  --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../📋️copy/🦀️component.rs:68:71
   |
68 |             fn copy_one(&self, candidate: &mut Self, _: &mut [usize], byte_candidate: &mut Vec<u8>, _: usize, work: usize) -> Result<UiComponentCopyProgress, UiFixedListAllocationError> {
   |                                                                       ^^^^^^^^^^^^^^
...
76 | scalar!(bool, u16, u64, f64, UiNodeId, UiRevision, Activity, TransitionHint, StyleSpec, Trigger, ContainerRole, InputKind, RowActionPlacement, SurfaceKind, Liveness, GridTrack, SpaceToken, Align, Justify, EdgeSpace, Axis, Anchor, ScrollAxes, Sizing);
   | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- in this macro invocation
   |
help: `byte_candidate` is captured in macro and introduced a unused variable
  --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../📋️copy/🦀️component.rs:68:71
   |
68 |             fn copy_one(&self, candidate: &mut Self, _: &mut [usize], byte_candidate: &mut Vec<u8>, _: usize, work: usize) -> Result<UiComponentCopyProgress, UiFixedListAllocationError> {
   |                                                                       ^^^^^^^^^^^^^^
...
76 | scalar!(bool, u16, u64, f64, UiNodeId, UiRevision, Activity, TransitionHint, StyleSpec, Trigger, ContainerRole, InputKind, RowActionPlacement, SurfaceKind, Liveness, GridTrack, SpaceToken, Align, Justify, EdgeSpace, Axis, Anchor, ScrollAxes, Sizing);
   | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- in this macro invocation
   = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default
   = note: this warning originates in the macro `scalar` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: unused variable: `byte_candidate`
  --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../📋️copy/🦀️component.rs:68:71
   |
68 |             fn copy_one(&self, candidate: &mut Self, _: &mut [usize], byte_candidate: &mut Vec<u8>, _: usize, work: usize) -> Result<UiComponentCopyProgress, UiFixedListAllocationError> {
   |                                                                       ^^^^^^^^^^^^^^
...
76 | scalar!(bool, u16, u64, f64, UiNodeId, UiRevision, Activity, TransitionHint, StyleSpec, Trigger, ContainerRole, InputKind, RowActionPlacement, SurfaceKind, Liveness, GridTrack, SpaceToken, Align, Justify, EdgeSpace, Axis, Anchor, ScrollAxes, Sizing);
   | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- in this macro invocation
   |
help: `byte_candidate` is captured in macro and introduced a unused variable
  --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../📋️copy/🦀️component.rs:68:71
   |
68 |             fn copy_one(&self, candidate: &mut Self, _: &mut [usize], byte_candidate: &mut Vec<u8>, _: usize, work: usize) -> Result<UiComponentCopyProgress, UiFixedListAllocationError> {
   |                                                                       ^^^^^^^^^^^^^^
...
76 | scalar!(bool, u16, u64, f64, UiNodeId, UiRevision, Activity, TransitionHint, StyleSpec, Trigger, ContainerRole, InputKind, RowActionPlacement, SurfaceKind, Liveness, GridTrack, SpaceToken, Align, Justify, EdgeSpace, Axis, Anchor, ScrollAxes, Sizing);
   | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- in this macro invocation
   = note: this warning originates in the macro `scalar` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: unused variable: `byte_candidate`
  --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../📋️copy/🦀️component.rs:81:63
   |
81 | ... _: &mut [usize], byte_candidate: &mut Vec<u8>, _: usize, work: usize) -> Result<UiComponentCopyProgress, UiFixedListAllocationEr...
   |                      ^^^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_byte_candidate`

warning: unused variable: `candidate`
   --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../📋️copy/🦀️component.rs:49:34
    |
 49 |             fn allocation(&self, candidate: &Self, path: &[usize]) -> Result<usize, &'static str> {
    |                                  ^^^^^^^^^
...
237 | ui_typed_field_catalog!(typed_fields);
    | ------------------------------------- in this macro invocation
    |
help: `candidate` is captured in macro and introduced a unused variable
   --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../📋️copy/🦀️component.rs:49:34
    |
 49 |             fn allocation(&self, candidate: &Self, path: &[usize]) -> Result<usize, &'static str> {
    |                                  ^^^^^^^^^
...
237 | ui_typed_field_catalog!(typed_fields);
    | ------------------------------------- in this macro invocation
    = note: this warning originates in the macro `typed_fields` which comes from the expansion of the macro `ui_typed_field_catalog` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: unused variable: `path`
   --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../📋️copy/🦀️component.rs:50:29
    |
 50 |                 let (index, path) = read_path(path)?;
    |                             ^^^^
...
237 | ui_typed_field_catalog!(typed_fields);
    | ------------------------------------- in this macro invocation
    |
help: `path` is captured in macro and introduced a unused variable
   --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../📋️copy/🦀️component.rs:50:29
    |
 50 |                 let (index, path) = read_path(path)?;
    |                             ^^^^
...
237 | ui_typed_field_catalog!(typed_fields);
    | ------------------------------------- in this macro invocation
    = note: this warning originates in the macro `typed_fields` which comes from the expansion of the macro `ui_typed_field_catalog` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: unused variable: `candidate`
   --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../📋️copy/🦀️component.rs:53:32
    |
 53 |             fn copy_one(&self, candidate: &mut Self, path: &mut [usize], byte_candidate: &mut Vec<u8>, allocation: usize, work: usi...
    |                                ^^^^^^^^^
...
237 | ui_typed_field_catalog!(typed_fields);
    | ------------------------------------- in this macro invocation
    |
help: `candidate` is captured in macro and introduced a unused variable
   --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../📋️copy/🦀️component.rs:53:32
    |
 53 |             fn copy_one(&self, candidate: &mut Self, path: &mut [usize], byte_candidate: &mut Vec<u8>, allocation: usize, work: usi...
    |                                ^^^^^^^^^
...
237 | ui_typed_field_catalog!(typed_fields);
    | ------------------------------------- in this macro invocation
    = note: this warning originates in the macro `typed_fields` which comes from the expansion of the macro `ui_typed_field_catalog` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: unused variable: `byte_candidate`
   --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../📋️copy/🦀️component.rs:53:74
    |
 53 |             fn copy_one(&self, candidate: &mut Self, path: &mut [usize], byte_candidate: &mut Vec<u8>, allocation: usize, work: usi...
    |                                                                          ^^^^^^^^^^^^^^
...
237 | ui_typed_field_catalog!(typed_fields);
    | ------------------------------------- in this macro invocation
    |
help: `byte_candidate` is captured in macro and introduced a unused variable
   --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../📋️copy/🦀️component.rs:53:74
    |
 53 |             fn copy_one(&self, candidate: &mut Self, path: &mut [usize], byte_candidate: &mut Vec<u8>, allocation: usize, work: usi...
    |                                                                          ^^^^^^^^^^^^^^
...
237 | ui_typed_field_catalog!(typed_fields);
    | ------------------------------------- in this macro invocation
    = note: this warning originates in the macro `typed_fields` which comes from the expansion of the macro `ui_typed_field_catalog` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: unused variable: `allocation`
   --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../📋️copy/🦀️component.rs:53:104
    |
 53 |             fn copy_one(&self, candidate: &mut Self, path: &mut [usize], byte_candidate: &mut Vec<u8>, allocation: usize, work: usi...
    |                                                                                                        ^^^^^^^^^^
...
237 | ui_typed_field_catalog!(typed_fields);
    | ------------------------------------- in this macro invocation
    |
help: `allocation` is captured in macro and introduced a unused variable
   --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../📋️copy/🦀️component.rs:53:104
    |
 53 |             fn copy_one(&self, candidate: &mut Self, path: &mut [usize], byte_candidate: &mut Vec<u8>, allocation: usize, work: usi...
    |                                                                                                        ^^^^^^^^^^
...
237 | ui_typed_field_catalog!(typed_fields);
    | ------------------------------------- in this macro invocation
    = note: this warning originates in the macro `typed_fields` which comes from the expansion of the macro `ui_typed_field_catalog` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: unused variable: `work`
   --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../📋️copy/🦀️component.rs:53:123
    |
 53 |             fn copy_one(&self, candidate: &mut Self, path: &mut [usize], byte_candidate: &mut Vec<u8>, allocation: usize, work: usi...
    |                                                                                                                           ^^^^
...
237 | ui_typed_field_catalog!(typed_fields);
    | ------------------------------------- in this macro invocation
    |
help: `work` is captured in macro and introduced a unused variable
   --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../📋️copy/🦀️component.rs:53:123
    |
 53 |             fn copy_one(&self, candidate: &mut Self, path: &mut [usize], byte_candidate: &mut Vec<u8>, allocation: usize, work: usi...
    |                                                                                                                           ^^^^
...
237 | ui_typed_field_catalog!(typed_fields);
    | ------------------------------------- in this macro invocation
    = note: this warning originates in the macro `typed_fields` which comes from the expansion of the macro `ui_typed_field_catalog` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: unused variable: `path`
   --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../📋️copy/🦀️component.rs:55:29
    |
 55 |                 let (index, path) = split(path)?;
    |                             ^^^^
...
237 | ui_typed_field_catalog!(typed_fields);
    | ------------------------------------- in this macro invocation
    |
help: `path` is captured in macro and introduced a unused variable
   --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../📋️copy/🦀️component.rs:55:29
    |
 55 |                 let (index, path) = split(path)?;
    |                             ^^^^
...
237 | ui_typed_field_catalog!(typed_fields);
    | ------------------------------------- in this macro invocation
    = note: this warning originates in the macro `typed_fields` which comes from the expansion of the macro `ui_typed_field_catalog` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: unused variable: `count`
   --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../📋️copy/🦀️component.rs:56:21
    |
 56 |                 let count = 0 $(+ { let _ = stringify!($field); 1 })*;
    |                     ^^^^^
...
237 | ui_typed_field_catalog!(typed_fields);
    | ------------------------------------- in this macro invocation
    |
help: `count` is captured in macro and introduced a unused variable
   --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../📋️copy/🦀️component.rs:56:21
    |
 56 |                 let count = 0 $(+ { let _ = stringify!($field); 1 })*;
    |                     ^^^^^
...
237 | ui_typed_field_catalog!(typed_fields);
    | ------------------------------------- in this macro invocation
    = note: this warning originates in the macro `typed_fields` which comes from the expansion of the macro `ui_typed_field_catalog` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: unused variable: `right`
   --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../⚖️compare/🦀️component.rs:60:35
    |
 60 |             fn compare_one(&self, right: &Self, path: &mut [usize], values: &mut ValueComparison, bytes: usize) -> Result<UiCompone...
    |                                   ^^^^^
...
128 | ui_typed_field_catalog!(typed_fields);
    | ------------------------------------- in this macro invocation
    |
help: `right` is captured in macro and introduced a unused variable
   --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../⚖️compare/🦀️component.rs:60:35
    |
 60 |             fn compare_one(&self, right: &Self, path: &mut [usize], values: &mut ValueComparison, bytes: usize) -> Result<UiCompone...
    |                                   ^^^^^
...
128 | ui_typed_field_catalog!(typed_fields);
    | ------------------------------------- in this macro invocation
    = note: this warning originates in the macro `typed_fields` which comes from the expansion of the macro `ui_typed_field_catalog` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: unused variable: `values`
   --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../⚖️compare/🦀️component.rs:60:69
    |
 60 |             fn compare_one(&self, right: &Self, path: &mut [usize], values: &mut ValueComparison, bytes: usize) -> Result<UiCompone...
    |                                                                     ^^^^^^
...
128 | ui_typed_field_catalog!(typed_fields);
    | ------------------------------------- in this macro invocation
    |
help: `values` is captured in macro and introduced a unused variable
   --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../⚖️compare/🦀️component.rs:60:69
    |
 60 |             fn compare_one(&self, right: &Self, path: &mut [usize], values: &mut ValueComparison, bytes: usize) -> Result<UiCompone...
    |                                                                     ^^^^^^
...
128 | ui_typed_field_catalog!(typed_fields);
    | ------------------------------------- in this macro invocation
    = note: this warning originates in the macro `typed_fields` which comes from the expansion of the macro `ui_typed_field_catalog` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: unused variable: `bytes`
   --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../⚖️compare/🦀️component.rs:60:99
    |
 60 |             fn compare_one(&self, right: &Self, path: &mut [usize], values: &mut ValueComparison, bytes: usize) -> Result<UiCompone...
    |                                                                                                   ^^^^^
...
128 | ui_typed_field_catalog!(typed_fields);
    | ------------------------------------- in this macro invocation
    |
help: `bytes` is captured in macro and introduced a unused variable
   --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../⚖️compare/🦀️component.rs:60:99
    |
 60 |             fn compare_one(&self, right: &Self, path: &mut [usize], values: &mut ValueComparison, bytes: usize) -> Result<UiCompone...
    |                                                                                                   ^^^^^
...
128 | ui_typed_field_catalog!(typed_fields);
    | ------------------------------------- in this macro invocation
    = note: this warning originates in the macro `typed_fields` which comes from the expansion of the macro `ui_typed_field_catalog` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: unused variable: `path`
   --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../⚖️compare/🦀️component.rs:62:29
    |
 62 |                 let (index, path) = split(path)?;
    |                             ^^^^
...
128 | ui_typed_field_catalog!(typed_fields);
    | ------------------------------------- in this macro invocation
    |
help: `path` is captured in macro and introduced a unused variable
   --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../⚖️compare/🦀️component.rs:62:29
    |
 62 |                 let (index, path) = split(path)?;
    |                             ^^^^
...
128 | ui_typed_field_catalog!(typed_fields);
    | ------------------------------------- in this macro invocation
    = note: this warning originates in the macro `typed_fields` which comes from the expansion of the macro `ui_typed_field_catalog` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: unused variable: `count`
   --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../⚖️compare/🦀️component.rs:63:21
    |
 63 |                 let count = 0 $(+ { let _ = stringify!($field); 1 })*;
    |                     ^^^^^
...
128 | ui_typed_field_catalog!(typed_fields);
    | ------------------------------------- in this macro invocation
    |
help: `count` is captured in macro and introduced a unused variable
   --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../⚖️compare/🦀️component.rs:63:21
    |
 63 |                 let count = 0 $(+ { let _ = stringify!($field); 1 })*;
    |                     ^^^^^
...
128 | ui_typed_field_catalog!(typed_fields);
    | ------------------------------------- in this macro invocation
    = note: this warning originates in the macro `typed_fields` which comes from the expansion of the macro `ui_typed_field_catalog` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: unused variable: `value`
   --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../♻️retirement/🌳️typed/🦀️component.rs:62:57
    |
 62 |             fn retire_typed(&mut self, path: &mut [u8], value: &mut Option<UiValueRetirement>, bytes: usize) -> Result<UiValueRetir...
    |                                                         ^^^^^
...
186 | ui_typed_field_catalog!(typed_fields);
    | ------------------------------------- in this macro invocation
    |
help: `value` is captured in macro and introduced a unused variable
   --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../♻️retirement/🌳️typed/🦀️component.rs:62:57
    |
 62 |             fn retire_typed(&mut self, path: &mut [u8], value: &mut Option<UiValueRetirement>, bytes: usize) -> Result<UiValueRetir...
    |                                                         ^^^^^
...
186 | ui_typed_field_catalog!(typed_fields);
    | ------------------------------------- in this macro invocation
    = note: this warning originates in the macro `typed_fields` which comes from the expansion of the macro `ui_typed_field_catalog` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: unused variable: `bytes`
   --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../♻️retirement/🌳️typed/🦀️component.rs:62:96
    |
 62 |             fn retire_typed(&mut self, path: &mut [u8], value: &mut Option<UiValueRetirement>, bytes: usize) -> Result<UiValueRetir...
    |                                                                                                ^^^^^
...
186 | ui_typed_field_catalog!(typed_fields);
    | ------------------------------------- in this macro invocation
    |
help: `bytes` is captured in macro and introduced a unused variable
   --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../♻️retirement/🌳️typed/🦀️component.rs:62:96
    |
 62 |             fn retire_typed(&mut self, path: &mut [u8], value: &mut Option<UiValueRetirement>, bytes: usize) -> Result<UiValueRetir...
    |                                                                                                ^^^^^
...
186 | ui_typed_field_catalog!(typed_fields);
    | ------------------------------------- in this macro invocation
    = note: this warning originates in the macro `typed_fields` which comes from the expansion of the macro `ui_typed_field_catalog` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: unused variable: `path`
   --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../♻️retirement/🌳️typed/🦀️component.rs:64:29
    |
 64 |                 let (index, path) = split(path)?;
    |                             ^^^^
...
186 | ui_typed_field_catalog!(typed_fields);
    | ------------------------------------- in this macro invocation
    |
help: `path` is captured in macro and introduced a unused variable
   --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../♻️retirement/🌳️typed/🦀️component.rs:64:29
    |
 64 |                 let (index, path) = split(path)?;
    |                             ^^^^
...
186 | ui_typed_field_catalog!(typed_fields);
    | ------------------------------------- in this macro invocation
    = note: this warning originates in the macro `typed_fields` which comes from the expansion of the macro `ui_typed_field_catalog` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: unused variable: `count`
   --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../♻️retirement/🌳️typed/🦀️component.rs:65:21
    |
 65 |                 let count = 0 $(+ { let _ = stringify!($field); 1 })*;
    |                     ^^^^^
...
186 | ui_typed_field_catalog!(typed_fields);
    | ------------------------------------- in this macro invocation
    |
help: `count` is captured in macro and introduced a unused variable
   --> 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../♻️retirement/🌳️typed/🦀️component.rs:65:21
    |
 65 |                 let count = 0 $(+ { let _ = stringify!($field); 1 })*;
    |                     ^^^^^
...
186 | ui_typed_field_catalog!(typed_fields);
    | ------------------------------------- in this macro invocation
    = note: this warning originates in the macro `typed_fields` which comes from the expansion of the macro `ui_typed_field_catalog` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: `semio-framework-ui-contract` (lib) generated 47 warnings (22 duplicates) (run `cargo fix --lib -p semio-framework-ui-contract` to apply 6 suggestions)
    Checking semio-framework-ui-runtime v0.1.0 (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust)
warning: unnecessary qualification
   --> 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:385:49
    |
385 | ...   let fixed = size_of::<Self>() + std::mem::size_of_val(self.ordinals.entries.entries.as_ref()) + std::mem::size_of_val(self.ke...
    |                                       ^^^^^^^^^^^^^^^^^^^^^
    |
    = note: requested on the command line with `-W unused-qualifications`
help: remove the unnecessary path segments
    |
385 -                 let fixed = size_of::<Self>() + std::mem::size_of_val(self.ordinals.entries.entries.as_ref()) + std::mem::size_of_val(self.key_index.entries.entries.as_ref()) + ui_contract::UiDocumentAssembly::required_open_bytes();
385 +                 let fixed = size_of::<Self>() + size_of_val(self.ordinals.entries.entries.as_ref()) + std::mem::size_of_val(self.key_index.entries.entries.as_ref()) + ui_contract::UiDocumentAssembly::required_open_bytes();
    |

warning: unnecessary qualification
   --> 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:385:113
    |
385 | ...   let fixed = size_of::<Self>() + std::mem::size_of_val(self.ordinals.entries.entries.as_ref()) + std::mem::size_of_val(self.ke...
    |                                                                                                       ^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
385 -                 let fixed = size_of::<Self>() + std::mem::size_of_val(self.ordinals.entries.entries.as_ref()) + std::mem::size_of_val(self.key_index.entries.entries.as_ref()) + ui_contract::UiDocumentAssembly::required_open_bytes();
385 +                 let fixed = size_of::<Self>() + std::mem::size_of_val(self.ordinals.entries.entries.as_ref()) + size_of_val(self.key_index.entries.entries.as_ref()) + ui_contract::UiDocumentAssembly::required_open_bytes();
    |

warning: unnecessary qualification
    --> 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:2053:95
     |
2053 | ...   diff.owned_copy = Some(RecordOwnedCopy::Bindings(ui_contract::UiBindingsCopy::new(std::mem::take(&mut diff.record.bindings))));
     |                                                                                         ^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
2053 -             diff.owned_copy = Some(RecordOwnedCopy::Bindings(ui_contract::UiBindingsCopy::new(std::mem::take(&mut diff.record.bindings))));
2053 +             diff.owned_copy = Some(RecordOwnedCopy::Bindings(ui_contract::UiBindingsCopy::new(take(&mut diff.record.bindings))));
     |

warning: unnecessary qualification
    --> 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:3508:95
     |
3508 | ...   *owned_copy = Some(RecordOwnedCopy::Bindings(ui_contract::UiBindingsCopy::new(std::mem::take(&mut record.bindings))));
     |                                                                                     ^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
3508 -                 *owned_copy = Some(RecordOwnedCopy::Bindings(ui_contract::UiBindingsCopy::new(std::mem::take(&mut record.bindings))));
3508 +                 *owned_copy = Some(RecordOwnedCopy::Bindings(ui_contract::UiBindingsCopy::new(take(&mut record.bindings))));
     |

warning: constant `SURFACE_RECONCILE_FIXED_OPS` is never used
  --> 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:36:7
   |
36 | const SURFACE_RECONCILE_FIXED_OPS: usize = SURFACE_RECONCILE_FIXED_NODES * 9 + 1;
   |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: methods `values` and `clear` are never used
   --> 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:172:8
    |
142 | impl<K: Eq, V, const N: usize> SurfaceLinearMap<K, V, N> {
    | -------------------------------------------------------- methods in this implementation
...
172 |     fn values(&self) -> impl Iterator<Item = &V> {
    |        ^^^^^^
...
176 |     fn clear(&mut self) {
    |        ^^^^^

warning: methods `iter`, `remove`, and `is_empty` are never used
   --> 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:208:8
    |
200 | impl<T: Eq, const N: usize> SurfaceLinearSet<T, N> {
    | -------------------------------------------------- methods in this implementation
...
208 |     fn iter(&self) -> impl Iterator<Item = &T> {
    |        ^^^^
...
212 |     fn remove(&mut self, value: &T) -> bool {
    |        ^^^^^^
...
221 |     fn is_empty(&self) -> bool {
    |        ^^^^^^^^

warning: method `get_index` is never used
   --> 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:540:8
    |
539 | impl<K, V, const N: usize> SurfaceLinearMap<K, V, N> {
    | ---------------------------------------------------- method in this implementation
540 |     fn get_index(&self, index: usize) -> Option<(&K, &V)> {
    |        ^^^^^^^^^

warning: method `bindings` is never used
   --> 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:678:8
    |
677 | impl RecordOwnedCopy {
    | -------------------- method in this implementation
678 |     fn bindings(&self) -> Option<&ui_contract::UiBindingsCopy> { if let Self::Bindings(value) = self { Some(value) } else { None } }
    |        ^^^^^^^^

warning: associated functions `new` and `new_with_limits` are never used
    --> 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:1309:19
     |
1308 | impl SurfaceReconcileCursor {
     | --------------------------- associated functions in this implementation
1309 |     pub(crate) fn new(tree: crate::ComponentTree, current: &SurfaceReconciler) -> Self {
     |                   ^^^
...
1313 |     pub(crate) fn new_with_limits(tree: crate::ComponentTree, current: &SurfaceReconciler, limits: SurfaceReconcileLimits) -> Self {
     |                   ^^^^^^^^^^^^^^^

warning: function `split_surface_reconcile` is never used
    --> 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:2242:4
     |
2242 | fn split_surface_reconcile(mut credit: ui_contract::UiResidentPermit) -> Result<(ui_contract::UiResidentPermit, ui_contract::UiRes...
     |    ^^^^^^^^^^^^^^^^^^^^^^^

warning: function `shrink_surface_reconcile` is never used
    --> 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:2249:4
     |
2249 | fn shrink_surface_reconcile(mut credit: ui_contract::UiResidentPermit, usage: SurfaceReconcileUsage) -> Result<ui_contract::UiResi...
     |    ^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `estimate_record_bytes` is never used
    --> 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:3454:4
     |
3454 | fn estimate_record_bytes(record: &ui_contract::UiNodeRecord) -> usize {
     |    ^^^^^^^^^^^^^^^^^^^^^

warning: `semio-framework-ui-runtime` (lib) generated 13 warnings (run `cargo fix --lib -p semio-framework-ui-runtime` to apply 4 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 2.13s



 NX   Successfully ran target check-wasm for project @semio-tech/ui-runtime-rs



```

