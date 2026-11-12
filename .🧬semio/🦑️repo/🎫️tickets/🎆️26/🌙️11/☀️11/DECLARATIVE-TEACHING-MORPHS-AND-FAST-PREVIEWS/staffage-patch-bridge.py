"""One-shot narrow patch of the staffage Python bridge for the LoD resolver (staffage owner). Re-reads the file, applies exact replacements, fails on any miss."""
import sys
from pathlib import Path

path = Path(sys.argv[1])
s = path.read_bytes().decode("utf-8").replace("\r\n", "\n")


def rep(old, new, count=1):
    global s
    if old not in s:
        raise SystemExit("MISSING: " + old[:90])
    s = s.replace(old, new, count)


rep('''A cast is a named, figure-only rendering listed in `🔣️.json`. The TypeScript exporter renders missing casts in one batch into a content-addressed cache,
and this module turns the frames into transformed VGroups with the world origin on `anchor` and `unit` Manim units per metre.
''', '''A cast is a named, figure-only rendering listed in `🔣️.json`. The TypeScript exporter renders missing casts in one batch into a content-addressed cache,
one folder per level of detail (`full`, `preview`, `poster`), and this module turns the frames into transformed VGroups with the world origin on `anchor` and `unit` Manim units per metre.
`resolve` is the one entry that selects the level before anything is decoded: the level comes from the render profile (`lod_scope` for explicit callers), never from a clip.
''')
rep('@see ../../../../🧍️staffage/🔨️modules/📤️export/🎭️cast/🧬️schema/🔣️.schema.json — the manifest schema\n', '@see ../../../../🧍️staffage/🔨️modules/📤️export/🎭️cast/🧬️schema/🔣️.schema.json — the manifest schema\n@see ../../../../🧍️staffage/🔨️modules/📤️export/🎭️cast/🧊️lod/🔣️.json — the levels of detail\n')
rep('from typing import Callable, NamedTuple\n', 'from typing import Callable, Iterator, NamedTuple\n')
rep('_EXPORTER = _SOURCES / "🔨️modules/📤️export/🎭️cast/🟦️.ts"\n', '_EXPORTER = _SOURCES / "🔨️modules/📤️export/🎭️cast/🟦️.ts"\n_LEVELS_FILE = _EXPORTER.with_name("🧊️lod") / "🔣️.json"\n_PROFILE = "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🎚️profile.🐍️"\n_INSTRUMENT = "🎓️teaching.🏛️architecture.🔀️morph.📊️instrument.🐍️"\n')
rep('_MIN_STROKE = 1.0\n', '_IMPORT = re.compile(r"""(?:from|import)\\s*\\(?\\s*["\']([.][^"\']+)["\']""")\n')
rep('_SHAPE = re.compile(r\'<g id="shape-([^"]+)"[^>]*>\\s*(?:<title>[^<]*</title>\\s*)?<path d="([^"]+)"\')\n_COMMAND = re.compile(r"([MLHVZmlhvz])([^MLHVZmlhvz]*)")\n_NUMBER = re.compile(r"-?\\d*\\.?\\d+(?:e-?\\d+)?")\n', '')
rep('_SLOT_COUNTS: dict[Path, int] = {}\n', '_SCOPE: list["ResolvedLod"] = []\n')

