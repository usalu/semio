#!/usr/bin/env python3
"""🏷️ U6 window-3 set C3 (stdio, wav): editing the data `kind` switches the sample format instead of failing.

Measured 03:24 (`.🧬semio/🌐hub/s14-u6-logs/a3-check-test.txt`, `data_kind_edit_publishes_the_requested_variant_and_reopens_natively`):
a details-panel edit of `/data/kind` falls through the wav editor's own arms (`/data`, `/data/value[/i]`) into the generic snapshot
patch, whose in-place path editor (`#[derive(FromValue)] edit_value_at_path`) refuses by design: "cannot edit an enum tag in place"
→ `snapshot-edit.path-invalid`. `WavData` is adjacently tagged (`kind` + `value`); a tag edit is a variant switch, which is a typed
replacement of `data`: the editor rebuilds `{kind, value}` from the current samples and decodes it as the requested kind, so every
sample must fit the new kind or the edit is refused typed (`stdio.wav.invalid-data`) — the same value-level semantics the `/data`
arm already has, scoped to the data payload (never the whole document). The emit path computed its expected document through the
same generic patch, so the stdio editing contract gains `SnapshotEditingEditor::snapshot_edit_expected` (default = the generic patch,
now the pub `generic_snapshot_edit_expected`; every other editor unchanged) and wav answers `/data/kind` with its typed retag. The
existing law `data_kind_edit_publishes_the_requested_variant_and_reopens_natively` is the test.

Usage: u6-wav-kind-retag.py [--dry-run | --write | --revert] [--root <repo root>]"""
import hashlib
import sys
from pathlib import Path

