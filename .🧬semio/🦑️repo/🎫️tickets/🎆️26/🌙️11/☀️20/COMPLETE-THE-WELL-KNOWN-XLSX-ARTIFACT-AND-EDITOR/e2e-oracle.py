"""📐 Third-party oracle for the XLSX end-to-end journey: openpyxl reads structure, typed values, formulas and styles; `formulas` recalculates the workbook independently.

Usage: uv run --no-project --with openpyxl --with formulas python -I e2e-oracle.py <workbook.xlsx>
https://openpyxl.readthedocs.io/ https://formulas.readthedocs.io/
"""
import json
import re
import sys
import zipfile

import openpyxl


def describe(path):
    book = openpyxl.load_workbook(path)
    cached = openpyxl.load_workbook(path, data_only=True)
    sheets = {}
    for sheet in book.worksheets:
        cells = {}
        for row in sheet.iter_rows():
            for cell in row:
                if cell.value is None and cell.style_id == 0:
                    continue
                cells[cell.coordinate] = {
                    "value": cell.value,
                    "type": cell.data_type,
                    "cached": cached[sheet.title][cell.coordinate].value,
                    "bold": bool(cell.font.b),
                    "numberFormat": cell.number_format,
                    "fill": cell.fill.fgColor.rgb if cell.fill.patternType == "solid" else None,
                }
        sheets[sheet.title] = {
            "cells": cells,
            "merged": [str(r) for r in sheet.merged_cells.ranges],
            "widths": {k: v.width for k, v in sheet.column_dimensions.items() if v.customWidth},
        }
    with zipfile.ZipFile(path) as archive:
        calc = re.search(r"<(?:\w+:)?calcPr\b[^>]*>", archive.read("xl/workbook.xml").decode())
    return {"sheetNames": book.sheetnames, "sheets": sheets, "calcPr": calc.group(0) if calc else None}


def recalculate(path):
    import formulas

    model = formulas.ExcelModel().loads(path).finish()
    solution = model.calculate()
    out = {}
    for key, value in solution.items():
        ref = key.split("]", 1)[-1].replace("'", "")
        if "!" not in ref or ":" in ref.split("!", 1)[1]:
            continue
        raw = value.value[0][0] if hasattr(value, "value") else value
        out[ref.upper()] = raw.item() if hasattr(raw, "item") else str(raw)
    return out


if __name__ == "__main__":
    path = sys.argv[1]
    result = describe(path)
    try:
        result["recalculated"] = recalculate(path)
    except Exception as error:
        result["recalculated"] = {"error": repr(error)[:300]}
    print(json.dumps(result, default=str))
