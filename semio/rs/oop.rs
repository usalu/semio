// Object-oriented kit API: instance-centric methods, entity `diff_from` math, KitClient session.
// Loaded after core model + diff helpers so `get_guid_collection_diff` and apply fns resolve.

use std::collections::HashSet;

use super::*;

// ——— Geometry aliases (diagram: Point, Coordinate, Offset)

/// Diagram `Point` — 3D placement; same wire shape as [`Vector`].
pub type Point = Vector;

/// Diagram `Coordinate` — piece layout in 2D parameter space; same as [`Coord`].
pub type Coordinate = Coord;

/// Translation in u/v space for [`Coordinate::translate`].
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Offset {
    pub du: f64,
    pub dv: f64,
}

impl Offset {
    pub fn new(du: f64, dv: f64) -> Self {
        Self { du, dv }
    }

    pub fn invert(&self) -> Self {
        Self {
            du: -self.du,
            dv: -self.dv,
        }
    }
}

impl Coord {
    pub fn translate(&self, offset: &Offset) -> Self {
        Self {
            u: self.u + offset.du,
            v: self.v + offset.dv,
        }
    }
}

impl Vector {
    fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let len = self.length();
        if len <= 1e-15 || !len.is_finite() {
            return Vector::zero();
        }
        Self {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
    }

    pub fn scale(&self, factor: f64) -> Self {
        Self {
            x: self.x * factor,
            y: self.y * factor,
            z: self.z * factor,
        }
    }

    pub fn translate_as_point(&self, vector: &Vector) -> Self {
        Self {
            x: self.x + vector.x,
            y: self.y + vector.y,
            z: self.z + vector.z,
        }
    }
}

impl Plane {
    pub fn move_to(&mut self, origin: Point) {
        self.origin = origin;
    }

    pub fn rotate(&mut self, x_axis: Vector, y_axis: Vector) {
        self.x_axis = x_axis;
        self.y_axis = y_axis;
    }
}

// ——— Traits (diagram Entity / Actor)

pub trait Entity: HasGuid {
    fn entity_name(&self) -> &str;
    fn update_description_str(&self, kit: &mut Kit, description: &str) -> Result<()>;
}

impl Entity for Concept {
    fn entity_name(&self) -> &str {
        &self.name
    }

    fn update_description_str(&self, kit: &mut Kit, description: &str) -> Result<()> {
        self.update_description(kit, description)
    }
}

impl Entity for Tag {
    fn entity_name(&self) -> &str {
        &self.name
    }

    fn update_description_str(&self, kit: &mut Kit, description: &str) -> Result<()> {
        self.update_description(kit, description)
    }
}

impl Entity for Quality {
    fn entity_name(&self) -> &str {
        &self.name
    }

    fn update_description_str(&self, kit: &mut Kit, description: &str) -> Result<()> {
        self.update_description(kit, description)
    }
}

impl Entity for Type {
    fn entity_name(&self) -> &str {
        &self.name
    }

    fn update_description_str(&self, kit: &mut Kit, description: &str) -> Result<()> {
        self.update_description(kit, description)
    }
}

impl Entity for Design {
    fn entity_name(&self) -> &str {
        &self.name
    }

    fn update_description_str(&self, kit: &mut Kit, description: &str) -> Result<()> {
        self.update_description(kit, description)
    }
}

pub trait Actor {
    fn get_name(&self) -> &str;
    fn get_email(&self) -> &str;
    fn get_color(&self) -> &str;
}

/// Interactive human actor (session starter).
#[derive(Debug, Clone)]
pub struct User {
    pub name: String,
    pub email: String,
    pub color: String,
}

impl Actor for User {
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_email(&self) -> &str {
        &self.email
    }
    fn get_color(&self) -> &str {
        &self.color
    }
}

impl User {
    pub fn start_session(&self, _timeout_seconds: f64) {
        let _ = (_timeout_seconds, self);
    }
}

/// Automated actor executing structured kit commands.
#[derive(Debug, Clone)]
pub struct Agent {
    pub name: String,
    pub email: String,
    pub color: String,
}

impl Actor for Agent {
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_email(&self) -> &str {
        &self.email
    }
    fn get_color(&self) -> &str {
        &self.color
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KitCommandKind {
    Query,
    Mutate,
}

impl Agent {
    pub fn execute<E: Entity>(&self, _command_kind: KitCommandKind, target: &E) {
        let _ = (self, target);
    }
}

// ——— Polymorphic kit entities for selection (diagram: no bare ids in API)

#[derive(Debug, Clone)]
pub enum KitEntity {
    Tag(Tag),
    Concept(Concept),
    Port(Port),
    Quality(Quality),
    Type(Type),
    Design(Design),
    File(File),
    Folder(Folder),
    Author(Author),
    Attribute(Attribute),
    Model(Model),
    Connector(Connector),
    Prop(Prop),
    Layer(Layer),
    Group(Group),
    Piece(Piece),
    Connection(Connection),
    Stat(Stat),
    Benchmark(Benchmark),
}

impl HasGuid for KitEntity {
    fn guid(&self) -> &str {
        match self {
            KitEntity::Tag(e) => e.guid(),
            KitEntity::Concept(e) => e.guid(),
            KitEntity::Port(e) => e.guid(),
            KitEntity::Quality(e) => e.guid(),
            KitEntity::Type(e) => e.guid(),
            KitEntity::Design(e) => e.guid(),
            KitEntity::File(e) => e.guid(),
            KitEntity::Folder(e) => e.guid(),
            KitEntity::Author(e) => e.guid(),
            KitEntity::Attribute(e) => e.guid(),
            KitEntity::Model(e) => e.guid(),
            KitEntity::Connector(e) => e.guid(),
            KitEntity::Prop(e) => e.guid(),
            KitEntity::Layer(e) => e.guid(),
            KitEntity::Group(e) => e.guid(),
            KitEntity::Piece(e) => e.guid(),
            KitEntity::Connection(e) => e.guid(),
            KitEntity::Stat(e) => e.guid(),
            KitEntity::Benchmark(e) => e.guid(),
        }
    }
}

// ——— KitOperation & KitClient

/// Design-level edit scoped selection + undo stack for operations within a [`KitClient`] session.
pub struct KitOperation {
    pub selection: Vec<KitEntity>,
    undo_stack: Vec<KitGraphChange>,
    redo_stack: Vec<KitGraphChange>,
}

impl Default for KitOperation {
    fn default() -> Self {
        Self {
            selection: vec![],
            undo_stack: vec![],
            redo_stack: vec![],
        }
    }
}

impl KitOperation {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_to_selection(&mut self, entity: KitEntity) {
        let g = entity.guid().to_string();
        if !self.selection.iter().any(|e| e.guid() == g) {
            self.selection.push(entity);
        }
    }

    pub fn add_many_to_selection(&mut self, entities: impl IntoIterator<Item = KitEntity>) {
        for e in entities {
            self.add_to_selection(e);
        }
    }

    pub fn remove_from_selection(&mut self, entity: &KitEntity) {
        let g = entity.guid();
        self.selection.retain(|e| e.guid() != g);
    }

    pub fn remove_many_from_selection(&mut self, entities: &[KitEntity]) {
        for e in entities {
            self.remove_from_selection(e);
        }
    }

    pub fn clear_selection(&mut self) {
        self.selection.clear();
    }

    pub fn record_change(&mut self, change: KitGraphChange) {
        self.undo_stack.push(change);
        self.redo_stack.clear();
    }

    pub fn undo(&mut self, kit: &mut Kit) -> bool {
        let Some(ch) = self.undo_stack.pop() else {
            return false;
        };
        apply_kit_diff(kit, &ch.backward);
        self.redo_stack.push(ch);
        true
    }

    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    pub fn redo(&mut self, kit: &mut Kit) -> bool {
        let Some(ch) = self.redo_stack.pop() else {
            return false;
        };
        apply_kit_diff(kit, &ch.forward);
        self.undo_stack.push(ch);
        true
    }

    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }
}

pub struct KitClient<A: Actor> {
    pub actor: A,
    pub session: KitGraphSession,
    pub operations: Vec<KitOperation>,
    active_index: Option<usize>,
    timeout_seconds: f64,
}

impl<A: Actor> KitClient<A> {
    pub fn new(kit: Kit, actor: A) -> Self {
        Self {
            actor,
            session: KitGraphSession::new(kit),
            operations: vec![],
            active_index: None,
            timeout_seconds: 0.0,
        }
    }

