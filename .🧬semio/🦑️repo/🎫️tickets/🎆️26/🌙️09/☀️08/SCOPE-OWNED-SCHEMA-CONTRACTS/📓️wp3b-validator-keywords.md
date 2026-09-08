# WP3b — owned draft-07 validator: full keyword coverage, repo-wide compile proof, runtime entries dump

Partition: `🧰️framework/🔨️modules/🧬️schema/**` only. Crate `semio-framework-schema`. No new dependency;
`ajv` + `ajv-formats` stay test-only oracles. Continues `📓️wp2-rust-schema-registry.md`.

## 1. Files changed

| File | Change |
|---|---|
| `🧰️framework/🔨️modules/🧬️schema/✅️validator.rs` | `if`/`then`/`else`, `dependencies` (both forms), `propertyNames`, `contains`, `patternProperties`, `minProperties`/`maxProperties`, `contentMediaType`/`contentEncoding`; pinned `format` policy (`ASSERTED_STRING_FORMATS`, `string_format_matches` + owned RFC 3339 / RFC 3986 / email / uuid / regex predicates); `$ref` to the document's own `$id`; lookahead `(?=…)`/`(?!…)` in `PatternMatcher` |
| `🧰️framework/🔨️modules/🧬️schema/🧫️fixtures/✅️draft07-validation-vectors.json` | renamed from `🧪️fixtures/` (taxonomy `testFixturesDirName`, contract §B) and extended from 7 to **16 cases / 36 valid / 73 invalid instances** |
| `🧰️framework/🔨️modules/🧬️schema/🧪️tests/✅️draft07-oracle/🟦️.ts` | `ajv-formats` added as the `format` oracle; corpus floor raised to 16; fixture path follows both renames. A repo sweep moved this file here from `🧬️schema/✅️draft07-oracle.test.ts` mid-session — see §7.1 |
| `🧰️framework/🔨️modules/🧬️schema/⚛️component.rs` | `include_str!` follows the fixture rename; vector floor 16; `owned_pattern_matcher_covers_the_supported_ecma_subset` extended with three lookahead patterns, `(?<=…)`/`(?<!…)` moved to the rejected list |
| `🧰️framework/🔨️modules/🧬️schema/🧪️tests/🧩️module-compile/🦀️.rs` | **new** — repo-wide compile walk + per-path unsupported-keyword inventory |
| `🧰️framework/🔨️modules/🧬️schema/🧪️tests/📤️schema-export-entries/🦀️.rs` | **new** — runtime `schema_export_catalog_entries()` dump |
| `<ticket>/wp3b-vitest-oracle.config.ts` | **new** — vitest config that includes the oracle spec at its taxonomy path, which the default `**/*.{test,spec}.*` glob does not match |
| `🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust/Cargo.toml` | two `[[test]]` targets (`schema-export-entries`, `schema-module-compile`), ASCII `name`, emoji `path` per the repo's existing `[[test]]` convention |

Nothing was deleted; the fixture rename is a move with both readers rewired.

## 2. Keyword coverage

`validate_schema_node` still rejects any keyword not in this table — unknown keywords are a hard
compile error, never silently ignored. `x-*` (so every `x-semio-*`) passes through as an extension.

