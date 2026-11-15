# Ladder spec flake (`mischief-ladder-and-phone`, "a climber raises its ladder")

## Root cause

The spec waited for a random event whose precondition the page rarely grants.

On the opened quiz page (`physics`, 1440 x 900) the survey gives the pets exactly one perch: the footer line (y 868). The card tops are blocked by text keep-outs, so no card is a perch. Every card wall sits 370 px or more above the line. Of all 12 walls, only the right side of the lowest card (x 1168) is within a ladder's reach (`ladderTo` is non-null only for `s6:right`; `outingsTo` offers one trip, none from any left wall). The only possible ladder foot is therefore near x = 1168 + 0.25 * rise, about 1260, at the far right of the footer.

A ladder trip is only taken when `embark` finds the whole claim of the trip clear (`claimClear`): the walk to the foot, the carry and the climb. Pets never pass each other on the footer, which is one lane, so a climber that has any other pet between itself and x 1260 (or a pet standing on the foot) can never set out. The pets keep the order they spawned in; only cast rotation reorders them, slowly. That is a heavy-tailed hitting time, not a rate: in a pure simulation of the page (the real survey, the real menagerie, the stage `advance`) 14 of 30 seeds raised no ladder in 16 minutes of pets' time, and others took 10 minutes.

Evidence:
- Two failures reproduced in the rehearsal with `--grep "raises its ladder" --repeat-each 12/16 --workers 4` (2 of 12 and 1 of 16). The dumps of the failing pages: all pets on the footer line, seven species cycled through, none of `hop`, `carry`, `climb`, `aim`, `reel` in 140-260 trail entries each. One run had `windy` standing at x 1239-1287 (on the foot), the other `kettly` at 1249; battery and solary were left of them.
- The page geometry was identical in passing and failing runs (survey dumped from the browser with the real `survey()`; 16 of 16 starts equal, the one footer 874 vs 868 difference came late and is not the cause).
- Offline simulation (scratch only, `/private/tmp/.../scratchpad/sim`, not committed) with the dumped survey: free pets: 14 of 30 seeds no ladder in 16 min. A climber put at the foot every 2 minutes of pets' time (what a hand does): 120 of 120 seeds raised a ladder, slowest 475 s of pets' time, median about 20 s.
- Not load, throttling or visibility: `visibilityState` was `visible`, tempo 8 and the budget is counted in pets' own time (`stagePasses`), so a loaded host cannot shorten it. The previous agent's 3x patience only made a rarer bad ordering less likely.

## Fix (spec, not product)

The product behaves as designed (pets queue on the footer; a ladder needs a clear way). The spec asserted something the product does not guarantee, so it now creates the precondition and waits for the pet's own choice.

- `🪜️mischief-ladder-and-phone/🟦️.ts`: the climber test runs rounds. Each round, if no pet is on a ladder, it takes a ladder climber with the hand to the free footer spot nearest the computed foot of the ladder (`ladderFoot`: right side of the lowest visible card part plus `LADDER_LEAN` of the rise), as the rope test already does for grapplers, then waits `LADDER_ROUND_MS` (15 s wall, 2 min of pets' time). `LADDER_MS` (the 3x padding) is replaced by `LADDER_ROUND_MS` and a total of `2 * WALK_MS` (16 rounds). Rationale: one round in five fails, one in twelve needs a second, so each further round fails about two in five; 16 rounds leave about 2e-7. The wait is only spent when unlucky.
- `🧰️pet-kit/🟦️.ts`: `floorSpot` takes an optional `near` (default: the middle of the window, so the rope spec is unchanged).
- `🧰️framework/🛍️products/🐾️pets/🔨️modules/🎪️stage/🧪️tests/🦾️body-and-gear/🟦️.ts`: new unit test "sets out for its ladder only along a way no other body occupies" for both companies: the same climber sets out when a pet idles beyond the foot and does not when one idles between it and the foot. This pins the mechanism (it passes before and after, since the product was not changed).
- No debug code is left in the spec.

## Commands

- `repo run @teaching/architecture-course:test-e2e --configuration=release --skip-remote-cache -- --topology rehearsal -- --grep "raises its ladder" --repeat-each 12 --workers 4` before the fix: exit 1 (2 failed of 12, with the 1x budget, debug dump spec); `--repeat-each 16`: 1 failed.
- same with `--repeat-each 24` after the fix: exit 0, 24 of 24 passed (the old failure rate of about 15 percent would pass 24 of 24 in about 2 percent of cases).
- `repo run @semio-tech/pets:test --skip-remote-cache`: exit 0, 1166 passed (34 files; body-and-gear 48 tests).
- `repo run @semio-tech/course-react:test --skip-remote-cache`: exit 0, 1100 passed (45 files).
- Full rehearsal, run 1: `repo run @teaching/architecture-course:test-e2e --configuration=release --skip-remote-cache -- --topology rehearsal`: exit 0, 1 warmup + 74 shared + 12 exclusive passed, 1098 s.
- Full rehearsal, run 2 (same command): exit 0, 1 warmup + 74 shared + 12 exclusive passed, 1410 s.

## Not proven

- 24 of 24 and the simulation bound the flake rate, they do not prove zero: the residual is the pet's own random choice (about 2e-7 per run by the tail of the simulation fitted over 120 seeds; the tail was extrapolated, not measured to that depth).
- The simulation drives `advance` with fixed 8-tick frames and a re-survey every 64 ticks, not the real pacer, and approximates the cast (initial cast of a seed, no rotation), so the numbers are indicative.
- The dev topology was not repeated; the cause is not topology-specific (why it passed there was not investigated; by the mechanism it can only be luck of the order).
- Whether a pet that sits left of a pet on the lane should ever get past it (a product question) was not touched.
