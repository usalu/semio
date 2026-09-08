# WP4c (W8d) — `🗄️stdio` mutation aggregates by absolute `$id` + codec facet cleanup

Partition: `✏️s/🔌️plugins/🗄️stdio/**/🧬️schema/🧬️mutations/**`.
Continues `📓️wp4-stdio-mutations.md` and `📓️wp4b-stdio-mutations.md`. Dispatched work:
cross-partition rows **79** (aggregate `$ref` form) and **81** (codec facet documents + gltf
`📜️contract` stray `$schema`). Scripts unchanged in location: `wp4-stdio-schemas.py`,
`wp4-stdio-validate.mjs`. Raw output: `🗑️generated/schema-check-w8d.jsonl`.

## 0. On-disk state found at start (W8c left no report)

| probe | result |
|---|---|
| aggregate `$ref` forms | `{'relative': 1035}` — **0 absolute**; row 79 not applied on disk |
| `git status … \| grep 🧬️mutations` | 66 `M`, all `📝️text`/`💾️binary/🔣️.json` — W8b's `facets --write`, nothing newer |
| `wp4-stdio-schemas.py` | `command_aggregates` **already carried** the row-79 edit (branch target = `leaf["id"]`, with the row-79 comment); `command_projections` did not |
| `wp4-stdio-validate.mjs` | unchanged — still re-keyed relative `$ref`s onto `urn:` per aggregate |

So W8c got as far as one function body in the script and stopped before running it. Everything below
re-derives from the source and was run to completion.

## 1. Row 79 — aggregate `oneOf` branches reference leaves by absolute `$id`

**Bare `$id`, never `<$id>#/$defs/Payload`.** Proven, not assumed: of the 915 leaf schema documents,
**0 declare a `$defs.Payload`** — the leaf document root *is* the payload — so the addressable export is
the document itself.

```
$ python3 -c "… 915 leaf 🧬️schema/🔣️.json …"
leaf schema files: 915   with $defs.Payload: 0   no $id: 0   distinct ids: 915   dups: []
```

### 1.1 What changed

- `command_aggregates` (already edited by W8c) run for the first time: **88 owning aggregates**
  rewritten, `oneOf` the only key that moved.
- `command_projections` **fixed by me**: the 8 gltf per-subset view catalogues still emitted
  filesystem-relative `$ref`s into a sibling subset. A view is an aggregate too, so its branches now
  name the owning leaf's absolute `$id` via the same `leaf_identity()`. Two supporting changes so the
  command stays re-runnable:
  - new `branch_reference(branch)` — reads the single leaf `$ref` out of a branch whatever the tagging
    shape (`allOf[0]`, or the one `properties.*` carrying a `$ref`). The old code hard-coded
    `properties.payload.$ref`, which stops matching the moment the branches are rewritten.
  - the reference→leaf resolution accepts the absolute form first (`by_id` map built from `leaves()`),
    falling back to the old sibling-subset path walk. Without this the second run could not read its
    own output.

```
$ python3 <ticket>/wp4-stdio-schemas.py aggregates --write
aggregates=88 {'internal': 45, 'external': 30, 'adjacent': 13} skipped=0 (write=True)
$ python3 <ticket>/wp4-stdio-schemas.py aggregates --write      # idempotent
aggregates=88 {'internal': 45, 'external': 30, 'adjacent': 13} skipped=0 (write=True)

$ python3 <ticket>/wp4-stdio-schemas.py projections --write
projections=8 skipped=0 (write=True)
$ python3 <ticket>/wp4-stdio-schemas.py projections --write     # idempotent
projections=8 skipped=0 (write=True)
```

Byte-level diff of the 88 owning aggregates against their pre-run snapshot: **`oneOf` is the only key
that differs, in all 88** (`unchanged: 8 changed: 88 / keys that differ: {'oneOf': 88}` — the 8
unchanged were the projections, rewritten in the following step).

Before → after, one branch of `las 1.0 header`:

```
-  {"$ref": "./➕insert-point/🧬️schema/🔣️.json"}
+  {"$ref": "https://semio.tech/schema/s/stdio/las/1.0/header/mutation/insert-point/schema.json"}
```

### 1.2 Proof that every aggregate `$ref` equals an existing leaf `$id`

Asked for by the brief in case the checker still mis-keys leaf ids (row 78). It does — §5.2 — so here
is the independent proof, run against the tree as it stands:

```
PROOF-1 aggregate branch refs resolve to declared leaf $ids
  aggregates=96 refs=1035 absolute=1035 relative=0
  distinct refs=915 leaf $ids on disk=915
  refs with no declaring leaf=0
  leaf $ids never referenced=0
```

1035 branches, 915 distinct targets, 915 leaves on disk, the two sets are **equal** — no dangling
reference and no orphan leaf. (1035 > 915 because the 8 gltf views re-reference the owning subset's
leaves; 120 of the 1035 are view branches.)

### 1.3 Validator: one Ajv keyed by `$id`

`wp4-stdio-validate.mjs` rewritten as row 79 requires. It no longer resolves `$ref`s on the filesystem
and no longer invents a `urn:` identity per aggregate. Instead:

- one `Ajv` registry per run; **every** leaf schema (915) and **every** codec facet document (27) is
  `addSchema`'d under its own declared `$id` — a document without an `$id` is a failure, not a silent
  skip;
- each of the 96 aggregates is compiled **in that registry**, so a branch resolves exactly the way the
  catalog will resolve it;
- a branch whose `$ref` names no registered leaf `$id` is now a reported failure rather than a
  silently-skipped branch (the old `registered.get(...) === undefined → continue`);
- new counters `facets` / `facets-compiled` / `registered` so the facet documents are covered by the
  same dialect + compilability bar as the leaves.

```
$ bun <ticket>/wp4-stdio-validate.mjs --all
aggregates=96 compiled=96 leaf-branches=1035 leaf-schemas=915 compiled-standalone=915 \
  facets=27 facets-compiled=27 registered=942 fixtures=188 failures=14
```

`aggregates=96 compiled=96`, `compiled-standalone=915`, `failures=14` — **all three as the brief
predicted, and unchanged from W8b.** The 14 are the same seven fixtures × (aggregate + leaf), all
pre-existing fixture-vs-Rust disagreements owned by the build wave (rows 46/47a/49, table in
`📓️wp4b-stdio-mutations.md` §4): `bcf/set-snapshot`, `mp3/set-snapshot`, `csv/set-snapshot` (row 49,
untagged aggregate enums) and `avi`, `semio/document`, `semio/presentation`, `semio/model` (row 47a,
`field_rename_all()` fallback). Not one is a schema defect and not one moved in this wave.

An interim run, before §2's deletions, showed `facets=65 facets-compiled=63 failures=18`: the two extra
failures were `semio/✉️base` and `semio/🔢️value`, whose `📝️text` documents declared `"type": "value"` —
**not a JSON Schema type**, so ajv refused them outright. Both were W1b placeholders and are gone (§2).

## 2. Row 81 — the codec facet documents

66 documents were in W8b's set. **65 sit under `🧬️schema/🧬️mutations/{📝️text,💾️binary}` (my
partition); the 66th is `🧊️gltf/…/♾️any/🚪️io/🧬️mutations/📝️text/🔣️.json`**, an `🚪️io` collection
descriptor, which W8b itself declared out of partition for the aggregate but then edited as a facet.
`facet_documents()` now requires the `🧬️mutations` parent to be `🧬️schema`, so the tool no longer
reaches it. See §6.1.

### 2.1 Nobody requires them to exist — verified, not assumed

