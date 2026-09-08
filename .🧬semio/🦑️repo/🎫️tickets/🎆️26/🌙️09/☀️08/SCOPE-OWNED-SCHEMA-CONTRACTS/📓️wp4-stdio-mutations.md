# WP4 — `🗄️stdio` mutation-leaf payload schemas

Partition: `✏️s/🔌️plugins/🗄️stdio/**/🧬️schema/🧬️mutations/**` only. Scripts kept in this folder:
`wp4-stdio-schemas.py` (authoring/relocation/aggregates/audit/verify) and `wp4-stdio-validate.mjs`
(ajv draft-07 oracle). Raw validation output: `🗑️generated/wp4-stdio-ajv.txt`.

## 1. End state (measured, not asserted)

```
$ python3 <ticket>/wp4-stdio-schemas.py census
leaves=915 present=915 missing=0 already-canonical=915
declared payloadSchema spellings: [('🧬️schema/🔣️.json', 915)]

$ python3 <ticket>/wp4-stdio-schemas.py verify
leaves=915 problems=0 stale-schema-files=0

$ python3 <ticket>/wp4-stdio-schemas.py audit
leaf schemas agreeing with their Rust payload=915 disagreeing=0 unprojectable=0 (write=False)

$ bun <ticket>/wp4-stdio-validate.mjs --all
aggregates=96 compiled=96 leaf-branches=1035 leaf-schemas=915
compiled-standalone=915 fixtures=188 failures=14
```

`verify` checks, per leaf: the descriptor's `payloadSchema` is exactly `🧬️schema/🔣️.json`, the file
exists at that path, parses, carries the draft-07 dialect, the expected `$id`, and a non-empty
`title`; plus a repo-wide sweep for surviving `*.schema.json` files anywhere under a stdio
`🧬️mutations` tree (0 remain).

## 2. Counts

| action | count |
|---|---:|
| leaves with a descriptor in the partition | 915 |
| **authored** from the Rust payload (family F — declared `json-schema`, no file at the declared path) | **501** |
| **relocated** onto `<leaf>/🧬️schema/🔣️.json` (408 moved, 6 already canonical) | **414** |
| descriptors rewritten to `payloadSchema: "🧬️schema/🔣️.json"` | 915 |
| leaf schemas whose body disagreed with their own Rust payload and were replaced by the projection | 216 + 109 + 9 + 41 (four convergence passes, see §5) |
| aggregates `🧬️mutations/🔣️.json` rewritten as pure `$ref` unions | 88 |
| per-subset gltf *view* catalogues repointed at the owning subset's leaves | 8 |
| stale/orphan schema files deleted | 3 |
| `🔮️oracle`/`🧪️oracle` mutation-manifest `payloadSchema` mirrors updated | 752 fields in 76 files |
| `🧪️tests/*direct-mutation-contract/🔣️.json` `requiredFiles` entries updated | 36 in 4 files |
| gltf generator `📜️script.ts` doc references updated | 2 |

WP0 §5 reported 500 missing in stdio; the live count is **501** (WP0's census used a leaf-detection
heuristic; the descriptor-driven count here is 915 leaves / 501 missing). WP0's `🧬️mutations` root
count of 88 for stdio matches the 88 owning aggregates found here.

## 3. Family I and the two orphans WP0 did not see

WP0 flagged one broken descriptor/file pair. There were **three**, all the same defect — a schema
file sitting beside a leaf under a name no descriptor points at:

| leaf | descriptor said | file on disk | resolution |
|---|---|---|---|
| `🧊️gltf/…/🌳️node/🏷️rename` | `🧬️schema/🔣️.json` (present, correct, full adjacently-tagged `ChangeNodeNameMutation`) | `🏷️.schema.json` — stale `GltfChangeNodeNamePayload` shape | orphan deleted |
| `☁️las/…/📸️set-snapshot` | `🔣️.schema.json` (**absent**) | `🧬️.schema.json` describing `LasSnapshot` itself, not `SetSnapshot { snapshot }` | orphan deleted, schema authored |
| `🎒️zip/…/📸️set-snapshot` | `🔣️.schema.json` (**absent**) | `🧬️.schema.json`, same wrong nesting | orphan deleted, schema authored |

