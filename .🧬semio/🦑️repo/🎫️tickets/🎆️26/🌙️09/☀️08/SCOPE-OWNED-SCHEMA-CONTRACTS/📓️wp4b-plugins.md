# WP4b — plugins (`✏️s/**`, everything outside `🧬️schema/🧬️mutations/**`)

Wave-2 pass over the same partition as `📓️wp4-plugins.md`. Assigned rows of
`📋️cross-partition-requests.md`: **55** (the eleven same-pattern files + the `🧪️fixtures` rename +
`PlaybookViewCommandLimits` + the puzzle wasm check), **22** (`✏️editor/*/🦀️.rs` `payload_schema`
literals), **39** (retained-command lane/disposition/execution vocabulary), **67** (the sequence
editor's wasm surface descriptor), **83** (stdio non-mutation `$id`s), plus contract §B's
`x-semio-formats` restricted-support rule.

A predecessor (W6b) was stopped by a process restart and left **no** durable edit: the row-55 files
were all still on disk, `payload_schema: "🔣️.schema.json"` still had 177 occurrences, no
`x-semio-formats` existed anywhere, and no `📓️wp4b-plugins.md` had been written. This pass started
from that measured state, not from a handover.

Helpers written for this pass (kept in the ticket folder):
`wp4b-retained-vocabulary.py`, `wp4b-plugin-schema-ids.py`, `wp4b-law-export-formats.py`,
`wp4b-fixture-dir-name.py`; `wp4-move.py` was extended (`definitions` hoisting, `--description`,
`--formats`, and a **fix**: it did not rewrite `#/$defs/<helper>` references *inside* the hoisted
`$defs` it renamed, only inside the moved document — that bug had to be repaired by hand in the gis,
stdio and sequence modules before their oracles compiled).

---

## 1. Row 55 — the eleven same-pattern files

`schema check`'s `placement-retired-location` named exactly eleven files in this partition (the
report's list plus `🪐️space/…/📬️apply-directory-event-page/🧬️receipt/🧬️.schema.json`, minus
`🗄️stdio/📇️registry/🧬️schema/📜️native-codec-factories.json`, which is data and matches no
`*.schema.json` pattern). All twelve are handled.

| Source (deleted / moved) | Owner scope | Export | Module |
|---|---|---|---|
| `🖨️raster/🔏️publication-authority/🧬️.schema.json` | `s.raster` | `RasterPublicationAuthority` | **new** `🖨️raster/🧬️schema/🔣️.json` |
| `🧩️puzzle/🔏️publication-authority/🧬️.schema.json` | `s.puzzle` | `PuzzlePublicationAuthority` | **new** `🧩️puzzle/🧬️schema/🔣️.json` |
| `➗️mathematical/📣️publication-authority/🧬️.schema.json` | `s.mathematical` | `MathematicalPublicationAuthority` | **new** `➗️mathematical/🧬️schema/🔣️.json` |
| `🌿️vcs/📇️native-codecs/🧬️.schema.json` | `s.vcs` | `VcsNativeCodecs` (+`…Receipt`, `…Field`) | existing `🌿️vcs/🧬️schema/🔣️.json` |
| `🌍️gis/📇️native-codecs/🧬️.schema.json` | `s.gis` | `GisNativeCodecs` (+`…Receipt`, `…Field`) | existing `🌍️gis/🧬️schema/🔣️.json` |
| `📕️norm/📇️registry/🧬️contract/🧬️schema/🧬️.schema.json` | `s.norm` | `NormArtifactPackageDefinition` | existing `📕️norm/🧬️schema/🔣️.json` |
| `📕️norm/📦️packages/🦀️rust/🧬️mutation-leaf-taxonomy-v1.schema.json` | `s.norm` | `NormMutationLeafTaxonomy` | same |
| `🗄️stdio/🧬️schema/📦️artifact-package/🧬️.schema.json` | `s.stdio` | `StdioArtifactPackage` (+4 hoisted) | existing `🗄️stdio/🧬️schema/🔣️.json` |
| `📐️cad/…/✳️any/🗄️retained-jobs/🧬️.schema.json` | `s.cad.cad` | `CadRetainedJobs` | subset module |
| `📐️cad/…/✏️editor/👥️presence/🧬️schema/🧬️.schema.json` | `app.cad.cad.presence` | `CadPresenceRetirementLaws` | the presence module's own `🔣️.json` |
| `🪐️space/…/📬️apply-directory-event-page/🧬️receipt/🧬️.schema.json` | `app.space.home.config` | `HomeDirectoryProjectionReceipt` | the Home config surface module |
| `🗄️stdio/📇️registry/🧬️schema/📜️native-codec-factories.json` | — (data, not a contract) | — | moved **out** of the module slot to `📇️registry/📜️native-codec-factories.json` |

`📕️norm/📇️registry/🧬️contract/🧬️schema/` and `🗄️stdio/🧬️schema/📦️artifact-package/` are removed
(empty). Two decisions worth naming:

- **`➗️mathematical` gets a plugin-root scope `s.mathematical`, not `s.equation.equation`.** The
  publication-authority census sits at the plugin root and describes the app, matching the
  `s.block` / `s.draw` precedent WP4 set for the same family; the equation *subset* module keeps the
  per-artifact laws (`EquationRetainedCommandLaw`, `EquationSceneOwnerLaw`).
- **`📜️native-codec-factories.json` is data.** Contract §B allows only the canonical five files and
  facet directories inside a `🧬️schema/` module, so a 295-line receipt projection cannot live there.
  It moved one directory up; `📇️registry/🦀️.rs:266`'s `include_str!` follows it. Three hub readers
  and two hub fixtures still name the old path — cross-partition request **A** below.

### Consumers rewired (all inside this partition)