| policy / reader | what it actually names | needs the facet `🔣️.json`? |
|---|---|---|
| `mutation/language-parity` (`📜️script.ts:24116-24117`) | `🧬️mutations/📝️text/🦀️.rs`, `🧬️mutations/💾️binary/🦀️.rs` | no |
| `mutation/schema-parity` (`📜️script.ts:24113-24115`) | `🧬️mutations/{🔣️.json,🔗️.graphql,🛰️.proto}` — the **aggregate** | no |
| `policyGrammarParseabilityBreaches` / `policyProtocolParseabilityBreaches` | `📝️text/📖️.grammar.semio`, `💾️binary/📡️.protocol.semio` | no |
| `policyTaxonomyDirsBreaches` (`representationDirs`) | directory **names** only | no |
| `🔣️taxonomy.json` | `representationDirs: ["📝️text","💾️binary"]`; no `fixedDirectoryContracts` entry for a file inside them | no |
| `🔣️schema-catalog.json` | hashes them — derived artifact, regenerated | no |
| Rust `include_str!` | **one hit repo-wide**: `🎨️svg/…/🧬️schema/📸️snapshot/🦀️.rs:1409` → `../🧬️mutations/📝️text/🔣️.json` | yes, svg only (a keeper) |

Reference sweep for the 38 deletion candidates: their 38 `$id`s and the two module-relative path forms
grepped repo-wide (excluding `target/`, `node_modules/`, `.git/`, `🎫️tickets/`) → **no hit outside the
files themselves**, plus one doc comment in `📚️library/🔍️discovery/🟦️.ts:2789` naming
`🔺️diff/📝️text/🔣️.json` as an example path. So the brief's fallback ("if a policy requires their
existence, make each a minimal honest document…") does not apply: **deletion is the correct branch.**

### 2.2 Classification and result — 38 deleted, 27 kept

Criteria applied, in order:

| code | criterion | n |
|---|---|---:|
| **D1** | body is exactly `{"type": "object"}` — an empty stub, and for a `📝️text` facet a *wrong* one | 28 |
| **D2** | `description` starts `🚧 scaffolded by W1b — generic facet mirror.` | 5 |
| **D3** | body's only content is a `mutation` enum that **equals the aggregate's own discriminator list** — a restatement carrying nothing the aggregate does not already own | 5 |

D3 was verified against each document's sibling aggregate rather than asserted: all five compare
`MATCH` (mp4 `📝️text`+`💾️binary`, avi `📝️text`+`💾️binary`, `semio/🎞️animation` `📝️text`). mp4's and
avi's two are additionally byte-identical to each other, exactly as W8b §7.4 reported.

Deleted (38): `las`, `epw`, `zip`, `gif`(87a, 89a), `ifc`(4, 2x3 text+binary), `bcf`, `binary`, `csv`,
`step`, `tsv`, `xlsx`, `pdf`(1.7), `docx`, `md`, `pptx`, `txt`, `stl`, `dwg`(ac1018, ac1024), `dxf`,
`deflate`, `obj`, `ply`, `json`, `semio/🔊️audio` [D1]; `mp3`, `wav`, `semio/✉️base`,
`semio/🔢️value`, `semio/🖼️image` [D2]; `mp4`×2, `avi`×2, `semio/🎞️animation` [D3].
No directory was emptied by this (the two empty `📝️text`/`💾️binary` dirs under stdio are gltf `♾️any`,
empty before this wave).

Kept (27) — every one describes the **codec's own wire form**, which is what a codec facet document is
for, and titles are now `<AggregateTitle>Text` / `<AggregateTitle>Binary`:

| scope | facet | title | shape |
|---|---|---|---|
| `🌐️html 🔖️5 ✳️any` | 📝️text | `HtmlMutationText` | string |
| `🎨️svg 🔖️1.1 🧱️base` | 📝️text | `SvgMutationText` | string |
| `📖️pdf 4️⃣1.4 🧱️base` | 📝️text | `PdfMutationText` | string+pattern |
| `📰️xml 🔖️1.0 🧱️base` | 📝️text | `XmlMutationText` | string |
| `📷️png 🔖️1.2 ✳️any` | 📝️text | `PngMutationText` | string+pattern |
| `📸️jpg 🔖️jfif-1.01 🧾️document` | 📝️text | `JpgMutationText` | string+pattern |
| `🖼️tiff 🔖️6.0 🧾️document` | 📝️text | `TiffMutationText` | string+pattern |
| `🪟️bmp 🔖️v3 ✳️any` | 📝️text | `BmpMutationText` | string+pattern |
| `🧿️semio 🔖️v1 🌊️flow` | 📝️text | `SemioFlowMutationText` | object(mutation,snapshot,node,edge,…) |
| `🧿️semio 🔖️v1 🎬️video` | 📝️text | `SemioVideoMutationText` | string |
| `🧿️semio 🔖️v1 🏛️model` | 📝️text | `SemioModelMutationText` | string |
| `🧿️semio 🔖️v1 📊️table` | 💾️binary | `SemioTableMutationBinary` | object(format,tag,payload) |
| `🧿️semio 🔖️v1 📊️table` | 📝️text | `SemioTableMutationText` | string |
| `🧿️semio 🔖️v1 📐️cad` | 📝️text | `SemioCadMutationText` | string |
| `🧿️semio 🔖️v1 📑️document` | 📝️text | `SemioDocumentMutationText` | string+pattern |
| `🧿️semio 🔖️v1 📦️object` | 💾️binary | `SemioObjectMutationBinary` | object(format,tag,payload) |
| `🧿️semio 🔖️v1 📦️object` | 📝️text | `SemioObjectMutationText` | object(line) |
| `🧿️semio 🔖️v1 📽️presentation` | 📝️text | `SemioPresentationMutationText` | object(keyword,args) |
| `🧿️semio 🔖️v1 🔤️text` | 💾️binary | `SemioTextMutationBinary` | object(format,tag,payload) |
| `🧿️semio 🔖️v1 🔤️text` | 📝️text | `SemioTextMutationText` | object(keyword,argsWire) |
| `🧿️semio 🔖️v1 🔺️mesh` | 📝️text | `SemioMeshMutationText` | string+pattern |
| `🧿️semio 🔖️v1 🕸️graph` | 💾️binary | `SemioGraphMutationBinary` | object(format,tag,payload) |
| `🧿️semio 🔖️v1 🕸️graph` | 📝️text | `SemioGraphMutationText` | object(keyword,args) |
| `🧿️semio 🔖️v1 🖊️drawing` | 📝️text | `SemioDrawingMutationText` | string |
| `🧿️semio 🔖️v1 🧊️brep` | 📝️text | `SemioBrepMutationText` | string |
| `🧿️semio 🔖️v1 🧰️kit` | 💾️binary | `SemioKitMutationBinary` | object(format,tag,payload) |
| `🧿️semio 🔖️v1 🧰️kit` | 📝️text | `SemioKitMutationText` | object(line) |

### 2.3 Title unification

`facet_documents()["title"]` no longer builds a name out of the path. It is now
`aggregate_title(<module>) + "Text"|"Binary"` — the `title` of the aggregate this document is a codec
facet **of**. Every stdio aggregate title already ends in `Mutation`, so the result is exactly the
`<Aggregate>MutationText` / `…MutationBinary` the row asks for, and `command_facets` enforces equality
instead of only repairing non-PascalCase strings.

```
$ python3 <ticket>/wp4-stdio-schemas.py facets --write
facet-documents=27 dialect-fixed=0 id-fixed=0 defs-to-definitions=1 title-fixed=16 (write=True)
distinct-facet-ids=27 collisions=0
$ python3 <ticket>/wp4-stdio-schemas.py facets                 # idempotent
facet-documents=27 dialect-fixed=0 id-fixed=0 defs-to-definitions=0 title-fixed=0 (write=False)
distinct-facet-ids=27 collisions=0
```

The six spellings W8b listed (`…MutationsText`, `…MutationTextDocument`, `…MutationOpText`,
`…MutationDsl`, `…MutationTextOp`, `…MutationOpFrameHeader`) are gone; 16 documents were re-titled.

