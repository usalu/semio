# Same Table Geometry

The worked example now uses the task matrix dimensions: a 16 mm header, 6 mm rows, 2 mm cell padding, and a 27 mm parameter column. Its three form columns retain four populated rows and add four empty rows. Header forms share a scale of 0.5, preserving their common geometric axes and proportions; printed header numbers are removed.

The shared matrix package exposes expandable `AssignmentParameterMatrixBlankRows`. Both default blank matrices and the worked example use the same blank-row implementation. The existing seven-argument worked table API and all graph geometry remain unchanged.

## Verification

A language-neutral fixture (`same-table-geometry.case.json`) and source conformance check were authored before editing. The initial check failed on the four-row example. After the changes it passes all dimension, scale, populated-value, and shared-helper assertions. Native Tectonic compilation and PDF visual verification are coordinated by the parent and bilingual review agents.
