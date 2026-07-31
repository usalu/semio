"""Re-apply legacy cleanup to lib.rs (idempotent where possible)."""
import re
from pathlib import Path

p = Path(r"c:\git\compose\compose\client\lib\rs\lib.rs")
t = p.read_text(encoding="utf-8")

t = t.replace(
    "    async fn legacy_created_fixed_piece_to_kit_op(input: &serde_json::Value)",
    "    async fn stored_create_fixed_piece_operation(input: &serde_json::Value)",
)
old_stored = """    pub(crate) async fn kit_operation_from_stored(kind: &str, input: &serde_json::Value) -> Result<crate::operation::Operation, ComposeError> {
        if kind == "createdFixedPiece" {
            return legacy_created_fixed_piece_to_kit_op(input).await;
        }
        kit_operation_from_step_json(input)
    }"""
new_stored = """    pub(crate) async fn kit_operation_from_stored(kind: &str, input: &serde_json::Value) -> Result<crate::operation::Operation, ComposeError> {
        match kind {
            "createFixedPiece" => stored_create_fixed_piece_operation(input).await,
            _ => kit_operation_from_step_json(input),
        }
    }"""
if old_stored in t:
    t = t.replace(old_stored, new_stored)
elif "legacy_created_fixed_piece" in t:
    t = t.replace(
        '        if kind == "createdFixedPiece" {\n            return legacy_created_fixed_piece_to_kit_op(input).await;\n        }\n        kit_operation_from_step_json(input)',
        '        match kind {\n            "createFixedPiece" => stored_create_fixed_piece_operation(input).await,\n            _ => kit_operation_from_step_json(input),\n        }',
    )

t = t.replace(
    "    /// @emoji 📑️ US-001 golden JSON: top-level `operations` array, or legacy key `ops` (see `kit-store.golden.ops.compose.json`).\n    pub fn golden_operation_records_ref",
    "    /// @emoji 📑️ US-001 golden JSON: top-level `operations` array.\n    pub fn golden_operation_records_ref",
)
t = t.replace(
    'src.get("operations").and_then(|v| v.as_array()).or_else(|| src.get("ops").and_then(|v| v.as_array())).ok_or_else(|| ComposeError::invalid("golden operations missing `operations` or `ops` array"))',
    'src.get("operations").and_then(|v| v.as_array()).ok_or_else(|| ComposeError::invalid("golden operations missing `operations` array"))',
)
t = re.sub(
    r"\n        pub async fn stub_ok\(\) -> Self \{[^}]+\}\n",
    "\n",
    t,
    count=1,
    flags=re.DOTALL,
)
t = t.replace(
    "crate::operation::CommandResponse::stub_ok().await",
    'crate::operation::CommandResponse::fail_msg("not implemented").await',
)
t = t.replace('if kind != "createdFixedPiece"', 'if kind != "createFixedPiece"')
t = t.replace('match kind {\n                    "createdFixedPiece" =>', 'match kind {\n                    "createFixedPiece" =>')
t = t.replace('.expect("operations|ops")', '.expect("operations")')
t = t.replace("legacy_workspace", "golden_draft_id")

helper = """    async fn dispatch_unsaved_kit_operation(rt: &Arc<ParentStore>, change_id: &Id, operation: crate::operation::Operation) -> crate::operation::CommandResponse {
        let Some((workspace_id, transaction_id)) = rt.wip_kit_scope.read().await.clone() else {
            return crate::operation::CommandResponse::fail_msg("no active kit scope").await;
        };
        if transaction_id != *change_id {
            return crate::operation::CommandResponse::fail_msg("change id mismatch for kit operation").await;
        }
        let request_id = Id::new().await;
        rt.dispatch_wip_wait(Command::ApplyOperation { request_id, workspace_id, transaction_id, operation }).await
    }

    pub struct UnsavedChangeCommand"""
if "dispatch_unsaved_kit_operation" not in t:
    t = t.replace("    pub struct UnsavedChangeCommand {", helper)

save_old = """                    Ok(crate::operation::CommandResponse::fail_msg("not implemented").await.into())
                }
                Err(e) => Ok(crate::operation::CommandResponse::fail(e).await.into()),
            }
        }
    }

    pub struct AlternativeCommand"""
save_new = """                    Ok(crate::operation::CommandResponse::ok_request(tx_id).await.into())
                }
                Err(e) => Ok(crate::operation::CommandResponse::fail(e).await.into()),
            }
        }
    }

    pub struct AlternativeCommand"""
if "commit_transaction" in t and "ok_request(tx_id)" not in t.split("UnsavedChangeCommand")[1][:2500]:
    t = t.replace(save_old, save_new, 1)

# to_backwards exhaustiveness
if "Operation::CreateDesign { .. } | Operation::CreateType { .. }" not in t:
    t = t.replace(
        """                Operation::FixPieceInDesign { scope, .. } => {
                    let Scope::PieceInDesign { design_id, piece_id } = scope else {
                        return Err(ComposeError::invalid("fixPieceInDesign expects Scope::PieceInDesign"));
                    };
                    let piece = ensure_piece(kit, design_id, piece_id).await?;
                    let connection_kind = {
                        let guard = piece.connection_kind.read().await;
                        *guard
                    };
                    drop(piece);
                    match connection_kind {
                        Some(crate::kit::design::piece::PieceConnectionKind::Fixed) => Ok(Vec::new()),
                        _ => Err(ComposeError::invalid("fixPieceInDesign backwards is unsupported for non-fixed pre-state")),
                    }
                }
            }
        }
    }

    fn validate_attribute_ids""",
        """                Operation::FixPieceInDesign { scope, .. } => {
                    let Scope::PieceInDesign { design_id, piece_id } = scope else {
                        return Err(ComposeError::invalid("fixPieceInDesign expects Scope::PieceInDesign"));
                    };
                    let piece = ensure_piece(kit, design_id, piece_id).await?;
                    let connection_kind = {
                        let guard = piece.connection_kind.read().await;
                        *guard
                    };
                    drop(piece);
                    match connection_kind {
                        Some(crate::kit::design::piece::PieceConnectionKind::Fixed) => Ok(Vec::new()),
                        _ => Err(ComposeError::invalid("fixPieceInDesign backwards is unsupported for non-fixed pre-state")),
                    }
                }
                Operation::CreateDesign { .. } | Operation::CreateType { .. } => Err(ComposeError::invalid("backwards not implemented for createDesign/createType")),
            }
        }
    }

    fn validate_attribute_ids""",
    )

p.write_text(t, encoding="utf-8", newline="\n")
print("apply-legacy-cleanup done")
