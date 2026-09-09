# WASI 1096 Diagnostics

Completed receipt: {"status":101,"signal":null,"reason":"exit","counts":{"error":9,"warning":0},"cargoWarnings":["warning: ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/Cargo.toml: `default-features` is ignored for semio-s-plugin-flow-extension-brep, since `default-features` was not specified for `workspace.dependencies.semio-s-plugin-flow-extension-brep`, this could become a hard error in the future","warning: ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/Cargo.toml: `default-features` is ignored for semio-s-plugin-flow-extension-math, since `default-features` was not specified for `workspace.dependencies.semio-s-plugin-flow-extension-math`, this could become a hard error in the future","warning: `semio-s-artifact-procedural-generation3d` (manifest) generated 2 warnings","warning: build failed, waiting for other jobs to finish..."],"startedAt":"2026-09-09T12:55:05.124Z","finishedAt":"2026-09-09T13:11:09.996Z","packages":160}

## error — cannot find derive macro `ToValue` in this scope

error: cannot find derive macro `ToValue` in this scope
 --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🪆️child/🏠️owner/🧬️schema/🦀️.rs:6:39
  |
6 | #[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
  |                                       ^^^^^^^
  |
note: `ToValue` is imported here, but it is only a trait, without a derive macro
 --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🪆️child/🏠️owner/🧬️schema/🦀️.rs:2:32
  |
2 | use crate::os_dsl::{FromValue, ToValue};
  |                                ^^^^^^^
help: consider importing one of these derive macros
  |
2 + use crate::ToValue;
  |
2 + use semio_framework_value_derive::ToValue;
  |


## error — cannot find derive macro `FromValue` in this scope

error: cannot find derive macro `FromValue` in this scope
 --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🪆️child/🏠️owner/🧬️schema/🦀️.rs:6:48
  |
6 | #[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
  |                                                ^^^^^^^^^
  |
note: `FromValue` is imported here, but it is only a trait, without a derive macro
 --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🪆️child/🏠️owner/🧬️schema/🦀️.rs:2:21
  |
2 | use crate::os_dsl::{FromValue, ToValue};
  |                     ^^^^^^^^^
help: consider importing one of these derive macros
  |
2 + use crate::FromValue;
  |
2 + use semio_framework_value_derive::FromValue;
  |


## error — cannot find attribute `value` in this scope

error: cannot find attribute `value` in this scope
 --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🪆️child/🏠️owner/🧬️schema/🦀️.rs:7:3
  |
7 | #[value(rename_all = "camelCase")]
  |   ^^^^^
  |
help: `value` is an attribute that can be used by the derive macros `FromValue` and `ToValue`, you might be missing a `derive` attribute
  |
