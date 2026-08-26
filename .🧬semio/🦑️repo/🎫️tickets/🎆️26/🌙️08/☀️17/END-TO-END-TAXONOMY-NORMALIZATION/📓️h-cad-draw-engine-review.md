# CAD and Draw Normalization Engine Review

## Outcome

Independent read-only review found no remaining blocking CAD or Draw projection defect at the frozen boundary. Strict taxonomy loading is green, the engine plans exactly 220 authority-owned file moves, the complete frozen reference surface is represented, false authority is rejected, configurable Draw library entries remain distinct from examples, apply/rollback/cancel/resume and post-apply verification converge, and stale source tokens are rejected even after an empty second plan.

This review did not apply a plan to the live repository and did not move or rewrite any live CAD/Draw tree. The actual workspace `compose/` and `temp/compose` trees were neither traversed, read, nor modified. Git state was not modified.

## Frozen Review Boundary

| Artifact | SHA-256 |
| --- | --- |
| `🔣️taxonomy.json` | `f15c641671df9fbee92413323e24e83c0c15b0e37e2510ad9ad1b25c88809df8` |
| `🔍️discovery/🟦️component.ts` | `bd570df82983c07d182e597dd5600198ea13c9a2260026073b4dd344d31a9c45` |
| `🧹️normalization/🟦️.ts` | `b3bc75814884ae6d815955a4a8a2c6e8a4de650bc18483b62ef535628b97b8fa` |
| `📦️packages/🟦️typescript/🧪️index.test.ts` | `80999f3d6a61f11f788f8ead45b59b30642d3b289445c4aecbed9202d731dbf1` |
| CAD/Draw golden `🔣️.json` | `1410a74ccc87561fd4a4b91db7d503614fe21ddce8bc78dee923d8237820f3e0` |
| `📓️h-cad-draw-independent.md` | `f6fa20ce937ecb0b87c638d57c95feb5603caa90ccf0ce83ba47dd9077648314` |
| `📓️h-cad-draw-reference-surface.md` | `aea8b2f1f3152b9fe52eda0c3bc470e313ee34805f4eb93e1cfb835a47f5939a` |

Production paths are relative to `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/`.

## Authority Reconciliation

| Projection contract | Files | Destination directories | Destination nodes | Maximum UTF-8 path | Mapping digest |
| --- | ---: | ---: | ---: | ---: | --- |
| `artifact-example-model-catalog-v1` | 209 | 244 | 453 | 237 bytes | `a09f60c5de5718394ddb856052444b306de7443b2d4ecd546e1e911dc44d40a6` |
| `artifact-editor-command-bundle-v1` | 11 | 9 | 20 | 210 bytes | `1f28fcc6e28e54001a9df6ce98b1c30b565cd42b824ed2491bb9b5e407b7436b` |

The engine-side mapping list is compared byte-for-byte with `semanticPathProjectionAuthority(...).mappings` before reference planning (`🧹️normalization/🟦️.ts:2528`). The permanent plan test independently compares all ordered source/destination pairs against the language-neutral golden (`📦️packages/🟦️typescript/🧪️index.test.ts:2944`). The result is exactly 209 CAD plus 11 Draw moves, with no projection or collision violation.

Draw realizes the corrected 20-node descendant union. Both source `📦️glue.rs` leaves are configurable `rust-library-entry` nodes whose destinations are `📚️library/🦀️.rs`; their two Cargo `lib.path` edits are reconciled one-for-one, including adapter, semantic location, old/new values, and preimage hash (`🧹️normalization/🟦️.ts:2744`). The `examples` and `library` directory kinds share the physical `📚️` emoji but have disjoint exact slugs `^examples$` and `^library$`; only `library` is admitted beneath Rust/TypeScript language directories. No examples/library conflation remains.

## Frozen Reference Surface

All 99 audited path-bearing occurrences are represented, plus the separately identified adjacent Rust root join:

