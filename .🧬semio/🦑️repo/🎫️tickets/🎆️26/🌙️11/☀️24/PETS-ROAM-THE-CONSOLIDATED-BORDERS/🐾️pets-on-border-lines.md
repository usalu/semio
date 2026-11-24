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
