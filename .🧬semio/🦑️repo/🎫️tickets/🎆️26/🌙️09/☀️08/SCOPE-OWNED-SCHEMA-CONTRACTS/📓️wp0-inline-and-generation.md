# WP0 — Inline Schemas & Generation Audit (content/role-discovered, non-`*schema*`-named)

Read-only audit. Partition: schema definitions discovered by **content and role**, not by filename
containing "schema" (that's a different auditor's partition — see task-2 note below for the one
place the literal task text overrides that framing). Repo MCP was down for this whole session; no
`ticket_*` calls were made. Base corpus: `git ls-files` minus `node_modules/`, `target*/`,
`storybook-static/`, `temp/`, `test-results/`, `.🧬semio/🦑️repo/🎫️tickets/` — **61,722 tracked files**.
Full machine-readable data: `📊️wp0-inline-and-generation.json` (same directory as this file).

---

## Task 1 — Inline schema builders / validator literals in source

### Rust: `schemars`/`JsonSchema` — confined to two products under `💻️os`

`git grep -n -E 'schemars::|derive\([^)]*JsonSchema|impl JsonSchema for|schema_for!'` over all
`*.rs` returns exactly **5 files**, all inside `🧰️framework/🛍️products/💻️os/🔨️modules/`: `🌉️mcp/🦀️.rs`,
`🌉️mcp/🧬️schema/🦀️.rs`, `🌉️mcp/🧭️protocol/🦀️.rs`, `🌉️mcp/⚠️errors/🦀️.rs`, `🖥️shell/🦀️.rs`. Two clean,
well-evidenced violations and one clean exception:

**🌉️mcp — a real `🧬️schema/` module exists but 6 sibling facets bypass it.**
`🌉️mcp/🧬️schema/🦀️.rs` is exactly the intended pattern: `schemars`-derived structs
(`RevisionStamp`, `InvocationReport`, `PreparedActionReport`, `SearchHit`, `JobStatus`,
`ContextSummary`) plus a re-exported `GatewayError` (deliberately, per its own doc comment: *"one
type, one owning facet"*), all cataloged by `pub fn schemas() -> Vec<(&'static str, Value)>`
(lines 267–279) whose doc comment says this is *"the normative source `🧬️schema/🔣️.json`/`🟦️.ts`
mirrors are generated from"* — **but no `🔣️.json` or `🟦️.ts` exists yet in that directory** (`find`
confirms only `🦀️.rs` is present). Meanwhile:

- `🌉️mcp/🦀️.rs:324-372` hand-writes **4 raw `json!({"$schema": "https://json-schema.org/draft/2020-12/schema", ...})` literals** (`action_prepare_input_schema`, `action_invoke_input_schema`, `handle_input_schema`, `transaction_begin_input_schema`) — none of these have a Rust struct or a `schemas()` entry anywhere. Uncatalogued, untraceable.
- `🌉️mcp/🦀️.rs:384,524` call `schemars::schema_for!(PreparedActionReport)` / `schema_for!(InvocationReport)` **a second time**, independently of the schema module's own `schemas()` call for the same two types (lines 271-272 of `🧬️schema/🦀️.rs`). `prepared_action_report_output_schema()` then hand-patches the known "bare boolean subschema" bug via `schema.pointer_mut("/properties/preview")` — a **drifted duplicate** of `🧬️schema/🦀️.rs`'s own `normalize_boolean_subschemas()` fix for the identical bug (documented in the same file's doc comment about the MCP SDK's Zod validation rejecting boolean sub-schemas).
- `🌉️mcp/💡️inference/🦀️.rs:195,205,214,224,1171,1182,1193,1203`, `🌉️mcp/🖥️ui/🦀️.rs:316-384` (7 sites), `🌉️mcp/🗂️catalog/🦀️.rs:322-668` (5 sites), `🌉️mcp/🗿️artifact/🦀️.rs:46-151` (10 sites) — same `json!({"$schema": ...})` pattern, same story: per-facet, hand-authored, zero coverage in the schema module's registry.
- `🌉️mcp/🧭️protocol/🦀️.rs:315,449,482,566,586,605,684,695,711,717` — `#[derive(..., JsonSchema, ...)]` on the **actual MCP wire-protocol types** (`ContentBlock`, `Tool`, `CallToolResult`, `Resource`, `ResourceTemplate`, `ResourceContent`, `PromptArgument`, `Prompt`, `PromptMessage`, `PromptGetResult`) — schemars-derived but **none** appear in `schemas()`. Open question: is `🧭️protocol/` meant to be exempt because it mirrors an external spec (MCP) rather than a semio-authored contract? Either way it isn't inside a `🧬️schema/`-named location today.

