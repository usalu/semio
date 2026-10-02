"""Every launch row a teaching, quiz or pets README names must exist in `.vscode/launch.json` and its seed.

    python launch_rows_in_docs.py
"""
import io
import json
import pathlib
import re
import sys

sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8")
ROOT = pathlib.Path(__file__).resolve().parents[7]
DOCS = [
    "🎓️teaching/README.md",
    "🎓️teaching/🏛️architecture/README.md",
    "🎓️teaching/🏛️architecture/❓️quiz/README.md",
    "🎓️teaching/🏛️architecture/🐾️pets/README.md",
    "🎓️teaching/🛂️proctor/README.md",
    "🧰️framework/🛍️products/❓️quiz/README.md",
    "🧰️framework/🛍️products/🐾️pets/README.md",
]
VERBS = "🛠️dev|🧪️test|⚖️gate|✅️check|🔁️rebuild|🚚️publish|📦️build|📦️bundle|🩺️health|💾️backup|♻️restore|🧨️erase|🧹️prune"
ROW = re.compile(rf"`((?:{VERBS})[^`\s]*)`")


def names(path: str) -> set[str]:
    text = (ROOT / path).read_text(encoding="utf-8")
    return set(re.findall(r'"name"\s*:\s*"([^"]+)"', text))


launch, seed = names(".vscode/launch.json"), names(".vscode/🧩️launch.seed.jsonc")
missing = 0
for doc in DOCS:
    file = ROOT / doc
    if not file.exists():
        continue
    for row in sorted(set(ROW.findall(file.read_text(encoding="utf-8")))):
        if "…" in row or "/" in row:
            continue
        where = [name for name, rows in (("launch.json", launch), ("seed", seed)) if row not in rows]
        if where:
            missing += 1
            print(f"{doc}: {row} missing in {', '.join(where)}")
print(f"missing={missing}")
sys.exit(1 if missing else 0)
