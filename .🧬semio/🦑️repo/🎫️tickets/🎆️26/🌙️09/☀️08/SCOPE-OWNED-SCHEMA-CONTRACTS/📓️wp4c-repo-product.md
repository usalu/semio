# WP4c — `🦑️repo` product modules (rows 68 client side, 72, and the `🔌️mcp` GraphQL placement)

Worker partition: `🧰️framework/🛍️products/🦑️repo/🔨️modules/**` except `📚️library/**` and `🧪️test/**`,
plus `🧰️framework/🛍️products/🦑️repo/README.md`.

Assigned: `📋️cross-partition-requests.md` **row 68** (client half), **row 72**, and the
`🔌️mcp/🔗️graphql/🔗️.graphql` → `🔌️mcp/🧬️schema/` move flagged in `📓️wp4b-repo-client.md` §7.3.

Repo MCP was down for the whole session; no MCP tool was called, no ticket was opened/closed/reopened,
`🗑️generated/` was not deleted, no git-modifying command was used, no worktree was created, no `cargo`
was run. Predecessor W9c left **nothing** on disk: `git status --porcelain` over the partition at session
start showed one file (`🪶️sqlite/📦️packages/🟦️typescript/🧪️tests/🟦️.ts`, W9b's), no
`🧩️vscode/🧬️schema`, no `🗂️technologies.json`, no `🔌️mcp/🧬️schema`.

---

## 1. New scopes and their exports

| Scope id | Module path | `$id` | Formats |
|---|---|---|---|
| `repo.client.vscode` | `🦑️repo/🔨️modules/💻️client/🧩️vscode/🧬️schema/` | `https://semio.tech/schema/repo/client/vscode/schema.json` | `🔣️.json` (draft-07), `🟦️.ts` |
| `repo.client.mcp` | `🦑️repo/🔨️modules/💻️client/🔌️mcp/🧬️schema/` | `https://semio.tech/schema/repo/client/mcp/schema.json` | `🔗️.graphql` (normative SDL), `🔣️.json` (draft-07) |

**Deviation from the brief, deliberate.** The brief named
`$id https://semio.tech/schema/repo/mcp/schema.json` for the MCP scope. Contract §A requires the scope id
to be derivable from the path, and the module owner is `🔨️modules/💻️client/🔌️mcp`, so the derived id is
`repo.client.mcp` — which is what `bun ./📜️script.ts schema audit` independently reports (§6.2), and what
the two sibling client scopes already use (`repo.client.sqlite`, and now `repo.client.vscode`). Using
`repo/mcp` would have produced a `module-scope-id-inconsistent` finding. Same reasoning applied to the
vscode scope, whose `$id` the brief left to the path (`repo/client/vscode`).

### 1.1 `repo.client.vscode` exports

| Export | Shape |
|---|---|
| `TechnologyCatalog` | array, `minItems: 1`, `uniqueItems: true`, items `TechnologyCatalogEntry`; declaration order is contract |
| `TechnologyCatalogEntry` | closed object, all five fields required |

Bounds and patterns are the ones the TS consumer actually relies on, not placeholders:
`id` kebab-case `^[a-z][a-z0-9]*(?:-[a-z0-9]+)*$` 3–32 (the extension switches on the family prefix,
`isDefinitionEntityId` tests `definition-`); `emoji` `^\S+$` 1–8 (**not** unique — two kinds share one
emoji, see below); `iconId` kebab-case 3–32 (not unique: `pen-tool` and `check-circle-2` each serve two
kinds); `label` trimmed 2–32; `filterable` boolean (false exactly for the six aggregate tree roots).

`🧬️schema/🟦️.ts` exports the two types, `parseTechnologyCatalog`, `parseTechnologyCatalogEntry`,
`VSCODE_SCHEMA_ID` and `VSCODE_PARSERS` (export id → parser, asserted key-for-key equal to `$defs`).
The parser implements the document *exactly*, no stronger: string lengths are counted in code points
(`[...value].length`) so `minLength`/`maxLength` agree with ajv's `ucs2length` on multi-unit emoji, and
array uniqueness is JSON-value identity, matching `uniqueItems`. Catalog-level invariants that draft-07
cannot express (unique `id`, unique `label`) are asserted against the data in the test, never smuggled
into the parser.

### 1.2 `repo.client.mcp` exports

62 exports: the `DateTime` scalar plus the 61 `type`/`input`/`interface`/`enum` definitions of the SDL
(`Node`, `Query`, `Mutation`, 9 `input`s, 8 `enum`s, 42 object types). Mapping rules, all enforced by the
parity test:

- `!` ⇒ the property shape excludes `null`; a nullable field ⇒ `anyOf: [shape, {type: null}]`.
- `input` types carry `required` = exactly their non-null fields (a request object is complete).
- `type` objects carry **no** `required`: a GraphQL response contains only the selected fields, so every
  property is optional and the `!` information lives in the shape, not in `required`.
- `interface Node` is the only `additionalProperties: true` export (an implementing type adds fields);
  every `type`/`input` is closed.
- Field arguments are not representable in JSON Schema and stay in the SDL; the affected exports carry a
  `$comment` saying so.
- Root `$ref` is `#/$defs/Query` — one resolved `Query` selection, the `data` payload the CLI executor
  answers with.

---

## 2. Files created / moved / deleted

### Created

- `💻️client/🧩️vscode/🗂️technologies.json` — the catalog data (58 entries), the only instance document of
  the new scope.
- `💻️client/🧩️vscode/🧬️schema/🔣️.json`, `💻️client/🧩️vscode/🧬️schema/🟦️.ts`
- `💻️client/🧩️vscode/🧪️tests/🔬️schema/🟦️.ts` — parser + ajv oracle case (22 tests).
- `💻️client/🧩️vscode/📦️packages/🟦️typescript/🧪️tests/🟦️.ts` — vitest config for that case.
- `💻️client/🔌️mcp/🧬️schema/🔣️.json`
- `💻️client/🔌️mcp/🧪️tests/🔬️schema/🟦️.ts` — SDL ↔ JSON parity + ajv oracle (67 tests).
- `💻️client/🔌️mcp/📦️packages/🟦️typescript/{📋️project.json,📜️script.ts,🧪️tests/🟦️.ts}` — new nx project
  `@semio-tech/repo-mcp-schema` (the module had no TypeScript test host; the module-root `repo-mcp`
  project only routes Go).
- `💻️client/🔌️mcp/README.md` — the module had none; it documents the new scope and replaces the deleted
  `🔗️graphql/README.md`.

Authoring aid kept in this ticket folder: `wp4c-graphql-to-jsonschema.py`, the one-off SDL → JSON Schema
derivation. Nothing depends on it at runtime; the parity is re-derived at test time, not by re-running it.

### Moved

| From | To |
|---|---|
| `💻️client/🔌️mcp/🔗️graphql/🔗️.graphql` | `💻️client/🔌️mcp/🧬️schema/🔗️.graphql` |
| catalog array formerly at `🧰️framework/🔨️modules/🧬️schema/🔣️.json` | `💻️client/🧩️vscode/🗂️technologies.json` |

The moved SDL lost its two trailing executable documents (`query Nodes`, `query NodesAndEdges`): they are
instance documents, not schema, and nothing reads them — the exhaustive Go case
`TestExhaustiveNodesAndEdges` embeds its own query string. Its self-referential header path was corrected.

### Deleted

- `💻️client/🔌️mcp/🔗️graphql/` — the whole `repo/graphql` bundle (`🔗️.graphql` moved, `📋️project.json`,
  `README.md`, `AGENTS.md` removed). The bundle had no target, no code, and existed only to hold the one
  SDL file. **This deletes an `AGENTS.md`.** CLAUDE.md forbids *editing* AGENTS.md; leaving a bundle
  manifest for a directory whose only content moved away would be exactly the kind of stale artefact the
  contract's "no leftovers" rule forbids, so it went with the bundle. Flagging it explicitly rather than
  silently.
- `💻️client/🔌️mcp/mcp` — a 12 MB Mach-O binary my own `go build .` verification produced. It is not
  covered by `.gitignore` and the auto-commit had already staged it; removed from the worktree so the
  next `git add -A` drops it.

### Edited

| File | Change |
|---|---|
| `💻️client/🧩️vscode/📦️packages/🟦️typescript/🟦️.ts` | the 56-pair `ENTITY_EMOJIS` literal is gone; `TECHNOLOGY_CATALOG` is `🗂️technologies.json` parsed through `🧬️schema/🟦️.ts` at load (throws on violation), and `ENTITY_EMOJIS` is derived from it |
| `💻️client/🧩️vscode/📦️packages/🟦️typescript/📜️script.ts` | `test` runs the vitest schema case at every level and adds the extension-host `vscode-test` run at `long`+; the extension-host build entry `🧪️extension.test.ts` (a path the taxonomy sweep removed) → `../../🧪️tests/🧩️extension/🟦️.ts` |
| `💻️client/🧩️vscode/📦️packages/🟦️typescript/tsconfig.json` | `include` → the four real files (same stale `🧪️extension.test.ts` entry) |
| `💻️client/🧩️vscode/README.md` | documents `🗂️technologies.json`, `🧬️schema`, both test cases |
| `💻️client/🪶️sqlite/README.md` | test path `📦️packages/🟦️typescript/🔬️schema.test.ts` → `🧪️tests/🔬️schema/🟦️.ts` (the sweep moved it after W9b wrote the line) |
| `💻️client/⌨️cli/🧪️tests/🔬️component/🐹️.go` | `TestBundleListIDs` expectation map: dropped the `repo/graphql` row with the bundle |
| `🖥️server/🎛️coordinator/🧪️tests/🔬️server-persistence/🟦️.ts` | row 72 D (below) |
| `🖥️server/🎛️coordinator/📦️packages/🟦️typescript/🧪️tests/🟦️.ts` | added `include: ["../../🧪️tests/*/🟦️.ts"]`, `passWithNoTests: false` |
| `🖥️server/🧬️schema/🔣️.json` | six columns added (below) |
| `🦑️repo/README.md` | row 72 C (below) |

---

## 3. Row 72

### 3.1 C — README link

`🦑️repo/README.md`'s `# 📦️ Bundles` list was seven pre-taxonomy paths (`cli/README.md`, `go/README.md`,
`graphql/README.md`, `postgres/README.md`, `server/README.md`, `sqlite/README.md`, `vscode/README.md`),
**none** of which resolves. Fixing only the sqlite one would have left six dead links, so the section is
now `# 🔨️ Modules` with the eight real module paths.

