#!/usr/bin/env python3
"""🗂️ Independent oracle of the design-case bridge: re-derives catalogue search and the first-order sizing remedy from the committed fixture, and checks an artifact input vocabulary (unique rows, bilingual labels, derivable unit symbols, valid choices and bounds).
Usage: python3 -I 🐍️.py [vocabulary.json ...]  (fixture: ../🧪️tests/🧫️fixtures/🔣️.json). Prints one JSON line. No third-party package covers catalogue search; the formulation is independent of the Rust code on purpose.
"""
import importlib.util
import json
import pathlib
import sys

HERE = pathlib.Path(__file__).resolve().parent
FIXTURE = HERE.parent / "🧪️tests" / "🧫️fixtures" / "🔣️.json"
UNITS = HERE.parent.parent / "📐️quantities-clauses-and-text" / "🔢️units" / "🔮️oracles" / "🐍️.py"


def fold(text):
    return "".join(ch for ch in text.lower() if ch not in " -_.§")


def search(entries, query):
    tokens = [t for t in (fold(w) for w in query.split()) if t]
    hits = []
    for index, entry in enumerate(entries):
        fields = [fold(entry[k]) for k in ("id", "part", "clause", "en", "de")]
        if all(any(t in f for f in fields) for t in tokens):
            ident, clause = fold(entry["id"]), fold(entry["clause"])
            rank = 0 if any(t == ident for t in tokens) else 1 if any(clause.startswith(t) for t in tokens) else 2
            hits.append((rank, index, entry["id"]))
    return [h[2] for h in sorted(hits)]


def check_fixture():
    fixture = json.loads(FIXTURE.read_text(encoding="utf-8"))
    for case in fixture["searches"]:
        got = search(fixture["entries"], case["query"])
        assert got == case["ids"], (case["query"], got, case["ids"])
    for lever in fixture["levers"]:
        assert abs(lever["current"] * lever["utilization"] - lever["required"]) < 1e-9, lever
    return len(fixture["searches"]), len(fixture["levers"])


def check_vocabulary(path):
    spec = importlib.util.spec_from_file_location("unit_oracle", UNITS)
    units = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(units)
    doc = json.loads(pathlib.Path(path).read_text(encoding="utf-8"))
    seen = set()
    for row in doc["inputs"]:
        ident = (row["key"], row["scope"])
        assert ident not in seen, ("duplicate row", ident)
        seen.add(ident)
        assert row["en"].strip() and row["de"].strip(), ("unlabelled", ident)
        if row["kind"] in ("number", "vector", "series"):
            units.unit(row["unit"])
            assert "choices" not in row
            if "min" in row or "max" in row:
                assert row["min"] <= row["max"], ident
        elif row["kind"] == "choice":
            values = [c["value"] for c in row["choices"]]
            assert values and len(values) == len(set(values)), ident
            assert row.get("example", values[0]) in values, ident
        else:
            assert "choices" not in row and "unit" not in row, ident
    return len(doc["inputs"])


if __name__ == "__main__":
    searches, levers = check_fixture()
    print(json.dumps({"searches": searches, "levers": levers, "vocabularies": [check_vocabulary(p) for p in sys.argv[1:]]}))
