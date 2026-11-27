"""Generates the formula-recalculation fixture; expected values come only from the third-party `formulas` library.

Usage: python -I formula-recalculation-oracle.py <output-fixture-path>
"""
import json, os, sys, tempfile, shutil
import openpyxl, formulas
from formulas.functions import XlError

S = lambda cells, name="Sheet1": {"name": name, "cells": cells}


def case(cid, cells, *more):
    return {"id": cid, "sheets": [S(cells)] + list(more)}


def single(cid, expr, cells=None):
    return case(cid, {**(cells or {}), "Z1": "=" + expr})


CASES = [
    single("literal-number", "42"),
    single("literal-decimal", "1.5+0.25"),
    single("literal-string", '"hello"'),
    single("literal-bool-true", "TRUE"),
    single("literal-bool-false", "FALSE"),
    single("unary-minus", "-5+2"),
    single("unary-minus-precedence-power", "-2^2"),
    single("power-right-assoc", "2^3^2"),
    single("percent-postfix", "50%"),
    single("percent-in-arithmetic", "200*10%"),
    single("precedence-mul-add", "1+2*3"),
    single("precedence-parentheses", "(1+2)*3"),
    single("nested-parentheses", "((2+3)*(4-1))/5"),
    single("division", "7/2"),
    single("concat-mixed", '"a"&1&TRUE'),
    single("cmp-equal", "1=1"),
    single("cmp-not-equal", "1<>2"),
    single("cmp-less", "1<2"),
    single("cmp-greater", "1>2"),
    single("cmp-less-equal", "2<=2"),
    single("cmp-greater-equal", "1>=2"),
    single("cmp-text-equal-case-insensitive", '"abc"="ABC"'),
    single("precedence-concat-vs-add", '1+2&3+4'),
    single("precedence-compare-vs-concat", '"a"&"b"="ab"'),
    single("ref-relative", "A1+A2", {"A1": 3, "A2": 4}),
    single("ref-absolute", "$A$1+A$2+$A3", {"A1": 1, "A2": 2, "A3": 4}),
    single("range-sum", "SUM(A1:A3)", {"A1": 1, "A2": 2, "A3": 3}),
    single("range-2d-sum", "SUM(A1:B2)", {"A1": 1, "B1": 2, "A2": 3, "B2": 4}),
    case("sheet-ref", {"A1": 1, "B1": "=Sheet2!A1*2"}, S({"A1": 21}, "Sheet2")),
    case("sheet-ref-quoted-range", {"B1": "=SUM('My Sheet'!A1:B2)"},
         S({"A1": 1, "B1": 2, "A2": 3, "B2": 4}, "My Sheet")),
    single("fn-sum-mixed-args", "SUM(A1:A2,10,5)", {"A1": 1, "A2": 2}),
    single("fn-average", "AVERAGE(A1:A4)", {"A1": 1, "A2": 2, "A3": 3, "A4": 6}),
    single("fn-min", "MIN(A1:A3)", {"A1": 5, "A2": -2, "A3": 9}),
    single("fn-max", "MAX(A1:A3)", {"A1": 5, "A2": -2, "A3": 9}),
    single("fn-count", "COUNT(A1:A4)", {"A1": 1, "A2": "x", "A3": True, "A4": 2}),
    single("fn-counta", "COUNTA(A1:A4)", {"A1": 1, "A2": "x", "A4": True}),
    single("fn-if-three-args-true", 'IF(A1>0,"pos","neg")', {"A1": 1}),
    single("fn-if-three-args-false", 'IF(A1>0,"pos","neg")', {"A1": -1}),
    single("fn-if-two-args-true", 'IF(A1>0,"pos")', {"A1": 1}),
    single("fn-if-two-args-false", 'IF(A1>0,"pos")', {"A1": -1}),
    single("fn-and", "AND(1<2,2<3,TRUE)"),
    single("fn-or", "OR(1>2,FALSE,3>2)"),
    single("fn-not", "NOT(FALSE)"),
    single("fn-round-positive-digits", "ROUND(3.14159,2)"),
    single("fn-round-negative-digits", "ROUND(1234.5,-2)"),
    single("fn-round-zero-digits", "ROUND(2.4,0)"),
    single("fn-round-half-away-positive", "ROUND(2.5,0)"),
    single("fn-round-half-away-negative", "ROUND(-2.5,0)"),
    single("fn-abs", "ABS(-7.5)"),
    single("fn-concatenate", 'CONCATENATE("a",1,TRUE,"c")'),
    single("fn-concat-range", 'CONCAT(A1:A2,"x")', {"A1": "p", "A2": "q"}),
    single("fn-len", 'LEN("héllo")'),
    single("fn-upper", 'UPPER("MiXed")'),
    single("fn-lower", 'LOWER("MiXed")'),
    single("fn-product", "PRODUCT(A1:A3)", {"A1": 2, "A2": 3, "A3": 4}),
    single("fn-iferror-caught", "IFERROR(1/0,-1)"),
    single("fn-iferror-passthrough", "IFERROR(5,-1)"),
    single("empty-cell-arithmetic", "A1+5"),
    single("empty-cell-concat", 'A1&"x"'),
    single("sum-ignores-text-in-range", "SUM(A1:A3)", {"A1": 1, "A2": "text", "A3": 2}),
    case("formula-chain", {"B1": 2, "B2": "=B1*2", "B3": "=B2+1", "B4": "=B3*B2"}),
    single("error-div-zero", "1/0"),
    single("error-div-zero-ref", "A1/A2", {"A1": 4}),
    single("bool-arithmetic", "TRUE+1"),
    single("text-coercion-arithmetic", '"3"+1'),
    single("text-not-coercible", '"abc"+1'),
    single("average-empty-range", "AVERAGE(A1:A3)"),
    case("scenario-sum-and-double", {"B2": 1200.5, "B3": 350.25, "B4": "=SUM(B2:B3)", "B5": "=B4*2"}),
    case("scenario-sum-and-double-400", {"B2": 1200.5, "B3": 400, "B4": "=SUM(B2:B3)", "B5": "=B4*2"}),
]


