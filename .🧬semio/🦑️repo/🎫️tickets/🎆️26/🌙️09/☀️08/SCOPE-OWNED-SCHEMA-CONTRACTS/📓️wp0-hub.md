# WP0 Audit — 🌎️hub

Partition: everything under `🌎️hub/`. Companion machine inventory: `📊️wp0-hub.json` (76 entries, one per
schema-shaped file found in this partition). Read-only audit; no files outside these two deliverables were
modified. Repo MCP was down for this session — ticket managed on disk per instructions.

## 0. Owner/scope model for 🌎️hub (task 5)

`🌎️hub` is registered in `🔣️taxonomy.json` (`areaLayers`) as **one** physical area, layer `implementation`
(alongside `🧰️framework`, `✏️s`, `♻️mit-bestand`). It is **not** subdivided into separately-registered scopes.
Only three real package manifests exist under `🌎️hub/`:

| Manifest | Package | Covers |
|---|---|---|
| `🌎️hub/📦️packages/🦀️rust/Cargo.toml` | `semio-hub` (`[package.metadata.semio] role = "hub"`) | The single Rust crate for essentially all of hub: `📇️directory/`, `🗿️artifact-authority/` (incl. `🔏️trusted-catalog`, `🌱️creation`, `📇️native-openable-provider`), `💡️inference/`, `🔐️auth/`, `🚀️local-bootstrap/`, `🛰️lag-rebootstrap/`, plus the root `🧪️fixtures/` — all reached via `#[path=...]` module declarations from one `[lib] path = "🦀️.rs"`. |
| `🌎️hub/📦️packages/🟦️typescript/package.json` | hub's TS e2e/client package | Hosts `🤝️index.test.ts` (Vitest e2e against the real `os-hub` binary) and consumes `🔐️auth/🧬️schema/🟦️.ts`. |
| `🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript/package.json` | admin UI module | Separate TS package with its own `📜️script.ts`/tests; unrelated domain (admin dependency-graph tool output), not part of hub's wire-protocol surface. |

**Finding:** sub-areas named in the task brief (`📇️directory`, `🗿️artifact-authority`, `🔏️trusted-catalog`,
`📦️packages`) are **directories/modules inside the one `semio-hub` crate**, not independently eligible scopes.
The only eligible complete scopes touching this partition are `semio-hub` (Rust) and the two TS packages above.
This matters directly for classification below: since hub is one scope, the correct — and, for `💡️inference`
and `🔐️auth`, already-used — location for an authoritative contract is `🌎️hub/<area>/🧬️schema/`, not a
separate per-sub-area scope.

## 1–2. Inventory (tasks 1–2)

76 schema-shaped files were found under `🌎️hub/` (see full detail with blob ids in `📊️wp0-hub.json`):
`.proto`/`.graphql` — **none** exist under hub. Everything is JSON Schema (`🔣️.json` or `*.schema.json`), plus
the Rust/TS modules that hold the real (non-JSON-Schema) implementations. Breakdown by classification:

