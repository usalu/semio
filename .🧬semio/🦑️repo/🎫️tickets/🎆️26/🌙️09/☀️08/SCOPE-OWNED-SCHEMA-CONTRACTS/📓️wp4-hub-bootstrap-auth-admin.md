# WP4 — `hub.local-bootstrap`, `hub.lag-rebootstrap`, `hub.auth`, `hub.admin`

Scope-owned schema modules for the four owner directories assigned to this work package. All four
modules are draft-07, carry a `https://semio.tech/schema/hub/<scope>/schema.json` `$id`, and expose
their contracts as PascalCase `$defs` export ids (lower-camel `$defs` keys are private helpers).
Every fixture-owned `*.schema.json` inside these scopes is deleted; the fixture files stay as data.

## A. Scope → export tables

### `hub.local-bootstrap` — `🌎️hub/🚀️local-bootstrap/🧬️schema/🔣️.json`
`$id` `https://semio.tech/schema/hub/local-bootstrap/schema.json`, root `$ref` `LocalBootstrapPipeV1`.

| Export id | Source it was consolidated from | Implementation it mirrors |
|---|---|---|
| `LocalBootstrapProfileV1` | `🧬️schema/🚇️pipe-v1/🔣️.json` `$defs.Profile` | `🌎️hub/🚀️local-bootstrap/🦀️.rs:35` `LocalBootstrapProfileWire` |
| `LocalBootstrapPipeInitializeV1` | `🧬️schema/🚇️pipe-v1/🔣️.json` `$defs.Initialize` | `🦀️.rs:44` `InitializeWire` |
| `LocalBootstrapPipeHelloV1` | `…$defs.Hello` | `🦀️.rs:60` `HelloWire` |
| `LocalBootstrapPipeHelloAcceptedV1` | `…$defs.HelloAccepted` | `🦀️.rs:100` `HelloAcceptedWire` |
| `LocalBootstrapPipeIssueV1` | `…$defs.Issue` | `🦀️.rs:114` `IssueWire` |
| `LocalBootstrapPipeRejectV1` | `…$defs.Reject` | `🦀️.rs:183` `RejectWire` |
| `LocalBootstrapPipeCancelV1` | `…$defs.Cancel` | `🦀️.rs:145` `TerminalInputWire` (`kind: "cancel"`) |
| `LocalBootstrapPipeShutdownV1` | `…$defs.Shutdown` | `🦀️.rs:145` `TerminalInputWire` (`kind: "shutdown"`) |
| `LocalBootstrapPipeV1` | `🧬️schema/🚇️pipe-v1/🔣️.json` root `oneOf` | the seven wire messages above |
| `LocalBootstrapCredentialEnvelopeV1` | `🧬️schema/📨️credential-envelope-v1/🔣️.json` (whole document) | `🦀️.rs:214` `CredentialWire` |
| `LocalBootstrapReadinessV1` | `🧬️schema/🩺️readiness-v1/🔣️.json` (whole document) | `🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:2088` readiness body; `📜️script.ts:1028` readiness gate |
| `LocalBootstrapIdleAdmissionV1` | `🧪️fixtures/⏳️idle-admission-v1/🧬️.schema.json` (whole document — was a fixture-owned schema) | `🦀️.rs` `read_admitted_frame` / `local_bootstrap_idle_listener_survives_until_admission_and_admitted_frame_is_deadline_bounded` |

Private helpers: `hex16`, `hex32`, `boundedId`, `clientClass`, `safeInteger`, `generation`,
`readyComponent`, `authenticated` (the signed-envelope base the five authenticated messages `allOf`).

### `hub.lag-rebootstrap` — `🌎️hub/🛰️lag-rebootstrap/🧬️schema/🔣️.json` (new)
`$id` `https://semio.tech/schema/hub/lag-rebootstrap/schema.json`, root `$ref` `CanonicalCheckpointPairSelectionV1`.

