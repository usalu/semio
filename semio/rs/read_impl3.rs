impl ReadPortCommand {
    pub fn execute(&self, p: &PortStoreRef) -> Result<ReadPortCommandOutput> {
        let p = p.read().map_err(|_| lp("port"))?;
        Ok(match self {
            ReadPortCommand::ReadPortFullCommand => ReadPortCommandOutput::ReadPortFullCommand { port: p.to_full_dto() },
            ReadPortCommand::ReadPortShallowCommand => ReadPortCommandOutput::ReadPortShallowCommand { port: p.to_shallow_dto() },
            ReadPortCommand::ReadPortMetadataCommand => ReadPortCommandOutput::ReadPortMetadataCommand { metadata: p.to_metadata_dto() },
            ReadPortCommand::ReadPortIdCommand => ReadPortCommandOutput::ReadPortIdCommand { id: p.to_id_dto() },
            ReadPortCommand::ReadPortNameCommand => ReadPortCommandOutput::ReadPortNameCommand { name: p.name.clone() },
            ReadPortCommand::ReadPortDescriptionCommand => ReadPortCommandOutput::ReadPortDescriptionCommand { description: p.description.clone() },
            ReadPortCommand::ReadPortIconCommand => ReadPortCommandOutput::ReadPortIconCommand { icon: p.icon.clone() },
            ReadPortCommand::ReadPortCompatibleFamiliesCommand => ReadPortCommandOutput::ReadPortCompatibleFamiliesCommand { families: p.compatible_families.clone() },
            ReadPortCommand::ReadPortMandatoryCommand => ReadPortCommandOutput::ReadPortMandatoryCommand { mandatory: p.mandatory },
            ReadPortCommand::ReadPortTCommand => ReadPortCommandOutput::ReadPortTCommand { t: p.t },
            ReadPortCommand::ReadPortPointCommand => ReadPortCommandOutput::ReadPortPointCommand { point: p.point },
            ReadPortCommand::ReadPortDirectionCommand => ReadPortCommandOutput::ReadPortDirectionCommand { direction: p.direction },
            ReadPortCommand::ReadPortCompatiblePortsCommand => ReadPortCommandOutput::ReadPortCompatiblePortsCommand { compatible_ports: p.compatible_ports.iter().filter_map(|w| w.upgrade().and_then(|p| p.read().ok().map(|p| p.to_id_dto()))).collect() },
            ReadPortCommand::ReadPortQualitiesFullCommand => ReadPortCommandOutput::ReadPortQualitiesFullCommand { qualities: p.qualities.iter().filter_map(|q| q.read().ok().map(|q| q.to_full_dto())).collect() },
            ReadPortCommand::ReadPortQualitiesShallowCommand => ReadPortCommandOutput::ReadPortQualitiesShallowCommand { qualities: p.qualities.iter().filter_map(|q| q.read().ok().map(|q| q.to_shallow_dto())).collect() },
            ReadPortCommand::ReadPortAttributesFullCommand => ReadPortCommandOutput::ReadPortAttributesFullCommand { attributes: p.attributes.iter().map(|a| a.to_full_dto()).collect() },
            ReadPortCommand::ReadPortAttributesShallowCommand => ReadPortCommandOutput::ReadPortAttributesShallowCommand { attributes: p.attributes.iter().map(|a| a.to_shallow_dto()).collect() },
            ReadPortCommand::ReadPortQualityCommands { id, commands } => {
                let q = p.qualities.iter().find(|q| q.read().map(|r| r.id == id.id).unwrap_or(false)).ok_or_else(|| nf("Quality", &id.id))?;
                let mut results = Vec::with_capacity(commands.len());
                for c in commands {
                    results.push(c.execute(q)?);
                }
                ReadPortCommandOutput::ReadPortQualityCommands { results }
            }
            ReadPortCommand::ReadPortAttributeCommands { id, commands } => {
                let a = p.attributes.iter().find(|a| a.id == id.id).ok_or_else(|| nf("Attribute", &id.id))?;
                let mut results = Vec::with_capacity(commands.len());
                for c in commands {
                    results.push(c.execute(a)?);
                }
                ReadPortCommandOutput::ReadPortAttributeCommands { results }
            }
        })
    }
}

impl ReadConnectorCommand {
    pub fn execute(&self, c: &ConnectorStoreRef) -> Result<ReadConnectorCommandOutput> {
        let c = c.read().map_err(|_| lp("connector"))?;
        Ok(match self {
            ReadConnectorCommand::ReadConnectorFullCommand => ReadConnectorCommandOutput::ReadConnectorFullCommand { connector: c.to_full_dto() },
            ReadConnectorCommand::ReadConnectorShallowCommand => ReadConnectorCommandOutput::ReadConnectorShallowCommand { connector: c.to_shallow_dto() },
            ReadConnectorCommand::ReadConnectorMetadataCommand => ReadConnectorCommandOutput::ReadConnectorMetadataCommand { metadata: c.to_metadata_dto() },
            ReadConnectorCommand::ReadConnectorIdCommand => ReadConnectorCommandOutput::ReadConnectorIdCommand { id: c.to_id_dto() },
            ReadConnectorCommand::ReadConnectorCodeCommand => ReadConnectorCommandOutput::ReadConnectorCodeCommand { code: c.code.clone() },
            ReadConnectorCommand::ReadConnectorDescriptionCommand => ReadConnectorCommandOutput::ReadConnectorDescriptionCommand { description: c.description.clone() },
            ReadConnectorCommand::ReadConnectorPortIdCommand => ReadConnectorCommandOutput::ReadConnectorPortIdCommand { port: c.to_metadata_dto().port },
            ReadConnectorCommand::ReadConnectorQualitiesFullCommand => ReadConnectorCommandOutput::ReadConnectorQualitiesFullCommand { qualities: c.qualities.iter().filter_map(|q| q.read().ok().map(|q| q.to_full_dto())).collect() },
            ReadConnectorCommand::ReadConnectorQualitiesShallowCommand => ReadConnectorCommandOutput::ReadConnectorQualitiesShallowCommand { qualities: c.qualities.iter().filter_map(|q| q.read().ok().map(|q| q.to_shallow_dto())).collect() },
            ReadConnectorCommand::ReadConnectorAttributesFullCommand => ReadConnectorCommandOutput::ReadConnectorAttributesFullCommand { attributes: c.attributes.iter().map(|a| a.to_full_dto()).collect() },
            ReadConnectorCommand::ReadConnectorAttributesShallowCommand => ReadConnectorCommandOutput::ReadConnectorAttributesShallowCommand { attributes: c.attributes.iter().map(|a| a.to_shallow_dto()).collect() },
            ReadConnectorCommand::ReadConnectorQualityCommands { id, commands } => {
                let q = c.qualities.iter().find(|q| q.read().map(|r| r.id == id.id).unwrap_or(false)).ok_or_else(|| nf("Quality", &id.id))?;
                let mut results = Vec::with_capacity(commands.len());
                for cmd in commands {
                    results.push(cmd.execute(q)?);
                }
                ReadConnectorCommandOutput::ReadConnectorQualityCommands { results }
            }
            ReadConnectorCommand::ReadConnectorAttributeCommands { id, commands } => {
                let a = c.attributes.iter().find(|a| a.id == id.id).ok_or_else(|| nf("Attribute", &id.id))?;
                let mut results = Vec::with_capacity(commands.len());
                for cmd in commands {
                    results.push(cmd.execute(a)?);
                }
                ReadConnectorCommandOutput::ReadConnectorAttributeCommands { results }
            }
        })
    }
}

