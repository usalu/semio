#!/usr/bin/env python3
"""🧫️ Generates the pets schema-conformance vectors from the Python oracle adapter of the case.

Run from the repository root: ``.venv/Scripts/python.exe <this file>`` (``.venv/bin/python`` elsewhere).
The documents are composed here: the sample menagerie (a walker, a hopper and a floater with bonds and the
casts of two scenes), its ensemble, minimal bases and one broken copy of a base per rule. Every expectation is
computed by ``🧰️framework/🛍️products/🐾️pets/🧪️tests/🧬️schema-conformance/🐍️.py`` (python-jsonschema on the
normative schema, the rule reference beside it), and the expectation written next to each vector below must
equal what that code finds, so the committed file, the oracle and the intention can never disagree.
"""

import copy
import importlib.util
import json
import os
import sys
import time

sys.dont_write_bytecode = True
sys.stdout.reconfigure(encoding="utf-8")
ROOT =os.path.abspath(os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "..", "..", "..", "..", "..", ".."))
PETS = os.path.join(ROOT, "🧰️framework", "🛍️products", "🐾️pets")
HOST = os.path.join(ROOT, "🧰️framework", "🛍️products", "🦑️repo", "🔨️modules", "🧪️test", "🖥️host", "🐍️.py")
CASE_DIRECTORY = "🧬️schema-conformance"
DOCUMENT = "🔣️.json"
MENAGERIE_SCHEMA = "semio.pets.menagerie/v1"
ENSEMBLE_SCHEMA = "semio.pets.ensemble/v1"
SOFT = [0.4, 0, 0.6, 1]


def load(path, name):
    """📦️ Loads one Python file as a module."""
    spec = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


sys.modules["semio_repo_test"] = load(HOST, "semio_repo_test")
CASE = load(os.path.join(PETS, "🧪️tests", CASE_DIRECTORY, "🐍️.py"), "schema_conformance")


def write(document):
    """💾️ Writes the fixture as indented UTF-8 JSON with a trailing newline."""
    path = os.path.join(PETS, "🧫️fixtures", CASE_DIRECTORY, "🔣️.json")
    os.makedirs(os.path.dirname(path), exist_ok=True)
    payload = (json.dumps(document, ensure_ascii=False, indent=2) + "\n").encode("utf-8")
    if os.path.exists(path) and open(path, "rb").read() == payload:
        print("unchanged %s" % os.path.relpath(path, ROOT))
        return
    for _ in range(20):
        try:
            with open(path, "wb") as handle:
                handle.write(payload)
            break
        except OSError as error:
            print("retrying %s after %s" % (os.path.relpath(path, ROOT), error))
            time.sleep(0.5)
    else:
        raise OSError("cannot write %s" % path)
    print("wrote %s (%d bytes)" % (os.path.relpath(path, ROOT), len(payload)))


# region 🔖️Builders
def T(en, de):
    """🌍️ A text in both languages."""
    return {"en": en, "de": de}


def bone(identifier, x, y, parent=None, rotation=None):
    """🦴️ A bone."""
    entry = {"id": identifier}
    if parent is not None:
        entry["parent"] = parent
    entry.update({"x": x, "y": y})
    if rotation is not None:
        entry["rotation"] = rotation
    return entry


def ellipse(cx, cy, rx, ry):
    """⭕️ An ellipse shape."""
    return {"kind": "ellipse", "cx": cx, "cy": cy, "rx": rx, "ry": ry}


def line(x1, y1, x2, y2):
    """➖️ A line shape."""
    return {"kind": "line", "x1": x1, "y1": y1, "x2": x2, "y2": y2}


def rect(x, y, width, height, radius=None):
    """▭️ A rectangle shape."""
    shape = {"kind": "rect", "x": x, "y": y, "width": width, "height": height}
    if radius is not None:
        shape["radius"] = radius
    return shape


def outline(d):
    """➰️ A path shape."""
    return {"kind": "path", "d": d}


def part(identifier, on, shape, fill, stroke="ink", width=None):
    """🧩️ A part on a bone."""
    entry = {"id": identifier, "bone": on, "shape": shape, "fill": fill, "stroke": stroke}
    if width is not None:
        entry["strokeWidth"] = width
    return entry


def eye(identifier, on, x, y, radius, pupil):
    """👁️ An eye on a bone."""
    return {"id": identifier, "bone": on, "x": x, "y": y, "radius": radius, "pupil": pupil}


def track(on, channel, *keys, ease=None):
    """🛤️ A track; a key is ``(at, value)`` or ``(at, value, ease)``, ``ease`` shapes every key but the last."""
    entries = []
    for index, key in enumerate(keys):
        entry = {"at": key[0], "value": key[1]}
        shaped = key[2] if len(key) > 2 else ease if index < len(keys) - 1 else None
        if shaped is not None:
            entry["ease"] = list(shaped)
        entries.append(entry)
    return {"bone": on, "channel": channel, "keys": entries}


def clip(identifier, seconds, loop, *tracks):
    """🎞️ A clip."""
    return {"id": identifier, "seconds": seconds, "loop": loop, "tracks": list(tracks)}


# endregion 🔖️Builders


