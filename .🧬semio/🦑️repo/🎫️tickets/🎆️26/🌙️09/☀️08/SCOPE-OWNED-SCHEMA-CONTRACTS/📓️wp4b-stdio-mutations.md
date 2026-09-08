# WP4b — `🗄️stdio` mutation `$id` grammar + build-wave preparation

Partition: `✏️s/🔌️plugins/🗄️stdio/**/🧬️schema/🧬️mutations/**`, plus the stdio `🔮️oracle/🔣️.json`
mutation manifests and the `direct-mutation-contract` fixtures W8 already maintained.
Continues `📓️wp4-stdio-mutations.md`; scripts unchanged in location:
`wp4-stdio-schemas.py` (three new subcommands, §6) and `wp4-stdio-validate.mjs`.
Raw output: `🗑️generated/wp4b-stdio-ajv.txt`, `🗑️generated/schema-check-w8b.jsonl`.

## 1. Row 45 — the `$id` rewrite

Cross-partition row 45 settles §9's open question against the brief W8 followed: contract §A applies to
stdio too, and the leaf key is the descriptor's **`semanticKind`**, not the leaf directory path.

```
$ python3 <ticket>/wp4-stdio-schemas.py ids --write
leaves=915 distinct-ids=915 collisions=0 rewritten=915 already-correct=0 missing-file=0 (write=True)
owning-aggregates=88 distinct-aggregate-ids=88 aggregate-collisions=0 aggregate-rewritten=0

$ python3 <ticket>/wp4-stdio-schemas.py ids          # idempotence
leaves=915 distinct-ids=915 collisions=0 rewritten=0 already-correct=915 missing-file=0 (write=False)
owning-aggregates=88 distinct-aggregate-ids=88 aggregate-collisions=0 aggregate-rewritten=0
```

| | before | after |
|---|---|---|
| leaf (gltf, path ≠ kind) | `…/s/stdio/gltf/2.0/any/mutation/sampler/create.json` | `…/s/stdio/gltf/2.0/any/mutation/create-sampler/schema.json` |
| leaf | `…/s/stdio/las/1.0/header/mutation/set-point.json` | `…/s/stdio/las/1.0/header/mutation/set-point/schema.json` |
| aggregate | `…/s/stdio/las/1.0/header/mutations.json` | unchanged — already the row-45 form |

- **915 leaf `$id`s rewritten**, all 915 in one pass; nothing else in the documents changed (`ids` writes
  only the `$id` key, and `audit`'s comparison deliberately excludes `$id`, so the two commands cannot
  mask each other).
- **96 aggregate `$id`s** were already `…/<subset>/mutations.json`: 88 owning aggregates + the 8 gltf
  per-subset view catalogues that `command_projections` emits. `aggregate-rewritten=0` is the proof, not
  an omission. The 97th `🧬️mutations/🔣️.json` under stdio
  (`🧊️gltf/…/♾️any/🚪️io/🧬️mutations/🔣️.json`) is an `x-semio` collection descriptor under `🚪️io`, not a
  `🧬️schema/🧬️mutations` catalogue — out of partition, untouched (it is the only one with no `$id`).
- Aggregates keep their **relative** `$ref`s, per the brief. Unchanged, and the ajv oracle still resolves
  all 1035 of them on the filesystem.

### 1.1 Uniqueness proof

