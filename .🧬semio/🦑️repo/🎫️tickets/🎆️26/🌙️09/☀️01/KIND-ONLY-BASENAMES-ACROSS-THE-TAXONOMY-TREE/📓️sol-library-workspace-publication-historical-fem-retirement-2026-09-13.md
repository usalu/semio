# Library Workspace Publication and Historical FEM Retirement

## Outcome

The bounded workspace-publication implementation now has five anonymous TypeScript leaves in the existing `🗂️workspaces` domain. Discovery, manifest projection, membership comparison, publication, and executable routing have separate owners. Their graph has five internal edges, no cycle, and no edge to the repository-library package or root barrel. The mandatory package command directly imports and registers `WorkspacePublicationScript`; it no longer declares workspace publication or historical FEM regeneration behavior.

The permanent `ticket-important-fem-handoff` regeneration authority is retired coherently from the package router, project targets, taxonomy generator catalog, exact owned-generator inventory, launch seed, derived launch catalog, and plugin-registry launch-order control. The five historical inputs/evidence files remain byte-identical, and the two malformed paths remain absent. Neither historical script was executed.

## Semantic owners

1. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️.ts`
   - `getWorkspaceRoot`
   - `computeWorkspaces`
   - `diffWorkspaces`
   - discovery operations, progress, and cancellation contracts
2. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/📄️manifest-projection/🟦️.ts`
   - `WorkspaceRootDocument`
   - `parseWorkspaceRootDocument`
   - `renderWorkspaceRootDocument`
3. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/⚖️membership-comparison/🟦️.ts`
   - `WorkspaceMembershipComparison`
   - `compareWorkspaceMembership`
4. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/📣️publication/🟦️.ts`
   - `WorkspacePublicationOperations`
   - `WorkspacePublicationResult`
   - `publishWorkspaceMembership`
5. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🏃️execution/🟦️.ts`
   - `parseWorkspacePublicationArguments`
   - `WorkspacePublicationScript`

The schema fixture binds each owner path to its exported declarations, exact direct owner imports, and complete ordered taxonomy context chain. The exact internal edge set is comparison → discovery; publication → discovery, projection, comparison; execution → publication.

## Preserved behavior

- Workspace discovery reads physical `package.json` and `Cargo.toml` leaves, retains the existing `pkg` resolution and duplicate-name law, orders relative paths deterministically, and never traverses directory links.
- Missing paths retain their intentional empty semantics. Unreadable directories/manifests and nonregular or linked manifests surface errors instead of creating a partial membership projection.
- Discovery exposes bounded progress and observes cancellation during both directory and entry processing.
- Root-document projection changes only `workspaces`; package name, scripts, private state, and arbitrary unrelated fields remain unchanged.
- `--check` and `--write` are the only admitted argument sequences. A stale check reports the exact missing/stale/order evidence, throws without invoking the writer, and leaves the document byte-identical. A write publishes the sorted expected membership; a subsequent fresh check performs no write.
- An installed `fast-glob` oracle independently matches the private physical package inventory with link following disabled.
- The package, Nx, seed-launch, and derived-launch route identities are exact for the source gate, read-only membership check, and membership publication.

## Historical retirement

The following generator relationships were removed together:

- router commands `generate-ticket-important-fem-handoff`, `preview-generated`, and `check-ticket-important-fem-handoff`;
- the three same-named repository-library project targets;
- taxonomy generator contract `ticket-important-fem-handoff`;
- its owned-generator inventory member;
- launch `📦️preview🤖️ticket-important-fem-handoff` and plugin-registry order `206.115`.

The generic `preview-generated` target was removed only from this repository-library package. Other projects' preview routes remain present. The replacement at seed order `206.115` is the current workspace membership publication launch.

The owned-generator inventory now exactly equals the live taxonomy-owned set: 18 sorted identifiers, including `playground-session` and `report-actor-network` and excluding the retired FEM authority. The production launch renderer regenerated 61 playgrounds and 59 component launchers from the current seed. The concurrently added `⚖️gate🔌️plugin📦️staging-root` entry remains in both seed and derived catalogs.

The preserved historical evidence is:

| Repository-relative path | Bytes | SHA-256 |
| --- | ---: | --- |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️05/FEM-PLUGIN-MIGRATION-TO-CRATE-AND-TAXONOMY-CONSOLIDATION/🔧️write-handoff.mjs` | 5,411 | `e18760f389273a8db1262c04e79a181e41a26adba2bbb753ac98c6a6d5c3c84b` |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️05/FEM-PLUGIN-MIGRATION-TO-CRATE-AND-TAXONOMY-CONSOLIDATION/🔧️update-handoff-tests.mjs` | 2,428 | `5dab8fd67e876c41b5ac411cf9b5f632bf301bef841a6f92f8e8eefddb971be5` |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️05/FEM-PLUGIN-MIGRATION-TO-CRATE-AND-TAXONOMY-CONSOLIDATION/📋️registrar-handoff.json` | 1,558 | `52e78199d6fc9c6001f70a87f050036adc687aa772182e81852f0a6777b230f0` |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️05/FEM-PLUGIN-MIGRATION-TO-CRATE-AND-TAXONOMY-CONSOLIDATION/📋️registrar-handoff.md` | 4,863 | `7262a2e4c601400531da691d1421f36677c42b085001a72dc1ec2ccabad8733f` |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️05/FEM-PLUGIN-MIGRATION-TO-CRATE-AND-TAXONOMY-CONSOLIDATION/📌️important.md` | 434 | `ff261e39dfde4ced2b7e258ec380d4f12c6d2cbbed8df84f95c178fe423b44eb` |

