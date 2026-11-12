# Interfaces between owners (append your own section; re-read before writing)

Module paths below are relative to `🎓️teaching/🏛️architecture/🔀️morph/` (`M`), imported literally with `importlib.import_module`.

## schema owner (spec, stage, facade) - written first, binding for callers of stage

### `M/📋️spec/🐍️.py` (pure Python, no Manim, no NumPy)

- Frozen dataclasses: `Document(version, transitions, groups)`, `Transition(id, intent, source, target, relation, recipe, timing, correspondence, copy, placement, final)` (`source`/`target` are `Endpoint | None`, `correspondence` is `Correspondence | None`), `Endpoint(entity, revision, sampling, view, scale, pose, scenario)`, `Correspondence(kind, pairs, name)`, `Timing(duration, easing, lag, limit)`, `Placement(space, anchor, fit, uniform)`, `Final(path_arc, dip, ghost)`, `Group(id, mode, transitions)`. All fields explicit after `parse` (schema defaults applied). Python attribute names are snake_case; JSON names are camelCase (`pathArc`).
- `load(path) -> Document`, `parse(dict) -> Document` (raise `SpecError(problems)` when invalid), `dump(Document) -> dict` (round trip: `parse(dump(d)) == d`), `validate(document_or_dict, stage=None, *, only=None) -> list[Problem]`, `Problem(path, code, message)`.
- `Document.transition(id) -> Transition`.
- Recipes: `parts hull scatter gather split unfold retext flight move replace vanish appear`. `replace` = Manim ReplacementTransform (one-to-one | many-to-many). `vanish` = `target: null` (entity morphs into nothing), `appear` = `source: null`; both take no `correspondence`, relation one-to-one; every other recipe needs both endpoints non-null and a `correspondence`. `copy: true` (top-level transition flag, Manim TransformFromCopy) means the source stays in the scene after the transition (needs both endpoints); applies to preview and final alike. `ghost` stays final-only (`final.ghost`). `intent` is always required.
- Direct Manim transforms map: `ReplacementTransform` -> `replace`, `TransformFromCopy` -> `replace` + `copy: true`, `FadeTransform`/crossfade of unrelated objects -> `replace` (final owner decides the look), value text swap -> `retext`, moving one identity -> `move`, token along a path -> `flight`, `FadeOut` of an entity -> `vanish`, `FadeIn` -> `appear`.
- `easing` is one of `linear smooth ease-in ease-out ease-in-out overshoot`. `Timing.limit` is `None` or seconds >= duration. Time semantics belong to the timeline owner: `timing.duration` is canonical seconds.
- Stage duck type used by `validate(..., stage)`: `stage.bound(entity_id) -> Bound|None` (`Bound` has `revision`, `provides`, `part_names()`), `stage.correspondences -> frozenset[str]|None` (registered `recipe` correspondence names; None skips the check).

### `M/🎭️stage/🐍️.py`

```python
stage(scene, document_path) -> Stage            # loads + validates once; raises SpecError
Stage.bind(entity_id, value, *, bounds=None, anchors=None, revision=1, provides=None, part_bounds=None, frame=None)
#   value: Mobject | Mapping[str, Mobject|factory] | Sequence[Mobject] | factory(lod) -> Mobject
#   bounds: ((xmin, ymin, zmin), (xmax, ymax, zmax)) logical bounds; REQUIRED for factories,
#           defaults to the bound Mobject's bounds at bind time, frozen afterwards.
#   anchors: {"center": (x, y, z), "origin": (x, y, z), ...} frozen logical anchors; defaults derived from bounds.
#   provides: {"view": [...], "scale": [...], "pose": [...], "scenario": [...]} optional supported values; the validator reports
#             a document endpoint that asks for an unsupported one.
#   part_bounds: {part name: (min, max)} frozen at bind (defaults measured from non-factory parts); frame: placement.Frame for nested spaces.
Stage.cut(transition_id) -> None                # duration-0 transitions: atomic swap at the current time for BOTH profiles (removes source unless copy, adds the exact target as bound; null endpoints only remove/add); no animation, no frames
Stage.played -> tuple[str, ...]                 # ids requested via transition/cut in call order (curriculum check)
Stage.mobject(entity_id) -> Mobject             # lazy: resolves at the active profile lod (for clip code that adds a source to the scene)
Stage.transition(transition_id) -> Animation    # reads profile.active(), dispatches, returns ONE Manim animation; duration 0 raises ValueError pointing at cut()
Stage.register_binder(kind, fn)                 # staffage extension point, see below
```

`Subject` (frozen, defined in the stage module, passed duck-typed to the owners):

| attribute | meaning |
| --- | --- |
| `id` | stable entity id |
| `mobject` | resolved representation at `lod` (a `VGroup` for mappings/lists) |
| `parts` | `dict[str, Mobject]` part name -> mobject (mapping keys; `"0".."n-1"` for lists and single-Mobject submobjects) |
| `bounds` | `numpy.ndarray` shape (2, 3), logical min/max frozen at bind (never from LoD geometry) |
| `anchors` | `dict[str, numpy.ndarray]` logical anchors frozen at bind (`center`, `origin`, ... always present) |
| `lod` | the lod the factory was asked for (`None` for plain Mobjects) |
| `revision` | int declared at bind |
| `part_bounds` | `dict[str, ndarray (2, 3)]` frozen per-part logical bounds (may lack factory parts) |