# region 🔖️Sample
def blobby():
    """🫧️ The walker: a blob on two legs (the reference rig of the ticket, with a full repertoire)."""
    return {
        "id": "blobby",
        "name": T("Blobby, the blob", "Blobby, der Klecks"),
        "thing": T("blob", "Klecks"),
        "grounds": ["sample menagerie of the pets product: the walker"],
        "size": {"width": 40, "height": 30},
        "palette": {"body": "#1e9b8d", "accent": "#34d1bf", "detail": "#fa9500"},
        "bones": [
            bone("root", 0, 0),
            bone("body", 0, -16, "root"),
            bone("leg-left", -6, 9, "body"),
            bone("leg-right", 6, 9, "body"),
            bone("arm-left", -13, 0, "body", 20),
            bone("arm-right", 13, 0, "body", -20),
            bone("tuft", 0, -13, "body"),
        ],
        "parts": [
            part("leg-left", "leg-left", line(0, 0, 0, 6), "none", "ink", 3),
            part("leg-right", "leg-right", line(0, 0, 0, 6), "none", "ink", 3),
            part("arm-left", "arm-left", line(0, 0, -6, 0), "none", "ink", 3),
            part("arm-right", "arm-right", line(0, 0, 6, 0), "none", "ink", 3),
            part("tuft", "tuft", outline("M -3 0 Q -2 -6 0 -2 Q 2 -7 3 0 Z"), "detail", "ink", 1.5),
            part("body", "body", ellipse(0, 0, 14, 12), "body"),
            part("belly", "body", ellipse(0, 5, 7, 4), "accent", "none"),
        ],
        "face": {"eyes": [eye("eye-left", "body", -5, -3, 3.5, 1.6), eye("eye-right", "body", 5, -3, 3.5, 1.6)], "mouth": {"bone": "body", "x": 0, "y": 3, "width": 6}},
        "clips": [
            clip("breathe", 3, True, track("body", "scaleY", (0, 1), (0.5, 1.05), (1, 1), ease=SOFT), track("body", "scaleX", (0, 1), (0.5, 0.98), (1, 1), ease=SOFT)),
            clip(
                "stroll",
                0.6,
                True,
                track("leg-left", "rotation", (0, 28), (0.5, -28), (1, 28)),
                track("leg-right", "rotation", (0, -28), (0.5, 28), (1, -28)),
                track("body", "y", (0, 0), (0.25, -1.5), (0.5, 0), (0.75, -1.5), (1, 0)),
                track("arm-left", "rotation", (0, -15), (0.5, 15), (1, -15)),
                track("arm-right", "rotation", (0, -15), (0.5, 15), (1, -15)),
            ),
            clip("wiggle", 1.2, False, track("body", "rotation", (0, 0), (0.2, -9), (0.5, 9), (0.8, -5), (1, 0)), track("tuft", "rotation", (0, 0), (0.3, 20), (0.6, -20), (1, 0))),
            clip(
                "stretch",
                1.6,
                False,
                track("body", "scaleY", (0, 1), (0.4, 1.12), (0.7, 1.12), (1, 1), ease=SOFT),
                track("arm-left", "rotation", (0, 0), (0.4, 60), (0.7, 60), (1, 0), ease=SOFT),
                track("arm-right", "rotation", (0, 0), (0.4, -60), (0.7, -60), (1, 0), ease=SOFT),
            ),
            clip("wave", 1.4, False, track("arm-right", "rotation", (0, 0, [0.3, 0, 0.2, 1]), (0.25, -70), (0.45, -40), (0.65, -70), (0.85, -40, [0.3, 0, 0.2, 1]), (1, 0))),
            clip("nuzzle", 1.6, True, track("body", "rotation", (0, 0), (0.25, 8), (0.75, -8), (1, 0), ease=SOFT), track("body", "y", (0, 0), (0.5, -1), (1, 0))),
            clip(
                "stomp",
                0.5,
                True,
                track("body", "y", (0, 0), (0.3, -3), (0.6, 0), (1, 0)),
                track("arm-left", "rotation", (0, 40), (0.5, 70), (1, 40)),
                track("arm-right", "rotation", (0, -40), (0.5, -70), (1, -40)),
                track("tuft", "rotation", (0, 0), (0.5, 12), (1, 0)),
            ),
            clip(
                "pout",
                2.4,
                True,
                track("body", "scaleY", (0, 0.93), (0.5, 0.95), (1, 0.93), ease=SOFT),
                track("body", "y", (0, 1.2), (1, 1.2)),
                track("tuft", "rotation", (0, 35), (1, 35)),
                track("arm-left", "rotation", (0, -25), (1, -25)),
                track("arm-right", "rotation", (0, 25), (1, 25)),
            ),
            clip("doze", 5, True, track("body", "scaleY", (0, 0.94), (0.5, 0.99), (1, 0.94), ease=SOFT), track("body", "y", (0, 1.5), (1, 1.5))),
            clip("squash", 0.3, False, track("body", "scaleY", (0, 0.8), (0.5, 1.06), (1, 1)), track("body", "scaleX", (0, 1.15), (0.5, 0.97), (1, 1)), track("body", "y", (0, 3), (0.5, -0.9), (1, 0))),
            clip(
                "leap",
                0.5,
                False,
                track("leg-left", "rotation", (0, 0), (0.3, 35), (0.8, 35), (1, 0)),
                track("leg-right", "rotation", (0, 0), (0.3, -35), (0.8, -35), (1, 0)),
                track("arm-left", "rotation", (0, 0), (0.3, 70), (0.8, 70), (1, 0)),
                track("arm-right", "rotation", (0, 0), (0.3, -70), (0.8, -70), (1, 0)),
            ),
            clip("flail", 0.4, True, track("arm-left", "rotation", (0, 50), (0.5, 90), (1, 50)), track("arm-right", "rotation", (0, -90), (0.5, -50), (1, -90)), track("tuft", "rotation", (0, -10), (0.5, 10), (1, -10))),
        ],
        "repertoire": {
            "idle": ["breathe"],
            "fidget": ["wiggle", "stretch"],
            "walk": ["stroll"],
            "hop": ["leap"],
            "fall": ["flail"],
            "land": ["squash"],
            "sleep": ["doze"],
            "greet": ["wave"],
            "cuddle": ["nuzzle"],
            "squabble": ["stomp"],
            "sulk": ["pout"],
        },
        "locomotion": {"gait": "walk", "speed": 36},
        "temperament": {"energy": 0.6, "sociability": 0.7, "curiosity": 0.6},
    }


