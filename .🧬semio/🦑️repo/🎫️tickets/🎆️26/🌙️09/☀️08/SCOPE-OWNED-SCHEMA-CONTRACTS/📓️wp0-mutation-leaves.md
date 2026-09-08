# WP0 — Mutation-Leaf Schema Audit

Partition: `✏️s/🔌️plugins/*/🗿️artifacts/*/🏅️standards/*/🪆️subsets/*/🧬️schema/🧬️mutations/**` and the equivalent
`✏️editor/🎚️config`, `👥️presence`, `🫧️transient`, and plugin `🎚️config/🧬️schema/🧬️mutations` trees, compared
against the aggregate `🧬️mutations/{🔣️.json,🛰️.proto,🔗️.graphql,🦀️.rs,🟦️.ts}` in the same module.

Read-only audit. Repo MCP unavailable this session; ticket folder managed on disk. Census built from
`git ls-files -z` (working tree, not a git-object snapshot — other sessions' concurrent edits may shift
individual counts by a handful, but not the shape of the findings). Scripts used to build the tables live
in the session scratchpad, not the ticket folder (per the "don't leave scratch scripts outside the ticket
that get swept" lesson, and because none of this analysis needed re-running mid-session); the exact
commands are reproduced inline below so the census is re-derivable from this document alone.

## 1. What the taxonomy declares as authority

Source: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`.

```json
"testMutationVocabularyDirName": "🧬️mutations",
"mutationPayloadSchemaLocation": { "directoryKindId": "schema", "directoryName": "🧬️schema", "fileKindId": "json" },
"mutationPayloadSchemaAuthority": {
  "contractKind": "descriptor-linked-mutation-payload-schema",
  "ownerAuthority": "mutationOwnerIdentity",
  "descriptorFileKindId": "json",
  "descriptorField": "payloadSchema",
  "descriptorSchemaVersion": 1,
  "descriptorCardinality": "one-canonical-no-competing-descriptor",
  "descriptorOwnerField": "owner",
  "descriptorIdentityField": "semanticKind",
  "jsonSchemaDialect": "http://json-schema.org/draft-07/schema#",
  "targetAuthority": "owner-relative-regular-json-schema"
},
"schemaChildDirs": ["📸️snapshot", "🔺️diff", "🧬️mutations", "💡️inferences"],
"mutationComponentFileKindId": "rust-source",
"mutationDescriptorFileKindId": "json",
"mutationBehaviorFacetDirs": ["🦠️mutation", "🔺️diff", "↩️inverse"],
"mutationDirectLeafForbiddenRegionMarkers": { "↩️inverse": "🔖️Inverse", "🔺️diff": "🔖️Diff" },
"mutationOrganizationalFacetDirs": ["🧩️plan", "📝️text", "💾️binary", "🧬️schema"]
```

`_mutationOwnershipComment`: "Every concrete `🧬️mutations/<emoji><verb>-<noun>/` directory directly owns
one `🦀️.rs` holding only its payload and dispatch... whenever a mutation's own apply/payload-definition,
diff, or inverse behavior exists, it lives in its own facet directory — `🦠️mutation`, `🔺️diff`, `↩️inverse`
— and must never be inlined back into the direct leaf... `🧩️plan`, `📝️text`, `💾️binary` and `🧬️schema`
remain optional organizational facets, genuinely absent for many mutations and never a completeness
requirement."

**Reading, cross-checked against live code (`📜️script.ts`, `🧰️framework/…/📚️library/🔍️discovery/🟦️.ts`) and
real leaves (§3):**

- `targetAuthority: "owner-relative-regular-json-schema"` + `descriptorOwnerField: "owner"` means the
  authority for a mutation's payload JSON Schema is **the mutation leaf's own directory**. The descriptor's
  `owner` field is that leaf's own path (`.../🧬️mutations/<domain>/<verb>` or `.../🧬️mutations/<mutation-name>`),
  and `payloadSchema` is a path *relative to that owner*, not to the module or the artifact. This is a
  **descriptor-linked, per-leaf** authority by explicit declaration, not an accident of layout.
- `mutationOrganizationalFacetDirs` lists `🧬️schema` (the payload-schema facet) as **optional** —
  "genuinely absent for many mutations and never a completeness requirement." So the taxonomy itself does
  not mandate every leaf carry a schema; only leaves whose descriptor lists `"json-schema"` in
  `requiredLanguageSurfaces` are obligated to have one (verified in `📜️script.ts:29048` — see §4).
- `mutationPayloadSchemaRelativePath()` (`🔍️discovery/🟦️.ts:1358`) — docstring: "Renders the **default**
  schema location for a **newly authored** mutation, not existing descriptor authority" — resolves to
  `🧬️schema/🔣️.json` (nested, one subdirectory under the leaf). This is the scaffold default used by
  `newMutationDescriptor()` (`📜️script.ts:22422`, the `bun 📜️script.ts new mutation …` scaffolder) when
  `--json-schema` is requested. It is **not** what most existing leaves use (§2) — most were authored before
  this default existed, or the plugin chose a different filename by hand; the descriptor's literal
  `payloadSchema` string is what's authoritative in each individual leaf, not this function.
- `mutationDescriptorFileKindId: "json"` (→ `🔣️.json`) is the **descriptor**, not the payload schema. Every
  leaf's `🔣️.json` carries `schemaVersion, owner, semanticKind, displayName, emoji, aggregateVariant,
  payloadSchema, textOpcode, binaryTag, invertibility, diffParticipation, outcomeClasses, composition,
  requiredLanguageSurfaces` — exactly 14 keys, enforced byte-for-byte by a Rust proc-macro at compile time
  (§4).

**Conclusion on Task 5's core question:** the mutation-leaf `🧬️.schema.json`-family layout **is the
declared taxonomy authority**, not an ad hoc per-contract-directory violation. It satisfies "one schema
module per scope with named exports" at the *mutation* grain: each leaf is its own scope, its own owner,
and its payload schema is that scope's sole normative JSON Schema, referenced by exactly one descriptor
field (`descriptorCardinality: "one-canonical-no-competing-descriptor"`). The open question is not
"leaf vs aggregate" (leaf wins by declaration) but whether the **aggregate** module-level files stay
honest projections of that authority or drift into a second, competing authority — see §3 and §6.

## 2. Census (git-tracked files only; `git ls-files -z`)

Method: every path under any `🧬️mutations/` directory in the partition (2,691 in-scope roots' worth of
files) was grouped by its `🧬️mutations` root, then a "leaf" was identified as the directory owning a direct
`🦀️.rs` that is neither inside a behavior facet (`🔺️diff`, `↩️inverse`, `📝️text`, `💾️binary`, `🦠️mutation` —
attributed one level up to the true leaf) nor inside any `🧪️tests/` subtree (per-scenario fixture `.rs`
files, not leaf definitions). This reproduces `taxonomy.mutationDirectoryPattern`/`policyListMutationDirs`'s
notion of a leaf closely enough for a census (cross-checked: `with_descriptor` = 2690/2691, i.e. essentially
every leaf found this way really is one — a plugin with no descriptor at all would be a red flag, and there
is exactly one).

**2,691 mutation leaves** in the partition (module estimate `≈2,100` in the master plan's seed notes was for
`🧬️.schema.json` *files*, a strict subset — see "schema variant" columns below).

| plugin | 🧬mutations roots | leaves | direct 🦀️.rs | descriptor 🔣️.json | flat `🧬️.schema.json` | nested `🧬️schema/🔣️.json` | flat `🔣️.schema.json` | vcs wire pair | seq `🛜️wire/🔣️.schema.json` | 🔺diff facet | ↩inverse facet | 🧪tests dir |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| ✒️writer | 3 | 14 | 14 | 14 | 14 | 0 | 0 | 0 | 0 | 4 | 4 | 4 |
| ➗mathematical | 4 | 17 | 17 | 17 | 17 | 0 | 0 | 0 | 0 | 15 | 15 | 15 |
| 🌀procedural | 3 | 37 | 37 | 37 | 35 | 0 | 0 | 0 | 0 | 37 | 37 | 37 |
| 🌊flow | 1 | 11 | 11 | 10 | 10 | 0 | 0 | 0 | 0 | 9 | 9 | 10 |
| 🌍gis | 5 | 24 | 24 | 24 | 14 | 10 | 0 | 0 | 0 | 14 | 14 | 14 |
| 🌿vcs | 1 | 6 | 6 | 6 | 0 | 0 | 0 | 6 | 0 | 6 | 6 | 6 |
| 🎞animate | 3 | 12 | 12 | 12 | 12 | 0 | 0 | 0 | 0 | 9 | 9 | 9 |
| 🎥shooting | 3 | 41 | 41 | 41 | 41 | 0 | 0 | 0 | 0 | 31 | 31 | 31 |
| 🎪demonstrator | 1 | 1 | 1 | 1 | 1 | 0 | 0 | 0 | 0 | 1 | 1 | 1 |
| 🎬sequence | 4 | 13 | 13 | 13 | 13 | 0 | 0 | 0 | 8 | 8 | 8 | 8 |
| 🏗fem | 10 | 50 | 50 | 50 | 50 | 0 | 0 | 0 | 0 | 50 | 50 | 50 |
| 🏛architect | 3 | 268 | 268 | 268 | 268 | 0 | 0 | 0 | 0 | 266 | 266 | 266 |
| 🏭process | 1 | 16 | 16 | 16 | 16 | 0 | 0 | 0 | 0 | 16 | 16 | 16 |
| 💠lowpoly | 1 | 17 | 17 | 17 | 17 | 0 | 0 | 0 | 0 | 17 | 17 | 17 |
| 💡reasoning | 3 | 13 | 13 | 13 | 13 | 0 | 0 | 0 | 0 | 10 | 10 | 10 |
| 📋forms | 2 | 22 | 12 | 22 | 22 | 0 | 0 | 0 | 0 | 10 | 10 | 10 |
| 📏layout | 3 | 32 | 32 | 32 | 32 | 0 | 0 | 0 | 0 | 0 | 0 | 25 |
| 📐cad | 2 | 21 | 21 | 21 | 21 | 0 | 0 | 0 | 0 | 20 | 20 | 20 |
| 📕norm | 16 | 393 | 372 | 393 | 393 | 0 | 0 | 0 | 0 | 392 | 392 | 392 |
| 📖playbook | 3 | 13 | 13 | 13 | 13 | 0 | 0 | 0 | 0 | 9 | 9 | 9 |
| 📜imperative | 2 | 8 | 8 | 8 | 8 | 0 | 0 | 0 | 0 | 4 | 4 | 4 |
| 📸remodel | 3 | 43 | 43 | 43 | 43 | 0 | 0 | 0 | 0 | 35 | 35 | 35 |
| 🔋energy | 1 | 276 | 276 | 276 | 276 | 0 | 0 | 0 | 0 | 276 | 276 | 276 |
| 🔱trinity | 6 | 35 | 35 | 35 | 35 | 0 | 0 | 0 | 0 | 15 | 15 | 15 |
| 🕸dag | 3 | 18 | 4 | 18 | 18 | 0 | 0 | 0 | 0 | 14 | 14 | 14 |
| 🖍draw | 4 | 14 | **0** | 14 | 0 | 0 | 13 | 0 | 0 | 14 | 14 | 14 |
| 🖨raster | 1 | 12 | 12 | 12 | 12 | 0 | 0 | 0 | 0 | 12 | 12 | 12 |
| 🗄stdio | 88 | 924 | 923 | 924 | 410 | 6 | 0 | 0 | 0 | 138 | 138 | 312 |
| 🗒note | 10 | 38 | 38 | 38 | 38 | 0 | 0 | 0 | 0 | 33 | 33 | 33 |
| 🧩puzzle | 3 | 89 | 89 | 89 | 86 | 0 | 0 | 0 | 0 | 89 | 89 | 89 |
| 🧰framework | 29 | 101 | 101 | 101 | 5 | 96 | 0 | 0 | 0 | 0 | 0 | 6 |
| 🧱block | 3 | 104 | 104 | 104 | 104 | 0 | 0 | 0 | 0 | 104 | 104 | 104 |
| 🪐space | 2 | 5 | 5 | 5 | 5 | 0 | 0 | 0 | 0 | 5 | 5 | 5 |
| 🪵sourcing | 1 | 3 | 3 | 3 | 3 | 0 | 0 | 0 | 0 | 3 | 3 | 3 |
| **total** | **256** | **2691** | **2631** | **2690** | **2045** | **112** | **13** | **6** | **8** | **1666** | **1666** | **1872** |

One extra filename variant found by exhaustive scan, not matching any declared kind:
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🌳️node/🏷️rename/🏷️.schema.json`
— named with the *mutation's own* leading emoji (`🏷️`) instead of a schema-kind emoji (`🧬️`/`🔣️`). See §6
Family I.

**Reading:**
- **Five filename conventions actually exist** for "the leaf's own payload schema" — flat `🧬️.schema.json`
  (2,045, the majority), nested `🧬️schema/🔣️.json` (112, concentrated in `🧰️framework` internal test/registration
  fixtures and part of `🌍️gis`/`🗄️stdio`), flat `🔣️.schema.json` (13, `🖍️draw` only), and the `🌿️vcs`-specific
  pair `📋️.schema.json` (payload) + `🧬️wire/🔣️.schema.json` (wire envelope, 6 leaves), and `🎬️sequence`'s
  editor-config leaves additionally carry `🛜️wire/🔣️.schema.json` (8) alongside a flat `🧬️.schema.json`. None
  of this is inconsistent *within* a leaf — see §3 — but it is inconsistent *across* plugins, and the
  taxonomy's own scaffold default (`🧬️schema/🔣️.json`) is a minority pattern (112/2,176 schema-bearing
  leaves) even among newly-authored-looking leaves.