    pub fn start_session(&mut self, timeout_seconds: f64) {
        self.timeout_seconds = timeout_seconds;
        let _ = self.session.start_transaction();
    }

    pub fn end_session(&mut self) {
        self.timeout_seconds = 0.0;
        self.active_index = None;
    }

    pub fn map_kit<T>(&self, f: impl FnOnce(&Kit) -> T) -> Result<T> {
        self.session.map_kit(f)
    }

    pub fn map_kit_mut<T>(&self, f: impl FnOnce(&mut Kit) -> T) -> Result<T> {
        self.session.map_kit_mut(f)
    }

    pub fn commit(&self, diff: KitDiff, opts: KitCommitOptions) -> Result<KitGraphChange> {
        self.session.commit(diff, opts)
    }

    pub fn undo(&self) -> Result<()> {
        self.session.undo_history()
    }

    pub fn can_undo(&self) -> Result<bool> {
        Ok(self.session.map_kit(|_| ())?.is_ok()) // session doesn't expose depth — keep API surface
    }

    pub fn redo(&self) -> Result<()> {
        self.session.redo_history()
    }

    pub fn can_redo(&self) -> Result<bool> {
        Ok(true)
    }

    pub fn start_new_operation(&mut self) {
        self.operations.push(KitOperation::new());
        self.active_index = Some(self.operations.len() - 1);
    }

    pub fn set_active_operation(&mut self, operation: KitOperation) {
        if let Some(i) = self.active_index {
            if i < self.operations.len() {
                self.operations[i] = operation;
                return;
            }
        }
        self.operations.push(operation);
        self.active_index = Some(self.operations.len() - 1);
    }

    pub fn submit_operation(&mut self, operation: KitOperation) {
        self.operations.push(operation);
    }

    pub fn submit_active_operation(&mut self) {
        self.active_index = None;
    }

    pub fn submit_all_operations(&mut self) {
        self.operations.clear();
        self.active_index = None;
    }

    pub fn cancel_operation(&mut self, _operation: &KitOperation) {
        // Caller retains ownership of passed-in op; drop local copy if matched — simplified clear-active.
        self.active_index = None;
    }

    pub fn cancel_active_operation(&mut self) {
        self.active_index = None;
    }

    pub fn cancel_all_operations(&mut self) {
        self.operations.clear();
        self.active_index = None;
    }

