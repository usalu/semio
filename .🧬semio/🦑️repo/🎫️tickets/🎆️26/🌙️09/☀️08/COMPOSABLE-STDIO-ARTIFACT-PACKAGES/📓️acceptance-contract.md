# Package Acceptance Contract

## Required Observable Results

- All 99 production artifact owners have independent Rust packages, with two additional shared contract packages. The 39 existing TypeScript entry points have appropriate package declarations without moving their implementation into package directories.
- Each artifact Cargo library entry resolves to its own domain tree outside `📦️packages`.
- No artifact package depends directly or transitively on the stdio plugin assembly package. Cross-artifact dependencies form a DAG and use shared public types rather than duplicated implementation mounts.
- Shared helpers and schema/assembly contracts have a domain-neutral owner. No dependency on a registry that compiles the entire artifact catalog is introduced.
- The stdio plugin composes artifact dependencies and preserves full-catalog and home-io behavior. Artifact-specific build/check/test can run without compiling the full plugin.
- Nx discovers every package, follows actual Cargo dependencies and hashes the package-neutral implementation tree; editing an unrelated artifact must not invalidate an independent artifact’s inputs.
- Every new executable workflow uses `📜️script.ts`, Bun and Nx, and is available from `launch.json`.
- Build outputs can be restored without overwriting other packages’ artifacts. Existing repo-native cache/task infrastructure is used.

## Validation Strategy

Define a language-neutral schema and fixture with positive and negative cases before changing package ownership. Validate it with an existing third-party schema implementation and compare package dependency expectations with Cargo metadata and the actual Nx project graph. Run selective leaf package compilation, a representative dependent artifact package, home-io and full plugin compilation, and existing runtime/codec tests. Record exact commands, exit status, and meaningful runtime output in ticket markdown. Cache verification should cover a second identical invocation and source ownership/invalidation boundaries.

## Review Concerns

Root `Cargo.toml`, `📜️script.ts`, `.vscode/launch.json`, and some artifact files already have concurrent edits. Re-read before each targeted change; never restore files or mutate Git state. Complete the packaging goal without taking ownership of unrelated changes.