| Keyword | Status | Behaviour |
|---|---|---|
| `type` (string and array), `enum`, `const` | assertion | `integer` accepts a float with zero fraction |
| `properties`, `required`, `additionalProperties` | assertion | `additionalProperties` skips names matched by `properties` **or** `patternProperties` |
| `patternProperties` | assertion **(new)** | patterns compiled once at schema-compile time and cached |
| `propertyNames` | assertion **(new)** | the name is validated as a JSON string; failure reported at the object's path (ajv's attribution) |
| `dependencies` — array form | assertion **(new)** | listed names must be present when the key is |
| `dependencies` — schema form | assertion **(new)** | the whole object is validated against the subschema |
| `minProperties` / `maxProperties` | assertion **(new)** | |
| `items` (single schema and tuple), `additionalItems` | assertion | |
| `contains` | assertion **(new)** | at least one item matches; an empty array always fails |
| `minItems` / `maxItems`, `uniqueItems` | assertion | `uniqueItems` is metered by the traversal node budget |
| `minimum` / `maximum` / `exclusiveMinimum` / `exclusiveMaximum` | assertion | draft-07 **numeric** form only; the draft-04 boolean form is a compile error |
| `multipleOf` | assertion | positive divisor enforced at compile time; float comparison with an 8·ε relative tolerance |
| `minLength` / `maxLength` | assertion | counted in `char`s, not bytes |
| `pattern` | assertion | owned `PatternMatcher`, unanchored search |
| `format` | **pinned policy, see §3** | assertion for seven values, annotation for everything else |
| `allOf` / `anyOf` / `oneOf` / `not` | assertion | branch arrays must be non-empty (draft-07 meta-schema) |
| `if` / `then` / `else` | assertion **(new)** | `if` never contributes an error of its own; the taken branch does |
| `$ref` | resolution | `#`, `#/…`, the document's **own `$id`** (new), and any registered sibling document by `$id`; recursion is cycle-guarded |
| `$defs`, `definitions` | container | both spellings walked |
| `$id`, `$schema`, `$anchor` | annotation | `$schema` is **not** enforced — dialect hygiene is a file check (WP7), see `📓️wp2-rust-schema-registry.md` §3.4 |
| `title`, `description`, `$comment`, `default`, `examples` | annotation | `examples` must be an array, `default` unconstrained |
| `contentMediaType`, `contentEncoding` | annotation **(new)** | draft-07 annotations; never assert, matching ajv's default |
| `readOnly`, `writeOnly`, `deprecated` | annotation | must be boolean |
| `x-*` (incl. every `x-semio-*`) | annotation | body unconstrained |

Deliberately **not** implemented, and therefore a hard compile error: `unevaluatedProperties`,
`unevaluatedItems`, `prefixItems`, `$dynamicRef`, `$dynamicAnchor`, `$vocabulary`,
`dependentRequired`, `dependentSchemas` (2019-09/2020-12), and `discriminator`/`nullable` (OpenAPI).
Contract §B says 2020-12 documents are migrated, not accommodated; no compatibility layer was added.

### `PatternMatcher` subset change

Lookahead `(?=…)` and `(?!…)` are now supported — 11 repo modules (path-safety guards such as
`^(?!/)(?!.*(?:^|/)\.\.(?:/|$)).+$` and nonzero-hash guards `^(?!0{64}$)[0-9a-f]{64}$`) are
unrepresentable without them. Still rejected at compile time rather than silently ignored:
backreferences, lookbehind `(?<=…)`/`(?<!…)`, named groups, `\b`/`\B`. This supersedes the
"lookaround is outside the supported subset" statement in `📓️wp2-rust-schema-registry.md` §3.3.

## 3. The pinned `format` policy

`ASSERTED_STRING_FORMATS` in `✅️validator.rs` is the whole policy and is exported:

```rust
pub const ASSERTED_STRING_FORMATS: [&str; 7] = ["date", "date-time", "email", "regex", "time", "uri", "uuid"];
pub fn string_format_matches(format: &str, value: &str) -> Option<bool>;   // None ⇒ annotation-only
```

| Format | Rule (implemented in-house, no crate) |
|---|---|
| `date` | RFC 3339 `full-date`, real calendar days incl. the Gregorian leap rule (`2024-02-29` ok, `2026-02-29` not) |
| `time` | RFC 3339 `full-time`: `HH:MM:SS[.fraction]` with a **mandatory** `Z`/`±HH[:]MM` offset; offset hour ≤ 23, minute ≤ 59; the `23:59:60` leap second is accepted only when it lands on 23:59 UTC |
| `date-time` | exactly one `t`/`T`/whitespace separator, `date` before it and `time` after it |
| `email` | dot-separated atoms `[A-Za-z0-9!#$%&'*+/=?^_\`{\|}~-]+` before `@`; at least two LDH labels after it, each starting and ending alphanumeric |
| `uri` | RFC 3986 absolute URI: mandatory scheme, optional `//authority` (userinfo/IP-literal/reg-name/port), path/query/fragment restricted to unreserved + sub-delims + pct-encoded. `uri-reference` is *not* this format, so a bare `/schema/x.json` fails |
| `uuid` | optional case-insensitive `urn:uuid:` prefix + `8-4-4-4-12` hex |
| `regex` | the value must compile through the owned `PatternMatcher` |

Everything else — `double` (666 uses), `uint32` (190), `float` (79), `int64` (28), `base64` (17): the
complete set of `format` values in the repo today — is **annotation-only** and can never reject an
instance. Turning assertions on therefore changed no existing module's verdict.

Two deliberate divergences from the `ajv-formats` oracle, both narrowing:

- `regex` is asserted against the owned subset, so a value using a backreference or lookbehind is
  rejected here and accepted by ajv. The vector corpus stays inside the intersection.
- `int32`/`int64`/`float`/`double` are numeric assertions in `ajv-formats` but annotations here.
  Only `int64` differs observably (ajv requires an integer). The repo uses `int64` exclusively on
  proto-derived integer fields that already carry `type: integer`, so nothing is weakened; if a scope
  ever needs the assertion it should state `type: integer`, not lean on `format`.

## 4. Repo-wide compile proof

`cargo test -p semio-framework-schema --test schema-module-compile`
(`🧰️framework/🔨️modules/🧬️schema/🧪️tests/🧩️module-compile/🦀️.rs`). It walks `🌎️hub`, `🧰️framework`
and `✏️s` for every `🧬️schema/🔣️.json` (mutation leaves **included**), skipping
`node_modules`/`target`/`dist`/`.git`/`.nx`/`🗑️generated`. The repo root is found by walking up from
`CARGO_MANIFEST_DIR` to the directory carrying `.🧬semio`; `SEMIO_SCHEMA_MODULE_ROOT` overrides it.
Each module is compiled with the sibling documents its own `$ref`s name, keyed by `$id`.
`SEMIO_SCHEMA_MODULE_COMPILE_OUT` writes the full JSON report
(`🗑️generated/wp3b-module-compile.json`).

```
[schema-module-compile] modules=3161 compiled=3108 draft07Failures=33 otherDialectFailures=20 collidingIds=1
[schema-module-compile] draft-07 failure category nonDraft07Keyword: 1
[schema-module-compile] draft-07 failure category unresolvedReference: 32
test every_draft07_schema_module_compiles_through_the_owned_validator ... ok
```

Dialects across the 3161 modules: **2962 draft-07**, 27 `2020-12`, 172 with no `$schema`.

**Zero `draft07KeywordGap` failures** — that is the assertion the test enforces: a draft-07 module
that fails on a keyword the validator should implement fails the test. The other categories are
inventories of `📋️execution-contract.md` §B violations owned by other partitions and are reported,
not asserted on, because peers add and fix modules while the test runs (the module count moved
3143 → 3161 and `otherDialectFailures` 78 → 20 during this session as WP4/WP5 landed migrations).

Remaining 33 draft-07 failures:

| Count | Category | Owner |
|---|---|---|
| 32 | `$ref` naming a relative path (`../../../../🧬️schema/🔣️.json#/$defs/I32`), a `semio:`/`https://semio.dev/…` id that no module declares, or a document that does not exist — §B requires the target's `$id` | os `🏪️store/🧫️fixtures` (13), os `📡️spr/🎮️command/🧪️tests` (5), os `♾️infinite` (4), plugin `📸️remodel` (3), os `🔌️plugin/🧪️tests` (3), 4 others |
| 1 | `discriminator` (OpenAPI) at `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🔣️.json` | os directory |

Remaining 20 non-draft-07 failures: 12 unresolved refs and 4 malformed bodies in `<undeclared>`
documents, 3 × `unevaluatedProperties`/`prefixItems` and 1 unresolved ref in `2020-12` documents.
One of the malformed bodies is in this partition, see §6.

Per-keyword inventory over all dialects: `discriminator` ×1, `prefixItems` ×1, `version` ×2 (two
`🌉️wasm`/`🕸️wasm` `🔣️.json` files that are not JSON Schema documents at all). One `$id` collision
is reported (`collidingIds=1`); the paths are in the JSON report.

## 5. Runtime entries dump — contract for `schema verify --rust-entries`

`cargo test -p semio-framework-schema --test schema-export-entries`
(`🧰️framework/🔨️modules/🧬️schema/🧪️tests/📤️schema-export-entries/🦀️.rs`). Its own test binary, so the
dump carries only what the linked crates register and nothing a sibling unit test left in the
process-wide catalog. With `SEMIO_SCHEMA_EXPORT_ENTRIES_OUT` set it writes:

```json
{
  "contractId": "schema-export-registry-entries-v1",
  "generator": "cargo test -p semio-framework-schema --test schema-export-entries",
  "entries": [
    { "scope": "framework.schema.entries", "export": "SchemaExportEntry", "format": "jsonschema" },
    { "scope": "framework.schema.entries", "export": "SchemaExportEntry", "format": "rust" },
    { "scope": "framework.schema.entries", "export": "SchemaExportEntry", "format": "typescript" }
  ]
}
```

