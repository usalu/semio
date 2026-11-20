//! 🛬️ Receives the authored tagged operation and assembles each grouped slot row under the caller's grant.

use super::{COMPONENT_PROTOCOL_SEMIO, OpBinary};
use crate::schema::mutations::{
    Wfc3dMutation, change_seed::ChangeSeed, change_tile_media::ChangeTileMedia,
    change_tile_weight::ChangeTileWeight, connect_slots::ConnectSlots, create_rule::CreateRule,
    create_slot::CreateSlot, create_tile::CreateTile, delete_rule::DeleteRule, delete_slot::DeleteSlot,
    delete_tile::DeleteTile, disconnect_slots::DisconnectSlots, drag_slots::DragSlots,
    move_slot::MoveSlot, pin_slot::PinSlot, resize_slot::ResizeSlot,
    set_slot_positions::{SetSlotPositions, Wfc3dSlotPosition}, unpin_slot::UnpinSlot,
};
use crate::schema::snapshot::{Tile, TileMedia3d};
use crate::standards::v1::subsets::any::io::text::snapshot::{
    GraphRuleDsl, Slot3dDsl, SlotEdgeDsl, edge_from_dsl, rule_from_dsl, slot_from_dsl,
};
use dsl::original_receiving::converted::OriginalConvertedOperation;
use semio_framework_dsl_record::{
    BorrowedDslField, BorrowedShape, DslField, FieldValue, Shape,
    receiving::owned::{DslOwnedReceivingField, NativeSourceOrigin, NativeSourceReceiving, invalid},
};
use semio_framework_dsl_record_derive::{DslEnum, DslOwnedReceiving, DslRecord};
use semio_framework_value::{
    DslValue, FromValue, RetainedAllocationEffect, RetainedCloneProgress, RetireOwned,
    RetirementDemand, ToValue, ValueError, ValueRefusalKind,
    native_decoding::{NativeDecodeControl, NativeReceive, owned::{NativeMappedReceiver, NativeReceiveMap}},
};
use std::mem::{size_of, take};

#[derive(RetireOwned)]
pub struct WireMedia(Option<TileMedia3d>);

impl NativeReceiveMap<Option<TileMedia3d>> for WireMedia {
    fn from_native(media: Option<TileMedia3d>) -> Self { Self(media) }
}

impl NativeReceive<DslValue> for WireMedia {
    type Receiver = NativeMappedReceiver<DslValue, Option<TileMedia3d>, Self>;
    fn receiver() -> Self::Receiver { Self::Receiver::new() }
}

pub struct MediaOrigin;

impl NativeSourceOrigin<DslValue> for MediaOrigin {
    const RECORD_FIELD: Option<u16> = None;

    fn check(field: &Option<FieldValue>) -> Result<(), ValueError> {
        match field { Some(FieldValue::Value(_)) => Ok(()), _ => Err(invalid()) }
    }

    fn extract(field: &mut Option<FieldValue>) -> Result<DslValue, ValueError> {
        Self::check(field)?;
        let Some(FieldValue::Value(value)) = field.take() else { unreachable!() };
        Ok(value)
    }
}

impl DslOwnedReceivingField for WireMedia {
    type Cursor = NativeSourceReceiving<DslValue, Self, MediaOrigin>;
    fn receiving_cursor() -> Self::Cursor { Self::Cursor::default() }
}

impl BorrowedDslField for WireMedia {
    const SHAPE: BorrowedShape = BorrowedShape::Value;
}

impl DslField for WireMedia {
    fn shape() -> Shape { Shape::Value }

    fn to_value(&self) -> FieldValue { FieldValue::Value(self.0.to_value()) }

    fn from_value(field: &FieldValue) -> Result<Self, String> {
        let FieldValue::Value(value) = field else { return Err("expected WFC 3D media".into()); };
        Option::<TileMedia3d>::from_value(value.clone()).map(Self).map_err(ValueError::into_message)
    }
}

#[derive(DslRecord, DslOwnedReceiving, RetireOwned)]
pub struct WireTile {
    id: String,
    label: Option<String>,
    weight: f64,
    media: WireMedia,
}

