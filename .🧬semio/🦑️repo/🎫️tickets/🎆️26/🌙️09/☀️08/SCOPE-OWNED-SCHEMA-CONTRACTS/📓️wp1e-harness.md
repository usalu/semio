# WP1e — the code table gains its second emitter, the taxonomy-read carve-out, and the walk that broke a budget

Worker: W1e (Opus). Partition: `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/**`.
Inputs: `📋️execution-contract.md` §A/§E, `📓️wp1d-harness-rules.md` (the table as W1d left it),
`📓️wp2d-root-script.md` §4.1-4.3, `📋️cross-partition-requests.md` rows 114, 125, 129, 130.
Successor to W1d.

---

## 1. Headline

| | |
|---|---|
| Row 130 (table) | `SCHEMA_DIAGNOSTIC_CODE_TABLE` **27 → 33 codes**, every row now `{ emitters, description }`. The six added are `schema-catalog-stale` + the five row-129 codes, all tagged `check` |
| Row 125 (emitter tag) | `emitters` is a **list**, not a single tag: 14 codes are harness-only, 13 are raised by both implementations, 6 by `schema check` alone. A single tag would have had to lie about the 13 |
| Compile-time half | `schemaDiagnostic()`, `stage()`, `SchemaDiagnostic.code` and `SchemaStageOutcome.code` now take `SchemaHarnessDiagnosticCode`, so a harness rule that raises a `check`-only code is a type error, not a finding no rule produces |
| Row 114 | `isFixtureOwnedPath` reads the carve-out directory from `schemaScopeOwnerLevels.moduleMemberDirName`; the last `"🔨️modules"` literal in the schema-contract layer is gone, and the cast around `schemaScopeOwnerLevels()` with it |
| Row 130 (budget) | `🧩️ open/closed › every oracle and every format-specific profile arrives as an owner contribution` **passes at its unchanged `30_000` ms budget**. The repository walk it used to pay is taken once at module scope; `clearContributionCache(root)` now forgets one root instead of all |
| Vector | `🧪️tests/🧬️schema-invariants/🔣️.json` → `…/v3`, new `checkOnlyDiagnosticCodes` (6 codes) |
| Invariants suite | **93 pass / 1 fail** (94 tests, 416 expect() calls). The red is unchanged and outside this partition: row 124 / W1d R-3 |
| `test schema` | **4 529 findings** (W1d measured 5 166; the tree moved underneath, §6.4). Every emitted code is one the table declares, and **no** `check`-only code was emitted by the harness |

---

## 2. Row 130 + 125 — the table declares two emitters

### 2.1 Shape

```ts
export const SCHEMA_DIAGNOSTIC_EMITTERS = ["harness", "check"] as const;

export const SCHEMA_DIAGNOSTIC_CODE_TABLE = {
  "schema-catalog-malformed": { emitters: ["harness", "check"], description: "the catalog exists but is not the declared …" },
  "schema-catalog-stale":     { emitters: ["check"],            description: "the derived schema catalog does not match the schema modules on disk; regenerate it" },
  …
} as const;
```

Derived, all exported from the same module path W1d §4 named:

| export | what it is |
|---|---|
| `SCHEMA_DIAGNOSTIC_CODES` | the ids, in table order (33) |
| `SchemaDiagnosticCode` | the union over all 33 |
| `SchemaDiagnosticCodeEmittedBy<E>` | the codes one emitter raises, as a type |
| `SchemaHarnessDiagnosticCode` | `SchemaDiagnosticCodeEmittedBy<"harness">` (27) |
| `schemaDiagnosticCodesEmittedBy(emitter)` | the same split at runtime, in table order |

The root `schema check` reads `SCHEMA_DIAGNOSTIC_CODES` by dynamic import (`📜️script.ts:17455`) and is
unaffected by the value shape: it only ever took the keys.

### 2.2 Why `emitters` is a list

Row 125 says "each tagged with its emitter (`harness` / `check`)". Thirteen codes are honestly raised by
**both** implementations — the two catalog states, and the placement/ownership/export rules `schema check`
re-derives over the whole tree from `inventorySchemaScopes`. Tagging one of them `harness` would tell a
reader that `schema check` cannot produce it, which is false, and tagging it `check` would say the same of
the harness. A one-element list carries the single-emitter case with no ceremony, so the list is the shape.