So Family I is not "the descriptor or the file needs correcting" — in all three cases the descriptor
was right about *where* the contract belongs and the loose file was superseded content. Contract §E
("delete superseded files in the same change") applied.

## 4. What the projection reads, and the two places it deviates from the brief

`wp4-stdio-schemas.py` projects `value_derive`
(`🧰️framework/🔨️modules/🌱️value/✨️derive/🦀️.rs` + `🌱️value/🔁️codec/🦀️.rs`), **not** serde. Every rule is
cited in the script. The load-bearing ones:

- container `#[value(rename_all = …)]` decides field wire names; **with no `rename_all` the wire name
  is the Rust identifier verbatim** (`field_wire_name`, derive:183-193).
- `Option<T>` encodes as `null` and decodes from `null` *or* absence → `anyOf: [T, {"type":"null"}]`,
  not required. This matches the convention the 414 pre-existing stdio schemas already used.
- `#[value(default)]`, `skip_serializing_if`, container `default` → not required. `skip` → absent from
  the wire entirely.
- one-field tuple structs and `#[value(transparent)]` are transparent (derive:53-63).
- enums: all-unit + no `tag` → `{"type":"string","enum":[…]}`; `tag`+`content` → adjacent `oneOf`;
  `tag` alone → internal (fields spliced beside the tag; a non-object newtype payload lands under
  `"value"`, derive:758-767); no `tag` with data variants → external `{"<Variant>": payload}`.
- `u8/u16/u32` carry `minimum`/`maximum` bounds.

### Deviation 1 — casing is the crate's, not unconditionally camelCase

The brief asked for "property names in camelCase per `schemaFormats.🦀️rust.fieldCasing` (snake →
camel)". **Applied literally this would produce schemas that reject real payloads.** 440 of the 500
projectable stdio payload structs carry no `#[value(rename_all)]` at all, so their wire names are
snake_case; 20 of those have multi-word fields where snake ≠ camel. Example, verified against the
crate and against the leaf's own committed fixture:

```
🧿️semio/v1/drawing/…/🖌️change-stroke-color  →  pub struct ChangeStrokeColor { style_name, new_color }   (no rename_all)
committed fixture                          →  {"ChangeStrokeColor": {"style_name": …, "new_color": …}}
pre-existing hand-authored schema          →  {"styleName", "newColor"}   ← wrong, ajv-rejected its own fixture
```

The schemas therefore carry the **actual** wire names. The taxonomy's `fieldCasing: camel` for JSON
Schema is a real conformance target, but reaching it is a **Rust change** (adding
`#[value(rename_all = "camelCase")]` to those payload structs and re-casing their fixtures), not a
schema change — see §8 cross-partition request 1.

### Deviation 2 — `format` on integers

The brief asked for `format` on unsigned integers. No stdio schema used `format` (0 occurrences
across the 414 pre-existing ones); the established convention is `minimum`/`maximum` bounds
(`maximum: 255` ×136, `65535` ×76, `4294967295` ×9). `format` is not a JSON-Schema-defined keyword for
integers and ajv ignores it, so it would add an inconsistency without adding a constraint. The
projection emits `minimum`/`maximum` instead.

### Deviation 3 — internally tagged modules declare their discriminator

45 of the 88 stdio aggregates are **internally tagged** (`#[value(tag = "mutation")]`, no `content`):
the wire object is the leaf's own entries *with the tag spliced in beside them*. A closed
(`additionalProperties: false`) payload schema cannot be `$ref`'d into that shape — draft-07 has no
`unevaluatedProperties`, so `allOf: [{$ref: leaf}, {tag}]` is rejected by the leaf's own
`additionalProperties: false`. The committed fixtures at those leaves are exactly that spliced form
(e.g. `☁️las/…/📸️set-snapshot/🧪️tests/…/🦠️mutation/🔣️.json` is `{"mutation":"setSnapshot","snapshot":…}`).

