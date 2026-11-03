# 📖️ View Documentation Inventory

Slot 20 (haiku-docs) of the staffage-multiview fleet. Read-only inspection: no source, schema, catalog or test file was changed.
Snapshot: 2026-11-02, while other workers still edit the React viewer, the depiction preview and the README (uncommitted). Line numbers
are from that snapshot and will drift; the names of sections, keys and functions are the stable references.

Library (L): `🎓️teaching/🏛️architecture/🧍️staffage`. Paths below are relative to L unless they start with `/`.

## 1. Where views are documented today

| Location | What it documents | Status |
|---|---|---|
| `README.md`, "Pipeline" and intro | "The same state drives all seven views": dark line, dark solid, light line, light solid, thermodynamic, structural forces, structural moments. | "seven views" means the seven render modes, not the viewpoints |
| `README.md`, "Conventions" | "World x right, y up, ground at y = 0". No viewing direction is stated for the side view. | ambiguous, see 4.1 |
| `README.md`, "Front and plan views" | Every authored representation draws `geometry.elevation` (side), `geometry.front` and `geometry.plan` (top) in rest-pose metres bound to the same bones. Viewpoint is a depiction option: `--viewpoint elevation\|front\|plan`, `--views` for a side, front and top sheet. The viewer "offers all three always". | the only prose contract for the three viewpoints |
| `README.md`, "Solid placements", "Lateral spread", "Axial twist" | Front and plan shape rules, and the statement that elevation, front and plan agree in every pose. | prose only, no matrix |
| `README.md`, "Course casts" table | Cast row `viewpoint`: `elevation` (default), `front` or `plan`. `section` is written to the cast index for a sectioned front or plan view. | default silently `elevation` |
| `🧬️schema/🔣️.json`, `Viewpoint` (line ~71) | "The projection an authored geometry set is drawn in: the rig plane (side or section), the orthogonal front elevation, or the plan." | contradicts itself, see 4.2 and 4.6 |
| `🧬️schema/🔣️.json`, `Representation` (line ~1521) | "The decision for one preset scale." | does not mention views |
| `🧬️schema/🔣️.json`, `AuthoredRepresentation.geometry` (line ~1518-1540) | requires `elevation`, `front`, `plan`, each `minItems: 1`. | the strict rule |
| `🧬️schema/🔣️.json`, `AuthoredRepresentation.sections` (line ~1395) and `ViewSection` (line ~1413) | sections only for `front` and `plan`; `plane` in metres; `label` is `Text`. | side cannot be a section, see 4.6 |
| `🧬️schema/🔣️.json`, `Alternative` (line ~1430) | `kind` is `scale`, `symbol` or `asset`. No geometry of its own. | |
| `🧬️schema/🔣️.json`, `SymbolRepresentation.geometry` (line ~1532-1560) | requires `elevation`; `front` and `plan` are optional. | the "Alternative only requires elevation" gap in the brief lives here, see 4.8 |
| `🧬️schema/🔣️.json`, `solid` (line ~708) | front and plan solid placements. | |
| `🧬️schema/🟦️.ts` (the typed twin) | line 9 frame ("right-handed elevation plane: x to the right, y up"); line 49 `VIEWPOINTS` doc ("elevation ... side or sagittal view of a figure, the section of a component"); line 291-297 `sections` and `ViewSection`; line 328 `SymbolRepresentation.geometry` with optional front and plan. | twin, same drift |
| `🔨️modules/🎨️depiction/🖨️preview/🟦️.ts` header (lines 1-6) | `--viewpoint` draws side (`elevation`), `front` or top (`plan`); `--views` writes one sheet per view with the three viewpoints side by side. | matches the README |
| `🔨️modules/📤️export/🎬️cli/🟦️.ts` `USAGE` | `--view <view>` (render mode) and `--viewpoint elevation\|front\|plan`, default `elevation` when absent. | silent default, see 4.11 |
| `🔨️modules/📤️export/🎭️cast/🧬️schema/🔣️.schema.json` line 40 | enum `elevation`, `front`, `plan`; cast default `elevation` in `viewpointOf`. | silent default |
| `🧪️tests/✂️section-views/🥒️.feature` and `🧫️fixtures/✂️section-views/🔣️.json` | caption expectations: Side view, Front view, Top view, Section A-A, Schnitt A-A. | the most explicit bilingual caption spec |
| `🧪️tests/🧍️front-projection/🟦️.ts` line ~158 | German issue messages expect `Vorderansicht` for front and `Grundriss` for plan. | contradicts the caption set, see 4.4 |
| `🗂️catalog/🟦️.ts` (validator messages) lines ~235-348 | front and plan rules and messages (`elevation-lateral`, `follow-tip-viewpoint`, `section-plane-outside`, `front-empty`, `plan-empty`, `symbol-empty`). | German plan wording varies, see 4.4 |
| `🔨️modules/🎨️depiction/✂️section/🟦️.ts` (line 28-33) | captions: Side view / Seitenansicht, Front view / Vorderansicht, Top view / Draufsicht; sections Section X-X / Schnitt X-X; the cut-line marker of the side view. | the source of sheet captions |
| `🔨️modules/🎨️depiction/🖨️preview/🟦️.ts` `viewsSheet` | caption per viewpoint "label · Side view" etc. | uses `✂️section` captions |
| `🎯️targets/⚛️react/🔨️modules/🔭️viewpoints/🟦️.ts` (new, uncommitted, 16:25) | `VIEWPOINT_NAMES`: Side/Seite, Front/Vorne, Top/Oben, "All three"/"Alle drei"; `VIEWPOINT_PHRASES`: side view/Seitenansicht, front view/Vorderansicht, top view/Draufsicht; `MISSING` reasons. | a third German vocabulary, see 4.5 |

