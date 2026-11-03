"""Wave 2 features agent: one-shot hand-normalisation of the norm catalogue `🥒️.feature` files to the shared step grammar
(`✏️s/🔌️plugins/📕️norm/🔮️oracles/🥒️steps/🧬️schema/🔣️.json`). Run once with `.venv/bin/python -I`; the features are the result.
Prose scenarios without a family move into the feature description (they stay asserted by the Rust unit tests)."""

import re
import sys
from pathlib import Path

ROOT = Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts")
SUB = "🏅️standards/🔖️1/🪆️subsets/✳️any"
FIXTURE = "shared://🧫️fixtures/📇️catalogue/🔣️.json"


def kebab(*parts):
    return re.sub(r"[^a-z0-9]+", "-", "-".join(str(p) for p in parts).lower()).strip("-")


def split(text):
    lines = text.splitlines()
    feature = next(i for i, l in enumerate(lines) if l.startswith("Feature:"))
    name = lines[feature][len("Feature:"):].strip()
    first = next(i for i, l in enumerate(lines) if i > feature and (l.strip().startswith("Scenario") or l.strip().startswith("@")))
    description = lines[feature + 1:first]
    while description and not description[-1].strip():
        description.pop()
    blocks, current = [], None
    for l in lines[first:]:
        if re.match(r"\s*Scenario", l):
            current = {"head": l.strip(), "body": []}
            blocks.append(current)
        elif current is not None:
            current["body"].append(l)
    return name, description, blocks


def rows_of(block):
    table = [[c.strip() for c in l.strip().strip("|").split("|")] for l in block["body"] if l.strip().startswith("|")]
    return table[0], table[1:]


def render_table(header, rows):
    cells = [header] + rows
    widths = [max(len(r[i]) for r in cells) for i in range(len(header))]
    return ["      | " + " | ".join(c.ljust(w) for c, w in zip(r, widths)) + " |" for r in cells]


def unique(ids):
    seen, out = {}, []
    for i in ids:
        seen[i] = seen.get(i, 0) + 1
        out.append(i if seen[i] == 1 else "%s-%d" % (i, seen[i]))
    return out


def prose_of(block):
    steps = [l.strip() for l in block["body"] if re.match(r"\s*(Given|When|Then|And|But)\b", l)]
    return "  - %s: %s" % (block["head"].split(":", 1)[1].strip(), " ".join(steps))


def feature_text(tags, name, description, outlines, dropped, note):
    out = list(tags) + ["Feature: " + name] + description
    if dropped:
        out += ["", "  " + note] + [prose_of(b) for b in dropped]
    for outline in outlines:
        out += [""] + outline
    return "\n".join(out) + "\n"


def outline(family, title, steps, header, rows, mode="differential"):
    return ["  @id-" + family, "  @level-quick", "  @mode-" + mode, "  Scenario Outline: " + title] + ["    " + s for s in steps] + ["    Examples:"] + render_table(header, rows)


def fixture_case(path, tags, entry_col, extra_cols):
    name, description, blocks = split(path.read_text(encoding="utf8"))
    main = blocks[0]
    header, rows = rows_of(main)
    pos = {h: i for i, h in enumerate(header)}
    out_rows = [[kebab(r[pos[entry_col]], r[pos["index"]]), r[pos["index"]], r[pos[entry_col]]] + [r[pos[c]] for c in extra_cols] for r in rows]
    steps = ["Given the committed catalogue case %s at index <index>" % FIXTURE, "When both implementations evaluate catalogue entry <entry> on that case", "Then every result named by the oracle agrees within the comparison profile"]
    block = outline("catalogue-fixture-case", "Evaluate <entry> case <index> and compare every named result", steps, ["id", "index", "entry"] + extra_cols, out_rows)
    path.write_text(feature_text(tags, name, description, [block], blocks[1:], "Further specification assertions, asserted by the Rust unit tests of the artifact and not by the platform adapters:"), encoding="utf8")


def examples(path, tags, columns):
    name, description, blocks = split(path.read_text(encoding="utf8"))
    header, rows = rows_of(blocks[0])
    pos = {h: i for i, h in enumerate(header)}
    get = lambda r, c: r[pos[columns[c]]]
    ids = unique([kebab(get(r, "entry"), get(r, "key"), get(r, "annex")) for r in rows])
    out_rows = [[i, get(r, "entry"), get(r, "annex"), get(r, "inputs"), get(r, "key"), get(r, "expected"), get(r, "tolerance")] for i, r in zip(ids, rows)]
    steps = ['Given the catalogue entry "<entry>" for annex "<annex>" with inputs "<inputs>"', "When the entry is evaluated", 'Then the output "<key>" equals <expected> within relative tolerance <tolerance>']
    block = outline("catalogue-example", "Catalogue entry <entry> output <key>", steps, ["id", "entry", "annex", "inputs", "key", "expected", "tolerance"], out_rows)
    path.write_text(feature_text(tags, name, description, [block], blocks[1:], "Further specification assertions, asserted by the Rust unit tests of the artifact and not by the platform adapters:"), encoding="utf8")


