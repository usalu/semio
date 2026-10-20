import sys, importlib.util
sys.path.insert(0, "tutorial")
spec = importlib.util.spec_from_file_location("scene_4", "tutorial/energy/demand/Ventilation/4_natuerliche_lueftung/scene_4.py")
m = importlib.util.module_from_spec(spec); spec.loader.exec_module(m)
from manim_visuals import caption_bar, subtitle_text
for name in dir(m):
    c = getattr(m, name)
    if isinstance(c, type) and hasattr(c, "NARRATION"):
        for key, en, de in c.NARRATION:
            try:
                caption_bar(subtitle_text(c.NARRATION, key))
            except Exception as e:
                print("[DEBUG] FAIL", name, key, str(e)[:60], "|", de.replace("\n", " / "))
print("[DEBUG] caption check done")
