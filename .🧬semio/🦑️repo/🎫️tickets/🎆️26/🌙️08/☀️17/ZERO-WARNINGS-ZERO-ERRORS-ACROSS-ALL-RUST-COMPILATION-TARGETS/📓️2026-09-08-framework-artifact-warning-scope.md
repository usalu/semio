# Framework Artifact Warning Scope

Pass587 added all seven framework artifact crates to the language-neutral native/WASI fixtures. The expected pre-change failure was recorded. Extending only the existing name filter did not satisfy the fixture because taxonomy package discovery omitted those framework artifact packages. The root script filter and Cargo oracle edits were applied, but post-change validation still failed; no passing receipt was produced.

Pass590 selects shipping Rust packages from the workspace's declared Cargo member manifests, expanding member globs and honoring exclusions. It reads package names and roles from those manifests and includes both S and framework artifacts. The independent cargo metadata comparison remains. This avoids coupling compiler warning coverage to whether an artifact has been registered as a frontend/task project.

Files:

- `📜️script.ts`
- `🧪️tests/🦀️rust-warnings/🔣️.json`

New coverage validation is pending. This change does not establish a warning-free build or runtime behavior.

Pass591: all 6 checks passed, including three target vectors, two rejected targets and the independent Cargo metadata catalog comparison. Coverage: native 164 packages; wasm32-wasip2 160 packages; wasm32-unknown-unknown 2 packages. This validates coverage policy, not compilation.

Pass593 adds the four native plugin support crates omitted by the shipping component role filter: Draw FSM and its macros, Jack LSP and Jack Shell. Their required native fixture initially failed; after enabling support-crate selection only for native checks, all 6 target/catalog checks passed. Scope: native 168, wasm32-wasip2 160, wasm32-unknown-unknown 2. WASI retains shipping components and artifacts. Actual compilation remains pending.
