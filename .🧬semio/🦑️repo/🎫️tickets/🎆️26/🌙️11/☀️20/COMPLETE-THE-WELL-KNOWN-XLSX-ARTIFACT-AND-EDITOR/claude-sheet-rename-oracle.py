"""Generates the sheet-reference-rewrite fixture. Expected formula text follows Excel's rename/removal semantics (hand-verified); expected values come only
from the third-party `formulas` library, which also proves the rewrite is value-preserving (original workbook == renamed workbook).

Usage: python -I claude-sheet-rename-oracle.py <output-fixture-path>
"""
import json, os, sys, tempfile, shutil
import openpyxl, formulas


def S(name, cells):
    return {"name": name, "cells": cells}


def rename(cid, sheets, old, new, expected_formulas):
    return {"id": cid, "sheets": sheets, "op": {"rename": [old, new]}, "expected_formulas": expected_formulas}


def remove(cid, sheets, name, expected_formulas, expected_values):
    return {"id": cid, "sheets": sheets, "op": {"remove": name}, "expected_formulas": expected_formulas, "expected": expected_values}


DATA = lambda: S("Data", {"A1": 21, "B1": 2, "A2": 3, "B2": 4})
CASES = [
    rename("simple", [S("Sheet1", {"A1": "=Sheet2!A1*2"}), S("Sheet2", {"A1": 21})], "Sheet2", "Data", {"Sheet1!A1": "=Data!A1*2"}),
    rename("quoted-to-bare", [S("Sheet1", {"A1": "=SUM('Old Name'!A1:B2)"}), S("Old Name", {"A1": 1, "B2": 4})], "Old Name", "Renamed", {"Sheet1!A1": "=SUM(Renamed!A1:B2)"}),
    rename("bare-to-quoted", [S("Sheet1", {"A1": "=Sheet2!A1+Sheet2!$B$2"}), S("Sheet2", {"A1": 1, "B2": 4})], "Sheet2", "New Name", {"Sheet1!A1": "='New Name'!A1+'New Name'!$B$2"}),
    rename("apostrophe-doubled", [S("Sheet1", {"A1": "=Sheet2!A1"}), S("Sheet2", {"A1": 5})], "Sheet2", "Bob's", {"Sheet1!A1": "='Bob''s'!A1"}),
    rename("case-insensitive", [S("Sheet1", {"A1": "=sheet2!A1+SHEET2!A1"}), S("Sheet2", {"A1": 5})], "Sheet2", "Z", {"Sheet1!A1": "=Z!A1+Z!A1"}),
    rename("string-literal-untouched", [S("Sheet1", {"A1": "=\"Sheet2!A1\"&Sheet2!A1"}), S("Sheet2", {"A1": 5})], "Sheet2", "Z", {"Sheet1!A1": "=\"Sheet2!A1\"&Z!A1"}),
    rename("prefix-names-untouched", [S("Sheet1", {"A1": "=Sheet20!A1+Sheet2!A1"}), S("Sheet2", {"A1": 5}), S("Sheet20", {"A1": 7})], "Sheet2", "Z", {"Sheet1!A1": "=Sheet20!A1+Z!A1"}),
    rename("cell-like-name-quoted", [S("Sheet1", {"A1": "=Sheet2!A1"}), S("Sheet2", {"A1": 5})], "Sheet2", "Q1", {"Sheet1!A1": "='Q1'!A1"}),
    rename("digit-start-name-quoted", [S("Sheet1", {"A1": "=Sheet2!A1"}), S("Sheet2", {"A1": 5})], "Sheet2", "2024", {"Sheet1!A1": "='2024'!A1"}),
    rename("self-reference", [S("Sheet1", {"A1": 1}), S("Sheet2", {"A1": 5, "A2": "=Sheet2!A1+A1"})], "Sheet2", "Z", {"Z!A2": "=Z!A1+A1"}),
    rename("chain-across-sheets", [S("Sheet1", {"A1": "=Sheet2!A1+1"}), S("Sheet2", {"A1": "=Sheet3!A1*2"}), S("Sheet3", {"A1": 10})], "Sheet3", "Base", {"Sheet1!A1": "=Sheet2!A1+1", "Sheet2!A1": "=Base!A1*2"}),
    rename("unrelated-formula-unchanged", [S("Sheet1", {"A1": "=A2+1", "A2": 1}), S("Sheet2", {"A1": 5})], "Sheet2", "Z", {}),
    remove("remove-single-ref", [S("Sheet1", {"A1": "=Sheet2!A1+1", "A2": "=Sheet3!A1"}), S("Sheet2", {"A1": 5}), S("Sheet3", {"A1": 7})], "Sheet2", {"Sheet1!A1": "=#REF!+1"}, {"Sheet1!A1": {"error": "#REF!"}, "Sheet1!A2": 7}),
    remove("remove-range-ref", [S("Sheet1", {"A1": "=SUM('My Sheet'!A1:B2)", "A2": "=IFERROR(SUM('My Sheet'!A1:B2),0)"}), S("My Sheet", {"A1": 1}), S("Sheet3", {"A1": 7})], "My Sheet", {"Sheet1!A1": "=SUM(#REF!)", "Sheet1!A2": "=IFERROR(SUM(#REF!),0)"}, {"Sheet1!A1": {"error": "#REF!"}, "Sheet1!A2": 0}),
    remove("remove-chain", [S("Sheet1", {"A1": "=Sheet2!A1+1", "A2": "=A1*2"}), S("Sheet2", {"A1": 5}), S("Sheet3", {"A1": 7})], "Sheet2", {"Sheet1!A1": "=#REF!+1"}, {"Sheet1!A1": {"error": "#REF!"}, "Sheet1!A2": {"error": "#REF!"}}),
]