| Export id | Source | Implementation it mirrors |
|---|---|---|
| `CanonicalCheckpointPairSelectionV1` | `🧪️fixtures/🪢️canonical-pair/🧬️.schema.json` `properties.selection` | `🌎️hub/🛰️lag-rebootstrap/🦀️.rs:259` `CanonicalCheckpointPairSelection` (wire header identity) |
| `CanonicalCheckpointPairBaselineV1` | `…$defs.frontierCase` minus the `id`/`accepted` oracle columns | `🦀️.rs:263` `CanonicalCheckpointPairSelection::baseline_frontier`, predicate `canonical_pair_frontier_is_exact` |
| `CanonicalCheckpointPairBlobV1` | `…$defs.part` minus the generator columns `multiplier`/`increment` | `🦀️.rs` `selection.pack` / `selection.spr` (`PublishedArtifactBlob` projection: `length` + `sha256`) |
| `CanonicalCheckpointPairLimitsV1` | `…properties.limits` | `🦀️.rs:17-19` `CANONICAL_CHECKPOINT_PAIR_HEADER_MAX_BYTES`, `_RECORD_BYTES`, `_MAX_RECORDS`, `🦀️.rs:15` `REBOOTSTRAP_DEADLINE_MS`, `AUTHORITY_MAX_PAIR_BYTES` |
| `CanonicalPairPartV1` | new (no prior schema) | `🦀️.rs:395` `enum CanonicalPairPart { Pack = 1, Spr = 2 }` |
| `CanonicalPairTerminalV1` | new (no prior schema) | `🦀️.rs:409` `enum CanonicalPairTerminal { Complete=0 … Deadline=4 }` |
| `RebootstrapTransferLimitsV1` | `🧪️fixtures/🛟️lag-rebootstrap/🔣️.json` (had **no** schema at all) | `🦀️.rs` `REBOOTSTRAP_SCOPE_MAX_BYTES`, `ARTIFACT_BOOTSTRAP_CHUNK_BYTES`, `ARTIFACT_BOOTSTRAP_MAX_TOTAL_BYTES`, `ARTIFACT_BOOTSTRAP_MAX_CHUNKS` + the 1013 `rebootstrap-required` close contract asserted in `lag_rebootstrap_neutral_wire_contract` |

Private helpers: `hash`, `scopeText`, `safeInteger`.

### `hub.auth` — `🌎️hub/🔐️auth/🧬️schema/🔣️.json`
`$id` `https://semio.tech/schema/hub/auth/schema.json` (was `…/hub/auth/capability-v1.json`,
dialect 2020-12), root `$ref` `AuthCapabilityVectorsV1`.

| Export id | Was `$defs` key | Implementation it mirrors |
|---|---|---|
| `SessionCapabilityV1` | `SessionCapability` | `🌎️hub/🔐️auth/🧬️schema/🟦️.ts` `parseSessionCapabilityV1` |
| `ShareCapabilityV1` | `ShareCapability` | `🟦️.ts` `parseShareCapabilityV1` |
| `InviteCapabilityV1` | `InviteCapability` | `🟦️.ts` `parseInviteCapabilityV1` |
| `SocketGrantCapabilityV1` | `SocketGrantCapability` | `🟦️.ts` `parseSocketGrantCapabilityV1` |
| `SocketGrantReceiptV1` | `SocketGrantReceipt` | `🧰️framework/🛍️products/💻️os/🟦️.ts` `parseSocketGrantReceiptV1` |
| `AuthLimitsV1` | `Limits` | hub auth selector/secret/TTL/device/assertion bounds |
| `AuthCapabilityVectorsV1` | the old root document | the cross-language capability vector table in `🧪️fixtures/🔑️capability-v1/🔣️.json` |

Private helpers (renamed from PascalCase so they are not exports): `lowerHex32`, `lowerHex64`,
`boundedText`, `capabilityVector`, `socketGrantVector`, `identityVector`, `revocationVector`,
`auditVector`. Capability lengths are now pinned exactly (`session` 108, `invite`/`socket` 107,
`share` 106) instead of the previous single 107 bound on socket only — verified against the fixture.

