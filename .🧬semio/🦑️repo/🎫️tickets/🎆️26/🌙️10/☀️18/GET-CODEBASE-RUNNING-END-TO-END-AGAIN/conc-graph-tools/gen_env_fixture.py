#!/usr/bin/env python3
"""Generates the fixture of the environment laws: extractor vectors answered by real parsers (json, tomllib, re) and environment digests computed by an independent model.

usage: gen_env_fixture.py"""
import hashlib, json, os, re, subprocess, sys

HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(HERE, "staged", "new", "env")


def canon(value):
    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=False)


def sha(text):
    return hashlib.sha256(text.encode("utf8")).hexdigest()


def extract(extractor, text):
    if text is None:
        return None
    if extractor == "project-json":
        try:
            data = json.loads(text)
        except ValueError:
            return {"error": "parse"}
        return {"name": data.get("name"), "nativeRoot": (data.get("metadata") or {}).get("nativeRoot")}
    if extractor == "package-json":
        try:
            data = json.loads(text)
        except ValueError:
            return {"error": "parse"}
        return {"name": data.get("name")}
    if extractor == "go-mod":
        match = re.search(r"^module\s+(\S+)", text, re.M)
        return {"module": match.group(1) if match else None}
    if extractor == "cargo-package":
        script = "const t=JSON.parse(await Bun.stdin.text());console.log(JSON.stringify(Bun.TOML.parse(t).package?.name??null))"
        out = subprocess.run(["bun", "-e", script], input=json.dumps(text), capture_output=True, text=True, check=True).stdout
        return {"name": json.loads(out)}
    raise SystemExit("unknown extractor")


vectors = [
    ("project name and native root", "project-json", '{"name":"a","metadata":{"nativeRoot":"crates/a"}}'),
    ("project without name", "project-json", '{"targets":{}}'),
    ("project not json", "project-json", "{"),
    ("missing manifest", "package-json", None),
    ("package name", "package-json", '{"name":"@x/y","dependencies":{"z":"1"}}'),
    ("package without name", "package-json", '{"private":true}'),
    ("go module", "go-mod", "module example.com/x\n\ngo 1.22\n"),
    ("go module after a comment", "go-mod", "// header\nmodule example.com/y\nrequire a v1\n"),
    ("go without module", "go-mod", "go 1.22\n"),
    ("crate", "cargo-package", '[package]\nname = "my-crate"\nversion = "0.1.0"\n'),
    ("crate after dependencies", "cargo-package", '[dependencies]\nserde = "1"\n\n[package]\nname = "later"\n'),
    ("crate with metadata table", "cargo-package", '[package]\nversion = "1"\nname = "meta"\n\n[package.metadata]\nname = "not-this"\n'),
    ("crate with commented name", "cargo-package", '[package]\n# name = "old"\nname = "new"\n'),
    ("crate with literal string", "cargo-package", "[package]\nname = 'literal'\n"),
    ("crate with windows line endings", "cargo-package", '[package]\r\nname = "crlf"\r\nversion = "1"\r\n'),
    ("workspace manifest without package", "cargo-package", '[workspace]\nmembers = ["a"]\n'),
]
cases = [{"name": name, "extractor": extractor, "text": text, "expected": extract(extractor, text)} for name, extractor, text in vectors]


def environment(tree, projects, manifests, lock, taxonomy):
    owners = sorted([[p["name"], p["root"], p.get("nativeRoot")] for p in projects], key=canon)
    packages, modules = [], []
    for p in projects:
        package = extract("package-json", tree.get(p["root"] + "/package.json"))
        if package and package.get("name"):
            packages.append([package["name"], p["name"]])
        go = extract("go-mod", tree.get(p["root"] + "/go.mod"))
        if go and go.get("module"):
            modules.append([go["module"], p["name"]])
    workspaces = sorted(([path, sha(tree[path])] for path in manifests if path in tree and "[workspace" in tree[path]), key=canon)
    return sha(canon({"owners": owners, "packages": sorted(packages, key=canon), "modules": sorted(modules, key=canon), "workspaces": workspaces, "lock": sha(tree[lock]) if lock in tree else None, "taxonomy": sha(tree[taxonomy]) if taxonomy in tree else None}))


