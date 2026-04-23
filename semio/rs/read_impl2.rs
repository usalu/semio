// Continuation of read_impl.rs (same `read` module scope).

impl ReadQualityCommand {
    pub fn execute(&self, q: &crate::quality::QualityStoreRef) -> Result<ReadQualityCommandOutput> {
        let q = q.read().map_err(|_| lp("quality"))?;
        Ok(match self {
            ReadQualityCommand::ReadQualityFullCommand => ReadQualityCommandOutput::ReadQualityFullCommand { quality: q.to_full_dto() },
            ReadQualityCommand::ReadQualityShallowCommand => ReadQualityCommandOutput::ReadQualityShallowCommand { quality: q.to_shallow_dto() },
            ReadQualityCommand::ReadQualityMetadataCommand => ReadQualityCommandOutput::ReadQualityMetadataCommand { metadata: q.to_metadata_dto() },
            ReadQualityCommand::ReadQualityIdCommand => ReadQualityCommandOutput::ReadQualityIdCommand { id: q.to_id_dto() },
            ReadQualityCommand::ReadQualityKeyCommand => ReadQualityCommandOutput::ReadQualityKeyCommand { key: q.key.clone() },
            ReadQualityCommand::ReadQualityValueCommand => ReadQualityCommandOutput::ReadQualityValueCommand { value: q.value.clone() },
            ReadQualityCommand::ReadQualityUnitCommand => ReadQualityCommandOutput::ReadQualityUnitCommand { unit: q.unit.clone() },
            ReadQualityCommand::ReadQualityDefinitionCommand => ReadQualityCommandOutput::ReadQualityDefinitionCommand { definition: q.definition.clone() },
            ReadQualityCommand::ReadQualityDescriptionCommand => ReadQualityCommandOutput::ReadQualityDescriptionCommand { description: q.description.clone() },
            ReadQualityCommand::ReadQualityBenchmarksFullCommand => ReadQualityCommandOutput::ReadQualityBenchmarksFullCommand {
                benchmarks: q.benchmarks.iter().filter_map(|b| b.read().ok().map(|b| b.to_full_dto())).collect(),
            },
            ReadQualityCommand::ReadQualityBenchmarksShallowCommand => ReadQualityCommandOutput::ReadQualityBenchmarksShallowCommand {
                benchmarks: q.benchmarks.iter().filter_map(|b| b.read().ok().map(|b| b.to_shallow_dto())).collect(),
            },
            ReadQualityCommand::ReadQualityBenchmarkCommands { id, commands } => {
                let b = q.benchmarks.iter().find(|b| b.read().map(|r| r.id == id.id).unwrap_or(false)).ok_or_else(|| nf("Benchmark", &id.id))?;
                let mut results = Vec::with_capacity(commands.len());
                for c in commands {
                    results.push(c.execute(b)?);
                }
                ReadQualityCommandOutput::ReadQualityBenchmarkCommands { results }
            }
        })
    }
}

impl ReadAuthorCommand {
    pub fn execute(&self, a: &crate::author::AuthorStoreRef) -> Result<ReadAuthorCommandOutput> {
        let a = a.read().map_err(|_| lp("author"))?;
        Ok(match self {
            ReadAuthorCommand::ReadAuthorFullCommand => ReadAuthorCommandOutput::ReadAuthorFullCommand { author: a.to_full_dto() },
            ReadAuthorCommand::ReadAuthorShallowCommand => ReadAuthorCommandOutput::ReadAuthorShallowCommand { author: a.to_shallow_dto() },
            ReadAuthorCommand::ReadAuthorMetadataCommand => ReadAuthorCommandOutput::ReadAuthorMetadataCommand { metadata: a.to_metadata_dto() },
            ReadAuthorCommand::ReadAuthorIdCommand => ReadAuthorCommandOutput::ReadAuthorIdCommand { id: a.to_id_dto() },
            ReadAuthorCommand::ReadAuthorNameCommand => ReadAuthorCommandOutput::ReadAuthorNameCommand { name: a.name.clone() },
            ReadAuthorCommand::ReadAuthorEmailCommand => ReadAuthorCommandOutput::ReadAuthorEmailCommand { email: a.email.clone() },
            ReadAuthorCommand::ReadAuthorRoleCommand => ReadAuthorCommandOutput::ReadAuthorRoleCommand { role: a.role.clone() },
            ReadAuthorCommand::ReadAuthorRankCommand => ReadAuthorCommandOutput::ReadAuthorRankCommand { rank: a.rank },
        })
    }
}

