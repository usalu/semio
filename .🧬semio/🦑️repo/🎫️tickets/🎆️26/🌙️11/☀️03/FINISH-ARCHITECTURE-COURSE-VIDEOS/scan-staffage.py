"""🔎 Report course clips that still draw physical staffage outside the catalog rules."""

import json
from pathlib import Path

TICKET = Path(__file__).resolve().parent
ROOT = next(path for path in TICKET.parents if (path / "🧰️framework").is_dir())
SOURCE = ROOT / "🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🧪️tests/🐍️staffage_only.py"
text = SOURCE.read_text(encoding="utf-8").replace(
    'V = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")',
    "V = None",
)
namespace = {"__file__": str(SOURCE), "__name__": "scan"}
exec(compile(text, str(SOURCE), "exec"), namespace)
CASES = namespace["CASES"]
ARCHITECTURE = namespace["ROOT"] / CASES["rule_a"]["scope"]
SKIP = set(CASES["skip_folders"])


def clip_sources():
    """📜 Clip scripts under architecture, without tests, caches or dependencies."""
    found = []
    for folder, names, files in ARCHITECTURE.walk():
        names[:] = [name for name in names if name not in SKIP and name not in {"node_modules", "__pycache__", ".venv"}]
        if CASES["clip_folder"] in folder.parts and CASES["file_name"] in files:
            found.append(folder / CASES["file_name"])
    return sorted(found)


namespace["clip_sources"] = clip_sources
allowed_a = set(CASES["rule_a"]["allowed"])
allowed_c = set(CASES["rule_c"]["allowed"])
report = []
for path in namespace["clip_sources"]():
    source = path.read_text(encoding="utf-8")
    key = namespace["clip_key"](path) if CASES["rule_a"]["scope"] in path.parts else path.relative_to(namespace["ROOT"]).as_posix()
    row = {
        "clip": key,
        "pictograms": namespace["physical_pictograms"](source),
        "primitives": [name for name, _ in namespace["primitive_objects"](source) if f"{key}::{name}" not in allowed_a],
        "composed": [name for name in namespace["composed_objects"](source) if f"{key}::{name}" not in allowed_c],
        "literals": namespace["literal_units"](source),
        "stretches": namespace["stretched_cast_functions"](source),
        "forbidden": sorted(namespace["ast_names"](source) & set(CASES["forbidden_names"])),
    }
    if any(row[name] for name in ("pictograms", "primitives", "composed", "literals", "stretches", "forbidden")):
        report.append(row)
print(json.dumps({"clips": len(namespace["clip_sources"]()), "violations": report}, ensure_ascii=False, indent=2))