Decision recorded per-file in the JSON (`inlineSchemas[]`). Net effect: `🌉️mcp` needs every one of
these facets to import named exports from `🧬️schema/🦀️.rs` instead of re-deriving/hand-writing.

**🖥️shell — no `🧬️schema/` module exists at all.** `find 🖥️shell -maxdepth 2` shows only
`🤖️generated/`, `🦀️.rs`, `🧫️fixtures/`, `🟦️.ts`, `📦️packages/` — no `🧬️schema/`. Yet `🖥️shell/🦀️.rs`
carries **~30** `#[derive(..., schemars::JsonSchema, ...)]` structs (`ActiveSession`, `Anchor`,
`LayoutNode`, `DockUiState`, `ShellCommand`, etc. — lines 222 through 2099) and calls
`schemars::schema_for!(ShellCommand)` directly at line 2274. This is the module's *only* source of
truth (no competing definition found), but it lives in the wrong place, and it is the direct input
to a live generator (see Task 4). Clean fix: create `🖥️shell/🧬️schema/🦀️.rs`, move the structs
there, make `🦀️.rs` a consumer.

**False positive worth flagging: `$schema` reused as a value-kind tag, not a schema keyword.**
`✏️s/🔌️plugins/🌀️procedural/.../🧊️generation3d/.../✏️editor/🦀️.rs:2637-2749` and
`🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs:4430` both write JSON like
`{"$schema": "point", "x":1.0, ...}` / `{"$schema":"number","value":7.5}` — this is the flow wire
format's own type-discriminator field, spelled identically to the JSON-Schema `$schema`
meta-keyword by coincidence of naming, not a schema definition. Same pattern appears legitimately
*inside* real schema files too (e.g. `🧰️framework/🔨️modules/🧵️job/⏱️budget/🧬️schema/⏱️clock.json`'s
`properties.$schema: {"const": "./🧬️schema/⏱️clock.json"}` — a self-locator field nested under
`properties`, distinct from the document's own top-level `$schema` draft declaration). Not a
scope-ownership violation; flagged as a naming-hygiene collision that also tripped up this audit's
own automated scan and will trip up any generic JSON-Schema tool.

### TypeScript: a hand-rolled zod-shaped validator owns an entire, uncovered REST API

`🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🟦️typescript/✅️validation.ts`
(90 lines) defines `class OwnedSchema<T>`, `OwnedStringSchema`, `ownedObject(shape)` — an in-house
validator-builder with a zod-shaped API (`.object()`, `.string()`, `.min()`, `.email()`,
`.default()`, `.nullable()`, `.safeParse()`), exported as `ownedSchema`. Per CLAUDE.md's "wrap
external libraries behind an interface" rule this is deliberate (confirmed: real `zod` only shows
up vendored inside `node_modules/` test fixtures, never imported at runtime) — **but it is itself
the schema-definition mechanism for the coordinator's whole REST surface**, and it lives outside
any `🧬️schema/` module. `🎛️coordinator` has no `🧬️schema/` directory (only `🧫️fixtures/` and
`📦️packages/`). Five route files import it as `{ ownedSchema as z }` and call `z.object({...})`:

- `app/api/v1/auth/route.ts:30,36,45,61` — `CreateKeySchema`, `CreateDeveloperSchema`, `RevokeKeySchema`
- `app/api/v1/diff/route.ts:21,25` — `DiffIngestSchema`
- `app/api/v1/event/route.ts:16` — `EventSchema`
- `app/api/v1/repo/route.ts:21,27,62` — `IndexFileSchema`, `ReindexSchema`
- `app/api/v1/ticket/route.ts:22,35,42,73` — `TicketOpenSchema`, `TicketCloseSchema`, `TicketReopenSchema` — **this is the literal wire contract for the `ticket_open`/`ticket_close`/`ticket_reopen` repo-MCP tools this very ticket workflow depends on**, and it has zero `🧬️schema/` coverage anywhere in the repo.

