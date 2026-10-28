"""🎞️ Independently compare encoded caption regions across audiences and languages."""

import ast
import json
import math
import os
import sys
from pathlib import Path

import av
import numpy as np
from PIL import Image, ImageDraw

sys.dont_write_bytecode = True
TICKET = Path(__file__).resolve().parent
ROOT = next(parent for parent in TICKET.parents if (parent / "🧰️framework").is_dir())
CLIP = ROOT / "🎓️teaching/🏛️architecture/🏗️bim/🎬️clip"
CASES = json.loads((CLIP / "🧪️cases/🔣️.json").read_text(encoding="utf-8"))
CONTRACT = json.loads((ROOT / "🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🔣️.json").read_text(encoding="utf-8"))
OUTPUT = TICKET / "🗑️generated/f"
EVIDENCE = TICKET / "🗑️generated/audience"
EVIDENCE.mkdir(parents=True, exist_ok=True)
VARIANTS = [(language, audience) for language in ("en", "de") for audience in ("speaker", "spectator")]


def duration(path):
    """⏱️ Read actual encoded video duration and rate without decoding the full movie."""
    with av.open(str(path)) as container:
        stream = container.streams.video[0]
        seconds = float(stream.duration * stream.time_base) if stream.duration else float(stream.frames / stream.average_rate)
        return seconds, float(stream.average_rate)


def frame(path, seconds):
    """🖼️ Decode the first encoded frame at or after a requested stable source instant."""
    with av.open(str(path)) as container:
        stream = container.streams.video[0]
        container.seek(max(0, int(seconds / float(stream.time_base))), stream=stream, backward=True)
        latest = None
        for decoded in container.decode(stream):
            latest = decoded
            if float(decoded.pts * stream.time_base) >= seconds - 0.001:
                return decoded.to_image().convert("RGB"), float(decoded.pts * stream.time_base)
        return latest.to_image().convert("RGB"), float(latest.pts * stream.time_base)


def caption_bounds(image):
    """📐 Map the source caption zone through the authored contained body viewport."""
    viewport = CONTRACT["focus"]["viewport"]
    sx, sy = image.width / CONTRACT["viewport"]["width"], image.height / CONTRACT["viewport"]["height"]
    width, height = round(viewport["width"] * sx), round(viewport["height"] * sy)
    source_width = height * 16 / 9
    left = viewport["x"] * sx + (width - source_width) / 2
    top = viewport["y"] * sy
    return tuple(round(value) for value in (left + source_width * 0.04, top + height * 0.82,
                                           left + source_width * 0.96, top + height * 0.992))


def source_marks(folder, source, scene):
    """🗺️ Read certified source marks or the renderer's per-scene trace facets."""
    certified = source.with_suffix(".trace.json")
    if certified.is_file():
        return json.loads(certified.read_text(encoding="utf-8"))["marks"]
    traces = list(folder.glob("trace/*/" + scene + "/🔣️.json"))
    if len(traces) != 1:
        raise ValueError("A unique source trace is required for " + scene)
    return json.loads(traces[0].read_text(encoding="utf-8"))["beats"][scene]


def bright_count(image):
    """🔬 Count encoded bright glyph pixels independently of the drawing implementation."""
    pixels = np.asarray(image).astype(float)
    light = pixels.mean(axis=2)
    return int(np.count_nonzero(light > 145))


