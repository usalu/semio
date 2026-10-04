"""📦️ The package entry of `@semio-tech/quiz-react` re-exports what §17 added: the grid tracks of home, the watch and
thinking constants and helpers of presence, the others inside a page, and the crowd module. Exact anchors."""

import pathlib
import sys
import time

path = pathlib.Path("C:/git/semio/🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🟦️.tsx")
text = path.read_text(encoding="utf-8")
EDITS = [
    ('export { HomeScreen, homeCells, homePages } from "./🔨️modules/🏠️home/🟦️.tsx";\n',
     'export { HOME_GRID_TRACKS, HomeScreen, homeCells, homePages } from "./🔨️modules/🏠️home/🟦️.tsx";\n'),
    ("  EMPTY_PRESENCE_VIEW,\n  OnlineMark,\n  PRESENCE_ANCHORS,\n  PRESENCE_FRAME_HZ,\n  PRESENCE_FRAME_INTERVAL_MS,\n  PresenceList,\n",
     "  EMPTY_PRESENCE_VIEW,\n  MAX_WATCHED_ROOMS,\n  OnlineMark,\n  PRESENCE_ANCHORS,\n  PRESENCE_FRAME_HZ,\n  PRESENCE_FRAME_INTERVAL_MS,\n  PanePeers,\n  PresenceList,\n"),
    ("  QuizPresence,\n  browserPresenceConnect,\n  cursorAt,\n  placePeers,\n  placeText,\n  presencePlace,\n  presenceSelf,\n  presenceView,\n",
     "  QuizPresence,\n  THINKING_FRAME_INTERVAL_MS,\n  WATCH_INTERVAL_MS,\n  browserPresenceConnect,\n  cursorAt,\n  homeWatchScopes,\n  paintStyle,\n  placePeers,\n  placeText,\n  presenceDrafts,\n  presencePlace,\n  presenceSelf,\n  presenceView,\n  sheetItemLabels,\n"),
    ("  PresenceConnect,\n  PresencePlace,\n", "  PresenceConnect,\n  PresenceDrafts,\n  PresencePlace,\n"),
    ("  RoomStatus,\n} from \"./🔨️modules/👥️presence/🟦️.tsx\";\n",
     "  RoomStatus,\n  WatchedState,\n} from \"./🔨️modules/👥️presence/🟦️.tsx\";\n"
     "export { CrowdChoices, CrowdPosition, CrowdProvider, CrowdSource, chooseCrowd, crowdPlace, liveItems, meanPosition, submittedItems, useCrowd } from \"./🔨️modules/🗳️crowd/🟦️.tsx\";\n"
     "export type { Crowd, CrowdChoice, ItemCrowd } from \"./🔨️modules/🗳️crowd/🟦️.tsx\";\n"),
]
for old, new in EDITS:
    if text.count(old) != 1:
        sys.exit(f"anchor found {text.count(old)} times: {old[:90]!r}")
    text = text.replace(old, new)
for attempt in range(40):
    try:
        path.write_text(text, encoding="utf-8", newline="")
        break
    except OSError:
        time.sleep(0.5)
else:
    sys.exit("write failed")
print("[crowd exports] done")
