#!/usr/bin/env python3
"""🩹️ Completes the REPO-PATH-BUDGET rename (ticket 26/10/01/REPO-PATH-BUDGET) inside the S2-CONTROLS trees, where it stopped
half-way: references and directories disagree on word-boundary-truncated names (`⛔️refuses-a-blank-name` vs `⛔️refuses`).
Every relative reference that names no file (Rust `#[path]` / `include_str!` / `include_bytes!`, TS `from` / `import()` /
`new URL()`, JSON `"path"`) is resolved at its first missing segment `m` inside the existing parent: a sibling directory
`m-…` (the long spelling) MOVES to `m` — except a command module directory, whose reference is rewritten to the existing name
instead (the budget renamed evidence and leaf directories, never commands); a sibling `d` with `m = d-…` REWRITES the
reference to `d`. Only unique matches act. Prints the plan; `--apply` performs it and loops until nothing changes.
Run from the repository root."""

import os
import pathlib
import re
import sys

ROOTS = ["✏️s/🔌️plugins/📋️forms", "✏️s/🔌️plugins/🔋️energy", "✏️s/🔌️plugins/🌍️gis", "✏️s/🔌️plugins/📖️playbook", "✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract"]
PATTERNS = {
    ".rs": re.compile(r'(?:include_str!|include_bytes!)\(\s*"([^"]+)"\s*\)|#\[path\s*=\s*"([^"]+)"\]'),
    ".ts": re.compile(r'(?:from|import\(|new URL\()\s*"(\.\.?/[^"]+)"'),
    ".tsx": re.compile(r'(?:from|import\(|new URL\()\s*"(\.\.?/[^"]+)"'),
    ".json": re.compile(r'"path"\s*:\s*"(\.\.?/[^"]+)"'),
}


def ascii_tail(name):
    return "".join(character for character in name if ord(character) < 128)


def references():
    for root in ROOTS:
        for directory, dirs, files in os.walk(root):
            dirs[:] = [name for name in dirs if name not in ("node_modules", "target", "dist")]
            for name in files:
                pattern = PATTERNS.get(pathlib.Path(name).suffix)
                if pattern is None:
                    continue
                file = pathlib.Path(directory) / name
                for match in pattern.finditer(file.read_text(encoding="utf-8")):
                    reference = next(group for group in match.groups() if group)
                    if reference != "." and not reference.startswith("/"):
                        yield file, reference


def plan():
    moves, rewrites, unresolved = {}, {}, []
    for file, reference in references():
        target = pathlib.Path(os.path.normpath(file.parent / reference.split("?")[0]))
        if target.exists():
            continue
        parts = target.parts
        index = next(i for i in range(len(parts)) if not pathlib.Path(*parts[: i + 1]).exists())
        parent, missing = pathlib.Path(*parts[:index]), parts[index]
        if not parent.is_dir():
            unresolved.append((file, reference))
            continue
        siblings = [entry.name for entry in parent.iterdir() if entry.is_dir()]
        longer = [name for name in siblings if name.startswith(missing + "-")]
        shorter = [name for name in siblings if missing.startswith(name + "-")]
        if len(longer) == 1 and ascii_tail(parent.name) != "commands":
            moves[str(parent / longer[0])] = str(parent / missing)
        elif len(longer) == 1 or len(shorter) == 1:
            existing = (longer or shorter)[0]
            fixed = reference.replace(f"/{missing}/", f"/{existing}/", 1) if f"/{missing}/" in reference else reference.replace(missing, existing, 1)
            rewrites.setdefault(str(file), {})[reference] = fixed
        else:
            unresolved.append((file, reference))
    return moves, rewrites, unresolved


def main(apply):
    for round in range(8):
        moves, rewrites, unresolved = plan()
        print(f"round {round}: {len(moves)} moves, {sum(len(edits) for edits in rewrites.values())} rewrites in {len(rewrites)} files, {len(unresolved)} unresolved")
        for old, new in sorted(moves.items()):
            print("  move", old[-110:], "->", pathlib.Path(new).name)
        for file, edits in sorted(rewrites.items()):
            print("  rewrite", file[-100:], len(edits))
        if not apply or (not moves and not rewrites):
            break
        for old, new in moves.items():
            if not os.path.exists(new):
                os.rename(old, new)
        for file, edits in rewrites.items():
            path = pathlib.Path(file)
            text = path.read_text(encoding="utf-8")
            for reference, fixed in edits.items():
                text = text.replace(f'"{reference}"', f'"{fixed}"')
            path.write_text(text, encoding="utf-8")
    for file, reference in unresolved:
        print("  unresolved", str(file)[-100:], reference[-100:])


if __name__ == "__main__":
    main("--apply" in sys.argv)
