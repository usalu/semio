import json
import os
import re
import sys
import time

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), *[".."] * 8))
SCHEMA = os.path.join(ROOT, "🧰️framework", "🛍️products", "🦑️repo", "🔨️modules", "📚️library", "⚡️caching", "🚀️bootstrap", "📦️dependencies", "🧾️generation", "🧬️schema", "🔣️.json")
LITERAL_ASSET = re.compile(r"""(["'])(\.{1,2}/[^"'\\\r\n]+\.(?:[cm]?[jt]sx?|json))\1""")
IMPORT_SPECIFIER = re.compile(r"""(?:from\s*|import\s*\(\s*|require\(\s*|import\s+)["']([^"']+)["']""")
SCHEMA_DIRECTORY = "\U0001F9EC️" + "schema"


def entries():
    schema = json.load(open(SCHEMA, encoding="utf-8"))
    return schema["$defs"]["Protocol"]["const"]["implementation"]["entries"]


def closure():
    seen, pending = {}, list(entries())
    while pending:
        path = pending.pop()
        if path in seen:
            continue
        full = os.path.join(ROOT, path)
        if not os.path.isfile(full):
            continue
        seen[path] = os.lstat(full)
        text = open(full, encoding="utf-8", errors="replace").read()
        directory = os.path.dirname(path)
        schema_dir = os.path.join(directory, SCHEMA_DIRECTORY)
        if os.path.isdir(os.path.join(ROOT, schema_dir)):
            for name in os.listdir(os.path.join(ROOT, schema_dir)):
                pending.append(os.path.join(schema_dir, name))
        for match in list(IMPORT_SPECIFIER.finditer(text)) + list(LITERAL_ASSET.finditer(text)):
            specifier = match.group(2) if match.re is LITERAL_ASSET else match.group(1)
            if not specifier.startswith("."):
                continue
            target = os.path.normpath(os.path.join(directory, specifier))
            if target.endswith((".ts", ".json", ".cjs", ".js", ".mjs")):
                pending.append(target)
    return seen


def main():
    rows = closure()
    now = time.time()
    buckets = [(3600, "1h"), (6 * 3600, "6h"), (24 * 3600, "24h"), (7 * 86400, "7d")]
    print("closure files", len(rows))
    for limit, label in buckets:
        changed = [p for p, s in rows.items() if now - max(s.st_mtime, s.st_ctime) < limit]
        print("changed within", label, len(changed))
    groups = {}
    for path in rows:
        key = "/".join(path.split("/")[:3])
        groups[key] = groups.get(key, 0) + 1
    for key, count in sorted(groups.items(), key=lambda row: -row[1])[:8]:
        print("group", count, key)
    if "--list" in sys.argv:
        for path, stat in sorted(rows.items(), key=lambda row: -max(row[1].st_mtime, row[1].st_ctime))[:40]:
            print(time.strftime("%m-%d %H:%M:%S", time.localtime(max(stat.st_mtime, stat.st_ctime))), path)


main()
