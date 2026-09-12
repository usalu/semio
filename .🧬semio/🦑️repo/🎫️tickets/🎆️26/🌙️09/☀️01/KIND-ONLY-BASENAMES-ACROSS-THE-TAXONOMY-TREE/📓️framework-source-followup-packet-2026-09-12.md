# Remaining Framework Source Ownership Packet

This follows the completed WGPU/TUI and runtime/scene/render lanes. It covers remaining UI host and UI contract Rust concerns, UI element/style helpers, actor wire-turn, schema component/validator, and asset resolver. Generated projections, including the tracked styling palette CSS, are owned by the generated-source lane; coordinate their imports and do not change their output contract independently. Some CSS files have anonymous-looking but noncanonical kind glyphs; use actual schema resolution rather than naming by visual resemblance. Existing semantic owners should be reused instead of arbitrary new namespaces. This snapshot deliberately omits tool configurations, which have a separate authority review.

First reread all live sources and relevant AGENTS before editing; current paths can change concurrently. Capture byte hashes and active referents before moving. Use semantic owner directories with anonymous implementation leaves. Domain implementations must be outside language package directories, with only minimal tool/compiler glue remaining. Update schema membership, all imports/includes, manifests, producer and test references, source-root assumptions, required authority digests, and launch seed/derived entries where relevant. Do not simply swap a basename and leave a language-first owner.

Add/extend a portable topology fixture and validate with existing native compiler or third-party parser tooling as an independent oracle. Use Bun and registered Nx tasks. Run bounded existing runtime tests for moved concerns and record precise output; do not infer a baseline for current failures. Scope directory inventory to touched semantic owner contexts. Keep disposable output under this ticket’s generated directory and retain a Markdown report with exact move map, supporting files, source/referent identity and validation evidence. No modifying Git commands, AGENTS edits, compatibility layers, migration scripts, or ticket/goal lifecycle changes.

## Snapshot Paths

- `🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🖼️wire-turn.ts`
- `🧰️framework/🔨️modules/🖱️ui/🌐️globals-ui.css`
- `🧰️framework/🔨️modules/🖱️ui/🌓️theme.css`
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🖌️ui.css`
- `🧰️framework/🔨️modules/🖱️ui/🔨️modules/🏷️class-name-composition/🪆️slot.tsx`
- `🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/📡️event.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/📥️enqueue.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/🔌️backend_alias.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/🪟️window.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/🌳️typed/📃️document.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/🌳️typed/🧱️component.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/♿️accessibility.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🎨️style.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🎬️action.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🏗️builder.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/👥️presence.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/📃️document.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/📐️layout.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🗺️surface.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🛡️limits.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🧩️component.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🪢️text_edit.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔌️Ports/📡️interactive-jobs.ts`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🕸️Diagram/📐️layout.ts`
- `🧰️framework/🔨️modules/🖱️ui/🧵️.css`
- `🧰️framework/🔨️modules/🖼️assets/🔍️resolver/🌐️delivery.ts`
- `🧰️framework/🔨️modules/🧬️schema/⚛️component.rs`
- `🧰️framework/🔨️modules/🧬️schema/✅️validator.rs`

## Test Ownership And Ambient Context

The completed UI runtime/render report lists 66 unresolved ambient directory paths. Unregistered semantic names are not automatically incorrect physical names, but the render test folders containing packages-rust explicitly encode the implementation language in the test owner. Review these against the active concurrent test-taxonomy structure: move tests to implementation-neutral semantic case owners, preserve native module access/mounts and portable fixture identity, and register exact contexts where necessary. This is separate from blindly clearing every normalization warning. The report also records five current Metal test assertions calling the absent Scene::finish; inspect the actual current scene contract before a targeted test correction, without inventing a historical pass.

## Reachable Storybook Browser Boundary

The Storybook executor's current native UI build successfully indexes the moved stories and transforms 1,889 modules, then fails preview bundling because `resolve` is not exported by Vite's browser external for Node path, reached at repo-library `🗂️workspaces/🟦️.ts`. The coordinator inspected a concrete static chain: UI `🎨️styling/🟦️.ts` imports `PLAYGROUND_PORTS` and helpers from repo-library `🎮️playground/🟦️.ts`; that module imports Node fs/path, `getWorkspaceRoot` from workspaces and taxonomy discovery. The styling root also imports React build-tooling. The final Storybook report will record the story/preview edge and any identity evidence; no historical baseline is inferred from the current failure.

Include the verified browser-versus-build source ownership boundary in this lane. Separate pure styling runtime/data exports from Node build/dev orchestration under their actual neutral concerns, and make browser imports reach only browser-safe sources. Preserve build/dev behavior and public consumers through explicit relevant exports; do not add Node polyfills, runtime dependencies, legacy paths or a compatibility umbrella that still imports tooling into the browser graph. Re-read current code before edits because the generated-source worker owns only the styling builder/output routing, and the active session-output worker modifies dev/registry scripts. Validate the affected Storybook preview bundle or a representative native browser bundle plus existing styling runtime and tool routes.

## Fresh Package Body Overlap

The corrected policy census identifies these additional or overlapping authored bodies within this lane. Read live content and extract real implementation outside package boundaries, not just named leaf violations. The styling Rust entry contains actual color conversion code after its generated-token re-export, and the TypeScript package entry contains the actual theme model; these are authored source, separate from the finalized generated tokens. Preserve those generated producers while placing the color/theme implementations under shared neutral concerns. Derive macro crate targets can point directly to their neutral source owner while preserving native proc-macro identity.

- `🧰️framework/🔨️modules/🖼️assets/📦️packages/🟦️typescript/🟦️.ts`
- `🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🖼️wire-turn.ts`
- `🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🟦️.ts`
- `🧰️framework/🔨️modules/🔄️machine/✨️derive/📦️packages/🦀️rust/🦀️.rs`
- `🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust/🦀️.rs`
- `🧰️framework/🔨️modules/🧬️schema/✨️derive/📦️packages/🦀️rust/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/♿️accessibility.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🎨️style.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🎬️action.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🏗️builder.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/👥️presence.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/📃️document.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/📐️layout.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🗺️surface.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🧩️component.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🪢️text_edit.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🛡️limits.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🟨️javascript/🟨️.js`
- `🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/📡️event.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/📥️enqueue.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/🪟️window.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🟦️typescript/🟦️.ts`

This is a 23-path overlap snapshot; named host/contract rows already occur above. It does not add the 138 mandated script reviews to this source extraction lane. Use the registered package-body policy after moves to distinguish the remaining thin compiler/package glue.