### 2.4 `$defs` → `definitions` in the facet documents (row 77)

W8b moved `semio/🌊️flow`'s six named subschemas from draft-07 `definitions` into `$defs`. Row 77 (since
decided) says `$defs` holds **exports** and `definitions` holds module-internal helpers, and `schema
check` proved the consequence: `export-id-duplicate` — `SemioFlowSnapshot` was declared both by
`🧬️mutations/📝️text/🔣️.json` (`$defs`) and by `📸️snapshot/📝️text/🔣️.json` (its `title`), two files
for one export id in one scope. A codec facet document exports exactly one thing, its `title`, so every
named subschema in it is a helper. `command_facets` now performs the inverse move
(`$defs`→`definitions`, `#/$defs/`→`#/definitions/`); one document affected, and the finding is at 0.

## 3. gltf `📜️contract/🔣️.json` — stray `$schema`

Row 81 says 120; the census says **120 contract documents, of which 8 carry the stray key**:

```
contract docs: 120
top-level key sets: {('id','laws'): 66, ('id','laws','referencePolicy','touchedPaths'): 27,
                     ('command','id','laws','referencePolicy','touchedPaths'): 17,
                     ('$schema','id','languageParity','vectors'): 8, ('id','laws','vectors'): 2}
$schema values: {None: 112, 'https://json-schema.org/draft/2020-12/schema': 8}
```

The 8 are `🌲️scene-root/{🔗️bind,✂️unbind}`, `🌳️node/{🏷️rename,📝️change-extras}`,
`🌿️node-child/{🔗️bind,✂️unbind}`, `💎️material/{🌫️change-alpha,🪞️change-sides}`.

