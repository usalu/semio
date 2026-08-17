# W0-I — Coordinator lane report

## Delivered

### 1. Ticket + contract
- Ticket `26/08/16/PLUGIN-DEPENDENCIES-ARTIFACT-CONTRIBUTIONS-AND-COMPOSITE-MUTATIONS` opened under goal `🎯aioptimizedrepo` (issue 2558), start commit `7ad8955884`.
- `📋️master-plan.md`, `📋️contract-freeze.md` (§0-§6, normative for every lane), `📋️ownership-and-handoffs.md` (leases, shared-tree rules, gate commands).

### 2. Taxonomy: the composite-mutation shape
A composite mutation folds its diff and inverse from its plan, so it must NOT own `🔺️diff`/`↩️inverse`. Added as first-class vocabulary rather than a convention:

- `🔣️taxonomy.json`: `🧩️plan` added to `taxonomyLeafParentDirs`; new `compositeMutationChildDirs: ["🦠️mutation", "🧩️plan"]` beside the unchanged leaf `mutationChildDirs`.
- `📚️library/🔍️discovery/🟦️component.ts`: `compositeMutationChildDirs` typed and validated (non-empty, every member in `taxonomyLeafParentDirs`, must include `🦠️mutation`, must NOT include `🔺️diff`/`↩️inverse`); the mutation-dir child level now allows the union of both sets. `mutationChildDirs` was deliberately NOT extended — the plugin registry script (`📇️registry/📜️script.ts:987`) uses it as a REQUIRED set, so adding `🧩️plan` there would demand a plan dir in every leaf mutation in the repo.

### 3. Policy gates (root `📜️script.ts`)
- `policyMutationTriadCompletenessBreaches` now accepts either the leaf triad or the composite pair, selected by the presence of `🧩️plan`, and reports a mixed shape (`🧩️plan` next to `🔺️diff`/`↩️inverse`) as a high breach — two competing sources of the same semantics.
- `policyMutationImplPresenceBreaches` expects `impl … CompositeMutationKind<…>` for plan-shaped dirs (the old regex `\bMutation\s*<` cannot match `CompositeMutationKind<`, so composites would have been false-flagged forever).
- New `policyPluginDependencyParityBreaches`: `.depends_on("x")` ⇔ Cargo `semio-s-plugin-x`, **both directions**. Missing Cargo dep = high; Cargo dep without runtime declaration = medium (that direction IS the migration this ticket performs, so it tracks rather than gates).
- New `policyContributionTargetBreaches`: every `ArtifactContribution::builder("s.<owner>.<artifact>")` must target a directly declared dependency, and the kind must be canonical.
- Both registered in the top-level policy runner.
- Checked against the pre-existing `taxonomy/plugin-dependency-allowlist` rule (`📜️script.ts:6351`): it only flags dependencies whose `path` points into `🧰️framework/`, so plugin→plugin `semio-s-plugin-*` deps do not collide with it.

### 4. Registry validator taught the same shape
`🔌️plugin/📦️packages/🟦️typescript/📇️registry/📜️script.ts` enforced the leaf triad unconditionally in two places (the nested facet walk and the mutation-triad walk), which would have reported two false "missing leaf" findings for the first composite pilot. Added `mutationDirChildDirs(dir)` selecting the required set by the presence of `🧩️plan`, used at both sites, and added the composite dirs to the `taxonomyLeafParents` set. Verified: `bun ./📜️script.ts check` → *"plugin registry catalog is fresh (59 plugin crates, 58 playgrounds, 23 framework packages); .vscode/launch.json is fresh"* (the accompanying `manifest-without-marker`/`ambiguous-lang-shape` discovery problems are the same pre-existing repo-state noise the library tests report).

### 5. Derived dependency inventory
`📓️dependency-inventory.md` — the reverse-direction gate produced the complete real dependency graph: **61 declarations across 40 owners** (every plugin → `stdio`; `procedural` → 7 flow extensions + stdio; `demonstrator` → 6 plugins; each extension → its host plugin). This is the W1/W3 work list, derived rather than hand-written.

## Evidence

`🧪️w0-i-policy-run.txt` — `bun ./📜️script.ts policy` after the changes:
- `plugin-dependency/contribution-target`: 0 (correct — no contributions exist yet)
- `plugin-dependency/parity`: 61, all the migration direction (no high-priority half)
- `mutation-migration/triad-completeness`: 85, `mutation-migration/impl-presence`: 1430 — both unchanged from baseline, as expected: with no `🧩️plan` dir anywhere on disk the composite branch is inert.

`bun nx run @semio-tech/repo-lib:test` — 163 pass / 18 fail. **All 18 failures are pre-existing and unrelated** (`snapshotChildDirs` is absent from `🔣️taxonomy.json` at the start commit `7ad8955884` too; the rest are ui-css tokens, playground ports, cargo package names, and discovery problems in `🖱️ui`/`🦑️repo` packages). Zero failures mention `compositeMutationChildDirs` or `🧩️plan`.

## Notes for later waves

- The composite shape is now enforced consistently in all three validators (taxonomy discovery, root policy, plugin registry). W3-A can create `🧬️mutations/<kind>/{🦠️mutation,🧩️plan}` without tripping any of them.
- The root `📜️script.ts` is under concurrent edit by another session (786 changed lines vs. the start commit, far more than this lane's ~180). All edits here were region-local; nothing foreign was reverted.
- Contract freeze gained §5.9/§5.10 (one pending transaction per instance; a pending transaction freezes the instance's mutating surface) and a frozen rejection-code taxonomy after this lane's design review — W1-B and W2-A/B implement against those.
