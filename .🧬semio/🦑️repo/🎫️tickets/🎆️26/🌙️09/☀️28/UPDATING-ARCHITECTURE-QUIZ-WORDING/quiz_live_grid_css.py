"""🥞️ The card layer over the live grid (design §17) in the react target's `🎨️.css`: the overlay takes the overview's own
track variables (`--layered-columns`/`--layered-rows`) with no gaps and no outer padding from 768 px, every card sits
compact and centred in `.quiz-home-cell` (the spacing lies inside the cells), and a sorting's crowd is a short track
with markers. Replaces the fixed-track block of the card grid; exact anchors, each found once."""

import pathlib
import sys
import time

path = pathlib.Path("C:/git/semio/🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🎨️.css")
text = path.read_text(encoding="utf-8")
start = text.index("/* 🏠️ The overview's cards over the glass:")
end = text.index("/* 🖼️ A first-visit screen with the preferences beside it from 1024 px. */")
GRID = """/* 🏠️ The overview's cards over the glass, each in the cell of its page on the live grid behind: the card layer takes
   the overview's own track variables with no gaps (the spacing lies inside the cells), and each card is compact and
   centred in its cell so its page shows around it; a card taller than a short cell starts at its top (never above
   it, under the navbar) and spills only into the spacing of the cell below. Below 768 px the overview is a list of
   sections and places the cards itself. DOM order is reading order at every width. */
.quiz-home-grid {
  display: grid;
  grid-template-columns: minmax(0, 1fr);
}

@media (min-width: 768px) {
  .quiz-home-grid {
    grid-template-columns: var(--layered-columns);
    grid-template-rows: var(--layered-rows);
  }
}

.quiz-home-cell {
  display: flex;
  min-width: 0;
  min-height: 0;
  align-items: safe center;
  justify-content: safe center;
  padding: var(--spacing-double);
}

.quiz-home-cell > * {
  width: 100%;
}

"""
text = text[:start] + GRID + text[end:]
ONLINE = ".quiz-online {\n  display: inline-block;"
if text.count(ONLINE) != 1:
    sys.exit("online anchor")
text = text.replace(ONLINE, """/* 🗳️ Where the others place an item of a sorting: their markers (live in their colours) on a short track. */
.quiz-crowd-track {
  position: relative;
  display: inline-block;
  flex: none;
  width: 4em;
  height: 0.5em;
  border: var(--stroke-hairline) solid var(--border-normal-color);
}

.quiz-crowd-marker {
  position: absolute;
  top: -2px;
  bottom: -2px;
  left: calc(var(--quiz-crowd-at) * 100%);
  width: 0.3em;
  transform: translateX(-50%);
  background: var(--quiz-peer, var(--active-base));
}

""" + ONLINE)
for attempt in range(40):
    try:
        path.write_text(text, encoding="utf-8", newline="")
        break
    except OSError:
        time.sleep(0.5)
else:
    sys.exit("write failed")
print("[live grid css] done")
