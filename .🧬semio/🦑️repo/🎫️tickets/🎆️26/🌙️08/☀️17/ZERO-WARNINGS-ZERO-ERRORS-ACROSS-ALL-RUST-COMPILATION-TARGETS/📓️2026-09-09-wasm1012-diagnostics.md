# WASI 1012 Diagnostics

Completed receipt: {"status":101,"signal":null,"reason":"exit","counts":{"error":2,"warning":0},"cargoWarnings":[],"startedAt":"2026-09-09T10:40:23.721Z","finishedAt":"2026-09-09T10:41:09.606Z","packages":160}

## E0277 — `P` cannot be sent between threads safely

error[E0277]: `P` cannot be sent between threads safely
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:1172:17
      |
 1172 |     match owner.close_step(super::ArtifactStoreOneItemGrant { maximum_items: grant.maximum_items.min(1), maximum_bytes: grant.max...
      |                 ^^^^^^^^^^ `P` cannot be sent between threads safely
      |
note: required by a bound in `os_store::component::ArtifactStoreBatchPublication::<P, Mutation>::close_step`
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:14026:12
      |
14024 |     pub fn close_step(&mut self, grant: ArtifactStoreOneItemGrant) -> Result<SnapshotRetirementStep, String>
      |            ---------- required by a bound in this associated function
14025 |     where
14026 |         P: Send + Sync + 'static,
      |            ^^^^ required by this bound in `ArtifactStoreBatchPublication::<P, Mutation>::close_step`
help: consider restricting type parameter `P` with trait `Send`
      |
 1169 | fn close_assembly_publication<P: std::marker::Send, Mutation>(publication: &mut Option<super::ArtifactStoreBatchPublication<P, Mutation>>, grant: super::ArtifactStoreOneItemGrant) -> Result<bool, DurableOwnedGroupDecisionError> {
      |                                +++++++++++++++++++


## E0277 — `P` cannot be shared between threads safely

error[E0277]: `P` cannot be shared between threads safely
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:1172:17
      |
 1172 |     match owner.close_step(super::ArtifactStoreOneItemGrant { maximum_items: grant.maximum_items.min(1), maximum_bytes: grant.max...
      |                 ^^^^^^^^^^ `P` cannot be shared between threads safely
      |
note: required by a bound in `os_store::component::ArtifactStoreBatchPublication::<P, Mutation>::close_step`
     --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:14026:19
      |
14024 |     pub fn close_step(&mut self, grant: ArtifactStoreOneItemGrant) -> Result<SnapshotRetirementStep, String>
      |            ---------- required by a bound in this associated function
14025 |     where
14026 |         P: Send + Sync + 'static,
      |                   ^^^^ required by this bound in `ArtifactStoreBatchPublication::<P, Mutation>::close_step`
help: consider restricting type parameter `P` with trait `Sync`
      |
 1169 | fn close_assembly_publication<P: std::marker::Sync, Mutation>(publication: &mut Option<super::ArtifactStoreBatchPublication<P, Mutation>>, grant: super::ArtifactStoreOneItemGrant) -> Result<bool, DurableOwnedGroupDecisionError> {
      |                                +++++++++++++++++++


