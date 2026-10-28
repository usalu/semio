"""🔬 Validate authored accent module imports and global name resolution without rendering."""

import ast
import builtins
import dis
import importlib
import json
import os
import re
import types
import sys
from pathlib import Path

ROOT = next(path for path in Path(__file__).resolve().parents if (path / "🧰️framework").is_dir())
TICKET = Path(__file__).resolve().parent
OUTPUT = TICKET / "🗑️generated" / "accent-imports"
OUTPUT.mkdir(parents=True, exist_ok=True)
sys.path.insert(0, str(ROOT))
sys.dont_write_bytecode = True
os.environ["SEMIO_TEACHING_OUTPUT_ROOT"] = str(OUTPUT)
os.environ["SEMIO_TEACHING_CACHE_DIR"] = str(OUTPUT / "cache")
paths = re.findall(r"^- `([^`]+\.py)`", (TICKET / "accent-scenes.md").read_text(encoding="utf-8"), re.M)
paths += [
    "🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️theme/🐍️.py",
    "🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🐍️.py",
    "🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🔤️fonts/🐍️.py",
]
results = []
from manim import tempconfig

with tempconfig({"media_dir": str(OUTPUT / "media"), "text_dir": str(OUTPUT / "text")}):
    for relative in paths:
        source = (ROOT / relative).read_text(encoding="utf-8")
        ast.parse(source, filename=relative)
        module = importlib.import_module(".".join(Path(relative).with_suffix("").parts))
        allowed = set(vars(module)) | set(vars(builtins))
        pending = [compile(source, relative, "exec")]
        unresolved = set()
        while pending:
            code = pending.pop()
            pending.extend(value for value in code.co_consts if isinstance(value, types.CodeType))
            instructions = tuple(dis.get_instructions(code))
            local_names = {item.argval for item in instructions if item.opname == "STORE_NAME"}
            if any(item.opname == "SETUP_ANNOTATIONS" for item in instructions):
                local_names.add("__annotations__")
            unresolved.update(item.argval for item in instructions
                              if item.opname in ("LOAD_GLOBAL", "LOAD_NAME")
                              and item.argval not in allowed
                              and (item.opname != "LOAD_NAME" or item.argval not in local_names))
        primary = getattr(module, "P_PRIMARY", None)
        if primary is not None and primary.lower() != "#ff344f":
            unresolved.add("P_PRIMARY canonical role")
        results.append({"path": relative, "unresolved_globals": sorted(unresolved), "primary": primary})
(OUTPUT / "imports.json").write_text(json.dumps(results, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
failures = [item for item in results if item["unresolved_globals"]]
sys.stdout.reconfigure(encoding="utf-8")
print("[DEBUG] Parsed and imported", len(results), "modules; unresolved-name modules:", len(failures))
for item in failures:
    print("[DEBUG]", item["path"], item["unresolved_globals"])
raise SystemExit(bool(failures))
