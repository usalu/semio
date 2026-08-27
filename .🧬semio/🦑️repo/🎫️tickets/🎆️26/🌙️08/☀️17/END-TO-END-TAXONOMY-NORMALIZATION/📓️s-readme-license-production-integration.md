# README and LICENSE Production Integration

## Release

Production owner-projection integration is released. No physical production source move or retirement was applied by this worker. The current exact-source inventory audit resolves all 40 frozen leaves to their registered destinations with zero leaf authority drift.

The frozen authority remains 32 README and eight LICENSE leaves: four fixed publisher leaves and 36 semantic projections. Execution now distinguishes 35 ordinary leaf moves from one assets-build generated-source retirement plus owned regeneration. The generated distinction does not alter the frozen 40-leaf catalog or its 62 reference-owner bindings.

Frozen production catalog SHA-256: `051394741822e92d51f3bda15ce64d84c236582c6927335c9c5e0ac3c18a1da4`.

## Production contracts

- `readme-license-owner-leaves-v1` is a distinct exact-owner-path catalog contract. Source basenames alone never authorize a projection. Four exact publisher paths remain fixed; all other accepted sources require the frozen path, owner classification, byte hash, size, and mode.
- README semantics reside at the registered owner-local `📃️readme/📝️.md`; LICENSE semantics at `⚖️license/📝️.md`. Both directory kinds are projection-only. Canonical directories require an exact registered owner and one regular Markdown leaf; counterfeit owner directories, source drift, occupied destinations, and NFC/case/VS16 collisions fail closed.
- Catalog, source, destination, owner-evidence, and concrete-consumer reads use no-follow ancestry. Collision traversal is limited to the two new owner-local segments; unrelated preexisting aliases higher in the tree do not widen a scoped leaf transaction.
- The complete 62 frozen bindings remain intact. Registered out-of-scope consumers are captured as structured edits without adding them to the source inventory or inventory digest. Their preimages and rendered results are included in affected-state digests.
- The concrete Go owner has nine exact structural rewrites: technology, bundle, policy, technology-discovery joins, and the folder README predicate. The CommonMark scratch reader has one exact Rust filesystem-call rewrite. Other equivalent-looking basenames are not blanket-rewritten.
- Moving Markdown documents rebases relative inline links and HTML references to unmoved targets lexically. The CommonMark scratch test proves 27 inline-link occurrences with MarkdownIt parity and 26 HTML references, including preservation of the `.cursor` leading-dot basename. No referenced Compose target is opened.
- The generated assets README is retired only through exact catalog-backed source-removal authority paired with an owned generator preview. Preview bytes must equal the frozen source; differing source or destination bytes fail closed. Historical source-path fields in the immutable catalog are recognized only when the complete catalog digest matches; arbitrary stale consumer references remain rejected.
- Generator input ledgers are read independently from their declared literal-owner patterns, including foreign generated inputs such as the external shortcode snapshot. Input capture does not widen the scoped inventory digest. Cancellation checks cover input traversal.

## Assets convergence evidence

The canonical generated README appeared while the raw source was still present. Both are regular mode `0644`, 4,010-byte files with SHA-256 `749b8edff24de4f5993224e6c1b92b8f1afe14e4c949a4823b84bfb9591cc06d`.

- Raw source: `🧰️framework/🔨️modules/🖼️assets/README.md`, observed mtime `2026-08-26T14:13:49.127Z`.
- Canonical output: `🧰️framework/🔨️modules/🖼️assets/📃️readme/📝️.md`, observed mtime `2026-08-27T02:40:20.386Z`.
- Exact owner: `🧰️framework/🔨️modules/🖼️assets/📦️packages/🟦️typescript`, target `@semio-tech/assets:build`.
- Read-only live scoped plan: zero moves, one exact generated-source retirement, one assets-build regeneration, zero unresolved findings.
- Frozen generator ledger: 281 inputs, 289 pre-output nodes, 289 preview output nodes.
- Preview manifest SHA-256: `67a1face4fa1f1f4a6c313261278194fc43e71ba45dd479f354204285d793549`.

