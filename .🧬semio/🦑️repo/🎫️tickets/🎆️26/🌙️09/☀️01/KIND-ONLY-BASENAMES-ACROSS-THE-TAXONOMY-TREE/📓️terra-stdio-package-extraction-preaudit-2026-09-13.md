# Stdio Package Contract and Router — Pre-Extraction Map

**Scope:** read-only map for the Stdio TypeScript composition package’s current command body, source-data consumers, test helper, Nx registration, and launch routing. No package, Cargo, Nx, build, graph, or native command was run by this audit. The root coordinator may separately run only the exported pure contract plus Ajv; that result is not evidence produced by this audit.

## Current Mixed Command and Required Owner Boundaries

The parent [`✏️s/🔌️plugins/🗄️stdio/📜️script.ts`](../../../../../../../✏️s/🔌️plugins/🗄️stdio/📜️script.ts) is a mixed implementation body, not a package-router helper:

| Responsibility | Current source | Required extraction boundary |
| --- | --- | --- |
| Artifact package inventory and identity projection | `artifactDefinitionPaths`, `packageRecord`, `stdioArtifactPackageContract` at lines 48–90 | Artifact-package inventory/contract owner. It must remain the canonical reader of the 36 per-artifact `📜️artifact-definition.json` records. |
| Contract structure and source topology | `assertDag`, `assertScriptTargets`, `assertDeclarationOnly`, `assertSourceContract` at lines 93–301 | Artifact-package contract proof owner. It compares declared artifact identities, Bun workspaces, Cargo members, package manifests, and non-composition dependency boundaries. |
| Schema law | `assertSchemaOracle` at lines 303–315 | Schema validation proof owner with the Stdio package schema and accepted/rejected fixture as direct source data. |
| Cargo metadata and graph law | `cargoMetadata`, `assertActualCargoDag`, `assertCargoMetadata` at lines 204–236 and 317–329 | Native metadata/graph proof owner. It remains a separate native seam; source placement or a factory import cannot claim this check ran. |
| TypeScript per-artifact and composition build/check/proof | `buildTypeScriptPackage`, `checkTypeScriptPackage`, declaration consumer probe, `buildTypeScriptComposition` at lines 338–382 | Package build/check/proof owners. These own `dist`, Bun build, compiler declarations, and fresh consumer verification. |
| Command selection | `runStdioCompositionPackageMain` at lines 384–420, `runStdioTypeScriptArtifactPackageMain` at lines 424–442, and the parent’s contract/graph main block at lines 451–463 | Thin package command router(s) composed from direct imports of the extracted command classes plus the existing `ScriptRouter`/`runBundleScriptMain` primitives. `runStdioCompositionPackageMain` and `runStdioTypeScriptArtifactPackageMain` must not survive as compatibility command APIs. |

The existing [`createStdioArtifactPackageTests`](../../../../../../../✏️s/🔌️plugins/🗄️stdio/🧪️tests/📦️artifact-package-graph/🟦️.ts:1) factory is permitted owned test behavior. It calls schema/source/Cargo proofs only when the returned test functions run; importing the factory does not execute them. It may retain explicit dependency-injection ports, provided the mandatory parent/root no longer supplies implementation and the factory’s ports are bound by its own semantic test owner. Preserve the schema/source and native-Cargo separation.

## Source Data and Consumer Closure

The inventory/proof closure is larger than the package project root:

| Source or producer | Direct current consumer | Required registered input relationship |
| --- | --- | --- |
| 36 `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/*/📜️artifact-definition.json` leaves, discovered by directory enumeration | Canonical inventory at parent lines 75–90; source contract; Cargo and Nx graph proofs | Explicit recursive artifact-definition source-data input for every contract/graph target. Preserve source admission: no silent symlink, missing, unreadable, or mutation-as-absence behavior. |
| `✏️s/🔌️plugins/🗄️stdio/🧬️schema/🔣️.json` | Ajv proof at lines 303–313 | Direct source-data input for the schema proof, not an inferred package-only input. |
| `✏️s/🔌️plugins/🗄️stdio/🧫️fixtures/📦️artifact-package/🔣️.json` | Ajv accepted/rejected vectors at lines 310–312 | Direct test source-data input. Global `production` explicitly excludes workspace fixtures, so a cached proof target must name it. |
| `✏️s/🔌️plugins/🗄️stdio/🟦️.ts` and every artifact TypeScript package export | Composition build/check at lines 373–406 | Direct producer inputs for composition build/check. The current package project is two directories below this barrel. |
| Root `Cargo.toml`, `Cargo.lock`, all Stdio artifact Cargo manifests, workspace `package.json`, each artifact package manifest/project | source and metadata contract proof at lines 238–301 and 317–329 | Direct proof inputs. `^production` does not replace this closure when Nx source-file/package analysis is disabled. |
| `artifact-package-graph/🟦️.ts` | Parent factory binding at lines 1–2 and 444–447 | Test-owner source input, with the actual proof owner imports after the split. |

