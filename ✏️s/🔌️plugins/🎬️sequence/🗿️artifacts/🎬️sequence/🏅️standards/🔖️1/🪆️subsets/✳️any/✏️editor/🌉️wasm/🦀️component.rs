//! 🌉️ Sequence editor owned byte/message bridge and primitive linear-memory exports.

#[path = "🦀️protocol.rs"]
pub mod protocol;

use crate::artifacts::sequence::{SequenceFixture, SlotRef};
use crate::editor::sequence::SequenceHost;
use infinite_board_port_directed_dag::DagLayoutOptions;
use protocol::{SequenceBridge, SequenceDomain, SequenceFailure, SequencePayloadReader};
use semio_framework::abi::{AbiErrorCode, AbiMessage, AbiPort, AbiPortPoll, AbiWorkBudget, decode_abi_message, encode_abi_message};
use std::cell::RefCell;

//#region 🔖️DomainAdapter

#[derive(Default)]
struct SequenceDomainAdapter {
    host: SequenceHost,
    width: u32,
    height: u32,
    dpr: f64,
    pointer_down: (f64, f64, u8),
}

impl SequenceDomain for SequenceDomainAdapter {
    fn execute(&mut self, operation: u16, payload: &[u8]) -> Result<Vec<u8>, SequenceFailure> {
        use protocol::*;
        match operation {
            SEQUENCE_OPERATION_LOAD_FIXTURE => {
                let fixture: SequenceFixture = serde_json::from_slice(payload).map_err(domain_error)?;
                self.host.replace_snapshot(fixture).map(|_| Vec::new()).map_err(domain_error)
            }
            SEQUENCE_OPERATION_FIXTURE => {
                self.host.sync_from_dag();
                self.host.to_json().map(String::into_bytes).map_err(domain_error)
            }
            SEQUENCE_OPERATION_CATALOGUE => Ok(self.host.catalogue_json().into_bytes()),
            SEQUENCE_OPERATION_ADD_STEP => self.add_step(payload, 0),
            SEQUENCE_OPERATION_ADD_STEP_DROPPED => self.add_step(payload, 1),
            SEQUENCE_OPERATION_ADD_STEP_TO_SLOT => self.add_step(payload, 2),
            SEQUENCE_OPERATION_SET_STEP_COLLAPSED => self.set_step_collapsed(payload),
            SEQUENCE_OPERATION_PICK_STEP => self.pick_step(payload),
            SEQUENCE_OPERATION_BUILD_PATH => self.host.build_path_json().map(String::into_bytes).map_err(domain_error),
            SEQUENCE_OPERATION_REMOVE_STEP => {
                let id = one_text(payload)?;
                Ok(vec![u8::from(self.host.remove_step(&id))])
            }
            SEQUENCE_OPERATION_SET_STEP_PARAMS => {
                let (id, json) = two_texts(payload)?;
                self.host.set_step_params_json(&id, &json).map(|_| Vec::new()).map_err(domain_error)
            }
            SEQUENCE_OPERATION_CONNECT_STEPS => {
                let (from, to) = two_texts(payload)?;
                self.host.connect_steps(&from, &to).map(String::into_bytes).map_err(domain_error)
            }
            SEQUENCE_OPERATION_DISCONNECT_STEPS => {
                let (from, to) = two_texts(payload)?;
                Ok(vec![u8::from(self.host.disconnect_steps(&from, &to))])
            }
            SEQUENCE_OPERATION_COMPILE_TEXT => Ok(self.host.compile_text().into_bytes()),
            SEQUENCE_OPERATION_COMPILED_WIRE => Ok(self.host.compiled_wire_literal().into_bytes()),
            SEQUENCE_OPERATION_RUN => {
                let result = self.host.run();
                serde_json::to_vec(&result).map_err(domain_error)
            }
            SEQUENCE_OPERATION_SET_SIZE => self.set_size(payload),
            SEQUENCE_OPERATION_RENDER_FRAME => self.render_frame(),
            SEQUENCE_OPERATION_WORLD_FROM_SCREEN => self.world_from_screen(payload),
            SEQUENCE_OPERATION_POINTER_DOWN => self.pointer_down(payload),
            SEQUENCE_OPERATION_POINTER_MOVE => self.pointer_move(payload),
            SEQUENCE_OPERATION_POINTER_UP => self.pointer_up(payload),
            SEQUENCE_OPERATION_WHEEL => self.wheel(payload),
            SEQUENCE_OPERATION_REORGANIZE => {
                let options: DagLayoutOptions = serde_json::from_slice(payload).map_err(domain_error)?;
                self.host.dag.reorganize(&options).map_err(domain_error)?;
                self.host.sync_from_dag();
                self.host.layout_expanded_slots();
                Ok(Vec::new())
            }
            SEQUENCE_OPERATION_LOD_SCALE => Ok(infinite_board_port_directed_dag::board::ports::directed_dag::dag_lod_scale_json().into_bytes()),
            SEQUENCE_OPERATION_SET_AUTOMATIC_LOD => {
                self.host.dag.set_automatic_lod(one_bool(payload)?);
                Ok(Vec::new())
            }
            SEQUENCE_OPERATION_SET_FORCED_LOD => {
                self.host.dag.set_forced_draw_lod_label(&one_text(payload)?);
                Ok(Vec::new())
            }
            SEQUENCE_OPERATION_DRAW_LOD => Ok(self.host.dag.draw_lod_label().to_string().into_bytes()),
            SEQUENCE_OPERATION_SET_THEME => {
                let json = std::str::from_utf8(payload).map_err(|_| abi_failure(AbiErrorCode::InvalidUtf8))?;
                self.host.dag.set_canvas_theme_from_json(json).map_err(domain_error)?;
                Ok(Vec::new())
            }
            SEQUENCE_OPERATION_SELECTED_NODES => serde_json::to_vec(&self.host.dag.selected_node_ids()).map_err(domain_error),
            SEQUENCE_OPERATION_SET_SELECTION => self.set_selection(payload),
            SEQUENCE_OPERATION_LABEL_OVERLAY => self.host.dag.label_overlay_paint_state_json().map(String::into_bytes).map_err(domain_error),
            SEQUENCE_OPERATION_HOVERED_NODE => Ok(self.host.dag.hovered_node_id().unwrap_or_default().into_bytes()),
            SEQUENCE_OPERATION_PRESELECT_NODES => serde_json::to_vec(&serde_json::json!({
                "ids": self.host.dag.preselect_widget_ids(),
                "removedIds": self.host.dag.preselect_removed_widget_ids(),
            }))
            .map_err(domain_error),
            SEQUENCE_OPERATION_SELECTION_PREVIEW_POINTS => Ok(self.host.dag.selection_preview_points_json().into_bytes()),
            SEQUENCE_OPERATION_SELECTION_PREVIEW_CROSSING => Ok(vec![u8::from(self.host.dag.selection_preview_crossing())]),
            SEQUENCE_OPERATION_SELECTION_PREVIEW_METHOD => Ok(self.host.dag.selection_preview_method().to_string().into_bytes()),
            SEQUENCE_OPERATION_SELECTION_BOUNDS => Ok(self.host.dag.selection_union_bounds_screen_json().into_bytes()),
            SEQUENCE_OPERATION_SET_SELECTION_OPTIONS => {
                let (method, mode) = two_texts(payload)?;
                self.host.dag.set_selection_options(&method, &mode, true, false, false);
                Ok(Vec::new())
            }
            SEQUENCE_OPERATION_SET_GHOST_STEP => {
                let mut reader = SequencePayloadReader::new(payload);
                let kind = reader.text().map_err(abi_failure)?.to_owned();
                let x = reader.f64().map_err(abi_failure)?;
                let y = reader.f64().map_err(abi_failure)?;
                reader.finish().map_err(abi_failure)?;
                self.host.set_ghost_step(&kind, x, y);
                Ok(Vec::new())
            }
            SEQUENCE_OPERATION_CLEAR_GHOST_STEP => {
                self.host.clear_ghost_step();
                Ok(Vec::new())
            }
            _ => Err(SequenceFailure::new(AbiErrorCode::UnknownOperation, "unknown Sequence operation")),
        }
    }
}

