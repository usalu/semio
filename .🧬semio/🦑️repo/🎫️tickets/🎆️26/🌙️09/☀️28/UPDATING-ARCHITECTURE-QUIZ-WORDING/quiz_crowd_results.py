"""🗳️ Results beside the crowd (design §17): every result table of `🏁️results/🟦️.tsx` gains "The others" — the others'
categories, average place or values for the item, from the submitted crowd of the quiz — and the summary card names the
crowd; nothing when the learner hides what the others think. Exact anchors, each found once."""

import pathlib
import sys
import time

path = pathlib.Path("C:/git/semio/🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🔨️modules/🏁️results/🟦️.tsx")
text = path.read_text(encoding="utf-8")
EDITS = [
    (' * summary and every task are cards. Verdicts are spelled out in words and symbols, never by colour alone.\n */',
     ' * summary and every task are cards. Verdicts are spelled out in words and symbols, never by colour alone. Unless the\n'
     ' * learner hides what the others think, every item stands beside the others: what the submitted runs answered.\n */'),
    ('import { PRESENCE_ANCHORS } from "../👥️presence/🟦️.tsx";\n',
     'import { PRESENCE_ANCHORS } from "../👥️presence/🟦️.tsx";\n'
     'import { CrowdChoices, CrowdPosition, CrowdProvider, CrowdSource, chooseCrowd, type Crowd, type ItemCrowd } from "../🗳️crowd/🟦️.tsx";\n'),
    ('function ClassificationResult(props: { readonly task: SheetClassificationTask; readonly result: ClassificationTaskResult; readonly text: QuizText; readonly locale: QuizLocale }): ReactElement {\n  const { task, result, text, locale } = props;\n  return (\n    <ResultTable head={[text("quiz.results.item"), text("quiz.results.yourAnswer"), text("quiz.results.solution"), text("quiz.results.credit"), text("quiz.results.explanation")]}>',
     'function Others(props: { readonly shown: boolean; readonly children: ReactNode }): ReactElement | null {\n'
     '  return props.shown ? <td className={EDGE}>{props.children ?? "–"}</td> : null;\n'
     '}\n\n'
     'function ClassificationResult(props: { readonly task: SheetClassificationTask; readonly result: ClassificationTaskResult; readonly crowd: ReadonlyMap<string, ItemCrowd> | undefined; readonly text: QuizText; readonly locale: QuizLocale }): ReactElement {\n'
     '  const { task, result, crowd, text, locale } = props;\n'
     '  return (\n'
     '    <ResultTable head={[text("quiz.results.item"), text("quiz.results.yourAnswer"), text("quiz.results.solution"), text("quiz.results.credit"), ...(crowd === undefined ? [] : [text("quiz.crowd.others")]), text("quiz.results.explanation")]}>'),
    ('          <td className={EDGE}>\n            {verdict(item.credit, text)} ({formatScore(item.credit, locale)})\n          </td>\n',
     '          <td className={EDGE}>\n            {verdict(item.credit, text)} ({formatScore(item.credit, locale)})\n          </td>\n'
     '          <Others shown={crowd !== undefined}>\n'
     '            <CrowdChoices item={crowd?.get(item.item)} subject={labelOf(task.items, item.item, locale)} label={(key) => labelOf(task.categories, key, locale)} text={text} />\n'
     '          </Others>\n'),
    ('function SortingResult(props: { readonly task: SheetSortingTask; readonly result: SortingTaskResult; readonly text: QuizText; readonly locale: QuizLocale }): ReactElement {\n  const { task, result, text, locale } = props;\n',
     'function SortingResult(props: { readonly task: SheetSortingTask; readonly result: SortingTaskResult; readonly crowd: ReadonlyMap<string, ItemCrowd> | undefined; readonly text: QuizText; readonly locale: QuizLocale }): ReactElement {\n  const { task, result, crowd, text, locale } = props;\n'),
    ('      <ResultTable head={[text("quiz.results.position"), text("quiz.results.item"), text("quiz.results.value"), text("quiz.results.rank"), text("quiz.results.explanation")]}>',
     '      <ResultTable head={[text("quiz.results.position"), text("quiz.results.item"), text("quiz.results.value"), text("quiz.results.rank"), ...(crowd === undefined ? [] : [text("quiz.crowd.others")]), text("quiz.results.explanation")]}>'),
    ('              {item.rank + 1} {item.rank === item.position ? `✓ ${text("quiz.results.correct")}` : `✗ ${text("quiz.results.wrong")}`}\n            </td>\n',
     '              {item.rank + 1} {item.rank === item.position ? `✓ ${text("quiz.results.correct")}` : `✗ ${text("quiz.results.wrong")}`}\n            </td>\n'
     '            <Others shown={crowd !== undefined}>\n'
     '              <CrowdPosition item={crowd?.get(item.item)} total={result.items.length} subject={labelOf(task.items, item.item, locale)} text={text} locale={locale} />\n'
     '            </Others>\n'),
    ('function MatchingResult(props: { readonly task: SheetMatchingTask; readonly result: MatchingTaskResult; readonly text: QuizText; readonly locale: QuizLocale }): ReactElement {\n  const { task, result, text, locale } = props;\n',
     'function MatchingResult(props: { readonly task: SheetMatchingTask; readonly result: MatchingTaskResult; readonly crowd: Crowd | undefined; readonly text: QuizText; readonly locale: QuizLocale }): ReactElement {\n  const { task, result, crowd, text, locale } = props;\n'),
    ('        const value = (amount: number): string => (sheet === undefined ? String(amount) : formatQuantity(amount, sheet.quantity, locale));\n',
     '        const value = (amount: number): string => (sheet === undefined ? String(amount) : formatQuantity(amount, sheet.quantity, locale));\n'
     '        const others = crowd?.items(task, dimension.dimension);\n'),
    ('            <ResultTable head={[text("quiz.results.item"), text("quiz.results.yourAnswer"), text("quiz.results.solution"), text("quiz.results.explanation")]}>',
     '            <ResultTable head={[text("quiz.results.item"), text("quiz.results.yourAnswer"), text("quiz.results.solution"), ...(others === undefined ? [] : [text("quiz.crowd.others")]), text("quiz.results.explanation")]}>'),
    ('                  <td className={NUMBER}>{value(item.correct)}</td>\n',
     '                  <td className={NUMBER}>{value(item.correct)}</td>\n'
     '                  <Others shown={others !== undefined}>\n'
     '                    <CrowdChoices item={others?.get(item.item)} subject={labelOf(task.items, item.item, locale)} label={(key) => value(Number(key))} text={text} />\n'
     '                  </Others>\n'),
    ('/** 🧾️ The result of one task beside its presented form. */\nexport function TaskResultView(props: { readonly task: SheetTask; readonly result: TaskResult; readonly text: QuizText; readonly locale: QuizLocale }): ReactElement | null {\n  const { task, result, text, locale } = props;\n'
     '  if (task.kind === "classification" && result.kind === "classification") return <ClassificationResult task={task} result={result} text={text} locale={locale} />;\n'
     '  if (task.kind === "sorting" && result.kind === "sorting") return <SortingResult task={task} result={result} text={text} locale={locale} />;\n'
     '  if (task.kind === "matching" && result.kind === "matching") return <MatchingResult task={task} result={result} text={text} locale={locale} />;\n',
     '/** 🧾️ The result of one task beside its presented form, and beside the others (`crowd`) when given. */\nexport function TaskResultView(props: { readonly task: SheetTask; readonly result: TaskResult; readonly crowd?: Crowd; readonly text: QuizText; readonly locale: QuizLocale }): ReactElement | null {\n  const { task, result, crowd, text, locale } = props;\n'
     '  if (task.kind === "classification" && result.kind === "classification") return <ClassificationResult task={task} result={result} crowd={crowd?.items(task)} text={text} locale={locale} />;\n'
     '  if (task.kind === "sorting" && result.kind === "sorting") return <SortingResult task={task} result={result} crowd={crowd?.items(task)} text={text} locale={locale} />;\n'
     '  if (task.kind === "matching" && result.kind === "matching") return <MatchingResult task={task} result={result} crowd={crowd} text={text} locale={locale} />;\n'),
    ('export function ResultsScreen(props: { readonly session: QuizSession; readonly state: QuizState; readonly run: Id; readonly text: QuizText; readonly locale: QuizLocale }): ReactElement | null {',
     'export function ResultsScreen(props: { readonly session: QuizSession; readonly state: QuizState; readonly run: Id; readonly text: QuizText; readonly locale: QuizLocale; readonly showAnswers?: boolean }): ReactElement | null {'),
    ('  const badges = runAwards(state, run).flatMap((id) => state.catalog?.badges.filter((badge) => badge.id === id) ?? []);\n  return (\n    <div className="quiz-results flex flex-col gap-double">\n',
     '  const badges = runAwards(state, run).flatMap((id) => state.catalog?.badges.filter((badge) => badge.id === id) ?? []);\n'
     '  const crowd = props.showAnswers === false ? undefined : chooseCrowd([], state.crowds[view.quiz]);\n'
     '  return (\n    <CrowdProvider crowd={crowd}>\n    <div className="quiz-results flex flex-col gap-double">\n'),
    ('        <p className="m-0 text-lg font-semibold tabular-nums">{text("quiz.results.score", { score: formatScore(result.score, locale) })}</p>\n',
     '        <p className="m-0 text-lg font-semibold tabular-nums">{text("quiz.results.score", { score: formatScore(result.score, locale) })}</p>\n'
     '        {crowd === undefined ? null : <CrowdSource crowd={crowd} text={text} />}\n'),
    ('            <TaskResultView task={task} result={taskResult} text={text} locale={locale} />\n          </QuizCard>\n        );\n      })}\n    </div>\n',
     '            <TaskResultView task={task} result={taskResult} crowd={crowd} text={text} locale={locale} />\n          </QuizCard>\n        );\n      })}\n    </div>\n    </CrowdProvider>\n'),
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
print("[crowd results] done")
