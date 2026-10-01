# 📓️ Weekly schedule weekday slots

Ticket `26/10/01/FIX-ENERGY-WEEKLY-SCHEDULE-WEEKDAY-SLOTS`.

## Convention

`CreateWeeklySchedule` documents seven daily profiles, Sunday first. The leaf `x-semio-ui` text now names that order: slot 0 is Sunday and slot 6 is Saturday, the field order of EnergyPlus `Schedule:Week:Daily` (Sunday, Monday, Tuesday, Wednesday, Thursday, Friday, Saturday). Honeybee stores the same order in `ScheduleRuleset._schedule_week_comments` and `ScheduleRule.week_apply_tuple`.

## Day of week

`SimDate::day_of_week` is the Zeller residue used by EnergyPlus and by Honeybee (`_dow_text_to_int`, `does_rule_apply` → `week_apply_tuple[dow - 1]`):

| Value | Day |
|---|---|
| 1 | Sunday |
| 2 | Monday |
| 3 | Tuesday |
| 4 | Wednesday |
| 5 | Thursday |
| 6 | Friday |
| 7 | Saturday |

Checked against the civil calendar: 2026-01-01 is Thursday and the function returns 5; 2026-01-04 is Sunday and it returns 1. The previous docstring said 1 = Monday … 7 = Sunday. That note was wrong, and it is what made the W2-R report describe Saturday and Sunday as sharing slot 6.

## The index

Both `ScheduleSet::weekly_value` and the kernel lookup (`schedule_lookup_step`, weekly stage) indexed with `min(day_of_week, 6)`.

With the real numbering that reads slots 1…6 only. Slot 0 (Sunday) is never selected. Friday (6) and Saturday (7) both land on slot 6, so Friday receives Saturday's profile. Every earlier day is shifted one slot toward the weekend.

The slot is `day_of_week - 1`, clamped into 0…6. Sunday (1) reads slot 0; Saturday (7) reads slot 6.

## Checks

- Language-agnostic case: `✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🗓️schedule/🧫️fixtures/weekly-day-slots/🔣️.json`. Seven 2026-01 dates, one per weekday, each with a distinct hourly value.
- Rust: `weekly_value_reads_sunday_first_slots` and `weekly_schedule_lookup_reads_sunday_first_slots` read that case.
- Honeybee (`honeybee_energy` 1.123): `weekly-day-slots/🐍️.py` builds one `ScheduleDay` per slot, evaluates `ScheduleRuleset.values` on each case date, and checks the emitted `Schedule:Week:Daily` field order against `slotOrder`.

## Verification

| Command | Result |
|---|---|
| `weekly-day-slots/🐍️.py` (oracle venv, honeybee_energy 1.123) | `honeybee weekly slots 7/7` |
| `cargo test -p semio-s-artifact-energy-model --lib sunday_first` (s workspace) | `weekly_schedule_lookup_reads_sunday_first_slots` ok, `weekly_value_reads_sunday_first_slots` ok. 2 passed, 6304 filtered out |
