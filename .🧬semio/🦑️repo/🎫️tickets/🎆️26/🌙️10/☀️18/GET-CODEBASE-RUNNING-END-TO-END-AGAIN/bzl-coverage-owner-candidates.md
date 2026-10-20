# Non-Rust tasks that are uncached only because the manifest says so (owner review)

Source: `C/bzl-coverage-tools/census.ts` over the static whole-graph reconstruction (8,027 tasks). Rule: family is not `cargo`/`wasm`, the task is `uncached` with reason `declared` (`cache: false` in the manifest), its target name is a verification or build verb (`test`, `check`, `verify`, `lint`, `typecheck`, `generate`, `schema`, `contract`, `build`, `package`, `canonical`) and not a host-state verb (`deps`, `setup`, `publish`, `docker`, `clean`, `bench`, `e2e`, `live`, `doctor`, ...).

These are CANDIDATES, not defects: a `cache: false` on a lease, storage or process test of the build tool itself (`repo:test-*`, `framework-process:test-*`) is correct. A candidate becomes a cached Bazel action (or a cached `bazel test` result when it is named `test*` and declares `outputs: []`) once its manifest declares the complete inputs and outputs; the staged adapter already executes it that way.

Total: 93 tasks in 26 projects.

## bun-script (24)

- `@semio-tech/cad-js`: `generate`
- `@semio-tech/framework-actor`: `canonical-architecture`
- `@semio-tech/framework-os`: `canonical-architecture`
- `@semio-tech/framework-os-dev`: `canonical-architecture`
- `@semio-tech/framework-renderer-wgpu`: `canonical-architecture`
- `@semio-tech/mit-bestand-demonstrator`: `build`, `canonical-architecture`
- `@semio-tech/mit-bestand-praesentation-projektetage`: `canonical-architecture`
- `@semio-tech/note-note-js`: `canonical-architecture`
- `@semio-tech/procedural-js`: `canonical-architecture`
- `@semio-tech/raster-js`: `canonical-architecture`
- `@semio-tech/repo-lib`: `canonical-architecture`, `schema-read-inventory`
- `@semio-tech/semio-tech-play`: `build`
- `@semio-tech/ui-react`: `canonical-architecture`
- `workspace`: `build`, `build-canvas-font-assets`, `build-storybook`, `generate-config`, `verify-build-owners-sync`, `verify-canonical-architecture`, `verify-gate`, `verify-layering`, `verify-typescript-project`

## bun-test (43)

- `@semio-tech/dag-dag`: `test`
- `@semio-tech/demonstrator-playground`: `test`
- `@semio-tech/hub-puzzle-retained-test`: `test-native`
- `@semio-tech/hub-space-document-sources-test`: `test-native`
- `@semio-tech/repo-lib`: `test-build-caller-birth`, `test-cargo-workspace-ownership`, `test-cross-platform-bootstrap`, `test-dependency-policy-bootstrap`, `test-fixture-ownership`, `test-go-executable-acceptance`, `test-original-default-preparation-control`, `test-preflight-reference-basis`, `test-registry-catalog-gitlink-boundary`
- `@semio-tech/repo-test-domain`: `test-fixture-generate`, `test-fixture-reproduce`, `test-gc`, `test-inventory`, `test-probe`
- `@semio-tech/trinity-jack`: `test`
- `@semio-tech/trinity-rewriting`: `test`
- `repo`: `test-bazel-storage`, `test-bazel-storage-runtime`, `test-bootstrap-original-invocation-source`, `test-cargo-build-lease`, `test-daemon-journal`, `test-daemon-journal-workspace`, `test-javascript-endpoint-fence`, `test-javascript-installed-paths`, `test-javascript-provider-walk`, `test-javascript-storage`, `test-lease-bootstrap`, `test-lease-queue`, `test-leases-partition`, `test-next-dev-runtime`, `test-next-runtime`, `test-runtime-bootstrap`, `test-runtime-verification-storage`, `test-runtime-verification-storage-runtime`, `test-rustup-toolchain`, `test-storage-budget`, `test-tools-partition`
- `workspace`: `test-build-tool-bazel`, `test-build-tool-bazel-bootstrap`

## go (1)

- `@semio-tech/repo-cli-go`: `test-entrypoint-source`

## tsc (1)

- `@teaching/architecture-course`: `check`

## vite (1)

- `@teaching/architecture-course`: `build`

## vitest (23)

- `@semio-tech/framework-os-mcp`: `test-native-staging`
- `@semio-tech/framework-process`: `test-budget`, `test-cargo-build-lease`, `test-cargo-source-admission`, `test-cargo-test-leases`, `test-compiler-capacity`, `test-compiler-capacity-custodian`, `test-compiler-capacity-inheritance`, `test-descriptor-lifetime`, `test-exact-cargo-storage`, `test-execution-budget`, `test-native-artifact-publication`, `test-native-image-observation`, `test-native-storage-platform`, `test-process-progress`, `test-process-tree-termination`, `test-resource-lease-fairness`, `test-resource-lease-waits`, `test-resource-leases`, `test-runtime-environment`
- `@semio-tech/mit-bestand-demonstrator`: `test`, `test-runtime-catalog`
- `@semio-tech/semio-tech-play`: `test`

