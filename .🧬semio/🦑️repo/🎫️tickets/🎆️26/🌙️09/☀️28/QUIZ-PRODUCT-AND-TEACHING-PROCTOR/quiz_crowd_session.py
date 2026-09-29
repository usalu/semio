"""🗳️ Brings the submitted crowd of every quiz (query `crowd`, design §17) into the quiz client: `ProctorClient.crowd`,
the session's `crowds` (ephemeral, per quiz, the last known), the `crowd-loaded` event, `refreshCrowd`, and the moments
it is asked for — home (every quiz), a run or its results (that quiz) and right after a submission. Exact anchors."""

import pathlib
import sys
import time

MODULES = pathlib.Path("C:/git/semio/🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🔨️modules")


def edit(relative: str, edits: list[tuple[str, str]]) -> None:
    path = MODULES / relative
    text = path.read_text(encoding="utf-8")
    for old, new in edits:
        if text.count(old) != 1:
            sys.exit(f"{relative}: anchor found {text.count(old)} times: {old[:80]!r}")
        text = text.replace(old, new)
    for attempt in range(40):
        try:
            path.write_text(text, encoding="utf-8", newline="")
            return
        except OSError:
            time.sleep(0.5)
    sys.exit(f"{relative}: write failed")


edit("🛂️proctor/🟦️.ts", [
    ("  /** 🏆️ Every learner with a submitted run. */\n",
     "  /** 👥️ What the learners answered in the submitted runs of `quiz`, per task and item. */\n"
     "  crowd(quiz: Slug, learner: Id | undefined, signal?: AbortSignal): Promise<CrowdView> {\n"
     "    return this.query<CrowdView>({ type: \"crowd\", quiz }, learner, signal);\n"
     "  }\n\n"
     "  /** 🏆️ Every learner with a submitted run. */\n"),
])
edit("🧭️session/🟦️.ts", [
    ("  CatalogView,\n  Event,\n", "  CatalogView,\n  CrowdView,\n  Event,\n"),
    ("  readonly leaderboard?: { readonly board: Leaderboard; readonly at: number };\n  readonly notice?: QuizNotice;\n}\n",
     "  readonly leaderboard?: { readonly board: Leaderboard; readonly at: number };\n  readonly crowds: Readonly<Record<Slug, CrowdView>>;\n  readonly notice?: QuizNotice;\n}\n"),
    ("  | { readonly type: \"leaderboard-loaded\"; readonly leaderboard: Leaderboard; readonly at: number }\n",
     "  | { readonly type: \"leaderboard-loaded\"; readonly leaderboard: Leaderboard; readonly at: number }\n  | { readonly type: \"crowd-loaded\"; readonly crowd: CrowdView }\n"),
    ("  return { ...persisted, step, awards: {} };\n", "  return { ...persisted, step, awards: {}, crowds: {} };\n"),
    ("    case \"leaderboard-loaded\":\n      return { ...state, leaderboard: { board: event.leaderboard, at: event.at } };\n",
     "    case \"leaderboard-loaded\":\n      return { ...state, leaderboard: { board: event.leaderboard, at: event.at } };\n"
     "    case \"crowd-loaded\":\n      return { ...state, crowds: { ...state.crowds, [event.crowd.quiz]: event.crowd } };\n"),
    ("  /** 🏆️ Asks for the leaderboard once; polling repeats it, so a failure only keeps the last known standings. */\n",
     "  /** 👥️ Asks for the submitted crowd of `quiz` once; a failure keeps the last known crowd. */\n"
     "  async refreshCrowd(quiz: Slug): Promise<void> {\n"
     "    try {\n"
     "      this.dispatch({ type: \"crowd-loaded\", crowd: await this.proctor.crowd(quiz, this.state.learner?.id, this.lifetime.signal) });\n"
     "    } catch {\n"
     "      return;\n"
     "    }\n"
     "  }\n\n"
     "  /** 🏆️ Asks for the leaderboard once; polling repeats it, so a failure only keeps the last known standings. */\n"),
    ("    if (step.screen === \"run\" || step.screen === \"results\") void this.loadRun(step.run);\n  }\n",
     "    if (step.screen === \"home\" && step.page === undefined) for (const quiz of this.state.catalog?.quizzes ?? []) void this.refreshCrowd(quiz.id);\n"
     "    if (step.screen === \"run\" || step.screen === \"results\") void this.loadRun(step.run).then((view) => view !== undefined && this.refreshCrowd(view.quiz));\n  }\n"),
    ("    void this.refreshLearner();\n    return undefined;\n  }\n",
     "    void this.refreshLearner();\n"
     "    const quiz = this.state.runs[run]?.quiz;\n"
     "    if (quiz !== undefined) void this.refreshCrowd(quiz);\n"
     "    return undefined;\n  }\n"),
])
edit("🛂️proctor/🟦️.ts", [
    ("type Command, type Event, type Id, type Leaderboard, type LearnerView, type Query, type Rejection, type RunView } from \"@semio-tech/quiz\";\n",
     "type Command, type CrowdView, type Event, type Id, type Leaderboard, type LearnerView, type Query, type Rejection, type RunView, type Slug } from \"@semio-tech/quiz\";\n"),
])
print("[crowd session] done")
