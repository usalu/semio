# Course Scheduled Quiz Access

The current UI calls the Energy topic a quiz; it contains Physical Understanding, Ventilation, Heating, Cooling and Energy Demand. Keep Energy and its five question sets available, and lock Parametrics, Grasshopper, Excel, BIM and Optimization.

Add a schema-first `unlocks` map to the course and its solution-free view. Keys name a topic, quiz or `quiz/task`. Values are Unix milliseconds: a date unlocks at that exact instant, null means awaiting scheduling, and an absent entry adds no restriction. Ancestor restrictions apply to descendants. This keeps authored schedules in one place and supports independent question dates without changing run contents or scoring.

Use the hidden URL argument `?preview=1` for author access. It is UI preview access, not authentication. Ordinary links, clicks, keyboard activation, navigation history, followed screens and restored runs must obey the schedule. Locked cards and questions keep their rendered preview behind a tint with localized status, and no operable controls.

Validation: language-neutral access vectors with JSON Schema/Ajv agreement, TypeScript and Rust implementations, browser interaction tests, actual course configuration checks, focused browser E2E, and relevant type/build checks through repo. Keep all transient output in this ticket's generated directory and remove it on completion.
