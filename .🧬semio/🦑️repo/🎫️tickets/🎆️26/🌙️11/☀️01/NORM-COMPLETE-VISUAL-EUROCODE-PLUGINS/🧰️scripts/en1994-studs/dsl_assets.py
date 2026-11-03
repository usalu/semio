"""🔩 One-shot rewrite of the three en1994 example DSL assets for the persisted stud collection: the inline `studs:REC` column of `beams` leaves the table and the stud objects become rows of the top-level `studs` table by the row rules of `schema-evolve/specs/en1994-studs.json` (the same rules that rewrote every JSON fixture). Run: python3 -I dsl_assets.py [--apply]"""
import json, pathlib, re, sys

HERE = pathlib.Path(__file__).resolve().parent
sys.path.insert(0, str(HERE.parent / "schema-evolve"))
sys.path.insert(0, str(HERE.parent / "field-adder"))
import l_convert

SPEC = json.loads((HERE.parent / "schema-evolve" / "specs" / "en1994-studs.json").read_text(encoding="utf8"))
RULE = l_convert.Rule(SPEC["convert"], "studs")
ASSETS = pathlib.Path(__file__).resolve().parents[8] / "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets"
SUPPORT = {"simply_supported": "simplySupported", "continuous_2_span": "continuous2Span", "end_span": "endSpan", "interior_span": "interiorSpan", "cantilever": "cantilever"}
OBJECT = re.compile(r"\{diameter-m=([0-9.eE+-]+)m height-m=([0-9.eE+-]+)m f-u-pa=([0-9.eE+-]+)Pa count-per-rib=(\d+) spacing-m=([0-9.eE+-]+)m total-count=(\d+)\}\s?")
ROW = re.compile(r"^  ([A-Za-z0-9-]+) ([0-9.eE+-]+)m ([0-9.eE+-]+)m ([a-z_0-9]+) ")


def number(value):
    return f"{value:.9g}"


def rewrite(text):
    out, rows = [], []
    for line in text.split("\n"):
        if line.startswith("beams [") and " studs:REC" in line:
            line = line.replace(" studs:REC", "")
        head = ROW.match(line)
        if head and OBJECT.search(line):
            bid, span, _, support = head.groups()
            d, h, fu, per, spacing, total = OBJECT.search(line).groups()
            obj = {"diameterM": float(d), "heightM": float(h), "fUPa": float(fu), "countPerRib": int(per), "spacingM": float(spacing), "totalCount": int(total)}
            parent = {"id": bid, "spanM": float(span), "support": SUPPORT[support], "adjacentSpanM": 0.0}
            rows += RULE.rows(obj, parent)
            line = OBJECT.sub("", line, count=1)
        out.append(line)
    text = "\n".join(out)
    table = "".join(f"  {r['id']} {r['beamId']} {number(r['xM'])}m {number(r['diameterM'])}m {number(r['heightM'])}m {number(r['fUPa'])}Pa\n" for r in rows)
    return re.sub(r"(studs \[id:TEXT beam-id:TEXT x-m:QTY diameter-m:QTY height-m:QTY f-u-pa:QTY\] \{\n)(\})", lambda m: m.group(1) + table + m.group(2), text, count=1), len(rows)


if __name__ == "__main__":
    for path in sorted(p for p in ASSETS.glob("*/*/🗣️.dsl.semio") if "showcase" not in p.parts[-3]):
        new, count = rewrite(path.read_text(encoding="utf8"))
        print(path.parent.name, count, "studs")
        if "--apply" in sys.argv:
            path.write_text(new, encoding="utf8")
