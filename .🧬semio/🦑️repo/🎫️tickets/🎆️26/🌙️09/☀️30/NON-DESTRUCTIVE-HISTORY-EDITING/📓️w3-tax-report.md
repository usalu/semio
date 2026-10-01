# 🧾️ W3-TAX: Registering `🧾️wire-witness` and `🩹️patch-snapshot` in the Taxonomy

The scratch outputs and probes are in `🗑️generated/w3-tax/`.

## 1. Result

- **`🧾️wire-witness`** now has its own semantic directory kind: `mutation-wire-witness`.
  - **Before:** 331 directories were flagged.
  - **Now:** all 236 existing directories resolve. Peers turned about 95 witnesses into full cases during this work.
- **`🩹️patch-snapshot`** is a real mutation leaf, not junk. It is registered in two places:
  - as a leaf name in `members-of-schema`, which covers the 7 stdio leaves on both the schema side and the fixture side;
  - as an oracle-vector fixture name in `members-of-fixtures`, which covers the mp4 and wav `🧫️fixtures/🩹️patch-snapshot` directories a peer is adding.
- **Inventory-level diff over all 48 owner scopes:** 0 findings added, 710 removed. Every removed finding is `directory-kind-unresolved`.
- **The verify crash** was caused by tampered sealed evidence, not by a tool defect. I restored 4 sealed documents to their registered bytes, and 0 seal mismatches remain. A peer codemod corrupted one of them again during this work (see §4).
- **New test:** `test-mutation-wire-witness`, 13/13 passing (run directly and through nx).
- **Blocked now:** a peer's in-progress library refactor currently breaks the taxonomy engine for every scope (see §6.1). Because of that, I could not re-run the wav and json scoped verifies.

## 2. How the taxonomy resolves these directories

These rules are in `🧹️normalization/🟦️.ts`: `canonicalDirectory`, then `matchDirectoryKind`, then the passes after inventory.

- **`semanticDirectoryKinds`**
  - The leading emoji and the slug select candidate kinds.
  - `parentKindIds` limits a kind to certain **direct** parent kinds. Contextual kinds win over global ones.
- **`semanticDirectoryMemberKinds`**
  - These are registry lists of exact names.
  - They match by `ownerKindIds` against the nearest kind among the parent and ancestors.
  - Mutation leaves are resolved this way: `members-of-schema` holds 1,612 names and is the `sourceMemberKindId` of the projected `mutation-test-subject`.
- **Scenario directories** under `🧫️fixtures/🧬️mutations/<leaf>/<case>` (`🚫️rejects-ghosts`, `🧱️zero-scale`, …) are not resolved by kind.
  - They are paired by `normalizeMutationCasePairs`, using the catalog vectors and the `mutation-fixture-bundle-v1` descendant contract.
  - After pairing, the pass clears `directory-kind-unresolved` on the scenario, the fixture scenario and the schema-side leaf.
  - That pairing does not fit a wire witness. A witness is payload-only (just `🦠️mutation/🔣️.json`) and has no implementation case or vector.
- **Where `🧾️wire-witness` sits.** Kinds were probed with a copy of the engine.
  - **Leaf-local:** `<leaf>/🧫️fixtures/🧾️wire-witness`. The parent kind is always `fixtures`; the grandparent is `plugin-test-mutations`, `store-fixture-mutations`, `members-of-schema` or unregistered.
  - **Fixture mirror:** `<owner>/🧫️fixtures/🧬️mutations/<leaf>/🧾️wire-witness`. The parent is the mirrored leaf, which is `members-of-schema`, `store-fixture-mutations`, or **unresolved** (237 of 287).

## 3. What I registered