This is a systemic gap, not a duplicate: 10 wire contracts across 5 endpoints, one single
hand-rolled validator library, no JSON Schema/`.ts`-named-export equivalent anywhere. Open
question for the ticket owner: should HTTP request-body validation be treated as the same
"application contract" concern the goal targets, or does it need its own convention?

### Python / Go / C#

- Python: only 3 hits, all `import jsonschema` / `from jsonschema import validators` inside test/oracle files (`🔋️energy` plugin tests, `🧩️puzzle` third-party oracle test) — genuine **consumers** validating against real schema files, not competing definitions. No `pydantic` usage found repo-wide.
- Go: 0 hits for `jsonschema.`/schema-shaped imports.
- C#: 0 hits for `JsonSchema`/`NJsonSchema`/`DataAnnotations`.

---

## Task 2 — Schema-shaped JSON files whose names don't say "schema"

Scanned all **22,354** tracked `.json` files with a Python JSON parser (21 failed to parse or were
skipped as oversized — see `meta.jsonParseFailureSample` in the JSON deliverable — none of the
failures were in schema-shaped-looking paths). Excluded exactly what the task text excludes:
`🧬️schema/**/🔣️.json` (canonical) and literal `*.schema.json` (dotted suffix — note this does
*not* exclude a file like `🛂️schema.json`, which lacks the delimiting dot before "schema", per the
task's own worked example). Everything else with `$schema`/`$id`/`$defs`/`definitions`, or
`type:"object"`+`properties`, is INCLUDED here even when its path contains the word "schema" in
some other form — that's what task 2's own example list (`🛂️schema.json`) establishes.

**707 files matched** (168 `definition`, 439 `instance-with-schema`, 100 `third-party` config files
like `📋️project.json`/`tsconfig.json` that happen to carry a `$schema` pointer into
`node_modules/nx/schemas/...` or similar — excluded from the counts below). Full per-file list
with classification and `$id` in `schemaShapedJson[]` of the JSON deliverable.

### `definition` bucket (168) — real schema documents, by naming pattern

| pattern | count | assessment |
|---|---|---|
| flat `🧬️schema.json` file at a module root (not a `🧬️schema/` directory) | 72 | Consistent, repo-wide convention under `🧰️framework/🔨️modules/**` — used for single-format (JSON-Schema-only) leaf contracts, as an apparent lighter-weight sibling to the `🧬️schema/` directory convention used by richer, multi-format scopes. Not obviously accidental drift (too consistent), but it does not match the goal statement's literal "`🧬️schema/` module" phrasing. **Flagged as an open architecture question**, not 72 individual violations — full list in JSON. |
| `🔣️.json` inside a directory named `📐️schema` (not `🧬️schema`) | 40 | Same JSON-Schema-draft-07 content/role as canonical, different emoji. Mostly `📐️fixture-schema/` (test-fixture-scoped) plus a few production ones, e.g. `🔌️plugin/📇️registry/📦️deployment/📐️schema.json` (flat file, real draft-07 schema, **no `🧬️schema/` dir exists for that module at all** — genuine violation, not just naming). |
| `🔣️.json` inside a directory named `🛂️schema` | 10 | Exclusively under `🧪️tests/*/` — test-fixture-expectation schemas (e.g. `🌳️workspace-taxonomy/🛂️schema.json`, confirmed by content: validates test-case arrays, not a production contract). Low priority. |
| `🔣️.json` elsewhere / other one-off names | 23 + 18 | See "other-named definitions" full list in JSON. Includes the ticket's own worked examples: `⏱️clock.json`, `🪢️binding.json`, `🪫️budget.json` (all inside `🧰️framework/🔨️modules/🧵️job/⏱️budget/🧬️schema/` — **right directory, multi-file naming instead of single `🔣️.json`**, confirmed genuine draft-07 documents by content), `🩹️receipt.json` / `🧵️production.json` (same pattern, `💻️os/.../⚛️reactor/`). |

**Key nuance:** several of these are in the *correct* `🧬️schema/` directory but use a per-concern
filename (`⏱️clock.json`, `🪢️binding.json`, ...) instead of the single canonical `🔣️.json`. That's a
convention deviation, not a missing-owner violation — worth a naming-convention ruling (does
`🧬️schema/` allow multiple named JSON Schema files, or must it collapse to one `🔣️.json` with
`$defs`?). By contrast `📇️deployment/📐️schema.json` and `🖱️ui/🧬️contract/🧵️retained/💾️resident/🧬️schema.json`
(a *third* directory-name variant, `🧬️contract`, doing the schema-directory role) are real
draft-07 documents with **no `🧬️schema/` directory anywhere in their module** — genuine violations.

### `instance-with-schema` bucket (439) — payloads/configs that carry `$schema` but aren't definitions

- 198 are `🧪️oracle/🔣️.json` files: a thin per-plugin pointer whose `$schema` value is a relative
  path back to the single shared
  `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/🔣️.json` — this is the *intended*
  consumer pattern (one canonical schema, many referencing instances), not drift.
- 183 are `📋️project.json` build-config files pointing at `node_modules/nx/schemas/project-schema.json`
  — third-party, no action.
- 58 remaining "other" — mostly test-fixture "law"/oracle files (`⚖️*-law.json`,
  `👑️*-scene-owner-law.json`) referencing sibling `*.schema.json`/`🧬️.schema.json` files by
  relative `$ref`-like `$schema` value, or genuine app configs (`🖱️ui/components.json` →
  `https://ui.shadcn.com/schema.json`, third-party shadcn config). Full list in JSON.

---

## Task 3 — Alternative formats outside `🧬️schema/`

`.proto`/`.graphql`/`.wit`/`.avsc`/`.xsd`/`.sql`/`.ksy`/`.ebnf`/`.abnf`/`.g4`/`.spicy`/
`📖️.grammar.semio`/`📡️.protocol.semio` (6,200 total tracked files with these extensions; 5,774
already live inside a `🧬️schema/` module and are out of scope here). **426 files across 103
directories** live outside `🧬️schema/`, and they are *not* scattered — every one sits under a
per-plugin, per-artifact path shaped
`✏️s/🔌️plugins/*/🗿️artifacts/*/…/🚪️io/{💡️inferences,📸️snapshot,🔺️diff,🧬️mutations}/{binary,text}/`,
one file per grammar dialect (`🅰️.g4`, `🔠️.abnf`, `🔤️.ebnf`, `🥋️.ksy`, `🌶️.spicy`, `🛰️.proto`,
`🔗️.graphql`, `📖️.grammar.semio`, `📡️.protocol.semio`). Sampled content is tiny, near-identical
scaffold (~100-130 bytes each, e.g. `✒️writer`'s `.graphql`: `scalar Bytes\ntype Document { schema:
String! payload: String! }`). This is a **deliberate, repo-wide structural pattern**, not drift —
but it lives under a differently-named directory (`🚪️io/...`) than the goal's `🧬️schema/`
convention, and nothing documents whether these are authoritative or meant to be
generated-from/projected-from each artifact's own `🧬️schema/` module. **Open question for the
ticket owner**, not an individual-file violation list. Full 426-path list in
`alternativeFormats.outsideSchemaDirsFiles`.

---

## Task 4 — Generators

`git grep -l` across every `📜️script.ts` for `schemars|prost|tonic|protoc|graphql-codegen|
json-schema-to|x-semio-generated|typegen|schema_for!` returns 18 files (full list in JSON). Two
were manually verified end-to-end; the rest are flagged for follow-up (see coverage below — most
hits from a quick spot-check are dependency-audit code scanning for these as literal *external
crate names*, not generators themselves).

1. **`🖥️shell` typegen — code→code, confirmed live.** `bun ./📜️script.ts typegen` (target wired in
   `📋️project.json`) runs `cargo test --features typegen` with `SEMIO_TYPEGEN_OUT` pointed at
   `🖥️shell/🤖️generated/🟦️.ts`, which carries the header *"@generated by bun nx run
   @semio-tech/framework-os-shell-rs:typegen from 🖥️shell owned schema metadata. Do not edit."* —
   provenance marker present. Source is the ~30 `schemars`-derived structs living directly in
   `🖥️shell/🦀️.rs` (Task 1 finding). `namedInputs.default` in `📋️project.json` covers
   `🖥️shell/**/*.rs`, so nx cache invalidation is correctly wired even though the `typegen` target
   itself declares no input override — only `outputs`. **This is the clearest code-is-source-of-truth
   violation found**: because there's no `🧬️schema/` module, the generator's input is ordinary
   module code, not a schema-module artifact.
2. **`🌉️mcp` gateway schema catalog — schema→code, designed but not yet built.** `🧬️schema/🦀️.rs`'s
   doc comment describes `🔣️.json`/`🟦️.ts` mirrors generated from `schemas()`, but neither file
   exists yet. Correct direction (schema owns, code/artifacts are generated), incomplete
   implementation — and meanwhile the 6 sibling facets in Task 1 hand-author their own schemas
   instead of waiting on/wiring into this pipeline.
3. The remaining 16 files in the grep hit list were not individually traced to a generation
   pipeline this session — recorded as a coverage gap below.

---

## Task 5 — `$ref`/`$id` graph

Parsed all 22,354 tracked JSON files; walked every dict for `$ref` and `$id` keys.

- **6,681 total `$ref` values.** 5,737 are same-document `#/...` fragments (always structurally
  local, not further resolved). 231 are absolute `https://...` `$id`-style refs (registry-resolved,
  not filesystem paths). The remaining 713 relative/bare refs were resolved against the referring
  file's own directory.