    /// Mutable kit reference through session for applying entity-level APIs.
    pub fn with_kit_mut<T>(&self, f: impl FnOnce(&mut Kit) -> T) -> Result<T> {
        self.session.map_kit_mut(f)
    }
}

// ——— Diff-from (entity-local diff math; used by `Kit::diff_from` / `Design::diff_from`)

impl Attribute {
    pub(crate) fn diff_from(&self, after: &Attribute) -> AttributeDiff {
        let mut diff = AttributeDiff {
            guid: self.guid.clone(),
            ..Default::default()
        };
        if self.key != after.key {
            diff.key = Some(after.key.clone());
        }
        if self.value != after.value {
            diff.value = Some(after.value.clone());
        }
        if self.definition != after.definition {
            diff.definition = Some(after.definition.clone());
        }
        diff
    }
}

impl Prop {
    pub(crate) fn diff_from(&self, after: &Prop) -> PropDiff {
        let mut diff = PropDiff {
            guid: self.guid.clone(),
            ..Default::default()
        };
        if self.quality != after.quality {
            diff.quality = Some(after.quality.clone());
        }
        if self.value != after.value {
            diff.value = Some(after.value.clone());
        }
        if self.unit != after.unit {
            diff.unit = Some(after.unit.clone());
        }
        diff.attributes = get_guid_collection_diff(
            &self.attributes,
            &after.attributes,
            "attribute",
            |b, a| b.diff_from(a),
        );
        diff
    }
}

impl Connector {
    pub(crate) fn diff_from(&self, after: &Connector) -> ConnectorDiff {
        let mut diff = ConnectorDiff {
            guid: self.guid.clone(),
            ..Default::default()
        };
        if self.point != after.point {
            diff.point = Some(Vector {
                x: after.point.x - self.point.x,
                y: after.point.y - self.point.y,
                z: after.point.z - self.point.z,
            });
        }
        if self.direction != after.direction {
            diff.direction = Some(Vector {
                x: after.direction.x - self.direction.x,
                y: after.direction.y - self.direction.y,
                z: after.direction.z - self.direction.z,
            });
        }
        if self.t != after.t {
            diff.t = Some(after.t);
        }
        if self.name != after.name {
            diff.name = Some(after.name.clone());
        }
        if self.description != after.description {
            diff.description = Some(after.description.clone());
        }
        if self.mandatory != after.mandatory {
            diff.mandatory = Some(after.mandatory);
        }
        if self.port != after.port {
            diff.port = Some(after.port.clone());
        }
        diff.props =
            get_guid_collection_diff(&self.props, &after.props, "prop", |b, a| b.diff_from(a));
        diff.attributes = get_guid_collection_diff(
            &self.attributes,
            &after.attributes,
            "attribute",
            |b, a| b.diff_from(a),
        );
        diff
    }
}

impl Model {
    pub(crate) fn diff_from(&self, after: &Model) -> ModelDiff {
        let mut diff = ModelDiff {
            guid: self.guid.clone(),
            ..Default::default()
        };
        if self.file != after.file {
            diff.file = Some(after.file.clone());
        }
        if self.name != after.name {
            diff.name = Some(after.name.clone());
        }
        if self.description != after.description {
            diff.description = Some(after.description.clone());
        }
        if self.tags != after.tags {
            diff.tags = Some(after.tags.clone());
        }
        diff.attributes = get_guid_collection_diff(
            &self.attributes,
            &after.attributes,
            "attribute",
            |b, a| b.diff_from(a),
        );
        diff
    }
}

impl Type {
    pub(crate) fn diff_from(&self, after: &Type) -> TypeDiff {
        let mut diff = TypeDiff {
            guid: self.guid.clone(),
            ..Default::default()
        };
        if self.name != after.name {
            diff.name = Some(after.name.clone());
        }
        if self.parent != after.parent {
            diff.parent = Some(after.parent.clone());
        }
        if self.description != after.description {
            diff.description = Some(after.description.clone());
        }
        if self.icon != after.icon {
            diff.icon = Some(after.icon.clone());
        }
        if self.image != after.image {
            diff.image = Some(after.image.clone());
        }
        if self.folder != after.folder {
            diff.folder = Some(after.folder.clone());
        }
        if self.unit != after.unit {
            diff.unit = Some(after.unit.clone());
        }
        if self.stock != after.stock {
            diff.stock = Some(after.stock);
        }
        if self.is_abstract != after.is_abstract {
            diff.is_abstract = Some(after.is_abstract);
        }
        if self.virtual_type != after.virtual_type {
            diff.virtual_type = Some(after.virtual_type);
        }
        if self.location != after.location {
            diff.location = Some(after.location.clone());
        }
        if self.concepts != after.concepts {
            diff.concepts = Some(after.concepts.clone());
        }
        if self.authors != after.authors {
            diff.authors = Some(after.authors.clone());
        }
        diff.props = get_guid_collection_diff(&self.props, &after.props, "prop", |b, a| {
            b.diff_from(a)
        });
        diff.models =
            get_guid_collection_diff(&self.models, &after.models, "model", |b, a| b.diff_from(a));
        diff.connectors = get_guid_collection_diff(
            &self.connectors,
            &after.connectors,
            "connector",
            |b, a| b.diff_from(a),
        );
        diff.attributes = get_guid_collection_diff(
            &self.attributes,
            &after.attributes,
            "attribute",
            |b, a| b.diff_from(a),
        );
        diff
    }
}

impl Piece {
    pub(crate) fn diff_from(&self, after: &Piece) -> PieceDiff {
        let mut diff = PieceDiff {
            guid: self.guid.clone(),
            ..Default::default()
        };
        if self.name != after.name {
            diff.name = Some(after.name.clone());
        }
        if self.type_ref != after.type_ref {
            diff.type_ref = Some(after.type_ref.clone());
        }
        if self.design != after.design {
            diff.design = Some(after.design.clone());
        }
        if self.plane != after.plane {
            diff.plane = Some(after.plane.clone());
        }
        if self.center != after.center {
            diff.center = Some(after.center.clone());
        }
        if self.scale != after.scale {
            diff.scale = Some(after.scale);
        }
        if self.mirror_plane != after.mirror_plane {
            diff.mirror_plane = Some(after.mirror_plane.clone());
        }
        if self.is_hidden != after.is_hidden {
            diff.is_hidden = Some(after.is_hidden);
        }
        if self.is_locked != after.is_locked {
            diff.is_locked = Some(after.is_locked);
        }
        if self.color != after.color {
            diff.color = Some(after.color.clone());
        }
        if self.description != after.description {
            diff.description = Some(after.description.clone());
        }
        diff.props =
            get_guid_collection_diff(&self.props, &after.props, "prop", |b, a| b.diff_from(a));
        diff.attributes = get_guid_collection_diff(
            &self.attributes,
            &after.attributes,
            "attribute",
            |b, a| b.diff_from(a),
        );
        diff
    }
}

impl Connection {
    pub(crate) fn diff_from(&self, after: &Connection) -> ConnectionDiff {
        let mut diff = ConnectionDiff {
            guid: self.guid.clone(),
            ..Default::default()
        };
        if self.connected != after.connected {
            let mut sd = SideDiff::default();
            if self.connected.piece != after.connected.piece {
                sd.piece = Some(after.connected.piece.clone());
            }
            if self.connected.design_piece != after.connected.design_piece {
                sd.design_piece = Some(after.connected.design_piece.clone());
            }
            if self.connected.connector != after.connected.connector {
                sd.connector = Some(after.connected.connector.clone());
            }
            diff.connected = Some(sd);
        }
        if self.connecting != after.connecting {
            let mut sd = SideDiff::default();
            if self.connecting.piece != after.connecting.piece {
                sd.piece = Some(after.connecting.piece.clone());
            }
            if self.connecting.design_piece != after.connecting.design_piece {
                sd.design_piece = Some(after.connecting.design_piece.clone());
            }
            if self.connecting.connector != after.connecting.connector {
                sd.connector = Some(after.connecting.connector.clone());
            }
            diff.connecting = Some(sd);
        }
        if self.gap != after.gap {
            diff.gap = Some(after.gap - self.gap);
        }
        if self.shift != after.shift {
            diff.shift = Some(after.shift - self.shift);
        }
        if self.rise != after.rise {
            diff.rise = Some(after.rise - self.rise);
        }
        if self.rotation != after.rotation {
            diff.rotation = Some(after.rotation - self.rotation);
        }
        if self.turn != after.turn {
            diff.turn = Some(after.turn - self.turn);
        }
        if self.tilt != after.tilt {
            diff.tilt = Some(after.tilt - self.tilt);
        }
        if self.u != after.u {
            diff.u = Some(match (self.u, after.u) {
                (Some(b), Some(a)) => Some(a - b),
                (None, Some(a)) => Some(a),
                (_, None) => None,
            });
        }
        if self.v != after.v {
            diff.v = Some(match (self.v, after.v) {
                (Some(b), Some(a)) => Some(a - b),
                (None, Some(a)) => Some(a),
                (_, None) => None,
            });
        }
        if self.description != after.description {
            diff.description = Some(after.description.clone());
        }
        diff.attributes = get_guid_collection_diff(
            &self.attributes,
            &after.attributes,
            "attribute",
            |b, a| b.diff_from(a),
        );
        diff
    }
}

impl Layer {
    pub(crate) fn diff_from(&self, after: &Layer) -> LayerDiff {
        let mut diff = LayerDiff {
            guid: self.guid.clone(),
            ..Default::default()
        };
        if self.path != after.path {
            diff.path = Some(after.path.clone());
        }
        if self.is_hidden != after.is_hidden {
            diff.is_hidden = Some(after.is_hidden);
        }
        if self.is_locked != after.is_locked {
            diff.is_locked = Some(after.is_locked);
        }
        if self.color != after.color {
            diff.color = Some(after.color.clone());
        }
        if self.description != after.description {
            diff.description = Some(after.description.clone());
        }
        diff.attributes = get_guid_collection_diff(
            &self.attributes,
            &after.attributes,
            "attribute",
            |b, a| b.diff_from(a),
        );
        diff
    }
}

impl Group {
    pub(crate) fn diff_from(&self, after: &Group) -> GroupDiff {
        let mut diff = GroupDiff {
            guid: self.guid.clone(),
            ..Default::default()
        };
        if self.name != after.name {
            diff.name = Some(after.name.clone());
        }
        if self.color != after.color {
            diff.color = Some(after.color.clone());
        }
        if self.description != after.description {
            diff.description = Some(after.description.clone());
        }
        if self.pieces != after.pieces {
            diff.pieces = Some(after.pieces.clone());
        }
        diff.attributes = get_guid_collection_diff(
            &self.attributes,
            &after.attributes,
            "attribute",
            |b, a| b.diff_from(a),
        );
        diff
    }
}

impl Stat {
    pub(crate) fn diff_from(&self, after: &Stat) -> StatDiff {
        let mut diff = StatDiff {
            guid: self.guid.clone(),
            ..Default::default()
        };
        if self.quality != after.quality {
            diff.quality = Some(after.quality.clone());
        }
        if self.min != after.min {
            diff.min = Some(after.min);
        }
        if self.min_excluded != after.min_excluded {
            diff.min_excluded = Some(after.min_excluded);
        }
        if self.max != after.max {
            diff.max = Some(after.max);
        }
        if self.max_excluded != after.max_excluded {
            diff.max_excluded = Some(after.max_excluded);
        }
        if self.unit != after.unit {
            diff.unit = Some(after.unit.clone());
        }
        diff
    }
}

impl Tag {
    pub(crate) fn diff_from(&self, after: &Tag) -> TagDiff {
        let mut diff = TagDiff {
            guid: self.guid.clone(),
            ..Default::default()
        };
        if self.name != after.name {
            diff.name = Some(after.name.clone());
        }
        if self.description != after.description {
            diff.description = Some(after.description.clone());
        }
        if self.icon != after.icon {
            diff.icon = Some(after.icon.clone());
        }
        diff
    }
}

impl Concept {
    pub(crate) fn diff_from(&self, after: &Concept) -> ConceptDiff {
        let mut diff = ConceptDiff {
            guid: self.guid.clone(),
            ..Default::default()
        };
        if self.name != after.name {
            diff.name = Some(after.name.clone());
        }
        if self.description != after.description {
            diff.description = Some(after.description.clone());
        }
        if self.icon != after.icon {
            diff.icon = Some(after.icon.clone());
        }
        diff
    }
}

impl Port {
    pub(crate) fn diff_from(&self, after: &Port) -> PortDiff {
        let mut diff = PortDiff {
            guid: self.guid.clone(),
            ..Default::default()
        };
        if self.name != after.name {
            diff.name = Some(after.name.clone());
        }
        if self.description != after.description {
            diff.description = Some(after.description.clone());
        }
        if self.icon != after.icon {
            diff.icon = Some(after.icon.clone());
        }
        if self.compatible_interfaces != after.compatible_interfaces {
            diff.compatible_interfaces = Some(after.compatible_interfaces.clone());
        }
        diff.attributes = get_guid_collection_diff(
            &self.attributes,
            &after.attributes,
            "attribute",
            |b, a| b.diff_from(a),
        );
        diff
    }
}

impl Quality {
    pub(crate) fn diff_from(&self, after: &Quality) -> QualityDiff {
        let mut diff = QualityDiff {
            guid: self.guid.clone(),
            ..Default::default()
        };
        if self.key != after.key {
            diff.key = Some(after.key.clone());
        }
        if self.name != after.name {
            diff.name = Some(after.name.clone());
        }
        if self.kind != after.kind {
            diff.kind = Some(after.kind.clone());
        }
        if self.default_value != after.default_value {
            diff.default_value = Some(after.default_value);
        }
        if self.formula != after.formula {
            diff.formula = Some(after.formula.clone());
        }
        if self.default_si_unit != after.default_si_unit {
            diff.default_si_unit = Some(after.default_si_unit.clone());
        }
        if self.default_imperial_unit != after.default_imperial_unit {
            diff.default_imperial_unit = Some(after.default_imperial_unit.clone());
        }
        if self.min != after.min {
            diff.min = Some(after.min);
        }
        if self.is_min_excluded != after.is_min_excluded {
            diff.is_min_excluded = Some(after.is_min_excluded);
        }
        if self.max != after.max {
            diff.max = Some(after.max);
        }
        if self.is_max_excluded != after.is_max_excluded {
            diff.is_max_excluded = Some(after.is_max_excluded);
        }
        if self.can_scale != after.can_scale {
            diff.can_scale = Some(after.can_scale);
        }
        if self.uri != after.uri {
            diff.uri = Some(after.uri.clone());
        }
        diff.attributes = get_guid_collection_diff(
            &self.attributes,
            &after.attributes,
            "attribute",
            |b, a| b.diff_from(a),
        );
        diff
    }
}

impl File {
    pub(crate) fn diff_from(&self, after: &File) -> FileDiff {
        let mut diff = FileDiff {
            guid: self.guid.clone(),
            ..Default::default()
        };
        if self.name != after.name {
            diff.name = Some(after.name.clone());
        }
        if self.remote != after.remote {
            diff.remote = Some(after.remote.clone());
        }
        if self.folder != after.folder {
            diff.folder = Some(after.folder.clone());
        }
        if self.size != after.size {
            diff.size = Some(after.size);
        }
        if self.hash != after.hash {
            diff.hash = Some(after.hash.clone());
        }
        diff
    }
}

impl Folder {
    pub(crate) fn diff_from(&self, after: &Folder) -> FolderDiff {
        let mut diff = FolderDiff {
            guid: self.guid.clone(),
            ..Default::default()
        };
        if self.name != after.name {
            diff.name = Some(after.name.clone());
        }
        if self.parent != after.parent {
            diff.parent = Some(after.parent.clone());
        }
        diff.attributes = get_guid_collection_diff(
            &self.attributes,
            &after.attributes,
            "attribute",
            |b, a| b.diff_from(a),
        );
        diff
    }
}

impl Author {
    pub(crate) fn diff_from(&self, after: &Author) -> AuthorDiff {
        let mut diff = AuthorDiff {
            guid: self.guid.clone(),
            ..Default::default()
        };
        if self.name != after.name {
            diff.name = Some(after.name.clone());
        }
        if self.email != after.email {
            diff.email = Some(after.email.clone());
        }
        diff.attributes = get_guid_collection_diff(
            &self.attributes,
            &after.attributes,
            "attribute",
            |b, a| b.diff_from(a),
        );
        diff
    }
}

impl Kit {
    /// Computes a structural diff from `self` to `after` (replaces `get_kit_diff` logic).
    pub fn diff_from(&self, after: &Kit) -> KitDiff {
        let mut diff = KitDiff {
            guid: self.guid.clone(),
            ..Default::default()
        };
        if self.name != after.name {
            diff.name = Some(after.name.clone());
        }
        if self.version != after.version {
            diff.version = Some(after.version.clone());
        }
        if self.description != after.description {
            diff.description = Some(after.description.clone());
        }
        if self.icon != after.icon {
            diff.icon = Some(after.icon.clone());
        }
        if self.image != after.image {
            diff.image = Some(after.image.clone());
        }
        if self.preview != after.preview {
            diff.preview = Some(after.preview.clone());
        }
        if self.remote != after.remote {
            diff.remote = Some(after.remote.clone());
        }
        if self.homepage != after.homepage {
            diff.homepage = Some(after.homepage.clone());
        }
        if self.license != after.license {
            diff.license = Some(after.license.clone());
        }
        diff.types =
            get_guid_collection_diff(&self.types, &after.types, "type", |b, a| b.diff_from(a));
        diff.designs =
            get_guid_collection_diff(&self.designs, &after.designs, "design", |b, a| {
                b.diff_from(a)
            });
        diff.tags =
            get_guid_collection_diff(&self.tags, &after.tags, "tag", |b, a| b.diff_from(a));
        diff.concepts =
            get_guid_collection_diff(&self.concepts, &after.concepts, "concept", |b, a| {
                b.diff_from(a)
            });
        diff.ports =
            get_guid_collection_diff(&self.ports, &after.ports, "port", |b, a| b.diff_from(a));
        diff.qualities = get_guid_collection_diff(
            &self.qualities,
            &after.qualities,
            "quality",
            |b, a| b.diff_from(a),
        );
        diff.files =
            get_guid_collection_diff(&self.files, &after.files, "file", |b, a| b.diff_from(a));
        diff.folders =
            get_guid_collection_diff(&self.folders, &after.folders, "folder", |b, a| {
                b.diff_from(a)
            });
        diff.authors =
            get_guid_collection_diff(&self.authors, &after.authors, "author", |b, a| {
                b.diff_from(a)
            });
        diff.attributes = get_guid_collection_diff(
            &self.attributes,
            &after.attributes,
            "attribute",
            |b, a| b.diff_from(a),
        );
        diff
    }

