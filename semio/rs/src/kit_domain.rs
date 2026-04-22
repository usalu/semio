//! High-level design / kit commands ported from the JS domain layer.
//! Each command mutates through [`crate::kit::KitStore::apply_design_diff`] and related kit hooks.

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use crate::connection::{ConnectionFullDto, ConnectionIdDto};
use crate::design::DesignFullDto;
use crate::design::DesignStore;
use crate::diff::DesignDiff;
use crate::error::SetError;
use crate::error::SetResult;
use crate::event_wire;
use crate::geom::{Coordinate, Plane};
use crate::guid::Guid;
use crate::kit::{KitStore, KitStoreRef};
use crate::piece::{PieceFullDto, PieceIdDto};
use crate::typ::TypeIdDto;
use crate::typ::TypeStoreRef;

fn type_index(kit: &KitStore) -> HashMap<Guid, TypeStoreRef> {
    kit.types
        .iter()
        .filter_map(|t| t.read().ok().map(|r| (r.guid.clone(), t.clone())))
        .collect()
}

/// Insert a fully specified design (e.g. clustered child) into the kit graph.
pub fn insert_design_ref(kit: &KitStoreRef, dto: DesignFullDto) -> SetResult {
    let mut g = kit
        .write()
        .map_err(|_| SetError::LockPoisoned("kit".into()))?;
    if g
        .designs
        .iter()
        .any(|d| d.read().map(|r| r.guid.as_str() == dto.guid.as_str()).unwrap_or(false))
    {
        return Err(SetError::DuplicateGuid(format!("design {}", dto.guid)));
    }
    let idx = type_index(&g);
    let design = DesignStore::hydrate_from_full_dto(dto, &idx);
    {
        let kw = Arc::downgrade(kit);
        if let Ok(mut dw) = design.write() {
            dw.parent_kit = kw;
        }
    }
    g.designs.push(design);
    g.invalidate_hash();
    g.invalidate_validation();
    drop(g);
    event_wire::wire_graph_bus(kit);
    Ok(())
}

fn map_semio(e: crate::error::SemioError) -> SetError {
    match e {
        crate::error::SemioError::NotFound { kind, guid } => {
            SetError::NotFound(format!("{} {}", kind, guid.as_str()))
        }
        crate::error::SemioError::LockPoisoned(s) => SetError::LockPoisoned(s.to_string()),
        crate::error::SemioError::InvalidOperation(m) => SetError::Internal(m),
        crate::error::SemioError::Json(j) => SetError::InvalidValue(j.to_string()),
        crate::error::SemioError::Io(i) => SetError::Internal(i.to_string()),
        #[cfg(not(target_arch = "wasm32"))]
        crate::error::SemioError::Sqlite(s) => SetError::Internal(s.to_string()),
        #[cfg(not(target_arch = "wasm32"))]
        crate::error::SemioError::Zip(z) => SetError::Internal(z.to_string()),
        crate::error::SemioError::Other(o) => SetError::Internal(o),
    }
}