- **260 of those 713 did not resolve to a file on disk.** Split into two buckets in the JSON
  (`refGraph.unresolvedLikelyBroken`, 134 entries) and (`refGraph.unresolvedAmbiguousIdStyle`, 126
  entries, truncated to 80 in the JSON for size — count is exact).
  - **Genuinely broken** (spot-checked, e.g. stdio/gltf's `🧬️mutations/🔣️.json` `$ref`s like
    `🎞️animation/🌱️create/🧬️.schema.json`): the target subdirectory **does not exist** — `find` on
    `🧬️mutations/` shows only a flat `🔣️.json`+`🛰️.proto`+`🔗️.graphql`+`🟦️.ts`, no per-mutation
    subfolders. This reads as stale `$ref`s left over from an earlier per-mutation-file layout that
    was since consolidated into one `🔣️.json`, without updating the internal refs. Repeats across
    gltf (animation, camera, scene), and the bare-filename form `snapshot.json#/definitions/...`
    (no such literal filename exists anywhere — real files are `📸️snapshot/🔣️.json`) repeats across
    LAS, IFC, PDF, MP4, AVI, DWG stdio artifacts.
  - **Ambiguous/likely intentional**: bare `$id`-style refs like
    `semio.flow.artifact-canonical#/definitions/widget` — these look like they're meant to resolve
    through a runtime schema-registry keyed by `$id`, not a filesystem path, so "unresolved by
    naive path resolution" may not mean "broken" for these. Not verified against the actual
    validator/loader code this session — flagged as needing the runtime loader's resolution
    algorithm to confirm.