**Reader check (the brief's condition):** `languageParity` has **no reader anywhere in the repo** — the
only 8 occurrences are the 8 declaring files themselves. Nor does anything read gltf's
`📜️contract/🔣️.json` at all (the only `📜️contract` importers are framework actor/plugin tests reading
their own kernel contracts). So nothing needed the key, and nothing needs the dialect.

Removed textually, one line per file, so the hand-authored compact formatting survives:

```
$ git diff -- …/🌳️node/🏷️rename/📜️contract/🔣️.json
 {
-  "$schema": "https://json-schema.org/draft/2020-12/schema",
   "id": "s.stdio.gltf.mutation.change-node-name.v1",
   "languageParity": ["rust", "typescript"],
```

`gltf contract documents=120 stray-$schema-removed=8 / remaining with $schema: 0`. Data untouched.

## 4. Verification — every command run, output verbatim

```
$ python3 <ticket>/wp4-stdio-schemas.py verify
leaves=915 problems=0 stale-schema-files=0

$ python3 <ticket>/wp4-stdio-schemas.py audit
leaf schemas agreeing with their Rust payload=915 disagreeing=0 unprojectable=0 (write=False)

$ python3 <ticket>/wp4-stdio-schemas.py ids
leaves=915 distinct-ids=915 collisions=0 rewritten=0 already-correct=915 missing-file=0 (write=False)
owning-aggregates=88 distinct-aggregate-ids=88 aggregate-collisions=0 aggregate-rewritten=0

$ python3 <ticket>/wp4-stdio-schemas.py aggregates
aggregates=88 {'internal': 45, 'external': 30, 'adjacent': 13} skipped=0 (write=False)

$ python3 <ticket>/wp4-stdio-schemas.py projections
projections=8 skipped=0 (write=False)

$ python3 <ticket>/wp4-stdio-schemas.py facets
facet-documents=27 dialect-fixed=0 id-fixed=0 defs-to-definitions=0 title-fixed=0 (write=False)
distinct-facet-ids=27 collisions=0

$ bun <ticket>/wp4-stdio-validate.mjs --all
aggregates=96 compiled=96 leaf-branches=1035 leaf-schemas=915 compiled-standalone=915 \
  facets=27 facets-compiled=27 registered=942 fixtures=188 failures=14
```

No stdio build was run (brief: python3 + ajv only). §6.2 is the one place where that matters.

## 5. `schema check` — rows whose path is in this partition

```
$ bun ./📜️script.ts schema check --report <ticket>/🗑️generated/schema-check-w8d.jsonl
[schema check] modules=3203 scopes=3021 findings=10391
```

Filtering `🗄️stdio` × `🧬️mutations`: **1009 of the 10391 rows** (W8b measured 1751).

| code | W8b | **W8d** | note |
|---|---:|---:|---|
| `ref-not-catalog-addressable` | 1035 | **0** | row 79 — §1 |
| `export-id-duplicate` | — | **0** | was 1 mid-wave; row 77 — §2.4 |
| `document-id-duplicate` | 0 | **0** | |
| `document-id-missing` | 0 | **0** | |
| `document-id-unaddressable` | 0 | **0** | |
| `document-dialect-unexpected` | 0 | **0** | |
| `export-id-invalid` | 0 | **0** | |
| `ref-not-export-addressed` | 0 | **0** | |
| `mutation-aggregate-kinds-redundant` | 0 | **0** | |
| `scope-id-duplicate` | 0 | **0** | |
| `module-level-ineligible` | 120 | **0** | row 80 landed: two-segment gltf leaves are now eligible |
| `mutation-leaf-id-grammar` | 491 | 915 | one disagreement, §5.2 — coverage gap fixed (491→915 of 915) |
| `mutation-aggregate-id-grammar` | — | 50 | same disagreement, new code |
| `module-scope-id-inconsistent` | 105 | 24 | same disagreement, now only the 24 surviving facet documents |
| `dependency-undeclared` | — | 20 | checker anomaly, §5.3 |

Repo-wide, `ref-not-catalog-addressable` fell **2752 → 121**.

### 5.1 What the four remaining codes are not

None of them is a defect in this partition's documents. All 989 of
`mutation-leaf-id-grammar` + `mutation-aggregate-id-grammar` + `module-scope-id-inconsistent` are one
sentence: *"your ids are rooted at `s/stdio/<artifact>/<standard>/<subset>`, the module you belong to
says `s.stdio.<artifact>`."*

### 5.2 The grammar disagreement is now rooted in the non-mutation `$id`s (row 83), not the checker

W2c has done half of row 78. The checker no longer hard-codes the dotted form: it derives the expected
scope path **from the enclosing module's own `🧬️schema/🔣️.json` `$id`**, exactly as row 78 asked —

```
mutation-leaf-id-grammar
  $id "…/s/stdio/las/1.0/header/mutation/set-point/schema.json"
  must be "…/s.stdio.las/mutation/set-point/schema.json": a mutation leaf is its own scope…
module-scope-id-inconsistent
  must keep this module's scope path https://semio.tech/schema/s.stdio.html/ and vary only the facet filename
```

and the module `$id`s it reads are still the pre-migration artifact-level ones:

```
🌐️html 5 any        → https://semio.tech/schema/s.stdio.html/artifact.json
☁️las 1.0 header    → https://semio.tech/schema/s.stdio.las/artifact.json
📖️pdf 1.4 base      → https://semio.tech/schema/s.stdio.pdf/artifact.json
📖️pdf 1.7 base      → https://semio.tech/schema/s.stdio.pdf.1.7/artifact.json     ← not even self-consistent
```

**That is row 83's open item (W6c: stdio outside `🧬️mutations`), not a defect here.** The moment those
root modules declare the subset-level path — `…/s/stdio/las/1.0/header/artifact.json` — all 989
findings clear with **zero further edits in this partition**, because my ids are already
`<that scope path>/mutation/<semanticKind>/schema.json` and `<that scope path>/mutations.json`.

The alternative — moving my 1011 ids to the root's current form — is **impossible**, and this is the
hard constraint the coordinator needs:

```
PROOF-2 the checker's expected id is not injective over this partition
  915 leaves -> 766 distinct ids under the artifact-level dotted root;
  229 leaves share 80 ids
    11 x https://semio.tech/schema/s.stdio.semio/mutation/set-snapshot/schema.json
     7 x https://semio.tech/schema/s.stdio.step/mutation/set-file-schema/schema.json
     7 x https://semio.tech/schema/s.stdio.step/mutation/set-snapshot/schema.json
```

Dropping `<standard>/<subset>` merges `semio`'s eleven subsets, `step`'s seven, `ifc` 2x3 with 4,
`svg` 1.1's three, `pdf`'s five, `dwg` ac1018 with ac1024, `gif` 87a with 89a. The subset-level path is
the only one of the two that can carry 915 distinct contracts. Row 78 already decided this; the work
that remains is row 83's, in the root modules.

Coverage improvements to note: the checker now reaches **915/915 leaves** (was 491), 50 of 88 owning
aggregates via the new `mutation-aggregate-id-grammar`, and `module-scope-id-inconsistent` is no longer
double-counted on aggregates. `module-level-ineligible` is 0 here (row 80 landed).

### 5.3 `dependency-undeclared=20` — a checker anomaly, reported not chased

All 20 sit on **one** document, `🎞️gif 89a base`'s aggregate; repo-wide the code has 21 rows (the 21st
is a gis presence aggregate). Every one of the other 87 stdio owning aggregates now makes exactly the
same kind of cross-scope reference and raises nothing.