#[derive(DslEnum, DslOwnedReceiving, RetireOwned)]
pub enum WireOperation {
    CreateSlot { index: usize, #[dsl(block)] slot: Slot3dDsl },
    DeleteSlot { id: String },
    MoveSlot { id: String, x: f64, y: f64, z: f64 },
    ResizeSlot { id: String, width: f64, height: f64, depth: f64 },
    ConnectSlots { index: usize, #[dsl(block)] edge: SlotEdgeDsl },
    DisconnectSlots { id: String },
    PinSlot { id: String, tile_id: String },
    UnpinSlot { id: String },
    CreateTile { index: usize, #[dsl(block)] tile: WireTile },
    DeleteTile { id: String },
    ChangeTileWeight { id: String, weight: f64 },
    ChangeTileMedia { id: String, media: WireMedia },
    CreateRule { index: usize, #[dsl(block)] rule: GraphRuleDsl },
    DeleteRule { id: String },
    ChangeSeed { seed: u64 },
    DragSlots { targets: Vec<String>, dx: f64, dy: f64, dz: f64 },
    SetSlotPositions { ids: Vec<String>, xs: Vec<f64>, ys: Vec<f64>, zs: Vec<f64> },
}

impl OpBinary for WireOperation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_tagged_op(COMPONENT_PROTOCOL_SEMIO, self)
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_tagged_op(COMPONENT_PROTOCOL_SEMIO, bytes)
    }
}

dsl::original_tagged_operation_receiving!(WireOperation, COMPONENT_PROTOCOL_SEMIO);

fn move_operation(operation: WireOperation) -> Wfc3dMutation {
    match operation {
        WireOperation::CreateSlot { index, slot } => Wfc3dMutation::CreateSlot(CreateSlot { index, slot: slot_from_dsl(slot) }),
        WireOperation::DeleteSlot { id } => Wfc3dMutation::DeleteSlot(DeleteSlot { id }),
        WireOperation::MoveSlot { id, x, y, z } => Wfc3dMutation::MoveSlot(MoveSlot { id, x, y, z }),
        WireOperation::ResizeSlot { id, width, height, depth } => Wfc3dMutation::ResizeSlot(ResizeSlot { id, width, height, depth }),
        WireOperation::ConnectSlots { index, edge } => Wfc3dMutation::ConnectSlots(ConnectSlots { index, edge: edge_from_dsl(edge) }),
        WireOperation::DisconnectSlots { id } => Wfc3dMutation::DisconnectSlots(DisconnectSlots { id }),
        WireOperation::PinSlot { id, tile_id } => Wfc3dMutation::PinSlot(PinSlot { id, tile_id }),
        WireOperation::UnpinSlot { id } => Wfc3dMutation::UnpinSlot(UnpinSlot { id }),
        WireOperation::CreateTile { index, tile: WireTile { id, label, weight, media } } => {
            Wfc3dMutation::CreateTile(CreateTile { index, tile: Tile { id, label, weight, media: media.0.unwrap_or_default() } })
        }
        WireOperation::DeleteTile { id } => Wfc3dMutation::DeleteTile(DeleteTile { id }),
        WireOperation::ChangeTileWeight { id, weight } => Wfc3dMutation::ChangeTileWeight(ChangeTileWeight { id, weight }),
        WireOperation::ChangeTileMedia { id, media } => Wfc3dMutation::ChangeTileMedia(ChangeTileMedia { id, media: media.0.unwrap_or_default() }),
        WireOperation::CreateRule { index, rule } => Wfc3dMutation::CreateRule(CreateRule { index, rule: rule_from_dsl(rule) }),
        WireOperation::DeleteRule { id } => Wfc3dMutation::DeleteRule(DeleteRule { id }),
        WireOperation::ChangeSeed { seed } => Wfc3dMutation::ChangeSeed(ChangeSeed { seed }),
        WireOperation::DragSlots { targets, dx, dy, dz } => Wfc3dMutation::DragSlots(DragSlots { targets, dx, dy, dz }),
        WireOperation::SetSlotPositions { .. } => unreachable!(),
    }
}

#[derive(Default, RetireOwned)]
pub struct Conversion {
    rows: Vec<Wfc3dSlotPosition>,
}

impl OriginalConvertedOperation for Wfc3dMutation {
    type Mirror = WireOperation;
    type Conversion = Conversion;

    fn conversion_demand(state: &Conversion, input: &Option<WireOperation>, _: usize) -> Result<RetirementDemand, ValueError> {
        let capacity_bytes = match input {
            Some(WireOperation::SetSlotPositions { ids, xs, ys, zs }) if state.rows.capacity() == 0 => {
                if ids.len() != xs.len() || ids.len() != ys.len() || ids.len() != zs.len() {
                    return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "set-slot-positions carries one x, y and z per id"));
                }
                ids.len().checked_mul(size_of::<Wfc3dSlotPosition>()).ok_or_else(|| ValueError::invariant("WFC 3D position row capacity overflow"))?
            }
            _ => 0,
        };
        let copy_bytes = match input {
            Some(WireOperation::SetSlotPositions { ids, .. }) if state.rows.len() < ids.len() && capacity_bytes == 0 => size_of::<Wfc3dSlotPosition>(),
            _ => size_of::<Option<WireOperation>>() + size_of::<Option<Wfc3dMutation>>(),
        };
        Ok(RetirementDemand { copy_bytes, capacity_bytes, depth: 1, ..Default::default() })
    }

    fn convert_step(
        state: &mut Conversion, input: &mut Option<WireOperation>, output: &mut Option<Self>, body: usize,
        _: &mut NativeDecodeControl<'_>, effect: &mut RetainedAllocationEffect<'_>,
    ) -> Result<RetainedCloneProgress, ValueError> {
        let demand = Self::conversion_demand(state, input, body)?;
        if !effect.performed(1)? {
            return Ok(Default::default());
        }
        let mut progress = RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() };
        match input.as_mut() {
            Some(WireOperation::SetSlotPositions { ids, xs, ys, zs }) => {
                if demand.capacity_bytes != 0 {
                    *effect.actual() = Some(0);
                    state.rows.try_reserve_exact(ids.len()).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "WFC 3D position row allocation refused"))?;
                    let capacity = state.rows.capacity() * size_of::<Wfc3dSlotPosition>();
                    *effect.actual() = Some(capacity);
                    progress.retained_capacity_bytes = capacity;
                } else if let Some(id) = ids.get_mut(state.rows.len()) {
                    let index = state.rows.len();
                    state.rows.push(Wfc3dSlotPosition { id: take(id), x: xs[index], y: ys[index], z: zs[index] });
                } else {
                    *output = Some(Self::SetSlotPositions(SetSlotPositions { positions: take(&mut state.rows) }));
                }
            }
            Some(_) => *output = input.take().map(move_operation),
            None => return Err(ValueError::invariant("WFC 3D original operation input is absent")),
        }
        Ok(progress)
    }
}

dsl::original_converted_operation_receiving!(Wfc3dMutation);

