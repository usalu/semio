# Library Workspaces and Historical FEM Handoff — Independent Pre-Audit

> **Current status — accepted for the bounded workspace-publication and historical-FEM-retirement extraction.** This report began as a preaudit; its current direct/Nx evidence and live-root limits appear in the final acceptance below. No historical generator, live workspace write, or Git command was run by this audit.

## Historical Pre-Extraction Responsibilities — Resolved

Before the move, the mandatory library package command at `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts:467` still defines `WorkspacesScript`. It reads the root `package.json`, obtains membership through `computeWorkspaces`, compares exact order and membership, reports missing/stale/order-only states, and writes only `workspaces` for `--write`. Its `--check`/`--write` exclusive admission and all diagnostic strings are behavior that must migrate from the package command.

The preserved lower owner `📚️library/🗂️workspaces/🟦️.ts` already owned workspace-root selection, physical manifest discovery, package-name collision detection, `computeWorkspaces`, and `diffWorkspaces`. The publication extraction must import these lower authorities. It must not duplicate the discovery walk, put an implementation back in the package leaf, or turn the lower owner into a general library barrel. Semantic children beside that lower owner should separate document projection, current-versus-expected comparison, publication, and command execution; the final owner names must follow the executor's schema-first map.

This is a present import boundary rather than a cosmetic preference: `📚️library/📦️packages/🟦️typescript/🟦️.ts` is a one-line re-export of `../../🟦️.ts`, and the current router obtains `computeWorkspaces` through that package/root barrel. The extracted workspace execution owner must instead import `🗂️workspaces/🟦️.ts` directly. Its source control should reject an owner-to-package/root-barrel edge.

The same package command still contains `TICKET_IMPORTANT_FEM_*`, `ticketImportantFemHandoffBytes`, and three generate/preview/check classes at lines 510–562. This code reexecutes two dated ticket scripts, copies a historical Markdown document to a malformed parsed name when necessary, and rewrites an important-file token. It is historical report reconstruction, not a current compiler. Its removal must leave the historical inputs and archived handoff evidence untouched.

## Historical byte-preservation checkpoint

I independently recomputed the current SHA-256 values. All five preserved inputs match the intake packet exactly:

| Path suffix | Bytes | SHA-256 |
| --- | ---: | --- |
| `🔧️write-handoff.mjs` | 5,411 | `e18760f389273a8db1262c04e79a181e41a26adba2bbb753ac98c6a6d5c3c84b` |
| `🔧️update-handoff-tests.mjs` | 2,428 | `5dab8fd67e876c41b5ac411cf9b5f632bf301bef841a6f92f8e8eefddb971be5` |
| `📋️registrar-handoff.json` | 1,558 | `52e78199d6fc9c6001f70a87f050036adc687aa772182e81852f0a6777b230f0` |
| `📋️registrar-handoff.md` | 4,863 | `7262a2e4c601400531da691d1421f36677c42b085001a72dc1ec2ccabad8733f` |
| `📌️important.md` | 434 | `ff261e39dfde4ced2b7e258ec380d4f12c6d2cbbed8df84f95c178fe423b44eb` |

The malformed rewritten target `📓️important/📝️.md` is absent. The distinct malformed `�クレアregistrar-handoff.md` path specified by the intake is also to remain absent. The real preserved input is `📋️registrar-handoff.md`; it must remain present and byte-identical.

## Historical Route and Launch Closure — Retired

The FEM generator is registered in the mandatory package router under `generate-ticket-important-fem-handoff`, `preview-generated`, and `check-ticket-important-fem-handoff`. Its project manifest has three corresponding targets at lines 891–916. The generator contract `ticket-important-fem-handoff` in `📚️library/🔣️taxonomy.json:27795` names those targets, two historical executable inputs, and the tracked historical JSON output. The exact generator inventory fixture includes that identifier.

### Historical Inventory Defect — Resolved Before Acceptance

The current taxonomy has 23 generator contracts, of which 19 are owned and expose a preview route. The fixture `🧫️fixtures/🏭️owned-generator-preview-inventory/🔣️.json` currently lists only 17 owned ids: it omits current `playground-session` and `report-actor-network` as well as containing FEM. The existing workspace-contract test compares the fixture's sorted ids to the taxonomy-owned set, so this is already a substantive source-control mismatch. Removing FEM alone from the 17-row fixture would make it worse. The completed retirement must have 18 owned preview contracts and its exact fixture must include both omitted non-FEM contracts while deleting only FEM.

The launch authority is triplicated and must be retired coherently:

- authored seed `.vscode/🧩️launch.seed.jsonc:9758` defines `📦️preview🤖️ticket-important-fem-handoff` with `bun nx run @semio-tech/repo-lib:preview-generated`;
- derived `.vscode/launch.json:11336` contains the matching generated entry;
- the OS plugin-registry launch assertion lists `ticket-important-fem-handoff` at order `206.115`.

Removing only the library code or project target would leave an executable launch that dispatches the generic `preview-generated` target without a historical generator contract. The completed map must instead remove the contract, project targets, router bindings, inventory record, seed record, derived launch record, and registry expected order together, while retaining every other generator contract and launch entry in its present order.

The active workspace targets `workspaces-check` and `workspaces-write` must remain ordinary Bun/Nx routes. Their output and package-root behavior needs private-tree controls; this audit did not run either against the live checkout.

## Pre-Acceptance Controls

The executor's schema/fixture/test should prove:

1. every new workspace owner has an ancestry chain resolved by the live taxonomy; its exports are real and the package router imports the execution owner directly;
2. a private root gives the same membership result through the extracted lower discovery authority and an independent JSON/package-resolution or installed-library oracle, with ordering, duplicate names, missing/stale comparison, unrelated `package.json` fields, and `--check`/`--write` admission preserved;
3. publication operates only in the private root, reports cancellation/progress for discovery or write work, and rejects symlinked or unreadable admission paths rather than silently publishing a partial manifest;
4. all five historical byte hashes and the two absent malformed paths are unchanged before and after extraction;
5. all FEM generator routes and their catalog/inventory/launch records are absent, while the remaining generator catalog validates and all remaining preview entries retain their order;
6. named inputs for the workspace route include the semantic workspace owners and private fixture/test authorities, while no cache/launch input keeps a retired FEM executable relation.

The existing `workspace-contract` fixture already has private discovery vectors, including physical manifest, order, `pkg/`, and duplicate-name coverage. Those are useful lower-owner controls, but they do not by themselves prove the relocated document comparison/publication behavior or the retirement/launch closure.

## Limits

This is read-only source and identity evidence. I did not run the dated scripts, reconstruct the report, alter the historical archive, invoke a live `workspaces --write`, run a generator, or run a registered Nx target. The existing workspace walker still accepts physical package manifests independently of semantic role markers; that separate membership-policy behavior is not broadened or accepted by this extraction.

## Current Source Update — Pending Runtime Evidence

The implementation has since reached the intended five-owner structure:

- `🗂️workspaces/🟦️.ts` for lower discovery and diff;
- `🗂️workspaces/📄️manifest-projection/🟦️.ts`;
- `🗂️workspaces/⚖️membership-comparison/🟦️.ts`;
- `🗂️workspaces/📣️publication/🟦️.ts`;
- `🗂️workspaces/🏃️execution/🟦️.ts`.

Static reinspection confirms that the source fixture and source test now cover the earlier gaps: exported declarations, exact owner-to-owner import graph, ordered taxonomy contexts, the mandatory router's direct `WorkspacePublicationScript` import and `workspaces` binding, removal of the old package-body workspace/FEM declarations, exact named-input arrays, all three package/project/seed/derived routes, and retained historical source inputs.

The owned preview fixture now has exactly 18 IDs. It includes `playground-session` and `report-actor-network`, while `ticket-important-fem-handoff` is absent. This closes the prior 19-versus-17 inventory finding at the static-source level.

The source test contains suitable private-tree controls for fast-glob parity, order, unrelated root-document preservation, duplicate names, cancellation, linked directories, unreadable/nonregular admission, and exact command arguments. It also treats the five historical files as byte-preserved inputs and asserts both malformed paths absent.

This update is not acceptance. I have not run the direct or registered route. Acceptance remains contingent on Sol's current direct and isolated Nx evidence, then a final check that the live taxonomy, project, seed, derived launch, registry launch assertion, and inventory all agree.


## Executor Evidence Update — Registered Route Pending

Sol reports the current direct source gate is green: 8 cases / 120 assertions. It covers the five owners, five internal edges, five full context chains, real exports, exact edges, no package-barrel/backedge, mandatory `WorkspacePublicationScript` router binding, exact named/target inputs, final 18-entry generator inventory, five historical hashes, two malformed-path absences, and all historical authority retirements. The existing owned-preview inventory control is also green at 1 case / 55 assertions, and production launch rendering completed 61 retained playground entries without FEM.

This remains an interim result. The initial isolated Nx attempt failed before the target started because an `@nx/js` worker did not start; Sol is rerunning once with fresh private roots. It is not a product test failure. Final acceptance requires that current isolated registered evidence.

An ordinary read-only `workspaces --check` correctly found a separate repository data condition: duplicate `@semio-tech/framework-surface-rs` paths. It made no write. This is a live workspace-data limit and does not invalidate the bounded private publication control, but it prevents any claim that the live root membership is currently fresh.


## Final Independent Acceptance

**Accepted for the bounded workspace-publication and historical-FEM-retirement extraction.** I independently re-read the final source topology before accepting it. The mandatory package router directly imports `WorkspacePublicationScript` from `🗂️workspaces/🏃️execution/🟦️.ts` and registers `workspaces` without retaining a workspace or FEM-generator body. The extracted graph preserves the existing lower discovery/diff owner and has the expected one-way roles: execution → publication → projection/comparison → lower `🗂️workspaces/🟦️.ts`; it contains no owner-to-package or root-barrel facade.

The current source fixture/test proves five owners, five internal edges, five taxonomy context chains, direct router binding, exact named/target input arrays, the 18-entry preview inventory, retained historical hashes, two malformed-path absences, and retirement of the command/project/catalog/inventory/seed/derived/registry FEM authorities. Static inspection corroborates `playground-session` and `report-actor-network` in the current 18-entry inventory and the retained `workspaces-check`/`workspaces-write` launch routes.

Executor evidence is current and isolated: the direct source route passed **8 cases / 123 assertions in 1.51 s**; `@semio-tech/repo-lib:test-workspace-publication-source --skip-nx-cache` passed **8 / 123**, Bun body **1.82 s**, Nx **2.3 s**, with private workspace/cache roots. The existing installed owned-preview inventory control passed **1 / 55 in 1.90 s**. The earlier `@nx/js` worker-startup failure occurred before target execution and is retained as infrastructure history, not a product failure.

The historical five inputs remain source-preserved evidence under their recorded original hashes and no historical script was executed. The ordinary read-only live-root `workspaces --check` found an existing duplicate `@semio-tech/framework-surface-rs` at the surface Rust root/bindings and stopped without a write. That repository-data condition is outside the bounded private publication proof: this acceptance does **not** claim current live-root membership freshness or authorize a live `--write`.
