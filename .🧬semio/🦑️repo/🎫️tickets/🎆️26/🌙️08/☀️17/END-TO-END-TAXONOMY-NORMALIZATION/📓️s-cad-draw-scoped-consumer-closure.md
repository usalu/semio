# CAD/Draw Scoped Reference Closure

## Outcome

The exact source-subtree plans now capture the complete declared consumer graph without widening their source or inventory digests. No production move or apply was performed.

| Exact scope | Inventory entries | Moves | Structured edits | Unresolved | Owned regeneration |
| --- | ---: | ---: | ---: | ---: | --- |
| CAD model catalog source | 312 | 209 | 75 | 0 | plugin-registry |
| Draw command source | 29 | 11 | 27 | 0 | plugin-registry |

The complete Draw count is **27**, not the old narrow 25: the two newly captured external package consumers add one Cargo path edit and one Rust module path edit. The 27 consists of 10 moving Nx edits, two configurable Cargo entry edits, two moving Rust glue rebases, 11 root Cargo/dependency/script edits, and two package consumer edits.

CAD edits are exactly interaction Rust 49, runtime TypeScript 11, interaction-spec Rust 14, and spatial-kernel TypeScript 1. Runtime’s ten structural glob edits cover twelve selectors because two zero-source members are removed; the eleventh edit is its catalog grammar comment.

## Production changes

- Taxonomy registers nine exact CAD/Draw consumer identities (previously four). Three CAD artifact-owned consumers outside the catalog subtree and two Draw package consumers outside the command subtree are now explicit external consumers.
- Discovery validates the exact nine-identity contract set. Existing source and prospective canonical leaf paths are explicitly enumerated; regexes cannot authorize unlisted owners.
- External consumer reads validate all lexical ancestors without following symlinks.
- The declared-consumer stale graph remains active for a canonical destination-scoped empty plan, so reintroducing an old external token is rejected after convergence.
- Generator inputs are now admitted from tracked, untracked, and ignored Git candidates plus exact declarations before pattern matching and no-follow reads. Matching ignored wildcard inputs are retained. Unmatched ignored symlinks are not opened; admitted symlinks still fail closed. Candidate ancestor directories are retained where a declared `/**` pattern owns them. Duplicate candidate ancestors are evaluated once.
- No generator input grammar, source/canonical projection rule, profile vector, journal format, or transaction transition was weakened.

Changed production files:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts`

## Live plan observations

CAD:

- Historical source scope: [frozen CAD `/projections/0/sourceRoot` coordinate](../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🧪️cad-draw-path-projection/🔣️.json#/projections/0/sourceRoot).
- Source digest: `fa99d231c0190e627aff3d557c634ed3feb58391b090176673dbe272ca1dffe5`
- Inventory digest: `b4ad2c8d0dd78ae80a39c540fd146320dad7942c1b5521d7930bc38ba969ff5b`
- Plan digest: `16963c32e89ad543fa9c2240e2210eed298dacbbd37d80744935c02461c7e7c7`
- Registry regeneration: 107,695 declared input nodes, ten pre-output nodes, ten output nodes.

Draw:

- Historical source scope: [frozen Draw `/projections/1/sourceRoot` coordinate](../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🧪️cad-draw-path-projection/🔣️.json#/projections/1/sourceRoot).
- Source digest: `25defb08725cc7a5dc60e733444024078ed1b039c607c383c1d3aaa9d8b6a9c4`
- Inventory digest: `a66920d165007afb51bf6583b0fb113913ea34c560232a648734ba29de198930`
- Plan digest: `d9dfa85e04eab1de88fce7eb34d4280f5b3bf4cd7578805c04308bb14f0878d9`
- Registry regeneration: 107,715 declared input nodes, ten pre-output nodes, ten output nodes.

These are observations, not reusable apply artifacts: concurrent root/script/schema/cache work continued. A fresh exact plan is required for any apply. The registry input count changed by 20 between observations despite unchanged 220 projected source leaves.

All 220 sources remain regular and byte-identical to this lane’s frozen preimages. The original projection golden and its mapping digests remain unchanged. Source paths can be as long as 293 bytes; the authorized destinations retain their 237-byte CAD / 210-byte Draw maxima.

The root script changed concurrently after fixture capture and again after the live Draw plan. Its live-plan preimage was SHA-256 `1a4a3131bbfb5cb86a46b2b1196122c3bacb76dd7f8df2378978c082d70d7b0a`, size 2,670,274, mode 0644. A later read was `272d1f3d13dc3850e2be399ec611826303f88e8f9fded15229ae3e0be8a97be9`, size 2,672,610. This is explicit consumer preimage drift; the old observed plan must not be reused.

## Test-first evidence

- Initial language-neutral declaration test failed because `cad-editor-interaction` had no authority.
- Initial generator candidate test failed on the nonmatching ignored `python` symlink.
- Final ticket-local packet: **7 pass, 0 fail, 628 assertions, 101.77s**.
- Existing CAD/Draw structured reference and negative owner/selector packet: **2 pass, 0 fail, 92 assertions, 18.25s**.
- README generated-source rollback/commit/convergence and out-of-scope input drift regression: **3 pass, 0 fail, 88 assertions, 14.48s**.
- An earlier aggregate attempt had six passes and one default-five-second fixture setup timeout; the declaration test now has an explicit 120-second limit, and the final seven-case run passed.

The fixtures prove exact source-scope counts, unchanged inventory/source digests under external-only preimage changes, preimage drift rejection before apply, full external edit rollback, commit, empty canonical replan, external stale reintroduction rejection, unregistered-neighbor exclusion, no-follow ancestors, ignored input parity with fast-glob, and admitted symlink rejection.

Commands used explicit file paths:

```sh
bun test './.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/🧪️cad-draw-scoped-consumers.test.ts'
bun test './🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts' --test-name-pattern 'plans the exact CAD and Draw authority|rejects unowned artifact prose'
bun test './.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/🧪️readme-license-owner-integration.test.ts' --test-name-pattern 'retires the generated source|out-of-scope generated input drift'
```

Added ticket artifacts:

- `🧪️cad-draw-scoped-consumers.test.ts`
- `🧪️cad-draw-scoped-consumers/🔣️.json`
- `🧪️cad-draw-scoped-consumers/🧪️preimages/🔣️.json`
- `🧪️cad-draw-scoped-consumers/📇️registry-read-audit/🔣️.json`
- This report and `📓️h-plugin-registry-input-authority.md`.

The immutable fixture archive contains all 220 source leaves plus nine external consumers. The 2.6MB root script is represented by its exact supported consumer form and separately records the original full source hash; it is not executed. Archive SHA-256: `de893809a1754fc48555c8e2ace9898d377a382bd13b886627030559cd91a324`, 438,497 bytes.

## Boundary and follow-up

All actual Compose trees remained opaque. No shared Git mutation occurred; Git setup/commits were confined to newly created isolated test repositories. No production source move, generator write, or apply occurred. See the separate registry audit before treating the large registry ledger as minimal or complete semantic input authority.

All lane test, live-plan, and audit processes exited. Two isolated declaration-fixture directories left by interrupted fixture setup were removed; their inputs remain reproducible from the retained frozen archive. No production data was removed.
