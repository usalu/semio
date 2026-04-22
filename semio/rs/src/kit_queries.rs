//! Read-only query helpers for the worker / React hooks (flatten-derived data, collections).

use std::collections::{HashMap, HashSet, VecDeque};

use serde::Serialize;

use crate::error::SetError;
use crate::geom::{Coordinate, Plane};
use crate::guid::Guid;
use crate::kit::KitStoreRef;

#[derive(Clone, Debug, Serialize)]
pub struct PiecePlacementMetadataJson {
    pub plane: Plane,
    pub center: Coordinate,
    #[serde(rename = "fixedPieceId")]
    pub fixed_piece_id: String,
    #[serde(rename = "parentPieceId")]
    pub parent_piece_id: Option<String>,
    pub depth: i32,
    pub path: Vec<String>,
}

/// Pieces metadata map keyed by piece guid (flatten placement + simple tree hints).
pub fn get_pieces_metadata_json(
    kit: &KitStoreRef,
    design_guid: &str,
) -> Result<serde_json::Value, SetError> {
    let g = kit
        .read()
        .map_err(|_| SetError::LockPoisoned("kit".into()))?;
    let d = g
        .design(design_guid)
        .ok_or_else(|| SetError::NotFound(format!("design {design_guid}")))?;
    let dr = d
        .read()
        .map_err(|_| SetError::LockPoisoned("design".into()))?;
    let flat = dr.flatten_map();

    let mut parent_of: HashMap<Guid, Guid> = HashMap::new();
    let mut is_child: HashSet<Guid> = HashSet::new();
    for c in &dr.connections {
        let Ok(conn) = c.read() else { continue };
        let Ok(s0) = conn.connected.read() else { continue };
        let Ok(s1) = conn.connecting.read() else { continue };
        let Some(pg0) = s0.piece.upgrade().and_then(|p| p.read().ok().map(|r| r.guid.clone())) else { continue };
        let Some(pg1) = s1.piece.upgrade().and_then(|p| p.read().ok().map(|r| r.guid.clone())) else { continue };
        parent_of.insert(pg1.clone(), pg0);
        is_child.insert(pg1);
    }

    let mut depth: HashMap<Guid, i32> = HashMap::new();
    let mut roots: Vec<Guid> = Vec::new();
    for p in &dr.pieces {
        if let Ok(pr) = p.read() {
            if !is_child.contains(&pr.guid) {
                roots.push(pr.guid.clone());
            }
        }
    }
    let mut q: VecDeque<(Guid, i32)> = VecDeque::new();
    let mut seen_d: HashSet<Guid> = HashSet::new();
    for r in roots {
        q.push_back((r, 0));
    }
    while let Some((gid, dpt)) = q.pop_front() {
        if !seen_d.insert(gid.clone()) {
            continue;
        }
        depth.insert(gid.clone(), dpt);
        for p in &dr.pieces {
            if let Ok(pr) = p.read() {
                if parent_of.get(&pr.guid) == Some(&gid) {
                    q.push_back((pr.guid.clone(), dpt + 1));
                }
            }
        }
    }

    let mut out: HashMap<String, PiecePlacementMetadataJson> = HashMap::new();
    for p in &dr.pieces {
        if let Ok(pr) = p.read() {
            let guid_s = pr.guid.to_string();
            let (plane, center) = flat
                .get(&pr.guid)
                .cloned()
                .unwrap_or_else(|| (Plane::world_xy(), Coordinate::ZERO));
            let parent_piece_id = parent_of.get(&pr.guid).map(|g| g.to_string());
            let dpt = *depth.get(&pr.guid).unwrap_or(&0);
            out.insert(
                guid_s.clone(),
                PiecePlacementMetadataJson {
                    plane,
                    center,
                    fixed_piece_id: guid_s.clone(),
                    parent_piece_id,
                    depth: dpt,
                    path: vec![guid_s],
                },
            );
        }
    }
    serde_json::to_value(&out).map_err(|e| SetError::InvalidValue(e.to_string()))
}

pub fn get_kit_json(kit: &KitStoreRef) -> Result<serde_json::Value, SetError> {
    let g = kit
        .read()
        .map_err(|_| SetError::LockPoisoned("kit".into()))?;
    serde_json::to_value(g.to_metadata_dto()).map_err(|e| SetError::InvalidValue(e.to_string()))
}

pub fn get_designs_json(kit: &KitStoreRef) -> Result<serde_json::Value, SetError> {
    let g = kit
        .read()
        .map_err(|_| SetError::LockPoisoned("kit".into()))?;
    let v: Vec<_> = g
        .designs
        .iter()
        .filter_map(|d| d.read().ok().map(|r| r.to_shallow_dto()))
        .collect();
    serde_json::to_value(&v).map_err(|e| SetError::InvalidValue(e.to_string()))
}

pub fn get_types_json(kit: &KitStoreRef) -> Result<serde_json::Value, SetError> {
    let g = kit
        .read()
        .map_err(|_| SetError::LockPoisoned("kit".into()))?;
    let v: Vec<_> = g
        .types
        .iter()
        .filter_map(|t| t.read().ok().map(|r| r.to_shallow_dto()))
        .collect();
    serde_json::to_value(&v).map_err(|e| SetError::InvalidValue(e.to_string()))
}

pub fn get_authors_json(kit: &KitStoreRef) -> Result<serde_json::Value, SetError> {
    let g = kit
        .read()
        .map_err(|_| SetError::LockPoisoned("kit".into()))?;
    let v: Vec<_> = g
        .authors
        .iter()
        .filter_map(|a| a.read().ok().map(|r| r.to_shallow_dto()))
        .collect();
    serde_json::to_value(&v).map_err(|e| SetError::InvalidValue(e.to_string()))
}

pub fn get_pieces_json(kit: &KitStoreRef, design_guid: &str) -> Result<serde_json::Value, SetError> {
    let g = kit
        .read()
        .map_err(|_| SetError::LockPoisoned("kit".into()))?;
    let d = g
        .design(design_guid)
        .ok_or_else(|| SetError::NotFound(format!("design {design_guid}")))?;
    let dr = d
        .read()
        .map_err(|_| SetError::LockPoisoned("design".into()))?;
    let v: Vec<_> = dr
        .pieces
        .iter()
        .filter_map(|p| p.read().ok().map(|r| r.to_full_dto()))
        .collect();
    serde_json::to_value(&v).map_err(|e| SetError::InvalidValue(e.to_string()))
}

pub fn get_connections_json(
    kit: &KitStoreRef,
    design_guid: &str,
) -> Result<serde_json::Value, SetError> {
    let g = kit
        .read()
        .map_err(|_| SetError::LockPoisoned("kit".into()))?;
    let d = g
        .design(design_guid)
        .ok_or_else(|| SetError::NotFound(format!("design {design_guid}")))?;
    let dr = d
        .read()
        .map_err(|_| SetError::LockPoisoned("design".into()))?;
    let v: Vec<_> = dr
        .connections
        .iter()
        .filter_map(|c| c.read().ok().map(|r| r.to_full_dto()))
        .collect();
    serde_json::to_value(&v).map_err(|e| SetError::InvalidValue(e.to_string()))
}
