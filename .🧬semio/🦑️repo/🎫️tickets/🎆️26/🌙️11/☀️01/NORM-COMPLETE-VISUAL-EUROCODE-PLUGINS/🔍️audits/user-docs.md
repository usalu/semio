# User documentation of the visual Eurocode editors

## Convention found

The `🎯updateduserdocs` goal (`UPDATED-DOCS/UPDATED-USER-DOCS`) has no content folder. Existing user docs are `README.md` files with front matter `name: <plugin>` and `kind: user` in each plugin root (for example `procedural/README.md`, `writer/README.md`), with the languages as sections `## English` and then `## Deutsch` inside one file (procedural). Language-specific files elsewhere use `.en.md`/`.de.md`. The norm plugin had no README. Followed: one README per plugin plus one per Eurocode artifact, each English first and German second, no default language.

## Files written (NORM = `✏️s/🔌️plugins/📕️norm`)

- `NORM/README.md`: shared guide (windows, selection, editing, drag handles, live evaluation, utilisation colours/patterns/glyphs, unverified basis, keyboard shortcuts from `focus-order` and `keyboard`, accessibility, customisation, known limits).
- `NORM/🗿️artifacts/<code>/README.md` for the ten codes: `⚖️en1990`, `🏋️en1991`, `🏛️en1992`, `🔩️en1993`, `🧩️en1994`, `🪵️en1995`, `🪨️en1996`, `🌍️en1997`, `🫨️en1998`, `🪶️en1999`. Each: status, covered clauses, gaps, unverified/NA-DE values, 3D views, diagrams, handles, worked examples on the shipped examples.

## Honesty rules applied

- Every page opens with the status that nothing was compiled/run as a whole (EN 1993 clause library: standalone tests only; EN 1999: Python oracle only) and that the NDP registry has 0 of 520 verified.
- Reference numbers are labelled specification/oracle expectations, not results.
- Gaps (not wired, missing model fields, missing edit rules, unimplemented insert tool and inline dimension entry, `setFields` not wired) are stated, not hidden.

## Open points for the maintainers

- Artifact-level `README.md` files are new in the taxonomy; confirm no taxonomy gate rejects them.
- Shortcut table follows `⌨️focus-order` (Alt+1..5, F6, Alt+arrows pan, Ctrl+arrows nudge) and `⌨️keyboard` (plain arrows nudge when a handle has focus); the two overlap in Shift/Alt meaning, so the in-app help (Shift+?) is declared authoritative.
- The worked examples describe flows over shipped examples; re-verify handle names and effects after the first green build.
- The shared guide links the NDP worklist inside the ticket; move the link when the worklist is promoted.
