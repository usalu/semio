"""🌐️ Adds the presence chrome of the quiz client (design §15) to both bundles of `🌐️i18n/🟦️.ts`: the cursor preference
and the `quiz.presence` group, in English and in German (du). Exact anchors, each found once; writes once."""

import pathlib
import sys
import time

path = pathlib.Path("C:/git/semio/🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🔨️modules/🌐️i18n/🟦️.ts")
text = path.read_text(encoding="utf-8")
PRESENCE = {
    "en": [
        ("online", "Online: {{count}}"),
        ("whoIsWhere", "Who is where"),
        ("you", "you"),
        ("away", "away"),
        ("onlineMark", "online"),
        ("learningNow", "Learning now: {{count}}"),
        ("atIntroduction", "reading the introduction"),
        ("atIdentity", "choosing a name"),
        ("atHome", "on the overview"),
        ("atLeaderboard", "on the leaderboard"),
        ("atRun", "working on {{quiz}}"),
        ("atResults", "looking at the results of {{quiz}}"),
    ],
    "de": [
        ("online", "Online: {{count}}"),
        ("whoIsWhere", "Wer ist wo"),
        ("you", "du"),
        ("away", "abwesend"),
        ("onlineMark", "online"),
        ("learningNow", "Gerade dabei: {{count}}"),
        ("atIntroduction", "liest die Einführung"),
        ("atIdentity", "wählt einen Namen"),
        ("atHome", "in der Übersicht"),
        ("atLeaderboard", "in der Rangliste"),
        ("atRun", "arbeitet an {{quiz}}"),
        ("atResults", "schaut sich die Ergebnisse von {{quiz}} an"),
    ],
}
CURSORS = {"en": "Show others' cursors", "de": "Cursor der anderen anzeigen"}
EDITS = [
    ('      german: phrase("Deutsch"),\n    },\n    introduction: {\n      continue: phrase("Continue"),\n', "en", "preferences"),
    ('      german: phrase("Deutsch"),\n    },\n    introduction: {\n      continue: phrase("Weiter"),\n', "de", "preferences"),
    ('      full: phrase("Full leaderboard"),\n    },\n    rejection: {\n', "en", "presence"),
    ('      full: phrase("Ganze Rangliste"),\n    },\n    rejection: {\n', "de", "presence"),
]
for anchor, locale, kind in EDITS:
    if text.count(anchor) != 1:
        sys.exit(f"{locale} {kind}: anchor found {text.count(anchor)} times")
    if kind == "preferences":
        head = '      german: phrase("Deutsch"),\n'
        replacement = head + f'      cursors: phrase("{CURSORS[locale]}"),\n' + anchor[len(head):]
    else:
        head, tail = anchor.split("    },\n", 1)
        group = "".join(f'      {key}: phrase("{value}"),\n' for key, value in PRESENCE[locale])
        replacement = head + "    },\n    presence: {\n" + group + "    },\n" + tail
    text = text.replace(anchor, replacement)
for attempt in range(20):
    try:
        path.write_text(text, encoding="utf-8", newline="")
        break
    except OSError:
        time.sleep(0.25 * (attempt + 1))
print("[presence i18n] done")
