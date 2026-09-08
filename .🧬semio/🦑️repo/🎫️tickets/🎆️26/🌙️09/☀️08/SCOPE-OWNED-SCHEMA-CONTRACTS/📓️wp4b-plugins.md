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

| Key | Was | Now | Data | Schema |
|---|---|---|---|---|
| `lanes` / `lane` / `publicationLane(s)` | `Artifact` | `artifact` | 200 | 11 |
| | `Config` | `config` | 116 | 11 |
| | `HostOnly`, `hostOnly` | `host-only` | 77 | 11 |
| | `Transient` | `transient` | 30 | 9 |
| | `Child` | `child` | 1 | 9 |
| | `Draft` | `draft` | 0 | 8 |
| | `Presence` | `presence` | 0 | 8 |
| `disposition` / `status` / `admission` / `classification` | `Migrated` | `migrated` | 128 | 10 |
| | `BatchOnlyPendingRewrite`, `batch-only` | `batch-only-pending-rewrite` | 93 | 21 |
| | `failClosed` | `fail-closed` | 37 | 1 |

**20 data files** (483 value rewrites) and **18 schema modules** (134 `const`/`enum` rewrites) in this
partition. The normaliser only touches a node that carries a retained-command row shape (`id`/`toolId`
beside a lane/disposition/execution key, or `lanes` beside `status`/`routes`), so the architect
program's unrelated `status: "Blocked"/"Open"/"Closed"` payloads and lowpoly's separate
`preparation: ["Artifact","Config"]` axis are left alone — an earlier, looser rule would have
rewritten 127 of them and was discarded.

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

483 moved ids were rewritten repo-wide in every `$ref`/reader literal that named them; 14 old ids were
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

*(filled in below)*

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

## 9. Open questions

*(filled in below)*
