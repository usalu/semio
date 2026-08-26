# S-CAD-DRAW-PROJECTION-SCHEMA

## Outcome

The taxonomy/discovery authority now defines exactly two additional schema-first, forward-only projection contracts:

- `artifact-example-model-catalog-v1` for the CAD distributed JSON model catalog.
- `artifact-editor-command-bundle-v1` for the Draw `🖱️canvas-pointer-down` command bundle.

Both use the shared `artifact-standard-subset-v1` renderer. The mutation-only `standard-subset-v1` contract and its source/canonical vector identities remain unchanged. No normalization plan/apply behavior was added, and no physical CAD, Draw, Compose, or temporary Compose path was changed.

## Production authority

Changed production authority:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`
  - registered `standard-subset-profile`, `models`, the eight exact CAD category kinds, and the Draw `fsm`/`macros` structural kinds;
  - added the two projection contracts and shared renderer;
  - added tagged `distributed-json-manifest-catalog` and `exact-owner-vectors` catalog contracts;
  - added the recursive CAD descendant contract and exact 18-node Draw descendant contract;
  - represented the three Draw Rust source leaves explicitly as `sourceFilename: "🦀️component.rs"` while their physical-format destination remains `🦀️.rs`;
  - retained fixed `Cargo.toml`, `📋️project.json`, `📜️script.ts`, and `📦️glue.rs` authority.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts`
  - added strict tagged schema validation for recursive catalog versus exact-bundle descendants;
  - added member-registry and command-directory capture semantics without weakening the mutation capture contract;
  - added `renderArtifactPathProjectionRoot`, the shared forward-only renderer;
  - added the pure, read-only `semanticPathProjectionAuthority` validator/renderer;
  - rejects missing/unowned model manifests, duplicate model/member authority, wrong category schemas or shapes, symlinks, non-NFC/VS15 paths, collisions, occupied destinations, and paths over 240 bytes;
  - emits mappings only when the complete authority has no problems.

The CAD category union is fail-closed and exact:

| Source category | Shape | Required schema | Destination member |
|---|---|---|---|
| `🎬️actions` | direct semantic JSON | `spatial.action` | `🎬️<stem>/🔣️.json` |
| `🎬️interactions` | direct semantic JSON | `spatial.interaction` | `🎬️<stem>/🔣️.json` |
| `🏷️attributeDefinitions` | direct semantic JSON | `spatial.attribute` | `🏷️<stem>/🔣️.json` |
| `📊️statDefinitions` | direct semantic JSON | `spatial.stat` | `📊️<stem>/🔣️.json` |
| `🏷️propertyKinds` | direct semantic JSON | `spatial.property` | `🏷️<stem>/🔣️.json` |
| `🔧️propertyDefinitions` | direct semantic JSON | `spatial.property` | `🔧️<stem>/🔣️.json` |
| `🗂️typologies` | nested fixed JSON | `spatial.typology` | existing member/`🔣️.json` |
| `🔀️transformations` | nested fixed JSON | `spatial.transformation` | existing member/`🔣️.json` |

Every model directory must contain exactly one `🔣️modelDefinition.json` declaring `schema: "spatial.modelDefinition"`, `version: "1.0.0"`, and an `id` equal to the directory semantic stem. The newly authored concrete manifest therefore participates as authority rather than an inferred path identity.

## Permanent language-neutral golden

Added:

`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🧪️cad-draw-path-projection/🔣️.json`

The JSON fixture owns all 220 current source/destination file pairs, profile identities, nine CAD model manifest identities, the eight category rules, the Draw fixed-file union, counts, maxima, and digests. Mapping digests use `sha256-source-nul-destination-lines-v1`: byte-sort by source path, encode each row as `source + NUL + destination`, join rows with LF, and do not append a final LF.

| Projection | Existing files | Destination directories | Destination nodes | Maximum bytes | Mapping SHA-256 |
|---|---:|---:|---:|---:|---|
| CAD model catalog | 209 | 244 | 453 | 237 | `a09f60c5de5718394ddb856052444b306de7443b2d4ecd546e1e911dc44d40a6` |
| Draw command bundle | 11 | 7 | 18 | 204 | `2341b92ad57c7e9103a7a4ee40e47d99fe561a21641c51a1a87fa2197fa76814` |
| Total | 220 | 251 | 471 | — | — |