Contract, as consumed by `SchemaRustEntryDump` / `schemaRustEntryDiagnostics` in
`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts`:

- `contractId` — the taxonomy `schemaExportResolution.rustEntriesContractId`, today
  `"schema-export-registry-entries-v1"`. The consumer bails with `rust-entries-contract-unknown`
  on any other value, so the two must be changed together.
- `generator` — the exact command that produced the file.
- `entries` — every `(scope, export, format)` triple that resolves to a **non-empty** leaf, sorted
  and deduplicated by that triple. `format` is the ascii `SchemaFormat::id()` spelling
  (`rust`/`typescript`/`graphql`/`jsonschema`/`protobuf`); `SCHEMA_FORMAT_TAXONOMY_KEYS` in `🟦️.ts`
  maps those to the emoji `schemaFormats` keys. An export whose leaf for a format is `""` produces
  **no** entry — asserted by the test for `graphql`/`protobuf`.

The shape was aligned to the already-wired consumer rather than to the shape my brief sketched
(`{ entries: [...] }`): `📜️script.ts` `schema verify --rust-entries <file>` is live today and would
reject a dump without `contractId`/`generator`.

The three entries above come from the fixture scope the test registers itself. The dump is only as
complete as the crates linked into the process: `semio-framework-schema` alone registers nothing, so
`schema verify --rust-entries` must be pointed at a dump produced by a binary that links the scopes
it wants compared, and `--rust-entries-complete` will report every catalogued scope as
`rust-scope-unregistered` until WP4/WP5 add the `register_scope_schema_exports` calls
(`📓️wp2-rust-schema-registry.md` §5.1).

## 6. Verification output

### 6.1 `cargo test -p semio-framework-schema` (full log `🗑️generated/wp3b-cargo-test.txt`)

```
     Running unittests 🦀️.rs (…/target-w3/debug/deps/semio_framework_schema-b8be941b77a6388e)
running 26 tests
test result: ok. 26 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

     Running ../../🧪️tests/📤️schema-export-entries/🦀️.rs (…/deps/schema_export_entries-3c46f74fad9b8f6b)
running 1 test
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s

     Running ../../🧪️tests/🧩️module-compile/🦀️.rs (…/deps/schema_module_compile-273386fdbc58d1af)
running 1 test
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 32.08s

   Doc-tests semio_framework_schema
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

### 6.2 `cargo clippy -p semio-framework-schema --all-targets`

```
warning: `semio-framework-replication` (lib) generated 2 warnings
warning: `semio-framework-os-kernel` (lib) generated 1 warning
    Finished `dev` profile [unoptimized] target(s) in 1m 10s
```

Zero warnings attributed to `semio-framework-schema`. The three that remain are pre-existing
`large_enum_variant` warnings in peer crates outside this partition.

### 6.3 ajv + ajv-formats oracle (full log `🗑️generated/wp3b-ajv-oracle.txt`)

`bunx vitest run --config <ticket>/wp3b-vitest-oracle.config.ts --reporter=verbose`
(the spec lives at `🧰️framework/🔨️modules/🧬️schema/🧪️tests/✅️draft07-oracle/🟦️.ts`, which vitest's
default include glob does not match — see §7.1)

```
 ✓ … > declares the draft-07 dialect on every case 2ms
 ✓ … > agrees with ajv on object-required-and-additional-properties 31ms
 ✓ … > agrees with ajv on string-pattern-and-enum-and-const 88ms
 ✓ … > agrees with ajv on array-list-items-and-bounds 78ms
 ✓ … > agrees with ajv on array-tuple-items-and-additional-items 64ms
 ✓ … > agrees with ajv on combinators-one-of-any-of-all-of 62ms
 ✓ … > agrees with ajv on local-ref-into-definitions 10ms
 ✓ … > agrees with ajv on cross-document-ref-by-id 5ms
 ✓ … > agrees with ajv on conditional-if-then-else 6ms
 ✓ … > agrees with ajv on object-dependencies-property-and-schema-forms 4ms
 ✓ … > agrees with ajv on object-property-names-and-counts 10ms
 ✓ … > agrees with ajv on object-pattern-properties-and-additional 27ms
 ✓ … > agrees with ajv on array-contains-and-unique-items 4ms
 ✓ … > agrees with ajv on numeric-exclusive-bounds-and-multiple-of 46ms
 ✓ … > agrees with ajv on annotations-and-negation 36ms
 ✓ … > agrees with ajv on string-formats-pinned-assertions 341ms
 ✓ … > agrees with ajv on annotation-only-formats-never-reject 9ms

 Test Files  1 passed (1)
      Tests  17 passed (17)
