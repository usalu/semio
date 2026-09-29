"""🗳️ Home asks for the submitted crowd of every quiz while it shows (instead of on `session.open`), and the quiz pages
honour the learner's choice to hide what the others think. Exact anchors, each found once per file. (The quiz page's
crowd section itself was applied inline in the session of 2026-09-29 night; see `📖️quiz-page/🟦️.tsx`.)"""

import pathlib
import sys
import time

MODULES = pathlib.Path("C:/git/semio/🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🔨️modules")


def edit(relative: str, edits: list[tuple[str, str]]) -> None:
    path = MODULES / relative
    text = path.read_text(encoding="utf-8")
    for old, new in edits:
        if text.count(old) != 1:
            sys.exit(f"{relative}: anchor found {text.count(old)} times: {old[:90]!r}")
        text = text.replace(old, new)
    for attempt in range(40):
        try:
            path.write_text(text, encoding="utf-8", newline="")
            return
        except OSError:
            time.sleep(0.5)
    sys.exit(f"{relative}: write failed")


edit("🧭️session/🟦️.ts", [
    ("    if (step.screen === \"home\" && step.page === undefined) for (const quiz of this.state.catalog?.quizzes ?? []) void this.refreshCrowd(quiz.id);\n", ""),
])
edit("🏠️home/🟦️.tsx", [
    ("  usePolling(session.refreshLeaderboard, LEADERBOARD_POLL_MS);\n",
     "  usePolling(session.refreshLeaderboard, LEADERBOARD_POLL_MS);\n"
     "  const quizIds = state.catalog?.quizzes.map((quiz) => quiz.id).join(\" \") ?? \"\";\n"
     "  useEffect(() => {\n"
     "    for (const quiz of quizIds.split(\" \").filter(Boolean)) void session.refreshCrowd(quiz);\n"
     "  }, [quizIds, session]);\n"),
    ("      if (quiz !== undefined) return <QuizPage {...common} quiz={quiz} busy={busy} act={act} view={view} />;\n",
     "      if (quiz !== undefined) return <QuizPage {...common} quiz={quiz} busy={busy} act={act} view={view} showAnswers={preferences.showAnswers} />;\n"),
])
print("[crowd home] done")
