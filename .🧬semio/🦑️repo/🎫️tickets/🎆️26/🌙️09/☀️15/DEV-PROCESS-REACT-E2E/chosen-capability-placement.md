# Chosen Capability Placement

An available Catalogue row adds its installed capability immediately when the user activates the label. Its localized **Place capability** row action uses the same machine and capability IDs and arms the Workpiece for a positioned placement. The Cut, Drill and Attach toolbar utilities retain their generic-by-kind selection.

The action uses the dispatcher’s concrete Workpiece address and the existing Workpiece window transient owner. The arm and pointer position belong to the same local window root. The window owner resets on document replacement. React routes panel actions through the active window, and WGPU resolves its panel alias to the concrete Workpiece before guest dispatch. The internal `placeCapability` utility is selected by the arm effect and has no separate toolbar entry. Window instances can arm different capabilities from the same workshop without overwriting each other.

The preview and commit share the positioned measure resolver. It looks up the authored workshop’s exact machine/capability pair, validates the current stock and requires valid tool dimensions. Moving the pointer changes only the wireframe. One click commits one world transaction with the selected origin and position. Escape clears the calling window’s arm and wireframe. If a stock edit invalidates the recipe, the preview disappears and the existing armed session still admits a click to disarm without creating a step.

The mounted Workpiece regression uses the neutral fixture to arm Robotic Mill in one instance and the same machine’s Bore in a second instance. It checks independent wireframes, abort and a delayed click, the Mill's saved pose and origin, and stock-limit invalidation. The Catalogue regression checks that availability gates both actions and that activation retains the identical argument map. The retained route law verifies all 34 routes, including the timeline drop’s Artifact lane and the placement arm’s WindowTransient lane. The Three.js oracle reproduces the Mill’s wireframe extent, segment count and reach decision from the published descriptor. Ajv validates the placement state shape.

| Native test unit | Files | Active lines | Bytes |
| --- | ---: | ---: | ---: |
| Editor library | 19 | 3,778 | 255,066 |
| Workpiece integration | 5 | 744 | 51,338 |
| Catalogue integration | 3 | 626 | 38,907 |

The repository source reader measured each manifest-declared entry with `unix,test`. All fit the limits of 32 files, 4,000 active lines and 262,144 bytes. The independent route and placement oracles and scoped whitespace checks passed. The mounted Rust and browser tests were left to the active parent verification run.

[Three.js and Ajv oracle](chosen-placement-oracle.ts)
