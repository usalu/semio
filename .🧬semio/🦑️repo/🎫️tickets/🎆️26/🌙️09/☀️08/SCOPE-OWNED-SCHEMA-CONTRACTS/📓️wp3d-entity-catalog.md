# WP3d — the entity-kind catalog becomes a `framework.schema` contract (row 122, framework side)

Partition: `🧰️framework/🔨️modules/🧬️schema/**`. Continues `📓️wp3c-framework-schema.md` (which restored the
data to `🔣️entity-kinds.json` and raised R-1) and implements the coordinator decision in
`📋️cross-partition-requests.md` **row 122**: the catalog is owned by `framework.schema`, the vscode
`TechnologyCatalog*` scope is retired, and the vscode client becomes a consumer of the generated projection.

Repo MCP was down for the whole session: no MCP tool was called, no ticket opened/closed/reopened,
`🗑️generated/` was not deleted, no git-modifying command was used, no worktree was created. Cargo ran only
with `-p semio-framework-schema -p semio-framework-schema-registry`, `CARGO_TARGET_DIR=<scratchpad>/target-w3`,
`RUSTC_WRAPPER=""`.

## 1. Files changed

| File | Change |
|---|---|
| `🧬️schema/🔣️.json` | root `$ref` → `oneOf` of the two serialized documents; **new** `$defs.EntityKindCatalog`, `$defs.EntityKind` |
| `🧬️schema/🟦️.ts` | **new** region `🔖️EntityKindCatalog`: `EntityKind`, `EntityKindCatalog`, `parseEntityKind`, `parseEntityKindCatalog`, `entityKindIndexByEmoji` |
| `🧬️schema/⚛️component.rs` | `FRAMEWORK_SCHEMA_ENTITY_CATALOG_LEAVES`, `ENTITY_KIND_CATALOG_JSON`, `FRAMEWORK_SCHEMA_EXPORTS` 8 → **10** |
| `🧬️schema/📦️packages/🦀️rust/📜️script.ts` | rewritten emitter: provenance header (generator id + source sha256), source parsed through the owned parser, first-wins emoji index in all three projections, `EntityKindSpec` → `EntityKind` |
| `🧬️schema/🧪️tests/🏷️entity-kinds/🟦️.ts` | **new** — ajv oracle + owned-parser agreement + projection provenance |
| `🧬️schema/🧪️tests/🔬️component-unit/🦀️.rs` | **new** region `🔖️EntityKindCatalog` — three tests (owned validator, first-wins, negative corpus) |
| `🧬️schema/🧪️tests/📤️schema-export-entries/🟦️.ts` | root assertion `$ref` → `oneOf` |
| `🧬️schema/🧫️fixtures/📤️schema-export-entries-dump.json` | regenerated: 27 → **33** entries (the two new exports × 3 formats) |
| `🧬️schema/🤖️generated.rs`, `🧬️schema/🤖️generated/🟦️entity-kinds.ts`, `🦑️repo/…/⌨️cli/🐹️entity_kinds.g.go` | regenerated (git-ignored `outputRoots`) |

A concurrent repo sweep renamed `🧬️schema/🧪️vitest.config.ts` → `🧬️schema/vitest.config.ts` (and added
`passWithNoTests: false`) mid-session; it is not this partition's edit, was not reverted, and the suite was
re-run green against the new path (§4.4). R-3's snippet below uses the new path.

`🧬️schema/🔣️entity-kinds.json` is **untouched** — still the byte-identical HEAD data, 58 entries,
sha256 `f237cdd640bd1ac2b4546726f78f33c90aaf0c2ed11e1f6115294c8e9c3be242`.

## 2. The two new exports

### 2.1 `EntityKindCatalog`

`type: array`, `minItems: 1`, `uniqueItems: true`, `items: {$ref: #/$defs/EntityKind}`.

The `description` states the three things the shape cannot: declaration order is contract; `id` is unique
catalog-wide while `emoji` is **not**; every projection's emoji index keeps the **FIRST** entry of a repeated
emoji. `uniqueItems` only forbids two byte-identical entries — uniqueness of `id` alone and of `label` alone is
not expressible in draft-07, so it is asserted against the data by `parseEntityKindCatalog` and by the Rust
test, never smuggled into the schema.

