"""🗳️ Honest crowd wording (design §17): the submitted crowd of the proctor counts every submitted run of the quiz,
the learner's own included, so it speaks of everyone and all runs — "What everyone answered", "All runs on Kettle:
…", "On average all runs put Horse at place 3 of 3", the results column "Everyone" — while the live crowd, which never
includes the own learner, keeps speaking of the others. Exact anchors, each found once."""

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


edit("🌐️i18n/🟦️.ts", [
    ('      pastTitle: phrase("What others answered"),\n', '      pastTitle: phrase("What everyone answered"),\n'),
    ('      item: phrase("The others on {{item}}: {{choices}}"),\n',
     '      item: phrase("The others on {{item}}: {{choices}}"),\n      itemAll: phrase("All runs on {{item}}: {{choices}}"),\n'),
    ('      position: phrase("On average the others put {{item}} at place {{position}} of {{total}}"),\n',
     '      position: phrase("On average the others put {{item}} at place {{position}} of {{total}}"),\n      positionAll: phrase("On average all runs put {{item}} at place {{position}} of {{total}}"),\n'),
    ('      others: phrase("The others"),\n', '      everyone: phrase("Everyone"),\n'),
    ('      pastTitle: phrase("Was die anderen geantwortet haben"),\n', '      pastTitle: phrase("Was alle geantwortet haben"),\n'),
    ('      item: phrase("Die anderen bei {{item}}: {{choices}}"),\n',
     '      item: phrase("Die anderen bei {{item}}: {{choices}}"),\n      itemAll: phrase("Alle Durchgänge bei {{item}}: {{choices}}"),\n'),
    ('      position: phrase("Im Schnitt setzen die anderen {{item}} auf Platz {{position}} von {{total}}"),\n',
     '      position: phrase("Im Schnitt setzen die anderen {{item}} auf Platz {{position}} von {{total}}"),\n      positionAll: phrase("Im Schnitt aller Durchgänge steht {{item}} auf Platz {{position}} von {{total}}"),\n'),
    ('      others: phrase("Die anderen"),\n', '      everyone: phrase("Alle"),\n'),
])
edit("🗳️crowd/🟦️.tsx", [
    (" * for a matching which values the others give it. The crowd shown is live — the\n"
     " * drafts of the learners thinking along in the same quiz right now, each in their presence colour — or, when nobody else\n"
     " * is, the submitted runs of that quiz. Every figure has a sentence for assistive technology; nothing here is a live\n"
     " * region, so updates never interrupt.\n",
     " * for a matching which values the others give it. The crowd shown is live — the\n"
     " * drafts of the learners thinking along in the same quiz right now, each in their presence colour — or, when nobody else\n"
     " * is, every submitted run of that quiz (the learner's own included, so it speaks of everyone). Every figure has a\n"
     " * sentence for assistive technology; nothing here is a live region, so updates never interrupt.\n"),
    ("  const { item, subject, label, text } = props;\n  if (item === undefined || item.choices.length === 0) return null;\n",
     "  const { item, subject, label, text } = props;\n  const live = useCrowd().crowd?.source !== \"submitted\";\n  if (item === undefined || item.choices.length === 0) return null;\n"),
    ('      <span className="sr-only">{text("quiz.crowd.item", { item: subject, choices })}</span>\n',
     '      <span className="sr-only">{text(live ? "quiz.crowd.item" : "quiz.crowd.itemAll", { item: subject, choices })}</span>\n'),
    ("  const { paint } = useCrowd();\n  const mean = meanPosition(item);\n",
     "  const { paint, crowd } = useCrowd();\n  const mean = meanPosition(item);\n"),
    ('      <span className="sr-only">{text("quiz.crowd.position", { item: subject, position, total })}</span>\n',
     '      <span className="sr-only">{text(crowd?.source === "submitted" ? "quiz.crowd.positionAll" : "quiz.crowd.position", { item: subject, position, total })}</span>\n'),
])
edit("🏁️results/🟦️.tsx", [
    ('text("quiz.results.credit"), ...(crowd === undefined ? [] : [text("quiz.crowd.others")])', 'text("quiz.results.credit"), ...(crowd === undefined ? [] : [text("quiz.crowd.everyone")])'),
    ('text("quiz.results.rank"), ...(crowd === undefined ? [] : [text("quiz.crowd.others")])', 'text("quiz.results.rank"), ...(crowd === undefined ? [] : [text("quiz.crowd.everyone")])'),
    ('...(others === undefined ? [] : [text("quiz.crowd.others")])', '...(others === undefined ? [] : [text("quiz.crowd.everyone")])'),
    (" * learner hides what the others think, every item stands beside the others: what the submitted runs answered.\n",
     " * learner hides what the others think, every item stands beside everyone: what all submitted runs answered.\n"),
])
print("[crowd wording] done")