| File | Change |
|---|---|
| `🧩️puzzle/📦️packages/🟦️typescript/📜️script.ts` | fixture-local schema read → `addSchema(s.puzzle module)` + `$ref …#/$defs/PuzzlePublicationAuthority` |
| `➗️mathematical/📦️packages/🟦️typescript/📜️script.ts` | → `s.mathematical#/$defs/MathematicalPublicationAuthority` |
| `🌍️gis/📦️packages/🦀️rust/📜️script.ts` | `proveGisNativeCodecReceipts` uses `compileGisScopeExport(…, "GisNativeCodecs")`; `Ajv2020` dropped |
| `🌿️vcs/📦️packages/🦀️rust/📜️script.ts` | new `compileVcsScopeExport(repoRoot, exportId)` helper; `Ajv2020` dropped |
| `🌍️gis` + `🌿️vcs` `📜️script.ts` | the deleted os fixture schema `🔌️plugin/🧪️fixtures/🌱️artifact-document-id-v1/🧬️.schema.json` → `os.plugin.registry` module `#/$defs/ArtifactDocumentIdV1` (both scripts were **already red** on that ENOENT before this pass) |
| `🗄️stdio/📜️script.ts` | `PACKAGE_SCHEMA_PATH` → `PACKAGE_SCHEMA_MODULE_PATH` + `$ref …#/$defs/StdioArtifactPackage`; `Ajv2020` → `Ajv` |
| `📕️norm/📦️packages/🦀️rust/📜️script.ts` | mutation-leaf taxonomy check reads `../../🧬️schema/🔣️.json` + `#/$defs/NormMutationLeafTaxonomy` |
| `🗄️stdio/📇️registry/🦀️.rs:266` | `include_str!("🧬️schema/📜️native-codec-factories.json")` → `include_str!("📜️native-codec-factories.json")` |

### `PlaybookViewCommandLimits`

No consumer anywhere in `✏️s`, `🧰️framework` or `🌎️hub`; its only mentions were the export itself,
the generated catalog and the stale taxonomy `memberNames` row. Per contract §B ("an export with no
consumer and no fixture binding is dead and is deleted together with its data") the export and
`📖️playbook/🗿️artifacts/📖️playbook/🧪️fixtures/👁️playbook-view-command-limits.json` are **deleted**.
The playbook subset module now exports `PlaybookStep`, `PlaybookBlock`, `PlaybookSceneOwnerLaw`.

### Hygiene grep — empty

```
$ git ls-files '✏️s' | grep -E 'schema\.json$' | grep -v '🧬️schema/🧬️mutations/' \
    | while read -r f; do [ -e "$f" ] && echo "STILL: $f"; done
(no output)
$ find '✏️s' -name '*.schema.json' -not -path '*/.venv/*' -not -path '*🧬️mutations*'
(no output)
```

### `🧪️fixtures` → `🧫️fixtures` (ledger row 27)

`testFixturesDirName` is `🧫️fixtures` and `testsDirName` is `🧪️tests`; `🧪️fixtures` is not a taxonomy
name. **96 directories renamed, 131 files under `✏️s` rewritten**, 0 collisions (no parent already had
a `🧫️fixtures` sibling), 0 `🧪️fixtures` directories left under `✏️s`. `wp4b-fixture-dir-name.py` is in
the ticket folder.

Two corrections were needed after the sweep and are worth recording:

1. **A blanket literal replace is wrong.** Five files under `✏️s` spell a path into *another*
   partition (`🧰️framework/…/🔌️plugin/🧪️fixtures/🌱️artifact-document-id-v1`), and those directories
   are still `🧪️fixtures`. Six occurrences in five files were reverted by checking each rewritten path
   against the filesystem; the check now reports 0 referenced-but-missing foreign fixture directories.
2. **`🌍️gis/…/🌉️component-cold-map-patch` had already moved.** A peer's test-layout sweep relocated it
   from `🧫️fixtures/` to `🧪️tests/` (it is a Cargo integration-test target, `📓️wp4-plugins.md` Family
   9), leaving `🌍️gis/📦️packages/🦀️rust/📜️script.ts:239` stale before this pass. Repointed to
   `✏️s/🔌️plugins/🌍️gis/🧪️tests/🌉️component-cold-map-patch`; the check passes again (§7.4).

Four files outside this partition still name an `✏️s/…/🧪️fixtures/…` path — request **F**.

---

## 2. Row 67 — the sequence editor's wasm surface

`🎬️sequence/…/✳️any/✏️editor/🌉️wasm/🧬️schema/` held three JSON documents. Per the settled decision a
wasm surface descriptor is an **interface facet**, normative format `📜️wit`:

- `🧬️schema/🔣️.json` (a self-describing document that was both the ABI schema *and* the ABI data) →
  **`🧬️schema/📜️.wit`**, `package semio:sequence-browser-abi@1.0.0`, `interface abi` with
  `enum feature` (10), `enum operation` (47, grouped by feature in source order), `enum event` (8),
  `record identity`, `record limits`, `record framing`, `type surface-handle`/`canvas-handle`, and
  `world sequence-browser { export abi; }`. The numeric opcodes are **not** restated: they live in
  the two producers the facet's doc comment names (`🖥️sequence-host.js`'s `SequenceOperation` and
  `📡️protocol.rs`), and the reader cross-checks the two.
- `🧬️schema/🧵️retained/🔣️.json` (a retained-command law, not a facet directory) → subset module
  `s.sequence.sequence` `$defs.SequenceRetainedActions` (+ `…Contract`, `…Route`).
- `🧬️schema/🧪️oracle/🔣️.json` → **deleted**. It is contract §B's "spec of one example"
  (`operation: {const: 2305}`, `semantic: {const: "step-101"}`) with no reader; the example data
  `🌉️wasm/🧪️fixtures/🔣️.json` stays.

