//! Machine-generated read command surface: exhaustive commands + outputs, live `KitGraph` execution.
// Regenerate: `python gen_read_module.py` from semio/rs (optional; hand-edit for tweaks).

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::attribute::{
    AttributeFullDto, AttributeMetadataDto, AttributeShallowDto, AttributeIdDto,
};
use crate::author::{AuthorFullDto, AuthorMetadataDto, AuthorShallowDto, AuthorIdDto};
use crate::benchmark::{BenchmarkFullDto, BenchmarkMetadataDto, BenchmarkShallowDto, BenchmarkIdDto};
use crate::concept::{ConceptFullDto, ConceptMetadataDto, ConceptShallowDto, ConceptIdDto};
use crate::connection::{
    ConnectionFullDto, ConnectionMetadataDto, ConnectionShallowDto, ConnectionIdDto, ConnectionStoreRef,
};
use crate::connector::{
    ConnectorFullDto, ConnectorMetadataDto, ConnectorShallowDto, ConnectorIdDto, ConnectorStoreRef,
};
use crate::design::{
    DesignFullDto, DesignMetadataDto, DesignShallowDto, DesignIdDto, DesignStoreRef,
};
use crate::family::{FamilyFullDto, FamilyMetadataDto, FamilyShallowDto, FamilyIdDto, FamilyStoreRef};
use crate::file::{FileFullDto, FileMetadataDto, FileShallowDto, FileIdDto};
use crate::folder::{FolderFullDto, FolderMetadataDto, FolderShallowDto, FolderIdDto};
use crate::geom::{Coordinate, Plane};
use crate::group::{GroupFullDto, GroupMetadataDto, GroupShallowDto, GroupIdDto};
use crate::kit_graph::{KitFullDto, KitIdDto, KitMetadataDto, KitShallowDto, KitGraph};
use crate::layer::{LayerFullDto, LayerMetadataDto, LayerShallowDto, LayerIdDto};
use crate::location::{LocationFullDto, LocationShallowDto, LocationIdDto, LocationMetadataDto};
use crate::piece::{
    FixedPieceOutputDto, PieceAlternativesDto, PieceFullDto, PieceMetadataDto, PieceShallowDto, PieceIdDto,
    PieceStoreRef, ConnectedPieceOutputDto, PoseFullDto,
};
use crate::port::{PortFullDto, PortMetadataDto, PortShallowDto, PortIdDto, PortStoreRef};
use crate::prop::{PropFullDto, PropShallowDto, PropIdDto};
use crate::quality::{QualityFullDto, QualityMetadataDto, QualityShallowDto, QualityIdDto};
use crate::representation::{
    RepresentationFullDto, RepresentationMetadataDto, RepresentationShallowDto, RepresentationIdDto, RepresentationStoreRef,
};
use crate::side::{SideFullDto, SideIdDto, SideMetadataDto, SideShallowDto, SideStoreRef};
use crate::stat::{StatFullDto, StatMetadataDto, StatShallowDto, StatIdDto};
use crate::tag::{TagFullDto, TagMetadataDto, TagShallowDto, TagIdDto};
use crate::typ::{TypeFullDto, TypeMetadataDto, TypeShallowDto, TypeIdDto, TypeStoreRef};
use crate::id::Id;
use crate::{error::Result, error::SemioError};

/// One row of [`crate::design::DesignStore::flatten_map`].
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DesignFlattenMapEntryDto {
    pub piece_id: Id,
    pub plane: Plane,
    pub center: Coordinate,
}

fn nf<K: AsRef<str>>(kind: &'static str, id: K) -> SemioError {
    SemioError::NotFound { kind: kind.into(), id: Id::from(id.as_ref()) }
}