- **(a) authoritative scope schema, in a real `🧬️schema/` module — 10 files.** `💡️inference/🧬️schema/{🦀️.rs,🟦️.ts,🔣️.json,✅️approval/🦀️.rs}`, `🔐️auth/🧬️schema/{🔣️.json,🟦️.ts}`, `🚀️local-bootstrap/🧬️schema/{📨️credential-envelope-v1,🚇️pipe-v1,🩺️readiness-v1}/🔣️.json`, `🗿️artifact-authority/🔏️trusted-catalog/🧬️schema/🔣️bundle.schema.json`.
- **(b) fixture-owned contract (violation) — 50 files.** A `🧬️.schema.json` (or `🔣️.json`) sitting next to a `🔣️.json` fixture inside a `🧪️fixtures`/`🧫️fixtures`/`🧪️tests` leaf, functioning as a hand-written, independently-authored duplicate of a contract whose real definition lives elsewhere (usually a Rust struct in the same `semio-hub` crate).
- **(c) per-contract directory (violation, worse) — 14 files.** Either (i) a fixtures leaf holding *multiple* distinct `*.schema.json` siblings for different data shapes (`🔏️trusted-catalog/🧪️fixtures/📤️publication/*`, 7 files), (ii) a versioned leaf under a real module that mixes production `.rs` + fixture + multiple schemas flat (`🌱️creation/📚️operation-v1`, `🌱️creation/🧪️transaction-v1`, 4 files), or (iii) a `🧬️schema/` **subdirectory nested inside a `🧪️fixtures` leaf**, aping the authoritative-module naming convention while remaining test-scoped (`presence-lease-v1`, `presence-normalization-v1`, `artifact-chunk-cas`, 3 files).
- **(d) inert parser test input — 2 files.** `🔨️modules/🛡️admin/📦️packages/🟦️typescript/🧪️tests/{📐️entry-graph,🧵️stylesheet-graph}.schema.json` — validate a *dev-tool's own graph output shape*, not a hub application/wire contract. Different domain entirely; correctly test-local.
- **(e) config instance with `$schema`, (f) third-party, (g) generated copy — none found** as domain contracts. (`project.json`/`package.json`/`tsconfig.json` files under hub carry `$schema` pointing at tooling schemas — excluded as noise, listed in Coverage below.)

**Cross-cutting dialect finding:** taxonomy's `mutationPayloadSchemaAuthority.jsonSchemaDialect` mandates
`http://json-schema.org/draft-07/schema#`. Of the 76 files, only **3** use draft-07
(`📦️packages/🦀️rust/🧪️fixtures/🛂️admin-presence-target-recovery-v1/🧬️.schema.json`, the two admin
`🧪️tests` schemas, and `🧪️fixtures/🪪️execution-target-relay-v1/🧬️.schema.json`). Every other file —
**including the authoritative `🧬️schema/` modules themselves** (`💡️inference`, `🔐️auth`,
`🚀️local-bootstrap`) — declares `https://json-schema.org/draft/2020-12/schema`. This is a repo-wide dialect
violation independent of the fixture-ownership problem and should be fixed together with relocation.

**Repo-wide consumption pattern (task 4):** with the exception noted above for `🔐️auth` (real named-export
consumption), every fixture/schema pair in this partition is read by exactly one `async function
prove<Name>Fixture(repoRoot)` in `🌎️hub/📦️packages/🦀️rust/📜️script.ts` (87 such functions total; the
`nx`-registered `os-hub` fixture-check target calls them via `TestScript`/`BundleScript` subclasses), using
`ajv`/`ajv/dist/2020` (a third-party library) to independently validate the fixture JSON — this is exactly the
CLAUDE.md-mandated "one language-agnostic test with a third-party library producing the same output as our own
implementation" pattern, cross-checked against the real Rust struct's own `#[cfg(test)]` module (which loads the
*same* `🔣️.json` via `include_str!`). Three exceptions use a **different** harness: `🌎️hub/📦️packages/🟦️typescript/🤝️index.test.ts`
(Vitest, for `🛰️lag-rebootstrap/🪢️canonical-pair` and `🗿️artifact-authority/🧪️fixtures/🧱️artifact-chunk-cas`)
and `🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript/📜️script.ts` (for the two admin graph schemas).

This confirms the fixture-adjacent schemas are functioning as **intentional dual-implementation test oracles**,
not accidental drift — but they still violate the target architecture stated in the goal: the JSON Schema half
of that oracle is hand-authored next to the fixture instead of being the scope's own `🧬️schema/` JSON form
(which, for `💡️inference`, already exists and is even loaded directly by one of these functions —
`proveGisMapProposalApprovalFixture` at `📜️script.ts:7097` does `ajv.addSchema(...🌎️hub/💡️inference/🧬️schema/🔣️.json)`
— proving the pattern is only partially applied).

