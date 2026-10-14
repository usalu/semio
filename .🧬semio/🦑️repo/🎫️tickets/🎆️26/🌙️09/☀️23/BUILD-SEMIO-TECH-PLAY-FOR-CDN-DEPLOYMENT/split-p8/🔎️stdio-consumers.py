#!/usr/bin/env python3
"""🔎️ Indexes every Rust source under the consumer roots that names a `semio_s_artifact_stdio_*` crate ident (one read
per file), writing `🗑️generated/stdio-consumers.json` = {ident: [repo-relative file...]}.

usage: 🔎️stdio-consumers.py [root...]   (default roots: ✏️s 🌎️hub 🧰️framework ♻️mit-bestand 🎓️teaching)
"""
import collections, json, os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *[".."] * 8))
roots = sys.argv[1:] or ["✏️s", "🌎️hub", "🧰️framework", "♻️mit-bestand", "🎓️teaching"]
SKIP = {"node_modules", "target", "🗑️generated", ".git", "dist", ".nx"}
index = collections.defaultdict(list)
pattern = re.compile(r"\bsemio_s_artifact_stdio_[a-z0-9_]+")
for root in roots:
    for dirpath, dirnames, filenames in os.walk(os.path.join(REPO, root)):
        dirnames[:] = [d for d in dirnames if d not in SKIP and not d.startswith(".")]
        for name in filenames:
            if name.endswith(".rs"):
                path = os.path.join(dirpath, name)
                try:
                    text = open(path, encoding="utf-8").read()
                except (UnicodeDecodeError, OSError):
                    continue
                for ident in set(pattern.findall(text)):
                    index[ident].append(os.path.relpath(path, REPO))
json.dump({k: sorted(v) for k, v in sorted(index.items())}, open(os.path.join(HERE, "🗑️generated", "stdio-consumers.json"), "w"), indent=0, ensure_ascii=False)
for k, v in sorted(index.items()):
    print(len(v), k)
