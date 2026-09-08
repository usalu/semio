# WP4d — `🧰️framework/🔨️modules/🖱️ui/**` rows 92 + 96

Partition: `🧰️framework/🔨️modules/🖱️ui/**` only (the `framework.ui` schema module and the
`framework.ui.contract` crate). Continuation of `📓️wp4c-framework-modules.md` §4.2, §5, §8.
Rows worked: `📋️cross-partition-requests.md` **92** (disposition vocabulary decision) and **96**
(`x-semio-formats` instead of empty format leaves).

## 1. Result per row

| row | request | result |
|---|---|---|
| 92 | `retainedCommandDisposition` = runtime `InteractiveJobClassification` in kebab case, no collapse to `batch-only`; seven lanes kept | **done** — enum is now 6 members; `wp4c-retained-command-fixtures.mjs` re-cases to `batch-only-pending-rewrite` and printed the per-file counts (§4) |
| 96 | annotate `ConformanceCatalogFixture`/`ContractFixture` with `x-semio-formats: ["🔣️jsonschema","🦀️rust"]`, registration declares only those two leaves | **done** — plus the Rust half of both exports, which is what makes the `🦀️rust` claim true (§3.2); `schema check` findings for the scope went 2 → 0 |

## 2. Row 92 — the disposition vocabulary

`🧰️framework/🔨️modules/🖱️ui/🧬️schema/🔣️.json`, `definitions.retainedCommandDisposition`
(`$id https://semio.tech/schema/framework/ui/schema.json`, referenced by
`retainedCommandRouteDisposition.disposition`, `retainedCommandRouteExecutionFeature.admission`
and `.status`, and `retainedCommandCohortRoute.admission`):

```
-  migrated | batch-only | fail-closed
+  unclassified | migrated | batch-only-pending-rewrite | forbidden-from-ui | deleted | fail-closed
```

`retainedCommandLane` is untouched: the seven lanes
`artifact|config|host-only|draft|presence|transient|child` stay exactly as WP4c left them.

### 2.1 Vocabulary table — verified against the runtime, not assumed

Runtime enum read at `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs:836`
(`#[serde(rename_all = "camelCase")] #[value(rename_all = "camelCase")] pub enum
InteractiveJobClassification`), five variants, `Unclassified` is `#[default]`:

| runtime variant | wire (camelCase) | shared shape (kebab) | in the enum because |
|---|---|---|---|
| `Unclassified` | `unclassified` | `unclassified` | runtime variant (the non-executable default) |
| `Migrated` | `migrated` | `migrated` | runtime variant |
| `BatchOnlyPendingRewrite` | `batchOnlyPendingRewrite` | `batch-only-pending-rewrite` | runtime variant |
| `ForbiddenFromUi` | `forbiddenFromUi` | `forbidden-from-ui` | runtime variant |
| `Deleted` | `deleted` | `deleted` | runtime variant |
| — | — | `fail-closed` | **not a runtime variant**: the admission verdict a route carries when no classification admits it. Contract §B names it, and `🎥️shooting`'s fixture is its only instance (37 values, spelled `failClosed` before the re-casing). |

`fail-closed` is the one member of the six that the runtime enum does **not** carry; everything else
is a one-to-one mirror. The `$comment` in the module says so, so the next reader does not have to
re-derive it.

## 3. Row 96 — `framework.ui.contract` declares its two formats honestly

### 3.1 The annotation

`🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧬️schema/🔣️.json` — both `$defs` entries gained

```json
"x-semio-formats": ["🔣️jsonschema", "🦀️rust"]
```