### `hub.admin` — `🌎️hub/🔨️modules/🛡️admin/🧬️schema/🔣️.json` (new)
`$id` `https://semio.tech/schema/hub/admin/schema.json`, root `$ref` `AdminEntryGraphV1`.

| Export id | Source (fixture-owned schema, deleted) | Implementation it mirrors |
|---|---|---|
| `AdminEntryGraphV1` | `📦️packages/🟦️typescript/🧪️tests/📐️entry-graph.schema.json` | `📦️packages/🟦️typescript/📜️script.ts` `type AdminEntryGraph` / `verifyAdminEntryGraph` |
| `AdminStylesheetGraphV1` | `📦️packages/🟦️typescript/🧪️tests/🧵️stylesheet-graph.schema.json` | `📜️script.ts` `type AdminStylesheetGraph` / `verifyAdminStylesheetGraph` |

Private helper: `relativePath`.

**Drift found and corrected:** the deleted `🧵️stylesheet-graph.schema.json` pinned
`shared.export` to `"./🎨️.css"` while the fixture `🎨️stylesheet-graph.json` carries `"./🧵️.css"`
(and `verifyAdminStylesheetGraph` resolves the latter). The schema was never actually used to
validate the fixture — the script only read `properties.version.const` and `properties.laws.min/maxItems`
out of it — so the drift was invisible. `AdminStylesheetGraphV1` pins `"./🧵️.css"`, the value the
production code actually resolves, and the script now asserts `"./🎨️.css"` is rejected.

## B. Files created / moved / deleted

Created
- `🌎️hub/🚀️local-bootstrap/🧬️schema/🔣️.json` (consolidated module)
- `🌎️hub/🛰️lag-rebootstrap/🧬️schema/🔣️.json`
- `🌎️hub/🔨️modules/🛡️admin/🧬️schema/🔣️.json`

Deleted
- `🌎️hub/🚀️local-bootstrap/🧬️schema/📨️credential-envelope-v1/` (dir + `🔣️.json`)
- `🌎️hub/🚀️local-bootstrap/🧬️schema/🚇️pipe-v1/` (dir + `🔣️.json`)
- `🌎️hub/🚀️local-bootstrap/🧬️schema/🩺️readiness-v1/` (dir + `🔣️.json`)
- `🌎️hub/🚀️local-bootstrap/🧪️fixtures/⏳️idle-admission-v1/🧬️.schema.json`
- `🌎️hub/🛰️lag-rebootstrap/🧪️fixtures/🪢️canonical-pair/🧬️.schema.json`
- `🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript/🧪️tests/📐️entry-graph.schema.json`
- `🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript/🧪️tests/🧵️stylesheet-graph.schema.json`

Modified
- `🌎️hub/🔐️auth/🧬️schema/🔣️.json` — draft-07, new `$id`, PascalCase export ids, private helpers.
- `🌎️hub/🚀️local-bootstrap/🧪️fixtures/⏳️idle-admission-v1/🔣️.json` — added the `hostile` expectation table (3 rows).
- `🌎️hub/🛰️lag-rebootstrap/🧪️fixtures/🪢️canonical-pair/🔣️.json` — added the `hostile` expectation table (5 rows).
- `🌎️hub/📦️packages/🟦️typescript/🤝️index.test.ts` — module-bound validation (see §C).
- `🌎️hub/📦️packages/🦀️rust/📜️script.ts` — `adminLiveJourneyFixture` idle-admission half (see §C).
- `🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript/📜️script.ts` — `adminSchemaExport` + both verify functions.
- `🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript/📋️project.json` — `namedInputs.default` now includes `{workspaceRoot}/🌎️hub/🔨️modules/🛡️admin/🧬️schema/**/*` so the nx cache keys on the module the build/test oracles now read.

