#!/usr/bin/env python3
"""🏷️ Field-meta choice witness: every choice token of a properties-panel picker must be a member of the schema enum the field edits, and every enum member must be offered.

Usage: python3 -I 🐍️check.py [--artifact en1995 ...] [--verbose]
Exit code 1 when a token is no enum member, an enum member has no choice, or a generated table differs from the one `🐍️generate.py` renders (tokens and EN/DE labels from the schema options).
"""
import os, sys
sys.dont_write_bytecode = True
import importlib.util
_spec = importlib.util.spec_from_file_location("choices", os.path.join(os.path.dirname(os.path.abspath(__file__)), "🐍️choices.py"))
c = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(c)


def check(key, root, verbose):
    problems, seen, plain = [], 0, 0
    schemas = c.load_schemas(root)
    for meta in c.meta_files(root):
        consts, uses = c.parse_meta(open(meta, encoding="utf-8").read())
        for name, paths in uses.items():
            rows = consts[name]
            tokens = [r[0] for r in rows]
            for path in paths:
                enum = next((n for n in (c.enum_node(d, c.norm_path(path)) for d in schemas) if n), None)
                seen += 1
                if enum is None:
                    plain += 1
                    if verbose:
                        print(f"[{key}] note: {path} ({name}) has no schema enum")
                    continue
                bad = [t for t in tokens if t not in enum["enum"]]
                missing = [e for e in enum["enum"] if e not in tokens]
                if bad or missing:
                    problems.append(f"[{key}] {path} ({name}): tokens not in the enum {bad}; enum members without choice {missing}")
                elif tokens != enum["enum"]:
                    problems.append(f"[{key}] {path} ({name}): choice order differs from the enum {enum['enum']}")
                elif c.expected_rows(enum, rows)[0] != rows:
                    problems.append(f"[{key}] {path} ({name}): labels differ from the schema options (run 🐍️generate.py)")
    return problems, seen, plain


def main():
    args = sys.argv[1:]
    only = {args[i + 1] for i, a in enumerate(args) if a == "--artifact"}
    total, found = 0, []
    for key, root in c.artifact_dirs(only):
        problems, seen, plain = check(key, root, "--verbose" in args)
        total += seen
        found += problems
        print(f"[{key}] choice fields={seen} schema-string fields={plain} mismatches={len(problems)}")
    for p in found:
        print(p)
    print(f"choices={total} mismatches={len(found)}")
    return 1 if found else 0


if __name__ == "__main__":
    sys.exit(main())
