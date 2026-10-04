"""🧬️ Brings `quiz_react_mutation_checks.py` to the live grid and what the others think (design §17): the mutations of
behaviour that is gone (fixed track lists, the leaderboard's own polling gate, the private drag) give way to the new
guards — cards spaced in their cells, home polling, the grid rest, the card layer on the overview's tracks without
gaps, the watch (again after a rejoin, snapshots replace, unwatched rooms forgotten, only asked rooms, the joined room
left out), drafts at most twice a second, the drag shared, the own learner never in a crowd, the others in the pages
off with the cursors, and the crowd (numeric values, one learner per tag, place rounding, "all runs" wording, hidden
at the learner's wish in the run and on a quiz's page, "nobody" in results). Exact anchors in the script's text."""

import pathlib
import sys
import time

path = pathlib.Path(__file__).resolve().parent / "quiz_react_mutation_checks.py"
text = path.read_text(encoding="utf-8")
BS = "\\"


def swap(old: str, new: str) -> None:
    global text
    old, new = old.replace("⏎", BS + "n"), new.replace("⏎", BS + "n")
    if text.count(old) != 1:
        sys.exit(f"found {text.count(old)}: {old[:90]!r}")
    text = text.replace(old, new)


swap('"""🧬️ Mutation checks for the 2026-09-29 rounds (card grid, then shared presence): each mutation breaks one guarded behaviour, runs the test\nfile that guards it and expects it to fail; the original bytes are restored afterwards whatever happens.',
     '"""🧬️ Mutation checks for the 2026-09-29 rounds (card grid, shared presence, the layered home, then the live grid and what the others\nthink): each mutation breaks one guarded behaviour, runs the test file that guards it and expects it to fail; the original bytes are\nrestored afterwards whatever happens.')
swap('PRESENCE = QUIZ_TARGET / "🔨️modules" / "👥️presence" / "🟦️.tsx"\n',
     'PRESENCE = QUIZ_TARGET / "🔨️modules" / "👥️presence" / "🟦️.tsx"\nCROWD = QUIZ_TARGET / "🔨️modules" / "🗳️crowd" / "🟦️.tsx"\n')
swap('''    ("home-cards-centred", "quiz", QUIZ_TARGET / "🎨️.css",
     "  align-content: start;⏎  align-items: center;⏎", "  align-content: start;⏎  align-items: stretch;⏎", "🏠️home-grid"),''',
     '''    ("home-cards-spaced-in-cells", "quiz", QUIZ_TARGET / "🎨️.css",
     "  padding: var(--spacing-double);⏎}⏎⏎.quiz-home-cell > * {", "}⏎⏎.quiz-home-cell > * {", "🏠️home-grid"),''')
swap('''    ("leaderboard-polling-gate", "quiz", QUIZ_TARGET / "🔨️modules" / "🏆️leaderboard" / "🟦️.tsx",
     "usePolling(session.refreshLeaderboard, LEADERBOARD_POLL_MS, view.opened || view.revealed);", "usePolling(session.refreshLeaderboard, LEADERBOARD_POLL_MS);", "🏠️home-grid"),''',
     '''    ("home-polls-leaderboard", "quiz", QUIZ_TARGET / "🔨️modules" / "🏠️home" / "🟦️.tsx",
     "  usePolling(session.refreshLeaderboard, LEADERBOARD_POLL_MS);⏎", "", "🏠️home-grid"),
    ("home-grid-rest", "quiz", QUIZ_TARGET / "🔨️modules" / "🏠️home" / "🟦️.tsx",
     'rest="grid"', 'rest="panorama"', "🏠️home-grid"),''')
swap('''    ("desktop-breakpoint", "quiz", QUIZ_TARGET / "🎨️.css",
     "@media (min-width: 1024px) {⏎  .quiz-home-grid {", "@media (min-width: 1100px) {⏎  .quiz-home-grid {", "🏠️home-grid"),
    ("tablet-board-span", "quiz", QUIZ_TARGET / "🎨️.css",
     '  .quiz-home-grid [data-card="board"] {⏎    grid-column: 1 / -1;', '  .quiz-home-grid [data-card="board"] {⏎    grid-column: auto;', "🏠️home-grid"),''',
     '''    ("grid-layer-tracks", "quiz", QUIZ_TARGET / "🎨️.css",
     "    grid-template-columns: var(--layered-columns);⏎", "    grid-template-columns: repeat(3, minmax(0, 1fr));⏎", "🏠️home-grid"),
    ("grid-layer-no-gaps", "quiz", QUIZ_TARGET / "🎨️.css",
     "  display: grid;⏎  grid-template-columns: minmax(0, 1fr);⏎}", "  display: grid;⏎  gap: var(--spacing-double);⏎  grid-template-columns: minmax(0, 1fr);⏎}", "🏠️home-grid"),''')
