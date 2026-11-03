import json, pathlib, re, sys
HERE = pathlib.Path(__file__).resolve().parent
REGISTRY = HERE.parent.parent / "📜️annex-and-tables" / "🔖️ndp" / "🔣️.json"
MARKERS = re.compile(r"contradict|differs|disagree|should be|TODO|FIXME|\?", re.IGNORECASE)
AS_WRITTEN = re.compile(r"(?:\(spec value used\)|as used by the artifact):?")
PRODUCT_STANDARDS = {"EN 1995": ("EN 338", "EN 14080", "EN 14374", "EN 16351"), "EN 1997": ("DIN 1054",)}

def entries():
    return json.loads(REGISTRY.read_text(encoding="utf-8"))["entries"]

def contradicting_notes(rows):
    bad = []
    for x in rows:
        if x["status"] != "verified":
            continue
        for field in ("specRef", "asWritten", "evidence"):
            if MARKERS.search(x[field]):
                bad.append((x["id"], field))
        numbers = re.findall(r"-?\d+(?:\.\d+)?", x["asWritten"].split(" = ")[-1] if " = " in x["asWritten"] else x["asWritten"].rsplit(" ", 1)[-1])
        if AS_WRITTEN.search(x["asWritten"]) and isinstance(x["recommended"], (int, float)) and numbers and float(numbers[-1]) != float(x["recommended"]):
            bad.append((x["id"], "asWritten value"))
    return bad

def foreign_sources(rows):
    bad = []
    for x in rows:
        number = x["code"].replace("EN ", "")
        document = x["source"]["document"]
        own = re.search(rf"(?<!\d){number}(?!\d)", document) or document.startswith(PRODUCT_STANDARDS.get(x["code"], ("\0",)))
        if x["id"].split(".")[1] != "en" + number or not own:
            bad.append((x["id"], x["code"], document))
    return bad

def main():
    rows = entries()
    assert len(rows) > 1000
    bad = contradicting_notes(rows)
    assert not bad, f"verified rows with contradicting notes: {bad[:10]} ({len(bad)})"
    foreign = foreign_sources(rows)
    assert not foreign, f"rows citing a document outside their code family: {foreign[:10]} ({len(foreign)})"
    synthetic = [dict(rows[0], status="verified", specRef="contradicts the print")]
    assert contradicting_notes(synthetic) == [(rows[0]["id"], "specRef")]
    wrong = [dict(rows[0], code="EN 1998", id="ndp.en1998.x", source={"document": "DIN EN 1997-1/NA:2023-11"})]
    assert len(foreign_sources(wrong)) == 1
    print("ndp registry law ok", len(rows), "rows")

if __name__ == "__main__":
    sys.exit(main())
