#!/usr/bin/env python3
"""🧷️ S5-STORE wave FW (design §22.1, coordinator decision 2026-10-05): `Withdrawn` is admitted for every operation, an
operation that plans foreign steps takes no input but its own recorded one, the store answers `unit_id` / `unit_operations`,
and a supersession naming an operation of a cross-artifact unit is refused with `VcsError::UnitSpansDocuments`
(`history.unit-spans-documents`).

Every edit is keyed on an anchor that must occur exactly once; both files are written only when every anchor resolved, so
the wave is compile-atomic. Idempotent. `--check` prints what is pending and writes nothing; `--emit <dir>` writes the
edited files into `<dir>` (for a parse check) and leaves the tree alone.

    python3 🧪️s5-store-foreign-withdraw.py [--check | --emit <dir>]
"""
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[7]
MODULES = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules"
STORE = MODULES / "🏪️store/🦀️.rs"
VCS = MODULES / "🌿️vcs/🦀️.rs"

VCS_EDITS = [
    (
        "variant",
        "    TooLarge { rows: usize, capacity: usize },\n}\n",
        "    TooLarge { rows: usize, capacity: usize },\n"
        "    /// 🧷️ The operation `mutation_id` belongs to the cross-artifact unit `unit`: it planned steps in other documents, or it is\n"
        "    /// such a step. A unit is superseded in all of its documents at once, which no replica can do yet, so the supersession\n"
        "    /// was refused before anything was recorded.\n"
        "    UnitSpansDocuments { mutation_id: String, unit: String },\n}\n",
    ),
    (
        "display",
        '            Self::TooLarge { rows, capacity } => write!(formatter, "the edit needs {rows} staged rows where at most {capacity} are admitted"),\n',
        '            Self::TooLarge { rows, capacity } => write!(formatter, "the edit needs {rows} staged rows where at most {capacity} are admitted"),\n'
        '            Self::UnitSpansDocuments { mutation_id, unit } => write!(formatter, "operation {mutation_id} belongs to the cross-artifact unit {unit}, which one document alone cannot supersede"),\n',
    ),
    (
        "code",
        '            Self::TooLarge { .. } => "mutation.too-large",\n',
        '            Self::TooLarge { .. } => "mutation.too-large",\n'
        '            Self::UnitSpansDocuments { .. } => "history.unit-spans-documents",\n',
    ),
    (
        "code-doc",
        "/// `history.replaying` and `mutation.too-large`.\n",
        "/// `history.replaying`, `mutation.too-large` and `history.unit-spans-documents`.\n",
    ),
]

OLD_LAW = '''/// 🛂️ The supersede law every input obeys — authored, ingested, loaded or folded: `original` plans no foreign steps, and
/// a replacement is an operation of `schema` in its canonical encoding planning no foreign steps. Answers the decoded
/// replacement (`None` for a withdrawal) or why the input breaks the law.
pub fn admit_replacement<P, Mutation>(original: &Mutation, replacement: &protocol::InputReplacement, schema: &str) -> Result<Option<Mutation>, String>
where
    Mutation: self::Mutation<P> + OpBinary,
{
    if original.may_emit_foreign_steps() {
        return Err("targets an operation that plans foreign steps".to_string());
    }
    let protocol::InputReplacement::Input { schema: named, payload } = replacement else { return Ok(None) };
    if named != schema {
        return Err(format!("names schema {named}, not {schema}"));
    }
    let decoded = Mutation::decode_op(payload).map_err(|error| format!("does not decode: {error}"))?;
    let refusal = match decoded.encode_op() {
        Err(error) => Some(format!("does not re-encode: {error}")),
        Ok(recoded) if recoded != *payload => Some("is not canonically encoded".to_string()),
        Ok(_) if decoded.may_emit_foreign_steps() => Some("plans foreign steps".to_string()),
        Ok(_) => None,
    };
'''

