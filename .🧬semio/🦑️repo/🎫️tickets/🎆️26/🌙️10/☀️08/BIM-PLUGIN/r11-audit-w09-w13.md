# 🔎️ R11 Audit — w09-ramps (WP-09) and w13-schedules (WP-13)

Status: both **NEARLY DONE**. The ramp-runs field lives in `🛝️ramp-runs` (not `🪜️`).

## WP-09 present
Leaves `🛝️create-ramp` (13), `🛹️set-ramp` (12), `🛼️delete-ramp` (4, hosted-railing cascade), tags 9000–9002; enum,
KINDS, mounts, id uniqueness, storey cascade, sum laws; railing host (stair/ramp/slab edge) in create/set-railing;
inference `🛝️ramp-runs` (15 tests) + solids `🛝️ramps` (11) + `🪝️rail-hosts`, graph-wired; plan linework (no tests);
`ramp_quantity` (no test); `RampSlope` emitted (no test); mutate feature rows; ramp tool (shift+t), host picker, en+de;
IFC export (IfcRamp, IfcRampFlight, landings, railings) + import, unit tested.

## WP-09 missing
1. Compile: `✏️editor/🧵️gestures/🧱️chain/🦀️.rs:120` → `ctx.name_of(|labels| labels.kind_ramp, count)`.
2. RampSlope/RampNoRun diagnostic tests; ramp quantities test; plan-linework ramp tests.
3. Python slope oracle + `.feature`; IFC python oracle IfcRamp/IfcRampFlight checks.
4. Ramp in house/office example; model-graph ramp gating test.

## WP-13 present
Leaves `📊️create-schedule` (11), `📈️set-schedule` (13), `📉️delete-schedule` (2), tags 13000–13002; registration, id
uniqueness; inference `📋️schedules` (`🗂️rows`, `🧮️table`) graph-wired; snapshot `📋️schedule-kit` PRESETS (door, window,
room, wall, material); schedule window reads inference; schedule editor `✏️edit`, config, vocabulary, `📋️edit-schedule`
command, entity, en+de; CSV codec `📊️csv` registered; mutate feature rows.

## WP-13 missing
1. Test modules (`📋️schedules`, `📊️csv`, `🧮️schedule/✏️edit`) — owned by r11-baseline until its report exists.
2. Storey delete leaves schedule storey scopes dangling → RULING: `delete-storey` cascades (deletes) schedules scoped
   to that storey with concrete inverse creates, like views. Test in `🚮️delete-storey/🧪️tests`.
3. Python oracle for schedule rows + `.feature` ("a quantity edit updates the schedule"); model-graph gating test.
4. CSV export action in the schedule window (through the stdio csv dialect, as a job with progress/cancel).
5. Room finish schedule preset (finish areas from `🎨️finishes`), door schedule in the house example + `🧰️checks`.
6. Tests for `🗂️rows`/`🧮️table` (natural order, filters, grouping, totals, phase/storey scope, material layers).