`semanticKind` is not the leaf directory name for 120 leaves (all gltf's `<domain>/<verb>` nesting), so
keying on it is a real re-mapping, not a rename. The bijection is checked by `command_ids` itself, which
builds the `$id → [leaf]` multimap before writing and refuses to claim success with a non-empty collision
set:

```
915 leaves → 915 distinct $ids → collisions=0
```

Independently: `verify` re-derives every expected `$id` from the descriptor and compares it to the file
on disk — `leaves=915 problems=0` (§2). The two agree.

### 1.2 Absolute `$ref`s pointing at old stdio leaf ids: none exist

```
$ grep -rn 'schema/s/stdio' --include='*.json' --include='*.ts' --include='*.rs' --include='*.mjs' \
        --include='*.tsx' --include='*.py' --include='*.go' --include='*.md' . | grep -v '/🎫️tickets/'
```

Every hit outside a `🧬️mutations` tree is a `"$id"` **declaration** of some other stdio document
(`🧬️schema/🔣️.json`, `📇️registry`, the `💡️inferences` facets), never a `"$ref"`. The single absolute
`$ref` anywhere in stdio is
`🧊️gltf/…/🚪️io/💡️inferences/📝️text/🔣️.json:13 → https://semio.tech/schema/s/stdio/gltf/inference.json`,
which targets an inference facet, not a mutation leaf. A second sweep with no `--include` filter for the
old id shape (`stdio.*mutation/<kebab>.json`) returned 791 files, **all** of them inside a
`🧬️mutations` tree, i.e. the leaves' own `$id` lines. **No cross-partition request is needed for row 45.**

The stdio `🔮️oracle/🔣️.json` manifests (106 files) carry no absolute schema URL at all — they mirror the
descriptor's *relative* `payloadSchema`, which this change does not touch. Same for the four
`🧪️tests/*direct-mutation-contract/🔣️.json` `requiredFiles` lists. Both were left alone deliberately.

## 2. Row 23 — nothing to drop in stdio

`x-semio-mutationKinds` **does not occur anywhere under `✏️s/🔌️plugins/🗄️stdio`**. The top-level key
census of the 96 stdio aggregates is `{$schema: 96, $id: 96, title: 96, description: 96, oneOf: 96}`; the
only `x-semio` key under a stdio `🧬️mutations` directory is on the out-of-partition `🚪️io` collection
descriptor. `schema check` agrees: `mutation-aggregate-kinds-redundant` = **59 repo-wide, 0 in this
partition**.

W8 built the aggregates as pure `$ref` unions with the discriminator expressed structurally (a `const`
on the tag property, or the external/adjacent branch shape), never as a kind list. So row 23 is a
**no-op for stdio** whether or not W2b's structural aggregate check lands, and stdio does not block it.
Row 23 stays open for W7's partition only.

## 3. Extra work in-partition: the 66 mutation codec facet documents

`schema check` put 83 `document-*` / dialect findings inside my glob that W8's report never mentions,
all on `🧬️mutations/📝️text/🔣️.json` and `🧬️mutations/💾️binary/🔣️.json` — the codec facet documents of
the mutations module, which are neither leaves nor aggregates. New subcommand `facets` (§6) repaired them:

```
$ python3 <ticket>/wp4-stdio-schemas.py facets --write
facet-documents=66 dialect-fixed=60 id-fixed=66 definitions-to-defs=1 title-fixed=6 (write=True)
distinct-facet-ids=66 collisions=0
```

| defect | count | fix |
|---|---:|---|
| no `$schema` at all | 59 | draft-07 |
| `$schema` was 2020-12 | 1 | migrated (`migrate_dialect`) then draft-07 |
| `$id` missing | 6 | contract §A |
| `$id` colliding (`mp4`/`avi` text vs binary, `dwg` ac1018/ac1024, `gif` 87a/89a) | 8 files / 4 ids | one `$id` per document |
| `$id` in an ad-hoc dotted shape (`stdio.epw/mutations/text.epw`, `stdio.mp4.mutations.json`, …) | 60 | `…/<artifact>/<standard>/<subset>/mutations/{text,binary}.json` |
| draft-07 `definitions` + `#/definitions/` refs (semio flow) | 1 file / 12 refs | `$defs` + `#/$defs/` |
| title is a generator slug (`Semio_mp3_mutations`, `Stdio_semio_mesh_mutation_text`, …) | 6 | PascalCase export id (`Mp3MutationsText`, `SemioMeshMutationsText`, …) |

The `$id` follows contract §A's facet rule — a facet document keeps its module's scope path and varies
only the facet filename — so the mutations text facet of a subset is `…/<subset>/mutations/text.json`
beside the aggregate's `…/<subset>/mutations.json`, exactly as `📸️snapshot/📝️text` is
`…/snapshot/text.json` beside `…/snapshot.json`.

Two content defects were found and **not** fixed, because fixing them is a deletion or a wire decision
outside this partition's mandate — see §7.4 and §7.5.

## 4. Verification (all commands run, output pasted verbatim)

```
$ python3 <ticket>/wp4-stdio-schemas.py verify
leaves=915 problems=0 stale-schema-files=0

$ python3 <ticket>/wp4-stdio-schemas.py audit
leaf schemas agreeing with their Rust payload=915 disagreeing=0 unprojectable=0 (write=False)

$ bun <ticket>/wp4-stdio-validate.mjs --all
aggregates=96 compiled=96 leaf-branches=1035 leaf-schemas=915 compiled-standalone=915 fixtures=188 failures=14
```

`problems=0`, `disagreeing=0`, `unprojectable=0`, `aggregates=96 compiled=96`,
`compiled-standalone=915` — all as W8 left them, i.e. the `$id` rewrite and the facet repair changed no
shape. `failures=14` is **unchanged and fully explained**: 7 fixtures × (aggregate + leaf), each a
pre-existing fixture-vs-Rust disagreement, none a schema defect —

| fixture leaf | root cause | owner |
|---|---|---|
| `💬️bcf/…/🗃️set-snapshot` | `BcfMutation` has no `#[value(tag)]`, fixture is the tagged form | row 49 → `wp4b-stdio-rust-changes.md` §2 |
| `🎵️mp3/…/📸️set-snapshot` | `Mp3Mutation`, same | row 49 |
| `📊️csv/…/📸️set-snapshot` | `CsvMutation`, same | row 49 |
| `📼️avi/…/📸️set-snapshot` | `AviStreamFormat` — `field_rename_all()` falls back `rename_all_fields → rename_all` | row 47a |
| `🧿️semio/v1/document/…` | `DocBlock`, same fallback | row 47a |
| `🧿️semio/v1/presentation/…` | same class | row 47a |
| `🧿️semio/v1/model/…` | `GeometryRef`, same fallback | row 47a |

The build wave takes 14 → 6 (rows 46+49) → 0 (row 47a). Derivation in
`wp4b-stdio-rust-changes.md` §4.

## 5. `schema check` — rows whose path is in this partition

```
$ bun ./📜️script.ts schema check --report <ticket>/🗑️generated/schema-check-w8b.jsonl
[schema check] modules=3194 scopes=2618 findings=7450
```

Filtering `🗄️stdio` × `🧬️mutations` (1751 of the 7450 rows):

| code | before this wave | **after** | note |
|---|---:|---:|---|
| `scope-id-duplicate` | 0 | **0** | target met (it was already 0 after W8) |
| `document-id-duplicate` | 2 | **0** | §3 |
| `document-id-missing` | 6 | **0** | §3 |
| `document-id-unaddressable` | 16 | **0** | §3 |
| `document-dialect-unexpected` | 59 | **0** | §3 |
| `export-id-invalid` | 3 | **0** | §3 |
| `ref-not-export-addressed` | 12 | **0** | §3, `definitions` → `$defs` |
| `mutation-aggregate-kinds-redundant` | 0 | **0** | §2 |
| `ref-not-catalog-addressable` | 1035 | 1035 | **contract-mandated**, see §5.1 |
| `mutation-leaf-id-grammar` | 491 | 491 | **grammar conflict**, see §5.2 |
| `module-scope-id-inconsistent` | 90 | 105 | same conflict; 50 aggregates + 55 codec facets |
| `module-level-ineligible` | 120 | 120 | taxonomy gap, see §5.3 |

Every code in my instructions' target list is at **0**. The four that remain are not mine to clear alone;
each names a decision that sits with W2c tooling or the coordinator.

### 5.1 `ref-not-catalog-addressable=1035` — expected

> `/oneOf/0/allOf/0/$ref uses "./➕insert-point/🧬️schema/🔣️.json"; a cross-scope reference names the target $id`

1035 = exactly the `leaf-branches` count. My brief says in as many words: *"Aggregates keep relative
`$ref`s"*, and contract §B says module aggregates are pure `$ref` unions. The check wants every aggregate
branch to name the leaf's `$id` instead. Those two cannot both be satisfied. **Coordinator decision
needed** (§7.1) — it is a 1-line change in `command_aggregates` and a re-run either way, but it also
changes what `wp4-stdio-validate.mjs` has to do (an absolute-`$id` union needs every leaf registered in
one Ajv instance, not filesystem resolution).

### 5.2 `mutation-leaf-id-grammar=491` — the tooling's grammar is not row 45's

> `$id "https://semio.tech/schema/s/stdio/las/1.0/header/mutation/set-point/schema.json"`
> `must be "https://semio.tech/schema/s.stdio.las/mutation/set-point/schema.json"`

The live check expects the **dotted artifact-level** scope id `s.stdio.las`; row 45 dispatched the
**slash subset-level** scope path `s/stdio/las/1.0/header`. I followed row 45, which is the later and
partition-specific coordinator decision, and which W8's §9 had explicitly escalated.

The check's own grammar is **not injective over this partition**:

```
915 stdio leaves → 766 distinct ids under s.stdio.<artifact>/mutation/<kind>/schema.json
                   80 colliding ids covering 229 leaves
   e.g. 5 × https://semio.tech/schema/s.stdio.ifc/mutation/set-snapshot/schema.json
        3 × https://semio.tech/schema/s.stdio.svg/mutation/set-transform/schema.json
```

Dropping `<standard>/<subset>` merges `ifc` 2x3-base with 4-any, `svg` 1.1's three subsets, `pdf`'s five
standards, `dwg` ac1018 with ac1024, `gif` 87a with 89a. Row 45's grammar is a bijection (§1.1). Unless
the tooling's scope-id derivation is meant to keep standard and subset, **row 45's form is the only one
of the two that can carry 915 distinct contracts**. Recorded as request §7.2; I did not change 915 ids a
second time on the strength of a check whose expected value collides.

Two secondary observations for W2c, independent of which grammar wins:

- the check reaches only **491 of the 915** leaves (28 of 36 artifacts; `avi` 12/12 leaves unflagged,
  `svg` 28/28 unflagged, `pdf` 21 of 104, `step` 10 of 38) and only **50 of the 88** aggregates and
  **55 of the 66** facet documents. The coverage gap does not correlate with leaf count, kind count or
  `module-level-ineligible`. Whatever the grammar, a check that silently skips half its subjects will
  read green on a half-migrated tree.
- `module-scope-id-inconsistent` on an **aggregate** and `mutation-leaf-id-grammar` on its leaves are the
  same disagreement counted twice.

### 5.3 `module-level-ineligible=120` — gltf's two-segment leaves

All 120 are `🧊️gltf/…/🧬️mutations/<domain>/<verb>/🧬️schema`, e.g.
`✅️required-extension/➕️add`, `🎛️sampler/🌱️create`. Contract §A declares the mutation-leaf owner level as
`…/🧬️schema/🧬️mutations/<leaf>` — one segment. gltf nests two, and has done since before this ticket;
the 120 directories are real leaves with real descriptors, and their `semanticKind` (`add-required-extension`,
`create-sampler`) is precisely what makes row 45's id keying work for them. **Taxonomy change**, §7.3.

## 6. Script changes (all inside the ticket folder)

`wp4-stdio-schemas.py` gained three subcommands and one flag; no behaviour of the existing five changed.

| subcommand | what it does |
|---|---|
| `ids [--write]` | rewrites leaf + aggregate `$id` to the contract §A grammar and proves the id map is injective before writing |
| `facets [--write]` | dialect / `$id` / `$defs` / export-id repair of the `📝️text`+`💾️binary` codec facet documents (§3) |
| `casing` | plans rows 46 + 49 without a build: the containers to annotate with file:line, and the fixtures the annotation re-cases |

`ASSUME_CAMEL_CASE` (module flag, consulted only by `field_wire_name`) lets `casing` project the
**post-change** crate. It is never set while authoring or auditing; `variant_wire_name` does not consult
it, so the simulation is exactly "`rename_all` on every un-annotated struct + `rename_all_fields` on every
un-annotated enum" and moves no variant name.

`leaves()` now derives `id` from `descriptor["semanticKind"]`; that is the whole of the row-45 change.

## 7. Cross-partition requests

1. **Coordinator — aggregate `$ref` form.** `schema check`'s `ref-not-catalog-addressable` (1035 rows
   here, 2752 repo-wide) requires aggregate branches to reference the leaf's absolute `$id`; my brief and
   contract §B require relative `$ref`s. Pick one. If absolute wins, stdio is one edit in
   `command_aggregates` + `aggregates --write`, and `wp4-stdio-validate.mjs` must switch from filesystem
   resolution to registering all 915 leaves in a single Ajv instance by `$id`.