impl ReadConceptCommand {
    pub fn execute(&self, c: &crate::concept::ConceptStoreRef) -> Result<ReadConceptCommandOutput> {
        let c = c.read().map_err(|_| lp("concept"))?;
        Ok(match self {
            ReadConceptCommand::ReadConceptFullCommand => ReadConceptCommandOutput::ReadConceptFullCommand { concept: c.to_full_dto() },
            ReadConceptCommand::ReadConceptShallowCommand => ReadConceptCommandOutput::ReadConceptShallowCommand { concept: c.to_shallow_dto() },
            ReadConceptCommand::ReadConceptMetadataCommand => ReadConceptCommandOutput::ReadConceptMetadataCommand { metadata: c.to_metadata_dto() },
            ReadConceptCommand::ReadConceptIdCommand => ReadConceptCommandOutput::ReadConceptIdCommand { id: c.to_id_dto() },
            ReadConceptCommand::ReadConceptNameCommand => ReadConceptCommandOutput::ReadConceptNameCommand { name: c.name.clone() },
            ReadConceptCommand::ReadConceptDescriptionCommand => ReadConceptCommandOutput::ReadConceptDescriptionCommand { description: c.description.clone() },
            ReadConceptCommand::ReadConceptOrderCommand => ReadConceptCommandOutput::ReadConceptOrderCommand { order: c.order },
        })
    }
}

impl ReadFileCommand {
    pub fn execute(&self, f: &crate::file::FileStoreRef) -> Result<ReadFileCommandOutput> {
        let f = f.read().map_err(|_| lp("file"))?;
        Ok(match self {
            ReadFileCommand::ReadFileFullCommand => ReadFileCommandOutput::ReadFileFullCommand { file: f.to_full_dto() },
            ReadFileCommand::ReadFileShallowCommand => ReadFileCommandOutput::ReadFileShallowCommand { file: f.to_shallow_dto() },
            ReadFileCommand::ReadFileMetadataCommand => ReadFileCommandOutput::ReadFileMetadataCommand { metadata: f.to_metadata_dto() },
            ReadFileCommand::ReadFileIdCommand => ReadFileCommandOutput::ReadFileIdCommand { id: f.to_id_dto() },
            ReadFileCommand::ReadFileUrlCommand => ReadFileCommandOutput::ReadFileUrlCommand { url: f.url.clone() },
            ReadFileCommand::ReadFileMimeCommand => ReadFileCommandOutput::ReadFileMimeCommand { mime: f.mime.clone() },
            ReadFileCommand::ReadFileSizeCommand => ReadFileCommandOutput::ReadFileSizeCommand { size: f.size },
            ReadFileCommand::ReadFileHashCommand => ReadFileCommandOutput::ReadFileHashCommand { hash: f.hash.clone() },
            ReadFileCommand::ReadFileDescriptionCommand => ReadFileCommandOutput::ReadFileDescriptionCommand { description: f.description.clone() },
            ReadFileCommand::ReadFileCreatedCommand => ReadFileCommandOutput::ReadFileCreatedCommand { created: f.created.clone() },
            ReadFileCommand::ReadFileUpdatedCommand => ReadFileCommandOutput::ReadFileUpdatedCommand { updated: f.updated.clone() },
        })
    }
}

impl ReadFolderCommand {
    pub fn execute(&self, f: &crate::folder::FolderStoreRef) -> Result<ReadFolderCommandOutput> {
        let f = f.read().map_err(|_| lp("folder"))?;
        Ok(match self {
            ReadFolderCommand::ReadFolderFullCommand => ReadFolderCommandOutput::ReadFolderFullCommand { folder: f.to_full_dto() },
            ReadFolderCommand::ReadFolderShallowCommand => ReadFolderCommandOutput::ReadFolderShallowCommand { folder: f.to_shallow_dto() },
            ReadFolderCommand::ReadFolderMetadataCommand => ReadFolderCommandOutput::ReadFolderMetadataCommand { metadata: f.to_metadata_dto() },
            ReadFolderCommand::ReadFolderIdCommand => ReadFolderCommandOutput::ReadFolderIdCommand { id: f.to_id_dto() },
            ReadFolderCommand::ReadFolderPathCommand => ReadFolderCommandOutput::ReadFolderPathCommand { path: f.path.clone() },
            ReadFolderCommand::ReadFolderDescriptionCommand => ReadFolderCommandOutput::ReadFolderDescriptionCommand { description: f.description.clone() },
        })
    }
}