No file was renamed on disk, so **no `include_str!` needed changing**: both
`🌎️hub/🚀️local-bootstrap/🦀️.rs:986,990` (`🧪️fixtures/🚇️pipe-v1/🔣️.json`,
`🧪️fixtures/⏳️idle-admission-v1/🔣️.json`) and `🌎️hub/🛰️lag-rebootstrap/🦀️.rs:770,799,831`
(`🧪️fixtures/🪢️canonical-pair/🔣️.json`, `🧪️fixtures/🛟️lag-rebootstrap/🔣️.json`) point at fixture
**data** files that were kept; only the `🧬️.schema.json` siblings were deleted. Both Rust call
sites deserialize into `serde_json::Value`, so the added `hostile` tables are inert there.
`.vscode/launch.json` and `📋️project.json` reference nx target names (`canonical-pair-check`,
`admin-live-journey-check`), not schema paths, so nothing there needed updating.

## C. Prove-function rewiring

| Prove function | File | Before | After |
|---|---|---|---|
| `adminLiveJourneyFixture` (idle-admission half only) | `🌎️hub/📦️packages/🦀️rust/📜️script.ts` | `Ajv2020.compile(⏳️idle-admission-v1/🧬️.schema.json)` | `hubSchemaExport(repoRoot, "schema://hub.local-bootstrap/LocalBootstrapIdleAdmissionV1")`, plus a `hostile` table drift check (exactly 3 rows, unique ids) and `assertHubFixtureExpectation` per row |
| `canonical checkpoint pair neutral contract` | `🌎️hub/📦️packages/🟦️typescript/🤝️index.test.ts` | `Ajv2020.compile(🪢️canonical-pair/🧬️.schema.json)` over the whole fixture file, one `{…fixture, locator}` negative | module exports `CanonicalCheckpointPairSelectionV1` / `…BaselineV1` / `…BlobV1` / `…LimitsV1` over the contract-shaped members; explicit oracle-table assertions (10 frontier rows, `Set` size 10, 5 hostile rows, `Set` size 5) and per-row `stage`/`result` enforcement |
| `hub harness quick contract` (local bootstrap) | `🤝️index.test.ts` | three `Ajv2020.compile` calls over the three per-contract schema directories | one `hubSchemaExport(root, "🌎️hub/🚀️local-bootstrap/🧬️schema/🔣️.json")` binder; seven exports validated against the fixture members; the same ten negatives now run against the module exports |
| `validates typed auth capabilities …` | `🤝️index.test.ts` | `Ajv2020.compile(🔐️auth/🧬️schema/🔣️.json)` — **would have hard-failed** on the draft-07 migration | `AuthCapabilityVectorsV1` plus per-export validation of the six other exports, and the four hostile socket capabilities now asserted rejected by `SocketGrantCapabilityV1` (previously only by the TS `parse*`) |
| `verifyAdminEntryGraph` | `🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript/📜️script.ts` | read the fixture-local schema as a **data structure** (`properties.version.const`, `properties.laws.minItems/maxItems`, `required`) — it never ran a validator | `adminSchemaExport(root, "AdminEntryGraphV1")` (draft-07 `ajv`), accepts the fixture, rejects an unknown member and a short law list, and asserts the exact four-law literal set |
| `verifyAdminStylesheetGraph` | same file | same data-structure reading | `adminSchemaExport(root, "AdminStylesheetGraphV1")`, accepts the fixture, rejects an unknown member and the stale `"./🎨️.css"` shared export, asserts the exact five-law literal set |

The envelope shapes of the two deleted fixture wrappers (`schema` const,
`semio.hub.canonical-checkpoint-pair.fixture/v1`, the `frontierCases` array bound, the
`maximumBytes`-style ceilings that only describe the fixture file) were **not** recreated anywhere.
Their validation power is replaced by (a) module-export validation of the contract-shaped members
and (b) explicit literal/`Set`-size/exhaustive-enumeration assertions on the oracle tables, listed
in the table above.

A local `hubSchemaExport(root, modulePath)` helper was added to `🤝️index.test.ts` rather than
importing the one in `🌎️hub/📦️packages/🦀️rust/📜️script.ts`: that script's import graph pulls
playwright, the MCP binary resolver and the whole os/directory TS surface, which the quick vitest
lane must not load (a direct `await import` of it did not finish inside 120 s).

## D. Verification (real output)

