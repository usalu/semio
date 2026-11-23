# L20 Locale Audit (en/de) for the Puzzle Editors and Time-Travel Scenarios

## Storage
- Editor labels are inline en/de pairs in Rust (`app_labels!`, `LocalizedLabel::native`), in `✏️editor/🗣️terminology/🦀️.rs` per dimension. 2D has 111 fields, 3D 96 and 5D 155, and every field has both languages.
- Time-travel harness copy: `🧰️framework/🔨️modules/⏪️time-travel/🧪️testing/🌍️copy/🟦️.ts` (en/de, 49 keys each). The puzzle overlay is `✏️s/🔌️plugins/🧩️puzzle/🧪️tests/⏪️time-travel/🌍️copy/🟦️.ts`.

## Findings
- 3D terminology `:40-41`: `vortices`/`vortex` have identical de values ("Vortices"/"Vortex"). It is a domain term, so this needs review.
- 5D lifecycle `🖐️5d-lifecycle/🟦️.ts:135`: `paletteQuery` "Add Part"/"Teil hinzufügen". The audit first called it dead, but on verification it matches 5D terminology `add_part` (`🗣️terminology/🦀️.rs:156`). It is correct and is a required field of `TimeTravelScenarioV1`.
- Time-travel `🧮️gaps/🟦️.ts:427,433`: `/\btrue\b/`, `/\bfalse\b/` on the lock value. This would break if de localizes booleans. Unverified.
- `🌄️drag-journey/🟦️.ts:198`: the negative regex has a partial de branch, so it can pass vacuously in de.
- `🔂️reload-check/🟦️.ts:52,55,74,77`: inline bilingual regexes instead of COPY keys. They work but are fragile.
- 3D lifecycle: bilingual and fixture-driven. No issue found.

Unverified until the en/de time-travel runs.
