# Procedural Semantic Repair

Each choice below follows the actual language-neutral mutation payload, reviewed before moving the path. Geometry dimension suites use `🌀️mutate-procedural-2d-1`, `🧊️mutate-procedural-3d-1` and `🧩️mutate-assembly-1`. The assembly compatibility-rule creator will use `🚦️create-rule`, reflecting allowed/forbidden adjacency rather than a measuring ruler. Existing meaningful command and operation identities are otherwise retained.

| Artifact / operation | Old scenario | Handpicked scenario | Evidence |
| --- | --- | --- | --- |
| 🧩️assembly / 🔢️change-weight | 🔌️raises-the-wall-module-selection-bias | ⚖️raises-the-wall-module-selection-bias | Raises the module selection weight. |
| 🧩️assembly / ✂️disconnect-slots | 🤖️severs-edge-ab-leaving-both-slots | ✂️severs-edge-ab-leaving-both-slots | Cuts only the connecting edge. |
| 🧩️assembly / 🔗️connect-slots | 🚪️joins-slot-b-to-slot-c-at-index-1 | 🔗️joins-slot-b-to-slot-c-at-index-1 | Connects the two assembly slots. |
| 🧩️assembly / 📏️create-rule | 🦅️appends-a-rule-forbidding-roof-over-wall | ⛔️appends-a-rule-forbidding-roof-over-wall | Creates a forbidden adjacency rule. |
| 🧩️assembly / 🧩️create-slot | 🚪️appends-slot-c-at-index-2 | 🧩️appends-slot-c-at-index-2 | Creates an assembly slot. |
| 🧩️assembly / 🎲️change-seed | 🌴️reseeds-the-solve-from-7-to-99 | 🎲️reseeds-the-solve-from-7-to-99 | Changes the random seed. |
| 🧩️assembly / 🪶️remove-weight | 🔌️drops-the-wall-module-weight-override | 🪶️drops-the-wall-module-weight-override | Removes a selection weight override. |
| 🧊️generation3d / 🔤️change-schema | 🧫️restamps-the-fixture-schema-id | 🏷️restamps-the-fixture-schema-id | Replaces the schema identifier string. |
| 🧊️generation3d / ✂️disconnect-synapse | 🤖️cuts-wire-ab-leaving-both-nodes | ✂️cuts-wire-ab-leaving-both-nodes | Disconnects the wire. |
| 🧊️generation3d / 🩹update-widget | 🎞️retunes-the-knob-slider-value | 🎚️retunes-the-knob-slider-value | Changes the input slider value. |
| 🧊️generation3d / 🔗️connect-synapse | 🚪️wires-node-b-to-node-c-at-index-1 | 🔌️wires-node-b-to-node-c-at-index-1 | Connects the declared graph ports. |
| 🧊️generation3d / 📍️move-widget | 🌳️repositions-node-a-in-the-graph | 📍️repositions-node-a-in-the-graph | Changes graph layout coordinates. |
| 🧊️generation3d / 🏷️rename-generation | 🌱️retitles-generation-1-via-new-name | 🏷️retitles-generation-1-via-new-name | Renames an existing generation. |
| 🧊️generation3d / 📷️update-camera | 🌳️frames-the-graph-at-double-zoom | 🔍️frames-the-graph-at-double-zoom | Changes graph camera zoom to two. |
| 🧊️generation3d / ➕create-generation | 🎨️appends-generation-2-and-moves-the-selection | 🌱️appends-generation-2-and-moves-the-selection | Creates and selects a new generation. |
| 🧊️generation3d / 🔧️change-generation-value | 🍎️raises-the-storeys-answer-in-generation-1 | 🏢️raises-the-storeys-answer-in-generation-1 | Changes the storey-count answer. |
| 🧊️generation3d / 🌱️create-widget | 🚪️inserts-node-c-at-index-2 | 📝️inserts-node-c-at-index-2 | Inserts an inputNote widget. |
| 🧊️generation3d / 🧹️delete-widget-position | 🌵️unpins-the-node-a-position | 🧹️unpins-the-node-a-position | Clears an explicit node position. |
| 🌀️generation2d / 🔤️change-schema | 🧫️restamps-the-fixture-schema | 🏷️restamps-the-fixture-schema | Replaces the schema identifier string. |
| 🌀️generation2d / ✂️disconnect-synapse | 🤖️severs-link-ab-leaving-both-notes | ✂️severs-link-ab-leaving-both-notes | Disconnects the link. |
| 🌀️generation2d / 🔗️connect-synapse | 🚪️joins-note-b-to-note-c-at-index-1 | 🔗️joins-note-b-to-note-c-at-index-1 | Connects two note nodes. |
| 🌀️generation2d / 🎛️set-camera | 🌳️pans-and-zooms-the-graph-camera | 📷️pans-and-zooms-the-graph-camera | Changes graph camera transform. |
| 🌀️generation2d / 📍️move-widget | 🦁️repositions-note-a-on-the-canvas | 📍️repositions-note-a-on-the-canvas | Changes canvas coordinates. |
| 🌀️generation2d / 🏷️rename-generation | 🦁️retitles-generation-1 | 🏷️retitles-generation-1 | Renames the generation. |
| 🌀️generation2d / 🔢️change-generation-value | 🐸️raises-the-height-answer-in-generation-1 | 📏️raises-the-height-answer-in-generation-1 | Changes the height answer. |
| 🌀️generation2d / 🧹clear-widget-layout | 🚪️drops-the-note-a-layout-entry | 🧹️drops-the-note-a-layout-entry | Clears the note layout. |
| 🌀️generation2d / ➕create-generation | 🟪️appends-generation-2-and-selects-it | 🌱️appends-generation-2-and-selects-it | Creates and selects a new generation. |
| 🌀️generation2d / 🔁️replace-widget | 🛟️rewrites-the-note-b-body-in-place | ✍️rewrites-the-note-b-body-in-place | Rewrites note text. |
| 🌀️generation2d / 🌱️create-widget | 🚪️inserts-note-c-at-index-2 | 📝️inserts-note-c-at-index-2 | Inserts an inputNote widget. |