impl SequenceDomainAdapter {
    fn add_step(&mut self, payload: &[u8], variant: u8) -> Result<Vec<u8>, SequenceFailure> {
        let mut reader = SequencePayloadReader::new(payload);
        let kind = reader.text().map_err(abi_failure)?.to_owned();
        let x = reader.f64().map_err(abi_failure)?;
        let y = reader.f64().map_err(abi_failure)?;
        let id = match variant {
            0 => {
                reader.finish().map_err(abi_failure)?;
                self.host.add_step(&kind, x, y)
            }
            1 => {
                let picked = reader.optional_text().map_err(abi_failure)?.map(str::to_owned);
                reader.finish().map_err(abi_failure)?;
                self.host.add_step_dropped(&kind, x, y, picked.as_deref())
            }
            2 => {
                let owner = reader.text().map_err(abi_failure)?.to_owned();
                let name = reader.text().map_err(abi_failure)?.to_owned();
                reader.finish().map_err(abi_failure)?;
                self.host.add_step_in_slot(&kind, x, y, Some(SlotRef { owner, name }))
            }
            _ => return Err(abi_failure(AbiErrorCode::MalformedTag)),
        };
        Ok(id.into_bytes())
    }

    fn set_step_collapsed(&mut self, payload: &[u8]) -> Result<Vec<u8>, SequenceFailure> {
        let mut reader = SequencePayloadReader::new(payload);
        let id = reader.text().map_err(abi_failure)?.to_owned();
        let collapsed = byte_bool(reader.u8().map_err(abi_failure)?)?;
        reader.finish().map_err(abi_failure)?;
        Ok(vec![u8::from(self.host.set_step_collapsed(&id, collapsed))])
    }