```

`ajv-formats@3.0.1` resolves from `node_modules` alongside `ajv@8.20.0`; no hand-rolled format
substitute was needed. Both are still transitive-only and should be declared explicitly in root
`devDependencies` (`📓️wp2-rust-schema-registry.md` §5.4 extends to `ajv-formats`).

Oracle contract, unchanged from WP2 and now covering nine more cases: both implementations must
agree on the **verdict** for all 109 instances; the owned validator's reported error path must equal
the vector's `errorPath` exactly, and ajv must report an error at that path or at one of its
ancestors (ajv attributes `additionalProperties`, `propertyNames`, `dependencies`, `contains`,
`uniqueItems`, `if` and `oneOf` failures to the containing object/array).

Three lookahead expectations were cross-checked against V8 directly rather than only against ajv:

```
$ node -e 'console.log(/(?=.*x)ab/.test("xab"), /(?=.*x)ab/.test("abx"), /^(?!0{4}$)[0-9a-f]{4}$/.test("0000"), /^(?!\/)(?!.*(?:^|\/)\.\.(?:\/|$)).+$/.test("a/..b"))'
false true false true
```

### 6.4 New vector cases

`conditional-if-then-else`, `object-dependencies-property-and-schema-forms`,
`object-property-names-and-counts`, `object-pattern-properties-and-additional`,
`array-contains-and-unique-items`, `numeric-exclusive-bounds-and-multiple-of`,
`annotations-and-negation`, `string-formats-pinned-assertions`,
`annotation-only-formats-never-reject`. 16 cases, 36 valid and 73 invalid instances, every invalid
one carrying its expected error path.

## 7. Cross-partition requests

1. **`📜️script.ts` vs. the taxonomy sweep — the oracle spec path.** Twice during this session a repo
   tool moved `🧰️framework/🔨️modules/🧬️schema/✅️draft07-oracle.test.ts` to the taxonomy location
   `🧰️framework/🔨️modules/🧬️schema/🧪️tests/✅️draft07-oracle/🟦️.ts` (I restored the old path once; it
   was moved back within ten minutes, so the sweep is authoritative and recurring). The file now
   lives at the taxonomy path with its fixture path corrected to `"..", "..", "🧫️fixtures"`, and
   **`schema test` is broken until the root script follows**, in two ways:
   - `SCHEMA_DRAFT07_ORACLE_SPEC` (`📜️script.ts:22732`) still names
     `"🧰️framework/🔨️modules/🧬️schema/✅️draft07-oracle.test.ts"`, a path that no longer exists. It
     must become `"🧰️framework/🔨️modules/🧬️schema/🧪️tests/✅️draft07-oracle/🟦️.ts"`.
   - vitest's default include is `**/*.{test,spec}.?(c|m)[jt]s?(x)`, which does **not** match
     `🟦️.ts`. `bunx vitest run --root . <spec>` therefore exits 1 with "No test files found" — a
     silent green in a shell that ignores the exit code. The invocation at `📜️script.ts:22868` needs
     a config whose `include` names the spec; `<ticket>/wp3b-vitest-oracle.config.ts` is a working
     one-line example and is what §6.3 was run with. Whether every repo `🧪️tests/<slug>/🟦️.ts` gets
     one shared vitest config is the root-script owner's call.
2. **Root `📜️script.ts` / `📋️project.json` / `.vscode/launch.json`.** Two new commands to register:
   `cargo test -p semio-framework-schema --test schema-module-compile` (the repo-wide compile gate,
   with `SEMIO_SCHEMA_MODULE_COMPILE_OUT` for the report) and
   `cargo test -p semio-framework-schema --test schema-export-entries` with
   `SEMIO_SCHEMA_EXPORT_ENTRIES_OUT=<file>` as the producer for `schema verify --rust-entries <file>`.
3. **Root `package.json`.** Declare `ajv` **and** `ajv-formats` explicitly in `devDependencies`; both
   resolve today only as hoisted transitive dependencies. They must stay dev-only.
4. **32 `$ref` targets that name a path instead of an `$id`** (§B). Largest groups:
   `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧫️fixtures/**/🧬️mutations/*/🧬️schema/🔣️.json` (13,
   `../../../../🧬️schema/🔣️.json#/$defs/I32`), `📡️spr/🎮️command/🧪️tests/🧬️mutation-laws/**` (5,
   `../../../📐️schema/🔣️.json#/$defs/I64`), `♾️infinite/**` (4),
   `✏️s/🔌️plugins/📸️remodel/**/✏️editor/{🎚️config,👥️presence}/🧬️schema/🧬️mutations/*` (3,
   `../../../🔣️.json`), `🔌️plugin/🧪️tests/🛰️declaration-channels/**` and
   `🔌️plugin/⚛️reactor/💼️jobs/🧬️mutation-plan/**` (3, `https://semio.dev/schema/…` ids no module
   declares), `🔌️plugin/📇️registry/📦️deployment` (1, `semio:installation-directory-v1`). Each must
   become the target document's `$id` + `#/$defs/<ExportId>`. Full list in
   `🗑️generated/wp3b-module-compile.json`.
5. **`🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🔣️.json`** uses the OpenAPI
   `discriminator` keyword inside a draft-07 document. Express the variant selection as `oneOf` with
   a `const` tag property.
6. **Two `🔣️.json` files inside a `🧬️schema/` module are not JSON Schema documents**:
   `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🧬️schema/🔣️.json`
   and `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🧬️schema/🔣️.json` (top-level `version`,
   plus method lists). They are wasm surface descriptors occupying the schema slot.
7. **This partition's own `🔣️.json` is not a schema either.**
   `🧰️framework/🔨️modules/🧬️schema/🔣️.json` is a 6.4 kB JSON **array** of technology catalog entries
   (`{id, emoji, iconId, label, filterable}`, `technology-user`, `technology-infrastructure`, …),
   last touched 2026-09-02, with **no** path reader anywhere in the tree; the same catalog is
   duplicated in
   `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/🟦️.ts`. It is
   in my partition, but deciding whether that data is deleted (as a duplicate) or relocated into the
   vscode client scope touches a partition I do not own, and the `framework.schema` module's real
   JSON Schema facet (`$id https://semio.tech/schema/framework/schema/schema.json`, `$defs` for
   `SchemaExport`, `ScopeSchemaExports`, `SchemaExportEntry`, `SchemaResolveError` — the TS twins
   already exist in `🟦️.ts`) is a catalog-shaped decision for the WP2/WP7 owner. I left the file
   untouched and report it rather than guessing. It shows in the compile report as
   `<undeclared>` / `$: schema must be an object or boolean`.
8. **One `$id` collision** across the 3161 modules (`collidingIds=1`, paths in
   `🗑️generated/wp3b-module-compile.json`). §B makes `$id` the resolution key, so this must be
   resolved before the catalog generator can key on it. Whether `schema check` enforces `$id`
   uniqueness statically stays the WP7 decision flagged in `📓️wp2-rust-schema-registry.md` §6.

## 8. Open questions

- **`enum: []`.** The draft-07 meta-schema puts `minItems: 1` on `enum`, and ajv refuses to compile
  such a schema ("data/propertyNames/enum must NOT have fewer than 1 items"). The owned validator
  agrees and rejects it. A peer briefly landed
  `🏪️store/🧫️fixtures/🧮️demo/🧬️mutations/🗑️delete-n` with `propertyNames: { enum: [] }` meaning "no
  properties allowed" and has since changed it. If that spelling is wanted deliberately, the
  draft-07-legal form is `propertyNames: false` or `maxProperties: 0` — not an empty `enum`.
- **`format` for non-string types.** The seven pinned formats are string assertions only, so a
  `format: "date"` on a number is inert, matching ajv. If a scope ever needs a numeric format
  assertion the policy constant has to grow a type dimension.
- **`ValidationControl` budget vs `pattern`.** Unchanged from WP2 §6: pattern matching, now including
  lookahead, is not metered by the node budget. Lookahead makes catastrophic backtracking easier to
  write, so if a scope ever accepts an untrusted `pattern` value `PatternMatcher::is_match` needs a
  step budget. Every `pattern` in the tree today is handcrafted repo-owned text.
- **`$ref` to a sibling's `$id` requires the sibling to be handed in.** The compile test derives the
  needed set from the module's own `$ref` strings. `structural_validator_for` instead offers every
  registered JSON Schema leaf. Once the catalog exists, `schema check` should compile each module
  against exactly its declared `dependsOn` set so an undeclared cross-scope edge is an error rather
  than an accident of what happened to be registered.
