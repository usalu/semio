# Wave 8 — full gate

## Headline

The artifact-schema policy family reports **0 breaches over all 54 artifacts**, and that number is
trustworthy for the first time: when W8 started the scanner was structurally blind and the policy CLI
was mute. Both defects are fixed, and the 127 real breaches the repaired scanner then surfaced are
resolved.

## Defects found and fixed in the gate itself

### 1. Field/type extractors selected the schema type positionally

`policyExtractRustSchemaFields`, `…Typescript…`, `…Graphql…` and `…Protobuf…` each took the *first*
`pub struct` / `export interface` / `type` / `message` in the leaf. Helper types may legally precede
the facet type, so for `cad` and `curate` the scanners compared the wrong body against the normative
JSON Schema and emitted 127 bogus-looking breaches (117 field-parity, 10 type-name-parity).

Fixed at the root: a new `policyFindSchemaDeclaration` selects the declaration **by name**, and
`policyLoadSchemaFacetLeaves` derives the expected name from the facet path itself via
`policyExpectedSchemaTypeNameForFacetPath`. Name-based selection is unambiguous because
`XArtifact`/`XSnapshot`/`XDiff` are already pinned by the type-name-parity rule; the old positional
convention was an invisible coupling. When the expected type is absent the first declaration is still
returned, so type-name-parity keeps reporting the mismatch.

Verified: 127 → 1 breach, and a direct probe confirms `policyExtractRustSchemaFields(src, "CadArtifact")`
skips a preceding `Helper` struct while the no-argument call still returns `Helper`.

### 2. `bun ./📜️script.ts policy` exited 1 while printing nothing

`runPolicyExit` filtered for high-priority breaches and called `process.exit(1)` without ever printing
them — only leftover `[DEBUG]` trace lines reached the terminal. Every W5 fan-out agent "verified" its
plugin by grepping that empty output.

`runPolicyExit` now prints a per-rule tally plus one line per high-priority breach through the new
`formatBreachReport`, and names the cache path holding the full set. The seven leftover `[DEBUG]` lines
in `runPolicyScript` are removed.

### 3. `validateTaxonomy` never gained the W1 checks

`🔣️taxonomy.json` carried `snapshotChildDirs`, `diffChildDirs`, `schemaFormats` and
`artifactSchemaSpecFilenames`, and `🧪️index.test.ts` asserted that the validator rejects drift in each —
but none of those checks existed, so six tests failed. The validator also rejected the plan-mandated
nested `artifactSpecFilenames` key `📸️snapshot/🎒️pack`, because it required every key to be a member of
`artifactComponentDirs`.

Added a `SchemaFacetContract` region covering all four keys (non-empty child dirs, leaf/extension
agreement, `fieldCasing` ∈ {snake, camel}, schema spec filenames equal to the normative jsonschema
leaf, rejection of a bare `🎒️pack` as facet or key) plus `artifactFacetPathIsDeclared`, which walks a
`/`-joined facet path through the declared child-dir tables. The `Taxonomy` type gained the four
missing fields. Result: 13/13 `validateTaxonomy` tests pass and the shipped vocabulary is clean.

### 4. One genuine leaf disagreement, and a real format gap

After the extractor fix the single remaining breach was cad's diff proto: `references_by_model_definition_id`
is an optional map everywhere, but proto3 **rejects an `optional` label on a map field**, so presence
is inexpressible there. The leaf is correct; the rule was wrong. Field parity now exempts optionality
for protobuf map fields only, cardinality still compared, with the reason recorded in the rule's
docstring.

## Legacy `Projection` sweep

- **Build break the W6 sweep missed**: `🧰️framework/🔨️modules/🗺️surface/🎲️board-2d/🦀️component.rs` still
  imported `Puzzle2dProjection`. `board-2d` is a *default* feature of `semio-framework-surface`, so
  `cargo check -p semio-framework-surface` failed with E0432. Retargeted to `Puzzle2dSnapshot`
  (binding and docstring too); the crate is now green.
- **Pack relocation the sweep missed**: the kernel's DSL fixture sweep still `include_str!`d six
  pre-move `🗿️artifacts/<a>/🎒️pack/📡️component.protocol.semio` paths, which made
  `semio-framework-os-kernel`'s lib tests unbuildable. All six relocated under `📸️snapshot/`.
- 13 docstrings across puzzle and space named `DocumentApp::Projection`, an associated type that no
  longer exists; renamed to `DocumentApp::Snapshot`. One process docstring named `Box<CadProjection>`;
  the real binding is `Box<CadSnapshot>`.
- Preserved deliberately: `🛢️db/📽️projection` read-model, camera/world projection (`SetProjection`,
  `CadProjectionDsl`, `isOrthographicCamera`), GIS reprojection, and historical mentions of the
  dissolved `OsProjection`.

## Kernel lib tests: from unbuildable to running

