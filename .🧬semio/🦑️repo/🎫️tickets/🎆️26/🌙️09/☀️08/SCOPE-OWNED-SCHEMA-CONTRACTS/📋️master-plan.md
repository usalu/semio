# Scope-Owned Schema Contracts — Master Plan

Ticket: `2026/09/08/SCOPE-OWNED-SCHEMA-CONTRACTS` · Goal: `🎯aioptimizedrepo🎯repomechanisms🎯repoprojectmechanism`
Coordinator: Fable 5.1 (this session). Execution: Opus 5 workers. Audits: Sonnet 5 read-only auditors.
Repo MCP is down for this session; ticket folder is managed on disk.

## Invariant

Every application contract has exactly one eligible scope owner. The owner's `🧬️schema/` module holds the
authoritative native format implementations (`🔣️.json` JSON Schema, `🛰️.proto`, `🔗️.graphql`, `🦀️.rs`, `🟦️.ts`,
`📜️.wit` for interfaces) and exposes named exports. Consumers resolve `(scope id, export id, format)`.
Fixtures are examples only: they bind to an export, they never define the contract. Production artifacts run
without fixture directories. No duplicate authority, no fixture-local fallback, no per-contract scope.

## What the repo already has (anchor points, verified 2026-09-08)

- Vocabulary: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` — `schemaFormats`
  (rust/typescript/graphql/jsonschema/protobuf/wit with field casing), `schemaFacetKinds` (`🧬️data` normative
  jsonschema, `📜️interface` normative wit), `artifactSchemaSpecFileKinds`, `surfaceSchemaSpecFileKinds`
  (`🎚️config`/`👥️presence`/`🫧️transient` `/🧬️schema`), `mutationPayloadSchemaLocation`,
  `mutationPayloadSchemaAuthority` (descriptor-linked, draft-07), `schemaChildDirs`
  (`📸️snapshot`, `🔺️diff`, `🧬️mutations`, `💡️inferences`), `testSchemaLocation`.
- Canonical module example: `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/`
  with all five formats and `$id: https://semio.tech/schema/s/writer/writer/artifact.json`.
- ~495 `🧬️schema/` modules, ~2,272 `🧬️.schema.json` leaf files (≈2,100 mutation leaves, 164 under 🧪️/🧫️ trees),
  ~70 flat `🧬️schema.json` files in framework modules, 43 `🔣️.schema.json`, six `🧬️contracts/*` per-contract dirs.
- Test module: `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts` treats
  `🧬️schema/<json>` beside a leaf and a flat `🔣️.schema.json` as alternatives (~line 3580).

## Seed observations

1. `🌎️hub/🧪️fixtures/✅️inference-approval-v1/🧬️.schema.json` — a fixture-wrapper schema
   (`$id semio.hub.inference-approval-fixture/v1`, `hostile[]` cases, `maximumBytes`). The application contract
   `semio.hub.inference-approval/v1` exists only as `$defs.request` inside the fixture schema. Owner to be settled
   by the hub audit (producer/validator/consumer trace).
2. `…/🏛️ShellHost/🧬️contracts/🎟️invite-capability/🧬️.schema.json` — `$id …/directory/invite-capability-transfer.v1`,
   a schema of `const`s that mirrors its single instance `🔣️.json` (state machine spec + labels). `$id` says
   `directory`, the path says ShellHost UI element. Owner to be settled by the os audit; likely the shared
   directory/capability owner, not the UI consumer.

Both seeds are "schema of one example" documents. Relocation alone is insufficient; the contract must be
re-derived from the producer/consumer code.

## Work packages and gates

| WP | Work | Exit gate |
|----|------|-----------|
| WP0 audit | Eight Sonnet auditors: hub, os/ShellHost, framework modules, plugin fixtures, mutation leaves, mechanism, inline/generation, Pass-A ledger | No unresolved owner, no unexplained coverage gap; consolidated inventory `📊️wp0-inventory.json` |
| WP1 invariants | Failing tests for placement, owner eligibility, completeness, export resolution, fixture isolation, false positives | Tests distinguish the seeds from valid structures |
| WP2 mechanism | Owner declarations in the taxonomy/owner model, derived catalog, resolver `(scope, export, format)`, format handlers, diagnostics | Resolution works without fixtures or heuristic search |
| WP3 shared scopes | Consolidate genuine shared authorities (directory/capability, retained-command-limits, plugin identity, …) | Migrated consumers use canonical exports |
| WP4 application scopes | Migrate product/module/element contracts incl. the two seeds | Complete declared coverage per scope |
| WP5 fixture system | Explicit fixture bindings, rebind every fixture/test adapter, staged harness outcomes | Every fixture targets a declared export and fails/passes at the intended stage |
| WP6 implementations & publication | Bindings, native resources, packages, production-without-fixtures smoke | Conformance + production-only smoke pass |
| WP7 enforcement & removal | `schema audit/check/generate/verify/test/docs` commands in `📜️script.ts`, launch.json entries, nx inputs, delete obsolete authorities and fallbacks | Clean active-reference scan, clean production graph |

