//! 🧰️ Authoring kit of the puzzle 5d editor: the retained add-node, brush, board-event, fastener, active-example and mesh work plus the one-item store preparations. See [`super`].

use super::*;

//#region 🔖️PlayApp
//#region 🧵️RetainedCommands
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle5dAddNodeStage {
    Catalog,
    Grips,
    Complete,
    Closing,
}

/// ♻️ The owners one `Puzzle5dAddNodeWork` still holds when its job closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
struct Puzzle5dAddNodeWorkOwners {
    pub(super) mutation: Option<Puzzle5dMutation>,
    pub(super) grips: Vec<crate::Puzzle5dGrip>,
    pub(super) mesh_url: Option<String>,
}

pub(super) struct Puzzle5dAddNodeWork {
    pub(super) stage: Puzzle5dAddNodeStage,
    catalog_cursor: usize,
    pub(super) grip_cursor: usize,
    catalog_index: Option<usize>,
    pub(super) mesh_url: Option<String>,
    pub(super) grips: Vec<crate::Puzzle5dGrip>,
    pub(super) mutation: Option<Puzzle5dMutation>,
    pub(super) operation_nonce: u64,
    pub(super) close_owners: crate::puzzle_job::WorkClosing<Puzzle5dAddNodeWorkOwners>,
}

impl Default for Puzzle5dAddNodeWork {
    fn default() -> Self {
        Self { close_owners: Default::default(), stage: Puzzle5dAddNodeStage::Catalog, catalog_cursor: 0, grip_cursor: 0, catalog_index: None, mesh_url: None, grips: Vec::with_capacity(crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS), mutation: None, operation_nonce: 0 }
    }
}

impl Puzzle5dAddNodeWork {
    pub(super) fn part_kind(command: &Puzzle5dCommand) -> &str {
        command.args().and_then(|args| args.get("kind")).and_then(Value::as_str).unwrap_or("Part")
    }

    pub(super) fn catalogs(snapshot: &Puzzle5dPlaySnapshot) -> Vec<Value> {
        let projection = puzzle5d_editor_projection(snapshot);
        projection.get("kindCatalogs").and_then(|catalogs| catalogs.get("parts")).and_then(Value::as_array).cloned().unwrap_or_default()
    }
}

impl Puzzle5dAddNodeWork {
    /// 🪪️ Binds the operation nonce its minted ids derive from.
    pub(super) fn bound(mut self, operation_nonce: u64) -> Self {
        self.operation_nonce = operation_nonce;
        self
    }
}

impl semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<Puzzle5dPlayApp>> for Puzzle5dAddNodeWork {
    fn tool_id(&self) -> &'static str {
        "addNode"
    }

    fn extent(&self, _command: &Puzzle5dCommand, snapshot: &Puzzle5dPlaySnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Puzzle5dPlayApp>>>) -> Option<usize> {
        let items = Self::catalogs(snapshot).len().checked_add(crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS)?;
        (items <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS).then_some(items)
    }

    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Puzzle5dPlayApp>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        crate::retained_command::step_demands::<EditorApp<Puzzle5dPlayApp>>(size_of::<Self>())
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Puzzle5dPlayApp>>, _cx: &mut StepContext<'_>) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<Puzzle5dPlayApp>>, Fault> {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command, snapshot, config: _config, interaction: _interaction, hover: _hover, context: _context, .. } = *input;
        let projection = puzzle5d_editor_projection(snapshot);
        let catalogs = Self::catalogs(snapshot);
        match self.stage {
            Puzzle5dAddNodeStage::Catalog => {
                let Some(entry) = catalogs.get(self.catalog_cursor) else {
                    self.stage = Puzzle5dAddNodeStage::Grips;
                    return Ok(crate::puzzle_progress_step!("puzzle5d-add-node-grip", "Reading grip template", "Griffvorlage wird gelesen"));
                };
                let index = self.catalog_cursor;
                self.catalog_cursor += 1;
                if entry.get("id").and_then(Value::as_str) == Some(Self::part_kind(command)) {
                    self.catalog_index = Some(index);
                    self.mesh_url = entry.get("meshUrl").and_then(Value::as_str).filter(|url| !url.is_empty()).map(str::to_string);
                    self.stage = Puzzle5dAddNodeStage::Grips;
                }
                Ok(crate::puzzle_progress_step!("puzzle5d-add-node-catalog", "Reading part catalog", "Teilekatalog wird gelesen"))
            }
            Puzzle5dAddNodeStage::Grips => {
                let templates = self.catalog_index.and_then(|index| catalogs.get(index)).and_then(|entry| entry.get("grips")).and_then(Value::as_array).map(Vec::as_slice).unwrap_or_default();
                if let Some(template) = templates.get(self.grip_cursor) {
                    if self.grips.len() >= crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS {
                        return Err(Fault::from("puzzle5d-add-node-grip-capacity"));
                    }
                    let grip_kind = template.get("gripKind").and_then(Value::as_str).unwrap_or("grip").to_string();
                    let grip_2d: crate::Puzzle5dGrip2d = match template.get("2d") {
                        Some(value) => <crate::Puzzle5dGrip2d as FromValue>::from_value(semio_framework_pack_json::to_dsl_value(value)).map_err(|_| Fault::from("puzzle5d-add-node-grip2d-malformed"))?,
                        None => Default::default(),
                    };
                    let grip_3d: crate::Puzzle5dGrip3d = match template.get("3d") {
                        Some(value) => <crate::Puzzle5dGrip3d as FromValue>::from_value(semio_framework_pack_json::to_dsl_value(value)).map_err(|_| Fault::from("puzzle5d-add-node-grip3d-malformed"))?,
                        None => Default::default(),
                    };
                    self.grips.push(crate::Puzzle5dGrip { id: format!("v{}", self.grip_cursor), grip_kind: Some(grip_kind), grip_2d, grip_3d });
                    self.grip_cursor += 1;
                    return Ok(crate::puzzle_progress_step!("puzzle5d-add-node-grip", "Reading grip template", "Griffvorlage wird gelesen"));
                }
                let x = command.args().and_then(|args| args.get("x")).and_then(Value::as_f64).unwrap_or(120.0);
                let y = command.args().and_then(|args| args.get("y")).and_then(Value::as_f64).unwrap_or(120.0);
                let flat_to_world = PUZZLE5D_FLAT_TO_WORLD;
                let origin = projection.get("parts").and_then(Value::as_array).and_then(|parts| parts.first()).map_or([x * flat_to_world, -y * flat_to_world, 0.0], |peer| {
                    let peer_2d = peer.get("2d");
                    let peer_3d = peer.get("3d");
                    let peer_x = peer_2d.and_then(|part| part.get("x")).and_then(Value::as_f64).unwrap_or_default();
                    let peer_y = peer_2d.and_then(|part| part.get("y")).and_then(Value::as_f64).unwrap_or_default();
                    let peer_origin = peer_3d.and_then(|part| part.get("origin")).and_then(puzzle5d_value_as_f64_3).unwrap_or_default();
                    [peer_origin[0] + (x - peer_x) * flat_to_world, peer_origin[1] - (y - peer_y) * flat_to_world, peer_origin[2]]
                });
                let part_kind = Self::part_kind(command).to_string();
                let created_id = format!("part-{:016x}-0", self.operation_nonce);
                let part = crate::Puzzle5dPart {
                    id: created_id.clone(),
                    part_kind: Some(part_kind.clone()),
                    anchor: Default::default(),
                    part_2d: crate::Puzzle5dPart2d { x, y, shape: Some("circle".to_string()), radius: Some(PUZZLE5D_DEFAULT_PART_RADIUS), text: Some(part_kind.clone()), ..Default::default() },
                    part_3d: crate::Puzzle5dPart3d { origin, mesh_url: self.mesh_url.take(), orientation: Some([0.0, 0.0, 0.0, 1.0]), label: Some(puzzle5d_next_part_label_from_projection(&projection, &part_kind)), ..Default::default() },
                    grips: std::mem::take(&mut self.grips),
                };
                self.mutation = Some(crate::standards::v1::subsets::any::schema::mutations::create_part(part, None));
                self.stage = Puzzle5dAddNodeStage::Complete;
                // 🕹️ Re-select what this gesture just created — a palette drop that leaves the old selection
                // standing makes every follow-up verb (patch, transform, delete) act on the wrong part.
                Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit {
                    artifact_mutations: self.mutation.take().into_iter().collect(),
                    interaction_writes: vec![InteractionWrite::replace(PUZZLE5D_INTERACTION_DOMAIN, PUZZLE5D_GRANULARITY_PART, [created_id])],
                    ui_scope: UiDirtyScope::Full,
                    ..Default::default()
                }))
            }
            Puzzle5dAddNodeStage::Complete => Err(Fault::from("puzzle5d-add-node-complete-repolled")),
            Puzzle5dAddNodeStage::Closing => Err(Fault::from("puzzle5d-add-node-closing")),
        }
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle5dAddNodeStage::Closing;
        self.close_owners.stage(Puzzle5dAddNodeWorkOwners { mutation: std::mem::take(&mut self.mutation), grips: std::mem::take(&mut self.grips), mesh_url: std::mem::take(&mut self.mesh_url) });
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
        self.close_owners.close_step(grant)
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.stage == Puzzle5dAddNodeStage::Closing && self.close_owners.is_empty()
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle5dAddBrushPartStage {
    Catalog,
    Grips,
    Target,
    Create,
    Connect,
    Complete,
    Closing,
}

/// ♻️ The owners one `Puzzle5dAddBrushPartWork` still holds when its job closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
struct Puzzle5dAddBrushPartWorkOwners {
    pub(super) mutations: Vec<Puzzle5dMutation>,
    pub(super) payload: Option<Value>,
    pub(super) grips: Vec<crate::Puzzle5dGrip>,
    pub(super) mesh_url: Option<String>,
    target_id: Option<String>,
    target_position: Option<[f64; 3]>,
    target_direction: Option<[f64; 3]>,
    created_id: Option<String>,
    created_grip_id: Option<String>,
}

pub(super) struct Puzzle5dAddBrushPartWork {
    pub(super) tool_id: &'static str,
    pub(super) payload: Option<Value>,
    pub(super) stage: Puzzle5dAddBrushPartStage,
    catalog_cursor: usize,
    pub(super) grip_cursor: usize,
    pub(super) part_cursor: usize,
    target_grip_cursor: usize,
    pub(super) processed_units: usize,
    catalog_index: Option<usize>,
    pub(super) mesh_url: Option<String>,
    target_id: Option<String>,
    target_position: Option<[f64; 3]>,
    target_direction: Option<[f64; 3]>,
    created_id: Option<String>,
    created_grip_id: Option<String>,
    pub(super) grips: Vec<crate::Puzzle5dGrip>,
    pub(super) mutations: Vec<Puzzle5dMutation>,
    pub(super) operation_nonce: u64,
    pub(super) fresh_cursor: u64,
    pub(super) close_owners: crate::puzzle_job::WorkClosing<Puzzle5dAddBrushPartWorkOwners>,
}

impl Puzzle5dAddBrushPartWork {
    fn is_closing(&self) -> bool {
        self.stage == Puzzle5dAddBrushPartStage::Closing
    }

    pub(super) fn new(tool_id: &'static str) -> Self {
        Self { close_owners: Default::default(),
            tool_id,
            payload: None,
            stage: Puzzle5dAddBrushPartStage::Catalog,
            catalog_cursor: 0,
            grip_cursor: 0,
            part_cursor: 0,
            target_grip_cursor: 0,
            processed_units: 0,
            catalog_index: None,
            mesh_url: None,
            target_id: None,
            target_position: None,
            target_direction: None,
            created_id: None,
            created_grip_id: None,
            grips: Vec::with_capacity(crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS),
            mutations: Vec::with_capacity(2),
            operation_nonce: 0,
            fresh_cursor: 0,
        }
    }

    fn from_board_payload(payload: Value) -> Self {
        let mut work = Self::new("addBrushPart");
        work.payload = Some(payload);
        work
    }

    /// 🌉️ `self.payload` (materialized once via `take_payload`/`parse`) and `command.args()` are
    /// both this file's own first-party `Value` now, so no bridge is needed — an in-progress board
    /// payload wins, falling back to the dispatch args only when there is none.
    pub(super) fn args(&self, command: &Puzzle5dCommand) -> Option<Value> {
        self.payload.clone().or_else(|| command.args().cloned())
    }

