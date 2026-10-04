"""Rewrite the hand-authored crowd-client fixture for design §20: gates, answer figures, score figures, shares.

Keeps the quiz, the sheet, the others' drafts, the places and the draft vectors; replaces the live/submitted/choices
vectors (the crowd lines they described are gone) by the vectors of the figures. It writes the document on one line
to the file named as its argument (a scratch file outside the repository, named `<scratch>.json`); prettier then lays
it out with the repository's configuration, run from the scratch directory (a file outside its working directory is
printed unchanged), and the result replaces the fixture by a rename — a running test watcher of another session may
hold the fixture open, which blocks writing into it but not renaming over it:

    .venv/Scripts/python.exe <this file> <scratch>.json
    (cd <scratch directory> && bun x prettier --config <repository>/.prettierrc.json --write <scratch>.json)
    cp <scratch>.json "<fixture>.new" && mv -f "<fixture>.new" "<fixture>"

with <fixture> = `🧰️framework/🛍️products/❓️quiz/🧫️fixtures/💭️crowd-client/🔣️.json`.
"""
import json
import pathlib
import sys

root = pathlib.Path(__file__).resolve().parents[7]
path = root / "🧰️framework" / "🛍️products" / "❓️quiz" / "🧫️fixtures" / "💭️crowd-client" / "🔣️.json"
old = json.loads(path.read_text(encoding="utf-8"))

A, B, C = "0000000a", "0000000b", "0000000c"


def cell(key, count=0, share=0, tags=(), own=False, correct=False):
    return {"key": key, "count": count, "share": share, "tags": list(tags), "own": own, "correct": correct}


def row(item, answers, cells, mean=None):
    return {"item": item, "answers": answers, "cells": cells, **({} if mean is None else {"mean": mean})}


def figure(identifier, task, expected, dimension=None, crowd=True, thinking=(), answer=False, result=False):
    return {"id": identifier, "task": task, "dimension": dimension, "crowd": crowd, "thinking": list(thinking), "answer": answer, "result": result, "expected": expected}


view = {
    "quiz": "household",
    "runs": 4,
    "scores": [0, 0, 0, 1, 0, 1, 0, 0, 1, 1],
    "tasks": [
        {
            "task": "heat",
            "kind": "classification",
            "scores": [0, 0, 0, 1, 0, 0, 1, 0, 0, 2],
            "items": [
                {"item": "kettle", "answers": 4, "counts": [{"key": "energy", "count": 1}, {"key": "power", "count": 3}]},
                {"item": "candle", "answers": 1, "counts": [{"key": "power", "count": 1}]},
            ],
        },
        {
            "task": "masses",
            "kind": "sorting",
            "scores": [1, 0, 0, 0, 0, 1, 0, 0, 0, 2],
            "items": [
                {"item": "mouse", "answers": 4, "meanPosition": 0.25, "places": [2, 2, 0]},
                {"item": "cat", "answers": 4, "meanPosition": 0.375, "places": [2, 1, 1]},
                {"item": "horse", "answers": 4, "meanPosition": 0.875, "places": [0, 1, 3]},
            ],
        },
        {
            "task": "lamps",
            "kind": "matching",
            "dimension": "power",
            "scores": [0, 0, 0, 2, 0, 0, 0, 0, 0, 2],
            "items": [
                {"item": "led", "answers": 4, "counts": [{"key": "120", "count": 1}, {"key": "2000", "count": 1}, {"key": "8", "count": 2}]},
                {"item": "halogen", "answers": 3, "counts": [{"key": "120", "count": 2}, {"key": "45", "count": 1}]},
            ],
        },
    ],
}

