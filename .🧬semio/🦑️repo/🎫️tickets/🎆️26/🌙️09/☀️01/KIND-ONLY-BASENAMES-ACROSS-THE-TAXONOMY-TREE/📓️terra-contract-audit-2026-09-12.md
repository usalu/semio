# 🌳️ Terra Contract Audit — 2026-09-12

## Decision

The canonical authored-source shape is:

```text
<semantic owner>/<semantic concern>/<file-kind emoji>.<registered extension chain>
```

A target is a semantic concern and must occur before a package boundary:

```text
<owner>/🎯️targets/<target>/<semantic concern>/🦀️.rs
<owner>/🎯️targets/<target>/📦️packages/🦀️rust/📚️library/🦀️.rs
```

The second path is package glue only. Therefore the requested counterexample has this disposition:

```text
.../📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🔁️reconcile.rs  # reject
.../🎯️targets/🧊️wgpu/🔁️reconcile/🦀️.rs                  # authored source
.../🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/🦀️.rs  # package glue
```

`🔁️reconcile.rs` carries a semantic stem in a leaf and is invalid even after its directory order is corrected. The semantic concern is `🔁️reconcile`; `🦀️.rs` is the anonymous Rust leaf.

## Verified Authority

[`🔣️taxonomy.json`](../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json) is already the schema-first authority:

- `physicalLeafRendering.filename` is `file-kind-emoji-and-extension-chain`.
- `_treePurityComment` assigns semantic concerns to registered emoji-plus-slug directories and admits only exact contracts plus proven glue roles at package roots.
- `canonicalFilenamesForKind` and `canonicalLeafFilenameForSourcePath` in [`🔍️discovery/🟦️.ts`](../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts) render `🦀️.rs`, `🟦️.ts`, and compound-chain equivalents. The latter explicitly says it does not authorize an owner or move.
- `fixedFilenameContracts` and `configurableEntryContracts` are the narrow exception mechanism. External names such as `Cargo.toml`, `package.json`, `build.rs`, and `vitest.config.ts` remain legal only where their exact contract applies. Configurable authored entries resolve to kind-only `🦀️.rs`, `🟦️.ts`, or `🟦️.tsx` through a declared manifest/tool configuration source.

This supplies a complete source of truth for an enforcement policy; it must not infer exceptions from a basename alone.

## Enforcement Gaps

### 1. A renderer is present; a repo-wide basename breach is not

The discovery module exposes canonical rendering, but the audited source has no repo-wide policy that walks governed files and compares each non-exempt leaf with `canonicalLeafFilenameForSourcePath`. Existing package purity diagnostics classify content inside a package boundary. They cannot reject a named source leaf outside it, and they cannot see an otherwise legal `🦀️reconcile.rs` as a naming breach.

The policy must emit a distinct stable breach, for example `taxonomy/kind-only-basename`, with `path`, resolved `fileKindId`, actual basename, canonical basename, and the exemption authority when present. Reporting and enforcement must consume the same full finding set; enforcement must fail on a non-empty set.

The matcher must be fail-closed and ordered as follows:

1. Ignore only taxonomy `pathExclusions` and generated/cache roots already excluded by the workspace census.
2. Admit an exact fixed filename contract in its exact scope.
3. Admit an exact configurable entry contract only when its configuration source resolves to the file.
4. Admit registered scoped-file identities and structural collection members only when their owner relationship proves that the leaf itself is collection identity. Assets are the demonstrated case; source files are never made structural merely because they have an emoji prefix.
5. Resolve the longest registered extension chain. If it resolves to a file kind, require exact equality with the canonical kind-only filename. If it does not resolve, emit the existing unknown-file-kind/path finding rather than silently exempting it.

The old basename must not be normalized in memory before the comparison. Doing so would preserve every emoji-prefixed semantic stem and recreate the hole this ticket closes.

### 2. Discovery accepts the forbidden package-first target topology

[`🔍️discovery/🟦️.ts`](../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts) documents `📦️packages/<language>/🎯️targets/<target>` as a supported three-level shape and `scanPackagesDir` implements it by looking for `targetsAbs` below each language directory. The workspace contract also has an installed-shape test for the UI React package using this arrangement.

This is an implementation-first topology. It reverses the required ownership relationship and lets the package boundary own a semantic target. Replace it with discovery of:

```text
<owner>/🎯️targets/<target>/📦️packages/<language>/<manifest>
```