Relocation review and contract-correction review are separate steps within WP3–WP5.

## Fleet log

- 2026-09-08 · WP0 launched: `📓️wp0-hub`, `📓️wp0-os-shellhost`, `📓️wp0-framework-modules`,
  `📓️wp0-plugin-fixtures`, `📓️wp0-mutation-leaves`, `📓️wp0-mechanism`, `📓️wp0-inline-and-generation`,
  `📓️wp0-ledger` (+ `📊️*.json`, `wp0-ledger.py`, `🗑️generated/wp0-ledger-full.jsonl`).
- 2026-09-08 · WP0 landed (7 partitions + 4 framework-module slices being merged into `📓️wp0-framework-modules`).
  Key corrections to the initial reading: mutation leaves are the *declared* authority (not violations);
  the real duplicate authority there is inline-payload aggregates (G-A) and 505 missing declared leaf schemas
  (500 in stdio). ShellHost is structurally ineligible (`🧱️elements` kind ui). Hub's `hub.inference` module
  lacks approval/receipt definitions entirely; four independent copies of the approval contract exist.
- 2026-09-08 · `📋️execution-contract.md` written; Opus execution fleet launched over disjoint partitions:
  W1 harness invariants + `schema://` (🧪️test), W2 tooling/taxonomy/catalog (root script, 📚️library),
  W3 Rust schema registry exports + validator (🔨️modules/🧬️schema), W4 hub, W5 os (excl. mutations),
  W6 plugin fixtures (✏️s excl. mutations), W7 mutation aggregates + leaf relocation (non-stdio),
  W8 stdio mutation schemas (500 missing), W9 repo product/coordinator + print + mit-bestand,
  W10 framework modules (excl. 🧬️schema registry). Reports: `📓️wp1-*`, `📓️wp2-*`, `📓️wp4-*`.
- 2026-09-08 · Wave 1 landed for W2 (tooling: taxonomy keys, catalog, `schema` commands, 8,395 findings
  baseline), W3 (Rust registry + owned validator, 26 tests), W4 (hub: 10 modules / 143 exports, seeds fixed),
  W7 (1,766 leaves relocated, 59 aggregates → `$ref` unions), W9 (coordinator API module, Postgres rows,
  print/mit-bestand). `$id` grammar settled in the contract after W2's `scope-id-duplicate=2249`.
- 2026-09-08 · Wave 2 launched: W2b (leaf scopes in generator, Rust parity, ajv devDependency, torn reads),
  W7b (leaf `$id` grammar, 61 architect descriptors, framework/os leaves to draft-07), W9b (sqlite client
  ownership), W3b (validator `if/then/else` + full draft-07 keywords + runtime entries dump).
  Routing ledger: `📋️cross-partition-requests.md`. Harness `bun test`: 144 pass / 28 fail, triage queued (row 32).
- 2026-09-08 · Wave 1 fully landed (W1 harness 61 invariant tests; W6 plugins 57 violations + 12 plugin-root
  modules; W8 stdio 501 schemas authored / 414 relocated; W10 framework 117 modules / 211 exports).
  Wave 2 running: W2b, W7b, W8b, W9b, W3b, W10b, W1b, W4b. Queued on partition availability: W5b (os),
  W6b (plugins), W2c (tooling fix-ups), W10c (framework derive + ui widenings), W8c/W6c build wave
  (stdio casing + tagging, needs an idle machine). Contract §B gained format-coverage/`x-semio-formats`,
  fixture dir naming, `$id` grammar, and export-file catalog shape.