### D.1 Four modules, `ajv` draft-07 `new Ajv({strict:true,allErrors:true})` + `addSchema` + `getSchema`

```
$ cd /Users/ueli/Documents/semio && bun -e '<four-module load + fixture-member validation>'
hub.local-bootstrap | https://semio.tech/schema/hub/local-bootstrap/schema.json | exports: LocalBootstrapProfileV1, LocalBootstrapPipeInitializeV1, LocalBootstrapPipeHelloV1, LocalBootstrapPipeHelloAcceptedV1, LocalBootstrapPipeIssueV1, LocalBootstrapPipeRejectV1, LocalBootstrapPipeCancelV1, LocalBootstrapPipeShutdownV1, LocalBootstrapPipeV1, LocalBootstrapCredentialEnvelopeV1, LocalBootstrapReadinessV1, LocalBootstrapIdleAdmissionV1
hub.lag-rebootstrap | https://semio.tech/schema/hub/lag-rebootstrap/schema.json | exports: CanonicalCheckpointPairBaselineV1, CanonicalCheckpointPairBlobV1, CanonicalCheckpointPairSelectionV1, CanonicalCheckpointPairLimitsV1, CanonicalPairPartV1, CanonicalPairTerminalV1, RebootstrapTransferLimitsV1
hub.auth | https://semio.tech/schema/hub/auth/schema.json | exports: InviteCapabilityV1, SessionCapabilityV1, ShareCapabilityV1, SocketGrantCapabilityV1, SocketGrantReceiptV1, AuthLimitsV1, AuthCapabilityVectorsV1
hub.admin | https://semio.tech/schema/hub/admin/schema.json | exports: AdminEntryGraphV1, AdminStylesheetGraphV1
---- fixture members ----
accepted contract members: 34/34
  rejected selection-unknown-member -> unknown-member
  rejected selection-short-descriptor-digest -> digest-not-sha256
  rejected selection-empty-document -> empty-document-id
  rejected baseline-negative-ordinal -> ordinal-not-safe-unsigned
  rejected baseline-unknown-member -> unknown-member
  rejected unknown-member -> unknown-member
  rejected idle-not-past-deadline -> idle-not-past-exchange-deadline
  rejected frame-below-length-prefix -> frame-shorter-than-length-prefix
rejected hostile members: 12/12
```

The 34 accepted members cover every fixture in the four scopes: `🚇️pipe-v1` (initialize/hello/issue
through the union, both profiles, credential, all three readiness bodies), `⏳️idle-admission-v1`,
`🪢️canonical-pair` (selection, limits, both blob projections, all 10 frontier rows),
`🛟️lag-rebootstrap` (transfer-limit projection), `🔑️capability-v1` (vectors + the four capability
strings + receipt + limits), and both admin graph fixtures.

### D.2 The shared script's own `//#region 🧬️Scope-owned schema resolution`, executed verbatim

The region was extracted from `🌎️hub/📦️packages/🦀️rust/📜️script.ts` as-is and run against the
repo root (the surrounding script cannot be imported in isolation — see §C):

```
[DEBUG] scopes: hub.admin,hub.artifact-authority,hub.artifact-authority.creation,hub.artifact-authority.native-openable-provider,hub.artifact-authority.trusted-catalog,hub.auth,hub.directory,hub.inference,hub.lag-rebootstrap,hub.local-bootstrap
idle-admission contract: accepted
hostile enforced: unknown-member unknown-member
hostile enforced: idle-not-past-deadline idle-not-past-exchange-deadline
hostile enforced: frame-below-length-prefix frame-shorter-than-length-prefix
frame oracle: true
resolved schema://hub.local-bootstrap/LocalBootstrapPipeV1
resolved schema://hub.local-bootstrap/LocalBootstrapCredentialEnvelopeV1
resolved schema://hub.local-bootstrap/LocalBootstrapReadinessV1
resolved schema://hub.lag-rebootstrap/CanonicalCheckpointPairSelectionV1
resolved schema://hub.lag-rebootstrap/RebootstrapTransferLimitsV1
resolved schema://hub.auth/AuthCapabilityVectorsV1
resolved schema://hub.auth/SessionCapabilityV1
resolved schema://hub.admin/AdminEntryGraphV1
resolved schema://hub.admin/AdminStylesheetGraphV1
```