def normalize(v):
    if isinstance(v, XlError):
        return {"error": str(v)}
    if hasattr(v, "tolist"):
        v = v.tolist()
        while isinstance(v, list):
            v = v[0]
    if isinstance(v, XlError):
        return {"error": str(v)}
    if isinstance(v, bool) or isinstance(v, str):
        return v
    if hasattr(v, "item"):
        v = v.item()
    if isinstance(v, bool):
        return v
    if isinstance(v, (int, float)):
        return float(v) if isinstance(v, float) and not v.is_integer() else (int(v) if float(v).is_integer() else v)
    raise TypeError(repr(v))


def compute(c, workdir):
    wb = openpyxl.Workbook()
    wb.remove(wb.active)
    formula_cells = []
    for sh in c["sheets"]:
        ws = wb.create_sheet(sh["name"])
        for ref, v in sh["cells"].items():
            ws[ref] = v
            if isinstance(v, str) and v.startswith("="):
                formula_cells.append((sh["name"], ref))
    path = os.path.join(workdir, c["id"] + ".xlsx")
    wb.save(path)
    model = formulas.ExcelModel().loads(path).finish()
    sol = model.calculate()
    by_key = {}
    for k, cell in sol.items():
        by_key[k.upper().replace("'", "")] = cell
    expected = {}
    for sheet, ref in formula_cells:
        key = ("[" + c["id"] + ".xlsx]" + sheet + "!" + ref).upper()
        cell = by_key[key]
        expected[sheet + "!" + ref] = normalize(cell.value)
    return expected


def main(out):
    gen = os.path.join(os.path.dirname(os.path.abspath(__file__)), "🗑️generated")
    os.makedirs(gen, exist_ok=True)
    workdir = tempfile.mkdtemp(dir=gen)
    cases, dropped = [], []
    try:
        for c in CASES:
            try:
                c = dict(c, expected=compute(c, workdir))
                cases.append(c)
            except Exception as e:
                dropped.append((c["id"], repr(e)))
    finally:
        shutil.rmtree(workdir, ignore_errors=True)
    doc = {"generator": "formulas " + formulas.__version__ + " via openpyxl " + openpyxl.__version__, "cases": cases}
    os.makedirs(os.path.dirname(out), exist_ok=True)
    with open(out, "w", encoding="utf-8") as f:
        json.dump(doc, f, ensure_ascii=False, indent=1)
        f.write("\n")
    print(len(cases), "cases written;", len(dropped), "dropped", dropped)


main(sys.argv[1])
