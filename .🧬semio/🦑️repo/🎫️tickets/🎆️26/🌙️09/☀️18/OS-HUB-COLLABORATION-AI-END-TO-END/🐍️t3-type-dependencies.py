#!/usr/bin/env python3
"""🐍️ T3 helper: replace an extracted test's `dependencies: any` with a `Pick<>` of its real module.

Usage: python3 🐍️t3-type-dependencies.py <test 🟦️.ts> <relative module specifier>

These suites were machine-extracted out of their module: the module keeps an `import.meta.vitest`
block that dynamically imports the suite and hands it an object literal of its own exports, and the
extraction typed that parameter `any`. Every implicit-any callback parameter, every "untyped function
calls may not accept type arguments" and every "cannot find name" inside such a file traces back to
that one `any`.

This rewrites `dependencies: any` to `Pick<typeof import("<module>"), "<name>" | …>`, with the names
read from the suite's own top-level destructuring — so the parameter type is exactly the set of
exports the call site passes, and nothing is widened.
"""
import io
import re
import sys

DESTRUCTURE = re.compile(r"^  const \{ (?P<names>[^}]*) \} = dependencies;$", re.M)
PARAMETER = re.compile(r"dependencies: (?:any|Record<string, any>)")


def main(argv):
    if len(argv) != 3:
        raise SystemExit(__doc__)
    path, module = argv[1], argv[2]
    source = io.open(path, encoding="utf-8").read()
    blocks = list(DESTRUCTURE.finditer(source))
    if not blocks:
        raise SystemExit(f"{path}: no `const {{ … }} = dependencies;` block found")
    names = sorted({n.strip() for block in blocks for n in block.group("names").split(",") if n.strip()})
    union = " | ".join(f'"{n}"' for n in names)
    replaced, count = PARAMETER.subn(f'dependencies: Pick<typeof import("{module}"), {union}>', source)
    if count == 0:
        raise SystemExit(f"{path}: no `dependencies: any` parameter found")
    io.open(path, "w", encoding="utf-8").write(replaced)
    print(f"{path}: typed {count} parameter(s) against {len(names)} export(s) of {module}")


if __name__ == "__main__":
    main(sys.argv)
