# Wave 3.c — Flow brep as extension

## Split
- Kernel side APIs: `🌊️flow/️️core/📐️brep-geometry` re-exported from `semio-framework-os-flow`
- Operators: `✏️s/🔌️plugins/🌊️flow/️️extensions/📐️brep` with ExtensionBundle
- Framework operator path-mod removed; `install_builtin_flow_extensions` empty
- Callers use `flow_core::` geometry APIs

## Members
Flow extension crates under plugins should be Cargo workspace members (wave 4 completes packaging).
