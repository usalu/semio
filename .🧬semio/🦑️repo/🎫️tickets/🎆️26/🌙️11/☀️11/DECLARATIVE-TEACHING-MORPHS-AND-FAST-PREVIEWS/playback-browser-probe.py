"""Probe: render the root overview frame once through the real presentation browser and print its log on failure."""
import importlib
import os
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[7]
sys.path.insert(0, str(ROOT))
PACKAGE = "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation"
browser = importlib.import_module(PACKAGE + ".🌐️browser.🐍️")
hierarchy = importlib.import_module(PACKAGE + ".🥞️hierarchy.🐍️")
render = importlib.import_module(PACKAGE + ".🥞️hierarchy.🖌️render.🐍️")
work = Path(tempfile.mkdtemp(prefix="probe-", dir=Path(os.environ["SEMIO_TEACHING_OUTPUT_ROOT"]).resolve()))
frames = [{"output": str(work / f"{i}.png"), "media": [], "hierarchy": render.initial_hierarchy(), "cursor": {"point": None}, "transition_elapsed": 1000} for i in range(2)]
try:
    result = browser.render_batch({"schema": "semio-presentation-browser/v1", "viewport": {"width": 480, "height": 270}, "catalog": hierarchy.hierarchy_contract(), "language": "en", "work_directory": str(work), "frames": frames})
    print("OK", result["outputs"])
except Exception as error:
    print("FAILED", error)
    for log in work.rglob("run.log"):
        print(log.read_text(encoding="utf-8")[-3000:])
