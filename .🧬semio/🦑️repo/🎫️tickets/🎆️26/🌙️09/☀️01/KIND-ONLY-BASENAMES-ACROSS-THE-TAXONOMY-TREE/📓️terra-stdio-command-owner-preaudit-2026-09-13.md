# Stdio Command Ownership Preaudit

## Status

Read-only review of the schema-first plan. No Stdio build, Cargo metadata, Nx graph, package installation, service, or package mutation was run. The current root script still contains the command implementations, so this is a pre-extraction boundary review only.

## Current Ownership Problem

`✏️s/🔌️plugins/🗄️stdio/📜️script.ts` contains artifact definition discovery, schema/dependency validation, Cargo/Nx graph inspection, TypeScript composition build and declaration checks, the five composition command classes, the two root proof command classes, and two exported forwarding facades. The composition package router is only:

```ts
import { runStdioCompositionPackageMain } from "../../📜️script.ts";
await runStdioCompositionPackageMain(import.meta.dir, import.meta.url);
```

That imports implementation from the mandatory parent. The proposed seven-owner split is the correct boundary: artifact inventory, artifact contract, artifact graph, artifact proof commands, composition build, composition commands, and test orchestration. Inventory must remain the sole reader/admitter of 36 definition leaves; contract consumes its admitted projection; graph consumes the contract rather than independently rediscovering the filesystem. Composition build must own Bun/declaration/private-consumer output logic and must not absorb Cargo/Nx validation.

## Router And Consumer Closure

The direct composition package has five existing commands: build, check, test, package-contract, and package-graph. Its final router should directly import the five semantic command classes, create one ScriptRouter, and use one existing `runBundleScriptMain` terminal with default test. The Stdio root router should directly import only the two proof command classes and use the same standard terminal with default package-contract. These are existing generic routing primitives; no Stdio-specific terminal grammar or facade is justified.

A whole-workspace source search finds `runStdioCompositionPackageMain` only in the current composition router and root definition. `runStdioTypeScriptArtifactPackageMain` currently has no external caller. The source control must prove that zero-consumer state before its removal and separately prove every artifact TypeScript router already calls the domain-neutral `runArtifactTypeScriptPackageMain`. It should not turn an unused facade into a compatibility shim.

The test factory at `✏️s/🔌️plugins/🗄️stdio/🧪️tests/📦️artifact-package-graph/🟦️.ts` is a test concern. Dependency injection is permitted inside that test factory where it improves isolated controls; the mandatory root router must not supply its implementation ports. The final test owner may instead directly import the semantic contract and graph owners, but the acceptance rule is the absence of root-to-test implementation coupling, not a blanket ban on test ports.

## Required Source Data And Inputs

The contract/inventory closure must include all 36 `📜️artifact-definition.json` leaves and each artifact package manifest/project/router that the contract projects. The graph owner additionally reads root Cargo.toml, Cargo.lock, root package metadata, and the Nx graph. Composition build reads its own barrel and package metadata plus the 36 declared TypeScript package exports. The target input equality must include those real reads, the Stdio command-ownership schema/fixture/test, all seven owners, both routers, project/package manifests, and seed plus generated launch registrations.

The plan correctly separates source-data reads from producer execution: Cargo metadata and Nx graph are bounded proof commands; Bun build/declaration/consumer compilation is composition build; no proof route may silently publish the live composition dist directory. Private ticket-local roots are required for the planned compilation checks.

## Required Acceptance Controls

- Installed Ajv validates schema and fixture, including hostile owner path, missing/linked/unreadable definition, duplicate identity, stale command/facade, dynamic import, local command class, type-only import, and second-terminal cases.
- Inventory performs component-wise no-follow admission. A missing definition is distinct from unreadable or linked input; any non-missing issue rejects the whole contract without a partial 36-package result.
- Contract proves exact canonical identities, declaration-only package roots, Bun/Cargo workspace membership, dependency DAG, and composition isolation. It must not snapshot root-script body text or use a body hash.
- The native/registered package-contract result must state whether Cargo metadata ran. The package-graph route is only needed if contract evidence does not prove actual Nx edges.
- Private composition build/declaration and consumer compilation may prove the 36 exports, but neither establishes artifact runtime behavior.

## Limits

No current evidence proves the proposed source split, Cargo metadata, Nx graph, composition compilation, or an artifact package runtime. This preaudit does not claim a Stdio service, artifact probe, package publication, full application build, or native artifact execution.

## Route Name Clarification

A read-only current-source search confirms that Stdio's root `package-contract` and `package-graph` commands are distinct from the repository-library cache command also named `artifact-package-contract`. The cache route is registered by `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📜️script.ts` and implemented by its existing `📦️artifacts/📋️package-orchestration/🟦️.ts`; it validates generic artifact package boundaries and is not an implementation or facade consumer of Stdio's command router. The Stdio control must record the two actual Stdio direct routes separately and must not claim that the cache command belongs to the Stdio owner tree merely because the command names overlap.

The same search found exactly one external consumer of `runStdioCompositionPackageMain`, the Stdio composition package router, and no external consumer of `runStdioTypeScriptArtifactPackageMain`. This is static pre-extraction evidence only; it establishes the required zero-consumer proof before deleting the latter facade and the one-router rebind required for the former.

## In-Flight Admission Finding

The first extracted inventory owner is present, but its current `admittedDirectory(stdioRoot)` and `admittedDirectory(artifactsRoot)` calls inspect only final paths. `lstatSync` resolves linked ancestors before it receives those final paths, so a linked ancestor between the physical repository root and either admitted directory remains followed. This does not satisfy the planned component-wise no-follow property. Acceptance requires a physical-root-to-leaf segment walk and a fixture that places a link in a non-final ancestor while preserving a syntactically valid catalogued definition path. The current final-directory, catalog-member, missing, unreadable, and unlisted-child checks do not close that ancestor boundary.

## In-Flight Legacy-Consumer Scan Finding

The extracted contract owner uses `rustSources(repoRoot)` to support an absence assertion for legacy `semio_s_plugin_stdio::artifacts::` Rust consumers. Its recursive `readdirSync` traversal skips symbolic-link directory entries instead of rejecting them. A linked Rust source can therefore be omitted from the negative consumer assertion. Before accepting this source-as-data proof, the scan must fail closed for links and unreadable traversal, or operate over a separately admitted physical source inventory. This is distinct from the repaired artifact-definition admission boundary.
