from importlib.metadata import version
from pathlib import Path
import sys

import imageio_ffmpeg
import manim
import numpy
import dotenv

ticket = Path(__file__).parent
environment = ticket / "🗑️generated" / "v"
paths = [str(path) for path in environment.rglob("*")]
longest = max(paths, key=len)
ffmpeg = imageio_ffmpeg.get_ffmpeg_exe()
packages = ["manim", "numpy", "python-dotenv", "imageio-ffmpeg", "av", "scipy"]
report = "# Rendering Dependencies\n\n"
report += f"Isolated interpreter: `{sys.executable}`. Cached base interpreter: Python {sys.version.split()[0]}.\n\n"
report += "Packages installed through `repo tools exec --light uv`; no shared Python environment or global installation was changed. Package cache was disabled, and compiler/download temporary files stayed under the ticket generated directory.\n\n"
report += "| Package | Version |\n| --- | --- |\n"
report += "".join(f"| {package} | {version(package)} |\n" for package in packages)
report += f"\nImport smoke check passed for Manim, NumPy, dotenv, and imageio-ffmpeg. Bundled ffmpeg executable: `{ffmpeg}`.\n\n"
report += f"Longest environment path is {len(longest)} characters: `{longest}`.\n"
(ticket / "dependency.md").write_text(report, encoding="utf-8")
print(f"[DEBUG] Python: {sys.executable}; ffmpeg: {ffmpeg}; longest path: {len(longest)}")
if len(longest) > 256:
    raise RuntimeError(f"Environment path exceeds 256 characters: {longest}")