`📓️important/📝️.md` and `�クレアregistrar-handoff.md` remain absent under that historical ticket.

## TDD and runtime evidence

- The focused source gate initially reported 5 passes, 3 failures, and 94 assertions while the live taxonomy call signature, derived launch, and package-script projection controls were incomplete.
- Final direct package source gate: 8 tests, 123 assertions, 0 failures, 1.51 seconds.
- Final isolated registered Nx target `@semio-tech/repo-lib:test-workspace-publication-source`, with daemon disabled, lane-private workspace/cache roots, and cache skipped: 8 tests, 123 assertions, 0 failures; Bun body 1.82 seconds, Nx target 2.3 seconds.
- Existing owned-generator preview inventory control: 1 selected test, 55 assertions, 0 failures, 1.90 seconds.
- Installed Bun bundler checks after formatting: source gate bundled 322 modules and the package router bundled 289 modules, with zero compile errors.
- Production launch rendering: 61 playgrounds and 59 component launchers, with the three workspace routes present once in seed and derived catalogs and the FEM identifier absent.

One isolated Nx attempt ended before target execution because the `@nx/js` plugin worker did not receive its load message. A fresh private workspace/cache retry passed; the pre-target worker exit is retained as infrastructure evidence rather than a product failure.

## Limits

The ordinary live read-only `workspaces --check` performed no write and stopped on an existing duplicate package identity: `@semio-tech/framework-surface-rs` is declared by both `🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust` and its `🕸️bindings` child. The private duplicate-name control confirms the extracted owner surfaces this condition. This slice does not claim the live root workspace list is fresh.

No live `workspaces --write`, historical FEM generator/check/preview, Git command, service, install, cache reset/prune, cleanup, or publication outside private temporary roots ran. Native no-follow evidence was obtained on macOS. Windows link admission is represented by the same `lstat`-based owner and the cross-platform junction fixture branch, but this run does not claim a Windows runtime result.

## Exact attribution

Created product and contract files:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/📄️manifest-projection/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/⚖️membership-comparison/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/📣️publication/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🏃️execution/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🧱️workspace-publication-source/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🧱️workspace-publication-source/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️workspace-publication-source/🟦️.ts`

Updated implementation, routing, cache-input, taxonomy, inventory, and launch files:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📋️project.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/package.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🏭️owned-generator-preview-inventory/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧪️tests/🚀️launch/🟦️.ts`
- `.vscode/🧩️launch.seed.jsonc`
- `.vscode/launch.json`

Ticket record created:

- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/KIND-ONLY-BASENAMES-ACROSS-THE-TAXONOMY-TREE/📓️sol-library-workspace-publication-historical-fem-retirement-2026-09-13.md`

The five historical files listed above were read and hash-verified, not modified. All temporary logs, compiler output, Nx state/cache, and private test roots were kept under `🗑️generated/sol-library-workspace-publication` and are disposable after this report.