/// Cluster selected pieces into a new child design and rewire external connections.
pub fn cluster_pieces_cmd(
    kit: &KitStoreRef,
    design_guid: &str,
    piece_guids: Vec<String>,
    cluster_name: String,
) -> SetResult {
    if piece_guids.is_empty() {
        return Err(SetError::InvalidValue(
            "no piece IDs provided for clustering".into(),
        ));
    }
    let parent_dto: DesignFullDto = {
        let g = kit
            .read()
            .map_err(|_| SetError::LockPoisoned("kit".into()))?;
        let d = g
            .design(design_guid)
            .ok_or_else(|| SetError::NotFound(format!("design {design_guid}")))?;
        let dr = d
            .read()
            .map_err(|_| SetError::LockPoisoned("design".into()))?;
        dr.to_full_dto()
    };

    let cluster_set: HashSet<&str> = piece_guids.iter().map(|s| s.as_str()).collect();
    let clustered_pieces: Vec<PieceFullDto> = parent_dto
        .pieces
        .iter()
        .filter(|p| cluster_set.contains(p.guid.as_str()))
        .cloned()
        .collect();
    if clustered_pieces.is_empty() {
        return Err(SetError::InvalidValue(
            "no pieces found matching the provided IDs".into(),
        ));
    }

    let internal_connections: Vec<ConnectionFullDto> = parent_dto
        .connections
        .iter()
        .filter(|c| {
            cluster_set.contains(c.connected.piece.guid.as_str())
                && cluster_set.contains(c.connecting.piece.guid.as_str())
        })
        .cloned()
        .collect();

    let external_connections: Vec<ConnectionFullDto> = parent_dto
        .connections
        .iter()
        .filter(|c| {
            let a = cluster_set.contains(c.connected.piece.guid.as_str());
            let b = cluster_set.contains(c.connecting.piece.guid.as_str());
            a != b
        })
        .cloned()
        .collect();

    let new_guid = Guid::new_v7();
    let clustered_dto = DesignFullDto {
        guid: new_guid.clone(),
        name: cluster_name,
        description: Some(format!(
            "Clustered design with {} pieces",
            clustered_pieces.len()
        )),
        unit: parent_dto.unit.clone(),
        kit: parent_dto.kit.clone(),
        pieces: clustered_pieces,
        connections: internal_connections,
        ..Default::default()
    };

    let mut added_connections: Vec<ConnectionFullDto> = Vec::new();
    for mut c in external_connections {
        let connected_in = cluster_set.contains(c.connected.piece.guid.as_str());
        let connecting_in = cluster_set.contains(c.connecting.piece.guid.as_str());
        if connected_in {
            c.connected.design_piece = Some(PieceIdDto {
                guid: new_guid.clone(),
            });
        } else if connecting_in {
            c.connecting.design_piece = Some(PieceIdDto {
                guid: new_guid.clone(),
            });
        }
        added_connections.push(c);
    }

    let removed_connections: Vec<ConnectionIdDto> = parent_dto
        .connections
        .iter()
        .filter(|c| {
            cluster_set.contains(c.connected.piece.guid.as_str())
                || cluster_set.contains(c.connecting.piece.guid.as_str())
        })
        .map(|c| ConnectionIdDto { guid: c.guid.clone() })
        .collect();

    let removed_pieces: Vec<PieceIdDto> = piece_guids
        .iter()
        .filter(|g| cluster_set.contains(g.as_str()))
        .map(|g| PieceIdDto {
            guid: Guid::from(g.as_str()),
        })
        .collect();

    let forward = DesignDiff {
        removed_pieces,
        removed_connections,
        added_connections,
        ..Default::default()
    };

    insert_design_ref(kit, clustered_dto)?;
    let mut g = kit
        .write()
        .map_err(|_| SetError::LockPoisoned("kit".into()))?;
    g.apply_design_diff(design_guid, &forward).map_err(map_semio)
}

fn normalize_coord(v: Coordinate) -> Coordinate {
    let n = v.length();
    if n < 1e-12 {
        Coordinate::ZERO
    } else {
        v.scale(1.0 / n)
    }
}

fn cross(a: Coordinate, b: Coordinate) -> Coordinate {
    Coordinate::new(
        a.y * b.z - a.z * b.y,
        a.z * b.x - a.x * b.z,
        a.x * b.y - a.y * b.x,
    )
}

fn move_translation_world(plane: &Plane, gap: f64, shift: f64, rise: f64) -> Coordinate {
    let x = normalize_coord(plane.x_axis);
    let y = normalize_coord(plane.y_axis);
    let z = cross(x, y);
    let zn = z.length();
    if zn < 1e-12 {
        return Coordinate::ZERO;
    }
    let z = z.scale(1.0 / zn);
    y.scale(gap).add(&x.scale(shift)).add(&z.scale(rise))
}