def hoppy():
    """🐇️ The hopper: a round body on two big feet with long ears."""
    return {
        "id": "hoppy",
        "name": T("Hoppy, the spring", "Hoppy, die Sprungfeder"),
        "thing": T("spring", "Sprungfeder"),
        "grounds": ["sample menagerie of the pets product: the hopper"],
        "size": {"width": 30, "height": 36},
        "palette": {"body": "#b87f46", "accent": "#dbbea1", "detail": "#a3472f"},
        "bones": [
            bone("root", 0, 0),
            bone("body", 0, -13, "root"),
            bone("foot-left", -6, 8.5, "body"),
            bone("foot-right", 6, 8.5, "body"),
            bone("ear-left", -5, -9, "body", -12),
            bone("ear-right", 5, -9, "body", 12),
            bone("tail", -11.5, 4, "body"),
        ],
        "parts": [
            part("tail", "tail", ellipse(0, 0, 3, 3), "paper", "ink", 1.5),
            part("ear-left", "ear-left", ellipse(0, -6, 2.6, 6.5), "accent"),
            part("ear-right", "ear-right", ellipse(0, -6, 2.6, 6.5), "accent"),
            part("foot-left", "foot-left", ellipse(1, 2, 5, 2.5), "detail"),
            part("foot-right", "foot-right", ellipse(1, 2, 5, 2.5), "detail"),
            part("body", "body", ellipse(0, 0, 12, 11), "body"),
            part("belly", "body", ellipse(0, 4, 6.5, 5), "accent", "none"),
        ],
        "face": {"eyes": [eye("eye-left", "body", -4, -2.5, 3.2, 1.5), eye("eye-right", "body", 4, -2.5, 3.2, 1.5)], "mouth": {"bone": "body", "x": 0, "y": 3, "width": 5}},
        "clips": [
            clip(
                "breathe",
                2.8,
                True,
                track("body", "scaleY", (0, 1), (0.5, 1.06), (1, 1), ease=SOFT),
                track("ear-left", "rotation", (0, 0), (0.5, -5), (1, 0), ease=SOFT),
                track("ear-right", "rotation", (0, 0), (0.5, 5), (1, 0), ease=SOFT),
            ),
            clip(
                "bounce",
                0.5,
                True,
                track("body", "y", (0, 0, [0.2, 0.7, 0.5, 1]), (0.5, -9, [0.5, 0, 0.8, 0.3]), (1, 0)),
                track("body", "scaleY", (0, 0.86), (0.2, 1.1), (0.5, 1), (0.8, 1.08), (1, 0.86)),
                track("foot-left", "rotation", (0, 0), (0.5, 28), (1, 0)),
                track("foot-right", "rotation", (0, 0), (0.5, 28), (1, 0)),
                track("ear-left", "rotation", (0, 0), (0.5, -14), (1, 0)),
                track("ear-right", "rotation", (0, 0), (0.5, 14), (1, 0)),
            ),
            clip("ear-twitch", 0.8, False, track("ear-left", "rotation", (0, 0), (0.2, -26), (0.45, 6), (0.7, -16), (1, 0))),
            clip("thump", 1, False, track("foot-right", "rotation", (0, 0), (0.2, -32), (0.4, 0), (0.6, -32), (0.8, 0), (1, 0)), track("body", "rotation", (0, 0), (0.5, -4), (1, 0))),
            clip(
                "perk",
                1.2,
                False,
                track("ear-left", "rotation", (0, 0), (0.3, -30), (0.8, -30), (1, 0), ease=SOFT),
                track("ear-right", "rotation", (0, 0), (0.3, 30), (0.8, 30), (1, 0), ease=SOFT),
                track("body", "y", (0, 0), (0.3, -4), (0.5, 0), (1, 0)),
            ),
            clip("snuggle", 1.6, True, track("body", "rotation", (0, 0), (0.25, -7), (0.75, 7), (1, 0), ease=SOFT), track("ear-left", "rotation", (0, -25), (1, -25)), track("ear-right", "rotation", (0, 25), (1, 25))),
            clip(
                "stamp",
                0.4,
                True,
                track("body", "x", (0, 0), (0.25, -1.5), (0.75, 1.5), (1, 0)),
                track("ear-left", "rotation", (0, 10), (1, 10)),
                track("ear-right", "rotation", (0, -10), (1, -10)),
                track("foot-left", "rotation", (0, 0), (0.5, -25), (1, 0)),
            ),
            clip(
                "droop",
                2.4,
                True,
                track("ear-left", "rotation", (0, -55), (0.5, -50), (1, -55), ease=SOFT),
                track("ear-right", "rotation", (0, 55), (0.5, 50), (1, 55), ease=SOFT),
                track("body", "scaleY", (0, 0.95), (1, 0.95)),
                track("body", "y", (0, 0.6), (1, 0.6)),
            ),
            clip(
                "nap",
                4.5,
                True,
                track("body", "scaleY", (0, 0.92), (0.5, 0.97), (1, 0.92), ease=SOFT),
                track("body", "y", (0, 1), (1, 1)),
                track("ear-left", "rotation", (0, -65), (1, -65)),
                track("ear-right", "rotation", (0, 65), (1, 65)),
            ),
            clip("plop", 0.3, False, track("body", "scaleY", (0, 0.78), (0.5, 1.06), (1, 1)), track("body", "scaleX", (0, 1.18), (0.5, 0.97), (1, 1)), track("body", "y", (0, 2.6), (0.5, -0.7), (1, 0))),
        ],
        "repertoire": {
            "idle": ["breathe"],
            "fidget": ["ear-twitch", "thump"],
            "walk": ["bounce"],
            "hop": ["bounce"],
            "land": ["plop"],
            "sleep": ["nap"],
            "greet": ["perk"],
            "cuddle": ["snuggle"],
            "squabble": ["stamp"],
            "sulk": ["droop"],
        },
        "locomotion": {"gait": "hop", "speed": 44},
        "temperament": {"energy": 0.9, "sociability": 0.5, "curiosity": 0.8},
    }


