# Shared Contract (coordinator-owned, v1)

Binding for every worker. Changes go through the coordinator. Acceptance: `teaching-morph-preview-launch-kit/acceptance.md`.

## 1. Module tree (Python, Manim)

Root `🎓️teaching/🏛️architecture/🔀️morph/` (`M` below). Import literally with `importlib.import_module`.

| Path | Owner | Content |
| --- | --- | --- |
| `M/🧬️schema/🔣️.schema.json` | schema | JSON Schema (draft 2020-12) of a transition document |
| `M/📋️spec/🐍️.py` | schema | frozen dataclasses + `load(path)`, `parse(dict)`, `validate(document, stage=None) -> list[Problem]`, `Problem(path, code, message)` with actionable messages |
| `M/🎭️stage/🐍️.py` | schema | per-scene entity registry and the single `play` entry point |
| `M/⏱️timeline/🐍️.py` | timeline | pure (no Manim): windows, schedules, frame ownership, progress |
| `M/📍️placement/🐍️.py` | preview | pure NumPy: logical bounds, anchors, coordinate spaces, fit policy, affine interpolation |
| `M/🖼️preview/🐍️.py` | preview | `PreviewSwap` Manim animation |
| `M/🎞️full/🐍️.py` | final-morph | full morph recipes (today's toolkit body moves here) + `execute(...)` |
| `M/📊️instrument/🐍️.py` | profiles | per-job work counters |
| `M/🐍️.py` | schema | public facade: re-exports only (`stage`, `token`, `travel`, `tether`, `load`, `validate`, ...) |
| `M/🧪️tests/...` | each owner | unit tests + language-agnostic JSON cases (`🧫️cases/*.json`) |

Render policy lives in shared render infrastructure: `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🎚️profile/` (`🔣️.json` profiles, `🔣️.schema.json`, `🐍️.py`). Owner: profiles.

Clip transition documents: one per clip folder, `<clip folder>/🔀️transitions/🔣️.json`, validated against `M/🧬️schema/🔣️.schema.json`.

## 2. Transition document (schema owner may refine names, not semantics)

```json
{
  "$schema": "../../../../🔀️morph/🧬️schema/🔣️.schema.json",
  "version": 1,
  "transitions": [
    {
      "id": "trees.birth.capsules-to-leaves",
      "intent": "abstraction",
      "source": { "entity": "tower.capsules", "revision": 1, "sampling": "snapshot", "view": "front", "scale": "1:200", "pose": null, "scenario": null },
      "target": { "entity": "diagram.leaves", "revision": 1, "sampling": "snapshot" },
      "relation": "many-to-many",
      "correspondence": { "kind": "key" },
      "recipe": "parts",
      "timing": { "duration": 1.2, "easing": "smooth", "lag": 0.012, "limit": 2.0 },
      "placement": { "space": "scene", "anchor": "center", "fit": "contain", "uniform": true },
      "final": { "pathArc": 0.3, "dip": 0.0, "ghost": 0.0 }
    }
  ]
}
```

- `intent`: `abstraction | concretization | continuation | transformation`.
- `relation`: `one-to-one | one-to-many | many-to-one | many-to-many`.
- `correspondence.kind`: `identity` (same object), `key` (bound mappings share keys), `order` (lists in order), `pairs` (explicit `[[sourcePart, targetPart], ...]` part names), `recipe` (named correspondence function registered in `🎞️full`, e.g. `capsule-item`). Incompatible topologies MUST use `pairs` or `recipe`.
- `recipe` (final execution family): `parts`, `hull`, `scatter`, `gather`, `split`, `unfold`, `retext`, `flight`, `move` (continuation: same identity moves/scales/restyles). The validator rejects unsupported recipe/relation combinations with actionable errors.
- `sampling`: `live` (entity keeps its own animation/updaters during the transition) or `snapshot` (frozen state captured once at the transition start, deterministically).
- `timing.duration` is canonical seconds. `lag`/`limit` stagger elements of n:n, 1:n, n:1 transitions; the expansion into per-element windows is deterministic (timeline owner).
- `placement.space`: `scene` (frame coordinates) or `entity:<id>` (nested; resolved through the parent's logical transform). `anchor`: `center | bottom | top | left | right | origin` (origin = the entity's declared logical origin, e.g. staffage foot contact). `fit`: `none | contain | width | height`, `uniform` true keeps aspect.
- Durations, ids and endpoints are identical for every profile.

## 3. Stage API (clip code is declarative)

```python
stage = _morph.stage(self, "<clip folder>/🔀️transitions/🔣️.json")   # loads + validates once per scene
stage.bind("tower.capsules", {key: mobject_or_factory, ...}, bounds=None, anchors=None)
stage.bind("diagram.leaves", factory)                                   # factory(lod) -> Mobject, called lazily
self.play(stage.transition("trees.birth.capsules-to-leaves"))           # returns one Manim Animation
```

- `bind` accepts a Mobject, a mapping key->Mobject, a list, or a factory `callable(lod)`; logical bounds/anchors default to the bound representation's bounds at bind time and are then frozen (never re-derived from reduced-detail or screen-space geometry). Staffage entities bind a canonical staffage reference instead (staffage owner provides `bind_staffage`).
- `stage.transition(id)` reads the active profile (section 5) and returns a `PreviewSwap` (preview) or the `🎞️full.execute` animation (final). No other code path may branch on the profile for morph execution.
- Clip code must not construct morph geometry, recipes, Transforms or crossfades itself.

## 4. Timeline semantics (timeline owner implements, everyone relies on)

- Schedule: output fps `F`, transition start `t0` (scene time), duration `T`. Manim samples `t0 + k/F` for `k = 0..n-1` inside `play` (half-open). The shared evaluator converts once: inclusive frame range `[f0, f1]` with `f1 = f0 + n - 1`. For `n == 1` the sole frame is the target. For `T == 0` the switch is atomic at `t0` with no frames.
- Identity: `frame < f1` source, `frame >= f1` target. Progress for translation/scale: `p = clamp((frame - f0) / (f1 - f0), 0, 1)`; easing maps `p` only for placement; overshoot or early `1.0` never switches identity.
- Continuous time (seek/scrub without a frame schedule): boundary time `tb = t0 + (n - 1)/F` for the job's schedule; `t >= tb` is target.
- Frame index inside a Manim animation is derived from `alpha` deterministically: `k = round(alpha * run_time * F)`; never from callback counts or accumulated dt.
- Final execution uses the same `[f0, f1]`: the full morph's progress is remapped so that `f1` shows the exact authored target (progress 1), keeping the same duration. Both profiles therefore end on the target at `f1`.
- Adjacent transitions: the later transition owns the shared timestamp from its `f0`; a target handed to the next transition is the same bound object.

## 5. Render profiles (profiles owner)

- `🎚️profile/🔣️.json` defines at least `preview` and `final`: `morph` (`swap` | `full`), `lod` (staffage/asset level), output (`width`, `height`, `fps`), effect budgets (curve samples, heatmap resolution, particle/trail counts, antialias, shadows), and preview performance budgets.
- The profile is always explicit: render CLI `--profile preview|final` is required; scene processes receive `SEMIO_TEACHING_PROFILE`; a direct scene render without it fails with an actionable error (no default, no inheritance from an editor toggle). The old `-q/--quality` flag is replaced (no compatibility layer).
- A `Job` (profile, job id, fps, resolution, seed) is created once per scene process; caches and output provenance include the profile fingerprint; preview output can never satisfy a final request.
- Profiles never change timing, narration, ids or endpoints.

## 6. Instrumentation

`📊️instrument` counts per job id and profile: `full.prepare` (correspondence/plan construction), `full.evaluate` (full-morph frames), `preview.swap`, `asset.load[lod]`, cache create/hit per cache. Written to `<output>/…/work.json` next to the scene output. Tests assert preview jobs have `full.prepare == full.evaluate == 0` and final jobs `> 0`.

## 7. Ownership map (logical)

schema: spec, schema, stage, facade, validation. timeline: timeline + boundary cases. preview: placement + PreviewSwap. final-morph: full recipes, correspondences, boundary remap. profiles: profile layer, Job, render CLI wiring, cache keys, instrumentation. staffage: staffage resolver + LoD variants. playback: presentation/browser player, seeking/scrub/reverse/loop, posters. authoring: morph authoring CLI/controls on the same JSON. clips-a/clips-b: clip migrations (batches assigned from INVENTORY.md). exports: render/export jobs, course preview joiner, thumbnails/posters, generated media. lifecycle: cancellation, ownership, disposal, leak cycles. validation: queue, baselines, after-measurements, integrated runs. foundations: runtime/env defects.
