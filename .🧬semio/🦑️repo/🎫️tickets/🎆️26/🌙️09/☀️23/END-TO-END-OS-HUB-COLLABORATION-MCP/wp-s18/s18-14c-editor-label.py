# -*- coding: utf-8 -*-
"""S18 14c: localize the text editor's accessible name (was hardcoded English `${language} editor` / `Editor`)."""
import pathlib

TEXT = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/✏️TextEditor/🟦️.tsx")
I18N = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/📚️I18n/🟦️.tsx")
BUNDLES = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx")

EDITS = [
    (TEXT, '  const contextMenuTitleLabel = useLabel(contextMenu?.titleKey ?? "ui.surfaceContextMenu.editor");\n',
     '  const contextMenuTitleLabel = useLabel(contextMenu?.titleKey ?? "ui.surfaceContextMenu.editor");\n  const editorLabel = useLabel(scene.language ? "ui.host.languageEditor" : "ui.host.editor", { language: scene.language });\n'),
    (TEXT, '        aria-label={scene.language ? `${scene.language} editor` : "Editor"}\n', '        aria-label={editorLabel}\n'),
    (I18N, "      readonly languageDocument: UiLabelValue;\n", "      readonly languageDocument: UiLabelValue;\n      readonly editor: UiLabelValue;\n      readonly languageEditor: UiLabelValue;\n"),
    (BUNDLES, '          languageDocument: { label: { normal: "{{language}}-Dokument", beginner: "{{language}}-Dokument" } },\n',
     '          languageDocument: { label: { normal: "{{language}}-Dokument", beginner: "{{language}}-Dokument" } },\n          editor: { label: { normal: "Editor", beginner: "Editor" } },\n          languageEditor: { label: { normal: "{{language}}-Editor", beginner: "{{language}}-Editor" } },\n'),
    (BUNDLES, '          languageDocument: { label: { normal: "{{language}} document", beginner: "{{language}} document" } },\n',
     '          languageDocument: { label: { normal: "{{language}} document", beginner: "{{language}} document" } },\n          editor: { label: { normal: "Editor", beginner: "Editor" } },\n          languageEditor: { label: { normal: "{{language}} editor", beginner: "{{language}} editor" } },\n'),
]


def main() -> None:
    texts = {path: path.read_text(encoding="utf-8") for path in {edit[0] for edit in EDITS}}
    for path, old, new in EDITS:
        if new in texts[path]:
            continue
        assert texts[path].count(old) == 1, (path.name, old[:60])
        texts[path] = texts[path].replace(old, new)
    for path, text in texts.items():
        path.write_text(text, encoding="utf-8")
    print("ok")


main()
