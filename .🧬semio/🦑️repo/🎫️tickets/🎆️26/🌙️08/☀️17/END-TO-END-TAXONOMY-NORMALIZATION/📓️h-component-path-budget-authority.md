# Component Path-Budget Authority

## Outcome

The `>240` blocker does not require abbreviating mutation or scenario identities. The existing schema-first `artifact-mutation-tests-v1` directory projection is the exact semantic compaction authority:

```text
<artifact>/🏅️standards/🔖️<standard>/🪆️subsets/✳️<subset>/🧬️schema/🧬️mutations/<mutation>/🧪️tests/<scenario>
→
<artifact>/🧪️tests/🪆️<standard>-<subset>/<canonical-mutation>/<canonical-scenario>
```

The profile is rendered forward-only from separately captured `standardVersion` and `subsetId`; the mutation and scenario identities come from exact owner-local catalog rows. No substring, truncation, hash, acronym, or reverse parser is involved. The catalog already covers 144 files, 144 catalogs, 1,555 vectors, 1,555 scenarios, and the exact 13-node scenario bundles. Six component leaves per bundle produce 9,330 component-file mappings.

At the first follow-up observation, the naive component-leaf projection produced 7,355 paths over 240 bytes. Exact composition with the registered mutation directory authority resolved 6,744 immediately. The apparent 611 residuals were all bare glTF mutation preimage paths that were concurrently being deleted while canonical emoji directories were being created. A later Git-state split observed 610 such deleted over-budget preimages and 6,745 physically present over-budget leaves; every physically present path was inside the existing catalog grammar.

Therefore the semantic answer is composition, not a second shortening vocabulary:

1. project exact catalog-governed mutation-test directories with `artifact-mutation-tests-v1`;
2. project `component` physical leaf stems to primary file-kind leaves inside the same atomic plan;
3. rewrite registered references from the same source/destination ledger;
4. validate the final composed destinations against the 240-byte, NFC, case-fold, and VS16-fold rules.

No production file, normalization/transaction code, Git state, or Compose path was changed. `compose/**` and `temp/compose/**` remained opaque.

## Observation boundaries

The parent authority froze 35,595 generic component leaves before this follow-up. Its mapping ledger was `bb024bcd396e4a627bd731f4d8a19cd007954f4f6260f7402725667603be7a91`, with zero collisions and a 304-byte maximum naive destination.

During this follow-up another lane began a physical mutation rename. A single `git ls-files --cached --others --exclude-standard` view was no longer a physical-tree view: it admitted tracked-deleted preimages and untracked destinations simultaneously. Two consecutive derivations changed from 37,160 to 37,168 component paths and from 611 to 610 uncovered preimages. That drift is why this packet does not add a golden or parity test: a golden over the mixed dirty boundary would invent authority over paths that no longer exist.

The final read-only state split used exact literal pathspecs:

```sh
git ls-files --cached -z -- ':(literal)✏️s'
git ls-files --others --exclude-standard -z -- ':(literal)✏️s'
git diff --name-only --diff-filter=D -z -- ':(literal)✏️s'
```

| View | All paths | Generic component leaves | Naive destinations `>240` | Maximum naive destination |
|---|---:|---:|---:|---:|
| Cached/index preimage | 41,747 | 35,584 | 6,655 | 304 bytes |
| Untracked destinations/additions | 1,745 | 1,604 | 700 | 312 bytes |
| Tracked deletions | 2,712 | 2,712 | 610 | 299 bytes |
| Physical working tree (`cached - deleted + untracked`) | 40,780 | 34,476 | 6,745 | 312 bytes |

The physical over-budget source-to-naive-leaf ledger SHA-256 is `bfde92c1b5b45f6bfdf04603366349c4f6d9f68595f8d434ad5cd14513ec3f18`. It contains 6,213 `🔣️component.json`, 479 `🦀️component.rs`, and 53 `🚫️component.absent` leaves.

The largest physical owner partitions are:

