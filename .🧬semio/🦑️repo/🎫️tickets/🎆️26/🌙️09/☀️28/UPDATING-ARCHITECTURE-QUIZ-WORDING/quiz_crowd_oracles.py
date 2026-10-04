"""🔮️ The oracle registry of the quiz after §17: lightningcss now reads the card layer's tracks (a track list or the
overview's variables, and the absence of spacing between tracks) and the overview's own track lists as fr weights;
lodash's throttle also judges the drafts of a thinking room (same drafts, never later); a new lodash entry judges the
crowd shown by the web client (`groupBy`/`orderBy` recompute the choices from the raw drafts and counts, `mean` the
positions, `round` the places) in the suite `🧪️tests/💭️crowd-client`. Exact anchors in the registry text."""

import pathlib
import sys
import time

path = pathlib.Path("C:/git/semio/🧰️framework/🛍️products/❓️quiz/🔮️oracles/🔣️.json")
text = path.read_text(encoding="utf-8")

OLD_CSS = "\"rationale\": \"Judges the home grid of @semio-tech/quiz-react inside its unit suite `🧪️tests/🏠️home-grid` (vitest, devDependency of the react package): lightningcss — the CSS engine Tailwind itself runs the site's styles through — parses the renderer's stylesheet into media queries and grid track lists, and the column count and leaderboard span it yields at 320 to 1440 px, and the breakpoints themselves, must equal the shared vectors and the design system's `UI_MOBILE_MAX_WIDTH_PX` / `UI_TABLET_MAX_WIDTH_PX`. jsdom lays nothing out and evaluates no media query, so the stylesheet's own parse is the independent reading; the site walk in a real browser confirms the same counts.\""
NEW_CSS = "\"rationale\": \"Judges the home grid of @semio-tech/quiz-react inside its unit suite `🧪️tests/🏠️home-grid` (vitest, devDependency of the react package): lightningcss — the CSS engine Tailwind itself runs the site's styles through — parses the renderer's stylesheet into media queries, grid track lists and variables, and the tracks the card layer takes at 320 to 1440 px (one column below tablets, the overview's `--layered-columns`/`--layered-rows` from there, no gap or padding between tracks, the spacing inside `.quiz-home-cell`) and the breakpoints themselves must equal the shared vectors and the design system's `UI_MOBILE_MAX_WIDTH_PX` / `UI_TABLET_MAX_WIDTH_PX`; it also parses the overview's own track lists into fr weights that must equal the shared weights of each layout (1 : 1.5 : 1 by 1 : 1.4 : 1 on desktops). jsdom lays nothing out and evaluates no media query (the width is emulated through `matchMedia`), so the stylesheet's own parse is the independent reading; the two-device walk in a real browser confirms the same grid.\""
OLD_THROTTLE = "Schedules that repeat a state are judged by the shared vectors alone, since the room never resends the state it sent last.\""
NEW_THROTTLE = "Schedules that repeat a state are judged by the shared vectors alone, since the room never resends the state it sent last. The drafts a thinking room sends (at most every 500 ms) must be the drafts lodash's throttle invokes with the same interval, each sent no later than lodash's (the room times from its last frame, lodash from its timer).\""
ENTRY_END = NEW_THROTTLE + "\n    }\n  ],\n  \"noOracleDecisions\""
CROWD = NEW_THROTTLE + """
    },
    {
      "id": "quiz-react-lodash-crowd",
      "kind": "third-party-library",
      "ecosystem": "javascript",
      "package": "lodash",
      "version": "4.18.1",
      "source": {
        "repository": "https://github.com/lodash/lodash",
        "license": "MIT"
      },
      "engine": {
        "family": "lodash",
        "implementation": "lodash/groupBy, lodash/orderBy, lodash/sortBy, lodash/mean and lodash/round",
        "version": "4.18.1"
      },
      "capabilities": [
        "quiz-react-crowd"
      ],
      "comparisonProfiles": [
        "ordered-json-v1"
      ],
      "license": "MIT",
      "testOnly": true,
      "productionReachable": false,
      "networkDuringExecution": false,
      "homepage": "https://lodash.com/docs/#orderBy",
      "hostPath": "🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/📦️packages/🟦️typescript",
      "rationale": "Judges what the others think as @semio-tech/quiz-react shows it, inside its unit suite `🧪️tests/💭️crowd-client` (vitest, devDependency of the react package): for every shared vector, the choices of a live item recomputed by lodash straight from the others' raw drafts (grouped by category or value, tags sorted, ordered by count descending, then numerically, then by code point) and the choices of a submitted item reordered by lodash from the crowd view's code point ordered counts must equal the client's; the mean of a live item's positions must equal lodash's mean, and every place lodash's rounding to one decimal of `1 + position × (total − 1)`."
    }
  ],
  "noOracleDecisions\""""
for old, new in ((OLD_CSS, NEW_CSS), (OLD_THROTTLE, NEW_THROTTLE), (ENTRY_END, CROWD)):
    if text.count(old) != 1:
        sys.exit(f"anchor found {text.count(old)} times: {old[:80]!r}")
    text = text.replace(old, new)
for attempt in range(40):
    try:
        path.write_text(text, encoding="utf-8", newline="")
        break
    except OSError:
        time.sleep(0.5)
else:
    sys.exit("write failed")
print("[crowd oracles] done")
