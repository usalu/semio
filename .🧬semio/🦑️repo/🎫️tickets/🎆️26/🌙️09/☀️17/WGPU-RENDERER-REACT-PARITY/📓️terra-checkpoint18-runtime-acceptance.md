+# Checkpoint 18 Runtime Acceptance

**Scope.** Read-only source and ticket-probe audit. No activation, server, browser, Cargo, Nx, or probe command was run for this packet. The ticket's earlier generated evidence was removed, so every acceptance result below must be freshly generated.

## Fresh Artifact and Listener Contract

Activate the selected Puzzle3d runtime once per renderer from the workspace root:

```sh
bun nx run @semio-tech/framework-os-dev:activate-puzzle3d-react-dev
bun nx run @semio-tech/framework-os-dev:activate-puzzle3d-wgpu-dev
```

Those are the generated activation targets. They own the complete preparation closure, rather than accepting a pre-existing wasm or component output. The source creates one isolated runtime namespace per renderer, profile, and variant:

```text
🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/dist/runtime/
  react/dev/puzzle3d/activation/🔣️receipt.json
  wgpu/dev/puzzle3d/activation/🔣️receipt.json
```

Each receipt must parse as `semio.dev.activation/v1`, name `puzzle3d` and `dev`, and contain the selected component digest. This location comes from `developmentRuntimeRoot` in [activation](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🟦️.ts:65) and the activation writer in [execution](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🏃️execution/🟦️.ts:89).

Start the two fresh listeners from the workspace root after their corresponding activation. The Dev `ServeScript` accepts **React only**; WGPU must use its renderer-owned browser server.

```sh
S_OS_PORT=6313 bun nx run @semio-tech/framework-os-dev:serve-puzzle3d-react-dev
S_OS_PORT=6213 bun nx run @semio-tech/framework-os-dev:serve-puzzle3d-wgpu-dev
```

The server target generation dispatches React to `📜️script.ts serve <variant> react <profile>` and WGPU to the WGPU browser server's `📜️script.ts serve <variant> <profile>` ([target generator](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs:1024)). React reads and checks its activation receipt before it serves ([React serve](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🌐️serve/🟦️.ts:45)); WGPU's server fixes `SEMIO_PLUGIN`, `SEMIO_RENDERER=wgpu`, and build mode and reports its `?plugin=` URL ([WGPU server](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🌐️server/📜️script.ts:8)).

The controlled probe pair is:

```text
React  http://127.0.0.1:6313/?plugin=puzzle3d
WGPU   http://127.0.0.1:6213/?plugin=puzzle3d
```

The catalog defaults are 6013/6113; 6313/6213 are deliberate non-default ports used by the ticket's main and dock fixtures. React's selected app comes from its injected `VITE_SEMIO_PLUGIN`, so its query string is harmless but not the selector ([React serve](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🌐️serve/🟦️.ts:54)). Do not set `SEMIO_LOCKED_LOCALE`, `SEMIO_LOCKED_APPEARANCE`, `SEMIO_LOCKED_THEME`, `SEMIO_LOCKED_TERMINOLOGY`, or a locked example for the main run: those are projected into the browser's public environment and would invalidate the settings/example journey ([locked preferences](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎮️playground/🔒️preferences/🟦️.ts:3)).

## Required Fresh Main, Dock, and Canvas Runs

Run the main journey from this ticket after both Puzzle3d listeners say ready:

```sh
cd /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY
SEMIO_PROBE_OUT=checkpoint-18-main \
SEMIO_PROBE_TARGETS=react,wgpu \
SEMIO_PROBE_REACT_URL=http://127.0.0.1:6313/?plugin=puzzle3d \
SEMIO_PROBE_WGPU_URL=http://127.0.0.1:6213/?plugin=puzzle3d \
SEMIO_PROBE_SETTLE=2400 \
SEMIO_PROBE_APPEARANCE=dark \
SEMIO_PROBE_LANGUAGE=de \
bun 🐍️parity-interact-probe.mjs
```