## Verification

The 29 scenario moves preserve all 145 JSON payloads byte for byte, verified against the captured SHA-256 of sorted relative-path/hash pairs for each scenario. Another 27 captured files (eight example documents, one identity fixture, fourteen retained window configuration placeholders and four PDF adapter files) retain their hashes. The example carriers now mirror their actual shapes: shell, sweep, mushroom column, torus subtraction, solid fusion, wire, extruded volume and fillet. PDF adapters align with the reviewed `📖️pdf` publication owner. Window `⚙️config` is explicitly registered separately from sibling `🎚️options`; an intermediate configuration/options collision was detected and undone before the final scoped audit.

The final scoped audit covers 1,143 files, 945 directories and 2,080 governed entries, with zero findings across all eight categories. Exact scenario names and assembly operation metadata are reconciled in all three mutation catalogs; one stale 2D delete-generation scenario ID is aligned with its existing physical case. Seventy-four Gherkin vector-table rows now use the real case coordinates. Production parsing reports zero errors and resolves all 185 fixture URIs. The actual registered independent Python adapters pass 19 assembly, 29 generation3d and 29 generation2d scenarios (77 total). This is independent specification-vector verification, not a claim that the Rust subject ran.

Three over-shallow shared-law mounts are repaired. The JavaScript example runner previously only printed a success message; its existing Nx target now executes the eleven committed example tests. The first uncached run failed on all eleven nonexistent `artifact.ts` imports. The imports now use the existing Bun test runner and the eight renamed shape carriers use their exact paths. The fresh uncached target passes all eleven tests and assertions in 270 ms. The fresh bounded native Nx run after shared PDF references settled exited at its 1,200,000 ms build budget while waiting for the shared Cargo build-directory lock. No native assertion ran; no native pass is claimed, and other sessions' processes were not interrupted. A subsequent hash recheck still passes all 145 scenario JSONs and 27 carrier files.
