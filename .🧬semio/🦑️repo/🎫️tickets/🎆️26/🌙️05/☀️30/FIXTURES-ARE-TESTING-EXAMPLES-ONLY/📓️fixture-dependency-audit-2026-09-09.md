# Fixture Dependency Boundary Audit

Date: 2026-09-09  
Scope: resolved production, build, packaging, and cache dependency paths into `🧫️fixtures`; fixture-layout enforcement in the repo test platform. This is a read-only audit of the live shared worktree.

## Boundary used by this audit

A fixture is a test example only when it has a canonical semantic owner, lives below that owner's `🧫️fixtures`, is outside every `🧪️tests` case directory, and is reached only by a test or an explicitly test-only check. A source-looking file below `🧫️fixtures` is not automatically an error: it can be opaque input/output data for a test. It becomes executable support or build source when a package manifest, project definition, script entry, compiler target, static-asset manifest, or resolved non-test consumer treats it as code or configuration.

`🖼️assets` is the correct owner for production static data. A production asset manifest rooted at `🧫️fixtures` is a production fixture dependency even when it only serves bytes; it must point at the semantic owner's `🖼️assets` instead.

The previous physical census remains available in [📓️global-fixture-asset-audit-2026-09-09.md](📓️global-fixture-asset-audit-2026-09-09.md). This report verifies live resolved edges and does not reclassify a JSON schema or a textual path mention as a fixture dependency.

## Confirmed production or build graph edges

| Consumer and evidence | Resolved fixture target | Classification and required action |
| --- | --- | --- |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/🦀️.rs:51-52` unconditionally declares `#[path = "../../🧫️fixtures/🦀️.rs"] pub mod fixtures;`; `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🦀️.rs:20` reexports it. The package manifest declares product role at `.../Cargo.toml:8-10`. | `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧫️fixtures/🦀️.rs`, which has `include_str!("🧪️eval/🔣️.json")` at `:425`. | **Confirmed production compilation and transitive include edge.** Move the Rust support module out of `🧫️fixtures` into a test-only support target/module. If the JSON remains a fixture, resolve it only under `cfg(test)` or an independently test-only target. Remove the product reexport. |
| Root `Cargo.toml:189` makes `🧰️framework/🛍️products/💻️os/🧫️fixtures/⚖️scale/📦️packages/🦀️rust` a workspace member; `Cargo.toml:361` exposes it in `[workspace.dependencies]`. The fixture package's `Cargo.toml:15` declares `role = "testkit"`. | Scale fixture Cargo package and its `Cargo.toml`, `Cargo.lock`, Rust source, and `📜️script.ts`. Its script runs Cargo check/build at `.../📜️script.ts:14,20,27-32`. | **Confirmed workspace/package build node and dependency exposure.** This audit did not prove a normal product crate links it, but it is executable testkit source under a fixture path and can be built through the root workspace. Move it to a test-support semantic owner outside `🧫️fixtures`; retain only opaque Scale examples in the canonical owner fixture directory. |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts:5400-5419` fixes the JCO guest destination beneath OS fixtures and reads Cargo manifest/lock, component source, and WIT. Root `📜️script.ts:754-762` calls `semanticPackageAdapterPreview` and writes the adapter there. | `🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/👽️guest`, including its Cargo package and WIT/component sources. The fixed destination is also declared at `🧰️framework/🛍️products/💻️os/🧬️schema/🔣️.json:1647-1653`. | **Confirmed generator/build ownership edge.** This is build source, not an opaque fixture. Relocate the guest package to a generator or test-support semantic owner, then update the schema contract, discovery path, and root generator together. |
| `✏️s/🔌️plugins/📐️cad/📦️packages/🦀️rust/Cargo.toml:24` gives Storybook a fixture source root; `:36-40` configures it as a Semio static directory at `/cad-fixture`. | `✏️s/🔌️plugins/📐️cad/📚️examples/🧫️fixtures`. | **Confirmed product static-asset configuration edge.** Rehome runtime CAD data into its semantic owner's `🖼️assets`, then update Storybook and Semio asset configuration. If the files are only examples for tests, remove them from the product static manifest instead. |
| `✏️s/🔌️plugins/🎪️demonstrator/📦️packages/🦀️rust/Cargo.toml:81-85` serves the CAD fixture tree, and `:93-97` serves the OS Infinite fixture tree. `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/Cargo.toml:64` also serves the OS Infinite fixture tree. | CAD fixture root above; `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🧫️fixtures`. | **Confirmed cross-product static-asset edges.** Move actual runtime data to canonical `🖼️assets` owners and repoint each plugin manifest. A generic fixture-path exclusion would conceal these runtime routes. |
| `🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/📋️project.json:9` includes OS fixtures in the default Nx named input. | OS fixture tree. | **Confirmed cache-input edge, not runtime consumption.** Split test fixture inputs into a named input used by test/check targets only; ordinary build target hashes must not depend on fixtures. |

