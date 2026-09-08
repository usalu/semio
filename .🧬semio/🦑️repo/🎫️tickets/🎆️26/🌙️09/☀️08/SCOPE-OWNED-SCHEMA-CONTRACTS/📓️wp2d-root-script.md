# WP2d — root script partition (rows 113, 59 re-verify, 97 adoption)

Partition: root `📜️script.ts`, root `📋️project.json`, root `package.json`, `.vscode/launch.json`, `nx.json`.
Successor to W2c (`📓️wp2c-root-script.md`). Every number below is a real run on this tree, pasted verbatim.

## 1. Per-row results

| Row | Result |
|---|---|
| 113 (`policyRepositoryOwnedRoots()` → `state === "clean"`) | **done** — R-A applied verbatim, verified running (§3.1) |
| 59 (no test-domain path / phase vocabulary in the root script) | **re-verified green** — both greps 0, harness test passes (§3.2). Re-measured *after* all of this pass's edits |
| 97 (shared diagnostic-code table) | **root half done** — `schema check` reads the harness table and emits `schema-*` for every code it owns; the 15 library-owned codes are named in a new summary line and requested in §4.1 (§3.3) |
| `schema check --report` summary | **done** (§3.4) |

## 2. What changed

### `📜️script.ts`

1. **`policyRepositoryOwnedRoots()`** (`:23481`) — walk roots are now the taxonomy areas whose state is
   `"clean"`, exactly as `📓️wp2c-library-tooling.md` §8 R-A specified:

   ```ts
   const configured = Object.entries(loadTaxonomy().areas ?? {})
     .filter(([, state]) => state === "clean")
     .map(([area]) => area);
   ```

   `🔣️taxonomy.json` declares `".🧬semio/🦑️repo/🎫️tickets": "exempt"` and six `"clean"` areas. Before this
   change the exempt tickets area became a walk root (nothing else is its path prefix, so the existing
   descendant filter did not remove it).

