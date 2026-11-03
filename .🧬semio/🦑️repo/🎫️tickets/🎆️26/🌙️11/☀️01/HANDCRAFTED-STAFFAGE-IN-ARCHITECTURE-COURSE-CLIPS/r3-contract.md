# Round 3 scene/cast provenance contract (W05, `contract`)

S = `🎓️teaching/🏛️architecture/🧍️staffage`, V = `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals`, T = this ticket folder. This file supersedes the optional/defaulted parts of `contract.md`. Requests for changes: `T/r3-requests/<id>.md`.

## Rule

Every cast names every dimension of its drawing explicitly. The exporter, the Python loader and the scene all refuse a missing or unresolvable combination with an error that names the cast, the dimension and what is available. Nothing is substituted: no default scale, no default frame rate, no default ease, no default environment, no nearest or derived scale, no empty view.

## Identification of one drawn cast

| Dimension | Manifest field (S schema `Cast`) | Provenance field (`🔣️.json` index, `provenance`) | Rule |
|---|---|---|---|
| asset id | `asset` | `asset.id`, `asset.category` | must be in the catalog; exactly one file `🗂️catalog/<category>/<id>/🔣️.json` |
| asset revision | derived | `asset.revision` = `sha256:` + 64 hex of the asset file bytes with CRLF turned into LF | recomputed by the Python loader and compared, a mismatch fails |
| view style | `view` (`dark-line`, `dark-solid`, `light-line`, `light-solid`, `thermal`, `forces`, `moments`) | `view.style` | required |
| view direction | `viewpoint` (`elevation` = side, `front`, `plan` = top) | `view.viewpoint` | required; the authored scale must hold a non-empty geometry set for it; `mirror` only with `elevation` |
| theme | `theme` (`dark`, `light`) | `view.theme` | required |
| mirror | `mirror` (optional boolean, absent means not mirrored) | `view.mirror` (always boolean) | |
| authored scale variant | `scale` (required, one of 500, 200, 100, 50) | `scale.denominator` | the asset must hold an authored representation at exactly that denominator: `unavailable` and custom (derived) denominators fail |
| pose or clip | exactly one of `pose`, `clip` | `state.kind`, `state.id` | must exist on the asset |
| clock | `fps` (required for a clip or a timeline, forbidden for a still pose), `transportAgeS` (optional offset of a transport clip) | `clock.fps`, `clock.durationS`, `clock.loop`, `clock.offsetS`, `clock.frames` | `loop` and `durationS` come from the clip (a timeline is non-looping); `offsetS` is `transportAgeS` or 0 when the cast has no transport offset; the scene adds `rate` (see below) |
| thermal scenario | `environment` xor `timeline` (exactly one is required for the `thermal` view), `thermalSpanK` (optional), `transport` | `scenario.kind = "thermal"`, `scenario.environment` `{id, revision}` or `scenario.timeline` `[{t, id, ease, revision}]`, `scenario.spanK` (number or null), `scenario.transport` | a preset revision is `sha256:` + hex of the preset in canonical JSON (sorted keys, no whitespace, integral numbers without `.0`); every timeline key needs `ease` (`step` or `linear`) |
| structural scenario | none (the asset structure model and the pose or clip carry it) | `scenario.kind = "structural"`, `scenario.dynamics`, `scenario.gravityMS2` | for `forces` and `moments` |
| geometric only | none | `scenario.kind = "geometry"` | for line and solid views |

Clothing, activity and equipment state are authored inside the asset: a different clothing level, activity or operating state is a different pose or clip of the same asset (or a different asset), hence part of `state.id` and of the asset revision. A scenario that is not authored on the asset does not exist.

## Cast index (`🔣️.json` written by the exporter)

Existing fields stay (`version`, `cast`, `asset`, `view`, `viewpoint`, `section`, `frames`, `fps`, `durationS`, `loop`, `mmPerMetre`, `originMm`, `boundsM`, `figureBoundsM`, `heightM`) and `provenance` is required. Schema: `S/🔨️modules/📤️export/🎭️cast/🧬️schema/🔣️.schema.json`, `$defs/Index` and `$defs/Provenance`.

## Cache key (V `cast_key`)

sha256 over: the canonical cast entry (all fields above, so every scenario field), the asset revision, the revision of every named environment preset, and the digest of all exporter and schema sources. Entries are validated before hashing (`entry_issues`).

## Scene exposure (V `🧍️staffage/🐍️.py`)

- `staffage_provenance(cast, rate=None)` returns the validated provenance record of a cast (ensures the cast, reads the index, checks it against the manifest entry and the asset revision). With `rate` it adds `clock.rate`.
- `staffage(...)` and `staffage_motion(...)` return mobjects with a `.provenance` attribute (motion adds `clock.rate`).
- `drawn_casts()` returns the provenance of every cast drawn in this process since `clear_drawn_casts()`, so a scene or the runtime can dump what it depicted.

## Failure modes (all explicit)

| Situation | Where | Message names |
|---|---|---|
| unknown asset | exporter, Python key | cast, asset, catalog |
| missing/ambiguous catalog file | exporter, Python key | asset, matches |
| unknown pose or clip | exporter | asset, available poses or clips |
| missing `scale`, or scale not a preset | schema, twin, Python | cast, scale values |
| scale `unavailable` or not authored | exporter | asset, scale, authored scales |
| empty geometry for the viewpoint | exporter | asset, scale, viewpoint |
| thermal without environment or timeline | schema, twin, Python | cast |
| missing `fps` for clip or timeline | schema, twin, Python | cast |
| missing `ease` | schema, twin, Python | cast, timeline key |
| unknown environment preset | exporter, Python key | preset, presets |
| index without or with a different provenance | Python loader | cast, field |

## Test vectors (language-agnostic)

`S/🧫️fixtures/🏷️cast-provenance/🔣️.json`: revision vectors (LF normalisation), canonical-JSON vectors, valid and invalid entries with the issue text. Used by `S/🧪️tests/🏷️cast-provenance` (vitest, with ajv as the third-party validator) and by `V/🧍️staffage/🧪️tests/🐍️.py` (unittest, with `jsonschema`).
