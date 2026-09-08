# WP4d — `🦑️repo` product, `📓️print`, `♻️mit-bestand`: clearing the last findings

Worker partition: `🧰️framework/🛍️products/🦑️repo/🔨️modules/**` except `📚️library/**` and `🧪️test/**`,
plus `🧰️framework/🛍️products/🦑️repo/🧪️tests/**` (where the two assigned findings actually sat),
`🧰️framework/🛍️products/📓️print/**` and `♻️mit-bestand/**` (submodule `♻️mit-bestand/🔎️recherche`
untouched — confirmed a gitlink, see §3).

Predecessors: `📓️wp4-repo-product.md` (W9d), `📓️wp4b-repo-client.md` (W9b), `📓️wp4c-repo-product.md` (W9c).
Repo MCP was down for the whole session; no MCP tool was called, no ticket was opened/closed/reopened,
`🗑️generated/` was not deleted, no git-modifying command was used, no worktree, no cargo. Working logs went
to the session scratchpad; every output below is pasted verbatim from a real run.

---

## 1. Headline

| root | assigned | measured at session start | after |
|---|---|---|---|
| `🧰️framework/🛍️products/🦑️repo/**` (minus `📚️library`, `🔨️modules/🧪️test`) | 7 | **7** (5 `schema-export-incomplete` + 1 `schema-owner-ineligible` + 1 `schema-fixture-defines-schema`) | **0** |
| `🧰️framework/🛍️products/📓️print/**` | 0 | **0** | **0** |
| `♻️mit-bestand/**` | 8 | **8** — *all eight inside the `🔎️recherche` git submodule* | **8**, unchanged and unfixable here (§3) |

Two independent implementations agree on 0 for the partition: the harness
(`🧪️test/📜️script.ts test schema`) and the root `📜️script.ts schema audit` / `schema check` (§5).

**Measurement note that matters for anyone re-running this.** `test schema --under <root>` runs only the
placement / owner-eligibility / fixture-isolation passes; `schemaContractDiagnostics` gates the whole
catalog-driven half on `under.length === 0`
(`🧪️test/📦️packages/🟦️typescript/🟦️.ts:4861`). So the five `schema-export-incomplete` rows are invisible
to a `--under` run by construction and only a **whole-tree** run measures them. Both are reported below.

---

## 2. Per-finding decision table

| # | code | subject | decision | outcome |
|---|---|---|---|---|
| 1–5 | `schema-export-incomplete` | `repo.server.coordinator` exports `Timestamp`, `NullableTimestamp`, `NullableString`, `NullableInteger`, `EmailAddress` missing from `🧬️schema/🟦️.ts` | **Implement the format, do not annotate.** The scope really does provide TypeScript (34 of the 39 `$defs` exports were already there, each with its `parse<Export>()`); the five shared scalar `$defs` had a `parse*` function but no exported type, so `declaresSchemaExport("🟦️typescript", …)` (which requires `export interface\|type\|const\|class <Name>`) found nothing. An `x-semio-formats` annotation would have been a lie: the format is provided and the export is transported in it. | 5 exported type aliases added and **wired into the 23 interface fields that `$ref` them** in `🔣️.json`, so the TS mirrors the JSON reference graph instead of restating `string`/`string \| null` |
| 6 | `schema-owner-ineligible` | `🦑️repo/🧪️tests/🧪️transaction-process-ownership` owns a `🧬️schema/` module | **Move the module to an eligible level.** No `schemaScopeOwnerLevels` request is warranted: the level is not a missing product module, it is a `🧪️*` test collection, which the taxonomy correctly refuses (`fixtureOwnerReason`). The genuine owner did not exist because the *implementation* was in the test collection too. | new product-module scope `repo.native.observe` at `🦑️repo/🔨️modules/🔩️native/👁️observe/` |
| 7 | `schema-fixture-defines-schema` | `…/🧪️transaction-process-ownership/🧬️schema/🔣️.json` | **Move the contract to its owner, keep the data.** The document was two things at once: a real domain contract (observation, birth, native record layout, decision vocabulary, admission roles) and a fixture envelope. Both now live in the owner module; the vector `🔣️.json` stayed put as data and was not edited by a single byte (sha256 `7d094633bfc6d4043d109ffc4da68927a2e9a654576d8285947da37748c4be18`, unchanged before and after). | contract at `🔩️native/👁️observe/🧬️schema/🔣️.json`, old file deleted |