## 3. Seed case: `🌎️hub/🧪️fixtures/✅️inference-approval-v1/🧬️.schema.json`

**Fixture contents.** `$id: semio.hub.inference-approval-fixture/v1`; wraps one valid `request` (`$defs.request`,
`$id`-equivalent `semio.hub.inference-approval/v1`, fields `schema/version/jobId/proposalHash`) plus 14
`hostile` mutation cases. Blob `44b1a3eb…`, sibling `🔣️.json` blob `73eef8c2…`.

**Producer/validator trace.**
- The real, authoritative decoder is `InferenceApprovalRequestV1::decode` in
  `🌎️hub/💡️inference/🧬️schema/✅️approval/🦀️.rs:9-27` — `#[serde(rename_all = "camelCase", deny_unknown_fields)]`,
  checks `schema == "semio.hub.inference-approval/v1"`, `version == 1`, `job_id` is 32 hex chars, `proposal_hash`
  is 64 hex chars, and enforces `REQUEST_MAX_BYTES = 1024` (`💡️inference/🧬️schema/🦀️.rs:9`). Its own
  `#[cfg(test)]` module (lines 30-49) loads the **same** `🔣️.json` fixture via `include_str!` and iterates the
  same 14 hostile cases — i.e. the fixture is already correctly bound to the Rust implementation as example data.
- The route that calls it: `approve_gis_map_job` in `🌎️hub/💡️inference/🏃️runtime/🦀️.rs:3427-3451`
  (`let approval = super::schema::InferenceApprovalRequestV1::decode(body)?;`), wired into the hub binary at
  `🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:8077`.
- `🌎️hub/📦️packages/🦀️rust/📜️script.ts:7634-7661` (`proveInferenceApprovalRequestFixture`) is the **second**,
  independent implementation: reads `🔣️.json` + `🧬️.schema.json` from the fixture directory, compiles the
  schema with `ajv/dist/2020`, and re-runs the same valid/hostile/byte-boundary assertions in JS. This is the
  file that actually reads `🧬️.schema.json` — it has no other consumer.

**Is this really Hub's contract?** Yes — but it is **not hub-exclusive**. `semio.hub.inference-approval/v1` is
the wire contract between the OS/MCP bridge (client) and Hub (server), and Hub is the correct *authority*
because it is the party that decodes/enforces it (server-side validation is canonical). However the schema id
and struct are **independently re-implemented three more times**, once per client-side consumer:
1. `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs:1995` (`GisMapInferenceApprovalRequestV1`)
   and the matching TS at `…/📇️directory/🧬️schema/🟦️.ts:1626-2020` (`sealGisMapInferenceApprovalRequestV1`).
2. `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/💡️inference/🦀️.rs:654-661,986` (another
   `GisMapInferenceApprovalRequestV1`, own `new()` constructor, own `GIS_MAP_INFERENCE_APPROVAL_SCHEMA` const).
3. `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript/💡️inference-bridge.ts:61,259` (inline
   AJV-shaped object literal).
All four independent definitions agree on the exact field set (`schema`, `version`, `jobId`, `proposalHash`,
`deny_unknown_fields`/`additionalProperties: false`) — so today they are *consistent*, but nothing enforces that
they stay consistent; a change to hub's struct would not fail any of the other three at compile/type time.

**What `-v1` means.** It is **not** an independently versioned protocol artifact — no `📸️snapshot`/`🔺️diff`
history, no version-negotiation code. It is simply the fixture-directory naming convention (`<slug>-v1`) used
uniformly across all ~60 fixture leaves in this partition; the `version: 1` field inside the payload is the only
real protocol version marker, and it is asserted as a `const` in every implementation.

**Fixture schema vs. real contract — gap analysis.**
- Request fields match exactly (`schema`, `version`, `jobId`, `proposalHash`) between the fixture schema, the
  Rust struct, and all three cross-scope duplicates. No field-level gap on the *request* side.
