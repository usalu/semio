#!/usr/bin/env python3
"""Generates the mutate-semio-presentation case fixtures.

Base state is a faithful transcription of the REAL committed example artifact
`🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/📽️deck/🖼️assets/🗣️example.dsl.semio`
(byte-identical to `demo_semio_presentation_snapshot()`).

Apply semantics mirror `🪆️subsets/✳️presentation/🧬️schema/🔺️diff/🦀️component.rs`:
  masters/layouts  -> apply_named   (retain, patch, push added at the END)
  slides/shapes/notes/blocks -> apply_indexed (patch, remove desc, insert at index)
"""
import copy, json, os, sys

OUT = sys.argv[1]

PLAIN = {"bold": False, "italic": False, "underline": False, "size": None, "font": None, "color": None, "link": None}


def para(text):
    return {"kind": "paragraph", "style_id": None, "runs": [{"text": text, "style": copy.deepcopy(PLAIN)}]}


def frame(x, y, w, h):
    return {"origin": {"x": x, "y": y}, "width": w, "height": h}


BASE = {
    "schema": "s.stdio.semio.presentation",
    "masters": [{"id": "master1", "shapes": [{"shapeKind": "placeholder", "frame": frame(0.0, 0.0, 100.0, 20.0), "kind": {"kind": "title"}}]}],
    "layouts": [
        {
            "id": "layout1",
            "masterId": "master1",
            "shapes": [{"shapeKind": "placeholder", "frame": frame(0.0, 30.0, 100.0, 15.0), "kind": {"kind": "subtitle"}}],
        }
    ],
    "slides": [
        {
            "id": "slide1",
            "layoutId": "layout1",
            "shapes": [
                {"shapeKind": "textBox", "frame": frame(1.0, 2.0, 50.0, 10.0), "blocks": [para("Hello Slide")]},
                {"shapeKind": "picture", "frame": frame(0.0, 0.0, 10.0, 10.0), "image": {"assetId": "img1", "mime": "image/png", "bytes": [1, 2, 3]}},
                {"shapeKind": "table", "frame": frame(0.0, 0.0, 30.0, 30.0), "rows": [{"cells": [{"blocks": [para("cell")]}]}]},
                {"shapeKind": "placeholder", "frame": frame(0.0, 40.0, 100.0, 10.0), "kind": {"kind": "other", "value": "custom"}},
            ],
            "notes": [para("Speaker notes")],
        }
    ],
}

MASTER2 = {"id": "m-2", "shapes": [{"shapeKind": "placeholder", "frame": frame(0.0, 0.0, 100.0, 24.0), "kind": {"kind": "footer"}}]}
LAYOUT2 = {"id": "l-2", "masterId": "master1", "shapes": [{"shapeKind": "placeholder", "frame": frame(0.0, 60.0, 100.0, 12.0), "kind": {"kind": "body"}}]}
SLIDE2 = {"id": "slide2", "layoutId": "layout1", "shapes": [], "notes": []}
AGENDA = {"shapeKind": "textBox", "frame": frame(2.0, 4.0, 40.0, 8.0), "blocks": [para("Agenda")]}


def named(items, removed=(), modified=(), added=()):
    out = [i for i in items if i["id"] not in removed]
    for key, patch in modified:
        for item in out:
            if item["id"] == key:
                item.update(copy.deepcopy(patch))
    return out + [copy.deepcopy(a) for a in added]


def with_masters(base, **kw):
    s = copy.deepcopy(base)
    s["masters"] = named(s["masters"], **kw)
    return s


def with_layouts(base, **kw):
    s = copy.deepcopy(base)
    s["layouts"] = named(s["layouts"], **kw)
    return s


def with_slides(base, fn):
    s = copy.deepcopy(base)
    fn(s["slides"])
    return s


AFTER_INSERT_MASTER = with_masters(BASE, added=[MASTER2])
AFTER_INSERT_LAYOUT = with_layouts(BASE, added=[LAYOUT2])

