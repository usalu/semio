// Granular change commands: run against `KitStoreRef`, inverses for undo.
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::attribute::AttributeFullDto;
use crate::attribute::AttributeIdDto;
use crate::author::AuthorFullDto;
use crate::author::AuthorIdDto;
use crate::concept::ConceptFullDto;
use crate::concept::ConceptIdDto;
use crate::connection::ConnectionFullDto;
use crate::connection::ConnectionIdDto;
use crate::connector::ConnectorFullDto;
use crate::connector::ConnectorIdDto;
use crate::design::DesignFullDto;
use crate::design::DesignIdDto;
use crate::diff::DesignDiff;
use crate::file::FileFullDto;
use crate::file::FileIdDto;
use crate::folder::FolderFullDto;
use crate::folder::FolderIdDto;
use crate::geom::Coordinate;
use crate::geom::Location;
use crate::geom::Plane;
use crate::group::GroupFullDto;
use crate::group::GroupIdDto;
use crate::id::Id;
use crate::kit::KitStore;
use crate::kit::KitStoreRef;
use crate::kit_change::KitChangeKind;
use crate::kit_diff::KitDiff;
use crate::layer::LayerFullDto;
use crate::layer::LayerIdDto;
use crate::piece::PieceFullDto;
use crate::piece::PieceIdDto;
use crate::port::PortFullDto;
use crate::port::PortIdDto;
use crate::prop::PropFullDto;
use crate::prop::PropIdDto;
use crate::quality::QualityFullDto;
use crate::quality::QualityIdDto;
use crate::representation::RepresentationFullDto;
use crate::representation::RepresentationIdDto;
use crate::side::SideMetadataDto;
use crate::stat::StatFullDto;
use crate::stat::StatIdDto;
use crate::tag::TagFullDto;
use crate::tag::TagIdDto;
use crate::typ::TypeFullDto;
use crate::typ::TypeIdDto;
use crate::geom::Camera;
use crate::{error::Result, error::SemioError};

fn se(e: crate::error::SetError) -> SemioError {
    SemioError::InvalidOperation(e.to_string())
}

