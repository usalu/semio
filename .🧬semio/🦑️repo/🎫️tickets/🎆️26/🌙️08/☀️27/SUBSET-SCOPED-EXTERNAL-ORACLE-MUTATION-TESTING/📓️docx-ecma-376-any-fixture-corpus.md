# DOCX ECMA-376/✳️any — third-party-generated fixture corpus

Target: artifact `s.stdio.docx`, standard `ecma-376`, subset `any` (`🪆️subsets/✳️any`). Mirrors the
`bcf@2.1/✳️any` working reference exactly: a second, JS-ecosystem oracle entry alongside the untouched
pre-existing Rust `zip`+`quick-xml` entry, because `semio-s-plugin-stdio` does not compile right now (a
peer's in-flight migration — confirmed independently, not this ticket's bug) and a probe library must
only READ, never predict.

## What was built

1. **Generator** — `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🏭️generator/📜️script.ts`
   `bun 📜️script.ts generate [--only <id>]`. 25 recipes, each a typed `DocxModel` (body block tree +
   style table + optional extra OPC parts) authored directly in TypeScript, handed to `jszip` 3.10.1 +
   `fast-xml-parser` 5.11.1's `XMLBuilder` to write a real OPC package: `[Content_Types].xml`,
   `_rels/.rels`, `word/_rels/document.xml.rels`, `word/document.xml`, `word/styles.xml`, and (for
   `set-part`/`remove-part` recipes) `docProps/core.xml`. Nothing here executes this repository's own
   mutation dispatch — before and after are two independently-authored trees, same as `bcf`'s `RECIPES[].build()`.
   Uses the XMLBuilder's `preserveOrder: true` mode throughout (both builder and, in the probes, the
   parser) — the only mode that keeps a `w:p` followed by a `w:tbl` in the order they were appended,
   since the simpler object-keyed mode (which `bcf`'s own generator uses, and which never needs
   cross-tag ordering) folds same-tag siblings into one array and loses order between *different* tag
   names at the same nesting level.
2. **Probes** — `.../✳️any/🔬️probes/📜️script.ts`: `docx-import`, `docx-project`, `docx-compare`, emitting
   `semio.repository-test.probe-report/v2`. The projection implements exactly the typed view
   `semantic-docx-ecma-376-mutate-v1`'s own `description` promises: `body` (ordered paragraph/table
   block tree — paragraphs with style ref + ordered runs, tables with ordered rows/cells recursively)
   and `styles` (ordered id/name/basedOn list), both order-sensitive; every other real OPC part compared
   by content-type (resolved from `[Content_Types].xml` defaults-by-extension + overrides-by-path) plus
   content digest, as an unordered path-keyed map; `[Content_Types].xml` and every `*.rels` part
   excluded entirely.
3. **Oracle registration** — `.../✳️any/🧪️oracle/🔣️.json`:
   - New oracle `jszip-fast-xml-parser-docx-ecma-376-mutate` (javascript ecosystem, `kind:
     "third-party-library"`), capability `docx-ecma-376-mutate`. The pre-existing Rust
     `zip-quick-xml-docx-ecma-376-mutate` entry is untouched.
   - New comparisonProfile `semantic-docx-ecma-376-jszip-v1` delegating via `pipeline` to a new
     comparisonPipeline `docx-ecma-376-jszip-compare-v1` (two stages: `docx-import` asserting
     `bothImport: true`, then `docx-compare` asserting `equal: true`) — mirrors `bcf-2-1-jszip-compare-v1`'s shape.
   - Three new `probes[]` entries (`docx-import`, `docx-project`, `docx-compare`), all `qualification.status: "qualified"`.
   - A new `mutationManifests` entry: 13 mutations (`no-mutation` … `remove-part`), each with
     `productionDispatch` naming the real `DocxMutation` enum variant (grepped from
     `.../🧬️mutations/🦀️.rs`, never invented) and `oracleRequirements` naming the new oracle.
   - 25 new `fixtureManifests` entries, `class: "third-party-generated"`, paths
     `../🧫️fixtures/<recipe>/<file>` resolved against the oracle directory.
4. **Fixtures** — `.../✳️any/🧫️fixtures/<recipe>/{before.docx[, after.docx]}`, 43 files across 25
   recipes: 18 with `before.docx` + `after.docx` (applied/no-op outcomes), 7 with `before.docx` only
   (rejected outcomes — no legal after-state to author independently, same convention as BCF's
   `insert-topic-rejected-duplicate`).

## Recipe coverage

All 13 mutation kinds have at least one `applied` scenario. `no-op`/`rejected` scenarios were added only
where the subject's own `🔺️diff/🦀️component.rs` / `🧬️mutations/🦀️.rs` documents that outcome is
reachable (playbook Step 2 — grepped, not assumed):

