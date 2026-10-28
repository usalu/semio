"""🎬 Demonstrate canonical UI hover, real fullscreen clicks and click-activated equipment."""

import hashlib
import importlib
import json
import os
import shutil
import sys
import time
from pathlib import Path

ROOT = next(path for path in Path(__file__).resolve().parents if (path / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
sys.dont_write_bytecode = True
from manim import FadeIn, FadeOut, Line, Rectangle, Scene, VGroup, tempconfig
import numpy as np

BASE = "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring"
p = importlib.import_module(BASE + ".🎬️clip.🪟️presentation.🐍️")
runtime = importlib.import_module(BASE + ".🎬️clip.⚙️runtime.🐍️")
helper = importlib.import_module(BASE + ".🎬️clip.🪟️presentation.🖱️interaction.🎬️scene.🐍️")
fonts = importlib.import_module(BASE + ".🔤️fonts.🐍️")
visuals = importlib.import_module(BASE + ".🎨️visuals.🐍️")
output = Path(os.environ["SEMIO_TEACHING_OUTPUT_ROOT"])
output.mkdir(parents=True, exist_ok=True)
os.environ["SEMIO_TEACHING_AUDIENCE"] = "spectator"
settings = {"quality": "-ql", "resolution": "960,540", "fps": 8, "language": "de", "audience": "spectator"}


class DeviceDemo(Scene):
    """🔌 A real native control starts physical heat animation only after the cursor click commits."""

    def construct(self):
        fonts.apply_scene_style(self)
        system = visuals.window_frame(10, 5.4, title_width=2.2, title_depth=0.45, color=visuals.color("gray"))
        device = visuals.window_frame(6.6, 3.4, title_width=2.3, title_depth=0.45, color=visuals.color("gray")).shift(np.array([0.3, 0, 0]))
        system_label = fonts.body_text("System", font_size=fonts.BODY_FONT_SIZE, color=visuals.P_WHITE).move_to([-4.4, 2.46, 0])
        device_label = fonts.body_text("Heizgerät", font_size=fonts.BODY_FONT_SIZE, color=visuals.P_WHITE).move_to([-2.05, 1.46, 0])
        switch = visuals.chip("Ein", font_size=fonts.BODY_FONT_SIZE).move_to([2.4, 0.85, 0])
        radiator = VGroup(Rectangle(width=1.7, height=1.5, color=visuals.color("gray"), fill_opacity=0.1),
                          *[Line([x, -0.55, 0], [x, 0.55, 0], color=visuals.color("gray")) for x in (-0.55, -0.25, 0.05, 0.35, 0.65)]).move_to([-1.3, -0.15, 0])
        reading = fonts.body_text("0 W", font_size=fonts.FORMULA_FONT_SIZE, color=visuals.P_WHITE).move_to([1.5, -0.55, 0])
        note = fonts.body_text("Mit dem Cursor einschalten", font_size=fonts.BODY_FONT_SIZE, color=visuals.P_WHITE).move_to([0, -2.05, 0])
        waves = visuals.radiation_waves([-1.3, 0.45, 0], height=0.35, color=visuals.P_PRIMARY)
        self.add(system, device, system_label, device_label, switch, radiator, reading, note)
        self.wait(0.5)

        def activate():
            switch[1].become(fonts.body_text("An", font_size=fonts.BODY_FONT_SIZE, color=visuals.P_DEEP_DARK).move_to(switch[1]))
            reading.become(fonts.body_text("1500 W", font_size=fonts.FORMULA_FONT_SIZE, color=visuals.P_PRIMARY).move_to(reading))
            self.play(radiator.animate.set_color(visuals.P_PRIMARY), FadeIn(waves), run_time=0.8)
            self.play(waves.animate.shift(np.array([0, 0.15, 0])), run_time=0.8)

        helper.cursor_click(self, switch, activate, parents=(system, device), role="toggle",
                            move_time=0.6, hover_time=0.35, press_time=0.25, release_time=0.25)
        self.wait(0.6)


body = output / "body" / "v" / "device.mp4"
if "--reuse-body" not in sys.argv or not body.is_file():
    runtime.render_config(output / "body")
    with tempconfig({"media_dir": str(output / "body"), "video_dir": str(output / "body" / "v"),
                     "partial_movie_dir": str(output / "body" / "p"), "text_dir": str(output / "body" / "t"),
                     "output_file": "device", "pixel_width": 960, "pixel_height": 540, "frame_rate": 8,
                     "write_to_movie": True, "save_last_frame": False, "disable_caching": True,
                     "verbosity": "ERROR", "progress_bar": "none"}):
        scene = DeviceDemo()
        scene.render()
        body = Path(scene.renderer.file_writer.movie_file_path)
    helper._dump()
identity = hashlib.sha256(Path(__file__).parent.relative_to(ROOT).as_posix().encode()).hexdigest()[:10]
trace = json.loads((output / "trace" / identity / "DeviceDemo" / "👆️.json").read_text(encoding="utf-8"))
runtime.publish_cursor_intervals(body, trace["intervals"])
lesson = output / "lesson.mp4"
p.render_presentation("physical_fundamentals", body, lesson, settings)

frames = output / "tour"
frames.mkdir(parents=True, exist_ok=True)
index = 0
menu_active = False


def tour_point(start, end, duration, menu=False, pressed=False):
    """🧭 Move through the same real hit regions used by the lesson compositor."""
    global index, menu_active
    count = max(1, round(duration * settings["fps"]))
    for step in range(count):
        u = step / max(1, count - 1)
        u = u * u * (3 - 2 * u)
        point = [start[i] * (1 - u) + end[i] * u for i in (0, 1)]
        on_trigger = 132 <= point[0] < 282 and 8 <= point[1] < 52
        in_menu = 132 <= point[0] < 476 and 52 <= point[1] < 286
        menu_active = menu or on_trigger or menu_active and in_menu
        p.render_interaction_frame("physical_fundamentals", "focus", 0.2, frames / f"{index:04d}.png", settings,
                                   point=point, pressed=pressed, menu_open=menu_active)
        index += 1


park, nav, row, footer = [1572, 840], [210, 30], [250, 122], [1522, 883]
tour_point(park, park, 0.4)
tour_point(park, nav, 0.55)
tour_point(nav, nav, 0.3)
tour_point(nav, row, 0.45, True)
tour_point(row, row, 0.4, True)
tour_point(row, footer, 0.6)
tour_point(footer, footer, 0.35)
tour_point(footer, park, 0.4)
tour_point(park, park, 0.3)
tour = output / "tour.mp4"
runtime.run_command([*runtime.ffmpeg_command(), "-hide_banner", "-loglevel", "error", "-y",
                     "-framerate", str(settings["fps"]), "-i", str(frames / "%04d.png"), "-an",
                     "-c:v", "libx264", "-crf", "0", "-pix_fmt", "yuv420p", str(tour)], log=output / f"tour-{time.time_ns()}.log")
joined = output / "cursor-ui.mp4"
runtime.concat_videos([tour, lesson], joined, output / "join.txt", keep_audio=False)
final = Path(__file__).with_name("cursor-ui.mp4")
shutil.copy2(joined, final)
p.render_overview(output / "cursor-ui.png", settings)
shutil.copy2(output / "cursor-ui.png", Path(__file__).with_name("cursor-ui.png"))
(Path(__file__).parent / "cursor-demo-events.md").write_text("# Authored Cursor Demo Events\n\n" +
    "The native helper emits the following event-driven activation trace; the physical heater starts only after the committed click.\n\n" +
    "\n".join(f"- {event['time']:.3f} s: {event['type']}" for event in trace["events"]) +
    "\n\nEnclosing window activity: " + str(trace["intervals"]) + "\n", encoding="utf-8")
print(f"[DEBUG] Cursor UI review film: {final}", flush=True)
