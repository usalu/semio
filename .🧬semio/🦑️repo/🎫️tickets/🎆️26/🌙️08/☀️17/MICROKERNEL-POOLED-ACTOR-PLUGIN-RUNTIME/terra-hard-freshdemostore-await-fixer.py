#!/usr/bin/env python3
# 🩹 terra-hard-freshdemostore-await-fixer.py
#
# R10-compliant recovery tool (diagnostic-shape-driven, NOT name-keyed against std collisions).
#
# Root cause: `let mut <name> = fresh_demo_store();` never got `.await` appended at the binding
# (fresh_demo_store is `async fn`), so <name> is bound to an unpolled `impl Future<Output=
# ArtifactStore<..>>`. A subsequent bulk pass then (wrongly) inserted `.await` after every use of
# <name> as a receiver — `<name>.await.method()` — instead of at the one binding site. This is the
# SAME defect shape as the `ArtifactStore::new()` fixer in `terra-store-artifactstorenew-await-
# fixer.py` (alltargets-kernel packet), applied to a second constructor.
#
# This tool is safe against R10's std-collision trap because it is NOT name-keyed globally: for
# each `fresh_demo_store()` binding it (a) awaits the binding itself, then (b) rewrites `<name>
# .await` back to bare `<name>` ONLY inside that binding's own enclosing brace-delimited scope
# (computed by brace-depth tracking from the binding forward to its block's closing `}`), so a
# same-named local in a different test function is never touched.
import re
import sys
import argparse

BINDING_RE = re.compile(r"let (?:mut )?(\w+) = fresh_demo_store\(\);")


def find_scope_end(text: str, start: int) -> int:
    depth = 0
    i = start
    while i < len(text):
        c = text[i]
        if c == "{":
            depth += 1
        elif c == "}":
            if depth == 0:
                return i
            depth -= 1
        i += 1
    return len(text)


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("path")
    ap.add_argument("--apply", action="store_true")
    args = ap.parse_args()

    text = open(args.path, encoding="utf-8").read()
    edits = []  # (start, end, replacement)
    sites = 0
    freed = 0

    for m in BINDING_RE.finditer(text):
        name = m.group(1)
        sites += 1
        # (a) await the binding itself: insert ".await" before the trailing ";"
        call_end = m.end() - 1  # position of the ";"
        edits.append((call_end, call_end, ".await"))

        # (b) strip erroneous "<name>.await" -> "<name>" within this binding's own scope
        scope_end = find_scope_end(text, m.end())
        use_re = re.compile(r"(?<![\w.])" + re.escape(name) + r"\.await\b")
        for um in use_re.finditer(text, m.end(), scope_end):
            edits.append((um.start(), um.end(), name))
            freed += 1

    print(f"found {sites} fresh_demo_store() bindings, {freed} stray '.await' uses to strip")

    if not args.apply:
        print("dry-run only; pass --apply to write")
        return 0

    edits.sort(key=lambda e: e[0], reverse=True)
    out = text
    for start, end, repl in edits:
        out = out[:start] + repl + out[end:]

    with open(args.path, "w", encoding="utf-8") as f:
        f.write(out)
    print(f"applied {len(edits)} edits")
    return 0


if __name__ == "__main__":
    sys.exit(main())
