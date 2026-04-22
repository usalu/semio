// Appended to change_command_ext via include!.

impl ChangeDesignCommand {
    pub fn run(&self, kit: &KitStoreRef, design_id: &Id) -> Result<()> {
        self.apply(kit, design_id)?;
        Ok(())
    }
    /// Inverse atoms in forward order; [`ChangeKitCommand`] reverses the batch.
    pub fn apply(&self, kit: &KitStoreRef, design_id: &Id) -> Result<Vec<ChangeDesignCommand>> {
        let d = kit
            .read()
            .map_err(|_| SemioError::LockPoisoned("kit"))?
            .design(design_id.as_str())
            .ok_or_else(|| SemioError::NotFound {
                kind: "Design",
                id: design_id.clone(),
            })?;
        match self {
            ChangeDesignCommand::Name { name } => {
                let old = d.read().map_err(|_| SemioError::LockPoisoned("design"))?.name.clone();
                d.write().map_err(|_| SemioError::LockPoisoned("design"))?.set_name(name.clone()).map_err(se)?;
                if old == *name { Ok(vec![]) } else { Ok(vec![ChangeDesignCommand::Name { name: old }]) }
            }
            ChangeDesignCommand::Description { description } => {
                let old = d.read().map_err(|_| SemioError::LockPoisoned("design"))?.description.clone();
                d.write()
                    .map_err(|_| SemioError::LockPoisoned("design"))?
                    .set_description(description.clone())
                    .map_err(se)?;
                if old == *description { Ok(vec![]) } else { Ok(vec![ChangeDesignCommand::Description { description: old }]) }
            }
            ChangeDesignCommand::Icon { icon } => {
                let old = d.read().map_err(|_| SemioError::LockPoisoned("design"))?.icon.clone();
                d.write()
                    .map_err(|_| SemioError::LockPoisoned("design"))?
                    .set_icon(icon.clone())
                    .map_err(se)?;
                if old == *icon { Ok(vec![]) } else { Ok(vec![ChangeDesignCommand::Icon { icon: old }]) }
            }
            ChangeDesignCommand::Image { image } => {
                let old = d.read().map_err(|_| SemioError::LockPoisoned("design"))?.image.clone();
                d.write()
                    .map_err(|_| SemioError::LockPoisoned("design"))?
                    .set_image(image.clone())
                    .map_err(se)?;
                if old == *image { Ok(vec![]) } else { Ok(vec![ChangeDesignCommand::Image { image: old }]) }
            }
            ChangeDesignCommand::Variant { variant } => {
                let old = d.read().map_err(|_| SemioError::LockPoisoned("design"))?.variant.clone();
                d.write()
                    .map_err(|_| SemioError::LockPoisoned("design"))?
                    .set_variant(variant.clone())
                    .map_err(se)?;
                if old == *variant { Ok(vec![]) } else { Ok(vec![ChangeDesignCommand::Variant { variant: old }]) }
            }
            ChangeDesignCommand::View { view } => {
                let old = d.read().map_err(|_| SemioError::LockPoisoned("design"))?.view.clone();
                d.write()
                    .map_err(|_| SemioError::LockPoisoned("design"))?
                    .set_view(view.clone())
                    .map_err(se)?;
                if old == *view { Ok(vec![]) } else { Ok(vec![ChangeDesignCommand::View { view: old }]) }
            }
            ChangeDesignCommand::Location { location } => {
                let old = d.read().map_err(|_| SemioError::LockPoisoned("design"))?.location;
                d.write()
                    .map_err(|_| SemioError::LockPoisoned("design"))?
                    .set_location(*location)
                    .map_err(se)?;
                if old == *location { Ok(vec![]) } else { Ok(vec![ChangeDesignCommand::Location { location: old }]) }
            }
            ChangeDesignCommand::Camera { camera } => {
                let old = d.read().map_err(|_| SemioError::LockPoisoned("design"))?.camera;
                d.write()
                    .map_err(|_| SemioError::LockPoisoned("design"))?
                    .set_camera(*camera)
                    .map_err(se)?;
                if old == *camera { Ok(vec![]) } else { Ok(vec![ChangeDesignCommand::Camera { camera: old }]) }
            }
            ChangeDesignCommand::Unit { unit } => {
                let old = d.read().map_err(|_| SemioError::LockPoisoned("design"))?.unit.clone();
                d.write()
                    .map_err(|_| SemioError::LockPoisoned("design"))?
                    .set_unit(unit.clone())
                    .map_err(se)?;
                if old == *unit { Ok(vec![]) } else { Ok(vec![ChangeDesignCommand::Unit { unit: old }]) }
            }
            ChangeDesignCommand::Created { created } => {
                let old = d.read().map_err(|_| SemioError::LockPoisoned("design"))?.created.clone();
                d.write()
                    .map_err(|_| SemioError::LockPoisoned("design"))?
                    .set_created(created.clone())
                    .map_err(se)?;
                if old == *created { Ok(vec![]) } else { Ok(vec![ChangeDesignCommand::Created { created: old }]) }
            }
            ChangeDesignCommand::Updated { updated } => {
                let old = d.read().map_err(|_| SemioError::LockPoisoned("design"))?.updated.clone();
                d.write()
                    .map_err(|_| SemioError::LockPoisoned("design"))?
                    .set_updated(updated.clone())
                    .map_err(se)?;
                if old == *updated { Ok(vec![]) } else { Ok(vec![ChangeDesignCommand::Updated { updated: old }]) }
            }
            ChangeDesignCommand::AddPiece { piece } => {
                let id = piece.id.clone();
                let did = design_id.to_string();
                {
                    let mut g = kit.write().map_err(|_| SemioError::LockPoisoned("kit"))?;
                    let mut diff = DesignDiff::default();
                    diff.added_pieces.push(piece.clone());
                    g.apply_design_diff(&did, &diff).map_err(|e| SemioError::InvalidOperation(e.to_string()))?;
                }
                Ok(vec![ChangeDesignCommand::RemovePiece { piece_id: PieceIdDto { id } }])
            }
            ChangeDesignCommand::RemovePiece { piece_id } => {
                let did = design_id.to_string();
                let snap = {
                    let g = kit.read().map_err(|_| SemioError::LockPoisoned("kit"))?;
                    let dref = g.design(&did).ok_or_else(|| SemioError::NotFound {
                        kind: "Design",
                        id: design_id.clone(),
                    })?;
                    let dr = dref.read().map_err(|_| SemioError::LockPoisoned("design"))?;
                    let p = dr
                        .piece(piece_id.id.as_str())
                        .ok_or_else(|| SemioError::NotFound {
                            kind: "Piece",
                            id: piece_id.id.clone(),
                        })?;
                    let pr = p.read().map_err(|_| SemioError::LockPoisoned("piece"))?;
                    pr.to_full_dto()
                };
                {
                    let mut g = kit.write().map_err(|_| SemioError::LockPoisoned("kit"))?;
                    let mut diff = DesignDiff::default();
                    diff.removed_pieces.push(piece_id.clone());
                    g.apply_design_diff(&did, &diff).map_err(|e| SemioError::InvalidOperation(e.to_string()))?;
                }
                Ok(vec![ChangeDesignCommand::AddPiece { piece: snap }])
            }
            ChangeDesignCommand::ChangePieceCommands { piece_id, commands } => {
                let mut inverses: Vec<ChangePieceCommand> = Vec::new();
                for c in commands {
                    let v = c.apply(kit, design_id, &piece_id.id)?;
                    inverses.extend(v);
                }
                inverses.reverse();
                Ok(vec![ChangeDesignCommand::ChangePieceCommands { piece_id: piece_id.clone(), commands: inverses }])
            }
            ChangeDesignCommand::AddConnection { connection } => {
                let id = connection.id.clone();
                let did = design_id.to_string();
                {
                    let mut g = kit.write().map_err(|_| SemioError::LockPoisoned("kit"))?;
                    let mut diff = DesignDiff::default();
                    diff.added_connections.push(connection.clone());
                    g.apply_design_diff(&did, &diff).map_err(|e| SemioError::InvalidOperation(e.to_string()))?;
                }
                Ok(vec![ChangeDesignCommand::RemoveConnection {
                    connection_id: ConnectionIdDto { id },
                }])
            }
            ChangeDesignCommand::RemoveConnection { connection_id } => {
                let did = design_id.to_string();
                let snap = {
                    let g = kit.read().map_err(|_| SemioError::LockPoisoned("kit"))?;
                    let dref = g.design(&did).ok_or_else(|| SemioError::NotFound {
                        kind: "Design",
                        id: design_id.clone(),
                    })?;
                    let dr = dref.read().map_err(|_| SemioError::LockPoisoned("design"))?;
                    dr.connections
                        .iter()
                        .find(|c| c.read().map(|r| r.id == connection_id.id).unwrap_or(false))
                        .and_then(|c| c.read().ok().map(|c| c.to_full_dto()))
                }
                .ok_or_else(|| SemioError::NotFound {
                    kind: "Connection",
                    id: connection_id.id.clone(),
                })?;
                {
                    let mut g = kit.write().map_err(|_| SemioError::LockPoisoned("kit"))?;
                    let mut diff = DesignDiff::default();
                    diff.removed_connections.push(connection_id.clone());
                    g.apply_design_diff(&did, &diff).map_err(|e| SemioError::InvalidOperation(e.to_string()))?;
                }
                Ok(vec![ChangeDesignCommand::AddConnection { connection: snap }])
            }
            ChangeDesignCommand::ChangeConnectionCommands { connection_id, commands } => {
                let mut inv = Vec::new();
                for c in commands {
                    let v = c.apply(kit, design_id, &connection_id.id)?;
                    inv.extend(v);
                }
                inv.reverse();
                Ok(vec![ChangeDesignCommand::ChangeConnectionCommands {
                    connection_id: connection_id.clone(),
                    commands: inv,
                }])
            }
            ChangeDesignCommand::AddLayer { layer: _ } => Err(SemioError::InvalidOperation("AddLayer: wire to DesignStore.layers + LayerStore::from_full_dto".into())),
            ChangeDesignCommand::RemoveLayer { .. } => Err(SemioError::InvalidOperation("RemoveLayer: not yet wired".into())),
            ChangeDesignCommand::ChangeLayerCommands { .. } => Err(SemioError::InvalidOperation("ChangeLayerCommands: not yet wired".into())),
            ChangeDesignCommand::AddGroup { .. } | ChangeDesignCommand::RemoveGroup { .. } | ChangeDesignCommand::ChangeGroupCommands { .. } => {
                Err(SemioError::InvalidOperation("Group change: not yet wired".into()))
            }
            ChangeDesignCommand::AddStat { .. } | ChangeDesignCommand::RemoveStat { .. } | ChangeDesignCommand::ChangeStatCommands { .. } => {
                Err(SemioError::InvalidOperation("Stat change: not yet wired".into()))
            }
            ChangeDesignCommand::AddDesignAuthor { .. }
            | ChangeDesignCommand::RemoveDesignAuthor { .. }
            | ChangeDesignCommand::AddDesignConcept { .. }
            | ChangeDesignCommand::RemoveDesignConcept { .. }
            | ChangeDesignCommand::AddDesignTag { .. }
            | ChangeDesignCommand::RemoveDesignTag { .. }
            | ChangeDesignCommand::AddDesignQuality { .. }
            | ChangeDesignCommand::RemoveDesignQuality { .. }
            | ChangeDesignCommand::AddDesignProp { .. }
            | ChangeDesignCommand::RemoveDesignProp { .. }
            | ChangeDesignCommand::AddDesignAttribute { .. }
            | ChangeDesignCommand::RemoveDesignAttribute { .. } => Err(SemioError::InvalidOperation("design-scoped child entity: not yet wired (push on DesignStore vectors)".into())),
            ChangeDesignCommand::Other => Ok(vec![]),
        }
    }
}

