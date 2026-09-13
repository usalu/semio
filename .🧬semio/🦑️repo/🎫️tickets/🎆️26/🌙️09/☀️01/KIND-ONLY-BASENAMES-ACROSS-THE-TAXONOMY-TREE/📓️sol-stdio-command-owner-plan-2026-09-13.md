# Stdio Command Ownership Execution Plan

## Scope

This lane removes the Stdio TypeScript composition package's dependency on the parent `runStdioCompositionPackageMain` command facade. It also deletes the unused `runStdioTypeScriptArtifactPackageMain` facade; all 36 artifact TypeScript packages already use the shared domain-neutral `runArtifactTypeScriptPackageMain` owner.

The five existing composition commands remain `build`, `check`, `test`, `package-contract`, and `package-graph`. The package router will import command classes directly, register them with `ScriptRouter`, and call `runBundleScriptMain`. The Stdio root router will do the same for `package-contract` and `package-graph`. No Stdio terminal name is added to the router grammar.

## Semantic Owner Map

| Owner | Path | Responsibility |
| --- | --- | --- |
| Artifact inventory | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📇️inventory/🟦️.ts` | Safely admits and sorts the 36 artifact definitions, projects canonical package identities, and rejects missing, linked, unreadable, malformed, duplicate, or silently omitted candidates. |
| Artifact contract | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🛂️contract/🟦️.ts` | Validates the JSON schema and fixture, package declarations, dependency DAG, Bun/Cargo workspace membership, declaration-only package roots, and composition isolation. |
| Artifact graph | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🕸️graph/🟦️.ts` | Runs bounded Cargo metadata and Nx graph projections, then compares actual native/project edges with the admitted contract. |
| Artifact proof commands | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏃️commands/🟦️.ts` | Exports the `package-contract` and `package-graph` `BundleScript` classes. |
| Composition build and proof | `✏️s/🔌️plugins/🗄️stdio/🧩️composition/🏗️build/🟦️.ts` | Owns Bun build/check, TypeScript declaration emission, private consumer compilation, and the exact 36-export composition proof. |
| Composition commands | `✏️s/🔌️plugins/🗄️stdio/🧩️composition/🏃️commands/🟦️.ts` | Exports the `build`, `check`, and `test` `BundleScript` classes. |
| Test orchestration | `✏️s/🔌️plugins/🗄️stdio/🧪️tests/📦️artifact-package-graph/🟦️.ts` | Directly imports the contract and graph owners and exports the two existing test operations without dependency-injected implementation bodies. |

The implementation leaves remain anonymous. `🗿️artifacts`, `📇️inventory`, `🛂️contract`, `🕸️graph`, `🏃️commands`, `🧩️composition`, and `🏗️build` are semantic concerns and receive exact contextual taxonomy chains.

## Schema-First Ownership Control

Add `🧬️schema/🏃️command-ownership/🔣️.json`, `🧫️fixtures/🏃️command-ownership/🔣️.json`, and `🧪️tests/🏃️command-ownership/🟦️.ts` beneath Stdio. The fixture will enumerate the seven implementation owners, two routers, their exact exported command classes/functions, the 36 artifact-definition source-data leaves, external workspace/schema/fixture/test inputs, the package targets, and seed/generated launch entries.

The portable test will prove:

- schema validity and hostile owner/router/command/source-data cases with installed Ajv;
- exact anonymous owner paths, exported bindings, and contextual ancestor chains;
- component-wise no-follow source admission and controlled missing/unreadable failures without a partial contract;
- package router imports the five command classes, constructs one `ScriptRouter`, and calls one `runBundleScriptMain` terminal with default `test`;
- root router imports the two proof command classes and uses the same standard terminal with default `package-contract`;
- neither router contains local command classes, dynamic imports, injected filesystem/process I/O, the removed facade names, or a second terminal;
- exact Nx input equality for every external source-data read, including root `Cargo.toml`, `Cargo.lock`, root `package.json`, recursive artifact definitions/manifests/projects/routers, Stdio schema/fixture/barrel, semantic owners, and test owners;
- actual package scripts and seed/generated launch configurations bind the five existing package targets.

## Execution Evidence

1. Capture the portable ownership first red before moving the command bodies.
2. Run the direct ownership/schema/source-admission control after the split.
3. Run private ticket-local composition build/declaration/consumer checks so no live package `dist` is published.
4. Run the existing `package-contract` registered target with private Nx workspace/cache/temp roots; this is the native Cargo metadata route and must report whether Cargo metadata actually executed.
5. Run the existing `package-graph` target only once if the private registered contract result leaves graph wiring unproven.
6. Run the focused router classifier against both final routers and hostile local-shadow/type-only/dynamic-import/second-terminal cases. Preserve the existing grammar; do not add a product-specific allowlist.

No Stdio service, long-running probe, full application build, package install, Git mutation, lifecycle edit, or compatibility facade is in scope.
