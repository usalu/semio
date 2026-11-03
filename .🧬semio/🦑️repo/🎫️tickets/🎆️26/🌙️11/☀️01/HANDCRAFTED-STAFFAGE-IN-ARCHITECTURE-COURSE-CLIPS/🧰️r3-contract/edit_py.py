import sys


def edit(p, pairs):
    s = open(p, encoding="utf-8", newline="").read()
    crlf = "\r\n" in s
    s = s.replace("\r\n", "\n")
    for a, b in pairs:
        assert s.count(a) == 1, (a, s.count(a))
        s = s.replace(a, b)
    if crlf:
        s = s.replace("\n", "\r\n")
    open(p, "w", encoding="utf-8", newline="").write(s)


NEW_KEY = '''def _number(value: float) -> str:
    """🔢 Print a number the way ECMAScript does: integral values without a fraction, shortest round-trip digits, `e-7` instead of `e-07`."""
    if isinstance(value, bool) or not isinstance(value, (int, float)):
        raise TypeError(f"not a number: {value!r}")
    if float(value).is_integer() and abs(value) < 1e21:
        return str(int(value))
    text = repr(float(value))
    if "e" in text:
        mantissa, exponent = text.split("e")
        return f"{mantissa}e{'+' if int(exponent) > 0 else '-'}{abs(int(exponent))}"
    return text


def canonical_json(value) -> str:
    """🧾 Serialise JSON canonically: sorted keys, no whitespace, non-ASCII kept; byte-identical to `canonicalJson` of the TypeScript exporter."""
    if isinstance(value, dict):
        return "{" + ",".join(f"{json.dumps(key, ensure_ascii=False)}:{canonical_json(value[key])}" for key in sorted(value)) + "}"
    if isinstance(value, list):
        return "[" + ",".join(canonical_json(item) for item in value) + "]"
    if isinstance(value, (int, float)) and not isinstance(value, bool):
        return _number(value)
    return json.dumps(value, ensure_ascii=False)


def revision_of(data: bytes) -> str:
    """🔖 Name a revision: `sha256:` and the hex digest of the bytes with CRLF turned into LF."""
    return "sha256:" + hashlib.sha256(_lf(data)).hexdigest()


def asset_revision(slug: str, root: Path = _SOURCES) -> str:
    """🔖 Return the content hash of an asset's catalog file."""
    return revision_of(_catalog_bytes(slug, root))


def preset_revision(preset: dict) -> str:
    """🔖 Return the content hash of an environment preset in canonical JSON."""
    return "sha256:" + hashlib.sha256(canonical_json(preset).encode("utf-8")).hexdigest()


def _preset_revisions(entry: dict, root: Path = _SOURCES) -> dict[str, str]:
    """🌤 Return the revision of every environment preset a cast names, failing on an unknown one."""
    presets = _presets(root)
    slugs = sorted({*([entry["environment"]] if "environment" in entry else []), *(key["environment"] for key in entry.get("timeline", []))})
    unknown = [slug for slug in slugs if slug not in presets]
    if unknown:
        raise KeyError(f"unknown environment preset {', '.join(unknown)}; presets: {', '.join(sorted(presets))}")
    return {slug: preset_revision(presets[slug]) for slug in slugs}


def entry_issues(entry, path: str = "cast") -> list[str]:
    """✅ List every violation of the cast schema in one manifest entry, with the messages of the TypeScript twin; empty when the entry is valid."""
    if not isinstance(entry, dict):
        return [f"{path}: must be an object"]
    issues: list[str] = []
    slug = re.compile(r"^[a-z0-9]+(?:-[a-z0-9]+)*$")
    is_slug = lambda value: isinstance(value, str) and len(value) <= 64 and slug.match(value) is not None
    positive = lambda value: isinstance(value, (int, float)) and not isinstance(value, bool) and value > 0
    for key in entry:
        if key not in _CAST_KEYS:
            issues.append(f"{path}: unknown property {key}")
    for key in ("asset", "pose", "clip", "environment"):
        if key in entry and not is_slug(entry[key]):
            issues.append(f"{path}/{key}: must be a slug")
    if "asset" not in entry:
        issues.append(f"{path}: asset is required")
    if "theme" not in entry:
        issues.append(f"{path}: theme is required")
    elif entry["theme"] not in _THEMES:
        issues.append(f"{path}/theme: must be one of {', '.join(_THEMES)}")
    layers = entry.get("transport")
    if "transport" in entry and not (isinstance(layers, list) and all(layer in _TRANSPORT_LAYERS for layer in layers) and len(set(layers)) == len(layers)):
        issues.append(f"{path}/transport: must be unique layers of {', '.join(_TRANSPORT_LAYERS)}")
    for key in ("transportAgeS", "transportStrokeMm", "transportTrailS"):
        if key in entry and not positive(entry[key]):
            issues.append(f"{path}/{key}: must be a positive number")
        if key in entry and "transport" not in entry:
            issues.append(f"{path}: {key} needs transport layers")
    if "transportAgeS" in entry and "clip" not in entry:
        issues.append(f"{path}: transportAgeS needs a clip")
    if "transport" in entry and "transportAgeS" not in entry:
        issues.append(f"{path}: transport layers need transportAgeS, the explicit start of the field")
    if "transport" in entry and entry.get("view") != "thermal":
        issues.append(f"{path}: transport layers exist for the thermal view only")
    if entry.get("view") not in _VIEWS:
        issues.append(f"{path}/view: must be one of {', '.join(_VIEWS)}")
    if entry.get("viewpoint") not in _VIEWPOINTS:
        issues.append(f"{path}/viewpoint: is required and must be one of {', '.join(_VIEWPOINTS)}")
    if entry.get("mirror") is True and entry.get("viewpoint") != "elevation":
        issues.append(f"{path}: mirror needs viewpoint elevation, a mirrored front or plan would fake a view")
    if entry.get("scale") not in _SCALES or isinstance(entry.get("scale"), bool):
        issues.append(f"{path}/scale: is required and must be one of {', '.join(map(str, _SCALES))}")
    fps = entry.get("fps")
    if "fps" in entry and not (isinstance(fps, int) and not isinstance(fps, bool) and 1 <= fps <= 120):
        issues.append(f"{path}/fps: must be an integer in 1..120")
    if "thermalSpanK" in entry and not positive(entry["thermalSpanK"]):
        issues.append(f"{path}/thermalSpanK: must be a positive number of kelvin")
    if "thermalSpanK" in entry and entry.get("view") != "thermal":
        issues.append(f"{path}: thermalSpanK exists for the thermal view only")
    if "mirror" in entry and not isinstance(entry["mirror"], bool):
        issues.append(f"{path}/mirror: must be a boolean")
    if ("pose" in entry) == ("clip" in entry):
        issues.append(f"{path}: give exactly one of pose and clip")
    if ("environment" in entry or "timeline" in entry) and entry.get("view") != "thermal":
        issues.append(f"{path}: environment and timeline exist for the thermal view only")
    if "environment" in entry and "timeline" in entry:
        issues.append(f"{path}: environment and timeline exclude each other")
    if "fps" in entry and not ("clip" in entry or "timeline" in entry):
        issues.append(f"{path}/fps: needs a clip or a timeline")
    if "fps" not in entry and ("clip" in entry or "timeline" in entry):
        issues.append(f"{path}/fps: is required for a clip or a timeline")
    if entry.get("view") == "thermal" and not ("environment" in entry or "timeline" in entry):
        issues.append(f"{path}: the thermal view needs an environment or a timeline")
    if "timeline" in entry:
        timeline = entry["timeline"]
        if not isinstance(timeline, list) or not timeline:
            issues.append(f"{path}/timeline: must be a non-empty array")
        else:
            for position, key in enumerate(timeline):
                at = f"{path}/timeline/{position}"
                if not isinstance(key, dict):
                    issues.append(f"{at}: must be an object")
                    continue
                issues.extend(f"{at}: unknown property {name}" for name in key if name not in ("t", "environment", "ease"))
                if not (isinstance(key.get("t"), (int, float)) and not isinstance(key.get("t"), bool) and key["t"] >= 0):
                    issues.append(f"{at}/t: must be a number >= 0")
                if not is_slug(key.get("environment")):
                    issues.append(f"{at}/environment: must be a slug")
                if key.get("ease") not in ("step", "linear"):
                    issues.append(f"{at}/ease: is required and must be step or linear")
    return issues


def cast_key(entry: dict, root: Path = _SOURCES) -> str:
    """🔑 Derive the content address of a cast from its validated entry (every view, scale, state, clock and scenario field), the revision of its asset, the revision of every environment preset it names and the exporter sources."""
    issues = entry_issues(entry, f"cast entry for {entry.get('asset')!r}")
    if issues:
        raise ValueError("invalid cast entry\\n" + "\\n".join(issues))
    digest = hashlib.sha256()
    digest.update(json.dumps(entry, sort_keys=True, separators=(",", ":"), ensure_ascii=False).encode("utf-8") + b"\\0")
    digest.update(asset_revision(entry["asset"], root).encode("ascii") + b"\\0")
    digest.update(json.dumps(_preset_revisions(entry, root), sort_keys=True, separators=(",", ":")).encode("ascii") + b"\\0")
    digest.update(_source_digest(root).encode("ascii"))
    return digest.hexdigest()[:_KEY_LENGTH]
'''

