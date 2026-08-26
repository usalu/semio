# Residual Package and Fixed-Contract Census

## Decision

Acceptance criteria 4, 5, 7, 8, 9, 10, and 11 are not yet closed. The taxonomy schema itself loads cleanly at v7, all 48 fixed-filename records have the required evidence fields, and all six governed areas are labelled `clean`; the current paths and the last canonical inventory do not verify those declarations.

This is a read-only baseline taken while the normalization writer was still active. Current-tree numbers below are therefore explicitly provisional until a stable-boundary inventory is regenerated. They are nevertheless fingerprinted and grouped so the next census can measure convergence rather than restart discovery.

## Scope and safety

- Repository root: `/Users/ueli/Documents/semio`.
- Normative baseline: `9f449b10659b95148c8bcb3f91ce583bf7446973`.
- Normative criteria: attached plan section 10, criteria 4, 5, 7, 8, 9, 10, and 11.
- Population for current acceptance counts: Git-indexed plus current non-ignored untracked paths, because production inventory is Git-index exact.
- Both `compose` and `temp/compose` were excluded lexically before filesystem access. Neither opaque tree was traversed or read.
- No Git-mutating command and no production edit was made.
- Observation: `2026-08-26T16:03:26.109Z`.
- Admitted path records: 64,806; SHA-256 of NFC, byte-sorted, NUL-delimited paths: `65bae2138fb880f5a68a0ddf097b4974f3686274fe3dc70efffc63d69b35a4b7`.
- Admitted regular files present on disk: 64,786; indexed-but-missing records: 14.
- Taxonomy observed during the census: schema v7, taxonomy digest `ee820f04c637370103a9764125383004be7162724b74d082f0fb278d11235537`, `validateTaxonomy()` problems: 0.

The existing canonical artifact at `📊️taxonomy-inventory/🔣️.json` is a separate, stable evidence layer. It records 103,892 entries, 41,539 violations, inventory digest `68166b9fdcf70c4ad85d3a521803c4f0e460c5a27a28c0c0cf24f73521878934`, and source-tree digest `e8504fdfe1cb218b37d6abafadde51469c0d128db427db4ac05e22453ac89bc8`. It predates current concurrent edits and must not be represented as the current tree.

## Acceptance status

| Criterion | Status | Exact residual evidence |
| --- | --- | --- |
| 4. Every renameable basename is one registered file-kind emoji | Fail | Of 60,561 renameable files, 8 are already canonical and 60,553 are not. Nineteen of the non-canonical set do not resolve to a file kind from their physical extension. |
| 5. No semantic stem remains in a renameable filename | Fail | 38,302 non-canonical files contain one of the explicitly forbidden stems; the remaining 22,251 carry other semantic stems. |
| 7. No implementation file beneath `📦️packages` | Fail | 256 confirmed implementations after the production fixed/config skip; 427 under a literal all-source-format analysis. The former is a hard lower bound. |
| 8. Every package source passes its language classifier | Fail | Contract-aware corpus: 119 pass, 256 implementation, 46 uncertain, 12 have no JavaScript package rule. Literal physical-source corpus: 120 pass, 427 implementation, 46 uncertain, 12 unsupported. |
| 9. Every fixed filename has an exact evidenced contract | Fail | 48 schema records are complete, but 32 current exact-name occurrences fall outside any contract's declared scope. |
| 10. No broad suffix/taxonomy blanket exception | Fail | Three overlap families cover 275 files; broad README/LICENSE/cache contracts cover 101 occurrences; one historical-ticket scoped kind blankets 37 suffixes with `^.+$`. |
| 11. Every formerly mixed/legacy non-Compose area is marked and verified clean | Fail | Six areas are marked clean, but all six plus undeclared paths retain criterion-4/5 residuals; the stable inventory still has 41,539 violations. |

## Criteria 4 and 5: physical file leaves

The current admitted regular-file partition is:

| Class | Files |
| --- | ---: |
| Exact fixed-contract match | 4,225 |
| Renameable and already canonical | 8 |
| Renameable and not canonical | 60,553 |
| Total | 64,786 |

The 60,553 non-canonical renameable files group by enforcement area as follows. `undeclared` is deliberately not treated as exempt because `areaEnforcement.undeclaredAreas` is `enforce`.

| Area | Non-canonical renameable files |
| --- | ---: |
| `✏️s/🔌️plugins` | 41,325 |
| Undeclared paths, principally governed ticket evidence | 15,936 |
| `🧰️framework` excluding the more-specific repo area | 2,903 |
| `♻️mit-bestand` | 209 |
| `🧰️framework/🛍️products/🦑️repo` | 128 |
| `🌎️hub` | 27 |
| `✏️s/🔨️modules` | 25 |
| **Total** | **60,553** |

