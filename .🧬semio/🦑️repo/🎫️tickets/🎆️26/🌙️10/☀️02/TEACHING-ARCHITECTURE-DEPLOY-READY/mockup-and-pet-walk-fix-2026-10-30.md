# Mockup Region and Pet-Walk Throws

E2E run `55895`, report `🗑️generated/nx/e2e/20261030T102602-1008322/report/dev-shared`. Ticket `26/10/02/TEACHING-ARCHITECTURE-DEPLOY-READY` was already open under the teaching-architecture deploy goal. `repo://goals` was read first.

## Failure 3 — software mockups in de

`figure[data-mockup="grasshopper"]` contains two regions when the drawn item has a tree. The scroll surface is `div.course-mockup-scroll` with `role="region"`, `tabIndex={0}` and `aria-labelledby`. The data tree is `section.course-tree-panel` with `aria-label` (the unit test asks for the region named `Datenbaum` on purpose). The first canvas item in this run was "Alle Raumlisten", so `getByRole("region")` matched both and `focus()` failed strict mode.

The quiz spec now takes `.course-mockup-scroll[role="region"]`, then still focuses it and checks `aria-labelledby`.

## Failure 4 — lively minute throws

Trace `…/🐕️pet-walk-…-pets/trace.zip` (`test.trace`, monotonic origin `2097550`):

| Wall ms from test start | What happened |
| --- | --- |
| 22813 | Hash set to `#prefs` |
| 27483 | Escape |
| 27993–32446 | Prefs `data-opened` is gone; URL is back to `/` |
| 33391 | Overlap watch starts (`705` frames, `6` pets at once, `0` overlaps, `60105` ms) |
| 51611 | `housy` mouseup (first throw). Carry `mouse.move` was `14265` ms |
| 51909–53397 | `housy` leaves the hand and is `idle/perch` again (~1.5 s) |
| 74701 | `double-curved-surface` mouseup (second throw). Carry `mouse.move` was `16670` ms |
| 75553–77657 | That pet is `glide/chute`, top `593` → `804`. `housy` is already perched |
| 78242 | It has left the glide. Loop budget `LIVELY_MS - 15_000` is already spent |
| 78423–93497 | No third throw. The test only waits for the overlap watch |

`Back: Settings` in the failure snapshot is the history button after the prefs visit and Escape. The Settings control marked active is the overview card link `#prefs`, not an open prefs pane. The URL never returned to `#prefs`. That label did not eat the throw budget.

The loop stopped at two throws because each continuous carry cost about 14–17 s, and `expectPerched(..., ANSWER_MS * 2)` then waits until the same pet is perched. A parachute opens on a high release and sinks at 20–45 px/s (`LANDING_MS` is 60 s). Here the second glide was short, but the perch wait still crossed the 45 s guard, and the last 15 s of the minute only collected the overlap watch.

The lively loop now runs for the whole minute, prefers a pet not yet thrown, and after `mouseup` only waits until that pet's footing is no longer `hand`. A glide (`footing` `chute`) frees the hand at once, so the next perched pet can be dragged and thrown while the previous one is still in the air.
