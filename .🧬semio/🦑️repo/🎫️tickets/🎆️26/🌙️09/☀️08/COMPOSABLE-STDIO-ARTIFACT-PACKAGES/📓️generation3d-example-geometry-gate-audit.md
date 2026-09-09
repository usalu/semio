# Generation3D Example-Geometry Gate Audit

Captured at `2026-09-09T16:38:10+02:00`. This was a read-only source, retained-graph, and receipt audit. It did not invoke Nx, Cargo, a build, or a test.

## Result

Generation3D has one correct direct native route that selects and hashes the `example-geometry` target and its two test-only Flow extensions:

```text
bun nx run @semio-tech/procedural-generation3d-rs:test -- --test example-geometry
```

The ordinary unfiltered route is also valid and covers both declared integration targets:

```text
bun nx run @semio-tech/procedural-generation3d-rs:test
```

No Cargo feature argument belongs on either command. `Cargo.toml` declares `example-geometry` without `required-features`; its `brep` and `math` dependencies are ordinary test-only dependencies with `default-features = false`, rather than feature-gated test targets. The geometry binary directly calls both extension registration functions.

The requested focused command is still missing terminal acceptance. It should be the next Generation3D-specific gate because it proves the newly expanded test closure and the actual geometry harness. Parent or other-plugin transitive compilation does not establish this result.

## Route And Cache Contract

- The owner project is `@semio-tech/procedural-generation3d-rs`; its `test` target calls `bun ./📜️script.ts test` and forwards arguments. The generic `📦️artifact package target` launch route accepts that project and target.
- The package router delegates to `runArtifactRustTests` with no configured test features. The shared runner performs a package-scoped `cargo nextest list` followed by `nextest run`; when passed `--test example-geometry`, its filter partition keeps that selector on the build command.
- The retained graph `🗑️generated/nx-root-pdf-native-proof/project-graph.json` records the normalized target as cacheable, serial, and rooted in `nativeTestSources`. It replaces the transitive input with 78 explicit native projects, including both `@semio-tech/flow-extension-brep-rust` and `@semio-tech/flow-extension-math-rust`; both are static project edges. Thus edits to either extension's native sources invalidate this test target, in addition to the Generation3D test owner and its manifest/lock/toolchain inputs.
- The native-input resolver deliberately admits `dev-dependencies` only when resolving a test root, then expands their local closure into `nativeSources` project inputs. The graph evidence and current manifest agree on that contract.

## Prior Plan And Evidence Boundary

The recovery matrix places this owner in the multi-artifact default/component queue, marked “planned or partially attempted; no complete current 17-leaf acceptance.” The two retained `owned-15` default receipts reached `semio-s-artifact-procedural-generation3d` only as a check step, then stopped with upstream compiler errors or `No space left on device` (status 101). Neither receipt shows an `example-geometry` test binary, test execution, or terminal pass. They therefore do not accept this new test-only closure.

No source defect or route/cache gap was found. The sole missing acceptance is the direct focused native test above, under the normal default feature set, after the active queue permits it.

## Evidence

- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/Cargo.toml:53` through `:73`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/📋️project.json:35`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🧪️tests/🧩️geometry/🦀️.rs:85`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/📜️script.ts:15`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts:1698`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs:407`
- `.vscode/launch.json:1060`
- `🗑️generated/nx-root-pdf-native-proof/project-graph.json`
- `🗑️generated/owned-15-default-resume.txt` and `🗑️generated/owned-15-default-post-boundary.txt`