impl ReadLocationCommand {
    pub fn execute(&self, l: &crate::location::LocationStoreRef) -> Result<ReadLocationCommandOutput> {
        let l = l.read().map_err(|_| lp("location"))?;
        Ok(match self {
            ReadLocationCommand::ReadLocationFullCommand => ReadLocationCommandOutput::ReadLocationFullCommand { location: l.to_full_dto() },
            ReadLocationCommand::ReadLocationShallowCommand => ReadLocationCommandOutput::ReadLocationShallowCommand { location: l.to_shallow_dto() },
            ReadLocationCommand::ReadLocationMetadataCommand => ReadLocationCommandOutput::ReadLocationMetadataCommand { metadata: l.to_metadata_dto() },
            ReadLocationCommand::ReadLocationIdCommand => ReadLocationCommandOutput::ReadLocationIdCommand { id: l.to_id_dto() },
            ReadLocationCommand::ReadLocationLongitudeCommand => ReadLocationCommandOutput::ReadLocationLongitudeCommand { longitude: l.longitude },
            ReadLocationCommand::ReadLocationLatitudeCommand => ReadLocationCommandOutput::ReadLocationLatitudeCommand { latitude: l.latitude },
            ReadLocationCommand::ReadLocationAltitudeCommand => ReadLocationCommandOutput::ReadLocationAltitudeCommand { altitude: l.altitude },
            ReadLocationCommand::ReadLocationAttributesFullCommand => ReadLocationCommandOutput::ReadLocationAttributesFullCommand {
                attributes: l.attributes.iter().filter_map(|a| a.read().ok().map(|a| a.to_full_dto())).collect(),
            },
            ReadLocationCommand::ReadLocationAttributesShallowCommand => ReadLocationCommandOutput::ReadLocationAttributesShallowCommand {
                attributes: l.attributes.iter().filter_map(|a| a.read().ok().map(|a| a.to_shallow_dto())).collect(),
            },
            ReadLocationCommand::ReadLocationAttributeCommands { id, commands } => {
                let a = l
                    .attributes
                    .iter()
                    .find(|a| a.read().map(|r| r.id == id.id).unwrap_or(false))
                    .ok_or_else(|| nf("Attribute", &id.id))?;
                let mut results = Vec::with_capacity(commands.len());
                for c in commands {
                    results.push(c.execute_ref(a)?);
                }
                ReadLocationCommandOutput::ReadLocationAttributeCommands { results }
            }
        })
    }
}

impl ReadLayerCommand {
    pub fn execute(&self, l: &crate::layer::LayerStoreRef) -> Result<ReadLayerCommandOutput> {
        let l = l.read().map_err(|_| lp("layer"))?;
        Ok(match self {
            ReadLayerCommand::ReadLayerFullCommand => ReadLayerCommandOutput::ReadLayerFullCommand { layer: l.to_full_dto() },
            ReadLayerCommand::ReadLayerShallowCommand => ReadLayerCommandOutput::ReadLayerShallowCommand { layer: l.to_shallow_dto() },
            ReadLayerCommand::ReadLayerMetadataCommand => ReadLayerCommandOutput::ReadLayerMetadataCommand { metadata: l.to_metadata_dto() },
            ReadLayerCommand::ReadLayerIdCommand => ReadLayerCommandOutput::ReadLayerIdCommand { id: l.to_id_dto() },
            ReadLayerCommand::ReadLayerNameCommand => ReadLayerCommandOutput::ReadLayerNameCommand { name: l.name.clone() },
            ReadLayerCommand::ReadLayerDescriptionCommand => ReadLayerCommandOutput::ReadLayerDescriptionCommand { description: l.description.clone() },
            ReadLayerCommand::ReadLayerColorCommand => ReadLayerCommandOutput::ReadLayerColorCommand { color: l.color.clone() },
            ReadLayerCommand::ReadLayerOrderCommand => ReadLayerCommandOutput::ReadLayerOrderCommand { order: l.order },
            ReadLayerCommand::ReadLayerVisibleCommand => ReadLayerCommandOutput::ReadLayerVisibleCommand { visible: l.visible },
            ReadLayerCommand::ReadLayerLockedCommand => ReadLayerCommandOutput::ReadLayerLockedCommand { locked: l.locked },
        })
    }
}