    /// 🗂️ The kind this run adds: the caller's own `partKind`, else THIS tool's own declared select
    /// default — never a literal.
    ///
    /// 🐛️ It used to fall back to the string `"Part"`, the very literal
    /// `PUZZLE5D_SHIPPED_PART_KINDS` replaced in the select, so a bare `addPartKind` (the arg form
    /// dispatched with no args, which is what an agent sends) added a part of a kind no catalog
    /// declares. Measured 2026-09-22 (slice PZ2): `add_part_kind_materializes_the_declared_kind_default`
    /// read back `"Part"` where the declared default is `"Hexagonal Cut Concrete Forest Left"`.
    /// `addBrushPart` declares a one-row `"Part"` select of its own, so reading each tool's OWN
    /// declared default keeps that verb byte-identical while fixing this one.
    fn owned_part_kind(&self, command: &Puzzle5dCommand) -> String {
        self.args(command)
            .and_then(|args| args.get("partKind").or_else(|| args.get("objectKindId")).or_else(|| args.get("nodeKind")).and_then(Value::as_str).map(str::to_string))
            .filter(|kind| !kind.is_empty())
            .unwrap_or_else(|| if self.tool_id == "addPartKind" { puzzle5d_default_part_kind(&puzzle5d_part_kind_options()) } else { "Part".to_string() })
    }

    pub(super) fn catalogs(snapshot: &Puzzle5dPlaySnapshot) -> Vec<Value> {
        Puzzle5dAddNodeWork::catalogs(snapshot)
    }

    pub(super) fn target(&self, command: &Puzzle5dCommand, interaction: &protocol::InteractionState) -> Option<String> {
        self.args(command)
            .and_then(|args| args.get("targetVortexFullId").or_else(|| args.get("targetGripFullId")).and_then(Value::as_str).map(str::to_string))
            .filter(|id| !id.is_empty())
            .or_else(|| interaction.selection.get(PUZZLE5D_INTERACTION_DOMAIN).filter(|selection| selection.granularity == PUZZLE5D_GRANULARITY_GRIP).and_then(|selection| selection.ids.first().cloned()))
    }

    fn world_direction(part: &Value, grip: &Value) -> [f64; 3] {
        let orientation = part.get("3d").and_then(|part| part.get("orientation")).and_then(puzzle5d_value_as_f64_4).unwrap_or([0.0, 0.0, 0.0, 1.0]);
        let direction = grip.get("3d").and_then(|grip| grip.get("direction")).and_then(puzzle5d_value_as_f64_3).unwrap_or([0.0, 0.0, -1.0]);
        quat_rotate_vector(orientation, direction)
    }
}

impl Puzzle5dAddBrushPartWork {
    /// 🪪️ Binds the operation nonce its minted ids derive from.
    pub(super) fn bound(mut self, operation_nonce: u64) -> Self {
        self.operation_nonce = operation_nonce;
        self
    }
}

impl semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<Puzzle5dPlayApp>> for Puzzle5dAddBrushPartWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn extent(&self, _command: &Puzzle5dCommand, snapshot: &Puzzle5dPlaySnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Puzzle5dPlayApp>>>) -> Option<usize> {
        let projection = puzzle5d_editor_projection(snapshot);
        let items = Self::catalogs(snapshot).len().checked_add(crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS)?.checked_add(projection.get("parts").and_then(Value::as_array).map_or(0, Vec::len))?.checked_add(2)?;
        (items <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS).then_some(items)
    }

    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Puzzle5dPlayApp>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        crate::retained_command::step_demands::<EditorApp<Puzzle5dPlayApp>>(size_of::<Self>())
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Puzzle5dPlayApp>>, _cx: &mut StepContext<'_>) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<Puzzle5dPlayApp>>, Fault> {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command, snapshot, config: _config, interaction, hover: _hover, context: _context, .. } = *input;
        let projection = puzzle5d_editor_projection(snapshot);
        if self.processed_units >= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS {
            return Err(Fault::from("puzzle5d-add-brush-part-work-capacity"));
        }
        self.processed_units += 1;
        let catalogs = Self::catalogs(snapshot);
        match self.stage {
            Puzzle5dAddBrushPartStage::Catalog => {
                let Some(entry) = catalogs.get(self.catalog_cursor) else {
                    self.stage = Puzzle5dAddBrushPartStage::Grips;
                    return Ok(crate::puzzle_progress_step!("puzzle5d-brush-grip", "Reading grip template", "Griffvorlage wird gelesen"));
                };
                let index = self.catalog_cursor;
                self.catalog_cursor += 1;
                if entry.get("id").and_then(Value::as_str) == Some(self.owned_part_kind(command).as_str()) {
                    self.catalog_index = Some(index);
                    self.mesh_url = entry.get("meshUrl").and_then(Value::as_str).filter(|url| !url.is_empty()).map(str::to_string);
                    self.stage = Puzzle5dAddBrushPartStage::Grips;
                }
                Ok(crate::puzzle_progress_step!("puzzle5d-brush-catalog", "Reading part catalog", "Teilekatalog wird gelesen"))
            }
            Puzzle5dAddBrushPartStage::Grips => {
                let templates = self.catalog_index.and_then(|index| catalogs.get(index)).and_then(|entry| entry.get("grips")).and_then(Value::as_array).map(Vec::as_slice).unwrap_or_default();
                if let Some(template) = templates.get(self.grip_cursor) {
                    if self.grips.len() >= crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS {
                        return Err(Fault::from("puzzle5d-add-brush-part-grip-capacity"));
                    }
                    let grip_kind = template.get("gripKind").and_then(Value::as_str).unwrap_or("grip").to_string();
                    let grip_2d = template.get("2d").map(|value| <crate::Puzzle5dGrip2d as FromValue>::from_value(semio_framework_pack_json::to_dsl_value(value))).transpose().map_err(|_| Fault::from("puzzle5d-add-brush-part-grip2d-malformed"))?.unwrap_or_default();
                    let grip_3d = template.get("3d").map(|value| <crate::Puzzle5dGrip3d as FromValue>::from_value(semio_framework_pack_json::to_dsl_value(value))).transpose().map_err(|_| Fault::from("puzzle5d-add-brush-part-grip3d-malformed"))?.unwrap_or_default();
                    let id = format!("v{}", self.grip_cursor);
                    if self.created_grip_id.is_none() {
                        self.created_grip_id = Some(id.clone());
                    }
                    self.grips.push(crate::Puzzle5dGrip { id, grip_kind: Some(grip_kind), grip_2d, grip_3d });
                    self.grip_cursor += 1;
                    return Ok(crate::puzzle_progress_step!("puzzle5d-brush-grip", "Reading grip template", "Griffvorlage wird gelesen"));
                }
                self.target_id = self.target(command, interaction);
                self.stage = if self.target_id.is_some() { Puzzle5dAddBrushPartStage::Target } else { Puzzle5dAddBrushPartStage::Create };
                Ok(crate::puzzle_progress_step!("puzzle5d-brush-target", "Finding target grip", "Zielgriff wird gesucht"))
            }
            Puzzle5dAddBrushPartStage::Target => {
                let Some(part) = projection.get("parts").and_then(Value::as_array).and_then(|parts| parts.get(self.part_cursor)) else {
                    self.target_id = None;
                    self.stage = Puzzle5dAddBrushPartStage::Create;
                    return Ok(crate::puzzle_progress_step!("puzzle5d-brush-create", "Creating brush part", "Pinselteil wird erstellt"));
                };
                let Some(grip) = part.get("grips").and_then(Value::as_array).and_then(|grips| grips.get(self.target_grip_cursor)) else {
                    self.part_cursor += 1;
                    self.target_grip_cursor = 0;
                    return Ok(crate::puzzle_progress_step!("puzzle5d-brush-target", "Finding target grip", "Zielgriff wird gesucht"));
                };
                self.target_grip_cursor += 1;
                let Some(part_id) = part.get("id").and_then(Value::as_str) else { return Ok(crate::puzzle_progress_step!("puzzle5d-brush-target", "Finding target grip", "Zielgriff wird gesucht")) };
                let Some(grip_id) = grip.get("id").and_then(Value::as_str) else { return Ok(crate::puzzle_progress_step!("puzzle5d-brush-target", "Finding target grip", "Zielgriff wird gesucht")) };
                if self.target_id.as_deref() == Some(puzzle5d_grip_full_id(part_id, grip_id).as_str()) {
                    self.target_position = Some(Puzzle5dProximityConnectWork::world_position(part, grip));
                    self.target_direction = Some(Self::world_direction(part, grip));
                    self.stage = Puzzle5dAddBrushPartStage::Create;
                }
                Ok(crate::puzzle_progress_step!("puzzle5d-brush-target", "Finding target grip", "Zielgriff wird gesucht"))
            }
            Puzzle5dAddBrushPartStage::Create => {
                let x = self.args(command).and_then(|args| args.get("x").and_then(Value::as_f64)).unwrap_or(120.0);
                let y = self.args(command).and_then(|args| args.get("y").and_then(Value::as_f64)).unwrap_or(120.0);
                let origin = match (self.target_position, self.target_direction) {
                    (Some(position), Some(direction)) => [position[0] + direction[0], position[1] + direction[1], position[2] + direction[2]],
                    _ => [x * PUZZLE5D_FLAT_TO_WORLD, -y * PUZZLE5D_FLAT_TO_WORLD, 0.0],
                };
                let part_kind = self.owned_part_kind(command);
                let id = self.args(command).and_then(|args| args.get("nodeId").or_else(|| args.get("partId")).or_else(|| args.get("objectId")).and_then(Value::as_str).map(str::to_string)).filter(|id| !id.is_empty()).unwrap_or_else(|| {
                    let id = format!("part-{:016x}-{}", self.operation_nonce, self.fresh_cursor);
                    self.fresh_cursor = self.fresh_cursor.saturating_add(1);
                    id
                });
                let part = crate::Puzzle5dPart {
                    id: id.clone(),
                    part_kind: Some(part_kind.clone()),
                    anchor: Default::default(),
                    part_2d: crate::Puzzle5dPart2d { x, y, shape: Some("circle".to_string()), radius: Some(PUZZLE5D_DEFAULT_PART_RADIUS), text: Some(part_kind.clone()), ..Default::default() },
                    part_3d: crate::Puzzle5dPart3d { origin, mesh_url: self.mesh_url.take(), orientation: Some([0.0, 0.0, 0.0, 1.0]), label: Some(puzzle5d_next_part_label_from_projection(&projection, &part_kind)), ..Default::default() },
                    grips: std::mem::take(&mut self.grips),
                };
                self.created_id = Some(id);
                self.mutations.push(crate::standards::v1::subsets::any::schema::mutations::create_part(part, None));
                self.stage = Puzzle5dAddBrushPartStage::Connect;
                Ok(crate::puzzle_progress_step!("puzzle5d-brush-create", "Creating brush part", "Pinselteil wird erstellt"))
            }
            Puzzle5dAddBrushPartStage::Connect => {
                if let (Some(source), Some(part), Some(grip)) = (self.target_id.as_ref(), self.created_id.as_ref(), self.created_grip_id.as_ref()) {
                    let id = self.args(command).and_then(|args| args.get("edgeId").and_then(Value::as_str).map(str::to_string)).filter(|id| !id.is_empty()).unwrap_or_else(|| {
                        let id = format!("fastener-{:016x}-{}", self.operation_nonce, self.fresh_cursor);
                        self.fresh_cursor = self.fresh_cursor.saturating_add(1);
                        id
                    });
                    self.mutations.push(crate::standards::v1::subsets::any::schema::mutations::connect_grips(id, source.clone(), puzzle5d_grip_full_id(part, grip), None, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, None));
                }
                self.stage = Puzzle5dAddBrushPartStage::Complete;
                // 🕹️ Re-select the placed part so the brush's next gesture chains off it, exactly as puzzle
                // 3d's `addBrushObject` does.
                let interaction_writes = self.created_id.take().map(|id| InteractionWrite::replace(PUZZLE5D_INTERACTION_DOMAIN, PUZZLE5D_GRANULARITY_PART, [id])).into_iter().collect();
                Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit { artifact_mutations: std::mem::take(&mut self.mutations), interaction_writes, ui_scope: UiDirtyScope::Full, ..Default::default() }))
            }
            Puzzle5dAddBrushPartStage::Complete => Err(Fault::from("puzzle5d-add-brush-part-complete-repolled")),
            Puzzle5dAddBrushPartStage::Closing => Err(Fault::from("puzzle5d-add-brush-part-closing")),
        }
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle5dAddBrushPartStage::Closing;
        self.close_owners.stage(Puzzle5dAddBrushPartWorkOwners { mutations: std::mem::take(&mut self.mutations), payload: std::mem::take(&mut self.payload), grips: std::mem::take(&mut self.grips), mesh_url: std::mem::take(&mut self.mesh_url), target_id: std::mem::take(&mut self.target_id), target_position: std::mem::take(&mut self.target_position), target_direction: std::mem::take(&mut self.target_direction), created_id: std::mem::take(&mut self.created_id), created_grip_id: std::mem::take(&mut self.created_grip_id) });
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
        self.close_owners.close_step(grant)
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.stage == Puzzle5dAddBrushPartStage::Closing && self.close_owners.is_empty()
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle5dBoardEventsStage {
    Open,
    Scan,
    Decode,
    Dispatch,
    ScanEdge,
    ScanDeleteEdges,
    Brush,
    DrainBrush,
    CloseBrush,
    Complete,
    Closing,
}

