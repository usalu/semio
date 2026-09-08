# WP1d — Row-98 rule activation, the shared diagnostic-code table, and the new live measurement

Worker: W1d (Opus). Partition: `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/**`.
Inputs: `📋️execution-contract.md` §A (export presence per format, diagnostic code table),
`📓️wp1c-harness-rules.md` (§7 open questions, now decided), `📋️cross-partition-requests.md` rows 97, 98.

Assignment: the four row-98 decisions (keep `schema-export-format-undeclared`; a distinct
`schema-ref-broken-internal`; the six GraphQL keywords; TS completeness = type **and**
`parse<Export>()`, activated now), row 97's single importable code table, vector + tests, real runs.

---

## 1. Headline

| | |
|---|---|
| Q1 | **kept as it was.** `x-semio-formats` must name the normative format too; a new case proves the code fires on an annotation that omits it |
| Q2 | new code **`schema-ref-broken-internal`** — a module-internal JSON pointer that lands on nothing is a document defect, no longer folded into the cross-document refusal `schema-ref-unresolved` |
| Q3 | GraphQL presence accepts `type\|input\|enum\|interface\|union\|scalar` — the vocabulary is now the exported `GRAPHQL_EXPORT_KEYWORDS`, asserted directly and through five case repositories |
| Q4 | new code **`schema-export-parser-missing`**, activated. TypeScript completeness is the exported type **and** `parse<Export>()` |
| Row 97 | the code table is one exported constant, `SCHEMA_DIAGNOSTIC_CODE_TABLE`, code → one-line description; import path in §4 |
| Vector | `🧪️tests/🧬️schema-invariants/🔣️.json` 21 → **30** catalog cases (2 expectations re-coded, 9 added) |
| Invariants suite | **90 pass / 1 fail** (91 tests, 358 expect() calls, 4.1 s). The red is unchanged: W1c's R-3 generator-vector policy disagreement, outside this partition |
| `test schema` | **5 166 findings** (was 3 493), of which **1 696 × `schema-export-parser-missing`** — the large new count row 98 anticipated. Per-code and per-owner tables in §5.4 |

---

## 2. Rule changes, with the test that holds each one

The rules are in `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts`; every
case is declared in the language-agnostic vector `🧪️tests/🧬️schema-invariants/🔣️.json` and asserted by
`🧪️tests/🧬️schema-invariants/🟦️.ts`.

### 2.1 Q1 — an annotation must name the normative format (no change, now proved)

`schema-export-format-undeclared` stays and keeps firing when the export is defined in the normative
format and `x-semio-formats` does not list it. The reading "the normative format is implicit" was the
alternative; row 98 refused it, so the annotation stays honest in both directions and a reader never has
to know which format is normative to read the annotation.

New case, because the previously shipped cases only covered *non*-normative formats:

| case | asserts |
|---|---|
| `export-annotation-that-omits-the-normative-format` | `x-semio-formats: ["🦀️rust"]` on an export defined in JSON Schema → exactly one `schema-export-format-undeclared`, `format: 🔣️jsonschema` |

### 2.2 Q2 — `schema-ref-broken-internal`

`schemaRefDiagnostics` judged three shapes and reported two very different defects under one code. Now:

| situation | code |
|---|---|
| `#/definitions/<helper>`, `#/$defs/<Export>`, `#` that resolves | none — module-internal addressing is legal (row 77) |
| a module-internal pointer that lands on nothing | **`schema-ref-broken-internal`** |
| a relative file path, an unknown `$id`, `<target $id>#/definitions/<helper>`, any other cross-document fragment | `schema-ref-unresolved` (unchanged) |

The split matters for routing, not for aesthetics: a broken internal pointer is one document's own defect
and is fixed by its owner in place; a `schema-ref-unresolved` is an *addressing* refusal that usually
needs the target scope to export something, or a `dependsOn` entry. On the live tree the 120 references
that used to be one number are now 115 + 5.

