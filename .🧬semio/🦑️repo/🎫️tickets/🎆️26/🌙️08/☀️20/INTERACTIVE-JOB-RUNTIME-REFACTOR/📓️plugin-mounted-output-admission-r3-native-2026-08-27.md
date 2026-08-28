# Mounted Output Candidate R3

Actual native compile failed with exactly two E0308 diagnostics: DummyFixtureJob command at dummy fixture209 and TxnFixtureJob command at transaction fixture218 require inline commands, while the current request owns Box<Command>. No tests executed; no PatchTracker compile or behavioral PASS inferred. Cargo101/Nx1. The owning Mutation lane is notified; no command unboxing or fixture edit was made by this lane.

This capture includes the current live pool/key candidate and Dag's newly staged dropped-Future fixture assertions. It is not the unchanged R8 binary. Source hold released on process exit. No atomic full-input snapshot was taken for this diagnostic, so no stronger reproducibility claim is made.

## Exact Command

```sh
set -o pipefail
SEMIO_COVERAGE=0 NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad' SEMIO_TEST_ARTIFACT_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts' SEMIO_BUILD_BUDGET_MS=3600000 RUST_BACKTRACE=1 bun x nx run @semio-tech/framework-plugin:test --skip-nx-cache --args='mounted_output_admission_ -- --nocapture' 2>&1 | tee '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️member-plugin-mounted-output-admission-r3-2026-08-27.txt'
```

## Full Captured Output

```text

> nx run @semio-tech/framework-plugin:test --args=mounted_output_admission_ -- --nocapture

> bun 📜️script.ts test mounted_output_admission_ -- --nocapture

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
   --> 🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/./../../📡️wire/🦀️component.rs:149:51
    |
149 |         self.backing.as_ref().map_or(0, |backing| std::mem::size_of_val(backing.as_ref()))
    |                                                   ^^^^^^^^^^^^^^^^^^^^^
    |
    = note: requested on the command line with `-W unused-qualifications`
help: remove the unnecessary path segments
    |
149 -         self.backing.as_ref().map_or(0, |backing| std::mem::size_of_val(backing.as_ref()))
149 +         self.backing.as_ref().map_or(0, |backing| size_of_val(backing.as_ref()))
    |

warning: method `push` is never used
   --> 🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/../../🔗️causal/🦀️component.rs:142:8
    |
109 | impl<T> MutationDagFixedSlots<T> {
    | -------------------------------- method in this implementation
...
142 |     fn push(&mut self, value: T) -> Result<(), T> {
    |        ^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: `semio-framework-replication` (lib) generated 2 warnings (run `cargo fix --lib -p semio-framework-replication` to apply 1 suggestion)
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
warning: fields `header_len` and `stored_len` are never read
   --> 🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust/../../📐️format/🦀️component.rs:178:5
    |
176 | struct EncodedSegment {
    |        -------------- fields in this struct
177 |     bytes: Vec<u8>,
178 |     header_len: usize,
    |     ^^^^^^^^^^
179 |     stored_len: usize,
    |     ^^^^^^^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: `semio-framework-pack` (lib) generated 1 warning
warning: field `source_path` is never read
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/📦️packages/🦀️rust/📦️glue.rs:48:5
   |
45 | struct MutationAggregateSourceAuthority {
   |        -------------------------------- field in this struct
...
48 |     source_path: PathBuf,
   |     ^^^^^^^^^^^
   |
   = note: `MutationAggregateSourceAuthority` has a derived impl for the trait `Debug`, but this is intentionally ignored during dead code analysis
   = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: `semio-framework-os-kernel-dsl-derive` (lib) generated 1 warning
warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:1765:13
     |
1765 |     marker: std::marker::PhantomData<fn() -> (P, Mutation)>,
     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
     = note: requested on the command line with `-W unused-qualifications`
help: remove the unnecessary path segments
     |
1765 -     marker: std::marker::PhantomData<fn() -> (P, Mutation)>,
1765 +     marker: PhantomData<fn() -> (P, Mutation)>,
     |

warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:1770:119
     |
1770 | ...active: std::mem::ManuallyDrop::new(None), marker: std::marker::PhantomData }
     |                                                       ^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
1770 -         Self { phase: ArtifactStoreCursorDisposerPhase::Displaced, active: std::mem::ManuallyDrop::new(None), marker: std::marker::PhantomData }
1770 +         Self { phase: ArtifactStoreCursorDisposerPhase::Displaced, active: std::mem::ManuallyDrop::new(None), marker: PhantomData }
     |

warning: unnecessary qualification
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/👥️presence/♻️retirement/🦀️component.rs:167:43
    |
167 | impl<P: Clone + Send + Sync + 'static, M: self::Mutation<P>> PresenceStore<P, M> {
    |                                           ^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
167 - impl<P: Clone + Send + Sync + 'static, M: self::Mutation<P>> PresenceStore<P, M> {
167 + impl<P: Clone + Send + Sync + 'static, M: Mutation<P>> PresenceStore<P, M> {
    |

warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:5905:49
     |
5905 | struct ArtifactRepositoryHistoryEntryDecoder<T>(std::marker::PhantomData<T>);
     |                                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
5905 - struct ArtifactRepositoryHistoryEntryDecoder<T>(std::marker::PhantomData<T>);
5905 + struct ArtifactRepositoryHistoryEntryDecoder<T>(PhantomData<T>);
     |

warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:5909:14
     |
5909 |         Self(std::marker::PhantomData)
     |              ^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
5909 -         Self(std::marker::PhantomData)
5909 +         Self(PhantomData)
     |

warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:5994:14
     |
5994 |     catalog: std::sync::Arc<dyn ArtifactEnvelopeOwnedFieldCatalog<P, Mutation>>,
     |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
5994 -     catalog: std::sync::Arc<dyn ArtifactEnvelopeOwnedFieldCatalog<P, Mutation>>,
5994 +     catalog: Arc<dyn ArtifactEnvelopeOwnedFieldCatalog<P, Mutation>>,
     |

warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:5995:23
     |
5995 |     mutation_factory: std::sync::Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>,
     |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
5995 -     mutation_factory: std::sync::Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>,
5995 +     mutation_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>,
     |

warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:6013:18
     |
6013 |         catalog: std::sync::Arc<dyn ArtifactEnvelopeOwnedFieldCatalog<P, Mutation>>,
     |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
6013 -         catalog: std::sync::Arc<dyn ArtifactEnvelopeOwnedFieldCatalog<P, Mutation>>,
6013 +         catalog: Arc<dyn ArtifactEnvelopeOwnedFieldCatalog<P, Mutation>>,
     |

warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:6014:27
     |
6014 |         mutation_factory: std::sync::Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>,
     |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
6014 -         mutation_factory: std::sync::Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>,
6014 +         mutation_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>,
     |

warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:6208:14
     |
6208 |     catalog: std::sync::Arc<dyn ArtifactEnvelopeOwnedFieldCatalog<P, Mutation>>,
     |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
6208 -     catalog: std::sync::Arc<dyn ArtifactEnvelopeOwnedFieldCatalog<P, Mutation>>,
6208 +     catalog: Arc<dyn ArtifactEnvelopeOwnedFieldCatalog<P, Mutation>>,
     |

warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:6209:23
     |
6209 |     mutation_factory: std::sync::Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>,
     |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
6209 -     mutation_factory: std::sync::Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>,
6209 +     mutation_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>,
     |

warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:6210:25
     |
6210 |     retirement_factory: std::sync::Arc<dyn ArtifactOwnedValueRetirementFactory<Edit<Mutation>>>,
     |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
6210 -     retirement_factory: std::sync::Arc<dyn ArtifactOwnedValueRetirementFactory<Edit<Mutation>>>,
6210 +     retirement_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<Edit<Mutation>>>,
     |

warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:6228:18
     |
6228 |         catalog: std::sync::Arc<dyn ArtifactEnvelopeOwnedFieldCatalog<P, Mutation>>,
     |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
6228 -         catalog: std::sync::Arc<dyn ArtifactEnvelopeOwnedFieldCatalog<P, Mutation>>,
6228 +         catalog: Arc<dyn ArtifactEnvelopeOwnedFieldCatalog<P, Mutation>>,
     |

warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:6229:27
     |
6229 |         mutation_factory: std::sync::Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>,
     |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
6229 -         mutation_factory: std::sync::Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>,
6229 +         mutation_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>,
     |

warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:6230:29
     |
6230 |         retirement_factory: std::sync::Arc<dyn ArtifactOwnedValueRetirementFactory<Edit<Mutation>>>,
     |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
6230 -         retirement_factory: std::sync::Arc<dyn ArtifactOwnedValueRetirementFactory<Edit<Mutation>>>,
6230 +         retirement_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<Edit<Mutation>>>,
     |

warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:6448:14
     |
6448 |     catalog: std::sync::Arc<dyn ArtifactEnvelopeOwnedFieldCatalog<P, Mutation>>,
     |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
6448 -     catalog: std::sync::Arc<dyn ArtifactEnvelopeOwnedFieldCatalog<P, Mutation>>,
6448 +     catalog: Arc<dyn ArtifactEnvelopeOwnedFieldCatalog<P, Mutation>>,
     |

warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:6449:23
     |
6449 |     mutation_factory: std::sync::Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>,
     |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
6449 -     mutation_factory: std::sync::Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>,
6449 +     mutation_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>,
     |

warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:6458:29
     |
6458 |         retirement_factory: std::sync::Arc<dyn ArtifactOwnedValueRetirementFactory<Edit<Mutation>>>,
     |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
6458 -         retirement_factory: std::sync::Arc<dyn ArtifactOwnedValueRetirementFactory<Edit<Mutation>>>,
6458 +         retirement_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<Edit<Mutation>>>,
     |

warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:6674:12
     |
6674 |     state: std::sync::Mutex<ArtifactEnvelopeFieldDecoderRegistryState<P, Mutation>>,
     |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
6674 -     state: std::sync::Mutex<ArtifactEnvelopeFieldDecoderRegistryState<P, Mutation>>,
6674 +     state: Mutex<ArtifactEnvelopeFieldDecoderRegistryState<P, Mutation>>,
     |

warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:6692:20
     |
6692 | ...   state: std::sync::Mutex::new(ArtifactEnvelopeFieldDecoderRegistryState { slots, free, free_len: ARTIFACT_ENVELOPE_FIELD_DECO...
     |              ^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
6692 -             state: std::sync::Mutex::new(ArtifactEnvelopeFieldDecoderRegistryState { slots, free, free_len: ARTIFACT_ENVELOPE_FIELD_DECODER_CAPACITY }),
6692 +             state: Mutex::new(ArtifactEnvelopeFieldDecoderRegistryState { slots, free, free_len: ARTIFACT_ENVELOPE_FIELD_DECODER_CAPACITY }),
     |

warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:8064:12
     |
8064 |     state: std::sync::Mutex<ArtifactEnvelopeCompletedRecordRegistryState<P, Mutation>>,
     |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
8064 -     state: std::sync::Mutex<ArtifactEnvelopeCompletedRecordRegistryState<P, Mutation>>,
8064 +     state: Mutex<ArtifactEnvelopeCompletedRecordRegistryState<P, Mutation>>,
     |

warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:8074:32
     |
8074 | ...   Arc::new(Self { state: std::sync::Mutex::new(ArtifactEnvelopeCompletedRecordRegistryState { slots, free, free_len: ARTIFACT_...
     |                              ^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
8074 -         Arc::new(Self { state: std::sync::Mutex::new(ArtifactEnvelopeCompletedRecordRegistryState { slots, free, free_len: ARTIFACT_ENVELOPE_COMPLETED_RECORD_CAPACITY, live: 0, occupied: 0, closing: 0 }) })
8074 +         Arc::new(Self { state: Mutex::new(ArtifactEnvelopeCompletedRecordRegistryState { slots, free, free_len: ARTIFACT_ENVELOPE_COMPLETED_RECORD_CAPACITY, live: 0, occupied: 0, closing: 0 }) })
     |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:12001:26
      |
12001 |             .checked_mul(std::mem::size_of::<String>())?
      |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
12001 -             .checked_mul(std::mem::size_of::<String>())?
12001 +             .checked_mul(size_of::<String>())?
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:12002:68
      |
12002 |             .checked_add(self.redo_edit_ids.capacity().checked_mul(std::mem::size_of::<String>())?)?
      |                                                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
12002 -             .checked_add(self.redo_edit_ids.capacity().checked_mul(std::mem::size_of::<String>())?)?
12002 +             .checked_add(self.redo_edit_ids.capacity().checked_mul(size_of::<String>())?)?
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:12003:78
      |
12003 |             .checked_add(self.cursor_applied_edit_ids.capacity().checked_mul(std::mem::size_of::<String>())?)?
      |                                                                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
12003 -             .checked_add(self.cursor_applied_edit_ids.capacity().checked_mul(std::mem::size_of::<String>())?)?
12003 +             .checked_add(self.cursor_applied_edit_ids.capacity().checked_mul(size_of::<String>())?)?
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:12004:75
      |
12004 |             .checked_add(self.cursor_redo_edit_ids.capacity().checked_mul(std::mem::size_of::<String>())?)?
      |                                                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
12004 -             .checked_add(self.cursor_redo_edit_ids.capacity().checked_mul(std::mem::size_of::<String>())?)?
12004 +             .checked_add(self.cursor_redo_edit_ids.capacity().checked_mul(size_of::<String>())?)?
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:12005:71
      |
12005 |             .checked_add(self.applied_revision.capacity().checked_mul(std::mem::size_of::<CursorRevisionRecord>())?)?
      |                                                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
12005 -             .checked_add(self.applied_revision.capacity().checked_mul(std::mem::size_of::<CursorRevisionRecord>())?)?
12005 +             .checked_add(self.applied_revision.capacity().checked_mul(size_of::<CursorRevisionRecord>())?)?
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:12006:68
      |
12006 |             .checked_add(self.redo_revision.capacity().checked_mul(std::mem::size_of::<CursorRevisionRecord>())?)
      |                                                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
12006 -             .checked_add(self.redo_revision.capacity().checked_mul(std::mem::size_of::<CursorRevisionRecord>())?)
12006 +             .checked_add(self.redo_revision.capacity().checked_mul(size_of::<CursorRevisionRecord>())?)
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:12584:6
      |
12584 | impl serde::Serialize for ArtifactEditMessageLedger {
      |      ^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
12584 - impl serde::Serialize for ArtifactEditMessageLedger {
12584 + impl Serialize for ArtifactEditMessageLedger {
      |

warning: the `applied_edit_ids:` in this pattern is redundant
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:13598:13
      |
13598 |             applied_edit_ids: mut applied_edit_ids,
      |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use shorthand field pattern: `mut applied_edit_ids`
      |
      = note: `#[warn(non_shorthand_field_patterns)]` on by default

warning: the `redo_edit_ids:` in this pattern is redundant
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:13599:13
      |
13599 |             redo_edit_ids: mut redo_edit_ids,
      |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use shorthand field pattern: `mut redo_edit_ids`

warning: the `cursor_applied_edit_ids:` in this pattern is redundant
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:13600:13
      |
13600 |             cursor_applied_edit_ids: mut cursor_applied_edit_ids,
      |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use shorthand field pattern: `mut cursor_applied_edit_ids`

warning: the `cursor_redo_edit_ids:` in this pattern is redundant
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:13601:13
      |
13601 |             cursor_redo_edit_ids: mut cursor_redo_edit_ids,
      |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use shorthand field pattern: `mut cursor_redo_edit_ids`

warning: `semio-framework-os-kernel` (lib) generated 33 warnings (run `cargo fix --lib -p semio-framework-os-kernel` to apply 33 suggestions)
   Compiling semio-framework-ui v0.1.0 (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust)
warning: method `terminal_is_empty` is never used
   --> 🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust/🦀️math.rs:527:8
    |
