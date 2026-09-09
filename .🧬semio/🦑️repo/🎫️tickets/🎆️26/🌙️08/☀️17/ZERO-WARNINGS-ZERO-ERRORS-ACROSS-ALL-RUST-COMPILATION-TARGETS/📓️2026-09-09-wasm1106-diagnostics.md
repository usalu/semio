# WASI 1106 Diagnostics

Completed receipt: {"status":101,"signal":null,"reason":"exit","counts":{"error":26,"warning":0},"cargoWarnings":["warning: build failed, waiting for other jobs to finish..."],"startedAt":"2026-09-09T13:13:53.754Z","finishedAt":"2026-09-09T13:16:21.805Z","packages":160}

## E0277 — the trait bound `<A as ArtifactApp>::Snapshot: ArtifactCompositionFields` is not satisfied

error[E0277]: the trait bound `<A as ArtifactApp>::Snapshot: ArtifactCompositionFields` is not satisfied
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:18632:54
      |
18632 |             let messages = SpaceMember::preview_wire(&self.store, &op_bytes).await;
      |                            ------------------------- ^^^^^^^^^^^ the trait `semio_framework_schema::ArtifactCompositionFields` is not implemented for `<A as component::app::ArtifactApp>::Snapshot`
      |                            |
      |                            required by a bound introduced by this call
      |
      = note: required for `ArtifactStore<<A as ArtifactApp>::Snapshot, ...>` to implement `dsl::SpaceMember`
      = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS/🗑️generated/derive-target/wasm32-wasip2/debug/deps/semio_framework_plugin-01555633483147ea.long-type-11059312915897378850.txt'
      = note: consider using `--verbose` to print the full type name to the console
help: consider further restricting the associated type
      |
