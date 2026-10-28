# Mixed Form Audit

The version 7 language-neutral schema and fixture replace all former three-box assumptions. The example uses (L,B,H,Form): (2.5,1.5,2,box), (2.5,1.5,2,pyramid), (3.5,1.5,2,elliptical-cylinder). Dimensions are positive real values; Form is an explicit category.

The TypeScript verifier passes against the frozen English and German sources. AJV rejects mutated mixed coordinates, categories, domains, transitions and the inconsistent direct change. mathjs independently subtracts and sums numeric vectors; category endpoints are compared and composed without arithmetic. Omitting Form collides points 1/2. Three binary flags with exactly one active need two flips for a category replacement. The correct cycle sums to (0,0,0) and returns to box; the deliberately wrong direct +2 L gives (-1,0,0). Existing D3, source icon hashes, SVG/PDF provenance, matrix and K16 implementation checks remain intact.

The final compiled German PDF passes all three independent PDF verifiers:
- Mixed: four filled parameter rows; exact numeric cells; categorical word cells; three distinct native vector families; independent projected widths/heights for box, pyramid and skew elliptical cylinder; K3; three localized transition entries; numeric cycle and one-hot constraint.
- Matrix: two 9×9 grids on page 2; 6.14 mm writing rows; icon insets 2.13–2.22 mm; maximum centering error 0.031 mm.
- K16: 16 distinct canonical vector nodes; 120 unique edges; all 16 ring edges and four prescribed middle chords; 20 matching blank ledger lines; 170.001×83.602 mm graph; vertical radius 32 mm.

Content command:
```powershell
repo tools exec --light bun -- -e "import {verifyParametrizationAssignmentContentV1} from './🧰️framework/🛍️products/📓️print/🧪️tests/📝️parametrization-assignment/🟦️.ts';verifyParametrizationAssignmentContentV1();"
```

PDF command pattern (repeat for verify-matrix-pdf.py and verify-k16-pdf.py):
```powershell
$env:SEMIO_TOOL_PYTHON='C:/Users/Ueli/.cache/codex-runtimes/codex-primary-runtime/dependencies/python/python.exe'
$env:PYTHONIOENCODING='utf-8'
repo tools exec --light python -- '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️26/ADD-PARAMETRIZATION-ASSIGNMENT-CLASS/verify-mixed-pdf.py' '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️26/ADD-PARAMETRIZATION-ASSIGNMENT-CLASS/🗑️generated/compiled/parametrization-de.pdf'
```

Changed production tests: 🧬️cases.schema.json, 🧫️cases.json, 🟦️.ts and 🥒️.feature. Added durable ticket inputs: mixed-test-update.cjs, mixed-pdf-inspect.py and verify-mixed-pdf.py. No production document edits were made by this audit agent.
