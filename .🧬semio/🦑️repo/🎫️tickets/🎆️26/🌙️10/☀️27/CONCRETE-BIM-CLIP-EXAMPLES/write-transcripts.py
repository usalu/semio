"""🗣️ Publish accessible transcripts from the current bilingual NARRATION and TITLE of the six BIM scenes.

Usage: python write-transcripts.py
Reads the chapter list from 🧪️cases/🔣️.json, takes TITLE and NARRATION from each scene class (literal parse, import as fallback), writes 🗣️transcript.<language>.md.
Idempotent: output depends only on the scene sources.
"""

import ast
import importlib
import json
import re
import sys
import time
from pathlib import Path

ROOT = next(parent for parent in Path(__file__).resolve().parents if (parent / "🧰️framework").is_dir())
CLIP = ROOT / "🎓️teaching/🏛️architecture/🏗️bim/🎬️clip"
CASES = json.loads((CLIP / "🧪️cases/🔣️.json").read_text(encoding="utf-8"))
LANGUAGES = {
    "en": {"column": 1, "title": "BIM Videos", "intro": "Complete transcripts of the six BIM videos. All values come from one rounded teaching model of the Villa Savoye in Poissy (Le Corbusier and Pierre Jeanneret, 1928–1931); values that are assumed are marked as such in the videos. One paragraph per narrated step, in narration order.", "contents": "Contents", "video": "Video"},
    "de": {"column": 2, "title": "BIM-Videos", "intro": "Vollständige Texte der sechs BIM-Videos. Alle Werte stammen aus einem gerundeten Lehrmodell der Villa Savoye in Poissy (Le Corbusier und Pierre Jeanneret, 1928–1931); angenommene Werte sind in den Videos als solche gekennzeichnet. Ein Absatz pro gesprochenem Schritt, in Sprechreihenfolge.", "contents": "Inhalt", "video": "Video"},
}


def scene_fields(chapter):
    path = CLIP / chapter["folder"] / "🐍️.py"
    tree = ast.parse(path.read_text(encoding="utf-8"))
    scene = next(node for node in tree.body if isinstance(node, ast.ClassDef) and node.name == chapter["scene"])
    fields = {}
    for node in scene.body:
        if isinstance(node, ast.Assign):
            for target in node.targets:
                if isinstance(target, ast.Name) and target.id in {"TITLE", "NARRATION"}:
                    try:
                        fields[target.id] = ast.literal_eval(node.value)
                    except ValueError:
                        fields[target.id] = None
    if len(fields) < 2 or None in fields.values():
        sys.path.insert(0, str(ROOT))
        module = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip." + chapter["folder"] + ".🐍️")
        scene_class = getattr(module, chapter["scene"])
        fields = {"TITLE": scene_class.TITLE, "NARRATION": scene_class.NARRATION}
    return fields


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


def transcript(language, chapters):
    spec = LANGUAGES[language]
    titles = [fields["TITLE"][language] for _, fields in chapters]
    lines = ["# " + spec["title"], "", spec["intro"], "", "## " + spec["contents"], ""]
    lines.extend(f"{number}. [{title}](#{anchor(title)})" for number, title in enumerate(titles, 1))
    lines.append("")
    for (chapter, fields), title in zip(chapters, titles):
        lines.extend(["## " + title, "", f"{spec['video']}: `{chapter['video_id']}`", ""])
        for paragraph in acts(fields["NARRATION"], spec["column"]):
            lines.extend([paragraph, ""])
    return "\n".join(lines).rstrip("\n") + "\n"


def publish(path, text):
    """💾 Write a transcript, retrying while another process holds the file for a moment."""
    for attempt in range(8):
        try:
            path.write_text(text, encoding="utf-8", newline="\n")
            return
        except OSError:
            if attempt == 7:
                raise
            time.sleep(0.5)


def main():
    chapters = [(chapter, scene_fields(chapter)) for chapter in CASES["chapters"]]
    for language in LANGUAGES:
        publish(CLIP / ("🗣️transcript." + language + ".md"), transcript(language, chapters))
        print(language, "transcript published", sum(len(fields["NARRATION"]) for _, fields in chapters), "clauses")


main()
