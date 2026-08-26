# Semantic-Directory Residual Authority Census

## Decision

The current admitted non-Compose directory population is not closed, but most of it now has exact schema authority. Of 37,994 directory paths:

- 27,252 resolve to an existing global semantic-directory kind.
- 6,432 resolve through an exact owner-local registry or current manifest.
- 22 have one context-valid repository precedent but no owner manifest.
- 3,543 match one existing external/fixed directory contract.
- 745 remain blocking.

The manifest-backed population is substantially stronger than a stem heuristic: 145 current mutation catalog files contain 1,555 exact vector identities and 1,555 exact scenario identities. They yield 3,101 path-specific source/canonical authorities with no conflict. Those authorities, not a guessed emoji, close the repeated mutation hierarchy.

This packet is read-only. It did not modify production, tests, manifests, Git state, `compose`, or `temp/compose`.

## Scope and fingerprint

- Repository root: `/Users/ueli/Documents/semio`.
- Baseline: `9f449b10659b95148c8bcb3f91ce583bf7446973`.
- Population: Git-indexed plus current non-ignored untracked paths.
- Opaque rejection: both `compose` and `temp/compose` were excluded in Git pathspecs and rejected lexically before any path-derived read.
- Admitted path records: 64,816.
- NFC, byte-sorted, NUL-delimited admitted-path digest: `5f50ca143f35040d69824ebbb47fac76e1a0f446ae942feaab3d204d433984bb`.
- Lexically derived directory paths: 37,994.
- Canonical authority-row digest: `64cc6a3593de425a0f807c9867b655cccbf6e1dacca413d1408d9238d65c39ae`.
- Taxonomy: schema v7, SHA-256 `ee820f04c637370103a9764125383004be7162724b74d082f0fb278d11235537`, `validateTaxonomy()` problems: 0.
- Taxonomy authority inventory: 103 global kinds, 74 owner-local member kinds, two projected-member kinds, and 21 fixed-directory contracts.
- Observation: `2026-08-26T16:19:17.820Z`. Concurrent normalization work means a final gate must regenerate this census at a stable boundary.

The census never walked a directory. It derived ancestors from admitted path strings, normalized NFC/VS16, and resolved each segment using the already resolved parent/ancestor kind chain. This avoids entering either opaque root and makes the result byte reproducible.

## Exact partition

| Class | Directory paths | Distinct basenames within class | Authority |
| --- | ---: | ---: | --- |
| A — globally registered | 27,252 | 208 | One `semanticDirectoryKinds` match in structural context |
| B — owner-local/manifested | 6,432 | 4,567 | Exact `semanticDirectoryMemberKinds` membership or exact mutation catalog source/canonical tuple |
| C — unique repository precedent | 22 | 13 | One existing canonical basename with the exact NFC stem and one context-valid registry resolution |
| D — external/fixed | 3,543 | 3,509 | Exactly one `fixedDirectoryContracts` match |
| E — unresolved | 745 | 639 | No exact authority, ambiguous owner membership, or emoji-leading name absent from the registry |
| **Total** | **37,994** |  |  |

Distinct-basename counts are per class and are not additive because the same basename may be authoritative in one context and unresolved in another.

## Source-to-canonical decisions

There are 2,905 direct basename decisions. Ancestor propagation makes 17,608 directory paths change as full paths, but those descendants are not additional semantic decisions.

| Decision authority | Direct basename decisions | Rule |
| --- | ---: | --- |
| Mutation catalog source/canonical tuple | 2,603 | Use the exact catalog path, catalog ID, `sourceMutationDirectoryName`, `mutationDirectoryName`, scenario `id`, and `directoryName` |
| Exact owner member with missing canonical VS16 | 259 | Render the exact registered `memberNames` value |
| Unprefixed stem with one exact owner-local member in the resolved owner chain | 21 | Render that one member value; no global inference |
| Unique repository precedent | 22 | Conditional candidates listed below; require owner confirmation before transactional use |
| **Total** | **2,905** | |

The 145 current catalog files comprise 145 records: 31 empty registries and 114 non-empty registries. They contain exactly 1,555 vectors and 1,555 scenarios. Of 3,101 current directory paths covered by their exact source/canonical registry, 2,603 need a basename change and 498 are already canonical. No source path received conflicting catalog decisions.

