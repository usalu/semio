# 🗂️ Work Board: Staffage Human Collection Wave

Wave launched 2026-11-22: 18 Opus 5.5 workers, 54 new human assets (catalog grows from 28 to 82 people). Brief: `📋️brief.md`.

| Worker | Theme | Assets | Status |
|---|---|---|---|
| HW-01 | Office work | standing-desk-worker, presenter-at-flipchart, office-worker-carrying-laptop | finished 22:11 by finisher A: 3 of 3 valid, check ✓, rows, report `📝️HW-01.md` |
| HW-02 | Hospitality | waiter-with-tray, barista, hotel-receptionist | finished 22:11 by finisher A: waiter-with-tray, barista valid, check ✓, rows, report `📝️HW-02.md`; hotel-receptionist never started |
| HW-03 | Healthcare | nurse-walking, doctor-with-clipboard, patient-with-iv-pole | finished 22:11 by finisher A: 3 of 3 valid, check ✓, rows, report `📝️HW-03.md` |
| HW-04 | Construction | construction-worker-with-shovel, worker-with-jackhammer, surveyor-with-total-station | finished 22:11 by finisher A: 3 of 3 valid, check ✓, rows, report `📝️HW-04.md` |
| HW-05 | Children and teens | walking-toddler, schoolchild-with-backpack, teenager-with-headphones | finished 22:11 by finisher A: 3 of 3 valid, check ✓, rows, report `📝️HW-05.md` |
| HW-06 | Elderly and care | elderly-person-knitting, mobility-scooter-user, carer-pushing-wheelchair | finished 22:11 by finisher A: elderly-person-knitting, mobility-scooter-user valid, check ✓, rows, report `📝️HW-06.md`; carer-pushing-wheelchair never started |
| HW-07 | Ball sports and yoga | tennis-player, basketball-player, yoga-practitioner | finished 22:11 by finisher A: tennis-player, basketball-player valid, check ✓, rows, report `📝️HW-07.md`; yoga-practitioner never started |
| HW-08 | Outdoor fitness | hiker-with-trekking-poles, person-stretching, inline-skater | finished 22:11 by finisher A: hiker-with-trekking-poles, person-stretching valid, check ✓, rows, report `📝️HW-08.md`; inline-skater never started |
| HW-09 | Urban mobility | e-scooter-rider, traveller-with-rolling-suitcase, person-pushing-bicycle | finished 22:11 by finisher A: 3 of 3 valid, check ✓, rows, report `📝️HW-09.md` |
| HW-10 | Eating and drinking | picnicker-sitting-on-ground, person-eating-standing, person-drinking-coffee | finished by finisher B 2026-11-22: 3/3 validate 0, check ✓, rows, report `📝️HW-10.md` |
| HW-11 | Gestures and photos | waving-person, photographer-with-camera, person-taking-selfie | finished by the worker, measured again by finisher B: 3/3 validate 0, check ✓, rows, report `📝️HW-11.md` |
| HW-12 | Culture and music | standing-violinist, painter-at-easel, dancer | finished by finisher B: 3/3 validate 0, check ✓, rows, report `📝️HW-12.md` |
| HW-13 | Domestic chores | person-sweeping, person-hanging-laundry, person-carrying-shopping-bags | finished by the worker, completed by finisher B: 3/3 validate 0, check ✓, rows, report `📝️HW-13.md` |
| HW-14 | Services | window-cleaner, postal-carrier, security-guard | postal-carrier finished by finisher B (validate 0, check ✓, row); window-cleaner and security-guard never started; report `📝️HW-14.md` |
| HW-15 | Accessibility | person-with-guide-dog, person-signing, person-with-arm-sling | finished by finisher B: 3/3 validate 0, check ✓, rows, report `📝️HW-15.md` |
| HW-16 | Weather and seasons | person-in-rain-poncho, person-in-winter-coat, sunbather-on-towel | finished by finisher B: 3/3 validate 0, check ✓, rows, report `📝️HW-16.md` |
| HW-17 | Waiting and crowds | person-queuing, person-checking-watch, parent-with-child-on-shoulders | finished by finisher B: 3/3 validate 0, check ✓, rows, report `📝️HW-17.md` |
| HW-18 | Logistics and grounds | worker-with-pallet-jack, person-carrying-ladder, gardener-with-wheelbarrow | worker-with-pallet-jack and person-carrying-ladder finished by finisher B (validate 0, check ✓, rows); gardener-with-wheelbarrow never started; report `📝️HW-18.md` |

## Coordinator integration (after the wave)

1. Merge `📥️manifest-rows/*.json` into `🗂️catalog/🔣️.json` in one write; add collections (e.g. workplaces, care-and-health, sports-and-leisure, urban-mobility, services), bilingual; extend `accessibility` and `street-life` where fitting.
2. Run the staffage test suite over the full catalog and the site build.
3. Delete `🗑️generated/` raw outputs, close the ticket with every file touched.