def floaty():
    """🎈️ The floater: a small balloon with two puffs and a wisp that carries a tag."""
    return {
        "id": "floaty",
        "name": T("Floaty, the balloon", "Floaty, der Ballon"),
        "thing": T("balloon", "Ballon"),
        "grounds": ["sample menagerie of the pets product: the floater"],
        "size": {"width": 34, "height": 30},
        "palette": {"body": "#bfe6ea", "accent": "#8282dd", "detail": "#a64a63"},
        "bones": [bone("root", 0, 0), bone("body", 0, -20, "root"), bone("wisp", 0, 9, "body"), bone("puff-left", -11, 2, "body"), bone("puff-right", 11, 2, "body")],
        "parts": [
            part("wisp", "wisp", outline("M -3 0 Q 0 5 -1 8 Q 2 5 3 0 Z"), "accent", "ink", 1.5),
            part("tag", "wisp", rect(-2, 8, 4, 3, 1), "detail", "ink", 1),
            part("puff-left", "puff-left", ellipse(0, 0, 6, 5), "body"),
            part("puff-right", "puff-right", ellipse(0, 0, 6, 5), "body"),
            part("body", "body", ellipse(0, 0, 12, 10), "body"),
            part("cheek-left", "body", ellipse(-7.5, 3, 2, 1.2), "detail", "none"),
            part("cheek-right", "body", ellipse(7.5, 3, 2, 1.2), "detail", "none"),
        ],
        "face": {"eyes": [eye("eye-left", "body", -4.5, -2, 3.4, 1.5), eye("eye-right", "body", 4.5, -2, 3.4, 1.5)], "mouth": {"bone": "body", "x": 0, "y": 3.5, "width": 5}, "above": "body"},
        "clips": [
            clip(
                "bob",
                3.2,
                True,
                track("body", "y", (0, 0), (0.5, -3), (1, 0), ease=SOFT),
                track("wisp", "rotation", (0, 0), (0.25, 8), (0.75, -8), (1, 0), ease=SOFT),
                track("puff-left", "scaleX", (0, 1), (0.5, 1.06), (1, 1)),
                track("puff-right", "scaleX", (0, 1), (0.5, 1.06), (1, 1)),
            ),
            clip("twirl", 1.2, False, track("body", "rotation", (0, 0, [0.4, 0, 0.2, 1]), (1, 360))),
            clip(
                "puff-up",
                1.4,
                False,
                track("puff-left", "scaleX", (0, 1), (0.4, 1.3), (1, 1), ease=SOFT),
                track("puff-left", "scaleY", (0, 1), (0.4, 1.3), (1, 1), ease=SOFT),
                track("puff-right", "scaleX", (0, 1), (0.4, 1.3), (1, 1), ease=SOFT),
                track("puff-right", "scaleY", (0, 1), (0.4, 1.3), (1, 1), ease=SOFT),
                track("body", "scaleX", (0, 1), (0.4, 1.08), (1, 1), ease=SOFT),
            ),
            clip("dip", 1.2, False, track("body", "y", (0, 0), (0.3, 4), (0.6, -2), (1, 0), ease=SOFT), track("body", "rotation", (0, 0), (0.3, 10), (0.6, -4), (1, 0), ease=SOFT)),
            clip(
                "glow",
                1.8,
                True,
                track("body", "scaleX", (0, 1), (0.5, 1.06), (1, 1), ease=SOFT),
                track("body", "scaleY", (0, 1), (0.5, 1.06), (1, 1), ease=SOFT),
                track("puff-left", "rotation", (0, 0), (0.5, -10), (1, 0)),
                track("puff-right", "rotation", (0, 0), (0.5, 10), (1, 0)),
            ),
            clip("whirl", 0.8, True, track("wisp", "rotation", (0, 0), (1, 360)), track("body", "x", (0, 0), (0.25, -1.5), (0.75, 1.5), (1, 0))),
            clip("sag", 2.6, True, track("body", "y", (0, 3), (0.5, 3.6), (1, 3), ease=SOFT), track("body", "scaleY", (0, 0.94), (1, 0.94)), track("wisp", "rotation", (0, 14), (1, 14))),
            clip("drift", 5, True, track("body", "y", (0, 2), (0.5, 4), (1, 2), ease=SOFT), track("body", "scaleY", (0, 0.95), (0.5, 0.98), (1, 0.95), ease=SOFT)),
            clip("settle", 0.3, False, track("body", "y", (0, 3), (0.5, -1), (1, 0)), track("body", "scaleY", (0, 0.9), (0.5, 1.04), (1, 1))),
        ],
        "repertoire": {"idle": ["bob"], "fidget": ["twirl", "puff-up"], "walk": ["bob"], "land": ["settle"], "sleep": ["drift"], "greet": ["dip"], "cuddle": ["glow"], "squabble": ["whirl"], "sulk": ["sag"]},
        "locomotion": {"gait": "float", "speed": 28, "hover": 14},
        "temperament": {"energy": 0.3, "sociability": 0.6, "curiosity": 0.4},
    }


def sample():
    """🎪️ The sample ensemble, its species documents in ensemble order, and the menagerie they assemble to."""
    members = [("🫧️blobby/" + DOCUMENT, blobby()), ("🐇️hoppy/" + DOCUMENT, hoppy()), ("🎈️floaty/" + DOCUMENT, floaty())]
    bonds = [{"between": ["blobby", "hoppy"], "affinity": 0.6}, {"between": ["blobby", "floaty"], "affinity": -0.4}, {"between": ["hoppy", "floaty"], "affinity": 0.1}]
    casts = [{"scene": "home", "core": ["blobby", "hoppy"], "rotation": ["floaty"]}, {"scene": "meadow", "core": ["hoppy"], "rotation": ["floaty", "blobby"]}]
    title = T("Sample menagerie", "Beispiel-Menagerie")
    ensemble = {"$schema": "../../🧬️schema/🔣️.json#/$defs/Ensemble", "schema": ENSEMBLE_SCHEMA, "id": "sample", "title": title, "species": [path for path, _ in members], "bonds": bonds, "casts": casts}
    species = [{"path": path, "document": {"$schema": "../../../🧬️schema/🔣️.json#/$defs/Species", **document}} for path, document in members]
    menagerie = {"schema": MENAGERIE_SCHEMA, "id": "sample", "title": title, "species": [document for _, document in members], "bonds": bonds, "casts": casts}
    return ensemble, species, menagerie


# endregion 🔖️Sample


# region 🔖️Bases
def mini(identifier="mini"):
    """🔹️ The smallest complete species: two bones, one part, one eye, a mouth, an idle and a walk clip."""
    return {
        "id": identifier,
        "name": T("Mini", "Mini"),
        "thing": T("dot", "Punkt"),
        "grounds": ["minimal base of the rule vectors"],
        "size": {"width": 16, "height": 16},
        "palette": {"body": "#1e9b8d", "accent": "#34d1bf", "detail": "#fa9500"},
        "bones": [bone("root", 0, 0), bone("body", 0, -8, "root")],
        "parts": [part("body", "body", ellipse(0, 0, 8, 8), "body")],
        "face": {"eyes": [eye("eye", "body", 0, -2, 3, 1.3)], "mouth": {"bone": "body", "x": 0, "y": 3, "width": 4}},
        "clips": [clip("rest", 2, True, track("body", "scaleY", (0, 1), (0.5, 1.05), (1, 1), ease=SOFT)), clip("step", 0.5, True, track("body", "y", (0, 0), (0.5, -2), (1, 0)))],
        "repertoire": {"idle": ["rest"], "walk": ["step"]},
        "locomotion": {"gait": "walk", "speed": 30},
        "temperament": {"energy": 0.5, "sociability": 0.5, "curiosity": 0.5},
    }


