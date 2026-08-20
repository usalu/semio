#!/usr/bin/env python3
"""👁️ R9 full-file de-asyncify for db_preview (🛢️db/👁️preview/🦀️component.rs), db-dedyn packet.

WHY
---
Same shape as `terra-number-deasync.py`/`terra-db-state-deasync.py`/`terra-db-conflict-deasync.py`.
`db_preview`'s OWN module doc already declares it: "This crate has no dependency on
`db_wal`/`db_storage`/`db_snapshot` and performs ZERO I/O — `PreviewStore` is a pure, in-memory,
single-threaded bookkeeping structure." A dedicated test
(`preview_crate_never_references_wal_shaped_symbols`) statically enforces this by scanning the file's
own source for WAL/storage-shaped tokens and asserting none appear outside that one string-literal
list — independent, pre-existing confirmation this file is I/O-free (846 lines, 58 `async fn`).

Its `ConflictOracle` extension point was already converted to R11(a) generic-parameter form
(`&impl ConflictOracle`, not `&dyn`) by this same packet; nothing here needs to stay async to satisfy
that trait, since the trait itself is being converted too. `db_conflict`/`db_state`, this crate's own
dependencies, were converted the same way earlier in this packet.

WHAT IT DOES
------------
Removes `async` from every `async fn` signature and every `.await` postfix in the file.

USAGE
-----
    python3 terra-db-preview-deasync.py --scan   <file>
    python3 terra-db-preview-deasync.py --apply  <file>
"""
import re
import sys

FN_PAT = re.compile(r'\basync(\s+)fn\b')
AWAIT_PAT = re.compile(r'\.await\b')


def main() -> None:
    if len(sys.argv) != 3 or sys.argv[1] not in ("--scan", "--apply"):
        print(__doc__)
        sys.exit(1)
    mode, path = sys.argv[1], sys.argv[2]
    with open(path, encoding="utf-8") as f:
        content = f.read()
    fn_matches = list(FN_PAT.finditer(content))
    await_matches = list(AWAIT_PAT.finditer(content))
    print(f"found {len(fn_matches)} 'async fn' and {len(await_matches)} '.await' occurrences in {path}")
    if mode == "--scan":
        return
    content = FN_PAT.sub("fn", content)
    content = AWAIT_PAT.sub("", content)
    with open(path, "w", encoding="utf-8") as f:
        f.write(content)
    print("applied")


if __name__ == "__main__":
    main()
