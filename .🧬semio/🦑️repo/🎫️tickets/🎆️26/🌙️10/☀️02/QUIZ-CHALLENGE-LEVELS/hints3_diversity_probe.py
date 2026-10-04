#!/usr/bin/env python3
"""🔬️ Searches sorting orders and matching assignments of the generator's tasks whose hints show each branch of the reference-diversity rule (design §8.4b item 3): a later hint takes an unused reference over a smaller claim or a familiar one, a reference is reused when the tie window holds nothing else, a dropped hint does not use up a reference, and the same reference serves two dimensions.

Run from the repository root: ``.venv/Scripts/python.exe <this file>``.
"""

import glob
import importlib.util
import itertools
import os
import sys

sys.dont_write_bytecode = True
GENERATOR = glob.glob(os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "..", "..", "🌙️09", "☀️28", "*", "generate_quiz_vectors.py"))[0]
spec = importlib.util.spec_from_file_location("generate_quiz_vectors", GENERATOR)
G = importlib.util.module_from_spec(spec)
spec.loader.exec_module(G)
CH = G.CH
original = CH.referenced


def blind(kept):
    """🙈️ The references chosen without the diversity rule (familiar, smallest claim, sheet order)."""
    hints = []
    for hint in kept:
        if "tied" not in hint:
            hints.append(hint)
            continue
        other, _, found, claim, _ = min(hint["tied"], key=lambda entry: (not entry[4], CH.oriented(entry[3])))
        hints.append({"kind": "compare", "item": hint["item"], "other": other, **({} if hint["dimension"] is None else {"dimension": hint["dimension"]}), **claim, "verdict": found})
    return hints


def before_cap(task, sheet_task, answer):
    """⏮️ The hints when references are chosen over every hint before the cap."""
    captured = []
    CH.capped, saved = (lambda weighted: captured.extend(weighted) or []), CH.capped
    try:
        CH.hints_of(task, sheet_task, answer)
    finally:
        CH.capped = saved
    chosen = original([hint for _, hint in captured])
    ranked = sorted(range(len(captured)), key=lambda index: (captured[index][0] is None, 0 if captured[index][0] is None else -captured[index][0], index))
    kept = set(ranked[:CH.HINTS_PER_TASK])
    return [hint for index, hint in enumerate(chosen) if index in kept]


def variants(task, sheet_task, answer):
    """🔀️ The hints now, without diversity and with references chosen before the cap."""
    now = CH.hints_of(task, sheet_task, answer)
    CH.referenced = blind
    try:
        plain = CH.hints_of(task, sheet_task, answer)
    finally:
        CH.referenced = original
    return now, plain, before_cap(task, sheet_task, answer)


def describe(hints):
    """📝️ item→other per hint."""
    return ", ".join("%s%s→%s" % (hint["item"], "/" + hint["dimension"] if hint.get("dimension") else "", hint.get("other")) for hint in hints)


def sortings(label, task, sheet_task, limit=4):
    """📶️ Every order of a sorting, grouped by which variant differs."""
    found = {"diverse": [], "early": []}
    for order in itertools.permutations(G.ascending_ids(task, sheet_task)):
        answer = {"kind": "sorting", "order": list(order)}
        now, plain, early = variants(task, sheet_task, answer)
        if now != plain:
            found["diverse"].append((order, now, plain))
        if now != early:
            found["early"].append((order, now, early))
    for kind, entries in found.items():
        print("[DEBUG] %s %s: %d orders" % (label, kind, len(entries)))
        for order, now, other in entries[:limit]:
            print("[DEBUG]   %s\n           now   %s\n           other %s" % (list(order), describe(now), describe(other)))


power = G.sheet_sorting(G.POWER, ["nuclear-plant", "led-bulb", "kettle", "hair-dryer", "wallbox", "tea-light"])
def merged(kept):
    """🌐️ The references chosen with one set of used references for all dimensions."""
    return [{**hint, **({"dimension": hint["dimension"]} if "dimension" in hint else {})} for hint in original([{**hint, "dimension": None} if "tied" in hint else hint for hint in kept])]


