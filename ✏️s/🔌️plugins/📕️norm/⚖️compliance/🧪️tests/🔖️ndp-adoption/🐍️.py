import json, pathlib, re, sys
HERE = pathlib.Path(__file__).resolve().parent
NORM = HERE.parent.parent.parent
REGISTRY = NORM / "⚖️compliance" / "📜️annex-and-tables" / "🔖️ndp" / "🔣️.json"
EXPECTED = HERE.parent / "🧫️fixtures" / "🔖️ndp-annex-differences" / "🔣️.json"
ARTIFACTS = NORM / "🗿️artifacts"
SCHEMA = NORM / "⚖️compliance" / "🧬️schema" / "🔣️.json"
LIBRARY = re.compile(r"\b(blue-prints|structuralcodes|groundhog|concreteproperties|sectionproperties) \d+\.\d+")
PRIVATE = [r"\bUNVERIFIED_NDP\b", r"\bNDP_REGISTRY\b", r"\bpub mod ndp\b", r"pub const UNVERIFIED\b", r"\bpub const NDPS\b", r"\bpub const GAMMA_M_TABLE\b", r"\bis_unverified\b", r"\bndp_row\b"]
CITED = re.compile(r'"(ndp\.en199\d\.[A-Za-z0-9._\-{}]*[A-Za-z0-9}])"')

def registry():
    return {e["id"]: e for e in json.loads(REGISTRY.read_text())["entries"]}

def sources():
    for path in ARTIFACTS.rglob("*.rs"):
        if "🧪️tests" in path.parts or path.name != "🦀️.rs":
            continue
        yield path, path.read_text(errors="ignore")

def cited_unknown():
    reg = registry()
    bad = {}
    for path, text in sources():
        for cited in CITED.findall(text):
            if "{" in cited:
                continue
            if cited in reg or any(i.startswith(cited + ".") for i in reg):
                continue
            bad.setdefault(cited, path.name)
    return bad

def private_lists():
    return [(path.parent.name, pattern) for path, text in sources() for pattern in PRIVATE if re.search(pattern, text)]

def differing():
    return sorted(i for i, e in registry().items() if isinstance(e["recommended"], (int, float)) and isinstance(e["naDe"], (int, float)) and e["recommended"] != e["naDe"])

def library_backed():
    return sorted(i for i, e in registry().items() if (e["status"] == "verified" and not LIBRARY.search(e["evidence"])) or any(side["status"] == "verified" and not LIBRARY.search(side["evidence"]) for side in (e.get("annexStatus") or {}).values()))

def schema_violations():
    try:
        import jsonschema
    except ImportError:
        return None
    definitions = json.loads(SCHEMA.read_text())["definitions"]
    validator = jsonschema.Draft7Validator({"$ref": "#/definitions/NdpEntry", "definitions": definitions})
    return [(i, error.message[:80]) for i, e in registry().items() for error in validator.iter_errors(e)]

def main():
    bad = cited_unknown()
    assert not bad, f"ids cited in artifact code but missing in the registry: {sorted(bad)[:20]}"
    left = private_lists()
    assert not left, f"private NDP lists remain: {left[:10]}"
    expected = json.loads(EXPECTED.read_text())["differing"]
    assert differing() == expected, f"annex differences changed: +{sorted(set(differing()) - set(expected))} -{sorted(set(expected) - set(differing()))}"
    unbacked = library_backed()
    assert not unbacked, f"verified flags without a named library and version: {unbacked[:10]}"
    violations = schema_violations()
    assert not violations, f"entries violating the NdpEntry schema (third-party jsonschema): {violations[:5]}"
    reg = registry()
    for family in (f"en199{n}" for n in range(0, 10)):
        assert any(i.startswith(f"ndp.{family}.") for i in reg), family
    print("ndp adoption ok", len(reg), "entries", len(expected), "annex-sensitive values; schema check", "skipped" if violations is None else "passed")

if __name__ == "__main__":
    main()