fn has_selected_ancestor_drag(
    piece: &str,
    selected: &HashSet<String>,
    parent_map: &HashMap<String, (String, String)>,
) -> bool {
    let mut current = piece.to_string();
    while let Some((_, parent)) = parent_map.get(&current) {
        if selected.contains(parent) {
            return true;
        }
        current = parent.clone();
    }
    false
}

/// Drag pieces in the diagram plane (fixed roots move center; connected children bump parent connection x/y).
pub fn drag_pieces_cmd(
    kit: &KitStoreRef,
    design_guid: &str,
    piece_guids: Vec<String>,
    du: f64,
    dv: f64,
) -> SetResult {
    if piece_guids.is_empty() {
        return Ok(());
    }
    let dto: DesignFullDto = {
        let g = kit
            .read()
            .map_err(|_| SetError::LockPoisoned("kit".into()))?;
        let d = g
            .design(design_guid)
            .ok_or_else(|| SetError::NotFound(format!("design {design_guid}")))?;
        let dr = d
            .read()
            .map_err(|_| SetError::LockPoisoned("design".into()))?;
        dr.to_full_dto()
    };

    let selected: HashSet<String> = piece_guids.into_iter().collect();
    let mut parent_map: HashMap<String, (String, String)> = HashMap::new();
    for c in &dto.connections {
        let child = c.connecting.piece.guid.to_string();
        let parent = c.connected.piece.guid.to_string();
        parent_map.insert(
            child,
            (c.guid.to_string(), parent),
        );
    }

    let fixed_guids: Vec<String> = selected
        .iter()
        .filter(|g| !parent_map.contains_key(*g))
        .cloned()
        .collect();

    let mut modified_pieces: Vec<PieceFullDto> = Vec::new();
    for g in &fixed_guids {
        if let Some(p) = dto.pieces.iter().find(|p| p.guid.as_str() == g) {
            let mut np = p.clone();
            if let Some(ce) = np.center.as_mut() {
                ce.x += du;
                ce.y += dv;
            }
            modified_pieces.push(np);
        }
    }

    let mut modified_connections: Vec<ConnectionFullDto> = Vec::new();
    for g in &selected {
        if fixed_guids.iter().any(|x| x == g) {
            continue;
        }
        if has_selected_ancestor_drag(g, &selected, &parent_map) {
            continue;
        }
        if let Some((conn_guid, _)) = parent_map.get(g) {
            if let Some(c) = dto.connections.iter().find(|c| c.guid.as_str() == conn_guid) {
                let mut nc = c.clone();
                nc.x = Some(nc.x.unwrap_or(0.0) + du);
                nc.y = Some(nc.y.unwrap_or(0.0) + dv);
                modified_connections.push(nc);
            }
        }
    }

    let mut diff = DesignDiff::default();
    if !modified_pieces.is_empty() {
        diff.modified_pieces = modified_pieces;
    }
    if !modified_connections.is_empty() {
        diff.modified_connections = modified_connections;
    }
    if diff.modified_pieces.is_empty() && diff.modified_connections.is_empty() {
        return Ok(());
    }
    let mut g = kit
        .write()
        .map_err(|_| SetError::LockPoisoned("kit".into()))?;
    g.apply_design_diff(design_guid, &diff).map_err(map_semio)
}

