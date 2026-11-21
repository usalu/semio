# Raster editor frontend

## Example switching

The retained example action now compares the persisted raster snapshot, including document identity, title, layers and content-addressed image handles, and confirms that the expected image owners are materialized. A matching example remains a no-op. The existing protected-layer guard remains in force.

Six neutral cases cover unchanged examples, a different document identity, edited title, replaced image identity, missing image materialization, and protected layers. The independent oracle uses fast-json-patch to apply each test edit and Lodash to compare the reference document.

## Importing a local PNG

The Layers panel and command palette now offer Import PNG in English and German. The action requests a single bounded file through the existing host picker. Both renderer families return file chunks through the normal app action ingress. The framework assembles a complete data URL before calling the app’s internal `importPngFile` action. The picker accepts PNG files up to 3 MiB minus 1 KiB, leaving room for the data URL and base64 encoding inside the framework’s 4 MiB staging limit.

The callback validates the data URL header and extent before taking its original String. The retained command producer moves that command into its original custody and starts at Preflight with no raw command buffer. Its work passes the untouched data URL to the existing Base64DecodeCursor using an input range, so there is no prefix-removal copy. The PNG decoder, image metadata, layer and child candidate, and retirement logic are shared with the existing `image:in` port. The two artifact mutations publish in one operation, after bounded work against the captured document. Both shells already report progress and support cancellation during chunk delivery. The React browser picker also supports cancellation during bounded file reads. The Layers panel’s operation controls cover the retained decoding and publication job.

Neutral picker fixtures cover both host data URL headers, refusal of a JPEG carrier, refusal of an empty body, and the localized control in an empty document. The mounted test drives the picker action, submits two shell chunk envelopes, proves the first chunk leaves the document empty, verifies the exact one-pixel content and checks a single Undo removes both the layer and image asset. Sharp independently decoded the one-pixel fixture as RGBA `[10, 20, 30, 255]`.

## Native verification

The original focused editor companion admission stopped before compilation at 270,115 of 262,144 allowed source bytes. The example-switch, duplicate and move tests are now mounted in their existing command implementation crate. The standalone filter tests and the field-level patch laws are also mounted in their existing command crates. The editor companion still runs archive, compositing, and mounted history coverage. The cases remain at their original taxonomy nodes and the Raster Rust package test router explicitly includes the second commands crate. This removes the oversized companion’s command-only test sources and fixtures without changing the shared policy.

Both independent oracles passed. Pinned rustfmt parsed all changed integration files. Focused native command crate run 60221 reached dependency compilation. It was cancelled after becoming superseded and released its leases. Raster mounted test run 77535 reached source admission and refused the companion at 262,808 of 262,144 bytes, which prompted the smaller companion mount for the filter laws. Runs 82887 and 94046 reached the same gate at 266,084 and 264,008 bytes. The last revision tightened historical comments without changing assertions or increasing policy limits. Mounted picker run 49327 completed all 1,039 source-admission units and entered compilation. It was cancelled after the subsequent ToolJobContext import correction so only the final source is executed. Final mounted PNG test run 25807 exited 1 before Cargo because disk maintenance could not provision the build reserve. Roughly 5 GiB remained after collection. The dependency image’s existing setup holder delayed cache GC and the descriptor build. Execution verdicts and browser confirmation are tracked by the surrounding ticket.


## Boxed retained command work

The framework’s boxed work adapter now forwards the original output-queue receiver, wire demand quote, wire receiver, and wire-retention flag through its existing virtual dispatch. This restores the behavior declared by Layout’s output receiver and IFC's retained wire receiver, as well as Raster’s original-wire refusal. The Raster file callback already enters Preflight with an original typed command and does not require raw wire decoding.

The native regression reuses the ToyRunApp and original-caller fixtures. It checks the wire quote, typed wire receipt and retention flag, and verifies a denied turn keeps the real queue in the caller, while a funded turn moves the same queue allocation to the inner owner. The independent fixture and forwarding check failed before the implementation and passed after it. Native focused run 20521 passed source admission, then exited 101 while compiling the upstream semio-framework-actor-retained-context library. The retained Cargo fingerprint contains warnings and no Rust error diagnostic. The boxed-work regression itself did not execute.

## Original effect migration

The Raster picker and `image:in` share the PNG admission path. The editor now uses a resident construction admission for the retained base64 source, decoded PNG candidate, payload, and mutation handoff. The PNG decoder runs on the caller’s original allocation effect. The payload quote includes the physical Arc copy and capacity plus the caller’s transfer work. Its Rust regression checks that one-byte and one-unit-short grants preserve the same pixel allocation until a funded publication.

The paint preview uses a caller-owned effect for semantic apply and a settled original construction for its snapshot. Selection uses the packed span builder’s current effect API for publication and close. The mask and flatten paths include the image birth in their budget and route cleanup through current retained effects. Mounted tests continue to cover the underlying editor operations and independent fixture expectations. The export and Store directories are coordinated with their separate owners.

The independent Sharp PNG picker, six example-switch cases, and boxed forwarding oracles all passed after the migration. Pinned rustfmt parsed the edited Rust sources. Git’s whitespace checks passed. At this report’s update, the coordinated WASI recheck had not yet compiled the Raster editor leaf and this migration had no new native execution verdict.

## App startup signature audit

