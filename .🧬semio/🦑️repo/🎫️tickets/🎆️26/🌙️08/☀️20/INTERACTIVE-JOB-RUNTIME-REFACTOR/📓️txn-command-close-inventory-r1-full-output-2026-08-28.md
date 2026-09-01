# Transaction Command Close Inventory R1 Full Output

Complete raw inventory copied through 44 checked, untruncated chunks covering 8644 lines. Original tee stream is retained separately. Cargo/Nx0,25.53s; this is build inventory,not execution of the six tests. No warning or output is elided below.

```text

> nx run @semio-tech/framework-plugin:test --args=exhaustive --no-run txn_command_close_

> bun 📜️script.ts test exhaustive --no-run txn_command_close_

[0m[33mWarning[0m[2m:[0m [1mThe 'NO_COLOR' env is ignored due to the 'FORCE_COLOR' env being set.[0m
[0m      [2mat [0m[0m[1m[3mwarnOnDeactivatedColors[0m[2m ([0m[0m[36minternal:tty[0m[2m:[0m[33m33[0m[2m:[33m24[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3mgetColorDepth[0m[2m ([0m[0m[36minternal:tty[0m[2m:[0m[33m42[0m[2m:[33m39[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3mshouldColorize[0m[2m ([0m[0m[36minternal:util/colors[0m[2m:[0m[33m14[0m[2m:[33m109[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3mrefresh[0m[2m ([0m[0m[36minternal:util/colors[0m[2m:[0m[33m18[0m[2m:[33m31[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3minternal:util/colors[0m[2m ([0m[0m[36minternal:util/colors[0m[2m:[0m[33m24[0m[2m:[33m16[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3minternal:assert/assertion_error[0m[2m ([0m[0m[36minternal:assert/assertion_error[0m[2m:[0m[33m2[0m[2m:[33m187[0m[2m)[0m
[0m      [2mat [0m[0m[1m[3mloadAssertionError[0m[2m ([0m[0m[36mnode:assert[0m[2m:[0m[33m28[0m[2m:[33m96[0m[2m)[0m

[DEBUG] plugin-runner-oracle cases=6
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

warning: `semio-framework-os-kernel-dsl-derive` (lib) generated 1 warning
warning: `semio-framework-pack` (lib) generated 1 warning
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
  --> 🧰️framework/📦️packages/🦀️rust/../../🛍️products/💻️os/🔨️modules/🔁️workflow/🧬️schema/🧬️mutations/🔗connect-ports/🦀️.rs:16:23
   |
16 |     fn inverse(&self, base: &WorkflowSnapshot) -> Vec<WorkflowMutation> { vec![WorkflowMutation::DisconnectEdge(DisconnectEdge { edg...
   |                       ^^^^ help: if this is intentional, prefix it with an underscore: `_base`

warning: unused variable: `base`
  --> 🧰️framework/📦️packages/🦀️rust/../../🛍️products/💻️os/🔨️modules/🔁️workflow/🧬️schema/🧬️mutations/➕️add-node/🦀️.rs:16:23
   |
16 |     fn inverse(&self, base: &WorkflowSnapshot) -> Vec<WorkflowMutation> { vec![WorkflowMutation::RemoveNode(RemoveNode { node_id: se...
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
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:27531:17
      |
27531 |             use crate::__semio_dispatch_PluginApp;
      |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
note: the macro is defined here
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:11479:5
      |
11479 |     #[dyn_enum]
      |     ^^^^^^^^^^^
      = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
      = note: for more information, see issue #52234 <https://github.com/rust-lang/rust/issues/52234>
      = note: `-W macro-expanded-macro-exports-accessed-by-absolute-paths` implied by `-W future-incompatible`
      = help: to override `-W future-incompatible` add `#[allow(macro_expanded_macro_exports_accessed_by_absolute_paths)]`
      = note: this warning originates in the attribute macro `dyn_enum` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: macro-expanded `macro_export` macros from the current crate cannot be referred to by absolute paths
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33240:13
      |
33240 |         use crate::__semio_dispatch_PluginApp;
      |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
note: the macro is defined here
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:11479:5
      |
11479 |     #[dyn_enum]
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
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:11479:5
      |
11479 |     #[dyn_enum]
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
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🩹️patches/🦀️component.rs:258:10
    |
258 |     key: super::instance_lifetime::NativeCloseKey,
    |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
258 -     key: super::instance_lifetime::NativeCloseKey,
258 +     key: NativeCloseKey,
    |

warning: unnecessary qualification
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🩹️patches/🦀️component.rs:555:24
    |
555 |         let metadata = std::mem::size_of::<ReadySlot>();
    |                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
555 -         let metadata = std::mem::size_of::<ReadySlot>();
555 +         let metadata = size_of::<ReadySlot>();
    |

warning: unnecessary qualification
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🩹️patches/🦀️component.rs:648:54
    |
648 |     pub(crate) fn reserve_close_instance(&self, key: super::instance_lifetime::NativeCloseKey) -> Result<(), &'static str> {
    |                                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
648 -     pub(crate) fn reserve_close_instance(&self, key: super::instance_lifetime::NativeCloseKey) -> Result<(), &'static str> {
648 +     pub(crate) fn reserve_close_instance(&self, key: NativeCloseKey) -> Result<(), &'static str> {
    |

warning: unnecessary qualification
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🩹️patches/🦀️component.rs:666:55
    |
666 |     pub(crate) fn activate_close_instance(&self, key: super::instance_lifetime::NativeCloseKey) -> Result<(), &'static str> {
    |                                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
666 -     pub(crate) fn activate_close_instance(&self, key: super::instance_lifetime::NativeCloseKey) -> Result<(), &'static str> {
666 +     pub(crate) fn activate_close_instance(&self, key: NativeCloseKey) -> Result<(), &'static str> {
    |

warning: unnecessary qualification
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🩹️patches/🦀️component.rs:673:55
    |
673 |     pub(crate) fn close_instance_complete(&self, key: super::instance_lifetime::NativeCloseKey) -> Result<bool, &'static str> {
    |                                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
673 -     pub(crate) fn close_instance_complete(&self, key: super::instance_lifetime::NativeCloseKey) -> Result<bool, &'static str> {
673 +     pub(crate) fn close_instance_complete(&self, key: NativeCloseKey) -> Result<bool, &'static str> {
    |

warning: unnecessary qualification
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🩹️patches/🦀️component.rs:678:54
    |
678 |     pub(crate) fn release_close_instance(&self, key: super::instance_lifetime::NativeCloseKey) -> Result<(), &'static str> {
    |                                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
678 -     pub(crate) fn release_close_instance(&self, key: super::instance_lifetime::NativeCloseKey) -> Result<(), &'static str> {
678 +     pub(crate) fn release_close_instance(&self, key: NativeCloseKey) -> Result<(), &'static str> {
    |

warning: unnecessary qualification
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🩹️patches/🦀️component.rs:897:30
    |
897 |         let receiver_bytes = std::mem::size_of::<ReadySlot>();
    |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
897 -         let receiver_bytes = std::mem::size_of::<ReadySlot>();
897 +         let receiver_bytes = size_of::<ReadySlot>();
    |

warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🩹️patches/🦀️component.rs:1062:19
     |
1062 |         let key = super::super::instance_lifetime::NativeCloseKey::fixture(instance, 1);
     |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
1062 -         let key = super::super::instance_lifetime::NativeCloseKey::fixture(instance, 1);
1062 +         let key = NativeCloseKey::fixture(instance, 1);
     |

warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🩹️patches/🦀️component.rs:1099:27
     |
1099 |         let mut outputs = semio_framework_ui_runtime::SurfaceReconcileOutputs::default();
     |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
1099 -         let mut outputs = semio_framework_ui_runtime::SurfaceReconcileOutputs::default();
1099 +         let mut outputs = SurfaceReconcileOutputs::default();
     |

warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🩹️patches/🦀️component.rs:1287:21
     |
1287 |         let bytes = std::mem::size_of::<PatchTrackerState>();
     |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
1287 -         let bytes = std::mem::size_of::<PatchTrackerState>();
1287 +         let bytes = size_of::<PatchTrackerState>();
     |

warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🩹️patches/🦀️component.rs:1742:19
     |
1742 |         let key = super::super::instance_lifetime::NativeCloseKey::fixture(instance, 1);
     |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
1742 -         let key = super::super::instance_lifetime::NativeCloseKey::fixture(instance, 1);
1742 +         let key = NativeCloseKey::fixture(instance, 1);
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
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:11771:5
      |
11771 | /     /// 🗃️ sdk-dedyn (O1/§1.5): the default-composes-nothing case (design-dedyn.md §1.6's `NoMembers`
11772 | |     /// pattern, applied here to `PluginApp`) — a zero-variant enum, `dyn_enum_close!`-generated
11773 | |     /// (every method's body degenerates to `match *self {}` since there is no value to construct).
11774 | |     /// The default `PA` for every generic in this file's declaration tree, so a library-only plugin
11775 | |     /// (or a test that never actually instantiates an app) never has to name a real app enum.
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

warning: unnecessary qualification
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././🧪️tests/🧬️mutation-fixtures/🔀️transaction/🧪️tests/🧪️command-close/🦀️.rs:70:25
   |
70 |     let command_bytes = std::mem::size_of_val(command.as_ref());
   |                         ^^^^^^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
70 -     let command_bytes = std::mem::size_of_val(command.as_ref());
70 +     let command_bytes = size_of_val(command.as_ref());
   |

warning: unused import: `Mutation`
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././🧪️tests/🧬️mutation-fixtures/🔀️transaction/🦀️.rs:13:16
   |
13 | use protocol::{Mutation, MutationDiff};
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
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:6971:64
     |