impl ReadRepresentationCommand {
    pub fn execute(&self, r: &RepresentationStoreRef) -> Result<ReadRepresentationCommandOutput> {
        let r = r.read().map_err(|_| lp("representation"))?;
        Ok(match self {
            ReadRepresentationCommand::ReadRepresentationFullCommand => ReadRepresentationCommandOutput::ReadRepresentationFullCommand { representation: r.to_full_dto() },
            ReadRepresentationCommand::ReadRepresentationShallowCommand => ReadRepresentationCommandOutput::ReadRepresentationShallowCommand { representation: r.to_shallow_dto() },
            ReadRepresentationCommand::ReadRepresentationMetadataCommand => ReadRepresentationCommandOutput::ReadRepresentationMetadataCommand { metadata: r.to_metadata_dto() },
            ReadRepresentationCommand::ReadRepresentationIdCommand => ReadRepresentationCommandOutput::ReadRepresentationIdCommand { id: r.to_id_dto() },
            ReadRepresentationCommand::ReadRepresentationUrlCommand => ReadRepresentationCommandOutput::ReadRepresentationUrlCommand { url: r.url.clone() },
            ReadRepresentationCommand::ReadRepresentationDescriptionCommand => ReadRepresentationCommandOutput::ReadRepresentationDescriptionCommand { description: r.description.clone() },
            ReadRepresentationCommand::ReadRepresentationFileIdCommand => ReadRepresentationCommandOutput::ReadRepresentationFileIdCommand { file: r.to_metadata_dto().file },
            ReadRepresentationCommand::ReadRepresentationTagsFullCommand => ReadRepresentationCommandOutput::ReadRepresentationTagsFullCommand { tags: r.tags.iter().map(|t| t.to_full_dto()).collect() },
            ReadRepresentationCommand::ReadRepresentationTagsShallowCommand => ReadRepresentationCommandOutput::ReadRepresentationTagsShallowCommand { tags: r.tags.iter().map(|t| t.to_shallow_dto()).collect() },
            ReadRepresentationCommand::ReadRepresentationQualitiesFullCommand => ReadRepresentationCommandOutput::ReadRepresentationQualitiesFullCommand { qualities: r.qualities.iter().filter_map(|q| q.read().ok().map(|q| q.to_full_dto())).collect() },
            ReadRepresentationCommand::ReadRepresentationQualitiesShallowCommand => ReadRepresentationCommandOutput::ReadRepresentationQualitiesShallowCommand { qualities: r.qualities.iter().filter_map(|q| q.read().ok().map(|q| q.to_shallow_dto())).collect() },
            ReadRepresentationCommand::ReadRepresentationAttributesFullCommand => ReadRepresentationCommandOutput::ReadRepresentationAttributesFullCommand { attributes: r.attributes.iter().map(|a| a.to_full_dto()).collect() },
            ReadRepresentationCommand::ReadRepresentationAttributesShallowCommand => ReadRepresentationCommandOutput::ReadRepresentationAttributesShallowCommand { attributes: r.attributes.iter().map(|a| a.to_shallow_dto()).collect() },
            ReadRepresentationCommand::ReadRepresentationTagCommands { id, commands } => {
                let t = r.tags.iter().find(|t| t.id == id.id).ok_or_else(|| nf("Tag", &id.id))?;
                let mut results = Vec::with_capacity(commands.len());
                for c in commands {
                    results.push(c.execute(t)?);
                }
                ReadRepresentationCommandOutput::ReadRepresentationTagCommands { results }
            }
            ReadRepresentationCommand::ReadRepresentationQualityCommands { id, commands } => {
                let q = r.qualities.iter().find(|q| q.read().map(|o| o.id == id.id).unwrap_or(false)).ok_or_else(|| nf("Quality", &id.id))?;
                let mut results = Vec::with_capacity(commands.len());
                for c in commands {
                    results.push(c.execute(q)?);
                }
                ReadRepresentationCommandOutput::ReadRepresentationQualityCommands { results }
            }
            ReadRepresentationCommand::ReadRepresentationAttributeCommands { id, commands } => {
                let a = r.attributes.iter().find(|a| a.id == id.id).ok_or_else(|| nf("Attribute", &id.id))?;
                let mut results = Vec::with_capacity(commands.len());
                for c in commands {
                    results.push(c.execute(a)?);
                }
                ReadRepresentationCommandOutput::ReadRepresentationAttributeCommands { results }
            }
        })
    }
}

