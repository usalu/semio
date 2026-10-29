"""🧾 Register the Grasshopper and Excel videos in the manifest, titles, presentation and portable test cases (idempotent)."""

from __future__ import annotations

import importlib
import json
import os
import sys
from pathlib import Path

os.environ.setdefault("SEMIO_TEACHING_LANGUAGE", "en")
sys.dont_write_bytecode = True
from series import ARCH, CLIP_AUTHORING, ROOT, SERIES

sys.path.insert(0, str(ROOT))
MANIFEST = ARCH / "⚡️energy" / "🎬️clip" / "🔣️.json"
PRESENTATION = CLIP_AUTHORING / "🪟️presentation"
CASES = {
    "contract": CLIP_AUTHORING / "🧪️tests" / "🔣️.cases.json",
    "presentation": PRESENTATION / "🧪️tests" / "🔣️.cases.json",
    "hierarchy": PRESENTATION / "🥞️hierarchy" / "🧪️tests" / "🔣️.cases.json",
    "chapters": PRESENTATION / "🥞️hierarchy" / "🧪️tests" / "🧩️chapters" / "🔣️.cases.json",
}


def load(path: Path):
    return json.loads(path.read_text(encoding="utf-8"))


def save(path: Path, data) -> None:
    path.write_text(json.dumps(data, indent=2, ensure_ascii=False) + "\n", encoding="utf-8", newline="\n")


def videos() -> list[dict]:
    rows = []
    for key, series in SERIES.items():
        for video, scene, folder, chapter, output, icon, title in series["videos"]:
            module = importlib.import_module(f"🎓️teaching.🏛️architecture.{series['folder']}.🎬️clip.{folder}.🐍️")
            title = dict(getattr(getattr(module, scene), "TITLE", title))
            rows.append({"topic": key, "id": video, "scene": scene, "script": f"🏛️architecture/{series['folder']}/🎬️clip/{folder}/🐍️.py",
                         "entry": f"🏛️architecture/{series['folder']}/{folder}/🎬️clip/🐍️.py", "chapter": chapter, "output": output, "icon": icon, "title": title})
    return rows


def place(items: list, entries: list, key=lambda item: item["id"]) -> list:
    ids = {key(entry) for entry in entries}
    kept = [item for item in items if key(item) not in ids]
    anchor = max(index for index, item in enumerate(kept) if key(item).startswith("bim_")) + 1
    return kept[:anchor] + entries + kept[anchor:]


def main() -> None:
    rows = videos()
    manifest = load(MANIFEST)
    manifest["videos"] = place(manifest["videos"], [{"id": row["id"], "script": row["entry"], "chapters": [{"scene": row["chapter"], "segments": [{"script": row["script"], "scene": row["scene"]}]}], "output": row["output"]} for row in rows])
    save(MANIFEST, manifest)

    path = PRESENTATION / "📚️collection" / "🔣️.json"
    collection = load(path)
    for row in rows:
        collection["chapters"][row["chapter"]] = row["title"]
    save(path, collection)

    path = PRESENTATION / "🔣️.json"
    presentation = load(path)
    for key in SERIES:
        topic = next(topic for topic in presentation["topics"] if topic["id"] == key)
        topic["videos"] = [{"id": row["id"], "title": row["title"], "icon": row["icon"], "symbol": key, "cell": {"column": index % 3, "row": index // 3}}
                           for index, row in enumerate(r for r in rows if r["topic"] == key)]
    save(path, presentation)

    data = load(CASES["contract"])
    curriculum = data["curriculum"]
    known = {name: (chapters, segments) for name, chapters, segments in zip(curriculum["ids"], curriculum["chapter_counts"], curriculum["segment_counts"])}
    known.update({row["id"]: (1, [1]) for row in rows})
    order = place([{"id": name} for name in curriculum["ids"]], [{"id": row["id"]} for row in rows])
    curriculum["ids"] = [item["id"] for item in order]
    curriculum["chapter_counts"] = [known[name][0] for name in curriculum["ids"]]
    curriculum["segment_counts"] = [known[name][1] for name in curriculum["ids"]]
    totals = [sum(counts) for counts in curriculum["segment_counts"]]
    curriculum["first_segment_ordinals"] = [1 + sum(totals[:index]) for index in range(len(totals))]
    curriculum["render_segment_count"] = sum(totals)
    curriculum["logical_clip_count"] = sum(curriculum["chapter_counts"])
    save(CASES["contract"], data)

    data = load(CASES["presentation"])
    for key in SERIES:
        topic = next(topic for topic in data["topics"] if topic["id"] == key)
        topic["videos"] = [{"id": row["id"], "title": row["title"]} for row in rows if row["topic"] == key]
    save(CASES["presentation"], data)

    data = load(CASES["hierarchy"])
    for key in SERIES:
        topic = next(topic for topic in data["topics"] if topic["id"] == key)
        topic["videos"] = [row["id"] for row in rows if row["topic"] == key]
    save(CASES["hierarchy"], data)

    data = load(CASES["chapters"])
    clips = place(data["clips"], [{"id": row["id"], "topic": row["topic"], "chapters": [{"scene": row["chapter"], "segments": [{"script": row["script"], "scene": row["scene"]}]}]} for row in rows])
    data["clips"] = clips
    chapters = [(clip["topic"], clip["id"], chapter["scene"]) for clip in clips for chapter in clip["chapters"]]
    sources = {(segment["script"], segment["scene"]) for clip in clips for chapter in clip["chapters"] for segment in chapter["segments"]}
    occurrences = sum(len(chapter["segments"]) for clip in clips for chapter in clip["chapters"])
    data["counts"].update({"clips": len(clips), "chapters": len(chapters), "canonical_sources": len(sources), "segment_occurrences": occurrences})
    boundaries = [{"left": None, "right": chapters[0][2], "path": [], "level": "courses"}]
    for left, right in zip(chapters, chapters[1:]):
        if left[0] != right[0]:
            path, level = [data["course"]], "topics"
        elif left[1] != right[1]:
            path, level = [data["course"], left[0]], "clips"
        else:
            path, level = [data["course"], left[0], left[1]], "chapters"
        boundaries.append({"left": left[2], "right": right[2], "path": path, "level": level})
    boundaries.append({"left": chapters[-1][2], "right": None, "path": [], "level": "courses"})
    data["boundaries"] = boundaries
    save(CASES["chapters"], data)
    for key, series in SERIES.items():
        mine = [row for row in rows if row["topic"] == key]
        cases = {"series": key, "folder": series["folder"], "languages": ["en", "de"], "chapters": [
            {"folder": SERIES[key]["videos"][index][2], "scene": row["scene"], "video_id": row["id"], "entry": row["entry"], "chapter": row["chapter"], "title": row["title"], "output": row["output"]}
            for index, row in enumerate(mine)]}
        save(ARCH / series["folder"] / "🎬️clip" / "🧪️tests" / "📇️registry" / "🔣️.cases.json", cases)
    print(f"[DEBUG] registered {len(rows)} videos, {len(chapters)} chapters, {len(boundaries)} boundaries")


if __name__ == "__main__":
    main()
