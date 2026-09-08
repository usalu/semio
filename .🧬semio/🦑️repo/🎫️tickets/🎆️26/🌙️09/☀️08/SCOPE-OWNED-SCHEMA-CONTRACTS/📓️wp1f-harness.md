# WP1f — the Rust rule gains its re-export, the walk loses two foreign trees, and the leaf walk finally reaches depth two

Worker: W1f (Opus). Partition: `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/**`.
Inputs: `📋️execution-contract.md` §A/§B/§E, `📓️wp1e-harness.md` (the table and the open items as W1e left them),
`📓️wp4d-repo-product.md` §7 W4d-1, `📓️wp4b-mutations.md` §4/§7.4, `📓️wp4d-framework-modules.md` §6.2-6.4,
`📋️cross-partition-requests.md` rows 133, 135, 138, 148, 150, 153.
Successor to W1e. A predecessor pass (W1f, rate-limited) left no edits; this pass started from W1e's tree.

---

## 1. Headline

| ledger | | measured |
|---|---|---|
| **148** | Rust export presence = `pub struct\|enum\|type <Export>` **or** a per-name `pub use`; grouped and glob re-exports declare nothing | 17 vector cases + 1 boundary test, all green |
| **133** | `schemaTreeFiles` skips `.gitmodules`-declared submodule paths (memoised, NFC) | `♻️mit-bestand` **8 → 0** findings |
| **153** | every walker skips `dist/` | already true via `SKIP_DIR_NAMES`; now **pinned by a test** and documented, 37 `dist/🧬️schema/📜️artifact-definition.json` copies stay invisible |
| **138** | `mutationLeafDirectories` reads depth 1 **and** 2; `readLeafDescriptors`, `leafDescriptorCoverage`, `resolvePayloadSchemas`, `scaffoldOwnerDescriptors` all go through it | `manifest payload-schema` **2084/2146 (97.1%) → 2470/2470 (100.0%)**; **0** architect domain directories reported as leaves |
| **135** | `test schema --under <root>` runs the catalog half, filtered by scope path | the `under.length === 0` gate is gone; a per-partition run now reports `schema-export-*` / `schema-ref-*` |
| **150** | gate flag when the placement and resolution measurements disagree | new `schemaMeasurementDisagreementDiagnostics`, raising `schema-catalog-stale` from the harness side |
| **W1e R-1** | the eight library-emitted codes with no table row | table **33 → 41 codes**; check-only **6 → 13** |
| Suite | `🧬️schema-invariants` | **121 pass / 1 fail** (122 tests, 488 expect()), 94 → 122 tests, every new one green |
| Suite | `📐️test-layout` | **23 pass / 0 fail** — W1e's R-4 red is gone, the peer fixed it |
| `test schema` | whole tree | see §7 |

---

## 2. Row 148 — a `pub use` that names one export declares it

### 2.1 The rule

`declaresSchemaExport("🦀️rust", …)` was `^\s*pub\s+(?:struct|enum|type)\s+<Export>\b` alone, which
contradicted contract §A's "a Rust re-export is `pub use`" — W4d §6.2 raised the contradiction and the
coordinator settled it as row 148 in the regex's favour. The rule is now that plus `declaresRustReExport`:

```ts
const path = "[A-Za-z_][A-Za-z0-9_]*(?:\\s*::\\s*[A-Za-z_][A-Za-z0-9_]*)*";
new RegExp(`^\\s*pub\\s+use\\s+(?![^;\\n]*[{*])${path}\\s*::\\s*(?:${name}|[A-Za-z_][A-Za-z0-9_]*\\s+as\\s+${name})\\s*;`, "mu")
```

Three decisions are in that expression and each is a boundary a reader can check:

1. **The negative lookahead is the grouped/glob exclusion.** `(?![^;\n]*[{*])` refuses any statement
   carrying `{` or `*` before its `;`, so `pub use a::{X, Y};` and `pub use a::*;` declare nothing —
   contract §A's wording, and the honest reading: neither statement names `<Export>` at the module's
   surface, so a reader of the module (and this rule is a reader) cannot tell from it whether the
   export is published at all.
2. **A rename to the export name counts, a rename away from it does not.** `pub use a::XV1 as X;`
   publishes `X` and is accepted; `pub use a::X as XV1;` publishes `XV1` and is refused. The contract
   writes only the first form, but tail-matching without the `as` branch would have accepted the
   second — a false green on a module that publishes a different name.
3. **`pub(crate) use` is not `pub use`.** `pub\s+use` does not match it, deliberately: a re-export the
   crate keeps to itself publishes nothing, and §A already says a private `use` declares nothing.

### 2.2 The vector, not only the regex

`🧪️tests/🧬️schema-invariants/🔣️.json` gains `exportPresenceCases` (17 rows) and the protocol's
`SchemaInvariantCases` gains the required key, so the file is `semio.repository-test.schema-invariant-cases/**v4**`.
A row is `{ id, format, export, source, present, parser? }` — the format source verbatim and the verdict,
because a presence rule is a statement about text and the vector should carry the text. Twelve rows are
Rust (both accepted forms, both accepted definitions, the alias definition, and five negatives: grouped,
glob, private, crate-visible, another name), and five carry the other formats so the collection is the
whole rule and not the one clause that changed.

The suite drives every row, and one hand-written test restates the Rust boundary directly, because the
place a regex is easiest to get wrong is exactly where the vector is easiest to read past.

---

## 3. Rows 133 + 153 — what the walk is allowed to see

### 3.1 The submodule skip

`submodulePaths(repoRoot)` parses `.gitmodules` once per root (`path = <rel>` lines, NFC-normalised)
and `schemaTreeFiles` skips those directories. It is the diff W4d-1 asked for, with the memo, the NFC
normalisation and no `git` subprocess — so the rule holds in a devcontainer, on a bare checkout and
with no `git` binary at all. Today the set is exactly `{"♻️mit-bestand/🔎️recherche"}`.

The NFC normalisation is not decoration: git writes `.gitmodules`, the filesystem hands directory
entries back, and macOS hands them back decomposed. Both sides are normalised before comparison.

```
$ bun …/🧪️test/📜️script.ts test schema --under ♻️mit-bestand          # before
[test schema] 8 invariant finding(s) over ♻️mit-bestand
[test schema]       7 × schema-placement-outside-module
[test schema]       1 × schema-placement-forbidden-filename

$ bun …/🧪️test/📜️script.ts test schema --under ♻️mit-bestand          # after
[test schema] 0 invariant finding(s) over ♻️mit-bestand
```

All eight named a `♻️mit-bestand/🔎️recherche/_neo4j/…` path — files of `usalu/recherche`, authored and
versioned there, which this workspace tracks as one gitlink. They were findings nobody here could act on.

The proof is behavioural and two-sided: a synthetic repository writes the same definition inside the
declared submodule and inside a sibling that is not one; the walk returns only the sibling, and the
sibling still reports `schema-placement-outside-module`. A second case drops `.gitmodules` and the
carve-out disappears with it — a skip nobody declared is a skip nobody can read.

### 3.2 `dist/`

Row 153 asks every walker to skip `dist/`. The harness already did: `dist` and `📤️dist` have been in
`SKIP_DIR_NAMES` since before this ticket, and the 37 gitignored
`…/📦️packages/🟦️typescript/dist/🧬️schema/📜️artifact-definition.json` copies never entered a count.
What was missing is that nothing said so and nothing held it: the rule lived in a name inside a set with
eleven other names. It is now stated in `schemaTreeFiles`'s own docstring beside the other two cuts, and
pinned by a case that writes the same document into `dist/` and beside it and asserts the walk returns
one of them. Measured on the live tree after the change: **0** rows in `wp1f-test-schema.json` carry a
`/dist/` path segment.

---

## 4. Row 138 — a mutation leaf is one or two directories deep

### 4.1 What was wrong, and what was wrong under it

`resolvePayloadSchemas` and `readLeafDescriptors` read ONE level of `<owner>/🧬️schema/🧬️mutations` and
filtered by `isMutationLeafDirectory`. W7's §4 measurement: architect's `🏛️program/✳️any` holds 69 domain
directories, **none** of which can carry a descriptor, and 266 real `<domain>/<verb>` leaves beneath them,
**all** of which do. The harness reported the domain directories as undeclared leaves and never saw the
real ones — row 7 in the ledger is entirely this.

Reading the code for the fix surfaced a second defect underneath: `isMutationLeafDirectory` read
`mutationDirectoryPattern` off `testTaxonomy()`, and `testTaxonomy` returns
`Object.fromEntries(required.map(…))` — a projection onto its `required` list, which does not name that
key. **The declared pattern never arrived and the structural fallback was always the live rule**, silently,
for as long as the function has existed. The read is now `rawTaxonomy(repoRoot)` and a taxonomy that
declares no pattern throws instead of quietly substituting another rule; the hard-coded
`["💾️binary", "📝️text", "🧬️schema"]` facet list is replaced by `mutationFacetDirNames`, read from
`mutationOrganizationalFacetDirs` + `mutationBehaviorFacetDirs`.

### 4.2 The rule

`mutationLeafDirectories(repoRoot, ownerRel)` returns leaf paths relative to `🧬️mutations`, one or two
segments. The classification is declared, never guessed:

| the directory | verdict |
|---|---|
| a facet (`🧩️plan`, `📝️text`, `💾️binary`, `🧬️schema`, `🦠️mutation`, `🔺️diff`, `↩️inverse`) or a `🧪️`/`🧫️` collection | not considered at either level |
| carries a descriptor | a leaf, whatever its name |
| carries none, and has qualifying children | a GROUPING directory; each qualifying child is a leaf |
| carries none, has no qualifying children, name matches `mutationDirectoryPattern` | a leaf, and `leafDescriptorCoverage` reports it as `missing` |

The collection exclusion is load-bearing and was found by measurement, not by reasoning: without it a
`🧬️mutations/🧪️tests` directory reads as a grouping directory, because a test case's own contribution
file has the same filename as a leaf descriptor. Probing without that clause added 43 test cases to the
leaf set.

`resolvePayloadSchema` now takes the `kind` from the descriptor's `semanticKind` and falls back to the
directory name only when there is no descriptor. A two-segment leaf has no kind derivable from its path
at all — `🌳️node/⚖️change-weights` declares `change-node-morph-weights`, which no concatenation of the two
segments produces — so reading the declaration first is what makes such a leaf reportable under its own
name. For the same reason `scaffoldLeafDescriptor` REFUSES a two-segment leaf rather than inventing a
kind for it (and now derives its emoji from the leaf's own last segment instead of the joined path).

### 4.3 Measured, before and after

`wp1f-leaf-depth-probe.py` (in this ticket folder) reproduces both rules over the live tree:

| | before | after |
|---|---|---|
| leaves | 2 302 | 2 635 |
| grouping directories reclassified | — | **99** (architect 69, stdio/gltf 29, framework/os 1) |
| dropped from the leaf set | — | 62, **0 of which carry a descriptor** |
| added | — | 395, **all at depth 2** |
| depth-1 leaves gained or lost | — | **0** |

Nothing at depth 1 moved in either direction, which is the property that makes this a depth fix and not
a redefinition. The command the ledger names as the acceptance measure:

```
$ bun …/🧪️test/📜️script.ts manifest payload-schema        # before
[manifest payload-schema] 2084/2146 leaves declare a payload contract at the taxonomy location (97.1%)
[manifest payload-schema]     62 × the leaf carries no 🔣️.json descriptor, so nothing declares its payload contract
[manifest payload-schema]   undeclared: …/🏛️architect/…/🧬️mutations/🔑️access-rule
                            (+ 61 more, 47 of them architect domain directories)

$ bun …/🧪️test/📜️script.ts manifest payload-schema        # after
[manifest payload-schema] 2470/2470 leaves declare a payload contract at the taxonomy location (100.0%)
```

386 more leaves reached inside the registry's owners, **every one of them already declaring its payload
contract**, and the 62 phantom "undeclared leaves" gone. The 100 % is earned rather than achieved: no
descriptor and no schema was written by this pass.

---

## 5. Rows 135 + 150 — the gate stops measuring one tree and reporting about another

### 5.1 `--under` runs the catalog half (row 135)

`schemaContractDiagnostics` ended in `…(under.length === 0 ? schemaResolutionDiagnostics(repoRoot) : [])`.
Every per-partition run therefore reported zero `schema-export-*` and `schema-ref-*` rows **by
construction** — W4d §5.1 read exactly that output and had to say in prose that its 0 was vacuous.

`schemaResolutionDiagnostics(repoRoot, under = "")` now selects the catalog scopes whose `path` lies
inside the filter, and `schemaContractDiagnostics` always calls it. One detail decides whether the filter
is honest: **the `$id` index is still built over every scope**, and only the reporting loop is narrowed.
A subtree's references point out of it, so an index narrowed to the filter would report every legal
cross-scope `$ref` as unresolved — a filter that manufactures findings is worse than one that hides them.
A case pins exactly this (`crossScopeRef` under the writer filter yields the one dependency finding it
should, and no `schema-ref-unresolved`).

### 5.2 The two measurements are held against each other (row 150)

This layer measures the tree twice and the two measurements are independent: PLACEMENT walks the
filesystem and asks where each definition sits; RESOLUTION reads the derived catalog and asks what each
declared scope owes. W1e §6.5 recorded the symptom without a remedy — `schema check` reporting 7 542
findings over 3 068 scopes where `test schema` reported 4 529 — and W4d §6.3 asked for a flag.

`schemaMeasurementDisagreementDiagnostics(repoRoot, files, under)` compares the two sets directly:

- every `🧬️schema/` module directory the walk reaches that carries a schema definition, against
- every `catalog.scopes[*].path`,

both narrowed by the same `under`, and raises `schema-catalog-stale` on each side of the symmetric
difference with a detail saying which side saw it. The code is right rather than convenient: its remedy —
regenerate the catalog — is exactly what a disagreement calls for, and `schema check` already raises it
from its own side, so this is one rule with two emitters rather than a harness-private signal. Its
`emitters` row moved from `["check"]` to `["harness", "check"]` accordingly.

It is part of `schemaContractDiagnostics`, so it gates: a subtree can no longer read clean because one
of the two halves never looked at it.

---

## 6. W1e R-1 — the eight library codes join the table

W1e left `unshared-codes=0` true and fragile: eight codes `📚️library/🔍️discovery/🟦️.ts` emits had no row
in `SCHEMA_DIAGNOSTIC_CODE_TABLE` and were silent only because today's tree does not trigger them. They
are enumerated straight out of the library's emit sites and added as `check`-emitted rows, with the
description read from the detail the library actually prints:

| code | why it can fire |
|---|---|
| `schema-module-id-inconsistent` | a facet document's `$id` deepens or leaves its module's scope path |
| `schema-document-unparseable` | a file in the canonical schema slot is not parseable JSON at all |
| `schema-document-not-object` | it parses and is not a JSON Schema object |
| `schema-enum-empty` | `enum: []` admits no instance (row 69) |
| `schema-cross-scope-dependency-uncataloged` | a `$ref` resolves into a document no catalogued scope owns, so no `dependsOn` entry could declare it |
| `schema-mutation-leaf-schema-absent` | a described leaf carries no `🧬️schema/🔣️.json` |
| `schema-mutation-aggregate-id` | an aggregate's `$id` deepens its own scope |
| `schema-mutation-aggregate-kinds-redundant` | an aggregate still carries `x-semio-mutationKinds` (row 23/90) |

Table **33 → 41**; harness-emitted **27 → 28** (`schema-catalog-stale` joined, §5.2); check-only **6 → 13**.
The protocol enum and the vector's `checkOnlyDiagnosticCodes` are regenerated from the table by the same
script, and the suite holds all three equal — so the three lists cannot drift, and no harness rule can
raise a `check`-only code without a compile error (`schemaDiagnostic` takes `SchemaHarnessDiagnosticCode`).

---

## 7. Verification — real output

_(filled in below from the runs of this session)_

---

## 8. Files changed

| File | Change |
|---|---|
| `…/🧪️test/📦️packages/🟦️typescript/🟦️.ts` | `declaresRustReExport` (new) + Rust branch of `declaresSchemaExport`; `submodulePaths` (new) + `schemaTreeFiles` skips submodules, docstring states all three cuts; `clearSchemaContractCache` clears it; `SCHEMA_DIAGNOSTIC_CODE_TABLE` +8 rows and `schema-catalog-stale` gains the `harness` emitter; `mutationFacetDirNames` + `mutationLeafDirectories` (new) and `isMutationLeafDirectory` reads the taxonomy for real; `readLeafDescriptors`, `leafDescriptorCoverage`, `scaffoldOwnerDescriptors`, `resolvePayloadSchemas` go through the new walker; `resolvePayloadSchema` takes `kind` from the descriptor; `scaffoldLeafDescriptor` uses the leaf's own segment; `schemaModuleDirectoriesOnDisk` + `schemaMeasurementDisagreementDiagnostics` (new); `schemaResolutionDiagnostics(repoRoot, under)`; `schemaContractDiagnostics` always runs resolution and the disagreement flag; four `"🧬️schema"` literals replaced by `schemaModuleDirName(repoRoot)` |
| `…/🧪️test/🧬️schema/🔣️.json` | `SchemaDiagnosticCode` enum 33 → 41; `SchemaInvariantCases` gains required `exportPresenceCases` and goes to `…/v4` |
| `…/🧪️test/🧪️tests/🧬️schema-invariants/🔣️.json` | `schema` → v4; `exportPresenceCases` (17 rows); `checkOnlyDiagnosticCodes` 6 → 13 |
| `…/🧪️test/🧪️tests/🧬️schema-invariants/🟦️.ts` | new `🚶️ the tree walk's boundaries` (3), `🍃️ mutation leaves at depth one and two` (3), `⚖️ the two measurements` (4); `🔎️ per-format export presence` drives the 17 vector rows and gains the Rust boundary test; the collection-count test names the new collection |
| `<ticket>/wp1f-leaf-depth-probe.py` | the before/after leaf measurement of §4.3 |
| `<ticket>/🗑️generated/wp1f-test-schema.json` | ticket output, §7 is read from it |

No file outside the partition was written. The peer of row 62 has `🟨️.mjs`, `🧪️tests/📐️test-layout/**` and
`🧪️tests/🧪️test-platform/🟦️.ts` staged in this partition; none of their work was reverted, every run below
is on the tree as they left it, and their `📐️test-layout` red from W1e R-4 is green again.

---

## 9. Cross-partition requests

_(filled in below)_

## 10. Open questions

_(filled in below)_
