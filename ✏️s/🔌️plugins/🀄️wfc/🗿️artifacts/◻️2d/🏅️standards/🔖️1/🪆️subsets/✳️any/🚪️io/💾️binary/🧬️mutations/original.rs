//! 📥️ Receives the authored ordinal operation records under the archive decoder's original grant.

use crate::schema::mutations::{
    Wfc2dMutation, change_seed::ChangeSeed, change_tile_media::ChangeTileMedia,
    change_tile_weight::ChangeTileWeight, connect_slots::ConnectSlots, create_rule::CreateRule,
    create_slot::CreateSlot, create_tile::CreateTile, delete_rule::DeleteRule, delete_slot::DeleteSlot,
    delete_tile::DeleteTile, disconnect_slots::DisconnectSlots, drag_slots::DragSlots,
    move_slot::MoveSlot, pin_slot::PinSlot, resize_slot::ResizeSlot,
    set_slot_positions::{SetSlotPositions, Wfc2dSlotPosition}, unpin_slot::UnpinSlot,
};
use crate::schema::snapshot::{Wfc2dTile, Wfc2dTileMedia};
use crate::standards::v1::subsets::any::io::text::snapshot::{
    Wfc2dRuleDsl, Wfc2dSlotDsl, Wfc2dSlotEdgeDsl, edge_from_dsl, rule_from_dsl, slot_from_dsl,
};
use dsl::os_dsl::original_receiving::converted::OriginalConvertedOperation;
use semio_framework_dsl_record::{
    BorrowedDslField, BorrowedShape, DslField, FieldValue, Shape,
    receiving::owned::{DslOwnedReceivingField, NativeSourceOrigin, NativeSourceReceiving, invalid},
};
use semio_framework_dsl_record_derive::{DslEnum, DslOwnedReceiving, DslRecord};
use semio_framework_value::{
    DslValue, FromValue, NativeDecodeControl, RetainedAllocationEffect, RetainedCloneProgress,
    RetireOwned, RetirementDemand, ToValue, ValueError, ValueRefusalKind,
};
use semio_framework_value::native_decoding::{NativeReceive, owned::{NativeMappedReceiver, NativeReceiveMap}};

#[derive(RetireOwned)]
pub struct WireMedia(Option<Wfc2dTileMedia>);

impl NativeReceiveMap<Option<Wfc2dTileMedia>> for WireMedia {
    fn from_native(media: Option<Wfc2dTileMedia>) -> Self { Self(media) }
}

impl NativeReceive<DslValue> for WireMedia {
    type Receiver = NativeMappedReceiver<DslValue, Option<Wfc2dTileMedia>, Self>;
    fn receiver() -> Self::Receiver { NativeMappedReceiver::new() }
}

pub struct WireMediaOrigin;

impl NativeSourceOrigin<DslValue> for WireMediaOrigin {
    const RECORD_FIELD: Option<u16> = None;

    fn check(field: &Option<FieldValue>) -> Result<(), ValueError> {
        match field {
            Some(FieldValue::Value(_)) => Ok(()),
            _ => Err(invalid()),
        }
    }

    fn extract(field: &mut Option<FieldValue>) -> Result<DslValue, ValueError> {
        Self::check(field)?;
        let Some(FieldValue::Value(value)) = field.take() else { unreachable!() };
        Ok(value)
    }
}

impl DslOwnedReceivingField for WireMedia {
    type Cursor = NativeSourceReceiving<DslValue, Self, WireMediaOrigin>;
    fn receiving_cursor() -> Self::Cursor { Self::Cursor::default() }
}

impl BorrowedDslField for WireMedia {
    const SHAPE: BorrowedShape = BorrowedShape::Value;
}

impl DslField for WireMedia {
    fn shape() -> Shape { Shape::Value }

    fn to_value(&self) -> FieldValue { FieldValue::Value(self.0.to_value()) }

    fn from_value(field: &FieldValue) -> Result<Self, String> {
        let FieldValue::Value(value) = field else { return Err("expected WFC2D tile media".into()); };
        Option::<Wfc2dTileMedia>::from_value(value.clone()).map(Self).map_err(ValueError::into_message)
    }
}