/// Structural move: translates fixed root pieces by moving their planes; skips connected pieces (Jacobian move not ported).
pub fn move_pieces_cmd(
    kit: &KitStoreRef,
    design_guid: &str,
    piece_guids: Vec<String>,
    gap: f64,
    shift: f64,
    rise: f64,
) -> SetResult {
    if piece_guids.is_empty() {
        return Ok(());
    }
    let dto: DesignFullDto = {
        let g = kit
            .read()
            .map_err(|_| SetError::LockPoisoned("kit".into()))?;
        let d = g
            .design(design_guid)
            .ok_or_else(|| SetError::NotFound(format!("design {design_guid}")))?;
        let dr = d
            .read()
            .map_err(|_| SetError::LockPoisoned("design".into()))?;
        dr.to_full_dto()
    };

    let selected: HashSet<String> = piece_guids.into_iter().collect();
    let mut parent_map: HashMap<String, (String, String)> = HashMap::new();
    for c in &dto.connections {
        parent_map.insert(
            c.connecting.piece.guid.to_string(),
            (c.guid.to_string(), c.connected.piece.guid.to_string()),
        );
    }

    let fixed_guids: Vec<String> = selected
        .iter()
        .filter(|g| !parent_map.contains_key(*g))
        .cloned()
        .collect();

    let mut modified_pieces: Vec<PieceFullDto> = Vec::new();
    for g in fixed_guids {
        if let Some(p) = dto.pieces.iter().find(|p| p.guid.as_str() == g) {
            let base = p.plane.unwrap_or_else(Plane::world_xy);
            let t = move_translation_world(&base, gap, shift, rise);
            let mut np = p.clone();
            let pl = np.plane.get_or_insert(base);
            pl.origin = pl.origin.add(&t);
            modified_pieces.push(np);
        }
    }

    if modified_pieces.is_empty() {
        return Ok(());
    }
    let diff = DesignDiff {
        modified_pieces,
        ..Default::default()
    };
    let mut g = kit
        .write()
        .map_err(|_| SetError::LockPoisoned("kit".into()))?;
    g.apply_design_diff(design_guid, &diff).map_err(map_semio)
}

/// Remove the parent connection for each piece (fix / ground in diagram).
pub fn fix_pieces_cmd(kit: &KitStoreRef, design_guid: &str, piece_ids: Vec<String>) -> SetResult {
    if piece_ids.is_empty() {
        return Ok(());
    }
    let dto: DesignFullDto = {
        let g = kit
            .read()
            .map_err(|_| SetError::LockPoisoned("kit".into()))?;
        let d = g
            .design(design_guid)
            .ok_or_else(|| SetError::NotFound(format!("design {design_guid}")))?;
        let dr = d
            .read()
            .map_err(|_| SetError::LockPoisoned("design".into()))?;
        dr.to_full_dto()
    };

    let mut removed: Vec<ConnectionIdDto> = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();
    for pid in &piece_ids {
        for c in &dto.connections {
            if c.connecting.piece.guid.as_str() == pid.as_str() {
                let s = c.guid.to_string();
                if seen.insert(s.clone()) {
                    removed.push(ConnectionIdDto { guid: Guid::from(s.as_str()) });
                }
            }
        }
    }
    if removed.is_empty() {
        return Ok(());
    }
    let diff = DesignDiff {
        removed_connections: removed,
        ..Default::default()
    };
    let mut g = kit
        .write()
        .map_err(|_| SetError::LockPoisoned("kit".into()))?;
    g.apply_design_diff(design_guid, &diff).map_err(map_semio)
}

pub fn delete_connection_cmd(kit: &KitStoreRef, design_guid: &str, connection_guid: &str) -> SetResult {
    let diff = DesignDiff {
        removed_connections: vec![ConnectionIdDto {
            guid: Guid::from(connection_guid),
        }],
        ..Default::default()
    };
    let mut g = kit
        .write()
        .map_err(|_| SetError::LockPoisoned("kit".into()))?;
    g.apply_design_diff(design_guid, &diff).map_err(map_semio)
}

/// Apply flatten forward diff (plane/center from flatten cache).
pub fn flatten_design_cmd(kit: &KitStoreRef, design_guid: &str) -> SetResult {
    let report = {
        let g = kit
            .read()
            .map_err(|_| SetError::LockPoisoned("kit".into()))?;
        g.flatten_design(design_guid).map_err(map_semio)?
    };
    if !report.ok {
        return Err(SetError::Internal("flatten_design failed".into()));
    }
    let Some(change) = report.value else {
        return Err(SetError::Internal("flatten_design missing value".into()));
    };
    let mut g = kit
        .write()
        .map_err(|_| SetError::LockPoisoned("kit".into()))?;
    g.apply_design_diff(design_guid, &change.forward)
        .map_err(map_semio)
}

