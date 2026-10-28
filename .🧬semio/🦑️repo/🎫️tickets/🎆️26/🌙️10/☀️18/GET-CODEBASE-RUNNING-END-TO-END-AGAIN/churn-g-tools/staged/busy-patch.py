#!/usr/bin/env python3
"""Third batch: a machine that ran out of file handles (EMFILE/ENFILE, seen 08:2x when the fleet overflowed the system file table and a run died inside the classifier) is a transient `busy` condition of a read, announced with its own reason in EN/DE; Windows sharing violations use the same reason. usage: busy-patch.py (from the repo root)"""
import json
import sys

O = "🧰️framework/🔨️modules/📁️filesystem/🧾️observation/🔁️churn/"


def edit(path, pairs):
    text = open(path, encoding="utf-8").read()
    for old, new in pairs:
        if new in text and old not in text:
            continue
        assert text.count(old) == 1, (path, old[:80], text.count(old))
        text = text.replace(old, new)
    open(path, "w", encoding="utf-8").write(text)


edit(O + "🧬️schema/🔣️.json", [
    ('"Reason": {\n      "enum": ["bytes", "identity", "vanished", "appeared", "roster", "length", "ancestry", "output", "descriptor", "generation", "input"]', '"Reason": {\n      "enum": ["bytes", "identity", "vanished", "appeared", "roster", "length", "ancestry", "output", "descriptor", "generation", "input", "busy"]'),
    ('            "input": "one of its inputs changed"\n', '            "input": "one of its inputs changed",\n            "busy": "it was busy or the machine ran out of file handles"\n'),
    ('            "input": "eine ihrer Eingaben änderte sich"\n', '            "input": "eine ihrer Eingaben änderte sich",\n            "busy": "sie war belegt oder der Maschine gingen die Dateihandles aus"\n'),
])
edit(O + "🟦️.ts", [('"generation"|"input";', '"generation"|"input"|"busy";')])
races = json.load(open(O + "🧬️schema/🌊️races.json", encoding="utf-8"))
table = races["definitions"]["Table"]["const"]
table["codes"]["all"].update({"EMFILE": "busy", "ENFILE": "busy"})
table["reasons"]["busy"] = "busy"
open(O + "🧬️schema/🌊️races.json", "w", encoding="utf-8").write(json.dumps(races, indent=2, ensure_ascii=False) + "\n")
edit(O + "🌊️races/🟦️.ts", [('export type RaceReasonV1="vanished"|"identity";', 'export type RaceReasonV1="vanished"|"identity"|"busy";')])
print("busy reason landed")
