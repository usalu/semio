"""😀️ Shows every emoji of the quiz renderer in text presentation (the monochrome emoji face of the design system):
a `textPresentation` helper that swaps VS16 for VS15, used by the glyph and every emoji inside a sentence, and catalog
icons instead of arrow characters for the sort state of the leaderboard columns. Exact, unique anchors."""
import io, os, sys, time

root = sys.argv[1]
edits = {
    "🔨️modules/🪟️chrome/🟦️.tsx": [
        ("/** 😀️ An emoji shown as a glyph of the monochrome emoji face (`font-variant-emoji: text`, so ❄️ matches 🔥 and 🧲). */\nexport function Glyph(props: { readonly emoji: string; readonly className?: string }): ReactElement {\n  return (\n    <span aria-hidden=\"true\" className={cn(\"quiz-glyph shrink-0\", props.className)}>\n      {props.emoji}\n    </span>\n  );\n}",
         "/** 🔡️ `emoji` asking for text presentation: every emoji variation selector (VS16) becomes the text one (VS15), so a\n * sequence such as ❄️ renders from the monochrome emoji face like 🔥 and 🧲 instead of falling back to a colour font. */\nexport function textPresentation(emoji: string): string {\n  return emoji.replaceAll(\"\uFE0F\", \"\uFE0E\");\n}\n\n/** 😀️ An emoji shown as a glyph of the monochrome emoji face (text presentation, `font-variant-emoji: text`). */\nexport function Glyph(props: { readonly emoji: string; readonly className?: string }): ReactElement {\n  return (\n    <span aria-hidden=\"true\" className={cn(\"quiz-glyph shrink-0\", props.className)}>\n      {textPresentation(props.emoji)}\n    </span>\n  );\n}"),
    ],
    "🔨️modules/🏠️home/🟦️.tsx": [
        ("import { BodyButton, CardAction, CardIcon, Facts, Glyph, QuizCard } from \"../🪟️chrome/🟦️.tsx\";", "import { BodyButton, CardAction, CardIcon, Facts, Glyph, QuizCard, textPresentation } from \"../🪟️chrome/🟦️.tsx\";"),
        ("{ badges: earned.map((badge) => `${badge.emoji} ${localized(badge.label, locale)}`).join(\", \") }", "{ badges: earned.map((badge) => `${textPresentation(badge.emoji)} ${localized(badge.label, locale)}`).join(\", \") }"),
    ],
    "🔨️modules/🏆️leaderboard/🟦️.tsx": [
        ("import { CardAction, CardIcon, QuizCard, cn } from \"../🪟️chrome/🟦️.tsx\";", "import { Icon } from \"@semio-tech/ui-react/chrome\";\nimport { CardAction, CardIcon, QuizCard, cn, textPresentation } from \"../🪟️chrome/🟦️.tsx\";"),
        ("    return badge === undefined ? id : `${badge.emoji} ${localized(badge.label, locale)}`;", "    return badge === undefined ? id : `${textPresentation(badge.emoji)} ${localized(badge.label, locale)}`;"),
        ("                        <span aria-hidden=\"true\">{sort.key === column.key ? (sort.direction === \"ascending\" ? \" ▲\" : \" ▼\") : \" ↕\"}</span>",
         "                        <Icon icon={sort.key !== column.key ? \"chevrons-up-down\" : sort.direction === \"ascending\" ? \"chevron-up\" : \"chevron-down\"} size=\"small\" className={cn(\"shrink-0\", sort.key !== column.key && \"text-muted-foreground\")} />"),
    ],
}
for rel, pairs in edits.items():
    path = os.path.join(root, rel)
    text = io.open(path, encoding="utf-8", newline="").read()
    for old, new in pairs:
        if text.count(old) == 0 and text.count(new) == 1:
            continue
        if text.count(old) != 1:
            sys.exit(f"{rel}: anchor found {text.count(old)} times: {old[:70]!r}")
        text = text.replace(old, new)
    for attempt in range(20):
        try:
            with io.open(path, "w", encoding="utf-8", newline="") as handle:
                handle.write(text)
            break
        except OSError:
            time.sleep(0.25)
    else:
        sys.exit(f"{rel}: could not write")
    print("edited", rel)