- **`🖍️draw` has zero leaves with a direct-root `🦀️.rs`.** All 14 sampled draw leaves put the actual mutation
  struct inside the `🦠️mutation/` facet directory instead of at the leaf root, contradicting
  `_mutationOwnershipComment`'s "every concrete mutation directory directly owns one `🦀️.rs`". `🕸️dag` (14/18
  missing) and `📕️norm` (21/393 missing) show the same shape at smaller scale. This is a `mutation/direct-owner`
  policy breach class (`📜️script.ts:28291`, `policyMutationDirectOwnerBreaches`), not specific to schema
  ownership, but it means the descriptor's `owner` field and the payload schema's directory anchor sit one
  level above where the actual Rust type lives for those leaves — worth flagging to whichever WP owns
  `mutation/direct-owner` remediation (this ticket is scoped to schema authority, not full mutation-leaf
  structural conformance).
- `🧬️wire/🔣️.schema.json` and `📋️.schema.json` are genuinely two different contracts, not two names for one
  thing (§3 vcs sample): the payload schema and a wire/envelope schema that `$ref`s into it.

## 3. Ten representative leaves (descriptor, leaf schema, aggregate cross-check)

| # | plugin | leaf | descriptor `payloadSchema` | file present? | leaf/aggregate relationship |
|---|---|---|---|---|---|
| 1 | 🏛️architect | `…/🏛️program/…/✳️any/🧬️schema/🧬️mutations/ℹ️information-requirement/🌱️create` | `🧬️.schema.json` | yes | **Drift.** Aggregate `$defs.CreateInformationRequirement` is a stub `{informationRequirement:{type:"object"}}`; the leaf's own `🧬️.schema.json` has the full 20-property nested schema (enums, required lists, nested objects). See §6 Family G-A. |
| 2 | 🗄️stdio | `…/☁️las/…/🎩️header/🧬️schema/🧬️mutations/✏️set-point` | `🔣️.schema.json` | **no** | Descriptor requires `json-schema` in `requiredLanguageSurfaces`; leaf directory contains only `🔣️.json` + `🦀️.rs`. Missing schema file — §6 Family F. |
| 3 | 📕️norm | `…/⚖️en1990/…/✳️any/🧬️schema/🧬️mutations/⚓️change-permanent-action` | `🧬️.schema.json` | yes | Present, consistent naming; not independently diffed against aggregate (norm's aggregate `🔣️.json` at 393 mutations is large — spot check below). |
| 4 | 🔋️energy | `…/🔋️model/…/✳️any/🧬️schema/🧬️mutations/↗️change-air-loop-supply-node` | `🧬️.schema.json` | yes | Present, `requiredLanguageSurfaces` also lists `text`/`binary` (has `📝️text`/`💾️binary` facets — not audited here, out of this partition's schema focus). |
| 5 | 🌿️vcs | `…/🌿️vcs/…/✳️any/🧬️schema/🧬️mutations/🏷️add-tag` | `📋️.schema.json` | yes | **Clean reference pattern.** Payload schema `$id …/mutation/add-tag/payload.json`; sibling `🧬️wire/🔣️.schema.json` (`$id …/mutation/add-tag.json`) `$ref`s the payload schema's `tag` property by URL (`…/payload.json#/properties/tag`) rather than duplicating it. Exemplar for §6 recommendation. |
| 6 | 🎬️sequence | `…/🎬️sequence/…/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/🏃️set-last-run` | `🧬️.schema.json` | yes | Present; editor-config subtree (not an artifact standard/subset), same descriptor shape. |
| 7 | 🖍️draw | `…/🖍️drawing/…/🎨️style/🧬️schema/🧬️mutations/🌓️set-layer-blend-mode` | `🔣️.schema.json` | yes | Present, but the mutation struct itself lives in `🦠️mutation/🦀️.rs`, not at leaf root (§2 finding). |
| 8 | 🌍️gis | `…/🏔️gisterrain/…/✏️editor/🎚️config/🧬️schema/🧬️mutations/🎥️set-camera` | `🧬️schema/🔣️.json` (nested) | yes | **Clean reference pattern.** Aggregate `🔣️.json` at the config root uses `allOf`+`$ref: ".../set-camera#/$defs/payload"` (by `$id` URL) plus a local `operation` const — no duplicated property list. |
| 9 | 🪐️space | `…/🏠️home/…/✳️any/🧬️schema/🧬️mutations/🔢️change-catalog-generation` | `🧬️.schema.json` | yes | **Cleanest reference pattern found.** Aggregate `🔣️.json` is `{oneOf:[{$ref:"./🔢️change-catalog-generation/🧬️.schema.json"}], "x-semio-mutationKinds":[...]}` — a relative-path `$ref` straight into the leaf file, zero duplication, zero drift surface. |
| 10 | ✒️writer | `…/✒️writer/…/✏️editor/🎚️config/🧬️schema/🧬️mutations/⚙️set-editor-settings` | `🧬️.schema.json` | yes | Present; this editor-config module has no aggregate `🔣️.json` at all at the `🧬️mutations` root — only per-leaf files plus the module's own `🧬️schema/🔣️.json` one level up (artifact-level schema, not mutation-catalog union). Aggregation for this module happens at a different grain than architect/space/gis; not a violation, just a fourth aggregate shape (see §6 open question). |

**Drift verdict:** of the 5 leaves with a comparable module-level aggregate union schema (architect, gis,
space, and by extension vcs/sequence's wire schemas), **one duplicates the leaf's property definitions as a
hand-maintained stub that has already drifted (architect)**, and **the rest reference the leaf by `$ref`
and cannot drift** (gis, space, vcs). This is the central finding for the recommendation in §6.

## 4. Consumers

- **Rust proc-macro, compile time** — `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️.rs:629-660`
  (`#[derive(dsl::Mutations)]` / `#[mutation_leaf(...)]` expansion, region `🔣️MutationLeafJson`). Parses
  each leaf's `🔣️.json` descriptor at macro-expansion time, rejects duplicate keys
  (`mutation_leaf_reject_duplicate_keys`), requires **exactly** the 14 declared keys
  (`MUTATION_LEAF_DESCRIPTOR_KEYS`, line 649), and asserts `descriptor.owner == authority.owner` and
  `descriptor.semanticKind` matches the registered domain owner. It captures `payload_schema` as an opaque
  string (line 645: `let payload_schema = string("payloadSchema")?;`) — it validates the *descriptor* is
  well-formed and self-consistent, but does not itself open or validate the referenced JSON Schema file's
  content. This is the single strongest compile-time enforcement point: a malformed or misattributed
  descriptor fails the Rust build, not just a lint.
- **`📜️script.ts` (root), structural policy gate** — `mutationPayloadSchemaProblems`
  (`🧰️framework/…/📚️library/🔍️discovery/🟦️.ts:1401`) is invoked as `policyMutationPayloadSchemaProblems`
  (`📜️script.ts:28259`) from inside `policyMutationStructuralBreachesView`
  (`📜️script.ts:28977-29090`, the `"mutation/schema-parity"` policy family, surface spec at line 29019).
  For every leaf whose descriptor lists `"json-schema"` in `requiredLanguageSurfaces`, it (a) resolves
  `owner/payloadSchema` as a strictly local, traversal-free, `.json`-suffixed relative path, (b) requires
  every path segment up to the file to be an admitted regular directory, (c) requires the file itself to be
  an admitted regular file, (d) parses it as JSON and validates Draft-07 keyword shapes
  (`mutationPayloadSchemaDocumentProblems`, line 1364) and rejects duplicate JSON keys
  (`jsonDocumentDuplicateKeys`). It also separately checks (surface loop, line 29063-29070) that the
  **aggregate** root file at `${mutationsRel}/🔣️.json` contains some textual identity for the mutation
  (`semanticKind`/`variantName`) — an *identity/name* check, not a structural-equality check against the
  leaf's schema body. This is exactly why Family G-A (architect's stub) passes today: the aggregate's
  `$defs.CreateInformationRequirement` key exists and is named correctly, so the identity check is
  satisfied even though the body has drifted.
  - This gate is wired to `bun 📜️script.ts clean taxonomy inventory --kind mutation` /
    `runMutationTaxonomyCli` (`📜️script.ts:21286-21402`, `inventoryMutationTaxonomy` at line 21153). **I
    attempted to run it live** (`bun 📜️script.ts clean taxonomy inventory --kind mutation --format json`)
    to get an authoritative breach list instead of hand-deriving one; it failed before reaching the
    mutation checks with `Invalid taxonomy schema: generatorContracts["print-latex-tokens"].previewTarget
    must route exactly to its declared script invocation…` (and the same for `report-actor-network`) — a
    pre-existing, unrelated `loadTaxonomy()` validation failure elsewhere in the taxonomy (print/report
    generators, nothing to do with mutations or schemas), most likely a concurrent session's in-flight
    edit. This blocks the *whole* `clean taxonomy` CLI repo-wide right now, not just this partition. I did
    not touch it (out of scope, and against the "ignore unrelated churn, don't fix other sessions' state"
    instruction) — flagging it here since it also blocks WP1's planned test-based verification of this
    gate.
- **Scaffolder (write path, not a schema consumer)** — `newMutationDescriptor`/`newMutationRustLeaf`
  (`📜️script.ts:22402-22422`) generate a *new* leaf's descriptor and an empty schema stub
  (`{$schema, title, type:"object"}`, line 22575) when `--json-schema` is passed. It never touches the
  aggregate.
- **Test module payload-schema generator** — `derivePayloadSchemas`/`payloadSchemaCommand`
  (`🧰️framework/…/🧪️test/📦️packages/🟦️typescript/🟦️.ts:4210`, wired at
  `🧰️framework/…/🧪️test/📜️script.ts:1281`, CLI segment `manifest payload-schema`). Reflects each leaf's Rust
  payload struct into a JSON Schema (`rustTypeToJsonSchema`) as a **derivation candidate for fixture
  authoring**, and writes it to disk only if `existsSync(path)` is false and `--write` is passed — this
  writes to a file literally named `🔣️.schema.json` at the leaf (per its own `join(leaf, "🔣️.schema.json")`,
  line 1298), which is the *draw* filename convention, not the majority `🧬️.schema.json` convention nor the
  taxonomy scaffold default (`🧬️schema/🔣️.json`). This is a **third, independent generator with its own
  fourth filename opinion** — it happens not to collide with existing files today (I did not find any leaf
  with both `🧬️.schema.json` and a tool-generated `🔣️.schema.json`), but it is one more source of
  convention drift if ever run with `--write` across a plugin that uses the flat `🧬️.schema.json` majority
  convention.
- **Reference/import resolution test** — `🧰️framework/…/📚️library/🧪️tests/🔎️json-reference-owner-lookup/🟦️.ts`
  exercises `$ref`/`$id` resolution generally (not mutation-specific, but the same mechanism vcs/gis/space
  leaf schemas rely on for cross-file references — see §3).
- **No TypeScript runtime reader of leaf schema *content*** was found outside the policy/derivation tooling
  above — no `ajv`/schema-compiler consumer loads `🧬️.schema.json` files to validate live payloads at
  runtime; the only runtime JSON-Schema production path is the unrelated `dsl::schema` shape-based
  generator (`🧰️framework/…/💻️os/🔨️modules/🗣️dsl/🧬️schema/🦀️.rs:218-327`, `shape_json_schema`/
  `record_spec_json_schema`), which derives schema from a `Shape`/`RecordSpec` DSL value for tool-calling
  gateways and is architecturally unrelated to the static mutation-leaf files.
- **`🧪️tests/` fixture harness** at each leaf (present for 1,872/2,691 leaves, §2) consumes the leaf's own
  `🦀️.rs` and produces `🎯️outcome`/`📸️snapshot`/`🔺️diff` fixtures under `🔣️.json`; it does not appear to
  load the sibling payload schema file directly (fixtures are Rust-value-driven, not schema-validated at
  the harness level as far as this partition's tooling shows).

## 5. Missing declared payload schema — the headline drift finding

Method: for every leaf with a `🔣️.json` descriptor (2,690), parsed `requiredLanguageSurfaces`; where it
includes `"json-schema"`, resolved `owner + "/" + payloadSchema` and checked the file exists on disk.

| | count |
|---|---:|
| descriptor requires `json-schema`, file present | 2,172 |
| descriptor requires `json-schema`, file **missing** | **505** |
| descriptor does not require `json-schema` | 13 |

505/2,677 (18.9%) of leaves that declare a JSON-Schema payload surface have **no file at the declared
path**. This is overwhelmingly one plugin:

| plugin | required | present | missing |
|---|---:|---:|---:|
| 🗄️stdio | 914 | 414 | **500** |
| 🧩️puzzle | 89 | 86 | 3 |
| 🌀️procedural | 37 | 35 | 2 |
| all other 30 plugins | 1,637 | 1,637 | 0 |

`🗄️stdio` alone is missing 500/914 (54.7%) of its declared payload schemas. Examples (leaf directory
contains only `🔣️.json` + `🦀️.rs`, no schema file at all, though the descriptor requires one):

```
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/🎩️header/🧬️schema/🧬️mutations/✏️set-point
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/🎩️header/🧬️schema/🧬️mutations/🏢set-system-identifier
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/🎩️header/🧬️schema/🧬️mutations/🗃️set-vlr-data
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/🎩️header/🧬️schema/🧬️mutations/📏set-scale-and-offset
```

(Note `🏢set-system-identifier` and `📏set-scale-and-offset` are also missing the variation-selector
separator between the leading emoji and the text — `🏢set-system-identifier` not `🏢️set-system-identifier`
— a second, narrower naming-conformance defect in the same plugin, not counted separately here.)

Per §4, `policyMutationPayloadSchemaProblems`/the `mutation/schema-parity` gate is the tooling that is
*supposed* to catch exactly this — every one of these 505 leaves should already fail
`bun 📜️script.ts clean taxonomy inventory --kind mutation` today. I could not confirm this live because the
CLI currently fails earlier on an unrelated taxonomy validation error (§4); this finding is instead
confirmed by direct filesystem inspection against each leaf's own descriptor, which is the authoritative
source per §1.

## 6. Finding families and recommendation

### Family A — flat `🧬️.schema.json` leaf (majority pattern)
- **Path pattern:** `…/🧬️schema/🧬️mutations/<domain>[/<verb>]/🧬️.schema.json`
- **Blob count:** 2,045
- **Current role/owner:** payload JSON Schema, owned by the leaf; referenced by the leaf's own `🔣️.json`
  descriptor (`payloadSchema` field).
- **Intended owner + exports:** unchanged — this is the correct, declared pattern (§1). Export identity =
  `(leaf owner path, "payload", "json-schema")`; consumers resolve by descriptor, not by filename
  convention.
- **Formats:** Draft-07 JSON Schema only at this file; sibling `🦀️.rs`/`🟦️.ts`/`🔗️.graphql`/`🛰️.proto` at
  the same leaf carry the other format surfaces per `requiredLanguageSurfaces`.
- **Consumers:** §4 (Rust derive macro reads the descriptor; `mutation/schema-parity` gate reads this file's
  bytes and validates Draft-07 shape).
- **References to rewrite:** none — keep as-is.
- **Decision:** **keep.** This is the taxonomy-declared authority.
- **Evidence:** §1, §3 samples 1/3/4/6/9/10.
- **Open questions:** none.

### Family B — nested `🧬️schema/🔣️.json` leaf variant
- **Path pattern:** `…/🧬️mutations/<domain>[/<verb>]/🧬️schema/🔣️.json`
- **Blob count:** 112 (96 in `🧰️framework` internal registration-test fixtures, 10 in `🌍️gis`, 6 in `🗄️stdio`)
- **Current role/owner:** same role as Family A (payload JSON Schema), same descriptor-linked authority —
  just the taxonomy's own scaffold-default filename (`mutationPayloadSchemaRelativePath()`, §1) instead of
  the majority's flat name.
- **Intended owner + exports:** unchanged; functionally equivalent to Family A.
- **Formats:** Draft-07 JSON Schema.
- **Consumers:** same as Family A — the gate resolves whatever `payloadSchema` says, so this variant is
  already correctly handled by existing tooling; it is not a broken consumer, only a naming split.
- **References to rewrite:** none required for correctness. If a single canonical filename is later chosen
  repo-wide (recommended below), these 112 are already on the taxonomy's own declared default and would be
  the *unchanged* side of any rename.
- **Decision:** **keep functionally; flag for convention consolidation** (not a violation, but see the
  cross-plugin recommendation below).
- **Evidence:** §2 table, §3 sample 8 (gis).
- **Open questions:** should the taxonomy pick one canonical filename (flat vs nested) and normalize the
  other 2,045+13+6 leaves onto it, or formally bless both as equally valid? Given `_mutationOwnershipComment`
  never mentions a nested `🧬️schema/` facet under a mutation leaf as required, and the flat form is >18x
  more common, the flat form is the de facto standard; the nested form's own justification
  (`mutationPayloadSchemaLocation`) reads as intended for a different layer (artifact/module `🧬️schema/`
  siblings such as `📸️snapshot`, not a facet nested one level below a mutation leaf) — this needs the
  taxonomy author's decision, not a unilateral rewrite.

### Family C — flat `🔣️.schema.json` leaf variant (draw only)
- **Path pattern:** `…/🧬️mutations/<domain>/🔣️.schema.json`
- **Blob count:** 13, all under `✏️s/🔌️plugins/🖍️draw`
- **Current role/owner:** payload JSON Schema; descriptor-linked, same as Family A.
- **Intended owner + exports:** unchanged.
- **Formats:** Draft-07 JSON Schema.
- **Consumers:** same gate; also incidentally the exact filename the test module's
  `payloadSchemaCommand`/`derivePayloadSchemas` generator would write to any *other* plugin missing a schema
  (§4) — so draw's hand-authored convention and the generator's default convention happen to already agree,
  which is worth knowing before extending that generator's `--write` mode to other plugins (it would silently
  standardize on this name, not on Family A's, unless changed first).
- **References to rewrite:** none required.
- **Decision:** **keep; note generator/convention collision above for whoever owns WP2/WP7 mechanism work.**
- **Evidence:** §2 table, §3 sample 7.
- **Open questions:** should `🔣️.schema.json` become the repo-wide canonical name instead of `🧬️.schema.json`,
  given it's what the automatic-derivation tool already emits? (Mutually exclusive with Family B's question
  — the two candidate canonical names disagree with each other too.)

### Family D — vcs payload + wire schema pair
- **Path pattern:** `…/🧬️mutations/<domain>/📋️.schema.json` (payload) + `…/🧬️mutations/<domain>/🧬️wire/🔣️.schema.json` (wire envelope)
- **Blob count:** 6 leaves × 2 files = 12 files
- **Current role/owner:** two distinct contracts, correctly separated: the payload schema is the mutation's
  argument shape; the wire schema is the on-the-wire envelope (`{mutation: const, ...payload fields}`) and
  `$ref`s the payload schema by `$id` URL rather than duplicating it (§3 sample 5).
- **Intended owner + exports:** unchanged. This is the **best-designed** sample found — two named exports
  from one leaf (`payload`, `wire`), one `$ref` relationship, zero duplication.
- **Formats:** Draft-07 JSON Schema, both files.
- **Consumers:** `mutation/schema-parity` gate covers the payload file identically to Family A; the wire
  file is validated by the same `requiredLanguageSurfaces` mechanism if declared (not independently
  confirmed for wire specifically — descriptor only names one `payloadSchema` field, so the wire schema's
  own gate coverage should be checked by whoever owns vcs, if it isn't already covered by the identity check
  at the aggregate `🔗️.graphql`/`🛰️.proto` level for vcs's module).
- **References to rewrite:** none.
- **Decision:** **keep as the reference example for Family G's recommendation below.**
- **Evidence:** §3 sample 5, file contents quoted there.
- **Open questions:** is `🧬️wire/🔣️.schema.json`'s gate coverage as strong as the payload's, or does it only
  get checked incidentally via GraphQL/protobuf identity? Not confirmed in this pass — narrow, vcs-specific,
  low blob count.

### Family E — sequence `🛜️wire/🔣️.schema.json`
- **Path pattern:** `…/🧬️mutations/<domain>/🛜️wire/🔣️.schema.json`, alongside a flat `🧬️.schema.json` payload
- **Blob count:** 8, all under `✏️s/🔌️plugins/🎬️sequence`'s editor-config leaves
- **Current role/owner:** same pattern as Family D (payload + wire pair), different emoji for the wire
  facet directory (`🛜️` vs `🌿️vcs`'s `🧬️`).
- **Intended owner + exports:** unchanged.
- **Decision:** **keep; note the facet-directory-name split from Family D** (`🧬️wire` vs `🛜️wire`) as a
  second small naming inconsistency for the same convention-consolidation question as Family B/C.
- **Evidence:** §2 table.
- **Open questions:** should the wire-facet directory name be unified across vcs and sequence?

### Family F — missing declared payload schema
- **Path pattern:** any leaf whose `🔣️.json` descriptor lists `"json-schema"` in `requiredLanguageSurfaces`
  but has no file at the resolved `owner/payloadSchema` path.
- **Blob count:** 505 leaves (0 files — that's the defect), 500 of them in `🗄️stdio`.
- **Current role/owner:** none — the contract is declared but unfulfilled.
- **Intended owner + exports:** the leaf itself, per §1 — this is not a placement question, it's a
  completeness gap.
- **Formats:** N/A (missing).
- **Consumers:** the `mutation/schema-parity` gate (§4) is designed to reject exactly this; I could not
  confirm it currently runs clean-to-red repo-wide because the CLI errors out earlier on an unrelated
  taxonomy validation failure (§4/§5).
- **References to rewrite:** none (nothing references these — that's the point); 505 new schema files need
  authoring, most straightforwardly via the test module's `payloadSchemaCommand --write`
  (`bun 🧰️framework/…/🧪️test/📜️script.ts manifest payload-schema --write`, scoped per subset) which derives
  them from each leaf's own Rust payload struct — but see Family C's note that this generator currently
  targets the `🔣️.schema.json` filename, not stdio's already-used-elsewhere `🧬️.schema.json`/nested
  `🧬️schema/🔣️.json` conventions, so it should not be run blind against stdio without first deciding which
  filename convention stdio's remaining 500 leaves should use.
- **Decision:** **real gap, out of this ticket's remediation scope** (this ticket audits ownership/authority
  shape, not schema completeness) — **flag to whichever WP owns completeness/generation** (WP2 mechanism or
  a dedicated stdio-schema-completeness ticket).
- **Evidence:** §5.
- **Open questions:** was `🗄️stdio`'s 500-leaf gap always there, or is it new since the last time
  `mutation/schema-parity` ran clean? Not determinable from a single filesystem snapshot; git blame /
  ticket history would answer it but is out of this pass's time budget.

### Family G — aggregate module-level union files (`🧬️mutations/{🔣️.json,🦀️.rs,🟦️.ts,🛰️.proto,🔗️.graphql}`)
Two sub-patterns observed, not one:

- **G-A, duplicated stub (architect):** aggregate `$defs.<Variant>` inline-redeclares the payload shape as
  a shallow stub (`{type:"object"}` for nested fields) instead of referencing the leaf. **This is the one
  confirmed drift instance**: `🔣️.json`'s `$defs.CreateInformationRequirement` has
  `informationRequirement: {type:"object"}` while the leaf's `🧬️.schema.json` has the full ~20-property
  schema with enums and nested objects (§3 sample 1, full diff quoted there). The `.rs` aggregate does **not**
  have this problem — it is a thin enum wrapper (`CreateInformationRequirement(super::create_information_requirement::CreateInformationRequirement)`)
  that re-exports the leaf's own Rust type, so Rust has no duplication; only the hand-maintained JSON Schema
  aggregate duplicates and drifts.
- **G-B, `$ref` composition (space, gis, vcs-wire):** aggregate is a thin `oneOf`/`allOf` union that `$ref`s
  each leaf's own schema file (relative path for space: `"$ref": "./🔢️change-catalog-generation/🧬️.schema.json"`;
  `$id`-URL for gis: `"$ref": "https://semio.tech/schema/gis/gis3d/set-camera#/$defs/payload"`; `$id`-URL
  for vcs wire: `"$ref": "https://semio.tech/schema/s/vcs/vcs/mutation/add-tag/payload.json#/properties/tag"`).
  Zero duplication, zero drift surface, and it's what "consumers resolve named exports" (this ticket's
  stated goal) literally looks like in JSON Schema.
- **Blob count:** ~256 aggregate `🔣️.json` files repo-wide in this partition (one per `🧬️mutations` root);
  not all sampled, so the G-A/G-B split ratio across all 256 is **not** established by this pass — only
  confirmed for the 4 modules sampled (architect=G-A, space/gis/vcs=G-B).
- **Current role/owner:** transparent assembly/union of the leaves in its module — correctly *not* an
  independent authority per `_mutationOwnershipComment` ("The `🧬️mutations` root is transparent assembly
  only"), but G-A's JSON Schema file violates that by carrying independently-maintained content.
- **Intended owner + exports:** the aggregate should never own payload content; it should export only the
  union/discriminator, referencing each leaf by `$ref`.
- **Formats:** JSON Schema (this family); `.rs` aggregate already correctly does this via Rust module
  re-export/wrapping in every sample checked, so no `.rs` remediation needed.
- **Consumers:** `mutation/schema-parity`'s aggregate-side check (§4) only verifies the aggregate contains
  the mutation's *identity* (name string), not structural equality — which is exactly why G-A's drift is
  invisible to current tooling.
- **References to rewrite:** every G-A-style aggregate `🔣️.json` `$defs` entry that inlines a payload shape
  instead of `$ref`ing its leaf.
- **Decision:** **the leaf is the single owner (confirmed, §1); the aggregate JSON Schema files must become
  pure `$ref` unions (G-B shape) with no inlined payload content.** This is the actual "duplicate authority"
  violation in this partition — not the leaf/aggregate split itself (which is declared and correct), but
  G-A's specific choice to hand-duplicate content that G-B proves can be a clean reference instead.
- **Evidence:** §3 samples 1 (G-A), 5 (G-B/vcs), 8 (G-B/gis), 9 (G-B/space).
- **Open questions:** how many of the 256 aggregate roots are G-A vs G-B? Not established — needs a
  repo-wide `$defs`-vs-`$ref` scan across all 256, which is straightforward (one jq/python pass over the
  aggregate files already enumerated in `by_root.json`) but was not run to completion in this pass given
  the time budget; flagging as the concrete next step rather than guessing the ratio.

### Family H — leaves without a direct-root `🦀️.rs`
- **Path pattern:** leaf directories whose only `🦀️.rs` lives under a `🦠️mutation/` facet, not at the leaf
  root.
- **Blob count:** 60 leaves (14/14 in 🖍️draw sampled roots, 14/18 in 🕸️dag, 21/393 in 📕️norm, 10/11 in
  🌊️flow, 1/924 in 🗄️stdio).
- **Current role/owner:** structural — the descriptor's `owner` field still correctly names the leaf
  directory (not the `🦠️mutation` subfolder) in every case checked, so the schema-authority claim in §1
  still resolves correctly even where the Rust type does not directly sit there. This is a
  `mutation/direct-owner` policy question, not a schema-authority one.
- **Decision:** **out of this ticket's scope** (schema contracts, not mutation-leaf Rust placement) —
  flagging for the WP that owns general mutation-leaf structural conformance.
- **Evidence:** §2 table (rust column), §3 sample 7 (draw).
- **Open questions:** none for this ticket; cross-reference to whichever ticket tracks
  `mutation/direct-owner` breaches.

### Family I — broken descriptor/file reference (naming bug)
- **Path pattern:** single instance —
  `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🌳️node/🏷️rename/`
- **Blob count:** 1
- **Current role/owner:** descriptor declares `payloadSchema: "🧬️schema/🔣️.json"` (Family B nested
  convention), but the actual file present is `🏷️.schema.json` (flat, wrong emoji — the mutation's own
  leading emoji `🏷️` used in place of a schema-kind emoji). The declared path resolves to nothing (counted
  in Family F's 505); the actual file is orphaned/unreferenced by its own descriptor.
- **Decision:** **real bug** — either the descriptor's `payloadSchema` field or the file needs correcting to
  match; a one-file fix, not a pattern.
- **Evidence:** §2 "other schema variants" note; both file contents inspected directly.
- **Open questions:** none — this is unambiguous once both files are read side by side.

## Coverage

- Read: `🔣️taxonomy.json` (relevant keys quoted in full in §1), `📜️script.ts` (mutation policy/scaffold
  regions, `~28200-29110` and `~21150-21410` and `~22380-22600`), `🧰️framework/…/📚️library/🔍️discovery/🟦️.ts`
  (`mutationPayloadSchemaRelativePath`, `mutationPayloadSchemaDocumentProblems`, `mutationPayloadSchemaProblems`,
  lines 1320-1425), `🧰️framework/…/🧪️test/📜️script.ts` (`payloadSchemaCommand`, `scaffoldCommand`,
  ~1264-1340) and its `🟦️.ts` package (`derivePayloadSchemas`, ~4150-4220), the `dsl` derive macro
  (`🧰️framework/…/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️.rs`, region `🔣️MutationLeafJson`, ~609-700), and the
  `dsl::schema` runtime shape-schema module (unrelated consumer, ruled out — §4).
- Census: exhaustive over `git ls-files -z` for all 2,691 leaves in the stated partition (all 34 plugins
  listed in §2's table cover every plugin directory found under `✏️s/🔌️plugins/*` that has a `🧬️mutations`
  tree, plus `✏️editor`/`👥️presence`/`🫧️transient` roots where present — none of the latter three actually
  occur inside this partition's `by_root.json`; every mutations root found groups under a plugin, editor-config,
  or plugin-level `🎚️config` path).
- Sampled in full (descriptor + schema + aggregate cross-check): the 10 leaves in §3, covering architect,
  stdio, norm, energy, vcs, sequence, draw, gis, space, writer as required.
- **Not done / explicitly out of scope for this pass:**
  - The G-A vs G-B ratio across all 256 aggregate roots (only 4 sampled) — flagged as the concrete next
    step in Family G.
  - Live confirmation via `bun 📜️script.ts clean taxonomy inventory --kind mutation` — blocked by an
    unrelated, pre-existing `loadTaxonomy()` failure (`generatorContracts["print-latex-tokens"/"report-actor-network"].previewTarget`)
    outside this partition; not something this ticket should fix. Findings in §5/§6 are instead confirmed
    by direct filesystem/descriptor inspection, which is the same authority the gate itself reads from.
  - Whether `🧬️wire`/`🛜️wire` schemas (Families D/E) have their own independent gate coverage as strong as
    the payload schema's — narrow, low blob count, flagged as an open question rather than resolved.
  - Git-history attribution of when the 505 missing-schema leaves (Family F) diverged — out of this pass's
    time budget.
