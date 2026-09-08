# Test Layout Layering Update

Updated the shrink-only layering ratchet for moved tests. Five obsolete zero-reference entries were removed (189 total allowed references). Renderer and library budgets transferred unchanged at 2 and 39. The DSL fixture-sweep budget of 18 was split among its five extracted Rust leaves as 13 + 1 + 1 + 1 + 2. Total allowance fell from 466 to 277. No budget was increased.

Independent historical audit showed renderer already had 4 references against allowance 2 when this baseline was introduced. Library predecessor already had 66 against allowance 39; current code has 72. These existing overages remain visible. The full `bun nx workspace:verify-layering` execution before retargeting failed with 142 files and 5,200 excess references across unrelated production, schema, fixture, and script work. This update does not claim the repository-wide layering gate passes.

Validation: parsed final JSON; all seven canonical destination files exist; Rust split counts exactly match the 18-reference original allowance; no legacy test filename keys remain. The three unrelated missing non-test baseline entries were preserved. The singular `🧪️test` semantic module is valid and remains present for its Nx plugin.

```json
[
  "🧅️layering.json",
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/📓️test-layout-layering-update-2026-09-08.md",
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/📓️test-layout-layering-baseline-audit-2026-09-08.md"
]
```