def duo():
    """🔸️ What the minimal menagerie and ensemble share: one bond and one cast over two species."""
    return {"bonds": [{"between": ["mini", "midi"], "affinity": 0.5}], "casts": [{"scene": "home", "core": ["mini"], "rotation": ["midi"]}]}


BASES = {
    "species": mini(),
    "menagerie": {"schema": MENAGERIE_SCHEMA, "id": "base", "title": T("Base", "Basis"), "species": [mini("mini"), mini("midi")], **duo()},
    "ensemble": {"schema": ENSEMBLE_SCHEMA, "id": "base", "title": T("Base", "Basis"), "species": ["mini/" + DOCUMENT, "midi/" + DOCUMENT], **duo()},
}
BASE_OF = {"Species": "species", "Menagerie": "menagerie", "Ensemble": "ensemble"}


def edit(definition, *operations):
    """✏️ A copy of the definition's base after ``("set" | "add" | "del", [path…], value)`` operations."""
    document = copy.deepcopy(BASES[BASE_OF[definition]])
    for operation in operations:
        target = document
        for step in operation[1][:-1]:
            target = target[step]
        last = operation[1][-1]
        if operation[0] == "set":
            target[last] = copy.deepcopy(operation[2])
        elif operation[0] == "add":
            target[last].append(copy.deepcopy(operation[2]))
        else:
            del target[last]
    return document


# endregion 🔖️Bases


# region 🔖️Vectors
STEP_KEYS = ["clips", 1, "tracks", 0, "keys"]
STEP_POINTER = "/clips/1/tracks/0/keys"
REST_KEYS = ["clips", 0, "tracks", 0, "keys"]
REST_POINTER = "/clips/0/tracks/0/keys"


def accepted():
    """✅️ Documents that conform and break no rule: the sample, the bases and the edges the rules allow."""
    referred = [
        ("sample-menagerie", "Menagerie", "/menagerie"),
        ("sample-ensemble", "Ensemble", "/ensemble"),
        ("sample-walker", "Species", "/species/0/document"),
        ("sample-hopper", "Species", "/species/1/document"),
        ("sample-floater", "Species", "/species/2/document"),
        ("base-species", "Species", "/bases/species"),
        ("base-menagerie", "Menagerie", "/bases/menagerie"),
        ("base-ensemble", "Ensemble", "/bases/ensemble"),
    ]
    inline = [
        ("full-turn-loop", "Species", edit("Species", ("set", ["clips", 1, "tracks", 0], track("body", "rotation", (0, 0), (1, 360))))),
        ("opposite-angles-loop", "Species", edit("Species", ("set", ["clips", 1, "tracks", 0], track("body", "rotation", (0, -180), (0.5, 0), (1, 180))))),
        ("two-turns-back-loop", "Species", edit("Species", ("set", ["clips", 1, "tracks", 0], track("body", "rotation", (0, 90), (1, -630))))),
        ("decimal-turn-loop", "Species", edit("Species", ("set", ["clips", 1, "tracks", 0], track("body", "rotation", (0, -359.8), (1, -719.8))))),
        ("decimal-two-turns-loop", "Species", edit("Species", ("set", ["clips", 1, "tracks", 0], track("body", "rotation", (0, -359.9), (1, -1079.9))))),
        ("open-ended-clip", "Species", edit("Species", ("set", ["clips", 1], clip("step", 0.5, False, track("body", "y", (0, 0), (1, -2)))))),
        ("overshooting-ease", "Species", edit("Species", ("set", REST_KEYS + [0, "ease"], [0.3, -0.4, 0.2, 1.6]))),
        ("floater-without-walk-clip", "Species", edit("Species", ("set", ["locomotion"], {"gait": "float", "speed": 20, "hover": 6}), ("set", ["repertoire"], {"idle": ["rest"]}))),
        ("face-above-a-part", "Species", edit("Species", ("set", ["face", "above"], "body"))),
        ("species-with-schema-hint", "Species", {"$schema": "../../🧬️schema/🔣️.json#/$defs/Species", **mini()}),
        ("menagerie-with-schema-hint", "Menagerie", {"$schema": "../🧬️schema/🔣️.json", **copy.deepcopy(BASES["menagerie"])}),
        ("empty-menagerie", "Menagerie", {"schema": MENAGERIE_SCHEMA, "id": "empty", "title": T("Empty", "Leer"), "species": [], "bonds": [], "casts": []}),
        ("opposed-bond", "Ensemble", edit("Ensemble", ("set", ["bonds", 0, "affinity"], -1))),
    ]
    vectors = [{"id": identifier, "definition": definition, "pointer": pointer} for identifier, definition, pointer in referred]
    vectors += [{"id": identifier, "definition": definition, "document": document} for identifier, definition, document in inline]
    return vectors


