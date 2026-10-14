#!/usr/bin/env python3
"""🧪️ Turns a mutations crate's mounted fixture test module (`🧬️mutations/🧪️tests/🔬️fixture/🦀️.rs`, one `#[path] mod`
per mutation fixture case) into integration targets of at most 31 files each, grouped by verb family (lifecycle:
create/delete/insert/remove/add/duplicate; edit: everything else), then removes the fixture module and its mount.

usage: 🧪️fixture-targets.py <mutations dir (repo-rel)> <crate label for docs> plan|apply
"""
import json, os, re, subprocess, sys
HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *[".."] * 8))
mdir, label, mode = sys.argv[1], sys.argv[2], sys.argv[3]
base = os.path.join(REPO, mdir)
fixture = os.path.join(base, "🧪️tests", "🔬️fixture", "🦀️.rs")
text = open(fixture, encoding="utf-8").read()
cases = []
for m in re.finditer(r'#\[path\s*=\s*"([^"]+)"\]\s*mod\s+(\w+)\s*;', text):
    path = os.path.normpath(os.path.join(os.path.dirname(fixture), m.group(1)))
    kind = re.sub(r"^[^a-z]+", "", os.path.relpath(path, base).split("/")[0])
    embeds = len(re.findall(r'include_(?:str|bytes)\s*!', open(path, encoding="utf-8").read()))
    cases.append((kind, path, 1 + embeds))
LIFECYCLE = ("create", "delete", "insert", "remove", "add", "duplicate", "append", "drop")
groups = {"lifecycle": [], "edit": []}
for case in sorted(cases):
    groups["lifecycle" if case[0].split("-")[0] in LIFECYCLE else "edit"].append(case)
targets = []
for family, members in groups.items():
    chunks, current, files = [], [], 0
    for case in members:
        if current and files + case[2] > 30:
            chunks.append(current); current, files = [], 0
        current.append(case); files += case[2]
    if current:
        chunks.append(current)
    for i, chunk in enumerate(chunks):
        name = family if len(chunks) == 1 else f"{family}-{chunk[0][0].split('-')[0]}" if len({c[0].split('-')[0] for c in chunk}) == 1 else f"{family}-{i + 1}"
        kinds = ", ".join(c[0] for c in chunk)
        targets.append({"name": name.replace("-", "_") + "_mutations", "root": os.path.relpath(os.path.join(base, "🧪️tests", "🔬️" + name, "🦀️.rs"), REPO),
                        "doc": f"//! 🧪️ Fixture laws of the {label} {family} mutations ({kinds}), over the mutation crate's public API.",
                        "tests": [re.escape(os.path.relpath(c[1], REPO)) + "$" for c in chunk]})
spec = {"manifest": os.path.relpath(os.path.join(base, "📦️packages", "🦀️rust", "Cargo.toml"), REPO), "targets": targets}
spec_path = os.path.join(HERE, "🗑️generated", re.sub(r"\W", "_", label) + "-fixture-targets.json")
json.dump(spec, open(spec_path, "w"), ensure_ascii=False, indent=1)
for t in targets:
    print(t["name"], len(t["tests"]))
if mode == "apply":
    subprocess.run([sys.executable, os.path.join(HERE, "🧪️integration-tests.py"), spec_path, "apply"], check=True)
    for target in targets:
        root = os.path.join(REPO, target["root"])
        body = open(root, encoding="utf-8").read()
        body = re.sub(r"\nmod fixture_tests_", "\nmod ", body)
        open(root, "w", encoding="utf-8").write(body)
    component = os.path.join(base, "🦀️.rs")
    ctext = open(component, encoding="utf-8").read()
    ctext = re.sub(r'(?m)^(?:///[^\n]*\n)*#\[cfg\(test\)\]\n#\[path = "🧪️tests/🔬️fixture/🦀️\.rs"\]\nmod \w+;\n', "", ctext)
    ctext = re.sub(r"//#region 🧪️FixtureTests\n//#endregion 🧪️FixtureTests\n?", "", ctext)
    open(component, "w", encoding="utf-8").write(re.sub(r"\n{3,}", "\n\n", ctext))
    os.remove(fixture)
    os.rmdir(os.path.dirname(fixture))