This is the exact code path `adminLiveJourneyFixture` now takes for the idle-admission fixture,
including `assertHubFixtureExpectation` over the new `hostile` table.

### D.3 `os-hub-ts` lane (`bun ./📜️script.ts test long`)

Final state — the whole file is green:

```
 RUN  v4.1.10 /Users/ueli/Documents/semio/🌎️hub/📦️packages/🟦️typescript


 Test Files  1 passed (1)
      Tests  11 passed | 1 skipped (12)
   Start at  16:25:00
   Duration  21.24s (transform 14.18s, setup 0ms, import 15.77s, tests 4.70s, environment 0ms)
```

(The single skipped case is the `HUB_E2E=1`-gated real-binary journey.) An earlier `test quick` run
during the rewiring showed the same 11 passes with one unrelated failure in
`🌎️hub/🗿️artifact-authority` — `ENOENT … 🧪️fixtures/🧱️artifact-chunk-cas/🧬️schema/🔣️.json` —
while that partition's worker was mid-move; it has since landed and the lane is clean.
The four cases this work package owns are `canonical checkpoint pair neutral contract`,
`hub harness quick contract > validates local bootstrap, one-shot credential, and readiness schemas
with independent HMAC`, `… > validates typed auth capabilities and independently recomputes the
socket grant with AJV and WebCrypto`, and the trusted-catalog case.
`test quick` (30 s budget) is at ~22 s and does get killed under concurrent load; `test long` is the
reliable lane on a loaded box.

### D.4 `os-hub-admin` oracles (`bun ./📜️script.ts test`)

```
[DEBUG] hub admin entry graph oracle: 4 laws, 1 HTML module entry, 1 package export, 1125 entry bytes
[DEBUG] hub admin stylesheet graph oracle: 5 laws, 1 canonical import, 2 Tailwind sources, 3 resolved shared imports across 4 stylesheets
```

Both rewired oracles run to completion against `hub.admin`. The vitest step that follows them fails
for an unrelated reason in another partition:

```
 FAIL  |@semio-tech/hub-admin| 🛡️admin.test.tsx [ 🛡️admin.test.tsx ]
Error: Failed to resolve import "../../🔨️modules/🎠️kernel/🧫️fixtures/📐️source-watch.schema.json" from "../../../../../🧰️framework/📦️packages/🟦️typescript/🟦️.ts". Does the file exist?
 Test Files  1 failed | 1 passed (2)
      Tests  3 passed (3)
```

A later re-run of both lanes aborted before vitest with
`error: Cannot find module '../📃️pageSchema/🟦️.ts' from '🧰️framework/🔨️modules/🎭️actor/📤️return/🟦️.ts'`
— `🧰️framework/🔨️modules/🎭️actor/📃️pageSchema` has been renamed to `📃️page` by a concurrent
worker and `📤️return/🟦️.ts` still imports the old path. Neither failure touches this partition.

### D.5 Layout gates

```
$ cd /Users/ueli/Documents/semio && git ls-files 🌎️hub/🚀️local-bootstrap 🌎️hub/🛰️lag-rebootstrap 🌎️hub/🔐️auth 🌎️hub/🔨️modules/🛡️admin | grep -E 'schema\.json$'
exit=1        # no matches

$ ls 🌎️hub/🚀️local-bootstrap/🧬️schema/
🔣️.json
```

