# Concise Assignment Content

Both language variants now use one numbered section and two subsections: the worked example and dwelling capsules. Front matter, subtitle, metadata banner, extra section headings and reflection writing lines were removed. Course footer and header setup remain.

Page 1 uses the shared three-column box matrix, K3 and worked change ledger. Length, width and height provide the valid model. Each criterion retains one short isolated counterexample: omitted width for uniqueness, an incorrect direct transition for consistency, and invertible coupled coordinates for minimality.

Page 2 retains both nine-by-nine exercise tables and their writing space. Page 3 retains the K16 and twenty selected changes, with only short numbering, direction and cycle instructions.

## Verification

- \`check-concise-content.cjs\` passes in both languages: exactly one section, two subsections, two page breaks, both exercise tables, each worked/example macro once, and no title front matter.
- The same check confirms the box values, zero cycle, incorrect cycle sum \`(0,-1,0)\`, inverse coupled coordinates, and two versus one changed coordinates.
- Native English Tectonic compilation passes after the shared matrix row expansion repair. The PDF contains exactly three pages.
- The independent pypdfium2 check passes for the rendered heading hierarchy, both exercise tables, twenty selected pairs, ring/chord instructions and directed changes. Extracted page text lengths are 696, 310 and 535 characters.
- All three rendered English pages were visually inspected together. The example matrix and K3 are aligned, both capsule tables fit on page 2, and the oval K16 plus two-column ledger fit on page 3.
- The English compiler log contains zero overfull boxes, underfull boxes or missing characters. Existing font substitution and package integration warnings remain.
- The initial PDF check assumed heading numbers preceded labels; the existing theme places them on the right. The retained assertion now checks the actual label/number order.

## Retained Files

- \`parametrization-de.content.tex\`
- \`parametrization-en.content.tex\`
- \`check-concise-content.cjs\`
- \`check-concise-en.py\`
- This report.
