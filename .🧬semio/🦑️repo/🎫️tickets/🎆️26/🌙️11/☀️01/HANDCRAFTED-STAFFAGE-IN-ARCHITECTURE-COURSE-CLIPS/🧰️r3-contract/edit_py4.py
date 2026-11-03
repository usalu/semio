import sys
sys.path.insert(0, sys.argv[1])
from edit_py_helper import edit
p = sys.argv[2]
s = open(p, encoding="utf-8", newline="").read()
crlf = "\r\n" in s
s = s.replace("\r\n", "\n")
a = s.index("def _provenance(")
b = s.index("_DRAWN: list[dict] = []")
new = '''def expected_provenance(cast: str, root: Path = _SOURCES) -> dict:
    """🔖 Derive what the provenance of a rendered cast must say from its manifest entry, the catalog asset and the environment presets: every dimension except the clock, which the exporter measures."""
    entry = _entry(cast)
    found = {
        "version": 1,
        "asset": {"id": entry["asset"], "revision": asset_revision(entry["asset"], root)},
        "view": {"style": entry["view"], "viewpoint": entry["viewpoint"], "theme": entry["theme"], "mirror": entry.get("mirror", False)},
        "scale": {"denominator": entry["scale"]},
        "state": {"kind": "clip", "id": entry["clip"]} if "clip" in entry else {"kind": "pose", "id": entry["pose"]},
    }
    if entry["view"] == "thermal":
        revisions = _preset_revisions(entry, root)
        scenario = {"kind": "thermal", "spanK": entry.get("thermalSpanK"), "transport": entry.get("transport", [])}
        if "timeline" in entry:
            scenario["timeline"] = [{"t": key["t"], "id": key["environment"], "ease": key["ease"], "revision": revisions[key["environment"]]} for key in sorted(entry["timeline"], key=lambda key: key["t"])]
        else:
            scenario["environment"] = {"id": entry["environment"], "revision": revisions[entry["environment"]]}
    else:
        scenario = {"kind": "structural" if entry["view"] in ("forces", "moments") else "geometry"}
    return {**found, "scenario": scenario}


def _provenance(cast: str, index: dict, root: Path = _SOURCES) -> dict:
    """🔖 Check the provenance of a rendered cast against its manifest entry, the catalog asset, the environment presets and its own index, and return it; a missing or differing dimension raises."""
    found = index.get("provenance")
    if not isinstance(found, dict):
        raise ValueError(f"cast {cast!r}: its rendered index has no provenance record; export it again")
    expected = expected_provenance(cast, root)
    for section, named in expected.items():
        drawn = found.get(section)
        if section == "asset" and isinstance(drawn, dict):
            drawn = {key: drawn.get(key) for key in named}
        if section == "scenario" and isinstance(drawn, dict) and named["kind"] == "structural":
            drawn = {"kind": drawn.get("kind")}
        if drawn != named:
            raise ValueError(f"cast {cast!r}: provenance {section} is {drawn!r} but the manifest and the catalog say {named!r}; export it again")
    clock = found.get("clock")
    wanted = {"fps": index.get("fps"), "durationS": index.get("durationS"), "loop": index.get("loop"), "frames": len(index.get("frames", []))}
    if not isinstance(clock, dict) or any(clock.get(key) != value for key, value in wanted.items()) or "offsetS" not in clock:
        raise ValueError(f"cast {cast!r}: provenance clock is {clock!r} but its index says {wanted!r}; export it again")
    return found


'''
s = s[:a] + new + s[b:]
c = s.index("def staffage_provenance(")
d = s.index("def drawn_casts(")
s = s[:c] + '''def staffage_provenance(cast: str, rate: float | None = None) -> dict:
    """🔖 Return the checked provenance record of a cast for a scene: asset id and revision, view, authored scale, pose or clip, clock and scenario; `rate` adds the scene's playback rate to the clock."""
    record = json.loads(json.dumps(_index(ensure_casts([cast])[cast])["provenance"]))
    if rate is not None:
        record["clock"]["rate"] = rate
    return record


''' + s[d:]
s = s.replace('''        _DRAWN.append({"cast": cast, "key": cast_key(_entry(cast)), **record})''', '''        _DRAWN.append({"cast": cast, "key": ensure_casts([cast])[cast].parent.name, **record})''')
if crlf:
    s = s.replace("\n", "\r\n")
open(p, "w", encoding="utf-8", newline="").write(s)
