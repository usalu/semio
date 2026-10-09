"""🖼️ [DEBUG] Contact sheets for overlap review — one frame every STEP seconds, tiled with timestamps.

Usage: contact_sheet.py <video.mp4> <out_prefix> [step_seconds] [t0] [t1]
Writes <out_prefix>_01.png, _02.png … (12 frames per sheet, 3×4 grid).
"""
import subprocess
import sys
import tempfile
from pathlib import Path

from PIL import Image, ImageDraw

video, prefix = Path(sys.argv[1]), sys.argv[2]
step = float(sys.argv[3]) if len(sys.argv) > 3 else 1.5
dur = float(subprocess.check_output([
    "ffprobe", "-v", "error", "-show_entries", "format=duration", "-of", "csv=p=0", str(video),
]).decode().strip())
t0 = float(sys.argv[4]) if len(sys.argv) > 4 else 0.0
t1 = float(sys.argv[5]) if len(sys.argv) > 5 else dur - 0.05

times = []
t = t0
while t <= t1:
    times.append(round(t, 2))
    t += step

W, H, COLS, ROWS = 640, 360, 3, 4
with tempfile.TemporaryDirectory() as tmp:
    frames = []
    for i, ts in enumerate(times):
        out = Path(tmp) / f"f{i:04d}.png"
        subprocess.run(["ffmpeg", "-y", "-v", "error", "-ss", str(ts), "-i", str(video),
                        "-frames:v", "1", "-vf", f"scale={W}:{H}", str(out)], check=True)
        if out.exists():
            frames.append((ts, Image.open(out).copy()))
    per = COLS * ROWS
    for s in range(0, len(frames), per):
        sheet = Image.new("RGB", (COLS * W, ROWS * H), "black")
        draw = ImageDraw.Draw(sheet)
        for k, (ts, img) in enumerate(frames[s:s + per]):
            x, y = (k % COLS) * W, (k // COLS) * H
            sheet.paste(img, (x, y))
            draw.rectangle([x, y, x + 70, y + 20], fill="black")
            draw.text((x + 4, y + 4), f"t={ts:.1f}s", fill="yellow")
            draw.rectangle([x, y, x + W - 1, y + H - 1], outline="#444444")
        path = f"{prefix}_{s // per + 1:02d}.png"
        sheet.save(path)
        print(path)
