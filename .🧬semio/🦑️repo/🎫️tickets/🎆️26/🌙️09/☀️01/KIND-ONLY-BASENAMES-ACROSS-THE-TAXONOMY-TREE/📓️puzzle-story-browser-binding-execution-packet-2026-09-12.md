# Puzzle Story Browser Parser Binding

This is a bounded follow-up within the active taxonomy goal, not a completion claim. Preserve concurrent work. No product files were changed for this investigation.

## Observed Production Gap

The authored timeline story `✏️s/🔌️plugins/🧩️puzzle/📖️stories/🎭️5d-timeline/🧪️.story.tsx` dynamically imports `@semio-tech/puzzle-5d-rs`, initializes its default export, then calls `puzzle5dParseDslJson` on three real DSL examples. A native TypeScript parse reports zero syntax diagnostics, while module resolution from the actual story reports `MODULE_NOT_FOUND`. The earlier unscoped Storybook build stopped at this import after 113 modules; the scoped UI build's green result does not cover this consumer.

The requested parser exists: the Rust bridge at `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️.rs` exports `puzzle5dParseDslJson` with wasm-bindgen. It calls the existing `Puzzle5dSnapshot::parse_dsl` and DSL JSON serializer. Its parent mount is gated by `component-app-assembly`, which is not a default feature. The bridge also owns editor/app envelope functions, so exposing a parser must not blindly duplicate or relocate that whole assembly.

The identically named Nx project is a Rust artifact project, not an installed JavaScript package. Its Cargo library points at the anonymous artifact Rust entry, declares no crate type, and defaults to no features. Its build/check/test routes use the native shared artifact Cargo runner; no JavaScript manifest, browser facade, declarations or wasm-bindgen package production is established by that project name. The component feature pulls Infinite, the component guest, async/UI assemblies and wasm-bindgen. The current `@semio-tech/puzzle-5d-react` package composes timeline/topology models but provides no DSL parser, so swapping the import to that package is not a solution.

## Execution Boundary

Implement a genuine browser projection of the existing parser under its semantic domain. Choose the smallest actual parser binding boundary after inspecting Cargo feature and module dependencies; avoid forcing the entire editor/app assembly into a read-only story if the parser can be independently owned. Keep source leaves anonymous, keep the original parser/JSON schema authority, and make source ownership precede language implementation. Do not create a compatibility npm alias, suppress the import, substitute an ad hoc TypeScript parser or commit precomputed story results.

Add schema-backed portable cases for the three story DSL inputs and malformed input. Establish first-red browser resolution/behavior, then compare native parser output with the browser binding using the existing third-party/native compiler pipeline. Wire actual Wasm generation, JavaScript/type exports, Nx producer dependencies and inputs/outputs, Bun package routing and launch entries using existing repository conventions. Generated output admission requires a real producer contract and consumer identity. Verify runtime parsing and the ordinary Storybook build, not just externalized bundling or successful TypeScript parsing. Keep all transient artifacts under this ticket's generated directory and record exact owned paths.

## Evidence and Limits

The read-only probe is retained in `🗑️generated/coordinator/puzzle-story-binding-inspection.json`. It records the actual story, native parse, module-resolution failure, Cargo feature/library data and Nx targets. No native Wasm build, browser parser execution or Storybook rerun was performed in this investigation.

An adjacent Puzzle JavaScript package currently contains CAD-specific metadata/routes. That is separate observed metadata debt; inspect its actual intended ownership before changing it as part of any binding closure.