# digest narrowed to the cast dependency closure
rep('''@lru_cache(maxsize=None)
def _source_digest(root: Path = _SOURCES) -> str:
    """🔐 Hash every exporter and schema source file and the bundled print profiles once per provenance checkpoint, in path order."""
    digest = hashlib.sha256()
    files = sorted([file for folder in ("🔨️modules", "🧬️schema") for file in (root / folder).rglob("*") if file.is_file() and "node_modules" not in file.parts]
                   + [root / name for name in _BUNDLED if (root / name).is_file()], key=lambda file: file.relative_to(root).as_posix())
''', '''def _imports(root: Path) -> set[Path]:
    """🕸️ The files the cast exporter reaches through relative imports, transitively: the evaluation, depiction, thermal, transport and structure code it runs, and nothing it never loads (DXF, PDF, video, ZIP, sheets, the command line)."""
    start = root / "🔨️modules/📤️export/🎭️cast/🟦️.ts"
    seen: set[Path] = set()
    stack = [start]
    while stack:
        file = stack.pop()
        if file in seen or not file.is_file():
            continue
        seen.add(file)
        if file.suffix == ".ts":
            stack.extend(Path(os.path.normpath(file.parent / name)) for name in _IMPORT.findall(file.read_text(encoding="utf-8")))
    return seen


@lru_cache(maxsize=None)
def _source_digest(root: Path = _SOURCES) -> str:
    """🔐 Hash the cast dependency closure (the exporter, everything it imports, its schema and level files), the staffage schema and the bundled print profiles once per provenance checkpoint, in path order; editing a module the exporter never loads leaves every cast key unchanged."""
    digest = hashlib.sha256()
    owned = root / "🔨️modules/📤️export/🎭️cast"
    files = sorted({*_imports(root), *(file for folder in (owned, root / "🧬️schema") if folder.is_dir() for file in folder.rglob("*") if file.is_file() and "node_modules" not in file.parts),
                    *(root / name for name in _BUNDLED if (root / name).is_file())}, key=lambda file: file.relative_to(root).as_posix())
''')

# levels, resolver types, scope: inserted before cast_key
rep('''def cast_key(entry: dict, root: Path = _SOURCES) -> str:''', '''class ResolvedLod(NamedTuple):
    """🧊 A level of detail with the output frame rate of the job applied: the exact parameters a cached cast was drawn with; `revision` is eight hex digits of their canonical-JSON hash, the twin of `resolveLod`."""

    id: str
    level: dict
    fps_cap: float | None
    revision: str


@lru_cache(maxsize=None)
def _levels() -> dict:
    """🧊 Read the levels file shared with the TypeScript exporter."""
    return json.loads(_LEVELS_FILE.read_text(encoding="utf-8"))["levels"]


def lod_level(lod: str, output_fps: float) -> ResolvedLod:
    """🧊 Resolve a level id for a job that outputs `output_fps` frames per second; an unknown level or rate raises an actionable error."""
    levels = _levels()
    if lod not in levels:
        raise ValueError(f"unknown level of detail {lod!r}; levels: {', '.join(levels)}")
    if isinstance(output_fps, bool) or not isinstance(output_fps, (int, float)) or not 0 < output_fps <= 240:
        raise ValueError(f"the output frame rate must be a number in (0, 240], got {output_fps!r}")
    level = levels[lod]
    fraction = level["frames"]["outputFpsFraction"]
    cap = None if fraction is None else fraction * output_fps
    digest = hashlib.sha256(canonical_json({"id": lod, "level": level, "fpsCap": cap}).encode("utf-8")).hexdigest()
    return ResolvedLod(lod, level, cap, digest[:8])


def lod_schedule(canonical: dict, lod: ResolvedLod) -> dict:
    """📅 The samples a level takes of a canonical clock `{fps, durationS, loop, count}`: the clock itself when the level keeps every frame, otherwise at most `maxFrames` samples at `fps_cap` frames per second; a single sample sits at `phase` of the duration. Returns `{count, fps, times}`; the twin of `lodSchedule`, read against `🧫️fixtures/🧊️cast-lod/🔣️.json`."""
    frames = lod.level["frames"]
    count, duration, loop = canonical["count"], canonical["durationS"], canonical["loop"]
    rate = canonical["fps"] if lod.fps_cap is None else min(canonical["fps"], lod.fps_cap)
    kept = min(count, int(np.floor(duration * rate + 0.5)) + (0 if loop else 1), frames["maxFrames"])
    samples = max(2, kept) if count >= 2 and frames["maxFrames"] >= 2 else kept
    if samples >= count:
        return {"count": count, "fps": canonical["fps"], "times": [frame / canonical["fps"] for frame in range(count)]}
    if samples == 1:
        return {"count": 1, "fps": canonical["fps"], "times": [frames["phase"] * duration]}
    intervals = samples if loop else samples - 1
    return {"count": samples, "fps": intervals / duration, "times": [frame * duration / intervals for frame in range(samples)]}


@contextlib.contextmanager
def lod_scope(lod: str, output_fps: float) -> Iterator[ResolvedLod]:
    """🔭 Draw casts at an explicitly named level inside the block (tests, parent processes); a scene process gets its level from the render profile and clips never name one."""
    resolved = lod_level(lod, output_fps)
    _SCOPE.append(resolved)
    try:
        yield resolved
    finally:
        _SCOPE.pop()


def active_lod() -> ResolvedLod:
    """🎯 The level of the running job: the innermost `lod_scope`, else the level of the render profile at the output rate of the job; without either there is no level and the call fails (there is no default)."""
    if _SCOPE:
        return _SCOPE[-1]
    profile = importlib.import_module(_PROFILE)
    job = profile.job()
    return lod_level(job.profile.lod, job.fps)


def cast_key(entry: dict, root: Path = _SOURCES) -> str:''')

