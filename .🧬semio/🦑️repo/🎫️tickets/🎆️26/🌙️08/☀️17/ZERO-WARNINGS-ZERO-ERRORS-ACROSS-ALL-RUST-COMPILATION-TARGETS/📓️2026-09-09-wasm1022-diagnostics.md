# WASI 1022 Diagnostics

Completed receipt: {"status":101,"signal":null,"reason":"exit","counts":{"error":11,"warning":0},"cargoWarnings":[],"startedAt":"2026-09-09T10:55:00.858Z","finishedAt":"2026-09-09T10:57:53.532Z","packages":160}

## unused_qualifications — unnecessary qualification

error: unnecessary qualification
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🫧️transient/🧵️publication/🦀️.rs:141:27
    |
141 |     terminal_root: Option<std::sync::Weak<P>>,
    |                           ^^^^^^^^^^^^^^^^^^
    |
    = note: `-D unused-qualifications` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(unused_qualifications)]`
help: remove the unnecessary path segments
    |
141 -     terminal_root: Option<std::sync::Weak<P>>,
141 +     terminal_root: Option<Weak<P>>,
    |


## unused_qualifications — unnecessary qualification

error: unnecessary qualification
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🫧️transient/🧵️publication/🦀️.rs:157:100
    |
157 | ...   owner.generation_now() == self.terminal_generation && self.terminal_root.as_ref().and_then(std::sync::Weak::upgrade).is_some_...
    |                                                                                                  ^^^^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
157 -         owner.generation_now() == self.terminal_generation && self.terminal_root.as_ref().and_then(std::sync::Weak::upgrade).is_some_and(|root| Arc::ptr_eq(&root, &owner.current_root()))
157 +         owner.generation_now() == self.terminal_generation && self.terminal_root.as_ref().and_then(Weak::upgrade).is_some_and(|root| Arc::ptr_eq(&root, &owner.current_root()))
    |


## dead_code — struct `BoundedTransientPreparation` is never constructed

error: struct `BoundedTransientPreparation` is never constructed
 --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🫧️transient/🧵️publication/🦀️.rs:8:8
  |
8 | struct BoundedTransientPreparation<P, M> {
  |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `-D dead-code` implied by `-D warnings`
  = help: to override `-D warnings` add `#[expect(dead_code)]` or `#[allow(dead_code)]`


## dead_code — struct `BoundedTransientPreparationFactory` is never constructed

error: struct `BoundedTransientPreparationFactory` is never constructed
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🫧️transient/🧵️publication/🦀️.rs:17:8
   |
17 | struct BoundedTransientPreparationFactory<P, M>(std::marker::PhantomData<fn() -> (P, M)>);
   |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


## dead_code — struct `BoundedTransientRootRetirement` is never constructed

error: struct `BoundedTransientRootRetirement` is never constructed
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🫧️transient/🧵️publication/🦀️.rs:99:8
   |
99 | struct BoundedTransientRootRetirement<P> {
   |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


## dead_code — struct `BoundedTransientRootRetirementFactory` is never constructed

error: struct `BoundedTransientRootRetirementFactory` is never constructed
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🫧️transient/🧵️publication/🦀️.rs:120:8
    |
120 | struct BoundedTransientRootRetirementFactory<P>(std::marker::PhantomData<fn() -> P>);
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


## dead_code — struct `BoundedTransientStoreDisposer` is never constructed

error: struct `BoundedTransientStoreDisposer` is never constructed
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🫧️transient/🧵️publication/🦀️.rs:138:8
    |
138 | struct BoundedTransientStoreDisposer<P, M> {
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


## dead_code — method `owns_terminal` is never used

error: method `owns_terminal` is never used
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🫧️transient/🧵️publication/🦀️.rs:156:8
    |
151 | / impl<P, M> BoundedTransientStoreDisposer<P, M>
152 | | where
153 | |     P: Clone + Default,
154 | |     M: protocol::Mutation<P>,
    | |_____________________________- method in this implementation
155 |   {
156 |       fn owns_terminal(&self, owner: &store::TransientStore<P, M>) -> bool {
    |          ^^^^^^^^^^^^^


## dead_code — function `bounded_transient_preparation_factory` is never used

error: function `bounded_transient_preparation_factory` is never used
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🫧️transient/🧵️publication/🦀️.rs:192:8
    |
192 | pub fn bounded_transient_preparation_factory<P, M>() -> Arc<dyn store::ArtifactEphemeralOneItemPreparationFactory<P, M>>
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


## dead_code — function `bounded_transient_root_retirement_factory` is never used

error: function `bounded_transient_root_retirement_factory` is never used
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🫧️transient/🧵️publication/🦀️.rs:200:8
    |
200 | pub fn bounded_transient_root_retirement_factory<P>() -> Arc<dyn store::SnapshotRetirementFactory<P>>
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


## dead_code — function `bounded_transient_store_disposer` is never used

error: function `bounded_transient_store_disposer` is never used
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🫧️transient/🧵️publication/🦀️.rs:207:8
    |
207 | pub fn bounded_transient_store_disposer<P, M>() -> Box<dyn ArtifactOwnedDisposer<store::TransientStore<P, M>>>
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