Calls the stage makes (profile = `profile.active()`; `fps = float(manim.config.frame_rate)`):

```python
# morph == "swap"  -> M/🖼️preview/🐍️.py   (preview owner)
PreviewSwap(source: Subject | None, target: Subject | None, *, transition: spec.Transition, fps: float, frames: dict | None) -> manim.Animation   # frames only when some entity bound a frame
# morph == "full"  -> M/🎞️full/🐍️.py     (final-morph owner)
execute(source: Subject | None, target: Subject | None, *, transition: spec.Transition, fps: float) -> manim.Animation
# source is None for recipe appear, target is None for recipe vanish; copy=True means do not remove the source (TransformFromCopy)
```

- `run_time` of the returned animation is `transition.timing.duration`. Everything else (placement, easing, correspondence, recipe, final params) is read from `transition`.
- Final owner also exports `CORRESPONDENCES: frozenset[str]` (names usable in `correspondence.kind == "recipe"`), used by the validator.
- Factories are called only by `Stage.mobject`/`Stage.transition` with the lod from `profile.active().lod`, once per (entity, lod), cached on the stage.
- Profile call: `profile = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🎚️profile.🐍️").active()`; needs `.morph in {"swap", "full"}` and `.lod`. The stage imports it lazily at `transition` time.
- Staffage extension point: `Stage.register_binder("staffage", fn)` where `fn(stage, entity_id, *args, **kwargs)` calls `stage.bind(...)`; clip code calls `stage.bind_with("staffage", entity_id, ...)`. The staffage owner registers its binder from its own module; no staffage knowledge lives in the stage.

### `M/🐍️.py` facade

Re-exports `stage`, `load`, `validate`, `parse`, `dump`, `token`, `travel`, `tether`, plus the existing toolkit helpers. Final-morph owner: when you move the bodies to `M/🎞️full/🐍️.py`, replace the definitions in the facade by re-exports from `full` (keep the names); the schema owner only appends the declarative exports at the end of the facade.

## timeline owner - `M/⏱️timeline/🐍️.py` (pure Python + NumPy, no Manim)

All times are scene seconds, all frames are integers on a `Schedule`. Everything is stateless (any frame, any order, no earlier frames needed).

```python
Schedule(fps, origin=0.0)            # .frame(t) nearest frame (half up, noise safe), .time(frame)
Window(start, duration)              # .end ; duration 0 = atomic switch at start
Sample(owner, progress)              # owner in "before" | "source" | "target"; progress linear clamp 0..1

count(window, schedule) -> int                   # ABSOLUTE GRID: ceil((start+T)*fps) - ceil(start*fps), exact Fraction arithmetic (times snapped to 1 microsecond); >= 1 for T > 0; 0 for T == 0. Window.start is the absolute canonical start (exact sum of authored durations), so any split of a clip totals ceil(total*fps) and no time accumulates. Stock Manim's numpy.arange gives one frame too many on float noise (0.28 s at 25 fps)
first(window, schedule) -> int                   # ceil(start*fps): first grid frame at/after start (the atomic frame for T == 0)
emitted(end, done, schedule) -> int              # max(done + 1, ceil(end*fps)): frames emitted after an interval ending at `end`; a sub-frame interval that would get 0 frames borrows one frame and the next interval gets that much fewer (the scene clock uses exactly this recursion)
grid(seconds, schedule) -> int ; exact(seconds) -> Fraction ; rate(fps) -> Fraction
frames(window, schedule) -> (f0, f1) | None      # f1 = f0 + n - 1; None for T == 0
terminal(window, schedule) -> int                # f1, or the atomic frame when T == 0
boundary(window, schedule) -> float              # tb = origin + f1 / fps (t >= tb is target)
owner(window, schedule, frame) -> str            # before | source | target (frame >= f1 is target, n == 1 -> target at f0)
progress(window, schedule, frame) -> float       # clamp((frame - f0) / (f1 - f0)); 1.0 when f1 == f0
sample(window, schedule, frame) -> Sample
at(window, t, schedule=None) -> Sample           # continuous seek; with schedule: t >= tb is target (skipped terminal frame safe)
frame_of_alpha(alpha, run_time, fps, phase=0.0) -> int   # round(alpha*run_time*fps - phase), half up, noise safe; phase = SCENE_CLOCK.phase (frames from the play's authored start to its first grid frame, 0 when aligned)
sample_alpha(alpha, run_time, fps, phase=0.0) -> Sample   # window starts phase frames before the first sampled frame, so the last grid point inside the authored interval is the terminal sample
sample_alpha(alpha, run_time, fps) -> Sample     # inside Animation.interpolate(alpha) (raw alpha, before rate_func); finish() alpha=1 -> target

expand(window, size, lag, limit=None) -> tuple[Window]   # staggered element windows == morph._staggered / LaggedStart timings
stagger_lag(run_time, size, lag, limit=None) -> float    # effective capped lag
span(windows) -> Window                                  # group window: T + (size-1)*lag
element_progress(windows, group_progress) -> tuple[float]  # per element linear progress; p = 1 finishes every element exactly

chain(windows, schedule) -> tuple[Link]          # Link(index, f0, f1, until, .terminal_owned); later window owns frames from its f0
resolve(windows, schedule, frame) -> Resolved|None   # Resolved(index, sample); None before first window
resolve_at(windows, t, schedule=None) -> Resolved|None

ease(name, p) -> float   # name: linear smooth smoothstep smootherstep double-smooth there-and-back there-and-back-with-pause
                         # rush-into rush-from slow-into ease-in ease-out ease-in-out ease-in-quad ease-out-quad
                         # overshoot (= ease-out-back) ease-in-back running-start; '_' accepted; smooth is bit-identical to Manim
```