```
$ git status --porcelain 🌎️hub/🚀️local-bootstrap 🌎️hub/🛰️lag-rebootstrap 🌎️hub/🔐️auth 🌎️hub/🔨️modules/🛡️admin
M  🌎️hub/🔐️auth/🧬️schema/🔣️.json
M  🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript/📜️script.ts
D  🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript/🧪️tests/📐️entry-graph.schema.json
D  🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript/🧪️tests/🧵️stylesheet-graph.schema.json
A  🌎️hub/🔨️modules/🛡️admin/🧬️schema/🔣️.json
M  🌎️hub/🚀️local-bootstrap/🧪️fixtures/⏳️idle-admission-v1/🔣️.json
D  🌎️hub/🚀️local-bootstrap/🧪️fixtures/⏳️idle-admission-v1/🧬️.schema.json
D  🌎️hub/🚀️local-bootstrap/🧬️schema/📨️credential-envelope-v1/🔣️.json
A  🌎️hub/🚀️local-bootstrap/🧬️schema/🔣️.json
D  🌎️hub/🚀️local-bootstrap/🧬️schema/🚇️pipe-v1/🔣️.json
D  🌎️hub/🚀️local-bootstrap/🧬️schema/🩺️readiness-v1/🔣️.json
M  🌎️hub/🛰️lag-rebootstrap/🧪️fixtures/🪢️canonical-pair/🔣️.json
D  🌎️hub/🛰️lag-rebootstrap/🧪️fixtures/🪢️canonical-pair/🧬️.schema.json
A  🌎️hub/🛰️lag-rebootstrap/🧬️schema/🔣️.json
```

### D.6 What needs a build (not run here)

Per the work-package constraint, no `cargo` was run at all. These gates cover this partition and
still need a compiled binary or a browser:

- `bun 🌎️hub/📦️packages/🦀️rust/📜️script.ts canonical-pair-check` — `cargo test --lib canonical_pair`,
  `cargo test --bin os-hub canonical_pair_route`, `cargo check --all-features`, plus the `os-hub-ts`
  quick case that D.3 already covers.
- `bun 🌎️hub/📦️packages/🦀️rust/📜️script.ts admin-live-journey-check` — the only caller of the
  rewired `adminLiveJourneyFixture`; needs `cargo test --lib`, `cargo build --bin os-hub`, the built
  admin SPA and a headless Chromium. Its pure-TS half is executed verbatim in D.2.
- `bun 🌎️hub/📦️packages/🦀️rust/📜️script.ts admin-backend-check` — cargo laws plus `os-hub-admin:test`
  (the admin half of which is D.4).
- `cargo test -p <hub crate> lag_rebootstrap_neutral_wire_contract` /
  `canonical_pair_baseline_admits_exact_genesis_or_edited_before_pair_allocation` — both read the
  fixture data files this WP left in place and only gained inert extra keys.

## E. Cross-partition requests

