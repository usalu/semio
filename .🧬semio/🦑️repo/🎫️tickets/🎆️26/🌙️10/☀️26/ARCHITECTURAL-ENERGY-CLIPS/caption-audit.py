"""💬 Audit every canonical narration clause through the real caption layout interface."""

import ast
import gc
import importlib
import json
import os
import sys
import time
from pathlib import Path

TICKET = Path(__file__).resolve().parent
WORKSPACE = next(parent for parent in TICKET.parents if (parent / "🧰️framework").is_dir())
sys.path.insert(0, str(WORKSPACE))
OUTPUT = TICKET / "🗑️generated" / "captions"
OUTPUT.mkdir(parents=True, exist_ok=True)
os.environ["SEMIO_TEACHING_OUTPUT_ROOT"] = str(OUTPUT)
from manim import tempconfig
BASE = "🎓️teaching.🏛️architecture.⚡️energy."
runtime = importlib.import_module(BASE + "🏗️authoring.🎬️clip.⚙️runtime.🐍️")
fonts = importlib.import_module(BASE + "🏗️authoring.🔤️fonts.🐍️")
visuals = importlib.import_module(BASE + "🏗️authoring.🎨️visuals.🐍️")
fonts.apply_body_font()
selected = set(sys.argv[1:])
jobs = runtime.source_plan(runtime.segment_plan())
results, cached = [], {}
prior = OUTPUT / "results.json"
if prior.exists():
    for row in json.loads(prior.read_text(encoding="utf-8")):
        cached[row["text"]] = row.get("error")
import atexit
setup = tempconfig({"text_dir": str(OUTPUT / "svg"), "media_dir": str(OUTPUT), "verbosity": "ERROR"})
setup.__enter__()
atexit.register(setup.__exit__, None, None, None)
original_wrap = fonts.wrap_body_lines

def boundary_wrap(text, *, font_size, max_width):
    """📏 Verify exact greedy boundaries against real shaped line widths."""
    wrapped = []
    color = fonts._theme.CHROME_FOREGROUND
    for paragraph in text.split("\n"):
        words = paragraph.split()
        if not words:
            continue
        normalized = " ".join(words)
        shape = fonts.body_text(normalized, font_size=font_size, color=color)
        if shape.width <= max_width:
            wrapped.append(normalized)
            continue
        centers = [char.get_center()[0] for index, char in enumerate(shape.chars) if not normalized[index].isspace()]
        if any(right < left - 1e-8 for left, right in zip(centers, centers[1:])):
            wrapped.extend(original_wrap(normalized, font_size=font_size, max_width=max_width))
            continue
        starts, cursor = [], 0
        for word in words:
            starts.append(cursor)
            cursor += len(word) + 1
        start = 0
        while start < len(words):
            stop = start + 1
            while stop < len(words):
                end = starts[stop] + len(words[stop])
                if shape.chars[starts[start]:end].width > max_width:
                    break
                stop += 1
            line = " ".join(words[start:stop])
            while stop > start + 1 and fonts.body_text(line, font_size=font_size, color=color).width > max_width:
                stop -= 1
                line = " ".join(words[start:stop])
            while stop < len(words):
                candidate = " ".join(words[start:stop + 1])
                if fonts.body_text(candidate, font_size=font_size, color=color).width > max_width:
                    break
                stop += 1
                line = candidate
            wrapped.append(line)
            start = stop
    return wrapped or [""]

for sample in ["A short caption.", "Ein Elektroheizstab liefert am Haus genau 1.",
               "Am Haus liefert ein Elektroheizstab genau 1. Bei Brennstoffen geht der Rest mit dem Abgas verloren — Strom verursacht gegebenenfalls anderswo Abgas.",
               "Nur der Anteil 1 − η bleibt als Verlust: bei η"]:
    actual = original_wrap(sample, font_size=fonts.CAPTION_FONT_SIZE, max_width=visuals.CAPTION_MAX_WIDTH)
    improved = boundary_wrap(sample, font_size=fonts.CAPTION_FONT_SIZE, max_width=visuals.CAPTION_MAX_WIDTH)
    assert actual == improved, (sample, actual, improved)
fonts.wrap_body_lines = boundary_wrap
print("[DEBUG] Exact boundary wrapping matches the original interface on short, long, and fallback-glyph examples", flush=True)
started = time.monotonic()
with tempconfig({"text_dir": str(OUTPUT / "svg"), "media_dir": str(OUTPUT), "verbosity": "ERROR"}):
    for number, job in enumerate(jobs, 1):
        segment = job["segment"]
        if selected and segment["scene"] not in selected:
            continue
        path = WORKSPACE / "🎓️teaching" / segment["script"]
        tree = ast.parse(path.read_text(encoding="utf-8"))
        node = next((node for node in tree.body if isinstance(node, ast.ClassDef) and node.name == segment["scene"]), None)
        assignment = next((node for node in (node.body if node is not None else []) if isinstance(node, ast.Assign)
                           and any(isinstance(target, ast.Name) and target.id == "NARRATION" for target in node.targets)), None)
        try:
            narration = ast.literal_eval(assignment.value) if assignment is not None else None
        except (ValueError, TypeError):
            narration = None
        if narration is None:
            module = importlib.import_module("🎓️teaching." + segment["script"].removesuffix(".py").replace("/", "."))
            narration = getattr(getattr(module, segment["scene"]), "NARRATION", [])
        count = 0
        for row in narration:
            for index, language in ((1, "en"), (2, "de")):
                text = row[index]
                if text not in cached:
                    try:
                        visuals.caption_bar(text)
                        cached[text] = None
                    except Exception as error:
                        cached[text] = str(error)
                result = {"script": segment["script"], "scene": segment["scene"], "key": row[0],
                          "language": language, "text": text, "status": "failed" if cached[text] else "passed"}
                if cached[text]:
                    result["error"] = cached[text]
                    print(f"[DEBUG] Caption overflow: {segment['scene']} {row[0]} {language}: {cached[text]}", flush=True)
                results.append(result)
                count += 1
        (OUTPUT / "results.json").write_text(json.dumps(results, ensure_ascii=False, indent=2), encoding="utf-8")
        failures = [row for row in results if row["status"] == "failed"]
        report = "# Canonical Caption Audit\n\n"
        report += f"Checked {len(results)} language-specific clauses across {number}/{len(jobs)} canonical sources in {time.monotonic()-started:.1f} s.\n\n"
        report += f"Current overflowing clauses: {len(failures)}. Actual shared caption_bar and bundled Pango typography were used for every unique clause. The ticket-only boundary search verifies greedy line endpoints with the original body_text measurements and first checks equivalence against the original wrapping implementation.\n\n"
        report += "\n".join(f"- `{row['scene']}` / `{row['key']}` / `{row['language']}`: {row['text']}" for row in failures)
        (TICKET / "caption-audit.md").write_text(report + "\n", encoding="utf-8")
        print(f"[DEBUG] Caption source {number}/{len(jobs)}: {segment['scene']} ({count} clauses); {len(failures)} overflows so far", flush=True)
        gc.collect()
print(f"[DEBUG] Caption audit complete: {len(results)} clauses; {len(failures)} overflows", flush=True)
sys.exit(bool(failures))