- 2026-09-08 19:30 · Claude Code process restarted; ten in-flight wave-2 workers stopped without reports
  (W2b, W7b, W4b, W9c, W6b, W3c, W10c, W8c, W5 lanes). Their partial edits are on disk. Relaunched as
  W2t (📚️library tooling), W2s (root script/launch/package), W7c (mutations non-stdio), W4c (hub),
  W9d (repo product), W6c (plugins), W3d (framework.schema facet + registry crate split), W10d
  (value derive + ui widenings), W8d (stdio aggregate refs + facets), W5b (os completion + rows).
  Each was briefed to assess the on-disk partial state first. Reports: `📓️wp2c-library-tooling`,
  `📓️wp2c-root-script`, `📓️wp4b-mutations`, `📓️wp4b-hub`, `📓️wp4c-repo-product`, `📓️wp4b-plugins`,
  `📓️wp3c-framework-schema`, `📓️wp4c-framework-modules`, `📓️wp4c-stdio-mutations`, `📓️wp4b-os`.
- 2026-09-08 20:40 · Wave 3 (completeness + refs + registration) opened on the first live-tree measurement
  with a real catalog: 3,493 findings (plugins 1,977 · stdio 691 · os 400 · mutations 135 · library 104 ·
  framework 103 · hub 68 · mit-bestand 8 · repo 7). Dispatched W4d (hub), W10f (framework excl. ui), W1d
  (harness rule activation: TS `parse<Export>()`, GraphQL keywords, `schema-ref-broken-internal`, shared code
  table). Other partitions follow as their wave-2 workers finish. Registry crate `semio-framework-schema-registry`
  exists (W3d), so every crate can register exports.
- 2026-09-08 20:25 · Machine rebooted ~18:31 (that killed the wave-2 workers); load 80–90 with 9 peer
  compilers, ~64 MB free. Stdio build wave (rows 46/49/82/94/110: `rename_all`/`tag` attributes, 569 structs,
  17+33+17 `rename_all_fields` enums, fixture re-casing, prepared in `wp4b-stdio-rust-changes.md`) stays
  deferred until the box is idle; JSON-only stdio work is complete (`wp4c-stdio-mutations`: aggregates by
  absolute `$id`, 38 scaffold facets deleted / 27 kept, `ref-not-catalog-addressable` 2,752 → 121 repo-wide).
- 2026-09-08 21:05 · Wave 2 relaunch complete for tooling (W2t/W2s), hub (W4c), repo (W9d), stdio (W8d),
  framework schema (W3d: dependency-free registry crate, `framework.schema` facet), framework modules
  (W10d), harness (W1c/W1d: 101/102 invariants, TS parser rule live), plugins (W6c). Running: W7c mutations,
  W5b os (five lanes), W10e ui vocabulary, W4d hub w3, W10f framework w3, W9e repo w3, W2v root, W2u library,
  W3e entity catalog, W6d plugins w3 (largest). Contract gained: no restated `$defs` across scopes, surface
  lanes as own scopes, runtime-enum vocabularies, `parse<Export>()` requirement, shared diagnostic-code table,
  entity-kind catalog owned by `framework.schema`. Queued: W4e, W2w, W5c, W7d, stdio build wave (load-gated).
- 2026-09-08 21:40 · Landed: W2t/W2v (library + root tooling, exempt area, shared code table half), W3d/W3e
  (registry crate, `framework.schema` facet, entity-kind catalog owned by framework with generated first-wins
  projections), W7c (mutations: absolute refs, injective ids, kind lists dropped), W9d/W9e/W9f (repo product
  clean incl. vscode revert, CLI consumes generated Go catalog), W4c/W4d (hub clean, 68 completeness rows
  cleared, 3 scopes registered), W10d/W10e (derive fixes, ui vocabularies from runtime enums), W1c/W1d
  (rules: internal refs, `x-semio-formats`, TS parser, GraphQL keywords, code table), W6c (plugins wave 2),
  W8d (stdio JSON-only complete). Running: W5b os (five lanes), W10f framework w3, W2u library, W6d plugins w3,
  W2x root, W1e harness, W7d mutations w3, W4e hub final, W10g ui single-owner. Queued: W2w, W5c, W1f, W2y,
  W6e; stdio build wave load-gated (load 128, 49 compilers at 21:25).
- 2026-09-08 22:35 · Session rate limit (reset 22:30) killed nine in-flight workers (W1f, W2y, W4e, W10g, W7d,
  W2u, W6d, W5b, W8e); W1e and W2y had already delivered (W2y: rows 133/138 need no root change — walker is
  library-owned, depth already correct). Relaunched as assess-first successors: W1f' harness, W2w library
  (incl. rows 133/147/115/124/129 + taxonomy), W7d' mutations, W10h ui + value scalars, W6e plugins wave 3
  (+ 24 missing root modules, stdio registry inventory deletion), W5c os (completion + all os rows + wave 3),
  W8f stdio roots + twins, W4f hub registration laws. Load 77 at relaunch.