/// ♻️ The owners one `Puzzle5dBoardEventsWork` still holds when its job closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
struct Puzzle5dBoardEventsWorkOwners {
    pub(super) mutations: Vec<Puzzle5dMutation>,
    brush_first: Option<Puzzle5dMutation>,
    brush_second: Option<Puzzle5dMutation>,
    event: Option<Value>,
    drags: Vec<world3d::utilities::transform::Puzzle5dSelectionRecord>,
    drag_at: Option<usize>,
    pending_source: Option<String>,
    pending_target: Option<String>,
    pending_edge_id: Option<String>,
    pending_edge_kind: Option<String>,
    pending_delete_id: Option<String>,
    pub(super) camera2d: Option<Puzzle5dCamera2d>,
    select_ids: Option<Vec<String>>,
    removed_ids: Vec<String>,
    placed_id: Option<String>,
}

pub(super) struct Puzzle5dBoardEventsWork {
    pub(super) stage: Puzzle5dBoardEventsStage,
    byte_cursor: usize,
    event_start: Option<usize>,
    event_end: usize,
    pub(super) depth: usize,
    in_string: bool,
    escape: bool,
    event: Option<Value>,
    /// 🎬️ Every `drag` gesture record of the batch, each one board drag the transform tool yields — the whole
    /// batch's drags are ONE tool transaction.
    drags: Vec<world3d::utilities::transform::Puzzle5dSelectionRecord>,
    /// 📍️ Where the first drag sits among the batch's mutations — the tool's leaves publish there, in event order.
    drag_at: Option<usize>,
    /// 🌱️ The admission's authoring seed the drags' tool transaction is minted from.
    pub(super) authoring_seed: String,
    pending_source: Option<String>,
    pending_target: Option<String>,
    pending_edge_id: Option<String>,
    pending_edge_kind: Option<String>,
    pub(super) fastener_cursor: usize,
    pending_delete_id: Option<String>,
    pub(super) brush: Option<Puzzle5dAddBrushPartWork>,
    brush_first: Option<Puzzle5dMutation>,
    brush_second: Option<Puzzle5dMutation>,
    pub(super) camera2d: Option<Puzzle5dCamera2d>,
    pub(super) mutations: Vec<Puzzle5dMutation>,
    pub(super) operation_nonce: u64,
    pub(super) fresh_cursor: u64,
    /// 🕹️ The LAST `select` row of the batch is the engine's whole selection set (replace semantics), so
    /// later rows supersede earlier ones inside one batch.
    select_ids: Option<Vec<String>>,
    /// 🧹️ Ids this batch removed from the document — a delete must not leave a dangling selection, so each
    /// one is subtracted from the live selection through the sanctioned reducer channel.
    removed_ids: Vec<String>,
    /// 🖌️ The part a `brushPlace` row created, re-selected so the next brush gesture chains off it.
    placed_id: Option<String>,
    locked_refused: bool,
    pub(super) close_owners: crate::puzzle_job::WorkClosing<Puzzle5dBoardEventsWorkOwners>,
}

impl Puzzle5dBoardEventsWork {
    pub(super) fn new(authoring_seed: String) -> Self {
        Self { close_owners: Default::default(),
            stage: Puzzle5dBoardEventsStage::Open,
            byte_cursor: 0,
            event_start: None,
            event_end: 0,
            depth: 0,
            in_string: false,
            escape: false,
            event: None,
            drags: Vec::new(),
            drag_at: None,
            authoring_seed,
            pending_source: None,
            pending_target: None,
            pending_edge_id: None,
            pending_edge_kind: None,
            fastener_cursor: 0,
            pending_delete_id: None,
            brush: None,
            brush_first: None,
            brush_second: None,
            camera2d: None,
            mutations: Vec::with_capacity(crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS),
            operation_nonce: 0,
            fresh_cursor: 0,
            select_ids: None,
            removed_ids: Vec::new(),
            placed_id: None,
            locked_refused: false,
        }
    }
}

impl Puzzle5dBoardEventsWork {
    pub(super) fn source(command: &Puzzle5dCommand) -> Result<&str, Fault> {
        command.args().and_then(|args| args.get("eventsJson")).and_then(Value::as_str).ok_or_else(|| Fault::from("puzzle5d-board-events-input-missing"))
    }

    pub(super) fn push(&mut self, mutation: Puzzle5dMutation) -> Result<(), Fault> {
        if self.mutations.len() >= crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS {
            return Err(Fault::from("puzzle5d-board-events-mutation-capacity"));
        }
        self.mutations.push(mutation);
        Ok(())
    }

    fn next_event(&mut self) {
        self.event = None;
        self.event_start = None;
        self.event_end = 0;
        self.depth = 0;
        self.in_string = false;
        self.escape = false;
        self.stage = Puzzle5dBoardEventsStage::Scan;
    }

