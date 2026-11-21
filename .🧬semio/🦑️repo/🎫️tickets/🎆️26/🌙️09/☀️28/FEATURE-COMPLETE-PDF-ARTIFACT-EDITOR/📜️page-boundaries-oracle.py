"""📐 Independently checks page boundary edits with MuPDF."""
import json
from pathlib import Path
import sys

import fitz


def pdf_rect(value):
    return "[" + " ".join(str(item) for item in value) + "]"


def main():
    case = json.loads(Path(sys.argv[1]).read_text())["pageBoundaries"]
    document = fitz.open()
    page = document.new_page(width=case["initialSize"][0], height=case["initialSize"][1])
    for edit in case["edits"]:
        coordinates = [float(value) for value in edit["coordinates"].split()]
        assert coordinates == edit["value"]
        document.xref_set_key(page.xref, edit["key"], pdf_rect(coordinates))
    document.xref_set_key(page.xref, "UserUnit", str(case["userUnit"]))
    document = fitz.open(stream=document.tobytes(), filetype="pdf")
    page = document[0]
    for edit in case["edits"]:
        kind, raw = document.xref_get_key(page.xref, edit["key"])
        assert kind == "array" and [float(value) for value in raw[1:-1].split()] == edit["value"], (kind, raw)
    assert (page.rect.width, page.rect.height) == tuple(case["viewSize"])
    if "decimalDrafts" in case:
        move = case["decimalDrafts"]["move"]
        old = next(edit for edit in case["edits"] if edit["object"] == move["object"])
        accepted = [float(token) for token in move["coordinates"].split()]
        assert accepted == move["value"] and accepted[0] > old["value"][2]
        document.xref_set_key(page.xref, old["key"], pdf_rect(accepted))
        document = fitz.open(stream=document.tobytes(), filetype="pdf")
        page = document[0]
        kind, raw = document.xref_get_key(page.xref, old["key"])
        assert kind == "array" and [float(value) for value in raw[1:-1].split()] == accepted
        print("[DEBUG] MuPDF verified a precise translated TrimBox beyond its original right edge.")
    document.xref_set_key(page.xref, case["clear"]["key"], "null")
    document = fitz.open(stream=document.tobytes(), filetype="pdf")
    page = document[0]
    assert document.xref_get_key(page.xref, case["clear"]["key"])[0] == "null"
    assert (page.rect.width, page.rect.height) == tuple(case["clearedViewSize"])
    print("MuPDF verified five persisted boundaries, the scaled crop, and the media fallback after clearing CropBox.")


if __name__ == "__main__":
    main()