All edits are in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`. Each was one atomic Edit-tool change on a unique anchor.

1. **`semanticDirectoryKinds.mutation-wire-witness`** (placed after `mutation`):
   - `emoji` `🧾️`, `slugPattern` `^wire-witness$`, `allowEmojiOnly` false;
   - `parentKindIds`: `fixtures`, `members-of-schema`, `plugin-test-mutations`, `store-fixture-mutations`.

   This is one kind for one concept. Its parent is the mutation-leaf fixture scope in both placements. There is no list of individual paths.
2. **`members-of-schema` gained 230 leaf names.** These are the leaves that host a witness but had no registered identity.
   - 227 were found in the first scan. 3 came later from cad: `✋️drag-selection`, `🔄️rotate-selection` and `🔍️scale-selection`.
   - `🩹️patch-snapshot` is one of them.
   - **Why this was needed:** the witness created 237 new fixture-side leaf directories that could not resolve. A kind tied to a parent cannot resolve under a parent that is itself unresolved.
   - **Why it is safe:**
     - this is the existing leaf-identity registry;
     - the pairing logic does not depend on leaf kind (`canonicalProjectedMutationOwner`, `projectionSourceAt` and `validateMutationPayloadSchemas` were checked);
     - the canonical paths stay the same;
     - no new emoji-statute findings appear.
   - The schema-side twins of these leaves now resolve too.
3. **`members-of-fixtures` gained `🩹️patch-snapshot` and `🩹️patch-data`.** These are the handcrafted before/after vector directories under mp4 and wav `🧫️fixtures/`. They follow the existing sibling convention (`🎚️set-fmt`, `🔊️set-data` are already registered).

**Excluded: `🌦change-member-exposure` (norm en1992).**
- The `🌦` emoji has no VS16, so the taxonomy rejects the name as invalid.
- The leaf needs a rename to `🌦️change-member-exposure` by its owner. A peer was editing it at the time.
- Its witness has since been converted into a full case (`✏️to-xd1`).

`validateTaxonomy(loadTaxonomy())` returns `[]`.

## 4. The verify crash: a fixture defect, not a tool defect

**Symptom.** In the reference-authority step, `verify taxonomy report` threw:

```
frozen-coordinate-evidence-invalid: …/🧫️fixtures/📐️cad-draw-path-projection/🔣️.json: document digest does not match registered bytes
```

The throw came from `frozenCoordinateEvidenceCoordinates`, called from `incomingReferenceSnapshot`, which runs for every incoming candidate in any plan that has moves. This is why it showed up repo-wide.

**Root cause.**
- A codemod renamed `aec.building.structure.classic` to `aec.building` inside the **sealed** evidence. It landed in auto-commit 48d881aa7ab.
- The seal (`frozenCoordinateEvidenceContracts.cad-draw-projection-vectors-v1.sha256` = `9264c9de…`) still matches the bytes at 3eeee4f9119.
- Frozen evidence is immutable by design:
  - the planner never edits it (`frozen-coordinate-evidence-unowned`);
  - the seal test in `🕰️historical-json-source-encoding` forbids re-sealing.

**Fix.** I wrote the registered bytes back for every sealed document that had drifted:

| Document | Restored from | Cause |
|---|---|---|
| `📚️library/🧫️fixtures/📐️cad-draw-path-projection/🔣️.json` | 3eeee4f9119 | the codemod above |
| `.🧬semio/…/☀️09/APP-SCHEMA-FACETS/📜️normative-spec.md` | 101a6b4ea83 | 09-15 `$id`-host codemod |
| `.🧬semio/…/☀️08/ARTIFACT-SCHEMA-FACETS/📜️normative-spec.md` | 101a6b4ea83 | 09-15 `$id`-host codemod |
| `📚️library/🧫️fixtures/🧼️remaining-package-purity-authority/🔣️.json` | HEAD | a peer codemod rewrote `✏️s/🔌️plugins/📐️cad/📦️packages` → `✏️s/🧑‍💻dev/…` **during this session** (02:18) |

A re-check of every non-retired frozen contract (JSON and Markdown) finds 0 mismatches.

**Why I did not change the tool.**
- Throwing on tampered sealed evidence is the engine's deliberate fail-closed behaviour, and it is tested: several tests expect `toThrow(/frozen-coordinate-evidence-invalid/)`.
- Apply also refuses any plan that has unresolved errors.
- Turning the throw into a report would require the planner to treat a tampered document as fully sealed. That is possible as a follow-up (§6), but it touches the shared transaction engine and is not needed to fix this defect.

**Recommendation.** Path codemods must exclude every `frozen*CoordinateEvidenceContracts[*].path`. This happened three times. A cheap guard would be a library test asserting that every live sealed document still matches its digest; today only the retirement and existence checks exist.

## 5. Verification (all run by me)

- **Inventory diff, current taxonomy vs. candidate**, over all 48 witness owner scopes (`probe-diff.ts`, files `diff-1…5.txt`): **added 0; removed 710, all `directory-kind-unresolved`.**
- **Kind probe** over every `🧾️wire-witness` directory on disk (`find`, 236, `probe-context.ts`): all resolve to `mutation-wire-witness`.
  - Parent `fixtures`: 44.
  - Parent `members-of-schema`: 186.
  - Parent `store-fixture-mutations`: 6.
- **Leaf directories, both sides** (474 entries, `leaf-dirs-candidate.txt`): 472 resolve to `members-of-schema` with unchanged paths. The other 2 are the `🌦` leaf.
- **Scoped CLI `bun ./📜️script.ts verify taxonomy report --scope …`:**
  - interaction (leaf-local placement): 2 errors before, 1 after. The one left is a pre-existing `mutation-payload-schema-authority-invalid` on `🔁️set-state/🔣️.json`.
  - norm `🪟️results/🎚️config`: 5 before, 2 after. Both leaf directories and the witness now resolve; the 2 left are pre-existing `🔬️window-ownership` findings.
  - csv subset: 0 `wire-witness` or `patch-snapshot` findings. The 16 `directory-kind-unresolved` left are pre-existing and include unregistered leaves that have no witness.
  - mp4 subset: 32 errors, of which 0 are `wire-witness` or `patch-snapshot`.
  - cad subset (where the crash happened): after the restore it completes, about 20 min, with no crash.
  - **wav and json: blocked** by the peer engine break (§6.1).
- **New test `🧪️tests/🧪️mutation-wire-witness`.**
  - Fixture: `🧫️fixtures/🧫️mutation-wire-witness/🔣️.json`.
  - Schema: `🧬️schema/🔣️mutation-wire-witness/🔣️.json`.
  - It covers four things:
    - Ajv validates the vectors against the schema;
    - exactly one kind claims the name;
    - 9 resolution vectors from the discovery resolver are checked against an Ajv-compiled oracle;
    - 2 real placements get an engine inventory (3 nodes, exact leaf `🦠️mutation/🔣️.json`, no kind/stem findings, a parseable payload, an existing descriptor).
  - Results:
    - `bun test <abs path>`: **13 pass / 0 fail**;
    - `bun nx run @semio-tech/repo-lib:test-mutation-wire-witness --skip-nx-cache`: **13 pass / 0 fail**.
  - Both runs happened before the peer break. The placement tests now hit the same break.
- **Other library tests** (`bun test` on each):

| Test | Pass/fail | Failure cause |
|---|---|---|
| `🔤️taxonomy-leading-grapheme` | **9/0** | — (covers the 232 new member names) |
| `🏺️historical-package-owner-identity` | 25/1 | after my purity restore, only the peer loader break (§6.1) remains |
| `☂️frozen-coordinate-wildcard-coverage` | 4/1 | pre-existing: 232 vs 234 coordinates, and I changed neither the contract nor the bytes |
| `🕰️historical-json-source-encoding` | 21/1 | pre-existing: the seal law expects 38 original contracts, there are 40; `cad-draw-projection-vectors-v1` was re-sealed by peers on 09-25 and 09-30 |
| `❄️frozen-markdown-coordinates` | 34/2 | pre-existing: same JSON-seal drift, plus ENOENT on a closed ticket's `🧾️runs` |
| `🔏️path-emoji-statutes` | 33/4 | peer edits to `normalization` broke its extracted helpers (`existsSync` and `dirname` undefined), plus a TSV leaf schema missing `mutation` |
| `📍️draw-destination-observation` | 0/1 | pre-existing stale pin: `catalogSha256` `1410a74c…` is the 09-19 version of the cad-draw fixture |

## 6. Open items

1. **The taxonomy engine is currently broken by peers**, so every inventory, verify and test that uses it throws. There are two causes:
   - **Projection contract mismatch.** `semanticDescendantContracts.draw-editor-command-bundle-v1` was reduced to a 2-node bundle. Discovery was updated (`realizedNodeCount !== 2`), but the `🧹️normalization/🟦️.ts` loader (~line 1350) still requires 3 source-named / 3 `rust-source` nodes. It throws `semanticPathProjectionContracts.artifact-editor-command-bundle-v1 has invalid source-filename descendant authority`.
   - **Unfinished refactor.** A `📇️catalog/` refactor is in progress (untracked). For a while, `📇️catalog/📣️publication/🟦️.ts` imported a missing `../../../🧬️schema/✅️validation/🟦️.ts`, so `🧹️normalization` could not be imported. That import works again at my last check; the loader mismatch above was still there.

   Both belong to peers; I did not touch them. After they land, re-run the scoped verifies for wav and json (`--scope ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/{🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any,🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base}`) and `test-mutation-wire-witness`.
2. **Launch row.** `⚖️test-mutation-wire-witness📚️library🟦️` (4_gate, next to `test-mutation-case-pair`) is needed for the coordinator's central launch.json regeneration.
3. **`🌦change-member-exposure`** (en1992) needs a VS16 rename to `🌦️change-member-exposure`, done by the norm owner.
4. **Pre-existing, outside scope: unregistered leaves that have no witness.** For example, csv `✏️set-field`, `📤remove-record`, and mp4 `➕insert-track`, plus the mp4 oracle-vector siblings `🧫️fixtures/➕insert-track` and similar.
   - The name registry does not scale.
   - Long term, either use a structural mutation-leaf rule (a direct child of a `🧬️mutations` schema directory that owns a descriptor), or have a generator sync `members-of-schema` from the mutation catalogs.
5. **Test-contract breach, not taxonomy.** The csv and json `🔮️oracles/🔣️.json` list `patch-snapshot` in `kinds` but have no `mutationManifests` entry for it. The leaf itself is legitimate (descriptor, Rust, leaf schema, wire witness), so the gap is in the manifest.
6. **Optional tool hardening.** In planning and report mode, a sealed document whose bytes no longer match could become an unresolved `frozen-coordinate-evidence-invalid` finding, with the document treated as fully sealed (any rewrite gets `frozen-coordinate-evidence-unowned`). Apply is already blocked by unresolved errors. This would require coordinated work in the transaction engine.

## 7. Files

**Edited**
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`: kind, 230 leaf names, 2 fixture names.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts`: the `test mutation-wire-witness` branch.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📋️project.json`: the `test-mutation-wire-witness` target.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/package.json`: the script.

**Created**
- `📚️library/🧪️tests/🧪️mutation-wire-witness/🟦️.ts`
- `📚️library/🧫️fixtures/🧫️mutation-wire-witness/🔣️.json`
- `📚️library/🧬️schema/🔣️mutation-wire-witness/🔣️.json`

**Restored to sealed bytes**
- `📚️library/🧫️fixtures/📐️cad-draw-path-projection/🔣️.json`
- `📚️library/🧫️fixtures/🧼️remaining-package-purity-authority/🔣️.json`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️09/APP-SCHEMA-FACETS/📜️normative-spec.md`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️08/ARTIFACT-SCHEMA-FACETS/📜️normative-spec.md`

**Probes** (in `🗑️generated/w3-tax/`)
- These import a private copy of the engine, which I deleted after use. Regenerate it with:

  ```
  sed -e 's#from "\.\./#from "<lib>/#g' -e 's#from "\./#from "<lib>/🧹️normalization/#g' 🧹️normalization/🟦️.ts > probe-normalization.ts
  ```

  then append `export { canonicalDirectory as __canonicalDirectory, loadTaxonomy as __loadTaxonomy, ancestorDirectoryKindIds as __ancestorDirectoryKindIds };`.

## 8. Follow-up 1: structural mutation leaf identity (replaces the per-name leaf lists)

### 8.1 Design

The authority is the leaf's own canonical descriptor `<leaf>/🔣️.json` (`mutationPayloadSchemaAuthority`). A directory is
a mutation leaf when it lies under a `🧬️mutations` root and its descriptor has `schemaVersion: 1`, an `owner` naming
**exactly this directory**, and a non-empty `semanticKind`.

- Whether the identity agrees with the directory name stays with the existing payload-authority law. That law already
  reports `mutation-payload-schema-authority-invalid`; the kind rule does not report the same mismatch a second time.
- I chose an engine rule over a generator target. `🔣️taxonomy.json` is hand-authored and shared, so a generator
  rewriting it would clash with peers. The engine already resolves domain-owner leaves structurally to
  `members-of-schema`; this rule generalises that.

Engine changes, all in `📚️library/🧹️normalization/🟦️.ts`:

- **New `provenMutationLeafOwners`.** A pre-pass in `inventoryTaxonomyWithSourceParentPruning` before the directory
  loop. It reads every admitted descriptor. For mirrors and vector directories whose schema side lies outside the
  inventory scope, it also reads the schema-side descriptor from disk, following the catalog-reader precedent.
- **Change to `canonicalDirectory`.** New optional parameter `mutationLeaves: ReadonlySet<string> = new Set()`. Three
  shapes resolve before `matchDirectoryKind`:
  - a proven leaf, or its `🧫️fixtures/🧬️mutations/<leaf>` mirror, resolves to `members-of-schema`;
  - a `<owner>/🧫️fixtures/<leaf>` oracle vector directory whose `<owner>/🧬️schema/🧬️mutations/<leaf>` is proven
    resolves to `members-of-fixtures`.

  Canonical names are unchanged, and emoji statutes now apply to these directories as they do to any resolved one. The
  function gains no new free identifiers, because `🔬️workspace-contract` compiles it standalone.

### 8.2 Taxonomy (Edit tool, unique anchors)

- **`members-of-schema`:** 1,844 names → 44. The 1,800 removed names are mutation leaves, including my 230. The 44 left
  are not leaves, for example hub schema subjects, `*-internals` and `📸️set-snapshot`.
- **`members-of-fixtures`:** 292 → 240. I removed 52 leaf-named vector directories, including my `🩹️patch-snapshot` and
  `🩹️patch-data`.
- **Registries deleted:** `plugin-test-mutations` (16 names) and `store-fixture-mutations` (13 names). Their leaves now
  resolve structurally.
- **Dependents updated:**
  - `neutral-fixture.parentKindIds`: `plugin-test-mutations` → `members-of-schema`;
  - `mutation-wire-witness.parentKindIds` = `[fixtures, members-of-schema]`.
- **Real inconsistency fixed:** the `🔁️set-state` test-case directory was named like its leaf (`🧪️tests/🔁️set-state`)
  and only resolved through the name list. I renamed it to the open pattern `🧪️tests/🧪️set-state` and updated its
  `#[path]` in `🔌️plugin/🕹️interaction/🧬️mutations/🔁️set-state/🦀️.rs`.

