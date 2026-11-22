# Author Brief — Used HVAC Components

The dev's order: "Use a massive wave of Opus 5.5 agents to extend the staffage with a huge collection of used HVAC components." You author ONE new staffage asset from nothing to published: the used HVAC component named in your prompt (its row in `assign.md`). Eighteen authors work in parallel, one asset each. A sibling ticket `R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/RECLAIMED-BUILDING-COMPONENTS-STAFFAGE-COLLECTION` runs the same kind of wave for reclaimed construction components at the same time: its assets (for example `reclaimed-cast-iron-radiator`, `reclaimed-cast-iron-stove`) are good worked examples once published, never yours to edit.

Paths: repo root `R` = `/Users/ueli/Documents/semio` (read `R/CLAUDE.md` first). Product `S` = `R/🎓️teaching/🏛️architecture/🧍️staffage`. This ticket `K` = `R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/USED-HVAC-COMPONENTS-STAFFAGE-COLLECTION`. Tool ticket `T` = `R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE` (tools only; its assignments, ledgers and reports are another session's and not yours to edit).

## What a used HVAC component is here

A dismantled heating, ventilation, air-conditioning, refrigeration or building-services component as a component exchange (Bauteilbörse) or an HVAC reuse dealer stores and offers it: the real standard product of its period with real dimensions, connections and weights, drained, capped and taken out.

- **Device physics as in the existing HVAC assets**: study `S/🗂️catalog/🔌️appliances/{gas-boiler,circulation-pump,duct-fan,mvhr-unit,split-ac-indoor-unit,hot-water-cylinder,ceiling-diffuser,thermostatic-radiator-valve}` and `S/🗂️catalog/🏗️machinery/{heat-pump-outdoor-unit,plate-heat-exchanger,finned-heat-exchanger,scroll-compressor,sorption-wheel,expansion-valve}` and `S/🗂️catalog/🧱️components/panel-radiator` for operating keys, emitters (supply/exhaust jets, plumes, condensate), heating circuits, `ratedPowerW` with the right `ratedPowerBasis` (README section "Power": a heat pump counts its electrical input, a boiler its fuel input, a fed radiator, coil, damper, duct, pipe or tank is `passive` 0 W).
- **`reinstalled` is the operating state**: back in service with its media at design temperatures and flows (flow/return temperatures, air volume flow, refrigerant state), so the thermal view and air flow show it working; add an `off`/`drained` reading where useful.
- **Traces of use and dismantling**: cut and capped pipe stubs, blanking plates on flanges, sawn duct ends, remaining hangers and brackets, limescale and corrosion tones, dented fins, faded casings, old labels and type plates, removed covers. Refrigerant equipment is stored recovered (state it), water equipment drained.
- `tags` MUST include `"component"`, `"reclaimed"`, `"used"` and `"hvac"` plus the trade (`"heating"`, `"ventilation"`, `"cooling"`, `"plumbing"`, `"controls"`) and material/type tags.

## What a used component is here (general rules, all binding)

A dismantled component as a component exchange (Bauteilbörse) stores and offers it for reuse. It is the real historic or standard product with real dimensions, and it shows that it was used and taken out:

- **Traces of use and dismantling**, modelled as real geometry at the scales where they are above the smallest feature and as texture/detail strokes below: old mortises, peg and bolt holes, notches, cut ends, flame-cut edges, remaining end plates and cleats, mortar remnants, cut reinforcement bars, old fixings and fittings, chipped corners, worn tread nosings, paint or patina as a material tone. Never invented damage that would make it unusable.
- **Storage state** as the default pose: how it really lies or stands in the yard (on squared-timber bearers, on a pallet with straps, on edge in an A-frame rack, in a crate, in a big bag) with the bearers, pallet, rack or straps as parts of the asset on their own bones where they stay behind when the component is lifted.
- **At least three poses**: `stored` (default), `lifted` (hanging from slings, a clamp, a forklift fork or carried, with the sling/attachment points as supports and the storage aids left on the ground) and `reinstalled` (the component back in its structural role under its service load; give it the load states that role has, e.g. distributed and point load for a beam, wind for a window). Add poses the subject asks for (door leaf open, casement open, radiator in operation).
- **At least two clips**: `lift` (stored → lifted, with root or part motion) and one clip of the reinstalled life (moving load, opening cycle, sun passing, heating up).
- **Thermal**: every exposed surface a region with real material data (emissivity, solar absorptance, conductivity, heat capacity; weathered and painted surfaces differ from new ones — say so with a source tag), multi-layer regions where the component is layered (box window, door, insulating glass, sandwich panel), `operating` and `ratedPowerW` where it is a device (radiator fed from elsewhere is `passive` 0 W; a stove or lamp has a nameplate; README section "Power"). Pose bindings exhaustive.
- **Structure**: the load path in each pose: bearer or pallet supports when stored, sling points when lifted, the real supports and loads when reinstalled; members with the real section properties of the historic profile or species (old INP/IPN tables, DIN 1025, DIN 1026, strength class of old oak or softwood, concrete of the period), masses from section × density, summing to `dimensions.massKg`.
- **Provenance**: `provenance.references` with the standards, historic profile tables and literature you used (cite precisely; tag parameters `literature`, `authored-estimate` or `illustrative` — nothing is `measured`). `tags` MUST include `"component"` and `"reclaimed"` plus material and type tags. `summary` and every text `{ en, de }`, English first, German second, real sentences, no default language.

## What an asset document is

One file `K/assets/<slug>/🔣️.json` = `{ "$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": { id, category: "components", title, summary, tags, provenance, dimensions, materials, rig, anchors, representations, symbols, poses, clips, thermal, structure, volumes, presentation, exports, completion } }` (the `$schema` string is the one the catalog files carry; you develop the candidate in `K/assets/<slug>/` and the catalog copy is written by the publish tool).

- `representations` `"50" "100" "200" "500"`: each `authored` with individually drawn 2D geometry for `elevation`, `front` and `plan`, pens, hatches, tones, `decisions`, `validRange` (the four bands must cover 1:20 … 1:1000 without a hole: 20–75, 75–150, 150–350, 350–1000) — or `unavailable` with reason, limiting feature and alternatives when the whole component is under 3 mm of paper at that scale (a single brick at 1:500; a pallet of bricks is not).
- `volumes` `"50" "100" "200" "500"`: per authored scale an `AuthoredVolume` with handcrafted `body`, `thermal` and `structure` solid lists and bilingual `decisions`. Each scale is its own model for its smallest feature (7.5 / 15 / 30 / 75 mm): the same object, proportions, bones and stable ids, never the 1:50 list with entries deleted.
- `exports`: all twelve formats as in the sibling assets. `completion: "authored"`.

Learn the contract from, in this order: `S/README.md` (Pipeline, Conventions, Power, Front and plan views with "Solid placements", Volume, 3D models), `S/🧬️schema/🟦️.ts` (typed twin with docstrings; normative JSON `S/🧬️schema/🔣️.json`), then two finished siblings as worked examples — `S/🗂️catalog/🧱️components/glulam-beam/🔣️.json` (131 kB, the smallest complete component: beam on two bearings, load poses, moving-load clip) and one closer to your subject (`steel-column`, `stair-flight`, `tilt-window`, `sliding-door`, `panel-radiator`, `solid-brick-wall-1960`, `curtain-wall-bay`, `tied-canopy`; `S/🗂️catalog/🪑️objects/storage-crate-stack` for stacked goods). Read them with small python scripts section by section (they are single-line-heavy JSON; do not dump whole files into your context). Then `T/validator.md` items 1–19 ("Rule changes authors must know"), `T/design.md` (volume rules, amendments and rulings), `T/r3-tools.md` (all of it), and the lessons of the previous component author: `T/../../☀️10/ENGINE-NEUTRAL-3D-MODELS-FOR-EVERY-STAFFAGE/authoring-components.md`.

## Quality bar

1. **Reference table first.** Before modelling, write into your report the real dimensions, section properties, densities and masses you model to, with sources (DIN/EN, historic profile tables, manufacturer-typical data). Then measure your model against it; more than 3 % off at 1:50 is a defect.
2. **Rig**: bones at the real pivots and at every part that moves or stays behind (component, bearers/pallet/rack, straps, leaf/casement hinges, sling carrier). Fixed joints for what is rigid, revolute/prismatic with real limits for what moves.
3. **Form and detail**: every part the real object has at that scale; profiles are extrudes with their true section (flange taper and root radii of old I-sections at 1:50), turned parts revolves, boxes only where the subject is a box. Stacked goods are modelled as the real stack (rows, layers, interlock, strapping, gaps) with `repeat`, not as one block at 1:50; coarser scales merge them into well-proportioned masses (`envelope: true` for typified rows, validator item 13). Nothing floats, nothing interpenetrates visibly, no gaps.
4. **2D and 3D agree** in elevation, front and plan at every scale (`volumes-silhouette`, `volumes-views-disagree`). Line hierarchy and hatches like the siblings; closed filled shapes for every physical part (the previous author's missed gates all came from parts drawn only as open polylines).
5. **Each coarse scale gets the same care**: true silhouettes, readable at its printed size, no stick figures, no copies of the finer scale.
6. **Thermal and structure lists** are spatial at every authored scale (regions as solids, layers nested, areas agreeing with `areaM2`; members with real sections along member bones, joints, supports by type at the support anchors, masses at the real centres).
7. **Poses and clips stay closed and natural**: render every pose at 1:50 and two instants of every clip.
8. Validators are the floor, not the bar. You are modeller and art director: iterate until a professional architectural-visualisation modeller would sign every sheet.

## Tools (native; never use `repo dashboard`, never wait on the `repo` graph)

```
cd "$T" ; BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1) ; C="$K/assets/<slug>/🔣️.json"
$BUN r3-validate.ts --file "$C" --no-print                                   # schema + validateAsset, about 1 s: your inner loop
$BUN r3-validate.ts --file "$C" --scale N --rules silhouette,connected --no-print   # one scale, named measured rules, seconds
$BUN r3-validate.ts --file "$C" --volumes                                    # the full proof (print suitability + every measured volume rule)
$BUN r3-sheet.ts --file "$C" --scale N [--list body|thermal|structure] [--pose ID] [--clip ID --t S] [--only <view>] [--cut]
$BUN r3-2d.ts --file "$C" --scale N [--pose ID] [--views light-line,light-solid,thermal,forces,moments]
$BUN "$S/🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --file "$C"                      # the product's whole pipeline: every pose, clip sample, view, scale; must print ✓
```

`--file` output goes to `T/🗑️generated/r3-sheet/file/<slug>.…png` and `T/🗑️generated/r3-2d/file/<slug>.…png` (named by the folder of the file, so keep the candidate in `K/assets/<slug>/`). OPEN every rendered sheet with the Read tool: every scale × body/thermal/structure, every pose at 1:50, the 2D drawings at every scale. Never claim to have seen an image you did not open. `r3-project2d.ts` and `volumes-*.ts` take catalog assets only; do not use them before you are published (you do not need them: draw the 2D by hand to the same numbers as the solids).

Build the candidate with your own small scripts `K/assets/<slug>/author-*.py|ts` that state the hand-chosen numbers and write the document (they are your chisel and stay in the ticket as input files); keep one authoritative script so the document can be regenerated. zsh does not split variables into words: write arguments out.

## Publish

When the full proof (`--volumes`) ends `VALID` with no finding, the check CLI prints `✓`, and you have looked at every sheet again:

1. Write `K/assets/<slug>/distinction.json` = `{ "en": "…", "de": "…" }`: one sentence per language that tells this asset from its siblings in the archive (what it is, the decisive dimensions and what its use left behind), as in the manifest rows of the siblings.
2. `cd "$K" && $BUN publish.ts <slug>` — copies the document to `S/🗂️catalog/🧱️components/<slug>/🔣️.json` and writes your manifest row and collection membership under a lock. It is the ONLY way you touch the catalog or the manifest. Re-publish the same way after a later correction.
3. `cd "$T" && $BUN r3-validate.ts components/<slug> --catalog-only` must print `VALID`.

## Rules

- One shared live worktree replicated to 50+ machines; others edit concurrently. NO git command that changes anything, no worktrees, no stash. Re-read a file right before you write it. Keep going when you see unrelated changes.
- Write only inside `K/assets/<slug>/` (candidate, scripts, `distinction.json`), your report `K/reports/<slug>.md`, and tool output folders named after your slug. Never edit another asset, the manifest by hand, `K/brief.md`, `K/assign.md`, `K/publish.ts`, anything of the sibling ticket, `AGENTS.md`, or anything in `T` except what the tools write. Never delete a file you did not create; never clear `🗑️generated` folders (delete only your own PNGs named `<slug>.*` when you are published, keep none as evidence unless your report cites it).
- Never weaken a rule, threshold, test or tool to pass. A rule that is measurably wrong for your asset (with numbers), a tool defect or a product defect: append a section `## <slug> — <rule or tool>` to `K/requests.md` (re-read before writing) with the measurement and your proposal, work around it without editing shared code, and continue.
- The host is shared (16 GB, 10 cores, load over 30): exactly one tool process at a time, in the foreground, no background jobs, no sub-agents, no vitest suites (the coordinator runs the catalog suites after the wave). A killed tool: wait a few seconds and retry.
- Do not open, close or reopen tickets or goals. Temporary logs carry the `[DEBUG] ` prefix. No comments inside definitions; every docstring starts with a unique fitting emoji. Every path stays under 256 characters.
- Never stop to ask and never stop halfway: take the most appropriate decision yourself, note it in the report.

## Report

`K/reports/<slug>.md`: reference table with sources; model-vs-reference measurements; rig, poses and clips; per scale the solid counts per list and what was merged or generalised; thermal regions and structure assumptions with sources; the final lines of the full proof, of the check CLI and of the publish run; which sheets you opened; what remains imperfect, stated plainly. Your final message is only: the report path and at most five lines (published or not, what is open).
