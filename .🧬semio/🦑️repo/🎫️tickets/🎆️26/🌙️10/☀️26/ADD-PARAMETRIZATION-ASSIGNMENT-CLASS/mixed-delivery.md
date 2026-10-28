# Mixed Continuous and Categorical Example

The concise assignment now uses one box, one rectangular pyramid and one elliptical cylinder in both its parameter table and native vector K3. The four parameter rows are length, width, height and Form. Dimensions can take any positive real values; Form is categorical.

The samples are (2.5,1.5,2,box), (2.5,1.5,2,pyramid) and (3.5,1.5,2,elliptical cylinder). The first transition changes only Form; the other two change length by +1 and Form. Category changes use directed names, not numeric subtraction. A cycle has zero numeric totals and returns to its starting category.

The three short failure cases remain independent: omitting Form conflates the first two shapes; an incorrect direct length change breaks the cycle; three Form flags, constrained to exactly one active, require two flips instead of one category change. Capsule instructions now distinguish reversing numeric signs from reversing category arrows.

The existing LaTeX print product, heading tree, three-page structure, compact centered capsule matrices and K16 with twenty selected transitions are preserved. No separate course geometry model exists: the course declaration contains metadata and PDF references only.

## Verification

- Schema-first version-seven fixture and source tests pass, including independent AJV, mathjs and D3 checks for numeric and categorical changes, collision, cycle and one-hot minimality.
- Native German light, English light and German dark compilation succeeds. The final German log has no overfull, underfull or missing-character errors.
- The new independent PDF audit verifies all four parameter rows and values, categorical names, the exactly-one-active condition, K3 and distinct native box, pyramid and cylinder primitives. Projected dimensions are checked independently, including the skew ellipse extent.
- Existing PDF audits pass for both 9-by-9 matrices and the full-width K16, all 120 base edges, sixteen ring edges, four chords and twenty matching blank entries.
- All three final German pages were rendered with PDFium and visually reviewed. English and dark versions were independently reviewed.

## Final Artifact

- output/pdf/Fingeruebung-Parametrisierung.pdf
- Three A4 pages, 115165 bytes
- SHA-256: 518453bdb8c32e8781e27e4c3c24d3037d6c979208567ab3b3c0fd2af5a63743

Authored scripts, fixtures and reports remain in this ticket. Compiler intermediates, temporary PDFs and review images are removed after verification. Concurrent repository changes, shared product chrome and original capsule icons are preserved.