### 8.3 Proof

The repo-wide directory-kind census ran the engine's `canonicalDirectory` over all 75,885 repository directories, with
the pre-change engine and names versus the new engine and new taxonomy:

| Measure | Result |
|---|---|
| `directory-kind-unresolved` added | **0** |
| `directory-kind-unresolved` removed | **2,433** |
| Canonical paths changed | 0 |
| Kind changes: `∅→members-of-schema` | 2,155 |
| Kind changes: `∅→members-of-fixtures` (vector dirs) | 276 |
| Kind changes: retired registries → `members-of-schema` | 40 |

- **No leaf directory remains unresolved.** That covers schema leaves, mirrors and vector directories, including csv
  `✏️set-field`, csv `🧫️fixtures/🧬️mutations/🩹️patch-snapshot`, mp4 `🧬️schema/🧬️mutations/➕insert-track` and mp4
  `🧫️fixtures/➕insert-track`.
- **Six direct children of `🧬️mutations` roots stay unresolved.** None is a leaf, because none has a self-owning
  descriptor; they are helper modules, owned by their peers:
  - os config `🎨️ui-preferences` (macro-generated leaf group);
  - workflow run `⚡️apply`;
  - xlsx `🧭️canonical-edit`, `🧭️cell-address`;
  - docx `🧭️xml-address`;
  - `🗺️map/🎚️config/🧫️fixtures/🧬️mutations/🪟️map-window-config-direct`.