def structural():
    """🚫️ Documents that break the type-level structure, each at one keyword of the schema."""
    rows = [
        ("not-an-object", "Species", [], "type", ""),
        ("missing-size", "Species", edit("Species", ("del", ["size"])), "required", "/size"),
        ("undeclared-property", "Species", edit("Species", ("set", ["mood"], 1)), "additionalProperties", "/mood"),
        ("schema-hint-not-a-string", "Species", edit("Species", ("set", ["$schema"], 7)), "type", "/$schema"),
        ("width-not-a-number", "Species", edit("Species", ("set", ["size", "width"], "wide")), "type", "/size/width"),
        ("missing-language", "Species", edit("Species", ("del", ["name", "de"])), "required", "/name/de"),
        ("undeclared-language", "Species", edit("Species", ("set", ["name", "fr"], "Mini")), "additionalProperties", "/name/fr"),
        ("unknown-paint", "Species", edit("Species", ("set", ["parts", 0, "fill"], "glitter")), "enum", "/parts/0/fill"),
        ("unknown-shape-kind", "Species", edit("Species", ("set", ["parts", 0, "shape", "kind"], "star")), "const", "/parts/0/shape/kind"),
        ("missing-shape-kind", "Species", edit("Species", ("del", ["parts", 0, "shape", "kind"])), "required", "/parts/0/shape/kind"),
        ("radius-not-a-number", "Species", edit("Species", ("set", ["parts", 0, "shape", "rx"], "wide")), "type", "/parts/0/shape/rx"),
        ("outline-on-an-ellipse", "Species", edit("Species", ("set", ["parts", 0, "shape", "d"], "M 0 0 L 4 4")), "additionalProperties", "/parts/0/shape/d"),
        ("eyes-not-a-list", "Species", edit("Species", ("set", ["face", "eyes"], {})), "type", "/face/eyes"),
        ("unknown-channel", "Species", edit("Species", ("set", ["clips", 0, "tracks", 0, "channel"], "skew")), "enum", "/clips/0/tracks/0/channel"),
        ("loop-not-a-boolean", "Species", edit("Species", ("set", ["clips", 0, "loop"], "yes")), "type", "/clips/0/loop"),
        ("key-without-value", "Species", edit("Species", ("del", REST_KEYS + [1, "value"])), "required", REST_POINTER + "/1/value"),
        ("ease-of-three", "Species", edit("Species", ("set", REST_KEYS + [0, "ease"], [0.4, 0, 0.6])), "minItems", REST_POINTER + "/0/ease"),
        ("ease-of-five", "Species", edit("Species", ("set", REST_KEYS + [0, "ease"], [0.4, 0, 0.6, 1, 1])), "maxItems", REST_POINTER + "/0/ease"),
        ("unknown-activity", "Species", edit("Species", ("set", ["repertoire", "dance"], ["rest"])), "additionalProperties", "/repertoire/dance"),
        ("unknown-gait", "Species", edit("Species", ("set", ["locomotion", "gait"], "swim")), "enum", "/locomotion/gait"),
        ("other-schema-version", "Menagerie", edit("Menagerie", ("set", ["schema"], "semio.pets.menagerie/v2")), "const", "/schema"),
        ("species-not-an-object", "Menagerie", edit("Menagerie", ("add", ["species"], "maxi/" + DOCUMENT)), "type", "/species/2"),
        ("path-not-a-string", "Ensemble", edit("Ensemble", ("set", ["species", 0], 5)), "type", "/species/0"),
        ("bond-of-three", "Ensemble", edit("Ensemble", ("set", ["bonds", 0, "between"], ["mini", "midi", "maxi"])), "maxItems", "/bonds/0/between"),
        ("bond-of-one", "Ensemble", edit("Ensemble", ("set", ["bonds", 0, "between"], ["mini"])), "minItems", "/bonds/0/between"),
        ("cast-without-rotation", "Ensemble", edit("Ensemble", ("del", ["casts", 0, "rotation"])), "required", "/casts/0/rotation"),
    ]
    return [{"id": identifier, "definition": definition, "document": document, "violates": {"keyword": keyword, "at": pointer}, "issues": [{"path": pointer, "code": CASE.CODES[keyword]}]} for identifier, definition, document, keyword, pointer in rows]


