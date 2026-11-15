import json
from pathlib import Path

manifest = Path(r"C:\git\semio\🎓️teaching\🏛️architecture\⚡️energy\🎬️clip\🔣️.json")
data = json.loads(manifest.read_text(encoding="utf-8"))
for video in data["videos"]:
    print("VIDEO", video.get("id"))
    for chapter in video["chapters"]:
        print("  CH", chapter.get("id") or chapter.get("name"), [(s["script"].split("/")[-2], s["scene"]) for s in chapter["segments"]])