    pub fn find_type_entity<'a>(&'a self, t: &Type) -> Option<&'a Type> {
        self.types.as_ref()?.iter().find(|x| x.guid == t.guid)
    }

    pub fn find_design_entity<'a>(&'a self, d: &Design) -> Option<&'a Design> {
        self.designs.as_ref()?.iter().find(|x| x.guid == d.guid)
    }

    pub fn find_tag<'a>(&'a self, tag: &Tag) -> Option<&'a Tag> {
        self.tags.as_ref()?.iter().find(|x| x.guid == tag.guid)
    }

    pub fn find_concept<'a>(&'a self, c: &Concept) -> Option<&'a Concept> {
        self.concepts.as_ref()?.iter().find(|x| x.guid == c.guid)
    }

    pub fn find_port<'a>(&'a self, p: &Port) -> Option<&'a Port> {
        self.ports.as_ref()?.iter().find(|x| x.guid == p.guid)
    }

    pub fn find_quality<'a>(&'a self, q: &Quality) -> Option<&'a Quality> {
        self.qualities.as_ref()?.iter().find(|x| x.guid == q.guid)
    }

    pub fn find_file<'a>(&'a self, f: &File) -> Option<&'a File> {
        self.files.as_ref()?.iter().find(|x| x.guid == f.guid)
    }

    pub fn find_folder<'a>(&'a self, f: &Folder) -> Option<&'a Folder> {
        self.folders.as_ref()?.iter().find(|x| x.guid == f.guid)
    }

    pub fn find_author<'a>(&'a self, a: &Author) -> Option<&'a Author> {
        self.authors.as_ref()?.iter().find(|x| x.guid == a.guid)
    }

    pub fn create_tag(&mut self, tag: Tag) -> Result<()> {
        let v = self.tags.get_or_insert_with(Vec::new);
        if v.iter().any(|t| t.guid == tag.guid) {
            return Err(SemioError::InvalidOperation {
                message: "tag guid already exists".into(),
            });
        }
        v.push(tag);
        Ok(())
    }

    pub fn create_tags(&mut self, tags: Vec<Tag>) -> Result<()> {
        for t in tags {
            self.create_tag(t)?;
        }
        Ok(())
    }

    pub fn delete_tag(&mut self, tag: &Tag) -> Result<()> {
        let Some(v) = self.tags.as_mut() else {
            return Ok(());
        };
        v.retain(|t| t.guid != tag.guid);
        Ok(())
    }

    pub fn delete_tags(&mut self, tags: &[Tag]) -> Result<()> {
        for t in tags {
            self.delete_tag(t)?;
        }
        Ok(())
    }

    pub fn create_concept(&mut self, concept: Concept) -> Result<()> {
        let v = self.concepts.get_or_insert_with(Vec::new);
        if v.iter().any(|c| c.guid == concept.guid) {
            return Err(SemioError::InvalidOperation {
                message: "concept guid already exists".into(),
            });
        }
        v.push(concept);
        Ok(())
    }

    pub fn create_concepts(&mut self, concepts: Vec<Concept>) -> Result<()> {
        for c in concepts {
            self.create_concept(c)?;
        }
        Ok(())
    }

    pub fn delete_concept(&mut self, c: &Concept) -> Result<()> {
        let Some(v) = self.concepts.as_mut() else {
            return Ok(());
        };
        v.retain(|x| x.guid != c.guid);
        Ok(())
    }

    pub fn delete_concepts(&mut self, concepts: &[Concept]) -> Result<()> {
        for c in concepts {
            self.delete_concept(c)?;
        }
        Ok(())
    }

    pub fn create_port(&mut self, port: Port) -> Result<()> {
        let v = self.ports.get_or_insert_with(Vec::new);
        if v.iter().any(|p| p.guid == port.guid) {
            return Err(SemioError::InvalidOperation {
                message: "port guid already exists".into(),
            });
        }
        v.push(port);
        Ok(())
    }

    pub fn create_ports(&mut self, ports: Vec<Port>) -> Result<()> {
        for p in ports {
            self.create_port(p)?;
        }
        Ok(())
    }

    pub fn delete_port(&mut self, p: &Port) -> Result<()> {
        let Some(v) = self.ports.as_mut() else {
            return Ok(());
        };
        v.retain(|x| x.guid != p.guid);
        Ok(())
    }

    pub fn delete_ports(&mut self, ports: &[Port]) -> Result<()> {
        for p in ports {
            self.delete_port(p)?;
        }
        Ok(())
    }

    pub fn create_quality(&mut self, quality: Quality) -> Result<()> {
        let v = self.qualities.get_or_insert_with(Vec::new);
        if v.iter().any(|q| q.guid == quality.guid) {
            return Err(SemioError::InvalidOperation {
                message: "quality guid already exists".into(),
            });
        }
        v.push(quality);
        Ok(())
    }

    pub fn create_qualities(&mut self, qualities: Vec<Quality>) -> Result<()> {
        for q in qualities {
            self.create_quality(q)?;
        }
        Ok(())
    }

    pub fn delete_quality(&mut self, q: &Quality) -> Result<()> {
        let Some(v) = self.qualities.as_mut() else {
            return Ok(());
        };
        v.retain(|x| x.guid != q.guid);
        Ok(())
    }

    pub fn delete_qualities(&mut self, qualities: &[Quality]) -> Result<()> {
        for q in qualities {
            self.delete_quality(q)?;
        }
        Ok(())
    }

    pub fn create_type(&mut self, t: Type) -> Result<()> {
        let v = self.types.get_or_insert_with(Vec::new);
        if v.iter().any(|x| x.guid == t.guid) {
            return Err(SemioError::InvalidOperation {
                message: "type guid already exists".into(),
            });
        }
        v.push(t);
        Ok(())
    }

    pub fn create_types(&mut self, types: Vec<Type>) -> Result<()> {
        for t in types {
            self.create_type(t)?;
        }
        Ok(())
    }

    pub fn delete_type(&mut self, t: &Type) -> Result<()> {
        let Some(v) = self.types.as_mut() else {
            return Ok(());
        };
        v.retain(|x| x.guid != t.guid);
        Ok(())
    }

    pub fn delete_types(&mut self, types: &[Type]) -> Result<()> {
        for t in types {
            self.delete_type(t)?;
        }
        Ok(())
    }

    pub fn create_design(&mut self, d: Design) -> Result<()> {
        let v = self.designs.get_or_insert_with(Vec::new);
        if v.iter().any(|x| x.guid == d.guid) {
            return Err(SemioError::InvalidOperation {
                message: "design guid already exists".into(),
            });
        }
        v.push(d);
        Ok(())
    }

    pub fn create_designs(&mut self, designs: Vec<Design>) -> Result<()> {
        for d in designs {
            self.create_design(d)?;
        }
        Ok(())
    }

    pub fn delete_design(&mut self, d: &Design) -> Result<()> {
        let Some(v) = self.designs.as_mut() else {
            return Ok(());
        };
        v.retain(|x| x.guid != d.guid);
        Ok(())
    }

    pub fn delete_designs(&mut self, designs: &[Design]) -> Result<()> {
        for d in designs {
            self.delete_design(d)?;
        }
        Ok(())
    }
}