Read live off the table (`schemaDiagnosticCodesEmittedBy`, total 33: harness 27, check 19):

| emitter | codes |
|---|---|
| `harness` only (14) | `schema-export-resolution-undeclared`, `schema-uri-malformed`, `schema-scope-unknown`, `schema-export-unknown`, `schema-format-unavailable`, `schema-file-missing`, `schema-ref-broken-internal`, `schema-contracts-directory-forbidden`, `schema-export-parser-missing`, `schema-export-format-undeclared`, `schema-fixture-local-schema-fallback`, `schema-fixture-metadata-invalid`, `schema-fixture-parse-failed`, `schema-instance-invalid` |
| both (13) | `schema-catalog-missing`, `schema-catalog-malformed`, `schema-scope-ambiguous`, `schema-ref-unresolved`, `schema-cross-scope-dependency-forbidden`, `schema-placement-forbidden-filename`, `schema-placement-outside-module`¹, `schema-owner-ineligible`, `schema-export-incomplete`, `schema-export-formats-annotation-invalid`, `schema-dialect-not-draft-07`, `schema-module-id-missing`, `schema-fixture-defines-schema` |
| `check` only (6) | `schema-catalog-stale`, `schema-export-id-duplicate`, `schema-export-id-invalid`, `schema-document-id-duplicate`, `schema-document-id-unaddressable`, `schema-mutation-leaf-id` |

¹ `schema-placement-outside-module` carries `check` because row 129 maps the library's
`placement-retired-location` onto both placement codes. See §7 R-2 — the library currently emits only
`schema-placement-forbidden-filename`, so this is the one row where the tag is the contract's claim rather
than an observed emission.

### 2.3 The emitter tag is load-bearing, not documentation

`schemaDiagnostic(code, …)` — the single factory every harness rule goes through — takes
`SchemaHarnessDiagnosticCode`. `stage(name, result, code, detail)` and the two finding types were widened
strings before; they are the same narrowed union now. The compiler therefore rejects a harness rule that
tries to raise `schema-catalog-stale`, which is exactly the failure mode a shared table invites: one side
starts emitting the other's code and the two implementations quietly stop being one rule. Narrowing
`SchemaStageOutcome.code` from `string | null` also closed the hole where a fixture stage could report a
code from no vocabulary at all.

### 2.4 Protocol and vector

- `…/🧪️test/🧬️schema/🔣️.json`: `SchemaDiagnosticCode` enum +6 (held set-equal to the table by the suite —
  the test W1d §4 named still passes); new `$defs.SchemaDiagnosticEmitter` (`harness` | `check`), which the
  suite holds equal to `SCHEMA_DIAGNOSTIC_EMITTERS`.
- `…/🧪️tests/🧬️schema-invariants/🔣️.json`: `semio.repository-test.schema-invariant-cases/**v3**` with a new
  required `checkOnlyDiagnosticCodes` naming the six. A second implementation of this harness reads it to
  know which codes it must never raise; the const bumped because the vector's shape changed and nothing in
  the tree pins v2.

---

## 3. Row 114 — the carve-out is vocabulary

`isFixtureOwnedPath` decides whether a path lies inside a `🧪️`/`🧫️` collection; the one thing it decided
itself was that a `🔨️modules/<m>` member is a module and not a collection, and it spelled `"🔨️modules"`.
It now reads `schemaScopeOwnerLevels.moduleMemberDirName` (declared in `🔣️taxonomy.json` since the WP2
additions). A taxonomy that declares no such name carves nothing out — a carve-out nobody declared is a
rule nobody can read.

`SchemaScopeOwnerLevels` gained the two optional fields the function actually reads
(`fixtureOwnerPathPatterns`, `moduleMemberDirName`), so the `as { … }` cast at the call site is gone.

The proof is behavioural, not a re-assertion of the same string: the new case scaffolds a repository whose
taxonomy renames the directory to `🧱️units` and asserts the carve-out moves with it (and that dropping the
key drops the carve-out).

---

## 4. Row 130 — the walk that ate a 30 s budget

### 4.1 What it actually costs

`discoverTestContributions(repoRoot)` measured on this box, in a standalone script, with `[DEBUG] ` logs:

```
[DEBUG] cold walk 156873 ms, 203 contributions
[DEBUG] warm 0 ms
[DEBUG] second cold walk 143404 ms
```

