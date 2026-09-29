#!/usr/bin/env python3
"""🪜️ U6 window-3 set (T2, stdio): txt `replace-text` must never publish a half-applied document.

Measured 01:55 (`.🧬semio/🌐hub/s14-u6-logs/a2-check-test.txt`): `txt_emit` lowered a replacement to "remove every line
(last→first), insert every line". Removing the last remaining line of a TERMINATED document passes through `[]` + terminator,
which `remove-line` refuses (`native_shape_error`: "a document with no lines cannot carry a trailing terminator"), and the leaf
answered `MutationOutcome::error("mutation.invariant", …)` — an ERROR carries an empty diff that the bounded preparation applies,
so the store published the old first line plus the new text (`["alpha","beta","Hello, stdio.txt!"]`). Two faults, one set:

* LOWERING: collapse to the neutral pivot `[""]` + terminator (valid for LF and CRLF, empty text is valid everywhere), switch the
  line ending there, then build the target; an unterminated edge line is carried as a one-space placeholder until the terminator
  state matches. Every intermediate document is a native shape by construction; an unrepresentable target is refused typed
  (`stdio.txt.replace-text-unrepresentable`) before any mutation exists.
* SEVERITY: every txt leaf refused an invariant with `error`; the store's own contract (`expected_mutation_message_level`,
  `🏪️store`) says `mutation.invariant` is FATAL, and only a Fatal outcome makes the bounded preparation refuse the whole edit
  (`bounded-store.…-fatal-mutation`) instead of journaling an empty diff → all 14 sites `fatal`.
* LAW (test-only, txt editor unit tests): every ordered pair of a 19-body corpus (LF/CRLF, terminated/unterminated, empty, bare
  CR, blank lines) lowers to mutations that each apply message-free and end exactly at `TxtSnapshot::from_body(new)`; the body
  round trip `to_body()` is the oracle.

Usage: u6-txt-replace-lowering.py [--dry-run | --write | --revert] [--root <repo root>]"""
import hashlib
import sys
from pathlib import Path

