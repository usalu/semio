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

use crate::artifacts::gismap::schema::mutations::text::GisMapMutation;
use crate::artifacts::gismap::{GisMapSnapshot, MapFeature};
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
const GIS_MAP_OWNED_FIELD_BYTES: usize = store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES;

enum GisMapRetirementOwner {
    Snapshot(GisMapSnapshot),
    Mutation(GisMapMutation),
    MutationFields(GisMapMutationFields),
    Feature(MapFeature),
    Value(dsl::DslValue),
    ValueEntry { key: String, value: Option<dsl::DslValue> },
}

enum GisMapMutationFields {
    Feature(Option<MapFeature>),
    String(String),
    Value { id: String, value: Option<dsl::DslValue> },
}

struct GisMapOwnedRetirement {
    owner: std::mem::ManuallyDrop<Option<GisMapRetirementOwner>>,
    active: std::mem::ManuallyDrop<Option<Box<GisMapOwnedRetirement>>>,
    phase: u8,
}

impl GisMapOwnedRetirement {
    fn new(owner: GisMapRetirementOwner) -> Self {
        Self { owner: std::mem::ManuallyDrop::new(Some(owner)), active: std::mem::ManuallyDrop::new(None), phase: 0 }
    }

    fn spawn(active: &mut Option<Box<Self>>, owner: GisMapRetirementOwner) -> store::SnapshotRetirementStep {
        *active = Some(Box::new(Self::new(owner)));
        store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }
    }

    fn release_string(value: &mut String, phase: &mut u8, next: u8, maximum_items: usize, maximum_bytes: usize) -> store::SnapshotRetirementStep {
        if maximum_items == 0 || value.len() > maximum_bytes {
            return store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 };
        }
        let value = std::mem::take(value);
        let released_bytes = value.len();
        drop(value);
        *phase = next;
        store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes }
    }

    fn child_step<S>(child: &mut store::ArtifactChild<S>, phase: &mut u8, base: u8, maximum_items: usize, maximum_bytes: usize) -> Option<store::SnapshotRetirementStep> {
        let step = match *phase - base {
            0 => Self::release_string(&mut child.child_id, phase, base + 1, maximum_items, maximum_bytes),
            1 => Self::release_string(&mut child.target.artifact_id, phase, base + 2, maximum_items, maximum_bytes),
            2 => Self::release_string(&mut child.target.dialect.artifact_kind, phase, base + 3, maximum_items, maximum_bytes),
            3 => Self::release_string(&mut child.target.dialect.standard, phase, base + 4, maximum_items, maximum_bytes),
            4 => Self::release_string(&mut child.target.dialect.subset, phase, base + 5, maximum_items, maximum_bytes),
            _ => return None,
        };
        Some(step)
    }

    fn advance(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {
        if maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        let Some(owner) = self.owner.as_mut() else { return Ok(store::SnapshotRetirementStep::Complete) };
        match owner {
            GisMapRetirementOwner::Snapshot(value) => match self.phase {
                0 => {
                    if let Some(value) = value.positions.pop() {
                        return Ok(Self::spawn(&mut self.active, GisMapRetirementOwner::Feature(value)));
                    }
                    self.phase = 1;
                    Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 })
                }
                1 => {
                    if let Some(value) = value.routes.pop() {
                        return Ok(Self::spawn(&mut self.active, GisMapRetirementOwner::Feature(value)));
                    }
                    self.phase = 2;
                    Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 })
                }
                2 => {
                    if let Some(value) = value.regions.pop() {
                        return Ok(Self::spawn(&mut self.active, GisMapRetirementOwner::Feature(value)));
                    }
                    self.phase = 3;
                    Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 })
                }
                3..=7 => Ok(Self::child_step(&mut value.drawing, &mut self.phase, 3, maximum_items, maximum_bytes).expect("GIS drawing child phase is exact")),
                8..=12 => {
                    if let Some(image) = value.image.as_mut() {
                        Ok(Self::child_step(image, &mut self.phase, 8, maximum_items, maximum_bytes).expect("GIS image child phase is exact"))
                    } else {
                        self.phase = 13;
                        Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 })
                    }
                }
                13..=17 => Ok(Self::child_step(&mut value.value, &mut self.phase, 13, maximum_items, maximum_bytes).expect("GIS value child phase is exact")),
                _ => {
                    drop(self.owner.take());
                    Ok(store::SnapshotRetirementStep::Complete)
                }
            },
            GisMapRetirementOwner::Feature(value) => match self.phase {
                0 => Ok(Self::release_string(&mut value.id, &mut self.phase, 1, maximum_items, maximum_bytes)),
                1 => {
                    let data = std::mem::replace(&mut value.data, dsl::DslValue::Null);
                    self.phase = 2;
                    Ok(Self::spawn(&mut self.active, GisMapRetirementOwner::Value(data)))
                }
                _ => {
                    drop(self.owner.take());
                    Ok(store::SnapshotRetirementStep::Complete)
                }
            },
            GisMapRetirementOwner::Value(value) => match value {
                dsl::DslValue::String(value) => {
                    if self.phase == 0 {
                        return Ok(Self::release_string(value, &mut self.phase, 1, maximum_items, maximum_bytes));
                    }
                    drop(self.owner.take());
                    Ok(store::SnapshotRetirementStep::Complete)
                }
                dsl::DslValue::Array(values) => {
                    if let Some(value) = values.pop() {
                        Ok(Self::spawn(&mut self.active, GisMapRetirementOwner::Value(value)))
                    } else {
                        drop(self.owner.take());
                        Ok(store::SnapshotRetirementStep::Complete)
                    }
                }
                dsl::DslValue::Object(values) => {
                    if let Some((key, value)) = values.pop() {
                        Ok(Self::spawn(&mut self.active, GisMapRetirementOwner::ValueEntry { key, value: Some(value) }))
                    } else {
                        drop(self.owner.take());
                        Ok(store::SnapshotRetirementStep::Complete)
                    }
                }
                dsl::DslValue::Null | dsl::DslValue::Bool(_) | dsl::DslValue::Number(_) => {
                    drop(self.owner.take());
                    Ok(store::SnapshotRetirementStep::Complete)
                }
            },
            GisMapRetirementOwner::ValueEntry { key, value } => match self.phase {
                0 => Ok(Self::release_string(key, &mut self.phase, 1, maximum_items, maximum_bytes)),
                1 => {
                    let value = value.take().ok_or_else(|| "GIS value entry lost its retained value".to_string())?;
                    self.phase = 2;
                    Ok(Self::spawn(&mut self.active, GisMapRetirementOwner::Value(value)))
                }
                _ => {
                    drop(self.owner.take());
                    Ok(store::SnapshotRetirementStep::Complete)
                }
            },
            GisMapRetirementOwner::Mutation(_) => {
                use GisMapMutation::*;
                let mutation = match self.owner.take() {
                    Some(GisMapRetirementOwner::Mutation(value)) => value,
                    _ => unreachable!("GIS mutation owner variant remains exact"),
                };
                let fields = match mutation {
                    CreatePosition(payload) => GisMapMutationFields::Feature(Some(payload.item)),
                    CreateRoute(payload) => GisMapMutationFields::Feature(Some(payload.item)),
                    CreateRegion(payload) => GisMapMutationFields::Feature(Some(payload.item)),
                    DeletePosition(payload) => GisMapMutationFields::String(payload.id),
                    DeleteRoute(payload) => GisMapMutationFields::String(payload.id),
                    DeleteRegion(payload) => GisMapMutationFields::String(payload.id),
                    ReorderPositions(payload) => GisMapMutationFields::String(payload.id),
                    ReorderRoutes(payload) => GisMapMutationFields::String(payload.id),
                    ReorderRegions(payload) => GisMapMutationFields::String(payload.id),
                    ReplacePositionData(payload) => GisMapMutationFields::Value { id: payload.id, value: Some(payload.new_data) },
                    ReplaceRouteData(payload) => GisMapMutationFields::Value { id: payload.id, value: Some(payload.new_data) },
                    ReplaceRegionData(payload) => GisMapMutationFields::Value { id: payload.id, value: Some(payload.new_data) },
                };
                *self.owner = Some(GisMapRetirementOwner::MutationFields(fields));
                Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 })
            }
            GisMapRetirementOwner::MutationFields(fields) => match fields {
                GisMapMutationFields::Feature(value) => {
                    if let Some(value) = value.take() {
                        return Ok(Self::spawn(&mut self.active, GisMapRetirementOwner::Feature(value)));
                    }
                    drop(self.owner.take());
                    Ok(store::SnapshotRetirementStep::Complete)
                }
                GisMapMutationFields::String(value) => {
                    if self.phase == 0 {
                        return Ok(Self::release_string(value, &mut self.phase, 1, maximum_items, maximum_bytes));
                    }
                    drop(self.owner.take());
                    Ok(store::SnapshotRetirementStep::Complete)
                }
                GisMapMutationFields::Value { id, value } => match self.phase {
                    0 => Ok(Self::release_string(id, &mut self.phase, 1, maximum_items, maximum_bytes)),
                    1 => {
                        let value = value.take().ok_or_else(|| "GIS mutation value owner was lost".to_string())?;
                        self.phase = 2;
                        Ok(Self::spawn(&mut self.active, GisMapRetirementOwner::Value(value)))
                    }
                    _ => {
                        drop(self.owner.take());
                        Ok(store::SnapshotRetirementStep::Complete)
                    }
                },
            },
        }
    }
}

