//! Structural change commands; applied to a [`KitStore`] during an open transaction.
use serde::{Deserialize, Serialize};

use crate::design::DesignIdDto;
use crate::diff::DesignDiff;
use crate::id::Id;
use crate::kit::KitStore;
use crate::kit_change::KitChangeKind;
use crate::piece::PieceFullDto;
use crate::piece::PieceIdDto;
use crate::typ::TypeIdDto;
use crate::{error::Result, error::SemioError};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ChangePieceCommand {
    Name { name: String },
    Fix,
    #[serde(other)]
    Other,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ChangeDesignCommand {
    Name { name: String },
    ChangePieceCommands {
        piece_id: PieceIdDto,
        commands: Vec<ChangePieceCommand>,
    },
    #[serde(other)]
    Other,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ChangeTypeCommand {
    Name { name: String },
    #[serde(other)]
    Other,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ChangeKitCommand {
    Name { name: String },
    Description { description: Option<String> },
    ChangeTypeCommands {
        type_id: TypeIdDto,
        commands: Vec<ChangeTypeCommand>,
    },
    ChangeDesignCommands {
        design_id: DesignIdDto,
        commands: Vec<ChangeDesignCommand>,
    },
    #[serde(other)]
    Other,
}

fn find_design_id_for_piece(kit: &KitStore, piece_id: &Id) -> Option<String> {
    for d in &kit.designs {
        if let Ok(dr) = d.read() {
            if dr.piece(piece_id.as_str()).is_some() {
                return Some(dr.id.to_string());
            }
        }
    }
    None
}

/// Apply one change command; caller wraps with before/after snapshot for [`crate::kit_change::KitChange`].
pub fn apply_change_kit_command(kit: &mut KitStore, cmd: &ChangeKitCommand) -> Result<KitChangeKind> {
    match cmd {
        ChangeKitCommand::Name { name } => {
            kit.set_name(name.clone())
                .map_err(|e| SemioError::InvalidOperation(e.to_string()))?;
            Ok(KitChangeKind::SetKitMetadata)
        }
        ChangeKitCommand::Description { description } => {
            kit.set_description(description.clone())
                .map_err(|e| SemioError::InvalidOperation(e.to_string()))?;
            Ok(KitChangeKind::SetKitMetadata)
        }
        ChangeKitCommand::ChangeTypeCommands { type_id, commands } => {
            let tid = type_id.id.to_string();
            for c in commands {
                match c {
                    ChangeTypeCommand::Name { name } => {
                        let t = kit
                            .semio_type(tid.as_str())
                            .ok_or_else(|| SemioError::NotFound {
                                kind: "Type",
                                id: type_id.id.clone(),
                            })?;
                        t.write()
                            .map_err(|_| SemioError::LockPoisoned("type"))?
                            .set_name(name.clone())
                            .map_err(|e| SemioError::InvalidOperation(e.to_string()))?;
                    }
                    ChangeTypeCommand::Other => {}
                }
            }
            Ok(KitChangeKind::ModifyType)
        }
        ChangeKitCommand::ChangeDesignCommands { design_id, commands } => {
            let dg = design_id.id.to_string();
            for c in commands {
                match c {
                    ChangeDesignCommand::Name { name } => {
                        let d = kit
                            .design(dg.as_str())
                            .ok_or_else(|| SemioError::NotFound {
                                kind: "Design",
                                id: design_id.id.clone(),
                            })?;
                        d.write()
                            .map_err(|_| SemioError::LockPoisoned("design"))?
                            .set_name(name.clone())
                            .map_err(|e| SemioError::InvalidOperation(e.to_string()))?;
                    }
                    ChangeDesignCommand::ChangePieceCommands { piece_id, commands: pcmds } => {
                        for pc in pcmds {
                            match pc {
                                ChangePieceCommand::Name { name } => {
                                    let did = find_design_id_for_piece(kit, &piece_id.id)
                                        .ok_or_else(|| SemioError::NotFound {
                                            kind: "Piece",
                                            id: piece_id.id.clone(),
                                        })?;
                                    let full = kit.to_full_dto();
                                    let mut p = full
                                        .designs
                                        .iter()
                                        .find(|d| d.id.as_str() == did.as_str())
                                        .and_then(|d| d.pieces.iter().find(|p| p.id == piece_id.id))
                                        .cloned()
                                        .ok_or_else(|| SemioError::NotFound {
                                            kind: "Piece",
                                            id: piece_id.id.clone(),
                                        })?;
                                    p.name = Some(name.clone());
                                    let mut diff = DesignDiff::default();
                                    diff.modified_pieces.push(p);
                                    kit.apply_design_diff(&did, &diff)
                                        .map_err(|e| SemioError::InvalidOperation(e.to_string()))?;
                                }
                                ChangePieceCommand::Fix | ChangePieceCommand::Other => {}
                            }
                        }
                    }
                    ChangeDesignCommand::Other => {}
                }
            }
            Ok(KitChangeKind::ModifyDesign)
        }
        ChangeKitCommand::Other => Ok(KitChangeKind::Other("changeKit".into())),
    }
}

/// Apply design diff RPC shape (for future `execute` shims; kept alongside legacy RPCs).
#[allow(dead_code)]
pub fn apply_design_diff_cmd(kit: &mut KitStore, design_id: &str, diff: &DesignDiff) -> Result<KitChangeKind> {
    kit.apply_design_diff(design_id, diff)
        .map_err(|e| SemioError::InvalidOperation(e.to_string()))?;
    Ok(KitChangeKind::ApplyDesignDiff)
}

/// Add a piece under a design (for future `execute` shims; kept for parity with the plan's surface).
#[allow(dead_code)]
pub fn add_piece(kit: &mut KitStore, design_id: &str, piece: PieceFullDto) -> Result<KitChangeKind> {
    let mut d = DesignDiff::default();
    d.added_pieces.push(piece);
    kit.apply_design_diff(design_id, &d)
        .map_err(|e| SemioError::InvalidOperation(e.to_string()))?;
    Ok(KitChangeKind::AddPiece)
}
