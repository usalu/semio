//! 🛬️ Receives the tagged mutation wire while its media and literal direction still have owners.

use super::{COMPONENT_PROTOCOL_SEMIO, OpBinary};
use crate::schema::mutations::{
    Grid2dMutation, change_cell_size::ChangeCellSize, change_periodicity::ChangePeriodicity,
    change_seed::ChangeSeed, change_tile_media::ChangeTileMedia, change_tile_weight::ChangeTileWeight,
    create_rule::CreateRule, create_tile::CreateTile, delete_rule::DeleteRule, delete_tile::DeleteTile,
    mask_cell::MaskCell, pin_cell::PinCell, resize_grid::ResizeGrid, unmask_cell::UnmaskCell,
    unpin_cell::UnpinCell,
};
use crate::schema::snapshot::{WfcAdjacencyRule2d, WfcDirection2d, WfcTile2d, WfcTileMedia2d};
use crate::standards::v1::subsets::any::io::text::snapshot::direction_token;
use semio_framework_dsl_record::{
    BorrowedDslField, BorrowedShape, DslField, FieldValue, Shape,
    receiving::owned::{DslOwnedReceivingField, NativeSourceOrigin, NativeSourceReceiving, invalid},
};
use semio_framework_dsl_record_derive::{DslEnum, DslOwnedReceiving, DslRecord};
use semio_framework_value::{DslValue, FromValue, RetireOwned, ToValue, ValueError};
use semio_framework_value::native_decoding::{
    NativeReceive,
    owned::{NativeMappedReceiver, NativeReceiveMap},
};

#[derive(RetireOwned)]
pub struct Grid2dWireMedia(Option<WfcTileMedia2d>);

impl NativeReceiveMap<Option<WfcTileMedia2d>> for Grid2dWireMedia {
    fn from_native(media: Option<WfcTileMedia2d>) -> Self {
        Self(media)
    }
}

impl NativeReceive<DslValue> for Grid2dWireMedia {
    type Receiver = NativeMappedReceiver<DslValue, Option<WfcTileMedia2d>, Self>;

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

impl DslOwnedReceivingField for Grid2dWireMedia {
    type Cursor = NativeSourceReceiving<DslValue, Self, WireMediaOrigin>;

    fn receiving_cursor() -> Self::Cursor { Self::Cursor::default() }
}

impl BorrowedDslField for Grid2dWireMedia {
    const SHAPE: BorrowedShape = BorrowedShape::Value;
}

impl DslField for Grid2dWireMedia {
    fn shape() -> Shape { Shape::Value }

    fn to_value(&self) -> FieldValue { FieldValue::Value(self.0.to_value()) }

    fn from_value(field: &FieldValue) -> Result<Self, String> {
        let FieldValue::Value(value) = field else { return Err("expected Grid2d media".into()); };
        <Option<WfcTileMedia2d> as FromValue>::from_value(value.clone())
            .map(NativeReceiveMap::from_native)
            .map_err(ValueError::into_message)
    }
}

#[derive(Clone, Copy, RetireOwned)]
pub struct Grid2dWireDirection(WfcDirection2d);

fn parse_direction(value: &FieldValue) -> Result<Grid2dWireDirection, ValueError> {
    let FieldValue::Text(value) = value else { return Err(invalid()); };
    match value.as_str() {
        "LEFT" => Ok(Grid2dWireDirection(WfcDirection2d::Left)),
        "RIGHT" => Ok(Grid2dWireDirection(WfcDirection2d::Right)),
        "TOP" => Ok(Grid2dWireDirection(WfcDirection2d::Top)),
        "BOTTOM" => Ok(Grid2dWireDirection(WfcDirection2d::Bottom)),
        _ => Err(invalid()),
    }
}

semio_framework_dsl_record::dsl_owned_scalar_receiving!(copy Grid2dWireDirection, parse_direction);

impl BorrowedDslField for Grid2dWireDirection {
    const SHAPE: BorrowedShape = BorrowedShape::Text;
}

impl DslField for Grid2dWireDirection {
    fn shape() -> Shape { Shape::Text }

    fn to_value(&self) -> FieldValue { FieldValue::Text(direction_token(self.0).into()) }

