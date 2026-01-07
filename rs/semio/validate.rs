// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 semio contributors

use std::collections::HashMap;
use super::Guid;
use super::model::*;
use super::diff::*;

// #region Validation Core Types

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EntityKind {
    Kit,
    Type,
    Design,
    Piece,
    Connection,
    Connector,
    Attribute,
    File,
    Folder,
    Quality,
    Interface,
    Prop,
    Model,
    Layer,
    Group,
    Stat,
}

impl std::fmt::Display for EntityKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EntityKind::Kit => write!(f, "Kit"),
            EntityKind::Type => write!(f, "Type"),
            EntityKind::Design => write!(f, "Design"),
            EntityKind::Piece => write!(f, "Piece"),
            EntityKind::Connection => write!(f, "Connection"),
            EntityKind::Connector => write!(f, "Connector"),
            EntityKind::Attribute => write!(f, "Attribute"),
            EntityKind::File => write!(f, "File"),
            EntityKind::Folder => write!(f, "Folder"),
            EntityKind::Quality => write!(f, "Quality"),
            EntityKind::Interface => write!(f, "Interface"),
            EntityKind::Prop => write!(f, "Prop"),
            EntityKind::Model => write!(f, "Model"),
            EntityKind::Layer => write!(f, "Layer"),
            EntityKind::Group => write!(f, "Group"),
            EntityKind::Stat => write!(f, "Stat"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DomainLocation {
    pub entity_kind: EntityKind,
    pub entity_guid: Option<Guid>,
    pub field: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Fix {
    pub title: String,
    pub diff: KitDiff,
}

#[derive(Debug, Clone)]
pub struct Problem {
    pub constraint_id: String,
    pub message: String,
    pub location: DomainLocation,
    pub related_guids: Vec<Guid>,
    pub fixes: Vec<Fix>,
}

#[derive(Debug, Clone, Default)]
pub struct ValidationResult {
    pub problems: Vec<Problem>,
}

impl ValidationResult {
    pub fn has_errors(&self) -> bool {
        !self.problems.is_empty()
    }
}

// #endregion Validation Core Types

// #region Validation Context

pub struct ValidationContext<'a> {
    pub kit: &'a Kit,
    pub types_by_guid: HashMap<Guid, &'a Type>,
    pub designs_by_guid: HashMap<Guid, &'a Design>,
    pub pieces_by_guid: HashMap<Guid, (Guid, &'a Piece)>,
    pub connectors_by_type_guid: HashMap<Guid, Vec<&'a Connector>>,
    pub models_by_type_guid: HashMap<Guid, Vec<&'a Model>>,
}

impl<'a> ValidationContext<'a> {
    pub fn new(kit: &'a Kit) -> Self {
        let mut types_by_guid = HashMap::new();
        let mut designs_by_guid = HashMap::new();
        let mut pieces_by_guid = HashMap::new();
        let mut connectors_by_type_guid = HashMap::new();
        let mut models_by_type_guid = HashMap::new();

        if let Some(types) = &kit.types {
            for t in types {
                types_by_guid.insert(t.guid.clone(), t);
                if let Some(connectors) = &t.connectors {
                    connectors_by_type_guid.insert(t.guid.clone(), connectors.iter().collect());
                }
                if let Some(models) = &t.models {
                    models_by_type_guid.insert(t.guid.clone(), models.iter().collect());
                }
            }
        }

        if let Some(designs) = &kit.designs {
            for d in designs {
                designs_by_guid.insert(d.guid.clone(), d);
                if let Some(pieces) = &d.pieces {
                    for p in pieces {
                        pieces_by_guid.insert(p.guid.clone(), (d.guid.clone(), p));
                    }
                }
            }
        }

        Self {
            kit,
            types_by_guid,
            designs_by_guid,
            pieces_by_guid,
            connectors_by_type_guid,
            models_by_type_guid,
        }
    }
}

// #endregion Validation Context

// #region Constraint Type

pub type Constraint = fn(&ValidationContext) -> Vec<Problem>;

// #endregion Constraint Type

// #region Validation Engine

pub struct ValidationConfig {
    pub constraints: Vec<Constraint>,
}

impl Default for ValidationConfig {
    fn default() -> Self {
        Self {
            constraints: vec![
                guid_uniqueness_constraint,
                type_name_uniqueness_constraint,
                design_name_uniqueness_constraint,
                piece_name_uniqueness_constraint,
            ],
        }
    }
}

pub fn validate_kit(kit: &Kit, config: Option<ValidationConfig>) -> ValidationResult {
    let ctx = ValidationContext::new(kit);
    let config = config.unwrap_or_default();
    
    let problems: Vec<Problem> = config
        .constraints
        .iter()
        .flat_map(|constraint| constraint(&ctx))
        .collect();
    
    ValidationResult { problems }
}

// #endregion Validation Engine

// #region Fix Helper

pub fn make_fix<F>(ctx: &ValidationContext, title: &str, mutate: F) -> Fix
where
    F: FnOnce(&mut Kit),
{
    let mut clone: Kit = serde_json::from_str(&serde_json::to_string(ctx.kit).unwrap()).unwrap();
    mutate(&mut clone);
    let diff = get_kit_diff(ctx.kit, &clone);
    Fix {
        title: title.to_string(),
        diff,
    }
}

// #endregion Fix Helper

// #region Constraint: GUID Uniqueness

pub fn guid_uniqueness_constraint(ctx: &ValidationContext) -> Vec<Problem> {
    let mut problems = Vec::new();
    let mut seen: HashMap<Guid, EntityKind> = HashMap::new();

    let mut check = |kind: EntityKind, guid: &Guid| {
        if let Some(_existing) = seen.get(guid) {
            problems.push(Problem {
                constraint_id: "guid-unique".to_string(),
                message: format!("Duplicate GUID \"{}\". First occurrence kept.", guid),
                location: DomainLocation {
                    entity_kind: kind,
                    entity_guid: Some(guid.clone()),
                    field: Some("guid".to_string()),
                },
                related_guids: vec![guid.clone()],
                fixes: vec![],
            });
        } else {
            seen.insert(guid.clone(), kind);
        }
    };

    check(EntityKind::Kit, &ctx.kit.guid);

    if let Some(types) = &ctx.kit.types {
        for t in types {
            check(EntityKind::Type, &t.guid);
        }
    }

    if let Some(designs) = &ctx.kit.designs {
        for d in designs {
            check(EntityKind::Design, &d.guid);
            if let Some(pieces) = &d.pieces {
                for p in pieces {
                    check(EntityKind::Piece, &p.guid);
                }
            }
            if let Some(connections) = &d.connections {
                for c in connections {
                    check(EntityKind::Connection, &c.guid);
                }
            }
            if let Some(stats) = &d.stats {
                for s in stats {
                    check(EntityKind::Stat, &s.guid);
                }
            }
        }
    }

    if let Some(qualities) = &ctx.kit.qualities {
        for q in qualities {
            check(EntityKind::Quality, &q.guid);
        }
    }

    if let Some(ports) = &ctx.kit.ports {
        for p in ports {
            check(EntityKind::Interface, &p.guid);
        }
    }

    if let Some(files) = &ctx.kit.files {
        for f in files {
            check(EntityKind::File, &f.guid);
        }
    }

    if let Some(folders) = &ctx.kit.folders {
        for f in folders {
            check(EntityKind::Folder, &f.guid);
        }
    }

    problems
}

// #endregion Constraint: GUID Uniqueness

// #region Constraint: Type Name Uniqueness

pub fn type_name_uniqueness_constraint(ctx: &ValidationContext) -> Vec<Problem> {
    let mut problems = Vec::new();

    if let Some(types) = &ctx.kit.types {
        let mut by_parent: HashMap<Option<Guid>, Vec<&Type>> = HashMap::new();
        
        for t in types {
            let parent_guid = t.parent.as_ref().map(|p| p.guid.clone());
            by_parent.entry(parent_guid).or_default().push(t);
        }

        for (_parent_guid, siblings) in by_parent {
            let mut names: HashMap<String, Vec<&Type>> = HashMap::new();
            
            for t in &siblings {
                names.entry(t.name.clone()).or_default().push(t);
            }

            for (name, group) in names {
                if group.len() <= 1 {
                    continue;
                }

                for t in group.iter().skip(1) {
                    problems.push(Problem {
                        constraint_id: "type-name-unique".to_string(),
                        message: format!("Duplicate type name \"{}\" among siblings.", name),
                        location: DomainLocation {
                            entity_kind: EntityKind::Type,
                            entity_guid: Some(t.guid.clone()),
                            field: Some("name".to_string()),
                        },
                        related_guids: group.iter().map(|x| x.guid.clone()).collect(),
                        fixes: vec![],
                    });
                }
            }
        }
    }

    problems
}

// #endregion Constraint: Type Name Uniqueness

// #region Constraint: Design Name Uniqueness

pub fn design_name_uniqueness_constraint(ctx: &ValidationContext) -> Vec<Problem> {
    let mut problems = Vec::new();

    if let Some(designs) = &ctx.kit.designs {
        let mut by_parent: HashMap<Option<Guid>, Vec<&Design>> = HashMap::new();
        
        for d in designs {
            let parent_guid = d.parent.as_ref().map(|p| p.guid.clone());
            by_parent.entry(parent_guid).or_default().push(d);
        }

        for (_parent_guid, siblings) in by_parent {
            let mut names: HashMap<String, Vec<&Design>> = HashMap::new();
            
            for d in &siblings {
                names.entry(d.name.clone()).or_default().push(d);
            }

            for (name, group) in names {
                if group.len() <= 1 {
                    continue;
                }

                for d in group.iter().skip(1) {
                    problems.push(Problem {
                        constraint_id: "design-name-unique".to_string(),
                        message: format!("Duplicate design name \"{}\" among siblings.", name),
                        location: DomainLocation {
                            entity_kind: EntityKind::Design,
                            entity_guid: Some(d.guid.clone()),
                            field: Some("name".to_string()),
                        },
                        related_guids: group.iter().map(|x| x.guid.clone()).collect(),
                        fixes: vec![],
                    });
                }
            }
        }
    }

    problems
}

// #endregion Constraint: Design Name Uniqueness

// #region Constraint: Piece Name Uniqueness

pub fn piece_name_uniqueness_constraint(ctx: &ValidationContext) -> Vec<Problem> {
    let mut problems = Vec::new();

    if let Some(designs) = &ctx.kit.designs {
        for design in designs {
            if let Some(pieces) = &design.pieces {
                let mut names: HashMap<String, Vec<&Piece>> = HashMap::new();
                
                for p in pieces {
                    let name = p.name.clone().unwrap_or_default();
                    names.entry(name).or_default().push(p);
                }

                for (name, group) in names {
                    if group.len() <= 1 {
                        continue;
                    }

                    for p in group.iter().skip(1) {
                        problems.push(Problem {
                            constraint_id: "piece-name-unique".to_string(),
                            message: format!("Duplicate piece name \"{}\" in design.", name),
                            location: DomainLocation {
                                entity_kind: EntityKind::Piece,
                                entity_guid: Some(p.guid.clone()),
                                field: Some("name".to_string()),
                            },
                            related_guids: group.iter().map(|x| x.guid.clone()).collect(),
                            fixes: vec![],
                        });
                    }
                }
            }
        }
    }

    problems
}

// #endregion Constraint: Piece Name Uniqueness
