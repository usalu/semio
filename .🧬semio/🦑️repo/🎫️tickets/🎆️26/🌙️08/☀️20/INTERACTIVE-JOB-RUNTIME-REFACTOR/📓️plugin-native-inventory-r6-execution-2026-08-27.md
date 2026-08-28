# Plugin Native Inventory R6

Actual canonical `@semio-tech/framework-plugin:test --args='--no-run'`, unchanged shared target/profile, SEMIO_COVERAGE=0. Exit 1; Cargo exit 101. The former missing contributed fixture path no longer stops compilation. The native lib-test compile now reports 19 errors; no tests executed. These are actual diagnostics, not guest/Wasm readiness evidence. All captured-source holds released immediately on completion.

Raw output: `🧪️member-plugin-native-inventory-r6-2026-08-27.txt`. Selected nested hashes: `📓️plugin-native-inventory-r6-source-inputs-2026-08-27.md`.

```text
Warning: truncated output (original token count: 124417)
Total output lines: 7804


> nx run @semio-tech/framework-plugin:test --args=--no-run

> bun 📜️script.ts test --no-run

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
   Compiling semio-framework-os-kernel v0.1.0 (/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust)
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
   Compiling semio-framework-actor v0.1.0 (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📦️packages/🦀️rust)
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
   Compiling semio-framework-schema v0.1.0 (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust)
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
  --> 🧰️framework/📦️packages/🦀️rust/../../🛍️products/💻️os/🔨️modules/🔁️workflow/🧬️schema/🧬️mutations/🔄update-node-ports/🦀️.rs:16:23
   |
16 |     fn inverse(&self, base: &WorkflowSnapshot) -> Vec<WorkflowMutation> { Vec::new() }
   |                       ^^^^ help: if this is intentional, prefix it with an underscore: `_base`
   |
   = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

warning: unused variable: `base`
  --> 🧰️framework/📦️packages/🦀️rust/../../🛍️products/💻️os/🔨️modules/🔁️workflow/🧬️schema/🧬️mutations/🔒bind-parameter-field/🦀️.rs:16:23
   |
16 |     fn inverse(&self, base: &WorkflowSnapshot) -> Vec<WorkflowMutation> { vec![WorkflowMutation::UnbindParameterField(UnbindParamete...
   |                       ^^^^ help: if this is intentional, prefix it with an underscore: `_base`

warning: unused variable: `base`
  --> 🧰️framework/📦️packages/🦀️rust/../../🛍️products/💻️os/🔨️modules/🔁️workflow/🧬️schema/🧬️mutations/🧩add-parameter/🦀️.rs:16:23
   |
16 |     fn inverse(&self, base: &WorkflowSnapshot) -> Vec<WorkflowMutation> { vec![WorkflowMutation::RemoveParameter(RemoveParameter { p...
   |                       ^^^^ help: if this is intentional, prefix it with an underscore: `_base`

warning: unused variable: `base`
  --> 🧰️framework/📦️packages/🦀️rust/../../🛍️products/💻️os/🔨️modules/🔁️workflow/🧬️schema/🧬️mutations/📥add-input/🦀️.rs:16:23
   |
16 |     fn inverse(&self, base: &WorkflowSnapshot) -> Vec<WorkflowMutation> { vec![WorkflowMutation::RemoveInput(RemoveInput { input_id:...
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

warning: type alias `Page` is never used
 --> 🧰️framework/📦️packages/🦀️rust/../../🔨️modules/🛂️manifest/../🎠️kernel/📤️return/🏠️source/📚️entries/🦀️component.rs:4:6
  |
4 | type Page<T> = Vec<Node<T>>;
  |      ^^^^
  |
  = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: method `request_session_close` is never used
    --> 🧰️framework/📦️packages/🦀️rust/../../🔨️modules/🛂️manifest/../🎠️kernel/🦀️component.rs:1296:8
     |
1260 | impl UiTurnPatchTransportArena {
     | ------------------------------ method in this implementation
...
1296 |     fn request_session_close(&mut self, session: u64) -> bool {
     |        ^^^^^^^^^^^^^^^^^^^^^

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
   Compiling semio-framework-ui-runtime v0.1.0 (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust)
warning: unnecessary qualification
   --> 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:366:49
    |
366 | ...   let fixed = size_of::<Self>() + std::mem::size_of_val(self.ordinals.entries.entries.as_ref()) + std::mem::size_of_val(self.ke...
    |                                       ^^^^^^^^^^^^^^^^^^^^^
    |
    = note: requested on the command line with `-W unused-qualifications`
help: remove the unnecessary path segments
    |
366 -                 let fixed = size_of::<Self>() + std::mem::size_of_val(self.ordinals.entries.entries.as_ref()) + std::mem::size_of_val(self.key_index.entries.entries.as_ref()) + ui_contract::UiDocumentAssembly::required_open_bytes();
366 +                 let fixed = size_of::<Self>() + size_of_val(self.ordinals.entries.entries.as_ref()) + std::mem::size_of_val(self.key_index.entries.entries.as_ref()) + ui_contract::UiDocumentAssembly::required_open_bytes();
    |

warning: unnecessary qualification
   --> 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:366:113
    |
366 | ...   let fixed = size_of::<Self>() + std::mem::size_of_val(self.ordinals.entries.entries.as_ref()) + std::mem::size_of_val(self.ke...
    |                                                                                                       ^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
366 -                 let fixed = size_of::<Self>() + std::mem::size_of_val(self.ordinals.entries.entries.as_ref()) + std::mem::size_of_val(self.key_index.entries.entries.as_ref()) + ui_contract::UiDocumentAssembly::required_open_bytes();
366 +                 let fixed = size_of::<Self>() + std::mem::size_of_val(self.ordinals.entries.entries.as_ref()) + size_of_val(self.key_index.entries.entries.as_ref()) + ui_contract::UiDocumentAssembly::required_open_bytes();
    |

warning: unnecessary qualification
    --> 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:2033:95
     |
2033 | ...   diff.owned_copy = Some(RecordOwnedCopy::Bindings(ui_contract::UiBindingsCopy::new(std::mem::take(&mut diff.record.bindings))));
     |                                                                                         ^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
2033 -             diff.owned_copy = Some(RecordOwnedCopy::Bindings(ui_contract::UiBindingsCopy::new(std::mem::take(&mut diff.record.bindings))));
2033 +             diff.owned_copy = Some(RecordOwnedCopy::Bindings(ui_contract::UiBindingsCopy::new(take(&mut diff.record.bindings))));
     |

warning: unnecessary qualification
    --> 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:3456:95
     |
3456 | ...   *owned_copy = Some(RecordOwnedCopy::Bindings(ui_contract::UiBindingsCopy::new(std::mem::take(&mut record.bindings))));
     |                                                                                     ^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
3456 -                 *owned_copy = Some(RecordOwnedCopy::Bindings(ui_contract::UiBindingsCopy::new(std::mem::take(&mut record.bindings))));
3456 +                 *owned_copy = Some(RecordOwnedCopy::Bindings(ui_contract::UiBindingsCopy::new(take(&mut record.bindings))));
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
   --> 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:521:8
    |
520 | impl<K, V, const N: usize> SurfaceLinearMap<K, V, N> {
    | ---------------------------------------------------- method in this implementation
521 |     fn get_index(&self, index: usize) -> Option<(&K, &V)> {
    |        ^^^^^^^^^

warning: method `bindings` is never used
   --> 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:658:8
    |
657 | impl RecordOwnedCopy {
    | -------------------- method in this implementation
658 |     fn bindings(&self) -> Option<&ui_contract::UiBindingsCopy> { if let Self::Bindings(value) = self { Some(value) } else { None } }
    |        ^^^^^^^^

warning: associated functions `new` and `new_with_limits` are never used
    --> 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:1289:19
     |
1288 | impl SurfaceReconcileCursor {
     | --------------------------- associated functions in this implementation
1289 |     pub(crate) fn new(tree: crate::ComponentTree, current: &SurfaceReconciler) -> Self {
     |                   ^^^
...
1293 |     pub(crate) fn new_with_limits(tree: crate::ComponentTree, current: &SurfaceReconciler, limits: SurfaceReconcileLimits) -> Self {
     |                   ^^^^^^^^^^^^^^^

warning: function `split_surface_reconcile` is never used
    --> 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:2222:4
     |
2222 | fn split_surface_reconcile(mut credit: ui_contract::UiResidentPermit) -> Result<(ui_contract::UiResidentPermit, ui_contract::UiRes...
     |    ^^^^^^^^^^^^^^^^^^^^^^^

warning: function `shrink_surface_reconcile` is never used
    --> 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:2229:4
     |
2229 | fn shrink_surface_reconcile(mut credit: ui_contract::UiResidentPermit, usage: SurfaceReconcileUsage) -> Result<ui_contract::UiResi...
     |    ^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `estimate_record_bytes` is never used
    --> 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:3402:4
     |
3402 | fn estimate_record_bytes(record: &ui_contract::UiNodeRecord) -> usize {
     |    ^^^^^^^^^^^^^^^^^^^^^

warning: `semio-framework-ui-runtime` (lib) generated 13 warnings (run `cargo fix --lib -p semio-framework-ui-runtime` to apply 4 suggestions)
   Compiling semio-framework-plugin v0.1.0 (/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust)
error[E0432]: unresolved import `super::super::TestConfig`
 --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🧬️test-app-mutations/🎚️config/🧬️mutations/📝️change-test-config-selection/🦀️.rs:1:20
  |
1 | use super::super::{TestConfig,TestConfigDiff,TestConfigMutation}; use protocol::{MutationKind,MutationOutcome,OpBinary,OpText,Protoco...
  |                    ^^^^^^^^^^ no `TestConfig` in `component::test_app_mutation_fixture::config`
  |
  = help: consider importing this struct through its public re-export instead:
          crate::plugin_runtime::TestConfig

error[E0432]: unresolved import `crate::app::TestConfig`
 --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🧬️test-app-mutations/🎚️config/🦀️.rs:5:16
  |
5 | pub(crate) use crate::app::TestConfig;
  |                ^^^^^^^^^^^^----------
  |                            |
  |                            no `TestConfig` in `component::app`
  |
  = note: unresolved item `crate::test_app_mutation_fixture::config::mutations::change_test_config_selection::tests::TestConfig` exists but is inaccessible
help: consider importing this struct through its public re-export instead
  |
5 - pub(crate) use crate::app::TestConfig;
5 + pub(crate) use crate::plugin_runtime::TestConfig;
  |

error[E0432]: unresolved imports `super::super::TestDiff`, `super::super::TestSnapshot`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🧬️test-app-mutations/🧬️document/🧬️mutations/📝️set-test-count/🦀️.rs:2:20
      |
    2 | use super::super::{TestDiff,TestMutation,TestSnapshot};
      |                    ^^^^^^^^              ^^^^^^^^^^^^ no `TestSnapshot` in `component::test_app_mutation_fixture::document`
      |                    |
      |                    no `TestDiff` in `component::test_app_mutation_fixture::document`
      |
note: struct `crate::plugin_runtime::plugin_builder_contract_tests::TestDiff` exists but is inaccessible
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32888:9
      |
32888 |         pub(crate) struct TestDiff {
      |         ^^^^^^^^^^^^^^^^^^^^^^^^^^ not accessible
note: struct `crate::plugin_runtime::plugin_builder_contract_tests::TestSnapshot` exists but is inaccessible
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32789:9
      |
32789 |         pub(crate) struct TestSnapshot {
      |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not accessible

error[E0432]: unresolved imports `super::super::TestDiff`, `super::super::TestSnapshot`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🧬️test-app-mutations/🧬️document/🧬️mutations/🏷️set-label/🦀️.rs:2:20
      |
    2 | use super::super::{TestDiff,TestMutation,TestSnapshot};
      |                    ^^^^^^^^              ^^^^^^^^^^^^ no `TestSnapshot` in `component::test_app_mutation_fixture::document`
      |                    |
      |                    no `TestDiff` in `component::test_app_mutation_fixture::document`
      |
note: these items exist but are inaccessible:
      crate::test_app_mutation_fixture::document::mutations::set_count::tests::TestDiff
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32888:9
      |
32888 |         pub(crate) struct TestDiff {
      |         ^^^^^^^^^^^^^^^^^^^^^^^^^^ `crate::plugin_runtime::plugin_builder_contract_tests::TestDiff`: not accessible
note: these items exist but are inaccessible:
      crate::test_app_mutation_fixture::document::mutations::set_count::tests::TestSnapshot
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32789:9
      |
32789 |         pub(crate) struct TestSnapshot {
      |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `crate::plugin_runtime::plugin_builder_contract_tests::TestSnapshot`: not accessible

error[E0432]: unresolved import `super::super::TestSnapshot`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🧬️test-app-mutations/🧬️document/🧬️mutations/🦀️.rs:29:31
      |
   29 | mod tests { use super::*; use super::super::TestSnapshot; use protocol::{Mutation,MutationDiff,OpBinary,OpText}; #[test] fn direc...
      |                               ^^^^^^^^^^^^^^------------
      |                                             |
      |                                             no `TestSnapshot` in `component::test_app_mutation_fixture::document`
      |
note: these items exist but are inaccessible:
      crate::test_app_mutation_fixture::document::mutations::set_label::tests::TestSnapshot
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32789:9
      |
32789 |         pub(crate) struct TestSnapshot {
      |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `crate::plugin_runtime::plugin_builder_contract_tests::TestSnapshot`: not accessible

error[E0432]: unresolved imports `crate::app::TestDiff`, `crate::app::TestSnapshot`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🧬️test-app-mutations/🧬️document/🦀️.rs:4:29
      |
    4 | pub(crate) use crate::app::{TestDiff,TestSnapshot};
      |                             ^^^^^^^^ ^^^^^^^^^^^^ no `TestSnapshot` in `component::app`
      |                             |
      |                             no `TestDiff` in `component::app`
      |
note: these items exist but are inaccessible:
      crate::test_app_mutation_fixture::document::mutations::set_label::tests::TestDiff
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32888:9
      |
32888 |         pub(crate) struct TestDiff {
      |         ^^^^^^^^^^^^^^^^^^^^^^^^^^ `crate::plugin_runtime::plugin_builder_contract_tests::TestDiff`: not accessible
note: these items exist but are inaccessible:
      crate::test_app_mutation_fixture::document::mutations::set_label::tests::TestSnapshot
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32789:9
      |
32789 |         pub(crate) struct TestSnapshot {
      |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `crate::plugin_runtime::plugin_builder_contract_tests::TestSnapshot`: not accessible
help: a similar name exists in the module
      |
    4 - pub(crate) use crate::app::{TestDiff,TestSnapshot};
    4 + pub(crate) use crate::app::{TestDiff,UiSnapshot};
      |

error[E0603]: trait import `ArtifactPack` is private
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/📄️declaration-channels/🧪️tests/🦀️.rs:82:102
    |
 82 | where S: Clone + Debug + PartialEq + Serialize + DeserializeOwned + store::ArtifactDsl + crate::app::ArtifactPack,
    |                                                                                                      ^^^^^^^^^^^^ private trait import
    |
note: the trait import `ArtifactPack` is defined here...
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:284:117
    |
284 | ...   build_history_columns, create_config_envelope, create_document_envelope, ArtifactCommand, ArtifactEnvelope, ArtifactPack, Art...
    |                                                                                                                   ^^^^^^^^^^^^
note: ...and refers to the trait `ArtifactPack` which is defined here
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📦️glue.rs:288:9
    |
288 | pub use crate::os_store::*;
    |         ^^^^^^^^^^^^^^^ you could import this directly
help: consider importing one of these traits instead
    |
 82 - where S: Clone + Debug + PartialEq + Serialize + DeserializeOwned + store::ArtifactDsl + crate::app::ArtifactPack,
 82 + where S: Clone + Debug + PartialEq + Serialize + DeserializeOwned + store::ArtifactDsl + crate::dsl::ArtifactPack,
    |
 82 - where S: Clone + Debug + PartialEq + Serialize + DeserializeOwned + store::ArtifactDsl + crate::app::ArtifactPack,
 82 + where S: Clone + Debug + PartialEq + Serialize + DeserializeOwned + store::ArtifactDsl + semio_framework_os_kernel::ArtifactPack,
    |

warning: macro-expanded `macro_export` macros from the current crate cannot be referred to by absolute paths
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:28231:17
      |
28231 |             use crate::__semio_dispatch_PluginApp;
      |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
note: the macro is defined here
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:12177:5
      |
12177 |     #[dyn_enum]
      |     ^^^^^^^^^^^
      = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
      = note: for more information, see issue #52234 <https://github.com/rust-lang/rust/issues/52234>
      = note: `-W macro-expanded-macro-exports-accessed-by-absolute-paths` implied by `-W future-incompatible`
      = help: to override `-W future-incompatible` add `#[allow(macro_expanded_macro_exports_accessed_by_absolute_paths)]`
      = note: this warning originates in the attribute macro `dyn_enum` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: macro-expanded `macro_export` macros from the current crate cannot be referred to by absolute paths
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33962:13
      |
