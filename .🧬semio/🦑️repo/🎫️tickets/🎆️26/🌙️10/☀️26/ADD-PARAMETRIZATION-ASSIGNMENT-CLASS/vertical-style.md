# Filled Matrix Vertical Alignment

The writing strut reserves equal height and depth around the table baseline. Filled labels and values previously shared that baseline, placing their visible glyph center above the row center. The correction must center each text box using its own measured height and depth while keeping its contribution to row dimensions zero. The unchanged symmetric strut continues to own row geometry. Header cells and empty exercise rows keep their existing builders.

## Test First

`vertical-style.case.json` records language-neutral filled label, filled value, blank row, and header invariants. `vertical-style.verify.cjs` checks the shared filled-cell helper and unchanged strut. Independent PDF measurements are coordinated with the audit agent; native DE compilation is owned by the root agent.

## Shared Style Correction

The test-first source contract failed before the fix and passes all four cases after it. The shared `semio_assignment_matrix_text:n` helper now pairs the unchanged symmetric writing strut with a zero-extent raised text box, shifted by half the difference between its measured depth and height. Filled labels and values both use the helper. No hardcoded text offsets, content edits, header changes, blank-row changes, or global helper changes were needed. Source is frozen for native verification.