fn design_dto_by_guid(kit: &KitStore, guid: &str) -> Option<DesignFullDto> {
    kit.design(guid).and_then(|d| d.read().ok().map(|r| r.to_full_dto()))
}

/// Expand nested designs referenced by `design_guid` (child design id) into `parent_design_guid`.
pub fn expand_design_cmd(
    kit: &KitStoreRef,
    parent_design_guid: &str,
    nested_design_guid: &str,
) -> SetResult {
    let before: DesignFullDto = {
        let g = kit
            .read()
            .map_err(|_| SetError::LockPoisoned("kit".into()))?;
        let d = g
            .design(parent_design_guid)
            .ok_or_else(|| SetError::NotFound(format!("design {parent_design_guid}")))?;
        let dr = d
            .read()
            .map_err(|_| SetError::LockPoisoned("design".into()))?;
        dr.to_full_dto()
    };

    let mut expanded_child: DesignFullDto = {
        let g = kit
            .read()
            .map_err(|_| SetError::LockPoisoned("kit".into()))?;
        let d = g
            .design(nested_design_guid)
            .ok_or_else(|| SetError::NotFound(format!("design {nested_design_guid}")))?;
        let dr = d
            .read()
            .map_err(|_| SetError::LockPoisoned("design".into()))?;
        dr.to_full_dto()
    };

    {
        let kg = kit
            .read()
            .map_err(|_| SetError::LockPoisoned("kit".into()))?;
        expanded_child = expand_nested_design_pieces_in_dto(&*kg, expanded_child)?;
    }

    let existing_piece: HashSet<String> = before
        .pieces
        .iter()
        .map(|p| p.guid.to_string())
        .collect();
    let add_pieces: Vec<PieceFullDto> = expanded_child
        .pieces
        .into_iter()
        .filter(|p| !existing_piece.contains(p.guid.as_str()))
        .collect();

    let existing_conn_key: HashSet<String> = before
        .connections
        .iter()
        .map(connection_key)
        .collect();
    let add_connections: Vec<ConnectionFullDto> = expanded_child
        .connections
        .into_iter()
        .filter(|c| !existing_conn_key.contains(&connection_key(c)))
        .collect();

    let mut after = before.clone();
    after.pieces.extend(add_pieces);
    let mut conns: Vec<ConnectionFullDto> = after
        .connections
        .iter()
        .map(|c| strip_design_piece_guid(c, nested_design_guid))
        .collect();
    conns.extend(add_connections);
    after.connections = conns;

    let diff = DesignDiff::between(&before, &after);
    let mut g = kit
        .write()
        .map_err(|_| SetError::LockPoisoned("kit".into()))?;
    g.apply_design_diff(parent_design_guid, &diff).map_err(map_semio)
}

fn connection_key(c: &ConnectionFullDto) -> String {
    format!(
        "{}|{}|{}|{}",
        c.guid,
        c.connected.piece.guid,
        c.connecting.piece.guid,
        c.connected.port.as_ref().map(|p| p.guid.to_string()).unwrap_or_default()
    )
}

fn strip_design_piece_guid(c: &ConnectionFullDto, nested: &str) -> ConnectionFullDto {
    let mut o = c.clone();
    if o.connected
        .design_piece
        .as_ref()
        .is_some_and(|d| d.guid.as_str() == nested)
    {
        o.connected.design_piece = None;
    }
    if o.connecting
        .design_piece
        .as_ref()
        .is_some_and(|d| d.guid.as_str() == nested)
    {
        o.connecting.design_piece = None;
    }
    o
}