    fn scan_one(&mut self, source: &str) -> Result<(), Fault> {
        let bytes = source.as_bytes();
        let Some(byte) = bytes.get(self.byte_cursor).copied() else {
            return Err(Fault::from("puzzle5d-board-events-array-unterminated"));
        };
        match self.stage {
            Puzzle5dBoardEventsStage::Open => {
                self.byte_cursor += 1;
                if byte.is_ascii_whitespace() {
                    return Ok(());
                }
                if byte != b'[' {
                    return Err(Fault::from("puzzle5d-board-events-array-malformed"));
                }
                self.stage = Puzzle5dBoardEventsStage::Scan;
            }
            Puzzle5dBoardEventsStage::Scan if self.event_start.is_none() => {
                self.byte_cursor += 1;
                if byte.is_ascii_whitespace() || byte == b',' {
                    return Ok(());
                }
                if byte == b']' {
                    self.stage = Puzzle5dBoardEventsStage::Complete;
                    return Ok(());
                }
                if byte != b'{' {
                    return Err(Fault::from("puzzle5d-board-events-event-malformed"));
                }
                self.event_start = Some(self.byte_cursor - 1);
                self.depth = 1;
            }
            Puzzle5dBoardEventsStage::Scan => {
                self.byte_cursor += 1;
                if self.in_string {
                    if self.escape {
                        self.escape = false;
                    } else if byte == b'\\' {
                        self.escape = true;
                    } else if byte == b'"' {
                        self.in_string = false;
                    }
                    return Ok(());
                }
                if byte == b'"' {
                    self.in_string = true;
                } else if byte == b'{' || byte == b'[' {
                    self.depth = self.depth.checked_add(1).ok_or_else(|| Fault::from("puzzle5d-board-events-depth-capacity"))?;
                } else if byte == b'}' || byte == b']' {
                    self.depth = self.depth.checked_sub(1).ok_or_else(|| Fault::from("puzzle5d-board-events-depth-malformed"))?;
                    if self.depth == 0 {
                        self.event_end = self.byte_cursor;
                        self.stage = Puzzle5dBoardEventsStage::Decode;
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn take_payload(&mut self) -> Value {
        self.event.as_mut().and_then(Value::as_object_mut).and_then(|event| event.get_mut("payload")).map_or(Value::Null, |value| std::mem::replace(value, Value::Null))
    }
}

impl Puzzle5dBoardEventsWork {
    /// 🪪️ Binds the operation nonce its minted ids derive from.
    pub(super) fn bound(mut self, operation_nonce: u64) -> Self {
        self.operation_nonce = operation_nonce;
        self
    }
}

impl semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<Puzzle5dPlayApp>> for Puzzle5dBoardEventsWork {
    fn tool_id(&self) -> &'static str {
        "applyBoardEvents"
    }

    fn extent(&self, command: &Puzzle5dCommand, snapshot: &Puzzle5dPlaySnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Puzzle5dPlayApp>>>) -> Option<usize> {
        let projection = puzzle5d_editor_projection(snapshot);
        let bytes = Self::source(command).ok()?.len();
        let document_items = projection.get("parts").and_then(Value::as_array).map_or(0, Vec::len).checked_add(projection.get("fasteners").and_then(Value::as_array).map_or(0, Vec::len))?.checked_add(2)?;
        let items = bytes.checked_mul(document_items)?;
        (bytes <= crate::retained_command::PUZZLE_COMMAND_RAW_BYTES && items <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS).then_some(items.max(1))
    }

    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Puzzle5dPlayApp>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        crate::retained_command::step_demands::<EditorApp<Puzzle5dPlayApp>>(size_of::<Self>())
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Puzzle5dPlayApp>>, cx: &mut StepContext<'_>) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<Puzzle5dPlayApp>>, Fault> {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command, snapshot, config: _, interaction: _, hover: _hover, context, .. } = *input;
        let view_state = context.and_then(|context| context.view_state.as_ref());
        let window_config_snapshot = context.and_then(|context| context.window_config.as_ref());
        let projection = puzzle5d_editor_projection(snapshot);
        let source = Self::source(command)?;
        match self.stage {
            Puzzle5dBoardEventsStage::Open | Puzzle5dBoardEventsStage::Scan => {
                self.scan_one(source)?;
                Ok(crate::puzzle_progress_step!("puzzle5d-board-event-scan", "Reading board event", "Board-Ereignis wird gelesen"))
            }
            Puzzle5dBoardEventsStage::Decode => {
                let start = self.event_start.ok_or_else(|| Fault::from("puzzle5d-board-events-event-owner-missing"))?;
                self.event = Some(parse(source.get(start..self.event_end).ok_or_else(|| Fault::from("puzzle5d-board-events-event-range"))?, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|_| Fault::from("puzzle5d-board-events-event-malformed"))?);
                self.stage = Puzzle5dBoardEventsStage::Dispatch;
                Ok(crate::puzzle_progress_step!("puzzle5d-board-event-decode", "Decoding board event", "Board-Ereignis wird dekodiert"))
            }
            Puzzle5dBoardEventsStage::Dispatch => {
                let name = self.event.as_ref().and_then(|event| event.get("name")).and_then(Value::as_str).map(str::to_string);
                let payload = self.take_payload();
                match name.as_deref() {
                    Some("camera") => {
                        self.camera2d = Some(<Puzzle5dCamera2d as FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&payload)).map_err(|_| Fault::from("puzzle5d-board-events-camera-malformed"))?);
                        self.next_event();
                    }
                    // 🎬️ A board drag is ONE `drag` gesture record: its targets (each once) move by its offset as a
                    // `drag-selection2d` leaf the transform tool yields at completion — relative, so the drag replays
                    // on whatever flat poses its base holds. The board drag moves ONLY the flat pose: the world origin
                    // stays where the 3d pane put it, which is what makes a board drag a plan edit. Other gesture
                    // kinds move no part here.
                    Some("gesture") => {
                        let drag = payload.get("kind").and_then(Value::as_str) == Some("drag");
                        let offset = payload.get("dx").and_then(Value::as_f64).zip(payload.get("dy").and_then(Value::as_f64)).filter(|(dx, dy)| dx.is_finite() && dy.is_finite());
                        if let Some((dx, dy)) = offset.filter(|_| drag) {
                            let targets = payload.get("targets").and_then(Value::as_array).into_iter().flatten().filter_map(Value::as_str).filter(|id| !id.is_empty()).map(str::to_string);
                            let record = world3d::utilities::transform::Puzzle5dSelectionRecord::new(targets, world3d::utilities::transform::Puzzle5dSelectionMotion::Board { dx, dy });
                            if self.drags.len() >= crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS {
                                return Err(Fault::from("puzzle5d-board-events-drag-capacity"));
                            }
                            self.drag_at.get_or_insert(self.mutations.len());
                            self.drags.push(record);
                        }
                        self.next_event();
                    }
                    Some("edgeCreate") => {
                        self.pending_source = payload.get("source").and_then(Value::as_str).filter(|id| !id.is_empty()).map(str::to_string);
                        self.pending_target = payload.get("target").and_then(Value::as_str).filter(|id| !id.is_empty()).map(str::to_string);
                        self.pending_edge_id = payload.get("id").and_then(Value::as_str).filter(|id| !id.is_empty()).map(str::to_string);
                        if self.pending_edge_id.is_none() {
                            self.pending_edge_id = Some(format!("fastener-{:016x}-{}", self.operation_nonce, self.fresh_cursor));
                            self.fresh_cursor = self.fresh_cursor.saturating_add(1);
                        }
                        self.pending_edge_kind = payload.get("edgeKind").and_then(Value::as_str).filter(|kind| !kind.is_empty()).map(str::to_string);
                        self.fastener_cursor = 0;
                        self.stage = Puzzle5dBoardEventsStage::ScanEdge;
                    }
                    Some("edgeDelete") => {
                        if let Some(id) = payload.get("id").and_then(Value::as_str).filter(|id| !id.is_empty()) {
                            self.push(crate::standards::v1::subsets::any::schema::mutations::disconnect_grips(id.to_string()))?;
                            self.removed_ids.push(id.to_string());
                        }
                        self.next_event();
                    }
                    Some("select") => {
                        self.select_ids = Some(payload.get("ids").and_then(Value::as_array).map(|ids| ids.iter().filter_map(Value::as_str).map(str::to_string).collect()).unwrap_or_default());
                        self.next_event();
                    }
                    Some("nodeDelete") => {
                        self.pending_delete_id = payload.get("id").and_then(Value::as_str).filter(|id| !id.is_empty()).map(str::to_string);
                        self.fastener_cursor = 0;
                        self.stage = Puzzle5dBoardEventsStage::ScanDeleteEdges;
                    }
                    Some("brushPlace") => {
                        let mut brush = Puzzle5dAddBrushPartWork::from_board_payload(payload);
                        brush.operation_nonce = self.operation_nonce;
                        brush.fresh_cursor = self.fresh_cursor;
                        self.fresh_cursor = self.fresh_cursor.saturating_add(2);
                        self.brush = Some(brush);
                        self.stage = Puzzle5dBoardEventsStage::Brush;
                    }
                    _ => self.next_event(),
                }
                Ok(crate::puzzle_progress_step!("puzzle5d-board-event-dispatch", "Applying board event", "Board-Ereignis wird angewendet"))
            }
            Puzzle5dBoardEventsStage::ScanEdge => {
                if self.pending_source.is_none() || self.pending_target.is_none() {
                    self.next_event();
                    return Ok(crate::puzzle_progress_step!("puzzle5d-board-edge", "Checking board edge", "Board-Kante wird geprüft"));
                }
                if let Some(fastener) = projection.get("fasteners").and_then(Value::as_array).and_then(|fasteners| fasteners.get(self.fastener_cursor)) {
                    self.fastener_cursor += 1;
                    let source = fastener.get("source").and_then(Value::as_str);
                    let target = fastener.get("target").and_then(Value::as_str);
                    if (source == self.pending_source.as_deref() && target == self.pending_target.as_deref()) || (source == self.pending_target.as_deref() && target == self.pending_source.as_deref()) {
                        self.pending_source = None;
                        self.pending_target = None;
                    }
                    return Ok(crate::puzzle_progress_step!("puzzle5d-board-edge", "Checking board edge", "Board-Kante wird geprüft"));
                }
                let id = self.pending_edge_id.take().expect("preflighted edge id");
                let source = self.pending_source.take().expect("preflighted edge source");
                let target = self.pending_target.take().expect("preflighted edge target");
                let kind = self.pending_edge_kind.take();
                self.push(crate::standards::v1::subsets::any::schema::mutations::connect_grips(id, source, target, kind, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, None))?;
                self.next_event();
                Ok(crate::puzzle_progress_step!("puzzle5d-board-edge", "Creating board edge", "Board-Kante wird erstellt"))
            }
            Puzzle5dBoardEventsStage::ScanDeleteEdges => {
                let Some(id) = self.pending_delete_id.as_deref() else {
                    self.next_event();
                    return Ok(crate::puzzle_progress_step!("puzzle5d-board-delete", "Deleting board node", "Board-Knoten wird gelöscht"));
                };
                if let Some(fastener) = projection.get("fasteners").and_then(Value::as_array).and_then(|fasteners| fasteners.get(self.fastener_cursor)) {
                    self.fastener_cursor += 1;
                    let incident = fastener.get("source").and_then(Value::as_str).is_some_and(|grip| grip.split_once(':').is_some_and(|(part_id, _)| part_id == id))
                        || fastener.get("target").and_then(Value::as_str).is_some_and(|grip| grip.split_once(':').is_some_and(|(part_id, _)| part_id == id));
                    if incident {
                        if let Some(fastener_id) = fastener.get("id").and_then(Value::as_str) {
                            self.push(crate::standards::v1::subsets::any::schema::mutations::disconnect_grips(fastener_id.to_string()))?;
                        }
                    }
                    return Ok(crate::puzzle_progress_step!("puzzle5d-board-delete-edge", "Removing attached edge", "Verbundene Kante wird entfernt"));
                }
                let id = self.pending_delete_id.take().expect("preflighted deleted part");
                self.removed_ids.push(id.clone());
                self.push(crate::standards::v1::subsets::any::schema::mutations::delete_part(id))?;
                self.next_event();
                Ok(crate::puzzle_progress_step!("puzzle5d-board-delete", "Deleting board node", "Board-Knoten wird gelöscht"))
            }
            Puzzle5dBoardEventsStage::Brush => {
                let brush = self.brush.as_mut().ok_or_else(|| Fault::from("puzzle5d-board-brush-owner-missing"))?;
                match <Puzzle5dAddBrushPartWork as semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<Puzzle5dPlayApp>>>::step(brush, input, cx)? {
                    semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Progress { stage, preview } => Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Progress { stage, preview }),
                    semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Replay { stage, preview } => Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Replay { stage, preview }),
                    semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(emit) => {
                        self.placed_id = emit.interaction_writes.iter().flat_map(|write| write.targets.iter()).map(|target| target.id.clone()).next();
                        let mut mutations = emit.artifact_mutations.into_iter();
                        self.brush_first = mutations.next();
                        self.brush_second = mutations.next();
                        if mutations.next().is_some() {
                            return Err(Fault::from("puzzle5d-board-brush-output-capacity"));
                        }
                        self.stage = Puzzle5dBoardEventsStage::DrainBrush;
                        Ok(crate::puzzle_progress_step!("puzzle5d-board-brush-transfer", "Publishing brush mutation", "Pinselmutation wird veröffentlicht"))
                    }
                    semio_framework_plugin::retained_command::ArtifactCommandWorkStep::CompleteDownload { .. } | semio_framework_plugin::retained_command::ArtifactCommandWorkStep::CompleteWithEphemeral { .. } => Err(Fault::from("puzzle5d-board-brush-download-unsupported")),
                }
            }
            Puzzle5dBoardEventsStage::DrainBrush => {
                if let Some(mutation) = self.brush_first.take() {
                    self.push(mutation)?;
                    return Ok(crate::puzzle_progress_step!("puzzle5d-board-brush-transfer", "Publishing brush mutation", "Pinselmutation wird veröffentlicht"));
                }
                if let Some(mutation) = self.brush_second.take() {
                    self.push(mutation)?;
                    return Ok(crate::puzzle_progress_step!("puzzle5d-board-brush-transfer", "Publishing brush mutation", "Pinselmutation wird veröffentlicht"));
                }
                if let Some(brush) = self.brush.as_mut() {
                    <Puzzle5dAddBrushPartWork as semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<Puzzle5dPlayApp>>>::begin_close(brush);
                }
                self.stage = Puzzle5dBoardEventsStage::CloseBrush;
                Ok(crate::puzzle_progress_step!("puzzle5d-board-brush-close", "Releasing brush owners", "Pinseleigentümer werden freigegeben"))
            }
            Puzzle5dBoardEventsStage::CloseBrush => {
                let Some(brush) = self.brush.as_mut() else {
                    self.next_event();
                    return Ok(crate::puzzle_progress_step!("puzzle5d-board-event-scan", "Reading board event", "Board-Ereignis wird gelesen"));
                };
                let progress = match <Puzzle5dAddBrushPartWork as semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<Puzzle5dPlayApp>>>::close_step(brush, cx.retained_grant()) {
                    semio_framework_job::InteractiveJobCloseStep::Pending { progress } | semio_framework_job::InteractiveJobCloseStep::Complete { progress } => progress,
                    semio_framework_job::InteractiveJobCloseStep::Blocked => return Err(Fault::from("puzzle5d-board-brush-close-blocked")),
                    semio_framework_job::InteractiveJobCloseStep::Refused { .. } => return Err(Fault::from("puzzle5d-board-brush-close-refused")),
                };
                cx.consume_retained(progress).map_err(|_| Fault::from("puzzle5d-board-brush-close-grant"))?;
                if <Puzzle5dAddBrushPartWork as semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<Puzzle5dPlayApp>>>::terminal_is_empty(brush) {
                    self.brush.take();
                    self.next_event();
                }
                Ok(crate::puzzle_progress_step!("puzzle5d-board-brush-close", "Releasing brush owners", "Pinseleigentümer werden freigegeben"))
            }
            Puzzle5dBoardEventsStage::Complete => {
                self.stage = Puzzle5dBoardEventsStage::Closing;
                // 🪟️ A board `camera` row is the pane's own persisted pose. Dropping it (which this work did
                // until 5A2) made pan/zoom in the board window revert on every refresh.
                let camera2d = self.camera2d.take();
                let window_config_mutations = match (camera2d, view_state) {
                    (Some(camera2d), Some(view)) => {
                        let mut next = window_ownership::config_from_snapshot(window_config_snapshot);
                        next.camera2d = camera2d;
                        vec![window_ownership::addressed_config(view, next)?]
                    }
                    _ => Vec::new(),
                };
                let mut interaction_writes = Vec::new();
                match (self.select_ids.take(), self.placed_id.take()) {
                    (Some(ids), _) => interaction_writes.push(InteractionWrite::replace(PUZZLE5D_INTERACTION_DOMAIN, PUZZLE5D_GRANULARITY_PART, ids)),
                    (None, Some(id)) => interaction_writes.push(InteractionWrite::replace(PUZZLE5D_INTERACTION_DOMAIN, PUZZLE5D_GRANULARITY_PART, [id])),
                    (None, None) => {}
                }
                let removed = std::mem::take(&mut self.removed_ids);
                if !removed.is_empty() {
                    // 🧹️ A delete clears what it removed through a SUBTRACTIVE write naming exactly those ids;
                    // an empty `Replace` is a silent no-op in `protocol::next_selection`.
                    let targets: Vec<InteractionTarget> = removed
                        .into_iter()
                        .flat_map(|id| [InteractionTarget { granularity: PUZZLE5D_GRANULARITY_PART.to_string(), id: id.clone() }, InteractionTarget { granularity: PUZZLE5D_GRANULARITY_FASTENER.to_string(), id }])
                        .collect();
                    interaction_writes.push(InteractionWrite { domain: PUZZLE5D_INTERACTION_DOMAIN.into(), targets, merge: MergeMode::Subtractive });
                }
                let drags = std::mem::take(&mut self.drags);
                let drag_at = self.drag_at.take().unwrap_or(self.mutations.len()).min(self.mutations.len());
                let base = snapshot.typed_arc();
                // 🔒️ A locked part refuses the drag; the rest of the batch still lands, and the leaf names the
                // skipped part as `mutation.partial`.
                self.locked_refused |= drags.iter().any(|record| record.targets.iter().any(|id| base.parts.iter().any(|part| &part.id == id && part.part_2d.locked == Some(true))));
                let request = world3d::utilities::transform::TransformToolRequest { base, records: drags };
                let committed = (!request.records.is_empty()).then(|| world3d::utilities::transform::puzzle5d_transform_tool_commit("applyBoardEvents", &self.authoring_seed, request)).flatten();
                let mut artifact_mutations = std::mem::take(&mut self.mutations);
                let transaction = committed.map(|(transaction, leaves)| {
                    let later = artifact_mutations.split_off(drag_at);
                    artifact_mutations.extend(leaves);
                    artifact_mutations.extend(later);
                    transaction
                });
                let effects = if self.locked_refused {
                    self.locked_refused = false;
                    puzzle5d_notice_emit(view_state, |labels| labels.selection_locked.as_str()).effects
                } else {
                    Vec::new()
                };
                // 🛠️ A committed drag transaction stamps every op of this ONE edit.
                Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit {
                    artifact_mutations,
                    window_config_mutations,
                    interaction_writes,
                    effects,
                    transaction: transaction.filter(|_| !self.authoring_seed.is_empty()),
                    ui_scope: UiDirtyScope::Full,
                    ..Default::default()
                }))
            }
            Puzzle5dBoardEventsStage::Closing => Err(Fault::from("puzzle5d-board-events-closing")),
        }
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle5dBoardEventsStage::Closing;
        if let Some(brush) = self.brush.as_mut().filter(|brush| !brush.is_closing()) {
            <Puzzle5dAddBrushPartWork as semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<Puzzle5dPlayApp>>>::begin_close(brush);
        }
        self.close_owners.stage(Puzzle5dBoardEventsWorkOwners { mutations: std::mem::take(&mut self.mutations), brush_first: std::mem::take(&mut self.brush_first), brush_second: std::mem::take(&mut self.brush_second), event: std::mem::take(&mut self.event), drags: std::mem::take(&mut self.drags), drag_at: std::mem::take(&mut self.drag_at), pending_source: std::mem::take(&mut self.pending_source), pending_target: std::mem::take(&mut self.pending_target), pending_edge_id: std::mem::take(&mut self.pending_edge_id), pending_edge_kind: std::mem::take(&mut self.pending_edge_kind), pending_delete_id: std::mem::take(&mut self.pending_delete_id), camera2d: std::mem::take(&mut self.camera2d), select_ids: std::mem::take(&mut self.select_ids), removed_ids: std::mem::take(&mut self.removed_ids), placed_id: std::mem::take(&mut self.placed_id) });
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
        let Some(brush) = self.brush.as_mut() else { return self.close_owners.close_step(grant) };
        let step = <Puzzle5dAddBrushPartWork as semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<Puzzle5dPlayApp>>>::close_step(brush, grant);
        if <Puzzle5dAddBrushPartWork as semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<Puzzle5dPlayApp>>>::terminal_is_empty(brush) {
            self.brush.take();
        }
        match step {
            semio_framework_job::InteractiveJobCloseStep::Complete { progress } => semio_framework_job::InteractiveJobCloseStep::Pending { progress },
            other => other,
        }
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        if let Some(brush) = self.brush.as_ref() {
            return <Puzzle5dAddBrushPartWork as semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<Puzzle5dPlayApp>>>::next_close_copy_byte_demand(brush);
        }
        Ok(self.close_owners.demands(0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        if let Some(brush) = self.brush.as_ref() {
            return <Puzzle5dAddBrushPartWork as semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<Puzzle5dPlayApp>>>::next_close_capacity_byte_demand(brush, maximum_copy_bytes);
        }
        Ok(self.close_owners.demands(maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        if let Some(brush) = self.brush.as_ref() {
            return <Puzzle5dAddBrushPartWork as semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<Puzzle5dPlayApp>>>::next_close_release_byte_demand(brush);
        }
        Ok(self.close_owners.demands(0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        if let Some(brush) = self.brush.as_ref() {
            return <Puzzle5dAddBrushPartWork as semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<Puzzle5dPlayApp>>>::next_close_depth_demand(brush);
        }
        Ok(self.close_owners.demands(0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.stage == Puzzle5dBoardEventsStage::Closing && self.brush.is_none() && self.close_owners.is_empty()
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle5dCreateFastenerStage {
    Source,
    Target,
    Existing,
    Compatibility,
    Emit,
    Complete,
    Closing,
}

pub(super) enum Puzzle5dGripScan {
    Progress,
    Found(Option<String>),
    Exhausted,
}

/// ♻️ The owners one `Puzzle5dCreateFastenerWork` still holds when its job closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
struct Puzzle5dCreateFastenerWorkOwners {
    pub(super) mutation: Option<Puzzle5dMutation>,
    pub(super) source_kind: Option<String>,
    pub(super) target_kind: Option<String>,
}

pub(super) struct Puzzle5dCreateFastenerWork {
    pub(super) stage: Puzzle5dCreateFastenerStage,
    pub(super) part_cursor: usize,
    pub(super) grip_cursor: usize,
    pub(super) fastener_cursor: usize,
    pub(super) compatibility_cursor: usize,
    pub(super) processed_units: usize,
    pub(super) source_kind: Option<String>,
    pub(super) target_kind: Option<String>,
    pub(super) mutation: Option<Puzzle5dMutation>,
    pub(super) operation_nonce: u64,
    pub(super) close_owners: crate::puzzle_job::WorkClosing<Puzzle5dCreateFastenerWorkOwners>,
}

impl Default for Puzzle5dCreateFastenerWork {
    fn default() -> Self {
        Self { close_owners: Default::default(), stage: Puzzle5dCreateFastenerStage::Source, part_cursor: 0, grip_cursor: 0, fastener_cursor: 0, compatibility_cursor: 0, processed_units: 0, source_kind: None, target_kind: None, mutation: None, operation_nonce: 0 }
    }
}

impl Puzzle5dCreateFastenerWork {
    fn endpoint<'a>(command: &'a Puzzle5dCommand, primary: &str, alias: &str) -> &'a str {
        command.args().and_then(|args| args.get(primary).or_else(|| args.get(alias))).and_then(Value::as_str).filter(|id| !id.is_empty()).unwrap_or("")
    }

    pub(super) fn scan_grip(&mut self, snapshot: &Puzzle5dPlaySnapshot, target: &str) -> Puzzle5dGripScan {
        let projection = puzzle5d_editor_projection(snapshot);
        let Some(part) = projection.get("parts").and_then(Value::as_array).and_then(|parts| parts.get(self.part_cursor)) else {
            return Puzzle5dGripScan::Exhausted;
        };
        let Some(grip) = part.get("grips").and_then(Value::as_array).and_then(|grips| grips.get(self.grip_cursor)) else {
            self.part_cursor += 1;
            self.grip_cursor = 0;
            return Puzzle5dGripScan::Progress;
        };
        self.grip_cursor += 1;
        let Some(part_id) = part.get("id").and_then(Value::as_str) else { return Puzzle5dGripScan::Progress };
        let Some(grip_id) = grip.get("id").and_then(Value::as_str) else { return Puzzle5dGripScan::Progress };
        if puzzle5d_grip_full_id(part_id, grip_id) != target {
            return Puzzle5dGripScan::Progress;
        }
        let kind = grip.get("gripKind").and_then(Value::as_str).filter(|kind| !kind.is_empty()).or_else(|| grip.get("2d").and_then(|value| value.get("gripKind")).and_then(Value::as_str).filter(|kind| !kind.is_empty())).map(str::to_string);
        Puzzle5dGripScan::Found(kind)
    }

    pub(super) fn complete_empty(&mut self) -> semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<Puzzle5dPlayApp>> {
        self.stage = Puzzle5dCreateFastenerStage::Complete;
        semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit::default())
    }

    fn arg_f64(command: &Puzzle5dCommand, key: &str) -> f64 {
        command.args().and_then(|args| args.get(key)).and_then(Value::as_f64).unwrap_or(0.0)
    }
}

impl Puzzle5dCreateFastenerWork {
    /// 🪪️ Binds the operation nonce its minted ids derive from.
    pub(super) fn bound(mut self, operation_nonce: u64) -> Self {
        self.operation_nonce = operation_nonce;
        self
    }
}

impl semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<Puzzle5dPlayApp>> for Puzzle5dCreateFastenerWork {
    fn tool_id(&self) -> &'static str {
        "createFastener"
    }

    fn extent(&self, _command: &Puzzle5dCommand, snapshot: &Puzzle5dPlaySnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Puzzle5dPlayApp>>>) -> Option<usize> {
        let projection = puzzle5d_editor_projection(snapshot);
        let parts = projection.get("parts").and_then(Value::as_array).map_or(0, Vec::len);
        let fasteners = projection.get("fasteners").and_then(Value::as_array).map_or(0, Vec::len);
        let compatibility = projection.get("kindCompatibility").and_then(Value::as_array).map_or(0, Vec::len);
        let items = parts.checked_mul(2)?.checked_add(fasteners)?.checked_add(compatibility)?.checked_add(1)?;
        (items <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS).then_some(items)
    }

    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Puzzle5dPlayApp>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        crate::retained_command::step_demands::<EditorApp<Puzzle5dPlayApp>>(size_of::<Self>())
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Puzzle5dPlayApp>>, _cx: &mut StepContext<'_>) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<Puzzle5dPlayApp>>, Fault> {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command, snapshot, config: _config, interaction: _interaction, hover: _hover, context: _context, .. } = *input;
        let projection = puzzle5d_editor_projection(snapshot);
        if self.processed_units >= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS {
            return Err(Fault::from("puzzle5d-create-fastener-work-capacity"));
        }
        self.processed_units += 1;
        let source = Self::endpoint(command, "source", "attracting");
        let target = Self::endpoint(command, "target", "attracted");
        if source.is_empty() || target.is_empty() || source == target {
            return Ok(self.complete_empty());
        }
        match self.stage {
            Puzzle5dCreateFastenerStage::Source => match self.scan_grip(snapshot, source) {
                Puzzle5dGripScan::Progress => Ok(crate::puzzle_progress_step!("puzzle5d-create-fastener-source", "Finding source grip", "Quellgriff wird gesucht")),
                Puzzle5dGripScan::Found(kind) => {
                    self.source_kind = kind;
                    self.part_cursor = 0;
                    self.grip_cursor = 0;
                    self.stage = Puzzle5dCreateFastenerStage::Target;
                    Ok(crate::puzzle_progress_step!("puzzle5d-create-fastener-target", "Finding target grip", "Zielgriff wird gesucht"))
                }
                Puzzle5dGripScan::Exhausted => Ok(self.complete_empty()),
            },
            Puzzle5dCreateFastenerStage::Target => match self.scan_grip(snapshot, target) {
                Puzzle5dGripScan::Progress => Ok(crate::puzzle_progress_step!("puzzle5d-create-fastener-target", "Finding target grip", "Zielgriff wird gesucht")),
                Puzzle5dGripScan::Found(kind) => {
                    self.target_kind = kind;
                    self.stage = Puzzle5dCreateFastenerStage::Existing;
                    Ok(crate::puzzle_progress_step!("puzzle5d-create-fastener-existing", "Checking existing fastener", "Bestehende Verbindung wird geprüft"))
                }
                Puzzle5dGripScan::Exhausted => Ok(self.complete_empty()),
            },
            Puzzle5dCreateFastenerStage::Existing => {
                if let Some(fastener) = projection.get("fasteners").and_then(Value::as_array).and_then(|fasteners| fasteners.get(self.fastener_cursor)) {
                    self.fastener_cursor += 1;
                    let existing_source = fastener.get("source").and_then(Value::as_str).unwrap_or("");
                    let existing_target = fastener.get("target").and_then(Value::as_str).unwrap_or("");
                    if (existing_source == source && existing_target == target) || (existing_source == target && existing_target == source) {
                        return Ok(self.complete_empty());
                    }
                    return Ok(crate::puzzle_progress_step!("puzzle5d-create-fastener-existing", "Checking existing fastener", "Bestehende Verbindung wird geprüft"));
                }
                self.stage = Puzzle5dCreateFastenerStage::Compatibility;
                Ok(crate::puzzle_progress_step!("puzzle5d-create-fastener-compatibility", "Checking kind compatibility", "Artkompatibilität wird geprüft"))
            }
            Puzzle5dCreateFastenerStage::Compatibility => {
                if self.source_kind.is_none() || self.target_kind.is_none() {
                    self.stage = Puzzle5dCreateFastenerStage::Emit;
                    return Ok(crate::puzzle_progress_step!("puzzle5d-create-fastener-emit", "Creating fastener", "Verbindung wird erstellt"));
                }
                let rows = projection.get("kindCompatibility").and_then(Value::as_array).map(Vec::as_slice).unwrap_or_default();
                if rows.is_empty() {
                    self.stage = Puzzle5dCreateFastenerStage::Emit;
                    return Ok(crate::puzzle_progress_step!("puzzle5d-create-fastener-emit", "Creating fastener", "Verbindung wird erstellt"));
                }
                let Some(row) = rows.get(self.compatibility_cursor) else { return Ok(self.complete_empty()) };
                self.compatibility_cursor += 1;
                let row_source = row.get("source").and_then(Value::as_str).unwrap_or("");
                let row_target = row.get("target").and_then(Value::as_str).unwrap_or("");
                let bidirectional = row.get("bidirectional").and_then(Value::as_bool).unwrap_or(false);
                let source_kind = self.source_kind.as_deref().unwrap_or("");
                let target_kind = self.target_kind.as_deref().unwrap_or("");
                if (row_source == source_kind && row_target == target_kind) || (bidirectional && row_source == target_kind && row_target == source_kind) {
                    self.stage = Puzzle5dCreateFastenerStage::Emit;
                }
                Ok(crate::puzzle_progress_step!("puzzle5d-create-fastener-compatibility", "Checking kind compatibility", "Artkompatibilität wird geprüft"))
            }
            Puzzle5dCreateFastenerStage::Emit => {
                let id = command.args().and_then(|args| args.get("id").or_else(|| args.get("fastenerId"))).and_then(Value::as_str).filter(|id| !id.is_empty()).map_or_else(|| format!("fastener-{:016x}-0", self.operation_nonce), str::to_string);
                let fastener_kind = command.args().and_then(|args| args.get("fastenerKind").or_else(|| args.get("edgeKind"))).and_then(Value::as_str).filter(|kind| !kind.is_empty()).map(str::to_string);
                self.mutation = Some(crate::standards::v1::subsets::any::schema::mutations::connect_grips(
                    id,
                    source.to_string(),
                    target.to_string(),
                    fastener_kind,
                    Self::arg_f64(command, "gap"),
                    Self::arg_f64(command, "shift"),
                    Self::arg_f64(command, "rise"),
                    Self::arg_f64(command, "rotation"),
                    Self::arg_f64(command, "turn"),
                    Self::arg_f64(command, "tilt"),
                    Self::arg_f64(command, "x"),
                    Self::arg_f64(command, "y"), None,
                ));
                self.stage = Puzzle5dCreateFastenerStage::Complete;
                Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit { artifact_mutations: self.mutation.take().into_iter().collect(), ui_scope: UiDirtyScope::Full, ..Default::default() }))
            }
            Puzzle5dCreateFastenerStage::Complete => Err(Fault::from("puzzle5d-create-fastener-complete-repolled")),
            Puzzle5dCreateFastenerStage::Closing => Err(Fault::from("puzzle5d-create-fastener-closing")),
        }
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle5dCreateFastenerStage::Closing;
        self.close_owners.stage(Puzzle5dCreateFastenerWorkOwners { mutation: std::mem::take(&mut self.mutation), source_kind: std::mem::take(&mut self.source_kind), target_kind: std::mem::take(&mut self.target_kind) });
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
        self.close_owners.close_step(grant)
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.stage == Puzzle5dCreateFastenerStage::Closing && self.close_owners.is_empty()
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }
}

/// 📦️ Rows one `setActiveExample` step replays. A declared unit is a CHUNK, not a row:
/// `🌙️capsule-dream` is 2 880 parts + 2 865 fasteners, so a row-per-unit extent (≈5 749, and ≈11 500 when
/// reloading capsule-dream over itself) overruns the shared `PUZZLE_COMMAND_WORK_ITEMS` band and the
/// switch could only ever refuse. Chunking makes the same replay fit without widening a constant three
/// fixtures pin. It is 8× puzzle 3d's `PUZZLE3D_SET_ACTIVE_EXAMPLE_CHUNK` on purpose: 3d steps over a
/// TYPED snapshot, while every 5d work re-derives `puzzle5d_projection_value` per step, so a step's cost
/// is dominated by that one O(document) projection rather than by the rows it then replays — fewer,
/// fatter steps is strictly cheaper here, and capsule-dream's switch drops from ≈720 steps to ≈100.
const PUZZLE5D_SET_ACTIVE_EXAMPLE_CHUNK: usize = 64;

/// 🔢️ Stage transitions and whole-document rows a switch always pays on top of its chunks (label, domain,
/// description, catalogs and one exhaustion step per cursored stage). 3d's own count, kept identical.
const PUZZLE5D_SET_ACTIVE_EXAMPLE_FIXED_STEPS: usize = 13;

/// 🧮️ Mutations one switch may accumulate: exactly what the declared extent can legitimately produce, so
/// `push` can never refuse work `extent` already admitted.
const PUZZLE5D_SET_ACTIVE_EXAMPLE_MUTATIONS: usize = crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS * PUZZLE5D_SET_ACTIVE_EXAMPLE_CHUNK;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle5dSetActiveExampleStage {
    ClearFasteners,
    ClearParts,
    Label,
    Domain,
    Description,
    ClearCompatibility,
    AddCompatibility,
    Catalogs,
    AddParts,
    AddFasteners,
    Complete,
    Closing,
}

/// ♻️ The owners one `Puzzle5dSetActiveExampleWork` still holds when its job closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
struct Puzzle5dSetActiveExampleWorkOwners {
    pub(super) mutations: Vec<Puzzle5dMutation>,
    pub(super) before: Option<std::sync::Arc<Puzzle5dSetActiveExampleBefore>>,
}

pub(super) struct Puzzle5dSetActiveExampleWork {
    pub(super) stage: Puzzle5dSetActiveExampleStage,
    pub(super) cursor: usize,
    pub(super) admitted: bool,
    pub(super) mutations: Vec<Puzzle5dMutation>,
    /// 🗂️ The BEFORE document's ids to clear — fastener ids, part ids and compatibility pairs —
    /// harvested ONCE and then merely indexed by the cursored clearing stages.
    ///
    /// 🐛️ `step` used to call `puzzle5d_projection_value` over the run's snapshot on entry, i.e. re-derive the
    /// whole document (`Value` → `DslValue` → os-pack `Value`) on each of the ~110 chunk
    /// steps one example switch takes, and then re-walk its arrays. The run's snapshot is an `Arc`
    /// the retained driver holds FIXED for the whole run (nothing publishes before `Complete`), so
    /// every one of those derivations produced the same bytes — and leaving `🌙️capsule-dream`
    /// (2 880 parts, ~3.5 MB of JSON) paid that whole projection ~110 times for one switch.
    pub(super) before: Option<std::sync::Arc<Puzzle5dSetActiveExampleBefore>>,
    pub(super) close_owners: crate::puzzle_job::WorkClosing<Puzzle5dSetActiveExampleWorkOwners>,
}

/// 🗂️ Everything the clearing stages need from the BEFORE document, harvested in one pass.
#[derive(Default, semio_framework_value::RetireOwned)]
struct Puzzle5dSetActiveExampleBefore {
    pub(super) fastener_ids: Vec<String>,
    pub(super) part_ids: Vec<String>,
    pub(super) compatibility: Vec<(String, String)>,
}

impl Default for Puzzle5dSetActiveExampleWork {
    fn default() -> Self {
        Self { close_owners: Default::default(), stage: Puzzle5dSetActiveExampleStage::ClearFasteners, cursor: 0, admitted: false, mutations: Vec::with_capacity(crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS), before: None }
    }
}

impl Puzzle5dSetActiveExampleWork {
    /// 📏️ How many semantic units switching to `target` from `snapshot` costs — every old fastener and
    /// part cleared, every old compatibility row dropped, then the target's own compatibility rows, parts
    /// and fasteners, each counted in `PUZZLE5D_SET_ACTIVE_EXAMPLE_CHUNK`-sized steps, plus the fixed
    /// stage-transition and whole-document rows.
    pub(super) fn units(command: &Puzzle5dCommand, snapshot: &Puzzle5dPlaySnapshot) -> Option<usize> {
        let projection = puzzle5d_editor_projection(snapshot);
        let target = Self::target(command)?;
        let rows = [
            snapshot.typed().fasteners.len(),
            projection.get("parts").and_then(Value::as_array).map_or(0, Vec::len),
            projection.get("kindCompatibility").and_then(Value::as_array).map_or(0, Vec::len),
            Self::compatibility_rows(target).len(),
            target.parts.len(),
            target.fasteners.len(),
        ];
        rows.into_iter().try_fold(PUZZLE5D_SET_ACTIVE_EXAMPLE_FIXED_STEPS, |units, len| units.checked_add(len.div_ceil(PUZZLE5D_SET_ACTIVE_EXAMPLE_CHUNK)))
    }

    /// ✂️ The next `PUZZLE5D_SET_ACTIVE_EXAMPLE_CHUNK` rows of a cursored stage, advancing the cursor.
    fn take_chunk(cursor: &mut usize, len: usize) -> std::ops::Range<usize> {
        if *cursor >= len {
            return *cursor..*cursor;
        }
        let end = cursor.saturating_add(PUZZLE5D_SET_ACTIVE_EXAMPLE_CHUNK).min(len);
        let range = *cursor..end;
        *cursor = end;
        range
    }

    pub(super) fn target(command: &Puzzle5dCommand) -> Option<&'static Puzzle5dDocument> {
        let example_id = command.args().and_then(|args| args.get("exampleId")).and_then(Value::as_str).unwrap_or("");
        match example_id {
            "" => Some(&EMPTY_EXAMPLE_DOCUMENT),
            PUZZLE5D_EXAMPLE_CONCRETE_FOREST | "concrete" => Some(&CONCRETE_FOREST_EXAMPLE_DOCUMENT),
            PUZZLE5D_EXAMPLE_NAKAGIN | "nakagin" => Some(&NAKAGIN_EXAMPLE_DOCUMENT),
            PUZZLE5D_EXAMPLE_CAPSULE_DREAM | "capsule" => Some(&CAPSULE_DREAM_EXAMPLE_DOCUMENT),
            _ => None,
        }
    }

    fn compatibility_rows(document: &Puzzle5dDocument) -> &[Value] {
        document.kind_compatibility.as_ref().and_then(Value::as_array).map(Vec::as_slice).unwrap_or_default()
    }

    pub(super) fn push(&mut self, mutation: Puzzle5dMutation) -> Result<(), Fault> {
        if self.mutations.len() >= PUZZLE5D_SET_ACTIVE_EXAMPLE_MUTATIONS {
            return Err(Fault::from("puzzle5d-set-active-example-output-capacity"));
        }
        self.mutations.push(mutation);
        Ok(())
    }
}

