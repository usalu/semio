"""🧍 Compare public-domain photo-derived human contours for architectural entourage."""

import json
from pathlib import Path
from manim import Camera, SVGMobject
from PIL import Image, ImageDraw

OUT = Path(__file__).resolve().parent / "🗑️generated/silhouettes/references"
sheet = Image.new("RGB", (3000, 1000), "#171717")
draw = ImageDraw.Draw(sheet)
for index, name in enumerate(("standing", "seated", "walk", "runner", "sprint", "office")):
    figure = SVGMobject(str(OUT / (name + ".svg")), height=2.2, width=None).set_color("#FFFFFF")
    width, height = 500, 920
    span = max(figure.height, figure.width * height / width) * 1.15
    camera = Camera(pixel_width=width, pixel_height=height, frame_width=span * width / height,
                    frame_height=span, frame_center=figure.get_center(), background_color="#171717")
    camera.capture_mobjects([figure])
    sheet.paste(camera.get_image().convert("RGB"), (index * 500, 60))
    draw.text((index * 500 + 20, 10), name, fill="white")
    print("[DEBUG] " + name + " parts " + str([(len(p.points),len(p.get_subpaths())) for p in figure.family_members_with_points()]), flush=True)
sheet.save(OUT / "review.png")