    fn from_value(value: &FieldValue) -> Result<Self, String> {
        parse_direction(value).map_err(ValueError::into_message)
    }
}

#[derive(DslRecord, DslOwnedReceiving, RetireOwned)]
pub struct Grid2dWireTile {
    id: String,
    label: Option<String>,
    weight: f64,
    media: Grid2dWireMedia,
}

#[derive(DslRecord, DslOwnedReceiving, RetireOwned)]
pub struct Grid2dWireRule {
    id: String,
    tile_a_id: String,
    tile_b_id: String,
    direction: Grid2dWireDirection,
    allowed: bool,
}

#[derive(DslEnum, DslOwnedReceiving, RetireOwned)]
pub enum Grid2dWireOperation {
    ChangeSeed { seed: u64 },
    ResizeGrid { width: u32, height: u32 },
    ChangeCellSize { cell_width: f64, cell_height: f64 },
    ChangePeriodicity { periodic_x: bool, periodic_y: bool },
    CreateTile { #[dsl(block)] tile: Grid2dWireTile },
    DeleteTile { id: String },
    ChangeTileWeight { id: String, weight: f64 },
    ChangeTileMedia { id: String, media: Grid2dWireMedia },
    CreateRule { #[dsl(block)] rule: Grid2dWireRule },
    DeleteRule { id: String },
    PinCell { x: u32, y: u32, tile_id: String },
    UnpinCell { x: u32, y: u32 },
    MaskCell { x: u32, y: u32 },
    UnmaskCell { x: u32, y: u32 },
}

impl OpBinary for Grid2dWireOperation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_tagged_op(COMPONENT_PROTOCOL_SEMIO, self)
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_tagged_op(COMPONENT_PROTOCOL_SEMIO, bytes)
    }
}

fn receive_operation(operation: Grid2dWireOperation) -> Grid2dMutation {
    match operation {
        Grid2dWireOperation::ChangeSeed { seed } => Grid2dMutation::ChangeSeed(ChangeSeed { seed }),
        Grid2dWireOperation::ResizeGrid { width, height } => Grid2dMutation::ResizeGrid(ResizeGrid { width, height }),
        Grid2dWireOperation::ChangeCellSize { cell_width, cell_height } => Grid2dMutation::ChangeCellSize(ChangeCellSize { cell_width, cell_height }),
        Grid2dWireOperation::ChangePeriodicity { periodic_x, periodic_y } => Grid2dMutation::ChangePeriodicity(ChangePeriodicity { periodic_x, periodic_y }),
        Grid2dWireOperation::CreateTile { tile: Grid2dWireTile { id, label, weight, media } } => {
            Grid2dMutation::CreateTile(CreateTile { tile: WfcTile2d { id, label, weight, media: media.0.unwrap_or_default() } })
        }
        Grid2dWireOperation::DeleteTile { id } => Grid2dMutation::DeleteTile(DeleteTile { id }),
        Grid2dWireOperation::ChangeTileWeight { id, weight } => Grid2dMutation::ChangeTileWeight(ChangeTileWeight { id, weight }),
        Grid2dWireOperation::ChangeTileMedia { id, media } => Grid2dMutation::ChangeTileMedia(ChangeTileMedia { id, media: media.0.unwrap_or_default() }),
        Grid2dWireOperation::CreateRule { rule: Grid2dWireRule { id, tile_a_id, tile_b_id, direction, allowed } } => {
            Grid2dMutation::CreateRule(CreateRule { rule: WfcAdjacencyRule2d { id, tile_a_id, tile_b_id, direction: direction.0, allowed } })
        }
        Grid2dWireOperation::DeleteRule { id } => Grid2dMutation::DeleteRule(DeleteRule { id }),
        Grid2dWireOperation::PinCell { x, y, tile_id } => Grid2dMutation::PinCell(PinCell { x, y, tile_id }),
        Grid2dWireOperation::UnpinCell { x, y } => Grid2dMutation::UnpinCell(UnpinCell { x, y }),
        Grid2dWireOperation::MaskCell { x, y } => Grid2dMutation::MaskCell(MaskCell { x, y }),
        Grid2dWireOperation::UnmaskCell { x, y } => Grid2dMutation::UnmaskCell(UnmaskCell { x, y }),
    }
}

dsl::original_tagged_operation_receiving!(Grid2dWireOperation, COMPONENT_PROTOCOL_SEMIO);
dsl::original_mirrored_operation_receiving!(Grid2dMutation, Grid2dWireOperation, receive_operation);
