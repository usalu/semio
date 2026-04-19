// Included by lib.rs (`mod kit_graph`). TypeScript-parallel graph commit: `KitGraphChange`, session, mutex, transactions, history.

use std::collections::HashMap;
use std::sync::Mutex;

use super::*;

// ——— KitGraphChange (aligns with TypeScript `KitChange` for graph mutations: diffs + pre-apply validation)

/// Bidirectional kit graph edit with [`KitDiffValidationResult`] captured before [`apply_kit_diff`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KitGraphChange {
    pub forward: KitDiff,
    pub backward: KitDiff,
    pub validation: KitDiffValidationResult,
}

/// [`KitGraphChange`] from precomputed diffs without running the validation pipeline (validation is a no-op success).
pub fn kit_graph_change_from_diffs(forward: KitDiff, backward: KitDiff) -> KitGraphChange {
    KitGraphChange {
        forward,
        backward,
        validation: KitDiffValidationResult {
            ok: true,
            errors: vec![],
            warnings: vec![],
            diff: None,
        },
    }
}

// ——— Backbone (TypeScript `Backbone` parallel)

/// Notified after a successful graph commit (see [`KitGraphSession::commit`]).
pub trait KitBackbone {
    fn changed(&mut self, change: &KitGraphChange);

    /// Optional hook for inbound remote edits. Default: no-op.
    ///
    /// Callers must **not** invoke `commit_inbound` synchronously if that re-enters the same
    /// [`KitGraphSession`] lock (deadlock). Prefer a channel or queued apply. TODO: structured inbound pipeline.
    fn attach(&mut self, _kit: &mut Kit, _commit_inbound: &mut dyn FnMut(KitDiff)) {}
}

// ——— Commit options (TypeScript `KitCommitOptions`)

#[derive(Debug, Clone)]
pub struct KitCommitOptions {
    pub transaction_id: Option<String>,
    pub notify_backbone: bool,
    pub skip_global_history: bool,
}

impl Default for KitCommitOptions {
    fn default() -> Self {
        Self {
            transaction_id: None,
            notify_backbone: true,
            skip_global_history: false,
        }
    }
}

// ——— Session (mutex + transactions + undo stacks)

struct KitOpenTransaction {
    start_kit: Kit,
    steps: Vec<KitGraphChange>,
    redo_steps: Vec<KitGraphChange>,
}

struct KitGraphSessionInner {
    kit: Kit,
    strict_mode: bool,
    is_conflicted: bool,
    open_transactions: HashMap<String, KitOpenTransaction>,
    history_past: Vec<KitGraphChange>,
    history_future: Vec<KitGraphChange>,
    backbone: Option<Box<dyn KitBackbone>>,
}

/// Managed kit graph: serializes mutations with a mutex, tracks transactions and undo/redo stacks (TypeScript `Kit` private state parallel).
pub struct KitGraphSession {
    inner: Mutex<KitGraphSessionInner>,
}

impl KitGraphSession {
    pub fn new(kit: Kit) -> Self {
        Self {
            inner: Mutex::new(KitGraphSessionInner {
                kit,
                strict_mode: false,
                is_conflicted: false,
                open_transactions: HashMap::new(),
                history_past: vec![],
                history_future: vec![],
                backbone: None,
            }),
        }
    }

    pub fn with_backbone(kit: Kit, backbone: Box<dyn KitBackbone>) -> Self {
        Self {
            inner: Mutex::new(KitGraphSessionInner {
                kit,
                strict_mode: false,
                is_conflicted: false,
                open_transactions: HashMap::new(),
                history_past: vec![],
                history_future: vec![],
                backbone: Some(backbone),
            }),
        }
    }

    pub fn set_backbone(&self, backbone: Option<Box<dyn KitBackbone>>) -> Result<()> {
        let mut g = self.inner.lock().map_err(|_| SemioError::InvalidOperation {
            message: "KitGraphSession mutex poisoned".into(),
        })?;
        g.backbone = backbone;
        Ok(())
    }

    pub fn set_strict_mode(&self, strict: bool) -> Result<()> {
        let mut g = self.inner.lock().map_err(|_| SemioError::InvalidOperation {
            message: "KitGraphSession mutex poisoned".into(),
        })?;
        g.strict_mode = strict;
        Ok(())
    }

    pub fn clear_conflict(&self) -> Result<()> {
        let mut g = self.inner.lock().map_err(|_| SemioError::InvalidOperation {
            message: "KitGraphSession mutex poisoned".into(),
        })?;
        g.is_conflicted = false;
        Ok(())
    }

    pub fn map_kit<T, F: FnOnce(&Kit) -> T>(&self, f: F) -> Result<T> {
        let g = self.inner.lock().map_err(|_| SemioError::InvalidOperation {
            message: "KitGraphSession mutex poisoned".into(),
        })?;
        Ok(f(&g.kit))
    }

