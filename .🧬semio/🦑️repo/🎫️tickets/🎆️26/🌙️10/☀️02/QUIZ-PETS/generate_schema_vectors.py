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


def state(identifier, name, tint=None, clip=None, emitter=None, lasts=None, then=None):
    """🔦️ A state of a species."""
    entry = {"id": identifier, "name": name}
    for key, value in [("tint", tint), ("clip", clip), ("emitter", emitter), ("lasts", lasts), ("then", then)]:
        if value is not None:
            entry[key] = value
    return entry


def trick(identifier, name, clip, cues, emitter=None, origin=None, to=None, mood=None):
    """🪄️ A trick of a species; ``origin`` lists the states it is on offer in (``from`` on the wire)."""
    entry = {"id": identifier, "name": name, "clip": clip, "cues": list(cues)}
    for key, value in [("emitter", emitter), ("from", origin), ("to", to), ("mood", mood)]:
        if value is not None:
            entry[key] = value
    return entry


def emitter(identifier, on, x, y, shape, fill, stroke, motion, count, life, speed, spread, width=None):
    """✨️ An emitter on a bone."""
    entry = {"id": identifier, "bone": on, "x": x, "y": y, "shape": shape, "fill": fill, "stroke": stroke}
    if width is not None:
        entry["strokeWidth"] = width
    entry.update({"motion": motion, "count": count, "life": life, "speed": speed, "spread": spread})
    return entry


def reaction(identifier, when, near, within, every, then, where=None, chance=None, unless=None, affinity=None):
    """⚗️ A reaction between two species; ``when``, ``near`` and ``unless`` are traits, ``affinity`` the bounds ``[low, high]``, ``then`` a list of effects."""
    entry = {"id": identifier, "when": when, "near": near, "within": within}
    if where is not None:
        entry["where"] = where
    if unless is not None:
        entry["unless"] = unless
    if affinity is not None:
        entry["affinity"] = affinity
    entry["every"] = every
    if chance is not None:
        entry["chance"] = chance
    entry["then"] = list(then)
    return entry


RESTING = {"id": "resting", "name": {"en": "Resting", "de": "In Ruhe"}}


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
            clip("shimmer", 1.6, True, track("body", "scaleX", (0, 1), (0.5, 1.03), (1, 1), ease=SOFT), track("tuft", "scaleY", (0, 1), (0.5, 1.2), (1, 1), ease=SOFT)),
            clip(
                "dangle",
                1.2,
                True,
                track("leg-left", "rotation", (0, -12), (0.5, 12), (1, -12), ease=SOFT),
                track("leg-right", "rotation", (0, 12), (0.5, -12), (1, 12), ease=SOFT),
                track("arm-left", "rotation", (0, -70), (0.5, -60), (1, -70)),
                track("arm-right", "rotation", (0, 70), (0.5, 60), (1, 70)),
            ),
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
            "hang": ["dangle"],
            "tumble": ["flail"],
            "glide": ["dangle"],
            "aim": ["stretch"],
            "reel": ["dangle"],
            "climb": ["stroll"],
            "mantle": ["leap"],
            "slide": ["flail"],
            "carry": ["stroll"],
            "trick": ["wiggle"],
            "purr": ["nuzzle"],
            "dizzy": ["wiggle"],
            "shrug": ["stretch"],
            "scoot": ["stroll"],
            "push": ["stomp"],
        },
        "locomotion": {"gait": "walk", "speed": 36},
        "temperament": {"energy": 0.6, "sociability": 0.7, "curiosity": 0.6},
        "states": [
            RESTING,
            state("glowing", T("Glowing", "Glimmend"), tint={"body": "#35c9b8"}, emitter="sparkle", lasts=30, then="resting"),
            state("radiant", T("Radiant", "Strahlend"), tint={"body": "#7ee8d9", "accent": "#fccf05"}, clip="shimmer", emitter="sparkle", lasts=12, then="glowing"),
        ],
        "tricks": [
            trick("cheer", T("Cheer", "Jubeln"), "stretch", ["click", "whim"], emitter="sparkle", mood="happy"),
            trick("brighten", T("Brighten", "Aufleuchten"), "wiggle", ["circle"], origin=["resting"], to="glowing", mood="proud"),
            trick("blaze", T("Blaze", "Erstrahlen"), "wiggle", ["circle", "show"], emitter="sparkle", origin=["glowing"], to="radiant", mood="proud"),
            trick("wane", T("Wane", "Nachlassen"), "squash", ["countercircle"], origin=["radiant"], to="glowing"),
            trick("fade", T("Fade", "Verblassen"), "squash", ["countercircle"], origin=["glowing"], to="resting", mood="sleepy"),
            trick("shake-off", T("Shake off", "Abschütteln"), "wiggle", ["shake"], mood="grumpy"),
        ],
        "purr": {"clip": "nuzzle", "emitter": "hum"},
        "emitters": [
            emitter("sparkle", "tuft", 0, -4, outline("M 0 -3 L 1 -1 L 3 0 L 1 1 L 0 3 L -1 1 L -3 0 L -1 -1 Z"), "accent", "none", "burst", 6, 0.8, 40, 1),
            emitter("hum", "body", 0, -14, ellipse(0, 0, 1.5, 1.5), "detail", "ink", "rise", 3, 1.2, 14, 0.1, width=1),
        ],
        "gear": ["climb", "ladder", "grapple", "parachute"],
        "grip": 27,
        "reach": 10,
        "canopy": outline("M -16 0 Q 0 -18 16 0 Z"),
        "mood": "content",
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
            "hang": ["breathe"],
            "tumble": ["breathe"],
            "glide": ["breathe"],
            "aim": ["perk"],
            "reel": ["bounce"],
            "purr": ["snuggle"],
            "dizzy": ["ear-twitch"],
            "shrug": ["thump"],
            "push": ["stamp"],
        },
        "locomotion": {"gait": "hop", "speed": 44},
        "temperament": {"energy": 0.9, "sociability": 0.5, "curiosity": 0.8},
        "states": [RESTING, state("wound-up", T("Wound up", "Aufgezogen"), tint={"accent": "#f2d9b8"}, lasts=10, then="resting")],
        "tricks": [
            trick("boing", T("Boing", "Boing"), "thump", ["click", "stroke"], mood="playful"),
            trick("wind-up", T("Wind up", "Aufziehen"), "ear-twitch", ["circle", "shake"], origin=["resting"], to="wound-up"),
        ],
        "purr": {"clip": "snuggle"},
        "emitters": [],
        "gear": ["grapple", "parachute"],
        "grip": 32,
        "reach": 7.5,
        "mood": "playful",
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
        "repertoire": {
            "idle": ["bob"],
            "fidget": ["twirl", "puff-up"],
            "walk": ["bob"],
            "land": ["settle"],
            "sleep": ["drift"],
            "greet": ["dip"],
            "cuddle": ["glow"],
            "squabble": ["whirl"],
            "sulk": ["sag"],
            "hang": ["bob"],
            "tumble": ["bob"],
            "glide": ["drift"],
            "purr": ["glow"],
            "dizzy": ["twirl"],
            "shrug": ["dip"],
            "push": ["puff-up"],
        },
        "locomotion": {"gait": "float", "speed": 28, "hover": 14},
        "temperament": {"energy": 0.3, "sociability": 0.6, "curiosity": 0.4},
        "states": [RESTING],
        "tricks": [],
        "purr": {"clip": "glow"},
        "emitters": [],
        "gear": ["parachute"],
        "grip": 27,
        "reach": 8.5,
        "mood": "sleepy",
    }