- The **approval receipt** (`InferenceApprovalReceiptDtoV1` at `🏃️runtime/🦀️.rs:3160`,
  `GisMapInferenceApprovalReceiptV1` in both framework copies) has **no JSON Schema representation anywhere** —
  not in the fixture, not in `💡️inference/🧬️schema/🔣️.json` (confirmed: `grep -c approval` on that file is 0),
  not in `💡️inference/🧬️schema/🟦️.ts` (only 2 exports total, neither approval-related). The receipt shape is
  validated only implicitly through hand-written assertions in `📜️script.ts` and the framework-side Rust/TS
  duplicates.
- `💡️inference/🧬️schema/🔣️.json` — the one place the JSON-Schema half of this contract *should* live per the
  goal — currently only defines `request`/`identity`/`binding` (`$id: urn:semio:hub:inference-job:v1`); it has
  **zero** definitions for approval request or receipt.

**Decision:** owner scope is **`semio-hub`** (specifically `💡️inference`), consistent with the `semio.hub.*`
naming — confirmed by grep, not assumed. Intended fix: extend `🌎️hub/💡️inference/🧬️schema/🔣️.json` and
`🟦️.ts` with `approval`/`approval-receipt` definitions/exports (mirroring the pattern already used for
`request`/`identity`/`binding`), have `📜️script.ts`'s oracle load the schema from there (as
`proveGisMapProposalApprovalFixture` already partially does) instead of from a private copy in the fixture
directory, and — separately — decide whether the framework/os three duplicate client-side re-implementations
should instead re-export from hub's module (cross-scope import) or remain intentionally independent
per-CLAUDE.md multi-implementation policy; that decision is out of this partition's scope (belongs to the os/
framework auditor) but is flagged here because it directly bears on where "the" contract lives.

### 3b. The eight named sibling `-v1` fixtures

All eight follow the **identical** pattern confirmed above (own `🧬️.schema.json` fixture-oracle consumed by
exactly one `prove*Fixture` in `📜️script.ts`, cross-checked against a real Rust implementation elsewhere in
`semio-hub`); full evidence lines are in `📊️wp0-hub.json`. Summary:

| Fixture | Real owner (producer/consumer) | Notes |
|---|---|---|
| `↩️gis-map-approval-undo-v1` | `💡️inference/🏃️runtime/🦀️.rs::undo_gis_map_approval` + framework `GisMapApprovalUndoReceiptV1` | same request/receipt split as the seed case |
| `⏸️gis-inference-checkpoint-control-v1` | `💡️inference/🏃️runtime/🦀️.rs` checkpoint routes + `📦️packages/🦀️rust/🚀️bin.rs` | |
| `⛓️inference-wal-chain-v1` | `💡️inference/🧾️wal/🦀️.rs` | depends on the `🧾️inference-wal-proof-v1` fixture (loaded cross-fixture at `📜️script.ts:6671`) |
| `✉️inference-command-v1` | hub inference command decode (`💡️inference/✉️command/`) | |
| `🎯️inference-catalog-selection-v1` | `💡️inference/📇️catalog/🦀️.rs` + `🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs` | |
| `🖥️inference-server-identity-v1` | `SERVER_ID_MAX_BYTES` bound in `💡️inference/🧬️schema/🦀️.rs` | |
| `🗳️gis-map-proposal-approval-v1` | `💡️inference/🏃️runtime` + `🧾️wal` | **only fixture that already `ajv.addSchema`s the real module schema** (`📜️script.ts:7097`) — the template to generalize |
| `🗺️gis-inference-job-v1` | `💡️inference/🏃️runtime` + `🪶️sqlite` job ledger | its `🔣️.json` is itself loaded as shared input by three other fixtures' oracles |