440 | impl Mesh3dOwner {
    | ---------------- method in this implementation
...
527 |     fn terminal_is_empty(&self) -> bool {
    |        ^^^^^^^^^^^^^^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: `semio-framework-ui-scene` (lib) generated 1 warning
warning: use of deprecated method `std::sync::atomic::Atomic::<usize>::fetch_update`: renamed to `try_update` for consistency
   --> 🧰️framework/🔨️modules/🎭️actor/📦️packages/🦀️rust/../../🦀️component.rs:987:14
    |
987 | ...   .fetch_update(std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire, |pages| pages.checked_add(1).filter(...
    |        ^^^^^^^^^^^^
    |
    = note: `#[warn(deprecated)]` on by default
help: replace the use of the deprecated method
    |
987 -             .fetch_update(std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire, |pages| pages.checked_add(1).filter(|next| *next <= JOB_REPLAY_PROCESS_PAGE_CAPACITY))
987 +             .try_update(std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire, |pages| pages.checked_add(1).filter(|next| *next <= JOB_REPLAY_PROCESS_PAGE_CAPACITY))
    |

warning: struct `JobPayloadProjection` is never constructed
    --> 🧰️framework/🔨️modules/🎭️actor/📦️packages/🦀️rust/../../🦀️component.rs:1671:8
     |
1671 | struct JobPayloadProjection {
     |        ^^^^^^^^^^^^^^^^^^^^
     |
     = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: associated items `new`, `step`, and `take_bytes` are never used
    --> 🧰️framework/🔨️modules/🎭️actor/📦️packages/🦀️rust/../../🦀️component.rs:1678:8
     |
1677 | impl JobPayloadProjection {
     | ------------------------- associated items in this implementation
1678 |     fn new(owner: job::RetainedJobPayload) -> Self {
     |        ^^^
...
1683 |     fn step(&mut self) -> (bool, bool) {
     |        ^^^^
...
1705 |     fn take_bytes(&mut self) -> Vec<u8> {
     |        ^^^^^^^^^^

warning: enum `JobOutcomeProjection` is never used
    --> 🧰️framework/🔨️modules/🎭️actor/📦️packages/🦀️rust/../../🦀️component.rs:1711:6
     |
1711 | enum JobOutcomeProjection {
     |      ^^^^^^^^^^^^^^^^^^^^

warning: associated items `start` and `step` are never used
    --> 🧰️framework/🔨️modules/🎭️actor/📦️packages/🦀️rust/../../🦀️component.rs:1719:8
     |
1718 | impl JobOutcomeProjection {
     | ------------------------- associated items in this implementation
1719 |     fn start(outcome: job::StepOutcome) -> Result<JobStepOutcome, Self> {
     |        ^^^^^
...
1730 |     fn step(&mut self) -> Option<JobStepOutcome> {
     |        ^^^^

warning: `semio-framework-actor` (lib) generated 5 warnings (run `cargo fix --lib -p semio-framework-actor` to apply 1 suggestion)
   Compiling semio-framework-ui-runtime v0.1.0 (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust)
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
   Compiling semio-framework v0.1.0 (/Users/ueli/Documents/semio/🧰️framework/📦️packages/🦀️rust)
warning: unnecessary qualification
    --> 🧰️framework/📦️packages/🦀️rust/../../🔨️modules/🛂️manifest/../🎠️kernel/🦀️component.rs:1120:23
     |
1120 | const _: () = assert!(std::mem::size_of::<(UiTurnPatchRetireKey, UiTurnPatchContents)>() <= 4096);
     |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
     = note: requested on the command line with `-W unused-qualifications`
help: remove the unnecessary path segments
     |
1120 - const _: () = assert!(std::mem::size_of::<(UiTurnPatchRetireKey, UiTurnPatchContents)>() <= 4096);
1120 + const _: () = assert!(size_of::<(UiTurnPatchRetireKey, UiTurnPatchContents)>() <= 4096);
     |

warning: unnecessary qualification
    --> 🧰️framework/📦️packages/🦀️rust/../../🔨️modules/🛂️manifest/../🎠️kernel/🦀️component.rs:1331:23
     |
1331 | const _: () = assert!(std::mem::size_of::<(UiTurnPatchTransportKey, Option<UiTurnPatches>)>() <= 4096);
     |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
1331 - const _: () = assert!(std::mem::size_of::<(UiTurnPatchTransportKey, Option<UiTurnPatches>)>() <= 4096);
1331 + const _: () = assert!(size_of::<(UiTurnPatchTransportKey, Option<UiTurnPatches>)>() <= 4096);
     |

warning: unused variable: `base`
  --> 🧰️framework/📦️packages/🦀️rust/../../🛍️products/💻️os/🔨️modules/🔁️workflow/🧬️schema/🧬️mutations/🔒bind-parameter-field/🦀️.rs:16:23
   |
16 |     fn inverse(&self, base: &WorkflowSnapshot) -> Vec<WorkflowMutation> { vec![WorkflowMutation::UnbindParameterField(UnbindParamete...
   |                       ^^^^ help: if this is intentional, prefix it with an underscore: `_base`
   |
   = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

warning: unused variable: `base`
  --> 🧰️framework/📦️packages/🦀️rust/../../🛍️products/💻️os/🔨️modules/🔁️workflow/🧬️schema/🧬️mutations/🔄update-node-ports/🦀️.rs:16:23
   |
16 |     fn inverse(&self, base: &WorkflowSnapshot) -> Vec<WorkflowMutation> { Vec::new() }
   |                       ^^^^ help: if this is intentional, prefix it with an underscore: `_base`

warning: unused variable: `base`
  --> 🧰️framework/📦️packages/🦀️rust/../../🛍️products/💻️os/🔨️modules/🔁️workflow/🧬️schema/🧬️mutations/📥add-input/🦀️.rs:16:23
   |
16 |     fn inverse(&self, base: &WorkflowSnapshot) -> Vec<WorkflowMutation> { vec![WorkflowMutation::RemoveInput(RemoveInput { input_id:...
   |                       ^^^^ help: if this is intentional, prefix it with an underscore: `_base`

warning: unused variable: `base`
  --> 🧰️framework/📦️packages/🦀️rust/../../🛍️products/💻️os/🔨️modules/🔁️workflow/🧬️schema/🧬️mutations/🧩add-parameter/🦀️.rs:16:23
   |
16 |     fn inverse(&self, base: &WorkflowSnapshot) -> Vec<WorkflowMutation> { vec![WorkflowMutation::RemoveParameter(RemoveParameter { p...
   |                       ^^^^ help: if this is intentional, prefix it with an underscore: `_base`

warning: unused variable: `base`
  --> 🧰️framework/📦️packages/🦀️rust/../../🛍️products/💻️os/🔨️modules/🔁️workflow/🧬️schema/🧬️mutations/➕️add-node/🦀️.rs:16:23
   |
16 |     fn inverse(&self, base: &WorkflowSnapshot) -> Vec<WorkflowMutation> { vec![WorkflowMutation::RemoveNode(RemoveNode { node_id: se...
   |                       ^^^^ help: if this is intentional, prefix it with an underscore: `_base`

warning: unused variable: `base`
  --> 🧰️framework/📦️packages/🦀️rust/../../🛍️products/💻️os/🔨️modules/🔁️workflow/🧬️schema/🧬️mutations/🔗connect-ports/🦀️.rs:16:23
   |
16 |     fn inverse(&self, base: &WorkflowSnapshot) -> Vec<WorkflowMutation> { vec![WorkflowMutation::DisconnectEdge(DisconnectEdge { edg...
   |                       ^^^^ help: if this is intentional, prefix it with an underscore: `_base`

warning: method `request_session_close` is never used
    --> 🧰️framework/📦️packages/🦀️rust/../../🔨️modules/🛂️manifest/../🎠️kernel/🦀️component.rs:1296:8
     |
1260 | impl UiTurnPatchTransportArena {
     | ------------------------------ method in this implementation
...
1296 |     fn request_session_close(&mut self, session: u64) -> bool {
     |        ^^^^^^^^^^^^^^^^^^^^^
     |
     = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: type alias `Page` is never used
 --> 🧰️framework/📦️packages/🦀️rust/../../🔨️modules/🛂️manifest/../🎠️kernel/📤️return/🏠️source/📚️entries/🦀️component.rs:4:6
  |
4 | type Page<T> = Vec<Node<T>>;
  |      ^^^^

warning: type alias `Head` is never used
 --> 🧰️framework/📦️packages/🦀️rust/../../🔨️modules/🛂️manifest/../🎠️kernel/📤️return/🏠️source/📚️entries/🦀️component.rs:5:6
  |
5 | type Head<T> = Option<Page<T>>;
  |      ^^^^

warning: struct `Node` is never constructed
 --> 🧰️framework/📦️packages/🦀️rust/../../🔨️modules/🛂️manifest/../🎠️kernel/📤️return/🏠️source/📚️entries/🦀️component.rs:7:8
  |
7 | struct Node<T> { next: Head<T>, value: Option<T> }
  |        ^^^^

warning: struct `ReturnSourceAllocationError` is never constructed
  --> 🧰️framework/📦️packages/🦀️rust/../../🔨️modules/🛂️manifest/../🎠️kernel/📤️return/🏠️source/📚️entries/🦀️component.rs:10:19
   |
10 | pub(crate) struct ReturnSourceAllocationError { pub reason: &'static str, pub allocated_bytes: usize }
   |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: struct `ReturnSourceReservation` is never constructed
  --> 🧰️framework/📦️packages/🦀️rust/../../🔨️modules/🛂️manifest/../🎠️kernel/📤️return/🏠️source/📚️entries/🦀️component.rs:13:19
   |
13 | pub(crate) struct ReturnSourceReservation { pub ready: bool, pub allocated_bytes: usize }
   |                   ^^^^^^^^^^^^^^^^^^^^^^^

warning: struct `ReturnSourceEntryStep` is never constructed
  --> 🧰️framework/📦️packages/🦀️rust/../../🔨️modules/🛂️manifest/../🎠️kernel/📤️return/🏠️source/📚️entries/🦀️component.rs:16:19
   |
16 | pub(crate) struct ReturnSourceEntryStep {
   |                   ^^^^^^^^^^^^^^^^^^^^^

warning: enum `Phase` is never used
  --> 🧰️framework/📦️packages/🦀️rust/../../🔨️modules/🛂️manifest/../🎠️kernel/📤️return/🏠️source/📚️entries/🦀️component.rs:24:6
   |
24 | enum Phase { Building, Freezing, Frozen, Closing }
   |      ^^^^^

warning: struct `ReturnSourceEntries` is never constructed
  --> 🧰️framework/📦️packages/🦀️rust/../../🔨️modules/🛂️manifest/../🎠️kernel/📤️return/🏠️source/📚️entries/🦀️component.rs:27:19
   |
27 | pub(crate) struct ReturnSourceEntries<T> {
   |                   ^^^^^^^^^^^^^^^^^^^

warning: multiple associated items are never used
   --> 🧰️framework/📦️packages/🦀️rust/../../🔨️modules/🛂️manifest/../🎠️kernel/📤️return/🏠️source/📚️entries/🦀️component.rs:39:19
    |
 38 | impl<T> ReturnSourceEntries<T> {
    | ------------------------------ associated items in this implementation
 39 |     pub(crate) fn new(maximum_entries: usize) -> Self {
    |                   ^^^
...
 43 |     pub(crate) const fn required_allocation_bytes() -> usize { size_of::<Node<T>>() }
    |                         ^^^^^^^^^^^^^^^^^^^^^^^^^
 44 |     pub(crate) const fn required_placement_bytes() -> usize { size_of::<Node<T>>() + size_of::<Head<T>>() * 2 }
    |                         ^^^^^^^^^^^^^^^^^^^^^^^^
 45 |     pub(crate) const fn required_freeze_bytes() -> usize { size_of::<Head<T>>() * 4 }
    |                         ^^^^^^^^^^^^^^^^^^^^^
 46 |     pub(crate) const fn required_handoff_bytes() -> usize { size_of::<ReturnSourceEntry<T>>() + size_of::<Head<T>>() * 3 }
    |                         ^^^^^^^^^^^^^^^^^^^^^^
 47 |     pub(crate) fn allocated_bytes(&self) -> u128 { self.allocated_bytes }
    |                   ^^^^^^^^^^^^^^^
 48 |     pub(crate) fn terminal_is_empty(&self) -> bool { self.building.is_none() && self.frozen.is_none() && self.reserved.is_none() &&...
    |                   ^^^^^^^^^^^^^^^^^
...
 51 |     pub(crate) fn reserve_step(&mut self, maximum_allocation_bytes: usize) -> Result<ReturnSourceReservation, ReturnSourceAllocatio...
    |                   ^^^^^^^^^^^^
...
 60 |     fn reserve_capacity(&mut self, maximum_allocation_bytes: usize, capacity: usize) -> Result<ReturnSourceReservation, ReturnSourc...
    |        ^^^^^^^^^^^^^^^^
...
 86 |     pub(crate) fn try_push_reserved(&mut self, source: &mut Option<T>, maximum_placement_bytes: usize) -> Result<usize, &'static st...
    |                   ^^^^^^^^^^^^^^^^^
...
101 |     pub(crate) fn freeze_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<ReturnSourceEntryStep, &'static str> {
    |                   ^^^^^^^^^^^
...
119 |     pub(crate) fn take_front_into(&mut self, target: &mut Option<ReturnSourceEntry<T>>, maximum_bytes: usize) -> Result<bool, &'sta...
    |                   ^^^^^^^^^^^^^^^
...
125 |     pub(crate) fn begin_close(&mut self) { self.phase = Phase::Closing; }
    |                   ^^^^^^^^^^^
...
128 |     pub(crate) fn take_close_entry_into(&mut self, target: &mut Option<ReturnSourceEntry<T>>, maximum_bytes: usize) -> Result<bool,...
    |                   ^^^^^^^^^^^^^^^^^^^^^
...
135 |     fn handoff(head: &mut Head<T>, length: &mut usize, allocated_bytes: &mut u128, target: &mut Option<ReturnSourceEntry<T>>, maxim...
    |        ^^^^^^^

warning: struct `ReturnSourceEntry` is never constructed
   --> 🧰️framework/📦️packages/🦀️rust/../../🔨️modules/🛂️manifest/../🎠️kernel/📤️return/🏠️source/📚️entries/🦀️component.rs:157:19
    |
157 | pub(crate) struct ReturnSourceEntry<T> { page: ManuallyDrop<Head<T>>, allocated_bytes: usize }
    |                   ^^^^^^^^^^^^^^^^^

warning: methods `value`, `allocated_bytes`, `take_value_into`, and `close_empty_step` are never used
   --> 🧰️framework/📦️packages/🦀️rust/../../🔨️modules/🛂️manifest/../🎠️kernel/📤️return/🏠️source/📚️entries/🦀️component.rs:160:19
    |
159 | impl<T> ReturnSourceEntry<T> {
    | ---------------------------- methods in this implementation
160 |     pub(crate) fn value(&self) -> Option<&T> { self.page.as_ref().and_then(|page| page.first()).and_then(|node| node.value.as_ref()) }
    |                   ^^^^^
161 |     pub(crate) fn allocated_bytes(&self) -> usize { self.allocated_bytes }
    |                   ^^^^^^^^^^^^^^^
...
164 |     pub(crate) fn take_value_into(&mut self, target: &mut Option<T>, maximum_bytes: usize) -> Result<usize, &'static str> {
    |                   ^^^^^^^^^^^^^^^
...
173 |     pub(crate) fn close_empty_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<ReturnSourceEntryStep, &'static...
    |                   ^^^^^^^^^^^^^^^^

warning: function `page_bytes` is never used
   --> 🧰️framework/📦️packages/🦀️rust/../../🔨️modules/🛂️manifest/../🎠️kernel/📤️return/🏠️source/📚️entries/🦀️component.rs:187:4
    |
187 | fn page_bytes<T>(page: &Page<T>) -> usize { page.capacity() * size_of::<Node<T>>() }
    |    ^^^^^^^^^^

warning: `semio-framework` (lib) generated 21 warnings (run `cargo fix --lib -p semio-framework` to apply 8 suggestions)
   Compiling semio-framework-plugin v0.1.0 (/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust)
warning: macro-expanded `macro_export` macros from the current crate cannot be referred to by absolute paths
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:27536:17
      |
27536 |             use crate::__semio_dispatch_PluginApp;
      |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
note: the macro is defined here
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:11484:5
      |
11484 |     #[dyn_enum]
      |     ^^^^^^^^^^^
      = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
      = note: for more information, see issue #52234 <https://github.com/rust-lang/rust/issues/52234>
      = note: `-W macro-expanded-macro-exports-accessed-by-absolute-paths` implied by `-W future-incompatible`
      = help: to override `-W future-incompatible` add `#[allow(macro_expanded_macro_exports_accessed_by_absolute_paths)]`
      = note: this warning originates in the attribute macro `dyn_enum` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: macro-expanded `macro_export` macros from the current crate cannot be referred to by absolute paths
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33233:13
      |
33233 |         use crate::__semio_dispatch_PluginApp;
      |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
note: the macro is defined here
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:11484:5
      |
11484 |     #[dyn_enum]
      |     ^^^^^^^^^^^
      = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
      = note: for more information, see issue #52234 <https://github.com/rust-lang/rust/issues/52234>
      = note: this warning originates in the attribute macro `dyn_enum` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: macro-expanded `macro_export` macros from the current crate cannot be referred to by absolute paths
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🏗️builder/🦀️component.rs:944:9
      |
  944 |     use crate::__semio_dispatch_PluginApp;
      |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
note: the macro is defined here
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:11484:5
      |
11484 |     #[dyn_enum]
      |     ^^^^^^^^^^^
      = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
      = note: for more information, see issue #52234 <https://github.com/rust-lang/rust/issues/52234>
      = note: this warning originates in the attribute macro `dyn_enum` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: unused import: `SurfaceKind`
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🏗️builder/🦀️component.rs:867:41
    |
867 |     use ui_wgpu::wgpu::{LocalizedLabel, SurfaceKind};
    |                                         ^^^^^^^^^^^
    |
    = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: unnecessary qualification
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🏗️builder/🦀️component.rs:905:107
    |
905 | ...onfigView<'_, NoConfig>) -> crate::app::UiAssemblyResult<semio_framework_ui_runtime::ComponentTree> {
    |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: requested on the command line with `-W unused-qualifications`
help: remove the unnecessary path segments
    |
905 -         fn render(_body_key: &str, _doc: &ArtifactView<'_, NoConfig>, _cfg: &ConfigView<'_, NoConfig>) -> crate::app::UiAssemblyResult<semio_framework_ui_runtime::ComponentTree> {
905 +         fn render(_body_key: &str, _doc: &ArtifactView<'_, NoConfig>, _cfg: &ConfigView<'_, NoConfig>) -> UiAssemblyResult<semio_framework_ui_runtime::ComponentTree> {
    |

warning: unnecessary qualification
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🏗️builder/🦀️component.rs:905:136
    |
905 | ...NoConfig>) -> crate::app::UiAssemblyResult<semio_framework_ui_runtime::ComponentTree> {
    |                                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
905 -         fn render(_body_key: &str, _doc: &ArtifactView<'_, NoConfig>, _cfg: &ConfigView<'_, NoConfig>) -> crate::app::UiAssemblyResult<semio_framework_ui_runtime::ComponentTree> {
905 +         fn render(_body_key: &str, _doc: &ArtifactView<'_, NoConfig>, _cfg: &ConfigView<'_, NoConfig>) -> crate::app::UiAssemblyResult<ComponentTree> {
    |

warning: unnecessary qualification
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🏗️builder/🦀️component.rs:906:13
    |
906 |             crate::app::built_text_to_component_tree(ui_wgpu::wgpu::Label::data("schema-stamp-editor"))
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
906 -             crate::app::built_text_to_component_tree(ui_wgpu::wgpu::Label::data("schema-stamp-editor"))
906 +             built_text_to_component_tree(ui_wgpu::wgpu::Label::data("schema-stamp-editor"))
    |

warning: unnecessary qualification
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🏗️builder/🦀️component.rs:934:107
    |
934 | ...onfigView<'_, NoConfig>) -> crate::app::UiAssemblyResult<semio_framework_ui_runtime::ComponentTree> {
    |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
934 -         fn render(_body_key: &str, _doc: &ArtifactView<'_, NoConfig>, _cfg: &ConfigView<'_, NoConfig>) -> crate::app::UiAssemblyResult<semio_framework_ui_runtime::ComponentTree> {
934 +         fn render(_body_key: &str, _doc: &ArtifactView<'_, NoConfig>, _cfg: &ConfigView<'_, NoConfig>) -> UiAssemblyResult<semio_framework_ui_runtime::ComponentTree> {
    |

warning: unnecessary qualification
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🏗️builder/🦀️component.rs:934:136
    |
934 | ...NoConfig>) -> crate::app::UiAssemblyResult<semio_framework_ui_runtime::ComponentTree> {
    |                                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
934 -         fn render(_body_key: &str, _doc: &ArtifactView<'_, NoConfig>, _cfg: &ConfigView<'_, NoConfig>) -> crate::app::UiAssemblyResult<semio_framework_ui_runtime::ComponentTree> {
934 +         fn render(_body_key: &str, _doc: &ArtifactView<'_, NoConfig>, _cfg: &ConfigView<'_, NoConfig>) -> crate::app::UiAssemblyResult<ComponentTree> {
    |

warning: unnecessary qualification
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🏗️builder/🦀️component.rs:935:13
    |
935 |             crate::app::built_text_to_component_tree(ui_wgpu::wgpu::Label::data("schema-stamp-viewer"))
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
935 -             crate::app::built_text_to_component_tree(ui_wgpu::wgpu::Label::data("schema-stamp-viewer"))
935 +             built_text_to_component_tree(ui_wgpu::wgpu::Label::data("schema-stamp-viewer"))
    |

warning: unnecessary qualification
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🏗️builder/🦀️component.rs:948:20
    |
948 |             Editor(crate::app::VcsArtifactApp<crate::app::EditorApp<SchemaStampEditorFixture>>),
    |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
948 -             Editor(crate::app::VcsArtifactApp<crate::app::EditorApp<SchemaStampEditorFixture>>),
948 +             Editor(VcsArtifactApp<crate::app::EditorApp<SchemaStampEditorFixture>>),
    |

warning: unnecessary qualification
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🏗️builder/🦀️component.rs:948:47
    |
948 |             Editor(crate::app::VcsArtifactApp<crate::app::EditorApp<SchemaStampEditorFixture>>),
    |                                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
948 -             Editor(crate::app::VcsArtifactApp<crate::app::EditorApp<SchemaStampEditorFixture>>),
948 +             Editor(crate::app::VcsArtifactApp<EditorApp<SchemaStampEditorFixture>>),
    |

warning: unnecessary qualification
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🏗️builder/🦀️component.rs:949:20
    |
949 |             Viewer(crate::app::VcsArtifactApp<crate::app::ViewerApp<SchemaStampViewerFixture>>),
    |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
949 -             Viewer(crate::app::VcsArtifactApp<crate::app::ViewerApp<SchemaStampViewerFixture>>),
949 +             Viewer(VcsArtifactApp<crate::app::ViewerApp<SchemaStampViewerFixture>>),
    |

warning: unnecessary qualification
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🏗️builder/🦀️component.rs:949:47
    |
949 |             Viewer(crate::app::VcsArtifactApp<crate::app::ViewerApp<SchemaStampViewerFixture>>),
    |                                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
949 -             Viewer(crate::app::VcsArtifactApp<crate::app::ViewerApp<SchemaStampViewerFixture>>),
949 +             Viewer(crate::app::VcsArtifactApp<ViewerApp<SchemaStampViewerFixture>>),
    |

warning: unnecessary qualification
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🏗️builder/🦀️component.rs:953:64
    |
953 |     fn minimal_surface_def(dialect: Dialect, role: AppRole) -> crate::app::AppDefinition {
    |                                                                ^^^^^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
953 -     fn minimal_surface_def(dialect: Dialect, role: AppRole) -> crate::app::AppDefinition {
953 +     fn minimal_surface_def(dialect: Dialect, role: AppRole) -> AppDefinition {
    |

warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🏗️builder/🦀️component.rs:1013:31
     |
1013 | ...   let plugin = Plugin::<crate::app::NoPluginApp>::builder(metadata.owner).label("Builder Test Routed Inference").version("0.1....
     |                             ^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
1013 -         let plugin = Plugin::<crate::app::NoPluginApp>::builder(metadata.owner).label("Builder Test Routed Inference").version("0.1.0").routed_inference(metadata).try_build().expect("metadata-only routed inference must assemble");
1013 +         let plugin = Plugin::<NoPluginApp>::builder(metadata.owner).label("Builder Test Routed Inference").version("0.1.0").routed_inference(metadata).try_build().expect("metadata-only routed inference must assemble");
     |

warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🏗️builder/🦀️component.rs:1014:25
     |
1014 | ...   let roster: Vec<crate::app::WireArtifactInferenceMetadata> = serde_json::from_slice(&plugin.wire_list_artifact_inference_ser...
     |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
1014 -         let roster: Vec<crate::app::WireArtifactInferenceMetadata> = serde_json::from_slice(&plugin.wire_list_artifact_inference_services().expect("frozen roster encodes")).expect("frozen roster decodes");
1014 +         let roster: Vec<WireArtifactInferenceMetadata> = serde_json::from_slice(&plugin.wire_list_artifact_inference_services().expect("frozen roster encodes")).expect("frozen roster decodes");
     |

warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🏗️builder/🦀️component.rs:1016:17
     |
1016 | ...   assert!(crate::app::artifact_inference_service(metadata.artifact_kind, metadata.inference_schema).expect("global service loo...
     |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
1016 -         assert!(crate::app::artifact_inference_service(metadata.artifact_kind, metadata.inference_schema).expect("global service lookup").is_none(), "route must not manufacture a synchronous service facade");
1016 +         assert!(artifact_inference_service(metadata.artifact_kind, metadata.inference_schema).expect("global service lookup").is_none(), "route must not manufacture a synchronous service facade");
     |

warning: unnecessary qualification
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/💼️jobs/💡️infer/🦀️component.rs:504:25
    |
504 | ...   let operation = semio_framework_job::Operation::new(semio_framework_job::OperationId(7), semio_framework_job::RevisionId(11),...
    |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
504 -         let operation = semio_framework_job::Operation::new(semio_framework_job::OperationId(7), semio_framework_job::RevisionId(11), semio_framework_job::Generation(3), 0);
504 +         let operation = Operation::new(semio_framework_job::OperationId(7), semio_framework_job::RevisionId(11), semio_framework_job::Generation(3), 0);
    |

warning: unnecessary qualification
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/💼️jobs/💡️infer/🦀️component.rs:504:61
    |
504 | ...   let operation = semio_framework_job::Operation::new(semio_framework_job::OperationId(7), semio_framework_job::RevisionId(11),...
    |                                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
504 -         let operation = semio_framework_job::Operation::new(semio_framework_job::OperationId(7), semio_framework_job::RevisionId(11), semio_framework_job::Generation(3), 0);
504 +         let operation = semio_framework_job::Operation::new(OperationId(7), semio_framework_job::RevisionId(11), semio_framework_job::Generation(3), 0);
    |

warning: unnecessary qualification
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/💼️jobs/💡️infer/🦀️component.rs:504:98
    |
504 | ...   let operation = semio_framework_job::Operation::new(semio_framework_job::OperationId(7), semio_framework_job::RevisionId(11),...
    |                                                                                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
504 -         let operation = semio_framework_job::Operation::new(semio_framework_job::OperationId(7), semio_framework_job::RevisionId(11), semio_framework_job::Generation(3), 0);
504 +         let operation = semio_framework_job::Operation::new(semio_framework_job::OperationId(7), RevisionId(11), semio_framework_job::Generation(3), 0);
    |

warning: unnecessary qualification
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/💼️jobs/💡️infer/🦀️component.rs:504:135
    |
504 | ...tionId(7), semio_framework_job::RevisionId(11), semio_framework_job::Generation(3), 0);
    |                                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
504 -         let operation = semio_framework_job::Operation::new(semio_framework_job::OperationId(7), semio_framework_job::RevisionId(11), semio_framework_job::Generation(3), 0);
504 +         let operation = semio_framework_job::Operation::new(semio_framework_job::OperationId(7), semio_framework_job::RevisionId(11), Generation(3), 0);
    |

warning: unnecessary qualification
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/💼️jobs/💡️infer/🦀️component.rs:529:25
    |
529 | ...   let operation = semio_framework_job::Operation::new(semio_framework_job::OperationId(8), semio_framework_job::RevisionId(11),...
    |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
529 -         let operation = semio_framework_job::Operation::new(semio_framework_job::OperationId(8), semio_framework_job::RevisionId(11), semio_framework_job::Generation(3), 0);
529 +         let operation = Operation::new(semio_framework_job::OperationId(8), semio_framework_job::RevisionId(11), semio_framework_job::Generation(3), 0);
    |

warning: unnecessary qualification
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/💼️jobs/💡️infer/🦀️component.rs:529:61
    |
529 | ...   let operation = semio_framework_job::Operation::new(semio_framework_job::OperationId(8), semio_framework_job::RevisionId(11),...
    |                                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
529 -         let operation = semio_framework_job::Operation::new(semio_framework_job::OperationId(8), semio_framework_job::RevisionId(11), semio_framework_job::Generation(3), 0);
529 +         let operation = semio_framework_job::Operation::new(OperationId(8), semio_framework_job::RevisionId(11), semio_framework_job::Generation(3), 0);
    |

warning: unnecessary qualification
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/💼️jobs/💡️infer/🦀️component.rs:529:98
    |
529 | ...   let operation = semio_framework_job::Operation::new(semio_framework_job::OperationId(8), semio_framework_job::RevisionId(11),...
    |                                                                                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
529 -         let operation = semio_framework_job::Operation::new(semio_framework_job::OperationId(8), semio_framework_job::RevisionId(11), semio_framework_job::Generation(3), 0);
529 +         let operation = semio_framework_job::Operation::new(semio_framework_job::OperationId(8), RevisionId(11), semio_framework_job::Generation(3), 0);
    |

warning: unnecessary qualification
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/💼️jobs/💡️infer/🦀️component.rs:529:135
    |
529 | ...tionId(8), semio_framework_job::RevisionId(11), semio_framework_job::Generation(3), 0);
    |                                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
529 -         let operation = semio_framework_job::Operation::new(semio_framework_job::OperationId(8), semio_framework_job::RevisionId(11), semio_framework_job::Generation(3), 0);
529 +         let operation = semio_framework_job::Operation::new(semio_framework_job::OperationId(8), semio_framework_job::RevisionId(11), Generation(3), 0);
    |

warning: unnecessary qualification
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🩹️patches/🦀️component.rs:257:10
    |
257 |     key: super::instance_lifetime::NativeCloseKey,
    |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
257 -     key: super::instance_lifetime::NativeCloseKey,
257 +     key: NativeCloseKey,
    |

warning: unnecessary qualification
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🩹️patches/🦀️component.rs:554:24
    |
554 |         let metadata = std::mem::size_of::<ReadySlot>();
    |                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
554 -         let metadata = std::mem::size_of::<ReadySlot>();
554 +         let metadata = size_of::<ReadySlot>();
    |

warning: unnecessary qualification
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🩹️patches/🦀️component.rs:647:54
    |
647 |     pub(crate) fn reserve_close_instance(&self, key: super::instance_lifetime::NativeCloseKey) -> Result<(), &'static str> {
    |                                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
647 -     pub(crate) fn reserve_close_instance(&self, key: super::instance_lifetime::NativeCloseKey) -> Result<(), &'static str> {
647 +     pub(crate) fn reserve_close_instance(&self, key: NativeCloseKey) -> Result<(), &'static str> {
    |

warning: unnecessary qualification
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🩹️patches/🦀️component.rs:665:55
    |
665 |     pub(crate) fn activate_close_instance(&self, key: super::instance_lifetime::NativeCloseKey) -> Result<(), &'static str> {
    |                                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
665 -     pub(crate) fn activate_close_instance(&self, key: super::instance_lifetime::NativeCloseKey) -> Result<(), &'static str> {
665 +     pub(crate) fn activate_close_instance(&self, key: NativeCloseKey) -> Result<(), &'static str> {
    |

warning: unnecessary qualification
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🩹️patches/🦀️component.rs:672:55
    |
672 |     pub(crate) fn close_instance_complete(&self, key: super::instance_lifetime::NativeCloseKey) -> Result<bool, &'static str> {
    |                                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
672 -     pub(crate) fn close_instance_complete(&self, key: super::instance_lifetime::NativeCloseKey) -> Result<bool, &'static str> {
672 +     pub(crate) fn close_instance_complete(&self, key: NativeCloseKey) -> Result<bool, &'static str> {
    |

warning: unnecessary qualification
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🩹️patches/🦀️component.rs:677:54
    |
677 |     pub(crate) fn release_close_instance(&self, key: super::instance_lifetime::NativeCloseKey) -> Result<(), &'static str> {
    |                                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
677 -     pub(crate) fn release_close_instance(&self, key: super::instance_lifetime::NativeCloseKey) -> Result<(), &'static str> {
677 +     pub(crate) fn release_close_instance(&self, key: NativeCloseKey) -> Result<(), &'static str> {
    |

warning: unnecessary qualification
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🩹️patches/🦀️component.rs:888:30
    |
888 |         let receiver_bytes = std::mem::size_of::<ReadySlot>();
    |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
888 -         let receiver_bytes = std::mem::size_of::<ReadySlot>();
888 +         let receiver_bytes = size_of::<ReadySlot>();
    |

warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🩹️patches/🦀️component.rs:1053:19
     |
1053 |         let key = super::super::instance_lifetime::NativeCloseKey::fixture(instance, 1);
     |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
1053 -         let key = super::super::instance_lifetime::NativeCloseKey::fixture(instance, 1);
1053 +         let key = NativeCloseKey::fixture(instance, 1);
     |

warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🩹️patches/🦀️component.rs:1090:27
     |
1090 |         let mut outputs = semio_framework_ui_runtime::SurfaceReconcileOutputs::default();
     |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
1090 -         let mut outputs = semio_framework_ui_runtime::SurfaceReconcileOutputs::default();
1090 +         let mut outputs = SurfaceReconcileOutputs::default();
     |

warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🩹️patches/🦀️component.rs:1169:21
     |
1169 |         let bytes = std::mem::size_of::<PatchTrackerState>();
     |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
1169 -         let bytes = std::mem::size_of::<PatchTrackerState>();
1169 +         let bytes = size_of::<PatchTrackerState>();
     |

warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🩹️patches/🦀️component.rs:1585:19
     |
1585 |         let key = super::super::instance_lifetime::NativeCloseKey::fixture(instance, 1);
     |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
1585 -         let key = super::super::instance_lifetime::NativeCloseKey::fixture(instance, 1);
1585 +         let key = NativeCloseKey::fixture(instance, 1);
     |

warning: unnecessary qualification
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/📨️pending/🦀️component.rs:93:37
   |
93 |                 if admitted_bytes < std::mem::size_of::<UiPatch>() { return Ok(None); }
   |                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
93 -                 if admitted_bytes < std::mem::size_of::<UiPatch>() { return Ok(None); }
93 +                 if admitted_bytes < size_of::<UiPatch>() { return Ok(None); }
   |

warning: unused import: `HashMap`
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🦀️component.rs:58:24
   |
58 | use std::collections::{HashMap, VecDeque};
   |                        ^^^^^^^

warning: unnecessary qualification
 --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🕹️interaction/🧬️mutations/🔁️set-state/🧪️.rs:8:16
  |
8 |     let state: protocol::InteractionState = serde_json::from_value(source.clone()).unwrap();
  |                ^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
help: remove the unnecessary path segments
  |
8 -     let state: protocol::InteractionState = serde_json::from_value(source.clone()).unwrap();
8 +     let state: InteractionState = serde_json::from_value(source.clone()).unwrap();
  |

warning: unnecessary qualification
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🕹️interaction/🧬️mutations/🔁️set-state/🧪️.rs:21:32
   |
21 |     assert_eq!(mutation.apply(&protocol::InteractionState::default()).unwrap(), state);
   |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
21 -     assert_eq!(mutation.apply(&protocol::InteractionState::default()).unwrap(), state);
21 +     assert_eq!(mutation.apply(&InteractionState::default()).unwrap(), state);
   |

warning: unnecessary qualification
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🕹️interaction/🧬️mutations/🔁️set-state/🧪️.rs:22:37
   |
22 |     let inverse = mutation.inverse(&protocol::InteractionState::default());
   |                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
22 -     let inverse = mutation.inverse(&protocol::InteractionState::default());
22 +     let inverse = mutation.inverse(&InteractionState::default());
   |

warning: unnecessary qualification
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🕹️interaction/🧬️mutations/🔁️set-state/🧪️.rs:23:51
   |
23 |     assert_eq!(inverse[0].apply(&state).unwrap(), protocol::InteractionState::default());
   |                                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
23 -     assert_eq!(inverse[0].apply(&state).unwrap(), protocol::InteractionState::default());
23 +     assert_eq!(inverse[0].apply(&state).unwrap(), InteractionState::default());
   |

warning: unnecessary qualification
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🧬️publication-fixtures/👥️presence/🧬️mutations/📝️change-publication-presence/🦀️.rs:17:50
   |
17 |     fn parse_revision(line: &str) -> Result<u64, crate::store::TextError> {
   |                                                  ^^^^^^^^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
17 -     fn parse_revision(line: &str) -> Result<u64, crate::store::TextError> {
17 +     fn parse_revision(line: &str) -> Result<u64, store::TextError> {
   |

warning: unnecessary qualification
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🧬️publication-fixtures/👥️presence/🧬️mutations/📝️change-publication-presence/🦀️.rs:18:92
   |
18 | ...   let revision = line.strip_prefix(&format!("{} ", Self::TEXT_OPCODE)).ok_or_else(|| crate::store::TextError::new(format!("unkno...
   |                                                                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
18 -         let revision = line.strip_prefix(&format!("{} ", Self::TEXT_OPCODE)).ok_or_else(|| crate::store::TextError::new(format!("unknown publication presence op '{line}'"), crate::store::TextSpan::at(1, 1)))?;
18 +         let revision = line.strip_prefix(&format!("{} ", Self::TEXT_OPCODE)).ok_or_else(|| store::TextError::new(format!("unknown publication presence op '{line}'"), crate::store::TextSpan::at(1, 1)))?;
   |

warning: unnecessary qualification
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🧬️publication-fixtures/👥️presence/🧬️mutations/📝️change-publication-presence/🦀️.rs:18:174
   |
18 | ...rmat!("unknown publication presence op '{line}'"), crate::store::TextSpan::at(1, 1)))?;
   |                                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
18 -         let revision = line.strip_prefix(&format!("{} ", Self::TEXT_OPCODE)).ok_or_else(|| crate::store::TextError::new(format!("unknown publication presence op '{line}'"), crate::store::TextSpan::at(1, 1)))?;
18 +         let revision = line.strip_prefix(&format!("{} ", Self::TEXT_OPCODE)).ok_or_else(|| crate::store::TextError::new(format!("unknown publication presence op '{line}'"), store::TextSpan::at(1, 1)))?;
   |

warning: unnecessary qualification
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🧬️publication-fixtures/👥️presence/🧬️mutations/📝️change-publication-presence/🦀️.rs:20:24
   |
20 | ...   return Err(crate::store::TextError::new("publication presence revision must be one unsigned decimal", crate::store::TextSpan::...
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
20 -             return Err(crate::store::TextError::new("publication presence revision must be one unsigned decimal", crate::store::TextSpan::at(1, 1)));
20 +             return Err(store::TextError::new("publication presence revision must be one unsigned decimal", crate::store::TextSpan::at(1, 1)));
   |

warning: unnecessary qualification
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🧬️publication-fixtures/👥️presence/🧬️mutations/📝️change-publication-presence/🦀️.rs:20:115
   |
20 | ...n presence revision must be one unsigned decimal", crate::store::TextSpan::at(1, 1)));
   |                                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
20 -             return Err(crate::store::TextError::new("publication presence revision must be one unsigned decimal", crate::store::TextSpan::at(1, 1)));
20 +             return Err(crate::store::TextError::new("publication presence revision must be one unsigned decimal", store::TextSpan::at(1, 1)));
   |

warning: unnecessary qualification
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🧬️publication-fixtures/👥️presence/🧬️mutations/📝️change-publication-presence/🦀️.rs:22:38
   |
22 | ...   revision.parse().map_err(|_| crate::store::TextError::new("publication presence revision is outside u64", crate::store::TextSp...
   |                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
22 -         revision.parse().map_err(|_| crate::store::TextError::new("publication presence revision is outside u64", crate::store::TextSpan::at(1, 1)))
22 +         revision.parse().map_err(|_| store::TextError::new("publication presence revision is outside u64", crate::store::TextSpan::at(1, 1)))
   |

warning: unnecessary qualification
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🧬️publication-fixtures/👥️presence/🧬️mutations/📝️change-publication-presence/🦀️.rs:22:115
   |
22 | ...ew("publication presence revision is outside u64", crate::store::TextSpan::at(1, 1)))
   |                                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
22 -         revision.parse().map_err(|_| crate::store::TextError::new("publication presence revision is outside u64", crate::store::TextSpan::at(1, 1)))
22 +         revision.parse().map_err(|_| crate::store::TextError::new("publication presence revision is outside u64", store::TextSpan::at(1, 1)))
   |

warning: unnecessary qualification
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🧬️publication-fixtures/👥️presence/🧬️mutations/📝️change-publication-presence/🦀️.rs:27:45
   |
27 |     fn parse_op(line: &str) -> Result<Self, crate::store::TextError> {
   |                                             ^^^^^^^^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
27 -     fn parse_op(line: &str) -> Result<Self, crate::store::TextError> {
27 +     fn parse_op(line: &str) -> Result<Self, store::TextError> {
   |

warning: unnecessary qualification
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🧬️publication-fixtures/👥️presence/🧬️mutations/🦀️.rs:15:45
   |
15 |     fn parse_op(line: &str) -> Result<Self, crate::store::TextError> {
   |                                             ^^^^^^^^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
15 -     fn parse_op(line: &str) -> Result<Self, crate::store::TextError> {
15 +     fn parse_op(line: &str) -> Result<Self, store::TextError> {
   |

warning: unnecessary qualification
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🧬️publication-fixtures/🫧️transient/🧬️mutations/📝️change-publication-transient/🦀️.rs:17:50
   |
17 |     fn parse_revision(line: &str) -> Result<u64, crate::store::TextError> {
   |                                                  ^^^^^^^^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
17 -     fn parse_revision(line: &str) -> Result<u64, crate::store::TextError> {
17 +     fn parse_revision(line: &str) -> Result<u64, store::TextError> {
   |

warning: unnecessary qualification
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🧬️publication-fixtures/🫧️transient/🧬️mutations/📝️change-publication-transient/🦀️.rs:18:92
   |
18 | ...   let revision = line.strip_prefix(&format!("{} ", Self::TEXT_OPCODE)).ok_or_else(|| crate::store::TextError::new(format!("unkno...
   |                                                                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
18 -         let revision = line.strip_prefix(&format!("{} ", Self::TEXT_OPCODE)).ok_or_else(|| crate::store::TextError::new(format!("unknown publication transient op '{line}'"), crate::store::TextSpan::at(1, 1)))?;
18 +         let revision = line.strip_prefix(&format!("{} ", Self::TEXT_OPCODE)).ok_or_else(|| store::TextError::new(format!("unknown publication transient op '{line}'"), crate::store::TextSpan::at(1, 1)))?;
   |

warning: unnecessary qualification
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🧬️publication-fixtures/🫧️transient/🧬️mutations/📝️change-publication-transient/🦀️.rs:18:175
   |
18 | ...mat!("unknown publication transient op '{line}'"), crate::store::TextSpan::at(1, 1)))?;
   |                                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
18 -         let revision = line.strip_prefix(&format!("{} ", Self::TEXT_OPCODE)).ok_or_else(|| crate::store::TextError::new(format!("unknown publication transient op '{line}'"), crate::store::TextSpan::at(1, 1)))?;
18 +         let revision = line.strip_prefix(&format!("{} ", Self::TEXT_OPCODE)).ok_or_else(|| crate::store::TextError::new(format!("unknown publication transient op '{line}'"), store::TextSpan::at(1, 1)))?;
   |

warning: unnecessary qualification
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🧬️publication-fixtures/🫧️transient/🧬️mutations/📝️change-publication-transient/🦀️.rs:20:24
   |
20 | ...   return Err(crate::store::TextError::new("publication transient revision must be one unsigned decimal", crate::store::TextSpan:...
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
20 -             return Err(crate::store::TextError::new("publication transient revision must be one unsigned decimal", crate::store::TextSpan::at(1, 1)));
20 +             return Err(store::TextError::new("publication transient revision must be one unsigned decimal", crate::store::TextSpan::at(1, 1)));
   |

warning: unnecessary qualification
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🧬️publication-fixtures/🫧️transient/🧬️mutations/📝️change-publication-transient/🦀️.rs:20:116
   |
20 | ... transient revision must be one unsigned decimal", crate::store::TextSpan::at(1, 1)));
   |                                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
20 -             return Err(crate::store::TextError::new("publication transient revision must be one unsigned decimal", crate::store::TextSpan::at(1, 1)));
20 +             return Err(crate::store::TextError::new("publication transient revision must be one unsigned decimal", store::TextSpan::at(1, 1)));
   |

warning: unnecessary qualification
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🧬️publication-fixtures/🫧️transient/🧬️mutations/📝️change-publication-transient/🦀️.rs:22:38
   |
22 | ...   revision.parse().map_err(|_| crate::store::TextError::new("publication transient revision is outside u64", crate::store::TextS...
   |                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
22 -         revision.parse().map_err(|_| crate::store::TextError::new("publication transient revision is outside u64", crate::store::TextSpan::at(1, 1)))
22 +         revision.parse().map_err(|_| store::TextError::new("publication transient revision is outside u64", crate::store::TextSpan::at(1, 1)))
   |

warning: unnecessary qualification
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🧬️publication-fixtures/🫧️transient/🧬️mutations/📝️change-publication-transient/🦀️.rs:22:116
   |
22 | ...w("publication transient revision is outside u64", crate::store::TextSpan::at(1, 1)))
   |                                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
22 -         revision.parse().map_err(|_| crate::store::TextError::new("publication transient revision is outside u64", crate::store::TextSpan::at(1, 1)))
22 +         revision.parse().map_err(|_| crate::store::TextError::new("publication transient revision is outside u64", store::TextSpan::at(1, 1)))
   |

warning: unnecessary qualification
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🧬️publication-fixtures/🫧️transient/🧬️mutations/📝️change-publication-transient/🦀️.rs:27:45
   |
27 |     fn parse_op(line: &str) -> Result<Self, crate::store::TextError> {
   |                                             ^^^^^^^^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
27 -     fn parse_op(line: &str) -> Result<Self, crate::store::TextError> {
27 +     fn parse_op(line: &str) -> Result<Self, store::TextError> {
   |

warning: unnecessary qualification
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🧬️publication-fixtures/🫧️transient/🧬️mutations/🦀️.rs:15:45
   |
15 |     fn parse_op(line: &str) -> Result<Self, crate::store::TextError> {
   |                                             ^^^^^^^^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
15 -     fn parse_op(line: &str) -> Result<Self, crate::store::TextError> {
15 +     fn parse_op(line: &str) -> Result<Self, store::TextError> {
   |

warning: unnecessary qualification
 --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🧬️test-app-mutations/🎚️config/🧬️mutations/📝️change-test-config-selection/🦀️.rs:5:81
  |
5 | impl OpText for ChangeTestConfigSelection { fn parse_op(line:&str)->Result<Self,crate::store::TextError>{let value=line.strip_prefix(...
  |                                                                                 ^^^^^^^^^^^^^^^^^^^^^^^
  |
help: remove the unnecessary path segments
  |
5 - impl OpText for ChangeTestConfigSelection { fn parse_op(line:&str)->Result<Self,crate::store::TextError>{let value=line.strip_prefix("change-test-config-selection ").ok_or_else(||crate::store::TextError::new("expected change-test-config-selection",crate::store::TextSpan::at(1,1)))?; Ok(Self{selected:serde_json::from_str(value).map_err(|_|crate::store::TextError::new("selection must be a JSON nullable string",crate::store::TextSpan::at(1,1)))?})} fn print_op(&self)->String{format!("{} {}",Self::OPCODE,serde_json::to_string(&self.selected).expect("nullable string serializes"))} }
5 + impl OpText for ChangeTestConfigSelection { fn parse_op(line:&str)->Result<Self,store::TextError>{let value=line.strip_prefix("change-test-config-selection ").ok_or_else(||crate::store::TextError::new("expected change-test-config-selection",crate::store::TextSpan::at(1,1)))?; Ok(Self{selected:serde_json::from_str(value).map_err(|_|crate::store::TextError::new("selection must be a JSON nullable string",crate::store::TextSpan::at(1,1)))?})} fn print_op(&self)->String{format!("{} {}",Self::OPCODE,serde_json::to_string(&self.selected).expect("nullable string serializes"))} }
  |

warning: unnecessary qualification
 --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🧬️test-app-mutations/🎚️config/🧬️mutations/📝️change-test-config-selection/🦀️.rs:5:180
  |
5 | ...efix("change-test-config-selection ").ok_or_else(||crate::store::TextError::new("expected change-test-config-selection",crate::sto...
  |                                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
help: remove the unnecessary path segments
  |
5 - impl OpText for ChangeTestConfigSelection { fn parse_op(line:&str)->Result<Self,crate::store::TextError>{let value=line.strip_prefix("change-test-config-selection ").ok_or_else(||crate::store::TextError::new("expected change-test-config-selection",crate::store::TextSpan::at(1,1)))?; Ok(Self{selected:serde_json::from_str(value).map_err(|_|crate::store::TextError::new("selection must be a JSON nullable string",crate::store::TextSpan::at(1,1)))?})} fn print_op(&self)->String{format!("{} {}",Self::OPCODE,serde_json::to_string(&self.selected).expect("nullable string serializes"))} }
5 + impl OpText for ChangeTestConfigSelection { fn parse_op(line:&str)->Result<Self,crate::store::TextError>{let value=line.strip_prefix("change-test-config-selection ").ok_or_else(||store::TextError::new("expected change-test-config-selection",crate::store::TextSpan::at(1,1)))?; Ok(Self{selected:serde_json::from_str(value).map_err(|_|crate::store::TextError::new("selection must be a JSON nullable string",crate::store::TextSpan::at(1,1)))?})} fn print_op(&self)->String{format!("{} {}",Self::OPCODE,serde_json::to_string(&self.selected).expect("nullable string serializes"))} }
  |

warning: unnecessary qualification
 --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🧬️test-app-mutations/🎚️config/🧬️mutations/📝️change-test-config-selection/🦀️.rs:5:249
  |
5 | ...tError::new("expected change-test-config-selection",crate::store::TextSpan::at(1,1)))?; Ok(Self{selected:serde_json::from_str(valu...
  |                                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
help: remove the unnecessary path segments
  |
5 - impl OpText for ChangeTestConfigSelection { fn parse_op(line:&str)->Result<Self,crate::store::TextError>{let value=line.strip_prefix("change-test-config-selection ").ok_or_else(||crate::store::TextError::new("expected change-test-config-selection",crate::store::TextSpan::at(1,1)))?; Ok(Self{selected:serde_json::from_str(value).map_err(|_|crate::store::TextError::new("selection must be a JSON nullable string",crate::store::TextSpan::at(1,1)))?})} fn print_op(&self)->String{format!("{} {}",Self::OPCODE,serde_json::to_string(&self.selected).expect("nullable string serializes"))} }
5 + impl OpText for ChangeTestConfigSelection { fn parse_op(line:&str)->Result<Self,crate::store::TextError>{let value=line.strip_prefix("change-test-config-selection ").ok_or_else(||crate::store::TextError::new("expected change-test-config-selection",store::TextSpan::at(1,1)))?; Ok(Self{selected:serde_json::from_str(value).map_err(|_|crate::store::TextError::new("selection must be a JSON nullable string",crate::store::TextSpan::at(1,1)))?})} fn print_op(&self)->String{format!("{} {}",Self::OPCODE,serde_json::to_string(&self.selected).expect("nullable string serializes"))} }
  |

warning: unnecessary qualification
 --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🧬️test-app-mutations/🎚️config/🧬️mutations/📝️change-test-config-selection/🦀️.rs:5:341
  |
5 | ...lf{selected:serde_json::from_str(value).map_err(|_|crate::store::TextError::new("selection must be a JSON nullable string",crate::...
  |                                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
help: remove the unnecessary path segments
  |
5 - impl OpText for ChangeTestConfigSelection { fn parse_op(line:&str)->Result<Self,crate::store::TextError>{let value=line.strip_prefix("change-test-config-selection ").ok_or_else(||crate::store::TextError::new("expected change-test-config-selection",crate::store::TextSpan::at(1,1)))?; Ok(Self{selected:serde_json::from_str(value).map_err(|_|crate::store::TextError::new("selection must be a JSON nullable string",crate::store::TextSpan::at(1,1)))?})} fn print_op(&self)->String{format!("{} {}",Self::OPCODE,serde_json::to_string(&self.selected).expect("nullable string serializes"))} }
5 + impl OpText for ChangeTestConfigSelection { fn parse_op(line:&str)->Result<Self,crate::store::TextError>{let value=line.strip_prefix("change-test-config-selection ").ok_or_else(||crate::store::TextError::new("expected change-test-config-selection",crate::store::TextSpan::at(1,1)))?; Ok(Self{selected:serde_json::from_str(value).map_err(|_|store::TextError::new("selection must be a JSON nullable string",crate::store::TextSpan::at(1,1)))?})} fn print_op(&self)->String{format!("{} {}",Self::OPCODE,serde_json::to_string(&self.selected).expect("nullable string serializes"))} }
  |

warning: unnecessary qualification
 --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🧬️test-app-mutations/🎚️config/🧬️mutations/📝️change-test-config-selection/🦀️.rs:5:413
  |
5 | ...ror::new("selection must be a JSON nullable string",crate::store::TextSpan::at(1,1)))?})} fn print_op(&self)->String{format!("{} {...
  |                                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
help: remove the unnecessary path segments
  |
5 - impl OpText for ChangeTestConfigSelection { fn parse_op(line:&str)->Result<Self,crate::store::TextError>{let value=line.strip_prefix("change-test-config-selection ").ok_or_else(||crate::store::TextError::new("expected change-test-config-selection",crate::store::TextSpan::at(1,1)))?; Ok(Self{selected:serde_json::from_str(value).map_err(|_|crate::store::TextError::new("selection must be a JSON nullable string",crate::store::TextSpan::at(1,1)))?})} fn print_op(&self)->String{format!("{} {}",Self::OPCODE,serde_json::to_string(&self.selected).expect("nullable string serializes"))} }
5 + impl OpText for ChangeTestConfigSelection { fn parse_op(line:&str)->Result<Self,crate::store::TextError>{let value=line.strip_prefix("change-test-config-selection ").ok_or_else(||crate::store::TextError::new("expected change-test-config-selection",crate::store::TextSpan::at(1,1)))?; Ok(Self{selected:serde_json::from_str(value).map_err(|_|crate::store::TextError::new("selection must be a JSON nullable string",store::TextSpan::at(1,1)))?})} fn print_op(&self)->String{format!("{} {}",Self::OPCODE,serde_json::to_string(&self.selected).expect("nullable string serializes"))} }
  |

warning: unnecessary qualification
 --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🧬️test-app-mutations/🎚️config/🧬️mutations/🦀️.rs:4:84
  |
4 | impl protocol::OpText for TestConfigMutation { fn parse_op(line:&str)->Result<Self,crate::store::TextError>{Ok(ChangeTestConfigSelect...
  |                                                                                    ^^^^^^^^^^^^^^^^^^^^^^^
  |
help: remove the unnecessary path segments
  |
4 - impl protocol::OpText for TestConfigMutation { fn parse_op(line:&str)->Result<Self,crate::store::TextError>{Ok(ChangeTestConfigSelection::parse_op(line)?.into())} fn print_op(&self)->String{match self{Self::ChangeTestConfigSelection(value)=>value.print_op()}} }
4 + impl protocol::OpText for TestConfigMutation { fn parse_op(line:&str)->Result<Self,store::TextError>{Ok(ChangeTestConfigSelection::parse_op(line)?.into())} fn print_op(&self)->String{match self{Self::ChangeTestConfigSelection(value)=>value.print_op()}} }
  |

warning: unused import: `TestConfigDiff`
 --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🧬️test-app-mutations/🦀️.rs:3:64
  |
3 | pub(crate) use config::{ChangeTestConfigSelection, TestConfig, TestConfigDiff, TestConfigMutation};
  |                                                                ^^^^^^^^^^^^^^

warning: unnecessary qualification
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🧬️test-app-mutations/🧬️document/🧬️mutations/🦀️.rs:13:41
   |
13 |     fn parse_op(line:&str)->Result<Self,crate::store::TextError>{
   |                                         ^^^^^^^^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
13 -     fn parse_op(line:&str)->Result<Self,crate::store::TextError>{
13 +     fn parse_op(line:&str)->Result<Self,store::TextError>{
   |

warning: unnecessary qualification
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🧬️contributed-mutation-wire/🦀️.rs:12:6
   |
12 | impl crate::store::ArtifactPack for WireTestSnapshot {
   |      ^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
12 - impl crate::store::ArtifactPack for WireTestSnapshot {
12 + impl store::ArtifactPack for WireTestSnapshot {
   |

warning: unnecessary qualification
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🧬️contributed-mutation-wire/🦀️.rs:13:43
   |
13 |     fn encode_pack_with(&self, _options: &crate::store::PackEncodeOptions) -> Result<Vec<u8>, crate::store::PackError> {
   |                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
13 -     fn encode_pack_with(&self, _options: &crate::store::PackEncodeOptions) -> Result<Vec<u8>, crate::store::PackError> {
13 +     fn encode_pack_with(&self, _options: &store::PackEncodeOptions) -> Result<Vec<u8>, crate::store::PackError> {
   |

warning: unnecessary qualification
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🧬️contributed-mutation-wire/🦀️.rs:13:95
   |
13 |     fn encode_pack_with(&self, _options: &crate::store::PackEncodeOptions) -> Result<Vec<u8>, crate::store::PackError> {
   |                                                                                               ^^^^^^^^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
13 -     fn encode_pack_with(&self, _options: &crate::store::PackEncodeOptions) -> Result<Vec<u8>, crate::store::PackError> {
13 +     fn encode_pack_with(&self, _options: &crate::store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
   |

warning: unnecessary qualification
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🧬️contributed-mutation-wire/🦀️.rs:14:50
   |
14 |         serde_json::to_vec(self).map_err(|error| crate::store::PackError::Schema(error.to_string()))
   |                                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
14 -         serde_json::to_vec(self).map_err(|error| crate::store::PackError::Schema(error.to_string()))
14 +         serde_json::to_vec(self).map_err(|error| store::PackError::Schema(error.to_string()))
   |

warning: unnecessary qualification
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🧬️contributed-mutation-wire/🦀️.rs:17:50
   |
17 |     fn decode_pack_with(bytes: &[u8], _options: &crate::store::PackDecodeOptions) -> Result<Self, crate::store::PackError> {
   |                                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
17 -     fn decode_pack_with(bytes: &[u8], _options: &crate::store::PackDecodeOptions) -> Result<Self, crate::store::PackError> {
17 +     fn decode_pack_with(bytes: &[u8], _options: &store::PackDecodeOptions) -> Result<Self, crate::store::PackError> {
   |

warning: unnecessary qualification
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🧬️contributed-mutation-wire/🦀️.rs:17:99
   |
17 |     fn decode_pack_with(bytes: &[u8], _options: &crate::store::PackDecodeOptions) -> Result<Self, crate::store::PackError> {
   |                                                                                                   ^^^^^^^^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
17 -     fn decode_pack_with(bytes: &[u8], _options: &crate::store::PackDecodeOptions) -> Result<Self, crate::store::PackError> {
17 +     fn decode_pack_with(bytes: &[u8], _options: &crate::store::PackDecodeOptions) -> Result<Self, store::PackError> {
   |

warning: unnecessary qualification
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🧬️contributed-mutation-wire/🦀️.rs:18:55
   |
18 |         serde_json::from_slice(bytes).map_err(|error| crate::store::PackError::Schema(error.to_string()))
   |                                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
18 -         serde_json::from_slice(bytes).map_err(|error| crate::store::PackError::Schema(error.to_string()))
18 +         serde_json::from_slice(bytes).map_err(|error| store::PackError::Schema(error.to_string()))
   |

warning: unnecessary qualification
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🧬️contributed-mutation-wire/🧬️mutations/🦀️.rs:20:50
   |
20 |         serde_json::to_vec(self).map_err(|error| crate::store::PackError::Schema(error.to_string()).into())
   |                                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
20 -         serde_json::to_vec(self).map_err(|error| crate::store::PackError::Schema(error.to_string()).into())
20 +         serde_json::to_vec(self).map_err(|error| store::PackError::Schema(error.to_string()).into())
   |

warning: unnecessary qualification
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🧬️contributed-mutation-wire/🧬️mutations/🦀️.rs:24:55
   |
24 |         serde_json::from_slice(bytes).map_err(|error| crate::store::PackError::Schema(error.to_string()).into())
   |                                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
24 -         serde_json::from_slice(bytes).map_err(|error| crate::store::PackError::Schema(error.to_string()).into())
24 +         serde_json::from_slice(bytes).map_err(|error| store::PackError::Schema(error.to_string()).into())
   |

warning: unused doc comment
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:11776:5
      |
11776 | /     /// 🗃️ sdk-dedyn (O1/§1.5): the default-composes-nothing case (design-dedyn.md §1.6's `NoMembers`
11777 | |     /// pattern, applied here to `PluginApp`) — a zero-variant enum, `dyn_enum_close!`-generated
11778 | |     /// (every method's body degenerates to `match *self {}` since there is no value to construct).
11779 | |     /// The default `PA` for every generic in this file's declaration tree, so a library-only plugin
11780 | |     /// (or a test that never actually instantiates an app) never has to name a real app enum.
      | |_____-----------------------------------------------------------------------------------------^
      |       |
      |       rustdoc does not generate documentation for macro invocations
      |
      = help: to document an item produced by a macro, the macro must produce the documentation as part of its expansion
      = note: `#[warn(unused_doc_comments)]` (part of `#[warn(unused)]`) on by default

warning: unused imports: `PresentCx` and `Present`
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:266:38
    |
266 |     use semio_framework_ui_runtime::{Present, PresentCx};
    |                                      ^^^^^^^  ^^^^^^^^^

warning: unused import: `std::sync::Arc`
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:274:9
    |
274 |     use std::sync::Arc;
    |         ^^^^^^^^^^^^^^

warning: unused import: `ui_wgpu::wgpu::UiMenuRef`
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:296:9
    |
296 |     use ui_wgpu::wgpu::UiMenuRef;
    |         ^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused imports: `ContextMenuPoint` and `ContextMenuResponse`
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:303:67
    |
303 | ...   collect_window_kind_ids_from_layout, ContextMenuItemSpec, ContextMenuPoint, ContextMenuRequest, ContextMenuResponse, ContextM...
    |                                                                 ^^^^^^^^^^^^^^^^                      ^^^^^^^^^^^^^^^^^^^

warning: unnecessary qualification
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:322:9
    |
322 | ...   ui::surface(props).try_id(id).map_err(|_| ui_assembly_error("scene-surface.id"))?.try_build().map_err(|_| ui_assembly_error("...
    |       ^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
322 -         ui::surface(props).try_id(id).map_err(|_| ui_assembly_error("scene-surface.id"))?.try_build().map_err(|_| ui_assembly_error("scene-surface.build"))
322 +         surface(props).try_id(id).map_err(|_| ui_assembly_error("scene-surface.id"))?.try_build().map_err(|_| ui_assembly_error("scene-surface.build"))
    |

warning: unnecessary qualification
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:329:9
    |
329 |         ui::text(value).try_build().map_err(|_| label)
    |         ^^^^^^^^
    |
help: remove the unnecessary path segments
    |
329 -         ui::text(value).try_build().map_err(|_| label)
329 +         text(value).try_build().map_err(|_| label)
    |

warning: unnecessary qualification
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:359:92
    |
359 | ...ee(ui_wgpu::wgpu::Label::data("x".repeat(semio_framework_ui_contract::UI_TEXT_MAX_BYTES + 1))).expect_err("oversized dynamic tex...
    |                                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
359 -             let error = built_text_to_component_tree(ui_wgpu::wgpu::Label::data("x".repeat(semio_framework_ui_contract::UI_TEXT_MAX_BYTES + 1))).expect_err("oversized dynamic text must not panic or truncate");
359 +             let error = built_text_to_component_tree(ui_wgpu::wgpu::Label::data("x".repeat(UI_TEXT_MAX_BYTES + 1))).expect_err("oversized dynamic text must not panic or truncate");
    |

warning: unnecessary qualification
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:465:49
    |
465 |     async fn ui_tree_domain_topology(sections: &ui::BuiltChildren, granularity: &str) -> UiAssemblyResult<protocol::DomainTopology> {
    |                                                 ^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
465 -     async fn ui_tree_domain_topology(sections: &ui::BuiltChildren, granularity: &str) -> UiAssemblyResult<protocol::DomainTopology> {
465 +     async fn ui_tree_domain_topology(sections: &BuiltChildren, granularity: &str) -> UiAssemblyResult<protocol::DomainTopology> {
    |

warning: unnecessary qualification
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:467:27
    |
467 |             children: &'a ui::BuiltChildren,
    |                           ^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
467 -             children: &'a ui::BuiltChildren,
467 +             children: &'a BuiltChildren,
    |

warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:5844:27
     |
5844 |             let builder = ui::tree_section(label.unwrap_or_default()).default_open(default_open);
     |                           ^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
5844 -             let builder = ui::tree_section(label.unwrap_or_default()).default_open(default_open);
5844 +             let builder = tree_section(label.unwrap_or_default()).default_open(default_open);
     |

warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:5897:27
     |
5897 |             let builder = ui::tree().try_id(&self.namespace).map_err(|_| ui_assembly_error("panel-tree.root-id"))?;
     |                           ^^^^^^^^
     |
help: remove the unnecessary path segments
     |
5897 -             let builder = ui::tree().try_id(&self.namespace).map_err(|_| ui_assembly_error("panel-tree.root-id"))?;
5897 +             let builder = tree().try_id(&self.namespace).map_err(|_| ui_assembly_error("panel-tree.root-id"))?;
     |

warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:5994:27
     |
5994 |             let builder = ui::field(ui_label(label, "form-panel.field-label")?);
     |                           ^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
5994 -             let builder = ui::field(ui_label(label, "form-panel.field-label")?);
5994 +             let builder = field(ui_label(label, "form-panel.field-label")?);
     |

warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:6019:31
     |
6019 |                 let builder = ui::input(InputKind::Text).value(value);
     |                               ^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
6019 -                 let builder = ui::input(InputKind::Text).value(value);
6019 +                 let builder = input(InputKind::Text).value(value);
     |

warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:6034:27
     |
6034 |             let builder = ui::button(ui_label(label, "form-panel.submit-label")?).icon(icon);
     |                           ^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
6034 -             let builder = ui::button(ui_label(label, "form-panel.submit-label")?).icon(icon);
6034 +             let builder = button(ui_label(label, "form-panel.submit-label")?).icon(icon);
     |

warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:6049:27
     |
6049 |             let builder = ui::column().try_id(&self.namespace).map_err(|_| ui_assembly_error("form-panel.root-id"))?;
     |                           ^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
6049 -             let builder = ui::column().try_id(&self.namespace).map_err(|_| ui_assembly_error("form-panel.root-id"))?;
6049 +             let builder = column().try_id(&self.namespace).map_err(|_| ui_assembly_error("form-panel.root-id"))?;
     |

warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:6065:21
     |
6065 | ...   let title = ui::text(ui_label(title, "entity-detail.title")?).try_build().map_err(|_| ui_assembly_error("entity-detail.title...
     |                   ^^^^^^^^
     |
help: remove the unnecessary path segments
     |
6065 -         let title = ui::text(ui_label(title, "entity-detail.title")?).try_build().map_err(|_| ui_assembly_error("entity-detail.title-build"))?;
6065 +         let title = text(ui_label(title, "entity-detail.title")?).try_build().map_err(|_| ui_assembly_error("entity-detail.title-build"))?;
     |

warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:6068:28
     |
6068 |             let subtitle = ui::text(subtitle).try_build().map_err(|_| ui_assembly_error("entity-detail.subtitle-build"))?;
     |                            ^^^^^^^^
     |
help: remove the unnecessary path segments
     |
6068 -             let subtitle = ui::text(subtitle).try_build().map_err(|_| ui_assembly_error("entity-detail.subtitle-build"))?;
6068 +             let subtitle = text(subtitle).try_build().map_err(|_| ui_assembly_error("entity-detail.subtitle-build"))?;
     |

warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:6080:23
     |
6080 |         let builder = ui::column().try_children(children).map_err(|_| ui_assembly_error("entity-detail.children"))?;
     |                       ^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
6080 -         let builder = ui::column().try_children(children).map_err(|_| ui_assembly_error("entity-detail.children"))?;
6080 +         let builder = column().try_children(children).map_err(|_| ui_assembly_error("entity-detail.children"))?;
     |

warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:6092:27
     |
6092 |             let control = ui::input(InputKind::Text)
     |                           ^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
6092 -             let control = ui::input(InputKind::Text)
6092 +             let control = input(InputKind::Text)
     |

warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:6135:26
     |
6135 |             let button = ui::button(Label::try_from("Edit").expect("bounded fixture"))
     |                          ^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
6135 -             let button = ui::button(Label::try_from("Edit").expect("bounded fixture"))
6135 +             let button = button(Label::try_from("Edit").expect("bounded fixture"))
     |

warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:6295:24
     |
6295 |         List { cursor: ui::UiListCursor, output: Vec<Value> },
     |                        ^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
6295 -         List { cursor: ui::UiListCursor, output: Vec<Value> },
6295 +         List { cursor: UiListCursor, output: Vec<Value> },
     |

warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:6296:23
     |
6296 |         Map { cursor: ui::UiMapCursor, output: serde_json::Map<String, Value>, key: Option<String> },
     |                       ^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
6296 -         Map { cursor: ui::UiMapCursor, output: serde_json::Map<String, Value>, key: Option<String> },
6296 +         Map { cursor: UiMapCursor, output: serde_json::Map<String, Value>, key: Option<String> },
     |

warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:6499:36
     |
6499 |         fn ui_text(value: &str) -> ui::UiText {
     |                                    ^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
6499 -         fn ui_text(value: &str) -> ui::UiText {
6499 +         fn ui_text(value: &str) -> UiText {
     |

warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:6500:13
     |
6500 |             ui::UiText::try_from_str(value).expect("bounded fixture text")
     |             ^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
6500 -             ui::UiText::try_from_str(value).expect("bounded fixture text")
6500 +             UiText::try_from_str(value).expect("bounded fixture text")
     |

warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:6503:66
     |
6503 |         fn ui_list(values: impl IntoIterator<Item = UiValue>) -> ui::UiList {
     |                                                                  ^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
6503 -         fn ui_list(values: impl IntoIterator<Item = UiValue>) -> ui::UiList {
6503 +         fn ui_list(values: impl IntoIterator<Item = UiValue>) -> UiList {
     |

warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:6504:31
     |
6504 |             let mut builder = ui::UiListBuilder::try_new().expect("fixed list builder");
     |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
6504 -             let mut builder = ui::UiListBuilder::try_new().expect("fixed list builder");
6504 +             let mut builder = UiListBuilder::try_new().expect("fixed list builder");
     |

warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:6511:76
     |
6511 |         fn ui_map(entries: impl IntoIterator<Item = (String, UiValue)>) -> ui::UiMap {
     |                                                                            ^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
6511 -         fn ui_map(entries: impl IntoIterator<Item = (String, UiValue)>) -> ui::UiMap {
6511 +         fn ui_map(entries: impl IntoIterator<Item = (String, UiValue)>) -> UiMap {
     |

warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:6512:31
     |
6512 |             let mut builder = ui::UiMapBuilder::try_new().expect("fixed map builder");
     |                               ^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
6512 -             let mut builder = ui::UiMapBuilder::try_new().expect("fixed map builder");
6512 +             let mut builder = UiMapBuilder::try_new().expect("fixed map builder");
     |

warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:6526:27
     |
6526 |             let mut map = std::collections::BTreeMap::new();
     |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
6526 -             let mut map = std::collections::BTreeMap::new();
6526 +             let mut map = BTreeMap::new();
     |

warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:6545:32
     |
6545 |             let mut args_map = std::collections::BTreeMap::new();
     |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
6545 -             let mut args_map = std::collections::BTreeMap::new();
6545 +             let mut args_map = BTreeMap::new();
     |

warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:6548:33
     |
6548 |             let mut input_map = std::collections::BTreeMap::new();
     |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
6548 -             let mut input_map = std::collections::BTreeMap::new();
6548 +             let mut input_map = BTreeMap::new();
     |

warning: unnecessary qualification
 --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././🧪️tests/🧬️mutation-fixtures/🎲️dummy/🧬️mutations/📝️set-dummy-count/🦀️.rs:9:73
  |
9 | impl OpText for SetDummyCount { fn parse_op(line: &str) -> Result<Self, crate::store::TextError> { let value = line.strip_prefix("set...
  |                                                                         ^^^^^^^^^^^^^^^^^^^^^^^
  |
help: remove the unnecessary path segments
  |
9 - impl OpText for SetDummyCount { fn parse_op(line: &str) -> Result<Self, crate::store::TextError> { let value = line.strip_prefix("set-dummy-count ").ok_or_else(|| crate::store::TextError::new("expected set-dummy-count", crate::store::TextSpan::at(1, 1)))?.parse().map_err(|_| crate::store::TextError::new("dummy count must be i32", crate::store::TextSpan::at(1, 1)))?; Ok(Self { value }) } fn print_op(&self) -> String { format!("{} {}", Self::OPCODE, self.value) } }
9 + impl OpText for SetDummyCount { fn parse_op(line: &str) -> Result<Self, store::TextError> { let value = line.strip_prefix("set-dummy-count ").ok_or_else(|| crate::store::TextError::new("expected set-dummy-count", crate::store::TextSpan::at(1, 1)))?.parse().map_err(|_| crate::store::TextError::new("dummy count must be i32", crate::store::TextSpan::at(1, 1)))?; Ok(Self { value }) } fn print_op(&self) -> String { format!("{} {}", Self::OPCODE, self.value) } }
  |

warning: unnecessary qualification
 --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././🧪️tests/🧬️mutation-fixtures/🎲️dummy/🧬️mutations/📝️set-dummy-count/🦀️.rs:9:164
  |
9 | ...ine.strip_prefix("set-dummy-count ").ok_or_else(|| crate::store::TextError::new("expected set-dummy-count", crate::store::TextSpan...
  |                                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
help: remove the unnecessary path segments
  |
9 - impl OpText for SetDummyCount { fn parse_op(line: &str) -> Result<Self, crate::store::TextError> { let value = line.strip_prefix("set-dummy-count ").ok_or_else(|| crate::store::TextError::new("expected set-dummy-count", crate::store::TextSpan::at(1, 1)))?.parse().map_err(|_| crate::store::TextError::new("dummy count must be i32", crate::store::TextSpan::at(1, 1)))?; Ok(Self { value }) } fn print_op(&self) -> String { format!("{} {}", Self::OPCODE, self.value) } }
9 + impl OpText for SetDummyCount { fn parse_op(line: &str) -> Result<Self, crate::store::TextError> { let value = line.strip_prefix("set-dummy-count ").ok_or_else(|| store::TextError::new("expected set-dummy-count", crate::store::TextSpan::at(1, 1)))?.parse().map_err(|_| crate::store::TextError::new("dummy count must be i32", crate::store::TextSpan::at(1, 1)))?; Ok(Self { value }) } fn print_op(&self) -> String { format!("{} {}", Self::OPCODE, self.value) } }
  |

warning: unnecessary qualification
 --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././🧪️tests/🧬️mutation-fixtures/🎲️dummy/🧬️mutations/📝️set-dummy-count/🦀️.rs:9:221
  |
9 | ...::store::TextError::new("expected set-dummy-count", crate::store::TextSpan::at(1, 1)))?.parse().map_err(|_| crate::store::TextErro...
  |                                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
help: remove the unnecessary path segments
  |
9 - impl OpText for SetDummyCount { fn parse_op(line: &str) -> Result<Self, crate::store::TextError> { let value = line.strip_prefix("set-dummy-count ").ok_or_else(|| crate::store::TextError::new("expected set-dummy-count", crate::store::TextSpan::at(1, 1)))?.parse().map_err(|_| crate::store::TextError::new("dummy count must be i32", crate::store::TextSpan::at(1, 1)))?; Ok(Self { value }) } fn print_op(&self) -> String { format!("{} {}", Self::OPCODE, self.value) } }
9 + impl OpText for SetDummyCount { fn parse_op(line: &str) -> Result<Self, crate::store::TextError> { let value = line.strip_prefix("set-dummy-count ").ok_or_else(|| crate::store::TextError::new("expected set-dummy-count", store::TextSpan::at(1, 1)))?.parse().map_err(|_| crate::store::TextError::new("dummy count must be i32", crate::store::TextSpan::at(1, 1)))?; Ok(Self { value }) } fn print_op(&self) -> String { format!("{} {}", Self::OPCODE, self.value) } }
  |

warning: unnecessary qualification
 --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././🧪️tests/🧬️mutation-fixtures/🎲️dummy/🧬️mutations/📝️set-dummy-count/🦀️.rs:9:277
  |
9 | ...::store::TextSpan::at(1, 1)))?.parse().map_err(|_| crate::store::TextError::new("dummy count must be i32", crate::store::TextSpan:...
  |                                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
help: remove the unnecessary path segments
  |
9 - impl OpText for SetDummyCount { fn parse_op(line: &str) -> Result<Self, crate::store::TextError> { let value = line.strip_prefix("set-dummy-count ").ok_or_else(|| crate::store::TextError::new("expected set-dummy-count", crate::store::TextSpan::at(1, 1)))?.parse().map_err(|_| crate::store::TextError::new("dummy count must be i32", crate::store::TextSpan::at(1, 1)))?; Ok(Self { value }) } fn print_op(&self) -> String { format!("{} {}", Self::OPCODE, self.value) } }
9 + impl OpText for SetDummyCount { fn parse_op(line: &str) -> Result<Self, crate::store::TextError> { let value = line.strip_prefix("set-dummy-count ").ok_or_else(|| crate::store::TextError::new("expected set-dummy-count", crate::store::TextSpan::at(1, 1)))?.parse().map_err(|_| store::TextError::new("dummy count must be i32", crate::store::TextSpan::at(1, 1)))?; Ok(Self { value }) } fn print_op(&self) -> String { format!("{} {}", Self::OPCODE, self.value) } }
  |

warning: unnecessary qualification
 --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././🧪️tests/🧬️mutation-fixtures/🎲️dummy/🧬️mutations/📝️set-dummy-count/🦀️.rs:9:333
  |
9 | ...e::store::TextError::new("dummy count must be i32", crate::store::TextSpan::at(1, 1)))?; Ok(Self { value }) } fn print_op(&self) -...
  |                                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
help: remove the unnecessary path segments
  |
9 - impl OpText for SetDummyCount { fn parse_op(line: &str) -> Result<Self, crate::store::TextError> { let value = line.strip_prefix("set-dummy-count ").ok_or_else(|| crate::store::TextError::new("expected set-dummy-count", crate::store::TextSpan::at(1, 1)))?.parse().map_err(|_| crate::store::TextError::new("dummy count must be i32", crate::store::TextSpan::at(1, 1)))?; Ok(Self { value }) } fn print_op(&self) -> String { format!("{} {}", Self::OPCODE, self.value) } }
9 + impl OpText for SetDummyCount { fn parse_op(line: &str) -> Result<Self, crate::store::TextError> { let value = line.strip_prefix("set-dummy-count ").ok_or_else(|| crate::store::TextError::new("expected set-dummy-count", crate::store::TextSpan::at(1, 1)))?.parse().map_err(|_| crate::store::TextError::new("dummy count must be i32", store::TextSpan::at(1, 1)))?; Ok(Self { value }) } fn print_op(&self) -> String { format!("{} {}", Self::OPCODE, self.value) } }
  |

warning: unnecessary qualification
 --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././🧪️tests/🧬️mutation-fixtures/🎲️dummy/🧬️mutations/🦀️.rs:9:83
  |
9 | impl protocol::OpText for DummyMutation { fn parse_op(line: &str) -> Result<Self, crate::store::TextError> { Ok(SetDummyCount::parse_...
  |                                                                                   ^^^^^^^^^^^^^^^^^^^^^^^
  |
help: remove the unnecessary path segments
  |
9 - impl protocol::OpText for DummyMutation { fn parse_op(line: &str) -> Result<Self, crate::store::TextError> { Ok(SetDummyCount::parse_op(line)?.into()) } fn print_op(&self) -> String { match self { Self::SetDummyCount(value) => value.print_op() } } }
9 + impl protocol::OpText for DummyMutation { fn parse_op(line: &str) -> Result<Self, store::TextError> { Ok(SetDummyCount::parse_op(line)?.into()) } fn print_op(&self) -> String { match self { Self::SetDummyCount(value) => value.print_op() } } }
  |

warning: unused import: `Mutation`
 --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././🧪️tests/🧬️mutation-fixtures/🎲️dummy/🦀️.rs:9:16
  |
9 | use protocol::{Mutation, MutationDiff};
  |                ^^^^^^^^

warning: unnecessary qualification
 --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././🧪️tests/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/📝️set-transaction-count/🦀️.rs:4:79
  |
4 | impl OpText for SetTransactionCount { fn parse_op(line: &str) -> Result<Self, crate::store::TextError> { Ok(Self { value: line.strip_...
  |                                                                               ^^^^^^^^^^^^^^^^^^^^^^^
  |
help: remove the unnecessary path segments
  |
4 - impl OpText for SetTransactionCount { fn parse_op(line: &str) -> Result<Self, crate::store::TextError> { Ok(Self { value: line.strip_prefix("set-transaction-count ").ok_or_else(|| crate::store::TextError::new("expected set-transaction-count", crate::store::TextSpan::at(1, 1)))?.parse().map_err(|_| crate::store::TextError::new("transaction count must be i32", crate::store::TextSpan::at(1, 1)))? }) } fn print_op(&self) -> String { format!("{} {}", Self::OPCODE, self.value) } }
4 + impl OpText for SetTransactionCount { fn parse_op(line: &str) -> Result<Self, store::TextError> { Ok(Self { value: line.strip_prefix("set-transaction-count ").ok_or_else(|| crate::store::TextError::new("expected set-transaction-count", crate::store::TextSpan::at(1, 1)))?.parse().map_err(|_| crate::store::TextError::new("transaction count must be i32", crate::store::TextSpan::at(1, 1)))? }) } fn print_op(&self) -> String { format!("{} {}", Self::OPCODE, self.value) } }
  |

warning: unnecessary qualification
 --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././🧪️tests/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/📝️set-transaction-count/🦀️.rs:4:181
  |
4 | ...rip_prefix("set-transaction-count ").ok_or_else(|| crate::store::TextError::new("expected set-transaction-count", crate::store::Te...
  |                                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
help: remove the unnecessary path segments
  |
4 - impl OpText for SetTransactionCount { fn parse_op(line: &str) -> Result<Self, crate::store::TextError> { Ok(Self { value: line.strip_prefix("set-transaction-count ").ok_or_else(|| crate::store::TextError::new("expected set-transaction-count", crate::store::TextSpan::at(1, 1)))?.parse().map_err(|_| crate::store::TextError::new("transaction count must be i32", crate::store::TextSpan::at(1, 1)))? }) } fn print_op(&self) -> String { format!("{} {}", Self::OPCODE, self.value) } }
4 + impl OpText for SetTransactionCount { fn parse_op(line: &str) -> Result<Self, crate::store::TextError> { Ok(Self { value: line.strip_prefix("set-transaction-count ").ok_or_else(|| store::TextError::new("expected set-transaction-count", crate::store::TextSpan::at(1, 1)))?.parse().map_err(|_| crate::store::TextError::new("transaction count must be i32", crate::store::TextSpan::at(1, 1)))? }) } fn print_op(&self) -> String { format!("{} {}", Self::OPCODE, self.value) } }
  |

warning: unnecessary qualification
 --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././🧪️tests/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/📝️set-transaction-count/🦀️.rs:4:244
  |
4 | ...e::TextError::new("expected set-transaction-count", crate::store::TextSpan::at(1, 1)))?.parse().map_err(|_| crate::store::TextErro...
  |                                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
help: remove the unnecessary path segments
  |
4 - impl OpText for SetTransactionCount { fn parse_op(line: &str) -> Result<Self, crate::store::TextError> { Ok(Self { value: line.strip_prefix("set-transaction-count ").ok_or_else(|| crate::store::TextError::new("expected set-transaction-count", crate::store::TextSpan::at(1, 1)))?.parse().map_err(|_| crate::store::TextError::new("transaction count must be i32", crate::store::TextSpan::at(1, 1)))? }) } fn print_op(&self) -> String { format!("{} {}", Self::OPCODE, self.value) } }
4 + impl OpText for SetTransactionCount { fn parse_op(line: &str) -> Result<Self, crate::store::TextError> { Ok(Self { value: line.strip_prefix("set-transaction-count ").ok_or_else(|| crate::store::TextError::new("expected set-transaction-count", store::TextSpan::at(1, 1)))?.parse().map_err(|_| crate::store::TextError::new("transaction count must be i32", crate::store::TextSpan::at(1, 1)))? }) } fn print_op(&self) -> String { format!("{} {}", Self::OPCODE, self.value) } }
  |

warning: unnecessary qualification
 --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././🧪️tests/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/📝️set-transaction-count/🦀️.rs:4:300
  |
4 | ...::store::TextSpan::at(1, 1)))?.parse().map_err(|_| crate::store::TextError::new("transaction count must be i32", crate::store::Tex...
  |                                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
help: remove the unnecessary path segments
  |
4 - impl OpText for SetTransactionCount { fn parse_op(line: &str) -> Result<Self, crate::store::TextError> { Ok(Self { value: line.strip_prefix("set-transaction-count ").ok_or_else(|| crate::store::TextError::new("expected set-transaction-count", crate::store::TextSpan::at(1, 1)))?.parse().map_err(|_| crate::store::TextError::new("transaction count must be i32", crate::store::TextSpan::at(1, 1)))? }) } fn print_op(&self) -> String { format!("{} {}", Self::OPCODE, self.value) } }
4 + impl OpText for SetTransactionCount { fn parse_op(line: &str) -> Result<Self, crate::store::TextError> { Ok(Self { value: line.strip_prefix("set-transaction-count ").ok_or_else(|| crate::store::TextError::new("expected set-transaction-count", crate::store::TextSpan::at(1, 1)))?.parse().map_err(|_| store::TextError::new("transaction count must be i32", crate::store::TextSpan::at(1, 1)))? }) } fn print_op(&self) -> String { format!("{} {}", Self::OPCODE, self.value) } }
  |

warning: unnecessary qualification
 --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././🧪️tests/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/📝️set-transaction-count/🦀️.rs:4:362
  |
4 | ...re::TextError::new("transaction count must be i32", crate::store::TextSpan::at(1, 1)))? }) } fn print_op(&self) -> String { format...
  |                                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
help: remove the unnecessary path segments
  |
4 - impl OpText for SetTransactionCount { fn parse_op(line: &str) -> Result<Self, crate::store::TextError> { Ok(Self { value: line.strip_prefix("set-transaction-count ").ok_or_else(|| crate::store::TextError::new("expected set-transaction-count", crate::store::TextSpan::at(1, 1)))?.parse().map_err(|_| crate::store::TextError::new("transaction count must be i32", crate::store::TextSpan::at(1, 1)))? }) } fn print_op(&self) -> String { format!("{} {}", Self::OPCODE, self.value) } }
4 + impl OpText for SetTransactionCount { fn parse_op(line: &str) -> Result<Self, crate::store::TextError> { Ok(Self { value: line.strip_prefix("set-transaction-count ").ok_or_else(|| crate::store::TextError::new("expected set-transaction-count", crate::store::TextSpan::at(1, 1)))?.parse().map_err(|_| crate::store::TextError::new("transaction count must be i32", store::TextSpan::at(1, 1)))? }) } fn print_op(&self) -> String { format!("{} {}", Self::OPCODE, self.value) } }
  |

warning: unnecessary qualification
 --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././🧪️tests/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/📝️set-transaction-count-without-preflight/🦀️.rs:4:91
  |
4 | impl OpText for SetTransactionCountWithoutPreflight { fn parse_op(line:&str)->Result<Self,crate::store::TextError>{Ok(Self{value:line...
  |                                                                                           ^^^^^^^^^^^^^^^^^^^^^^^
  |
help: remove the unnecessary path segments
  |
4 - impl OpText for SetTransactionCountWithoutPreflight { fn parse_op(line:&str)->Result<Self,crate::store::TextError>{Ok(Self{value:line.strip_prefix("set-transaction-count-without-preflight ").ok_or_else(||crate::store::TextError::new("expected set-transaction-count-without-preflight",crate::store::TextSpan::at(1,1)))?.parse().map_err(|_|crate::store::TextError::new("transaction count must be i32",crate::store::TextSpan::at(1,1)))?})} fn print_op(&self)->String{format!("{} {}",Self::OPCODE,self.value)} }
4 + impl OpText for SetTransactionCountWithoutPreflight { fn parse_op(line:&str)->Result<Self,store::TextError>{Ok(Self{value:line.strip_prefix("set-transaction-count-without-preflight ").ok_or_else(||crate::store::TextError::new("expected set-transaction-count-without-preflight",crate::store::TextSpan::at(1,1)))?.parse().map_err(|_|crate::store::TextError::new("transaction count must be i32",crate::store::TextSpan::at(1,1)))?})} fn print_op(&self)->String{format!("{} {}",Self::OPCODE,self.value)} }
  |

warning: unnecessary qualification
 --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././🧪️tests/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/📝️set-transaction-count-without-preflight/🦀️.rs:4:205
  |
4 | ...ransaction-count-without-preflight ").ok_or_else(||crate::store::TextError::new("expected set-transaction-count-without-preflight"...
  |                                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
help: remove the unnecessary path segments
  |
4 - impl OpText for SetTransactionCountWithoutPreflight { fn parse_op(line:&str)->Result<Self,crate::store::TextError>{Ok(Self{value:line.strip_prefix("set-transaction-count-without-preflight ").ok_or_else(||crate::store::TextError::new("expected set-transaction-count-without-preflight",crate::store::TextSpan::at(1,1)))?.parse().map_err(|_|crate::store::TextError::new("transaction count must be i32",crate::store::TextSpan::at(1,1)))?})} fn print_op(&self)->String{format!("{} {}",Self::OPCODE,self.value)} }
4 + impl OpText for SetTransactionCountWithoutPreflight { fn parse_op(line:&str)->Result<Self,crate::store::TextError>{Ok(Self{value:line.strip_prefix("set-transaction-count-without-preflight ").ok_or_else(||store::TextError::new("expected set-transaction-count-without-preflight",crate::store::TextSpan::at(1,1)))?.parse().map_err(|_|crate::store::TextError::new("transaction count must be i32",crate::store::TextSpan::at(1,1)))?})} fn print_op(&self)->String{format!("{} {}",Self::OPCODE,self.value)} }
  |

warning: unnecessary qualification
 --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././🧪️tests/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/📝️set-transaction-count-without-preflight/🦀️.rs:4:285
  |
4 | ...("expected set-transaction-count-without-preflight",crate::store::TextSpan::at(1,1)))?.parse().map_err(|_|crate::store::TextError:...
  |                                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
help: remove the unnecessary path segments
  |
4 - impl OpText for SetTransactionCountWithoutPreflight { fn parse_op(line:&str)->Result<Self,crate::store::TextError>{Ok(Self{value:line.strip_prefix("set-transaction-count-without-preflight ").ok_or_else(||crate::store::TextError::new("expected set-transaction-count-without-preflight",crate::store::TextSpan::at(1,1)))?.parse().map_err(|_|crate::store::TextError::new("transaction count must be i32",crate::store::TextSpan::at(1,1)))?})} fn print_op(&self)->String{format!("{} {}",Self::OPCODE,self.value)} }
4 + impl OpText for SetTransactionCountWithoutPreflight { fn parse_op(line:&str)->Result<Self,crate::store::TextError>{Ok(Self{value:line.strip_prefix("set-transaction-count-without-preflight ").ok_or_else(||crate::store::TextError::new("expected set-transaction-count-without-preflight",store::TextSpan::at(1,1)))?.parse().map_err(|_|crate::store::TextError::new("transaction count must be i32",crate::store::TextSpan::at(1,1)))?})} fn print_op(&self)->String{format!("{} {}",Self::OPCODE,self.value)} }
  |

warning: unnecessary qualification
 --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././🧪️tests/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/📝️set-transaction-count-without-preflight/🦀️.rs:4:339
  |
4 | ...te::store::TextSpan::at(1,1)))?.parse().map_err(|_|crate::store::TextError::new("transaction count must be i32",crate::store::Text...
  |                                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
help: remove the unnecessary path segments
  |
4 - impl OpText for SetTransactionCountWithoutPreflight { fn parse_op(line:&str)->Result<Self,crate::store::TextError>{Ok(Self{value:line.strip_prefix("set-transaction-count-without-preflight ").ok_or_else(||crate::store::TextError::new("expected set-transaction-count-without-preflight",crate::store::TextSpan::at(1,1)))?.parse().map_err(|_|crate::store::TextError::new("transaction count must be i32",crate::store::TextSpan::at(1,1)))?})} fn print_op(&self)->String{format!("{} {}",Self::OPCODE,self.value)} }
4 + impl OpText for SetTransactionCountWithoutPreflight { fn parse_op(line:&str)->Result<Self,crate::store::TextError>{Ok(Self{value:line.strip_prefix("set-transaction-count-without-preflight ").ok_or_else(||crate::store::TextError::new("expected set-transaction-count-without-preflight",crate::store::TextSpan::at(1,1)))?.parse().map_err(|_|store::TextError::new("transaction count must be i32",crate::store::TextSpan::at(1,1)))?})} fn print_op(&self)->String{format!("{} {}",Self::OPCODE,self.value)} }
  |

warning: unnecessary qualification
 --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././🧪️tests/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/📝️set-transaction-count-without-preflight/🦀️.rs:4:400
  |
4 | ...ore::TextError::new("transaction count must be i32",crate::store::TextSpan::at(1,1)))?})} fn print_op(&self)->String{format!("{} {...
  |                                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
help: remove the unnecessary path segments
  |
4 - impl OpText for SetTransactionCountWithoutPreflight { fn parse_op(line:&str)->Result<Self,crate::store::TextError>{Ok(Self{value:line.strip_prefix("set-transaction-count-without-preflight ").ok_or_else(||crate::store::TextError::new("expected set-transaction-count-without-preflight",crate::store::TextSpan::at(1,1)))?.parse().map_err(|_|crate::store::TextError::new("transaction count must be i32",crate::store::TextSpan::at(1,1)))?})} fn print_op(&self)->String{format!("{} {}",Self::OPCODE,self.value)} }
4 + impl OpText for SetTransactionCountWithoutPreflight { fn parse_op(line:&str)->Result<Self,crate::store::TextError>{Ok(Self{value:line.strip_prefix("set-transaction-count-without-preflight ").ok_or_else(||crate::store::TextError::new("expected set-transaction-count-without-preflight",crate::store::TextSpan::at(1,1)))?.parse().map_err(|_|crate::store::TextError::new("transaction count must be i32",store::TextSpan::at(1,1)))?})} fn print_op(&self)->String{format!("{} {}",Self::OPCODE,self.value)} }
  |

warning: unnecessary qualification
 --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././🧪️tests/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/📝️set-transaction-count-and-notify/🦀️.rs:4:84
  |
4 | impl OpText for SetTransactionCountAndNotify { fn parse_op(line:&str)->Result<Self,crate::store::TextError>{Ok(Self{value:line.strip_...
  |                                                                                    ^^^^^^^^^^^^^^^^^^^^^^^
  |
help: remove the unnecessary path segments
  |
4 - impl OpText for SetTransactionCountAndNotify { fn parse_op(line:&str)->Result<Self,crate::store::TextError>{Ok(Self{value:line.strip_prefix("set-transaction-count-and-notify ").ok_or_else(||crate::store::TextError::new("expected set-transaction-count-and-notify",crate::store::TextSpan::at(1,1)))?.parse().map_err(|_|crate::store::TextError::new("transaction count must be i32",crate::store::TextSpan::at(1,1)))?})} fn print_op(&self)->String{format!("{} {}",Self::OPCODE,self.value)} }
4 + impl OpText for SetTransactionCountAndNotify { fn parse_op(line:&str)->Result<Self,store::TextError>{Ok(Self{value:line.strip_prefix("set-transaction-count-and-notify ").ok_or_else(||crate::store::TextError::new("expected set-transaction-count-and-notify",crate::store::TextSpan::at(1,1)))?.parse().map_err(|_|crate::store::TextError::new("transaction count must be i32",crate::store::TextSpan::at(1,1)))?})} fn print_op(&self)->String{format!("{} {}",Self::OPCODE,self.value)} }
  |

warning: unnecessary qualification
 --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././🧪️tests/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/📝️set-transaction-count-and-notify/🦀️.rs:4:191
  |
4 | ...("set-transaction-count-and-notify ").ok_or_else(||crate::store::TextError::new("expected set-transaction-count-and-notify",crate:...
  |                                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
help: remove the unnecessary path segments
  |
4 - impl OpText for SetTransactionCountAndNotify { fn parse_op(line:&str)->Result<Self,crate::store::TextError>{Ok(Self{value:line.strip_prefix("set-transaction-count-and-notify ").ok_or_else(||crate::store::TextError::new("expected set-transaction-count-and-notify",crate::store::TextSpan::at(1,1)))?.parse().map_err(|_|crate::store::TextError::new("transaction count must be i32",crate::store::TextSpan::at(1,1)))?})} fn print_op(&self)->String{format!("{} {}",Self::OPCODE,self.value)} }
4 + impl OpText for SetTransactionCountAndNotify { fn parse_op(line:&str)->Result<Self,crate::store::TextError>{Ok(Self{value:line.strip_prefix("set-transaction-count-and-notify ").ok_or_else(||store::TextError::new("expected set-transaction-count-and-notify",crate::store::TextSpan::at(1,1)))?.parse().map_err(|_|crate::store::TextError::new("transaction count must be i32",crate::store::TextSpan::at(1,1)))?})} fn print_op(&self)->String{format!("{} {}",Self::OPCODE,self.value)} }
  |

warning: unnecessary qualification
 --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././🧪️tests/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/📝️set-transaction-count-and-notify/🦀️.rs:4:264
  |
4 | ...or::new("expected set-transaction-count-and-notify",crate::store::TextSpan::at(1,1)))?.parse().map_err(|_|crate::store::TextError:...
  |                                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
help: remove the unnecessary path segments
  |
4 - impl OpText for SetTransactionCountAndNotify { fn parse_op(line:&str)->Result<Self,crate::store::TextError>{Ok(Self{value:line.strip_prefix("set-transaction-count-and-notify ").ok_or_else(||crate::store::TextError::new("expected set-transaction-count-and-notify",crate::store::TextSpan::at(1,1)))?.parse().map_err(|_|crate::store::TextError::new("transaction count must be i32",crate::store::TextSpan::at(1,1)))?})} fn print_op(&self)->String{format!("{} {}",Self::OPCODE,self.value)} }
4 + impl OpText for SetTransactionCountAndNotify { fn parse_op(line:&str)->Result<Self,crate::store::TextError>{Ok(Self{value:line.strip_prefix("set-transaction-count-and-notify ").ok_or_else(||crate::store::TextError::new("expected set-transaction-count-and-notify",store::TextSpan::at(1,1)))?.parse().map_err(|_|crate::store::TextError::new("transaction count must be i32",crate::store::TextSpan::at(1,1)))?})} fn print_op(&self)->String{format!("{} {}",Self::OPCODE,self.value)} }
  |

warning: unnecessary qualification
 --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././🧪️tests/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/📝️set-transaction-count-and-notify/🦀️.rs:4:318
  |
4 | ...te::store::TextSpan::at(1,1)))?.parse().map_err(|_|crate::store::TextError::new("transaction count must be i32",crate::store::Text...
  |                                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
help: remove the unnecessary path segments
  |
4 - impl OpText for SetTransactionCountAndNotify { fn parse_op(line:&str)->Result<Self,crate::store::TextError>{Ok(Self{value:line.strip_prefix("set-transaction-count-and-notify ").ok_or_else(||crate::store::TextError::new("expected set-transaction-count-and-notify",crate::store::TextSpan::at(1,1)))?.parse().map_err(|_|crate::store::TextError::new("transaction count must be i32",crate::store::TextSpan::at(1,1)))?})} fn print_op(&self)->String{format!("{} {}",Self::OPCODE,self.value)} }
4 + impl OpText for SetTransactionCountAndNotify { fn parse_op(line:&str)->Result<Self,crate::store::TextError>{Ok(Self{value:line.strip_prefix("set-transaction-count-and-notify ").ok_or_else(||crate::store::TextError::new("expected set-transaction-count-and-notify",crate::store::TextSpan::at(1,1)))?.parse().map_err(|_|store::TextError::new("transaction count must be i32",crate::store::TextSpan::at(1,1)))?})} fn print_op(&self)->String{format!("{} {}",Self::OPCODE,self.value)} }
  |

warning: unnecessary qualification
 --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././🧪️tests/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/📝️set-transaction-count-and-notify/🦀️.rs:4:379
  |
4 | ...ore::TextError::new("transaction count must be i32",crate::store::TextSpan::at(1,1)))?})} fn print_op(&self)->String{format!("{} {...
  |                                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
help: remove the unnecessary path segments
  |
4 - impl OpText for SetTransactionCountAndNotify { fn parse_op(line:&str)->Result<Self,crate::store::TextError>{Ok(Self{value:line.strip_prefix("set-transaction-count-and-notify ").ok_or_else(||crate::store::TextError::new("expected set-transaction-count-and-notify",crate::store::TextSpan::at(1,1)))?.parse().map_err(|_|crate::store::TextError::new("transaction count must be i32",crate::store::TextSpan::at(1,1)))?})} fn print_op(&self)->String{format!("{} {}",Self::OPCODE,self.value)} }
4 + impl OpText for SetTransactionCountAndNotify { fn parse_op(line:&str)->Result<Self,crate::store::TextError>{Ok(Self{value:line.strip_prefix("set-transaction-count-and-notify ").ok_or_else(||crate::store::TextError::new("expected set-transaction-count-and-notify",crate::store::TextSpan::at(1,1)))?.parse().map_err(|_|crate::store::TextError::new("transaction count must be i32",store::TextSpan::at(1,1)))?})} fn print_op(&self)->String{format!("{} {}",Self::OPCODE,self.value)} }
  |

warning: unnecessary qualification
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././🧪️tests/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/🦀️.rs:12:81
   |
12 | impl protocol::OpText for TxnMutation { fn parse_op(line: &str) -> Result<Self, crate::store::TextError> { if line.starts_with("set-...
   |                                                                                 ^^^^^^^^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
12 - impl protocol::OpText for TxnMutation { fn parse_op(line: &str) -> Result<Self, crate::store::TextError> { if line.starts_with("set-transaction-count-and-notify ") { Ok(SetTransactionCountAndNotify::parse_op(line)?.into()) } else if line.starts_with("set-transaction-count-without-preflight ") { Ok(SetTransactionCountWithoutPreflight::parse_op(line)?.into()) } else { Ok(SetTransactionCount::parse_op(line)?.into()) } } fn print_op(&self) -> String { match self { Self::SetTransactionCount(value) => value.print_op(), Self::SetTransactionCountWithoutPreflight(value) => value.print_op(), Self::SetTransactionCountAndNotify(value) => value.print_op() } } }
12 + impl protocol::OpText for TxnMutation { fn parse_op(line: &str) -> Result<Self, store::TextError> { if line.starts_with("set-transaction-count-and-notify ") { Ok(SetTransactionCountAndNotify::parse_op(line)?.into()) } else if line.starts_with("set-transaction-count-without-preflight ") { Ok(SetTransactionCountWithoutPreflight::parse_op(line)?.into()) } else { Ok(SetTransactionCount::parse_op(line)?.into()) } } fn print_op(&self) -> String { match self { Self::SetTransactionCount(value) => value.print_op(), Self::SetTransactionCountWithoutPreflight(value) => value.print_op(), Self::SetTransactionCountAndNotify(value) => value.print_op() } } }
   |

warning: unused import: `Mutation`
 --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././🧪️tests/🧬️mutation-fixtures/🔀️transaction/🦀️.rs:9:16
  |
9 | use protocol::{Mutation, MutationDiff};
  |                ^^^^^^^^

warning: unnecessary qualification
 --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././🧪️tests/🧬️mutation-fixtures/🪟️surface/🧬️mutations/📝️set-surface-count/🦀️.rs:4:71
  |
4 | impl OpText for SetSurfaceCount { fn parse_op(line:&str)->Result<Self,crate::store::TextError>{Ok(Self{value:line.strip_prefix("set-s...
  |                                                                       ^^^^^^^^^^^^^^^^^^^^^^^
  |
help: remove the unnecessary path segments
  |
4 - impl OpText for SetSurfaceCount { fn parse_op(line:&str)->Result<Self,crate::store::TextError>{Ok(Self{value:line.strip_prefix("set-surface-count ").ok_or_else(||crate::store::TextError::new("expected set-surface-count",crate::store::TextSpan::at(1,1)))?.parse().map_err(|_|crate::store::TextError::new("surface count must be i32",crate::store::TextSpan::at(1,1)))?})} fn print_op(&self)->String{format!("{} {}",Self::OPCODE,self.value)} }
4 + impl OpText for SetSurfaceCount { fn parse_op(line:&str)->Result<Self,store::TextError>{Ok(Self{value:line.strip_prefix("set-surface-count ").ok_or_else(||crate::store::TextError::new("expected set-surface-count",crate::store::TextSpan::at(1,1)))?.parse().map_err(|_|crate::store::TextError::new("surface count must be i32",crate::store::TextSpan::at(1,1)))?})} fn print_op(&self)->String{format!("{} {}",Self::OPCODE,self.value)} }
  |

warning: unnecessary qualification
 --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././🧪️tests/🧬️mutation-fixtures/🪟️surface/🧬️mutations/📝️set-surface-count/🦀️.rs:4:163
  |
4 | ...ne.strip_prefix("set-surface-count ").ok_or_else(||crate::store::TextError::new("expected set-surface-count",crate::store::TextSpa...
  |                                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
help: remove the unnecessary path segments
  |
4 - impl OpText for SetSurfaceCount { fn parse_op(line:&str)->Result<Self,crate::store::TextError>{Ok(Self{value:line.strip_prefix("set-surface-count ").ok_or_else(||crate::store::TextError::new("expected set-surface-count",crate::store::TextSpan::at(1,1)))?.parse().map_err(|_|crate::store::TextError::new("surface count must be i32",crate::store::TextSpan::at(1,1)))?})} fn print_op(&self)->String{format!("{} {}",Self::OPCODE,self.value)} }
4 + impl OpText for SetSurfaceCount { fn parse_op(line:&str)->Result<Self,crate::store::TextError>{Ok(Self{value:line.strip_prefix("set-surface-count ").ok_or_else(||store::TextError::new("expected set-surface-count",crate::store::TextSpan::at(1,1)))?.parse().map_err(|_|crate::store::TextError::new("surface count must be i32",crate::store::TextSpan::at(1,1)))?})} fn print_op(&self)->String{format!("{} {}",Self::OPCODE,self.value)} }
  |

warning: unnecessary qualification
 --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././🧪️tests/🧬️mutation-fixtures/🪟️surface/🧬️mutations/📝️set-surface-count/🦀️.rs:4:221
  |
4 | ...:store::TextError::new("expected set-surface-count",crate::store::TextSpan::at(1,1)))?.parse().map_err(|_|crate::store::TextError:...
  |                                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
help: remove the unnecessary path segments
  |
4 - impl OpText for SetSurfaceCount { fn parse_op(line:&str)->Result<Self,crate::store::TextError>{Ok(Self{value:line.strip_prefix("set-surface-count ").ok_or_else(||crate::store::TextError::new("expected set-surface-count",crate::store::TextSpan::at(1,1)))?.parse().map_err(|_|crate::store::TextError::new("surface count must be i32",crate::store::TextSpan::at(1,1)))?})} fn print_op(&self)->String{format!("{} {}",Self::OPCODE,self.value)} }
4 + impl OpText for SetSurfaceCount { fn parse_op(line:&str)->Result<Self,crate::store::TextError>{Ok(Self{value:line.strip_prefix("set-surface-count ").ok_or_else(||crate::store::TextError::new("expected set-surface-count",store::TextSpan::at(1,1)))?.parse().map_err(|_|crate::store::TextError::new("surface count must be i32",crate::store::TextSpan::at(1,1)))?})} fn print_op(&self)->String{format!("{} {}",Self::OPCODE,self.value)} }
  |

warning: unnecessary qualification
 --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././🧪️tests/🧬️mutation-fixtures/🪟️surface/🧬️mutations/📝️set-surface-count/🦀️.rs:4:275
  |
4 | ...te::store::TextSpan::at(1,1)))?.parse().map_err(|_|crate::store::TextError::new("surface count must be i32",crate::store::TextSpan...
  |                                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
help: remove the unnecessary path segments
  |
4 - impl OpText for SetSurfaceCount { fn parse_op(line:&str)->Result<Self,crate::store::TextError>{Ok(Self{value:line.strip_prefix("set-surface-count ").ok_or_else(||crate::store::TextError::new("expected set-surface-count",crate::store::TextSpan::at(1,1)))?.parse().map_err(|_|crate::store::TextError::new("surface count must be i32",crate::store::TextSpan::at(1,1)))?})} fn print_op(&self)->String{format!("{} {}",Self::OPCODE,self.value)} }
4 + impl OpText for SetSurfaceCount { fn parse_op(line:&str)->Result<Self,crate::store::TextError>{Ok(Self{value:line.strip_prefix("set-surface-count ").ok_or_else(||crate::store::TextError::new("expected set-surface-count",crate::store::TextSpan::at(1,1)))?.parse().map_err(|_|store::TextError::new("surface count must be i32",crate::store::TextSpan::at(1,1)))?})} fn print_op(&self)->String{format!("{} {}",Self::OPCODE,self.value)} }
  |

warning: unnecessary qualification
 --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././🧪️tests/🧬️mutation-fixtures/🪟️surface/🧬️mutations/📝️set-surface-count/🦀️.rs:4:332
  |
4 | ...::store::TextError::new("surface count must be i32",crate::store::TextSpan::at(1,1)))?})} fn print_op(&self)->String{format!("{} {...
  |                                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
help: remove the unnecessary path segments
  |
4 - impl OpText for SetSurfaceCount { fn parse_op(line:&str)->Result<Self,crate::store::TextError>{Ok(Self{value:line.strip_prefix("set-surface-count ").ok_or_else(||crate::store::TextError::new("expected set-surface-count",crate::store::TextSpan::at(1,1)))?.parse().map_err(|_|crate::store::TextError::new("surface count must be i32",crate::store::TextSpan::at(1,1)))?})} fn print_op(&self)->String{format!("{} {}",Self::OPCODE,self.value)} }
4 + impl OpText for SetSurfaceCount { fn parse_op(line:&str)->Result<Self,crate::store::TextError>{Ok(Self{value:line.strip_prefix("set-surface-count ").ok_or_else(||crate::store::TextError::new("expected set-surface-count",crate::store::TextSpan::at(1,1)))?.parse().map_err(|_|crate::store::TextError::new("surface count must be i32",store::TextSpan::at(1,1)))?})} fn print_op(&self)->String{format!("{} {}",Self::OPCODE,self.value)} }
  |

warning: unnecessary qualification
 --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././🧪️tests/🧬️mutation-fixtures/🪟️surface/🧬️mutations/🦀️.rs:4:81
  |
4 | impl protocol::OpText for SurfaceMutation { fn parse_op(line:&str)->Result<Self,crate::store::TextError>{Ok(SetSurfaceCount::parse_op...
  |                                                                                 ^^^^^^^^^^^^^^^^^^^^^^^
  |
help: remove the unnecessary path segments
  |
4 - impl protocol::OpText for SurfaceMutation { fn parse_op(line:&str)->Result<Self,crate::store::TextError>{Ok(SetSurfaceCount::parse_op(line)?.into())} fn print_op(&self)->String{match self{Self::SetSurfaceCount(value)=>value.print_op()}} }
4 + impl protocol::OpText for SurfaceMutation { fn parse_op(line:&str)->Result<Self,store::TextError>{Ok(SetSurfaceCount::parse_op(line)?.into())} fn print_op(&self)->String{match self{Self::SetSurfaceCount(value)=>value.print_op()}} }
  |

warning: unused import: `ViewerApp`
 --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././🧪️tests/🧬️mutation-fixtures/🪟️surface/🦀️.rs:6:343
  |
6 | ...p, REVERT_TO_COMMAND_ACTION_ID, UiAssemblyResult, ViewEmit, ViewerApp};
  |                                                                ^^^^^^^^^

warning: unused import: `Mutation`
 --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././🧪️tests/🧬️mutation-fixtures/🪟️surface/🦀️.rs:8:16
  |
8 | use protocol::{Mutation, MutationDiff};
  |                ^^^^^^^^

warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:6976:64
     |
6976 | ...   pub async fn assert_declaration_tree_registers_all<PA: super::PluginApp>(plugin_id: &str, declaration: declarations::Artifac...
     |                                                              ^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
6976 -         pub async fn assert_declaration_tree_registers_all<PA: super::PluginApp>(plugin_id: &str, declaration: declarations::ArtifactDeclaration<PA>) {
6976 +         pub async fn assert_declaration_tree_registers_all<PA: PluginApp>(plugin_id: &str, declaration: declarations::ArtifactDeclaration<PA>) {
     |

warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:7010:68
     |
7010 | ...   pub async fn assert_declaration_registration_is_atomic<PA: super::PluginApp>(plugin_id: &str, invalid: declarations::Artifac...
     |                                                                  ^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
7010 -         pub async fn assert_declaration_registration_is_atomic<PA: super::PluginApp>(plugin_id: &str, invalid: declarations::ArtifactDeclaration<PA>) {
7010 +         pub async fn assert_declaration_registration_is_atomic<PA: PluginApp>(plugin_id: &str, invalid: declarations::ArtifactDeclaration<PA>) {
     |

warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:7021:68
     |
7021 | ...   pub async fn assert_subset_declaration_ids_are_derived<PA: super::PluginApp>(declaration: &declarations::ArtifactDeclaration...
     |                                                                  ^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
7021 -         pub async fn assert_subset_declaration_ids_are_derived<PA: super::PluginApp>(declaration: &declarations::ArtifactDeclaration<PA>) {
7021 +         pub async fn assert_subset_declaration_ids_are_derived<PA: PluginApp>(declaration: &declarations::ArtifactDeclaration<PA>) {
     |

warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:9776:77
     |
9776 | ...ocol::MutationLeafDescriptor] = &[<crate::local_interaction::set_state::SetInteractionState as protocol::MutationLeaf>::DESCRIP...
     |                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
9776 -         const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[<crate::local_interaction::set_state::SetInteractionState as protocol::MutationLeaf>::DESCRIPTOR];
9776 +         const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[<SetInteractionState as protocol::MutationLeaf>::DESCRIPTOR];
     |

warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:9959:27
     |
9959 |             let builder = ui::button(ui_label(label, "history-panel.button-label")?).icon(icon.clone()).disabled(!enabled);
     |                           ^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
9959 -             let builder = ui::button(ui_label(label, "history-panel.button-label")?).icon(icon.clone()).disabled(!enabled);
9959 +             let builder = button(ui_label(label, "history-panel.button-label")?).icon(icon.clone()).disabled(!enabled);
     |

warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:9973:34
     |
9973 |         let mut filter_builder = ui::select(ui_text(filter_value, "history-panel.filter-value")?);
     |                                  ^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
9973 -         let mut filter_builder = ui::select(ui_text(filter_value, "history-panel.filter-value")?);
9973 +         let mut filter_builder = select(ui_text(filter_value, "history-panel.filter-value")?);
     |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:10033:31
      |
10033 | ...   let actions_builder = ui::tree_section(ui_label(if is_de { "Aktionen" } else { "Actions" }, "history-panel.actions-label")?...
      |                             ^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
10033 -         let actions_builder = ui::tree_section(ui_label(if is_de { "Aktionen" } else { "Actions" }, "history-panel.actions-label")?).default_open(true);
10033 +         let actions_builder = tree_section(ui_label(if is_de { "Aktionen" } else { "Actions" }, "history-panel.actions-label")?).default_open(true);
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:10036:32
      |
10036 | ...   let commands_builder = ui::tree_section(ui_label(if is_de { "Befehle" } else { "Commands" }, "history-panel.commands-label"...
      |                              ^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
10036 -         let commands_builder = ui::tree_section(ui_label(if is_de { "Befehle" } else { "Commands" }, "history-panel.commands-label")?).default_open(true);
10036 +         let commands_builder = tree_section(ui_label(if is_de { "Befehle" } else { "Commands" }, "history-panel.commands-label")?).default_open(true);
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:10042:9
      |
10042 | ...   ui::tree().try_children(sections).map_err(|_| ui_assembly_error("history-panel.sections"))?.try_build().map_err(|_| ui_asse...
      |       ^^^^^^^^
      |
help: remove the unnecessary path segments
      |
10042 -         ui::tree().try_children(sections).map_err(|_| ui_assembly_error("history-panel.sections"))?.try_build().map_err(|_| ui_assembly_error("history-panel.build"))
10042 +         tree().try_children(sections).map_err(|_| ui_assembly_error("history-panel.sections"))?.try_build().map_err(|_| ui_assembly_error("history-panel.build"))
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:12206:18
      |
12206 |             Fut: std::future::Future<Output = Menu<'a>>,
      |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
12206 -             Fut: std::future::Future<Output = Menu<'a>>,
12206 +             Fut: Future<Output = Menu<'a>>,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:12964:29
      |
12964 | ...   fn try_serialize<T: serde::Serialize>(token: TypedOperationResultToken, lane: TypedOperationResultLane, value: &T) -> Resul...
      |                           ^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
12964 -         fn try_serialize<T: serde::Serialize>(token: TypedOperationResultToken, lane: TypedOperationResultLane, value: &T) -> Result<Self, Fault> {
12964 +         fn try_serialize<T: Serialize>(token: TypedOperationResultToken, lane: TypedOperationResultLane, value: &T) -> Result<Self, Fault> {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:13280:32
      |
13280 |         C: Clone + Serialize + serde::de::DeserializeOwned + ArtifactPack + Send + Sync + 'static,
      |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
13280 -         C: Clone + Serialize + serde::de::DeserializeOwned + ArtifactPack + Send + Sync + 'static,
13280 +         C: Clone + Serialize + DeserializeOwned + ArtifactPack + Send + Sync + 'static,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:13281:32
      |
13281 |         M: Clone + Serialize + serde::de::DeserializeOwned + store::Mutation<C> + OpBinary + OpText + Send + 'static,
      |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
13281 -         M: Clone + Serialize + serde::de::DeserializeOwned + store::Mutation<C> + OpBinary + OpText + Send + 'static,
13281 +         M: Clone + Serialize + DeserializeOwned + store::Mutation<C> + OpBinary + OpText + Send + 'static,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:13281:62
      |
13281 |         M: Clone + Serialize + serde::de::DeserializeOwned + store::Mutation<C> + OpBinary + OpText + Send + 'static,
      |                                                              ^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
13281 -         M: Clone + Serialize + serde::de::DeserializeOwned + store::Mutation<C> + OpBinary + OpText + Send + 'static,
13281 +         M: Clone + Serialize + serde::de::DeserializeOwned + Mutation<C> + OpBinary + OpText + Send + 'static,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:13294:32
      |
13294 |         C: Clone + Serialize + serde::de::DeserializeOwned + ArtifactPack + Send + Sync + 'static,
      |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
13294 -         C: Clone + Serialize + serde::de::DeserializeOwned + ArtifactPack + Send + Sync + 'static,
13294 +         C: Clone + Serialize + DeserializeOwned + ArtifactPack + Send + Sync + 'static,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:13295:32
      |
13295 |         M: Clone + Serialize + serde::de::DeserializeOwned + store::Mutation<C> + OpBinary + OpText + Send + 'static,
      |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
13295 -         M: Clone + Serialize + serde::de::DeserializeOwned + store::Mutation<C> + OpBinary + OpText + Send + 'static,
13295 +         M: Clone + Serialize + DeserializeOwned + store::Mutation<C> + OpBinary + OpText + Send + 'static,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:13295:62
      |
13295 |         M: Clone + Serialize + serde::de::DeserializeOwned + store::Mutation<C> + OpBinary + OpText + Send + 'static,
      |                                                              ^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
13295 -         M: Clone + Serialize + serde::de::DeserializeOwned + store::Mutation<C> + OpBinary + OpText + Send + 'static,
13295 +         M: Clone + Serialize + serde::de::DeserializeOwned + Mutation<C> + OpBinary + OpText + Send + 'static,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:13303:32
      |
13303 |         P: Clone + Serialize + serde::de::DeserializeOwned + ArtifactPack + Send + Sync + 'static,
      |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
13303 -         P: Clone + Serialize + serde::de::DeserializeOwned + ArtifactPack + Send + Sync + 'static,
13303 +         P: Clone + Serialize + DeserializeOwned + ArtifactPack + Send + Sync + 'static,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:13304:32
      |
13304 |         M: Clone + Serialize + serde::de::DeserializeOwned + store::Mutation<P> + OpBinary + OpText + Send + 'static,
      |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
13304 -         M: Clone + Serialize + serde::de::DeserializeOwned + store::Mutation<P> + OpBinary + OpText + Send + 'static,
13304 +         M: Clone + Serialize + DeserializeOwned + store::Mutation<P> + OpBinary + OpText + Send + 'static,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:13304:62
      |
13304 |         M: Clone + Serialize + serde::de::DeserializeOwned + store::Mutation<P> + OpBinary + OpText + Send + 'static,
      |                                                              ^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
13304 -         M: Clone + Serialize + serde::de::DeserializeOwned + store::Mutation<P> + OpBinary + OpText + Send + 'static,
13304 +         M: Clone + Serialize + serde::de::DeserializeOwned + Mutation<P> + OpBinary + OpText + Send + 'static,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:13312:32
      |
13312 |         P: Clone + Serialize + serde::de::DeserializeOwned + ArtifactPack + Send + Sync + 'static,
      |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
13312 -         P: Clone + Serialize + serde::de::DeserializeOwned + ArtifactPack + Send + Sync + 'static,
13312 +         P: Clone + Serialize + DeserializeOwned + ArtifactPack + Send + Sync + 'static,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:13313:32
      |
13313 |         M: Clone + Serialize + serde::de::DeserializeOwned + store::Mutation<P> + OpBinary + OpText + Send + 'static,
      |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
13313 -         M: Clone + Serialize + serde::de::DeserializeOwned + store::Mutation<P> + OpBinary + OpText + Send + 'static,
13313 +         M: Clone + Serialize + DeserializeOwned + store::Mutation<P> + OpBinary + OpText + Send + 'static,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:13313:62
      |
13313 |         M: Clone + Serialize + serde::de::DeserializeOwned + store::Mutation<P> + OpBinary + OpText + Send + 'static,
      |                                                              ^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
13313 -         M: Clone + Serialize + serde::de::DeserializeOwned + store::Mutation<P> + OpBinary + OpText + Send + 'static,
13313 +         M: Clone + Serialize + serde::de::DeserializeOwned + Mutation<P> + OpBinary + OpText + Send + 'static,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:13332:32
      |
13332 |         P: Clone + Serialize + serde::de::DeserializeOwned + ArtifactPack + Send + Sync + 'static,
      |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
13332 -         P: Clone + Serialize + serde::de::DeserializeOwned + ArtifactPack + Send + Sync + 'static,
13332 +         P: Clone + Serialize + DeserializeOwned + ArtifactPack + Send + Sync + 'static,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:13333:39
      |
13333 |         Mutation: Clone + Serialize + serde::de::DeserializeOwned + store::Mutation<P> + OpBinary + OpText + Send + 'static,
      |                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
13333 -         Mutation: Clone + Serialize + serde::de::DeserializeOwned + store::Mutation<P> + OpBinary + OpText + Send + 'static,
13333 +         Mutation: Clone + Serialize + DeserializeOwned + store::Mutation<P> + OpBinary + OpText + Send + 'static,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:13336:19
      |
13336 |             match store::SpaceMember::close_owned_step(owner, maximum_items.min(1), maximum_bytes).map_err(plugin_sdk_fault)? {
      |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
13336 -             match store::SpaceMember::close_owned_step(owner, maximum_items.min(1), maximum_bytes).map_err(plugin_sdk_fault)? {
13336 +             match SpaceMember::close_owned_step(owner, maximum_items.min(1), maximum_bytes).map_err(plugin_sdk_fault)? {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:13345:13
      |
13345 |             store::SpaceMember::close_owned_terminal_is_empty(owner)
      |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
13345 -             store::SpaceMember::close_owned_terminal_is_empty(owner)
13345 +             SpaceMember::close_owned_terminal_is_empty(owner)
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:13354:32
      |
13354 |         P: Clone + Serialize + serde::de::DeserializeOwned,
      |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
13354 -         P: Clone + Serialize + serde::de::DeserializeOwned,
13354 +         P: Clone + Serialize + DeserializeOwned,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:13355:39
      |
13355 |         Mutation: Clone + Serialize + serde::de::DeserializeOwned + store::Mutation<P>,
      |                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
13355 -         Mutation: Clone + Serialize + serde::de::DeserializeOwned + store::Mutation<P>,
13355 +         Mutation: Clone + Serialize + DeserializeOwned + store::Mutation<P>,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:13367:32
      |
13367 |         P: Clone + Serialize + serde::de::DeserializeOwned,
      |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
13367 -         P: Clone + Serialize + serde::de::DeserializeOwned,
13367 +         P: Clone + Serialize + DeserializeOwned,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:13368:39
      |
13368 |         Mutation: Clone + Serialize + serde::de::DeserializeOwned + store::Mutation<P>,
      |                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
13368 -         Mutation: Clone + Serialize + serde::de::DeserializeOwned + store::Mutation<P>,
13368 +         Mutation: Clone + Serialize + DeserializeOwned + store::Mutation<P>,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:13377:32
      |
13377 |         P: Clone + Serialize + serde::de::DeserializeOwned,
      |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
13377 -         P: Clone + Serialize + serde::de::DeserializeOwned,
13377 +         P: Clone + Serialize + DeserializeOwned,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:13378:39
      |
13378 |         Mutation: Clone + Serialize + serde::de::DeserializeOwned + store::Mutation<P>,
      |                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
13378 -         Mutation: Clone + Serialize + serde::de::DeserializeOwned + store::Mutation<P>,
13378 +         Mutation: Clone + Serialize + DeserializeOwned + store::Mutation<P>,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:13417:32
      |
13417 |         P: Clone + Serialize + serde::de::DeserializeOwned + Send,
      |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
13417 -         P: Clone + Serialize + serde::de::DeserializeOwned + Send,
13417 +         P: Clone + Serialize + DeserializeOwned + Send,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:13418:39
      |
13418 |         Mutation: Clone + Serialize + serde::de::DeserializeOwned + store::Mutation<P> + Send,
      |                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
13418 -         Mutation: Clone + Serialize + serde::de::DeserializeOwned + store::Mutation<P> + Send,
13418 +         Mutation: Clone + Serialize + DeserializeOwned + store::Mutation<P> + Send,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:13470:32
      |
13470 |         P: Clone + Serialize + serde::de::DeserializeOwned,
      |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
13470 -         P: Clone + Serialize + serde::de::DeserializeOwned,
13470 +         P: Clone + Serialize + DeserializeOwned,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:13471:39
      |
13471 |         Mutation: Clone + Serialize + serde::de::DeserializeOwned + store::Mutation<P>,
      |                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
13471 -         Mutation: Clone + Serialize + serde::de::DeserializeOwned + store::Mutation<P>,
13471 +         Mutation: Clone + Serialize + DeserializeOwned + store::Mutation<P>,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:13770:33
      |
13770 |             struct DropSentinel(std::sync::Arc<std::sync::atomic::AtomicUsize>);
      |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
13770 -             struct DropSentinel(std::sync::Arc<std::sync::atomic::AtomicUsize>);
13770 +             struct DropSentinel(Arc<std::sync::atomic::AtomicUsize>);
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:13776:25
      |
13776 |             let drops = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
      |                         ^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
13776 -             let drops = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
13776 +             let drops = Arc::new(std::sync::atomic::AtomicUsize::new(0));
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:13805:29
      |
13805 |             struct DropItem(std::sync::Arc<std::sync::atomic::AtomicUsize>);
      |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
13805 -             struct DropItem(std::sync::Arc<std::sync::atomic::AtomicUsize>);
13805 +             struct DropItem(Arc<std::sync::atomic::AtomicUsize>);
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:13813:64
      |
13813 | ...   fn close_step(&mut self, snapshot: &mut Option<std::sync::Arc<Vec<DropItem>>>, maximum_items: usize, _maximum_bytes: usize)...
      |                                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
13813 -                 fn close_step(&mut self, snapshot: &mut Option<std::sync::Arc<Vec<DropItem>>>, maximum_items: usize, _maximum_bytes: usize) -> Result<PluginCloseStep, Fault> {
13813 +                 fn close_step(&mut self, snapshot: &mut Option<Arc<Vec<DropItem>>>, maximum_items: usize, _maximum_bytes: usize) -> Result<PluginCloseStep, Fault> {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:13818:39
      |
13818 | ...   let Some(items) = std::sync::Arc::get_mut(owner) else { return Ok(PluginCloseStep::Blocked { reason: "snapshot remains exte...
      |                         ^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
13818 -                     let Some(items) = std::sync::Arc::get_mut(owner) else { return Ok(PluginCloseStep::Blocked { reason: "snapshot remains externally owned" }) };
13818 +                     let Some(items) = Arc::get_mut(owner) else { return Ok(PluginCloseStep::Blocked { reason: "snapshot remains externally owned" }) };
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:13826:63
      |
13826 |                 fn terminal_is_empty(&self, snapshot: &Option<std::sync::Arc<Vec<DropItem>>>) -> bool {
      |                                                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
13826 -                 fn terminal_is_empty(&self, snapshot: &Option<std::sync::Arc<Vec<DropItem>>>) -> bool {
13826 +                 fn terminal_is_empty(&self, snapshot: &Option<Arc<Vec<DropItem>>>) -> bool {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:13831:25
      |
13831 |             let drops = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
      |                         ^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
13831 -             let drops = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
13831 +             let drops = Arc::new(std::sync::atomic::AtomicUsize::new(0));
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:13832:27
      |
13832 |             let cache_a = std::sync::Arc::new(vec![DropItem(drops.clone()), DropItem(drops.clone())]);
      |                           ^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
13832 -             let cache_a = std::sync::Arc::new(vec![DropItem(drops.clone()), DropItem(drops.clone())]);
13832 +             let cache_a = Arc::new(vec![DropItem(drops.clone()), DropItem(drops.clone())]);
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:13835:27
      |
13835 |             let cache_b = std::sync::Arc::new(Vec::<DropItem>::new());
      |                           ^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
13835 -             let cache_b = std::sync::Arc::new(Vec::<DropItem>::new());
13835 +             let cache_b = Arc::new(Vec::<DropItem>::new());
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:13846:21
      |
13846 |             assert!(std::sync::Arc::strong_count(&cache_b) == 1, "cache B is independent of retired A authority");
      |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
13846 -             assert!(std::sync::Arc::strong_count(&cache_b) == 1, "cache B is independent of retired A authority");
13846 +             assert!(Arc::strong_count(&cache_b) == 1, "cache B is independent of retired A authority");
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:15439:42
      |
15439 |         struct ActiveMediaExportSentinel(std::sync::Arc<std::sync::atomic::AtomicUsize>);
      |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
15439 -         struct ActiveMediaExportSentinel(std::sync::Arc<std::sync::atomic::AtomicUsize>);
15439 +         struct ActiveMediaExportSentinel(Arc<std::sync::atomic::AtomicUsize>);
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:15441:42
      |
15441 |         struct SnapshotRetentionSentinel(std::sync::Arc<std::sync::atomic::AtomicUsize>);
      |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
15441 -         struct SnapshotRetentionSentinel(std::sync::Arc<std::sync::atomic::AtomicUsize>);
15441 +         struct SnapshotRetentionSentinel(Arc<std::sync::atomic::AtomicUsize>);
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:15443:42
      |
15443 |         struct SegmentedDownloadSentinel(std::sync::Arc<std::sync::atomic::AtomicUsize>);
      |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
15443 -         struct SegmentedDownloadSentinel(std::sync::Arc<std::sync::atomic::AtomicUsize>);
15443 +         struct SegmentedDownloadSentinel(Arc<std::sync::atomic::AtomicUsize>);
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:15459:81
      |
15459 |         fn reject_duplicate<T: std::fmt::Debug>(first: T, duplicate: T, drops: &std::sync::Arc<std::sync::atomic::AtomicUsize>) {
      |                                                                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
15459 -         fn reject_duplicate<T: std::fmt::Debug>(first: T, duplicate: T, drops: &std::sync::Arc<std::sync::atomic::AtomicUsize>) {
15459 +         fn reject_duplicate<T: std::fmt::Debug>(first: T, duplicate: T, drops: &Arc<std::sync::atomic::AtomicUsize>) {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:15478:31
      |
15478 |             let media_drops = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
      |                               ^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
15478 -             let media_drops = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
15478 +             let media_drops = Arc::new(std::sync::atomic::AtomicUsize::new(0));
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:15480:34
      |
15480 |             let snapshot_drops = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
      |                                  ^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
15480 -             let snapshot_drops = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
15480 +             let snapshot_drops = Arc::new(std::sync::atomic::AtomicUsize::new(0));
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:15482:34
      |
15482 |             let download_drops = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
      |                                  ^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
15482 -             let download_drops = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
15482 +             let download_drops = Arc::new(std::sync::atomic::AtomicUsize::new(0));
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:15490:24
      |
15490 |                 drops: std::sync::Arc<std::sync::atomic::AtomicUsize>,
      |                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
15490 -                 drops: std::sync::Arc<std::sync::atomic::AtomicUsize>,
15490 +                 drops: Arc<std::sync::atomic::AtomicUsize>,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:15497:25
      |
15497 |             let drops = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
      |                         ^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
15497 -             let drops = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
15497 +             let drops = Arc::new(std::sync::atomic::AtomicUsize::new(0));
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:15517:27
      |
15517 |                 identity: std::sync::Arc<()>,
      |                           ^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
15517 -                 identity: std::sync::Arc<()>,
15517 +                 identity: Arc<()>,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:15518:24
      |
15518 |                 drops: std::sync::Arc<std::sync::atomic::AtomicUsize>,
      |                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
15518 -                 drops: std::sync::Arc<std::sync::atomic::AtomicUsize>,
15518 +                 drops: Arc<std::sync::atomic::AtomicUsize>,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:15525:25
      |
15525 |             let drops = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
      |                         ^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
15525 -             let drops = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
15525 +             let drops = Arc::new(std::sync::atomic::AtomicUsize::new(0));
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:15526:34
      |
15526 |             let first_identity = std::sync::Arc::new(());
      |                                  ^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
15526 -             let first_identity = std::sync::Arc::new(());
15526 +             let first_identity = Arc::new(());
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:15527:35
      |
15527 |             let second_identity = std::sync::Arc::new(());
      |                                   ^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
15527 -             let second_identity = std::sync::Arc::new(());
15527 +             let second_identity = Arc::new(());
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:15533:21
      |
15533 |             assert!(std::sync::Arc::ptr_eq(&first.identity, &first_identity));
      |                     ^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
15533 -             assert!(std::sync::Arc::ptr_eq(&first.identity, &first_identity));
15533 +             assert!(Arc::ptr_eq(&first.identity, &first_identity));
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:15534:21
      |
15534 |             assert!(std::sync::Arc::ptr_eq(&live.get(42).expect("unrelated media owner remains live").identity, &second_identity));
      |                     ^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
15534 -             assert!(std::sync::Arc::ptr_eq(&live.get(42).expect("unrelated media owner remains live").identity, &second_identity));
15534 +             assert!(Arc::ptr_eq(&live.get(42).expect("unrelated media owner remains live").identity, &second_identity));
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:15588:20
      |
15588 |             drops: std::sync::Arc<std::sync::atomic::AtomicUsize>,
      |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
15588 -             drops: std::sync::Arc<std::sync::atomic::AtomicUsize>,
15588 +             drops: Arc<std::sync::atomic::AtomicUsize>,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:15627:25
      |
15627 |             let drops = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
      |                         ^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
15627 -             let drops = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
15627 +             let drops = Arc::new(std::sync::atomic::AtomicUsize::new(0));
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:16946:43
      |
16946 |         struct SerdeFixtureOracle<'a>(&'a serde_json::Value);
      |                                           ^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
16946 -         struct SerdeFixtureOracle<'a>(&'a serde_json::Value);
16946 +         struct SerdeFixtureOracle<'a>(&'a Value);
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:16998:95
      |
16998 |                     self.prepared = Some(store::ArtifactEphemeralOneItemPrepared { next_root: std::sync::Arc::new(next_root) });
      |                                                                                               ^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
16998 -                     self.prepared = Some(store::ArtifactEphemeralOneItemPrepared { next_root: std::sync::Arc::new(next_root) });
16998 +                     self.prepared = Some(store::ArtifactEphemeralOneItemPrepared { next_root: Arc::new(next_root) });
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:17038:26
      |
17038 |             root: Option<std::sync::Arc<PublicationPresence>>,
      |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
17038 -             root: Option<std::sync::Arc<PublicationPresence>>,
17038 +             root: Option<Arc<PublicationPresence>>,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:17060:40
      |
17060 |             fn retire(&self, snapshot: std::sync::Arc<PublicationPresence>) -> Box<dyn store::ErasedSnapshotRetirement> {
      |                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
17060 -             fn retire(&self, snapshot: std::sync::Arc<PublicationPresence>) -> Box<dyn store::ErasedSnapshotRetirement> {
17060 +             fn retire(&self, snapshot: Arc<PublicationPresence>) -> Box<dyn store::ErasedSnapshotRetirement> {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:17149:30
      |
17149 | ...   let factory: std::sync::Arc<dyn store::SnapshotRetirementFactory<PublicationPresence>> = std::sync::Arc::new(PublicationPre...
      |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
17149 -                 let factory: std::sync::Arc<dyn store::SnapshotRetirementFactory<PublicationPresence>> = std::sync::Arc::new(PublicationPresenceLocalRootRetirementFactory);
17149 +                 let factory: Arc<dyn store::SnapshotRetirementFactory<PublicationPresence>> = std::sync::Arc::new(PublicationPresenceLocalRootRetirementFactory);
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:17149:106
      |
17149 | ...   let factory: std::sync::Arc<dyn store::SnapshotRetirementFactory<PublicationPresence>> = std::sync::Arc::new(PublicationPre...
      |                                                                                                ^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
17149 -                 let factory: std::sync::Arc<dyn store::SnapshotRetirementFactory<PublicationPresence>> = std::sync::Arc::new(PublicationPresenceLocalRootRetirementFactory);
17149 +                 let factory: std::sync::Arc<dyn store::SnapshotRetirementFactory<PublicationPresence>> = Arc::new(PublicationPresenceLocalRootRetirementFactory);
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:17177:59
      |
17177 | ...   let mut close = presence.begin_retirement(std::sync::Arc::new(PublicationPresence::default()), |_| true).ok().unwrap();
      |                                                 ^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
17177 -                 let mut close = presence.begin_retirement(std::sync::Arc::new(PublicationPresence::default()), |_| true).ok().unwrap();
17177 +                 let mut close = presence.begin_retirement(Arc::new(PublicationPresence::default()), |_| true).ok().unwrap();
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:17183:54
      |
17183 |         fn fixture_latest_wins_key(scope: &Value) -> std::sync::Arc<String> {
      |                                                      ^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
17183 -         fn fixture_latest_wins_key(scope: &Value) -> std::sync::Arc<String> {
17183 +         fn fixture_latest_wins_key(scope: &Value) -> Arc<String> {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:17252:167
      |
17252 | ...pp.store.generation_now() - generation, "sameRoot": std::sync::Arc::ptr_eq(&before, &app.store.snapshot_root()) });
      |                                                        ^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
17252 -                     let actual = serde_json::json!({ "count": observe(&app.store.snapshot_root()), "generation": app.store.generation_now() - generation, "sameRoot": std::sync::Arc::ptr_eq(&before, &app.store.snapshot_root()) });
17252 +                     let actual = serde_json::json!({ "count": observe(&app.store.snapshot_root()), "generation": app.store.generation_now() - generation, "sameRoot": Arc::ptr_eq(&before, &app.store.snapshot_root()) });
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:17407:27
      |
17407 |                 let key = std::sync::Arc::new(format!("target-{index:04}"));
      |                           ^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
17407 -                 let key = std::sync::Arc::new(format!("target-{index:04}"));
17407 +                 let key = Arc::new(format!("target-{index:04}"));
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:17500:26
      |
17500 |             let fixture: serde_json::Value = serde_json::from_str(FIXTURE).expect("language-neutral typed-command fixture");
      |                          ^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
17500 -             let fixture: serde_json::Value = serde_json::from_str(FIXTURE).expect("language-neutral typed-command fixture");
17500 +             let fixture: Value = serde_json::from_str(FIXTURE).expect("language-neutral typed-command fixture");
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:17515:26
      |
17515 |             let fixture: serde_json::Value = serde_json::from_str(FIXTURE).expect("language-neutral typed-command fixture");
      |                          ^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
17515 -             let fixture: serde_json::Value = serde_json::from_str(FIXTURE).expect("language-neutral typed-command fixture");
17515 +             let fixture: Value = serde_json::from_str(FIXTURE).expect("language-neutral typed-command fixture");
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:17537:87
      |
17537 |                 let phases = law["phases"].as_array().cloned().unwrap_or_else(|| vec![serde_json::Value::Null]);
      |                                                                                       ^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
17537 -                 let phases = law["phases"].as_array().cloned().unwrap_or_else(|| vec![serde_json::Value::Null]);
17537 +                 let phases = law["phases"].as_array().cloned().unwrap_or_else(|| vec![Value::Null]);
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:17639:31
      |
17639 | ...   let root_factory: std::sync::Arc<dyn store::SnapshotRetirementFactory<PublicationPresence>> = std::sync::Arc::new(Publicati...
      |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
17639 -             let root_factory: std::sync::Arc<dyn store::SnapshotRetirementFactory<PublicationPresence>> = std::sync::Arc::new(PublicationPresenceLocalRootRetirementFactory);
17639 +             let root_factory: Arc<dyn store::SnapshotRetirementFactory<PublicationPresence>> = std::sync::Arc::new(PublicationPresenceLocalRootRetirementFactory);
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:17639:107
      |
17639 | ...   let root_factory: std::sync::Arc<dyn store::SnapshotRetirementFactory<PublicationPresence>> = std::sync::Arc::new(Publicati...
      |                                                                                                     ^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
17639 -             let root_factory: std::sync::Arc<dyn store::SnapshotRetirementFactory<PublicationPresence>> = std::sync::Arc::new(PublicationPresenceLocalRootRetirementFactory);
17639 +             let root_factory: std::sync::Arc<dyn store::SnapshotRetirementFactory<PublicationPresence>> = Arc::new(PublicationPresenceLocalRootRetirementFactory);
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:17661:55
      |
17661 |             let mut close = presence.begin_retirement(std::sync::Arc::new(PublicationPresence::default()), |_| true).ok().unwrap();
      |                                                       ^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
17661 -             let mut close = presence.begin_retirement(std::sync::Arc::new(PublicationPresence::default()), |_| true).ok().unwrap();
17661 +             let mut close = presence.begin_retirement(Arc::new(PublicationPresence::default()), |_| true).ok().unwrap();
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:17668:26
      |
17668 | ...   let fixture: serde_json::Value = serde_json::from_str(include_str!("../🏪️store/🧪️member-publication.json")).expect("retaine...
      |                    ^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
17668 -             let fixture: serde_json::Value = serde_json::from_str(include_str!("../🏪️store/🧪️member-publication.json")).expect("retained child fixture");
17668 +             let fixture: Value = serde_json::from_str(include_str!("../🏪️store/🧪️member-publication.json")).expect("retained child fixture");
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:18129:32
      |
18129 |         P: Clone + Serialize + serde::de::DeserializeOwned + ArtifactPack + Send + Sync + 'static,
      |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
18129 -         P: Clone + Serialize + serde::de::DeserializeOwned + ArtifactPack + Send + Sync + 'static,
18129 +         P: Clone + Serialize + DeserializeOwned + ArtifactPack + Send + Sync + 'static,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:18130:39
      |
18130 |         Mutation: Clone + Serialize + serde::de::DeserializeOwned + store::Mutation<P> + OpBinary + OpText + Send + 'static,
      |                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
18130 -         Mutation: Clone + Serialize + serde::de::DeserializeOwned + store::Mutation<P> + OpBinary + OpText + Send + 'static,
18130 +         Mutation: Clone + Serialize + DeserializeOwned + store::Mutation<P> + OpBinary + OpText + Send + 'static,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:18149:32
      |
18149 |         P: Clone + Serialize + serde::de::DeserializeOwned + ArtifactPack + Send + Sync + 'static,
      |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
18149 -         P: Clone + Serialize + serde::de::DeserializeOwned + ArtifactPack + Send + Sync + 'static,
18149 +         P: Clone + Serialize + DeserializeOwned + ArtifactPack + Send + Sync + 'static,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:18150:39
      |
18150 |         Mutation: Clone + Serialize + serde::de::DeserializeOwned + store::Mutation<P> + OpBinary + OpText + Send + 'static,
      |                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
18150 -         Mutation: Clone + Serialize + serde::de::DeserializeOwned + store::Mutation<P> + OpBinary + OpText + Send + 'static,
18150 +         Mutation: Clone + Serialize + DeserializeOwned + store::Mutation<P> + OpBinary + OpText + Send + 'static,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:18169:32
      |
18169 |         P: Clone + Serialize + serde::de::DeserializeOwned + ArtifactPack + Send + Sync + 'static,
      |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
18169 -         P: Clone + Serialize + serde::de::DeserializeOwned + ArtifactPack + Send + Sync + 'static,
18169 +         P: Clone + Serialize + DeserializeOwned + ArtifactPack + Send + Sync + 'static,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:18170:39
      |
18170 |         Mutation: Clone + Serialize + serde::de::DeserializeOwned + store::Mutation<P> + OpBinary + OpText + Send + 'static,
      |                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
18170 -         Mutation: Clone + Serialize + serde::de::DeserializeOwned + store::Mutation<P> + OpBinary + OpText + Send + 'static,
18170 +         Mutation: Clone + Serialize + DeserializeOwned + store::Mutation<P> + OpBinary + OpText + Send + 'static,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:18328:32
      |
18328 |         P: Clone + Serialize + serde::de::DeserializeOwned + ArtifactPack + Send + Sync + 'static,
      |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
18328 -         P: Clone + Serialize + serde::de::DeserializeOwned + ArtifactPack + Send + Sync + 'static,
18328 +         P: Clone + Serialize + DeserializeOwned + ArtifactPack + Send + Sync + 'static,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:18329:39
      |
18329 |         Mutation: Clone + Serialize + serde::de::DeserializeOwned + store::Mutation<P> + OpBinary + OpText + Send + 'static,
      |                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
18329 -         Mutation: Clone + Serialize + serde::de::DeserializeOwned + store::Mutation<P> + OpBinary + OpText + Send + 'static,
18329 +         Mutation: Clone + Serialize + DeserializeOwned + store::Mutation<P> + OpBinary + OpText + Send + 'static,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:20416:46
      |
20416 |             if artifact_mutations.iter().any(protocol::Mutation::may_emit_foreign_steps) {
      |                                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
20416 -             if artifact_mutations.iter().any(protocol::Mutation::may_emit_foreign_steps) {
20416 +             if artifact_mutations.iter().any(Mutation::may_emit_foreign_steps) {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:22168:29
      |
22168 | ...                   store::HistoryLane::Document,
      |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
22168 -                             store::HistoryLane::Document,
22168 +                             HistoryLane::Document,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:22184:175
      |
22184 | ...ion, mounted.meta.actor.clone(), mutation, None, store::HistoryLane::Document, self.config_one_item_factory.as_deref()) {
      |                                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
22184 -                         match self.config_store.begin_apply_one(mounted.operation.operation, mounted.config_generation, revision, mounted.meta.actor.clone(), mutation, None, store::HistoryLane::Document, self.config_one_item_factory.as_deref()) {
22184 +                         match self.config_store.begin_apply_one(mounted.operation.operation, mounted.config_generation, revision, mounted.meta.actor.clone(), mutation, None, HistoryLane::Document, self.config_one_item_factory.as_deref()) {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:22197:173
      |
22197 | ...ion, mounted.meta.actor.clone(), mutation, None, store::HistoryLane::Document, self.draft_one_item_factory.as_deref()) {
      |                                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
22197 -                         match self.draft_store.begin_apply_one(mounted.operation.operation, mounted.draft_generation, revision, mounted.meta.actor.clone(), mutation, None, store::HistoryLane::Document, self.draft_one_item_factory.as_deref()) {
22197 +                         match self.draft_store.begin_apply_one(mounted.operation.operation, mounted.draft_generation, revision, mounted.meta.actor.clone(), mutation, None, HistoryLane::Document, self.draft_one_item_factory.as_deref()) {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:24681:31
      |
24681 |         apps: HashMap<String, crate::app::declarations::AppFactory<PA>>,
      |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
24681 -         apps: HashMap<String, crate::app::declarations::AppFactory<PA>>,
24681 +         apps: HashMap<String, declarations::AppFactory<PA>>,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:24823:70
      |
24823 |         pub fn register_app_factory(mut self, mut app: App, factory: crate::app::declarations::AppFactory<PA>) -> Self {
      |                                                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
24823 -         pub fn register_app_factory(mut self, mut app: App, factory: crate::app::declarations::AppFactory<PA>) -> Self {
24823 +         pub fn register_app_factory(mut self, mut app: App, factory: declarations::AppFactory<PA>) -> Self {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:24946:27
      |
24946 |             let builder = ui::surface(props);
      |                           ^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
24946 -             let builder = ui::surface(props);
24946 +             let builder = surface(props);
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:24983:27
      |
24983 |             let builder = ui::surface(props);
      |                           ^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
24983 -             let builder = ui::surface(props);
24983 +             let builder = surface(props);
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:25228:28
      |
25228 |             let mut root = ui::column().try_id(Self::KIND_ID).map_err(|_| ui_assembly_error("table-rows-window.id"))?;
      |                            ^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
25228 -             let mut root = ui::column().try_id(Self::KIND_ID).map_err(|_| ui_assembly_error("table-rows-window.id"))?;
25228 +             let mut root = column().try_id(Self::KIND_ID).map_err(|_| ui_assembly_error("table-rows-window.id"))?;
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:25229:30
      |
25229 |             let mut header = ui::row().try_id("header").map_err(|_| ui_assembly_error("table-rows-window.header-id"))?;
      |                              ^^^^^^^
      |
help: remove the unnecessary path segments
      |
25229 -             let mut header = ui::row().try_id("header").map_err(|_| ui_assembly_error("table-rows-window.header-id"))?;
25229 +             let mut header = row().try_id("header").map_err(|_| ui_assembly_error("table-rows-window.header-id"))?;
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:25231:43
      |
25231 |                 header = header.try_child(ui::text(Label(column))).map_err(|_| ui_assembly_error("table-rows-window.header"))?;
      |                                           ^^^^^^^^
      |
help: remove the unnecessary path segments
      |
25231 -                 header = header.try_child(ui::text(Label(column))).map_err(|_| ui_assembly_error("table-rows-window.header"))?;
25231 +                 header = header.try_child(text(Label(column))).map_err(|_| ui_assembly_error("table-rows-window.header"))?;
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:25234:43
      |
25234 | ...   header = header.try_child(ui::text(Label(actions_label))).map_err(|_| ui_assembly_error("table-rows-window.actions-header"))?;
      |                                 ^^^^^^^^
      |
help: remove the unnecessary path segments
      |
25234 -                 header = header.try_child(ui::text(Label(actions_label))).map_err(|_| ui_assembly_error("table-rows-window.actions-header"))?;
25234 +                 header = header.try_child(text(Label(actions_label))).map_err(|_| ui_assembly_error("table-rows-window.actions-header"))?;
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:25239:31
      |
25239 |                 let mut row = ui::row().try_id(id.as_str()).map_err(|_| ui_assembly_error("table-rows-window.row-id"))?;
      |                               ^^^^^^^
      |
help: remove the unnecessary path segments
      |
25239 -                 let mut row = ui::row().try_id(id.as_str()).map_err(|_| ui_assembly_error("table-rows-window.row-id"))?;
25239 +                 let mut row = row().try_id(id.as_str()).map_err(|_| ui_assembly_error("table-rows-window.row-id"))?;
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:25241:41
      |
25241 |                     row = row.try_child(ui::text(Label(cell))).map_err(|_| ui_assembly_error("table-rows-window.cell"))?;
      |                                         ^^^^^^^^
      |
help: remove the unnecessary path segments
      |
25241 -                     row = row.try_child(ui::text(Label(cell))).map_err(|_| ui_assembly_error("table-rows-window.cell"))?;
25241 +                     row = row.try_child(text(Label(cell))).map_err(|_| ui_assembly_error("table-rows-window.cell"))?;
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:25246:34
      |
25246 |                     let button = ui::button(label).icon(icon);
      |                                  ^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
25246 -                     let button = ui::button(label).icon(icon);
25246 +                     let button = button(label).icon(icon);
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:25315:35
      |
25315 |             let section_builder = ui::tree_section(Label::default()).default_open(true);
      |                                   ^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
25315 -             let section_builder = ui::tree_section(Label::default()).default_open(true);
25315 +             let section_builder = tree_section(Label::default()).default_open(true);
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:25318:32
      |
25318 |             let tree_builder = ui::tree().try_id(Self::KIND_ID).map_err(|_| ui_assembly_error("tree-window.id"))?;
      |                                ^^^^^^^^
      |
help: remove the unnecessary path segments
      |
25318 -             let tree_builder = ui::tree().try_id(Self::KIND_ID).map_err(|_| ui_assembly_error("tree-window.id"))?;
25318 +             let tree_builder = tree().try_id(Self::KIND_ID).map_err(|_| ui_assembly_error("tree-window.id"))?;
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:25352:27
      |
25352 |             let builder = ui::image(src).alt(alt);
      |                           ^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
25352 -             let builder = ui::image(src).alt(alt);
25352 +             let builder = image(src).alt(alt);
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:25387:27
      |
25387 |             let builder = ui::surface(props);
      |                           ^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
25387 -             let builder = ui::surface(props);
25387 +             let builder = surface(props);
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:25423:31
      |
25423 |                 let builder = ui::text(ui_label(page.text.clone(), "document-window.page-text")?);
      |                               ^^^^^^^^
      |
help: remove the unnecessary path segments
      |
25423 -                 let builder = ui::text(ui_label(page.text.clone(), "document-window.page-text")?);
25423 +                 let builder = text(ui_label(page.text.clone(), "document-window.page-text")?);
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:25427:27
      |
25427 |             let builder = ui::column().try_id(Self::KIND_ID).map_err(|_| ui_assembly_error("document-window.id"))?;
      |                           ^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
25427 -             let builder = ui::column().try_id(Self::KIND_ID).map_err(|_| ui_assembly_error("document-window.id"))?;
25427 +             let builder = column().try_id(Self::KIND_ID).map_err(|_| ui_assembly_error("document-window.id"))?;
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:25597:64
      |
25597 | ...   assert_eq!(props, semio_framework_ui_scene::encode(semio_framework_ui_contract::SurfaceKind::TextEditor, &expected).expect(...
      |                                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
25597 -             assert_eq!(props, semio_framework_ui_scene::encode(semio_framework_ui_contract::SurfaceKind::TextEditor, &expected).expect("bounded fixture"));
25597 +             assert_eq!(props, semio_framework_ui_scene::encode(SurfaceKind::TextEditor, &expected).expect("bounded fixture"));
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:25623:64
      |
25623 | ...   assert_eq!(props, semio_framework_ui_scene::encode(semio_framework_ui_contract::SurfaceKind::TextEditor, &expected).expect(...
      |                                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
25623 -             assert_eq!(props, semio_framework_ui_scene::encode(semio_framework_ui_contract::SurfaceKind::TextEditor, &expected).expect("bounded fixture"));
25623 +             assert_eq!(props, semio_framework_ui_scene::encode(SurfaceKind::TextEditor, &expected).expect("bounded fixture"));
      |

warning: unused imports: `Component`, `Label`, `TextProps`, and `TreeNode`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:27275:134
      |
27275 | ...finition, ArtifactDialect, ArtifactEditor, ArtifactKindId, ArtifactPack, ArtifactView, ArtifactViewer, Component, ComponentTree, ConfigView, Dialect, DraftView, Editor, Emit, EngineHandles, Fault, Icon...
      |                                                                                                           ^^^^^^^^^
27276 | ...w, Label, LocalizedLabel, Mutation, MutationDiff, NoConfig, NoConfigMutation, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation, Plugin, StandardId, SubsetId, SurfaceKind, TextProps, TreeNode, V...
      |       ^^^^^                                                                                                                                                                           ^^^^^^^^^  ^^^^^^^^

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:27502:100
      |
27502 |                 M: Mutation<S> + PartialEq + Serialize + DeserializeOwned + Send + Sync + OpText + protocol::OpBinary + 'static,
      |                                                                                                    ^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
27502 -                 M: Mutation<S> + PartialEq + Serialize + DeserializeOwned + Send + Sync + OpText + protocol::OpBinary + 'static,
27502 +                 M: Mutation<S> + PartialEq + Serialize + DeserializeOwned + Send + Sync + OpText + OpBinary + 'static,
      |

warning: unused import: `crate::__semio_dispatch_PluginApp`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:27536:17
      |
27536 |             use crate::__semio_dispatch_PluginApp;
      |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unnecessary qualification
 --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🚪️lifetime/🧪️aggregate-admission.rs:6:18
  |
6 |     let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️aggregate-admission.json")).unwrap();
  |                  ^^^^^^^^^^^^^^^^^
  |
help: remove the unnecessary path segments
  |
6 -     let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️aggregate-admission.json")).unwrap();
6 +     let fixture: Value = serde_json::from_str(include_str!("🧪️aggregate-admission.json")).unwrap();
  |

warning: unnecessary qualification
  --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🚪️lifetime/🧪️aggregate-admission.rs:10:26
   |
10 |     let retained_bytes = std::mem::size_of_val(registry.slots.as_ref());
   |                          ^^^^^^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
10 -     let retained_bytes = std::mem::size_of_val(registry.slots.as_ref());
10 +     let retained_bytes = size_of_val(registry.slots.as_ref());
   |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:27984:26
      |
27984 | ...   let fixture: serde_json::Value = serde_json::from_str(include_str!("⚛️reactor/🧪️fixtures/📬️operation-continuation.json")).un...
      |                    ^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
27984 -             let fixture: serde_json::Value = serde_json::from_str(include_str!("⚛️reactor/🧪️fixtures/📬️operation-continuation.json")).unwrap();
27984 +             let fixture: Value = serde_json::from_str(include_str!("⚛️reactor/🧪️fixtures/📬️operation-continuation.json")).unwrap();
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:28016:41
      |
28016 |                     self.0.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
      |                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
28016 -                     self.0.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
28016 +                     self.0.fetch_add(1, Ordering::SeqCst);
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:28025:35
      |
28025 |             assert_eq!(drops.load(std::sync::atomic::Ordering::SeqCst), 0, "the close handoff must not run the nested destructor");
      |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
28025 -             assert_eq!(drops.load(std::sync::atomic::Ordering::SeqCst), 0, "the close handoff must not run the nested destructor");
28025 +             assert_eq!(drops.load(Ordering::SeqCst), 0, "the close handoff must not run the nested destructor");
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:28027:35
      |
28027 |             assert_eq!(drops.load(std::sync::atomic::Ordering::SeqCst), 0);
      |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
28027 -             assert_eq!(drops.load(std::sync::atomic::Ordering::SeqCst), 0);
28027 +             assert_eq!(drops.load(Ordering::SeqCst), 0);
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:28029:35
      |
28029 | ...   assert_eq!(drops.load(std::sync::atomic::Ordering::SeqCst), 0, "an incomplete registry shell must fail safe without walking...
      |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
28029 -             assert_eq!(drops.load(std::sync::atomic::Ordering::SeqCst), 0, "an incomplete registry shell must fail safe without walking or dropping nested values");
28029 +             assert_eq!(drops.load(Ordering::SeqCst), 0, "an incomplete registry shell must fail safe without walking or dropping nested values");
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:28037:41
      |
28037 |                     self.0.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
      |                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
28037 -                     self.0.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
28037 +                     self.0.fetch_add(1, Ordering::SeqCst);
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:28045:35
      |
28045 |             assert_eq!(drops.load(std::sync::atomic::Ordering::SeqCst), 0);
      |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
28045 -             assert_eq!(drops.load(std::sync::atomic::Ordering::SeqCst), 0);
28045 +             assert_eq!(drops.load(Ordering::SeqCst), 0);
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:28047:35
      |
28047 |             assert_eq!(drops.load(std::sync::atomic::Ordering::SeqCst), 1);
      |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
28047 -             assert_eq!(drops.load(std::sync::atomic::Ordering::SeqCst), 1);
28047 +             assert_eq!(drops.load(Ordering::SeqCst), 1);
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:28081:31
      |
28081 |         close_cleanup_cursor: std::cell::Cell<usize>,
      |                               ^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
28081 -         close_cleanup_cursor: std::cell::Cell<usize>,
28081 +         close_cleanup_cursor: Cell<usize>,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:28082:30
      |
28082 |         live_cleanup_cursor: std::cell::Cell<usize>,
      |                              ^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
28082 -         live_cleanup_cursor: std::cell::Cell<usize>,
28082 +         live_cleanup_cursor: Cell<usize>,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:28083:36
      |
28083 |         typed_continuation_cursor: std::cell::Cell<usize>,
      |                                    ^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
28083 -         typed_continuation_cursor: std::cell::Cell<usize>,
28083 +         typed_continuation_cursor: Cell<usize>,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:28084:27
      |
28084 |         close_generation: std::cell::Cell<u64>,
      |                           ^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
28084 -         close_generation: std::cell::Cell<u64>,
28084 +         close_generation: Cell<u64>,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:28097:39
      |
28097 |                 close_cleanup_cursor: std::cell::Cell::new(0),
      |                                       ^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
28097 -                 close_cleanup_cursor: std::cell::Cell::new(0),
28097 +                 close_cleanup_cursor: Cell::new(0),
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:28098:38
      |
28098 |                 live_cleanup_cursor: std::cell::Cell::new(0),
      |                                      ^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
28098 -                 live_cleanup_cursor: std::cell::Cell::new(0),
28098 +                 live_cleanup_cursor: Cell::new(0),
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:28099:44
      |
28099 |                 typed_continuation_cursor: std::cell::Cell::new(0),
      |                                            ^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
28099 -                 typed_continuation_cursor: std::cell::Cell::new(0),
28099 +                 typed_continuation_cursor: Cell::new(0),
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:28100:35
      |
28100 |                 close_generation: std::cell::Cell::new(0),
      |                                   ^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
28100 -                 close_generation: std::cell::Cell::new(0),
28100 +                 close_generation: Cell::new(0),
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:29250:49
      |
29250 |         static RUNTIME_CLOSE_CONSTRUCTION_LIVE: std::cell::Cell<Option<bool>> = const { std::cell::Cell::new(None) };
      |                                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
29250 -         static RUNTIME_CLOSE_CONSTRUCTION_LIVE: std::cell::Cell<Option<bool>> = const { std::cell::Cell::new(None) };
29250 +         static RUNTIME_CLOSE_CONSTRUCTION_LIVE: Cell<Option<bool>> = const { std::cell::Cell::new(None) };
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:29250:89
      |
29250 |         static RUNTIME_CLOSE_CONSTRUCTION_LIVE: std::cell::Cell<Option<bool>> = const { std::cell::Cell::new(None) };
      |                                                                                         ^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
29250 -         static RUNTIME_CLOSE_CONSTRUCTION_LIVE: std::cell::Cell<Option<bool>> = const { std::cell::Cell::new(None) };
29250 +         static RUNTIME_CLOSE_CONSTRUCTION_LIVE: std::cell::Cell<Option<bool>> = const { Cell::new(None) };
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:29251:49
      |
29251 |         static RUNTIME_CLOSE_CONSTRUCTION_FAIL: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
      |                                                 ^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
29251 -         static RUNTIME_CLOSE_CONSTRUCTION_FAIL: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
29251 +         static RUNTIME_CLOSE_CONSTRUCTION_FAIL: Cell<bool> = const { std::cell::Cell::new(false) };
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:29251:81
      |
29251 |         static RUNTIME_CLOSE_CONSTRUCTION_FAIL: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
      |                                                                                 ^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
29251 -         static RUNTIME_CLOSE_CONSTRUCTION_FAIL: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
29251 +         static RUNTIME_CLOSE_CONSTRUCTION_FAIL: std::cell::Cell<bool> = const { Cell::new(false) };
      |

warning: unused doc comment
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34363:9
      |
34363 | /         /// 🧬️ dedyn-fw-os-spacemember: the closed-set `M` these composition tests register/open
34364 | |         /// children through — `store::space_members!`'s generated `SpaceMember`/`MemberFactory`
34365 | |         /// match-delegation over ONE variant, replacing the old `Box<dyn SpaceMember>` erasure (see
34366 | |         /// `📓️terra-dedyn-fw-os-spacemember-report.md`).
      | |_________------------------------------------------------^
      |           |
      |           rustdoc does not generate documentation for macro invocations
      |
      = help: to document an item produced by a macro, the macro must produce the documentation as part of its expansion

warning: unused import: `Label`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32025:29
      |
32025 |         use ui_wgpu::wgpu::{Label, LocalizedLabel};
      |                             ^^^^^

warning: unused import: `SurfaceKind`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32035:78
      |
32035 |         use crate::{selection_count_phrase, IconName, MediaClass, MediaType, SurfaceKind, ViewModel};
      |                                                                              ^^^^^^^^^^^

warning: unused import: `MutationDiff`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32036:34
      |
32036 |         use protocol::{Mutation, MutationDiff};
      |                                  ^^^^^^^^^^^^

warning: unnecessary qualification
  --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🕹️interaction/📡️live/🧪️dispatch/🧪️component.rs:23:17
   |
23 |         status: std::sync::atomic::AtomicU8::new(RUNTIME_CLOSE_QUEUED), stalled_steps: std::sync::atomic::AtomicU8::new(0),
   |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
23 -         status: std::sync::atomic::AtomicU8::new(RUNTIME_CLOSE_QUEUED), stalled_steps: std::sync::atomic::AtomicU8::new(0),
23 +         status: AtomicU8::new(RUNTIME_CLOSE_QUEUED), stalled_steps: std::sync::atomic::AtomicU8::new(0),
   |

warning: unnecessary qualification
  --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🕹️interaction/📡️live/🧪️dispatch/🧪️component.rs:23:88
   |
23 |         status: std::sync::atomic::AtomicU8::new(RUNTIME_CLOSE_QUEUED), stalled_steps: std::sync::atomic::AtomicU8::new(0),
   |                                                                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
23 -         status: std::sync::atomic::AtomicU8::new(RUNTIME_CLOSE_QUEUED), stalled_steps: std::sync::atomic::AtomicU8::new(0),
23 +         status: std::sync::atomic::AtomicU8::new(RUNTIME_CLOSE_QUEUED), stalled_steps: AtomicU8::new(0),
   |

warning: unnecessary qualification
  --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🕹️interaction/📡️live/🧪️dispatch/🧪️component.rs:24:27
   |
24 |         preview_sequence: std::sync::atomic::AtomicU64::new(0), last_callback_elapsed_us: std::sync::atomic::AtomicU64::new(0),
   |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
24 -         preview_sequence: std::sync::atomic::AtomicU64::new(0), last_callback_elapsed_us: std::sync::atomic::AtomicU64::new(0),
24 +         preview_sequence: AtomicU64::new(0), last_callback_elapsed_us: std::sync::atomic::AtomicU64::new(0),
   |

warning: unnecessary qualification
  --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🕹️interaction/📡️live/🧪️dispatch/🧪️component.rs:24:91
   |
24 |         preview_sequence: std::sync::atomic::AtomicU64::new(0), last_callback_elapsed_us: std::sync::atomic::AtomicU64::new(0),
   |                                                                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
24 -         preview_sequence: std::sync::atomic::AtomicU64::new(0), last_callback_elapsed_us: std::sync::atomic::AtomicU64::new(0),
24 +         preview_sequence: std::sync::atomic::AtomicU64::new(0), last_callback_elapsed_us: AtomicU64::new(0),
   |

warning: unnecessary qualification
  --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🕹️interaction/📡️live/🧪️dispatch/🧪️component.rs:25:73
   |
25 |         last_fault: std::sync::Mutex::new([0; 256]), last_fault_origin: std::sync::atomic::AtomicU8::new(0),
   |                                                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
25 -         last_fault: std::sync::Mutex::new([0; 256]), last_fault_origin: std::sync::atomic::AtomicU8::new(0),
25 +         last_fault: std::sync::Mutex::new([0; 256]), last_fault_origin: AtomicU8::new(0),
   |

warning: unnecessary qualification
  --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🕹️interaction/📡️live/🧪️dispatch/🧪️component.rs:26:36
   |
26 | ...   callback_phase_started_us: std::sync::atomic::AtomicU64::new(0), callback_phase_us: std::array::from_fn(|_| std::sync::atomic:...
   |                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
26 -         callback_phase_started_us: std::sync::atomic::AtomicU64::new(0), callback_phase_us: std::array::from_fn(|_| std::sync::atomic::AtomicU64::new(0)),
26 +         callback_phase_started_us: AtomicU64::new(0), callback_phase_us: std::array::from_fn(|_| std::sync::atomic::AtomicU64::new(0)),
   |

warning: unnecessary qualification
  --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🕹️interaction/📡️live/🧪️dispatch/🧪️component.rs:26:117
   |
26 | ...(0), callback_phase_us: std::array::from_fn(|_| std::sync::atomic::AtomicU64::new(0)),
   |                                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
26 -         callback_phase_started_us: std::sync::atomic::AtomicU64::new(0), callback_phase_us: std::array::from_fn(|_| std::sync::atomic::AtomicU64::new(0)),
26 +         callback_phase_started_us: std::sync::atomic::AtomicU64::new(0), callback_phase_us: std::array::from_fn(|_| AtomicU64::new(0)),
   |

warning: unnecessary qualification
  --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🕹️interaction/📡️live/🧪️dispatch/🧪️component.rs:33:18
   |
33 |     let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../🔨️modules/🎭️actor/🚪️lifetime/🧪️fault.fi...
   |                  ^^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
33 -     let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../🔨️modules/🎭️actor/🚪️lifetime/🧪️fault.fixture.json")).unwrap();
33 +     let fixture: Value = serde_json::from_str(include_str!("../../../../../../../🔨️modules/🎭️actor/🚪️lifetime/🧪️fault.fixture.json")).unwrap();
   |

warning: unnecessary qualification
  --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🕹️interaction/📡️live/🧪️dispatch/🧪️component.rs:36:34
   |
36 |     assert_eq!(state.status.load(std::sync::atomic::Ordering::SeqCst) == RUNTIME_CLOSE_COMPLETE, fixture["owners"]["terminalVisibleB...
   |                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
36 -     assert_eq!(state.status.load(std::sync::atomic::Ordering::SeqCst) == RUNTIME_CLOSE_COMPLETE, fixture["owners"]["terminalVisibleBeforeWatchdog"].as_bool().unwrap());
36 +     assert_eq!(state.status.load(Ordering::SeqCst) == RUNTIME_CLOSE_COMPLETE, fixture["owners"]["terminalVisibleBeforeWatchdog"].as_bool().unwrap());
   |

warning: unnecessary qualification
  --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🕹️interaction/📡️live/🧪️dispatch/🧪️component.rs:39:51
   |
39 |         state.status.store(RUNTIME_CLOSE_RUNNING, std::sync::atomic::Ordering::SeqCst);
   |                                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
39 -         state.status.store(RUNTIME_CLOSE_RUNNING, std::sync::atomic::Ordering::SeqCst);
39 +         state.status.store(RUNTIME_CLOSE_RUNNING, Ordering::SeqCst);
   |

warning: unnecessary qualification
  --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🕹️interaction/📡️live/🧪️dispatch/🧪️component.rs:41:38
   |
41 |         assert_eq!(state.status.load(std::sync::atomic::Ordering::SeqCst), status(row["published"].as_str().unwrap()));
   |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
41 -         assert_eq!(state.status.load(std::sync::atomic::Ordering::SeqCst), status(row["published"].as_str().unwrap()));
41 +         assert_eq!(state.status.load(Ordering::SeqCst), status(row["published"].as_str().unwrap()));
   |

warning: unnecessary qualification
  --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🕹️interaction/📡️live/🧪️dispatch/🧪️component.rs:42:56
   |
42 |         assert_eq!(state.last_callback_elapsed_us.load(std::sync::atomic::Ordering::SeqCst), row["elapsedUs"].as_u64().unwrap());
   |                                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
42 -         assert_eq!(state.last_callback_elapsed_us.load(std::sync::atomic::Ordering::SeqCst), row["elapsedUs"].as_u64().unwrap());
42 +         assert_eq!(state.last_callback_elapsed_us.load(Ordering::SeqCst), row["elapsedUs"].as_u64().unwrap());
   |

warning: unnecessary qualification
  --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🕹️interaction/📡️live/🧪️dispatch/🧪️component.rs:49:18
   |
49 |     let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../🔨️modules/🎭️actor/🚪️lifetime/🧪️fault.fi...
   |                  ^^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
49 -     let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../🔨️modules/🎭️actor/🚪️lifetime/🧪️fault.fixture.json")).unwrap();
49 +     let fixture: Value = serde_json::from_str(include_str!("../../../../../../../🔨️modules/🎭️actor/🚪️lifetime/🧪️fault.fixture.json")).unwrap();
   |

warning: unnecessary qualification
  --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🕹️interaction/📡️live/🧪️dispatch/🧪️component.rs:61:18
   |
61 |     let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../🔨️modules/🎭️actor/🚪️lifetime/🧪️fault.fi...
   |                  ^^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
61 -     let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../🔨️modules/🎭️actor/🚪️lifetime/🧪️fault.fixture.json")).unwrap();
61 +     let fixture: Value = serde_json::from_str(include_str!("../../../../../../../🔨️modules/🎭️actor/🚪️lifetime/🧪️fault.fixture.json")).unwrap();
   |

warning: unnecessary qualification
  --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🕹️interaction/📡️live/🧪️dispatch/🧪️component.rs:64:73
   |
64 |         let mut samples = row["samples"].as_array().unwrap().iter().map(serde_json::Value::as_u64);
   |                                                                         ^^^^^^^^^^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
64 -         let mut samples = row["samples"].as_array().unwrap().iter().map(serde_json::Value::as_u64);
64 +         let mut samples = row["samples"].as_array().unwrap().iter().map(Value::as_u64);
   |

warning: unnecessary qualification
  --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🕹️interaction/📡️live/🧪️dispatch/🧪️component.rs:67:38
   |
67 |         assert_eq!(state.status.load(std::sync::atomic::Ordering::SeqCst), expected, "{row}");
   |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
67 -         assert_eq!(state.status.load(std::sync::atomic::Ordering::SeqCst), expected, "{row}");
67 +         assert_eq!(state.status.load(Ordering::SeqCst), expected, "{row}");
   |

warning: unnecessary qualification
  --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🕹️interaction/📡️live/🧪️dispatch/🧪️component.rs:80:18
   |
80 |     let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../🔨️modules/🎭️actor/🚪️lifetime/🧪️fault.fi...
   |                  ^^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
80 -     let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../🔨️modules/🎭️actor/🚪️lifetime/🧪️fault.fixture.json")).unwrap();
80 +     let fixture: Value = serde_json::from_str(include_str!("../../../../../../../🔨️modules/🎭️actor/🚪️lifetime/🧪️fault.fixture.json")).unwrap();
   |

warning: unnecessary qualification
  --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🕹️interaction/📡️live/🧪️dispatch/🧪️component.rs:81:56
   |
81 |     let cell = std::sync::Arc::new(RuntimeAppCell::new(crate::app::AppInstance { id: 7, app: TestRuntimeApps::from(query_app().await...
   |                                                        ^^^^^^^^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
81 -     let cell = std::sync::Arc::new(RuntimeAppCell::new(crate::app::AppInstance { id: 7, app: TestRuntimeApps::from(query_app().await) }));
81 +     let cell = std::sync::Arc::new(RuntimeAppCell::new(AppInstance { id: 7, app: TestRuntimeApps::from(query_app().await) }));
   |

warning: unnecessary qualification
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🕹️interaction/📡️live/🧪️dispatch/🧪️component.rs:102:18
    |
102 |     let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../🔨️modules/🎭️actor/🚪️lifetime/🧪️fault.f...
    |                  ^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
102 -     let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../🔨️modules/🎭️actor/🚪️lifetime/🧪️fault.fixture.json")).unwrap();
102 +     let fixture: Value = serde_json::from_str(include_str!("../../../../../../../🔨️modules/🎭️actor/🚪️lifetime/🧪️fault.fixture.json")).unwrap();
    |

warning: unnecessary qualification
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🕹️interaction/📡️live/🧪️dispatch/🧪️component.rs:131:36
    |
131 |     let status = state.status.load(std::sync::atomic::Ordering::SeqCst);
    |                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
131 -     let status = state.status.load(std::sync::atomic::Ordering::SeqCst);
131 +     let status = state.status.load(Ordering::SeqCst);
    |

warning: unnecessary qualification
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🕹️interaction/📡️live/🧪️dispatch/🧪️component.rs:150:70
    |
150 |     let cell = std::sync::Arc::new(super::super::RuntimeAppCell::new(crate::app::AppInstance { id: 7, app: TestRuntimeApps::from(qu...
    |                                                                      ^^^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
150 -     let cell = std::sync::Arc::new(super::super::RuntimeAppCell::new(crate::app::AppInstance { id: 7, app: TestRuntimeApps::from(query_app().await) }));
150 +     let cell = std::sync::Arc::new(super::super::RuntimeAppCell::new(AppInstance { id: 7, app: TestRuntimeApps::from(query_app().await) }));
    |

warning: unnecessary qualification
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🕹️interaction/📡️live/🧪️dispatch/🧪️component.rs:173:18
    |
173 |     let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../🔨️modules/🎭️actor/🚪️lifetime/🧪️fixture...
    |                  ^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
173 -     let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../🔨️modules/🎭️actor/🚪️lifetime/🧪️fixture.json")).unwrap();
173 +     let fixture: Value = serde_json::from_str(include_str!("../../../../../../../🔨️modules/🎭️actor/🚪️lifetime/🧪️fixture.json")).unwrap();
    |

warning: unnecessary qualification
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🕹️interaction/📡️live/🧪️dispatch/🧪️component.rs:193:18
    |
193 |     let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🚪️lifetime/🧪️construction.json")).unwrap();
    |                  ^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
193 -     let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🚪️lifetime/🧪️construction.json")).unwrap();
193 +     let fixture: Value = serde_json::from_str(include_str!("../../../🚪️lifetime/🧪️construction.json")).unwrap();
    |

warning: unnecessary qualification
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🕹️interaction/📡️live/🧪️dispatch/🧪️component.rs:210:18
    |
210 |     let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../🔨️modules/🎭️actor/🚪️lifetime/🧪️fixture...
    |                  ^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
210 -     let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../🔨️modules/🎭️actor/🚪️lifetime/🧪️fixture.json")).unwrap();
210 +     let fixture: Value = serde_json::from_str(include_str!("../../../../../../../🔨️modules/🎭️actor/🚪️lifetime/🧪️fixture.json")).unwrap();
    |

warning: unnecessary qualification
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🕹️interaction/📡️live/🧪️dispatch/🧪️component.rs:233:18
    |
233 |     let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🚪️lifetime/🧪️construction.json")).unwrap();
    |                  ^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
233 -     let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🚪️lifetime/🧪️construction.json")).unwrap();
233 +     let fixture: Value = serde_json::from_str(include_str!("../../../🚪️lifetime/🧪️construction.json")).unwrap();
    |

warning: unnecessary qualification
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🕹️interaction/📡️live/🧪️dispatch/🧪️component.rs:286:70
    |
286 |     let cell = std::sync::Arc::new(super::super::RuntimeAppCell::new(crate::app::AppInstance { id: 7, app: TestRuntimeApps::from(qu...
    |                                                                      ^^^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
286 -     let cell = std::sync::Arc::new(super::super::RuntimeAppCell::new(crate::app::AppInstance { id: 7, app: TestRuntimeApps::from(query_app().await) }));
286 +     let cell = std::sync::Arc::new(super::super::RuntimeAppCell::new(AppInstance { id: 7, app: TestRuntimeApps::from(query_app().await) }));
    |

warning: unnecessary qualification
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🕹️interaction/📡️live/🧪️dispatch/🧪️component.rs:321:18
    |
321 |     let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../🔨️modules/📡️replication/📡️wire/🏠️local...
    |                  ^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
321 -     let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../🔨️modules/📡️replication/📡️wire/🏠️local-interaction/🧪️fixtures/🔣️local-interaction.json")).unwrap();
321 +     let fixture: Value = serde_json::from_str(include_str!("../../../../../../../🔨️modules/📡️replication/📡️wire/🏠️local-interaction/🧪️fixtures/🔣️local-interaction.json")).unwrap();
    |

warning: unnecessary qualification
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🕹️interaction/📡️live/🧪️dispatch/🧪️component.rs:324:16
    |
324 |     let state: protocol::InteractionState = serde_json::from_value(state).unwrap();
    |                ^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
324 -     let state: protocol::InteractionState = serde_json::from_value(state).unwrap();
324 +     let state: InteractionState = serde_json::from_value(state).unwrap();
    |

warning: unnecessary qualification
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🕹️interaction/📡️live/🧪️dispatch/🧪️component.rs:325:54
    |
325 |     let envelope = store::create_document_envelope::<protocol::InteractionState, crate::app::InteractionConfigMutation>("framework....
    |                                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
325 -     let envelope = store::create_document_envelope::<protocol::InteractionState, crate::app::InteractionConfigMutation>("framework.interaction", "query-dispatch", state, None);
325 +     let envelope = store::create_document_envelope::<InteractionState, crate::app::InteractionConfigMutation>("framework.interaction", "query-dispatch", state, None);
    |

warning: unnecessary qualification
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🕹️interaction/📡️live/🧪️dispatch/🧪️component.rs:325:82
    |
325 |     let envelope = store::create_document_envelope::<protocol::InteractionState, crate::app::InteractionConfigMutation>("framework....
    |                                                                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
325 -     let envelope = store::create_document_envelope::<protocol::InteractionState, crate::app::InteractionConfigMutation>("framework.interaction", "query-dispatch", state, None);
325 +     let envelope = store::create_document_envelope::<protocol::InteractionState, InteractionConfigMutation>("framework.interaction", "query-dispatch", state, None);
    |

warning: unnecessary qualification
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🕹️interaction/📡️live/🧪️dispatch/🧪️component.rs:339:70
    |
339 |     let cell = std::sync::Arc::new(super::super::RuntimeAppCell::new(crate::app::AppInstance { id: 7, app: TestRuntimeApps::from(qu...
    |                                                                      ^^^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
339 -     let cell = std::sync::Arc::new(super::super::RuntimeAppCell::new(crate::app::AppInstance { id: 7, app: TestRuntimeApps::from(query_app().await) }));
339 +     let cell = std::sync::Arc::new(super::super::RuntimeAppCell::new(AppInstance { id: 7, app: TestRuntimeApps::from(query_app().await) }));
    |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32067:26
      |
32067 | ...   let fixture: serde_json::Value = serde_json::from_str(include_str!("🧵️retained-command/🧪️fixtures/🧬️request-context.json"))...
      |                    ^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32067 -             let fixture: serde_json::Value = serde_json::from_str(include_str!("🧵️retained-command/🧪️fixtures/🧬️request-context.json")).expect("request context fixture");
32067 +             let fixture: Value = serde_json::from_str(include_str!("🧵️retained-command/🧪️fixtures/🧬️request-context.json")).expect("request context fixture");
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32075:28
      |
32075 |             let identity = crate::app::test_artifact_owned_tool_job_context_identity_digest;
      |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32075 -             let identity = crate::app::test_artifact_owned_tool_job_context_identity_digest;
32075 +             let identity = test_artifact_owned_tool_job_context_identity_digest;
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32092:13
      |
32092 |             semio_framework::surface_app_id(&TEST_APP_DIALECT.into(), semio_framework::AppRole::Editor)
      |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32092 -             semio_framework::surface_app_id(&TEST_APP_DIALECT.into(), semio_framework::AppRole::Editor)
32092 +             surface_app_id(&TEST_APP_DIALECT.into(), semio_framework::AppRole::Editor)
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32092:71
      |
32092 |             semio_framework::surface_app_id(&TEST_APP_DIALECT.into(), semio_framework::AppRole::Editor)
      |                                                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32092 -             semio_framework::surface_app_id(&TEST_APP_DIALECT.into(), semio_framework::AppRole::Editor)
32092 +             semio_framework::surface_app_id(&TEST_APP_DIALECT.into(), AppRole::Editor)
      |

warning: unused import: `TestDiff`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32097:48
      |
32097 |         use crate::test_app_mutation_fixture::{TestDiff, TestSnapshot};
      |                                                ^^^^^^^^

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32194:148
      |
32194 | ...dencies: Vec::new(), base_version: 0, author_id: Some(protocol::ActorId(authority.actor().into())), timestamp: authority.next_...
      |                                                          ^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32194 -                         mutation_meta: vec![protocol::MutationMeta { mutation_id: None, dependencies: Vec::new(), base_version: 0, author_id: Some(protocol::ActorId(authority.actor().into())), timestamp: authority.next_clock(),
32194 +                         mutation_meta: vec![protocol::MutationMeta { mutation_id: None, dependencies: Vec::new(), base_version: 0, author_id: Some(ActorId(authority.actor().into())), timestamp: authority.next_clock(),
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32195:42
      |
32195 | ...   undo_policy: protocol::UndoPolicy::ExactBaseOnly, payload_hash: None, semantic_kind: None, label: None, group_id: None, ori...
      |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32195 -                             undo_policy: protocol::UndoPolicy::ExactBaseOnly, payload_hash: None, semantic_kind: None, label: None, group_id: None, origin: Default::default() }],
32195 +                             undo_policy: UndoPolicy::ExactBaseOnly, payload_hash: None, semantic_kind: None, label: None, group_id: None, origin: Default::default() }],
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32348:95
      |
32348 | ...   fn extent(&self, _command: &TestCommand, _snapshot: &TestSnapshot, _interaction: &protocol::InteractionState, _context: Opt...
      |                                                                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32348 -             fn extent(&self, _command: &TestCommand, _snapshot: &TestSnapshot, _interaction: &protocol::InteractionState, _context: Option<&crate::app::ArtifactOwnedToolJobContext<TestApp>>) -> Option<usize> {
32348 +             fn extent(&self, _command: &TestCommand, _snapshot: &TestSnapshot, _interaction: &InteractionState, _context: Option<&crate::app::ArtifactOwnedToolJobContext<TestApp>>) -> Option<usize> {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32348:141
      |
32348 | ...l::InteractionState, _context: Option<&crate::app::ArtifactOwnedToolJobContext<TestApp>>) -> Option<usize> {
      |                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32348 -             fn extent(&self, _command: &TestCommand, _snapshot: &TestSnapshot, _interaction: &protocol::InteractionState, _context: Option<&crate::app::ArtifactOwnedToolJobContext<TestApp>>) -> Option<usize> {
32348 +             fn extent(&self, _command: &TestCommand, _snapshot: &TestSnapshot, _interaction: &protocol::InteractionState, _context: Option<&ArtifactOwnedToolJobContext<TestApp>>) -> Option<usize> {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32358:32
      |
32358 |                 _interaction: &protocol::InteractionState,
      |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32358 -                 _interaction: &protocol::InteractionState,
32358 +                 _interaction: &InteractionState,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32360:35
      |
32360 |                 _context: Option<&crate::app::ArtifactOwnedToolJobContext<TestApp>>,
      |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32360 -                 _context: Option<&crate::app::ArtifactOwnedToolJobContext<TestApp>>,
32360 +                 _context: Option<&ArtifactOwnedToolJobContext<TestApp>>,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32361:30
      |
32361 |                 _operation: &crate::app::AppOperationContext,
      |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32361 -                 _operation: &crate::app::AppOperationContext,
32361 +                 _operation: &AppOperationContext,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32388:23
      |
32388 |             keys: Vec<semio_framework::ToolFactoryKey>,
      |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32388 -             keys: Vec<semio_framework::ToolFactoryKey>,
32388 +             keys: Vec<ToolFactoryKey>,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32393:35
      |
32393 | ...   Self { keys: vec![semio_framework::ToolFactoryKey::new(TEST_RETAINED_COMMAND_CONTROLLER, TEST_RETAINED_COMMAND_TOOL)] }
      |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32393 -                 Self { keys: vec![semio_framework::ToolFactoryKey::new(TEST_RETAINED_COMMAND_CONTROLLER, TEST_RETAINED_COMMAND_TOOL)] }
32393 +                 Self { keys: vec![ToolFactoryKey::new(TEST_RETAINED_COMMAND_CONTROLLER, TEST_RETAINED_COMMAND_TOOL)] }
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32399:14
      |
32399 |         impl semio_framework::ToolJobFactory for OtherTestRetainedCommandFactory {
      |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32399 -         impl semio_framework::ToolJobFactory for OtherTestRetainedCommandFactory {
32399 +         impl ToolJobFactory for OtherTestRetainedCommandFactory {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32402:33
      |
32402 |             fn keys(&self) -> &[semio_framework::ToolFactoryKey] { &self.0.keys }
      |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32402 -             fn keys(&self) -> &[semio_framework::ToolFactoryKey] { &self.0.keys }
32402 +             fn keys(&self) -> &[ToolFactoryKey] { &self.0.keys }
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32404:41
      |
32404 | ...   fn classification(&self) -> semio_framework::InteractiveJobClassification { semio_framework::InteractiveJobClassification::...
      |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32404 -             fn classification(&self) -> semio_framework::InteractiveJobClassification { semio_framework::InteractiveJobClassification::Migrated }
32404 +             fn classification(&self) -> InteractiveJobClassification { semio_framework::InteractiveJobClassification::Migrated }
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32404:89
      |
32404 | ...rk::InteractiveJobClassification { semio_framework::InteractiveJobClassification::Migrated }
      |                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32404 -             fn classification(&self) -> semio_framework::InteractiveJobClassification { semio_framework::InteractiveJobClassification::Migrated }
32404 +             fn classification(&self) -> semio_framework::InteractiveJobClassification { InteractiveJobClassification::Migrated }
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32405:45
      |
32405 | ...   fn execution_contract(&self) -> semio_framework::ToolExecutionContract { semio_framework::ToolJobFactory::execution_contrac...
      |                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32405 -             fn execution_contract(&self) -> semio_framework::ToolExecutionContract { semio_framework::ToolJobFactory::execution_contract(&self.0) }
32405 +             fn execution_contract(&self) -> ToolExecutionContract { semio_framework::ToolJobFactory::execution_contract(&self.0) }
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32405:86
      |
32405 | ...o_framework::ToolExecutionContract { semio_framework::ToolJobFactory::execution_contract(&self.0) }
      |                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32405 -             fn execution_contract(&self) -> semio_framework::ToolExecutionContract { semio_framework::ToolJobFactory::execution_contract(&self.0) }
32405 +             fn execution_contract(&self) -> semio_framework::ToolExecutionContract { ToolJobFactory::execution_contract(&self.0) }
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32406:126
      |
32406 | ...payload: Self::Payload) -> Result<Self::Job, semio_framework::ToolJobFactoryError> {
      |                                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32406 -             fn create_job(&mut self, operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, semio_framework::ToolJobFactoryError> {
32406 +             fn create_job(&mut self, operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32407:17
      |
32407 |                 semio_framework::ToolJobFactory::create_job(&mut self.0, operation, payload)
      |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32407 -                 semio_framework::ToolJobFactory::create_job(&mut self.0, operation, payload)
32407 +                 ToolJobFactory::create_job(&mut self.0, operation, payload)
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32425:14
      |
32425 |         impl semio_framework::ToolJobFactory for TestRetainedCommandFactory {
      |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32425 -         impl semio_framework::ToolJobFactory for TestRetainedCommandFactory {
32425 +         impl ToolJobFactory for TestRetainedCommandFactory {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32429:33
      |
32429 |             fn keys(&self) -> &[semio_framework::ToolFactoryKey] {
      |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32429 -             fn keys(&self) -> &[semio_framework::ToolFactoryKey] {
32429 +             fn keys(&self) -> &[ToolFactoryKey] {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32437:41
      |
32437 |             fn classification(&self) -> semio_framework::InteractiveJobClassification {
      |                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32437 -             fn classification(&self) -> semio_framework::InteractiveJobClassification {
32437 +             fn classification(&self) -> InteractiveJobClassification {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32438:17
      |
32438 |                 semio_framework::InteractiveJobClassification::Migrated
      |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32438 -                 semio_framework::InteractiveJobClassification::Migrated
32438 +                 InteractiveJobClassification::Migrated
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32441:45
      |
32441 |             fn execution_contract(&self) -> semio_framework::ToolExecutionContract {
      |                                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32441 -             fn execution_contract(&self) -> semio_framework::ToolExecutionContract {
32441 +             fn execution_contract(&self) -> ToolExecutionContract {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32442:17
      |
32442 | ...   semio_framework::ToolExecutionContract::resumable(TEST_RETAINED_COMMAND_RAW_BYTES, 4, 1, TEST_RETAINED_COMMAND_RAW_BYTES, 7...
      |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32442 -                 semio_framework::ToolExecutionContract::resumable(TEST_RETAINED_COMMAND_RAW_BYTES, 4, 1, TEST_RETAINED_COMMAND_RAW_BYTES, 7_500, 1, 1)
32442 +                 ToolExecutionContract::resumable(TEST_RETAINED_COMMAND_RAW_BYTES, 4, 1, TEST_RETAINED_COMMAND_RAW_BYTES, 7_500, 1, 1)
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32445:127
      |
32445 | ...payload: Self::Payload) -> Result<Self::Job, semio_framework::ToolJobFactoryError> {
      |                                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32445 -             fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, semio_framework::ToolJobFactoryError> {
32445 +             fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32453:24
      |
32453 |                 input: semio_framework::action_bus::RetainedToolWireInput,
      |                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32453 -                 input: semio_framework::action_bus::RetainedToolWireInput,
32453 +                 input: action_bus::RetainedToolWireInput,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32454:36
      |
32454 |                 checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>,
      |                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32454 -                 checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>,
32454 +                 checkpoint: Option<action_bus::RetainedToolWireInput>,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32455:37
      |
32455 | ...   ) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<se...
      |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32455 -             ) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
32455 +             ) -> Result<Self::Job, (ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32455:75
      |
32455 | ...   ) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<se...
      |                                                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32455 -             ) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
32455 +             ) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32455:134
      |
32455 | ...on_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
      |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32455 -             ) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
32455 +             ) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<action_bus::RetainedToolWireInput>)> {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32457:33
      |
32457 |                     return Err((semio_framework::ToolJobFactoryError::new("test retained command extent"), input, checkpoint));
      |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32457 -                     return Err((semio_framework::ToolJobFactoryError::new("test retained command extent"), input, checkpoint));
32457 +                     return Err((ToolJobFactoryError::new("test retained command extent"), input, checkpoint));
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32473:60
      |
32473 | ...   async fn test_retained_command_payload(completion: crate::app::ArtifactToolCompletion<TestApp>) -> crate::retained_command:...
      |                                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32473 -         async fn test_retained_command_payload(completion: crate::app::ArtifactToolCompletion<TestApp>) -> crate::retained_command::ArtifactRetainedCommandPayload<TestApp> {
32473 +         async fn test_retained_command_payload(completion: ArtifactToolCompletion<TestApp>) -> crate::retained_command::ArtifactRetainedCommandPayload<TestApp> {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32479:37
      |
32479 |                 std::sync::Arc::new(protocol::InteractionState::default()),
      |                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32479 -                 std::sync::Arc::new(protocol::InteractionState::default()),
32479 +                 std::sync::Arc::new(InteractionState::default()),
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32481:17
      |
32481 | ...   crate::app::AppOperationContext { app_instance_id: 7, parent_document_id: "test-document".into(), operation_id: 41, generat...
      |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32481 -                 crate::app::AppOperationContext { app_instance_id: 7, parent_document_id: "test-document".into(), operation_id: 41, generation: 3, canonical_base_revision: [5; 32] },
32481 +                 AppOperationContext { app_instance_id: 7, parent_document_id: "test-document".into(), operation_id: 41, generation: 3, canonical_base_revision: [5; 32] },
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32491:43
      |
32491 | ...   fn test_retained_wire_input(bus: &semio_framework::ActionBus, bytes: &[u8]) -> (semio_framework::ToolWireAdmission, semio_f...
      |                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32491 -         fn test_retained_wire_input(bus: &semio_framework::ActionBus, bytes: &[u8]) -> (semio_framework::ToolWireAdmission, semio_framework::action_bus::RetainedToolWireInput) {
32491 +         fn test_retained_wire_input(bus: &ActionBus, bytes: &[u8]) -> (semio_framework::ToolWireAdmission, semio_framework::action_bus::RetainedToolWireInput) {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32491:89
      |
32491 | ...   fn test_retained_wire_input(bus: &semio_framework::ActionBus, bytes: &[u8]) -> (semio_framework::ToolWireAdmission, semio_f...
      |                                                                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32491 -         fn test_retained_wire_input(bus: &semio_framework::ActionBus, bytes: &[u8]) -> (semio_framework::ToolWireAdmission, semio_framework::action_bus::RetainedToolWireInput) {
32491 +         fn test_retained_wire_input(bus: &semio_framework::ActionBus, bytes: &[u8]) -> (ToolWireAdmission, semio_framework::action_bus::RetainedToolWireInput) {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32491:125
      |
32491 | ... (semio_framework::ToolWireAdmission, semio_framework::action_bus::RetainedToolWireInput) {
      |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32491 -         fn test_retained_wire_input(bus: &semio_framework::ActionBus, bytes: &[u8]) -> (semio_framework::ToolWireAdmission, semio_framework::action_bus::RetainedToolWireInput) {
32491 +         fn test_retained_wire_input(bus: &semio_framework::ActionBus, bytes: &[u8]) -> (semio_framework::ToolWireAdmission, action_bus::RetainedToolWireInput) {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32493:38
      |
32493 |             for page in bytes.chunks(semio_framework::action_bus::TOOL_WIRE_PAGE_BYTES) {
      |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32493 -             for page in bytes.chunks(semio_framework::action_bus::TOOL_WIRE_PAGE_BYTES) {
32493 +             for page in bytes.chunks(action_bus::TOOL_WIRE_PAGE_BYTES) {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32494:34
      |
32494 | ...   input.admit_page(semio_framework::action_bus::ToolWirePage::try_copy_from(page).expect("test retained wire page")).map_err(...
      |                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32494 -                 input.admit_page(semio_framework::action_bus::ToolWirePage::try_copy_from(page).expect("test retained wire page")).map_err(|(fault, _)| fault).expect("test retained page admission");
32494 +                 input.admit_page(action_bus::ToolWirePage::try_copy_from(page).expect("test retained wire page")).map_err(|(fault, _)| fault).expect("test retained page admission");
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32745:28
      |
32745 |                 presence: &crate::app::PresenceView<'_, PublicationPresence>,
      |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32745 -                 presence: &crate::app::PresenceView<'_, PublicationPresence>,
32745 +                 presence: &PresenceView<'_, PublicationPresence>,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32746:29
      |
32746 |                 transient: &crate::app::TransientView<'_, PublicationTransient>,
      |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32746 -                 transient: &crate::app::TransientView<'_, PublicationTransient>,
32746 +                 transient: &TransientView<'_, PublicationTransient>,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32747:18
      |
32747 |             ) -> crate::app::EphemeralEmit<Self> {
      |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32747 -             ) -> crate::app::EphemeralEmit<Self> {
32747 +             ) -> EphemeralEmit<Self> {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32749:47
      |
32749 |                     TestCommand::Increment => crate::app::EphemeralEmit {
      |                                               ^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32749 -                     TestCommand::Increment => crate::app::EphemeralEmit {
32749 +                     TestCommand::Increment => EphemeralEmit {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32753:26
      |
32753 |                     _ => crate::app::EphemeralEmit::default(),
      |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32753 -                     _ => crate::app::EphemeralEmit::default(),
32753 +                     _ => EphemeralEmit::default(),
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32780:70
      |
32780 |             async fn command_from_action(action: &str, args: Option<&serde_json::Value>) -> Result<Self::Command, Fault> {
      |                                                                      ^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32780 -             async fn command_from_action(action: &str, args: Option<&serde_json::Value>) -> Result<Self::Command, Fault> {
32780 +             async fn command_from_action(action: &str, args: Option<&Value>) -> Result<Self::Command, Fault> {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32783:140
      |
32783 | ...rgs.and_then(|value| value.get("value")).and_then(serde_json::Value::as_str).unwrap_or_default().to_string() }),
      |                                                      ^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32783 -                     "setLabelViaCommand" => Ok(TestCommand::SetLabelViaCommand { value: args.and_then(|value| value.get("value")).and_then(serde_json::Value::as_str).unwrap_or_default().to_string() }),
32783 +                     "setLabelViaCommand" => Ok(TestCommand::SetLabelViaCommand { value: args.and_then(|value| value.get("value")).and_then(Value::as_str).unwrap_or_default().to_string() }),
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32784:137
      |
32784 | ....and_then(|value| value.get("windowId")).and_then(serde_json::Value::as_str).unwrap_or_default().to_string() }),
      |                                                      ^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32784 -                     "targetWindow" => Ok(TestCommand::SetLabelViaCommand { value: args.and_then(|value| value.get("windowId")).and_then(serde_json::Value::as_str).unwrap_or_default().to_string() }),
32784 +                     "targetWindow" => Ok(TestCommand::SetLabelViaCommand { value: args.and_then(|value| value.get("windowId")).and_then(Value::as_str).unwrap_or_default().to_string() }),
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32786:81
      |
32786 | ...   slot: args.and_then(|value| value.get("slot")).and_then(serde_json::Value::as_str).unwrap_or_default().to_string(),
      |                                                               ^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32786 -                         slot: args.and_then(|value| value.get("slot")).and_then(serde_json::Value::as_str).unwrap_or_default().to_string(),
32786 +                         slot: args.and_then(|value| value.get("slot")).and_then(Value::as_str).unwrap_or_default().to_string(),
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32787:88
      |
32787 | ...   child_id: args.and_then(|value| value.get("childId")).and_then(serde_json::Value::as_str).unwrap_or_default().to_string(),
      |                                                                      ^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32787 -                         child_id: args.and_then(|value| value.get("childId")).and_then(serde_json::Value::as_str).unwrap_or_default().to_string(),
32787 +                         child_id: args.and_then(|value| value.get("childId")).and_then(Value::as_str).unwrap_or_default().to_string(),
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32821:135
      |
32821 | ...nt(AppEvent { kind: "active-utility".into(), payload: dsl::to_dsl_value(&json!({ "utilityId": utility_id.clone() })).unwrap_or...
      |                                                          ^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32821 -                     TestCommand::SetActiveUtility { utility_id } => Ok(Emit::event(AppEvent { kind: "active-utility".into(), payload: dsl::to_dsl_value(&json!({ "utilityId": utility_id.clone() })).unwrap_or(dsl::DslValue::Null) })),
32821 +                     TestCommand::SetActiveUtility { utility_id } => Ok(Emit::event(AppEvent { kind: "active-utility".into(), payload: to_dsl_value(&json!({ "utilityId": utility_id.clone() })).unwrap_or(dsl::DslValue::Null) })),
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32821:208
      |
32821 | ...son!({ "utilityId": utility_id.clone() })).unwrap_or(dsl::DslValue::Null) })),
      |                                                         ^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32821 -                     TestCommand::SetActiveUtility { utility_id } => Ok(Emit::event(AppEvent { kind: "active-utility".into(), payload: dsl::to_dsl_value(&json!({ "utilityId": utility_id.clone() })).unwrap_or(dsl::DslValue::Null) })),
32821 +                     TestCommand::SetActiveUtility { utility_id } => Ok(Emit::event(AppEvent { kind: "active-utility".into(), payload: dsl::to_dsl_value(&json!({ "utilityId": utility_id.clone() })).unwrap_or(DslValue::Null) })),
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32855:139
      |
32855 | ...View<'_, TestConfig>) -> UiAssemblyResult<semio_framework_ui_runtime::ComponentTree> {
      |                                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32855 -             async fn render(_body_key: &str, doc: &ArtifactView<'_, TestSnapshot>, _cfg: &ConfigView<'_, TestConfig>) -> UiAssemblyResult<semio_framework_ui_runtime::ComponentTree> {
32855 +             async fn render(_body_key: &str, doc: &ArtifactView<'_, TestSnapshot>, _cfg: &ConfigView<'_, TestConfig>) -> UiAssemblyResult<ComponentTree> {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32926:119
      |
32926 | ...napshot>, _cfg: &ConfigView<'_, TestConfig>) -> protocol::InteractionTopology {
      |                                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32926 -             async fn interaction_topology(doc: &ArtifactView<'_, TestSnapshot>, _cfg: &ConfigView<'_, TestConfig>) -> protocol::InteractionTopology {
32926 +             async fn interaction_topology(doc: &ArtifactView<'_, TestSnapshot>, _cfg: &ConfigView<'_, TestConfig>) -> InteractionTopology {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32928:91
      |
32928 | ...   let ordered = if doc.snapshot.label.is_empty() { Vec::new() } else { vec![protocol::TopologyNode { id: "item-1".into(), gra...
      |                                                                                 ^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32928 -                 let ordered = if doc.snapshot.label.is_empty() { Vec::new() } else { vec![protocol::TopologyNode { id: "item-1".into(), granularity: "item".into(), parent: None }] };
32928 +                 let ordered = if doc.snapshot.label.is_empty() { Vec::new() } else { vec![TopologyNode { id: "item-1".into(), granularity: "item".into(), parent: None }] };
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32929:53
      |
32929 |                 domains.insert("items".to_string(), protocol::DomainTopology { ordered });
      |                                                     ^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32929 -                 domains.insert("items".to_string(), protocol::DomainTopology { ordered });
32929 +                 domains.insert("items".to_string(), DomainTopology { ordered });
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32930:17
      |
32930 |                 protocol::InteractionTopology { domains }
      |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32930 -                 protocol::InteractionTopology { domains }
32930 +                 InteractionTopology { domains }
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32956:25
      |
32956 |             raw: Option<semio_framework::action_bus::RetainedToolWireInput>,
      |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32956 -             raw: Option<semio_framework::action_bus::RetainedToolWireInput>,
32956 +             raw: Option<action_bus::RetainedToolWireInput>,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33006:45
      |
33006 |         struct KeyedTestFactory { keys: Vec<semio_framework::ToolFactoryKey> }
      |                                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33006 -         struct KeyedTestFactory { keys: Vec<semio_framework::ToolFactoryKey> }
33006 +         struct KeyedTestFactory { keys: Vec<ToolFactoryKey> }
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33008:14
      |
33008 |         impl semio_framework::ToolJobFactory for KeyedTestFactory {
      |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33008 -         impl semio_framework::ToolJobFactory for KeyedTestFactory {
33008 +         impl ToolJobFactory for KeyedTestFactory {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33011:33
      |
33011 |             fn keys(&self) -> &[semio_framework::ToolFactoryKey] { &self.keys }
      |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33011 -             fn keys(&self) -> &[semio_framework::ToolFactoryKey] { &self.keys }
33011 +             fn keys(&self) -> &[ToolFactoryKey] { &self.keys }
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33013:41
      |
33013 | ...   fn classification(&self) -> semio_framework::InteractiveJobClassification { semio_framework::InteractiveJobClassification::...
      |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33013 -             fn classification(&self) -> semio_framework::InteractiveJobClassification { semio_framework::InteractiveJobClassification::Migrated }
33013 +             fn classification(&self) -> InteractiveJobClassification { semio_framework::InteractiveJobClassification::Migrated }
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33013:89
      |
33013 | ...rk::InteractiveJobClassification { semio_framework::InteractiveJobClassification::Migrated }
      |                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33013 -             fn classification(&self) -> semio_framework::InteractiveJobClassification { semio_framework::InteractiveJobClassification::Migrated }
33013 +             fn classification(&self) -> semio_framework::InteractiveJobClassification { InteractiveJobClassification::Migrated }
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33014:45
      |
33014 | ...   fn execution_contract(&self) -> semio_framework::ToolExecutionContract { semio_framework::ToolExecutionContract::resumable(...
      |                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33014 -             fn execution_contract(&self) -> semio_framework::ToolExecutionContract { semio_framework::ToolExecutionContract::resumable(32_768, 4, 1, 4_096, 500, 1, 1) }
33014 +             fn execution_contract(&self) -> ToolExecutionContract { semio_framework::ToolExecutionContract::resumable(32_768, 4, 1, 4_096, 500, 1, 1) }
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33014:86
      |
33014 | ...io_framework::ToolExecutionContract { semio_framework::ToolExecutionContract::resumable(32_768, 4, 1, 4_096, 500, 1, 1) }
      |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33014 -             fn execution_contract(&self) -> semio_framework::ToolExecutionContract { semio_framework::ToolExecutionContract::resumable(32_768, 4, 1, 4_096, 500, 1, 1) }
33014 +             fn execution_contract(&self) -> semio_framework::ToolExecutionContract { ToolExecutionContract::resumable(32_768, 4, 1, 4_096, 500, 1, 1) }
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33015:127
      |
33015 | ...payload: Self::Payload) -> Result<Self::Job, semio_framework::ToolJobFactoryError> { Ok(payload) }
      |                                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33015 -             fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, semio_framework::ToolJobFactoryError> { Ok(payload) }
33015 +             fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> { Ok(payload) }
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33016:146
      |
33016 | ...n, mut payload: Self::Payload, input: semio_framework::action_bus::RetainedToolWireInput, checkpoint: Option<semio_framework::...
      |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33016 -             fn create_job_from_wire_pages_with_payload(&mut self, _operation: semio_framework_job::Operation, mut payload: Self::Payload, input: semio_framework::action_bus::RetainedToolWireInput, checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
33016 +             fn create_job_from_wire_pages_with_payload(&mut self, _operation: semio_framework_job::Operation, mut payload: Self::Payload, input: action_bus::RetainedToolWireInput, checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33016:217
      |
33016 | ...inedToolWireInput, checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>) -> Result<Self::Job, (semio_framewo...
      |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33016 -             fn create_job_from_wire_pages_with_payload(&mut self, _operation: semio_framework_job::Operation, mut payload: Self::Payload, input: semio_framework::action_bus::RetainedToolWireInput, checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
33016 +             fn create_job_from_wire_pages_with_payload(&mut self, _operation: semio_framework_job::Operation, mut payload: Self::Payload, input: semio_framework::action_bus::RetainedToolWireInput, checkpoint: Option<action_bus::RetainedToolWireInput>) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33016:292
      |
33016 | ...etainedToolWireInput>) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, semio_framework::action_bus::RetainedToolWi...
      |                                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33016 -             fn create_job_from_wire_pages_with_payload(&mut self, _operation: semio_framework_job::Operation, mut payload: Self::Payload, input: semio_framework::action_bus::RetainedToolWireInput, checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
33016 +             fn create_job_from_wire_pages_with_payload(&mut self, _operation: semio_framework_job::Operation, mut payload: Self::Payload, input: semio_framework::action_bus::RetainedToolWireInput, checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>) -> Result<Self::Job, (ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33016:330
      |
33016 | ...semio_framework::ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::...
      |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33016 -             fn create_job_from_wire_pages_with_payload(&mut self, _operation: semio_framework_job::Operation, mut payload: Self::Payload, input: semio_framework::action_bus::RetainedToolWireInput, checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
33016 +             fn create_job_from_wire_pages_with_payload(&mut self, _operation: semio_framework_job::Operation, mut payload: Self::Payload, input: semio_framework::action_bus::RetainedToolWireInput, checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33016:389
      |
33016 | ...on_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
      |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33016 -             fn create_job_from_wire_pages_with_payload(&mut self, _operation: semio_framework_job::Operation, mut payload: Self::Payload, input: semio_framework::action_bus::RetainedToolWireInput, checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
33016 +             fn create_job_from_wire_pages_with_payload(&mut self, _operation: semio_framework_job::Operation, mut payload: Self::Payload, input: semio_framework::action_bus::RetainedToolWireInput, checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<action_bus::RetainedToolWireInput>)> {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33023:14
      |
33023 |         impl crate::app::ArtifactOwnedToolJobFactory for KeyedTestFactory {
      |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33023 -         impl crate::app::ArtifactOwnedToolJobFactory for KeyedTestFactory {
33023 +         impl ArtifactOwnedToolJobFactory for KeyedTestFactory {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33027:52
      |
33027 | ...   const PUBLICATION_CONTRACTS: &'static [crate::app::ArtifactToolPublicationContract] = &[crate::app::ArtifactToolPublication...
      |                                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33027 -             const PUBLICATION_CONTRACTS: &'static [crate::app::ArtifactToolPublicationContract] = &[crate::app::ArtifactToolPublicationContract { tool_id: "compositeEdit", lanes: &[crate::app::ArtifactToolPublicationLane::Artifact] }];
33027 +             const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[crate::app::ArtifactToolPublicationContract { tool_id: "compositeEdit", lanes: &[crate::app::ArtifactToolPublicationLane::Artifact] }];
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33027:101
      |
33027 | ...pp::ArtifactToolPublicationContract] = &[crate::app::ArtifactToolPublicationContract { tool_id: "compositeEdit", lanes: &[crat...
      |                                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33027 -             const PUBLICATION_CONTRACTS: &'static [crate::app::ArtifactToolPublicationContract] = &[crate::app::ArtifactToolPublicationContract { tool_id: "compositeEdit", lanes: &[crate::app::ArtifactToolPublicationLane::Artifact] }];
33027 +             const PUBLICATION_CONTRACTS: &'static [crate::app::ArtifactToolPublicationContract] = &[ArtifactToolPublicationContract { tool_id: "compositeEdit", lanes: &[crate::app::ArtifactToolPublicationLane::Artifact] }];
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33027:182
      |
33027 | ... { tool_id: "compositeEdit", lanes: &[crate::app::ArtifactToolPublicationLane::Artifact] }];
      |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33027 -             const PUBLICATION_CONTRACTS: &'static [crate::app::ArtifactToolPublicationContract] = &[crate::app::ArtifactToolPublicationContract { tool_id: "compositeEdit", lanes: &[crate::app::ArtifactToolPublicationLane::Artifact] }];
33027 +             const PUBLICATION_CONTRACTS: &'static [crate::app::ArtifactToolPublicationContract] = &[crate::app::ArtifactToolPublicationContract { tool_id: "compositeEdit", lanes: &[ArtifactToolPublicationLane::Artifact] }];
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33046:30
      |
33046 |             type Transient = crate::app::NoTransient;
      |                              ^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33046 -             type Transient = crate::app::NoTransient;
33046 +             type Transient = NoTransient;
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33047:38
      |
33047 |             type TransientMutation = crate::app::NoTransientMutation;
      |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33047 -             type TransientMutation = crate::app::NoTransientMutation;
33047 +             type TransientMutation = NoTransientMutation;
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33052:27
      |
33052 | ...   contract: semio_framework::ToolExecutionContract::resumable(32_768, 4, 1, 4_096, 500, 1, 1), tools: ["compositeEdit"]
      |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33052 -                 contract: semio_framework::ToolExecutionContract::resumable(32_768, 4, 1, 4_096, 500, 1, 1), tools: ["compositeEdit"]
33052 +                 contract: ToolExecutionContract::resumable(32_768, 4, 1, 4_096, 500, 1, 1), tools: ["compositeEdit"]
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33054:59
      |
33054 |             fn register_tool_job_factories(registry: &mut crate::app::ArtifactToolFactoryRegistry<'_, Self>) -> Result<(), Fault> {
      |                                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33054 -             fn register_tool_job_factories(registry: &mut crate::app::ArtifactToolFactoryRegistry<'_, Self>) -> Result<(), Fault> {
33054 +             fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, Self>) -> Result<(), Fault> {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33055:65
      |
33055 | ...   registry.register(KeyedTestFactory { keys: vec![semio_framework::ToolFactoryKey::new(registry.controller_id(), "compositeEd...
      |                                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33055 -                 registry.register(KeyedTestFactory { keys: vec![semio_framework::ToolFactoryKey::new(registry.controller_id(), "compositeEdit")] })
33055 +                 registry.register(KeyedTestFactory { keys: vec![ToolFactoryKey::new(registry.controller_id(), "compositeEdit")] })
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33057:98
      |
33057 | ...   async fn build_tool_job(request: ArtifactOwnedToolJobRequest<Self>) -> Result<Option<semio_framework::ToolOperationSpec>, F...
      |                                                                                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33057 -             async fn build_tool_job(request: ArtifactOwnedToolJobRequest<Self>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
33057 +             async fn build_tool_job(request: ArtifactOwnedToolJobRequest<Self>) -> Result<Option<ToolOperationSpec>, Fault> {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33060:25
      |
33060 | ...   Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, job, req...
      |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33060 -                 Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, job, request.operation)))
33060 +                 Ok(Some(ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, job, request.operation)))
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33069:159
      |
33069 | ...f::PresenceMutation>>>> { Some(crate::app::mutation_fixture::no_state::presence_store_disposer()) }
      |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33069 -             fn build_presence_store_disposer() -> Option<Box<dyn ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> { Some(crate::app::mutation_fixture::no_state::presence_store_disposer()) }
33069 +             fn build_presence_store_disposer() -> Option<Box<dyn ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> { Some(mutation_fixture::no_state::presence_store_disposer()) }
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33070:163
      |
33070 | ...::TransientMutation>>>> { Some(crate::app::mutation_fixture::no_state::transient_store_disposer()) }
      |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33070 -             fn build_transient_store_disposer() -> Option<Box<dyn ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> { Some(crate::app::mutation_fixture::no_state::transient_store_disposer()) }
33070 +             fn build_transient_store_disposer() -> Option<Box<dyn ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> { Some(mutation_fixture::no_state::transient_store_disposer()) }
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33071:144
      |
33071 | ...y<Self::Presence>>> { Some(crate::app::mutation_fixture::no_state::presence_peer_retirement_factory()) }
      |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33071 -             fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> { Some(crate::app::mutation_fixture::no_state::presence_peer_retirement_factory()) }
33071 +             fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> { Some(mutation_fixture::no_state::presence_peer_retirement_factory()) }
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33072:150
      |
33072 | ...elf::Presence>>> { Some(crate::app::mutation_fixture::no_state::presence_local_root_retirement_factory()) }
      |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33072 -             fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> { Some(crate::app::mutation_fixture::no_state::presence_local_root_retirement_factory()) }
33072 +             fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> { Some(mutation_fixture::no_state::presence_local_root_retirement_factory()) }
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33073:152
      |
33073 | ...f::Transient>>> { Some(crate::app::mutation_fixture::no_state::transient_local_root_retirement_factory()) }
      |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33073 -             fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> { Some(crate::app::mutation_fixture::no_state::transient_local_root_retirement_factory()) }
33073 +             fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> { Some(mutation_fixture::no_state::transient_local_root_retirement_factory()) }
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33077:133
      |
33077 | ...View<'_, TestConfig>) -> UiAssemblyResult<semio_framework_ui_runtime::ComponentTree> { TestApp::render(body, doc, cfg).await }
      |                                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33077 -             async fn render(body: &str, doc: &ArtifactView<'_, TestSnapshot>, cfg: &ConfigView<'_, TestConfig>) -> UiAssemblyResult<semio_framework_ui_runtime::ComponentTree> { TestApp::render(body, doc, cfg).await }
33077 +             async fn render(body: &str, doc: &ArtifactView<'_, TestSnapshot>, cfg: &ConfigView<'_, TestConfig>) -> UiAssemblyResult<ComponentTree> { TestApp::render(body, doc, cfg).await }
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33100:138
      |
33100 | ...t_factory().expect("transient local factory"), crate::app::NoTransient::default());
      |                                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33100 -             close_root(<KeyedTestApp as ArtifactApp>::build_transient_local_root_retirement_factory().expect("transient local factory"), crate::app::NoTransient::default());
33100 +             close_root(<KeyedTestApp as ArtifactApp>::build_transient_local_root_retirement_factory().expect("transient local factory"), NoTransient::default());
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33113:57
      |
33113 | ...   let mut transient = store::TransientStore::<crate::app::NoTransient, crate::app::NoTransientMutation>::new(crate::app::NoTr...
      |                                                   ^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33113 -             let mut transient = store::TransientStore::<crate::app::NoTransient, crate::app::NoTransientMutation>::new(crate::app::NoTransient::default());
33113 +             let mut transient = store::TransientStore::<NoTransient, crate::app::NoTransientMutation>::new(crate::app::NoTransient::default());
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33113:82
      |
33113 | ...   let mut transient = store::TransientStore::<crate::app::NoTransient, crate::app::NoTransientMutation>::new(crate::app::NoTr...
      |                                                                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33113 -             let mut transient = store::TransientStore::<crate::app::NoTransient, crate::app::NoTransientMutation>::new(crate::app::NoTransient::default());
33113 +             let mut transient = store::TransientStore::<crate::app::NoTransient, NoTransientMutation>::new(crate::app::NoTransient::default());
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33113:120
      |
33113 | ...ansient, crate::app::NoTransientMutation>::new(crate::app::NoTransient::default());
      |                                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33113 -             let mut transient = store::TransientStore::<crate::app::NoTransient, crate::app::NoTransientMutation>::new(crate::app::NoTransient::default());
33113 +             let mut transient = store::TransientStore::<crate::app::NoTransient, crate::app::NoTransientMutation>::new(NoTransient::default());
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33132:52
      |
33132 |             transient = store::TransientStore::new(crate::app::NoTransient::default());
      |                                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33132 -             transient = store::TransientStore::new(crate::app::NoTransient::default());
33132 +             transient = store::TransientStore::new(NoTransient::default());
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33150:39
      |
33150 |                     .interactive_jobs(semio_framework::InteractiveJobClassification::Migrated)
      |                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33150 -                     .interactive_jobs(semio_framework::InteractiveJobClassification::Migrated)
33150 +                     .interactive_jobs(InteractiveJobClassification::Migrated)
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33158:13
      |
33158 |             crate::app::test_retained_keyed_dispatch::<KeyedTestApp>(
      |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33158 -             crate::app::test_retained_keyed_dispatch::<KeyedTestApp>(
33158 +             test_retained_keyed_dispatch::<KeyedTestApp>(
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33168:13
      |
33168 |             crate::app::test_retained_keyed_dispatch::<KeyedTestApp>(
      |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33168 -             crate::app::test_retained_keyed_dispatch::<KeyedTestApp>(
33168 +             test_retained_keyed_dispatch::<KeyedTestApp>(
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33178:26
      |
33178 | ...   let fixture: serde_json::Value = serde_json::from_str(include_str!("⚛️reactor/🧪️fixtures/📬️operation-continuation.json")).un...
      |                    ^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33178 -             let fixture: serde_json::Value = serde_json::from_str(include_str!("⚛️reactor/🧪️fixtures/📬️operation-continuation.json")).unwrap();
33178 +             let fixture: Value = serde_json::from_str(include_str!("⚛️reactor/🧪️fixtures/📬️operation-continuation.json")).unwrap();
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33186:71
      |
33186 |             let cell = std::sync::Arc::new(super::RuntimeAppCell::new(crate::app::AppInstance { id, app }));
      |                                                                       ^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33186 -             let cell = std::sync::Arc::new(super::RuntimeAppCell::new(crate::app::AppInstance { id, app }));
33186 +             let cell = std::sync::Arc::new(super::RuntimeAppCell::new(AppInstance { id, app }));
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33196:47
      |
33196 | ...   assert_ne!(page.lane, crate::app::TypedOperationResultLane::Fault, "{}", String::from_utf8_lossy(page.bytes()));
      |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33196 -                         assert_ne!(page.lane, crate::app::TypedOperationResultLane::Fault, "{}", String::from_utf8_lossy(page.bytes()));
33196 +                         assert_ne!(page.lane, TypedOperationResultLane::Fault, "{}", String::from_utf8_lossy(page.bytes()));
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33197:50
      |
33197 |                         terminal |= page.lane == crate::app::TypedOperationResultLane::Terminal;
      |                                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33197 -                         terminal |= page.lane == crate::app::TypedOperationResultLane::Terminal;
33197 +                         terminal |= page.lane == TypedOperationResultLane::Terminal;
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33257:30
      |
33257 |             type Transient = crate::app::NoTransient;
      |                              ^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33257 -             type Transient = crate::app::NoTransient;
33257 +             type Transient = NoTransient;
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33258:38
      |
33258 |             type TransientMutation = crate::app::NoTransientMutation;
      |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33258 -             type TransientMutation = crate::app::NoTransientMutation;
33258 +             type TransientMutation = NoTransientMutation;
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33280:137
      |
33280 | ...View<'_, TestConfig>) -> UiAssemblyResult<semio_framework_ui_runtime::ComponentTree> {
      |                                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33280 -             async fn render(body_key: &str, doc: &ArtifactView<'_, TestSnapshot>, cfg: &ConfigView<'_, TestConfig>) -> UiAssemblyResult<semio_framework_ui_runtime::ComponentTree> {
33280 +             async fn render(body_key: &str, doc: &ArtifactView<'_, TestSnapshot>, cfg: &ConfigView<'_, TestConfig>) -> UiAssemblyResult<ComponentTree> {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33330:41
      |
33330 |                 .await.interactive_jobs(semio_framework::InteractiveJobClassification::BatchOnlyPendingRewrite).await,
      |                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33330 -                 .await.interactive_jobs(semio_framework::InteractiveJobClassification::BatchOnlyPendingRewrite).await,
33330 +                 .await.interactive_jobs(InteractiveJobClassification::BatchOnlyPendingRewrite).await,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33342:28
      |
33342 |             let platform = semio_framework::Platform::new(None).await;
      |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33342 -             let platform = semio_framework::Platform::new(None).await;
33342 +             let platform = Platform::new(None).await;
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33367:28
      |
33367 |             let platform = semio_framework::Platform::new(None).await;
      |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33367 -             let platform = semio_framework::Platform::new(None).await;
33367 +             let platform = Platform::new(None).await;
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33401:34
      |
33401 |                 assert_eq!(key, &semio_framework::ToolFactoryKey::new(&controller_id, tool_id));
      |                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33401 -                 assert_eq!(key, &semio_framework::ToolFactoryKey::new(&controller_id, tool_id));
33401 +                 assert_eq!(key, &ToolFactoryKey::new(&controller_id, tool_id));
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33460:13
      |
33460 | ...   crate::app::test_retained_factory_proof_join::<TestApp, TestRetainedCommandFactory, OtherTestRetainedCommandFactory, CopyDrawApp>(co...
      |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33460 -             crate::app::test_retained_factory_proof_join::<TestApp, TestRetainedCommandFactory, OtherTestRetainedCommandFactory, CopyDrawApp>(contract_registry().await, TEST_RETAINED_COMMAND_CONTROLLER, TEST_RETAINED_COMMAND_TOOL, TestRetainedCommandFactory::new());
33460 +             test_retained_factory_proof_join::<TestApp, TestRetainedCommandFactory, OtherTestRetainedCommandFactory, CopyDrawApp>(contract_registry().await, TEST_RETAINED_COMMAND_CONTROLLER, TEST_RETAINED_COMMAND_TOOL, TestRetainedCommandFactory::new());
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33465:13
      |
33465 |             crate::app::test_retained_cancellation_publication_boundaries::<TestApp>().await;
      |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33465 -             crate::app::test_retained_cancellation_publication_boundaries::<TestApp>().await;
33465 +             test_retained_cancellation_publication_boundaries::<TestApp>().await;
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33470:13
      |
33470 |             crate::app::test_retained_latest_wins_slot_and_publication_fairness::<TestApp>().await;
      |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33470 -             crate::app::test_retained_latest_wins_slot_and_publication_fairness::<TestApp>().await;
33470 +             test_retained_latest_wins_slot_and_publication_fairness::<TestApp>().await;
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33480:13
      |
33480 | ...   crate::app::test_retained_document_cancellation::<TestApp>(&TestCountOneItemPreparationFactory, || TestMutation::SetCount(S...
      |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33480 -             crate::app::test_retained_document_cancellation::<TestApp>(&TestCountOneItemPreparationFactory, || TestMutation::SetCount(SetCount { value: 42 }), |snapshot| snapshot.count).await;
33480 +             test_retained_document_cancellation::<TestApp>(&TestCountOneItemPreparationFactory, || TestMutation::SetCount(SetCount { value: 42 }), |snapshot| snapshot.count).await;
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33487:63
      |
33487 |             declaration.semantics.execution.interactive_job = semio_framework::InteractiveJobClassification::Migrated;
      |                                                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33487 -             declaration.semantics.execution.interactive_job = semio_framework::InteractiveJobClassification::Migrated;
33487 +             declaration.semantics.execution.interactive_job = InteractiveJobClassification::Migrated;
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33490:21
      |
33490 |             assert!(crate::app::test_unregistered_tool_job_admission_rejected::<CopyDrawApp>(&owner, &["canvasPointerDown"]));
      |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33490 -             assert!(crate::app::test_unregistered_tool_job_admission_rejected::<CopyDrawApp>(&owner, &["canvasPointerDown"]));
33490 +             assert!(test_unregistered_tool_job_admission_rejected::<CopyDrawApp>(&owner, &["canvasPointerDown"]));
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33498:17
      |
33498 |                 semio_framework::ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
      |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33498 -                 semio_framework::ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
33498 +                 ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33815:23
      |
33815 |             let bus = semio_framework::ActionBus::new();
      |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33815 -             let bus = semio_framework::ActionBus::new();
33815 +             let bus = ActionBus::new();
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33820:39
      |
33820 |             let original_completion = crate::app::ArtifactToolCompletion::<TestApp>::new();
      |                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33820 -             let original_completion = crate::app::ArtifactToolCompletion::<TestApp>::new();
33820 +             let original_completion = ArtifactToolCompletion::<TestApp>::new();
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33823:33
      |
33823 | ...   let original_spec = semio_framework::ToolOperationSpec::new(TEST_RETAINED_COMMAND_CONTROLLER, TEST_RETAINED_COMMAND_TOOL, T...
      |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33823 -             let original_spec = semio_framework::ToolOperationSpec::new(TEST_RETAINED_COMMAND_CONTROLLER, TEST_RETAINED_COMMAND_TOOL, TEST_RETAINED_COMMAND_SCHEMA, original_payload, operation);
33823 +             let original_spec = ToolOperationSpec::new(TEST_RETAINED_COMMAND_CONTROLLER, TEST_RETAINED_COMMAND_TOOL, TEST_RETAINED_COMMAND_SCHEMA, original_payload, operation);
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33866:38
      |
33866 |             let resumed_completion = crate::app::ArtifactToolCompletion::<TestApp>::new();
      |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33866 -             let resumed_completion = crate::app::ArtifactToolCompletion::<TestApp>::new();
33866 +             let resumed_completion = ArtifactToolCompletion::<TestApp>::new();
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33869:32
      |
33869 | ...   let resumed_spec = semio_framework::ToolOperationSpec::new(TEST_RETAINED_COMMAND_CONTROLLER, TEST_RETAINED_COMMAND_TOOL, TE...
      |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33869 -             let resumed_spec = semio_framework::ToolOperationSpec::new(TEST_RETAINED_COMMAND_CONTROLLER, TEST_RETAINED_COMMAND_TOOL, TEST_RETAINED_COMMAND_SCHEMA, resumed_payload, operation);
33869 +             let resumed_spec = ToolOperationSpec::new(TEST_RETAINED_COMMAND_CONTROLLER, TEST_RETAINED_COMMAND_TOOL, TEST_RETAINED_COMMAND_SCHEMA, resumed_payload, operation);
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33909:40
      |
33909 |             let cancelled_completion = crate::app::ArtifactToolCompletion::<TestApp>::new();
      |                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33909 -             let cancelled_completion = crate::app::ArtifactToolCompletion::<TestApp>::new();
33909 +             let cancelled_completion = ArtifactToolCompletion::<TestApp>::new();
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33912:34
      |
33912 | ...   let cancelled_spec = semio_framework::ToolOperationSpec::new(TEST_RETAINED_COMMAND_CONTROLLER, TEST_RETAINED_COMMAND_TOOL, ...
      |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33912 -             let cancelled_spec = semio_framework::ToolOperationSpec::new(TEST_RETAINED_COMMAND_CONTROLLER, TEST_RETAINED_COMMAND_TOOL, TEST_RETAINED_COMMAND_SCHEMA, cancelled_payload, operation);
33912 +             let cancelled_spec = ToolOperationSpec::new(TEST_RETAINED_COMMAND_CONTROLLER, TEST_RETAINED_COMMAND_TOOL, TEST_RETAINED_COMMAND_SCHEMA, cancelled_payload, operation);
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33953:34
      |
33953 |                     .interaction(semio_framework::InteractionDefinition {
      |                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33953 -                     .interaction(semio_framework::InteractionDefinition {
33953 +                     .interaction(InteractionDefinition {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33956:45
      |
33956 | ...   granularities: vec![semio_framework::GranularityDefinition { id: "item".into(), label: LocalizedLabel::data("Item"), icon_i...
      |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33956 -                         granularities: vec![semio_framework::GranularityDefinition { id: "item".into(), label: LocalizedLabel::data("Item"), icon_id: IconName::AppWindow }],
33956 +                         granularities: vec![GranularityDefinition { id: "item".into(), label: LocalizedLabel::data("Item"), icon_id: IconName::AppWindow }],
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33957:36
      |
33957 |                         hierarchy: protocol::HierarchyProvider::Topology,
      |                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33957 -                         hierarchy: protocol::HierarchyProvider::Topology,
33957 +                         hierarchy: HierarchyProvider::Topology,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33958:32
      |
33958 |                         hover: protocol::HoverSpec::default(),
      |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33958 -                         hover: protocol::HoverSpec::default(),
33958 +                         hover: HoverSpec::default(),
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33959:36
      |
33959 |                         selection: protocol::SelectionSpec {
      |                                    ^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33959 -                         selection: protocol::SelectionSpec {
33959 +                         selection: SelectionSpec {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33960:41
      |
33960 | ...                   modes: vec![protocol::SelectionMode::Multiple, protocol::SelectionMode::Single],
      |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33960 -                             modes: vec![protocol::SelectionMode::Multiple, protocol::SelectionMode::Single],
33960 +                             modes: vec![SelectionMode::Multiple, protocol::SelectionMode::Single],
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33960:76
      |
33960 | ...                   modes: vec![protocol::SelectionMode::Multiple, protocol::SelectionMode::Single],
      |                                                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33960 -                             modes: vec![protocol::SelectionMode::Multiple, protocol::SelectionMode::Single],
33960 +                             modes: vec![protocol::SelectionMode::Multiple, SelectionMode::Single],
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33961:43
      |
33961 | ...                   methods: vec![protocol::SelectionMethod::Pick],
      |                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33961 -                             methods: vec![protocol::SelectionMethod::Pick],
33961 +                             methods: vec![SelectionMethod::Pick],
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33962:42
      |
33962 | ...   merges: vec![protocol::MergeMode::Replace, protocol::MergeMode::Additive, protocol::MergeMode::Subtractive, protocol::Merge...
      |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33962 -                             merges: vec![protocol::MergeMode::Replace, protocol::MergeMode::Additive, protocol::MergeMode::Subtractive, protocol::MergeMode::Invertive, protocol::MergeMode::Range],
33962 +                             merges: vec![MergeMode::Replace, protocol::MergeMode::Additive, protocol::MergeMode::Subtractive, protocol::MergeMode::Invertive, protocol::MergeMode::Range],
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33962:72
      |
33962 | ...   merges: vec![protocol::MergeMode::Replace, protocol::MergeMode::Additive, protocol::MergeMode::Subtractive, protocol::Merge...
      |                                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33962 -                             merges: vec![protocol::MergeMode::Replace, protocol::MergeMode::Additive, protocol::MergeMode::Subtractive, protocol::MergeMode::Invertive, protocol::MergeMode::Range],
33962 +                             merges: vec![protocol::MergeMode::Replace, MergeMode::Additive, protocol::MergeMode::Subtractive, protocol::MergeMode::Invertive, protocol::MergeMode::Range],
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33962:103
      |
33962 | ...   merges: vec![protocol::MergeMode::Replace, protocol::MergeMode::Additive, protocol::MergeMode::Subtractive, protocol::Merge...
      |                                                                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33962 -                             merges: vec![protocol::MergeMode::Replace, protocol::MergeMode::Additive, protocol::MergeMode::Subtractive, protocol::MergeMode::Invertive, protocol::MergeMode::Range],
33962 +                             merges: vec![protocol::MergeMode::Replace, protocol::MergeMode::Additive, MergeMode::Subtractive, protocol::MergeMode::Invertive, protocol::MergeMode::Range],
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33962:137
      |
33962 | ...de::Additive, protocol::MergeMode::Subtractive, protocol::MergeMode::Invertive, protocol::MergeMode::Range],
      |                                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33962 -                             merges: vec![protocol::MergeMode::Replace, protocol::MergeMode::Additive, protocol::MergeMode::Subtractive, protocol::MergeMode::Invertive, protocol::MergeMode::Range],
33962 +                             merges: vec![protocol::MergeMode::Replace, protocol::MergeMode::Additive, protocol::MergeMode::Subtractive, MergeMode::Invertive, protocol::MergeMode::Range],
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33962:169
      |
33962 | ...ode::Subtractive, protocol::MergeMode::Invertive, protocol::MergeMode::Range],
      |                                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33962 -                             merges: vec![protocol::MergeMode::Replace, protocol::MergeMode::Additive, protocol::MergeMode::Subtractive, protocol::MergeMode::Invertive, protocol::MergeMode::Range],
33962 +                             merges: vec![protocol::MergeMode::Replace, protocol::MergeMode::Additive, protocol::MergeMode::Subtractive, protocol::MergeMode::Invertive, MergeMode::Range],
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33968:60
      |
33968 |                     .window_kind_interactions("main", vec![semio_framework::InteractionRef::new("items")])
      |                                                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33968 -                     .window_kind_interactions("main", vec![semio_framework::InteractionRef::new("items")])
33968 +                     .window_kind_interactions("main", vec![InteractionRef::new("items")])
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33982:43
      |
33982 |         fn interaction_target_args(extra: serde_json::Value, id: &str) -> serde_json::Value {
      |                                           ^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33982 -         fn interaction_target_args(extra: serde_json::Value, id: &str) -> serde_json::Value {
33982 +         fn interaction_target_args(extra: Value, id: &str) -> serde_json::Value {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33982:75
      |
33982 |         fn interaction_target_args(extra: serde_json::Value, id: &str) -> serde_json::Value {
      |                                                                           ^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33982 -         fn interaction_target_args(extra: serde_json::Value, id: &str) -> serde_json::Value {
33982 +         fn interaction_target_args(extra: serde_json::Value, id: &str) -> Value {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33983:55
      |
33983 | ...   let targets = serde_json::to_string(&vec![protocol::InteractionTarget { granularity: "item".into(), id: id.into() }]).expec...
      |                                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33983 -             let targets = serde_json::to_string(&vec![protocol::InteractionTarget { granularity: "item".into(), id: id.into() }]).expect("targets serialize");
33983 +             let targets = serde_json::to_string(&vec![InteractionTarget { granularity: "item".into(), id: id.into() }]).expect("targets serialize");
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33989:52
      |
33989 |         async fn __semio_plugin_bundle() -> Result<crate::Plugin<TestRuntimeApps>, crate::PluginAssemblyError> {
      |                                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33989 -         async fn __semio_plugin_bundle() -> Result<crate::Plugin<TestRuntimeApps>, crate::PluginAssemblyError> {
33989 +         async fn __semio_plugin_bundle() -> Result<Plugin<TestRuntimeApps>, crate::PluginAssemblyError> {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33989:84
      |
33989 |         async fn __semio_plugin_bundle() -> Result<crate::Plugin<TestRuntimeApps>, crate::PluginAssemblyError> {
      |                                                                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33989 -         async fn __semio_plugin_bundle() -> Result<crate::Plugin<TestRuntimeApps>, crate::PluginAssemblyError> {
33989 +         async fn __semio_plugin_bundle() -> Result<crate::Plugin<TestRuntimeApps>, PluginAssemblyError> {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33990:13
      |
33990 | ...   crate::Plugin::<TestRuntimeApps>::builder("synthetic").label("Synthetic").version("0.0.1").document_app::<TestApp>(syntheti...
      |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33990 -             crate::Plugin::<TestRuntimeApps>::builder("synthetic").label("Synthetic").version("0.0.1").document_app::<TestApp>(synthetic_play_app().await).document_app_mutation_roster::<TestApp>().try_build()
33990 +             Plugin::<TestRuntimeApps>::builder("synthetic").label("Synthetic").version("0.0.1").document_app::<TestApp>(synthetic_play_app().await).document_app_mutation_roster::<TestApp>().try_build()
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34038:29
      |
34038 |             let timestamp = protocol::HybridLogicalTimestamp::new(1, u64::MAX);
      |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34038 -             let timestamp = protocol::HybridLogicalTimestamp::new(1, u64::MAX);
34038 +             let timestamp = HybridLogicalTimestamp::new(1, u64::MAX);
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34039:35
      |
34039 | ...   let mutation_ids: Vec<protocol::MutationId> = envelope.vcs.edits.iter().flat_map(|edit| edit.mutation_meta.iter().filter_ma...
      |                             ^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34039 -             let mutation_ids: Vec<protocol::MutationId> = envelope.vcs.edits.iter().flat_map(|edit| edit.mutation_meta.iter().filter_map(|meta| meta.mutation_id.clone())).collect();
34039 +             let mutation_ids: Vec<MutationId> = envelope.vcs.edits.iter().flat_map(|edit| edit.mutation_meta.iter().filter_map(|meta| meta.mutation_id.clone())).collect();
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34040:65
      |
34040 | ...   let conflict_id = protocol::ConflictId::new(&kind, &protocol::ArtifactId(envelope.id.clone()), &mutation_ids, &timestamp).a...
      |                                                           ^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34040 -             let conflict_id = protocol::ConflictId::new(&kind, &protocol::ArtifactId(envelope.id.clone()), &mutation_ids, &timestamp).await;
34040 +             let conflict_id = protocol::ConflictId::new(&kind, &ArtifactId(envelope.id.clone()), &mutation_ids, &timestamp).await;
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34046:30
      |
34046 |                 actors: vec![protocol::ActorId("local".into())],
      |                              ^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34046 -                 actors: vec![protocol::ActorId("local".into())],
34046 +                 actors: vec![ActorId("local".into())],
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34133:31
      |
34133 | ...   app.handle_action(semio_framework::INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items",...
      |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34133 -             app.handle_action(semio_framework::INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "item-1")), &meta()).await.expect("interactionSelect");
34133 +             app.handle_action(INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "item-1")), &meta()).await.expect("interactionSelect");
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34147:85
      |
34147 |         fn sample_presence_peer(actor: &str, color: Option<u8>, with_pack: bool) -> protocol::PresencePeer {
      |                                                                                     ^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34147 -         fn sample_presence_peer(actor: &str, color: Option<u8>, with_pack: bool) -> protocol::PresencePeer {
34147 +         fn sample_presence_peer(actor: &str, color: Option<u8>, with_pack: bool) -> PresencePeer {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34148:13
      |
34148 |             protocol::PresencePeer {
      |             ^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34148 -             protocol::PresencePeer {
34148 +             PresencePeer {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34164:119
      |
34164 | ...TestApp>, seq: u64, own_color: Option<u8>, peers: &[protocol::PresencePeer], now_ms: i64) -> PresenceRosterOutcome {
      |                                                        ^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34164 -         async fn publish_presence_roster(app: &mut VcsArtifactApp<TestApp>, seq: u64, own_color: Option<u8>, peers: &[protocol::PresencePeer], now_ms: i64) -> PresenceRosterOutcome {
34164 +         async fn publish_presence_roster(app: &mut VcsArtifactApp<TestApp>, seq: u64, own_color: Option<u8>, peers: &[PresencePeer], now_ms: i64) -> PresenceRosterOutcome {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34165:64
      |
34165 |             let roster = peers.iter().map(|peer| resolve_ready(protocol::encode_presence_peer(peer))).collect::<Vec<_>>();
      |                                                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34165 -             let roster = peers.iter().map(|peer| resolve_ready(protocol::encode_presence_peer(peer))).collect::<Vec<_>>();
34165 +             let roster = peers.iter().map(|peer| resolve_ready(encode_presence_peer(peer))).collect::<Vec<_>>();
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34168:107
      |
34168 | ...q, own_color, roster.len() as u32, semio_framework::kernel::FixedCommandPage::try_copy_from(&first).expect("test peer page is ...
      |                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34168 -             let cursor = protocol::PresenceCommandCursor::admit_page(seq, own_color, roster.len() as u32, semio_framework::kernel::FixedCommandPage::try_copy_from(&first).expect("test peer page is fixed-authority"))
34168 +             let cursor = protocol::PresenceCommandCursor::admit_page(seq, own_color, roster.len() as u32, FixedCommandPage::try_copy_from(&first).expect("test peer page is fixed-authority"))
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34178:32
      |
34178 | ...   let page = semio_framework::kernel::FixedCommandPage::try_copy_from(roster.iter().nth(next_page).expect("retained roster pa...
      |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34178 -                     let page = semio_framework::kernel::FixedCommandPage::try_copy_from(roster.iter().nth(next_page).expect("retained roster page")).expect("test peer page is fixed-authority");
34178 +                     let page = FixedCommandPage::try_copy_from(roster.iter().nth(next_page).expect("retained roster page")).expect("test peer page is fixed-authority");
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34263:88
      |
34263 | ...ndCursor::admit_page(seq, None, 0, semio_framework::kernel::FixedCommandPage::try_copy_from(&[]).expect("empty fixed page")).m...
      |                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34263 -                 let cursor = protocol::PresenceCommandCursor::admit_page(seq, None, 0, semio_framework::kernel::FixedCommandPage::try_copy_from(&[]).expect("empty fixed page")).map_err(|(error, _)| error).expect("empty roster cursor");
34263 +                 let cursor = protocol::PresenceCommandCursor::admit_page(seq, None, 0, FixedCommandPage::try_copy_from(&[]).expect("empty fixed page")).map_err(|(error, _)| error).expect("empty roster cursor");
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34286:24
      |
34286 |             let page = semio_framework::kernel::FixedCommandPage::try_copy_from(&[0xA5; 17]).expect("fixed retained peer page");
      |                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34286 -             let page = semio_framework::kernel::FixedCommandPage::try_copy_from(&[0xA5; 17]).expect("fixed retained peer page");
34286 +             let page = FixedCommandPage::try_copy_from(&[0xA5; 17]).expect("fixed retained peer page");
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34312:82
      |
34312 | ...mandCursor::admit_page(9, None, 0, semio_framework::kernel::FixedCommandPage::try_copy_from(&[]).expect("empty fixed page")).m...
      |                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34312 -             let cursor = protocol::PresenceCommandCursor::admit_page(9, None, 0, semio_framework::kernel::FixedCommandPage::try_copy_from(&[]).expect("empty fixed page")).map_err(|(error, _)| error).expect("stale empty roster cursor");
34312 +             let cursor = protocol::PresenceCommandCursor::admit_page(9, None, 0, FixedCommandPage::try_copy_from(&[]).expect("empty fixed page")).map_err(|(error, _)| error).expect("stale empty roster cursor");
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34334:22
      |
34334 |                 Some(protocol::PresenceInteraction {
      |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34334 -                 Some(protocol::PresenceInteraction {
34334 +                 Some(PresenceInteraction {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34336:35
      |
34336 | ...   domains: vec![protocol::PresenceDomain { domain: "items".to_string(), granularity: "item".to_string(), selected: selected.i...
      |                     ^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34336 -                     domains: vec![protocol::PresenceDomain { domain: "items".to_string(), granularity: "item".to_string(), selected: selected.iter().map(|id| id.to_string()).collect(), hovered: hovered.iter().map(|id| id.to_string()).collect() }],
34336 +                     domains: vec![PresenceDomain { domain: "items".to_string(), granularity: "item".to_string(), selected: selected.iter().map(|id| id.to_string()).collect(), hovered: hovered.iter().map(|id| id.to_string()).collect() }],
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34343:25
      |
34343 |             let state = protocol::InteractionState::default();
      |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34343 -             let state = protocol::InteractionState::default();
34343 +             let state = InteractionState::default();
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34472:42
      |
34472 |         async fn test_child_dialect() -> store::os_io::ArtifactDialect {
      |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34472 -         async fn test_child_dialect() -> store::os_io::ArtifactDialect {
34472 +         async fn test_child_dialect() -> ArtifactDialect {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34473:13
      |
34473 |             store::os_io::ArtifactDialect { artifact_kind: "s.test.child".into(), standard: "native".into(), subset: "*".into() }
      |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34473 -             store::os_io::ArtifactDialect { artifact_kind: "s.test.child".into(), standard: "native".into(), subset: "*".into() }
34473 +             ArtifactDialect { artifact_kind: "s.test.child".into(), standard: "native".into(), subset: "*".into() }
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34488:32
      |
34488 |             let child_handle = crate::app::artifact_handle_of("child-1").await;
      |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34488 -             let child_handle = crate::app::artifact_handle_of("child-1").await;
34488 +             let child_handle = artifact_handle_of("child-1").await;
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34531:31
      |
34531 |                 let dialect = store::os_io::ArtifactDialect::parse_coordinate(&entry.dialect).expect("dialect round trips");
      |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34531 -                 let dialect = store::os_io::ArtifactDialect::parse_coordinate(&entry.dialect).expect("dialect round trips");
34531 +                 let dialect = ArtifactDialect::parse_coordinate(&entry.dialect).expect("dialect round trips");
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34772:125
      |
34772 | ... { action_id: "os.setThemeId".into(), args: semio_framework::optional_json_to_dsl(Some(json!({ "themeId": "light" }))) }]);
      |                                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34772 -             assert_eq!(result.requested_effects, vec![Effect::ReplayShellCommand { action_id: "os.setThemeId".into(), args: semio_framework::optional_json_to_dsl(Some(json!({ "themeId": "light" }))) }]);
34772 +             assert_eq!(result.requested_effects, vec![Effect::ReplayShellCommand { action_id: "os.setThemeId".into(), args: optional_json_to_dsl(Some(json!({ "themeId": "light" }))) }]);
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35023:17
      |
35023 | ...   let semio_framework_ui_contract::Component::TreeSection(actions_props) = &all_panel.children[0].component else { panic!("ex...
      |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35023 -             let semio_framework_ui_contract::Component::TreeSection(actions_props) = &all_panel.children[0].component else { panic!("expected a TreeSection") };
35023 +             let Component::TreeSection(actions_props) = &all_panel.children[0].component else { panic!("expected a TreeSection") };
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35027:17
      |
35027 | ...   let semio_framework_ui_contract::Component::TreeSection(commands_props) = &all_panel.children[1].component else { panic!("e...
      |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35027 -             let semio_framework_ui_contract::Component::TreeSection(commands_props) = &all_panel.children[1].component else { panic!("expected a TreeSection") };
35027 +             let Component::TreeSection(commands_props) = &all_panel.children[1].component else { panic!("expected a TreeSection") };
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35030:17
      |
35030 | ...   let semio_framework_ui_contract::Component::TreeItem(revertible_props) = &all_panel.children[1].children[0].component else ...
      |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35030 -             let semio_framework_ui_contract::Component::TreeItem(revertible_props) = &all_panel.children[1].children[0].component else { panic!("expected a TreeItem") };
35030 +             let Component::TreeItem(revertible_props) = &all_panel.children[1].children[0].component else { panic!("expected a TreeItem") };
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35032:17
      |
35032 | ...   let semio_framework_ui_contract::Component::TreeItem(non_revertible_props) = &all_panel.children[1].children[1].component e...
      |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35032 -             let semio_framework_ui_contract::Component::TreeItem(non_revertible_props) = &all_panel.children[1].children[1].component else { panic!("expected a TreeItem") };
35032 +             let Component::TreeItem(non_revertible_props) = &all_panel.children[1].children[1].component else { panic!("expected a TreeItem") };
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35438:39
      |
35438 |             assert_eq!(event.payload, dsl::to_dsl_value(&json!({ "utilityId": "brush" })).unwrap());
      |                                       ^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35438 -             assert_eq!(event.payload, dsl::to_dsl_value(&json!({ "utilityId": "brush" })).unwrap());
35438 +             assert_eq!(event.payload, to_dsl_value(&json!({ "utilityId": "brush" })).unwrap());
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35569:25
      |
35569 |             let plugin: crate::Plugin = crate::Plugin::new("fixture", "Fixture", "0.1.0").plugin_command(
      |                         ^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35569 -             let plugin: crate::Plugin = crate::Plugin::new("fixture", "Fixture", "0.1.0").plugin_command(
35569 +             let plugin: Plugin = crate::Plugin::new("fixture", "Fixture", "0.1.0").plugin_command(
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35569:41
      |
35569 |             let plugin: crate::Plugin = crate::Plugin::new("fixture", "Fixture", "0.1.0").plugin_command(
      |                                         ^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35569 -             let plugin: crate::Plugin = crate::Plugin::new("fixture", "Fixture", "0.1.0").plugin_command(
35569 +             let plugin: crate::Plugin = Plugin::new("fixture", "Fixture", "0.1.0").plugin_command(
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35574:33
      |
35574 |                         output: dsl::DslValue::Null,
      |                                 ^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35574 -                         output: dsl::DslValue::Null,
35574 +                         output: DslValue::Null,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35615:31
      |
35615 | ...   app.handle_action(semio_framework::INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items",...
      |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35615 -             app.handle_action(semio_framework::INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "item-1")), &meta()).await.expect("interactionSelect");
35615 +             app.handle_action(INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "item-1")), &meta()).await.expect("interactionSelect");
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35625:31
      |
35625 | ...   app.handle_action(semio_framework::INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items",...
      |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35625 -             app.handle_action(semio_framework::INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "item-1")), &meta()).await.expect("interactionSelect");
35625 +             app.handle_action(INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "item-1")), &meta()).await.expect("interactionSelect");
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35628:31
      |
35628 | ...   app.handle_action(semio_framework::INTERACTION_HOVER_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", ...
      |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35628 -             app.handle_action(semio_framework::INTERACTION_HOVER_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "channel": "pointer" }), "item-1")), &meta()).await.expect("interactionHover");
35628 +             app.handle_action(INTERACTION_HOVER_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "channel": "pointer" }), "item-1")), &meta()).await.expect("interactionHover");
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35634:31
      |
35634 | ...   app.handle_action(semio_framework::INTERACTION_HOVER_ACTION_ID, Some(&json!({ "domainId": "items", "channel": "pointer", "t...
      |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35634 -             app.handle_action(semio_framework::INTERACTION_HOVER_ACTION_ID, Some(&json!({ "domainId": "items", "channel": "pointer", "targets": "[]" })), &meta()).await.expect("clear hover");
35634 +             app.handle_action(INTERACTION_HOVER_ACTION_ID, Some(&json!({ "domainId": "items", "channel": "pointer", "targets": "[]" })), &meta()).await.expect("clear hover");
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35642:31
      |
35642 | ...   app.handle_action(semio_framework::INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items",...
      |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35642 -             app.handle_action(semio_framework::INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "item-1")), &meta()).await.expect("interactionSelect");
35642 +             app.handle_action(INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "item-1")), &meta()).await.expect("interactionSelect");
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35655:31
      |
35655 | ...   app.handle_action(semio_framework::SET_SELECTION_MODE_ACTION_ID, Some(&json!({ "domainId": "items", "mode": "single" })), &...
      |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35655 -             app.handle_action(semio_framework::SET_SELECTION_MODE_ACTION_ID, Some(&json!({ "domainId": "items", "mode": "single" })), &meta()).await.expect("setSelectionMode");
35655 +             app.handle_action(SET_SELECTION_MODE_ACTION_ID, Some(&json!({ "domainId": "items", "mode": "single" })), &meta()).await.expect("setSelectionMode");
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35656:94
      |
35656 |             assert_eq!(app.interaction_state().await.active_mode.get("items").copied(), Some(protocol::SelectionMode::Single));
      |                                                                                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35656 -             assert_eq!(app.interaction_state().await.active_mode.get("items").copied(), Some(protocol::SelectionMode::Single));
35656 +             assert_eq!(app.interaction_state().await.active_mode.get("items").copied(), Some(SelectionMode::Single));
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35658:31
      |
35658 | ...   app.handle_action(semio_framework::SET_INTERACTION_GRANULARITY_ACTION_ID, Some(&json!({ "domainId": "items", "granularityId...
      |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35658 -             app.handle_action(semio_framework::SET_INTERACTION_GRANULARITY_ACTION_ID, Some(&json!({ "domainId": "items", "granularityId": "item" })), &meta()).await.expect("setInteractionGranularity");
35658 +             app.handle_action(SET_INTERACTION_GRANULARITY_ACTION_ID, Some(&json!({ "domainId": "items", "granularityId": "item" })), &meta()).await.expect("setInteractionGranularity");
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35662:43
      |
35662 | ...   let error = app.handle_action(semio_framework::SET_INTERACTION_GRANULARITY_ACTION_ID, Some(&json!({ "domainId": "items", "g...
      |                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35662 -             let error = app.handle_action(semio_framework::SET_INTERACTION_GRANULARITY_ACTION_ID, Some(&json!({ "domainId": "items", "granularityId": "bogus" })), &meta()).await.expect_err("undeclared granularity must be rejected");
35662 +             let error = app.handle_action(SET_INTERACTION_GRANULARITY_ACTION_ID, Some(&json!({ "domainId": "items", "granularityId": "bogus" })), &meta()).await.expect_err("undeclared granularity must be rejected");
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35671:31
      |
35671 |             app.handle_action(semio_framework::SELECT_ALL_ACTION_ID, None, &meta()).await.expect("selectAll");
      |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35671 -             app.handle_action(semio_framework::SELECT_ALL_ACTION_ID, None, &meta()).await.expect("selectAll");
35671 +             app.handle_action(SELECT_ALL_ACTION_ID, None, &meta()).await.expect("selectAll");
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35674:31
      |
35674 |             app.handle_action(semio_framework::CLEAR_SELECTION_ACTION_ID, None, &meta()).await.expect("clearSelection");
      |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35674 -             app.handle_action(semio_framework::CLEAR_SELECTION_ACTION_ID, None, &meta()).await.expect("clearSelection");
35674 +             app.handle_action(CLEAR_SELECTION_ACTION_ID, None, &meta()).await.expect("clearSelection");
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35682:31
      |
35682 | ...   app.handle_action(semio_framework::INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items",...
      |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35682 -             app.handle_action(semio_framework::INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "item-1")), &meta()).await.expect("interactionSelect");
35682 +             app.handle_action(INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "item-1")), &meta()).await.expect("interactionSelect");
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35696:31
      |
35696 | ...   app.handle_action(semio_framework::INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items",...
      |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35696 -             app.handle_action(semio_framework::INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "item-1")), &meta()).await.expect("interactionSelect");
35696 +             app.handle_action(INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "item-1")), &meta()).await.expect("interactionSelect");
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35699:39
      |
35699 |             assert_eq!(row.action_id, semio_framework::INTERACTION_SELECT_ACTION_ID);
      |                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35699 -             assert_eq!(row.action_id, semio_framework::INTERACTION_SELECT_ACTION_ID);
35699 +             assert_eq!(row.action_id, INTERACTION_SELECT_ACTION_ID);
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35714:64
      |
35714 |                 let builder = resolve_ready(__base.interaction(semio_framework::InteractionDefinition {
      |                                                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35714 -                 let builder = resolve_ready(__base.interaction(semio_framework::InteractionDefinition {
35714 +                 let builder = resolve_ready(__base.interaction(InteractionDefinition {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35717:41
      |
35717 | ...   granularities: vec![semio_framework::GranularityDefinition { id: "item".into(), label: LocalizedLabel::data("Item"), icon_i...
      |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35717 -                     granularities: vec![semio_framework::GranularityDefinition { id: "item".into(), label: LocalizedLabel::data("Item"), icon_id: IconName::AppWindow }],
35717 +                     granularities: vec![GranularityDefinition { id: "item".into(), label: LocalizedLabel::data("Item"), icon_id: IconName::AppWindow }],
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35718:32
      |
35718 |                     hierarchy: protocol::HierarchyProvider::Flat,
      |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35718 -                     hierarchy: protocol::HierarchyProvider::Flat,
35718 +                     hierarchy: HierarchyProvider::Flat,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35719:28
      |
35719 |                     hover: protocol::HoverSpec { transitive: true, ..protocol::HoverSpec::default() },
      |                            ^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35719 -                     hover: protocol::HoverSpec { transitive: true, ..protocol::HoverSpec::default() },
35719 +                     hover: HoverSpec { transitive: true, ..protocol::HoverSpec::default() },
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35719:70
      |
35719 |                     hover: protocol::HoverSpec { transitive: true, ..protocol::HoverSpec::default() },
      |                                                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35719 -                     hover: protocol::HoverSpec { transitive: true, ..protocol::HoverSpec::default() },
35719 +                     hover: protocol::HoverSpec { transitive: true, ..HoverSpec::default() },
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35720:32
      |
35720 | ...   selection: protocol::SelectionSpec { modes: vec![protocol::SelectionMode::Single], methods: vec![protocol::SelectionMethod:...
      |                  ^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35720 -                     selection: protocol::SelectionSpec { modes: vec![protocol::SelectionMode::Single], methods: vec![protocol::SelectionMethod::Pick], merges: vec![protocol::MergeMode::Replace], transitive: false, broadcast: true },
35720 +                     selection: SelectionSpec { modes: vec![protocol::SelectionMode::Single], methods: vec![protocol::SelectionMethod::Pick], merges: vec![protocol::MergeMode::Replace], transitive: false, broadcast: true },
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35720:70
      |
35720 | ...   selection: protocol::SelectionSpec { modes: vec![protocol::SelectionMode::Single], methods: vec![protocol::SelectionMethod:...
      |                                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35720 -                     selection: protocol::SelectionSpec { modes: vec![protocol::SelectionMode::Single], methods: vec![protocol::SelectionMethod::Pick], merges: vec![protocol::MergeMode::Replace], transitive: false, broadcast: true },
35720 +                     selection: protocol::SelectionSpec { modes: vec![SelectionMode::Single], methods: vec![protocol::SelectionMethod::Pick], merges: vec![protocol::MergeMode::Replace], transitive: false, broadcast: true },
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35720:118
      |
35720 | ...rotocol::SelectionMode::Single], methods: vec![protocol::SelectionMethod::Pick], merges: vec![protocol::MergeMode::Replace], t...
      |                                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35720 -                     selection: protocol::SelectionSpec { modes: vec![protocol::SelectionMode::Single], methods: vec![protocol::SelectionMethod::Pick], merges: vec![protocol::MergeMode::Replace], transitive: false, broadcast: true },
35720 +                     selection: protocol::SelectionSpec { modes: vec![protocol::SelectionMode::Single], methods: vec![SelectionMethod::Pick], merges: vec![protocol::MergeMode::Replace], transitive: false, broadcast: true },
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35720:165
      |
35720 | ...![protocol::SelectionMethod::Pick], merges: vec![protocol::MergeMode::Replace], transitive: false, broadcast: true },
      |                                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35720 -                     selection: protocol::SelectionSpec { modes: vec![protocol::SelectionMode::Single], methods: vec![protocol::SelectionMethod::Pick], merges: vec![protocol::MergeMode::Replace], transitive: false, broadcast: true },
35720 +                     selection: protocol::SelectionSpec { modes: vec![protocol::SelectionMode::Single], methods: vec![protocol::SelectionMethod::Pick], merges: vec![MergeMode::Replace], transitive: false, broadcast: true },
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35731:31
      |
35731 | ...   app.handle_action(semio_framework::INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items",...
      |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35731 -             app.handle_action(semio_framework::INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "item-1")), &meta()).await.expect("interactionSelect");
35731 +             app.handle_action(INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "item-1")), &meta()).await.expect("interactionSelect");
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35743:43
      |
35743 |                         interaction: Some(protocol::PresenceInteraction {
      |                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35743 -                         interaction: Some(protocol::PresenceInteraction {
35743 +                         interaction: Some(PresenceInteraction {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35745:43
      |
35745 | ...   domains: vec![protocol::PresenceDomain { domain: "items".to_string(), granularity: "item".to_string(), selected: vec!["item...
      |                     ^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35745 -                             domains: vec![protocol::PresenceDomain { domain: "items".to_string(), granularity: "item".to_string(), selected: vec!["item-1".to_string()], hovered: Vec::new() }],
35745 +                             domains: vec![PresenceDomain { domain: "items".to_string(), granularity: "item".to_string(), selected: vec!["item-1".to_string()], hovered: Vec::new() }],
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35759:24
      |
35759 |             let item = semio_framework_ui_runtime::TreeNode::try_new(
      |                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35759 -             let item = semio_framework_ui_runtime::TreeNode::try_new(
35759 +             let item = TreeNode::try_new(
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35761:17
      |
35761 |                 semio_framework_ui_contract::Component::TreeItem(semio_framework_ui_contract::TreeItemProps {
      |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35761 -                 semio_framework_ui_contract::Component::TreeItem(semio_framework_ui_contract::TreeItemProps {
35761 +                 Component::TreeItem(semio_framework_ui_contract::TreeItemProps {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35761:66
      |
35761 |                 semio_framework_ui_contract::Component::TreeItem(semio_framework_ui_contract::TreeItemProps {
      |                                                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35761 -                 semio_framework_ui_contract::Component::TreeItem(semio_framework_ui_contract::TreeItemProps {
35761 +                 semio_framework_ui_contract::Component::TreeItem(TreeItemProps {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35762:28
      |
35762 | ...   label: semio_framework_ui_contract::Label(semio_framework_ui_contract::UiText::try_from_str("Item 1").expect("bounded fixtu...
      |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35762 -                     label: semio_framework_ui_contract::Label(semio_framework_ui_contract::UiText::try_from_str("Item 1").expect("bounded fixture")),
35762 +                     label: Label(semio_framework_ui_contract::UiText::try_from_str("Item 1").expect("bounded fixture")),
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35762:63
      |
35762 | ...   label: semio_framework_ui_contract::Label(semio_framework_ui_contract::UiText::try_from_str("Item 1").expect("bounded fixtu...
      |                                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35762 -                     label: semio_framework_ui_contract::Label(semio_framework_ui_contract::UiText::try_from_str("Item 1").expect("bounded fixture")),
35762 +                     label: semio_framework_ui_contract::Label(UiText::try_from_str("Item 1").expect("bounded fixture")),
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35769:34
      |
35769 |                     row_actions: semio_framework_ui_contract::UiFixedList::default(),
      |                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35769 -                     row_actions: semio_framework_ui_contract::UiFixedList::default(),
35769 +                     row_actions: UiFixedList::default(),
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35773:27
      |
35773 | ...   let section = semio_framework_ui_runtime::TreeNode::try_new("sec", semio_framework_ui_contract::Component::TreeSection(semi...
      |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35773 -             let section = semio_framework_ui_runtime::TreeNode::try_new("sec", semio_framework_ui_contract::Component::TreeSection(semio_framework_ui_contract::TreeSectionProps { label: None, default_open: None }))
35773 +             let section = TreeNode::try_new("sec", semio_framework_ui_contract::Component::TreeSection(semio_framework_ui_contract::TreeSectionProps { label: None, default_open: None }))
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35773:80
      |
35773 | ...   let section = semio_framework_ui_runtime::TreeNode::try_new("sec", semio_framework_ui_contract::Component::TreeSection(semi...
      |                                                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35773 -             let section = semio_framework_ui_runtime::TreeNode::try_new("sec", semio_framework_ui_contract::Component::TreeSection(semio_framework_ui_contract::TreeSectionProps { label: None, default_open: None }))
35773 +             let section = semio_framework_ui_runtime::TreeNode::try_new("sec", Component::TreeSection(semio_framework_ui_contract::TreeSectionProps { label: None, default_open: None }))
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35773:132
      |
35773 | ...ork_ui_contract::Component::TreeSection(semio_framework_ui_contract::TreeSectionProps { label: None, default_open: None }))
      |                                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35773 -             let section = semio_framework_ui_runtime::TreeNode::try_new("sec", semio_framework_ui_contract::Component::TreeSection(semio_framework_ui_contract::TreeSectionProps { label: None, default_open: None }))
35773 +             let section = semio_framework_ui_runtime::TreeNode::try_new("sec", semio_framework_ui_contract::Component::TreeSection(TreeSectionProps { label: None, default_open: None }))
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35777:24
      |
35777 |             let root = semio_framework_ui_runtime::TreeNode::try_new(
      |                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35777 -             let root = semio_framework_ui_runtime::TreeNode::try_new(
35777 +             let root = TreeNode::try_new(
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35779:17
      |
35779 | ...   semio_framework_ui_contract::Component::Tree(semio_framework_ui_contract::TreeProps { interaction_domain: Some(semio_framew...
      |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35779 -                 semio_framework_ui_contract::Component::Tree(semio_framework_ui_contract::TreeProps { interaction_domain: Some(semio_framework_ui_contract::UiText::try_from_str("items").expect("bounded fixture")) }),
35779 +                 Component::Tree(semio_framework_ui_contract::TreeProps { interaction_domain: Some(semio_framework_ui_contract::UiText::try_from_str("items").expect("bounded fixture")) }),
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35779:62
      |
35779 | ...   semio_framework_ui_contract::Component::Tree(semio_framework_ui_contract::TreeProps { interaction_domain: Some(semio_framew...
      |                                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35779 -                 semio_framework_ui_contract::Component::Tree(semio_framework_ui_contract::TreeProps { interaction_domain: Some(semio_framework_ui_contract::UiText::try_from_str("items").expect("bounded fixture")) }),
35779 +                 semio_framework_ui_contract::Component::Tree(TreeProps { interaction_domain: Some(semio_framework_ui_contract::UiText::try_from_str("items").expect("bounded fixture")) }),
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35779:128
      |
35779 | ...:TreeProps { interaction_domain: Some(semio_framework_ui_contract::UiText::try_from_str("items").expect("bounded fixture")) }),
      |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35779 -                 semio_framework_ui_contract::Component::Tree(semio_framework_ui_contract::TreeProps { interaction_domain: Some(semio_framework_ui_contract::UiText::try_from_str("items").expect("bounded fixture")) }),
35779 +                 semio_framework_ui_contract::Component::Tree(semio_framework_ui_contract::TreeProps { interaction_domain: Some(UiText::try_from_str("items").expect("bounded fixture")) }),
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35784:24
      |
35784 |             let tree = semio_framework_ui_runtime::ComponentTree { root };
      |                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35784 -             let tree = semio_framework_ui_runtime::ComponentTree { root };
35784 +             let tree = ComponentTree { root };
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35868:65
      |
35868 | ...   crate::plugin_runtime::test_push_instance(&runtime, crate::plugin_runtime::AppInstance { id: resumed_instance, app: TestRun...
      |                                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35868 -             crate::plugin_runtime::test_push_instance(&runtime, crate::plugin_runtime::AppInstance { id: resumed_instance, app: TestRuntimeApps::from(app) }).await;
35868 +             crate::plugin_runtime::test_push_instance(&runtime, AppInstance { id: resumed_instance, app: TestRuntimeApps::from(app) }).await;
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35993:29
      |
35993 | ...   let completion: serde_json::Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🧪️tests/...o
      |                       ^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35993 -             let completion: serde_json::Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🧪️tests/⏳️completion/🧪️fixture.json"))).unwrap();
35993 +             let completion: Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🧪️tests/⏳️completion/🧪️fixture.json"))).unwrap();
      |

warning: ambiguous glob re-exports
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:36932:13
      |
36932 |     pub use semio_framework::kernel::*;
      |             ^^^^^^^^^^^^^^^^^^^^^^^^^^ the name `ActionId` in the type namespace is first re-exported here
...
36935 |     pub use semio_framework_ui_contract::*;
      |             ------------------------------ but the name `ActionId` in the type namespace is also re-exported here
      |
      = note: `#[warn(ambiguous_glob_reexports)]` on by default

warning: ambiguous glob re-exports
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:36935:13
      |
36931 |     pub use crate::app::*;
      |             ------------- but the name `tree_item` in the value namespace is also re-exported here
...
36935 |     pub use semio_framework_ui_contract::*;
      |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the name `tree_item` in the value namespace is first re-exported here

warning: ambiguous glob re-exports
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:36935:13
      |
36931 |     pub use crate::app::*;
      |             ------------- but the name `PeerMark` in the type namespace is also re-exported here
...
36935 |     pub use semio_framework_ui_contract::*;
      |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the name `PeerMark` in the type namespace is first re-exported here

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:37608:26
      |
37608 |         let registered = semio_framework::io::list_registered_subset_validator_dialects().await.expect("registry observation");
      |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
37608 -         let registered = semio_framework::io::list_registered_subset_validator_dialects().await.expect("registry observation");
37608 +         let registered = io::list_registered_subset_validator_dialects().await.expect("registry observation");
      |

error[E0308]: mismatched types
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././🧪️tests/🧬️mutation-fixtures/🎲️dummy/🦀️.rs:209:51
    |
209 | ...   let job = DummyFixtureJob { command: Some(request.command), completion: Some(request.completion), count: request.snapshot.cou...
    |                                            ---- ^^^^^^^^^^^^^^^ expected `DummyCommand`, found `Box<DummyCommand>`
    |                                            |
    |                                            arguments to this enum variant are incorrect
    |
    = note: expected enum `DummyCommand`
             found struct `std::boxed::Box<DummyCommand>`
help: the type constructed contains `std::boxed::Box<DummyCommand>` due to the type of the argument passed
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././🧪️tests/🧬️mutation-fixtures/🎲️dummy/🦀️.rs:209:46
    |
209 | ...   let job = DummyFixtureJob { command: Some(request.command), completion: Some(request.completion), count: request.snapshot.cou...
    |                                            ^^^^^---------------^
    |                                                 |
    |                                                 this argument influences the type of `Some`
note: tuple variant defined here
   --> /Users/ueli/.rustup/toolchains/nightly-2026-07-07-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/option.rs:606:5
    |
606 |     Some(#[stable(feature = "rust1", since = "1.0.0")] T),
    |     ^^^^
help: consider unboxing the value
    |
209 |         let job = DummyFixtureJob { command: Some(*request.command), completion: Some(request.completion), count: request.snapshot.count, closing: false };
    |                                                   +

error[E0308]: mismatched types
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././🧪️tests/🧬️mutation-fixtures/🔀️transaction/🦀️.rs:218:49
    |
218 | ...   let job = TxnFixtureJob { command: Some(request.command), completion: Some(request.completion), count: request.snapshot.count...
    |                                          ---- ^^^^^^^^^^^^^^^ expected `TxnCommand`, found `Box<TxnCommand>`
    |                                          |
    |                                          arguments to this enum variant are incorrect
    |
    = note: expected enum `TxnCommand`
             found struct `std::boxed::Box<TxnCommand>`
help: the type constructed contains `std::boxed::Box<TxnCommand>` due to the type of the argument passed
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././🧪️tests/🧬️mutation-fixtures/🔀️transaction/🦀️.rs:218:44
    |
218 | ...   let job = TxnFixtureJob { command: Some(request.command), completion: Some(request.completion), count: request.snapshot.count...
    |                                          ^^^^^---------------^
    |                                               |
    |                                               this argument influences the type of `Some`
note: tuple variant defined here
   --> /Users/ueli/.rustup/toolchains/nightly-2026-07-07-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/option.rs:606:5
    |
606 |     Some(#[stable(feature = "rust1", since = "1.0.0")] T),
    |     ^^^^
help: consider unboxing the value
    |
218 |         let job = TxnFixtureJob { command: Some(*request.command), completion: Some(request.completion), count: request.snapshot.count, closing: false };
    |                                                 +

warning: variable does not need to be mutable
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:29408:17
      |
29408 |             let mut instance = find_instance(list, instance_id)?;
      |                 ----^^^^^^^^
      |                 |
      |                 help: remove this `mut`
      |
      = note: `#[warn(unused_mut)]` (part of `#[warn(unused)]`) on by default

warning: variable does not need to be mutable
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:29812:17
      |
29812 |             let mut instance = find_instance(list, instance_id)?;
      |                 ----^^^^^^^^
      |                 |
      |                 help: remove this `mut`

warning: variable does not need to be mutable
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:29831:17
      |
29831 |             let mut instance = find_instance(list, instance_id)?;
      |                 ----^^^^^^^^
      |                 |
      |                 help: remove this `mut`

warning: variable does not need to be mutable
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:30961:37
      |
30961 | ...                   let mut instance = find_instance(list, instance_id)?;
      |                           ----^^^^^^^^
      |                           |
      |                           help: remove this `mut`

warning: variable does not need to be mutable
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:31041:41
      |
31041 | ...                   let mut instance = find_instance(list, instance_id)?;
      |                           ----^^^^^^^^
      |                           |
      |                           help: remove this `mut`

warning: variable does not need to be mutable
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:31091:29
      |
31091 |                         let mut instance = find_instance(list, instance_id)?;
      |                             ----^^^^^^^^
      |                             |
      |                             help: remove this `mut`

warning: variable does not need to be mutable
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:31125:29
      |
31125 |                         let mut instance = find_instance(list, instance_id)?;
      |                             ----^^^^^^^^
      |                             |
      |                             help: remove this `mut`

warning: variable does not need to be mutable
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:31312:29
      |
31312 |                         let mut instance = find_instance(list, instance_id)?;
      |                             ----^^^^^^^^
      |                             |
      |                             help: remove this `mut`

warning: variable does not need to be mutable
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:31344:17
      |
31344 |             let mut instance = find_instance(list, instance_id)?;
      |                 ----^^^^^^^^
      |                 |
      |                 help: remove this `mut`

warning: variable does not need to be mutable
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:30762:13
      |
30762 |         let mut retry_command = None;
      |             ----^^^^^^^^^^^^^
      |             |
      |             help: remove this `mut`

warning: variable does not need to be mutable
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:30764:13
      |
30764 |         let mut presence_pending = None;
      |             ----^^^^^^^^^^^^^^^^
      |             |
      |             help: remove this `mut`

warning: use of deprecated method `std::sync::atomic::Atomic::<u64>::fetch_update`: renamed to `try_update` for consistency
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:29359:22
      |
29359 |                     .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |generation| generation.checked_add(1))
      |                      ^^^^^^^^^^^^
      |
      = note: `#[warn(deprecated)]` on by default
help: replace the use of the deprecated method
      |
29359 -                     .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |generation| generation.checked_add(1))
29359 +                     .try_update(Ordering::SeqCst, Ordering::SeqCst, |generation| generation.checked_add(1))
      |

warning: use of deprecated method `std::sync::atomic::Atomic::<u64>::fetch_update`: renamed to `try_update` for consistency
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:14750:41
      |
14750 | ...   let _ = self.app_generation.fetch_update(std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire, |genera...
      |                                   ^^^^^^^^^^^^
      |
help: replace the use of the deprecated method
      |
14750 -             let _ = self.app_generation.fetch_update(std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire, |generation| Some(generation.saturating_add(1)));
14750 +             let _ = self.app_generation.try_update(std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire, |generation| Some(generation.saturating_add(1)));
      |

warning: use of deprecated method `std::sync::atomic::Atomic::<usize>::fetch_update`: renamed to `try_update` for consistency
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:13665:43
      |
13665 | ...   let result = self.state.bytes.fetch_update(std::sync::atomic::Ordering::SeqCst, std::sync::atomic::Ordering::SeqCst, |curre...
      |                                     ^^^^^^^^^^^^
      |
help: replace the use of the deprecated method
      |
13665 -             let result = self.state.bytes.fetch_update(std::sync::atomic::Ordering::SeqCst, std::sync::atomic::Ordering::SeqCst, |current| current.checked_add(bytes).filter(|next| *next <= self.maximum)).map(|previous| previous + bytes);
13665 +             let result = self.state.bytes.try_update(std::sync::atomic::Ordering::SeqCst, std::sync::atomic::Ordering::SeqCst, |current| current.checked_add(bytes).filter(|next| *next <= self.maximum)).map(|previous| previous + bytes);
      |

warning: unused import: `Mutation`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32036:24
      |
32036 |         use protocol::{Mutation, MutationDiff};
      |                        ^^^^^^^^

warning: unused import: `MutationKind`
 --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🧬️contributed-mutation-wire/🧪️tests/🦀️.rs:5:63
  |
5 | use protocol::{CompositeMutationKind, Mutation, MutationDiff, MutationKind, MutationLeaf, OpBinary};
  |                                                               ^^^^^^^^^^^^

warning: unused variable: `parent_document_id`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:18721:17
      |
18721 |             let parent_document_id = self.store.envelope().id.clone();
      |                 ^^^^^^^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_parent_document_id`
      |
      = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

warning: unused variable: `restart_command`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:36000:21
      |
36000 |                 let restart_command = restart_command.clone();
      |                     ^^^^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_restart_command`

warning: unused variable: `envelope_seq`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:30841:22
      |
30841 |         if let Some((envelope_seq, mut owner)) = command {
      |                      ^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_envelope_seq`

warning: unused variable: `actor`
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:9240:26
     |
9240 |                     let (actor, pack) = self.packs[self.packs_len - 1].as_ref().expect("retained app-typed presence pack");
     |                          ^^^^^ help: if this is intentional, prefix it with an underscore: `_actor`

warning: unused variable: `index`
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🩹️patches/🦀️component.rs:717:25
    |
717 | ...   if let Some(index) = state.unadmitted.iter().position(|entry| entry.as_ref().is_some_and(|entry| surface_instance(entry.surfa...
    |                   ^^^^^ help: if this is intentional, prefix it with an underscore: `_index`

For more information about this error, try `rustc --explain E0308`.
warning: `semio-framework-plugin` (lib test) generated 556 warnings
error: could not compile `semio-framework-plugin` (lib test) due to 2 previous errors; 559 warnings emitted
[0m[1m1741 |[0m  * throws on non-zero exit, signal, or budget exceed (the [0m[32m`[budget]`[0m line is printed
[0m[1m1742 |[0m  * to stderr first so it survives a caller[0m[32m's try/catch, e.g. [[tryRun]]).[0m
[0m[1m1743 |[0m  */
[0m[1m1744 |[0m [0m[35mexport[0m [0m[35mfunction[0m runCmd(cmd: [0m[34mstring[0m, args: [0m[34mstring[0m[], opts: RunCmdOpts = {}): [0m[35mvoid[0m {
[0m[1m1745 |[0m   [0m[35mconst[0m status = runCmdInternal(cmd, args, opts)[0m[2m;[0m
[0m[1m1746 |[0m   [0m[35mif[0m (status !== [0m[33m0[0m) [0m[35mthrow[0m [0m[35mnew[0m [0m[1mError[0m([0m[32m`[0m${cmd}[0m[32m [0m${args[0m[3m[1m.join[0m([0m[32m" "[0m)}[0m[32m exited with status [0m${status}[0m[32m`[0m)[0m[2m;[0m
                                     [31m[1m^[0m
[0m[31merror[0m[2m:[0m [1mcargo test --manifest-path Cargo.toml --lib mounted_output_admission_ -- --nocapture exited with status 101[0m
[0m      [2mat [0m[0m[1m[3mrunCmd[0m[2m ([0m[0m[36m/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts[0m[2m:[0m[33m1746[0m[2m:[33m31[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3mrunCargo[0m[2m ([0m[0m[36m/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts[0m[2m:[0m[33m2693[0m[2m:[33m3[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3mrun[0m[2m ([0m[0m[36m[2m/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/[0m[36m📜️script.ts[0m[2m:[0m[33m14[0m[2m:[33m11[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3mrun[0m[2m ([0m[0m[36m/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts[0m[2m:[0m[33m1048[0m[2m:[33m71[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3mrunBundleScriptMain[0m[2m ([0m[0m[36m/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts[0m[2m:[0m[33m1078[0m[2m:[33m16[0m[2m)[0m
[0m
[2mBun v1.3.14 (macOS arm64)[0m
Warning: command "bun 📜️script.ts test mounted_output_admission_ -- --nocapture" exited with non-zero status code


 NX   Running target test for project @semio-tech/framework-plugin failed

Failed tasks:

- @semio-tech/framework-plugin:test

Hint: run the command with --verbose for more details.


```

