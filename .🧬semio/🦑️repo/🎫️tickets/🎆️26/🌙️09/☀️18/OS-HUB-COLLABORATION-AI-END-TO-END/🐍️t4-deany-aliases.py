#!/usr/bin/env python3
"""🧬️ Replaces `type X = any;` placeholders in extracted test modules with the real declaration.

Each extracted `registerTestsN` harness re-declares the names it needs as `= any`, which erases every
type the caller actually passes. This resolves each name against the exported declarations of the os
product and the framework modules and rewrites it to `import("<relative path>").X` when exactly one
module exports it. Ambiguous or unresolved names are left alone and printed, so nothing is guessed.
"""
import io, os, re, sys

ROOT = "/Users/ueli/Documents/semio"
SEARCH = ["🧰️framework/🛍️products/💻️os", "🧰️framework/🔨️modules"]
SKIP_DIR = re.compile(r"(^|/)(node_modules|dist|📤️dist|🎯️target|🗑️generated|🤖️generated|🕸️bindings|🔌️plugin-modules|🧩️extension-modules|🌐️browser-bundles|🧪️tests)(/|$)")
EXPORT = re.compile(r"^export\s+(?:declare\s+)?(?:abstract\s+)?(?:type|interface|class|enum)\s+([A-Za-z_$][\w$]*)", re.M)
ALIAS = re.compile(r"^(\s*)type ([A-Za-z_$][\w$]*) = any;$", re.M)


def index() -> dict[str, list[str]]:
    """🗂️ Maps every exported type name to the modules that export it."""
    out: dict[str, list[str]] = {}
    for base in SEARCH:
        for folder, dirs, files in os.walk(os.path.join(ROOT, base)):
            rel = os.path.relpath(folder, ROOT)
            if SKIP_DIR.search(rel):
                dirs[:] = []
                continue
            for name in files:
                if not name.endswith((".ts", ".tsx")) or name.endswith(".d.ts"):
                    continue
                path = os.path.join(folder, name)
                try:
                    text = io.open(path, encoding="utf-8").read()
                except OSError:
                    continue
                for match in EXPORT.finditer(text):
                    out.setdefault(match.group(1), []).append(path)
    return out


def main(argv: list[str]) -> int:
    apply = "--apply" in argv
    targets = [a for a in argv[1:] if not a.startswith("--")]
    table = index()
    for target in targets:
        text = io.open(target, encoding="utf-8").read()
        changed, unresolved, ambiguous = 0, [], []
        for match in list(ALIAS.finditer(text)):
            indent, name = match.group(1), match.group(2)
            owners = sorted(set(table.get(name, [])))
            if not owners:
                unresolved.append(name)
                continue
            if len(owners) > 1:
                ambiguous.append((name, len(owners)))
                continue
            spec = os.path.relpath(owners[0], os.path.dirname(os.path.join(ROOT, target)))
            if not spec.startswith("."):
                spec = "./" + spec
            text = text.replace(match.group(0), f'{indent}type {name} = import("{spec}").{name};')
            changed += 1
        print(f"{target}: {changed} resolved, unresolved={unresolved}, ambiguous={ambiguous}")
        if apply and changed:
            io.open(target, "w", encoding="utf-8").write(text)
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