impl semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<Puzzle5dPlayApp>> for Puzzle5dSetActiveExampleWork {
    fn tool_id(&self) -> &'static str {
        "setActiveExample"
    }

    /// 🧯️ An example whose switch costs more than one edit's fixed work capacity is REFUSED with a notice
    /// on the first step, not admitted and faulted: `🌙️capsule-dream` is 2,880 parts + 2,865 fasteners, so
    /// it exceeds `PUZZLE_COMMAND_WORK_ITEMS` (4,096) on its own, and a bare `None` here would surface as
    /// the framework's opaque "exceeds fixed semantic work capacity" fault instead of a sentence the user
    /// can read. The refusal path therefore declares one unit, and `step` completes with the notice.
    fn extent(&self, command: &Puzzle5dCommand, snapshot: &Puzzle5dPlaySnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Puzzle5dPlayApp>>>) -> Option<usize> {
        let units = Self::units(command, snapshot)?;
        Some(if units <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS { units } else { 1 })
    }

    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Puzzle5dPlayApp>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        crate::retained_command::step_demands::<EditorApp<Puzzle5dPlayApp>>(size_of::<Self>())
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Puzzle5dPlayApp>>, _cx: &mut StepContext<'_>) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<Puzzle5dPlayApp>>, Fault> {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command, snapshot, config, interaction: _interaction, hover: _hover, context, .. } = *input;
        let view_state = context.and_then(|context| context.view_state.as_ref());
        if self.before.is_none() {
            let projection = puzzle5d_editor_projection(snapshot);
            let strings = |rows: Option<&Vec<Value>>, key: &str| -> Vec<String> {
                rows.map(|rows| rows.iter().filter_map(|row| row.get(key)).filter_map(Value::as_str).map(str::to_string).collect()).unwrap_or_default()
            };
            self.before = Some(std::sync::Arc::new(Puzzle5dSetActiveExampleBefore {
                fastener_ids: strings(projection.get("fasteners").and_then(Value::as_array), "id"),
                part_ids: strings(projection.get("parts").and_then(Value::as_array), "id"),
                compatibility: projection
                    .get("kindCompatibility")
                    .and_then(Value::as_array)
                    .map(|rows| rows.iter().map(|row| (row.get("source").and_then(Value::as_str).unwrap_or("").to_string(), row.get("target").and_then(Value::as_str).unwrap_or("").to_string())).collect())
                    .unwrap_or_default(),
            }));
        }
        if !self.admitted {
            self.admitted = true;
            if Self::target(command).is_none() || Self::units(command, snapshot).is_none_or(|units| units > crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS) {
                self.stage = Puzzle5dSetActiveExampleStage::Complete;
                return Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(puzzle5d_notice_emit(view_state, |labels| labels.example_too_large.as_str())));
            }
        }
        // 🔗️ An `Arc` clone, so the cached rows stay readable while the arms below take `&mut self`.
        let before = self.before.clone().unwrap_or_default();
        let Some(target) = Self::target(command) else {
            self.stage = Puzzle5dSetActiveExampleStage::Complete;
            return Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(puzzle5d_notice_emit(view_state, |labels| labels.example_too_large.as_str())));
        };
        match self.stage {
            Puzzle5dSetActiveExampleStage::ClearFasteners => {
                let range = Self::take_chunk(&mut self.cursor, before.fastener_ids.len());
                if !range.is_empty() {
                    for id in before.fastener_ids[range].to_vec() {
                        self.push(crate::standards::v1::subsets::any::schema::mutations::disconnect_grips(id))?;
                    }
                    return Ok(crate::puzzle_progress_step!("puzzle5d-example-clear-fastener", "Removing old fastener", "Alte Verbindung wird entfernt"));
                }
                self.cursor = 0;
                self.stage = Puzzle5dSetActiveExampleStage::ClearParts;
                Ok(crate::puzzle_progress_step!("puzzle5d-example-clear-part", "Removing old part", "Altes Teil wird entfernt"))
            }
            Puzzle5dSetActiveExampleStage::ClearParts => {
                let range = Self::take_chunk(&mut self.cursor, before.part_ids.len());
                if !range.is_empty() {
                    for id in before.part_ids[range].to_vec() {
                        self.push(crate::standards::v1::subsets::any::schema::mutations::delete_part(id))?;
                    }
                    return Ok(crate::puzzle_progress_step!("puzzle5d-example-clear-part", "Removing old part", "Altes Teil wird entfernt"));
                }
                self.cursor = 0;
                self.stage = Puzzle5dSetActiveExampleStage::Label;
                Ok(crate::puzzle_progress_step!("puzzle5d-example-label", "Updating document label", "Dokumenttitel wird aktualisiert"))
            }
            Puzzle5dSetActiveExampleStage::Label => {
                self.push(crate::standards::v1::subsets::any::schema::mutations::rename_puzzle5d(target.label.clone()))?;
                self.stage = Puzzle5dSetActiveExampleStage::Domain;
                Ok(crate::puzzle_progress_step!("puzzle5d-example-domain", "Updating document domain", "Dokumentdomäne wird aktualisiert"))
            }
            Puzzle5dSetActiveExampleStage::Domain => {
                self.push(crate::standards::v1::subsets::any::schema::mutations::change_domain(target.domain.clone()))?;
                self.stage = Puzzle5dSetActiveExampleStage::Description;
                Ok(crate::puzzle_progress_step!("puzzle5d-example-description", "Updating description", "Beschreibung wird aktualisiert"))
            }
            Puzzle5dSetActiveExampleStage::Description => {
                let description = target.meta.as_ref().and_then(|meta| meta.get("description")).and_then(Value::as_str).unwrap_or("");
                self.push(crate::standards::v1::subsets::any::schema::mutations::change_description(description.to_string()))?;
                self.stage = Puzzle5dSetActiveExampleStage::ClearCompatibility;
                Ok(crate::puzzle_progress_step!("puzzle5d-example-clear-compatibility", "Removing old compatibility", "Alte Kompatibilität wird entfernt"))
            }
            Puzzle5dSetActiveExampleStage::ClearCompatibility => {
                let range = Self::take_chunk(&mut self.cursor, before.compatibility.len());
                if !range.is_empty() {
                    for (source, target) in before.compatibility[range].to_vec() {
                        self.push(crate::standards::v1::subsets::any::schema::mutations::disconnect_kind_compatibility(source, target))?;
                    }
                    return Ok(crate::puzzle_progress_step!("puzzle5d-example-clear-compatibility", "Removing old compatibility", "Alte Kompatibilität wird entfernt"));
                }
                self.cursor = 0;
                self.stage = Puzzle5dSetActiveExampleStage::AddCompatibility;
                Ok(crate::puzzle_progress_step!("puzzle5d-example-add-compatibility", "Adding compatibility", "Kompatibilität wird hinzugefügt"))
            }
            Puzzle5dSetActiveExampleStage::AddCompatibility => {
                let rows = Self::compatibility_rows(target);
                let range = Self::take_chunk(&mut self.cursor, rows.len());
                if !range.is_empty() {
                    for row in &rows[range] {
                        let row: crate::Puzzle5dKindCompatibility = <crate::Puzzle5dKindCompatibility as FromValue>::from_value(row.to_value()).map_err(|_| Fault::from("puzzle5d-set-active-example-compatibility-malformed"))?;
                        self.push(crate::standards::v1::subsets::any::schema::mutations::connect_kind_compatibility(row.source, row.target, row.bidirectional, row.important, row.specificity, None))?;
                    }
                    return Ok(crate::puzzle_progress_step!("puzzle5d-example-add-compatibility", "Adding compatibility", "Kompatibilität wird hinzugefügt"));
                }
                self.cursor = 0;
                self.stage = Puzzle5dSetActiveExampleStage::Catalogs;
                Ok(crate::puzzle_progress_step!("puzzle5d-example-catalogs", "Updating kind catalogs", "Artenkataloge werden aktualisiert"))
            }
            Puzzle5dSetActiveExampleStage::Catalogs => {
                let catalogs = target.kind_catalogs.as_ref().map(|catalogs| <crate::Puzzle5dKindCatalogs as FromValue>::from_value(catalogs.to_value())).transpose().map_err(|_| Fault::from("puzzle5d-set-active-example-catalogs-malformed"))?;
                self.push(crate::standards::v1::subsets::any::schema::mutations::replace_kind_catalogs(catalogs))?;
                self.stage = Puzzle5dSetActiveExampleStage::AddParts;
                Ok(crate::puzzle_progress_step!("puzzle5d-example-add-part", "Adding example part", "Beispielteil wird hinzugefügt"))
            }
            Puzzle5dSetActiveExampleStage::AddParts => {
                let range = Self::take_chunk(&mut self.cursor, target.parts.len());
                if !range.is_empty() {
                    for part in &target.parts[range] {
                        let value = part.to_value();
                        let part = <crate::Puzzle5dPart as FromValue>::from_value(value.to_value()).map_err(|_| Fault::from("puzzle5d-set-active-example-part-malformed"))?;
                        self.push(crate::standards::v1::subsets::any::schema::mutations::create_part(part, None))?;
                    }
                    return Ok(crate::puzzle_progress_step!("puzzle5d-example-add-part", "Adding example part", "Beispielteil wird hinzugefügt"));
                }
                self.cursor = 0;
                self.stage = Puzzle5dSetActiveExampleStage::AddFasteners;
                Ok(crate::puzzle_progress_step!("puzzle5d-example-add-fastener", "Adding example fastener", "Beispielverbindung wird hinzugefügt"))
            }
            Puzzle5dSetActiveExampleStage::AddFasteners => {
                let range = Self::take_chunk(&mut self.cursor, target.fasteners.len());
                if !range.is_empty() {
                    for fastener in &target.fasteners[range] {
                        self.push(crate::standards::v1::subsets::any::schema::mutations::connect_grips(
                            fastener.id.clone(),
                            fastener.source.clone(),
                            fastener.target.clone(),
                            fastener.fastener_kind.clone(),
                            fastener.gap,
                            fastener.shift,
                            fastener.rise,
                            fastener.rotation,
                            fastener.turn,
                            fastener.tilt,
                            fastener.x,
                            fastener.y, None,
                        ))?;
                    }
                    return Ok(crate::puzzle_progress_step!("puzzle5d-example-add-fastener", "Adding example fastener", "Beispielverbindung wird hinzugefügt"));
                }
                self.stage = Puzzle5dSetActiveExampleStage::Complete;
                // 🧹️ Every part the old document held is gone, so a surviving selection would address ids that
                // no longer exist; it is cleared with a SUBTRACTIVE write naming exactly what is selected now.
                let selection = _interaction.selection.get(PUZZLE5D_INTERACTION_DOMAIN);
                let targets: Vec<InteractionTarget> = selection
                    .map(|selection| selection.ids.iter().map(|id| InteractionTarget { granularity: selection.granularity.clone(), id: id.clone() }).collect())
                    .unwrap_or_default();
                let interaction_writes = if targets.is_empty() { Vec::new() } else { vec![InteractionWrite { domain: PUZZLE5D_INTERACTION_DOMAIN.into(), targets, merge: MergeMode::Subtractive }] };
                Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit {
                    artifact_mutations: std::mem::take(&mut self.mutations),
                    config_mutations: config.mutations_to(&Puzzle5dConfig::default()),
                    interaction_writes,
                    ui_scope: UiDirtyScope::Full,
                    ..Default::default()
                }))
            }
            Puzzle5dSetActiveExampleStage::Complete => Err(Fault::from("puzzle5d-set-active-example-complete-repolled")),
            Puzzle5dSetActiveExampleStage::Closing => Err(Fault::from("puzzle5d-set-active-example-closing")),
        }
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle5dSetActiveExampleStage::Closing;
        self.close_owners.stage(Puzzle5dSetActiveExampleWorkOwners { mutations: std::mem::take(&mut self.mutations), before: std::mem::take(&mut self.before) });
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
        self.close_owners.close_step(grant)
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.stage == Puzzle5dSetActiveExampleStage::Closing && self.close_owners.is_empty()
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }
}

