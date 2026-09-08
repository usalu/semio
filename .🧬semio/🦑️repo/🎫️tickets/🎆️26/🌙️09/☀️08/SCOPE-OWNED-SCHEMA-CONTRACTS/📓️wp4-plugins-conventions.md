# WP4-plugins — binding conventions for every worker on the `✏️s/**` partition

Derived from `📋️execution-contract.md` §A/§B/§D. Every file this work package writes obeys these.

## 1. Module file

`<scope>/🧬️schema/🔣️.json`:

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "$id": "https://semio.tech/schema/<scope path>/<facet>.json",
  "title": "<RootExport>",
  ...root schema...,
  "$defs": { "<ExportId>": { ... } }
}
```

- Dialect is **always** `http://json-schema.org/draft-07/schema#`. Migrate every 2020-12 document:
  `prefixItems: [A,B]` → `items: [A,B]` + `additionalItems: false`; `items: false` after `prefixItems`
  → `additionalItems: false`; `$defs` stays (draft-07 tolerates it and Ajv resolves `#/$defs/...`).
- A module that already exists keeps its `$id`; new exports are added as `$defs` keys only.
- A **new** module gets `$id`:
  - plugin root: `https://semio.tech/schema/s/<plugin>/schema.json` (scope id `s.<plugin>`)
  - other new scope: `https://semio.tech/schema/s/<plugin>/<segment>/schema.json`
- Export ids are PascalCase `$defs` keys. Fixture data binds `schema://<scope id>/<ExportId>`.

## 2. What moves and what stays

- The **contract** moves into the owner module `$defs`. The **example data** stays in its fixture
  directory (`🔣️.json` and friends are untouched).
- A fixture-local `*.schema.json` / `🧬️.schema.json` is **deleted** in the same change. No aliases,
  no re-export files, no redirect stubs.
- Contract §B: a "spec of one example" wrapper is not a contract. Re-derive the real contract from
  producer/consumer code:
  - `"$schema": { "const": "./<sibling>.schema.json" }` self-reference properties are **dropped**
    (they only existed to point at the deleted file) — and the corresponding key is removed from the
    data file and from `required`.
  - Hostile/negative expectations expressed as schema become fixture expectations
    `{ "stage": …, "result": "rejected", "code": … }`, not schema keywords.
- Genuine per-artifact laws (`const` route censuses, byte budgets, oneOf arm counts) are load-bearing
  and are preserved verbatim inside the owner's export.

## 3. Shared retained-command shape (cross-partition)

The retained-command family shares a *shape*, not content. The shape is already published by
`🧰️framework/🔨️modules/🖱️ui/🧬️schema/🔣️.json`, `$id https://semio.tech/schema/framework/ui/schema.json`
(another worker's partition — do not edit it). **Read that file before writing any `$ref`.** As of
this writing its `$defs` are exactly:

- `RetainedCommandLimits` — the whole retained-command-limits *document*
  (`oneOf` of `retainedCommandDeclaredLimits` / `retainedCommandCorpusLimits`)
- `RetainedCommandRoutes` — the route *array*
- `RetainedCommandRoutesDocument`, `UIDialogModalFixture`

There is **no** `RetainedCommandRoute` (singular) `$defs` export. Never `$ref` one; never `$ref` into
that module's `definitions` (contract §A: cross-scope refs point at `$id` + `#/$defs/<ExportId>`).

Per-artifact exports reference the document-level export and narrow it:

```json
"<Artifact>RetainedCommandLimits": { "allOf": [
  { "$ref": "https://semio.tech/schema/framework/ui/schema.json#/$defs/RetainedCommandLimits" },
  { "type": "object", "additionalProperties": false, "required": [...],
    "properties": { ...the per-artifact const law verbatim... } } ] }
```

and narrow the route table at the array level:

```json
"routes": { "allOf": [
  { "$ref": "https://semio.tech/schema/framework/ui/schema.json#/$defs/RetainedCommandRoutes" },
  { "type": "array", "minItems": 18, "maxItems": 18, "items": [ ... ], "additionalItems": false } ] }
```

Known gap in the framework export (report it, do not work around it in committed code): its
`definitions.retainedCommandOracle` is `additionalProperties:false` over `library`/`scope`/
`runtimeDependency` only, so a fixture whose `oracle` also carries `ownedInterface`/`expected`
cannot satisfy the shared document shape. Where that bites, `$ref` only what does fit and record the
needed framework change.

Consumers that compile with Ajv must `addSchema(frameworkUiModule)` before compiling the export.

## 4. Consumer rewiring

A `📜️script.ts` oracle that read `<fixture>/🧬️.schema.json` now reads the owner module once and
picks the export:

```ts
const module = JSON.parse(readFileSync(join(scopeRoot, "🧬️schema", "🔣️.json"), "utf8"));
const ajv = new Ajv({ strict: true, allErrors: true });
ajv.addSchema(module);                       // registers the module under its own $id
const validate = ajv.compile({ $ref: `${module.$id}#/$defs/<ExportId>` });
```

- `Ajv` (draft-07 default export) replaces `Ajv2020` wherever the moved document was 2020-12.
- Rust keeps `include_str!` of the **data** file; only a Rust site that included a *schema* file is
  repointed at the owner module `🔣️.json`.

## 5. Hygiene

`git ls-files '✏️s' | grep -E '(🧪️|🧫️|🚧️)' | grep -E 'schema\.json$' | grep -v '🧬️schema/🧬️mutations/'`
must be empty when this work package closes.
