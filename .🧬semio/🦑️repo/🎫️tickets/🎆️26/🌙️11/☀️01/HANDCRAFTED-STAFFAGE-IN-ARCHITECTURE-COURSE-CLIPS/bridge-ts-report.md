# bridge-ts report: cast exporter

S = `🎓️teaching/🏛️architecture/🧍️staffage`. All files below are under S.

## Built
- `🔨️modules/📤️export/🎭️cast/🧬️schema/🔣️.schema.json` (draft-07, id `semio.staffage.cast/v1`; oneOf pose/clip, thermal-only environment/timeline, fps needs clip or timeline).
- `🔨️modules/📤️export/🎭️cast/🧬️schema/🟦️.ts`: types (`Cast`, `CastManifest`, `CastIndex`), dependency-free validator `castManifestIssues` / `parseCastManifest`.
- `🔨️modules/📤️export/🎭️cast/🟦️.ts`: `exportCast(manifest, ids, out, {catalog, language, signal, progress})` plus CLI (same `--key value` parsing as the general CLI). Reuses `evaluate`, `depict`, `writeSvg`, `frameSchedule`, `loadCatalogFromFiles`, `loadEnvironments`, `solveThermalAt`. No depiction change needed: `annotations` all off + `diagrams.freeBody` off + transparent paper gives figure-only output.
- Fixed viewBox: union of frame extents, each frame wrapped in `<g id="staffage-cast" transform>` (translate, or `matrix(-1 0 0 1 ..)` for mirror) so origin alignment is identical. Atomic publish (temp dir then rename), stderr progress, SIGINT/AbortSignal.
- Barrel: `📦️packages/🟦️typescript/🟦️.ts` exports `exportCast`, `parseCastManifest`, `castManifestIssues`, `CAST_SCHEMA_ID` and the types.
- Tests: `🧪️tests/🎭️cast-export/🟦️.ts` (17 cases), fixture `🧫️fixtures/🎭️cast-export/🔣️.json` (manifest in, index fields out, mirror pairs, invalid casts). Third-party cross-checks: ajv for the schema, jsdom for the SVG output.

## Results
`repo tools exec bun -- x vitest run --config ../../🧪️tests/🎚️config/🟦️.ts cast-export`: 17 passed. `tsc --noEmit` shows no errors in cast files (one pre-existing unrelated d3-format error). Smoke run of 4 casts (walker clip dark-line mirrored, radiator pose dark-solid, human thermal clip winter-overcast, radiator transient timeline) took about 4 s; inspected in the browser pane on #001117 (no rsvg-convert on this machine): figures upright, faces left when mirrored, thermal colours visible, frames aligned.

## CLI
`repo tools exec bun -- S/🔨️modules/📤️export/🎭️cast/🟦️.ts --manifest <file> --ids a,b --out <dir> --lang en` (unchanged from the contract; additions listed at the end of contract.md: optional `timeline`, static pose index = 1 frame/fps 1/durationS 0, transparent paper).
Smoke manifest: `cast-smoke.json` in this folder.

## Follow-up (audit sections 0 and 4)
- `theme` ("dark"|"light") is required on every cast (schema, TS twin `CAST_THEMES`, validator) and passed to `defaultDepictionOptions`, so thermal/forces/moments use dark ink on #001117. `V/🧍️staffage/🔣️.json` (48 casts) and the bridge-py fixture entry were migrated to `"theme": "dark"`.
- `transport` (optional layers) added; `🎨️depiction/💨️transport-view/🟦️.ts` now draws emitter markers/labels and layer tags only when `annotations.labels` is on (existing view test stub updated; new test for labels off). Cast environment feeds `environment`/`environmentTimeline` of the depiction options.
- Tests: 22 cast cases (ink per theme, plume figure-only, missing/unknown theme, bad transport layer) and the transport-view suite pass. `transport-field` has a 5 s timeout flake under machine load, not related.
- Contract: `contract.md` section "Changes" tells bridge-py.