which is exactly the scope's `formats` in the derived catalog
(`📚️library/🔣️schema-catalog.json`: `{"🦀️rust": "🦀️.rs", "🔣️jsonschema": "🔣️.json"}`), and matches
contract §A ("`x-semio-formats` lists every format the export exists in, including the normative
one"). Ids are the taxonomy `schemaFormats` keys, checked against
`📚️library/🔣️taxonomy.json`.

### 3.2 The registration, and why the Rust half had to become real

`🧬️schema/🦀️.rs` now derives the registration from one constant instead of restating a format list:

```rust
pub const DECLARED_FORMATS: [SchemaFormat; 2] = [SchemaFormat::JsonSchema, SchemaFormat::Rust];
const LEAVES: FacetLeaves = FacetLeaves { rust: include_str!("🦀️.rs"), typescript: "", graphql: "", json_schema: include_str!("🔣️.json"), proto: "" };
```

`FacetLeaves` has five fields by construction and lives in `🧰️framework/🔨️modules/🧬️schema/📇️registry`
(outside this partition), whose own docstring defines an empty body as "the scope does not provide
that format". "Declare only those two leaves" is therefore expressed as `DECLARED_FORMATS` + the
registry's own spelling of absence, and the law asserts the two agree — rather than by editing the
registry struct.

The law (`🧬️schema/🧪️tests/🔬️scope-schema-export-law-standalone/🦀️.rs`, extracted from the module by
the concurrent test-layout refactor while this pass ran, contents intact) changed from "these three
formats error" to two tests:

- `registers_and_resolves_exactly_the_declared_formats` — for every export × every
  `SchemaFormat::ALL`, `resolve_schema_export(...).is_ok() == DECLARED_FORMATS.contains(&format)`.
  The three absent formats are now covered by the same assertion as the two present ones instead of
  by a hand-written list.
- `restricted_formats_match_the_annotation` — parses `🔣️.json` and asserts each export's
  `x-semio-formats` equals `DECLARED_FORMATS.map(taxonomy_key)`, in order, and that `$defs` has
  exactly `EXPORTS.len()` entries. The annotation and the registration cannot drift.

**Deviation from the literal brief, applied because the annotation is otherwise a false claim.**
Naming `🦀️rust` in the annotation makes the export *promise* a Rust declaration; the harness rule
(`📓️wp1c-harness-rules.md` §2.1) and `📚️library/🔍️discovery/🟦️.ts:3036` both test that promise by
looking for `struct|enum|type|trait|fn <ExportId>` (or a `pub use`) in the module's `🦀️.rs`. Before
this pass the file declared neither name, and `bun 📜️script.ts schema check` reported it:

```
{"code":"export-format-missing","path":"🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧬️schema/🦀️.rs","detail":"framework.ui.contract declares export ConformanceCatalogFixture in 🔣️.json; 🦀️rust carries no declaration of it. …"}
{"code":"export-format-missing","path":"🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧬️schema/🦀️.rs","detail":"framework.ui.contract declares export ContractFixture in 🔣️.json; 🦀️rust carries no declaration of it. …"}
```

So the module now carries the Rust half of both exports, which is exactly what contract §B says a
`🧬️schema/🦀️.rs` is for ("pub types + decode/validate entry points, `include_str!` of siblings"):

| export | Rust type | data it decodes | consumer rewired |
|---|---|---|---|
| `ConformanceCatalogFixture` (+ helper `ConformanceCatalogFixtureGroup`) | moved out of the test file, where it was the same shape under a different name (`CorpusCatalog`/`CorpusGroup`) | `📚️examples/🧪️conformance/📇️catalog.json` | `🧪️tests/🔬️conformance-unit/🦀️.rs` now `use crate::schema_metadata::ConformanceCatalogFixture` |
| `ContractFixture` (+ `ContractFixtureCase`, `ContractFixtureFlags`) | new; `case.update` is the crate's own `PresenceUpdate`, not a structural clone | `🧬️contract/🧪️fixtures/👥️presence-overlay.json` | `🧪️tests/🔬️presence-unit/🦀️.rs` decodes the whole file typed instead of poking `serde_json::Value` by string key, and still byte-compares the re-serialized update with the committed wire |

No new type is invented for the sake of the rule: one was a rename-and-move of an existing test-local
struct, the other replaces stringly `Value` indexing in a test that was already asserting the shape.
After the change `schema check` reports **zero** findings for `framework.ui.contract` (§4.4).

## 4. Verification — real output

All commands run from the repo root. Cargo confined to
`CARGO_TARGET_DIR=…/scratchpad/target-w10`, `RUSTC_WRAPPER=""`, `-p semio-framework-ui-contract`.
Logs copied to `🗑️generated/wp4d-*.txt`.

### 4.1 Every framework schema module still compiles (ajv draft-07)

`bun wp4-framework-validate.mjs` — the script gained `x-semio-formats` in its ajv strict keyword
allow-list beside `x-semio-binary`/`x-semio-state`:

```
modules=116 exports=231 badDialect=0 badId=0 noExports=0 problems=0
```

(217 exports at the start of this pass; the other partitions added the rest concurrently.)

### 4.2 The 11 retained-command fixtures against the row-92 vocabulary

`bun wp4c-retained-command-fixtures.mjs`, normalization map updated to
`BatchOnlyPendingRewrite → batch-only-pending-rewrite`, `batch-only → batch-only-pending-rewrite`,
`failClosed|FailClosed → fail-closed`, plus the three variants that had no instance yet
(`Unclassified`, `ForbiddenFromUi|forbiddenFromUi`, `Deleted`). A fixture already spelling
`batch-only-pending-rewrite` needs no edit and is no longer counted.

**First run of this pass** (the per-file counts the plugins worker asked for — 137 values across 5
files, up from WP4c's 130 across 7, because `batch-only` now moves too and the two already-correct
route documents dropped out):

```
fixtures=11 committed-pass=6 normalized-pass=11

pending plugin re-casing: 137 values across 5 files
  ✏️s/🔌️plugins/🎞️animate/…/🧫️retained-command-limits/🔣️.json
    14× BatchOnlyPendingRewrite → batch-only-pending-rewrite
    2× Config → config
    2× HostOnly → host-only
    4× Migrated → migrated
  ✏️s/🔌️plugins/🎥️shooting/…/🧫️retained-command-limits/🔣️.json
    37× failClosed → fail-closed
    2× hostOnly → host-only
  ✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🧪️fixtures/🧫️retained-command-limits/🔣️.json
    25× batch-only → batch-only-pending-rewrite
    6× hostOnly → host-only
  ✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/…/🧫️retained-command-limits/🔣️.json
    3× Artifact → artifact
    4× Config → config
    11× HostOnly → host-only
    18× Migrated → migrated
  ✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/…/🧫️retained-command-limits/🔣️.json
    9× hostOnly → host-only
```

**Final run**, after the plugins partition applied the re-casing in the working tree (unstaged at the
time of writing; the fixtures now read `batch-only-pending-rewrite`, `fail-closed`, `host-only`):

```
fixtures=11 committed-pass=11 normalized-pass=11

pending plugin re-casing: 0 values across 0 files
```

Row 93's fixture half is therefore satisfied against the row-92 vocabulary; the Rust readers,
the nine owner-schema `const` narrowings and `🪐️space/📦️packages/🦀️rust/📜️script.ts` remain that
row's work and are outside this partition.

### 4.3 `cargo test -p semio-framework-ui-contract`

```
running 9 tests
test schema_metadata::scope_schema_export_law::restricted_formats_match_the_annotation ... ok
test presence::tests::presence_overlay_fixture_preserves_separate_own_flags ... ok
test schema_metadata::scope_schema_export_law::registers_and_resolves_exactly_the_declared_formats ... ok
test conformance::tests::every_ui_patch_op_variant_appears_in_a_patch_case ... ok
test conformance::tests::patch_fixtures_apply_cleanly_and_match_their_expectations ... ok
test conformance::tests::rejection_fixtures_are_rejected_with_the_named_violation_and_leave_state_unchanged ... ok
test conformance::tests::snapshot_only_fixtures_are_valid_and_match_their_expectations ... ok
test conformance::tests::every_component_variant_appears_in_the_corpus ... ok
test conformance::tests::corpus_has_no_orphan_fixtures ... ok
test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 156 filtered out; finished in 0.67s
```

Filtered to the tests this pass touches. The crate's **whole** suite cannot be run to completion
right now and the reason is not this change: `cargo test -p semio-framework-ui-contract` aborts with
`SIGABRT` in `action::retirement::*` (`"UI value retirement arena is poisoned"`, then a panic in
`UiValueRetirement::drop` → `panic in a destructor during cleanup`, which is non-unwinding and kills
the whole test binary before any summary prints). Five `♻️retirement`/`builder` laws were already
`FAILED` before the abort. That subtree is the END-TO-END-TESTING-REFACTOR peer's live work area
(`♻️retirement` mtimes minutes old throughout this pass) and is untouched by anything here.

Warnings: the lib build is clean (the `DECLARED_FORMATS is never used` warning from the first
iteration is gone — the constant is `pub`, which is also how a boot call site reads the declared set).
The 7 remaining `unnecessary qualification` warnings are all in peer-owned test files
(`🎟️resident/🌳️root/🧪️tests/🦀️.rs`, `📃️document/🎟️assembly/🧪️tests/🦀️.rs`), none in a file this pass
edited.

### 4.4 `bun 📜️script.ts schema check` (repo-wide, for the row-96 claim)

Before the Rust half landed: 2 rows naming `framework.ui.contract`
(`export-format-missing`, quoted in §3.2), repo totals `modules=3204 scopes=3021 findings=10331`,
`export-format-missing=6583`.

After: **0** rows naming `framework.ui.contract` and **0** rows under `🖱️ui/🧬️schema`; repo totals
`modules=3206 scopes=3023 findings=10315`, `export-format-missing=6567`. (The repo totals move
because every partition is editing concurrently; the per-scope count is the load-bearing number.)

### 4.5 The ajv strict consumer of the annotated module

`conformanceCorpusSelfTests()` compiles `framework.ui.contract`'s `ConformanceCatalogFixture` with
`new Ajv({ strict: true })`, which rejects an unknown keyword, so it now registers
`x-semio-formats` (`metaSchema: {type:"array", minItems:1, items:{type:"string"}}`):

```
conformance-corpus-catalog cases=62
```

The helper moved from `📦️packages/🦀️rust/📜️script.ts` to
`🧪️tests/🔬️conformance-corpus/🟦️.ts` mid-pass (the same peer refactor); the edit travelled with it and
was re-run from the new location.

## 5. Files changed

Production (all inside `🧰️framework/🔨️modules/🖱️ui/**`):

- `🖱️ui/🧬️schema/🔣️.json` — row 92: `retainedCommandDisposition` enum + `$comment`.
- `🖱️ui/🧬️contract/🧬️schema/🔣️.json` — row 96: `x-semio-formats` on both `$defs` exports.
- `🖱️ui/🧬️contract/🧬️schema/🦀️.rs` — `pub const DECLARED_FORMATS`; `LEAVES` docstring; the rewritten
  law; new `🔖️Exports` region with `ConformanceCatalogFixture`, `ConformanceCatalogFixtureGroup`,
  `ContractFixture`, `ContractFixtureCase`, `ContractFixtureFlags`; module header.
- `🖱️ui/🧬️contract/🧪️tests/🔬️conformance-unit/🦀️.rs` — `CorpusCatalog`/`CorpusGroup` deleted, decodes
  through the export.
- `🖱️ui/🧬️contract/🧪️tests/🔬️presence-unit/🦀️.rs` — decodes through `ContractFixture`.
- `🖱️ui/🧬️contract/📦️packages/🦀️rust/📜️script.ts` — ajv `x-semio-formats` keyword (the peer's refactor
  has since moved this function to `🖱️ui/🧬️contract/🧪️tests/🔬️conformance-corpus/🟦️.ts`, keyword
  included).
- `🖱️ui/🧬️contract/🪞️copy/🦀️.rs` and `🖱️ui/🧬️contract/♻️retirement/🌳️typed/📃️document.rs` — **repairs,
  not part of either row**: two `#[path]` mounts left dangling by the concurrent test-layout move
  (`🔬️bytes.rs` → `🧪️tests/🔬️bytes/🦀️.rs`, `🔬️document.rs` → `🧪️tests/🔬️document/🦀️.rs`; both moves
  already committed, both mounts still pointing at the deleted names). The crate did not compile at
  all until they were fixed, so no verification of this pass was possible without them. Rewritten to
  the peer's own new convention, identical to the `📋️list/🦀️.rs` → `🧪️tests/🔬️counter/🦀️.rs` mount they
  had already converted. Nothing was reverted.

Ticket folder (inputs, kept):

- `wp4c-retained-command-fixtures.mjs` — normalization map + header updated for row 92.
- `wp4-framework-validate.mjs` — `x-semio-formats` added to the ajv strict keyword allow-list.

Ticket folder (generated, delete at ticket close): `🗑️generated/wp4d-framework-validate.txt`,
`🗑️generated/wp4d-retained-command-fixtures.txt`, `🗑️generated/wp4d-ui-contract-law.txt`,
`🗑️generated/wp4d-schema-check.txt`.

## 6. Cross-partition requests

### 6.1 W2c tooling — regenerate the derived catalog

`📚️library/🔣️schema-catalog.json` is stale for `framework.ui.contract`: both source hashes changed
(`🔣️.json` was `389a616a…`, `🦀️.rs` was `fe781c86…`) and `schema check` reports `catalog-stale=1`.
`framework.ui`'s row is stale too (its `🔣️.json` hash moved with the row-92 enum). WP4c §5.5 already
asked for a regeneration; this adds two more source hashes to it. Nothing else about either row
changes: export sets are unchanged by this pass.

### 6.2 Whoever owns `💻️os/🧑‍💻dev/📤️distribution` — regenerate the distribution manifest

`🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📤️distribution/🧾️manifest.json` pins `bytes` + `sha256`
for `🖱️ui/🧬️contract/🧬️schema/🔣️.json` (4239 / `389a616a…`) and `…/🦀️.rs` (48407 / `a28e6b43…`).
Both are now wrong. It is a generated file outside this partition; it needs whatever command emits it.

### 6.3 W6c plugins — row 93's non-fixture half

The 11 fixtures now pass as committed (§4.2), so the remaining row-93 work is the nine owner schemas'
`const` narrowings, the Rust lane/disposition string readers, and
`🪐️space/📦️packages/🦀️rust/📜️script.ts:135,431` — all as WP4c §5.1 lists them, with `batch-only`
reading `batch-only-pending-rewrite` per row 92. Note `🪐️space/⚙️engine/🪐️space`'s fixture used the
shorthand `batch-only` for 25 route rows; those are `BatchOnlyPendingRewrite` in the runtime
(`🪐️space/📦️packages/🦀️rust/📜️script.ts:444` asserts on that variant by name), so the expansion is
semantics-preserving, not a widening.

### 6.4 Row 95 still open, unchanged

`semio_framework_ui_contract::schema_metadata::register_scope_exports()` still has no production
call site. `DECLARED_FORMATS` is now `pub` so that call site (or a conformance checker) can read the
scope's declared formats rather than restating them.

## 7. Open questions

1. **`ContractFixture` and `framework.ui`'s `PresenceOverlayFixture` describe the same file.**
   `🧬️contract/🧪️fixtures/👥️presence-overlay.json` is validated by `PresenceOverlayFixture`
   (`framework.ui`, used by `💻️os/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts`) and typed by
   `ContractFixture` (`framework.ui.contract`, used by the Rust presence law). Two scopes owning one
   fixture is one owner too many under §A. The Rust side wants the type in the crate; the TypeScript
   consumer wants the JSON export in `framework.ui`. Coordinator call: either `framework.ui.contract`
   owns it and `framework.ui` drops `PresenceOverlayFixture` (the TS test then `$ref`s across), or the
   reverse and `ContractFixture` is deleted with its Rust half. Not decided here — both rows I was
   given are silent on it, and either direction edits a partition I do not own.
2. **`ContractFixtureCase.update` is `crate::PresenceUpdate`, which is wider than the JSON.** The JSON
   `update` is `additionalProperties:false` over `surface|nodeKey|own|ttlMs`; `PresenceUpdate` also
   carries `peers` (defaulted, skipped when empty) and `own.color`. Reusing the transport type is the
   honest choice — a structural clone would be a second source of truth — but it means the Rust half
   accepts documents the JSON half rejects. If the format-coverage rule ever grows an equivalence
   check, this is the first place it will fire.
3. **The `♻️retirement` abort (§4.3) blocks any full-suite claim for this crate.** Five laws fail and
   the binary aborts in a destructor, so `cargo test -p semio-framework-ui-contract` cannot report a
   summary at all. Whoever owns that subtree needs to land it; until then every worker touching this
   crate can only report filtered runs.
4. **Two `#[path]` mounts were repaired here that belong to another ticket's refactor** (§5). If that
   peer's tool re-runs and rewrites them differently, the crate breaks again; the convention it should
   converge on is the one already used by `📋️list/🦀️.rs` (`#[path = "🧪️tests/<case>/🦀️.rs"]`).