| case | asserts |
|---|---|
| `unresolved-local-ref` (re-coded) | `#/$defs/Author` with no `Author` → `schema-ref-broken-internal` |
| `module-internal-helper-ref-that-lands-on-nothing` (re-coded) | `#/definitions/author` with no `definitions` → `schema-ref-broken-internal` |
| `cross-document-ref-into-another-modules-definitions` (unchanged) | still `schema-ref-unresolved` — this is the pair that proves the two codes are distinguishable |
| `module-internal-helper-ref-is-never-a-finding` (unchanged) | false-positive guard: a resolving helper ref produces nothing |

### 2.3 Q3 — the GraphQL export keywords

Presence was already tolerant of six keywords; row 98 makes that the declared rule, so it is now named
and exported rather than inlined in a regular expression:

```ts
export const GRAPHQL_EXPORT_KEYWORDS = ["type", "input", "enum", "interface", "union", "scalar"] as const;
```

An export whose JSON Schema is an `enum` or a `oneOf` has no honest GraphQL `type` form; forcing one
would make the GraphQL document a lie about the contract, which is why the wide vocabulary is the rule
and not a leniency.

| case | asserts |
|---|---|
| `graphql-export-declared-as-an-input` / `-an-enum` / `-an-interface` / `-a-union` / `-a-scalar` | five case repositories, each declaring `Artifact` with that keyword → **no** finding |
| `🔎️ per-format export presence > graphql accepts every type-system keyword the contract lists, and nothing else` | every keyword accepted; `directive @Artifact` and `type ArtifactDraft` rejected (the prefix guard) |

### 2.4 Q4 — TypeScript completeness is the type **and** `parse<Export>()`

Contract §A defines a TS export as "an exported `interface`/`type` **and** `parse<Export>`". Only the
first half was checked. A type alone is erased at runtime, so a consumer handed a `schema://` payload has
no way to enter the contract — the declaration is a comment. Activated as a **separate code**, not a
`detail` on `schema-export-incomplete`:

| code | fires when |
|---|---|
| `schema-export-incomplete` | the format declares no entity named `<Export>` at all |
| **`schema-export-parser-missing`** | the format declares `<Export>` and no `parse<Export>` entry point |

Separate, because they are separate repairs (write the type vs write the parser), they land on different
owners' lists, and one finding per defect keeps the count honest — a file missing the export entirely is
reported once, not twice. `declaresSchemaExportParser(format, source, exported)` answers `true`
unconditionally for every non-TS format: their entity *is* their entry point.

Accepted declaration forms: `export function parseX`, `export async function parseX`, `export const parseX`,
`export let parseX`, `export declare function parseX`. A non-exported `function parseX` and a
`parseXDraft` are both absent as far as a consumer is concerned.

| case | asserts |
|---|---|
| `typescript-export-without-its-parse-function` | type present, parser absent → one `schema-export-parser-missing`, `format: 🟦️typescript` |
| `typescript-parse-function-written-as-an-exported-const` | false-positive guard: the arrow form satisfies the rule → no finding |
| `a-format-the-annotation-restricted-away-is-never-asked-for-a-parser` | TS restricted away by `x-semio-formats` and missing its parser → only the three `schema-export-format-undeclared`, never a parser finding |
| `complete-scope-across-every-declared-format` (existing, now stricter) | the default case repository had to grow `parseArtifact`/`parseApproval`/`parseArtifactMutation` to stay green — which is itself the proof the rule is live in every catalog case |
| `🔎️ per-format export presence > a typescript export is the type AND its parse function, in either declaration form` | all five accepted forms, both rejected near-misses |
| `🔎️ per-format export presence > no other format is asked for an entry point beside its entity` | rust/proto/graphql/jsonschema answer `true` on empty source |

---

## 3. Files changed