Fixing the fixture-sweep paths exposed 26 further compile errors that had been masked. All were stale
test-module wiring, now repaired:

- `📡️spr/🧾️wire` tests lacked `DictBuilder`, `DictReader`, `DocumentVersion`, `HybridLogicalTimestamp`,
  `MutationId`, `RecordHasher`; and reached `scalar` via `super::super::` instead of
  `crate::os_spr::scalar::scalar`.
- the `DIM_*` dimension constants were private, so the lexer test could not name `DIM_ANGLE`; the whole
  family is now `pub(crate)`.
- the `🌿️vcs` test's `Patchable` impl still returned an inverse from `apply_patch` and omitted
  `diff_patch`; conformed to the current trait.
- `📡️spr/🎮️command` tests used the retired `Add { id, at }` / `Move { to }` field names. `os_spr::command`
  now simply re-exports the vcs type, so the "frozen-contract twin" docstring in `🌿️vcs` was stale and
  is corrected alongside.
- `📖️playbook`'s two nested test modules resolved `super::` to their enclosing `generation_forms` /
  `builder_kit` module instead of the file root.

`cargo test -p semio-framework-os-kernel --lib` now compiles and runs: **695 pass, 42 fail**. All 42
panic inside `🗣️dsl/🦀️component.rs` (grammar/derive round-trips, `family-*.grammar must parse`), which is
the in-flight handcrafted-grammar workstream, not this ticket — the same debt the policy gate reports
as `handcrafted-grammar/*` (352 breaches).

## Gate results

| Gate | Result |
| --- | --- |
| `policyArtifactSchemaBreaches` | 0 over 54 artifacts |
| `validateTaxonomy` | 0 problems; 13/13 tests pass |
| `cargo test -p semio-framework-schema` | 4/4 pass, incl. all-54 catalog validation |
| plugin crates, `cargo test --lib` | 61/62 pass (the 62nd, `trinity-jack-shell`, has no lib target) |
| `plugin-registry:check` | 0 findings on schema facets (395 pre-existing findings elsewhere) |
| `plugin-registry:generate` | succeeds; `.vscode/launch.json` already current |
| `cargo check --workspace --all-targets` | green except `semio-compose-rs` and `semio-framework-os-kernel-db` |

`workspace:verify-gate` still exits 1, stopping at its "generated catalog freshness" stage on the 395
pre-existing registry findings, so it never reaches the artifact-schema stage. That stage calls
`policyArtifactSchemaBreaches`, verified directly at 0. `bun ./📜️script.ts policy` also exits 1, on
1173 high-priority breaches, **none** in the `artifact-schema/*` family:

```
 291 taxonomy/emoji-prefix          206 handcrafted-grammar/declared-use
 113 mutation-migration/emoji-uniqueness   112 taxonomy/dead-example-leaf
  98 protocol-migration/command-envelope-completeness   89 dsl-migration/diff-completeness
  69 handcrafted-grammar/empty-example     47 pack-migration/completeness
  47 handcrafted-grammar/spec-distinctness 36 os-state-authority/item-scope-global
  30 handcrafted-grammar/spec-wiring-include  … 15 further rules
```

The 37 of these that mention `📸️snapshot/🎒️pack` only do so because the pack moved; they are
`protocol-migration/command-envelope-completeness`, and the same files lack
`assert_command_envelope_round_trip` at HEAD too.

## Out of scope, left failing (other workstreams)

- `semio-compose-rs` — unresolved `dsl`/`vcs` crates. Compose is explicitly excluded by the plan; it
  blocks `cargo test --workspace` as a dependency.
- `semio-framework-os-kernel-db` — `include_str!`s a `👁️preview` module that does not exist in the repo;
  identical to HEAD, and in the db read-model area the plan excludes.
- `semio-framework-os-flow` lib tests — 176 errors, plus two `include_str!`s of a `📚️examples/🌊️default.flow`
  fixture that exists nowhere (the module's `📚️examples/` is empty, i.e. the
  `handcrafted-grammar/empty-example` debt). Writing that fixture is the grammar workstream's call, not
  a mechanical fix.
- `semio-framework-{plugin-host, os-run, os-renderer-wgpu}` — missing `os_dsl` module.
  `semio-framework-os-infinite` — missing `🧊️capsule_J.glb` asset. `semio-framework-surface` lib tests —
  `MapHost` field drift. `semio-framework-os-kernel-neural-engine` — its own local `Schema` struct has
  no `extension` field. `semio-framework-editor` 1 failure, `semio-framework-math` 13 failures.
- `@semio-tech/framework-renderer-react` — the nx target's 15s command budget kills vitest before it
  finishes. Run directly: **503 pass, 12 fail**, all UI (icon keyframes, panel drag, tree guides,
  shell edges, camera duck-typing); none touch snapshots or schemas.
- 395 `plugin-registry:check` findings — missing `⚙️engine/🟦️component.ts` twins (52) and glue `#[path]`
  declarations; none concern the new facets.