def sample():
    """🎪️ The sample ensemble, its species documents in ensemble order, and the menagerie they assemble to."""
    members = [("🫧️blobby/" + DOCUMENT, blobby()), ("🐇️hoppy/" + DOCUMENT, hoppy()), ("🎈️floaty/" + DOCUMENT, floaty())]
    bonds = [{"between": ["blobby", "hoppy"], "affinity": 0.6}, {"between": ["blobby", "floaty"], "affinity": -0.4}, {"between": ["hoppy", "floaty"], "affinity": 0.1}]
    casts = [{"scene": "home", "core": ["blobby", "hoppy"], "rotation": ["floaty"]}, {"scene": "meadow", "core": ["hoppy"], "rotation": ["floaty", "blobby"]}]
    chemistry = [
        reaction(
            "radiance-cheers-the-spring",
            {"species": "blobby", "state": "radiant"},
            {"species": "hoppy"},
            120,
            20,
            [{"on": "near", "mood": "playful", "amount": 0.6}, {"on": "near", "trick": "boing"}, {"on": "when", "mood": "proud", "amount": 0.4, "rapport": 0.05}],
            chance=0.5,
        ),
        reaction("a-wound-up-spring-startles-the-balloon", {"species": "hoppy", "state": "wound-up"}, {"species": "floaty"}, 80, 15, [{"on": "near", "mood": "scared", "amount": 0.7, "rapport": -0.05}, {"on": "when", "state": "resting"}], where="below"),
        reaction("a-sleepy-balloon-calms-a-grumpy-blob", {"species": "floaty", "mood": "sleepy", "activity": "idle"}, {"species": "blobby", "mood": "grumpy"}, 60, 30, [{"on": "near", "mood": "content", "amount": 0.5, "encounter": "greet"}], where="above"),
        reaction("a-glowing-blob-warms-its-friends", {"species": "blobby", "state": "glowing", "held": 4}, {}, 100, 20, [{"on": "near", "mood": "happy", "amount": 0.5}, {"on": "near", "encounter": "cuddle"}], affinity=[0.4, 1]),
        reaction("a-boing-entertains-anyone", {"species": "hoppy", "trick": "boing"}, {}, 120, 10, [{"on": "near", "mood": "happy", "amount": 0.4}], unless={"species": "floaty", "mood": "scared"}),
        reaction("a-purr-lulls-a-sleepy-neighbour", {"mood": "sleepy"}, {"activity": "purr"}, 80, 16, [{"on": "when", "activity": "sleep"}, {"on": "near", "mood": "sleepy", "amount": 0.3}], chance=0.5),
    ]
    title = T("Sample menagerie", "Beispiel-Menagerie")
    ensemble = {"$schema": "../../🧬️schema/🔣️.json#/$defs/Ensemble", "schema": ENSEMBLE_SCHEMA, "id": "sample", "title": title, "species": [path for path, _ in members], "bonds": bonds, "casts": casts, "chemistry": chemistry}
    species = [{"path": path, "document": {"$schema": "../../../🧬️schema/🔣️.json#/$defs/Species", **document}} for path, document in members]
    menagerie = {"schema": MENAGERIE_SCHEMA, "id": "sample", "title": title, "species": [document for _, document in members], "bonds": bonds, "casts": casts, "chemistry": chemistry}
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
        "repertoire": {"idle": ["rest"], "walk": ["step"], "hang": ["rest"], "tumble": ["rest"], "purr": ["rest"], "dizzy": ["step"], "shrug": ["rest"], "push": ["step"]},
        "locomotion": {"gait": "walk", "speed": 30},
        "temperament": {"energy": 0.5, "sociability": 0.5, "curiosity": 0.5},
        "states": [RESTING],
        "tricks": [],
        "purr": {"clip": "rest"},
        "emitters": [],
        "gear": [],
        "grip": 14,
        "reach": 4,
        "mood": "content",
    }