Explicit forbidden-token hits are non-exclusive because a basename can contain more than one token:

| Token | Occurrences |
| --- | ---: |
| `component` | 36,177 |
| `test` | 1,639 |
| `index` | 127 |
| `spec` | 102 |
| `tests` | 89 |
| `impl` | 73 |
| `implementation` | 48 |
| `glue` | 220 |
| `backend` | 15 |
| `components` | 5 |

Closure is not another suffix exception. The schema-first sequence is:

1. Resolve the 19 unknown physical extensions to a real file-kind or a separately evidenced exact fixed contract.
2. Render every remaining physical leaf from its registered file kind.
3. Preserve semantic identity in a registered semantic owner directory; do not discard a stem and do not invent an emoji.
4. Re-run the byte-stable inventory and require exactly zero non-canonical renameable leaves and zero semantic stems.

## Criteria 7 and 8: package boundaries

### Current corpus

There are 203 lexical `📦️packages/<language>` boundaries containing 1,039 admitted files. Of these:

- 571 match an existing exact fixed/configurable contract.
- 433 are renameable source-kind leaves after fixed/config separation.
- 35 are other physical kinds.
- All 433 renameable source leaves are presently non-canonical.

The contract-aware source corpus is:

| Ecosystem | Declaration/pass | Implementation | Uncertain | Unsupported | Total |
| --- | ---: | ---: | ---: | ---: | ---: |
| Rust | 64 | 160 | 45 | 0 | 269 |
| TypeScript | 55 | 92 | 0 | 0 | 147 |
| Python | 0 | 2 | 1 | 0 | 3 |
| Go | 0 | 1 | 0 | 0 | 1 |
| .NET | 0 | 1 | 0 | 0 | 1 |
| JavaScript | 0 | 0 | 0 | 12 | 12 |
| **Total** | **119** | **256** | **46** | **12** | **433** |

The 12 unsupported JavaScript files occupy three concrete owners:

- Sequence WASM bridge: 4 files under `✏️s/🔌️plugins/🎬️sequence/.../🌉️wasm/📦️packages/🟨️javascript`.
- UI browser host and WebGPU surface: 4 files under `🧰️framework/🔨️modules/🖱️ui/.../📦️packages/🟨️javascript`.
- OS flow WASM bridge: 4 files under `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌉️wasm/📦️packages/🟨️javascript`.

The 46 uncertain files are 45 Rust and one Python. Most Rust uncertainty is the repeated `📦️glue.rs` form; other exact examples are UI `🦀️backend_alias.rs`, WGPU `🦀️lib.rs`, repo-test `📦️lib.rs`, and Python styling `🐍️__init__.py`. Uncertainty must fail closed.

### Fixed/config source loophole

The production package walker skips a file as soon as its basename matches an allowed fixed/configurable contract. That skips 172 source-format files before purity analysis. Running the same language classifiers over all 605 physical source-format leaves produces 427 implementation, 120 declaration, 46 uncertain, and 12 unsupported. The delta is 171 implementation-classified and one declaration-classified fixed/config source files.

This does not prove every `📜️script.ts` is forbidden: the plan explicitly permits tool configuration and build metadata that cannot live elsewhere. It does prove that basename alone is insufficient evidence. Until the 172 leaves have an explicit semantic disposition and validator, 256 is only the confirmed lower bound and criterion 7 cannot use it as a final total.

Required schema-first closure:

1. Give every permitted source-format fixed/config entry an explicit disposition such as `tool-metadata` or `adapter-source`, with authority and verifier. Do not infer the disposition from a fixed basename.
2. Analyze `adapter-source`; validate `tool-metadata` against its command-router/configuration contract. A broad skip is forbidden.
3. Add a first-class JavaScript package boundary rule and structural analyzer. Sharing the TypeScript parser implementation is appropriate, but JavaScript must have its own declared ecosystem rule rather than an implicit alias.
4. Add the plan-required C/C++ structural analyzer contract even though this snapshot contains no C/C++ package source leaf; fail closed when a future native leaf appears.
5. Move the 256 confirmed domain implementations beside their package boundaries under registered semantic owners. Keep only canonical physical adapter leaves beneath packages.
6. Resolve all 46 uncertain files structurally; do not turn uncertainty into an allowed role.
7. Re-run the same classifier after moves and require every source leaf to have one allowed, evidenced role.

