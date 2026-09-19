#!/usr/bin/env python3
"""🧬️ Emits an exported `typeof`-only dependency type for an extracted `registerTestsN` harness.

The caller hands its test module a bag of module-internal bindings. Naming that bag as
`Pick<typeof import(caller), …>` only works for exported keys; this instead writes a type alias
beside the call site whose every member is `typeof <the live binding>`, so the harness parameter is
exact and cannot drift from what is actually passed. Type-only, therefore erased from the bundle.
"""
import io, os, re, sys

DECL = re.compile(r"^(?:export\s+)?(?:declare\s+)?(?:async\s+)?(?:const|let|var|function|class|enum)\s+([A-Za-z_$][\w$]*)", re.M)


def call_keys(text: str, fn: str) -> list[str]:
    """📥️ Reads the object-literal keys the caller passes to `registerTestsN`."""
    marker = f"await {fn}(import.meta.vitest,"
    line = next(l for l in text.split("\n") if marker in l)
    i = line.index("{", line.index("import.meta.vitest"))
    j = line.index("}", i)
    return [k.strip() for k in line[i + 1 : j].split(",") if k.strip()]


IMPORT = re.compile(r'import\s+(?:type\s+)?\{([^}]*)\}\s*from\s*"', re.S)


DESTRUCTURE = re.compile(r"^(?:export\s+)?(?:const|let|var)\s*\{([^}]*)\}\s*=", re.M | re.S)


def declared(text: str) -> set[str]:
    """🔍️ Collects every top-level declaration, destructured binding and value import the caller owns."""
    out = {m.group(1) for m in DECL.finditer(text)}
    for blk in DESTRUCTURE.findall(text):
        for part in blk.split(","):
            part = part.strip()
            if part:
                out.add(part.split(":")[-1].strip())
    for blk in IMPORT.findall(text):
        for part in blk.split(","):
            part = part.strip()
            if not part or part.startswith("type "):
                continue
            out.add(part.split(" as ")[-1].strip())
    return out


def main() -> int:
    caller, fn, name = sys.argv[1], sys.argv[2], sys.argv[3]
    skip = set(sys.argv[4:])
    text = io.open(caller, encoding="utf-8").read()
    names = declared(text)
    keys = call_keys(text, fn)
    missing = [k for k in keys if k not in names and k not in skip]
    if missing:
        print(f"MISSING {missing}", file=sys.stderr)
        return 1
    body = "\n".join(f"  readonly {k}: typeof {k};" for k in keys if k not in skip)
    print(f"export type {name} = {{\n{body}\n}};")
    return 0


if __name__ == "__main__":
    sys.exit(main())
