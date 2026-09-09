# WASI 1015 Diagnostics

Completed receipt: {"status":101,"signal":null,"reason":"exit","counts":{"error":8,"warning":0},"cargoWarnings":["warning: build failed, waiting for other jobs to finish..."],"startedAt":"2026-09-09T10:42:42.100Z","finishedAt":"2026-09-09T10:44:49.379Z","packages":160}

## E0432 — unresolved imports `super::transient_publication::bounded_transient_preparation_factory`, `super::transient_publication::bounded_transient_root_retirement_factory`, `super::transient_publication::bounded_transient_store_disposer`

error[E0432]: unresolved imports `super::transient_publication::bounded_transient_preparation_factory`, `super::transient_publication::bounded_transient_root_retirement_factory`, `super::transient_publication::bounded_transient_store_disposer`
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:274:44
    |
274 | ...on::{bounded_transient_preparation_factory, bounded_transient_root_retirement_factory, bounded_transient_store_disposer};
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `bounded_transient_store_disposer` in `component::transient_publication`
    |         |                                      |
    |         |                                      no `bounded_transient_root_retirement_factory` in `component::transient_publication`
    |         no `bounded_transient_preparation_factory` in `component::transient_publication`
    |
help: a similar name exists in the module
    |
274 -     pub use super::transient_publication::{bounded_transient_preparation_factory, bounded_transient_root_retirement_factory, bounded_transient_store_disposer};
274 +     pub use super::transient_publication::{bounded_transient_preparation_factory, bounded_transient_root_retirement_factory, transient_store_disposer};
    |


## E0432 — unresolved imports `super::window_transient::bounded_window_transient_preparation_factory`, `super::window_transient::bounded_window_transient_root_retirement_factory`, `super::window_transient::bounded_window_transient_store_disposer`

error[E0432]: unresolved imports `super::window_transient::bounded_window_transient_preparation_factory`, `super::window_transient::bounded_window_transient_root_retirement_factory`, `super::window_transient::bounded_window_transient_store_disposer`
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:279:9
    |
279 | ...   bounded_window_transient_preparation_factory, bounded_window_transient_root_retirement_factory, bounded_window_transient_store_disposer, W...
    |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `bounded_window_transient_store_disposer` in `component::window_transient`
    |       |                                             |
    |       |                                             no `bounded_window_transient_root_retirement_factory` in `component::window_transient`
    |       no `bounded_window_transient_preparation_factory` in `component::window_transient`


## clippy::multiple_bound_locations — bound is defined in more than one place

error: bound is defined in more than one place
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:6963:50
     |
6963 | ...   pub async fn assert_viewer_never_mutates<V: ArtifactViewer>()
     |                                                ^
6964 | ...   where
6965 | ...       V: ArtifactViewer<Presence = super::NoPresence, PresenceMutation = super::NoPresenceMutation, Transient = super::NoTrans...
     |           ^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#multiple_bound_locations
     = note: `-D clippy::multiple-bound-locations` implied by `-D warnings`
     = help: to override `-D warnings` add `#[allow(clippy::multiple_bound_locations)]`


## E0080 — evaluation panicked: assertion failed: size_of::<Store>() == size_of::<(Arc<NoTransient>, u64)>()

error[E0080]: evaluation panicked: assertion failed: size_of::<Store>() == size_of::<(Arc<NoTransient>, u64)>()
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🫧️transient/♻️retirement/🦀️.rs:10:15
   |
10 | const _: () = assert!(size_of::<Store>() == size_of::<(Arc<NoTransient>, u64)>());
   |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ evaluation of `component::transient_retirement::_` failed here


## E0063 — missing fields `maintenance_cursor` and `owners` in initializer of `component::window_transient::TypedWindowTransientStoreOwner<O>`

error[E0063]: missing fields `maintenance_cursor` and `owners` in initializer of `component::window_transient::TypedWindowTransientStoreOwner<O>`
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🪟️window/🫧️transient/🦀️.rs:290:56
    |
290 | ...WINDOW_KIND_ID, Box::new(TypedWindowTransientStoreOwner::<O> { partitions: BTreeMap::new(), retirement_cursor: None }));
    |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `maintenance_cursor` and `owners`


## E0308 — mismatched types

error[E0308]: mismatched types
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🪟️window/🫧️transient/🦀️.rs:307:17
    |
307 |         Ok(Some(owner.capture(window_id, self.document_generation)))
    |            ---- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `WindowTransientAuthority`, found `Result<WindowTransientAuthority, Fault>`
    |            |
    |            arguments to this enum variant are incorrect
    |
    = note: expected struct `component::window_transient::WindowTransientAuthority`
                 found enum `std::result::Result<component::window_transient::WindowTransientAuthority, dsl::Fault>`
help: the type constructed contains `std::result::Result<component::window_transient::WindowTransientAuthority, dsl::Fault>` due to the type of the argument passed
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🪟️window/🫧️transient/🦀️.rs:307:12
    |
307 |         Ok(Some(owner.capture(window_id, self.document_generation)))
    |            ^^^^^--------------------------------------------------^
    |                 |
    |                 this argument influences the type of `Some`
note: tuple variant defined here
   --> /Users/ueli/.rustup/toolchains/nightly-2026-07-07-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/option.rs:606:5
    |
606 |     Some(#[stable(feature = "rust1", since = "1.0.0")] T),
    |     ^^^^
help: use the `?` operator to extract the `std::result::Result<component::window_transient::WindowTransientAuthority, dsl::Fault>` value, propagating a `Result::Err` value to the caller
    |
307 |         Ok(Some(owner.capture(window_id, self.document_generation)?))
    |                                                                   +


## E0502 — cannot borrow `self.owners.preparation` as immutable because it is also borrowed as mutable

error[E0502]: cannot borrow `self.owners.preparation` as immutable because it is also borrowed as mutable
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🪟️window/🫧️transient/🦀️.rs:211:27
    |
210 |           let partition = self.partition(&window_id);
    |                           ---- mutable borrow occurs here
211 |           let preparation = self.owners.preparation.clone();
    |                             ^^^^^^^^^^^^^^^^^^^^^^^ immutable borrow occurs here
212 |           let retirement = self.owners.state_retirement.clone();
213 |           let publication = partition
    |  ___________________________-
214 | |             .store
    | |__________________- mutable borrow later used here


## E0502 — cannot borrow `self.owners.state_retirement` as immutable because it is also borrowed as mutable

error[E0502]: cannot borrow `self.owners.state_retirement` as immutable because it is also borrowed as mutable
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🪟️window/🫧️transient/🦀️.rs:212:26
    |
210 |           let partition = self.partition(&window_id);
    |                           ---- mutable borrow occurs here
211 |           let preparation = self.owners.preparation.clone();
212 |           let retirement = self.owners.state_retirement.clone();
    |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ immutable borrow occurs here
213 |           let publication = partition
    |  ___________________________-
214 | |             .store
    | |__________________- mutable borrow later used here