ROOT = Path(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else Path("/Users/ueli/Documents/semio")
WRITE = "--write" in sys.argv
REVERT = "--revert" in sys.argv
BACKUP = Path("/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-u6-backup/txt-replace-lowering")
TXT = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/"
EDITOR = TXT + "✏️editor/🦀️.rs"
EDITOR_TEST = TXT + "✏️editor/🧪️tests/🔬️unit/🦀️.rs"
ERROR = 'protocol::MutationOutcome::error("mutation.invariant", reason, Vec::<String>::new());'
FATAL = 'protocol::MutationOutcome::fatal("mutation.invariant", reason, Vec::<String>::new());'

OLD_DOC = """//! (`TextWindowKit`), replacing the document through the direct line, line-ending, and trailing-newline mutations
//! — a `replace-text` command is inherently whole-buffer, so per-line `InsertLine`/`SetLine` are not
//! reachable through this window (documented, not silently dropped: a future line-addressable editor
//! could target those directly).
"""
NEW_DOC = """//! (`TextWindowKit`), replacing the whole buffer through the direct line, line-ending, and trailing-newline mutations
//! (`txt_replacement_mutations`: every intermediate document stays a native shape).
"""
OLD_USE = """use crate::schema::mutation_support::txt_usize_to_u32;
use crate::schema::mutations::{InsertLineMutation, RemoveLineMutation, SetLineEndingMutation, SetTrailingNewlineMutation};
"""
NEW_USE = """use crate::schema::mutation_support::{native_snapshot_error, txt_usize_to_u32};
use crate::schema::mutations::{InsertLineMutation, RemoveLineMutation, SetLineEndingMutation, SetLineMutation, SetTrailingNewlineMutation};
"""
OLD_EMIT = """    let mut next = TxtSnapshot::from_body(text);
    next.schema.clone_from(&snapshot.schema);
    if &next == snapshot {
        return Ok(Emit::default());
    }
    let mut mutations = Vec::new();
    for index in (0..snapshot.lines.len()).rev() {
        let index = txt_usize_to_u32(index).map_err(|detail| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("txt.mutation.index-out-of-range"), detail))?;
        mutations.push(TxtMutation::RemoveLine(RemoveLineMutation { index }));
    }
    for (index, text) in next.lines.iter().cloned().enumerate() {
        let index = txt_usize_to_u32(index).map_err(|detail| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("txt.mutation.index-out-of-range"), detail))?;
        mutations.push(TxtMutation::InsertLine(InsertLineMutation { index, text }));
    }
    if next.trailing_newline != snapshot.trailing_newline {
        mutations.push(TxtMutation::SetTrailingNewline(SetTrailingNewlineMutation { value: next.trailing_newline }));
    }
    if next.line_ending != snapshot.line_ending {
        mutations.push(TxtMutation::SetLineEnding(SetLineEndingMutation { value: next.line_ending }));
    }
    Ok(Emit { artifact_mutations: mutations, description: Some("Replace text".into()), ..Default::default() })
}
"""
NEW_EMIT = """    let mut next = TxtSnapshot::from_body(text);
    next.schema.clone_from(&snapshot.schema);
    if &next == snapshot {
        return Ok(Emit::default());
    }
    if let Some(reason) = native_snapshot_error(&next) {
        return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.txt.replace-text-unrepresentable"), reason));
    }
    Ok(Emit { artifact_mutations: txt_replacement_mutations(snapshot, &next)?, description: Some("Replace text".into()), ..Default::default() })
}

/// 🪜️ The placeholder an unterminated edge line carries while the document is terminated: non-empty, free of CR and LF, so
/// it is a native line under both line endings whether or not a separator follows it.
const REPLACEMENT_PLACEHOLDER_LINE: &str = " ";

/// 🪜️ Lowers a whole-buffer replacement of a native `snapshot` by a native `next` into line mutations whose EVERY intermediate
/// document is a native shape, so no leaf ever refuses mid-edit: collapse to the neutral pivot `[""]` + terminator (valid for LF
/// and CRLF), switch the line ending there, then build `next`, carrying an unterminated edge line as the placeholder until the
/// terminator state matches. Emits `O(old + new)` mutations and never a no-op.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn txt_replacement_mutations(snapshot: &TxtSnapshot, next: &TxtSnapshot) -> Result<Vec<TxtMutation>, Fault> {
    let index = |line: usize| txt_usize_to_u32(line).map_err(|detail| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("txt.mutation.index-out-of-range"), detail));
    let mut mutations = Vec::new();
    let mut first = snapshot.lines.first().cloned().unwrap_or_default();
    if snapshot.lines.is_empty() {
        mutations.push(TxtMutation::InsertLine(InsertLineMutation { index: 0, text: REPLACEMENT_PLACEHOLDER_LINE.into() }));
        first = REPLACEMENT_PLACEHOLDER_LINE.into();
    } else if !snapshot.trailing_newline {
        let last = snapshot.lines.len() - 1;
        if snapshot.lines[last] != REPLACEMENT_PLACEHOLDER_LINE {
            mutations.push(TxtMutation::SetLine(SetLineMutation { index: index(last)?, text: REPLACEMENT_PLACEHOLDER_LINE.into() }));
        }
        if last == 0 {
            first = REPLACEMENT_PLACEHOLDER_LINE.into();
        }
    }
    if !snapshot.trailing_newline {
        mutations.push(TxtMutation::SetTrailingNewline(SetTrailingNewlineMutation { value: true }));
    }
    for line in (1..snapshot.lines.len()).rev() {
        mutations.push(TxtMutation::RemoveLine(RemoveLineMutation { index: index(line)? }));
    }
    if next.line_ending != snapshot.line_ending {
        if !first.is_empty() {
            mutations.push(TxtMutation::SetLine(SetLineMutation { index: 0, text: String::new() }));
            first.clear();
        }
        mutations.push(TxtMutation::SetLineEnding(SetLineEndingMutation { value: next.line_ending }));
    }
    let Some(last) = next.lines.len().checked_sub(1) else {
        if first != REPLACEMENT_PLACEHOLDER_LINE {
            mutations.push(TxtMutation::SetLine(SetLineMutation { index: 0, text: REPLACEMENT_PLACEHOLDER_LINE.into() }));
        }
        mutations.push(TxtMutation::SetTrailingNewline(SetTrailingNewlineMutation { value: false }));
        mutations.push(TxtMutation::RemoveLine(RemoveLineMutation { index: 0 }));
        return Ok(mutations);
    };
    let staged = |line: usize| if line == last && !next.trailing_newline { REPLACEMENT_PLACEHOLDER_LINE } else { next.lines[line].as_str() };
    if first != staged(0) {
        mutations.push(TxtMutation::SetLine(SetLineMutation { index: 0, text: staged(0).into() }));
    }
    for line in 1..next.lines.len() {
        mutations.push(TxtMutation::InsertLine(InsertLineMutation { index: index(line)?, text: staged(line).into() }));
    }
    if !next.trailing_newline {
        mutations.push(TxtMutation::SetTrailingNewline(SetTrailingNewlineMutation { value: false }));
        if next.lines[last] != REPLACEMENT_PLACEHOLDER_LINE {
            mutations.push(TxtMutation::SetLine(SetLineMutation { index: index(last)?, text: next.lines[last].clone() }));
        }
    }
    Ok(mutations)
}
"""
LAW_ANCHOR = """//#region 🎬️ExampleSwitchLaws
"""
LAW = """/// ⚖️ LAW: a whole-buffer replacement between ANY two native documents lowers to mutations that each apply without a message
/// (no refused leaf, no empty diff journaled) and end exactly at the replacement — the text round trip `to_body()` is the
/// oracle. The corpus crosses LF/CRLF, terminated/unterminated, empty, blank-line and bare-CR bodies.
#[test]
fn every_replacement_lowers_through_native_documents_only() {
    const BODIES: [&str; 19] = ["", "a", "a\\n", "a\\r\\n", "\\n", "\\r\\n", "a\\nb", "a\\nb\\n", "a\\r\\nb", "a\\r\\r\\nb\\r\\n", "x\\r\\r\\ny\\r\\n", "a\\r", "a\\n\\n", "\\r\\n\\r\\n", "alpha\\nbeta", "Hello, stdio.txt!\\n", " ", " \\n", "\\r"];
    for old_body in BODIES {
        let snapshot = TxtSnapshot::from_body(old_body);
        assert_eq!(native_snapshot_error(&snapshot), None, "corpus body {old_body:?} is native");
        let revision = semio_s_artifact_stdio_contract::window_kit_snapshot_revision(&snapshot);
        for new_body in BODIES {
            let emit = txt_emit(&TxtEditorCommand::ReplaceText { revision: revision.clone(), text: new_body.into() }, &snapshot, None).expect("native replacement lowers");
            let mut next = snapshot.clone();
            for mutation in &emit.artifact_mutations {
                let outcome = <TxtMutation as protocol::Mutation<TxtSnapshot>>::diff(mutation, &next);
                assert!(outcome.messages().is_empty(), "{old_body:?} → {new_body:?}: {mutation:?} refused: {:?}", outcome.messages());
                next = protocol::MutationDiff::apply(outcome.diff(), &next).expect("native mutation applies");
            }
            assert_eq!(next.to_body(), new_body, "{old_body:?} → {new_body:?}");
            let mut expected = TxtSnapshot::from_body(new_body);
            expected.schema.clone_from(&snapshot.schema);
            assert_eq!(next, expected, "{old_body:?} → {new_body:?}");
        }
    }
}

/// ⚖️ LAW: an invariant refusal is FATAL (the store's contract for `mutation.invariant`), so the bounded preparation refuses the
/// whole edit instead of journaling the leaf's empty diff.
#[test]
fn invariant_refusals_are_fatal() {
    let terminated = TxtSnapshot::from_body("only\\n");
    let outcome = <TxtMutation as protocol::Mutation<TxtSnapshot>>::diff(&TxtMutation::RemoveLine(RemoveLineMutation { index: 0 }), &terminated);
    assert_eq!(outcome.messages().iter().map(|message| (message.code.0.as_str(), message.level)).collect::<Vec<_>>(), vec![("mutation.invariant", protocol::Severity::Fatal)]);
    assert_eq!(outcome.diff(), &Default::default());
}

"""


def key(rel: str) -> str:
    return hashlib.sha256(rel.encode()).hexdigest()[:16]


LEAVES = [TXT + f"🧬️schema/🧬️mutations/{leaf}/🦀️.rs" for leaf in ["📥️insert-line", "✏️set-line", "🔚️set-line-ending", "🗑️remove-line", "↩️set-trailing-newline"]]
COUNTS = {LEAVES[0]: 4, LEAVES[1]: 3, LEAVES[2]: 2, LEAVES[3]: 2, LEAVES[4]: 3}
SETS = {EDITOR: [(OLD_DOC, NEW_DOC, 1), (OLD_USE, NEW_USE, 1), (OLD_EMIT, NEW_EMIT, 1)], EDITOR_TEST: [(LAW_ANCHOR, LAW + LAW_ANCHOR, 1)]}
SETS.update({leaf: [(ERROR, FATAL, COUNTS[leaf])] for leaf in LEAVES})


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
        print(f"{'WRITE' if WRITE else 'DRY'} {rel.rsplit('/✳️any/', 1)[-1]}: {len(hunks)} hunks")
    if WRITE and problems == 0:
        BACKUP.mkdir(parents=True, exist_ok=True)
        for rel, path, text in planned:
            (BACKUP / key(rel)).write_bytes(path.read_bytes())
            path.write_text(text)
    print(f"{'write' if WRITE and problems == 0 else 'dry-run'}: {len(SETS)} files, {problems} problems")
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main())