if sys.argv[1:] == ["across"]:
    carriers = {"kind": "matching", "id": "energy-carriers", "title": G.CARRIERS["title"], "prompt": G.CARRIERS["prompt"], "dimensions": [{"id": "energy-density", "quantity": G.CARRIERS["dimensions"][0]["quantity"], "cards": [18, 42.6, 120, 42.6, 29]}, {"id": "co2-factor", "quantity": G.CARRIERS["dimensions"][1]["quantity"], "cards": [0.27, 0.34, 0, 0.027, 0.27]}], "items": [{"id": identifier, "label": identifier} for identifier in ["diesel", "wood-pellets", "coal", "hydrogen", "heating-oil"]]}
    ids = [item["id"] for item in carriers["items"]]
    shown = 0
    for density in itertools.permutations(range(5)):
        for co2 in itertools.permutations(range(5)):
            answer = {"kind": "matching", "assignments": {"energy-density": dict(zip(ids, density)), "co2-factor": dict(zip(ids, co2))}}
            now = CH.hints_of(G.CARRIERS, carriers, answer)
            CH.referenced = merged
            try:
                shared = CH.hints_of(G.CARRIERS, carriers, answer)
            finally:
                CH.referenced = original
            if [{key: value for key, value in hint.items() if key != "dimension"} for hint in now] != [{key: value for key, value in hint.items() if key != "dimension"} for hint in shared] and shown < 8:
                shown += 1
                print("[DEBUG] carriers %s %s\n           now    %s\n           shared %s" % (density, co2, describe(now), describe(shared)))
    sys.exit(0)
if sys.argv[1:] == ["early"]:
    sortings("power-all-eight", G.POWER, G.sheet_sorting(G.POWER, ["nuclear-plant", "led-bulb", "kettle", "hair-dryer", "wallbox", "tea-light", "laptop", "wind-turbine"]), 8)
    sys.exit(0)
sortings("power", G.POWER, power)
sortings("familiar-power", G.FAMILIAR_POWER, G.sheet_sorting(G.FAMILIAR_POWER, ["nuclear-plant", "led-bulb", "kettle", "hair-dryer", "wallbox", "tea-light"]))
sortings("power-with-laptop", G.POWER, G.sheet_sorting(G.POWER, ["nuclear-plant", "led-bulb", "kettle", "laptop", "wallbox", "tea-light"]))
for answer in [{"kind": "sorting", "order": ["wallbox", "nuclear-plant", "led-bulb", "kettle", "tea-light", "hair-dryer"]}]:
    print("[DEBUG] reuse %s" % describe(CH.hints_of(G.POWER, power, answer)))
carriers = G.CH and {"kind": "matching", "id": "energy-carriers", "title": G.CARRIERS["title"], "prompt": G.CARRIERS["prompt"], "dimensions": [{"id": "energy-density", "quantity": G.CARRIERS["dimensions"][0]["quantity"], "cards": [18, 42.6, 120, 42.6, 29]}, {"id": "co2-factor", "quantity": G.CARRIERS["dimensions"][1]["quantity"], "cards": [0.27, 0.34, 0, 0.027, 0.27]}], "items": [{"id": identifier, "label": identifier} for identifier in ["diesel", "wood-pellets", "coal", "hydrogen", "heating-oil"]]}
ids = [item["id"] for item in carriers["items"]]
shown = 0
for density in itertools.permutations(range(5)):
    for co2 in itertools.permutations(range(5)):
        answer = {"kind": "matching", "assignments": {"energy-density": dict(zip(ids, density)), "co2-factor": dict(zip(ids, co2))}}
        now, plain, early = variants(G.CARRIERS, carriers, answer)
        dims = {}
        for hint in now:
            dims.setdefault(hint["dimension"], []).append(hint["other"])
        across = len(dims) == 2 and set(dims["energy-density"]) & set(dims["co2-factor"])
        if now != plain and across and shown < 6:
            shown += 1
            print("[DEBUG] carriers %s %s\n           now   %s\n           plain %s" % (density, co2, describe(now), describe(plain)))