ROOT = Path(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else Path("/Users/ueli/Documents/semio")
WRITE = "--write" in sys.argv
REVERT = "--revert" in sys.argv
BACKUP = Path("/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-u6-backup/wav-kind-retag")
EDITOR = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/✏️editor/🦀️.rs"

ARM_OLD = """        if path == "/data/value" {
            let mut next = snapshot.clone();
"""
ARM_NEW = """        if path == "/data/kind" {
            let mut next = snapshot.clone();
            next.data = wavEditor_retag_data(&snapshot.data, value)?;
            return Ok(next);
        }
        if path == "/data/value" {
            let mut next = snapshot.clone();
"""
FN_OLD = "fn wavEditor_snapshot_edit(event: &editing::SnapshotEditEvent, snapshot: &WavSnapshot) -> Result<WavSnapshot, Fault> {\n"
FN_NEW = """/// 🏷️ A `kind` edit switches the sample format: `{kind, value}` is rebuilt from the current samples and decoded as the requested
/// kind, so each sample must fit it — the generic in-place path editor refuses an enum tag by design.
fn wavEditor_retag_data(data: &WavData, kind: &dsl::DslValue) -> Result<WavData, Fault> {
    let samples = match data {
        WavData::Pcm16(samples) => dsl::ToValue::to_value(samples),
        WavData::Pcm8(samples) | WavData::Raw(samples) => dsl::ToValue::to_value(samples),
        WavData::Float32(samples) => dsl::ToValue::to_value(samples),
    };
    <WavData as dsl::FromValue>::from_value(dsl::DslValue::Object(vec![("kind".into(), kind.clone()), ("value".into(), samples)])).map_err(|error| wavEditor_edit_fault("stdio.wav.invalid-data", error.to_string()))
}

""" + FN_OLD
CONTRACT = "✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🦀️.rs"
EMIT_OLD = """    fn snapshot_edit_emit(event: &SnapshotEditEvent, snapshot: &Self::Snapshot) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, Fault> {
        let patch = prepare_snapshot_patch(snapshot, event).map_err(|error| edit_fault(error.code, error.to_string()))?;
        let expected = apply_snapshot_patch_for_dialect(snapshot, &patch, Self::DIALECT, Self::DOCUMENT_SCHEMA).map_err(|error| edit_fault(error.code, error.to_string()))?;
        if &expected == snapshot {
"""
EMIT_NEW = """    /// 🎯️ The document an edit must publish, which the emitted mutations are checked against: the generic in-place snapshot
    /// patch. An editor whose document has typed edits the generic path refuses by design (switching a tagged union's variant)
    /// answers those itself and delegates every other edit to [`generic_snapshot_edit_expected`].
    fn snapshot_edit_expected(event: &SnapshotEditEvent, snapshot: &Self::Snapshot) -> Result<Self::Snapshot, Fault> {
        generic_snapshot_edit_expected::<Self>(event, snapshot)
    }

    fn snapshot_edit_emit(event: &SnapshotEditEvent, snapshot: &Self::Snapshot) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, Fault> {
        let expected = Self::snapshot_edit_expected(event, snapshot)?;
        if &expected == snapshot {
"""
FREE_OLD = """        Ok(emit)
    }
}

/// 🧵️ Supplies native mutations to the same retained, cancelable execution lane as snapshot edits.
"""
FREE_NEW = """        Ok(emit)
    }
}

/// 🎯️ The generic in-place snapshot patch of one edit — the default of [`SnapshotEditingEditor::snapshot_edit_expected`].
pub fn generic_snapshot_edit_expected<E: SnapshotEditingEditor>(event: &SnapshotEditEvent, snapshot: &E::Snapshot) -> Result<E::Snapshot, Fault> {
    let patch = prepare_snapshot_patch(snapshot, event).map_err(|error| edit_fault(error.code, error.to_string()))?;
    apply_snapshot_patch_for_dialect(snapshot, &patch, E::DIALECT, E::DOCUMENT_SCHEMA).map_err(|error| edit_fault(error.code, error.to_string()))
}

/// 🧵️ Supplies native mutations to the same retained, cancelable execution lane as snapshot edits.
"""
IMPL_OLD = """    fn snapshot_edit_event(command: &Self::Command) -> Option<&editing::SnapshotEditEvent> {
        match command { WavEditCommand::EditSnapshot { event } => Some(event), _ => None }
    }
"""
IMPL_NEW = IMPL_OLD + """    fn snapshot_edit_expected(event: &editing::SnapshotEditEvent, snapshot: &Self::Snapshot) -> Result<Self::Snapshot, Fault> {
        match event {
            editing::SnapshotEditEvent::SetValue { path, .. } if path == "/data/kind" => wavEditor_snapshot_edit(event, snapshot),
            _ => editing::generic_snapshot_edit_expected::<Self>(event, snapshot),
        }
    }
"""
SETS = {CONTRACT: [(EMIT_OLD, EMIT_NEW, 1), (FREE_OLD, FREE_NEW, 1)], EDITOR: [(ARM_OLD, ARM_NEW, 1), (FN_OLD, FN_NEW, 1), (IMPL_OLD, IMPL_NEW, 1)]}


def key(rel: str) -> str:
    return hashlib.sha256(rel.encode()).hexdigest()[:16]


def main() -> int:
    if REVERT:
        for rel in SETS:
            backup = BACKUP / key(rel)
            if backup.exists():
                (ROOT / rel).write_bytes(backup.read_bytes())
                backup.unlink()
                print(f"REVERTED {rel}")
        return 0
    problems, planned = 0, []
    for rel, hunks in SETS.items():
        path = ROOT / rel
        text = path.read_text()
        for old, new, count in hunks:
            found = text.count(old)
            if found != count:
                print(f"PROBLEM {rel}: {'already applied' if new in text else f'anchor count {found} != {count}'}: {old[:70]!r}")
                problems += 1
                continue
            text = text.replace(old, new)
        planned.append((rel, path, text))
        print(f"{'WRITE' if WRITE else 'DRY'} {rel.rsplit('/🪆️subsets/', 1)[-1]}: {len(hunks)} hunks")
    if WRITE and problems == 0:
        BACKUP.mkdir(parents=True, exist_ok=True)
        for rel, path, text in planned:
            (BACKUP / key(rel)).write_bytes(path.read_bytes())
            path.write_text(text)
    print(f"{'write' if WRITE and problems == 0 else 'dry-run'}: {len(SETS)} files, {problems} problems")
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main())
