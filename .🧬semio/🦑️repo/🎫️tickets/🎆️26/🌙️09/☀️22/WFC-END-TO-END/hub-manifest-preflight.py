"""🧭️ Validate hub-local Cargo dependency facts without executing or changing Cargo."""
import json
import tomllib
from pathlib import Path

ticket = Path(__file__).resolve().parent
root = next(parent for parent in ticket.parents if (parent / ".git").exists())
cache = {}


def read(path):
    if path not in cache:
        cache[path] = tomllib.loads(path.read_text(encoding="utf-8"))
    return cache[path]


def workspace(path, data):
    if "workspace" in data:
        return path
    explicit = data.get("package", {}).get("workspace")
    if explicit:
        return (path.parent / explicit / "Cargo.toml").resolve()
    return next((parent / "Cargo.toml" for parent in path.parent.parents
                 if (parent / "Cargo.toml").is_file()
                 and "workspace" in read(parent / "Cargo.toml")), None)


hub = root / "🌎️hub/Cargo.toml"
queue = []
for pattern in read(hub)["workspace"].get("members", []):
    for member in hub.parent.glob(pattern):
        path = member if member.name == "Cargo.toml" else member / "Cargo.toml"
        if path.exists():
            queue.append(path.resolve())
seen, issues = set(), []
while queue:
    path = queue.pop()
    if path in seen:
        continue
    seen.add(path)
    try:
        data = read(path)
        owner = workspace(path, data)
        inherited = read(owner).get("workspace", {}).get("dependencies", {}) if owner else {}
    except Exception as error:
        issues.append({"manifest": str(path.relative_to(root)), "parse": str(error)})
        continue
    tables = [data.get(kind, {}) for kind in ("dependencies", "dev-dependencies", "build-dependencies")]
    for target in data.get("target", {}).values():
        tables.extend(target.get(kind, {}) for kind in ("dependencies", "dev-dependencies", "build-dependencies"))
    for table in tables:
        for key, dependency in table.items():
            if not isinstance(dependency, dict):
                continue
            directory = path.parent
            if dependency.get("workspace"):
                if key not in inherited:
                    issues.append({"manifest": str(path.relative_to(root)), "missingKey": key})
                    continue
                dependency, directory = inherited[key], owner.parent
            if isinstance(dependency, dict) and "path" in dependency:
                child = (directory / dependency["path"] / "Cargo.toml").resolve()
                if child.exists():
                    queue.append(child)
                else:
                    issues.append({"manifest": str(path.relative_to(root)), "dependency": key, "missingPath": str(child)})
output = ticket / "🗑️generated/hub-manifest-preflight-current.json"
output.parent.mkdir(exist_ok=True)
output.write_text(json.dumps({"manifests": len(seen), "issues": issues}, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
(ticket / "run-r5-hub-preflight-current.md").write_text(
    "# Current Hub Manifest Preflight\n\n"
    f"Visited {len(seen)} local manifests from declared hub members. Found {len(issues)} unresolved dependency facts.\n\n"
    "The walk checks normal, dev, build and target dependency keys against each explicit workspace, and local dependency paths. "
    "It is static schema evidence; Cargo and runtime results remain required.\n\n"
    + "\n".join(f"- `{row['manifest']}`: {row}" for row in issues) + "\n", encoding="utf-8")
print(json.dumps({"manifests": len(seen), "issues": len(issues)}))
raise SystemExit(bool(issues))
