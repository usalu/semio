//! ⚖️ GIS map artifact — state-patch-representation wire codec + laws (was: constitutional
//! `protocol`; no `📡️protocol` path segment may survive under plugins).
//!
//! 🧷️ `GisMapMutation` derives `dsl::DslEnum` directly (no foreign `CollectionMutation` in its
//! shape — every variant wraps a local `dsl::DslRecord` payload declared in its own triad leaf), so
//! this component is a pure pass-through over the derived codec, matching `🏔️gisterrain`'s sibling
//! facet's identical shape.

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::standards::v1::subsets::any::schema::mutations::GisMapMutation;
use crate::host::owned::{GisMapMutationRetirementFactory, GisMapSnapshotRetirementFactory, GIS_MAP_OWNED_FIELD_BYTES};
use crate::{GisMapSnapshot, MapFeature};
use protocol::{Mutation, MutationDiff, OpBinary};

//#region 🔖️Codec
/// 📦️ Encodes a `GisMapMutation` to its binary command form.
pub fn encode_op(operation: &GisMapMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `GisMapMutation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<GisMapMutation, protocol::ProtocolError> {
    GisMapMutation::decode_op(bytes)
}
//#endregion 🔖️Codec

//#region 🔖️OwnedSprCatalog






























fn decode_gis_map_snapshot_pack(bytes: &[u8]) -> Result<GisMapSnapshot, ()> {
    <GisMapSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|_| ())
}

fn decode_gis_map_mutation_pack(bytes: &[u8]) -> Result<GisMapMutation, ()> {
    GisMapMutation::decode_op(bytes).map_err(|_| ())
}

macro_rules! gis_map_owned_field_close_capacity {
    (ArtifactEnvelopeSnapshotFieldAuthority) => {
        fn next_close_byte_demand(&self) -> Result<usize, store::OwnedSchemaDecodeDiagnostic> {
            Ok(usize::from(self.retirement.is_some()) * GIS_MAP_OWNED_FIELD_BYTES)
        }

        fn maximum_close_byte_demand(&self) -> usize {
            store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES
        }

        fn maximum_retained_close_bytes(&self) -> usize {
            store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES
        }
    };
    (ArtifactEnvelopeMutationFieldAuthority) => {};
}

macro_rules! gis_map_owned_field_authority {
    ($state:ident, $authority:ident, $value:ty, $authority_trait:ident, $target_trait:ident, $publish:ident, $decode:path, $factory:expr, $kind:literal) => {
        #[expect(clippy::large_enum_variant, reason = "The active decoder keeps its fixed path and admitted hex authority inline without a second allocation at the state transition.")]
        enum $state {
            AwaitToken,
            Decode(store::OwnedSchemaHexAuthority<GIS_MAP_OWNED_FIELD_BYTES>),
            Ready,
            Published,
            Closing,
            Complete,
        }

        pub(crate) struct $authority {
            operation: semio_framework_job::OperationId,
            generation: semio_framework_job::Generation,
            path: store::OwnedSchemaPath,
            state: $state,
            value: std::mem::ManuallyDrop<Option<$value>>,
            retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
        }

        impl $authority {
            pub(crate) fn new(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::OwnedSchemaPath) -> Self {
                Self { operation, generation, path, state: $state::AwaitToken, value: std::mem::ManuallyDrop::new(None), retirement: std::mem::ManuallyDrop::new(None) }
            }

            fn diagnostic(&self, code: &'static str, offset: u64) -> store::OwnedSchemaDecodeDiagnostic {
                store::OwnedSchemaDecodeDiagnostic { code, offset, line: 0, column: 0, path: self.path }
            }
        }

        impl store::$authority_trait<$value> for $authority {
            fn accept_token(
                &mut self,
                token: store::OwnedSchemaToken,
                terminal: bool,
                source: &store::OwnedSchemaRecordCursor,
                cx: &mut semio_framework_job::StepContext<'_>,
            ) -> Result<store::ArtifactEnvelopeFieldDecodeStep, store::OwnedSchemaDecodeDiagnostic> {
                let path = self.path;
                let diagnostic = |code: &'static str, offset| store::OwnedSchemaDecodeDiagnostic { code, offset, line: 0, column: 0, path };
                if matches!(self.state, $state::AwaitToken) {
                    if !terminal {
                        return Err(diagnostic(concat!("gis-map-envelope.", $kind, "-pack-must-be-scalar"), token.start));
                    }
                    self.state = $state::Decode(store::OwnedSchemaHexAuthority::try_new(self.operation, self.generation, token, self.path)?);
                }
                let $state::Decode(authority) = &mut self.state else {
                    return Err(diagnostic(concat!("gis-map-envelope.", $kind, "-pack-token-replayed"), token.start));
                };
                match authority.step(source, cx) {
                    store::OwnedSchemaHexStep::Pending => Ok(store::ArtifactEnvelopeFieldDecodeStep::Pending),
                    store::OwnedSchemaHexStep::Complete => {
                        let bytes = authority.as_bytes().ok_or_else(|| diagnostic(concat!("gis-map-envelope.", $kind, "-pack-missing"), token.start))?;
                        let value = $decode(bytes).map_err(|_| diagnostic(concat!("gis-map-envelope.", $kind, "-pack-malformed"), token.start))?;
                        if !authority.release() {
                            return Err(diagnostic(concat!("gis-map-envelope.", $kind, "-pack-release-duplicate"), token.start));
                        }
                        *self.value = Some(value);
                        self.state = $state::Ready;
                        Ok(store::ArtifactEnvelopeFieldDecodeStep::FieldComplete)
                    }
                    store::OwnedSchemaHexStep::Cancelled => Err(diagnostic(concat!("gis-map-envelope.", $kind, "-pack-cancelled"), token.start)),
                    store::OwnedSchemaHexStep::Fault(diagnostic) => Err(diagnostic),
                }
            }

            fn publish_reserved(
                &mut self,
                target: &mut dyn store::$target_trait<$value>,
                reservation: store::ArtifactEnvelopeFieldReservation,
                _cx: &mut semio_framework_job::StepContext<'_>,
            ) -> Result<store::ArtifactEnvelopeFieldDecodeStep, store::OwnedSchemaDecodeDiagnostic> {
                if !matches!(self.state, $state::Ready) {
                    return Err(self.diagnostic(concat!("gis-map-envelope.", $kind, "-pack-not-ready"), 0));
                }
                let value = self.value.take().ok_or_else(|| self.diagnostic(concat!("gis-map-envelope.", $kind, "-owner-missing"), 0))?;
                target.$publish(reservation, value);
                self.state = $state::Published;
                Ok(store::ArtifactEnvelopeFieldDecodeStep::FieldComplete)
            }

            gis_map_owned_field_close_capacity!($authority_trait);

            fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, store::OwnedSchemaDecodeDiagnostic> {
                if maximum_items == 0 {
                    return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
                }
                if let $state::Decode(authority) = &mut self.state {
                    authority.cancel();
                    self.state = $state::Closing;
                    return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
                }
                if self.retirement.is_none() {
                    if let Some(value) = self.value.take() {
                        *self.retirement = Some(store::ArtifactOwnedValueRetirementFactory::retire_owned($factory, value));
                        self.state = $state::Closing;
                        return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
                    }
                    self.state = $state::Complete;
                    return Ok(store::SnapshotRetirementStep::Complete);
                }
                let path = self.path;
                let retirement = self.retirement.as_mut().expect("GIS packed field retirement remains retained");
                match retirement.close_step(maximum_items.min(1), maximum_bytes).map_err(|_| store::OwnedSchemaDecodeDiagnostic { code: concat!("gis-map-envelope.", $kind, "-retirement-fault"), offset: 0, line: 0, column: 0, path })? {
                    store::SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => {
                        drop(self.retirement.take());
                        self.state = $state::Complete;
                        Ok(store::SnapshotRetirementStep::Complete)
                    }
                    store::SnapshotRetirementStep::Complete => Err(self.diagnostic(concat!("gis-map-envelope.", $kind, "-retirement-false-terminal"), 0)),
                    step => Ok(step),
                }
            }

            fn terminal_is_empty(&self) -> bool {
                matches!(self.state, $state::Published | $state::Complete) && self.value.is_none() && self.retirement.is_none()
            }
        }

        impl Drop for $authority {
            fn drop(&mut self) {
                assert!(std::thread::panicking() || (matches!(self.state, $state::Published | $state::Complete) && self.value.is_none() && self.retirement.is_none()), concat!("GIS ", $kind, " decode reached Drop before publication or bounded retirement"));
            }
        }
    };
}

gis_map_owned_field_authority!(
    GisMapSnapshotDecodeState,
    GisMapSnapshotDecodeAuthority,
    GisMapSnapshot,
    ArtifactEnvelopeSnapshotFieldAuthority,
    ArtifactEnvelopeSnapshotFieldTarget,
    publish_snapshot_reserved,
    decode_gis_map_snapshot_pack,
    &GisMapSnapshotRetirementFactory,
    "snapshot"
);

gis_map_owned_field_authority!(
    GisMapMutationDecodeState,
    GisMapMutationDecodeAuthority,
    GisMapMutation,
    ArtifactEnvelopeMutationFieldAuthority,
    ArtifactEnvelopeMutationFieldTarget,
    publish_mutation_reserved,
    decode_gis_map_mutation_pack,
    &GisMapMutationRetirementFactory,
    "mutation"
);










//#endregion 🔖️OwnedSprCatalog

//#region 🔖️RetainedStoreInitialization



















//#endregion 🔖️RetainedStoreInitialization

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

mod native_codec {
use super::*;
use crate::schema::mutations::{apply_gis_map_mutation, inverse_gis_map_mutation, GisMapMutation};

impl protocol::OpBinary for GisMapMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_tagged_op(include_str!("📡️.protocol.semio"), self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_tagged_op(include_str!("📡️.protocol.semio"), bytes)
    }
}
}