### 2.1 Why `🔩️native/👁️observe` and not a new module

`schemaScopeOwnerLevels.levels.product-module` is `🧰️framework/🛍️products/*/🔨️modules/**`, so **any** depth
under `🔨️modules` is eligible. A brand-new top-level module (`🦑️repo/🔨️modules/<new>`) would have needed an
entry in `semanticDirectoryMemberKinds.members-of-modules.memberNames`
(`📚️library/🔣️taxonomy.json`) — another partition, and a red workspace-taxonomy gate until it landed.
`🔩️native` is already a registered module member and `👁️observe` is already a registered
`members-of-members-of-modules` name, so the placement needed **no taxonomy edit at all**, and it is the
honest domain: the subject dlopens libc, decodes `proc_bsdinfo` at fixed darwin offsets, parses
`/proc/<pid>/stat`, and decodes a Windows `FILETIME`. Verified rather than assumed — the root audit derives
the scope on its own:

```
{"modulePath": "🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/👁️observe/🧬️schema",
 "ownerPath": "🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/👁️observe",
 "level": "product-module", "facetKindId": "🧬️data", "scopeId": "repo.native.observe"}
```

### 2.2 `repo.native.observe` exports

`$id https://semio.tech/schema/repo/native/observe/schema.json`, draft-07, root `$ref` →
`#/$defs/TransactionProcessOwnershipVector`. Formats provided: `🔣️jsonschema` only, so no
`x-semio-formats` annotation is required and none was invented.

| Export | Mirrors | Why it is an export and not a `definitions` helper |
|---|---|---|
| `TransactionProcessBirth` | TS `TransactionProcessBirth` | the per-platform birth token that distinguishes a live pid from a reused one |
| `TransactionProcessObservation` | TS `TransactionProcessObservation` | addressed directly by the decision oracle (`#/$defs/TransactionProcessObservation`) |
| `TransactionProcessLayout` | TS `TransactionProcessLayout` | one `oneOf` of the darwin (136 B) and win32 (568 B) records, replacing the two anonymous `darwinLayout`/`windowsLayout` helpers |
| `TransactionProcessDecisionKind` | TS `TransactionProcessDecision["kind"]` | the five verdicts |
| `TransactionProcessDecisionCase` | — | one decision the observer owes when a base observation changes in exactly one way |
| `TransactionProcessRole` | TS `TransactionProcessRole` | the admission vocabulary; the implementation rejects an unlisted role |
| `TransactionProcessFiletime` | — | the Windows FILETIME halves and the decimal they decode to |
| `TransactionProcessProbeBudget` | — | `signals: false` / `retry: false` are the contract, not a setting |
| `TransactionProcessOwnershipVector` | — | the language-agnostic conformance vector every implementation is held against |

`decimal` and `pid` stayed module-internal in `definitions` (contract §A). Every negative case the old
wrapper enforced still fails: root `additionalProperties: false`, `nativeProbe.signals` `const false`,
`darwinLayout.size` `const 136`, `scope` `const` — the four negative cases the test feeds to ajv, §5.5.

---

## 3. `♻️mit-bestand`: the eight rows are inside a git submodule

Every one of the eight is under `♻️mit-bestand/🔎️recherche/_neo4j/…`. That path is not a directory of this
repository:

```
$ git ls-files -s ♻️mit-bestand | awk '$1=="160000"'
160000 92036c7ca0149b43ddea28db8c8e516f983fe718 0	♻️mit-bestand/🔎️recherche

$ cat .gitmodules
[submodule "♻️mit-bestand/recherche"]
	path = ♻️mit-bestand/🔎️recherche
	url = https://github.com/usalu/recherche.git
	ignore = dirty
```