The two tied 237-byte CAD destinations remain the reinforced-concrete external-wall and internal-wall `From2PointsAndHeight` action leaves. No destination exceeds `collisionPolicy.maxPathBytes = 240`.

No separate Rust taxonomy-golden consumer exists in this authority surface. The permanent JSON is language-neutral; the current production consumer and parity test are Bun/TypeScript, with `fast-glob` as the independent third-party census.

## Tests

Changed:

`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts`

Coverage includes live fast-glob parity, all 220 exact mappings, counts/digests/maxima, missing concrete manifest, duplicate model authority, wrong schema, unknown category, symlink, VS15, reverse-profile input, case-fold collision, over-budget path, occupied destination, partial/extra Draw bundle, and rejection of classifying the fixed Nx manifest as an ordinary JSON kind.

TDD red evidence:

```text
bun nx run @semio-tech/repo-lib:test -- --test-name-pattern=artifact-example-model-catalog-projection
0 pass, 1 fail, 1 module error
Export named 'semanticPathProjectionAuthority' not found
```

Final strict schema check:

```text
bun -e '...clearDiscoveryCache(); const taxonomy=loadTaxonomy(); ...validateTaxonomy(taxonomy)...'
schemaVersion=7
problems=0
projectionContracts=[artifact-mutation-tests-v1, artifact-example-model-catalog-v1, artifact-editor-command-bundle-v1]
```

Final focused authority suite:

```text
bun test '.../🧪️index.test.ts' --test-name-pattern='artifact-(?:example-model-catalog|editor-command)-projection'
3 pass, 223 filtered out, 0 fail, 30 assertions
```

Each focused Nx invocation ran its inner Bun selector successfully, including the final Draw result `1 pass, 225 filtered out, 0 fail, 10 assertions`, but the Nx parent process then exited 1 while writing its cache transcript:

```text
ENOENT: no such file or directory, open '.nx/cache/terminalOutputs/<hash>'
```

This is an external concurrent cache-directory failure after the test result, not an authority failure. No cache path was created or repaired in this lane.

The repo-lib lint target was also run. It reached TypeScript and reported only existing unrelated errors in UI styling `ImportMeta.env/glob` declarations and framework OS files outside repo-lib `rootDir`; it reported no CAD/Draw authority diagnostic.

## Acceptance boundary

- Exactly two new projection contracts: satisfied.
- Re-derived live population: CAD 209 files with nine model manifests; Draw 11 files; total 220 mappings.
- Re-derived destination population: CAD 453 nodes; Draw 18 nodes.
- Strict taxonomy load: green with zero validation problems.
- Language-neutral golden and third-party fast-glob parity: green.
- Engine plan/apply moves, reference edits, rollback, and physical tree mutation: intentionally not implemented in this schema lane.

## Addendum: canonical Draw package entries and external consumers

This addendum supersedes the earlier Draw statements that retained `📦️glue.rs`, an 18-node destination, a 72-byte descendant reserve, and digest `2341b92a…`. The two physical glue files are source evidence only. Each now projects to the configured Rust library entry at `📚️library/🦀️.rs`; `Cargo.toml`, `📋️project.json`, and `📜️script.ts` remain exact fixed filenames at the package root.

The schema adds the dedicated semantic directory kind `library` (`📚️`, `^library$`, Rust/TypeScript language parents), narrows `examples` to `^examples$`, and admits `library` explicitly in the Rust package boundary. It does not conflate package libraries with artifact examples.

The exact descendant node variant is:

```text
sourcePathSegments
destinationPathSegments
nodeType = file
configurableEntry.contractId = rust-library-entry
configurableEntry.sourceFilename = 📦️glue.rs
configurableEntry.configurationReferences[0] = cargo-manifest / toml / lib.path
```

The destination basename is derived only from `configurableEntryContracts.rust-library-entry.filename = 🦀️.rs`. Both source Cargo manifests contain `[lib] path = "📦️glue.rs"`; authority validates that structured value and emits exactly two edits at the mapped destination manifests:

| Source package | Preimage SHA-256 | New `lib.path` |
|---|---|---|
| `🔄️fsm/📦️packages/🦀️rust` | `35f3abecfcdfac2a01a433fdb61718ddc1802e5a5dcc05a413467d7afb18eaac` | `📚️library/🦀️.rs` |
| `🔄️fsm/✨️macros/📦️packages/🦀️rust` | `47213c84c9999d121abd74998de513b6d45c5838bcdcaf27e1dcf673acc01024` | `📚️library/🦀️.rs` |

Updated Draw authority:

| Existing files | Destination directories | Destination nodes | Descendant reserve | Maximum path bytes | Mapping SHA-256 | Structured edits |
|---:|---:|---:|---:|---:|---|---:|
| 11 | 9 | 20 | 78 | 210 | `1f28fcc6e28e54001a9df6ce98b1c30b565cd42b824ed2491bb9b5e407b7436b` | 2 |

CAD remains unchanged at 209 mappings, 244 destination directories, 453 destination nodes, 237 maximum bytes, and digest `a09f60c5de5718394ddb856052444b306de7443b2d4ecd546e1e911dc44d40a6`. The combined authority remains 220 existing-file mappings and is now 253 destination directories / 473 destination nodes.

### Schema-owned external reference consumers

`semanticPathProjectionReferenceConsumerContracts` replaces consumer inference with four exact external contracts. Every contract requires `projectionContractId`, a unique `consumerIdentity`, `ownership: external`, an anchored NFC `sourcePathPattern`, exact `sourcePathIdentities`, closed adapter/form arrays, and NFC stale markers. Patterns are pairwise non-overlapping for shared projection forms. Internal artifact-owned consumers remain owner-derived.

| Contract | Projection | Exact identities | Adapter/form | Stale marker |
|---|---|---|---|---|
| `cad-spatial-kernel-geometry` | CAD | current `…/📐️geometry/🟦️component.ts`; canonical `…/📐️geometry/🟦️.ts` | TypeScript / `artifact-catalog-prose:root-marker` | `🖼️assets/🏗️modelDefinitions` |
| `draw-workspace-cargo` | Draw | `Cargo.toml` | TOML / `path-reference` | exact legacy Draw command tail |
| `draw-dependency-registry` | Draw | `🔒️dependencies.json` | JSON / `path-reference` | exact legacy Draw command tail |
| `draw-workspace-script` | Draw | `📜️script.ts` | TypeScript / `path-reference` | exact legacy Draw command tail |

The exported `semanticPathProjectionReferenceConsumers` resolver returns a consumer only when projection id, exact `sourcePathIdentities` membership, anchored path-pattern evidence, adapter, and form all match. The exact identity check is conjunctive; a pattern is never runtime authorization by itself. The ticket harness also broadens a cloned CAD regex deliberately and proves that a counterfeit regex-matching path still resolves no consumer. There is no selector-less or basename fallback. The permanent language-neutral golden now records the four consumers and five current/canonical identities alongside both structured Cargo edits.

### TDD and final evidence

Golden-first red:

```text
bun .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/🧪️draw-package-entry-authority.ts
exit 1
actual destination: …/🦀️rust/📦️glue.rs
expected destination: …/🦀️rust/📚️library/🦀️.rs
```

Strict taxonomy validation:

```text
schemaVersion=7
semanticDirectoryKinds=104
semanticPathProjectionReferenceConsumerContracts=4
problems=[]
```

Final ticket-local authority harness:

```text
{"destinationDirectoryCount":9,"destinationNodeCount":20,"mappingDigest":"1f28fcc6e28e54001a9df6ce98b1c30b565cd42b824ed2491bb9b5e407b7436b","maxPathBytes":210,"referenceEdits":2,"resolvedConsumerIdentities":5,"sourceFileCount":11}
```

`bun nx run @semio-tech/repo-lib:lint` reached TypeScript and reported only the already-recorded unrelated `ImportMeta.env/glob` and cross-project `rootDir` diagnostics; it emitted no taxonomy/discovery diagnostic. The shared repo-lib test remained untouched under the normalization writer's lease. Its four stale assertions must be updated to digest `1f28fcc6…`, 9 directories, 20 nodes, and 210 bytes, and it may assert the two golden reference edits.