`@semio-tech/stdio-js` currently exposes `build`, `check`, `test`, `package-contract`, and `package-graph` through its package file and [`📋️project.json`](../../../../../../../✏️s/🔌️plugins/🗄️stdio/📦️packages/🟦️typescript/📋️project.json:5). Its only explicit cached build inputs are `production` and `^production`. Nx globally defines `production` from `{projectRoot}/**/*`, and disables source-file and package analysis in [`nx.json`](../../../../../../../nx.json:24). Those project-local patterns cannot establish that the parent command, direct semantic owners, Stdio barrel, recursive definitions, schema, or fixture are cache inputs. The extracted targets need an explicit named input that closes those real reads; no cache correctness claim is supportable until then.

## Bounded Current Prerequisite Evidence

The coordinator independently invoked only the exported `stdioArtifactPackageContract` reader and installed Ajv on the actual Stdio schema and fixture. It reported **36 packages**, **two accepted values** (fixture and live contract), and **two rejected values** in **329 ms**, exit zero. The raw receipt is `🗑️generated/coordinator/stdio-package-contract-prerequisite.json`. This establishes the current inventory projection and schema disposition only. It does **not** execute the factory’s source-contract, Cargo metadata, Nx graph, TypeScript build, declaration-consumer, or native behavior.

## Registered Route and Launch Map

- The package router is the four-line [`📦️packages/🟦️typescript/📜️script.ts`](../../../../../../../✏️s/🔌️plugins/🗄️stdio/📦️packages/🟦️typescript/📜️script.ts:3). It currently imports the mixed parent command façade, which is the source-side failure to repair.
- The package’s Nx project registers build/check/test/contract/graph commands, all through `bun ./📜️script.ts …` as required. The move must retain those exact command identities while rebinding their implementation owners.
- The generated and seed launch files contain a generic `repo:artifact-package-contract` entry, but no `@semio-tech/stdio-js` entry. The generic entry routes the repository caching contract and must not be presented as Stdio package contract execution. Register the actual Stdio executable commands in seed and generated launch configuration after the target/input closure is final.
- The Rust package is an independent producer with broad Stdio Rust named input. It is not evidence that the TypeScript package’s external reads are registered, nor should this extraction change its native execution scope.

## Acceptance Controls Needed

1. A portable inventory control must prove exactly 36 canonical definitions, unique artifact/identity/directory/package names, sorted discovery, and reject missing, duplicate, malformed, unreadable, and symlinked definition candidates without silently shrinking the contract.
2. The Ajv fixture must continue to reject umbrella Rust packages and escaping TypeScript entries while accepting the canonical projection. It is a source-data consumer, not a body snapshot.
3. Direct command-router tests must prove the package router imports semantic command classes, binds `ScriptRouter`, calls only `runBundleScriptMain`, preserves default command and exact five command names, and rejects a local helper, type-only command class, injected I/O, or a second terminal.
4. Registered Nx tests must use isolated current-ticket roots and `--skip-nx-cache` after explicit input closure is registered. Distinguish source-only/Ajv evidence from Cargo metadata, Nx graph, TypeScript build, and consumer-import evidence.
5. The final source graph must have no import back into the package router or the old parent command façade; the test factory imports proof owners, never the router.

**Status:** pre-extraction map complete. The current Stdio package caller cannot be admitted by a special terminal name. It needs actual package contract/build/proof extraction, a standard class-based router, source-data cache registration, and real launch entries before acceptance.