2. **W2c tooling — reconcile the leaf/module scope grammar with row 45.** The check expects
   `s.stdio.<artifact>` where row 45 dispatched `s/stdio/<artifact>/<standard>/<subset>`. The check's
   expectation collides 229 of 915 stdio leaves onto 80 shared ids (§5.2). Either the check adopts row
   45's path form, or its scope-id derivation must retain standard and subset — in which case stdio
   re-runs `ids --write` with a one-line `ID_ROOT`/`leaves()` change and this partition is clean again in
   one command. Also: the check reaches only 491/915 leaves, 50/88 aggregates, 55/66 facet documents
   (§5.2), which needs fixing before anyone reads a 0 from it.
3. **W2c tooling — `schemaScopeOwnerLevels` must admit a two-segment mutation leaf.** 120 gltf leaves
   (`…/🧬️mutations/<domain>/<verb>`) are real declared leaves and are currently `module-level-ineligible`.
4. **Codec/parity owner — `🧬️mutations/📝️text` and `💾️binary` are unowned and half-scaffolded.** After
   §3 they are structurally valid, but their *content* is not: `mp4` and `avi` have byte-identical text
   and binary documents that merely restate the aggregate's kind list; several are literal placeholders
   (`"description": "🚧 scaffolded by W1b — generic facet mirror."`); `bcf`'s is `{"type": "object"}`;
   others (`🧿️semio/…/🔺️mesh`) are real hand-written OpText grammars. Title conventions vary across six
   spellings (`…MutationsText`, `…MutationText`, `…MutationTextDocument`, `…MutationOpText`,
   `…MutationDsl`, `…MutationTextOp`). I fixed only what was invalid, not what was merely inconsistent —
   deleting a restatement or unifying the export ids is a decision for whoever owns the codec facets and
   the multi-format parity policies (row 48).
