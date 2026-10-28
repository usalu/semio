"""🖼️ Reproduce the actual EB07 encoder clock with native body and browser PNG metadata."""
from pathlib import Path
import argparse
from contextlib import nullcontext
import hashlib
import importlib
import json
import shutil
import sys
from unittest.mock import patch

ROOT = next(path for path in Path(__file__).resolve().parents if (path / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
TICKET = Path(__file__).resolve().parent
BASE = TICKET / "🗑️generated/i/de/spectator"
parser = argparse.ArgumentParser()
parser.add_argument("--uniform-sar", action="store_true")
parser.add_argument("--debug", action="store_true")
parser.add_argument("--case", default="actual")
parser.add_argument("--actual-ui", action="store_true")
parser.add_argument("--reuse-phases")
args = parser.parse_args()
work = TICKET / "🗑️generated/ie" / args.case
work.mkdir(parents=True, exist_ok=True)
raw = BASE / "5/v/EnergyBalance_07_TheorieVsPraxis.mp4"
raw_hash = hashlib.sha256(raw.read_bytes()).hexdigest()
runtime = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
presentation = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation.🐍️")
settings = dict(language="de", quality="-ql", resolution="480,270", fps=4.0, audience="spectator")
fixture = work.parent / "browser.png"
if not fixture.exists():
    candidate = next(iter([*BASE.glob("rendered/chapters/p-*/i/0000.png"), *TICKET.glob("🗑️generated/icb/chapter-dom-*/en0.png")]))
    shutil.copyfile(candidate, fixture)
original = runtime.run_command
prefix = runtime.ffmpeg_command()
def traced(command, **kwargs):
    """🔬 Keep the exact filter graph while exposing native decoder/filter reinitialization."""
    if args.debug and Path(kwargs.get("log", "")).name == "compose.log":
        command = [*command[:len(prefix)], "-loglevel", "debug", *command[len(prefix):]]
    return original(command, **kwargs)
def transition(chapter, snapshot, output, settings, session, *, phase):
    """🧾 Reuse actual browser PNG bytes and the actual native snapshot at its boundary."""
    from PIL import Image
    count = 9 if phase == "focus" else 18
    if args.reuse_phases:
        directory = work.parent / args.reuse_phases / ("i" if phase == "focus" else "o")
        assert len(list(directory.glob("[0-9][0-9][0-9][0-9].png"))) == count
        return directory, count
    directory = output / ("i" if phase == "focus" else "o")
    directory.mkdir(parents=True, exist_ok=True)
    for index in range(count):
        target = directory / f"{index:04d}.png"
        chosen = snapshot if index == (count - 1 if phase == "focus" else 0) else fixture
        if args.uniform_sar:
            with Image.open(chosen) as image:
                image.convert("RGB").save(target, "PNG")
        else:
            shutil.copyfile(chosen, target)
    return directory, count
session = runtime.ToolSession()
with patch.object(runtime, "run_command", traced), (nullcontext() if args.actual_ui else patch.object(presentation, "_transition", transition)):
    presentation._render_chapter("EnergyBalance_07_TheorieVsPraxis", raw, work / "encoded.mp4", settings, session, work)
    original([*prefix, "-hide_banner", "-i", str(work / "encoded.mp4"), "-map", "0:v:0", "-an", "-f", "null", "-"], session=session, log=work / "decoded.log")
import re
decoded = int(re.findall(r"frame=\s*(\d+)", (work / "decoded.log").read_text(encoding="utf-8"))[-1])
log = (work / "compose.log").read_text(encoding="utf-8")
events = [line for line in log.splitlines() if any(word in line.lower() for word in ("reinit", "changing", "changed from", "sar:"))]
result = dict(expected_frames=1336, decoded_frames=decoded, raw_sha256=raw_hash,
              raw_unchanged=hashlib.sha256(raw.read_bytes()).hexdigest() == raw_hash,
              native_events=events, uniform_sar=args.uniform_sar, debug=args.debug, actual_ui=args.actual_ui, reused_phases=args.reuse_phases)
(work / "result.json").write_text(json.dumps(result, ensure_ascii=False, indent=2), encoding="utf-8")
print("[DEBUG] " + json.dumps({**result, "native_events": events[:8], "native_event_count": len(events)}, ensure_ascii=False), flush=True)
raise SystemExit(decoded != 1336)
