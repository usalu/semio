// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 semio contributors

use serde::{Deserialize, Serialize};
use super::Guid;
use super::model::*;

// #region Attribute Diff

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct AttributeDiff {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct AttributesDiff {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub added: Option<Vec<Attribute>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub removed: Option<Vec<AttributeId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<Vec<AttributeUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AttributeUpdate {
    pub attribute: AttributeId,
    pub diff: AttributeDiff,
}

// #endregion Attribute Diff

// #region Vector Diff

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct VectorDiff {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub z: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CoordDiff {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct PlaneDiff {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<VectorDiff>,
    #[serde(rename = "xAxis", skip_serializing_if = "Option::is_none")]
    pub x_axis: Option<VectorDiff>,
    #[serde(rename = "yAxis", skip_serializing_if = "Option::is_none")]
    pub y_axis: Option<VectorDiff>,
}

// #endregion Vector Diff

// #region Prop Diff

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct PropDiff {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<AttributesDiff>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct PropsDiff {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub added: Option<Vec<Prop>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub removed: Option<Vec<PropId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<Vec<PropUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PropUpdate {
    pub prop: PropId,
    pub diff: PropDiff,
}

// #endregion Prop Diff

// #region Model Diff

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ModelDiff {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<Guid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Guid>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<AttributesDiff>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ModelsDiff {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub added: Option<Vec<Model>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub removed: Option<Vec<ModelId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<Vec<ModelUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ModelUpdate {
    pub model: ModelId,
    pub diff: ModelDiff,
}

// #endregion Model Diff

// #region Connector Diff

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ConnectorDiff {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub point: Option<VectorDiff>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<VectorDiff>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandatory: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<InterfaceId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub props: Option<PropsDiff>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<AttributesDiff>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ConnectorsDiff {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub added: Option<Vec<Connector>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub removed: Option<Vec<ConnectorId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<Vec<ConnectorUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectorUpdate {
    pub connector: ConnectorId,
    pub diff: ConnectorDiff,
}

// #endregion Connector Diff

// #region Type Diff

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TypeDiff {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stock: Option<i32>,
    #[serde(rename = "isAbstract", skip_serializing_if = "Option::is_none")]
    pub is_abstract: Option<bool>,
    #[serde(rename = "virtual", skip_serializing_if = "Option::is_none")]
    pub virtual_type: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<LocationId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concepts: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authors: Option<Vec<AuthorId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub props: Option<PropsDiff>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub models: Option<ModelsDiff>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connectors: Option<ConnectorsDiff>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<AttributesDiff>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TypesDiff {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub added: Option<Vec<Type>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub removed: Option<Vec<TypeId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<Vec<TypeUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TypeUpdate {
    #[serde(rename = "type")]
    pub type_id: TypeId,
    pub diff: TypeDiff,
}

// #endregion Type Diff

// #region Piece Diff

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct PieceDiff {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_ref: Option<TypeId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub design: Option<DesignId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plane: Option<PlaneDiff>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub center: Option<CoordDiff>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<f64>,
    #[serde(rename = "mirrorPlane", skip_serializing_if = "Option::is_none")]
    pub mirror_plane: Option<PlaneDiff>,
    #[serde(rename = "isHidden", skip_serializing_if = "Option::is_none")]
    pub is_hidden: Option<bool>,
    #[serde(rename = "isLocked", skip_serializing_if = "Option::is_none")]
    pub is_locked: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub props: Option<PropsDiff>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<AttributesDiff>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct PiecesDiff {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub added: Option<Vec<Piece>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub removed: Option<Vec<PieceId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<Vec<PieceUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PieceUpdate {
    pub piece: PieceId,
    pub diff: PieceDiff,
}

// #endregion Piece Diff

// #region Connection Diff

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ConnectionDiff {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gap: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shift: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rise: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turn: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tilt: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<AttributesDiff>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ConnectionsDiff {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub added: Option<Vec<Connection>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub removed: Option<Vec<ConnectionId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<Vec<ConnectionUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectionUpdate {
    pub connection: ConnectionId,
    pub diff: ConnectionDiff,
}

// #endregion Connection Diff

// #region Design Diff

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct DesignDiff {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(rename = "isAbstract", skip_serializing_if = "Option::is_none")]
    pub is_abstract: Option<bool>,
    #[serde(rename = "canScale", skip_serializing_if = "Option::is_none")]
    pub can_scale: Option<bool>,
    #[serde(rename = "canMirror", skip_serializing_if = "Option::is_none")]
    pub can_mirror: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concepts: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pieces: Option<PiecesDiff>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connections: Option<ConnectionsDiff>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<AttributesDiff>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct DesignsDiff {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub added: Option<Vec<Design>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub removed: Option<Vec<DesignId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<Vec<DesignUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DesignUpdate {
    pub design: DesignId,
    pub diff: DesignDiff,
}

// #endregion Design Diff

// #region Interface Diff

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct InterfaceDiff {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(rename = "compatibleInterfaces", skip_serializing_if = "Option::is_none")]
    pub compatible_interfaces: Option<Vec<InterfaceId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<AttributesDiff>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct InterfacesDiff {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub added: Option<Vec<Interface>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub removed: Option<Vec<InterfaceId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<Vec<InterfaceUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InterfaceUpdate {
    pub port: InterfaceId,
    pub diff: InterfaceDiff,
}

// #endregion Interface Diff

// #region Quality Diff

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct QualityDiff {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<AttributesDiff>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct QualitiesDiff {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub added: Option<Vec<Quality>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub removed: Option<Vec<QualityId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<Vec<QualityUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QualityUpdate {
    pub quality: QualityId,
    pub diff: QualityDiff,
}

// #endregion Quality Diff

// #region File Diff

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct FileDiff {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct FilesDiff {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub added: Option<Vec<File>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub removed: Option<Vec<FileId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<Vec<FileUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FileUpdate {
    pub file: FileId,
    pub diff: FileDiff,
}

// #endregion File Diff

// #region Folder Diff

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct FolderDiff {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<AttributesDiff>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct FoldersDiff {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub added: Option<Vec<Folder>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub removed: Option<Vec<FolderId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<Vec<FolderUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FolderUpdate {
    pub folder: FolderId,
    pub diff: FolderDiff,
}

// #endregion Folder Diff

// #region Author Diff

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct AuthorDiff {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<AttributesDiff>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct AuthorsDiff {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub added: Option<Vec<Author>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub removed: Option<Vec<AuthorId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<Vec<AuthorUpdate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AuthorUpdate {
    pub author: AuthorId,
    pub diff: AuthorDiff,
}

// #endregion Author Diff

// #region Kit Diff

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct KitDiff {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concepts: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<TypesDiff>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub designs: Option<DesignsDiff>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ports: Option<InterfacesDiff>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualities: Option<QualitiesDiff>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<FilesDiff>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folders: Option<FoldersDiff>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authors: Option<AuthorsDiff>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<AttributesDiff>,
}

// #endregion Kit Diff

// #region Diff Functions

fn is_empty_diff<T>(diff: &Option<T>) -> bool {
    diff.is_none()
}

pub fn get_attribute_diff(before: &Attribute, after: &Attribute) -> AttributeDiff {
    let mut diff = AttributeDiff::default();
    if before.key != after.key {
        diff.key = Some(after.key.clone());
    }
    if before.value != after.value {
        diff.value = after.value.clone();
    }
    if before.definition != after.definition {
        diff.definition = after.definition.clone();
    }
    diff
}

pub fn apply_attribute_diff(attr: &mut Attribute, diff: &AttributeDiff) {
    if let Some(key) = &diff.key {
        attr.key = key.clone();
    }
    if let Some(value) = &diff.value {
        attr.value = Some(value.clone());
    }
    if let Some(definition) = &diff.definition {
        attr.definition = Some(definition.clone());
    }
}

pub fn get_attributes_diff(before: &Option<Vec<Attribute>>, after: &Option<Vec<Attribute>>) -> AttributesDiff {
    let before_attrs = before.clone().unwrap_or_default();
    let after_attrs = after.clone().unwrap_or_default();
    
    let before_map: std::collections::HashMap<_, _> = before_attrs.iter().map(|a| (a.guid.clone(), a)).collect();
    let after_map: std::collections::HashMap<_, _> = after_attrs.iter().map(|a| (a.guid.clone(), a)).collect();
    
    let mut added = Vec::new();
    let mut removed = Vec::new();
    let mut updated = Vec::new();
    
    for attr in &after_attrs {
        if !before_map.contains_key(&attr.guid) {
            added.push(attr.clone());
        }
    }
    
    for attr in &before_attrs {
        if !after_map.contains_key(&attr.guid) {
            removed.push(AttributeId { guid: attr.guid.clone() });
        }
    }
    
    for attr in &after_attrs {
        if let Some(before_attr) = before_map.get(&attr.guid) {
            let diff = get_attribute_diff(before_attr, attr);
            if diff.key.is_some() || diff.value.is_some() || diff.definition.is_some() {
                updated.push(AttributeUpdate {
                    attribute: AttributeId { guid: attr.guid.clone() },
                    diff,
                });
            }
        }
    }
    
    AttributesDiff {
        added: if added.is_empty() { None } else { Some(added) },
        removed: if removed.is_empty() { None } else { Some(removed) },
        updated: if updated.is_empty() { None } else { Some(updated) },
    }
}

pub fn apply_attributes_diff(attrs: &mut Option<Vec<Attribute>>, diff: &AttributesDiff) {
    let mut current = attrs.take().unwrap_or_default();
    
    if let Some(removed) = &diff.removed {
        let removed_guids: std::collections::HashSet<_> = removed.iter().map(|r| &r.guid).collect();
        current.retain(|a| !removed_guids.contains(&a.guid));
    }
    
    if let Some(updated) = &diff.updated {
        for update in updated {
            if let Some(attr) = current.iter_mut().find(|a| a.guid == update.attribute.guid) {
                apply_attribute_diff(attr, &update.diff);
            }
        }
    }
    
    if let Some(added) = &diff.added {
        current.extend(added.clone());
    }
    
    *attrs = if current.is_empty() { None } else { Some(current) };
}

pub fn get_piece_diff(before: &Piece, after: &Piece) -> PieceDiff {
    let mut diff = PieceDiff::default();
    
    if before.name != after.name {
        diff.name = after.name.clone();
    }
    if before.type_ref != after.type_ref {
        diff.type_ref = after.type_ref.clone();
    }
    if before.design != after.design {
        diff.design = after.design.clone();
    }
    if before.scale != after.scale {
        diff.scale = after.scale;
    }
    if before.is_hidden != after.is_hidden {
        diff.is_hidden = after.is_hidden;
    }
    if before.is_locked != after.is_locked {
        diff.is_locked = after.is_locked;
    }
    if before.color != after.color {
        diff.color = after.color.clone();
    }
    if before.description != after.description {
        diff.description = after.description.clone();
    }
    
    let attrs_diff = get_attributes_diff(&before.attributes, &after.attributes);
    if attrs_diff.added.is_some() || attrs_diff.removed.is_some() || attrs_diff.updated.is_some() {
        diff.attributes = Some(attrs_diff);
    }
    
    diff
}

pub fn apply_piece_diff(piece: &mut Piece, diff: &PieceDiff) {
    if let Some(name) = &diff.name {
        piece.name = Some(name.clone());
    }
    if let Some(type_ref) = &diff.type_ref {
        piece.type_ref = Some(type_ref.clone());
    }
    if let Some(design) = &diff.design {
        piece.design = Some(design.clone());
    }
    if let Some(scale) = diff.scale {
        piece.scale = Some(scale);
    }
    if let Some(is_hidden) = diff.is_hidden {
        piece.is_hidden = Some(is_hidden);
    }
    if let Some(is_locked) = diff.is_locked {
        piece.is_locked = Some(is_locked);
    }
    if let Some(color) = &diff.color {
        piece.color = Some(color.clone());
    }
    if let Some(description) = &diff.description {
        piece.description = Some(description.clone());
    }
    if let Some(attrs_diff) = &diff.attributes {
        apply_attributes_diff(&mut piece.attributes, attrs_diff);
    }
}

pub fn get_connection_diff(before: &Connection, after: &Connection) -> ConnectionDiff {
    let mut diff = ConnectionDiff::default();
    
    if before.gap != after.gap {
        diff.gap = Some(after.gap);
    }
    if before.shift != after.shift {
        diff.shift = Some(after.shift);
    }
    if before.rise != after.rise {
        diff.rise = Some(after.rise);
    }
    if before.rotation != after.rotation {
        diff.rotation = Some(after.rotation);
    }
    if before.turn != after.turn {
        diff.turn = Some(after.turn);
    }
    if before.tilt != after.tilt {
        diff.tilt = Some(after.tilt);
    }
    if before.u != after.u {
        diff.u = after.u;
    }
    if before.v != after.v {
        diff.v = after.v;
    }
    if before.description != after.description {
        diff.description = after.description.clone();
    }
    
    let attrs_diff = get_attributes_diff(&before.attributes, &after.attributes);
    if attrs_diff.added.is_some() || attrs_diff.removed.is_some() || attrs_diff.updated.is_some() {
        diff.attributes = Some(attrs_diff);
    }
    
    diff
}

pub fn apply_connection_diff(conn: &mut Connection, diff: &ConnectionDiff) {
    if let Some(gap) = diff.gap {
        conn.gap = gap;
    }
    if let Some(shift) = diff.shift {
        conn.shift = shift;
    }
    if let Some(rise) = diff.rise {
        conn.rise = rise;
    }
    if let Some(rotation) = diff.rotation {
        conn.rotation = rotation;
    }
    if let Some(turn) = diff.turn {
        conn.turn = turn;
    }
    if let Some(tilt) = diff.tilt {
        conn.tilt = tilt;
    }
    if let Some(u) = diff.u {
        conn.u = Some(u);
    }
    if let Some(v) = diff.v {
        conn.v = Some(v);
    }
    if let Some(description) = &diff.description {
        conn.description = Some(description.clone());
    }
    if let Some(attrs_diff) = &diff.attributes {
        apply_attributes_diff(&mut conn.attributes, attrs_diff);
    }
}

pub fn get_design_diff(before: &Design, after: &Design) -> DesignDiff {
    let mut diff = DesignDiff::default();
    
    if before.name != after.name {
        diff.name = Some(after.name.clone());
    }
    if before.description != after.description {
        diff.description = after.description.clone();
    }
    if before.icon != after.icon {
        diff.icon = after.icon.clone();
    }
    if before.image != after.image {
        diff.image = after.image.clone();
    }
    if before.concepts != after.concepts {
        diff.concepts = after.concepts.clone();
    }
    
    let attrs_diff = get_attributes_diff(&before.attributes, &after.attributes);
    if attrs_diff.added.is_some() || attrs_diff.removed.is_some() || attrs_diff.updated.is_some() {
        diff.attributes = Some(attrs_diff);
    }
    
    diff
}

pub fn apply_design_diff(design: &mut Design, diff: &DesignDiff) {
    if let Some(name) = &diff.name {
        design.name = name.clone();
    }
    if let Some(description) = &diff.description {
        design.description = Some(description.clone());
    }
    if let Some(icon) = &diff.icon {
        design.icon = Some(icon.clone());
    }
    if let Some(image) = &diff.image {
        design.image = Some(image.clone());
    }
    if let Some(concepts) = &diff.concepts {
        design.concepts = Some(concepts.clone());
    }
    
    if let Some(pieces_diff) = &diff.pieces {
        apply_pieces_diff(&mut design.pieces, pieces_diff);
    }
    if let Some(conns_diff) = &diff.connections {
        apply_connections_diff(&mut design.connections, conns_diff);
    }
    if let Some(attrs_diff) = &diff.attributes {
        apply_attributes_diff(&mut design.attributes, attrs_diff);
    }
}

pub fn apply_pieces_diff(pieces: &mut Option<Vec<Piece>>, diff: &PiecesDiff) {
    let mut current = pieces.take().unwrap_or_default();
    
    if let Some(removed) = &diff.removed {
        let removed_guids: std::collections::HashSet<_> = removed.iter().map(|r| &r.guid).collect();
        current.retain(|p| !removed_guids.contains(&p.guid));
    }
    
    if let Some(updated) = &diff.updated {
        for update in updated {
            if let Some(piece) = current.iter_mut().find(|p| p.guid == update.piece.guid) {
                apply_piece_diff(piece, &update.diff);
            }
        }
    }
    
    if let Some(added) = &diff.added {
        current.extend(added.clone());
    }
    
    *pieces = if current.is_empty() { None } else { Some(current) };
}

pub fn apply_connections_diff(conns: &mut Option<Vec<Connection>>, diff: &ConnectionsDiff) {
    let mut current = conns.take().unwrap_or_default();
    
    if let Some(removed) = &diff.removed {
        let removed_guids: std::collections::HashSet<_> = removed.iter().map(|r| &r.guid).collect();
        current.retain(|c| !removed_guids.contains(&c.guid));
    }
    
    if let Some(updated) = &diff.updated {
        for update in updated {
            if let Some(conn) = current.iter_mut().find(|c| c.guid == update.connection.guid) {
                apply_connection_diff(conn, &update.diff);
            }
        }
    }
    
    if let Some(added) = &diff.added {
        current.extend(added.clone());
    }
    
    *conns = if current.is_empty() { None } else { Some(current) };
}

pub fn get_kit_diff(before: &Kit, after: &Kit) -> KitDiff {
    let mut diff = KitDiff::default();
    
    if before.name != after.name {
        diff.name = Some(after.name.clone());
    }
    if before.version != after.version {
        diff.version = after.version.clone();
    }
    if before.description != after.description {
        diff.description = after.description.clone();
    }
    if before.icon != after.icon {
        diff.icon = after.icon.clone();
    }
    if before.image != after.image {
        diff.image = after.image.clone();
    }
    if before.preview != after.preview {
        diff.preview = after.preview.clone();
    }
    if before.remote != after.remote {
        diff.remote = after.remote.clone();
    }
    if before.homepage != after.homepage {
        diff.homepage = after.homepage.clone();
    }
    if before.license != after.license {
        diff.license = after.license.clone();
    }
    
    let attrs_diff = get_attributes_diff(&before.attributes, &after.attributes);
    if attrs_diff.added.is_some() || attrs_diff.removed.is_some() || attrs_diff.updated.is_some() {
        diff.attributes = Some(attrs_diff);
    }
    
    diff
}

pub fn apply_kit_diff(kit: &mut Kit, diff: &KitDiff) {
    if let Some(name) = &diff.name {
        kit.name = name.clone();
    }
    if let Some(version) = &diff.version {
        kit.version = Some(version.clone());
    }
    if let Some(description) = &diff.description {
        kit.description = Some(description.clone());
    }
    if let Some(icon) = &diff.icon {
        kit.icon = Some(icon.clone());
    }
    if let Some(image) = &diff.image {
        kit.image = Some(image.clone());
    }
    if let Some(preview) = &diff.preview {
        kit.preview = Some(preview.clone());
    }
    if let Some(remote) = &diff.remote {
        kit.remote = Some(remote.clone());
    }
    if let Some(homepage) = &diff.homepage {
        kit.homepage = Some(homepage.clone());
    }
    if let Some(license) = &diff.license {
        kit.license = Some(license.clone());
    }
    if let Some(attrs_diff) = &diff.attributes {
        apply_attributes_diff(&mut kit.attributes, attrs_diff);
    }
}

// #endregion Diff Functions
