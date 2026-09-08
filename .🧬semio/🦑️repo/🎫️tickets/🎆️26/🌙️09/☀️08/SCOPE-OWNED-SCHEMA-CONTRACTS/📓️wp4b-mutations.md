# WP4b/c — non-stdio mutation `$id` grammar, absolute aggregate refs, kind-list removal

Partition: every `🧬️schema/🧬️mutations/**` subtree repo-wide **except** `✏️s/🔌️plugins/🗄️stdio/**` (W8's) and
`.🧬semio/…/🎫️tickets/**` (frozen captures). Continues `📓️wp4-mutations.md` (W7) and the unreported W7b pass
that a process restart interrupted.

Scripts (inputs, in this folder): `wp4-mutation-aggregates.py`, `wp4-mutation-validate.mjs`,
`wp4b-draft07-equivalence.mjs`. Raw output: `🗑️generated/wp4c-*.json`, `🗑️generated/schema-check-w7c.jsonl`.

## 0. What W7b had already landed (assessed, not re-done)

`git status` + a python census of the partition at session start:

```
leaf 🧬️schema/🔣️.json documents = 1766
  …/mutation/<kind>/schema.json  1766      …/mutation/<kind>.json  0
aggregates present = 79, all …/mutations.json ; mutation roots = 159 (80 without an aggregate)
framework/os mutation documents = 250: draft-07 122, no $schema 126 (descriptors/data), 2020-12 0
$ python3 <ticket>/wp4-mutation-aggregates.py --ids     # idempotence of W7b's pass
leaves=1766 leavesChanged=0 aggregates=79 aggregatesChanged=0 facets=54 facetsChanged=0 collisions=0
$ bun <ticket>/wp4-mutation-validate.mjs
roots=159 leaves=1766 aggregates=79 ajvCompiled=79 ; plugins problems=0 ; framework/os problems=0
```

So W7b had run `--ids --apply` (row 10, in a **path-derived** grammar — see §1) and `--draft07 --apply`
(task 4). `🗑️generated/wp4b-{ids,draft07}-*.json` are its records: 1766+79+54 ids stamped, 100 documents
migrated off 2020-12 with 22 `$ref`s lifted into `allOf` and 100 `unevaluatedProperties:false` closures
re-expressed, 0 refusals. **Rows 79 and 66 were untouched**, and every aggregate branch was still a
filesystem-relative path.

## 1. Row 10 — the grammar was path-derived; it is now module-`$id`-derived

W7b derived the scope path from the **directory path** (`s/writer/writer/1/any/…`). Contract §A says a scope
id comes from the module `$id` ("Never derive ids by stripping emoji from paths"), and row 78 spells the leaf
rule out: `<root module $id scope path>/mutation/<semanticKind>/schema.json`, read from the root
`🧬️schema/🔣️.json`. Measured, the two disagree for **every** root in the partition:

| root | module `$id` scope path | W7b path-derived |
|---|---|---|
| `✒️writer/…/✳️any/🧬️schema/🧬️mutations` | `s/writer/writer` | `s/writer/writer/1/any` |
| `➗️mathematical/🗿️artifacts/➗️equation/…` | `s/equation/equation` | `s/mathematical/equation/1/any` |
| `🌍️gis/…/✏️editor/🎚️config/…` | `app/gis/gis2d` | `s/gis/gismap/1/any/editor/config` |

`module_scope_path()` (new) reads the one declaration; `reidentify()` now keys on it. The live checker agrees:
`mutation-leaf-id-grammar` prints `must be https://semio.tech/schema/app/writer/writer/mutation/…`.

### 1.1 Injectivity proof — established before any write

`reidentify()` builds the whole `$id → [document]` multimap for leaves, aggregates and the `📝️text`/`💾️binary`
facet documents **first**, refuses every id claimed more than once together with all its claimants, and only
then stamps. A collision therefore cannot be written and discovered afterwards.

```
$ python3 <ticket>/wp4-mutation-aggregates.py --ids                      # first dry run
leaves=1766 leavesChanged=1590 aggregates=79 aggregatesChanged=60 facets=43 facetsChanged=43
collisions=2 unowned=191 duplicateTargets=0
  https://semio.tech/schema/app/gis/gis2d/mutation/set-camera/schema.json is claimed by 2 documents:
    …/🗺️gismap/…/✏️editor/🎚️config/🧬️schema/🧬️mutations/🎥️set-camera/🧬️schema/🔣️.json,
    …/🗺️gismap/…/✏️editor/👥️presence/🧬️schema/🧬️mutations/🎥️set-camera/🧬️schema/🔣️.json
  https://semio.tech/schema/app/gis/gis2d/mutations.json is claimed by 2 documents: (the same two roots)
```

Both collisions had one cause: `✏️editor/🎚️config` and `✏️editor/👥️presence` declared the **same** module `$id`,
although contract §A makes a surface its own scope. A peer (W6b) split those scopes mid-session
(`…/remodeling/config.json` → `…/remodeling/config/schema.json`); after re-running against the split tree:

```
$ python3 <ticket>/wp4-mutation-aggregates.py --ids --apply
leaves=1766 leavesChanged=109 aggregates=79 aggregatesChanged=0 facets=43 facetsChanged=0
collisions=0 unowned=191 duplicateTargets=0
$ python3 <ticket>/wp4-mutation-aggregates.py --ids                      # idempotence
leaves=1766 leavesChanged=0 aggregates=79 aggregatesChanged=0 facets=43 facetsChanged=0
collisions=0 unowned=191 duplicateTargets=0
```

**1766 leaves → 1766 distinct `$id`s → 0 collisions, 0 duplicate targets**, confirmed independently by
`wp4-mutation-validate.mjs`'s repo-wide `$id` uniqueness assertion (§5) and by `schema check`
(`document-id-duplicate` = 0 in the partition).

### 1.2 The 174 leaves that keep a path-derived `$id`, and why they are not guessed at

`unowned=191` documents (174 leaves + 17 aggregates) sit under a module that declares **no**
`🧬️schema/🔣️.json` at all, so there is no scope path to inherit and none is invented. 24 plugin subset
modules (`🗒️note` ×8 subsets, `🏗️fem` ×10, `➗️mathematical` ×3, `🖍️draw` ×5, `🎬️sequence` ×2,
`🌀️procedural/🧩️assembly`, `📐️cad`/`📖️playbook` extensions) have a `🧬️schema/` directory whose only child is
`🧬️mutations/`. Authoring those module documents is outside this partition → request §7.1.

Final provenance census:

| | count |
|---|---:|
| leaves, `$id` shape `…/mutation/<semanticKind>/schema.json` | **1766 / 1766** |
| of those, derived from the root module `$id` | 1592 |
| of those, kept path-derived because the module declares no `$id` | 174 |
| duplicate leaf `$id`s | **0** |
| aggregates, `$id` shape `…/mutations.json` | **79 / 79** |

## 2. Row 79 — aggregate branches now name the leaf `$id`

New subcommand `--absref [--apply] [--remap <ids.json>]`. It rewrites every filesystem-relative `$ref` in a
partition mutation document to the target document's `$id` (JSON pointer carried over untouched), refuses a
target that declares no `$id`, and — with `--remap` — re-points absolute refs through an `$id` mapping so a
grammar change and the refs that name it land together.

```
$ python3 <ticket>/wp4-mutation-aggregates.py --absref --apply
documents=1900 refs=1645 rewritten=1644 aggregates=78 leaves=21 refused=1
  🏪️store/👥️presence/♻️retirement/🧪️fixtures/🧬️mutations/🔢️set-value/…: ../../../📐️schema/🔣️.json#/$defs/I32
  → 🏪️store/👥️presence/♻️retirement/🧪️fixtures/📐️schema/🔣️.json declares no $id
```

The single refusal was resolved by W5b during the session (they deleted that helper module and pointed the
ref at `https://semio.tech/schema/os/store/presence/component.json#/$defs/I32`), so the partition now holds
**0 relative `$ref`s**:

```
aggregate branch $ref forms: {"absolute": 1618}      # 1618 branches over 79 aggregates
```

`apply_aggregate()` was changed in the same place row 79 predicted (one line): it emits
`{"$ref": leaf_schema_id(scope, semanticKind)}` instead of `./<leaf>/🧬️schema/🔣️.json`.

### 2.1 Row 66 — the three `📸️remodel` editor leaves

They were part of the same sweep, no special case: `../../../🔣️.json` → the target module's `$id`.

```
📸️remodel/…/✏️editor/🎚️config/…/🎥️set-camera   ../../../🔣️.json#/properties/camera
  → https://semio.tech/schema/app/remodel/remodeling/config/schema.json#/properties/camera
📸️remodel/…/✏️editor/🎚️config/…/📸️replace-config → …/config/schema.json
📸️remodel/…/✏️editor/👥️presence/…/👥️replace-presence → …/presence/schema.json
```

The pointer is still `#/properties/camera`, not `#/$defs/<ExportId>`, because the remodel config module
declares an empty `$defs` — it exports nothing to address. That is the one remaining
`ref-not-catalog-addressable` in the partition → request §7.2.

### 2.2 Validator: one Ajv keyed by `$id`, as row 79 requires

`wp4-mutation-validate.mjs` was rewritten. It no longer resolves branches on the filesystem: it indexes every
`$id` in `✏️s` + `🧰️framework`, and for each aggregate **and each leaf** registers the transitive `$ref`
closure in one Ajv instance keyed by `$id`, then compiles. It additionally asserts that every aggregate branch
`$ref` is absolute (a path is reported as a row-79 breach) and that the named `$id` exists on disk.

## 3. Row 23 — `x-semio-mutationKinds` dropped (precondition verified, not assumed)

The brief said to wait for the tooling worker's structural aggregate check. It has landed, in two places:

- `📜️script.ts:23689 policyMutationAggregateMembers` — resolves each branch `$ref` against the direct leaves
  *by relative path or by the payload document's `$id`*, and its own docstring says this "replaces reading the
  leaf's semantic kind out of the aggregate's `x-semio-mutationKinds` string list, which no consumer reads";
  `rootHasIdentity` for `surface.id === "json-schema"` now reads `aggregate.referenced`, not text.
- `📚️library/🔍️discovery/🟦️.ts:3069` emits `mutation-aggregate-kinds-redundant` for the key.

W7's stated reason for keeping the key (a relative `$ref` path does not contain the semantic kind for nested
leaves such as architect's `ℹ️information-requirement/🌱️create` → `create-information-requirement`) is void
under row 79: every branch now literally reads `…/mutation/create-information-requirement/schema.json`, so the
textual fallbacks still find the kind.

`--dropkinds` refuses any aggregate whose declared kinds are not exactly what its own branches name:

```
$ python3 <ticket>/wp4-mutation-aggregates.py --dropkinds --apply
aggregates=59 dropped=59 refused=0
```

`apply_aggregate()` no longer writes the key. Repo-wide sweep: the only remaining occurrences are
`📜️script.ts`'s docstring, `📚️library/🔍️discovery/🟦️.ts`'s diagnostic, and W2c's own negative fixture
`📚️library/🧪️tests/🧬️schema-scope-catalog/🧫️fixtures/🔣️.json` — all correct. `schema check`:
`mutation-aggregate-kinds-redundant` **59 → 0**.

## 4. Row 7 — the 61 "undeclared leaves" are not leaves

`bun 🧪️test/📜️script.ts manifest payload-schema` reports
`61 × the leaf carries no 🔣️.json descriptor`, all under `🏛️architect/🗿️artifacts/🏛️program/…`. **They are
grouping directories of a two-segment leaf layout, and authoring descriptors for them would invent 47
mutation kinds no Rust enum declares.** Measured:

```
🏛️program/…/✳️any/🧬️schema/🧬️mutations : 71 directories
   2 facet dirs (📝️text, 💾️binary)
  69 domain dirs, 0 of which carry 🔣️.json or 🦀️.rs
     → each contains ONLY verb subdirectories, 266 of them, ALL with a descriptor. anomalies = 0
  47 of the 69 match the taxonomy mutationDirectoryPattern (a hyphen + VS16) and are what the harness counts
```

The harness's `resolvePayloadSchemas` (`🧪️test/📦️packages/🟦️typescript/🟦️.ts:5044`) reads **one** level of
`<owner>/🧬️schema/🧬️mutations` and filters by `isMutationLeafDirectory`, so it both mis-reports the 47 domain
directories as undeclared leaves *and* never sees the 266 real ones. Repo-wide, exactly 50 first-level
pattern-matching directories carry no descriptor (47 architect + 3 framework/os); the rest of the reported 61
are gltf's identically-shaped `<domain>/<verb>` directories in stdio.

Contract §B already settles the shape ("A two-segment leaf directory … is a leaf"), and the schema checker
already implements it (`mutationRootModule` accepts `depth` 1 **or** 2 — `module-level-ineligible` in this
partition went 266 → 0). So row 7 is a **harness bug**, not authoring work → request §7.4.

Of the 3 non-architect ones, two were real defects and are fixed here: `🔌️plugin/🧪️tests/📢️publication-fixtures/{👥️presence,🫧️transient}/🧬️mutations/📈️advance-publication-{presence,transient}` were
**empty orphan directories** (an empty `🧬️schema/`, no descriptor, no Rust, 0 tracked files) for a variant the
enum does not have — `PublicationPresenceMutation` has exactly one variant, `ChangePublicationPresence`.
Deleted (contract §B: a declaration with no consumer is dead and goes with its data). The third,
`📡️replication/🔗️causal/🧪️fixtures/🧬️mutations/➕️causal-add`, uses a `🛂️schema`/`🧪️descriptor` layout of its
own inside a fixture tree → §7.3.

## 5. Task 4 — the framework/os leaves

All 101 framework/os leaves (W7 counted 96 already-canonical + 5 relocated) are draft-07 with a grammar `$id`
and a `title` equal to the descriptor's `aggregateVariant`:

```
framework/os leaves = 101 : {"draft-07": 101}      # no no-$id, no bad-$id-grammar, no title mismatch
documents still using unevaluated* in the partition = 0
documents still naming 2020-12 = 0                 # 3 stray nested `$schema` keys pruned, §5.2
```

### 5.1 Equivalence evidence for the 2020-12 → draft-07 migration

`wp4b-draft07-equivalence.mjs` compiles the pre-migration document with `ajv/dist/2020` and the migrated one
with draft-07 `ajv`, over every subset of the property names each schema mentions, each also with an alien key
added, plus the non-object instances — the corpus that probes exactly the `unevaluatedProperties` closure the
migration had to re-express.

```
$ bun <ticket>/wp4b-draft07-equivalence.mjs            # at session start, before my $id work
documents compared=95 instances=7668 disagreements=0
accepted by both=87 rejected by both=7581; closure actually exercised=86
could not compare 5   (all 🌊️flow/🌿️vcs: the 2020-12 side's own `https://semio.dev/…` refs never resolved)

$ bun <ticket>/wp4b-draft07-equivalence.mjs            # after §1/§2
documents compared=81 instances=7476 disagreements=0
accepted by both=74 rejected by both=7402; closure actually exercised=73
could not compare 19
```

**0 disagreements in both runs.** The uncomparable set grew from 5 to 19 only on the *before* side: peers
deleted `🏪️store/…/📐️schema/🔣️.json` and PascalCased `🌊️flow/🌿️vcs`'s exports during the session, which
invalidates refs in the frozen pre-migration snapshot. The after side compiles in full (§6).

### 5.2 Two repairs the migration had left behind

- **Stray nested `$schema`.** `--stray --apply` removed 3 `$schema` keys declared on *sub*schemas in
  `🔱️trinity/🔌️jack`'s `➕️create-node` and `🌉️create-edge`. Draft-07 reads `$schema` only at the document root,
  so these changed no verdict — they made the documents read as half-migrated to anything that greps the
  dialect string.
- **Peer export re-casing.** `🌊️flow/🌿️vcs`'s module renamed `$defs.{index,widget,synapse,fixture,layoutEntry}`
  to PascalCase mid-session, silently invalidating 10 cross-document pointers in my leaves. `--absref` gained
  `repoint()`, which repairs a `#/$defs/<key>` pointer **only** when exactly one key of the target differs by
  case, and reports anything else. 10 repaired, 0 refused.

### 5.3 Export ids inside the partition (`export-id-invalid` 43 → 0)

62 leaf and aggregate documents declared lowercase `$defs` keys (`payload` ×60, plus `fingerprint`/`output`
and `hover`/`selection`), which contract §A cannot address as exports — the checker reported them twice, as
`export-id-invalid` on the leaf and `ref-unresolved` on the aggregate branch that named them.

```
$ python3 <ticket>/wp4-mutation-aggregates.py --pascal --apply
documents=89 exports=65 pointers=66 refused=0 files=62
$ python3 <ticket>/wp4-mutation-aggregates.py --absref --apply     # cross-document pointers
repointed=51 refused=0
```

Only the leading character moves; a key that would not be PascalCase afterwards, or that would collide with an
existing export, is refused. No consumer outside the mutation trees named these keys (repo-wide grep for
`$defs/payload|hover|selection|fingerprint|output` outside `🧬️mutations`: no hits).

## 6. Verification

### 6.1 Own validator + ajv oracle (final state)

```
$ bun <ticket>/wp4-mutation-validate.mjs
roots=159 leaves=1766 aggregates=79 branches=1618 ajvCompiled=79 leavesCompiledStandalone=1766
WP4 partition (✏️s/🔌️plugins, non-stdio) problems=0
framework/os mutation trees problems=0
```

Per leaf: `payloadSchema` is the taxonomy default and exists; parses; draft-07; `$id` and `title` present;
`title == aggregateVariant`; `$id` unique repo-wide; **and the leaf compiles standalone** with its whole
cross-document closure registered by `$id`. Per aggregate: draft-07; every branch names ≥1 leaf **by absolute
`$id`**; that `$id` exists; and ajv (draft-07) compiles the union with the closure registered by `$id`.
1618/1618 branches absolute, 79/79 aggregates compiled, 1766/1766 leaves compiled.

Independent implementation, per the repo rule: `ajv` 8.x draft-07 is the third-party engine doing the
compiling here, and `📕️norm`'s separately written ajv harness re-validates the same contract with hostile
payloads (§6.2).

### 6.2 Plugin-owned oracle — `📕️norm` `config-mutation-source`

Row 79 changed what this oracle asserts, so it was updated with the change (it is the direct consumer of the
aggregate shape):

```diff
-    ajv.addVocabulary(["x-semio-mutationKinds"]);
-    const payloadRef = aggregate.oneOf[0].$ref;
-    if (decodeURI(payloadRef) !== "./☑️change-selected-check-index/🧬️schema/🔣️.json") throw …
-    const validateMutation = ajv.compile({ ...aggregate, oneOf: [{ $ref: schema.$id }] });
+    if (aggregate.oneOf[0].$ref !== schema.$id) throw new Error("config aggregate schema does not reference its owned payload by $id");
+    const validateMutation = ajv.compile(aggregate);
```

The `oneOf: [{ $ref: schema.$id }]` substitution existed **only** because ajv resolves a relative `$ref`
against the document `$id` and never against the file path — W7's §4.4 called that "the one real ergonomic
cost of relative-path unions". With row 79 the aggregate compiles as written.

```
$ bun ./📜️script.ts config-mutation-source        # cwd ✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust
Norm config schema oracle passed: 5 cases, 5 hostile payloads, 4 undeclared wire forms, 13 text vectors, 25 binary vectors
```

### 6.3 `schema check` — partition rows, before and after

Before = `🗑️generated/schema-check-w6c-baseline.jsonl` (19:35, i.e. after W7b, before this pass).
After = `bun ./📜️script.ts schema check --report <ticket>/🗑️generated/schema-check-w7c.jsonl`,
filtered to `🧬️mutations` × non-stdio × non-ticket (176 of 8235 rows).

| code | before | after | why |
|---|---:|---:|---|
| `ref-not-catalog-addressable` | 1595 | **1** | §2 — the last one has no export to address (§7.2) |
| `mutation-leaf-id-grammar` | 1272 | **131** | §1 — the remainder are the modules with no `$id` (§7.1) |
| `module-level-ineligible` | 266 | **0** | the checker now admits two-segment leaves (row 80 landed) |
| `mutation-aggregate-kinds-redundant` | 59 | **0** | §3 |
| `export-id-invalid` | 43 | **0** | §5.3 |
| `ref-unresolved` | 33 | **1** | §5.3; the last one is the misnamed energy leaf (§7.3) |
| `module-scope-id-inconsistent` | 27 | **0** | §1 (aggregates now inherit the module scope path) |
| `mutation-aggregate-id-grammar` | 0 | **0** | was 2 mid-pass, cleared when W6b split the surface scopes |
| `fixture-defines-schema` | 0 | 43 | new rule (row 42) landed during the session; §7.3 |
| **partition total** | **3295** | **176** | |

Repo-wide the same run reports `modules=3206 scopes=3070 findings=8235`.

### 6.4 Idempotence

Every subcommand re-run after `--apply` reports zero changes: `--ids` (0/0/0, collisions 0), `--absref`
(edits 0), `--pascal` (exports 0), `--stray` (keys 0), and `--dropkinds` is a no-op because the key is gone.

## 7. Cross-partition requests

1. **W6b plugins — 24 subset/extension modules declare no `🧬️schema/🔣️.json`, so 174 leaves have no scope
   path to inherit** (131 `mutation-leaf-id-grammar` rows). Each needs a module root document with an `$id`
   in the contract §A grammar; the mutation leaves then re-derive with one command
   (`wp4-mutation-aggregates.py --ids --apply`). The modules:
   `🗒️note/🗿️artifacts/🗒️note/…/🪆️subsets/{🧱️block,🎨️canvas,📊️table,🖋️ink,🖼️asset,📜️document,📝️text,🧮️math}/🧬️schema`,
   `🏗️fem/🗿️artifacts/{◻️2d,🧊️3d}/…/🪆️subsets/{🕸️mesh,🏋️load,🛡️boundary,🧱️material,📈️analysis}/🧬️schema`,
   `➗️mathematical/…/🪆️subsets/{🕸️graph,📐️geometry,➗️equation}/🧬️schema`,
   `🖍️draw/…/🪆️subsets/{🎨️style,🧱️structure,🏷️metadata,🔀️transform}/🧬️schema`,
   `🎬️sequence/…/🪆️subsets/{🪜️step,🔗️dependency}/🧬️schema`,
   `🌀️procedural/🗿️artifacts/🧩️assembly/…/✳️any/🧬️schema`,
   `📐️cad/🧩️extensions/🏢️aec-building/🧬️schema`, `📖️playbook/🧩️extensions/🌀️procedural/🧬️schema`.
2. **W6b plugins — `📸️remodel`'s two surface modules export nothing.**
   `…/✏️editor/🎚️config/🧬️schema/🔣️.json` (`$id …/app/remodel/remodeling/config/schema.json`) and its
   `👥️presence` sibling declare `$defs: {}`, so the three mutation leaves can only address
   `#/properties/camera` / the document root. Add `$defs.RemodelingConfig`, `$defs.RemodelingPresence` and
   `$defs.Camera`; I will re-point the three refs in one `--absref` run. (Same class as the surface-scope split
   you already did — thank you, it cleared both of my `$id` collisions.)
3. **`🔋️energy` owner — a mutation leaf directory is named with the fixtures emoji.**
   `🔋️energy/🗿️artifacts/🔋️model/…/✳️any/🧬️schema/🧬️mutations/🧫️change-plant-loop-return-temperature` (added by
   a peer mid-run) uses `🧫️` as its descriptor `emoji`, and `schemaScopeOwnerExcluded` therefore treats the
   whole leaf as a fixture collection: it is the partition's last `ref-unresolved` (its own aggregate branch
   "names no catalogued document") and one of the `fixture-defines-schema` rows. Rename the directory and the
   descriptor `emoji` to a non-`🧫️`/`🧪️` emoji; the call sites are the descriptor `owner`, the aggregate
   `🦀️.rs` `#[path]`, and `…/✳️any/🔮️oracle/🔣️.json:2913-2914`
   (`sourceMutationDirectoryName`, `mutationDirectoryName`). I did not do it because two of the four are
   outside this partition.