Both physical files were preserved. The canonical producer output registration has no raw README output alias.

The assets script previously dispatched its default build unconditionally when imported. That is a demonstrated production-write risk, not proof of which invocation created the observed canonical file. This worker's earlier discovery/normalizer and authority tests did not directly import the assets script. The script now dispatches only under `import.meta.main`. The isolated full-script import regression proves zero command dispatches, zero animation dispatches, and zero generated files. Isolated generator fixtures hard-pin their own root/cwd and run fixture-local scripts through Nx; they do not import the production assets script.

## Verification

Final exact-path runs:

- README/LICENSE integration suite: **18 passed, zero failed, 1,317 assertions, 25.31 seconds**. This includes Ajv/fast-glob authority parity; all 40 fixed/project/regenerate/canonical fact cases; exact source, mode, size, catalog, and collision rejection; real Go/Rust external-edit rollback/commit/empty-replan; MarkdownIt parity; generated-absent and generated-identical retirement/regeneration with real fixture Nx build/check; scoped input-ledger parity; and import purity.
- Existing owner authority plus ticket-important authority: **eight passed, zero failed, 649 assertions, 10.78 seconds**. The owner subset separately passed four tests and 619 assertions. Existing ticket contract-set checks include the distinct owner-primary-file contract integrated by the root coordinator.
- Live read-only inventory over each of the 40 exact source scopes, without unrelated explicit ticket admission: **40 expected destinations, zero leaf errors, zero drift**.
- Static production taxonomy validation and normalizer imports were exercised successfully. No independent full TypeScript typecheck or whole-monorepo acceptance claim is made here.

The ticket suite stores eight frozen source/consumer/evidence preimages so it remains runnable after physical projections. Its language-neutral gzip/base64 archive SHA-256 is `72505dc174f192470a2e67a57962c94f791f346018ab099221abbe5d47268cee`. Live corpus checks are phase-aware: frozen preimages constrain raw/fixed sources, while canonical non-generated Markdown may contain the checked relative-link edits.

## Safety and audit limitations

No shared-repository Git mutation, physical production move, handwritten source deletion, or production build/apply was performed. Isolated tests initialized and mutated only their own fixture repositories. The only production generator execution by this worker was the inspected read-only preview.

Two initial Bun test launcher attempts, one by this worker and one by its test subagent, omitted the leading `./` on a relative ticket test path. Bun treated them as filters, reported 165,039 and 166,129 files searched respectively, and ran no tests. Their opaque-tree enumeration boundary cannot be certified. No actual Compose path was intentionally opened, restored, edited, or targeted; nevertheless these discovery attempts prevent an absolute claim that no opaque enumeration occurred. Every subsequent test command used an explicit `./` or absolute file path.

One live inventory audit with the active ticket directory raced a fixture's disposal and failed with ENOENT before completion. The final complete 40-source audit omitted unrelated explicit ticket admission and succeeded. No authority relaxation was introduced for that race.

All worker-owned test processes have exited and all disposable fixture roots have been removed. Frozen test data and reports remain in the ticket.

## Released files

Production:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts`
- `🧰️framework/🔨️modules/🖼️assets/📦️packages/🟦️typescript/📜️script.ts`

Ticket:

- `🧪️readme-license-owner-authority.test.ts`
- `🧪️ticket-important-owner-authority.test.ts` (shared contract-set assertion, subsequently extended by the root coordinator)
- `🧪️readme-license-owner-integration.test.ts`
- `🧪️readme-license-owner-integration/🔣️.json`
- `🧪️readme-license-owner-integration/🧪️preimages/🔣️.json`
- This integration report; the earlier preapply report remains as its original observation record.

No new permanent script filename, runtime dependency, Git compatibility layer, migration script, or actual Compose implementation was added.