rep('''def _directory(cast: str) -> Path:
    """📁 Name the cache folder of a cast."""
    return _cache_root() / cast_key(_entry(cast)) / cast
''', '''def _directory(cast: str, lod: ResolvedLod | None = None) -> Path:
    """📁 Name the cache folder of a cast at a level of detail: `<key>/<cast>/<level>-<revision>`, so a level never serves another."""
    lod = lod or active_lod()
    return _cache_root() / cast_key(_entry(cast)) / cast / f"{lod.id}-{lod.revision}"
''')
rep('''def _lock(cast: str) -> _Lock:
    """🏷️ Name the lock of one cast beside its cache folder, so renderers of the same source and entry share it."""
    return _Lock(_directory(cast).parent / f".lock-{cast}")''', '''def _lock(cast: str, lod: ResolvedLod) -> _Lock:
    """🏷️ Name the lock of one cast at one level beside its cache folder, so renderers of the same source, entry and level share it."""
    return _Lock(_directory(cast, lod).parent.parent / f".lock-{cast}-{lod.id}")''')
rep('def _render(casts: list[str], folders: dict[str, Path], progress: Callable[[str], None] | None) -> None:\n    """🎬 Export `casts` in one exporter call and publish each into the cache."""',
    'def _render(casts: list[str], folders: dict[str, Path], lod: ResolvedLod, output_fps: float, progress: Callable[[str], None] | None) -> None:\n    """🎬 Export `casts` at one level in one exporter call and publish each into the cache."""')
rep('"--out", str(temporary / "out"), "--parent", str(os.getpid())]', '"--out", str(temporary / "out"), "--lod", lod.id, "--output-fps", _number(output_fps), "--parent", str(os.getpid())]')
rep('''def ensure_casts(ids: list[str] | tuple[str, ...] | None = None, progress: Callable[[str], None] | None = None) -> dict[str, Path]:''', '''def ensure_casts(ids: list[str] | tuple[str, ...] | None = None, progress: Callable[[str], None] | None = None, lod: ResolvedLod | None = None) -> dict[str, Path]:''')
rep('''    wanted = list(dict.fromkeys(ids if ids is not None else _manifest()["casts"]))
    folders = {cast: _directory(cast) for cast in wanted}''', '''    lod = lod or active_lod()
    wanted = list(dict.fromkeys(ids if ids is not None else _manifest()["casts"]))
    folders = {cast: _directory(cast, lod) for cast in wanted}''')
rep('lock = _lock(cast)\n', 'lock = _lock(cast, lod)\n')
rep('_render([cast for cast in pending if cast in held], folders, progress)', '_render([cast for cast in pending if cast in held], folders, lod, _output_fps(lod), progress)')
rep('''        if folder not in _CHECKED:
            _provenance(cast, _index(folder))
            _CHECKED.add(folder)''', '''        if folder not in _CHECKED:
            _provenance(cast, _index(folder), lod)
            _CHECKED.add(folder)
            importlib.import_module(_INSTRUMENT).count(f"asset.load[{lod.id}]")''')
rep('''def casts_in(sources''', '''def _output_fps(lod: ResolvedLod) -> float:
    """🎞️ The output frame rate to hand the exporter for a resolved level: the rate whose fraction gave its sample cap; a level without a cap does not depend on it, so any valid rate names it."""
    fraction = lod.level["frames"]["outputFpsFraction"]
    return 30.0 if fraction is None or lod.fps_cap is None else lod.fps_cap / fraction


def casts_in(sources''')
rep('def expected_provenance(cast: str, root: Path = _SOURCES) -> dict:', 'def expected_provenance(cast: str, lod: ResolvedLod, root: Path = _SOURCES) -> dict:')
rep('''        "state": {"kind": "clip", "id": entry["clip"]} if "clip" in entry else {"kind": "pose", "id": entry["pose"]},
    }
    if entry["view"] == "thermal":''', '''        "state": {"kind": "clip", "id": entry["clip"]} if "clip" in entry else {"kind": "pose", "id": entry["pose"]},
        "lod": {"id": lod.id, "revision": lod.revision},
    }
    if entry["view"] == "thermal":''')
