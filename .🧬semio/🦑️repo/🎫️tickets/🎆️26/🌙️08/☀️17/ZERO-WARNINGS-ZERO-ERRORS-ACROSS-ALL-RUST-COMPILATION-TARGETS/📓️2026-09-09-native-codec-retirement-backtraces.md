# Native Codec Retirement Backtraces

Reran the two failing integration laws from their hash-verified executables with Rust backtraces enabled. These are actual failing runtime receipts, not source-only checks.

## gis_native_receipts_bind_literal_two_codec_closure_without_identity_or_factory_substitution

Status 101


running 1 test
test gis_native_receipts_bind_literal_two_codec_closure_without_identity_or_factory_substitution ... FAILED

failures:

failures:
    gis_native_receipts_bind_literal_two_codec_closure_without_identity_or_factory_substitution

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.02s



thread 'gis_native_receipts_bind_literal_two_codec_closure_without_identity_or_factory_substitution' (10164725) panicked at 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:2618:9:
artifact envelope terminal shell reached Drop before its app-owned bounded retirement authority detached every nested owner
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: <semio_framework_os_kernel::os_store::component::ArtifactEnvelope<semio_s_artifact_gis_gismap::standards::v1::subsets::any::schema::snapshot::component::GisMapSnapshot, semio_s_artifact_gis_gismap::standards::v1::subsets::any::schema::mutations::component::GisMapMutation> as core::ops::drop::Drop>::drop
   3: core::ptr::drop_glue::<semio_framework_os_kernel::os_store::component::ArtifactEnvelope<semio_s_artifact_gis_gismap::standards::v1::subsets::any::schema::snapshot::component::GisMapSnapshot, semio_s_artifact_gis_gismap::standards::v1::subsets::any::schema::mutations::component::GisMapMutation>>
   4: semio_framework_plugin::component::app::native_artifact_genesis_for_editor::<semio_s_artifact_gis_gismap::editor::gis2d::component::Gis2dPlayApp>::{closure#0}
   5: <core::pin::Pin<alloc::boxed::Box<dyn core::future::future::Future<Output = core::result::Result<semio_framework_os_kernel::os_store::component::ArtifactPackFiles, semio_framework_os_kernel::os_vcs::VcsError>> + core::marker::Send>> as core::future::future::Future>::poll
   6: native_codecs::gis_native_receipts_bind_literal_two_codec_closure_without_identity_or_factory_substitution::{closure#0}
   7: native_codecs::gis_native_receipts_bind_literal_two_codec_closure_without_identity_or_factory_substitution::__semio_async_test_block_on::<native_codecs::gis_native_receipts_bind_literal_two_codec_closure_without_identity_or_factory_substitution::{closure#0}>
   8: native_codecs::gis_native_receipts_bind_literal_two_codec_closure_without_identity_or_factory_substitution
   9: native_codecs::gis_native_receipts_bind_literal_two_codec_closure_without_identity_or_factory_substitution::{closure#0}
  10: <native_codecs::gis_native_receipts_bind_literal_two_codec_closure_without_identity_or_factory_substitution::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

## vcs_native_receipts_bind_literal_one_codec_closure_without_identity_or_factory_substitution

Status 101


running 1 test
test vcs_native_receipts_bind_literal_one_codec_closure_without_identity_or_factory_substitution ... FAILED

failures:

failures:
    vcs_native_receipts_bind_literal_one_codec_closure_without_identity_or_factory_substitution

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.04s



thread 'vcs_native_receipts_bind_literal_one_codec_closure_without_identity_or_factory_substitution' (10164728) panicked at 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:2618:9:
artifact envelope terminal shell reached Drop before its app-owned bounded retirement authority detached every nested owner
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: <semio_framework_os_kernel::os_store::component::ArtifactEnvelope<semio_s_artifact_vcs_vcs::standards::v1::subsets::any::schema::snapshot::component::VcsSnapshot, semio_s_artifact_vcs_vcs::standards::v1::subsets::any::schema::mutations::component::VcsDemoMutation> as core::ops::drop::Drop>::drop
   3: core::ptr::drop_glue::<semio_framework_os_kernel::os_store::component::ArtifactEnvelope<semio_s_artifact_vcs_vcs::standards::v1::subsets::any::schema::snapshot::component::VcsSnapshot, semio_s_artifact_vcs_vcs::standards::v1::subsets::any::schema::mutations::component::VcsDemoMutation>>
   4: semio_framework_plugin::component::app::native_artifact_genesis_for_editor::<semio_s_artifact_vcs_vcs::editor::vcs::component::VcsPlayApp>::{closure#0}
   5: <core::pin::Pin<alloc::boxed::Box<dyn core::future::future::Future<Output = core::result::Result<semio_framework_os_kernel::os_store::component::ArtifactPackFiles, semio_framework_os_kernel::os_vcs::VcsError>> + core::marker::Send>> as core::future::future::Future>::poll
   6: native_codecs::vcs_native_receipts_bind_literal_one_codec_closure_without_identity_or_factory_substitution::{closure#0}
   7: native_codecs::vcs_native_receipts_bind_literal_one_codec_closure_without_identity_or_factory_substitution::__semio_async_test_block_on::<native_codecs::vcs_native_receipts_bind_literal_one_codec_closure_without_identity_or_factory_substitution::{closure#0}>
   8: native_codecs::vcs_native_receipts_bind_literal_one_codec_closure_without_identity_or_factory_substitution
   9: native_codecs::vcs_native_receipts_bind_literal_one_codec_closure_without_identity_or_factory_substitution::{closure#0}
  10: <native_codecs::vcs_native_receipts_bind_literal_one_codec_closure_without_identity_or_factory_substitution::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
