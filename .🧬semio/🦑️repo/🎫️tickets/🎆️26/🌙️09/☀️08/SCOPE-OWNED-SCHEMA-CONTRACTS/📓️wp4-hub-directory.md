# WP4 — `hub.directory` scope-owned schema module

Owner scope: `hub.directory` → `🌎️hub/📇️directory/🧬️schema/🔣️.json`
`$id`: `https://semio.tech/schema/hub/directory/schema.json`
Dialect: `http://json-schema.org/draft-07/schema#` (no `prefixItems`, no `unevaluated*`, no 2020-12 `$schema`).
Root document is exactly `{ $schema, $id, $defs }`; no root `$ref`, no envelope shape anywhere in the module.

## A. Scope → export table

30 exports (PascalCase `$defs` keys). 25 lower-camel `$defs` keys are private helpers and are not exports:
`id, identifier, text, hash, nonzeroHash, requestId, safeInteger, positiveSafeInteger, spaceKind, spaceVisibility, spaceRole, documentOwner, documentFrontier, publicSpace, memberSpace, publicDocument, scopedDocumentDescriptor, spaceDocumentView, spaceMember, spaceInvite, directoryEventHlc, directoryEventActor, directoryEvent, directoryCommandResult, presenceBoundedText`.

| Scope id | Export id | Source fixture the contract was lifted out of | Implementation it mirrors |
|---|---|---|---|
| hub.directory | `DirectoryDocumentScopeV1` | `🔏️document-execution-target-lease-v1`, `🪪️execution-target-relay-v1` (`intent.scope`) | `DocumentScope` in `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🔣️.json`; `DocumentScope` in `🌎️hub/📇️directory/🦀️.rs` |
| hub.directory | `DirectorySocketScopeV1` | `🔌️scoped-socket-revocation-v1` (`scope`, `vectors[].grantScope`, `vectors[].urlScope`), `🛂️admin-presence-target-recovery-v1` (`scope`) | socket URL/grant scope in `🌎️hub/📦️packages/🦀️rust/🚀️bin.rs` (`socket_binding_reads_are_exact_id_generation_selector_scope_and_status`) |
| hub.directory | `DirectorySocketMessageRoutingV1` | `🔌️scoped-socket-revocation-v1` (`vectors[].message`) | `DirectoryStreamMessage` (os directory module `$defs`), `DirectoryStreamMessage::…` in `🌎️hub/📇️directory/🦀️.rs` |
| hub.directory | `DocumentOpenIntentV1` | `🔏️document-execution-target-lease-v1` (`intent`), `🪪️execution-target-relay-v1` (`intent`) | `parseDocumentOpenIntentV1` in `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts`; wire id `semio.hub.document-open-intent/v1` |
| hub.directory | `SocketGrantReceiptV1` | `🔏️document-execution-target-lease-v1` (`socketGrant`) | `parseSocketGrantReceiptV1` / `socketGrantProtocolsV1` in `🧰️framework/🛍️products/💻️os/🟦️.ts`; wire id `semio.hub.socket-grant/v1` |
| hub.directory | `ExecutionTargetAssetRouteV1` | `🪪️execution-target-relay-v1` (`definitions/admittedRoute`, applied to `routes[]`) | `localRelayUpstreamPath` in `🌎️hub/📦️packages/🦀️rust/📜️script.ts` + `execution_target_asset_routes_…` law in `🚀️bin.rs` |
| hub.directory | `DirectoryCommandReceiptV1` | `🧾️command-receipt-v1` (`receipts[].receipt`) | `parseDirectoryCommandReceiptV1` + `DirectoryCommandReceiptV1` (os directory `🟦️.ts` / `🔣️.json`); wire id `semio.directory.command-receipt.v1` |
| hub.directory | `DirectorySessionBindingV1` | `📅️event-page-route-v1` (`session`) | event-page session binding in `🌎️hub/📦️packages/🦀️rust/🚀️bin.rs` (domain `semio/hub/directory-event-page/session-binding/v1`) |
| hub.directory | `DirectorySpaceSessionBindingV1` | `🏘️space-administration-page-v1` (`session`) | space-administration session binding (domain `semio/hub/directory-space-administration/session-binding/v1`) |
| hub.directory | `SpaceAdministrationSpaceV1` | `🏘️space-administration-page-v1` (`space`) | `MemberSpaceViewV1` / `DirectorySpaceAdministrationPageV1` (os directory `$defs`) |
| hub.directory | `SpaceAdministrationMemberRowV1` | `🏘️space-administration-page-v1` (`members[]`) | `DirectorySpaceAdministrationMemberRowV1` (os directory `$defs`) |
| hub.directory | `SpaceAdministrationInviteRowV1` | `🏘️space-administration-page-v1` (`invites[]`) | `DirectorySpaceAdministrationInviteRowV1` (os directory `$defs`) |
| hub.directory | `PublicSpaceDetailV1` | `🏛️public-space-detail-v1` (`positives.anonymous`, `positives.publicNonmember`) | `PublicSpaceViewV1` / `PublicDocumentCatalogEntryV1` (os directory `$defs`); `space_public_boundary_*` laws in `🚀️bin.rs` |
| hub.directory | `MemberSpaceDetailV1` | `🏛️public-space-detail-v1` (`positives.member`) | `MemberSpaceViewV1` + `DocumentView` (os directory `$defs`) |
| hub.directory | `AuthorSpaceDetailV1` | `🏛️public-space-detail-v1` (`positives.author`) | `MemberSpaceViewV1` + `InviteView` (os directory `$defs`) |
| hub.directory | `DirectoryDocumentDescriptorV1` | `🚻️space-journey-v1` (`document`) | `DocumentDescriptor` (os directory `$defs`); `DocumentDescriptor` in `🌎️hub/📇️directory/🦀️.rs` |
| hub.directory | `DirectoryEventPageCursorV1` | `🌐️directory-home-browser-process-v1` (`pages.*`) | `DirectoryEventPageV1` (os directory `$defs`); event-page cursor/receipt in `🚀️bin.rs` |
| hub.directory | `DirectoryProfileV1` | `🚻️space-journey-v1` (`profiles.a|b`), `🌐️directory-home-browser-process-v1` (`profiles.a|b`), `🚶️admin-live-journey-v1` (`profile`) | `LocalProfile` in `🌎️hub/📦️packages/🦀️rust/📜️script.ts`, local-bootstrap profile in `🌎️hub/🚀️local-bootstrap` |
| hub.directory | `LocalizedTextV1` | `🚻️space-journey-v1` (`steps[].labels`, `skips[].labels`), `🔏️document-execution-target-lease-v1` (`expected.status.*`) | EN/DE status bundle consumed by `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer` |
| hub.directory | `AdminPrincipalV1` | `🎯️admin-intent-v1` (`principal`) | admin REST principal in `🚀️bin.rs` (peer class `admin-rest`) |
| hub.directory | `AdminCreateSpaceIntentV1` | `🎯️admin-intent-v1` (`createSpaceIntent`), `🚶️admin-live-journey-v1` (`mutation`) | `AdminIntentV1` create-space variant (os directory `$defs`) |
| hub.directory | `AdminOperationIntentV1` | `🚶️admin-live-journey-v1` (`operation`) | `AdminIntentV1` rebuild-directory-projections variant (os directory `$defs`) |
| hub.directory | `AdminActorV1` | `🎯️admin-intent-v1` (`expectedActor`) | `DirectoryActor` / `DirectoryActorKind` (os directory `$defs`) |
| hub.directory | `AdminRecordedConnectionV1` | `🎯️admin-intent-v1` (`recordedConnection.stored`) | `AdminRecordedConnectionV1` (os directory `$defs`) |
| hub.directory | `AdminIntentOutcomeV1` | `🎯️admin-intent-v1` (`outcomes.durableRevoke`, `outcomes.ephemeralKick`) | `AdminIntentOutcomeV1` (os directory `$defs`) |
| hub.directory | `InviteRedemptionCallV1` | `🎟️invite-redemption-transaction-v1` (`vectors[].calls[]`) | server-derived redemption call in `🌎️hub/📇️directory/🦀️.rs` / `🪶️sqlite/🦀️.rs` — the client supplies no capability, space, role or event id |
| hub.directory | `InviteAcceptanceMarkerV1` | `🎟️invite-redemption-transaction-v1` (`vectors[].expected.marker`) | invite acceptance marker (`accepted_at` / `accepted_event_id`) in `🌎️hub/📇️directory/🦀️.rs` + `🪶️sqlite/🦀️.rs` |
| hub.directory | `PresenceAdmissionV1` | `🪪️presence-normalization-v1` (`vectors[].admission`) | presence admission normalization in `🌎️hub/📦️packages/🦀️rust/🚀️bin.rs`; `ArtifactPresencePeer` in `🧰️framework/🔨️modules/📡️replication/🟦️.ts` |
| hub.directory | `PresenceLeaseOperationV1` | `👥️presence-lease-v1` (`vectors[].operations[]`) | presence lease install/refresh/tick/close/fill/restart in `🚀️bin.rs` (`presence_lease_*` laws) |
| hub.directory | `PresenceScopeSummaryV1` | `👥️presence-lease-v1` (`vectors[].expected.final[]`) | presence roster summary (`PRESENCE_ROSTER_MAXIMUM_ITEMS`) in `🚀️bin.rs` / os kernel |

