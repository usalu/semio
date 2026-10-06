#!/usr/bin/env python3
"""🧯️ S5-STORE wave P2 (design §22.5): a Report replay records a leaf's refused inverse as one `Fatal`
`mutation.inverse-refused` message on that mutation and keeps replaying; a Merge replay still refuses with
`VcsError::InverseRefused`. The operation folds forward exactly as every messageless fold site folds it, and its edit stages
no inverse rows for it.

Precondition: the outcome vocabulary holds `mutation.inverse-refused` at `Fatal` (S5-PUZZLE's vocabulary wave) — persistence
refuses a message whose code it does not know. The script refuses to apply before that.

The edit is keyed on an anchor that must occur exactly once. Idempotent. `--check` prints what is pending and writes nothing;
`--emit <dir>` writes the edited file into `<dir>` (for a parse check) and leaves the tree alone.

    python3 🧪️s5-store-inverse-refused.py [--check | --emit <dir>]
"""
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[7]
STORE = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs"
VOCABULARY = ROOT / "🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs"
CODE = "mutation.inverse-refused"

OLD = '''    fn replay_operation(&mut self, edit: &Edit<Mutation>, edit_ids: &[MutationId]) -> Result<(), VcsError> {
        let index = self.operation;
        let mutation_id = edit_ids.get(index).cloned().unwrap_or_else(|| MutationId(format!("{}#{index}", edit.id)));
        let effective = effective_operation::<P, Mutation>(&edit.forwards[index], index as u32, mutation_id, &self.schema, &self.supersessions);
        let mut messages: Vec<crate::os_spr::MutationMessage> = effective.fault().cloned().into_iter().collect();
        if let Some(operation) = effective.operation() {
            let base = match self.mode {
                ReplayMode::Merge(_) => self.candidate.as_ref().or(self.state.as_ref()),
                ReplayMode::Report => self.state.as_ref(),
            }
            .ok_or_else(|| VcsError::ValidationFailed("edit replay lost its running projection".into()))?;
            let mut back = operation.inverse(base).map_err(VcsError::InverseRefused)?;
            back.reverse();
            let next = match self.mode {
                ReplayMode::Report => {
                    let (next, operation_messages) = fold_operation::<P, Mutation>(base, operation, index as u32);
                    messages = operation_messages;
                    next
                }
                ReplayMode::Merge(_) => {
                    let (diff, operation_messages) = operation.diff(base).stamp_op_index(index as u32).into_parts();
                    let applied = diff.apply(base);
                    <Mutation::Diff as MutationDiff<P>>::retire_cold(diff);
                    messages = operation_messages;
                    match applied {
                        Ok(next) => Some(next),
                        Err(error) => {
                            retire_scratch_operations::<P, Mutation>(back);
                            return Err(error.into());
                        }
                    }
                }
            };
            match next {
'''

NEW = '''    /// 🫀️ Replays operation `self.operation` of `edit`: its effective input is re-diffed and re-inverted against the running
    /// state and its outcome recorded. A `Report` replay never aborts on a leaf: an inverse the leaf refuses on the replayed
    /// state is one `Fatal` `mutation.inverse-refused` message on that mutation, the operation folds forward exactly as every
    /// messageless fold site folds it, and its edit stages no inverse rows for it — the report blocks finalizing until that
    /// mutation is edited or withdrawn. A `Merge` replay refuses with [`VcsError::InverseRefused`].
    fn replay_operation(&mut self, edit: &Edit<Mutation>, edit_ids: &[MutationId]) -> Result<(), VcsError> {
        let index = self.operation;
        let mutation_id = edit_ids.get(index).cloned().unwrap_or_else(|| MutationId(format!("{}#{index}", edit.id)));
        let effective = effective_operation::<P, Mutation>(&edit.forwards[index], index as u32, mutation_id, &self.schema, &self.supersessions);
        let mut messages: Vec<crate::os_spr::MutationMessage> = effective.fault().cloned().into_iter().collect();
        if let Some(operation) = effective.operation() {
            let base = match self.mode {
                ReplayMode::Merge(_) => self.candidate.as_ref().or(self.state.as_ref()),
                ReplayMode::Report => self.state.as_ref(),
            }
            .ok_or_else(|| VcsError::ValidationFailed("edit replay lost its running projection".into()))?;
            let (back, refused) = match operation.inverse(base) {
                Ok(mut back) => {
                    back.reverse();
                    (back, None)
                }
                Err(cause) if self.mode == ReplayMode::Report => (Vec::new(), Some(cause)),
                Err(cause) => return Err(VcsError::InverseRefused(cause)),
            };
            let next = match self.mode {
                ReplayMode::Report => {
                    let (next, operation_messages) = fold_operation::<P, Mutation>(base, operation, index as u32);
                    messages = operation_messages;
                    next
                }
                ReplayMode::Merge(_) => {
                    let (diff, operation_messages) = operation.diff(base).stamp_op_index(index as u32).into_parts();
                    let applied = diff.apply(base);
                    <Mutation::Diff as MutationDiff<P>>::retire_cold(diff);
                    messages = operation_messages;
                    match applied {
                        Ok(next) => Some(next),
                        Err(error) => {
                            retire_scratch_operations::<P, Mutation>(back);
                            return Err(error.into());
                        }
                    }
                }
            };
            if let Some(cause) = refused {
                let mut fatal = crate::os_spr::MutationMessage::fatal("mutation.inverse-refused", format!("the operation has no inverse on the replayed state: {cause}")).at([effective.mutation_id.0.clone()]);
                fatal.op_index = Some(index as u32);
                messages.push(fatal);
            }
            match next {
'''

MARKER = "                Err(cause) if self.mode == ReplayMode::Report => (Vec::new(), Some(cause)),\n"


def main():
    text = STORE.read_text(encoding="utf-8")
    if MARKER in text:
        print("pending: none")
        return
    if text.count(OLD) != 1:
        raise SystemExit(f"anchor `replay_operation` occurs {text.count(OLD)} times (expected 1): re-derive the wave")
    print("pending: store:replay_operation")
    edited = text.replace(OLD, NEW)
    if "--emit" in sys.argv[1:]:
        target = Path(sys.argv[sys.argv.index("--emit") + 1])
        target.mkdir(parents=True, exist_ok=True)
        (target / "store.rs").write_text(edited, encoding="utf-8")
        print(f"emitted to {target}")
        return
    if "--check" in sys.argv[1:]:
        return
    if f'"{CODE}"' not in VOCABULARY.read_text(encoding="utf-8"):
        raise SystemExit(f"the outcome vocabulary does not hold {CODE} yet (S5-PUZZLE's wave): nothing applied")
    STORE.write_text(edited, encoding="utf-8")
    print("applied")


main()