impl ChangePieceCommand {
    pub fn run(&self, kit: &KitStoreRef, design_id: &Id, piece_id: &Id) -> Result<()> {
        self.apply(kit, design_id, piece_id)?;
        Ok(())
    }
    pub fn apply(&self, kit: &KitStoreRef, design_id: &Id, piece_id: &Id) -> Result<Vec<ChangePieceCommand>> {
        let pref = {
            let g = kit.read().map_err(|_| SemioError::LockPoisoned("kit"))?;
            let d = g
                .design(design_id.as_str())
                .ok_or_else(|| SemioError::NotFound { kind: "Design", id: design_id.clone() })?;
            let dr = d.read().map_err(|_| SemioError::LockPoisoned("design"))?;
            dr.piece(piece_id.as_str())
                .ok_or_else(|| SemioError::NotFound { kind: "Piece", id: piece_id.clone() })?
        };
        match self {
            ChangePieceCommand::Name { name } => {
                let old = pref.read().map_err(|_| SemioError::LockPoisoned("piece"))?.name.clone();
                pref.write()
                    .map_err(|_| SemioError::LockPoisoned("piece"))?
                    .set_name(name.clone())
                    .map_err(se)?;
                if old == *name { Ok(vec![]) } else { Ok(vec![ChangePieceCommand::Name { name: old }]) }
            }
            ChangePieceCommand::Description { description } => {
                let old = pref.read().map_err(|_| SemioError::LockPoisoned("piece"))?.description.clone();
                pref.write()
                    .map_err(|_| SemioError::LockPoisoned("piece"))?
                    .set_description(description.clone())
                    .map_err(se)?;
                if old == *description { Ok(vec![]) } else { Ok(vec![ChangePieceCommand::Description { description: old }]) }
            }
            ChangePieceCommand::Plane { plane } => {
                let old = pref.read().map_err(|_| SemioError::LockPoisoned("piece"))?.pose.plane;
                pref.write()
                    .map_err(|_| SemioError::LockPoisoned("piece"))?
                    .set_plane(plane.clone())
                    .map_err(se)?;
                if old == *plane { Ok(vec![]) } else { Ok(vec![ChangePieceCommand::Plane { plane: old }]) }
            }
            ChangePieceCommand::Center { center } => {
                let old = pref.read().map_err(|_| SemioError::LockPoisoned("piece"))?.pose.center;
                pref.write()
                    .map_err(|_| SemioError::LockPoisoned("piece"))?
                    .set_center(*center)
                    .map_err(se)?;
                if old == *center { Ok(vec![]) } else { Ok(vec![ChangePieceCommand::Center { center: old }]) }
            }
            ChangePieceCommand::Color { color } => {
                let old = pref.read().map_err(|_| SemioError::LockPoisoned("piece"))?.color.clone();
                pref.write()
                    .map_err(|_| SemioError::LockPoisoned("piece"))?
                    .set_color(color.clone())
                    .map_err(se)?;
                if old == *color { Ok(vec![]) } else { Ok(vec![ChangePieceCommand::Color { color: old }]) }
            }
            ChangePieceCommand::Type { type_id: tid } => {
                let old_weak = pref.read().map_err(|_| SemioError::LockPoisoned("piece"))?.type_ref.clone();
                let tref = tid.as_ref().and_then(|tip| {
                    kit.read()
                        .ok()?
                        .semio_type(tip.id.as_str())
                        .map(|t| Arc::downgrade(&t))
                });
                let old_tid = old_weak.as_ref().and_then(|w| w.upgrade()).and_then(|t| t.read().ok().map(|r| TypeIdDto { id: r.id.clone() }));
                pref.write()
                    .map_err(|_| SemioError::LockPoisoned("piece"))?
                    .set_type_weak(tref)
                    .map_err(se)?;
                Ok(vec![ChangePieceCommand::Type { type_id: old_tid }])
            }
            ChangePieceCommand::Scale { scale } => {
                let old = pref.read().map_err(|_| SemioError::LockPoisoned("piece"))?.scale;
                pref.write()
                    .map_err(|_| SemioError::LockPoisoned("piece"))?
                    .set_scale(*scale)
                    .map_err(se)?;
                if old == *scale { Ok(vec![]) } else { Ok(vec![ChangePieceCommand::Scale { scale: old }]) }
            }
            ChangePieceCommand::MirrorPlane { mirror_plane } => {
                let old = pref.read().map_err(|_| SemioError::LockPoisoned("piece"))?.mirror_plane;
                pref.write()
                    .map_err(|_| SemioError::LockPoisoned("piece"))?
                    .set_mirror_plane(mirror_plane.clone())
                    .map_err(se)?;
                if old == *mirror_plane { Ok(vec![]) } else { Ok(vec![ChangePieceCommand::MirrorPlane { mirror_plane: old }]) }
            }
            ChangePieceCommand::Hidden { hidden } => {
                let old = pref.read().map_err(|_| SemioError::LockPoisoned("piece"))?.hidden;
                pref.write()
                    .map_err(|_| SemioError::LockPoisoned("piece"))?
                    .set_hidden(*hidden)
                    .map_err(se)?;
                if old == *hidden { Ok(vec![]) } else { Ok(vec![ChangePieceCommand::Hidden { hidden: old }]) }
            }
            ChangePieceCommand::Locked { locked } => {
                let old = pref.read().map_err(|_| SemioError::LockPoisoned("piece"))?.locked;
                pref.write()
                    .map_err(|_| SemioError::LockPoisoned("piece"))?
                    .set_locked(*locked)
                    .map_err(se)?;
                if old == *locked { Ok(vec![]) } else { Ok(vec![ChangePieceCommand::Locked { locked: old }]) }
            }
            ChangePieceCommand::Id { id } => {
                let old = pref.read().map_err(|_| SemioError::LockPoisoned("piece"))?.id.clone();
                pref.write()
                    .map_err(|_| SemioError::LockPoisoned("piece"))?
                    .set_id(id.clone())
                    .map_err(se)?;
                if old == *id { Ok(vec![]) } else { Ok(vec![ChangePieceCommand::Id { id: old }]) }
            }
            ChangePieceCommand::AddProp { .. } | ChangePieceCommand::RemoveProp { .. } | ChangePieceCommand::ChangePropCommands { .. } => {
                Err(SemioError::InvalidOperation("piece prop batch: not yet fully wired; use a DesignDiff on modified_pieces for now".into()))
            }
            ChangePieceCommand::AddAttribute { .. } | ChangePieceCommand::RemoveAttribute { .. } | ChangePieceCommand::ChangeAttributeCommands { .. } => {
                Err(SemioError::InvalidOperation("piece attribute: not yet wired".into()))
            }
            ChangePieceCommand::Fix => {
                pref.write().map_err(|_| SemioError::LockPoisoned("piece"))?.fix()?;
                Err(SemioError::InvalidOperation(
                    "ChangePieceCommand::Fix: no parametric inverse; re-materialize from DTO for undo if needed".into(),
                ))
            }
            ChangePieceCommand::Other => Ok(vec![]),
        }
    }
}