| File | Change |
|---|---|
| `…/🧪️test/📦️packages/🟦️typescript/🟦️.ts` | `SCHEMA_DIAGNOSTIC_CODE_TABLE` (new, 27 entries with descriptions) replaces the bare list; `SCHEMA_DIAGNOSTIC_CODES`/`SchemaDiagnosticCode` derive from it. `GRAPHQL_EXPORT_KEYWORDS` and `declaresSchemaExportParser` added; `declaresSchemaExport` reads the keyword list. `schemaExportCompletenessDiagnostics` emits `schema-export-parser-missing`. `schemaRefDiagnostics` emits `schema-ref-broken-internal` for internal pointers |
| `…/🧪️test/🧬️schema/🔣️.json` | `SchemaDiagnosticCode` +2 (`schema-ref-broken-internal`, `schema-export-parser-missing`); `catalogCases` items gain `dropTypescriptParser`, `typescriptParserAsConst`, `graphqlKeyword` |
| `…/🧪️test/🧪️tests/🧬️schema-invariants/🔣️.json` | 2 expectations re-coded to `schema-ref-broken-internal`, 9 cases added (21 → 30 `catalogCases`) |
| `…/🧪️test/🧪️tests/🧬️schema-invariants/🟦️.ts` | `typescriptModule`/`graphqlModule` helpers; `catalogRepo` and `resolutionRepo` write real parsers; new describes `🔠️ the diagnostic code table is the single vocabulary` (3 tests) and `🔎️ per-format export presence` (3 tests) |
| `<ticket>/wp1d-owner-counts.py` | ticket input: per-code and per-owner rollup over a `test schema --json` report, same owner buckets as `📓️wp1c-harness-rules.md` §5.3 |

No file outside the partition was touched. A peer edited `🟦️.ts` and `🧪️tests/📐️test-layout/` in this partition throughout the session (row 62);
none of their changes were reverted, and the runs below are on the tree as they left it.

---

## 4. Row 97 — the code table export path, for the tooling worker (W2u)

The harness `schema-*` vocabulary is canonical (row 97). It is now **one importable constant**, so the
root `schema check` adopts it by importing rather than by restating:

```ts
import {
  SCHEMA_DIAGNOSTIC_CODE_TABLE,   // Readonly<Record<SchemaDiagnosticCode, string>> — code → one-line description
  SCHEMA_DIAGNOSTIC_CODES,        // readonly SchemaDiagnosticCode[], table order
  type SchemaDiagnosticCode,      // the union, derived from the table's keys
} from "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
```

- **Module path**: `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts`
  (the test package's single entry point; already imported cross-partition both relatively —
  `🧰️framework/🔨️modules/🎠️kernel/🧪️tests/*/🟦️.ts:8` — and dynamically from the repo root —
  `📚️library/⚡️caching/📜️script.ts:773`, `await import(join(root, …))`).
- **Shape**: `SCHEMA_DIAGNOSTIC_CODE_TABLE` is a plain object literal, `as const`, keyed by code, valued
  by a single-line description. Nothing else in it: no severity, no owner, no fix text — those are the
  finding's business, not the code's.
- **Adding a code** is one edit in the table plus the same string in the protocol enum
  `…/🧪️test/🧬️schema/🔣️.json#/$defs/SchemaDiagnosticCode`. The two are held set-equal by
  `🔠️ the diagnostic code table is the single vocabulary > the exported table and the protocol enum
  declare exactly the same codes`, so a code added on one side alone turns the suite red.
- **The renames row 97 asks for** land in `📚️library/🔍️discovery/🟦️.ts` (outside this partition):
  `mutation-leaf-id-grammar` at `:3273`, `export-format-missing` at `:3346`, plus the expectations in
  `📚️library/🧪️tests/🧬️schema-scope-catalog/🧫️fixtures/🔣️.json`. Once they import the table, the parity
  test in `🧪️tests/🧬️schema-invariants/🟦️.ts` can compare **codes** as well as paths and levels — today
  it deliberately compares only the code-free halves.
- One new code the tooling side has no counterpart for yet: `schema-export-parser-missing`. The
  generator's own `export-format-missing` currently covers "TS declares no entity"; it must not also be
  used for "TS declares the entity and no parser", or the two implementations stop being one rule.

---

## 5. Verification — real output

### 5.1 Type check

```
$ bunx tsc -p tsconfig.json --noEmit          # …/🧪️test/📦️packages/🟦️typescript
🟦️.ts(6967,225): error TS2339: Property 'oracle' does not exist on type 'Readonly<{ capability: string; … }>'.
🟦️.ts(6967,275): error TS2339: Property 'oracle' does not exist on type 'Readonly<{ capability: string; … }>'.
```

