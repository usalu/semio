# Independent Print Audit

## Actionable Finding

P2: 🧰️framework/🛍️products/📓️print/🧾️template/📝️assignment/🟦️.ts:22 looks up properties through the Object prototype. A declaration with valid language, theme and title plus own toString string is accepted by isAssignment even though additionalProperties=false rejects it. Confirmed by repo tools exec --light bun: isAssignment({language: en, theme: light, title: X, toString: non-schema string}) returned true. Require Object.hasOwn(schema.properties,key) before reading the property and add a toString or constructor fixture adjudicated by Ajv.

## Verified Structure

The worked dimensions A=(2,1,1), B=(3,1,1), C=(3,2,1) match the drawn box calls (geometry lines 123–124), the bilingual prose and fixture. Dropping width makes B and C identical; a wrong AC change gives the stated cycle sum; the invertible coupled coordinates need two changes for AB while ordinary dimensions need one. Mathjs and the own implementation agree.

Both source documents explicitly select language (semio.cls lines 21–28 rejects missing and unsupported assignment languages). Each has two table calls, two graph calls and two ledgers. Parameter table headers have Parameter only in the first cell; its five generated header cells and row cells are empty. Eight shape rows alternate capsule/cylinder, without row names or numbers. Sixteen geometry tuples are distinct. Rounded silhouettes have hemispherical ends; cylinder silhouettes have elliptical flat-end projections, so they differ at equal length/diameter.

The shared 28-pair sequence contains each unordered edge exactly once and is reused by graph and ledger. Graph vertices reuse the table shape sequence. Ledger has two 14-entry columns with 67mm writing lines and 6.3mm vertical spacing. Graph diameter is 64mm plus node labels. Table height is approximately 122mm plus rule thickness, within 257mm page content height; graph plus ledger uses approximately 165–175mm before prose. Five authored page sections are plausible, but actual pagination and overflow remain unverified without native PDF compilation.

Catalog adds both bilingual documents with all three source files per document. Merge-contract fixture includes both ids. Quick pipeline imports and runs verifyParametrizationAssignmentContentV1. The schema declares explicit language/theme/title, disallows extra keys and has correct field types. English/German content is parallel.

## Validation

- repo tools exec --light bun -- test ./🧰️framework/🛍️products/📓️print/🧪️tests/📝️assignment/🟦️.ts: 11 passed, 0 failed, 26 assertions.
- Imported and ran verifyParametrizationAssignmentContentV1 through repo tools exec --light bun: passed.
- No compiler invoked and no implementation files changed.

## Limits

Source contracts test tuples and pairs rather than rendered geometry. The language-neutral feature claims a fourteen-row two-column ledger but the exercise verifier does not currently assert ledger constants, just reuse of the pair macro. This is test-strength advice rather than a visible product defect. Actual PDF rendering remains the required confirmation for TeX expansion, font handling, glyphs and layout.

## Targeted TeX Prerequisite Reaudit

The root review caught the undefined semio-chrome-text-muted color missed in the initial source audit. Geometry now uses semio-chrome-foreground!65!semio-chrome-canvas for muted edges and semio-chrome-foreground for primary shapes. Both names are defined in semio-core.sty lines 61/67 and 73/79 for dark/light themes. This correction removes the identified undefined-color failure.

Load-order prerequisites verified through semio.cls: semio-fonts defines SemioSans at line 10; semio-core declares subtitle storage at line 18 and subtitle command at line 119; semio-tokens loads xcolor and defines stroke hairline; semio-table loads array and colortbl (arrayrulecolor); semio-window loads graphicx (scalebox) and tikz; semio-viz also loads tikz. All are loaded before the template geometry is used. The assignment .sty is therefore self-consistent as a semio document-kind package. Its use outside semio.cls is not documented or assessed as a standalone package.

Expl3 storage, conditionals, keys, command/environment definitions and dimension expressions inspected. Assignment table subtracts the shape column, twelve tabcolsep widths and seven rules for the six-column default, then divides the remainder across five value columns. With A4/20mm margins this gives approximately 23.2mm per blank column and a total width of 170mm. All table dimensions fit the authored 21mm maximum silhouette length and 9mm maximum height. Row height uses a 14mm strut with 4.2mm depth; shape strokes may enlarge the row slightly but do not invalidate the page estimate. Grouped options reset at environment exit. Repeated tabs and row-ending commands are inside the captured table body as intended. No further definite TeX syntax/prerequisite defect found in static inspection.

This is a static conclusion only. A native PDF build is still necessary to confirm actual TeX expansion, glyph fallback, inherited page chrome, table/graph baseline behavior and pagination. The initial audit should not be treated as render verification.