4. **W2c tooling / W1b harness — `resolvePayloadSchemas` cannot see a two-segment leaf.**
   `🧪️test/📦️packages/🟦️typescript/🟦️.ts:5044` reads one directory level, so `manifest payload-schema` reports
   47 architect **domain** directories as leaves without a payload contract and never reaches the 266 real
   leaves beneath them (row 7 is entirely this bug). `schema check`'s `mutationRootModule` already accepts
   `depth` 1 or 2 — mirror it. Same call shape affects `readLeafDescriptors` (`:3616`) and
   `policyMutationAggregateMembers` (root `📜️script.ts:23689`), whose `leafNames` are direct children only, so
   an architect aggregate's branches will read as "not a direct leaf payload schema" once that check is run
   over this partition.
5. **W5b os — 42 `fixture-defines-schema` rows are os mutation vocabularies living inside fixture trees.**
   `🏪️store/🧫️fixtures/{🚦️severity,🧮️demo,⏱️timestamped,🛂️validated,🪤️lossy}/🧬️mutations/*`,
   `🏪️store/👥️presence/♻️retirement/🧪️fixtures/🧬️mutations/*`,
   `📡️spr/{🎮️command,🧪️testkit}/🧪️tests/🧬️mutation-laws/*`,
   `🔌️plugin/🧪️tests/{🧬️mutation-fixtures/🔀️transaction,🖥️test-app-mutations/🧬️document,📡️contributed-mutation-wire}/*`,
   `🔌️plugin/🏗️builder/🧪️tests/🔗️dependency-contribution/*`. Per row 42 each needs either an
   `inertSchemaData` declaration on the enclosing case or a move into the owning module. They are conformance
   vocabularies, not parser inputs, so the honest answer is probably the move — an os module decision, not mine.
   The same set is why 17 aggregates and 174 leaves have no module scope path (§1.2) on the framework side.
