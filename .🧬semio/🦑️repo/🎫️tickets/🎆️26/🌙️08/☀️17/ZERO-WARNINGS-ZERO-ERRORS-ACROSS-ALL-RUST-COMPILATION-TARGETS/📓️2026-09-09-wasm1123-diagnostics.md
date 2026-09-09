# WASI 1123 Diagnostics

Completed receipt: {"status":101,"signal":null,"reason":"exit","counts":{"error":10,"warning":0},"cargoWarnings":["warning: build failed, waiting for other jobs to finish..."],"startedAt":"2026-09-09T13:42:30.140Z","finishedAt":"2026-09-09T13:55:11.144Z","packages":160}

## E0425 — cannot find value `MEMBER_OPEN_IDENTITY_BYTES` in crate `store`

error[E0425]: cannot find value `MEMBER_OPEN_IDENTITY_BYTES` in crate `store`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:16939:61
      |
16939 |             if maximum_items == 0 || maximum_bytes < store::MEMBER_OPEN_IDENTITY_BYTES {
      |                                                             ^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in `store`
      |
help: consider importing one of these constants
      |
  278 +     use crate::dsl::member_open::MEMBER_OPEN_IDENTITY_BYTES;
      |
  278 +     use semio_framework_os_kernel::member_open::MEMBER_OPEN_IDENTITY_BYTES;
      |
help: if you import `MEMBER_OPEN_IDENTITY_BYTES`, refer to it directly
      |
16939 -             if maximum_items == 0 || maximum_bytes < store::MEMBER_OPEN_IDENTITY_BYTES {
16939 +             if maximum_items == 0 || maximum_bytes < MEMBER_OPEN_IDENTITY_BYTES {
      |


## E0277 — `<M as dsl::MemberFactory>::Open` cannot be sent between threads safely

error[E0277]: `<M as dsl::MemberFactory>::Open` cannot be sent between threads safely
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:23870:18
      |
23870 |             self.load_document_pack(&store::ArtifactPackFiles { pack: pack.to_vec(), spr: spr.to_vec(), ops: String::new() }).await
      |                  ^^^^^^^^^^^^^^^^^^ `<M as dsl::MemberFactory>::Open` cannot be sent between threads safely
      |
      = help: within `[MaybeUninit<(u64, ActiveArtifactStoreReplacement<..., ..., ...>)>]`, the trait `std::marker::Send` is not implemented for `<M as dsl::MemberFactory>::Open`
note: required because it appears within the type `std::option::Option<<M as dsl::MemberFactory>::Open>`
     --> /Users/ueli/.rustup/toolchains/nightly-2026-07-07-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/option.rs:598:10
      |
  598 | pub enum Option<T> {
      |          ^^^^^^
note: required because it appears within the type `std::mem::MaybeDangling<std::option::Option<<M as dsl::MemberFactory>::Open>>`
     --> /Users/ueli/.rustup/toolchains/nightly-2026-07-07-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/mem/maybe_dangling.rs:73:12
      |
   73 | pub struct MaybeDangling<P: ?Sized>(P);
      |            ^^^^^^^^^^^^^
note: required because it appears within the type `std::mem::ManuallyDrop<std::option::Option<<M as dsl::MemberFactory>::Open>>`
     --> /Users/ueli/.rustup/toolchains/nightly-2026-07-07-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/mem/manually_drop.rs:158:12
      |
  158 | pub struct ManuallyDrop<T: ?Sized> {
      |            ^^^^^^^^^^^^
note: required because it appears within the type `ActiveArtifactStoreReplacement<<A as ArtifactApp>::Snapshot, ..., ...>`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:16704:12
      |
16704 |     struct ActiveArtifactStoreReplacement<P, Mutation, M>
      |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      = note: required because it appears within the type `(u64, ActiveArtifactStoreReplacement<..., ..., ...>)`
note: required because it appears within the type `MaybeDangling<(u64, ActiveArtifactStoreReplacement<..., ..., ...>)>`
     --> /Users/ueli/.rustup/toolchains/nightly-2026-07-07-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/mem/maybe_dangling.rs:73:12
      |
   73 | pub struct MaybeDangling<P: ?Sized>(P);
      |            ^^^^^^^^^^^^^
note: required because it appears within the type `ManuallyDrop<(u64, ActiveArtifactStoreReplacement<..., ..., ...>)>`
     --> /Users/ueli/.rustup/toolchains/nightly-2026-07-07-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/mem/manually_drop.rs:158:12
      |
  158 | pub struct ManuallyDrop<T: ?Sized> {
      |            ^^^^^^^^^^^^
note: required because it appears within the type `MaybeUninit<(u64, ActiveArtifactStoreReplacement<..., ..., ...>)>`
     --> /Users/ueli/.rustup/toolchains/nightly-2026-07-07-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/mem/maybe_uninit.rs:355:11
      |
  355 | pub union MaybeUninit<T> {
      |           ^^^^^^^^^^^
      = note: required because it appears within the type `[MaybeUninit<(u64, ActiveArtifactStoreReplacement<..., ..., ...>)>]`
      = note: required for `Unique<[MaybeUninit<(u64, ...)>]>` to implement `std::marker::Send`
note: required because it appears within the type `Box<[MaybeUninit<(u64, ...)>]>`
     --> /Users/ueli/.rustup/toolchains/nightly-2026-07-07-aarch64-apple-darwin/lib/rustlib/src/rust/library/alloc/src/boxed.rs:236:12
      |
  236 | pub struct Box<
      |            ^^^
note: required because it appears within the type `ArtifactFixedRegistry<ActiveArtifactStoreReplacement<..., ..., ...>>`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:15045:23
      |
15045 |     pub(crate) struct ArtifactFixedRegistry<T> {
      |                       ^^^^^^^^^^^^^^^^^^^^^
note: required because it appears within the type `component::app::VcsArtifactApp<A, M>`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:17250:16
      |
17250 |     pub struct VcsArtifactApp<A: ArtifactApp, M: SpaceMember + MemberFactory = NoMembers> {
      |                ^^^^^^^^^^^^^^
note: required by a bound in `component::app::PluginApp::load_document_pack`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:11121:26
      |
11121 |     pub trait PluginApp: Send + 'static {
      |                          ^^^^ required by this bound in `PluginApp::load_document_pack`
...
11332 |         async fn load_document_pack(&mut self, files: &store::ArtifactPackFiles) -> Result<(), Fault>;
      |                  ------------------ required by a bound in this associated function
      = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS/🗑️generated/derive-target/wasm32-wasip2/debug/deps/semio_framework_plugin-01555633483147ea.long-type-5462339182651029531.txt'
      = note: consider using `--verbose` to print the full type name to the console
help: consider further restricting the associated type
      |
23866 |         async fn hydrate_document_lane(&mut self, pack: &[u8], spr: &[u8]) -> Result<(), Fault> where <M as dsl::MemberFactory>::Open: std::marker::Send {
      |                                                                                                 ++++++++++++++++++++++++++++++++++++++++++++++++++++++++


## E0277 — `<M as dsl::MemberFactory>::Open` cannot be sent between threads safely

error[E0277]: `<M as dsl::MemberFactory>::Open` cannot be sent between threads safely
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:23877:18
      |
23877 |             self.load_config_pack(&store::ArtifactPackFiles { pack: pack.to_vec(), spr: spr.to_vec(), ops: String::new() }).await
      |                  ^^^^^^^^^^^^^^^^ `<M as dsl::MemberFactory>::Open` cannot be sent between threads safely
      |
      = help: within `[MaybeUninit<(u64, ActiveArtifactStoreReplacement<..., ..., ...>)>]`, the trait `std::marker::Send` is not implemented for `<M as dsl::MemberFactory>::Open`
note: required because it appears within the type `std::option::Option<<M as dsl::MemberFactory>::Open>`
     --> /Users/ueli/.rustup/toolchains/nightly-2026-07-07-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/option.rs:598:10
      |
  598 | pub enum Option<T> {
      |          ^^^^^^
note: required because it appears within the type `std::mem::MaybeDangling<std::option::Option<<M as dsl::MemberFactory>::Open>>`
     --> /Users/ueli/.rustup/toolchains/nightly-2026-07-07-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/mem/maybe_dangling.rs:73:12
      |
   73 | pub struct MaybeDangling<P: ?Sized>(P);
      |            ^^^^^^^^^^^^^
note: required because it appears within the type `std::mem::ManuallyDrop<std::option::Option<<M as dsl::MemberFactory>::Open>>`
     --> /Users/ueli/.rustup/toolchains/nightly-2026-07-07-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/mem/manually_drop.rs:158:12
      |
  158 | pub struct ManuallyDrop<T: ?Sized> {
      |            ^^^^^^^^^^^^
note: required because it appears within the type `ActiveArtifactStoreReplacement<<A as ArtifactApp>::Snapshot, ..., ...>`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:16704:12
      |
16704 |     struct ActiveArtifactStoreReplacement<P, Mutation, M>
      |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      = note: required because it appears within the type `(u64, ActiveArtifactStoreReplacement<..., ..., ...>)`
note: required because it appears within the type `MaybeDangling<(u64, ActiveArtifactStoreReplacement<..., ..., ...>)>`
     --> /Users/ueli/.rustup/toolchains/nightly-2026-07-07-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/mem/maybe_dangling.rs:73:12
      |
   73 | pub struct MaybeDangling<P: ?Sized>(P);
      |            ^^^^^^^^^^^^^
note: required because it appears within the type `ManuallyDrop<(u64, ActiveArtifactStoreReplacement<..., ..., ...>)>`
     --> /Users/ueli/.rustup/toolchains/nightly-2026-07-07-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/mem/manually_drop.rs:158:12
      |
  158 | pub struct ManuallyDrop<T: ?Sized> {
      |            ^^^^^^^^^^^^
note: required because it appears within the type `MaybeUninit<(u64, ActiveArtifactStoreReplacement<..., ..., ...>)>`
     --> /Users/ueli/.rustup/toolchains/nightly-2026-07-07-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/mem/maybe_uninit.rs:355:11
      |
  355 | pub union MaybeUninit<T> {
      |           ^^^^^^^^^^^
      = note: required because it appears within the type `[MaybeUninit<(u64, ActiveArtifactStoreReplacement<..., ..., ...>)>]`
      = note: required for `Unique<[MaybeUninit<(u64, ...)>]>` to implement `std::marker::Send`
note: required because it appears within the type `Box<[MaybeUninit<(u64, ...)>]>`
     --> /Users/ueli/.rustup/toolchains/nightly-2026-07-07-aarch64-apple-darwin/lib/rustlib/src/rust/library/alloc/src/boxed.rs:236:12
      |
  236 | pub struct Box<
      |            ^^^
note: required because it appears within the type `ArtifactFixedRegistry<ActiveArtifactStoreReplacement<..., ..., ...>>`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:15045:23
      |
15045 |     pub(crate) struct ArtifactFixedRegistry<T> {
      |                       ^^^^^^^^^^^^^^^^^^^^^
note: required because it appears within the type `component::app::VcsArtifactApp<A, M>`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:17250:16
      |
17250 |     pub struct VcsArtifactApp<A: ArtifactApp, M: SpaceMember + MemberFactory = NoMembers> {
      |                ^^^^^^^^^^^^^^
note: required by a bound in `component::app::PluginApp::load_config_pack`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:11121:26
      |
11121 |     pub trait PluginApp: Send + 'static {
      |                          ^^^^ required by this bound in `PluginApp::load_config_pack`
...
11288 |         async fn load_config_pack(&mut self, files: &store::ArtifactPackFiles) -> Result<(), Fault>;
      |                  ---------------- required by a bound in this associated function
      = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS/🗑️generated/derive-target/wasm32-wasip2/debug/deps/semio_framework_plugin-01555633483147ea.long-type-5462339182651029531.txt'
      = note: consider using `--verbose` to print the full type name to the console
help: consider further restricting the associated type
      |
23873 |         async fn hydrate_config_lane(&mut self, pack: &[u8], spr: &[u8]) -> Result<(), Fault> where <M as dsl::MemberFactory>::Open: std::marker::Send {
      |                                                                                               ++++++++++++++++++++++++++++++++++++++++++++++++++++++++


## E0277 — `<M as dsl::MemberFactory>::Open` cannot be sent between threads safely

error[E0277]: `<M as dsl::MemberFactory>::Open` cannot be sent between threads safely
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:23897:76
      |
23897 | ...esence_interaction(self.app_id().await, &interaction_state, &hover_specs, &selection_specs).await;
      |                            ^^^^^^ `<M as dsl::MemberFactory>::Open` cannot be sent between threads safely
      |
      = help: within `[MaybeUninit<(u64, ActiveArtifactStoreReplacement<..., ..., ...>)>]`, the trait `std::marker::Send` is not implemented for `<M as dsl::MemberFactory>::Open`
note: required because it appears within the type `std::option::Option<<M as dsl::MemberFactory>::Open>`
     --> /Users/ueli/.rustup/toolchains/nightly-2026-07-07-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/option.rs:598:10
      |
  598 | pub enum Option<T> {
      |          ^^^^^^
note: required because it appears within the type `std::mem::MaybeDangling<std::option::Option<<M as dsl::MemberFactory>::Open>>`
     --> /Users/ueli/.rustup/toolchains/nightly-2026-07-07-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/mem/maybe_dangling.rs:73:12
      |
   73 | pub struct MaybeDangling<P: ?Sized>(P);
      |            ^^^^^^^^^^^^^
note: required because it appears within the type `std::mem::ManuallyDrop<std::option::Option<<M as dsl::MemberFactory>::Open>>`
     --> /Users/ueli/.rustup/toolchains/nightly-2026-07-07-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/mem/manually_drop.rs:158:12
      |
  158 | pub struct ManuallyDrop<T: ?Sized> {
      |            ^^^^^^^^^^^^
note: required because it appears within the type `ActiveArtifactStoreReplacement<<A as ArtifactApp>::Snapshot, ..., ...>`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:16704:12
      |
16704 |     struct ActiveArtifactStoreReplacement<P, Mutation, M>
      |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      = note: required because it appears within the type `(u64, ActiveArtifactStoreReplacement<..., ..., ...>)`
note: required because it appears within the type `MaybeDangling<(u64, ActiveArtifactStoreReplacement<..., ..., ...>)>`
     --> /Users/ueli/.rustup/toolchains/nightly-2026-07-07-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/mem/maybe_dangling.rs:73:12
      |
   73 | pub struct MaybeDangling<P: ?Sized>(P);
      |            ^^^^^^^^^^^^^
note: required because it appears within the type `ManuallyDrop<(u64, ActiveArtifactStoreReplacement<..., ..., ...>)>`
     --> /Users/ueli/.rustup/toolchains/nightly-2026-07-07-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/mem/manually_drop.rs:158:12
      |
  158 | pub struct ManuallyDrop<T: ?Sized> {
      |            ^^^^^^^^^^^^
note: required because it appears within the type `MaybeUninit<(u64, ActiveArtifactStoreReplacement<..., ..., ...>)>`
     --> /Users/ueli/.rustup/toolchains/nightly-2026-07-07-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/mem/maybe_uninit.rs:355:11
      |
  355 | pub union MaybeUninit<T> {
      |           ^^^^^^^^^^^
      = note: required because it appears within the type `[MaybeUninit<(u64, ActiveArtifactStoreReplacement<..., ..., ...>)>]`
      = note: required for `Unique<[MaybeUninit<(u64, ...)>]>` to implement `std::marker::Send`
note: required because it appears within the type `Box<[MaybeUninit<(u64, ...)>]>`
     --> /Users/ueli/.rustup/toolchains/nightly-2026-07-07-aarch64-apple-darwin/lib/rustlib/src/rust/library/alloc/src/boxed.rs:236:12
      |
  236 | pub struct Box<
      |            ^^^
note: required because it appears within the type `ArtifactFixedRegistry<ActiveArtifactStoreReplacement<..., ..., ...>>`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:15045:23
      |
15045 |     pub(crate) struct ArtifactFixedRegistry<T> {
      |                       ^^^^^^^^^^^^^^^^^^^^^
note: required because it appears within the type `component::app::VcsArtifactApp<A, M>`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:17250:16
      |
17250 |     pub struct VcsArtifactApp<A: ArtifactApp, M: SpaceMember + MemberFactory = NoMembers> {
      |                ^^^^^^^^^^^^^^
note: required by a bound in `component::app::PluginApp::app_id`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:11121:26
      |
11121 |     pub trait PluginApp: Send + 'static {
      |                          ^^^^ required by this bound in `PluginApp::app_id`
...
11133 |         async fn app_id(&self) -> &str;
      |                  ------ required by a bound in this associated function
      = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS/🗑️generated/derive-target/wasm32-wasip2/debug/deps/semio_framework_plugin-01555633483147ea.long-type-5462339182651029531.txt'
      = note: consider using `--verbose` to print the full type name to the console
help: consider further restricting the associated type
      |
23893 |         async fn ephemeral_snapshot(&self) -> EphemeralSnapshot where <M as dsl::MemberFactory>::Open: std::marker::Send {
      |                                                                 ++++++++++++++++++++++++++++++++++++++++++++++++++++++++


## E0277 — `<M as dsl::MemberFactory>::Open` cannot be sent between threads safely

error[E0277]: `<M as dsl::MemberFactory>::Open` cannot be sent between threads safely
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:24280:35
      |
24280 |                 let handle = self.submit_media_export(port).await?;
      |                                   ^^^^^^^^^^^^^^^^^^^ `<M as dsl::MemberFactory>::Open` cannot be sent between threads safely
      |
      = help: within `[MaybeUninit<(u64, ActiveArtifactStoreReplacement<..., ..., ...>)>]`, the trait `std::marker::Send` is not implemented for `<M as dsl::MemberFactory>::Open`
note: required because it appears within the type `std::option::Option<<M as dsl::MemberFactory>::Open>`
     --> /Users/ueli/.rustup/toolchains/nightly-2026-07-07-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/option.rs:598:10
      |
  598 | pub enum Option<T> {
      |          ^^^^^^
note: required because it appears within the type `std::mem::MaybeDangling<std::option::Option<<M as dsl::MemberFactory>::Open>>`
     --> /Users/ueli/.rustup/toolchains/nightly-2026-07-07-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/mem/maybe_dangling.rs:73:12
      |
   73 | pub struct MaybeDangling<P: ?Sized>(P);
      |            ^^^^^^^^^^^^^
note: required because it appears within the type `std::mem::ManuallyDrop<std::option::Option<<M as dsl::MemberFactory>::Open>>`
     --> /Users/ueli/.rustup/toolchains/nightly-2026-07-07-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/mem/manually_drop.rs:158:12
      |
  158 | pub struct ManuallyDrop<T: ?Sized> {
      |            ^^^^^^^^^^^^
note: required because it appears within the type `ActiveArtifactStoreReplacement<<A as ArtifactApp>::Snapshot, ..., ...>`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:16704:12
      |
16704 |     struct ActiveArtifactStoreReplacement<P, Mutation, M>
      |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      = note: required because it appears within the type `(u64, ActiveArtifactStoreReplacement<..., ..., ...>)`
note: required because it appears within the type `MaybeDangling<(u64, ActiveArtifactStoreReplacement<..., ..., ...>)>`
     --> /Users/ueli/.rustup/toolchains/nightly-2026-07-07-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/mem/maybe_dangling.rs:73:12
      |
   73 | pub struct MaybeDangling<P: ?Sized>(P);
      |            ^^^^^^^^^^^^^
note: required because it appears within the type `ManuallyDrop<(u64, ActiveArtifactStoreReplacement<..., ..., ...>)>`
     --> /Users/ueli/.rustup/toolchains/nightly-2026-07-07-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/mem/manually_drop.rs:158:12
      |
  158 | pub struct ManuallyDrop<T: ?Sized> {
      |            ^^^^^^^^^^^^
note: required because it appears within the type `MaybeUninit<(u64, ActiveArtifactStoreReplacement<..., ..., ...>)>`
     --> /Users/ueli/.rustup/toolchains/nightly-2026-07-07-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/mem/maybe_uninit.rs:355:11
      |
  355 | pub union MaybeUninit<T> {
      |           ^^^^^^^^^^^
      = note: required because it appears within the type `[MaybeUninit<(u64, ActiveArtifactStoreReplacement<..., ..., ...>)>]`
      = note: required for `Unique<[MaybeUninit<(u64, ...)>]>` to implement `std::marker::Send`
note: required because it appears within the type `Box<[MaybeUninit<(u64, ...)>]>`
     --> /Users/ueli/.rustup/toolchains/nightly-2026-07-07-aarch64-apple-darwin/lib/rustlib/src/rust/library/alloc/src/boxed.rs:236:12
      |
  236 | pub struct Box<
      |            ^^^
note: required because it appears within the type `ArtifactFixedRegistry<ActiveArtifactStoreReplacement<..., ..., ...>>`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:15045:23
      |
15045 |     pub(crate) struct ArtifactFixedRegistry<T> {
      |                       ^^^^^^^^^^^^^^^^^^^^^
note: required because it appears within the type `component::app::VcsArtifactApp<A, M>`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:17250:16
      |
17250 |     pub struct VcsArtifactApp<A: ArtifactApp, M: SpaceMember + MemberFactory = NoMembers> {
      |                ^^^^^^^^^^^^^^
note: required by a bound in `component::app::PluginApp::submit_media_export`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:11121:26
      |
11121 |     pub trait PluginApp: Send + 'static {
      |                          ^^^^ required by this bound in `PluginApp::submit_media_export`
...
11369 |         async fn submit_media_export(&mut self, _port: &str) -> Result<ArtifactMediaExportHandle, MediaError> {
      |                  ------------------- required by a bound in this associated function
      = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS/🗑️generated/derive-target/wasm32-wasip2/debug/deps/semio_framework_plugin-01555633483147ea.long-type-5462339182651029531.txt'
      = note: consider using `--verbose` to print the full type name to the console
help: consider further restricting the associated type
      |
24277 |         async fn export_media(&mut self, port: &str) -> Result<Media, MediaError> where <M as dsl::MemberFactory>::Open: std::marker::Send {
      |                                                                                   ++++++++++++++++++++++++++++++++++++++++++++++++++++++++


## E0277 — `<M as dsl::MemberFactory>::Open` cannot be sent between threads safely

error[E0277]: `<M as dsl::MemberFactory>::Open` cannot be sent between threads safely
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:24282:32
      |
24282 |                     match self.poll_media_export(&handle).await? {
      |                                ^^^^^^^^^^^^^^^^^ `<M as dsl::MemberFactory>::Open` cannot be sent between threads safely
      |
      = help: within `[MaybeUninit<(u64, ActiveArtifactStoreReplacement<..., ..., ...>)>]`, the trait `std::marker::Send` is not implemented for `<M as dsl::MemberFactory>::Open`
note: required because it appears within the type `std::option::Option<<M as dsl::MemberFactory>::Open>`
     --> /Users/ueli/.rustup/toolchains/nightly-2026-07-07-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/option.rs:598:10
      |
  598 | pub enum Option<T> {
      |          ^^^^^^
note: required because it appears within the type `std::mem::MaybeDangling<std::option::Option<<M as dsl::MemberFactory>::Open>>`
     --> /Users/ueli/.rustup/toolchains/nightly-2026-07-07-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/mem/maybe_dangling.rs:73:12
      |
   73 | pub struct MaybeDangling<P: ?Sized>(P);
      |            ^^^^^^^^^^^^^
note: required because it appears within the type `std::mem::ManuallyDrop<std::option::Option<<M as dsl::MemberFactory>::Open>>`
     --> /Users/ueli/.rustup/toolchains/nightly-2026-07-07-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/mem/manually_drop.rs:158:12
      |
  158 | pub struct ManuallyDrop<T: ?Sized> {
      |            ^^^^^^^^^^^^
note: required because it appears within the type `ActiveArtifactStoreReplacement<<A as ArtifactApp>::Snapshot, ..., ...>`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:16704:12
      |
16704 |     struct ActiveArtifactStoreReplacement<P, Mutation, M>
      |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      = note: required because it appears within the type `(u64, ActiveArtifactStoreReplacement<..., ..., ...>)`
note: required because it appears within the type `MaybeDangling<(u64, ActiveArtifactStoreReplacement<..., ..., ...>)>`
     --> /Users/ueli/.rustup/toolchains/nightly-2026-07-07-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/mem/maybe_dangling.rs:73:12
      |
   73 | pub struct MaybeDangling<P: ?Sized>(P);
      |            ^^^^^^^^^^^^^
note: required because it appears within the type `ManuallyDrop<(u64, ActiveArtifactStoreReplacement<..., ..., ...>)>`
     --> /Users/ueli/.rustup/toolchains/nightly-2026-07-07-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/mem/manually_drop.rs:158:12
      |
  158 | pub struct ManuallyDrop<T: ?Sized> {
      |            ^^^^^^^^^^^^
note: required because it appears within the type `MaybeUninit<(u64, ActiveArtifactStoreReplacement<..., ..., ...>)>`
     --> /Users/ueli/.rustup/toolchains/nightly-2026-07-07-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/mem/maybe_uninit.rs:355:11
      |
  355 | pub union MaybeUninit<T> {
      |           ^^^^^^^^^^^
      = note: required because it appears within the type `[MaybeUninit<(u64, ActiveArtifactStoreReplacement<..., ..., ...>)>]`
      = note: required for `Unique<[MaybeUninit<(u64, ...)>]>` to implement `std::marker::Send`
note: required because it appears within the type `Box<[MaybeUninit<(u64, ...)>]>`
     --> /Users/ueli/.rustup/toolchains/nightly-2026-07-07-aarch64-apple-darwin/lib/rustlib/src/rust/library/alloc/src/boxed.rs:236:12
      |
  236 | pub struct Box<
      |            ^^^
note: required because it appears within the type `ArtifactFixedRegistry<ActiveArtifactStoreReplacement<..., ..., ...>>`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:15045:23
      |
15045 |     pub(crate) struct ArtifactFixedRegistry<T> {
      |                       ^^^^^^^^^^^^^^^^^^^^^
note: required because it appears within the type `component::app::VcsArtifactApp<A, M>`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:17250:16
      |
17250 |     pub struct VcsArtifactApp<A: ArtifactApp, M: SpaceMember + MemberFactory = NoMembers> {
      |                ^^^^^^^^^^^^^^
note: required by a bound in `component::app::PluginApp::poll_media_export`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:11121:26
      |
11121 |     pub trait PluginApp: Send + 'static {
      |                          ^^^^ required by this bound in `PluginApp::poll_media_export`
...
11373 |         async fn poll_media_export(&mut self, _handle: &ArtifactMediaExportHandle) -> Result<ArtifactMediaExportPoll, MediaError> {
      |                  ----------------- required by a bound in this associated function
      = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS/🗑️generated/derive-target/wasm32-wasip2/debug/deps/semio_framework_plugin-01555633483147ea.long-type-5462339182651029531.txt'
      = note: consider using `--verbose` to print the full type name to the console
help: consider further restricting the associated type
      |
24277 |         async fn export_media(&mut self, port: &str) -> Result<Media, MediaError> where <M as dsl::MemberFactory>::Open: std::marker::Send {
      |                                                                                   ++++++++++++++++++++++++++++++++++++++++++++++++++++++++


## E0277 — `<M as dsl::MemberFactory>::Open` cannot be sent between threads safely

error[E0277]: `<M as dsl::MemberFactory>::Open` cannot be sent between threads safely
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22427:89
      |
22427 |     impl<A: ArtifactApp, M: SpaceMember + MemberFactory + Send + 'static> PluginApp for VcsArtifactApp<A, M> {
      |                                                                                         ^^^^^^^^^^^^^^^^^^^^ `<M as dsl::MemberFactory>::Open` cannot be sent between threads safely
      |
      = help: within `[MaybeUninit<(u64, ActiveArtifactStoreReplacement<..., ..., ...>)>]`, the trait `std::marker::Send` is not implemented for `<M as dsl::MemberFactory>::Open`
note: required because it appears within the type `std::option::Option<<M as dsl::MemberFactory>::Open>`
     --> /Users/ueli/.rustup/toolchains/nightly-2026-07-07-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/option.rs:598:10
      |
  598 | pub enum Option<T> {
      |          ^^^^^^
note: required because it appears within the type `std::mem::MaybeDangling<std::option::Option<<M as dsl::MemberFactory>::Open>>`
     --> /Users/ueli/.rustup/toolchains/nightly-2026-07-07-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/mem/maybe_dangling.rs:73:12
      |
   73 | pub struct MaybeDangling<P: ?Sized>(P);
      |            ^^^^^^^^^^^^^
note: required because it appears within the type `std::mem::ManuallyDrop<std::option::Option<<M as dsl::MemberFactory>::Open>>`
     --> /Users/ueli/.rustup/toolchains/nightly-2026-07-07-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/mem/manually_drop.rs:158:12
      |
  158 | pub struct ManuallyDrop<T: ?Sized> {
      |            ^^^^^^^^^^^^
note: required because it appears within the type `ActiveArtifactStoreReplacement<<A as ArtifactApp>::Snapshot, ..., ...>`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:16704:12
      |
16704 |     struct ActiveArtifactStoreReplacement<P, Mutation, M>
      |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      = note: required because it appears within the type `(u64, ActiveArtifactStoreReplacement<..., ..., ...>)`
note: required because it appears within the type `MaybeDangling<(u64, ActiveArtifactStoreReplacement<..., ..., ...>)>`
     --> /Users/ueli/.rustup/toolchains/nightly-2026-07-07-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/mem/maybe_dangling.rs:73:12
      |
   73 | pub struct MaybeDangling<P: ?Sized>(P);
      |            ^^^^^^^^^^^^^
note: required because it appears within the type `ManuallyDrop<(u64, ActiveArtifactStoreReplacement<..., ..., ...>)>`
     --> /Users/ueli/.rustup/toolchains/nightly-2026-07-07-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/mem/manually_drop.rs:158:12
      |
  158 | pub struct ManuallyDrop<T: ?Sized> {
      |            ^^^^^^^^^^^^
note: required because it appears within the type `MaybeUninit<(u64, ActiveArtifactStoreReplacement<..., ..., ...>)>`
     --> /Users/ueli/.rustup/toolchains/nightly-2026-07-07-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/mem/maybe_uninit.rs:355:11
      |
  355 | pub union MaybeUninit<T> {
      |           ^^^^^^^^^^^
      = note: required because it appears within the type `[MaybeUninit<(u64, ActiveArtifactStoreReplacement<..., ..., ...>)>]`
      = note: required for `Unique<[MaybeUninit<(u64, ...)>]>` to implement `std::marker::Send`
note: required because it appears within the type `Box<[MaybeUninit<(u64, ...)>]>`
     --> /Users/ueli/.rustup/toolchains/nightly-2026-07-07-aarch64-apple-darwin/lib/rustlib/src/rust/library/alloc/src/boxed.rs:236:12
      |
  236 | pub struct Box<
      |            ^^^
note: required because it appears within the type `ArtifactFixedRegistry<ActiveArtifactStoreReplacement<..., ..., ...>>`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:15045:23
      |
15045 |     pub(crate) struct ArtifactFixedRegistry<T> {
      |                       ^^^^^^^^^^^^^^^^^^^^^
note: required because it appears within the type `component::app::VcsArtifactApp<A, M>`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:17250:16
      |
17250 |     pub struct VcsArtifactApp<A: ArtifactApp, M: SpaceMember + MemberFactory = NoMembers> {
      |                ^^^^^^^^^^^^^^
note: required by a bound in `component::app::PluginApp`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:11121:26
      |
11121 |     pub trait PluginApp: Send + 'static {
      |                          ^^^^ required by this bound in `PluginApp`
      = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS/🗑️generated/derive-target/wasm32-wasip2/debug/deps/semio_framework_plugin-01555633483147ea.long-type-5462339182651029531.txt'
      = note: consider using `--verbose` to print the full type name to the console
help: consider further restricting the associated type
      |
22427 |     impl<A: ArtifactApp, M: SpaceMember + MemberFactory + Send + 'static> PluginApp for VcsArtifactApp<A, M> where <M as dsl::MemberFactory>::Open: std::marker::Send {
      |                                                                                                              ++++++++++++++++++++++++++++++++++++++++++++++++++++++++


## E0107 — struct takes 2 generic arguments but 1 generic argument was supplied

error[E0107]: struct takes 2 generic arguments but 1 generic argument was supplied
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:18369:30
      |
18369 | ...   let mut target = ArtifactStoreReplacementAdmissionTarget::<A> { jobs: &mut self.store_replacement_jobs, operation: handle.o...
      |                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^   - supplied 1 generic argument
      |                        |
      |                        expected 2 generic arguments
      |
note: struct defined here, with 2 generic parameters: `A`, `M`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:17190:12
      |
17190 |     struct ArtifactStoreReplacementAdmissionTarget<'a, A: ArtifactApp, M: SpaceMember + MemberFactory> {
      |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^     -               -
help: add missing generic argument
      |
18369 |             let mut target = ArtifactStoreReplacementAdmissionTarget::<A, M> { jobs: &mut self.store_replacement_jobs, operation: handle.operation, generation: handle.generation, marker: std::marker::PhantomData };
      |                                                                         +++


## E0277 — `<M as dsl::MemberFactory>::Open` cannot be sent between threads safely

error[E0277]: `<M as dsl::MemberFactory>::Open` cannot be sent between threads safely
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:22448:27
      |
22448 |                 drop(self.take_local_interaction_query_reply());
      |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `<M as dsl::MemberFactory>::Open` cannot be sent between threads safely
      |
      = help: within `[MaybeUninit<(u64, ActiveArtifactStoreReplacement<..., ..., ...>)>]`, the trait `std::marker::Send` is not implemented for `<M as dsl::MemberFactory>::Open`
note: required because it appears within the type `std::option::Option<<M as dsl::MemberFactory>::Open>`
     --> /Users/ueli/.rustup/toolchains/nightly-2026-07-07-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/option.rs:598:10
      |
  598 | pub enum Option<T> {
      |          ^^^^^^
note: required because it appears within the type `std::mem::MaybeDangling<std::option::Option<<M as dsl::MemberFactory>::Open>>`
     --> /Users/ueli/.rustup/toolchains/nightly-2026-07-07-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/mem/maybe_dangling.rs:73:12
      |
   73 | pub struct MaybeDangling<P: ?Sized>(P);
      |            ^^^^^^^^^^^^^
note: required because it appears within the type `std::mem::ManuallyDrop<std::option::Option<<M as dsl::MemberFactory>::Open>>`
     --> /Users/ueli/.rustup/toolchains/nightly-2026-07-07-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/mem/manually_drop.rs:158:12
      |
  158 | pub struct ManuallyDrop<T: ?Sized> {
      |            ^^^^^^^^^^^^
note: required because it appears within the type `ActiveArtifactStoreReplacement<<A as ArtifactApp>::Snapshot, ..., ...>`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:16704:12
      |
16704 |     struct ActiveArtifactStoreReplacement<P, Mutation, M>
      |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      = note: required because it appears within the type `(u64, ActiveArtifactStoreReplacement<..., ..., ...>)`
note: required because it appears within the type `MaybeDangling<(u64, ActiveArtifactStoreReplacement<..., ..., ...>)>`
     --> /Users/ueli/.rustup/toolchains/nightly-2026-07-07-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/mem/maybe_dangling.rs:73:12
      |
   73 | pub struct MaybeDangling<P: ?Sized>(P);
      |            ^^^^^^^^^^^^^
note: required because it appears within the type `ManuallyDrop<(u64, ActiveArtifactStoreReplacement<..., ..., ...>)>`
     --> /Users/ueli/.rustup/toolchains/nightly-2026-07-07-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/mem/manually_drop.rs:158:12
      |
  158 | pub struct ManuallyDrop<T: ?Sized> {
      |            ^^^^^^^^^^^^
note: required because it appears within the type `MaybeUninit<(u64, ActiveArtifactStoreReplacement<..., ..., ...>)>`
     --> /Users/ueli/.rustup/toolchains/nightly-2026-07-07-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/mem/maybe_uninit.rs:355:11
      |
  355 | pub union MaybeUninit<T> {
      |           ^^^^^^^^^^^
      = note: required because it appears within the type `[MaybeUninit<(u64, ActiveArtifactStoreReplacement<..., ..., ...>)>]`
      = note: required for `Unique<[MaybeUninit<(u64, ...)>]>` to implement `std::marker::Send`
note: required because it appears within the type `Box<[MaybeUninit<(u64, ...)>]>`
     --> /Users/ueli/.rustup/toolchains/nightly-2026-07-07-aarch64-apple-darwin/lib/rustlib/src/rust/library/alloc/src/boxed.rs:236:12
      |
  236 | pub struct Box<
      |            ^^^
note: required because it appears within the type `ArtifactFixedRegistry<ActiveArtifactStoreReplacement<..., ..., ...>>`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:15045:23
      |
15045 |     pub(crate) struct ArtifactFixedRegistry<T> {
      |                       ^^^^^^^^^^^^^^^^^^^^^
note: required because it appears within the type `component::app::VcsArtifactApp<A, M>`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:17250:16
      |
17250 |     pub struct VcsArtifactApp<A: ArtifactApp, M: SpaceMember + MemberFactory = NoMembers> {
      |                ^^^^^^^^^^^^^^
note: required by a bound in `component::app::PluginApp::take_local_interaction_query_reply`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:11121:26
      |
11121 |     pub trait PluginApp: Send + 'static {
      |                          ^^^^ required by this bound in `PluginApp::take_local_interaction_query_reply`
...
11157 |         fn take_local_interaction_query_reply(&mut self) -> Option<protocol::LocalInteractionQueryReply> {
      |            ---------------------------------- required by a bound in this associated function
      = note: the full name for the type has been written to '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS/🗑️generated/derive-target/wasm32-wasip2/debug/deps/semio_framework_plugin-01555633483147ea.long-type-5462339182651029531.txt'
      = note: consider using `--verbose` to print the full type name to the console
help: consider further restricting the associated type
      |
22432 |         fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<PluginCloseStep, Fault> where <M as dsl::MemberFactory>::Open: std::marker::Send {
      |                                                                                                                ++++++++++++++++++++++++++++++++++++++++++++++++++++++++


## E0004 — non-exhaustive patterns: `component::app::ActiveArtifactStoreReplacementState::AwaitingMembers`, `component::app::ActiveArtifactStoreReplacementState::OpeningMembers`, `component::app::ActiveArtifactStoreReplacementState::ClosingRejectedMember` and 3 more not covered

error[E0004]: non-exhaustive patterns: `component::app::ActiveArtifactStoreReplacementState::AwaitingMembers`, `component::app::ActiveArtifactStoreReplacementState::OpeningMembers`, `component::app::ActiveArtifactStoreReplacementState::ClosingRejectedMember` and 3 more not covered
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:18391:227
      |
18391 | ...peDecodeOperationPoll::Fault, |active| match active.state {
      |                                                 ^^^^^^^^^^^^ patterns `component::app::ActiveArtifactStoreReplacementState::AwaitingMembers`, `component::app::ActiveArtifactStoreReplacementState::OpeningMembers`, `component::app::ActiveArtifactStoreReplacementState::ClosingRejectedMember` and 3 more not covered
      |
note: `component::app::ActiveArtifactStoreReplacementState` defined here
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:16690:10
      |
16690 |     enum ActiveArtifactStoreReplacementState {
      |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
16691 |         Initializing,
16692 |         AwaitingMembers,
      |         --------------- not covered
16693 |         OpeningMembers,
      |         -------------- not covered
16694 |         ClosingRejectedMember,
      |         --------------------- not covered
16695 |         ValidatingClosure,
      |         ----------------- not covered
16696 |         PreparingCandidateViews,
      |         ----------------------- not covered
      = note: the matched value is of type `component::app::ActiveArtifactStoreReplacementState`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern as shown, or multiple match arms
      |
18396 ~                 ActiveArtifactStoreReplacementState::Complete => ArtifactEnvelopeDecodeOperationPoll::Fault,
18397 ~                 _ => todo!(),
      |