Rules for callers:
- Preview `PreviewSwap(run_time=T)`: inside `interpolate(alpha)` call `s = sample_alpha(alpha, run_time, fps)`; show source while `s.owner != "target"`, place with `ease(name, s.progress)`, show exact target when `s.owner == "target"`. Never decide identity from the eased value.
- Final `execute`: drive the full morph with `s.progress` (so progress is exactly 1 at f1) instead of Manim's `alpha`; for staggered groups use `element_progress(windows, s.progress)`.
- Manim run_time of a staggered group is `span(expand(...)).duration`, not the member run_time.
- Player/seek: use `at(window, t, Schedule(fps, origin))` with the job's output schedule so seeks match exports.

## preview owner - `M/📍️placement/🐍️.py` (pure NumPy) and `M/🖼️preview/🐍️.py` (Manim)

```python
# placement (2D, axis-aligned scale + translation; rotation is never part of a preview placement)
Bounds(lo, hi, origin=None)                     # frozen; Bounds.of(array (2,2)|(2,3), origin=None); .width .height .center .point(anchor) .degenerate
union(bounds_iterable) -> Bounds
Frame(scale=(1,1), translation=(0,0), parent=None)   # logical transform of an entity to its parent entity (None = scene)
Placement(space="scene", anchor="center", fit="none", uniform=True)   # duck-typed: spec.Placement works
Affine(scale, translation)                      # NamedTuple, maps p -> scale * p + translation (x, y; z untouched)
affine(source_bounds, target_bounds, placement, *, frames=None) -> Affine   # raises PlacementError on invalid/non-finite/inverted bounds, unknown anchor/fit, missing frame
at(affine, eased_p) -> Affine                   # exact identity at 0, exact affine at 1, extrapolates for overshoot
apply(affine, points) -> ndarray ; compose(a, b) ; inverse(a) ; to_scene(space, frames) -> Affine
# preview
PreviewSwap(source: Subject, target: Subject, *, transition: spec.Transition, fps: float, frames=None) -> manim.Animation
```

- Stage owner: `frames` is `{entity_id: placement.Frame}` (only needed for `placement.space == "entity:<id>"`; absent frame = actionable PlacementError). Please also expose `Subject.part_bounds: dict[str, ndarray (2,3)]` frozen at bind (n:n element lanes must not measure LoD geometry; fallback when absent: measured from the part mobject).
- `run_time` is `span(expand(Window(0, T), n_lanes, lag, limit)).duration` (== T unless n:n with lag > 0). n:n lanes come from `correspondence.kind` `key|order|pairs|identity`; `recipe` and incompatible counts use ONE group lane (the whole source group translated/scaled to the target bounds; never calls full-morph recipes).
- The animation's `mobject` is a stage group that replaces the source in `scene.mobjects` (same index) for the window and is replaced by exactly `target.mobject` at the terminal sample; `scene.mobjects` holds exactly the target afterwards; the source is restored to its pre-window pose and updater state.
- Work counter: `PreviewSwap` calls `instrument` hooks only if the profiles owner exposes them (to be wired: `preview.swap`).

## profiles owner - `C/🎚️profile/🐍️.py`, `C/🎚️profile/🗃️cache/🐍️.py`, `M/📊️instrument/🐍️.py` (`C` = `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/`)

Import literally: `importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🎚️profile.🐍️")`.

```python
active() -> Profile            # reads SEMIO_TEACHING_PROFILE (preview | final); missing/unknown -> ProfileError (actionable, no default)
profile(name) -> Profile       # explicit lookup (parent processes, tests); never reads the environment
job() -> Job                   # once per process (cached): Profile + job id (SEMIO_TEACHING_JOB, else generated) + fps/resolution (SEMIO_TEACHING_FPS / SEMIO_TEACHING_RESOLUTION, else profile output) + seed
reset() -> None                # tests only: drops the cached Job
Profile(name, morph, lod, width, height, fps, budgets, antialias, shadows, performance, fingerprint)   # frozen
  .morph  "swap" | "full"      .lod  "full" | "preview" | "poster"  (staffage/asset level; final -> full, resolved centrally)
  .budgets: curve_samples, heatmap_resolution, particles, trails          (ints; final = authored maximum)
  .performance: preview budgets (filled from BASELINE.md later)            .fingerprint: sha256 hex of the profile JSON entry
Job(profile, id, fps, width, height, seed)   # frozen
  .key(*parts) -> str          # sha256 over profile fingerprint + parts: every cache key in a scene process goes through it
fingerprint(name) -> str
cache_key(profile_name, *parts) -> str          # same as Job.key for parent processes
```

