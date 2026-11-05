"""🔎 Audit active code/templates for generation2d mounts and wrong receiving-owner emission."""
import json
import os
import re
from pathlib import Path

root = Path.cwd()
roots = ["✏️s", "🧰️framework", "🌎️hub", ".agents"]
extensions = {".rs", ".ts", ".tsx", ".js", ".mjs", ".py", ".json", ".toml", ".tmpl", ".template", ".hbs", ".j2"}
excluded = {".git", ".r", "node_modules", "🗑️generated", "target", "dist", "build", "pkg"}
wrong = []
mounts = []
generator_candidates = []
count = 0
for selected in roots:
    for directory, dirs, files in os.walk(root / selected):
        dirs[:] = [name for name in dirs if name not in excluded]
        for filename in files:
            path = Path(directory) / filename
            if path.suffix not in extensions:
                continue
            text = path.read_text(errors="strict")
            count += 1
            relative = path.relative_to(root).as_posix()
            lines = text.splitlines()
            bad = [{"line": index, "text": line} for index, line in enumerate(lines, 1) if re.search(r"native_owned\s*=\s*[^;\n]{0,8}semio_framework_pack_json", line)]
            if bad:
                wrong.append({"path": relative, "matches": bad})
            if path.suffix == ".rs" and "generation2d" not in relative:
                hits = [{"line": index, "text": line} for index, line in enumerate(lines, 1) if "generation2d" in line or "semio_s_artifact_procedural_2d" in line]
                if hits:
                    mounts.append({"path": relative, "matches": hits})
            if "native_owned" in text and "semio_framework_pack_json" in text and "🗿️artifacts/" not in relative:
                generator_candidates.append({"path": relative, "nativeOwnerLines": [{"line": index, "text": line} for index, line in enumerate(lines, 1) if "native_owned" in line]})
report = {"scannedFiles": count, "roots": roots, "excludedDirectories": sorted(excluded), "wrong": wrong, "outsideRustReferences": mounts, "generatorCandidates": generator_candidates}
output = Path(__file__).parent / "🗑️generated" / "generation2d-source-audit.json"
output.write_text(json.dumps(report, ensure_ascii=False, indent=2) + "\n")
print(json.dumps({"scannedFiles": count, "wrongFiles": len(wrong), "wrongAttributes": sum(len(row["matches"]) for row in wrong), "outsideRustReferences": len(mounts), "generatorCandidates": [row["path"] for row in generator_candidates]}, ensure_ascii=False))
