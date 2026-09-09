# WASI 1030 Diagnostics

Completed receipt: {"status":101,"signal":null,"reason":"exit","counts":{"error":11,"warning":0},"cargoWarnings":["warning: build failed, waiting for other jobs to finish..."],"startedAt":"2026-09-09T11:03:50.114Z","finishedAt":"2026-09-09T11:10:40.977Z","packages":160}

## E0425 — cannot find function `base64_standard` in the crate root

error[E0425]: cannot find function `base64_standard` in the crate root
  --> ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs:25:107
   |
25 |     ImageView { width: snapshot.width, height: snapshot.height, mime: "image/jpeg".into(), base64: crate::base64_standard(&bytes) }
   |                                                                                                           ^^^^^^^^^^^^^^^ not found in the crate root
   |
help: consider importing this function
   |
 4 + use semio_s_artifact_stdio_contract::base64_standard;
   |
help: if you import `base64_standard`, refer to it directly
   |
25 -     ImageView { width: snapshot.width, height: snapshot.height, mime: "image/jpeg".into(), base64: crate::base64_standard(&bytes) }
25 +     ImageView { width: snapshot.width, height: snapshot.height, mime: "image/jpeg".into(), base64: base64_standard(&bytes) }
   |


## E0425 — cannot find function `base64_standard` in the crate root

error[E0425]: cannot find function `base64_standard` in the crate root
  --> ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs:25:107
   |
25 |     ImageView { width: snapshot.width, height: snapshot.height, mime: "image/jpeg".into(), base64: crate::base64_standard(&bytes) }
   |                                                                                                           ^^^^^^^^^^^^^^^ not found in the crate root
   |
help: consider importing this function
   |
 4 + use semio_s_artifact_stdio_contract::base64_standard;
   |
help: if you import `base64_standard`, refer to it directly
   |
25 -     ImageView { width: snapshot.width, height: snapshot.height, mime: "image/jpeg".into(), base64: crate::base64_standard(&bytes) }
25 +     ImageView { width: snapshot.width, height: snapshot.height, mime: "image/jpeg".into(), base64: base64_standard(&bytes) }
   |


## E0433 — cannot find `NoTransientStoreDisposer` in `semio_framework_plugin`

error[E0433]: cannot find `NoTransientStoreDisposer` in `semio_framework_plugin`
    --> ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:6797:47
     |
6797 |         Some(Box::new(semio_framework_plugin::NoTransientStoreDisposer::new()))
     |                                               ^^^^^^^^^^^^^^^^^^^^^^^^ could not find `NoTransientStoreDisposer` in `semio_framework_plugin`


## E0425 — cannot find function `bounded_window_transient_store_owners` in crate `semio_framework_plugin`

error[E0425]: cannot find function `bounded_window_transient_store_owners` in crate `semio_framework_plugin`
   --> ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🦀️.rs:147:33
    |
147 |         semio_framework_plugin::bounded_window_transient_store_owners::<Self>()
    |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
   ::: 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🪟️window/🎚️config/🦀️.rs:201:1
    |