## 2. Labels in the React i18n (`🎯️targets/⚛️react/🔨️modules/🌐️i18n/`)

Both languages are typed to the same key set (`Record<keyof typeof VIEWER_EN, string>` in German), so a missing key fails the
type check. The view-related keys:

| Key | en | de |
|---|---|---|
| `viewer.view` | View | Ansicht |
| `viewer.viewpoint` | Viewpoint | Blickrichtung |
| `viewer.viewpoint.elevation` | Side | Seitenansicht |
| `viewer.viewpoint.front` | Front | Vorderansicht |
| `viewer.viewpoint.plan` | Top | Draufsicht |
| `viewer.background.auto` | By view | Je Ansicht |
| `viewer.contrast` | "...choose \"By view\" or another view." | "...wählen Sie „Je Ansicht“ oder eine andere Ansicht." |
| `playground.view` | View | Ansicht |
| `playground.view.dark-line` | Dark line | Dunkle Strichzeichnung |
| `playground.view.dark-solid` | Dark solid | Dunkler Vollton |
| `playground.view.light-line` | Light line | Helle Strichzeichnung |
| `playground.view.light-solid` | Light solid | Heller Vollton |
| `playground.view.thermal` / `.forces` / `.moments` | Thermal / Forces / Moments | Thermisch / Kräfte / Momente |
| `compare.view`, `compare.sameAsset`, `compare.sameView` | View, Same asset, other views | Ansicht, Gleiches Asset, andere Ansichten |
| `detail.export.lede` | "...the {view} view and scale 1:{scale}." | "...die Ansicht {view} und den Maßstab 1:{scale}." |
| `detail.export.sheetSingle` / `sheetAll` | This view, one page / All seven views, one page each | Diese Ansicht, eine Seite / Alle sieben Ansichten, je eine Seite |
| `compare.lede` | "...other views, styles and analytic states." | "...in anderen Ansichten, Stilen und Analysezuständen." |

The sheet and caption texts (Side view, Section A-A, ...) are not in the i18n module; they are bilingual `tx()` strings in
`🔨️modules/🎨️depiction/✂️section/🟦️.ts` and in the mode captions of `🔨️modules/🎨️depiction/🟦️.ts` (lines ~54-57).

## 3. Terminology in en and de