    fn pick_step(&mut self, payload: &[u8]) -> Result<Vec<u8>, SequenceFailure> {
        let (sx, sy) = point(payload)?;
        Ok(self.host.pick_step_id_at_screen(sx, sy, self.width, self.height, self.dpr).unwrap_or_default().into_bytes())
    }

    fn set_size(&mut self, payload: &[u8]) -> Result<Vec<u8>, SequenceFailure> {
        let mut reader = SequencePayloadReader::new(payload);
        self.width = reader.u32().map_err(abi_failure)?.max(1);
        self.height = reader.u32().map_err(abi_failure)?.max(1);
        self.dpr = reader.f64().map_err(abi_failure)?.max(1.0);
        reader.finish().map_err(abi_failure)?;
        self.host.dag.set_viewport(self.width, self.height, self.dpr);
        Ok(Vec::new())
    }

    fn render_frame(&mut self) -> Result<Vec<u8>, SequenceFailure> {
        let fixture = self.host.to_json().map_err(domain_error)?;
        let labels = self.host.dag.label_overlay_paint_state_json().map_err(domain_error)?;
        Ok(format!("{{\"fixture\":{fixture},\"labels\":{labels}}}").into_bytes())
    }

    fn world_from_screen(&self, payload: &[u8]) -> Result<Vec<u8>, SequenceFailure> {
        use infinite_canvas::Point;
        use infinite_canvas::camera::{Camera, Viewport, screen_to_world};
        let (sx, sy) = point(payload)?;
        let viewport = Viewport { width: self.width.max(1), height: self.height.max(1), dpr: self.dpr.max(1.0) };
        let camera = Camera { x: self.host.dag.fixture.camera.x, y: self.host.dag.fixture.camera.y, zoom: self.host.dag.fixture.camera.zoom };
        let world = screen_to_world(&camera, &viewport, Point::new(sx, sy));
        Ok(format!("{{\"x\":{},\"y\":{}}}", world.x, world.y).into_bytes())
    }

    fn pointer_down(&mut self, payload: &[u8]) -> Result<Vec<u8>, SequenceFailure> {
        let mut reader = SequencePayloadReader::new(payload);
        let sx = reader.f64().map_err(abi_failure)?;
        let sy = reader.f64().map_err(abi_failure)?;
        let button = reader.u8().map_err(abi_failure)?;
        let shift = byte_bool(reader.u8().map_err(abi_failure)?)?;
        let ctrl = byte_bool(reader.u8().map_err(abi_failure)?)?;
        let alt = byte_bool(reader.u8().map_err(abi_failure)?)?;
        reader.finish().map_err(abi_failure)?;
        self.pointer_down = (sx, sy, button);
        self.host.dag.pointer_down_screen(sx, sy, button, shift, ctrl, alt, false);
        Ok(Vec::new())
    }