impl Design {
    pub fn diff_from(&self, after: &Design) -> DesignDiff {
        let mut diff = DesignDiff {
            guid: self.guid.clone(),
            ..Default::default()
        };
        if self.name != after.name {
            diff.name = Some(after.name.clone());
        }
        if self.parent != after.parent {
            diff.parent = Some(after.parent.clone());
        }
        if self.description != after.description {
            diff.description = Some(after.description.clone());
        }
        if self.icon != after.icon {
            diff.icon = Some(after.icon.clone());
        }
        if self.image != after.image {
            diff.image = Some(after.image.clone());
        }
        if self.folder != after.folder {
            diff.folder = Some(after.folder.clone());
        }
        if self.unit != after.unit {
            diff.unit = Some(after.unit.clone());
        }
        if self.is_abstract != after.is_abstract {
            diff.is_abstract = Some(after.is_abstract);
        }
        if self.can_scale != after.can_scale {
            diff.can_scale = Some(after.can_scale);
        }
        if self.can_mirror != after.can_mirror {
            diff.can_mirror = Some(after.can_mirror);
        }
        if self.concepts != after.concepts {
            diff.concepts = Some(after.concepts.clone());
        }
        if self.authors != after.authors {
            diff.authors = Some(after.authors.clone());
        }
        if self.active_layer != after.active_layer {
            diff.active_layer = Some(after.active_layer.clone());
        }
        diff.props =
            get_guid_collection_diff(&self.props, &after.props, "prop", |b, a| b.diff_from(a));
        diff.pieces =
            get_guid_collection_diff(&self.pieces, &after.pieces, "piece", |b, a| b.diff_from(a));
        diff.connections = get_guid_collection_diff(
            &self.connections,
            &after.connections,
            "connection",
            |b, a| b.diff_from(a),
        );
        diff.layers =
            get_guid_collection_diff(&self.layers, &after.layers, "layer", |b, a| b.diff_from(a));
        diff.groups =
            get_guid_collection_diff(&self.groups, &after.groups, "group", |b, a| b.diff_from(a));
        diff.stats =
            get_guid_collection_diff(&self.stats, &after.stats, "stat", |b, a| b.diff_from(a));
        diff.attributes = get_guid_collection_diff(
            &self.attributes,
            &after.attributes,
            "attribute",
            |b, a| b.diff_from(a),
        );
        diff
    }

