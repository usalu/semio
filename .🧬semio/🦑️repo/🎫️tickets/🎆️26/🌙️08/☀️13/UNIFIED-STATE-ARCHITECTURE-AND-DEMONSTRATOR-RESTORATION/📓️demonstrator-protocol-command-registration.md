# Demonstrator Protocol And Command Registration

## Scope

This report covers the demonstrator/plugin worker protocol failures observed while restoring the unified state architecture: invalid typed contribution pushes, recursive command routing through the action boundary, worker initialization watchdog warnings, and the browser acceptance of the six-pane demonstrator.

## Reproduced Failures

- The renderer warned that typed `setContributions` and `setAppRegistrations` pushes were skipped because the receiving app manifests did not declare those commands.
- The CAD manifest declared `setContributions`, but its typed action bridge did not decode that declared command.
- `ShellHelpers.makeEffectDispatchOne` encoded a host effect with the pack wire codec (`pk:...`) and passed it to `PluginRuntime.handleAction`, whose typed boundary accepts JSON. The browser therefore raised `SyntaxError: Unexpected token 'p'` while parsing the initial document action.
- After correcting the pack/JSON boundary, the next masked fault was `SemioFaultError: action '' is not a framework-reserved action`. Temporary `[DEBUG]` tracing showed the pre-bridge payload was `{ "pluginId": "procedural", "appId": "procedural3d-play", "action": "flowEvalTick" }`: a typed recursive app command was being sent through `handleAction`.
- Under a cold build with Cargo/Nx saturation, the renderer emitted its 10-second worker-unresponsive watchdog message for procedural and demonstrator. Both workers subsequently became live and no restart occurred. On a warm boot, both became live without the warning. This establishes a cold staging/load artifact rather than a worker registration restart loop; final acceptance still requires a warning-free boot.

## Root-Cause Fixes

- `ShellHost` now encodes owner-qualified `CommandInvocation` JSON and uses manifest declarations to decide whether a typed contribution or app-registration push is supported. Undeclared pushes are not attempted; declared pushes report only real failures.
- The bundled contribution consumers declare hidden, non-palette `setContributions(json)` commands. Space declares hidden `setAppRegistrations(json)`.
- CAD's typed bridge decodes its declared `setContributions` command.
- `ShellHelpers` now constructs fully scoped JSON `ActionInvocation` values for the action boundary. It constructs owner-qualified JSON `CommandInvocation` values for declared app commands and selects `handleCommand` versus `handleAction` from the active app's command declarations. There is no trial dispatch, fallback adapter, pack-wire input, or legacy path.
- Procedural 2D, procedural 3D, and Flow declare hidden `flowEvalTick` app commands, so recursive worker effects remain on the typed command channel.
- Temporary `[DEBUG]` effect tracing was removed after identifying the source.

## Extended Existing Tests

- The existing CAD production bridge test asserts exact typed `setContributions` decoding.
- The existing demonstrator app test asserts the exact four bundled contribution consumers, their hidden JSON command contract, and procedural 3D's hidden argument-free `flowEvalTick` command.
- The existing renderer test asserts exact fully scoped JSON for recursive action invocation and exact owner-qualified JSON for recursive command invocation.

## Validation Evidence

- `bun nx run @semio-tech/framework-renderer-react:lint --skip-nx-cache --output-style=stream`: passed; `framework-renderer-react: region/host-contract lint passed` and Nx succeeded.
- Earlier focused renderer boundary validation passed before the command assertion was added: 1 passed, 314 skipped.
- The post-command-assertion focused reruns did not execute a test under current shared-machine saturation. One exceeded the 60-second process budget; the 180-second rerun reported `Test Files no tests` and `Timeout waiting for worker to respond`. These are not counted as passing validation.
- Earlier current-source demonstrator validation passed 20/20 before adding the `flowEvalTick` assertion.
- The current demonstrator rerun was stopped while waiting on the shared default Cargo target, per coordination instruction; it is not counted as passing validation.
- The full renderer quick suite previously exceeded its 180-second budget and is not counted as passing.
- The procedural quick suite previously passed 74 tests before an unrelated existing mesh-codec assertion failed (`procedural3d_mesh_bridges_round_trip_through_obj_glb_stl_codecs`, empty positions).

## Artifact And Runtime Gate

- Intended active demonstrator listener: Vite PID `59882`, `127.0.0.1:6029`, working directory `♻️mit-bestand/🧺️demonstrator`.
- Stale PID `83668` has the same command line but does not own the 6029 listener; it is excluded from evidence.
- Fresh demonstrator worker artifact: `semio_s_plugin_demonstrator_component.core.wasm`, staged `2026-08-14 12:03:36`.
- The standalone procedural artifact currently served by `/plugin-modules/procedural` is still stale (`2026-08-07 21:14:12`, 41,928,392 bytes). Browser acceptance is therefore blocked until a post-fix procedural-only build stages a newer artifact.
- A single owned procedural-only build is active on port 6098 with `SEMIO_PLUGIN_ONLY=procedural`, `FORCE_PLUGIN_BUILD=1`, and two Cargo jobs. It is currently waiting for unrelated default-target Cargo holders.

## Required Final Acceptance

The final fresh-tab proof must show both `[DEBUG] plugin worker + procedural (2 live)` and `[DEBUG] plugin worker + demonstrator (2 live)`, all six pane markers (`Sechseckige Pilzsäule`, `Entwerfen mit Bestand · cad`, `Abbau Aufbau`, `Beispielbestand`, `Holzbalkenverbindung`, `Karte wiederverwenden`), no alerts, and zero page errors/warnings, including no `setContributions`, `setAppRegistrations`, `pk:`, empty action, timeout, restart, or unresponsive message.

## Source-Stability Gate Evidence

- Demonstrator acceptance was held while the shared stdio schema gate was red; building the procedural worker during those writes would have produced stale evidence.
- A temporary assignment to the XML/SVG stdio compile clusters found active same-file collisions: `XmlSnapshot` and `SvgSnapshot` gained `lexical` fields during one read and lost them again before the next, while diff/mutation callers still referenced those fields. Malformed generated initializer fragments also appeared and disappeared between reads.
- An isolated canonical Nx check used a separate ticket-local `🎯️target-xml-svg` and reached dependency compilation, but was intentionally stopped with exit 130 after root established exclusive ownership with the artifact tasks. It is not counted as validation.
- No XML/SVG source edits were made by the demonstrator-protocol lane. The retained cold target and this report are the evidence requested before returning to source-stable demonstrator acceptance.