**`🌎️hub/🧪️fixtures/🔣️.json` and `🌎️hub/🧪️fixtures/🧬️.schema.json` at the fixtures root do not exist**
(`git ls-files` returns nothing for either path) — the task brief's premise for these two paths does not hold
in this partition; nothing to classify there.

## 4. What loads each fixture (task 4)

Confirmed exhaustively via `grep -n 'join(repoRoot, "🌎️hub' 🌎️hub/📦️packages/🦀️rust/📜️script.ts'` (117 hits) plus
targeted greps for the three exceptions:

- **Default harness (73 of 76 files):** `🌎️hub/📦️packages/🦀️rust/📜️script.ts` — always an **explicit,
  hand-written fixture-local path** (`join(repoRoot, "🌎️hub/…/<slug>", "🔣️.json"/"🧬️.schema.json")`); there is
  no nearest-parent search or generic binding mechanism. Each fixture is wired to its harness function by a
  literal path string.
- **`🌎️hub/📦️packages/🟦️typescript/🤝️index.test.ts` (Vitest, HUB_E2E-gated):** `🛰️lag-rebootstrap/🪢️canonical-pair`
  (line 38, also independently loaded by Rust `include_str!` in `🛰️lag-rebootstrap/🦀️.rs:770,799`) and
  `🗿️artifact-authority/🧪️fixtures/🧱️artifact-chunk-cas` (line 435-437).
- **`🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript/📜️script.ts`:** the two admin graph schemas (lines 13, 45).

No fixture in this partition uses a taxonomy-style `shared://`/`local://` scheme
(`testFixtureSchemes`) or nearest-parent discovery — every binding is a literal path.

## Decisions / recommended fixes (grouped)

1. **Seed cluster (`💡️inference`, 9 fixtures + `two-author-shell-v1`):** extend
   `💡️inference/🧬️schema/{🔣️.json,🟦️.ts}` with the missing approval/receipt/undo/command/checkpoint/
   catalog-selection/server-identity/proposal-approval/job definitions; point every `prove*Fixture` at that
   module (the `ajv.addSchema` pattern at `📜️script.ts:7097` is the template); keep the `🔣️.json` fixtures as
   example-only data; delete the hand-duplicated `🧬️.schema.json` files next to each fixture once the module
   covers them.
2. **`two-author-shell-v1`:** relocate out of `💡️inference/🧬️schema/` (it should never have been inside the
   module directory) into `🧪️fixtures/`; separately, confirm with the dev whether "two-author GIS Map
   composition" is a live or planned feature — no application implementation exists anywhere in the repo today.
3. **`🔏️trusted-catalog/🧪️fixtures/📤️publication/*` (7 files):** worst case in this partition — the same
   receipt/command contract has **three** independent implementations (private, non-`pub` Rust structs at
   `🔏️trusted-catalog/🦀️.rs:211,222`; a hand-written decode function `trustedBootstrapPublicationReceipt` defined
   *inside the test script itself* at `📜️script.ts:9882`; and six AJV micro-schemas). Consolidate into one
   `pub`-exported module under `🔏️trusted-catalog/🧬️schema/`.
4. **`🌱️creation/📚️operation-v1` + `🧪️transaction-v1` (4 files):** real production `🦀️.rs` already sits in
   `📚️operation-v1/`, but flat alongside `🔣️.json`/`🛑️cancel.schema.json`/`🧬️.schema.json`. Split into
   `🧬️schema/` (implementation + schema) and `🧪️fixtures/` (data only).