5. **Parity-vector owner — 120 gltf `📜️contract/🔣️.json` files declare a JSON Schema dialect but are
   data.** `🧊️gltf/…/🧬️mutations/<domain>/<verb>/📜️contract/🔣️.json` carry
   `"$schema": "https://json-schema.org/draft/2020-12/schema"` with keys `id`, `languageParity`,
   `vectors`. They raise no finding (the check does not treat them as schema documents) so nobody will be
   told; the stray key should be deleted, not re-dialected. Contract §B: fixture data is never a schema.
6. **W8c (build wave) — rows 46 and 49 are fully prepared, and 46 is coupled to 47a.** Work order:
   **`wp4b-stdio-rust-changes.md`**. It carries the 585 containers with file:line and insertion point,
   the exact attribute for the three untagged aggregate enums, the 49 re-cased fixture bodies, and — new
   — the 17 enums that row 47a will silently snake-case unless row 46 adds `rename_all_fields` to them in
   the same commit. Two corrections to the request as written in the ledger:
   - the attribute for row 49 is `#[value(tag = "mutation", rename_all = "camelCase")]`, not
     `#[value(tag = "mutation")]`: without `rename_all` the discriminator is `"SetSnapshot"`, and the
     committed fixtures say `"setSnapshot"`. Reference: `☁️las/…/🧬️mutations/🦀️.rs:55`.
   - row 46's "440 structs" is **569 structs** measured descriptor-first across all 915 leaves (W8's 440
     counted only the 501 leaves it authored). 128 of them actually move the wire. 16 further containers
     without `rename_all` are **enums** and must be excluded — on an enum the attribute re-cases variant
     names, which for eight of them is the external aggregate key and for six is a glTF spec constant.