6. **Coordinator — record that row 10's grammar is only as stable as the module `$id`s it reads.** Deriving the
   leaf id from the module `$id` (rather than from a second path heuristic) is right, but it means every module
   `$id` a wave-2 worker fixes moves its leaves too. That happened twice in this session (W6b's surface split,
   W5b's export re-casing) and cost two extra `--ids --apply` + `--absref --remap` cycles. It is one command,
   but the last partition to touch a module `$id` must re-run it, or the aggregate refs go stale.

## 8. Open questions

1. **`app/…` scope paths are still in the tree.** 1592 leaf ids now inherit their module's declared scope path,
   and for 13 artifact apps that path is still `app/<plugin>/<artifact>` rather than `s/…`. If that is legacy,
   the leaves move again when it is fixed (§7.6); if it is deliberate (surfaces are "app" scopes), say so and
   it is settled.
2. **Is `#/properties/<x>` ever a legal cross-document pointer?** Contract §A says cross-document references
   address an export (`#/$defs/<ExportId>`). Three remodel leaves and several framework/os leaves point into
   `#/properties/…` of a module that declares no `$defs`. Either those modules gain exports (§7.2) or the
   contract admits property pointers; today the check says the former.
3. **The 80 mutation roots with no aggregate** (W7's open question 1) are unchanged and still coupled to a
   `requiredLanguageSurfaces` decision per root. Creating the file flips `rootExists` to true, which then
   breaches every leaf in the root that does not declare `json-schema`.
