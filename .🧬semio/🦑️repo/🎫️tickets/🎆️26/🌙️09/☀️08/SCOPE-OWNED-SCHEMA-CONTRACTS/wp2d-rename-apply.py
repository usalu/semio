#!/usr/bin/env python3
"""🛂️ Applies the row-115 rename: per-case schema authorities take the taxonomy `test-fixture-schema-authority` name.

Usage: wp2d-rename-apply.py <renames.tsv> [--dry]
The TSV comes from `wp2d-rename-plan.py`, itself fed by a `schema check --report` run.
"""
import os, re, subprocess, sys

LIB = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
SPECIAL = {
    f"{LIB}/🧪️tests/🫙️artifact-empty-facet-authority/📐️options.schema.json":
        f"{LIB}/🧪️tests/🫙️artifact-empty-facet-authority/🛂️schema/☑️options.json",
}
SOURCE_SUFFIXES = (".ts", ".tsx", ".json", ".mjs", ".js", ".go", ".md", ".feature", ".jsonc")
# 🔏️path-emoji-statutes/🔣️.json holds synthetic mutation-payload path vectors (the taxonomy `🧬️schema/🔣️.json`
# location), not references to this library's own case authorities, so it is never rewritten.
SKIP = {f"{LIB}/🔣️schema-catalog.json", f"{LIB}/📓️schema-catalog.md", f"{LIB}/🧪️tests/🔏️path-emoji-statutes/🔣️.json"}
LITERAL = re.compile(r"(?<=[\"'])[^\"'`\n]*(?:🧬️schema/🔣️\.json|🧬️schema\.json|🧬️\.schema\.json|🧬️fixture\.schema\.json|📐️options\.schema\.json)(?=[\"'])")


def load(path):
    mapping = {}
    for line in open(path, encoding="utf-8"):
        line = line.rstrip("\n")
        if line and not line.startswith("#"):
            source, target = line.split("\t")
            mapping[source] = SPECIAL.get(source, target)
    return mapping


def library_sources():
    listed = subprocess.run(["git", "ls-files", "-z", LIB], capture_output=True, check=True).stdout
    return [path for path in listed.decode("utf-8").split("\0") if path.endswith(SOURCE_SUFFIXES) and path not in SKIP]


def resolve(base, literal):
    parts = []
    for segment in base.split("/") + literal.split("/"):
        if segment in ("", "."):
            continue
        if segment == "..":
            parts and parts.pop()
            continue
        parts.append(segment)
    return "/".join(parts)


def documents(mapping):
    rows = []
    for source, target in mapping.items():
        parent = os.path.dirname(source)
        tailSource, tailTarget = source[len(parent) + 1:], target[len(parent) + 1:]
        if os.path.basename(source) == "🧬️schema":
            source, target = f"{source}/🔣️.json", f"{target}/🔣️.json"
            tailSource, tailTarget = f"{tailSource}/🔣️.json", f"{tailTarget}/🔣️.json"
        rows.append((source, tailSource, tailTarget))
    return rows


def rewrite(text, path, mapping, rows):
    base, hits = os.path.dirname(path), 0
    for source, target in mapping.items():
        hits += text.count(source)
        text = text.replace(source, target)

    def literal(match):
        nonlocal hits
        value = match.group(0)
        for source, tailSource, tailTarget in rows:
            if value.endswith(tailSource) and resolve(base, value) == source:
                hits += 1
                return value[: len(value) - len(tailSource)] + tailTarget
        return value

    return LITERAL.sub(literal, text), hits


def main():
    mapping, dry = load(sys.argv[1]), "--dry" in sys.argv
    rows = documents(mapping)
    for source, target in mapping.items():
        if not dry:
            os.makedirs(os.path.dirname(target), exist_ok=True)
            os.rename(source, target)
        print(f"mv {source} -> {target}")
    touched = 0
    for path in library_sources():
        if not os.path.exists(path):
            continue
        before = open(path, encoding="utf-8").read()
        after, hits = rewrite(before, path, mapping, rows)
        if after != before:
            touched += 1
            print(f"edit {path} ({hits})")
            if not dry:
                open(path, "w", encoding="utf-8").write(after)
    print(f"# {len(mapping)} renames, {touched} files rewritten")


main()