    pub fn map_kit_mut<T, F: FnOnce(&mut Kit) -> T>(&self, f: F) -> Result<T> {
        let mut g = self.inner.lock().map_err(|_| SemioError::InvalidOperation {
            message: "KitGraphSession mutex poisoned".into(),
        })?;
        Ok(f(&mut g.kit))
    }

    /// Validates against the current kit, inverts, applies, then records transaction/history/backbone (see [`KitCommitOptions`]).
    pub fn commit(&self, diff: KitDiff, opts: KitCommitOptions) -> Result<KitGraphChange> {
        let mut g = self.inner.lock().map_err(|_| SemioError::InvalidOperation {
            message: "KitGraphSession mutex poisoned".into(),
        })?;
        g.commit_graph(diff, opts)
    }

    pub fn start_transaction(&self) -> Result<String> {
        let mut g = self.inner.lock().map_err(|_| SemioError::InvalidOperation {
            message: "KitGraphSession mutex poisoned".into(),
        })?;
        if g.is_conflicted {
            return Err(SemioError::InvalidOperation {
                message: "Kit has unresolved validation conflicts; call clear_conflict() first".into(),
            });
        }
        let id = guid();
        let start_kit = g.kit.clone();
        g.open_transactions.insert(
            id.clone(),
            KitOpenTransaction {
                start_kit,
                steps: vec![],
                redo_steps: vec![],
            },
        );
        Ok(id)
    }

    pub fn abort_transaction(&self, transaction_id: &str) -> Result<()> {
        let mut g = self.inner.lock().map_err(|_| SemioError::InvalidOperation {
            message: "KitGraphSession mutex poisoned".into(),
        })?;
        let tx = g
            .open_transactions
            .remove(transaction_id)
            .ok_or_else(|| SemioError::InvalidOperation {
                message: format!("Unknown transaction {}", transaction_id),
            })?;
        if g.is_conflicted {
            return Err(SemioError::InvalidOperation {
                message: "Kit is conflicted; call clear_conflict() before aborting a transaction".into(),
            });
        }
        for step in tx.steps.iter().rev() {
            apply_kit_diff(&mut g.kit, &step.backward);
        }
        Ok(())
    }

    pub fn finalize_transaction(&self, transaction_id: &str) -> Result<KitGraphChange> {
        let mut g = self.inner.lock().map_err(|_| SemioError::InvalidOperation {
            message: "KitGraphSession mutex poisoned".into(),
        })?;
        if g.is_conflicted {
            return Err(SemioError::InvalidOperation {
                message: "Kit is conflicted; call clear_conflict() before finalizing a transaction".into(),
            });
        }
        let tx = g
            .open_transactions
            .remove(transaction_id)
            .ok_or_else(|| SemioError::InvalidOperation {
                message: format!("Unknown transaction {}", transaction_id),
            })?;
        let sk = &tx.start_kit;
        let forward_raw = get_kit_diff(sk, &g.kit);
        let validation = validate_kit_diff(sk, &forward_raw, false);
        if !validation.ok || !validation.errors.is_empty() {
            g.open_transactions.insert(transaction_id.to_string(), tx);
            return Err(SemioError::Validation {
                message: validation
                    .errors
                    .iter()
                    .map(|e| e.message.clone())
                    .collect::<Vec<_>>()
                    .join("; "),
            });
        }
        if g.strict_mode && !validation.warnings.is_empty() {
            g.open_transactions.insert(transaction_id.to_string(), tx);
            return Err(SemioError::Validation {
                message: validation
                    .warnings
                    .iter()
                    .map(|e| e.message.clone())
                    .collect::<Vec<_>>()
                    .join("; "),
            });
        }
        let diff_to_apply = validation.diff.clone().unwrap_or_else(|| forward_raw.clone());
        let backward = inverse_kit_diff(sk, &diff_to_apply);
        let squashed = KitGraphChange {
            forward: diff_to_apply,
            backward,
            validation,
        };
        g.history_past.push(squashed.clone());
        g.history_future.clear();
        if let Some(ref mut bb) = g.backbone {
            bb.changed(&squashed);
        }
        Ok(squashed)
    }

    pub fn undo_within_transaction(&self, transaction_id: &str) -> Result<()> {
        let mut g = self.inner.lock().map_err(|_| SemioError::InvalidOperation {
            message: "KitGraphSession mutex poisoned".into(),
        })?;
        if g.is_conflicted {
            return Err(SemioError::InvalidOperation {
                message: "Kit is conflicted".into(),
            });
        }
        if !g.open_transactions.contains_key(transaction_id) {
            return Err(SemioError::InvalidOperation {
                message: format!("Unknown transaction {}", transaction_id),
            });
        }
        let ch = {
            let tx = g.open_transactions.get_mut(transaction_id).expect("transaction id checked");
            tx.steps.pop()
        };
        let Some(ch) = ch else { return Ok(()) };
        apply_kit_diff(&mut g.kit, &ch.backward);
        g.open_transactions
            .get_mut(transaction_id)
            .expect("transaction id checked")
            .redo_steps
            .push(ch);
        Ok(())
    }

