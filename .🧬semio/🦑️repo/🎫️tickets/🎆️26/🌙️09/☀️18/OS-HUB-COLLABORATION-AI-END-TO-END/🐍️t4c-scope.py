#!/usr/bin/env python3
"""🔭️ Writes an os-scoped tsconfig whose `files` are the repo-relative paths given on argv.

The config extends `🧰️framework/🛍️products/💻️os/tsconfig.json`, so every compiler option, `paths`
entry and ambient declaration of the real `@semio-tech/framework-os:typecheck` target applies
unchanged; only the program root set shrinks, which turns the ~85 s whole-program run into a couple
of seconds per file. Diagnostics for the named files are identical to the full run.
"""

import json
import pathlib
import sys

FOLDER = pathlib.Path(__file__).resolve().parent
ROOT = FOLDER.parents[6]
PREFIX = "../../../../../../../"
BASE = PREFIX + "🧰️framework/🛍️products/💻️os/tsconfig.json"
AMBIENT = (
    "🧰️framework/📦️packages/🟦️typescript/🌿️ambient/🟦️.d.ts",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🌿️environment/🟦️.d.ts",
)


def main() -> int:
    if len(sys.argv) < 3:
        print("usage: 🐍️t4c-scope.py <out.json> <repo-relative source> ...", file=sys.stderr)
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