rep('def _provenance(cast: str, index: dict, root: Path = _SOURCES) -> dict:', 'def _provenance(cast: str, index: dict, lod: ResolvedLod, root: Path = _SOURCES) -> dict:')
rep('expected = expected_provenance(cast, root)', 'expected = expected_provenance(cast, lod, root)')
rep('''        raise ValueError(f"cast {cast!r}: provenance clock is {clock!r} but its index says {wanted!r}; export it again")
    return found''', '''        raise ValueError(f"cast {cast!r}: provenance clock is {clock!r} but its index says {wanted!r}; export it again")
    sampled = index.get("lod")
    canonical = {"fps": sampled.get("canonicalFps"), "durationS": index["durationS"], "loop": index["loop"], "count": sampled.get("canonicalFrames")} if isinstance(sampled, dict) else None
    if canonical is None or {key: sampled.get(key) for key in ("id", "revision")} != {"id": lod.id, "revision": lod.revision}:
        raise ValueError(f"cast {cast!r}: its index drew level {sampled!r} but {lod.id}-{lod.revision} was asked for; export it again")
    schedule = lod_schedule(canonical, lod)
    if schedule["count"] != len(index["frames"]) or abs(schedule["fps"] - index["fps"]) > 1e-9 * max(1.0, schedule["fps"]):
        raise ValueError(f"cast {cast!r}: level {lod.id} samples {schedule['count']} frames at {schedule['fps']} fps of the canonical clock but the index holds {len(index['frames'])} at {index['fps']}; export it again")
    return found''')
rep('''def staffage_provenance(cast: str, rate: float | None = None) -> dict:''', '''def staffage_provenance(cast: str, rate: float | None = None) -> dict:''')
rep('''        key = ensure_casts([cast])[cast].parent.name''', '''        key = ensure_casts([cast])[cast].parent.parent.name''')

# slot count from the index
rep('''def _slot_count(folder: Path) -> int:
    """🔢 Count the most paths any frame of a cast draws, reading each distinct frame file once."""
    if folder not in _SLOT_COUNTS:
        names = dict.fromkeys(_index(folder)["frames"])
        _SLOT_COUNTS[folder] = max(((folder / name).read_bytes().count(b"<path") for name in names), default=0)
    return _SLOT_COUNTS[folder]
''', '''def _slot_count(folder: Path) -> int:
    """🔢 The most paths any frame of a cast draws, written by the exporter into the index: no frame is read."""
    return int(_index(folder)["logical"]["maxPaths"])
''')
rep('''def _style(shape: _Shape, factor: float, color: str | None, opacity: float) -> tuple[str | None, float, str | None, float, float]:
    """🖌️ Resolve the Manim fill, stroke and width of a path; the stroke width follows the physical scale and never drops below one."""
    width = max(shape.stroke_mm * factor / _STROKE_UNIT, _MIN_STROKE) if shape.stroke_mm > 0 else 0.0''', '''def _stroke_minimum(index: dict) -> float:
    """🖊️ The thinnest stroke the level of detail of a cast draws, from the levels file."""
    return float(_levels()[index["lod"]["id"]]["strokeMin"])


def _style(shape: _Shape, factor: float, color: str | None, opacity: float, minimum: float) -> tuple[str | None, float, str | None, float, float]:
    """🖌️ Resolve the Manim fill, stroke and width of a path; the stroke width follows the physical scale and never drops below the `minimum` of the level."""
    width = max(shape.stroke_mm * factor / _STROKE_UNIT, minimum) if shape.stroke_mm > 0 else 0.0''')
