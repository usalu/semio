// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 semio contributors

use super::model::*;
use super::{Result, SemioError};

// #region Serialization

pub fn serialize_kit(kit: &Kit) -> Result<String> {
    serde_json::to_string(kit).map_err(|e| SemioError::Serialization {
        message: e.to_string(),
    })
}

pub fn serialize_kit_pretty(kit: &Kit) -> Result<String> {
    serde_json::to_string_pretty(kit).map_err(|e| SemioError::Serialization {
        message: e.to_string(),
    })
}

pub fn deserialize_kit(json: &str) -> Result<Kit> {
    serde_json::from_str(json).map_err(|e| SemioError::Serialization {
        message: e.to_string(),
    })
}

pub fn serialize_design(design: &Design) -> Result<String> {
    serde_json::to_string(design).map_err(|e| SemioError::Serialization {
        message: e.to_string(),
    })
}

pub fn deserialize_design(json: &str) -> Result<Design> {
    serde_json::from_str(json).map_err(|e| SemioError::Serialization {
        message: e.to_string(),
    })
}

pub fn serialize_type(type_: &Type) -> Result<String> {
    serde_json::to_string(type_).map_err(|e| SemioError::Serialization {
        message: e.to_string(),
    })
}

pub fn deserialize_type(json: &str) -> Result<Type> {
    serde_json::from_str(json).map_err(|e| SemioError::Serialization {
        message: e.to_string(),
    })
}

// #endregion Serialization

// #region Kit Equality

pub fn are_kits_equal(a: &Kit, b: &Kit) -> bool {
    if a.guid != b.guid || a.name != b.name {
        return false;
    }
    if a.version != b.version || a.description != b.description {
        return false;
    }
    if a.icon != b.icon || a.image != b.image {
        return false;
    }
    if a.preview != b.preview || a.remote != b.remote {
        return false;
    }
    if a.homepage != b.homepage || a.license != b.license {
        return false;
    }

    if !are_types_equal(&a.types, &b.types) {
        return false;
    }
    if !are_designs_equal(&a.designs, &b.designs) {
        return false;
    }

    true
}

fn are_types_equal(a: &Option<Vec<Type>>, b: &Option<Vec<Type>>) -> bool {
    match (a, b) {
        (None, None) => true,
        (Some(a), Some(b)) => {
            if a.len() != b.len() {
                return false;
            }
            for type_a in a {
                let type_b = b.iter().find(|t| t.guid == type_a.guid);
                if type_b.is_none() {
                    return false;
                }
                let type_b = type_b.unwrap();
                if type_a.name != type_b.name {
                    return false;
                }
            }
            true
        }
        _ => false,
    }
}

fn are_designs_equal(a: &Option<Vec<Design>>, b: &Option<Vec<Design>>) -> bool {
    match (a, b) {
        (None, None) => true,
        (Some(a), Some(b)) => {
            if a.len() != b.len() {
                return false;
            }
            for design_a in a {
                let design_b = b.iter().find(|d| d.guid == design_a.guid);
                if design_b.is_none() {
                    return false;
                }
                let design_b = design_b.unwrap();
                if design_a.name != design_b.name {
                    return false;
                }
                if !are_pieces_equal(&design_a.pieces, &design_b.pieces) {
                    return false;
                }
                if !are_connections_equal(&design_a.connections, &design_b.connections) {
                    return false;
                }
            }
            true
        }
        _ => false,
    }
}

fn are_pieces_equal(a: &Option<Vec<Piece>>, b: &Option<Vec<Piece>>) -> bool {
    match (a, b) {
        (None, None) => true,
        (Some(a), Some(b)) => {
            if a.len() != b.len() {
                return false;
            }
            for piece_a in a {
                let piece_b = b.iter().find(|p| p.guid == piece_a.guid);
                if piece_b.is_none() {
                    return false;
                }
            }
            true
        }
        _ => false,
    }
}

fn are_connections_equal(a: &Option<Vec<Connection>>, b: &Option<Vec<Connection>>) -> bool {
    match (a, b) {
        (None, None) => true,
        (Some(a), Some(b)) => {
            if a.len() != b.len() {
                return false;
            }
            for conn_a in a {
                let conn_b = b.iter().find(|c| c.guid == conn_a.guid);
                if conn_b.is_none() {
                    return false;
                }
            }
            true
        }
        _ => false,
    }
}

// #endregion Kit Equality

// #region Supported Model Extensions

const SUPPORTED_MODEL_EXTENSIONS: &[&str] = &[
    "gltf", "glb", "fbx", "obj", "dae", "3ds", "stl", "ply", "usdz", "vrm", "ifc", "3mf",
];

pub fn is_supported_model_extension(ext: &str) -> bool {
    let ext_lower = ext.to_lowercase();
    let ext_clean = ext_lower.trim_start_matches('.');
    SUPPORTED_MODEL_EXTENSIONS.contains(&ext_clean)
}

pub fn validate_model_file(filename: &str) -> bool {
    if let Some(ext) = filename.rsplit('.').next() {
        is_supported_model_extension(ext)
    } else {
        false
    }
}

// #endregion Supported Model Extensions