def vectors(path, tags, part_of, vector_of, quantity_col="quantity"):
    name, description, blocks = split(path.read_text(encoding="utf8"))
    header, rows = rows_of(blocks[0])
    pos = {h: i for i, h in enumerate(header)}
    out_rows = []
    for r in rows:
        part, vector, quantity = part_of(r, pos), vector_of(r, pos), r[pos[quantity_col]]
        out_rows.append([kebab(vector, quantity, part if part_of is not None else ""), vector, part, quantity, r[pos["tolerance"]]])
    ids = unique([r[0] for r in out_rows])
    for r, i in zip(out_rows, ids):
        r[0] = i
    steps = ['Given the clause vector "<vector>" of "<part>"', 'When the quantity "<quantity>" is evaluated', "Then it equals the independent oracle value within relative tolerance <tolerance>"]
    outlines = [outline("clause-vector", "<vector> <quantity> equals the oracle value", steps, ["id", "vector", "part", "quantity", "tolerance"], out_rows)]
    rest = blocks[1:]
    for b in list(rest):
        if "reports the oracle utilisation" in b["head"]:
            h, rs = rows_of(b)
            p = {x: i for i, x in enumerate(h)}
            rows2 = [[kebab(r[p["check"]]), r[p["check"]], r[p["quantity"]], r[p["tolerance"]]] for r in rs]
            s2 = ['Given the showcase subject "special-parts" with the DIN annex', "When the compliance report is evaluated", 'Then the utilisation of the check "<check>" equals the oracle quantity "<quantity>" within relative tolerance <tolerance>']
            outlines.append(outline("oracle-check", "the check <check> reports the oracle utilisation", s2, ["id", "check", "quantity", "tolerance"], rows2))
            rest.remove(b)
    path.write_text(feature_text(tags, name, description, outlines, rest, "Further specification assertions, asserted by the Rust unit tests of the artifact and not by the platform adapters:"), encoding="utf8")


def published(path, tags):
    name, description, blocks = split(path.read_text(encoding="utf8"))
    header, rows = rows_of(blocks[0])
    pos = {h: i for i, h in enumerate(header)}
    ids = unique([kebab(r[pos["example"]], r[pos["quantity"]]) for r in rows])
    out_rows = [[i, r[pos["example"]], r[pos["quantity"]], r[pos["expected"]], r[pos["tolerance"]]] for i, r in zip(ids, rows)]
    steps = ['Given the worked example "<example>" with its published inputs', 'When the quantity "<quantity>" is evaluated', "Then it equals <expected> within relative tolerance <tolerance>"]
    block = outline("published-example", "<example> <quantity> is reproduced", steps, ["id", "example", "quantity", "expected", "tolerance"], out_rows)
    path.write_text(feature_text(tags, name, description, [block], blocks[1:], "Further specification assertions, asserted by the Rust unit tests of the artifact and not by the platform adapters:"), encoding="utf8")


def tag(capability, oracle, comparison):
    return ["@capability-" + capability, "@oracle-" + oracle, "@comparison-" + comparison]


def main():
    fixture = lambda artifact, case: ROOT / artifact / SUB / "🧪️tests" / case / "🥒️.feature"
    fixture_case(fixture("⚖️en1990", "📇️catalogue-en1990-1"), tag("en1990-1-evaluate", "en1990-1-compliance-independent", "numerical-v1"), "entry", ["case"])
    fixture_case(fixture("🪵️en1995", "📇️catalogue-en1995-1"), tag("en1995-1-evaluate", "en1995-1-compliance-independent", "method-spread-v1"), "entry", [])
    fixture_case(fixture("🌍️en1997", "📇️catalogue-en1997-1"), tag("en1997-1-evaluate", "en1997-1-compliance-independent", "method-spread-v1"), "entry", ["label"])
    cols = {"entry": "entry", "annex": "annex", "inputs": "inputs", "key": "key", "expected": "expected", "tolerance": "tolerance"}
    examples(fixture("🏋️en1991", "🏋️en1991-catalogue"), tag("en1991-1-evaluate", "en1991-1-compliance-independent", "numerical-v1"), {**cols, "expected": "value"})
    examples(ROOT / "🏛️en1992" / SUB / "🧬️schema" / "🧪️tests" / "📇️catalogue" / "🥒️.feature", tag("en1992-1-evaluate", "en1992-1-compliance-independent", "numerical-v1"), {**cols, "entry": "id"})
    examples(fixture("🪨️en1996", "📇️en1996-catalogue"), tag("en1996-1-evaluate", "en1996-1-compliance-independent", "numerical-v1"), cols)
    vectors(fixture("🔩️en1993", "🧮️clauses"), tag("en1993-1-evaluate", "en1993-1-compliance-independent", "numerical-v1"), lambda r, p: "EN 1993-1-1", lambda r, p: r[p["id"]])
    vectors(fixture("🔩️en1993", "🔭️special-parts"), tag("en1993-1-evaluate", "en1993-1-compliance-independent", "numerical-v1"), lambda r, p: r[p["part"]], lambda r, p: "special-parts")
    published(fixture("🧩️en1994", "🧮️catalogue-en1994"), tag("en1994-1-evaluate", "en1994-1-compliance-independent", "numerical-v1"))


main()
