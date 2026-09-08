# Sequence Browser Artifact Package

The package-purity scan found two handwritten JavaScript implementation files and a handwritten declaration file inside the artifact's nested Wasm package directory. They now belong to adjacent browser and host domain facets. The artifact exposes its browser API and types from its taxonomy-root TypeScript entry, with declarations under `📦️packages/🟦️typescript` and name `@semio-tech/sequence-sequence`. This adds a fortieth TypeScript artifact package to the dynamic inventory.

Before implementing the declaration, the existing browser-consumer test was changed to use the new public package. Running it exited 1 with the expected missing-package error. The existing language-neutral protocol fixture and AJV oracle now consume that same public browser entry and compare its encoded protocol frame and semantic response against the independent oracle. Its URL base was corrected to preserve the Wasm facet directory.

The former Sequence JavaScript parent declaration only reexported this artifact and carried copied CAD commands and runtime dependencies. Its facade was removed. It is now a private `@semio-tech/sequence-tests` integration harness, depending on the artifact build and running the existing interface, host, public browser, protocol-oracle and example tests. No old API compatibility export is retained. The handwritten browser types now match the supported default WebAssembly input categories, BufferSource and Module.

The repo package-purity authority fixture references an older preimage census with different filenames and is a separate captured audit; it was not rewritten as if these new moves were its original observations. Runtime, emitted-declaration, Nx and launch checks remain pending.

The new artifact entry and private test harness are explicitly registered in the Bun root workspace, preserving the repository's current explicit registration convention. The harness declares the existing AJV 8.20.0 dependency only for tests. The neutral declaration copier now preserves its handwritten declaration sidecar. Its first post-build typecheck passed, then runtime package import failed because workspace links were not yet refreshed; this was recorded as a failing test and prompted the Bun workspace install.

The second direct artifact TypeScript test exited 0 after Bun workspace registration and installation, confirming successful declaration consumer checking and runtime package import. The runtime integration harness subsequently exited 1, but its redirected diagnostic file was removed by the concurrent generated-directory deletion before inspection. A second integration invocation now streams diagnostics to both the tool and a recreated ticket log.

## Current Public Browser Runtime Gate

The full direct integration script completed with exit 0 after adding the missing canonical ABI fixture schema. The schema was also absent from HEAD while the Rust protocol already referenced it with `include_str!`. The existing neutral JSON fixture remains unchanged. Interface checks covered 10 features, 47 operations and 8 events. Host laws covered cancellation, hostile controls, resource ownership, bounded pages/events, retry/acknowledgment and terminal emptiness. The public package consumer passed, the AJV test-only oracle reported identical protocol bytes and semantic output, and both existing demo asset examples passed (2 passed, 0 failed). An ordinary Nx harness run including the artifact build prerequisite is running separately.

## Normal Nx Prerequisite Gate

`bun nx run @semio-tech/sequence-tests:test --output-style=stream` completed with exit 0. Nx ran the artifact build prerequisite (three emitted outputs) and then the complete public browser integration harness. Both tasks succeeded without cache hits; Nx reported 56.8 seconds for the task run. This confirms the normal prerequisite route, public API runtime, declaration production, protocol oracle and two examples.

## Coordinator-Owned Files

- `package.json`
- `bun.lock`
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🟦️.ts`
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/📦️packages/🟦️typescript/package.json`
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/📦️packages/🟦️typescript/📜️script.ts`
- `✏️s/🔌️plugins/🎬️sequence/📦️packages/🟦️typescript/package.json`
- `✏️s/🔌️plugins/🎬️sequence/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🎬️sequence/📦️packages/🟦️typescript/📜️script.ts`
- `✏️s/🔌️plugins/🎬️sequence/📦️packages/🟦️typescript/🟦️.ts`
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🌐️browser/🟨️.js`
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🌐️browser/🟨️.d.ts`
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🖥️host/🟨️.js`
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🧬️schema/🔣️.json`
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🧬️schema/📜️.wit`
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🧪️tests/🧬️schema/🟨️.js`
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🧪️tests/🖥️host/🟨️.js`
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/📦️packages/🟨️javascript/🌐️sequence-browser.js`
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/📦️packages/🟨️javascript/📜️sequence-browser.d.ts`
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/📦️packages/🟨️javascript/🖥️sequence-host.js`
- `✏️s/🔌️plugins/🎬️sequence/🧪️tests/🌐️browser-consumer/🟨️.js`
- `✏️s/🔌️plugins/🎬️sequence/🧪️tests/🔮️protocol-oracle/🟨️.js`