def duo():
    """🔸️ What the minimal menagerie and ensemble share: one bond and one cast over two species, and no chemistry."""
    return {"bonds": [{"between": ["mini", "midi"], "affinity": 0.5}], "casts": [{"scene": "home", "core": ["mini"], "rotation": ["midi"]}], "chemistry": []}


BASES = {
    "species": mini(),
    "menagerie": {"schema": MENAGERIE_SCHEMA, "id": "base", "title": T("Base", "Basis"), "species": [mini("mini"), mini("midi")], **duo()},
    "ensemble": {"schema": ENSEMBLE_SCHEMA, "id": "base", "title": T("Base", "Basis"), "species": ["mini/" + DOCUMENT, "midi/" + DOCUMENT], **duo()},
}
BASE_OF = {"Species": "species", "Menagerie": "menagerie", "Ensemble": "ensemble"}


def hello(identifier="hello", when=None, near=None, within=40, every=10, then=None, where=None, chance=None, unless=None, affinity=None):
    """👋️ The smallest reaction of the bases, with the members a vector changes: a happy mini cheers up the midi beside it."""
    return reaction(identifier, {"species": "mini"} if when is None else when, {"species": "midi"} if near is None else near, within, every, [{"on": "near", "mood": "happy", "amount": 0.5}] if then is None else then, where=where, chance=chance, unless=unless, affinity=affinity)


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
        ("floater-without-walk-clip", "Species", edit("Species", ("set", ["locomotion"], {"gait": "float", "speed": 20, "hover": 6}), ("del", ["repertoire", "walk"]))),
        ("face-above-a-part", "Species", edit("Species", ("set", ["face", "above"], "body"))),
        ("species-with-schema-hint", "Species", {"$schema": "../../🧬️schema/🔣️.json#/$defs/Species", **mini()}),
        ("menagerie-with-schema-hint", "Menagerie", {"$schema": "../🧬️schema/🔣️.json", **copy.deepcopy(BASES["menagerie"])}),
        ("empty-menagerie", "Menagerie", {"schema": MENAGERIE_SCHEMA, "id": "empty", "title": T("Empty", "Leer"), "species": [], "bonds": [], "casts": [], "chemistry": []}),
        ("opposed-bond", "Ensemble", edit("Ensemble", ("set", ["bonds", 0, "affinity"], -1))),
        ("trick-without-cues", "Species", edit("Species", ("add", ["tricks"], trick("bow", T("Bow", "Verbeugung"), "step", [])))),
        ("state-that-lasts", "Species", edit("Species", ("add", ["states"], state("lit", T("Lit", "Erleuchtet"), tint={"detail": "#fccf05"}, clip="step", lasts=5, then="resting")))),
        ("state-that-names-a-successor-and-stays", "Species", edit("Species", ("add", ["states"], state("lit", T("Lit", "Erleuchtet"), then="resting")))),
        ("states-that-give-way-to-each-other", "Species", edit("Species", ("set", ["states", 0, "lasts"], 4), ("set", ["states", 0, "then"], "lit"), ("add", ["states"], state("lit", T("Lit", "Erleuchtet"), lasts=4, then="resting")))),
        ("floater-with-parachute", "Species", edit("Species", ("set", ["locomotion"], {"gait": "float", "speed": 20, "hover": 6}), ("set", ["gear"], ["parachute"]), ("set", ["repertoire", "glide"], ["rest"]))),
        ("walker-with-every-gear", "Species", edit("Species", ("set", ["gear"], ["climb", "ladder", "grapple", "parachute"]), *[("set", ["repertoire", activity], ["step"]) for activity in ["glide", "aim", "reel", "climb", "mantle", "slide", "carry"]], ("set", ["canopy"], rect(-8, -6, 16, 6, 3)))),
        ("grip-at-the-top-and-reach-of-the-whole-width", "Species", edit("Species", ("set", ["grip"], 16), ("set", ["reach"], 16))),
        ("no-reach", "Species", edit("Species", ("set", ["reach"], 0))),
        ("emitter-at-its-bounds", "Species", edit("Species", ("add", ["emitters"], emitter("spark", "body", 0, -8, line(0, 0, 0, -3), "none", "accent", "orbit", 32, 0.25, 0, 0, width=1.5)), ("add", ["emitters"], emitter("dust", "root", -2, 0, rect(-1, -1, 2, 2), "paper", "none", "drift", 1, 2, 6, 1)), ("set", ["purr", "emitter"], "dust"))),
        (
            "reaction-with-everything",
            "Menagerie",
            edit(
                "Menagerie",
                ("add", ["species", 1, "tricks"], trick("bow", T("Bow", "Verbeugung"), "step", ["show"])),
                ("add", ["chemistry"], reaction("hello", {"species": "mini", "state": "resting", "mood": "happy", "activity": "purr"}, {"species": "midi", "mood": "content"}, 48, 12, [{"on": "near", "state": "resting", "mood": "proud", "amount": 1, "rapport": -1, "encounter": "cuddle", "trick": "bow"}, {"on": "when", "amount": 0, "rapport": 1}], where="beside", chance=1)),
                ("add", ["chemistry"], reaction("anywhere", {"species": "midi"}, {"species": "mini"}, 0.5, 0.25, [{"on": "when"}], where="any", chance=0)),
            ),
        ),
        ("reaction-of-unassembled-species", "Ensemble", edit("Ensemble", ("add", ["chemistry"], reaction("hello", {"species": "ghost", "state": "haunting"}, {"species": "mini"}, 40, 10, [{"on": "when", "state": "fading", "trick": "boo"}])))),
        (
            "reaction-of-anyone",
            "Menagerie",
            edit(
                "Menagerie",
                ("add", ["species", 1, "tricks"], trick("bow", T("Bow", "Verbeugung"), "step", ["click"])),
                ("add", ["chemistry"], reaction("anyone", {}, {"state": "resting", "held": 0.5, "trick": "bow", "activity": "trick"}, 30, 2, [{"on": "when", "activity": "fidget"}, {"on": "near", "activity": "sleep", "trick": "bow"}], unless={"species": "mini", "mood": "scared"}, affinity=[-1, 1])),
                ("add", ["chemistry"], reaction("between-friends", {"species": "mini"}, {"species": "midi"}, 30, 2, [{"on": "near", "activity": "walk"}, {"on": "when", "activity": "hop"}], unless={}, affinity=[0.5, 0.5])),
            ),
        ),
    ]
    vectors = [{"id": identifier, "definition": definition, "pointer": pointer} for identifier, definition, pointer in referred]
    vectors += [{"id": identifier, "definition": definition, "document": document} for identifier, definition, document in inline]
    return vectors