impl ReadGroupCommand {
    pub fn execute(&self, g: &crate::group::GroupStoreRef) -> Result<ReadGroupCommandOutput> {
        let g = g.read().map_err(|_| lp("group"))?;
        Ok(match self {
            ReadGroupCommand::ReadGroupFullCommand => ReadGroupCommandOutput::ReadGroupFullCommand { group: g.to_full_dto() },
            ReadGroupCommand::ReadGroupShallowCommand => ReadGroupCommandOutput::ReadGroupShallowCommand { group: g.to_shallow_dto() },
            ReadGroupCommand::ReadGroupMetadataCommand => ReadGroupCommandOutput::ReadGroupMetadataCommand { metadata: g.to_metadata_dto() },
            ReadGroupCommand::ReadGroupIdCommand => ReadGroupCommandOutput::ReadGroupIdCommand { id: g.to_id_dto() },
            ReadGroupCommand::ReadGroupNameCommand => ReadGroupCommandOutput::ReadGroupNameCommand { name: g.name.clone() },
            ReadGroupCommand::ReadGroupDescriptionCommand => ReadGroupCommandOutput::ReadGroupDescriptionCommand { description: g.description.clone() },
            ReadGroupCommand::ReadGroupColorCommand => ReadGroupCommandOutput::ReadGroupColorCommand { color: g.color.clone() },
            ReadGroupCommand::ReadGroupIconCommand => ReadGroupCommandOutput::ReadGroupIconCommand { icon: g.icon.clone() },
            ReadGroupCommand::ReadGroupPiecesCommand => ReadGroupCommandOutput::ReadGroupPiecesCommand { pieces: g.pieces.iter().filter_map(|w| w.upgrade().and_then(|p| p.read().ok().map(|p| p.to_id_dto()))).collect() },
        })
    }
}

impl ReadFamilyCommand {
    pub fn execute(&self, f: &FamilyStoreRef) -> Result<ReadFamilyCommandOutput> {
        let f = f.read().map_err(|_| lp("family"))?;
        Ok(match self {
            ReadFamilyCommand::ReadFamilyFullCommand => ReadFamilyCommandOutput::ReadFamilyFullCommand { family: f.to_full_dto() },
            ReadFamilyCommand::ReadFamilyShallowCommand => ReadFamilyCommandOutput::ReadFamilyShallowCommand { family: f.to_shallow_dto() },
            ReadFamilyCommand::ReadFamilyMetadataCommand => ReadFamilyCommandOutput::ReadFamilyMetadataCommand { metadata: f.to_metadata_dto() },
            ReadFamilyCommand::ReadFamilyIdCommand => ReadFamilyCommandOutput::ReadFamilyIdCommand { id: f.to_id_dto() },
            ReadFamilyCommand::ReadFamilyNameCommand => ReadFamilyCommandOutput::ReadFamilyNameCommand { name: f.name.clone() },
            ReadFamilyCommand::ReadFamilyDescriptionCommand => ReadFamilyCommandOutput::ReadFamilyDescriptionCommand { description: f.description.clone() },
            ReadFamilyCommand::ReadFamilyIconCommand => ReadFamilyCommandOutput::ReadFamilyIconCommand { icon: f.icon.clone() },
            ReadFamilyCommand::ReadFamilyPortsFullCommand => ReadFamilyCommandOutput::ReadFamilyPortsFullCommand { ports: f.ports.iter().filter_map(|p| p.read().ok().map(|p| p.to_full_dto())).collect() },
            ReadFamilyCommand::ReadFamilyPortsShallowCommand => ReadFamilyCommandOutput::ReadFamilyPortsShallowCommand { ports: f.ports.iter().filter_map(|p| p.read().ok().map(|p| p.to_shallow_dto())).collect() },
            ReadFamilyCommand::ReadFamilyAttributesFullCommand => ReadFamilyCommandOutput::ReadFamilyAttributesFullCommand { attributes: f.attributes.iter().map(|a| a.to_full_dto()).collect() },
            ReadFamilyCommand::ReadFamilyAttributesShallowCommand => ReadFamilyCommandOutput::ReadFamilyAttributesShallowCommand { attributes: f.attributes.iter().map(|a| a.to_shallow_dto()).collect() },
            ReadFamilyCommand::ReadFamilyPortCommands { id, commands } => {
                let p = f.ports.iter().find(|p| p.read().map(|r| r.id == id.id).unwrap_or(false)).ok_or_else(|| nf("Port", &id.id))?;
                let mut results = Vec::with_capacity(commands.len());
                for c in commands {
                    results.push(c.execute(p)?);
                }
                ReadFamilyCommandOutput::ReadFamilyPortCommands { results }
            }
            ReadFamilyCommand::ReadFamilyAttributeCommands { id, commands } => {
                let a = f.attributes.iter().find(|a| a.id == id.id).ok_or_else(|| nf("Attribute", &id.id))?;
                let mut results = Vec::with_capacity(commands.len());
                for c in commands {
                    results.push(c.execute(a)?);
                }
                ReadFamilyCommandOutput::ReadFamilyAttributeCommands { results }
            }
        })
    }
}