impl ChangeConnectionCommand {
    pub fn run(&self, kit: &KitStoreRef, design_id: &Id, connection_id: &Id) -> Result<()> {
        self.apply(kit, design_id, connection_id)?;
        Ok(())
    }
    pub fn apply(&self, kit: &KitStoreRef, design_id: &Id, connection_id: &Id) -> Result<Vec<ChangeConnectionCommand>> {
        let cref = {
            let g = kit.read().map_err(|_| SemioError::LockPoisoned("kit"))?;
            let d = g
                .design(design_id.as_str())
                .ok_or_else(|| SemioError::NotFound { kind: "Design", id: design_id.clone() })?;
            let dr = d.read().map_err(|_| SemioError::LockPoisoned("design"))?;
            dr.connections
                .iter()
                .find(|c| c.read().map(|r| r.id == *connection_id).unwrap_or(false))
                .cloned()
                .ok_or_else(|| SemioError::NotFound { kind: "Connection", id: connection_id.clone() })?
        };
        match self {
            ChangeConnectionCommand::Gap { value } => {
                let old = cref.read().map_err(|_| SemioError::LockPoisoned("connection"))?.gap;
                cref.write()
                    .map_err(|_| SemioError::LockPoisoned("connection"))?
                    .set_gap(*value)
                    .map_err(se)?;
                if old == *value { Ok(vec![]) } else { Ok(vec![ChangeConnectionCommand::Gap { value: old }]) }
            }
            ChangeConnectionCommand::Shift { value } => {
                let old = cref.read().map_err(|_| SemioError::LockPoisoned("connection"))?.shift;
                cref.write()
                    .map_err(|_| SemioError::LockPoisoned("connection"))?
                    .set_shift(*value)
                    .map_err(se)?;
                if old == *value { Ok(vec![]) } else { Ok(vec![ChangeConnectionCommand::Shift { value: old }]) }
            }
            ChangeConnectionCommand::Rise { value } => {
                let old = cref.read().map_err(|_| SemioError::LockPoisoned("connection"))?.rise;
                cref.write()
                    .map_err(|_| SemioError::LockPoisoned("connection"))?
                    .set_rise(*value)
                    .map_err(se)?;
                if old == *value { Ok(vec![]) } else { Ok(vec![ChangeConnectionCommand::Rise { value: old }]) }
            }
            ChangeConnectionCommand::Rotation { value } => {
                let old = cref.read().map_err(|_| SemioError::LockPoisoned("connection"))?.rotation;
                cref.write()
                    .map_err(|_| SemioError::LockPoisoned("connection"))?
                    .set_rotation(*value)
                    .map_err(se)?;
                if old == *value { Ok(vec![]) } else { Ok(vec![ChangeConnectionCommand::Rotation { value: old }]) }
            }
            ChangeConnectionCommand::Turn { value } => {
                let old = cref.read().map_err(|_| SemioError::LockPoisoned("connection"))?.turn;
                cref.write()
                    .map_err(|_| SemioError::LockPoisoned("connection"))?
                    .set_turn(*value)
                    .map_err(se)?;
                if old == *value { Ok(vec![]) } else { Ok(vec![ChangeConnectionCommand::Turn { value: old }]) }
            }
            ChangeConnectionCommand::Tilt { value } => {
                let old = cref.read().map_err(|_| SemioError::LockPoisoned("connection"))?.tilt;
                cref.write()
                    .map_err(|_| SemioError::LockPoisoned("connection"))?
                    .set_tilt(*value)
                    .map_err(se)?;
                if old == *value { Ok(vec![]) } else { Ok(vec![ChangeConnectionCommand::Tilt { value: old }]) }
            }
            ChangeConnectionCommand::X { value } => {
                let old = cref.read().map_err(|_| SemioError::LockPoisoned("connection"))?.x;
                cref.write()
                    .map_err(|_| SemioError::LockPoisoned("connection"))?
                    .set_x(*value)
                    .map_err(se)?;
                if old == *value { Ok(vec![]) } else { Ok(vec![ChangeConnectionCommand::X { value: old }]) }
            }
            ChangeConnectionCommand::Y { value } => {
                let old = cref.read().map_err(|_| SemioError::LockPoisoned("connection"))?.y;
                cref.write()
                    .map_err(|_| SemioError::LockPoisoned("connection"))?
                    .set_y(*value)
                    .map_err(se)?;
                if old == *value { Ok(vec![]) } else { Ok(vec![ChangeConnectionCommand::Y { value: old }]) }
            }
            ChangeConnectionCommand::Description { value } => {
                let old = cref.read().map_err(|_| SemioError::LockPoisoned("connection"))?.description.clone();
                cref.write()
                    .map_err(|_| SemioError::LockPoisoned("connection"))?
                    .set_description(value.clone())
                    .map_err(se)?;
                if old == *value { Ok(vec![]) } else { Ok(vec![ChangeConnectionCommand::Description { value: old }]) }
            }
            ChangeConnectionCommand::ReplaceConnected { side: _ } | ChangeConnectionCommand::ReplaceConnecting { side: _ } => Err(SemioError::InvalidOperation("Replace side: requires SideStore rewire; use DesignDiff for now".into())),
            ChangeConnectionCommand::AddConnectionAttribute { .. } | ChangeConnectionCommand::RemoveConnectionAttribute { .. } => {
                Err(SemioError::InvalidOperation("connection attribute: not yet wired".into()))
            }
            ChangeConnectionCommand::Other => Ok(vec![]),
        }
    }
}

