#!/usr/bin/env python3
"""Round 2 slice T-A2: prompt_toolkit as an independent decoder for the input decoding fixture.

  verify   every fixture row tagged "prompt_toolkit" must decode, by prompt_toolkit's Vt100Parser, to the fixture events
  suggest  list the untagged key rows prompt_toolkit already agrees with (to extend the tags)

Capability rows tagged "terminfo" are checked against `infocmp -x -1 $TERM`: RGB -> TrueColor, colors >= 256 -> Ansi256,
colors 8 or 16 -> Ansi16, none -> Monochrome.

Run with the Python that has prompt_toolkit (3.0.52 here): python3 -X utf8 r2-ta-oracle.py verify
"""
import json
import os
import re
import subprocess
import sys

from prompt_toolkit.input.vt100_parser import Vt100Parser
from prompt_toolkit.keys import Keys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = HERE
while not os.path.isdir(os.path.join(ROOT, ".git")):
    ROOT = os.path.dirname(ROOT)
FIXTURE = os.path.join(ROOT, "🧰️framework", "🔨️modules", "🖱️ui", "⌨️tui", "🧫️fixtures", "⌨️input-decoding", "🔣️.json")

NAMES = {
    "up": "Up", "down": "Down", "left": "Left", "right": "Right", "home": "Home", "end": "End", "pageup": "PageUp",
    "pagedown": "PageDown", "insert": "Insert", "delete": "Delete", "backspace": "Backspace", "enter": "Enter", "tab": "Tab",
    "s-tab": "BackTab", "escape": "Esc",
}
CTRL_ARROWS = {"c-": "+ctrl", "s-": "+shift", "c-s-": "+ctrl+shift"}


def project(press):
    key = press.key
    value = key.value if isinstance(key, Keys) else key
    if key == Keys.Any:
        text = press.data
        return "key:" + {" ": "Space", "+": "Plus"}.get(text, text)
    if value in NAMES:
        return "key:" + NAMES[value]
    if value.startswith("f") and value[1:].isdigit():
        return "key:F" + value[1:]
    for prefix, suffix in CTRL_ARROWS.items():
        if value.startswith(prefix) and value[len(prefix):] in NAMES:
            return "key:" + NAMES[value[len(prefix):]] + suffix
    if value in ("c-m", "c-i", "c-h"):
        return "key:" + {"c-m": "Enter", "c-i": "Tab", "c-h": "Backspace"}[value]
    if value.startswith("c-") and len(value) == 3 and value[2].isalpha():
        return "key:" + value[2] + "+ctrl"
    if value == "c-@":
        return "key:Space+ctrl"
    return "key:?" + value


def decode(data):
    presses = []
    parser = Vt100Parser(presses.append)
    parser.feed(data)
    parser.flush()
    return [project(press) for press in presses]


def terminfo_depth(term):
    text = subprocess.run(["infocmp", "-x", "-1", term], capture_output=True, text=True, check=True).stdout
    if re.search(r"^\s*RGB,", text, re.M):
        return "TrueColor"
    match = re.search(r"^\s*colors#(0x[0-9a-f]+|\d+),", text, re.M)
    if not match:
        return "Monochrome"
    return "Ansi256" if int(match.group(1), 0) >= 256 else "Ansi16"


def verify_capabilities(fixture):
    agreed, failed = 0, []
    for row in fixture["capabilities"]:
        if "terminfo" in row.get("oracle", []):
            got = terminfo_depth(row["env"]["TERM"])
            if got == row["color"]:
                agreed += 1
            else:
                failed.append((row["id"], got, row["color"]))
    return agreed, failed


def main():
    mode = sys.argv[1] if len(sys.argv) > 1 else "verify"
    with open(FIXTURE, encoding="utf-8") as handle:
        fixture = json.load(handle)
    agreed, failed = 0, []
    for row in fixture["decoding"]:
        tagged = "prompt_toolkit" in row.get("oracle", [])
        events = row["events"]
        if not events or not all(e.startswith("key:") for e in events):
            continue
        got = decode(row["input"])
        if mode == "suggest":
            if not tagged and got == events:
                print("agrees", row["id"])
            continue
        if tagged:
            if got == events:
                agreed += 1
            else:
                failed.append((row["id"], got, events))
    if mode == "verify":
        capability_agreed, capability_failed = verify_capabilities(fixture)
        for identifier, got, expected in capability_failed:
            print("TERMINFO DISAGREES", identifier, got, expected)
        print("terminfo reproduces", capability_agreed, "tagged capability rows,", len(capability_failed), "disagree")
        failed += capability_failed
        agreed += capability_agreed
        for identifier, got, events in failed:
            print("DISAGREES", identifier, got, events)
        print("prompt_toolkit decodes", agreed, "tagged rows,", len(failed), "disagree")
        return 1 if failed or agreed == 0 else 0
    return 0


sys.exit(main())
