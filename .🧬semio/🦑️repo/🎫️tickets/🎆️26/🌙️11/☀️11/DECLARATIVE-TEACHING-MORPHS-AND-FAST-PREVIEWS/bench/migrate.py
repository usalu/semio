"""Batch E1 clip migration helper: exact-swap edits of a clip module plus its transitions document (scratch tool, ticket-local)."""

import json
import re
from pathlib import Path

MODULE = '_morph = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.🐍️")\nTRANSITIONS = Path(__file__).resolve().parent / "🔀️transitions" / "🔣️.json"\n'
STAGE = "        stage = _morph.stage(self, TRANSITIONS)\n"


def T(id, intent, source, target, recipe, duration, *, relation="one-to-one", corr="order", easing="smooth", lag=0.0, limit=None, copy=False, final=None, sampling=None, placement=None, pairs=None):
    def end(value):
        if value is None:
            return None
        out = {"entity": value} if isinstance(value, str) else dict(value)
        return out

    item = {"id": id, "intent": intent, "source": end(source), "target": end(target), "relation": relation, "recipe": recipe}
    if corr is not None:
        item["correspondence"] = {"kind": corr} if pairs is None else {"kind": "pairs", "pairs": pairs}
    item["timing"] = {"duration": duration, "easing": easing}
    if lag:
        item["timing"]["lag"] = lag
    if limit:
        item["timing"]["limit"] = limit
    if copy:
        item["copy"] = True
    if final:
        item["final"] = final
    if sampling:
        for side in ("source", "target"):
            if item[side] is not None:
                item[side]["sampling"] = sampling
    if placement:
        item["placement"] = placement
    return item


def swap(text, old, new, count=1):
    found = text.count(old)
    assert found == count, f"expected {count} occurrence(s), found {found}: {old[:90]!r}"
    return text.replace(old, new)


def run(clip, swaps, transitions, scenes, anchor=None, depth=5, var="stage", module=True):
    """Apply `swaps` [(old, new[, count])], register `scenes` classes' construct with a stage, write transitions."""
    clip = Path(clip)
    path = clip / "🐍️.py"
    text = path.read_text(encoding="utf-8")
    if module and "TRANSITIONS = " not in text:
        anchors = [a for a in ([anchor] if anchor else []) + ["tr = _manim_module.tr\n", "set_vo_language = _manim_module.set_vo_language\n"] if text.count(a) >= 1]
        assert anchors, "no import anchor"
        first = text.index(anchors[0]) + len(anchors[0])
        text = text[:first] + MODULE + text[first:]
    line = STAGE.replace("stage =", f"{var} =")
    for scene in scenes:
        start = re.search(rf"^class {re.escape(scene)}\(", text, re.M)
        assert start, scene
        head = re.compile(r"^    def construct\(self\):\n", re.M).search(text, start.end())
        assert head, scene
        nxt = re.search(r"^class ", text[start.end():], re.M)
        assert nxt is None or head.start() < start.end() + nxt.start(), f"construct outside class {scene}"
        if "_morph.stage(self, TRANSITIONS)" not in text[head.end():head.end() + 400]:
            text = text[: head.end()] + line + text[head.end():]
    for entry in swaps:
        text = swap(text, *entry)
    path.write_text(text, encoding="utf-8", newline="\n")
    folder = clip / "🔀️transitions"
    folder.mkdir(exist_ok=True)
    document = folder / "🔣️.json"
    base = json.loads(document.read_text(encoding="utf-8")) if document.exists() else {"$schema": "../" * depth + "🔀️morph/🧬️schema/🔣️.schema.json", "version": 1, "transitions": []}
    known = {t["id"] for t in base["transitions"]}
    for item in transitions:
        assert item["id"] not in known, item["id"]
        base["transitions"].append(item)
    document.write_text(json.dumps(base, ensure_ascii=False, indent=2) + "\n", encoding="utf-8", newline="\n")