### Fixtures that yielded no export (pure oracle tables)

Six of the 21 fixture schemas were 100 % envelope: a `schema` const plus decision tables whose rows are
oracle expectations, not wire values. Per the execution contract §B ("wrapper schemas … are NOT contracts")
nothing was recreated for them; the validation power moved into explicit assertions in the prove function
(exact literal comparison of the declared bounds, `Object.keys(...).sort()` envelope equality, exact
inventory strings, `Set` size uniqueness checks, exhaustive enum enumeration per row).

- `📣️ordered-append-broadcast-v1` — exact 4-case id list, `maximumEventsPerDecision === 2`, safe-integer sequences.
- `🛡️command-authority-v1` — capacity literals, 4/5/4 table sizes, exact invite-scope id list, per-row enum closure.
- `🔐️share-issuance-atomicity` — 7 cases, exact 7-entry `sourceHostiles` string, per-row enum closure.
- `🏛️retained-short-admin` — capacity bound, 15/21/6/25 table sizes, uniqueness, commit-outcome enum closure and 0/1 counters.
- `🌐️directory-message-authority-v1` — 4/6 table sizes, uniqueness, `close ∈ {0,4401}`, message JSON byte bounds.
- `🏛️admin-directory-authority-v1` — 5/8/15 table sizes, exact 15-name binding inventory in order, principal-union `^(user|session|space|membership|share):` pattern per key.