#[derive(DslRecord, DslOwnedReceiving, RetireOwned)]
pub struct WireTile {
    id: String,
    label: Option<String>,
    weight: f64,
    media: WireMedia,
}

/// 🪞️ Matches the text mirror's variant order and record tags used by the binary encoder.
#[derive(DslEnum, DslOwnedReceiving, RetireOwned)]
pub enum Wfc2dWireOperation {
    ChangeSeed { seed: u64 },
    CreateSlot { #[dsl(block)] slot: Wfc2dSlotDsl },
    DeleteSlot { id: String },
    MoveSlot { id: String, x: f64, y: f64 },
    ResizeSlot { id: String, width: f64, height: f64 },
    ConnectSlots { #[dsl(block)] edge: Wfc2dSlotEdgeDsl },
    DisconnectSlots { id: String },
    PinSlot { id: String, tile_id: String },
    UnpinSlot { id: String },
    CreateTile { #[dsl(block)] tile: WireTile },
    DeleteTile { id: String },
    ChangeTileWeight { tile_id: String, weight: f64 },
    ChangeTileMedia { tile_id: String, media: WireMedia },
    CreateRule { #[dsl(block)] rule: Wfc2dRuleDsl },
    DeleteRule { id: String },
    DragSlots { targets: Vec<String>, dx: f64, dy: f64 },
    SetSlotPositions { ids: Vec<String>, xs: Vec<f64>, ys: Vec<f64> },
}

impl protocol::OpBinary for Wfc2dWireOperation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}

dsl::original_ordinal_operation_receiving!(Wfc2dWireOperation);

fn move_operation(operation: Wfc2dWireOperation) -> Wfc2dMutation {
    use Wfc2dWireOperation as Wire;
    match operation {
        Wire::ChangeSeed { seed } => Wfc2dMutation::ChangeSeed(ChangeSeed { seed }),
        Wire::CreateSlot { slot } => Wfc2dMutation::CreateSlot(CreateSlot { slot: slot_from_dsl(slot) }),
        Wire::DeleteSlot { id } => Wfc2dMutation::DeleteSlot(DeleteSlot { id }),
        Wire::MoveSlot { id, x, y } => Wfc2dMutation::MoveSlot(MoveSlot { id, x, y }),
        Wire::ResizeSlot { id, width, height } => Wfc2dMutation::ResizeSlot(ResizeSlot { id, width, height }),
        Wire::ConnectSlots { edge } => Wfc2dMutation::ConnectSlots(ConnectSlots { edge: edge_from_dsl(edge) }),
        Wire::DisconnectSlots { id } => Wfc2dMutation::DisconnectSlots(DisconnectSlots { id }),
        Wire::PinSlot { id, tile_id } => Wfc2dMutation::PinSlot(PinSlot { id, tile_id }),
        Wire::UnpinSlot { id } => Wfc2dMutation::UnpinSlot(UnpinSlot { id }),
        Wire::CreateTile { tile } => Wfc2dMutation::CreateTile(CreateTile {
            tile: Wfc2dTile { id: tile.id, label: tile.label, weight: tile.weight, media: tile.media.0.unwrap_or_default() },
        }),
        Wire::DeleteTile { id } => Wfc2dMutation::DeleteTile(DeleteTile { id }),
        Wire::ChangeTileWeight { tile_id, weight } => Wfc2dMutation::ChangeTileWeight(ChangeTileWeight { tile_id, weight }),
        Wire::ChangeTileMedia { tile_id, media } => Wfc2dMutation::ChangeTileMedia(ChangeTileMedia { tile_id, media: media.0.unwrap_or_default() }),
        Wire::CreateRule { rule } => Wfc2dMutation::CreateRule(CreateRule { rule: rule_from_dsl(rule) }),
        Wire::DeleteRule { id } => Wfc2dMutation::DeleteRule(DeleteRule { id }),
        Wire::DragSlots { targets, dx, dy } => Wfc2dMutation::DragSlots(DragSlots { targets, dx, dy }),
        Wire::SetSlotPositions { .. } => unreachable!("grouped positions use the retained row converter"),
    }
}

#[derive(Default, RetireOwned)]
pub struct PositionRows {
    rows: Option<Vec<Wfc2dSlotPosition>>,
}

impl OriginalConvertedOperation for Wfc2dMutation {
    type Mirror = Wfc2dWireOperation;
    type Conversion = PositionRows;

    fn conversion_demand(
        state: &PositionRows,
        input: &Option<Wfc2dWireOperation>,
        _body: usize,
    ) -> Result<RetirementDemand, ValueError> {
        let mut demand = RetirementDemand { depth: 1, ..Default::default() };
        match input.as_ref().ok_or_else(|| ValueError::invariant("WFC2D operation source is absent"))? {
            Wfc2dWireOperation::SetSlotPositions { ids, .. } => match &state.rows {
                None => {
                    demand.copy_bytes = size_of::<Option<Vec<Wfc2dSlotPosition>>>();
                    demand.capacity_bytes = ids.len().checked_mul(size_of::<Wfc2dSlotPosition>())
                        .filter(|bytes| *bytes <= isize::MAX as usize)
                        .ok_or_else(|| ValueError::literal(ValueRefusalKind::OwnershipLimit, "WFC2D position row size overflow"))?;
                }
                Some(rows) if rows.len() < ids.len() => {
                    demand.copy_bytes = size_of::<Wfc2dSlotPosition>() + size_of::<String>();
                }
                Some(_) => {
                    demand.copy_bytes = size_of::<Option<Vec<Wfc2dSlotPosition>>>() + size_of::<Option<Wfc2dMutation>>();
                }
            },
            _ => {
                demand.copy_bytes = size_of::<Option<Wfc2dWireOperation>>() + size_of::<Option<Wfc2dMutation>>();
            }
        }
        Ok(demand)
    }

    fn convert_step(
        state: &mut PositionRows,
        input: &mut Option<Wfc2dWireOperation>,
        output: &mut Option<Wfc2dMutation>,
        body: usize,
        _native: &mut NativeDecodeControl<'_>,
        effect: &mut RetainedAllocationEffect<'_>,
    ) -> Result<RetainedCloneProgress, ValueError> {
        let demand = Self::conversion_demand(state, input, body)?;
        if output.is_some() {
            return Err(ValueError::invariant("WFC2D operation output is already occupied"));
        }
        if !effect.performed(1)? {
            return Ok(RetainedCloneProgress::default());
        }
        let mut progress = RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() };
        let Some(Wfc2dWireOperation::SetSlotPositions { ids, xs, ys }) = input.as_mut() else {
            *output = input.take().map(move_operation);
            return Ok(progress);
        };
        if xs.len() != ids.len() || ys.len() != ids.len() {
            return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "set-slot-positions carries one x and one y per id"));
        }
        match state.rows.as_mut() {
            None => {
                let mut rows = Vec::new();
                if demand.capacity_bytes != 0 {
                    *effect.actual() = Some(0);
                    rows.try_reserve_exact(ids.len()).map_err(|_| ValueError::literal(
                        ValueRefusalKind::AllocationFailed, "WFC2D position row allocation failed",
                    ))?;
                    progress.retained_capacity_bytes = rows.capacity() * size_of::<Wfc2dSlotPosition>();
                    *effect.actual() = Some(progress.retained_capacity_bytes);
                }
                state.rows = Some(rows);
            }
            Some(rows) if rows.len() < ids.len() => {
                let index = rows.len();
                rows.push(Wfc2dSlotPosition { id: std::mem::take(&mut ids[index]), x: xs[index], y: ys[index] });
            }
            Some(_) => {
                *output = state.rows.take().map(|positions| Wfc2dMutation::SetSlotPositions(SetSlotPositions { positions }));
            }
        }
        Ok(progress)
    }
}

dsl::os_dsl::original_converted_operation_receiving!(Wfc2dMutation);
