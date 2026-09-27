#!/usr/bin/env python3
"""🎨️ R10 item 4: assigns the hand-chosen docstring emoji of every `needs-emoji` definition of a comment-hoist dry run.

Each row of `CANDIDATES` (same order as the dry run's `needs-emoji` lines) lists fitting emoji, best first; the first one
not already opening a docstring in that file (nor picked for another definition of it) is written to
`comment-hoist-emoji.json` under the codemod's text key (`<path>\t<definition line text>\t<ordinal>`). Usage: python3 comment-hoist-pick.py <dry-run.txt>
"""
import json
import re
import sys

ROOT = "/Users/ueli/Documents/semio"
PICKS = f"{ROOT}/.tmp-ticket/wp-r10/comment-hoist-emoji.json"
CANDIDATES = [
    "🔐️ 🧮️", "🧩️ 🧵️ 📦️", "🚫️ 🧱️ 📏️", "👻️ 🫥️ ⭕️", "🔢️ 🧮️ 🤝️", "🔎️ 👀️", "✏️ 🛠️", "🫳️ 📤️", "🔍️ 👁️", "🖊️ 🔧️",
    "📤️ 🧺️ 🫴️ 🪝️", "⏹️ 🛑️", "🐕️ ⏰️", "📜️ 🧾️", "↩️ ⏪️", "🧯️ ⏮️", "🔁️ 🌀️", "🪢️ 🧶️ 🎏️", "👁️ 🔭️ 🪞️", "🔄️ ⏪️ 🔙️",
    "🫧️ 📣️ 🎈️", "✂️ 📋️ 🗒️", "🧬️ 🪡️ 🩹️", "⚖️ 🪢️ 🧷️ 🪡️", "🐚️ 📦️ 🥡️", "🔍️ 🗝️ 👀️", "📤️ 🫳️ 🫴️", "🗂️ 🔎️ 🧿️", "🖋️ 🛠️ ✍️", "🗑️ ➖️ 🪤️",
    "📇️ 🔎️ 📎️", "🫴️ 📤️ 🧤️", "🎥️ 📐️ 📷️", "💣️ 🧨️", "🕰️ ⏱️", "🧪️ 🗃️", "👻️ 📢️", "🔦️ 🚨️", "🚧️ 🧱️", "✂️ 🧹️",
    "🧼️ 📨️", "↩️ 💾️", "🔁️ 📝️", "📍️ 🏁️", "📁️ 📂️", "🎭️ 🚪️", "🔌️ 🤝️", "📬️ 🔁️", "🚿️ 📤️", "🎞️ ▶️",
    "🏃️ ⚙️", "🪐️ 🗺️ 🌌️ 🛸️", "⏱️ 🎞️", "🪟️ ✒️", "🧭️ 📍️", "🔚️ ⌨️", "🖱️ 📍️", "↕️ 📏️", "🎯️ 📥️ 🪂️ 🧲️", "↔️ 🪓️",
    "🌳️ ♻️", "🧰️ 🗃️", "⌨️ 🎹️", "🚦️ 🧷️", "🔀️ 💡️", "🛂️ 🔒️", "💤️ 🧊️", "💲️ ➗️", "⚪️ 🔵️", "☀️ 🌞️",
    "🏔️ 🌈️", "🛟️ 🧯️",
]


def docstring_emojis(path):
    used, previous = set(), False
    for line in open(path, encoding="utf-8").read().split("\n"):
        stripped = line.lstrip()
        doc = stripped.startswith("///") or stripped.startswith("//!")
        if doc and not previous:
            match = re.match(r"^(\S+)", stripped[3:].strip())
            if match:
                used.add(match.group(1))
        previous = doc
    return used


def main():
    rows = [line for line in open(sys.argv[1], encoding="utf-8") if line.startswith("needs-emoji ")]
    if len(rows) != len(CANDIDATES):
        sys.exit(f"{len(rows)} needs-emoji rows but {len(CANDIDATES)} candidate rows — re-review the list")
    picks = json.load(open(PICKS, encoding="utf-8"))
    used_by_file, fallback = {}, []
    for index, row in enumerate(rows):
        key = "\t".join(row[len("needs-emoji "):].split("\t")[:3])
        path = key.split("\t")[0]
        location = key
        used = used_by_file.setdefault(path, docstring_emojis(f"{ROOT}/{path}"))
        options = CANDIDATES[index].split()
        choice = next((option for option in options if option not in used), None)
        if choice is None:
            sys.exit(f"no unused candidate for {location}")
        if choice != options[0]:
            fallback.append(f"{location} {choice}")
        used.add(choice)
        picks[location] = choice
    json.dump(picks, open(PICKS, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    print(json.dumps({"assigned": len(rows), "total": len(picks), "secondChoice": fallback}, ensure_ascii=False))


main()