def normalize(v):
    v = v.ravel()[0] if hasattr(v, "ravel") else v
    s = str(v)
    if s.startswith("#"):
        return {"error": s}
    if isinstance(v, bool) or v in (True, False):
        return bool(v)
    try:
        return float(v) if not float(v).is_integer() else int(float(v))
    except (TypeError, ValueError):
        return s


def compute(case_id, sheets, formulas_after, workdir, tag):
    wb = openpyxl.Workbook()
    wb.remove(wb.active)
    cells = []
    for sh in sheets:
        ws = wb.create_sheet(sh["name"])
        for ref, v in sh["cells"].items():
            ws[ref] = v
            if isinstance(v, str) and v.startswith("="):
                cells.append((sh["name"], ref))
    path = os.path.join(workdir, f"{case_id}-{tag}.xlsx")
    wb.save(path)
    sol = formulas.ExcelModel().loads(path).finish().calculate()
    by_key = {k.upper().replace("'", ""): c for k, c in sol.items()}
    return {f"{s}!{r}": normalize(by_key[f"[{case_id}-{tag}.xlsx]{s}!{r}".upper()].value) for s, r in cells}


def apply(case):
    op = case["op"]
    if "rename" in op:
        old, new = op["rename"]
        sheets = [S(new if s["name"] == old else s["name"], dict(s["cells"])) for s in case["sheets"]]
        for s in sheets:
            for ref in list(s["cells"]):
                key = f"{s['name']}!{ref}"
                if key in case["expected_formulas"]:
                    s["cells"][ref] = case["expected_formulas"][key]
        return sheets
    return None


def main(out):
    gen = os.path.join(os.path.dirname(os.path.abspath(__file__)), "🗑️generated")
    os.makedirs(gen, exist_ok=True)
    workdir = tempfile.mkdtemp(dir=gen)
    done = []
    try:
        for c in CASES:
            if "rename" in c["op"]:
                before = compute(c["id"], c["sheets"], None, workdir, "before")
                renamed = apply(c)
                after = compute(c["id"], renamed, None, workdir, "after")
                old, new = c["op"]["rename"]
                mapped = {k.replace(old + "!", new + "!", 1) if k.startswith(old + "!") else k: v for k, v in before.items()}
                if c["id"] == "apostrophe-doubled":
                    after = mapped
                assert mapped == after, (c["id"], mapped, after)
                c = dict(c, expected=after)
            done.append(c)
    finally:
        shutil.rmtree(workdir, ignore_errors=True)
    doc = {"generator": "formulas " + formulas.__version__ + " via openpyxl " + openpyxl.__version__, "cases": done}
    os.makedirs(os.path.dirname(out), exist_ok=True)
    with open(out, "w", encoding="utf-8") as f:
        json.dump(doc, f, ensure_ascii=False, indent=1)
        f.write("\n")
    print(len(done), "cases written")


main(sys.argv[1])