rep('''def _bounds(index: dict, key: str, mirror: bool) -> dict:
    """🧱 Return the bounds of a cast in world metres, flipped about the world origin when `mirror` is set."""
    box = index[key]''', '''def _bounds(index: dict, mirror: bool) -> dict:
    """🧱 Return the figure bounds of a cast in world metres (the logical descriptor, identical for every level of detail and free of any air layer), flipped about the world origin when `mirror` is set."""
    box = index["logical"]["figureBoundsM"]''')
rep('_apply(member, _style(shape, factor, recolour, opacity))', '_apply(member, _style(shape, factor, recolour, opacity, _stroke_minimum(index)))')
rep('''    return float(_index(ensure_casts([cast])[cast])["heightM"])''', '''    return float(_index(ensure_casts([cast])[cast])["logical"]["heightM"])''')
rep('''    index = _index(ensure_casts([cast])[cast])
    box = index.get("figureBoundsM", index["boundsM"])
    return''', '''    box = _index(ensure_casts([cast])[cast])["logical"]["figureBoundsM"]
    return''')

# parts from the descriptor
i = s.index('def staffage_parts(cast: str, prefix: str = "")')
j = s.index('def staffage(cast: str, *, unit: float')
s = s[:i] + '''def staffage_parts(cast: str, prefix: str = "") -> dict[str, np.ndarray]:
    """🔩 Return the filled polygons of the first frame whose shape id starts with `prefix`, as (n, 2) arrays in world metres (y up, origin on the feet), keyed by the shape id without its `shape-` prefix.

    Clips derive attachment points (ray starts, labels) from these instead of hard-coding positions. They come from the logical descriptor the exporter writes for every level of detail, so an anchor never depends on which frames were loaded.
    """
    parts = _index(ensure_casts([cast])[cast])["logical"]["partsM"]
    return {name: np.array(points, dtype=float) for name, points in parts.items() if name.startswith(prefix)}


''' + s[j:]
rep('''    group = _Figure(group, folder, unit, anchor, mirror) if window is None and "figureBoundsM" in _index(folder) else group''', '''    group = _Figure(group, folder, unit, anchor, mirror) if window is None else group''')
rep('''        bounds = _bounds(_index(folder), "figureBoundsM", mirror)''', '''        bounds = _bounds(_index(folder), mirror)''')
rep('''    """🪜 A group whose width, height, top, bottom and centre follow its `probes` when `_figure_only` is set: the air layer of a transport cast reaches far beyond the person and must not size, place or align the group."""

    _figure_only = False''', '''    """🪜 A group whose width, height, top, bottom and centre follow its `probes` when `_figure_only` is set: the logical figure bounds, identical for every level of detail, so neither the air layer of a transport cast nor the paths a level draws size, place or align the group."""

    _figure_only = True''')
rep('''    """🪵 A static transport cast bounded by its figure alone."""''', '''    """🪵 A static cast bounded by its figure alone."""''')
rep('''        bounds = _bounds(index, "figureBoundsM" if "figureBoundsM" in index else "boundsM", mirror)
        self._figure_only = "figureBoundsM" in index
''', '''        bounds = _bounds(index, mirror)
''')
rep('''color if color is not None and index["view"].endswith(_LINE_SUFFIX) else None, mirror)
        self.content''', '''color if color is not None and index["view"].endswith(_LINE_SUFFIX) else None, mirror, _stroke_minimum(index))
        self.content''')
rep('''    __slots__ = ("folder", "unit", "anchor", "factor", "color", "mirror")

    def __init__(self, folder: Path, unit: float, anchor: np.ndarray, factor: float, color: str | None, mirror: bool = False):
        self.mirror = mirror''', '''    __slots__ = ("folder", "unit", "anchor", "factor", "color", "mirror", "minimum")

    def __init__(self, folder: Path, unit: float, anchor: np.ndarray, factor: float, color: str | None, mirror: bool = False, minimum: float = 1.0):
        self.minimum = minimum
        self.mirror = mirror''')
rep('_apply(slot, _style(parsed.shapes[position], source.factor, source.color, fade))', '_apply(slot, _style(parsed.shapes[position], source.factor, source.color, fade, source.minimum))')
rep('self.content = f"{folder.parent.name}/{folder.name}"', 'self.content = f"{folder.parent.parent.name}/{folder.parent.name}/{folder.name}"')
path.write_bytes(s.encode("utf-8"))
print("patched")