| Artifact owner | `>240` leaves |
|---|---:|
| `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program` | 1,302 |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf` | 700 |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio` | 619 |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📗️din16798` | 344 |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1998` | 195 |
| `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel` | 170 |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1991` | 137 |
| `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting` | 135 |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📓️iso16757` | 118 |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1996` | 118 |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📕️din4108` | 116 |
| `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d` | 108 |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1992` | 107 |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1995` | 101 |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1997` | 100 |

All 6,745 physical over-budget leaves have the exact source grammar registered by `artifact-mutation-tests-v1`. The 610 excluded paths are not a second semantic cohort: they are deleted bare glTF preimages. For example, `git status` reports the old `.../🧬️mutations/bind-default-scene/**` nodes as `D` and the catalog-owned `.../🧬️mutations/🔗️🎬️bind-default-scene/**` root as `??` during the in-flight rename.

## Exact semantic compaction authority

The existing taxonomy contracts provide every identity needed for deterministic composition:

| Contract | Authority |
|---|---|
| `artifact-mutation-tests-v1` | Exact source grammar and compact destination grammar. |
| `standard-subset-v1` | Forward-only profile renderer `🪆️{standardVersion}-{subsetId}`. |
| `mutation-catalog-vectors-v1` | Required physical source mutation name, canonical mutation name, and canonical scenario identity. |
| `mutation-scenario-bundle-v1` | Exact 13-node descendant shape and 42-byte longest canonical descendant reserve. |
| 144 `🧪️oracle/🔣️component.json` catalogs | Owner-local source/canonical identities for all 1,555 physical bundles. |

The existing strict catalog audit reports:

```json
{
  "vectors": 1555,
  "scenarios": 1555,
  "projectedDestinationCount": 1555,
  "projectedCollisions": 0,
  "exact13": 1555,
  "badBundles": 0,
  "missingRoots": 0,
  "maxProjectedBytesWithReserve": 240,
  "reserve": 42,
  "maxPathBytes": 240
}
```

The follow-up composition audit across the temporarily combined component population also returned zero exact, NFC, case-fold, and VS16-fold destination collisions; zero exact/NFC/case/VS16 occupied destinations; zero non-NFC paths; zero VS15 paths; zero Windows-reserved segments; and zero trailing-dot/space segments. Removing deleted preimages can only remove rows and cannot introduce a collision. The 611 composed destinations that remained over budget in the first combined scan were exactly the then-deleted, catalog-unmatched bare glTF preimages; every catalog-matched destination observes the schema-owned 240-byte bound.

## Stem-producing contracts that must change

The directory authority is already sufficient. The missing schema unit is a physical-leaf projection contract that can compose with it.

### Taxonomy

Current taxonomy still exposes `componentFileKinds` and `mutationComponentFileKindId` as stem-oriented concepts. Add one exact `semanticPhysicalLeafProjectionContracts` row, for example `component-stem-to-kind-only-v1`, with:

- literal admitted root `✏️s` and explicit opaque roots `compose`, `temp/compose`;
- source rendering `fileKind.emoji + "component" + longest registered extensionChain`, with the registered unprefixed Gherkin exception explicit rather than inferred;
- destination rendering `fileKind.emoji + extensionChain`;
- allowed structural owners `artifact`, `plugin`, and module;
- forward-only direction;
- required composition with any applicable semantic path-projection contract before path-budget validation;
- collision comparisons `exact`, `NFC`, locale-independent case-fold, and VS16-fold;
- a reference-consumer contract id and generator-consumer contract ids.

This makes the physical stem a source identity only. `componentFileKinds` may remain only as ecosystem-to-file-kind selection; it must no longer imply a filename stem. `mutationComponentFileKindId` should select `rust-source`, whose physical leaf is `🦀️.rs`.

### Discovery

`🔍️discovery/🟦️component.ts` still:

- builds component filenames with `canonicalStemmedFilenameForKind(kindId, "component", taxonomy)`;
- requires mutation leaves to render exactly `🦀️component.rs` and descriptors as `🔣️component.json`;
- detects language mirrors from stemmed basenames.

Replace those checks with the registered leaf projection resolver. Discovery must expose source and destination identities separately, compose directory and leaf mappings, and validate path budget only on the final destination. Language mirrors should resolve by destination file kind, never by the string `component`.

### Scaffolders and generators

Root `📜️script.ts` still calls `taxonomyMappedFilename(..., "component")` throughout artifact, standard, example, mutation, window, and package scaffolding. The plugin-registry `new surface` command still derives some component leaves with `canonicalStemmedFilenameForKind`. Both must render `canonicalFilenameForKind`/primary kind-only filenames and register their live `✏️s` outputs. Preview and check targets must compare the same composed path plan that normalization consumes.

The smallest generator change is not a separate renamer. It is one shared renderer used by root scaffolding, plugin-registry scaffolding, discovery, preview, freshness checks, and the transaction planner.

## Lexical reference owners

The prior stable packet observed 41,399 raw lexical component-token occurrences across `✏️s`, `🧰️framework`, `🌎️hub`, and root `📜️script.ts`. During the in-flight rename this fell to 40,923 occurrences in 7,588 files, with ledger SHA-256 `531099daf70a02a7d0ec1b8c4136dd1e54ae7bb3a32ca90c520c6004bb580870`. This drift is further evidence that no exact reference golden should be minted mid-transaction.

The moving observation partitions as follows:

| Root | Occurrences |
|---|---:|
| `✏️s` | 38,692 |
| `🧰️framework` | 1,860 |
| root `📜️script.ts` | 326 |
| `🌎️hub` | 45 |

Dominant consumer formats are Rust 32,414, TypeScript 4,319, JSON 1,027, Gherkin 680, TSX 660, grammar 387, protocol 310, protobuf 236, Python 177, Kaitai 144, GraphQL 139, ABNF 93, EBNF 83, G4 83, Spicy 81, text 47, TOML 18, JavaScript 15, Markdown 7, WIT 2, and HTML 1.

The largest exact consumer owners are:

| Consumer owner | Occurrences |
|---|---:|
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio` | 3,603 |
| `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program` | 3,047 |
| `✏️s/🔌️plugins/🗄️stdio` | 2,467 |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf` | 2,108 |
| `✏️s/🔌️plugins/📕️norm` | 1,858 |
| `✏️s/🔌️plugins/🏛️architect` | 862 |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📗️din16798` | 819 |
| `✏️s/🔌️plugins/🧩️puzzle` | 758 |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1998` | 663 |
| `✏️s/🔌️plugins/🧱️block` | 637 |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1992` | 496 |
| `🧰️framework/🔨️modules/🖱️ui` | 448 |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1991` | 430 |
| `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel` | 402 |
| `✏️s/🔌️plugins/🏗️fem` | 369 |
| root router | 326 |

These are lexical hits, not all structured path references. The smallest safe reference schema adds adapter-specific consumer forms under the physical-leaf projection contract:

1. Rust `#[path]`, `include_*`, module/path literals, and schema/doc attributes;
2. TypeScript/TSX static imports, dynamic imports, URLs, and literal path registries;
3. JSON/TOML structured path locations, including `$schema`, manifest, and package/config entries;
4. GraphQL/protobuf/grammar/protocol/Kaitai/ABNF/EBNF/G4/Spicy import or include forms;
5. textual stale-marker checks for Markdown, comments, fixtures, and prose that mention an old basename but are not executable references.

Every adapter must receive exact source path identities and a preimage hash. Basename-wide replacement is not admissible. The root router, discovery library, plugin registry, and all owner-local consumers must be regenerated or edited in the same atomic transaction; a stale-marker postcondition must reject every old source token outside explicitly fixed historical evidence.

## Acceptance sequence

1. Allow the active mutation rename to reach a stable physical boundary; enumerate `cached - deleted + untracked`, not cached-plus-untracked alone.
2. Register `component-stem-to-kind-only-v1` and its composition order with `artifact-mutation-tests-v1`.
3. Freeze a new physical source/destination/reference golden only after two consecutive NUL ledgers agree.
4. Require zero exact/NFC/case/VS16 collisions and occupancy, zero platform-name violations, and a maximum final destination of 240 bytes.
5. Change taxonomy, discovery, root scaffolder, plugin-registry scaffolder, generator preview/check targets, structured reference adapters, and normalization planner together.
6. Apply the directory projection, leaf projection, and all reference edits atomically or roll back all of them.

Until step 1 stabilizes, adding a golden or parity test would encode a transient mix of deleted preimages and untracked destinations. This packet therefore records routing evidence only, as requested when an exact bounded cohort cannot be made stable and reviewable.