| Concept | en UI | en captions | de UI | de captions and prose | Schema and code name | Notes |
|---|---|---|---|---|---|---|
| side projection | Side | Side view | Seitenansicht (i18n, captions); Seite (new viewpoints module) | Seitenansicht (catalog prose, 257 substring hits); "Aufriss" 16; English "elevation" in catalog prose 836 | `elevation` | "elevation" is the architectural name of a front face; here it is the side |
| front projection | Front | Front view | Vorderansicht; Vorne (new module) | Vorderansicht (782 hits); Stirnansicht 15; Frontansicht 13 | `front` | German synonyms in prose only |
| top projection | Top | Top view | Draufsicht; Oben (new module) | Draufsicht (832 hits); Grundriss (503 hits) in prose and in validator and test messages | `plan` | "Grundriss" is the architectural floor plan; "Draufsicht" is the top view |
| section | (none) | Section A-A | (none) | Schnitt A-A; Querschnitt for the cross-section of a member (`detail.col.section`) | `sections.front`, `sections.plan`, `ViewSection` | side cannot be a section in the code |
| all viewpoints | All three | side, front and top view together | Alle drei; Seiten-, Vorder- und Draufsicht zusammen | | (none) | new, uncommitted |
| render mode (one of seven) | View; mode labels "Dark line" etc. | "Line drawing, dark" etc. (different wording) | Ansicht; "Dunkle Strichzeichnung" etc. | "Strichzeichnung, dunkel" etc. | `View` type (`VIEWS` in `🔗️contracts`), value `dark-line` | the word "view" is used for modes, see 4.3 |
| projection concept | Viewpoint | | Blickrichtung | | `Viewpoint`, `VIEWPOINTS`, `vp` URL key | |

Counts are case-insensitive substring occurrences in `🗂️catalog/**/🔣️.json` (not per-meaning), taken as a rough indicator only.

## 4. Inconsistencies

Severity: H means it blocks a correct contract or a reader's understanding; M means wrong or confusing text; L means cosmetic.

1. (H) **Handedness of the proposed frame.** The brief states "world x forward, y up, z the asset's left (+). This is right-handed." With
   x forward and y up, a right-handed frame has right = forward × up = x × y = +z, so +z is the asset's right. The README and the
   abduction text use +z as left (`abductionDeg`, "left of the asset, +z"), so the code's frame is left-handed unless the brief means
   z to the right. The README says only "World x right, y up" and does not give a viewing direction. The side view is seen from +z
   (the left) with x to the right on paper; that part is consistent. Decision for the coordinator: state the frame as x forward, y up,
   z left, and drop or correct "right-handed".
2. (H) **"Elevation" names the side.** The schema key, the TS twin, the cast enum and the English catalog prose (836 hits) call the side
   view "elevation". In architecture (English and German) an elevation or Aufriss is a vertical face, usually the front. The schema's
   own `Viewpoint` description calls the front "the orthogonal front elevation", so one description uses the word for two views.
   The structure module also uses "Aufrissebene" for the rig plane. Decision for the coordinator: keep `elevation` as the key name
   (renaming is a schema change), but make all prose say "side view" / "Seitenansicht" and stop using "elevation" for the front.
3. (H) **"View" is used for two different things.** The seven render modes are called "view" (i18n `viewer.view`, `playground.view`,
   `compare.view`, `VIEWS`, the README "seven views", hero copy "seven synchronized views"). The three projections are called
   "viewpoint" in code, but "view" in the captions and in the English and German catalog prose ("front view", "Vorderansicht").
   German "Ansicht" for the modes collides with "Ansicht" as the generic word for an architectural elevation. The brief also says
   "all seven views/modes". Decision for the coordinator: name the modes "mode" (or "render mode") and reserve "view" for the three
   projections, or keep "view" for modes and always say "viewpoint" for projections. Either way the text must be one of the two.
4. (M) **German plan wording.** Plan is "Draufsicht" in the i18n files, in the section caption ("Draufsicht") and in the new viewpoints
   module, but "Grundriss" in the validator messages for geometry (`plan-empty`: "Grundrissgeometrie"; `follow-tip-viewpoint`:
   "Grundrissformen"), in `🧪️tests/🧍️front-projection` (`Grundriss` expected for plan), and in `🧪️tests/🔤️caption-fit` (a caption
   "Größe in °C · Grundriss µ"). The validator's section message uses "Draufsicht" for plan in the same file. Architecturally "Grundriss"
   is a floor plan, so for a top view of an object "Draufsicht" is the better word. Decision for the coordinator; then one German
   word everywhere, including the test expectation.
5. (H) **Three vocabularies for the viewpoints, one of them new.** The i18n keys say Side / Front / Top and Seitenansicht / Vorderansicht /
   Draufsicht. The depiction captions say Side view / Front view / Top view. The new `🔭️viewpoints` module says Side / Front / Top
   and Seite / Vorne / Oben, with long phrases side view / Seitenansicht. "Seite" and "Vorne" / "Oben" are not in the i18n files and
   are not architectural German terms. The new module should use the i18n keys (or the caption set), not its own German. Owner of the
   viewer: sonnet-viewer; the German wording is a decision for the coordinator.