## B. Files created / moved / deleted

Created
- `🌎️hub/📇️directory/🧬️schema/🔣️.json` (new owner module, draft-07, 30 exports + 25 private helpers)

Deleted (21 fixture-owned schemas — the contract moved into the module, the fixture directories now hold data only)
1. `🌎️hub/📇️directory/🧪️tests/🏛️retained-short-admin/🧬️.schema.json`
2. `🌎️hub/📇️directory/🧪️tests/🔐️share-issuance-atomicity/🧬️.schema.json`
3. `🌎️hub/📇️directory/🧫️fixtures/🌐️directory-home-browser-process-v1/🧬️.schema.json`
4. `🌎️hub/📇️directory/🧫️fixtures/🎟️invite-redemption-transaction-v1/🧬️.schema.json`
5. `🌎️hub/📇️directory/🧫️fixtures/🎯️admin-intent-v1/🧬️.schema.json`
6. `🌎️hub/📇️directory/🧫️fixtures/🏛️public-space-detail-v1/🧬️.schema.json`
7. `🌎️hub/📇️directory/🧫️fixtures/🚶️admin-live-journey-v1/🧬️.schema.json`
8. `🌎️hub/📇️directory/🧫️fixtures/🛡️command-authority-v1/🧬️.schema.json`
9. `🌎️hub/📇️directory/🧫️fixtures/🧪️fixtures/📣️ordered-append-broadcast-v1/🧬️.schema.json`
10. `🌎️hub/📇️directory/🧫️fixtures/🧪️fixtures/🔌️scoped-socket-revocation-v1/🧬️.schema.json`
11. `🌎️hub/🧪️fixtures/📇️directory/🏘️space-administration-page-v1/🧬️.schema.json`
12. `🌎️hub/🧪️fixtures/📇️directory/📅️event-page-route-v1/🧬️.schema.json`
13. `🌎️hub/🧪️fixtures/📇️directory/🔏️document-execution-target-lease-v1/🧬️.schema.json`
14. `🌎️hub/🧪️fixtures/📇️directory/🚻️space-journey-v1/🧬️.schema.json`
15. `🌎️hub/🧪️fixtures/📇️directory/🧾️command-receipt-v1/🧬️.schema.json`
16. `🌎️hub/🧪️fixtures/🪪️execution-target-relay-v1/🧬️.schema.json`
17. `🌎️hub/📦️packages/🦀️rust/🧪️fixtures/🌐️directory-message-authority-v1/🧬️.schema.json`
18. `🌎️hub/📦️packages/🦀️rust/🧪️fixtures/🏛️admin-directory-authority-v1/🧬️.schema.json`
19. `🌎️hub/📦️packages/🦀️rust/🧪️fixtures/🛂️admin-presence-target-recovery-v1/🧬️.schema.json`
20. `🌎️hub/📦️packages/🦀️rust/🧪️fixtures/👥️presence-lease-v1/🧬️schema/` (whole illegally nested module directory, `rm -r`; `🧪️fixture/🔣️.json` kept as data)
21. `🌎️hub/📦️packages/🦀️rust/🧪️fixtures/🪪️presence-normalization-v1/🧬️schema/` (same violation, same treatment; `🧪️fixture/🔣️.json` kept as data)

Edited
- `🌎️hub/📦️packages/🦀️rust/📜️script.ts` — 19 prove/run sites rewired (see §C); no new imports; `HUB_SCHEMA_SCOPES` untouched (`hub.directory` was already registered).
- `🌎️hub/📇️directory/🧫️fixtures/🏛️public-space-detail-v1/🔣️.json` — each of the 21 `hostileMutations[]` entries gained `"stage": "contract", "result": "rejected", "code": "<space|detail|document>-unknown-field"`. Nothing else in the file changed (line-anchored edit, no reformat).
- `🌎️hub/📇️directory/🧫️fixtures/🎯️admin-intent-v1/🧪️oracle/🟦️.ts` — now loads the owner module with draft-07 `ajv` and resolves five exports instead of compiling the deleted fixture-local wrapper; all pre-existing domain assertions kept, envelope assertions added.

