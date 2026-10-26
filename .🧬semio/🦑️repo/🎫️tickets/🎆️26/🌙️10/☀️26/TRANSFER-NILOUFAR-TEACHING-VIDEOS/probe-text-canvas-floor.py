"""📐 Verify a private temporary Pango canvas without modifying product code."""
from importlib import import_module
import hashlib
import json
from pathlib import Path
import sys

TICKET = Path(__file__).resolve().parent
WORKSPACE = next(path for path in TICKET.parents if (path / "🧰️framework").is_dir())
sys.path.insert(0, str(WORKSPACE))
from manim import Scene, config, tempconfig

fonts = import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")
fonts.apply_body_font()
output = TICKET / "🗑️generated/text-canvas-floor"
baseline = json.loads((TICKET / "🗑️generated/text-canvas-probe/result.json").read_text(encoding="utf-8"))
expected = [row for row in baseline if (row["pixel_width"], row["pixel_height"]) == (854, 480)]
rows = []
with tempconfig({"pixel_width": 320, "pixel_height": 180}):
    scene = Scene()
    camera_before = (scene.camera.pixel_width, scene.camera.pixel_height)
    for number, case in enumerate(expected):
        media = output / str(number)
        with tempconfig({"pixel_width": 854, "pixel_height": 480, "media_dir": str(media), "text_dir": str(media / "t")}):
            obj = fonts.body_text(case["text"], font_size=case["font_size"], color="#FFFFFF")
        points = b"".join(member.points.tobytes() for member in obj.family_members_with_points())
        digest = hashlib.sha256(points).hexdigest()
        assert digest == case["geometry_sha256"], (case["text"], digest)
        assert (config.pixel_width, config.pixel_height) == (320, 180)
        assert (scene.camera.pixel_width, scene.camera.pixel_height) == camera_before == (320, 180)
        rows.append({"text": case["text"], "font_size": case["font_size"], "geometry_sha256": digest, "matches_original_480p": True, "output_dimensions": [config.pixel_width, config.pixel_height], "camera_dimensions": list(camera_before), "svg": str(obj.file_name)})
output.mkdir(parents=True, exist_ok=True)
(output / "result.json").write_text(json.dumps(rows, ensure_ascii=False, indent=2), encoding="utf-8")
print("[DEBUG] All three scoped-canvas objects match original480p vectors exactly; camera/output remain320x180", flush=True)
