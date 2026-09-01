# 🔤️ Slug-Shortening Proof — 🌍️gis

**Rule** (full detail: `📓️goal-slug-rule.md`): tokenize on `-` → fold spelled-out numbers/fractions
to digits (`two-and-a-half`→`2-5`, no dots — `MUTATION_ID_RE` in `🧪️test`'s
`📦️index.ts:648` forbids them) → drop pure articles (`a an the of its every`), KEEPING
`after/before/between/from/to` (they carry ordering/range meaning) → if still >~40 bytes, drop
one downgradable quantifier (`single only just …`) → uniqueness-within-parent by re-adding the
shortest disambiguating original word, never a numeric suffix. Deterministic, same input → same
output.

**Applied to all 14 offending `🧪️tests/` sentence-directories in gis** (12 under `🗺️gismap`, 2 under
`🏔️gisterrain`) — e.g. `adds-the-old-town-region-after-the-harbor-district` (50B) →
`adds-old-town-region-after-harbor-district` (42B); `raises-the-exaggeration-from-one-to-two-and-a-half`
(50B) → `raises-exaggeration-from-1-to-2-5` (33B). No collisions arose, so the disambiguation step
never fired.

**References rewritten**: 7 shared files (1 `📦️glue.rs` crate mount, 2 `🧪️oracle/🔣️.json`
catalogs, 2 `mutate-*-1/🥒️.feature` Example tables — realigned — 2 `mutate-*-1/🦀️.rs`
`include_str!` sets) + all 14 fixtures' own `🦀️component.rs` (their doc comments/assert messages
name themselves). Verified: real `mutationCatalogProblems` validator passes on both rewritten
oracle catalogs; `git diff` on every touched `.rs` line is a pure substring swap, nothing else.

**Budget, measured on disk (not `git ls-files`, which goes stale after a plain `mv`)**:
over-budget paths in gis **73 → 65**; longest path **289 → 278 bytes**
(`.../📥change-imported-features/🧪️tests/imports-harbor-position-descriptor/📸️snapshot/⬅️before/🔣️component.json`).

**Budget NOT fully met, and the rule is not why.** All 73/65 over-budget rows trace to exactly
these 14 scenarios — nothing else in gis was ever over budget. But the fixed scaffolding before
any scenario name (`.../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<mutationDir>/🧪️tests/`
+ `/📸️snapshot/⬅️before|➡️after/🔣️component.json`) already costs 231–236 bytes for a mid-length
mutation dir — leaving under 10 bytes for the scenario name itself. Even
`✂️delete-route/removes-tram-route` (14+18 bytes, about as short as a real name gets) still
overflows by 7–8 bytes on its two `📸️snapshot/*` leaves. No amount of scenario-name shortening
closes this; only shortening mutation-directory names or the `📸️snapshot/⬅️before`/`➡️after`
wire-format nesting itself would, and both are out of this task's scope (not the identified
offender class, and the latter is a semantic directory this task must not flatten).

**Where the rule itself strains**: `raises-exaggeration-from-1-to-2-5` — the `-5` decimal
substitute reads as "2.5" only once you know the convention; it's the one real meaning cost, not
just a byte-cap overshoot.