This repository tracks exactly one object for that path — the gitlink. `kg_jsonl_record_schema.json`,
`manifest_schema.json`, `lane_schema.json` and `patch_record.schema.json` are files of
`usalu/recherche`, authored and versioned there. They cannot be folded into a `🧬️schema/` module here,
their readers are not in this tree, and moving them would corrupt the submodule's working tree. The brief's
description of these rows as flat mit-bestand files does not match the tree: the *actual* mit-bestand flat
files were already folded into `🧬️schema/` modules by W9d (`📓️wp4-repo-product.md` §2), and
`♻️mit-bestand` outside the submodule measures **0** today.

The defect is therefore in the walker, not in the data — see cross-partition request **W4d-1**.

---

## 4. Files changed

### Created

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/👁️observe/🧬️schema/🔣️.json` — the `repo.native.observe`
  contract (9 exports, 2 internal helpers).

### Moved

| From | To |
|---|---|
| `🦑️repo/🧪️tests/🧪️transaction-process-ownership/🟦️.ts` (the observer implementation, 219 lines, 4 exported entry points) | `🦑️repo/🔨️modules/🔩️native/👁️observe/🟦️.ts` — byte-identical, sha256 `6ad98b886bd44dbafd4b60ff4ef63b1263b0f7fed12edcdd5ef1458062935b20` before and after |

Production code with an exported API had been living inside a `🧪️tests` collection; that is what made the
schema homeless in the first place, so the module and its contract moved together.

### Deleted

- `🦑️repo/🧪️tests/🧪️transaction-process-ownership/🧬️schema/🔣️.json` — superseded outright by the owner
  module. No alias, no re-export, no redirect left behind.
- `🦑️repo/🧪️tests/🧪️transaction-process-ownership/🧪️test/` — emptied by a concurrent peer sweep mid-session
  (§6); removed the leftover directory.

### Edited

| File | Change |
|---|---|
| `🦑️repo/🔨️modules/🖥️server/🎛️coordinator/🧬️schema/🟦️.ts` | `+5` exported type aliases (`Timestamp`, `NullableTimestamp`, `NullableString`, `NullableInteger`, `EmailAddress`), each with a docstring; 23 interface fields across `AuthWhoAmIResponse`, `Developer`, `DeveloperApiKey`, `CreateDeveloperRequest`, `CreateKeyRequest`, `Ticket`, `Scope`, `Warning`, `Breach`, `Event` retyped from the inline primitive to the alias the JSON `$ref`s. Structurally identical, so no consumer changed |
| `🦑️repo/🧪️tests/…transaction-process-ownership/🟦️.ts` (the test) | reads implementation and schema from `🔩️native/👁️observe`; `#/definitions/observation` → `#/$defs/TransactionProcessObservation` (and the `properties/required/additionalProperties: undefined` stripping the old root shape needed is gone); the inertness list follows; **+3 new assertions** pinning `$schema`, `$id` and the root `$ref` of the owner module, so the binding is declared rather than positional |

Nothing under `📓️print/**` or `♻️mit-bestand/**` needed a change; both measured 0 at session start and 0
at the end.

---

## 5. Verification (real output)

### 5.1 Harness, per root — the `--under` gate

```
$ bun 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts test schema --under <root>

=== 🧰️framework/🛍️products/🦑️repo/🔨️modules ===
[test schema] 148 invariant finding(s) over 🧰️framework/🛍️products/🦑️repo/🔨️modules
[test schema]      74 × schema-placement-outside-module
[test schema]      74 × schema-fixture-defines-schema
[test schema] 0/0 schema-bound fixture(s) reached their declared stage
=== 🧰️framework/🛍️products/🦑️repo/🧪️tests ===
[test schema] 0 invariant finding(s) over 🧰️framework/🛍️products/🦑️repo/🧪️tests
[test schema] 0/0 schema-bound fixture(s) reached their declared stage
=== ♻️mit-bestand ===
[test schema] 8 invariant finding(s) over ♻️mit-bestand
[test schema]       7 × schema-placement-outside-module
[test schema]       1 × schema-placement-forbidden-filename
[test schema] 0/0 schema-bound fixture(s) reached their declared stage
=== 🧰️framework/🛍️products/📓️print ===
[test schema] 0 invariant finding(s) over 🧰️framework/🛍️products/📓️print
[test schema] 0/0 schema-bound fixture(s) reached their declared stage
```

All 148 under `🔨️modules` are in `📚️library/**` (tooling worker), zero in this partition — filtered
programmatically, not by eye:

```
$ … test schema --under 🧰️framework/🛍️products/🦑️repo/🔨️modules --json | (filter out 📚️library, 🧪️test)
total 148 mine 0
```

The 148 are a peer's live churn, not a regression: at 20:34 the same root read 104 findings dominated by
`schema-placement-forbidden-filename` on `📚️library/…/🧫️fixtures/*/🧬️schema.json`; a sweep converted those
to `🧬️schema/🔣️.json` inside the same fixture directories at 20:52, which trades one code for two.

All eight `♻️mit-bestand` rows name a `🔎️recherche/…` path (§3).

### 5.2 Harness, whole tree — the only run that measures export completeness

```
$ bun 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts test schema --json
(4m39s)
total 5062 Counter({'schema-export-incomplete': 2856, 'schema-export-parser-missing': 1696,
 'schema-ref-unresolved': 169, 'schema-fixture-defines-schema': 156, 'schema-owner-ineligible': 68,
 'schema-placement-outside-module': 51, 'schema-dialect-not-draft-07': 40,
 'schema-placement-forbidden-filename': 18, 'schema-ref-broken-internal': 5, 'schema-file-missing': 3})
mine 0
```

`mine` here is the union of *every path* in the partition **and** *every finding whose `scope` starts with*
`repo.` / `print.` / `mit-bestand.` — so it also covers the scope-keyed rows that carry no path
(`schema-export-incomplete` with only `{scope, export, format}`). Zero. In particular
`repo.server.coordinator`'s 39 exports now satisfy both the type rule and the newer
`schema-export-parser-missing` rule.

### 5.3 Root `schema audit` — independent implementation, same verdict

```
$ bun ./📜️script.ts schema audit --out <scratchpad>
[schema audit] 3206 modules, 10315 findings -> …
```
```
findings in this partition: 0
Counter()  # over codes export-format-missing, mutation-leaf-id-grammar, document-dialect-unexpected,
           # export-id-duplicate, export-id-invalid, module-scope-id-inconsistent, ref-not-catalog-addressable,
           # fixture-defines-schema, scope-id-duplicate, module-level-ineligible, …
```

### 5.4 Root `schema check`

```
$ bun ./📜️script.ts schema check --report <scratchpad>/schema-check.jsonl
[schema check] wrote 8320 findings to …
[schema check] modules=3206 scopes=3070 findings=8320
[schema check] document-dialect-unexpected=4
[schema check] document-id-duplicate=1
[schema check] document-id-missing=1
[schema check] document-id-unaddressable=2
[schema check] export-format-missing=6649
[schema check] export-id-duplicate=655
[schema check] export-id-invalid=323
[schema check] fixture-defines-schema=61
[schema check] module-level-ineligible=63
[schema check] module-scope-id-missing=41
[schema check] mutation-leaf-id-grammar=362
[schema check] placement-retired-location=1
[schema check] ref-not-catalog-addressable=55
[schema check] ref-unresolved=99
[schema check] schema-catalog-stale=1
[schema check] scope-id-duplicate=2
```
```
findings in this partition: 0
```

The single `schema-catalog-stale` row names
`📚️library/🔣️schema-catalog.json` — the derived catalog, another partition's file; request **W4d-2**.

### 5.5 The moved test

Baseline **before** any of my edits, at the default 5 s budget:

```
$ bun test ./🧰️framework/🛍️products/🦑️repo/🧪️tests/🧪️transaction-process-ownership/🧪️test/🟦️.ts
(fail) complete private helper passes actual strict TypeScript declaration checking [9353.61ms]
  ^ this test timed out after 5000ms.
 7 pass
 1 fail
 339 expect() calls
```

After the move, at the same default budget — **identical shape**, the same one test over budget:

```
 7 pass
 1 fail
 339 expect() calls
```

And with the budget raised past that one slow `tsc` case, at the final on-disk location:

```
$ bun test --timeout 60000 ./🧰️framework/🛍️products/🦑️repo/🧪️tests/⚙️transaction-process-ownership/🟦️.ts
[DEBUG] Private process observation inert input 6ad98b886bd44dbafd4b60ff4ef63b1263b0f7fed12edcdd5ef1458062935b20; no native calls or subjects
[DEBUG] Process observer inert endpoints {"pid":14061,"inputs":[
  {"path":"…/🔨️modules/🔩️native/👁️observe/🟦️.ts","bytes":20011,"sha256":"6ad98b886bd44dbafd4b60ff4ef63b1263b0f7fed12edcdd5ef1458062935b20"},
  {"path":"…/🧪️tests/🧪️transaction-process-ownership/🔣️.json","bytes":4212,"sha256":"7d094633bfc6d4043d109ffc4da68927a2e9a654576d8285947da37748c4be18"},
  {"path":"…/🔨️modules/🔩️native/👁️observe/🧬️schema/🔣️.json","bytes":8298,"sha256":"b5a6c79f8703aac7af2a9e41f53fde4ca4e74f72382b7ac95503d3d5547bb34d"},
  {"path":"…/🧪️tests/⚙️transaction-process-ownership/🟦️.ts","bytes":19062,"sha256":"58e3a62b6089cda3db9444f548ae28c69fb4b2f52da9b4abed7a4b224b26eb56"}]}

 8 pass
 0 fail
 342 expect() calls
```

339 → 342 is exactly the three `$schema`/`$id`/`$ref` assertions added; no assertion was lost in the move.
The ajv 8.20.0 draft-07 oracle compiles the new document (`$ref: "#/$defs/…"` plus `#/definitions/…`
helpers) under `strict: true` — probed directly before writing it, and exercised twice per run by the test.

### 5.6 Coordinator suites

```
$ cd …/🖥️server/🎛️coordinator/📦️packages/🟦️typescript
$ node <repo>/node_modules/vitest/vitest.mjs run --config vitest.config.ts
 RUN  v4.1.10 …/🖥️server/🎛️coordinator/📦️packages/🟦️typescript
 Test Files  2 passed (2)
      Tests  118 passed (118)
   Start at  20:44:37
   Duration  5.07s
```