- **Cross-scope refs: 0 found** by a coarse "first 4 path segments differ" heuristic. Caveat: this
  heuristic is crude for a repo this deep; not a strong guarantee.
- **19 duplicate `$id` values.** 18 of them are expected/likely-intentional: the same logical
  artifact schema reused verbatim across sibling *standard-version* subsets (e.g. stdio/gif
  standard `7️⃣87a` vs `9️⃣89a`, stdio/pdf `1.4` vs `1.7`, stdio/avi, stdio/dwg) — same `$id`,
  different standard revision directories. Worth a ruling on whether standard-version schema `$id`s
  should be revision-qualified, but not obviously a bug.
  - **The 19th is a real bug, found and diffed this session:**
    `https://semio.tech/schemas/os/gis-map-approval-history-fixture-v1.json` is shared by
    `🧰️framework/🛍️products/💻️os/🧪️fixtures/↩️gis-map-approval-history-v1/🧬️.schema.json` **and**
    `🧰️framework/🛍️products/💻️os/🧫️fixtures/↩️gis-map-approval-history-v1/🧬️.schema.json` — same
    `$id`, **different content**: one requires `oldOwnerRetired` with `minItems`/`maxItems` 8/8 on
    an array field, the other omits it from `required` and uses 9/9. See Task 6 for the root cause
    (`🧪️fixtures` vs `🧫️fixtures` directory-naming split).
- **`x-semio-*` extension keywords found:** `x-semio-state`, `x-semio-derived`,
  `x-semio-mutationKinds`, `x-semio-retained-discriminator`, `x-semio-invariant`,
  `x-semio-child-kind`, `x-semio-link-roles`, `x-semio`, `x-semio-mutation`, `x-semio-child`,
  `x-semio-link`, `x-semio-maxEncodedUtf8Bytes`, `x-semio-maxUtf8Bytes`, `x-semio-binary`,
  `x-semio-note` — 15 distinct keywords, file counts and inferred meanings in
  `extensionKeywords` of the JSON deliverable (meanings inferred from key names and surrounding
  schema context, not from a written spec — flagged as inferred, not confirmed).

