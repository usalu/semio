#!/usr/bin/env python3
"""🧬️ Emits an exported `typeof`-only dependency type beside a harness call site.

An extracted test harness takes a bag of the caller's module-internal bindings. Naming that bag
`Pick<typeof import(caller), …>` only works for exported keys; this writes a type alias into the
caller whose every member is `typeof <the live binding>`, so the harness parameter is exact and
cannot drift from what is actually passed. Type-only, therefore erased from every bundle.

usage: 🐍️t4c-deps-type.py <caller path> <call marker> <type name> [anchor line prefix]
The keys are read from the first `{ … }` object literal on the line containing <call marker>.
The alias is inserted immediately before the line matching <anchor line prefix> (default: the call).
"""

import pathlib
import sys


def main() -> int:
    if len(sys.argv) < 4:
        print(__doc__, file=sys.stderr)
        return 2
    caller, marker, name = sys.argv[1], sys.argv[2], sys.argv[3]
    anchor = sys.argv[4] if len(sys.argv) > 4 else None
    path = pathlib.Path(caller)
    lines = path.read_text(encoding="utf8").split("\n")
    call = next(i for i, line in enumerate(lines) if marker in line)
    body = lines[call]
    start = body.index("{", body.index(marker))
    end = body.index("}", start)
    keys = [key.strip() for key in body[start + 1 : end].split(",") if key.strip()]
    if any(":" in key for key in keys):
        print(f"not a shorthand bag: {keys}", file=sys.stderr)
        return 1
    members = "\n".join(f"  readonly {key}: typeof {key};" for key in keys)
    alias = f"/** 🧬️ The exact bag handed to `{marker.rstrip('(')}` — `typeof` of the live bindings, so it cannot drift. */\nexport type {name} = Readonly<{{\n{members}\n}}>;"
    at = call if anchor is None else next(i for i, line in enumerate(lines) if line.startswith(anchor))
    lines.insert(at, alias)
    path.write_text("\n".join(lines), encoding="utf8")
    print(f"{name}: {len(keys)} members -> {caller}:{at + 1}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