swap('''    ("presence-drag-private", "quiz", PRESENCE,
     "event.buttons === 0 ? anchorOf(event.target) : null", "anchorOf(event.target)", "📡️presence-client"),''',
     '''    ("presence-drag-by-grip-only", "quiz", PRESENCE,
     'target.closest("[data-quiz-grip]") === null', "false", "📡️presence-client"),''')
swap('''    ("presence-overlay-off", "quiz", PRESENCE,
     "const active = show && view.peers.length > 0;", "const active = view.peers.length > 0;", "📡️presence-client"),''',
     '''    ("presence-overlay-off", "quiz", PRESENCE,
     "const active = show && view.peers.length > 0;", "const active = view.peers.length > 0;", "📡️presence-client"),
    ("watch-again-after-rejoin", "quiz", PRESENCE,
     "        this.sendWatch();⏎        break;", "        break;", "📡️presence-client"),
    ("watch-snapshot-replaces", "quiz", PRESENCE,
     "const room = snapshot ? new Map<string, RoomMember<W>>() : new Map(this.watchedMembers.get(scope) ?? []);", "const room = new Map(this.watchedMembers.get(scope) ?? []);", "📡️presence-client"),
    ("watch-forgets-unwatched", "quiz", PRESENCE,
     "for (const scope of [...this.watchedMembers.keys()]) if (!kept.includes(scope)) this.watchedMembers.delete(scope);", "", "📡️presence-client"),
    ("watch-only-asked-rooms", "quiz", PRESENCE,
     "if (parse === undefined || !this.watching?.scopes.includes(scope)) return;", "if (parse === undefined) return;", "📡️presence-client"),
    ("watch-without-joined-room", "quiz", PRESENCE,
     "  return scopes.filter((scope) => scope !== joined).slice(0, MAX_WATCHED_ROOMS);", "  return scopes.slice(0, MAX_WATCHED_ROOMS);", "📡️presence-client"),
    ("thinking-twice-a-second", "quiz", PRESENCE,
     '"thinking", { parse: parseThinking, intervalMs: THINKING_FRAME_INTERVAL_MS }', '"thinking", { parse: parseThinking }', "📡️presence-client"),
    ("drag-shared", "quiz", PRESENCE,
     "...(this.dragged === undefined ? {} : { drag: { item: this.dragged } }),", "", "📡️presence-client"),
    ("thinking-without-own", "quiz", PRESENCE,
     "thinking.set(scope, others(members, undefined).flatMap(", "thinking.set(scope, members.flatMap(", "📡️presence-client"),
    ("pane-peers-off", "quiz", PRESENCE,
     "const active = showCursors && !props.opened && peers.length > 0;", "const active = peers.length > 0;", "🏠️home-grid"),
    ("crowd-values-numerically", "quiz", CROWD,
     "  if (Number.isFinite(a) && Number.isFinite(b)) return a - b;⏎", "", "💭️crowd-client"),
    ("crowd-one-learner-per-tag", "quiz", CROWD,
     "const thinkers = new Set(others.map((state) => state.tag)).size;", "const thinkers = others.length;", "💭️crowd-client"),
    ("crowd-place-rounding", "quiz", CROWD,
     "return Math.round((1 + position * Math.max(0, total - 1)) * 10) / 10;", "return Math.floor((1 + position * Math.max(0, total - 1)) * 10) / 10;", "💭️crowd-client"),
    ("crowd-all-runs-wording", "quiz", CROWD,
     'text(live ? "quiz.crowd.item" : "quiz.crowd.itemAll"', 'text("quiz.crowd.item"', "💭️crowd-client"),
    ("crowd-hidden-in-run", "quiz", QUIZ_TARGET / "🔨️modules" / "▶️run" / "🟦️.tsx",
     "const crowd = props.showAnswers === false ? undefined : chooseCrowd(others, state.crowds[view.quiz]);", "const crowd = chooseCrowd(others, state.crowds[view.quiz]);", "💭️crowd-client"),
    ("crowd-nobody-in-results", "quiz", QUIZ_TARGET / "🔨️modules" / "🏁️results" / "🟦️.tsx",
     '<span className="sr-only">{text("quiz.crowd.nobody")}</span>', "", "💭️crowd-client"),
    ("crowd-hidden-on-quiz-page", "quiz", QUIZ_TARGET / "🔨️modules" / "📖️quiz-page" / "🟦️.tsx",
     "  if (!props.shown) return null;⏎", "", "💭️crowd-client"),''')
for attempt in range(40):
    try:
        path.write_text(text, encoding="utf-8", newline="")
        break
    except OSError:
        time.sleep(0.5)
else:
    sys.exit("write failed")
print("[mutations live grid] done")