1. **`🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:5269-5277`**
   (`CanonicalBootstrapFolderMirrorCheckScript`) reads the now-deleted
   `🌎️hub/🛰️lag-rebootstrap/🧪️fixtures/🪢️canonical-pair/🧬️.schema.json` and will throw `ENOENT`.
   This is the only remaining reader outside `🌎️hub`. Exact change needed — replace

   ```ts
   const canonicalPairSchema = JSON.parse(readFileSync(join(canonicalPairRoot, "🧬️.schema.json"), "utf8"));
   …
   const validateCanonicalPair = ajv.compile(canonicalPairSchema);
   if (!validateCanonicalPair(canonicalPairCorpus)) throw new Error(`canonical pair corpus: ${JSON.stringify(validateCanonicalPair.errors)}`);
   ```

   with (the surrounding `ajv` there is `Ajv2020`; the module is draft-07, so it needs its own
   plain `ajv` instance)

   ```ts
   const { default: Ajv } = await import("ajv");
   const lagModule = JSON.parse(readFileSync(join(this.repoRoot, "🌎️hub/🛰️lag-rebootstrap/🧬️schema/🔣️.json"), "utf8")) as { $id: string };
   const lagAjv = new Ajv({ strict: true, allErrors: true });
   lagAjv.addSchema(lagModule);
   const validateCanonicalPairSelection = lagAjv.getSchema(`${lagModule.$id}#/$defs/CanonicalCheckpointPairSelectionV1`)!;
   const validateCanonicalPairBaseline = lagAjv.getSchema(`${lagModule.$id}#/$defs/CanonicalCheckpointPairBaselineV1`)!;
   if (!validateCanonicalPairSelection(canonicalPairCorpus.selection)) throw new Error(`canonical pair selection: ${JSON.stringify(validateCanonicalPairSelection.errors)}`);
   for (const { id, accepted, ...frontier } of canonicalPairCorpus.frontierCases)
     if (!validateCanonicalPairBaseline(frontier)) throw new Error(`canonical pair frontier ${id}: ${JSON.stringify(validateCanonicalPairBaseline.errors)}`);
   ```

   and widen the `canonicalPairCorpus` cast to include `selection.baseline`. The corpus type there
   already declares `selection: { documentId: string }` and `frontierCases`.

2. **`🌎️hub/📦️packages/🦀️rust/📜️script.ts:2905`** — after the directory worker's own rewiring of
   `adminLiveJourneyFixture`, the local `const Ajv2020 = (await import("ajv/dist/2020.js")).default;`
   at the top of that function is now unused (both halves resolve through `hubSchemaExport`). It sits
   in the directory partition's lines; it should be deleted by whoever finishes that half.

3. **`🌎️hub/📦️packages/🟦️typescript/📋️project.json`** (`os-hub-ts`, shared hub packages dir, not in
   this partition) — `namedInputs.default` is `["{projectRoot}/**/*", "{workspaceRoot}/🌎️hub/📦️packages/🦀️rust/**/*.rs"]`.
   `🤝️index.test.ts` now reads three scope modules outside the project root. `test`/`test-quick` are
   `cache: false`, so nothing is wrong today, but if caching is ever enabled the input list needs
   `{workspaceRoot}/🌎️hub/**/🧬️schema/🔣️.json`.

4. **WP2 taxonomy/catalog** — `🌎️hub/🛰️lag-rebootstrap/🧬️schema/` and
   `🌎️hub/🔨️modules/🛡️admin/🧬️schema/` are new module directories; the derived
   `🔣️schema-catalog.json` and the `schemaScopeOwnerLevels` entry for hub area modules and hub
   `🔨️modules` members must include them.

## F. Open questions

1. **`ArtifactFrontier` ownership.** `hub.lag-rebootstrap` exports
   `CanonicalCheckpointPairBaselineV1` rather than an `ArtifactFrontierV1`, because the Rust
   `ArtifactFrontier` / `PublishedArtifactBlob` / `RebootstrapRequired` structs live in
   `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs` (scope `os.directory`), not
   here. Once `os.directory` publishes its module these two exports should become cross-scope
   `$ref`s to `https://semio.tech/schema/os/directory/schema.json#/$defs/ArtifactFrontierV1` and
   `…/PublishedArtifactBlobV1`, and `CanonicalCheckpointPairBaselineV1`/`CanonicalCheckpointPairBlobV1`
   should be retired. I did not duplicate an `ArtifactFrontierV1` export id here to avoid two owners
   for one shape.
2. **`🛟️lag-rebootstrap` fixture `control`.** Its `control.control` member is a
   `DirectoryStreamMessage::RebootstrapRequired` (byte-array hashes, DSL-value encoding) owned by
   `os.directory`, so `hub.lag-rebootstrap` exports only the transfer bounds
   (`RebootstrapTransferLimitsV1`) it genuinely owns. The `control` member is currently validated
   only by the Rust typed decode in `lag_rebootstrap_neutral_wire_contract`; it should get a
   cross-scope `$ref` once `os.directory` ships its module.
3. **`🟦️.ts` / `🛰️.proto` / `🔗️.graphql` siblings.** Only `hub.auth` has a `🟦️.ts`; the three other
   scopes have `🔣️.json` alone. WP4's brief covered the JSON Schema modules only. If the layout gate
   requires a TS parse surface per module, `hub.local-bootstrap` (12 exports),
   `hub.lag-rebootstrap` (7) and `hub.admin` (2) need `parse<Export>` functions written next.
4. **`CanonicalPairPartV1` / `CanonicalPairTerminalV1`** have no JSON fixture today (they only exist
   as wire discriminant bytes in the Rust encoder). They are exported so the wire framing has a
   named contract, but nothing validates against them yet.
