"""🖼️ Writes the icon of every item, category and dimension into the four energy quizzes, keeping their formatting.

Run from the repository root: `.venv/Scripts/python.exe <this file>`. Entries that already carry an icon are skipped.
"""

import io
import os
import re

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), *[".."] * 7))
ENERGY = os.path.join(ROOT, "🎓️teaching", "🏛️architecture", "⚡️energy")

ICONS = {
    "🧲️physics": {
        "power-or-energy": {
            "power": ("⚡", "pulse"),
            "energy": ("🔋", "bounce"),
            "tea-light": ("🕯️", "float"),
            "kettle": ("🫖", "bounce"),
            "resting-person": ("🧘", "pulse"),
            "pv-module-peak": ("🔆", "pulse"),
            "heating-load": ("🏠", "pulse"),
            "nuclear-unit": ("☢️", "spin"),
            "car-engine": ("🏎️", "sway"),
            "wallbox": ("🔌", "bounce"),
            "household-electricity": ("💡", "pulse"),
            "ev-battery": ("🚙", "bounce"),
            "heating-oil-litre": ("🛢️", "sway"),
            "heating-demand-house": ("🏚️", "sway"),
            "chocolate-bar": ("🍫", "flip"),
            "pv-annual-yield": ("🌤️", "float"),
            "phone-charge": ("📱", "bounce"),
            "energy-certificate": ("📜", "sway"),
        },
        "powers": {
            "tea-light": ("🕯️", "float"),
            "resting-person": ("🧘", "pulse"),
            "sunlight-square-metre": ("🔆", "pulse"),
            "kettle": ("🫖", "bounce"),
            "wallbox": ("🔌", "bounce"),
            "heating-load-old-house": ("🏚️", "sway"),
            "car-engine": ("🏎️", "sway"),
            "wind-turbine": ("💨", "float"),
            "ice-train": ("🚄", "bounce"),
            "nuclear-unit": ("☢️", "spin"),
            "germany-electricity": ("🏙️", "pulse"),
            "world-primary-power": ("🌍", "spin"),
            "sunlight-on-earth": ("🌅", "float"),
            "sun": ("☀️", "spin"),
        },
        "energies": {
            "phone-charge": ("📱", "bounce"),
            "boil-water": ("♨️", "pulse"),
            "chocolate-bar": ("🍫", "flip"),
            "daily-food": ("🍽️", "sway"),
            "heating-oil-litre": ("🛢️", "sway"),
            "ev-battery": ("🚙", "bounce"),
            "petrol-tank": ("⛽", "pulse"),
            "household-electricity": ("💡", "pulse"),
            "heating-demand-house": ("🏚️", "sway"),
            "wind-turbine-year": ("💨", "float"),
            "germany-primary-energy": ("🏙️", "pulse"),
            "world-primary-energy": ("🌍", "spin"),
        },
    },
    "🔥️heating": {
        "u-values": {
            "u-value": ("🌡️", "pulse"),
            "single-glazing": ("🪟", "sway"),
            "aluminium-window-1970s": ("🪟", "flip"),
            "roller-shutter-box": ("📦", "bounce"),
            "box-type-window": ("🖼️", "sway"),
            "solid-roof-1950s": ("🏚️", "sway"),
            "front-door-geg": ("🚪", "sway"),
            "half-timbered-wall": ("🪵", "bounce"),
            "window-geg": ("🪟", "pulse"),
            "hollow-brick-wall-1970s": ("🧱", "flip"),
            "window-passive-house": ("🪟", "float"),
            "masonry-wall-1980s": ("🧱", "bounce"),
            "triple-glazing": ("🪟", "bounce"),
            "floor-geg": ("🪨", "pulse"),
            "wall-geg": ("🧱", "pulse"),
            "roof-geg": ("🏠", "float"),
            "wall-passive-house": ("🧱", "sway"),
            "roof-passive-house": ("🏡", "float"),
        },
        "heating-load-and-demand": {
            "heating-load": ("🌡️", "pulse"),
            "heating-demand": ("📅", "flip"),
            "passive-house": ("🌱", "sway"),
            "gruenderzeit-retrofit": ("🛠️", "sway"),
            "kfw-40": ("🏡", "float"),
            "kfw-55": ("🏠", "pulse"),
            "geg-2024": ("🏗️", "sway"),
            "sfh-2000s": ("🏘️", "bounce"),
            "plattenbau": ("🏢", "bounce"),
            "sfh-1990s": ("🏠", "sway"),
            "gruenderzeit": ("🏛️", "pulse"),
            "sfh-1970s": ("🏚️", "sway"),
            "sfh-1960s": ("🏚️", "bounce"),
        },
    },
    "❄️cooling": {
        "air-change-rates": {
            "air-change-rate": ("🔄", "spin"),
            "warehouse": ("📦", "bounce"),
            "passive-house-dwelling": ("🌱", "sway"),
            "apartment": ("🏠", "pulse"),
            "sports-hall": ("🏀", "bounce"),
            "single-office": ("💼", "sway"),
            "residential-car-park": ("🚗", "bounce"),
            "cinema": ("🎬", "flip"),
            "classroom": ("🏫", "pulse"),
            "restaurant": ("🍽️", "sway"),
            "laboratory": ("🧪", "sway"),
            "operating-room": ("🩺", "sway"),
            "commercial-kitchen": ("🍳", "bounce"),
            "cleanroom-iso-7": ("💊", "flip"),
            "cleanroom-iso-6": ("🥼", "sway"),
            "cleanroom-iso-5": ("🔬", "pulse"),
        },
        "cooling-load-and-demand": {
            "cooling-load": ("🧊", "pulse"),
            "cooling-demand": ("📅", "flip"),
            "passive-house-home": ("🌱", "sway"),
            "new-home-geg": ("🏠", "pulse"),
            "school-new-build": ("🏫", "pulse"),
            "office-passive-house": ("🏢", "sway"),
            "office-geg-shading": ("⛱️", "sway"),
            "attic-flat": ("🔆", "pulse"),
            "hospital": ("🏥", "pulse"),
            "office-1970s": ("🏙️", "float"),
            "data-centre": ("🖥️", "pulse"),
        },
    },
    "📊️demand": {
        "standard-profiles": {
            "profile-a": ("🔺", "pulse"),
            "profile-b": ("⬛", "sway"),
            "profile-c": ("⚫", "bounce"),
            "profile-d": ("🔷", "float"),
            "profile-e": ("⭐", "spin"),
            "profile-f": ("🔻", "flip"),
            "unrenovated-old-building": ("🏚️", "sway"),
            "wschvo-1995": ("📜", "sway"),
            "enev-2014": ("🏠", "pulse"),
            "kfw-40": ("🏡", "float"),
            "passive-house": ("🌱", "sway"),
            "plus-energy-house": ("➕", "pulse"),
        },
        "final-energy": {
            "final-energy": ("🧾", "sway"),
            "kfw-40-heat-pump": ("🏡", "float"),
            "deep-retrofit-heat-pump": ("🛠️", "sway"),
            "passive-house-direct-electric": ("🌱", "sway"),
            "kfw-55-gas-solar": ("🌤️", "float"),
            "enev-2014-gas": ("🏠", "pulse"),
            "old-house-heat-pump": ("🌀", "spin"),
            "wschvo-1995-gas": ("📜", "sway"),
            "gruenderzeit-gas": ("🏛️", "pulse"),
            "old-house-gas": ("🏚️", "sway"),
        },
    },
}

