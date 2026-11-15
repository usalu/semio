import importlib
import json
import shutil
import sys
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path
from unittest.mock import patch

from PIL import Image

sys.stdout.reconfigure(encoding="utf-8")
root = Path(__file__).resolve().parents[7]
sys.path.insert(0, str(root))
P = "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip."
posters = importlib.import_module(P + "🪟️presentation.🖼️posters.🐍️")
runtime = importlib.import_module(P + "⚙️runtime.🐍️")
out = Path(__file__).resolve().parent / "🗑️generated" / "cand"
settings = {"language": "de", "profile": "preview", "audience": "speaker", "resolution": None, "fps": None}
known = posters.chapters()
jobs = []
for argument in sys.argv[1:]:
    scene, index, grid = argument.split(":")
    video = next(v for v in runtime.load_manifest(source_ids=[]) if any(c["scene"] == scene for c in v["chapters"]))
    chapter = next(c for c in video["chapters"] if c["scene"] == scene)
    for animation in grid.split(","):
        jobs.append((scene, int(index), chapter["segments"][int(index)], int(animation)))


def work(job):
    scene, index, segment, animation = job
    info = known[scene]
    patched = {**known, scene: {**info, "declaration": {"segment": index, "animation": animation}, "segment": {"script": segment["script"], "scene": segment["scene"]}, "ordinal": 100 + jobs.index(job)}}
    with patch.object(posters, "sources", return_value={}), patch.object(posters, "chapters", return_value=patched), patch.object(posters, "asset", return_value=(out / f"{scene}.{index}.{animation}.png", out / f"{scene}.{index}.{animation}.json")):
        session = runtime.ToolSession()
        try:
            posters.render_frame(scene, settings, out / "work", session)
        except (ValueError, RuntimeError):
            pass
    media = out / "work" / "m" / str(100 + jobs.index(job)) / "i"
    images = list(media.glob("*.png"))
    if not images:
        return scene, index, animation, segment["scene"], None
    target = out / f"{scene}.{index}.{animation}.raw.png"
    shutil.copyfile(images[0], target)
    with Image.open(target) as image:
        share = posters.coverage(image.convert("RGB").resize(posters.SIZE))
    return scene, index, animation, segment["scene"], round(share, 3)


out.mkdir(parents=True, exist_ok=True)
with ThreadPoolExecutor(2) as pool:
    for result in pool.map(work, jobs):
        print(*result, flush=True)