SET_SNAPSHOT_TARGET = with_slides(BASE, lambda s: s[0]["shapes"][0]["blocks"].__setitem__(0, para("Hallo Folie")))

CASES = [
    ("no-mutation", BASE, {"mutation": "noMutation"}, BASE),
    ("set-snapshot", BASE, {"mutation": "setSnapshot", "snapshot": SET_SNAPSHOT_TARGET}, SET_SNAPSHOT_TARGET),
    ("insert-slide", BASE, {"mutation": "insertSlide", "index": 1, "slide": SLIDE2}, with_slides(BASE, lambda s: s.insert(1, copy.deepcopy(SLIDE2)))),
    ("remove-slide", BASE, {"mutation": "removeSlide", "index": 0}, with_slides(BASE, lambda s: s.pop(0))),
    ("set-slide-layout", BASE, {"mutation": "setSlideLayout", "index": 0, "layout_id": None}, with_slides(BASE, lambda s: s[0].__setitem__("layoutId", None))),
    (
        "set-slide-notes",
        BASE,
        {"mutation": "setSlideNotes", "index": 0, "notes": [para("Fünf Minuten Puffer")]},
        with_slides(BASE, lambda s: s[0].__setitem__("notes", [para("Fünf Minuten Puffer")])),
    ),
    (
        "insert-shape",
        BASE,
        {"mutation": "insertShape", "slide_index": 0, "shape_index": 1, "shape": AGENDA},
        with_slides(BASE, lambda s: s[0]["shapes"].insert(1, copy.deepcopy(AGENDA))),
    ),
    ("remove-shape", BASE, {"mutation": "removeShape", "slide_index": 0, "shape_index": 1}, with_slides(BASE, lambda s: s[0]["shapes"].pop(1))),
    (
        "set-shape-frame",
        BASE,
        {"mutation": "setShapeFrame", "slide_index": 0, "shape_index": 0, "frame": frame(4.0, 8.0, 60.0, 12.0)},
        with_slides(BASE, lambda s: s[0]["shapes"][0].__setitem__("frame", frame(4.0, 8.0, 60.0, 12.0))),
    ),
    (
        "set-textbox-blocks",
        BASE,
        {"mutation": "setTextBoxBlocks", "slide_index": 0, "shape_index": 0, "blocks": [para("Hallo Folie"), para("Zweiter Absatz")]},
        with_slides(BASE, lambda s: s[0]["shapes"][0].__setitem__("blocks", [para("Hallo Folie"), para("Zweiter Absatz")])),
    ),
    ("insert-master", BASE, {"mutation": "insertMaster", "master": MASTER2}, AFTER_INSERT_MASTER),
    ("remove-master", AFTER_INSERT_MASTER, {"mutation": "removeMaster", "id": "m-2"}, BASE),
    ("insert-layout", BASE, {"mutation": "insertLayout", "layout": LAYOUT2}, AFTER_INSERT_LAYOUT),
    ("remove-layout", AFTER_INSERT_LAYOUT, {"mutation": "removeLayout", "id": "l-2"}, BASE),
    (
        "set-layout-master",
        AFTER_INSERT_MASTER,
        {"mutation": "setLayoutMaster", "id": "layout1", "master_id": "m-2"},
        with_layouts(AFTER_INSERT_MASTER, modified=[("layout1", {"masterId": "m-2"})]),
    ),
]


def write(path, value):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as handle:
        json.dump(value, handle, ensure_ascii=False, indent=2)
        handle.write("\n")


for kind, before, mutation, after in CASES:
    write(os.path.join(OUT, kind, "⬅️before.json"), before)
    write(os.path.join(OUT, kind, "🦠️mutation.json"), mutation)
    write(os.path.join(OUT, kind, "➡️after.json"), after)

print(f"presentation: {len(CASES)} kinds, {len(CASES) * 3} files -> {OUT}")