6. (M) **Side as a section.** `Viewpoint` says the rig plane can be "side or section", but `sections` only admits `front` and `plan`,
   and the `ViewSection` description says the side view shows the cut-line marker. Pick one: the side is never a section (fix the
   `Viewpoint` description), or add `sections.elevation` (schema change).
7. (M) **Two sources for the viewpoint captions.** The viewer labels come from the i18n keys (`viewer.viewpoint.*`), the sheet and
   export captions come from `✂️section` `tx()` strings. A section caption (Section A-A) comes only from `✂️section`. The two must match
   word for word; they currently do for en and de, but nothing enforces it.
8. (H) **Symbol geometry gap.** `SymbolRepresentation.geometry` requires `elevation` only (`minItems: 1`); `front` and `plan` are
   optional. The validator message `symbol-empty` (`🗂️catalog/🟦️.ts` line ~348) says "A symbol needs elevation or plan geometry" and
   accepts a symbol with plan only, which the schema already rejects, so that branch is unreachable. The brief's "Alternative.geometry
   only requires elevation" is this symbol rule. A conventional symbol that lacks a front or plan view is an explicit, named failure in
   the new contract, not a silent substitution.
9. (M) **Raw ids in user-facing text.** `detail.export.lede` receives `request.view` (for example `dark-line`), so the German reads
   "die Ansicht dark-line" and the English "the dark-line view". The lede names no viewpoint at all. The preview sheet column headers
   are raw `VIEWS` ids (`sheet()` in `🖨️preview`, no language). The PDF subject reads "view dark-line, elevation"; the SVG `desc`
   and `staffage:scene` metadata use raw ids. The README says all text is `{ en, de }`; these are not localised text.
10. (M) **Mode labels diverge.** The playground i18n says "Dark line" / "Dunkle Strichzeichnung"; the depiction mode captions say
    "Line drawing, dark" / "Strichzeichnung, dunkel" (and "Solid, dark" / "Vollton, dunkel"). Same modes, two wordings, two files.
11. (M) **Silent defaults against the no-substitution rule.** The export CLI defaults `--viewpoint` to `elevation`; `viewpointOf` in the
    cast exporter defaults to `elevation`; the cast README row says "`elevation` (default)". The brief forbids a silent substitution, so
    once the contract lands an absent viewpoint must be an explicit validation failure (or an explicit documented default with a reason).
12. (L) **Undocumented URL keys.** The routing module reads `view` (mode) and `vp` (viewpoint) and `scale`; `vp` appears in no document.
13. (M) **Front and plan rules are in prose only.** The README "Front and plan views" section and the schema descriptions do not state
    the coverage rule (every pose, every clip instant, every transition, every scale band, every alternative). The brief asks for a
    named failure when one combination is missing; the docs do not yet say so.
14. (L) **Scale in the README.** Presets are 1:500, 1:200, 1:100, 1:50 (README "Conventions"); the brief adds "any declared band". The
    README already says any positive denominator is allowed; the preset list and the coverage rule should match.
15. (L) **Front and plan drawing rule vs the TS twin.** README "Conventions" says x right; the TS twin says plan is "x ahead, y sideways"
    and front is "x across the width". Both are right for their view; the README should say which x is meant per view.
16. (L) **Heights of a cast.** README says `heightM` "always measures the figure"; the exporter computes `heightM` for front and plan from
    the paper height (`viewpointOf(cast) === "elevation" ? heightM : (height - 2 * MARGIN_MM) / k`). Verify the README sentence for
    front and plan before it is repeated in the handoff.

## 5. Places that need updates once the multiview contract lands

Owner in brackets, from the brief's table. Nothing below has been edited by this slot.

- `README.md` (coordinator and haiku-docs): intro ("seven views" vs viewpoints), Conventions (frame and handedness, viewing direction),
  "Front and plan views" (the coverage rule, named failures, no silent substitution, time, pose, scale and root motion held fixed),
  Alternative and symbol rules, "Course casts" row `viewpoint`, "Run" (the new test and proof-sheet commands). (contracts, coordinator)
