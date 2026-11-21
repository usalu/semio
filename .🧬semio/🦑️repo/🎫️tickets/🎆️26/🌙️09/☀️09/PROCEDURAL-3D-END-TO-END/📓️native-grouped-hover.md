# Native Grouped Hover

The native scene bridge admits the optional authored \`hoverTargets\` list as canonical compact JSON. It preserves the first occurrence of each exact granularity and ID pair. It treats empty, malformed, or oversized lists as an ordinary instance hover. Instance aliases share the scene's 1024-entry carrier limit and 256-byte identifier limit. The canonical JSON writer escapes each string before enforcing the 4096-byte action argument bound.

The world retains the compact string keyed by rendered instance ID and replaces the map after sealing a changed guest scene. The changed map advances the interaction revision so an earlier pointer plan cannot publish it. The native ray cursor continues to choose the nearest instance and looks up only that winner's aliases. Hover plans carry the group in one string span, while the publisher checks the whole action's byte credits and retains the complete canonical payload for deduplication only after publication. Background clears, pointer release, authored hover handoff, and world retirement release that payload. Click selection and prioritized component picks use their ordinary addresses.

The shared World3dHost JSON fixture covers the two possible coincident winners, independent instances that reuse the mesh, empty and malformed lists, escaping, and heterogeneous target granularities. Registered native tests add ray and action publication, unchanged-primary group changes, selection, clear and re-hover, full action credit fallback, and scene replacement and retirement. The bounded encoding and serde parser passed two isolated Rust tests against the shared fixture, using the real production encoder and parser with serde_json as the independent encoding oracle. The nightly Rust parser accepted all 12 changed native Rust source files.

The integrated native test binaries still require a coherent WGPU dependency build. A direct metadata check from the local native cache could not start source analysis because cached WGPU input and related crates expect an earlier strict version hash of the UI declaration crate than the artifact at the same cache path. The managed repository build and registered tests remain the authority for integrated execution.


## Native Runtime Verification, November 21

The managed preview build refreshed the native World3d host and its dependencies. A focused Rust harness linked those production libraries and ran four registered test bodies with production scene fixture setup. All four passed.

- The real ray and action pipeline published both targets for either coincident winner, preserved the independent same-mesh instance and singleton selection, deduplicated repeated hovers, republished changed targets, and cleared and re-hovered.
- Native admission matched the shared escaped JSON, UTF-8 identifier, and target count bounds.
- An alias list that fits the target string but exceeds the aggregate action budget fell back to the ordinary target.
- The production scene bridge installed, replaced, removed, and retired its authored hover targets.

A separate build of the stock Cargo test binaries reached the world scene crate, where concurrent terrain changes refer to the unavailable \`semio_framework_surface_terrain::overlay\`, \`TerrainSessionCore::overlay_tile\`, and \`WorldTerrainStyle\`. The focused harness bypassed that testkit rebuild by using the production native libraries from the managed build. It exercised native application state and action publication. It did not launch a WGPU window.
