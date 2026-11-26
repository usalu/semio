#!/usr/bin/env python3
"""💰️ Third-party ORACLE for the `s.bim.model@1` inference `💰️costs`.

The subject (Rust, `semio-s-artifact-bim-model-costs`) prices every element of a linked type with `quantity * unit price * factor`, rounds the exact
product once, half up, to the precision of the item and sums exact decimals per currency. This file reproduces every case from the SAME committed
fixture with `sympy` rationals, a library that has never seen this repository: the product is a `Rational` of the shortest round-trip decimal strings
of the doubles, the half-up rounding is `floor(x * 10^precision + 1/2) / 10^precision`, every sum is an exact `Rational`, and only the published amounts
are converted to the nearest double (an amount that no double holds is an overflow diagnostic).

    python 🐍️.py check <path to 🧫️fixtures/💡️inferences/💰️costs/🔣️.json>

@see ../🧫️fixtures/💡️inferences/💰️costs/🔣️.json — the committed items, links, quantity rows and expected totals
@see https://docs.sympy.org/latest/modules/core.html#sympy.core.numbers.Rational
"""

import copy
import json
import sys
from pathlib import Path

import sympy
from sympy import Rational

BASES = {"Count": "count", "Length": "length", "GrossArea": "gross_area", "NetArea": "net_area", "SurfaceArea": "surface_area", "GrossSideArea": "gross_side_area", "NetSideArea": "net_side_area", "GrossVolume": "gross_volume", "NetVolume": "net_volume", "Mass": "mass"}
SCOPES = ("elements", "types", "storeys", "items", "project")


def exact(value):
    """🔟️ The exact rational of the shortest decimal string of a double, as the subject reads it."""
    return Rational(repr(float(value)))


def nonnegative(value):
    return isinstance(value, (int, float)) and not isinstance(value, bool) and value == value and abs(value) != float("inf") and value >= 0


def double(value):
    """🎯️ The nearest double of an exact rational, or None when no finite double holds it."""
    try:
        return int(value.p) / int(value.q)
    except OverflowError:
        return None


def rounded(value, precision):
    return sympy.floor(value * 10**precision + Rational(1, 2)) / Rational(10**precision)


def valid_item(item):
    return bool(str(item["name"]).strip()) and bool(str(item["currency"]).strip()) and item["basis"] in BASES and nonnegative(item["unit_cost"]) and isinstance(item["precision"], int) and 0 <= item["precision"] <= 9


def infer(items, links, elements):
    """💡️ The costs of the committed rows, with exact arithmetic."""
    lines, diagnostics, sums = [], [], {scope: {} for scope in SCOPES}
    by_type = {}
    for link_id in sorted(links):
        link = links[link_id]
        item = items.get(link["item"])
        code = "cost.link-invalid" if not str(link["type_id"]).strip() or not nonnegative(link["factor"]) else "cost.item-missing" if item is None else "cost.item-invalid" if not valid_item(item) else None
        if code:
            diagnostics.append({"code": code, "link": link_id, "item": link["item"], "element": ""})
        else:
            by_type.setdefault(link["type_id"], []).append((link_id, link, item))
    for element in sorted(elements):
        row = elements[element]
        for link_id, link, item in by_type.get(row["type_id"], []):
            quantity = row.get(BASES[item["basis"]], 0)
            if not nonnegative(quantity):
                diagnostics.append({"code": "cost.quantity-invalid", "link": link_id, "item": link["item"], "element": element})
                continue
            amount = rounded(exact(quantity) * exact(item["unit_cost"]) * exact(link["factor"]), item["precision"])
            keys = (element, row["type_id"], row["storey"], link["item"], "")
            updates = [sums[scope].get((key, item["currency"]), Rational(0)) + amount for scope, key in zip(SCOPES, keys)]
            if double(amount) is None or any(double(update) is None for update in updates):
                diagnostics.append({"code": "cost.overflow", "link": link_id, "item": link["item"], "element": element})
                continue
            for scope, key, update in zip(SCOPES, keys, updates):
                sums[scope][(key, item["currency"])] = update
            lines.append({"element": element, "link": link_id, "item": link["item"], "type_id": row["type_id"], "storey": row["storey"], "currency": item["currency"], "quantity": quantity, "unit_cost": item["unit_cost"], "factor": link["factor"], "precision": item["precision"], "total": double(amount)})
    costs = {"lines": lines, "diagnostics": diagnostics}
    for scope in SCOPES[:4]:
        costs[scope] = {}
        for (key, currency), value in sorted(sums[scope].items()):
            costs[scope].setdefault(key, {})[currency] = double(value)
    costs["project"] = {currency: double(value) for (key, currency), value in sorted(sums["project"].items())}
    return costs