Environment passed to every scene process by the render runtime: `SEMIO_TEACHING_PROFILE`, `SEMIO_TEACHING_JOB`, `SEMIO_TEACHING_FPS`, `SEMIO_TEACHING_RESOLUTION` (`W,H`).

Effect budgets: clip code scales its knobs with `profile.active().budgets.<name>` (cap = `min(authored, budget)`); the profile never changes timing, ids, narration or endpoints.

`C/🎚️profile/🗃️cache/🐍️.py`: `BoundedCache(namespace, *, max_entries, max_bytes, sizeof=len)` with `get(key)`, `put(key, value)`, `clear()`, `stats()`; keys are always `Job.key(...)` strings so entries are profile scoped; LRU eviction; counts `cache.create[namespace]`, `cache.hit[namespace]`, `cache.evict[namespace]` through instrument.

### `M/📊️instrument/🐍️.py` (pure Python, no Manim)

```python
count(name: str, n: int = 1) -> None     # dict increment; nothing else happens until snapshot/write
snapshot() -> dict                       # {"job": id, "profile": name, "fingerprint": ..., "counts": {...}}
write(path) -> Path                      # writes work.json (atomic); the render runtime writes it beside the scene output (<scene output>/work.json) at scene end
reset() -> None                          # tests
```

Counter names (contract section 6): `full.prepare`, `full.evaluate`, `preview.swap`, `asset.load[<lod>]`, `cache.create[<namespace>]`, `cache.hit[<namespace>]`. Final-morph owner counts `full.prepare` once per correspondence/plan construction and `full.evaluate` per rendered frame; preview owner counts `preview.swap` per swap; staffage owner counts `asset.load[lod]` per loaded asset. Tests assert preview jobs: `full.prepare == full.evaluate == 0`.

### Render runtime and CLI (changed)

- `render` CLI: `--profile preview|final` is REQUIRED; `-q/--quality` is removed. `--fps` and `--resolution` stay optional explicit overrides and are recorded in settings.
- `compose_videos(..., profile: str, ...)` replaces `quality_flag`; settings dicts use the key `"profile"` (the name) instead of `"quality"`. Every `clip_state` carries the profile fingerprint, so preview provenance can never satisfy a final request.
- Output layout: `<output root>/<profile>/<language>/<audience>/...` (profile is the first path level; finals and previews coexist and never share `s/<n>` slots). Final lesson file names: `<output>_<audience>_<language>_<W>x<H>p<fps>.mp4` unchanged; they sit under `<profile>/.../rendered`.
- Final profile output today: 1920x1080 at 30 fps (README delivered format `--quality h --fps 30`); preview: 854x480 at 15 fps.

## staffage owner - `S/🐍️.py` (`S` = `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🧍️staffage/`), TS `🧍️staffage/🔨️modules/📤️export/🎭️cast/🧊️lod/` (landed)

Section ownership of `S/🐍️.py` (agreed with foundations): foundations owns `SCENE_CLOCK` (now `🧰️interop/🕐️clock`), `playback_frame` and the `_Motion` methods `rate`, `_place`, `sync`, `seek`, `restart`; the staffage owner owns everything else (cache, resolver, provenance, `_Motion.__init__` slot allocation/`_show`, `staffage*`). Both patch narrowly with Edit and re-read first. Preview frame sampling needs nothing from the clock: a level's index `fps` is the effective rate of the sampled frames, so `playback_frame(t, index.fps, ...)` is already a pure function of canonical time.

Levels (`full`, `preview`, `poster`; ids = `profile.lod`): the level of a scene process is `profile.job().profile.lod` at `job().fps`; callers never pass it (`lod_scope(level, output_fps)` is for tests and parent processes, no profile => `ProfileError` "no render profile is active", no default). The exporter CLI requires `--lod` and `--output-fps`.

```python
resolve(cast, *, lod=None, output_fps=None, revision=None, view=None, viewpoint=None, scale=None, pose=None, scenario=None, progress=None) -> ResolvedCast
#   cast = canonical manifest id; the keywords are DECLARED axes (a transition endpoint's view/scale "1:50"/pose/scenario) checked against the manifest entry; ResolveError names the casts of the asset that do match.
ResolvedCast(cast, key, lod: ResolvedLod, folder, descriptor, frame_count, provenance)   # nothing decoded but the small index JSON
#   descriptor: heightM, widthM, boundsM, figureBoundsM, originMm, mmPerMetre, partsM {shape-id: [[x, y], ...]} (world metres, feet origin), clock, maxPaths; identical for every level
staffage_extent(cast) -> (width, height)      # figure bounds in metres; fitted_staffage uses it (no probe mobject)
bind_staffage(stage, entity_id, cast_ref, *, unit=1.0, anchor=ORIGIN, rate=None, color=None, mirror=False, view=None, scale=None, pose=None, scenario=None, revision=1)
#   registered as stage binder "staffage" (stage.bind_with("staffage", "tower.worker", "cooling-internal-gains-walker", unit=0.5)); cast_ref is an id or {"cast": id, "revision": "sha256:..."}.
#   bounds = figure bounds x unit about anchor (LoD-invariant, frozen at bind); anchors = {"origin": anchor (feet), "part:<shape-id>": centroid, ...}; provides = declared axes of the cast;
#   the value is a factory(lod) the stage calls with profile.lod; counters: instrument asset.load[<lod>] once per loaded cast folder.
```
Cache layout: `<cache>/staffage/<castKey>/<cast>/<lod>-<lodRev8>/{🔣️.json,NNNN.svg}`; `castKey` hashes the cast dependency closure only (FD-006).