The same two pre-existing `OracleRequirement.oracle` errors W1, W1b and W1c all recorded, on a line this
work-package did not touch. Everything added here type-checks clean.

### 5.2 The invariants suite

```
$ bun test --timeout 60000 ./🧪️tests/🧬️schema-invariants/🟦️.ts   # cwd …/🔨️modules/🧪️test
 90 pass
 1 fail
 358 expect() calls
Ran 91 tests across 1 file. [4.09s]
```

Every new case passes, both false-positive guards included. The single red is unchanged and is **not** in
this partition:

```
(fail) 🤝️ parity with the catalog generator's own vector > every generator case places the same files and levels as this harness does
Expected: "fixture-defines-schema:🌎️hub/💡️inference/🧪️fixtures/🧫️approval/🧬️.schema.json"
Received: "fixture-defines-schema:🌎️hub/💡️inference/🧪️fixtures/🧫️approval/🧬️.schema.json,
                                  🌎️hub/💡️inference/🧪️fixtures/🧫️approval/🧬️schema/🔣️.json"
```

W1b's R-4 / W1c's R-3, still open: the generator's case expects no placement finding for a schema inside a
`🧪️fixtures` tree, which contradicts execution contract §C. Writing a translation on this side would be
the adapter the ticket exists to remove.

### 5.3 The partition's full `bun test`

The partition holds four suites. Three of them complete and are reported here in full; the fourth,
`🧪️tests/🧪️test-platform/🟦️.ts`, does **not** complete on this box and is reported honestly as such.

```
$ bun test --timeout 60000 ./🧪️tests/📐️test-layout/🟦️.ts ./🧪️tests/🖥️host-protocol-parity/🟦️.ts
 14 pass
 0 fail
 46 expect() calls
Ran 14 tests across 2 files. [4.21s]

$ bun test --timeout 60000 ./🧪️tests/🧬️schema-invariants/🟦️.ts
 90 pass / 1 fail / 358 expect() calls        (§5.2)
```

`🧪️tests/🧪️test-platform/🟦️.ts` ran for **20+ minutes without reaching its summary** and was still
running when this report was written; its individual tests walk the whole repository (one of them,
`🔍️ discovery and contract > every committed case satisfies the frozen contract`, took 487 s on its own,
`🚫️ oracle purity` 129 s), a peer session was running the same suite concurrently, and after
`🚫️ oracle purity` the log stopped advancing for 30 minutes. **No pass/fail total for that suite is
claimed.** What it did report before stalling — 11 failures, none of them in a schema-contract check and
none reachable from anything this work-package touched:

```
(fail) ⚖️ comparison profiles > semantic-pdf-v1 canonicalizes the nondeterministic artefacts …
(fail) 📇️ oracle registry > every registered oracle is test-only and declares its license and capabilities
(fail) 📇️ oracle registry > every recorded no-oracle decision names its rationale and its substitutes
(fail) 🔍️ discovery and contract > discovery is idempotent
(fail) 🔍️ discovery and contract > every committed case satisfies the frozen contract
(fail) 🧹️ clean safety > no tracked fixture, source file or compose path is ever a clean candidate
(fail) 🔒️ dependency ratchet > the committed baseline classifies every ecosystem it tracks …
(fail) 📈️ non-aggregate metrics > oracle coverage counts every discovered case as backed by an oracle …
(fail) 🪆️ case above subset > the only live case-above-subset violation is the one C4 documented …
(fail) 🧫️ mutation without fixture > the live registry retains the independent Stdio declaration census …
(fail) 🚫️ oracle purity > no production source imports a registered oracle
```

Two of them are W1b's R-1/R-2 (`loadOracleRegistry` reading a registry whose rows have no `substitutes`),
the rest are repo-state assertions owned by other partitions mid-migration. W1c left this suite's slot
empty for the same reason; recording the observed reds and the non-completion is the most this box
supports today.

### 5.4 `test schema` over the live tree

