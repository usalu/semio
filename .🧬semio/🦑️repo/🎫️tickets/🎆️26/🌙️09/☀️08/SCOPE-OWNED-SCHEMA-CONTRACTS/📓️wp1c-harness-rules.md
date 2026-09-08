# WP1c — Format-coverage rule, reference rules, catalog shape adoption

Worker: W1c (Opus). Partition: `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/**`.
Inputs: `📋️execution-contract.md` §A/§B/§C, `📓️wp1-harness-invariants.md`, `📓️wp1b-harness-triage.md`,
`📋️cross-partition-requests.md` rows 21, 32, 43, 44, 56, 62, 77, 78.

Assignment: row 56 (`x-semio-formats` completeness), row 77 (module-internal vs cross-document `$ref`s,
aggregate branches by absolute leaf `$id`, leaf `$id` grammar per row 78), row 43 (catalog `exports`
object shape), rows 21/44 verification, vector + test updates, real runs.

---

## 1. Headline

| | |
|---|---|
| Rules landed | row 56 (two new diagnostic codes), row 77 (three legal reference shapes, JSON-pointer resolution), row 78 leaf-`$id` grammar as a derived helper |
| Catalog shape | row 43 **landed on both sides during this work-package**: the harness already read the new shape (W1b), and the tooling worker regenerated `🔣️schema-catalog.json` into it at 19:51 — 3 011 scopes, `exports: {Id: {file, facet}}`, nested facet files included. The live `test schema` run below is therefore a real measurement, not a shape refusal |
| Vector | `🧪️tests/🧬️schema-invariants/🔣️.json` +9 catalog cases, +1 new collection (`mutationLeafIdCases`, 3 cases) |
| Invariants suite | **90 pass / 1 fail** (91 tests, 356 expect() calls, 17.8 s) after a peer extended the same suite mid-session (§7). The one red is W1b's R-4 policy disagreement, deliberately left red |
| `test schema` | 5 165 findings over the tree, 0 × `schema-catalog-malformed` (was 2 583), per-owner table in §5.3 |
| Rows 21 / 44 | verified on disk; row 44 now migrated on **both** sides |

---

## 2. Rule changes, with the test that holds each one

All in `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts`; every row is
driven from the language-agnostic vector `🧪️tests/🧬️schema-invariants/🔣️.json` and asserted by
`🧪️tests/🧬️schema-invariants/🟦️.ts`.

### 2.1 Row 56 — format coverage and `x-semio-formats`

The rule, as the contract §B states it and as the code now implements it:

- an export with **no** annotation must exist in **every format the scope provides** (a format the scope
  provides = a format file present in the module, which is what the catalog's `formats` records);
- an export carrying `"x-semio-formats": [...]` must exist in **exactly** those formats — both
  directions, because "restricted support declared honestly" is only honest if it is also complete:
  a format it declared and does not implement is a broken promise, and a format it implements without
  declaring makes the annotation a comment rather than a contract;
- the annotation's vocabulary is the taxonomy's `schemaFormats` keys, read from the taxonomy, never
  restated in the harness.

Two new diagnostic codes, one per distinguishable failure (the file's existing discipline):

| code | fires when |
|---|---|
| `schema-export-format-undeclared` | the export IS declared in a format its `x-semio-formats` does not name |
| `schema-export-formats-annotation-invalid` | the annotation is not a non-empty array, or names an id the taxonomy does not declare as a schema format |

A format the annotation names that the scope implements no file for stays `schema-export-incomplete`
(with the `format` field set and no `path`, since there is no file to point at) — the export is missing
from a format it declared, which is the same defect the unannotated case already had a code for.

New API: `restrictedExportFormats(repoRoot, definition)`, `declaredSchemaFormatIds(repoRoot)`,
`SCHEMA_EXPORT_FORMATS_KEYWORD`.

Tests (`📚️ catalog completeness and cross-scope dependencies`, one per vector case):

| case | asserts |
|---|---|
| `export-restricted-to-json-schema-needs-no-other-format` | **false positive guard**: a JSON-Schema-only export in a five-format scope whose four other files declare something else produces *no* finding |
| `export-restricted-to-json-schema-that-a-format-implements-anyway` | 4 × `schema-export-format-undeclared`, one per format that implements it anyway |
| `export-missing-from-a-format-it-declared-itself` | `schema-export-incomplete` for `🦀️rust` only |
| `export-annotation-naming-a-format-the-taxonomy-does-not-declare` | `schema-export-formats-annotation-invalid` (`"rust"` instead of `🦀️rust`) |
| `export-annotation-that-is-not-a-list-of-formats` | `schema-export-formats-annotation-invalid` (a bare string) |

**Cross-implementation agreement.** W2c's generator vector already carries the same rule as case
`restricted-format-support` (`📚️library/🧪️tests/🧬️schema-scope-catalog/🧫️fixtures/🔣️.json`): a
`RetainedCommandLaw` restricted to `🔣️jsonschema` beside an unrestricted `RetainedCommandRoute`, with a
Rust file that declares neither, expecting exactly one finding — for the *unrestricted* export. Run
through this harness the same materialisation produces exactly one finding, for the same export, named
`schema-export-incomplete` instead of `export-format-missing`. The two implementations agree on the rule
and disagree only on the code vocabulary, which is the open coordinator call W1 already recorded (§8).

**Live effect today: zero.** No export in the tree carries the annotation yet (`grep -rl x-semio-formats`
over `✏️s`, `🌎️hub`, `🧰️framework` returns the two vectors and nothing else); annotating the law/contract
exports is row 55 (W6c). The rule is therefore proved on the vector and inert on the tree — which is the
correct state for it, and it means the 2 891 live `schema-export-incomplete` findings in §5.3 are all the
unannotated rule, not the new one.

### 2.2 Row 77 — module-internal refs are never findings

`schemaResolutionDiagnostics` previously accepted exactly two shapes: `#/$defs/<Export>` and
`<$id>#/$defs/<Export>`. Everything else — including every `#/definitions/<helper>` — was reported
`schema-ref-unresolved`. That is now three legal shapes, judged by what they address:

| shape | verdict |
|---|---|
| `#…` (any JSON pointer into the same document: `#/definitions/<helper>`, `#/$defs/<Export>`, `#`) | **legal**. Resolved by JSON pointer (`~0`/`~1` unescaping, array indices). Only a pointer that lands on nothing is a finding, and that is a broken document, not an addressing rule (a peer has since given it its own code, `schema-ref-broken-internal`) |
| `<target $id>` with no fragment | **legal** — the aggregate-branch form. A mutation leaf is its own scope and its whole document IS the payload contract, so an aggregate references it by absolute `$id`. Subject to the `dependsOn` rule like any cross-scope reference |
| `<target $id>#/$defs/<ExportId>` | **legal**, as before |
| anything else — a relative file path, `<target $id>#/definitions/<helper>`, any other fragment | `schema-ref-unresolved`. A cross-document reference into another module's private helpers addresses no export, and one resolved by filesystem position is a dependency nobody declared |

Scope of the scan widened at the same time: refs are now read from **every normative document the scope
owns** (`scopeNormativeDocuments` = module root + each export's carrier file), not the module root alone.
A facet child (`🔺️diff/`, `📸️snapshot/`, `💡️inferences/`, `🧬️mutations/`, and their nested
`📝️text`/`💾️binary` children) declares exports of the same scope and writes its own refs; before row 43
there was no way to know which file that was. The `$id` index feeding cross-document resolution is built
from the same set, so a legal `<facet $id>#/$defs/<Export>` is no longer reported unresolved.

New API: `scopeNormativeDocuments(scope, normativeFormat)`, `mutationLeafSchemaId(...)` (§2.3), internal
`jsonPointerTarget`, `schemaRefDiagnostics`.

Tests:

| case | asserts |
|---|---|
| `module-internal-helper-ref-is-never-a-finding` | **false positive guard**: `definitions.author` + `{"$ref": "#/definitions/author"}` → no finding |
| `module-internal-helper-ref-that-lands-on-nothing` | `schema-ref-unresolved` |
| `cross-document-ref-into-another-modules-definitions` | `schema-ref-unresolved`, even with `dependsOn` declared |
| `mutation-aggregate-branches-reference-leaves-by-absolute-id` | **false positive guard**: an aggregate `🧬️mutations/🔣️.json` whose branch is `{"$ref": "<leaf $id>"}`, with the leaf catalogued as its own scope → no finding |
| `mutation-aggregate-branch-whose-leaf-scope-is-not-declared-as-a-dependency` | `schema-cross-scope-dependency-forbidden` |
| (existing) `recursive-ref-inside-one-module`, `unresolved-local-ref`, `ref-by-relative-file-path`, `cross-scope-ref-declared-in-depends-on`, `cross-scope-ref-not-declared-in-depends-on` | unchanged verdicts |

Measured effect on the real tree (`wp1c-shadow-probe.ts`, 3 655 normative documents of 3 011 scopes):

```
  16340 internal #/$defs/                 legal before and now
    541 internal #/definitions/           WAS 541 findings, now 0 unless the pointer is broken
    106 cross-doc #/$defs/                legal before and now
     68 cross-doc #/definitions/          finding (addresses no export)
     45 relative file path                finding
     15 cross-doc whole document ($id)    WAS 15 findings, now legal (aggregate-branch form)
      2 internal other pointer            legal unless broken
      1 cross-doc other fragment          finding
```

**556 of the 684 non-`#/$defs/` references stop being findings**, and the ones that remain are the real
ones — broken pointers, private-helper reaches and file paths. §5.3's run reports
`115 schema-ref-unresolved + 5 schema-ref-broken-internal`, split `68 cross-document fragment not
#/$defs/ · 45 relative file path · 5 internal pointer lands on nothing · 2 target $id no catalog scope
declares`. (A peer split the broken-pointer case into its own code `schema-ref-broken-internal` after
this work-package landed the rule — see §7 and open question 3, which it answers.)

### 2.3 Row 78 — the leaf `$id` grammar, derived and never restated

`mutationLeafSchemaId(repoRoot, rootModuleId, semanticKind)` returns
`<root module $id scope path>/<mutationScopeSegment>/<semanticKind>/<mutationLeafFacetFilename>` —
both segment and filename read from `taxonomy.schemaExportResolution` (`"mutation"`, `"schema.json"`),
neither written into the harness. Checked against the tree:
`✏️s/🔌️plugins/🗄️stdio/…/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🩻️node-skin/🔗️bind/🧬️schema/🔣️.json`
carries `$id …/s/stdio/gltf/2.0/any/mutation/bind-node-skin/schema.json`, and the sibling subset's
aggregate references exactly that string.

Tests (`🧬️ a mutation leaf's own $id`, driven by the new `mutationLeafIdCases` collection):

| case | asserts |
|---|---|
| `single-segment-leaf` | `🎛️sampler/🌱️create` → `…/mutation/create-sampler/schema.json` |
| `two-segment-leaf-directory-is-keyed-by-its-semantic-kind` | `🩻️node-skin/🔗️bind` → `…/mutation/bind-node-skin/schema.json` — the glTF two-segment shape, keyed by `semanticKind`, not by the directory |
| `a-facet-filename-other-than-artifact-does-not-deepen-the-scope` | a `schema.json`-facet root gives the same leaf ids an `artifact.json` root would |
| `the semantic kind keys the leaf, so a two-segment leaf directory still gets one id` | the three ids are distinct and each ends `/<semanticKind>/schema.json` |
| `a facet filename never deepens the scope a leaf hangs off` | `snapshot.json` and `schema.json` roots yield the identical leaf id |

The harness does **not** emit a repo-wide leaf-`$id` diagnostic: `mutation-leaf-id-grammar` is W2c's
`schema check` (row 78, dispatched to W2t). What the harness owns is the derivation, so the two
implementations can be held against one vector rather than one of them owning a private rule.

### 2.4 Row 43 — the catalog `exports` object shape

W1b had already rewritten the reader for `exports: {ExportId: {file, facet}}` **with no dual shape**;
this work-package verified that on disk, extended it (`scopeNormativeDocuments`, the `$id` index) and
finished the row on the harness's own side by migrating the **protocol schema**, which still declared the
array:

`🧪️test/🧬️schema/🔣️.json` → `$defs.SchemaCatalogScope.exports` is now an object with
`propertyNames: SchemaExportId` and `{file, facet}` values, both required. `facet` is documented as a
label; the path comes from `file` alone, nested facet directories included (W1b open question 1 — the
regenerated catalog confirms the label form, see §6.1).

Also removed: the last two `"🔣️jsonschema"` literals in the harness (`schemaExportCompletenessDiagnostics`
and the `$id` index now call `normativeSchemaFormat(repoRoot)`), so the normative format is read from the
taxonomy in every call site.

---

## 3. Files changed

| File | Change |
|---|---|
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts` | rows 56/77/78: `SCHEMA_DIAGNOSTIC_CODES` +2; `declaredSchemaFormatIds`, `SCHEMA_EXPORT_FORMATS_KEYWORD`, `restrictedExportFormats`; `schemaExportCompletenessDiagnostics` rewritten around the annotation; `mutationLeafSchemaId`, `scopeNormativeDocuments`, `jsonPointerTarget`, `schemaRefDiagnostics` added and `schemaResolutionDiagnostics` rebuilt on them; two format literals replaced by `normativeSchemaFormat` |
| `…/🧪️test/🧬️schema/🔣️.json` | `SchemaDiagnosticCode` +2; `SchemaCatalogScope.exports` array → object (row 43); `SchemaInvariantCases` gains `mutationLeafIdCases` (required) and 7 new `catalogCases` keys |
| `…/🧪️test/🧪️tests/🧬️schema-invariants/🔣️.json` | +9 `catalogCases`, +3 `mutationLeafIdCases` |
| `…/🧪️test/🧪️tests/🧬️schema-invariants/🟦️.ts` | `catalogRepo` materialises the new options (helper `definitions`, aggregate + leaf scope, `x-semio-formats`); `writeSchemaModule` takes document-level extras; new `🧬️ a mutation leaf's own $id` describe; collection-count test extended |
| `<ticket>/wp1c-shadow-catalog.py` | ticket input: projects a catalog into the row-43 shape and mounts it over a symlinked shadow repository root |
| `<ticket>/wp1c-shadow-probe.ts` | ticket input: runs the catalog rules against a root, dumps findings + per-owner counts |

No file outside the partition was touched.

---

## 4. Rows 21, 32, 44 — verification on disk

**Row 21 (nothing writes a retired schema filename).** Re-confirmed after W1b:

```
$ grep -rn "schema.json" --include=*.ts 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/ \
    | grep -E '🔣️\.schema\.json|🧬️\.schema\.json|📋️\.schema\.json|🧬️schema\.json'
🧪️tests/🧬️schema-invariants/🟦️.ts:264   ← an example path handed to isFixtureOwnedPath
📦️packages/🟦️typescript/🟦️.ts:4014     ← a docstring naming what the taxonomy forbids
```

Two occurrences, both describing a violation rather than producing one; `payloadSchemaCommand` still has
no `--write` and reads only `mutationPayloadSchemaLocation`. **Done.**

**Row 44 (test-protocol `$id`).** Both halves are on disk:

```
$ python3 -c "import json;print(json.load(open('🧪️test/🧬️schema/🔣️.json'))['\$id'])"
https://semio.tech/schema/repo/test/schema.json

$ grep -n 'semio.tech/schema/repo/test' 📚️library/🧪️tests/🗺️testing-readme-coordinates/🟦️.ts
127:  expect(JSON.parse(schemaBytes.toString("utf8")).$id).toBe("https://semio.tech/schema/repo/test/schema.json");
```

W2c applied R-5. The catalog now also carries the scope (W1b's R-8):
`repo.test → 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema`, level `product-module`.
**Done.**

**Row 32.** W1b's triage stands; §5 below is the re-measurement after this work-package, with the
classification of every remaining red.

---

## 5. Verification — real output

### 5.1 Type check

```
$ bunx tsc -p tsconfig.json --noEmit          # …/🧪️test/📦️packages/🟦️typescript
🟦️.ts(6875,225): error TS2339: Property 'oracle' does not exist on type 'Readonly<{ capability: string; … }>'.
🟦️.ts(6875,275): error TS2339: Property 'oracle' does not exist on type 'Readonly<{ capability: string; … }>'.
```

The same two pre-existing `OracleRequirement.oracle` errors W1 and W1b both recorded; everything this
work-package added type-checks clean. (The command also prints ~380 lines of errors from files reached
only through `../` imports — `ImportMeta.dir`, kernel test files — which are other partitions and outside
this package's `include`.)

### 5.2 The invariants suite

```
$ bun test --timeout 60000 ./🧪️tests/🧬️schema-invariants/🟦️.ts    # cwd …/🔨️modules/🧪️test
 90 pass
 1 fail
 356 expect() calls
Ran 91 tests across 1 file. [17.82s]
```

61 tests (W1) → 76 after this work-package's cases → 91 after a peer extended the same suite (§7). Every
case this work-package added passes, including all five false-positive guards. The single failure is
`🤝️ parity with the catalog generator's own vector > every generator case places the same files and
levels as this harness does`, on generator case `fixture-defines-schema`:

```
Expected: "fixture-defines-schema:<hub fixtures>/🧬️.schema.json"
Received: "fixture-defines-schema:<hub fixtures>/🧬️.schema.json,<hub fixtures>/🧬️schema/🔣️.json"
        (<hub fixtures> = 🌎️hub/💡️inference/🧪️fixtures/🧫️approval)
```

This is **W1b's request R-4, narrowed but still open**. The case was renamed and now expects the retired
filename as a finding — so the blanket fixture exemption is gone — but it still expects the schema MODULE
inside the same fixture tree to be silent, which contradicts execution contract §C ("A schema document
inside a `🧪️*`/`🧫️*` tree is a finding unless the enclosing case declares `inertSchemaData`. No
blanket fixture exemption") and would leave half of master-plan seed 1 unguarded. Writing a translation
here would be the adapter this ticket exists to remove. Coordinator call.

### 5.3 `test schema` over the live tree

```
$ bun 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts test schema --json \
      > <ticket>/🗑️generated/wp1c-test-schema.json
exit 1

findings 5165   schema-bound fixtures 0
  2891 x schema-export-incomplete
  1696 x schema-export-parser-missing      <- the peer's parse<Export>() rule (§7), not this work-package
   162 x schema-dialect-not-draft-07
   155 x schema-fixture-defines-schema
   115 x schema-ref-unresolved
    69 x schema-owner-ineligible
    51 x schema-placement-outside-module
    18 x schema-placement-forbidden-filename
     5 x schema-ref-broken-internal
     3 x schema-file-missing
```

**Read against W1b (3 386 findings, 2 583 of them `schema-catalog-malformed`).** That row is now **zero**:
the tooling worker regenerated the catalog into the row-43 shape mid-session, so resolution, completeness
and cross-scope `$ref` checking run for the first time since W1's measurement — the codes that read 0
under W1b are measured again, not absent. Against W1's 4 111 (the last full measurement of these rules):

| code | W1 | now | why |
|---|---|---|---|
| `schema-export-unknown` | 1 377 | **0** | row 43: the catalog names each export's carrier file, so a facet-child export is looked up in the document that declares it |
| `schema-ref-unresolved` | 506 | 115 (+5 `schema-ref-broken-internal`) | row 77: 541 internal `#/definitions/` refs and 15 aggregate-branch `$id` refs stop being findings; the rest are real |
| `schema-export-incomplete` | 1 006 | 2 891 | the completeness check now reaches every export of all 3 011 scopes; before, 1 341 of them could not be located at all |
| `schema-dialect-not-draft-07` | 287 | 162 | other partitions migrating |
| `schema-fixture-defines-schema` | 254 | 155 | other partitions migrating |
| `schema-placement-forbidden-filename` | 158 | 18 | other partitions migrating |
| `schema-contracts-directory-forbidden` | 1 | 0 | the ShellHost per-contract directory is gone (master-plan seed 2) |
| `schema-owner-ineligible` | 460 | 69 | W2c's `schemaScopeOwnerLevels` restructure |

Per partition owner (full rows in `🗑️generated/wp1c-test-schema.json`; code names abbreviated by
dropping the `schema-` prefix):

| owner root | findings | by code |
|---|---|---|
| `✏️s/🔌️plugins/🗄️stdio` | 1243 | export-parser-missing=570 export-incomplete=521 dialect-not-draft-07=64 ref-unresolved=46 owner-ineligible=38 placement-outside-module=2 ref-broken-internal=2 |
| `✏️s/🔌️plugins/🏛️architect` | 713 | export-incomplete=566 export-parser-missing=142 dialect-not-draft-07=5 |
| `🧰️framework/🛍️products/💻️os` | 454 | export-incomplete=275 fixture-defines-schema=79 export-parser-missing=68 placement-outside-module=21 owner-ineligible=7 dialect-not-draft-07=4 |
| `✏️s/🔌️plugins/📕️norm` | 375 | export-incomplete=203 export-parser-missing=138 dialect-not-draft-07=16 owner-ineligible=15 ref-broken-internal=3 |
| `✏️s/🔌️plugins/🧩️puzzle` | 258 | export-incomplete=131 export-parser-missing=123 dialect-not-draft-07=4 |
| `✏️s/🔌️plugins/🌊️flow` | 217 | export-incomplete=195 export-parser-missing=21 dialect-not-draft-07=1 |
| `✏️s/🔌️plugins/🧱️block` | 201 | export-incomplete=109 export-parser-missing=85 dialect-not-draft-07=6 owner-ineligible=1 |
| `✏️s/🔌️plugins/🏗️fem` | 162 | export-incomplete=96 export-parser-missing=58 dialect-not-draft-07=8 |
| `✏️s/🔌️plugins/📸️remodel` | 137 | export-incomplete=75 export-parser-missing=59 ref-unresolved=2 dialect-not-draft-07=1 |
| `🧰️framework/🔨️modules` | 105 | ref-unresolved=67 export-incomplete=32 export-parser-missing=2 placement-outside-module=1 placement-forbidden-filename=1 owner-ineligible=1 fixture-defines-schema=1 |
| `🧰️framework/🛍️products/🦑️repo` | 105 | fixture-defines-schema=72 placement-forbidden-filename=16 placement-outside-module=8 dialect-not-draft-07=8 owner-ineligible=1 |
| `✏️s/🔌️plugins/🔱️trinity` | 98 | export-incomplete=51 export-parser-missing=43 dialect-not-draft-07=4 |
| `✏️s/🔌️plugins/🌀️procedural` | 94 | export-incomplete=52 export-parser-missing=38 dialect-not-draft-07=4 |
| `✏️s/🔌️plugins/🌍️gis` | 83 | export-incomplete=42 export-parser-missing=26 dialect-not-draft-07=8 file-missing=3 owner-ineligible=2 fixture-defines-schema=2 |
| `✏️s/🔌️plugins/📏️layout` | 77 | export-incomplete=61 export-parser-missing=14 dialect-not-draft-07=2 |
| `✏️s/🔌️plugins/🎥️shooting` | 73 | export-incomplete=42 export-parser-missing=30 dialect-not-draft-07=1 |
| `✏️s/🔌️plugins/📐️cad` | 64 | export-incomplete=42 export-parser-missing=21 dialect-not-draft-07=1 |
| `✏️s/🔌️plugins/💠️lowpoly` | 64 | export-incomplete=34 export-parser-missing=28 dialect-not-draft-07=2 |
| `✏️s/🔌️plugins/🪐️space` | 63 | export-incomplete=52 export-parser-missing=9 owner-ineligible=2 |
| `✏️s/🔌️plugins/🎬️sequence` | 49 | export-incomplete=28 export-parser-missing=17 owner-ineligible=2 placement-outside-module=1 dialect-not-draft-07=1 |
| `✏️s/🔌️plugins/🏭️process` | 49 | export-incomplete=31 export-parser-missing=17 dialect-not-draft-07=1 |
| `✏️s/🔌️plugins/📜️imperative` | 41 | export-incomplete=23 export-parser-missing=17 dialect-not-draft-07=1 |
| `🌎️hub/💡️inference` | 39 | export-incomplete=39 |
| `✏️s/🔌️plugins/🪵️sourcing` | 38 | export-parser-missing=21 export-incomplete=14 dialect-not-draft-07=2 placement-outside-module=1 |
| `✏️s/🔌️plugins/🖨️raster` | 35 | export-incomplete=18 export-parser-missing=15 dialect-not-draft-07=2 |
| `✏️s/🔌️plugins/📖️playbook` | 34 | export-incomplete=21 export-parser-missing=12 dialect-not-draft-07=1 |
| `✏️s/🔌️plugins/🕸️dag` | 33 | export-parser-missing=18 export-incomplete=12 dialect-not-draft-07=2 placement-outside-module=1 |
| `✏️s/🔌️plugins/✒️writer` | 31 | export-incomplete=16 export-parser-missing=13 placement-outside-module=1 dialect-not-draft-07=1 |
| `✏️s/🔌️plugins/➗️mathematical` | 28 | export-incomplete=14 export-parser-missing=12 placement-outside-module=1 dialect-not-draft-07=1 |
| `✏️s/🔌️plugins/🎞️animate` | 26 | export-parser-missing=13 export-incomplete=11 placement-outside-module=1 dialect-not-draft-07=1 |
| `✏️s/🔌️plugins/🔋️energy` | 26 | export-incomplete=13 export-parser-missing=10 placement-outside-module=1 fixture-defines-schema=1 dialect-not-draft-07=1 |
| `🌎️hub/🗿️artifact-authority` | 26 | export-incomplete=26 |
| `✏️s/🔌️plugins/🎪️demonstrator` | 22 | export-incomplete=12 export-parser-missing=9 dialect-not-draft-07=1 |
| `✏️s/🔌️plugins/🗒️note` | 21 | export-parser-missing=14 export-incomplete=4 dialect-not-draft-07=2 placement-outside-module=1 |
| `✏️s/🔌️plugins/📋️forms` | 20 | export-incomplete=9 export-parser-missing=8 dialect-not-draft-07=2 placement-outside-module=1 |
| `✏️s/🔌️plugins/🖍️draw` | 20 | export-parser-missing=10 export-incomplete=7 dialect-not-draft-07=2 placement-outside-module=1 |
| `✏️s/🔌️plugins/🌿️vcs` | 15 | export-parser-missing=8 export-incomplete=5 placement-outside-module=1 dialect-not-draft-07=1 |
| `✏️s/🔌️plugins/💡️reasoning` | 15 | export-parser-missing=7 export-incomplete=6 placement-outside-module=1 dialect-not-draft-07=1 |
| `♻️mit-bestand/🔎️recherche` | 8 | placement-outside-module=7 placement-forbidden-filename=1 |
| `🌎️hub/🔐️auth` | 3 | export-incomplete=3 |

The repository test module's own subtree carries **0** findings (asserted as an empty list by
`🩹️ the test platform's own subtree carries no schema-contract finding`, green).

`0/0 schema-bound fixture(s)` — nothing in the tree declares a `schemaFixtures` block yet; that is WP5
work in the fixture partitions (W1 §6.5), and the mechanism stays covered by the 8 synthetic pipeline
cases.

### 5.4 The shadow-catalog probe (row 43, independent projection)

Written before the regeneration landed, to answer the row-43 question the live run could not: it projects
the catalog into the new shape by locating, for every catalogued export, the document that actually
declares it, and mounts that over a repository root whose every other entry is a symlink to the live
tree. It measured 3 011 scopes / 11 308 exports, of which **1 349 are declared in a facet child** and
**0 nowhere**, and resolved e.g.

```
[DEBUG] schema://s.stdio.gltf/GltfDiffTextDocument
        → ✏️s/…/🪆️subsets/♾️any/🧬️schema/🔺️diff/📝️text/🔣️.json 543e651cdb12
```

— a *nested* facet child, the case that made W1's `schema://` unusable for 1 341 exports. The scripts are
kept as ticket inputs; the numbers now corroborate the generator rather than substitute for it (its
independent projection and the regenerated catalog agree on the shape, and their finding counts differ
only where the generator's `file` differs from mine: `export-incomplete` 2 925 vs 2 841,
`ref-unresolved` 128 vs 128).

### 5.5 The partition's full `bun test`

<!-- WP1C_FULL_RUN -->

---

## 6. A peer extended these rules inside this partition, mid-session

Between the first and second full `bun test` of §5.5 another session edited both of this partition's
schema files — the same pattern W1b recorded (ticket row 62). Everything this work-package landed
survived, verified by reading the current files rather than by trusting the diff:

| what the peer added | relation to this work-package |
|---|---|
| `SCHEMA_DIAGNOSTIC_CODE_TABLE` (code → one-line description; `SCHEMA_DIAGNOSTIC_CODES` derived from it) | both codes added here are in it, with the descriptions the rules imply |
| `schema-ref-broken-internal`, split out of `schema-ref-unresolved` | **answers this work-package's open question 3.** The addressing refusals and the broken-pointer defect now have separate codes |
| `GRAPHQL_EXPORT_KEYWORDS` + five vector cases | answers W1's open question 2 (the six-keyword vocabulary stands, and is now declared once) |
| `declaresSchemaExportParser` + `schema-export-parser-missing` + two vector cases | lands W1's open question 3 (`parse<Export>()`); 1 696 live findings, all in other partitions |
| vector case `export-annotation-that-omits-the-normative-format` | **answers this work-package's open question 1**, and confirms the implementation: an annotation that omits `🔣️jsonschema` while the export is defined there is `schema-export-format-undeclared` |
| vector case `a-format-the-annotation-restricted-away-is-never-asked-for-a-parser` | the parser rule is subordinate to `x-semio-formats`: a format the annotation restricted away is never asked for a parser |

Per `CLAUDE.md` nothing of theirs was reverted and none of their work was chased. The consequence for
this report is that §5.3's counts include their rule (`schema-export-parser-missing`, 1 696) and their
code split (`schema-ref-broken-internal`, 5) — both are marked as theirs in the table above and in §5.3.

## 7. Cross-partition requests

| id | To | Request |
|---|---|---|
| **R-1** | W2c (catalog generator) | `facet` labels are inconsistent: 11 297 exports use an ASCII label (`"diff/text"`) and **11 use the emoji directory chain** (`"📸️snapshot/📝️text"`, `"🔺️diff/📝️text"`, `"🔺️diff"`, `"🔺️diff/💾️binary"`, `"📸️snapshot/💾️binary"`). Harmless here — this harness validates `facet` as a non-empty label and resolves strictly from `file` — but a consumer that decodes the label will break on those 11. Pick one form. |
| **R-2** | coordinator | **Diagnostic-code vocabulary.** The two implementations of one rule set name their findings differently (`export-format-missing` vs `schema-export-incomplete`, `ref-not-export-addressed` vs `schema-ref-unresolved`, `mutation-leaf-id-grammar` vs the harness's derivation helper). The parity test can therefore compare only the code-free halves. Decide whose vocabulary is canonical and the parity test can compare codes too — which is the only way a rule change in one implementation is caught by the other. |
| **R-3** | W2c (`📚️library/🧪️tests/🧬️schema-scope-catalog/🧫️fixtures/🔣️.json`) | **W1b's R-4, still open and still the only red in the invariants suite.** Case `fixture-owned-schema` expects `placementPaths: []` for a schema inside `🌎️hub/💡️inference/🧪️fixtures/`, contradicting contract §C and retiring master-plan seed 1. Either expect the two paths, or have the case declare `inertSchemaData`. |
| **R-4** | W6c (plugins) | Row 55's `x-semio-formats` annotations can land now: the rule is implemented and tested on both sides, and today it is inert because no export carries the keyword. Annotate the law/contract exports rather than letting them sit in the 2 841 `schema-export-incomplete`. |
| **R-5** | W2c (taxonomy) + coordinator | W1b's R-1/R-2/R-3 are unchanged and still red in the test-platform suite (`taxonomy.areas` has no `exempt` entry; `testContributionDirectoryOverrides` pruned 176 → 2; the root `📜️script.ts` still contains the literal `taxonomy.testDomainPath`). None is reachable from anything WP1c touched. |
| **R-6** | the peer refactoring this partition (row 62) | `📦️packages/🟦️typescript/tsconfig.json` still `include`s only `🟦️.ts`, so the three relocated suites are not type-checked by the package's `lint` target (W1b R-6, unchanged). The removal of `loadMigrationBaseline`/`surveyUnmanagedTests` is complete on both sides — the suites load, and nothing was re-added. |

---

## 8. Open questions

1. ~~Should an annotated export be allowed to omit the normative format?~~ **Answered** by the peer's
   vector case `export-annotation-that-omits-the-normative-format` (§6): it may not — an annotation that
   omits `🔣️jsonschema` while the export is defined there is `schema-export-format-undeclared`, which
   is what this work-package implemented.
2. **`dependsOn` for mutation aggregates.** With row 77's aggregate-branch form legal, every aggregate
   that references its leaves is a cross-scope reference and needs each leaf scope in its `dependsOn`.
   The regenerated catalog derives `dependsOn` for 135 scopes; once the generator catalogues aggregates
   and leaves as scopes (row 11), that number has to grow with them or the aggregates will report
   `schema-cross-scope-dependency-forbidden` in bulk.
3. ~~Broken internal pointers under their own code?~~ **Answered** by the peer (§6):
   `schema-ref-broken-internal`, 5 live findings, all in other partitions
   (`📕️norm` 3, `🗄️stdio` 2).
4. ~~W1's GraphQL vocabulary and `parse<Export>()` questions~~ — both **landed by the peer** (§6). The
   parser rule's 1 696 findings are now the largest single code after `schema-export-incomplete`, and
   they are entirely other partitions' work; the coordinator may want to route them as one request.
5. **Still open, and the only rule question this work-package leaves:** whether an export the annotation
   restricts away should still be required to EXIST in the formats it named when the scope provides no
   file for one of them. Today that is `schema-export-incomplete` with a `format` and no `path`; the
   alternative is a distinct code for "declared a format its scope does not implement".