def rules():
    """⚖️ Structurally sound documents that break a rule: ``(id, definition, document, findings, schema verdict)``."""
    arm = bone("arm", 0, 0, "body")
    rows = [
        ("duplicate-bone", "Species", edit("Species", ("add", ["bones"], bone("body", 0, 0, "root"))), [("/bones/2/id", "duplicate-id")], None),
        ("duplicate-part", "Species", edit("Species", ("add", ["parts"], part("body", "body", ellipse(0, 0, 4, 4), "accent"))), [("/parts/1/id", "duplicate-id")], None),
        ("duplicate-eye", "Species", edit("Species", ("add", ["face", "eyes"], eye("eye", "body", 3, -2, 3, 1.3))), [("/face/eyes/1/id", "duplicate-id")], None),
        ("duplicate-clip", "Species", edit("Species", ("add", ["clips"], clip("rest", 1, False))), [("/clips/2/id", "duplicate-id")], None),
        ("duplicate-species", "Menagerie", edit("Menagerie", ("add", ["species"], mini("mini"))), [("/species/2/id", "duplicate-id")], None),
        ("duplicate-species-path", "Ensemble", edit("Ensemble", ("add", ["species"], "mini/" + DOCUMENT)), [("/species/2", "duplicate-id")], None),
        ("unknown-parent", "Species", edit("Species", ("add", ["bones"], bone("arm", 0, 0, "ghost"))), [("/bones/2/parent", "unknown-reference")], None),
        ("part-on-unknown-bone", "Species", edit("Species", ("set", ["parts", 0, "bone"], "ghost")), [("/parts/0/bone", "unknown-reference")], None),
        ("eye-on-unknown-bone", "Species", edit("Species", ("set", ["face", "eyes", 0, "bone"], "ghost")), [("/face/eyes/0/bone", "unknown-reference")], None),
        ("mouth-on-unknown-bone", "Species", edit("Species", ("set", ["face", "mouth", "bone"], "ghost")), [("/face/mouth/bone", "unknown-reference")], None),
        ("face-above-unknown-part", "Species", edit("Species", ("set", ["face", "above"], "ghost")), [("/face/above", "unknown-reference")], None),
        ("track-on-unknown-bone", "Species", edit("Species", ("set", ["clips", 0, "tracks", 0, "bone"], "ghost")), [("/clips/0/tracks/0/bone", "unknown-reference")], None),
        ("repertoire-of-unknown-clip", "Species", edit("Species", ("set", ["repertoire", "idle"], ["rest", "ghost"])), [("/repertoire/idle/1", "unknown-reference")], None),
        ("bond-with-unknown-species", "Menagerie", edit("Menagerie", ("set", ["bonds", 0, "between"], ["mini", "ghost"])), [("/bonds/0/between/1", "unknown-reference")], None),
        ("core-of-unknown-species", "Menagerie", edit("Menagerie", ("set", ["casts", 0, "core"], ["ghost"])), [("/casts/0/core/0", "unknown-reference")], None),
        ("rotation-of-unknown-species", "Menagerie", edit("Menagerie", ("set", ["casts", 0, "rotation"], ["midi", "ghost"])), [("/casts/0/rotation/1", "unknown-reference")], None),
        ("root-with-parent", "Species", edit("Species", ("set", ["bones", 0, "parent"], "body")), [("/bones/0/parent", "bone-order")], None),
        ("root-with-unknown-parent", "Species", edit("Species", ("set", ["bones", 0, "parent"], "ghost")), [("/bones/0/parent", "bone-order"), ("/bones/0/parent", "unknown-reference")], None),
        ("second-root", "Species", edit("Species", ("add", ["bones"], bone("arm", 0, 0))), [("/bones/2", "bone-order")], None),
        ("parent-listed-later", "Species", edit("Species", ("add", ["bones"], bone("arm", 0, 0, "hand")), ("add", ["bones"], bone("hand", 0, 0, "body"))), [("/bones/2/parent", "bone-order")], None),
        ("own-parent", "Species", edit("Species", ("add", ["bones"], bone("arm", 0, 0, "arm"))), [("/bones/2/parent", "bone-order")], None),
        ("single-key", "Species", edit("Species", ("set", STEP_KEYS, [{"at": 0, "value": 0}])), [(STEP_POINTER, "key-order")], ("minItems", STEP_POINTER)),
        ("first-key-late", "Species", edit("Species", ("set", STEP_KEYS + [0, "at"], 0.1)), [(STEP_POINTER + "/0/at", "key-order")], None),
        ("last-key-early", "Species", edit("Species", ("set", STEP_KEYS + [2, "at"], 0.9)), [(STEP_POINTER + "/2/at", "key-order")], None),
        ("keys-not-ascending", "Species", edit("Species", ("set", ["clips", 1, "tracks", 0], track("body", "y", (0, 0), (0.5, -2), (0.5, -1), (1, 0)))), [(STEP_POINTER + "/2/at", "key-order")], None),
        ("key-beyond-the-clip", "Species", edit("Species", ("set", STEP_KEYS + [1, "at"], 1.5)), [(STEP_POINTER + "/1/at", "key-order"), (STEP_POINTER + "/2/at", "key-order")], ("maximum", STEP_POINTER + "/1/at")),
        ("open-loop", "Species", edit("Species", ("set", REST_KEYS + [2, "value"], 1.05)), [(REST_POINTER + "/2/value", "loop-seam")], None),
        ("half-turn-loop", "Species", edit("Species", ("set", ["clips", 1, "tracks", 0], track("body", "rotation", (0, 0), (1, 180)))), [(STEP_POINTER + "/1/value", "loop-seam")], None),
        ("almost-a-turn-loop", "Species", edit("Species", ("set", ["clips", 1, "tracks", 0], track("body", "rotation", (0, 0), (1, 359.999)))), [(STEP_POINTER + "/1/value", "loop-seam")], None),
        ("offset-of-a-full-turn", "Species", edit("Species", ("set", ["clips", 1, "tracks", 0], track("body", "x", (0, 0), (1, 360)))), [(STEP_POINTER + "/1/value", "loop-seam")], None),
        ("ease-leaves-early", "Species", edit("Species", ("set", REST_KEYS + [0, "ease"], [1.2, 0, 0.6, 1])), [(REST_POINTER + "/0/ease/0", "ease-range")], ("maximum", REST_POINTER + "/0/ease/0")),
        ("ease-arrives-backwards", "Species", edit("Species", ("set", REST_KEYS + [0, "ease"], [0.4, 0, -0.1, 1])), [(REST_POINTER + "/0/ease/2", "ease-range")], ("minimum", REST_POINTER + "/0/ease/2")),
        ("flat-size", "Species", edit("Species", ("set", ["size", "width"], 0)), [("/size/width", "out-of-range")], ("exclusiveMinimum", "/size/width")),
        ("uppercase-colour", "Species", edit("Species", ("set", ["palette", "body"], "#1E9B8D")), [("/palette/body", "out-of-range")], ("pattern", "/palette/body")),
        ("named-colour", "Species", edit("Species", ("set", ["palette", "detail"], "orange")), [("/palette/detail", "out-of-range")], ("pattern", "/palette/detail")),
        ("negative-ellipse-radius", "Species", edit("Species", ("set", ["parts", 0, "shape", "rx"], -8)), [("/parts/0/shape/rx", "out-of-range")], ("exclusiveMinimum", "/parts/0/shape/rx")),
        ("negative-corner-radius", "Species", edit("Species", ("add", ["parts"], part("box", "body", rect(-2, -2, 4, 4, -1), "accent"))), [("/parts/1/shape/radius", "out-of-range")], ("minimum", "/parts/1/shape/radius")),
        ("stroke-without-width", "Species", edit("Species", ("set", ["parts", 0, "strokeWidth"], 0)), [("/parts/0/strokeWidth", "out-of-range")], ("exclusiveMinimum", "/parts/0/strokeWidth")),
        ("eye-without-radius", "Species", edit("Species", ("set", ["face", "eyes", 0, "radius"], 0)), [("/face/eyes/0/radius", "out-of-range")], ("exclusiveMinimum", "/face/eyes/0/radius")),
        ("pupil-fills-the-eye", "Species", edit("Species", ("set", ["face", "eyes", 0, "pupil"], 3)), [("/face/eyes/0/pupil", "out-of-range")], None),
        ("mouth-without-width", "Species", edit("Species", ("set", ["face", "mouth", "width"], 0)), [("/face/mouth/width", "out-of-range")], ("exclusiveMinimum", "/face/mouth/width")),
        ("clip-without-length", "Species", edit("Species", ("set", ["clips", 0, "seconds"], 0)), [("/clips/0/seconds", "out-of-range")], ("exclusiveMinimum", "/clips/0/seconds")),
        ("backward-speed", "Species", edit("Species", ("set", ["locomotion", "speed"], -30)), [("/locomotion/speed", "out-of-range")], ("exclusiveMinimum", "/locomotion/speed")),
        ("hover-on-the-ground", "Species", edit("Species", ("set", ["locomotion"], {"gait": "float", "speed": 30, "hover": 0})), [("/locomotion/hover", "out-of-range")], ("exclusiveMinimum", "/locomotion/hover")),
        ("overdriven-trait", "Species", edit("Species", ("set", ["temperament", "energy"], 1.2)), [("/temperament/energy", "out-of-range")], ("maximum", "/temperament/energy")),
        ("overdriven-affinity", "Ensemble", edit("Ensemble", ("set", ["bonds", 0, "affinity"], -1.5)), [("/bonds/0/affinity", "out-of-range")], ("minimum", "/bonds/0/affinity")),
        ("self-bond", "Ensemble", edit("Ensemble", ("set", ["bonds", 0, "between"], ["mini", "mini"])), [("/bonds/0/between", "self-bond")], ("uniqueItems", "/bonds/0/between")),
        ("duplicate-bond", "Ensemble", edit("Ensemble", ("add", ["bonds"], {"between": ["midi", "mini"], "affinity": -0.2})), [("/bonds/1/between", "duplicate-bond")], None),
        ("walker-without-walk-clip", "Species", edit("Species", ("del", ["repertoire", "walk"])), [("/locomotion/gait", "missing-gait-clip")], None),
        ("walker-with-empty-walk", "Species", edit("Species", ("set", ["repertoire", "walk"], [])), [("/locomotion/gait", "missing-gait-clip")], None),
        ("hopper-without-hop-clip", "Species", edit("Species", ("set", ["locomotion", "gait"], "hop")), [("/locomotion/gait", "missing-gait-clip")], None),
        ("floater-without-hover", "Species", edit("Species", ("set", ["locomotion", "gait"], "float")), [("/locomotion/hover", "float-hover")], None),
        ("walker-with-hover", "Species", edit("Species", ("set", ["locomotion", "hover"], 10)), [("/locomotion/hover", "float-hover")], None),
        ("cast-without-core", "Ensemble", edit("Ensemble", ("set", ["casts", 0, "core"], [])), [("/casts/0/core", "empty-cast")], ("minItems", "/casts/0/core")),
        ("duplicate-scene", "Ensemble", edit("Ensemble", ("add", ["casts"], {"scene": "home", "core": ["midi"], "rotation": []})), [("/casts/1/scene", "duplicate-scene")], None),
        ("capital-id", "Species", edit("Species", ("set", ["id"], "Mini")), [("/id", "slug-invalid")], ("pattern", "/id")),
        ("bone-id-with-space", "Species", edit("Species", ("add", ["bones"], {**arm, "id": "left arm"})), [("/bones/2/id", "slug-invalid")], ("pattern", "/bones/2/id")),
        ("empty-name", "Species", edit("Species", ("set", ["name", "en"], "")), [("/name/en", "length-invalid")], ("minLength", "/name/en")),
        ("empty-ground", "Species", edit("Species", ("add", ["grounds"], "")), [("/grounds/1", "length-invalid")], ("minLength", "/grounds/1")),
        ("empty-outline", "Species", edit("Species", ("add", ["parts"], part("mark", "body", outline(""), "none"))), [("/parts/1/shape/d", "length-invalid")], ("minLength", "/parts/1/shape/d")),
        ("empty-species-path", "Ensemble", edit("Ensemble", ("set", ["species", 0], "")), [("/species/0", "length-invalid")], ("minLength", "/species/0")),
        (
            "boneless",
            "Species",
            edit("Species", ("set", ["bones"], []), ("set", ["parts"], []), ("set", ["face"], {"eyes": []}), ("set", ["clips", 0, "tracks"], []), ("set", ["clips", 1, "tracks"], [])),
            [("/bones", "items-too-few")],
            ("minItems", "/bones"),
        ),
        (
            "several-findings",
            "Menagerie",
            edit("Menagerie", ("set", ["species", 1, "id"], "mini"), ("set", ["species", 0, "temperament", "energy"], 2)),
            [("/bonds/0/between/1", "unknown-reference"), ("/casts/0/rotation/0", "unknown-reference"), ("/species/0/temperament/energy", "out-of-range"), ("/species/1/id", "duplicate-id")],
            ("maximum", "/species/0/temperament/energy"),
        ),
    ]
    vectors = []
    for identifier, definition, document, intended, verdict in rows:
        found = CASE.findings(definition, document)
        if found != [{"path": path, "code": code} for path, code in intended]:
            raise AssertionError("%s: intended %r, the reference finds %r" % (identifier, intended, found))
        vectors.append({"id": identifier, "definition": definition, "document": document, "issues": found, "violates": None if verdict is None else {"keyword": verdict[0], "at": verdict[1]}})
    return vectors