fn expand_nested_design_pieces_in_dto(
    kit: &KitStore,
    mut design: DesignFullDto,
) -> Result<DesignFullDto, SetError> {
    let nested_ids: Vec<String> = {
        let mut s: HashSet<String> = HashSet::new();
        for c in &design.connections {
            if let Some(dp) = &c.connected.design_piece {
                s.insert(dp.guid.to_string());
            }
            if let Some(dp) = &c.connecting.design_piece {
                s.insert(dp.guid.to_string());
            }
        }
        s.into_iter().collect()
    };

    for nid in nested_ids {
        let Some(child) = design_dto_by_guid(kit, &nid) else {
            continue;
        };
        let expanded = expand_nested_design_pieces_in_dto(kit, child)?;
        let existing: HashSet<String> = design
            .pieces
            .iter()
            .map(|p| p.guid.to_string())
            .collect();
        let add_p: Vec<PieceFullDto> = expanded
            .pieces
            .into_iter()
            .filter(|p| !existing.contains(p.guid.as_str()))
            .collect();
        let keys: HashSet<String> = design.connections.iter().map(connection_key).collect();
        let add_c: Vec<ConnectionFullDto> = expanded
            .connections
            .into_iter()
            .filter(|c| !keys.contains(&connection_key(c)))
            .collect();
        design.pieces.extend(add_p);
        let mut new_conns: Vec<ConnectionFullDto> = design
            .connections
            .iter()
            .map(|c| strip_design_piece_guid(c, &nid))
            .collect();
        new_conns.extend(add_c);
        design.connections = new_conns;
    }
    Ok(design)
}

pub fn change_piece_type_cmd(
    kit: &KitStoreRef,
    design_guid: &str,
    piece_guid: &str,
    new_type_guid: &str,
) -> SetResult {
    let p: PieceFullDto = {
        let g = kit
            .read()
            .map_err(|_| SetError::LockPoisoned("kit".into()))?;
        let d = g
            .design(design_guid)
            .ok_or_else(|| SetError::NotFound(format!("design {design_guid}")))?;
        let dr = d
            .read()
            .map_err(|_| SetError::LockPoisoned("design".into()))?;
        let dto = dr.to_full_dto();
        dto
            .pieces
            .into_iter()
            .find(|p| p.guid.as_str() == piece_guid)
            .ok_or_else(|| SetError::NotFound(format!("piece {piece_guid}")))?
    };
    let mut np = p;
    np.r#type = Some(TypeIdDto {
        guid: Guid::from(new_type_guid),
    });
    let diff = DesignDiff {
        modified_pieces: vec![np],
        ..Default::default()
    };
    let mut g = kit
        .write()
        .map_err(|_| SetError::LockPoisoned("kit".into()))?;
    g.apply_design_diff(design_guid, &diff).map_err(map_semio)
}

/// Placeholder: full paste port pending clipboard DTO contract.
pub fn paste_design_selection_cmd(
    _kit: &KitStoreRef,
    _design_guid: &str,
    _selection_json: serde_json::Value,
    _plane: Option<Plane>,
) -> SetResult {
    Err(SetError::InvalidValue(
        "pasteDesignSelection: not yet implemented in Rust store".into(),
    ))
}

pub fn create_hanging_pieces_cmd(
    _kit: &KitStoreRef,
    _design_guid: &str,
    _type_guids: Vec<String>,
    _plane: Plane,
) -> SetResult {
    Err(SetError::InvalidValue(
        "createHangingPieces: not yet implemented in Rust store".into(),
    ))
}

pub fn create_connected_piece_cmd(
    _kit: &KitStoreRef,
    _design_guid: &str,
    _parent_piece: &str,
    _parent_port: &str,
    _child_type: &str,
    _child_port: &str,
) -> SetResult {
    Err(SetError::InvalidValue(
        "createConnectedPiece: not yet implemented in Rust store".into(),
    ))
}

pub fn create_fixed_piece_cmd(
    _kit: &KitStoreRef,
    _design_guid: &str,
    _type_guid: &str,
    _plane: Plane,
) -> SetResult {
    Err(SetError::InvalidValue(
        "createFixedPiece: not yet implemented in Rust store".into(),
    ))
}
