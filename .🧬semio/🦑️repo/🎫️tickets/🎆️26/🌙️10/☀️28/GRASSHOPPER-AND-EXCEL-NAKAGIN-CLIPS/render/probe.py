"""🔎 Print duration and video stream of mp4 files with the bundled ffmpeg."""
import subprocess
import sys

import imageio_ffmpeg

sys.stdout.reconfigure(encoding="utf-8", errors="backslashreplace")
for file in sys.argv[1:]:
    text = subprocess.run([imageio_ffmpeg.get_ffmpeg_exe(), "-i", file], capture_output=True, encoding="utf-8", errors="replace").stderr or ""
    print(file.replace("\\", "/").split("/")[-1], [line.strip()[:110] for line in text.splitlines() if "Duration" in line or "Video:" in line])
