# r2-ed: reference environment and small-object dimensions

S = `🎓️teaching/🏛️architecture/🧍️staffage`.

## 1. `thermal.referenceEnvironment`
- Schema: `S/🧬️schema/🔣️.json` (`ThermalModel.properties.referenceEnvironment`, optional, `oneOf` Slug | Environment) and twin `S/🧬️schema/🟦️.ts` (`referenceEnvironment?: Slug | Environment`). The inline form is the existing `Environment` (same shape as the preset's `environment`, as casts resolve presets by id).
- Resolver `resolveThermalEnvironment(asset, presets, explicit?)` in `S/🔨️modules/🗂️catalog/🟦️.ts`. Precedence: explicit preset, then `referenceEnvironment`, then `undefined`. With `undefined` the solver applies the asset's own `thermal.environment` (the existing explicit default; no hidden fallback added). An unknown id throws and lists the presets.
- Validator: `AssetContext.environments`; an unknown preset id gives `unresolved-reference thermal.referenceEnvironment`. The catalog check (`🩺️check/🟦️.ts`) loads the environment document and passes the ids (`CheckOptions.environments`).
- CLI `S/🔨️modules/📤️export/🎬️cli/🟦️.ts`: `--environment <preset>`; thermal view only (refused with another view); the resolved environment goes into the depiction options. Header doc and usage updated.
- Asset: `solar-thermal-collector` declares `summer-heatwave-calm` (900 W/m2, 0.3 m/s, 38 degC).
- README `S/README.md` (Thermal options) documents declaration, precedence and default.

## 2. Dimension layout
`dimensionLine` in `S/🔨️modules/🎨️depiction/🏷️annotations/🟦️.ts`: if the paper span between the extension lines is below `DIMENSION_FIT_MM` (3 mm), the label is lifted 3.2 mm above the upper extension line on a leader (`dimension:leader`, extension of the dimension line), so the text clears the lines and tick tips. Otherwise unchanged (centred between the lines). Applies to every viewpoint and scale.

## Tests (new)
- `S/🧪️tests/🌤️reference-environment/🟦️.ts` + `🥒️.feature`: schema JSON via ajv, validator, precedence, physics (absorber warmer under the sun), CLI output byte-equal to the library oracle, override, tealight keeps `thermal.environment`, refusals. 12 cases.
- `S/🧪️tests/📐️dimension-leader/🟦️.ts` + `🥒️.feature`: tealight and thermostatic radiator valve (leader, label clear of lines, ticks, right of ticks), standing adult (no leader, label between the lines). 3 cases.

## Runs
- Library vitest (q2 alias config): 58 of 58 files, 1161 passed, 11 skipped.
- React vitest: 18 of 18 files, 117 passed, 1 skipped.
- Catalog check `--asset`: solar-thermal-collector, tealight, thermostatic-radiator-valve all pass.

## Open
- The catalog check's thermal solve still uses `thermal.environment`, not the reference environment.
- The cast exporter keeps its own per-cast `environment`; it does not read `referenceEnvironment`.
- No visual render of the leader layout was looked at; geometry is asserted from the SVG numbers.
