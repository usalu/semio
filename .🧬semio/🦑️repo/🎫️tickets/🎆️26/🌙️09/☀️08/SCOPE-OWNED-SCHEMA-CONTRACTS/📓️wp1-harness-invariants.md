# WP1 — Harness invariants, `schema://` binding, staged fixtures

Worker: W1 (Opus). Partition: `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/**`.
Baseline commit at start: `0a0bb74380`. Work landed by auto-commit from `6152f9ca6a` onward.

## 1. What changed

| File | Change |
|---|---|
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts` | +3 new regions (`🧬️SchemaContracts`, `🧬️SchemaValidation`, `🧫️SchemaFixtures`), rewritten `🧬️PayloadSchema` region, `schema://` in `fixtureUrisIn`/`resolveFixtures`, single payload-schema convention in `scaffoldLeafDescriptor`. 6 040 → 6 479 lines. |
| `…/🧪️test/📦️packages/🟦️typescript/🧬️invariants.test.ts` | **new** — 61 tests, the WP1 suite. |
| `…/🧪️test/🧪️tests/🧬️schema-invariants/🔣️.json` | **new** — the language-agnostic vector (eligibility, tree, resolution, catalog, validation, pipeline cases). |
| `…/🧪️test/🧬️schema/🔣️.json` | dialect 2020-12 → draft-07; 56 → 73 `$defs` for the schema-contract layer; `FixtureRef` learns the `schema` scheme; 4 new root `oneOf` branches. |
| `…/🧪️test/🧪️tests/🧭️contribution-directory-ownership/🧬️schema/🔣️.json` | **deleted** — a case directory may not define the contract it is an example of. |
| `…/🧪️test/🧪️tests/🧭️contribution-directory-ownership/🔣️.json` | now names the owning module's schema in `$schema`. |
| `…/🧪️test/📦️packages/🟦️typescript/🧪️index.test.ts` | rebound to `protocol#/$defs/ContributionDirectoryOwnershipCases` instead of the deleted fixture-local schema. |
| `…/🧪️test/📜️script.ts` | `test schema` / `schema` entry point (`SchemaScript`, `TestScript`); `manifest payload-schema` is a report, no longer a generator. |
| `…/🧪️test/📋️project.json` | new `test-schema` target (`bun ./📜️script.ts test schema`). |
| `…/🧪️test/📦️packages/🟦️typescript/📜️script.ts`, `tsconfig.json` | the package's `test` target runs both suites; `exclude` lists both. |

## 2. Invariants and their tests

All in `🧬️invariants.test.ts`; every row is driven by `🧪️tests/🧬️schema-invariants/🔣️.json`, so a second
implementation runs the same vector.

### 2.1 Owner eligibility — `🏛️ owner eligibility`

- `every declared level and exclusion is decided by position, not by filename` — 18 `eligibilityCases`
  covering all seven declared levels and all four exclusions.
- `a trailing slash never changes a verdict`.
- `the glob vocabulary the taxonomy writes its levels in is honoured exactly` — the `**`/`*` semantics of
  `schemaScopeOwnerLevels.pathPatterns`, plus `isFixtureOwnedPath` on the three cases that matter.

Eligibility is **read from the taxonomy** (`schemaScopeOwnerLevels.levels` + `excludedOwnerPathPatterns`,
landed by W2), never from a table in the harness — a second table would be a second opinion. Diagnostic:
`schema-owner-ineligible`. Collection membership (is this path inside a test/fixture collection?) reads
`schemaScopeOwnerLevels.fixtureOwnerPathPatterns`, with one rule decided here and only here: a
`🔨️modules/<m>` member is a **module**, whatever emoji its name starts with — see §6.1.

### 2.2 Placement, ownership and fixture isolation — `📍️ placement, ownership and fixture isolation`

12 `treeCases`, each materialised into a synthetic repository carrying the real taxonomy:

| Case | Expected |
|---|---|
| `seed-hub-inference-approval-fixture-wrapper` | `schema-placement-forbidden-filename`, `schema-fixture-defines-schema` |
| `seed-shellhost-invite-capability-contract-directory` | `schema-contracts-directory-forbidden`, `schema-placement-forbidden-filename` |
| `valid-canonical-artifact-subset-module` | *(none)* |
| `valid-mutation-leaf-payload-schema` | *(none)* — false positive: a leaf `🧬️schema/🔣️.json` is legal |
| `valid-config-instance-carrying-a-schema-reference` | *(none)* — false positive: `$schema` naming a file is an instance |
| `valid-inert-parser-corpus-declared-as-test-data` | *(none)* — false positive: declared inert corpus |
| `invalid-undeclared-schema-shaped-document-in-a-test-collection` | `schema-placement-outside-module`, `schema-fixture-defines-schema` |
| `valid-repo-test-module-owns-its-protocol-schema` | *(none)* — false positive: `🔨️modules/🧪️test` is a module, see §6.1 |
| `invalid-ui-element-owner` | `schema-owner-ineligible` |
| `invalid-package-owner` | `schema-owner-ineligible` |
| `invalid-relocated-fixture-schema-module` | `schema-fixture-defines-schema` |
| `invalid-2020-12-dialect-inside-a-schema-module` | `schema-dialect-not-draft-07` |

Plus `a configuration instance naming its validating schema is never a definition` — the definition/instance
discriminator: a document DEFINES a schema iff its `$schema` names a `json-schema.org` **meta-schema**. An
instance names the file that validates it (`"../🧬️schema/🔣️.json"`), so every `$schema`-carrying config in the
tree stays out of the findings by rule, not by exception list.

**Inert test data.** Two mechanisms, both explicit: a schema-shaped document embedded as a *string* inside a
case file (the existing `📚️library/…/🧫️fixtures/🍃️artifact-support-leaf-authority/🔣️.json` convention), or a
standalone file named in the case's own `🔣️.json` under `inertSchemaData: [...]`. Nothing is exempted by
shape or by location.

### 2.3 Export resolution — `🔗️ export resolution`

11 `resolutionCases`, one diagnostic code each, all distinct:

`schema-uri-malformed` · `schema-scope-unknown` · `schema-export-unknown` · `schema-format-unavailable` ·
`schema-catalog-missing` · `schema-export-resolution-undeclared` · `schema-scope-ambiguous` (duplicate id in
the catalog text, and two ids claiming one directory) · `schema-fixture-local-schema-fallback` (a catalog row
answering out of a `🧪️*`/`🧫️*` tree) · plus the two positive cases (normative format, consumer-chosen format).

