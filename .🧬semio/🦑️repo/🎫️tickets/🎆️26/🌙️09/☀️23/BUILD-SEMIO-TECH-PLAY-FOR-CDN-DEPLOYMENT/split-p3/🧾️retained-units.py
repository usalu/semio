#!/usr/bin/env python3
"""🧾️ Re-owns the dev composition's retained artifact law units after a family split.

For every retained unit of `✏️s/🧑‍💻dev/🧩️composition/🧪️tests/🧫️fixtures/🔣️.json` it finds the family package and
Cargo target (lib or `[[test]]`) whose module tree compiles the unit's file, rewrites each law to the file's module
path in that target, records `package` and `target`, and refreshes the file digest.

usage: 🧾️retained-units.py <family workspace Cargo.toml> plan|apply
"""
import hashlib, json, os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *[".."] * 8))
ns = {}
exec(compile(open(os.path.join(HERE, "🕸️module-graph.py"), encoding="utf-8").read().rsplit("\nmain()", 1)[0], "module-graph", "exec"), ns)
Crate = ns["Crate"]
FIXTURE = os.path.join(REPO, "✏️s/🧑‍💻dev/🧩️composition/🧪️tests/🧫️fixtures/🔣️.json")


def targets(ws):
    ws_dir = os.path.dirname(ws)
    for member in re.findall(r'"([^"]+)"', re.search(r"(?s)members = \[(.*?)\]", open(ws, encoding="utf-8").read()).group(1)):
        path = os.path.join(ws_dir, member, "Cargo.toml")
        text = open(path, encoding="utf-8").read()
        name = re.search(r'(?m)^name\s*=\s*"([^"]+)"', text).group(1)
        lib = re.search(r'(?ms)^\[lib\][^\[]*?^path\s*=\s*"([^"]+)"', text)
        yield name, {"kind": "lib", "name": name.replace("-", "_")}, os.path.normpath(os.path.join(os.path.dirname(path), lib.group(1)))
        for t in re.finditer(r'(?ms)^\[\[test\]\]\s*\nname\s*=\s*"([^"]+)"\s*\npath\s*=\s*"([^"]+)"', text):
            yield name, {"kind": "test", "name": t.group(1)}, os.path.normpath(os.path.join(os.path.dirname(path), t.group(2)))


def main():
    ws, mode = sys.argv[1], sys.argv[2]
    owners = {}
    for package, target, root in targets(os.path.abspath(ws)):
        for f, mp in Crate(root).files.items():
            owners.setdefault(f, (package, target, tuple(s for s in mp if s != "component")))
    fixture = json.load(open(FIXTURE, encoding="utf-8"))
    for unit in fixture["retainedArtifactUnits"]:
        path = os.path.normpath(os.path.join(REPO, unit["path"]))
        package, target, mp = owners[path]
        source = open(path, encoding="utf-8").read()
        unit["package"], unit["target"] = package, target
        unit["sha256"] = hashlib.sha256(source.encode("utf-8")).hexdigest()
        unit["laws"] = ["::".join(mp + (law.split("::")[-1],)) for law in unit["laws"]]
        print(package, target["kind"], target["name"], unit["laws"][0])
    if mode == "apply":
        ordered = [{k: u[k] for k in ("path", "package", "target", "sha256", "laws")} for u in fixture["retainedArtifactUnits"]]
        fixture["retainedArtifactUnits"] = ordered
        open(FIXTURE, "w", encoding="utf-8").write(json.dumps(fixture, ensure_ascii=False, indent=2) + "\n")


main()