// Empty impls to satisfy the type graph for serde: callers use [`ChangeLayerCommand`], etc. in JSON.
impl ChangeLayerCommand {
    pub fn run(&self, _kit: &KitStoreRef, _design_id: &Id, _layer_id: &Id) -> Result<()> {
        Err(SemioError::InvalidOperation("ChangeLayerCommand: not yet wired".into()))
    }
    pub fn apply(&self, _kit: &KitStoreRef, _design_id: &Id, _layer_id: &Id) -> Result<Vec<ChangeLayerCommand>> {
        Err(SemioError::InvalidOperation("ChangeLayerCommand: not yet wired".into()))
    }
}
impl ChangeGroupCommand {
    pub fn run(&self, _kit: &KitStoreRef, _design_id: &Id, _group_id: &Id) -> Result<()> {
        Err(SemioError::InvalidOperation("ChangeGroupCommand: not yet wired".into()))
    }
    pub fn apply(&self, _kit: &KitStoreRef, _design_id: &Id, _group_id: &Id) -> Result<Vec<ChangeGroupCommand>> {
        Err(SemioError::InvalidOperation("ChangeGroupCommand: not yet wired".into()))
    }
}
impl ChangeStatCommand {
    pub fn run(&self, _kit: &KitStoreRef, _design_id: &Id, _stat_id: &Id) -> Result<()> {
        Err(SemioError::InvalidOperation("ChangeStatCommand: not yet wired".into()))
    }
    pub fn apply(&self, _kit: &KitStoreRef, _design_id: &Id, _stat_id: &Id) -> Result<Vec<ChangeStatCommand>> {
        Err(SemioError::InvalidOperation("ChangeStatCommand: not yet wired".into()))
    }
}
impl ChangePropCommand {
    pub fn run(&self, _kit: &KitStoreRef) -> Result<()> { Err(SemioError::InvalidOperation("ChangePropCommand: not yet wired".into())) }
    pub fn apply(&self, _kit: &KitStoreRef) -> Result<Vec<ChangePropCommand>> { Err(SemioError::InvalidOperation("ChangePropCommand: not yet wired".into())) }
}
impl ChangeAttributeCommand {
    pub fn run(&self, _kit: &KitStoreRef) -> Result<()> { Err(SemioError::InvalidOperation("ChangeAttributeCommand: not yet wired".into())) }
    pub fn apply(&self, _kit: &KitStoreRef) -> Result<Vec<ChangeAttributeCommand>> { Err(SemioError::InvalidOperation("ChangeAttributeCommand: not yet wired".into())) }
}
impl ChangePortCommand {
    pub fn run(&self, _kit: &KitStoreRef) -> Result<()> { Err(SemioError::InvalidOperation("ChangePortCommand: not yet wired".into())) }
    pub fn apply(&self, _kit: &KitStoreRef) -> Result<Vec<ChangePortCommand>> { Err(SemioError::InvalidOperation("ChangePortCommand: not yet wired".into())) }
}
impl ChangeConnectorCommand {
    pub fn run(&self, _kit: &KitStoreRef) -> Result<()> { Err(SemioError::InvalidOperation("ChangeConnectorCommand: not yet wired".into())) }
    pub fn apply(&self, _kit: &KitStoreRef) -> Result<Vec<ChangeConnectorCommand>> { Err(SemioError::InvalidOperation("ChangeConnectorCommand: not yet wired".into())) }
}
impl ChangeRepresentationCommand {
    pub fn run(&self, _kit: &KitStoreRef) -> Result<()> { Err(SemioError::InvalidOperation("ChangeRepresentationCommand: not yet wired".into())) }
    pub fn apply(&self, _kit: &KitStoreRef) -> Result<Vec<ChangeRepresentationCommand>> { Err(SemioError::InvalidOperation("ChangeRepresentationCommand: not yet wired".into())) }
}
