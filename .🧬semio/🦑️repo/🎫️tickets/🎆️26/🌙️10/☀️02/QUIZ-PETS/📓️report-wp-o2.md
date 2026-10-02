# 📓️ Report — work package O2 (art polish: housy, waly, windowy, roofy, insuly, solary, battery, shady, venty, servy)

Ticket `2026/10/02/QUIZ-PETS`, written 2026-10-02. `TK` = this ticket folder, `AP` = `🎓️teaching/🏛️architecture/🐾️pets`, `P` = `🧰️framework/🛍️products/🐾️pets`, `OUT` = `TK/🗑️generated/wp-o2`.

**State: the ten species documents are polished in motion on the real stage, valid (exit 0), and the site suite is green (5 files, 131 tests). Only the ten `AP/<species>/🔣️.json` files were edited, with the Edit tool. Two rows of `AP/README.md` are now out of date — see §5 (for the coordinator).**

## 1. How I looked (tools, all in `TK`)

| Tool | What it does |
|---|---|
| `wp_o2_stage_sheet.ts` (bun) | Plays a species on the **real stage** (`openStage` / `advance` / `frameOf`, one-species cast, the actor's activity forced by replacing fields of the plain `Stage`), draws every frame with the React target's `depictionMarkup` and the product's own `🎨️.css`, and writes an HTML sheet: rest and gaze (pointer events), the same at 1×, blink, idle, both fidgets, walk on a ground with 10 px marks at the true travel, turning round, the end of a walk, greet, cuddle, squabble, sulk, sleep, fall + land — 9–23 frames each, with the species box at the feet. So the idle loop under every activity, the 8-tick fades, the turn squeeze and the gaze lean are the engine's, not mine. `--only` picks rows; `WP_O2_ENGINE` points to a copy of the engine. |
| `wp_o2_shoot.mjs` (node + Playwright) | One PNG per three rows, light and dark, plus per row how far the painted drawing (stroke included) reaches beyond the species box and below the feet (`<pet>-reach.txt`). |
| `wp_o2_measure.ts` (bun) | Pupil travel, one-shot clips that do not start/end on rest values, clip lengths per activity, and on the real stage how far a planted sole drifts over the ground while the species walks. |
| `wp_o2_lowest.mjs` | Names the shape that reaches lowest / widest in a row (found the culprits). |
| `wp_o2_menagerie.ts` + `wp_o2_gallery.mjs` | The ten as a menagerie of their own for the **stories gallery** (real `PetLayer`, mock cards): the probe watches the sandbox for a minute, measures per species the lowest painted point against the edge it stands on, and takes close-ups at device scale 2. |
| `wp_o2_montage.mjs`, `wp_o2_snapshot.sh` | Contact sheet of PNGs; a copy of the engine TypeScript (the live stage module was mid-edit by another agent twice — `turning is not defined` — so I worked on a snapshot and re-took it at the end; the snapshot is deleted). |

Gallery: started with the package's dev script on **port 6347** (`PETS_STORIES_PORT=6347 PETS_MENAGERIE=TK/wp_o2_menagerie.ts bun ./📜️script.ts dev`), stopped by its PID (58360, the only listener on 6347; the launcher itself had been ended by the task time limit before). I removed the dependency cache it created (`…/📦️packages/🟦️typescript/node_modules/.vite/stories-6347`). No other port was touched.

Two things about the measures: the engine now leans the bone that carries the eyes after the gaze, so the "rest and gaze" rows reach 2–3.5 px beyond the box (the blink rows are the true rest extents); and the browser's box of a *rotated* circle is the box of its rotated square, so servy's turning casters read "1.2 px below the feet" although their sole stays at exactly 0.00 (`measure.txt`).

## 2. What applies to all ten

- **Walk cycles no longer slide.** Before, the planted foot drifted 2.4–6.9 px over the ground per stance (measured on the stage). Every gait is re-keyed so that stride per cycle = speed × clip seconds: stance leg linear in time, swing leg eased and lifted, the hip dropping by `L·(1 − cos a)` so the sole stays on the line. Now ≤ 0.07 px per tick, total drift ≤ 0.06 px (table in §4). The two hop gaits (windowy, insuly) slid during the squash because the stage moves them at constant speed: `root.x` now lags and leads, so the feet are planted while it squashes and the body travels in the air.
- **`land` holds its squash**: 0.55–0.6 s (was 0.3–0.45 s), squash reached at 18–20 %, held to 50 %, overshoot at 76 %, rest at 100 %, all keys eased.
- **Nothing pokes below the ground line**: legs with a round cap were 0.5 px too long (housy, windowy, solary, battery, shady; insuly 0.25); several clips pushed feet through the ground (below). In the real layer over 4 × 60 s: lowest painted point 0.00…0.32 px below the edge for the walkers, venty always ≥ 0.9 px *above* it.
- **One-shot clips start and end on rest values** (`measure.txt`: no finding); rotation tracks that spin end on a multiple of 360°.
- **Eyes**: travel is ≥ 1.55 px for all ten with the new rim (`radius − pupil − 0.25`); only shady needed a change.
- Ids, palettes and temperaments are unchanged. Changed on purpose: speeds of housy (30 → 26), waly (24 → 20), solary (34 → 30); `size`/`hover` of venty; waly's clip `flex` → `knock`; servy's German noun.

## 3. Per pet

### housy — 48×52, walks 26 px/s
- Changed: `waddle` 0.5 s (was 0.64 s at 30 px/s; the 6 px legs cannot cover 19 px per cycle), legs swing ±36° and lift; legs 0.5 px shorter; `touchdown` held, roof slams down and bounces.
- Seen: chimney stretches and a smoke ring rises and grows (`smoke-ring`); a draught blows the door open, it shivers with clutched arms, the door slams (`shiver`) — both read at 2× and 1×. Greet tips the roof like a hat with the door open; cuddle leans over with the door ajar; squabble rattles the roof with arms akimbo. Mirrored face fine.
- Still bothers me: the open door is a 2 px sliver of cream at 1×; the roof corner reaches up to 6 px towards the partner in `rattle` (it just touches at the 6 px meeting gap).

### waly — 48×46, walks 20 px/s
- Changed: `march` 0.6 s (was 0.7 s at 24 px/s), heavy swing with a late plant; `thud` held, the loose brick jumps out on impact. **`flex` (a bodybuilder pose, nothing to do with a wall) is replaced by `knock`**: it knocks three times on its own side, the loose brick creeps out a little with every knock, it leans and shoves it back — the character sheet's "knocks on itself and listens", and both fidgets are now about bricks.
- Seen: `loose-brick` pops the brick out of the top course and pushes it back (clear); `knock` reads at 2×, at 1× mostly as "arm taps, brick creeps".
- Still bothers me: the arms are 9 px, so the knocking fist is small; the brick pops 5 px beyond the box to the front.

### windowy — 40×52, hops 40 px/s
- Changed: hop no longer slides during the squash (`root.x` ±3.56 px); legs 0.5 px shorter; `clatter` (land) held with both sashes flapping.
- Seen: `air` opens the right sash (dark opening behind it), shivers, shuts it; `gleam` sweeps the glint across both panes; hop with flapping sashes. Eyes sit in grey sockets on the glass and stay readable when a sash opens; mouth sits just above the sill.
- Still bothers me: the mouth is close to the sill and small; sashes "open" by narrowing (no perspective), which reads at 2×, less at 1×.

### roofy — 56×42, walks 46 px/s
- Changed: `scamper` re-keyed (was sliding 3.5 px per stance; the hip now drops with the stride and the swing leg lifts 2.2 px); `loose-tile` lifts the tile to 62° and lets it grow (was 38°, hardly visible); `ripple` doubled (rows jump 3.2 px and squeeze, the tile flips 40°); `hunker` (sulk) no longer pushes the left arm 2.6 px through the ground and 1.4 px out of the box; `plop` held.
- Seen: tile flaps while the arm fusses with it; ripple reads as two rows of tiles hopping after each other with the chimney bouncing. At rest roofy is exactly inside its 56 px box (eave arms included); the "eaves touch card borders" note of N came from clips (now ≤ 1 px in walk, 3–5 px in greet/cuddle/squabble).
- Still bothers me: tile lines are 1.5 px dark strokes on slate; the ripple is subtle at 1×.

### insuly — 48×40, hops 22 px/s
- Changed: hop no longer slides (`root.x` ±3.5 px); legs 0.25 px shorter; `flump` held; the foil tail unrolls less (tuck-in 2.3 → 1.9, snuggle 1.8…2.2 → 1.5…1.8: it reached 8 px behind the box, now 4.6 / 2.7).
- Seen: `fluff-up` — tufts swell one after the other, it shakes itself, settles (reads well, also at 1×); `tuck-in` unrolls the strip behind it and pats it. Squishy hop is the nicest gait of the ten.
- Still bothers me: nothing serious; the batting squiggle under the mouth is busy at 1×.

### solary — 46×52, walks 30 px/s
- Changed: `stroll` 0.4375 s (28 ticks; was 0.6 s at 34 px/s and sliding 3 px); legs 0.5 px shorter; in `sunbathe`, `dim` and `doze` the *panel* sinks instead of the hips (the feet went 1.5–2 px under the ground line); `sunbathe` tilts 17° instead of 21° and slides the panel 2.5 px forward on the hips (it reached 8.7 px behind the box, now 3.9); `touchdown` held with a nod of the panel. H4's open point: the mullion of the middle row now shows as two stubs above and below the mouth, so the grid reads 2×3.
- Seen: tilt towards the sun with the glint pulsing; `gleam` sweeps the glint over the cells and pops a sparkle at the corner.
- Still bothers me: mouth is ink on blue (3.1:1 on light); the walk clip is shorter than the design's 0.5–0.7 s.

### battery — 42×52, walks 46 px/s
- Changed: **legs hang on `root`** (were children of `body`: every bob and lean dragged the feet 0.6–3.1 px through the ground in greet, cuddle, squabble, sulk, sleep); hops moved from `body.y` to `root.y`; leaning clips lift the root so the trailing foot stays on the line; `bounce` is a run with a flight phase (stance 9 of 32 ticks, drift 0.00); `touchdown` held, the top bar blanks during the squash.
- Seen: `recharge` drains the bars top-down, sags, pops them back bottom-up, springs up — the clearest signature fidget of the ten; `zap` rattles the cap and throws a spark above it.
- Still bothers me (H4's open point, not fixable inside the palette): the charge window is `ink`, so on the dark theme it is cream and the mint bars have about 1.8:1 against it — the bars still read by their gaps and by appearing/disappearing, but weakly. A dark window on both themes needs a fourth palette colour or a "pupil" paint for parts. The spark flies 20 px above the box for half a second.

### shady — 44×48, walks 22 px/s
- Changed: eyes radius 3 → 3.4, pupil 1.35 → 1.5 (travel 1.65 px); **glasses 2.5 px lower**, so the whole eye shows above the rim at rest (they covered the lower half); `roll`, `shut`, `siesta` pull them 8.4 px up over the eyes on purpose; **squabble pushes them up onto the head rail** and glares (the eyes were hidden for the whole squabble); mouth rides on slat 5 (it floated over a slat gap, ink on ink, when the blind compressed — H4's open point); cord 1 px inwards (tassel was 0.7 px out of the box) and swings less in greet/cuddle (reached 8–10 px towards the partner, now 5.5–6.4); leans lift the root; `shuffle` re-keyed; `touchdown` held as a concertina squash.
- Seen: `slat-wave` tilts the slats open top to bottom (dark gaps run down the blind); `roll` rolls the blind down, glasses up, springs back. Gaze reads at 1× on both themes.
- Still bothers me: in sulk and sleep the eyes are behind the lenses by design (a blind that shuts) — say so if that is not wanted; the opposing slat tilts of `clatter` still look like a slightly crooked blind.

### venty — **44×41, floats 28 px/s, hover 3** (was 44×36, hover 6)
- Changed: the body sits 5 px higher in its rig, so the **jets end at the origin, inside the box**, and 3 px of clear air remain above the edge (casing 8 px above the edge, was 6). Jets are 0.75 px longer; every clip keeps them above the edge (measured lowest point ≤ 2.6 px below the feet = ≥ 0.4 px above the edge; in the real layer ≥ 0.9 px). **The fan really spins**: `hover` and `glide` key 0 → 360 per loop; `spin-up` turns three times with two air streaks shooting out of the back while it rises on longer jets. Leans turn the casing (`body.rotation`), not the whole rig about the ground point (it swung 4–6 px sideways). Sleep sits down onto the edge with the jets off; `touchdown` held, ducts squash. Streaks trail 14 px instead of 18–19.
- Seen: reads as a hovering unit on both themes; gaze reads at 1×.
- Still bothers me: the fan has three-fold symmetry and turns once per 3.2 s in idle — a calm rotation, not a blur; glide streaks still reach about 6 px behind the box.

### servy — 46×56, rolls 20 px/s
- Changed: **`thing` = "server rack" / "Serverschrank", `name` = "Servy, the server rack" / "Servy, der Serverschrank"**. Casters really turn: `roll` is 0.9375 s (60 ticks) = 18.75 px = one circumference of the r = 3 casters, hubs r 0.9 → 1.25 so the turning shows (H4's open point); the cabinet leans on the casters (`body.rotation` + `body.x`) instead of the whole rig rotating about the ground point, which lifted one caster and sank the other by up to 1.8 px; leaning clips lift the root; `touchdown` held.
- Seen: `count` runs the three green LEDs through a binary count while the arms type; `overheat` pulses the red LED with two heat waves rising. LEDs blink off one after the other in idle.
- Still bothers me: the walk clip is longer than the design's 0.5–0.7 s (one caster revolution); at 1× the hub dot is still only a hint; mouth ink on slate 3.1:1.

## 4. Verification — commands and real results (final files)

```
$ bun TK/validate_species.ts <the ten files>          (from /c/git/semio)
ok   🎓️teaching/🏛️architecture/🐾️pets/🏠️housy/🔣️.json
ok   …/🧱️waly/🔣️.json      ok   …/🪟️windowy/🔣️.json   ok   …/🛖️roofy/🔣️.json   ok   …/🧶️insuly/🔣️.json
ok   …/🔆️solary/🔣️.json    ok   …/🔋️battery/🔣️.json   ok   …/😎️shady/🔣️.json   ok   …/🌬️venty/🔣️.json   ok   …/🖥️servy/🔣️.json
validate exit=0                                         (full output: OUT/final/validate.txt)

$ bun ./📜️script.ts test        (in 🎓️teaching/🏛️architecture/❓️quiz/📦️packages/🟦️typescript)
 Test Files  5 passed (5)
      Tests  131 passed (131)                           exit 0 — includes 🐾️pet-cast (ajv + owned validator + grounds + casts)
```

`OUT/final/measure.txt` (real stage): no one-shot clip off rest; pupil travel 1.55 px (windowy) … 1.65 px; planted soles:

| Pet | Gait clip | Body per cycle | Stance drift | Sole |
|---|---|---|---|---|
| housy | waddle 0.5 s | 13.00 px | 0.03 px | −0.01…0.00, lifted 1.6 |
| waly | march 0.6 s | 11.88 px | −0.06 px | −0.01…0.00, lifted 2.0 |
| windowy | hop 0.6 s | 23.75 px | 0.00 px | 0.00, lifted 7.8 |
| roofy | scamper 0.5 s | 23.00 px | 0.04 px | −0.02…0.00, lifted 2.2 |
| insuly | hop 0.8 s | 17.53 px | 0.01 px | 0.00, lifted 6.5 |
| solary | stroll 0.4375 s | 13.13 px | 0.00 px | −0.01…0.00, lifted 1.8 |
| battery | bounce 0.5 s | 23.00 px | 0.00 px | −0.01…0.00, lifted 3.9 |
| shady | shuffle 0.6 s | 13.06 px | 0.00 px | −0.01…0.00, lifted 1.2 |
| venty | glide 0.7 s | 19.69 px | floats | jets ≥ 0.4 px above the edge |
| servy | roll 0.9375 s | 18.75 px | casters at 0.00 | one revolution per cycle |

Real layer (`OUT/gallery/*.txt`, lively, capacity 5, 60 s each, light and dark, no page or console error): lowest painted point below the edge — housy 0.10, waly 0.00, windowy 0.00, roofy 0.32, insuly 0.00, solary 0.10, battery 0.03, shady 0.05 px; venty −0.90 (above); servy 1.24 (the rotated-circle artefact). Farthest sideways beyond the box in an encounter or fidget: waly 6.6, housy 6.2, venty 6.0, windowy 4.2, insuly 4.2, solary 4.2, servy 2.7, battery 1.5, roofy 1.0 px (shady 9.5 before the cord was calmed; sheet value now 6.4).

Not run: cargo, e2e gates, deploy checks, the Rust twins. No git command that modifies anything.

## 5. For the coordinator

- `AP/README.md` roster row 20 still says `server rack / Rechenzentrum` → `Serverschrank`; row 15 says venty `44 × 36, floats` → `44 × 41`. `📓️explore-topic-pets.md` §2 (roster table) also names "das Rechenzentrum". I did not edit those files. No test or site README pins either value (grep).
- Design §9 asks for walk clips of 0.5…0.7 s: solary (0.4375 s) and servy (0.9375 s) are outside it on purpose (§3).
- Still open across the ten: battery's charge window on the dark theme (needs a palette decision); parts that swing towards a partner reach 4–6.6 px beyond the box in encounters, i.e. they touch at the 6 px meeting gap (housy's roof, waly's arm, venty's casing, shady's cord).
- The engine's gaze lean (5° and 1.5 px per unit of gaze on the eye bone) leans whole bodies for housy, waly, windowy, roofy, insuly, battery, venty, servy, because their eyes ride on `body`; it looks lively, but it is the reason "rest and gaze" rows reach 2–3.5 px beyond the box.

## 6. Paths

- Final sheets, per pet, light and dark: `OUT/final/<pet>-{light,dark}-00.png` (rest and gaze, the same at 1×, blink), `-01` (idle, both fidgets), `-02` (walk on ground marks, turning, end of a walk), `-03` (greet, cuddle, squabble), `-04` (sulk, sleep, fall and land); the pages themselves `OUT/final/<pet>.html`; reach per row `OUT/final/<pet>-reach.txt`; `OUT/final/measure.txt`; `OUT/final/validate.txt`.
- All ten at 1× on both themes: `OUT/final/roster-1x.png`.
- Real layer: `OUT/gallery/{envelope,technology}-{light,dark}-{0…5}.png` (whole sandbox), `…-<n>-<pet>.png` (close-ups at device scale 2), `closeups-{envelope,technology}-{light,dark}.png` (all close-ups of a run on one sheet), `*.txt`, `server.log`.
- Before my changes: `OUT/before/` (same layout, light only, `measure.txt`).
- Site suite log: `OUT/site-test.log`.