- `set-snapshot`, `set-block-content`, `set-run-text`, `set-run-formatting`, `set-part`: each has an
  explicit `DocxDiff::default()` no-op branch in the diff builder itself (identical snapshot / unchanged
  block / unchanged run text / unchanged formatting flags / identical part content) — one `no-op` recipe
  each.
- `insert-block`, `remove-block`, `insert-style`, `remove-style`, `set-style-name`,
  `set-style-based-on`, `remove-part`: no diff-level no-op branch, but `apply_indexed`/`apply_named`
  raise `mutation.apply.invalid-index` / `mutation.apply.missing-target` /
  `mutation.apply.duplicate-target` when the diff targets an index/id that isn't there — one `rejected`
  recipe each, before-only, matching BCF's `insert-topic-rejected-duplicate` /
  `remove-topic-rejected-missing` pattern.
- `no-mutation` is the identity element (`DocxDiff::default()` unconditionally) — one `no-op` recipe.
- `insert-block-appends-a-pricing-table` is also the corpus's one exercise of table/row/cell
  serialization and projection (a 2×2 table appended as the body's third block); every other recipe uses
  a two-paragraph body (`Nakagin `/`Capsule Tower` heading + a closing paragraph), the same opening
  paragraph already committed in `set-snapshot/🧪️tests/bolds-the-tower-run-of-the-opening-paragraph`'s
  own `📸️snapshot/⬅️before` JSON — `bolds-the-tower-run-of-the-opening-paragraph` reuses that scenario id
  and content on purpose so the independently-built DOCX and that hand-authored `DocxSnapshot` describe
  the same document.

## Verification — real output

`fixture verify`:
```
[fixture verify] 25 fixture(s), 0 file problem(s)
```

`fixture reproduce` (regenerates each fixture in a fresh subprocess and compares hashes — this is the
per-fixture check the playbook's reproducibility note requires, not a whole-corpus double-run):
```
[fixture reproduce] 25 generated fixture(s), 0 problem(s)
```

Sanity check on the pre-existing reference, exactly as instructed:
```
$ bun 🧰️framework/…/🧪️test/📜️script.ts fixture reproduce --artifact bcf
[fixture reproduce] 15 generated fixture(s), 0 problem(s)
```

`matrix --artifact docx` (informational; numbers reported, not fought):
```
[matrix] runtimeMutationCoverage           45.45%  30/66
[matrix] subsetOwnershipCoverage           97.93%  614/627
[matrix] externalOracleCoverage            64.59%  405/627
[matrix] oracleEvidenceCoverage            34.61%  217/627
[matrix] oracleCapabilityCoverage          71.74%  33/46
[matrix] fixtureClassCoverage             100.00%  3/3
[matrix] fixtureProvenanceCoverage         94.16%  403/428
[matrix] fixtureReproducibilityCoverage   100.00%  428/428
[matrix] dependencyIsolationCoverage      100.00%  203/203
```
These are repo-wide totals across every artifact, not docx-specific (the platform's `matrix` command has
no per-artifact isolation of these particular rows). Two rows are worth flagging honestly:

- **"Which mutations remain under wildcard ownership?"** names all 13 of my new `s.stdio.docx` mutations.
  This is `WILDCARD_SUBSET_IDS.includes("any")` combined with docx/ecma-376 declaring sibling subsets
  `strict`/`transitional` in its own `🪆️subsets/🔣️component.json` (whose `subsets` key for the
  unconstrained profile is spelled `"*"`, not `"any"` — the directory is `✳️any` but the declared subset
  id is `"*"`). This is a **pre-existing** taxonomy mismatch: it already existed for the one committed
  `set-snapshot` scenario before this session touched anything, is not something these 25 fixtures
  introduced, and is out of this ticket's scope (the task named `✳️any` as the exact target, mirroring
  BCF, and did not ask for a subset-taxonomy rename). It does not affect `fixture verify` or `fixture
  reproduce`, which don't consult `fixtureManifestProblems`.
- **"Which fixtures lack provenance?"** lists all 25 new fixtures — traced directly to the SAME
  wildcard-subset check: `fixtureProvenanceCoverage`'s `withoutProvenance` is computed by running
  `fixtureManifestProblems(fixture, repoRoot)`, and for every one of my fixtures the *only* item
  `fixtureManifestProblems` returns is `'target.subset "any" is a wildcard'` (confirmed by calling the
  function directly). Provenance itself (`license`, `attribution`, `security`, `privacy`) is fully
  populated on every fixture — the report line is just mislabeled for what it actually measures.
- **"Which subsets have no real-world fixture?"** lists `s.stdio.docx@ecma-376/any` alongside `bcf@2.1/any`
  and several other pilot subsets — expected: every fixture here is `class: "third-party-generated"`,
  never `real-world`, same as BCF's own 15.

## Gate validated both ways — real numbers

(a) `no-mutation-no-op`'s before/after pair (byte-identical, `sha256:4ccedf9c…db0df1a` both sides):
```
$ bun 🔬️probes/📜️script.ts docx-compare --input …/no-mutation-no-op/before.docx --input …/no-mutation-no-op/after.docx
{"equal": true, "diffCount": 0, "diffs": []}
```

(b) That same `before.docx` against a deliberately corrupted copy (`word/styles.xml`'s `Heading1` style
name replaced with `"CORRUPTED HEADING"` via a direct `jszip` edit, independent of the generator):
```
$ bun 🔬️probes/📜️script.ts docx-compare --input …/no-mutation-no-op/before.docx --input …corrupted-style-name.docx
{"equal": false, "diffCount": 1, "diffs": ["$.styles[0].name: \"heading 1\" ≠ \"CORRUPTED HEADING\""]}
```

Also exercised on a genuine applied mutation (`bolds-the-tower-run-of-the-opening-paragraph`):
```
$ bun 🔬️probes/📜️script.ts docx-compare --input …/before.docx --input …/after.docx
equal False diffCount 1 diffs ['$.body[0].runs[1].bold: false ≠ true']
```

The gate accepts the known-good pair and rejects the known-bad pair, naming the exact field both times —
the playbook's Step 3 lesson, done both directions with real fixtures, not asserted.

## Reproducibility (playbook lessons applied, not just cited)

- `FIXED_DATE` (`2026-01-01T00:00:00Z`) passed to every `zip.file()` call, **including** an explicit
  `zip.file("word/", null, {dir:true,date:FIXED_DATE})`-style entry for every directory level
  (`_rels/`, `word/`, `word/_rels/`, and `docProps/` when a recipe uses it) BEFORE the file that lives in
  it — jszip's implicit-parent-folder auto-creation stamps `new Date()` on an undeclared folder entry
  regardless of the child's own `date` option, exactly as `bcf`'s generator documents.
- Proved **per-fixture** via `test fixture reproduce`, which spawns a fresh `bun` subprocess per fixture
  (confirmed by reading `FixtureScript`'s `reproduce` case) — not a whole-corpus double-run, so any
  jszip/fast-xml-parser process-global state (none is known to exist, unlike OCCT's STEP counters, but
  the check doesn't rely on that assumption) would have been caught.
- The metrics/manifest files were never rewritten after their bytes were hashed — `fixtureManifests`
  were computed once from the already-generated `.docx` files and spliced into `🧪️oracle/🔣️.json` in a
  single pass.

## What could not be verified, and why

- **Production-side round-trip** (`InsertBlock`/`RemoveBlock`/etc. actually applied by
  `crate::artifacts::docx` and compared against these fixtures) was not run: `semio-s-plugin-stdio` does
  not currently compile (a peer's in-flight migration, confirmed independently before this session
  started, not attempted to fix per the task's explicit instruction). This is exactly why the JS-ecosystem
  oracle exists as a second, independent entry rather than a replacement.
- **Rejected-outcome recipes** (7 of them) carry only a `before.docx`: there is no legal after-state to
  author independently for an operation production would refuse, so there is nothing beyond the
  `notes` field (naming the exact `MutationApplyError` code the corresponding Rust apply path would
  raise) tying each one to the code. This mirrors BCF's own `*-rejected-*` fixtures exactly.
- The subset-taxonomy wildcard mismatch (`✳️any` directory vs. declared subset id `"*"` vs. sibling
  `strict`/`transitional`) was identified but deliberately not fixed — pre-existing, out of scope, and
  fixing it would be a taxonomy-wide rename affecting the one already-committed `set-snapshot` scenario
  too, not something to do unilaterally inside a fixture-corpus ticket.

## Files touched

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🏭️generator/📜️script.ts` (new)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🔬️probes/📜️script.ts` (new)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧪️oracle/🔣️.json` (edited — new oracle, comparisonProfile, comparisonPipeline, probes, mutationManifests, fixtureManifests)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧫️fixtures/<25 recipe dirs>/{before.docx[, after.docx]}` (new, 43 files)
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️27/SUBSET-SCOPED-EXTERNAL-ORACLE-MUTATION-TESTING/📜️docx-build-fixture-manifests.ts` (new, ticket-root input script — recomputes `fixtureManifests` JSON from `RECIPES` + on-disk file hashes; kept for reproducibility of the oracle-json edit)
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️27/SUBSET-SCOPED-EXTERNAL-ORACLE-MUTATION-TESTING/📜️docx-patch-oracle.cjs` (new, ticket-root input script — splices the oracle entry/profile/pipeline/probes/manifests into `🧪️oracle/🔣️.json`)
