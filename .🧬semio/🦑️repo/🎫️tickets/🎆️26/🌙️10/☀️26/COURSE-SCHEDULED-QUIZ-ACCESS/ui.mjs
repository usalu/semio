import fs from "node:fs";
const core = "🧰️framework/🛍️products/📚️course";
const ui = `${core}/🎯️targets/⚛️react/🔨️modules`;
const edit = (path, change) => { const before = fs.readFileSync(path, "utf8"); const after = change(before); if (after === before) throw new Error(path); fs.writeFileSync(path, after); };
for (const [path, labels] of [["🔠️english-bundle", 'locked: phrase("Not available yet"), opens: phrase("Unlocks {{date}}")'], ["🔡️german-bundle", 'locked: phrase("Noch nicht freigeschaltet"), opens: phrase("Freigeschaltet ab {{date}}")']]) edit(`${ui}/🌐️i18n/${path}/🟦️.ts`, text => text.replace("  course: {", `  course: {\n    access: { ${labels} },`));
edit(`${ui}/🏠️home/🟦️.tsx`, text => text
  .replace('import { createContext,', 'import { AccessGate, clientAccess, useReleaseClock } from "../🔓️access/🟦️.tsx";\nimport { createContext,')
  .replace("interface HomeOverviewProps extends HomeProps {", "interface HomeOverviewProps extends HomeProps {\n  readonly at: number;")
  .replace('render: (pane) => {\n', 'render: (pane) => {\n        const content = (): ReactNode => {\n')
  .replace('      },\n    };\n  });\n  const cells', '        };\n        return <AccessGate course={course} page={page} at={props.at} locale={locale} text={text}>{content()}</AccessGate>;\n      },\n    };\n  });\n  const cells')
  .replace('const shown = { revealed: card.revealed, onOpen: card.open };', 'const shown = { revealed: card.revealed, onOpen: clientAccess(course, pane.id, props.at) ? card.open : undefined };')
  .replace('const renderCard = (pane: LayeredPane, card: LayeredCardState): ReactNode => {', 'const renderCard = (pane: LayeredPane, card: LayeredCardState): ReactNode => {\n    const gatedCard = <AccessGate course={course} page={pane.id} at={props.at} locale={locale} text={text}>{cardOf(pane, card)}</AccessGate>;')
  .replaceAll('{cardOf(pane, card)}{activity}', '{gatedCard}{activity}')
  .replace('onOpenedIdChange={(page) => session.open(', 'onOpenedIdChange={(page) => (page === null || clientAccess(course, page, props.at)) && session.open(')
  .replace('const { session, state, text } = props;\n  const previewPointer', 'const { session, state, text } = props;\n  const at = useReleaseClock(state.course);\n  const previewPointer')
  .replace('<HomeOverview {...props} enabled busy=', '<HomeOverview {...props} at={at} enabled busy='));
edit(`${ui}/📖️quiz-page/🟦️.tsx`, text => text
  .replace('import { useEffect,', 'import { clientAccess } from "../🔓️access/🟦️.tsx";\nimport { useEffect,')
  .replace('readonly onOpen: () => void', 'readonly onOpen?: () => void')
  .replace('href={`#${quiz.id}`}', 'href={onOpen === undefined ? undefined : `#${quiz.id}`}')
  .replace('const { quiz, session, state, text, busy, act, challenge } = props;', 'const { quiz, session, state, text, busy, act, challenge } = props;\n  const locked = !clientAccess(state.course, quiz.id, Date.now());')
  .replaceAll('disabled={busy}', 'disabled={busy || locked}'));
edit(`${ui}/▶️run/🟦️.tsx`, text => text
  .replace('import {', 'import { AccessGate, clientAccess, useReleaseClock } from "../🔓️access/🟦️.tsx";\nimport {')
  .replace('  const scope = useId();', '  const scope = useId();\n  const at = useReleaseClock(state.course);')
  .replace('onClick={() => go(position)}', 'disabled={!clientAccess(state.course, view.quiz, at, entry.id)}\n                    onClick={() => go(position)}')
  .replace('    moved.current = true;\n    setCurrent(Math.max(0, Math.min(tasks.length - 1, target)));', '    const direction = target < index ? -1 : 1;\n    let next = target;\n    while (next >= 0 && next < tasks.length && !clientAccess(state.course, view.quiz, at, tasks[next]!.id)) next += direction;\n    if (next < 0 || next >= tasks.length) return;\n    moved.current = true;\n    setCurrent(next);')
  .replace('    <div className="course-run flex flex-col gap-double">', '    <AccessGate course={state.course} page={view.quiz} at={at} locale={locale} text={text}>\n    <div className="course-run flex flex-col gap-double">')
  .replace('      {task === undefined ? null : (\n        <CourseCard', '      {task === undefined ? null : (\n        <AccessGate course={state.course} page={view.quiz} task={task.id} at={at} locale={locale} text={text}>\n        <CourseCard')
  .replace('      {submission === undefined ? null : (', '      {submission === undefined ? null : (')
  .replace('        </CourseCard>\n      )}\n      {submission', '        </CourseCard>\n        </AccessGate>\n      )}\n      {submission')
  .replace('    </div>\n  );\n}', '    </div>\n    </AccessGate>\n  );\n}'));
