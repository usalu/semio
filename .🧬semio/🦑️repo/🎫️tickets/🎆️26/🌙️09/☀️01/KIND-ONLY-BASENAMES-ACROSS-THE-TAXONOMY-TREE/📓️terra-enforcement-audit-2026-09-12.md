# Independent Enforcement Audit

## Scope

Read-only audit of the focused implementation-leaf and inverse-package-target guard in `🔍️discovery/🟦️.ts`, its schema, normalizer integration, portable fixture, and command wiring. The audit used the live concurrent tree. `git diff HEAD`, rather than the index diff, was used because other agents stage work in this workspace.

## Evidence

- The registered target `@semio-tech/repo-lib:test-kind-only-basename` passed in an isolated Nx state: 4 tests, 34 expectations, 28 materialized files, and 17 findings. Its filesystem census is independently checked with `fast-glob` and Ajv const predicates. See `🗑️generated/terra-enforcement/kind-only-basename-test.log`.
- The root focused report completed successfully in report mode. It visited and classified 137,438 paths and emitted 530 current findings; report mode intentionally exits successfully while findings remain. See `🗑️generated/coordinator/implementation-report.log` and `🗑️generated/coordinator/nx-implementation-findings-wave1.json`.
- Direct API adversarial cases confirm target-first detection, root and nested generated/cache glob matching, and no generic hidden-directory exclusion. See `🗑️generated/terra-enforcement/adversarial-api.json`.
- A direct current package discovery found 173 `package-implementation` and 161 `package-role-unresolved` diagnostics, while the existing package-purity policy filters both out. The sampled evidence is in `🗑️generated/terra-enforcement/package-body-census.json`.

## Resolved Finding

The initial focused report misclassified the TypeScript declaration leaf `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧩️runtime/🟨️.d.mts`, asking for `🟦️.mts`. The schema had `.d.ts` but omitted `.d.mts` and `.d.cts`; longest-extension matching therefore discarded the declaration suffix.

The enforcement lane corrected this by registering `.d.mts` and `.d.cts` in `typescript-source`, adding their physical resolution rules, and extending the portable fixture with canonical and named cases. The independently run registered test above verifies the resulting canonical names `🟦️.d.mts` and `🟦️.d.cts` through both the implementation census and the independent oracle.

## Open Findings

### P0 — Broad generated-directory skip conceals generated implementation leaves

`implementationLeafPolicy.ignoredPathPatterns` includes `**/🤖️generated`, causing the focused census to skip every such directory before it classifies files. This is a generic directory-name exemption, even though the taxonomy's generator contracts distinguish individual output roots and their tracked/ignored status.

An independent census of the live, declared generator output roots found 332 files, including 47 implementation leaves. Twenty-four do not have a kind-only basename and are therefore hidden from the focused guard: for example `🦀️icon_name.rs`, `🦀️registry.rs`, `🟦️actor.ts`, `🟦️tokens.generated.ts`, and generator-owned CSS. The UI WGPU output `🤖️generated/🦀️.rs` is already canonical, which demonstrates that generated status and anonymous-leaf status are independent. See `🗑️generated/terra-enforcement/declared-generated-source-census.json`.

The exact hidden leaves are:

- `assets-build`: `🧰️framework/🔨️modules/🖼️assets/🔣️icons/🤖️generated/🦀️icon_name.rs`, `🔤️shortcodes.ts`, `🐍️icons.py`, `🔷️Icons.cs`, and `🟦️icons.ts`; plus `🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🔣️icons/🤖️generated/🟦️metabolism_icons.ts` and `🦀️metabolism_icon_name.rs`.
- `actor-typegen`: `🧰️framework/🔨️modules/🎭️actor/🤖️generated/🟦️actor.ts`.
- `ui-contract`: `🧰️framework/🔨️modules/🛂️manifest/🤖️generated/📜️ui-contract.ts`; `framework-manifest`: `🧰️framework/🔨️modules/🛂️manifest/🤖️generated/🪪️manifest.ts`; and `ui-axes`: `🧰️framework/🔨️modules/🛂️manifest/🤖️generated/🎚️ui-axes.ts`.
- `graph-catalog`: `🧰️framework/🔨️modules/🕸️graph/🤖️generated/🦀️registry.rs` and `🔠️types.ts`.
- `schema-entity-catalog`: `🧰️framework/🔨️modules/🧬️schema/🤖️generated/🟦️entity-kinds.ts`; and `async-typegen`: `🧰️framework/🔨️modules/⏳️async/🤖️generated/🟦️async.ts`.
- `styling-tokens`: `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🤖️generated/🚦️palette-presence.css`, `🔤️palette-fonts.css`, `🌓️palette-theme.css`, and `🟦️tokens.generated.ts`.
- `plugin-registry`: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🏗️framework.ts`, `🖥️hosts.rs`, `🎮️playgrounds.ts`, `🧩️plugins.ts`, and `🗿️artifacts.rs`.

Generated code is implementation and must obey the same physical-leaf rule. Replace the broad `🤖️generated` skip with exact compiler/cache output paths only, then update each generator's declared output topology, producer, references, and fixtures to generate domain-owned directories with kind-only leaves. Do not turn these named outputs into blanket fixed filename exemptions.

### P2 — Cargo build-script admission is lexical, not manifest-bound

`implementationLeafBasenameFinding("🧰️owner/📦️packages/🦀️rust/build.rs")` returns no finding even when that path is not accompanied by `Cargo.toml`. The implementation supplies `packageRoot: true` from the `📦️packages/🦀️rust` path shape; it does not establish a Cargo package. The same condition is used by the focused census. This is weaker than the declared `cargo-build-script` authority and does not prove an exact external contract for a bare directory.

The portable fixture contains a valid build-script path but no counterexample without a manifest. Resolve the semantic choice before treating the guard as exact: either bind this admission to a discovered package manifest and add the negative fixture, or explicitly define the taxonomy term `package-root` as a lexical boundary that deliberately needs no manifest. The latter still leaves build-script authority unverified by the focused guard.

### P1 — Package-body purity is not enforced by this command

This is a scope limitation, not a reason to weaken the focused filename checker. `discoverPackageProblems` classifies substantive bodies as `package-implementation` and uncertain glue as `package-role-unresolved`; `policyPackageLanguagePurityBreaches` in root `📜️script.ts` retains only `packaging-violation` and `unknown-lang`. The live direct discovery counted 334 omitted diagnostics (173 implementation, 161 unresolved), including the repo library package leaf itself.

Consequently, a clean focused implementation-basename report would not establish the goal's broader rule that substantive code belongs to domain owners outside language package roots. Preserve this as a separate package-body migration and policy-enforcement lane with a thin-wrapper fixture; do not fold it into basename exemptions.

## Boundary Review

- The census filters only the schema-declared opaque paths (`compose/`, `temp/compose/`, and `♻️mit-bestand/🔎️recherche/`) before filesystem access. It does not silently omit `.cursor`, `.devcontainer`, `.storybook`, or `.agents`; the current report contains 2, 6, and 83 findings respectively for the first three hidden roots.
- The `target-*` skip is supported by the current tree: its 16 matching directories are Cargo build-cache `target-lexicon-*` descendants, with no tracked source. See `🗑️generated/terra-enforcement/target-dash-directories.txt`. The broad `🤖️generated` skip is not supported by the same evidence and is the P0 issue above.
- The 53 `temp/merge` findings are from a Git-ignored temporary tree (the repository `.gitignore` ignores `temp`; observed modification time 2026-09-02), not from any declared opaque path. The tree contains merge material and duplicated Mit-Bestand source. It must remain visible until a responsible lane moves it under governed ticket generated material, adopts it into the taxonomy, or obtains an exact schema-backed boundary. A blanket `temp/**` exclusion would conceal authored inputs and conflicts with the current explicit-only opaque policy.

## Passed Coverage and Limits

The focused guard correctly enforces known implementation-kind basenames, CSS/HTML inclusion, longest registered extension selection, fixed-contract scope handling, cancellation, deterministic no-follow traversal, and the direct inverse `📦️packages/<language>/🎯️targets` boundary. It does not classify unknown source extensions as a finding; that remains the full taxonomy validator's responsibility. It also does not prove that package files are thin glue, nor does it validate compiler/package semantics for a fixed name beyond its current lexical scope.
