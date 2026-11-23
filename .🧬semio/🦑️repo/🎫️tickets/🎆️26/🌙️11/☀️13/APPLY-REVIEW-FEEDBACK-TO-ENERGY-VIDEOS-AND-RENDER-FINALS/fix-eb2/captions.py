import importlib
import os
import sys

os.environ.setdefault("SEMIO_TEACHING_LANGUAGE", "de")
sys.path.insert(0, "C:/git/semio")
from manim import VGroup

B = "🎓️teaching.🏛️architecture.⚡️energy."
shared = importlib.import_module(B + "🏗️authoring.🎨️visuals.🐍️")
D = B + "📊️demand.🎬️clip."
worst = 0
for clip in ("⚖️regulations", "🏭️primary-energy", "🧪️practice", "🏷️certificate"):
    module = importlib.import_module(D + clip + ".🐍️")
    for name, scene in vars(module).items():
        if not (name.startswith("Beat") and hasattr(scene, "NARRATION")):
            continue
        for key, en, de in scene.NARRATION:
            text = shared.subtitle_text(scene.NARRATION, key)
            try:
                label = shared.caption_bar(text)[1]
                lines = len(label) if isinstance(label, VGroup) else 1
            except ValueError:
                lines = "ERR"
            if lines not in (1, 2):
                print(clip.encode("ascii", "backslashreplace").decode(), key, lines)
            worst = max(worst, lines if isinstance(lines, int) else 9)
print("max lines", worst)