/// Batch-style forward step: apply a structural [`KitDiff`]; inverse is a matching backward diff
/// (produced with [`KitChange::from_dto_pair`] in [`crate::kit_change`]).
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ChangeKitCommand {
    FromKitDiff { diff: KitDiff },
    // --- kit metadata (mirrors set_* on [`KitStore`]) ---
    Name { name: String },
    Description { description: Option<String> },
    Icon { icon: Option<String> },
    Image { image: Option<String> },
    Preview { preview: Option<String> },
    Version { version: Option<String> },
    Remote { remote: Option<String> },
    Homepage { homepage: Option<String> },
    License { license: Option<String> },
    Uri { uri: Option<String> },
    Created { created: Option<String> },
    Updated { updated: Option<String> },
    // --- life-cycle (kit-scoped) ---
    AddType { r#type: TypeFullDto },
    RemoveType { type_id: TypeIdDto },
    AddDesign { design: DesignFullDto },
    RemoveDesign { design_id: DesignIdDto },
    AddFile { file: FileFullDto },
    RemoveFile { file_id: FileIdDto },
    AddFolder { folder: FolderFullDto },
    RemoveFolder { folder_id: FolderIdDto },
    AddAuthor { author: AuthorFullDto },
    RemoveAuthor { author_id: AuthorIdDto },
    AddConcept { concept: ConceptFullDto },
    RemoveConcept { concept_id: ConceptIdDto },
    AddTag { tag: TagFullDto },
    RemoveTag { tag_id: TagIdDto },
    AddQuality { quality: QualityFullDto },
    RemoveQuality { quality_id: QualityIdDto },
    AddKitProp { prop: PropFullDto },
    RemoveKitProp { prop_id: PropIdDto },
    AddKitAttribute { attribute: AttributeFullDto },
    RemoveKitAttribute { id: AttributeIdDto },
    // --- nested scoping ---
    ChangeTypeCommands { type_id: TypeIdDto, commands: Vec<ChangeTypeCommand> },
    ChangeDesignCommands { design_id: DesignIdDto, commands: Vec<ChangeDesignCommand> },
    #[serde(other)]
    Other,
}

/// Per-field commands on a [`crate::typ::TypeStore`].
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ChangeTypeCommand {
    Name { name: String },
    Description { description: Option<String> },
    Icon { icon: Option<String> },
    Image { image: Option<String> },
    Variant { variant: Option<String> },
    Stock { stock: Option<i64> },
    /// Maps to `TypeStore::set_virtual` (serde wire: `typeVirtual` when camelCased on this variant name).
    TypeVirtual { value: Option<bool> },
    Unit { unit: Option<String> },
    Location { location: Option<Location> },
    Created { created: Option<String> },
    Updated { updated: Option<String> },
    AddPort { port: PortFullDto },
    RemovePort { port_id: PortIdDto },
    ChangePortCommands { port_id: PortIdDto, commands: Vec<ChangePortCommand> },
    AddConnector { connector: ConnectorFullDto },
    RemoveConnector { connector_id: ConnectorIdDto },
    ChangeConnectorCommands { connector_id: ConnectorIdDto, commands: Vec<ChangeConnectorCommand> },
    AddRepresentation { representation: RepresentationFullDto },
    RemoveRepresentation { id: RepresentationIdDto },
    ChangeRepresentationCommands { id: RepresentationIdDto, commands: Vec<ChangeRepresentationCommand> },
    AddTypeAuthor { author: AuthorFullDto },
    RemoveTypeAuthor { author_id: AuthorIdDto },
    AddTypeConcept { concept: ConceptFullDto },
    RemoveTypeConcept { concept_id: ConceptIdDto },
    AddTypeTag { tag: TagFullDto },
    RemoveTypeTag { tag_id: TagIdDto },
    AddTypeQuality { quality: QualityFullDto },
    RemoveTypeQuality { quality_id: QualityIdDto },
    AddTypeProp { prop: PropFullDto },
    RemoveTypeProp { prop_id: PropIdDto },
    AddTypeAttribute { attribute: AttributeFullDto },
    RemoveTypeAttribute { id: AttributeIdDto },
    #[serde(other)]
    Other,
}

/// Per-field on [`crate::port::PortStore`].
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ChangePortCommand {
    Id { id: Id },
    Family { family: Option<String> },
    CompatibleFamilies { families: Vec<String> },
    Mandatory { mandatory: Option<bool> },
    T { t: Option<f64> },
    Description { description: Option<String> },
    Point { point: Option<Coordinate> },
    Direction { direction: Option<crate::geom::Vector> },
    AddPortQuality { quality: QualityFullDto },
    RemovePortQuality { quality_id: QualityIdDto },
    AddPortAttribute { attribute: AttributeFullDto },
    RemovePortAttribute { id: AttributeIdDto },
    #[serde(other)]
    Other,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ChangeConnectorCommand {
    Code { code: String },
    Description { description: Option<String> },
    Port { port: Option<PortIdDto> },
    AddProp { prop: PropFullDto },
    RemoveProp { prop_id: PropIdDto },
    AddAttribute { attribute: AttributeFullDto },
    RemoveAttribute { id: AttributeIdDto },
    #[serde(other)]
    Other,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ChangeRepresentationCommand {
    Url { url: String },
    Description { description: Option<String> },
    File { file: Option<FileIdDto> },
    AddTag { tag: TagIdDto },
    RemoveTag { tag: TagIdDto },
    AddQuality { quality: QualityFullDto },
    RemoveQuality { quality_id: QualityIdDto },
    AddAttribute { attribute: AttributeFullDto },
    RemoveAttribute { id: AttributeIdDto },
    #[serde(other)]
    Other,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ChangeDesignCommand {
    Name { name: String },
    Description { description: Option<String> },
    Icon { icon: Option<String> },
    Image { image: Option<String> },
    Variant { variant: Option<String> },
    View { view: Option<String> },
    Location { location: Option<Location> },
    Camera { camera: Option<Camera> },
    Unit { unit: Option<String> },
    Created { created: Option<String> },
    Updated { updated: Option<String> },
    AddPiece { piece: PieceFullDto },
    RemovePiece { piece_id: PieceIdDto },
    ChangePieceCommands { piece_id: PieceIdDto, commands: Vec<ChangePieceCommand> },
    AddConnection { connection: ConnectionFullDto },
    RemoveConnection { connection_id: ConnectionIdDto },
    ChangeConnectionCommands { connection_id: ConnectionIdDto, commands: Vec<ChangeConnectionCommand> },
    AddLayer { layer: LayerFullDto },
    RemoveLayer { layer_id: LayerIdDto },
    ChangeLayerCommands { layer_id: LayerIdDto, commands: Vec<ChangeLayerCommand> },
    AddGroup { group: GroupFullDto },
    RemoveGroup { group_id: GroupIdDto },
    ChangeGroupCommands { group_id: GroupIdDto, commands: Vec<ChangeGroupCommand> },
    AddStat { stat: StatFullDto },
    RemoveStat { stat_id: StatIdDto },
    ChangeStatCommands { stat_id: StatIdDto, commands: Vec<ChangeStatCommand> },
    AddDesignAuthor { author: AuthorFullDto },
    RemoveDesignAuthor { author_id: AuthorIdDto },
    AddDesignConcept { concept: ConceptFullDto },
    RemoveDesignConcept { concept_id: ConceptIdDto },
    AddDesignTag { tag: TagFullDto },
    RemoveDesignTag { tag_id: TagIdDto },
    AddDesignQuality { quality: QualityFullDto },
    RemoveDesignQuality { quality_id: QualityIdDto },
    AddDesignProp { prop: PropFullDto },
    RemoveDesignProp { prop_id: PropIdDto },
    AddDesignAttribute { attribute: AttributeFullDto },
    RemoveDesignAttribute { id: AttributeIdDto },
    #[serde(other)]
    Other,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ChangeLayerCommand {
    Name { name: String },
    Description { description: Option<String> },
    Color { color: Option<String> },
    Order { order: Option<i64> },
    Visible { visible: Option<bool> },
    Locked { locked: Option<bool> },
    AddLayerAttribute { attribute: AttributeFullDto },
    RemoveLayerAttribute { id: AttributeIdDto },
    #[serde(other)]
    Other,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ChangeGroupCommand {
    Name { name: String },
    Description { description: Option<String> },
    Color { color: Option<String> },
    Icon { icon: Option<String> },
    Pieces { pieces: Vec<PieceIdDto> },
    AddGroupAttribute { attribute: AttributeFullDto },
    RemoveGroupAttribute { id: AttributeIdDto },
    #[serde(other)]
    Other,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ChangeStatCommand {
    Key { key: String },
    Value { value: String },
    Unit { unit: Option<String> },
    Description { description: Option<String> },
    #[serde(other)]
    Other,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ChangeConnectionCommand {
    Gap { value: Option<f64> },
    Shift { value: Option<f64> },
    Rise { value: Option<f64> },
    Rotation { value: Option<f64> },
    Turn { value: Option<f64> },
    Tilt { value: Option<f64> },
    X { value: Option<f64> },
    Y { value: Option<f64> },
    Description { value: Option<String> },
    ReplaceConnected { side: SideMetadataDto },
    ReplaceConnecting { side: SideMetadataDto },
    AddConnectionAttribute { attribute: AttributeFullDto },
    RemoveConnectionAttribute { id: AttributeIdDto },
    #[serde(other)]
    Other,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ChangePieceCommand {
    Name { name: Option<String> },
    Description { description: Option<String> },
    Plane { plane: Option<Plane> },
    Center { center: Option<Coordinate> },
    Color { color: Option<String> },
    Type { type_id: Option<TypeIdDto> },
    Scale { scale: Option<f64> },
    MirrorPlane { mirror_plane: Option<Plane> },
    Hidden { hidden: Option<bool> },
    Locked { locked: Option<bool> },
    Id { id: Id },
    AddProp { prop: PropFullDto },
    RemoveProp { prop_id: PropIdDto },
    ChangePropCommands { prop_id: PropIdDto, commands: Vec<ChangePropCommand> },
    AddAttribute { attribute: AttributeFullDto },
    RemoveAttribute { id: AttributeIdDto },
    ChangeAttributeCommands { id: AttributeIdDto, commands: Vec<ChangeAttributeCommand> },
    Fix,
    #[serde(other)]
    Other,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ChangePropCommand {
    Key { key: String },
    Value { value: String },
    Unit { unit: Option<String> },
    AddPropAttribute { attribute: AttributeFullDto },
    RemovePropAttribute { id: AttributeIdDto },
    #[serde(other)]
    Other,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ChangeAttributeCommand {
    Key { key: String },
    Value { value: String },
    Definition { definition: Option<String> },
    #[serde(other)]
    Other,
}

// --- impl: ChangeKitCommand ---

impl ChangeKitCommand {
    /// Apply the command; does not return inverses (checkpoint replay, redo).
    pub fn run(&self, kit: &KitStoreRef) -> Result<KitChangeKind> {
        self.apply(kit).map(|(k, _)| k)
    }

    /// Apply and return the inverse command list to undo this step.
    pub fn apply(&self, kit: &KitStoreRef) -> Result<(KitChangeKind, Vec<ChangeKitCommand>)> {
        match self {
            ChangeKitCommand::FromKitDiff { diff } => {
                let before = kit.read().map_err(|_| SemioError::LockPoisoned("kit"))?.to_full_dto();
                diff.apply(kit).map_err(se)?;
                let after = kit.read().map_err(|_| SemioError::LockPoisoned("kit"))?.to_full_dto();
                let backward = KitDiff::between(&after, &before);
                let inv = if backward.is_empty() {
                    vec![]
                } else {
                    vec![ChangeKitCommand::FromKitDiff { diff: backward }]
                };
                Ok((KitChangeKind::Inferred, inv))
            }
            ChangeKitCommand::Name { name } => {
                let old = { kit.read().map_err(|_| SemioError::LockPoisoned("kit"))?.name.clone() };
                kit.write().map_err(|_| SemioError::LockPoisoned("kit"))?.set_name(name.clone()).map_err(se)?;
                Ok((
                    KitChangeKind::SetKitMetadata,
                    if old == *name { vec![] } else { vec![ChangeKitCommand::Name { name: old }] },
                ))
            }
            ChangeKitCommand::Description { description } => {
                let old = { kit.read().map_err(|_| SemioError::LockPoisoned("kit"))?.description.clone() };
                kit.write()
                    .map_err(|_| SemioError::LockPoisoned("kit"))?
                    .set_description(description.clone())
                    .map_err(se)?;
                Ok((
                    KitChangeKind::SetKitMetadata,
                    if old == *description { vec![] } else { vec![ChangeKitCommand::Description { description: old }] },
                ))
            }
            ChangeKitCommand::Icon { icon } => {
                let old = { kit.read().map_err(|_| SemioError::LockPoisoned("kit"))?.icon.clone() };
                kit.write()
                    .map_err(|_| SemioError::LockPoisoned("kit"))?
                    .set_icon(icon.clone())
                    .map_err(se)?;
                Ok((
                    KitChangeKind::SetKitMetadata,
                    if old == *icon { vec![] } else { vec![ChangeKitCommand::Icon { icon: old }] },
                ))
            }
            ChangeKitCommand::Image { image } => {
                let old = { kit.read().map_err(|_| SemioError::LockPoisoned("kit"))?.image.clone() };
                kit.write()
                    .map_err(|_| SemioError::LockPoisoned("kit"))?
                    .set_image(image.clone())
                    .map_err(se)?;
                Ok((
                    KitChangeKind::SetKitMetadata,
                    if old == *image { vec![] } else { vec![ChangeKitCommand::Image { image: old }] },
                ))
            }
            ChangeKitCommand::Preview { preview } => {
                let old = { kit.read().map_err(|_| SemioError::LockPoisoned("kit"))?.preview.clone() };
                kit.write()
                    .map_err(|_| SemioError::LockPoisoned("kit"))?
                    .set_preview(preview.clone())
                    .map_err(se)?;
                Ok((
                    KitChangeKind::SetKitMetadata,
                    if old == *preview { vec![] } else { vec![ChangeKitCommand::Preview { preview: old }] },
                ))
            }
            ChangeKitCommand::Version { version } => {
                let old = { kit.read().map_err(|_| SemioError::LockPoisoned("kit"))?.version.clone() };
                kit.write()
                    .map_err(|_| SemioError::LockPoisoned("kit"))?
                    .set_version(version.clone())
                    .map_err(se)?;
                Ok((
                    KitChangeKind::SetKitMetadata,
                    if old == *version { vec![] } else { vec![ChangeKitCommand::Version { version: old }] },
                ))
            }
            ChangeKitCommand::Remote { remote } => {
                let old = { kit.read().map_err(|_| SemioError::LockPoisoned("kit"))?.remote.clone() };
                kit.write()
                    .map_err(|_| SemioError::LockPoisoned("kit"))?
                    .set_remote(remote.clone())
                    .map_err(se)?;
                Ok((
                    KitChangeKind::SetKitMetadata,
                    if old == *remote { vec![] } else { vec![ChangeKitCommand::Remote { remote: old }] },
                ))
            }
            ChangeKitCommand::Homepage { homepage } => {
                let old = { kit.read().map_err(|_| SemioError::LockPoisoned("kit"))?.homepage.clone() };
                kit.write()
                    .map_err(|_| SemioError::LockPoisoned("kit"))?
                    .set_homepage(homepage.clone())
                    .map_err(se)?;
                Ok((
                    KitChangeKind::SetKitMetadata,
                    if old == *homepage { vec![] } else { vec![ChangeKitCommand::Homepage { homepage: old }] },
                ))
            }
            ChangeKitCommand::License { license } => {
                let old = { kit.read().map_err(|_| SemioError::LockPoisoned("kit"))?.license.clone() };
                kit.write()
                    .map_err(|_| SemioError::LockPoisoned("kit"))?
                    .set_license(license.clone())
                    .map_err(se)?;
                Ok((
                    KitChangeKind::SetKitMetadata,
                    if old == *license { vec![] } else { vec![ChangeKitCommand::License { license: old }] },
                ))
            }
            ChangeKitCommand::Uri { uri } => {
                let old = { kit.read().map_err(|_| SemioError::LockPoisoned("kit"))?.uri.clone() };
                kit.write()
                    .map_err(|_| SemioError::LockPoisoned("kit"))?
                    .set_uri(uri.clone())
                    .map_err(se)?;
                Ok((
                    KitChangeKind::SetKitMetadata,
                    if old == *uri { vec![] } else { vec![ChangeKitCommand::Uri { uri: old }] },
                ))
            }
            ChangeKitCommand::Created { created } => {
                let old = { kit.read().map_err(|_| SemioError::LockPoisoned("kit"))?.created.clone() };
                kit.write()
                    .map_err(|_| SemioError::LockPoisoned("kit"))?
                    .set_created(created.clone())
                    .map_err(se)?;
                Ok((
                    KitChangeKind::SetKitMetadata,
                    if old == *created { vec![] } else { vec![ChangeKitCommand::Created { created: old }] },
                ))
            }
            ChangeKitCommand::Updated { updated } => {
                let old = { kit.read().map_err(|_| SemioError::LockPoisoned("kit"))?.updated.clone() };
                kit.write()
                    .map_err(|_| SemioError::LockPoisoned("kit"))?
                    .set_updated(updated.clone())
                    .map_err(se)?;
                Ok((
                    KitChangeKind::SetKitMetadata,
                    if old == *updated { vec![] } else { vec![ChangeKitCommand::Updated { updated: old }] },
                ))
            }
            ChangeKitCommand::AddType { r#type } => {
                let id = r#type.id.clone();
                KitStore::insert_type_dto(kit, r#type.clone()).map_err(se)?;
                Ok((KitChangeKind::AddType, vec![ChangeKitCommand::RemoveType { type_id: TypeIdDto { id } }]))
            }
            ChangeKitCommand::RemoveType { type_id } => {
                let snap = KitStore::remove_type_dto(kit, type_id.id.as_str()).map_err(se)?;
                let inv = if let Some(dto) = snap {
                    vec![ChangeKitCommand::AddType { r#type: dto }]
                } else {
                    return Err(SemioError::NotFound {
                        kind: "Type",
                        id: type_id.id.clone(),
                    });
                };
                Ok((KitChangeKind::RemoveType, inv))
            }
            ChangeKitCommand::AddDesign { design } => {
                let id = design.id.clone();
                KitStore::insert_design_ref(kit, design.clone()).map_err(se)?;
                Ok((
                    KitChangeKind::AddDesign,
                    vec![ChangeKitCommand::RemoveDesign {
                        design_id: DesignIdDto { id },
                    }],
                ))
            }
            ChangeKitCommand::RemoveDesign { design_id } => {
                let snap = KitStore::remove_design_dto(kit, design_id.id.as_str()).map_err(se)?;
                let inv = if let Some(dto) = snap {
                    vec![ChangeKitCommand::AddDesign { design: dto }]
                } else {
                    return Err(SemioError::NotFound {
                        kind: "Design",
                        id: design_id.id.clone(),
                    });
                };
                Ok((KitChangeKind::RemoveDesign, inv))
            }
            ChangeKitCommand::AddFile { file } => {
                let id = file.id.clone();
                KitStore::insert_file_dto(kit, file.clone()).map_err(se)?;
                Ok((
                    KitChangeKind::ModifyType,
                    vec![ChangeKitCommand::RemoveFile { file_id: FileIdDto { id } }],
                ))
            }
            ChangeKitCommand::RemoveFile { file_id } => {
                let snap = KitStore::remove_file_dto(kit, file_id.id.as_str()).map_err(se)?;
                if let Some(d) = snap {
                    Ok((
                        KitChangeKind::ModifyType,
                        vec![ChangeKitCommand::AddFile { file: d }],
                    ))
                } else {
                    Err(SemioError::NotFound {
                        kind: "File",
                        id: file_id.id.clone(),
                    })
                }
            }
            ChangeKitCommand::AddFolder { folder } => {
                let id = folder.id.clone();
                KitStore::insert_folder_dto(kit, folder.clone()).map_err(se)?;
                Ok((
                    KitChangeKind::ModifyType,
                    vec![ChangeKitCommand::RemoveFolder { folder_id: FolderIdDto { id } }],
                ))
            }
            ChangeKitCommand::RemoveFolder { folder_id } => {
                let snap = KitStore::remove_folder_dto(kit, folder_id.id.as_str()).map_err(se)?;
                if let Some(d) = snap {
                    Ok((
                        KitChangeKind::ModifyType,
                        vec![ChangeKitCommand::AddFolder { folder: d }],
                    ))
                } else {
                    Err(SemioError::NotFound {
                        kind: "Folder",
                        id: folder_id.id.clone(),
                    })
                }
            }
            ChangeKitCommand::AddAuthor { author } => {
                let id = author.id.clone();
                KitStore::insert_author_dto(kit, author.clone()).map_err(se)?;
                Ok((
                    KitChangeKind::SetKitMetadata,
                    vec![ChangeKitCommand::RemoveAuthor { author_id: AuthorIdDto { id } }],
                ))
            }
            ChangeKitCommand::RemoveAuthor { author_id } => {
                let snap = KitStore::remove_author_dto(kit, author_id.id.as_str()).map_err(se)?;
                if let Some(d) = snap {
                    Ok((
                        KitChangeKind::SetKitMetadata,
                        vec![ChangeKitCommand::AddAuthor { author: d }],
                    ))
                } else {
                    Err(SemioError::NotFound {
                        kind: "Author",
                        id: author_id.id.clone(),
                    })
                }
            }
            ChangeKitCommand::AddConcept { concept } => {
                let id = concept.id.clone();
                KitStore::insert_concept_dto(kit, concept.clone()).map_err(se)?;
                Ok((
                    KitChangeKind::SetKitMetadata,
                    vec![ChangeKitCommand::RemoveConcept { concept_id: ConceptIdDto { id } }],
                ))
            }
            ChangeKitCommand::RemoveConcept { concept_id } => {
                let snap = KitStore::remove_concept_dto(kit, concept_id.id.as_str()).map_err(se)?;
                if let Some(d) = snap {
                    Ok((
                        KitChangeKind::SetKitMetadata,
                        vec![ChangeKitCommand::AddConcept { concept: d }],
                    ))
                } else {
                    Err(SemioError::NotFound {
                        kind: "Concept",
                        id: concept_id.id.clone(),
                    })
                }
            }
            ChangeKitCommand::AddTag { tag } => {
                let id = tag.id.clone();
                KitStore::insert_tag_dto(kit, tag.clone()).map_err(se)?;
                Ok((
                    KitChangeKind::SetKitMetadata,
                    vec![ChangeKitCommand::RemoveTag { tag_id: TagIdDto { id } }],
                ))
            }
            ChangeKitCommand::RemoveTag { tag_id } => {
                let snap = KitStore::remove_tag_dto(kit, tag_id.id.as_str()).map_err(se)?;
                if let Some(d) = snap {
                    Ok((
                        KitChangeKind::SetKitMetadata,
                        vec![ChangeKitCommand::AddTag { tag: d }],
                    ))
                } else {
                    Err(SemioError::NotFound {
                        kind: "Tag",
                        id: tag_id.id.clone(),
                    })
                }
            }
            ChangeKitCommand::AddQuality { quality } => {
                let id = quality.id.clone();
                KitStore::insert_quality_dto(kit, quality.clone()).map_err(se)?;
                Ok((
                    KitChangeKind::SetKitMetadata,
                    vec![ChangeKitCommand::RemoveQuality { quality_id: QualityIdDto { id } }],
                ))
            }
            ChangeKitCommand::RemoveQuality { quality_id } => {
                let snap = KitStore::remove_quality_dto(kit, quality_id.id.as_str()).map_err(se)?;
                if let Some(d) = snap {
                    Ok((
                        KitChangeKind::SetKitMetadata,
                        vec![ChangeKitCommand::AddQuality { quality: d }],
                    ))
                } else {
                    Err(SemioError::NotFound {
                        kind: "Quality",
                        id: quality_id.id.clone(),
                    })
                }
            }
            ChangeKitCommand::AddKitProp { prop: _ } => {
                // Kit-level free props (if used): append to kit.props
                return Err(SemioError::InvalidOperation("AddKitProp: use a concrete owner scope (piece/type/design) or extend KitStore".into()));
            }
            ChangeKitCommand::RemoveKitProp { .. } => {
                return Err(SemioError::InvalidOperation("RemoveKitProp: not yet wired to KitStore::props".into()));
            }
            ChangeKitCommand::AddKitAttribute { .. } | ChangeKitCommand::RemoveKitAttribute { .. } => {
                return Err(SemioError::InvalidOperation("kit-level attribute add/remove not yet wired".into()));
            }
            ChangeKitCommand::ChangeTypeCommands { type_id, commands } => {
                let mut inv_nested = Vec::new();
                for c in commands {
                    let inv = c.apply(kit, &type_id.id)?;
                    inv_nested.extend(inv);
                }
                let inv_nested: Vec<ChangeTypeCommand> = inv_nested.into_iter().rev().collect();
                Ok((
                    KitChangeKind::ModifyType,
                    vec![ChangeKitCommand::ChangeTypeCommands {
                        type_id: type_id.clone(),
                        commands: inv_nested,
                    }],
                ))
            }
            ChangeKitCommand::ChangeDesignCommands { design_id, commands } => {
                let mut inv_nested = Vec::new();
                for c in commands {
                    let inv = c.apply(kit, &design_id.id)?;
                    inv_nested.extend(inv);
                }
                let inv_nested: Vec<ChangeDesignCommand> = inv_nested.into_iter().rev().collect();
                Ok((
                    KitChangeKind::ModifyDesign,
                    vec![ChangeKitCommand::ChangeDesignCommands {
                        design_id: design_id.clone(),
                        commands: inv_nested,
                    }],
                ))
            }
            ChangeKitCommand::Other => Ok((KitChangeKind::Other("changeKit".into()), vec![])),
        }
    }

    /// Apply many commands in order; inverses are concatenated in **undo order** (last command's
    /// inverses first) so that applying the returned `Vec` once reverses the whole batch.
    pub fn apply_many(kit: &KitStoreRef, cmds: &[ChangeKitCommand]) -> Result<(KitChangeKind, Vec<ChangeKitCommand>)> {
        let mut kind = KitChangeKind::Inferred;
        let mut groups: Vec<Vec<ChangeKitCommand>> = Vec::with_capacity(cmds.len());
        for c in cmds {
            let (k, inv) = c.apply(kit)?;
            if k != KitChangeKind::Inferred {
                kind = k;
            }
            groups.push(inv);
        }
        let mut out = Vec::new();
        for g in groups.into_iter().rev() {
            out.extend(g);
        }
        Ok((kind, out))
    }
}

// NOTE: ChangeTypeCommand::apply, ChangeDesignCommand, etc. continue in a second part to stay under
// the editor size limit. See the remaining impls appended below.

impl ChangeTypeCommand {
    pub fn run(&self, kit: &KitStoreRef, type_id: &Id) -> Result<()> {
        self.apply(kit, type_id)?;
        Ok(())
    }
    /// Returns inverse fragments (forward order) — the caller will reverse for nesting.
    pub fn apply(&self, kit: &KitStoreRef, type_id: &Id) -> Result<Vec<ChangeTypeCommand>> {
        let t = kit
            .read()
            .map_err(|_| SemioError::LockPoisoned("kit"))?
            .semio_type(type_id.as_str())
            .ok_or_else(|| SemioError::NotFound {
                kind: "Type",
                id: type_id.clone(),
            })?;
        match self {
            ChangeTypeCommand::Name { name } => {
                let old = t.read().map_err(|_| SemioError::LockPoisoned("type"))?.name.clone();
                t.write().map_err(|_| SemioError::LockPoisoned("type"))?.set_name(name.clone()).map_err(se)?;
                if old == *name {
                    return Ok(vec![]);
                }
                Ok(vec![ChangeTypeCommand::Name { name: old }])
            }
            ChangeTypeCommand::Description { description } => {
                let old = t
                    .read()
                    .map_err(|_| SemioError::LockPoisoned("type"))?
                    .description
                    .clone();
                t.write()
                    .map_err(|_| SemioError::LockPoisoned("type"))?
                    .set_description(description.clone())
                    .map_err(se)?;
                if old == *description {
                    return Ok(vec![]);
                }
                Ok(vec![ChangeTypeCommand::Description { description: old }])
            }
            ChangeTypeCommand::Icon { icon } => {
                let old = t.read().map_err(|_| SemioError::LockPoisoned("type"))?.icon.clone();
                t.write()
                    .map_err(|_| SemioError::LockPoisoned("type"))?
                    .set_icon(icon.clone())
                    .map_err(se)?;
                if old == *icon {
                    return Ok(vec![]);
                }
                Ok(vec![ChangeTypeCommand::Icon { icon: old }])
            }
            ChangeTypeCommand::Image { image } => {
                let old = t.read().map_err(|_| SemioError::LockPoisoned("type"))?.image.clone();
                t.write()
                    .map_err(|_| SemioError::LockPoisoned("type"))?
                    .set_image(image.clone())
                    .map_err(se)?;
                if old == *image {
                    return Ok(vec![]);
                }
                Ok(vec![ChangeTypeCommand::Image { image: old }])
            }
            ChangeTypeCommand::Variant { variant } => {
                let old = t
                    .read()
                    .map_err(|_| SemioError::LockPoisoned("type"))?
                    .variant
                    .clone();
                t.write()
                    .map_err(|_| SemioError::LockPoisoned("type"))?
                    .set_variant(variant.clone())
                    .map_err(se)?;
                if old == *variant {
                    return Ok(vec![]);
                }
                Ok(vec![ChangeTypeCommand::Variant { variant: old }])
            }
            ChangeTypeCommand::Stock { stock } => {
                let old = t.read().map_err(|_| SemioError::LockPoisoned("type"))?.stock;
                t.write()
                    .map_err(|_| SemioError::LockPoisoned("type"))?
                    .set_stock(*stock)
                    .map_err(se)?;
                if old == *stock {
                    return Ok(vec![]);
                }
                Ok(vec![ChangeTypeCommand::Stock { stock: old }])
            }
            ChangeTypeCommand::TypeVirtual { value: virt } => {
                let old = t.read().map_err(|_| SemioError::LockPoisoned("type"))?.virtual_;
                t.write()
                    .map_err(|_| SemioError::LockPoisoned("type"))?
                    .set_virtual(*virt)
                    .map_err(se)?;
                if old == *virt {
                    return Ok(vec![]);
                }
                Ok(vec![ChangeTypeCommand::TypeVirtual { value: old }])
            }
            ChangeTypeCommand::Unit { unit } => {
                let old = t.read().map_err(|_| SemioError::LockPoisoned("type"))?.unit.clone();
                t.write()
                    .map_err(|_| SemioError::LockPoisoned("type"))?
                    .set_unit(unit.clone())
                    .map_err(se)?;
                if old == *unit {
                    return Ok(vec![]);
                }
                Ok(vec![ChangeTypeCommand::Unit { unit: old }])
            }
            ChangeTypeCommand::Location { location } => {
                let old = t.read().map_err(|_| SemioError::LockPoisoned("type"))?.location;
                t.write()
                    .map_err(|_| SemioError::LockPoisoned("type"))?
                    .set_location(*location)
                    .map_err(se)?;
                if old == *location {
                    return Ok(vec![]);
                }
                Ok(vec![ChangeTypeCommand::Location { location: old }])
            }
            ChangeTypeCommand::Created { created } => {
                let old = t.read().map_err(|_| SemioError::LockPoisoned("type"))?.created.clone();
                t.write()
                    .map_err(|_| SemioError::LockPoisoned("type"))?
                    .set_created(created.clone())
                    .map_err(se)?;
                if old == *created {
                    return Ok(vec![]);
                }
                Ok(vec![ChangeTypeCommand::Created { created: old }])
            }
            ChangeTypeCommand::Updated { updated } => {
                let old = t.read().map_err(|_| SemioError::LockPoisoned("type"))?.updated.clone();
                t.write()
                    .map_err(|_| SemioError::LockPoisoned("type"))?
                    .set_updated(updated.clone())
                    .map_err(se)?;
                if old == *updated {
                    return Ok(vec![]);
                }
                Ok(vec![ChangeTypeCommand::Updated { updated: old }])
            }
            ChangeTypeCommand::AddPort { .. } => Err(SemioError::InvalidOperation("ChangeTypeCommand::AddPort: use extend TypeStore.ports in a follow-up".into())),
            ChangeTypeCommand::RemovePort { .. } => Err(SemioError::InvalidOperation("ChangeTypeCommand::RemovePort: not yet wired".into())),
            ChangeTypeCommand::ChangePortCommands { .. } => Err(SemioError::InvalidOperation("ChangeTypeCommand::ChangePortCommands: not yet wired".into())),
            ChangeTypeCommand::AddConnector { .. } | ChangeTypeCommand::RemoveConnector { .. } | ChangeTypeCommand::ChangeConnectorCommands { .. } => {
                Err(SemioError::InvalidOperation("connector change commands: not yet wired".into()))
            }
            ChangeTypeCommand::AddRepresentation { .. } | ChangeTypeCommand::RemoveRepresentation { .. } | ChangeTypeCommand::ChangeRepresentationCommands { .. } => {
                Err(SemioError::InvalidOperation("representation change commands: not yet wired".into()))
            }
            ChangeTypeCommand::AddTypeAuthor { .. }
            | ChangeTypeCommand::RemoveTypeAuthor { .. }
            | ChangeTypeCommand::AddTypeConcept { .. }
            | ChangeTypeCommand::RemoveTypeConcept { .. }
            | ChangeTypeCommand::AddTypeTag { .. }
            | ChangeTypeCommand::RemoveTypeTag { .. }
            | ChangeTypeCommand::AddTypeQuality { .. }
            | ChangeTypeCommand::RemoveTypeQuality { .. }
            | ChangeTypeCommand::AddTypeProp { .. }
            | ChangeTypeCommand::RemoveTypeProp { .. }
            | ChangeTypeCommand::AddTypeAttribute { .. }
            | ChangeTypeCommand::RemoveTypeAttribute { .. } => {
                Err(SemioError::InvalidOperation("type-embedded child entity commands: not yet wired (push onto TypeStore child vecs)".into()))
            }
            ChangeTypeCommand::Other => Ok(vec![]),
        }
    }
}

include!("change_command_ext2.rs");