7. **Nobody owns the stdio non-mutation `$id`s.** `💡️inferences` facet documents across stdio declare
   ids like `https://semio.tech/schema/s/stdio/bcf/inference.json` and
   `…/s/stdio/ifc/2x3/inference.json` — an artifact-level path with standard and subset dropped or
   half-dropped, matching neither grammar in §5.2. Outside my partition; flagged so it is not mistaken
   for settled. `schema check` reports 415 `document-dialect-unexpected` and 189
   `module-scope-id-inconsistent` under `🗄️stdio` outside `🧬️mutations`.

## 8. Open questions

- **Which grammar is the repo's?** §5.2. Everything else in this report is stable under either answer;
  1011 ids move with a one-line change and two commands if the coordinator picks the dotted form. The one
  hard constraint is injectivity: the dotted **artifact-level** form cannot carry stdio's 915 leaves.
- **Are relative aggregate `$ref`s the contract or a stopgap?** §5.1 / §7.1. 1035 findings in this
  partition hang on it, and it is the single largest code in the repo-wide report (2752).
- **Do the codec facet documents earn their place?** §7.4. Six of the 66 are real grammars; the rest are
  scaffolding, restatements or empty objects. Structurally they are now correct, which makes them *look*
  settled — that is the risk in leaving them.
