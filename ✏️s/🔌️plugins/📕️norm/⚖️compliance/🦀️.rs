//! 📏️ Norm core: shared quantities, clause identity, compliance results, and national annex selection.

#![allow(async_fn_in_trait)]

#[cfg(test)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_value_derive as value_derive;

use protocol::Mutation;
use std::fmt;
use store::TextError;

pub use semio_s_plugin_norm_compliance_core::*;

#[path = "🏛️norm-error-and-family/🦀️.rs"]
mod norm_error_and_family;
pub use norm_error_and_family::*;

#[path = "⚡️incremental-evaluation/🦀️.rs"]
pub mod incremental_evaluation;

#[path = "🧰️design-case/🦀️.rs"]
pub mod design_case;

#[path = "🧭️spec-trace/🦀️.rs"]
pub mod spec_trace;

#[path = "🗣️reasons/🦀️.rs"]
pub mod reasons;
pub use reasons::{explain_reason, german_check_leaks, german_leaks, looks_english, reason_copy, Reason, ReasonRule, RULES as REASON_RULES, UNMAPPED_REASON};

#[path = "🔤️op-text-and-pack-codec/🦀️.rs"]
mod op_text_and_pack_codec;
pub use op_text_and_pack_codec::*;

/// 🎯️ Typed driver that feeds blessed oracle cases to OUR evaluators through per-artifact adapter registries.
#[cfg(any(test, feature = "compliance-testing"))]
#[path = "🧪️tests/🎯️oracle-driver/🦀️.rs"]
pub mod oracle_driver;

/// 🧬️ Shared regeneration of committed assets: the bless switch, DSL and pack rewrite, SQLite contract numbers.
#[cfg(any(test, feature = "test-support", feature = "compliance-testing"))]
#[path = "🧪️tests/🧬️regen/🦀️.rs"]
pub mod regen;

/// 🪪️ Independent Serde oracle for composed child identities; local materialization never enters JSON.
#[cfg(any(test, feature = "compliance-testing"))]
#[path = "🧪️tests/🔬️child-identity-oracle/🦀️.rs"]
pub mod child_identity_oracle;

// #region 🔖️OpText

//#region 🔖️SnapshotDslPackCodec

/// 🧩️ Implements `pack::snapshot::ArtifactDsl` + `pack::snapshot::ArtifactPack` for a norm family's snapshot type,
/// entirely by delegating to the four shared functions above. `$extension`/`$envelope_id` are the
/// two literals every one of the fifteen hand-written impls previously hardcoded independently.
#[macro_export]
macro_rules! impl_norm_artifact_record {
    ($Snapshot:ty, extension = $extension:literal, envelope_id = $envelope_id:literal $(, sqlite = $sqlite:path)?) => {
        $crate::impl_norm_artifact_record!(@text $Snapshot, extension=$extension, envelope_id=$envelope_id);
        $crate::impl_norm_artifact_record!(@binary $Snapshot, extension=$extension, envelope_id=$envelope_id $(,sqlite=$sqlite)?);
    };
    (@text $Snapshot:ty, extension=$extension:literal, envelope_id=$envelope_id:literal) => {
        impl $crate::NormArtifactRecord for $Snapshot {
            const EXTENSION: &'static str = $extension;
            const ENVELOPE_ID: &'static str = $envelope_id;
            fn dsl_spec() -> semio_framework_dsl_record::RecordSpec {
                Self::__dsl_spec()
            }
            fn dsl_to_record(&self) -> semio_framework_dsl_record::RecordValue {
                self.__dsl_to_record()
            }
            fn dsl_from_record(record: &semio_framework_dsl_record::RecordValue) -> Result<Self, store::TextError> {
                Self::__dsl_from_record(record)
            }
        }
        impl pack::snapshot::ArtifactDsl for $Snapshot {
            const EXTENSION: &'static str = $extension;
            fn envelope_id() -> &'static str {
                $envelope_id
            }
            fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
                $crate::norm_parse_dsl(text)
            }
            fn print_dsl(&self) -> String {
                $crate::norm_print_dsl(self)
            }
        }
    };
    (@binary $Snapshot:ty, extension=$extension:literal, envelope_id=$envelope_id:literal $(,sqlite=$sqlite:path)?) => {
        impl pack::snapshot::ArtifactPack for $Snapshot {
            fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
                $crate::norm_encode_pack(self, options)
            }
            fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
                $crate::norm_decode_pack(bytes, options)
            }
            fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
                Some(<Self as $crate::NormArtifactRecord>::dsl_spec())
            }

        }
impl ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot for $Snapshot {
    fn reference_snapshot_codec()->Option<::semio_framework_os_kernel::os_store::serialization::ArtifactReferenceSnapshotCodec<Self>>{Some(::semio_framework_os_kernel::os_store::serialization::reference_snapshot_codec::<Self>())}

fn sqlite_snapshot_codec() -> Option<store::os_store::serialization::ArtifactSqliteSnapshotCodec> {
                $crate::impl_norm_artifact_record!(@sqlite $( $sqlite )?)
            }
}
    };
    (@sqlite) => { None };
    (@sqlite $sqlite:path) => { Some($sqlite()) };
}
//#endregion 🔖️SnapshotDslPackCodec

// #endregion 🔖️OpText

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "🧪️tests/🧭️ownership/🦀️.rs"]
mod ownership_tests;
