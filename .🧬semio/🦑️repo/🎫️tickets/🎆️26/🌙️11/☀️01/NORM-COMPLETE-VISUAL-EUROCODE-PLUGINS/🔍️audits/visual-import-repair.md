# Visual Wire Twin Import Repair

EN1990 and EN1993 TypeScript editor/viewer twins retained exports for deleted inputs/results/report windows. Removed these obsolete exports, matching the existing thirteen other standards and the actual native window taxonomy. No window implementations or legacy adapters were added.

The marker grip and diagram table twin imports now reach the shared structural scene schema from their real nesting depth. The live-lanes twin reaches that same schema and explicitly imports EntityKind, which GhostSource needs.

## Verification

A committed language-neutral fixture covers four artifact-role modules and three shared modules. A Bun suite compares import extraction with the independent TypeScript compiler AST, resolves every referenced module, verifies the shared scene owner, and dynamically loads both artifact-role twins to check dialect and mode exports. Registered in the main TypeScript package suite plan.

Before repair: 0 passed, 2 failed; missing obsolete window module reproduced. After repair including semantic compilation of the shared visual modules: 3 passed, 0 failed, 26 assertions. Command: repo --service=off tools exec --light bun -- test ./✏️s/🔌️plugins/📕️norm/🧪️tests/🪟️visual-imports/🟦️.ts (SEMIO_AGENT_SESSION and SEMIO_LANGUAGE=en supplied).

This verifies import/module coherence and semantic compilation of the three shared visual modules and their TypeScript dependency closure. It does not verify browser rendering or whole-program type checking.

## Changed Files

- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🟦️.ts
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🟦️.ts
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🟦️.ts
- ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🟦️.ts
- ✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/🪟️visual-windows/📈️diagrams2d/🪧marker-grips/🟦️.ts
- ✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/🪟️visual-windows/📈️diagrams2d/🪟️window/🟦️.ts
- ✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/🫧️live-lanes/🧬️schema/🟦️.ts
- ✏️s/🔌️plugins/📕️norm/🧪️tests/🪟️visual-imports/🟦️.ts
- ✏️s/🔌️plugins/📕️norm/🧪️tests/🧫️fixtures/🪟️visual-imports/🔣️.json
- ✏️s/🔌️plugins/📕️norm/🧪️tests/🧫️fixtures/🏃️package-runner/🔣️.json