NEW_LAW = '''/// 🛂️ The supersede law every input obeys — authored, ingested, loaded or folded. A withdrawal is admitted for every
/// operation. A replacement is an operation of `schema` in its canonical encoding that plans no foreign steps; an operation
/// that plans foreign steps takes no replacement but its own recorded input — what the undo of its withdrawal authors —
/// because any other input would plan steps the other documents never received. Answers the decoded replacement (`None` for
/// a withdrawal) or why the input breaks the law.
pub fn admit_replacement<P, Mutation>(original: &Mutation, replacement: &protocol::InputReplacement, schema: &str) -> Result<Option<Mutation>, String>
where
    Mutation: self::Mutation<P> + OpBinary,
{
    let protocol::InputReplacement::Input { schema: named, payload } = replacement else { return Ok(None) };
    if named != schema {
        return Err(format!("names schema {named}, not {schema}"));
    }
    let decoded = Mutation::decode_op(payload).map_err(|error| format!("does not decode: {error}"))?;
    let refusal = match decoded.encode_op() {
        Err(error) => Some(format!("does not re-encode: {error}")),
        Ok(recoded) if recoded != *payload => Some("is not canonically encoded".to_string()),
        Ok(recoded) if original.may_emit_foreign_steps() => (!original.encode_op().is_ok_and(|recorded| recorded == recoded)).then(|| "replaces the input of an operation that plans foreign steps".to_string()),
        Ok(_) if decoded.may_emit_foreign_steps() => Some("plans foreign steps".to_string()),
        Ok(_) => None,
    };
'''

READS_ANCHOR = "    /// 🕰️ The projection right before operation `mutation_id` with `drafts` laid over the effective inputs — nothing\n"

NEW_READS = '''    /// 🫂️ The cross-artifact unit `mutation_id` belongs to; `None` for an operation that is its own unit. An operation that
    /// plans foreign steps and whose plan on the projection right before it is not empty changed other documents too, and an
    /// operation of `Transaction` origin is such a step of another document's operation. The unit is named by the gesture's
    /// group where this replica holds one, else by the operation. A unit is superseded in all of its documents at once,
    /// which no replica can do yet ([`VcsError::UnitSpansDocuments`]).
    pub fn unit_id(&self, mutation_id: &MutationId) -> Result<Option<String>, VcsError> {
        let (position, index) = self.locate_mutation(mutation_id)?;
        let edit = self.applied_edit(position)?;
        let meta = edit.mutation_meta.get(index);
        let spans = matches!(meta.map(|meta| &meta.origin), Some(crate::os_spr::MutationOrigin::Transaction { .. })) || (edit.forwards[index].may_emit_foreign_steps() && self.plans_foreign_steps(position, index)?);
        Ok(spans.then(|| meta.and_then(|meta| meta.group_id.clone()).unwrap_or_else(|| mutation_id.0.clone())))
    }

    /// 🪬️ This store's part of the unit of `mutation_id`, in operation order: every operation its edit recorded under the
    /// unit's group, or the operation alone when it is its own unit ([`Self::unit_id`]).
    pub fn unit_operations(&self, mutation_id: &MutationId) -> Result<Vec<MutationId>, VcsError> {
        let (position, index) = self.locate_mutation(mutation_id)?;
        let edit = self.applied_edit(position)?;
        let group = edit.mutation_meta.get(index).and_then(|meta| meta.group_id.as_ref());
        if group.is_none() || self.unit_id(mutation_id)?.is_none() {
            return Ok(vec![mutation_id.clone()]);
        }
        Ok(crate::os_spr::mutation_ids_for_edit::<P, Mutation>(edit).into_iter().enumerate().filter(|(member, _)| edit.mutation_meta.get(*member).and_then(|meta| meta.group_id.as_ref()) == group).map(|(_, member)| member).collect())
    }

    /// 🫴️ Whether operation `index` of the applied edit at `position` plans a foreign step on the projection right before
    /// it. Reads through the prefix ring and retains nothing.
    fn plans_foreign_steps(&self, position: usize, index: usize) -> Result<bool, VcsError> {
        let applied: Vec<String> = self.applied_edit_ids.to_vec();
        let (base, recorded) = self.prefix_state_recorded(&applied, position, self.supersessions(), Self::prefix_stride(applied.len()))?;
        for (_, snapshot) in recorded {
            retire_shared_projection::<P, Mutation>(snapshot);
        }
        let mut running = base;
        let planned = match self.envelope.vcs.edits.find_near(|edit| edit.id == applied[position]) {
            Some(edit) => {
                fold_effective_edit(&mut running, edit, &self.envelope.schema, self.supersessions(), index);
                Ok(!edit.forwards[index].foreign_steps(&running).is_empty())
            }
            None => Err(VcsError::UnknownEdit(applied[position].clone())),
        };
        retire_shared_projection::<P, Mutation>(running);
        planned
    }

'''

