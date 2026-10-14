#!/usr/bin/env python3
"""🗂️ Registers directory member names in one `semanticDirectoryMemberKinds` registry of `🔣️taxonomy.json` with a
minimal textual insertion (the file is shared by concurrent lanes; it is re-read right before the write and never
reformatted). Names already registered are skipped.

usage: 🗂️taxonomy-register.py <registry id> <member name>...
"""
import json, os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *[".."] * 8))
PATH = os.path.join(REPO, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json")
registry, names = sys.argv[1], sys.argv[2:]
text = open(PATH, encoding="utf-8").read()
current = json.loads(text)["semanticDirectoryMemberKinds"][registry]["memberNames"]
missing = [n for n in names if n not in current]
if missing:
    head = re.search(r'(?m)^    "' + re.escape(registry) + r'": \{\n(?:.*\n)*?      "memberNames": \[\n', text)
    rows = "".join(f"        {json.dumps(n, ensure_ascii=False)},\n" for n in missing)
    text = text[:head.end()] + rows + text[head.end():]
    json.loads(text)
    open(PATH, "w", encoding="utf-8").write(text)
print(f"{registry}: added {missing}")