### 2.2 `EntityKind`

Closed object, all five members required. Bounds and patterns are the ones the three consumers actually rely
on (the VS Code extension, the CLI's Go catalog, the framework Rust catalog) — the same table the retired
`repo.client.vscode` `TechnologyCatalogEntry` carried, so nothing was invented or relaxed here:

| Member | Type | Bounds | Widest value in the data |
|---|---|---|---|
| `id` | string | 3–32, `^[a-z][a-z0-9]*(?:-[a-z0-9]+)*$` | 3–25 chars, 58/58 unique |
| `emoji` | string | 1–8, `^\S+$` | 2–5 code points, **56** distinct (2 shared) |
| `iconId` | string | 3–32, kebab | 3–14 chars, 56 distinct (`pen-tool`, `check-circle-2` serve two kinds each) |
| `label` | string | 2–32, `^\S(?:.*\S)?$` | 2–19 chars, 58/58 unique |
| `filterable` | boolean | — | `false` for exactly the six tree roots `codebase`, `technologies`, `bundles`, `folders`, `files`, `definitions` |

### 2.3 Root shape — why it became a `oneOf`

`structural_validator_for(scope, export)` resolves the export only to pick the **leaf file** and then compiles
that whole document, so validation always runs against the document root. With two instance documents on disk
(`🧫️fixtures/📤️schema-export-entries-dump.json` and `🔣️entity-kinds.json`) a single `$ref` root can gate only
one of them. Contract §B allows exactly this case ("root may be a `oneOf` of exports"), so the root is now

```json
"oneOf": [{ "$ref": "#/$defs/SchemaExportEntries" }, { "$ref": "#/$defs/EntityKindCatalog" }]
```

The two branches are disjoint by JSON type (object vs array), so exactly one can ever match and root-level
validation stays as precise as per-export validation for both documents. The ajv spec additionally validates
each export **precisely**, through `ajv.getSchema("<$id>#/$defs/<Export>")`.

### 2.4 Registration

`FRAMEWORK_SCHEMA_EXPORTS` grows to ten, with a third leaf set:

```rust
const FRAMEWORK_SCHEMA_ENTITY_CATALOG_LEAVES: FacetLeaves = FacetLeaves {
    rust: include_str!("🤖️generated.rs"),      // the generated Rust projection declares `EntityKind`
    typescript: include_str!("🟦️.ts"),          // the twin declares the types and `parse<Export>`
    graphql: "", json_schema: include_str!("🔣️.json"), proto: "",
};
```

`framework_schema_exports_match_the_modules_json_schema_defs_and_resolve` (pre-existing) enforces set equality
between `$defs` keys and registered exports, so the two additions had to land on both sides at once; it also
asserts all three declared formats resolve non-empty and that graphql/proto answer `FormatAbsent`.

`ENTITY_KIND_CATALOG_JSON` (`include_str!("🔣️entity-kinds.json")`) is the new public constant naming the single
instance document.

## 3. The emitter

`🧬️schema/📦️packages/🦀️rust/📜️script.ts`, generator id `schema-entity-catalog` (unchanged — it is the
`preview-generated` `contractId` and the taxonomy `generatorContracts` key).

**Provenance.** Every projection now opens with the same two lines, comment syntax aside:

```
🤖️ @generated by `schema-entity-catalog` from `🧰️framework/🔨️modules/🧬️schema/🔣️entity-kinds.json` (sha256 f237cdd640bd1ac2b4546726f78f33c90aaf0c2ed11e1f6115294c8e9c3be242) — do not edit.
Refresh with `bun nx run @semio-tech/framework-schema:generate`.
```

The digest is the sha256 of the **source bytes**, so it is stable across runs and `check`'s byte comparison
still holds; a hand-edit of the data without a regenerate is now visible in the projection itself, not only in
the diff.

**The source is parsed, not cast.** `readEntityCatalog()` runs the data through the owned
`parseEntityKindCatalog` from `🟦️.ts`. A catalog violating `#/$defs/EntityKindCatalog` now fails the generator
instead of reaching three consumers. (The previous `JSON.parse(...) as EntityKindSpec[]` cast is gone.)

**First-wins everywhere.** This is the substantive behaviour fix. `🤖️generated.rs`'s `entity_kind_by_emoji`
used `.iter().rev().find(…)` — **last**-wins — while its own docstring claimed to mirror the TS `Map`; the TS
projection built `new Map(ENTITY_KINDS.map(…))`, which is also last-wins; and the vscode literal it was all
supposed to agree with was first-wins. All three projections now index first-wins and say so:

| Projection | Symbol |
|---|---|
| `🤖️generated/🟦️entity-kinds.ts` | `ENTITY_KIND_BY_EMOJI: ReadonlyMap<string, EntityKind>`, `entityKindByEmoji(emoji)` |
| `🤖️generated.rs` | `entity_kind_by_emoji(emoji) -> Option<&'static EntityKind>` (`.iter().find(…)`) |
| `⌨️cli/🐹️entity_kinds.g.go` | `EntityKindByEmoji(emoji) (EntityKind, bool)` over a package-level first-wins map |

Concretely: 📝️ → `draft` (not `todo`), 🌱️ → `technology-mono` (not `interaction-started`). Exactly two of 58
kinds are shadowed; that number is pinned from both the Rust and the TypeScript side.

**Type name.** `EntityKindSpec` → `EntityKind` in all three projections, matching the export id. The TS
projection no longer restates the type — it imports it from the twin and re-exports it
(`import type { EntityKind, EntityKindCatalog } from "../🟦️";` + `export type { … };`), so the vscode client
can import value *and* type from one generated file. The import is deliberately **extension-less**: the
consuming tsconfig uses `moduleResolution: "Bundler"` without `allowImportingTsExtensions`, where a `.ts`
specifier is an error.

**Targets** are unchanged in identity (row 70 path already correct):
`🤖️generated/🟦️entity-kinds.ts`, `⌨️cli/🐹️entity_kinds.g.go`, `🤖️generated.rs`.

## 4. Verification — real output

### 4.1 `generate` / `check` / `preview-generated`

```
$ bun ./📜️script.ts generate
entity catalog refreshed (58 entity kinds, 2 emoji-shadowed, sha256 f237cdd640bd1ac2b4546726f78f33c90aaf0c2ed11e1f6115294c8e9c3be242) -> 🤖️generated/🟦️entity-kinds.ts, ⌨️cli/🐹️entity_kinds.g.go, 🤖️generated.rs

$ bun ./📜️script.ts check
entity catalog is fresh (58 entity kinds, sha256 f237cdd640bd1ac2b4546726f78f33c90aaf0c2ed11e1f6115294c8e9c3be242).

$ bun ./📜️script.ts preview-generated
schema-entity-catalog 1 staleRemovals: []
  🧰️framework/🔨️modules/🧬️schema/🤖️generated.rs                                    7565
  🧰️framework/🔨️modules/🧬️schema/🤖️generated/🟦️entity-kinds.ts                     7131
  🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🐹️entity_kinds.g.go       7051
```

### 4.2 `cargo test -p semio-framework-schema -p semio-framework-schema-registry`

```
running 28 tests
test component::tests::entity_kind_catalog_data_validates_through_the_owned_validator_and_matches_the_rust_projection ... ok
test component::tests::entity_kind_emoji_index_is_first_wins_in_the_rust_projection ... ok
test component::tests::entity_kind_catalog_rejects_every_declared_violation ... ok
test component::tests::framework_schema_exports_match_the_modules_json_schema_defs_and_resolve ... ok
test component::tests::framework_schema_facet_validates_a_real_runtime_entries_dump ... ok
… (23 pre-existing tests) …
test result: ok. 28 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

     Running ../../🧪️tests/📤️schema-export-entries/🦀️.rs
test schema_export_catalog_entries_dump_is_sorted_and_matches_the_declared_contract ... ok
test result: ok. 1 passed; 0 failed; …

     Running ../../🧪️tests/🧩️module-compile/🦀️.rs
test every_draft07_schema_module_compiles_through_the_owned_validator ... ok
test result: ok. 1 passed; 0 failed; … finished in 35.22s

     Running unittests 🦀️.rs (semio_framework_schema_registry)
test result: ok. 6 passed; 0 failed; …

     Running ../../🧪️tests/📇️catalog/🦀️.rs
test a_dependency_restricted_scope_registers_resolves_and_dumps_through_the_registry_crate_alone ... ok
test result: ok. 1 passed; 0 failed; …
```

37 tests, 0 failures. The first run **failed** on
`schema_export_catalog_entries_dump_is_sorted_and_matches_the_declared_contract` exactly as designed — the
committed dump was missing the six new `(EntityKind|EntityKindCatalog) × (jsonschema|rust|typescript)` entries.
Regenerated with `SEMIO_SCHEMA_EXPORT_ENTRIES_OUT=<fixture> cargo test … --test schema-export-entries`, then
green.

### 4.3 `cargo clippy -p semio-framework-schema -p semio-framework-schema-registry --all-targets`

```
warning: `semio-framework-replication` (lib) generated 2 warnings
warning: `semio-framework-os-kernel` (lib) generated 1 warning
    Finished `dev` profile [unoptimized] target(s) in 1m 52s
```

Zero warnings attributed to either crate; the three are the pre-existing `large_enum_variant` findings in peer
crates outside this partition (identical to `📓️wp3c` §6.2).

### 4.4 ajv oracle — `bunx vitest run --config 🧰️framework/🔨️modules/🧬️schema/vitest.config.ts`

```
 ✓ 🧪️tests/🏷️entity-kinds/🟦️.ts > … > declares both serialized documents at its root
 ✓ 🧪️tests/🏷️entity-kinds/🟦️.ts > … > accepts 🔣️entity-kinds.json in ajv and in the owned parser
 ✓ 🧪️tests/🏷️entity-kinds/🟦️.ts > … > rejects the same malformed entries in ajv and in the owned parser
 ✓ 🧪️tests/🏷️entity-kinds/🟦️.ts > … > rejects an empty catalog and a duplicated entry in both
 ✓ 🧪️tests/🏷️entity-kinds/🟦️.ts > … > indexes emoji FIRST-WINS, shadowing exactly two kinds
 ✓ 🧪️tests/🏷️entity-kinds/🟦️.ts > … > stamps every projection with the same generator and source sha256
 ✓ 🧪️tests/📤️schema-export-entries/🟦️.ts > … (4 cases)
 ✓ 🧪️tests/✅️draft07-oracle/🟦️.ts > … (17 cases)

 Test Files  3 passed (3)
      Tests  27 passed (27)
```

The eleven-instance negative corpus (undeclared property, non-kebab `id`, too-short `id`, emoji carrying
whitespace, empty emoji, non-kebab `iconId`, untrimmed `label`, one-character `label`, non-boolean
`filterable`, missing member, non-object) is asserted against **both** ajv and the owned `parseEntityKind`, so
the hand-written parser is pinned to the third-party reading of the same document.

### 4.5 Root gate — `bun ./📜️script.ts schema test`

```
[schema test] harness exit=1, draft-07 oracle exit=0
```

The **oracle half is green**. The harness half reports 4667 repo-wide invariant findings
(`2477 × schema-export-incomplete`, `1694 × schema-export-parser-missing`, `156 × schema-fixture-defines-schema`,
…) from partitions still in flight. **Zero** of them name this module: `grep -c "🔨️modules/🧬️schema/"` → 0,
`grep -c "framework\.schema"` → 0. Full log `🗑️generated/wp3d-schema-test.txt`.

Note the harness does **not** yet run the new spec: root `📜️script.ts:17503 oracleArguments()` still writes a
throwaway vitest config pinned to `SCHEMA_DRAFT07_ORACLE_SPEC` alone (`📓️wp3c` R-3 has not landed) — see R-3
below.

### 4.6 Type-check and Go build

```
$ bunx tsc --noEmit --skipLibCheck --strict --resolveJsonModule --target ES2022 --module ESNext \
    --moduleResolution Bundler --types node \
    🧬️schema/🟦️.ts 🧬️schema/🤖️generated/🟦️entity-kinds.ts \
    🧬️schema/🧪️tests/🏷️entity-kinds/🟦️.ts 🧬️schema/🧪️tests/📤️schema-export-entries/🟦️.ts
exit=0

$ gofmt -l 🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🐹️entity_kinds.g.go
(no output — gofmt-clean)

$ (cd …/⌨️cli && go build ./)
exit=0
```

The Go build is the proof that the renamed `EntityKind` type and the new `EntityKindByEmoji` / package-level
index do not collide with the package's pre-existing `EntityKinds` var or `AllEntityEmojis()`.

## 5. Cross-partition requests

### R-1 — **W9f repo client**: the exact vscode change

`🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/🟦️.ts`

Delete both imports (`:20`, `:21`):

```ts
import technologyCatalogDocument from "../../🗂️technologies.json";
import { parseTechnologyCatalog, type TechnologyCatalog } from "../../🧬️schema/🟦️";
```

and add (eight levels up, exactly as this file's `tsconfig.json` `paths` already spells the framework root;
**extension-less** — `moduleResolution: "Bundler"` without `allowImportingTsExtensions` rejects a `.ts`
specifier):

```ts
import { ENTITY_KINDS, ENTITY_KIND_BY_EMOJI } from "../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🤖️generated/🟦️entity-kinds";
```

Then replace the whole `TECHNOLOGY_CATALOG` + `ENTITY_EMOJIS` block (`:1262-1280`) with:

```ts
/**
 * Complete set of entity-identifying emojis that appear as kind prefixes in entity IDs.
 * Projected from the framework-owned entity-kind catalog; the generated index is already FIRST-WINS,
 * so 🌱️ resolves to technology-mono and 📝️ to draft. Regex patterns are derived from it.
 **/
export const ENTITY_EMOJIS: ReadonlyMap<string, string> = new Map([...ENTITY_KIND_BY_EMOJI].map(([emoji, kind]) => [emoji, kind.id]));
```

`TECHNOLOGY_CATALOG` has exactly three references, all inside that block (grep over the whole `🧩️vscode`
tree), so it is deleted, not aliased. Where the extension wants the ordered catalog it uses `ENTITY_KINDS`
directly. `ENTITY_EMOJIS` keeps its `emoji → id` shape and its first-wins content, so
`buildEntityEmojiPattern()`, `ENTITY_ID_REGEX` and `🧪️tests/🧩️extension/🟦️.ts` (which only asserts
`ENTITY_EMOJIS.has(…)`) need no edit.

Also delete, in the same change:

- `🧩️vscode/🗂️technologies.json`
- `🧩️vscode/🧬️schema/` (both `🔣️.json` and `🟦️.ts` — the `repo.client.vscode` scope ceases to exist)
- `🧩️vscode/🧪️tests/🔬️schema/🟦️.ts` (its only subject is the deleted scope)
- the two now-dangling entries of `📦️packages/🟦️typescript/tsconfig.json`'s `include`
  (`"../../🧬️schema/🟦️.ts"`, `"../../🧪️tests/🔬️schema/🟦️.ts"`)

`resolveJsonModule` may stay; nothing else in that package imports JSON.

### R-2 — **W2w library / taxonomy**: `generatorContracts.schema-entity-catalog.inputPatterns`

Current value (`📚️library/🔣️taxonomy.json`) still points at the retired slot and is missing the parser the
emitter now depends on. Replace `inputPatterns` with, exactly:

```json
"inputPatterns": [
  "🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust/📋️project.json",
  "🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust/📜️script.ts",
  "🧰️framework/🔨️modules/🧬️schema/🟦️.ts",
  "🧰️framework/🔨️modules/🧬️schema/🔣️entity-kinds.json"
]
```

The first two lines are unchanged (the module root carries no `📋️project.json` — verified). The load-bearing
edits are `🔣️.json` → `🔣️entity-kinds.json` and the **new** `🟦️.ts` entry, because `readEntityCatalog()` now
parses through `parseEntityKindCatalog` exported from it, so a change there can change generator behaviour.
`outputRoots` is already correct — all three paths, `⌨️cli/🐹️entity_kinds.g.go` included, verified against
`preview-generated` (§4.1). This supersedes `📓️wp3c` R-2, whose `outputRoots` half has since landed.

### R-3 — **root `📜️script.ts` worker**: `schema test` still cannot see this module's specs (re-raise of `📓️wp3c` R-3)

`:17324` `SCHEMA_DRAFT07_ORACLE_SPEC` and `:17503 oracleArguments()` are unchanged. With the tracked module
config now carrying **three** spec files, the throwaway config runs one of them. The replacement is unchanged
from `📓️wp3c`:

```ts
private oracleArguments(): string[] {
  return ["vitest", "run", "--config", join(this.root, "🧰️framework/🔨️modules/🧬️schema/vitest.config.ts")];
}
```

plus deleting `SCHEMA_DRAFT07_ORACLE_SPEC` and the now-unused `tmpdir`/`writeFileSync` there. The config's
`include` is `🧪️tests/**/🟦️.ts`, so `📤️schema-export-entries` and `🏷️entity-kinds` are picked up with no
further edit.

### R-4 — **W9f repo client / CLI owner**: the Go catalog is generated but unused

`⌨️cli/🧩️component.go:1014 AllEntityEmojis()` still `add(...)`s ~58 hand-written `Emoji*` constants instead of
ranging over the generated `EntityKindCatalog`. The generated file now also offers `EntityKindByEmoji`. That is
the CLI owner's call, not this partition's, but while it stands the Go projection is a dead output and the
CLI's emoji set can drift from the catalog silently. (Recorded as an observation; no schema finding depends on
it.)

