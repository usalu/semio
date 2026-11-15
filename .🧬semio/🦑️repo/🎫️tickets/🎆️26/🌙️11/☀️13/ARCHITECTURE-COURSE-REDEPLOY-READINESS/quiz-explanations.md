# Quiz explanations: item-specific physics feedback and a repetition guard

## Changed

- `🎓️teaching/🏛️architecture/⚡️energy/🧲️physics/❓️quiz/🔣️.json`: the 48 boilerplate explanations of the power-or-energy task (24 power, 24 energy) now carry item-specific English and German text: magnitude with unit, where it comes from and why it is a rate or an amount. Only those 96 lines changed (diff: 96 insertions, 96 deletions); ids, key order and formatting are untouched. Values stated in labels are honoured (2 kW x 0.5 h = 1 kWh, 1 kW x 2,000 h, 3 kWp x 950 h = 2,850 kWh, 150 m2 x 100 kWh/(m2 a) = 15,000 kWh, 100 g rice, 200 l x 40 K = 9.3 kWh, 1.5 and 4 MWh households matching the sorting task). Cross-references use the same file's other values (kettle, 60 Wh laptop, 100 kWh EV, 11 kW wall box, Isar 2, 33,000 kWh house).
- `🎓️teaching/🏛️architecture/📚️course/🧪️tests/🧩️curriculum/🟦️.ts`: new test per quiz, "<id> shares no explanation between more than 5 items per language" (10 tests, written before the content fix).
- Edits were applied by a throw-away script kept only in the session scratchpad, not in the repo.

## Measurement (all ten quizzes, excluding the 48)

Maximum repetition within one quiz and language: 5. The 2 x 3 repeats in physics are the sorting groups ("1 kW ...", "1 kWh ..."). The 5-fold sentences in ventilation (window, shaft, extract, central, decentral), Excel (7 commands), Grasshopper (9 roles), optimization (Pareto relations: 5, 5, 4) and demand (profiles, 4 each) were read: each states concrete, checkable facts (numbers, the dominance rule, the profile envelope) shared by deliberately parallel item variants of one concept, not a generic "this is a X" statement. They are legitimate and left unchanged.

## Threshold

5 (fails at 6 or more). It is the honest maximum of the remaining content; anything tighter would fail legitimate parallel variants. Physics itself now peaks at 3, so a future regression there is still caught only above 5; tightening per quiz was judged not worth a configuration table.

## Verification

- `repo run @teaching/architecture-course:test --skip-remote-cache`: exit 0, 19 files, 859 tests passed (was 849; +10 = one new guard per quiz).
- `repo run @teaching/architecture-course:check --skip-remote-cache`: exit 0.

## Unverified

- The new test was not observed red: the first run waited on memory capacity and was stopped; the pre-fix state was instead measured directly (two sentences x 24 occurrences in physics, per language).
- Physical figures are standard orders of magnitude, not looked up item by item: Hoover Dam about 2.1 GW, Isar 2 thermal about 3.95 GW, Vectron up to 6.4 MW and the 900 W EU vacuum-cleaner cap are from memory.