```
$ bun 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts test schema --json \
      > <ticket>/🗑️generated/wp1d-test-schema.json
exit 1

findings 5166   schema-bound fixtures 0
   2891 × schema-export-incomplete
   1696 × schema-export-parser-missing
    162 × schema-dialect-not-draft-07
    156 × schema-fixture-defines-schema
    115 × schema-ref-unresolved
     69 × schema-owner-ineligible
     51 × schema-placement-outside-module
     18 × schema-placement-forbidden-filename
      5 × schema-ref-broken-internal
      3 × schema-file-missing
```

**Read against W1c's 3 493.** The 1 673 net rise is the row-98 activation plus real migration by other
partitions in between:

| code | W1c | now | why |
|---|---|---|---|
| `schema-export-parser-missing` | — | **1 696** | Q4, activated. New code, new count |
| `schema-export-incomplete` | 2 841 | 2 891 | unchanged rule; by format `🦀️rust` 1 243 · `🛰️protobuf` 596 · `🔗️graphql` 581 · `🟦️typescript` 471 |
| `schema-ref-unresolved` | 128 | 115 | Q2 moved the broken internal pointers out; split `68 cross-document fragment not #/$defs/ · 45 relative file path · 2 target $id no catalog scope declares` |
| `schema-ref-broken-internal` | — | **5** | Q2. W1c's split of `schema-ref-unresolved` counted 13 broken internal pointers; 5 remain. The 8 are gone from the tree, not from the rule — the rule is unchanged from W1c's, only its code is new |
| `schema-export-unknown` | 21 | 0 | fixed by the owning partitions |
| `schema-file-missing` | 31 | 3 | idem |
| `schema-dialect-not-draft-07` | 173 | 162 | idem |
| `schema-placement-outside-module` | 55 | 51 | idem |
| `schema-fixture-defines-schema` | 157 | 156 | idem |
| `schema-owner-ineligible` / `schema-placement-forbidden-filename` | 69 / 18 | 69 / 18 | unchanged |
| `schema-catalog-malformed`, `schema-cross-scope-dependency-forbidden` | 0 / 0 | 0 / 0 | still clean |

**Sizing the new code.** 152 scopes provide a `🟦️.ts`, carrying 2 349 exports between them. 471 of those
exports have no TypeScript entity at all (`schema-export-incomplete`), 1 696 have the type and no parser
(`schema-export-parser-missing`, spread over **734 distinct files** in **144 scopes**; the densest are
`🏛️architect/…/🔺️diff/🟦️.ts` 133, `💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts` 51 and
`📸️remodel/…/📸️snapshot/🟦️.ts` 43), and **182 are complete**. Repo-wide
there are 318 `parse<Export>` entry points in `🟦️.ts` files today. So the rule is not "nobody wrote
parsers" — it is "the convention exists and roughly one export in thirteen follows it". Owners clear
their rows in wave 3, either by writing the parser or by annotating the export's real format support
with `x-semio-formats` (row 55 / W6c).

Per partition owner (rollup by `<ticket>/wp1d-owner-counts.py`, full rows in
`🗑️generated/wp1d-test-schema.json`; a finding with no `path` is bucketed through its scope's catalog
`path`, so nothing is unattributed):

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
| `🧰️framework/🛍️products/🦑️repo` | 106 | fixture-defines-schema=73 placement-forbidden-filename=16 placement-outside-module=8 dialect-not-draft-07=8 owner-ineligible=1 |
| `🧰️framework/🔨️modules` | 105 | ref-unresolved=67 export-incomplete=32 export-parser-missing=2 placement-outside-module=1 placement-forbidden-filename=1 owner-ineligible=1 fixture-defines-schema=1 |
| `✏️s/🔌️plugins/🔱️trinity` | 98 | export-incomplete=51 export-parser-missing=43 dialect-not-draft-07=4 |
| `✏️s/🔌️plugins/🌀️procedural` | 94 | export-incomplete=52 export-parser-missing=38 dialect-not-draft-07=4 |
| `✏️s/🔌️plugins/🌍️gis` | 83 | export-incomplete=42 export-parser-missing=26 dialect-not-draft-07=8 file-missing=3 owner-ineligible=2 fixture-defines-schema=2 |
| `✏️s/🔌️plugins/📏️layout` | 77 | export-incomplete=61 export-parser-missing=14 dialect-not-draft-07=2 |
| `✏️s/🔌️plugins/🎥️shooting` | 73 | export-incomplete=42 export-parser-missing=30 dialect-not-draft-07=1 |
| `✏️s/🔌️plugins/💠️lowpoly` | 64 | export-incomplete=34 export-parser-missing=28 dialect-not-draft-07=2 |
| `✏️s/🔌️plugins/📐️cad` | 64 | export-incomplete=42 export-parser-missing=21 dialect-not-draft-07=1 |
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