impl ReadSideCommand {
    pub fn execute(&self, s: &SideStoreRef) -> Result<ReadSideCommandOutput> {
        let s = s.read().map_err(|_| lp("side"))?;
        Ok(match self {
            ReadSideCommand::ReadSideFullCommand => ReadSideCommandOutput::ReadSideFullCommand { side: s.to_full_dto() },
            ReadSideCommand::ReadSideShallowCommand => ReadSideCommandOutput::ReadSideShallowCommand { side: s.to_shallow_dto() },
            ReadSideCommand::ReadSideMetadataCommand => ReadSideCommandOutput::ReadSideMetadataCommand { side: s.to_metadata_dto() },
            ReadSideCommand::ReadSideIdCommand => ReadSideCommandOutput::ReadSideIdCommand { id: s.to_id_dto() },
            ReadSideCommand::ReadSidePieceIdCommand => ReadSideCommandOutput::ReadSidePieceIdCommand { piece: s.to_metadata_dto().piece },
            ReadSideCommand::ReadSidePortIdCommand => ReadSideCommandOutput::ReadSidePortIdCommand { port: s.to_metadata_dto().port },
            ReadSideCommand::ReadSideDesignPieceIdCommand => ReadSideCommandOutput::ReadSideDesignPieceIdCommand { design_piece: s.to_metadata_dto().design_piece },
        })
    }
}