Readers rewired: `🌉️wasm/🧪️tests/🧬️schema/🟨️.js` now parses the `.wit` enums and asserts them against
the JS host ledger; `🌉️wasm/📦️packages/🟨️javascript/🧪️tests/🟨️.js` compiles the subset-module export
(it had been pointing at `🧬️schema/🧪️retained-actions/🔣️.json`, a path that does not exist at HEAD).

```
$ node '…/🌉️wasm/🧪️tests/🧬️schema/🟨️.js'
{"facet":"📜️interface","features":10,"operations":47,"events":8,"framing":"A1-AbiMessage-v1"}
$ node '…/🌉️wasm/📦️packages/🟨️javascript/🧪️tests/🟨️.js'
{"oracle":"ajv-draft07+dagre-0.8.5+graphlib","routes":17,"migrated":17,"pending":0,"hostileLaws":7,
 "persistentScenarios":3,"maximumStepMicros":7500,"locales":["en","de"],
 "accessibility":"bounded-progress-cancel-close","customization":["orientation","locale","viewport"]}
```

**Open (cross-partition):** taxonomy `schemaFacetKinds.📜️interface.facetPathIdentities` lists only
the os plugin module, so `resolveSchemaFacetKind` still calls this module `🧬️data`. It produces no
finding today (the module has no JSON leaf left, so nothing is read), but the catalog will not record
the `📜️wit` format until the module path is added — request **E**.

---

## 3. Row 22 — `payload_schema` literals

`payload_schema: "🔣️.schema.json"` (177) and `payload_schema: "🧬️.schema.json"` (9) in **43**
`✏️editor/{🎚️config,👥️presence,🖌️session,…}/🦀️.rs` files → `payload_schema: "🧬️schema/🔣️.json"`,
**186 occurrences**. That is the taxonomy location and the value 3,341 on-disk descriptors already
carry; `validate_mutation_leaf_descriptor` (`📡️replication/🎮️mutation/🦀️.rs:405`) only requires the
field to be non-empty, so the field is not dropped — these owners name no leaf directory on disk at
all (`…/🎚️config/📄snapshot` does not exist), and the honest reading of the field is "wherever this
leaf's payload schema lives, it lives at the taxonomy default".

```
$ grep -r 'payload_schema: "🔣️.schema.json"\|payload_schema: "🧬️.schema.json"' ✏️s | wc -l
0
```

---

## 4. Row 39 — one retained-command vocabulary

`🧰️framework/🔨️modules/🖱️ui/🧬️schema/🔣️.json` was tightened by the framework worker **while this pass
ran** (its `retainedCommandDisposition` gained `unclassified`/`forbidden-from-ui`/`deleted` and spells
`batch-only-pending-rewrite` in full at 20:22, after an earlier 19:xx read said `batch-only`). The
final vocabulary this pass normalised to is the one on disk now:

| Axis | `framework.ui` definition | Vocabulary |
|---|---|---|
| lane | `retainedCommandLane` | `artifact`, `config`, `host-only`, `draft`, `presence`, `transient`, `child` |
| disposition / status / admission / classification | `retainedCommandDisposition` | `unclassified`, `migrated`, `batch-only-pending-rewrite`, `forbidden-from-ui`, `deleted`, `fail-closed` |
| execution | `retainedCommandExecution` | `bounded-first-step`, `bounded`, `batch`, `resumable` |

### Casing table actually applied

| Axis keys | Was (every spelling found) | Now |
|---|---|---|
| `lane`, `lanes`, `publicationLane`, `publicationLanes` | `Artifact` | `artifact` |
| | `Config` | `config` |
| | `HostOnly`, `hostOnly` | `host-only` |
| | `Transient` | `transient` |
| | `Child` | `child` |
| | `Draft` | `draft` |
| | `Presence` | `presence` |
| `disposition`, `status`, `admission`, `classification` | `Migrated` | `migrated` |
| | `BatchOnlyPendingRewrite`, `batchOnlyPendingRewrite`, `batchOnly`, `BatchOnly`, `batch-only` | `batch-only-pending-rewrite` |
| | `failClosed`, `FailClosed` | `fail-closed` |
| | `Unclassified` / `ForbiddenFromUi` / `Deleted` | `unclassified` / `forbidden-from-ui` / `deleted` |
| `execution` | `Bounded`, `Batch`, `Resumable`, `BoundedFirstStep` | `bounded`, `batch`, `resumable`, `bounded-first-step` |

The vocabulary was applied in two waves because `framework.ui` changed underneath (O-6): the first
wave rewrote **20 data files / 483 values** and **18 schema modules / 134 `const`+`enum` values**, the
second re-mapped `batch-only` → `batch-only-pending-rewrite` in **7 data files / 68 values** and the
matching schema narrowings. Both passes are idempotent and now report nothing left to do:

```
$ python3 wp4b-retained-vocabulary.py plan     → files: 0  rewrites: 0
$ python3 wp4b-retained-vocabulary.py schema   → files: 0  rewrites: 0
```