heat = ["power", "energy"]
places = ["0", "1", "2"]
watts = ["8", "45", "120", "2000"]
figures = [
    figure(
        "classification-submitted-runs",
        "heat",
        {
            "runs": 4,
            "thinkers": 0,
            "columns": heat,
            "rows": [
                row("kettle", 4, [cell("power", 3, 0.75), cell("energy", 1, 0.25)]),
                row("fridge", 0, [cell("power"), cell("energy")]),
                row("candle", 1, [cell("power", 1, 1), cell("energy")]),
            ],
        },
    ),
    figure(
        "classification-thinkers-and-the-learners-own-answer",
        "heat",
        {
            "runs": 4,
            "thinkers": 3,
            "columns": heat,
            "rows": [
                row("kettle", 4, [cell("power", 3, 0.75, [A, B], own=True), cell("energy", 1, 0.25, [C])]),
                row("fridge", 0, [cell("power", tags=[B], own=True), cell("energy", tags=[A])]),
                row("candle", 1, [cell("power", 1, 1, [A], own=True), cell("energy")]),
            ],
        },
        thinking=[0, 1, 2],
        answer=True,
    ),
    figure(
        "classification-result-marks-the-correct-category",
        "heat",
        {
            "runs": 4,
            "thinkers": 0,
            "columns": heat,
            "rows": [
                row("kettle", 4, [cell("power", 3, 0.75, own=True, correct=True), cell("energy", 1, 0.25)]),
                row("fridge", 0, [cell("power", own=True), cell("energy", correct=True)]),
                row("candle", 1, [cell("power", 1, 1, own=True, correct=True), cell("energy")]),
            ],
        },
        answer=True,
        result=True,
    ),
    figure(
        "classification-thinkers-before-any-run-one-per-tag",
        "heat",
        {
            "runs": 0,
            "thinkers": 2,
            "columns": heat,
            "rows": [
                row("kettle", 0, [cell("power", tags=[A]), cell("energy", tags=[C])]),
                row("fridge", 0, [cell("power"), cell("energy", tags=[A])]),
                row("candle", 0, [cell("power", tags=[A]), cell("energy")]),
            ],
        },
        crowd=False,
        thinking=[0, 0, 2],
    ),
    figure(
        "sorting-in-the-learners-own-order-with-places-and-thinkers",
        "masses",
        {
            "runs": 4,
            "thinkers": 2,
            "columns": places,
            "rows": [
                row("mouse", 4, [cell("0", 2, 0.5, [A], own=True), cell("1", 2, 0.5, [B]), cell("2")], 1.5),
                row("cat", 4, [cell("0", 2, 0.5, [B]), cell("1", 1, 0.25, [A], own=True), cell("2", 1, 0.25)], 1.8),
                row("horse", 4, [cell("0"), cell("1", 1, 0.25), cell("2", 3, 0.75, [A, B], own=True)], 2.8),
            ],
        },
        thinking=[0, 1],
        answer=True,
    ),
    figure(
        "sorting-in-sheet-order-without-an-answer",
        "masses",
        {
            "runs": 4,
            "thinkers": 0,
            "columns": places,
            "rows": [
                row("horse", 4, [cell("0"), cell("1", 1, 0.25), cell("2", 3, 0.75)], 2.8),
                row("mouse", 4, [cell("0", 2, 0.5), cell("1", 2, 0.5), cell("2")], 1.5),
                row("cat", 4, [cell("0", 2, 0.5), cell("1", 1, 0.25), cell("2", 1, 0.25)], 1.8),
            ],
        },
    ),
    figure(
        "sorting-result-marks-the-correct-place",
        "masses",
        {
            "runs": 4,
            "thinkers": 0,
            "columns": places,
            "rows": [
                row("mouse", 4, [cell("0", 2, 0.5, own=True, correct=True), cell("1", 2, 0.5), cell("2")], 1.5),
                row("cat", 4, [cell("0", 2, 0.5), cell("1", 1, 0.25, own=True, correct=True), cell("2", 1, 0.25)], 1.8),
                row("horse", 4, [cell("0"), cell("1", 1, 0.25), cell("2", 3, 0.75, own=True, correct=True)], 2.8),
            ],
        },
        answer=True,
        result=True,
    ),
    figure(
        "sorting-before-the-crowd-is-known",
        "masses",
        {
            "runs": 0,
            "thinkers": 0,
            "columns": places,
            "rows": [row(item, 0, [cell(place) for place in places]) for item in ["horse", "mouse", "cat"]],
        },
        crowd=False,
    ),
    figure(
        "matching-values-ascending-with-a-value-of-another-sheet",
        "lamps",
        {
            "runs": 4,
            "thinkers": 3,
            "columns": watts,
            "rows": [
                row("led", 4, [cell("8", 2, 0.5, [A], own=True), cell("45"), cell("120", 1, 0.25, [B]), cell("2000", 1, 0.25, [C])]),
                row("halogen", 3, [cell("8"), cell("45", 1, 1 / 3), cell("120", 2, 2 / 3, [A, B], own=True), cell("2000")]),
                row("floodlight", 0, [cell("8"), cell("45"), cell("120"), cell("2000", tags=[A], own=True)]),
            ],
        },
        dimension="power",
        thinking=[0, 1, 2],
        answer=True,
    ),
    figure(
        "matching-result-marks-the-correct-value",
        "lamps",
        {
            "runs": 4,
            "thinkers": 0,
            "columns": watts,
            "rows": [
                row("led", 4, [cell("8", 2, 0.5, own=True, correct=True), cell("45"), cell("120", 1, 0.25), cell("2000", 1, 0.25)]),
                row("halogen", 3, [cell("8"), cell("45", 1, 1 / 3), cell("120", 2, 2 / 3, own=True, correct=True), cell("2000")]),
                row("floodlight", 0, [cell("8"), cell("45"), cell("120"), cell("2000", own=True, correct=True)]),
            ],
        },
        dimension="power",
        answer=True,
        result=True,
    ),
]


def gate(others, place, asked, submitted):
    if others == "never":
        return "off"
    if others == "always" or place == "results" or (place == "quiz" and submitted):
        return "open"
    return "asked" if asked else "locked"