201 | pub fn bounded_window_config_store_owners<O: WindowConfigOwner>() -> store::MemberStoreOwners<O::State, O::Mutation> {
    | -------------------------------------------------------------------------------------------------------------------- similarly named function `bounded_window_config_store_owners` defined here
    |
help: a function with a similar name exists
    |
147 -         semio_framework_plugin::bounded_window_transient_store_owners::<Self>()
147 +         semio_framework_plugin::bounded_window_config_store_owners::<Self>()
    |


## clippy::vec_init_then_push — calls to `push` immediately after creation

error: calls to `push` immediately after creation
   --> ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs:118:9
    |
118 | /         let mut mutations = Vec::with_capacity(22);
119 | |         mutations.push(En1994Mutation::ChangeFYMpa(change_f_y_mpa::ChangeFYMpa { new_f_y_mpa: snapshot.f_y_mpa }));
120 | |         mutations.push(En1994Mutation::ChangeVEdKn(change_v_ed_kn::ChangeVEdKn { new_v_ed_kn: snapshot.v_ed_kn }));
121 | |         mutations.push(En1994Mutation::ChangeDMm(change_d_mm::ChangeDMm { new_d_mm: snapshot.d_mm }));
...   |
139 | |         mutations.push(En1994Mutation::ChangeDeckType(change_deck_type::ChangeDeckType { new_deck_type: snapshot.deck_type.clone(...
140 | |         mutations.push(En1994Mutation::ChangeVLRd(change_v_l_rd::ChangeVLRd { new_v_l_rd: snapshot.v_l_rd }));
    | |______________________________________________________________________________________________________________^ help: consider using the `vec![]` macro: `let mutations = vec![..];`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#vec_init_then_push
    = note: `-D clippy::vec-init-then-push` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::vec_init_then_push)]`


## clippy::needless_update — struct update has no effect, all the fields in the struct have already been specified

error: struct update has no effect, all the fields in the struct have already been specified
  --> ✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:33:92
   |
33 |         Self { schema: snapshot.schema, catalog_generation: snapshot.catalog_generation, ..Self::default() }
   |                                                                                            ^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_update
   = note: `-D clippy::needless-update` implied by `-D warnings`
   = help: to override `-D warnings` add `#[allow(clippy::needless_update)]`


## clippy::too_many_arguments — this function has too many arguments (8/7)

error: this function has too many arguments (8/7)
   --> ✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:120:1
    |
120 | / fn home_retained_reduce(
121 | |     command: &HomeCommand,
122 | |     snapshot: &SHomeSnapshot,
123 | |     config: &HomeConfig,
...   |
128 | |     operation: &AppOperationContext,
129 | | ) -> Result<Emit<crate::standards::v1::subsets::any::schema::mutations::text::SHomeMutation, HomeConfigMutation, NoDraftMutation>, Fault> {
    | |_________________________________________________________________________________________________________________________________________^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#too_many_arguments
    = note: `-D clippy::too-many-arguments` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::too_many_arguments)]`


## clippy::unnecessary_filter_map — this `.filter_map(..)` can be written more simply using `.map(..)`

error: this `.filter_map(..)` can be written more simply using `.map(..)`
  --> ✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:93:54
   |
93 | ...   let config_mutations = events.iter().filter_map(|event| Some(pack::to_json_string(event))).map(|event_json| HomeConfigMutation...
   |                                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#unnecessary_filter_map
   = note: `-D clippy::unnecessary-filter-map` implied by `-D warnings`
   = help: to override `-D warnings` add `#[allow(clippy::unnecessary_filter_map)]`


## clippy::too_many_arguments — this function has too many arguments (8/7)

error: this function has too many arguments (8/7)
   --> ✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:177:1
    |
177 | / fn block5d_retained_reduce(
178 | |     command: &Block5dCommand,
179 | |     snapshot: &Block5dSnapshot,
180 | |     config: &Block5dConfig,
...   |
185 | |     operation: &AppOperationContext,
186 | | ) -> Result<Emit<Block5dMutation, Block5dConfigMutation, NoDraftMutation>, Fault> {
    | |_________________________________________________________________________________^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#too_many_arguments
    = note: `-D clippy::too-many-arguments` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::too_many_arguments)]`


## clippy::derivable_impls — this `impl` can be derived

error: this `impl` can be derived
  --> ✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs:64:1
   |
64 | / impl Default for Block5dConfig {
65 | |     fn default() -> Self {
66 | |         Self {}
67 | |     }
68 | | }
   | |_^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#derivable_impls
   = note: `-D clippy::derivable-impls` implied by `-D warnings`
   = help: to override `-D warnings` add `#[allow(clippy::derivable_impls)]`
help: replace the manual implementation with a derive attribute
   |
18 + #[derive(Default)]
19 | pub struct Block5dConfig {}
   |


## clippy::iter_nth — called `.iter().nth()` on a `Vec`

error: called `.iter().nth()` on a `Vec`
  --> ✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/📦️packages/🦀️rust/../.././././././././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📊️csv/🔖️rfc4180/✳️any/🦀️.rs:13:29
   |
13 |     let first_data_record = from.records.iter().nth(usize::from(from.has_header));
   |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#iter_nth
   = note: `-D clippy::iter-nth` implied by `-D warnings`
   = help: to override `-D warnings` add `#[allow(clippy::iter_nth)]`
help: `get` is equivalent but more concise
   |
13 -     let first_data_record = from.records.iter().nth(usize::from(from.has_header));
13 +     let first_data_record = from.records.get(usize::from(from.has_header));
   |