6971 | ...   pub async fn assert_declaration_tree_registers_all<PA: super::PluginApp>(plugin_id: &str, declaration: declarations::Artifac...
     |                                                              ^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
6971 -         pub async fn assert_declaration_tree_registers_all<PA: super::PluginApp>(plugin_id: &str, declaration: declarations::ArtifactDeclaration<PA>) {
6971 +         pub async fn assert_declaration_tree_registers_all<PA: PluginApp>(plugin_id: &str, declaration: declarations::ArtifactDeclaration<PA>) {
     |

warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:7005:68
     |
7005 | ...   pub async fn assert_declaration_registration_is_atomic<PA: super::PluginApp>(plugin_id: &str, invalid: declarations::Artifac...
     |                                                                  ^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
7005 -         pub async fn assert_declaration_registration_is_atomic<PA: super::PluginApp>(plugin_id: &str, invalid: declarations::ArtifactDeclaration<PA>) {
7005 +         pub async fn assert_declaration_registration_is_atomic<PA: PluginApp>(plugin_id: &str, invalid: declarations::ArtifactDeclaration<PA>) {
     |

warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:7016:68
     |
7016 | ...   pub async fn assert_subset_declaration_ids_are_derived<PA: super::PluginApp>(declaration: &declarations::ArtifactDeclaration...
     |                                                                  ^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
7016 -         pub async fn assert_subset_declaration_ids_are_derived<PA: super::PluginApp>(declaration: &declarations::ArtifactDeclaration<PA>) {
7016 +         pub async fn assert_subset_declaration_ids_are_derived<PA: PluginApp>(declaration: &declarations::ArtifactDeclaration<PA>) {
     |

warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:9771:77
     |
9771 | ...ocol::MutationLeafDescriptor] = &[<crate::local_interaction::set_state::SetInteractionState as protocol::MutationLeaf>::DESCRIP...
     |                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
9771 -         const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[<crate::local_interaction::set_state::SetInteractionState as protocol::MutationLeaf>::DESCRIPTOR];
9771 +         const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[<SetInteractionState as protocol::MutationLeaf>::DESCRIPTOR];
     |

warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:9954:27
     |
9954 |             let builder = ui::button(ui_label(label, "history-panel.button-label")?).icon(icon.clone()).disabled(!enabled);
     |                           ^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
9954 -             let builder = ui::button(ui_label(label, "history-panel.button-label")?).icon(icon.clone()).disabled(!enabled);
9954 +             let builder = button(ui_label(label, "history-panel.button-label")?).icon(icon.clone()).disabled(!enabled);
     |

warning: unnecessary qualification
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:9968:34
     |
9968 |         let mut filter_builder = ui::select(ui_text(filter_value, "history-panel.filter-value")?);
     |                                  ^^^^^^^^^^
     |
help: remove the unnecessary path segments
     |
9968 -         let mut filter_builder = ui::select(ui_text(filter_value, "history-panel.filter-value")?);
9968 +         let mut filter_builder = select(ui_text(filter_value, "history-panel.filter-value")?);
     |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:10028:31
      |
10028 | ...   let actions_builder = ui::tree_section(ui_label(if is_de { "Aktionen" } else { "Actions" }, "history-panel.actions-label")?...
      |                             ^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
10028 -         let actions_builder = ui::tree_section(ui_label(if is_de { "Aktionen" } else { "Actions" }, "history-panel.actions-label")?).default_open(true);
10028 +         let actions_builder = tree_section(ui_label(if is_de { "Aktionen" } else { "Actions" }, "history-panel.actions-label")?).default_open(true);
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:10031:32
      |
10031 | ...   let commands_builder = ui::tree_section(ui_label(if is_de { "Befehle" } else { "Commands" }, "history-panel.commands-label"...
      |                              ^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
10031 -         let commands_builder = ui::tree_section(ui_label(if is_de { "Befehle" } else { "Commands" }, "history-panel.commands-label")?).default_open(true);
10031 +         let commands_builder = tree_section(ui_label(if is_de { "Befehle" } else { "Commands" }, "history-panel.commands-label")?).default_open(true);
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:10037:9
      |
10037 | ...   ui::tree().try_children(sections).map_err(|_| ui_assembly_error("history-panel.sections"))?.try_build().map_err(|_| ui_asse...
      |       ^^^^^^^^
      |
help: remove the unnecessary path segments
      |
10037 -         ui::tree().try_children(sections).map_err(|_| ui_assembly_error("history-panel.sections"))?.try_build().map_err(|_| ui_assembly_error("history-panel.build"))
10037 +         tree().try_children(sections).map_err(|_| ui_assembly_error("history-panel.sections"))?.try_build().map_err(|_| ui_assembly_error("history-panel.build"))
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:12201:18
      |
12201 |             Fut: std::future::Future<Output = Menu<'a>>,
      |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
12201 -             Fut: std::future::Future<Output = Menu<'a>>,
12201 +             Fut: Future<Output = Menu<'a>>,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:12959:29
      |
12959 | ...   fn try_serialize<T: serde::Serialize>(token: TypedOperationResultToken, lane: TypedOperationResultLane, value: &T) -> Resul...
      |                           ^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
12959 -         fn try_serialize<T: serde::Serialize>(token: TypedOperationResultToken, lane: TypedOperationResultLane, value: &T) -> Result<Self, Fault> {
12959 +         fn try_serialize<T: Serialize>(token: TypedOperationResultToken, lane: TypedOperationResultLane, value: &T) -> Result<Self, Fault> {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:13275:32
      |
13275 |         C: Clone + Serialize + serde::de::DeserializeOwned + ArtifactPack + Send + Sync + 'static,
      |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
13275 -         C: Clone + Serialize + serde::de::DeserializeOwned + ArtifactPack + Send + Sync + 'static,
13275 +         C: Clone + Serialize + DeserializeOwned + ArtifactPack + Send + Sync + 'static,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:13276:32
      |
13276 |         M: Clone + Serialize + serde::de::DeserializeOwned + store::Mutation<C> + OpBinary + OpText + Send + 'static,
      |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
13276 -         M: Clone + Serialize + serde::de::DeserializeOwned + store::Mutation<C> + OpBinary + OpText + Send + 'static,
13276 +         M: Clone + Serialize + DeserializeOwned + store::Mutation<C> + OpBinary + OpText + Send + 'static,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:13276:62
      |
13276 |         M: Clone + Serialize + serde::de::DeserializeOwned + store::Mutation<C> + OpBinary + OpText + Send + 'static,
      |                                                              ^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
13276 -         M: Clone + Serialize + serde::de::DeserializeOwned + store::Mutation<C> + OpBinary + OpText + Send + 'static,
13276 +         M: Clone + Serialize + serde::de::DeserializeOwned + Mutation<C> + OpBinary + OpText + Send + 'static,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:13289:32
      |
13289 |         C: Clone + Serialize + serde::de::DeserializeOwned + ArtifactPack + Send + Sync + 'static,
      |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
13289 -         C: Clone + Serialize + serde::de::DeserializeOwned + ArtifactPack + Send + Sync + 'static,
13289 +         C: Clone + Serialize + DeserializeOwned + ArtifactPack + Send + Sync + 'static,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:13290:32
      |
13290 |         M: Clone + Serialize + serde::de::DeserializeOwned + store::Mutation<C> + OpBinary + OpText + Send + 'static,
      |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
13290 -         M: Clone + Serialize + serde::de::DeserializeOwned + store::Mutation<C> + OpBinary + OpText + Send + 'static,
13290 +         M: Clone + Serialize + DeserializeOwned + store::Mutation<C> + OpBinary + OpText + Send + 'static,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:13290:62
      |
13290 |         M: Clone + Serialize + serde::de::DeserializeOwned + store::Mutation<C> + OpBinary + OpText + Send + 'static,
      |                                                              ^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
13290 -         M: Clone + Serialize + serde::de::DeserializeOwned + store::Mutation<C> + OpBinary + OpText + Send + 'static,
13290 +         M: Clone + Serialize + serde::de::DeserializeOwned + Mutation<C> + OpBinary + OpText + Send + 'static,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:13298:32
      |
13298 |         P: Clone + Serialize + serde::de::DeserializeOwned + ArtifactPack + Send + Sync + 'static,
      |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
13298 -         P: Clone + Serialize + serde::de::DeserializeOwned + ArtifactPack + Send + Sync + 'static,
13298 +         P: Clone + Serialize + DeserializeOwned + ArtifactPack + Send + Sync + 'static,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:13299:32
      |
13299 |         M: Clone + Serialize + serde::de::DeserializeOwned + store::Mutation<P> + OpBinary + OpText + Send + 'static,
      |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
13299 -         M: Clone + Serialize + serde::de::DeserializeOwned + store::Mutation<P> + OpBinary + OpText + Send + 'static,
13299 +         M: Clone + Serialize + DeserializeOwned + store::Mutation<P> + OpBinary + OpText + Send + 'static,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:13299:62
      |
13299 |         M: Clone + Serialize + serde::de::DeserializeOwned + store::Mutation<P> + OpBinary + OpText + Send + 'static,
      |                                                              ^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
13299 -         M: Clone + Serialize + serde::de::DeserializeOwned + store::Mutation<P> + OpBinary + OpText + Send + 'static,
13299 +         M: Clone + Serialize + serde::de::DeserializeOwned + Mutation<P> + OpBinary + OpText + Send + 'static,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:13307:32
      |
13307 |         P: Clone + Serialize + serde::de::DeserializeOwned + ArtifactPack + Send + Sync + 'static,
      |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
13307 -         P: Clone + Serialize + serde::de::DeserializeOwned + ArtifactPack + Send + Sync + 'static,
13307 +         P: Clone + Serialize + DeserializeOwned + ArtifactPack + Send + Sync + 'static,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:13308:32
      |
13308 |         M: Clone + Serialize + serde::de::DeserializeOwned + store::Mutation<P> + OpBinary + OpText + Send + 'static,
      |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
13308 -         M: Clone + Serialize + serde::de::DeserializeOwned + store::Mutation<P> + OpBinary + OpText + Send + 'static,
13308 +         M: Clone + Serialize + DeserializeOwned + store::Mutation<P> + OpBinary + OpText + Send + 'static,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:13308:62
      |
13308 |         M: Clone + Serialize + serde::de::DeserializeOwned + store::Mutation<P> + OpBinary + OpText + Send + 'static,
      |                                                              ^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
13308 -         M: Clone + Serialize + serde::de::DeserializeOwned + store::Mutation<P> + OpBinary + OpText + Send + 'static,
13308 +         M: Clone + Serialize + serde::de::DeserializeOwned + Mutation<P> + OpBinary + OpText + Send + 'static,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:13327:32
      |
13327 |         P: Clone + Serialize + serde::de::DeserializeOwned + ArtifactPack + Send + Sync + 'static,
      |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
13327 -         P: Clone + Serialize + serde::de::DeserializeOwned + ArtifactPack + Send + Sync + 'static,
13327 +         P: Clone + Serialize + DeserializeOwned + ArtifactPack + Send + Sync + 'static,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:13328:39
      |
13328 |         Mutation: Clone + Serialize + serde::de::DeserializeOwned + store::Mutation<P> + OpBinary + OpText + Send + 'static,
      |                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
13328 -         Mutation: Clone + Serialize + serde::de::DeserializeOwned + store::Mutation<P> + OpBinary + OpText + Send + 'static,
13328 +         Mutation: Clone + Serialize + DeserializeOwned + store::Mutation<P> + OpBinary + OpText + Send + 'static,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:13331:19
      |
13331 |             match store::SpaceMember::close_owned_step(owner, maximum_items.min(1), maximum_bytes).map_err(plugin_sdk_fault)? {
      |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
13331 -             match store::SpaceMember::close_owned_step(owner, maximum_items.min(1), maximum_bytes).map_err(plugin_sdk_fault)? {
13331 +             match SpaceMember::close_owned_step(owner, maximum_items.min(1), maximum_bytes).map_err(plugin_sdk_fault)? {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:13340:13
      |
13340 |             store::SpaceMember::close_owned_terminal_is_empty(owner)
      |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
13340 -             store::SpaceMember::close_owned_terminal_is_empty(owner)
13340 +             SpaceMember::close_owned_terminal_is_empty(owner)
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:13349:32
      |
13349 |         P: Clone + Serialize + serde::de::DeserializeOwned,
      |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
13349 -         P: Clone + Serialize + serde::de::DeserializeOwned,
13349 +         P: Clone + Serialize + DeserializeOwned,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:13350:39
      |
13350 |         Mutation: Clone + Serialize + serde::de::DeserializeOwned + store::Mutation<P>,
      |                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
13350 -         Mutation: Clone + Serialize + serde::de::DeserializeOwned + store::Mutation<P>,
13350 +         Mutation: Clone + Serialize + DeserializeOwned + store::Mutation<P>,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:13362:32
      |
13362 |         P: Clone + Serialize + serde::de::DeserializeOwned,
      |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
13362 -         P: Clone + Serialize + serde::de::DeserializeOwned,
13362 +         P: Clone + Serialize + DeserializeOwned,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:13363:39
      |
13363 |         Mutation: Clone + Serialize + serde::de::DeserializeOwned + store::Mutation<P>,
      |                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
13363 -         Mutation: Clone + Serialize + serde::de::DeserializeOwned + store::Mutation<P>,
13363 +         Mutation: Clone + Serialize + DeserializeOwned + store::Mutation<P>,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:13372:32
      |
13372 |         P: Clone + Serialize + serde::de::DeserializeOwned,
      |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
13372 -         P: Clone + Serialize + serde::de::DeserializeOwned,
13372 +         P: Clone + Serialize + DeserializeOwned,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:13373:39
      |
13373 |         Mutation: Clone + Serialize + serde::de::DeserializeOwned + store::Mutation<P>,
      |                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
13373 -         Mutation: Clone + Serialize + serde::de::DeserializeOwned + store::Mutation<P>,
13373 +         Mutation: Clone + Serialize + DeserializeOwned + store::Mutation<P>,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:13412:32
      |
13412 |         P: Clone + Serialize + serde::de::DeserializeOwned + Send,
      |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
13412 -         P: Clone + Serialize + serde::de::DeserializeOwned + Send,
13412 +         P: Clone + Serialize + DeserializeOwned + Send,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:13413:39
      |
13413 |         Mutation: Clone + Serialize + serde::de::DeserializeOwned + store::Mutation<P> + Send,
      |                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
13413 -         Mutation: Clone + Serialize + serde::de::DeserializeOwned + store::Mutation<P> + Send,
13413 +         Mutation: Clone + Serialize + DeserializeOwned + store::Mutation<P> + Send,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:13465:32
      |
13465 |         P: Clone + Serialize + serde::de::DeserializeOwned,
      |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
13465 -         P: Clone + Serialize + serde::de::DeserializeOwned,
13465 +         P: Clone + Serialize + DeserializeOwned,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:13466:39
      |
13466 |         Mutation: Clone + Serialize + serde::de::DeserializeOwned + store::Mutation<P>,
      |                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
13466 -         Mutation: Clone + Serialize + serde::de::DeserializeOwned + store::Mutation<P>,
13466 +         Mutation: Clone + Serialize + DeserializeOwned + store::Mutation<P>,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:13765:33
      |
13765 |             struct DropSentinel(std::sync::Arc<std::sync::atomic::AtomicUsize>);
      |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
13765 -             struct DropSentinel(std::sync::Arc<std::sync::atomic::AtomicUsize>);
13765 +             struct DropSentinel(Arc<std::sync::atomic::AtomicUsize>);
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:13771:25
      |
13771 |             let drops = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
      |                         ^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
13771 -             let drops = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
13771 +             let drops = Arc::new(std::sync::atomic::AtomicUsize::new(0));
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:13800:29
      |
13800 |             struct DropItem(std::sync::Arc<std::sync::atomic::AtomicUsize>);
      |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
13800 -             struct DropItem(std::sync::Arc<std::sync::atomic::AtomicUsize>);
13800 +             struct DropItem(Arc<std::sync::atomic::AtomicUsize>);
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:13808:64
      |
13808 | ...   fn close_step(&mut self, snapshot: &mut Option<std::sync::Arc<Vec<DropItem>>>, maximum_items: usize, _maximum_bytes: usize)...
      |                                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
13808 -                 fn close_step(&mut self, snapshot: &mut Option<std::sync::Arc<Vec<DropItem>>>, maximum_items: usize, _maximum_bytes: usize) -> Result<PluginCloseStep, Fault> {
13808 +                 fn close_step(&mut self, snapshot: &mut Option<Arc<Vec<DropItem>>>, maximum_items: usize, _maximum_bytes: usize) -> Result<PluginCloseStep, Fault> {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:13813:39
      |
13813 | ...   let Some(items) = std::sync::Arc::get_mut(owner) else { return Ok(PluginCloseStep::Blocked { reason: "snapshot remains exte...
      |                         ^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
13813 -                     let Some(items) = std::sync::Arc::get_mut(owner) else { return Ok(PluginCloseStep::Blocked { reason: "snapshot remains externally owned" }) };
13813 +                     let Some(items) = Arc::get_mut(owner) else { return Ok(PluginCloseStep::Blocked { reason: "snapshot remains externally owned" }) };
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:13821:63
      |
13821 |                 fn terminal_is_empty(&self, snapshot: &Option<std::sync::Arc<Vec<DropItem>>>) -> bool {
      |                                                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
13821 -                 fn terminal_is_empty(&self, snapshot: &Option<std::sync::Arc<Vec<DropItem>>>) -> bool {
13821 +                 fn terminal_is_empty(&self, snapshot: &Option<Arc<Vec<DropItem>>>) -> bool {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:13826:25
      |
13826 |             let drops = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
      |                         ^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
13826 -             let drops = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
13826 +             let drops = Arc::new(std::sync::atomic::AtomicUsize::new(0));
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:13827:27
      |
13827 |             let cache_a = std::sync::Arc::new(vec![DropItem(drops.clone()), DropItem(drops.clone())]);
      |                           ^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
13827 -             let cache_a = std::sync::Arc::new(vec![DropItem(drops.clone()), DropItem(drops.clone())]);
13827 +             let cache_a = Arc::new(vec![DropItem(drops.clone()), DropItem(drops.clone())]);
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:13830:27
      |
13830 |             let cache_b = std::sync::Arc::new(Vec::<DropItem>::new());
      |                           ^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
13830 -             let cache_b = std::sync::Arc::new(Vec::<DropItem>::new());
13830 +             let cache_b = Arc::new(Vec::<DropItem>::new());
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:13841:21
      |
13841 |             assert!(std::sync::Arc::strong_count(&cache_b) == 1, "cache B is independent of retired A authority");
      |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
13841 -             assert!(std::sync::Arc::strong_count(&cache_b) == 1, "cache B is independent of retired A authority");
13841 +             assert!(Arc::strong_count(&cache_b) == 1, "cache B is independent of retired A authority");
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:15434:42
      |
15434 |         struct ActiveMediaExportSentinel(std::sync::Arc<std::sync::atomic::AtomicUsize>);
      |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
15434 -         struct ActiveMediaExportSentinel(std::sync::Arc<std::sync::atomic::AtomicUsize>);
15434 +         struct ActiveMediaExportSentinel(Arc<std::sync::atomic::AtomicUsize>);
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:15436:42
      |
15436 |         struct SnapshotRetentionSentinel(std::sync::Arc<std::sync::atomic::AtomicUsize>);
      |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
15436 -         struct SnapshotRetentionSentinel(std::sync::Arc<std::sync::atomic::AtomicUsize>);
15436 +         struct SnapshotRetentionSentinel(Arc<std::sync::atomic::AtomicUsize>);
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:15438:42
      |
15438 |         struct SegmentedDownloadSentinel(std::sync::Arc<std::sync::atomic::AtomicUsize>);
      |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
15438 -         struct SegmentedDownloadSentinel(std::sync::Arc<std::sync::atomic::AtomicUsize>);
15438 +         struct SegmentedDownloadSentinel(Arc<std::sync::atomic::AtomicUsize>);
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:15454:81
      |
15454 |         fn reject_duplicate<T: std::fmt::Debug>(first: T, duplicate: T, drops: &std::sync::Arc<std::sync::atomic::AtomicUsize>) {
      |                                                                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
15454 -         fn reject_duplicate<T: std::fmt::Debug>(first: T, duplicate: T, drops: &std::sync::Arc<std::sync::atomic::AtomicUsize>) {
15454 +         fn reject_duplicate<T: std::fmt::Debug>(first: T, duplicate: T, drops: &Arc<std::sync::atomic::AtomicUsize>) {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:15473:31
      |
15473 |             let media_drops = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
      |                               ^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
15473 -             let media_drops = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
15473 +             let media_drops = Arc::new(std::sync::atomic::AtomicUsize::new(0));
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:15475:34
      |
15475 |             let snapshot_drops = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
      |                                  ^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
15475 -             let snapshot_drops = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
15475 +             let snapshot_drops = Arc::new(std::sync::atomic::AtomicUsize::new(0));
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:15477:34
      |
15477 |             let download_drops = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
      |                                  ^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
15477 -             let download_drops = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
15477 +             let download_drops = Arc::new(std::sync::atomic::AtomicUsize::new(0));
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:15485:24
      |
15485 |                 drops: std::sync::Arc<std::sync::atomic::AtomicUsize>,
      |                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
15485 -                 drops: std::sync::Arc<std::sync::atomic::AtomicUsize>,
15485 +                 drops: Arc<std::sync::atomic::AtomicUsize>,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:15492:25
      |
15492 |             let drops = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
      |                         ^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
15492 -             let drops = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
15492 +             let drops = Arc::new(std::sync::atomic::AtomicUsize::new(0));
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:15512:27
      |
15512 |                 identity: std::sync::Arc<()>,
      |                           ^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
15512 -                 identity: std::sync::Arc<()>,
15512 +                 identity: Arc<()>,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:15513:24
      |
15513 |                 drops: std::sync::Arc<std::sync::atomic::AtomicUsize>,
      |                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
15513 -                 drops: std::sync::Arc<std::sync::atomic::AtomicUsize>,
15513 +                 drops: Arc<std::sync::atomic::AtomicUsize>,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:15520:25
      |
15520 |             let drops = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
      |                         ^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
15520 -             let drops = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
15520 +             let drops = Arc::new(std::sync::atomic::AtomicUsize::new(0));
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:15521:34
      |
15521 |             let first_identity = std::sync::Arc::new(());
      |                                  ^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
15521 -             let first_identity = std::sync::Arc::new(());
15521 +             let first_identity = Arc::new(());
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:15522:35
      |
15522 |             let second_identity = std::sync::Arc::new(());
      |                                   ^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
15522 -             let second_identity = std::sync::Arc::new(());
15522 +             let second_identity = Arc::new(());
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:15528:21
      |
15528 |             assert!(std::sync::Arc::ptr_eq(&first.identity, &first_identity));
      |                     ^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
15528 -             assert!(std::sync::Arc::ptr_eq(&first.identity, &first_identity));
15528 +             assert!(Arc::ptr_eq(&first.identity, &first_identity));
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:15529:21
      |
15529 |             assert!(std::sync::Arc::ptr_eq(&live.get(42).expect("unrelated media owner remains live").identity, &second_identity));
      |                     ^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
15529 -             assert!(std::sync::Arc::ptr_eq(&live.get(42).expect("unrelated media owner remains live").identity, &second_identity));
15529 +             assert!(Arc::ptr_eq(&live.get(42).expect("unrelated media owner remains live").identity, &second_identity));
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:15583:20
      |
15583 |             drops: std::sync::Arc<std::sync::atomic::AtomicUsize>,
      |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
15583 -             drops: std::sync::Arc<std::sync::atomic::AtomicUsize>,
15583 +             drops: Arc<std::sync::atomic::AtomicUsize>,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:15622:25
      |
15622 |             let drops = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
      |                         ^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
15622 -             let drops = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
15622 +             let drops = Arc::new(std::sync::atomic::AtomicUsize::new(0));
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:16941:43
      |
16941 |         struct SerdeFixtureOracle<'a>(&'a serde_json::Value);
      |                                           ^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
16941 -         struct SerdeFixtureOracle<'a>(&'a serde_json::Value);
16941 +         struct SerdeFixtureOracle<'a>(&'a Value);
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:16993:95
      |
16993 |                     self.prepared = Some(store::ArtifactEphemeralOneItemPrepared { next_root: std::sync::Arc::new(next_root) });
      |                                                                                               ^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
16993 -                     self.prepared = Some(store::ArtifactEphemeralOneItemPrepared { next_root: std::sync::Arc::new(next_root) });
16993 +                     self.prepared = Some(store::ArtifactEphemeralOneItemPrepared { next_root: Arc::new(next_root) });
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:17033:26
      |
17033 |             root: Option<std::sync::Arc<PublicationPresence>>,
      |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
17033 -             root: Option<std::sync::Arc<PublicationPresence>>,
17033 +             root: Option<Arc<PublicationPresence>>,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:17055:40
      |
17055 |             fn retire(&self, snapshot: std::sync::Arc<PublicationPresence>) -> Box<dyn store::ErasedSnapshotRetirement> {
      |                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
17055 -             fn retire(&self, snapshot: std::sync::Arc<PublicationPresence>) -> Box<dyn store::ErasedSnapshotRetirement> {
17055 +             fn retire(&self, snapshot: Arc<PublicationPresence>) -> Box<dyn store::ErasedSnapshotRetirement> {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:17144:30
      |
17144 | ...   let factory: std::sync::Arc<dyn store::SnapshotRetirementFactory<PublicationPresence>> = std::sync::Arc::new(PublicationPre...
      |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
17144 -                 let factory: std::sync::Arc<dyn store::SnapshotRetirementFactory<PublicationPresence>> = std::sync::Arc::new(PublicationPresenceLocalRootRetirementFactory);
17144 +                 let factory: Arc<dyn store::SnapshotRetirementFactory<PublicationPresence>> = std::sync::Arc::new(PublicationPresenceLocalRootRetirementFactory);
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:17144:106
      |
17144 | ...   let factory: std::sync::Arc<dyn store::SnapshotRetirementFactory<PublicationPresence>> = std::sync::Arc::new(PublicationPre...
      |                                                                                                ^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
17144 -                 let factory: std::sync::Arc<dyn store::SnapshotRetirementFactory<PublicationPresence>> = std::sync::Arc::new(PublicationPresenceLocalRootRetirementFactory);
17144 +                 let factory: std::sync::Arc<dyn store::SnapshotRetirementFactory<PublicationPresence>> = Arc::new(PublicationPresenceLocalRootRetirementFactory);
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:17172:59
      |
17172 | ...   let mut close = presence.begin_retirement(std::sync::Arc::new(PublicationPresence::default()), |_| true).ok().unwrap();
      |                                                 ^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
17172 -                 let mut close = presence.begin_retirement(std::sync::Arc::new(PublicationPresence::default()), |_| true).ok().unwrap();
17172 +                 let mut close = presence.begin_retirement(Arc::new(PublicationPresence::default()), |_| true).ok().unwrap();
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:17178:54
      |
17178 |         fn fixture_latest_wins_key(scope: &Value) -> std::sync::Arc<String> {
      |                                                      ^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
17178 -         fn fixture_latest_wins_key(scope: &Value) -> std::sync::Arc<String> {
17178 +         fn fixture_latest_wins_key(scope: &Value) -> Arc<String> {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:17247:167
      |
17247 | ...pp.store.generation_now() - generation, "sameRoot": std::sync::Arc::ptr_eq(&before, &app.store.snapshot_root()) });
      |                                                        ^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
17247 -                     let actual = serde_json::json!({ "count": observe(&app.store.snapshot_root()), "generation": app.store.generation_now() - generation, "sameRoot": std::sync::Arc::ptr_eq(&before, &app.store.snapshot_root()) });
17247 +                     let actual = serde_json::json!({ "count": observe(&app.store.snapshot_root()), "generation": app.store.generation_now() - generation, "sameRoot": Arc::ptr_eq(&before, &app.store.snapshot_root()) });
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:17402:27
      |
17402 |                 let key = std::sync::Arc::new(format!("target-{index:04}"));
      |                           ^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
17402 -                 let key = std::sync::Arc::new(format!("target-{index:04}"));
17402 +                 let key = Arc::new(format!("target-{index:04}"));
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:17495:26
      |
17495 |             let fixture: serde_json::Value = serde_json::from_str(FIXTURE).expect("language-neutral typed-command fixture");
      |                          ^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
17495 -             let fixture: serde_json::Value = serde_json::from_str(FIXTURE).expect("language-neutral typed-command fixture");
17495 +             let fixture: Value = serde_json::from_str(FIXTURE).expect("language-neutral typed-command fixture");
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:17510:26
      |
17510 |             let fixture: serde_json::Value = serde_json::from_str(FIXTURE).expect("language-neutral typed-command fixture");
      |                          ^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
17510 -             let fixture: serde_json::Value = serde_json::from_str(FIXTURE).expect("language-neutral typed-command fixture");
17510 +             let fixture: Value = serde_json::from_str(FIXTURE).expect("language-neutral typed-command fixture");
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:17532:87
      |
17532 |                 let phases = law["phases"].as_array().cloned().unwrap_or_else(|| vec![serde_json::Value::Null]);
      |                                                                                       ^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
17532 -                 let phases = law["phases"].as_array().cloned().unwrap_or_else(|| vec![serde_json::Value::Null]);
17532 +                 let phases = law["phases"].as_array().cloned().unwrap_or_else(|| vec![Value::Null]);
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:17634:31
      |
17634 | ...   let root_factory: std::sync::Arc<dyn store::SnapshotRetirementFactory<PublicationPresence>> = std::sync::Arc::new(Publicati...
      |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
17634 -             let root_factory: std::sync::Arc<dyn store::SnapshotRetirementFactory<PublicationPresence>> = std::sync::Arc::new(PublicationPresenceLocalRootRetirementFactory);
17634 +             let root_factory: Arc<dyn store::SnapshotRetirementFactory<PublicationPresence>> = std::sync::Arc::new(PublicationPresenceLocalRootRetirementFactory);
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:17634:107
      |
17634 | ...   let root_factory: std::sync::Arc<dyn store::SnapshotRetirementFactory<PublicationPresence>> = std::sync::Arc::new(Publicati...
      |                                                                                                     ^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
17634 -             let root_factory: std::sync::Arc<dyn store::SnapshotRetirementFactory<PublicationPresence>> = std::sync::Arc::new(PublicationPresenceLocalRootRetirementFactory);
17634 +             let root_factory: std::sync::Arc<dyn store::SnapshotRetirementFactory<PublicationPresence>> = Arc::new(PublicationPresenceLocalRootRetirementFactory);
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:17656:55
      |
17656 |             let mut close = presence.begin_retirement(std::sync::Arc::new(PublicationPresence::default()), |_| true).ok().unwrap();
      |                                                       ^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
17656 -             let mut close = presence.begin_retirement(std::sync::Arc::new(PublicationPresence::default()), |_| true).ok().unwrap();
17656 +             let mut close = presence.begin_retirement(Arc::new(PublicationPresence::default()), |_| true).ok().unwrap();
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:17663:26
      |
17663 | ...   let fixture: serde_json::Value = serde_json::from_str(include_str!("../🏪️store/🧪️member-publication.json")).expect("retaine...
      |                    ^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
17663 -             let fixture: serde_json::Value = serde_json::from_str(include_str!("../🏪️store/🧪️member-publication.json")).expect("retained child fixture");
17663 +             let fixture: Value = serde_json::from_str(include_str!("../🏪️store/🧪️member-publication.json")).expect("retained child fixture");
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:18124:32
      |
18124 |         P: Clone + Serialize + serde::de::DeserializeOwned + ArtifactPack + Send + Sync + 'static,
      |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
18124 -         P: Clone + Serialize + serde::de::DeserializeOwned + ArtifactPack + Send + Sync + 'static,
18124 +         P: Clone + Serialize + DeserializeOwned + ArtifactPack + Send + Sync + 'static,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:18125:39
      |
18125 |         Mutation: Clone + Serialize + serde::de::DeserializeOwned + store::Mutation<P> + OpBinary + OpText + Send + 'static,
      |                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
18125 -         Mutation: Clone + Serialize + serde::de::DeserializeOwned + store::Mutation<P> + OpBinary + OpText + Send + 'static,
18125 +         Mutation: Clone + Serialize + DeserializeOwned + store::Mutation<P> + OpBinary + OpText + Send + 'static,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:18144:32
      |
18144 |         P: Clone + Serialize + serde::de::DeserializeOwned + ArtifactPack + Send + Sync + 'static,
      |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
18144 -         P: Clone + Serialize + serde::de::DeserializeOwned + ArtifactPack + Send + Sync + 'static,
18144 +         P: Clone + Serialize + DeserializeOwned + ArtifactPack + Send + Sync + 'static,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:18145:39
      |
18145 |         Mutation: Clone + Serialize + serde::de::DeserializeOwned + store::Mutation<P> + OpBinary + OpText + Send + 'static,
      |                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
18145 -         Mutation: Clone + Serialize + serde::de::DeserializeOwned + store::Mutation<P> + OpBinary + OpText + Send + 'static,
18145 +         Mutation: Clone + Serialize + DeserializeOwned + store::Mutation<P> + OpBinary + OpText + Send + 'static,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:18164:32
      |
18164 |         P: Clone + Serialize + serde::de::DeserializeOwned + ArtifactPack + Send + Sync + 'static,
      |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
18164 -         P: Clone + Serialize + serde::de::DeserializeOwned + ArtifactPack + Send + Sync + 'static,
18164 +         P: Clone + Serialize + DeserializeOwned + ArtifactPack + Send + Sync + 'static,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:18165:39
      |
18165 |         Mutation: Clone + Serialize + serde::de::DeserializeOwned + store::Mutation<P> + OpBinary + OpText + Send + 'static,
      |                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
18165 -         Mutation: Clone + Serialize + serde::de::DeserializeOwned + store::Mutation<P> + OpBinary + OpText + Send + 'static,
18165 +         Mutation: Clone + Serialize + DeserializeOwned + store::Mutation<P> + OpBinary + OpText + Send + 'static,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:18323:32
      |
18323 |         P: Clone + Serialize + serde::de::DeserializeOwned + ArtifactPack + Send + Sync + 'static,
      |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
18323 -         P: Clone + Serialize + serde::de::DeserializeOwned + ArtifactPack + Send + Sync + 'static,
18323 +         P: Clone + Serialize + DeserializeOwned + ArtifactPack + Send + Sync + 'static,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:18324:39
      |
18324 |         Mutation: Clone + Serialize + serde::de::DeserializeOwned + store::Mutation<P> + OpBinary + OpText + Send + 'static,
      |                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
18324 -         Mutation: Clone + Serialize + serde::de::DeserializeOwned + store::Mutation<P> + OpBinary + OpText + Send + 'static,
18324 +         Mutation: Clone + Serialize + DeserializeOwned + store::Mutation<P> + OpBinary + OpText + Send + 'static,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:20411:46
      |
20411 |             if artifact_mutations.iter().any(protocol::Mutation::may_emit_foreign_steps) {
      |                                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
20411 -             if artifact_mutations.iter().any(protocol::Mutation::may_emit_foreign_steps) {
20411 +             if artifact_mutations.iter().any(Mutation::may_emit_foreign_steps) {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:22163:29
      |
22163 | ...                   store::HistoryLane::Document,
      |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
22163 -                             store::HistoryLane::Document,
22163 +                             HistoryLane::Document,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:22179:175
      |
22179 | ...ion, mounted.meta.actor.clone(), mutation, None, store::HistoryLane::Document, self.config_one_item_factory.as_deref()) {
      |                                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
22179 -                         match self.config_store.begin_apply_one(mounted.operation.operation, mounted.config_generation, revision, mounted.meta.actor.clone(), mutation, None, store::HistoryLane::Document, self.config_one_item_factory.as_deref()) {
22179 +                         match self.config_store.begin_apply_one(mounted.operation.operation, mounted.config_generation, revision, mounted.meta.actor.clone(), mutation, None, HistoryLane::Document, self.config_one_item_factory.as_deref()) {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:22192:173
      |
22192 | ...ion, mounted.meta.actor.clone(), mutation, None, store::HistoryLane::Document, self.draft_one_item_factory.as_deref()) {
      |                                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
22192 -                         match self.draft_store.begin_apply_one(mounted.operation.operation, mounted.draft_generation, revision, mounted.meta.actor.clone(), mutation, None, store::HistoryLane::Document, self.draft_one_item_factory.as_deref()) {
22192 +                         match self.draft_store.begin_apply_one(mounted.operation.operation, mounted.draft_generation, revision, mounted.meta.actor.clone(), mutation, None, HistoryLane::Document, self.draft_one_item_factory.as_deref()) {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:24676:31
      |
24676 |         apps: HashMap<String, crate::app::declarations::AppFactory<PA>>,
      |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
24676 -         apps: HashMap<String, crate::app::declarations::AppFactory<PA>>,
24676 +         apps: HashMap<String, declarations::AppFactory<PA>>,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:24818:70
      |
24818 |         pub fn register_app_factory(mut self, mut app: App, factory: crate::app::declarations::AppFactory<PA>) -> Self {
      |                                                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
24818 -         pub fn register_app_factory(mut self, mut app: App, factory: crate::app::declarations::AppFactory<PA>) -> Self {
24818 +         pub fn register_app_factory(mut self, mut app: App, factory: declarations::AppFactory<PA>) -> Self {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:24941:27
      |
24941 |             let builder = ui::surface(props);
      |                           ^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
24941 -             let builder = ui::surface(props);
24941 +             let builder = surface(props);
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:24978:27
      |
24978 |             let builder = ui::surface(props);
      |                           ^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
24978 -             let builder = ui::surface(props);
24978 +             let builder = surface(props);
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:25223:28
      |
25223 |             let mut root = ui::column().try_id(Self::KIND_ID).map_err(|_| ui_assembly_error("table-rows-window.id"))?;
      |                            ^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
25223 -             let mut root = ui::column().try_id(Self::KIND_ID).map_err(|_| ui_assembly_error("table-rows-window.id"))?;
25223 +             let mut root = column().try_id(Self::KIND_ID).map_err(|_| ui_assembly_error("table-rows-window.id"))?;
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:25224:30
      |
25224 |             let mut header = ui::row().try_id("header").map_err(|_| ui_assembly_error("table-rows-window.header-id"))?;
      |                              ^^^^^^^
      |
help: remove the unnecessary path segments
      |
25224 -             let mut header = ui::row().try_id("header").map_err(|_| ui_assembly_error("table-rows-window.header-id"))?;
25224 +             let mut header = row().try_id("header").map_err(|_| ui_assembly_error("table-rows-window.header-id"))?;
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:25226:43
      |
25226 |                 header = header.try_child(ui::text(Label(column))).map_err(|_| ui_assembly_error("table-rows-window.header"))?;
      |                                           ^^^^^^^^
      |
help: remove the unnecessary path segments
      |
25226 -                 header = header.try_child(ui::text(Label(column))).map_err(|_| ui_assembly_error("table-rows-window.header"))?;
25226 +                 header = header.try_child(text(Label(column))).map_err(|_| ui_assembly_error("table-rows-window.header"))?;
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:25229:43
      |
25229 | ...   header = header.try_child(ui::text(Label(actions_label))).map_err(|_| ui_assembly_error("table-rows-window.actions-header"))?;
      |                                 ^^^^^^^^
      |
help: remove the unnecessary path segments
      |
25229 -                 header = header.try_child(ui::text(Label(actions_label))).map_err(|_| ui_assembly_error("table-rows-window.actions-header"))?;
25229 +                 header = header.try_child(text(Label(actions_label))).map_err(|_| ui_assembly_error("table-rows-window.actions-header"))?;
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:25234:31
      |
25234 |                 let mut row = ui::row().try_id(id.as_str()).map_err(|_| ui_assembly_error("table-rows-window.row-id"))?;
      |                               ^^^^^^^
      |
help: remove the unnecessary path segments
      |
25234 -                 let mut row = ui::row().try_id(id.as_str()).map_err(|_| ui_assembly_error("table-rows-window.row-id"))?;
25234 +                 let mut row = row().try_id(id.as_str()).map_err(|_| ui_assembly_error("table-rows-window.row-id"))?;
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:25236:41
      |
25236 |                     row = row.try_child(ui::text(Label(cell))).map_err(|_| ui_assembly_error("table-rows-window.cell"))?;
      |                                         ^^^^^^^^
      |
help: remove the unnecessary path segments
      |
25236 -                     row = row.try_child(ui::text(Label(cell))).map_err(|_| ui_assembly_error("table-rows-window.cell"))?;
25236 +                     row = row.try_child(text(Label(cell))).map_err(|_| ui_assembly_error("table-rows-window.cell"))?;
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:25241:34
      |
25241 |                     let button = ui::button(label).icon(icon);
      |                                  ^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
25241 -                     let button = ui::button(label).icon(icon);
25241 +                     let button = button(label).icon(icon);
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:25310:35
      |
25310 |             let section_builder = ui::tree_section(Label::default()).default_open(true);
      |                                   ^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
25310 -             let section_builder = ui::tree_section(Label::default()).default_open(true);
25310 +             let section_builder = tree_section(Label::default()).default_open(true);
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:25313:32
      |
25313 |             let tree_builder = ui::tree().try_id(Self::KIND_ID).map_err(|_| ui_assembly_error("tree-window.id"))?;
      |                                ^^^^^^^^
      |
help: remove the unnecessary path segments
      |
25313 -             let tree_builder = ui::tree().try_id(Self::KIND_ID).map_err(|_| ui_assembly_error("tree-window.id"))?;
25313 +             let tree_builder = tree().try_id(Self::KIND_ID).map_err(|_| ui_assembly_error("tree-window.id"))?;
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:25347:27
      |
25347 |             let builder = ui::image(src).alt(alt);
      |                           ^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
25347 -             let builder = ui::image(src).alt(alt);
25347 +             let builder = image(src).alt(alt);
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:25382:27
      |
25382 |             let builder = ui::surface(props);
      |                           ^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
25382 -             let builder = ui::surface(props);
25382 +             let builder = surface(props);
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:25418:31
      |
25418 |                 let builder = ui::text(ui_label(page.text.clone(), "document-window.page-text")?);
      |                               ^^^^^^^^
      |
help: remove the unnecessary path segments
      |
25418 -                 let builder = ui::text(ui_label(page.text.clone(), "document-window.page-text")?);
25418 +                 let builder = text(ui_label(page.text.clone(), "document-window.page-text")?);
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:25422:27
      |
25422 |             let builder = ui::column().try_id(Self::KIND_ID).map_err(|_| ui_assembly_error("document-window.id"))?;
      |                           ^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
25422 -             let builder = ui::column().try_id(Self::KIND_ID).map_err(|_| ui_assembly_error("document-window.id"))?;
25422 +             let builder = column().try_id(Self::KIND_ID).map_err(|_| ui_assembly_error("document-window.id"))?;
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:25592:64
      |
25592 | ...   assert_eq!(props, semio_framework_ui_scene::encode(semio_framework_ui_contract::SurfaceKind::TextEditor, &expected).expect(...
      |                                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
25592 -             assert_eq!(props, semio_framework_ui_scene::encode(semio_framework_ui_contract::SurfaceKind::TextEditor, &expected).expect("bounded fixture"));
25592 +             assert_eq!(props, semio_framework_ui_scene::encode(SurfaceKind::TextEditor, &expected).expect("bounded fixture"));
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:25618:64
      |
25618 | ...   assert_eq!(props, semio_framework_ui_scene::encode(semio_framework_ui_contract::SurfaceKind::TextEditor, &expected).expect(...
      |                                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
25618 -             assert_eq!(props, semio_framework_ui_scene::encode(semio_framework_ui_contract::SurfaceKind::TextEditor, &expected).expect("bounded fixture"));
25618 +             assert_eq!(props, semio_framework_ui_scene::encode(SurfaceKind::TextEditor, &expected).expect("bounded fixture"));
      |

warning: unused imports: `Component`, `Label`, `TextProps`, and `TreeNode`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:27270:134
      |
27270 | ...finition, ArtifactDialect, ArtifactEditor, ArtifactKindId, ArtifactPack, ArtifactView, ArtifactViewer, Component, ComponentTree, ConfigView, Dialect, DraftView, Editor, Emit, EngineHandles, Fault, Icon...
      |                                                                                                           ^^^^^^^^^
27271 | ...w, Label, LocalizedLabel, Mutation, MutationDiff, NoConfig, NoConfigMutation, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation, Plugin, StandardId, SubsetId, SurfaceKind, TextProps, TreeNode, V...
      |       ^^^^^                                                                                                                                                                           ^^^^^^^^^  ^^^^^^^^

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:27497:100
      |
27497 |                 M: Mutation<S> + PartialEq + Serialize + DeserializeOwned + Send + Sync + OpText + protocol::OpBinary + 'static,
      |                                                                                                    ^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
27497 -                 M: Mutation<S> + PartialEq + Serialize + DeserializeOwned + Send + Sync + OpText + protocol::OpBinary + 'static,
27497 +                 M: Mutation<S> + PartialEq + Serialize + DeserializeOwned + Send + Sync + OpText + OpBinary + 'static,
      |

warning: unused import: `crate::__semio_dispatch_PluginApp`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:27531:17
      |
27531 |             use crate::__semio_dispatch_PluginApp;
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
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:27979:26
      |
27979 | ...   let fixture: serde_json::Value = serde_json::from_str(include_str!("⚛️reactor/🧪️fixtures/📬️operation-continuation.json")).un...
      |                    ^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
27979 -             let fixture: serde_json::Value = serde_json::from_str(include_str!("⚛️reactor/🧪️fixtures/📬️operation-continuation.json")).unwrap();
27979 +             let fixture: Value = serde_json::from_str(include_str!("⚛️reactor/🧪️fixtures/📬️operation-continuation.json")).unwrap();
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:28011:41
      |
28011 |                     self.0.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
      |                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
28011 -                     self.0.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
28011 +                     self.0.fetch_add(1, Ordering::SeqCst);
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:28020:35
      |
28020 |             assert_eq!(drops.load(std::sync::atomic::Ordering::SeqCst), 0, "the close handoff must not run the nested destructor");
      |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
28020 -             assert_eq!(drops.load(std::sync::atomic::Ordering::SeqCst), 0, "the close handoff must not run the nested destructor");
28020 +             assert_eq!(drops.load(Ordering::SeqCst), 0, "the close handoff must not run the nested destructor");
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:28022:35
      |
28022 |             assert_eq!(drops.load(std::sync::atomic::Ordering::SeqCst), 0);
      |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
28022 -             assert_eq!(drops.load(std::sync::atomic::Ordering::SeqCst), 0);
28022 +             assert_eq!(drops.load(Ordering::SeqCst), 0);
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:28024:35
      |
28024 | ...   assert_eq!(drops.load(std::sync::atomic::Ordering::SeqCst), 0, "an incomplete registry shell must fail safe without walking...
      |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
28024 -             assert_eq!(drops.load(std::sync::atomic::Ordering::SeqCst), 0, "an incomplete registry shell must fail safe without walking or dropping nested values");
28024 +             assert_eq!(drops.load(Ordering::SeqCst), 0, "an incomplete registry shell must fail safe without walking or dropping nested values");
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:28032:41
      |
28032 |                     self.0.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
      |                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
28032 -                     self.0.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
28032 +                     self.0.fetch_add(1, Ordering::SeqCst);
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:28040:35
      |
28040 |             assert_eq!(drops.load(std::sync::atomic::Ordering::SeqCst), 0);
      |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
28040 -             assert_eq!(drops.load(std::sync::atomic::Ordering::SeqCst), 0);
28040 +             assert_eq!(drops.load(Ordering::SeqCst), 0);
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:28042:35
      |
28042 |             assert_eq!(drops.load(std::sync::atomic::Ordering::SeqCst), 1);
      |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
28042 -             assert_eq!(drops.load(std::sync::atomic::Ordering::SeqCst), 1);
28042 +             assert_eq!(drops.load(Ordering::SeqCst), 1);
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:28076:31
      |
28076 |         close_cleanup_cursor: std::cell::Cell<usize>,
      |                               ^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
28076 -         close_cleanup_cursor: std::cell::Cell<usize>,
28076 +         close_cleanup_cursor: Cell<usize>,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:28077:30
      |
28077 |         live_cleanup_cursor: std::cell::Cell<usize>,
      |                              ^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
28077 -         live_cleanup_cursor: std::cell::Cell<usize>,
28077 +         live_cleanup_cursor: Cell<usize>,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:28078:36
      |
28078 |         typed_continuation_cursor: std::cell::Cell<usize>,
      |                                    ^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
28078 -         typed_continuation_cursor: std::cell::Cell<usize>,
28078 +         typed_continuation_cursor: Cell<usize>,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:28079:27
      |
28079 |         close_generation: std::cell::Cell<u64>,
      |                           ^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
28079 -         close_generation: std::cell::Cell<u64>,
28079 +         close_generation: Cell<u64>,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:28092:39
      |
28092 |                 close_cleanup_cursor: std::cell::Cell::new(0),
      |                                       ^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
28092 -                 close_cleanup_cursor: std::cell::Cell::new(0),
28092 +                 close_cleanup_cursor: Cell::new(0),
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:28093:38
      |
28093 |                 live_cleanup_cursor: std::cell::Cell::new(0),
      |                                      ^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
28093 -                 live_cleanup_cursor: std::cell::Cell::new(0),
28093 +                 live_cleanup_cursor: Cell::new(0),
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:28094:44
      |
28094 |                 typed_continuation_cursor: std::cell::Cell::new(0),
      |                                            ^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
28094 -                 typed_continuation_cursor: std::cell::Cell::new(0),
28094 +                 typed_continuation_cursor: Cell::new(0),
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:28095:35
      |
28095 |                 close_generation: std::cell::Cell::new(0),
      |                                   ^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
28095 -                 close_generation: std::cell::Cell::new(0),
28095 +                 close_generation: Cell::new(0),
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:29245:49
      |
29245 |         static RUNTIME_CLOSE_CONSTRUCTION_LIVE: std::cell::Cell<Option<bool>> = const { std::cell::Cell::new(None) };
      |                                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
29245 -         static RUNTIME_CLOSE_CONSTRUCTION_LIVE: std::cell::Cell<Option<bool>> = const { std::cell::Cell::new(None) };
29245 +         static RUNTIME_CLOSE_CONSTRUCTION_LIVE: Cell<Option<bool>> = const { std::cell::Cell::new(None) };
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:29245:89
      |
29245 |         static RUNTIME_CLOSE_CONSTRUCTION_LIVE: std::cell::Cell<Option<bool>> = const { std::cell::Cell::new(None) };
      |                                                                                         ^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
29245 -         static RUNTIME_CLOSE_CONSTRUCTION_LIVE: std::cell::Cell<Option<bool>> = const { std::cell::Cell::new(None) };
29245 +         static RUNTIME_CLOSE_CONSTRUCTION_LIVE: std::cell::Cell<Option<bool>> = const { Cell::new(None) };
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:29246:49
      |
29246 |         static RUNTIME_CLOSE_CONSTRUCTION_FAIL: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
      |                                                 ^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
29246 -         static RUNTIME_CLOSE_CONSTRUCTION_FAIL: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
29246 +         static RUNTIME_CLOSE_CONSTRUCTION_FAIL: Cell<bool> = const { std::cell::Cell::new(false) };
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:29246:81
      |
29246 |         static RUNTIME_CLOSE_CONSTRUCTION_FAIL: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
      |                                                                                 ^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
29246 -         static RUNTIME_CLOSE_CONSTRUCTION_FAIL: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
29246 +         static RUNTIME_CLOSE_CONSTRUCTION_FAIL: std::cell::Cell<bool> = const { Cell::new(false) };
      |

warning: unused doc comment
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34370:9
      |
34370 | /         /// 🧬️ dedyn-fw-os-spacemember: the closed-set `M` these composition tests register/open
34371 | |         /// children through — `store::space_members!`'s generated `SpaceMember`/`MemberFactory`
34372 | |         /// match-delegation over ONE variant, replacing the old `Box<dyn SpaceMember>` erasure (see
34373 | |         /// `📓️terra-dedyn-fw-os-spacemember-report.md`).
      | |_________------------------------------------------------^
      |           |
      |           rustdoc does not generate documentation for macro invocations
      |
      = help: to document an item produced by a macro, the macro must produce the documentation as part of its expansion

warning: unused import: `Label`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32020:29
      |
32020 |         use ui_wgpu::wgpu::{Label, LocalizedLabel};
      |                             ^^^^^

warning: unused import: `SurfaceKind`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32030:78
      |
32030 |         use crate::{selection_count_phrase, IconName, MediaClass, MediaType, SurfaceKind, ViewModel};
      |                                                                              ^^^^^^^^^^^

warning: unused import: `MutationDiff`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32031:34
      |
32031 |         use protocol::{Mutation, MutationDiff};
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
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32062:26
      |
32062 | ...   let fixture: serde_json::Value = serde_json::from_str(include_str!("🧵️retained-command/🧪️fixtures/🧬️request-context.json"))...
      |                    ^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32062 -             let fixture: serde_json::Value = serde_json::from_str(include_str!("🧵️retained-command/🧪️fixtures/🧬️request-context.json")).expect("request context fixture");
32062 +             let fixture: Value = serde_json::from_str(include_str!("🧵️retained-command/🧪️fixtures/🧬️request-context.json")).expect("request context fixture");
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32070:28
      |
32070 |             let identity = crate::app::test_artifact_owned_tool_job_context_identity_digest;
      |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32070 -             let identity = crate::app::test_artifact_owned_tool_job_context_identity_digest;
32070 +             let identity = test_artifact_owned_tool_job_context_identity_digest;
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32087:13
      |
32087 |             semio_framework::surface_app_id(&TEST_APP_DIALECT.into(), semio_framework::AppRole::Editor)
      |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32087 -             semio_framework::surface_app_id(&TEST_APP_DIALECT.into(), semio_framework::AppRole::Editor)
32087 +             surface_app_id(&TEST_APP_DIALECT.into(), semio_framework::AppRole::Editor)
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32087:71
      |
32087 |             semio_framework::surface_app_id(&TEST_APP_DIALECT.into(), semio_framework::AppRole::Editor)
      |                                                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32087 -             semio_framework::surface_app_id(&TEST_APP_DIALECT.into(), semio_framework::AppRole::Editor)
32087 +             semio_framework::surface_app_id(&TEST_APP_DIALECT.into(), AppRole::Editor)
      |

warning: unused import: `TestDiff`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32092:48
      |
32092 |         use crate::test_app_mutation_fixture::{TestDiff, TestSnapshot};
      |                                                ^^^^^^^^

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32189:148
      |
32189 | ...dencies: Vec::new(), base_version: 0, author_id: Some(protocol::ActorId(authority.actor().into())), timestamp: authority.next_...
      |                                                          ^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32189 -                         mutation_meta: vec![protocol::MutationMeta { mutation_id: None, dependencies: Vec::new(), base_version: 0, author_id: Some(protocol::ActorId(authority.actor().into())), timestamp: authority.next_clock(),
32189 +                         mutation_meta: vec![protocol::MutationMeta { mutation_id: None, dependencies: Vec::new(), base_version: 0, author_id: Some(ActorId(authority.actor().into())), timestamp: authority.next_clock(),
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32190:42
      |
32190 | ...   undo_policy: protocol::UndoPolicy::ExactBaseOnly, payload_hash: None, semantic_kind: None, label: None, group_id: None, ori...
      |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32190 -                             undo_policy: protocol::UndoPolicy::ExactBaseOnly, payload_hash: None, semantic_kind: None, label: None, group_id: None, origin: Default::default() }],
32190 +                             undo_policy: UndoPolicy::ExactBaseOnly, payload_hash: None, semantic_kind: None, label: None, group_id: None, origin: Default::default() }],
      |

warning: unnecessary qualification
 --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/⏳️completion/🦀️.rs:7:189
  |
7 | ..., TestApp::<RETAINED>::DOCUMENT_SCHEMA, semio_framework::ToolExecutionContract::resumable(4_096, 4, 1, 4_096, 7_500, 1, 1)).with_f...
  |                                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
help: remove the unnecessary path segments
  |
7 -     vec![ArtifactBoundedFirstStepProof::new::<TestApp<RETAINED>>(file!(), TestApp::<RETAINED>::APP_ID, "TestRestartFactory<true>", TEST_RESTART_TOOL, TestApp::<RETAINED>::DOCUMENT_SCHEMA, semio_framework::ToolExecutionContract::resumable(4_096, 4, 1, 4_096, 7_500, 1, 1)).with_factory_type::<TestApp<RETAINED>, TestRestartFactory<RETAINED>>()]
7 +     vec![ArtifactBoundedFirstStepProof::new::<TestApp<RETAINED>>(file!(), TestApp::<RETAINED>::APP_ID, "TestRestartFactory<true>", TEST_RESTART_TOOL, TestApp::<RETAINED>::DOCUMENT_SCHEMA, ToolExecutionContract::resumable(4_096, 4, 1, 4_096, 7_500, 1, 1)).with_factory_type::<TestApp<RETAINED>, TestRestartFactory<RETAINED>>()]
  |

warning: unnecessary qualification
  --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/⏳️completion/🦀️.rs:10:63
   |
10 | fn test_restart_register<const RETAINED: bool>(registry: &mut crate::app::ArtifactToolFactoryRegistry<'_, TestApp<RETAINED>>) -> Res...
   |                                                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
10 - fn test_restart_register<const RETAINED: bool>(registry: &mut crate::app::ArtifactToolFactoryRegistry<'_, TestApp<RETAINED>>) -> Result<(), Fault> {
10 + fn test_restart_register<const RETAINED: bool>(registry: &mut ArtifactToolFactoryRegistry<'_, TestApp<RETAINED>>) -> Result<(), Fault> {
   |

warning: unnecessary qualification
  --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/⏳️completion/🦀️.rs:12:67
   |
12 |     registry.register(TestRestartFactory::<RETAINED> { keys: vec![semio_framework::ToolFactoryKey::new(registry.controller_id(), TES...
   |                                                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
12 -     registry.register(TestRestartFactory::<RETAINED> { keys: vec![semio_framework::ToolFactoryKey::new(registry.controller_id(), TEST_RESTART_TOOL)] })
12 +     registry.register(TestRestartFactory::<RETAINED> { keys: vec![ToolFactoryKey::new(registry.controller_id(), TEST_RESTART_TOOL)] })
   |

warning: unnecessary qualification
  --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/⏳️completion/🦀️.rs:15:125
   |
15 | ...obRequest<TestApp<RETAINED>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
   |                                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
15 - async fn test_restart_build<const RETAINED: bool>(request: ArtifactOwnedToolJobRequest<TestApp<RETAINED>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
15 + async fn test_restart_build<const RETAINED: bool>(request: ArtifactOwnedToolJobRequest<TestApp<RETAINED>>) -> Result<Option<ToolOperationSpec>, Fault> {
   |

warning: unnecessary qualification
  --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/⏳️completion/🦀️.rs:20:13
   |
20 |     Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, job, request....
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
20 -     Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, job, request.operation)))
20 +     Ok(Some(ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, job, request.operation)))
   |

warning: unnecessary qualification
  --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/⏳️completion/🦀️.rs:30:31
   |
30 |             .interactive_jobs(semio_framework::InteractiveJobClassification::Migrated).await,
   |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
30 -             .interactive_jobs(semio_framework::InteractiveJobClassification::Migrated).await,
30 +             .interactive_jobs(InteractiveJobClassification::Migrated).await,
   |

warning: unnecessary qualification
  --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/⏳️completion/🦀️.rs:35:92
   |
35 | async fn test_restart_publish_and_close(command: TestCommand, meta: &ActionMeta, fixture: &serde_json::Value) {
   |                                                                                            ^^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
35 - async fn test_restart_publish_and_close(command: TestCommand, meta: &ActionMeta, fixture: &serde_json::Value) {
35 + async fn test_restart_publish_and_close(command: TestCommand, meta: &ActionMeta, fixture: &Value) {
   |

warning: unnecessary qualification
  --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/⏳️completion/🦀️.rs:55:21
   |
55 |                     crate::app::TypedOperationResultLane::Artifact => { artifact += 1; None }
   |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
55 -                     crate::app::TypedOperationResultLane::Artifact => { artifact += 1; None }
55 +                     TypedOperationResultLane::Artifact => { artifact += 1; None }
   |

warning: unnecessary qualification
  --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/⏳️completion/🦀️.rs:56:21
   |
56 |                     crate::app::TypedOperationResultLane::Ui => { ui += 1; None }
   |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
56 -                     crate::app::TypedOperationResultLane::Ui => { ui += 1; None }
56 +                     TypedOperationResultLane::Ui => { ui += 1; None }
   |

warning: unnecessary qualification
  --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/⏳️completion/🦀️.rs:57:21
   |
57 |                     crate::app::TypedOperationResultLane::Terminal => { terminal += 1; None }
   |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
57 -                     crate::app::TypedOperationResultLane::Terminal => { terminal += 1; None }
57 +                     TypedOperationResultLane::Terminal => { terminal += 1; None }
   |

warning: unnecessary qualification
  --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/⏳️completion/🦀️.rs:58:21
   |
58 | ...   crate::app::TypedOperationResultLane::Fault => Some(Fault::from(format!("restart publication fault: {}", String::from_utf8_los...
   |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
help: remove the unnecessary path segments
   |
58 -                     crate::app::TypedOperationResultLane::Fault => Some(Fault::from(format!("restart publication fault: {}", String::from_utf8_lossy(page.bytes())))),
58 +                     TypedOperationResultLane::Fault => Some(Fault::from(format!("restart publication fault: {}", String::from_utf8_lossy(page.bytes())))),
   |

warning: unnecessary qualification
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/⏳️completion/🦀️.rs:102:17
    |
102 |     raw: Option<semio_framework::action_bus::RetainedToolWireInput>,
    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
102 -     raw: Option<semio_framework::action_bus::RetainedToolWireInput>,
102 +     raw: Option<action_bus::RetainedToolWireInput>,
    |

warning: unnecessary qualification
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/⏳️completion/🦀️.rs:132:25
    |
132 |             let bytes = std::mem::size_of::<TestCommand>();
    |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
132 -             let bytes = std::mem::size_of::<TestCommand>();
132 +             let bytes = size_of::<TestCommand>();
    |

warning: unnecessary qualification
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/⏳️completion/🦀️.rs:145:61
    |
145 | struct TestRestartFactory<const RETAINED: bool> { keys: Vec<semio_framework::ToolFactoryKey> }
    |                                                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
145 - struct TestRestartFactory<const RETAINED: bool> { keys: Vec<semio_framework::ToolFactoryKey> }
145 + struct TestRestartFactory<const RETAINED: bool> { keys: Vec<ToolFactoryKey> }
    |

warning: unnecessary qualification
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/⏳️completion/🦀️.rs:147:28
    |
147 | impl<const RETAINED: bool> semio_framework::ToolJobFactory for TestRestartFactory<RETAINED> {
    |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
147 - impl<const RETAINED: bool> semio_framework::ToolJobFactory for TestRestartFactory<RETAINED> {
147 + impl<const RETAINED: bool> ToolJobFactory for TestRestartFactory<RETAINED> {
    |

warning: unnecessary qualification
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/⏳️completion/🦀️.rs:150:25
    |
150 |     fn keys(&self) -> &[semio_framework::ToolFactoryKey] { &self.keys }
    |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
150 -     fn keys(&self) -> &[semio_framework::ToolFactoryKey] { &self.keys }
150 +     fn keys(&self) -> &[ToolFactoryKey] { &self.keys }
    |

warning: unnecessary qualification
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/⏳️completion/🦀️.rs:152:33
    |
152 |     fn classification(&self) -> semio_framework::InteractiveJobClassification { semio_framework::InteractiveJobClassification::Migr...
    |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
152 -     fn classification(&self) -> semio_framework::InteractiveJobClassification { semio_framework::InteractiveJobClassification::Migrated }
152 +     fn classification(&self) -> InteractiveJobClassification { semio_framework::InteractiveJobClassification::Migrated }
    |

warning: unnecessary qualification
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/⏳️completion/🦀️.rs:152:81
    |
152 | ...ork::InteractiveJobClassification { semio_framework::InteractiveJobClassification::Migrated }
    |                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
152 -     fn classification(&self) -> semio_framework::InteractiveJobClassification { semio_framework::InteractiveJobClassification::Migrated }
152 +     fn classification(&self) -> semio_framework::InteractiveJobClassification { InteractiveJobClassification::Migrated }
    |

warning: unnecessary qualification
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/⏳️completion/🦀️.rs:153:37
    |
153 |     fn execution_contract(&self) -> semio_framework::ToolExecutionContract { semio_framework::ToolExecutionContract::resumable(4_09...
    |                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
153 -     fn execution_contract(&self) -> semio_framework::ToolExecutionContract { semio_framework::ToolExecutionContract::resumable(4_096, 4, 1, 4_096, 7_500, 1, 1) }
153 +     fn execution_contract(&self) -> ToolExecutionContract { semio_framework::ToolExecutionContract::resumable(4_096, 4, 1, 4_096, 7_500, 1, 1) }
    |

warning: unnecessary qualification
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/⏳️completion/🦀️.rs:153:78
    |
153 |     fn execution_contract(&self) -> semio_framework::ToolExecutionContract { semio_framework::ToolExecutionContract::resumable(4_09...
    |                                                                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
153 -     fn execution_contract(&self) -> semio_framework::ToolExecutionContract { semio_framework::ToolExecutionContract::resumable(4_096, 4, 1, 4_096, 7_500, 1, 1) }
153 +     fn execution_contract(&self) -> semio_framework::ToolExecutionContract { ToolExecutionContract::resumable(4_096, 4, 1, 4_096, 7_500, 1, 1) }
    |

warning: unnecessary qualification
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/⏳️completion/🦀️.rs:154:119
    |
154 | ... payload: Self::Payload) -> Result<Self::Job, semio_framework::ToolJobFactoryError> { Ok(payload) }
    |                                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
154 -     fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, semio_framework::ToolJobFactoryError> { Ok(payload) }
154 +     fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> { Ok(payload) }
    |

warning: unnecessary qualification
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/⏳️completion/🦀️.rs:155:138
    |
155 | ...on, mut payload: Self::Payload, input: semio_framework::action_bus::RetainedToolWireInput, checkpoint: Option<semio_framework::a...
    |                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
155 -     fn create_job_from_wire_pages_with_payload(&mut self, _operation: semio_framework_job::Operation, mut payload: Self::Payload, input: semio_framework::action_bus::RetainedToolWireInput, checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
155 +     fn create_job_from_wire_pages_with_payload(&mut self, _operation: semio_framework_job::Operation, mut payload: Self::Payload, input: action_bus::RetainedToolWireInput, checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
    |

warning: unnecessary qualification
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/⏳️completion/🦀️.rs:155:209
    |
155 | ...ainedToolWireInput, checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>) -> Result<Self::Job, (semio_framewor...
    |                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
155 -     fn create_job_from_wire_pages_with_payload(&mut self, _operation: semio_framework_job::Operation, mut payload: Self::Payload, input: semio_framework::action_bus::RetainedToolWireInput, checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
155 +     fn create_job_from_wire_pages_with_payload(&mut self, _operation: semio_framework_job::Operation, mut payload: Self::Payload, input: semio_framework::action_bus::RetainedToolWireInput, checkpoint: Option<action_bus::RetainedToolWireInput>) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
    |

warning: unnecessary qualification
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/⏳️completion/🦀️.rs:155:284
    |
155 | ...RetainedToolWireInput>) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, semio_framework::action_bus::RetainedToolWir...
    |                                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
155 -     fn create_job_from_wire_pages_with_payload(&mut self, _operation: semio_framework_job::Operation, mut payload: Self::Payload, input: semio_framework::action_bus::RetainedToolWireInput, checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
155 +     fn create_job_from_wire_pages_with_payload(&mut self, _operation: semio_framework_job::Operation, mut payload: Self::Payload, input: semio_framework::action_bus::RetainedToolWireInput, checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>) -> Result<Self::Job, (ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
    |

warning: unnecessary qualification
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/⏳️completion/🦀️.rs:155:322
    |
155 | ...(semio_framework::ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::R...
    |                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
155 -     fn create_job_from_wire_pages_with_payload(&mut self, _operation: semio_framework_job::Operation, mut payload: Self::Payload, input: semio_framework::action_bus::RetainedToolWireInput, checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
155 +     fn create_job_from_wire_pages_with_payload(&mut self, _operation: semio_framework_job::Operation, mut payload: Self::Payload, input: semio_framework::action_bus::RetainedToolWireInput, checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
    |

warning: unnecessary qualification
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/⏳️completion/🦀️.rs:155:381
    |
155 | ...ion_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
    |                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
155 -     fn create_job_from_wire_pages_with_payload(&mut self, _operation: semio_framework_job::Operation, mut payload: Self::Payload, input: semio_framework::action_bus::RetainedToolWireInput, checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
155 +     fn create_job_from_wire_pages_with_payload(&mut self, _operation: semio_framework_job::Operation, mut payload: Self::Payload, input: semio_framework::action_bus::RetainedToolWireInput, checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<action_bus::RetainedToolWireInput>)> {
    |

warning: unnecessary qualification
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/⏳️completion/🦀️.rs:156:47
    |
156 | ...   if checkpoint.is_some() { return Err((semio_framework::ToolJobFactoryError::new("restart resume starts a fresh command owner"...
    |                                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
156 -         if checkpoint.is_some() { return Err((semio_framework::ToolJobFactoryError::new("restart resume starts a fresh command owner"), input, checkpoint)); }
156 +         if checkpoint.is_some() { return Err((ToolJobFactoryError::new("restart resume starts a fresh command owner"), input, checkpoint)); }
    |

warning: unnecessary qualification
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/⏳️completion/🦀️.rs:208:18
    |
208 |     let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixture.json")).unwrap();
    |                  ^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
208 -     let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixture.json")).unwrap();
208 +     let fixture: Value = serde_json::from_str(include_str!("🧪️fixture.json")).unwrap();
    |

warning: unnecessary qualification
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/⏳️completion/🦀️.rs:214:13
    |
214 |     assert!(std::mem::size_of::<PublicationTransient>() + std::mem::size_of_val(&owner) + 2 * std::mem::size_of::<usize>() <= 4_096);
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
214 -     assert!(std::mem::size_of::<PublicationTransient>() + std::mem::size_of_val(&owner) + 2 * std::mem::size_of::<usize>() <= 4_096);
214 +     assert!(size_of::<PublicationTransient>() + std::mem::size_of_val(&owner) + 2 * std::mem::size_of::<usize>() <= 4_096);
    |

warning: unnecessary qualification
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/⏳️completion/🦀️.rs:214:59
    |
214 |     assert!(std::mem::size_of::<PublicationTransient>() + std::mem::size_of_val(&owner) + 2 * std::mem::size_of::<usize>() <= 4_096);
    |                                                           ^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
214 -     assert!(std::mem::size_of::<PublicationTransient>() + std::mem::size_of_val(&owner) + 2 * std::mem::size_of::<usize>() <= 4_096);
214 +     assert!(std::mem::size_of::<PublicationTransient>() + size_of_val(&owner) + 2 * std::mem::size_of::<usize>() <= 4_096);
    |

warning: unnecessary qualification
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/⏳️completion/🦀️.rs:214:95
    |
214 |     assert!(std::mem::size_of::<PublicationTransient>() + std::mem::size_of_val(&owner) + 2 * std::mem::size_of::<usize>() <= 4_096);
    |                                                                                               ^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
214 -     assert!(std::mem::size_of::<PublicationTransient>() + std::mem::size_of_val(&owner) + 2 * std::mem::size_of::<usize>() <= 4_096);
214 +     assert!(std::mem::size_of::<PublicationTransient>() + std::mem::size_of_val(&owner) + 2 * size_of::<usize>() <= 4_096);
    |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32345:95
      |
32345 | ...   fn extent(&self, _command: &TestCommand, _snapshot: &TestSnapshot, _interaction: &protocol::InteractionState, _context: Opt...
      |                                                                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32345 -             fn extent(&self, _command: &TestCommand, _snapshot: &TestSnapshot, _interaction: &protocol::InteractionState, _context: Option<&crate::app::ArtifactOwnedToolJobContext<TestApp>>) -> Option<usize> {
32345 +             fn extent(&self, _command: &TestCommand, _snapshot: &TestSnapshot, _interaction: &InteractionState, _context: Option<&crate::app::ArtifactOwnedToolJobContext<TestApp>>) -> Option<usize> {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32345:141
      |
32345 | ...l::InteractionState, _context: Option<&crate::app::ArtifactOwnedToolJobContext<TestApp>>) -> Option<usize> {
      |                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32345 -             fn extent(&self, _command: &TestCommand, _snapshot: &TestSnapshot, _interaction: &protocol::InteractionState, _context: Option<&crate::app::ArtifactOwnedToolJobContext<TestApp>>) -> Option<usize> {
32345 +             fn extent(&self, _command: &TestCommand, _snapshot: &TestSnapshot, _interaction: &protocol::InteractionState, _context: Option<&ArtifactOwnedToolJobContext<TestApp>>) -> Option<usize> {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32355:32
      |
32355 |                 _interaction: &protocol::InteractionState,
      |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32355 -                 _interaction: &protocol::InteractionState,
32355 +                 _interaction: &InteractionState,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32357:35
      |
32357 |                 _context: Option<&crate::app::ArtifactOwnedToolJobContext<TestApp>>,
      |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32357 -                 _context: Option<&crate::app::ArtifactOwnedToolJobContext<TestApp>>,
32357 +                 _context: Option<&ArtifactOwnedToolJobContext<TestApp>>,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32358:30
      |
32358 |                 _operation: &crate::app::AppOperationContext,
      |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32358 -                 _operation: &crate::app::AppOperationContext,
32358 +                 _operation: &AppOperationContext,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32385:23
      |
32385 |             keys: Vec<semio_framework::ToolFactoryKey>,
      |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32385 -             keys: Vec<semio_framework::ToolFactoryKey>,
32385 +             keys: Vec<ToolFactoryKey>,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32390:35
      |
32390 | ...   Self { keys: vec![semio_framework::ToolFactoryKey::new(TEST_RETAINED_COMMAND_CONTROLLER, TEST_RETAINED_COMMAND_TOOL)] }
      |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32390 -                 Self { keys: vec![semio_framework::ToolFactoryKey::new(TEST_RETAINED_COMMAND_CONTROLLER, TEST_RETAINED_COMMAND_TOOL)] }
32390 +                 Self { keys: vec![ToolFactoryKey::new(TEST_RETAINED_COMMAND_CONTROLLER, TEST_RETAINED_COMMAND_TOOL)] }
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32396:14
      |
32396 |         impl semio_framework::ToolJobFactory for OtherTestRetainedCommandFactory {
      |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32396 -         impl semio_framework::ToolJobFactory for OtherTestRetainedCommandFactory {
32396 +         impl ToolJobFactory for OtherTestRetainedCommandFactory {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32399:33
      |
32399 |             fn keys(&self) -> &[semio_framework::ToolFactoryKey] { &self.0.keys }
      |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32399 -             fn keys(&self) -> &[semio_framework::ToolFactoryKey] { &self.0.keys }
32399 +             fn keys(&self) -> &[ToolFactoryKey] { &self.0.keys }
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32401:41
      |
32401 | ...   fn classification(&self) -> semio_framework::InteractiveJobClassification { semio_framework::InteractiveJobClassification::...
      |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32401 -             fn classification(&self) -> semio_framework::InteractiveJobClassification { semio_framework::InteractiveJobClassification::Migrated }
32401 +             fn classification(&self) -> InteractiveJobClassification { semio_framework::InteractiveJobClassification::Migrated }
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32401:89
      |
32401 | ...rk::InteractiveJobClassification { semio_framework::InteractiveJobClassification::Migrated }
      |                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32401 -             fn classification(&self) -> semio_framework::InteractiveJobClassification { semio_framework::InteractiveJobClassification::Migrated }
32401 +             fn classification(&self) -> semio_framework::InteractiveJobClassification { InteractiveJobClassification::Migrated }
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32402:45
      |
32402 | ...   fn execution_contract(&self) -> semio_framework::ToolExecutionContract { semio_framework::ToolJobFactory::execution_contrac...
      |                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32402 -             fn execution_contract(&self) -> semio_framework::ToolExecutionContract { semio_framework::ToolJobFactory::execution_contract(&self.0) }
32402 +             fn execution_contract(&self) -> ToolExecutionContract { semio_framework::ToolJobFactory::execution_contract(&self.0) }
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32402:86
      |
32402 | ...o_framework::ToolExecutionContract { semio_framework::ToolJobFactory::execution_contract(&self.0) }
      |                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32402 -             fn execution_contract(&self) -> semio_framework::ToolExecutionContract { semio_framework::ToolJobFactory::execution_contract(&self.0) }
32402 +             fn execution_contract(&self) -> semio_framework::ToolExecutionContract { ToolJobFactory::execution_contract(&self.0) }
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32403:126
      |
32403 | ...payload: Self::Payload) -> Result<Self::Job, semio_framework::ToolJobFactoryError> {
      |                                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32403 -             fn create_job(&mut self, operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, semio_framework::ToolJobFactoryError> {
32403 +             fn create_job(&mut self, operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32404:17
      |
32404 |                 semio_framework::ToolJobFactory::create_job(&mut self.0, operation, payload)
      |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32404 -                 semio_framework::ToolJobFactory::create_job(&mut self.0, operation, payload)
32404 +                 ToolJobFactory::create_job(&mut self.0, operation, payload)
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32422:14
      |
32422 |         impl semio_framework::ToolJobFactory for TestRetainedCommandFactory {
      |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32422 -         impl semio_framework::ToolJobFactory for TestRetainedCommandFactory {
32422 +         impl ToolJobFactory for TestRetainedCommandFactory {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32426:33
      |
32426 |             fn keys(&self) -> &[semio_framework::ToolFactoryKey] {
      |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32426 -             fn keys(&self) -> &[semio_framework::ToolFactoryKey] {
32426 +             fn keys(&self) -> &[ToolFactoryKey] {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32434:41
      |
32434 |             fn classification(&self) -> semio_framework::InteractiveJobClassification {
      |                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32434 -             fn classification(&self) -> semio_framework::InteractiveJobClassification {
32434 +             fn classification(&self) -> InteractiveJobClassification {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32435:17
      |
32435 |                 semio_framework::InteractiveJobClassification::Migrated
      |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32435 -                 semio_framework::InteractiveJobClassification::Migrated
32435 +                 InteractiveJobClassification::Migrated
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32438:45
      |
32438 |             fn execution_contract(&self) -> semio_framework::ToolExecutionContract {
      |                                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32438 -             fn execution_contract(&self) -> semio_framework::ToolExecutionContract {
32438 +             fn execution_contract(&self) -> ToolExecutionContract {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32439:17
      |
32439 | ...   semio_framework::ToolExecutionContract::resumable(TEST_RETAINED_COMMAND_RAW_BYTES, 4, 1, TEST_RETAINED_COMMAND_RAW_BYTES, 7...
      |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32439 -                 semio_framework::ToolExecutionContract::resumable(TEST_RETAINED_COMMAND_RAW_BYTES, 4, 1, TEST_RETAINED_COMMAND_RAW_BYTES, 7_500, 1, 1)
32439 +                 ToolExecutionContract::resumable(TEST_RETAINED_COMMAND_RAW_BYTES, 4, 1, TEST_RETAINED_COMMAND_RAW_BYTES, 7_500, 1, 1)
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32442:127
      |
32442 | ...payload: Self::Payload) -> Result<Self::Job, semio_framework::ToolJobFactoryError> {
      |                                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32442 -             fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, semio_framework::ToolJobFactoryError> {
32442 +             fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32450:24
      |
32450 |                 input: semio_framework::action_bus::RetainedToolWireInput,
      |                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32450 -                 input: semio_framework::action_bus::RetainedToolWireInput,
32450 +                 input: action_bus::RetainedToolWireInput,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32451:36
      |
32451 |                 checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>,
      |                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32451 -                 checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>,
32451 +                 checkpoint: Option<action_bus::RetainedToolWireInput>,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32452:37
      |
32452 | ...   ) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<se...
      |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32452 -             ) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
32452 +             ) -> Result<Self::Job, (ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32452:75
      |
32452 | ...   ) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<se...
      |                                                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32452 -             ) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
32452 +             ) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32452:134
      |
32452 | ...on_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
      |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32452 -             ) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
32452 +             ) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<action_bus::RetainedToolWireInput>)> {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32454:33
      |
32454 |                     return Err((semio_framework::ToolJobFactoryError::new("test retained command extent"), input, checkpoint));
      |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32454 -                     return Err((semio_framework::ToolJobFactoryError::new("test retained command extent"), input, checkpoint));
32454 +                     return Err((ToolJobFactoryError::new("test retained command extent"), input, checkpoint));
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32470:60
      |
32470 | ...   async fn test_retained_command_payload(completion: crate::app::ArtifactToolCompletion<TestApp>) -> crate::retained_command:...
      |                                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32470 -         async fn test_retained_command_payload(completion: crate::app::ArtifactToolCompletion<TestApp>) -> crate::retained_command::ArtifactRetainedCommandPayload<TestApp> {
32470 +         async fn test_retained_command_payload(completion: ArtifactToolCompletion<TestApp>) -> crate::retained_command::ArtifactRetainedCommandPayload<TestApp> {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32476:37
      |
32476 |                 std::sync::Arc::new(protocol::InteractionState::default()),
      |                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32476 -                 std::sync::Arc::new(protocol::InteractionState::default()),
32476 +                 std::sync::Arc::new(InteractionState::default()),
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32478:17
      |
32478 | ...   crate::app::AppOperationContext { app_instance_id: 7, parent_document_id: "test-document".into(), operation_id: 41, generat...
      |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32478 -                 crate::app::AppOperationContext { app_instance_id: 7, parent_document_id: "test-document".into(), operation_id: 41, generation: 3, canonical_base_revision: [5; 32] },
32478 +                 AppOperationContext { app_instance_id: 7, parent_document_id: "test-document".into(), operation_id: 41, generation: 3, canonical_base_revision: [5; 32] },
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32488:43
      |
32488 | ...   fn test_retained_wire_input(bus: &semio_framework::ActionBus, bytes: &[u8]) -> (semio_framework::ToolWireAdmission, semio_f...
      |                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32488 -         fn test_retained_wire_input(bus: &semio_framework::ActionBus, bytes: &[u8]) -> (semio_framework::ToolWireAdmission, semio_framework::action_bus::RetainedToolWireInput) {
32488 +         fn test_retained_wire_input(bus: &ActionBus, bytes: &[u8]) -> (semio_framework::ToolWireAdmission, semio_framework::action_bus::RetainedToolWireInput) {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32488:89
      |
32488 | ...   fn test_retained_wire_input(bus: &semio_framework::ActionBus, bytes: &[u8]) -> (semio_framework::ToolWireAdmission, semio_f...
      |                                                                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32488 -         fn test_retained_wire_input(bus: &semio_framework::ActionBus, bytes: &[u8]) -> (semio_framework::ToolWireAdmission, semio_framework::action_bus::RetainedToolWireInput) {
32488 +         fn test_retained_wire_input(bus: &semio_framework::ActionBus, bytes: &[u8]) -> (ToolWireAdmission, semio_framework::action_bus::RetainedToolWireInput) {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32488:125
      |
32488 | ... (semio_framework::ToolWireAdmission, semio_framework::action_bus::RetainedToolWireInput) {
      |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32488 -         fn test_retained_wire_input(bus: &semio_framework::ActionBus, bytes: &[u8]) -> (semio_framework::ToolWireAdmission, semio_framework::action_bus::RetainedToolWireInput) {
32488 +         fn test_retained_wire_input(bus: &semio_framework::ActionBus, bytes: &[u8]) -> (semio_framework::ToolWireAdmission, action_bus::RetainedToolWireInput) {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32490:38
      |
32490 |             for page in bytes.chunks(semio_framework::action_bus::TOOL_WIRE_PAGE_BYTES) {
      |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32490 -             for page in bytes.chunks(semio_framework::action_bus::TOOL_WIRE_PAGE_BYTES) {
32490 +             for page in bytes.chunks(action_bus::TOOL_WIRE_PAGE_BYTES) {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32491:34
      |
32491 | ...   input.admit_page(semio_framework::action_bus::ToolWirePage::try_copy_from(page).expect("test retained wire page")).map_err(...
      |                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32491 -                 input.admit_page(semio_framework::action_bus::ToolWirePage::try_copy_from(page).expect("test retained wire page")).map_err(|(fault, _)| fault).expect("test retained page admission");
32491 +                 input.admit_page(action_bus::ToolWirePage::try_copy_from(page).expect("test retained wire page")).map_err(|(fault, _)| fault).expect("test retained page admission");
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32676:59
      |
32676 | ...   fn register_tool_job_factories(registry: &mut crate::app::ArtifactToolFactoryRegistry<'_, Self>) -> Result<(), Fault> { tes...
      |                                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32676 -             fn register_tool_job_factories(registry: &mut crate::app::ArtifactToolFactoryRegistry<'_, Self>) -> Result<(), Fault> { test_restart_register::<RETAINED>(registry) }
32676 +             fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, Self>) -> Result<(), Fault> { test_restart_register::<RETAINED>(registry) }
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32678:98
      |
32678 | ...   async fn build_tool_job(request: ArtifactOwnedToolJobRequest<Self>) -> Result<Option<semio_framework::ToolOperationSpec>, F...
      |                                                                                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32678 -             async fn build_tool_job(request: ArtifactOwnedToolJobRequest<Self>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> { test_restart_build::<RETAINED>(request).await }
32678 +             async fn build_tool_job(request: ArtifactOwnedToolJobRequest<Self>) -> Result<Option<ToolOperationSpec>, Fault> { test_restart_build::<RETAINED>(request).await }
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32752:28
      |
32752 |                 presence: &crate::app::PresenceView<'_, PublicationPresence>,
      |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32752 -                 presence: &crate::app::PresenceView<'_, PublicationPresence>,
32752 +                 presence: &PresenceView<'_, PublicationPresence>,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32753:29
      |
32753 |                 transient: &crate::app::TransientView<'_, PublicationTransient>,
      |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32753 -                 transient: &crate::app::TransientView<'_, PublicationTransient>,
32753 +                 transient: &TransientView<'_, PublicationTransient>,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32754:18
      |
32754 |             ) -> crate::app::EphemeralEmit<Self> {
      |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32754 -             ) -> crate::app::EphemeralEmit<Self> {
32754 +             ) -> EphemeralEmit<Self> {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32756:47
      |
32756 |                     TestCommand::Increment => crate::app::EphemeralEmit {
      |                                               ^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32756 -                     TestCommand::Increment => crate::app::EphemeralEmit {
32756 +                     TestCommand::Increment => EphemeralEmit {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32760:26
      |
32760 |                     _ => crate::app::EphemeralEmit::default(),
      |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32760 -                     _ => crate::app::EphemeralEmit::default(),
32760 +                     _ => EphemeralEmit::default(),
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32787:70
      |
32787 |             async fn command_from_action(action: &str, args: Option<&serde_json::Value>) -> Result<Self::Command, Fault> {
      |                                                                      ^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32787 -             async fn command_from_action(action: &str, args: Option<&serde_json::Value>) -> Result<Self::Command, Fault> {
32787 +             async fn command_from_action(action: &str, args: Option<&Value>) -> Result<Self::Command, Fault> {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32790:140
      |
32790 | ...rgs.and_then(|value| value.get("value")).and_then(serde_json::Value::as_str).unwrap_or_default().to_string() }),
      |                                                      ^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32790 -                     "setLabelViaCommand" => Ok(TestCommand::SetLabelViaCommand { value: args.and_then(|value| value.get("value")).and_then(serde_json::Value::as_str).unwrap_or_default().to_string() }),
32790 +                     "setLabelViaCommand" => Ok(TestCommand::SetLabelViaCommand { value: args.and_then(|value| value.get("value")).and_then(Value::as_str).unwrap_or_default().to_string() }),
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32791:137
      |
32791 | ....and_then(|value| value.get("windowId")).and_then(serde_json::Value::as_str).unwrap_or_default().to_string() }),
      |                                                      ^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32791 -                     "targetWindow" => Ok(TestCommand::SetLabelViaCommand { value: args.and_then(|value| value.get("windowId")).and_then(serde_json::Value::as_str).unwrap_or_default().to_string() }),
32791 +                     "targetWindow" => Ok(TestCommand::SetLabelViaCommand { value: args.and_then(|value| value.get("windowId")).and_then(Value::as_str).unwrap_or_default().to_string() }),
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32793:81
      |
32793 | ...   slot: args.and_then(|value| value.get("slot")).and_then(serde_json::Value::as_str).unwrap_or_default().to_string(),
      |                                                               ^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32793 -                         slot: args.and_then(|value| value.get("slot")).and_then(serde_json::Value::as_str).unwrap_or_default().to_string(),
32793 +                         slot: args.and_then(|value| value.get("slot")).and_then(Value::as_str).unwrap_or_default().to_string(),
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32794:88
      |
32794 | ...   child_id: args.and_then(|value| value.get("childId")).and_then(serde_json::Value::as_str).unwrap_or_default().to_string(),
      |                                                                      ^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32794 -                         child_id: args.and_then(|value| value.get("childId")).and_then(serde_json::Value::as_str).unwrap_or_default().to_string(),
32794 +                         child_id: args.and_then(|value| value.get("childId")).and_then(Value::as_str).unwrap_or_default().to_string(),
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32828:135
      |
32828 | ...nt(AppEvent { kind: "active-utility".into(), payload: dsl::to_dsl_value(&json!({ "utilityId": utility_id.clone() })).unwrap_or...
      |                                                          ^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32828 -                     TestCommand::SetActiveUtility { utility_id } => Ok(Emit::event(AppEvent { kind: "active-utility".into(), payload: dsl::to_dsl_value(&json!({ "utilityId": utility_id.clone() })).unwrap_or(dsl::DslValue::Null) })),
32828 +                     TestCommand::SetActiveUtility { utility_id } => Ok(Emit::event(AppEvent { kind: "active-utility".into(), payload: to_dsl_value(&json!({ "utilityId": utility_id.clone() })).unwrap_or(dsl::DslValue::Null) })),
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32828:208
      |
32828 | ...son!({ "utilityId": utility_id.clone() })).unwrap_or(dsl::DslValue::Null) })),
      |                                                         ^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32828 -                     TestCommand::SetActiveUtility { utility_id } => Ok(Emit::event(AppEvent { kind: "active-utility".into(), payload: dsl::to_dsl_value(&json!({ "utilityId": utility_id.clone() })).unwrap_or(dsl::DslValue::Null) })),
32828 +                     TestCommand::SetActiveUtility { utility_id } => Ok(Emit::event(AppEvent { kind: "active-utility".into(), payload: dsl::to_dsl_value(&json!({ "utilityId": utility_id.clone() })).unwrap_or(DslValue::Null) })),
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32862:139
      |
32862 | ...View<'_, TestConfig>) -> UiAssemblyResult<semio_framework_ui_runtime::ComponentTree> {
      |                                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32862 -             async fn render(_body_key: &str, doc: &ArtifactView<'_, TestSnapshot>, _cfg: &ConfigView<'_, TestConfig>) -> UiAssemblyResult<semio_framework_ui_runtime::ComponentTree> {
32862 +             async fn render(_body_key: &str, doc: &ArtifactView<'_, TestSnapshot>, _cfg: &ConfigView<'_, TestConfig>) -> UiAssemblyResult<ComponentTree> {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32933:119
      |
32933 | ...napshot>, _cfg: &ConfigView<'_, TestConfig>) -> protocol::InteractionTopology {
      |                                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32933 -             async fn interaction_topology(doc: &ArtifactView<'_, TestSnapshot>, _cfg: &ConfigView<'_, TestConfig>) -> protocol::InteractionTopology {
32933 +             async fn interaction_topology(doc: &ArtifactView<'_, TestSnapshot>, _cfg: &ConfigView<'_, TestConfig>) -> InteractionTopology {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32935:91
      |
32935 | ...   let ordered = if doc.snapshot.label.is_empty() { Vec::new() } else { vec![protocol::TopologyNode { id: "item-1".into(), gra...
      |                                                                                 ^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32935 -                 let ordered = if doc.snapshot.label.is_empty() { Vec::new() } else { vec![protocol::TopologyNode { id: "item-1".into(), granularity: "item".into(), parent: None }] };
32935 +                 let ordered = if doc.snapshot.label.is_empty() { Vec::new() } else { vec![TopologyNode { id: "item-1".into(), granularity: "item".into(), parent: None }] };
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32936:53
      |
32936 |                 domains.insert("items".to_string(), protocol::DomainTopology { ordered });
      |                                                     ^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32936 -                 domains.insert("items".to_string(), protocol::DomainTopology { ordered });
32936 +                 domains.insert("items".to_string(), DomainTopology { ordered });
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32937:17
      |
32937 |                 protocol::InteractionTopology { domains }
      |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32937 -                 protocol::InteractionTopology { domains }
32937 +                 InteractionTopology { domains }
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32963:25
      |
32963 |             raw: Option<semio_framework::action_bus::RetainedToolWireInput>,
      |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
32963 -             raw: Option<semio_framework::action_bus::RetainedToolWireInput>,
32963 +             raw: Option<action_bus::RetainedToolWireInput>,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33013:45
      |
33013 |         struct KeyedTestFactory { keys: Vec<semio_framework::ToolFactoryKey> }
      |                                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33013 -         struct KeyedTestFactory { keys: Vec<semio_framework::ToolFactoryKey> }
33013 +         struct KeyedTestFactory { keys: Vec<ToolFactoryKey> }
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33015:14
      |
33015 |         impl semio_framework::ToolJobFactory for KeyedTestFactory {
      |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33015 -         impl semio_framework::ToolJobFactory for KeyedTestFactory {
33015 +         impl ToolJobFactory for KeyedTestFactory {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33018:33
      |
33018 |             fn keys(&self) -> &[semio_framework::ToolFactoryKey] { &self.keys }
      |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33018 -             fn keys(&self) -> &[semio_framework::ToolFactoryKey] { &self.keys }
33018 +             fn keys(&self) -> &[ToolFactoryKey] { &self.keys }
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33020:41
      |
33020 | ...   fn classification(&self) -> semio_framework::InteractiveJobClassification { semio_framework::InteractiveJobClassification::...
      |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33020 -             fn classification(&self) -> semio_framework::InteractiveJobClassification { semio_framework::InteractiveJobClassification::Migrated }
33020 +             fn classification(&self) -> InteractiveJobClassification { semio_framework::InteractiveJobClassification::Migrated }
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33020:89
      |
33020 | ...rk::InteractiveJobClassification { semio_framework::InteractiveJobClassification::Migrated }
      |                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33020 -             fn classification(&self) -> semio_framework::InteractiveJobClassification { semio_framework::InteractiveJobClassification::Migrated }
33020 +             fn classification(&self) -> semio_framework::InteractiveJobClassification { InteractiveJobClassification::Migrated }
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33021:45
      |
33021 | ...   fn execution_contract(&self) -> semio_framework::ToolExecutionContract { semio_framework::ToolExecutionContract::resumable(...
      |                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33021 -             fn execution_contract(&self) -> semio_framework::ToolExecutionContract { semio_framework::ToolExecutionContract::resumable(32_768, 4, 1, 4_096, 500, 1, 1) }
33021 +             fn execution_contract(&self) -> ToolExecutionContract { semio_framework::ToolExecutionContract::resumable(32_768, 4, 1, 4_096, 500, 1, 1) }
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33021:86
      |
33021 | ...io_framework::ToolExecutionContract { semio_framework::ToolExecutionContract::resumable(32_768, 4, 1, 4_096, 500, 1, 1) }
      |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33021 -             fn execution_contract(&self) -> semio_framework::ToolExecutionContract { semio_framework::ToolExecutionContract::resumable(32_768, 4, 1, 4_096, 500, 1, 1) }
33021 +             fn execution_contract(&self) -> semio_framework::ToolExecutionContract { ToolExecutionContract::resumable(32_768, 4, 1, 4_096, 500, 1, 1) }
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33022:127
      |
33022 | ...payload: Self::Payload) -> Result<Self::Job, semio_framework::ToolJobFactoryError> { Ok(payload) }
      |                                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33022 -             fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, semio_framework::ToolJobFactoryError> { Ok(payload) }
33022 +             fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> { Ok(payload) }
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33023:146
      |
33023 | ...n, mut payload: Self::Payload, input: semio_framework::action_bus::RetainedToolWireInput, checkpoint: Option<semio_framework::...
      |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33023 -             fn create_job_from_wire_pages_with_payload(&mut self, _operation: semio_framework_job::Operation, mut payload: Self::Payload, input: semio_framework::action_bus::RetainedToolWireInput, checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
33023 +             fn create_job_from_wire_pages_with_payload(&mut self, _operation: semio_framework_job::Operation, mut payload: Self::Payload, input: action_bus::RetainedToolWireInput, checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33023:217
      |
33023 | ...inedToolWireInput, checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>) -> Result<Self::Job, (semio_framewo...
      |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33023 -             fn create_job_from_wire_pages_with_payload(&mut self, _operation: semio_framework_job::Operation, mut payload: Self::Payload, input: semio_framework::action_bus::RetainedToolWireInput, checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
33023 +             fn create_job_from_wire_pages_with_payload(&mut self, _operation: semio_framework_job::Operation, mut payload: Self::Payload, input: semio_framework::action_bus::RetainedToolWireInput, checkpoint: Option<action_bus::RetainedToolWireInput>) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33023:292
      |
33023 | ...etainedToolWireInput>) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, semio_framework::action_bus::RetainedToolWi...
      |                                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33023 -             fn create_job_from_wire_pages_with_payload(&mut self, _operation: semio_framework_job::Operation, mut payload: Self::Payload, input: semio_framework::action_bus::RetainedToolWireInput, checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
33023 +             fn create_job_from_wire_pages_with_payload(&mut self, _operation: semio_framework_job::Operation, mut payload: Self::Payload, input: semio_framework::action_bus::RetainedToolWireInput, checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>) -> Result<Self::Job, (ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33023:330
      |
33023 | ...semio_framework::ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::...
      |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33023 -             fn create_job_from_wire_pages_with_payload(&mut self, _operation: semio_framework_job::Operation, mut payload: Self::Payload, input: semio_framework::action_bus::RetainedToolWireInput, checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
33023 +             fn create_job_from_wire_pages_with_payload(&mut self, _operation: semio_framework_job::Operation, mut payload: Self::Payload, input: semio_framework::action_bus::RetainedToolWireInput, checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33023:389
      |
33023 | ...on_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
      |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33023 -             fn create_job_from_wire_pages_with_payload(&mut self, _operation: semio_framework_job::Operation, mut payload: Self::Payload, input: semio_framework::action_bus::RetainedToolWireInput, checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
33023 +             fn create_job_from_wire_pages_with_payload(&mut self, _operation: semio_framework_job::Operation, mut payload: Self::Payload, input: semio_framework::action_bus::RetainedToolWireInput, checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<action_bus::RetainedToolWireInput>)> {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33030:14
      |
33030 |         impl crate::app::ArtifactOwnedToolJobFactory for KeyedTestFactory {
      |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33030 -         impl crate::app::ArtifactOwnedToolJobFactory for KeyedTestFactory {
33030 +         impl ArtifactOwnedToolJobFactory for KeyedTestFactory {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33034:52
      |
33034 | ...   const PUBLICATION_CONTRACTS: &'static [crate::app::ArtifactToolPublicationContract] = &[crate::app::ArtifactToolPublication...
      |                                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33034 -             const PUBLICATION_CONTRACTS: &'static [crate::app::ArtifactToolPublicationContract] = &[crate::app::ArtifactToolPublicationContract { tool_id: "compositeEdit", lanes: &[crate::app::ArtifactToolPublicationLane::Artifact] }];
33034 +             const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[crate::app::ArtifactToolPublicationContract { tool_id: "compositeEdit", lanes: &[crate::app::ArtifactToolPublicationLane::Artifact] }];
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33034:101
      |
33034 | ...pp::ArtifactToolPublicationContract] = &[crate::app::ArtifactToolPublicationContract { tool_id: "compositeEdit", lanes: &[crat...
      |                                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33034 -             const PUBLICATION_CONTRACTS: &'static [crate::app::ArtifactToolPublicationContract] = &[crate::app::ArtifactToolPublicationContract { tool_id: "compositeEdit", lanes: &[crate::app::ArtifactToolPublicationLane::Artifact] }];
33034 +             const PUBLICATION_CONTRACTS: &'static [crate::app::ArtifactToolPublicationContract] = &[ArtifactToolPublicationContract { tool_id: "compositeEdit", lanes: &[crate::app::ArtifactToolPublicationLane::Artifact] }];
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33034:182
      |
33034 | ... { tool_id: "compositeEdit", lanes: &[crate::app::ArtifactToolPublicationLane::Artifact] }];
      |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33034 -             const PUBLICATION_CONTRACTS: &'static [crate::app::ArtifactToolPublicationContract] = &[crate::app::ArtifactToolPublicationContract { tool_id: "compositeEdit", lanes: &[crate::app::ArtifactToolPublicationLane::Artifact] }];
33034 +             const PUBLICATION_CONTRACTS: &'static [crate::app::ArtifactToolPublicationContract] = &[crate::app::ArtifactToolPublicationContract { tool_id: "compositeEdit", lanes: &[ArtifactToolPublicationLane::Artifact] }];
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33053:30
      |
33053 |             type Transient = crate::app::NoTransient;
      |                              ^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33053 -             type Transient = crate::app::NoTransient;
33053 +             type Transient = NoTransient;
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33054:38
      |
33054 |             type TransientMutation = crate::app::NoTransientMutation;
      |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33054 -             type TransientMutation = crate::app::NoTransientMutation;
33054 +             type TransientMutation = NoTransientMutation;
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33059:27
      |
33059 | ...   contract: semio_framework::ToolExecutionContract::resumable(32_768, 4, 1, 4_096, 500, 1, 1), tools: ["compositeEdit"]
      |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33059 -                 contract: semio_framework::ToolExecutionContract::resumable(32_768, 4, 1, 4_096, 500, 1, 1), tools: ["compositeEdit"]
33059 +                 contract: ToolExecutionContract::resumable(32_768, 4, 1, 4_096, 500, 1, 1), tools: ["compositeEdit"]
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33061:59
      |
33061 |             fn register_tool_job_factories(registry: &mut crate::app::ArtifactToolFactoryRegistry<'_, Self>) -> Result<(), Fault> {
      |                                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33061 -             fn register_tool_job_factories(registry: &mut crate::app::ArtifactToolFactoryRegistry<'_, Self>) -> Result<(), Fault> {
33061 +             fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, Self>) -> Result<(), Fault> {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33062:65
      |
33062 | ...   registry.register(KeyedTestFactory { keys: vec![semio_framework::ToolFactoryKey::new(registry.controller_id(), "compositeEd...
      |                                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33062 -                 registry.register(KeyedTestFactory { keys: vec![semio_framework::ToolFactoryKey::new(registry.controller_id(), "compositeEdit")] })
33062 +                 registry.register(KeyedTestFactory { keys: vec![ToolFactoryKey::new(registry.controller_id(), "compositeEdit")] })
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33064:98
      |
33064 | ...   async fn build_tool_job(request: ArtifactOwnedToolJobRequest<Self>) -> Result<Option<semio_framework::ToolOperationSpec>, F...
      |                                                                                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33064 -             async fn build_tool_job(request: ArtifactOwnedToolJobRequest<Self>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
33064 +             async fn build_tool_job(request: ArtifactOwnedToolJobRequest<Self>) -> Result<Option<ToolOperationSpec>, Fault> {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33067:25
      |
33067 | ...   Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, job, req...
      |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33067 -                 Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, job, request.operation)))
33067 +                 Ok(Some(ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, job, request.operation)))
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33076:159
      |
33076 | ...f::PresenceMutation>>>> { Some(crate::app::mutation_fixture::no_state::presence_store_disposer()) }
      |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33076 -             fn build_presence_store_disposer() -> Option<Box<dyn ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> { Some(crate::app::mutation_fixture::no_state::presence_store_disposer()) }
33076 +             fn build_presence_store_disposer() -> Option<Box<dyn ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> { Some(mutation_fixture::no_state::presence_store_disposer()) }
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33077:163
      |
33077 | ...::TransientMutation>>>> { Some(crate::app::mutation_fixture::no_state::transient_store_disposer()) }
      |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33077 -             fn build_transient_store_disposer() -> Option<Box<dyn ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> { Some(crate::app::mutation_fixture::no_state::transient_store_disposer()) }
33077 +             fn build_transient_store_disposer() -> Option<Box<dyn ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> { Some(mutation_fixture::no_state::transient_store_disposer()) }
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33078:144
      |
33078 | ...y<Self::Presence>>> { Some(crate::app::mutation_fixture::no_state::presence_peer_retirement_factory()) }
      |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33078 -             fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> { Some(crate::app::mutation_fixture::no_state::presence_peer_retirement_factory()) }
33078 +             fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> { Some(mutation_fixture::no_state::presence_peer_retirement_factory()) }
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33079:150
      |
33079 | ...elf::Presence>>> { Some(crate::app::mutation_fixture::no_state::presence_local_root_retirement_factory()) }
      |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33079 -             fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> { Some(crate::app::mutation_fixture::no_state::presence_local_root_retirement_factory()) }
33079 +             fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> { Some(mutation_fixture::no_state::presence_local_root_retirement_factory()) }
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33080:152
      |
33080 | ...f::Transient>>> { Some(crate::app::mutation_fixture::no_state::transient_local_root_retirement_factory()) }
      |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33080 -             fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> { Some(crate::app::mutation_fixture::no_state::transient_local_root_retirement_factory()) }
33080 +             fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> { Some(mutation_fixture::no_state::transient_local_root_retirement_factory()) }
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33084:133
      |
33084 | ...View<'_, TestConfig>) -> UiAssemblyResult<semio_framework_ui_runtime::ComponentTree> { TestApp::<false>::render(body, doc, cfg...
      |                                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33084 -             async fn render(body: &str, doc: &ArtifactView<'_, TestSnapshot>, cfg: &ConfigView<'_, TestConfig>) -> UiAssemblyResult<semio_framework_ui_runtime::ComponentTree> { TestApp::<false>::render(body, doc, cfg).await }
33084 +             async fn render(body: &str, doc: &ArtifactView<'_, TestSnapshot>, cfg: &ConfigView<'_, TestConfig>) -> UiAssemblyResult<ComponentTree> { TestApp::<false>::render(body, doc, cfg).await }
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33107:138
      |
33107 | ...t_factory().expect("transient local factory"), crate::app::NoTransient::default());
      |                                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33107 -             close_root(<KeyedTestApp as ArtifactApp>::build_transient_local_root_retirement_factory().expect("transient local factory"), crate::app::NoTransient::default());
33107 +             close_root(<KeyedTestApp as ArtifactApp>::build_transient_local_root_retirement_factory().expect("transient local factory"), NoTransient::default());
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33120:57
      |
33120 | ...   let mut transient = store::TransientStore::<crate::app::NoTransient, crate::app::NoTransientMutation>::new(crate::app::NoTr...
      |                                                   ^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33120 -             let mut transient = store::TransientStore::<crate::app::NoTransient, crate::app::NoTransientMutation>::new(crate::app::NoTransient::default());
33120 +             let mut transient = store::TransientStore::<NoTransient, crate::app::NoTransientMutation>::new(crate::app::NoTransient::default());
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33120:82
      |
33120 | ...   let mut transient = store::TransientStore::<crate::app::NoTransient, crate::app::NoTransientMutation>::new(crate::app::NoTr...
      |                                                                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33120 -             let mut transient = store::TransientStore::<crate::app::NoTransient, crate::app::NoTransientMutation>::new(crate::app::NoTransient::default());
33120 +             let mut transient = store::TransientStore::<crate::app::NoTransient, NoTransientMutation>::new(crate::app::NoTransient::default());
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33120:120
      |
33120 | ...ansient, crate::app::NoTransientMutation>::new(crate::app::NoTransient::default());
      |                                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33120 -             let mut transient = store::TransientStore::<crate::app::NoTransient, crate::app::NoTransientMutation>::new(crate::app::NoTransient::default());
33120 +             let mut transient = store::TransientStore::<crate::app::NoTransient, crate::app::NoTransientMutation>::new(NoTransient::default());
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33139:52
      |
33139 |             transient = store::TransientStore::new(crate::app::NoTransient::default());
      |                                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33139 -             transient = store::TransientStore::new(crate::app::NoTransient::default());
33139 +             transient = store::TransientStore::new(NoTransient::default());
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33157:39
      |
33157 |                     .interactive_jobs(semio_framework::InteractiveJobClassification::Migrated)
      |                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33157 -                     .interactive_jobs(semio_framework::InteractiveJobClassification::Migrated)
33157 +                     .interactive_jobs(InteractiveJobClassification::Migrated)
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33165:13
      |
33165 |             crate::app::test_retained_keyed_dispatch::<KeyedTestApp>(
      |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33165 -             crate::app::test_retained_keyed_dispatch::<KeyedTestApp>(
33165 +             test_retained_keyed_dispatch::<KeyedTestApp>(
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33175:13
      |
33175 |             crate::app::test_retained_keyed_dispatch::<KeyedTestApp>(
      |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33175 -             crate::app::test_retained_keyed_dispatch::<KeyedTestApp>(
33175 +             test_retained_keyed_dispatch::<KeyedTestApp>(
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33185:26
      |
33185 | ...   let fixture: serde_json::Value = serde_json::from_str(include_str!("⚛️reactor/🧪️fixtures/📬️operation-continuation.json")).un...
      |                    ^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33185 -             let fixture: serde_json::Value = serde_json::from_str(include_str!("⚛️reactor/🧪️fixtures/📬️operation-continuation.json")).unwrap();
33185 +             let fixture: Value = serde_json::from_str(include_str!("⚛️reactor/🧪️fixtures/📬️operation-continuation.json")).unwrap();
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33193:71
      |
33193 |             let cell = std::sync::Arc::new(super::RuntimeAppCell::new(crate::app::AppInstance { id, app }));
      |                                                                       ^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33193 -             let cell = std::sync::Arc::new(super::RuntimeAppCell::new(crate::app::AppInstance { id, app }));
33193 +             let cell = std::sync::Arc::new(super::RuntimeAppCell::new(AppInstance { id, app }));
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33203:47
      |
33203 | ...   assert_ne!(page.lane, crate::app::TypedOperationResultLane::Fault, "{}", String::from_utf8_lossy(page.bytes()));
      |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33203 -                         assert_ne!(page.lane, crate::app::TypedOperationResultLane::Fault, "{}", String::from_utf8_lossy(page.bytes()));
33203 +                         assert_ne!(page.lane, TypedOperationResultLane::Fault, "{}", String::from_utf8_lossy(page.bytes()));
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33204:50
      |
33204 |                         terminal |= page.lane == crate::app::TypedOperationResultLane::Terminal;
      |                                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33204 -                         terminal |= page.lane == crate::app::TypedOperationResultLane::Terminal;
33204 +                         terminal |= page.lane == TypedOperationResultLane::Terminal;
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33264:30
      |
33264 |             type Transient = crate::app::NoTransient;
      |                              ^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33264 -             type Transient = crate::app::NoTransient;
33264 +             type Transient = NoTransient;
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33265:38
      |
33265 |             type TransientMutation = crate::app::NoTransientMutation;
      |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33265 -             type TransientMutation = crate::app::NoTransientMutation;
33265 +             type TransientMutation = NoTransientMutation;
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33287:137
      |
33287 | ...View<'_, TestConfig>) -> UiAssemblyResult<semio_framework_ui_runtime::ComponentTree> {
      |                                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33287 -             async fn render(body_key: &str, doc: &ArtifactView<'_, TestSnapshot>, cfg: &ConfigView<'_, TestConfig>) -> UiAssemblyResult<semio_framework_ui_runtime::ComponentTree> {
33287 +             async fn render(body_key: &str, doc: &ArtifactView<'_, TestSnapshot>, cfg: &ConfigView<'_, TestConfig>) -> UiAssemblyResult<ComponentTree> {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33337:41
      |
33337 |                 .await.interactive_jobs(semio_framework::InteractiveJobClassification::BatchOnlyPendingRewrite).await,
      |                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33337 -                 .await.interactive_jobs(semio_framework::InteractiveJobClassification::BatchOnlyPendingRewrite).await,
33337 +                 .await.interactive_jobs(InteractiveJobClassification::BatchOnlyPendingRewrite).await,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33349:28
      |
33349 |             let platform = semio_framework::Platform::new(None).await;
      |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33349 -             let platform = semio_framework::Platform::new(None).await;
33349 +             let platform = Platform::new(None).await;
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33374:28
      |
33374 |             let platform = semio_framework::Platform::new(None).await;
      |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33374 -             let platform = semio_framework::Platform::new(None).await;
33374 +             let platform = Platform::new(None).await;
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33408:34
      |
33408 |                 assert_eq!(key, &semio_framework::ToolFactoryKey::new(&controller_id, tool_id));
      |                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33408 -                 assert_eq!(key, &semio_framework::ToolFactoryKey::new(&controller_id, tool_id));
33408 +                 assert_eq!(key, &ToolFactoryKey::new(&controller_id, tool_id));
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33467:13
      |
33467 | ...   crate::app::test_retained_factory_proof_join::<TestApp, TestRetainedCommandFactory, OtherTestRetainedCommandFactory, CopyDrawApp>(co...
      |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33467 -             crate::app::test_retained_factory_proof_join::<TestApp, TestRetainedCommandFactory, OtherTestRetainedCommandFactory, CopyDrawApp>(contract_registry().await, TEST_RETAINED_COMMAND_CONTROLLER, TEST_RETAINED_COMMAND_TOOL, TestRetainedCommandFactory::new());
33467 +             test_retained_factory_proof_join::<TestApp, TestRetainedCommandFactory, OtherTestRetainedCommandFactory, CopyDrawApp>(contract_registry().await, TEST_RETAINED_COMMAND_CONTROLLER, TEST_RETAINED_COMMAND_TOOL, TestRetainedCommandFactory::new());
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33472:13
      |
33472 |             crate::app::test_retained_cancellation_publication_boundaries::<TestApp>().await;
      |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33472 -             crate::app::test_retained_cancellation_publication_boundaries::<TestApp>().await;
33472 +             test_retained_cancellation_publication_boundaries::<TestApp>().await;
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33477:13
      |
33477 |             crate::app::test_retained_latest_wins_slot_and_publication_fairness::<TestApp>().await;
      |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33477 -             crate::app::test_retained_latest_wins_slot_and_publication_fairness::<TestApp>().await;
33477 +             test_retained_latest_wins_slot_and_publication_fairness::<TestApp>().await;
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33487:13
      |
33487 | ...   crate::app::test_retained_document_cancellation::<TestApp>(&TestCountOneItemPreparationFactory, || TestMutation::SetCount(S...
      |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33487 -             crate::app::test_retained_document_cancellation::<TestApp>(&TestCountOneItemPreparationFactory, || TestMutation::SetCount(SetCount { value: 42 }), |snapshot| snapshot.count).await;
33487 +             test_retained_document_cancellation::<TestApp>(&TestCountOneItemPreparationFactory, || TestMutation::SetCount(SetCount { value: 42 }), |snapshot| snapshot.count).await;
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33494:63
      |
33494 |             declaration.semantics.execution.interactive_job = semio_framework::InteractiveJobClassification::Migrated;
      |                                                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33494 -             declaration.semantics.execution.interactive_job = semio_framework::InteractiveJobClassification::Migrated;
33494 +             declaration.semantics.execution.interactive_job = InteractiveJobClassification::Migrated;
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33497:21
      |
33497 |             assert!(crate::app::test_unregistered_tool_job_admission_rejected::<CopyDrawApp>(&owner, &["canvasPointerDown"]));
      |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33497 -             assert!(crate::app::test_unregistered_tool_job_admission_rejected::<CopyDrawApp>(&owner, &["canvasPointerDown"]));
33497 +             assert!(test_unregistered_tool_job_admission_rejected::<CopyDrawApp>(&owner, &["canvasPointerDown"]));
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33505:17
      |
33505 |                 semio_framework::ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
      |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33505 -                 semio_framework::ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
33505 +                 ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33822:23
      |
33822 |             let bus = semio_framework::ActionBus::new();
      |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33822 -             let bus = semio_framework::ActionBus::new();
33822 +             let bus = ActionBus::new();
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33827:39
      |
33827 |             let original_completion = crate::app::ArtifactToolCompletion::<TestApp>::new();
      |                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33827 -             let original_completion = crate::app::ArtifactToolCompletion::<TestApp>::new();
33827 +             let original_completion = ArtifactToolCompletion::<TestApp>::new();
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33830:33
      |
33830 | ...   let original_spec = semio_framework::ToolOperationSpec::new(TEST_RETAINED_COMMAND_CONTROLLER, TEST_RETAINED_COMMAND_TOOL, T...
      |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33830 -             let original_spec = semio_framework::ToolOperationSpec::new(TEST_RETAINED_COMMAND_CONTROLLER, TEST_RETAINED_COMMAND_TOOL, TEST_RETAINED_COMMAND_SCHEMA, original_payload, operation);
33830 +             let original_spec = ToolOperationSpec::new(TEST_RETAINED_COMMAND_CONTROLLER, TEST_RETAINED_COMMAND_TOOL, TEST_RETAINED_COMMAND_SCHEMA, original_payload, operation);
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33873:38
      |
33873 |             let resumed_completion = crate::app::ArtifactToolCompletion::<TestApp>::new();
      |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33873 -             let resumed_completion = crate::app::ArtifactToolCompletion::<TestApp>::new();
33873 +             let resumed_completion = ArtifactToolCompletion::<TestApp>::new();
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33876:32
      |
33876 | ...   let resumed_spec = semio_framework::ToolOperationSpec::new(TEST_RETAINED_COMMAND_CONTROLLER, TEST_RETAINED_COMMAND_TOOL, TE...
      |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33876 -             let resumed_spec = semio_framework::ToolOperationSpec::new(TEST_RETAINED_COMMAND_CONTROLLER, TEST_RETAINED_COMMAND_TOOL, TEST_RETAINED_COMMAND_SCHEMA, resumed_payload, operation);
33876 +             let resumed_spec = ToolOperationSpec::new(TEST_RETAINED_COMMAND_CONTROLLER, TEST_RETAINED_COMMAND_TOOL, TEST_RETAINED_COMMAND_SCHEMA, resumed_payload, operation);
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33916:40
      |
33916 |             let cancelled_completion = crate::app::ArtifactToolCompletion::<TestApp>::new();
      |                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33916 -             let cancelled_completion = crate::app::ArtifactToolCompletion::<TestApp>::new();
33916 +             let cancelled_completion = ArtifactToolCompletion::<TestApp>::new();
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33919:34
      |
33919 | ...   let cancelled_spec = semio_framework::ToolOperationSpec::new(TEST_RETAINED_COMMAND_CONTROLLER, TEST_RETAINED_COMMAND_TOOL, ...
      |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33919 -             let cancelled_spec = semio_framework::ToolOperationSpec::new(TEST_RETAINED_COMMAND_CONTROLLER, TEST_RETAINED_COMMAND_TOOL, TEST_RETAINED_COMMAND_SCHEMA, cancelled_payload, operation);
33919 +             let cancelled_spec = ToolOperationSpec::new(TEST_RETAINED_COMMAND_CONTROLLER, TEST_RETAINED_COMMAND_TOOL, TEST_RETAINED_COMMAND_SCHEMA, cancelled_payload, operation);
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33960:34
      |
33960 |                     .interaction(semio_framework::InteractionDefinition {
      |                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33960 -                     .interaction(semio_framework::InteractionDefinition {
33960 +                     .interaction(InteractionDefinition {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33963:45
      |
33963 | ...   granularities: vec![semio_framework::GranularityDefinition { id: "item".into(), label: LocalizedLabel::data("Item"), icon_i...
      |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33963 -                         granularities: vec![semio_framework::GranularityDefinition { id: "item".into(), label: LocalizedLabel::data("Item"), icon_id: IconName::AppWindow }],
33963 +                         granularities: vec![GranularityDefinition { id: "item".into(), label: LocalizedLabel::data("Item"), icon_id: IconName::AppWindow }],
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33964:36
      |
33964 |                         hierarchy: protocol::HierarchyProvider::Topology,
      |                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33964 -                         hierarchy: protocol::HierarchyProvider::Topology,
33964 +                         hierarchy: HierarchyProvider::Topology,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33965:32
      |
33965 |                         hover: protocol::HoverSpec::default(),
      |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33965 -                         hover: protocol::HoverSpec::default(),
33965 +                         hover: HoverSpec::default(),
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33966:36
      |
33966 |                         selection: protocol::SelectionSpec {
      |                                    ^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33966 -                         selection: protocol::SelectionSpec {
33966 +                         selection: SelectionSpec {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33967:41
      |
33967 | ...                   modes: vec![protocol::SelectionMode::Multiple, protocol::SelectionMode::Single],
      |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33967 -                             modes: vec![protocol::SelectionMode::Multiple, protocol::SelectionMode::Single],
33967 +                             modes: vec![SelectionMode::Multiple, protocol::SelectionMode::Single],
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33967:76
      |
33967 | ...                   modes: vec![protocol::SelectionMode::Multiple, protocol::SelectionMode::Single],
      |                                                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33967 -                             modes: vec![protocol::SelectionMode::Multiple, protocol::SelectionMode::Single],
33967 +                             modes: vec![protocol::SelectionMode::Multiple, SelectionMode::Single],
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33968:43
      |
33968 | ...                   methods: vec![protocol::SelectionMethod::Pick],
      |                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33968 -                             methods: vec![protocol::SelectionMethod::Pick],
33968 +                             methods: vec![SelectionMethod::Pick],
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33969:42
      |
33969 | ...   merges: vec![protocol::MergeMode::Replace, protocol::MergeMode::Additive, protocol::MergeMode::Subtractive, protocol::Merge...
      |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33969 -                             merges: vec![protocol::MergeMode::Replace, protocol::MergeMode::Additive, protocol::MergeMode::Subtractive, protocol::MergeMode::Invertive, protocol::MergeMode::Range],
33969 +                             merges: vec![MergeMode::Replace, protocol::MergeMode::Additive, protocol::MergeMode::Subtractive, protocol::MergeMode::Invertive, protocol::MergeMode::Range],
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33969:72
      |
33969 | ...   merges: vec![protocol::MergeMode::Replace, protocol::MergeMode::Additive, protocol::MergeMode::Subtractive, protocol::Merge...
      |                                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33969 -                             merges: vec![protocol::MergeMode::Replace, protocol::MergeMode::Additive, protocol::MergeMode::Subtractive, protocol::MergeMode::Invertive, protocol::MergeMode::Range],
33969 +                             merges: vec![protocol::MergeMode::Replace, MergeMode::Additive, protocol::MergeMode::Subtractive, protocol::MergeMode::Invertive, protocol::MergeMode::Range],
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33969:103
      |
33969 | ...   merges: vec![protocol::MergeMode::Replace, protocol::MergeMode::Additive, protocol::MergeMode::Subtractive, protocol::Merge...
      |                                                                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33969 -                             merges: vec![protocol::MergeMode::Replace, protocol::MergeMode::Additive, protocol::MergeMode::Subtractive, protocol::MergeMode::Invertive, protocol::MergeMode::Range],
33969 +                             merges: vec![protocol::MergeMode::Replace, protocol::MergeMode::Additive, MergeMode::Subtractive, protocol::MergeMode::Invertive, protocol::MergeMode::Range],
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33969:137
      |
33969 | ...de::Additive, protocol::MergeMode::Subtractive, protocol::MergeMode::Invertive, protocol::MergeMode::Range],
      |                                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33969 -                             merges: vec![protocol::MergeMode::Replace, protocol::MergeMode::Additive, protocol::MergeMode::Subtractive, protocol::MergeMode::Invertive, protocol::MergeMode::Range],
33969 +                             merges: vec![protocol::MergeMode::Replace, protocol::MergeMode::Additive, protocol::MergeMode::Subtractive, MergeMode::Invertive, protocol::MergeMode::Range],
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33969:169
      |
33969 | ...ode::Subtractive, protocol::MergeMode::Invertive, protocol::MergeMode::Range],
      |                                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33969 -                             merges: vec![protocol::MergeMode::Replace, protocol::MergeMode::Additive, protocol::MergeMode::Subtractive, protocol::MergeMode::Invertive, protocol::MergeMode::Range],
33969 +                             merges: vec![protocol::MergeMode::Replace, protocol::MergeMode::Additive, protocol::MergeMode::Subtractive, protocol::MergeMode::Invertive, MergeMode::Range],
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33975:60
      |
33975 |                     .window_kind_interactions("main", vec![semio_framework::InteractionRef::new("items")])
      |                                                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33975 -                     .window_kind_interactions("main", vec![semio_framework::InteractionRef::new("items")])
33975 +                     .window_kind_interactions("main", vec![InteractionRef::new("items")])
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33989:43
      |
33989 |         fn interaction_target_args(extra: serde_json::Value, id: &str) -> serde_json::Value {
      |                                           ^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33989 -         fn interaction_target_args(extra: serde_json::Value, id: &str) -> serde_json::Value {
33989 +         fn interaction_target_args(extra: Value, id: &str) -> serde_json::Value {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33989:75
      |
33989 |         fn interaction_target_args(extra: serde_json::Value, id: &str) -> serde_json::Value {
      |                                                                           ^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33989 -         fn interaction_target_args(extra: serde_json::Value, id: &str) -> serde_json::Value {
33989 +         fn interaction_target_args(extra: serde_json::Value, id: &str) -> Value {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33990:55
      |
33990 | ...   let targets = serde_json::to_string(&vec![protocol::InteractionTarget { granularity: "item".into(), id: id.into() }]).expec...
      |                                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33990 -             let targets = serde_json::to_string(&vec![protocol::InteractionTarget { granularity: "item".into(), id: id.into() }]).expect("targets serialize");
33990 +             let targets = serde_json::to_string(&vec![InteractionTarget { granularity: "item".into(), id: id.into() }]).expect("targets serialize");
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33996:52
      |
33996 |         async fn __semio_plugin_bundle() -> Result<crate::Plugin<TestRuntimeApps>, crate::PluginAssemblyError> {
      |                                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33996 -         async fn __semio_plugin_bundle() -> Result<crate::Plugin<TestRuntimeApps>, crate::PluginAssemblyError> {
33996 +         async fn __semio_plugin_bundle() -> Result<Plugin<TestRuntimeApps>, crate::PluginAssemblyError> {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33996:84
      |
33996 |         async fn __semio_plugin_bundle() -> Result<crate::Plugin<TestRuntimeApps>, crate::PluginAssemblyError> {
      |                                                                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33996 -         async fn __semio_plugin_bundle() -> Result<crate::Plugin<TestRuntimeApps>, crate::PluginAssemblyError> {
33996 +         async fn __semio_plugin_bundle() -> Result<crate::Plugin<TestRuntimeApps>, PluginAssemblyError> {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:33997:13
      |
33997 | ...   crate::Plugin::<TestRuntimeApps>::builder("synthetic").label("Synthetic").version("0.0.1").document_app::<TestApp>(syntheti...
      |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
33997 -             crate::Plugin::<TestRuntimeApps>::builder("synthetic").label("Synthetic").version("0.0.1").document_app::<TestApp>(synthetic_play_app().await).document_app_mutation_roster::<TestApp>().try_build()
33997 +             Plugin::<TestRuntimeApps>::builder("synthetic").label("Synthetic").version("0.0.1").document_app::<TestApp>(synthetic_play_app().await).document_app_mutation_roster::<TestApp>().try_build()
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34045:29
      |
34045 |             let timestamp = protocol::HybridLogicalTimestamp::new(1, u64::MAX);
      |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34045 -             let timestamp = protocol::HybridLogicalTimestamp::new(1, u64::MAX);
34045 +             let timestamp = HybridLogicalTimestamp::new(1, u64::MAX);
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34046:35
      |
34046 | ...   let mutation_ids: Vec<protocol::MutationId> = envelope.vcs.edits.iter().flat_map(|edit| edit.mutation_meta.iter().filter_ma...
      |                             ^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34046 -             let mutation_ids: Vec<protocol::MutationId> = envelope.vcs.edits.iter().flat_map(|edit| edit.mutation_meta.iter().filter_map(|meta| meta.mutation_id.clone())).collect();
34046 +             let mutation_ids: Vec<MutationId> = envelope.vcs.edits.iter().flat_map(|edit| edit.mutation_meta.iter().filter_map(|meta| meta.mutation_id.clone())).collect();
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34047:65
      |
34047 | ...   let conflict_id = protocol::ConflictId::new(&kind, &protocol::ArtifactId(envelope.id.clone()), &mutation_ids, &timestamp).a...
      |                                                           ^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34047 -             let conflict_id = protocol::ConflictId::new(&kind, &protocol::ArtifactId(envelope.id.clone()), &mutation_ids, &timestamp).await;
34047 +             let conflict_id = protocol::ConflictId::new(&kind, &ArtifactId(envelope.id.clone()), &mutation_ids, &timestamp).await;
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34053:30
      |
34053 |                 actors: vec![protocol::ActorId("local".into())],
      |                              ^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34053 -                 actors: vec![protocol::ActorId("local".into())],
34053 +                 actors: vec![ActorId("local".into())],
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34140:31
      |
34140 | ...   app.handle_action(semio_framework::INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items",...
      |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34140 -             app.handle_action(semio_framework::INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "item-1")), &meta()).await.expect("interactionSelect");
34140 +             app.handle_action(INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "item-1")), &meta()).await.expect("interactionSelect");
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34154:85
      |
34154 |         fn sample_presence_peer(actor: &str, color: Option<u8>, with_pack: bool) -> protocol::PresencePeer {
      |                                                                                     ^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34154 -         fn sample_presence_peer(actor: &str, color: Option<u8>, with_pack: bool) -> protocol::PresencePeer {
34154 +         fn sample_presence_peer(actor: &str, color: Option<u8>, with_pack: bool) -> PresencePeer {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34155:13
      |
34155 |             protocol::PresencePeer {
      |             ^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34155 -             protocol::PresencePeer {
34155 +             PresencePeer {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34171:119
      |
34171 | ...TestApp>, seq: u64, own_color: Option<u8>, peers: &[protocol::PresencePeer], now_ms: i64) -> PresenceRosterOutcome {
      |                                                        ^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34171 -         async fn publish_presence_roster(app: &mut VcsArtifactApp<TestApp>, seq: u64, own_color: Option<u8>, peers: &[protocol::PresencePeer], now_ms: i64) -> PresenceRosterOutcome {
34171 +         async fn publish_presence_roster(app: &mut VcsArtifactApp<TestApp>, seq: u64, own_color: Option<u8>, peers: &[PresencePeer], now_ms: i64) -> PresenceRosterOutcome {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34172:64
      |
34172 |             let roster = peers.iter().map(|peer| resolve_ready(protocol::encode_presence_peer(peer))).collect::<Vec<_>>();
      |                                                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34172 -             let roster = peers.iter().map(|peer| resolve_ready(protocol::encode_presence_peer(peer))).collect::<Vec<_>>();
34172 +             let roster = peers.iter().map(|peer| resolve_ready(encode_presence_peer(peer))).collect::<Vec<_>>();
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34175:107
      |
34175 | ...q, own_color, roster.len() as u32, semio_framework::kernel::FixedCommandPage::try_copy_from(&first).expect("test peer page is ...
      |                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34175 -             let cursor = protocol::PresenceCommandCursor::admit_page(seq, own_color, roster.len() as u32, semio_framework::kernel::FixedCommandPage::try_copy_from(&first).expect("test peer page is fixed-authority"))
34175 +             let cursor = protocol::PresenceCommandCursor::admit_page(seq, own_color, roster.len() as u32, FixedCommandPage::try_copy_from(&first).expect("test peer page is fixed-authority"))
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34185:32
      |
34185 | ...   let page = semio_framework::kernel::FixedCommandPage::try_copy_from(roster.iter().nth(next_page).expect("retained roster pa...
      |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34185 -                     let page = semio_framework::kernel::FixedCommandPage::try_copy_from(roster.iter().nth(next_page).expect("retained roster page")).expect("test peer page is fixed-authority");
34185 +                     let page = FixedCommandPage::try_copy_from(roster.iter().nth(next_page).expect("retained roster page")).expect("test peer page is fixed-authority");
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34270:88
      |
34270 | ...ndCursor::admit_page(seq, None, 0, semio_framework::kernel::FixedCommandPage::try_copy_from(&[]).expect("empty fixed page")).m...
      |                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34270 -                 let cursor = protocol::PresenceCommandCursor::admit_page(seq, None, 0, semio_framework::kernel::FixedCommandPage::try_copy_from(&[]).expect("empty fixed page")).map_err(|(error, _)| error).expect("empty roster cursor");
34270 +                 let cursor = protocol::PresenceCommandCursor::admit_page(seq, None, 0, FixedCommandPage::try_copy_from(&[]).expect("empty fixed page")).map_err(|(error, _)| error).expect("empty roster cursor");
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34293:24
      |
34293 |             let page = semio_framework::kernel::FixedCommandPage::try_copy_from(&[0xA5; 17]).expect("fixed retained peer page");
      |                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34293 -             let page = semio_framework::kernel::FixedCommandPage::try_copy_from(&[0xA5; 17]).expect("fixed retained peer page");
34293 +             let page = FixedCommandPage::try_copy_from(&[0xA5; 17]).expect("fixed retained peer page");
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34319:82
      |
34319 | ...mandCursor::admit_page(9, None, 0, semio_framework::kernel::FixedCommandPage::try_copy_from(&[]).expect("empty fixed page")).m...
      |                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34319 -             let cursor = protocol::PresenceCommandCursor::admit_page(9, None, 0, semio_framework::kernel::FixedCommandPage::try_copy_from(&[]).expect("empty fixed page")).map_err(|(error, _)| error).expect("stale empty roster cursor");
34319 +             let cursor = protocol::PresenceCommandCursor::admit_page(9, None, 0, FixedCommandPage::try_copy_from(&[]).expect("empty fixed page")).map_err(|(error, _)| error).expect("stale empty roster cursor");
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34341:22
      |
34341 |                 Some(protocol::PresenceInteraction {
      |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34341 -                 Some(protocol::PresenceInteraction {
34341 +                 Some(PresenceInteraction {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34343:35
      |
34343 | ...   domains: vec![protocol::PresenceDomain { domain: "items".to_string(), granularity: "item".to_string(), selected: selected.i...
      |                     ^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34343 -                     domains: vec![protocol::PresenceDomain { domain: "items".to_string(), granularity: "item".to_string(), selected: selected.iter().map(|id| id.to_string()).collect(), hovered: hovered.iter().map(|id| id.to_string()).collect() }],
34343 +                     domains: vec![PresenceDomain { domain: "items".to_string(), granularity: "item".to_string(), selected: selected.iter().map(|id| id.to_string()).collect(), hovered: hovered.iter().map(|id| id.to_string()).collect() }],
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34350:25
      |
34350 |             let state = protocol::InteractionState::default();
      |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34350 -             let state = protocol::InteractionState::default();
34350 +             let state = InteractionState::default();
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34479:42
      |
34479 |         async fn test_child_dialect() -> store::os_io::ArtifactDialect {
      |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34479 -         async fn test_child_dialect() -> store::os_io::ArtifactDialect {
34479 +         async fn test_child_dialect() -> ArtifactDialect {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34480:13
      |
34480 |             store::os_io::ArtifactDialect { artifact_kind: "s.test.child".into(), standard: "native".into(), subset: "*".into() }
      |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34480 -             store::os_io::ArtifactDialect { artifact_kind: "s.test.child".into(), standard: "native".into(), subset: "*".into() }
34480 +             ArtifactDialect { artifact_kind: "s.test.child".into(), standard: "native".into(), subset: "*".into() }
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34495:32
      |
34495 |             let child_handle = crate::app::artifact_handle_of("child-1").await;
      |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34495 -             let child_handle = crate::app::artifact_handle_of("child-1").await;
34495 +             let child_handle = artifact_handle_of("child-1").await;
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34538:31
      |
34538 |                 let dialect = store::os_io::ArtifactDialect::parse_coordinate(&entry.dialect).expect("dialect round trips");
      |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34538 -                 let dialect = store::os_io::ArtifactDialect::parse_coordinate(&entry.dialect).expect("dialect round trips");
34538 +                 let dialect = ArtifactDialect::parse_coordinate(&entry.dialect).expect("dialect round trips");
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:34779:125
      |
34779 | ... { action_id: "os.setThemeId".into(), args: semio_framework::optional_json_to_dsl(Some(json!({ "themeId": "light" }))) }]);
      |                                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
34779 -             assert_eq!(result.requested_effects, vec![Effect::ReplayShellCommand { action_id: "os.setThemeId".into(), args: semio_framework::optional_json_to_dsl(Some(json!({ "themeId": "light" }))) }]);
34779 +             assert_eq!(result.requested_effects, vec![Effect::ReplayShellCommand { action_id: "os.setThemeId".into(), args: optional_json_to_dsl(Some(json!({ "themeId": "light" }))) }]);
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35030:17
      |
35030 | ...   let semio_framework_ui_contract::Component::TreeSection(actions_props) = &all_panel.children[0].component else { panic!("ex...
      |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35030 -             let semio_framework_ui_contract::Component::TreeSection(actions_props) = &all_panel.children[0].component else { panic!("expected a TreeSection") };
35030 +             let Component::TreeSection(actions_props) = &all_panel.children[0].component else { panic!("expected a TreeSection") };
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35034:17
      |
35034 | ...   let semio_framework_ui_contract::Component::TreeSection(commands_props) = &all_panel.children[1].component else { panic!("e...
      |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35034 -             let semio_framework_ui_contract::Component::TreeSection(commands_props) = &all_panel.children[1].component else { panic!("expected a TreeSection") };
35034 +             let Component::TreeSection(commands_props) = &all_panel.children[1].component else { panic!("expected a TreeSection") };
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35037:17
      |
35037 | ...   let semio_framework_ui_contract::Component::TreeItem(revertible_props) = &all_panel.children[1].children[0].component else ...
      |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35037 -             let semio_framework_ui_contract::Component::TreeItem(revertible_props) = &all_panel.children[1].children[0].component else { panic!("expected a TreeItem") };
35037 +             let Component::TreeItem(revertible_props) = &all_panel.children[1].children[0].component else { panic!("expected a TreeItem") };
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35039:17
      |
35039 | ...   let semio_framework_ui_contract::Component::TreeItem(non_revertible_props) = &all_panel.children[1].children[1].component e...
      |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35039 -             let semio_framework_ui_contract::Component::TreeItem(non_revertible_props) = &all_panel.children[1].children[1].component else { panic!("expected a TreeItem") };
35039 +             let Component::TreeItem(non_revertible_props) = &all_panel.children[1].children[1].component else { panic!("expected a TreeItem") };
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35445:39
      |
35445 |             assert_eq!(event.payload, dsl::to_dsl_value(&json!({ "utilityId": "brush" })).unwrap());
      |                                       ^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35445 -             assert_eq!(event.payload, dsl::to_dsl_value(&json!({ "utilityId": "brush" })).unwrap());
35445 +             assert_eq!(event.payload, to_dsl_value(&json!({ "utilityId": "brush" })).unwrap());
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35576:25
      |
35576 |             let plugin: crate::Plugin = crate::Plugin::new("fixture", "Fixture", "0.1.0").plugin_command(
      |                         ^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35576 -             let plugin: crate::Plugin = crate::Plugin::new("fixture", "Fixture", "0.1.0").plugin_command(
35576 +             let plugin: Plugin = crate::Plugin::new("fixture", "Fixture", "0.1.0").plugin_command(
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35576:41
      |
35576 |             let plugin: crate::Plugin = crate::Plugin::new("fixture", "Fixture", "0.1.0").plugin_command(
      |                                         ^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35576 -             let plugin: crate::Plugin = crate::Plugin::new("fixture", "Fixture", "0.1.0").plugin_command(
35576 +             let plugin: crate::Plugin = Plugin::new("fixture", "Fixture", "0.1.0").plugin_command(
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35581:33
      |
35581 |                         output: dsl::DslValue::Null,
      |                                 ^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35581 -                         output: dsl::DslValue::Null,
35581 +                         output: DslValue::Null,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35622:31
      |
35622 | ...   app.handle_action(semio_framework::INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items",...
      |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35622 -             app.handle_action(semio_framework::INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "item-1")), &meta()).await.expect("interactionSelect");
35622 +             app.handle_action(INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "item-1")), &meta()).await.expect("interactionSelect");
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35632:31
      |
35632 | ...   app.handle_action(semio_framework::INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items",...
      |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35632 -             app.handle_action(semio_framework::INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "item-1")), &meta()).await.expect("interactionSelect");
35632 +             app.handle_action(INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "item-1")), &meta()).await.expect("interactionSelect");
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35635:31
      |
35635 | ...   app.handle_action(semio_framework::INTERACTION_HOVER_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", ...
      |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35635 -             app.handle_action(semio_framework::INTERACTION_HOVER_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "channel": "pointer" }), "item-1")), &meta()).await.expect("interactionHover");
35635 +             app.handle_action(INTERACTION_HOVER_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "channel": "pointer" }), "item-1")), &meta()).await.expect("interactionHover");
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35641:31
      |
35641 | ...   app.handle_action(semio_framework::INTERACTION_HOVER_ACTION_ID, Some(&json!({ "domainId": "items", "channel": "pointer", "t...
      |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35641 -             app.handle_action(semio_framework::INTERACTION_HOVER_ACTION_ID, Some(&json!({ "domainId": "items", "channel": "pointer", "targets": "[]" })), &meta()).await.expect("clear hover");
35641 +             app.handle_action(INTERACTION_HOVER_ACTION_ID, Some(&json!({ "domainId": "items", "channel": "pointer", "targets": "[]" })), &meta()).await.expect("clear hover");
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35649:31
      |
35649 | ...   app.handle_action(semio_framework::INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items",...
      |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35649 -             app.handle_action(semio_framework::INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "item-1")), &meta()).await.expect("interactionSelect");
35649 +             app.handle_action(INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "item-1")), &meta()).await.expect("interactionSelect");
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35662:31
      |
35662 | ...   app.handle_action(semio_framework::SET_SELECTION_MODE_ACTION_ID, Some(&json!({ "domainId": "items", "mode": "single" })), &...
      |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35662 -             app.handle_action(semio_framework::SET_SELECTION_MODE_ACTION_ID, Some(&json!({ "domainId": "items", "mode": "single" })), &meta()).await.expect("setSelectionMode");
35662 +             app.handle_action(SET_SELECTION_MODE_ACTION_ID, Some(&json!({ "domainId": "items", "mode": "single" })), &meta()).await.expect("setSelectionMode");
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35663:94
      |
35663 |             assert_eq!(app.interaction_state().await.active_mode.get("items").copied(), Some(protocol::SelectionMode::Single));
      |                                                                                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35663 -             assert_eq!(app.interaction_state().await.active_mode.get("items").copied(), Some(protocol::SelectionMode::Single));
35663 +             assert_eq!(app.interaction_state().await.active_mode.get("items").copied(), Some(SelectionMode::Single));
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35665:31
      |
35665 | ...   app.handle_action(semio_framework::SET_INTERACTION_GRANULARITY_ACTION_ID, Some(&json!({ "domainId": "items", "granularityId...
      |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35665 -             app.handle_action(semio_framework::SET_INTERACTION_GRANULARITY_ACTION_ID, Some(&json!({ "domainId": "items", "granularityId": "item" })), &meta()).await.expect("setInteractionGranularity");
35665 +             app.handle_action(SET_INTERACTION_GRANULARITY_ACTION_ID, Some(&json!({ "domainId": "items", "granularityId": "item" })), &meta()).await.expect("setInteractionGranularity");
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35669:43
      |
35669 | ...   let error = app.handle_action(semio_framework::SET_INTERACTION_GRANULARITY_ACTION_ID, Some(&json!({ "domainId": "items", "g...
      |                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35669 -             let error = app.handle_action(semio_framework::SET_INTERACTION_GRANULARITY_ACTION_ID, Some(&json!({ "domainId": "items", "granularityId": "bogus" })), &meta()).await.expect_err("undeclared granularity must be rejected");
35669 +             let error = app.handle_action(SET_INTERACTION_GRANULARITY_ACTION_ID, Some(&json!({ "domainId": "items", "granularityId": "bogus" })), &meta()).await.expect_err("undeclared granularity must be rejected");
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35678:31
      |
35678 |             app.handle_action(semio_framework::SELECT_ALL_ACTION_ID, None, &meta()).await.expect("selectAll");
      |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35678 -             app.handle_action(semio_framework::SELECT_ALL_ACTION_ID, None, &meta()).await.expect("selectAll");
35678 +             app.handle_action(SELECT_ALL_ACTION_ID, None, &meta()).await.expect("selectAll");
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35681:31
      |
35681 |             app.handle_action(semio_framework::CLEAR_SELECTION_ACTION_ID, None, &meta()).await.expect("clearSelection");
      |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35681 -             app.handle_action(semio_framework::CLEAR_SELECTION_ACTION_ID, None, &meta()).await.expect("clearSelection");
35681 +             app.handle_action(CLEAR_SELECTION_ACTION_ID, None, &meta()).await.expect("clearSelection");
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35689:31
      |
35689 | ...   app.handle_action(semio_framework::INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items",...
      |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35689 -             app.handle_action(semio_framework::INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "item-1")), &meta()).await.expect("interactionSelect");
35689 +             app.handle_action(INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "item-1")), &meta()).await.expect("interactionSelect");
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35703:31
      |
35703 | ...   app.handle_action(semio_framework::INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items",...
      |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35703 -             app.handle_action(semio_framework::INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "item-1")), &meta()).await.expect("interactionSelect");
35703 +             app.handle_action(INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "item-1")), &meta()).await.expect("interactionSelect");
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35706:39
      |
35706 |             assert_eq!(row.action_id, semio_framework::INTERACTION_SELECT_ACTION_ID);
      |                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35706 -             assert_eq!(row.action_id, semio_framework::INTERACTION_SELECT_ACTION_ID);
35706 +             assert_eq!(row.action_id, INTERACTION_SELECT_ACTION_ID);
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35721:64
      |
35721 |                 let builder = resolve_ready(__base.interaction(semio_framework::InteractionDefinition {
      |                                                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35721 -                 let builder = resolve_ready(__base.interaction(semio_framework::InteractionDefinition {
35721 +                 let builder = resolve_ready(__base.interaction(InteractionDefinition {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35724:41
      |
35724 | ...   granularities: vec![semio_framework::GranularityDefinition { id: "item".into(), label: LocalizedLabel::data("Item"), icon_i...
      |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35724 -                     granularities: vec![semio_framework::GranularityDefinition { id: "item".into(), label: LocalizedLabel::data("Item"), icon_id: IconName::AppWindow }],
35724 +                     granularities: vec![GranularityDefinition { id: "item".into(), label: LocalizedLabel::data("Item"), icon_id: IconName::AppWindow }],
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35725:32
      |
35725 |                     hierarchy: protocol::HierarchyProvider::Flat,
      |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35725 -                     hierarchy: protocol::HierarchyProvider::Flat,
35725 +                     hierarchy: HierarchyProvider::Flat,
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35726:28
      |
35726 |                     hover: protocol::HoverSpec { transitive: true, ..protocol::HoverSpec::default() },
      |                            ^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35726 -                     hover: protocol::HoverSpec { transitive: true, ..protocol::HoverSpec::default() },
35726 +                     hover: HoverSpec { transitive: true, ..protocol::HoverSpec::default() },
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35726:70
      |
35726 |                     hover: protocol::HoverSpec { transitive: true, ..protocol::HoverSpec::default() },
      |                                                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35726 -                     hover: protocol::HoverSpec { transitive: true, ..protocol::HoverSpec::default() },
35726 +                     hover: protocol::HoverSpec { transitive: true, ..HoverSpec::default() },
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35727:32
      |
35727 | ...   selection: protocol::SelectionSpec { modes: vec![protocol::SelectionMode::Single], methods: vec![protocol::SelectionMethod:...
      |                  ^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35727 -                     selection: protocol::SelectionSpec { modes: vec![protocol::SelectionMode::Single], methods: vec![protocol::SelectionMethod::Pick], merges: vec![protocol::MergeMode::Replace], transitive: false, broadcast: true },
35727 +                     selection: SelectionSpec { modes: vec![protocol::SelectionMode::Single], methods: vec![protocol::SelectionMethod::Pick], merges: vec![protocol::MergeMode::Replace], transitive: false, broadcast: true },
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35727:70
      |
35727 | ...   selection: protocol::SelectionSpec { modes: vec![protocol::SelectionMode::Single], methods: vec![protocol::SelectionMethod:...
      |                                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35727 -                     selection: protocol::SelectionSpec { modes: vec![protocol::SelectionMode::Single], methods: vec![protocol::SelectionMethod::Pick], merges: vec![protocol::MergeMode::Replace], transitive: false, broadcast: true },
35727 +                     selection: protocol::SelectionSpec { modes: vec![SelectionMode::Single], methods: vec![protocol::SelectionMethod::Pick], merges: vec![protocol::MergeMode::Replace], transitive: false, broadcast: true },
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35727:118
      |
35727 | ...rotocol::SelectionMode::Single], methods: vec![protocol::SelectionMethod::Pick], merges: vec![protocol::MergeMode::Replace], t...
      |                                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35727 -                     selection: protocol::SelectionSpec { modes: vec![protocol::SelectionMode::Single], methods: vec![protocol::SelectionMethod::Pick], merges: vec![protocol::MergeMode::Replace], transitive: false, broadcast: true },
35727 +                     selection: protocol::SelectionSpec { modes: vec![protocol::SelectionMode::Single], methods: vec![SelectionMethod::Pick], merges: vec![protocol::MergeMode::Replace], transitive: false, broadcast: true },
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35727:165
      |
35727 | ...![protocol::SelectionMethod::Pick], merges: vec![protocol::MergeMode::Replace], transitive: false, broadcast: true },
      |                                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35727 -                     selection: protocol::SelectionSpec { modes: vec![protocol::SelectionMode::Single], methods: vec![protocol::SelectionMethod::Pick], merges: vec![protocol::MergeMode::Replace], transitive: false, broadcast: true },
35727 +                     selection: protocol::SelectionSpec { modes: vec![protocol::SelectionMode::Single], methods: vec![protocol::SelectionMethod::Pick], merges: vec![MergeMode::Replace], transitive: false, broadcast: true },
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35738:31
      |
35738 | ...   app.handle_action(semio_framework::INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items",...
      |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35738 -             app.handle_action(semio_framework::INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "item-1")), &meta()).await.expect("interactionSelect");
35738 +             app.handle_action(INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "item-1")), &meta()).await.expect("interactionSelect");
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35750:43
      |
35750 |                         interaction: Some(protocol::PresenceInteraction {
      |                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35750 -                         interaction: Some(protocol::PresenceInteraction {
35750 +                         interaction: Some(PresenceInteraction {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35752:43
      |
35752 | ...   domains: vec![protocol::PresenceDomain { domain: "items".to_string(), granularity: "item".to_string(), selected: vec!["item...
      |                     ^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35752 -                             domains: vec![protocol::PresenceDomain { domain: "items".to_string(), granularity: "item".to_string(), selected: vec!["item-1".to_string()], hovered: Vec::new() }],
35752 +                             domains: vec![PresenceDomain { domain: "items".to_string(), granularity: "item".to_string(), selected: vec!["item-1".to_string()], hovered: Vec::new() }],
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35766:24
      |
35766 |             let item = semio_framework_ui_runtime::TreeNode::try_new(
      |                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35766 -             let item = semio_framework_ui_runtime::TreeNode::try_new(
35766 +             let item = TreeNode::try_new(
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35768:17
      |
35768 |                 semio_framework_ui_contract::Component::TreeItem(semio_framework_ui_contract::TreeItemProps {
      |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35768 -                 semio_framework_ui_contract::Component::TreeItem(semio_framework_ui_contract::TreeItemProps {
35768 +                 Component::TreeItem(semio_framework_ui_contract::TreeItemProps {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35768:66
      |
35768 |                 semio_framework_ui_contract::Component::TreeItem(semio_framework_ui_contract::TreeItemProps {
      |                                                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35768 -                 semio_framework_ui_contract::Component::TreeItem(semio_framework_ui_contract::TreeItemProps {
35768 +                 semio_framework_ui_contract::Component::TreeItem(TreeItemProps {
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35769:28
      |
35769 | ...   label: semio_framework_ui_contract::Label(semio_framework_ui_contract::UiText::try_from_str("Item 1").expect("bounded fixtu...
      |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35769 -                     label: semio_framework_ui_contract::Label(semio_framework_ui_contract::UiText::try_from_str("Item 1").expect("bounded fixture")),
35769 +                     label: Label(semio_framework_ui_contract::UiText::try_from_str("Item 1").expect("bounded fixture")),
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35769:63
      |
35769 | ...   label: semio_framework_ui_contract::Label(semio_framework_ui_contract::UiText::try_from_str("Item 1").expect("bounded fixtu...
      |                                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35769 -                     label: semio_framework_ui_contract::Label(semio_framework_ui_contract::UiText::try_from_str("Item 1").expect("bounded fixture")),
35769 +                     label: semio_framework_ui_contract::Label(UiText::try_from_str("Item 1").expect("bounded fixture")),
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35776:34
      |
35776 |                     row_actions: semio_framework_ui_contract::UiFixedList::default(),
      |                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35776 -                     row_actions: semio_framework_ui_contract::UiFixedList::default(),
35776 +                     row_actions: UiFixedList::default(),
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35780:27
      |
35780 | ...   let section = semio_framework_ui_runtime::TreeNode::try_new("sec", semio_framework_ui_contract::Component::TreeSection(semi...
      |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35780 -             let section = semio_framework_ui_runtime::TreeNode::try_new("sec", semio_framework_ui_contract::Component::TreeSection(semio_framework_ui_contract::TreeSectionProps { label: None, default_open: None }))
35780 +             let section = TreeNode::try_new("sec", semio_framework_ui_contract::Component::TreeSection(semio_framework_ui_contract::TreeSectionProps { label: None, default_open: None }))
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35780:80
      |
35780 | ...   let section = semio_framework_ui_runtime::TreeNode::try_new("sec", semio_framework_ui_contract::Component::TreeSection(semi...
      |                                                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35780 -             let section = semio_framework_ui_runtime::TreeNode::try_new("sec", semio_framework_ui_contract::Component::TreeSection(semio_framework_ui_contract::TreeSectionProps { label: None, default_open: None }))
35780 +             let section = semio_framework_ui_runtime::TreeNode::try_new("sec", Component::TreeSection(semio_framework_ui_contract::TreeSectionProps { label: None, default_open: None }))
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35780:132
      |
35780 | ...ork_ui_contract::Component::TreeSection(semio_framework_ui_contract::TreeSectionProps { label: None, default_open: None }))
      |                                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35780 -             let section = semio_framework_ui_runtime::TreeNode::try_new("sec", semio_framework_ui_contract::Component::TreeSection(semio_framework_ui_contract::TreeSectionProps { label: None, default_open: None }))
35780 +             let section = semio_framework_ui_runtime::TreeNode::try_new("sec", semio_framework_ui_contract::Component::TreeSection(TreeSectionProps { label: None, default_open: None }))
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35784:24
      |
35784 |             let root = semio_framework_ui_runtime::TreeNode::try_new(
      |                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35784 -             let root = semio_framework_ui_runtime::TreeNode::try_new(
35784 +             let root = TreeNode::try_new(
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35786:17
      |
35786 | ...   semio_framework_ui_contract::Component::Tree(semio_framework_ui_contract::TreeProps { interaction_domain: Some(semio_framew...
      |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35786 -                 semio_framework_ui_contract::Component::Tree(semio_framework_ui_contract::TreeProps { interaction_domain: Some(semio_framework_ui_contract::UiText::try_from_str("items").expect("bounded fixture")) }),
35786 +                 Component::Tree(semio_framework_ui_contract::TreeProps { interaction_domain: Some(semio_framework_ui_contract::UiText::try_from_str("items").expect("bounded fixture")) }),
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35786:62
      |
35786 | ...   semio_framework_ui_contract::Component::Tree(semio_framework_ui_contract::TreeProps { interaction_domain: Some(semio_framew...
      |                                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35786 -                 semio_framework_ui_contract::Component::Tree(semio_framework_ui_contract::TreeProps { interaction_domain: Some(semio_framework_ui_contract::UiText::try_from_str("items").expect("bounded fixture")) }),
35786 +                 semio_framework_ui_contract::Component::Tree(TreeProps { interaction_domain: Some(semio_framework_ui_contract::UiText::try_from_str("items").expect("bounded fixture")) }),
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35786:128
      |
35786 | ...:TreeProps { interaction_domain: Some(semio_framework_ui_contract::UiText::try_from_str("items").expect("bounded fixture")) }),
      |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35786 -                 semio_framework_ui_contract::Component::Tree(semio_framework_ui_contract::TreeProps { interaction_domain: Some(semio_framework_ui_contract::UiText::try_from_str("items").expect("bounded fixture")) }),
35786 +                 semio_framework_ui_contract::Component::Tree(semio_framework_ui_contract::TreeProps { interaction_domain: Some(UiText::try_from_str("items").expect("bounded fixture")) }),
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35791:24
      |
35791 |             let tree = semio_framework_ui_runtime::ComponentTree { root };
      |                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35791 -             let tree = semio_framework_ui_runtime::ComponentTree { root };
35791 +             let tree = ComponentTree { root };
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:35875:65
      |
35875 | ...   crate::plugin_runtime::test_push_instance(&runtime, crate::plugin_runtime::AppInstance { id: resumed_instance, app: TestRun...
      |                                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
35875 -             crate::plugin_runtime::test_push_instance(&runtime, crate::plugin_runtime::AppInstance { id: resumed_instance, app: TestRuntimeApps::from(app) }).await;
35875 +             crate::plugin_runtime::test_push_instance(&runtime, AppInstance { id: resumed_instance, app: TestRuntimeApps::from(app) }).await;
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:36000:29
      |
36000 | ...   let completion: serde_json::Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🧪️tests/...o
      |                       ^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
36000 -             let completion: serde_json::Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🧪️tests/⏳️completion/🧪️fixture.json"))).unwrap();
36000 +             let completion: Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🧪️tests/⏳️completion/🧪️fixture.json"))).unwrap();
      |

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:36050:26
      |
36050 | ...   let fixture: serde_json::Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🧪️tests/⏳️co...
      |                    ^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
36050 -             let fixture: serde_json::Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🧪️tests/⏳️completion/🧪️fixture.json"))).unwrap();
36050 +             let fixture: Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🧪️tests/⏳️completion/🧪️fixture.json"))).unwrap();
      |

warning: ambiguous glob re-exports
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:36951:13
      |
36951 |     pub use semio_framework::kernel::*;
      |             ^^^^^^^^^^^^^^^^^^^^^^^^^^ the name `ActionId` in the type namespace is first re-exported here
...
36954 |     pub use semio_framework_ui_contract::*;
      |             ------------------------------ but the name `ActionId` in the type namespace is also re-exported here
      |
      = note: `#[warn(ambiguous_glob_reexports)]` on by default

warning: ambiguous glob re-exports
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:36954:13
      |
36950 |     pub use crate::app::*;
      |             ------------- but the name `tree_item` in the value namespace is also re-exported here
...
36954 |     pub use semio_framework_ui_contract::*;
      |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the name `tree_item` in the value namespace is first re-exported here

warning: ambiguous glob re-exports
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:36954:13
      |
36950 |     pub use crate::app::*;
      |             ------------- but the name `PeerMark` in the type namespace is also re-exported here
...
36954 |     pub use semio_framework_ui_contract::*;
      |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the name `PeerMark` in the type namespace is first re-exported here

warning: unnecessary qualification
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:37629:26
      |
37629 |         let registered = semio_framework::io::list_registered_subset_validator_dialects().await.expect("registry observation");
      |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
help: remove the unnecessary path segments
      |
37629 -         let registered = semio_framework::io::list_registered_subset_validator_dialects().await.expect("registry observation");
37629 +         let registered = io::list_registered_subset_validator_dialects().await.expect("registry observation");
      |

warning: variable does not need to be mutable
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:29403:17
      |
29403 |             let mut instance = find_instance(list, instance_id)?;
      |                 ----^^^^^^^^
      |                 |
      |                 help: remove this `mut`
      |
      = note: `#[warn(unused_mut)]` (part of `#[warn(unused)]`) on by default

warning: variable does not need to be mutable
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:29807:17
      |
29807 |             let mut instance = find_instance(list, instance_id)?;
      |                 ----^^^^^^^^
      |                 |
      |                 help: remove this `mut`

warning: variable does not need to be mutable
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:29826:17
      |
29826 |             let mut instance = find_instance(list, instance_id)?;
      |                 ----^^^^^^^^
      |                 |
      |                 help: remove this `mut`

warning: variable does not need to be mutable
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:30956:37
      |
30956 | ...                   let mut instance = find_instance(list, instance_id)?;
      |                           ----^^^^^^^^
      |                           |
      |                           help: remove this `mut`

warning: variable does not need to be mutable
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:31036:41
      |
31036 | ...                   let mut instance = find_instance(list, instance_id)?;
      |                           ----^^^^^^^^
      |                           |
      |                           help: remove this `mut`

warning: variable does not need to be mutable
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:31086:29
      |
31086 |                         let mut instance = find_instance(list, instance_id)?;
      |                             ----^^^^^^^^
      |                             |
      |                             help: remove this `mut`

warning: variable does not need to be mutable
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:31120:29
      |
31120 |                         let mut instance = find_instance(list, instance_id)?;
      |                             ----^^^^^^^^
      |                             |
      |                             help: remove this `mut`

warning: variable does not need to be mutable
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:31307:29
      |
31307 |                         let mut instance = find_instance(list, instance_id)?;
      |                             ----^^^^^^^^
      |                             |
      |                             help: remove this `mut`

warning: variable does not need to be mutable
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:31339:17
      |
31339 |             let mut instance = find_instance(list, instance_id)?;
      |                 ----^^^^^^^^
      |                 |
      |                 help: remove this `mut`

warning: variable does not need to be mutable
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:30757:13
      |
30757 |         let mut retry_command = None;
      |             ----^^^^^^^^^^^^^
      |             |
      |             help: remove this `mut`

warning: variable does not need to be mutable
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:30759:13
      |
30759 |         let mut presence_pending = None;
      |             ----^^^^^^^^^^^^^^^^
      |             |
      |             help: remove this `mut`

warning: use of deprecated method `std::sync::atomic::Atomic::<u64>::fetch_update`: renamed to `try_update` for consistency
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:29354:22
      |
29354 |                     .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |generation| generation.checked_add(1))
      |                      ^^^^^^^^^^^^
      |
      = note: `#[warn(deprecated)]` on by default
help: replace the use of the deprecated method
      |
29354 -                     .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |generation| generation.checked_add(1))
29354 +                     .try_update(Ordering::SeqCst, Ordering::SeqCst, |generation| generation.checked_add(1))
      |

warning: use of deprecated method `std::sync::atomic::Atomic::<u64>::fetch_update`: renamed to `try_update` for consistency
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:14745:41
      |
14745 | ...   let _ = self.app_generation.fetch_update(std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire, |genera...
      |                                   ^^^^^^^^^^^^
      |
help: replace the use of the deprecated method
      |
14745 -             let _ = self.app_generation.fetch_update(std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire, |generation| Some(generation.saturating_add(1)));
14745 +             let _ = self.app_generation.try_update(std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire, |generation| Some(generation.saturating_add(1)));
      |

warning: use of deprecated method `std::sync::atomic::Atomic::<usize>::fetch_update`: renamed to `try_update` for consistency
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:13660:43
      |
13660 | ...   let result = self.state.bytes.fetch_update(std::sync::atomic::Ordering::SeqCst, std::sync::atomic::Ordering::SeqCst, |curre...
      |                                     ^^^^^^^^^^^^
      |
help: replace the use of the deprecated method
      |
13660 -             let result = self.state.bytes.fetch_update(std::sync::atomic::Ordering::SeqCst, std::sync::atomic::Ordering::SeqCst, |current| current.checked_add(bytes).filter(|next| *next <= self.maximum)).map(|previous| previous + bytes);
13660 +             let result = self.state.bytes.try_update(std::sync::atomic::Ordering::SeqCst, std::sync::atomic::Ordering::SeqCst, |current| current.checked_add(bytes).filter(|next| *next <= self.maximum)).map(|previous| previous + bytes);
      |

warning: unused import: `Mutation`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:32031:24
      |
32031 |         use protocol::{Mutation, MutationDiff};
      |                        ^^^^^^^^

warning: unused import: `MutationKind`
 --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🧬️contributed-mutation-wire/🧪️tests/🦀️.rs:5:63
  |
5 | use protocol::{CompositeMutationKind, Mutation, MutationDiff, MutationKind, MutationLeaf, OpBinary};
  |                                                               ^^^^^^^^^^^^

warning: unused variable: `parent_document_id`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:18716:17
      |
18716 |             let parent_document_id = self.store.envelope().id.clone();
      |                 ^^^^^^^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_parent_document_id`
      |
      = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

warning: unused variable: `envelope_seq`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:30836:22
      |
30836 |         if let Some((envelope_seq, mut owner)) = command {
      |                      ^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_envelope_seq`

warning: unused variable: `actor`
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:9235:26
     |
9235 |                     let (actor, pack) = self.packs[self.packs_len - 1].as_ref().expect("retained app-typed presence pack");
     |                          ^^^^^ help: if this is intentional, prefix it with an underscore: `_actor`

warning: type `SchemaStampEditorFixture` is more private than the item `SchemaStampApps::Editor::0`
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🏗️builder/🦀️component.rs:948:20
    |
948 |             Editor(crate::app::VcsArtifactApp<crate::app::EditorApp<SchemaStampEditorFixture>>),
    |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ field `SchemaStampApps::Editor::0` is reachable at visibility `pub(in crate::component::builder)`
    |
note: but type `SchemaStampEditorFixture` is only usable at visibility `pub(self)`
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🏗️builder/🦀️component.rs:873:5
    |
873 |     struct SchemaStampEditorFixture;
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: `#[warn(private_interfaces)]` on by default

warning: type `SchemaStampViewerFixture` is more private than the item `SchemaStampApps::Viewer::0`
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🏗️builder/🦀️component.rs:949:20
    |
949 |             Viewer(crate::app::VcsArtifactApp<crate::app::ViewerApp<SchemaStampViewerFixture>>),
    |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ field `SchemaStampApps::Viewer::0` is reachable at visibility `pub(in crate::component::builder)`
    |
note: but type `SchemaStampViewerFixture` is only usable at visibility `pub(self)`
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🏗️builder/🦀️component.rs:911:5
    |
911 |     struct SchemaStampViewerFixture;
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: trait `component::reactor::instance_lifetime::terminal_owner::Sealed` is more private than the item `component::reactor::instance_lifetime::GuestLifetimeOwner`
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🚪️lifetime/🦀️component.rs:38:1
   |
38 | pub(crate) trait GuestLifetimeOwner: terminal_owner::Sealed {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ trait `component::reactor::instance_lifetime::GuestLifetimeOwner` is reachable at visibility `pub(crate)`
   |
note: but trait `component::reactor::instance_lifetime::terminal_owner::Sealed` is only usable at visibility `pub(in crate::component::reactor::instance_lifetime)`
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🚪️lifetime/🦀️component.rs:34:22
   |
34 | mod terminal_owner { pub(super) trait Sealed {} }
   |                      ^^^^^^^^^^^^^^^^^^^^^^^
   = note: `#[warn(private_bounds)]` on by default

warning: type `ArtifactToolRegistration` is more private than the item `component::app::AppActionRegistry::tool_job_registration`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:11945:9
      |
11945 | /         pub(crate) fn tool_job_registration<A: ArtifactApp>(
11946 | |             &self,
11947 | |             runtime_controller_id: &str,
11948 | |             document_schema: &str,
...     |
11951 | |             registrations: &BTreeMap<String, ArtifactToolRegistration>,
11952 | |         ) -> Result<(String, Vec<QualifiedBoundedFirstStepProof>), Fault> {
      | |_________________________________________________________________________^ method `component::app::AppActionRegistry::tool_job_registration` is reachable at visibility `pub(crate)`
      |
note: but type `ArtifactToolRegistration` is only usable at visibility `pub(self)`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:12571:5
      |
12571 |     struct ArtifactToolRegistration {
      |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: type `ToolCancellationLease` is more private than the item `component::app::ToolCancellationHandle::begin`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:14633:9
      |
14633 |         pub(crate) fn begin(&self, key: ToolOperationKey) -> Result<ToolCancellationLease, Fault> {
      |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ method `component::app::ToolCancellationHandle::begin` is reachable at visibility `pub(crate)`
      |
note: but type `ToolCancellationLease` is only usable at visibility `pub(self)`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:14802:5
      |
14802 |     struct ToolCancellationLease {
      |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: call to `.clone()` on a reference in this situation does nothing
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:3220:119
     |
3220 |                 Ok(schema) => match ArtifactIdentityClaim::new(ArtifactIdentityNamespace::extension(), codec.extension.clone()) {
     |                                                                                                                       ^^^^^^^^ help: remove this redundant call
     |
     = note: the type `str` does not implement `Clone`, so calling `clone` on `&str` copies the reference, which does not do anything and can be removed
     = note: `#[warn(noop_method_call)]` on by default

warning: call to `.clone()` on a reference in this situation does nothing
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:3254:119
     |
3254 |                 Ok(schema) => match ArtifactIdentityClaim::new(ArtifactIdentityNamespace::extension(), codec.extension.clone()) {
     |                                                                                                                       ^^^^^^^^ help: remove this redundant call
     |
     = note: the type `str` does not implement `Clone`, so calling `clone` on `&str` copies the reference, which does not do anything and can be removed

warning: calls to `std::mem::drop` with a value that implements `Copy` does nothing
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:18097:17
      |
18097 |                 drop(page);
      |                 ^^^^^----^
      |                      |
      |                      argument has type `OwnedSchemaDecodePage`
      |
      = note: `#[warn(dropping_copy_types)]` on by default
help: use `let _ = ...` to ignore the expression or result
      |
18097 -                 drop(page);
18097 +                 let _ = page;
      |

warning: method `insert` is never used
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🦀️component.rs:354:8
    |
341 | impl TaskRecordRegistry {
    | ----------------------- method in this implementation
...
354 |     fn insert(&mut self, id: executor::TaskId, record: TaskRecord) -> Result<(), TaskRecord> {
    |        ^^^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: method `put_at` is never used
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🦀️component.rs:429:8
    |
411 | impl ReactorCloseRegistry {
    | ------------------------- method in this implementation
...
429 |     fn put_at(&mut self, index: usize, state: ReactorCloseState) -> Result<(), ReactorCloseState> {
    |        ^^^^^^

warning: field `allocation_admitted` is never read
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🦀️component.rs:451:5
    |
446 | struct FixedTimerRegistry {
    |        ------------------ field in this struct
...
451 |     allocation_admitted: bool,
    |     ^^^^^^^^^^^^^^^^^^^

warning: methods `insert`, `first`, and `contains` are never used
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🦀️component.rs:465:8
    |
454 | impl FixedTimerRegistry {
    | ----------------------- methods in this implementation
...
465 |     fn insert(&mut self, instance: u32, id: u64) -> Result<(), u64> {
    |        ^^^^^^
...
514 |     fn first(&self) -> Option<u64> {
    |        ^^^^^
...
518 |     fn contains(&self, id: u64) -> bool {
    |        ^^^^^^^^

warning: method `is_empty` is never used
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🦀️component.rs:686:8
    |
662 | impl FixedResumeQueue {
    | --------------------- method in this implementation
...
686 |     fn is_empty(&self) -> bool {
    |        ^^^^^^^^

warning: variant `ExplicitStateMachineRequired` is never constructed
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/💼️jobs/🦀️component.rs:297:5
    |
290 | enum JobBody {
    |      ------- variant in this enum
...
297 |     ExplicitStateMachineRequired,
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: variant `Commit` is never constructed
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/💼️jobs/💡️infer/🦀️component.rs:42:5
   |
40 | enum LosslessInferenceItem {
   |      --------------------- variant in this enum
41 |     Checkpoint(semio_framework_job::Checkpoint),
42 |     Commit(CommitCandidate),
   |     ^^^^^^
   |
   = note: `LosslessInferenceItem` has a derived impl for the trait `Debug`, but this is intentionally ignored during dead code analysis

warning: method `insert` is never used
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/📮️requests/🦀️component.rs:117:8
    |
 49 | impl Inner {
    | ---------- method in this implementation
...
117 |     fn insert(&mut self, entry: SlotEntry) -> Result<(), SlotEntry> {
    |        ^^^^^^

warning: associated items `capture` and `lifetime` are never used
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🚪️lifetime/🦀️component.rs:10:19
   |
 9 | impl NativeCloseKey {
   | ------------------- associated items in this implementation
10 |     pub(crate) fn capture<PA: crate::app::PluginApp + 'static>(lifetime: ActorInstanceLifetime, lease: &crate::plugin_runtime::Plugi...
   |                   ^^^^^^^
...
16 |     pub(super) fn lifetime(self) -> ActorInstanceLifetime { self.lifetime }
   |                   ^^^^^^^^

warning: methods `matches_open` and `is_closing` are never used
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🚪️lifetime/🦀️component.rs:70:19
   |
59 | impl<O: GuestLifetimeOwner> GuestLifecycleCell<O> {
   | ------------------------------------------------- methods in this implementation
...
70 |     pub(crate) fn matches_open(&self, open: ActorInstanceOpenRequest) -> bool { self.open == open }
   |                   ^^^^^^^^^^^^
...
76 |     pub(crate) fn is_closing(&self) -> bool { matches!(self.phase, Phase::Accepted | Phase::Closing | Phase::Retired) }
   |                   ^^^^^^^^^^

warning: variant `Reconcile` is never constructed
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/📨️pending/🦀️component.rs:11:5
   |
10 | enum PendingPatchOwner {
   |      ----------------- variant in this enum
11 |     Reconcile(SurfaceReconcileReadyPatch),
   |     ^^^^^^^^^

warning: fields `sequence` and `emitted` are never read
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/📨️pending/🦀️component.rs:16:5
   |
15 | struct PendingPatchSlot {
   |        ---------------- fields in this struct
16 |     sequence: u64,
   |     ^^^^^^^^
...
22 |     emitted: bool,
   |     ^^^^^^^

warning: methods `has_capacity`, `push_reconcile`, `take_one`, `apply_published_ack`, and `has_unpublished` are never used
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/📨️pending/🦀️component.rs:52:19
    |
 37 | impl PendingPatchAuthority {
    | -------------------------- methods in this implementation
...
 52 |     pub(super) fn has_capacity(&self) -> bool {
    |                   ^^^^^^^^^^^^
...
 56 |     pub(super) fn push_reconcile(&mut self, owner: SurfaceReconcileReadyPatch) -> Result<(), SurfaceReconcileReadyPatch> {
    |                   ^^^^^^^^^^^^^^
...
 76 |     pub(super) fn take_one(&mut self, admitted_bytes: usize) -> Result<Option<UiPatch>, &'static str> {
    |                   ^^^^^^^^
...
110 |     pub(super) fn apply_published_ack(&mut self, surface: &str, revision: u64, admitted_bytes: usize, advance: impl FnOnce(&Surface...
    |                   ^^^^^^^^^^^^^^^^^^^
...
199 |     pub(super) fn has_unpublished(&self) -> bool {
    |                   ^^^^^^^^^^^^^^^

warning: function `with_state` is never used
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/📨️pending/🦀️component.rs:309:15
    |
309 | pub(super) fn with_state<R>(use_state: impl FnOnce(&RefCell<PendingPatchAuthority>) -> R) -> R { PENDING_PATCHES.with(use_state) }
    |               ^^^^^^^^^^

warning: constant `DIRECT_READ_CHUNK` is never used
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🌐host/📖️body/🦀️component.rs:17:7
   |
17 | const DIRECT_READ_CHUNK: usize = 64 * 1024;
   |       ^^^^^^^^^^^^^^^^^

warning: fields `document_generation`, `document_revision`, `config_generation`, and `config_revision` are never read
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🕹️interaction/🔐️authority/📖️inputs/🦀️component.rs:17:5
   |
15 | pub(crate) struct LocalInteractionInputReads<D, C> {
   |                   -------------------------- fields in this struct
16 |     owned: ManuallyDrop<InputReadState<D, C>>,
17 |     document_generation: u64,
   |     ^^^^^^^^^^^^^^^^^^^
18 |     document_revision: [u8; 32],
   |     ^^^^^^^^^^^^^^^^^
19 |     config_generation: u64,
   |     ^^^^^^^^^^^^^^^^^
20 |     config_revision: [u8; 32],
   |     ^^^^^^^^^^^^^^^

warning: associated items `new`, `document_revision`, `config_revision`, and `authority_is_current` are never used
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🕹️interaction/🔐️authority/📖️inputs/🦀️component.rs:25:19
   |
23 | impl<D, C> LocalInteractionInputReads<D, C> {
   | ------------------------------------------- associated items in this implementation
24 |     /// 📥️ Called under the app's exclusive owner with the read and fixed identity captured together.
25 |     pub(crate) fn new(document: SnapshotRead<D>, document_generation: u64, document_revision: [u8; 32], config: SnapshotRead<C>, con...
   |                   ^^^
...
34 |     pub(crate) fn document_revision(&self) -> [u8; 32] { self.document_revision }
   |                   ^^^^^^^^^^^^^^^^^
35 |     pub(crate) fn config_revision(&self) -> [u8; 32] { self.config_revision }
   |                   ^^^^^^^^^^^^^^^
36 |
37 |     pub(crate) fn authority_is_current(&self) -> bool {
   |                   ^^^^^^^^^^^^^^^^^^^^

warning: field `preview_sequence` is never read
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:27799:9
      |
27792 |     struct RuntimeCloseWorkerState<PA: PluginApp> {
      |            ----------------------- field in this struct
...
27799 |         preview_sequence: AtomicU64,
      |         ^^^^^^^^^^^^^^^^

warning: associated function `new` is never used
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:27830:12
      |
27829 |     impl RuntimeActorAuthority {
      |     -------------------------- associated function in this implementation
27830 |         fn new(actor: String) -> Result<Self, Fault> {
      |            ^^^

warning: methods `entry_mut` and `get_mut` are never used
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:27888:12
      |
27850 |     impl<T> RuntimeInstanceRegistry<T> {
      |     ---------------------------------- methods in this implementation
...
27888 |         fn entry_mut(&mut self, index: usize) -> Option<&mut (u32, T)> {
      |            ^^^^^^^^^
...
27922 |         fn get_mut(&mut self, instance_id: u32) -> Option<&mut T> {
      |            ^^^^^^^

warning: field `instance_id` is never read
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:28795:9
      |
28794 |     struct RuntimeCloseCleanupJob<PA: PluginApp> {
      |            ---------------------- field in this struct
28795 |         instance_id: u32,
      |         ^^^^^^^^^^^

warning: method `allocation_identity` is never used
  --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🚪️lifetime/🦀️component.rs:12:19
   |
11 | impl<PA: PluginApp + 'static> PluginInstanceCloseLease<PA> {
   | ---------------------------------------------------------- method in this implementation
12 |     pub(crate) fn allocation_identity(&self) -> (u32, usize) { (self.instance_id, self.cell.as_ptr().cast::<()>() as usize) }
   |                   ^^^^^^^^^^^^^^^^^^^

warning: associated function `from_operation` is never used
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:8023:12
     |
8022 |     impl AppOperationContext {
     |     ------------------------ associated function in this implementation
8023 |         fn from_operation(app_instance_id: u32, parent_document_id: String, operation: semio_framework_job::Operation, canonical_b...
     |            ^^^^^^^^^^^^^^

warning: associated function `with_dispatch_context` is never used
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:8057:18
     |
8039 |     impl<'a, P> ArtifactView<'a, P> {
     |     ------------------------------- associated function in this implementation
...
8057 |         async fn with_dispatch_context(snapshot: &'a P, history: &'a HistoryView, children: ChildContentView, operation: AppOperat...
     |                  ^^^^^^^^^^^^^^^^^^^^^

warning: field `seq` is never read
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:9073:9
     |
9072 |     struct ValidatedPeerRosterCommit {
     |            ------------------------- field in this struct
9073 |         seq: u64,
     |         ^^^

warning: associated function `new` is never used
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:13996:31
      |
13995 |             impl $job {
      |             --------- associated function in this implementation
13996 |                 pub(crate) fn new(raw: Vec<u8>, total_items: usize) -> Self {
      |                               ^^^
...
14165 |     framework_reserved_job!(FrameworkImportMediaJob, FrameworkImportMediaJobFactory, "import-media", 10, 8_388_608, 8_192, 4_096, 8_388_608);
      |     ---------------------------------------------------------------------------------------------------------------------------------------- in this macro invocation
      |
      = note: this warning originates in the macro `framework_reserved_job` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: associated function `new` is never used
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:14115:20
      |
14114 |             impl<A: ArtifactApp> $factory<A> {
      |             -------------------------------- associated function in this implementation
14115 |                 fn new(controller_id: &str) -> Self {
      |                    ^^^
...
14165 |     framework_reserved_job!(FrameworkImportMediaJob, FrameworkImportMediaJobFactory, "import-media", 10, 8_388_608, 8_192, 4_096, 8_388_608);
      |     ---------------------------------------------------------------------------------------------------------------------------------------- in this macro invocation
      |
      = note: this warning originates in the macro `framework_reserved_job` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: fields `verb` and `contract` are never read
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:15880:9
      |
15879 |     struct MountedTypedCommandFullOperation<A: ArtifactApp> {
      |            -------------------------------- fields in this struct
15880 |         verb: String,
      |         ^^^^
...
15889 |         contract: semio_framework::ToolExecutionContract,
      |         ^^^^^^^^

warning: variant `OutputValidation` is never constructed
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:16227:9
      |
16224 |     enum TypedCommandFullOperationPhase {
      |          ------------------------------ variant in this enum
...
16227 |         OutputValidation,
      |         ^^^^^^^^^^^^^^^^
      |
      = note: `TypedCommandFullOperationPhase` has derived impls for the traits `Debug` and `Clone`, but these are intentionally ignored during dead code analysis

warning: fields `app_instance_id` and `canonical_base_revision` are never read
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:16237:9
      |
16235 |     pub(crate) struct TypedCommandFullOperationJob<A: ArtifactApp> {
      |                       ---------------------------- fields in this struct
16236 |         operation: Option<semio_framework_job::Operation>,
16237 |         app_instance_id: u32,
      |         ^^^^^^^^^^^^^^^
16238 |         parent_document_id: Option<String>,
16239 |         canonical_base_revision: [u8; 32],
      |         ^^^^^^^^^^^^^^^^^^^^^^^

warning: method `admit_exposure_freshness` is never used
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:16301:12
      |
16273 |     impl<A: ArtifactApp> TypedCommandFullOperationJob<A> {
      |     ---------------------------------------------------- method in this implementation
...
16301 |         fn admit_exposure_freshness(&mut self, revision: semio_framework_job::RevisionId, generation: semio_framework_job::Genera...
      |            ^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused `Result` that must be used
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:24446:13
      |
24446 |             self.store.detach_backbone();
      |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
      = note: this `Result` may be an `Err` variant, which should be handled
      = note: `#[warn(unused_must_use)]` (part of `#[warn(unused)]`) on by default
help: use `let _ = ...` to ignore the resulting value
      |
24446 |             let _ = self.store.detach_backbone();
      |             +++++++

warning: `semio-framework-plugin` (lib test) generated 629 warnings (run `cargo fix --lib -p semio-framework-plugin --tests` to apply 581 suggestions)
    Finished `test` profile [unoptimized] target(s) in 25.53s
warning: the following packages contain code that will be rejected by a future version of Rust: semio-framework-plugin v0.1.0 (/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust)
note: to see what the problems were, use the option `--future-incompat-report`, or run `cargo report future-incompatibilities --id 25`
  Executable unittests 📦️glue.rs (/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_plugin-d81b8cb7f98afff8)



 NX   Successfully ran target test for project @semio-tech/framework-plugin



```

