"""🖼️ Contact sheet of an energy source scene for review."""
import sys
from pathlib import Path
import av
from PIL import Image, ImageDraw

source, output = map(Path, sys.argv[1:3])
fractions = [float(value) for value in sys.argv[3:]] or [0.4, 0.5, 0.6, 0.7, 0.8, 0.9]
with av.open(str(source)) as container:
    stream = container.streams.video[0]
    duration = float(stream.duration * stream.time_base)
    sheet = Image.new("RGB", (1280, 1080), "black")
    draw = ImageDraw.Draw(sheet)
    for index, fraction in enumerate(fractions):
        container.seek(int(duration * fraction / stream.time_base), stream=stream, backward=True)
        frame = next(frame for frame in container.decode(stream) if frame.time >= duration * fraction)
        x, y = index % 2 * 640, index // 2 * 360
        sheet.paste(frame.to_image().resize((640, 360)), (x, y))
        draw.text((x + 8, y + 8), str(fraction), fill="yellow")
    sheet.save(output)
