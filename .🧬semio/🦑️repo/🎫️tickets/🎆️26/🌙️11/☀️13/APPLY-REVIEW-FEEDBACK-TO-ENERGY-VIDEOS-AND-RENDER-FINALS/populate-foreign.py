import importlib
import json
import sys
import tempfile
from pathlib import Path

sys.stdout.reconfigure(encoding="utf-8")
root = Path(__file__).resolve().parents[7]
sys.path.insert(0, str(root))
posters = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation.🖼️posters.🐍️")
videos = root / ".🧬semio/🎓️teaching/⚡️cache/videos"
energy = {"physical_fundamentals", "ventilation", "heating", "cooling", "energy_balance"}
order = [("de", "spectator"), ("en", "spectator"), ("de", "speaker"), ("en", "speaker")]
report = []
for scene, info in posters.chapters().items():
    if info["video"]["id"] in energy:
        continue
    found = None
    for language, audience in order:
        for sidecar in sorted((videos / language / audience).glob(f"*/v/{scene}.render.json")):
            clip = sidecar.with_name(scene + ".mp4")
            if clip.is_file():
                found = (language, audience, clip, json.loads(sidecar.read_text(encoding="utf-8")))
                break
        if found:
            break
    if not found:
        report.append((scene, "NO CLIP"))
        continue
    language, audience, clip, state = found
    segment_state = state["segments"][info["declaration"]["segment"]]
    sources = {posters._relative(path): digest for path, digest in segment_state["sources"].items()}
    with tempfile.TemporaryDirectory() as directory:
        frame = posters.last_frame(clip, Path(directory) / "frame.png")
        posters.store(scene, frame, {"language": language, "profile": state["settings"].get("profile"), "audience": audience}, sources_state=sources)
    report.append((scene, f"{language}/{audience} stale={posters.stale(scene)}"))
for line in report:
    print(*line)