gates = [
    {"others": others, "place": place, "asked": asked, "submitted": submitted, "expected": gate(others, place, asked, submitted)}
    for others in ["never", "submitted", "always"]
    for place in ["run", "quiz", "results"]
    for asked in [False, True]
    for submitted in [False, True]
]

document = {
    "schema": "semio.quiz.crowd-client/v2",
    "description": (
        "What everyone answered, as the web client shows it (design §20). WHEN: `gates` is the whole truth table of the gate — the learner's choice"
        " of when the others show (never, once submitted, always), the place (a run, the page of a quiz, the results of a run), whether the learner"
        " asked to see them now and whether the learner has submitted a run of the quiz: `off` never shows, `open` shows, `asked` shows because of"
        " the ask, `locked` does not show yet and offers the ask. HOW: `figures` are the answers of one sheet task as a figure — the rows are the"
        " items of the viewer's sheet (a sorting in the viewer's own order once there is one, by own position once there is a result), the columns"
        " the categories in sheet order, the places 0 … m − 1 of a sorting, or every value on the sheet's cards, in the submitted counts or in a"
        " draft, ascending numerically (a crowd view's keys arrive in code point order, `120` before `8`); a cell holds the count of the submitted"
        " runs under that key, its share of the item's answers (0 while nobody answered), the tags of the others thinking it right now (one draft"
        " per tag, a sorting draft at the place its normalized position rounds half up to), and whether it is the viewer's own answer or — with a"
        " result — the correct one; a sorting row carries the place `1 + meanPosition × (m − 1)` to one decimal. `vector.crowd` says whether the"
        " submitted `view` is known, `thinking` which of `others` think along, `answer` and `result` whether the viewer's answer and its scored"
        " result are given. `scores` are the score figures: the ten bins (none known: ten zeros), how many runs they hold, the tallest bin and the"
        " bin of the viewer's own score. `shares` is how a share reads in each language: whole percent, but never nothing for something nor"
        " everything for less. `livePlaces` is the place a normalized position falls on, `places` the place a mean position reads as. A draft is"
        " the learner's own answer in the form peers read: classification and sorting unchanged, matching card indices replaced by the card"
        " values; an answer that does not fit its sheet task is no draft."
    ),
    "quiz": old["quiz"],
    "sheet": old["sheet"],
    "others": old["others"],
    "view": view,
    "gates": gates,
    "figures": figures,
    "scores": [
        {"id": "none-before-the-crowd-is-known", "bins": None, "own": None, "expected": {"bins": [0] * 10, "runs": 0, "peak": 0, "own": None}},
        {"id": "own-score-in-a-middle-bin", "bins": [0, 0, 0, 1, 0, 1, 0, 0, 1, 1], "own": 0.5555, "expected": {"bins": [0, 0, 0, 1, 0, 1, 0, 0, 1, 1], "runs": 4, "peak": 1, "own": 5}},
        {"id": "a-perfect-score-in-the-last-bin", "bins": [1, 0, 0, 0, 0, 2, 0, 0, 0, 5], "own": 1, "expected": {"bins": [1, 0, 0, 0, 0, 2, 0, 0, 0, 5], "runs": 8, "peak": 5, "own": 9}},
        {"id": "a-low-score-in-the-first-bin", "bins": [3, 0, 0, 0, 0, 0, 0, 0, 0, 0], "own": 0.04, "expected": {"bins": [3, 0, 0, 0, 0, 0, 0, 0, 0, 0], "runs": 3, "peak": 3, "own": 0}},
        {"id": "no-own-score", "bins": [0, 2, 0, 0, 0, 0, 0, 7, 0, 0], "own": None, "expected": {"bins": [0, 2, 0, 0, 0, 0, 0, 7, 0, 0], "runs": 9, "peak": 7, "own": None}},
    ],
    "shares": [
        {"share": 0, "en": "0%", "de": "0 %"},
        {"share": 0.004, "en": "<1%", "de": "<1 %"},
        {"share": 0.25, "en": "25%", "de": "25 %"},
        {"share": 1 / 3, "en": "33%", "de": "33 %"},
        {"share": 0.75, "en": "75%", "de": "75 %"},
        {"share": 0.996, "en": ">99%", "de": ">99 %"},
        {"share": 1, "en": "100%", "de": "100 %"},
    ],
    "livePlaces": [
        {"position": 0, "places": 3, "place": 0},
        {"position": 0.24, "places": 3, "place": 0},
        {"position": 0.25, "places": 3, "place": 1},
        {"position": 0.5, "places": 3, "place": 1},
        {"position": 1, "places": 3, "place": 2},
        {"position": 0.6, "places": 6, "place": 3},
        {"position": 1, "places": 1, "place": 0},
        {"position": 0.5, "places": 0, "place": 0},
    ],
    "places": old["places"],
    "drafts": old["drafts"],
}

pathlib.Path(sys.argv[1]).write_text(json.dumps(document, ensure_ascii=False).replace(" ", "\\u00a0") + "\n", encoding="utf-8", newline="\n")
print(len(gates), "gates,", len(figures), "figures")