(the config is `vitest.config.ts` now, not `🧪️tests/🟦️.ts` as W9d's report says; a peer renamed it.)

```
$ npx tsc --noEmit --strict --target es2022 --module esnext --moduleResolution bundler \
    🦑️repo/🔨️modules/🖥️server/🎛️coordinator/🧬️schema/🟦️.ts
node_modules/@types/mdx/types.d.ts(23,38): error TS2503: Cannot find namespace 'JSX'.
node_modules/@types/mdx/types.d.ts(28,23): error TS2503: Cannot find namespace 'JSX'.
node_modules/@types/mdx/types.d.ts(47,28): error TS2503: Cannot find namespace 'JSX'.
node_modules/@types/mdx/types.d.ts(73,46): error TS2503: Cannot find namespace 'JSX'.
```

Four pre-existing `@types/mdx` errors, the same four W9d recorded; **zero** from the file itself.

### 5.7 Taxonomy statutes over the new directory

```
$ bun test --timeout 300000 ./…/📚️library/🧪️tests/🔏️path-emoji-statutes/🟦️.ts
 36 pass
 2 fail
 1344 expect() calls
```

Neither failure touches this partition: `graph manifest discovery …` dies on
`ENOENT … 🧰️framework/🔨️modules/🕸️graph/📦️packages/🦀️rust/build.rs` (framework partition, a file another
worker is mid-move on) and `the normalization reader accepts the same current reserved-name contracts`
times out at 30 s after 59 s. `👁️observe` under `🔩️native` produced no statute finding.

---

## 6. A concurrent sweep landed on this case mid-session

At 20:52, while the verification runs were going, a repo-wide sweep renamed
`🦑️repo/🧪️tests/🧪️transaction-process-ownership/🧪️test/🟦️.ts` to
`🦑️repo/🧪️tests/⚙️transaction-process-ownership/🟦️.ts` (a `🧪️*` directory nested inside a `🧪️tests` case is
not a taxonomy shape), **kept my edits**, rewrote the vector path to
`../🧪️transaction-process-ownership/🔣️.json`, and updated the caller
`📚️library/📦️packages/🟦️typescript/📜️script.ts:327` to match. Attribution is from disk state and mtimes,
not inference: the swept file contains my `resolve(owner, "../../🔨️modules/🔩️native/👁️observe")` line and
my three new assertions, and it is dated 20:52 against my 20:47 edit.

I did not fight it. §5.5's final run is against the swept location and is green. What it leaves behind is a
case split across two sibling directories — data in `🧪️transaction-process-ownership/`, test in
`⚙️transaction-process-ownership/` — and neither name is registered in
`semanticDirectoryMemberKinds.members-of-tests.memberNames` (nor was the old one, before this ticket). That
is open question 3.

---

## 7. Cross-partition requests

**W4d-1 — `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts`, `schemaTreeFiles`
(line 4442) → the harness worker (W1c/W1d). This is the whole of the remaining `♻️mit-bestand` count.**
The walk skips dot-directories, `SKIP_DIR_NAMES` and excluded test paths, but not git submodules, so it
audits `usalu/recherche` as if this repository authored it. Read the declared submodule paths from
`.gitmodules` once and skip them:

```ts
     for (const entry of entries) {
       if (entry.isSymbolicLink()) continue;
       const rel = relDir.length === 0 ? entry.name : `${relDir}/${entry.name}`;
       if (entry.isDirectory()) {
-        if (entry.name.startsWith(".") || SKIP_DIR_NAMES.has(entry.name) || isExcludedTestPath(repoRoot, rel)) continue;
+        // 🧩️A submodule is a FOREIGN repository: its files are authored, versioned and reviewed
+        // elsewhere, and this workspace tracks one gitlink for the whole tree. Auditing them reports
+        // findings nobody here can act on.
+        if (entry.name.startsWith(".") || SKIP_DIR_NAMES.has(entry.name) || submodulePaths(repoRoot).has(rel) || isExcludedTestPath(repoRoot, rel)) continue;
         walk(join(absDir, entry.name), rel);
         continue;
       }
```

with `submodulePaths(repoRoot)` a memoised parse of `.gitmodules` (`path = <rel>` lines, NFC-normalised) —
no `git` subprocess, so it works in a devcontainer and on a bare checkout. Today that set is exactly
`{"♻️mit-bestand/🔎️recherche"}`. Effect, measured: the 8 remaining `♻️mit-bestand` rows (7
`schema-placement-outside-module`, 1 `schema-placement-forbidden-filename`) and the repo-wide totals drop by
the same 8. **The root `📜️script.ts schema audit`/`check` walker needs the identical change** — it reports
the same four documents.

**W4d-2 — `📚️library/🔣️schema-catalog.json` → the tooling worker (W2c).** One new scope to pick up on the
next `bun ./📜️script.ts schema generate`. This is not hand-written: it is what the generator already
produces in memory today, copied out of the audit's own catalog.

```json
"repo.native.observe": {
  "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/👁️observe/🧬️schema",
  "level": "product-module",
  "facetKind": "🧬️data",
  "formats": { "🔣️jsonschema": "🔣️.json" },
  "exports": {
    "TransactionProcessBirth":           { "file": "🔣️.json", "facet": "schema" },
    "TransactionProcessDecisionCase":    { "file": "🔣️.json", "facet": "schema" },
    "TransactionProcessDecisionKind":    { "file": "🔣️.json", "facet": "schema" },
    "TransactionProcessFiletime":        { "file": "🔣️.json", "facet": "schema" },
    "TransactionProcessLayout":          { "file": "🔣️.json", "facet": "schema" },
    "TransactionProcessObservation":     { "file": "🔣️.json", "facet": "schema" },
    "TransactionProcessOwnershipVector": { "file": "🔣️.json", "facet": "schema" },
    "TransactionProcessProbeBudget":     { "file": "🔣️.json", "facet": "schema" },
    "TransactionProcessRole":            { "file": "🔣️.json", "facet": "schema" }
  },
  "dependsOn": [],
  "hashes": { "🔣️.json": "b5a6c79f8703aac7af2a9e41f53fde4ca4e74f72382b7ac95503d3d5547bb34d" }
}
```

Until it lands, the catalog-driven half of the gate does not see this scope, so its 0 is vacuous rather
than earned — the export completeness of a JSON-Schema-only scope is trivially satisfied, but say so
honestly.

**W4d-3 — `📚️library/🔣️taxonomy.json`,
`semanticDirectoryMemberKinds.members-of-tests.memberNames` → the tooling worker (W2c).**
`🦑️repo/🧪️tests` holds one case and **neither** its old name (`🧪️transaction-process-ownership`) nor the
name a sweep gave it at 20:52 (`⚙️transaction-process-ownership`) appears in that list; the sweep renamed the
directory without registering it. Add the name the sweep settled on. Pre-existing, not caused by this work,
but it is now the only unregistered directory left in this partition.

**W4d-4 — `📚️library/📦️packages/🟦️typescript/🧫️fixtures/🧼️remaining-package-purity-authority/🔣️.json`
→ the tooling worker (W2c).** If that authority is exhaustive over authored files, it needs a row for the
new implementation location `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/👁️observe/🟦️.ts`
(`implementation`/`authored`, owner `…/🔨️modules/🔩️native/👁️observe`) and the old
`…/🧪️tests/🧪️transaction-process-ownership/🟦️.ts` row removed, if one exists. I did not find one, so this
is conditional on how exhaustive that fixture is meant to be.

---

## 8. Open questions / deliberately not acted on

1. **No fixture in the repository declares `schemaFixtures` yet.** `git grep -l schemaFixtures` returns only the harness source
   (`🧪️test/📦️packages/🟦️typescript/🟦️.ts`) and two of this ticket's own reports, and every `test schema` run in §5.1 reports
   `0/0 schema-bound fixture(s)`. The natural first binding is exactly this vector
   (`schema://repo.native.observe/TransactionProcessOwnershipVector`), but the case's contribution file
   *is* the vector — the harness reads `schemaFixtures` out of the case's `🔣️.json`
   (`testContributionFileKindId: "json"`), and that document is closed
   (`additionalProperties: false`), so adding the key would make the vector fail its own contract. Binding
   it properly needs the case split into a descriptor plus a data file, which is a harness-shaped decision,
   not a partition-shaped one. The test pins the owner's `$id` and root `$ref` instead (§4), so the binding
   is at least declared. Recommend the harness worker define the descriptor/data split before anyone writes
   the first `schemaFixtures` block.
2. **`repo.native.observe` has no `🟦️.ts` facet.** The implementation at the module root exports
   `TransactionProcessObservation`, `TransactionProcessBirth`, `TransactionProcessLayout`,
   `TransactionProcessDecision` and `TransactionProcessRole` under those exact names, so a
   `🧬️schema/🟦️.ts` facet is a short step — but it would then have to carry `parse<Export>()` for **all
   nine** exports, including the vector, and nothing consumes a parser for the vector except the test, which
   uses ajv as its third-party oracle. Adding the facet to satisfy a rule rather than a consumer would be
   the dead-export shape contract §B forbids. Left json-only, deliberately.
3. **The case is split across two directories after the 20:52 sweep** (§6). Consolidating (vector into
   `⚙️transaction-process-ownership/`, delete `🧪️transaction-process-ownership/`) is one `mv` plus one path
   in the test, but the sweep is live and re-runs; racing it would produce a third copy. Whoever owns the
   sweep should finish the rename or the consolidation, together with W4d-3.
4. **`repo.server.coordinator` field types are now aliases, and that exposed nothing new.** Wiring the 23
   `$ref`-carrying fields to the five aliases was a pure structural no-op (`Timestamp = string`), so it
   proves the JSON and the TS agree on *which* fields are timestamps, e-mails and nullables — it does not
   validate the RFC-3339 pattern at the type level, which TypeScript cannot express. The runtime check
   still lives in `parseTimestamp`, and the 118 vitest cases hold it against the ajv oracle.
5. **W9d's open items 1–5 remain open** (`📓️wp4-repo-product.md` §8): the two competing dead SQLite
   schemas in `💻️client/🪶️sqlite`, the broken `@/lib` path in the coordinator `tsconfig.json`, the missing
   nx project for the server library, the two coordinator server implementations with different route
   shapes, and `🧫️invocations.json` without a schema or consumer. None of them is a schema-ownership
   finding today — `test schema`, `schema audit` and `schema check` all report 0 for this partition — so
   none was reopened here.