```
/oneOf/0/allOf/0/$ref crosses into scope s.stdio.gif.89a.base.mutation.add-app-extension,
which s.stdio.gif does not declare in dependsOn.
```

Either the rule is right and 87 aggregates are being skipped, or the rule fires on a catalog artefact
specific to gif 89a. `catalog-stale=1` in the same run makes the second more likely. Left for W2c
(§6.4); `dependsOn` is derived catalog metadata and is not mine to hand-edit.

## 6. Cross-partition requests

### 6.1 W6c stdio (`🚪️io`) — one orphaned codec facet document, with a dangling `$ref`

`🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🚪️io/🧬️mutations/📝️text/🔣️.json` was pulled into W8b's
"66" although its `🧬️mutations` parent is `🚪️io`, not `🧬️schema`. My `facet_documents()` no longer
reaches it, so it is now unowned. It carries

```json
"value":     {"$ref": "https://semio.tech/schema/s.stdio.gltf/mutation.json"},
"rejection": {"$ref": "https://semio.tech/schema/s.stdio.gltf/mutation.json#/$defs/rejection"}
```

and **no document in the repo declares `https://semio.tech/schema/s.stdio.gltf/mutation.json`** — two
dangling cross-document references. It also holds the `$id`
`…/s/stdio/gltf/2.0/any/mutations/text.json`, which W8b gave it and which now belongs to no schema
module. Decide whether the `🚪️io` mutation tree is a scope at all; if it is not, delete the document
with its `$id`.

### 6.2 W6c stdio (`📸️snapshot`) — svg's persistence-facet assert is now broken by contract §A

`🎨️svg/…/🧬️schema/📸️snapshot/🦀️.rs:1394-1419` `include_str!`s 20 sibling facet files and asserts

```rust
assert!(!facet.to_ascii_lowercase().contains("json"),
        "SVG diff/mutation persistence facet must describe the structured codec");
```

Measured over all 20 files: **19 contain no "json"; one does** —
`🧬️mutations/📝️text/🔣️.json`, and only because contract §A requires
`"$schema": "http://json-schema.org/draft-07/schema#"` and an `$id` ending in `.json`. At HEAD the
document had no `$schema` and an `$id` ending `…/mutations/text.svg`, so the assert passed; W8b's
`facets --write` made it fail. `🔺️diff/📝️text/🔣️.json` will fail the same way the moment row 83
reaches it.