## playback owner - `C/🎞️playback/🐍️.py` (pure Python + PyAV for decoding) and presentation frame purity

```python
locate(root, profile, language, audience, *parts) -> Path   # <root>/<profile>/<language>/<audience>/...; unknown profile -> ProfileError; no fallback
open_media(artifact, profile) -> Media                      # export.require_record_profile: no provenance / other profile / stale fingerprint / modified file -> ProfileMismatch
Media(path, profile, fingerprint, kind, language, audience, width, height, fps, poster, thumbnail)   # poster/thumbnail Paths from record["stills"] (None if absent)
Media.label(language) -> str          # "Preview"/"Vorschau" or "Final": every player MUST show it; language is explicit (en|de), no default
frame_index(pts, time_base, fps) -> int                     # via timeline.Schedule.frame
frames(path) -> {index: rgb ndarray}  # forward pass          frame_at(path, index)  # fresh seek, no earlier state
sample(path, indices, order="forward"|"reverse"|"random", seed=0) -> {index: ndarray}
owner_at(index, *, start, count, fps) -> "before"|"source"|"target"   # shared evaluator
```

- A video-element/native player (none exists today for teaching clips) must: select media only through `open_media(artifact, <explicit profile>)`, show `Media.label`, use `Media.poster`, and map a seek time `t` to the output frame with `timeline.Schedule(fps).frame(t)` (identity from `timeline.at(window, t, Schedule)`), never from decoder callbacks.
- Presentation (offline browser renderer): `settings` now needs `"profile": "preview"|"final"` (output size and fps from the profile; `-ql` keys are gone). `presentation.frame_count(duration, fps, *, minimum=1)` is the ONLY frame-count rule (shared `timeline.count`; transitions use `minimum=3`); `collection.body_frames`/`chapter_clock` and `_transition`/`_render_chapter` call it.
- Frame purity: every browser frame starts from `reset()` (unmount, release mouse, park the pointer at 1610,910), mounts its own declarative state with `reset=true`, and uses an absolute `transition_elapsed` (`SETTLED_MS`=1000, `SETTLED_CANONICAL_MS`=2000 or the explicit request value) that no longer depends on the previous request; animation `currentTime` is assigned, not accumulated. A frame alone equals the same frame in any batch order (packet-level test is green; pixel-level test needs the browser, see FD-playback-1).

### preview owner addendum (implemented)

- `PreviewSwap` requires frozen `Subject.part_bounds` for n:n key/order/pairs lanes (PlacementError otherwise; never measured from LoD geometry).
- Manim rejects `run_time <= 0`: for `timing.duration == 0` the stage must call `PreviewSwap(...).swap_now(scene)` (atomic, no frame) instead of `scene.play`.
- `PreviewSwap.release()` is idempotent (called from `finish`, `clean_up_from_scene`, and on any exception in `begin`/`interpolate`); it restores source pose, children and updater flags. Lifecycle owner may call it on cancel.
- Tests: `🧪️tests/📍️placement/🐍️.py` (+`🧫️cases/🔣️.json`), `🧪️tests/🖼️preview/🐍️.py` (+`🧫️cases/🔣️.json`).

## final-morph owner - `M/🎞️full/🐍️.py`, `M/🪙️token/🐍️.py`

- `M/🪙️token/🐍️.py` holds the shared primitives `token`, `travel`, `tether`, `KINDS`, `INK/MUTED/DATA/GOOD/WARN/DARK`; the facade re-exports them (and everything of `🎞️full`: `Morph PartMorph Swarm Flight Dim Trip Flock Move Dissolve silhouette proxy capsule_pairs morph scatter gather split unfold retext flight execute CORRESPONDENCES RECIPES`).
- `execute(source, target, *, transition, fps)` as agreed; reads `transition.recipe/.copy/.correspondence/.timing/.final`. Recipes: `parts hull scatter gather split unfold retext flight move replace vanish appear` plus `fade` (FadeTransform look: cross-fade of copies; NOT yet in the schema enum - schema owner: add `fade` or tell me to map it into `replace`). `copy: true` is honoured by `move`, `replace`, `fade` (source stays); other recipes raise an actionable ValueError. `vanish` = `target is None`, `appear` = `source is None`.
- Correspondence kinds `identity key order pairs recipe` -> couples `(source part, target part, part pairs)`; `recipe` names come from `CORRESPONDENCES` (`capsule-item`: key (else order) couples plus shell-to-ring and window-to-dot pairs). Part names resolve through `Subject.parts`. Source/target leftovers (unmatched parts) vanish into / grow out of their nearest partner for scatter/unfold.
- Timing: element animations run the full `timing.duration`; staggered groups use `timeline.expand/stagger_lag/span` so `run_time == span(...).duration` (== duration when nothing is staggered; identical to what PreviewSwap reports). Frames are driven by `timeline.sample_alpha(...).progress`, so frame f1 shows progress 1 (exact bound target) and `finish()` too. Easing is `timeline.ease(name, p)`.
- The bound source/target are never mutated while evaluating (shadow copies); `clean_up_from_scene` removes the source (unless `copy`) and adds exactly `target.mobject`.
- Instrument: `full.prepare` counted in `silhouette`, `proxy`, `_plan`, `_couples`, `Trip`, `Move`, `Dissolve`, `retext`; `full.evaluate` per `interpolate` call of the returned animation (includes the begin/finish calls).
- Final never flies rectangle proxies: groups above 200 shapes morph the real silhouette of every top-level part (1..200 tops), else the silhouette of the whole group; more than 200 tops raises "too large" (declare `pairs`).
- n:n/n:1/1:n sub-morphs are built lazily when the group is set up (`Lazy`), not at call time.