/// 📏️ The retained command route's own wire-admission band, widened off the shared 8 KiB/512 puzzle
/// default to puzzle 3d's (`PUZZLE3D_IMPORT_RAW_BYTES`/`PUZZLE3D_IMPORT_DECODED_ITEMS`): one whole `importSnapshot`
/// file, escaped, plus its envelope (`PUZZLE_IMPORT_RAW_BYTES`) — and a Nakagin-sized document's camera/grid/sun
/// publications, which the narrow band rejects before the job ever admits.
pub(super) const PUZZLE5D_RETAINED_RAW_BYTES: usize = crate::retained_command::PUZZLE_IMPORT_RAW_BYTES;
pub(super) const PUZZLE5D_RETAINED_DECODED_ITEMS: usize = 16_384;

/// 🔢️ Mesh numbers one interactive step of a `registerBrushMesh` page transfers. A page is bounded by the
/// retained wire grant, so a whole stream costs a fixed, small number of steps and no step approaches the
/// lane's own time budget.
const PUZZLE5D_MESH_PAGE_VALUES: usize = 512;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle5dRegisterBrushMeshStage {
    Positions,
    Indices,
    Derive,
    Complete,
    Closing,
}

/// 📋️ `registerBrushMesh` as real retained Work: the browser's GLB round-trip arrives as two flat number
/// arrays, transferred here one bounded page per step into puzzle 3d's process-wide mesh store (which the
/// brush suggestions and fill runs read). HostOnly — that store is neither document nor config state — but
/// the transfer itself is real, cursored work, not the `Emit::default()` a `BoundedFirstStepCommandWork`
/// would have produced in one unbounded turn.
/// ♻️ The owners one `Puzzle5dRegisterBrushMeshWork` still holds when its job closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
struct Puzzle5dRegisterBrushMeshWorkOwners {
    positions: Vec<f32>,
    indices: Vec<u32>,
}

