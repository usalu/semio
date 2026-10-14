#!/usr/bin/env python3
"""🧪️ Turns the mutation-kind fixture tests that the cut mounted as unit tests of the union crate into integration test
targets of that crate (P5 decisions D4 and D5): removes their `#[cfg(test)]` mounts from the union root, writes one test
root per kind group, rewrites `crate::` to the union crate, and reads every committed fixture at test runtime through
`semio_s_fem_test_support::fixture!` (paths relative to the union package) instead of embedding it.

usage: 🧪️kind-tests.py <config.json> plan|apply
config: {"union_root": <crate root>, "union_crate": <ident>, "union_package_dir": <dir>, "model_crate": <ident>,
         "subsets": <subsets dir>, "groups": [{"name", "root", "doc", "kinds": [<kind dir name>...]}],
         "laws": {<kind module>: <laws file>}}
"""
import json, os, re, sys
REPO = "/Users/ueli/Documents/semio"
cfg = json.load(open(sys.argv[1], encoding="utf-8")); mode = sys.argv[2]
union_root = os.path.join(REPO, cfg["union_root"]); pkg = os.path.join(REPO, cfg["union_package_dir"]); subsets = os.path.join(REPO, cfg["subsets"])
root = open(union_root, encoding="utf-8").read()
mount = re.compile(r'(?m)^[ \t]*#\[cfg\(test\)\]\n[ \t]*#\[path = "([^"]+)"\]\n[ \t]*mod (\w+);\n')
mounts = []
for m in mount.finditer(root):
    target = os.path.normpath(os.path.join(os.path.dirname(union_root), m.group(1)))
    if "/🧬️mutations/" in target and "/🧪️tests/" in target and "/🌐️any/" not in target:
        mounts.append((m.group(0), target, m.group(2)))
print("kind test mounts", len(mounts))
new_root = root
for text, _, _ in mounts: new_root = new_root.replace(text, "", 1)
while True:
    pruned = re.sub(r'(?m)^[ \t]*#\[path = "\."\]\n[ \t]*pub mod \w+ \{\n[ \t]*\}\n', "", new_root)
    if pruned == new_root: break
    new_root = pruned
by_kind = {}
for _, target, name in mounts:
    kind = target.split("/🧬️mutations/")[1].split("/")[0]
    by_kind.setdefault(kind, []).append((target, name))
tests = []
uc = cfg["union_crate"]
for g in cfg["groups"]:
    groot = os.path.join(subsets, g["root"])
    lines = [g["doc"], "", "extern crate semio_framework_os_kernel as protocol;", ""]
    count = 1
    for kind in g["kinds"]:
        mod = kind.split("️")[-1].replace("-", "_")
        if mod in cfg.get("laws", {}):
            law = os.path.join(subsets, cfg["laws"][mod])
            lines += [f'#[path = "{os.path.relpath(law, os.path.dirname(groot))}"]', "mod laws;"]; count += 1
        for target, name in sorted(by_kind.pop(kind, [])):
            lines += [f'#[path = "{os.path.relpath(target, os.path.dirname(groot))}"]', f"mod {mod}_{name.removeprefix('tests_')};"]; count += 1
    print(g["name"], "files", count)
    assert count <= 32, g["name"]
    tests.append((g["name"], os.path.relpath(groot, pkg), groot, "\n".join(lines) + "\n"))
assert not by_kind, f"ungrouped kinds {sorted(by_kind)}"
CONST = re.compile(r'(?m)^(?P<vis>pub(?:\([^)]*\))? )?const (?P<name>[A-Z][A-Z0-9_]*): &str = include_str!\("(?P<path>[^"]+)"\);\n')
def convert(path):
    s = open(path, encoding="utf-8").read()
    names = []
    def rep(m):
        fixture = os.path.normpath(os.path.join(os.path.dirname(path), m.group("path")))
        assert os.path.exists(fixture), fixture
        names.append(m.group("name"))
        return f'{m.group("vis") or ""}static {m.group("name")}: std::sync::LazyLock<String> = semio_s_fem_test_support::fixture!("{os.path.relpath(fixture, pkg)}");\n'
    s = CONST.sub(rep, s)
    out = []
    for line in s.split("\n"):
        if not line.lstrip().startswith("//") and not line.lstrip().startswith("static "):
            for n in names:
                line = re.sub(r"(?<![\w:\"])" + n + r"\b(?![\w:])", n + ".as_str()", line)
        out.append(line)
    s = "\n".join(out)
    s = re.sub(r"use [\w:]+::mutations::move_selection::laws;", "use crate::laws;", s)
    s = re.sub(r"(?<![\w:])crate::(?!laws\b)", uc + "::", s)
    return s
edits = {}
for _, target, _ in mounts: edits[target] = convert(target)
for law in cfg.get("laws", {}).values():
    p = os.path.join(subsets, law); s = open(p, encoding="utf-8").read()
    s = s.replace("crate::standards::v1::subsets::any::schema::diff::", cfg["model_crate"] + "::standards::v1::subsets::any::schema::diff::")
    s = s.replace("crate::standards::v1::subsets::any::schema::snapshot::", cfg["model_crate"] + "::standards::v1::subsets::any::schema::snapshot::")
    s = re.sub(r"(?<![\w:])crate::", uc + "::", s)
    edits[p] = s
print("test files", len(edits))
if mode == "apply":
    open(union_root, "w", encoding="utf-8").write(new_root)
    for p, s in edits.items(): open(p, "w", encoding="utf-8").write(s)
    for name, rel, groot, text in tests:
        os.makedirs(os.path.dirname(groot), exist_ok=True); open(groot, "w", encoding="utf-8").write(text)
    json.dump([{"name": n, "path": r} for n, r, _, _ in tests], open(os.path.join(os.path.dirname(os.path.abspath(sys.argv[1])), "🗑️generated", os.path.basename(sys.argv[1]).replace(".json", ".targets.json")), "w"), indent=1, ensure_ascii=False)
