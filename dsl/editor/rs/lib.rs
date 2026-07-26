//! 🧬 `dsl_editor` — the token-transaction editor layer: undo/redo history over `dsl_token`'s
//! invertible transactions (with gesture coalescing so a drag/typing sequence is one undo step),
//! stable-position selections, bracket-matching structural navigation, and a bridge from
//! `dsl_schema::LanguageService` completions to token-edit transactions.

use dsl_core::TokenKind;
use dsl_schema::{CompletionItem, LanguageService, RecordSpec};
use dsl_token::{Lineage, Position, TokenPatch, TokenRope, TokenTransaction, TransactionError};

//#region 🔖Selection
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Selection {
    pub anchor: Position,
    pub head: Position,
}

impl Selection {
    pub fn collapsed(position: Position) -> Self {
        Self { anchor: position, head: position }
    }
}
//#endregion 🔖Selection

//#region 🔖Document
/// @emoji 📝 One undo/redo step: a group of inverse transactions applied together (a coalesced
/// gesture collapses to one group instead of one step per tick), plus the selection to restore.
struct HistoryGroup {
    coalesce_key: Option<String>,
    inverses: Vec<TokenTransaction>,
    selection_before: Option<Selection>,
}

pub struct EditorDocument {
    rope: TokenRope,
    undo_stack: Vec<HistoryGroup>,
    redo_stack: Vec<HistoryGroup>,
    selection: Option<Selection>,
}

impl EditorDocument {
    pub fn from_rope(rope: TokenRope) -> Self {
        Self { rope, undo_stack: Vec::new(), redo_stack: Vec::new(), selection: None }
    }

    pub fn rope(&self) -> &TokenRope {
        &self.rope
    }

    pub fn selection(&self) -> Option<Selection> {
        self.selection
    }

    pub fn set_selection(&mut self, selection: Selection) {
        self.selection = Some(selection);
    }

    /// @emoji ⚡ Applies `transaction` as its own undo step (equivalent to `apply_coalesced` with
    /// `coalesce_key: None`).
    pub fn apply(&mut self, transaction: &TokenTransaction) -> Result<(), TransactionError> {
        self.apply_coalesced(transaction, None)
    }

    /// @emoji 🪢 Applies `transaction`. If `coalesce_key` is `Some` and matches the key of the
    /// most recent (still-open) undo group, the new inverse is appended to that SAME group
    /// instead of starting a new one — so a whole coalesced gesture (drag, continuous typing)
    /// undoes in one step. Always clears the redo stack (a fresh edit invalidates any redo history).
    pub fn apply_coalesced(&mut self, transaction: &TokenTransaction, coalesce_key: Option<&str>) -> Result<(), TransactionError> {
        let selection_before = self.selection;
        let inverse = self.rope.apply(transaction)?;
        let can_extend = coalesce_key.is_some()
            && self.undo_stack.last().is_some_and(|group| group.coalesce_key.as_deref() == coalesce_key);
        if can_extend {
            let group = self.undo_stack.last_mut().expect("can_extend implies a last group exists");
            // The new inverse undoes the LATEST tick, so it must run FIRST when the group is
            // undone (LIFO) — prepend it ahead of the earlier ticks' inverses.
            group.inverses.insert(0, inverse);
        } else {
            self.undo_stack.push(HistoryGroup { coalesce_key: coalesce_key.map(|k| k.to_string()), inverses: vec![inverse], selection_before });
        }
        self.redo_stack.clear();
        Ok(())
    }

    /// @emoji ↩️ Undoes the most recent group (every tick of a coalesced gesture at once),
    /// restoring the selection captured just before that group was first applied.
    pub fn undo(&mut self) -> bool {
        let Some(group) = self.undo_stack.pop() else { return false };
        let mut redo_inverses = Vec::with_capacity(group.inverses.len());
        for inverse in &group.inverses {
            // A group's inverses were captured one apply at a time, each stamped with the
            // revision immediately after ITS OWN forward apply — but revision keeps advancing
            // monotonically as we replay them here (applying inverse N bumps the revision that
            // inverse N+1's precondition was captured against), so the stored `base_revision` is
            // stale by the second entry. Re-stamp it to the rope's actual current revision before
            // each replay: this batch's ORDER is already known-correct (we built it ourselves),
            // only the precondition snapshot goes stale.
            let restamped = TokenTransaction { base_revision: self.rope.revision(), patches: inverse.patches.clone() };
            match self.rope.apply(&restamped) {
                Ok(redo_tx) => redo_inverses.push(redo_tx),
                Err(_) => return false,
            }
        }
        redo_inverses.reverse();
        let selection_after = self.selection;
        if let Some(before) = group.selection_before {
            self.selection = Some(before);
        }
        self.redo_stack.push(HistoryGroup { coalesce_key: group.coalesce_key, inverses: redo_inverses, selection_before: selection_after });
        true
    }