pub(super) struct Puzzle5dRegisterBrushMeshWork {
    pub(super) stage: Puzzle5dRegisterBrushMeshStage,
    position_cursor: usize,
    index_cursor: usize,
    positions: Vec<f32>,
    indices: Vec<u32>,
    pub(super) close_owners: crate::puzzle_job::WorkClosing<Puzzle5dRegisterBrushMeshWorkOwners>,
}

impl Default for Puzzle5dRegisterBrushMeshWork {
    fn default() -> Self {
        Self { close_owners: Default::default(), stage: Puzzle5dRegisterBrushMeshStage::Positions, position_cursor: 0, index_cursor: 0, positions: Vec::new(), indices: Vec::new() }
    }
}

impl Puzzle5dRegisterBrushMeshWork {
    pub(super) fn array<'a>(command: &'a Puzzle5dCommand, key: &str) -> &'a [Value] {
        command.args().and_then(|args| args.get(key)).and_then(Value::as_array).map(Vec::as_slice).unwrap_or_default()
    }

    pub(super) fn pages(command: &Puzzle5dCommand, key: &str) -> usize {
        Self::array(command, key).len().div_ceil(PUZZLE5D_MESH_PAGE_VALUES)
    }
}

impl semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<Puzzle5dPlayApp>> for Puzzle5dRegisterBrushMeshWork {
    fn tool_id(&self) -> &'static str {
        "registerBrushMesh"
    }

    fn extent(&self, command: &Puzzle5dCommand, _snapshot: &Puzzle5dPlaySnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Puzzle5dPlayApp>>>) -> Option<usize> {
        let items = Self::pages(command, "positions").checked_add(Self::pages(command, "indices"))?.checked_add(3)?;
        (items <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS).then_some(items)
    }

    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Puzzle5dPlayApp>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        crate::retained_command::step_demands::<EditorApp<Puzzle5dPlayApp>>(size_of::<Self>())
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Puzzle5dPlayApp>>, _cx: &mut StepContext<'_>) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<Puzzle5dPlayApp>>, Fault> {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command, snapshot: _snapshot, config: _config, interaction: _interaction, hover: _hover, context, .. } = *input;
        let view_state = context.and_then(|context| context.view_state.as_ref());
        match self.stage {
            Puzzle5dRegisterBrushMeshStage::Positions => {
                let page = Self::array(command, "positions");
                let end = page.len().min(self.position_cursor.saturating_add(PUZZLE5D_MESH_PAGE_VALUES));
                self.positions.extend(page[self.position_cursor..end].iter().filter_map(|value| value.as_f64().map(|number| number as f32)));
                self.position_cursor = end;
                if end >= page.len() {
                    self.stage = Puzzle5dRegisterBrushMeshStage::Indices;
                }
                Ok(crate::puzzle_progress_step!("puzzle5d-register-brush-mesh-position", "Reading mesh positions", "Mesh-Positionen werden gelesen"))
            }
            Puzzle5dRegisterBrushMeshStage::Indices => {
                let page = Self::array(command, "indices");
                let end = page.len().min(self.index_cursor.saturating_add(PUZZLE5D_MESH_PAGE_VALUES));
                self.indices.extend(page[self.index_cursor..end].iter().filter_map(|value| value.as_u64().and_then(|number| u32::try_from(number).ok())));
                self.index_cursor = end;
                if end >= page.len() {
                    self.stage = Puzzle5dRegisterBrushMeshStage::Derive;
                }
                Ok(crate::puzzle_progress_step!("puzzle5d-register-brush-mesh-index", "Reading mesh indices", "Mesh-Indizes werden gelesen"))
            }
            Puzzle5dRegisterBrushMeshStage::Derive => {
                self.stage = Puzzle5dRegisterBrushMeshStage::Complete;
                let url = command.args().and_then(|args| args.get("url")).and_then(Value::as_str).unwrap_or_default().to_string();
                let positions = std::mem::take(&mut self.positions);
                let indices = std::mem::take(&mut self.indices);
                if !register_brush_mesh::puzzle5d_install_brush_mesh(&url, &positions, &indices) {
                    return Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(puzzle5d_notice_emit(view_state, |labels| labels.brush_reason_pose_unavailable.as_str())));
                }
                Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit { ui_scope: UiDirtyScope::None, ..Default::default() }))
            }
            Puzzle5dRegisterBrushMeshStage::Complete => Err(Fault::from("puzzle5d-register-brush-mesh-complete-repolled")),
            Puzzle5dRegisterBrushMeshStage::Closing => Err(Fault::from("puzzle5d-register-brush-mesh-closing")),
        }
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle5dRegisterBrushMeshStage::Closing;
        self.close_owners.stage(Puzzle5dRegisterBrushMeshWorkOwners { positions: std::mem::take(&mut self.positions), indices: std::mem::take(&mut self.indices) });
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
        self.close_owners.close_step(grant)
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.stage == Puzzle5dRegisterBrushMeshStage::Closing && self.close_owners.is_empty()
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }
}
//#endregion 🧵️RetainedCommands