The probe fixes a 1600×1000 DPR-1 viewport, enables diagnostics unless explicitly disabled, writes both renderer consoles and one screenshot per journey step, and exits nonzero for a renderer error, parity difference, or physical-geometry difference ([main probe](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/🐍️parity-interact-probe.mjs:27), [result gate](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/🐍️parity-interact-probe.mjs:1434)). The full scripted journey covers boot/tour, panel and settings actions, pane chips, dock/window focus-close-reopen, five World3d gestures, palette/keyboard, fullscreen/chords, example switch, and role cycle ([journey](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/🐍️parity-interact-probe.mjs:694)).

Then run the eight-case Dock probe against the same fresh pair:

```sh
SEMIO_DOCK_OUT=checkpoint-18-dock \
bun 🔬️dock-interactions/📜️script.ts run react http://127.0.0.1:6313/?plugin=puzzle3d

SEMIO_DOCK_OUT=checkpoint-18-dock \
bun 🔬️dock-interactions/📜️script.ts run wgpu http://127.0.0.1:6213/?plugin=puzzle3d
```

It performs four splits, merge, reorder, Escape retirement, and template transfer. It takes a fresh browser context per case and treats renderer faults/page errors as failures ([Dock fixture](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/🔬️dock-interactions/🧫️fixtures/🔣️.json:37), [Dock runner](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/🔬️dock-interactions/📜️script.ts:492)).

Canvas families require their own fresh activated listener pair. For each of `draw`, `layout`, and `note`, activate React and WGPU and start their generated `serve-<variant>-<renderer>-dev` targets at the catalog ports below. Then use the matching two commands, replacing `<target>`, `<react-port>`, and `<wgpu-port>`:

```sh
bun nx run @semio-tech/framework-os-dev:activate-<target>-react-dev
bun nx run @semio-tech/framework-os-dev:activate-<target>-wgpu-dev
S_OS_PORT=<react-port> bun nx run @semio-tech/framework-os-dev:serve-<target>-react-dev
S_OS_PORT=<wgpu-port> bun nx run @semio-tech/framework-os-dev:serve-<target>-wgpu-dev

SEMIO_CANVAS_OUT=checkpoint-18-canvas \
SEMIO_CANVAS_RENDERER=react \
SEMIO_CANVAS_URL=http://127.0.0.1:<react-port>/?plugin=<target> \
bun 🔬️canvas-interactions/📜️script.ts run <target>

SEMIO_CANVAS_OUT=checkpoint-18-canvas \
SEMIO_CANVAS_RENDERER=wgpu \
SEMIO_CANVAS_URL=http://127.0.0.1:<wgpu-port>/?plugin=<target> \
bun 🔬️canvas-interactions/📜️script.ts run <target>
```

| Target | React / WGPU ports | Fresh physical cases |
| --- | --- | --- |
| `draw` | 6064 / 6164 | draw/select/modifiers, repeated same-geometry creation, Escape, double-click |
| `layout` | 6079 / 6179 | page/rect/text/image catalogue preview, leave retirement, and exact one-item drop |
| `note` | 6080 / 6180 | copy, structured/plain/image paste, focused-editor priority, trusted touch cancellation |

The catalog owns those target identities and ports ([generated catalog](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🎮️playgrounds/🟦️.ts:38)). The current Draw runner is a complete physical test, not the earlier discovery-only implementation: it asserts a real layer increase, distinct IDs for a repeated geometry, pixels, selection effects, cancellation, and exactly one double-click commit ([Draw runner](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/🔬️canvas-interactions/📜️script.ts:625)). Layout proves preview-retirement order and creation count ([Layout runner](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/🔬️canvas-interactions/📜️script.ts:973)); Note drives all six real cases ([Note runner](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/🔬️canvas-interactions/📜️script.ts:612)).

## Acceptance Artifacts and Verdicts

All raw run output is ephemeral ticket output:

```text
🗑️generated/checkpoint-18-main/
  steps.json, parity.md, geometry.md, physical.md, cameras.md, latency.md
  react/{console.txt,NN-step.png}
  wgpu/{console.txt,NN-step.png}

🗑️generated/checkpoint-18-dock/{react,wgpu}/
  receipt.json, <case>.png, <case>-console.txt
  <case>-failed.png and <case>-failed-state.json only on failure

🗑️generated/checkpoint-18-canvas/{draw,layout,note}/{react,wgpu}/
  receipt.json, final.png, per-case PNGs
  failure.json and failure.png only on failure
```

A main pass requires two successful boots, no step error or blocked crash, no `differ` verdict, and no `physicalDifferences`. A Dock pass requires every renderer receipt case to have `status: "passed"`, preserving identities/topology/camera semantics as applicable. A Canvas pass requires a receipt with every authored case passed, no `failure.json`, and the runner's action/pixel/model assertions satisfied. Capture the durable conclusion in ticket Markdown before normal generated-output cleanup.

## Five Immediately Measurable Remaining Acceptance Gaps

| Priority | Fresh evidence missing now | Existing probe and pass condition |
| --- | --- | --- |
| 1 | Main-shell and World3d behavior after the current activation and pointer changes | `checkpoint-18-main`: both boot; every source journey step completes without error; renderer actions/state/physical receipts contain no difference. |
| 2 | Dock behavior after current pane ownership work | `checkpoint-18-dock`: both renderer receipts pass all eight cases, including template camera direction/up and Escape release inertness. |
| 3 | Draw retained-owner behavior after its current repair | `checkpoint-18-canvas/draw`: six Draw cases pass on both renderers, including two separate operations at equal rectangle endpoints receiving distinct layer IDs. |
| 4 | Layout catalogue transport after current pointer/owner work | `checkpoint-18-canvas/layout`: all four types publish preview, leave without mutation, then retire preview before one exact created item. |
| 5 | Note clipboard and trusted cancellation after current pointer/owner work | `checkpoint-18-canvas/note`: copy/paste ownership, editor precedence, exact touch cancel without commit, and the next independent commit all pass on both renderers. |

These are acceptance gaps, not confirmed runtime defects: this audit did not execute them.

## Per-Pointer Residual Beyond the 16-Slot Capture Grant

The new fixed-size `PointerCapture` holds owner and last position per pointer ([Shell capture](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:3091)), and scene targets have a separate 16-owner grant ([Interpreter](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:423)). That does not make the remaining gesture authorities per-pointer.

1. **Foreign cancellation is currently destructive.** Renderer cancellation releases only its specified capture then calls the Shell with that ID ([renderer](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:15609)). The Shell then unconditionally cancels global Canvas and list-transfer state, clears dock/tree/chrome state, and clears the singleton input drag/hover/down state ([Shell cancellation](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:12453)). A foreign cancel can therefore finish the primary pointer's Canvas gesture with a spurious terminal `canvasPointerUp {cancelled:true}`, or discard its Chrome/transfer drag.

2. **Retained UI capture is global.** `retained_pointer_capture_window()` returns one window without a pointer ID ([Interpreter](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:602)); the Shell sends the foreign pointer's cancellation to that window. UI event routing therefore needs pointer-addressed capture before the Shell can correctly suppress foreign cancellation.

3. **Canvas and transfer state lack an owner pointer.** `CanvasGesture` records window/document/surface/controller but not a pointer ID ([Scenes](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:425)); `SceneListTransferSession` likewise has no pointer field ([Scenes](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:860)). They cannot distinguish a foreign cancel from their owner cancel.

The narrow next RED should extend the existing `wgpu-shell-input` fixture rather than add a string-only law: P77 starts Canvas2d or a dock split drag; P78 starts/cancels elsewhere; P77 then moves and releases outside. Assert that P78 produces no P77 terminal action, P77's drag/preview remains live until P77's own terminal event, and P77 emits exactly its one expected terminal effect. This complements the existing World-only `foreign-cancel` scenario, which currently counts World events and cannot observe Canvas or Chrome authority survival ([World fixture runner](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-shell-input/🦀️.rs:36)).

**Confidence:** high for commands, URLs, output locations, and probe behavior because they are current source routes; high that foreign cancellation is still global in the inspected source; no runtime result is asserted.