8 + #[derive(FromValue, ToValue)]
9 | pub struct OwnerRef {
  |


## E0599 — the method `close_owned_step` exists for mutable reference `&mut os_store::component::ArtifactStore<P, M>`, but its trait bounds were not satisfied

error[E0599]: the method `close_owned_step` exists for mutable reference `&mut os_store::component::ArtifactStore<P, M>`, but its trait bounds were not satisfied
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs:356:31
      |
  356 |             let step = member.close_owned_step(items, bytes)?;
      |                               ^^^^^^^^^^^^^^^^
      |
     ::: 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:14128:1
      |
14128 | pub struct ArtifactStore<P, Mutation>
      | ------------------------------------- doesn't satisfy `_: SpaceMember`
      |
note: trait bound `P: os_schema_composition::ArtifactCompositionFields` was not satisfied
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:18643:53
      |
18641 | impl<P, Mutation> SpaceMember for ArtifactStore<P, Mutation>
      |                   -----------     --------------------------
18642 | where
18643 |     P: Clone + ToValue + FromValue + ArtifactPack + crate::os_schema_composition::ArtifactCompositionFields + Send + Sync + 'static,
      |                                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unsatisfied trait bound introduced here
help: consider restricting the type parameter to satisfy the trait bound
     -->   🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:14131:63
      |
14131 |     Mutation: Clone + ToValue + FromValue + self::Mutation<P>, P: os_schema_composition::ArtifactCompositionFields
      |                                                                +++++++++++++++++++++++++++++++++++++++++++++++++++


## E0599 — the method `close_owned_terminal_is_empty` exists for mutable reference `&mut os_store::component::ArtifactStore<P, M>`, but its trait bounds were not satisfied

error[E0599]: the method `close_owned_terminal_is_empty` exists for mutable reference `&mut os_store::component::ArtifactStore<P, M>`, but its trait bounds were not satisfied
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs:357:75
      |
  357 |             if matches!(step, SnapshotRetirementStep::Complete) && member.close_owned_terminal_is_empty() {
      |                                                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      |
     ::: 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:14128:1
      |
14128 | pub struct ArtifactStore<P, Mutation>
      | ------------------------------------- doesn't satisfy `_: SpaceMember`
      |
note: trait bound `P: os_schema_composition::ArtifactCompositionFields` was not satisfied
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:18643:53
      |
18641 | impl<P, Mutation> SpaceMember for ArtifactStore<P, Mutation>
      |                   -----------     --------------------------
18642 | where
18643 |     P: Clone + ToValue + FromValue + ArtifactPack + crate::os_schema_composition::ArtifactCompositionFields + Send + Sync + 'static,
      |                                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unsatisfied trait bound introduced here
help: consider restricting the type parameter to satisfy the trait bound
     -->   🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:14131:63
      |
14131 |     Mutation: Clone + ToValue + FromValue + self::Mutation<P>, P: os_schema_composition::ArtifactCompositionFields
      |                                                                +++++++++++++++++++++++++++++++++++++++++++++++++++


## E0277 — the trait bound `os_store::component::artifact_child_owner_schema::OwnerRef: protocol::ToValue` is not satisfied

error[E0277]: the trait bound `os_store::component::artifact_child_owner_schema::OwnerRef: protocol::ToValue` is not satisfied
  --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:37:39
   |
37 | #[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
   |                                       ^^^^^^^ unsatisfied trait bound
   |
help: the trait `protocol::ToValue` is not implemented for `os_store::component::artifact_child_owner_schema::OwnerRef`
  --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🪆️child/🏠️owner/🧬️schema/🦀️.rs:8:1
   |
 8 | pub struct OwnerRef {
   | ^^^^^^^^^^^^^^^^^^^
   = help: the following other types implement trait `protocol::ToValue`:
             &str
             ()
             (A, B)
             (A, B, C)
             [T; N]
             [T]
             bool
             f32
           and 309 others
   = note: required for `std::option::Option<os_store::component::artifact_child_owner_schema::OwnerRef>` to implement `protocol::ToValue`
   = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS/🗑️generated/derive-target/wasm32-wasip2/debug/deps/semio_framework_os_kernel-05be6083ccc28851.long-type-5969068628415841318.txt'
   = note: consider using `--verbose` to print the full type name to the console
   = note: this error originates in the derive macro `ToValue` (in Nightly builds, run with -Z macro-backtrace for more info)


## E0277 — the trait bound `os_store::component::artifact_child_owner_schema::OwnerRef: protocol::FromValue` is not satisfied

error[E0277]: the trait bound `os_store::component::artifact_child_owner_schema::OwnerRef: protocol::FromValue` is not satisfied
  --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:37:48
   |
37 | #[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
   |                                                ^^^^^^^^^ unsatisfied trait bound
   |
help: the trait `protocol::FromValue` is not implemented for `os_store::component::artifact_child_owner_schema::OwnerRef`
  --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🪆️child/🏠️owner/🧬️schema/🦀️.rs:8:1
   |
 8 | pub struct OwnerRef {
   | ^^^^^^^^^^^^^^^^^^^
   = help: the following other types implement trait `protocol::FromValue`:
             ()
             (A, B)
             (A, B, C)
             [T; N]
             bool
             f32
             f64
             i16
           and 292 others
   = note: required for `std::option::Option<os_store::component::artifact_child_owner_schema::OwnerRef>` to implement `protocol::FromValue`
   = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS/🗑️generated/derive-target/wasm32-wasip2/debug/deps/semio_framework_os_kernel-05be6083ccc28851.long-type-5969068628415841318.txt'
   = note: consider using `--verbose` to print the full type name to the console
   = note: this error originates in the derive macro `FromValue` (in Nightly builds, run with -Z macro-backtrace for more info)


## E0277 — the trait bound `os_store::component::artifact_child_owner_schema::OwnerRef: protocol::ToValue` is not satisfied

error[E0277]: the trait bound `os_store::component::artifact_child_owner_schema::OwnerRef: protocol::ToValue` is not satisfied
  --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:81:39
   |
81 | #[derive(Clone, Debug, PartialEq, Eq, ToValue)]
   |                                       ^^^^^^^ unsatisfied trait bound
   |
help: the trait `protocol::ToValue` is not implemented for `os_store::component::artifact_child_owner_schema::OwnerRef`
  --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🪆️child/🏠️owner/🧬️schema/🦀️.rs:8:1
   |
 8 | pub struct OwnerRef {
   | ^^^^^^^^^^^^^^^^^^^
   = help: the following other types implement trait `protocol::ToValue`:
             &str
             ()
             (A, B)
             (A, B, C)
             [T; N]
             [T]
             bool
             f32
           and 309 others
   = note: required for `std::option::Option<os_store::component::artifact_child_owner_schema::OwnerRef>` to implement `protocol::ToValue`
   = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS/🗑️generated/derive-target/wasm32-wasip2/debug/deps/semio_framework_os_kernel-05be6083ccc28851.long-type-5969068628415841318.txt'
   = note: consider using `--verbose` to print the full type name to the console
   = note: this error originates in the derive macro `ToValue` (in Nightly builds, run with -Z macro-backtrace for more info)


## E0599 — no method named `to_value` found for reference `&os_store::component::artifact_child_owner_schema::OwnerRef` in the current scope

error[E0599]: no method named `to_value` found for reference `&os_store::component::artifact_child_owner_schema::OwnerRef` in the current scope
    --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:2500:54
     |
2500 |             entries.push(("owner".to_string(), owner.to_value()));
     |                                                      ^^^^^^^^ method not found in `&os_store::component::artifact_child_owner_schema::OwnerRef`
     |
     = help: items from traits can only be used if the trait is in scope
help: some of the expressions' fields have a method of the same name
     |
2500 |             entries.push(("owner".to_string(), owner.child_id.to_value()));
     |                                                      +++++++++
2500 |             entries.push(("owner".to_string(), owner.parent.to_value()));
     |                                                      +++++++
2500 |             entries.push(("owner".to_string(), owner.slot.to_value()));
     |                                                      +++++
help: trait `DslField` which provides `to_value` is implemented but not in scope; perhaps you want to import it
     |
  20 + use crate::os_dsl::component::DslField;
     |