- `🧬️schema/🔣️.json` and `🧬️schema/🟦️.ts` (sonnet-contracts): `Viewpoint` description (side vs section, front elevation), `Representation`
  description, `AuthoredRepresentation.geometry` description, `sections` and `ViewSection` descriptions, `SymbolRepresentation.geometry`
  (decide whether front and plan are required), `Alternative`, and any new coverage or view fields. Raise `WIRE_VERSION` on schema change
  if the viewer contract depends on it (memory note on wire versions).
- `🔨️modules/🗂️catalog/🟦️.ts` (sonnet-contracts): validator messages at lines ~235-348 (`elevation-lateral`, `follow-tip-viewpoint`,
  `section-plane-outside`, `front-empty`, `plan-empty`, `symbol-empty`); German "Draufsicht" vs "Grundriss"; new named coverage failures.
- `🔨️modules/🎨️depiction/✂️section/🟦️.ts` (sonnet-contracts): `CAPTIONS`, `viewpointCaption`; `🔨️modules/🎨️depiction/🟦️.ts` mode captions
  (lines ~54-57) if the mode names change; `🔨️modules/🎨️depiction/🖨️preview/🟦️.ts` (`viewsSheet`, `sheet()` headers, usage and header comment).
- `🎯️targets/⚛️react/🔨️modules/🌐️i18n/🔠️english/🎛️viewer.ts` and `🔡️german/🎛️viewer.ts` (sonnet-viewer): `viewer.view*`, `viewer.viewpoint*`,
  `playground.view*`, `compare.view`, `viewer.contrast`; `🔠️english/📖️detail.ts` and `🔡️german/📖️detail.ts` (`detail.export.lede`, `detail.export.sheet*`).
- `🎯️targets/⚛️react/🔨️modules/🔭️viewpoints/🟦️.ts` (new, sonnet-viewer): `VIEWPOINT_NAMES`, `VIEWPOINT_PHRASES`, `MISSING` (must use the same German
  words as the i18n and captions).
- `🎯️targets/⚛️react/🔨️modules/🎛️viewer/🟦️.tsx` (Segmented control, `viewpointName`, Tag), `🎛️viewer/🗺️stage/🟦️.tsx` (`viewpoint` prop),
  `🔬️detail/📤️export-panel` (request `view`, lede), `📤️downloads/🟦️.ts` (file description and metadata comment that carry `view` and
  `viewpoint`), `🗺️routing/🟦️.ts` (`vp` key, document it), `🧍️figure` and `🧩️element` (figure and element attributes, if any take a viewpoint),
  `🧪️tests/🧍️figure-render`, `🧪️tests/🧲️playground-drag` (sonnet-viewer, sonnet-validation).
- `🔨️modules/📤️export/` (sonnet-exports): `🎬️cli/🟦️.ts` (USAGE, default `elevation`, the `--environment` rule for `--view thermal`),
  `🎭️cast/🟦️.ts` (`viewpointOf` default, `section` in the index, `heightM` for front and plan), `🎭️cast/🧬️schema/🔣️.schema.json` and
  `🟦️.ts`, `🖼️svg/🟦️.ts` (desc and metadata with raw ids), `📕️pdf/🟦️.ts` (subject line with raw ids), DXF and DWG model scenes.
- `🔨️modules/🖨️representation/` (sonnet-exports): any representation text that names the views.
- Course consumer `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/`: `🧍️staffage/🔣️.json` (cast rows: `view`, `viewpoint`),
  its README section "Cast row" (the default), the Python API docs (no viewpoint argument yet), and the guard `🧪️tests/🐍️staffage_only.py`
  if the cast schema changes (sonnet-course).
- Catalog: `🗂️catalog/🔣️.json` (manifest rows, completion status per representation and per symbol) and the category assets under
  `🗂️catalog/<category>/<slug>/🔣️.json` (catalog prose uses "Stirnansicht", "Seitenansicht", "Grundrisssymbol"; the category workers own
  these files).
- Tests (sonnet-validation): `🧪️tests/✂️section-views/` (feature and `🧫️fixtures/✂️section-views/🔣️.json` captions), `🧪️tests/🧍️front-projection/`
  (German "Grundriss"), `🧪️tests/🔤️caption-fit/` (German captions).

## 6. Facts this slot did not verify

- No test was run and no build was started; all statements are from reading files at the snapshot time.
- The `🔭️viewpoints` module and several viewer files are uncommitted and still changing; their words may change before the contract lands.
- The brief states the current geometry rules; this slot did not check the depiction code against them.