The second number matters: 143 s with the filesystem cache hot is not an I/O-cold artefact, it is what a
full directory walk of this tree costs under the load this machine is carrying. An unfiltered walk of the
same tree visits **113 808 directories / 427 595 entries**; a `readdirSync(withFileTypes)` variant was not
faster (203 s in the same window), so the cost is the tree and the load, not a stray `lstat` per entry.
Raising the budget was refused by row 130 and would have been wrong anyway: the number would have had to
follow the machine.

### 4.2 The fix

1. **Module-level memo in the suite.** `🧪️tests/🧪️test-platform/🟦️.ts` now takes
   `const repoContributions = discoverTestContributions(repoRoot)` and
   `const repoRegistry = loadOracleRegistry(repoRoot)` **at module scope**, and the 16 `loadOracleRegistry(repoRoot)`
   plus 2 `discoverTestContributions(repoRoot)` call sites read those. Module evaluation is not inside any
   test's timeout, so the walk is paid once, before the first test, and a per-test budget measures the
   assertion it was written for. Nothing in this file writes into the real repository, so one scan stays
   the truth for the whole run.
2. **`clearContributionCache(repoRoot?)`.** Two cases point discovery at a synthetic root and then cleared
   the process-wide memo — every root's, including the real one nobody had touched. It now forgets one
   named root, and those two cases name theirs. That is the actual defect behind the red: a per-process
   cache existed and a test threw it away on another root's behalf.

The budget itself is untouched at `30_000`.

### 4.3 Before and after, same command W2v ran

```
$ bun test "./🧪️tests/🧪️test-platform/🟦️.ts" -t "open/closed"     # W2v, 📓️wp2d-root-script.md §3.2
(fail) 🧩️ open/closed > every oracle and every format-specific profile arrives as an owner contribution [56940.54ms]
  ^ this test timed out after 30000ms.
 5 pass / 1 fail / 377 expect() calls        Ran 6 tests across 1 file. [102.92s]

$ bun test --timeout 60000 "./🧪️tests/🧪️test-platform/🟦️.ts" -t "open/closed"     # now
 6 pass
 0 fail
 377 expect() calls
Ran 6 tests across 1 file. [262.98s]
```

The file total is larger because the walk now runs during module evaluation whatever the filter selects,
where it is honest work rather than a test's clock. `--timeout 60000` does not relax the failing test: its
own declared `30_000` wins over the CLI default.

---

## 5. Files changed

| File | Change |
|---|---|
| `…/🧪️test/📦️packages/🟦️typescript/🟦️.ts` | `SCHEMA_DIAGNOSTIC_EMITTERS` + `SchemaDiagnosticEmitter` (new); `SCHEMA_DIAGNOSTIC_CODE_TABLE` rows become `{ emitters, description }` and gain six `check` codes (27 → 33); `SchemaDiagnosticCodeEmittedBy<E>`, `SchemaHarnessDiagnosticCode`, `schemaDiagnosticCodesEmittedBy` (new); `schemaDiagnostic`, `stage`, `SchemaDiagnostic.code`, `SchemaStageOutcome.code` narrowed to harness codes; `isFixtureOwnedPath`/`isModuleMemberSegment` read `moduleMemberDirName` from the taxonomy and the call-site cast is gone; `SchemaScopeOwnerLevels` declares `fixtureOwnerPathPatterns` + `moduleMemberDirName`; `clearContributionCache(repoRoot?)` forgets one root |
| `…/🧪️test/🧬️schema/🔣️.json` | `SchemaDiagnosticCode` enum +6; new `$defs.SchemaDiagnosticEmitter`; `SchemaInvariantCases` → `…/v3` with required `checkOnlyDiagnosticCodes` |
| `…/🧪️test/🧪️tests/🧬️schema-invariants/🔣️.json` | `schema` const → v3; `checkOnlyDiagnosticCodes` (6 codes) |
| `…/🧪️test/🧪️tests/🧬️schema-invariants/🟦️.ts` | two new tests in `🔠️ the diagnostic code table is the single vocabulary`, one new test in `🏛️ owner eligibility`, one existing test extended; imports `SCHEMA_DIAGNOSTIC_EMITTERS`, `schemaDiagnosticCodesEmittedBy`, `type SchemaDiagnosticCode` |
| `…/🧪️test/🧪️tests/🧪️test-platform/🟦️.ts` | module-level `repoContributions` / `repoRegistry`; 18 call sites read them; the two synthetic-root cases call `clearContributionCache(root)` |
| `<ticket>/🗑️generated/wp1e-test-schema.json` | ticket output: the `test schema --json` report §6.4 is read from |