The catalog decisions must be rendered through `artifact-mutation-tests-v1`, not applied as isolated in-place segment renames. The exact destination is:

```text
<artifact>/🧪️tests/🪆️<standard>-<subset>/<mutationDirectoryName>/<scenario.directoryName>
```

The tuple is the reversible identity. The profile string is forward-only and must never be reverse-parsed.

### Class C: all unique-precedent candidates

These are all 22 paths in class C. They use an existing emoji, exact stem equality, and a registry-valid destination; no emoji was invented. They are candidates rather than executable moves because lexical uniqueness does not prove owner semantics. The `cad`, `v1`, and `windows` examples demonstrate why the owner must ratify the meaning.

| Source basename | Candidate | Count | Exact current paths |
| --- | --- | ---: | --- |
| `before` | `⬅️before` | 1 | `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️18/DEGENERALIZE-HARDCODED-S-APP-IDENTITY-FROM-OS-SHELL/before` |
| `cad` | `✳️cad` | 1 | `.storybook/stories/cad` |
| `diff` | `🔺️diff` | 2 | interactive-runtime `🧪️p10r-coordinator-route-build/diff`; repo coordinator `app/api/v1/diff` |
| `fixtures` | `🧫️fixtures` | 3 | OS package fixtures; lossless-format ticket fixtures; VCS ticket `ws-b-iso/framework/sync/fixtures` |
| `framework` | `🧰️framework` | 3 | `.storybook/framework`; `.storybook/stories/framework`; VCS ticket `ws-b-iso/framework` |
| `hub` | `🌎️hub` | 1 | `🌎️hub/📦️packages/🦀️rust/.semio/hub` |
| `os` | `💻️os` | 2 | `.storybook/framework/os`; `.storybook/stories/framework/os` |
| `repo` | `🦑️repo` | 2 | interactive-runtime `🧪️p10r-coordinator-route-build/repo`; repo coordinator `app/api/v1/repo` |
| `snapshot` | `📸️snapshot` | 1 | writer-normalization ticket `snapshot` |
| `styling` | `🎨️styling` | 2 | `.storybook/styling`; `.storybook/stories/styling` |
| `tests` | `🧪️tests` | 2 | dispatch Rust package `tests`; UI-contract Rust package `tests` |
| `v1` | `🔖️v1` | 1 | repo coordinator `app/api/v1` |
| `windows` | `🪟️windows` | 1 | OS Semio associations `windows` |

Required closure for class C is an exact owner-local member or external directory contract at each listed path. Do not turn any of these 13 stem mappings into a global stem rule.

## All unresolved blockers

Every one of the 745 class-E paths appears exactly once in each of the two exhaustive partitions below. This lists the complete blocker population by cause and owner family without embedding 745 unstable concurrent paths in prose. The census digest binds every exact source path, candidate set, authority, and projected path.

### By failure reason

| Failure reason | Paths | Required decision |
| --- | ---: | --- |
| No authority | 683 | Add an exact owner-local member/manifest or exact external contract; otherwise retain as unresolved |
| Emoji-leading but unregistered | 60 | Register the exact owner-local member only from an existing manifest/owner decision; the emoji itself is not authority |
| Ambiguous owner-local member | 2 | The owner must choose between multiple registered emoji meanings; no first-match rule |
| **Total** | **745** | |

One observed ambiguity is repo CLI `internal/mcp`, for which both `🌉️mcp` and `🔌️mcp` are visible through the resolved owner chain. This is a real semantic choice. The resolver correctly refuses to select by ordering.

### By disjoint physical family

