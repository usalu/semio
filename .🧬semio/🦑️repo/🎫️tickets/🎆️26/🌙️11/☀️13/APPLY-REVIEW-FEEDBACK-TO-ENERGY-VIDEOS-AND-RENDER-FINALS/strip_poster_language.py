"""🧽 Drop the retired ``captured.language`` field from the stored poster sidecars of the neighbouring courses; usage: python -X utf8 strip_poster_language.py."""
import json
from pathlib import Path

ROOT = Path("C:/git/semio/🎓️teaching/🏛️architecture")
for course in ("🎯️optimization", "🏗️bim", "📊️excel", "🦗️grasshopper"):
    for sidecar in sorted((ROOT / course).glob("*/🎬️clip/🖼️posters/*/*.json")):
        text = sidecar.read_text(encoding="utf-8")
        data = json.loads(text)
        if data.get("captured", {}).pop("language", None) is None:
            continue
        sidecar.write_text(json.dumps(data, ensure_ascii="\\u" in text, indent=2) + ("\n" if text.endswith("\n") else ""), encoding="utf-8", newline="\n")
        print("stripped", sidecar.parent.name, sidecar.name)