## Rust `include_str!` / `include_bytes!` results with compilation context

The scan resolved literal include arguments against the caller's directory and considered only targets physically below `🧫️fixtures`. A caller path alone is insufficient; the surrounding cfg and target invocation determine whether a result is production-reachable.

| Caller | Target | Context | Result |
| --- | --- | --- | --- |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧫️fixtures/🦀️.rs:425` | `.../🌉️mcp/🧫️fixtures/🧪️eval/🔣️.json` | The caller is included unconditionally by the product package entry at `📦️packages/🦀️rust/🦀️.rs:51-52`. | Production-transitive edge; fix with the MCP module move described above. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:3263` | sibling `.../durable-group/🧫️fixtures/🔣️.json` | `#[cfg(feature = "testkit")]` guards the function. | Conditional testkit edge. Permit only after the feature is shown absent from every production build invocation; otherwise rehome it. The static gate must record the cfg, not blanket-pass it. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🦀️.rs:649` | `.../retained-command/🧫️fixtures/🚪️raw-allocation-close.json` | `#[cfg(test)]` guards the function at `:647`. | Test-only direct include. This is allowed once the fixture is canonically owned and outside test case directories. |

No other literal Rust include found in this audit established a non-test consumer. Includes in `🧪️tests` callers are test reads rather than production edges.

## Executable or configuration candidates beneath fixtures

These entries must remain visible to the layout/dependency checker. Their presence is not, by itself, a violation; the listed consuming edge decides the result.

| Fixture member | Why it cannot be hidden by a path exclusion | Current consumer classification |
| --- | --- | --- |
| `.../🌉️mcp/🧫️fixtures/🦀️.rs` and its evaluation JSON | Rust module entry and compile-time include. | Confirmed product edge. |
| `.../🧫️fixtures/⚖️scale/📦️packages/🦀️rust/{Cargo.toml,Cargo.lock,🦀️.rs,📜️script.ts}` | Cargo project plus build/check script. | Root workspace node; move to test support. |
| `.../🧫️fixtures/🧩️jcoprobe/👽️guest/{Cargo.toml,Cargo.lock,📚️library/🦀️.rs,🧩️component/🦀️.rs,🧬️schema/📜️world.wit}` | Cargo/WIT package generated and read by the repo root script. | Confirmed generator/build source; move. |
| `.../🔌️plugin/🌐️browser-bundle/🧫️fixtures/🌊️actor-import/{📋️project.json,📜️script.ts,👽️guest/📦️packages/🦀️rust/...}` | Nx project, script, and Cargo guest package. `📜️script.ts:7` imports a test source. | Current resolved consumer is test-only: `.../🧪️tests/🌊️actor-import/🟦️.ts:10,39-40` resolves the fixture and builds it. It remains executable test support and should move outside fixtures; it is not proven a production runtime dependency. |
| Store durable-group, SPR command/testkit, and plugin mutation fixture Rust trees, including `.../🏪️store/🧫️fixtures/🦀️.rs`, `.../📡️spr/🎮️command/🧫️fixtures/.../🦀️.rs`, and `.../🔌️plugin/🧫️fixtures/.../🧬️mutations/*.rs` | Rust sources can be imported, compiled as fixture packages, or be opaque test subjects. | Candidate only in this audit; require resolved consumer analysis before labeling a violation. |
| `🧰️framework/🔨️modules/🌱️value/🗂️ordered/🧫️fixtures/📜️script.ts` | Test runner source (`Ajv`, `Bun.file`). | Executable test support; move outside fixtures. |
| `🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/📜️script.ts:24,30` and `🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/.../📦️packages/🦀️rust/📜️script.ts:12-15` | Dynamically import fixture scripts from `SourceTestScript` routes. | Resolved test-source/check route; move support code but do not report a production dependency without a non-test target. |
| `🧰️framework/🛍️products/🦑️repo/🖼️assets/🧫️fixtures/...` source-shaped snippets | Located under assets, so they may be static payload. | No resolved import/build edge in this audit. Keep as assets only if they are active production payload; otherwise classify as test fixtures and move. Their filenames/content are not evidence of compilation. |

## Test-platform enforcement gap

The live repo test platform is `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts`.

