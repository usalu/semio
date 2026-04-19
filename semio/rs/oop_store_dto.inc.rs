// Included from `mod oop` in lib.rs — Store / DTO diagram surface.

use serde_json::{Map, Value};
use std::collections::HashMap;

// ——— DTO bases

pub trait Dto {
    fn validate(&self) -> bool {
        true
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct IdDto {
    pub guid: String,
}

impl IdDto {
    pub fn new(guid: impl Into<String>) -> Self {
        Self {
            guid: guid.into(),
        }
    }

    pub fn as_guid(&self) -> &str {
        self.guid.as_str()
    }

    pub fn is_empty(&self) -> bool {
        self.guid.is_empty()
    }
}

impl Dto for IdDto {}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct InputDto {
    pub fields: Map<String, Value>,
    #[serde(default)]
    pub references: HashMap<String, IdDto>,
    #[serde(default)]
    pub children: HashMap<String, Vec<InputDto>>,
}

impl InputDto {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set(&mut self, field: impl Into<String>, value: Value) {
        self.fields.insert(field.into(), value);
    }

    pub fn add_reference(&mut self, field: impl Into<String>, reference: IdDto) {
        self.references.insert(field.into(), reference);
    }

    pub fn add_child(&mut self, field: impl Into<String>, child: InputDto) {
        self.children
            .entry(field.into())
            .or_default()
            .push(child);
    }

    pub fn validate(&self) -> bool {
        true
    }
}

impl Dto for InputDto {
    fn validate(&self) -> bool {
        true
    }
}

pub trait MetadataDto: Dto {
    fn add_reference(&mut self, field: impl Into<String>, reference: IdDto);
    fn has_reference(&self, field: &str) -> bool;
    fn validate(&self) -> bool {
        true
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct MetadataRecord {
    pub payload: Value,
    pub references: HashMap<String, IdDto>,
}

impl Dto for MetadataRecord {}

impl MetadataDto for MetadataRecord {
    fn add_reference(&mut self, field: impl Into<String>, reference: IdDto) {
        self.references.insert(field.into(), reference);
    }

    fn has_reference(&self, field: &str) -> bool {
        self.references.contains_key(field)
    }
}

pub trait ShallowDtoTrait: MetadataDto {
    fn add_child_view(&mut self, field: impl Into<String>, child: MetadataRecord);
    fn flatten_children(&self) -> Vec<MetadataRecord> {
        vec![]
    }
    fn validate(&self) -> bool {
        true
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ShallowRecord {
    pub meta: MetadataRecord,
    #[serde(default)]
    pub child_views: HashMap<String, Vec<MetadataRecord>>,
}

impl Dto for ShallowRecord {}

impl MetadataDto for ShallowRecord {
    fn add_reference(&mut self, field: impl Into<String>, reference: IdDto) {
        self.meta.add_reference(field, reference);
    }

    fn has_reference(&self, field: &str) -> bool {
        self.meta.has_reference(field)
    }
}

impl ShallowDtoTrait for ShallowRecord {
    fn add_child_view(&mut self, field: impl Into<String>, child: MetadataRecord) {
        self.child_views.entry(field.into()).or_default().push(child);
    }

    fn flatten_children(&self) -> Vec<MetadataRecord> {
        self.child_views.values().flatten().cloned().collect()
    }
}

pub trait FullDtoTrait: Dto {
    fn add_reference_full(&mut self, field: impl Into<String>, reference: IdDto);
    fn add_child(&mut self, field: impl Into<String>, child: FullRecord);
    fn add_derived(&mut self, key: impl Into<String>, value: Value);
    fn compute_derived(&mut self) {}
    fn validate(&self) -> bool {
        true
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct FullRecord {
    pub root: Value,
    pub references: HashMap<String, IdDto>,
    #[serde(default)]
    pub children: HashMap<String, Vec<FullRecord>>,
    #[serde(default)]
    pub derived: Map<String, Value>,
}

impl Dto for FullRecord {}

impl FullDtoTrait for FullRecord {
    fn add_reference_full(&mut self, field: impl Into<String>, reference: IdDto) {
        self.references.insert(field.into(), reference);
    }

    fn add_child(&mut self, field: impl Into<String>, child: FullRecord) {
        self.children.entry(field.into()).or_default().push(child);
    }

    fn add_derived(&mut self, key: impl Into<String>, value: Value) {
        self.derived.insert(key.into(), value);
    }

    fn compute_derived(&mut self) {}
}

include!("oop_dto_entities.inc.rs");

// ——— Store trait (diagram: abstract Store)

pub trait Store: HasGuid + Serialize {
    fn get_name(&self) -> &str;
    fn update_description(&self, kit: &mut Kit, description: &str) -> Result<()>;
    fn to_id_dto(&self) -> IdDto {
        IdDto::new(self.guid())
    }
    fn to_input_dto(&self) -> InputDto {
        json_input_from(self)
    }
    fn to_metadata_dto(&self) -> MetadataRecord {
        MetadataRecord {
            payload: serde_json::to_value(self).unwrap_or(Value::Null),
            references: HashMap::new(),
        }
    }
    fn to_shallow_dto(&self) -> ShallowRecord {
        ShallowRecord {
            meta: self.to_metadata_dto(),
            child_views: HashMap::new(),
        }
    }
    fn to_full_dto(&self) -> FullRecord {
        FullRecord {
            root: serde_json::to_value(self).unwrap_or(Value::Null),
            references: HashMap::new(),
            children: HashMap::new(),
            derived: Map::new(),
        }
    }
}

fn json_input_from<T: Serialize + ?Sized>(v: &T) -> InputDto {
    match serde_json::to_value(v) {
        Ok(Value::Object(map)) => InputDto {
            fields: map,
            references: HashMap::new(),
            children: HashMap::new(),
        },
        Ok(other) => {
            let mut i = InputDto::new();
            i.set("_", other);
            i
        }
        Err(_) => InputDto::new(),
    }
}

impl Store for Kit {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn update_description(&self, kit: &mut Kit, description: &str) -> Result<()> {
        if self.guid != kit.guid {
            return Err(SemioError::InvalidOperation {
                message: "Kit Store update_description requires context kit with matching guid".into(),
            });
        }
        kit.description = Some(description.to_string());
        Ok(())
    }

    fn to_input_dto(&self) -> InputDto {
        json_input_from(self)
    }
}

impl Store for Concept {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn update_description(&self, kit: &mut Kit, description: &str) -> Result<()> {
        Concept::update_description(self, kit, description)
    }

    fn to_input_dto(&self) -> InputDto {
        json_input_from(self)
    }
}

impl Store for Tag {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn update_description(&self, kit: &mut Kit, description: &str) -> Result<()> {
        Tag::update_description(self, kit, description)
    }

    fn to_input_dto(&self) -> InputDto {
        json_input_from(self)
    }
}

impl Store for Attribute {
    fn get_name(&self) -> &str {
        &self.key
    }

    fn update_description(&self, _kit: &mut Kit, _description: &str) -> Result<()> {
        Ok(())
    }

    fn to_input_dto(&self) -> InputDto {
        json_input_from(self)
    }
}

impl Store for Author {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn update_description(&self, _kit: &mut Kit, _description: &str) -> Result<()> {
        Ok(())
    }

    fn to_input_dto(&self) -> InputDto {
        json_input_from(self)
    }
}

impl Store for Folder {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn update_description(&self, _kit: &mut Kit, _description: &str) -> Result<()> {
        Ok(())
    }

    fn to_input_dto(&self) -> InputDto {
        json_input_from(self)
    }
}

impl Store for File {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn update_description(&self, _kit: &mut Kit, _description: &str) -> Result<()> {
        Ok(())
    }

    fn to_input_dto(&self) -> InputDto {
        json_input_from(self)
    }
}

impl Store for Quality {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn update_description(&self, _kit: &mut Kit, _description: &str) -> Result<()> {
        Ok(())
    }

    fn to_input_dto(&self) -> InputDto {
        json_input_from(self)
    }
}

impl Store for Benchmark {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn update_description(&self, _kit: &mut Kit, _description: &str) -> Result<()> {
        Ok(())
    }

    fn to_input_dto(&self) -> InputDto {
        json_input_from(self)
    }
}

impl Store for Stat {
    fn get_name(&self) -> &str {
        self.guid.as_str()
    }

    fn update_description(&self, _kit: &mut Kit, _description: &str) -> Result<()> {
        Ok(())
    }

    fn to_input_dto(&self) -> InputDto {
        json_input_from(self)
    }
}

impl Store for Port {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn update_description(&self, _kit: &mut Kit, _description: &str) -> Result<()> {
        Ok(())
    }

    fn to_input_dto(&self) -> InputDto {
        json_input_from(self)
    }
}

impl Store for Connector {
    fn get_name(&self) -> &str {
        self.name.as_deref().unwrap_or(self.guid.as_str())
    }

    fn update_description(&self, _kit: &mut Kit, _description: &str) -> Result<()> {
        Ok(())
    }

    fn to_input_dto(&self) -> InputDto {
        json_input_from(self)
    }
}

impl Store for Prop {
    fn get_name(&self) -> &str {
        self.guid.as_str()
    }

    fn update_description(&self, _kit: &mut Kit, _description: &str) -> Result<()> {
        Ok(())
    }

    fn to_input_dto(&self) -> InputDto {
        json_input_from(self)
    }
}

impl Store for Layer {
    fn get_name(&self) -> &str {
        &self.path
    }

    fn update_description(&self, _kit: &mut Kit, _description: &str) -> Result<()> {
        Ok(())
    }

    fn to_input_dto(&self) -> InputDto {
        json_input_from(self)
    }
}

impl Store for Group {
    fn get_name(&self) -> &str {
        self.name.as_deref().unwrap_or(self.guid.as_str())
    }

    fn update_description(&self, _kit: &mut Kit, _description: &str) -> Result<()> {
        Ok(())
    }

    fn to_input_dto(&self) -> InputDto {
        json_input_from(self)
    }
}

impl Store for Piece {
    fn get_name(&self) -> &str {
        self.name.as_deref().unwrap_or(self.guid.as_str())
    }

    fn update_description(&self, _kit: &mut Kit, _description: &str) -> Result<()> {
        Ok(())
    }

    fn to_input_dto(&self) -> InputDto {
        json_input_from(self)
    }
}

impl Store for Connection {
    fn get_name(&self) -> &str {
        self.guid.as_str()
    }

    fn update_description(&self, _kit: &mut Kit, _description: &str) -> Result<()> {
        Ok(())
    }

    fn to_input_dto(&self) -> InputDto {
        json_input_from(self)
    }
}

impl Store for Type {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn update_description(&self, _kit: &mut Kit, _description: &str) -> Result<()> {
        Ok(())
    }

    fn to_input_dto(&self) -> InputDto {
        json_input_from(self)
    }
}

impl Store for Design {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn update_description(&self, kit: &mut Kit, description: &str) -> Result<()> {
        let before = kit
            .designs
            .as_ref()
            .and_then(|d| d.iter().find(|x| x.guid == self.guid))
            .ok_or_else(|| SemioError::NotFound {
                kind: "Design".into(),
                guid: self.guid.clone(),
            })?
            .clone();
        let mut after = before.clone();
        after.description = Some(description.to_string());
        let kd = KitDiff {
            guid: kit.guid.clone(),
            designs: Some(CollectionDiff {
                updated: Some(vec![DiffUpdate {
                    key: "design".into(),
                    guid: self.guid.clone(),
                    diff: before.diff_from(&after),
                }]),
                removed: None,
                added: None,
            }),
            ..KitDiff::default()
        };
        apply_kit_diff(kit, &kd);
        Ok(())
    }

    fn to_input_dto(&self) -> InputDto {
        json_input_from(self)
    }
}

impl Store for Model {
    fn get_name(&self) -> &str {
        self.name.as_deref().unwrap_or(self.guid.as_str())
    }

    fn update_description(&self, _kit: &mut Kit, _description: &str) -> Result<()> {
        Ok(())
    }

    fn to_input_dto(&self) -> InputDto {
        json_input_from(self)
    }
}

impl Store for Location {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn update_description(&self, _kit: &mut Kit, _description: &str) -> Result<()> {
        Ok(())
    }

    fn to_input_dto(&self) -> InputDto {
        json_input_from(self)
    }
}

// ——— Diagram type aliases (*Store)

pub type KitStore = Kit;
pub type AttributeStore = Attribute;
pub type AuthorStore = Author;
pub type LocationStore = Location;
pub type FolderStore = Folder;
pub type FileStore = File;
pub type ConceptStore = Concept;
pub type QualityStore = Quality;
pub type BenchmarkStore = Benchmark;
pub type StatStore = Stat;
pub type TagStore = Tag;
pub type ModelStore = Model;
pub type PortStore = Port;
pub type ConnectorStore = Connector;
pub type PropStore = Prop;
pub type LayerStore = Layer;
pub type GroupStore = Group;
pub type PieceStore = Piece;
pub type ConnectionStore = Connection;
pub type TypeStore = Type;
pub type DesignStore = Design;

// ——— Selection carrier (diagram: KitOperation uses Store)

#[derive(Debug, Clone)]
pub enum AnyStore {
    Kit(Kit),
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
    Location(Location),
}

impl HasGuid for AnyStore {
    fn guid(&self) -> &str {
        match self {
            AnyStore::Kit(e) => e.guid.as_str(),
            AnyStore::Tag(e) => e.guid(),
            AnyStore::Concept(e) => e.guid(),
            AnyStore::Port(e) => e.guid(),
            AnyStore::Quality(e) => e.guid(),
            AnyStore::Type(e) => e.guid(),
            AnyStore::Design(e) => e.guid(),
            AnyStore::File(e) => e.guid(),
            AnyStore::Folder(e) => e.guid(),
            AnyStore::Author(e) => e.guid(),
            AnyStore::Attribute(e) => e.guid(),
            AnyStore::Model(e) => e.guid(),
            AnyStore::Connector(e) => e.guid(),
            AnyStore::Prop(e) => e.guid(),
            AnyStore::Layer(e) => e.guid(),
            AnyStore::Group(e) => e.guid(),
            AnyStore::Piece(e) => e.guid(),
            AnyStore::Connection(e) => e.guid(),
            AnyStore::Stat(e) => e.guid(),
            AnyStore::Benchmark(e) => &e.guid,
            AnyStore::Location(e) => e.guid.as_str(),
        }
    }
}

/// Back-compat name for [`AnyStore`].
pub type KitEntity = AnyStore;

impl Side {
    pub fn set_piece_store(&mut self, piece: &PieceStore) {
        self.piece = PieceId {
            guid: piece.guid.clone(),
        };
    }

    pub fn set_design_piece_store(&mut self, design_piece: &PieceStore) {
        self.design_piece = Some(PieceId {
            guid: design_piece.guid.clone(),
        });
    }

    pub fn set_connector_store(&mut self, connector: &ConnectorStore) {
        self.connector = Some(ConnectorId {
            guid: connector.guid.clone(),
        });
    }
}