33962 |         use crate::__semio_dispatch_PluginApp;
      |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
note: the macro is defined here
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:12177:5
      |
12177 |     #[dyn_enum]
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
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:12177:5
      |
12177 |     #[dyn_enum]
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
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🩹️patches/🦀️component.rs:1029:21
     |
1029 |         let bytes = std::mem::size_of::<PatchTrackerState>();
     |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
1029 -         let bytes = std::mem::size_of::<PatchTrackerState>();
1029 +         let bytes = size_of::<PatchTrackerState>();
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
 --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🧬️mutation-fixtures/🎲️dummy/🧬️mutations/📝️set-dummy-count/🦀️.rs:9:73
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
 --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🧬️mutation-fixtures/🎲️dummy/🧬️mutations/📝️set-dummy-count/🦀️.rs:9:164
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
 --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🧬️mutation-fixtures/🎲️dummy/🧬️mutations/📝️set-dummy-count/🦀️.rs:9:221
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
 --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🧬️mutation-fixtures/🎲️dummy/🧬️mutations/📝️set-dummy-count/🦀️.rs:9:277
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
 --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🧬️mutation-fixtures/🎲️dummy/🧬️mutations/📝️set-dummy-count/🦀️.rs:9:333
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
 --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🧬️mutation-fixtures/🎲️dummy/🧬️mutations/🦀️.rs:9:83
  |