Resolution: in internally tagged modules the leaf schema declares its aggregate discriminator as a
**non-required `const`** property, mechanically derived from the module's `#[value(tag = …)]` plus
`variant_wire_name(descriptor.aggregateVariant, aggregate rename_all)` — never restated by hand:

```json
"mutation": { "const": "setPoint",
              "description": "Aggregate discriminator spliced in by LasMutation; absent when the payload stands alone." }
```

This keeps the payload closed, lets the aggregate stay a pure `$ref` union, and lets both the leaf
and the aggregate accept the leaf's own committed wire object. Adjacently and externally tagged
modules need nothing extra. The alternative — moving every internally tagged aggregate to
`content = "payload"` — is a wire-format change across 45 subsets and is offered as cross-partition
request 2.

## 5. Rust shapes the first pass could not map, and how each was resolved

The generator refuses rather than guessing; every refusal below was resolved by reading the crate.

| shape | leaves | resolution |
|---|---:|---|
| `store::ArtifactChild<S>` | 12 | hand-written `ToValue`/`FromValue` (`🏪️store/🦀️.rs:2817-2837`): `{childId, target: ArtifactRef}`, phantom + local owner never reach the wire. Encoded as a named override citing that impl. |
| generic structs/enums (`DwgVisualStyleProperty<u32>`, `GltfCollectionDiff<T, D>`) | 51 | real generic instantiation: parameters captured at index time, arguments substituted through the *current* bindings before rebinding (`GltfCollectionDiff<T,D>`'s field `Vec<GltfModified<D>>` only means something in the instantiation). |
| generic type **aliases** (`pub type GltfWeakCollectionDiff<T> = GltfCollectionDiff<T, T>;`) | 50 | aliases now carry their own generic parameters in the index. |
| `#[value(deserialize_with = "deserialize_double_option")]` | 2 | double-option = absent \| null \| value → optional, `anyOf` with null. |
| `#[value(with = "ordered_attr_map…")]` on `Vec<(String, usize)>` | 50 | read the codec body (`📸️snapshot/🦀️.rs:220-231`): object of accessor indices, not an array of pairs. |
| `GltfMorphTarget(Vec<(String, usize)>)` with hand-written impls | 50 | same object shape, named override citing `📸️snapshot/🦀️.rs:413-421`. |
| `#[value(deserialize_with = "deserialize_page")]` | 1 | `PagePayload` names exactly `PageDoc`'s own fields, so `PageDoc` is the honest shape. |
| internally tagged newtype variants with a **scalar** payload (`JsonPathSegment::Key(String)`, `XlsxCellValue::Number(f64)`) | 12 | derive:758-767 wraps a non-object payload under `"value"`, so the branch is `{tag: const, value: <scalar>}`. **Note the decode side (derive:973-986) strips the tag and hands the remaining object to `String::from_value`, which cannot succeed — an encode/decode asymmetry in `value_derive`, reported in §8.** |
| multi-field tuple structs, non-`String`-keyed maps | 0 | never occurred. |
| `📖️pdf/…/📥️insert-page`, `🧊️gltf` sampler/animation/etc. | 51 | all resolved by the above; **final unprojectable count is 0**. |

### A parser bug that silently dropped fields

The first comment stripper was a single regex `sub`. Leftmost-match semantics let a quote inside one
construct swallow the opener of the next, so a documented field could survive as unparsed text and be
**skipped without any error** — `OpcPackage.relationships` (`HashMap<String, Vec<OpcRelationship>>`,
preceded by a doc comment containing `""`) vanished from 9 xlsx/docx/pptx schemas. Fixed by replacing
it with a linear scanner over the four Rust lexical openers (string, raw string, char literal,
comment), and — more importantly — by making `parse_named_fields` record any text it cannot read as a
**gap** that refuses the leaf instead of dropping it. With the guard in place the whole partition
reports 0 gaps.

Because of this, convergence took four `audit --write` passes (216 → 109 → 9 → 41 leaves rewritten);
the last pass came from a separate fix (the aggregate enum is the one carrying `#[mutations(…)]`, not
merely the first `pub enum` in the module file — `🧿️semio/v1/value` declares a `SemioValuePathSegment`
enum above its aggregate and was picking up that enum's `kind` tag).

## 6. Aggregates

Every `🧬️schema/🧬️mutations/🔣️.json` is now a `oneOf` whose every payload is a relative `$ref` to
`./<leaf>/🧬️schema/🔣️.json`; no payload content is restated anywhere. The branch shape follows the
module's *own* aggregate enum, read from its `🦀️.rs`:

| representation | modules | branch |
|---|---:|---|
| internal (`tag` only) | 45 | `{"allOf": [{"$ref": leaf}, {"required": [tag], "properties": {tag: {"const": wire}}}]}` |
| external (no `tag`) | 30 | `{"required": ["<Variant>"], "properties": {"<Variant>": {"$ref": leaf}}}`, closed |
| adjacent (`tag` + `content`) | 13 | `{"required": [tag, content], "properties": {tag: {"const": wire}, content: {"$ref": leaf}}}`, closed |

Notes:
- the discriminator constant is `variant_wire_name(aggregateVariant, aggregate rename_all)` — the
  aggregate enum's own casing, **not** `semanticKind`. bmp is `kebab-case` (`insert-palette-entry`),
  las/dxf/most are `camelCase` (`setPoint`). The pre-existing files disagreed on this in several
  places; e.g. dwg ac1018's aggregate declared `maintenanceVersion` while the payload struct has no
  `rename_all` and puts `maintenance_version` on the wire.
- `🖊️dwg/4️⃣ac1018` owns no aggregate enum — it is `pub use crate::standards::v_ac1024::…::mutations::*;`.
  The re-export is followed to `🔟ac1024`'s enum, so ac1018's catalogue is generated from the same
  authority rather than hand-kept.
- the 8 per-subset gltf catalogues (`🎞️animation`, `💿️buffer`, `🎬️scene`, `💎️material`, `🕸️mesh`,
  `🪪️asset`, `🦴️skin`, `🎥️camera`) own no leaves; they were `$ref`ing `🎞️animation/🌱️create/🧬️.schema.json`
  **relative to themselves**, i.e. at paths that do not exist in those subsets — dangling in every
  case, and with kebab-case constants that the `♾️any` aggregate (`GltfMutation`, adjacent,
  camelCase) does not use. They are now views: same membership, `♾️any`'s representation, and
  `../../../♾️any/🧬️schema/🧬️mutations/<leaf>/🧬️schema/🔣️.json` refs that resolve.
- `🧊️gltf/…/♾️any/🚪️io/🧬️mutations/🔣️.json` is a `x-semio` collection descriptor under `🚪️io`, not a
  `🧬️schema/🧬️mutations` catalogue, and is out of the partition. Untouched.

## 7. Validation

`wp4-stdio-validate.mjs` (bun + ajv 8.20.0, draft-07) does three things:

1. resolves every aggregate `$ref` **on the filesystem** (they are relative file paths, so URI
   normalization is the wrong resolver — percent-encoding each emoji segment only invents a second
   identity), re-keys each leaf onto a `urn:` and compiles the aggregate. **96/96 compile.**
2. compiles every leaf schema standalone and checks its dialect. **915/915 compile, 915 draft-07.**
3. replays every leaf's committed `🧪️tests/*/🦠️mutation/🔣️.json` (188 fixtures) through the aggregate
   *and* through the leaf's own schema, unwrapping the payload according to the branch shape.

`failures=14` — 7 fixtures × (aggregate + leaf). **None of them is a schema defect**; each is a
pre-existing disagreement between a committed fixture and its own Rust, surfaced by this work, and
each needs a change outside a payload schema:

| fixture leaf | disagreement |
|---|---|
| `💬️bcf/2.1/markup/🗃️set-snapshot` | `BcfMutation` has **no** `#[value(tag)]` → externally tagged, wire is `{"SetSnapshot": …}`; the fixture is `{"mutation":"setSnapshot", …}`. |
| `🎵️mp3/mpeg1-layer3/any/📸️set-snapshot` | same |
| `📊️csv/rfc4180/any/📸️set-snapshot` | same |
| `📼️avi/1.0/hdrl/📸️set-snapshot` | `AviStreamFormat` is `#[value(tag="format", rename_all="camelCase")]` with **named-field** variants; `value_derive`'s `field_rename_all()` falls back to `rename_all`, so the wire is `bitCount`/`sizeImage`/`xPelsPerMeter`. Fixture has `bit_count`/`size_image`/`x_pels_per_meter`. |
| `🧿️semio/v1/document/📸️set-snapshot` | `DocBlock`, same fallback: wire `styleId`, fixture `style_id`. |
| `🧿️semio/v1/presentation/📸️set-snapshot` | same class (`blocks[].level`/casing). |
| `🧿️semio/v1/model/📸️set-snapshot` | `GeometryRef` `{Brep{brep_id}, Mesh{mesh_id}}`: wire `brepId`/`meshId`, fixture `brep_id`/`mesh_id`. |

The last four are one root cause: **serde's `rename_all` on an enum cases only variant *names*
(field casing needs `rename_all_fields`, serde ≥ 1.0.190), but `value_derive`'s `field_rename_all()`
(`✨️derive/🦀️.rs:226-228`) falls back from `rename_all_fields` to `rename_all`.** These fixtures were
written against serde semantics. I did **not** rewrite them: they are test vectors I cannot execute
here (stdio must not be built in this session), and CLAUDE.md forbids claiming a behaviour I have not
run. See §8 request 3.

### Policy commands (task 5)

`bun 📜️script.ts clean taxonomy inventory --kind mutation` — the `mutation/schema-parity` gate that
owns the 501-leaf gap — is **runnable again**: WP0 §4's blocker (`generatorContracts["print-latex-tokens"].previewTarget`
failing `loadTaxonomy()`) is gone, and the CLI now reaches the mutation phase. It still could not be
driven to completion, on three consecutive attempts, each time aborting mid-scan with

```
error: [clean taxonomy --kind mutation] admitted source disappeared before content capture: <path>
```

for a *different* file each run — `🧰️framework/…/📖️playbook/🦀️.rs`, `🧰️framework/…/🐹️entity_kinds.g.go`,
and `✏️s/🔌️plugins/💠️lowpoly/🧪️interactive-job/🧬️.schema.json`. The third is the sibling WP4 worker
deleting `🧬️.schema.json` files in its own partition right now; the command snapshots the whole repo
regardless of `--scope`, so it cannot complete while any partition is mid-relocation. Log:
`🗑️generated/wp4-stdio-policy-attempt.txt`. **It should be re-run once every WP4 partition has
landed**; for the stdio slice specifically, the two properties that gate checks — the descriptor
resolving to an admitted regular file under a traversal-free relative path, and that file parsing as
a Draft-07 document without duplicate keys — are verified independently above (915/915).

## 8. Cross-partition requests

1. **Rust (stdio artifact crates, `🧬️schema/📸️snapshot` + mutation leaves) — casing conformance.**
   The taxonomy declares `🔣️jsonschema.fieldCasing: "camel"`, but 440 stdio payload structs and many
   of their nested snapshot types carry no `#[value(rename_all = "camelCase")]`, so their wire names
   are snake_case and the schemas honestly say so. Bringing stdio to the declared casing means adding
   that attribute to those structs **and** re-casing their committed fixtures, then regenerating with
   `wp4-stdio-schemas.py audit --write`. It is a wire-format change and needs a build; I did not
   attempt it. The 20 top-level offenders are listed by
   `python3 wp4-stdio-schemas.py audit` output plus a grep for `pub struct` without `#[value(`.
2. **Rust (`🧬️mutations/🦀️.rs`, 45 stdio subsets) — optional: adjacent tagging everywhere.**
   `#[value(tag = "mutation", content = "payload")]` on the internally tagged aggregates would let the
   aggregate `$ref` the closed payload with no discriminator declared inside the leaf, removing
   Deviation 3. Purely optional; today's construction validates.
3. **Framework (`🧰️framework/🔨️modules/🌱️value/✨️derive/🦀️.rs:226-228`) — `field_rename_all()` fallback.**
   Falling back from `rename_all_fields` to `rename_all` for an enum variant's own fields is
   documented in that module as "serde's default too"; serde does not do this (that is exactly why
   `rename_all_fields` exists). Four stdio fixtures were authored against serde's behaviour and now
   disagree with the derive. Either the fallback goes, or those four fixtures are re-cased — a
   framework-owner decision, not a schema one.
4. **Framework (`🧰️framework/🔨️modules/🌱️value/✨️derive/🦀️.rs:758-767` vs `:973-986`) — internally
   tagged newtype asymmetry.** Encoding a scalar newtype payload emits `{tag, "value": scalar}`;
   decoding strips the tag and hands the remaining **object** to the payload type's `FromValue`, which
   a `String`/`f64` cannot accept. Affects `JsonPathSegment` (json/i-json, 7 leaves) and
   `XlsxCellValue` (xlsx, 5 leaves). The schemas describe the encode side, which is the observable
   wire form.
5. **Tooling worker (root `📜️script.ts`).** `mutationPayloadSchemaRelativePath()` already returns the
   canonical `🧬️schema/🔣️.json`, and `derivePayloadSchemas`/`payloadSchemaCommand`
   (`🧰️framework/…/🧪️test/📜️script.ts` `manifest payload-schema`) writes to `🔣️.schema.json`
   (`🟦️.ts:1298`). That filename no longer exists anywhere in stdio; running that generator with
   `--write` would re-create the pre-relocation convention. It should be pointed at
   `🧬️schema/🔣️.json`. Its projection also lacks the `value_derive` rules this ticket needed
   (no-`rename_all` verbatim casing, internally tagged splicing, generic aliases, the two custom
   codecs) — `wp4-stdio-schemas.py` is the tested reference for those.
6. **Nobody currently owns** `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/*/…/🔮️oracle/🔣️.json`. Those
   `mutation-manifest/v2` records mirror the descriptor's `payloadSchema` string; I updated the 752
   stdio ones so they stay honest. If another WP owns the manifest schema, note that the field is
   declared optional and is not path-validated by `mutationManifestProblems`.

## 9. Open questions

- **`$id` shape.** The brief specified `https://semio.tech/schema/s/stdio/<artifact>/<standard>/<subset>/mutation/<leaf>.json`
  (slash form) and that is what all 915 leaves and 96 aggregates now carry. It **diverges from the 206
  pre-existing stdio `$id`s**, which used the contract §A dotted scope form
  (`https://semio.tech/schema/s.stdio.pdf.1.7.e/mutation/set-output-intent.json`). stdio is now
  internally uniform on the slash form; if §A's dotted scope id is the repo-wide rule, all 1011 ids
  here are a one-line change in `ID_ROOT`/`leaves()` and a re-run of `audit --write` + `aggregates
  --write`. **Coordinator decision needed** — I followed the brief over the contract because the
  brief was specific to this partition.
- **`<leaf>` in the `$id` is the leaf's directory path, not `semanticKind`.** They differ for 120
  leaves, all gltf's `<domain>/<verb>` nesting (`🎛️sampler/🌱️create` → `sampler/create`, semanticKind
  `create-sampler`). Path-derived keeps `$id` a 1:1 map onto file location and is collision-free
  across all 915 (checked). If `$id` must key on `semanticKind` instead, that is a one-line change.
- **`🎬️sequence`-style `🛜️wire`/`🧬️wire` sibling schemas (WP0 families D/E) do not occur in stdio**;
  no stdio leaf carries a second contract file. Nothing to decide here.
- **Leaf `🟦️.ts` / `🔗️.graphql` / `🛰️.proto` surfaces were not touched.** Several leaves declare them
  in `requiredLanguageSurfaces`; the multi-format parity policies
  (`policyExtractJsonSchemaFields` and siblings, `📜️script.ts:30343`, `:30447`, `:30883`) compare the
  JSON Schema's `title` + field names against them. The projection sets `title` to the Rust payload
  type name and the field names to the real wire names, which should *improve* that comparison, but
  the other four surfaces may now be the drifted side. Out of this partition; flagged for whoever
  owns multi-format parity.
