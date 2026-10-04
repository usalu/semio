"""🎛️ Adds the local-only "Show others' cursors" preference (design §15) to `🎛️preferences/🟦️.tsx`: the field (on by
default), its reading and a labelled checkbox under the text size; and lets the card carry a presence anchor."""

import pathlib
import sys
import time

path = pathlib.Path("C:/git/semio/🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🔨️modules/🎛️preferences/🟦️.tsx")
text = path.read_text(encoding="utf-8")
EDITS = [
    ("/** 🎛️ Persisted local-only display preferences: language, colour theme and text size.\n",
     "/** 🎛️ Persisted local-only display preferences: language, colour theme, text size and whether others' cursors show.\n"),
    ("  readonly textSize: TextSize;\n}\n",
     "  readonly textSize: TextSize;\n  readonly showCursors: boolean;\n}\n"),
    ("/** 💾️ The stored preferences, falling back to the system theme and normal text. */",
     "/** 💾️ The stored preferences, falling back to the system theme, normal text and others' cursors shown. */"),
    ('    textSize: TEXT_SIZES.some((size) => size.value === record.textSize) ? (record.textSize as TextSize) : "normal",\n  };\n',
     '    textSize: TEXT_SIZES.some((size) => size.value === record.textSize) ? (record.textSize as TextSize) : "normal",\n    showCursors: record.showCursors !== false,\n  };\n'),
    ("/** 🎛️ Language, theme and text size as three labelled segmented choices. */",
     "/** 🎛️ Language, theme and text size as three labelled segmented choices, and others' cursors as a checkbox. */"),
    ('        <Segments label={text("quiz.preferences.textSize")} options={TEXT_SIZES.map((size) => ({ value: size.value, label: text(size.label) }))} value={preferences.textSize} onChange={(textSize) => onChange({ ...preferences, textSize })} />\n      </div>\n    </div>\n',
     '        <Segments label={text("quiz.preferences.textSize")} options={TEXT_SIZES.map((size) => ({ value: size.value, label: text(size.label) }))} value={preferences.textSize} onChange={(textSize) => onChange({ ...preferences, textSize })} />\n      </div>\n      <label className="quiz-target flex w-fit cursor-pointer items-center gap-single text-sm">\n        <input type="checkbox" className="quiz-check" checked={preferences.showCursors} onChange={(event) => onChange({ ...preferences, showCursors: event.target.checked })} />\n        {text("quiz.preferences.cursors")}\n      </label>\n    </div>\n'),
    ("/** 🎛️ The preferences card of the home grid. */\nexport function PreferencesCard(props: { readonly preferences: QuizPreferences; readonly locale: QuizLocale; readonly text: QuizText; readonly onChange: (preferences: QuizPreferences) => void }): ReactElement {\n  const id = useId();\n  return (\n    <QuizCard id={id} card=\"preferences\" icon",
     "/** 🎛️ The preferences card of the home grid (and beside a first visit); `anchor` is its presence landmark. */\nexport function PreferencesCard(props: { readonly preferences: QuizPreferences; readonly locale: QuizLocale; readonly text: QuizText; readonly onChange: (preferences: QuizPreferences) => void; readonly anchor?: string }): ReactElement {\n  const id = useId();\n  return (\n    <QuizCard id={id} card=\"preferences\" anchor={props.anchor} icon"),
]
for old, new in EDITS:
    if text.count(old) != 1:
        sys.exit(f"anchor found {text.count(old)} times: {old[:80]!r}")
    text = text.replace(old, new)
for attempt in range(20):
    try:
        path.write_text(text, encoding="utf-8", newline="")
        break
    except OSError:
        time.sleep(0.25 * (attempt + 1))
print("[presence preferences] done")