| Blocker family | Paths | Typical evidence and closure owner |
| --- | ---: | --- |
| Ticket evidence | 293 | Nested ticket roots, probes, scratch/build roots, retained snapshots. Govern by exact ticket retention manifests or remove; never globalize. |
| Tests and fixtures | 213 | Test case/fixture identity directories outside the mutation vector registry. Add exact test/fixture catalogs. |
| Inference descendants | 66 | Exact children beneath registered inference members. Extend the inference owner manifest/descendant contract. |
| Framework other | 54 | Renderer element identities, repo CLI internals, platform association values. Register at their true owner. |
| `✏️s` other | 49 | Plugin/CAD schema descendants and non-catalog test structures. Use artifact/plugin owner manifests. |
| Package internals | 23 | API route segments, Rust tests/benches, package-local metadata. Resolve while closing the package-boundary contract. |
| `♻️mit-bestand` | 19 | Presentation slide/topic and asset hierarchy. Use its existing owner-local member chain rather than English/German stem guessing. |
| Storybook | 12 | Generated story grouping. Add it to the Storybook generator authority and regenerate. |
| External dot-root metadata | 11 | `.storybook`, external `plans`/`agents` children, etc. Require exact tool-owned contracts. |
| Agent skills | 5 | Current skill directory IDs. Govern from the exact skill inventory rather than `**` wildcard acceptance. |
| **Total** | **745** | |

### By nearest resolved semantic owner

This second exhaustive view identifies the schema region that must own each decision.

| Nearest resolved owner kind | Blockers |
| --- | ---: |
| `ticket-day` | 251 |
| `tests` | 178 |
| `members-of-inferences` | 66 |
| `fixtures` | 38 |
| `typologies` | 38 |
| `elements` | 34 |
| `members-of-ticket-day` | 34 |
| No resolved semantic owner | 26 |
| `members-of-members-of-mit-bestand` | 19 |
| `members-of-members-of-modules` | 14 |
| `transformations` | 10 |
| `capabilities` | 8 |
| `standard` | 8 |
| `assets` | 5 |
| `members-of-members-of-fixtures` | 5 |
| `typescript-language` | 4 |
| `hub` | 2 |
| `configuration` | 1 |
| `members-of-ticket-year` | 1 |
| `modules` | 1 |
| `rust-language` | 1 |
| `ticket-year` | 1 |
| **Total** | **745** |

### Repeated exact blocker groups

Most blocker basenames occur once: 745 paths comprise 639 distinct basenames. The repeated groups worth schema-first treatment are:

| Basename | Paths | Closure |
| --- | ---: | --- |
| `src` | 46 | Ticket evidence/build-source owner contract; not a semantic global directory |
| `wasm32-unknown-unknown` | 10 | Exact Rust target-triple contract inside registered build-output evidence |
| `wasm32-wasip2` | 9 | Same target-triple authority |
| `interfaces` | 6 | Exact JCO output descendant contract |
| `PHASE-9-RUNTIME-DEPENDENCY-REMOVAL` | 4 | Nested ticket identity; current ticket-root contract does not authorize it |
| `rs` | 4 | Ticket/VCS evidence language grouping; exact evidence manifest required |
| `🔀️from_aec.building.structure` | 4 | CAD model transformation member manifest |
| `cargo-target` | 3 | Exact ticket build-output retention, never a global semantic kind |
| `generators` | 3 | Ticket-local generator evidence manifest |
| `plans` | 3 | Tool-specific external directory contracts, separately for each tool |
| `Benutzeroberfläche` | 2 | Mit-Bestand presentation member |
| `Recherche` | 2 | Mit-Bestand presentation member |
| `agents` | 2 | Exact GitHub/Kiro external contracts |
| `anta`, `noto-emoji`, `share-tech-mono` | 2 each | Font asset catalog members |
| `auth`, `event`, `ticket` | 2 each | Coordinator API route registry |
| `benches` | 2 | Exact Rust/Cargo package-layout contract or semantic relocation |
| `fixture-gen`, `playwright-output`, `probe`, `work` | 2 each | Ticket evidence retention manifests |
| `font` | 2 | Asset catalog owner |
| `hosts` | 2 | Storybook generator authority |
| `no-mutation`, `set-snapshot` | 2 each | Semio test-fixture catalog |
| `🏠️Roof` | 2 | CAD typology catalog member |
| `🧪️generator-preview` | 2 | Exact generator-preview fixture/evidence member |

All remaining groups are singletons but remain blocking. Representative production singletons include renderer elements such as `AgentApprovals` and `EngineCanvas`, Go CLI internals such as `humanize` and `ignore`, inference outputs such as `mean-curvature` and `minimum-thickness`, CAD transformation/typology members, Storybook `2d`/`3d`/`5d`, and Mit-Bestand presentation topics. Singleton status is not permission to invent a mapping.

## Collision forecast

Applying only the authority-backed and conditional segment renderings yields one collision under byte, NFC, case-fold, and VS16-fold comparison; there are no additional folded-only collision groups.

