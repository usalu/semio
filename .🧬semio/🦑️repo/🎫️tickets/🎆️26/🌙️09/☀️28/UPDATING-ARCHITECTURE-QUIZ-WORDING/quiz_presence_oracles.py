"""🔮️ Registers the two oracles of the presence round (design §15): lodash's `throttle` for the quiz client's frame
throttle and d3-color for the ui presence palette paint — appended textually to each registry's `oracles` list."""

import json
import pathlib
import sys

ROOT = pathlib.Path("C:/git/semio")
ENTRIES = {
    "🧰️framework/🛍️products/❓️quiz/🔮️oracles/🔣️.json": {
        "id": "quiz-react-lodash-throttle",
        "kind": "third-party-library",
        "ecosystem": "javascript",
        "package": "lodash",
        "version": "4.18.1",
        "source": {"repository": "https://github.com/lodash/lodash", "license": "MIT"},
        "engine": {"family": "lodash", "implementation": "lodash/throttle with leading and trailing edges", "version": "4.18.1"},
        "capabilities": ["quiz-react-presence-throttle"],
        "comparisonProfiles": ["ordered-json-v1"],
        "license": "MIT",
        "testOnly": True,
        "productionReachable": False,
        "networkDuringExecution": False,
        "homepage": "https://lodash.com/docs/#throttle",
        "hostPath": "🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/📦️packages/🟦️typescript",
        "rationale": "Judges the presence frame throttle of @semio-tech/quiz-react inside its unit suite `🧪️tests/📡️presence-client` (vitest, devDependency of the react package): for every shared call schedule whose states are pairwise distinct, the frames a presence room sends (time and state, under fake timers) must equal the invocations of lodash's throttle with the same interval — the reference leading-and-trailing-edge throttle. Schedules that repeat a state are judged by the shared vectors alone, since the room never resends the state it sent last.",
    },
    "🧰️framework/🔨️modules/🖱️ui/🔮️oracles/🔣️.json": {
        "id": "d3-color",
        "kind": "third-party-library",
        "ecosystem": "javascript",
        "package": "d3-color",
        "version": "3.1.0",
        "source": {"repository": "https://github.com/d3/d3-color", "license": "ISC"},
        "engine": {"family": "d3-color", "implementation": "d3-color hsl().formatHex()", "version": "3.1.0"},
        "capabilities": ["presence-paint"],
        "comparisonProfiles": ["ordered-json-v1"],
        "license": "ISC",
        "testOnly": True,
        "productionReachable": False,
        "networkDuringExecution": False,
        "homepage": "https://d3js.org/d3-color",
        "hostPath": "🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript",
        "rationale": "Judges `presencePaint` of 👥️presence-presentation in `🧪️tests/🔬️unit`: the generated `--presence-N` palette variables (light and dark) and the inline literals past the first cycle must convert to the same sRGB hex as `presenceColor`'s HSL, both converted by d3-color — so every palette slot paints one colour whichever path renders it.",
    },
}
for relative, entry in ENTRIES.items():
    path = ROOT / relative
    text = path.read_text(encoding="utf-8")
    if f'"id": "{entry["id"]}"' in text:
        continue
    anchor = "\n  ],\n  \"noOracleDecisions\""
    if text.count(anchor) != 1:
        sys.exit(f"{relative}: anchor found {text.count(anchor)} times")
    block = "\n".join("    " + line for line in json.dumps(entry, ensure_ascii=False, indent=2).splitlines())
    text = text.replace(anchor, ",\n" + block + anchor)
    json.loads(text)
    path.write_text(text, encoding="utf-8", newline="")
print("[presence oracles] done")