- **Findings that were hidden before and now show.** 40 `path-emoji-presentation` findings in norm en1991. They cover 20
  leaves, schema side and mirror, whose emoji lack VS16. Before, `directory-kind-unresolved` masked them, because
  unresolved directories skip the emoji statutes. W2-W-norm-1 is restoring these emoji.
- **Identity mismatches the payload law owns** (not kind findings):
  - `🔁️set-state` declares `set-interaction-state`;
  - `📝️set-test-count` declares `set-count`;
  - en1992 `🧷change-anchor-a-s` declares `change-anchor-as`;
  - four energy leaves whose directories were shortened while the descriptors kept the long kinds.

### 8.4 Tests

- **New: `🧪️tests/🧪️mutation-leaf-identity`.** Fixture `🧫️fixtures/🧫️mutation-leaf-identity/🔣️.json`, schema
  `🧬️schema/🔣️mutation-leaf-identity/🔣️.json`.
  - Covers six real cases: schema leaf, fixture mirror, fixture vector, module leaf, nested store leaf, and one negative
    (a helper module without a descriptor).
  - Each case is checked by an Ajv descriptor-proof oracle against the engine inventory.
  - It also checks that no per-name leaf registry remains and that the descriptor authority equals the taxonomy's.
  - Result: `bun test`, **8 pass / 0 fail**.
  - Wired as `test-mutation-leaf-identity` in the library `📜️script.ts`, `📋️project.json` and `package.json`.
