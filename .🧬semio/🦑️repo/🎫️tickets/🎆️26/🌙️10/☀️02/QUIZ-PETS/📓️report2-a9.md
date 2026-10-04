# 📓️ Work package A9 (second round): drawing gear, tilt, tints and particles; the artists' preview tool

Ticket `2026/10/02/QUIZ-PETS`, second round, phase A. `P` = `🧰️framework/🛍️products/🐾️pets`, `PR` = `P/🎯️targets/⚛️react`, `TK` = this ticket folder. All results are from runs on 2026-10-03 (Windows, bun 1.4.2). Tool output: `TK/🗑️generated/a9/`.

**In one line:** two self-contained modules of the React target draw everything new of §21/§22 — `PR/🔨️modules/🧰️gear` (parachute, rope, hook, gun, carried and standing ladders, whole-drawing tilt) and `PR/🔨️modules/✨️effects` (particle pools, state tints) —, with their own structural input types mirroring §21, 23 tests with gl-matrix as oracle, 56 of 56 mutants killed, CSS for their roots and the grab cursor, and a rebuilt preview tool that draws species sheets with the product's own modules. **Nothing the layer does today changed**: the layer does not call the new code yet (that is C1).

## 1. Files

| File | What |
|---|---|
| `PR/🔨️modules/🧰️gear/🟦️.ts` | new: tools of an actor, standing ladders, tilt |
| `PR/🔨️modules/✨️effects/🟦️.ts` | new: particles and tints |
| `PR/🔨️modules/🖌️depiction/🟦️.ts` | anchored, behaviour-free: exports `SVG_NAMESPACE`, `Attributes`, `ShapeTag`, `Drawn` (= `Pick<Part, "shape" \| "fill" \| "stroke" \| "strokeWidth">`), `rounded`, `maker(document)`, `placementText`, `partShape(drawn)`; `depict` builds with `maker` (same calls, same order) |
| `PR/🎨️.css` | two anchored rules: `.pet-ladders, .pet-effects` (roots at the layer origin) and the cursor rules |
| `PR/🟦️.tsx` | barrel: four export lines (functions, constants, types of both modules; no name collides) |
| `P/🧪️tests/🧰️gear-depiction/🟦️.tsx` | new suite, 14 tests |
| `P/🧪️tests/🎆️effect-painting/🟦️.tsx` | new suite, 9 tests |
| `P/🔮️oracles/🔣️.json` | anchored: oracle `pets-react-gl-matrix` gains the capabilities `pets-react-gear-depiction`, `pets-react-effect-painting`, the functions used, and a paragraph of rationale |
| `P/README.md` | anchored: one paragraph after the React target's pacing paragraph |
| `TK/render_species_preview.mjs`, `TK/render_species_preview.entry.ts` | the preview tool, rebuilt (§5) |
| `TK/wp_a9_gear_sheet.mjs`, `TK/wp_a9_gear_sheet.entry.ts` | screenshot sheet of all gear drawings over their parameter ranges |
| `TK/wp_a9_mutants.ts`, `TK/wp_a9_vitest.config.ts` | mutation check of the two suites (sandbox copy of the target) |
| `TK/wp_a9_code_rules.mjs` | docstring emojis, no line comments, no console, no forbidden writes |

The directory names were copied from `bun TK/taxonomy_name_probe_v2.ts` (`✨️effects` = U+2728 U+FE0F, `🧰️gear`, `🎆️effect-painting`, `🧰️gear-depiction`; bytes checked with `xxd`); A2 had registered them.

## 2. Exports, signatures, input types

### 2.1 `🧰️gear`