def structural():
    """🚫️ Documents that break the type-level structure, each at one keyword of the schema."""
    rows = [
        ("not-an-object", "Species", [], "type", ""),
        ("missing-size", "Species", edit("Species", ("del", ["size"])), "required", "/size"),
        ("undeclared-property", "Species", edit("Species", ("set", ["nickname"], 1)), "additionalProperties", "/nickname"),
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
        ("missing-states", "Species", edit("Species", ("del", ["states"])), "required", "/states"),
        ("missing-tricks", "Species", edit("Species", ("del", ["tricks"])), "required", "/tricks"),
        ("missing-purr", "Species", edit("Species", ("del", ["purr"])), "required", "/purr"),
        ("missing-emitters", "Species", edit("Species", ("del", ["emitters"])), "required", "/emitters"),
        ("missing-gear", "Species", edit("Species", ("del", ["gear"])), "required", "/gear"),
        ("missing-grip", "Species", edit("Species", ("del", ["grip"])), "required", "/grip"),
        ("missing-reach", "Species", edit("Species", ("del", ["reach"])), "required", "/reach"),
        ("missing-mood", "Species", edit("Species", ("del", ["mood"])), "required", "/mood"),
        ("unknown-mood", "Species", edit("Species", ("set", ["mood"], "hangry")), "enum", "/mood"),
        ("unknown-gear", "Species", edit("Species", ("set", ["gear"], ["jetpack"])), "enum", "/gear/0"),
        ("gear-not-a-list", "Species", edit("Species", ("set", ["gear"], "parachute")), "type", "/gear"),
        ("unknown-cue", "Species", edit("Species", ("add", ["tricks"], trick("bow", T("Bow", "Verbeugung"), "step", ["whistle"]))), "enum", "/tricks/0/cues/0"),
        ("trick-without-clip", "Species", edit("Species", ("add", ["tricks"], {"id": "bow", "name": T("Bow", "Verbeugung"), "cues": ["click"]})), "required", "/tricks/0/clip"),
        ("unknown-drift", "Species", edit("Species", ("add", ["emitters"], emitter("spark", "body", 0, -8, ellipse(0, 0, 1, 1), "accent", "none", "swirl", 4, 0.6, 30, 1))), "enum", "/emitters/0/motion"),
        ("fractional-particle-count", "Species", edit("Species", ("add", ["emitters"], emitter("spark", "body", 0, -8, ellipse(0, 0, 1, 1), "accent", "none", "burst", 2.5, 0.6, 30, 1))), "type", "/emitters/0/count"),
        ("undeclared-tint", "Species", edit("Species", ("set", ["states", 0, "tint"], {"glow": "#ffffff"})), "additionalProperties", "/states/0/tint/glow"),
        ("state-without-name", "Species", edit("Species", ("del", ["states", 0, "name"])), "required", "/states/0/name"),
        ("lasts-not-a-number", "Species", edit("Species", ("set", ["states", 0, "lasts"], "long"), ("set", ["states", 0, "then"], "resting")), "type", "/states/0/lasts"),
        ("purr-without-clip", "Species", edit("Species", ("set", ["purr"], {})), "required", "/purr/clip"),
        ("menagerie-without-chemistry", "Menagerie", edit("Menagerie", ("del", ["chemistry"])), "required", "/chemistry"),
        ("ensemble-without-chemistry", "Ensemble", edit("Ensemble", ("del", ["chemistry"])), "required", "/chemistry"),
        ("effect-on-nobody", "Menagerie", edit("Menagerie", ("add", ["chemistry"], hello(then=[{"on": "both"}]))), "enum", "/chemistry/0/then/0/on"),
        ("effect-without-side", "Menagerie", edit("Menagerie", ("add", ["chemistry"], hello(then=[{"mood": "happy"}]))), "required", "/chemistry/0/then/0/on"),
        ("unknown-placement", "Ensemble", edit("Ensemble", ("add", ["chemistry"], hello(where="inside"))), "enum", "/chemistry/0/where"),
        ("unknown-encounter", "Ensemble", edit("Ensemble", ("add", ["chemistry"], hello(then=[{"on": "near", "encounter": "hug"}]))), "enum", "/chemistry/0/then/0/encounter"),
        ("trait-doing-an-unknown-activity", "Menagerie", edit("Menagerie", ("add", ["chemistry"], hello(near={"species": "midi", "activity": "dance"}))), "enum", "/chemistry/0/near/activity"),
        ("unless-in-an-unknown-mood", "Menagerie", edit("Menagerie", ("add", ["chemistry"], hello(unless={"mood": "hangry"}))), "enum", "/chemistry/0/unless/mood"),
        ("effect-setting-off-on-a-greeting", "Ensemble", edit("Ensemble", ("add", ["chemistry"], hello(then=[{"on": "near", "activity": "greet"}]))), "enum", "/chemistry/0/then/0/activity"),
        ("affinity-not-a-list", "Menagerie", edit("Menagerie", ("add", ["chemistry"], hello(affinity=0.5))), "type", "/chemistry/0/affinity"),
        ("held-not-a-number", "Menagerie", edit("Menagerie", ("add", ["chemistry"], hello(when={"species": "mini", "held": "long"}))), "type", "/chemistry/0/when/held"),
        ("reaction-without-period", "Menagerie", edit("Menagerie", ("add", ["chemistry"], {key: value for key, value in hello().items() if key != "every"})), "required", "/chemistry/0/every"),
    ]
    return [{"id": identifier, "definition": definition, "document": document, "violates": {"keyword": keyword, "at": pointer}, "issues": [{"path": pointer, "code": CASE.CODES[keyword]}]} for identifier, definition, document, keyword, pointer in rows]