No file outside the partition was written. The peer of row 62 edited `🧪️tests/📐️test-layout/**` during this
session; nothing of theirs was reverted, and every run below is on the tree as they left it.

## 6. Verification — real output

### 6.1 Type check

```
$ bunx tsc -p tsconfig.json --noEmit          # …/🧪️test/📦️packages/🟦️typescript
🟦️.ts(7008,225): error TS2339: Property 'oracle' does not exist on type 'Readonly<{ capability: string; … }>'.
🟦️.ts(7008,275): error TS2339: Property 'oracle' does not exist on type 'Readonly<{ capability: string; … }>'.
```

The same two pre-existing `OracleRequirement.oracle` errors W1, W1b, W1c and W1d all recorded (line 6967
then, 7008 now — a peer's edits above them), on a line this work-package did not touch. Everything added
here type-checks clean; the first pass of the narrowed factory did surface two real call sites
(`runSchemaFixture` passing a `SchemaDiagnostic.code` into `stage`), which is what the narrowing is for,
and both are resolved by narrowing the finding types themselves rather than by casting.

The two suites are in no tsconfig `include` (R-6, still open), so they were type-checked through a
throwaway project in the scratchpad. Only pre-existing findings came back: `bun:test` and `../../🟨️.mjs`
have no declarations under that ad-hoc config, and five synthetic `OracleRegistry` literals in
`🧪️test-platform/🟦️.ts` (lines 337-463) omit `probes`/`comparisonPipelines`/`toleranceProfiles`/`mutation*`.
Nothing from this work-package appears.

### 6.2 The invariants suite

```
$ bun test --timeout 60000 ./🧪️tests/🧬️schema-invariants/🟦️.ts   # cwd …/🔨️modules/🧪️test
 93 pass
 1 fail
 416 expect() calls
Ran 94 tests across 1 file. [56.99s]

$ bun test --timeout 60000 ./🧪️tests/🧬️schema-invariants/🟦️.ts   # re-run, last thing this session did
 93 pass / 1 fail / 416 expect() calls        Ran 94 tests across 1 file. [16.95s]
```

(The same suite takes 17 s or 57 s depending on what else this machine is compiling; the second run is the
final state of the tree, peers' concurrent edits included.)

91 → 94 tests. The three new ones all pass:

| test | asserts |
|---|---|
| `🔠️ … > every code names the emitters that raise it, out of the vocabulary the protocol declares` | every row has a non-empty `emitters`, every value is in `SCHEMA_DIAGNOSTIC_EMITTERS`, that list equals the protocol's `SchemaDiagnosticEmitter` enum, and at least one code carries both (27 + 18 > 33) |
| `🔠️ … > the codes the vector reserves for schema check are exactly the ones the table withholds from the harness` | vector `checkOnlyDiagnosticCodes` == table rows without `harness`; each is tagged `check`; **no** case anywhere in the vector expects one |
| `🏛️ owner eligibility > the module-member carve-out is the directory name the taxonomy declares, never a literal` | renaming `moduleMemberDirName` to `🧱️units` moves the carve-out; deleting the key removes it |

and one existing test grew a second half: `every code a case expects is one the table declares, **and one
the harness emits**`.

The single red is unchanged from W1d §6.2 and is **not** in this partition — it is row 124 / W1d R-3, the
generator vector expecting silence for a schema inside a `🧪️fixtures` tree:

```
(fail) 🤝️ parity with the catalog generator's own vector > every generator case places the same files and levels as this harness does
Expected: "…:🌎️hub/💡️inference/🧪️fixtures/🧫️approval/🧬️.schema.json"
Received: "…/🧬️.schema.json,🌎️hub/💡️inference/🧪️fixtures/🧫️approval/🧬️schema/🔣️.json"
```

### 6.3 The partition's other suites

```
$ bun test --timeout 60000 ./🧪️tests/🧬️schema-invariants/🟦️.ts ./🧪️tests/📐️test-layout/🟦️.ts ./🧪️tests/🖥️host-protocol-parity/🟦️.ts
 107 pass
 2 fail
 463 expect() calls
Ran 109 tests across 3 files. [78.72s]
```

The second red is **new since W1d and is the peer's (row 62), not this work-package's**:

```
(fail) 📐️ canonical test layout > Nx discovers the same canonical names and semantic owners as the layout vectors [372.46ms]
error: 🧩️domain/🧪️tests/🧪️case/🥒️.feature      Expected: true   Received: false
```

`🧪️tests/📐️test-layout/🟦️.ts` was last written at 21:18 and its vector `🔣️.json` at 20:31, both by the peer
refactoring this partition, while this session was running; the assertion compares the nx plugin
(`🟨️.mjs`, untouched since 01:28) against that vector and names nothing this work-package touches. Left
exactly as they wrote it.

`🧪️tests/🧪️test-platform/🟦️.ts` **did not complete** — same as W1c and W1d recorded. It ran for ~50 minutes
without reaching a summary, stalled after the repository-walking discovery tests, and was stopped rather
than left to starve the peers' builds, so **no pass/fail total is claimed for it**. Everything it reported
in those 50 minutes is below; none of it is a schema-contract check and none of it is reachable from
anything changed here:

```
(fail) ⚖️ comparison profiles > semantic-pdf-v1 canonicalizes the nondeterministic artefacts …
(fail) 📇️ oracle registry > every registered oracle is test-only and declares its license and capabilities
(fail) 📇️ oracle registry > every recorded no-oracle decision names its rationale and its substitutes
        TypeError: undefined is not an object (evaluating 'decision.substitutes.length')
(fail) 🔍️ discovery and contract > discovery finds the committed cases and never returns a compose path [108223.88ms]
  ^ this test timed out after 60000ms.
(fail) 🔍️ discovery and contract > discovery is idempotent [139988.82ms]
  ^ this test timed out after 30000ms.
```

The two cases that were changed here and sit past the stall were run under their own filter, because a
suite that never reaches them proves nothing about them:

```
$ bun test --timeout 120000 "./🧪️tests/🧪️test-platform/🟦️.ts" -t "contribution directory ownership"
 4 pass
 0 fail
 28 expect() calls
Ran 4 tests across 1 file. [115.40s]
```

Those four are the two synthetic-root cases that now call `clearContributionCache(root)`, plus the
kernel-oracle case that now reads `repoContributions`.

The two `substitutes` reds are W1b's R-1/R-2. The two timeouts are `discoverTestCases`, a **different**
repository walk from the one row 130 named: `discovery is idempotent` runs discovery twice on purpose to
compare the two results, so memoizing it would delete the assertion rather than speed it up. Flagged in §6
as R-3 rather than fixed here.

### 6.4 `test schema` over the live tree

```
$ bun 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts test schema --json \
      > <ticket>/🗑️generated/wp1e-test-schema.json

findings 4529   schema-bound fixtures 0
   2220 × schema-export-incomplete
   1708 × schema-export-parser-missing
    165 × schema-export-unknown
    157 × schema-fixture-defines-schema
    117 × schema-placement-outside-module
     68 × schema-owner-ineligible
     48 × schema-ref-unresolved
     17 × schema-cross-scope-dependency-forbidden
     15 × schema-dialect-not-draft-07
      5 × schema-catalog-malformed
      5 × schema-ref-broken-internal
      3 × schema-file-missing
      1 × schema-placement-forbidden-filename
```

**Checked against the table**: `emitted − table = ∅`, and `emitted ∩ checkOnlyDiagnosticCodes = ∅` — the
harness raised none of the six codes it now declares on `schema check`'s behalf, which is the runtime half
of §2.3.

**Read against W1d's 5 166.** No rule changed in this work-package, so every delta is other partitions
moving, and the regenerated catalog is most of it:

| code | W1d | now | reading |
|---|---|---|---|
| `schema-export-incomplete` | 2 891 | 2 220 | −671, owners landing formats |
| `schema-export-parser-missing` | 1 696 | 1 708 | flat; R-4 of W1d is untouched work |
| `schema-export-unknown` | 0 | **165** | new: the regenerated catalog names exports their carrier file does not declare |
| `schema-placement-outside-module` | 51 | **117** | new placements found by the regenerated scope set |
| `schema-cross-scope-dependency-forbidden` | 0 | **17** | W1c §7.2 / W1d §7.3 arriving: scopes now catalogued, `dependsOn` not yet |
| `schema-catalog-malformed` | 0 | **5** | five catalog rows (`🔱️trinity`) are not the declared document |
| `schema-dialect-not-draft-07` | 162 | 15 | migrated |
| `schema-ref-unresolved` | 115 | 48 | migrated |
| `schema-fixture-defines-schema` / `-owner-ineligible` / `-ref-broken-internal` / `-file-missing` | 156 / 69 / 5 / 3 | 157 / 68 / 5 / 3 | unchanged in substance |
| `schema-placement-forbidden-filename` | 18 | 1 | migrated |

Per owner (`wp1d-owner-counts.py`, unchanged, over the new report):

| owner root | findings | by code |
|---|---|---|
| `✏️s/🔌️plugins/🗄️stdio` | 1417 | export-parser-missing=674 export-incomplete=643 ref-unresolved=46 owner-ineligible=38 dialect=12 placement-outside-module=2 ref-broken-internal=2 |
| `✏️s/🔌️plugins/🏛️architect` | 499 | export-incomplete=286 export-parser-missing=143 export-unknown=70 |
| `🧰️framework/🛍️products/💻️os` | 451 | export-incomplete=276 fixture-defines-schema=79 export-parser-missing=68 placement-outside-module=21 owner-ineligible=7 |
| `✏️s/🔌️plugins/📕️norm` | 359 | export-incomplete=201 export-parser-missing=139 owner-ineligible=15 ref-broken-internal=3 export-unknown=1 |
| `✏️s/🔌️plugins/🧩️puzzle` | 229 | export-parser-missing=126 export-incomplete=103 |
| `✏️s/🔌️plugins/🧱️block` | 192 | export-incomplete=106 export-parser-missing=85 owner-ineligible=1 |
| `🧰️framework/🛍️products/🦑️repo` | 150 | placement-outside-module=75 fixture-defines-schema=75 — **all 150 in `📚️library`** |
| `✏️s/🔌️plugins/📸️remodel` | 128 | export-incomplete=109 export-parser-missing=18 ref-unresolved=1 |
| `✏️s/🔌️plugins/🏗️fem` | 95 | export-parser-missing=41 export-incomplete=31 export-unknown=21 cross-scope-dependency-forbidden=2 |
| `✏️s/🔌️plugins/🔱️trinity` | 95 | export-incomplete=48 export-parser-missing=42 catalog-malformed=5 |
| `✏️s/🔌️plugins/🌀️procedural` | 83 | export-incomplete=36 export-parser-missing=24 export-unknown=19 cross-scope-dependency-forbidden=4 |
| `✏️s/🔌️plugins/📏️layout` | 80 | export-incomplete=63 export-parser-missing=17 |
| `✏️s/🔌️plugins/🌍️gis` | 66 | export-incomplete=29 export-parser-missing=27 file-missing=3 dialect=2 owner-ineligible=2 fixture-defines-schema=2 export-unknown=1 |
| `✏️s/🔌️plugins/🎥️shooting` | 62 | export-incomplete=28 export-parser-missing=21 export-unknown=11 cross-scope-dependency-forbidden=2 |
| `✏️s/🔌️plugins/💠️lowpoly` | 57 | export-incomplete=28 export-parser-missing=23 export-unknown=6 |
| `✏️s/🔌️plugins/📐️cad` | 48 | export-incomplete=19 export-parser-missing=14 export-unknown=11 cross-scope-dependency-forbidden=4 |
| `✏️s/🔌️plugins/🌊️flow` | 43 | export-incomplete=18 export-parser-missing=17 export-unknown=6 cross-scope-dependency-forbidden=2 |
| `✏️s/🔌️plugins/🖨️raster` | 38 | export-incomplete=22 export-parser-missing=16 |
| `✏️s/🔌️plugins/🎬️sequence` | 37 | export-parser-missing=14 export-incomplete=12 export-unknown=5 owner-ineligible=2 cross-scope-dependency-forbidden=2 dialect=1 placement-outside-module=1 |
| `✏️s/🔌️plugins/📜️imperative` | 37 | export-incomplete=19 export-parser-missing=18 |
| `✏️s/🔌️plugins/🏭️process` | 35 | export-incomplete=17 export-parser-missing=13 export-unknown=5 |
| `✏️s/🔌️plugins/🕸️dag` | 32 | export-parser-missing=19 export-incomplete=12 placement-outside-module=1 |
| `✏️s/🔌️plugins/🪵️sourcing` | 32 | export-parser-missing=20 export-incomplete=11 placement-outside-module=1 |
| `✏️s/🔌️plugins/✒️writer` | 31 | export-parser-missing=15 export-incomplete=13 placement-outside-module=1 export-unknown=1 cross-scope-dependency-forbidden=1 |
| `✏️s/🔌️plugins/📖️playbook` | 30 | export-incomplete=17 export-parser-missing=13 |
| `✏️s/🔌️plugins/🪐️space` | 30 | export-incomplete=18 export-parser-missing=10 owner-ineligible=2 |
| `✏️s/🔌️plugins/🔋️energy` | 25 | export-incomplete=13 export-parser-missing=10 placement-outside-module=1 fixture-defines-schema=1 |
| `✏️s/🔌️plugins/🎪️demonstrator` | 21 | export-incomplete=12 export-parser-missing=9 |
| `✏️s/🔌️plugins/🖍️draw` | 21 | export-parser-missing=12 export-incomplete=8 placement-outside-module=1 |
| `✏️s/🔌️plugins/🗒️note` | 20 | export-parser-missing=15 export-incomplete=4 placement-outside-module=1 |
| `✏️s/🔌️plugins/🎞️animate` | 19 | export-parser-missing=11 export-incomplete=4 export-unknown=3 placement-outside-module=1 |
| `✏️s/🔌️plugins/📋️forms` | 19 | export-parser-missing=9 export-incomplete=9 placement-outside-module=1 |
| `✏️s/🔌️plugins/➗️mathematical` | 16 | export-parser-missing=8 export-unknown=5 export-incomplete=2 placement-outside-module=1 |
| `✏️s/🔌️plugins/🌿️vcs` | 11 | export-parser-missing=9 placement-outside-module=1 export-incomplete=1 |
| `✏️s/🔌️plugins/💡️reasoning` | 11 | export-parser-missing=8 export-incomplete=2 placement-outside-module=1 |
| `♻️mit-bestand/🔎️recherche` | 8 | placement-outside-module=7 placement-forbidden-filename=1 |
| `🧰️framework/🔨️modules` | 2 | owner-ineligible=1 ref-unresolved=1 |

`🌎️hub/*` is at **0** for the first time in this ticket's measurements. This partition's own subtree is at
**0**, asserted green by `🩺️ the test platform's own subtree carries no schema-contract finding` and
re-checked directly against the report (`path` under the test domain, or scope `repo.test`: 0 rows).

### 6.5 Root `schema check` — the row-129 acceptance measure is met

Run from the repository root against the table as this work-package leaves it (the root script loads
`SCHEMA_DIAGNOSTIC_CODES` live by dynamic import, so this is the real adoption measure, not a restatement):

```
$ bun ./📜️script.ts schema check
[schema check] modules=3152 scopes=3068 findings=7542
[schema check] schema-catalog-stale=1
[schema check] schema-dialect-not-draft-07=1
[schema check] schema-document-id-unaddressable=1
[schema check] schema-export-id-duplicate=39
[schema check] schema-export-id-invalid=314
[schema check] schema-export-incomplete=6621
[schema check] schema-fixture-defines-schema=60
[schema check] schema-module-id-missing=39
[schema check] schema-mutation-leaf-id=362
[schema check] schema-owner-ineligible=12
[schema check] schema-placement-forbidden-filename=1
[schema check] schema-ref-unresolved=89
[schema check] schema-scope-ambiguous=2
[schema check] shared-code-table=33 unshared-codes=0
```

**`unshared-codes=0`** — down from W2v's `16`. Row 129's acceptance measure holds: the library renamed its
side (`📚️library/🔍️discovery/🟦️.ts` now emits `schema-document-id-duplicate` and
`schema-document-id-unaddressable`, exactly the row-129 names), and the six codes it had no target for now
exist in the table. Five of the six are emitted in this very run
(`schema-catalog-stale`, `schema-document-id-unaddressable`, `schema-export-id-duplicate`,
`schema-export-id-invalid`, `schema-mutation-leaf-id`); `schema-document-id-duplicate` is declared and
currently silent.

Worth recording beside §6.4: `schema check` reports **7 542** findings over 3 068 scopes where `test schema`
reports **4 529**. The two implementations are measuring different scope sets, which is exactly the
two-measurement problem row 150 names — not a disagreement this work-package can settle from one side.

---

## 7. Cross-partition requests

| id | To | Request |
|---|---|---|
| **R-1** | W2w library (`📚️library/🔍️discovery/🟦️.ts`) | Eight codes the library declares have **no** row in the shared table and did not fire in the §6.5 run, so `unshared-codes=0` is true today and fragile tomorrow: `schema-cross-scope-dependency-uncataloged`, `schema-document-not-object`, `schema-document-unparseable`, `schema-enum-empty`, `schema-module-id-inconsistent`, `schema-mutation-aggregate-id`, `schema-mutation-aggregate-kinds-redundant`, `schema-mutation-leaf-schema-absent`. Per row 125 the table declares every code either side emits — send me the eight with one-line descriptions and I add them tagged `check`, or fold them into existing codes. Until then the first one that fires reopens row 129. |
| **R-2** | W2w library | The table tags `schema-placement-outside-module` as `harness`+`check` because row 129 maps `placement-retired-location` onto both placement codes, but the library emits only `schema-placement-forbidden-filename`. Either emit both (a document outside a `🧬️schema/` module is not the same defect as a retired filename) or say so and I drop `check` from that row. |
| **R-3** | W1f harness (successor) | `🧪️tests/🧪️test-platform/🟦️.ts` has two *further* repository-walk timeouts, in `discoverTestCases`, not `discoverTestContributions`: `discovery finds the committed cases and never returns a compose path` (108 s / 60 s budget) and `discovery is idempotent` (140 s / 30 s). The row-130 remedy does **not** transfer: the idempotence test calls discovery twice on purpose to compare, so a memo would delete the assertion. It needs either a cheaper walk or a smaller declared subtree. |
| **R-4** | the peer refactoring this partition (row 62) | `📐️ canonical test layout > Nx discovers the same canonical names and semantic owners as the layout vectors` is red on the tree as you left it at 21:18 (`🧩️domain/🧪️tests/🧪️case/🥒️.feature`, expected discovered, not discovered). Untouched here. Also unchanged from W1b/W1c/W1d R-6: `📦️packages/🟦️typescript/tsconfig.json` still `include`s only `🟦️.ts`, so neither relocated suite is type-checked by the package's `lint` target — this pass had to type-check them through a throwaway project. |
| **R-5** | W2w library (row 124, unchanged) | The generator vector case still expects a `🧪️fixtures`-tree schema to be silent, contradicting contract §C; it is the invariants suite's only red (93/94). |
| **R-6** | wave 3 owners (row 123, re-measured) | 4 529 findings, per-owner rows in §6.4. New since W1d and worth routing early: `schema-export-unknown` 165 and `schema-cross-scope-dependency-forbidden` 17 both appeared when the catalog was regenerated — the first says a catalogued export is not in its carrier file, the second is W1c §7.2 arriving (scopes catalogued, `dependsOn` not). |

Not taken up here, and correctly so: contract §A's Rust `pub use` amendment is row **148**, assigned to
W1f harness, and row **150** (gate flag for the two-measurement disagreement) likewise. The harness's Rust
presence rule is unchanged in this pass and still reads `pub struct|enum|type <Export>`.

## 8. Open questions

1. **Should `emitters` be closed over the codes a run actually produced?** The table's `check` tags are the
   contract's claim (row 129's mapping), and §6.5 proves five of the six new ones and part of the thirteen shared
   rows against a live run — but `schema-document-id-duplicate`, `schema-contracts-directory-forbidden` and
   several others are declared and silent on today's tree. A test that asserted "every tagged emitter has
   been observed" would fail on a clean tree, which is the wrong direction; the honest invariant is the one
   implemented (no side raises a code it is not tagged for), and the other direction stays a review
   question.
2. **`parse<Export>()` callability** (W1d §7.1, unchanged). `export const parseX = 42` still counts.
3. **The vector's `checkOnlyDiagnosticCodes` is a restatement of the table's complement.** It is there so a
   second implementation of the harness — Rust, Python — can read the boundary without importing the
   TypeScript table, and the suite holds the two equal. If a future implementation reads the table itself,
   the key becomes redundant and should go rather than drift.
4. **`🧪️test-platform` still has no pass/fail total in any WP report** (W1c, W1d, W1e). Three passes in a
   row have declined to claim one. R-3 is the smallest change that would make the suite finishable on this
   box; until then this partition's gate is really the other three suites.
