"""🧰 R01 repair kit: targeted edits of catalog asset files (one shape of one view of one scale, one valid range) and of the volumes sources, re-reading every file right before writing; every byte outside the edited object stays as it was.

@see ./r5-range-gaps.md
@see ./volumes-insert.py — splices a volumes source into the catalog file
"""
import glob
import json
import os
import re
import subprocess
from shutil import which
from pathlib import Path

T = os.path.dirname(os.path.abspath(__file__))
CATALOG = os.path.normpath(os.path.join(T, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog"))
BUN = which("bun") or next((str(p) for p in (Path(T).parents[6] / ".cache/b/t/m/bun").glob("*/*") if p.name in ("bun", "bun.exe")), "bun")


def asset_path(spec):
    """📂 The catalog file of `category/slug`."""
    category, slug = spec.split("/")
    return [p for p in glob.glob(os.path.join(CATALOG, "*" + category, slug, "🔣️.json"))][0]


def source_path(spec):
    """📂 The volumes source of `category/slug`."""
    category, slug = spec.split("/")
    return os.path.join(T, "volumes", category, slug + ".json")


def object_end(text, start):
    """🔚 Index after the JSON value that starts at `start`."""
    depth, index, quoted = 0, start, False
    while index < len(text):
        char = text[index]
        if quoted:
            if char == "\\":
                index += 1
            elif char == '"':
                quoted = False
        elif char == '"':
            quoted = True
        elif char in "{[":
            depth += 1
        elif char in "}]":
            depth -= 1
            if depth == 0:
                return index + 1
        index += 1
    raise SystemExit("unbalanced json")


def key_value(text, key, start, end):
    """🔑 Span of the value of the first member `key` in text[start:end]."""
    found = re.compile(r'"' + re.escape(key) + r'"\s*:\s*').search(text, start, end)
    if found is None:
        raise SystemExit(f"member {key} not found")
    return found.end(), object_end(text, found.end())


def scalar_list(value):
    return isinstance(value, list) and all(not isinstance(v, (dict, list)) or (isinstance(v, list) and all(not isinstance(w, (dict, list)) for w in v)) for v in value)


def render(value, indent, unit):
    """🖨️ JSON text in the catalog style: objects one member per line, lists of numbers or points inline."""
    pad, inner = " " * indent, " " * (indent + unit)
    if isinstance(value, dict):
        if not value:
            return "{}"
        return "{\n" + ",\n".join(inner + json.dumps(k, ensure_ascii=False) + ": " + render(v, indent + unit, unit) for k, v in value.items()) + "\n" + pad + "}"
    if isinstance(value, list):
        if not value:
            return "[]"
        if scalar_list(value):
            return json.dumps(value, ensure_ascii=False)
        return "[\n" + ",\n".join(inner + render(v, indent + unit, unit) for v in value) + "\n" + pad + "]"
    return json.dumps(value, ensure_ascii=False)


def shape_span(text, scale, view, shape_id):
    """🎯 Span of the shape object `shape_id` in `representations[scale].geometry[view]`."""
    reps = key_value(text, "representations", 0, len(text))
    rep = key_value(text, str(scale), reps[0], reps[1])
    geometry = key_value(text, "geometry", rep[0], rep[1])
    shapes = key_value(text, view, geometry[0], geometry[1])
    index = shapes[0] + 1
    while True:
        opening = text.index("{", index)
        if opening >= shapes[1]:
            raise SystemExit(f"shape {shape_id} not in {scale} {view}")
        close = object_end(text, opening)
        if json.loads(text[opening:close]).get("id") == shape_id:
            return opening, close
        index = close


def edit_shapes(spec, edits):
    """✏️ Applies `edits` = [(scale, view, id, function(shape) -> shape)] to the catalog file, re-read right before writing; returns the old and new shapes."""
    path = asset_path(spec)
    text = open(path, encoding="utf-8").read()
    log = []
    for scale, view, shape_id, change in edits:
        start, end = shape_span(text, scale, view, shape_id)
        old = json.loads(text[start:end])
        new = change(json.loads(json.dumps(old)))
        line_start = text.rindex("\n", 0, start) + 1
        indent = start - line_start
        unit = re.search(r"\n( +)\"", text[start:end]).group(1).__len__() - indent
        text = text[:start] + render(new, indent, unit) + text[end:]
        log.append((scale, view, shape_id, old, new))
    json.loads(text)
    open(path, "w", encoding="utf-8").write(text)
    return log


def set_range(spec, scale, low, high):
    """📏 Sets `representations[scale].validRange` to [low, high] by a targeted replacement."""
    path = asset_path(spec)
    text = open(path, encoding="utf-8").read()
    reps = key_value(text, "representations", 0, len(text))
    rep = key_value(text, str(scale), reps[0], reps[1])
    span = key_value(text, "validRange", rep[0], rep[1])
    old = json.loads(text[span[0]:span[1]])
    body = text[span[0]:span[1]]
    body = re.sub(r'("minDenominator"\s*:\s*)[0-9.]+', lambda m: m.group(1) + json.dumps(low), body)
    body = re.sub(r'("maxDenominator"\s*:\s*)[0-9.]+', lambda m: m.group(1) + json.dumps(high), body)
    text = text[:span[0]] + body + text[span[1]:]
    assert json.loads(text)["asset"]["representations"][str(scale)]["validRange"] == {"minDenominator": low, "maxDenominator": high}
    open(path, "w", encoding="utf-8").write(text)
    return old


def edit_solids(spec, scale, edits):
    """🧊 Applies `edits` = [(list, id, function(solid) -> solid)] to the volumes source of `scale` (re-read right before writing) and splices it into the catalog."""
    path = source_path(spec)
    source = json.load(open(path, encoding="utf-8"))
    log = []
    for name, solid_id, change in edits:
        solids = source["volumes"][str(scale)][name]
        index = next(i for i, s in enumerate(solids) if s["id"] == solid_id)
        old = solids[index]
        solids[index] = change(json.loads(json.dumps(old)))
        log.append((scale, name, solid_id, old, solids[index]))
    with open(path, "w", encoding="utf-8") as f:
        json.dump(source, f, ensure_ascii=False, indent=2)
        f.write("\n")
    subprocess.run(["python3", os.path.join(T, "volumes-insert.py"), path, asset_path(spec)], check=True)
    return log


def validate(spec, volumes=True):
    """✅ `r3-validate.ts` of the asset; returns (exit code, output lines)."""
    run = subprocess.run([BUN, os.path.join(T, "r3-validate.ts"), spec] + (["--volumes"] if volumes else []), cwd=T, capture_output=True, text=True)
    return run.returncode, (run.stdout + run.stderr).strip().split("\n")
