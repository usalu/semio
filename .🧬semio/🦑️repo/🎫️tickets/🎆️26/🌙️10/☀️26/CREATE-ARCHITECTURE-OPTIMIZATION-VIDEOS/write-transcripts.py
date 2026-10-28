"""🗣️ Publish accessible transcripts from the six authored bilingual scene contracts."""

import ast
import json
from pathlib import Path

ROOT = next(parent for parent in Path(__file__).resolve().parents if (parent / "🧰️framework").is_dir())
CLIP = ROOT / "🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip"
CASES = json.loads((CLIP / "🧪️cases/🔣️.json").read_text(encoding="utf-8"))
for language, index in (("en", 1), ("de", 2)):
    lines = ["# Optimization Videos" if language == "en" else "# Optimierungsvideos", ""]
    for chapter in CASES["chapters"]:
        tree = ast.parse((CLIP / chapter["folder"] / "🐍️.py").read_text(encoding="utf-8"))
        scene = next(node for node in tree.body if isinstance(node, ast.ClassDef) and node.name == chapter["scene"])
        fields = {target.id: ast.literal_eval(node.value) for node in scene.body if isinstance(node, ast.Assign) for target in node.targets if isinstance(target, ast.Name) and target.id in {"TITLE", "NARRATION"}}
        lines.extend(["## " + fields["TITLE"][language], "", "`" + chapter["video_id"] + "`", ""])
        lines.extend(text for row in fields["NARRATION"] for text in (row[index], ""))
    (CLIP / ("🗣️transcript." + language + ".md")).write_text("\n".join(lines), encoding="utf-8")
    print(language, "transcript published")