- CAD 76: 61 exact Rust path edits; 12 selectors represented by 10 complete `import.meta.glob` edit records; two catalog comments; one catalog root marker. The adjacent `Path::join` root is separately represented by `artifact-catalog-root-join`.
- Draw 23: 20 exact supported edits, two workspace-glob edits, and one structured path-collection edit.
- Draw configuration authority adds two Cargo `lib.path` edits from `📦️glue.rs` to `📚️library/🦀️.rs`; these are schema-declared reference edits beyond the original 23-occurrence ledger.
- The exact zero-source set is three selectors represented by two complete removable object-member edits. Any non-registered nonempty selector blocks instead of being removed.

The four external consumer identities are schema-owned in `semanticPathProjectionReferenceConsumerContracts`: `draw-workspace-cargo`, `draw-dependency-registry`, `draw-workspace-script`, and `cad-spatial-kernel-geometry`. Engine selection delegates to `semanticPathProjectionReferenceConsumers(...)` (`🧹️normalization/🟦️.ts:2566`). A targeted source scan found none of those consumer IDs or their spatial path identity hardcoded in normalization production code.

## Findings and Closure Evidence

1. **Package-role violations initially suppressed valid Draw projection.** Closed by the configurable-entry descendant contract, dedicated `library` directory kind, and exact Cargo reference authority. The corrected golden proves 11 files, 9 directories, 20 nodes, two Cargo edits, and 210-byte maximum.
2. **The first engine fixture omitted 49 CAD exact includes and Draw external consumers.** Closed by the permanent exact-plan assertions: 61 CAD exact edits, 20 Draw exact edits, all structural edit counts, and the two relative Draw package consumers.
3. **Selectorless prose could bind the sole CAD authority, and counterfeit selectors could bypass owner checks.** Closed in `catalogProjectionForToken` (`🧹️normalization/🟦️.ts:2569`): selection requires the artifact owner or one exact registered consumer and, when selectors exist, an exact source-owner tail. The negative test at `index.test.ts:2994` covers unowned prose, unmatched selectors, escaped placeholders, and a counterfeit source owner.
4. **Source authority previously ignored the artifact-member owner relationship.** Closed in `artifactProjectionSourceLocation` (`🧹️normalization/🟦️.ts:3221`), which requires both the registered semantic member and its immediate canonical `🗿️artifacts` parent before projection authority can be constructed.
5. **External consumer paths and forms were hardcoded.** Closed by the four strict schema consumer contracts and discovery helper. Normalization contains adapter grammars but no external consumer identity literal; consumer admission requires contract ID, path identity/pattern, adapter, and supported form.
6. **Authority-declared configuration edits were not reconciled with the planned edits.** Closed by exact cardinality and preimage/value reconciliation at `🟦️.ts:2744`. The corrected Draw golden contains exactly two `referenceEdits`, and the plan test matches both.
7. **Post-apply checking originally covered mutation bundles only.** Closed by `artifactProjectionPostApplyViolations` (`🟦️.ts:4214`), which checks stale source files, every expected destination file/directory, symlinks, missing nodes, and unexpected descendants before commit.
8. **Stale scanning initially missed external consumers and appeared vulnerable after convergence.** Closed by schema-derived stale groups and registered external consumers (`🟦️.ts:4082`, `🟦️.ts:4128`). The transactional test applies the plan, verifies an empty second plan, reintroduces one CAD marker and two Draw markers, and requires exactly three `projection-old-token-stale` errors while planning zero projection moves.
9. **Normalization could drift from the strict discovery schema boundary.** The loader runs `validateTaxonomy` before constructing its normalization view, and the permanent negative at `index.test.ts:3074` rejects a mismatched consumer identity and an undeclared configurable-node alias. Strict live load reports schema version 7 and zero validation problems.
10. **The concurrent fixed/package schema transition temporarily broke Cargo self-reference classification and the generator-preview fixture.** Both were repaired without weakening validation. The final CAD/Draw transaction test and Nx-owned preview regression are green.

