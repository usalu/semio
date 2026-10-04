"""⚓️ Puts the presence anchors (design §15) on the cards every learner at a place renders, and the presence text where
the learners look for it: home (every card `home:<card>`, who is where on the learner card, learners in each quiz on its
card), the leaderboard (card and full table, online marks), badges, introduction, run (the run card and the task card
`task:<id>`, reporting the task) and results (`results`, `result:<task>`). Exact anchors, each found once per file."""

import pathlib
import sys
import time

MODULES = pathlib.Path("C:/git/semio/🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🔨️modules")
EDITS = {
    "🏠️home/🟦️.tsx": [
        ('import { BodyButton, CardAction, CardIcon, Facts, Glyph, QuizCard, textPresentation } from "../🪟️chrome/🟦️.tsx";\n',
         'import { BodyButton, CardAction, CardIcon, Facts, Glyph, QuizCard, textPresentation } from "../🪟️chrome/🟦️.tsx";\nimport { PRESENCE_ANCHORS, PresenceList, usePresenceView } from "../👥️presence/🟦️.tsx";\n'),
        ('    <QuizCard id={id} card="learner" icon={<CardIcon icon="user" />}',
         '    <QuizCard id={id} card="learner" anchor={PRESENCE_ANCHORS.home("learner")} icon={<CardIcon icon="user" />}'),
        ('          ...(row === undefined || leaderboard === undefined ? [] : [text("quiz.home.rank", { rank: row.rank, total: leaderboard.board.rows.length })]),\n        ]}\n      />\n    </QuizCard>\n',
         '          ...(row === undefined || leaderboard === undefined ? [] : [text("quiz.home.rank", { rank: row.rank, total: leaderboard.board.rows.length })]),\n        ]}\n      />\n      <PresenceList catalog={catalog} text={text} locale={locale} className="mt-auto" />\n    </QuizCard>\n'),
        ('  const earned = badgesEarnedIn(state, quiz.id);\n  const primary =\n',
         '  const earned = badgesEarnedIn(state, quiz.id);\n  const learning = usePresenceView().roster?.quizzes[quiz.id] ?? 0;\n  const primary =\n'),
        ('      card={`quiz:${quiz.id}`}\n',
         '      card={`quiz:${quiz.id}`}\n      anchor={PRESENCE_ANCHORS.home(`quiz:${quiz.id}`)}\n'),
        ('...(open === undefined ? [] : [text("quiz.home.open")])]} />',
         '...(open === undefined ? [] : [text("quiz.home.open")]), ...(learning === 0 ? [] : [text("quiz.presence.learningNow", { count: learning })])]} />'),
        ('      card="introduction"\n      icon={<CardIcon icon="info" />}\n      title={text("quiz.home.howItWorks")}\n',
         '      card="introduction"\n      anchor={PRESENCE_ANCHORS.home("introduction")}\n      icon={<CardIcon icon="info" />}\n      title={text("quiz.home.howItWorks")}\n'),
        ('<PreferencesCard key="preferences" preferences={preferences} locale={locale} text={text} onChange={onPreferences} />',
         '<PreferencesCard key="preferences" anchor={PRESENCE_ANCHORS.home("preferences")} preferences={preferences} locale={locale} text={text} onChange={onPreferences} />'),
    ],
    "🏆️leaderboard/🟦️.tsx": [
        ('import type { QuizSession, QuizState } from "../🧭️session/🟦️.ts";\n',
         'import type { QuizSession, QuizState } from "../🧭️session/🟦️.ts";\nimport { OnlineMark, PRESENCE_ANCHORS, usePresenceView } from "../👥️presence/🟦️.tsx";\n'),
        ('function BoardRow(props: { readonly row: LeaderboardRow; readonly mine: boolean; readonly name: string; readonly text: QuizText; readonly locale: QuizLocale }): ReactElement {\n  const { row, mine, name, text, locale } = props;\n',
         'function BoardRow(props: { readonly row: LeaderboardRow; readonly mine: boolean; readonly name: string; readonly online: number | undefined; readonly text: QuizText; readonly locale: QuizLocale }): ReactElement {\n  const { row, mine, name, online, text, locale } = props;\n'),
        ('        {mine ? <span className="font-normal text-muted-foreground"> ({text("quiz.leaderboard.you")})</span> : null}\n      </th>\n      <td className={cn(CELL, NUMBER, "text-right")}>{formatPoints(row.total, locale)}</td>\n',
         '        {mine ? <span className="font-normal text-muted-foreground"> ({text("quiz.leaderboard.you")})</span> : null}\n        {online === undefined ? null : <OnlineMark colour={online} text={text} />}\n      </th>\n      <td className={cn(CELL, NUMBER, "text-right")}>{formatPoints(row.total, locale)}</td>\n'),
        ('  const name = (row: LeaderboardRow): string => learnerName(row.identity, row.tag, text);\n  const head = cn(CELL, "quiz-nowrap font-medium");\n',
         '  const name = (row: LeaderboardRow): string => learnerName(row.identity, row.tag, text);\n  const head = cn(CELL, "quiz-nowrap font-medium");\n  const { colours } = usePresenceView();\n'),
        ('      card="board"\n', '      card="board"\n      anchor={PRESENCE_ANCHORS.home("board")}\n'),
        ('<BoardRow key={row.tag} row={row} mine={row.tag === mine} name={name(row)} text={text} locale={locale} />',
         '<BoardRow key={row.tag} row={row} mine={row.tag === mine} name={name(row)} online={colours.get(row.tag)} text={text} locale={locale} />'),
        ('<BoardRow row={own} mine name={name(own)} text={text} locale={locale} />',
         '<BoardRow row={own} mine name={name(own)} online={colours.get(own.tag)} text={text} locale={locale} />'),
        ('  const board = state.leaderboard;\n  const shown = columns(state.catalog, text, locale);\n',
         '  const board = state.leaderboard;\n  const shown = columns(state.catalog, text, locale);\n  const { colours } = usePresenceView();\n'),
        ('      card="leaderboard"\n      headingLevel={1}\n', '      card="leaderboard"\n      anchor={PRESENCE_ANCHORS.leaderboard}\n      headingLevel={1}\n'),
        ('                      {row.tag === mine ? <span className="font-normal text-muted-foreground"> ({text("quiz.leaderboard.you")})</span> : null}\n                    </th>\n',
         '                      {row.tag === mine ? <span className="font-normal text-muted-foreground"> ({text("quiz.leaderboard.you")})</span> : null}\n                      {colours.get(row.tag) === undefined ? null : <OnlineMark colour={colours.get(row.tag) ?? 0} text={text} />}\n                    </th>\n'),
    ],
    "🏅️badges/🟦️.tsx": [
        ('import type { QuizSession, QuizState } from "../🧭️session/🟦️.ts";\n', 'import type { QuizSession, QuizState } from "../🧭️session/🟦️.ts";\nimport { PRESENCE_ANCHORS } from "../👥️presence/🟦️.tsx";\n'),
        ('      card="badges"\n      icon={<CardIcon icon="award" />}\n', '      card="badges"\n      anchor={PRESENCE_ANCHORS.home("badges")}\n      icon={<CardIcon icon="award" />}\n'),
    ],
    "👋️introduction/🟦️.tsx": [
        ('import { CardAction, CardIcon, QuizCard } from "../🪟️chrome/🟦️.tsx";\n', 'import { CardAction, CardIcon, QuizCard } from "../🪟️chrome/🟦️.tsx";\nimport { PRESENCE_ANCHORS } from "../👥️presence/🟦️.tsx";\n'),
        ('      card="introduction"\n      headingLevel={1}\n', '      card="introduction"\n      anchor={PRESENCE_ANCHORS.introduction}\n      headingLevel={1}\n'),
    ],
    "🏁️results/🟦️.tsx": [
        ('import { runAwards, type QuizSession, type QuizState } from "../🧭️session/🟦️.ts";\n', 'import { runAwards, type QuizSession, type QuizState } from "../🧭️session/🟦️.ts";\nimport { PRESENCE_ANCHORS } from "../👥️presence/🟦️.tsx";\n'),
        ('<QuizCard id={`${scope}-title`} card="results" headingLevel={1} focusableHeading icon={icon} title={title} footerRight={home}>',
         '<QuizCard id={`${scope}-title`} card="results" anchor={PRESENCE_ANCHORS.results} headingLevel={1} focusableHeading icon={icon} title={title} footerRight={home}>'),
        ('        card="results"\n        headingLevel={1}\n', '        card="results"\n        anchor={PRESENCE_ANCHORS.results}\n        headingLevel={1}\n'),
        ('card="task-result" icon=', 'card="task-result" anchor={PRESENCE_ANCHORS.result(taskResult.task)} icon='),
    ],
    "▶️run/🟦️.tsx": [
        ('import type { QuizSession, QuizState, SubmissionPhase } from "../🧭️session/🟦️.ts";\n',
         'import type { QuizSession, QuizState, SubmissionPhase } from "../🧭️session/🟦️.ts";\nimport { PRESENCE_ANCHORS, usePresenceTask } from "../👥️presence/🟦️.tsx";\n'),
        ('  const view = state.runs[run];\n  if (view === undefined) return null;\n',
         '  const view = state.runs[run];\n  usePresenceTask(view?.sheet.tasks[Math.min(current, view.sheet.tasks.length - 1)]?.id);\n  if (view === undefined) return null;\n'),
        ('        card="run"\n        headingLevel={1}\n', '        card="run"\n        anchor={PRESENCE_ANCHORS.run}\n        headingLevel={1}\n'),
        ('          card="task"\n          headingRef={taskHeading}\n', '          card="task"\n          anchor={PRESENCE_ANCHORS.task(task.id)}\n          headingRef={taskHeading}\n'),
    ],
}


def apply(relative: str, edits: list[tuple[str, str]]) -> None:
    path = MODULES / relative
    text = path.read_text(encoding="utf-8")
    for old, new in edits:
        if text.count(old) != 1:
            sys.exit(f"{relative}: anchor found {text.count(old)} times: {old[:90]!r}")
        text = text.replace(old, new)
    for attempt in range(20):
        try:
            path.write_text(text, encoding="utf-8", newline="")
            return
        except OSError:
            time.sleep(0.25 * (attempt + 1))


for relative, edits in EDITS.items():
    apply(relative, edits)
print("[presence anchors] done")
