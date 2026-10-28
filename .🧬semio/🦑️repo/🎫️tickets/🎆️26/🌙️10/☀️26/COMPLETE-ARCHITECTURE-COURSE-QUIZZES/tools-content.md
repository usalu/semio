# Excel and Grasshopper Content Basis

Language-neutral expected cases were authored before quiz content in the course fixture folders. The command-output and tree/list examples are original teaching cases, with explicit fixed inputs and outputs rather than screenshots from a particular software version.

## Verified Sources

- Microsoft, [Overview of formulas](https://support.microsoft.com/en-us/excel/get-started/overview-of-formulas-in-excel): A1 notation, formulas and references.
- Microsoft, [Relative absolute and mixed references](https://support.microsoft.com/en-us/excel/switch-between-relative-absolute-and-mixed-references): independent row/column offsets on copying. The source prose contains an apparent direction typo; use its reference table and coordinate arithmetic.
- McNeel, [Grasshopper data trees and Python](https://developer.rhino3d.com/en/guides/rhinopython/grasshopper-datatrees-and-python/): paths address lists and item indices are separate from path indices.
- McNeel, [Advanced data structures](https://developer.rhino3d.com/guides/grasshopper/gh-algorithms-and-data-structures/advanced-data-structures/): branch structure changes matching; flatten and graft examples.
- McNeel, [Data Modifiers](https://www.rhino3d.com/docs/grasshopper2/topics/DataModifiers.html): flatten merges lists, graft isolates items, simplify removes shared path indices without merging branches. This definition is used only for unambiguous multi-branch leading-prefix cases common to Grasshopper 1 and 2.
- Grasshopper, [Data Matching](https://www.grasshopper3d.com/page/data-matching): distinguish branch matching from list item matching; list exercises explicitly constrain each input to one branch and state longest repeat-last and shortest trim-end conventions.

## Authoring Decisions

Excel formulas use English function names as a language-neutral notation; German explanations provide translated function names where helpful. Amounts are unitless teaching values. Formatting changes display while retaining values; Paste Values removes the copied formula. Sort examples sort the complete two-column table to preserve associations, while filters hide rows without reordering or deleting data.

Grasshopper typing exercises avoid ambiguous numeric rounding, complex geometry conversions and locale-dependent decimal parsing. Tree exercises use no empty branches or mixed-depth paths. Matching examples specify pairs, avoiding dependence on the order of a Cartesian product. Every semantic pool slot has five distinct variants and a 300-second task budget.

## Completed Content and Verification

Excel now contains three tasks and 110 authored items: workbook terminology (8 semantic slots), command-result matching (10 slots), and copied-reference behavior (4 slots). All use Excel mockups. Command alternatives vary numeric inputs, row order, filter thresholds and formulas where meaningful; each expected output remains compatible with its result category.

Grasshopper contains four tasks and 85 items: canvas terminology (9 slots), direct casting (2 slots), tree transformations (3 slots), and list matching (3 slots). All use node/port/wire/tree mockups. The plugin examples explicitly describe fictional teaching packages, avoiding a requirement to install any plugin. Tree transformation labels do not name the answer.

Additional verified source: McNeel [Algorithms and Data §1.6](https://developer.rhino3d.com/en/guides/grasshopper/gh-algorithms-and-data-structures/algorithms-data/) explicitly describes typed parameter casting and nonnumeric text failure. McNeel [vector/point conversion discussion](https://discourse.mcneel.com/t/closest-vector-how-can-i-script-this-in-grasshopper-itself/103598) confirms coordinate-preserving conversion.

Both language-neutral fixtures contain independent baseline answer cases authored before content, plus complete classification maps for curriculum integration. The separate concrete cases cover Excel formulas/copy offsets/table operations and Grasshopper exact tree/pair outputs.

Command run through repo: `repo tools exec bun -- <ticket>/verify.ts excel-workbook grasshopper-data`. Passed after correcting one English short label from a gerund to a noun phrase; rerun after strengthening command variants also passed. Verification reported 40 challenge/rotation runs with independent AJV and owned domain agreement, all pool alternatives at least five, all short labels <=40, unique item icon identities and perfect-score/no-solution checks.

Files created:
- 🎓️teaching/🏛️architecture/📊️excel/❓️quiz/🔣️.json
- 🎓️teaching/🏛️architecture/🦗️grasshopper/❓️quiz/🔣️.json
- 🎓️teaching/🏛️architecture/📚️course/🧫️fixtures/📊️excel/🔣️.json
- 🎓️teaching/🏛️architecture/📚️course/🧫️fixtures/🦗️grasshopper/🔣️.json
- This ticket report.

No generated outputs were left by this content-writing subtask. Central integration, repository targets and ticket closure are owned by the parent agent.

## Visual Fidelity Follow-up

Following parent visual review, every Grasshopper item now specifies optional widget presentation. Vocabulary renders typed capsules, complete operation ports, bounded numeric sliders, populated localized yellow panels and plugin package headers. Casting sources and destinations use typed capsules; list inputs use populated panels. Operation ports match Addition A/B→R, Multiplication A/B→R, Construct Point X/Y/Z→Pt, Move G/T→G and Circle P/R→C. Port highlights reference the corrected port ids. Mockup widget schema/render support is owned by implement_mockups; content revalidation follows that integration.

Revalidation after widget schema integration passed: all 195 items, seven tasks and 40 challenge/rotation runs with domain/AJV agreement.

## Excel Active-cell Fidelity Regression

Visual QA identified formula bars incorrectly associated with selected input B2. Added independent language-neutral presentation vectors before changing content, then reproduced the defect: formula-product active B2 failed the required D2 binding.

All 110 Excel mockups were reviewed:
- Ten formula/reference terminology examples now select output D2 and show the correct computed output: product24, total60, average20, difference18, ratio1.5, B7-reference28, B2-reference24, C3-reference6, range-reference60 and fixed-reference24.
- The B7 reference example now includes visible B7=14, supporting its D2=28 result.
- Seventy-five ordinary terminology or command-input mockups now show the selected top-left cell's actual content in the formula bar. Calculation commands remain in the exercise caption rather than appearing as formulas in their own input cells.
- Five Paste Values examples already select C1, whose displayed24 agrees with its formula. These are preserved.
- Twenty copied-reference examples explicitly describe formula display mode, so source C2 visibly contains its formula. The parent correction to copy-absolute-4 referencing D3 is preserved.
- German Grasshopper widget edits were untouched.

Durable regression coverage was appended to the existing curriculum suite: exact independent formula/source/result binding, absence of direct circular source references (including expanded ranges), all110active-cell/formula-bar associations, explicit copy display mode, and Paste Values association. New fixture.presentation contains ten terminology formula vectors and five Paste Values vectors. No schema expansion.

Verification: ticket verify-excel-presentation.ts failed before implementation with B2 !== D2 and passed afterward. The standard ticket verifier also passed all195items, seven tasks and40challenge/rotation runs with AJV/domain agreement. The mockup agent was asked to rerender the corrected Excel preview.

Additional changed/input files: existing curriculum test; ticket verify-excel-presentation.ts. No generated output artifacts were retained.