2. **`VerifyScript.runSemanticVocabulary()`** — new `verify semantic-vocabulary` subcommand. It is the
   standalone entry point for `policySemanticVocabularyBreaches`, built on the existing
   `runMutationOutcomeLaw` idiom. It exists because `policyRepositoryOwnedRoots()` has exactly one consumer
   (`policyListSemanticVocabularyScanFiles`, the only call site repo-wide) and that consumer was reachable
   only through the whole `defineLint("@semio-tech/workspace-app-plugin-consistency")` aggregate — i.e. row
   113 had no runnable proof. `high` breaches fail the command; the `Set*` dispatch-variant rule is advisory
   by its own declaration (`policySemanticVocabularyBreaches`'s docstring) and is printed without failing.
   It prints the resolved root list, so the row-113 invariant is observable from the command's own output.

3. **`VerifyScript.runPackagePurity()`** — new `verify package-purity` subcommand. `📋️project.json`'s
   `verify-package-purity` target has been calling `bun ./📜️script.ts policy package-purity`, and **`policy`
   is not a registered command** on the root router (`os semio examples setup start dev generate
   scale-fixture new schema lint verify format test bench stdio build cpp publish purge clean micro-commit
   commit`). The target and its launch entry `📦️verify📦️package🚦️purity` were therefore dead. Fixed in the
   script and in `📋️project.json` (§2.2).

4. **`SchemaScript.check()` — row 97 adoption.** `run` and `check` are now async (`Script.run` is already
   declared `void | Promise<void>` and `ScriptRouter.run` awaits it).
   - The three codes this command owns are renamed to the harness vocabulary:
     `catalog-absent` → **`schema-catalog-missing`**, `catalog-malformed` → **`schema-catalog-malformed`**,
     `catalog-stale` → **`schema-catalog-stale`**. Nothing else in the tree consumed the old names
     (grep over `*.ts *.json *.md *.rs`: only these three lines and W2c's own report).
   - New `SchemaScript.sharedDiagnosticCodes()` loads `SCHEMA_DIAGNOSTIC_CODES` from the harness package by
     dynamic import. The specifier is built as
     `pathToFileURL(join(root, this.testDomainPath(), "📦️packages/🟦️typescript/🟦️.ts"))`, so the root script
     still spells out no test-domain path literal — row 59 stays green (re-measured, §3.2). A missing or
     empty table is a hard error, not a silent skip.
   - `schema check` prints a new final line
     `[schema check] shared-code-table=<n> unshared-codes=<n>[: <codes>]`, naming every code it emitted that
     the shared table does not declare. This is the row-97 conformance signal: a real, running measurement of
     how far the two vocabularies still are apart, rather than a translation table at the boundary (which
     would be exactly the compatibility layer §E forbids).
   - `node:url` import extended with `pathToFileURL`.

### `📋️project.json`

- New target **`semantic-vocabulary`** (`nx:run-commands`, `dependsOn: []`), placed directly after
  `mutation-outcome-law`, whose shape it copies: `bun ./📜️script.ts verify semantic-vocabulary`.
- `verify-package-purity`'s command: `bun ./📜️script.ts policy package-purity` →
  `bun ./📜️script.ts verify package-purity` (the `policy` command does not exist).

### `.vscode/launch.json`

| name | command | group | order |
|---|---|---|---|
| `📦️verify🧬️mutations🚦️semantic-vocabulary` | `bun nx run workspace:semantic-vocabulary` | `4_build` | `209.16` |

Placed between `📦️verify📦️package🚦️purity` (`209.15`) and `📦️verify🏛️workspace🚦️gate` (`209.2`); no order
value had to shift. The name follows the neighbours' `📦️verify<subject><🚦️target>` shape and names the nx
target, as `🚦️purity` ← `verify-package-purity` and `🚦️enforce` ← `verify-taxonomy-enforce` do.
**This edit was silently reverted once** by a concurrent repo-wide sweep of `launch.json` (2 299 → 2 300
configurations, the file is under continuous peer edit); it was re-applied and re-validated as parseable
JSON with no duplicate `name`.

### `package.json`

**Untouched, deliberately.** There is no `verify:*` script group: none of `verify-gate`,
`mutation-outcome-law`, `verify-package-purity`, `verify-taxonomy-report`/`-enforce`,
`verify-rust-warnings`, `verify-layering` has a `package.json` script. Adding one only for
`semantic-vocabulary` would break the existing grouping CLAUDE.md requires following. Flagged as an open
question (§5.1) rather than silently deviating in either direction.

### `nx.json`

Untouched.

## 3. Verification (real output)

### 3.1 Row 113 — the exempt area is no longer a walk root

```
$ bun ./📜️script.ts verify semantic-vocabulary
[verify semantic-vocabulary] roots=♻️mit-bestand, ✏️s, 🌎️hub, 🧰️framework
[verify semantic-vocabulary] scanned=11118 breaches=1468 high=223 advisory=1245
[verify semantic-vocabulary] medium mutation-migration/semantic-vocabulary: "✏️s/🔌️plugins/✒️writer/…/🧬️mutations/⚙️set-editor-settings/🦀️.rs" declares 1 bare Set* identifier(s): SetEditorSettings
…
[verify semantic-vocabulary] high mutation-migration/semantic-vocabulary: "✏️s/🔌️plugins/✒️writer/…/🎮️commands/📸️set-snapshot/🦀️.rs" references banned generic mutation vocabulary: SetSnapshot
…
error: [verify semantic-vocabulary] 223 banned-vocabulary breach(es)
```

`.🧬semio/🦑️repo/🎫️tickets` does not appear in `roots=`. The 1 468 breaches are the pre-existing
SEMANTIC-MUTATIONS-OVERHAUL backlog in `✏️s`, untouched by this ticket — the command is new, the findings
are not.

The 20 files R-A predicted are real and would all have been scanned as production sources:

```
$ find ".🧬semio/🦑️repo/🎫️tickets" -name "*.rs" | grep -E "/🧬️mutations/|/🎮️commands/" | wc -l
      20
$ find ".🧬semio/🦑️repo/🎫️tickets" -name "*.rs" | wc -l
    2218
```

They are ticket scratch: ten `📸️/🧪️source-index-capture-66/🧫️run-*/owner/🧬️mutations/🦀️.rs` under
`26/08/12/SEMANTIC-MUTATIONS-OVERHAUL`, and ten
`26/09/02/COMPLETE-SEMIO-END-TO-END/🗑️generated/registry-plugin-root-ownership/plugin-root-ownership-*/…/🎮️commands/🦀️.rs`
— i.e. the pre-fix scan would have read files out of another ticket's `🗑️generated` folder.

### 3.2 Row 59 — re-verified, after this pass's edits

```
$ grep -c '🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test' 📜️script.ts
0
$ grep -c 'LEVELLESS_PHASES\|composeProjectNames' 📜️script.ts
0
```

The owning assertion, run in the harness package:

```
$ cd 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test
$ bun test "./🧪️tests/🧪️test-platform/🟦️.ts" -t "the root script names neither"
bun test v1.3.14 (0d9b296a)

 1 pass
 109 filtered out
 0 fail
 2 expect() calls
Ran 1 test across 1 file. [33.19s]
```

(The test is `🧩️ open/closed > the root script names neither the test module's location nor its phase
vocabulary`; it reads `📜️script.ts` from disk and asserts both. Run **after** the row-97 dynamic import
landed, which is the edit that could have reintroduced the literal.)

The whole `🧩️ open/closed` describe block:

```
$ bun test "./🧪️tests/🧪️test-platform/🟦️.ts" -t "open/closed"
bun test v1.3.14 (0d9b296a)

🧪️tests/🧪️test-platform/🟦️.ts:
(fail) 🧩️ open/closed > every oracle and every format-specific profile arrives as an owner contribution [56940.54ms]
  ^ this test timed out after 30000ms.

 5 pass
 104 filtered out
 1 fail
 377 expect() calls
Ran 6 tests across 1 file. [102.92s]
```

The one failure is `discoverTestContributions` exceeding its own declared `30_000` ms budget on this
machine (the assertion never ran; the discovery walk took 56.9 s under the current concurrent load). It is
not a root-script assertion and not in this partition — see §4.3.

### 3.3 Row 97 — the shared code table is loaded and measured

```
$ bun ./📜️script.ts schema check --report <ticket>/🗑️generated/schema-check-w2v.jsonl
…
[schema check] schema-catalog-stale=1
[schema check] shared-code-table=27 unshared-codes=16: document-dialect-unexpected, document-id-duplicate,
  document-id-missing, document-id-unaddressable, export-format-missing, export-id-duplicate,
  export-id-invalid, fixture-defines-schema, module-level-ineligible, module-scope-id-missing,
  mutation-leaf-id-grammar, placement-retired-location, ref-not-catalog-addressable, ref-unresolved,
  schema-catalog-stale, scope-id-duplicate
```

`shared-code-table=27` is the harness's `SCHEMA_DIAGNOSTIC_CODES` length, read live from
`SCHEMA_DIAGNOSTIC_CODE_TABLE` — the constant is present and exported, so the row-97 precondition is met.
Of the 16 unshared codes, 15 are emitted by `inventorySchemaScopes` in `📚️library/🔍️discovery/🟦️.ts`
(library partition, §4.1) and one — `schema-catalog-stale` — is emitted here and has **no counterpart in
the harness table** (§4.2). The two codes this command owns that *do* have counterparts,
`schema-catalog-missing` and `schema-catalog-malformed`, are gone from the unshared list: adoption of the
root half is complete.

### 3.4 `schema check` summary line

```
$ bun ./📜️script.ts schema check --report <ticket>/🗑️generated/schema-check-w2v.jsonl
[schema check] wrote 8802 findings to .🧬semio/…/🗑️generated/schema-check-w2v.jsonl
[schema check] modules=3206 scopes=3070 findings=8802
[schema check] document-dialect-unexpected=4
[schema check] document-id-duplicate=1
[schema check] document-id-missing=1
[schema check] document-id-unaddressable=2
[schema check] export-format-missing=6967
[schema check] export-id-duplicate=655
[schema check] export-id-invalid=323
[schema check] fixture-defines-schema=61
[schema check] module-level-ineligible=63
[schema check] module-scope-id-missing=41
[schema check] mutation-leaf-id-grammar=471
[schema check] placement-retired-location=1
[schema check] ref-not-catalog-addressable=121
[schema check] ref-unresolved=88
[schema check] schema-catalog-stale=1
[schema check] scope-id-duplicate=2
[schema check] shared-code-table=27 unshared-codes=16: …
```

Report: `🗑️generated/schema-check-w2v.jsonl` (3.7 MB, 8 802 lines). Compare `📓️wp2c-library-tooling.md`
§ its own run at 10 390 findings — the tree has moved under several wave-2 workers since; `export-format-missing`
is 6 967 here against 6 583 there, i.e. the largest code is still growing while lanes are split.

`schema-catalog-stale=1` is the library-partition catalog regeneration that `📓️wp2c-root-script.md` §4.3
deliberately deferred to the last action before ticket close; it is not a root-script finding.

### 3.5 The revived `verify package-purity`

```
$ bun ./📜️script.ts verify package-purity
error: [verify package-purity] 269 package language purity breach(es)
      at runPackagePurity (/Users/ueli/Documents/semio/📜️script.ts:7217:40)
```

The command dispatches now. What it did before, measured on the same tree:

```
$ bun ./📜️script.ts bogus-command
unknown command "bogus-command"
usage: bun ./📜️script.ts <os|semio|examples|setup|start|dev|generate|scale-fixture|new|schema|lint|
  verify|format|test|bench|stdio|build|cpp|publish|purge|clean|micro-commit|commit> [args…]
```

`policy` is absent from that registry, so `verify-package-purity` exited 1 with a router usage error and
never produced a purity result — a target that has been failing for a reason unrelated to what it measures.
(Both readings needed patience: the root script takes minutes to load its module graph on this loaded
machine, and two earlier attempts were killed at a 2–3 minute timeout with zero output before dispatch was
even reached.) The 269 breaches are pre-existing Shape-V2 packaging violations across the tree, outside this
partition and outside this ticket — reported, not fixed.

### 3.6 Dispatch sanity after making `SchemaScript.run` async

```
$ bun ./📜️script.ts schema bogus
error: unknown schema subcommand: "bogus" (expected audit | check | compile | docs | entries | generate | oracle | test | verify).

$ bun ./📜️script.ts schema verify
[schema verify] stale generated output: 🧰️framework/…/📚️library/🔣️schema-catalog.json, 🧰️framework/…/📚️library/📓️schema-catalog.md.
  Run bun ./📜️script.ts schema generate && bun ./📜️script.ts schema docs.
```

Both JSON files re-parsed after every edit: `📋️project.json` valid; `.vscode/launch.json` valid,
2 300 configurations, zero duplicate names.

## 4. Cross-partition requests

### 4.1 → library worker (row 97, the other half) — 15 codes in `📚️library/🔍️discovery/🟦️.ts`

`inventorySchemaScopes` / `schemaRustEntryDiagnostics` declare 32 diagnostic codes, none in the `schema-*`
vocabulary. The 15 that `schema check` actually emits today are listed verbatim by the new
`unshared-codes=` line (§3.3). The contract (§A, last bullet) makes the harness table canonical, so these
are renames in the library, not a mapping in the root script. The mapping the ledger row already names:

| library code | harness code |
|---|---|
| `export-format-missing` | `schema-export-incomplete` |
| `ref-unresolved`, `ref-not-catalog-addressable` | `schema-ref-unresolved` |
| `document-dialect-unexpected` | `schema-dialect-not-draft-07` |
| `document-id-missing`, `module-scope-id-missing` | `schema-module-id-missing` |
| `fixture-defines-schema` | `schema-fixture-defines-schema` |
| `placement-retired-location` | `schema-placement-forbidden-filename` / `schema-placement-outside-module` |
| `module-level-ineligible` | `schema-owner-ineligible` |
| `scope-id-duplicate` | `schema-scope-ambiguous` |
| `export-formats-annotation-invalid` | `schema-export-formats-annotation-invalid` |

Four have **no** harness counterpart and need a decision, not a rename: `document-id-duplicate`,
`document-id-unaddressable`, `export-id-duplicate`, `export-id-invalid`, `mutation-leaf-id-grammar`
(`📋️cross-partition-requests.md` row 97 proposes `schema-mutation-leaf-id`; that code does not exist in the
table today). The root script needs no further change when they land: it filters against the live table,
so the `unshared-codes` count falls to zero on its own and is the acceptance measure for row 97.

### 4.2 → harness worker — add `schema-catalog-stale` to `SCHEMA_DIAGNOSTIC_CODE_TABLE`

`schema check` distinguishes three catalog states and the shared table names only two. `schema-catalog-missing`
("does not exist") and `schema-catalog-malformed` ("exists but is not the declared document") do not cover
"exists, is well-formed, and does not match the modules on disk", which is the single most common finding a
developer sees. Exact addition, in table order beside its siblings:

```ts
  "schema-catalog-stale": "the derived schema catalog does not match the schema modules on disk; regenerate it",
```

The protocol schema's `SchemaDiagnosticCode` enum
(`🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/🔣️.json:2421`) is held equal to the table by the
invariants suite, so it needs the same member. Until then `schema check` emits the code honestly and reports
it as unshared rather than conflating it with `schema-catalog-malformed`.

### 4.3 → harness worker — `🧩️ open/closed` "every oracle … arrives as an owner contribution" exceeds its own timeout

`discoverTestContributions(repoRoot)` took 56 940 ms against a declared `30_000` ms budget (§3.2), so the
test fails before asserting anything. It is not flaky-adjacent noise: the walk is genuinely over budget on a
loaded machine. Either raise the budget or memoize the discovery walk. Not in this partition; flagged so the
red is not attributed to row 59, whose own assertion in the same block passes.

### 4.4 → library worker — `Taxonomy` interface (restating W2c §4.4, still open)

`readonly testDomainPath: string;` is still missing from the `Taxonomy` interface in
`📚️library/🔍️discovery/🟦️.ts` although `🔣️taxonomy.json` declares the key. `SchemaScript.testDomainPath()`
reads it through a narrowed cast; that cast is now load-bearing for **two** call sites (`schema test` and the
new `sharedDiagnosticCodes()`). Add the field and both casts collapse to a plain read.

## 5. Open questions

1. **`package.json` and the verify group.** No `verify` target of any kind has a `package.json` script today
   (§2). CLAUDE.md says `package.json` must call nx to run script commands; the existing tree simply does not
   apply that to `verify`. Either the whole verify family gets scripts in one pass, or none does. I did not
   introduce a group of one. Coordinator's call.
2. **Should `verify gate` run `semantic-vocabulary`?** It does not today, and it cannot until the 223 `high`
   breaches in `✏️s` are gone — that is the SEMANTIC-MUTATIONS-OVERHAUL backlog, not this ticket. The
   subcommand exists so the rule is runnable and measurable in isolation; wiring it into `verify-gate`'s
   `dependsOn` should follow the same graduation the rule's own docstring describes.
3. **`policyRepositoryOwnedRoots()`'s four hard-coded candidates.** `["✏️s", "🧰️framework", "🌎️hub",
   "♻️mit-bestand"]` are still unioned in ahead of the taxonomy areas. Three of them *are* declared areas;
   `✏️s` is not (`✏️s/🔌️plugins` and `✏️s/🔨️modules` are). With the literals removed the walk would lose
   `✏️s/🗿️artifacts` and any other `✏️s` child that is not one of those two areas. Row 113 did not ask for
   this and I did not change it, but the function still has two sources of truth for what a repository-owned
   root is; settling it means declaring the missing `✏️s` areas in the taxonomy (library partition) and then
   deleting the literals here.
4. **`verify-package-purity` has been failing on a router usage error, so its 269 breaches are unmeasured
   backlog** (§3.5). The target reported a failure, but never the one it exists to report. Whoever owns
   Shape-V2 packaging should read the real output before it is wired into anything blocking.
