# 📋️ Wave W2 Brief — read fully

You implement one new work package of the BIM plugin from scratch (no previous agent). T = ticket folder
`C:\git\semio\.🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️10\☀️08\BIM-PLUGIN`. S = `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any`.

## Read before coding
1. `T/r11-finish-brief.md` — ALL hard rules apply (laws, schema-first generators, oracles + features, accessibility,
   i18n, progress/cancel, gate, concurrency, 256-char paths, no Grep tool, no modifying git).
2. Your WP section in `T/r9-audit-completeness.md` §5, `T/r9-decisions.md`, `T/r11-decision-families.md` (families,
   components, MEP — reuse them where your WP needs typed parameters or geometry), and these exec reports for the
   patterns already established in W1: `T/r11-exec-*.md` (as they appear), `T/r7-exec-z-depth.md`,
   `T/r6-exec-z-mutations.md`, `T/r3-golden-leaf.md`, `T/r7-api-model-session.md`, `T/r3-recipe-ui.md`, `T/r3-recipe-io.md`.
3. Look at a completed W1 package of the same shape before writing (e.g. ceilings `🔲️`, ramps `🛝️`, views `🖼️`,
   annotations `🪧️`, schedules `📋️`): copy its structure for leaves, inference field, editor entity/tool, IO, tests.

## Package completeness (every item, or a justified exclusion in your report)
Snapshot types via `T/r3-f1-gen-model.ts`; every mutation leaf (golden-leaf recipe, sum law, fixtures, enum, KINDS,
binary tag block of your own (pick a free 1000-range: check `📡️.protocol.semio`), grammar, root mounts, mutate feature
rows, generators); cross-kind id uniqueness + delete cascades/refusals; inference field(s) in the model graph with honest
dependency, gating + cache-transparency tests; diagnostics with en+de messages; editor: entity fields, tool/gesture,
commands, library entries, panels as needed, hotkeys, labels en+de, accessible; IO: IFC export + import (validated by
ifcopenshell), glTF/SVG where geometric, text/binary codecs; examples (house/office) via generators + replay tests;
language-agnostic `.feature` + third-party oracle per feature; `cargo test --lib` counts, wasm32-wasip2 check.

## Build state
The framework os-kernel may still fail (owner commit 677; agent `r11-store` owns 🌱️value/🏪️store/📡️spr until
`T/r11-exec-store.md` exists). Write code first; verify when it compiles. Report `T/r12-exec-<label>.md`; final message
≤ 12 lines.
