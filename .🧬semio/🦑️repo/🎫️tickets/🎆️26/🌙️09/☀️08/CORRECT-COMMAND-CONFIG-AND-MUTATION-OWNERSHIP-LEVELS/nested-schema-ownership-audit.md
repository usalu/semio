# Nested Schema Ownership Audit

The previous ownership scan read top-level app configuration and command/mutation identities. It missed declarations inside job events, replacement payloads, and presence contracts.

The schema traversal now follows object properties, schema definitions, array item schemas, composition branches, and conditional result schemas. It does not interpret example/default/constant values as declarations. Explicit artifact-owned schemas remain exempt. Four language-independent fixtures compare the traversal with strict Ajv validation; the first run failed on a nested job event's locale field and the implementation then passed all four cases.

The expanded report initially exposed 21 declarations. The plugin implementation lane removed nested locale payload fields and Shooting's stale replacement-config utility field. The next report contains 15 presence utility declarations across Procedural Generation 3D, Shooting, Process 3D, Lowpoly, CAD, Remodel, Draw, Raster, Note, and Puzzle 2D/3D/5D. Presence producers, peer consumers, schema representations, retirement behavior, and fixtures must be traced before choosing removal or shared host-window projection. None of these report runs establishes a clean enforcement gate.

Generated evidence is held under the ticket's generated directory until cleanup. Durable validation results will be added here when presence review and enforcement complete.

## Current Enforcement Result

The isolated Nx `abstraction-ownership-validation:enforce` run completed successfully in 10.4 seconds with zero misplaced declarations. It checked 10 ownership vectors, 4 nested-schema vectors, 3 command-source vectors against Ajv, and 102 artifact/diff contracts across the five schema formats. This closes the remaining orphan utility enum findings. Native compile/runtime checks and the semantic window-state audit remain separate requirements.