impl ReadPieceCommand {
    pub fn execute(&self, p: &PieceStoreRef, _g: &KitGraph) -> Result<ReadPieceCommandOutput> {
        let o = p.read().map_err(|_| lp("piece"))?;
        Ok(match self {
            ReadPieceCommand::ReadPieceFullCommand => ReadPieceCommandOutput::ReadPieceFullCommand { dto: o.to_full_dto() },
            ReadPieceCommand::ReadPieceShallowCommand => ReadPieceCommandOutput::ReadPieceShallowCommand { dto: o.to_shallow_dto() },
            ReadPieceCommand::ReadPieceMetadataCommand => ReadPieceCommandOutput::ReadPieceMetadataCommand { metadata: o.to_metadata_dto() },
            ReadPieceCommand::ReadPieceIdCommand => ReadPieceCommandOutput::ReadPieceIdCommand { id: o.to_id_dto() },
            ReadPieceCommand::ReadPieceNameCommand => ReadPieceCommandOutput::ReadPieceNameCommand { name: o.name.clone() },
            ReadPieceCommand::ReadPieceDescriptionCommand => ReadPieceCommandOutput::ReadPieceDescriptionCommand { description: o.description.clone() },
            ReadPieceCommand::ReadPiecePlaneCommand => ReadPieceCommandOutput::ReadPiecePlaneCommand { plane: o.pose.plane },
            ReadPieceCommand::ReadPieceCenterCommand => ReadPieceCommandOutput::ReadPieceCenterCommand { center: o.pose.center },
            ReadPieceCommand::ReadPieceScaleCommand => ReadPieceCommandOutput::ReadPieceScaleCommand { scale: o.scale },
            ReadPieceCommand::ReadPieceMirrorPlaneCommand => ReadPieceCommandOutput::ReadPieceMirrorPlaneCommand { mirror_plane: o.mirror_plane },
            ReadPieceCommand::ReadPieceHiddenCommand => ReadPieceCommandOutput::ReadPieceHiddenCommand { hidden: o.hidden },
            ReadPieceCommand::ReadPieceLockedCommand => ReadPieceCommandOutput::ReadPieceLockedCommand { locked: o.locked },
            ReadPieceCommand::ReadPieceColorCommand => ReadPieceCommandOutput::ReadPieceColorCommand { color: o.color.clone() },
            ReadPieceCommand::ReadPieceTypeCommand => ReadPieceCommandOutput::ReadPieceTypeCommand { r#type: o.to_metadata_dto().r#type },
            ReadPieceCommand::ReadPieceDesignCommand => ReadPieceCommandOutput::ReadPieceDesignCommand { design: o.to_metadata_dto().design },
            ReadPieceCommand::ReadPiecePropsFullCommand => ReadPieceCommandOutput::ReadPiecePropsFullCommand { props: o.props.iter().filter_map(|p| p.read().ok().map(|p| p.to_full_dto())).collect() },
            ReadPieceCommand::ReadPiecePropsShallowCommand => ReadPieceCommandOutput::ReadPiecePropsShallowCommand { props: o.props.iter().filter_map(|p| p.read().ok().map(|p| p.to_shallow_dto())).collect() },
            ReadPieceCommand::ReadPieceAttributesFullCommand => ReadPieceCommandOutput::ReadPieceAttributesFullCommand { attributes: o.attributes.iter().filter_map(|a| a.read().ok().map(|a| a.to_full_dto())).collect() },
            ReadPieceCommand::ReadPieceAttributesShallowCommand => ReadPieceCommandOutput::ReadPieceAttributesShallowCommand { attributes: o.attributes.iter().filter_map(|a| a.read().ok().map(|a| a.to_shallow_dto())).collect() },
            ReadPieceCommand::ReadPieceFlatPlaneCommand => ReadPieceCommandOutput::ReadPieceFlatPlaneCommand { flat_plane: o.flat_plane() },
            ReadPieceCommand::ReadPieceFlatCenterCommand => ReadPieceCommandOutput::ReadPieceFlatCenterCommand { flat_center: o.flat_center() },
            ReadPieceCommand::ReadPieceFlatPoseCommand => ReadPieceCommandOutput::ReadPieceFlatPoseCommand { flat_pose: o.flat_pose_full_dto() },
            ReadPieceCommand::ReadPiecePathCommand => ReadPieceCommandOutput::ReadPiecePathCommand { path: o.path() },
            ReadPieceCommand::ReadPieceParentPieceIdCommand => ReadPieceCommandOutput::ReadPieceParentPieceIdCommand {
                parent_piece: o.parent_piece.as_ref().and_then(|w| w.upgrade()).and_then(|p| p.read().ok().map(|p| p.to_id_dto())),
            },
            ReadPieceCommand::ReadPieceParentConnectionIdCommand => ReadPieceCommandOutput::ReadPieceParentConnectionIdCommand {
                parent_connection: o.parent_connection.as_ref().and_then(|w| w.upgrade()).and_then(|c| c.read().ok().map(|c| c.to_id_dto())),
            },
            ReadPieceCommand::ReadPieceParentDesignIdCommand => ReadPieceCommandOutput::ReadPieceParentDesignIdCommand {
                parent_design: o
                    .parent_design
                    .upgrade()
                    .and_then(|d| d.read().ok().map(|d| DesignIdDto { id: d.id.clone() }))
                    .ok_or_else(|| SemioError::InvalidOperation("piece has no parent design".into()))?,
            },
            ReadPieceCommand::ReadPieceFixedCommand => {
                if o.parent_piece.is_some() || o.parent_connection.is_some() {
                    return Err(SemioError::InvalidOperation("ReadPieceFixedCommand requires no parent piece or connection".into()));
                }
                ReadPieceCommandOutput::ReadPieceFixedCommand {
                    fixed: crate::piece::FixedPieceOutputDto { piece: o.to_id_dto(), pose: o.pose_full_dto() },
                }
            }
            ReadPieceCommand::ReadPieceConnectedCommand => {
                let (pp, pc) = (o.parent_piece.as_ref().and_then(|w| w.upgrade()), o.parent_connection.as_ref().and_then(|w| w.upgrade()));
                let (Some(pp), Some(pc)) = (pp, pc) else {
                    return Err(SemioError::InvalidOperation("ReadPieceConnectedCommand requires parent piece and parent connection".into()));
                };
                let _pp = pp.read().map_err(|_| lp("piece"))?;
                let _pc = pc.read().map_err(|_| lp("connection"))?;
                ReadPieceCommandOutput::ReadPieceConnectedCommand {
                    connected: crate::piece::ConnectedPieceOutputDto {
                        piece: o.to_id_dto(),
                        parent_piece: _pp.to_id_dto(),
                        parent_connection: _pc.to_id_dto(),
                        flat_pose: o.flat_pose_full_dto(),
                    },
                }
            }
            ReadPieceCommand::ReadPieceAlternativesCommand => {
                let alt = o.alternatives();
                ReadPieceCommandOutput::ReadPieceAlternativesCommand {
                    alternatives: crate::piece::PieceAlternativesDto {
                        types: alt.types.iter().filter_map(|t| t.read().ok().map(|r| TypeIdDto { id: r.id.clone() })).collect(),
                        designs: alt.designs.iter().filter_map(|d| d.read().ok().map(|r| DesignIdDto { id: r.id.clone() })).collect(),
                    },
                }
            }
            ReadPieceCommand::ReadPieceAlternativeTypesCommand => ReadPieceCommandOutput::ReadPieceAlternativeTypesCommand {
                types: o.alternative_types().iter().filter_map(|t| t.read().ok().map(|r| TypeIdDto { id: r.id.clone() })).collect(),
            },
            ReadPieceCommand::ReadPieceAlternativeDesignsCommand => ReadPieceCommandOutput::ReadPieceAlternativeDesignsCommand {
                designs: o.alternative_designs().iter().filter_map(|d| d.read().ok().map(|r| DesignIdDto { id: r.id.clone() })).collect(),
            },
            ReadPieceCommand::ReadPiecePropCommands { id, commands } => {
                let pr = o.props.iter().find(|p| p.read().map(|r| r.id == id.id).unwrap_or(false)).ok_or_else(|| nf("Prop", &id.id))?;
                let mut results = Vec::with_capacity(commands.len());
                for c in commands {
                    results.push(c.execute(pr)?);
                }
                ReadPieceCommandOutput::ReadPiecePropCommands { results }
            }
            ReadPieceCommand::ReadPieceAttributeCommands { id, commands } => {
                let a = o.attributes.iter().find(|a| a.read().map(|r| r.id == id.id).unwrap_or(false)).ok_or_else(|| nf("Attribute", &id.id))?;
                let mut results = Vec::with_capacity(commands.len());
                for c in commands {
                    results.push(c.execute_ref(a)?);
                }
                ReadPieceCommandOutput::ReadPieceAttributeCommands { results }
            }
        })
    }
}

impl ReadTypeCommand {
    pub fn execute(&self, t: &TypeStoreRef, g: &KitGraph) -> Result<ReadTypeCommandOutput> {
        let r = t.read().map_err(|_| lp("type"))?;
        Ok(match self {
            ReadTypeCommand::ReadTypeFullCommand => ReadTypeCommandOutput::ReadTypeFullCommand { dto: r.to_full_dto() },
            ReadTypeCommand::ReadTypeShallowCommand => ReadTypeCommandOutput::ReadTypeShallowCommand { dto: r.to_shallow_dto() },
            ReadTypeCommand::ReadTypeMetadataCommand => ReadTypeCommandOutput::ReadTypeMetadataCommand { metadata: r.to_metadata_dto() },
            ReadTypeCommand::ReadTypeIdCommand => ReadTypeCommandOutput::ReadTypeIdCommand { id: r.to_id_dto() },
            ReadTypeCommand::ReadTypeNameCommand => ReadTypeCommandOutput::ReadTypeNameCommand { name: r.name.clone() },
            ReadTypeCommand::ReadTypeDescriptionCommand => ReadTypeCommandOutput::ReadTypeDescriptionCommand { description: r.description.clone() },
            ReadTypeCommand::ReadTypeIconCommand => ReadTypeCommandOutput::ReadTypeIconCommand { icon: r.icon.clone() },
            ReadTypeCommand::ReadTypeImageCommand => ReadTypeCommandOutput::ReadTypeImageCommand { image: r.image.clone() },
            ReadTypeCommand::ReadTypeStockCommand => ReadTypeCommandOutput::ReadTypeStockCommand { stock: r.stock },
            ReadTypeCommand::ReadTypeVirtualCommand => ReadTypeCommandOutput::ReadTypeVirtualCommand { r#virtual: r.virtual_ },
            ReadTypeCommand::ReadTypeUnitCommand => ReadTypeCommandOutput::ReadTypeUnitCommand { unit: r.unit.clone() },
            ReadTypeCommand::ReadTypeLocationCommand => ReadTypeCommandOutput::ReadTypeLocationCommand { location: r.location.clone() },
            ReadTypeCommand::ReadTypeCreatedCommand => ReadTypeCommandOutput::ReadTypeCreatedCommand { created: r.created.clone() },
            ReadTypeCommand::ReadTypeUpdatedCommand => ReadTypeCommandOutput::ReadTypeUpdatedCommand { updated: r.updated.clone() },
            ReadTypeCommand::ReadTypeFamiliesCommand => ReadTypeCommandOutput::ReadTypeFamiliesCommand { families: r.to_full_dto().families },
            ReadTypeCommand::ReadTypeConnectorsFullCommand => ReadTypeCommandOutput::ReadTypeConnectorsFullCommand { connectors: r.connectors.iter().filter_map(|c| c.read().ok().map(|c| c.to_full_dto())).collect() },
            ReadTypeCommand::ReadTypeConnectorsShallowCommand => ReadTypeCommandOutput::ReadTypeConnectorsShallowCommand { connectors: r.connectors.iter().filter_map(|c| c.read().ok().map(|c| c.to_shallow_dto())).collect() },
            ReadTypeCommand::ReadTypeRepresentationsFullCommand => ReadTypeCommandOutput::ReadTypeRepresentationsFullCommand { representations: r.representations.iter().filter_map(|x| x.read().ok().map(|x| x.to_full_dto())).collect() },
            ReadTypeCommand::ReadTypeRepresentationsShallowCommand => ReadTypeCommandOutput::ReadTypeRepresentationsShallowCommand { representations: r.representations.iter().filter_map(|x| x.read().ok().map(|x| x.to_shallow_dto())).collect() },
            ReadTypeCommand::ReadTypeAuthorsFullCommand => ReadTypeCommandOutput::ReadTypeAuthorsFullCommand { authors: r.authors.iter().filter_map(|a| a.read().ok().map(|a| a.to_full_dto())).collect() },
            ReadTypeCommand::ReadTypeAuthorsShallowCommand => ReadTypeCommandOutput::ReadTypeAuthorsShallowCommand { authors: r.authors.iter().filter_map(|a| a.read().ok().map(|a| a.to_shallow_dto())).collect() },
            ReadTypeCommand::ReadTypeConceptsFullCommand => ReadTypeCommandOutput::ReadTypeConceptsFullCommand { concepts: r.concepts.iter().filter_map(|c| c.read().ok().map(|c| c.to_full_dto())).collect() },
            ReadTypeCommand::ReadTypeConceptsShallowCommand => ReadTypeCommandOutput::ReadTypeConceptsShallowCommand { concepts: r.concepts.iter().filter_map(|c| c.read().ok().map(|c| c.to_shallow_dto())).collect() },
            ReadTypeCommand::ReadTypeTagsFullCommand => ReadTypeCommandOutput::ReadTypeTagsFullCommand { tags: r.tags.iter().filter_map(|t| t.read().ok().map(|t| t.to_full_dto())).collect() },
            ReadTypeCommand::ReadTypeTagsShallowCommand => ReadTypeCommandOutput::ReadTypeTagsShallowCommand { tags: r.tags.iter().filter_map(|t| t.read().ok().map(|t| t.to_shallow_dto())).collect() },
            ReadTypeCommand::ReadTypeQualitiesFullCommand => ReadTypeCommandOutput::ReadTypeQualitiesFullCommand { qualities: r.qualities.iter().filter_map(|q| q.read().ok().map(|q| q.to_full_dto())).collect() },
            ReadTypeCommand::ReadTypeQualitiesShallowCommand => ReadTypeCommandOutput::ReadTypeQualitiesShallowCommand { qualities: r.qualities.iter().filter_map(|q| q.read().ok().map(|q| q.to_shallow_dto())).collect() },
            ReadTypeCommand::ReadTypePropsFullCommand => ReadTypeCommandOutput::ReadTypePropsFullCommand { props: r.props.iter().filter_map(|p| p.read().ok().map(|p| p.to_full_dto())).collect() },
            ReadTypeCommand::ReadTypePropsShallowCommand => ReadTypeCommandOutput::ReadTypePropsShallowCommand { props: r.props.iter().filter_map(|p| p.read().ok().map(|p| p.to_shallow_dto())).collect() },
            ReadTypeCommand::ReadTypeAttributesFullCommand => ReadTypeCommandOutput::ReadTypeAttributesFullCommand { attributes: r.attributes.iter().filter_map(|a| a.read().ok().map(|a| a.to_full_dto())).collect() },
            ReadTypeCommand::ReadTypeAttributesShallowCommand => ReadTypeCommandOutput::ReadTypeAttributesShallowCommand { attributes: r.attributes.iter().filter_map(|a| a.read().ok().map(|a| a.to_shallow_dto())).collect() },
            ReadTypeCommand::ReadTypePortsFullCommand => ReadTypeCommandOutput::ReadTypePortsFullCommand { ports: type_all_ports(&*r) },
            ReadTypeCommand::ReadTypeConnectorForPortIdCommand { port_id } => ReadTypeCommandOutput::ReadTypeConnectorForPortIdCommand {
                connector: r.connector_for_port_id(&port_id.id).and_then(|c| c.read().ok().map(|c| c.to_full_dto())),
            },
            ReadTypeCommand::ReadTypeFamilyCommands { id, commands } => {
                let f = kit_family(g, &id.id).ok_or_else(|| nf("Family", &id.id))?;
                let mut results = Vec::with_capacity(commands.len());
                for c in commands {
                    results.push(c.execute(&f)?);
                }
                ReadTypeCommandOutput::ReadTypeFamilyCommands { results }
            }
            ReadTypeCommand::ReadTypeConnectorCommands { id, commands } => {
                let c = r.connector(id.id.as_str()).ok_or_else(|| nf("Connector", &id.id))?;
                let mut results = Vec::with_capacity(commands.len());
                for x in commands {
                    results.push(x.execute(&c)?);
                }
                ReadTypeCommandOutput::ReadTypeConnectorCommands { results }
            }
            ReadTypeCommand::ReadTypeRepresentationCommands { id, commands } => {
                let x = r.representation(id.id.as_str()).ok_or_else(|| nf("Representation", &id.id))?;
                let mut results = Vec::with_capacity(commands.len());
                for c in commands {
                    results.push(c.execute(&x)?);
                }
                ReadTypeCommandOutput::ReadTypeRepresentationCommands { results }
            }
            ReadTypeCommand::ReadTypePortCommands { id, commands } => {
                let p = r.port(id.id.as_str()).ok_or_else(|| nf("Port", &id.id))?;
                let mut results = Vec::with_capacity(commands.len());
                for c in commands {
                    results.push(c.execute(&p)?);
                }
                ReadTypeCommandOutput::ReadTypePortCommands { results }
            }
            ReadTypeCommand::ReadTypeAuthorCommands { id, commands } => {
                let a = r.authors.iter().find(|a| a.read().map(|o| o.id == id.id).unwrap_or(false)).ok_or_else(|| nf("Author", &id.id))?;
                let mut results = Vec::with_capacity(commands.len());
                for c in commands {
                    results.push(c.execute(a)?);
                }
                ReadTypeCommandOutput::ReadTypeAuthorCommands { results }
            }
            ReadTypeCommand::ReadTypeConceptCommands { id, commands } => {
                let a = r.concepts.iter().find(|a| a.read().map(|o| o.id == id.id).unwrap_or(false)).ok_or_else(|| nf("Concept", &id.id))?;
                let mut results = Vec::with_capacity(commands.len());
                for c in commands {
                    results.push(c.execute(a)?);
                }
                ReadTypeCommandOutput::ReadTypeConceptCommands { results }
            }
            ReadTypeCommand::ReadTypeTagCommands { id, commands } => {
                let a = r.tags.iter().find(|a| a.read().map(|o| o.id == id.id).unwrap_or(false)).ok_or_else(|| nf("Tag", &id.id))?;
                let mut results = Vec::with_capacity(commands.len());
                for c in commands {
                    results.push(c.execute_ref(a)?);
                }
                ReadTypeCommandOutput::ReadTypeTagCommands { results }
            }
            ReadTypeCommand::ReadTypeQualityCommands { id, commands } => {
                let a = r.qualities.iter().find(|a| a.read().map(|o| o.id == id.id).unwrap_or(false)).ok_or_else(|| nf("Quality", &id.id))?;
                let mut results = Vec::with_capacity(commands.len());
                for c in commands {
                    results.push(c.execute(a)?);
                }
                ReadTypeCommandOutput::ReadTypeQualityCommands { results }
            }
            ReadTypeCommand::ReadTypePropCommands { id, commands } => {
                let a = r.props.iter().find(|a| a.read().map(|o| o.id == id.id).unwrap_or(false)).ok_or_else(|| nf("Prop", &id.id))?;
                let mut results = Vec::with_capacity(commands.len());
                for c in commands {
                    results.push(c.execute(a)?);
                }
                ReadTypeCommandOutput::ReadTypePropCommands { results }
            }
            ReadTypeCommand::ReadTypeAttributeCommands { id, commands } => {
                let a = r.attributes.iter().find(|a| a.read().map(|o| o.id == id.id).unwrap_or(false)).ok_or_else(|| nf("Attribute", &id.id))?;
                let mut results = Vec::with_capacity(commands.len());
                for c in commands {
                    results.push(c.execute_ref(a)?);
                }
                ReadTypeCommandOutput::ReadTypeAttributeCommands { results }
            }
        })
    }
}

impl ReadDesignCommand {
    pub fn execute(&self, d: &DesignStoreRef, g: &KitGraph) -> Result<ReadDesignCommandOutput> {
        let o = d.read().map_err(|_| lp("design"))?;
        Ok(match self {
            ReadDesignCommand::ReadDesignFullCommand => ReadDesignCommandOutput::ReadDesignFullCommand { dto: o.to_full_dto() },
            ReadDesignCommand::ReadDesignShallowCommand => ReadDesignCommandOutput::ReadDesignShallowCommand { dto: o.to_shallow_dto() },
            ReadDesignCommand::ReadDesignMetadataCommand => ReadDesignCommandOutput::ReadDesignMetadataCommand { metadata: o.to_metadata_dto() },
            ReadDesignCommand::ReadDesignIdCommand => ReadDesignCommandOutput::ReadDesignIdCommand { id: o.to_id_dto() },
            ReadDesignCommand::ReadDesignNameCommand => ReadDesignCommandOutput::ReadDesignNameCommand { name: o.name.clone() },
            ReadDesignCommand::ReadDesignDescriptionCommand => ReadDesignCommandOutput::ReadDesignDescriptionCommand { description: o.description.clone() },
            ReadDesignCommand::ReadDesignIconCommand => ReadDesignCommandOutput::ReadDesignIconCommand { icon: o.icon.clone() },
            ReadDesignCommand::ReadDesignImageCommand => ReadDesignCommandOutput::ReadDesignImageCommand { image: o.image.clone() },
            ReadDesignCommand::ReadDesignLocationCommand => ReadDesignCommandOutput::ReadDesignLocationCommand { location: o.location.clone() },
            ReadDesignCommand::ReadDesignUnitCommand => ReadDesignCommandOutput::ReadDesignUnitCommand { unit: o.unit.clone() },
            ReadDesignCommand::ReadDesignCreatedCommand => ReadDesignCommandOutput::ReadDesignCreatedCommand { created: o.created.clone() },
            ReadDesignCommand::ReadDesignUpdatedCommand => ReadDesignCommandOutput::ReadDesignUpdatedCommand { updated: o.updated.clone() },
            ReadDesignCommand::ReadDesignKitCommand => ReadDesignCommandOutput::ReadDesignKitCommand { kit: o.to_full_dto().kit },
            ReadDesignCommand::ReadDesignFamiliesCommand => ReadDesignCommandOutput::ReadDesignFamiliesCommand { families: o.to_full_dto().families },
            ReadDesignCommand::ReadDesignPiecesFullCommand => ReadDesignCommandOutput::ReadDesignPiecesFullCommand { pieces: o.pieces.iter().filter_map(|p| p.read().ok().map(|p| p.to_full_dto())).collect() },
            ReadDesignCommand::ReadDesignPiecesShallowCommand => ReadDesignCommandOutput::ReadDesignPiecesShallowCommand { pieces: o.pieces.iter().filter_map(|p| p.read().ok().map(|p| p.to_shallow_dto())).collect() },
            ReadDesignCommand::ReadDesignConnectionsFullCommand => ReadDesignCommandOutput::ReadDesignConnectionsFullCommand { connections: o.connections.iter().filter_map(|c| c.read().ok().map(|c| c.to_full_dto())).collect() },
            ReadDesignCommand::ReadDesignConnectionsShallowCommand => ReadDesignCommandOutput::ReadDesignConnectionsShallowCommand { connections: o.connections.iter().filter_map(|c| c.read().ok().map(|c| c.to_shallow_dto())).collect() },
            ReadDesignCommand::ReadDesignLayersFullCommand => ReadDesignCommandOutput::ReadDesignLayersFullCommand { layers: o.layers.iter().filter_map(|l| l.read().ok().map(|l| l.to_full_dto())).collect() },
            ReadDesignCommand::ReadDesignLayersShallowCommand => ReadDesignCommandOutput::ReadDesignLayersShallowCommand { layers: o.layers.iter().filter_map(|l| l.read().ok().map(|l| l.to_shallow_dto())).collect() },
            ReadDesignCommand::ReadDesignGroupsFullCommand => ReadDesignCommandOutput::ReadDesignGroupsFullCommand { groups: o.groups.iter().filter_map(|x| x.read().ok().map(|x| x.to_full_dto())).collect() },
            ReadDesignCommand::ReadDesignGroupsShallowCommand => ReadDesignCommandOutput::ReadDesignGroupsShallowCommand { groups: o.groups.iter().filter_map(|x| x.read().ok().map(|x| x.to_shallow_dto())).collect() },
            ReadDesignCommand::ReadDesignAuthorsFullCommand => ReadDesignCommandOutput::ReadDesignAuthorsFullCommand { authors: o.authors.iter().filter_map(|a| a.read().ok().map(|a| a.to_full_dto())).collect() },
            ReadDesignCommand::ReadDesignAuthorsShallowCommand => ReadDesignCommandOutput::ReadDesignAuthorsShallowCommand { authors: o.authors.iter().filter_map(|a| a.read().ok().map(|a| a.to_shallow_dto())).collect() },
            ReadDesignCommand::ReadDesignConceptsFullCommand => ReadDesignCommandOutput::ReadDesignConceptsFullCommand { concepts: o.concepts.iter().filter_map(|c| c.read().ok().map(|c| c.to_full_dto())).collect() },
            ReadDesignCommand::ReadDesignConceptsShallowCommand => ReadDesignCommandOutput::ReadDesignConceptsShallowCommand { concepts: o.concepts.iter().filter_map(|c| c.read().ok().map(|c| c.to_shallow_dto())).collect() },
            ReadDesignCommand::ReadDesignTagsFullCommand => ReadDesignCommandOutput::ReadDesignTagsFullCommand { tags: o.tags.iter().filter_map(|t| t.read().ok().map(|t| t.to_full_dto())).collect() },
            ReadDesignCommand::ReadDesignTagsShallowCommand => ReadDesignCommandOutput::ReadDesignTagsShallowCommand { tags: o.tags.iter().filter_map(|t| t.read().ok().map(|t| t.to_shallow_dto())).collect() },
            ReadDesignCommand::ReadDesignQualitiesFullCommand => ReadDesignCommandOutput::ReadDesignQualitiesFullCommand { qualities: o.qualities.iter().filter_map(|q| q.read().ok().map(|q| q.to_full_dto())).collect() },
            ReadDesignCommand::ReadDesignQualitiesShallowCommand => ReadDesignCommandOutput::ReadDesignQualitiesShallowCommand { qualities: o.qualities.iter().filter_map(|q| q.read().ok().map(|q| q.to_shallow_dto())).collect() },
            ReadDesignCommand::ReadDesignPropsFullCommand => ReadDesignCommandOutput::ReadDesignPropsFullCommand { props: o.props.iter().filter_map(|p| p.read().ok().map(|p| p.to_full_dto())).collect() },
            ReadDesignCommand::ReadDesignPropsShallowCommand => ReadDesignCommandOutput::ReadDesignPropsShallowCommand { props: o.props.iter().filter_map(|p| p.read().ok().map(|p| p.to_shallow_dto())).collect() },
            ReadDesignCommand::ReadDesignAttributesFullCommand => ReadDesignCommandOutput::ReadDesignAttributesFullCommand { attributes: o.attributes.iter().filter_map(|a| a.read().ok().map(|a| a.to_full_dto())).collect() },
            ReadDesignCommand::ReadDesignAttributesShallowCommand => ReadDesignCommandOutput::ReadDesignAttributesShallowCommand { attributes: o.attributes.iter().filter_map(|a| a.read().ok().map(|a| a.to_shallow_dto())).collect() },
            ReadDesignCommand::ReadDesignStatsFullCommand => ReadDesignCommandOutput::ReadDesignStatsFullCommand { stats: o.stats.iter().filter_map(|s| s.read().ok().map(|s| s.to_full_dto())).collect() },
            ReadDesignCommand::ReadDesignStatsShallowCommand => ReadDesignCommandOutput::ReadDesignStatsShallowCommand { stats: o.stats.iter().filter_map(|s| s.read().ok().map(|s| s.to_shallow_dto())).collect() },
            ReadDesignCommand::ReadDesignFlattenMapCommand => {
                let m = o.flatten_map();
                let entries: Vec<DesignFlattenMapEntryDto> = m.into_iter().map(|(piece_id, (plane, center))| DesignFlattenMapEntryDto { piece_id, plane, center }).collect();
                ReadDesignCommandOutput::ReadDesignFlattenMapCommand { entries }
            }
            ReadDesignCommand::ReadDesignFamilyCommands { id, commands } => {
                let f = o
                    .families
                    .iter()
                    .filter_map(|w| w.upgrade())
                    .find(|f| f.read().map(|r| r.id == id.id).unwrap_or(false))
                    .ok_or_else(|| nf("Family", &id.id))?;
                let mut results = Vec::with_capacity(commands.len());
                for c in commands {
                    results.push(c.execute(&f)?);
                }
                ReadDesignCommandOutput::ReadDesignFamilyCommands { results }
            }
            ReadDesignCommand::ReadDesignPieceCommands { id, commands } => {
                let p = o.piece(id.id.as_str()).ok_or_else(|| nf("Piece", &id.id))?;
                let mut results = Vec::with_capacity(commands.len());
                for c in commands {
                    results.push(c.execute(&p, g)?);
                }
                ReadDesignCommandOutput::ReadDesignPieceCommands { results }
            }
            ReadDesignCommand::ReadDesignConnectionCommands { id, commands } => {
                let c = o.connection(id.id.as_str()).ok_or_else(|| nf("Connection", &id.id))?;
                let mut results = Vec::with_capacity(commands.len());
                for x in commands {
                    results.push(x.execute(&c)?);
                }
                ReadDesignCommandOutput::ReadDesignConnectionCommands { results }
            }
            ReadDesignCommand::ReadDesignLayerCommands { id, commands } => {
                let l = o.layer(id.id.as_str()).ok_or_else(|| nf("Layer", &id.id))?;
                let mut results = Vec::with_capacity(commands.len());
                for c in commands {
                    results.push(c.execute(&l)?);
                }
                ReadDesignCommandOutput::ReadDesignLayerCommands { results }
            }
            ReadDesignCommand::ReadDesignGroupCommands { id, commands } => {
                let l = o.group(id.id.as_str()).ok_or_else(|| nf("Group", &id.id))?;
                let mut results = Vec::with_capacity(commands.len());
                for c in commands {
                    results.push(c.execute(&l)?);
                }
                ReadDesignCommandOutput::ReadDesignGroupCommands { results }
            }
            ReadDesignCommand::ReadDesignAuthorCommands { id, commands } => {
                let a = o.authors.iter().find(|a| a.read().map(|x| x.id == id.id).unwrap_or(false)).ok_or_else(|| nf("Author", &id.id))?;
                let mut results = Vec::with_capacity(commands.len());
                for c in commands {
                    results.push(c.execute(a)?);
                }
                ReadDesignCommandOutput::ReadDesignAuthorCommands { results }
            }
            ReadDesignCommand::ReadDesignConceptCommands { id, commands } => {
                let a = o.concepts.iter().find(|a| a.read().map(|x| x.id == id.id).unwrap_or(false)).ok_or_else(|| nf("Concept", &id.id))?;
                let mut results = Vec::with_capacity(commands.len());
                for c in commands {
                    results.push(c.execute(a)?);
                }
                ReadDesignCommandOutput::ReadDesignConceptCommands { results }
            }
            ReadDesignCommand::ReadDesignTagCommands { id, commands } => {
                let a = o.tags.iter().find(|a| a.read().map(|x| x.id == id.id).unwrap_or(false)).ok_or_else(|| nf("Tag", &id.id))?;
                let mut results = Vec::with_capacity(commands.len());
                for c in commands {
                    results.push(c.execute_ref(a)?);
                }
                ReadDesignCommandOutput::ReadDesignTagCommands { results }
            }
            ReadDesignCommand::ReadDesignQualityCommands { id, commands } => {
                let a = o.qualities.iter().find(|a| a.read().map(|x| x.id == id.id).unwrap_or(false)).ok_or_else(|| nf("Quality", &id.id))?;
                let mut results = Vec::with_capacity(commands.len());
                for c in commands {
                    results.push(c.execute(a)?);
                }
                ReadDesignCommandOutput::ReadDesignQualityCommands { results }
            }
            ReadDesignCommand::ReadDesignPropCommands { id, commands } => {
                let a = o.props.iter().find(|a| a.read().map(|x| x.id == id.id).unwrap_or(false)).ok_or_else(|| nf("Prop", &id.id))?;
                let mut results = Vec::with_capacity(commands.len());
                for c in commands {
                    results.push(c.execute(a)?);
                }
                ReadDesignCommandOutput::ReadDesignPropCommands { results }
            }
            ReadDesignCommand::ReadDesignAttributeCommands { id, commands } => {
                let a = o.attributes.iter().find(|a| a.read().map(|x| x.id == id.id).unwrap_or(false)).ok_or_else(|| nf("Attribute", &id.id))?;
                let mut results = Vec::with_capacity(commands.len());
                for c in commands {
                    results.push(c.execute_ref(a)?);
                }
                ReadDesignCommandOutput::ReadDesignAttributeCommands { results }
            }
            ReadDesignCommand::ReadDesignStatCommands { id, commands } => {
                let a = o.stats.iter().find(|a| a.read().map(|x| x.id == id.id).unwrap_or(false)).ok_or_else(|| nf("Stat", &id.id))?;
                let mut results = Vec::with_capacity(commands.len());
                for c in commands {
                    results.push(c.execute(a)?);
                }
                ReadDesignCommandOutput::ReadDesignStatCommands { results }
            }
        })
    }
}

impl ReadKitCommand {
    pub fn execute(&self, g: &KitGraph) -> Result<ReadKitCommandOutput> {
        Ok(match self {
            ReadKitCommand::ReadKitFullCommand => ReadKitCommandOutput::ReadKitFullCommand { kit: g.to_full_dto() },
            ReadKitCommand::ReadKitShallowCommand => ReadKitCommandOutput::ReadKitShallowCommand { kit: g.to_shallow_dto() },
            ReadKitCommand::ReadKitMetadataCommand => ReadKitCommandOutput::ReadKitMetadataCommand { metadata: g.to_metadata_dto() },
            ReadKitCommand::ReadKitIdCommand => ReadKitCommandOutput::ReadKitIdCommand { id: g.to_id_dto() },
            ReadKitCommand::ReadKitNameCommand => ReadKitCommandOutput::ReadKitNameCommand { name: g.name.clone() },
            ReadKitCommand::ReadKitDescriptionCommand => ReadKitCommandOutput::ReadKitDescriptionCommand { description: g.description.clone() },
            ReadKitCommand::ReadKitIconCommand => ReadKitCommandOutput::ReadKitIconCommand { icon: g.icon.clone() },
            ReadKitCommand::ReadKitImageCommand => ReadKitCommandOutput::ReadKitImageCommand { image: g.image.clone() },
            ReadKitCommand::ReadKitPreviewCommand => ReadKitCommandOutput::ReadKitPreviewCommand { preview: g.preview.clone() },
            ReadKitCommand::ReadKitRemoteCommand => ReadKitCommandOutput::ReadKitRemoteCommand { remote: g.remote.clone() },
            ReadKitCommand::ReadKitHomepageCommand => ReadKitCommandOutput::ReadKitHomepageCommand { homepage: g.homepage.clone() },
            ReadKitCommand::ReadKitLicenseCommand => ReadKitCommandOutput::ReadKitLicenseCommand { license: g.license.clone() },
            ReadKitCommand::ReadKitUriCommand => ReadKitCommandOutput::ReadKitUriCommand { uri: g.uri.clone() },
            ReadKitCommand::ReadKitCreatedCommand => ReadKitCommandOutput::ReadKitCreatedCommand { created: g.created.clone() },
            ReadKitCommand::ReadKitUpdatedCommand => ReadKitCommandOutput::ReadKitUpdatedCommand { updated: g.updated.clone() },
            ReadKitCommand::ReadKitTypesFullCommand => ReadKitCommandOutput::ReadKitTypesFullCommand { types: g.types.iter().filter_map(|t| t.read().ok().map(|t| t.to_full_dto())).collect() },
            ReadKitCommand::ReadKitTypesShallowCommand => ReadKitCommandOutput::ReadKitTypesShallowCommand { types: g.types.iter().filter_map(|t| t.read().ok().map(|t| t.to_shallow_dto())).collect() },
            ReadKitCommand::ReadKitDesignsFullCommand => ReadKitCommandOutput::ReadKitDesignsFullCommand { designs: g.designs.iter().filter_map(|d| d.read().ok().map(|d| d.to_full_dto())).collect() },
            ReadKitCommand::ReadKitDesignsShallowCommand => ReadKitCommandOutput::ReadKitDesignsShallowCommand { designs: g.designs.iter().filter_map(|d| d.read().ok().map(|d| d.to_shallow_dto())).collect() },
            ReadKitCommand::ReadKitFilesFullCommand => ReadKitCommandOutput::ReadKitFilesFullCommand { files: g.files.iter().filter_map(|f| f.read().ok().map(|f| f.to_full_dto())).collect() },
            ReadKitCommand::ReadKitFilesShallowCommand => ReadKitCommandOutput::ReadKitFilesShallowCommand { files: g.files.iter().filter_map(|f| f.read().ok().map(|f| f.to_shallow_dto())).collect() },
            ReadKitCommand::ReadKitFoldersFullCommand => ReadKitCommandOutput::ReadKitFoldersFullCommand { folders: g.folders.iter().filter_map(|f| f.read().ok().map(|f| f.to_full_dto())).collect() },
            ReadKitCommand::ReadKitFoldersShallowCommand => ReadKitCommandOutput::ReadKitFoldersShallowCommand { folders: g.folders.iter().filter_map(|f| f.read().ok().map(|f| f.to_shallow_dto())).collect() },
            ReadKitCommand::ReadKitLocationsFullCommand => ReadKitCommandOutput::ReadKitLocationsFullCommand { locations: g.locations.iter().filter_map(|l| l.read().ok().map(|l| l.to_full_dto())).collect() },
            ReadKitCommand::ReadKitLocationsShallowCommand => ReadKitCommandOutput::ReadKitLocationsShallowCommand { locations: g.locations.iter().filter_map(|l| l.read().ok().map(|l| l.to_shallow_dto())).collect() },
            ReadKitCommand::ReadKitFamiliesFullCommand => ReadKitCommandOutput::ReadKitFamiliesFullCommand { families: g.families.iter().filter_map(|f| f.read().ok().map(|f| f.to_full_dto())).collect() },
            ReadKitCommand::ReadKitFamiliesShallowCommand => ReadKitCommandOutput::ReadKitFamiliesShallowCommand { families: g.families.iter().filter_map(|f| f.read().ok().map(|f| f.to_shallow_dto())).collect() },
            ReadKitCommand::ReadKitPortsFullCommand => {
                let mut ports: Vec<PortFullDto> = g.ports.iter().filter_map(|p| p.read().ok().map(|p| p.to_full_dto())).collect();
                for f in &g.families {
                    if let Ok(fr) = f.read() {
                        for p in &fr.ports {
                            if let Ok(pr) = p.read() {
                                ports.push(pr.to_full_dto());
                            }
                        }
                    }
                }
                ReadKitCommandOutput::ReadKitPortsFullCommand { ports }
            }
            ReadKitCommand::ReadKitAuthorsFullCommand => ReadKitCommandOutput::ReadKitAuthorsFullCommand { authors: g.authors.iter().filter_map(|a| a.read().ok().map(|a| a.to_full_dto())).collect() },
            ReadKitCommand::ReadKitAuthorsShallowCommand => ReadKitCommandOutput::ReadKitAuthorsShallowCommand { authors: g.authors.iter().filter_map(|a| a.read().ok().map(|a| a.to_shallow_dto())).collect() },
            ReadKitCommand::ReadKitConceptsFullCommand => ReadKitCommandOutput::ReadKitConceptsFullCommand { concepts: g.concepts.iter().filter_map(|c| c.read().ok().map(|c| c.to_full_dto())).collect() },
            ReadKitCommand::ReadKitConceptsShallowCommand => ReadKitCommandOutput::ReadKitConceptsShallowCommand { concepts: g.concepts.iter().filter_map(|c| c.read().ok().map(|c| c.to_shallow_dto())).collect() },
            ReadKitCommand::ReadKitTagsFullCommand => ReadKitCommandOutput::ReadKitTagsFullCommand { tags: g.tags.iter().filter_map(|t| t.read().ok().map(|t| t.to_full_dto())).collect() },
            ReadKitCommand::ReadKitTagsShallowCommand => ReadKitCommandOutput::ReadKitTagsShallowCommand { tags: g.tags.iter().filter_map(|t| t.read().ok().map(|t| t.to_shallow_dto())).collect() },
            ReadKitCommand::ReadKitQualitiesFullCommand => ReadKitCommandOutput::ReadKitQualitiesFullCommand { qualities: g.qualities.iter().filter_map(|q| q.read().ok().map(|q| q.to_full_dto())).collect() },
            ReadKitCommand::ReadKitQualitiesShallowCommand => ReadKitCommandOutput::ReadKitQualitiesShallowCommand { qualities: g.qualities.iter().filter_map(|q| q.read().ok().map(|q| q.to_shallow_dto())).collect() },
            ReadKitCommand::ReadKitPropsFullCommand => ReadKitCommandOutput::ReadKitPropsFullCommand { props: g.props.iter().filter_map(|p| p.read().ok().map(|p| p.to_full_dto())).collect() },
            ReadKitCommand::ReadKitPropsShallowCommand => ReadKitCommandOutput::ReadKitPropsShallowCommand { props: g.props.iter().filter_map(|p| p.read().ok().map(|p| p.to_shallow_dto())).collect() },
            ReadKitCommand::ReadKitAttributesFullCommand => ReadKitCommandOutput::ReadKitAttributesFullCommand { attributes: g.attributes.iter().filter_map(|a| a.read().ok().map(|a| a.to_full_dto())).collect() },
            ReadKitCommand::ReadKitAttributesShallowCommand => ReadKitCommandOutput::ReadKitAttributesShallowCommand { attributes: g.attributes.iter().filter_map(|a| a.read().ok().map(|a| a.to_shallow_dto())).collect() },
            ReadKitCommand::ReadKitTypeCommands { id, commands } => {
                let t = kit_type(g, &id.id).ok_or_else(|| nf("Type", &id.id))?;
                let mut results = Vec::with_capacity(commands.len());
                for c in commands {
                    results.push(c.execute(&t, g)?);
                }
                ReadKitCommandOutput::ReadKitTypeCommands { results }
            }
            ReadKitCommand::ReadKitDesignCommands { id, commands } => {
                let d = kit_design(g, &id.id).ok_or_else(|| nf("Design", &id.id))?;
                let mut results = Vec::with_capacity(commands.len());
                for c in commands {
                    results.push(c.execute(&d, g)?);
                }
                ReadKitCommandOutput::ReadKitDesignCommands { results }
            }
            ReadKitCommand::ReadKitFileCommands { id, commands } => {
                let f = g.file(id.id.as_str()).ok_or_else(|| nf("File", &id.id))?;
                let mut results = Vec::with_capacity(commands.len());
                for c in commands {
                    results.push(c.execute(&f)?);
                }
                ReadKitCommandOutput::ReadKitFileCommands { results }
            }
            ReadKitCommand::ReadKitFolderCommands { id, commands } => {
                let f = g.folder(id.id.as_str()).ok_or_else(|| nf("Folder", &id.id))?;
                let mut results = Vec::with_capacity(commands.len());
                for c in commands {
                    results.push(c.execute(&f)?);
                }
                ReadKitCommandOutput::ReadKitFolderCommands { results }
            }
            ReadKitCommand::ReadKitLocationCommands { id, commands } => {
                let l = kit_location(g, &id.id).ok_or_else(|| nf("Location", &id.id))?;
                let mut results = Vec::with_capacity(commands.len());
                for c in commands {
                    results.push(c.execute(&l)?);
                }
                ReadKitCommandOutput::ReadKitLocationCommands { results }
            }
            ReadKitCommand::ReadKitFamilyCommands { id, commands } => {
                let f = kit_family(g, &id.id).ok_or_else(|| nf("Family", &id.id))?;
                let mut results = Vec::with_capacity(commands.len());
                for c in commands {
                    results.push(c.execute(&f)?);
                }
                ReadKitCommandOutput::ReadKitFamilyCommands { results }
            }
            ReadKitCommand::ReadKitPortCommands { id, commands } => {
                let p = g.port_by_id(&id.id).ok_or_else(|| nf("Port", &id.id))?;
                let mut results = Vec::with_capacity(commands.len());
                for c in commands {
                    results.push(c.execute(&p)?);
                }
                ReadKitCommandOutput::ReadKitPortCommands { results }
            }
            ReadKitCommand::ReadKitAuthorCommands { id, commands } => {
                let a = kit_find_author(g, &id.id).ok_or_else(|| nf("Author", &id.id))?;
                let mut results = Vec::with_capacity(commands.len());
                for c in commands {
                    results.push(c.execute(&a)?);
                }
                ReadKitCommandOutput::ReadKitAuthorCommands { results }
            }
            ReadKitCommand::ReadKitConceptCommands { id, commands } => {
                let a = kit_find_concept(g, &id.id).ok_or_else(|| nf("Concept", &id.id))?;
                let mut results = Vec::with_capacity(commands.len());
                for c in commands {
                    results.push(c.execute(&a)?);
                }
                ReadKitCommandOutput::ReadKitConceptCommands { results }
            }
            ReadKitCommand::ReadKitTagCommands { id, commands } => {
                let a = kit_find_tag(g, &id.id).ok_or_else(|| nf("Tag", &id.id))?;
                let mut results = Vec::with_capacity(commands.len());
                for c in commands {
                    results.push(c.execute_ref(&a)?);
                }
                ReadKitCommandOutput::ReadKitTagCommands { results }
            }
            ReadKitCommand::ReadKitQualityCommands { id, commands } => {
                let a = kit_find_quality(g, &id.id).ok_or_else(|| nf("Quality", &id.id))?;
                let mut results = Vec::with_capacity(commands.len());
                for c in commands {
                    results.push(c.execute(&a)?);
                }
                ReadKitCommandOutput::ReadKitQualityCommands { results }
            }
            ReadKitCommand::ReadKitPropCommands { id, commands } => {
                let a = kit_find_prop(g, &id.id).ok_or_else(|| nf("Prop", &id.id))?;
                let mut results = Vec::with_capacity(commands.len());
                for c in commands {
                    results.push(c.execute(&a)?);
                }
                ReadKitCommandOutput::ReadKitPropCommands { results }
            }
            ReadKitCommand::ReadKitAttributeCommands { id, commands } => {
                let a = kit_find_attr(g, &id.id).ok_or_else(|| nf("Attribute", &id.id))?;
                let mut results = Vec::with_capacity(commands.len());
                for c in commands {
                    results.push(c.execute_ref(&a)?);
                }
                ReadKitCommandOutput::ReadKitAttributeCommands { results }
            }
        })
    }

    pub fn execute_many(g: &KitGraph, commands: &[ReadKitCommand]) -> Result<Vec<ReadKitCommandOutput>> {
        commands.iter().map(|c| c.execute(g)).collect()
    }
}
