# Play Release Descriptor Pair Audit

The Play release passes `playRuntimeComponentIds()` to the component driver. That function forms a union of closures over the generated `PLUGIN_BUILD_TARGETS` and `EXTENSION_TARGETS` catalog. All 74 rows in that catalog currently have both `🔣️.json` and `🛂️.descriptor.semio` in their component owner directory, so no catalog-selected Play component is missing a descriptor pair.

This is a presence audit over a superset of the Play closure, using generated catalog crate paths and the descriptor locations used by `componentStageTargetV1`. It does not establish that the generated catalog is current or that descriptor bytes match freshly built release components. No Cargo build or describe operation was run.
