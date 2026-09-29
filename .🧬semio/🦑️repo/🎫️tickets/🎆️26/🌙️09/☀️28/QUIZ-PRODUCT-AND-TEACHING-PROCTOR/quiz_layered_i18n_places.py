"""🌐️ Adds the words for the new presence places of design §16 (a quiz's page, the learner's profile, the badges page,
the preferences) to the `quiz.presence` group of both bundles in `🌐️i18n/🟦️.ts`, after `atResults`."""

import pathlib
import sys
import time

path = pathlib.Path("C:/git/semio/🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🔨️modules/🌐️i18n/🟦️.ts")
text = path.read_text(encoding="utf-8")
PLACES = {
    '      atResults: phrase("looking at the results of {{quiz}}"),\n': [
        ("atQuiz", "looking at {{quiz}}"),
        ("atLearner", "on their profile"),
        ("atBadges", "looking at the badges"),
        ("atPreferences", "in the settings"),
    ],
    '      atResults: phrase("schaut sich die Ergebnisse von {{quiz}} an"),\n': [
        ("atQuiz", "schaut sich {{quiz}} an"),
        ("atLearner", "im eigenen Profil"),
        ("atBadges", "schaut sich die Abzeichen an"),
        ("atPreferences", "in den Einstellungen"),
    ],
}
for anchor, entries in PLACES.items():
    if text.count(anchor) != 1:
        sys.exit(f"anchor found {text.count(anchor)} times: {anchor!r}")
    text = text.replace(anchor, anchor + "".join(f'      {key}: phrase("{value}"),\n' for key, value in entries))
for attempt in range(40):
    try:
        path.write_text(text, encoding="utf-8", newline="")
        break
    except OSError:
        time.sleep(0.5)
else:
    sys.exit("write failed")
print("[layered i18n places] done")