OLD_INPUT = '''    /// 🧩️ One supersede input: its encoded replacement, held to the supersede law ([`admit_replacement`]) every replica
    /// folds it under, and the address it and the original operation share.
    fn supersede_input(&self, target: &MutationId, original: &Mutation, replacement: Option<&Mutation>) -> Result<(protocol::SupersededInput, Vec<String>), VcsError> {
        let replacement = match replacement {
'''

NEW_INPUT = '''    /// 🧩️ One supersede input: its encoded replacement, held to the supersede law ([`admit_replacement`]) every replica
    /// folds it under, and the address it and the original operation share. An operation of a cross-artifact unit takes no
    /// supersession in one document alone ([`Self::unit_id`]).
    fn supersede_input(&self, target: &MutationId, original: &Mutation, replacement: Option<&Mutation>) -> Result<(protocol::SupersededInput, Vec<String>), VcsError> {
        if let Some(unit) = self.unit_id(target)? {
            return Err(VcsError::UnitSpansDocuments { mutation_id: target.0.clone(), unit });
        }
        let replacement = match replacement {
'''

STORE_EDITS = [("law", OLD_LAW, NEW_LAW), ("reads", READS_ANCHOR, NEW_READS + READS_ANCHOR), ("input", OLD_INPUT, NEW_INPUT)]


def plan(text, edits, marker_of):
    """🧮️ The edited text and the names of the edits still pending; an anchor that is neither present once nor already applied aborts."""
    pending = []
    for name, old, new in edits:
        if marker_of(name, new) in text:
            continue
        if text.count(old) != 1:
            raise SystemExit(f"anchor `{name}` occurs {text.count(old)} times (expected 1): re-derive the wave")
        text = text.replace(old, new)
        pending.append(name)
    return text, pending


def marker(name, new):
    """🔖️ A line only the applied edit `name` contains."""
    return {
        "variant": "    UnitSpansDocuments { mutation_id: String, unit: String },\n",
        "display": "            Self::UnitSpansDocuments { mutation_id, unit } => write!(",
        "code": '            Self::UnitSpansDocuments { .. } => "history.unit-spans-documents",\n',
        "code-doc": "`mutation.too-large` and `history.unit-spans-documents`.\n",
        "law": '"replaces the input of an operation that plans foreign steps"',
        "reads": "    pub fn unit_id(&self, mutation_id: &MutationId) -> Result<Option<String>, VcsError> {\n",
        "input": "            return Err(VcsError::UnitSpansDocuments { mutation_id: target.0.clone(), unit });\n",
    }[name]


def main():
    check = "--check" in sys.argv[1:]
    store, store_pending = plan(STORE.read_text(encoding="utf-8"), STORE_EDITS, marker)
    vcs, vcs_pending = plan(VCS.read_text(encoding="utf-8"), VCS_EDITS, marker)
    pending = [f"vcs:{name}" for name in vcs_pending] + [f"store:{name}" for name in store_pending]
    print("pending: " + (", ".join(pending) if pending else "none"))
    if "--emit" in sys.argv[1:]:
        target = Path(sys.argv[sys.argv.index("--emit") + 1])
        target.mkdir(parents=True, exist_ok=True)
        (target / "vcs.rs").write_text(vcs, encoding="utf-8")
        (target / "store.rs").write_text(store, encoding="utf-8")
        print(f"emitted to {target}")
        return
    if check or not pending:
        return
    VCS.write_text(vcs, encoding="utf-8")
    STORE.write_text(store, encoding="utf-8")
    print("applied")


main()