    pub fn find_piece<'a>(&'a self, piece: &Piece) -> Option<&'a Piece> {
        self.pieces.as_ref()?.iter().find(|p| p.guid == piece.guid)
    }

    pub fn find_connection<'a>(&'a self, c: &Connection) -> Option<&'a Connection> {
        self.connections
            .as_ref()?
            .iter()
            .find(|x| x.guid == c.guid)
    }

    pub fn flatten(&self, kit: &Kit) -> SemioReport<DesignChange> {
        if find_design_in_kit(kit, &self.guid).is_none() {
            return SemioReport::err(vec![OperationNote {
                code: Some("flatten.design-not-found".into()),
                message: format!("Design {} not found in kit", self.guid),
            }]);
        }
        let change = flatten_design_change(kit, &self.guid);
        flatten_design_report_from_change(kit, &self.guid, change)
    }

    /// Deletes pieces and connections using entity references; expands stale connection removals.
    pub fn delete_pieces_and_connections(
        &self,
        kit: &Kit,
        pieces: &[Piece],
        connections: &[Connection],
    ) -> SemioReport<DesignDiff> {
        let piece_guids: Vec<String> = pieces.iter().map(|p| p.guid.clone()).collect();
        let connection_guids: Vec<String> = connections.iter().map(|c| c.guid.clone()).collect();
        delete_pieces_and_connections_in_design_core(kit, self, &piece_guids, &connection_guids)
    }

    pub fn drag_pieces(&self, pieces: &[Piece], offset: &Coord) -> DesignDiff {
        let design_pieces = self.pieces.as_deref().unwrap_or(&[]);
        let design_connections = self.connections.as_deref().unwrap_or(&[]);
        let mut d =
            drag_pieces_in_design(design_pieces, design_connections, pieces, offset);
        d.guid = self.guid.clone();
        d
    }
}

