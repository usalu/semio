# Native134 Map Retirement and Command-Wiring Audit

## Scope and evidence

This was a source and receipt audit. I did not edit production or run a build.

Native134 records the Map failure and the four command-wiring failures in [failures.json](../🗑️generated/astra-runtime/renderer-native134-full/failures.json) and [run.log](../🗑️generated/astra-runtime/renderer-native134-full/run.log).

## Map key replacement

The Map helper is a full candidate publication, not a shortcut: [wgpu-engine-surfaces test:203](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧪️tests/🧩️wgpu-engine-surfaces/🦀️.rs:203) reconciles and paints, records visibility, registers clipped retained hits, seals, ACKs, and publishes hits. Native134 failed the immediate old-drag assertion in the key-replacement test at [line 901](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧪️tests/🧩️wgpu-engine-surfaces/🦀️.rs:901).

This sequence explains the observation:

1. Reconcile moves a candidate-removed host to accepted scene retirements, one credited owner at a time, only after the candidate no longer mounts it ([UI engine:1695](../../../../../../../../🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs:1695)).
2. ACK transfers that queue into scene retirements; it does not synchronously close a component ([UI engine:1183](../../../../../../../../🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs:1183)).
3. A following bounded document render receives one retirement and calls scene retirement ([Interpreter:2805](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:2805)). That retirement clears Map interaction ([Scenes:1367](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:1367)).

The failed predicate only reads internal old-host drag state ([Scenes:8536](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:8536)). It does not establish that old input is live. A replacement target fails exact liveness on key, component generation, host, kind, and document surface mismatch ([Interpreter:707](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:707)); native down/up and move routes filter through the Shell published-target predicate ([renderer:16651](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:16651), [renderer:16714](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:16714), [Shell:13367](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:13367)).

Conclusion: Native134 exposed an over-eager fixture assertion, not a proven production input defect. The correct law is immediate old-target non-liveness plus zero old actions, followed by bounded retirement progress and only then old drag/token removal. Root has applied this fixture correction for both replacement and removal. Do not make ACK synchronously drain internal, CPU, or GPU retirement.

Confidence: high.

## Clipboard, Ink, and Canvas fixture authority

The remaining four tests share candidate-only setup, but do not demonstrate one production dispatch defect.

- Clipboard applies a test-only tree and directly calls dispatch before it has a document, accepted pixels, or a presented router ([test:222](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🧪️tests/🔬️wgpu-ui-command-wiring/🦀️.rs:222)). It fails at its first Tab focus assertion, before clipboard readback. Production clipboard completion re-enters normal UI event dispatch after the I/O result ([Interpreter:1402](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:1402)).
- Ink cancellation and editor lifecycle use seed_scene_window_with, which only publishes and reconciles ([test:437](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🧪️tests/🔬️wgpu-ui-command-wiring/🦀️.rs:437)), then use hand-built Scene commands. Canvas replacement also mutates the candidate through apply_tree ([test:1385](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🧪️tests/🔬️wgpu-ui-command-wiring/🦀️.rs:1385)).
- Existing publish_ink_intent_document already gives the needed publish → reconcile → visible → seal → ACK fixture route ([test:604](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🧪️tests/🔬️wgpu-ui-command-wiring/🦀️.rs:604)). Generalize it for these legacy tests and dispatch their actual presented node/pointer route.

Two Ink expected strings are independently stale after host identity migration. The canonical projection makes surface_id the owning document and host_id the component instance ([reconcile:482](../../../../../../../../🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🔀️reconcile/🦀️.rs:482)). Native134 accordingly observed a host control ID of scene.1.1.ink… rather than the authored fixture value, because edit control IDs use the host ([Scenes:6206](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:6206)). The old expected focus string is at [test:892](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🧪️tests/🔬️wgpu-ui-command-wiring/🦀️.rs:892). Likewise the cancellation test should expect the mounted target's document surface, rather than the authored fixture surface, at [test:834](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🧪️tests/🔬️wgpu-ui-command-wiring/🦀️.rs:834).

Recommended fixture laws:

1. A focusable Input accepts Tab and mocked paste only after accepted publication, and exactly its presented editor changes.
2. Ink derives private control/focus identity from the mounted host but publishes public actions against the owning document surface; cancel publishes neither normal Up nor Ink mutation.
3. A staged, ACKed Canvas replacement revokes old capture through the presented interaction owner before checking cancellation.

Confidence: high that these Native134 failures are fixture-authority and stale-identity seams. The upgraded fixtures were not executed in this audit.

