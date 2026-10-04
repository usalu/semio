"""📡️ Renames the react presence suite `👥️presence-client` to `📡️presence-client` (its 👥️ repeated the sibling conformance
case `👥️shared-presence`; sibling identities are unique emojis) in tests and fixtures, updates every reference (the react
test config, the oracle registry, the suite's own imports, the ticket scripts) and registers the names of this round in
the taxonomy: the test/fixture member `📡️presence-client` replaces `👥️presence-client`, the react module `🗳️crowd` joins
the module members beside `📖️quiz-page`, and the new react suite `💭️crowd-client` (tests and fixtures) joins beside
`📡️presence-client`. Exact anchors; each replacement must find its anchor."""

import pathlib
import sys
import time

ROOT = pathlib.Path("C:/git/semio")
QUIZ = ROOT / "🧰️framework/🛍️products/❓️quiz"
TICKET = ROOT / ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️28/QUIZ-PRODUCT-AND-TEACHING-PROCTOR"
TAXONOMY = ROOT / "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"
OLD, NEW = "👥️presence-client", "📡️presence-client"


def write(path: pathlib.Path, text: str) -> None:
    for attempt in range(40):
        try:
            path.write_text(text, encoding="utf-8", newline="")
            return
        except OSError:
            time.sleep(0.5)
    sys.exit(f"{path}: write failed")


def replace(path: pathlib.Path, old: str, new: str, count: int | None = None) -> None:
    text = path.read_text(encoding="utf-8")
    found = text.count(old)
    if found == 0 or (count is not None and found != count):
        sys.exit(f"{path}: anchor found {found} times: {old[:80]!r}")
    write(path, text.replace(old, new))


for folder in ("🧪️tests", "🧫️fixtures"):
    source, target = QUIZ / folder / OLD, QUIZ / folder / NEW
    if target.exists():
        sys.exit(f"{target} exists")
    source.rename(target)

replace(QUIZ / "🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts", f"🧪️tests/{OLD}/", f"🧪️tests/{NEW}/", 1)
replace(QUIZ / "🔮️oracles/🔣️.json", f"`🧪️tests/{OLD}`", f"`🧪️tests/{NEW}`", 1)
replace(QUIZ / "🧪️tests" / NEW / "🟦️.tsx", f"🧫️fixtures/{OLD}/", f"🧫️fixtures/{NEW}/", 2)
for script in ("quiz_presence_oracles.py", "quiz_react_mutation_checks.py"):
    replace(TICKET / script, OLD, NEW)
replace(TAXONOMY, f'"{OLD}",', f'"{NEW}",', 2)
replace(TAXONOMY, '        "📖️quiz-page",\n', '        "📖️quiz-page",\n        "🗳️crowd",\n', 1)
print("[presence client rename] done")

replace(TAXONOMY, '        "📡️presence-client",\n        "📊️crowd-view"\n', '        "📡️presence-client",\n        "💭️crowd-client",\n        "📊️crowd-view"\n', 2)
print("[crowd client registration] done")