//#region 📬️StorePreparation
#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub(super) struct Puzzle5dStorePreparationFactory;

struct Puzzle5dStorePreparation {
    pub(super) owners: store::OneItemOwners<Puzzle5dPlaySnapshot, Puzzle5dMutation>,
    pub(super) checkpoint: store::os_store::one_item::ArtifactStoreOneItemCheckpoint,
    completed_bytes: usize,
    pub(super) phase: u8,
    cancelled: bool,
}

impl store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Puzzle5dPlaySnapshot, Puzzle5dMutation> for Puzzle5dStorePreparationFactory {
    fn begin_batch_digest(
        &self,
        edit: &mut Option<Box<protocol::Edit<Puzzle5dMutation>>>,
        grant: semio_framework_value::retained_clone::RetainedCloneGrant,
    ) -> Result<Option<(Box<dyn store::ArtifactStoreBatchDigest<Puzzle5dMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress)>, semio_framework_value::ValueError> {
        store::admit_artifact_batch_digest(edit, grant)
    }

    /// 🧾️ The forward row plus the inverse rows the leaf's payload schema declares (`x-semio-inverse-rows`): a selection
    /// leaf one setter per changed pose field of each target, a removal the record and the fasteners it severs.
    fn preflight(&self, mutation: &Puzzle5dMutation, lane: store::os_store::store_schema::HistoryLane) -> Result<store::os_store::one_item::ArtifactStoreOneItemFootprint, String> {
        if lane != store::os_store::store_schema::HistoryLane::Document {
            return Err("Puzzle5d Store preparation rejected its lane".into());
        }
        Ok(store::os_store::one_item::ArtifactStoreOneItemFootprint::for_leaf::<Puzzle5dPlaySnapshot, _>(mutation, store::os_store::one_item::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES))
    }

    fn begin_demand(&self, _mutation: &Puzzle5dMutation, _lane: store::os_store::store_schema::HistoryLane) -> Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand, semio_framework_value::ValueError> {
        Ok(semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes: size_of::<Puzzle5dStorePreparation>(), depth: 1 })
    }

    fn begin(
        &self,
        request: store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<Puzzle5dPlaySnapshot, Puzzle5dMutation, Puzzle5dMutation>,
        grant: store::os_store::one_item::ArtifactStoreOneItemGrant,
    ) -> Result<(Box<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<Puzzle5dPlaySnapshot, Puzzle5dMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress), (semio_framework_value::ValueError, store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<Puzzle5dPlaySnapshot, Puzzle5dMutation, Puzzle5dMutation>)> {
        if request.lane != store::os_store::store_schema::HistoryLane::Document || request.operation != request.authority.operation() || request.generation != request.authority.generation() || request.base_revision != request.authority.base_revision() || request.authority.actor().len() > store::os_store::one_item::ARTIFACT_STORE_ONE_ITEM_ID_BYTES {
            return Err((semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "Puzzle5d preparation rejected its original publication authority"), request));
        }
        let demand = match self.begin_demand(&request.mutation, request.lane) {
            Ok(demand) => demand,
            Err(error) => return Err((error, request)),
        };
        let progress = match demand.admit(grant.retained_grant()) {
            Ok(progress) => progress,
            Err(error) => return Err((error, request)),
        };
        Ok((Box::new(Puzzle5dStorePreparation { owners: store::OneItemOwners::from_request(request), checkpoint: store::os_store::one_item::ArtifactStoreOneItemCheckpoint::default(), completed_bytes: 0, phase: 0, cancelled: false }), progress))
    }
}

impl store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<Puzzle5dPlaySnapshot, Puzzle5dMutation> for Puzzle5dStorePreparation {
    fn advance(&mut self, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<store::os_store::one_item::ArtifactStoreOneItemPreparationStep, semio_framework_value::ValueError> {
        use protocol::Mutation as _;
        use semio_framework_value::{retained_clone::RetainedCloneProgress, ValueError, ValueRefusalKind};
        if !grant.permits_one() || self.cancelled || self.owners.is_closing() {
            return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.owners.refused.is_some() {
            return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "Puzzle5d preparation retains its original Store refusal"));
        }
        if self.owners.prepared.is_some() || self.phase >= 2 {
            return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, RetainedCloneProgress::default()));
        }
        match self.phase {
            0 => {
                let base = self.owners.base.as_ref().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "Puzzle5d preparation lost its exact base root"))?;
                let mutation = self.owners.mutation.as_ref().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "Puzzle5d preparation lost its mutation owner"))?;
                let inverse = mutation.inverse(base.get())?;
                let post = protocol::apply_diff(mutation.diff(base.get()).diff(), base.get()).map_err(|_| ValueError::literal(ValueRefusalKind::InvalidValue, "Puzzle5d mutation could not produce its post root"))?;
                let mutation = self.owners.mutation.take().expect("observed original mutation owner");
                *self.owners.candidate = Some((post, inverse, mutation));
                self.completed_bytes = 1;
                self.phase = 1;
                self.checkpoint = store::os_store::one_item::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: self.completed_bytes as u64, digest: [0; 32] };
                Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint, RetainedCloneProgress::default()))
            }
            1 => {
                let Some(authority) = self.owners.authority.as_ref() else {
                    return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "Puzzle5d preparation lost its Store authority"));
                };
                let (post, inverse, mutation) = self.owners.candidate.take().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "Puzzle5d preparation lost its semantic candidate"))?;
                let prepared = match authority.prepare_one_item(authority.next_edit(mutation, inverse), std::sync::Arc::new(post)) {
                    Ok(prepared) => prepared,
                    Err((error, edit, post)) => {
                        *self.owners.refused = Some((edit, post));
                        return Err(error);
                    }
                };
                self.phase = 2;
                self.checkpoint = store::os_store::one_item::ArtifactStoreOneItemCheckpoint { cursor: 2, completed_items: 2, completed_bytes: self.completed_bytes as u64, digest: prepared.edit_digest() };
                *self.owners.prepared = Some(prepared);
                Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, RetainedCloneProgress::default()))
            }
            _ => Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, RetainedCloneProgress::default())),
        }
    }

    fn checkpoint(&self) -> store::os_store::one_item::ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }

    fn prepared(&self) -> Option<&store::os_store::one_item_publication::ArtifactStoreOneItemPrepared<Puzzle5dPlaySnapshot, Puzzle5dMutation>> {
        self.owners.prepared.as_ref()
    }

    fn take_prepared(&mut self) -> Option<store::os_store::one_item_publication::ArtifactStoreOneItemPrepared<Puzzle5dPlaySnapshot, Puzzle5dMutation>> {
        self.owners.prepared.take()
    }

    fn cancel(&mut self) {
        self.cancelled = true;
    }

    fn begin_close(&mut self) {
        self.owners.begin_close();
    }

    fn close_step(&mut self, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value::ValueError> {
        self.owners.close_step(grant.retained_grant())
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.owners.close_demands(0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.owners.close_demands(maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.owners.close_demands(0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.owners.close_demands(0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.owners.terminal_is_empty()
    }
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub(super) struct Puzzle5dConfigStorePreparationFactory;

struct Puzzle5dConfigStorePreparation {
    pub(super) owners: store::OneItemOwners<Puzzle5dConfig, Puzzle5dConfigMutation>,
    pub(super) checkpoint: store::os_store::one_item::ArtifactStoreOneItemCheckpoint,
    cancelled: bool,
}

impl store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Puzzle5dConfig, Puzzle5dConfigMutation> for Puzzle5dConfigStorePreparationFactory {
    fn begin_batch_digest(
        &self,
        edit: &mut Option<Box<protocol::Edit<Puzzle5dConfigMutation>>>,
        grant: semio_framework_value::retained_clone::RetainedCloneGrant,
    ) -> Result<Option<(Box<dyn store::ArtifactStoreBatchDigest<Puzzle5dConfigMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress)>, semio_framework_value::ValueError> {
        store::admit_artifact_batch_digest(edit, grant)
    }

    fn preflight(&self, mutation: &Puzzle5dConfigMutation, lane: store::os_store::store_schema::HistoryLane) -> Result<store::os_store::one_item::ArtifactStoreOneItemFootprint, String> {
        if lane != store::os_store::store_schema::HistoryLane::Document {
            return Err("Puzzle5d config Store preparation rejected its lane".into());
        }
        Ok(store::os_store::one_item::ArtifactStoreOneItemFootprint::for_leaf(mutation, store::os_store::one_item::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES))
    }

    fn begin_demand(&self, _mutation: &Puzzle5dConfigMutation, _lane: store::os_store::store_schema::HistoryLane) -> Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand, semio_framework_value::ValueError> {
        Ok(semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes: size_of::<Puzzle5dConfigStorePreparation>(), depth: 1 })
    }

    fn begin(
        &self,
        request: store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<Puzzle5dConfig, Puzzle5dConfigMutation, Puzzle5dConfigMutation>,
        grant: store::os_store::one_item::ArtifactStoreOneItemGrant,
    ) -> Result<(Box<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<Puzzle5dConfig, Puzzle5dConfigMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress), (semio_framework_value::ValueError, store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<Puzzle5dConfig, Puzzle5dConfigMutation, Puzzle5dConfigMutation>)> {
        if request.lane != store::os_store::store_schema::HistoryLane::Document || request.operation != request.authority.operation() || request.generation != request.authority.generation() || request.base_revision != request.authority.base_revision() {
            return Err((semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "Puzzle5d config preparation rejected its original publication authority"), request));
        }
        let demand = match self.begin_demand(&request.mutation, request.lane) {
            Ok(demand) => demand,
            Err(error) => return Err((error, request)),
        };
        let progress = match demand.admit(grant.retained_grant()) {
            Ok(progress) => progress,
            Err(error) => return Err((error, request)),
        };
        Ok((Box::new(Puzzle5dConfigStorePreparation { owners: store::OneItemOwners::from_request(request), checkpoint: store::os_store::one_item::ArtifactStoreOneItemCheckpoint::default(), cancelled: false }), progress))
    }
}

impl store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<Puzzle5dConfig, Puzzle5dConfigMutation> for Puzzle5dConfigStorePreparation {
    fn advance(&mut self, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<store::os_store::one_item::ArtifactStoreOneItemPreparationStep, semio_framework_value::ValueError> {
        use protocol::Mutation as _;
        use semio_framework_value::{retained_clone::RetainedCloneProgress, ValueError, ValueRefusalKind};
        if !grant.permits_one() || self.cancelled || self.owners.is_closing() {
            return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.owners.refused.is_some() {
            return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "Puzzle5d config preparation retains its original Store refusal"));
        }
        if self.owners.prepared.is_some() {
            return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, RetainedCloneProgress::default()));
        }
        let base = self.owners.base.as_ref().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "Puzzle5d config preparation lost its exact base root"))?;
        let mutation = self.owners.mutation.as_ref().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "Puzzle5d config preparation lost its mutation owner"))?;
        let inverse = mutation.inverse(base.get())?;
        let post = protocol::apply_diff(mutation.diff(base.get()).diff(), base.get()).map_err(|_| ValueError::literal(ValueRefusalKind::InvalidValue, "Puzzle5d config mutation could not produce its post root"))?;
        let Some(authority) = self.owners.authority.as_ref() else {
            return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "Puzzle5d config preparation lost its Store authority"));
        };
        let forward = self.owners.mutation.take().expect("observed original mutation owner");
        let edit = authority.next_edit(forward, inverse);
        let prepared = match authority.prepare_one_item(edit, std::sync::Arc::new(post)) {
            Ok(prepared) => prepared,
            Err((error, edit, post)) => {
                *self.owners.refused = Some((edit, post));
                return Err(error);
            }
        };
        self.checkpoint = store::os_store::one_item::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: 1, digest: prepared.edit_digest() };
        *self.owners.prepared = Some(prepared);
        Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, RetainedCloneProgress::default()))
    }

    fn checkpoint(&self) -> store::os_store::one_item::ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }

    fn prepared(&self) -> Option<&store::os_store::one_item_publication::ArtifactStoreOneItemPrepared<Puzzle5dConfig, Puzzle5dConfigMutation>> {
        self.owners.prepared.as_ref()
    }

    fn take_prepared(&mut self) -> Option<store::os_store::one_item_publication::ArtifactStoreOneItemPrepared<Puzzle5dConfig, Puzzle5dConfigMutation>> {
        self.owners.prepared.take()
    }

    fn cancel(&mut self) {
        self.cancelled = true;
    }

    fn begin_close(&mut self) {
        self.owners.begin_close();
    }

    fn close_step(&mut self, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value::ValueError> {
        self.owners.close_step(grant.retained_grant())
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.owners.close_demands(0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.owners.close_demands(maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.owners.close_demands(0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.owners.close_demands(0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.owners.terminal_is_empty()
    }
}
//#endregion 📬️StorePreparation
//#endregion 🔖️PlayApp