Input types (structural; `GearTool` ≡ `ToolFrame`, `GearLadder` ≡ `LadderFrame` of §21, so the schema's types are assignable to them once they exist):

```ts
type GearChute = { kind: "chute"; open: number; sway: Turns };                // open 0 packed … 1 open, overshoot allowed; sway leans canopy+cords about the grip
type GearRope = { kind: "rope"; x: number; y: number; slack: number };       // far end, STAGE coordinates; middle hangs `slack` px below the chord (0 taut, < 0 bows up)
type GearHook = { kind: "hook"; x: number; y: number };                      // STAGE coordinates; points the way the rope arrives (up without a rope)
type GearGun = { kind: "gun"; aim: Turns };                                  // 0 = right, ¼ = down (screen angle, not mirrored)
type GearCarriedLadder = { kind: "ladder"; lean: Turns; length: number };    // lean from upright, clockwise on screen (¼ = lying, top to the right)
type GearTool = GearChute | GearRope | GearHook | GearGun | GearCarriedLadder; // of several of one kind the first is drawn
type GearBearer = { x; y; facing: 1 | -1; tilt: Turns; pivot: Point; tools: readonly GearTool[] }; // the second-round ActorFrame fields, stage units (before the draw size)
type GearLadder = { x0; y0; x1; y1; rungs; opacity };                        // foot → top, stage coordinates
type GearSpecies = Pick<Species, "size" | "grip" | "canopy">;
```

Functions and records:

```ts
tiltDegrees(facing: number, tilt: Turns): number                     // degrees the drawing turns inside its (mirrored) box, rounded to 0.01
tiltedPlacement(x, y, flip, scale, tilt: Turns, pivot: Point): string // CSS transform of <svg class="pet">; tilt ≈ 0 → exactly placementText (today's string)
canopyRim(species: GearSpecies): readonly [left: Point, right: Point] // where the cords end
equip(species: GearSpecies, document?): Equipment                     // builds every tool once, hidden
paintTools(equipment: Equipment, bearer: GearBearer): void
rackLadders(document?): LadderRack                                    // <svg class="pet-ladders">
paintLadders(rack: LadderRack, ladders: readonly GearLadder[], scale = 1): void
type Equipment = { species; back: SVGGElement; front: SVGGElement; chute; cords; canopy; rim; ladder: {group, rails, rungs}; rope; hook; gun; painted: Float64Array }
type LadderRack = { element: SVGSVGElement; ladders: {group, rails, rungs}[]; painted: number[] }
CHUTE_SPAN 0.75 (widths), CHUTE_RISE 0.7 (heights above the grip), CHUTE_DOME 0.45 (heights), GUN_REACH 0.42 (widths), GUN_HEIGHT 0.5 (heights), GUN_MUZZLE 11 px, LADDER_CARRY 0.45 (heights), LADDER_WIDTH 10 px, LADDER_RUNG 10.5 px
```

Conventions this package fixed (§21 does not say; B1/B3/B4 should emit accordingly):

1. **`tilt`**: turns, positive = clockwise on screen whichever way the actor faces (feet under a pivot above swing to the left). This is A3's `leanOf` exactly ("positive turns the x axis towards the y axis, clockwise on a screen; positive while the feet trail to the left"), so B1 can hand `leanOf(...)` through.
2. **`pivot`**: rig coordinates ("feet coordinates"): feet at the origin, x mirrored with `facing` (for `x = 0` both readings agree). Held: `{ x: 0, y: -species.grip }`.
3. **`rope` / `hook` `x, y`**: stage coordinates like `ActorFrame.x/y` and `LadderFrame` (the hook sits on a perch edge of the stage). The near end of the rope is not in the frame: it is the gun's muzzle while the frame holds a gun, else the pivot (a pet reeled up hangs at its pivot).
4. **`sway`, `aim`, `lean`**: screen angles in turns (clockwise), not mirrored.
5. **Plain canopy**: 1.5 widths wide, its rim 0.7 heights above the grip (0.4…1 of that while opening), dome 0.45 heights. For A5's `canopied(size, canopy)` (B1): `canopy = { width: 1.5·w + 2, height: grip + 1.15·h − h + 1 }` (+ stroke) for the plain one; an own canopy's extent is its shape at `CHUTE_RISE·h` above the grip.
6. **Gun hold**: the hand is at `(GUN_REACH·w·facing, −GUN_HEIGHT·h)`, the muzzle `GUN_MUZZLE` px along the aim (MECH §3.2 uses `(0.3 w, −0.65 h)`; B4 should take these constants for the line of sight so the drawn rope starts where the tested one does).
7. **Carried ladder**: held at its middle `LADDER_CARRY·h` above the feet, behind the pet; when raised so far that its lower end would go below the feet, it slides through the hold until its foot stands on the feet line. Rungs: `max(1, floor(length / 10.5))` — A4's `RUNG_SPACING` is the same 10.5.
8. **Own canopy** (artists): a `Shape` drawn in accent with an ink outline, authored around the point `CHUTE_RISE·h` above the grip — middle of the rim at (0, 0), canopy above (negative y), at its open size. Cords run to `canopyRim`: an ellipse's width ends, a rectangle's lower corners, a line's ends, an outline's leftmost and rightmost points (absolute and relative path commands are read; the lower of two equally far points) — the plain rim when the outline cannot be read. A2's sample `M -16 0 Q 0 -18 16 0 Z` gets its cords on its own rim.

### 2.2 `✨️effects`

```ts
type EffectParticle = { species: Slug; emitter: Slug; x; y; scale; rotation: Turns; opacity }   // ≡ ParticleFrame; stage coordinates
type EffectEmitter = Pick<Emitter, "id" | "shape" | "fill" | "stroke" | "strokeWidth" | "count">
type EffectSpecies = Pick<Species, "id" | "palette"> & { emitters: readonly EffectEmitter[] }
tintPalette(element: ElementCSSInlineStyle, palette: Palette, tint?: Tint): void   // writes --pet-body/accent/detail only where the value changes; no tint = the palette
stageEffects(document?): Effects                                                  // <svg class="pet-effects">
stockEffects(effects, species: EffectSpecies): void                               // all pools of a species ahead of time (idempotent)
paintEffects(effects, kinds: ReadonlyMap<Slug, EffectSpecies>, particles: readonly EffectParticle[], scale = 1): void
tintEffects(effects, species: EffectSpecies, tint?: Tint): void                   // also remembered for a species that has no particles yet
retireEffects(effects, id: Slug): void                                            // a species left: its group goes
strikeEffects(effects): void                                                      // teardown: everything goes
```

One group per species (`[data-pet-effects=<id>]`, carrying the palette, so particles follow the state tint and the theme), one pool per emitter of `floor(count)` shape elements (`[data-pet-emitter]`, paint classes of a part, `visibility="hidden"`). Per frame: the n-th particle of an emitter takes the n-th element; `transform="translate(x y) rotate(deg) scale(k)"` (place 0.01, turn 0.01°, size 0.001) and `opacity` only when changed, `visibility` only when an element comes into or falls out of use. Nothing is written when nothing lives. A particle of an unknown species or emitter, or beyond the emitter's `count`, is not drawn (the schema says `count` are alive at most; the pool never grows while particles fly).

## 3. How C1 wires them (paint, show, layer)

```ts
// 🖌️depiction · paint(): replace the placement write by the tilted one (two more memo slots: degrees, pivot x, pivot y)
const degrees = tiltDegrees(frame.facing * scale, frame.tilt);
if (x/y/flip/size or degrees/pivot changed) element.style.transform = tiltedPlacement(x, y, flip, size, frame.tilt, frame.pivot);
// 🫧️layer · show(): per actor, once with its depiction
const equipment = equip(kind, page); depiction.element.prepend(equipment.back); depiction.element.append(equipment.front); stockEffects(effects, kind);
// per frame, per actor (unscaled actor: stage units — the gear lives inside the scaled <svg>)
paintTools(equipment, actor);
const tint = kind.states.find((state) => state.id === actor.state)?.tint;
tintPalette(depiction.element, kind.palette, tint); tintEffects(effects, kind, tint);   // both write nothing when nothing changed
// per frame, layer-wide
paintLadders(rack, frame.ladders, size);              // rack = rackLadders(page), FIRST child of the layer (behind the actors)
paintEffects(effects, kinds, frame.particles, size);  // effects = stageEffects(page), LAST child of the layer
// when a species leaves: retireEffects(effects, id); on stop: rack.element.remove(); strikeEffects(effects)
// cursor: page.documentElement.setAttribute("data-pet-cursor", "grab" | "grabbing") / removeAttribute(...)
```

What C1 must adapt when it wires this in: `show()` today starts its `insertBefore` walk at `host.firstElementChild` — with the rack first and the effects last it has to start after the rack and stop before the effects. `T-DP` "draws one group per part" and `depictionMarkup` equality count the children of `svg.pet`: with `equip` in `depict` they must skip `g.pet-gear` (or `depictionMarkup` states the two groups). `T-DL`'s "every node computes `pointer-events: none`" and "never transitions" already hold for everything new (descendants of the layer). The cursor: a class on the document element is a `class` mutation the survey's `MutationObserver` hears (`SURVEY_ATTRIBUTES`) and makes the survey stale on every hover change; the stylesheet therefore also accepts `data-pet-cursor`, which it does not hear — use that.

## 4. CSS (`PR/🎨️.css`)

- `.pet-ladders, .pet-effects { position: absolute; left: 0; top: 0; width: 1px; height: 1px; overflow: visible; transform-origin: 0 0; stroke-linejoin: round; stroke-linecap: round }` — no `will-change` (they rarely move).
- `.pet-cursor-grab, .pet-cursor-grab *, [data-pet-cursor="grab"], [data-pet-cursor="grab"] * { cursor: grab !important }`, the same for `grabbing`.
- Nothing else: the tools use the paint classes; R's `transition-property: none !important` rule covers every new element (all are inside the layer); no `animation-name:`, no `pointer-events` value but `none`, no `@keyframes`/`@import`/`@apply` (the pins of `T-DP` and `T-DL` pass).

## 5. The preview tool (`TK/render_species_preview.mjs`)

Rebuilt for the second round: it bundles `render_species_preview.entry.ts` with `bun build` and draws the sheet **with the product's own modules** (core: `sampleClip`, `solveRig`, `particlesOf`, `emitterKey`, `speciesIssues`; React target: `depict`, `paint`, `equip`, `paintTools`, `paintLadders`, `paintEffects`, `tintPalette`, `tiltedPlacement`) — what a sheet shows is what the layer will show. The usage section is at the top of the file.

| Option | Meaning |
|---|---|
| `--states` | every state: rest under its tint; its overlay clip at 0.25 with its emitter's particles |
| `--tricks` | every trick's clip at four phases with a sample of its emitter (burst along its life, ring one lap on, stream when full), in the tint of its `from` state; then the purr |
| `--gear` | `hang` (tilt +20°, 0, −20°, 0 about the grip, red dot), `tumble` (tilt about the middle), `glide` under the open parachute (own or plain canopy; plus opening 0.45 and overshoot 1.25), `aim` with the gun, `reel` on a rope from the grip, `climb`/`slide` at a wall line on the facing side (+ on a standing ladder for a species with `ladder`), `mantle` over a corner, `carry` with a ladder (lying, raised), `push` against a block, `dizzy`, `shrug`, `scoot`; a line names the new activities without a clip |
| `--clips a,b` / `--clips all` | the plain clips at four phases (first theme only), as in round one |
| `--themes`, `--scale`, `--width` | grounds (default light,dark), screen px per pet px (default 3), page width |
| `--pages <px>` | cut a sheet taller than this (default 1600) into `<file>-1.png`, `-2.png`, … between bands, so each PNG can be looked at unshrunk; 0 = one PNG |

Without a section flag a sheet holds everything. Every species starts with its rest band (both themes) and a header: size, gait, grip, gear, counts of states/tricks/emitters, own canopy, and `valid` or the first issues of `speciesIssues`. Documents of the first round still render (missing fields are filled for the sheet; the header lists what validation misses). Examples:

```
node "TK/render_species_preview.mjs" --out "TK/🗑️generated/a9/preview/reference.png" "TK/reference-species.json"
node "TK/render_species_preview.mjs" --out "TK/🗑️generated/a9/preview/sample-blobby.png" --states --tricks --gear "TK/🗑️generated/a9/sample-blobby.json"
node "TK/render_species_preview.mjs" --out "TK/🗑️generated/a9/preview/housy-thermy-gear.png" --gear --scale 2 "🎓️teaching/🏛️architecture/🐾️pets/🏠️housy/🔣️.json" "🎓️teaching/🏛️architecture/🐾️pets/🌡️thermy/🔣️.json"
node "TK/render_species_preview.mjs" --out "TK/🗑️generated/a9/preview/chilly.png" --states --tricks --gear --clips glide,tumble "🎓️teaching/🏛️architecture/🐾️pets/❄️chilly/🔣️.json"
```

(`sample-blobby.json` is the first species of A2's sample menagerie in `P/🧫️fixtures/🧬️schema-conformance`, extracted because the twenty real species carry one resting state and no tricks or emitters yet.)

## 6. What I saw (PNGs looked at with the Read tool)

`TK/🗑️generated/a9/gear-sheet-x1-{light,dark}.png` (pets at their screen size), `gear-sheet-x3-*-row{1…7}.png` (enlarged), `preview/*.png`:

- **Parachute** reads at 1× next to a 48 px pet: red/accent dome with a paper middle gore, scalloped rim, four ink cords converging on the scruff; opening streams out narrow and close, 1.25 overshoots wide and flat. Sway and tilt lean canopy and body as one; mirrored pets keep the canopy upright. Thermy (32 px wide) gets a narrow tall canopy that still reads. A2's sample canopy (an umbrella half-ellipse ±16) gets its cords on its own rim.
- **Rope and hook** read: a 1.5 px ink line, sagging by its slack or bowed; the grapple hook (shank and two curled prongs, ~10 px) is small but legible at 1× and points along the arriving rope. Reeling (rope from the grip, body tilted ±20°) looks like hanging on a line.
- **Gun**: at the side, accent body, dark muzzle, ink handle kept down whichever way it aims (mirrored correctly). At 1× alone it reads as a short coloured tube; with the rope leaving it, it reads as a grappling gun. Acceptable; enlarged it is clearly a gun.
- **Ladders**: carried behind the pet (first try was in front and covered the eyes — moved to the back group); raised it stands on the feet line. Standing ladders (6 and 11 rungs, half faded, raising at 60°, two at once) read clearly on both themes.
- **Tilt**: +20° swings the feet left about the grip for both facings, −20° to the right; tumble about the middle; the red pivot dots stay put.
- **Tints and particles**: body-only and full tints, burst sparks, falling drops in a tinted detail, rising rings — all in the species' paints, on both themes (ink lines turn paper-coloured on dark through `--foreground`).

## 7. Commands and their real results

| Command | Result |
|---|---|
| `bun ./📜️script.ts typecheck` in `PR/📦️packages/🟦️typescript` | exit 0 (an earlier run during A2's schema change failed in A2's files only; A2 fixed them) |
| `bun ./📜️script.ts test` (same dir) | `Test Files 6 passed (6)`, `Tests 106 passed (106)` (83 existing + 14 + 9) |
| `bun ./📜️script.ts test gear-depiction` / `effect-painting` | `14 passed` / `9 passed` |
| `bun TK/wp_a9_mutants.ts` | `the untouched copy passes` … `56 mutants, 0 survived or did not apply`, exit 0 (§7.1) |
| `node TK/wp_a9_code_rules.mjs` | every file `clean`, exit 0 |
| `node TK/wp_a9_gear_sheet.mjs` | 18 PNGs written, no page or console error |
| the four preview commands of §5 | 4 + 8 + 6 + 7 PNGs, no page or console error; a first-round document (`git show HEAD:TK/reference-species.json`) renders with `10 issues` in its header |
| `node -e "JSON.parse(...oracles...)"` | parses |
| `bun TK/taxonomy_name_probe_v2.ts` | `0 fresh, 19 / 16 / 11 already present` |
| `cd TEST && bun ./📜️script.ts contract --owner "🧰️framework/🛍️products/🐾️pets"` | no output within 280 s (killed by `timeout`); not verified — earlier packages report the repository-wide backlog there |

### 7.1 Mutation check

`TK/wp_a9_mutants.ts` copies `PR` (barrel, stylesheet, modules) into `🗑️generated/a9/mutation`, breaks one expression at a time and runs both suites against the copy (`WP_A9_BARREL`). 56 mutants (mirror and tilt signs, the counter transform, attachment points, cords and rim, canopy opening and overshoot, rope slack and ends, hook direction, gun flip and units, rung spacing and skew, the ladder's foot, every no-write memo, visibility bookkeeping, pool size, tints, teardown). Two survivors on the way led to two more assertions (a hook on a zero-length rope; an outline that continues after `z`). Final full run (`TK/🗑️generated/a9/mutants.txt`): the untouched copy passes, 56 of 56 mutants killed, exit 0.

## 8. Deviations and decisions

1. **Two groups per actor** (`back`, `front`) instead of one: the carried ladder in front covered the face; parachute and ladder belong behind the parts, rope, hook and gun in front. Both carry the same counter transform.
2. **Conventions of §3 of this report** (rope/hook in stage coordinates, pivot in rig coordinates, screen angles) are decisions where §21 is silent.
3. **Cords follow the canopy's rim** instead of a fixed span, because A2's sample canopy is narrower than any fixed span; the rim is read from the shape (§2.1 point 8).
4. **Gun hold point** differs from MECH's muzzle (§2.1 point 6): MECH's point put the gun over the face.
5. **Particle pools are exactly `floor(count)`** and never grow; overflow is not drawn (schema: at most `count` alive).
6. **Cursor**: classes as asked plus `data-pet-cursor` (the survey does not hear data attributes).
7. **Depiction helpers exported** (behaviour-free refactor of `depict` onto `maker`) so the new modules share one drawing rule instead of copies.
8. **Preview tool rebuilt on the product** instead of its own re-implementation of the drawing rules; it now needs `bun` (for `bun build`) besides Playwright, and it breaks if a product module it imports does not bundle. It cuts tall sheets into numbered PNGs by default.
9. Twice I patched ticket tool files (`render_species_preview.entry.ts`, `wp_a9_gear_sheet.*`) with a short Python script instead of Edit; the strings held no backslashes, and both files were checked afterwards. No product file was changed other than by Write/Edit.
10. The test-harness contract check timed out (§7).

## 9. Open (for C1, C2, B1–B4, H1–H4)

- C1: the wiring of §3 and the test adjustments it names; memo slots for tilt in `paint`.
- B1/B3/B4: emit tools and pivots by §2.1 (1–7); clearance widths for the plain canopy by §2.1 point 5.
- C2 (stories): the gallery could show the tools by feeding `paintTools` from controls; the preview's `activityScenes` (in the entry) is a ready set of scenes.
- H1–H4: use `--states --tricks --gear`; author canopies by §2.1 point 8; check the header line for validation issues.
- Not done: no clip-specific poses for the tools (a rope held by hands, a gun in a hand bone): the rig has no hand bone; tools attach to fixed fractions of the box.