The repository test module's own subtree still carries **0** findings, asserted green by
`🩺️ the test platform's own subtree carries no schema-contract finding` — `repo.test` is a
JSON-Schema-only scope, so the parser rule does not reach it.

Every code emitted over the live tree is one the exported table declares (checked directly against the
report: `emitted − table = ∅`).

---

## 6. Cross-partition requests

| id | To | Request |
|---|---|---|
| **R-1** | W2u (tooling, `📚️library/🔍️discovery/🟦️.ts`) | Row 97 is ready on this side: import `SCHEMA_DIAGNOSTIC_CODE_TABLE` (§4) instead of the literal codes at `:3273` (`mutation-leaf-id-grammar` → `schema-mutation-leaf-id`) and `:3346` (`export-format-missing` → `schema-export-incomplete`), and update `📚️library/🧪️tests/🧬️schema-scope-catalog/🧫️fixtures/🔣️.json`. Note the table has **no** `schema-mutation-leaf-id` yet — the harness derives leaf ids and does not diagnose them; say the word and I add the code so your rename has a target. |
| **R-2** | W2u (tooling) | `schema-export-parser-missing` is new and the generator has no counterpart. Do not fold it into `export-format-missing`/`schema-export-incomplete`: those two are "no entity at all". |
| **R-3** | W2c (`📚️library/🧪️tests/🧬️schema-scope-catalog/🧫️fixtures/🔣️.json`) | **W1b R-4 / W1c R-3, still the only red.** Case `fixture-defines-schema` expects `placementPaths` without `🌎️hub/💡️inference/🧪️fixtures/🧫️approval/🧬️schema/🔣️.json`, contradicting contract §C. Either expect both paths, or have the case declare `inertSchemaData`. |
| **R-4** | wave 3, per partition | 1 696 `schema-export-parser-missing` over 734 files, per-owner rows in §5.4. Each owner either writes `parse<Export>()` beside the type, or annotates the export with `x-semio-formats` naming the formats it honestly supports (which, per Q1, must include `🔣️jsonschema`). |
| **R-5** | W6c (plugins) | Unchanged from W1c R-4: `x-semio-formats` is implemented, tested and still carried by **no** export in the tree. Row 55's annotations can land now, and they are the cheap half of R-4. |
| **R-6** | the peer refactoring this partition (row 62) | `📦️packages/🟦️typescript/tsconfig.json` still `include`s only `🟦️.ts`, so the relocated suites are not type-checked by the package's `lint` target (W1b R-6, W1c R-6 — unchanged). |

---

## 7. Open questions

1. **Should `parse<Export>()` be required to be a *function*?** Today `export const parseX = …` counts,
   which admits a non-callable constant. Checking callability means parsing TypeScript, which the harness
   does not do and should not start doing for this; the alternative is to accept only the
   `function`/`async function` forms and make the arrow form a finding. Currently permissive.
2. **`schema-mutation-leaf-id`.** Row 97 asks the tooling side to rename its `mutation-leaf-id-grammar`
   into this vocabulary, but the harness does not own that diagnostic (it owns the derivation,
   `mutationLeafSchemaId`) and therefore does not declare the code. Either the table gains a code nothing
   in the harness emits, or row 97's rename list drops that entry. I have left the table emitting-only.
3. **`dependsOn` for mutation aggregates** (W1c §7.2, unchanged). Once the generator catalogues aggregates
   and leaves as scopes (row 11), every aggregate needs each leaf scope in its `dependsOn` or the
   aggregates report `schema-cross-scope-dependency-forbidden` in bulk. Zero today because the leaves are
   not yet catalogued as scopes.