Six `const` narrowings sat in an `if`/`then` guard whose `properties` block names only the axis key
(sequence's `classification: {const: "Migrated"}`), which the deliberately conservative row test does
not reach; those were repaired in a second, explicitly-listed pass over six modules (flow
action-cohort, sequence, lowpoly, draw, raster, puzzle).

The normaliser only touches a node carrying a retained-command **row shape** (`id`/`toolId` beside a
lane/disposition/execution key, or `lanes` beside `status`/`routes`). That is why the architect
program's unrelated `status: "Blocked"/"Open"/"Closed"` mutation payloads and lowpoly's separate
`preparation: ["Artifact","Config"]` axis are untouched — a looser first rule would have rewritten 127
of the former and was discarded before anything was written.

### Oracles taught the mapping

Twelve `📜️script.ts`/`🟦️.ts`/`🟨️.js` oracles compared these values against Rust variant names. Each
grew one three-line helper

```ts
const variant = (value: string): string => value.split("-").map((part) => `${part[0]!.toUpperCase()}${part.slice(1)}`).join("");
```

(`host-only` → `HostOnly`, `batch-only-pending-rewrite` → `BatchOnlyPendingRewrite`,
`forbidden-from-ui` → `ForbiddenFromUi` — exactly the `ArtifactToolPublicationLane` and
`InteractiveJobClassification` variants) and its comparisons were switched to the kebab spelling:
flow, process, block, draw, mathematical, puzzle, lowpoly, sequence, note, norm, cad, space.

Final census of every retained-command row value left in plugin data:

```
lane:        {artifact: 164, child: 1, config: 93, document: 1, draft: 1, host-only: 55, transient: 30}
disposition: {batch-only-pending-rewrite: 68, fail-closed: 37, migrated: 165}
execution:   {batch: 25, bounded: 80, bounded-first-step: 8, resumable: 40}
```

Every value is in the `framework.ui` vocabulary except the single `document`, which is flow's **store**
lane (`document|config|draft|presence|transient` in `🏪️store-owners` and `👁️viewer/🧹️owners`) — a
different axis from `ArtifactToolPublicationLane`, correctly left alone.

### Proof — all ten `framework.ui`-referencing exports validate their real fixture

Before the pass, nine of ten failed against the tightened shared shape (`must be equal to one of the
allowed values ["migrated","batch-only-pending-rewrite",…]`, `must be equal to constant ["Artifact"]`).
After:

```
PASS PresentationRetainedCommandLimits   PASS ShootingRetainedCommandLimits
PASS Fem3dRetainedCommandLimits          PASS RemodelingRetainedCommandLimits
PASS HomeRetainedCommandLimits           PASS SpaceIndexRetainedCommandLimits
PASS SpacePlayRetainedCommandLimits      PASS VcsRetainedCommandRoutes
PASS WiresRetainedCommandRoutes          PASS ProcedureRetainedCommandRoutes
```

---

## 5. Row 83 + contract §A/§B — `$id` grammar and draft-07

`wp4b-plugin-schema-ids.py` derives every module's scope path from its own directory and writes
`$id = https://semio.tech/schema/<scope path>/<facet>.json` plus the draft-07 dialect on every facet
document, migrating 2020-12 keywords (`prefixItems` → `items` + `additionalItems`, `unevaluated*`
dropped).

```
$ python3 wp4b-plugin-schema-ids.py apply
modules: 330  id-rewrites: 507  dialect-rewrites: 821  skipped-facets: 0  action: apply
```

Three shapes changed:

1. **stdio (row 83).** `s.stdio.<artifact>` (a *dotted* segment, standard and subset dropped) →
   `s/stdio/<artifact>/<standard>/<subset>`, the same slash scope path row 45/78 settled for the
   mutation leaves. `https://semio.tech/schema/s.stdio.las/artifact.json` →
   `…/s/stdio/las/1.0/header/artifact.json`; `…/stdio.epw/snapshot.epw` (not even a `.json` facet) →
   `…/s/stdio/epw/energyplus/any/snapshot.json`. This is what makes the 491 stdio
   `mutation-leaf-id-grammar` findings resolvable: the leaves already write
   `…/s/stdio/las/1.0/header/mutation/set-point/schema.json`, and the checker derives the expected
   prefix from *this* root module's `$id`.
2. **Surface modules.** `✏️editor/🎚️config/🧬️schema` and `✏️editor/👥️presence/🧬️schema` both declared
   `…/app/<plugin>/<artifact>/{config,presence}.json`, which is **one** scope id
   (`app.<plugin>.<artifact>`) claimed by two modules — the 76 `scope-id-duplicate` rows. The surface
   is now the last scope segment and the facet is `schema.json`:
   `…/app/gis/gis2d/config/schema.json` and `…/app/gis/gis2d/presence/schema.json`, scope ids
   `app.gis.gis2d.config` / `app.gis.gis2d.presence`. Their facet children follow
   (`…/app/gis/gis2d/config/diff.json`).
3. **Facet children that had drifted off their module's scope path** (`cad/snapshot/text.json` →
   `s/cad/cad/snapshot/text.json`, `stdio.dwg/diff/text.json`, …) — the 191
   `module-scope-id-inconsistent` rows.

483 moved ids were rewritten repo-wide in every `$ref`/reader literal that named them — only **15
files** actually carried one, all of them stdio modules referencing their own siblings. (The
`🗑️generated/wp4b-id-map.json` this pass wrote was swept from the generated folder by another session
before the report was finished; re-running `wp4b-plugin-schema-ids.py apply` regenerates it and is a
no-op on the tree now that every `$id` already matches.) 13 old ids were
**ambiguous** (two modules had declared the same id — `s.stdio.gif/artifact.json` for both the `87a/any`
and `89a/base` subsets, `stdio.avi.diff.json` for three) and are reported rather than guessed, because
a reference to a duplicated id never named one module unambiguously in the first place.

---

## 6. Contract §B — `x-semio-formats`

`wp4b-law-export-formats.py` annotates `"x-semio-formats": ["🔣️jsonschema"]` on **80 exports across
21 multi-format modules**: the retained-command / scene-owner / identity / preview laws WP4 and this
pass merged into subset and surface modules, which are validated by an Ajv oracle and never
transported.

The rule is deliberately narrow. A first attempt — "annotate every `$defs` key that appears in none of
the module's `🦀️.rs`/`🟦️.ts`/`🔗️.graphql`/`🛰️.proto`" — reported 166 exports and was **wrong**:
`app.raster.raster.presence`'s `RasterCamera` is absent from those files only because the Rust twin is
spelled `RasterPresenceCamera`. Annotating it would have declared a real five-format export
JSON-only. The applied rule requires *both* absence from every sibling format *and* membership in the
explicit list of export ids `📓️wp4-plugins.md` §1–§4 and this report §1–§2 name. The `RasterCamera`
class of drift is an **export-id mismatch**, not a format gap — see open question O-1.