TASK = re.compile(r'^      "id": "([a-z0-9-]+)",$')
ENTRY = re.compile(r'^          "id": "([a-z0-9-]+)",$')


def write_icons(quiz, tasks):
    """✍️ Inserts the icon line of every entry after its label, or after its id when it has no label."""
    path = os.path.join(ENERGY, quiz, "❓️quiz", "🔣️.json")
    lines = io.open(path, encoding="utf-8", newline="").read().split("\n")
    written, task, pending, used = [], None, None, set()
    for index, line in enumerate(lines):
        written.append(line)
        head = TASK.match(line)
        if head:
            task = head.group(1)
        entry = ENTRY.match(line)
        if entry:
            pending = entry.group(1) if entry.group(1) in tasks.get(task, {}) else None
        if entry and pending is not None:
            if not lines[index + 1].startswith('          "label"'):
                pending = insert(written, tasks[task], task, pending, lines[index + 1], used)
        elif pending is not None and line.startswith('          "label"'):
            pending = insert(written, tasks[task], task, pending, lines[index + 1], used)
    missing = {(name, entry) for name, entries in tasks.items() for entry in entries} - used
    assert not missing, (quiz, sorted(missing))
    io.open(path, "w", encoding="utf-8", newline="").write("\n".join(written))


def insert(written, icons, task, entry, following, used):
    """➕️ Appends the icon line of `entry` unless the entry already carries one."""
    emoji, motion = icons[entry]
    used.add((task, entry))
    if not following.startswith('          "icon"'):
        written.append('          "icon": { "emoji": "%s", "motion": "%s" },' % (emoji, motion))
    return None


for quiz, tasks in ICONS.items():
    write_icons(quiz, tasks)