def rules():
    """⚖️ Structurally sound documents that break a rule: ``(id, definition, document, findings, schema verdict)``."""
    arm = bone("arm", 0, 0, "body")
    bow = trick("bow", T("Bow", "Verbeugung"), "step", ["click"])
    floating = ("set", ["locomotion"], {"gait": "float", "speed": 20, "hover": 6})

    def spark(bone="body", shape=None, count=4, life=0.6, speed=30, spread=1, width=None):
        """🎇️ The emitter the rule vectors add to the base species, with the members a vector changes."""
        return emitter("spark", bone, 0, -8, ellipse(0, 0, 1, 1) if shape is None else shape, "accent", "none" if width is None else "ink", "burst", count, life, speed, spread, width=width)

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
        ("duplicate-state", "Species", edit("Species", ("add", ["states"], state("resting", T("Resting again", "Wieder in Ruhe")))), [("/states/1/id", "duplicate-id")], None),
        ("duplicate-trick", "Species", edit("Species", ("add", ["tricks"], bow), ("add", ["tricks"], bow)), [("/tricks/1/id", "duplicate-id")], None),
        ("duplicate-emitter", "Species", edit("Species", ("add", ["emitters"], spark()), ("add", ["emitters"], spark())), [("/emitters/1/id", "duplicate-id")], None),
        ("state-with-unknown-clip", "Species", edit("Species", ("set", ["states", 0, "clip"], "ghost")), [("/states/0/clip", "unknown-reference")], None),
        ("state-with-unknown-emitter", "Species", edit("Species", ("set", ["states", 0, "emitter"], "ghost")), [("/states/0/emitter", "unknown-reference")], None),
        ("state-giving-way-to-unknown-state", "Species", edit("Species", ("set", ["states", 0, "lasts"], 5), ("set", ["states", 0, "then"], "ghost")), [("/states/0/then", "unknown-reference")], None),
        ("lasting-state-without-successor", "Species", edit("Species", ("set", ["states", 0, "lasts"], 5)), [("/states/0/then", "lasts-then")], None),
        ("trick-with-unknown-clip", "Species", edit("Species", ("add", ["tricks"], {**bow, "clip": "ghost"})), [("/tricks/0/clip", "unknown-reference")], None),
        ("trick-with-unknown-emitter", "Species", edit("Species", ("add", ["tricks"], {**bow, "emitter": "ghost"})), [("/tricks/0/emitter", "unknown-reference")], None),
        ("trick-from-unknown-state", "Species", edit("Species", ("add", ["tricks"], {**bow, "from": ["resting", "ghost"]})), [("/tricks/0/from/1", "unknown-reference")], None),
        ("trick-to-unknown-state", "Species", edit("Species", ("add", ["tricks"], {**bow, "to": "ghost"})), [("/tricks/0/to", "unknown-reference")], None),
        ("purr-with-unknown-clip", "Species", edit("Species", ("set", ["purr", "clip"], "ghost")), [("/purr/clip", "unknown-reference")], None),
        ("purr-with-unknown-emitter", "Species", edit("Species", ("set", ["purr", "emitter"], "ghost")), [("/purr/emitter", "unknown-reference")], None),
        ("emitter-on-unknown-bone", "Species", edit("Species", ("add", ["emitters"], spark(bone="ghost"))), [("/emitters/0/bone", "unknown-reference")], None),
        ("stateless", "Species", edit("Species", ("set", ["states"], [])), [("/states", "items-too-few")], ("minItems", "/states")),
        ("repeated-cue", "Species", edit("Species", ("add", ["tricks"], {**bow, "cues": ["click", "circle", "click"]})), [("/tricks/0/cues/2", "duplicate-entry")], ("uniqueItems", "/tricks/0/cues")),
        ("repeated-origin", "Species", edit("Species", ("add", ["tricks"], {**bow, "from": ["resting", "resting"]})), [("/tricks/0/from/1", "duplicate-entry")], ("uniqueItems", "/tricks/0/from")),
        ("repeated-gear", "Species", edit("Species", ("set", ["gear"], ["parachute", "parachute"]), ("set", ["repertoire", "glide"], ["rest"])), [("/gear/1", "duplicate-entry")], ("uniqueItems", "/gear")),
        ("climber-without-climbing-clips", "Species", edit("Species", ("set", ["gear"], ["climb"])), [("/gear/0", "missing-gear-clip")], None),
        ("climber-that-cannot-slide", "Species", edit("Species", ("set", ["gear"], ["climb"]), ("set", ["repertoire", "climb"], ["step"]), ("set", ["repertoire", "mantle"], ["step"])), [("/gear/0", "missing-gear-clip")], None),
        ("ladder-without-carry-clip", "Species", edit("Species", ("set", ["gear"], ["ladder"]), ("set", ["repertoire", "climb"], ["step"])), [("/gear/0", "missing-gear-clip")], None),
        ("grapple-without-reel-clip", "Species", edit("Species", ("set", ["gear"], ["grapple"]), ("set", ["repertoire", "aim"], ["step"])), [("/gear/0", "missing-gear-clip")], None),
        ("parachute-without-glide-clip", "Species", edit("Species", ("set", ["gear"], ["parachute"])), [("/gear/0", "missing-gear-clip")], None),
        ("parachute-with-empty-glide", "Species", edit("Species", ("set", ["gear"], ["parachute"]), ("set", ["repertoire", "glide"], [])), [("/gear/0", "missing-gear-clip")], None),
        ("second-gear-without-clips", "Species", edit("Species", ("set", ["gear"], ["parachute", "grapple"]), ("set", ["repertoire", "glide"], ["rest"])), [("/gear/1", "missing-gear-clip")], None),
        ("floater-with-ladder", "Species", edit("Species", floating, ("set", ["gear"], ["ladder"]), ("set", ["repertoire", "carry"], ["step"]), ("set", ["repertoire", "climb"], ["step"])), [("/gear/0", "floater-gear")], None),
        (
            "floater-with-every-gear",
            "Species",
            edit("Species", floating, ("set", ["gear"], ["parachute", "climb", "ladder", "grapple"]), *[("set", ["repertoire", activity], ["step"]) for activity in ["glide", "aim", "reel", "climb", "mantle", "slide", "carry"]]),
            [("/gear/1", "floater-gear"), ("/gear/2", "floater-gear"), ("/gear/3", "floater-gear")],
            None,
        ),
        ("floater-that-climbs-without-clips", "Species", edit("Species", floating, ("set", ["gear"], ["climb"])), [("/gear/0", "floater-gear"), ("/gear/0", "missing-gear-clip")], None),
        ("species-that-cannot-hang", "Species", edit("Species", ("del", ["repertoire", "hang"])), [("/repertoire/hang", "missing-activity-clip")], None),
        ("species-with-no-purr-clip", "Species", edit("Species", ("set", ["repertoire", "purr"], [])), [("/repertoire/purr", "missing-activity-clip")], None),
        (
            "first-round-repertoire",
            "Species",
            edit("Species", ("set", ["repertoire"], {"idle": ["rest"], "walk": ["step"]})),
            [("/repertoire/dizzy", "missing-activity-clip"), ("/repertoire/hang", "missing-activity-clip"), ("/repertoire/purr", "missing-activity-clip"), ("/repertoire/push", "missing-activity-clip"), ("/repertoire/shrug", "missing-activity-clip"), ("/repertoire/tumble", "missing-activity-clip")],
            None,
        ),
        ("state-that-lasts-no-time", "Species", edit("Species", ("set", ["states", 0, "lasts"], 0), ("set", ["states", 0, "then"], "resting")), [("/states/0/lasts", "out-of-range")], ("exclusiveMinimum", "/states/0/lasts")),
        ("uppercase-tint", "Species", edit("Species", ("set", ["states", 0, "tint"], {"body": "#1E9B8D"})), [("/states/0/tint/body", "out-of-range")], ("pattern", "/states/0/tint/body")),
        ("empty-state-name", "Species", edit("Species", ("set", ["states", 0, "name", "de"], "")), [("/states/0/name/de", "length-invalid")], ("minLength", "/states/0/name/de")),
        ("capital-state-id", "Species", edit("Species", ("set", ["states", 0, "id"], "Resting")), [("/states/0/id", "slug-invalid")], ("pattern", "/states/0/id")),
        ("emitter-without-particles", "Species", edit("Species", ("add", ["emitters"], spark(count=0))), [("/emitters/0/count", "out-of-range")], ("minimum", "/emitters/0/count")),
        ("emitter-with-too-many-particles", "Species", edit("Species", ("add", ["emitters"], spark(count=33))), [("/emitters/0/count", "out-of-range")], ("maximum", "/emitters/0/count")),
        ("emitter-without-life", "Species", edit("Species", ("add", ["emitters"], spark(life=0))), [("/emitters/0/life", "out-of-range")], ("exclusiveMinimum", "/emitters/0/life")),
        ("emitter-running-backwards", "Species", edit("Species", ("add", ["emitters"], spark(speed=-1))), [("/emitters/0/speed", "out-of-range")], ("minimum", "/emitters/0/speed")),
        ("emitter-spread-beyond-a-turn", "Species", edit("Species", ("add", ["emitters"], spark(spread=1.5))), [("/emitters/0/spread", "out-of-range")], ("maximum", "/emitters/0/spread")),
        ("emitter-stroke-without-width", "Species", edit("Species", ("add", ["emitters"], spark(width=0))), [("/emitters/0/strokeWidth", "out-of-range")], ("exclusiveMinimum", "/emitters/0/strokeWidth")),
        ("emitter-of-a-flat-particle", "Species", edit("Species", ("add", ["emitters"], spark(shape=ellipse(0, 0, 0, 1)))), [("/emitters/0/shape/rx", "out-of-range")], ("exclusiveMinimum", "/emitters/0/shape/rx")),
        ("grip-at-the-feet", "Species", edit("Species", ("set", ["grip"], 0)), [("/grip", "out-of-range")], ("exclusiveMinimum", "/grip")),
        ("grip-above-the-head", "Species", edit("Species", ("set", ["grip"], 16.5)), [("/grip", "out-of-range")], None),
        ("reach-backwards", "Species", edit("Species", ("set", ["reach"], -1)), [("/reach", "out-of-range")], ("minimum", "/reach")),
        ("reach-beyond-the-body", "Species", edit("Species", ("set", ["reach"], 16.5)), [("/reach", "out-of-range")], None),
        ("flat-canopy", "Species", edit("Species", ("set", ["canopy"], rect(-8, -6, 16, 0))), [("/canopy/height", "out-of-range")], ("exclusiveMinimum", "/canopy/height")),
        ("duplicate-reaction", "Menagerie", edit("Menagerie", ("add", ["chemistry"], hello()), ("add", ["chemistry"], hello())), [("/chemistry/1/id", "duplicate-id")], None),
        ("duplicate-reaction-in-an-ensemble", "Ensemble", edit("Ensemble", ("add", ["chemistry"], hello()), ("add", ["chemistry"], hello())), [("/chemistry/1/id", "duplicate-id")], None),
        ("capital-reaction-id", "Ensemble", edit("Ensemble", ("add", ["chemistry"], hello("Hello"))), [("/chemistry/0/id", "slug-invalid")], ("pattern", "/chemistry/0/id")),
        ("reaction-of-unknown-species", "Menagerie", edit("Menagerie", ("add", ["chemistry"], hello(when={"species": "ghost", "state": "haunting"}, then=[{"on": "when", "state": "fading", "trick": "boo"}]))), [("/chemistry/0/when/species", "unknown-reference")], None),
        ("reaction-near-unknown-species", "Menagerie", edit("Menagerie", ("add", ["chemistry"], hello(near={"species": "ghost"}))), [("/chemistry/0/near/species", "unknown-reference")], None),
        ("reaction-in-unknown-state", "Menagerie", edit("Menagerie", ("add", ["chemistry"], hello(when={"species": "mini", "state": "ghost"}))), [("/chemistry/0/when/state", "unknown-reference")], None),
        ("effect-into-unknown-state", "Menagerie", edit("Menagerie", ("add", ["chemistry"], hello(then=[{"on": "near", "state": "ghost"}]))), [("/chemistry/0/then/0/state", "unknown-reference")], None),
        ("effect-with-unknown-trick", "Menagerie", edit("Menagerie", ("add", ["chemistry"], hello(then=[{"on": "when", "trick": "bow"}]))), [("/chemistry/0/then/0/trick", "unknown-reference")], None),
        (
            "effect-with-the-state-and-trick-of-the-other-side",
            "Menagerie",
            edit("Menagerie", ("add", ["species", 0, "states"], state("lit", T("Lit", "Erleuchtet"))), ("add", ["species", 0, "tricks"], bow), ("add", ["chemistry"], hello(then=[{"on": "when", "state": "lit", "trick": "bow"}, {"on": "near", "state": "lit", "trick": "bow"}]))),
            [("/chemistry/0/then/1/state", "unknown-reference"), ("/chemistry/0/then/1/trick", "unknown-reference")],
            None,
        ),
        (
            "reaction-in-a-state-of-a-repeated-species",
            "Menagerie",
            edit("Menagerie", ("set", ["species", 1, "id"], "mini"), ("add", ["species", 1, "states"], state("lit", T("Lit", "Erleuchtet"))), ("add", ["chemistry"], hello(when={"species": "mini", "state": "lit"}, near={"species": "mini", "state": "resting"}))),
            [("/bonds/0/between/1", "unknown-reference"), ("/casts/0/rotation/0", "unknown-reference"), ("/chemistry/0/when/state", "unknown-reference"), ("/species/1/id", "duplicate-id")],
            None,
        ),
        ("reaction-at-no-distance", "Menagerie", edit("Menagerie", ("add", ["chemistry"], hello(within=0))), [("/chemistry/0/within", "out-of-range")], ("exclusiveMinimum", "/chemistry/0/within")),
        ("reaction-all-the-time", "Ensemble", edit("Ensemble", ("add", ["chemistry"], hello(every=0))), [("/chemistry/0/every", "out-of-range")], ("exclusiveMinimum", "/chemistry/0/every")),
        ("reaction-more-than-certain", "Menagerie", edit("Menagerie", ("add", ["chemistry"], hello(chance=1.5))), [("/chemistry/0/chance", "out-of-range")], ("maximum", "/chemistry/0/chance")),
        ("effect-of-negative-amount", "Menagerie", edit("Menagerie", ("add", ["chemistry"], hello(then=[{"on": "near", "mood": "sad", "amount": -0.1}]))), [("/chemistry/0/then/0/amount", "out-of-range")], ("minimum", "/chemistry/0/then/0/amount")),
        ("effect-of-overdriven-rapport", "Ensemble", edit("Ensemble", ("add", ["chemistry"], hello(then=[{"on": "near", "rapport": 2}]))), [("/chemistry/0/then/0/rapport", "out-of-range")], ("maximum", "/chemistry/0/then/0/rapport")),
        ("reaction-without-effects", "Menagerie", edit("Menagerie", ("add", ["chemistry"], hello(then=[]))), [("/chemistry/0/then", "items-too-few")], ("minItems", "/chemistry/0/then")),
        ("state-held-for-no-time", "Menagerie", edit("Menagerie", ("add", ["chemistry"], hello(when={"species": "mini", "state": "resting", "held": 0}))), [("/chemistry/0/when/held", "out-of-range")], ("exclusiveMinimum", "/chemistry/0/when/held")),
        ("trait-performing-an-unknown-trick", "Menagerie", edit("Menagerie", ("add", ["chemistry"], hello(near={"species": "midi", "trick": "juggle"}))), [("/chemistry/0/near/trick", "unknown-reference")], None),
        ("anyone-in-a-state-nobody-has", "Menagerie", edit("Menagerie", ("add", ["chemistry"], hello(when={"state": "haunting"}, then=[{"on": "when", "state": "fading"}]))), [("/chemistry/0/then/0/state", "unknown-reference"), ("/chemistry/0/when/state", "unknown-reference")], None),
        (
            "anyone-performing-a-trick-somebody-has",
            "Menagerie",
            edit("Menagerie", ("add", ["species", 0, "tricks"], bow), ("add", ["chemistry"], hello(when={"trick": "bow"}, unless={"trick": "curtsey"}, then=[{"on": "when", "trick": "bow"}, {"on": "near", "trick": "bow"}]))),
            [("/chemistry/0/then/1/trick", "unknown-reference"), ("/chemistry/0/unless/trick", "unknown-reference")],
            None,
        ),
        ("unless-of-unknown-species", "Menagerie", edit("Menagerie", ("add", ["chemistry"], hello(unless={"species": "ghost"}))), [("/chemistry/0/unless/species", "unknown-reference")], None),
        ("affinity-the-wrong-way-round", "Menagerie", edit("Menagerie", ("add", ["chemistry"], hello(affinity=[0.5, -0.5]))), [("/chemistry/0/affinity", "out-of-range")], None),
        ("affinity-beyond-adoration", "Ensemble", edit("Ensemble", ("add", ["chemistry"], hello(affinity=[0, 1.5]))), [("/chemistry/0/affinity/1", "out-of-range")], ("maximum", "/chemistry/0/affinity/1")),
        ("affinity-with-one-bound", "Menagerie", edit("Menagerie", ("add", ["chemistry"], hello(affinity=[0.5]))), [("/chemistry/0/affinity", "length-invalid")], ("minItems", "/chemistry/0/affinity")),
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