Consumers that compile these modules with `strict: true` must register the keyword, or Ajv throws
before validating anything (the same trap as `x-semio-state`):

```ts
ajv.addKeyword({ keyword: "x-semio-formats", metaSchema: { type: "array", items: { type: "string" } } });
```

---

## 7. Verification — real output

### 7.1 `schema check`, filtered to this partition

`bun ./📜️script.ts schema check --report <ticket>/🗑️generated/schema-check-w6c.jsonl`, then
`path.startswith("✏️s/") and "🧬️schema/🧬️mutations/" not in path`. **The diagnostic vocabulary was
rewritten by the tooling worker between the baseline and the final run** (`document-dialect-unexpected`
→ `schema-dialect-not-draft-07`, `placement-retired-location` → `schema-placement-forbidden-filename`,
`scope-id-duplicate` → `schema-scope-ambiguous`, `module-scope-id-inconsistent` retired, and two new
rules — `schema-export-incomplete` and `schema-export-id-duplicate` — added), so the table pairs each
old code with the rule that replaced it.

| Baseline code (my partition) | → | Final code (my partition) |
|---|---|---|
| `placement-retired-location` = **11** | → | `schema-placement-forbidden-filename` = **0** ✅ |
| `document-dialect-unexpected` = **825** | → | `schema-dialect-not-draft-07` = **0** ✅ |
| `document-id-unaddressable` = **54** | → | `schema-document-id-unaddressable` = **0** ✅ |
| `document-id-duplicate` = **9** | → | `schema-module-id-duplicate` = **0** ✅ |
| `scope-id-duplicate` = **76** | → | `schema-scope-ambiguous` = **0** ✅ |
| `module-scope-id-inconsistent` = **191** | → | rule retired; no successor row = **0** ✅ |
| `document-id-missing` = 9 + `module-scope-id-missing` = 48 | → | `schema-module-id-missing` = **38** (see below) |
| `module-level-ineligible` = 65 | → | `schema-owner-ineligible` = **58** |
| `export-id-invalid` = 221 | → | `schema-export-id-invalid` = **219** |
| `ref-unresolved` 20 + `ref-not-*` 197 | → | `schema-ref-unresolved` = **67** |
| — (new rule) | → | `schema-export-incomplete` = **6409** |
| — (new rule) | → | `schema-export-id-duplicate` = **655** |

```
$ bun ./📜️script.ts schema check --report …/🗑️generated/schema-check-w6c.jsonl
[schema check] modules=3205 scopes=3069 findings=8235
[schema check] schema-catalog-stale=1
[schema check] schema-dialect-not-draft-07=4
[schema check] schema-document-id-unaddressable=2
[schema check] schema-export-id-duplicate=655
[schema check] schema-export-id-invalid=322
[schema check] schema-export-incomplete=6650
[schema check] schema-fixture-defines-schema=61
[schema check] schema-module-id-missing=42
[schema check] schema-mutation-leaf-id=362
[schema check] schema-owner-ineligible=63
[schema check] schema-placement-forbidden-filename=1
[schema check] schema-ref-unresolved=70
[schema check] schema-scope-ambiguous=2

$ python3 - (filter to ✏️s/** outside 🧬️schema/🧬️mutations/**)
repo findings 8235 | my partition 7448
  schema-export-id-duplicate=655
  schema-export-id-invalid=219
  schema-export-incomplete=6409
  schema-fixture-defines-schema=2
  schema-module-id-missing=38
  schema-owner-ineligible=58
  schema-ref-unresolved=67
--- repo-wide codes with ZERO rows in my partition:
  schema-catalog-stale: repo=1  mine=0
  schema-dialect-not-draft-07: repo=4  mine=0
  schema-document-id-unaddressable: repo=2  mine=0
  schema-mutation-leaf-id: repo=362  mine=0
  schema-placement-forbidden-filename: repo=1  mine=0
  schema-scope-ambiguous: repo=2  mine=0
```

**Every placement / dialect / id code is 0 in this partition.** Two side-effects worth naming:

- repo-wide `schema-mutation-leaf-id` fell **1763 → 362**. The stdio root-module `$id` rewrite (§5.1)
  is what unblocked the 491 stdio leaves; the leaves themselves were never touched by this pass.
- `schema-module-id-missing = 38` is **not** an id-grammar defect. All 38 are subset modules that have
  **no `🧬️schema/🔣️.json` at all** (verified: `os.path.exists` is false for every one) — they carry
  `🦀️.rs`, `🟦️.ts` and a `🧬️mutations/` tree but no JSON artifact facet. Writing a root there would
  mean inventing a contract; the honest source is the Rust type projection, i.e.
  `wp4-stdio-schemas.py audit --write` — cross-partition request **G**.

### 7.2 Module structure — `wp4-final-check.py`

The `touched` list was extended with the six modules this pass created or wrote into (raster, puzzle,
mathematical plugin roots; the cad subset and cad presence surface; the sequence subset):

```
$ python3 wp4-final-check.py
modules checked: 36 | problems: 0
```

(dialect, `$id` prefix, and every intra-module / `framework.ui` `$ref` resolving.)

### 7.3 The ten `framework.ui` law exports against their real fixtures

`wp4b-retained-export-proof.ts` (kept in the ticket folder) loads `framework.ui` + each owner module
and validates the committed fixture:

```
$ bun wp4b-retained-export-proof.ts
PASS PresentationRetainedCommandLimits   PASS ShootingRetainedCommandLimits
PASS Fem3dRetainedCommandLimits          PASS RemodelingRetainedCommandLimits
PASS HomeRetainedCommandLimits           PASS SpaceIndexRetainedCommandLimits
PASS SpacePlayRetainedCommandLimits      PASS VcsRetainedCommandRoutes
PASS WiresRetainedCommandRoutes          PASS ProcedureRetainedCommandRoutes
```