Plus `a resolved export validates its example exactly as ajv does` (four instances, our verdict vs ajv's) and
`a malformed uri is refused before the catalog is even read`.

### 2.4 Catalog completeness and cross-scope dependencies — `📚️ …`

11 `catalogCases`: complete-across-five-formats, a single-export module declaring its export with the
taxonomy's `rootExportKeyword`, recursive `$ref` inside one module (legal),
unresolved local `$ref`, cross-scope `$ref` with and without `dependsOn`, `$ref` by relative file path,
and one `schema-export-incomplete` per format (rust / typescript / graphql / protobuf).

Completeness rule: an export's normative definition is `$defs.<ExportId>` **or**, for a single-export
module, the root document whose `schemaExportResolution.rootExportKeyword` (today `title`) names it — both
are declarations, and 1 587 of the 2 944 catalogued exports use one of them. For every such export, each
*present* format file must declare the same-named entity — proto `message`, GraphQL `type|input|interface|union|enum|scalar`, Rust
`pub struct|enum|type`, TS `export interface|type|const|class`.

### 2.5 Structural validation vs a third-party oracle — `🧬️ structural validation agrees with ajv`

28 `validationCases`. Each is run through **our** draft-07 validator (`validateAgainstJsonSchema`, region
`🧬️SchemaValidation`) *and* through **ajv 8.20.0**, and both must return the declared verdict. Covers type /
integer / enum / const / oneOf / anyOf / allOf / not / required / additionalProperties / patternProperties /
tuple `items` + `additionalItems` / uniqueItems / min-max items-length-properties / pattern / multipleOf /
exclusive bounds / local `$ref` / recursive `$ref` / code-point string length.

We own a validator because the harness may not take a runtime dependency on one; ajv keeps "our own" from
meaning "unverified". (W3 owns the Rust twin, `semio_framework_schema::structural_validator_for` — two native
implementations of one contract is the repo's multi-implementation rule, not duplication.)

### 2.6 Staged fixture pipeline — `🧫️ schema-bound fixtures run through named stages`

8 `pipelineCases` + `every stage is reported, and the ones after a failure are reported as skipped`.

### 2.7 `schema://` in the fixture resolver — `🔗️ the fixture resolver speaks schema://`

Extraction beside the other three schemes, resolution to the catalog file with a digest pin, and
"unresolvable is *missing*, never silently defaulted".

### 2.8 Cross-implementation parity — `🤝️ parity with the catalog generator's own vector`

The catalog generator (`📚️library`, W2) and this harness are two independent implementations of one
invariant set, so they are held against **one vector**: the generator's own eight cases at
`📚️library/🧪️tests/🧬️schema-scope-catalog/🧫️fixtures/🔣️.json`, materialised and run through these
checkers (this answers request #13 in `📋️cross-partition-requests.md`).

Only the **code-free halves** are compared — which paths are misplaced (`expected.placementPaths`) and
which level each eligible owner sits on (`expected.scopes[*].level`) — because the two implementations name
their diagnostics differently, and a translation table between two vocabularies would be exactly the adapter
this ticket exists to remove. Deciding whose code vocabulary is canonical is a coordinator call; until then
this test proves the two agree on everything that is not a name.

It found one real divergence on the first run and it was fixed here: a schema definition inside a
`🧬️contracts/` directory used to be reported twice, once as the forbidden directory and once as
`schema-placement-outside-module` for the file. The directory finding subsumes the file, and now only it
fires.

### 2.9 Live tree — `🩺️ the committed tree is measured by the same checkers`

`the test platform's own subtree carries no schema-contract finding` — **0 findings**, asserted as an empty
list so any regression in this partition names itself.

## 3. Fixture descriptor shape

A schema-bound fixture is a member of a `schemaFixtures` array in a case's own `🔣️.json`. Discovery is by
that **declaration key**, never by shape or by directory scan.

```jsonc
{
  "schemaFixtures": [
    {
      "id": "writer-artifact-minimal",              // Slug
      "uri": "schema://s.writer.writer/Artifact",   // schema://<scope id>/<ExportId>
      "target": { "scope": "s.writer.writer", "export": "Artifact", "format": "🔣️jsonschema" },
      "data": "🔣️subject.json",                    // case-relative; XOR with `inline`
      "inline": { "title": "a" },                   // XOR with `data`
      "expect": { "stage": "structural-validation", "result": "failed", "code": "schema-instance-invalid" }
    }
  ]
}
```

`target` must agree with `uri` (a disagreement fails at `metadata`). `format` is the consumer's choice; the
normative format of a `🧬️data` facet is `🔣️jsonschema`. Declared in
`🧪️test/🧬️schema/🔣️.json` as `$defs.SchemaBoundFixture` / `SchemaFixtureTarget` / `SchemaStageExpectation`,
with `SchemaStageOutcome` / `SchemaFixtureReport` for the output side.

### Pipeline stages (explicit, always all six reported)

| Stage | What it decides | Codes it can emit |
|---|---|---|
| `metadata` | the descriptor names a scope, an export, a format and exactly one subject | `schema-fixture-metadata-invalid` |
| `contract-resolution` | `schema://…` resolves through the catalog | `schema-catalog-missing`, `schema-catalog-malformed`, `schema-export-resolution-undeclared`, `schema-uri-malformed`, `schema-scope-unknown`, `schema-export-unknown`, `schema-format-unavailable`, `schema-file-missing`, `schema-fixture-local-schema-fallback` |
| `parse` | the subject decodes | `schema-fixture-parse-failed` |
| `structural-validation` | the subject satisfies the export's schema | `schema-instance-invalid`, `schema-export-unknown` |
| `domain` | every declared format of the scope implements the export | `schema-export-incomplete`, `schema-file-missing` |
| `assertions` | actual `{stage, result, code}` vs the declared expectation | — |

Execution stops at the first failing stage; the remainder are reported `skipped`, never omitted — a report
listing four stages out of six reads as four passes. A negative fixture that starts failing *earlier* than it
declared is a **failure**, not a pass: it is no longer the proof it was written to be
(`negative-fixture-that-fails-earlier-than-declared-is-not-the-proof-it-claims`).

## 4. Heuristics retired

`📓️wp0-mechanism.md` §5 rows 2–6, all in `…/🧪️test/📦️packages/🟦️typescript/🟦️.ts`:

| # | Mechanism | Old lines | Now |
|---|---|---|---|
| 2 | Dual payload-schema convention (`🧬️schema/<json>` **or** flat `🔣️.schema.json`, first `existsSync` wins) | 3579-3596 | One location, from the taxonomy: `payloadSchemaRelativePath()` reads `mutationPayloadSchemaLocation`; `scaffoldLeafDescriptor` accepts only that path (now ~3592-3597). |
| 3 | `rustTypeIndex` — `walkDirectories(repoRoot, …)` over **every** `.rs` file, indexed by bare name | 3825-3931 | **deleted** |
| 4 | Nearest-owner disambiguation for ambiguous struct names (longest common path prefix) | 4130-4165 | **deleted** |
| 5 | Nearest-owner disambiguation for ambiguous enum names (`nearest()`) | 4102-4119 | **deleted** |
| 6 | Nearest-owner disambiguation inside `resolveSnapshot` | 4028-4060 | **deleted** |

Deleted with them, because they existed only to serve those: `PayloadSchemaDerivation`, `derivePayloadSchema`,
`derivePayloadSchemas`, `rustTypeToJsonSchema`, `transparentRustTypes`, `RUST_SCALARS`, `structFields`,
`camelCase`, `jsonValueShaped`/`jsonValueBodies`, `rustIndexCache` — the whole 547-line `🧬️PayloadSchema`
region.

Replaced by `resolvePayloadSchema` / `resolvePayloadSchemas`: read the leaf's descriptor, take its declared
`payloadSchema`, require it to equal the taxonomy location, read the file, require draft-07. Every failure is
a named refusal; nothing is derived and nothing is searched for.

**No remaining caller needs a repo-wide type scan.** The only consumer of `rustTypeIndex` was
`payloadSchemaCommand` in `🧪️test/📜️script.ts`, which *generated* payload schemas with `--write`. Evidence:

```
$ grep -rn "derivePayloadSchema|rustTypeIndex|transparentRustTypes|rustTypeToJsonSchema|PayloadSchemaDerivation" \
    --include=*.ts --include=*.json --include=*.md .    # excluding node_modules and .🧬semio
(no output)
```

`manifest payload-schema` is now a read-only report of what is declared, and `--write` is gone: a payload
contract is authored by its owner, not reconstructed from whichever same-named struct happened to sit
nearest. Live result:

```
$ bun ./📜️script.ts manifest payload-schema
[manifest payload-schema] 2085/2146 leaves declare a payload contract at the taxonomy location (97.2%)
[manifest payload-schema]       61 × the leaf carries no 🔣️.json descriptor, so nothing declares its payload contract
```

(The 61 undeclared leaves are all under `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/…` — W7's partition.)

## 5. Verification

### 5.1 `test schema` entry point

`bun 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts test schema [--under <path>] [--json]`,
also reachable as `📜️script.ts schema …` and as the nx target `@semio-tech/repo-test-domain:test-schema`.
The invariants suite itself runs under `nx run @semio-tech/repo-test:test`, which now names both test files.
It runs the whole-tree invariant audit and every fixture bound with `schema://`, and exits non-zero on any
finding or any fixture that missed its declared stage.

### 5.2 `schema://` against the live, generated catalog

```
[DEBUG] scopes 460 catalog diagnostics 0 []
[DEBUG] schema://app.animate.presentation/PresentationConfig → ✏️s/🔌️plugins/🎞️animate/…/🎚️config/🧬️schema/🔣️.json 4f12a29083daaac1f96825815b0719dc
[DEBUG] schema://app.animate.presentation/Nope        → REFUSED schema-export-unknown
[DEBUG] schema://not.a.scope/X                        → REFUSED schema-scope-unknown
[DEBUG] schema://Bad/x                                → REFUSED schema-uri-malformed
[DEBUG] …/PresentationConfig (🦀️rust)                 → ✏️s/🔌️plugins/🎞️animate/…/🎚️config/🧬️schema/🦀️.rs
```

### 5.3 Whole-tree audit — `🗑️generated/wp1-schema-audit.txt` / `.json`

```
$ bun 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts test schema
[test schema] 4111 invariant finding(s) over the repository
[test schema]    1377 × schema-export-unknown
[test schema]    1006 × schema-export-incomplete
[test schema]     506 × schema-ref-unresolved
[test schema]     460 × schema-owner-ineligible
[test schema]     287 × schema-dialect-not-draft-07
[test schema]     254 × schema-fixture-defines-schema
[test schema]     158 × schema-placement-forbidden-filename
[test schema]      55 × schema-placement-outside-module
[test schema]       7 × schema-cross-scope-dependency-forbidden
[test schema]       1 × schema-contracts-directory-forbidden
[test schema] 0/0 schema-bound fixture(s) reached their declared stage
exit 1

$ … test schema --under 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test
[test schema] 0 invariant finding(s)
exit 0
```

The total moved 4 655 → 4 332 → 4 224 → 4 111 across four runs an hour apart: the tree is being migrated
under WP3–WP7 while this runs, so treat the number as a snapshot, not a baseline.

**The seeds.** Both were confirmed as violations by these checkers, with exactly the codes their fixture
cases encode:

```
🌎️hub/🧪️fixtures/✅️inference-approval-v1/🧬️.schema.json
    schema-placement-forbidden-filename
    schema-fixture-defines-schema
…/🧱️elements/🏛️ShellHost/🧬️contracts                                     schema-contracts-directory-forbidden
…/🧱️elements/🏛️ShellHost/🧬️contracts/🎟️invite-capability/🧬️.schema.json   schema-placement-forbidden-filename
```

Seed 1 has since been **fixed in the tree by the hub worker** — `🌎️hub/🧪️fixtures/✅️inference-approval-v1/`
now holds only its data `🔣️.json` — and the final audit reports nothing for it. Seed 2 is still there, with
14 findings under `🏛️ShellHost` (one forbidden `🧬️contracts` directory plus 13 retired filenames). Both
remain encoded as fixture cases, so neither can come back unnoticed.

### 5.4 Harness test suite, before and after

Both runs are `bun test` in `…/🧪️test/📦️packages/🟦️typescript`, on the live tree while nine other workers
migrate it, with several of their `bun test` processes running concurrently.

| | `🧪️index.test.ts` | `🧬️invariants.test.ts` |
|---|---|---|
| before (`0a0bb74380`) | 88 pass / 23 fail (750 s), then 87 pass / 24 fail (1 046 s) | — (did not exist) |
| after | **87 pass / 24 fail**, 2 787 expect() calls, 1 170 s | **61 pass / 0 fail**, 231 expect() calls, 3.4 s |

Failure sets compared line by line (`🗑️generated/wp1-index-before-failures.txt` vs
`wp1-index-after-failures.txt`): **24 before, 24 after, one swap.**

- newly failing: `🔍️ discovery and contract > discovery is idempotent` — a bun 5 s per-test timeout, not an
  assertion. Re-run alone on a quiet machine it **passes in 39.37 s**:
  ```
  $ bun test ./🧪️index.test.ts -t "discovery is idempotent"
   1 pass · 110 filtered out · 0 fail   [39.37s]
  ```
- newly passing: `🧭️ contribution directory ownership > root dependency discovery and classification honor
  the same neutral owner contract` — reads the root `📜️script.ts` through the TypeScript compiler; W2 is
  editing that file.

The full `🧪️index.test.ts` "after" run was captured **before** W2's second taxonomy restructure (§9); the
invariants suite and both `test schema` runs above are from after it. Two `🧪️index.test.ts` tests were
spot-checked after the restructure: the one this work-package rebound passes, and
`🔍️ discovery and contract > every committed case satisfies the frozen contract` still fails exactly as it
did in the baseline.

The 23 stable failures are the same before and after and belong to **other partitions**: oracle-registry
shape (`oracle.comparisonProfiles`/`decision.substitutes` undefined), the dependency ratchet, oracle purity,
clean safety, comparison profiles, and the case-above-subset ratchet — all assertions about files W2–W10 are
changing right now. None of them is reachable from anything this work-package touched, and the one test that
*is* mine (`handpicked owner directories match neutral cases and the lodash map oracle`, rebound to the
module export in §1) passes:

```
$ bun test ./🧪️index.test.ts -t "handpicked owner directories match neutral cases and the lodash map oracle"
 1 pass · 110 filtered out · 0 fail · 5 expect() calls   [17.74s]
```

### 5.5 Type check

```
$ bunx tsc -p tsconfig.json --noEmit    # …/🧪️test/📦️packages/🟦️typescript
🟦️.ts(6363,225): error TS2339: Property 'oracle' does not exist on type 'Readonly<{ capability: string; … }>'.
🟦️.ts(6363,275): error TS2339: Property 'oracle' does not exist on type 'Readonly<{ capability: string; … }>'.
```

Two errors, both pre-existing (`OracleRequirement.oracle` in `mutationCoverageBreaches`, untouched by this
work-package). Everything else this WP added type-checks clean. The remaining `TS2307` errors the same
command prints are `🧬️schema.json` imports in `🧰️framework/🔨️modules/**` that W10 is deleting mid-flight —
another partition, not reachable from anything here.

### 5.6 Catalog export/location gap (evidence for §6.3)

Measured directly against the generated catalog and the taxonomy's `schemaChildDirs`:

```
catalog exports total                        2944
  declared in the module root document       1587   ($defs key, or the root `title`)
  declared only in a taxonomy facet child    1006   (🔺️diff, 📸️snapshot, 💡️inferences, 🧬️mutations)
  declared nowhere reachable                  351
```

Example: `app.gis.gis2d` exports `Gis2dConfig` and `Gis2dConfigDiff`; `formats["🔣️jsonschema"]` names only
`🔣️.json`, and `Gis2dConfigDiff` lives in `🔺️diff/🔣️.json`. So `schema://app.gis.gis2d/Gis2dConfigDiff`
resolves to the wrong document today. That is the 1361 `schema-export-unknown` findings in §5.3.

## 6. Cross-partition requests

### 6.1 (W2, taxonomy) `fixtureOwnerPathPatterns` vs a module named `🧪️test` — RESOLVED, one rule left here

Raised earlier in this work-package: `excludedOwnerPathPatterns` contained `**/🧪️*`, which made the
repository test platform — a product module at `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test` that owns
the test-protocol contract every native host, every probe and `taxonomy.testSchemaLocation` point at —
ineligible to own its own schema. **W2 has since removed the `🧪️`/`🧫️` entries from
`excludedOwnerPathPatterns` and moved them to a new `fixtureOwnerPathPatterns`.** This harness follows the
new declaration and the module's subtree is now clean (§5.3).

Two notes on the new key, both acted on here:

1. **`**/🧪️*` still matches `🔨️modules/🧪️test`.** `fixtureOwnerPathPatterns` decides "is this a test or
   fixture COLLECTION", and by the letter of the pattern the test module is one. The harness applies the
   same carve-out it always did — a `🔨️modules/<m>` member is a module, whatever emoji its name starts with
   (`isFixtureOwnedPath`, one function, documented at the call site). If W2 would rather state that in the
   taxonomy, the shape that removes the rule from here is:
   ```jsonc
   "fixtureOwnerPathPatterns": ["**/🧪️*", "**/🧪️*/**", "**/🧫️*", "**/🧫️*/**"],
   "fixtureOwnerPathExceptions": ["**/🔨️modules/*"]
   ```
2. **`fixtureOwnerReason` says a fixture tree's schema modules are "fixture inputs, not findings".** This
   harness disagrees, deliberately: `schema-fixture-defines-schema` inside a fixture tree is the WP1
   fixture-isolation invariant and it is exactly master-plan **seed 1**. Silencing it would mean seed 1 was
   never a violation. What the harness does honour is an explicit per-case declaration — `inertSchemaData`
   in the case's own `🔣️.json`, §2.2 — which is how the generator's *own* corpus stays out of the findings
   without a blanket rule. **Coordinator call needed**: if `fixtureOwnerReason` is meant literally, seed 1
   drops out of the WP1 gate and §5.3's 254 `schema-fixture-defines-schema` findings go to zero.

Note also that removing `**/🧪️*` from `excludedOwnerPathPatterns` made two paths *eligible* that read
oddly: `🌎️hub/🧪️fixtures/✅️inference-approval-v1` now matches `hub-area-module` (`🌎️hub/**`), and
`…/🧪️test/🧪️tests/<case>` matches `product-module`. The fixture-isolation invariant still catches a schema
defined in either, so nothing escapes — but if the intent is that a collection can never be an owner, the
`fixtureOwnerPathPatterns` need to feed eligibility too (with exception 1 above, or the test module goes
ownerless again).

### 6.2 (W2, taxonomy) `$id` of the test-protocol schema

Contract §B says `$id` is `https://semio.tech/schema/<scope path>/<facet>.json`. `🧪️test/🧬️schema/🔣️.json`
still carries `$id: "https://semio-tech.com/schema/repo/test/v2"` and I did **not** change it, because
`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🗺️testing-readme-coordinates/🟦️.ts:127` asserts
that exact string and that file is outside my partition. Requested, as one change:

- `🧪️test/🧬️schema/🔣️.json` → `"$id": "https://semio.tech/schema/repo/test/protocol.json"` (mine to apply
  once the assertion moves)
- `📚️library/🧪️tests/🗺️testing-readme-coordinates/🟦️.ts:127` → expect the new id

Also unsettled by the contract: the `<facet>.json` filename for a **non-artifact** module. The canonical
example uses `artifact.json`; `hub.inference` would be `inference.json`. I used `protocol.json` above as a
placeholder — W2 should state the rule (`schemaExportResolution.idFacetFilenamePattern` accepts any
kebab-case name, so it needs a convention, not a pattern).

### 6.3 (W2) Catalog fields this harness reads

Already satisfied by the generated catalog — recorded so a future regeneration does not drop them:

- `scopes.<id>.path` — the **module** directory (`…/🧬️schema`).
- `scopes.<id>.formats` — format id → filename **relative to `path`**.
- `scopes.<id>.exports` — PascalCase ids; resolved against the module's `$defs` **or**, for a single-export
  module, its root `schemaExportResolution.rootExportKeyword` (`title`).
- `scopes.<id>.dependsOn` — scope ids; a cross-scope `$ref` not listed here is
  `schema-cross-scope-dependency-forbidden`.
- `scopes.<id>.hashes` — read only for provenance, not required.
- `schemaExportResolution.uriScheme` (`"schema"`), `.catalogPath`, `.uriPattern` (used verbatim when present;
  named groups `scope`/`export` or positional groups 1/2).

**Missing, and blocking `schema://` for 1 357 of 2 944 exports (§5.6): the catalog does not say WHICH FILE
carries each export.** `formats` is per-scope, but 1 006 exports live in a taxonomy facet child
(`🔺️diff/🔣️.json`, `📸️snapshot/🔣️.json`, `💡️inferences/🔣️.json`, `🧬️mutations/🔣️.json`) rather than in the
module root, and 351 are declared nowhere at all. `schema://<scope>/<Export>` therefore resolves to the
*wrong document* for the first group and to a document that does not declare the export for the second.

The consumer must not fix this by searching the four facet children — that is the nearest-parent habit in
new clothes. Requested shape (either works here; I will take whichever lands):

```jsonc
// per-export, in the catalog row
"exports": [
  { "id": "Gis2dConfig",     "formats": { "🔣️jsonschema": "🔣️.json",        "🦀️rust": "🦀️.rs" } },
  { "id": "Gis2dConfigDiff", "formats": { "🔣️jsonschema": "🔺️diff/🔣️.json", "🦀️rust": "🔺️diff/🦀️.rs" } }
]
// or, keeping today's flat list, a sibling map
"exportFacets": { "Gis2dConfig": "", "Gis2dConfigDiff": "🔺️diff" }
```

Also: the catalog should carry scopes whose `🔣️jsonschema` file is missing rather than omitting them,
otherwise `schema-file-missing` can never fire for a scope that exists on disk but failed generation.

### 6.3b (W2) Answer to request #21 — nothing writes `🔣️.schema.json` any more

```
$ grep -rn '🔣️.schema.json' --include=*.ts 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/
(no output)
```

`payloadSchemaCommand` no longer takes `--write` and the literal is gone from the harness entirely; the only
path it now reads is the taxonomy's `mutationPayloadSchemaLocation`.

### 6.4 (W7, `✏️s/🔌️plugins/🏛️architect/…`) 61 mutation leaves declare no payload contract

`bun 🧪️test/📜️script.ts manifest payload-schema` lists them; every one is under
`✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/`.
They previously "had" a schema only because the harness reconstructed one from the nearest same-named Rust
struct; that reconstruction is gone, so they now show up as what they are.

### 6.5 (WP5 follow-up) No committed `schema://` fixture yet

`test schema` reports `0/0` bound fixtures: nothing in the tree declares a `schemaFixtures` block yet. The
mechanism is exercised by 8 synthetic pipeline cases and by the live resolution probe in §5.2. Rebinding real
fixtures is WP5 work in the fixture/plugin partitions; the descriptor shape they need is §3.

## 7. Open questions

1. **Where does a fixture's `schemaFixtures` block live for an artifact subset?** I put discovery on the
   case's own `🔣️.json` (the file kind `testContributionFileKindId` already names). If WP5 wants fixtures
   declared in the *fixture manifest* instead, the discovery key moves; the descriptor shape does not.
2. **GraphQL export vocabulary.** Contract §A says "the same-named `type` (GraphQL)". A `$defs` entry that is
   an enum or a union naturally becomes `enum`/`union`, so the completeness check accepts
   `type|input|interface|union|enum|scalar`. If the contract means literally `type`, say so and I will
   tighten it — it will then report every enum export as incomplete.
3. **TS `parse<Export>()`.** Contract §A also requires an exported `parse<Export>` function per export. The
   completeness check currently verifies the exported *type* only; adding the function check will produce a
   large number of new findings across every partition at once, so I left it out of WP1 deliberately. Say the
   word and it is three lines.
4. **`dependsOn` direction.** I treat a `$ref` into another scope as requiring that scope in the referring
   scope's `dependsOn`. The generator currently emits `dependsOn: []` for most scopes, so once WP3/WP4 start
   adding real cross-scope `$ref`s, `schema generate` must populate it or every one of them will report
   `schema-cross-scope-dependency-forbidden`.

## 8. Deviations from the brief

- **`bun:test`, not vitest.** The sibling suite (`🧪️index.test.ts`) is `bun:test` and the package's `test`
  target runs `bun test ./🧪️index.test.ts`; this package has no vite config and vitest would not be picked
  up. `🧬️invariants.test.ts` uses `bun:test` for the same reason, and `📜️script.ts` runs both.
- **Owner-eligibility rules are read from the taxonomy, not hard-coded from contract §A.** Implementing §A
  separately would have created the second opinion this ticket exists to remove. The one rule this harness
  still decides on its own is the `🔨️modules/<m>` carve-out for collection membership (§6.1).
- **The `.nx/` and other dot-directories are skipped by the tree walk.** `.nx/cache` alone carried six
  generated copies of one violating file, reporting the same defect once per cache entry.

## 9. Note on a moving target

`🔣️taxonomy.json` and `🔣️schema-catalog.json` are W2's and both changed **twice while this work-package
was being written**: the catalog appeared mid-session (so the `schema://` resolver was rewritten from a
guessed shape to the generated one — module-relative `formats`, `rootExportKeyword`), and
`schemaScopeOwnerLevels` was restructured an hour later (`🧪️`/`🧫️` moved out of `excludedOwnerPathPatterns`
into `fixtureOwnerPathPatterns`). Both times the fixture vector caught the drift immediately, because the
harness reads the declaration rather than restating it — which is the whole argument for doing it that way.
Anyone re-reading this report should re-run `test schema` rather than trust the counts in §5.3.

## 10. Artifacts

In `<ticket>/🗑️generated/`:

- `wp1-schema-audit.txt` / `wp1-schema-audit.json` — the whole-tree `test schema` run (§5.3).
- `wp1-invariants-after.txt` — the WP1 suite result.
- `wp1-index-before-failures.txt` / `wp1-index-after-failures.txt` — the two failure sets compared in §5.4.