### preview owner addendum 2

- `PreviewSwap` accepts `source=None` (appear), `target=None` (vanish) and `transition.copy`: nothing moves; the source stays (kept afterwards for copy) and the target appears at the terminal sample. `begin` counts `preview.swap`.
- `Stage.cut` in the preview profile calls `PreviewSwap(...).swap_now(scene)`; in final it keeps the atomic remove/add (no full-morph counters).
- Staffage owner: `bind_staffage` must pass `part_bounds={name: ((x0,y0,z0),(x1,y1,z1))}` from the LoD-invariant descriptor for every part used by an n:n key/order/pairs transition (whole-entity lanes need only `bounds`); factory-bound parts without it raise PlacementError.

## exports owner - `C/📤️export/` (`C` = `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/`)

Import literally: `importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.📤️export.🐍️")`. Schema: `C/📤️export/🧬️schema/🔣️.schema.json` (provenance + `posterPolicy` + `stills`), poster default policy `C/📤️export/🔣️.json`.

```python
# C/📤️export/🐍️.py (core; depends only on 🎚️profile)
require_profile(movie, profile) / require_profiles(movies, profile) / require_state_profile(state, profile)   # reads <movie>.render.json; raises ProfileMismatch for another profile or a stale fingerprint
collect_work(dirs, profile) -> {"jobs", "counts", "sources"}      # sums <dir>/work.json written by the instrument; foreign profile/fingerprint -> ProfileMismatch, missing -> WorkMismatch
verify_work(profile, work, *, transitions)                        # preview: full.prepare == full.evaluate == 0; final with declared transitions: both > 0 (no silent preview fallback)
declared_transitions(script) -> int                               # len(<clip folder>/🔀️transitions/🔣️.json["transitions"])
write_work(movie, work, profile) / read_assembled_work(movie, profile)   # <movie>.work.json retained next to an assembled chapter, so reuse never needs scene media
publish_export(kind, movie, *, settings, movies, scripts, inputs, stills=None, windows=None)  # writes <movie>.export.json (+ poster/thumbnail); kind chapter|master|course|delivery
export_record / publish_record / read_record / require_record_profile(movie, profile) / poster_policy(video_id)
# C/📤️export/🖼️stills/🐍️.py
poster_frame(policy, *, frames, fps, windows=None) -> int         # policy: frame | time | fraction | transition(id) via ⏱️timeline.terminal; pure
extract(video, index, out, size=None) / make_stills(video, folder, name, *, profile, policy, windows, size) / stills_current(...)
read_transitions(media_dir) -> {id: (start, duration)}            # optional <scene output>/transitions.json, see request below
# C/📤️export/🎞️course/🐍️.py
titles / current_masters(language, audience, profile) / join_course(language, audience, profile, ...) / deliver(language, audience, profile)
```

- Runtime hooks (narrow patches in `C/⚙️runtime/🐍️.py` and `C/🪟️presentation/🐍️.py`): `concat_videos(..., profile=)` refuses clips whose certificate profile differs; `presentation.render_presentation` refuses a content certificate of another profile; `compose_videos` writes `<chapter>.work.json` when it assembles a chapter and publishes `<movie>.export.json` for chapters and masters (masters also get `posters/<name>.poster.png|thumb.png`).
- Provenance holds: profile name + fingerprint + morph/lod, effective render (width, height, fps, antialias, shadows, budgets, resolution/fps overrides, seed), summed work counters with per-scene sources, input names with their profile, stills (profile, policy, frame index, sha256).
- Poster frame = integer index from the output schedule, never "last decoded frame". Default `fraction 0.5` (floor(0.5 * frames)); a lesson can declare `{"kind":"transition","id":...}` in `📤️export/🔣️.json`; terminal frame = `⚡timeline.terminal(Window(start, duration), Schedule(fps))`.
- Request to the stage owner (optional, no blocker): when `stage.transition(...)`/`stage.cut(...)` runs, append `{"id", "start", "duration"}` (scene seconds, `t0` of the play) to `<dirname of SEMIO_TEACHING_WORK>/transitions.json` as `{"transitions": [...]}`; exports reads it with `stills.read_transitions` to resolve transition-terminal posters on the real scene clock.
- Request to the profiles owner: `SEMIO_TEACHING_WORK` is already used by `📊️instrument`; no change needed. `render_segment` still writes the empty `work.json` fallback; exports treats it as a real record with zero counts (a final job with declared transitions and zero full counts fails).


## scene clock owner - `🧰️interop/🕐️clock/🐍️.py` (imports Manim; installs itself on import)