and report `taxonomy/target-inside-package-boundary` for every former package-first path. Do not retain the old reader as a compatibility branch: this repository is greenfield and the rule is a taxonomy invariant. Update `DiscoveredPackage` documentation, scanner tests, and every consumer that assumes a package-owned target.

`packageBoundaryRules` currently lists `targets` among allowed child directory kinds for Rust, TypeScript, Go, and .NET. Remove it from every package language rule. A package boundary may contain its exact contracts and declared glue roles, never a target taxonomy subtree.

### 3. Implementation-specific language containment leaks inside the WGPU package

The correct target-first WGPU root exists at:

```text
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu
```

It also contains an implementation-specific subtree inside its Rust package:

```text
…/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/🟦️typescript/
```

That subtree carries named leaves including `🎞️frame-worker.js`, `🐚️plugin-bridge.ts`, and `🚀️boot.js`. It violates both package glue purity and kind-only source leaves. The taxonomy already registers semantic WGPU target members such as `🐚️plugin-bridge`, `🧵️frame-worker`, and `🚀️browser-boot` in `members-of-wgpu-target`; relocate authored code there as anonymous language-kind leaves. Generated output belongs in its semantic member’s generated facet. The Rust package’s `typescript-language` allowance must be removed after this relocation.

The live React directory remains package-first:

```text
…/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react
```

It is the regression specimen for topology conversion, not a precedent to preserve.

## Required Regression Proof

Create one language-agnostic JSON contract and JSON Schema pair, for example:

```text
📚️library/🧫️fixtures/🌳kind-only-basename/🔣️.json
📚️library/🧬️schema/🌳kind-only-basename/🔣️.json
```

The fixture must describe an actual virtual filesystem and expected findings. Required vectors:

| Vector | Expected result |
| --- | --- |
| `…/🎯️targets/🧊️wgpu/🔁️reconcile/🦀️.rs` | admitted authored source |
| `…/🎯️targets/🧊️wgpu/🔁️reconcile/🦀️reconcile.rs` | `taxonomy/kind-only-basename` |
| `…/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/…` | `taxonomy/target-inside-package-boundary` |
| package root `📚️library/🦀️.rs` containing a thin declaration | admitted glue |
| package root `🦀️reconcile.rs` or substantial implementation | basename breach and/or package-implementation breach |
| `Cargo.toml`, `package.json`, `build.rs`, configured `🦀️.rs` | admitted only in exact declared scope |
| asset collection member `🖼️logo/🖼️.svg` | admitted as structural collection identity |
| compound extension source, such as `🟪️.grammar.semio` | canonical longest-chain result |
| every registered implementation source kind: Rust, TypeScript/TSX, JavaScript, Go, Python, and .NET | identical language-neutral fixture semantics |

The test must first validate the JSON fixture using Ajv. It must then materialize the filesystem and compare policy findings with a separate `fast-glob` census plus an independently constructed Ajv filename predicate from the fixture’s kind/extension records. That oracle must not call `canonicalLeafFilenameForSourcePath`, the production filesystem walker, or the production policy. Compare sorted `{path, breachId, expectedBasename}` tuples. This makes the fixture portable across implementations while meeting the required independent third-party check.

Keep the existing package-language handoff fixture for package language directory identity, but revise it to assert that a language directory can occur only immediately below `📦️packages`; it must no longer authorize `🎯️targets` below it. Add a positive target-first traversal and an inverse-layout rejection to the workspace contract. The real-workspace assertion must require zero basename and topology findings before enforcement is enabled.

## Executor Order

1. **Topology lane:** move the React target to target-first placement, move the WGPU package-internal TypeScript members to their registered target semantic directories, update manifests/imports/Nx roots, remove `targets` and Rust `typescript-language` from package-boundary allowances, and make discovery target-first only.
2. **Leaf enforcement lane:** add the single shared policy to the repository script, feed it into both report and enforce, and use exact fixed/configurable/scoped/structural exemptions only.
3. **Contract lane:** add the JSON fixture/schema, internal policy test, and independent Ajv + fast-glob oracle. Make the real workspace zero-findings assertion the enforcement threshold.

The first and second lanes share topology and package-boundary files; run them concurrently only with the schema as the authority and reconcile the resulting diff before turning enforcement on.

## Audit Limits

This was a read-only audit. No source, test, manifest, or taxonomy changes were made. The coordinator’s separate taxonomy report is the runtime baseline for the current shared worktree; this report identifies the contract gaps and required proof surface.