The older canonical inventory reports 406 `implementation`, 341 `unresolved`, 16 `package-implementation-file`, and 390 `package-implementation-destination-unresolved`. These are valid evidence for the old source digest, not replacements for the current-tree counts above.

### Operational scanner risk outside the Git population

A direct read-only `discoverPackageProblems()` run on the whole non-opaque disk tree reported 13,930 problems: 13,849 `packaging-violation`, 73 `manifest-without-marker`, five `unknown-lang`, two `ambiguous-lang-shape`, and one residual implementation-directory problem. Most of the huge violation count comes from ignored isolated Cargo output such as `🧰️framework/📦️packages/🦀️rust/target-root-framework-schema`; `DISCOVERY_SKIP_DIRS` skips only the literal `target` name, not this registered isolated target-root form. Hub `.semio` database output is also entered.

These ignored files are not in the Git-index-exact acceptance population, so 13,930 must not be added to the 1,039 admitted-file census. The traversal is still an operational defect: package discovery should consume the same inventory population or an exact schema-owned ephemeral/generated-root exclusion registry. It must not add a generic `target-*` or suffix blanket.

## Criteria 9 and 10: fixed contracts

### What is already sound

- Taxonomy has 48 fixed-filename contracts.
- Every record has `authority`, `reason`, `configurability`, `scope`, `verification`, and `expires`.
- Taxonomy validation is green.
- 45 contracts currently match at least one admitted file.
- The three unused declarations are `python-init`, `caddyfile`, and `github-pages-cname`; absence is not itself a violation.

### Thirty-two scope misses

Exact scope-aware matching leaves 32 fixed-looking paths without a contract:

| Group | Count | Required decision |
| --- | ---: | --- |
| Nested `🎫️ticket.json` below another ticket slug | 19 | Relocate/remove invalid nested tickets, or define a real repo-MCP child-ticket contract. Never use `**/🎫️ticket.json`. |
| Go module files outside a declared Go package root | 6 | Five `go.mod` and one `go.sum`; move production modules to package roots or declare exact module-root owners. The ticket scratch pair should remain evidence only if governed explicitly. |
| `config.toml` | 2 | Add exact `.cargo/config.toml` and `.codex/config.toml` contracts with their distinct external authorities. |
| Ticket `progress.md` | 2 | Normalize/remove historical scratch; Ralph's exact `.ralph-tui/progress.md` contract is not authority for arbitrary ticket progress files. |
| `tsconfig.json` outside a TypeScript package root | 2 | Decide exact repository-root and window-kit workspace ownership; do not broaden the package-root contract. |
| Repository-root `pyproject.toml` | 1 | Add an exact repository-root Python-tooling contract or move it to a declared package owner. |
| **Total** | **32** | |

The four production Go module roots are repo CLI, repo MCP client, repo library, and repo coordinator. The fifth module pair is ticket scratch. The two TypeScript misses are repository-root `tsconfig.json` and OS window-kits `tsconfig.json`. The two progress files belong to `TOTAL-JSON-PURGE` and `HANDCRAFTED-GRAMMAR-FOR-EVERY-ARTIFACT`.

The 19 nested ticket manifests group exactly as: OS refactor foundations (1), generalize-apps child tickets (2), July 19 Storybook/photogrammetry/Network-X child tickets (6), July 29 app-move child tickets (2), and August 20 interactive-runtime child tickets (8).

### Broad and overlapping rules

Current exact match overlaps are:

| Overlap | Files |
| --- | ---: |
| `cargo-manifest` plus `root-cargo` | 194 |
| `node-package-manifest` plus `root-package` | 80 |
| `nx-project-manifest` plus `root-project` | 1 |
| **Total overlap events** | **275** |

The blanket candidates that require narrowing or removal are:

- `root-readme`: `**/README.md`, 33 occurrences. Repository-host rendering does not establish that every nested README is unconfigurable.
- `root-license`: `**/LICENSE.md`, 9 occurrences. Scanner evidence must be tied to exact repository/package ownership rather than every directory.
- `cargo-cache-tag`: `**/CACHEDIR.TAG`, 59 admitted occurrences, predominantly retained ticket build caches. Exact external naming is real, but retention/build-output ownership must be proved before these become taxonomy exceptions.
- `historical-ticket-evidence`: one ticket-scoped pseudo-kind owns 37 suffixes with `sourceFilenamePattern: "^.+$"`. It is narrower than a repository-wide suffix rule, but it still permits arbitrary semantic basenames anywhere below every historical ticket.
- `root-cargo` and `root-package` duplicate their package-root contracts with unrestricted `**` path-pattern scope. Replace them with explicit repository/workspace-root variants and package-root variants, then require one winning contract per file.

