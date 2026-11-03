# Viewport dynamics: field ramps, parameter tracks, mode playback

Status: code complete, NOT compiled (the integrator builds app-surface and structural-scene; request filed in `📌️important/requests.md`). Python oracle goldens generated; twins regenerated (`wp01-gen-twins.py --check` 99 types green, `wp02-gen-twins.py --check` green).

SURFACE = `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface`, VP = `SURFACE/🪟️visual-windows/🧊️viewport3d`.

## Framework findings (World3d)
- One `World3dScalarField` per scene, bound to one `mesh_id`, with legend `{title, unit, ticks}` and no per-instance tint. The framework has NO animation or time parameter: playback is host-driven, i.e. the host dispatches `PlaybackCommand::Tick(dt)` (or a pointer scrub) and re-lowers; `instances_delta` already ships only changed instances. Mode frames therefore move instance poses (beams) or re-bake meshes (plates, shells) per frame.
- Consequence: colours of every entity with `ColourChannel::Fields` are baked per vertex into its own mesh (mesh id = part key + entity + hash of the baked colours, so equal frames deduplicate); the single scalar field/legend follows the focused entity and uses the same value domain, so the legend matches the baked colours.

## 1. Per-vertex field ramps (`ColourChannel::Fields`)
- VP `🌈️fields`: `FieldSelection` (toolbar choice or track keyframes), `resolve` (entity falls back to its own `active` when it lacks the selected field), `value_at` for `Vertex` (per node when counts match, else resampled by index fraction or station), `Station` (member stations along the prism z), `Instance` (pattern instance index = part index), `domain` (min/max over all entities, over ALL keyframe fields of a selected track so the legend does not rescale while scrubbing), `title` (EN/DE for known ids, humanised otherwise), `available` (field menu), `grid_node` (maps flat-shaded grid mesh vertices back to row-major nodes).
- `🎨️colour`: `field_rgb` (ramp over `lo..hi`, quantised 1/32), `styled` (dim/stale/error tint extracted), `entity_rgb` no longer averages fields.
- `🧱️parts`: `Part.grid` for grid meshes (shell, terrain, slip); member parts carry stations also for `Fields::Station` and for deformed shapes.
- `🎞️lowering`: `lower_dynamic`; legend (title, presentation unit via `overlays::presentation`, range scaled to kN, MPa, ...) for the focused field entity. Focus order: selected utilisation member, selected field entity, any field entity when the toolbar field is set or no utilisation exists, else worst utilisation member (unchanged).
- Config: `Viewport3dWindowConfig.active_field: Option<String>` (persisted local-only), action `viewportSetActiveField {field}`; twins (ts, proto, graphql) updated.

## 2. Parameter tracks
- Schema (structural-scene `🎨️appearance`): `ParameterTrack { id, kind: TrackKind{Time,Age,Mode,Phase,Combination}, label, unit, min, max, step, looped, keyframes: Vec<Keyframe{at, caption, field?, layer?}> }`, with `Validate` (ordered range, positive step, ascending keyframes inside the range, field-or-layer per keyframe, keyframes required except for `Phase`) and test `parameter_tracks_validate_and_round_trip`. Twins regenerated.
- Semantics: `Time`/`Age` blend the two neighbouring keyframe fields linearly (fire temperature, char depth, creep, shrinkage); `Mode`/`Combination` snap to the nearest keyframe; layers of a track are shown only while their keyframe is current (one eigenmode or EN 1990 combination layer at a time); `Phase` drives the oscillation phase `sin(2 pi p)`.
- NOT done (needs the schema owner and a mechanical edit of 42 `StructuralScene { .. }` literals): a `tracks: Vec<ParameterTrack>` field on `StructuralScene`. Until then the producer returns tracks next to the scene and passes them via `Dynamics { tracks, playback }`.
- VP `⏯️playback`: ephemeral local-only `PlaybackState { track, value, playing, clock, reduced_motion }`, event reducer `reduce(state, tracks, speed, PlaybackCommand::{Select, Seek, Play, Pause, Toggle, Step, Rewind, Tick})` (seek snaps to the step; non-looped tracks stop at the end, looped and phase tracks wrap), `command_of(action id, text, number)`, `sample`, `frame`, `readout` (EN/DE, time as s/min/h/d/a).
- Toolbar actions (window definition in `🪟️manifest-and-media`): `viewportSetActiveField`, `viewportSelectTrack`, `viewportSeek`, `viewportTogglePlay`, `viewportStep`, `viewportSetPlaybackSpeed` (speed lives in config `playback_speed`). The scrubber is the host rendering of `viewportSeek` with the track's `min/max/step`; `LoweredViewport.readout` is the label and is appended to the accessible description.
- Reduced motion (a11y): `PlaybackState::with_reduced_motion(!animations_allowed(prefs))` makes `Play`/`Toggle`/`Tick` no-ops (no autoplay, no clock) and draws harmonic overlays at the manual `deformation_phase`; explicit seeking and stepping still work (user-initiated, discrete).