base = {
    "a/📋️project.json": '{"name":"a"}', "a/package.json": '{"name":"@x/a","dependencies":{"b":"1"}}',
    "b/📋️project.json": '{"name":"b","metadata":{"nativeRoot":"crates/b"}}', "b/go.mod": "module example.com/b\n",
    "crates/c/Cargo.toml": '[package]\nname = "c"\n', "Cargo.toml": '[workspace]\nmembers = ["crates/c"]\n', "bun.lock": "lock-1", "tax.json": "{}",
}
projects = [{"name": "a", "root": "a", "config": "a/📋️project.json"}, {"name": "b", "root": "b", "nativeRoot": "crates/b", "config": "b/📋️project.json"}, {"name": "c", "root": "crates/c", "config": "crates/c/Cargo.toml"}]
manifests = ["Cargo.toml", "crates/c/Cargo.toml"]
variants = [("base", {}), ("dependency list edited", {"a/package.json": '{"name":"@x/a","dependencies":{"b":"2"}}'}), ("package renamed", {"a/package.json": '{"name":"@x/renamed"}'}), ("workspace manifest edited", {"Cargo.toml": '[workspace]\nmembers = ["crates/c","crates/d"]\n'}), ("crate manifest without workspace edited", {"crates/c/Cargo.toml": '[package]\nname = "c"\nversion = "2"\n'}), ("lock edited", {"bun.lock": "lock-2"}), ("taxonomy edited", {"tax.json": '{"x":1}'}), ("go module renamed", {"b/go.mod": "module example.com/b2\n"}), ("lock removed", {"bun.lock": None})]
digests = []
for name, change in variants:
    tree = dict(base)
    for path, text in change.items():
        if text is None:
            tree.pop(path, None)
        else:
            tree[path] = text
    digests.append({"name": name, "tree": tree, "digest": environment(tree, projects, manifests, "bun.lock", "tax.json")})
fixture = {"version": 1, "cases": cases, "projects": projects, "workspaceManifests": manifests, "lock": "bun.lock", "taxonomy": "tax.json", "digests": digests}
with open(os.path.join(OUT, "fixture.json"), "w", encoding="utf8") as handle:
    json.dump(fixture, handle, ensure_ascii=False, indent=1, sort_keys=True)
    handle.write("\n")
schema = {"type": "object", "additionalProperties": False, "required": ["version", "cases", "projects", "workspaceManifests", "lock", "taxonomy", "digests"], "properties": {
    "version": {"const": 1},
    "cases": {"type": "array", "items": {"type": "object", "additionalProperties": False, "required": ["name", "extractor", "text", "expected"], "properties": {"name": {"type": "string"}, "extractor": {"enum": ["project-json", "package-json", "go-mod", "cargo-package"]}, "text": {"type": ["string", "null"]}, "expected": {}}}},
    "projects": {"type": "array", "items": {"type": "object", "additionalProperties": False, "required": ["name", "root", "config"], "properties": {"name": {"type": "string"}, "root": {"type": "string"}, "nativeRoot": {"type": "string"}, "config": {"type": "string"}}}},
    "workspaceManifests": {"type": "array", "items": {"type": "string"}}, "lock": {"type": "string"}, "taxonomy": {"type": "string"},
    "digests": {"type": "array", "items": {"type": "object", "additionalProperties": False, "required": ["name", "tree", "digest"], "properties": {"name": {"type": "string"}, "tree": {"type": "object", "additionalProperties": {"type": "string"}}, "digest": {"type": "string", "pattern": "^[a-f0-9]{64}$"}}}}}}
with open(os.path.join(OUT, "fixture-schema.json"), "w", encoding="utf8") as handle:
    json.dump(schema, handle, ensure_ascii=False, indent=1)
    handle.write("\n")
print("cases", len(cases), "digest variants", len(digests))
