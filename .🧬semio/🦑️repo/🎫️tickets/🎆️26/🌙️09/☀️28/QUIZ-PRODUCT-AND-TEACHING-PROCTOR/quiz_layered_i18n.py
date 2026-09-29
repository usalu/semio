"""🌐️ Adds the chrome of the layered home (design §16) to both bundles of `🌐️i18n/🟦️.ts`: the overview's labels and the
open action (`quiz.home`), the preferences card's summary, and the learner and quiz pages (`quiz.learner`,
`quiz.quizPage`), in English and German (du). Exact anchors, each found once; writes once."""

import pathlib
import sys
import time

path = pathlib.Path("C:/git/semio/🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🔨️modules/🌐️i18n/🟦️.ts")
text = path.read_text(encoding="utf-8")
HOME = {
    "en": [("openPage", "Open"), ("cards", "Quizzes and pages"), ("pageWaiting", "{{page}} is loading"), ("pageFailed", "{{page}} could not be shown.")],
    "de": [("openPage", "Öffnen"), ("cards", "Quizze und Seiten"), ("pageWaiting", "{{page}} wird geladen"), ("pageFailed", "{{page}} konnte nicht angezeigt werden.")],
}
SUMMARY = {"en": "Language, colour theme, text size and others' cursors.", "de": "Sprache, Farbschema, Textgröße und die Cursor der anderen."}
GROUPS = {
    "en": {
        "learner": [
            ("runs", "Your runs"),
            ("quiz", "Quiz"),
            ("status", "Status"),
            ("score", "Score"),
            ("started", "Started"),
            ("submitted", "Submitted"),
            ("statusOpen", "In progress"),
            ("statusSubmitted", "Submitted"),
            ("statusVoided", "Voided"),
            ("noRuns", "No runs yet."),
            ("badges", "Your badges"),
            ("noBadges", "No badges yet."),
            ("viewResult", "View result"),
        ],
        "quizPage": [("tasks", "Tasks in this quiz"), ("hint", "Every run draws and shuffles the items anew. Your best run counts.")],
    },
    "de": {
        "learner": [
            ("runs", "Deine Durchgänge"),
            ("quiz", "Quiz"),
            ("status", "Status"),
            ("score", "Wertung"),
            ("started", "Begonnen"),
            ("submitted", "Abgegeben"),
            ("statusOpen", "Läuft"),
            ("statusSubmitted", "Abgegeben"),
            ("statusVoided", "Ungültig"),
            ("noRuns", "Noch keine Durchgänge."),
            ("badges", "Deine Abzeichen"),
            ("noBadges", "Noch keine Abzeichen."),
            ("viewResult", "Ergebnis ansehen"),
        ],
        "quizPage": [("tasks", "Aufgaben in diesem Quiz"), ("hint", "Jeder Durchgang zieht und mischt die Elemente neu. Dein bester Durchgang zählt.")],
    },
}
ANCHORS = {
    "en": ('      allBadges: phrase("All badges"),\n    },\n    task: {\n', '      cursors: phrase("Show others\' cursors"),\n'),
    "de": ('      allBadges: phrase("Alle Abzeichen"),\n    },\n    task: {\n', '      cursors: phrase("Cursor der anderen anzeigen"),\n'),
}
line = lambda key, value: f'      {key}: phrase("{value}"),\n'
for locale, (home_anchor, cursors_anchor) in ANCHORS.items():
    for anchor in (home_anchor, cursors_anchor):
        if text.count(anchor) != 1:
            sys.exit(f"{locale}: anchor found {text.count(anchor)} times: {anchor!r}")
    head = home_anchor.split("    },\n", 1)[0]
    groups = "".join(f"    {name}: {{\n" + "".join(line(key, value) for key, value in entries) + "    },\n" for name, entries in GROUPS[locale].items())
    text = text.replace(home_anchor, head + "".join(line(key, value) for key, value in HOME[locale]) + "    },\n" + groups + "    task: {\n")
    text = text.replace(cursors_anchor, cursors_anchor + line("summary", SUMMARY[locale]))
for attempt in range(40):
    try:
        path.write_text(text, encoding="utf-8", newline="")
        break
    except OSError:
        time.sleep(0.5)
else:
    sys.exit("write failed")
print("[layered i18n] done")
