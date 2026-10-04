"""🧬️ Mutation checks for the 2026-09-29 rounds (card grid, shared presence, the layered home, then the live grid and what the others
think): each mutation breaks one guarded behaviour, runs the test file that guards it and expects it to fail; the original bytes are
restored afterwards whatever happens.

Run from anywhere: `python quiz_react_mutation_checks.py`. Output goes to `🗑️generated/react/mutations/`."""

import pathlib
import subprocess
import sys
import time

ROOT = pathlib.Path("C:/git/semio")
TICKET = pathlib.Path(__file__).resolve().parent
OUT = TICKET / "🗑️generated" / "react" / "mutations"
VITEST = ROOT / "node_modules" / "vitest" / "vitest.mjs"
QUIZ = ROOT / "🧰️framework" / "🛍️products" / "❓️quiz"
QUIZ_TARGET = QUIZ / "🎯️targets" / "⚛️react"
UI = ROOT / "🧰️framework" / "🔨️modules" / "🖱️ui"
SUITES = {
    "quiz": (QUIZ_TARGET / "📦️packages" / "🟦️typescript", QUIZ_TARGET / "🧪️tests" / "🎚️config" / "🟦️.ts"),
    "ui": (UI / "🎯️targets" / "⚛️react" / "📦️packages" / "🟦️typescript", UI / "🎯️targets" / "⚛️react" / "🧪️tests" / "🎚️config" / "🟦️.ts"),
}
PRESENCE = QUIZ_TARGET / "🔨️modules" / "👥️presence" / "🟦️.tsx"
CROWD = QUIZ_TARGET / "🔨️modules" / "🗳️crowd" / "🟦️.tsx"
NOWRAP_HEAD = 'className={cn(edge, "quiz-nowrap border-b-2")}'
MUTATIONS = [
    ("home-ring-order", "quiz", QUIZ_TARGET / "🔨️modules" / "🏠️home" / "🟦️.tsx",
     "HOME_PAGES.leaderboard, ...quiz(2),", "...quiz(2), HOME_PAGES.leaderboard,", "🏠️home-grid"),
    ("home-cells-board-alone-on-tablets", "quiz", QUIZ_TARGET / "🔨️modules" / "🏠️home" / "🟦️.tsx",
     'const alone = layout === "tablet" && page === HOME_PAGES.leaderboard;', "const alone = false;", "🏠️home-grid"),
    ("home-cards-spaced-in-cells", "quiz", QUIZ_TARGET / "🎨️.css",
     "  padding: var(--spacing-double);\n}\n\n.quiz-home-cell > * {", "}\n\n.quiz-home-cell > * {", "🏠️home-grid"),
    ("home-heading-links", "quiz", QUIZ_TARGET / "🔨️modules" / "🪟️chrome" / "🟦️.tsx",
     "props.href === undefined ? (", "true ? (", "🏠️home-grid"),
    ("home-polls-leaderboard", "quiz", QUIZ_TARGET / "🔨️modules" / "🏠️home" / "🟦️.tsx",
     "  usePolling(session.refreshLeaderboard, LEADERBOARD_POLL_MS);\n", "", "🏠️home-grid"),
    ("home-grid-rest", "quiz", QUIZ_TARGET / "🔨️modules" / "🏠️home" / "🟦️.tsx",
     'rest="grid"', 'rest="panorama"', "🏠️home-grid"),
    ("results-open-the-leaderboard-page", "quiz", QUIZ_TARGET / "🔨️modules" / "🏁️results" / "🟦️.tsx",
     'session.open({ screen: "home", page: HOME_PAGES.leaderboard })', 'session.open({ screen: "home" })', "🚶️learner-journey"),
    ("presence-quiz-page-place", "quiz", PRESENCE,
     '{ place: { screen: "quiz", quiz: step.page }, room: true }', '{ place: { screen: "home" }, room: true }', "📡️presence-client"),
    ("excerpt-size", "quiz", QUIZ_TARGET / "🔨️modules" / "🏆️leaderboard" / "🟦️.tsx",
     "export const BOARD_EXCERPT_SIZE = 5;", "export const BOARD_EXCERPT_SIZE = 4;", "🏠️home-grid"),
    ("header-nowrap", "quiz", QUIZ_TARGET / "🔨️modules" / "🏆️leaderboard" / "🟦️.tsx",
     NOWRAP_HEAD, 'className={cn(edge, "border-b-2")}', "🏠️home-grid"),
    ("nowrap-rule", "quiz", QUIZ_TARGET / "🎨️.css",
     "white-space: nowrap;\n  hyphens: none;", "hyphens: none;", "🏠️home-grid"),
    ("grid-layer-tracks", "quiz", QUIZ_TARGET / "🎨️.css",
     "    grid-template-columns: var(--layered-columns);\n", "    grid-template-columns: repeat(3, minmax(0, 1fr));\n", "🏠️home-grid"),
    ("grid-layer-no-gaps", "quiz", QUIZ_TARGET / "🎨️.css",
     "  display: grid;\n  grid-template-columns: minmax(0, 1fr);\n}", "  display: grid;\n  gap: var(--spacing-double);\n  grid-template-columns: minmax(0, 1fr);\n}", "🏠️home-grid"),
    ("text-presentation", "quiz", QUIZ_TARGET / "🔨️modules" / "🪟️chrome" / "🟦️.tsx",
     "return emoji.replaceAll(", "return emoji; void emoji.replaceAll(", "🏠️home-grid"),
    ("abortable-observes-loser", "quiz", QUIZ_TARGET / "🔨️modules" / "🛂️proctor" / "🟦️.ts",
     "    promise.catch(() => undefined);\n", "", "📬️outbox-delivery"),
    ("overview-title-chip-width", "ui", UI / "🧱️elements" / "🃏️OverviewCard" / "🟦️.tsx",
     '"flex min-w-0 max-w-full items-center gap-single px-single"', '"flex min-w-0 items-center gap-single px-single"', "🃏️OverviewCard"),
    ("overview-dialog-level", "ui", UI / "🧱️elements" / "🃏️OverviewCard" / "🟦️.tsx",
     'level="dialog"', 'level="window"', "🃏️OverviewCard"),
    ("presence-throttle", "quiz", PRESENCE,
     "Math.ceil(1000 / PRESENCE_FRAME_HZ)", "0", "📡️presence-client"),
    ("presence-latest-wins", "quiz", PRESENCE,
     " || this.desired === this.sent) return;", ") return;", "📡️presence-client"),
    ("presence-resend-after-rejoin", "quiz", PRESENCE,
     "    this.sent = undefined;\n    this.watchSent = undefined;\n", "    this.watchSent = undefined;\n", "📡️presence-client"),
    ("presence-rejoin-jitter", "quiz", PRESENCE,
     "await pause((this.options.random ?? Math.random)() * minMs, signal);", "await pause(0, signal);", "📡️presence-client"),
    ("presence-admits-members", "quiz", PRESENCE,
     "    if (state === undefined) this.members.delete(entry.session);\n    else this.members.set(", "    this.members.set(", "📡️presence-client"),
    ("presence-drag-by-grip-only", "quiz", PRESENCE,
     'target.closest("[data-quiz-grip]") === null', "false", "📡️presence-client"),
    ("presence-keyboard-focus-only", "quiz", PRESENCE,
     "presence.focusOn(keyboard ? anchorOf(event.target)?.dataset.presenceAnchor : undefined)", "presence.focusOn(anchorOf(event.target)?.dataset.presenceAnchor)", "📡️presence-client"),
    ("presence-results-without-task", "quiz", PRESENCE,
     'step.screen === "run" && task !== undefined ?', "task !== undefined ?", "📡️presence-client"),
    ("presence-one-colour", "quiz", PRESENCE,
     "colour: colours.get(member.state.tag) ?? member.colour,", "colour: member.colour,", "📡️presence-client"),
    ("presence-overlay-off", "quiz", PRESENCE,
     "const active = show && view.peers.length > 0;", "const active = view.peers.length > 0;", "📡️presence-client"),
    ("watch-again-after-rejoin", "quiz", PRESENCE,
     "        this.sendWatch();\n        break;", "        break;", "📡️presence-client"),
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
     "        others(members, undefined).flatMap((member) => (isThinking(member.state) ? [member.state] : [])),", "        members.flatMap((member) => (isThinking(member.state) ? [member.state] : [])),", "📡️presence-client"),
    ("pane-peers-off", "quiz", PRESENCE,
     "const active = showCursors && !props.opened && peers.length > 0;", "const active = peers.length > 0;", "🏠️home-grid"),
    ("crowd-values-numerically", "quiz", CROWD,
     "  if (Number.isFinite(a) && Number.isFinite(b)) return a - b;\n", "", "💭️crowd-client"),
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
     "  if (!props.shown) return null;\n", "", "💭️crowd-client"),
    ("presence-app-preference", "quiz", QUIZ_TARGET / "🟦️.tsx",
     "<PresenceOverlay view={view} show={preferences.showCursors} itemLabel={itemLabel} />", "<PresenceOverlay view={view} show itemLabel={itemLabel} />", "🚶️learner-journey"),
    ("presence-app-place", "quiz", QUIZ_TARGET / "🟦️.tsx",
     "const at = presencePlace(state.step, state, task);", "const at = presencePlace(state.step, state, undefined);", "🚶️learner-journey"),
]


