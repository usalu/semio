"""👀️ Shared vectors of what home watches (design §17) in the presence-client fixture: for a catalog, its quizzes and the
room home joined, the rooms watched — the introduction, leaderboard and badges pages, every quiz's page, every quiz's
thinking room — without the joined one, at most 16. Appends `watchScopes` after `cursors`; the rest is untouched."""

import pathlib
import sys
import time

path = pathlib.Path("C:/git/semio/🧰️framework/🛍️products/❓️quiz/🧫️fixtures/📡️presence-client/🔣️.json")
text = path.read_text(encoding="utf-8")
TAIL = '    { "id": "no-box", "box": { "left": 10, "top": 10, "width": 0, "height": 20 }, "point": [10, 15], "expected": null }\n  ]\n}'
if text.rstrip().count(TAIL) != 1 or not text.rstrip().endswith(TAIL):
    sys.exit("tail anchor")


def scopes(catalog: str, quizzes: list[str], joined: str | None) -> list[str]:
    rooms = [f"{catalog}/{page}" for page in ("introduction", "leaderboard", "badges")] + [f"{catalog}/quiz/{quiz}" for quiz in quizzes] + [f"{catalog}/quiz/{quiz}/thinking" for quiz in quizzes]
    return [scope for scope in rooms if scope != joined][:16]


VECTORS = [
    ("four-quizzes-from-home", "arch", ["physics", "heating", "cooling", "demand"], "arch/home"),
    ("no-quizzes", "arch", [], "arch/home"),
    ("joined-room-left-out", "arch", ["physics"], "arch/leaderboard"),
    ("at-most-sixteen", "arch", ["q1", "q2", "q3", "q4", "q5", "q6", "q7"], "arch/home"),
]


def json_list(values: list[str]) -> str:
    return "[" + ", ".join(f'"{value}"' for value in values) + "]"


entries = ",\n".join(
    f'    {{ "id": "{name}", "catalog": "{catalog}", "quizzes": {json_list(quizzes)}, "joined": "{joined}", "expected": {json_list(scopes(catalog, quizzes, joined))} }}'
    for name, catalog, quizzes, joined in VECTORS
)
text = text.rstrip()[: -len("\n  ]\n}")] + "\n  ],\n  \"watchScopes\": [\n" + entries + "\n  ]\n}\n"
for attempt in range(40):
    try:
        path.write_text(text, encoding="utf-8", newline="")
        break
    except OSError:
        time.sleep(0.5)
else:
    sys.exit("write failed")
print("[watch fixture] done")