## 3. Motion playback of mode shapes
- `Motion` is now authoritative for every overlay: `Static` factor 1, `Harmonic` = frame phase (`sin(2 pi 0.5 Hz t)` while playing at `playback_speed`, else the manual `deformation_phase`), `Ramp` = `min(1, t / 2 s)`. Displacement = `scale * deformation_scale (amplitude control, existing action) * factor`. The visual frequency is a fixed 0.5 Hz so modes of 1 Hz to 50 Hz stay readable; the real frequency is in the label.
- Beam modes: unchanged deformed polyline with ghost. Plate modes (`surface` samples, plane coordinates in metres): ghost prism plus a smooth two-sided surface grid (24 cells per side, 6 minimum on coarse devices, clipped to the outline by cell centres, holes excluded) displaced by inverse-distance weighting (power 2, exact at samples), drawn at mid thickness. Shell modes: `at` are NORMALISED grid coordinates `(column/(columns-1), row/(rows-1))` in `0..1` (documented here; the scene schema should repeat it on `SurfaceSample`), ghost grid plus displaced grid. `delta` is a world-space vector like the beam samples.
- Labels (`🏷️overlays`): `Mode n · f = 2.5 Hz · αcr = 3.42 · ×10` and `Eigenform n · f = 2,5 Hz · αcr = 3,42 · ×10` for every `Mode` overlay or any overlay with `load_factor`, at all densities except Off.

## 4. Tests
- Unit (`🧪️tests/🔬️unit`): phase/ramp frames, track blending, discrete snapping and layer keyframes, transport reducer incl. reduced motion, mode frame depth at fixed clocks (0, 0.5, 1, 1.5 s) vs golden, amplitude, reduced-motion freeze, ramp/static motion, IDW vs golden, grid node order vs golden, vertex blend vs golden, station-field bake with legend (kNm), toolbar field switch (m, MPa, DE titles), scrubbing with stable domain and readout, instance tint, plate and shell mode meshes, EN/DE mode label, field selection, action ids, config round trip with the new fields.
- Language-agnostic: 6 new scenarios in `🧪️tests/🥒️.feature` (tag `@oracle-python-dynamics`).
- Oracle/golden: `🔮️oracles/🎬️dynamics/🐍️.py` (stdlib closed forms; numpy `sin`/`interp` cross-check runs when importable, numpy absent on this host, shapely likewise) -> `🧪️tests/🧫️fixtures/🎬️dynamics.json`.

## Files
VP: new `⏯️playback/🦀️.rs`, `🌈️fields/🦀️.rs`, `🔮️oracles/🎬️dynamics/🐍️.py`, `🧪️tests/🧫️fixtures/🎬️dynamics.json`; edited `🦀️.rs` (mounts), `🎞️lowering`, `🧱️parts` (PartContext now carries `frame` instead of `deformation_phase`), `🎨️colour`, `🏷️overlays` (`presentation`, `mode_text`), `🎚️config` (+ three twins), `🧪️tests`. SURFACE `🪟️manifest-and-media/🦀️.rs` (six toolbar actions). Structural-scene `🧬️schema/🎨️appearance` (+ twins), `🧪️tests/🔬️unit`.

## Adoption by the artifacts (one line each, not done: 20 files owned by the visual agents)
Windows call `lower(..)` (static view, unchanged signature). To get dynamics replace it with `lower_dynamic(&scene, &inputs, &Dynamics { tracks: &tracks, playback }, &tess)`, keep a `PlaybackState` per window (reduced by `playback::reduce`, `with_reduced_motion(!animations_allowed(prefs))`), and route the six actions through `playback::command_of` / config. Scenes then supply `Fields` with the `Vertex`/`Station`/`Instance` samples, `ParameterTrack`s and mode overlays (`surface`, `frequency`, `load_factor`).

## Assumptions to review
- Instance fields index by part order within the entity (one part per pattern instance for bolts/bars; cages emit several parts per instance).
- Vertex fields whose length differs from the mesh are resampled by index fraction: producers should emit exactly the mesh node count for grid shells and surfaces (row-major), otherwise colours are approximate.
- Plate mode surface is a mid-plane sheet (no thickness); no per-vertex field ramp on it yet.
- Baked meshes change per frame while a track plays; the hosts' mesh cache handles it by ids, but long sessions should evict unreferenced meshes.