impl ReadConnectionCommand {
    pub fn execute(&self, c: &crate::connection::ConnectionStoreRef) -> Result<ReadConnectionCommandOutput> {
        let c = c.read().map_err(|_| lp("connection"))?;
        Ok(match self {
            ReadConnectionCommand::ReadConnectionFullCommand => ReadConnectionCommandOutput::ReadConnectionFullCommand { dto: c.to_full_dto() },
            ReadConnectionCommand::ReadConnectionShallowCommand => ReadConnectionCommandOutput::ReadConnectionShallowCommand { dto: c.to_shallow_dto() },
            ReadConnectionCommand::ReadConnectionMetadataCommand => ReadConnectionCommandOutput::ReadConnectionMetadataCommand { metadata: c.to_metadata_dto() },
            ReadConnectionCommand::ReadConnectionIdCommand => ReadConnectionCommandOutput::ReadConnectionIdCommand { id: c.to_id_dto() },
            ReadConnectionCommand::ReadConnectionConnectedSideMetadataCommand => ReadConnectionCommandOutput::ReadConnectionConnectedSideMetadataCommand { side: c.connected.read().map_err(|_| lp("side"))?.to_metadata_dto() },
            ReadConnectionCommand::ReadConnectionConnectingSideMetadataCommand => ReadConnectionCommandOutput::ReadConnectionConnectingSideMetadataCommand { side: c.connecting.read().map_err(|_| lp("side"))?.to_metadata_dto() },
            ReadConnectionCommand::ReadConnectionConnectedSideFullCommand => ReadConnectionCommandOutput::ReadConnectionConnectedSideFullCommand { side: c.connected.read().map_err(|_| lp("side"))?.to_full_dto() },
            ReadConnectionCommand::ReadConnectionConnectingSideFullCommand => ReadConnectionCommandOutput::ReadConnectionConnectingSideFullCommand { side: c.connecting.read().map_err(|_| lp("side"))?.to_full_dto() },
            ReadConnectionCommand::ReadConnectionGapCommand => ReadConnectionCommandOutput::ReadConnectionGapCommand { gap: c.gap },
            ReadConnectionCommand::ReadConnectionShiftCommand => ReadConnectionCommandOutput::ReadConnectionShiftCommand { shift: c.shift },
            ReadConnectionCommand::ReadConnectionRiseCommand => ReadConnectionCommandOutput::ReadConnectionRiseCommand { rise: c.rise },
            ReadConnectionCommand::ReadConnectionRotationCommand => ReadConnectionCommandOutput::ReadConnectionRotationCommand { rotation: c.rotation },
            ReadConnectionCommand::ReadConnectionTurnCommand => ReadConnectionCommandOutput::ReadConnectionTurnCommand { turn: c.turn },
            ReadConnectionCommand::ReadConnectionTiltCommand => ReadConnectionCommandOutput::ReadConnectionTiltCommand { tilt: c.tilt },
            ReadConnectionCommand::ReadConnectionUCommand => ReadConnectionCommandOutput::ReadConnectionUCommand { u: c.x },
            ReadConnectionCommand::ReadConnectionVCommand => ReadConnectionCommandOutput::ReadConnectionVCommand { v: c.y },
            ReadConnectionCommand::ReadConnectionDescriptionCommand => ReadConnectionCommandOutput::ReadConnectionDescriptionCommand { description: c.description.clone() },
            ReadConnectionCommand::ReadConnectionAttributesFullCommand => ReadConnectionCommandOutput::ReadConnectionAttributesFullCommand { attributes: c.attributes.iter().filter_map(|a| a.read().ok().map(|a| a.to_full_dto())).collect() },
            ReadConnectionCommand::ReadConnectionAttributesShallowCommand => ReadConnectionCommandOutput::ReadConnectionAttributesShallowCommand { attributes: c.attributes.iter().filter_map(|a| a.read().ok().map(|a| a.to_shallow_dto())).collect() },
            ReadConnectionCommand::ReadConnectionChildPlaneMatrixCommand => ReadConnectionCommandOutput::ReadConnectionChildPlaneMatrixCommand { matrix: c.child_plane_matrix_rows() },
            ReadConnectionCommand::ReadConnectionFlatSidesForChildCommand { child_piece_id } => {
                let (a, b) = c.flat_side_dtos_for_child(&child_piece_id.id).ok_or_else(|| SemioError::InvalidOperation("no flat sides for child on this connection".into()))?;
                ReadConnectionCommandOutput::ReadConnectionFlatSidesForChildCommand { connected: a, connecting: b }
            }
            ReadConnectionCommand::ReadConnectionAttributeCommands { id, commands } => {
                let a = c.attributes.iter().find(|a| a.read().map(|r| r.id == id.id).unwrap_or(false)).ok_or_else(|| nf("Attribute", &id.id))?;
                let mut results = Vec::with_capacity(commands.len());
                for cmd in commands {
                    results.push(cmd.execute_ref(a)?);
                }
                ReadConnectionCommandOutput::ReadConnectionAttributeCommands { results }
            }
            ReadConnectionCommand::ReadConnectionConnectedSideCommands { commands } => {
                let mut results = Vec::with_capacity(commands.len());
                for cmd in commands {
                    results.push(cmd.execute(&c.connected)?);
                }
                ReadConnectionCommandOutput::ReadConnectionConnectedSideCommands { results }
            }
            ReadConnectionCommand::ReadConnectionConnectingSideCommands { commands } => {
                let mut results = Vec::with_capacity(commands.len());
                for cmd in commands {
                    results.push(cmd.execute(&c.connecting)?);
                }
                ReadConnectionCommandOutput::ReadConnectionConnectingSideCommands { results }
            }
        })
    }
}

// NOTE: read_impl3.rs continues with Port, Connector, Representation, Piece, Type, Design, Kit

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/read_impl3.rs"));