    fn pointer_move(&mut self, payload: &[u8]) -> Result<Vec<u8>, SequenceFailure> {
        let (sx, sy, shift, ctrl, alt) = pointer(payload)?;
        self.host.dag.pointer_move_screen(sx, sy, shift, ctrl, alt);
        Ok(Vec::new())
    }

    fn pointer_up(&mut self, payload: &[u8]) -> Result<Vec<u8>, SequenceFailure> {
        let (sx, sy, shift, ctrl, alt) = pointer(payload)?;
        self.host.dag.pointer_up_screen(sx, sy, shift, ctrl, alt);
        self.host.sync_from_dag();
        if self.pointer_down.2 == 0 && !shift && !ctrl && !alt {
            let dx = sx - self.pointer_down.0;
            let dy = sy - self.pointer_down.1;
            if dx * dx + dy * dy <= 64.0 {
                let selected = self.host.dag.selected_node_ids();
                if let Some(id) = self.host.pick_step_id_at_screen(sx, sy, self.width, self.height, self.dpr) {
                    if selected.is_empty() || (selected.len() == 1 && !selected.contains(&id)) {
                        self.host.dag.set_selection(&[id]);
                    }
                }
            }
        }
        Ok(Vec::new())
    }

    fn wheel(&mut self, payload: &[u8]) -> Result<Vec<u8>, SequenceFailure> {
        use infinite_canvas::camera::{Camera, Viewport, wheel_screen};
        let mut reader = SequencePayloadReader::new(payload);
        let sx = reader.f64().map_err(abi_failure)?;
        let sy = reader.f64().map_err(abi_failure)?;
        let delta_y = reader.f64().map_err(abi_failure)?;
        reader.finish().map_err(abi_failure)?;
        self.host.dag.set_wheel_zoom_active(true);
        let viewport = Viewport { width: self.width.max(1), height: self.height.max(1), dpr: self.dpr.max(1.0) };
        let mut camera = Camera { x: self.host.dag.fixture.camera.x, y: self.host.dag.fixture.camera.y, zoom: self.host.dag.fixture.camera.zoom };
        wheel_screen(&mut camera, &viewport, sx, sy, delta_y);
        self.host.dag.set_camera(camera.x, camera.y, camera.zoom);
        self.host.dag.set_wheel_zoom_active(false);
        self.host.sync_from_dag();
        Ok(Vec::new())
    }

    fn set_selection(&mut self, payload: &[u8]) -> Result<Vec<u8>, SequenceFailure> {
        let mut reader = SequencePayloadReader::new(payload);
        let count = reader.u32().map_err(abi_failure)? as usize;
        let mut selected = Vec::with_capacity(count);
        for _ in 0..count {
            selected.push(reader.text().map_err(abi_failure)?.to_owned());
        }
        reader.finish().map_err(abi_failure)?;
        self.host.dag.set_selection(&selected);
        Ok(Vec::new())
    }
}

fn domain_error(error: impl std::fmt::Display) -> SequenceFailure {
    SequenceFailure::new(AbiErrorCode::MalformedTag, error.to_string())
}

fn abi_failure(code: AbiErrorCode) -> SequenceFailure {
    SequenceFailure::new(code, code.to_string())
}

fn byte_bool(value: u8) -> Result<bool, SequenceFailure> {
    match value {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(abi_failure(AbiErrorCode::MalformedTag)),
    }
}

fn one_bool(payload: &[u8]) -> Result<bool, SequenceFailure> {
    if payload.len() != 1 {
        return Err(abi_failure(AbiErrorCode::MalformedLength));
    }
    byte_bool(payload[0])
}

fn one_text(payload: &[u8]) -> Result<String, SequenceFailure> {
    let mut reader = SequencePayloadReader::new(payload);
    let value = reader.text().map_err(abi_failure)?.to_owned();
    reader.finish().map_err(abi_failure)?;
    Ok(value)
}

fn two_texts(payload: &[u8]) -> Result<(String, String), SequenceFailure> {
    let mut reader = SequencePayloadReader::new(payload);
    let value = (reader.text().map_err(abi_failure)?.to_owned(), reader.text().map_err(abi_failure)?.to_owned());
    reader.finish().map_err(abi_failure)?;
    Ok(value)
}