The document is a keeper (`SvgMutationText`, the print_op text grammar) and its `$schema`/`$id` are
contract-mandated, so the assert is what has to change. It is in `📸️snapshot/`, outside my partition.
Exact change: drop `🔣️.json` from the two `include_str!` lists (a JSON-Schema document is JSON by
construction, so the substring test is vacuous there), or test the document's `description`/`type`
rather than its raw bytes. **I could not run `cargo` to observe the failure (brief: no stdio build);
this is derived from the file contents, which are quoted above and reproducible with
`tr 'A-Z' 'a-z' < <file> | grep -c json`.**

### 6.3 W6c stdio — root module `$id`s block 989 findings in this partition

Row 83, restated with the number it now costs: until
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/*/🏅️standards/*/🪆️subsets/*/🧬️schema/🔣️.json` declares
`https://semio.tech/schema/s/stdio/<artifact>/<standard>/<subset>/artifact.json`, this partition reads
989 findings that are not its own (§5.2). The artifact-level form cannot be adopted here — it collides
229 of 915 leaves onto 80 ids. Note `pdf` 1.7 already deviates from its own siblings
(`s.stdio.pdf.1.7` vs `s.stdio.pdf`), so the current root ids are not a consistent scheme either.

### 6.4 W2c tooling — `dependency-undeclared` fires on 1 of 88 equivalent aggregates

§5.3. Also: `catalog-stale=1` — the derived `🔣️schema-catalog.json` has not been regenerated since this
wave, and it hashes the 38 deleted facet documents.

### 6.5 Codec/parity owner — the `🔗️.graphql` / `🛰️.proto` twins of the deleted 38 are still scaffolding

Every facet dir that carried a `🔣️.json` also carries a `🔗️.graphql` and a `🛰️.proto` (66/66/66 across
128 facet dirs). For the D1/D2/D3 dirs these are the same scaffold in another format, e.g. las:

```graphql
# stdio.las mutations text grammar schema
scalar Bytes
type Document { schema: String! payload: String! }
```
```proto
syntax = "proto3"; package semio.stdio_las.mutations_text;
message Artifact { string schema = 1; bytes payload = 2; }
```

Row 81 dispatched only the JSON documents, so I deleted only those. **I did not delete the graphql/proto
twins on purpose**: svg's assert (§6.2) `include_str!`s `🧬️mutations/📝️text/{🔗️.graphql,🛰️.proto}`, so
they are load-bearing for at least one crate and removing them is a compile-breaking change that needs
the same owner as §6.2. Contract §B ("an export with no consumer and no fixture binding is dead") points
at deleting the other 60 pairs; that is a second dispatch, not this one.

### 6.6 W8d build wave — rows 46/49/82 untouched here

`📓️wp4b-stdio-rust-changes.md` remains the work order and nothing in this wave changed its inputs: the
915 leaf payload schemas were not edited (`audit` still `disagreeing=0 unprojectable=0`), and the 14
validator failures are still exactly the seven fixtures rows 46/47a/49 will re-case.

## 7. Open questions

- **Who moves first on the stdio scope path — the roots or nothing?** §5.2/§6.3. This partition is
  already on row 78's form and cannot move to the other one (229-leaf collision). Until row 83 lands,
  989 findings stand that no edit here can clear.
- **Is the `🚪️io/🧬️mutations` tree a schema scope?** §6.1. It has an `$id`, a codec facet document and
  two `$ref`s to an id nothing declares. Today it is in no worker's partition.
- **Do the 60 remaining scaffold `🔗️.graphql`/`🛰️.proto` pairs earn their place?** §6.5. Deleting the
  JSON half and leaving the other two formats is the least defensible resting state of this wave, and
  it is only defensible for as long as row 81's scope stands.
- **`semio/🌊️flow`'s `📝️text` document is kept but is a structural mirror, not a text grammar.** It
  describes an object with a `mutation` enum and six helper shapes, i.e. the aggregate again, not the
  encoding. It survived the D3 test only because it adds `definitions` beyond the kind list. If the
  rule is "a codec facet describes the wire", it should be rewritten as a grammar or deleted; I kept it
  rather than destroy 3 KB of hand-written structure on my own judgement.