def fixture(ctx):
    uri = next(uri for uri in ctx.step_input_uris() if uri.endswith("/💰️costs/🔣️.json"))
    return json.loads(ctx.input_bytes(uri).decode("utf-8"))


def outcome(projection):
    from semio_repo_test import Outcome

    return Outcome(projection, raw=json.dumps(projection, separators=(",", ":"), ensure_ascii=False).encode("utf-8"))


def items_handler(ctx):
    """💰️ The committed fixture priced from scratch; it must reproduce the totals the fixture states."""
    case = fixture(ctx)
    costs = infer(case["items"], case["links"], case["elements"])
    wrong = [scope for scope in ("project", "storeys", "types", "items") if costs[scope] != case["expected"][scope]]
    if wrong:
        raise AssertionError("the committed expectations disagree with the oracle: " + ", ".join(wrong))
    return outcome(costs)


def currencies_handler(ctx):
    """💱️ The project retains one total per currency."""
    case = fixture(ctx)
    costs = infer(case["items"], case["links"], case["elements"])
    return outcome({"project": costs["project"], "storeys": costs["storeys"]})


def replay_handler(ctx):
    """↩️ Doubling one wall and restoring it prices the doubled and the original rows; the authored records are never touched."""
    case = fixture(ctx)
    authored = copy.deepcopy((case["items"], case["links"]))
    original = infer(case["items"], case["links"], case["elements"])
    doubled_rows = copy.deepcopy(case["elements"])
    for key in ("net_side_area", "net_volume"):
        doubled_rows["wall-a"][key] *= 2
    doubled = infer(case["items"], case["links"], doubled_rows)
    restored = infer(case["items"], case["links"], case["elements"])
    if authored != (case["items"], case["links"]) or restored != original:
        raise AssertionError("replay changed the authored records or did not restore the costs")
    return outcome({"original": original, "doubled": doubled, "restored": restored})


def diagnostics_handler(ctx):
    """🚫️ The fixture plus the further authored records of the scenario; invalid ones are reported and priced as nothing."""
    case, extra = fixture(ctx), ctx.doc_json()
    items, links = {**case["items"], **extra["items"]}, {**case["links"], **extra["links"]}
    return outcome(infer(items, links, case["elements"]))


def adapter():
    """🧭️ Registration in the ORACLE role only, by the feature's scenario id."""
    from semio_repo_test import Adapter

    return Adapter("python").oracle("costs-items", items_handler).oracle("costs-currencies", currencies_handler).oracle("costs-replay", replay_handler).oracle("costs-diagnostics", diagnostics_handler)


def main(arguments):
    """🏃️ `check` prices the committed fixture and compares it with its stated totals."""
    case = json.loads(Path(arguments[1]).read_text(encoding="utf-8"))
    costs = infer(case["items"], case["links"], case["elements"])
    wrong = [scope for scope in ("project", "storeys", "types", "items") if costs[scope] != case["expected"][scope]]
    print("sympy %s: %s" % (sympy.__version__, "disagrees on " + ", ".join(wrong) if wrong else "oracle agrees"))
    return 1 if wrong or costs["diagnostics"] else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