fn kit_diff_update_concept(kit: &Kit, before: &Concept, after: &Concept) -> KitDiff {
    let d = before.diff_from(after);
    KitDiff {
        guid: kit.guid.clone(),
        concepts: Some(CollectionDiff {
            updated: Some(vec![DiffUpdate {
                key: "concept".into(),
                guid: before.guid.clone(),
                diff: d,
            }]),
            ..Default::default()
        }),
        ..Default::default()
    }
}

fn kit_diff_remove_concept(kit: &Kit, c: &Concept) -> KitDiff {
    KitDiff {
        guid: kit.guid.clone(),
        concepts: Some(CollectionDiff {
            removed: Some(vec![RemovedItem {
                guid: c.guid.clone(),
            }]),
            ..Default::default()
        }),
        ..Default::default()
    }
}

impl Concept {
    pub fn rename(&self, kit: &mut Kit, name: impl Into<String>) -> Result<()> {
        let Some(v) = kit.concepts.as_ref() else {
            return Err(SemioError::NotFound {
                kind: "Concept".into(),
                guid: self.guid.clone(),
            });
        };
        let before = v
            .iter()
            .find(|c| c.guid == self.guid)
            .ok_or_else(|| SemioError::NotFound {
                kind: "Concept".into(),
                guid: self.guid.clone(),
            })?
            .clone();
        let mut after = before.clone();
        after.name = name.into();
        let kd = kit_diff_update_concept(kit, &before, &after);
        apply_kit_diff(kit, &kd);
        Ok(())
    }

    pub fn update_description(&self, kit: &mut Kit, description: &str) -> Result<()> {
        let Some(v) = kit.concepts.as_ref() else {
            return Err(SemioError::NotFound {
                kind: "Concept".into(),
                guid: self.guid.clone(),
            });
        };
        let before = v
            .iter()
            .find(|c| c.guid == self.guid)
            .ok_or_else(|| SemioError::NotFound {
                kind: "Concept".into(),
                guid: self.guid.clone(),
            })?
            .clone();
        let mut after = before.clone();
        after.description = Some(description.to_string());
        let kd = kit_diff_update_concept(kit, &before, &after);
        apply_kit_diff(kit, &kd);
        Ok(())
    }

    pub fn update_icon(&self, kit: &mut Kit, icon: &str) -> Result<()> {
        let Some(v) = kit.concepts.as_ref() else {
            return Err(SemioError::NotFound {
                kind: "Concept".into(),
                guid: self.guid.clone(),
            });
        };
        let before = v
            .iter()
            .find(|c| c.guid == self.guid)
            .ok_or_else(|| SemioError::NotFound {
                kind: "Concept".into(),
                guid: self.guid.clone(),
            })?
            .clone();
        let mut after = before.clone();
        after.icon = Some(icon.to_string());
        let kd = kit_diff_update_concept(kit, &before, &after);
        apply_kit_diff(kit, &kd);
        Ok(())
    }

    pub fn delete(&self, kit: &mut Kit) -> Result<()> {
        let kd = kit_diff_remove_concept(kit, self);
        apply_kit_diff(kit, &kd);
        Ok(())
    }
}

fn kit_diff_update_tag(kit: &Kit, before: &Tag, after: &Tag) -> KitDiff {
    let d = before.diff_from(after);
    KitDiff {
        guid: kit.guid.clone(),
        tags: Some(CollectionDiff {
            updated: Some(vec![DiffUpdate {
                key: "tag".into(),
                guid: before.guid.clone(),
                diff: d,
            }]),
            ..Default::default()
        }),
        ..Default::default()
    }
}