- **`🧪️mutation-wire-witness`:** vectors updated to the unified leaf kind; the discovery-only grandparent check is
  dropped. Result: **11 pass / 0 fail**.

### 8.5 More sealed-evidence restores (coordinator decision)

`📚️library/🖼️assets/📽️nested-cargo-package-projection/🔣️.json` was edited at 02:45, uncommitted and unclaimed:

- `wgpu-engine-relative` `sourceToken`/`destinationToken` lost their trailing `/`;
- the result hashed to `dd93cd84…` against the pinned `authorityCatalogSha256` `9445dea9…`;
- the edit made every inventory throw "Nested Cargo catalog digest drift".

I restored the HEAD bytes, and the sha now equals the pin. It matches the trailing-slash path-normalisation codemod
pattern; the same pattern also changed `"../../../../../../../"` to `"…/.."` in my new wire-witness test.

### 8.6 Open (follow-up 1)

- **The live taxonomy fails validation again because of a peer's in-progress edit.** It started 07:29 and was still
  present at 07:31:
  - new `🪪️identity/📁️installation` entries appended out of byte order in `generatorContracts["plugin-registry"]` and
    `["wgpu-frame-worker"].inputPatterns`;
  - the related `wgpu-frame-worker.packageGeneration` order.

  Until the peer fixes it, every engine run throws. I did not touch these entries.
- **Inventory-level before/after diff:** WRITTEN BUT NOT RUN, blocked by the bullet above. The directory-kind census
  covers the kind layer.

## Session 2 — 2026-10-01

Successor (S2-TAX), started 11:53. Scratch: `🗑️generated/s2-tax/`. Section in progress — the taxonomy report sweep (d)
is running; this section is rewritten in place at the end of the session.

### S2.1 Repair status (rule 21)

- Structural leaf identity (follow-up 1, §8) landed intact in auto-commit `4e36b2b5012`: `provenMutationLeafOwners` +
  `canonicalDirectory(…, mutationLeaves)` in `🧹️normalization/🟦️.ts`; `members-of-schema` 44 names, `members-of-fixtures`
  240, `plugin-test-mutations` / `store-fixture-mutations` gone, `mutation-wire-witness.parentKindIds = [fixtures,
  members-of-schema]`. No peer re-added leaf names. `validateTaxonomy(loadTaxonomy())` = `[]`.
- `test-mutation-leaf-identity` 8/0, `test-mutation-wire-witness` 11/0 (both run 12:00, `bun test`).
