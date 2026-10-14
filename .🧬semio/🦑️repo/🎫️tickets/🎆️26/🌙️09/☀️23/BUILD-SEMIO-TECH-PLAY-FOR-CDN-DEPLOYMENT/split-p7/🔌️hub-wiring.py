#!/usr/bin/env python3
"""🔌️ Points one hub composition at the owning family crates: every `crate::editor::<a>::X`, `crate::viewer::<a>::X` and
`crate::artifacts::<a>::<path>` in the composition's plugin root and tests is replaced by the path of the family crate
that owns it (per the replacement table), and the package root's `artifacts`/`editor`/`viewer` wiring facades are removed.

usage: 🔌️hub-wiring.py <hub composition dir (repo-relative)> <json table {"old path": "new path"}>
"""
import json, os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *[".."] * 8))
root = os.path.join(REPO, sys.argv[1])
table = json.loads(sys.argv[2])
for dirpath, _, files in os.walk(root):
    if "📦️packages" in dirpath.split(os.sep) or "🧩️component" in dirpath.split(os.sep):
        continue
    for name in files:
        if name != "🦀️.rs":
            continue
        path = os.path.join(dirpath, name)
        text = open(path, encoding="utf-8").read()
        new = text
        for old in sorted(table, key=len, reverse=True):
            new = new.replace(old, table[old])
        left = re.findall(r"crate::(?:editor|viewer|artifacts)::[A-Za-z_:]+", new)
        if left:
            print(f"LEFT {os.path.relpath(path, REPO)}: {sorted(set(left))}")
        if new != text:
            open(path, "w", encoding="utf-8").write(new)
            print(f"rewired {os.path.relpath(path, REPO)}")
package = os.path.join(root, "📦️packages/🦀️rust/🦀️.rs")
text = open(package, encoding="utf-8").read()
new = re.sub(r"(?s)//#region 🗿️Artifacts.*?//#endregion 👁️Viewer\n\n?", "", text)
new = re.sub(r"(?s)// 🧯️ `clippy::result_large_err`.*?#\[allow\(clippy::result_large_err\)\]\n", "", new)
if new != text:
    open(package, "w", encoding="utf-8").write(new)
    print(f"dropped facades in {os.path.relpath(package, REPO)}")
