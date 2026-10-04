"""🔤️ Keeps table headings and numbers of the quiz renderer on one line (no mid-word breaks such as "Ran k"): the
`quiz-nowrap` class on every column heading and numeric cell, `quiz-name` on learner names (the only cells that wrap),
and table cells out of the renderer's `overflow-wrap: anywhere` rule. Exact, unique anchors; refuses otherwise."""
import io, os, sys

root = sys.argv[1]
edits = {
    "🎨️.css": [
        (".quiz-app :where(p, h1, h2, h3, h4, li, label, legend, summary, figcaption, dd, dt, th, td) {\n  overflow-wrap: anywhere;\n}",
         ".quiz-app :where(p, h1, h2, h3, h4, li, label, legend, summary, figcaption, dd, dt) {\n  overflow-wrap: anywhere;\n}\n\n/* 🔤️ Table headings and numbers never break inside a word (the table scrolls in its card instead); learner names are\n   the one column that wraps, anywhere if a handle is one long word. */\n.quiz-nowrap {\n  white-space: nowrap;\n  hyphens: none;\n  overflow-wrap: normal;\n}\n\n.quiz-name {\n  hyphens: none;\n  overflow-wrap: anywhere;\n}"),
    ],
    "🔨️modules/🏆️leaderboard/🟦️.tsx": [
        ('const CELL = "px-single py-single align-top";', 'const CELL = "px-single py-single align-top";\nconst NUMBER = "quiz-nowrap tabular-nums";'),
        ('      <td className={cn(CELL, "tabular-nums")}>{row.rank}</td>\n      <th scope="row" className={cn(CELL, "text-left", mine ? "font-semibold" : "font-normal")}>',
         '      <td className={cn(CELL, NUMBER)}>{row.rank}</td>\n      <th scope="row" className={cn(CELL, "quiz-name text-left", mine ? "font-semibold" : "font-normal")}>'),
        ('      <td className={cn(CELL, "text-right tabular-nums")}>{formatPoints(row.total, locale)}</td>\n      <td className={cn(CELL, "text-right tabular-nums")}>{row.badges.length}</td>',
         '      <td className={cn(CELL, NUMBER, "text-right")}>{formatPoints(row.total, locale)}</td>\n      <td className={cn(CELL, NUMBER, "text-right")}>{row.badges.length}</td>'),
        ('  const head = cn(CELL, "font-medium");', '  const head = cn(CELL, "quiz-nowrap font-medium");'),
        ('                    <th key={column.key} scope="col" aria-sort={sort.key === column.key ? sort.direction : undefined} className={cn(edge, "border-b-2")}>',
         '                    <th key={column.key} scope="col" aria-sort={sort.key === column.key ? sort.direction : undefined} className={cn(edge, "quiz-nowrap border-b-2")}>'),
        ('                    <td className={cn(edge, "tabular-nums")}>{row.rank}</td>\n                    <th scope="row" className={cn(edge, row.tag === mine ? "font-semibold" : "font-normal")}>',
         '                    <td className={cn(edge, NUMBER)}>{row.rank}</td>\n                    <th scope="row" className={cn(edge, "quiz-name", row.tag === mine ? "font-semibold" : "font-normal")}>'),
        ('                    <td className={cn(edge, "tabular-nums")}>{formatPoints(row.total, locale)}</td>', '                    <td className={cn(edge, NUMBER)}>{formatPoints(row.total, locale)}</td>'),
        ('                      <td key={quiz.id} className={cn(edge, "tabular-nums")}>', '                      <td key={quiz.id} className={cn(edge, NUMBER)}>'),
        ('                      <span className="tabular-nums">{row.badges.length}</span>', '                      <span className="quiz-nowrap tabular-nums">{row.badges.length}</span>'),
        ('                    <td className={cn(edge, "tabular-nums")}>{row.runs}</td>\n                    <td className={edge}>{formatInstant(row.lastActivity, locale)}</td>',
         '                    <td className={cn(edge, NUMBER)}>{row.runs}</td>\n                    <td className={cn(edge, "quiz-nowrap")}>{formatInstant(row.lastActivity, locale)}</td>'),
    ],
    "🔨️modules/🏁️results/🟦️.tsx": [
        ('const HEAD = cn(EDGE, "border-b-2 font-semibold");', 'const HEAD = cn(EDGE, "quiz-nowrap border-b-2 font-semibold");\nconst NUMBER = cn(EDGE, "quiz-nowrap tabular-nums");'),
        ('            <td className={cn(EDGE, "tabular-nums")}>{item.position + 1}</td>', '            <td className={NUMBER}>{item.position + 1}</td>'),
        ('            <td className={cn(EDGE, "tabular-nums")}>{value(item.value)}</td>', '            <td className={NUMBER}>{value(item.value)}</td>'),
        ('                  <td className={cn(EDGE, "tabular-nums")}>{value(item.correct)}</td>', '                  <td className={NUMBER}>{value(item.correct)}</td>'),
    ],
    "🔨️modules/🃏️matching/🟦️.tsx": [
        ('const HEAD = `${EDGE} border-b-2 font-semibold`;', 'const HEAD = `${EDGE} quiz-nowrap border-b-2 font-semibold`;'),
    ],
    "🔨️modules/🕸️radar/🟦️.tsx": [
        ('const VALUE_CELL = "border-b border-normal px-single py-single text-left align-top tabular-nums";\nconst VALUE_HEAD = "border-b-2 border-normal px-single py-single text-left align-top font-semibold";',
         'const VALUE_CELL = "quiz-nowrap border-b border-normal px-single py-single text-left align-top tabular-nums";\nconst VALUE_HEAD = "quiz-nowrap border-b-2 border-normal px-single py-single text-left align-top font-semibold";'),
    ],
}
for rel, pairs in edits.items():
    path = os.path.join(root, rel)
    text = io.open(path, encoding="utf-8", newline="").read()
    for old, new in pairs:
        if text.count(old) != 1:
            sys.exit(f"{rel}: anchor found {text.count(old)} times: {old[:70]!r}")
        text = text.replace(old, new)
    io.open(path, "w", encoding="utf-8", newline="").write(text)
    print("edited", rel)