    /// @emoji ↪️ Redoes the most recently undone group.
    pub fn redo(&mut self) -> bool {
        let Some(group) = self.redo_stack.pop() else { return false };
        let mut undo_inverses = Vec::with_capacity(group.inverses.len());
        for inverse in &group.inverses {
            let restamped = TokenTransaction { base_revision: self.rope.revision(), patches: inverse.patches.clone() };
            match self.rope.apply(&restamped) {
                Ok(undo_tx) => undo_inverses.push(undo_tx),
                Err(_) => return false,
            }
        }
        undo_inverses.reverse();
        let selection_before = self.selection;
        if let Some(after) = group.selection_before {
            self.selection = Some(after);
        }
        self.undo_stack.push(HistoryGroup { coalesce_key: group.coalesce_key, inverses: undo_inverses, selection_before });
        true
    }

    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }
}
//#endregion 🔖Document

//#region 🔖Navigation
const OPEN_BRACKETS: &[TokenKind] = &[TokenKind::LBrace, TokenKind::LBracket, TokenKind::LParen];
const CLOSE_BRACKETS: &[TokenKind] = &[TokenKind::RBrace, TokenKind::RBracket, TokenKind::RParen];

fn matching_close(open: TokenKind) -> Option<TokenKind> {
    match open {
        TokenKind::LBrace => Some(TokenKind::RBrace),
        TokenKind::LBracket => Some(TokenKind::RBracket),
        TokenKind::LParen => Some(TokenKind::RParen),
        _ => None,
    }
}

/// @emoji 🧭 Finds the token index matching the delimiter at `index` (its closer if `index` is an
/// opener, its opener if `index` is a closer), respecting nesting. `None` if `index` isn't a
/// delimiter or has no partner.
pub fn matching_delimiter(rope: &TokenRope, index: usize) -> Option<usize> {
    let tokens = rope.snapshot();
    let tokens = tokens.tokens();
    let token = tokens.get(index)?;
    if OPEN_BRACKETS.contains(&token.kind) {
        let close_kind = matching_close(token.kind)?;
        let mut depth = 0i32;
        for (i, candidate) in tokens.iter().enumerate().skip(index + 1) {
            if candidate.kind == token.kind {
                depth += 1;
            } else if candidate.kind == close_kind {
                if depth == 0 {
                    return Some(i);
                }
                depth -= 1;
            }
        }
        None
    } else if CLOSE_BRACKETS.contains(&token.kind) {
        let open_kind = OPEN_BRACKETS[CLOSE_BRACKETS.iter().position(|k| *k == token.kind)?];
        let mut depth = 0i32;
        for i in (0..index).rev() {
            let candidate = &tokens[i];
            if candidate.kind == token.kind {
                depth += 1;
            } else if candidate.kind == open_kind {
                if depth == 0 {
                    return Some(i);
                }
                depth -= 1;
            }
        }
        None
    } else {
        None
    }
}
//#endregion 🔖Navigation

//#region 🔖Completion
/// @emoji 💡 Turns a `LanguageService` completion choice into a `TokenTransaction` that inserts
/// its label as a fresh `Ident` token right before `at` — the "completion result contains a token
/// edit transaction" requirement, generalized over any `RecordSpec`.
pub fn completion_transaction(rope: &mut TokenRope, at: Position, item: &CompletionItem, limits: &dsl_core::Limits) -> Result<TokenTransaction, dsl_core::TextError> {
    let index = at.resolve(&rope.snapshot());
    let fresh = rope.tokenize_fresh(&item.label, limits, Lineage::Generated)?;
    Ok(TokenTransaction { base_revision: rope.revision(), patches: vec![TokenPatch::insert_at(index, fresh)] })
}

