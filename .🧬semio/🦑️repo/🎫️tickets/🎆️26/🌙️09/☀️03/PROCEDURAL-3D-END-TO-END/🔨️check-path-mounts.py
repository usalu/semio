#!/usr/bin/env python3
"""🧭️ Resolve every `#[path = "…"]` mount in a crate's source tree against the filesystem.

A missing mount is `error: couldn't read …: No such file or directory (os error 2)`, which costs a
full cargo run to discover. This gate finds the whole class in under a second, so a tree churned by
the repo-wide emoji rename can be triaged without holding the build lock.

Usage: 🔨️check-path-mounts.py <root> [<root> …]
Exit 0 when every build-blocking reference resolves, 1 otherwise.

⚠️ A clear result is NECESSARY BUT NOT SUFFICIENT. Three things this cannot see:

1. **Paths passed to macros.** Two sub-cases, both invisible:
   - `include_bytes!($protocol)` inside a `macro_rules!` arm — no literal at the definition site, the
     path exists only per expansion (`📇️registry/🦀️.rs:897`).
   - a quoted path argument to an *ordinary* macro, e.g.
     `native_codec_factory!(obj_codec, …, "../🗿️artifacts/🗽️obj/…/📡️.protocol.semio")`
     (`📇️registry/🦀️.rs:923`) — neither a `#[path]` mount nor an `include_*`, so scanning for either
     misses it entirely. This class hid one of the two `🧊️obj` bugs.

   A `couldn't read` on a line this gate did not flag is the signature of both. Closing them means
   resolving macro *invocation arguments*, not just attribute and include literals.
2. **Reachability.** This is a per-file scan, so a reference in an orphaned file or in a crate nothing
   depends on looks identical to a live one. Filter by build graph in the consumer.
3. **Type errors.** Every unresolved-import and trait-bound failure is past this gate entirely. A crate
   whose mounts all resolve can still fail to compile — and a crate that dies HERE never type-checks at
   all, so any "0 errors of class X" taken from such a build is meaningless.
"""

import os
import re
import sys
import unicodedata

MOUNT = re.compile(r'#\[\s*path\s*=\s*"([^"]+)"\s*\]')
INCLUDE = re.compile(r'include(?:_bytes|_str)?\s*!\s*\(\s*"([^"]+)"\s*\)')
VS16 = "️"


def strip_marks(name: str) -> str:
    """🔤️ The comparison key that ignores emoji variation selectors — the drift class the rename tool
    introduces most often (`✍set-text` in source vs `✍️set-text` on disk)."""
    return unicodedata.normalize("NFC", name).replace(VS16, "")


CFG_TEST = re.compile(r'#\[\s*cfg\s*\(\s*(?:all\s*\(\s*)?test\b')


def in_test_scope(text: str, offset: int) -> bool:
    """🧪️ Whether `offset` sits inside a `#[cfg(test)]` item — brace-counted from each such attribute
    to the end of the block it guards.

    The distinction is the whole point of the gate: an unresolved reference outside `cfg(test)` stops
    `cargo check` and every component build, while one inside it only stops `cargo test`. Counting
    them together makes a blocked build and a merely untestable one look identical.

    Scope decides this, not the syntax — a production `include_bytes!` of a missing asset is every bit
    as fatal as a missing `#[path]` module, and a `#[path]` inside `#[cfg(test)]` is not fatal at all.
    """
    for guard in CFG_TEST.finditer(text):
        opening = text.find("{", guard.end())
        if opening == -1 or opening > offset:
            continue
        depth = 0
        for index in range(opening, len(text)):
            if text[index] == "{":
                depth += 1
            elif text[index] == "}":
                depth -= 1
                if depth == 0:
                    if opening < offset < index:
                        return True
                    break
    return False


def scan(root: str) -> list[tuple[str, int, str, str]]:
    findings = []
    for directory, _, files in os.walk(root):
        if "/target" in directory or "/node_modules" in directory:
            continue
        for name in files:
            if not name.endswith(".rs"):
                continue
            source = os.path.join(directory, name)
            try:
                text = open(source, encoding="utf-8").read()
            except (OSError, UnicodeDecodeError):
                continue
            for pattern in (MOUNT, INCLUDE):
                for match in pattern.finditer(text):
                    mount = match.group(1)
                    if not any(mark in mount for mark in ("/", "\\")) and pattern is INCLUDE:
                        continue
                    resolved = os.path.normpath(os.path.join(directory, mount))
                    if os.path.exists(resolved):
                        continue
                    line = text.count("\n", 0, match.start()) + 1
                    blocking = not in_test_scope(text, match.start())
                    findings.append((source, line, mount, nearest(resolved), blocking))
    return findings


def nearest(resolved: str) -> str:
    """🔎️ The on-disk sibling whose name matches once variation selectors are dropped — names the
    repair when the drift is a selector, and stays empty when the emoji itself was swapped."""
    parent, missing = os.path.split(resolved)
    while parent and not os.path.isdir(parent):
        parent, missing = os.path.split(parent)
    if not os.path.isdir(parent):
        return ""
    key = strip_marks(missing)
    matches = [entry for entry in os.listdir(parent) if strip_marks(entry) == key]
    return os.path.join(parent, matches[0]) if matches else ""


def main() -> int:
    roots = sys.argv[1:] or ["."]
    findings = [finding for root in roots for finding in scan(root)]
    blocking = [finding for finding in findings if finding[4]]
    for source, line, mount, suggestion, is_blocking in sorted(findings, key=lambda finding: not finding[4]):
        hint = f"  -> selector-only drift, on disk as {suggestion!r}" if suggestion else "  -> no selector-equivalent sibling; the emoji itself differs"
        print(f"[{'BUILD' if is_blocking else 'test '}] {source}:{line}: unresolved {mount!r}\n{hint}")
    print(f"\n{len(blocking)} build-blocking mount(s); {len(findings) - len(blocking)} test-only reference(s); {len(roots)} root(s)")
    return 1 if blocking else 0


if __name__ == "__main__":
    sys.exit(main())
