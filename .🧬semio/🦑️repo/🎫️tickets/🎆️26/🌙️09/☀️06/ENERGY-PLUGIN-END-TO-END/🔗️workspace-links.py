import collections
import json
import pathlib

root = next(parent for parent in pathlib.Path(__file__).resolve().parents if (parent / "✏️s").is_dir())
packages = collections.defaultdict(list)
for source in [root / "🧰️framework", root / "✏️s"]:
    for manifest in source.rglob("package.json"):
        if any(part in {"node_modules", "🤖️generated", "🗑️generated"} for part in manifest.parts):
            continue
        try:
            name = json.loads(manifest.read_text()).get("name", "")
        except (OSError, ValueError):
            continue
        if name.startswith("@semio-tech/"):
            packages[name].append(manifest.parent)
count = 0
for name, paths in packages.items():
    link = root / "node_modules" / name
    if len(paths) == 1 and not link.exists() and not link.is_symlink():
        link.parent.mkdir(parents=True, exist_ok=True)
        link.symlink_to(paths[0], target_is_directory=True)
        count += 1
print(f"Linked {count} uniquely declared local verification packages")