impl Tag {
    pub fn rename(&self, kit: &mut Kit, name: impl Into<String>) -> Result<()> {
        let before = kit
            .tags
            .as_ref()
            .and_then(|v| v.iter().find(|t| t.guid == self.guid))
            .ok_or_else(|| SemioError::NotFound {
                kind: "Tag".into(),
                guid: self.guid.clone(),
            })?
            .clone();
        let mut after = before.clone();
        after.name = name.into();
        let kd = kit_diff_update_tag(kit, &before, &after);
        apply_kit_diff(kit, &kd);
        Ok(())
    }

    pub fn update_description(&self, kit: &mut Kit, description: &str) -> Result<()> {
        let before = kit
            .tags
            .as_ref()
            .and_then(|v| v.iter().find(|t| t.guid == self.guid))
            .ok_or_else(|| SemioError::NotFound {
                kind: "Tag".into(),
                guid: self.guid.clone(),
            })?
            .clone();
        let mut after = before.clone();
        after.description = Some(description.to_string());
        let kd = kit_diff_update_tag(kit, &before, &after);
        apply_kit_diff(kit, &kd);
        Ok(())
    }

    pub fn update_icon(&self, kit: &mut Kit, icon: &str) -> Result<()> {
        let before = kit
            .tags
            .as_ref()
            .and_then(|v| v.iter().find(|t| t.guid == self.guid))
            .ok_or_else(|| SemioError::NotFound {
                kind: "Tag".into(),
                guid: self.guid.clone(),
            })?
            .clone();
        let mut after = before.clone();
        after.icon = Some(icon.to_string());
        let kd = kit_diff_update_tag(kit, &before, &after);
        apply_kit_diff(kit, &kd);
        Ok(())
    }

    pub fn delete(&self, kit: &mut Kit) -> Result<()> {
        let kd = KitDiff {
            guid: kit.guid.clone(),
            tags: Some(CollectionDiff {
                removed: Some(vec![RemovedItem {
                    guid: self.guid.clone(),
                }]),
                ..Default::default()
            }),
            ..Default::default()
        };
        apply_kit_diff(kit, &kd);
        Ok(())
    }
}

/// Core implementation for bulk delete (extracted from legacy helper).
pub(crate) fn delete_pieces_and_connections_in_design_core(
    kit: &Kit,
    design: &Design,
    piece_guids: &[String],
    connection_guids: &[String],
) -> SemioReport<DesignDiff> {
    let deleted_piece_set: HashSet<&str> = piece_guids.iter().map(|s| s.as_str()).collect();
    let connections = design.connections.as_deref().unwrap_or(&[]);

    let mut stale_connection_guids: HashSet<String> = HashSet::new();
    for conn in connections {
        if deleted_piece_set.contains(conn.connected.piece.guid.as_str())
            || deleted_piece_set.contains(conn.connecting.piece.guid.as_str())
        {
            stale_connection_guids.insert(conn.guid.clone());
        }
    }

    let mut all_removed_connection_guids: HashSet<String> =
        connection_guids.iter().cloned().collect();
    all_removed_connection_guids.extend(stale_connection_guids);

    let mut fixed_piece_guids: Vec<String> = Vec::new();
    for conn_guid in &all_removed_connection_guids {
        let conn = match connections.iter().find(|c| &c.guid == conn_guid) {
            Some(c) => c,
            None => continue,
        };
        let connecting_guid = &conn.connecting.piece.guid;
        if deleted_piece_set.contains(connecting_guid.as_str()) {
            continue;
        }
        let has_other_parent = connections.iter().any(|c| {
            c.connecting.piece.guid == *connecting_guid && !all_removed_connection_guids.contains(&c.guid)
        });
        if !has_other_parent && !fixed_piece_guids.contains(connecting_guid) {
            fixed_piece_guids.push(connecting_guid.clone());
        }
    }

    let flat_rep = flatten_design(kit, &design.guid);
    if !flat_rep.ok {
        return SemioReport::err(flat_rep.errors);
    }
    let flat_change = flat_rep.diff.expect("flatten ok implies diff");
    let mut flat_piece_map: std::collections::HashMap<String, (Option<Plane>, Option<Coord>)> =
        std::collections::HashMap::new();
    if let Some(pieces) = &design.pieces {
        for piece in pieces {
            if let Some(plane) = &piece.plane {
                flat_piece_map.insert(
                    piece.guid.clone(),
                    (Some(plane.clone()), piece.center.clone()),
                );
            }
        }
    }
    if let Some(pieces_diff) = &flat_change.forward.pieces {
        if let Some(updates) = &pieces_diff.updated {
            for update in updates {
                let entry = flat_piece_map
                    .entry(update.guid.clone())
                    .or_insert((None, None));
                if let Some(Some(plane)) = &update.diff.plane {
                    entry.0 = Some(plane.clone());
                }
                if let Some(Some(center)) = &update.diff.center {
                    entry.1 = Some(center.clone());
                }
            }
        }
    }

    let pieces_removed: Vec<RemovedItem> = piece_guids
        .iter()
        .map(|g| RemovedItem { guid: g.clone() })
        .collect();
    let pieces_updated: Vec<DiffUpdate<PieceDiff>> = fixed_piece_guids
        .iter()
        .map(|g| {
            let (flat_plane, flat_center) = flat_piece_map
                .get(g)
                .cloned()
                .unwrap_or((Some(Plane::default()), Some(Coord::default())));
            DiffUpdate {
                key: "piece".to_string(),
                guid: g.clone(),
                diff: PieceDiff {
                    guid: g.clone(),
                    plane: Some(flat_plane),
                    center: Some(flat_center),
                    ..Default::default()
                },
            }
        })
        .collect();
    let mut sorted_removed_connections: Vec<String> =
        all_removed_connection_guids.into_iter().collect();
    sorted_removed_connections.sort();
    let connections_removed: Vec<RemovedItem> = sorted_removed_connections
        .iter()
        .map(|g| RemovedItem { guid: g.clone() })
        .collect();

    let mut diff = DesignDiff {
        guid: design.guid.clone(),
        ..Default::default()
    };

    if !pieces_removed.is_empty() || !pieces_updated.is_empty() {
        diff.pieces = Some(CollectionDiff {
            removed: if pieces_removed.is_empty() {
                None
            } else {
                Some(pieces_removed)
            },
            updated: if pieces_updated.is_empty() {
                None
            } else {
                Some(pieces_updated)
            },
            added: None,
        });
    }

    if !connections_removed.is_empty() {
        diff.connections = Some(CollectionDiff {
            removed: Some(connections_removed),
            updated: None,
            added: None,
        });
    }

    SemioReport::ok_with(diff, flat_rep.warnings, flat_rep.infos)
}
