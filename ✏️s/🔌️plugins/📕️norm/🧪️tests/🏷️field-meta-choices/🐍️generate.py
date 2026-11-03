#!/usr/bin/env python3
"""🎲 Field-meta choice generator: rewrites the body of every `NormFieldChoice` table whose fields edit a schema enum, from the enum members and the `x-semio-ui.options` EN/DE labels of the schema.

Usage: python3 -I 🐍️generate.py [--artifact en1995 ...] [--skip en1994 ...] [--dry]
A table shared by fields with different enums is reported and left alone; a table that edits a plain schema string (no enum) stays hand-written.
"""
import os, sys
sys.dont_write_bytecode = True
import importlib.util
_spec = importlib.util.spec_from_file_location("choices", os.path.join(os.path.dirname(os.path.abspath(__file__)), "🐍️choices.py"))
c = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(c)


def main():
    args = sys.argv[1:]
    only = {args[i + 1] for i, a in enumerate(args) if a == "--artifact"}
    skip = {args[i + 1] for i, a in enumerate(args) if a == "--skip"}
    changed = 0
    for key, root in c.artifact_dirs(only):
        if key in skip:
            continue
        schemas = c.load_schemas(root)
        for meta in c.meta_files(root):
            text = open(meta, encoding="utf-8").read()
            consts, uses = c.parse_meta(text)
            literal = "const fn choice" not in text
            for name, paths in uses.items():
                enums = [next((n for n in (c.enum_node(d, c.norm_path(p)) for d in schemas) if n), None) for p in paths]
                enums = [e for e in enums if e is not None]
                if not enums or any(e["enum"] != enums[0]["enum"] for e in enums):
                    print(f"[{key}] {name}: kept (no single schema enum for {paths[:3]})")
                    continue
                rows, fallback = c.expected_rows(enums[0], consts[name])
                if fallback:
                    print(f"[{key}] {name}: tokens without any label {fallback}")
                body = "\n" + c.render(rows, literal) + "\n"
                pattern = c.CONST
                for m in pattern.finditer(text):
                    if m.group(2) == name and m.group(3) != body:
                        text = text[: m.start(3)] + body + text[m.end(3):]
                        changed += 1
                        print(f"[{key}] {name}: regenerated ({len(rows)} rows)")
                        break
            if not "--dry" in args:
                open(meta, "w", encoding="utf-8").write(text)
    print(f"tables regenerated={changed}")


if __name__ == "__main__":
    main()
