#!/usr/bin/env python3
"""🧪️ Splits one kind-test integration root (over the byte ceiling) into roots by kind (P5): every `#[path] mod`
entry whose path names one of a group's kind folders moves to that group's new root; the old root is removed.

usage: 🧪️kind-tests-regroup.py <old root> <new root>=<doc>=<kind folder>,<kind folder>... ...
"""
import os, re, sys
old = sys.argv[1]; src = open(sys.argv[1] if not os.environ.get("REGROUP_SOURCE") else os.environ["REGROUP_SOURCE"], encoding="utf-8").read()
head = src[:src.index("#[path")]
head = head[head.index("\n") + 1:]
entries = re.findall(r'(#\[path = "([^"]+)"\]\nmod \w+;\n)', src)
taken = 0
for spec in sys.argv[2:]:
    root, doc, kinds = spec.split("=", 2)
    kinds = kinds.split(",")
    mine = [e for e, p in entries if any("/" + k + "/" in p for k in kinds)]
    taken += len(mine)
    rel = os.path.relpath(os.path.dirname(old), os.path.dirname(root))
    body = "".join(re.sub(r'#\[path = "([^"]+)"\]', lambda m: '#[path = "' + os.path.normpath(os.path.join(rel, m.group(1))) + '"]', e) for e in mine)
    os.makedirs(os.path.dirname(root), exist_ok=True)
    open(root, "w", encoding="utf-8").write(doc + "\n" + head + body)
assert taken == len(entries), (taken, len(entries))
new_roots = [spec.split("=", 1)[0] for spec in sys.argv[2:]]
if old not in new_roots:
    os.remove(old); os.rmdir(os.path.dirname(old))
print("split", len(entries))