# endregion 🔖️Vectors


# region 🔖️Checks
class Replay:
    """🔁️ What the oracle handlers read from the harness: the scenario and the fixture bytes."""

    def __init__(self, payload):
        self.payload = payload
        self.scenario = {"id": "generation", "steps": []}

    def fixture_bytes(self, uri):
        """📥️ The vectors being generated, whatever the URI."""
        return self.payload


def main():
    """🏁️ Composes the vectors, replays the three oracle handlers on them and writes the fixture."""
    ensemble, species, menagerie = sample()
    comment = "Generated by .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/generate_schema_vectors.py from the Python reference in 🧪️tests/🧬️schema-conformance/🐍️.py — never edit by hand."
    document = {"$comment": comment, "menagerie": menagerie, "ensemble": ensemble, "species": species, "bases": copy.deepcopy(BASES), "accepted": accepted(), "structural": structural(), "rules": rules()}
    ids = [vector["id"] for group in ["accepted", "structural", "rules"] for vector in document[group]]
    if len(ids) != len(set(ids)):
        raise AssertionError("vector ids repeat: %r" % sorted(identifier for identifier in set(ids) if ids.count(identifier) > 1))
    replay = Replay(json.dumps(document, ensure_ascii=False).encode("utf-8"))
    for handler in [CASE.accepted_documents, CASE.structural_rejections, CASE.rule_violations]:
        handler(replay)
    codes = sorted({issue["code"] for group in ["structural", "rules"] for vector in document[group] for issue in vector["issues"]})
    print("accepted=%d structural=%d rules=%d codes=%s" % (len(document["accepted"]), len(document["structural"]), len(document["rules"]), ",".join(codes)))
    write(document)


# endregion 🔖️Checks


if __name__ == "__main__":
    main()
