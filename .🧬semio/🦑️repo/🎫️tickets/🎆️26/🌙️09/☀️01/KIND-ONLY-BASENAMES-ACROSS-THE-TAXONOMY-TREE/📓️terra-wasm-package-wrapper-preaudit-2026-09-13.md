# Wasm Package Wrapper Preaudit

## Status

**Accepted for the bounded wrapper scope.** The two parent wrapper manifests, source contract, package router branch, private artifact context, registered target, and launch route are present. The executor’s final ordinary unset-environment route and isolated cache-skipped Nx route each passed **8 tests / 59 assertions**; the Nx target took 8.3 s (8.2 s critical; about 30.2 s including graph setup). This audit did not rerun them.

No Wasm build, package installation, browser session, or binary instantiation was performed by this audit.

## Current Boundary

`runWasmPackWebBuild` in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts` generates the existing child `pkg/package.json` after it has verified the compiler outputs. The child manifests currently declare the actual package names and four generated companions:

| Package | Child module | Type declaration | Wasm binary | Wasm declaration |
| --- | --- | --- | --- | --- |
| `@semio-tech/framework-actor-rs` | `framework_actor.js` | `framework_actor.d.ts` | `framework_actor_bg.wasm` | `framework_actor_bg.wasm.d.ts` |
| `@semio-tech/puzzle-wasm` | `semio_puzzle.js` | `semio_puzzle.d.ts` | `semio_puzzle_bg.wasm` | `semio_puzzle_bg.wasm.d.ts` |

The current Nx producers remain the existing `wasm` targets. Actor uses project `@semio-tech/framework-actor-rs`; Puzzle's package identity is `@semio-tech/puzzle-wasm`, while its existing producer project remains `@semio-tech/puzzle-plugin`. That distinction must remain explicit in the fixture and project-input assertions.

The proposed parent `package.json` manifests are package metadata and payload authority, rather than implementation source leaves. They should remain adjacent to their generated `pkg` payload and must not duplicate the compiler's generated child manifest. The parent is responsible for stable package exports; the child remains compiler output and is a payload of that one workspace member.

## Source And Consumer Closure

The puzzle editor and stories use relative imports to `📦️packages/🦀️rust/pkg/semio_puzzle.js`. Those sources must retain that physical generated-module target. The producer router currently supplies the generated package metadata to `runWasmPackWebBuild`; any parent wrapper needs to preserve the exact name, module, declaration, binary, and output directory values without moving the compiler command into the parent manifest.

The prepared schema and fixture correctly distinguish:

- parent wrapper owner;
- generated child manifest;
- four generated companions;
- the actual Nx project name and existing `wasm` command/output;
- one repo-library ownership route and its seed/derived launch entries.

The prepared test uses `Bun.resolveSync`, Node `import.meta.resolve`, and TypeScript NodeNext resolution against private installed copies. That proves export resolution. It does not import or instantiate Wasm, build either crate, or assert a browser/runtime session.

## Historical Preconditions — Resolved

The ownership test requires `SEMIO_TEST_ARTIFACT_DIR` at module load. The repo-library router now supplies it through the existing `repoTestArtifactEnvironment` owner, includes that owner in target inputs, and the final ordinary unset-environment invocation passed.

The test allocates a uniquely owned child below the supplied artifact root and deletes only that child. The final control proves caller-root preservation and owned-child cleanup.

The final source contract proves the generated child manifest is suppressed as a workspace payload through the same-name parent plus concrete physical export targets. Generic workspace no-follow and conditional-export controls remain accepted separately and are not duplicated here.

## Acceptance Limits

This acceptance covers anonymous-source ownership around the wrapper, exact package export resolution, generated-payload membership, target inputs, and launch registration. It does not claim Wasm compilation, installation, browser execution, or loading the binary as a Node module.

## Final Route And Cleanup Evidence

The repo-library package router creates the artifact context through repoTestArtifactEnvironment for wasm-package-wrappers. The test creates an owned run child below that root and removes only the child. The ordinary unset-environment invocation proves the router supplies a current ticket-local context, while the final test adds caller-root preservation and current workspace membership: both parent package owners are members and both generated child manifests remain suppressed payloads. The registered source-input and launch controls cover the 25 exact declared inputs.
