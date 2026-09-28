# -*- coding: utf-8 -*-
"""S18 14c: the csv/tsv set-cell pin follows the row-scoped DOM ids (LB2 p8) and the windowed structural table the restaged stdio renders."""
import pathlib

DEV = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev")
EDITS = [
    (DEV / "🧫️fixtures/🧮️program-matrix.json",
     '"stdio.set-cell": { "control": "[id$=\\"::framework.window.table.0.0\\"]", "value": "Matrix Cell" }',
     '"stdio.set-cell": { "control": "[id$=\\"/row-0/cell-0\\"]", "value": "Matrix Cell" }'),
    (DEV / "🧪️tests/🧮️program-matrix/🧮️reducers/🟦️.ts",
     """      <table><tr><td><input id="spawned:stdio-2::framework.window.table.0.0" aria-label="name" value="alpha"></td><td><input id="spawned:stdio-2::framework.window.table.0.1" aria-label="note" value="plain"></td></tr></table>""",
     """      <table><tr><td><input id="spawned:stdio-2::framework.window.table/header-0/cell-0" aria-label="Header" value="name"></td></tr><tr><td><input id="spawned:stdio-2::framework.window.table/row-0/cell-0" aria-label="name" value="alpha"></td><td><input id="spawned:stdio-2::framework.window.table/row-0/cell-1" aria-label="note" value="plain"></td></tr><tr><td><input id="spawned:stdio-2::framework.window.table/row-1/cell-0" aria-label="name" value="Doe, John"></td></tr></table>"""),
    (DEV / "🧪️tests/🧮️program-matrix/🧮️reducers/🟦️.ts",
     """    expect(hit(pins.pluginEdits["stdio.set-cell"]!.control)).toEqual(["spawned:stdio-2::framework.window.table.0.0"]);""",
     """    expect(hit(pins.pluginEdits["stdio.set-cell"]!.control)).toEqual(["spawned:stdio-2::framework.window.table/row-0/cell-0"]);"""),
    (DEV / "🧪️tests/🧮️program-matrix/🧮️reducers/🟦️.ts",
     " * each rendered edit's control selects exactly the inline input the editor renders for it (jsdom is the oracle; the table\n * and JSON tree controls as measured live on serve 6540, the XML text node's input as its `render_editor` builds it).",
     " * each rendered edit's control selects exactly the inline input the editor renders for it (jsdom is the oracle; the table\n * and JSON tree controls as measured live on serve 6540 with row-scoped DOM ids, the XML text node's input as its\n * `render_editor` builds it)."),
]


def main() -> None:
    for path, old, new in EDITS:
        text = path.read_text(encoding="utf-8")
        if new in text:
            continue
        assert text.count(old) == 1, (path.name, old[:70])
        path.write_text(text.replace(old, new), encoding="utf-8")
    print("ok")


main()
