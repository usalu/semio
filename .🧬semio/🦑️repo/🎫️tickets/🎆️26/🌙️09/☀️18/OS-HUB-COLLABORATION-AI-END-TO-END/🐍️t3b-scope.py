#!/usr/bin/env python3
"""🔭️ Writes a framework-scoped tsconfig whose `files` are the repo-relative paths given on argv.

The config extends `🧰️framework/📦️packages/🟦️typescript/tsconfig.json`, so every compiler option,
`paths` entry and ambient declaration of the real typecheck target applies unchanged; only the
program root set shrinks, which turns a ~32 s whole-program run into a few seconds per file.
"""

import json
import pathlib
import sys

FOLDER = pathlib.Path(__file__).resolve().parent
ROOT = FOLDER.parents[6]
BASE = "../../../../../../../🧰️framework/📦️packages/🟦️typescript/tsconfig.json"
PREFIX = "../../../../../../../"
AMBIENT = (
    "🧰️framework/📦️packages/🟦️typescript/🌿️ambient/🟦️.d.ts",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🌿️environment/🟦️.d.ts",
)


def main() -> int:
    if len(sys.argv) < 3:
        print("usage: 🐍️t3b-scope.py <out.json> <repo-relative source> ...", file=sys.stderr)
        return 2
    out = FOLDER / sys.argv[1]
    files = []
    for entry in [*AMBIENT, *sys.argv[2:]]:
        if not (ROOT / entry).is_file():
            print(f"missing: {entry}", file=sys.stderr)
            return 1
        files.append(PREFIX + entry)
    files.append("🌿️t3b-ambient.d.ts")
    payload = {
        "extends": BASE,
        "compilerOptions": {"incremental": False, "noEmit": True},
        "files": files,
        "include": [],
    }
    out.write_text(json.dumps(payload, ensure_ascii=False, indent=2) + "\n", encoding="utf8")
    print(f"{out.name}: {len(files)} files")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