### 3.2 D — constraint-keyword filter, and the bug it was hiding

`🔬️server-persistence/🟦️.ts` matched table-constraint clauses with
`TABLE_CONSTRAINTS.some(c => clause.toUpperCase().startsWith(c))`. Replaced with the word-boundary regex
from `🪶️sqlite/🧪️tests/🔬️schema/🟦️.ts`:

```ts
const TABLE_CONSTRAINT = /^(?:PRIMARY KEY|UNIQUE|FOREIGN KEY|CHECK|CONSTRAINT|EXCLUDE)\b/i;
```

and the whole-line comment filter (`!line.trim().startsWith("--")`) with the end-of-line strip
(`line.replace(/--.*$/, "")`), which is what the SQLite version does.

**W9b's row-72 D note said "the postgres DDL has no such column today, so the test is green by luck".
That is not so — it had six, and both sides of the parity test were built from the same broken
extraction, so the schema was silently missing them.** With the fix the suite went red on six tables; the
missing exports were added to `🖥️server/🧬️schema/🔣️.json` (properties in DDL order, `required` = all
columns, nullability projected from the DDL):

| Export | Column | Shape |
|---|---|---|
| `KitCheckpointsRow` | `checkpointed_at` | `anyOf: [Timestamp, null]` |
| `KitCheckpointAuthorsRow` | `checkpoint_id` | `string` |
| `KitCheckpointChangesRow` | `checkpoint_id` | `string` |
| `KitAlternativeCheckpointsRow` | `checkpoint_id` | `string` |
| `KitReleasesRow` | `checkpoint_id` | `string` (the table's `PRIMARY KEY`) |
| `ArtifactsRow` | `checksum` | `string` |

A second, independent defect surfaced while doing this: the coordinator's vitest config had no `include`
and `passWithNoTests: true`, so after the taxonomy sweep relocated the two suites to
`🎛️coordinator/🧪️tests/<case>/🟦️.ts` the project had been reporting green over **zero** tests. Fixed in
the same change; it is why the six missing columns could not have been caught by anyone re-running that
target.

---

## 4. Row 68 — client side

The framework half is already done: `🧰️framework/🔨️modules/🧬️schema/🔣️.json` is now the
`framework.schema` facet (`$id …/framework/schema/schema.json`, `$defs` `SchemaFormat`/`FacetLeaves`/
`SchemaExport`/…). The catalog array that used to live there is at
`💻️client/🧩️vscode/🗂️technologies.json`, byte-identical to `git show HEAD:` of the old file.

The vscode `ENTITY_EMOJIS` literal was a **lossy** duplicate: 56 pairs against the catalog's 58 entries,
missing exactly `todo` and `interaction-started` — the two entries whose emoji (📝️, 🌱️) is already used
by an earlier entry (`draft`, `technology-mono`). So the literal was the catalog deduplicated by emoji,
first-wins. The derivation now states that rule instead of hiding it:

```ts
export const ENTITY_EMOJIS: ReadonlyMap<string, string> = ((): ReadonlyMap<string, string> => {
  const index = new Map<string, string>();
  for (const entry of TECHNOLOGY_CATALOG) if (!index.has(entry.emoji)) index.set(entry.emoji, entry.id);
  return index;
})();
```

`new Map(catalog.map(e => [e.emoji, e.id]))` would have been **last**-wins and silently changed 🌱️ to
`interaction-started` and 📝️ to `todo`; the test pins both, and pins that exactly two entries are
shadowed. (Note `🧰️framework/🔨️modules/🧬️schema/🤖️generated.rs`'s `entity_kind_by_emoji` uses `.rev()`,
i.e. last-wins, and its docstring claims to mirror the TS `Map` — it mirrors neither the old literal nor
the new derivation. Framework partition; recorded as a request, not touched.)

The extension imports the JSON (`resolveJsonModule` is already on, and vite/rollup inlines it into
`out/extension.js`, so `.vscodeignore`'s `../**` is irrelevant) and parses it once at module load.

---

## 5. `🔌️mcp` GraphQL placement (wp4b §7.3)

- The SDL is now `🔌️mcp/🧬️schema/🔗️.graphql` with the draft-07 facet beside it.
- **No taxonomy edit was needed.** W9b expected the `memberNames` allowlist to block it; it does not.
  `🧬️schema` appears in *no* `memberNames` list in `🔣️taxonomy.json` (it is a fixed directory contract),
  which is also why W9b's `🪶️sqlite/🧬️schema` needed none. `schema audit` discovers the module and derives
  `repo.client.mcp` unchanged. What *is* now stale is the opposite direction: `🔗️graphql` was the only
  directory of that name in the repo, so `semanticDirectoryMemberKinds/members-of-members-of-members-of-modules`
  (`🔣️taxonomy.json:8560`) carries a dead member name — request 100 below, which supersedes the
  `memberNames` clause of row 71.
- Readers rewired: nothing reads the file by path. The GraphQL executor
  (`⌨️cli/internal/graphql/🔗️graphql.go`) is a hand-written mini engine and the VS Code typed documents
  embed their query strings, so the SDL had exactly two references in the tree — the deleted bundle's own
  `README.md`/`AGENTS.md`, and `⌨️cli/🧪️tests/🔬️component/🐹️.go`'s `repo/graphql` bundle-id expectation,
  removed with the bundle.
- `⌨️cli/🧩️component.go:18639,18645` still declare the `repo` policy's scopes as `"go/repo/main.go"`,
  `"js/vscode/package.json"`, `"graphql/repo/🔗️.graphql"`. **Not touched**: all three were already dead
  before this move (the graphql one never pointed at `🔌️mcp/🔗️graphql/` either), the policy body
  hard-codes `go/repo/main.go` in `repoPolicy()` as well, and repointing one of three would half-fix a
  policy whose semantics are a content decision. Still open, as W9b left it.

---

## 6. Verification (real output)

### 6.1 Vitest — the three suites in this partition

```
$ cd …/💻️client/🧩️vscode/📦️packages/🟦️typescript && bun ./📜️script.ts test quick
 Test Files  1 passed (1)
      Tests  22 passed (22)
   Start at  20:13:07
   Duration  3.73s

$ cd …/💻️client/🔌️mcp/📦️packages/🟦️typescript && bun ./📜️script.ts test
 Test Files  1 passed (1)
      Tests  67 passed (67)
   Start at  20:13:13
   Duration  2.84s

$ cd …/💻️client/🪶️sqlite/📦️packages/🟦️typescript && bun ./📜️script.ts test     (unchanged, regression check)
 Test Files  1 passed (1)
      Tests  12 passed (12)
   Start at  20:13:18
   Duration  1.25s
```

```
$ cd …/🖥️server/🎛️coordinator/📦️packages/🟦️typescript && bun ./📜️script.ts test long
ok  	github.com/usalu/semio/repo/server	13.840s

 RUN  v4.1.10 …/🖥️server/🎛️coordinator/📦️packages/🟦️typescript
 Test Files  2 passed (2)
      Tests  118 passed (118)
   Start at  20:03:13
   Duration  4.45s
```

The Go half of that run is `🧬️schema-contract/🐹️.go`, `🗄️g3-event-store/🐹️.go` and
`🗂️g3-filesystem/🐹️.go` through the overlay runner, i.e. the six added `<Table>Row` columns pass the Go
struct ↔ `$defs` parity check too. The default (`test`, no level) run of that same target is killed by the
runner's 15 s budget before Go finishes compiling — pre-existing, not caused by this change.

`bun ./📜️script.ts test` on `🔌️mcp/📦️packages/🐹️go`: `ok github.com/usalu/semio/repo/mcp 0.588s`.

### 6.2 `schema audit` — the new scopes are discovered without a taxonomy edit

```json
{"modulePath":"…/💻️client/🔌️mcp/🧬️schema","ownerPath":"…/💻️client/🔌️mcp","level":"product-module","facetKindId":"🧬️data","scopeId":"repo.client.mcp"}
{"modulePath":"…/💻️client/🧩️vscode/🧬️schema","ownerPath":"…/💻️client/🧩️vscode","level":"product-module","facetKindId":"🧬️data","scopeId":"repo.client.vscode"}
{"modulePath":"…/💻️client/🪶️sqlite/🧬️schema","ownerPath":"…/💻️client/🪶️sqlite","level":"product-module","facetKindId":"🧬️data","scopeId":"repo.client.sqlite"}
```

### 6.3 `schema check` — **0 findings in this partition**

```
$ bun ./📜️script.ts schema check --report <ticket>/🗑️generated/schema-check-w9d.jsonl
[schema check] wrote 10412 findings to …/schema-check-w9d.jsonl
[schema check] modules=3203 scopes=3018 findings=10412
[schema check] catalog-stale=1
[schema check] dependency-undeclared=21
[schema check] document-dialect-unexpected=827
[schema check] document-id-duplicate=15
[schema check] document-id-missing=11
[schema check] document-id-unaddressable=58
[schema check] export-format-missing=6576
[schema check] export-id-duplicate=650
[schema check] export-id-invalid=367
[schema check] fixture-defines-schema=109
[schema check] module-level-ineligible=63
[schema check] module-scope-id-inconsistent=215
[schema check] module-scope-id-missing=54
[schema check] mutation-aggregate-id-grammar=52
[schema check] mutation-aggregate-kinds-redundant=59
[schema check] mutation-leaf-id-grammar=1048
[schema check] placement-retired-location=17
[schema check] ref-not-catalog-addressable=121
[schema check] ref-unresolved=68
[schema check] scope-id-duplicate=80
```

Filtered to `🦑️repo/🔨️modules/**` minus `📚️library/**` and `🧪️test/**`:

| code | count |
|---|---|
| *(every code)* | **0** |

(`grep` for `🦑️repo/🔨️modules/💻️client` and for `🦑️repo/🔨️modules/🖥️server` in the report both return 0
lines. Ledger row 12 is clear for this partition.)

### 6.4 TypeScript

`bunx tsc --noEmit -p tsconfig.json` in `🧩️vscode/📦️packages/🟦️typescript` reports no error on the new
import, on `🗂️technologies.json` or on `🧬️schema/🟦️.ts`. The pre-existing errors are unchanged: TS5097
(`allowImportingTsExtensions`) from every `@semio-tech/framework` module reached through
`ephemeralBox`/`ephemeralMap`, plus six long-standing errors in the extension itself
(`🟦️.ts:875,877,1571,1575,1579,3099`). The new import deliberately uses the extensionless form
(`"../../🧬️schema/🟦️"`), which both `moduleResolution: "Bundler"` and vite resolve, so it adds no TS5097.

`bun ./📜️script.ts build` in that package still fails, unchanged and not from this work:
`RollupError: Module format "cjs" does not support top-level await` at
`🧰️framework/🔨️modules/🛂️manifest/🟦️.ts` — reached through the pre-existing `@semio-tech/framework`
import. Rollup reaches the *render* stage, which proves every module in the graph (the new JSON and the
new schema module included) resolved.

### 6.5 Go

`go build .` clean in `⌨️cli` and in `🔌️mcp`. The full `⌨️cli` suite through the canonical runner
(`bun ./📜️script.ts test long`, 177 s, log kept in the scratchpad) is **72 `--- FAIL` lines**, against
W9b's 67-line baseline of the same target. None of the delta is mine: my only Go change removes one row
from an expectation map, which can only remove failures. `TestBundleListIDs` was already failing on
**all 15** of its expected bundles (`expected bundle "compose/py" not found in list`, … ) because CLI
bundle discovery finds none of the current layout; the `repo/graphql` row I removed would have been a
16th line of the same failure. New in the delta since W9b, from peer churn, e.g.
`TestSpecExactIDs/session*` (`expected "…⚪️", got "…⚪"` — a VS16 stripping regression).

---

## 7. Cross-partition requests

Routed by the coordinator while this report was being written: **A**+**C** → ledger row 101, **B**+**D** → row 102,
**E** → row 103, **F**+**G** → row 104, **H** → row 105, **I** → row 106.

**A — `🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust/📜️script.ts:23` → W3c framework schema.
Broken right now, one-line fix.** `readEntityKinds()` still reads the file W3c replaced:

```ts
function readEntityKinds(): EntityKindSpec[] {
-  const path = join(ownerRoot, "🔣️.json");
+  const path = join(getWorkspaceRoot(), "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/🗂️technologies.json");
  return JSON.parse(readFileSync(path, "utf8")) as EntityKindSpec[];
}
```

Confirmed broken, not inferred:

```
$ cd 🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust && bun ./📜️script.ts check
TypeError: kinds.map is not a function.
      at emitTypeScript (…/📜️script.ts:31:6)
      at generatedTargets (…/📜️script.ts:114:63)
```

The same `generatedTargets()` feeds `generate`, `preview-generated` and `check`, so the entity-catalog
target is dead until this lands. It pairs with row 70 (the Go output path), which is already applied in that file (`💻️client/⌨️cli/🐹️entity_kinds.g.go`).

**B — `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json:24649` → W2c tooling.**
`schema-entity-catalog.inputPatterns` names `🧰️framework/🔨️modules/🧬️schema/🔣️.json` as the generator's
source. Replace with
`🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/🗂️technologies.json`. Land with A.

**C — `🧰️framework/🔨️modules/🧬️schema/🤖️generated.rs` (and the emitter in 97) → W3c.**
`entity_kind_by_emoji` resolves a shared emoji with `.rev()` (last-wins) and its docstring claims to
mirror `ENTITY_KIND_BY_EMOJI`'s "`Map` insertion-order semantics". Neither claim holds: the vscode
consumer is **first**-wins (`🌱️` → `technology-mono`, `📝️` → `draft`), which is now the documented rule
of `https://semio.tech/schema/repo/client/vscode/schema.json#/$defs/TechnologyCatalog`. Make the Rust and
generated-TS indexes first-wins, or state the divergence in the schema.

**D — `📚️library/🔣️taxonomy.json:8560` → W2c tooling.** `🔗️graphql` in
`semanticDirectoryMemberKinds/members-of-members-of-members-of-modules.memberNames` is now dead: it was
the name of the one bundle directory in the repo and that directory is gone. Remove the entry. This
*replaces* the `memberNames` clause of row 71 — no allowlist ever blocked the move, so no addition is
needed either (`🧬️schema` is a fixed directory contract and appears in no `memberNames` list).

**E — `📚️library/📦️packages/🟦️typescript/🧫️fixtures/…` → W2c tooling.** Two authority fixtures carry
rows for files this change removed or that the taxonomy sweep already moved:

- `⚖️readme-license-owner-authority/🔣️.json:1021` — `sourcePath`
  `…/💻️client/🔌️mcp/🔗️graphql/README.md` (and its `destinationPath`
  `…/🔗️graphql/📃️readme/📝️.md`). Drop the row; if the authority is exhaustive, add one for the new
  `…/💻️client/🔌️mcp/README.md`.
- `🧼️remaining-package-purity-authority/🔣️.json:5007` — the row for
  `…/🧩️vscode/📦️packages/🟦️typescript/🧪️extension.test.ts`, a path that no longer exists; its own
  `move-test-implementation` target field says `…/🧩️vscode/🧪️tests/🧪️extension/🟦️.ts` while the real
  location is `…/🧩️vscode/🧪️tests/🧩️extension/🟦️.ts` (🧩, not 🧪). New authored files that belong in the
  same fixture if it is exhaustive: `🧩️vscode/🧬️schema/🟦️.ts`,
  `🔌️mcp/📦️packages/🟦️typescript/📜️script.ts` and `…/🧪️tests/🟦️.ts`.

**F — root `.vscode/launch.json` → W2c tooling.** One new nx project needs an entry, next to W9b's
still-open `🪶️sqlite schema test` request (row 71). Insert after it, same `group: "3_dev"`, shifting the
following order values:

```json
    {
      "name": "🔌️mcp schema test",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-mcp-schema:test",
      "cwd": "${workspaceFolder}",
      "presentation": { "group": "3_dev", "order": -1 }
    },
```

The `@semio-tech/repo-vscode:test*` targets already exist and are unchanged; they now additionally run the
vitest schema case, so no new entry is needed for them.

**G — root `package.json` `workspaces` → W2c tooling.** Add
`🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp/📦️packages/🟦️typescript` beside the existing
`…/🪶️sqlite/📦️packages/🟦️typescript` entry. Not blocking — nx discovers the project by `📋️project.json`
and the suite runs today — but it keeps the two client TS packages declared the same way.

**H — W2c catalog owner.** `📚️library/🔣️schema-catalog.json` needs the two new scopes on the next
`schema generate`; both `dependsOn: []`:

```json
"repo.client.vscode": {
  "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/🧬️schema",
  "level": "product-module", "facetKind": "🧬️data",
  "formats": { "🔣️jsonschema": "🔣️.json", "🟦️typescript": "🟦️.ts" },
  "exports": ["TechnologyCatalog", "TechnologyCatalogEntry"], "dependsOn": []
},
"repo.client.mcp": {
  "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp/🧬️schema",
  "level": "product-module", "facetKind": "🧬️data",
  "formats": { "🔣️jsonschema": "🔣️.json", "🔗️graphql": "🔗️.graphql" },
  "exports": ["AnalyzeMetrics", "AnalyzeResult", "Breach", "BreachPriority", "Bundle", "BundleKind", "Checkpoint",
              "Contributor", "ContributorAddInput", "ContributorBundle", "ContributorContributions",
              "ContributorDefinition", "ContributorFile", "ContributorFolder", "ContributorIcons",
              "ContributorLink", "ContributorSection", "DateTime", "Definition", "DefinitionKind", "File",
              "FileKind", "FilterInput", "FixResult", "Folder", "FolderKind", "Goal", "GoalChangeInput",
              "GoalCreateInput", "Interaction", "InteractionResource", "LineMetrics", "Location", "Mutation",
              "Node", "Policy", "PriorityCount", "Query", "Range", "Repo", "Section", "Statute", "Technology",
              "TechnologyKind", "Territory", "Ticket", "TicketBundleContrib", "TicketChangeInput",
              "TicketClient", "TicketCloseInput", "TicketDate", "TicketDay", "TicketFileContrib",
              "TicketMonth", "TicketOpenInput", "TicketReopenInput", "TicketSectionContrib", "TicketStatus",
              "TicketYear", "Todo", "TodoChangeInput", "TodoCreateInput"],
  "dependsOn": []
}
```

**I — coordinator / whoever owns the taxonomy test sweep.** Every `*_test.go` under `🦑️repo` is now
`🧪️tests/<case>/🐹️.go` inside an emoji directory, which Go cannot import
(`malformed import path …: invalid char '🧪'`). This is *not* broken — `runCanonicalGoTests` materializes
an overlay — but it means a bare `go test ./...` / `go vet ./...` in `⌨️cli`, `🔌️mcp` or `🎛️coordinator`
now fails at package load with nothing but that message. Any worker or CI step that shells out to `go`
directly instead of the package router will read that as a catastrophic failure. Worth one line in the
product AGENTS/README by whoever owns them.

---

## 8. Open questions / not acted on

1. **`Query`/`Mutation` field arguments** have no JSON Schema expression. They live in the SDL only and
   the JSON exports carry a `$comment` saying so. If contract §B's "every export in every format" is read
   strictly, these two exports are format-restricted and should carry
   `"x-semio-formats": ["🔗️graphql", "🔣️jsonschema"]` with a note — but that annotation says nothing
   useful when both formats *are* present and only one keyword is unrepresentable. Left as a `$comment`;
   a contract clarification would settle it.
2. **`DateTime`'s pattern is asserted, not observed.** I set the RFC-3339 pattern the coordinator's
   `Timestamp` uses, because the Go resolvers marshal `time.Time`. I did not run the CLI executor to
   confirm the wire spelling.
3. **`repo.client.vscode` has no Rust or proto facet**, and `repo.client.mcp` has no TypeScript facet even
   though the VS Code extension is its biggest consumer (it embeds query strings, not types). Generating a
   TS projection of the 62 GraphQL exports is the obvious next step and would make the extension's typed
   documents checkable; it is a bigger piece of work than this row and nothing consumes it yet.
4. **`⌨️cli`'s `repo` policy** (`🧩️component.go:18639-18645`, `repoPolicy()` at `:20543`) reads
   `go/repo/main.go`, `js/vscode/package.json` and `graphql/repo/🔗️.graphql` — three paths that have not
   existed since the taxonomy migration, so the policy breaches unconditionally. Content decision, left
   for the repo-policy sweep (same finding as `📓️wp4b-repo-client.md` §7.3).
5. **`💻️client/client` and `💻️client/mcp`** (10.5 MB and 12.6 MB Mach-O) are still untracked build
   residue at the module root, as W9b reported. `📦️packages/🐹️go`'s declared output root is exactly
   `💻️client/mcp`, so this is by design and only the `.gitignore` coverage is missing.
6. **`🧩️vscode/📦️packages/🟦️typescript/📜️script.ts`'s `lint` target** runs
   `eslint --config 🟦️eslint.config.ts`, and no such file exists in the package. Pre-existing; not in the
   row's scope.