Schema-first closure should make fixed scope a strict tagged union (`exact-path`, `repository-root`, `package-root`, or registered directory-kind owner), reject equal-specificity overlap, and require one winning contract for every fixed file. Historical evidence should be normalized to canonical physical leaves under semantic evidence directories or enumerated by an exact retention manifest; the 37-suffix blanket should then be deleted.

## Criterion 11: clean-area proof

Taxonomy currently declares exactly these areas `clean`:

1. `✏️s/🔌️plugins`
2. `✏️s/🔨️modules`
3. `🧰️framework`
4. `🌎️hub`
5. `♻️mit-bestand`
6. `🧰️framework/🛍️products/🦑️repo`

It also declares `requiredState: clean` and `undeclaredAreas: enforce`. This closes the marking half of criterion 11, but not verification. Every area has current file-leaf residuals, four top-level governed areas have package implementation residuals, and undeclared paths alone have 15,936 non-canonical renameable leaves.

Clean state must therefore become verified output, not a hand-authored assertion. For each area, persist or emit a digest-bound verification record containing zero counts for non-canonical file leaves, semantic filename stems, package implementation, uncertain/unsupported package roles, unmatched fixed files, blanket exceptions, collisions, and unresolved references. `clean` must fail closed if its source-tree/taxonomy digest differs from the inventory being verified.

## Implementation-ready closure order

1. Freeze the normalization writer boundary and regenerate one canonical full inventory; record its source-tree, inventory, taxonomy, and opaque-exclusion digests.
2. Close the 32 fixed-scope decisions and delete/narrow the blanket and overlapping contracts before planning renames.
3. Close package-role coverage: JavaScript plus native grammar declarations, explicit fixed/config source dispositions, and identical inventory/discovery classifier inputs.
4. Plan semantic-owner moves for every confirmed implementation and canonical leaf moves for every package adapter.
5. Normalize the remaining 60,553 file leaves, preserving semantic stems in registered directories.
6. Apply through the transactional normalization engine; regenerate owner outputs rather than editing generated leaves.
7. Generate per-area zero-residual verification against the exact post-state digest.
8. Run a second plan and require byte-identical empty moves/edits/regenerations/unresolved arrays.

## Acceptance checks after the stable boundary

Run through the existing Bun/Nx targets:

```text
bun nx run workspace:clean-taxonomy-inventory -- --ticket 26/08/17/END-TO-END-TAXONOMY-NORMALIZATION
bun nx run workspace:verify-package-purity
bun nx run workspace:verify-taxonomy-enforce
```

Required results for this packet's criteria:

- Inventory has zero criterion-4/5 violations and exactly zero unresolved file kinds.
- No package source has `implementation`, `unresolved`, or missing-analyzer role; fixed/config source disposition is explicitly verified rather than skipped by basename.
- Every fixed file resolves to exactly one evidenced contract; zero overlaps and zero scope misses.
- `scopedFileKinds` no longer supplies a broad historical suffix escape.
- Every declared and undeclared enforced area has a digest-matched zero-residual record.
- A second normalization plan is empty and byte deterministic.

## Evidence commands used

All commands were read-only. Representative commands:

```text
bun -e 'import { loadTaxonomy, validateTaxonomy } from "./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts"; const t=loadTaxonomy(); console.log(validateTaxonomy(t))'
git ls-files --cached --others --exclude-standard -z -- . ':(exclude)compose' ':(exclude)compose/**' ':(exclude)temp/compose' ':(exclude)temp/compose/**'
bun -e '<scope-aware fixed-contract and package-role census over the admitted path list>'
bun -e 'import { discoverPackageProblems } from "./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts"; /* aggregate only */'
```

The inline census rejected the run if either opaque prefix appeared in the admitted list and performed `existsSync`, `lstatSync`, or content reads only after that lexical rejection.

## Risks and interpretation limits

- Concurrent writers can change counts after the observation digest. This report is a residual baseline, not final verification.
- The canonical artifact and current census use different source digests; their counts are intentionally not combined.
- The lexical purity analyzer is conservative and can over-classify tool routers as implementation. That is why fixed/config source disposition requires explicit schema authority and verification instead of either automatic rejection or automatic exemption.
- The direct discovery scan includes ignored disk output that production inventory excludes. Aligning scanner population with inventory is necessary before using discovery counts as acceptance evidence.
- Ticket evidence dominates the renameable-file total. Ticket scope does not waive criteria 4, 5, 9, or 10.
