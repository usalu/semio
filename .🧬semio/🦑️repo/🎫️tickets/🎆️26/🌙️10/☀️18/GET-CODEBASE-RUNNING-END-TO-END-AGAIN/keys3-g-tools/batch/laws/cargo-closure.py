#!/usr/bin/env python3
"""keys3-g independent oracle for the cargo resolution law: from the REAL Cargo.lock of the workspace that owns a crate, the crate's lock closure by name. Pure standard library and its own line reader (Cargo.lock has a closed, line-oriented syntax; `tomllib` needs Python 3.11), no repository code.
Prints JSON {lock, package, closureSize, inside: [3 unique leaf names inside the closure], outside: [3 unique names outside]}.
usage: cargo-closure.py <repo root> <crate dir relative to the root>"""
import json, os, re, sys
root, crate = sys.argv[1:3]
def text(path):
    with open(os.path.join(root, path), encoding="utf8") as handle: return handle.read()
def has_workspace(path): return re.search(r"^\[workspace\]\s*$", text(path), re.M) is not None
def owner(directory):
    path = directory
    while True:
        candidate = (path + "/" if path else "") + "Cargo.toml"
        if os.path.exists(os.path.join(root, candidate)) and has_workspace(candidate): return path
        if not path: raise SystemExit("no workspace owner")
        path = os.path.dirname(path)
workspace = owner(crate)
lock_path = (workspace + "/" if workspace else "") + "Cargo.lock"
name = re.search(r'^\[package\][^\[]*?^name\s*=\s*"([^"]+)"', text(crate + "/Cargo.toml"), re.M | re.S).group(1)
entries = []
for block in re.split(r"^\[\[package\]\]\s*$", text(lock_path), flags=re.M)[1:]:
    deps = re.search(r"^dependencies = \[(.*?)^\]", block, re.M | re.S)
    entries.append({"name": re.search(r'^name = "([^"]+)"', block, re.M).group(1), "version": re.search(r'^version = "([^"]+)"', block, re.M).group(1), "source": (re.search(r'^source = "([^"]+)"', block, re.M) or [None, None])[1] if re.search(r'^source = "', block, re.M) else None, "deps": re.findall(r'"([^"]+)"', deps.group(1)) if deps else []})
by_name = {}
for entry in entries: by_name.setdefault(entry["name"], []).append(entry)
def resolve(dep):
    m = re.match(r"^(\S+)(?: (\S+))?(?: \((.+)\))?$", dep)
    found = [e for e in by_name.get(m.group(1), []) if (m.group(2) is None or e["version"] == m.group(2)) and (m.group(3) is None or e["source"] == m.group(3))]
    assert len(found) == 1, dep
    return found[0]
root_entry = [e for e in by_name[name] if e["source"] is None][0]
closure, queue = {id(root_entry): root_entry}, [root_entry]
while queue:
    for dep in queue.pop()["deps"]:
        target = resolve(dep)
        if id(target) not in closure: closure[id(target)] = target; queue.append(target)
inside_ids = set(closure)
unique = {n for n, rows in by_name.items() if len(rows) == 1 and rows[0]["source"] is not None}
inside = sorted(e["name"] for e in closure.values() if e["name"] in unique and not e["deps"])[:3]
outside = sorted(n for n in unique if id(by_name[n][0]) not in inside_ids)[:3]
closure_size = len(closure)
print(json.dumps({"lock": lock_path, "package": name, "closureSize": closure_size, "inside": inside, "outside": outside}, ensure_ascii=False))