### R-5 — coordinator: `📓️wp4c-repo-product.md` §1.1 / §4 are superseded

Row 122 retires `repo.client.vscode`'s `TechnologyCatalog`/`TechnologyCatalogEntry`. The wp4c report should be
marked superseded on that point so the scope table and the derived catalog do not keep listing a scope that no
longer exists after R-1 lands. The vscode report's own analysis is preserved here: its `ENTITY_EMOJIS`
first-wins finding is the reason the three projections were corrected (§3), and its bounds table is the
`EntityKind` bounds table (§2.2).

## 6. Open questions

- **`framework.schema` now provides two unrelated contracts.** The resolution contract and the entity-kind
  catalog share one scope because they share one module directory. They are genuinely unrelated subjects, and
  the root `oneOf` is the visible cost. The alternative — a nested `🏷️entity-kinds/🧬️schema/` module with its
  own scope id `framework.schema.entity-kinds` — is available under contract §A (nested module directories are
  eligible) and would restore a single-`$ref` root to each. It was **not** taken here because row 122 names the
  exports as `framework.schema` exports; if the coordinator prefers the split, it is a mechanical move of two
  `$defs`, two TS regions, one leaf constant and the emitter's output paths.
- **Where does the TS type live for a generated projection?** Settled here as: the twin `🟦️.ts` declares
  `EntityKind`/`EntityKindCatalog` and the parsers (it is the registered `typescript` leaf), the generated file
  imports and re-exports them. The Rust side is the mirror image — `🤖️generated.rs` declares the struct and
  **is** the registered `rust` leaf, because no hand-written Rust twin exists. That asymmetry is honest but
  worth a rule in the contract if other generated projections follow.
- **`parse<Export>` is still missing for the other eight `framework.schema` exports.** `🟦️.ts` declares their
  types but no parsers, which contract §A's TypeScript presence rule requires. Pre-existing (it predates this
  packet); the two new exports do satisfy it. Whoever closes the `schema-export-parser-missing` class
  (1694 findings repo-wide, §4.5) will reach this module too.
- **First-wins is now stated in four places** (the schema `description`, and one comment per projection) and
  pinned by two tests. If a fifth consumer ever builds its own index, nothing structurally stops it from being
  last-wins — the only real guard would be for every consumer to use the generated `entityKindByEmoji` /
  `EntityKindByEmoji` rather than re-deriving.