results, pending, failures = [], [], []
for chapter in CASES["chapters"]:
    found = {}
    for language, audience in VARIANTS:
        folder = OUTPUT / language / audience
        movies = sorted((folder / "rendered").glob(chapter["output"] + "_*.mp4")) if folder.exists() else []
        sources = sorted(folder.glob("s/*/v/" + chapter["scene"] + ".mp4")) if folder.exists() else []
        movies = [path for path in movies if path.with_suffix(".render.json").is_file()]
        sources = [path for path in sources if path.with_suffix(".render.json").is_file()]
        if movies and sources:
            found[(language, audience)] = (movies[-1], sources[-1])
    if len(found) != 4:
        pending.append({"video": chapter["video_id"], "ready": len(found), "missing": ["/".join(item) for item in VARIANTS if item not in found]})
        continue
    try:
        details = {}
        for variant, (movie, source) in found.items():
            seconds, rate = duration(source)
            final_seconds, final_rate = duration(movie)
            marks = source_marks(OUTPUT / variant[0] / variant[1], source, chapter["scene"])
            details[variant] = {"movie": movie, "source": source, "duration": seconds, "marks": marks,
                                "padding": (final_seconds - seconds) / 2, "final_duration": final_seconds, "rate": final_rate}
        common = set.intersection(*(set(item["marks"]) for item in details.values()))
        eligible = [key for key in sorted(common) if all(0.30 <= (item["padding"] + item["marks"][key]["start"] + item["marks"][key]["window"] * 0.55) / item["final_duration"] <= 0.60
                                                for item in details.values())]
        if not eligible:
            raise ValueError("No common narrated clause lies between 30% and 60% of all four movies")
        key = min(eligible, key=lambda clause: sum(abs((item["padding"] + item["marks"][clause]["start"] + item["marks"][clause]["window"] * 0.55) / item["final_duration"] - 0.45)
                                                  for item in details.values()))
        tree = ast.parse((CLIP / chapter["folder"] / "🐍️.py").read_text(encoding="utf-8"))
        scene = next(node for node in tree.body if isinstance(node, ast.ClassDef) and node.name == chapter["scene"])
        narration = next(ast.literal_eval(node.value) for node in scene.body if isinstance(node, ast.Assign)
                         and any(isinstance(target, ast.Name) and target.id == "NARRATION" for target in node.targets))
        clause = next(row for row in narration if row[0] == key)
        row = {"video": chapter["video_id"], "clause": key, "en": clause[1], "de": clause[2], "variants": {}}
        images, crops = {}, {}
        for variant, item in details.items():
            mark = item["marks"][key]
            instant = mark["start"] + mark["window"] * 0.55
            image, decoded_time = frame(item["movie"], item["padding"] + instant)
            if not 0.30 <= decoded_time / item["final_duration"] <= 0.60:
                raise ValueError("Decoded frame lies outside the requested middle-movie interval")
            crop = image.crop(caption_bounds(image))
            images[variant], crops[variant] = image, crop
            record = json.loads(item["movie"].with_suffix(".render.json").read_text(encoding="utf-8"))
            assert record["presentation"]["language"] == variant[0]
            row["variants"]["/".join(variant)] = {"seconds": decoded_time, "source_seconds": instant, "movie_fraction": decoded_time / item["final_duration"],
                                                   "bright_pixels": bright_count(crop), "padding": item["padding"],
                                                   "movie": item["movie"].relative_to(TICKET).as_posix()}
        for language in ("en", "de"):
            speaker, spectator = crops[(language, "speaker")], crops[(language, "spectator")]
            visible, hidden = bright_count(speaker), bright_count(spectator)
            if visible < 80 or visible < hidden + 60 or hidden > max(40, visible * 0.08):
                raise ValueError(f"{language}: caption glyph evidence insufficient ({visible} vs {hidden})")
            row[language + "_speaker_minus_spectator"] = visible - hidden
        first = np.asarray(crops[("en", "speaker")]).astype(float)
        second = np.asarray(crops[("de", "speaker")].resize(crops[("en", "speaker")].size)).astype(float)
        row["caption_language_pixel_difference"] = float(np.mean(np.abs(first - second)))
        if row["caption_language_pixel_difference"] < 1.0:
            raise ValueError("English and German visible caption regions do not differ")
        sheet = Image.new("RGB", (1280, 760), "#181818")
        draw = ImageDraw.Draw(sheet)
        strips = Image.new("RGB", (1100, 520), "#181818")
        strip_draw = ImageDraw.Draw(strips)
        for index, variant in enumerate(VARIANTS):
            x, y = (index % 2) * 640, (index // 2) * 380
            sheet.paste(images[variant].resize((640, 360)), (x, y + 20))
            draw.text((x + 5, y + 4), "/".join(variant), fill="white")
            strip = crops[variant].resize((1080, 110))
            strips.paste(strip, (10, index * 130 + 20))
            strip_draw.text((10, index * 130 + 3), "/".join(variant), fill="white")
        sheet.save(EVIDENCE / (chapter["video_id"] + ".png"))
        strips.save(EVIDENCE / (chapter["video_id"] + "-captions.png"))
        results.append(row)
    except Exception as error:
        failures.append({"video": chapter["video_id"], "error": str(error)})
payload = {"checked_topics": len(results), "pending": pending, "failures": failures, "results": results}
(EVIDENCE / "results.json").write_text(json.dumps(payload, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
report = ["# Encoded Video Audience and Language Audit", "", "Independent PyAV decoding and Pillow/NumPy pixel analysis; no production module imports.", "",
          f"Complete four-variant topics checked: {len(results)}/6.", f"Pending topics: {len(pending)}.", f"Failed checks: {len(failures)}.", "",
          "Glyph classifier: average RGB above 145; Speaker needs at least 80 bright pixels, Spectator at most max(40, 8% of Speaker), and the audience difference at least 60 pixels. Language caption mean RGB difference must exceed 1.0.", "",
          "Frames use a shared narrated clause, independently located through each source trace. All selected instants lie in the middle 30–60% of the finished movie; measured envelope duration supplies the source offset.", "",
          "| Video | Clause | English Speaker − Spectator Bright Pixels | German Speaker − Spectator Bright Pixels | EN/DE Caption Mean RGB Difference |",
          "| --- | --- | ---: | ---: | ---: |"]
for row in results:
    report.append(f"| {row['video']} | {row['clause']} | {row['en_speaker_minus_spectator']} | {row['de_speaker_minus_spectator']} | {row['caption_language_pixel_difference']:.3f} |")
report.extend(["", "## Expected Caption Text", ""])
for row in results:
    report.extend([f"- {row['video']} ({row['clause']}):", f"  - EN: {row['en']}", f"  - DE: {row['de']}"])
if pending:
    report.extend(["", "## Pending", "", *(f"- {row['video']}: {', '.join(row['missing'])}" for row in pending)])
if failures:
    report.extend(["", "## Failures", "", *(f"- {row['video']}: {row['error']}" for row in failures)])
report.extend(["", "Pixel differences verify visible audience/language distinctions; authored clauses and published certificates bind the expected language. This does not perform OCR.", "",
               "Contact sheets and quantitative JSON live under ticket 🗑️generated/audience; the audit input script is preserved as audience-audit.py.", ""])
(TICKET / "media-audience.md").write_text("\n".join(report), encoding="utf-8")
print(json.dumps({"checked_topics": len(results), "pending": len(pending), "failures": failures}, ensure_ascii=False))
if __name__ == "__main__":
    raise SystemExit(1 if failures else 0)