9 | impl protocol::OpText for DummyMutation { fn parse_op(line: &str) -> Result<Self, crate::store::TextError> { Ok(SetDummyCount::parse_...
  |                                                                                   ^^^^^^^^^^^^^^^^^^^^^^^
  |
help: remove the unnecessary path segments
  |
9 - impl protocol::OpText for DummyMutation { fn parse_op(line: &str) -> Result<Self, crate::store::TextError> { Ok(SetDummyCount::parse_op(line)?.into()) } fn print_op(&self) -> String { match self { Self::SetDummyCount(value) => value.print_op() } } }
9 + impl protocol::OpText for DummyMutation { fn parse_op(line: &str) -> Result<Self, store::TextError> { Ok(SetDummyCount::parse_op(line)?.into()) } fn print_op(&self) -> String { match self { Self::SetDummyCount(value) => value.print_op() } } }
  |

warning: unnecessary qualification
 --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/📝️set-transaction-count/🦀️.rs:4:79
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
 --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/📝️set-transaction-count/🦀️.rs:4:181
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
 --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/📝️set-transaction-count/🦀️.rs:4:244
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
 --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/📝️set-transaction-count/🦀️.rs:4:300
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
 --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/📝️set-transaction-count/🦀️.rs:4:362
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
 --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/📝️set-transaction-count-without-preflight/🦀️.rs:4:91
  |
4 | impl OpText for SetTransactionCountWithoutPreflight { fn parse_op(line:&str)->Result<Self,crate::store::TextError>{Ok(Self{value:line...
  |                                                                                           ^^^^^^^^^^^^^^^^^^^^^^^
  |
help: remove the unnecessary path segments
  |
4 - impl OpText for Set…54417 tokens truncated…ent(AppEvent { kind: "active-utility".into(), payload: to_dsl_value(&json!({ "utilityId": utility_id.clone() })).unwrap_or(dsl::DslValue::Null) })),
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33612:208
      |
33612 | ...son!({ "utilityId": utility_id.clone() })).unwrap_or(dsl::DslValue::Null) })),
      |                                                         ^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33612 -                     TestCommand::SetActiveUtility { utility_id } => Ok(Emit::event(AppEvent { kind: "active-utility".into(), payload: dsl::to_dsl_value(&json!({ "utilityId": utility_id.clone() })).unwrap_or(dsl::DslValue::Null) })),
33612 +                     TestCommand::SetActiveUtility { utility_id } => Ok(Emit::event(AppEvent { kind: "active-utility".into(), payload: dsl::to_dsl_value(&json!({ "utilityId": utility_id.clone() })).unwrap_or(DslValue::Null) })),
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33646:139
      |
33646 | ...View<'_, TestConfig>) -> UiAssemblyResult<semio_framework_ui_runtime::ComponentTree> {
      |                                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33646 -             async fn render(_body_key: &str, doc: &ArtifactView<'_, TestSnapshot>, _cfg: &ConfigView<'_, TestConfig>) -> UiAssemblyResult<semio_framework_ui_runtime::ComponentTree> {
33646 +             async fn render(_body_key: &str, doc: &ArtifactView<'_, TestSnapshot>, _cfg: &ConfigView<'_, TestConfig>) -> UiAssemblyResult<ComponentTree> {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33717:119
      |
33717 | ...napshot>, _cfg: &ConfigView<'_, TestConfig>) -> protocol::InteractionTopology {
      |                                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33717 -             async fn interaction_topology(doc: &ArtifactView<'_, TestSnapshot>, _cfg: &ConfigView<'_, TestConfig>) -> protocol::InteractionTopology {
33717 +             async fn interaction_topology(doc: &ArtifactView<'_, TestSnapshot>, _cfg: &ConfigView<'_, TestConfig>) -> InteractionTopology {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33719:91
      |
33719 | ...   let ordered = if doc.snapshot.label.is_empty() { Vec::new() } else { vec![protocol::TopologyNode { id: "item-1".into(), gra...
      |                                                                                 ^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33719 -                 let ordered = if doc.snapshot.label.is_empty() { Vec::new() } else { vec![protocol::TopologyNode { id: "item-1".into(), granularity: "item".into(), parent: None }] };
33719 +                 let ordered = if doc.snapshot.label.is_empty() { Vec::new() } else { vec![TopologyNode { id: "item-1".into(), granularity: "item".into(), parent: None }] };
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33720:53
      |
33720 |                 domains.insert("items".to_string(), protocol::DomainTopology { ordered });
      |                                                     ^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33720 -                 domains.insert("items".to_string(), protocol::DomainTopology { ordered });
33720 +                 domains.insert("items".to_string(), DomainTopology { ordered });
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33721:17
      |
33721 |                 protocol::InteractionTopology { domains }
      |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33721 -                 protocol::InteractionTopology { domains }
33721 +                 InteractionTopology { domains }
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33747:25
      |
33747 |             raw: Option<semio_framework::action_bus::RetainedToolWireInput>,
      |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33747 -             raw: Option<semio_framework::action_bus::RetainedToolWireInput>,
33747 +             raw: Option<action_bus::RetainedToolWireInput>,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33797:45
      |
33797 |         struct KeyedTestFactory { keys: Vec<semio_framework::ToolFactoryKey> }
      |                                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33797 -         struct KeyedTestFactory { keys: Vec<semio_framework::ToolFactoryKey> }
33797 +         struct KeyedTestFactory { keys: Vec<ToolFactoryKey> }
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33799:14
      |
33799 |         impl semio_framework::ToolJobFactory for KeyedTestFactory {
      |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33799 -         impl semio_framework::ToolJobFactory for KeyedTestFactory {
33799 +         impl ToolJobFactory for KeyedTestFactory {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33802:33
      |
33802 |             fn keys(&self) -> &[semio_framework::ToolFactoryKey] { &self.keys }
      |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33802 -             fn keys(&self) -> &[semio_framework::ToolFactoryKey] { &self.keys }
33802 +             fn keys(&self) -> &[ToolFactoryKey] { &self.keys }
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33804:41
      |
33804 | ...   fn classification(&self) -> semio_framework::InteractiveJobClassification { semio_framework::InteractiveJobClassification::...
      |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33804 -             fn classification(&self) -> semio_framework::InteractiveJobClassification { semio_framework::InteractiveJobClassification::Migrated }
33804 +             fn classification(&self) -> InteractiveJobClassification { semio_framework::InteractiveJobClassification::Migrated }
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33804:89
      |
33804 | ...rk::InteractiveJobClassification { semio_framework::InteractiveJobClassification::Migrated }
      |                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33804 -             fn classification(&self) -> semio_framework::InteractiveJobClassification { semio_framework::InteractiveJobClassification::Migrated }
33804 +             fn classification(&self) -> semio_framework::InteractiveJobClassification { InteractiveJobClassification::Migrated }
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33805:45
      |
33805 | ...   fn execution_contract(&self) -> semio_framework::ToolExecutionContract { semio_framework::ToolExecutionContract::resumable(...
      |                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33805 -             fn execution_contract(&self) -> semio_framework::ToolExecutionContract { semio_framework::ToolExecutionContract::resumable(32_768, 4, 1, 4_096, 500, 1, 1) }
33805 +             fn execution_contract(&self) -> ToolExecutionContract { semio_framework::ToolExecutionContract::resumable(32_768, 4, 1, 4_096, 500, 1, 1) }
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33805:86
      |
33805 | ...io_framework::ToolExecutionContract { semio_framework::ToolExecutionContract::resumable(32_768, 4, 1, 4_096, 500, 1, 1) }
      |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33805 -             fn execution_contract(&self) -> semio_framework::ToolExecutionContract { semio_framework::ToolExecutionContract::resumable(32_768, 4, 1, 4_096, 500, 1, 1) }
33805 +             fn execution_contract(&self) -> semio_framework::ToolExecutionContract { ToolExecutionContract::resumable(32_768, 4, 1, 4_096, 500, 1, 1) }
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33806:127
      |
33806 | ...payload: Self::Payload) -> Result<Self::Job, semio_framework::ToolJobFactoryError> { Ok(payload) }
      |                                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33806 -             fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, semio_framework::ToolJobFactoryError> { Ok(payload) }
33806 +             fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> { Ok(payload) }
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33807:146
      |
33807 | ...n, mut payload: Self::Payload, input: semio_framework::action_bus::RetainedToolWireInput, checkpoint: Option<semio_framework::...
      |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33807 -             fn create_job_from_wire_pages_with_payload(&mut self, _operation: semio_framework_job::Operation, mut payload: Self::Payload, input: semio_framework::action_bus::RetainedToolWireInput, checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
33807 +             fn create_job_from_wire_pages_with_payload(&mut self, _operation: semio_framework_job::Operation, mut payload: Self::Payload, input: action_bus::RetainedToolWireInput, checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33807:217
      |
33807 | ...inedToolWireInput, checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>) -> Result<Self::Job, (semio_framewo...
      |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33807 -             fn create_job_from_wire_pages_with_payload(&mut self, _operation: semio_framework_job::Operation, mut payload: Self::Payload, input: semio_framework::action_bus::RetainedToolWireInput, checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
33807 +             fn create_job_from_wire_pages_with_payload(&mut self, _operation: semio_framework_job::Operation, mut payload: Self::Payload, input: semio_framework::action_bus::RetainedToolWireInput, checkpoint: Option<action_bus::RetainedToolWireInput>) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33807:292
      |
33807 | ...etainedToolWireInput>) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, semio_framework::action_bus::RetainedToolWi...
      |                                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33807 -             fn create_job_from_wire_pages_with_payload(&mut self, _operation: semio_framework_job::Operation, mut payload: Self::Payload, input: semio_framework::action_bus::RetainedToolWireInput, checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
33807 +             fn create_job_from_wire_pages_with_payload(&mut self, _operation: semio_framework_job::Operation, mut payload: Self::Payload, input: semio_framework::action_bus::RetainedToolWireInput, checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>) -> Result<Self::Job, (ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33807:330
      |
33807 | ...semio_framework::ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::...
      |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33807 -             fn create_job_from_wire_pages_with_payload(&mut self, _operation: semio_framework_job::Operation, mut payload: Self::Payload, input: semio_framework::action_bus::RetainedToolWireInput, checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
33807 +             fn create_job_from_wire_pages_with_payload(&mut self, _operation: semio_framework_job::Operation, mut payload: Self::Payload, input: semio_framework::action_bus::RetainedToolWireInput, checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33807:389
      |
33807 | ...on_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
      |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33807 -             fn create_job_from_wire_pages_with_payload(&mut self, _operation: semio_framework_job::Operation, mut payload: Self::Payload, input: semio_framework::action_bus::RetainedToolWireInput, checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
33807 +             fn create_job_from_wire_pages_with_payload(&mut self, _operation: semio_framework_job::Operation, mut payload: Self::Payload, input: semio_framework::action_bus::RetainedToolWireInput, checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<action_bus::RetainedToolWireInput>)> {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33814:14
      |
33814 |         impl crate::app::ArtifactOwnedToolJobFactory for KeyedTestFactory {
      |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33814 -         impl crate::app::ArtifactOwnedToolJobFactory for KeyedTestFactory {
33814 +         impl ArtifactOwnedToolJobFactory for KeyedTestFactory {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33818:52
      |
33818 | ...   const PUBLICATION_CONTRACTS: &'static [crate::app::ArtifactToolPublicationContract] = &[crate::app::ArtifactToolPublication...
      |                                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33818 -             const PUBLICATION_CONTRACTS: &'static [crate::app::ArtifactToolPublicationContract] = &[crate::app::ArtifactToolPublicationContract { tool_id: "compositeEdit", lanes: &[crate::app::ArtifactToolPublicationLane::Artifact] }];
33818 +             const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[crate::app::ArtifactToolPublicationContract { tool_id: "compositeEdit", lanes: &[crate::app::ArtifactToolPublicationLane::Artifact] }];
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33818:101
      |
33818 | ...pp::ArtifactToolPublicationContract] = &[crate::app::ArtifactToolPublicationContract { tool_id: "compositeEdit", lanes: &[crat...
      |                                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33818 -             const PUBLICATION_CONTRACTS: &'static [crate::app::ArtifactToolPublicationContract] = &[crate::app::ArtifactToolPublicationContract { tool_id: "compositeEdit", lanes: &[crate::app::ArtifactToolPublicationLane::Artifact] }];
33818 +             const PUBLICATION_CONTRACTS: &'static [crate::app::ArtifactToolPublicationContract] = &[ArtifactToolPublicationContract { tool_id: "compositeEdit", lanes: &[crate::app::ArtifactToolPublicationLane::Artifact] }];
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33818:182
      |
33818 | ... { tool_id: "compositeEdit", lanes: &[crate::app::ArtifactToolPublicationLane::Artifact] }];
      |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33818 -             const PUBLICATION_CONTRACTS: &'static [crate::app::ArtifactToolPublicationContract] = &[crate::app::ArtifactToolPublicationContract { tool_id: "compositeEdit", lanes: &[crate::app::ArtifactToolPublicationLane::Artifact] }];
33818 +             const PUBLICATION_CONTRACTS: &'static [crate::app::ArtifactToolPublicationContract] = &[crate::app::ArtifactToolPublicationContract { tool_id: "compositeEdit", lanes: &[ArtifactToolPublicationLane::Artifact] }];
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33834:30
      |
33834 |             type Transient = crate::app::NoTransient;
      |                              ^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33834 -             type Transient = crate::app::NoTransient;
33834 +             type Transient = NoTransient;
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33835:38
      |
33835 |             type TransientMutation = crate::app::NoTransientMutation;
      |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33835 -             type TransientMutation = crate::app::NoTransientMutation;
33835 +             type TransientMutation = NoTransientMutation;
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33840:27
      |
33840 | ...   contract: semio_framework::ToolExecutionContract::resumable(32_768, 4, 1, 4_096, 500, 1, 1), tools: ["compositeEdit"]
      |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33840 -                 contract: semio_framework::ToolExecutionContract::resumable(32_768, 4, 1, 4_096, 500, 1, 1), tools: ["compositeEdit"]
33840 +                 contract: ToolExecutionContract::resumable(32_768, 4, 1, 4_096, 500, 1, 1), tools: ["compositeEdit"]
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33842:59
      |
33842 |             fn register_tool_job_factories(registry: &mut crate::app::ArtifactToolFactoryRegistry<'_, Self>) -> Result<(), Fault> {
      |                                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33842 -             fn register_tool_job_factories(registry: &mut crate::app::ArtifactToolFactoryRegistry<'_, Self>) -> Result<(), Fault> {
33842 +             fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, Self>) -> Result<(), Fault> {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33843:65
      |
33843 | ...   registry.register(KeyedTestFactory { keys: vec![semio_framework::ToolFactoryKey::new(registry.controller_id(), "compositeEd...
      |                                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33843 -                 registry.register(KeyedTestFactory { keys: vec![semio_framework::ToolFactoryKey::new(registry.controller_id(), "compositeEdit")] })
33843 +                 registry.register(KeyedTestFactory { keys: vec![ToolFactoryKey::new(registry.controller_id(), "compositeEdit")] })
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33845:98
      |
33845 | ...   async fn build_tool_job(request: ArtifactOwnedToolJobRequest<Self>) -> Result<Option<semio_framework::ToolOperationSpec>, F...
      |                                                                                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33845 -             async fn build_tool_job(request: ArtifactOwnedToolJobRequest<Self>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
33845 +             async fn build_tool_job(request: ArtifactOwnedToolJobRequest<Self>) -> Result<Option<ToolOperationSpec>, Fault> {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33848:25
      |
33848 | ...   Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, job, req...
      |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33848 -                 Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, job, request.operation)))
33848 +                 Ok(Some(ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, job, request.operation)))
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33865:133
      |
33865 | ...View<'_, TestConfig>) -> UiAssemblyResult<semio_framework_ui_runtime::ComponentTree> { TestApp::render(body, doc, cfg).await }
      |                                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33865 -             async fn render(body: &str, doc: &ArtifactView<'_, TestSnapshot>, cfg: &ConfigView<'_, TestConfig>) -> UiAssemblyResult<semio_framework_ui_runtime::ComponentTree> { TestApp::render(body, doc, cfg).await }
33865 +             async fn render(body: &str, doc: &ArtifactView<'_, TestSnapshot>, cfg: &ConfigView<'_, TestConfig>) -> UiAssemblyResult<ComponentTree> { TestApp::render(body, doc, cfg).await }
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33879:39
      |
33879 |                     .interactive_jobs(semio_framework::InteractiveJobClassification::Migrated)
      |                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33879 -                     .interactive_jobs(semio_framework::InteractiveJobClassification::Migrated)
33879 +                     .interactive_jobs(InteractiveJobClassification::Migrated)
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33887:13
      |
33887 |             crate::app::test_retained_keyed_dispatch::<KeyedTestApp>(
      |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33887 -             crate::app::test_retained_keyed_dispatch::<KeyedTestApp>(
33887 +             test_retained_keyed_dispatch::<KeyedTestApp>(
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33897:13
      |
33897 |             crate::app::test_retained_keyed_dispatch::<KeyedTestApp>(
      |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33897 -             crate::app::test_retained_keyed_dispatch::<KeyedTestApp>(
33897 +             test_retained_keyed_dispatch::<KeyedTestApp>(
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33907:26
      |
33907 | ...   let fixture: serde_json::Value = serde_json::from_str(include_str!("⚛️reactor/🧪️fixtures/📬️operation-continuation.json")).un...
      |                    ^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33907 -             let fixture: serde_json::Value = serde_json::from_str(include_str!("⚛️reactor/🧪️fixtures/📬️operation-continuation.json")).unwrap();
33907 +             let fixture: Value = serde_json::from_str(include_str!("⚛️reactor/🧪️fixtures/📬️operation-continuation.json")).unwrap();
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33915:71
      |
33915 |             let cell = std::sync::Arc::new(super::RuntimeAppCell::new(crate::app::AppInstance { id, app }));
      |                                                                       ^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33915 -             let cell = std::sync::Arc::new(super::RuntimeAppCell::new(crate::app::AppInstance { id, app }));
33915 +             let cell = std::sync::Arc::new(super::RuntimeAppCell::new(AppInstance { id, app }));
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33925:47
      |
33925 | ...   assert_ne!(page.lane, crate::app::TypedOperationResultLane::Fault, "{}", String::from_utf8_lossy(page.bytes()));
      |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33925 -                         assert_ne!(page.lane, crate::app::TypedOperationResultLane::Fault, "{}", String::from_utf8_lossy(page.bytes()));
33925 +                         assert_ne!(page.lane, TypedOperationResultLane::Fault, "{}", String::from_utf8_lossy(page.bytes()));
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33926:50
      |
33926 |                         terminal |= page.lane == crate::app::TypedOperationResultLane::Terminal;
      |                                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33926 -                         terminal |= page.lane == crate::app::TypedOperationResultLane::Terminal;
33926 +                         terminal |= page.lane == TypedOperationResultLane::Terminal;
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33986:30
      |
33986 |             type Transient = crate::app::NoTransient;
      |                              ^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33986 -             type Transient = crate::app::NoTransient;
33986 +             type Transient = NoTransient;
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33987:38
      |
33987 |             type TransientMutation = crate::app::NoTransientMutation;
      |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33987 -             type TransientMutation = crate::app::NoTransientMutation;
33987 +             type TransientMutation = NoTransientMutation;
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34009:137
      |
34009 | ...View<'_, TestConfig>) -> UiAssemblyResult<semio_framework_ui_runtime::ComponentTree> {
      |                                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34009 -             async fn render(body_key: &str, doc: &ArtifactView<'_, TestSnapshot>, cfg: &ConfigView<'_, TestConfig>) -> UiAssemblyResult<semio_framework_ui_runtime::ComponentTree> {
34009 +             async fn render(body_key: &str, doc: &ArtifactView<'_, TestSnapshot>, cfg: &ConfigView<'_, TestConfig>) -> UiAssemblyResult<ComponentTree> {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34059:41
      |
34059 |                 .await.interactive_jobs(semio_framework::InteractiveJobClassification::BatchOnlyPendingRewrite).await,
      |                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34059 -                 .await.interactive_jobs(semio_framework::InteractiveJobClassification::BatchOnlyPendingRewrite).await,
34059 +                 .await.interactive_jobs(InteractiveJobClassification::BatchOnlyPendingRewrite).await,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34071:28
      |
34071 |             let platform = semio_framework::Platform::new(None).await;
      |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34071 -             let platform = semio_framework::Platform::new(None).await;
34071 +             let platform = Platform::new(None).await;
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34096:28
      |
34096 |             let platform = semio_framework::Platform::new(None).await;
      |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34096 -             let platform = semio_framework::Platform::new(None).await;
34096 +             let platform = Platform::new(None).await;
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34130:34
      |
34130 |                 assert_eq!(key, &semio_framework::ToolFactoryKey::new(&controller_id, tool_id));
      |                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34130 -                 assert_eq!(key, &semio_framework::ToolFactoryKey::new(&controller_id, tool_id));
34130 +                 assert_eq!(key, &ToolFactoryKey::new(&controller_id, tool_id));
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34189:13
      |
34189 | ...   crate::app::test_retained_factory_proof_join::<TestApp, TestRetainedCommandFactory, OtherTestRetainedCommandFactory, CopyDrawApp>(co...
      |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34189 -             crate::app::test_retained_factory_proof_join::<TestApp, TestRetainedCommandFactory, OtherTestRetainedCommandFactory, CopyDrawApp>(contract_registry().await, TEST_RETAINED_COMMAND_CONTROLLER, TEST_RETAINED_COMMAND_TOOL, TestRetainedCommandFactory::new());
34189 +             test_retained_factory_proof_join::<TestApp, TestRetainedCommandFactory, OtherTestRetainedCommandFactory, CopyDrawApp>(contract_registry().await, TEST_RETAINED_COMMAND_CONTROLLER, TEST_RETAINED_COMMAND_TOOL, TestRetainedCommandFactory::new());
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34194:13
      |
34194 |             crate::app::test_retained_cancellation_publication_boundaries::<TestApp>().await;
      |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34194 -             crate::app::test_retained_cancellation_publication_boundaries::<TestApp>().await;
34194 +             test_retained_cancellation_publication_boundaries::<TestApp>().await;
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34199:13
      |
34199 |             crate::app::test_retained_latest_wins_slot_and_publication_fairness::<TestApp>().await;
      |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34199 -             crate::app::test_retained_latest_wins_slot_and_publication_fairness::<TestApp>().await;
34199 +             test_retained_latest_wins_slot_and_publication_fairness::<TestApp>().await;
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34209:13
      |
34209 | ...   crate::app::test_retained_document_cancellation::<TestApp>(&TestCountOneItemPreparationFactory, || TestMutation::SetCount(S...
      |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34209 -             crate::app::test_retained_document_cancellation::<TestApp>(&TestCountOneItemPreparationFactory, || TestMutation::SetCount(SetCount { value: 42 }), |snapshot| snapshot.count).await;
34209 +             test_retained_document_cancellation::<TestApp>(&TestCountOneItemPreparationFactory, || TestMutation::SetCount(SetCount { value: 42 }), |snapshot| snapshot.count).await;
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34216:63
      |
34216 |             declaration.semantics.execution.interactive_job = semio_framework::InteractiveJobClassification::Migrated;
      |                                                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34216 -             declaration.semantics.execution.interactive_job = semio_framework::InteractiveJobClassification::Migrated;
34216 +             declaration.semantics.execution.interactive_job = InteractiveJobClassification::Migrated;
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34219:21
      |
34219 |             assert!(crate::app::test_unregistered_tool_job_admission_rejected::<CopyDrawApp>(&owner, &["canvasPointerDown"]));
      |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34219 -             assert!(crate::app::test_unregistered_tool_job_admission_rejected::<CopyDrawApp>(&owner, &["canvasPointerDown"]));
34219 +             assert!(test_unregistered_tool_job_admission_rejected::<CopyDrawApp>(&owner, &["canvasPointerDown"]));
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34227:17
      |
34227 |                 semio_framework::ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
      |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34227 -                 semio_framework::ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
34227 +                 ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34544:23
      |
34544 |             let bus = semio_framework::ActionBus::new();
      |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34544 -             let bus = semio_framework::ActionBus::new();
34544 +             let bus = ActionBus::new();
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34549:39
      |
34549 |             let original_completion = crate::app::ArtifactToolCompletion::<TestApp>::new();
      |                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34549 -             let original_completion = crate::app::ArtifactToolCompletion::<TestApp>::new();
34549 +             let original_completion = ArtifactToolCompletion::<TestApp>::new();
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34552:33
      |
34552 | ...   let original_spec = semio_framework::ToolOperationSpec::new(TEST_RETAINED_COMMAND_CONTROLLER, TEST_RETAINED_COMMAND_TOOL, T...
      |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34552 -             let original_spec = semio_framework::ToolOperationSpec::new(TEST_RETAINED_COMMAND_CONTROLLER, TEST_RETAINED_COMMAND_TOOL, TEST_RETAINED_COMMAND_SCHEMA, original_payload, operation);
34552 +             let original_spec = ToolOperationSpec::new(TEST_RETAINED_COMMAND_CONTROLLER, TEST_RETAINED_COMMAND_TOOL, TEST_RETAINED_COMMAND_SCHEMA, original_payload, operation);
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34595:38
      |
34595 |             let resumed_completion = crate::app::ArtifactToolCompletion::<TestApp>::new();
      |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34595 -             let resumed_completion = crate::app::ArtifactToolCompletion::<TestApp>::new();
34595 +             let resumed_completion = ArtifactToolCompletion::<TestApp>::new();
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34598:32
      |
34598 | ...   let resumed_spec = semio_framework::ToolOperationSpec::new(TEST_RETAINED_COMMAND_CONTROLLER, TEST_RETAINED_COMMAND_TOOL, TE...
      |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34598 -             let resumed_spec = semio_framework::ToolOperationSpec::new(TEST_RETAINED_COMMAND_CONTROLLER, TEST_RETAINED_COMMAND_TOOL, TEST_RETAINED_COMMAND_SCHEMA, resumed_payload, operation);
34598 +             let resumed_spec = ToolOperationSpec::new(TEST_RETAINED_COMMAND_CONTROLLER, TEST_RETAINED_COMMAND_TOOL, TEST_RETAINED_COMMAND_SCHEMA, resumed_payload, operation);
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34638:40
      |
34638 |             let cancelled_completion = crate::app::ArtifactToolCompletion::<TestApp>::new();
      |                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34638 -             let cancelled_completion = crate::app::ArtifactToolCompletion::<TestApp>::new();
34638 +             let cancelled_completion = ArtifactToolCompletion::<TestApp>::new();
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34641:34
      |
34641 | ...   let cancelled_spec = semio_framework::ToolOperationSpec::new(TEST_RETAINED_COMMAND_CONTROLLER, TEST_RETAINED_COMMAND_TOOL, ...
      |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34641 -             let cancelled_spec = semio_framework::ToolOperationSpec::new(TEST_RETAINED_COMMAND_CONTROLLER, TEST_RETAINED_COMMAND_TOOL, TEST_RETAINED_COMMAND_SCHEMA, cancelled_payload, operation);
34641 +             let cancelled_spec = ToolOperationSpec::new(TEST_RETAINED_COMMAND_CONTROLLER, TEST_RETAINED_COMMAND_TOOL, TEST_RETAINED_COMMAND_SCHEMA, cancelled_payload, operation);
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34682:34
      |
34682 |                     .interaction(semio_framework::InteractionDefinition {
      |                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34682 -                     .interaction(semio_framework::InteractionDefinition {
34682 +                     .interaction(InteractionDefinition {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34685:45
      |
34685 | ...   granularities: vec![semio_framework::GranularityDefinition { id: "item".into(), label: LocalizedLabel::data("Item"), icon_i...
      |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34685 -                         granularities: vec![semio_framework::GranularityDefinition { id: "item".into(), label: LocalizedLabel::data("Item"), icon_id: IconName::AppWindow }],
34685 +                         granularities: vec![GranularityDefinition { id: "item".into(), label: LocalizedLabel::data("Item"), icon_id: IconName::AppWindow }],
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34686:36
      |
34686 |                         hierarchy: protocol::HierarchyProvider::Topology,
      |                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34686 -                         hierarchy: protocol::HierarchyProvider::Topology,
34686 +                         hierarchy: HierarchyProvider::Topology,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34687:32
      |
34687 |                         hover: protocol::HoverSpec::default(),
      |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34687 -                         hover: protocol::HoverSpec::default(),
34687 +                         hover: HoverSpec::default(),
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34688:36
      |
34688 |                         selection: protocol::SelectionSpec {
      |                                    ^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34688 -                         selection: protocol::SelectionSpec {
34688 +                         selection: SelectionSpec {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34689:41
      |
34689 | ...                   modes: vec![protocol::SelectionMode::Multiple, protocol::SelectionMode::Single],
      |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34689 -                             modes: vec![protocol::SelectionMode::Multiple, protocol::SelectionMode::Single],
34689 +                             modes: vec![SelectionMode::Multiple, protocol::SelectionMode::Single],
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34689:76
      |
34689 | ...                   modes: vec![protocol::SelectionMode::Multiple, protocol::SelectionMode::Single],
      |                                                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34689 -                             modes: vec![protocol::SelectionMode::Multiple, protocol::SelectionMode::Single],
34689 +                             modes: vec![protocol::SelectionMode::Multiple, SelectionMode::Single],
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34690:43
      |
34690 | ...                   methods: vec![protocol::SelectionMethod::Pick],
      |                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34690 -                             methods: vec![protocol::SelectionMethod::Pick],
34690 +                             methods: vec![SelectionMethod::Pick],
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34691:42
      |
34691 | ...   merges: vec![protocol::MergeMode::Replace, protocol::MergeMode::Additive, protocol::MergeMode::Subtractive, protocol::Merge...
      |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34691 -                             merges: vec![protocol::MergeMode::Replace, protocol::MergeMode::Additive, protocol::MergeMode::Subtractive, protocol::MergeMode::Invertive, protocol::MergeMode::Range],
34691 +                             merges: vec![MergeMode::Replace, protocol::MergeMode::Additive, protocol::MergeMode::Subtractive, protocol::MergeMode::Invertive, protocol::MergeMode::Range],
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34691:72
      |
34691 | ...   merges: vec![protocol::MergeMode::Replace, protocol::MergeMode::Additive, protocol::MergeMode::Subtractive, protocol::Merge...
      |                                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34691 -                             merges: vec![protocol::MergeMode::Replace, protocol::MergeMode::Additive, protocol::MergeMode::Subtractive, protocol::MergeMode::Invertive, protocol::MergeMode::Range],
34691 +                             merges: vec![protocol::MergeMode::Replace, MergeMode::Additive, protocol::MergeMode::Subtractive, protocol::MergeMode::Invertive, protocol::MergeMode::Range],
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34691:103
      |
34691 | ...   merges: vec![protocol::MergeMode::Replace, protocol::MergeMode::Additive, protocol::MergeMode::Subtractive, protocol::Merge...
      |                                                                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34691 -                             merges: vec![protocol::MergeMode::Replace, protocol::MergeMode::Additive, protocol::MergeMode::Subtractive, protocol::MergeMode::Invertive, protocol::MergeMode::Range],
34691 +                             merges: vec![protocol::MergeMode::Replace, protocol::MergeMode::Additive, MergeMode::Subtractive, protocol::MergeMode::Invertive, protocol::MergeMode::Range],
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34691:137
      |
34691 | ...de::Additive, protocol::MergeMode::Subtractive, protocol::MergeMode::Invertive, protocol::MergeMode::Range],
      |                                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34691 -                             merges: vec![protocol::MergeMode::Replace, protocol::MergeMode::Additive, protocol::MergeMode::Subtractive, protocol::MergeMode::Invertive, protocol::MergeMode::Range],
34691 +                             merges: vec![protocol::MergeMode::Replace, protocol::MergeMode::Additive, protocol::MergeMode::Subtractive, MergeMode::Invertive, protocol::MergeMode::Range],
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34691:169
      |
34691 | ...ode::Subtractive, protocol::MergeMode::Invertive, protocol::MergeMode::Range],
      |                                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34691 -                             merges: vec![protocol::MergeMode::Replace, protocol::MergeMode::Additive, protocol::MergeMode::Subtractive, protocol::MergeMode::Invertive, protocol::MergeMode::Range],
34691 +                             merges: vec![protocol::MergeMode::Replace, protocol::MergeMode::Additive, protocol::MergeMode::Subtractive, protocol::MergeMode::Invertive, MergeMode::Range],
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34697:60
      |
34697 |                     .window_kind_interactions("main", vec![semio_framework::InteractionRef::new("items")])
      |                                                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34697 -                     .window_kind_interactions("main", vec![semio_framework::InteractionRef::new("items")])
34697 +                     .window_kind_interactions("main", vec![InteractionRef::new("items")])
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34711:43
      |
34711 |         fn interaction_target_args(extra: serde_json::Value, id: &str) -> serde_json::Value {
      |                                           ^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34711 -         fn interaction_target_args(extra: serde_json::Value, id: &str) -> serde_json::Value {
34711 +         fn interaction_target_args(extra: Value, id: &str) -> serde_json::Value {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34711:75
      |
34711 |         fn interaction_target_args(extra: serde_json::Value, id: &str) -> serde_json::Value {
      |                                                                           ^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34711 -         fn interaction_target_args(extra: serde_json::Value, id: &str) -> serde_json::Value {
34711 +         fn interaction_target_args(extra: serde_json::Value, id: &str) -> Value {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34712:55
      |
34712 | ...   let targets = serde_json::to_string(&vec![protocol::InteractionTarget { granularity: "item".into(), id: id.into() }]).expec...
      |                                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34712 -             let targets = serde_json::to_string(&vec![protocol::InteractionTarget { granularity: "item".into(), id: id.into() }]).expect("targets serialize");
34712 +             let targets = serde_json::to_string(&vec![InteractionTarget { granularity: "item".into(), id: id.into() }]).expect("targets serialize");
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34718:52
      |
34718 |         async fn __semio_plugin_bundle() -> Result<crate::Plugin<TestRuntimeApps>, crate::PluginAssemblyError> {
      |                                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34718 -         async fn __semio_plugin_bundle() -> Result<crate::Plugin<TestRuntimeApps>, crate::PluginAssemblyError> {
34718 +         async fn __semio_plugin_bundle() -> Result<Plugin<TestRuntimeApps>, crate::PluginAssemblyError> {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34718:84
      |
34718 |         async fn __semio_plugin_bundle() -> Result<crate::Plugin<TestRuntimeApps>, crate::PluginAssemblyError> {
      |                                                                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34718 -         async fn __semio_plugin_bundle() -> Result<crate::Plugin<TestRuntimeApps>, crate::PluginAssemblyError> {
34718 +         async fn __semio_plugin_bundle() -> Result<crate::Plugin<TestRuntimeApps>, PluginAssemblyError> {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34719:13
      |
34719 | ...   crate::Plugin::<TestRuntimeApps>::builder("synthetic").label("Synthetic").version("0.0.1").document_app::<TestApp>(syntheti...
      |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34719 -             crate::Plugin::<TestRuntimeApps>::builder("synthetic").label("Synthetic").version("0.0.1").document_app::<TestApp>(synthetic_play_app().await).document_app_mutation_roster::<TestApp>().try_build()
34719 +             Plugin::<TestRuntimeApps>::builder("synthetic").label("Synthetic").version("0.0.1").document_app::<TestApp>(synthetic_play_app().await).document_app_mutation_roster::<TestApp>().try_build()
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34767:29
      |
34767 |             let timestamp = protocol::HybridLogicalTimestamp::new(1, u64::MAX);
      |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34767 -             let timestamp = protocol::HybridLogicalTimestamp::new(1, u64::MAX);
34767 +             let timestamp = HybridLogicalTimestamp::new(1, u64::MAX);
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34768:35
      |
34768 | ...   let mutation_ids: Vec<protocol::MutationId> = envelope.vcs.edits.iter().flat_map(|edit| edit.mutation_meta.iter().filter_ma...
      |                             ^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34768 -             let mutation_ids: Vec<protocol::MutationId> = envelope.vcs.edits.iter().flat_map(|edit| edit.mutation_meta.iter().filter_map(|meta| meta.mutation_id.clone())).collect();
34768 +             let mutation_ids: Vec<MutationId> = envelope.vcs.edits.iter().flat_map(|edit| edit.mutation_meta.iter().filter_map(|meta| meta.mutation_id.clone())).collect();
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34769:65
      |
34769 | ...   let conflict_id = protocol::ConflictId::new(&kind, &protocol::ArtifactId(envelope.id.clone()), &mutation_ids, &timestamp).a...
      |                                                           ^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34769 -             let conflict_id = protocol::ConflictId::new(&kind, &protocol::ArtifactId(envelope.id.clone()), &mutation_ids, &timestamp).await;
34769 +             let conflict_id = protocol::ConflictId::new(&kind, &ArtifactId(envelope.id.clone()), &mutation_ids, &timestamp).await;
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34775:30
      |
34775 |                 actors: vec![protocol::ActorId("local".into())],
      |                              ^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34775 -                 actors: vec![protocol::ActorId("local".into())],
34775 +                 actors: vec![ActorId("local".into())],
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34862:31
      |
34862 | ...   app.handle_action(semio_framework::INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items",...
      |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34862 -             app.handle_action(semio_framework::INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "item-1")), &meta()).await.expect("interactionSelect");
34862 +             app.handle_action(INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "item-1")), &meta()).await.expect("interactionSelect");
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34876:85
      |
34876 |         fn sample_presence_peer(actor: &str, color: Option<u8>, with_pack: bool) -> protocol::PresencePeer {
      |                                                                                     ^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34876 -         fn sample_presence_peer(actor: &str, color: Option<u8>, with_pack: bool) -> protocol::PresencePeer {
34876 +         fn sample_presence_peer(actor: &str, color: Option<u8>, with_pack: bool) -> PresencePeer {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34877:13
      |
34877 |             protocol::PresencePeer {
      |             ^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34877 -             protocol::PresencePeer {
34877 +             PresencePeer {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34893:119
      |
34893 | ...TestApp>, seq: u64, own_color: Option<u8>, peers: &[protocol::PresencePeer], now_ms: i64) -> PresenceRosterOutcome {
      |                                                        ^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34893 -         async fn publish_presence_roster(app: &mut VcsArtifactApp<TestApp>, seq: u64, own_color: Option<u8>, peers: &[protocol::PresencePeer], now_ms: i64) -> PresenceRosterOutcome {
34893 +         async fn publish_presence_roster(app: &mut VcsArtifactApp<TestApp>, seq: u64, own_color: Option<u8>, peers: &[PresencePeer], now_ms: i64) -> PresenceRosterOutcome {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34894:64
      |
34894 |             let roster = peers.iter().map(|peer| resolve_ready(protocol::encode_presence_peer(peer))).collect::<Vec<_>>();
      |                                                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34894 -             let roster = peers.iter().map(|peer| resolve_ready(protocol::encode_presence_peer(peer))).collect::<Vec<_>>();
34894 +             let roster = peers.iter().map(|peer| resolve_ready(encode_presence_peer(peer))).collect::<Vec<_>>();
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34897:107
      |
34897 | ...q, own_color, roster.len() as u32, semio_framework::kernel::FixedCommandPage::try_copy_from(&first).expect("test peer page is ...
      |                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34897 -             let cursor = protocol::PresenceCommandCursor::admit_page(seq, own_color, roster.len() as u32, semio_framework::kernel::FixedCommandPage::try_copy_from(&first).expect("test peer page is fixed-authority"))
34897 +             let cursor = protocol::PresenceCommandCursor::admit_page(seq, own_color, roster.len() as u32, FixedCommandPage::try_copy_from(&first).expect("test peer page is fixed-authority"))
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34907:32
      |
34907 | ...   let page = semio_framework::kernel::FixedCommandPage::try_copy_from(roster.iter().nth(next_page).expect("retained roster pa...
      |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34907 -                     let page = semio_framework::kernel::FixedCommandPage::try_copy_from(roster.iter().nth(next_page).expect("retained roster page")).expect("test peer page is fixed-authority");
34907 +                     let page = FixedCommandPage::try_copy_from(roster.iter().nth(next_page).expect("retained roster page")).expect("test peer page is fixed-authority");
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34992:88
      |
34992 | ...ndCursor::admit_page(seq, None, 0, semio_framework::kernel::FixedCommandPage::try_copy_from(&[]).expect("empty fixed page")).m...
      |                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34992 -                 let cursor = protocol::PresenceCommandCursor::admit_page(seq, None, 0, semio_framework::kernel::FixedCommandPage::try_copy_from(&[]).expect("empty fixed page")).map_err(|(error, _)| error).expect("empty roster cursor");
34992 +                 let cursor = protocol::PresenceCommandCursor::admit_page(seq, None, 0, FixedCommandPage::try_copy_from(&[]).expect("empty fixed page")).map_err(|(error, _)| error).expect("empty roster cursor");
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35015:24
      |
35015 |             let page = semio_framework::kernel::FixedCommandPage::try_copy_from(&[0xA5; 17]).expect("fixed retained peer page");
      |                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35015 -             let page = semio_framework::kernel::FixedCommandPage::try_copy_from(&[0xA5; 17]).expect("fixed retained peer page");
35015 +             let page = FixedCommandPage::try_copy_from(&[0xA5; 17]).expect("fixed retained peer page");
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35041:82
      |
35041 | ...mandCursor::admit_page(9, None, 0, semio_framework::kernel::FixedCommandPage::try_copy_from(&[]).expect("empty fixed page")).m...
      |                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35041 -             let cursor = protocol::PresenceCommandCursor::admit_page(9, None, 0, semio_framework::kernel::FixedCommandPage::try_copy_from(&[]).expect("empty fixed page")).map_err(|(error, _)| error).expect("stale empty roster cursor");
35041 +             let cursor = protocol::PresenceCommandCursor::admit_page(9, None, 0, FixedCommandPage::try_copy_from(&[]).expect("empty fixed page")).map_err(|(error, _)| error).expect("stale empty roster cursor");
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35063:22
      |
35063 |                 Some(protocol::PresenceInteraction {
      |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35063 -                 Some(protocol::PresenceInteraction {
35063 +                 Some(PresenceInteraction {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35065:35
      |
35065 | ...   domains: vec![protocol::PresenceDomain { domain: "items".to_string(), granularity: "item".to_string(), selected: selected.i...
      |                     ^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35065 -                     domains: vec![protocol::PresenceDomain { domain: "items".to_string(), granularity: "item".to_string(), selected: selected.iter().map(|id| id.to_string()).collect(), hovered: hovered.iter().map(|id| id.to_string()).collect() }],
35065 +                     domains: vec![PresenceDomain { domain: "items".to_string(), granularity: "item".to_string(), selected: selected.iter().map(|id| id.to_string()).collect(), hovered: hovered.iter().map(|id| id.to_string()).collect() }],
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35072:25
      |
35072 |             let state = protocol::InteractionState::default();
      |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35072 -             let state = protocol::InteractionState::default();
35072 +             let state = InteractionState::default();
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35201:42
      |
35201 |         async fn test_child_dialect() -> store::os_io::ArtifactDialect {
      |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35201 -         async fn test_child_dialect() -> store::os_io::ArtifactDialect {
35201 +         async fn test_child_dialect() -> ArtifactDialect {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35202:13
      |
35202 |             store::os_io::ArtifactDialect { artifact_kind: "s.test.child".into(), standard: "native".into(), subset: "*".into() }
      |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35202 -             store::os_io::ArtifactDialect { artifact_kind: "s.test.child".into(), standard: "native".into(), subset: "*".into() }
35202 +             ArtifactDialect { artifact_kind: "s.test.child".into(), standard: "native".into(), subset: "*".into() }
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35217:32
      |
35217 |             let child_handle = crate::app::artifact_handle_of("child-1").await;
      |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35217 -             let child_handle = crate::app::artifact_handle_of("child-1").await;
35217 +             let child_handle = artifact_handle_of("child-1").await;
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35260:31
      |
35260 |                 let dialect = store::os_io::ArtifactDialect::parse_coordinate(&entry.dialect).expect("dialect round trips");
      |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35260 -                 let dialect = store::os_io::ArtifactDialect::parse_coordinate(&entry.dialect).expect("dialect round trips");
35260 +                 let dialect = ArtifactDialect::parse_coordinate(&entry.dialect).expect("dialect round trips");
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35501:125
      |
35501 | ... { action_id: "os.setThemeId".into(), args: semio_framework::optional_json_to_dsl(Some(json!({ "themeId": "light" }))) }]);
      |                                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35501 -             assert_eq!(result.requested_effects, vec![Effect::ReplayShellCommand { action_id: "os.setThemeId".into(), args: semio_framework::optional_json_to_dsl(Some(json!({ "themeId": "light" }))) }]);
35501 +             assert_eq!(result.requested_effects, vec![Effect::ReplayShellCommand { action_id: "os.setThemeId".into(), args: optional_json_to_dsl(Some(json!({ "themeId": "light" }))) }]);
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35752:17
      |
35752 | ...   let semio_framework_ui_contract::Component::TreeSection(actions_props) = &all_panel.children[0].component else { panic!("ex...
      |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35752 -             let semio_framework_ui_contract::Component::TreeSection(actions_props) = &all_panel.children[0].component else { panic!("expected a TreeSection") };
35752 +             let Component::TreeSection(actions_props) = &all_panel.children[0].component else { panic!("expected a TreeSection") };
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35756:17
      |
35756 | ...   let semio_framework_ui_contract::Component::TreeSection(commands_props) = &all_panel.children[1].component else { panic!("e...
      |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35756 -             let semio_framework_ui_contract::Component::TreeSection(commands_props) = &all_panel.children[1].component else { panic!("expected a TreeSection") };
35756 +             let Component::TreeSection(commands_props) = &all_panel.children[1].component else { panic!("expected a TreeSection") };
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35759:17
      |
35759 | ...   let semio_framework_ui_contract::Component::TreeItem(revertible_props) = &all_panel.children[1].children[0].component else ...
      |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35759 -             let semio_framework_ui_contract::Component::TreeItem(revertible_props) = &all_panel.children[1].children[0].component else { panic!("expected a TreeItem") };
35759 +             let Component::TreeItem(revertible_props) = &all_panel.children[1].children[0].component else { panic!("expected a TreeItem") };
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35761:17
      |
35761 | ...   let semio_framework_ui_contract::Component::TreeItem(non_revertible_props) = &all_panel.children[1].children[1].component e...
      |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35761 -             let semio_framework_ui_contract::Component::TreeItem(non_revertible_props) = &all_panel.children[1].children[1].component else { panic!("expected a TreeItem") };
35761 +             let Component::TreeItem(non_revertible_props) = &all_panel.children[1].children[1].component else { panic!("expected a TreeItem") };
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:36167:39
      |
36167 |             assert_eq!(event.payload, dsl::to_dsl_value(&json!({ "utilityId": "brush" })).unwrap());
      |                                       ^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
36167 -             assert_eq!(event.payload, dsl::to_dsl_value(&json!({ "utilityId": "brush" })).unwrap());
36167 +             assert_eq!(event.payload, to_dsl_value(&json!({ "utilityId": "brush" })).unwrap());
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:36298:25
      |
36298 |             let plugin: crate::Plugin = crate::Plugin::new("fixture", "Fixture", "0.1.0").plugin_command(
      |                         ^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
36298 -             let plugin: crate::Plugin = crate::Plugin::new("fixture", "Fixture", "0.1.0").plugin_command(
36298 +             let plugin: Plugin = crate::Plugin::new("fixture", "Fixture", "0.1.0").plugin_command(
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:36298:41
      |
36298 |             let plugin: crate::Plugin = crate::Plugin::new("fixture", "Fixture", "0.1.0").plugin_command(
      |                                         ^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
36298 -             let plugin: crate::Plugin = crate::Plugin::new("fixture", "Fixture", "0.1.0").plugin_command(
36298 +             let plugin: crate::Plugin = Plugin::new("fixture", "Fixture", "0.1.0").plugin_command(
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:36303:33
      |
36303 |                         output: dsl::DslValue::Null,
      |                                 ^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
36303 -                         output: dsl::DslValue::Null,
36303 +                         output: DslValue::Null,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:36344:31
      |
36344 | ...   app.handle_action(semio_framework::INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items",...
      |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
36344 -             app.handle_action(semio_framework::INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "item-1")), &meta()).await.expect("interactionSelect");
36344 +             app.handle_action(INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "item-1")), &meta()).await.expect("interactionSelect");
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:36354:31
      |
36354 | ...   app.handle_action(semio_framework::INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items",...
      |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
36354 -             app.handle_action(semio_framework::INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "item-1")), &meta()).await.expect("interactionSelect");
36354 +             app.handle_action(INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "item-1")), &meta()).await.expect("interactionSelect");
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:36357:31
      |
36357 | ...   app.handle_action(semio_framework::INTERACTION_HOVER_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", ...
      |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
36357 -             app.handle_action(semio_framework::INTERACTION_HOVER_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "channel": "pointer" }), "item-1")), &meta()).await.expect("interactionHover");
36357 +             app.handle_action(INTERACTION_HOVER_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "channel": "pointer" }), "item-1")), &meta()).await.expect("interactionHover");
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:36363:31
      |
36363 | ...   app.handle_action(semio_framework::INTERACTION_HOVER_ACTION_ID, Some(&json!({ "domainId": "items", "channel": "pointer", "t...
      |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
36363 -             app.handle_action(semio_framework::INTERACTION_HOVER_ACTION_ID, Some(&json!({ "domainId": "items", "channel": "pointer", "targets": "[]" })), &meta()).await.expect("clear hover");
36363 +             app.handle_action(INTERACTION_HOVER_ACTION_ID, Some(&json!({ "domainId": "items", "channel": "pointer", "targets": "[]" })), &meta()).await.expect("clear hover");
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:36371:31
      |
36371 | ...   app.handle_action(semio_framework::INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items",...
      |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
36371 -             app.handle_action(semio_framework::INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "item-1")), &meta()).await.expect("interactionSelect");
36371 +             app.handle_action(INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "item-1")), &meta()).await.expect("interactionSelect");
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:36384:31
      |
36384 | ...   app.handle_action(semio_framework::SET_SELECTION_MODE_ACTION_ID, Some(&json!({ "domainId": "items", "mode": "single" })), &...
      |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
36384 -             app.handle_action(semio_framework::SET_SELECTION_MODE_ACTION_ID, Some(&json!({ "domainId": "items", "mode": "single" })), &meta()).await.expect("setSelectionMode");
36384 +             app.handle_action(SET_SELECTION_MODE_ACTION_ID, Some(&json!({ "domainId": "items", "mode": "single" })), &meta()).await.expect("setSelectionMode");
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:36385:94
      |
36385 |             assert_eq!(app.interaction_state().await.active_mode.get("items").copied(), Some(protocol::SelectionMode::Single));
      |                                                                                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
36385 -             assert_eq!(app.interaction_state().await.active_mode.get("items").copied(), Some(protocol::SelectionMode::Single));
36385 +             assert_eq!(app.interaction_state().await.active_mode.get("items").copied(), Some(SelectionMode::Single));
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:36387:31
      |
36387 | ...   app.handle_action(semio_framework::SET_INTERACTION_GRANULARITY_ACTION_ID, Some(&json!({ "domainId": "items", "granularityId...
      |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
36387 -             app.handle_action(semio_framework::SET_INTERACTION_GRANULARITY_ACTION_ID, Some(&json!({ "domainId": "items", "granularityId": "item" })), &meta()).await.expect("setInteractionGranularity");
36387 +             app.handle_action(SET_INTERACTION_GRANULARITY_ACTION_ID, Some(&json!({ "domainId": "items", "granularityId": "item" })), &meta()).await.expect("setInteractionGranularity");
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:36391:43
      |
36391 | ...   let error = app.handle_action(semio_framework::SET_INTERACTION_GRANULARITY_ACTION_ID, Some(&json!({ "domainId": "items", "g...
      |                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
36391 -             let error = app.handle_action(semio_framework::SET_INTERACTION_GRANULARITY_ACTION_ID, Some(&json!({ "domainId": "items", "granularityId": "bogus" })), &meta()).await.expect_err("undeclared granularity must be rejected");
36391 +             let error = app.handle_action(SET_INTERACTION_GRANULARITY_ACTION_ID, Some(&json!({ "domainId": "items", "granularityId": "bogus" })), &meta()).await.expect_err("undeclared granularity must be rejected");
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:36400:31
      |
36400 |             app.handle_action(semio_framework::SELECT_ALL_ACTION_ID, None, &meta()).await.expect("selectAll");
      |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
36400 -             app.handle_action(semio_framework::SELECT_ALL_ACTION_ID, None, &meta()).await.expect("selectAll");
36400 +             app.handle_action(SELECT_ALL_ACTION_ID, None, &meta()).await.expect("selectAll");
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:36403:31
      |
36403 |             app.handle_action(semio_framework::CLEAR_SELECTION_ACTION_ID, None, &meta()).await.expect("clearSelection");
      |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
36403 -             app.handle_action(semio_framework::CLEAR_SELECTION_ACTION_ID, None, &meta()).await.expect("clearSelection");
36403 +             app.handle_action(CLEAR_SELECTION_ACTION_ID, None, &meta()).await.expect("clearSelection");
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:36411:31
      |
36411 | ...   app.handle_action(semio_framework::INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items",...
      |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
36411 -             app.handle_action(semio_framework::INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "item-1")), &meta()).await.expect("interactionSelect");
36411 +             app.handle_action(INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "item-1")), &meta()).await.expect("interactionSelect");
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:36425:31
      |
36425 | ...   app.handle_action(semio_framework::INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items",...
      |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
36425 -             app.handle_action(semio_framework::INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "item-1")), &meta()).await.expect("interactionSelect");
36425 +             app.handle_action(INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "item-1")), &meta()).await.expect("interactionSelect");
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:36428:39
      |
36428 |             assert_eq!(row.action_id, semio_framework::INTERACTION_SELECT_ACTION_ID);
      |                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
36428 -             assert_eq!(row.action_id, semio_framework::INTERACTION_SELECT_ACTION_ID);
36428 +             assert_eq!(row.action_id, INTERACTION_SELECT_ACTION_ID);
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:36443:64
      |
36443 |                 let builder = resolve_ready(__base.interaction(semio_framework::InteractionDefinition {
      |                                                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
36443 -                 let builder = resolve_ready(__base.interaction(semio_framework::InteractionDefinition {
36443 +                 let builder = resolve_ready(__base.interaction(InteractionDefinition {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:36446:41
      |
36446 | ...   granularities: vec![semio_framework::GranularityDefinition { id: "item".into(), label: LocalizedLabel::data("Item"), icon_i...
      |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
36446 -                     granularities: vec![semio_framework::GranularityDefinition { id: "item".into(), label: LocalizedLabel::data("Item"), icon_id: IconName::AppWindow }],
36446 +                     granularities: vec![GranularityDefinition { id: "item".into(), label: LocalizedLabel::data("Item"), icon_id: IconName::AppWindow }],
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:36447:32
      |
36447 |                     hierarchy: protocol::HierarchyProvider::Flat,
      |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
36447 -                     hierarchy: protocol::HierarchyProvider::Flat,
36447 +                     hierarchy: HierarchyProvider::Flat,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:36448:28
      |
36448 |                     hover: protocol::HoverSpec { transitive: true, ..protocol::HoverSpec::default() },
      |                            ^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
36448 -                     hover: protocol::HoverSpec { transitive: true, ..protocol::HoverSpec::default() },
36448 +                     hover: HoverSpec { transitive: true, ..protocol::HoverSpec::default() },
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:36448:70
      |
36448 |                     hover: protocol::HoverSpec { transitive: true, ..protocol::HoverSpec::default() },
      |                                                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
36448 -                     hover: protocol::HoverSpec { transitive: true, ..protocol::HoverSpec::default() },
36448 +                     hover: protocol::HoverSpec { transitive: true, ..HoverSpec::default() },
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:36449:32
      |
36449 | ...   selection: protocol::SelectionSpec { modes: vec![protocol::SelectionMode::Single], methods: vec![protocol::SelectionMethod:...
      |                  ^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
36449 -                     selection: protocol::SelectionSpec { modes: vec![protocol::SelectionMode::Single], methods: vec![protocol::SelectionMethod::Pick], merges: vec![protocol::MergeMode::Replace], transitive: false, broadcast: true },
36449 +                     selection: SelectionSpec { modes: vec![protocol::SelectionMode::Single], methods: vec![protocol::SelectionMethod::Pick], merges: vec![protocol::MergeMode::Replace], transitive: false, broadcast: true },
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:36449:70
      |
36449 | ...   selection: protocol::SelectionSpec { modes: vec![protocol::SelectionMode::Single], methods: vec![protocol::SelectionMethod:...
      |                                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
36449 -                     selection: protocol::SelectionSpec { modes: vec![protocol::SelectionMode::Single], methods: vec![protocol::SelectionMethod::Pick], merges: vec![protocol::MergeMode::Replace], transitive: false, broadcast: true },
36449 +                     selection: protocol::SelectionSpec { modes: vec![SelectionMode::Single], methods: vec![protocol::SelectionMethod::Pick], merges: vec![protocol::MergeMode::Replace], transitive: false, broadcast: true },
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:36449:118
      |
36449 | ...rotocol::SelectionMode::Single], methods: vec![protocol::SelectionMethod::Pick], merges: vec![protocol::MergeMode::Replace], t...
      |                                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
36449 -                     selection: protocol::SelectionSpec { modes: vec![protocol::SelectionMode::Single], methods: vec![protocol::SelectionMethod::Pick], merges: vec![protocol::MergeMode::Replace], transitive: false, broadcast: true },
36449 +                     selection: protocol::SelectionSpec { modes: vec![protocol::SelectionMode::Single], methods: vec![SelectionMethod::Pick], merges: vec![protocol::MergeMode::Replace], transitive: false, broadcast: true },
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:36449:165
      |
36449 | ...![protocol::SelectionMethod::Pick], merges: vec![protocol::MergeMode::Replace], transitive: false, broadcast: true },
      |                                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
36449 -                     selection: protocol::SelectionSpec { modes: vec![protocol::SelectionMode::Single], methods: vec![protocol::SelectionMethod::Pick], merges: vec![protocol::MergeMode::Replace], transitive: false, broadcast: true },
36449 +                     selection: protocol::SelectionSpec { modes: vec![protocol::SelectionMode::Single], methods: vec![protocol::SelectionMethod::Pick], merges: vec![MergeMode::Replace], transitive: false, broadcast: true },
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:36460:31
      |
36460 | ...   app.handle_action(semio_framework::INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items",...
      |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
36460 -             app.handle_action(semio_framework::INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "item-1")), &meta()).await.expect("interactionSelect");
36460 +             app.handle_action(INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "item-1")), &meta()).await.expect("interactionSelect");
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:36472:43
      |
36472 |                         interaction: Some(protocol::PresenceInteraction {
      |                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
36472 -                         interaction: Some(protocol::PresenceInteraction {
36472 +                         interaction: Some(PresenceInteraction {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:36474:43
      |
36474 | ...   domains: vec![protocol::PresenceDomain { domain: "items".to_string(), granularity: "item".to_string(), selected: vec!["item...
      |                     ^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
36474 -                             domains: vec![protocol::PresenceDomain { domain: "items".to_string(), granularity: "item".to_string(), selected: vec!["item-1".to_string()], hovered: Vec::new() }],
36474 +                             domains: vec![PresenceDomain { domain: "items".to_string(), granularity: "item".to_string(), selected: vec!["item-1".to_string()], hovered: Vec::new() }],
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:36488:24
      |
36488 |             let item = semio_framework_ui_runtime::TreeNode::try_new(
      |                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
36488 -             let item = semio_framework_ui_runtime::TreeNode::try_new(
36488 +             let item = TreeNode::try_new(
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:36490:17
      |
36490 |                 semio_framework_ui_contract::Component::TreeItem(semio_framework_ui_contract::TreeItemProps {
      |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
36490 -                 semio_framework_ui_contract::Component::TreeItem(semio_framework_ui_contract::TreeItemProps {
36490 +                 Component::TreeItem(semio_framework_ui_contract::TreeItemProps {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:36490:66
      |
36490 |                 semio_framework_ui_contract::Component::TreeItem(semio_framework_ui_contract::TreeItemProps {
      |                                                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
36490 -                 semio_framework_ui_contract::Component::TreeItem(semio_framework_ui_contract::TreeItemProps {
36490 +                 semio_framework_ui_contract::Component::TreeItem(TreeItemProps {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:36491:28
      |
36491 | ...   label: semio_framework_ui_contract::Label(semio_framework_ui_contract::UiText::try_from_str("Item 1").expect("bounded fixtu...
      |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
36491 -                     label: semio_framework_ui_contract::Label(semio_framework_ui_contract::UiText::try_from_str("Item 1").expect("bounded fixture")),
36491 +                     label: Label(semio_framework_ui_contract::UiText::try_from_str("Item 1").expect("bounded fixture")),
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:36491:63
      |
36491 | ...   label: semio_framework_ui_contract::Label(semio_framework_ui_contract::UiText::try_from_str("Item 1").expect("bounded fixtu...
      |                                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
36491 -                     label: semio_framework_ui_contract::Label(semio_framework_ui_contract::UiText::try_from_str("Item 1").expect("bounded fixture")),
36491 +                     label: semio_framework_ui_contract::Label(UiText::try_from_str("Item 1").expect("bounded fixture")),
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:36498:34
      |
36498 |                     row_actions: semio_framework_ui_contract::UiFixedList::default(),
      |                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
36498 -                     row_actions: semio_framework_ui_contract::UiFixedList::default(),
36498 +                     row_actions: UiFixedList::default(),
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:36502:27
      |
36502 | ...   let section = semio_framework_ui_runtime::TreeNode::try_new("sec", semio_framework_ui_contract::Component::TreeSection(semi...
      |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
36502 -             let section = semio_framework_ui_runtime::TreeNode::try_new("sec", semio_framework_ui_contract::Component::TreeSection(semio_framework_ui_contract::TreeSectionProps { label: None, default_open: None }))
36502 +             let section = TreeNode::try_new("sec", semio_framework_ui_contract::Component::TreeSection(semio_framework_ui_contract::TreeSectionProps { label: None, default_open: None }))
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:36502:80
      |
36502 | ...   let section = semio_framework_ui_runtime::TreeNode::try_new("sec", semio_framework_ui_contract::Component::TreeSection(semi...
      |                                                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
36502 -             let section = semio_framework_ui_runtime::TreeNode::try_new("sec", semio_framework_ui_contract::Component::TreeSection(semio_framework_ui_contract::TreeSectionProps { label: None, default_open: None }))
36502 +             let section = semio_framework_ui_runtime::TreeNode::try_new("sec", Component::TreeSection(semio_framework_ui_contract::TreeSectionProps { label: None, default_open: None }))
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:36502:132
      |
36502 | ...ork_ui_contract::Component::TreeSection(semio_framework_ui_contract::TreeSectionProps { label: None, default_open: None }))
      |                                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
36502 -             let section = semio_framework_ui_runtime::TreeNode::try_new("sec", semio_framework_ui_contract::Component::TreeSection(semio_framework_ui_contract::TreeSectionProps { label: None, default_open: None }))
36502 +             let section = semio_framework_ui_runtime::TreeNode::try_new("sec", semio_framework_ui_contract::Component::TreeSection(TreeSectionProps { label: None, default_open: None }))
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:36506:24
      |
36506 |             let root = semio_framework_ui_runtime::TreeNode::try_new(
      |                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
36506 -             let root = semio_framework_ui_runtime::TreeNode::try_new(
36506 +             let root = TreeNode::try_new(
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:36508:17
      |
36508 | ...   semio_framework_ui_contract::Component::Tree(semio_framework_ui_contract::TreeProps { interaction_domain: Some(semio_framew...
      |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
36508 -                 semio_framework_ui_contract::Component::Tree(semio_framework_ui_contract::TreeProps { interaction_domain: Some(semio_framework_ui_contract::UiText::try_from_str("items").expect("bounded fixture")) }),
36508 +                 Component::Tree(semio_framework_ui_contract::TreeProps { interaction_domain: Some(semio_framework_ui_contract::UiText::try_from_str("items").expect("bounded fixture")) }),
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:36508:62
      |
36508 | ...   semio_framework_ui_contract::Component::Tree(semio_framework_ui_contract::TreeProps { interaction_domain: Some(semio_framew...
      |                                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
36508 -                 semio_framework_ui_contract::Component::Tree(semio_framework_ui_contract::TreeProps { interaction_domain: Some(semio_framework_ui_contract::UiText::try_from_str("items").expect("bounded fixture")) }),
36508 +                 semio_framework_ui_contract::Component::Tree(TreeProps { interaction_domain: Some(semio_framework_ui_contract::UiText::try_from_str("items").expect("bounded fixture")) }),
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:36508:128
      |
36508 | ...:TreeProps { interaction_domain: Some(semio_framework_ui_contract::UiText::try_from_str("items").expect("bounded fixture")) }),
      |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
36508 -                 semio_framework_ui_contract::Component::Tree(semio_framework_ui_contract::TreeProps { interaction_domain: Some(semio_framework_ui_contract::UiText::try_from_str("items").expect("bounded fixture")) }),
36508 +                 semio_framework_ui_contract::Component::Tree(semio_framework_ui_contract::TreeProps { interaction_domain: Some(UiText::try_from_str("items").expect("bounded fixture")) }),
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:36513:24
      |
36513 |             let tree = semio_framework_ui_runtime::ComponentTree { root };
      |                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
36513 -             let tree = semio_framework_ui_runtime::ComponentTree { root };
36513 +             let tree = ComponentTree { root };
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:36597:65
      |
36597 | ...   crate::plugin_runtime::test_push_instance(&runtime, crate::plugin_runtime::AppInstance { id: resumed_instance, app: TestRun...
      |                                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
36597 -             crate::plugin_runtime::test_push_instance(&runtime, crate::plugin_runtime::AppInstance { id: resumed_instance, app: TestRuntimeApps::from(app) }).await;
36597 +             crate::plugin_runtime::test_push_instance(&runtime, AppInstance { id: resumed_instance, app: TestRuntimeApps::from(app) }).await;
      |

warning: ambiguous glob re-exports
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:37657:13
      |
37657 |     pub use semio_framework::kernel::*;
      |             ^^^^^^^^^^^^^^^^^^^^^^^^^^ the name `ActionId` in the type namespace is first re-exported here
...
37660 |     pub use semio_framework_ui_contract::*;
      |             ------------------------------ but the name `ActionId` in the type namespace is also re-exported here
      |
      = note: `#[warn(ambiguous_glob_reexports)]` on by default

warning: ambiguous glob re-exports
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:37660:13
      |
37656 |     pub use crate::app::*;
      |             ------------- but the name `tree_item` in the value namespace is also re-exported here
...
37660 |     pub use semio_framework_ui_contract::*;
      |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the name `tree_item` in the value namespace is first re-exported here

warning: ambiguous glob re-exports
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:37660:13
      |
37656 |     pub use crate::app::*;
      |             ------------- but the name `PeerMark` in the type namespace is also re-exported here
...
37660 |     pub use semio_framework_ui_contract::*;
      |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the name `PeerMark` in the type namespace is first re-exported here

error[E0599]: no method named `publish` found for struct `semio_framework_ui_runtime::SurfaceReconcileReadyPatch` in the current scope
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🦀️component.rs:2487:34
     |
2487 |                     return owner.publish().map(|(patch, _)| patch);
     |                                  ^^^^^^^
     |
help: there is a method `publish_into` with a similar name, but with different arguments
    --> 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs:2760:5
     |
2760 |     pub fn publish_into(&mut self, payload: &mut ui_contract::UiPendingPatch, published: &mut Option<SurfaceReconcilePublishedPatch>, admitted_bytes: usize) -> Result<usize, &'static str> {
     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: variable does not need to be mutable
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:30092:17
      |
30092 |             let mut instance = find_instance(list, instance_id)?;
      |                 ----^^^^^^^^
      |                 |
      |                 help: remove this `mut`
      |
      = note: `#[warn(unused_mut)]` (part of `#[warn(unused)]`) on by default

warning: variable does not need to be mutable
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:30496:17
      |
30496 |             let mut instance = find_instance(list, instance_id)?;
      |                 ----^^^^^^^^
      |                 |
      |                 help: remove this `mut`

warning: variable does not need to be mutable
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:30515:17
      |
30515 |             let mut instance = find_instance(list, instance_id)?;
      |                 ----^^^^^^^^
      |                 |
      |                 help: remove this `mut`

warning: variable does not need to be mutable
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:31645:37
      |
31645 | ...                   let mut instance = find_instance(list, instance_id)?;
      |                           ----^^^^^^^^
      |                           |
      |                           help: remove this `mut`

warning: variable does not need to be mutable
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:31725:41
      |
31725 | ...                   let mut instance = find_instance(list, instance_id)?;
      |                           ----^^^^^^^^
      |                           |
      |                           help: remove this `mut`

warning: variable does not need to be mutable
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:31775:29
      |
31775 |                         let mut instance = find_instance(list, instance_id)?;
      |                             ----^^^^^^^^
      |                             |
      |                             help: remove this `mut`

warning: variable does not need to be mutable
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:31809:29
      |
31809 |                         let mut instance = find_instance(list, instance_id)?;
      |                             ----^^^^^^^^
      |                             |
      |                             help: remove this `mut`

warning: variable does not need to be mutable
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:31996:29
      |
31996 |                         let mut instance = find_instance(list, instance_id)?;
      |                             ----^^^^^^^^
      |                             |
      |                             help: remove this `mut`

warning: variable does not need to be mutable
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32028:17
      |
32028 |             let mut instance = find_instance(list, instance_id)?;
      |                 ----^^^^^^^^
      |                 |
      |                 help: remove this `mut`

warning: variable does not need to be mutable
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:31446:13
      |
31446 |         let mut retry_command = None;
      |             ----^^^^^^^^^^^^^
      |             |
      |             help: remove this `mut`

warning: variable does not need to be mutable
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:31448:13
      |
31448 |         let mut presence_pending = None;
      |             ----^^^^^^^^^^^^^^^^
      |             |
      |             help: remove this `mut`

error[E0308]: mismatched types
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33857:154
      |
33857 | ...-> Option<Box<dyn ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> { TestApp::build_presence_store_disposer() }
      |       ----------------------------------------------------------------------------------------------------   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `NoPresence`, found `presence::PublicationPresence`
      |       |
      |       expected `Option<Box<dyn ArtifactOwnedDisposer<PresenceStore<NoPresence, ...>>>>` because of return type
      |
      = note: expected enum `std::option::Option<std::boxed::Box<(dyn component::app::ArtifactOwnedDisposer<PresenceStore<NoPresence, NoPresenceMutation>> + 'static)>>`
                 found enum `Option<Box<dyn ArtifactOwnedDisposer<PresenceStore<..., ...>>>>`
      = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_plugin-d81b8cb7f98afff8.long-type-9679788482582067593.txt'
      = note: consider using `--verbose` to print the full type name to the console

error[E0308]: mismatched types
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33858:158
      |
33858 | ...-> Option<Box<dyn ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> { TestApp::build_transient_store_disposer() }
      |       -------------------------------------------------------------------------------------------------------   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `NoTransient`, found `transient::PublicationTransient`
      |       |
      |       expected `Option<Box<dyn ArtifactOwnedDisposer<TransientStore<..., ...>>>>` because of return type
      |
      = note: expected enum `std::option::Option<std::boxed::Box<(dyn component::app::ArtifactOwnedDisposer<TransientStore<NoTransient, NoTransientMutation>> + 'static)>>`
                 found enum `Option<Box<dyn ArtifactOwnedDisposer<TransientStore<..., ...>>>>`
      = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_plugin-d81b8cb7f98afff8.long-type-8363216533096283407.txt'
      = note: consider using `--verbose` to print the full type name to the console

error[E0308]: mismatched types
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33859:139
      |
33859 | ...-> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> { TestApp::build_presence_peer_retirement_factory() }
      |       ----------------------------------------------------------------------------   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `NoPresence`, found `presence::PublicationPresence`
      |       |
      |       expected `std::option::Option<std::sync::Arc<(dyn SnapshotRetirementFactory<NoPresence> + 'static)>>` because of return type
      |
      = note: expected enum `std::option::Option<std::sync::Arc<(dyn SnapshotRetirementFactory<NoPresence> + 'static)>>`
                 found enum `std::option::Option<std::sync::Arc<(dyn SnapshotRetirementFactory<presence::PublicationPresence> + 'static)>>`

error[E0308]: mismatched types
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33860:145
      |
33860 | ...-> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> { TestApp::build_presence_local_root_retirement_factory() }
      |       ----------------------------------------------------------------------------   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `NoPresence`, found `presence::PublicationPresence`
      |       |
      |       expected `std::option::Option<std::sync::Arc<(dyn SnapshotRetirementFactory<NoPresence> + 'static)>>` because of return type
      |
      = note: expected enum `std::option::Option<std::sync::Arc<(dyn SnapshotRetirementFactory<NoPresence> + 'static)>>`
                 found enum `std::option::Option<std::sync::Arc<(dyn SnapshotRetirementFactory<presence::PublicationPresence> + 'static)>>`

error[E0308]: mismatched types
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33861:147
      |
33861 | ...-> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> { TestApp::build_transient_local_root_retirement_factory() }
      |       -----------------------------------------------------------------------------   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `NoTransient`, found `transient::PublicationTransient`
      |       |
      |       expected `std::option::Option<std::sync::Arc<(dyn SnapshotRetirementFactory<NoTransient> + 'static)>>` because of return type
      |
      = note: expected enum `std::option::Option<std::sync::Arc<(dyn SnapshotRetirementFactory<NoTransient> + 'static)>>`
                 found enum `std::option::Option<std::sync::Arc<(dyn SnapshotRetirementFactory<transient::PublicationTransient> + 'static)>>`

error[E0559]: variant `document::mutations::TestMutation::SetCount` has no field named `value`
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🕹️interaction/📡️live/🧪️dispatch/🧪️component.rs:268:99
    |
268 | ...   let operation = <TestMutation as protocol::OpBinary>::encode_op(&TestMutation::SetCount { value: prepare_seq as i32 }).unwrap();
    |                                                                                                 ^^^^^ field does not exist
    |
   ::: 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🧬️test-app-mutations/🧬️document/🧬️mutations/🦀️.rs:10:32
    |
 10 | pub(crate) enum TestMutation { SetCount(SetCount), SetLabel(SetLabel) }
    |                                -------- `document::mutations::TestMutation::SetCount` defined here
    |
help: `document::mutations::TestMutation::SetCount` is a tuple variant, use the appropriate syntax
    |
268 -         let operation = <TestMutation as protocol::OpBinary>::encode_op(&TestMutation::SetCount { value: prepare_seq as i32 }).unwrap();
268 +         let operation = <TestMutation as protocol::OpBinary>::encode_op(&TestMutation::SetCount(/* set_count::SetCount */)).unwrap();
    |

warning: use of deprecated method `std::sync::atomic::Atomic::<u64>::fetch_update`: renamed to `try_update` for consistency
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:30043:22
      |
30043 |                     .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |generation| generation.checked_add(1))
      |                      ^^^^^^^^^^^^
      |
      = note: `#[warn(deprecated)]` on by default
help: replace the use of the deprecated method
      |
30043 -                     .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |generation| generation.checked_add(1))
30043 +                     .try_update(Ordering::SeqCst, Ordering::SeqCst, |generation| generation.checked_add(1))
      |

warning: use of deprecated method `std::sync::atomic::Atomic::<u64>::fetch_update`: renamed to `try_update` for consistency
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:15443:41
      |
15443 | ...   let _ = self.app_generation.fetch_update(std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire, |genera...
      |                                   ^^^^^^^^^^^^
      |
help: replace the use of the deprecated method
      |
15443 -             let _ = self.app_generation.fetch_update(std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire, |generation| Some(generation.saturating_add(1)));
15443 +             let _ = self.app_generation.try_update(std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire, |generation| Some(generation.saturating_add(1)));
      |

warning: use of deprecated method `std::sync::atomic::Atomic::<usize>::fetch_update`: renamed to `try_update` for consistency
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:14358:43
      |
14358 | ...   let result = self.state.bytes.fetch_update(std::sync::atomic::Ordering::SeqCst, std::sync::atomic::Ordering::SeqCst, |curre...
      |                                     ^^^^^^^^^^^^
      |
help: replace the use of the deprecated method
      |
14358 -             let result = self.state.bytes.fetch_update(std::sync::atomic::Ordering::SeqCst, std::sync::atomic::Ordering::SeqCst, |current| current.checked_add(bytes).filter(|next| *next <= self.maximum)).map(|previous| previous + bytes);
14358 +             let result = self.state.bytes.try_update(std::sync::atomic::Ordering::SeqCst, std::sync::atomic::Ordering::SeqCst, |current| current.checked_add(bytes).filter(|next| *next <= self.maximum)).map(|previous| previous + bytes);
      |

error[E0616]: field `count` of struct `TxnSnapshot` is private
 --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/📝️set-transaction-count/🦀️.rs:6:418
  |
6 | ...e:&TxnSnapshot)->Vec<TxnMutation>{vec![Self{value:base.count}.into()]} fn label(&self)->String{format!("Set transaction count to {...
  |                                                           ^^^^^ private field

error[E0616]: field `count` of struct `TxnSnapshot` is private
 --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/📝️set-transaction-count-without-preflight/🦀️.rs:6:491
  |
6 | ...->Vec<TxnMutation>{vec![SetTransactionCount{value:base.count}.into()]} fn label(&self)->String{format!("Set transaction count with...
  |                                                           ^^^^^ private field

error[E0616]: field `count` of struct `TxnSnapshot` is private
 --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/📝️set-transaction-count-and-notify/🦀️.rs:6:463
  |
6 | ...->Vec<TxnMutation>{vec![SetTransactionCount{value:base.count}.into()]} fn label(&self)->String{format!("Set transaction count and ...
  |                                                           ^^^^^ private field

error[E0616]: field `count` of struct `SurfaceSnapshot` is private
 --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🧬️mutation-fixtures/🪟️surface/🧬️mutations/📝️set-surface-count/🦀️.rs:6:420
  |
6 | ...ceSnapshot)->Vec<SurfaceMutation>{vec![Self{value:base.count}.into()]} fn label(&self)->String{format!("Set surface count to {}",s...
  |                                                           ^^^^^ private field

error[E0616]: field `count` of struct `DummySnapshot` is private
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🧬️mutation-fixtures/🎲️dummy/🧬️mutations/📝️set-dummy-count/🦀️.rs:11:432
   |
11 | ...pshot) -> Vec<DummyMutation> { vec![Self { value: base.count }.into()] } fn label(&self) -> String { format!("Set dummy count to ...
   |                                                           ^^^^^ private field

warning: unused import: `Mutation`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32723:24
      |
32723 |         use protocol::{Mutation, MutationDiff};
      |                        ^^^^^^^^

warning: unused import: `MutationKind`
 --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🧬️contributed-mutation-wire/🧪️tests/🦀️.rs:5:63
  |
5 | use protocol::{CompositeMutationKind, Mutation, MutationDiff, MutationKind, MutationLeaf, OpBinary};
  |                                                               ^^^^^^^^^^^^

warning: unused variable: `parent_document_id`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:19414:17
      |
19414 |             let parent_document_id = self.store.envelope().id.clone();
      |                 ^^^^^^^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_parent_document_id`
      |
      = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

warning: unused variable: `restart_command`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:36728:21
      |
36728 |                 let restart_command = restart_command.clone();
      |                     ^^^^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_restart_command`

warning: unused variable: `envelope_seq`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:31525:22
      |
31525 |         if let Some((envelope_seq, mut owner)) = command {
      |                      ^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_envelope_seq`

warning: unused variable: `actor`
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:9933:26
     |
9933 |                     let (actor, pack) = self.packs[self.packs_len - 1].as_ref().expect("retained app-typed presence pack");
     |                          ^^^^^ help: if this is intentional, prefix it with an underscore: `_actor`

Some errors have detailed explanations: E0308, E0432, E0559, E0599, E0603, E0616.
For more information about an error, try `rustc --explain E0308`.
warning: `semio-framework-plugin` (lib test) generated 535 warnings
error: could not compile `semio-framework-plugin` (lib test) due to 19 previous errors; 538 warnings emitted
1741 |  * throws on non-zero exit, signal, or budget exceed (the `[budget]` line is printed
1742 |  * to stderr first so it survives a caller's try/catch, e.g. [[tryRun]]).
1743 |  */
1744 | export function runCmd(cmd: string, args: string[], opts: RunCmdOpts = {}): void {
1745 |   const status = runCmdInternal(cmd, args, opts);
1746 |   if (status !== 0) throw new Error(`${cmd} ${args.join(" ")} exited with status ${status}`);
                                     ^
error: cargo test --manifest-path Cargo.toml --lib --no-run exited with status 101
      at runCmd (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts:1746:31)
      at runCargo (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts:2693:3)
      at run (/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️script.ts:14:11)
      at run (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts:1048:71)
      at runBundleScriptMain (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts:1078:16)

Bun v1.3.14 (macOS arm64)
Warning: command "bun 📜️script.ts test --no-run" exited with non-zero status code


 NX   Running target test for project @semio-tech/framework-plugin failed

Failed tasks:

- @semio-tech/framework-plugin:test

Hint: run the command with --verbose for more details.


```