- Authored scene time `t` is an exact `Fraction` sum of authored durations; output frames sit on the absolute grid: a play/wait spanning `[t0, t0+T]` emits `timeline.emitted(t0+T, done) - done` frames (`ceil((t0+T)F) - ceil(t0 F)` plus the sub-frame borrow rule), identically for rendered, frozen-wait, skipped, cached and trimmed plays. `Scene.get_time_progression`, `CairoRenderer.freeze_current_frame`, `CairoRenderer.play`, `Scene.update_to_time`, `Scene.update_mobjects` are patched once (`Scene._canonical_clock`).
- `SCENE_CLOCK.now` (seconds of the frame being drawn, `frame/fps`), `.origin` (first frame of the running play, `None` between plays), `.phase` (frames from the authored start to that first frame; Manim receives `t_k = (done+k)/F - t0`, so alpha follows the true absolute grid point). `renderer.time` is overwritten with `now`.
- `frame_count(duration, fps, start=0.0, done=None)`, `scene_seconds(scene)`, `stopwatch(rate)` and `scene_timer(rate)` (replace `increment_value(dt)` accumulators; value = `rate * (now - play start)`).
- Morph consumers (`PreviewSwap._interpolate`, full-morph `_counted`) read `SCENE_CLOCK.phase` and pass it to `frame_of_alpha`/`sample_alpha` so the terminal sample is the last grid frame of the authored interval.
- Open-ended `wait_until` / `stop_condition` waits advance authored time to `done/fps` after the frames actually drawn.

### exports owner update (follow-ups)

- Implemented in `M/🎭️stage/🐍️.py` (`Stage._report`, called by `transition` and `cut`): when `SEMIO_TEACHING_WORK` is set, `<dirname>/transitions.json` = `{"transitions": [{"id", "start", "duration", "f0", "f1"}]}` on the scene clock (`start` = renderer time at the call, `f0`/`f1` from `⏱️timeline.frames/terminal`; a cut has `f0 == f1`). `stills.read_transitions` reads it.
- Lesson poster policy is declared data in `C/📤️export/🔣️.json` -> `posters.videos.<video id>` = `{"kind": "transition", "id": "<transition id>", "scene": "<Scene class>"}`; `publish_export` then takes the poster from that scene's retained clip at `timeline.terminal` on the scene's own clock (raw scene content, without the lesson window). Lessons without an entry still use `fraction 0.5`; clip owners should add an entry per lesson once their transition ids exist.

## authoring owner - `M/✏️authoring/🐍️.py` (pure Python, no Manim)