def write(path: pathlib.Path, data: bytes) -> None:
    for attempt in range(20):
        try:
            path.write_bytes(data)
            return
        except OSError:
            time.sleep(0.25 * (attempt + 1))
    path.write_bytes(data)


def run(suite: str, target: str, log: pathlib.Path) -> int:
    cwd, config = SUITES[suite]
    with log.open("w", encoding="utf-8") as handle:
        return subprocess.run(["bun", str(VITEST), "run", "--config", str(config), target], cwd=cwd, stdout=handle, stderr=subprocess.STDOUT, timeout=900).returncode


def main() -> int:
    sys.stdout.reconfigure(encoding="utf-8")
    OUT.mkdir(parents=True, exist_ok=True)
    only = set(sys.argv[1:])
    results = []
    for name, suite, path, old, new, target in MUTATIONS:
        if only and name not in only:
            continue
        original = path.read_bytes()
        text = original.decode("utf-8")
        if text.count(old) != 1:
            results.append((name, f"anchor found {text.count(old)} times"))
            continue
        try:
            write(path, text.replace(old, new).encode("utf-8"))
            code = run(suite, target, OUT / f"{name}.log")
        finally:
            write(path, original)
        results.append((name, "killed" if code != 0 else "SURVIVED"))
    baseline = {}
    for index, (suite, target) in enumerate(sorted({(suite, target) for mutation, suite, _p, _o, _nw, target in MUTATIONS if not only or mutation in only})):
        baseline[f"{suite}:{target}"] = "passes" if run(suite, target, OUT / f"baseline-{index}.log") == 0 else "FAILS"
    for name, verdict in results:
        print(f"[mutation] {name}: {verdict}")
    for key, verdict in baseline.items():
        print(f"[baseline] {key}: {verdict}")
    return 0 if all(verdict == "killed" for _n, verdict in results) and all(v == "passes" for v in baseline.values()) else 1


if __name__ == "__main__":
    sys.exit(main())
