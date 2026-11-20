# FEM 3D browser verification audit

This is a source audit, not a claim that the browser or solver passed. Use the live playground on port 6087 after the ticket's activation/serve route; Storybook's 3D hosts use a local reducer and omit the solver, so they cannot prove runtime dispatch or analysis.

The editor manifest (`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/📋️manifest/🦀️.rs:102`) declares `setActiveExample` with options Concrete Forest (`concrete-forest`, default), Default (`demo`), and House (`house`). Invoke the action and submit each choice; wait for the document replacement, model scene and completed results rather than accepting a queued message. The play-app decodes `exampleId` (`🖥️play-app/🦀️.rs:387`); the handler publishes a `LoadDocument` effect (`🎮️commands/📚️set-active-example/🦀️.rs:23`). Test the actual action UI and resulting scene, not merely selection in the form. Switch back to Concrete Forest to detect stale document/visual state. The native pane smoke also dispatches AddNode followed by undo and redo (`🧪️tests/🪧️pane-smoke/🦀️.rs`).

| Example | Model evidence | Expected static result sources / checks |
| --- | --- | --- |
| Concrete Forest | 20 nodes, 20 frame members, two fixed bases, two stacked fan structures, no solids. | `dead`, `live`, `uls`, `sls`. SLS heads and cantilevers sag; peak vertical deflection is between 0.0001 and 0.05 m; equilibrium and residual are checked at 1e-6. |
| Default / demo | Fixture has 16 nodes, 16 frames, one slab solid, eight supports; 8 × 10 × 5.6 m frame plus separate 2 × 2 m slab. | `dead`, `live`, `uls`; modal and buckling counts are 3. |
| House | Eight solids: raft, two slabs, four walls, pitched roof; no line elements or sections. Model should visibly change to a house, subject to solid rendering support. | `dead`, `live`, `snow`, `uls`, `sls`. SLS attic and snow-loaded roof sag; peak SLS vertical deflection 0.00001–0.05 m, residual < 1e-6. |

Evidence: `📚️examples/🌲️concrete-forest/🧪️tests/🧩️example/🦀️.rs` and `📚️examples/🏠️house/🧪️tests/🧩️example/🦀️.rs` contain topology and solver assertions; `🖼️assets/🎬️demo/🗣️.dsl.semio` defines demo. These numerical bounds are solver-test evidence, not necessarily text shown by the UI. The results window maps absent source to the first load case and validates qualified `loadCase` or `combination` identities (`✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🦀️.rs:34`); select different cases and verify the completed visual changes. A running analysis may retain the previous completed visual while a replacement runs, so waiting and inspecting the actual selected source is essential.

Storybook's `🎭️3d-model/🧪️.story.tsx` provides `fem3d-load-default`, `fem3d-add-node`, and debug state, and `🎭️3d-results/🧪️.story.tsx` provides mode/case controls and debug state, but their reducer is illustrative, not the mounted application. Its example assumptions can be stale; do not use it as proof of the three runtime choices.