### 7.4 Rewired oracles that pass

```
$ ✏️s/🔌️plugins/➗️mathematical/📦️packages/🟦️typescript  bun ./📜️script.ts publication-authority-audit
validated Mathematical publication authority; routes=7; schema=Ajv; oracle=owned; hostile=3
 2 pass  0 fail

$ ✏️s/🔌️plugins/🖍️draw/📦️packages/🟦️typescript  bun ./📜️script.ts publication-authority-audit
validated Draw publication authority; apps=DrawingPlayApp:26; schema=Ajv; oracle=owned; hostile=5
 2 pass  0 fail

$ ✏️s/🔌️plugins/🗄️stdio  bun ./📜️script.ts package-contract
[stdio-package-contract] AJV accepted=2 rejected=2
[stdio-package-contract] Cargo metadata packages=36
[stdio-package-contract] source packages=36 dag=valid

$ ✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust  bun ./📜️script.ts mutation-leaf-taxonomy-check
norm mutation-leaf taxonomy is fresh: 392 payloads, AJV schema and hostile vectors passed

$ ✏️s/🔌️plugins/📕️norm/📦️packages/🟦️typescript  bun ./📜️script.ts test
norm retained cohort ok: 15 editors × 3 migrated routes, 0 descriptor rows carried a classification
 15 pass  0 fail

$ ✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust  bun ./📜️script.ts plugin-identity-check
plugin-identity-check: checks=11 clean
$ …                                          home-directory-projection-persistence-check
home-directory-projection-persistence-check: checks=11 clean
$ …                                          interactive-job-catalog-check
interactive-job-catalog: descriptor rows without an interactiveJob disposition: 0
interactive-job-catalog-check: checks=23 clean

$ ✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust  bun ./📜️script.ts component-cold-map-patch-check
gis-component-cold-map-patch-source: AJV=1 SHA256=node+webcrypto hostile=5 markers=9

$ (proveVcsNativeCodecReceipts + proveGisNativeCodecReceipts, run directly)
vcs-native-codec-oracle: receipts=1 hostile=9 ajv+node+webcrypto=1 dependency-coherence=3
[DEBUG] VcsNativeCodecs + ArtifactDocumentIdV1: fixture accepted, receipts proven
gis-native-codec-oracle: receipts=2 hostile=8 ajv+node+webcrypto=1
[DEBUG] GisNativeCodecs + ArtifactDocumentIdV1: fixture accepted, receipts proven

$ node '…/🌉️wasm/🧪️tests/🧬️schema/🟨️.js'
{"facet":"📜️interface","features":10,"operations":47,"events":8,"framing":"A1-AbiMessage-v1"}
$ node '…/🌉️wasm/📦️packages/🟨️javascript/🧪️tests/🟨️.js'
{"oracle":"ajv-draft07+dagre-0.8.5+graphlib","routes":17,"migrated":17,"pending":0,"hostileLaws":7,…}

$ (every export of the largest rewritten module, compiled against framework.ui + the vendor keywords)
[DEBUG] s.flow.flow: 47/47 exports compile []

$ (every JSON file under ✏️s, re-parsed after the sweeps)
json files parsed: 20959 | unparseable: 0

$ ✏️s/🔌️plugins/🧱️block  (publication-authority region, replayed directly)
[DEBUG] BlockPublicationAuthority accepts fixture: true
[DEBUG] Block2dPlayApp: rust-contracts=9  fixture-routes=9  lanes-match=true
[DEBUG] Block3dPlayApp: rust-contracts=23 fixture-routes=23 lanes-match=true
[DEBUG] Block5dPlayApp: rust-contracts=7  fixture-routes=7  lanes-match=true
```

### 7.5 Rust — `semio-s-plugin-puzzle`, wasm32-wasip2

```
$ CARGO_TARGET_DIR=<scratchpad>/target-w6 RUSTC_WRAPPER="" CARGO_PROFILE_WASM_DEV_DEBUG=false \
    cargo check --target wasm32-wasip2 -p semio-s-plugin-puzzle
…
error: could not compile `semio-s-artifact-puzzle-3d` (lib) due to 2 previous errors; 21 warnings

$ grep -o '^error\[[A-Z0-9]*\]' puzzle-wasm-check.txt | sort | uniq -c
   2 error[E0433]      # cannot find `examples` in `crate`
$ grep -c '⏳️precompute/🪣️fill' puzzle-wasm-check.txt
0
```

**WP4's 163 errors are down to 2, and neither names the file WP4 edited.** Both are
`crate::examples::puzzle3d::…` at
`🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️.rs:37`, unresolved because a peer's
**uncommitted** working-tree change in `🗿️artifacts/🧊️3d/🦀️.rs` put `pub mod examples` behind
`#[cfg(feature = "component-app-assembly")]` while the call site at line 37 is not gated
(`git diff HEAD` on that file shows the gate being introduced). Full log:
`🗑️generated/puzzle-wasm-check.txt`.

### 7.6 Failures confirmed as NOT this pass's

Each was checked against `HEAD` before being classified.

1. **`🧩️puzzle` `publication-authority-audit`** — the Ajv step passes; `ownerOracle` then rejects
   `Puzzle2dPlayApp` on two anchors (`Ok(puzzle2d_dispatch_emit(command, snapshot.0.clone(), …))` and
   the `before, config, interaction.selection(…)` twin) that are absent **both** in the worktree and at
   HEAD. Stale assertion, pre-existing.