fn point(payload: &[u8]) -> Result<(f64, f64), SequenceFailure> {
    let mut reader = SequencePayloadReader::new(payload);
    let value = (reader.f64().map_err(abi_failure)?, reader.f64().map_err(abi_failure)?);
    reader.finish().map_err(abi_failure)?;
    Ok(value)
}

fn pointer(payload: &[u8]) -> Result<(f64, f64, bool, bool, bool), SequenceFailure> {
    let mut reader = SequencePayloadReader::new(payload);
    let value = (reader.f64().map_err(abi_failure)?, reader.f64().map_err(abi_failure)?, byte_bool(reader.u8().map_err(abi_failure)?)?, byte_bool(reader.u8().map_err(abi_failure)?)?, byte_bool(reader.u8().map_err(abi_failure)?)?);
    reader.finish().map_err(abi_failure)?;
    Ok(value)
}

//#endregion 🔖️DomainAdapter

//#region 🌉️LinearMemory

struct RetainedMessage {
    _message: AbiMessage,
    bytes: Vec<u8>,
}

thread_local! {
    static BRIDGE: RefCell<SequenceBridge<SequenceDomainAdapter>> = RefCell::new(SequenceBridge::new(SequenceDomainAdapter::default));
    static RETAINED: RefCell<Option<RetainedMessage>> = const { RefCell::new(None) };
}

#[unsafe(no_mangle)]
pub extern "C" fn sequence_bridge_allocate(length: usize) -> *mut u8 {
    if length == 0 {
        return std::ptr::null_mut();
    }
    let mut bytes = Vec::<u8>::with_capacity(length);
    let pointer = bytes.as_mut_ptr();
    std::mem::forget(bytes);
    pointer
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sequence_bridge_release(pointer: *mut u8, capacity: usize) {
    if !pointer.is_null() && capacity != 0 {
        drop(unsafe { Vec::from_raw_parts(pointer, 0, capacity) });
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sequence_bridge_send(pointer: *const u8, length: usize, byte_credit: usize) -> i32 {
    if pointer.is_null() || length == 0 || length > protocol::SEQUENCE_MAX_REQUEST_BYTES + 32 {
        return -1;
    }
    let bytes = unsafe { std::slice::from_raw_parts(pointer, length) };
    let Ok(message) = decode_abi_message(bytes) else {
        return -1;
    };
    BRIDGE.with(|bridge| bridge.borrow_mut().try_send(message, AbiWorkBudget::credits(byte_credit)).map(|_| 1).unwrap_or(-1))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sequence_bridge_poll(pointer: *mut u8, capacity: usize, byte_credit: usize) -> i32 {
    if pointer.is_null() || capacity == 0 {
        return -1;
    }
    RETAINED.with(|retained| {
        if retained.borrow().is_none() {
            match BRIDGE.with(|bridge| bridge.borrow_mut().poll(AbiWorkBudget::credits(byte_credit))) {
                Ok(AbiPortPoll::Message(message)) => {
                    let bytes = encode_abi_message(&message);
                    *retained.borrow_mut() = Some(RetainedMessage { _message: message, bytes });
                }
                Ok(AbiPortPoll::Pending) => return 0,
                Ok(AbiPortPoll::Closed) | Err(_) => return -1,
            }
        }
        let mut retained = retained.borrow_mut();
        let value = retained.as_ref().expect("retained Sequence message");
        if value.bytes.len() > capacity {
            return i32::try_from(value.bytes.len()).unwrap_or(-1);
        }
        unsafe { std::ptr::copy_nonoverlapping(value.bytes.as_ptr(), pointer, value.bytes.len()) };
        let length = value.bytes.len();
        retained.take();
        i32::try_from(length).unwrap_or(-1)
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn sequence_bridge_begin_close() {
    RETAINED.with(|retained| retained.borrow_mut().take());
    BRIDGE.with(|bridge| bridge.borrow_mut().begin_close());
}

#[unsafe(no_mangle)]
pub extern "C" fn sequence_bridge_terminal_is_empty() -> i32 {
    let retained_empty = RETAINED.with(|retained| retained.borrow().is_none());
    i32::from(retained_empty && BRIDGE.with(|bridge| bridge.borrow().terminal_is_empty()))
}

//#endregion 🌉️LinearMemory