- `resolveFixtures` at `:1005-1035` resolves `shared://` to the discovered semantic owner's `🧫️fixtures` and `asset://` to its `🖼️assets`, rejects traversal, requires a real regular file, rejects symlinks, and hashes the resolved file. This is the correct execution-side ownership boundary.
- The URI recognizer at `:995` still accepts `local://`, while the resolver scope type contains only `shared`, `asset`, and `schema`. A local URI therefore degrades into a missing fixture failure. Replace this with an explicit forbidden-local-fixture diagnostic so a fixture under a test case cannot be silently normalized as a generic lookup error.
- `fixtureLayoutFindings` at `:1761-1772` correctly reports a fixture in a test case and a fixture below a delivery scope.
- `inspectTestLayoutSource` at `:2038-2052` first records those physical fixture findings, then returns before reading or inspecting any source below every fixture directory. `scanTestLayout` also supplies an empty source string for an initial fixture path at about `:2105`. This hides Cargo manifests, `📋️project.json`, `📜️script.ts`, build scripts, and entry modules under a fixture tree. It would hide the confirmed MCP, Scale, JCO, and browser-bundle forms above.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟨️.mjs:73-96` deliberately hashes owner fixtures as separate test inputs and excludes them from the broad owner input. That cache exclusion is appropriate only for duplicate hashing; it is not a source-dependency safety check. Asset URIs remain covered by the owner broad input.

The checker should inspect the structural members of every fixture tree before returning: manifests (`Cargo.toml`, `package.json`, `📋️project.json`, `nx.json`, `tsconfig*.json`), executable scripts (`📜️script.ts`, `build.rs`), package roots/entry files, and static asset configuration. It should not forbid arbitrary `*.rs`, `*.ts`, or `*.json`: an explicitly declared opaque test subject can have any extension. A discovered executable/configuration member needs a distinct finding such as `fixture-executable-member` or `fixture-build-config-member`, unless its manifest identifies it as opaque data and all resolved reverse consumers are test-only.

## Recommended enforcement shape

1. Build a resolved-edge inventory before applying any fixture-path exclusion. Normalize absolute paths and classify every endpoint by physical taxonomy, semantic owner, and execution role.
2. For Rust, collect package targets with Cargo metadata and the actual Nx/Bun Cargo invocations. Resolve literal `include_str!` and `include_bytes!` relative to their callers, carrying `cfg(test)`, `cfg(feature = ...)`, target kind, and enabled feature set. Mark conditional edges unresolved until the production invocation proves them absent. Emit a finding for nonliteral or macro-constructed include paths that cannot be resolved.
3. For TypeScript and JavaScript, resolve static `import`, `export ... from`, `require`, dynamic `import`, `Bun.file`, `readFile*`, `copyFile*`, `cp`, `new URL`, `join`, and `resolve` expressions when their path arguments are statically evaluable. An unresolved dynamic path from a build or production script must be rejected or be covered by a narrowly scoped test-only declaration.
4. Parse configuration as graph edges too: Cargo workspace members/dependencies, package/project manifests, Nx named inputs, copy globs, Storybook roots, and Semio static-dir roots. A glob or manifest root matching `🧫️fixtures` is an edge even without a language import. A schema is only a schema until a configuration loader resolves it as an input.
5. Fail a production/runtime/build edge that terminates beneath `🧫️fixtures`, whether direct or transitive. Permit test/testkit edges only when the target is canonically owned, outside `🧪️tests`, and no production configuration exposes it. Fail fixtures below test cases independently of edge analysis.
6. Keep executable test support outside fixture directories. The fixture directory can retain opaque compiled input/output data. Where test support needs source-shaped data, declare it as an opaque test subject in a test-owned manifest and verify every reverse reader is test-only.
7. Split Nx inputs so test/check targets include shared fixture files, while ordinary build targets do not. Do not use named-input exclusion as the dependency gate; run the resolved-edge inventory for every target class.

## Limitations and follow-up evidence needed

- This was a read-only source/configuration audit; no source, manifests, generated outputs, or build commands were changed or run.
- Rust results cover literal `include_str!`/`include_bytes!` calls. Macro-generated paths, build-script generated includes, and cfg feature activation need compiler/Cargo invocation evidence for complete closure.
- TypeScript/JavaScript results cover statically evidenced imports, filesystem reads, and manifest roots inspected in scope. Computed paths and package-manager side effects require the proposed resolver to produce a complete target-specific graph.
- The shared worktree was changing during the audit. All paths and line numbers are current observations on 2026-09-09; re-run the graph inventory after relocation work before making it a blocking check.
- No full reverse-consumer closure was performed for every source-shaped fixture payload. Candidate entries are intentionally not labeled production dependencies unless an actual non-test consumer was resolved.