18627 |         pub async fn preview(&self, ops: &[A::Mutation]) -> protocol::DispatchReport where <A as component::app::ArtifactApp>::Snapshot: semio_framework_schema::ArtifactCompositionFields {
      |                                                                                      +++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++


## E0277 — the trait bound `<A as ArtifactApp>::Snapshot: ArtifactCompositionFields` is not satisfied

error[E0277]: the trait bound `<A as ArtifactApp>::Snapshot: ArtifactCompositionFields` is not satisfied
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:18632:28
      |
18632 |             let messages = SpaceMember::preview_wire(&self.store, &op_bytes).await;
      |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `semio_framework_schema::ArtifactCompositionFields` is not implemented for `<A as component::app::ArtifactApp>::Snapshot`
      |
note: required by a bound in `<dsl::ArtifactStore<P, Mutation> as dsl::SpaceMember>`
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:18647:53
      |
18647 |     P: Clone + ToValue + FromValue + ArtifactPack + crate::os_schema_composition::ArtifactCompositionFields + Send + Sync + 'static,
      |                                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `<ArtifactStore<P, Mutation> as SpaceMember>`
      = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS/🗑️generated/derive-target/wasm32-wasip2/debug/deps/semio_framework_plugin-01555633483147ea.long-type-11059312915897378850.txt'
      = note: consider using `--verbose` to print the full type name to the console
help: consider further restricting the associated type
      |
18627 |         pub async fn preview(&self, ops: &[A::Mutation]) -> protocol::DispatchReport where <A as component::app::ArtifactApp>::Snapshot: semio_framework_schema::ArtifactCompositionFields {
      |                                                                                      +++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++


## E0277 — the trait bound `<A as ArtifactApp>::Snapshot: ArtifactCompositionFields` is not satisfied

error[E0277]: the trait bound `<A as ArtifactApp>::Snapshot: ArtifactCompositionFields` is not satisfied
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:18632:78
      |
18632 |             let messages = SpaceMember::preview_wire(&self.store, &op_bytes).await;
      |                                                                              ^^^^^ the trait `semio_framework_schema::ArtifactCompositionFields` is not implemented for `<A as component::app::ArtifactApp>::Snapshot`
      |
note: required by a bound in `<dsl::ArtifactStore<P, Mutation> as dsl::SpaceMember>`
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:18647:53
      |
18647 |     P: Clone + ToValue + FromValue + ArtifactPack + crate::os_schema_composition::ArtifactCompositionFields + Send + Sync + 'static,
      |                                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `<ArtifactStore<P, Mutation> as SpaceMember>`
      = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS/🗑️generated/derive-target/wasm32-wasip2/debug/deps/semio_framework_plugin-01555633483147ea.long-type-11059312915897378850.txt'
      = note: consider using `--verbose` to print the full type name to the console
help: consider further restricting the associated type
      |
18627 |         pub async fn preview(&self, ops: &[A::Mutation]) -> protocol::DispatchReport where <A as component::app::ArtifactApp>::Snapshot: semio_framework_schema::ArtifactCompositionFields {
      |                                                                                      +++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++


## E0277 — the trait bound `<A as ArtifactApp>::Snapshot: ArtifactCompositionFields` is not satisfied

error[E0277]: the trait bound `<A as ArtifactApp>::Snapshot: ArtifactCompositionFields` is not satisfied
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:19060:72
      |
19060 | ...  let receipt = self.composition.dispatch_group(&parent_ref, &mut self.store, &mut dispatches, parent_ops, Vec::new(), group_m...
      |                                     --------------              ^^^^^^^^^^^^^^^ the trait `semio_framework_schema::ArtifactCompositionFields` is not implemented for `<A as component::app::ArtifactApp>::Snapshot`
      |                                     |
      |                                     required by a bound introduced by this call
      |
      = note: required for `ArtifactStore<<A as ArtifactApp>::Snapshot, ...>` to implement `dsl::SpaceMember`
note: required by a bound in `dsl::TransactionCoordinator::dispatch_group`
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:20340:37
      |
20340 |     pub async fn dispatch_group<Mp: SpaceMember, Mc: SpaceMember + MemberFactory>(
      |                                     ^^^^^^^^^^^ required by this bound in `TransactionCoordinator::dispatch_group`
      = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS/🗑️generated/derive-target/wasm32-wasip2/debug/deps/semio_framework_plugin-01555633483147ea.long-type-11059312915897378850.txt'
      = note: consider using `--verbose` to print the full type name to the console
help: consider further restricting the associated type
      |
19006 |         ) -> Result<InvocationResult, Fault> where <A as component::app::ArtifactApp>::Snapshot: semio_framework_schema::ArtifactCompositionFields {
      |                                              +++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++


## E0277 — the trait bound `<A as ArtifactApp>::Snapshot: ArtifactCompositionFields` is not satisfied

error[E0277]: the trait bound `<A as ArtifactApp>::Snapshot: ArtifactCompositionFields` is not satisfied
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:19060:27
      |
19060 | ...pt = self.composition.dispatch_group(&parent_ref, &mut self.store, &mut dispatches, parent_ops, Vec::new(), group_meta).await....
      |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `semio_framework_schema::ArtifactCompositionFields` is not implemented for `<A as component::app::ArtifactApp>::Snapshot`
      |
      = note: required for `ArtifactStore<<A as ArtifactApp>::Snapshot, ...>` to implement `dsl::SpaceMember`
note: required by a bound in `dsl::TransactionCoordinator::dispatch_group`
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:20340:37
      |
20340 |     pub async fn dispatch_group<Mp: SpaceMember, Mc: SpaceMember + MemberFactory>(
      |                                     ^^^^^^^^^^^ required by this bound in `TransactionCoordinator::dispatch_group`
      = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS/🗑️generated/derive-target/wasm32-wasip2/debug/deps/semio_framework_plugin-01555633483147ea.long-type-11059312915897378850.txt'
      = note: consider using `--verbose` to print the full type name to the console
help: consider further restricting the associated type
      |
19006 |         ) -> Result<InvocationResult, Fault> where <A as component::app::ArtifactApp>::Snapshot: semio_framework_schema::ArtifactCompositionFields {
      |                                              +++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++


## E0277 — the trait bound `<A as ArtifactApp>::Snapshot: ArtifactCompositionFields` is not satisfied

error[E0277]: the trait bound `<A as ArtifactApp>::Snapshot: ArtifactCompositionFields` is not satisfied
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:19060:142
      |
19060 | ... dispatches, parent_ops, Vec::new(), group_meta).await.map_err(|error| plugin_sdk_fault(error.to_string()))?;
      |                                                     ^^^^^ the trait `semio_framework_schema::ArtifactCompositionFields` is not implemented for `<A as component::app::ArtifactApp>::Snapshot`
      |
      = note: required for `ArtifactStore<<A as ArtifactApp>::Snapshot, ...>` to implement `dsl::SpaceMember`
note: required by a bound in `dsl::TransactionCoordinator::dispatch_group`
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:20340:37
      |
20340 |     pub async fn dispatch_group<Mp: SpaceMember, Mc: SpaceMember + MemberFactory>(
      |                                     ^^^^^^^^^^^ required by this bound in `TransactionCoordinator::dispatch_group`
      = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS/🗑️generated/derive-target/wasm32-wasip2/debug/deps/semio_framework_plugin-01555633483147ea.long-type-11059312915897378850.txt'
      = note: consider using `--verbose` to print the full type name to the console
help: consider further restricting the associated type
      |
19006 |         ) -> Result<InvocationResult, Fault> where <A as component::app::ArtifactApp>::Snapshot: semio_framework_schema::ArtifactCompositionFields {
      |                                              +++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++


## E0277 — the trait bound `<A as ArtifactApp>::Snapshot: ArtifactCompositionFields` is not satisfied

error[E0277]: the trait bound `<A as ArtifactApp>::Snapshot: ArtifactCompositionFields` is not satisfied
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:19077:96
      |
19077 | ...      let compensation = CompositionCoordinator::undo_group(&parent_ref, &mut self.store, &mut members, &receipt.invocation_id...
      |                             ----------------------------------              ^^^^^^^^^^^^^^^ the trait `semio_framework_schema::ArtifactCompositionFields` is not implemented for `<A as component::app::ArtifactApp>::Snapshot`
      |                             |
      |                             required by a bound introduced by this call
      |
      = note: required for `ArtifactStore<<A as ArtifactApp>::Snapshot, ...>` to implement `dsl::SpaceMember`
note: required by a bound in `dsl::TransactionCoordinator::undo_group`
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:20578:33
      |
20578 |     pub async fn undo_group<Mp: SpaceMember, Mc: SpaceMember>(parent_ref: &crate::os_io::ArtifactRef, parent: &mut Mp, children: ...
      |                                 ^^^^^^^^^^^ required by this bound in `TransactionCoordinator::undo_group`
      = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS/🗑️generated/derive-target/wasm32-wasip2/debug/deps/semio_framework_plugin-01555633483147ea.long-type-11059312915897378850.txt'
      = note: consider using `--verbose` to print the full type name to the console
help: consider further restricting the associated type
      |
19006 |         ) -> Result<InvocationResult, Fault> where <A as component::app::ArtifactApp>::Snapshot: semio_framework_schema::ArtifactCompositionFields {
      |                                              +++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++


## E0277 — the trait bound `<A as ArtifactApp>::Snapshot: ArtifactCompositionFields` is not satisfied

error[E0277]: the trait bound `<A as ArtifactApp>::Snapshot: ArtifactCompositionFields` is not satisfied
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:19077:48
      |
19077 | ...sation = CompositionCoordinator::undo_group(&parent_ref, &mut self.store, &mut members, &receipt.invocation_id).await;
      |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `semio_framework_schema::ArtifactCompositionFields` is not implemented for `<A as component::app::ArtifactApp>::Snapshot`
      |
      = note: required for `ArtifactStore<<A as ArtifactApp>::Snapshot, ...>` to implement `dsl::SpaceMember`
note: required by a bound in `dsl::TransactionCoordinator::undo_group`
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:20578:33
      |
20578 |     pub async fn undo_group<Mp: SpaceMember, Mc: SpaceMember>(parent_ref: &crate::os_io::ArtifactRef, parent: &mut Mp, children: ...
      |                                 ^^^^^^^^^^^ required by this bound in `TransactionCoordinator::undo_group`
      = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS/🗑️generated/derive-target/wasm32-wasip2/debug/deps/semio_framework_plugin-01555633483147ea.long-type-11059312915897378850.txt'
      = note: consider using `--verbose` to print the full type name to the console
help: consider further restricting the associated type
      |
19006 |         ) -> Result<InvocationResult, Fault> where <A as component::app::ArtifactApp>::Snapshot: semio_framework_schema::ArtifactCompositionFields {
      |                                              +++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++


## E0277 — the trait bound `<A as ArtifactApp>::Snapshot: ArtifactCompositionFields` is not satisfied

error[E0277]: the trait bound `<A as ArtifactApp>::Snapshot: ArtifactCompositionFields` is not satisfied
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:19077:151
      |
19077 | ...elf.store, &mut members, &receipt.invocation_id).await;
      |                                                     ^^^^^ the trait `semio_framework_schema::ArtifactCompositionFields` is not implemented for `<A as component::app::ArtifactApp>::Snapshot`
      |
      = note: required for `ArtifactStore<<A as ArtifactApp>::Snapshot, ...>` to implement `dsl::SpaceMember`
note: required by a bound in `dsl::TransactionCoordinator::undo_group`
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:20578:33
      |
20578 |     pub async fn undo_group<Mp: SpaceMember, Mc: SpaceMember>(parent_ref: &crate::os_io::ArtifactRef, parent: &mut Mp, children: ...
      |                                 ^^^^^^^^^^^ required by this bound in `TransactionCoordinator::undo_group`
      = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS/🗑️generated/derive-target/wasm32-wasip2/debug/deps/semio_framework_plugin-01555633483147ea.long-type-11059312915897378850.txt'
      = note: consider using `--verbose` to print the full type name to the console
help: consider further restricting the associated type
      |
19006 |         ) -> Result<InvocationResult, Fault> where <A as component::app::ArtifactApp>::Snapshot: semio_framework_schema::ArtifactCompositionFields {
      |                                              +++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++


## E0277 — the trait bound `<A as ArtifactApp>::Snapshot: ArtifactCompositionFields` is not satisfied

error[E0277]: the trait bound `<A as ArtifactApp>::Snapshot: ArtifactCompositionFields` is not satisfied
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:19273:96
      |
19273 | ... = if action == "undo" { CompositionCoordinator::undo_group(&parent_ref, &mut self.store, &mut children, group_id).await } els...
      |                             ----------------------------------              ^^^^^^^^^^^^^^^ the trait `semio_framework_schema::ArtifactCompositionFields` is not implemented for `<A as component::app::ArtifactApp>::Snapshot`
      |                             |
      |                             required by a bound introduced by this call
      |
      = note: required for `ArtifactStore<<A as ArtifactApp>::Snapshot, ...>` to implement `dsl::SpaceMember`
note: required by a bound in `dsl::TransactionCoordinator::undo_group`
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:20578:33
      |
20578 |     pub async fn undo_group<Mp: SpaceMember, Mc: SpaceMember>(parent_ref: &crate::os_io::ArtifactRef, parent: &mut Mp, children: ...
      |                                 ^^^^^^^^^^^ required by this bound in `TransactionCoordinator::undo_group`
      = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS/🗑️generated/derive-target/wasm32-wasip2/debug/deps/semio_framework_plugin-01555633483147ea.long-type-11059312915897378850.txt'
      = note: consider using `--verbose` to print the full type name to the console
help: consider further restricting the associated type
      |
19247 |         async fn dispatch_group_history_action(&mut self, action: &str, group_id: &str, meta: &ActionMeta) -> Result<InvocationResult, Fault> where <A as component::app::ArtifactApp>::Snapshot: semio_framework_schema::ArtifactCompositionFields {
      |                                                                                                                                               +++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++


## E0277 — the trait bound `<A as ArtifactApp>::Snapshot: ArtifactCompositionFields` is not satisfied

error[E0277]: the trait bound `<A as ArtifactApp>::Snapshot: ArtifactCompositionFields` is not satisfied
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:19273:48
      |
19273 | ...on == "undo" { CompositionCoordinator::undo_group(&parent_ref, &mut self.store, &mut children, group_id).await } else { Compos...
      |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `semio_framework_schema::ArtifactCompositionFields` is not implemented for `<A as component::app::ArtifactApp>::Snapshot`
      |
      = note: required for `ArtifactStore<<A as ArtifactApp>::Snapshot, ...>` to implement `dsl::SpaceMember`
note: required by a bound in `dsl::TransactionCoordinator::undo_group`
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:20578:33
      |
20578 |     pub async fn undo_group<Mp: SpaceMember, Mc: SpaceMember>(parent_ref: &crate::os_io::ArtifactRef, parent: &mut Mp, children: ...
      |                                 ^^^^^^^^^^^ required by this bound in `TransactionCoordinator::undo_group`
      = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS/🗑️generated/derive-target/wasm32-wasip2/debug/deps/semio_framework_plugin-01555633483147ea.long-type-11059312915897378850.txt'
      = note: consider using `--verbose` to print the full type name to the console
help: consider further restricting the associated type
      |
19247 |         async fn dispatch_group_history_action(&mut self, action: &str, group_id: &str, meta: &ActionMeta) -> Result<InvocationResult, Fault> where <A as component::app::ArtifactApp>::Snapshot: semio_framework_schema::ArtifactCompositionFields {
      |                                                                                                                                               +++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++


## E0277 — the trait bound `<A as ArtifactApp>::Snapshot: ArtifactCompositionFields` is not satisfied

error[E0277]: the trait bound `<A as ArtifactApp>::Snapshot: ArtifactCompositionFields` is not satisfied
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:19273:138
      |
19273 | ...t_ref, &mut self.store, &mut children, group_id).await } else { CompositionCoordinator::redo_group(&parent_ref, &mut self.stor...
      |                                                     ^^^^^ the trait `semio_framework_schema::ArtifactCompositionFields` is not implemented for `<A as component::app::ArtifactApp>::Snapshot`
      |
      = note: required for `ArtifactStore<<A as ArtifactApp>::Snapshot, ...>` to implement `dsl::SpaceMember`
note: required by a bound in `dsl::TransactionCoordinator::undo_group`
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:20578:33
      |
20578 |     pub async fn undo_group<Mp: SpaceMember, Mc: SpaceMember>(parent_ref: &crate::os_io::ArtifactRef, parent: &mut Mp, children: ...
      |                                 ^^^^^^^^^^^ required by this bound in `TransactionCoordinator::undo_group`
      = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS/🗑️generated/derive-target/wasm32-wasip2/debug/deps/semio_framework_plugin-01555633483147ea.long-type-11059312915897378850.txt'
      = note: consider using `--verbose` to print the full type name to the console
help: consider further restricting the associated type
      |
19247 |         async fn dispatch_group_history_action(&mut self, action: &str, group_id: &str, meta: &ActionMeta) -> Result<InvocationResult, Fault> where <A as component::app::ArtifactApp>::Snapshot: semio_framework_schema::ArtifactCompositionFields {
      |                                                                                                                                               +++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++


## E0277 — the trait bound `<A as ArtifactApp>::Snapshot: ArtifactCompositionFields` is not satisfied

error[E0277]: the trait bound `<A as ArtifactApp>::Snapshot: ArtifactCompositionFields` is not satisfied
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:19273:201
      |
19273 | ...group_id).await } else { CompositionCoordinator::redo_group(&parent_ref, &mut self.store, &mut children, group_id).await };
      |                             ----------------------------------              ^^^^^^^^^^^^^^^ the trait `semio_framework_schema::ArtifactCompositionFields` is not implemented for `<A as component::app::ArtifactApp>::Snapshot`
      |                             |
      |                             required by a bound introduced by this call
      |
      = note: required for `ArtifactStore<<A as ArtifactApp>::Snapshot, ...>` to implement `dsl::SpaceMember`
note: required by a bound in `dsl::TransactionCoordinator::redo_group`
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:20594:33
      |
20594 |     pub async fn redo_group<Mp: SpaceMember, Mc: SpaceMember>(parent_ref: &crate::os_io::ArtifactRef, parent: &mut Mp, children: ...
      |                                 ^^^^^^^^^^^ required by this bound in `TransactionCoordinator::redo_group`
      = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS/🗑️generated/derive-target/wasm32-wasip2/debug/deps/semio_framework_plugin-01555633483147ea.long-type-11059312915897378850.txt'
      = note: consider using `--verbose` to print the full type name to the console
help: consider further restricting the associated type
      |
19247 |         async fn dispatch_group_history_action(&mut self, action: &str, group_id: &str, meta: &ActionMeta) -> Result<InvocationResult, Fault> where <A as component::app::ArtifactApp>::Snapshot: semio_framework_schema::ArtifactCompositionFields {
      |                                                                                                                                               +++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++


## E0277 — the trait bound `<A as ArtifactApp>::Snapshot: ArtifactCompositionFields` is not satisfied

error[E0277]: the trait bound `<A as ArtifactApp>::Snapshot: ArtifactCompositionFields` is not satisfied
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:19273:153
      |
19273 | ...await } else { CompositionCoordinator::redo_group(&parent_ref, &mut self.store, &mut children, group_id).await };
      |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `semio_framework_schema::ArtifactCompositionFields` is not implemented for `<A as component::app::ArtifactApp>::Snapshot`
      |
      = note: required for `ArtifactStore<<A as ArtifactApp>::Snapshot, ...>` to implement `dsl::SpaceMember`
note: required by a bound in `dsl::TransactionCoordinator::redo_group`
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:20594:33
      |
20594 |     pub async fn redo_group<Mp: SpaceMember, Mc: SpaceMember>(parent_ref: &crate::os_io::ArtifactRef, parent: &mut Mp, children: ...
      |                                 ^^^^^^^^^^^ required by this bound in `TransactionCoordinator::redo_group`
      = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS/🗑️generated/derive-target/wasm32-wasip2/debug/deps/semio_framework_plugin-01555633483147ea.long-type-11059312915897378850.txt'
      = note: consider using `--verbose` to print the full type name to the console
help: consider further restricting the associated type
      |
19247 |         async fn dispatch_group_history_action(&mut self, action: &str, group_id: &str, meta: &ActionMeta) -> Result<InvocationResult, Fault> where <A as component::app::ArtifactApp>::Snapshot: semio_framework_schema::ArtifactCompositionFields {
      |                                                                                                                                               +++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++


## E0277 — the trait bound `<A as ArtifactApp>::Snapshot: ArtifactCompositionFields` is not satisfied

error[E0277]: the trait bound `<A as ArtifactApp>::Snapshot: ArtifactCompositionFields` is not satisfied
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:19273:243
      |
19273 | ...t_ref, &mut self.store, &mut children, group_id).await };
      |                                                     ^^^^^ the trait `semio_framework_schema::ArtifactCompositionFields` is not implemented for `<A as component::app::ArtifactApp>::Snapshot`
      |
      = note: required for `ArtifactStore<<A as ArtifactApp>::Snapshot, ...>` to implement `dsl::SpaceMember`
note: required by a bound in `dsl::TransactionCoordinator::redo_group`
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:20594:33
      |
20594 |     pub async fn redo_group<Mp: SpaceMember, Mc: SpaceMember>(parent_ref: &crate::os_io::ArtifactRef, parent: &mut Mp, children: ...
      |                                 ^^^^^^^^^^^ required by this bound in `TransactionCoordinator::redo_group`
      = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS/🗑️generated/derive-target/wasm32-wasip2/debug/deps/semio_framework_plugin-01555633483147ea.long-type-11059312915897378850.txt'
      = note: consider using `--verbose` to print the full type name to the console
help: consider further restricting the associated type
      |
19247 |         async fn dispatch_group_history_action(&mut self, action: &str, group_id: &str, meta: &ActionMeta) -> Result<InvocationResult, Fault> where <A as component::app::ArtifactApp>::Snapshot: semio_framework_schema::ArtifactCompositionFields {
      |                                                                                                                                               +++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++


## E0599 — the method `tail_group_id` exists for struct `ArtifactStore<<A as ArtifactApp>::Snapshot, ...>`, but its trait bounds were not satisfied

error[E0599]: the method `tail_group_id` exists for struct `ArtifactStore<<A as ArtifactApp>::Snapshot, ...>`, but its trait bounds were not satisfied
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:20016:65
      |
20016 | ... group_id = if action == "undo" { self.store.tail_group_id().await } else { self.store.redo_tail().await.and_then(|(_, group_i...
      |                                                 ^^^^^^^^^^^^^ method cannot be called on `ArtifactStore<<A as ArtifactApp>::Snapshot, ...>` due to unsatisfied trait bounds
      |
     ::: 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:14128:1
      |
14128 | pub struct ArtifactStore<P, Mutation>
      | ------------------------------------- doesn't satisfy `_: SpaceMember`
      |
      = note: the following trait bounds were not satisfied:
              `<A as component::app::ArtifactApp>::Snapshot: semio_framework_schema::ArtifactCompositionFields`
              which is required by `dsl::ArtifactStore<<A as component::app::ArtifactApp>::Snapshot, <A as component::app::ArtifactApp>::Mutation>: dsl::SpaceMember`
      = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS/🗑️generated/derive-target/wasm32-wasip2/debug/deps/semio_framework_plugin-01555633483147ea.long-type-10382391721020070921.txt'
      = note: consider using `--verbose` to print the full type name to the console


## E0599 — the method `redo_tail` exists for struct `ArtifactStore<<A as ArtifactApp>::Snapshot, ...>`, but its trait bounds were not satisfied

error[E0599]: the method `redo_tail` exists for struct `ArtifactStore<<A as ArtifactApp>::Snapshot, ...>`, but its trait bounds were not satisfied
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:20016:107
      |
20016 | ...re.redo_tail().await.and_then(|(_, group_id)| group_id) };
      |       ^^^^^^^^^ method cannot be called on `ArtifactStore<<A as ArtifactApp>::Snapshot, ...>` due to unsatisfied trait bounds
      |
     ::: 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:14128:1
      |
14128 | pub struct ArtifactStore<P, Mutation>
      | ------------------------------------- doesn't satisfy `_: SpaceMember`
      |
      = note: the following trait bounds were not satisfied:
              `<A as component::app::ArtifactApp>::Snapshot: semio_framework_schema::ArtifactCompositionFields`
              which is required by `dsl::ArtifactStore<<A as component::app::ArtifactApp>::Snapshot, <A as component::app::ArtifactApp>::Mutation>: dsl::SpaceMember`
      = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS/🗑️generated/derive-target/wasm32-wasip2/debug/deps/semio_framework_plugin-01555633483147ea.long-type-10382391721020070921.txt'
      = note: consider using `--verbose` to print the full type name to the console


## E0277 — the size for values of type `str` cannot be known at compilation time

error[E0277]: the size for values of type `str` cannot be known at compilation time
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:20017:24
      |
20017 |                 if let Some(group_id) = group_id {
      |                        ^^^^^^^^^^^^^^ doesn't have a size known at compile-time
      |
      = help: the trait `std::marker::Sized` is not implemented for `str`
note: required by a bound in `std::prelude::v1::Some`
     --> /Users/ueli/.rustup/toolchains/nightly-2026-07-07-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/option.rs:598:17
      |
  598 | pub enum Option<T> {
      |                 ^ required by this bound in `std::prelude::v1::Some`
...
  606 |     Some(#[stable(feature = "rust1", since = "1.0.0")] T),
      |     ---- required by a bound in this tuple variant


## E0599 — the method `stamp_tail_group_id` exists for struct `ArtifactStore<<A as ArtifactApp>::Snapshot, ...>`, but its trait bounds were not satisfied

error[E0599]: the method `stamp_tail_group_id` exists for struct `ArtifactStore<<A as ArtifactApp>::Snapshot, ...>`, but its trait bounds were not satisfied
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:23197:24
      |
23197 |             self.store.stamp_tail_group_id(&txn_id).await.map_err(|error| Self::transaction_fault(FaultOrigin::Plugin, "transacti...
      |                        ^^^^^^^^^^^^^^^^^^^ method cannot be called on `ArtifactStore<<A as ArtifactApp>::Snapshot, ...>` due to unsatisfied trait bounds
      |
     ::: 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:14128:1
      |
14128 | pub struct ArtifactStore<P, Mutation>
      | ------------------------------------- doesn't satisfy `_: SpaceMember`
      |
      = note: the following trait bounds were not satisfied:
              `<A as component::app::ArtifactApp>::Snapshot: semio_framework_schema::ArtifactCompositionFields`
              which is required by `dsl::ArtifactStore<<A as component::app::ArtifactApp>::Snapshot, <A as component::app::ArtifactApp>::Mutation>: dsl::SpaceMember`
      = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS/🗑️generated/derive-target/wasm32-wasip2/debug/deps/semio_framework_plugin-01555633483147ea.long-type-10382391721020070921.txt'
      = note: consider using `--verbose` to print the full type name to the console


## E0599 — the method `stamp_tail_origin` exists for struct `ArtifactStore<<A as ArtifactApp>::Snapshot, ...>`, but its trait bounds were not satisfied

error[E0599]: the method `stamp_tail_origin` exists for struct `ArtifactStore<<A as ArtifactApp>::Snapshot, ...>`, but its trait bounds were not satisfied
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:23198:24
      |
23198 |             self.store.stamp_tail_origin(origin).await.map_err(|error| Self::transaction_fault(FaultOrigin::Plugin, "transaction....
      |                        ^^^^^^^^^^^^^^^^^ method cannot be called on `ArtifactStore<<A as ArtifactApp>::Snapshot, ...>` due to unsatisfied trait bounds
      |
     ::: 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:14128:1
      |
14128 | pub struct ArtifactStore<P, Mutation>
      | ------------------------------------- doesn't satisfy `_: SpaceMember`
      |
      = note: the following trait bounds were not satisfied:
              `<A as component::app::ArtifactApp>::Snapshot: semio_framework_schema::ArtifactCompositionFields`
              which is required by `dsl::ArtifactStore<<A as component::app::ArtifactApp>::Snapshot, <A as component::app::ArtifactApp>::Mutation>: dsl::SpaceMember`
      = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS/🗑️generated/derive-target/wasm32-wasip2/debug/deps/semio_framework_plugin-01555633483147ea.long-type-10382391721020070921.txt'
      = note: consider using `--verbose` to print the full type name to the console


## E0599 — the method `tail_group_id` exists for struct `ArtifactStore<<A as ArtifactApp>::Snapshot, ...>`, but its trait bounds were not satisfied

error[E0599]: the method `tail_group_id` exists for struct `ArtifactStore<<A as ArtifactApp>::Snapshot, ...>`, but its trait bounds were not satisfied
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:23215:27
      |
23215 |             if self.store.tail_group_id().await.as_deref() != Some(group_id) {
      |                           ^^^^^^^^^^^^^ method cannot be called on `ArtifactStore<<A as ArtifactApp>::Snapshot, ...>` due to unsatisfied trait bounds
      |
     ::: 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:14128:1
      |
14128 | pub struct ArtifactStore<P, Mutation>
      | ------------------------------------- doesn't satisfy `_: SpaceMember`
      |
      = note: the following trait bounds were not satisfied:
              `<A as component::app::ArtifactApp>::Snapshot: semio_framework_schema::ArtifactCompositionFields`
              which is required by `dsl::ArtifactStore<<A as component::app::ArtifactApp>::Snapshot, <A as component::app::ArtifactApp>::Mutation>: dsl::SpaceMember`
      = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS/🗑️generated/derive-target/wasm32-wasip2/debug/deps/semio_framework_plugin-01555633483147ea.long-type-10382391721020070921.txt'
      = note: consider using `--verbose` to print the full type name to the console


## E0599 — the method `undo` exists for struct `ArtifactStore<<A as ArtifactApp>::Snapshot, ...>`, but its trait bounds were not satisfied

error[E0599]: the method `undo` exists for struct `ArtifactStore<<A as ArtifactApp>::Snapshot, ...>`, but its trait bounds were not satisfied
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:23218:24
      |
23218 |             self.store.undo().await.map_err(|error| plugin_sdk_fault(format!("{error:?}")))?;
      |                        ^^^^ method cannot be called on `ArtifactStore<<A as ArtifactApp>::Snapshot, ...>` due to unsatisfied trait bounds
      |
     ::: 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:14128:1
      |
14128 | pub struct ArtifactStore<P, Mutation>
      | ------------------------------------- doesn't satisfy `_: SpaceMember`
      |
      = note: the following trait bounds were not satisfied:
              `<A as component::app::ArtifactApp>::Snapshot: semio_framework_schema::ArtifactCompositionFields`
              which is required by `dsl::ArtifactStore<<A as component::app::ArtifactApp>::Snapshot, <A as component::app::ArtifactApp>::Mutation>: dsl::SpaceMember`
      = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS/🗑️generated/derive-target/wasm32-wasip2/debug/deps/semio_framework_plugin-01555633483147ea.long-type-10382391721020070921.txt'
      = note: consider using `--verbose` to print the full type name to the console


## E0599 — the method `redo_tail` exists for struct `ArtifactStore<<A as ArtifactApp>::Snapshot, ...>`, but its trait bounds were not satisfied

error[E0599]: the method `redo_tail` exists for struct `ArtifactStore<<A as ArtifactApp>::Snapshot, ...>`, but its trait bounds were not satisfied
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:23224:30
      |
23224 |             match self.store.redo_tail().await {
      |                              ^^^^^^^^^ method cannot be called on `ArtifactStore<<A as ArtifactApp>::Snapshot, ...>` due to unsatisfied trait bounds
      |
     ::: 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:14128:1
      |
14128 | pub struct ArtifactStore<P, Mutation>
      | ------------------------------------- doesn't satisfy `_: SpaceMember`
      |
      = note: the following trait bounds were not satisfied:
              `<A as component::app::ArtifactApp>::Snapshot: semio_framework_schema::ArtifactCompositionFields`
              which is required by `dsl::ArtifactStore<<A as component::app::ArtifactApp>::Snapshot, <A as component::app::ArtifactApp>::Mutation>: dsl::SpaceMember`
      = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS/🗑️generated/derive-target/wasm32-wasip2/debug/deps/semio_framework_plugin-01555633483147ea.long-type-10382391721020070921.txt'
      = note: consider using `--verbose` to print the full type name to the console


## E0599 — the method `redo` exists for struct `ArtifactStore<<A as ArtifactApp>::Snapshot, ...>`, but its trait bounds were not satisfied

error[E0599]: the method `redo` exists for struct `ArtifactStore<<A as ArtifactApp>::Snapshot, ...>`, but its trait bounds were not satisfied
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:23226:32
      |
23226 |                     self.store.redo().await.map_err(|error| plugin_sdk_fault(format!("{error:?}")))?;
      |                                ^^^^ method cannot be called on `ArtifactStore<<A as ArtifactApp>::Snapshot, ...>` due to unsatisfied trait bounds
      |
     ::: 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:14128:1
      |
14128 | pub struct ArtifactStore<P, Mutation>
      | ------------------------------------- doesn't satisfy `_: SpaceMember`
      |
      = note: the following trait bounds were not satisfied:
              `<A as component::app::ArtifactApp>::Snapshot: semio_framework_schema::ArtifactCompositionFields`
              which is required by `dsl::ArtifactStore<<A as component::app::ArtifactApp>::Snapshot, <A as component::app::ArtifactApp>::Mutation>: dsl::SpaceMember`
      = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS/🗑️generated/derive-target/wasm32-wasip2/debug/deps/semio_framework_plugin-01555633483147ea.long-type-10382391721020070921.txt'
      = note: consider using `--verbose` to print the full type name to the console


## E0277 — the trait bound `P: semio_framework_schema::ArtifactCompositionFields` is not satisfied

error[E0277]: the trait bound `P: semio_framework_schema::ArtifactCompositionFields` is not satisfied
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:12815:49
      |
12815 |             match SpaceMember::close_owned_step(owner, maximum_items.min(1), maximum_bytes).map_err(plugin_sdk_fault)? {
      |                   ----------------------------- ^^^^^ the trait `semio_framework_schema::ArtifactCompositionFields` is not implemented for `P`
      |                   |
      |                   required by a bound introduced by this call
      |
      = note: required for `dsl::ArtifactStore<P, Mutation>` to implement `dsl::SpaceMember`
help: consider further restricting type parameter `P` with trait `ArtifactCompositionFields`
      |
12811 |         P: Clone + protocol::ToValue + protocol::FromValue + ArtifactPack + Send + Sync + 'static + semio_framework_schema::ArtifactCompositionFields,
      |                                                                                                   +++++++++++++++++++++++++++++++++++++++++++++++++++


## E0277 — the trait bound `P: semio_framework_schema::ArtifactCompositionFields` is not satisfied

error[E0277]: the trait bound `P: semio_framework_schema::ArtifactCompositionFields` is not satisfied
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:12824:56
      |
12824 |             SpaceMember::close_owned_terminal_is_empty(owner)
      |             ------------------------------------------ ^^^^^ the trait `semio_framework_schema::ArtifactCompositionFields` is not implemented for `P`
      |             |
      |             required by a bound introduced by this call
      |
      = note: required for `dsl::ArtifactStore<P, Mutation>` to implement `dsl::SpaceMember`
help: consider further restricting type parameter `P` with trait `ArtifactCompositionFields`
      |
12811 |         P: Clone + protocol::ToValue + protocol::FromValue + ArtifactPack + Send + Sync + 'static + semio_framework_schema::ArtifactCompositionFields,
      |                                                                                                   +++++++++++++++++++++++++++++++++++++++++++++++++++


