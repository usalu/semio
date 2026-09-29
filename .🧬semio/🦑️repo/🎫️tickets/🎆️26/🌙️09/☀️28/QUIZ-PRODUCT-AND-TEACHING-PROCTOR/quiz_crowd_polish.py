"""🗳️ Crowd polish (design §17): the crowd colours learners with the presence module's own paint (one appearance rule),
the sorting sentence names the item, the quiz page says when nobody has answered yet, and the unused crowd strings
(`versus`, `you`, `drag` — the peer's drag label is decorative) leave both bundles. Exact anchors, each found once."""

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


edit("👥️presence/🟦️.tsx", [
    ("function paintStyle(colour: number): CSSProperties {",
     "/** 🎨️ The custom property `--quiz-peer` that colours a learner of roster slot `colour` in the current appearance. */\n"
     "export function paintStyle(colour: number): CSSProperties {"),
])
edit("🗳️crowd/🟦️.tsx", [
    ('import { presencePaint } from "@semio-tech/ui-react/chrome";\n', ""),
    ('import { cn } from "../🪟️chrome/🟦️.tsx";\n', 'import { cn } from "../🪟️chrome/🟦️.tsx";\nimport { paintStyle } from "../👥️presence/🟦️.tsx";\n'),
    ("  readonly paint: (tag: string) => string | undefined;\n", "  readonly paint: (tag: string) => CSSProperties | undefined;\n"),
    ("  const dark = typeof document !== \"undefined\" && document.documentElement.classList.contains(\"dark\");\n"
     "  const paint = (tag: string): string | undefined => {\n"
     "    const colour = colours?.get(tag);\n"
     "    return colour === undefined ? undefined : presencePaint(colour, dark ? \"dark\" : \"light\");\n"
     "  };\n",
     "  const paint = (tag: string): CSSProperties | undefined => {\n"
     "    const colour = colours?.get(tag);\n"
     "    return colour === undefined ? undefined : paintStyle(colour);\n"
     "  };\n"),
    ("    const colour = paint(tag);\n    return colour === undefined ? [] : [{ tag, colour }];\n",
     "    const style = paint(tag);\n    return style === undefined ? [] : [{ tag, style }];\n"),
    ("      {painted.map(({ tag, colour }) => (\n"
     "        <span key={tag} className=\"quiz-online !m-0\" style={{ [\"--quiz-peer\" as string]: colour } as CSSProperties} />\n",
     "      {painted.map(({ tag, style }) => (\n"
     "        <span key={tag} className=\"quiz-online !m-0\" style={style} />\n"),
    ("style={{ [\"--quiz-crowd-at\" as string]: String(entry.position), ...(entry.tag === undefined || paint(entry.tag) === undefined ? {} : { [\"--quiz-peer\" as string]: paint(entry.tag) }) } as CSSProperties}",
     "style={{ [\"--quiz-crowd-at\" as string]: String(entry.position), ...(entry.tag === undefined ? {} : paint(entry.tag)) } as CSSProperties}"),
])
edit("🌐️i18n/🟦️.ts", [
    ('      position: phrase("The others\' average place: {{position}} of {{total}}"),\n',
     '      position: phrase("On average the others put {{item}} at place {{position}} of {{total}}"),\n'),
    ('      versus: phrase("You and the others"),\n      you: phrase("You"),\n', ""),
    ('      drag: phrase("{{name}} drags {{item}}"),\n', ""),
    ('      position: phrase("Durchschnittlicher Platz bei den anderen: {{position}} von {{total}}"),\n',
     '      position: phrase("Im Schnitt setzen die anderen {{item}} auf Platz {{position}} von {{total}}"),\n'),
    ('      versus: phrase("Du und die anderen"),\n      you: phrase("Du"),\n', ""),
    ('      drag: phrase("{{name}} zieht {{item}}"),\n', ""),
])
edit("📖️quiz-page/🟦️.tsx", [
    ("/** 👥️ What the others think of the quiz, task by task and item by item (labelled from the learner's own sheet); only\n"
     " * where the crowd comes from while the learner has no sheet of it yet. */\n"
     "function QuizCrowd(props: QuizProps & { readonly crowd: Crowd | undefined }): ReactElement | null {\n"
     "  const { quiz, state, text, locale, crowd } = props;\n"
     "  const id = useId();\n"
     "  const sheet = sheetOfQuiz(state, quiz.id);\n"
     "  if (crowd === undefined) return null;\n",
     "/** 👥️ What the others think of the quiz, task by task and item by item (labelled from the learner's own sheet); only\n"
     " * where the crowd comes from while the learner has no sheet of it yet, and that nobody answered while nobody did.\n"
     " * Nothing while the learner hides what the others think (`shown`). */\n"
     "function QuizCrowd(props: QuizProps & { readonly crowd: Crowd | undefined; readonly shown: boolean }): ReactElement | null {\n"
     "  const { quiz, state, text, locale, crowd } = props;\n"
     "  const id = useId();\n"
     "  const sheet = sheetOfQuiz(state, quiz.id);\n"
     "  if (!props.shown) return null;\n"
     "  if (crowd === undefined)\n"
     "    return (\n"
     "      <QuizCard id={`${id}-crowd`} card=\"quiz-crowd\" icon={<CardIcon icon=\"users\" />} title={text(\"quiz.crowd.pastTitle\")}>\n"
     "        <p className=\"m-0 text-sm text-muted-foreground\">{text(\"quiz.crowd.nobody\")}</p>\n"
     "      </QuizCard>\n"
     "    );\n"),
    ("        <QuizCrowd {...props} crowd={crowd} />\n", "        <QuizCrowd {...props} crowd={crowd} shown={props.showAnswers !== false} />\n"),
])
print("[crowd polish] done")
