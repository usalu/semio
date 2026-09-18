#!/usr/bin/env python3
"""🧬 Regenerate jco world-alias identifiers in generated `*_component.d.ts` from their WIT ids.

The 2026-09-03 rename-plan codemod incident spliced emoji into ASCII identifiers of
jco/wit-bindgen output, e.g.

    export type * as WasiCliEnvironmen<U+1F52C><U+FE0F>t029 from './interfaces/wasi-cli-environment.js'; // import wasi:cli/environment@0.2.9

This tool does NOT patch text by deleting the stray bytes. It re-derives every alias from
the SOURCE OF TRUTH that jco itself prints on the same line — the fully-qualified WIT
interface id in the trailing `// import <id>` / `// export <id>` comment — using jco's own
naming rule (`PascalCase(namespace) + PascalCase(package) + PascalCase(interface-path) +
version digits with the dots removed`, and the bare camelCase interface name for exports).

The rule is validated on every UNCORRUPTED line of every scanned file before a single byte
is written: if a derived name ever disagrees with an intact generator-written name, the run
aborts. That makes the rewrite a regeneration, not a guess.

Usage:
    python3 <ticket>/🔨️t1-regenerate-jco-aliases.py --check
    python3 <ticket>/🔨️t1-regenerate-jco-aliases.py --apply
"""
from __future__ import annotations

import io
import os
import re
import subprocess
import sys

SKIP_DIRS = {"node_modules", ".git", "target", "⚡️cache", ".venv", ".nx", ".turbo"}

IMPORT_LINE = re.compile(
    r"^(?P<head>export type \* as )(?P<alias>\S+)(?P<mid> from '(?P<mod>[^']+)'; // import )(?P<id>\S+)\s*$"
)
EXPORT_LINE = re.compile(
    r"^(?P<head>export \* as )(?P<alias>\S+)(?P<mid> from '(?P<mod>[^']+)'; // export )(?P<id>\S+)\s*$"
)
ASCII_IDENT = re.compile(r"^[A-Za-z_$][A-Za-z0-9_$]*$")


def pascal(segment: str) -> str:
    """🐫 jco's identifier casing for one kebab/snake WIT segment."""
    return "".join(part[:1].upper() + part[1:] for part in re.split(r"[-_]", segment) if part)


def camel(segment: str) -> str:
    """🐪 jco's export-alias casing: PascalCase with a lowered first character."""
    p = pascal(segment)
    return p[:1].lower() + p[1:]


def split_id(wit_id: str) -> tuple[list[str], str]:
    """🔖 Splits `ns:pkg/a/b@1.2.3` into its path segments and its version-digit suffix."""
    head, _, version = wit_id.partition("@")
    segments = [s for s in re.split(r"[:/]", head) if s]
    return segments, version.replace(".", "").replace("-", "")


def import_alias(wit_id: str) -> str:
    """📥 jco's world-level namespace alias for an imported interface."""
    segments, digits = split_id(wit_id)
    return "".join(pascal(s) for s in segments) + digits


def export_alias(wit_id: str) -> str:
    """📤 jco's world-level namespace alias for an exported interface."""
    segments, _ = split_id(wit_id)
    return camel(segments[-1])


def repo_root() -> str:
    return subprocess.run(
        ["git", "rev-parse", "--show-toplevel"],
        cwd=os.path.dirname(os.path.abspath(__file__)),
        capture_output=True,
        text=True,
        check=True,
    ).stdout.strip()


def main() -> int:
    apply = "--apply" in sys.argv
    root = repo_root()

    checked = 0
    mismatches: list[str] = []
    rewrites: list[tuple[str, int]] = []
    unmatched: list[str] = []
    total_fixed = 0

    for dirpath, dirnames, filenames in os.walk(root):
        dirnames[:] = [d for d in dirnames if d not in SKIP_DIRS]
        for name in filenames:
            if not name.endswith(".d.ts"):
                continue
            path = os.path.join(dirpath, name)
            rel = os.path.relpath(path, root)
            try:
                src = io.open(path, encoding="utf-8").read()
            except (OSError, UnicodeDecodeError):
                continue
            # jco stamps every world-level binding file with this header; it is the only
            # marker that selects generator output without guessing at file names.
            if not src.startswith("// world "):
                continue

            out_lines: list[str] = []
            fixed = 0
            for lineno, line in enumerate(src.split("\n"), 1):
                m = IMPORT_LINE.match(line)
                derive = import_alias
                if not m:
                    m = EXPORT_LINE.match(line)
                    derive = export_alias
                if not m:
                    if line.startswith("export ") and not ASCII_IDENT.match(line.split()[-1]):
                        unmatched.append(f"{rel}:{lineno}\t{line}")
                    out_lines.append(line)
                    continue

                want = derive(m.group("id"))
                have = m.group("alias")
                if ASCII_IDENT.match(have):
                    # Generator-written, intact: this line VALIDATES the derivation rule.
                    checked += 1
                    if have != want:
                        mismatches.append(f"{rel}:{lineno}\thave={have}\twant={want}\tid={m.group('id')}")
                    out_lines.append(line)
                    continue

                fixed += 1
                out_lines.append(f"{m.group('head')}{want}{m.group('mid')}{m.group('id')}")

            if fixed:
                rewrites.append((rel, fixed))
                total_fixed += fixed
                if apply and not mismatches:
                    io.open(path, "w", encoding="utf-8").write("\n".join(out_lines))

    print(f"intact generator-written aliases that validate the rule: {checked}")
    print(f"rule disagreements on intact aliases: {len(mismatches)}")
    for line in mismatches[:40]:
        print(f"  MISMATCH {line}")
    if unmatched:
        print(f"\nexport lines the grammar did not match ({len(unmatched)}):")
        for line in unmatched[:40]:
            print(f"  {line}")
    print(f"\nfiles with corrupted aliases: {len(rewrites)}  aliases regenerated: {total_fixed}")
    for rel, n in sorted(rewrites):
        print(f"  {'regenerated' if apply and not mismatches else 'would regenerate'} {n:3d}  {rel}")

    if mismatches:
        print("\nABORTED: the derivation rule disagrees with intact generator output; nothing written.")
        return 2
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
