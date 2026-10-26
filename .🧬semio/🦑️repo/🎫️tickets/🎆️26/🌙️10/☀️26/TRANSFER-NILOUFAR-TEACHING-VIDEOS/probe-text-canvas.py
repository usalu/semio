"""🔎 Isolate Pango canvas effects without changing frozen teaching sources."""
from importlib import import_module
import hashlib
import json
from pathlib import Path
import sys
import xml.etree.ElementTree as ET

TICKET = Path(__file__).resolve().parent
WORKSPACE = next(path for path in TICKET.parents if (path / "🧰️framework").is_dir())
sys.path.insert(0, str(WORKSPACE))
from manim import Text, config, tempconfig

fonts = import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")
fonts.apply_body_font()
output = TICKET / "🗑️generated/text-canvas-probe"
output.mkdir(parents=True, exist_ok=True)
cases = [
    ("erst beide Ströme geführt → Filter, Feuchte, Wärmerückgewinnung möglich", fonts.BODY_FONT_SIZE),
    ("volle Lüftungsverlust von 1500 W bleibt, η", fonts.CAPTION_FONT_SIZE),
    ("Nur der Anteil 1 − η bleibt als Verlust: bei η", fonts.CAPTION_FONT_SIZE),
]
profiles = [(320, 180), (854, 480), (1920, 1080), (320, 480), (854, 180)]
original = Text._gen_chars
observations = []
current = {}


def observe(self):
    """📍 Observe the framework boundary before its strict character check."""
    current.update(svg=str(self.file_name), glyph_submobjects=len(self.submobjects), rendered_font_size=self._font_size, overrides=self.t2f)
    return original(self)


Text._gen_chars = observe
try:
    for width, height in profiles:
        for number, (text, size) in enumerate(cases):
            media = output / f"{width}x{height}" / str(number)
            with tempconfig({"pixel_width": width, "pixel_height": height, "media_dir": str(media), "text_dir": str(media / "t")}):
                current = {"text": text, "font": fonts.BODY_FONT, "font_size": size, "pixel_width": width, "pixel_height": height, "nonspace_characters": sum(not char.isspace() for char in text)}
                try:
                    obj = fonts.body_text(text, font_size=size, color="#FFFFFF")
                    points = b"".join(member.points.tobytes() for member in obj.family_members_with_points())
                    current.update(ok=True, geometry_sha256=hashlib.sha256(points).hexdigest(), width=obj.width, height=obj.height)
                except Exception as error:
                    current.update(ok=False, error=repr(error))
                if current.get("svg"):
                    svg = ET.parse(current["svg"]).getroot()
                    uses = list(svg.iter("{http://www.w3.org/2000/svg}use"))
                    current.update(svg_attributes=svg.attrib, svg_uses=len(uses), use_y=sorted({entry.get("y") for entry in uses if entry.get("y") is not None}))
                observations.append(current)
                print("[DEBUG]", width, height, number, current["ok"], current.get("glyph_submobjects"), current["nonspace_characters"], flush=True)
finally:
    Text._gen_chars = original
(output / "result.json").write_text(json.dumps(observations, ensure_ascii=False, indent=2), encoding="utf-8")