PROVENANCE = '''def _provenance(cast: str, index: dict, root: Path = _SOURCES) -> dict:
    """🔖 Check the provenance of a rendered cast against its manifest entry, the catalog asset and the environment presets, and return it; any missing or differing dimension raises."""
    entry = _entry(cast)
    found = index.get("provenance")
    if not isinstance(found, dict):
        raise ValueError(f"cast {cast!r}: its rendered index has no provenance record; export it again")
    asset, view, state = found.get("asset", {}), found.get("view", {}), found.get("state", {})
    expected = {
        "asset.id": (asset.get("id"), entry["asset"]),
        "asset.revision": (asset.get("revision"), asset_revision(entry["asset"], root)),
        "view.style": (view.get("style"), entry["view"]),
        "view.viewpoint": (view.get("viewpoint"), entry["viewpoint"]),
        "view.theme": (view.get("theme"), entry["theme"]),
        "view.mirror": (view.get("mirror"), entry.get("mirror", False)),
        "scale.denominator": (found.get("scale", {}).get("denominator"), entry["scale"]),
        "state": ((state.get("kind"), state.get("id")), ("clip", entry["clip"]) if "clip" in entry else ("pose", entry["pose"])),
    }
    scenario = found.get("scenario", {})
    if entry["view"] == "thermal":
        revisions = _preset_revisions(entry, root)
        expected["scenario.kind"] = (scenario.get("kind"), "thermal")
        expected["scenario.spanK"] = (scenario.get("spanK"), entry.get("thermalSpanK"))
        expected["scenario.transport"] = (scenario.get("transport"), entry.get("transport", []))
        if "timeline" in entry:
            keys = sorted(entry["timeline"], key=lambda key: key["t"])
            expected["scenario.timeline"] = (scenario.get("timeline"), [{"t": key["t"], "id": key["environment"], "ease": key["ease"], "revision": revisions[key["environment"]]} for key in keys])
        else:
            expected["scenario.environment"] = (scenario.get("environment"), {"id": entry["environment"], "revision": revisions[entry["environment"]]})
    else:
        expected["scenario.kind"] = (scenario.get("kind"), "structural" if entry["view"] in ("forces", "moments") else "geometry")
    for field, (drawn, named) in expected.items():
        if drawn != named:
            raise ValueError(f"cast {cast!r}: provenance {field} is {drawn!r} but the manifest and the catalog say {named!r}; export it again")
    return found


_DRAWN: list[dict] = []


def staffage_provenance(cast: str, rate: float | None = None) -> dict:
    """🔖 Return the checked provenance record of a cast for a scene: asset id and revision, view, authored scale, pose or clip, clock and scenario; `rate` adds the scene's playback rate to the clock."""
    record = json.loads(json.dumps(_provenance(cast, _index(ensure_casts([cast])[cast]))))
    if rate is not None:
        record["clock"]["rate"] = rate
    return record


def drawn_casts() -> list[dict]:
    """📋 Return the provenance of every cast drawn in this process since `clear_drawn_casts`, in drawing order, each with its cast id and key."""
    return json.loads(json.dumps(_DRAWN))


def clear_drawn_casts() -> None:
    """🧹 Forget the casts drawn so far, for example at the start of a scene."""
    _DRAWN.clear()


def _drawn(cast: str, rate: float | None) -> dict:
    """📌 Record a drawn cast and return its provenance."""
    record = staffage_provenance(cast, rate)
    _DRAWN.append({"cast": cast, "key": cast_key(_entry(cast)), **record})
    return record


'''