impl store::ErasedSnapshotRetirement for GisMapOwnedRetirement {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {
        if let Some(active) = self.active.as_mut() {
            return match active.close_step(maximum_items.min(1), maximum_bytes)? {
                store::SnapshotRetirementStep::Complete if active.terminal_is_empty() => {
                    drop(self.active.take());
                    Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
                }
                store::SnapshotRetirementStep::Complete => Err("GIS nested retirement reported false terminal".into()),
                step => Ok(step),
            };
        }
        self.advance(maximum_items.min(1), maximum_bytes)
    }

    fn terminal_is_empty(&self) -> bool {
        self.owner.is_none() && self.active.is_none()
    }
}

impl Drop for GisMapOwnedRetirement {
    fn drop(&mut self) {
        assert!(store::ErasedSnapshotRetirement::terminal_is_empty(self), "GIS owner reached Drop before cursor retirement reached terminal-empty");
    }
}

pub struct GisMapSnapshotRetirementFactory;

impl store::ArtifactOwnedValueRetirementFactory<GisMapSnapshot> for GisMapSnapshotRetirementFactory {
    fn retire_owned(&self, value: GisMapSnapshot) -> Box<dyn store::ErasedSnapshotRetirement> {
        Box::new(GisMapOwnedRetirement::new(GisMapRetirementOwner::Snapshot(value)))
    }
}

struct GisMapSnapshotRootRetirement {
    owner: std::mem::ManuallyDrop<Option<std::sync::Arc<GisMapSnapshot>>>,
    retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
}

impl store::ErasedSnapshotRetirement for GisMapSnapshotRootRetirement {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {
        if maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(retirement) = self.retirement.as_mut() {
            return match retirement.close_step(1, maximum_bytes)? {
                store::SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => {
                    drop(self.retirement.take());
                    Ok(store::SnapshotRetirementStep::Complete)
                }
                store::SnapshotRetirementStep::Complete => Err("GIS snapshot root retirement reported false terminal".into()),
                step => Ok(step),
            };
        }
        let Some(owner) = self.owner.take() else { return Ok(store::SnapshotRetirementStep::Complete) };
        match std::sync::Arc::try_unwrap(owner) {
            Ok(value) => {
                *self.retirement = Some(store::ArtifactOwnedValueRetirementFactory::retire_owned(&GisMapSnapshotRetirementFactory, value));
                Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
            }
            Err(owner) => {
                *self.owner = Some(owner);
                Ok(store::SnapshotRetirementStep::Blocked)
            }
        }
    }

    fn terminal_is_empty(&self) -> bool {
        self.owner.is_none() && self.retirement.is_none()
    }
}

impl Drop for GisMapSnapshotRootRetirement {
    fn drop(&mut self) {
        assert!(self.owner.is_none() && self.retirement.is_none(), "GIS snapshot root reached Drop before exact Arc handback");
    }
}

impl store::SnapshotRetirementFactory<GisMapSnapshot> for GisMapSnapshotRetirementFactory {
    fn retire(&self, snapshot: std::sync::Arc<GisMapSnapshot>) -> Box<dyn store::ErasedSnapshotRetirement> {
        Box::new(GisMapSnapshotRootRetirement { owner: std::mem::ManuallyDrop::new(Some(snapshot)), retirement: std::mem::ManuallyDrop::new(None) })
    }
}

pub struct GisMapMutationRetirementFactory;

impl store::ArtifactOwnedValueRetirementFactory<GisMapMutation> for GisMapMutationRetirementFactory {
    fn retire_owned(&self, value: GisMapMutation) -> Box<dyn store::ErasedSnapshotRetirement> {
        Box::new(GisMapOwnedRetirement::new(GisMapRetirementOwner::Mutation(value)))
    }
}

fn decode_gis_map_snapshot_pack(bytes: &[u8]) -> Result<GisMapSnapshot, ()> {
    <GisMapSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|_| ())
}

fn decode_gis_map_mutation_pack(bytes: &[u8]) -> Result<GisMapMutation, ()> {
    GisMapMutation::decode_op(bytes).map_err(|_| ())
}

