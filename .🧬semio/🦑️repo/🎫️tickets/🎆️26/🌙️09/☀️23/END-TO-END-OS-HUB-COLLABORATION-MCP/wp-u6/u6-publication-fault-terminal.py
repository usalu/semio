#!/usr/bin/env python3
"""🧾️ U6 window-3 set C6 (framework plugin SDK, next train): a typed operation whose PUBLICATION is refused must not leave a
history row.

Measured 03:47 (`.🧬semio/🌐hub/s14-u6-logs/d1-probe-history.txt`, semio brep `registered_set_vertex_refuses_duplicate_ids_without_history`):
a refused `set-vertex` (ambiguous vertex id, refused inside the retained store preparation) settles with a History row
`{action_id: "set-vertex", kind: "mutation", op_lines: [], applied: false, revertible: false}` — the host reads `applied: false` as
UNDONE, so every refused edit shows up as a dimmed "undone" entry. Root cause (`🔌️plugin/🦀️.rs`, probed with a backtrace in the
scratch copy, `d2-probe-log.txt`: `retire_typed_operation_unit` → `record_settled_typed_operation_command` → `record_command`): the
worker fault IS recorded (`terminal_fault = Some(…)` on the `Fault` outcome), but building the Fault result page CONSUMED it
(`self.terminal_fault.take()`), so at retirement the recorder — "silent for a faulted operation" — saw no fault and filed the
refused verb as "a Mutation that emitted nothing". The page is now framed from a borrow, the fault stays the operation's terminal
state; and the publication-retry branch, which queued a Fault page without recording the fault at all, records it too. Laws: the
semio brep/mesh `registered_set_vertex_*` tests (history length unchanged after a refused edit).

Usage: u6-publication-fault-terminal.py [--dry-run | --write | --revert] [--root <repo root>]"""
import hashlib
import sys
from pathlib import Path

ROOT = Path(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else Path("/Users/ueli/Documents/semio")
WRITE = "--write" in sys.argv
REVERT = "--revert" in sys.argv
BACKUP = Path("/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-u6-backup/publication-fault-terminal")
PLUGIN = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs"
OLD = """                    let page = TypedOperationResultPage::try_new(mounted.next_token(), TypedOperationResultLane::Fault, &framed[..framed_len])?;
                    mounted.result_page = None;
                    mounted.stage = MountedTypedCommandFullOperationStage::Publishing;
                    mounted.queue_page(page)?;
"""
NEW = """                    let page = TypedOperationResultPage::try_new(mounted.next_token(), TypedOperationResultLane::Fault, &framed[..framed_len])?;
                    mounted.result_page = None;
                    mounted.stage = MountedTypedCommandFullOperationStage::Publishing;
                    mounted.terminal_fault = Some(bounded);
                    mounted.queue_page(page)?;
"""
TAKE_OLD = """            let page = match self.terminal_fault.take() {
                Some(fault) => {
"""
TAKE_NEW = """            let page = match self.terminal_fault.as_ref() {
                Some(fault) => {
"""
SETS = {PLUGIN: [(TAKE_OLD, TAKE_NEW, 1), (OLD, NEW, 1)]}


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
                print(f"PROBLEM {rel}: {'already applied' if new in text else f'anchor count {found} != {count}'}")
                problems += 1
                continue
            text = text.replace(old, new)
        planned.append((rel, path, text))
        print(f"{'WRITE' if WRITE else 'DRY'} {rel}: {len(hunks)} hunks")
    if WRITE and problems == 0:
        BACKUP.mkdir(parents=True, exist_ok=True)
        for rel, path, text in planned:
            (BACKUP / key(rel)).write_bytes(path.read_bytes())
            path.write_text(text)
    print(f"{'write' if WRITE and problems == 0 else 'dry-run'}: {len(SETS)} files, {problems} problems")
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main())
