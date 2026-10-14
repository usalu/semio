#!/usr/bin/env python3
"""🩹️ Adds the dependency rows a family cut's compile log proves missing (split lane P7).

Reads a `cargo check --message-format=short` log, finds every unresolved crate (`E0433` "cannot find module or crate
`X`", "could not find `X` in the crate root/list of imported crates", `E0463` "can't find crate for `X`") in a source
of a family crate, maps `X` to a dependency of the original monolith (by extern name or `extern crate … as` alias) or
to a family crate, and adds `<name> = { workspace = true }` to that crate's `[dependencies]` (`[dev-dependencies]`
for test code), with the monolith's features for that dependency kind.

usage: 🩹️dep-fixer.py <config.json> <log> plan|apply
"""
import collections, json, os, re, subprocess, sys

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *[".."] * 8))
ERRORS = [re.compile(r"^(?P<file>[^:\s][^:]*):\d+:\d+: error\[E0433\]: (?:cannot find module or crate|failed to resolve: use of unresolved module or unlinked crate) `(?P<name>\w+)`"),
          re.compile(r"^(?P<file>[^:\s][^:]*):\d+:\d+: error\[E0433\]: cannot find `(?P<name>\w+)` in the crate root"),
          re.compile(r"^(?P<file>[^:\s][^:]*):\d+:\d+: error\[E0433\]: failed to resolve: could not find `(?P<name>\w+)` in the list of imported crates"),
          re.compile(r"^(?P<file>[^:\s][^:]*):\d+:\d+: error\[E0463\]: can't find crate for `(?P<name>\w+)`"),
          re.compile(r"^(?P<file>[^:\s][^:]*):\d+:\d+: error\[E0432\]: unresolved import `(?P<name>\w+)`")]


def toml_loads(text):
    return json.loads(subprocess.run(["bun", "-e", "const t=await Bun.stdin.text();console.log(JSON.stringify(Bun.TOML.parse(t)))"], input=text, capture_output=True, text=True, check=True).stdout)


def add(text, section, line, name):
    match = re.search(r"(?ms)^\[" + re.escape(section) + r"\]\s*$(.*?)(?=^\[|\Z)", text)
    if not match:
        return text.rstrip("\n") + f"\n\n[{section}]\n{line}\n"
    if re.search(r"(?m)^" + re.escape(name) + r"\s*=", match.group(1)):
        return text
    at = match.start(1) + len(match.group(1).rstrip("\n")) + 1
    return text[:at] + line + "\n" + text[at:]


def main():
    config_path, log, mode = sys.argv[1:4]
    base = os.path.dirname(os.path.abspath(config_path))
    config = json.load(open(config_path, encoding="utf-8"))
    plan = json.load(open(os.path.join(base, "🗺️plan.json"), encoding="utf-8"))["family"]
    artifact = os.path.join(REPO, config["artifact"])
    monolith = toml_loads(open(os.path.join(base, "🧾️monolith-Cargo.toml"), encoding="utf-8").read())
    deps, devs = monolith.get("dependencies", {}), monolith.get("dev-dependencies", {})
    externs = {}
    for key in list(deps) + list(devs):
        externs[key.replace("-", "_")] = key
    for alias, target in config.get("aliases", {}).items():
        externs.setdefault(alias, next((k for k in list(deps) + list(devs) if k.replace("-", "_") == target), None))
    for name in plan:
        externs[name.replace("-", "_")] = name
    owner, testy = {}, set()
    for name, info in plan.items():
        for f in info["lib"]:
            owner[os.path.normpath(os.path.join(artifact, f))] = name
        for f in info["test"]:
            owner[os.path.normpath(os.path.join(artifact, f))] = name
            testy.add(os.path.normpath(os.path.join(artifact, f)))
        for t in info["targets"]:
            for f in t["files"]:
                owner[os.path.normpath(os.path.join(artifact, f))] = name
                testy.add(os.path.normpath(os.path.join(artifact, f)))
    for t in config.get("existing_tests", []):
        path = os.path.normpath(os.path.join(artifact, t["file"]))
        owner[path] = t["package"]
        testy.add(path)
    wanted = collections.defaultdict(set)
    for line in open(log, encoding="utf-8"):
        for pattern in ERRORS:
            m = pattern.match(line.strip())
            if not m:
                continue
            path = m.group("file")
            path = os.path.normpath(path if os.path.isabs(path) else (os.path.join(artifact, path) if os.path.exists(os.path.join(artifact, path)) else os.path.join(REPO, path)))
            crate, key = owner.get(path), externs.get(m.group("name"))
            if crate and key and key != crate:
                wanted[(crate, "dev-dependencies" if path in testy else "dependencies")].add(key)
            elif crate and not key:
                print(f"UNKNOWN {m.group('name')} in {os.path.relpath(path, artifact)}")
    monolith_package = config["crate"].replace("_", "-")
    for (crate, section), keys in sorted(wanted.items()):
        manifest = os.path.join(artifact, "📦️packages/🦀️rust/Cargo.toml") if crate == monolith_package else os.path.join(artifact, plan[crate]["owner"], "📦️packages/🦀️rust/Cargo.toml")
        text = open(manifest, encoding="utf-8").read()
        for key in sorted(keys):
            source = (devs if section == "dev-dependencies" else deps).get(key) or deps.get(key) or devs.get(key)
            feats = source.get("features", []) if isinstance(source, dict) else []
            line = f"{key} = {{ workspace = true" + (", features = [" + ", ".join(f'"{f}"' for f in feats) + "]" if feats else "") + " }"
            text = add(text, section, line, key)
        print(f"{crate} [{section}] += {sorted(keys)}")
        if mode == "apply":
            open(manifest, "w", encoding="utf-8").write(text)


main()