2. **`🏭️process` `test`** — passes the Ajv step and every lane comparison
   (`groups=5 publications=33`, all lanes matched after `variant()`); `sourceOracle` then fails on the
   single anchor `ToolCancellationPolicy::PerOperation`, present at HEAD and gone in the worktree — a
   peer's uncommitted edit.
3. **`💠️lowpoly` `test`** — Ajv, the 8-arm lane/preparation signature law and the 47 classification
   rows all pass after the vocabulary change; it then fails on
   `"addPrimitive" => semio_framework::ToolExecutionContract::resumable`, which has **0** occurrences
   at HEAD as well (the peer `super::`/qualifier-stripping sweep WP4 §5.6 already recorded).
4. **`🌊️flow` `action-cohort-audit`** — 16 publication-contract rows in the fixture have no matching
   `ArtifactToolPublicationContract` in the editor `🦀️.rs`. Reproduced identically against HEAD's copy
   of that file with the same fixture, so the divergence is in the source, not in the re-casing.
5. **`🌍️gis` `map-create-region-group-check`** — Ajv passes; the source law then fails on
   `fixture["membershipCases"]`, which has 0 occurrences in `💡️inferences/🦀️.rs` in the worktree **and**
   at HEAD (it passed for WP4 four hours earlier, so a peer removed it since).
6. **`🧱️block` `test`/`publication-authority-audit`** — dies before its own audit in the `bun test`
   step, on `🚪️io/🧪️tests/🧩️suite/🟦️.ts` reading `./🧫️fixtures/…`: a peer's test-layout sweep moved
   `🟦️.ts` into `🧩️suite/` and left `🧫️fixtures` one level up. The audit region itself was replayed
   directly and passes (§7.4).
7. **`🌿️vcs` `native-codec-check` full command** — the Ajv and receipt stages pass; the command then
   runs `runExactCargoLaws`, whose native `cargo test` stage fails with the workspace-wide
   `E0432: unresolved import semio_framework_schema_registry` (the framework-schema worker's in-flight
   registry split, ledger row 76).

---

## 8. Cross-partition requests

**A. `🌎️hub/📦️packages/🦀️rust/📜️script.ts` + two hub fixtures — `📜️native-codec-factories.json`
moved out of the stdio registry's schema module.** Exact replacements:

| File | Line | Was | Now |
|---|---|---|---|
| `🌎️hub/📦️packages/🦀️rust/📜️script.ts` | 9134 | `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️schema/📜️native-codec-factories.json` | `✏️s/🔌️plugins/🗄️stdio/📇️registry/📜️native-codec-factories.json` |
| same | 9166 | same literal | same replacement |
| same | 9170 | `const gisSchemaPath = "✏️s/🔌️plugins/🌍️gis/📇️native-codecs/🧬️.schema.json"` (**deleted**) | read `✏️s/🔌️plugins/🌍️gis/🧬️schema/🔣️.json`, `addSchema`, `$ref …/s/gis/schema.json#/$defs/GisNativeCodecs` (draft-07 `ajv`, register `x-semio-state` + `x-semio-formats` keywords) |
| `🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🧫️fixtures/🪪️v1/🔣️.json` | 3 | `"providerProjection": "…/🧬️schema/📜️native-codec-factories.json"` | drop the `🧬️schema/` segment |
| `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧫️fixtures/🧬️stdio-gis-bootstrap/🔣️.json` | 63 | `"stdioReceipts": "…/🧬️schema/📜️native-codec-factories.json"` | drop the `🧬️schema/` segment |

(The `🌎️hub` sites that read the gis/vcs **data** file `📇️native-codecs/🔣️.json` — lines 4912, 7968,
9135, 9167 and the two `include_str!`s in `🗿️artifact-authority` — are unaffected; only the schema
sibling moved.)

**B. `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧬️schema/🔣️.json` — `$id` unchanged,
export named.** gis and vcs now compile `ArtifactDocumentIdV1` out of that module. No change is
requested; this records the new dependency edge (`s.gis`, `s.vcs` → `os.plugin.registry`) for the
catalog's `dependsOn`.