macro_rules! gis_map_owned_field_authority {
    ($state:ident, $authority:ident, $value:ty, $authority_trait:ident, $target_trait:ident, $publish:ident, $decode:path, $factory:expr, $kind:literal) => {
        enum $state {
            AwaitToken,
            Decode(store::OwnedSchemaHexAuthority<GIS_MAP_OWNED_FIELD_BYTES>),
            Ready,
            Published,
            Closing,
            Complete,
        }

        struct $authority {
            operation: semio_framework_job::OperationId,
            generation: semio_framework_job::Generation,
            path: store::OwnedSchemaPath,
            state: $state,
            value: std::mem::ManuallyDrop<Option<$value>>,
            retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
        }

        impl $authority {
            fn new(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::OwnedSchemaPath) -> Self {
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
                assert!(matches!(self.state, $state::Published | $state::Complete) && self.value.is_none() && self.retirement.is_none(), concat!("GIS ", $kind, " decode reached Drop before publication or bounded retirement"));
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

struct GisMapRejectedConflictAuthority {
    terminal: bool,
}

impl store::ArtifactEnvelopeSprConflictAuthority for GisMapRejectedConflictAuthority {
    fn accept_token(
        &mut self,
        token: store::OwnedSchemaToken,
        _terminal: bool,
        _source: &store::OwnedSchemaRecordCursor,
        _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::ArtifactEnvelopeFieldDecodeStep, store::OwnedSchemaDecodeDiagnostic> {
        Err(store::OwnedSchemaDecodeDiagnostic { code: "gis-map-envelope.fresh-conflict-not-admitted", offset: token.start, line: 0, column: 0, path: store::OwnedSchemaPath::ROOT })
    }

    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, store::OwnedSchemaDecodeDiagnostic> {
        if maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        self.terminal = true;
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal
    }
}

pub struct GisMapEnvelopeOwnedFieldCatalog;

impl store::ArtifactEnvelopeOwnedFieldCatalog<GisMapSnapshot, GisMapMutation> for GisMapEnvelopeOwnedFieldCatalog {
    fn begin_vcs(&self, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::OwnedSchemaPath) -> Box<dyn store::ArtifactEnvelopeVcsFieldAuthority<GisMapSnapshot, GisMapMutation>> {
        Box::new(store::ArtifactEnvelopeFreshVcsAuthority::new(self.begin_snapshot(operation, generation, path), std::sync::Arc::new(GisMapSnapshotRetirementFactory), std::sync::Arc::new(GisMapMutationRetirementFactory), self.edit_history_decoder()))
    }

    fn begin_snapshot(&self, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::OwnedSchemaPath) -> Box<dyn store::ArtifactEnvelopeSnapshotFieldAuthority<GisMapSnapshot>> {
        Box::new(GisMapSnapshotDecodeAuthority::new(operation, generation, path))
    }

    fn begin_mutation(&self, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::OwnedSchemaPath) -> Box<dyn store::ArtifactEnvelopeMutationFieldAuthority<GisMapMutation>> {
        Box::new(GisMapMutationDecodeAuthority::new(operation, generation, path))
    }

    fn begin_spr_conflict(&self, _operation: semio_framework_job::OperationId, _generation: semio_framework_job::Generation, _path: store::OwnedSchemaPath) -> Box<dyn store::ArtifactEnvelopeSprConflictAuthority> {
        Box::new(GisMapRejectedConflictAuthority { terminal: false })
    }

    fn edit_history_decoder(&self) -> std::sync::Arc<dyn store::ArtifactOwnedHistoryEntryDecoder<protocol::Edit<GisMapMutation>>> {
        store::artifact_owned_spr_edit_history_decoder(std::sync::Arc::new(Self), std::sync::Arc::new(GisMapMutationRetirementFactory))
    }
}

pub fn gis_map_envelope_decode_owner_bundle() -> store::ArtifactEnvelopeDecodeOwnerBundle<GisMapSnapshot, GisMapMutation> {
    store::ArtifactEnvelopeDecodeOwnerBundle::new(std::sync::Arc::new(GisMapEnvelopeOwnedFieldCatalog), std::sync::Arc::new(GisMapSnapshotRetirementFactory), std::sync::Arc::new(GisMapMutationRetirementFactory))
}
//#endregion 🔖️OwnedSprCatalog

//#region 🔖️RetainedStoreInitialization
struct GisMapSnapshotCloneAuthority {
    value: std::mem::ManuallyDrop<Option<GisMapSnapshot>>,
    retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    phase: u8,
    index: usize,
    terminal: bool,
}

impl GisMapSnapshotCloneAuthority {
    fn empty_child<S>() -> store::ArtifactChild<S> {
        store::ArtifactChild::new(String::new(), store::os_io::ArtifactRef { artifact_id: String::new(), dialect: store::os_io::ArtifactDialect { artifact_kind: String::new(), standard: String::new(), subset: String::new() } })
    }

    fn new() -> Self {
        Self {
            value: std::mem::ManuallyDrop::new(Some(GisMapSnapshot { positions: Vec::new(), routes: Vec::new(), regions: Vec::new(), drawing: Self::empty_child(), image: None, value: Self::empty_child() })),
            retirement: std::mem::ManuallyDrop::new(None),
            phase: 0,
            index: 0,
            terminal: false,
        }
    }

    fn clone_string(source: &str) -> Result<String, &'static str> {
        if source.len() > GIS_MAP_OWNED_FIELD_BYTES {
            return Err("gis-map-store.initializer-field-too-large");
        }
        let mut value = String::new();
        value.try_reserve_exact(source.len()).map_err(|_| "gis-map-store.initializer-string-admission")?;
        value.push_str(source);
        Ok(value)
    }

    fn clone_feature(source: &MapFeature) -> Result<MapFeature, &'static str> {
        let encoded = dsl::os_pack::json::to_json_string(source).into_bytes();
        if encoded.len() > GIS_MAP_OWNED_FIELD_BYTES {
            return Err("gis-map-store.initializer-feature-too-large");
        }
        Ok(source.clone())
    }

    fn copy_child_field<'a, S>(target: &mut store::ArtifactChild<S>, source: &'a store::ArtifactChild<S>, phase: u8, base: u8) -> Result<&'a [u8], &'static str> {
        let observed = match phase - base {
            0 => {
                target.child_id = Self::clone_string(&source.child_id)?;
                source.child_id.as_bytes()
            }
            1 => {
                target.target.artifact_id = Self::clone_string(&source.target.artifact_id)?;
                source.target.artifact_id.as_bytes()
            }
            2 => {
                target.target.dialect.artifact_kind = Self::clone_string(&source.target.dialect.artifact_kind)?;
                source.target.dialect.artifact_kind.as_bytes()
            }
            3 => {
                target.target.dialect.standard = Self::clone_string(&source.target.dialect.standard)?;
                source.target.dialect.standard.as_bytes()
            }
            4 => {
                target.target.dialect.subset = Self::clone_string(&source.target.dialect.subset)?;
                source.target.dialect.subset.as_bytes()
            }
            _ => return Err("gis-map-store.initializer-child-phase"),
        };
        Ok(observed)
    }

    fn step(&mut self, source: &GisMapSnapshot, digest: &mut store::ArtifactStoreInitializationDigest, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        let target = self.value.as_mut().ok_or("gis-map-store.initializer-clone-target")?;
        let observed = match self.phase {
            0..=2 => {
                let (source, target, admission) = match self.phase {
                    0 => (&source.positions, &mut target.positions, "gis-map-store.initializer-position-admission"),
                    1 => (&source.routes, &mut target.routes, "gis-map-store.initializer-route-admission"),
                    _ => (&source.regions, &mut target.regions, "gis-map-store.initializer-region-admission"),
                };
                if self.index == 0 && target.capacity() == 0 {
                    target.try_reserve_exact(source.len()).map_err(|_| admission)?;
                }
                if let Some(feature) = source.get(self.index) {
                    let encoded = dsl::os_pack::json::to_json_string(feature).into_bytes();
                    if encoded.len() > GIS_MAP_OWNED_FIELD_BYTES {
                        return Err("gis-map-store.initializer-feature-too-large");
                    }
                    target.push(Self::clone_feature(feature)?);
                    self.index += 1;
                    digest.observe(&encoded);
                    cx.consume_fuel(encoded.len().max(1) as u64);
                    return Ok(false);
                }
                self.phase += 1;
                self.index = 0;
                &[]
            }
            3..=7 => Self::copy_child_field(&mut target.drawing, &source.drawing, self.phase, 3)?,
            8 => {
                if source.image.is_some() {
                    target.image = Some(Self::empty_child());
                }
                &[]
            }
            9..=13 => {
                let source = source.image.as_ref().ok_or("gis-map-store.initializer-image-source")?;
                let target = target.image.as_mut().ok_or("gis-map-store.initializer-image-target")?;
                Self::copy_child_field(target, source, self.phase, 9)?
            }
            14..=18 => Self::copy_child_field(&mut target.value, &source.value, self.phase, 14)?,
            _ => {
                self.terminal = true;
                return Ok(true);
            }
        };
        digest.observe(observed);
        self.phase = match self.phase {
            8 if source.image.is_none() => 14,
            value => value + 1,
        };
        cx.consume_fuel(observed.len().max(1) as u64);
        Ok(false)
    }

    fn take_value(&mut self) -> Option<GisMapSnapshot> {
        if !self.terminal {
            return None;
        }
        self.value.take()
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {
        if maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if self.retirement.is_none() {
            if let Some(value) = self.value.take() {
                *self.retirement = Some(store::ArtifactOwnedValueRetirementFactory::retire_owned(&GisMapSnapshotRetirementFactory, value));
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
            }
            self.terminal = true;
            return Ok(store::SnapshotRetirementStep::Complete);
        }
        let retirement = self.retirement.as_mut().expect("GIS clone retirement remains exact");
        match retirement.close_step(1, maximum_bytes)? {
            store::SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => {
                drop(self.retirement.take());
                Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
            }
            store::SnapshotRetirementStep::Complete => Err("GIS clone retirement reported false terminal".into()),
            step => Ok(step),
        }
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal && self.value.is_none() && self.retirement.is_none()
    }
}

impl Drop for GisMapSnapshotCloneAuthority {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "GIS snapshot clone reached Drop before exact handoff or cursor retirement");
    }
}

pub fn gis_map_document_store_owners() -> store::MemberStoreOwners<GisMapSnapshot, GisMapMutation> {
    store::MemberStoreOwners::new(
        std::sync::Arc::new(GisMapSnapshotRetirementFactory),
        std::sync::Arc::new(GisMapSnapshotRetirementFactory),
        std::sync::Arc::new(GisMapMutationRetirementFactory),
        Box::new(store::ArtifactStoreCursorDisposer::<GisMapSnapshot, GisMapMutation>::new()),
    )
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum GisMapStoreInitializationPhase {
    ValidateEnvelope,
    ValidateEditPair { left: usize, right: usize },
    CloneInitial,
    SeedHistory { edit: usize, lane: u8, index: usize },
    FindApplied { position: usize, scan: usize },
    ApplyForward { position: usize, edit: usize, mutation: usize },
    HashInverse { position: usize, edit: usize, mutation: usize },
    CommitApplied { position: usize, edit: usize },
    FindRedo { position: usize, scan: usize },
    HashRedoForward { position: usize, edit: usize, mutation: usize },
    HashRedoInverse { position: usize, edit: usize, mutation: usize },
    CommitRedo { position: usize, edit: usize },
    BuildCandidate,
    RetireCancelled,
    RetireFault,
    Complete,
    Cancelled,
    Fault,
}

struct GisMapStoreInitializationAuthority {
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    envelope: std::mem::ManuallyDrop<Option<store::ArtifactEnvelope<GisMapSnapshot, GisMapMutation>>>,
    runtime: std::mem::ManuallyDrop<Option<store::ArtifactStoreInitializationRuntime<GisMapSnapshot>>>,
    candidate: std::mem::ManuallyDrop<Option<store::ArtifactStore<GisMapSnapshot, GisMapMutation>>>,
    active: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    envelope_retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    clone: std::mem::ManuallyDrop<Option<GisMapSnapshotCloneAuthority>>,
    initial_digest: std::mem::ManuallyDrop<Option<store::ArtifactStoreInitializationDigest>>,
    edit_digest: std::mem::ManuallyDrop<Option<store::ArtifactStoreInitializationDigest>>,
    phase: GisMapStoreInitializationPhase,
    cancel_requested: bool,
    fault: Option<Vec<u8>>,
    terminal_handoff: bool,
}

impl GisMapStoreInitializationAuthority {
    fn new(envelope: store::ArtifactEnvelope<GisMapSnapshot, GisMapMutation>, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation) -> Self {
        Self {
            operation,
            generation,
            envelope: std::mem::ManuallyDrop::new(Some(envelope)),
            runtime: std::mem::ManuallyDrop::new(None),
            candidate: std::mem::ManuallyDrop::new(None),
            active: std::mem::ManuallyDrop::new(None),
            envelope_retirement: std::mem::ManuallyDrop::new(None),
            clone: std::mem::ManuallyDrop::new(Some(GisMapSnapshotCloneAuthority::new())),
            initial_digest: std::mem::ManuallyDrop::new(Some(store::ArtifactStoreInitializationDigest::new(b"gis-map.initial"))),
            edit_digest: std::mem::ManuallyDrop::new(None),
            phase: GisMapStoreInitializationPhase::ValidateEnvelope,
            cancel_requested: false,
            fault: None,
            terminal_handoff: false,
        }
    }

    fn applied_id(&self, position: usize) -> Option<&str> {
        let envelope = self.envelope.as_ref()?;
        match &envelope.cursor {
            Some(cursor) => cursor.applied_edit_ids.get(position).map(String::as_str),
            None => envelope.vcs.edits.get(position).map(|edit| edit.id.as_str()),
        }
    }

    fn redo_id(&self, position: usize) -> Option<&str> {
        self.envelope.as_ref()?.cursor.as_ref()?.redo_edit_ids.get(position).map(String::as_str)
    }

    fn fail(&mut self, code: &'static [u8]) {
        self.fault = Some(code.to_vec());
        self.phase = GisMapStoreInitializationPhase::RetireFault;
    }

    fn pump_active(&mut self) -> Result<bool, String> {
        let Some(active) = self.active.as_mut() else { return Ok(false) };
        match active.close_step(1, GIS_MAP_OWNED_FIELD_BYTES)? {
            store::SnapshotRetirementStep::Pending { released_items, released_bytes } if released_items <= 1 && released_bytes <= GIS_MAP_OWNED_FIELD_BYTES => Ok(true),
            store::SnapshotRetirementStep::Pending { .. } => Err("GIS store initializer retirement exceeded its exact grant".into()),
            store::SnapshotRetirementStep::Blocked => Ok(true),
            store::SnapshotRetirementStep::Complete if active.terminal_is_empty() => {
                drop(self.active.take());
                Ok(true)
            }
            store::SnapshotRetirementStep::Complete => Err("GIS store initializer retirement reported a false terminal".into()),
        }
    }

    fn pump_terminal_retirement(&mut self) -> Result<bool, String> {
        if self.pump_active()? {
            return Ok(false);
        }
        if let Some(runtime) = self.runtime.as_mut() {
            match runtime.close_step(&GisMapSnapshotRetirementFactory, 1, GIS_MAP_OWNED_FIELD_BYTES)? {
                store::SnapshotRetirementStep::Complete if runtime.terminal_is_empty() => {
                    drop(self.runtime.take());
                    return Ok(false);
                }
                store::SnapshotRetirementStep::Complete => return Err("GIS initialization runtime reported a false terminal".into()),
                _ => return Ok(false),
            }
        }
        if let Some(clone) = self.clone.as_mut() {
            match clone.close_step(1, GIS_MAP_OWNED_FIELD_BYTES)? {
                store::SnapshotRetirementStep::Complete if clone.terminal_is_empty() => {
                    drop(self.clone.take());
                    return Ok(false);
                }
                store::SnapshotRetirementStep::Complete => return Err("GIS snapshot clone reported a false terminal".into()),
                _ => return Ok(false),
            }
        }
        if self.envelope_retirement.is_none() {
            if let Some(envelope) = self.envelope.take() {
                *self.envelope_retirement = Some(gis_map_envelope_decode_owner_bundle().retire_envelope(envelope));
                return Ok(false);
            }
        }
        if let Some(retirement) = self.envelope_retirement.as_mut() {
            return match retirement.close_step(1, GIS_MAP_OWNED_FIELD_BYTES)? {
                store::SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => {
                    drop(self.envelope_retirement.take());
                    Ok(true)
                }
                store::SnapshotRetirementStep::Complete => Err("GIS initialization envelope retirement reported a false terminal".into()),
                _ => Ok(false),
            };
        }
        Ok(true)
    }

    fn terminal_is_empty_inner(&self) -> bool {
        self.terminal_handoff
            && self.envelope.is_none()
            && self.runtime.is_none()
            && self.candidate.is_none()
            && self.active.is_none()
            && self.envelope_retirement.is_none()
            && self.clone.is_none()
            && self.initial_digest.is_none()
            && self.edit_digest.is_none()
    }
}

impl semio_framework_plugin::ArtifactStoreInitializationAuthority<GisMapSnapshot, GisMapMutation> for GisMapStoreInitializationAuthority {
    fn step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> semio_framework_job::StepOutcome {
        if cx.operation() != self.operation || cx.generation() != self.generation {
            self.fail(b"gis-map-store.initializer-stale-authority");
        }
        if self.cancel_requested && !matches!(self.phase, GisMapStoreInitializationPhase::RetireCancelled | GisMapStoreInitializationPhase::Cancelled) {
            self.phase = GisMapStoreInitializationPhase::RetireCancelled;
        }
        if let Err(error) = self.pump_active() {
            self.fault = Some(error.into_bytes());
            self.phase = GisMapStoreInitializationPhase::RetireFault;
        } else if self.active.is_some() {
            return semio_framework_job::StepOutcome::Yield;
        }
        match self.phase {
            GisMapStoreInitializationPhase::ValidateEnvelope => {
                let Some(envelope) = self.envelope.as_ref() else {
                    self.fail(b"gis-map-store.initializer-envelope-missing");
                    return semio_framework_job::StepOutcome::Yield;
                };
                if envelope.schema != crate::artifacts::gismap::GIS_MAP_SCHEMA || envelope.id.is_empty() || envelope.id.len() > GIS_MAP_OWNED_FIELD_BYTES {
                    self.fail(b"gis-map-store.initializer-envelope-invalid");
                } else {
                    self.phase = GisMapStoreInitializationPhase::ValidateEditPair { left: 0, right: 1 };
                }
                cx.consume_fuel(1);
                semio_framework_job::StepOutcome::Yield
            }
            GisMapStoreInitializationPhase::ValidateEditPair { left, right } => {
                let envelope = self.envelope.as_ref().expect("validated GIS envelope remains retained");
                if left >= envelope.vcs.edits.len() {
                    self.phase = GisMapStoreInitializationPhase::CloneInitial;
                } else if right >= envelope.vcs.edits.len() {
                    self.phase = GisMapStoreInitializationPhase::ValidateEditPair { left: left + 1, right: left + 2 };
                } else if envelope.vcs.edits[left].id == envelope.vcs.edits[right].id || envelope.vcs.edits[left].id.len() > GIS_MAP_OWNED_FIELD_BYTES {
                    self.fail(b"gis-map-store.initializer-duplicate-or-hostile-edit");
                } else {
                    self.phase = GisMapStoreInitializationPhase::ValidateEditPair { left, right: right + 1 };
                }
                cx.consume_fuel(1);
                semio_framework_job::StepOutcome::Yield
            }
            GisMapStoreInitializationPhase::CloneInitial => {
                let source = &self.envelope.as_ref().expect("GIS envelope remains retained during initial clone").vcs.initial_snapshot;
                let clone = self.clone.as_mut().expect("GIS initial clone authority remains retained");
                let complete = match clone.step(source, self.initial_digest.as_mut().expect("GIS initial digest remains retained"), cx) {
                    Ok(complete) => complete,
                    Err(code) => {
                        self.fail(code.as_bytes());
                        return semio_framework_job::StepOutcome::Yield;
                    }
                };
                if complete {
                    let initial = clone.take_value().expect("GIS initial snapshot was built one semantic item at a time");
                    drop(self.clone.take());
                    let initial_digest = self.initial_digest.take().expect("GIS initial digest remains retained").finish();
                    let envelope = self.envelope.as_ref().expect("GIS envelope remains retained during runtime construction");
                    *self.runtime = Some(store::ArtifactStoreInitializationRuntime::new(&envelope.id, &envelope.schema, initial, initial_digest));
                    self.phase = GisMapStoreInitializationPhase::SeedHistory { edit: 0, lane: 0, index: 0 };
                }
                semio_framework_job::StepOutcome::Yield
            }
            GisMapStoreInitializationPhase::SeedHistory { edit, lane, index } => {
                let envelope = self.envelope.as_ref().expect("GIS envelope remains retained while causal history is seeded");
                let Some(entry) = envelope.vcs.edits.get(edit) else {
                    self.phase = GisMapStoreInitializationPhase::FindApplied { position: 0, scan: 0 };
                    return semio_framework_job::StepOutcome::Yield;
                };
                let runtime = self.runtime.as_mut().expect("GIS runtime remains retained while history is seeded");
                match lane {
                    0 => {
                        if let Err(error) = runtime.seed_mutation(protocol::MutationId(entry.id.clone())) {
                            self.fault = Some(error.into_bytes());
                            self.phase = GisMapStoreInitializationPhase::RetireFault;
                        } else {
                            runtime.observe_sequence(entry.sequence_number);
                            self.phase = GisMapStoreInitializationPhase::SeedHistory { edit, lane: 1, index: 0 };
                        }
                    }
                    1 if index < entry.forwards.len() => {
                        let id = entry.mutation_meta.get(index).and_then(|meta| meta.mutation_id.clone()).or_else(|| entry.forwards[index].mutation_id()).unwrap_or_else(|| protocol::MutationId(format!("{}#{index}", entry.id)));
                        if let Err(error) = runtime.seed_mutation(id) {
                            self.fault = Some(error.into_bytes());
                            self.phase = GisMapStoreInitializationPhase::RetireFault;
                        } else {
                            self.phase = GisMapStoreInitializationPhase::SeedHistory { edit, lane, index: index + 1 };
                        }
                    }
                    1 => self.phase = GisMapStoreInitializationPhase::SeedHistory { edit, lane: 2, index: 0 },
                    2 if index < entry.mutation_meta.len() => {
                        runtime.observe_timestamp(entry.mutation_meta[index].timestamp.clone());
                        self.phase = GisMapStoreInitializationPhase::SeedHistory { edit, lane, index: index + 1 };
                    }
                    _ => self.phase = GisMapStoreInitializationPhase::SeedHistory { edit: edit + 1, lane: 0, index: 0 },
                }
                cx.consume_fuel(1);
                semio_framework_job::StepOutcome::Yield
            }
            GisMapStoreInitializationPhase::FindApplied { position, scan } => {
                let Some(id) = self.applied_id(position) else {
                    let checkpoint = self.envelope.as_ref().and_then(|envelope| envelope.cursor.as_ref().and_then(|cursor| cursor.checkpoint_id.clone()).or_else(|| envelope.vcs.checkpoints.last().map(|checkpoint| checkpoint.id.clone())));
                    self.runtime.as_mut().expect("GIS runtime remains retained").set_current_checkpoint_id(checkpoint);
                    self.phase = GisMapStoreInitializationPhase::FindRedo { position: 0, scan: 0 };
                    return semio_framework_job::StepOutcome::Yield;
                };
                let envelope = self.envelope.as_ref().expect("GIS envelope remains retained");
                let Some(edit) = envelope.vcs.edits.get(scan) else {
                    self.fail(b"gis-map-store.initializer-applied-edit-missing");
                    return semio_framework_job::StepOutcome::Yield;
                };
                if edit.id == id {
                    let mut digest = store::ArtifactStoreInitializationDigest::new(b"gis-map.edit");
                    digest.observe(edit.id.as_bytes());
                    digest.observe(&edit.sequence_number.to_be_bytes());
                    digest.observe(edit.started_at.as_bytes());
                    *self.edit_digest = Some(digest);
                    self.phase = GisMapStoreInitializationPhase::ApplyForward { position, edit: scan, mutation: 0 };
                } else {
                    self.phase = GisMapStoreInitializationPhase::FindApplied { position, scan: scan + 1 };
                }
                cx.consume_fuel(1);
                semio_framework_job::StepOutcome::Yield
            }
            GisMapStoreInitializationPhase::ApplyForward { position, edit, mutation } => {
                let entry = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).expect("GIS applied edit remains retained");
                let Some(operation) = entry.forwards.get(mutation) else {
                    self.phase = GisMapStoreInitializationPhase::HashInverse { position, edit, mutation: 0 };
                    return semio_framework_job::StepOutcome::Yield;
                };
                let encoded = match operation.encode_op() {
                    Ok(encoded) if encoded.len() <= GIS_MAP_OWNED_FIELD_BYTES => encoded,
                    _ => {
                        self.fail(b"gis-map-store.initializer-forward-encoding");
                        return semio_framework_job::StepOutcome::Yield;
                    }
                };
                self.edit_digest.as_mut().expect("GIS edit digest remains retained").observe(&encoded);
                let current = self.runtime.as_mut().and_then(store::ArtifactStoreInitializationRuntime::current_mut).expect("GIS runtime current snapshot remains retained");
                let (diff, messages) = operation.diff(current).into_parts();
                if messages.iter().any(|message| message.level == protocol::Severity::Fatal) {
                    self.fail(b"gis-map-store.initializer-fatal-mutation");
                    return semio_framework_job::StepOutcome::Yield;
                }
                match diff.apply(current) {
                    Ok(next) => {
                        let previous = std::mem::replace(current, next);
                        *self.active = Some(store::ArtifactOwnedValueRetirementFactory::retire_owned(&GisMapSnapshotRetirementFactory, previous));
                        self.phase = GisMapStoreInitializationPhase::ApplyForward { position, edit, mutation: mutation + 1 };
                        cx.consume_fuel(encoded.len().max(1) as u64);
                    }
                    Err(error) => {
                        self.fault = Some(error.to_string().into_bytes());
                        self.phase = GisMapStoreInitializationPhase::RetireFault;
                    }
                }
                semio_framework_job::StepOutcome::Yield
            }
            GisMapStoreInitializationPhase::HashInverse { position, edit, mutation } => {
                let entry = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).expect("GIS applied edit remains retained");
                let Some(operation) = entry.inverse.get(mutation) else {
                    self.phase = GisMapStoreInitializationPhase::CommitApplied { position, edit };
                    return semio_framework_job::StepOutcome::Yield;
                };
                match operation.encode_op() {
                    Ok(encoded) if encoded.len() <= GIS_MAP_OWNED_FIELD_BYTES => {
                        self.edit_digest.as_mut().expect("GIS edit digest remains retained").observe(&encoded);
                        self.phase = GisMapStoreInitializationPhase::HashInverse { position, edit, mutation: mutation + 1 };
                        cx.consume_fuel(encoded.len().max(1) as u64);
                    }
                    _ => self.fail(b"gis-map-store.initializer-inverse-encoding"),
                }
                semio_framework_job::StepOutcome::Yield
            }
            GisMapStoreInitializationPhase::CommitApplied { position, edit } => {
                let entry = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).expect("GIS applied edit remains retained");
                let id = entry.id.clone();
                let actor = entry.actor.clone();
                let digest = self.edit_digest.take().expect("GIS applied edit digest remains retained").finish();
                let runtime = self.runtime.as_mut().expect("GIS runtime remains retained");
                if let Err(error) = runtime.push_applied(id, digest) {
                    self.fault = Some(error.into_bytes());
                    self.phase = GisMapStoreInitializationPhase::RetireFault;
                } else {
                    runtime.set_local_actor_id(actor);
                    self.phase = GisMapStoreInitializationPhase::FindApplied { position: position + 1, scan: 0 };
                }
                cx.consume_fuel(1);
                semio_framework_job::StepOutcome::Yield
            }
            GisMapStoreInitializationPhase::FindRedo { position, scan } => {
                let Some(id) = self.redo_id(position) else {
                    self.phase = GisMapStoreInitializationPhase::BuildCandidate;
                    return semio_framework_job::StepOutcome::Yield;
                };
                let envelope = self.envelope.as_ref().expect("GIS envelope remains retained");
                let Some(edit) = envelope.vcs.edits.get(scan) else {
                    self.fail(b"gis-map-store.initializer-redo-edit-missing");
                    return semio_framework_job::StepOutcome::Yield;
                };
                if edit.id == id {
                    let mut digest = store::ArtifactStoreInitializationDigest::new(b"gis-map.edit");
                    digest.observe(edit.id.as_bytes());
                    digest.observe(&edit.sequence_number.to_be_bytes());
                    digest.observe(edit.started_at.as_bytes());
                    *self.edit_digest = Some(digest);
                    self.phase = GisMapStoreInitializationPhase::HashRedoForward { position, edit: scan, mutation: 0 };
                } else {
                    self.phase = GisMapStoreInitializationPhase::FindRedo { position, scan: scan + 1 };
                }
                cx.consume_fuel(1);
                semio_framework_job::StepOutcome::Yield
            }
            GisMapStoreInitializationPhase::HashRedoForward { position, edit, mutation } => {
                let entry = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).expect("GIS redo edit remains retained");
                let Some(operation) = entry.forwards.get(mutation) else {
                    self.phase = GisMapStoreInitializationPhase::HashRedoInverse { position, edit, mutation: 0 };
                    return semio_framework_job::StepOutcome::Yield;
                };
                match operation.encode_op() {
                    Ok(encoded) if encoded.len() <= GIS_MAP_OWNED_FIELD_BYTES => {
                        self.edit_digest.as_mut().expect("GIS redo digest remains retained").observe(&encoded);
                        self.phase = GisMapStoreInitializationPhase::HashRedoForward { position, edit, mutation: mutation + 1 };
                        cx.consume_fuel(encoded.len().max(1) as u64);
                    }
                    _ => self.fail(b"gis-map-store.initializer-redo-forward-encoding"),
                }
                semio_framework_job::StepOutcome::Yield
            }
            GisMapStoreInitializationPhase::HashRedoInverse { position, edit, mutation } => {
                let entry = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).expect("GIS redo edit remains retained");
                let Some(operation) = entry.inverse.get(mutation) else {
                    self.phase = GisMapStoreInitializationPhase::CommitRedo { position, edit };
                    return semio_framework_job::StepOutcome::Yield;
                };
                match operation.encode_op() {
                    Ok(encoded) if encoded.len() <= GIS_MAP_OWNED_FIELD_BYTES => {
                        self.edit_digest.as_mut().expect("GIS redo digest remains retained").observe(&encoded);
                        self.phase = GisMapStoreInitializationPhase::HashRedoInverse { position, edit, mutation: mutation + 1 };
                        cx.consume_fuel(encoded.len().max(1) as u64);
                    }
                    _ => self.fail(b"gis-map-store.initializer-redo-inverse-encoding"),
                }
                semio_framework_job::StepOutcome::Yield
            }
            GisMapStoreInitializationPhase::CommitRedo { position, edit } => {
                let id = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).expect("GIS redo edit remains retained").id.clone();
                let digest = self.edit_digest.take().expect("GIS redo digest remains retained").finish();
                if let Err(error) = self.runtime.as_mut().expect("GIS runtime remains retained").push_redo(id, digest) {
                    self.fault = Some(error.into_bytes());
                    self.phase = GisMapStoreInitializationPhase::RetireFault;
                } else {
                    self.phase = GisMapStoreInitializationPhase::FindRedo { position: position + 1, scan: 0 };
                }
                cx.consume_fuel(1);
                semio_framework_job::StepOutcome::Yield
            }
            GisMapStoreInitializationPhase::BuildCandidate => {
                let Some(candidate_generation) = self.generation.0.checked_add(1) else {
                    self.fail(b"gis-map-store.initializer-generation-exhausted");
                    return semio_framework_job::StepOutcome::Yield;
                };
                let envelope = self.envelope.take().expect("GIS envelope remains retained until atomic store construction");
                let runtime = self.runtime.take().expect("GIS runtime remains retained until atomic store construction");
                let candidate = store::ArtifactStore::from_initialized_runtime_with_owners(envelope, runtime, candidate_generation, gis_map_document_store_owners());
                *self.candidate = Some(candidate);
                self.phase = GisMapStoreInitializationPhase::Complete;
                semio_framework_job::StepOutcome::Complete(semio_framework_job::CommitCandidate {
                    state: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitState),
                    output: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitOutput),
                })
            }
            GisMapStoreInitializationPhase::RetireCancelled | GisMapStoreInitializationPhase::RetireFault => match self.pump_terminal_retirement() {
                Ok(false) => semio_framework_job::StepOutcome::Yield,
                Ok(true) => {
                    drop(self.initial_digest.take());
                    drop(self.edit_digest.take());
                    self.terminal_handoff = true;
                    if self.phase == GisMapStoreInitializationPhase::RetireCancelled {
                        self.phase = GisMapStoreInitializationPhase::Cancelled;
                        semio_framework_job::StepOutcome::Cancelled
                    } else {
                        self.phase = GisMapStoreInitializationPhase::Fault;
                        let fault = self.fault.take().unwrap_or_else(|| b"gis-map-store.initializer-fault".to_vec());
                        let detail = cx
                            .payload_from_bytes(semio_framework_job::JobPayloadStream::Fault, &fault)
                            .unwrap_or_else(|_| semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::Fault));
                        semio_framework_job::StepOutcome::Fault(semio_framework_job::JobFault { detail })
                    }
                }
                Err(error) => {
                    self.fault = Some(error.into_bytes());
                    semio_framework_job::StepOutcome::Yield
                }
            },
            GisMapStoreInitializationPhase::Complete => semio_framework_job::StepOutcome::Complete(semio_framework_job::CommitCandidate {
                state: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitState),
                output: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitOutput),
            }),
            GisMapStoreInitializationPhase::Cancelled => semio_framework_job::StepOutcome::Cancelled,
            GisMapStoreInitializationPhase::Fault => {
                let fault = self.fault.as_deref().unwrap_or(b"gis-map-store.initializer-fault");
                let detail = cx
                    .payload_from_bytes(semio_framework_job::JobPayloadStream::Fault, fault)
                    .unwrap_or_else(|_| semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::Fault));
                semio_framework_job::StepOutcome::Fault(semio_framework_job::JobFault { detail })
            }
        }
    }

    fn request_cancel(&mut self) {
        self.cancel_requested = true;
    }

    fn begin_close(&mut self) {
        self.cancel_requested = true;
        if !matches!(self.phase, GisMapStoreInitializationPhase::Cancelled | GisMapStoreInitializationPhase::Fault) {
            self.phase = GisMapStoreInitializationPhase::RetireCancelled;
        }
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<semio_framework_plugin::PluginCloseStep, semio_framework::Fault> {
        self.begin_close();
        if maximum_items == 0 || maximum_bytes < GIS_MAP_OWNED_FIELD_BYTES {
            return Ok(semio_framework_plugin::PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
        }
        match self.pump_terminal_retirement() {
            Ok(false) => Ok(semio_framework_plugin::PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }),
            Ok(true) => {
                drop(self.initial_digest.take());
                drop(self.edit_digest.take());
                self.terminal_handoff = true;
                Ok(semio_framework_plugin::PluginCloseStep::Complete)
            }
            Err(error) => Err(semio_framework::Fault::new(semio_framework::FaultOrigin::Plugin, semio_framework::FaultCode::new("artifact-store.initializer-close"), format!("GIS Map initializer close failed: {error}"))),
        }
    }

    fn take_candidate(&mut self) -> Option<store::ArtifactStore<GisMapSnapshot, GisMapMutation>> {
        if self.phase != GisMapStoreInitializationPhase::Complete || self.terminal_handoff {
            return None;
        }
        let candidate = self.candidate.take()?;
        drop(self.initial_digest.take());
        drop(self.edit_digest.take());
        self.terminal_handoff = true;
        Some(candidate)
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal_is_empty_inner()
    }
}

