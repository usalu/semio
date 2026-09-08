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