// --- Kit ---

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ReadKitCommand {
    ReadKitFullCommand,
    ReadKitShallowCommand,
    ReadKitMetadataCommand,
    ReadKitIdCommand,
    ReadKitNameCommand,
    ReadKitDescriptionCommand,
    ReadKitIconCommand,
    ReadKitImageCommand,
    ReadKitPreviewCommand,
    ReadKitRemoteCommand,
    ReadKitHomepageCommand,
    ReadKitLicenseCommand,
    ReadKitUriCommand,
    ReadKitCreatedCommand,
    ReadKitUpdatedCommand,
    ReadKitTypesFullCommand,
    ReadKitTypesShallowCommand,
    ReadKitDesignsFullCommand,
    ReadKitDesignsShallowCommand,
    ReadKitFilesFullCommand,
    ReadKitFilesShallowCommand,
    ReadKitFoldersFullCommand,
    ReadKitFoldersShallowCommand,
    ReadKitLocationsFullCommand,
    ReadKitLocationsShallowCommand,
    ReadKitFamiliesFullCommand,
    ReadKitFamiliesShallowCommand,
    ReadKitPortsFullCommand,
    ReadKitAuthorsFullCommand,
    ReadKitAuthorsShallowCommand,
    ReadKitConceptsFullCommand,
    ReadKitConceptsShallowCommand,
    ReadKitTagsFullCommand,
    ReadKitTagsShallowCommand,
    ReadKitQualitiesFullCommand,
    ReadKitQualitiesShallowCommand,
    ReadKitPropsFullCommand,
    ReadKitPropsShallowCommand,
    ReadKitAttributesFullCommand,
    ReadKitAttributesShallowCommand,
    ReadKitTypeCommands { id: TypeIdDto, commands: Vec<ReadTypeCommand> },
    ReadKitDesignCommands { id: DesignIdDto, commands: Vec<ReadDesignCommand> },
    ReadKitFileCommands { id: FileIdDto, commands: Vec<ReadFileCommand> },
    ReadKitFolderCommands { id: FolderIdDto, commands: Vec<ReadFolderCommand> },
    ReadKitLocationCommands { id: LocationIdDto, commands: Vec<ReadLocationCommand> },
    ReadKitFamilyCommands { id: FamilyIdDto, commands: Vec<ReadFamilyCommand> },
    ReadKitPortCommands { id: PortIdDto, commands: Vec<ReadPortCommand> },
    ReadKitAuthorCommands { id: AuthorIdDto, commands: Vec<ReadAuthorCommand> },
    ReadKitConceptCommands { id: ConceptIdDto, commands: Vec<ReadConceptCommand> },
    ReadKitTagCommands { id: TagIdDto, commands: Vec<ReadTagCommand> },
    ReadKitQualityCommands { id: QualityIdDto, commands: Vec<ReadQualityCommand> },
    ReadKitPropCommands { id: PropIdDto, commands: Vec<ReadPropCommand> },
    ReadKitAttributeCommands { id: AttributeIdDto, commands: Vec<ReadAttributeCommand> },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ReadKitCommandOutput {
    ReadKitFullCommand { kit: KitFullDto },
    ReadKitShallowCommand { kit: KitShallowDto },
    ReadKitMetadataCommand { metadata: KitMetadataDto },
    ReadKitIdCommand { id: KitIdDto },
    ReadKitNameCommand { name: String },
    ReadKitDescriptionCommand { description: Option<String> },
    ReadKitIconCommand { icon: Option<String> },
    ReadKitImageCommand { image: Option<String> },
    ReadKitPreviewCommand { preview: Option<String> },
    ReadKitRemoteCommand { remote: Option<String> },
    ReadKitHomepageCommand { homepage: Option<String> },
    ReadKitLicenseCommand { license: Option<String> },
    ReadKitUriCommand { uri: Option<String> },
    ReadKitCreatedCommand { created: Option<String> },
    ReadKitUpdatedCommand { updated: Option<String> },
    ReadKitTypesFullCommand { types: Vec<TypeFullDto> },
    ReadKitTypesShallowCommand { types: Vec<TypeShallowDto> },
    ReadKitDesignsFullCommand { designs: Vec<DesignFullDto> },
    ReadKitDesignsShallowCommand { designs: Vec<DesignShallowDto> },
    ReadKitFilesFullCommand { files: Vec<FileFullDto> },
    ReadKitFilesShallowCommand { files: Vec<FileShallowDto> },
    ReadKitFoldersFullCommand { folders: Vec<FolderFullDto> },
    ReadKitFoldersShallowCommand { folders: Vec<FolderShallowDto> },
    ReadKitLocationsFullCommand { locations: Vec<LocationFullDto> },
    ReadKitLocationsShallowCommand { locations: Vec<LocationShallowDto> },
    ReadKitFamiliesFullCommand { families: Vec<FamilyFullDto> },
    ReadKitFamiliesShallowCommand { families: Vec<FamilyShallowDto> },
    ReadKitPortsFullCommand { ports: Vec<PortFullDto> },
    ReadKitAuthorsFullCommand { authors: Vec<AuthorFullDto> },
    ReadKitAuthorsShallowCommand { authors: Vec<AuthorShallowDto> },
    ReadKitConceptsFullCommand { concepts: Vec<ConceptFullDto> },
    ReadKitConceptsShallowCommand { concepts: Vec<ConceptShallowDto> },
    ReadKitTagsFullCommand { tags: Vec<TagFullDto> },
    ReadKitTagsShallowCommand { tags: Vec<TagShallowDto> },
    ReadKitQualitiesFullCommand { qualities: Vec<QualityFullDto> },
    ReadKitQualitiesShallowCommand { qualities: Vec<QualityShallowDto> },
    ReadKitPropsFullCommand { props: Vec<PropFullDto> },
    ReadKitPropsShallowCommand { props: Vec<PropShallowDto> },
    ReadKitAttributesFullCommand { attributes: Vec<AttributeFullDto> },
    ReadKitAttributesShallowCommand { attributes: Vec<AttributeShallowDto> },
    ReadKitTypeCommands { results: Vec<ReadTypeCommandOutput> },
    ReadKitDesignCommands { results: Vec<ReadDesignCommandOutput> },
    ReadKitFileCommands { results: Vec<ReadFileCommandOutput> },
    ReadKitFolderCommands { results: Vec<ReadFolderCommandOutput> },
    ReadKitLocationCommands { results: Vec<ReadLocationCommandOutput> },
    ReadKitFamilyCommands { results: Vec<ReadFamilyCommandOutput> },
    ReadKitPortCommands { results: Vec<ReadPortCommandOutput> },
    ReadKitAuthorCommands { results: Vec<ReadAuthorCommandOutput> },
    ReadKitConceptCommands { results: Vec<ReadConceptCommandOutput> },
    ReadKitTagCommands { results: Vec<ReadTagCommandOutput> },
    ReadKitQualityCommands { results: Vec<ReadQualityCommandOutput> },
    ReadKitPropCommands { results: Vec<ReadPropCommandOutput> },
    ReadKitAttributeCommands { results: Vec<ReadAttributeCommandOutput> },
}

// --- Type ---

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ReadTypeCommand {
    ReadTypeFullCommand,
    ReadTypeShallowCommand,
    ReadTypeMetadataCommand,
    ReadTypeIdCommand,
    ReadTypeNameCommand,
    ReadTypeDescriptionCommand,
    ReadTypeIconCommand,
    ReadTypeImageCommand,
    ReadTypeStockCommand,
    ReadTypeVirtualCommand,
    ReadTypeUnitCommand,
    ReadTypeLocationCommand,
    ReadTypeCreatedCommand,
    ReadTypeUpdatedCommand,
    ReadTypeFamiliesCommand,
    ReadTypeConnectorsFullCommand,
    ReadTypeConnectorsShallowCommand,
    ReadTypeRepresentationsFullCommand,
    ReadTypeRepresentationsShallowCommand,
    ReadTypeAuthorsFullCommand,
    ReadTypeAuthorsShallowCommand,
    ReadTypeConceptsFullCommand,
    ReadTypeConceptsShallowCommand,
    ReadTypeTagsFullCommand,
    ReadTypeTagsShallowCommand,
    ReadTypeQualitiesFullCommand,
    ReadTypeQualitiesShallowCommand,
    ReadTypePropsFullCommand,
    ReadTypePropsShallowCommand,
    ReadTypeAttributesFullCommand,
    ReadTypeAttributesShallowCommand,
    /// All ports on families referenced by this type (see [`TypeStore::port`] for single-id resolution).
    ReadTypePortsFullCommand,
    ReadTypeConnectorForPortIdCommand { port_id: PortIdDto },
    ReadTypeFamilyCommands { id: FamilyIdDto, commands: Vec<ReadFamilyCommand> },
    ReadTypeConnectorCommands { id: ConnectorIdDto, commands: Vec<ReadConnectorCommand> },
    ReadTypeRepresentationCommands { id: RepresentationIdDto, commands: Vec<ReadRepresentationCommand> },
    ReadTypePortCommands { id: PortIdDto, commands: Vec<ReadPortCommand> },
    ReadTypeAuthorCommands { id: AuthorIdDto, commands: Vec<ReadAuthorCommand> },
    ReadTypeConceptCommands { id: ConceptIdDto, commands: Vec<ReadConceptCommand> },
    ReadTypeTagCommands { id: TagIdDto, commands: Vec<ReadTagCommand> },
    ReadTypeQualityCommands { id: QualityIdDto, commands: Vec<ReadQualityCommand> },
    ReadTypePropCommands { id: PropIdDto, commands: Vec<ReadPropCommand> },
    ReadTypeAttributeCommands { id: AttributeIdDto, commands: Vec<ReadAttributeCommand> },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ReadTypeCommandOutput {
    ReadTypeFullCommand { dto: TypeFullDto },
    ReadTypeShallowCommand { dto: TypeShallowDto },
    ReadTypeMetadataCommand { metadata: TypeMetadataDto },
    ReadTypeIdCommand { id: TypeIdDto },
    ReadTypeNameCommand { name: String },
    ReadTypeDescriptionCommand { description: Option<String> },
    ReadTypeIconCommand { icon: Option<String> },
    ReadTypeImageCommand { image: Option<String> },
    ReadTypeStockCommand { stock: Option<i64> },
    ReadTypeVirtualCommand { r#virtual: Option<bool> },
    ReadTypeUnitCommand { unit: Option<String> },
    ReadTypeLocationCommand { location: Option<crate::location::LocationIdDto> },
    ReadTypeCreatedCommand { created: Option<String> },
    ReadTypeUpdatedCommand { updated: Option<String> },
    ReadTypeFamiliesCommand { families: Vec<FamilyIdDto> },
    ReadTypeConnectorsFullCommand { connectors: Vec<ConnectorFullDto> },
    ReadTypeConnectorsShallowCommand { connectors: Vec<ConnectorShallowDto> },
    ReadTypeRepresentationsFullCommand { representations: Vec<RepresentationFullDto> },
    ReadTypeRepresentationsShallowCommand { representations: Vec<RepresentationShallowDto> },
    ReadTypeAuthorsFullCommand { authors: Vec<AuthorFullDto> },
    ReadTypeAuthorsShallowCommand { authors: Vec<AuthorShallowDto> },
    ReadTypeConceptsFullCommand { concepts: Vec<ConceptFullDto> },
    ReadTypeConceptsShallowCommand { concepts: Vec<ConceptShallowDto> },
    ReadTypeTagsFullCommand { tags: Vec<TagFullDto> },
    ReadTypeTagsShallowCommand { tags: Vec<TagShallowDto> },
    ReadTypeQualitiesFullCommand { qualities: Vec<QualityFullDto> },
    ReadTypeQualitiesShallowCommand { qualities: Vec<QualityShallowDto> },
    ReadTypePropsFullCommand { props: Vec<PropFullDto> },
    ReadTypePropsShallowCommand { props: Vec<PropShallowDto> },
    ReadTypeAttributesFullCommand { attributes: Vec<AttributeFullDto> },
    ReadTypeAttributesShallowCommand { attributes: Vec<AttributeShallowDto> },
    ReadTypePortsFullCommand { ports: Vec<PortFullDto> },
    ReadTypeConnectorForPortIdCommand { connector: Option<ConnectorFullDto> },
    ReadTypeFamilyCommands { results: Vec<ReadFamilyCommandOutput> },
    ReadTypeConnectorCommands { results: Vec<ReadConnectorCommandOutput> },
    ReadTypeRepresentationCommands { results: Vec<ReadRepresentationCommandOutput> },
    ReadTypePortCommands { results: Vec<ReadPortCommandOutput> },
    ReadTypeAuthorCommands { results: Vec<ReadAuthorCommandOutput> },
    ReadTypeConceptCommands { results: Vec<ReadConceptCommandOutput> },
    ReadTypeTagCommands { results: Vec<ReadTagCommandOutput> },
    ReadTypeQualityCommands { results: Vec<ReadQualityCommandOutput> },
    ReadTypePropCommands { results: Vec<ReadPropCommandOutput> },
    ReadTypeAttributeCommands { results: Vec<ReadAttributeCommandOutput> },
}

// --- Design ---

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ReadDesignCommand {
    ReadDesignFullCommand,
    ReadDesignShallowCommand,
    ReadDesignMetadataCommand,
    ReadDesignIdCommand,
    ReadDesignNameCommand,
    ReadDesignDescriptionCommand,
    ReadDesignIconCommand,
    ReadDesignImageCommand,
    ReadDesignLocationCommand,
    ReadDesignUnitCommand,
    ReadDesignCreatedCommand,
    ReadDesignUpdatedCommand,
    ReadDesignKitCommand,
    ReadDesignFamiliesCommand,
    ReadDesignPiecesFullCommand,
    ReadDesignPiecesShallowCommand,
    ReadDesignConnectionsFullCommand,
    ReadDesignConnectionsShallowCommand,
    ReadDesignLayersFullCommand,
    ReadDesignLayersShallowCommand,
    ReadDesignGroupsFullCommand,
    ReadDesignGroupsShallowCommand,
    ReadDesignAuthorsFullCommand,
    ReadDesignAuthorsShallowCommand,
    ReadDesignConceptsFullCommand,
    ReadDesignConceptsShallowCommand,
    ReadDesignTagsFullCommand,
    ReadDesignTagsShallowCommand,
    ReadDesignQualitiesFullCommand,
    ReadDesignQualitiesShallowCommand,
    ReadDesignPropsFullCommand,
    ReadDesignPropsShallowCommand,
    ReadDesignAttributesFullCommand,
    ReadDesignAttributesShallowCommand,
    ReadDesignStatsFullCommand,
    ReadDesignStatsShallowCommand,
    ReadDesignFlattenMapCommand,
    ReadDesignFamilyCommands { id: FamilyIdDto, commands: Vec<ReadFamilyCommand> },
    ReadDesignPieceCommands { id: PieceIdDto, commands: Vec<ReadPieceCommand> },
    ReadDesignConnectionCommands { id: ConnectionIdDto, commands: Vec<ReadConnectionCommand> },
    ReadDesignLayerCommands { id: LayerIdDto, commands: Vec<ReadLayerCommand> },
    ReadDesignGroupCommands { id: GroupIdDto, commands: Vec<ReadGroupCommand> },
    ReadDesignAuthorCommands { id: AuthorIdDto, commands: Vec<ReadAuthorCommand> },
    ReadDesignConceptCommands { id: ConceptIdDto, commands: Vec<ReadConceptCommand> },
    ReadDesignTagCommands { id: TagIdDto, commands: Vec<ReadTagCommand> },
    ReadDesignQualityCommands { id: QualityIdDto, commands: Vec<ReadQualityCommand> },
    ReadDesignPropCommands { id: PropIdDto, commands: Vec<ReadPropCommand> },
    ReadDesignAttributeCommands { id: AttributeIdDto, commands: Vec<ReadAttributeCommand> },
    ReadDesignStatCommands { id: StatIdDto, commands: Vec<ReadStatCommand> },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ReadDesignCommandOutput {
    ReadDesignFullCommand { dto: DesignFullDto },
    ReadDesignShallowCommand { dto: DesignShallowDto },
    ReadDesignMetadataCommand { metadata: DesignMetadataDto },
    ReadDesignIdCommand { id: DesignIdDto },
    ReadDesignNameCommand { name: String },
    ReadDesignDescriptionCommand { description: Option<String> },
    ReadDesignIconCommand { icon: Option<String> },
    ReadDesignImageCommand { image: Option<String> },
    ReadDesignLocationCommand { location: Option<crate::location::LocationIdDto> },
    ReadDesignUnitCommand { unit: Option<String> },
    ReadDesignCreatedCommand { created: Option<String> },
    ReadDesignUpdatedCommand { updated: Option<String> },
    ReadDesignKitCommand { kit: Option<crate::kit_graph::KitIdDto> },
    ReadDesignFamiliesCommand { families: Vec<FamilyIdDto> },
    ReadDesignPiecesFullCommand { pieces: Vec<PieceFullDto> },
    ReadDesignPiecesShallowCommand { pieces: Vec<PieceShallowDto> },
    ReadDesignConnectionsFullCommand { connections: Vec<ConnectionFullDto> },
    ReadDesignConnectionsShallowCommand { connections: Vec<ConnectionShallowDto> },
    ReadDesignLayersFullCommand { layers: Vec<LayerFullDto> },
    ReadDesignLayersShallowCommand { layers: Vec<LayerShallowDto> },
    ReadDesignGroupsFullCommand { groups: Vec<GroupFullDto> },
    ReadDesignGroupsShallowCommand { groups: Vec<GroupShallowDto> },
    ReadDesignAuthorsFullCommand { authors: Vec<AuthorFullDto> },
    ReadDesignAuthorsShallowCommand { authors: Vec<AuthorShallowDto> },
    ReadDesignConceptsFullCommand { concepts: Vec<ConceptFullDto> },
    ReadDesignConceptsShallowCommand { concepts: Vec<ConceptShallowDto> },
    ReadDesignTagsFullCommand { tags: Vec<TagFullDto> },
    ReadDesignTagsShallowCommand { tags: Vec<TagShallowDto> },
    ReadDesignQualitiesFullCommand { qualities: Vec<QualityFullDto> },
    ReadDesignQualitiesShallowCommand { qualities: Vec<QualityShallowDto> },
    ReadDesignPropsFullCommand { props: Vec<PropFullDto> },
    ReadDesignPropsShallowCommand { props: Vec<PropShallowDto> },
    ReadDesignAttributesFullCommand { attributes: Vec<AttributeFullDto> },
    ReadDesignAttributesShallowCommand { attributes: Vec<AttributeShallowDto> },
    ReadDesignStatsFullCommand { stats: Vec<StatFullDto> },
    ReadDesignStatsShallowCommand { stats: Vec<StatShallowDto> },
    ReadDesignFlattenMapCommand { entries: Vec<DesignFlattenMapEntryDto> },
    ReadDesignFamilyCommands { results: Vec<ReadFamilyCommandOutput> },
    ReadDesignPieceCommands { results: Vec<ReadPieceCommandOutput> },
    ReadDesignConnectionCommands { results: Vec<ReadConnectionCommandOutput> },
    ReadDesignLayerCommands { results: Vec<ReadLayerCommandOutput> },
    ReadDesignGroupCommands { results: Vec<ReadGroupCommandOutput> },
    ReadDesignAuthorCommands { results: Vec<ReadAuthorCommandOutput> },
    ReadDesignConceptCommands { results: Vec<ReadConceptCommandOutput> },
    ReadDesignTagCommands { results: Vec<ReadTagCommandOutput> },
    ReadDesignQualityCommands { results: Vec<ReadQualityCommandOutput> },
    ReadDesignPropCommands { results: Vec<ReadPropCommandOutput> },
    ReadDesignAttributeCommands { results: Vec<ReadAttributeCommandOutput> },
    ReadDesignStatCommands { results: Vec<ReadStatCommandOutput> },
}

// --- Piece ---

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ReadPieceCommand {
    ReadPieceFullCommand,
    ReadPieceShallowCommand,
    ReadPieceMetadataCommand,
    ReadPieceIdCommand,
    ReadPieceNameCommand,
    ReadPieceDescriptionCommand,
    ReadPiecePlaneCommand,
    ReadPieceCenterCommand,
    ReadPieceScaleCommand,
    ReadPieceMirrorPlaneCommand,
    ReadPieceHiddenCommand,
    ReadPieceLockedCommand,
    ReadPieceColorCommand,
    ReadPieceTypeCommand,
    ReadPieceDesignCommand,
    ReadPiecePropsFullCommand,
    ReadPiecePropsShallowCommand,
    ReadPieceAttributesFullCommand,
    ReadPieceAttributesShallowCommand,
    ReadPieceFlatPlaneCommand,
    ReadPieceFlatCenterCommand,
    ReadPieceFlatPoseCommand,
    ReadPiecePathCommand,
    ReadPieceParentPieceIdCommand,
    ReadPieceParentConnectionIdCommand,
    ReadPieceParentDesignIdCommand,
    ReadPieceFixedCommand,
    ReadPieceConnectedCommand,
    ReadPieceAlternativesCommand,
    ReadPieceAlternativeTypesCommand,
    ReadPieceAlternativeDesignsCommand,
    ReadPiecePropCommands { id: PropIdDto, commands: Vec<ReadPropCommand> },
    ReadPieceAttributeCommands { id: AttributeIdDto, commands: Vec<ReadAttributeCommand> },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ReadPieceCommandOutput {
    ReadPieceFullCommand { dto: PieceFullDto },
    ReadPieceShallowCommand { dto: PieceShallowDto },
    ReadPieceMetadataCommand { metadata: PieceMetadataDto },
    ReadPieceIdCommand { id: PieceIdDto },
    ReadPieceNameCommand { name: Option<String> },
    ReadPieceDescriptionCommand { description: Option<String> },
    ReadPiecePlaneCommand { plane: Option<Plane> },
    ReadPieceCenterCommand { center: Option<Coordinate> },
    ReadPieceScaleCommand { scale: Option<f64> },
    ReadPieceMirrorPlaneCommand { mirror_plane: Option<Plane> },
    ReadPieceHiddenCommand { hidden: Option<bool> },
    ReadPieceLockedCommand { locked: Option<bool> },
    ReadPieceColorCommand { color: Option<String> },
    ReadPieceTypeCommand { r#type: Option<TypeIdDto> },
    ReadPieceDesignCommand { design: Option<DesignIdDto> },
    ReadPiecePropsFullCommand { props: Vec<PropFullDto> },
    ReadPiecePropsShallowCommand { props: Vec<PropShallowDto> },
    ReadPieceAttributesFullCommand { attributes: Vec<AttributeFullDto> },
    ReadPieceAttributesShallowCommand { attributes: Vec<AttributeShallowDto> },
    ReadPieceFlatPlaneCommand { flat_plane: Plane },
    ReadPieceFlatCenterCommand { flat_center: Coordinate },
    ReadPieceFlatPoseCommand { flat_pose: PoseFullDto },
    ReadPiecePathCommand { path: Vec<PieceIdDto> },
    ReadPieceParentPieceIdCommand { parent_piece: Option<PieceIdDto> },
    ReadPieceParentConnectionIdCommand { parent_connection: Option<ConnectionIdDto> },
    ReadPieceParentDesignIdCommand { parent_design: DesignIdDto },
    ReadPieceFixedCommand { fixed: FixedPieceOutputDto },
    ReadPieceConnectedCommand { connected: ConnectedPieceOutputDto },
    ReadPieceAlternativesCommand { alternatives: PieceAlternativesDto },
    ReadPieceAlternativeTypesCommand { types: Vec<TypeIdDto> },
    ReadPieceAlternativeDesignsCommand { designs: Vec<DesignIdDto> },
    ReadPiecePropCommands { results: Vec<ReadPropCommandOutput> },
    ReadPieceAttributeCommands { results: Vec<ReadAttributeCommandOutput> },
}

// --- Connection ---

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ReadConnectionCommand {
    ReadConnectionFullCommand,
    ReadConnectionShallowCommand,
    ReadConnectionMetadataCommand,
    ReadConnectionIdCommand,
    ReadConnectionConnectedSideMetadataCommand,
    ReadConnectionConnectingSideMetadataCommand,
    ReadConnectionConnectedSideFullCommand,
    ReadConnectionConnectingSideFullCommand,
    ReadConnectionGapCommand,
    ReadConnectionShiftCommand,
    ReadConnectionRiseCommand,
    ReadConnectionRotationCommand,
    ReadConnectionTurnCommand,
    ReadConnectionTiltCommand,
    ReadConnectionUCommand,
    ReadConnectionVCommand,
    ReadConnectionDescriptionCommand,
    ReadConnectionAttributesFullCommand,
    ReadConnectionAttributesShallowCommand,
    ReadConnectionChildPlaneMatrixCommand,
    ReadConnectionFlatSidesForChildCommand { child_piece_id: PieceIdDto },
    ReadConnectionAttributeCommands { id: AttributeIdDto, commands: Vec<ReadAttributeCommand> },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ReadConnectionCommandOutput {
    ReadConnectionFullCommand { dto: ConnectionFullDto },
    ReadConnectionShallowCommand { dto: ConnectionShallowDto },
    ReadConnectionMetadataCommand { metadata: ConnectionMetadataDto },
    ReadConnectionIdCommand { id: ConnectionIdDto },
    ReadConnectionConnectedSideMetadataCommand { side: SideMetadataDto },
    ReadConnectionConnectingSideMetadataCommand { side: SideMetadataDto },
    ReadConnectionConnectedSideFullCommand { side: SideFullDto },
    ReadConnectionConnectingSideFullCommand { side: SideFullDto },
    ReadConnectionGapCommand { gap: Option<f64> },
    ReadConnectionShiftCommand { shift: Option<f64> },
    ReadConnectionRiseCommand { rise: Option<f64> },
    ReadConnectionRotationCommand { rotation: Option<f64> },
    ReadConnectionTurnCommand { turn: Option<f64> },
    ReadConnectionTiltCommand { tilt: Option<f64> },
    ReadConnectionUCommand { u: Option<f64> },
    ReadConnectionVCommand { v: Option<f64> },
    ReadConnectionDescriptionCommand { description: Option<String> },
    ReadConnectionAttributesFullCommand { attributes: Vec<AttributeFullDto> },
    ReadConnectionAttributesShallowCommand { attributes: Vec<AttributeShallowDto> },
    ReadConnectionChildPlaneMatrixCommand { matrix: [[f64; 4]; 4] },
    ReadConnectionFlatSidesForChildCommand { connected: SideFullDto, connecting: SideFullDto },
    ReadConnectionAttributeCommands { results: Vec<ReadAttributeCommandOutput> },
}

// --- Side ---

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ReadSideCommand {
    ReadSideFullCommand,
    ReadSideShallowCommand,
    ReadSideMetadataCommand,
    ReadSideIdCommand,
    ReadSidePieceIdCommand,
    ReadSidePortIdCommand,
    ReadSideDesignPieceIdCommand,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ReadSideCommandOutput {
    ReadSideFullCommand { side: SideFullDto },
    ReadSideShallowCommand { side: SideShallowDto },
    ReadSideMetadataCommand { side: SideMetadataDto },
    ReadSideIdCommand { id: SideIdDto },
    ReadSidePieceIdCommand { piece: PieceIdDto },
    ReadSidePortIdCommand { port: Option<PortIdDto> },
    ReadSideDesignPieceIdCommand { design_piece: Option<PieceIdDto> },
}

// --- Port ---

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ReadPortCommand {
    ReadPortFullCommand,
    ReadPortShallowCommand,
    ReadPortMetadataCommand,
    ReadPortIdCommand,
    ReadPortNameCommand,
    ReadPortDescriptionCommand,
    ReadPortIconCommand,
    ReadPortCompatibleFamiliesCommand,
    ReadPortMandatoryCommand,
    ReadPortTCommand,
    ReadPortPointCommand,
    ReadPortDirectionCommand,
    ReadPortCompatiblePortsCommand,
    ReadPortQualitiesFullCommand,
    ReadPortQualitiesShallowCommand,
    ReadPortAttributesFullCommand,
    ReadPortAttributesShallowCommand,
    ReadPortQualityCommands { id: QualityIdDto, commands: Vec<ReadQualityCommand> },
    ReadPortAttributeCommands { id: AttributeIdDto, commands: Vec<ReadAttributeCommand> },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ReadPortCommandOutput {
    ReadPortFullCommand { port: PortFullDto },
    ReadPortShallowCommand { port: PortShallowDto },
    ReadPortMetadataCommand { metadata: PortMetadataDto },
    ReadPortIdCommand { id: PortIdDto },
    ReadPortNameCommand { name: String },
    ReadPortDescriptionCommand { description: Option<String> },
    ReadPortIconCommand { icon: Option<String> },
    ReadPortCompatibleFamiliesCommand { families: Vec<FamilyIdDto> },
    ReadPortMandatoryCommand { mandatory: Option<bool> },
    ReadPortTCommand { t: Option<f64> },
    ReadPortPointCommand { point: Option<crate::geom::Point> },
    ReadPortDirectionCommand { direction: Option<crate::geom::Vector> },
    ReadPortCompatiblePortsCommand { compatible_ports: Vec<PortIdDto> },
    ReadPortQualitiesFullCommand { qualities: Vec<QualityFullDto> },
    ReadPortQualitiesShallowCommand { qualities: Vec<QualityShallowDto> },
    ReadPortAttributesFullCommand { attributes: Vec<AttributeFullDto> },
    ReadPortAttributesShallowCommand { attributes: Vec<AttributeShallowDto> },
    ReadPortQualityCommands { results: Vec<ReadQualityCommandOutput> },
    ReadPortAttributeCommands { results: Vec<ReadAttributeCommandOutput> },
}

// --- Connector ---

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ReadConnectorCommand {
    ReadConnectorFullCommand,
    ReadConnectorShallowCommand,
    ReadConnectorMetadataCommand,
    ReadConnectorIdCommand,
    ReadConnectorCodeCommand,
    ReadConnectorDescriptionCommand,
    ReadConnectorPortIdCommand,
    ReadConnectorQualitiesFullCommand,
    ReadConnectorQualitiesShallowCommand,
    ReadConnectorAttributesFullCommand,
    ReadConnectorAttributesShallowCommand,
    ReadConnectorQualityCommands { id: QualityIdDto, commands: Vec<ReadQualityCommand> },
    ReadConnectorAttributeCommands { id: AttributeIdDto, commands: Vec<ReadAttributeCommand> },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ReadConnectorCommandOutput {
    ReadConnectorFullCommand { connector: ConnectorFullDto },
    ReadConnectorShallowCommand { connector: ConnectorShallowDto },
    ReadConnectorMetadataCommand { metadata: ConnectorMetadataDto },
    ReadConnectorIdCommand { id: ConnectorIdDto },
    ReadConnectorCodeCommand { code: String },
    ReadConnectorDescriptionCommand { description: Option<String> },
    ReadConnectorPortIdCommand { port: Option<PortIdDto> },
    ReadConnectorQualitiesFullCommand { qualities: Vec<QualityFullDto> },
    ReadConnectorQualitiesShallowCommand { qualities: Vec<QualityShallowDto> },
    ReadConnectorAttributesFullCommand { attributes: Vec<AttributeFullDto> },
    ReadConnectorAttributesShallowCommand { attributes: Vec<AttributeShallowDto> },
    ReadConnectorQualityCommands { results: Vec<ReadQualityCommandOutput> },
    ReadConnectorAttributeCommands { results: Vec<ReadAttributeCommandOutput> },
}

// --- Representation ---

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ReadRepresentationCommand {
    ReadRepresentationFullCommand,
    ReadRepresentationShallowCommand,
    ReadRepresentationMetadataCommand,
    ReadRepresentationIdCommand,
    ReadRepresentationUrlCommand,
    ReadRepresentationDescriptionCommand,
    ReadRepresentationFileIdCommand,
    ReadRepresentationTagsFullCommand,
    ReadRepresentationTagsShallowCommand,
    ReadRepresentationQualitiesFullCommand,
    ReadRepresentationQualitiesShallowCommand,
    ReadRepresentationAttributesFullCommand,
    ReadRepresentationAttributesShallowCommand,
    ReadRepresentationTagCommands { id: TagIdDto, commands: Vec<ReadTagCommand> },
    ReadRepresentationQualityCommands { id: QualityIdDto, commands: Vec<ReadQualityCommand> },
    ReadRepresentationAttributeCommands { id: AttributeIdDto, commands: Vec<ReadAttributeCommand> },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ReadRepresentationCommandOutput {
    ReadRepresentationFullCommand { representation: RepresentationFullDto },
    ReadRepresentationShallowCommand { representation: RepresentationShallowDto },
    ReadRepresentationMetadataCommand { metadata: RepresentationMetadataDto },
    ReadRepresentationIdCommand { id: RepresentationIdDto },
    ReadRepresentationUrlCommand { url: String },
    ReadRepresentationDescriptionCommand { description: Option<String> },
    ReadRepresentationFileIdCommand { file: Option<crate::file::FileIdDto> },
    ReadRepresentationTagsFullCommand { tags: Vec<TagFullDto> },
    ReadRepresentationTagsShallowCommand { tags: Vec<TagShallowDto> },
    ReadRepresentationQualitiesFullCommand { qualities: Vec<QualityFullDto> },
    ReadRepresentationQualitiesShallowCommand { qualities: Vec<QualityShallowDto> },
    ReadRepresentationAttributesFullCommand { attributes: Vec<AttributeFullDto> },
    ReadRepresentationAttributesShallowCommand { attributes: Vec<AttributeShallowDto> },
    ReadRepresentationTagCommands { results: Vec<ReadTagCommandOutput> },
    ReadRepresentationQualityCommands { results: Vec<ReadQualityCommandOutput> },
    ReadRepresentationAttributeCommands { results: Vec<ReadAttributeCommandOutput> },
}

// --- Family ---

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ReadFamilyCommand {
    ReadFamilyFullCommand,
    ReadFamilyShallowCommand,
    ReadFamilyMetadataCommand,
    ReadFamilyIdCommand,
    ReadFamilyNameCommand,
    ReadFamilyDescriptionCommand,
    ReadFamilyIconCommand,
    ReadFamilyPortsFullCommand,
    ReadFamilyPortsShallowCommand,
    ReadFamilyAttributesFullCommand,
    ReadFamilyAttributesShallowCommand,
    ReadFamilyPortCommands { id: PortIdDto, commands: Vec<ReadPortCommand> },
    ReadFamilyAttributeCommands { id: AttributeIdDto, commands: Vec<ReadAttributeCommand> },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ReadFamilyCommandOutput {
    ReadFamilyFullCommand { family: FamilyFullDto },
    ReadFamilyShallowCommand { family: FamilyShallowDto },
    ReadFamilyMetadataCommand { metadata: FamilyMetadataDto },
    ReadFamilyIdCommand { id: FamilyIdDto },
    ReadFamilyNameCommand { name: String },
    ReadFamilyDescriptionCommand { description: Option<String> },
    ReadFamilyIconCommand { icon: Option<String> },
    ReadFamilyPortsFullCommand { ports: Vec<PortFullDto> },
    ReadFamilyPortsShallowCommand { ports: Vec<PortShallowDto> },
    ReadFamilyAttributesFullCommand { attributes: Vec<AttributeFullDto> },
    ReadFamilyAttributesShallowCommand { attributes: Vec<AttributeShallowDto> },
    ReadFamilyPortCommands { results: Vec<ReadPortCommandOutput> },
    ReadFamilyAttributeCommands { results: Vec<ReadAttributeCommandOutput> },
}

// --- File ---

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ReadFileCommand {
    ReadFileFullCommand,
    ReadFileShallowCommand,
    ReadFileMetadataCommand,
    ReadFileIdCommand,
    ReadFileUrlCommand,
    ReadFileMimeCommand,
    ReadFileSizeCommand,
    ReadFileHashCommand,
    ReadFileDescriptionCommand,
    ReadFileCreatedCommand,
    ReadFileUpdatedCommand,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ReadFileCommandOutput {
    ReadFileFullCommand { file: FileFullDto },
    ReadFileShallowCommand { file: FileShallowDto },
    ReadFileMetadataCommand { metadata: FileMetadataDto },
    ReadFileIdCommand { id: FileIdDto },
    ReadFileUrlCommand { url: String },
    ReadFileMimeCommand { mime: Option<String> },
    ReadFileSizeCommand { size: Option<i64> },
    ReadFileHashCommand { hash: Option<String> },
    ReadFileDescriptionCommand { description: Option<String> },
    ReadFileCreatedCommand { created: Option<String> },
    ReadFileUpdatedCommand { updated: Option<String> },
}

// --- Folder ---

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ReadFolderCommand {
    ReadFolderFullCommand,
    ReadFolderShallowCommand,
    ReadFolderMetadataCommand,
    ReadFolderIdCommand,
    ReadFolderPathCommand,
    ReadFolderDescriptionCommand,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ReadFolderCommandOutput {
    ReadFolderFullCommand { folder: FolderFullDto },
    ReadFolderShallowCommand { folder: FolderShallowDto },
    ReadFolderMetadataCommand { metadata: FolderMetadataDto },
    ReadFolderIdCommand { id: FolderIdDto },
    ReadFolderPathCommand { path: String },
    ReadFolderDescriptionCommand { description: Option<String> },
}

// --- Location ---

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ReadLocationCommand {
    ReadLocationFullCommand,
    ReadLocationShallowCommand,
    ReadLocationMetadataCommand,
    ReadLocationIdCommand,
    ReadLocationLongitudeCommand,
    ReadLocationLatitudeCommand,
    ReadLocationAltitudeCommand,
    ReadLocationAttributesFullCommand,
    ReadLocationAttributesShallowCommand,
    ReadLocationAttributeCommands { id: AttributeIdDto, commands: Vec<ReadAttributeCommand> },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ReadLocationCommandOutput {
    ReadLocationFullCommand { location: LocationFullDto },
    ReadLocationShallowCommand { location: LocationShallowDto },
    ReadLocationMetadataCommand { metadata: LocationMetadataDto },
    ReadLocationIdCommand { id: LocationIdDto },
    ReadLocationLongitudeCommand { longitude: f64 },
    ReadLocationLatitudeCommand { latitude: f64 },
    ReadLocationAltitudeCommand { altitude: Option<f64> },
    ReadLocationAttributesFullCommand { attributes: Vec<AttributeFullDto> },
    ReadLocationAttributesShallowCommand { attributes: Vec<AttributeShallowDto> },
    ReadLocationAttributeCommands { results: Vec<ReadAttributeCommandOutput> },
}

// --- Layer ---

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ReadLayerCommand {
    ReadLayerFullCommand,
    ReadLayerShallowCommand,
    ReadLayerMetadataCommand,
    ReadLayerIdCommand,
    ReadLayerNameCommand,
    ReadLayerDescriptionCommand,
    ReadLayerColorCommand,
    ReadLayerOrderCommand,
    ReadLayerVisibleCommand,
    ReadLayerLockedCommand,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ReadLayerCommandOutput {
    ReadLayerFullCommand { layer: LayerFullDto },
    ReadLayerShallowCommand { layer: LayerShallowDto },
    ReadLayerMetadataCommand { metadata: LayerMetadataDto },
    ReadLayerIdCommand { id: LayerIdDto },
    ReadLayerNameCommand { name: String },
    ReadLayerDescriptionCommand { description: Option<String> },
    ReadLayerColorCommand { color: Option<String> },
    ReadLayerOrderCommand { order: Option<i64> },
    ReadLayerVisibleCommand { visible: Option<bool> },
    ReadLayerLockedCommand { locked: Option<bool> },
}

// --- Group ---

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ReadGroupCommand {
    ReadGroupFullCommand,
    ReadGroupShallowCommand,
    ReadGroupMetadataCommand,
    ReadGroupIdCommand,
    ReadGroupNameCommand,
    ReadGroupDescriptionCommand,
    ReadGroupColorCommand,
    ReadGroupIconCommand,
    ReadGroupPiecesCommand,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ReadGroupCommandOutput {
    ReadGroupFullCommand { group: GroupFullDto },
    ReadGroupShallowCommand { group: GroupShallowDto },
    ReadGroupMetadataCommand { metadata: GroupMetadataDto },
    ReadGroupIdCommand { id: GroupIdDto },
    ReadGroupNameCommand { name: String },
    ReadGroupDescriptionCommand { description: Option<String> },
    ReadGroupColorCommand { color: Option<String> },
    ReadGroupIconCommand { icon: Option<String> },
    ReadGroupPiecesCommand { pieces: Vec<PieceIdDto> },
}

// --- Author ---

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ReadAuthorCommand {
    ReadAuthorFullCommand,
    ReadAuthorShallowCommand,
    ReadAuthorMetadataCommand,
    ReadAuthorIdCommand,
    ReadAuthorNameCommand,
    ReadAuthorEmailCommand,
    ReadAuthorRoleCommand,
    ReadAuthorRankCommand,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ReadAuthorCommandOutput {
    ReadAuthorFullCommand { author: AuthorFullDto },
    ReadAuthorShallowCommand { author: AuthorShallowDto },
    ReadAuthorMetadataCommand { metadata: AuthorMetadataDto },
    ReadAuthorIdCommand { id: AuthorIdDto },
    ReadAuthorNameCommand { name: String },
    ReadAuthorEmailCommand { email: String },
    ReadAuthorRoleCommand { role: Option<String> },
    ReadAuthorRankCommand { rank: Option<i64> },
}

// --- Concept ---

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ReadConceptCommand {
    ReadConceptFullCommand,
    ReadConceptShallowCommand,
    ReadConceptMetadataCommand,
    ReadConceptIdCommand,
    ReadConceptNameCommand,
    ReadConceptDescriptionCommand,
    ReadConceptOrderCommand,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ReadConceptCommandOutput {
    ReadConceptFullCommand { concept: ConceptFullDto },
    ReadConceptShallowCommand { concept: ConceptShallowDto },
    ReadConceptMetadataCommand { metadata: ConceptMetadataDto },
    ReadConceptIdCommand { id: ConceptIdDto },
    ReadConceptNameCommand { name: String },
    ReadConceptDescriptionCommand { description: Option<String> },
    ReadConceptOrderCommand { order: Option<i64> },
}

// --- Tag ---

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ReadTagCommand {
    ReadTagFullCommand,
    ReadTagShallowCommand,
    ReadTagMetadataCommand,
    ReadTagIdCommand,
    ReadTagNameCommand,
    ReadTagOrderCommand,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ReadTagCommandOutput {
    ReadTagFullCommand { tag: TagFullDto },
    ReadTagShallowCommand { tag: TagShallowDto },
    ReadTagMetadataCommand { metadata: TagMetadataDto },
    ReadTagIdCommand { id: TagIdDto },
    ReadTagNameCommand { name: String },
    ReadTagOrderCommand { order: Option<i64> },
}

// --- Quality ---

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ReadQualityCommand {
    ReadQualityFullCommand,
    ReadQualityShallowCommand,
    ReadQualityMetadataCommand,
    ReadQualityIdCommand,
    ReadQualityKeyCommand,
    ReadQualityValueCommand,
    ReadQualityUnitCommand,
    ReadQualityDefinitionCommand,
    ReadQualityDescriptionCommand,
    ReadQualityBenchmarksFullCommand,
    ReadQualityBenchmarksShallowCommand,
    ReadQualityBenchmarkCommands { id: BenchmarkIdDto, commands: Vec<ReadBenchmarkCommand> },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ReadQualityCommandOutput {
    ReadQualityFullCommand { quality: QualityFullDto },
    ReadQualityShallowCommand { quality: QualityShallowDto },
    ReadQualityMetadataCommand { metadata: QualityMetadataDto },
    ReadQualityIdCommand { id: QualityIdDto },
    ReadQualityKeyCommand { key: String },
    ReadQualityValueCommand { value: Option<String> },
    ReadQualityUnitCommand { unit: Option<String> },
    ReadQualityDefinitionCommand { definition: Option<String> },
    ReadQualityDescriptionCommand { description: Option<String> },
    ReadQualityBenchmarksFullCommand { benchmarks: Vec<BenchmarkFullDto> },
    ReadQualityBenchmarksShallowCommand { benchmarks: Vec<BenchmarkShallowDto> },
    ReadQualityBenchmarkCommands { results: Vec<ReadBenchmarkCommandOutput> },
}

// --- Benchmark ---

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ReadBenchmarkCommand {
    ReadBenchmarkFullCommand,
    ReadBenchmarkShallowCommand,
    ReadBenchmarkMetadataCommand,
    ReadBenchmarkIdCommand,
    ReadBenchmarkNameCommand,
    ReadBenchmarkMinCommand,
    ReadBenchmarkMaxCommand,
    ReadBenchmarkMinExcludedCommand,
    ReadBenchmarkMaxExcludedCommand,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ReadBenchmarkCommandOutput {
    ReadBenchmarkFullCommand { benchmark: BenchmarkFullDto },
    ReadBenchmarkShallowCommand { benchmark: BenchmarkShallowDto },
    ReadBenchmarkMetadataCommand { metadata: BenchmarkMetadataDto },
    ReadBenchmarkIdCommand { id: BenchmarkIdDto },
    ReadBenchmarkNameCommand { name: String },
    ReadBenchmarkMinCommand { min: Option<f64> },
    ReadBenchmarkMaxCommand { max: Option<f64> },
    ReadBenchmarkMinExcludedCommand { min_excluded: Option<bool> },
    ReadBenchmarkMaxExcludedCommand { max_excluded: Option<bool> },
}

// --- Prop ---

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ReadPropCommand {
    ReadPropFullCommand,
    ReadPropShallowCommand,
    ReadPropIdCommand,
    ReadPropKeyCommand,
    ReadPropValueCommand,
    ReadPropUnitCommand,
    ReadPropQualityIdCommand,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ReadPropCommandOutput {
    ReadPropFullCommand { prop: PropFullDto },
    ReadPropShallowCommand { prop: PropShallowDto },
    ReadPropIdCommand { id: PropIdDto },
    ReadPropKeyCommand { key: String },
    ReadPropValueCommand { value: String },
    ReadPropUnitCommand { unit: Option<String> },
    ReadPropQualityIdCommand { quality: Option<QualityIdDto> },
}

// --- Attribute ---

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ReadAttributeCommand {
    ReadAttributeFullCommand,
    ReadAttributeShallowCommand,
    ReadAttributeMetadataCommand,
    ReadAttributeIdCommand,
    ReadAttributeKeyCommand,
    ReadAttributeValueCommand,
    ReadAttributeDefinitionCommand,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ReadAttributeCommandOutput {
    ReadAttributeFullCommand { attribute: AttributeFullDto },
    ReadAttributeShallowCommand { attribute: AttributeShallowDto },
    ReadAttributeMetadataCommand { metadata: AttributeMetadataDto },
    ReadAttributeIdCommand { id: AttributeIdDto },
    ReadAttributeKeyCommand { key: String },
    ReadAttributeValueCommand { value: String },
    ReadAttributeDefinitionCommand { definition: Option<String> },
}

// --- Stat ---

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ReadStatCommand {
    ReadStatFullCommand,
    ReadStatShallowCommand,
    ReadStatMetadataCommand,
    ReadStatIdCommand,
    ReadStatKeyCommand,
    ReadStatValueCommand,
    ReadStatUnitCommand,
    ReadStatDescriptionCommand,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ReadStatCommandOutput {
    ReadStatFullCommand { stat: StatFullDto },
    ReadStatShallowCommand { stat: StatShallowDto },
    ReadStatMetadataCommand { metadata: StatMetadataDto },
    ReadStatIdCommand { id: StatIdDto },
    ReadStatKeyCommand { key: String },
    ReadStatValueCommand { value: String },
    ReadStatUnitCommand { unit: Option<String> },
    ReadStatDescriptionCommand { description: Option<String> },
}
