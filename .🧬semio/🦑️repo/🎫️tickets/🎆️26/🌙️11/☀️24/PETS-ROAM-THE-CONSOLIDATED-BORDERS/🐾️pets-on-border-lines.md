# 🐾️ Pets on the border lines

## Finding

On the consolidated-border layout all six pets of the overview stood on the footer line and never left it. The pet engine
already walks, hops, climbs walls, raises ladders, fires a grappling hook and plays a "mischief" prank; what it lacked was
ground. The survey (`🐾️pets/🎯️targets/⚛️react/🔨️modules/📡️survey/🟦️.ts`) treated every surface element that stands on
another one, or that acts on a press, as a solid body. In the new grid every card stands on the next and every overview
card opens on a press, so each card became one keep-out box: measured on the live page, 2 perches (both on the footer
line) out of 9 surfaces.

## What changed

| Change | Where |
|---|---|
| A surface can be declared a frame around open room (`open`): its top edge carries pets and its sides are walls, its box is never a solid | survey `SurveyOptions.open`, `PetLayer` prop `open` |
| The course names its ground as border lines: every card frame (open), the chips at a card's foot (solid bodies a pet stands on), the footer line | `COURSE_PET_SURFACES`, `COURSE_PET_OPEN` in `📚️course/…/🐾️pets/🎪️stage/🟦️.tsx` |
| A press on a pet inside an overview card is the pet's; beside it the card opens as ever | `COURSE_PET_CONTROLS` without the overview cards |
| The label of the title chip of a card with a topic is what that topic's pet may try to steal (`data-pet-prop`): the existing prank tugs a copy out and puts it back | `stealable` in `📚️course/…/🪟️chrome/🟦️.tsx` |
| No reserved strip above the footer any more | rule removed from `🎨️styles/⚖️footer-and-forced-colors/🎨️.css` |

After the change the same page yields 14 perches: on the lines between the card rows, on top of the foot chips and on the
footer line; pets spawn spread over them.

## Tests

| Test | Result |
|---|---|
| Survey vectors (Python reference `generate_survey_vectors.py` extended with `open`, new scene "cards that share their border lines are frames around open room"), replayed by `🐾️pets/🧪️tests/📡️surface-survey` | 71 of 71 |
| Course suites `🐾️pet-companions`, `🎛️pet-allowances`, `🎪️pets-on-stage`, `🎨️stylesheet-tree` (expectations moved to border lines, stealable label, no strip) | pass; one 5 s timeout under load ("names every pet on stage…") |
| Playwright on the release rehearsal, kit moved to the new ground (`CARD_PARTS`, `CHIPS`): "pets live on the home screen … standing on what the cards show", "a click on a pet says hello …", "shoots its grapple" | 3 passed |
| Typecheck of course product and site | exit 0 |

## Open

- **Not verified: travel between levels on the overview and an actual chip theft.** The grapple spec passes on a quiz page;
  nobody watched a pet climb, raise a ladder or tug a title label on the overview. The browser pane was hidden, where
  animation frames do not fire.
- **A title label is rarely reachable for the prank.** `stationFor` wants a perch at the fixture's height or a climbable
  wall beside its end; a title chip sits on the card's top line, so only a climber on the card's left side qualifies.
  Walking up to a chip and carrying it away is not built: there is no carry-a-chip activity in the schema.
- **Mischief on a quiz page never starts** ("a pet lifts a copy of a task …", both variants, 180 s): the task rows span
  the whole card, which spans the whole stage, so no side has `LIFT_ROOM`. This follows from the edge-to-edge cards, not
  from the open frames.
- "no two pets ever overlap …" fails on its count of thrown pets (1 of 3 wanted); the rest of the pets project
  (parachute, deeds at 320 px, phone, settings play, ladder) was not rerun after the changes.
- Stale comments in the survey (lines 271–276, "tab on its body") and the survey not watching border redraws
  (`SURVEY_ATTRIBUTES`) are untouched.

## Second round (24 Nov, afternoon): mischief, ladder and rope on the new ground

Measured on the dev page with headless probes kept in this ticket (`mischief-probe.mjs`, `gear-probe.mjs`,
`terrain-probe.mjs`, `dom-probe.mjs`, `watch-probe.mjs`).

| Question | What the probe showed | Decision |
|---|---|---|
| Why does no pet lift a task on a quiz's page? | The rows of the task list were no ground, and the marked row began 6 px from the window's edge: the only station (to its right, pushing left) had 6 px of room where `LIFT_ROOM` wants 12. | The rows of the task list are surfaces and open room (`COURSE_PET_SURFACES`, `COURSE_PET_OPEN`); the mark `data-pet-prop` sits on the task's title alone, so number and glyph stay in place and the title has 52 px to its left; the words of a row are keep-outs (`[data-icon-host] > *`). |
| Why does no ladder stand on a quiz's page? | Its cards are stacked over the whole width: ten walls, all on the window's edge with their air side outside, one pitch 24 px high. | No ladder there, by design of the page. The home grid has walls between its cards: the ladder spec moved to the home screen and tries the foot of every card side in turn. |
| Why does no rope fly on a quiz's page? | A control as wide as the page ("show it now" of the crowd card) lies between the footer line and every line above it; a rope never crosses a control. Between the row lines ropes do fly (three routes). | The rope spec moved to the home screen, where the footer line reaches three card edges. |
| Why did the rope spec still wait in vain there? | No pet lives on the footer line any more, and the spec only moved grapplers that already stood there. | The hand takes a grappler from any line, the lowest first. |

Rehearsed on the release build (production proctor):

| Spec | Result |
|---|---|
| "a pet lifts a copy of a task …" on hover and on focus | 2 passed (1.0 min, 1.1 min) |
| "… raises its ladder: on the home screen a climber raises its ladder against a card of the grid" | passed (1.1 min) |
| "… shoots its grapple there: on the home screen a grappler on the footer line reels itself up a rope …" | passed (5.9 min) |

Still open from the list above: carrying a chip away is not an activity of the schema (the prank lifts a copy and is
thrown off); the pets on the footer line of a quiz's page have no way up with gear (they hop or are recast).