    pub fn redo_within_transaction(&self, transaction_id: &str) -> Result<()> {
        let mut g = self.inner.lock().map_err(|_| SemioError::InvalidOperation {
            message: "KitGraphSession mutex poisoned".into(),
        })?;
        if g.is_conflicted {
            return Err(SemioError::InvalidOperation {
                message: "Kit is conflicted".into(),
            });
        }
        if !g.open_transactions.contains_key(transaction_id) {
            return Err(SemioError::InvalidOperation {
                message: format!("Unknown transaction {}", transaction_id),
            });
        }
        let ch = {
            let tx = g.open_transactions.get_mut(transaction_id).expect("transaction id checked");
            tx.redo_steps.pop()
        };
        let Some(ch) = ch else { return Ok(()) };
        apply_kit_diff(&mut g.kit, &ch.forward);
        g.open_transactions
            .get_mut(transaction_id)
            .expect("transaction id checked")
            .steps
            .push(ch);
        Ok(())
    }

    pub fn undo_history(&self) -> Result<()> {
        let mut g = self.inner.lock().map_err(|_| SemioError::InvalidOperation {
            message: "KitGraphSession mutex poisoned".into(),
        })?;
        if g.is_conflicted {
            return Err(SemioError::InvalidOperation {
                message: "Kit is conflicted".into(),
            });
        }
        let Some(ch) = g.history_past.pop() else {
            return Ok(());
        };
        apply_kit_diff(&mut g.kit, &ch.backward);
        g.history_future.push(ch);
        Ok(())
    }

    pub fn redo_history(&self) -> Result<()> {
        let mut g = self.inner.lock().map_err(|_| SemioError::InvalidOperation {
            message: "KitGraphSession mutex poisoned".into(),
        })?;
        if g.is_conflicted {
            return Err(SemioError::InvalidOperation {
                message: "Kit is conflicted".into(),
            });
        }
        let Some(ch) = g.history_future.pop() else {
            return Ok(());
        };
        apply_kit_diff(&mut g.kit, &ch.forward);
        g.history_past.push(ch);
        Ok(())
    }

    pub fn can_undo_history(&self) -> Result<bool> {
        let g = self.inner.lock().map_err(|_| SemioError::InvalidOperation {
            message: "KitGraphSession mutex poisoned".into(),
        })?;
        Ok(!g.history_past.is_empty())
    }

    pub fn can_redo_history(&self) -> Result<bool> {
        let g = self.inner.lock().map_err(|_| SemioError::InvalidOperation {
            message: "KitGraphSession mutex poisoned".into(),
        })?;
        Ok(!g.history_future.is_empty())
    }
}

impl KitGraphSessionInner {
    fn commit_graph(&mut self, diff: KitDiff, opts: KitCommitOptions) -> Result<KitGraphChange> {
        if self.is_conflicted {
            return Err(SemioError::InvalidOperation {
                message: "Kit has unresolved validation conflicts; call clear_conflict() before applying further changes."
                    .into(),
            });
        }

        let validation = validate_kit_diff(&self.kit, &diff, false);
        if !validation.ok || !validation.errors.is_empty() {
            self.is_conflicted = true;
            return Err(SemioError::Validation {
                message: validation
                    .errors
                    .iter()
                    .map(|e| e.message.clone())
                    .collect::<Vec<_>>()
                    .join("; "),
            });
        }
        if self.strict_mode && !validation.warnings.is_empty() {
            self.is_conflicted = true;
            return Err(SemioError::Validation {
                message: validation
                    .warnings
                    .iter()
                    .map(|e| e.message.clone())
                    .collect::<Vec<_>>()
                    .join("; "),
            });
        }

        let diff_to_apply = validation.diff.clone().unwrap_or_else(|| diff.clone());
        let backward = inverse_kit_diff(&self.kit, &diff_to_apply);
        apply_kit_diff(&mut self.kit, &diff_to_apply);

        let change = KitGraphChange {
            forward: diff_to_apply,
            backward,
            validation,
        };

        if let Some(tx_id) = &opts.transaction_id {
            let tx = self.open_transactions.get_mut(tx_id).ok_or_else(|| SemioError::InvalidOperation {
                message: format!("Unknown transaction {}", tx_id),
            })?;
            tx.steps.push(change.clone());
            tx.redo_steps.clear();
        } else if !opts.skip_global_history {
            self.history_past.push(change.clone());
            self.history_future.clear();
        }

        let notify_backbone = opts.notify_backbone && opts.transaction_id.is_none();
        if notify_backbone {
            if let Some(ref mut bb) = self.backbone {
                bb.changed(&change);
            }
        }

        self.is_conflicted = false;
        Ok(change)
    }
}

/// Applies a validated graph mutation on [`KitGraphSession`] (low-level parallel to TypeScript `commitKitGraphChange`).
pub fn commit_kit_graph_change(session: &KitGraphSession, diff: KitDiff, opts: KitCommitOptions) -> Result<KitGraphChange> {
    session.commit(diff, opts)
}
