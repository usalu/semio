# Remaining roster closures: wasm32-wasip2 check per package

Command per package: `repo cargo --store=shared --configuration=release --parallelism=1 -- check --locked --offline --keep-going -p <pkg> --lib --target wasm32-wasip2 --manifest-path 🌎️hub/Cargo.toml --message-format=short` (logs: `🗑️generated/rc-<pkg>.log`).

Skipped: demonstrator, procedural (being rebuilt). Confirmed (has `🗑️generated/confirmed/*.json`): cad, energy, fem, gis, process, puzzle, flow-extension-bim, flow-extension-dictionary, sourcing-module-windows.

| Module | Package | Status | Fixes |
|---|---|---|---|
| (priority insert) | semio-s-artifact-stdio-pdf-mut-base-4 | clean (exit 0) at 06:2x PST | none by me; Cargo.toml had been edited by another agent at 05:57 and rechecked clean |
| sourcing | semio-hub-sourcing-component | clean (exit 0) | none needed |
| flow | semio-hub-flow-component | paused by coordinator (cargo pause); first run hit E0432 in semio-plugin-runtime-ops (unresolved `semio_plugin_runtime_ops_raw_media`) | none by me: dependency was added to ops Cargo.toml (and Cargo.lock) by another agent at 06:42:53, so the error looks like an in-flight race; recheck after cargo-ok.flag |
| playbook | semio-hub-playbook-component | pending | |
| sourcing-module-beams | semio-s-plugin-sourcing-beams-component | pending | |
| sourcing-module-slabs | semio-s-plugin-sourcing-slabs-component | pending | |
| flow-extension-brep | semio-s-plugin-flow-brep-component | pending | |
| flow-extension-draw | semio-s-plugin-flow-draw-component | pending | |
| flow-extension-list | semio-s-plugin-flow-list-component | pending | |
| flow-extension-logic | semio-s-plugin-flow-logic-component | pending | |
| flow-extension-math | semio-s-plugin-flow-math-component | pending | |
| flow-extension-primitive | semio-s-plugin-flow-primitive-component | pending | |
| flow-extension-text | semio-s-plugin-flow-text-component | pending | |
| playbook-module-procedural | semio-s-plugin-playbook-procedural-component | pending | |
| cad-extension-aec-building | semio-s-plugin-cad-aec-building-component | pending | |
| cad-extension-aec-building-energy | semio-s-plugin-cad-aec-energy-component | pending | |
| cad-extension-aec-building-structure | semio-s-plugin-cad-aec-structure-component | pending | |
| cad-extension-spatial-shape | semio-s-plugin-cad-spatial-shape-component | pending | |
| process-extension-concrete | semio-s-plugin-process-concrete-component | pending | |
| process-extension-metal | semio-s-plugin-process-metal-component | pending | |
| process-extension-robotic | semio-s-plugin-process-robotic-component | pending | |
| process-extension-wood | semio-s-plugin-process-wood-component | pending | |
