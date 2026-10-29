# Final Integration Audit

2026-10-28. Read-only source inspection; no compiler, graph preparation or browser execution. Root, OS and layout instructions and build skill read. Source files unchanged.

## Current Catalog Selection

The current declared `@semio-tech/framework-os-dev:activate-layout-react-prototype` target already receives the specific catalog through target generation. No additional target flag is required for that declared route with the current graph implementation.

Evidence: build graph `playground-targets/🟨️.mjs` collects component manifests only through catalogBuildContributionForManifestV1 (lines32-45), and its final mapping injects options.env.SEMIO_PLUGIN_CATALOG from the matched playground buildCatalog.publication.outputRoot (lines300-303). This includes activation, preparation, serving and related variant targets. Specific publication `✏️s/🧑‍💻dev/🧩️catalog/📣️publication/🟦️.ts` combines its own declaration with `🌎️hub/📇️catalog/🔣️.json`, whose packageRoots includes Hub compositions. Thus the layout component belongs to that specific publication, with outputRoot `✏️s/🧑‍💻dev/🧩️catalog/🤖️generated`.

All three current generated projections agree: the layout playground cratePath is `🌎️hub/🧩️compositions/📏️layout/🧩️component/📦️packages/🦀️rust`; source and deployed row packageName is semio-hub-layout-component and wasmOut is semio_hub_layout_component.wasm. The old parent crate remains a library without the plugin component metadata, so current target collection cannot select it as the layout plugin.

`runtimeCatalogProjectionRootV1` reads only SEMIO_PLUGIN_CATALOG and returns undefined if absent; it has no registry/dist/sessions/layout fallback. Direct OS owner script invocations can provide `--catalog <root>`, parsed and removed by its entrypoint before routing. Ticket launch-layout.py also pins SEMIO_PLUGIN_CATALOG for activate/serve/probe. It is useful for explicit caller evidence, but missing caller env on a current declared activation does not prove stale selection: the generated target supplies it. A previously sealed old graph must be diagnosed from its actual target/receipt, not assumed from current source. No live graph claim is made here.

## Editor Companion Integrity

The editor unit test companion Cargo package is registered exactly once as `semio-s-artifact-layout-layout-editor-tests` in root build owners, with its Cargo.toml as manifest. The companion lib resolves to the existing `🧪️cases.rs`, whose tests are cfg(test) and use public editor interfaces plus editor testkit context. The production editor mounts only `unit_tests/🦀️.rs`, which now contains the context module rather than all cases. The companion enables editor testkit and model testkit and carries test-only framework features. This is a coherent source separation; no missing registration or entrypoint was found. Its absolute Cargo manifest path is156characters. This source audit does not substitute for the pending native run.

## Remaining Browser Harness Gaps

The current archive/media harness has stronger real witnesses than the original probe, but these precise limitations remain:

- Fault assertions run only for contract.steps. artifact-panel-baseline is recorded but absent from browser-journey.json and never fault-checked. No assertion checks any state.hosts phase/fault, and ordinary console error lines remain ignored unless matching faultLines narrow regex. A surface fault can therefore pass despite a ready beacon.
- Undo requires at least one console line containing CreateFrame and final baseline frame count, but does not require submitted=ok, an exact newly added stable ID before undo, or successful Undo receipt. A failed creation that logs CreateFrame may still satisfy it. The current minimum is stronger than a pure no-op but is not authoritative document evidence.
- Archive restoration compares saved frame/page IDs and demands divergence from modified original pages. Good independent reopening evidence, assuming those lists address the active new instance. state() collects all document-global treeitems; it does not scope the Artifact tree to the focused instance. With multiple visible windows/trees, aggregate IDs can cause false failures or mask which instance was restored. Recording/focusing the new instance and scoping tree queries would make the witness exact.
- Media validation decodes SVG with jsdom/sharp, PNG with sharp, and PDF with pdfjs, including PDF page count and authored text. SVG/PNG only require positive dimensions and nonuniform pixels, so unrelated content can pass. A page/content marker or independent expected dimensions/content witness would strengthen those formats. PDF Hello layout assertion deliberately relies on the demo fixture and should remain tied to that fixture.
- Exports happen before document edits; the media checks prove demo export, while archive proves edited state round-trip. They do not yet demonstrate exported media includes subsequent edits. Redo and same-context reload persistence are still absent.

No live failure is inferred from these validation gaps. Current browser execution remains required before ticket end-to-end success.
