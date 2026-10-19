# result-law-sweep (2026-10-19)

Executor `law-sweep`. Inputs: `brief-1019.md`, `result-owners-gate.md`, `result-zt-law.md`, `result-fixups2.md`, `result-cache-rest.md`.

## Method

- Law inventory: the 202 `script.ts verify ...` commands of the root `📋️project.json`, plus the route table in `L/🎛️commands/✅️verify/🟦️.ts` (L = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`). Classified with the real `ownedScriptRoutes` loader: 90 are owned routes (`dispatchOwnedScriptRoute` runs `bun repo run <project>:<target>`, i.e. nx), 111 are handled in the root verify file, 1 is the bare `verify`.
- Each runnable law ran alone: `perl -e 'alarm 300..590; exec @ARGV' -- bun ./📜️script.ts verify <law>` from the repo root, sequentially, logs under the ticket `🗑️generated/law-sweep` (deleted at the end). Machine load average was 33-45 on 10 cores, so one law takes 25-180 s.
- Read before run: each handler was read for cargo/nx/bazel/playwright/vitest spawns (`runCargo`, `runVitest`, `repo run`, `run-many`).

## Skipped laws (never run) and why

| Laws | Reason |
|---|---|
| 90 owned-route laws (`fem-*-owners*`, `*-document-contract*`, `*-window-ownership-{oracle,native}`, `home-host-panel-owner*`, `forms/dag/stdio/map/terrain/program/layout/cad/note/norm/presentation/procedure/curation/playbook/rewriting/wires/jack-*`, `generation2d/3d-*`, `gis-*`, `fem2d/3d-window-config-contract*`, `fem3d-numerical-child-*`, `framework-ui-protocol-ownership`, `framework-flow-physical-retirement*`, `writer-window-state*`, ...) | route resolves to `bun repo run <project>:<target>` (nx, cargo tests behind it) |
| every `... native` / `*-native` law (`window-config-retained-pack-load native`, `host-ownership native`, `*-viewport-* native`, `framework-retained-pack-ownership *-native`, `snapshot-read-retirement native`, `semio-envelope-identity native`, `owned-document-closure native`, `media-export-encoding native`, `file-open-import native`, `ui-dirty-scope native`, `spawned-job-drive native`, `store-reload-publication native`, `block3d-window-transient native`, `ephemeral-transfer-preparation native`, `view-context-integer-carriers [native]`, `window-view-context native`) | `runCargo` |
| bare cargo-only laws: `dag-demo-ownership`, `reset-document-ownership`, `wires-pointer-move`, `jack-results-window-transient`, `shared-value-canonical-json`, `shared-value-clone`, `framework-job-physical-close`, `framework-paged-list-ownership`, `framework-variant-field-casing`, `surface-view-context [menu]`, `artifact-contract-ownership [test]`, bare `window-action-context`, `retained-window-input`, `publication-retirement-authority`, `shared-artifact-addressing`, `shared-map-delta` (their `oracle` variants ran) | `runCargo` after the TS oracle |
| `rust-warnings`, `host-ownership` | cargo clippy / cargo test |
| `layering`, `canonical-architecture`, `gate`, bare `verify`, bare `interactivity` | `repo run-many` / `repo run` (nx); bare `verify` was never run |
| `node-graph-viewport-ownership*` | `declarations` script + vitest + cargo |
| `browser-actor-host-context worker`, `test-browser-actor-host-context-worker` | vitest |
| `typescript check` | whole-repo tsc over 16058 roots, exceeds the 300 s budget at this load (`typescript project` ran) |
| `build-owners sync`, `dependencies write-baseline` | mutate tracked files |
| `taxonomy enforce`, `taxonomy implementation enforce` | same scan as the `report` variants, identical result by construction |

## Results (law -> before -> after -> cause / fix)

Laws that pass (run, rc 0, unchanged): `zero-touch`, `generated-corruption` (rc 142 at 300 s, rc 0 at 539 s: passes, just slow), `build-owners` (first run failed on a peer's transient import break, then rc 0), `artifact-field-parity test|report`, `policy-breach artifact-builder|io-serializer-matrix|io-terminality|codec-fidelity|contribution-target`, `production-placeholders`, `interface-owners`, `mutation-caps`, `history-closure` (fixed, below), and the TypeScript oracle laws `resolved-host-context`, `window-view-context`, `browser-actor-host-context`, `window-action-context oracle`, `retained-window-input oracle`, `publication-retirement-authority oracle`, `shared-artifact-addressing oracle` (fixed, below), `shared-map-delta oracle`, `shared-dynamic-value`, `window-config-provisioning-lifecycle`, `window-config-pack-identity`, `semio-envelope-identity`, `owned-document-closure`, `snapshot-read-retirement`, `media-export-encoding`, `file-open-import`, `ui-dirty-scope`, `spawned-job-drive`, `framework-retained-pack-ownership [value|diagnostic|inflater]`, `framework-empty-state-contract` (fixed, below), `typescript project`.

### Fixed (verified by re-running the law)

| Law | Before | After | Cause | Fix |
|---|---|---|---|---|
| `history-closure` | rc 1, 3 findings | rc 0 | (a) stale allowlist: the puzzle3d editor unit test file was split into `.../🔬️unit/🎛️dispatch-and-operation-laws/🦀️.rs`, the `bracket-verb` allow entry still named the old path (2 findings, legitimate negative assertions); (b) real: window-config replace edit hand-built `ArtifactStoreOneItemFootprint { work_items: 2, .. }` | (a) allow path updated in `L/⚖️laws/🔬️interactivity-audit/🪣️tool-run-and-history/🟦️.ts`; (b) new `ArtifactStoreOneItemFootprint::for_point_invertible_item` (uses the existing `ARTIFACT_STORE_ONE_ITEM_INVERTIBLE_WORK_ITEMS`) in `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🎮️one-item/🦀️.rs`, call site in `.../🔌️plugin/🪟️window/🎚️config/🧬️preparation/🔁️replace/🦀️.rs`. Rust change not compiled (no cargo allowed) |
| `shared-artifact-addressing oracle` | rc 1 `blob reference requires hash, unsigned size and mediaType` | rc 0 | stale test: `parseBlobRef` now requires a typed `bigint` size (commit 8f91299), the JSON fixture carries numbers | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🪪️artifact-addressing/🟦️.ts`: new `exactBlobSizes` projection applied before `parseArtifactLink`/`parseArtifactChild` |
| `framework-empty-state-contract` | rc 1 `Vitest failed to find the current suite` | rc 0 | stale law input: the contract module registered a vitest `test(...)` at import, but the law imports it from bun | removed the vitest import and registration from `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🚫️empty-state/🟦️.ts` (no other consumer) |
| `dependencies` (import-boundary step) | rc 1 `must import its policy APIs from owned module .../📦️packages/🟦️typescript/🟦️.ts` | the import-boundary step passes; the law now stops at a later, different failure (below) | stale law: the expected module `library/📦️packages/🟦️typescript/🟦️.ts` was deleted (package split into per-domain wrappers), the three policy routers import the policy wrapper / the library owner | `L/🕸️dependencies/⚖️truth/🟦️.ts`: routers outside the library expect `📦️packages/🟦️typescript/🚨️policy/🟦️.ts`, the library's own router expects its owner module `🚨️policy/🧭️entrypoints/🟦️.ts` |
| `abstraction-ownership [test]` (partial) | rc 1 at `process3d` | `process3d` passes, next failure `Generation2dArtifact` | extractor bug: `policyExtractTypescriptSchemaFields` required whitespace after `import`/`export` (`import{type X}from"..."` in compact sources not followed) | `L/🧬️schema/🔍️field-discovery/🟦️typescript/🟦️.ts` regexes use `\s*`; language-agnostic fixture row "compact import and interface extension without whitespace" added to `L/📏️ownership/🧪️tests/🧫️fixtures/🪪️field-parity/🔣️.json`; `verify artifact-field-parity test` re-run rc 0 (its oracle compares with the TypeScript compiler) |
| `framework-viewport-projection` (source part) | rc 1 `Cannot find module .../📦️packages/🟦️typescript/🟦️.ts` from a ticket script | still rc 1, now only for the missing `prettier` (below) | the product law ran a temporary ticket script (`.../26/09/08/CORRECT-COMMAND-.../validation/📜️script.ts`) that imports the deleted barrel | law inlined in `L/🎛️commands/✅️verify/🟦️.ts` next to `framework-viewport-ownership`, ticket dependency removed |

### Failing, root cause known, not fixed (real defects or owner decisions)

| Law | State | Cause | Needed |
|---|---|---|---|
| `dependencies`, `dependencies literal-external` | rc 1 `Invalid Cargo workspace contribution` | real: `✏️s/🔌️plugins/🔱️trinity/Cargo.toml` has `members = [ ]` since the trinity LSP shim crate was deleted (rules-fix); the physical contract refuses an empty workspace | delete the empty container workspace and its entries (root `Cargo.toml` member list, `✏️s/Cargo.toml`, `schema-document-index.json` if keyed by workspace) or give it a member; cargo-window owner (lockref/deps4) |
| `package-purity` | rc 1, 4418 findings (3350 package-purity, 744 package-body-ownership, 324 package-body-unresolved incl. 283 `node_modules` symlink entries) | real backlog of the Shape V2 migration; the 283 `node_modules` entries look like a law defect (installed dirs are not authored package bodies) | owner ruling; exclude installed `node_modules` in the law |
| `semantic-vocabulary` | rc 1, 80 high | `NoMutation`/`SetSnapshot` substring in 80 plugin Rust files: 100 comment lines ("was dropped"), 5 code lines in 3 files (`🌿️vcs .../⏸️no-operation`, vcs unit tests, `🖍️draw .../📸️set-snapshot`) | reword the comments; remove the three commands (cargo needed) |
| `mutation-outcome-law` | rc 1, 1285 | real migration backlog: 1083 `inverse-sum-law-untested`, 78 `host-apply`, 48 `leaf-applies-diff`, 37 `leaf-mutable-base`, 21 `base-clone-diff`, 10 `message-code` | fan-out wave |
| `composed-child-refs` | rc 1, 728 breaches | all 728 rows are copies inside a ticket run folder (`.🧬semio/.../END-TO-END-TAXONOMY-NORMALIZATION/📓️energy-support-acceptance/🧾️runs/...`): the walk descends into tickets | QUEUED (library freeze): add `🎫️tickets` to `COMPOSED_CHILD_REF_SKIPPED_DIRECTORIES` in `L/🎛️commands/✅️verify/🧷️composed-child-refs/🟦️.ts` |
| `framework-viewport-ownership`, `framework-viewport-projection` | rc 1 `Module not found node_modules/prettier/bin/prettier.cjs` | stale law: `prettier` is declared in no `package.json` (and not installed); only these two lanes call it | QUEUED (library freeze): replace the `prettier --check` of the viewport `.graphql` files with a `graphql` (`^17`, a root dependency) `parse` in `L/🎛️commands/✅️verify/🟦️.ts` lines ~353 and ~367; deps owner may instead add prettier |
| `abstraction-ownership [test]` | rc 1, 17 artifact-representation gaps of 104 | missing declarations (probe over the fixture list): `Generation2dArtifact`/`Generation3dArtifact` (TS), `Puzzle3dArtifact` (Rust), `En1996Snapshot` (Rust/GraphQL/proto), `En1999Artifact`/`En1993Artifact` (GraphQL, proto), `ProgramDiff`/`WiresDiff` (TS/GraphQL diff), `din16798` (JSON schema has no `title`) | regenerate/author the representations (schema-first), large |
| `artifact-field-parity enforce` | rc 1 | `✒️writer` diff schema, `➗️equation` and subsets lack artifact fields (`missing=artifact`, `missing=equation`, `source:missing`) | schema owners |
| `policy-breach artifact-analyzer|artifact-composer|artifact-builder-migrated` | rc 1, 179 each | schema `🦀️.rs` of 179 subsets lacks `derive_artifact_facets!`; many subsets deliberately replaced it by the generic `SnapshotBuilder` (ticket CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM) | design ruling: the three laws are stale for migrated subsets or the subsets must derive |
| `policy-breach schema-representation` / `standards-coverage` | rc 1, 383 / 444 | missing spec leaves and required children in writer, equation, wfc, ... | schema/standards owners |
| `policy-breach artifact-decomposer` | rc 1, 1 | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📇️inventory` is a TypeScript contract helper (38 references) placed under `🗿️artifacts`, so it is scanned as an unmigrated artifact | move it out of `🗿️artifacts` |
| `policy-breach plugin-dependency-parity` | rc 1, 27 | `.depends_on("flow"|"imperative"|"process"|"cad"|"sourcing"...)` without `[package.metadata.semio].depends-on` | plugin owners |
| `interactivity commands` | rc 1, `non-literal=1` (0/1179 unreachable) | `🏙️bim .../✏️editor/📋️manifest/🦀️.rs:115` classifies every command in a loop (`action_interactive_job(&id, Migrated)`), the census only sees literals | declare literals or teach the census loops over the closed roster (library freeze) |
| `interactivity apps` | rc 1 | `descriptors=0`, `RENDERERS axis of the playground commands is not readable` in `🎛️dashboard/🎮️registry/🦀️.rs`: stale vs the dashboard control-plane rewrite | owner of the dashboard |
| `interactivity p1q-b1-b6`, `p1w`, `p1x`, `p1y`, `p1z`, `p5d`, `p5e`, `p3mn` | rc 1 | source-pinned text laws: pinned code shapes drifted (`unlosable hook request`, `DatabaseCatalogBootstrapKey`, `DB_IO_OPERATION_PAGES`, retained release witness, mutation no longer binds live source) | each pin owner decides stale vs real |
| `debug-tags` | rc 1 | 1146 lines in 629 sources still carry `[DEBUG]` (many are intentional test strings) | cleanup wave |
| `agnostic framework-names` / `utility-machines` | rc 1 | 91 naming sources (482 lines) / 5 offenders | backlog |
| `docstrings at-emoji|emoji-first|emoji-unique` | rc 1 | 489 / 5086 / 37707 docstring findings | backlog |
| `taxonomy implementation report` | rc 0 but `clean=false errors=66` (mit-bestand slide leaves and `repo` cli/server mains) | real | tax owner |
| `taxonomy report`, `interactivity tool-jobs` | not conclusive: timed out at 571 s / 570 s at load 45 | - | rerun on an idle machine |

## Transient failures seen (not law defects)

- `bun ./📜️script.ts` failed to load for ~1 h because a peer's storage refactor removed `native-build/💾️storage/🧱️fixed/🗂️system/🟦️.ts` while importers still named it; it resolved on its own (`bun build --target=bun 📜️script.ts`: 2495 modules).
- `loadCatalogTaxonomy` threw twice (`collisionPolicy`, `mutationCatalogProjection`) while a peer edited `🔣️taxonomy.json`; retried after it validated again.

## Verification

- Every row above marked "fixed" was re-run with `bun ./📜️script.ts verify <law>`: `history-closure` rc 0, `shared-artifact-addressing oracle` rc 0, `framework-empty-state-contract` rc 0, `artifact-field-parity test` rc 0 (new fixture row), `dependencies` passes the import-boundary step.
- Not run: `bun test` (no runner used), cargo (the Rust edit in the store and in the window-config replace edit is NOT compiled), nx, bazel.
- Coordinator hard freeze arrived mid-run; all edits under `L/` were made before it. Queued edits (apply after the lift): composed-child-refs ticket skip, graphql parse instead of prettier in the two viewport lanes.

## Files changed

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🔍️field-discovery/🟦️typescript/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📏️ownership/🧪️tests/🧫️fixtures/🪪️field-parity/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🔬️interactivity-audit/🪣️tool-run-and-history/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/⚖️truth/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/✅️verify/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🎮️one-item/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🧬️preparation/🔁️replace/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🪪️artifact-addressing/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🚫️empty-state/🟦️.ts`

## Open items

1. Apply the two queued edits after the freeze, rerun `composed-child-refs`, `framework-viewport-ownership`, `framework-viewport-projection`.
2. Trinity empty workspace (blocks both `dependencies` laws).
3. Compile-check the `for_point_invertible_item` Rust change in the cargo window.
4. One recursive `grep -rl` of a peer (pid 22520) was killed by me by mistake (it was not mine); read-only, no damage.