**C. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`.**
1. `schemaFacetKinds.📜️interface.facetPathIdentities` must gain
   `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🧬️schema`
   so the sequence wasm module is catalogued as an interface facet (row 67).
2. `semanticDirectoryMemberKinds.members-of-fixtures.memberNames` still lists
   `🎚️playbook-view-command-limits.schema.json` and `👁️playbook-view-command-limits.json`; both are
   deleted (§1).
3. `schemaScopeOwnerLevels` — the four surface scope ids gained a segment
   (`app.<plugin>.<artifact>.config|presence`); nothing in the taxonomy pins the old two-segment form,
   but the catalog must be regenerated.

**D. Catalog regeneration.** `🔣️schema-catalog.json` / `📓️schema-catalog.md` are stale against 507
rewritten `$id`s, 12 new plugin-root scopes, the surface scope split (≈ +80 scopes) and the deleted
`PlaybookViewCommandLimits`. `catalog-stale=1` is expected until `schema generate` runs.

**E. Repo library helper (restates `📓️wp4-plugins.md` §6G, now larger).** Fifteen scripts in this
partition each carry the same `compile<scope>Export` six-liner, and every one of them now also has to
register `x-semio-state` **and** `x-semio-formats` plus the numeric formats. Hoist
`compileScopeExport(modulePath, exportId)` into the repo library TS package.

**F. `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧬️schema/📸️snapshot/🧪️tests/🔬️slider-label/🦀️.rs`,
`…/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts`, `🌎️hub/📦️packages/🦀️rust/📜️script.ts`,
root `📜️script.ts`** name an `✏️s/…/🧪️fixtures/…` path that this pass renamed to `🧫️fixtures`
(taxonomy `testFixturesDirName`, ledger row 27). Replace the `🧪️fixtures` segment with `🧫️fixtures`
in the `✏️s` paths those four files spell.

---

**G. `wp4-stdio-schemas.py` (W8c) — 38 subsets have no root `🧬️schema/🔣️.json`.** They carry
`🦀️.rs`, `🟦️.ts` and a full `🧬️mutations/` tree, so `schema check` sees documents in the module but no
root `$id`. The honest root is the Rust type projection the stdio tool already produces; writing one by
hand would invent a contract. The 38 (all `🗄️stdio`): `zip/2.0/iso21320`, `svg/1.1/{tiny,basic,base}`,
`ifc/2x3/{cobie,cv20,sav}`, `step/ap214/{cc1..cc6}`, `pdf/{4️⃣1.4/{x,a},7️⃣1.7/{ua,h,e,x,a,vt}}`,
`docx/ecma-376/{strict,transitional}`, `pptx/ecma-376/{transitional,strict}`,
`xlsx/ecma-376/{transitional,strict}`, `xml/1.0/{valid,base}`, `csv/rfc4180/any`, `tsv/iana/any`,
`epw/energyplus/any`, `mp4/isobmff/any`, `avi/1.0/hdrl`, `jpg/jfif-1.01/baseline`, `tiff/6.0/baseline`,
`json/rfc8259/i-json`, `gltf/2.0/{animation,camera,scene,material,buffer,mesh,skin,asset}`,
`semio/v1/presentation`.

---

## 9. Open questions

**O-1. `schema-export-incomplete = 6409` and `schema-export-id-duplicate = 655` are an export-naming
job, not a format job.** Both rules landed in the tooling while this pass ran. Sampling them shows the
same root cause twice:

- `app.writer.writer.config declares export WriterCamera in 🔣️.json; 🦀️rust carries no declaration of
  it` — the Rust twin is spelled `WriterConfigCamera`. The export exists in all five formats under a
  different name. Annotating `x-semio-formats` here would be a lie, which is exactly why §6's rule
  refuses to (the `RasterCamera` / `RasterPresenceCamera` case).
- `Export WriterArtifact is already declared by …/🧬️schema/🔣️.json` from `🔺️diff/🔣️.json` — facet
  children restate the root's `$defs` rather than referencing them.

Both need one deliberate pass that aligns JSON `$defs` keys with the Rust/TS/GraphQL/proto declarations
and makes facet children `$ref` the root's exports. That is a work package, not a sweep, and it should
be scheduled with a decision on which side is authoritative (contract §A says the export id is the
same-named Rust `struct/enum`, so the Rust spelling wins).

**O-2. `schema-export-id-invalid = 219` is blocked on ledger row 77 being implemented.** They are
camelCase `$defs` keys (`valueId`, `xmlNode`, `visualStyle`, …) in the stdio and cad artifact modules —
module-internal helpers that contract §A says belong in `definitions`. Moving them there is mechanical,
but the checker still reports every local `#/definitions/<helper>` reference as a bad ref until W2c
implements row 77's decision, so the fix would trade 219 rows of one code for a larger number of
another. Recommendation: land row 77 first, then move the helpers in one pass.

**O-3. `schema-owner-ineligible = 58` — three distinct shapes, one taxonomy question each.**
- 36 stdio + 15 norm `🗿️artifacts/<a>/🧬️schema/📜️artifact-definition.json`: an **artifact**-level
  module holding the package-definition row that the artifact's `🏅️standards/🔖️N/🦀️.rs` `include_str!`s.
  Either `schemaScopeOwnerLevels` admits `✏️s/🔌️plugins/*/🗿️artifacts/*`, or the definition file moves
  out of the `🧬️schema/` slot (it is data, like `📜️native-codec-factories.json` in §1).
- `✏️editor/🌉️wasm/🧬️schema` (sequence) and `📸️snapshot/🧬️generation/🧬️schema` (procedural),
  `🎬️action-cohort` (flow), `📇️registry` (stdio), `🧩️extensions/*` (playbook, cad): nested module
  directories at levels no pattern matches. Ledger row 53 already asks for the `plugin-extension`
  level; the same decision is needed for `🌉️wasm` (row 67, request **C**) and for a plugin's own
  named sub-areas (`🎬️action-cohort`, `📇️registry`).
- `📕️norm/{🎚️config,👥️presence}` and `🪐️space/⚙️engine/🪐️space/{🎚️config,👥️presence}`: surface lanes
  that are **not** under an `✏️editor/`, so `**/✏️editor/🎚️config` does not match them. The pattern
  should be `**/🎚️config` (etc.), or these two plugins should grow the `✏️editor` segment.

**O-4. The 13 ambiguous stdio ids.** `s.stdio.gif/artifact.json` was declared by both the `87a/any`
and `89a/base` subsets, `stdio.avi.diff.json` by three modules, and so on. The `$id`s are now distinct,
but the ref rewriter deliberately left references to the old ids alone rather than guess a target.
Verified: `git grep` finds **no** reference to any of the 13 anywhere — they were `$id` declarations
only, never `$ref` targets — so nothing dangles. Recorded in case a later pass expects a mapping.

**O-5. `x-semio-formats` needs a home in the shared Ajv setup.** Registering the keyword had to be
added to 15 scripts by hand (request **E**); a sixteenth consumer written tomorrow will throw
`strict mode: unknown keyword: "x-semio-formats"` before validating anything, exactly as
`x-semio-state` does today. Until `compileScopeExport` exists in the repo library, every new consumer
repeats the trap.

**O-6. The framework worker's vocabulary moved mid-pass.** `retainedCommandDisposition` read
`["migrated","batch-only","fail-closed"]` at 19:xx and
`["unclassified","migrated","batch-only-pending-rewrite","forbidden-from-ui","deleted","fail-closed"]`
at 20:22. The data was normalised twice as a result. If `framework.ui` is tightened again, re-run
`wp4b-retained-vocabulary.py plan` and `schema-apply` — the tables at the top of that script are the
single place to update.