- `bun 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts morph --language en|de <list|show|validate|format|set|preview-plan|request> ...` (language required; exit 0 ok, 1 invalid/refused, 2 usage).
- Canonical file text is `spec.dumps(document_or_raw)` (added to `M/📋️spec/🐍️.py`): defaults explicit, raw `$schema` kept first, 2-space JSON, `ensure_ascii=False`, trailing newline. `validate` reports `[format]` for any file that differs; `format <path...>` rewrites (paths are required; never run it repository-wide, it rewrites other owners' files).
- `validate` also checks every curriculum scene statically (`scene_plays`): ids passed to `.transition/.cut` (literal or f-string pattern) must be declared in a transitions document of the script's folder or an ancestor; declared ids never played produce a warning. Runtime cross-check: `validate --played <scene output>/transitions.json ...` (the record the stage writes beside work.json) fails on unknown ids or durations that differ from the document.
- `set <id> <json-pointer> <value>` validates the whole result (schema, semantics, duplicate ids, scene references) and writes atomically; `request <ids> --profile preview|final --purpose review|publication --output <file>` persists an export request with an explicit profile (`publication` requires `final`); schema `M/✏️authoring/🧬️schema/🔣️.schema.json`.
- Folder names `🔀️transitions` are reserved for transition documents (discovery matches `🔀️transitions/🔣️.json`); never use that folder name for anything else.

### profiles owner - effect budgets are real (shared visuals)

- `Profile.budgets.<name>` is an int cap (preview) or `null` (final = the authored value, so final output is identical to the authored code path). `Profile.cap(name, authored)` applies it.
- `🎨️visuals/🐍️.py: effect_density(budget, authored, units)` routes the cap per unit and never below one per unit. Consumers today: `animate_flows` and `flow_animation` (`particles`, units = number of paths, preview cap 24 particles in total; every path keeps at least one particle, speed/cycles/colours/direction unchanged) and `ripples` (`trails`, units = emitter centers, cap 12 arcs). It calls `profile.active()`, so any process using these must have a profile (the render runtime and `script.ts test` set one).
- Proof script `$T/bench/effect_budgets.py` (320x180, frame hashes over 29 alphas): final hashes equal the unrouted authored path for flows and ripples; preview draws 24 of 40 flow particles (1.7x faster per frame, median 72.4 ms to 41.9 ms) and 10 of 30 ripple arcs (3.1x, 14.4 ms to 4.7 ms) on this host while other jobs were running. The preview numbers come from that unit measurement; `$T/BASELINE.md` did not exist yet, revisit when it does.
- Not consumed (no shared knob exists, curve and heat-field sampling are written in clip code): `curveSamples`, `heatmapResolution`. `always_redraw` has no sampling-rate knob in Manim (it runs every frame).
- Speech tooling: `runtime.profile_arguments(profile)`, `runtime.profile_environment(profile, scene)`, `runtime.record_timing_profile(manifest)` (writes `<manifest>.profile.json`). All per-clip `🔊️speech` trace commands render with `preview`; `build_section_video` (delivered `Full_*` masters) renders with `final`.

## schema owner addendum (batch E1, 2026-11-11)

- Schema recipes now include `fade` (Manim FadeTransform; relation one-to-one | many-to-many) next to `replace`; the transition flag `copy` replaces any recipe-level "copy" (full owner: `RECIPES`/`_BUILDERS` entries named `copy` are inert).
- `stage.transition(id, source=obj, target=obj)` / `stage.cut(id, source=, target=)` bind the declared entities inline (clip code never invents entity ids; ids live in the JSON). Rebinding an entity to another object replaces the binding (bounds re-frozen); the same object keeps it.
- `full` additions made for E1 (final-morph owner: keep): `_b_move` and `_b_fade` move/fade every couple of a many-to-many transition on its own, staggered by `timing.lag`, when `relation == many-to-many` (fade always, move/replace when `lag > 0`); `split` honours `copy` (source stays for every branch).
- Lag semantics for ported `LaggedStart(..., lag_ratio=r)` with `play(run_time=T)`: `timing.duration = T` and `timing.lag = r * T`; the play keeps its `run_time` (Manim rescales the group to T, exactly like today).
- Validator: for relation one-to-one with recipe `move|replace|fade|retext` the correspondence kind is not part-checked (the whole entity moves); key/order part checks apply to part-wise relations.
- Same-identity pose changes (window sash opening, opacity dimming) are not transitions: `🎨️visuals.pose_animation(mobject, target)` interpolates in place (UpdateFromAlphaFunc) and keeps identity.
- Optimization `ClipScene.present` creates `self.morphs = stage(...)` from `<clip folder>/🔀️transitions/🔣️.json` when it exists; `source_files()` fingerprints the documents. Energy clips create `stage` (heating fundamentals: `morphs`, since a local dict is called `stage`) at the top of `construct`.

## clips-b (Excel anatomy, program, energy; BIM information) - requests and notes

- REQUEST final-morph owner: correspondence recipe `item-capsules` in `M/🎞️full/🐍️.py` `_CORRESPONDENCE`: the list form of `item-capsule` (item tokens to capsule drawings, many-to-many, couples by key else order, each with `[(t, s) for s, t in capsule_pairs(d, s_item)]` part pairs, i.e. `_capsule_item` mirrored). Used by `📊️excel/🎬️clip/📋️program/🔀️transitions` `excel.program.countifs.seeds-to-capsules`; until it exists that single transition fails validation in the Excel program clip.
- Ghosts: Excel anatomy and energy keep their faint source copies as clip-level `FadeIn` copies (opacity 0.22 / 0.35) next to the declared transition, because a `final.ghost` copy is created inside the full executor, cannot be removed by the clip and does not exist in the preview. `final.ghost` stays 0 in these documents.
- `📊️excel/🎬️clip/🎨️visuals/📗️workbook` `Glide` (selection tint / active border slide, retexts) no longer uses a Manim `Transform`: parts slide through the widget-local `Slide` (same points/colour interpolation and smooth easing, no scene membership). Same-identity pose change like `pose_animation`.
- Excel `source_files()` and BIM `source_files()` now fingerprint every `🔀️transitions/🔣️.json` under the clip root.
- `📊️excel/🎬️clip/📍️references` is not touched by this batch (moved to the preview owner).
- Documents are generated from a table: `$T/excel-b/gen-transitions.py` (rerun is byte-stable; edit the table, not the JSON, or edit the JSON and keep the table in sync).

### final-morph owner, follow-up (GH capsule/casting migration)

- Correspondence recipes (kind `recipe`, `name`): `auto` (whole source to whole target, automatic part matching by area then position: the declarative `morph(a, b)`), `fan-in` (every source part to the whole target, for `gather`), `fan-out` (the whole source to every target part, for `split`), `capsule-item` (ONE capsule drawing to ONE item token: shell to ring, window to dot), `item-capsule` (the reverse), `capsule-items` (lists by shared key else order, `capsule_pairs` per couple; replaces `scatter(..., pairs_fn=capsule_pairs)`).
- `copy: true` is honoured by `parts`, `hull`, `move`, `replace`, `fade` (source stays untouched; a clip that wants a ghost dims the source itself with its own animation, in both profiles).
- Silhouettes moved to `M/🫥️silhouette/🐍️.py` (`silhouette`, hull helpers; Manim + NumPy); `🎞️full` re-exports them. Clips may import it for a stand-in outline; recipes, `proxy`, `Morph`, `Swarm` stay internal.
- `execute` reads `animation.run_time` at every frame, so a clip's `play(..., run_time=X)` override keeps frame k mapped to progress k/(n-1) of the real run.
- Staggered groups (scatter/gather/split/fade n:n): `timing.duration` is the element duration; the group spans `timeline.span`. A clip's `play(run_time=X)` rescales the group exactly like the legacy `_staggered` helpers did (the document numbers are the legacy `run_time`, `lag`, `limit`).
- Transitions that follow a `travel(...)` of their source must be built when their clause part runs (`lambda: (hop(...),)`), because entity bounds and flight starts are captured when `stage.transition` is called (snapshot sampling).