p = sys.argv[1]
s = open(p, encoding="utf-8", newline="").read()
crlf = "\r\n" in s
s = s.replace("\r\n", "\n")
start = s.index("def _environment_bytes(")
end = s.index("def _directory(cast: str)")
s = s[:start] + NEW_KEY + "\n\n" + s[end:]
if crlf:
    s = s.replace("\n", "\r\n")
open(p, "w", encoding="utf-8", newline="").write(s)

edit(p, [
    ('_VIEWPOINTS = ("elevation", "front", "plan")\n',
     '_VIEWPOINTS = ("elevation", "front", "plan")\n_VIEWS = ("dark-line", "dark-solid", "light-line", "light-solid", "thermal", "forces", "moments")\n_THEMES = ("dark", "light")\n_SCALES = (500, 200, 100, 50)\n_TRANSPORT_LAYERS = ("parcels", "pathlines", "streamlines", "condensation")\n_CAST_KEYS = frozenset(("asset", "pose", "clip", "view", "viewpoint", "scale", "fps", "environment", "timeline", "thermalSpanK", "mirror", "theme", "transport", "transportAgeS", "transportStrokeMm", "transportTrailS"))\n'),
    ('''        drawn, named = _index(folder)["viewpoint"], _manifest()["casts"][cast]["viewpoint"]
        if drawn != named:
            raise ValueError(f"cast {cast!r} is named {named!r} in the manifest but its rendered index {folder / _INDEX} drew {drawn!r}")
''', '''        drawn, named = _index(folder)["viewpoint"], _manifest()["casts"][cast]["viewpoint"]
        if drawn != named:
            raise ValueError(f"cast {cast!r} is named {named!r} in the manifest but its rendered index {folder / _INDEX} drew {drawn!r}")
        _provenance(cast, _index(folder))
'''),
    ('''def _index(folder: Path) -> dict:''', PROVENANCE + '''def _index(folder: Path) -> dict:'''),
])