## Transaction and Safety Review

- Plan authority: exact golden equality, deterministic ordered moves, exact configuration-reference reconciliation, and no projection/collision/reference errors.
- Apply safety: stale plan digest/preimage checks remain active; injected failure after edits rolls back to the byte-identical initial workspace.
- Cancellation: pre-apply cancellation changes no fixture bytes; the independent cancellation regression also rolls back and converges on retry.
- Resume: resuming the committed journal is idempotently committed.
- Post-apply: exact CAD/Draw descendants and stale sources are checked before the expected post-state digest and opaque-tree digests.
- Convergence: the immediate second plan has no CAD/Draw moves, no relevant reference edits, and no projection violations.
- Strict second-plan stale detection: reintroduced source markers block with three exact violations and no renewed projection moves.

No violation-suppression path was found that turns an unowned or uncertain projection into a valid move. Projection-specific cleanup removes only superseded filename/directory/path-policy diagnostics after schema authority has accepted the exact mapping (`🟦️.ts:3294`); authority, collision, reference, and package violations remain fail-closed.

## Commands and Results

Strict load:

```text
bun -e 'import { loadTaxonomy, validateTaxonomy } from "./🔍️discovery/🟦️component.ts"; const taxonomy=loadTaxonomy(); console.log(JSON.stringify({schemaVersion:taxonomy.schemaVersion,problems:validateTaxonomy(taxonomy).length}));'
{"schemaVersion":7,"problems":0}
```

Final focused matrix:

```text
bun test '📦️packages/🟦️typescript/🧪️index.test.ts' --test-name-pattern 'artifact-example-model-catalog-projection is schema-owned|artifact-example-model-catalog-projection fails closed|artifact-editor-command-projection|projects every registered golden bundle|plans the exact CAD and Draw authority mappings|rejects unowned artifact prose|rolls back and atomically applies CAD and Draw projections|normalization rejects malformed projection consumers|cancellation rolls back and a successful retry converges|plans, applies, verifies, and converges an exact Nx-owned preview'

10 pass
223 filtered out
0 fail
312 expect() calls
```

The matrix covers CAD fast-glob parity and negative authority, corrected Draw union/fast-glob parity, mutation projection regression, exact CAD/Draw plan and reference surface, negative counterfeit/selector-less cases, apply/rollback/cancel/resume/convergence/reintroduction, strict projection schema negatives, and generator preview apply/check convergence.

Snapshot and authority evidence were obtained with targeted `shasum -a 256`, `rg`, `sed`, and Bun JSON reads. No broad filesystem traversal was used.

## Residual Risk and Acceptance

No blocking finding remains for this packet. The focused tests exercise temporary fixture repositories rather than applying the 220 operations to the live tree; therefore the final live transaction must still be created from the frozen current inventory and rejected if its source-tree digest, plan digest, reference preimages, occupancy, opaque-tree digests, or post-state digest differ. That is an execution precondition, not an engine gap.

Acceptance checks satisfied:

- [x] Exactly 209 CAD and 11 Draw mappings; both authority digests match.
- [x] Corrected Draw 9-directory/20-node union, two Cargo edits, and 210-byte maximum.
- [x] No examples/library conflation.
- [x] All 76 CAD and 23 Draw frozen occurrences plus adjacent Rust join represented.
- [x] No hardcoded external consumer identities in normalization production code.
- [x] Counterfeit owner, unowned prose, selector mismatch, escaped selector, and zero-source behavior fail closed.
- [x] Rollback, cancellation, resume, post-apply verification, empty second plan, and stale reintroduction verified.
- [x] Mutation and Nx generator-preview regressions green.
- [x] Strict taxonomy load green with no compatibility/default parsing.
- [x] Actual Compose trees, live CAD/Draw trees, and Git state untouched.
