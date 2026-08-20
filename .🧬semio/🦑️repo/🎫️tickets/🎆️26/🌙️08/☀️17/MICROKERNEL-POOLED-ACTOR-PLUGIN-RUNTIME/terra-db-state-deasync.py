#!/usr/bin/env python3
"""🌲 R9 full-file de-asyncify for db_state (🛢️db/🔘️state/🦀️component.rs), db-dedyn packet.

WHY
---
Same shape as `terra-number-deasync.py` (semio-framework-number, already landed on this ticket):
`db_state` is a hand-rolled, `Rc`-based persistent data structure library (32-way HAMT `PMap`,
bitmapped-trie `PVec`, rope `PText`, weight-balanced `PTree`, adjacency-map `PGraph`) with genuinely
ZERO I/O. Verified before writing this script:

  * ZERO `std::fs`/`tokio`/`reqwest`/`ureq`/`File::`/`TcpStream`/`spawn`/`sleep`/`SystemTime` anywhere
    in the file.
  * The only cross-crate reference is `pack::ContentHash` used as a plain VALUE TYPE (a `[u8; 32]`
    newtype) plus a sync `blake3::hash(...)` call — no genuinely-async external dependency.
  * `CanonicalEncode`, the one first-party trait declared in this file, is used ONLY within this file
    (no other crate module in `db_state`'s dependents references it), so reverting its method is
    self-contained.
  * 131 `async fn` / 108 `.await` before this script ran — the gap is exactly the missing-await
    residue this packet was already mechanically clearing; irrelevant to the R9 case, since every
    `.await` here targets ANOTHER fn in this same file that is ALSO being reverted.

WHAT IT DOES
------------
1. Removes `async` from every `async fn` signature.
2. Removes every `.await` (the receiver expression is left as-is; only the `.await` postfix goes).

Both are whole-token matches, so this cannot fire inside a doc comment or string literal (same
argument as `terra-number-deasync.py`: this is a structural signature/postfix edit over one owned
file, not the name-keyed call-site guessing R10 bans).

USAGE
-----
    python3 terra-db-state-deasync.py --scan   <file>
    python3 terra-db-state-deasync.py --apply  <file>
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
