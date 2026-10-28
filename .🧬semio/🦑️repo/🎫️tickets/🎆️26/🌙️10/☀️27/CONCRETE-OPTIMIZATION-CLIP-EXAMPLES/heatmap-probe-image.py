"""[DEBUG] Probe ImageMobject rendering with the installed Pillow."""
import traceback
from pathlib import Path
import numpy as np
import manim

HERE = Path(__file__).parent


class S(manim.Scene):
    def construct(self):
        array = np.zeros((40, 60, 4), dtype="uint8")
        array[..., 0] = np.linspace(0, 255, 60, dtype="uint8")[None, :]
        array[..., 3] = 255
        image = manim.ImageMobject(array)
        image.stretch_to_fit_width(4).stretch_to_fit_height(2).move_to([1, 0, 0])
        self.add(manim.Square(side_length=.5, color=manim.WHITE).move_to([-3, 0, 0]), image)
        self.renderer.update_frame(self, mobjects=self.mobjects, ignore_skipping=True)
        self.renderer.camera.get_image().save(HERE / "🗑️generated/toolkit/probe-image.png")
        print("[DEBUG] rendered")


with manim.tempconfig({"pixel_width": 640, "pixel_height": 360, "frame_rate": 1, "write_to_movie": False, "save_last_frame": False, "disable_caching": True, "skip_animations": True, "media_dir": str(HERE / "🗑️generated/toolkit/media"), "verbosity": "ERROR"}):
    try:
        S().render()
    except Exception:
        traceback.print_exc()
