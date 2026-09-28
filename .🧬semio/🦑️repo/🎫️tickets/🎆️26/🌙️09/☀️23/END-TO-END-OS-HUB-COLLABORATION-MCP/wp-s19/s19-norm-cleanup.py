#!/usr/bin/env python3
"""🧹️ S19 one-off codemod (set `norm-cleanup`): drops en1994's hand-kept `@deprecated` `En1994Artifact = En1994Snapshot`
alias from the snapshot TS (zero importers: `git grep En1994Artifact` hits only the schema's and diff's own interfaces).
Idempotent. usage: s19-norm-cleanup.py <root>"""
import os
import sys

root = sys.argv[1]
SNAPSHOT = "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts"
path = os.path.join(root, SNAPSHOT)
text = open(path, encoding="utf-8").read()
alias = "\n/** @deprecated Use En1994Snapshot */\nexport type En1994Artifact = En1994Snapshot;\n"
if alias in text:
    open(path, "w", encoding="utf-8").write(text.replace(alias, "\n", 1).rstrip("\n") + "\n")
    print(f"edited {SNAPSHOT}")
