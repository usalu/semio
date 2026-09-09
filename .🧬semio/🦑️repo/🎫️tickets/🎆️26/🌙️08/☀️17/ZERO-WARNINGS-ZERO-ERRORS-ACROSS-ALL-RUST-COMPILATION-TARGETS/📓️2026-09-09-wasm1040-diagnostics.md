# WASI 1037 Diagnostics

Completed receipt: {"status":101,"signal":null,"reason":"exit","counts":{"error":7,"warning":0},"cargoWarnings":["warning: build failed, waiting for other jobs to finish..."],"startedAt":"2026-09-09T11:21:54.666Z","finishedAt":"2026-09-09T11:27:10.549Z","packages":160}

## dead_code — constant `BOUNDED_WINDOW_TRANSIENT_RETIREMENT_BYTES` is never used

error: constant `BOUNDED_WINDOW_TRANSIENT_RETIREMENT_BYTES` is never used
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🪟️window/🫧️transient/🦀️.rs:36:7
   |
36 | const BOUNDED_WINDOW_TRANSIENT_RETIREMENT_BYTES: usize = store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES;
   |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `-D dead-code` implied by `-D warnings`
   = help: to override `-D warnings` add `#[expect(dead_code)]` or `#[allow(dead_code)]`


## dead_code — struct `BoundedWindowTransientValueRetirement` is never constructed

error: struct `BoundedWindowTransientValueRetirement` is never constructed
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🪟️window/🫧️transient/🦀️.rs:38:8
   |
38 | struct BoundedWindowTransientValueRetirement<T> {
   |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


## dead_code — struct `BoundedWindowTransientValueRetirementFactory` is never constructed

error: struct `BoundedWindowTransientValueRetirementFactory` is never constructed
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🪟️window/🫧️transient/🦀️.rs:66:8
   |
66 | struct BoundedWindowTransientValueRetirementFactory<T>(std::marker::PhantomData<fn() -> T>);
   |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


## dead_code — struct `BoundedWindowTransientPreparationFactory` is never constructed

error: struct `BoundedWindowTransientPreparationFactory` is never constructed
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🪟️window/🫧️transient/🦀️.rs:80:8
   |
80 | struct BoundedWindowTransientPreparationFactory<P, M> {
   |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


## dead_code — struct `BoundedWindowTransientPreparation` is never constructed

error: struct `BoundedWindowTransientPreparation` is never constructed
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🪟️window/🫧️transient/🦀️.rs:113:8
    |
113 | struct BoundedWindowTransientPreparation<P, M> {
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


## dead_code — method `drive_retirement` is never used

error: method `drive_retirement` is never used
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🪟️window/🫧️transient/🦀️.rs:127:8
    |
126 | impl<P, M> BoundedWindowTransientPreparation<P, M> {
    | -------------------------------------------------- method in this implementation
127 |     fn drive_retirement(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<Option<store::SnapshotRetirementStep>, String> {
    |        ^^^^^^^^^^^^^^^^


## dead_code — function `bounded_window_transient_store_owners` is never used

error: function `bounded_window_transient_store_owners` is never used
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🪟️window/🫧️transient/🦀️.rs:229:8
    |
229 | pub fn bounded_window_transient_store_owners<O: WindowTransientOwner>() -> WindowTransientOwnerBundle<O::State, O::Mutation> {
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


