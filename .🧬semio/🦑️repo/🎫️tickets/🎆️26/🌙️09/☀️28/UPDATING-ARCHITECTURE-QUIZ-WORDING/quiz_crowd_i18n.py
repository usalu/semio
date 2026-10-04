"""🌐️ Adds the chrome of sharing within the quizzes (design §17) to both bundles of `🌐️i18n/🟦️.ts`: the preference to
hide what others think, and the `quiz.crowd` group (live and submitted answers of the others per item, you versus the
others on the results), in English and German (du). Exact anchors, each found once; writes once."""

import pathlib
import sys
import time

path = pathlib.Path("C:/git/semio/🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🔨️modules/🌐️i18n/🟦️.ts")
text = path.read_text(encoding="utf-8")
PREFERENCE = {"en": "Show what others think", "de": "Zeigen, was die anderen denken"}
CROWD = {
    "en": [
        ("liveTitle", "What others think now"),
        ("pastTitle", "What others answered"),
        ("thinking", "Thinking along: {{count}}"),
        ("runs", "Submitted runs: {{count}}"),
        ("choice", "{{label}} {{count}}×"),
        ("item", "The others on {{item}}: {{choices}}"),
        ("position", "On average the others put {{item}} at place {{position}} of {{total}}"),
        ("positionShort", "⌀ place {{position}}"),
        ("nobody", "Nobody has answered this yet."),
        ("versus", "You and the others"),
        ("you", "You"),
        ("others", "The others"),
        ("drag", "{{name}} drags {{item}}"),
    ],
    "de": [
        ("liveTitle", "Was die anderen gerade denken"),
        ("pastTitle", "Was die anderen geantwortet haben"),
        ("thinking", "Denken gerade mit: {{count}}"),
        ("runs", "Abgegebene Durchgänge: {{count}}"),
        ("choice", "{{label}} {{count}}×"),
        ("item", "Die anderen bei {{item}}: {{choices}}"),
        ("position", "Im Schnitt setzen die anderen {{item}} auf Platz {{position}} von {{total}}"),
        ("positionShort", "⌀ Platz {{position}}"),
        ("nobody", "Dazu hat noch niemand geantwortet."),
        ("versus", "Du und die anderen"),
        ("you", "Du"),
        ("others", "Die anderen"),
        ("drag", "{{name}} zieht {{item}}"),
    ],
}
ANCHORS = {
    "en": ('      summary: phrase("Language, colour theme, text size and others\' cursors."),\n', '    presence: {\n      online: phrase("Online: {{count}}"),\n'),
    "de": ('      summary: phrase("Sprache, Farbschema, Textgröße und die Cursor der anderen."),\n', '    presence: {\n      online: phrase("Online: {{count}}"),\n'),
}
line = lambda key, value: f'      {key}: phrase("{value}"),\n'
for locale, (preference_anchor, presence_anchor) in ANCHORS.items():
    if text.count(preference_anchor) != 1:
        sys.exit(f"{locale}: preference anchor found {text.count(preference_anchor)} times")
    text = text.replace(preference_anchor, preference_anchor + line("answers", PREFERENCE[locale]))
    start = text.index(preference_anchor)
    at = text.index(presence_anchor, start)
    group = "    crowd: {\n" + "".join(line(key, value) for key, value in CROWD[locale]) + "    },\n"
    text = text[:at] + group + text[at:]
for attempt in range(40):
    try:
        path.write_text(text, encoding="utf-8", newline="")
        break
    except OSError:
        time.sleep(0.5)
else:
    sys.exit("write failed")
print("[crowd i18n] done")