Compared the production `RasterPlayApp` editor implementation, app assembly, fixed document genesis, and native instance and ephemeral declarations with their current framework contracts. The editor’s disposal, mutation outcome, reference birth and contextual render return types resolve to the current framework aliases. The expected startup owner and reserved job methods are present. Pinned rustfmt parsed the five app and startup sources and the scoped Git whitespace check passed. This source audit required no production changes. The narrowed WASI check was still running.

## Image source custody audit

A concurrent replica update added `source_body_bytes: Option<u32>` at typed Image component field ordinal 2, preserving `src` at 0 and `alt` at 1. The existing Rust and TypeScript wire schema keep `sourceBodyBytes` optional. Present values drive the exact retained source-body lookup. Absent values keep inline source-key behavior. The retained browser typed reader produced the expected output for both language-agnostic Image cases, checked with Node strict equality. Pinned rustfmt and Git whitespace checks passed. The audit made no overlapping source edit, and the descriptor retry was handed back to the build owner.

## Window resource module audit

The native picker build’s four unresolved `window_resource` references now resolve through the existing `semio_ui_contract::*` import. A concurrent update mounted the canonical page and byte-count types as the public `window_resource` module in the contract crate that the Window transient package already declares. The module compiled standalone with pinned rustc. The module, crate root and consumer passed the Rust syntax and whitespace checks. No overlapping edit was needed. The build owner was notified to retry with a fresh source snapshot.

## Mounted picker and Chromium route

The native mounted picker law is `raster_file_picker_imports_png_as_one_undoable_layer` in the editor test companion. The root Cargo package selector exited 101 because the companion is selected by its own manifest. The one-job shared prototype Cargo invocation with the explicit `🧪️tests/🥣️companion/📦️packages/🦀️rust/Cargo.toml` manifest passed all 1,040 source admissions and entered the shared store as session 51217.

The current React shell routes `RequestFileOpen` through bounded file reading and one ordered dispatch per `importPayloadChunks` result. The paint surface, controls, status, action rail, dock tab and import completion selectors match their defining components. The Chromium probe now verifies the actual chunk count using the same library as the shell. Sharp produced a 262,913-byte PNG, and the shared chunker emitted eleven pieces that joined to the same data URL. Bun syntax and Git whitespace checks passed. The browser run still depends on the canonical component build.

The follow-up scan covered 20 recursively mounted Rust test sources. The envelope and scene laws now import `ArtifactPack` through the companion’s existing `store` alias. The archive coverage owner updated its two stale archive admission calls. The export coverage owner updated all three typed result-page captures to use the real fixture effect and retire each page before acknowledging it. The native run continued to warm the shared dependency cache after these fixes.

Native session 51217 was safely interrupted at `semio-os-spr`, before the Raster leaf, to prioritize the canonical browser component under host memory pressure. The repository wrapper exited 130 and confirmed its leases were released. The completed shared compilation units remain available to a subsequent invocation. There is no mounted picker runtime verdict from this run.

## Paint preview snapshot custody

The canonical WASI descriptor reached Raster modes and refused the transient preview’s raw `Arc<RasterSnapshot>` because that type no longer implements `RetireOwned`. The transient state, producer, and cancellation garbage now share the framework’s `ArcLease` wrapper. The paid constructor still allocates the same snapshot once. Clones and equality use the same backing, and the renderer borrows it only for its exact base revision. The new Rust law exercises observation after one lease closes, stale revision rejection, and final funded retirement. The fixture’s revision and identity check passed independently with Node Buffer and strict equality. Rust syntax and whitespace checks passed. The canonical descriptor retry was handed back to the build owner.

## Image-backed selection custody

Raster’s image-backed wand now uses the framework’s exact shared allocation layout and records the actual work on the source effect. The original pixels move into the shared backing. A denied quote preserves the source, and a failed allocation returns it to the operation’s closeable slot. The live flood cursor already keeps its own Raster lease. Raster wraps the extracted source in `ArcLease` before transferring it to retirement. The native law checks one-short copy, capacity and Work refusals, the same pixel allocation after handoff, exact flood coverage, and custody on cancellation. Sharp independently matched the neutral two-pixel fixture. Rust syntax and whitespace checks passed. The canonical build remains the execution path for this change.

## Chromium proof readiness

The React action rail, bounded picker delivery, selected-layer callback and paint completion status match the probe’s workflow. The probe now filters action rows by visibility before requesting the rail and clicking. The archive loader focuses a spawned document whose layout contains separate Composite and Navigator windows. The probe selects the newly namespaced Composite window and verifies the imported asset’s exact restored identity and subsequent mutation in that window’s panel. A jsdom case with Navigator listed first selected the correct Composite identity. Bun syntax and Git whitespace checks passed. This was a source and test-harness check. Chromium execution remains with the launcher owner.

## Component compiler one-slot admission

The native component pass now derives its minimum of one jobserver token from the live host decision. The preferred count, memory budget, and reason are retained. The same component decision creates the counted lease request and validates its eventual grant. The existing compiler capacity scope continues to own CPU and memory admission. Global capacity policy and scheduler bootstrap are unchanged. The focused regression passed 10 assertions, including an independent Ajv check for zero refusal, one admission, and refusal above the unchanged preferred count. Scoped Git whitespace checks passed. The descriptor restart remained with its owner.