No `include_str!`, `📋️project.json`, `.vscode/launch.json` or `package.json` reference pointed at any deleted
path (`grep -rn 'include_str!' 🌎️hub/📇️directory 🌎️hub/📦️packages/🦀️rust` reaches only `🔣️.json` data files;
a repo-wide grep for the five affected fixture directory names found only `📜️script.ts` and `🦀️.rs` data
reads plus this ticket's own WP0 JSON).

## C. Prove-function rewiring

`hubSchemaExport(repoRoot, "schema://hub.directory/<ExportId>")` replaces every
`new Ajv2020(...).compile(readFileSync(join(root, "🧬️.schema.json")))` below. No fixture-local schema path is
read by any of them any more.

| Function / `run()` | Module exports now used | Envelope replaced by |
|---|---|---|
| `orderedDirectoryPublicationOracle` | — | exact 4-id case inventory, `maximumEventsPerDecision` literal, safe-integer sequence bounds |
| `adminLiveJourneyFixture` | `DirectoryProfileV1`, `AdminCreateSpaceIntentV1`, `AdminOperationIntentV1` | schema const, `responseBytes`/journey/poll bounds, exactly-two-languages |
| `proveScopedDirectorySocketRevocationFixture` | `DirectorySocketScopeV1`, `DirectorySocketMessageRoutingV1` | envelope key set, three limit literals, 12–32 unique vectors, 3 distinct client closes; hostiles are now contract-stage scope/message mutations |
| `AdminDirectoryAuthorityCheckScript.run` | — | envelope key set, 5/8/15 sizes, ordered 15-name binding inventory, per-row enum closure, principal-key pattern |
| `proveShareIssuanceAtomicity` | — | envelope key set, 7 unique cases, exact `sourceHostiles` string, per-row enum closure |
| `proveRetainedShortAdmin` | — | envelope key set, capacity bound, 15/21/6/25 sizes, commit-outcome enum closure + 0/1 counters |
| `DirectoryMessageAuthorityCheckScript.run` | — | envelope key set, 4/6 sizes, uniqueness, close-code/keys enums, message JSON byte bounds |
| `proveExecutionTargetRelay` | `DocumentOpenIntentV1`, `ExecutionTargetAssetRouteV1` | envelope key set, route/response inventories, exhaustive fence oracle table, response byte/delay/status bounds |
| `proveExecutionTargetLeaseCorpus` | `DocumentOpenIntentV1`, `SocketGrantReceiptV1`, `LocalizedTextV1` | envelope key set, clock/origin/hex shape, full `expected` key set + every bound literal, renderer-claim closure, asset-path derivation from `openPlanPath`, status/statusRoles inventory, rotation distinctness, hostile stage closure. `leaseFields` validation moved from the deleted fixture schema to the canonical os module export `DocumentExecutionTargetLeaseFieldsV1` |
| `proveDirectoryEventPageRouteV1` | `DirectorySessionBindingV1` | envelope key set, four limit literals, exact 5-vector and 5-hostile inventories, 12 query cases with enum/bound closure; the old whole-fixture unknown-field hostile became three contract-stage session mutations (unknown field, concatenated binding digest, generation 0) |
| `proveDirectoryHomeBrowserProcessSource` | `DirectoryProfileV1`, `DirectoryEventPageCursorV1` | envelope key set, guest-boundary literals, Home app-binding literals, ≥4 pages, 8–16 unique traces, exhaustive hostile mutation set |
| `proveDirectoryCommandReceiptV1` | `DirectoryCommandReceiptV1` | envelope key set, five limit literals, table sizes, transport inventory; per-receipt unknown-field rejection |
| `proveDirectoryCommandAuthorityV1` | — | version const, envelope key set, capacity literals, table sizes, id uniqueness, ordered invite-scope inventory, per-row enum closure |
| `proveDirectorySpaceAdministrationPageV1` | `DirectorySpaceSessionBindingV1`, `SpaceAdministrationSpaceV1`, `SpaceAdministrationMemberRowV1`, `SpaceAdministrationInviteRowV1` | envelope key set, four limit literals, session↔space binding equality, window inventories, invite-secret rejection, ordered vector + 9-hostile inventories, 8 cursor cases; the old whole-fixture unknown-field hostile became contract-stage member/space mutations |
| `provePresenceLeaseFixture` | `PresenceLeaseOperationV1`, `PresenceScopeSummaryV1` | envelope key set, four limit literals, 8–16 unique vectors, per-vector ephemeral/member-fanout and outcome closure; hostiles are now contract-stage operation/summary mutations |
| `provePresenceNormalizationFixture` | `PresenceAdmissionV1` | envelope key set, `maximumEntryBytes` literal, 12–32 unique vectors, hex shape/bounds, `durableWrites === 0`; forged-claim rejections |
| `AdminPresenceTargetRecoveryCheckScript.run` | `DirectorySocketScopeV1` | envelope key set, surface literal, membership shape, full `expected` key set + every literal |
| `proveInviteRedemptionTransaction` | `InviteRedemptionCallV1`, `InviteAcceptanceMarkerV1` | envelope key set, three limit literals, table sizes, exhaustive hostile-mutation set, per-vector call/outcome arity. The seven `hostiles[]` are now dispatched: five client-authority mutations are rejected by `InviteRedemptionCallV1`, `unknown-field` by the envelope key set, `oversized-identifier` by `InviteAcceptanceMarkerV1` |
| `proveDirectorySpaceJourneyV1` | `DirectoryProfileV1`, `DirectoryDocumentDescriptorV1`, `LocalizedTextV1` | envelope key set, bound/deadline literals, distinct profiles, EN/DE label distinctness + min length, kebab id shape, step/skip/route inventories and uniqueness, privacy inventory, space taxonomy |
| `proveSpacePublicBoundaryFixture` | `PublicSpaceDetailV1`, `MemberSpaceDetailV1`, `AuthorSpaceDetailV1` | envelope key set, access inventory, 21/21 forbidden-key and hostile inventories; the 21 hostiles now run through `assertHubFixtureExpectation` against the fixture-declared `stage/result/code` |
| `🎯️admin-intent-v1/🧪️oracle/🟦️.ts` | `AdminPrincipalV1`, `AdminCreateSpaceIntentV1`, `AdminActorV1`, `AdminRecordedConnectionV1`, `AdminIntentOutcomeV1` | envelope key set, four limit literals, public-projection/audit/invalid/redaction inventories; generic-command and non-admin-peer-class rejections |

## D. Verification (real output)

### D.1 Module loads as draft-07 and every rewired fixture member validates

Harness: `<scratchpad>/wp4-hub-directory-verify.ts` — `new Ajv({ strict: true, allErrors: true })` from the
`ajv` package (draft-07, **not** `ajv/dist/2020`), `addSchema` of the module, `getSchema("<$id>#/$defs/<Export>")`
per export, then the real fixture member plus the negative mutations.

```
$ bun <scratchpad>/wp4-hub-directory-verify.ts
hub.directory $id=https://semio.tech/schema/hub/directory/schema.json
exports (30): DirectoryDocumentScopeV1, DirectorySocketScopeV1, DirectorySocketMessageRoutingV1, DocumentOpenIntentV1, ExecutionTargetAssetRouteV1, SocketGrantReceiptV1, DirectoryCommandReceiptV1, DirectorySessionBindingV1, DirectorySpaceSessionBindingV1, SpaceAdministrationSpaceV1, SpaceAdministrationMemberRowV1, SpaceAdministrationInviteRowV1, PublicSpaceDetailV1, MemberSpaceDetailV1, AuthorSpaceDetailV1, DirectoryDocumentDescriptorV1, DirectoryEventPageCursorV1, DirectoryProfileV1, LocalizedTextV1, AdminPrincipalV1, AdminCreateSpaceIntentV1, AdminOperationIntentV1, AdminActorV1, AdminRecordedConnectionV1, AdminIntentOutcomeV1, InviteRedemptionCallV1, InviteAcceptanceMarkerV1, PresenceAdmissionV1, PresenceLeaseOperationV1, PresenceScopeSummaryV1
private helpers (25): id, identifier, text, hash, nonzeroHash, requestId, safeInteger, positiveSafeInteger, spaceKind, spaceVisibility, spaceRole, documentOwner, documentFrontier, publicSpace, memberSpace, publicDocument, scopedDocumentDescriptor, spaceDocumentView, spaceMember, spaceInvite, directoryEventHlc, directoryEventActor, directoryEvent, directoryCommandResult, presenceBoundedText
public-space-detail: 21 fixture-declared contract rejections all denied by PublicSpaceDetailV1
fixtures bound to a module export (15): 🔌️scoped-socket-revocation-v1, 🚶️admin-live-journey-v1, 🛂️admin-presence-target-recovery-v1, 👥️presence-lease-v1, 🪪️presence-normalization-v1, 🏘️space-administration-page-v1, 📅️event-page-route-v1, 🔏️document-execution-target-lease-v1, 🚻️space-journey-v1, 🧾️command-receipt-v1, 🪪️execution-target-relay-v1, 🌐️directory-home-browser-process-v1, 🏛️public-space-detail-v1, 🎯️admin-intent-v1, 🎟️invite-redemption-transaction-v1
fixtures replaced by explicit oracle-table assertions (6): 📣️ordered-append-broadcast-v1, 🛡️command-authority-v1, 🔐️share-issuance-atomicity, 🏛️retained-short-admin, 🌐️directory-message-authority-v1, 🏛️admin-directory-authority-v1
hub.directory module verification: assertions=304 exports=30 fixtures=21
```

Second harness `<scratchpad>/wp4-hub-directory-envelope.ts` re-evaluates every literal envelope/inventory
string this WP wrote into `📜️script.ts` against the real fixtures:

```
$ cd /Users/ueli/Documents/semio && bun <scratchpad>/wp4-hub-directory-envelope.ts
hub.directory envelope-assertion re-evaluation: literals=33 all matched the real fixtures
```

### D.2 Hub oracle commands at the real router (no cargo)

Ran before a concurrent peer broke the shared import graph (see §E.5):

```
$ bun 🌎️hub/📦️packages/🦀️rust/📜️script.ts directory-ordered-publication-check
directory-ordered-publication-check: checks=9 clean

$ bun 🌎️hub/📦️packages/🦀️rust/📜️script.ts share-issuance-atomicity-check
share-issuance-atomicity-oracle: AJV=1 bun-sqlite=7 source-hostiles=7; PostgreSQL/Neo4j are source-only
share-issuance-atomicity-check: checks=14 mode=source

$ bun 🌎️hub/📦️packages/🦀️rust/📜️script.ts retained-short-admin-check
retained-short-admin-oracle: AJV=1 bun-sqlite=42 source-hostiles=25
retained-short-admin-check: checks=67 mode=source

$ bun 🌎️hub/📦️packages/🦀️rust/📜️script.ts admin-directory-authority-check source
admin-directory-authority-check: AJV=1 ordering=5 short-actions=8 bindings=15 phase=source

$ bun 🌎️hub/📦️packages/🦀️rust/📜️script.ts directory-message-authority-check source
directory-message-authority-check: AJV=1 ordering=4 wire-scopes=6 hostile-source=5 phase=source

$ bun 🌎️hub/📦️packages/🦀️rust/📜️script.ts presence-lease-check
presence-lease-oracle: hub.directory-exports=2 vectors=8 contract-hostiles=6 source-hostiles=5 browser-schedule=1
presence-lease-check: checks=20 phase=source

$ bun 🌎️hub/📦️packages/🦀️rust/📜️script.ts presence-normalization-check source
presence-normalization-independent-oracle: AJV=1 LEB128=1 exact-vectors=17
presence-normalization-check: checks=17 phase=source

$ bun 🌎️hub/📦️packages/🦀️rust/📜️script.ts admin-presence-target-recovery-check source
admin-presence-target-recovery-independent-oracle: AJV=1 scoped-removal=1
admin-presence-target-recovery-check: phase=source

$ bun 🌎️hub/📇️directory/🧫️fixtures/🎯️admin-intent-v1/🧪️oracle/🟦️.ts
admin-intent-v1 oracle: 5/5; invalid inventory 22/22; hub.directory exports 5/5
```

After the peer breakage every `bun 📜️script.ts <cmd>` aborts at
`Cannot find module '../📃️pageSchema/🟦️.ts' from '🧰️framework/🔨️modules/🎭️actor/📤️return/🟦️.ts'`. It was still
broken after 27 retry attempts over 17 minutes, so the remaining commands were run through a *test-only*
Bun `onResolve` shim in the scratchpad (`wp4-hub-directory-shim.ts`, maps `📃️pageSchema` → the `📃️page`
directory that is actually on disk). Nothing in the repository was changed for this.

```
$ bun --preload <scratchpad>/wp4-hub-directory-shim.ts 🌎️hub/📦️packages/🦀️rust/📜️script.ts space-administration-check
space-administration-oracle: AJV=2 vectors=4 cursors=8 hostiles=9 source-hostiles=9 component-schema=9 sha256=1 binding=1
space-administration-check: checks=41 phase=source

$ bun --preload <shim> 🌎️hub/📦️packages/🦀️rust/📜️script.ts space-journey-check source
space-journey-check: checks=149 phase=source steps=20 skips=5

$ bun --preload <shim> 🌎️hub/📦️packages/🦀️rust/📜️script.ts execution-target-relay-check
hub-native-artifact-provider-frontier-oracle: scope-exports=2 headless=sqlite plugin-deps=0 production-receipts=29 configured-no-provider=reject
execution-target-provider-frontier: checks=12
execution-target relay runtime routes=21 responses=9 bounded-capacity=2 exact-bytes=true
execution-target-relay-check: checks=39
execution-target-relay-check: existing browser proof-ratchet runtime regression clean

$ bun --preload <shim> 🌎️hub/📦️packages/🦀️rust/📜️script.ts execution-target-lease-check
execution-target-lease-oracle: ajv=1 positive=1 manifest-fields=49 byte-vectors=13 lifecycle=11 hostile=73 component-bytes=1024 descriptor-bytes=656 node+webcrypto-sha256=agree first-party-blake3=known-answer status=5 passed
execution-target-lease-source: browser lease region=renderer-free hub routes=4 accessor=generation-bound native=full-field passed

$ bun --preload <shim> 🌎️hub/📦️packages/🦀️rust/📜️script.ts scoped-directory-socket-check source
scoped-directory-socket-oracle: hub.directory-exports=2 decisions=19 hostiles=3 client-closes=3 relay=3
```

#### Three commands stop later, in Rust source fences that predate this work

`directory-event-page-v1-check`, `directory-command-receipt-check` and `invite-redemption-transaction-check`
each get **past** every rewired schema/contract assertion and then fail in a Rust source-mutation oracle over
`🌎️hub/📇️directory/{🦀️.rs,🪶️sqlite,🐘️postgres,🌐️neo4j}` — files this WP never touched.

```
$ bun --preload <shim> … directory-event-page-v1-check
error: directory event page route/storage source boundary is incomplete   (📜️script.ts:12929, sourceClosed)

$ bun --preload <shim> … directory-command-receipt-check
error: PostgreSQL invite scope predicates or bindings drifted             (📜️script.ts:14315, inside proveDirectoryCommandAuthorityV1 — i.e. proveDirectoryCommandReceiptV1 already returned)

$ bun --preload <shim> … invite-redemption-transaction-check
error: archive authority oracle admitted removed fence 0                  (📜️script.ts:15840, archiveClosed)
```

Attribution — the two fence predicates were extracted from `📜️script.ts` and re-evaluated against
`git show 0a0bb74380:<rust file>` (the last commit before this session began):

```
$ bun <scratchpad>/wp4-fence-attribution.ts
directory-event-page fence @ worktree   : false
directory-event-page fence @ 0a0bb74380 : false

$ bun <scratchpad>/wp4-fence-attribution2.ts
invite-redemption archive fence @ worktree:   base=true hostile0-detected=false hostile1-detected=true
invite-redemption archive fence @ 0a0bb74380: base=true hostile0-detected=false hostile1-detected=true
```

And for the PostgreSQL binding drift, the `.bind(...)` order in `🌎️hub/📇️directory/🐘️postgres/🦀️.rs` grew four
extra binds between `aa72759d41` (2026-09-07 16:52) and `0a0bb74380` (2026-09-08 14:30), both before this
session:

```
now      : ['invite_id','revoked_at','reason','space_id','invite_id','space_id','invite_id','revoked_at','reason','space_id']
0a0bb743 : ['invite_id','revoked_at','reason','space_id','invite_id','space_id','invite_id','revoked_at','reason','space_id']
aa72759d : ['invite_id','revoked_at','reason','space_id','invite_id','space_id']
expected : ['invite_id','revoked_at','reason','space_id','invite_id','space_id']
```

All three are pre-existing regressions owned by whoever is refactoring the directory backends; they are listed
under §E as cross-partition requests, not fixed here.

#### Not run (would invoke cargo or a built binary — this ticket forbids cargo)

- `space-public-boundary-check` (`runProbe("cargo", […"--list"])` + `runCargo` after the oracle) — `proveSpacePublicBoundaryFixture`'s contract logic is covered by D.1 instead (4 positives, 21 declared hostile rejections, raw-event denial).
- `directory-home-browser-process-check` — `proveDirectoryHomeBrowserProcessSource` itself shells out to `cargo tree`.
- `admin-live-journey-check` — needs a built `os-hub` binary and Chromium.
- every `--native` / `process` phase of the commands above.

### D.3 `📜️script.ts` still parses

```
$ cd /Users/ueli/Documents/semio && bun build --target=bun --no-bundle 🌎️hub/📦️packages/🦀️rust/📜️script.ts
Transpiled file in 83ms
```

The router's no-arg usage line also printed correctly during D.2 (the full `<setup|build|test|…>` usage banner),
which only happens after the whole file and its import graph load.

A repo-wide `tsc --noEmit` is not a usable gate here: this workspace has no `@types/bun` installed and the root
`tsconfig.json` does not set `allowImportingTsExtensions`, so a scoped run over `📜️script.ts` reports 726
pre-existing errors (`Cannot find name 'Bun'`, `Cannot find module 'bun:sqlite'`, `TS5097`, …). Cross-checking
those 726 error lines against the lines this WP authored found zero attributable to the rewiring.

### D.4 No fixture-owned schema left on disk

`git ls-files` still lists the 21 blobs because they are only removed from the working tree — staging a
deletion needs a git-modifying command, which this ticket forbids; the repository's auto-commit picks them up.
The equivalent worktree check over the whole hub:

```
$ git ls-files 🌎️hub | grep -E 'schema\.json$' \
    | while IFS= read -r f; do [ -e "$f" ] && echo "$f"; done
🌎️hub/💡️inference/🧬️schema/🤝️two-author-shell-v1/🧬️.schema.json
🌎️hub/🧪️fixtures/↩️gis-map-approval-undo-v1/🧬️.schema.json
🌎️hub/🧪️fixtures/⏸️gis-inference-checkpoint-control-v1/🧬️.schema.json
🌎️hub/🧪️fixtures/⛓️inference-wal-chain-v1/🧬️.schema.json
🌎️hub/🧪️fixtures/✅️inference-approval-v1/🧬️.schema.json
🌎️hub/🧪️fixtures/✉️inference-command-v1/🧬️.schema.json
🌎️hub/🧪️fixtures/🎯️inference-catalog-selection-v1/🧬️.schema.json
🌎️hub/🧪️fixtures/🖥️inference-server-identity-v1/🧬️.schema.json
🌎️hub/🧪️fixtures/🗳️gis-map-proposal-approval-v1/🧬️.schema.json
🌎️hub/🧪️fixtures/🗺️gis-inference-job-v1/🧬️.schema.json
🌎️hub/🧪️fixtures/🗺️gis-inference-retained-runtime-v1/🧬️.schema.json
🌎️hub/🧪️fixtures/🛂️inference-author-v1/🧬️.schema.json
🌎️hub/🧪️fixtures/🧊️gis-map-frozen-binding-v1/🧬️.schema.json
🌎️hub/🧪️fixtures/🧾️inference-wal-proof-v1/🧬️.schema.json
```

All 14 survivors belong to the `hub.inference` / GIS-inference partition (the `-v1` fixtures the execution
contract §D assigns to `hub.inference`). None of the 21 `hub.directory` files remains, and both
`👥️presence-lease-v1/🧬️schema/` and `🪪️presence-normalization-v1/🧬️schema/` are gone as directories:

```
$ ls 🌎️hub/📦️packages/🦀️rust/🧪️fixtures/👥️presence-lease-v1 🌎️hub/📦️packages/🦀️rust/🧪️fixtures/🪪️presence-normalization-v1
🌎️hub/📦️packages/🦀️rust/🧪️fixtures/👥️presence-lease-v1:
🧪️fixture
🧫️fixture

🌎️hub/📦️packages/🦀️rust/🧪️fixtures/🪪️presence-normalization-v1:
🧪️fixture
```

## E. Cross-partition requests

1. **`os.directory` (`🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/`)** — left exactly as-is, still
   2020-12 and still read directly by `proveExecutionTargetLeaseCorpus` for `DocumentOpenPlanV1` and
   `DocumentExecutionTargetLeaseFieldsV1`, and by `🌐️browser-actor/🔣️.schema.json`. When that partition
   migrates to draft-07 it must keep both export ids; `proveExecutionTargetLeaseCorpus` binds to them by name.
   It should additionally export `DocumentOpenIntentV1` and a socket-grant type, or explicitly re-export
   `hub.directory`'s — today `parseDocumentOpenIntentV1` / `parseSocketGrantReceiptV1` live in os TS while the
   wire ids are `semio.hub.*`, so `hub.directory` claims them.
2. **`hub.local-bootstrap`** — `adminLiveJourneyFixture` still compiles
   `🌎️hub/🚀️local-bootstrap/🧪️fixtures/⏳️idle-admission-v1/🧬️.schema.json` with `Ajv2020`. Untouched; that
   fixture schema belongs to the `hub.local-bootstrap` worker.
3. **`hub.inference` / plugin partitions** — the other 22 `🧬️.schema.json` reads left in `📜️script.ts` all point
   outside `🌎️hub/📇️directory`, `🌎️hub/🧪️fixtures/📇️directory`, `🌎️hub/🧪️fixtures/🪪️execution-target-relay-v1`
   and `🌎️hub/📦️packages/🦀️rust/🧪️fixtures/{🌐️directory-message-authority-v1,🏛️admin-directory-authority-v1,
   🛂️admin-presence-target-recovery-v1,👥️presence-lease-v1,🪪️presence-normalization-v1}` and were not touched.
4. **WP2 catalog** — `hub.directory` now has a module; the derived catalog
   (`🦑️repo/🔨️modules/📚️library/🔣️schema-catalog.json`) must list the 30 export ids above.
5. **`framework.actor` peer refactor (blocking every hub command).** At 16:20 today
   `🧰️framework/🔨️modules/🎭️actor/📤️return/🟦️.ts:2` began importing `../📃️pageSchema/🟦️.ts` while the directory
   on disk is still `📃️page`, so every `bun 🌎️hub/📦️packages/🦀️rust/📜️script.ts <cmd>` aborts at module
   resolution. Still broken after 27 polls over 17 minutes. Needs the `📃️page` → `📃️pageSchema` directory
   rename (or the import reverted). D.2's later runs used a scratchpad-only Bun resolver shim to work around it.
6. **Directory backend Rust fences already red** (owner: whoever is refactoring `🌎️hub/📇️directory/*`), all
   proven pre-existing in D.2:
   - `📜️script.ts:12929` `sourceClosed` — `directory event page route/storage source boundary is incomplete`.
   - `📜️script.ts:14315` — PostgreSQL `revoke_invite_as` now issues 10 `.bind(...)` calls where the fence
     expects 6; drifted between `aa72759d41` and `0a0bb74380`.
   - `📜️script.ts:15840` `archiveClosed` hostile 0 — removing `space_kind == Some("archive")` from
     `🌎️hub/📇️directory/🦀️.rs` no longer flips the fence, so the mutant is admitted.

## F. Open questions

1. **Self-contained module vs cross-scope `$ref`.** `hubSchemaExport` compiles the scope's module alone
   (`ajv.addSchema(document)`), so any cross-scope `$ref` in it would fail to resolve at validation time.
   `hub.directory` is therefore self-contained: shapes that also exist in `os.directory` (`DocumentScope`,
   `DirectoryCommandReceiptV1`, `MemberSpaceViewV1`, the admin `$defs`) are declared locally rather than
   `$ref`'d. Either the resolver must learn to preload the referenced scopes (`HUB_SCHEMA_SCOPES` already has
   the paths), or the contract must say that same-shape declarations across scopes are checked by a
   conformance test instead. Flagging for the ticket-level decision.
2. **`🔣️.json`-only module.** The task specified only the JSON Schema facet, so no `🛰️.proto`, `🔗️.graphql`,
   `🦀️.rs` or `🟦️.ts` siblings were written for `hub.directory` (the reference `hub.inference` module has a
   `🟦️.ts`). If WP5/WP6 requires the canonical five per module, `hub.directory` still needs four more facets.
3. **`ExecutionTargetAssetRouteV1` is intentionally open.** It keeps the source schema's shape (no
   `additionalProperties: false`) because it is applied to relay `routes[]` rows that also carry `id`/`admitted`;
   it is a path/method admission predicate, not a closed message.
4. **`👥️presence-lease-v1` still has both `🧪️fixture/` and `🧫️fixture/`.** Only the illegal `🧬️schema/` was
   removed; the duplicated data directory is a separate naming question for the fixture-taxonomy work.
