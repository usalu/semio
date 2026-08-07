# Wave 3.c — Flow brep as extension

## Split
- Kernel side APIs in `🌊️flow/️️core/📐️brep-geometry`, re-exported from `semio-framework-os-flow`
- Operators packaged at `✏️s/🔌️plugins/🌊️flow/️️extensions/📐️brep` (`ExtensionBundle` + evaluate)
- Framework operator path-mod removed; `install_builtin_flow_extensions` empty
- Callers (procedural3d, playbook) use `flow_core::` geometry APIs
- Removed phantom `✏️s/🔨️plugins` duplicate tree that poisoned registry discovery

## Catalog
- `flow-extension-brep` present in EXTENSION_TARGETS after registry generate