5. **`presence-lease-v1`, `presence-normalization-v1`, `artifact-chunk-cas` (3 files):** each has a `🧬️schema/`
   subdirectory *inside* a `🧪️fixtures` leaf — rename away from the reserved `🧬️schema` name (it collides with
   taxonomy's `mutationPayloadSchemaLocation.directoryName`) or relocate the real presence/chunk-CAS contract to
   a genuine scope module and keep only example data here.
6. **Remaining ~45 single-schema fixtures** (`📇️directory`, `📦️packages/🦀️rust`, `📇️native-openable-provider`,
   most of `🔏️trusted-catalog/🧪️fixtures/*`, `🚀️local-bootstrap`, `🛰️lag-rebootstrap`): same remediation as
   the seed cluster — each needs its real Rust/TS owner's `🧬️schema/` module identified (mostly already exists
   at the area level: `📇️directory/🦀️.rs`, `🗿️artifact-authority/🦀️.rs`, `🚀️local-bootstrap/🦀️.rs`, etc.) and
   the hand-duplicated schema migrated there.
7. **Dialect:** migrate all `2020-12` schemas in this partition to `draft-07` per
   `mutationPayloadSchemaAuthority.jsonSchemaDialect`, including the three already-authoritative modules
   (`💡️inference`, `🔐️auth`, `🚀️local-bootstrap`).
8. **`🔏️trusted-catalog/🧬️schema/🔣️bundle.schema.json`:** flag to dev — appears orphaned (zero consumers found
   repo-wide for its `$id` or filename). Confirm before any move/deletion.
9. **`🔐️auth/🧬️schema/🟦️.ts`:** no action — this is the one file in the partition that already matches the
   target pattern exactly: `🤝️index.test.ts:29` imports `parseInviteCapabilityV1` /
   `parseSessionCapabilityV1` / `parseShareCapabilityV1` / `parseSocketGrantCapabilityV1` as **named exports**
   directly from the scope module, no fixture-local schema involved. Use as the reference example when writing
   WP2's resolver/enforcement.

## Coverage — files not (fully) classified, and why

- **Consumers "not individually traced"** (marked in `📊️wp0-hub.json`'s `open` field): the three
  `🚀️local-bootstrap/🧬️schema/*/🔣️.json` authoritative files, `🔐️auth/🧬️schema/🔣️.json`, and about a dozen
  of the ~45 single-schema fixtures in group (6) above were confirmed to be read by *some* function in
  `📜️script.ts` via the bulk `grep -n 'join(repoRoot, "🌎️hub'` sweep (117 matches, all recorded in the
  scratchpad), but the specific `prove*` function name/line was not individually re-verified for every one of
  them by opening the file — given ~140 total files and the confirmed uniformity of the pattern (verified by
  direct reads of 20+ representative cases spanning every sub-area), the remaining ones were classified by this
  confirmed structural pattern rather than each being opened individually. None of the spot-checks contradicted
  the pattern.
- **Non-domain noise excluded:** `🌎️hub/{📦️packages/🟦️typescript,🔨️modules/🛡️admin/📦️packages/🟦️typescript}/{package.json,tsconfig.json,📋️project.json}` — these carry a `$schema` key but point at generic tooling schemas (npm/TS/nx), not a hub domain contract; not included in the 76-file inventory or the JSON deliverable.
- **Two paths named in the task brief that do not exist:** `🌎️hub/🧪️fixtures/🔣️.json` and
  `🌎️hub/🧪️fixtures/🧬️.schema.json` (no schema/fixture pair sits directly at the fixtures root) — confirmed via
  `git ls-files`, nothing to report.
- **`🔏️trusted-catalog/🧬️schema/🔣️bundle.schema.json`'s consumer:** could not be found anywhere in the repo
  (not just this partition) — see decision item 8. This is the one entry where "no consumer" itself, rather
  than "wrong location", is the open question.
- **Framework/os-side duplicates of hub contracts** (the three re-implementations of
  `semio.hub.inference-approval/v1` and related types under `🧰️framework/🛍️products/💻️os/…`) are described
  here for context (they are essential to correctly deciding hub's ownership) but are **out of this partition's
  write scope** — their disposition belongs to the os/framework WP0 auditor(s); flagged for the coordinator to
  cross-reference against `📓️wp0-os-shellhost.md`.