impl Drop for GisMapStoreInitializationAuthority {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty_inner(), "GIS store initialization authority reached Drop before exact candidate handoff or retained rejection close");
    }
}

pub fn gis_map_document_store_initialization_job(
    envelope: store::ArtifactEnvelope<GisMapSnapshot, GisMapMutation>,
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
) -> semio_framework_plugin::ArtifactStoreInitializationJob<GisMapSnapshot, GisMapMutation> {
    semio_framework_plugin::ArtifactStoreInitializationJob::new(Box::new(GisMapStoreInitializationAuthority::new(envelope, operation, generation)))
}
//#endregion 🔖️RetainedStoreInitialization

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::gismap::mutations::{
        create_position, create_region, create_route, delete_position, delete_region, delete_route, reorder_positions, reorder_regions, reorder_routes, replace_position_data, replace_region_data, replace_route_data,
    };
    use crate::artifacts::gismap::schema::{default_document, empty_gis_map_snapshot};
    use crate::artifacts::gismap::GIS_MAP_SCHEMA;
    use serde_json::json;

    fn dsl_of(value: &serde_json::Value) -> dsl::DslValue {
        dsl::DslValue::from(value)
    }

    fn sample_feature(id: &str) -> crate::artifacts::gismap::MapFeature {
        crate::artifacts::gismap::MapFeature { id: id.into(), data: dsl_of(&json!({ "id": id, "lon": 1.0, "lat": 2.0 })) }
    }

    #[semio_framework_async_macros::async_test]
    async fn op_binary_round_trips_and_agrees_with_text() {
        let operation = GisMapMutation::CreatePosition(create_position::CreatePosition { index: 0, item: sample_feature("p1") });
        store::os_store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    #[semio_framework_async_macros::async_test]
    async fn gis_map_positions_op_lines_round_trip() {
        store::os_store::test_support::assert_op_line_round_trip(&GisMapMutation::CreatePosition(create_position::CreatePosition { index: 0, item: sample_feature("p1") }));
        store::os_store::test_support::assert_op_line_round_trip(&GisMapMutation::DeletePosition(delete_position::DeletePosition { id: "p1".into() }));
        store::os_store::test_support::assert_op_line_round_trip(&GisMapMutation::ReorderPositions(reorder_positions::ReorderPositions { id: "p1".into(), to_index: 3 }));
        store::os_store::test_support::assert_op_line_round_trip(&GisMapMutation::ReplacePositionData(replace_position_data::ReplacePositionData { id: "p1".into(), new_data: dsl_of(&json!({ "label": "Home" })) }));
    }

    #[semio_framework_async_macros::async_test]
    async fn gis_map_routes_op_lines_round_trip() {
        store::os_store::test_support::assert_op_line_round_trip(&GisMapMutation::CreateRoute(create_route::CreateRoute { index: 0, item: sample_feature("p1") }));
        store::os_store::test_support::assert_op_line_round_trip(&GisMapMutation::ReorderRoutes(reorder_routes::ReorderRoutes { id: "p1".into(), to_index: 1 }));
    }

    #[semio_framework_async_macros::async_test]
    async fn gis_map_regions_op_lines_round_trip() {
        store::os_store::test_support::assert_op_line_round_trip(&GisMapMutation::CreateRegion(create_region::CreateRegion { index: 0, item: sample_feature("p1") }));
        store::os_store::test_support::assert_op_line_round_trip(&GisMapMutation::ReorderRegions(reorder_regions::ReorderRegions { id: "p1".into(), to_index: 2 }));
    }

    #[semio_framework_async_macros::async_test]
    async fn gis_map_document_text_round_trips_through_store() {
        let initial = empty_gis_map_snapshot();
        let envelope = store::create_document_envelope(GIS_MAP_SCHEMA, "gis2d-demo", initial, None);
        let mut store = store::ArtifactStore::new(envelope).expect("valid artifact store fixture");
        store.dispatch(store::ArtifactCommand::Apply { mutations: vec![GisMapMutation::CreatePosition(create_position::CreatePosition { index: 0, item: sample_feature("p1") })], description: None }).expect("apply");
        store::os_store::test_support::assert_document_text_round_trip(&store);
        store::os_store::test_support::assert_document_pack_round_trip(&store);
    }

    #[semio_framework_async_macros::async_test]
    async fn gis_map_default_document_is_non_empty() {
        assert!(!default_document().positions.is_empty());
    }

    fn empty_gis_map_initializer(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation) -> GisMapStoreInitializationAuthority {
        let envelope = store::create_document_envelope(GIS_MAP_SCHEMA, "gis-map-retained-load", empty_gis_map_snapshot(), None);
        GisMapStoreInitializationAuthority::new(envelope, operation, generation)
    }

    fn drive_gis_map_initializer(authority: &mut GisMapStoreInitializationAuthority, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation) -> semio_framework_job::StepOutcome {
        let cancel = semio_framework_job::root_cancel_token();
        let mut preview_sequence = 0;
        for _ in 0..100_000 {
            let mut context = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(4_096, u64::MAX), cancel.clone(), semio_framework_job::default_now_us, &mut preview_sequence);
            let outcome = semio_framework_plugin::ArtifactStoreInitializationAuthority::step(authority, &mut context);
            if outcome.is_terminal() {
                return outcome;
            }
        }
        panic!("GIS retained initializer did not reach a bounded terminal")
    }

    fn close_gis_map_candidate(mut candidate: store::ArtifactStore<GisMapSnapshot, GisMapMutation>) {
        use semio_framework_plugin::ArtifactOwnedDisposer;

        let mut disposer = semio_framework_plugin::ArtifactDocumentStoreDisposer::<GisMapSnapshot, GisMapMutation>::new();
        for _ in 0..100_000 {
            match disposer.close_step(&mut candidate, 1, GIS_MAP_OWNED_FIELD_BYTES).expect("GIS candidate close step") {
                semio_framework_plugin::PluginCloseStep::Pending { released_items, released_bytes } => {
                    assert!(released_items <= 1);
                    assert!(released_bytes <= GIS_MAP_OWNED_FIELD_BYTES);
                }
                semio_framework_plugin::PluginCloseStep::Blocked { reason } => panic!("fresh GIS candidate close unexpectedly blocked: {reason}"),
                semio_framework_plugin::PluginCloseStep::Complete => {
                    assert!(disposer.terminal_is_empty(&candidate));
                    drop(disposer);
                    drop(candidate);
                    return;
                }
            }
        }
        panic!("GIS candidate did not reach terminal-empty close")
    }

    #[test]
    fn gis_map_store_initializer_publishes_next_generation_and_candidate_closes_incrementally() {
        let operation = semio_framework_job::OperationId(601);
        let generation = semio_framework_job::Generation(21);
        let mut authority = empty_gis_map_initializer(operation, generation);
        assert!(matches!(drive_gis_map_initializer(&mut authority, operation, generation), semio_framework_job::StepOutcome::Complete(_)));
        let candidate = semio_framework_plugin::ArtifactStoreInitializationAuthority::take_candidate(&mut authority).expect("exact GIS candidate");
        assert_eq!(candidate.generation_now(), 22);
        assert!(semio_framework_plugin::ArtifactStoreInitializationAuthority::terminal_is_empty(&authority));
        drop(authority);
        close_gis_map_candidate(candidate);
    }

    #[test]
    fn gis_map_store_initializer_cancel_and_stale_generation_return_every_owner_terminal_empty() {
        let operation = semio_framework_job::OperationId(602);
        let generation = semio_framework_job::Generation(23);
        let mut cancelled = empty_gis_map_initializer(operation, generation);
        semio_framework_plugin::ArtifactStoreInitializationAuthority::request_cancel(&mut cancelled);
        assert!(matches!(drive_gis_map_initializer(&mut cancelled, operation, generation), semio_framework_job::StepOutcome::Cancelled));
        assert!(semio_framework_plugin::ArtifactStoreInitializationAuthority::terminal_is_empty(&cancelled));
        drop(cancelled);

        let mut stale = empty_gis_map_initializer(operation, generation);
        assert!(matches!(drive_gis_map_initializer(&mut stale, operation, semio_framework_job::Generation(generation.0 + 1)), semio_framework_job::StepOutcome::Fault(_)));
        assert!(semio_framework_plugin::ArtifactStoreInitializationAuthority::terminal_is_empty(&stale));
        drop(stale);
    }

    #[test]
    fn gis_map_nested_value_mutation_and_all_child_handles_retire_one_owner_per_grant() {
        fn drain(mut retirement: Box<dyn store::ErasedSnapshotRetirement>) {
            for _ in 0..10_000 {
                match retirement.close_step(1, GIS_MAP_OWNED_FIELD_BYTES).expect("one nested GIS owner retires") {
                    store::SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                        assert!(released_items <= 1);
                        assert!(released_bytes <= GIS_MAP_OWNED_FIELD_BYTES);
                    }
                    store::SnapshotRetirementStep::Complete => {
                        assert!(retirement.terminal_is_empty());
                        drop(retirement);
                        return;
                    }
                    store::SnapshotRetirementStep::Blocked => panic!("owned GIS retirement cannot block"),
                }
            }
            panic!("nested GIS retirement did not reach terminal")
        }

        let mut snapshot = empty_gis_map_snapshot();
        snapshot.image = Some(store::ArtifactChild::new("image-child".into(), snapshot.drawing.target.clone()));
        drain(store::ArtifactOwnedValueRetirementFactory::retire_owned(&GisMapSnapshotRetirementFactory, snapshot));

        let mutation = GisMapMutation::ReplacePositionData(replace_position_data::ReplacePositionData {
            id: "position".repeat(32),
            new_data: dsl::DslValue::Object(vec![("nested".repeat(32), dsl::DslValue::Array(vec![dsl::DslValue::String("payload".repeat(128)), dsl::DslValue::String("tail".into())]))]),
        });
        drain(store::ArtifactOwnedValueRetirementFactory::retire_owned(&GisMapMutationRetirementFactory, mutation));
    }

    #[test]
    fn gis_map_all_twelve_mutation_variants_preserve_catalog_order_and_zero_grant_ownership() {
        let feature = |id: &str| MapFeature { id: id.into(), data: dsl::DslValue::Null };
        let mutations = vec![
            GisMapMutation::CreatePosition(create_position::CreatePosition { index: 0, item: feature("position") }),
            GisMapMutation::DeletePosition(delete_position::DeletePosition { id: "position".into() }),
            GisMapMutation::ReorderPositions(reorder_positions::ReorderPositions { id: "position".into(), to_index: 1 }),
            GisMapMutation::ReplacePositionData(replace_position_data::ReplacePositionData { id: "position".into(), new_data: dsl::DslValue::Null }),
            GisMapMutation::CreateRoute(create_route::CreateRoute { index: 0, item: feature("route") }),
            GisMapMutation::DeleteRoute(delete_route::DeleteRoute { id: "route".into() }),
            GisMapMutation::ReorderRoutes(reorder_routes::ReorderRoutes { id: "route".into(), to_index: 1 }),
            GisMapMutation::ReplaceRouteData(replace_route_data::ReplaceRouteData { id: "route".into(), new_data: dsl::DslValue::Null }),
            GisMapMutation::CreateRegion(create_region::CreateRegion { index: 0, item: feature("region") }),
            GisMapMutation::DeleteRegion(delete_region::DeleteRegion { id: "region".into() }),
            GisMapMutation::ReorderRegions(reorder_regions::ReorderRegions { id: "region".into(), to_index: 1 }),
            GisMapMutation::ReplaceRegionData(replace_region_data::ReplaceRegionData { id: "region".into(), new_data: dsl::DslValue::Null }),
        ];
        for mutation in mutations {
            let mut retirement = store::ArtifactOwnedValueRetirementFactory::retire_owned(&GisMapMutationRetirementFactory, mutation);
            assert!(matches!(retirement.close_step(0, GIS_MAP_OWNED_FIELD_BYTES).expect("zero-grant GIS retirement"), store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }));
            for _ in 0..100 {
                match retirement.close_step(1, GIS_MAP_OWNED_FIELD_BYTES).expect("one catalog owner retires") {
                    store::SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                        assert!(released_items <= 1);
                        assert!(released_bytes <= GIS_MAP_OWNED_FIELD_BYTES);
                    }
                    store::SnapshotRetirementStep::Complete => {
                        assert!(retirement.terminal_is_empty());
                        break;
                    }
                    store::SnapshotRetirementStep::Blocked => panic!("unshared GIS mutation owner cannot block"),
                }
            }
            assert!(retirement.terminal_is_empty());
            drop(retirement);
        }
    }
}
//#endregion 🧪️Tests