/// @emoji 💡 Convenience: builds a `LanguageService` for `spec` and returns its completions at
/// `offset` in `rope`'s current text.
pub fn completions_for(rope: &TokenRope, spec: &RecordSpec, offset: usize) -> Vec<CompletionItem> {
    let service = LanguageService::new(spec);
    service.completions(&rope.snapshot().text(), offset)
}
//#endregion 🔖Completion

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use dsl_core::Limits;

    fn sample_rope() -> TokenRope {
        TokenRope::from_text("a b c", &Limits::default()).expect("rope")
    }

    #[test]
    fn apply_undo_redo_round_trip_restores_text_and_selection() {
        let mut doc = EditorDocument::from_rope(sample_rope());
        let b_id = doc.rope().snapshot().tokens()[2].id;
        doc.set_selection(Selection::collapsed(Position::Before(b_id)));

        let mut rope_clone = TokenRope::from_text("a b c", &Limits::default()).expect("rope");
        let fresh = rope_clone.tokenize_fresh("X", &Limits::default(), Lineage::Authored).expect("tokenize");
        let tx = TokenTransaction { base_revision: doc.rope().revision(), patches: vec![TokenPatch::replace_range(2, 3, fresh)] };
        doc.apply(&tx).expect("apply");
        assert_eq!(doc.rope().snapshot().text(), "a X c");

        assert!(doc.undo());
        assert_eq!(doc.rope().snapshot().text(), "a b c");
        assert_eq!(doc.selection(), Some(Selection::collapsed(Position::Before(b_id))));

        assert!(doc.redo());
        assert_eq!(doc.rope().snapshot().text(), "a X c");
    }

    #[test]
    fn coalesced_gesture_undoes_in_a_single_step() {
        let mut doc = EditorDocument::from_rope(sample_rope());
        // Position 2 is "b" in "a b c"; each iteration replaces WHATEVER token currently sits
        // there (not a stale id — once replaced, the original "b" token's identity is gone, and
        // chasing its old `StableId` across further edits is exactly the bug this test used to
        // have).
        for replacement in ["X", "XY", "XYZ"] {
            let mut scratch = TokenRope::from_text("_", &Limits::default()).expect("scratch");
            let fresh = scratch.tokenize_fresh(replacement, &Limits::default(), Lineage::Authored).expect("tokenize");
            let tx = TokenTransaction { base_revision: doc.rope().revision(), patches: vec![TokenPatch::replace_range(2, 3, fresh)] };
            doc.apply_coalesced(&tx, Some("drag")).expect("apply coalesced");
        }
        assert_eq!(doc.rope().snapshot().text(), "a XYZ c");
        assert!(doc.undo(), "undo must succeed");
        assert_eq!(doc.rope().snapshot().text(), "a b c", "one undo must revert the WHOLE coalesced gesture");
        assert!(!doc.can_undo(), "the coalesced gesture was exactly one undo step");
    }

    #[test]
    fn non_coalesced_edits_undo_one_at_a_time() {
        let mut doc = EditorDocument::from_rope(sample_rope());
        for replacement in ["X", "Y"] {
            let mut scratch = TokenRope::from_text("_", &Limits::default()).expect("scratch");
            let fresh = scratch.tokenize_fresh(replacement, &Limits::default(), Lineage::Authored).expect("tokenize");
            let tx = TokenTransaction { base_revision: doc.rope().revision(), patches: vec![TokenPatch::replace_range(2, 3, fresh)] };
            doc.apply(&tx).expect("apply");
        }
        assert_eq!(doc.rope().snapshot().text(), "a Y c");
        assert!(doc.undo());
        assert_eq!(doc.rope().snapshot().text(), "a X c", "each un-coalesced apply is its own undo step");
        assert!(doc.undo());
        assert_eq!(doc.rope().snapshot().text(), "a b c");
    }

    #[test]
    fn matching_delimiter_finds_the_partner_across_nesting() {
        let rope = TokenRope::from_text("{ a { b } c }", &Limits::default()).expect("rope");
        let tokens: Vec<_> = rope.snapshot().tokens().iter().map(|t| t.kind).collect();
        let outer_open = tokens.iter().position(|k| *k == TokenKind::LBrace).unwrap();
        let outer_close = matching_delimiter(&rope, outer_open).expect("match");
        assert_eq!(tokens[outer_close], TokenKind::RBrace);
        // The outer closer must be the LAST brace, not the inner one.
        let last_close = tokens.iter().rposition(|k| *k == TokenKind::RBrace).unwrap();
        assert_eq!(outer_close, last_close);
    }

    #[test]
    fn apply_rejects_stale_revision_and_leaves_history_untouched() {
        let mut doc = EditorDocument::from_rope(sample_rope());
        let stale = TokenTransaction { base_revision: 999, patches: vec![] };
        assert!(doc.apply(&stale).is_err());
        assert!(!doc.can_undo());
    }
}
//#endregion 🧪Tests
