#!/usr/bin/env python3
"""🧮 R9 full-crate de-asyncify for semio-framework-number.

WHY
---
`🧰️framework/🔨️modules/🔢️number/🦀️component.rs` (4290 lines, 384 `async fn`) was blanket-converted
by the universal-async codemod. Verified before writing this script:

  * ZERO `.await` anywhere in the file (confirmed by direct grep on the raw source) — nothing here
    ever actually suspends.
  * ZERO I/O markers anywhere in the file — no `std::fs`, `tokio`, `reqwest`, `ureq`, `File::`,
    `TcpStream`, `spawn`, `sleep`, `SystemTime`, `async_std`, `smol`.
  * ZERO `async move` / `async {}` blocks — the only use of the `async` keyword in the whole file is
    the 384 `async fn` signatures matched below.
  * ZERO pre-existing `// 🚫️async: E<n>` tags — nothing has already been hand-classified.
  * The crate's public surface (`Natural`, `Integer`, `Rational`, `ModInt`, `MatG`/`VecG`) is entirely
    consumed through E1 impls of externally-declared traits (`Display`, `Debug`, `FromStr`, `Ord`,
    `PartialOrd`, `From`) for those types, which transitively call the `Ring`/`Field` trait hierarchy
    and every arithmetic helper in the file (R9: E1 propagates one hop backwards along the call
    graph — and here the call graph IS the crate).

Given no suspension point anywhere and a fully-connected E1-rooted call graph, every one of the 384
fns is R9-eligible, not a subset — this is the "verify I/O-freedom per function" requirement satisfied
at the file level, not a shortcut around it. This is the same pattern as the `🌱️value` (11+8 fns) and
`⚠️diagnostic` (39+2 fns) worked precedents, at larger scale.

WHAT IT DOES
------------
Removes exactly the `async` keyword from every `async fn` line in the target file. Nothing else.
`.await` removal is not needed (there are none). Idempotent — a second run finds nothing to change.

SAFETY
------
Whole-token match (`\basync\s+fn\b`), so it cannot fire inside a doc comment (verified separately:
0 matches on comment-prefixed lines) or a string literal (verified separately: no quote immediately
precedes any match). This is a structural signature-level edit, not a name-keyed call-site edit — the
R10 hazard (std method names colliding with first-party async fn names) applies to `.await`
INSERTION at call sites, not to stripping the `async` keyword from `fn` declarations in a single
owned file.

USAGE
-----
    python3 terra-number-deasync.py --scan   <file>
    python3 terra-number-deasync.py --apply  <file>
"""
import re
import sys

PAT = re.compile(r'\basync(\s+)fn\b')


def main() -> None:
    if len(sys.argv) != 3 or sys.argv[1] not in ("--scan", "--apply"):
        print(__doc__)
        sys.exit(1)
    mode, path = sys.argv[1], sys.argv[2]
    with open(path, encoding="utf-8") as f:
        content = f.read()
    matches = list(PAT.finditer(content))
    print(f"found {len(matches)} 'async fn' occurrences in {path}")
    if mode == "--scan":
        return
    new_content = PAT.sub("fn", content)
    if new_content == content:
        print("no change")
        return
    with open(path, "w", encoding="utf-8") as f:
        f.write(new_content)
    remaining = len(PAT.findall(new_content))
    print(f"applied. remaining 'async fn' occurrences: {remaining}")


if __name__ == "__main__":
    main()