---

## Task 6 — Cross-platform collision check

Checked all 61,722 tracked paths for (a) case-insensitive collisions, (b) NFC vs NFD Unicode
normalization producing distinct byte sequences for "the same" path, (c) presence/absence of the
U+FE0F variation selector on emoji path segments. **All three came back empty** — the repo's
emoji-path convention is byte-for-byte consistent everywhere tracked.

Two **related but distinct** findings, not pure Unicode collisions:

1. **Schema-directory role is spelled at least 3 different ways**: `🧬️schema` (canonical, hundreds
   of uses), `📐️schema` (~40 uses, mostly `📐️fixture-schema/`), `🛂️schema` (~10 uses, test-only),
   plus one-off `🧬️contract` (`🖱️ui/🧬️contract/🧵️retained/💾️resident/🧬️schema.json`). Same semantic
   role, different emoji — not a byte-identical collision, but exactly the kind of drift that makes
   "does every scope have exactly one eligible schema owner" unanswerable by directory-name search
   alone.
2. **Fixture-directory naming split caused a real bug.** `🧪️fixtures` (536 tracked-path
   occurrences) and `🧫️fixtures` (4,200 occurrences) are both live, widely-used, non-typo
   conventions for the same role, including as *siblings inside the same product* (`💻️os` has
   both). This directly produced the duplicate-`$id`/drifted-content bug documented under Task 5.
   Needs an owner decision on which is canonical and reconciliation of the stale duplicate (not
   performed here — read-only audit).

---

## Coverage — what was and wasn't fully run down

- **Fully verified, line-cited:** all Rust `schemars`/`JsonSchema` usage (5 files); the coordinator
  TS validator + all 5 route files; the shell typegen pipeline end-to-end; the mcp schema-module
  doc-comment-vs-reality gap; the `$schema`-as-tag false positive; the duplicate-`$id` fixture bug
  (diffed byte-for-byte); the 3-way schema-directory-name variance.
- **Grep-complete, not individually narrated per file:** the 707-entry `schemaShapedJson` list (all
  paths + classification + `$id` in the JSON); the 426-entry alternative-formats list (pattern
  identified and one sample content-checked, not all 426 opened); the 260 unresolved `$ref`s (one
  cluster per stdio artifact spot-checked by directory-listing, not all resolved individually); the
  15 `x-semio-*` keyword meanings (inferred from name + one usage site each, not cross-checked
  against every occurrence).
- **Named but not traced to a concrete pipeline:** 16 of the 18 `📜️script.ts` files matched by the
  Task 4 generator grep (`♻️mit-bestand/📋️bericht/...`, `🌎️hub/📦️packages/🦀️rust/📜️script.ts`,
  `🧰️framework/📦️packages/🦀️rust/📜️script.ts`, `🧰️framework/🔨️modules/⏱️trace/⏱️clock/🏁️tail/...`,
  `⏳️async`, `🎭️actor`, `🖱️ui/🎨️styling`, `🖱️ui/🧬️contract`, `🔌️plugin/🌐️browser-bundle/🧪️fixtures/*`
  (×2), `🔌️plugin/📦️packages/🦀️rust`, `🧑‍💻dev/📦️packages/🟦️typescript`, `📓️print/🔨️modules/🖨️tectonic-template-compilation/*`
  (×2), `🦑️repo/🔨️modules/📚️library/⚡️caching`). Full path list in the JSON's `generators` follow-up
  note. Recommend a targeted second pass on these specifically.
- **Not attempted:** opening and content-verifying all 426 alt-format files individually (only 1
  sampled per extension family); resolving the 126 ambiguous `$id`-style unresolved refs against
  the actual runtime schema-loader code (loader source not located/read this session); cross-scope
  ref detection used a coarse 4-segment-path heuristic rather than a real scope-ownership map.
- **Go / C#**: zero hits repo-wide for the searched patterns — either genuinely no Go/C# schema
  code exists, or the repo has very little Go/C# source at all (39 `.go` and 5 `.cs` files
  tracked total) — not deeply investigated further given the near-zero surface area.
