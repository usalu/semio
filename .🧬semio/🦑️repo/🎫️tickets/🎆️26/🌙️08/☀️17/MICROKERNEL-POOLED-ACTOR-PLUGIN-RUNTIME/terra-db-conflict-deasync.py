#!/usr/bin/env python3
"""⚔️ R9 full-file de-asyncify for db_conflict (🛢️db/⚔️conflict/🦀️component.rs), db-dedyn packet.

WHY
---
Same shape as `terra-number-deasync.py`/`terra-db-state-deasync.py`: `db_conflict` (touched-region
intersection, bloom pre-filter, command-kind matrix, constraint conflict detection) is pure in-memory
computation. Verified before writing this script:

  * ZERO `std::fs`/`tokio`/`reqwest`/`ureq`/`File::`/`TcpStream`/`spawn`/`sleep`/`SystemTime` anywhere
    in the file (684 lines, 48 `async fn`).
  * Every cross-crate reference (`protocol::MutationId`/`ActorId`/`HybridLogicalTimestamp`/…) is used
    purely as a VALUE TYPE — no genuinely-async external call anywhere.
  * Overwhelming existing-call-site evidence it was meant to stay sync: `ConflictDetector::new()` is
    called WITHOUT `.await` at 14+ sites across this crate (tests here, `db_preview`, `db_artifact`),
    vs. exactly one stray `.await` (mechanical residue this packet's own tooling added).

WHAT IT DOES
------------
Removes `async` from every `async fn` signature and every `.await` postfix in the file. Whole-token
matches only (same discipline as the two precedents above — structural signature/postfix edit over one
owned file, not R10's banned name-keyed call-site guessing).

USAGE
-----
    python3 terra-db-conflict-deasync.py --scan   <file>
    python3 terra-db-conflict-deasync.py --apply  <file>
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
