"""🗣️ Publish accessible transcripts of the Grasshopper and Excel videos from the current bilingual NARRATION and TITLE of their scenes.

Usage: python write-transcripts.py
Imports each scene class (the narration may be assembled in code), writes 🗣️transcript.<language>.md per series. Idempotent.
"""

from __future__ import annotations

import importlib
import os
import re
import sys
import time
from pathlib import Path

os.environ.setdefault("SEMIO_TEACHING_LANGUAGE", "en")
sys.dont_write_bytecode = True
from series import ARCH, ROOT, SERIES

sys.path.insert(0, str(ROOT))
TEXT = {
    "grasshopper": {
        "en": {"title": "Grasshopper Videos", "intro": "Complete transcripts of the five Grasshopper videos. All values come from one rounded teaching model of the Nakagin Capsule Tower in Ginza, Tokyo (Kisho Kurokawa, 1970–1972, demolished 2022); values that are assumed are marked as such in the videos. One paragraph per narrated step, in narration order."},
        "de": {"title": "Grasshopper-Videos", "intro": "Vollständige Texte der fünf Grasshopper-Videos. Alle Werte stammen aus einem gerundeten Lehrmodell des Nakagin Capsule Tower in Ginza, Tokio (Kisho Kurokawa, 1970–1972, 2022 abgerissen); angenommene Werte sind in den Videos als solche gekennzeichnet. Ein Absatz pro gesprochenem Schritt, in Sprechreihenfolge."},
    },
    "excel": {
        "en": {"title": "Excel Videos", "intro": "Complete transcripts of the four Excel videos. All values come from one rounded teaching model of the Nakagin Capsule Tower in Ginza, Tokyo (Kisho Kurokawa, 1970–1972, demolished 2022); values that are assumed are marked as such in the videos. One paragraph per narrated step, in narration order."},
        "de": {"title": "Excel-Videos", "intro": "Vollständige Texte der vier Excel-Videos. Alle Werte stammen aus einem gerundeten Lehrmodell des Nakagin Capsule Tower in Ginza, Tokio (Kisho Kurokawa, 1970–1972, 2022 abgerissen); angenommene Werte sind in den Videos als solche gekennzeichnet. Ein Absatz pro gesprochenem Schritt, in Sprechreihenfolge."},
    },
}
COLUMN = {"en": 1, "de": 2}
LABEL = {"en": ("Contents", "Video"), "de": ("Inhalt", "Video")}


def acts(narration, column):
    groups = []
    for row in narration:
        stem = re.sub(r"_\d+$", "", row[0])
        if groups and groups[-1][0] == stem:
            groups[-1][1].append(row[column].strip())
        else:
            groups.append((stem, [row[column].strip()]))
    return [" ".join(texts) for _, texts in groups]


def anchor(title):
    return re.sub(r"\s+", "-", re.sub(r"[^\w\s-]", "", title.lower()).strip())


def transcript(key, language, scenes):
    contents, video = LABEL[language]
    titles = [cls.TITLE[language] for _, cls in scenes]
    lines = ["# " + TEXT[key][language]["title"], "", TEXT[key][language]["intro"], "", "## " + contents, ""]
    lines.extend(f"{number}. [{title}](#{anchor(title)})" for number, title in enumerate(titles, 1))
    lines.append("")
    for (video_id, cls), title in zip(scenes, titles):
        lines.extend(["## " + title, "", f"{video}: `{video_id}`", ""])
        for paragraph in acts(cls.NARRATION, COLUMN[language]):
            lines.extend([paragraph, ""])
    return "\n".join(lines).rstrip("\n") + "\n"


def publish(path, text):
    for attempt in range(8):
        try:
            path.write_text(text, encoding="utf-8", newline="\n")
            return
        except OSError:
            if attempt == 7:
                raise
            time.sleep(0.5)


def main():
    for key, series in SERIES.items():
        scenes = [(video, getattr(importlib.import_module(f"🎓️teaching.🏛️architecture.{series['folder']}.🎬️clip.{folder}.🐍️"), scene))
                  for video, scene, folder, *_ in series["videos"]]
        for language in COLUMN:
            publish(ARCH / series["folder"] / "🎬️clip" / f"🗣️transcript.{language}.md", transcript(key, language, scenes))
        print(f"[DEBUG] {key}: transcripts published, {sum(len(cls.NARRATION) for _, cls in scenes)} clauses")


main()
