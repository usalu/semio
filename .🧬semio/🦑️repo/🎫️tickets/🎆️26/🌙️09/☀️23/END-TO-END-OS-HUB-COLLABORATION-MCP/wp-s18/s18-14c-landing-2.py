# -*- coding: utf-8 -*-
"""📝️ S18 §14c: landing row for the text editor's localized accessible name (idempotent)."""
import pathlib

LANDING = pathlib.Path("/Users/ueli/Documents/semio/.tmp-ticket/📓️landing.md")
ROW = "| S18 | 14c a11y/i18n — the text editor's accessible name was hardcoded English (`${language} editor` / `Editor`): now `ui.host.languageEditor` (en \"{{language}} editor\", de \"{{language}}-Editor\") / `ui.host.editor` (\"Editor\") — `📚️I18n` `UiTranslationSchema` + both `⚛️react` bundles (`🧰️framework/🔨️modules/🖱️ui`), `💻️os/…/🧱️elements/✏️TextEditor/🟦️.tsx` (`WasmEditorSurface`); codemod `wp-s18/s18-14c-editor-label.py` | TS only (host + ui) | tsc 0 (`wp-s18/generated/s18-14c-tsc-editor-1.txt`), boot Home 9 s 0 pageerrors (`s18-14c-boot-3.txt`); live xml editor textarea named \"xml editor\" (en-US) / \"xml-Editor\" (de-DE), 0 pageerrors (`s18-14c-editor-label-{en-US,de-DE}.txt`) | 28 19:0x |"


def main() -> None:
    text = LANDING.read_text(encoding="utf-8")
    if ROW[:70] not in text:
        LANDING.write_text(text.rstrip("\n") + "\n" + ROW + "\n", encoding="utf-8")
        print("added 1")
    else:
        print("added 0")


main()
