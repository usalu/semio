# Mixed Scientific Mode Default And Override Adjudication — 2026-10-05

Read-only source snapshot, not actual mixed-branch GREEN. Four exact source inputs retained under authored-inputs/mixed-mode-default-independent-adjudication. Current schema has no numeric domain/range default for these four families, which avoids falsely reporting one mode's numeric window as universal. Native parent enter defaults domain/range empty; branch selects its own fallback, then applies each explicit authored bound independently. Shared reset starts numeric0..1; physical begin deliberately replaces that with0..width/height before override.

## Truthful Resolved Defaults

| Family / mode | Omitted domain | Omitted range | Source |
|---|---|---|---|
| sci-orbital / molecular-orbital | -1,1 (column coordinate) | -8,8 (energy coordinate) | chemistry359–360 |
| sci-orbital / configuration, orbital | 0,width native millimetres | 0,height native millimetres | chemistry351–352 → field778–781 |
| sci-phase-diagram / unary, binary | 0,1 normalized Cartesian coordinate | 0,1 normalized Cartesian coordinate | field751; chemistry473 |
| sci-phase-diagram / ternary | 0,width physical projected millimetres | 0,height physical projected millimetres | chemistry471 → field778–781; barycentric projection happens before canvas transform |
| sci-electrochem / pourbaix | 0,14 pH | -1.2,1.6 potential | chemistry572–573 |
| sci-electrochem / frost | species oxidation-state extent plus12% span margin | species nE° extent plus12% span margin | chemistry590–598 → field466–473; equal extents expand±1 in shared window |
| sci-electrochem / latimer | 0,width physical millimetres | 0,height physical millimetres | chemistry565 → field778–781 |
| sci-optimization / feasible-region, objective-contours, pareto | -0.5,4 data coordinate | -0.5,5 data coordinate | mathematics1616–1617 |
| sci-optimization / queueing | 0,width physical station millimetres after unit scaling | 0,height physical station millimetres after unit scaling | mathematics1627 → field778–781; parent numeric bounds are superseded |
| sci-optimization / decision-lattice | Current chrome -0.5,4 | Current chrome -0.5,5 | mathematics1616/1634–1637; body exception below |

Domain-only override must preserve range fallback and range-only override must preserve domain fallback. Explicit empty is the branch-fallback request. For physical branches bounds are already projected millimetres, not raw orbital shell indices, barycentric composition fractions or unscaled station coordinates. For numerical branches bounds are the corresponding data coordinates. Same window must govern visible ticks and actual body coordinates. Do not put one static numeric JSON `default` on a mixed option; a truthful internal default is empty string, while bilingual descriptions must state the mode-conditioned fallback and coordinate units. Current no-default metadata is acceptable as unspecified, but generic descriptions 'scale or sweep' omit these distinctions and should be clarified.

Suggested EN domain description: 'Optional x viewport min,max. Empty uses the mode default: [mode defaults from table]. Physical modes use projected millimetres; numeric modes use their data coordinates. An explicit domain replaces only the x viewport.' Range: equivalent y text. DE: 'Optionales x-Sichtfenster min,max. Leer verwendet die modusspezifische Voreinstellung: [Tabelle]. Physische Modi verwenden projizierte Millimeter, numerische Modi ihre Datenkoordinaten. Ein expliziter Bereich ersetzt nur das x-Sichtfenster.' Equivalent y wording for range. The generated API should preserve empty-as-fallback truth, and can display the same resolved fallback table without claiming every mode has the same numeric default.

## Concrete Remaining Branch Exception

Decision-lattice body mathematics1738–1754 uses width/height directly for every dot and edge, ignoring dx/dy window. Therefore current authored domain/range change chrome while body stays fixed. Truthful schema descriptions cannot certify body viewport semantics until this branch is repaired and independently tested. Two coherent choices in the existing owner: treat lattice as physical branch and wrap it in physical begin/end (document physical mm bounds), or assign semantic depth/index data coordinates and consume mx/my consistently (document numerical bounds). To preserve current uncustomized geometry, a physical scope is the minimal truthful physical contract. Do not simply retain a numerical default label while leaving the body unchanged.

Native mode declarations/comments list orbital molecular-orbital/configuration/orbital, phase unary/binary/ternary, electrochem pourbaix/frost/latimer, optimization feasible-region/objective-contours/pareto/queueing/decision-lattice. Current schema mode descriptors are identifier strings without enum and native fallback branches admit unknown modes to an unrelated default. Closed mode vocabularies require schema and native guards; fallback is not evidence an unknown mode is implemented. This is source adjudication, no new compiler execution.

| Snapshotted source | SHA-256 |
|---|---|| 🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-scientific-field.sty | 63E76C197A48A17E07DAF81DF97426E03A9B8A9FEBE861D1ECD90A27F1F1F899 |
| 🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-scientific-chemistry.sty | CC1A6C821F34BE15A3070535DCED7523949CF5A9CBB333909A55B85138F3186A |
| 🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-scientific-mathematics.sty | DA892E6DA41099A69E49A6E64CACA2E57B812D3396F2C1B876D129BD07F6C043 |
| 🧰️framework/🛍️products/📓️print/🧬️schema/🔣️.json | B55B192B9ABE852B23405EFC05C82310CEEF4C7972FEE8E10C2A0393BFE70103 |