```text
.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/
SEMIO-ARTIFACT-LOSSLESS-WELL-KNOWN-FORMAT-ROUNDTRIPS/
  pdf-render
  📄️pdf-render
        ↓
  📄️pdf-render
```

The canonical sibling is already occupied. The planner must block until the ticket owner proves a content-preserving merge/removal or selects a different exact owner identity. Staging order cannot resolve two logical sources targeting one destination.

## Path-budget and platform forecast

For the current 64,816 admitted path records and `collisionPolicy.maxPathBytes = 240`:

| Measure | Current source paths | Segment-only canonical rendering |
| --- | ---: | ---: |
| Paths over 240 bytes | 8,212 | 9,031 |
| Maximum bytes | 313 | 321 |
| Newly over budget | 0 | 819 |
| Formerly over budget brought under | 0 | 0 |
| Maximum growth | 0 | 21 bytes |

This segment-only forecast is deliberately a rejection test, not the intended plan. In-place mutation/scenario prefixing duplicates the deep source hierarchy and makes path length worse. The already registered `artifact-mutation-tests-v1` projection must be applied as one structural move. The stable projection design proves 20,215 destinations for 1,555 scenarios, zero collisions/outside occupancy, and only five initially over-budget records; the three exact DIN 16798 scenario rewrites reduce all five to at most 238 bytes. The 240-byte limit must remain unchanged.

One current admitted ticket path has a trailing-space directory segment:

```text
.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️05/FEM-PLUGIN-MIGRATION-TO-CRATE-AND-TAXONOMY-CONSOLIDATION/ 
```

It remains invalid after every known directory decision. No current admitted path in this snapshot adds a Windows-reserved segment. The trailing-space path requires an exact ticket-evidence owner decision and must remain unresolved.

## Schema-first closure

1. Keep global kinds structural. Do not add a global kind merely because one of the 639 unresolved basenames exists.
2. Materialize exact owner registries for the ten blocker families. Prefer existing catalog/manifests; add strict catalogs only where a real owner can verify completeness.
3. Make generated Storybook/JCO paths generator-owned preview outputs, not hand-authored wildcard exceptions.
4. Extend test/fixture and inference descendant contracts with exact registered IDs and full coverage checks.
5. Give external tool directories exact contracts per tool and scope. Do not add `**/agents`, `**/plans`, `**/src`, `**/target-*`, or suffix-derived exceptions.
6. Resolve the two ambiguous members with owner decisions and register the chosen exact canonical value.
7. Resolve the `pdf-render` collision and trailing-space ticket path before plan emission.
8. Co-plan the 1,555 mutation identities through the existing projection contract; never apply their 2,603 direct basename changes in place.
9. Re-run the census at the stable writer boundary and require category E, collision groups, over-budget final destinations, reserved names, and trailing-dot/space paths all to be zero.

## Acceptance checks

- The five classes sum exactly to the admitted directory population.
- Every A/B/D path resolves to exactly one authority under its actual ancestor context.
- Every planned source/canonical pair is supplied by an exact registry/manifest tuple; class C is blocked until owner ratification.
- All 745 current E paths receive an exact owner decision or remain plan-blocking; none is hidden by a wildcard.
- Full mutation projection reports 1,555 identities, 20,215 distinct nodes, zero collisions, and no path over 240 bytes.
- The `pdf-render` collision and trailing-space path are absent from the plan's expected post-state.
- A second census at the same source/taxonomy digest is byte-identical; the post-apply second plan is empty.

## Reproducible read-only evidence

Representative commands used:

```text
git ls-files --cached --others --exclude-standard -z -- . ':(exclude)compose' ':(exclude)compose/**' ':(exclude)temp/compose' ':(exclude)temp/compose/**'
bun -e '<derive every ancestor lexically; resolve global/member/fixed kinds in parent context; reject opaque prefixes>'
bun -e '<read admitted 🧪️oracle/🔣️component.json paths; bind exact vector/scenario source and canonical names>'
bun -e '<project known directory decisions; compare byte/NFC/case/VS16 destinations and 240-byte budget>'
shasum -a 256 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json
```

No search traversed an opaque path, no production/test file was edited, and no Git-mutating command was run.
