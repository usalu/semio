# Owned Pure Coordinate Interfaces R57

Root assigned the concrete return-type findings in pure-interface-audit-r57.md. Public P now returns a three-float tuple; wall_frame returns three builtin float-coordinate tuples and a builtin float length; UP is a float tuple. The owned Prim constructor defines all supported face payloads (poly, cylinder, line and segments) using builtin containers/numbers and normalizes its foot coordinates. NumPy stays private to calculations. No shared energy/font framework or source JSON was changed.

Renderer vector math now explicitly converts builtin coordinates with np.asarray at use sites. Rendering producer imports bind the same owned functions/types. Axon _lift converts polygon rings and line points before array indexing. No compatibility wrapper or external-type alias was introduced. Tests that previously extracted a fake prism constructor now call the actual pure production assembly.

## TDD and Actual Values

The portable native_containers cases were added to axon/📏️geometry.json before the correction. The new actual-return test failed RED because ndarray P equality had ambiguous truth. After correction, all24 focused tests pass: three actual axon contracts, five independent geometry/interface contracts, one material role and15 frustum/stair regressions. Actual347-test hook collection is clean, without Manim imported. Full final feature execution is still a separate parent-controlled gate.

Every face and foot in all12,094 actual cached R44 primitives passes recursive builtin-instance validation. Complete before/after primitive rows are exactly numerically equal. Raw JSON hash changes47d7fc5423191b885093846799472a20cf043e3d3bd767c0bededb8d38b96491 →125139cc572b1860faa9406fe4b7fe1f0d6ba9d4ccfca751013a388bddd668ee because source integer coordinates such as19 are now consistently float19.0. Root explicitly approved this format normalization rather than integer special cases. Canonical number-normalized before and after hashes are identically55da25a7cc71829d0e13b96eb33fddeb9032b8b78c0a204d662b78fccb31e970. No geometry variation or numerical tolerance was introduced.

Generated pure-interface-r57.json records actual P/wall_frame/UP/Prim samples and the metrics. Independent R57 auditor was notified with those paths. R11 was notified to use the actual tuple-returning P in callback preparation so it cannot mask vector arithmetic regressions. Native frames and typography remain separate gates.

## Exact Production/Test File Scope

- 🎨️visuals/📐️drafting/📏️geometry.py
- 🎨️visuals/🏛️savoye/🧱️core/📐️geometry.py
- 🎨️visuals/🏛️savoye/🧊️axon/🧩️primitives.py
- 🎨️visuals/📐️drafting/🐍️.py
- 🎨️visuals/🏛️savoye/🧊️axon/🐍️.py
- 🎨️visuals/🏛️savoye/📏️sheet/🐍️.py
- 🎨️visuals/🏛️savoye/✂️section/🐍️.py
- 🎨️visuals/🏛️savoye/🔩️detail/🐍️.py
- 🎨️visuals/🧮️analysis/🐍️.py
- 👁️representations/🐍️.py
- 🧩️models/🐍️.py
- 🔌️exchange/🐍️.py
- 🎨️visuals/🏛️savoye/🧊️axon/📏️geometry.py
- 🎨️visuals/🏛️savoye/🧊️axon/📏️geometry.json
- 🎨️visuals/🏛️savoye/🧪️ortho-remodel/test_frustums.py
- 🧱️model/🧪️tests/🛝️ramps.py

Files are frozen pending final cohort. Temporary Windows open-for-write UNKNOWN errors were resolved with scoped r+ handles that close in finally; no shell fallback was used.
