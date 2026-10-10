#!/usr/bin/env python3
"""📖️ Third-party ORACLE (pypdf) for the PDF 1.7 sheet set of the BIM room.

pypdf has never seen this repository's writer. It opens the committed file `🧫️fixtures/🚪️sheets/🏠️room/sheets.pdf`, counts the pages, converts the media box of every page from points to
millimetres (to a tenth), and extracts the text of every page with its own content-stream interpreter. The oracle also reads the committed room (the sheets of the model, in the order of their
numbers) and requires that the text of page `i` contains the number and the title of sheet `i`; the table it answers is `{pages: [{width, height, has_number, has_name}]}`.

    python 🐍️.py check <path to 🧫️fixtures/🚪️sheets>     # exit 1 on any disagreement
    python 🐍️.py write <path to 🧫️fixtures/🚪️sheets>     # rewrite the measured table from the committed file

@see ../../🔮️oracles/🔣️.json — the registration of the oracle this file answers for
@see ../../🚪️io/📤️export/📄️sheets/📖️pdf/🦀️.rs — the writer under test
"""

# region 🔖️Imports
import io
import json
import sys
from pathlib import Path

# endregion 🔖️Imports


# region 🔖️Reading
MM_PER_POINT = 25.4 / 72.0


def print_order(snapshot):
    """🔢️ The sheets of the model in print order: by number, then id."""
    return [row[2] for row in sorted((sheet["number"], key, sheet) for key, sheet in snapshot["sheets"].items())]


def measure(document, snapshot):
    """📖️ The table of the committed PDF: page sizes in millimetres to a tenth and whether the text of each page shows the number and the title of its sheet."""
    import pypdf

    reader = pypdf.PdfReader(io.BytesIO(document))
    sheets = print_order(snapshot)
    pages = []
    for index, page in enumerate(reader.pages):
        box = page.mediabox
        text = page.extract_text() or ""
        sheet = sheets[index] if index < len(sheets) else {"number": "", "name": ""}
        pages.append({
            "width": round(float(box.width) * MM_PER_POINT, 1),
            "height": round(float(box.height) * MM_PER_POINT, 1),
            "has_number": bool(sheet["number"]) and sheet["number"] in text,
            "has_name": bool(sheet["name"]) and sheet["name"] in text,
        })
    return {"pages": pages}


def audit(document, snapshot, table):
    """🩺️ One page per sheet, and no page without text."""
    import pypdf

    problems = []
    reader = pypdf.PdfReader(io.BytesIO(document))
    if len(reader.pages) != len(snapshot["sheets"]):
        problems.append("%d pages for %d sheets" % (len(reader.pages), len(snapshot["sheets"])))
    for index, page in enumerate(reader.pages):
        if not (page.extract_text() or "").strip():
            problems.append("page %d has no text" % (index + 1))
    return problems


# endregion 🔖️Reading


# region 🔖️Handlers
def export_handler(ctx):
    """📖️ Oracle answer: the table of the committed file."""
    from semio_repo_test import Outcome

    pdf = next(uri for uri in ctx.step_input_uris() if uri.endswith(".pdf"))
    snapshot_uri = next(uri for uri in ctx.step_input_uris() if "📸️snapshot" in uri)
    snapshot = json.loads(ctx.input_bytes(snapshot_uri).decode("utf-8"))
    table = measure(ctx.input_bytes(pdf), snapshot)
    problems = audit(ctx.input_bytes(pdf), snapshot, table)
    if problems:
        raise AssertionError("; ".join(problems))
    return Outcome(table, raw=json.dumps(table, separators=(",", ":"), ensure_ascii=False).encode("utf-8"))


def adapter():
    """🧭️ Registration in the ORACLE role only, by the feature's scenario id."""
    from semio_repo_test import Adapter

    return Adapter("python").oracle("export-sheets-pdf-room", export_handler)


# endregion 🔖️Handlers


# region 🔖️Standalone
def main(arguments):
    """🏃️ `check` audits the committed file against the committed table; `write` rewrites the table from the file."""
    command, root = arguments[0], Path(arguments[1])
    problems = []
    for case in sorted(path for path in root.iterdir() if path.is_dir()):
        document = (case / "sheets.pdf").read_bytes()
        snapshot = json.loads(root.parent.joinpath("💡️inferences", "📄️sheet-layout", case.name, "📸️snapshot", "🔣️.json").read_text(encoding="utf-8"))
        table = measure(document, snapshot)
        problems += audit(document, snapshot, table)
        path = case / "🔬️measure-pdf" / "🔣️.json"
        if command == "write":
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text(json.dumps(table, indent=2, ensure_ascii=False) + "\n", encoding="utf-8", newline="\n")
            print("%s: wrote the table of %d pages" % (case.name, len(table["pages"])))
        elif json.loads(path.read_text(encoding="utf-8")) != table:
            problems.append("%s: the committed table differs from the measurement of the committed file" % case.name)
    for problem in problems:
        print("[FAIL] %s" % problem)
    print("%s: %s" % (command, "%d problem(s)" % len(problems) if problems else "oracle agrees"))
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))

# endregion 🔖️Standalone
