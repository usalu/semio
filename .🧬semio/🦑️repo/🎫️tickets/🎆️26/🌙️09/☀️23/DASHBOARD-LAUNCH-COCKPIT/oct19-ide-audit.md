# October 19 IDE Authority Audit

Read-only inspection of the current checkout. No source edits or runtime tests performed.

## Current Configurations

`.vscode/launch.json` (8,453,380 bytes) and `.vscode/🧩️launch.seed.jsonc` (5,826,119 bytes) are active again. The expanded file contains 13,043 configurations, 12,138 distinct command strings, 128 inputs and four compounds. Every configuration is `node-terminal` and declares environment values; 358 declare serverReadyAction. There are no native debugger-adapter entries in the expanded file.

The four compound operations are space React + OS MCP, space React + OS hub, two-user space React + OS hub, and architecture course + proctor. These require neutral multi-process composition semantics. The leaf launch commands include workspace/project commands, ticket-defined source acceptance and typecheck commands, generated crate routes, playground launches, and prototype/release variants. Environment and invocation policy matter independently of command strings.

## Active Generator Dependencies

All paths below are relative to repository root.

- `📋️project.json:1840`: generate-launch and generate-launch-check targets dispatch `generate launch` and `generate launch --check`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🏭️generate/🟦️.ts:31`: active generate launch routing, usage text and private generateLaunch implementation imports catalog writer/checker.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/🚀️launch-catalog/🟦️.ts`: reader/writer/checker, authored seed and expanded document types, registry read, dev route expansion, parameter reference validation and crate route registration. writeLaunchSeed and writeLaunchCatalog persist editor files.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/🚀️launch-catalog/🧬️schema/🔣️.json`: explicitly defines editor seed/catalog paths and editor launch semantics.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/🎮️command/🟦️.ts:118`: carve imports registerCrateLaunchRoutes; line 175 calls it. Merely deleting configs leaves carve dependent on editor seed.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🚀️launch-routes/🟦️.ts`: generates crate route entries consumed by catalog. Preserve useful crate action coverage in neutral inventory.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🏗️authoring/🟦️.ts`: public reexports catalog reader/writers/checker.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts:431`: test launch-catalog command registration.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/📦️packages/🦀️rust/📜️script.ts:24`: LaunchScript implements test, generate, check and run. generate/check read editor seed, project ticket commands into it, and rewrite/check expanded catalog. run resolves the ticket command and invokes dashboard; retain execution capability independently of publication.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🌳️command-tree/🚀️launch/🟦️.ts`: projectDashboardLaunch produces editor documents; ticketLaunchCommands and launchParameterArguments provide useful execution-related functionality.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧬️schema/🚀️launch/🔣️.json`: current launch document contract.

## Authority Exemptions and Documentation

- `README.md:776` prescribes F5, seed editing and workspace:generate-launch.
- `🧰️framework/🛍️products/🦑️repo/README.md:184` introduces the active launch catalog; lines 186–196 prescribe seed authority, generation/check commands and adding entries. These are the only active README matches found by repository search for launch.json, launch.seed, generate-launch, F5 and the main IDE config paths. The same repo README already describes VS Code extension as passive navigation/editing with dashboard control.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧬️schema/🧭️authority/🔣️.json:214` exempts launch files from the developer-instructions ban through regex negative lookahead.
- The same schema at line 231 explicitly admits `.vscode`, launch.json and launch.seed.jsonc through ActiveRepositoryPath enum.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🧭️authority/🟦️.ts:22` short-circuits path authority checks for the same entries. Corresponding authority fixtures admit them.

## Tests and Fixtures

33 framework TypeScript files mention launch.json/launch.seed. Many are direct readers and will fail if the config files vanish. Representative owners: dashboard launch tests; library launch-catalog tests; artifact-root-source; build graph; storage-pressure; Cargo replan; Cargo target recipe; Bazel build-tool tests; prepared schema compiler tests; IO control snapshot; replication error; crate boundary tests in OS/UI/DB; plugin owned-output tests; workspace-cleanup stray-process test. Replace editor-registration assertions with neutral command inventory or owner declaration assertions, keeping the substantive compiler/runtime/third-party checks.

Historical frozen-seal-ledger fixture rows explicitly record prior deletion of launch.json/seed. These are historical evidence and should not become live configuration authority. Other fixture paths model rejected editor configs and must remain valid test inputs. Watch exclusion policies containing `.vscode` are generic ignored-root policy, not config generation.

`.cursor/plans` contains many historical plan Markdown documents. It is intentionally allowed by existing authority policy and should be distinguished from Cursor configuration. The product `💻️client/🧩️vscode` is extension source; its `.vscodeignore`, packaging, and integration-test launchArgs are product implementation rather than repository IDE launch authority. Do not delete that product because its name mentions VS Code.

## Neutral Capability Preservation

Capture command identity, owner, argv/cwd, configuration mode, environment/invocation policy, parameter definitions and values, readiness/output discovery, cancellation/progress, and compound process membership before retiring the editor artifacts. Current commands are terminal execution, not debug adapter protocols. Preserve runtime command behavior and input semantics in dashboard command schema/inventory; retire editor-specific presentation, `${input:...}` string substitution, editor launch file generation, and F5 instructions. Validate neutral inventory coverage against authored launch data before deleting it. Avoid an adapter or compatibility layer back to editor documents.
