#!/usr/bin/env python3
# 🌐️ Read-only check of localized JSON tables in the repo build library: every {en,de} leaf must carry every declared language.
# Usage: python3 check_tables.py <file-list> <repo-root>
import json
import sys
from collections import Counter

LIB_PREFIX = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
LANG_KEYS = {"en", "de"}
TABLE_KEYS = {"labels", "messages", "texts", "summary", "description", "label", "title", "notes", "refusal"}


def leaves(node, path):
    if isinstance(node, dict):
        keys = set(node.keys())
        if keys & LANG_KEYS and all(isinstance(v, str) for v in node.values()):
            yield path, node
            return
        for key, value in node.items():
            yield from leaves(value, path + [key])
    elif isinstance(node, list):
        for index, value in enumerate(node):
            yield from leaves(value, path + [str(index)])


def main():
    files_list, root = sys.argv[1], sys.argv[2]
    paths = [line.strip() for line in open(files_list, encoding="utf-8") if line.strip().endswith(".json")]
    findings = []
    counts = Counter()
    for rel in paths:
        if "/🧪️" in rel or "/🧫️" in rel:
            continue
        try:
            data = json.load(open(root + "/" + rel, encoding="utf-8"))
        except (OSError, ValueError):
            continue
        short = rel[len(LIB_PREFIX):] if rel.startswith(LIB_PREFIX) else rel
        declared = None
        if isinstance(data, dict) and "languages" in data:
            declared = data["languages"]
            if declared not in (["en", "de"],):
                findings.append((short, "-", "languages declared as %s (expected [\"en\", \"de\"], EN first)" % json.dumps(declared, ensure_ascii=False)))
        expected = set(declared or ["en", "de"])
        if isinstance(data, dict):
            for table in TABLE_KEYS & set(data.keys()):
                value = data[table]
                if isinstance(value, dict):
                    for key, leaf in value.items():
                        if isinstance(leaf, str):
                            findings.append((short, table + "." + key, "monolingual plain string in localized table"))
        for path, leaf in leaves(data, []):
            counts["leaves"] += 1
            have = set(leaf.keys())
            missing = expected - have
            extra = have - expected
            where = ".".join(path)
            if missing:
                findings.append((short, where, "missing " + ",".join(sorted(missing)) + " (has " + ",".join(sorted(have)) + ")"))
            if extra:
                findings.append((short, where, "extra language(s) " + ",".join(sorted(extra))))
            empties = [k for k, v in leaf.items() if v.strip() == ""]
            if empties and len(empties) < len(leaf):
                findings.append((short, where, "empty text for " + ",".join(sorted(empties))))
            if empties and len(empties) == len(leaf):
                counts["all-empty-leaves"] += 1
    for item in findings:
        print("\t".join(item))
    print("# leaves", counts["leaves"], "all-empty", counts["all-empty-leaves"], "findings", len(findings), file=sys.stderr)


if __name__ == "__main__":
    main()
