//! Read-only kit graph commands; execute against a [`KitFullDto`] snapshot.
use serde::{Deserialize, Serialize};

use crate::connection::ConnectionFullDto;
use crate::connection::ConnectionIdDto;
use crate::connector::ConnectorFullDto;
use crate::connector::ConnectorIdDto;
use crate::design::DesignFullDto;
use crate::design::DesignIdDto;
use crate::id::Id;
use crate::kit::KitFullDto;
use crate::piece::PieceFullDto;
use crate::piece::PieceIdDto;
use crate::port::PortFullDto;
use crate::port::PortIdDto;
use crate::representation::RepresentationFullDto;
use crate::representation::RepresentationIdDto;
use crate::typ::TypeFullDto;
use crate::typ::TypeIdDto;
use crate::{error::SemioError, error::Result};

// --- Read command enums (mirrors plan; extensible) ---

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ReadConnectorCommand {
    Everything,
    Name,
    #[serde(other)]
    Other,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ReadConnectorCommandResult {
    Everything { dto: ConnectorFullDto },
    Name { name: String },
    Other,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ReadRepresentationCommand {
    Everything,
    Name,
    #[serde(other)]
    Other,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ReadRepresentationCommandResult {
    Everything { dto: RepresentationFullDto },
    Name { name: String },
    Other,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ReadPortCommand {
    Everything,
    Name,
    #[serde(other)]
    Other,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ReadPortCommandResult {
    Everything { dto: PortFullDto },
    Name { name: String },
    Other,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ReadPieceCommand {
    Everything,
    Name,
    #[serde(other)]
    Other,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ReadPieceCommandResult {
    Everything { dto: PieceFullDto },
    Name { name: String },
    Other,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ReadConnectionCommand {
    Everything,
    #[serde(other)]
    Other,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ReadConnectionCommandResult {
    Everything { dto: ConnectionFullDto },
    Other,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ReadTypeCommand {
    Everything,
    Name,
    Connectors,
    Representations,
    ReadConnectorCommands { id: ConnectorIdDto, commands: Vec<ReadConnectorCommand> },
    ReadRepresentationCommands { id: RepresentationIdDto, commands: Vec<ReadRepresentationCommand> },
    ReadPortCommands { id: PortIdDto, commands: Vec<ReadPortCommand> },
    #[serde(other)]
    Other,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ReadTypeCommandResult {
    Everything { dto: TypeFullDto },
    Name { name: String },
    Connectors { list: Vec<ConnectorFullDto> },
    Representations { list: Vec<RepresentationFullDto> },
    ReadConnectorCommands { results: Vec<ReadConnectorCommandResult> },
    ReadRepresentationCommands { results: Vec<ReadRepresentationCommandResult> },
    ReadPortCommands { results: Vec<ReadPortCommandResult> },
    Other,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ReadDesignCommand {
    Everything,
    Name,
    ReadPieceCommands { id: PieceIdDto, commands: Vec<ReadPieceCommand> },
    ReadConnectionCommands { id: ConnectionIdDto, commands: Vec<ReadConnectionCommand> },
    #[serde(other)]
    Other,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ReadDesignCommandResult {
    Everything { dto: DesignFullDto },
    Name { name: String },
    ReadPieceCommands { results: Vec<ReadPieceCommandResult> },
    ReadConnectionCommands { results: Vec<ReadConnectionCommandResult> },
    Other,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ReadKitCommand {
    /// Use empty object `{}` in JSON (camelCase `everything`) so the WASM/JSON spec shape works.
    Everything {},
    Name,
    Description,
    Types,
    Designs,
    Files,
    ReadTypeCommands { id: TypeIdDto, commands: Vec<ReadTypeCommand> },
    ReadDesignCommands { id: DesignIdDto, commands: Vec<ReadDesignCommand> },
    #[serde(other)]
    Other,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ReadKitCommandResult {
    Everything { dto: KitFullDto },
    Name { name: String },
    Description { description: Option<String> },
    Types { list: Vec<TypeFullDto> },
    Designs { list: Vec<DesignFullDto> },
    ReadTypeCommands { results: Vec<ReadTypeCommandResult> },
    ReadDesignCommands { results: Vec<ReadDesignCommandResult> },
    Other,
}

fn find_type(kit: &KitFullDto, id: &Id) -> Option<TypeFullDto> {
    kit.types.iter().find(|t| t.id == *id).cloned()
}

/// Execute a single [`ReadKitCommand`] against a DTO snapshot.
pub fn read_kit(kit: &KitFullDto, cmd: &ReadKitCommand) -> Result<ReadKitCommandResult> {
    match cmd {
        ReadKitCommand::Everything {} => Ok(ReadKitCommandResult::Everything { dto: kit.clone() }),
        ReadKitCommand::Name => Ok(ReadKitCommandResult::Name {
            name: kit.name.clone(),
        }),
        ReadKitCommand::Description => Ok(ReadKitCommandResult::Description {
            description: kit.description.clone(),
        }),
        ReadKitCommand::Types => Ok(ReadKitCommandResult::Types {
            list: kit.types.clone(),
        }),
        ReadKitCommand::Designs => Ok(ReadKitCommandResult::Designs {
            list: kit.designs.clone(),
        }),
        ReadKitCommand::ReadTypeCommands { id, commands } => {
            let t = find_type(kit, &id.id).ok_or_else(|| SemioError::NotFound {
                kind: "Type",
                id: id.id.clone(),
            })?;
            let mut results = Vec::with_capacity(commands.len());
            for c in commands {
                results.push(read_type(&t, c)?);
            }
            Ok(ReadKitCommandResult::ReadTypeCommands { results })
        }
        ReadKitCommand::ReadDesignCommands { id, commands } => {
            let d = kit
                .designs
                .iter()
                .find(|x| x.id == id.id)
                .cloned()
                .ok_or_else(|| SemioError::NotFound {
                    kind: "Design",
                    id: id.id.clone(),
                })?;
            let mut results = Vec::with_capacity(commands.len());
            for c in commands {
                results.push(read_design(&d, c)?);
            }
            Ok(ReadKitCommandResult::ReadDesignCommands { results })
        }
        ReadKitCommand::Files => Ok(ReadKitCommandResult::Other), // DTO has `files` via Everything
        ReadKitCommand::Other => Ok(ReadKitCommandResult::Other),
    }
}

fn read_type(t: &TypeFullDto, cmd: &ReadTypeCommand) -> Result<ReadTypeCommandResult> {
    match cmd {
        ReadTypeCommand::Everything => Ok(ReadTypeCommandResult::Everything { dto: t.clone() }),
        ReadTypeCommand::Name => Ok(ReadTypeCommandResult::Name {
            name: t.name.clone(),
        }),
        ReadTypeCommand::Connectors => Ok(ReadTypeCommandResult::Connectors {
            list: t.connectors.clone(),
        }),
        ReadTypeCommand::Representations => Ok(ReadTypeCommandResult::Representations {
            list: t.representations.clone(),
        }),
        ReadTypeCommand::ReadConnectorCommands { id, commands } => {
            let c = t
                .connectors
                .iter()
                .find(|x| x.id == id.id)
                .ok_or_else(|| SemioError::NotFound {
                    kind: "Connector",
                    id: id.id.clone(),
                })?;
            let mut results = Vec::new();
            for x in commands {
                results.push(match x {
                    ReadConnectorCommand::Everything => ReadConnectorCommandResult::Everything {
                        dto: c.clone(),
                    },
                    ReadConnectorCommand::Name => ReadConnectorCommandResult::Name {
                        name: c.code.clone(),
                    },
                    _ => ReadConnectorCommandResult::Other,
                });
            }
            Ok(ReadTypeCommandResult::ReadConnectorCommands { results })
        }
        ReadTypeCommand::ReadRepresentationCommands { id, commands } => {
            let r = t
                .representations
                .iter()
                .find(|x| x.id == id.id)
                .ok_or_else(|| SemioError::NotFound {
                    kind: "Representation",
                    id: id.id.clone(),
                })?;
            let mut results = Vec::new();
            for x in commands {
                results.push(match x {
                    ReadRepresentationCommand::Everything => {
                        ReadRepresentationCommandResult::Everything { dto: r.clone() }
                    }
                    ReadRepresentationCommand::Name => ReadRepresentationCommandResult::Name {
                        name: r
                            .description
                            .clone()
                            .unwrap_or_else(|| r.url.clone()),
                    },
                    _ => ReadRepresentationCommandResult::Other,
                });
            }
            Ok(ReadTypeCommandResult::ReadRepresentationCommands { results })
        }
        ReadTypeCommand::ReadPortCommands { id, commands } => {
            let p = t
                .ports
                .iter()
                .find(|x| x.id == id.id)
                .ok_or_else(|| SemioError::NotFound {
                    kind: "Port",
                    id: id.id.clone(),
                })?;
            let mut results = Vec::new();
            for x in commands {
                results.push(match x {
                    ReadPortCommand::Everything => ReadPortCommandResult::Everything { dto: p.clone() },
                    ReadPortCommand::Name => ReadPortCommandResult::Name {
                        name: p.family.clone().unwrap_or_default(),
                    },
                    _ => ReadPortCommandResult::Other,
                });
            }
            Ok(ReadTypeCommandResult::ReadPortCommands { results })
        }
        ReadTypeCommand::Other => Ok(ReadTypeCommandResult::Other),
    }
}

fn read_design(d: &DesignFullDto, cmd: &ReadDesignCommand) -> Result<ReadDesignCommandResult> {
    match cmd {
        ReadDesignCommand::Everything => Ok(ReadDesignCommandResult::Everything { dto: d.clone() }),
        ReadDesignCommand::Name => Ok(ReadDesignCommandResult::Name {
            name: d.name.clone(),
        }),
        ReadDesignCommand::ReadPieceCommands { id, commands } => {
            let p = d
                .pieces
                .iter()
                .find(|x| x.id == id.id)
                .ok_or_else(|| SemioError::NotFound {
                    kind: "Piece",
                    id: id.id.clone(),
                })?;
            let mut results = Vec::new();
            for x in commands {
                results.push(match x {
                    ReadPieceCommand::Everything => ReadPieceCommandResult::Everything { dto: p.clone() },
                    ReadPieceCommand::Name => ReadPieceCommandResult::Name {
                        name: p.name.clone().unwrap_or_default(),
                    },
                    _ => ReadPieceCommandResult::Other,
                });
            }
            Ok(ReadDesignCommandResult::ReadPieceCommands { results })
        }
        ReadDesignCommand::ReadConnectionCommands { id, commands } => {
            let c = d
                .connections
                .iter()
                .find(|x| x.id == id.id)
                .ok_or_else(|| SemioError::NotFound {
                    kind: "Connection",
                    id: id.id.clone(),
                })?;
            let mut results = Vec::new();
            for x in commands {
                results.push(match x {
                    ReadConnectionCommand::Everything => {
                        ReadConnectionCommandResult::Everything { dto: c.clone() }
                    }
                    _ => ReadConnectionCommandResult::Other,
                });
            }
            Ok(ReadDesignCommandResult::ReadConnectionCommands { results })
        }
        ReadDesignCommand::Other => Ok(ReadDesignCommandResult::Other),
    }
}

/// Run many read commands, preserving order.
pub fn read_kits(kit: &KitFullDto, commands: &[ReadKitCommand]) -> Result<Vec<ReadKitCommandResult>> {
    commands.iter().map(|c| read_kit(kit, c)).collect()
}
