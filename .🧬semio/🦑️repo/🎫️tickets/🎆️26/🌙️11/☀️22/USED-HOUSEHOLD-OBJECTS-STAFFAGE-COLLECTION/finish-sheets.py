"""🖼 Renders the review sheets of candidates of this ticket one tool process at a time and joins them into contact sheets under `🗑️generated/contact/`: `<slug>.poses.png` (the 1:50 body sheet of every pose that is not a key and the middle instant of every clip), `<slug>.scales.png` (body at the coarser authored scales, thermal and structure at 1:50) and `<slug>.2d.png` (the drawings of every authored scale).

Usage: `python3 finish-sheets.py <slug> …`
"""
import json
import subprocess
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
TOOLS = (HERE / "../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE").resolve()
BUN = sorted((Path.home() / "Library/Caches/semio/runtimes/bun").glob("*/bun"))[0]
OUT = HERE / "🗑️generated" / "contact"


def run(script, *args):
    """🔧 One tool run in the tool ticket; returns the PNG path it reports."""
    for attempt in range(3):
        done = subprocess.run([str(BUN), script, *args], cwd=TOOLS, capture_output=True, text=True)
        if done.returncode == 0:
            break
    text = done.stdout + done.stderr
    paths = [word for line in text.splitlines() for word in line.split() if word.endswith(".png")]
    if done.returncode != 0 or not paths:
        print(f"[DEBUG] FAILED {script} {' '.join(args)}: {text.strip()[-300:]}")
        return None
    path = Path(paths[-1])
    return str(path if path.is_absolute() else TOOLS / path)


def contact(name, columns, width, files):
    """🗃 One contact sheet of the files that rendered."""
    files = [file for file in files if file]
    if files:
        subprocess.run([str(BUN), str(HERE / "finish-contact.ts"), str(OUT / name), str(columns), str(width), *files], cwd=TOOLS, check=True)


OUT.mkdir(parents=True, exist_ok=True)
for slug in sys.argv[1:]:
    file = str(HERE / "assets" / slug / "🔣️.json")
    asset = json.loads(Path(file).read_text())["asset"]
    poses = [pose["id"] for pose in asset["poses"] if not pose.get("key")]
    sheets = [run("r3-sheet.ts", "--file", file, "--scale", "50", "--list", "body", "--pose", pose) for pose in poses]
    sheets += [run("r3-sheet.ts", "--file", file, "--scale", "50", "--list", "body", "--clip", clip["id"], "--t", str(round(clip["durationS"] / 2, 2))) for clip in asset["clips"]]
    contact(f"{slug}.poses.png", 2, 1100, sheets)
    coarse = [scale for scale in ("100", "200", "500") if asset["volumes"][scale].get("status") == "authored"]
    sheets = [run("r3-sheet.ts", "--file", file, "--scale", scale, "--list", "body") for scale in coarse]
    sheets += [run("r3-sheet.ts", "--file", file, "--scale", "50", "--list", name) for name in ("thermal", "structure")]
    contact(f"{slug}.scales.png", 2, 1100, sheets)
    drawn = [scale for scale in ("50", "100", "200", "500") if asset["representations"][scale]["status"] == "authored"]
    contact(f"{slug}.2d.png", 2, 1100, [run("r3-2d.ts", "--file", file, "--scale", scale) for scale in drawn])
    print(f"{slug}: {len(poses)} poses, {len(asset['clips'])} clips, coarse {coarse}, drawn {drawn}")
