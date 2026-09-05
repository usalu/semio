# Sol Report — Document Open Plan Foundation

Date: 2026-09-04  
Packet: `implement_document_open_plan_foundation`

## Boundary

This packet lands the dependency-free D0 foundation for `DocumentOpenPlanV1` plus the smallest truthful D1 server activation slice: strict cross-language public schemas, a language-neutral corpus and independent oracle, a bounded digest-only one-use in-process plan ledger, an immutable verified open-target catalog projection, authenticated plan issuance, and authenticated receipt-to-document-`SocketGrantV1` exchange. Both routes are mounted, but both fail closed and readiness advertises both features false unless startup has completely verified a nonempty trusted catalog generation. Ordinary startup currently links no native codec bindings, so no browser/native/WGPU/MCP acquisition or transport availability is claimed.

## Implemented source

- Rust and TypeScript strict codecs for `DocumentOpenIntentV1`, `DocumentOpenPlanV1`, `DocumentPlanSocketGrantIntentV1`, exact selected package/artifact/surface/grant/checkpoint projections, and the bounded public error vocabulary.
- Recursive unknown-field rejection, Unicode `Cc` rejection, UTF-8 byte bounds, canonical 32-byte base64url receipt grammar, 30-second maximum TTL, and the shared `2^53-1` integer ceiling for expiry, frontier, and revalidation numbers.
- A neutral JSON corpus with independent documented byte encodings and fixed SHA-256 vectors for descriptor, catalog generation, and receipt digest, plus hostile scope/authority/expiry/unknown-field/control/integer/canonical-tail vectors.
- A first-party Bun/Node built-in structural oracle that hand-decodes all three public forms and does not import the production codec for its decision path. Production TypeScript decoding is compared only in a later, separately labelled parity phase.
- A process-local ledger that stores receipt digests only, wipes capability bytes, allows one outstanding plan per binding and scope, caps 64 issued plans per binding and 1,024 records per process, consumes exactly once, binds the socket selector, expires/revokes entries, and disappears on restart.
- Replacement-index removal is conditional on exact digest ownership. The A→B, expire/sweep A, issue C law rejects stale B and exchanges only C.
- Session self-revoke, administrator user-session revoke, and administrator document-share revoke invalidate matching unconsumed plan bindings together with socket grants.
- The uncached `os-hub:open-plan-check` target and launch seed entry resolve full Rust test names from `cargo test --list`, require exactly one match, and execute those full names with `--exact`.
- The internal D1 exchange holds the plan mutex through socket-ledger issuance, binds the exact document audience, server actor, authenticated subject, and full private plan identity, caps socket expiry at both plan and binding expiry, and marks the plan consumed only after socket issuance succeeds. A socket-capacity failure leaves the plan issued and retryable.
- Receipt decoding uses a wipe-on-drop 32-byte candidate. Its late-invalid-character law observes 31 nonzero candidate bytes before drop and exactly 32 zero bytes after drop.
- `POST /spaces/{space_id}/documents/{document_id}/socket-grants` now accepts only strict `DocumentPlanSocketGrantIntentV1` JSON, rejects query smuggling and bodies over 8 KiB, applies one two-second request deadline, reauthenticates the exact session/share binding, revalidates its authority generation and expiry under the subject gate, matches the private plan scope and current durable descriptor, and consumes the plan only when the bounded socket grant has been issued.
- The route returns only the public socket-grant envelope or the bounded redacted `DocumentOpenPlanErrorV1`; it never releases the receipt digest, private descriptor, user/session identity, or internal authority.
- Hub readiness now advertises `openPlan` and `openPlanExchange` from the same verified-catalog activation condition. The route laws prove both handlers enforce that condition, preventing advertised/actual availability drift.
- The route runtime law covers exact success, replay, wrong principal, wrong path, unknown fields, query smuggling, oversize body, share read-only authority, noncanonical receipt, non-consumption on pre-exchange denials, and response redaction.
- The trusted bundle schema now requires a bounded `openTargets` list per package. Loader publication resolves each target only after the selected dependency closure, component SHA-256/BLAKE3, descriptor bytes, package identity and exact native codec schema/hash all verify. It rejects duplicate or codec-unbound targets, sorts the retained public projections, and derives one deterministic generation SHA-256 from the neutral encoding.
- `HubState` retains the same process-lifetime verified catalog generation behind one private resolver authority. Test doubles implement only that private interface; the production catalog exposes no constructor or mutable registration bypass.
- `POST /spaces/{space_id}/documents/{document_id}/open-plan` accepts only one strict `DocumentOpenIntentV1` body under 8 KiB and a 10-second request deadline. It authenticates an exact current session membership or exact-document share, serializes by subject binding, loads the durable descriptor and active checkpoint, selects exactly one role/surface target, derives the server actor and private client-instance digest, rechecks directory revision and subject authority at the publication fence, caps expiry at 30 seconds and the credential expiry, then publishes one private receipt atomically into the bounded ledger.
- Issuance returns only `DocumentOpenPlanV1`; raw bearer/session/share identifiers, client instance, descriptor bytes, executable factories and private catalog rows never enter the public response. Session authors receive only editor/write targets; shares and spectators resolve only viewer/read-only targets, with no fallback across role or requested surface.
- Readiness and handler availability use the same immutable catalog condition: `openPlan` and `openPlanExchange` are both true only for a completely verified nonempty generation, and both handlers recheck their advertised feature fence. Empty/unconfigured catalog startup advertises both false and returns `catalog-unavailable` without issuing or consuming authority.
- The neutral corpus now contains two deterministic editor/viewer catalog rows plus 11 independent issuer decisions spanning author default/exact selection, share viewer, role/surface mismatch, foreign scope, wrong schema, unknown authority, Unicode control and client-id bound. The self-contained Node oracle decides these without importing production codecs.
- Socket grant consumption rechecks the retained private plan authority before the one-use transition and again before `Welcome`/live traffic: exact selected surface, durable descriptor/digest, immutable catalog generation and row, directory revision, active checkpoint, actor and subject must still match. A stale or substituted pending grant is terminally rejected; transient directory failure remains retryable.
- The neutral corpus includes six independent socket-consume decisions for exact authority and surface/descriptor/catalog/revision/checkpoint substitution. The self-contained oracle evaluates them without a production codec or Rust implementation.
- The registered default-feature gate is expanded to eight exact-one laws: one real trusted-loader generation/resolution law and seven hub laws including issuer and socket-consume authority. The historical all-features gate is expanded to nine exact-one selectors but retains session `45804` as an unresolved external qualification RED until it can complete from current bytes.

## Evidence

### Independent oracle

Command:

```text
bun nx run os-hub:open-plan-check --skip-nx-cache
```

Session `84420` emitted:

```text
document-open-plan-oracle: descriptor=1 catalog=1 receipt=1 codecs=3 negative=9 redaction=1 passed
```

The session then exited 1 before an owned Rust law because concurrent OS mutation modules referenced missing `MutationLeaf` source-authority paths. This is recorded as external pre-owned-code failure and is not runtime evidence. The corpus has since expanded to 16 hostile mutations; the registered gate must be rerun on the final source.

Session `88185` then exited 1 at the strengthened non-vacuity preflight with `selected 0`: it proved that the earlier `os-hub --lib` selector did not own the schema test and refused to count it. The gate now resolves that law from its actual `semio-framework-os-kernel --lib` package. This failed-safe discovery is not a test pass.

Final-source session `45804` emitted:

```text
document-open-plan-oracle: descriptor=1 catalog=1 receipt=1 independent-codecs=3 negative=16 redaction=1 passed
document-open-plan-production-parity: codecs=3 rejected=12 passed
```

The status-preserving exact-law preflight then exited 1 with Cargo status 101 before it could list or execute an owned Rust law. Cargo's first diagnostic was external to this packet:

```text
error[E0425]: cannot find function `issues_scoped_to_new_solids` in this scope
.../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🔺️diff/🔀️boolean/🦀️.rs:500:18
```

This terminal proves the independent oracle and production TypeScript parity phases on final bytes. It is not Rust runtime evidence: the all-features `semio-framework-os-kernel` preflight compiled `semio-s-plugin-stdio`, which failed in concurrent B-rep source before the owned test binary could be enumerated.

Default-feature server session `24575` then reached owned code and exited 1 before test selection with `E0063` because one synthetic directory-visibility fixture did not initialize the newly private `document_plan` socket-record field. The fixture now sets `None`; no pass is claimed from that run.

Session `63523` was intentionally terminated with exit 130 after Terra identified the late-invalid receipt decoder wipe defect in its active source. Before interruption it had non-vacuously resolved all three then-registered full names (`tests::document_open_plan_ledger_...`, `tests::document_open_plan_admin_revocation_...`, and `tests::document_open_plan_receipt_exchange_...`) and was waiting for the shared Cargo lock; no Rust law had started. The decoder and a fourth exact wipe law have since been added, so this interrupted run is superseded and is not evidence.

Pre-activation registered server-subset session `34384` exited 0 for the then-private four-law boundary. Its independent phases emitted:

```text
document-open-plan-oracle: descriptor=1 catalog=1 receipt=1 independent-codecs=3 negative=16 redaction=1 activation=0 passed
document-open-plan-production-parity: codecs=3 rejected=12 passed
```

It then exact-listed exactly one fully qualified name for each of the two D0 server laws and two D1 laws, and exact-ran each under the hub's default feature set with `--test-threads=1`:

```text
tests::document_open_plan_ledger_is_digest_only_bounded_single_use_revalidated_and_restart_scoped: 1 passed, 0 failed
tests::document_open_plan_admin_revocation_invalidates_session_and_share_bindings: 1 passed, 0 failed
tests::document_open_plan_receipt_exchange_mints_one_exact_bounded_socket_grant_without_route: 1 passed, 0 failed
tests::document_open_plan_late_invalid_receipt_wipes_exact_candidate_bytes: 1 passed, 0 failed
document-open-plan-server-check: default-feature subset passed; kernel all-features schema qualification remains separate
```

This is historical pre-activation evidence only. The exchange handler, readiness contract, hostile route law, and current selector names were added afterward.

Final current-source registered server-subset command:

```text
bun nx run os-hub:open-plan-server-check --skip-nx-cache
```

Session `79424` exited 0. Its independent phases emitted:

```text
document-open-plan-oracle: descriptor=1 catalog=1 receipt=1 independent-codecs=3 negative=16 exchange-negative=5 redaction=1 activation=exchange-only passed
document-open-plan-production-parity: codecs=3 rejected=12 exchange-rejected=5 passed
```

The gate then listed each current fully qualified hub law, required exactly one match, and exact-ran each with `--test-threads=1`:

```text
tests::document_open_plan_ledger_is_digest_only_bounded_single_use_revalidated_and_restart_scoped: 1 selected, 1 passed, 0 failed
tests::document_open_plan_admin_revocation_invalidates_session_and_share_bindings: 1 selected, 1 passed, 0 failed
tests::document_open_plan_receipt_exchange_mints_one_exact_bounded_socket_grant: 1 selected, 1 passed, 0 failed
tests::document_open_plan_exchange_route_is_authenticated_exact_hostile_and_single_use: 1 selected, 1 passed, 0 failed
tests::document_open_plan_late_invalid_receipt_wipes_exact_candidate_bytes: 1 selected, 1 passed, 0 failed
document-open-plan-server-check: default-feature subset passed; kernel all-features schema qualification remains separate
```

This is final-source runtime evidence for the exchange-only server boundary. It does not clear session `45804`'s kernel/all-features schema qualification failure, and it is not browser, native, WGPU, or MCP transport evidence.

That statement is retained as historical exchange-only evidence. The later issuer/catalog source superseded it.

### Catalog-backed issuer progress

Owned target root:

```text
/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🗑️generated/open-plan-issuer-target
```

- Session `90977`: production `semio-hub --lib` compile exited 0 after the trusted-catalog schema, deterministic generation and exact resolver landed.
- Production `os-hub --bin` compile exited 0 on the same owned incremental target before test compilation.
- Session `31166`: exact issuer-law binary test compilation exited 0 after replacing the invalid cross-crate `#[cfg(test)]` catalog constructor with a private resolver authority and test implementation.
- Sessions `90101` and `93667`: focused issuer law exited 1 with `component-unavailable` then `stale`; these exposed and corrected the fixture projection's component SHA-256/package-hash binding. Neither is pass evidence.
- Session `13832`: focused issuer route law exited 0 with `1 passed; 29 filtered`, covering authenticated session/share issue→exchange, exact role/surface/catalog/descriptor authority, TTL/redaction, hostile JSON/path/query/oversize input, catalog absence, capacity, and abort before publication.
- Session `32009`: catalog loader law first exited 101 because the new test moved a field from a Drop-owning fixture. The field now clones; no pass is claimed.
- Session `53281`: exact real-loader law exited 0 with `1 passed; 60 filtered`, proving complete bundle verification precedes a deterministic two-target generation and exact editor/viewer resolution.
- Sessions `99288` and `62949`: focused issuer route law exposed that the raw HTTP helper treated a valid server-side oversized-body connection close as a harness panic. The helper now preserves normal-response strictness, records an explicit transport-close rejection only for that hostile request, and separately asserts the route boundary returns HTTP 413 when directly exercised. These red runs are not pass evidence.
- Session `92484` was intentionally terminated with exit 130 after removing the client write-half shutdown exposed that its EOF-based response reader could wait indefinitely on a valid framed response. No law result is claimed.
- Session `65885`: the final focused issuer law exited 0 with `1 passed; 29 filtered`. The raw test client now completes on the exact `Content-Length` frame, while preserving explicit reset handling for the hostile oversized request. The cancellation fixture uses the same catalog readiness fence and reaches a bounded publication semaphore before abort; the ledger remains empty.

Pre-socket-consume-fence registered server-subset session `40418` exited 0. Its independent phases emitted:

```text
document-open-plan-oracle: descriptor=1 catalog=2 receipt=1 independent-codecs=3 issuer=11 negative=16 exchange-negative=5 redaction=1 activation=catalog-gated-issuer+exchange passed
document-open-plan-production-parity: codecs=3 rejected=12 exchange-rejected=5 passed
document-open-plan-server-laws: qualification=default-feature-subset exact=7 laws=artifact_authority::trusted_catalog::tests::verified_trusted_catalog_document_open_generation_and_resolution_are_exact,tests::document_open_plan_ledger_is_digest_only_bounded_single_use_revalidated_and_restart_scoped,tests::document_open_plan_admin_revocation_invalidates_session_and_share_bindings,tests::document_open_plan_receipt_exchange_mints_one_exact_bounded_socket_grant,tests::document_open_plan_exchange_route_is_authenticated_exact_hostile_and_single_use,tests::document_open_plan_late_invalid_receipt_wipes_exact_candidate_bytes,tests::document_open_plan_issue_route_is_catalog_bound_authenticated_bounded_cancel_safe_and_exchangeable
document-open-plan-server-check: default-feature subset passed; kernel all-features schema qualification remains separate
NX Successfully ran target open-plan-server-check for project os-hub
```

Every one of the seven printed fully qualified laws was first required to resolve exactly once, then ran with `--exact --test-threads=1` and passed. The later consume-time authority fence supersedes this terminal as final-source evidence.

Session `095981` exact-ran `tests::document_open_plan_socket_consume_revalidates_surface_descriptor_catalog_revision_and_checkpoint` and exited 0 with `1 passed; 30 filtered`. It proves wrong surface, newly published checkpoint/directory revision, changed catalog generation, private descriptor/revision/checkpoint substitution, terminal stale-grant removal, and an exact fresh consume.

Final current-source registered server-subset session `95998` exited 0. Its independent and exact-selection phases emitted:

```text
document-open-plan-oracle: descriptor=1 catalog=2 receipt=1 independent-codecs=3 issuer=11 consume=6 negative=16 exchange-negative=5 redaction=1 activation=catalog-gated-issuer+exchange passed
document-open-plan-production-parity: codecs=3 rejected=12 exchange-rejected=5 passed
document-open-plan-server-laws: qualification=default-feature-subset exact=8 laws=artifact_authority::trusted_catalog::tests::verified_trusted_catalog_document_open_generation_and_resolution_are_exact,tests::document_open_plan_ledger_is_digest_only_bounded_single_use_revalidated_and_restart_scoped,tests::document_open_plan_admin_revocation_invalidates_session_and_share_bindings,tests::document_open_plan_receipt_exchange_mints_one_exact_bounded_socket_grant,tests::document_open_plan_exchange_route_is_authenticated_exact_hostile_and_single_use,tests::document_open_plan_late_invalid_receipt_wipes_exact_candidate_bytes,tests::document_open_plan_issue_route_is_catalog_bound_authenticated_bounded_cancel_safe_and_exchangeable,tests::document_open_plan_socket_consume_revalidates_surface_descriptor_catalog_revision_and_checkpoint
document-open-plan-server-check: default-feature subset passed; kernel all-features schema qualification remains separate
NX Successfully ran target open-plan-server-check for project os-hub
```

All eight printed fully qualified laws resolved exactly once and exact-ran with `--test-threads=1`; all passed. This is the final server-only catalog/issuer/exchange/consume-authority terminal. It does not clear the historical all-features qualification RED or prove browser/native/WGPU/MCP transport.

### Gate non-vacuity

The all-features target performs independent `--list` preflights for nine exact laws: the kernel schema/corpus law, the real trusted-loader catalog generation/resolution law, and seven binary laws for ledger, revocation, atomic exchange, hostile exchange route, late-invalid wipe, authenticated catalog-backed issuance and consume-time authority. The default-feature subset uses the same exact-one contract for the catalog law and seven hub laws, eight total. Both gates fail unless every suffix resolves to exactly one `: test` row, print every resolved full name, and execute each with `--exact`. Session `45804` remains evidence that an all-features compile failure cannot be mistaken for a zero-test success. Session `79424` is historical exchange-only evidence; final current-source eight-law evidence is session `95998`.

## Current readiness

- Public issuer: mounted at `POST /spaces/{space_id}/documents/{document_id}/open-plan`; strict authenticated server authority exists, but the handler returns `catalog-unavailable` when the verified generation fence is false.
- Plan-to-socket-grant exchange: mounted at `POST /spaces/{space_id}/documents/{document_id}/socket-grants` and protected by the same generation/readiness fence plus the exact authenticated private plan authority.
- Verified openable catalog: implemented as an immutable process-lifetime projection of a completely verified trusted bundle. Ordinary startup currently supplies no linked native codec bindings, so a configured bundle cannot silently become usable through ambient registration.
- Document-open readiness: both `openPlan=false` and `openPlanExchange=false` for the ordinary unconfigured startup. They become true together only when startup retains a verified nonempty generation; the handlers independently enforce the same fence.
- D0 source and independent neutral oracle: implemented; historical all-features qualification remains red at session `45804`.
- D1 server issuer/exchange/consume-authority source: implemented. Focused loader/route/consume laws and the final registered eight-law default-feature gate are green in sessions `53281`, `65885`, `095981`, and `95998`.
- Browser/native/WGPU/MCP acquisition, renderer launch, tail transport and native mount: not claimed or promoted by this packet.

### Launch generation

`bun nx run @semio-tech/plugin-registry:generate --skip-nx-cache` session `44631` exited 0 and regenerated the catalog plus `.vscode/launch.json` from the seed, including `os-hub:open-plan-server-check`. Final current-source freshness session `4b3212`, running `bun nx run @semio-tech/plugin-registry:check-generated --skip-nx-cache`, exited 0 with `plugin registry generated catalog and launch bytes are fresh.`

## Remaining dependency boundary

This packet stops at the server HTTP authority boundary. A real production profile must explicitly link native codecs and supply a verified bundle with nonempty open targets before either feature is advertised. React/native/WGPU/MCP must then acquire the plan and derived SocketGrant per dial, obey checkpoint/tail revalidation, and mount only the exact returned target. Those client transports, renderer activation and native effect/event attachment remain separate work; there is no compatibility route, raw descriptor/factory response or readiness overclaim.

## Public parent-dialect follow-on

Current source now projects the complete verified application parent dialect through the public Rust and TypeScript `DocumentOpenPlanV1` contract. `DocumentOpenParentDialectV1` is a strict three-field shared DTO (`artifactKind`, `standard`, `subset`), requires artifact-kind equality plus nonempty, UTF-8-byte-bounded, control-free and trim-free values, and is reexported from the public directory facade. The Hub copies only the already verified catalog selection into `public_plan`; the authenticated receipt exchange and socket validity fences compare the same private dialect again. Native `DirectoryClient` retains the public value in `DocumentSocketAuthorityV1`; browser installed-target admission compares all three fields before receipt exchange. No compatibility or caller-selected dialect form was added.

The neutral plan corpus commits the parent dialect in catalog generation framing and contains eight malformed catalog rows, four public-plan validation/equality cases and three consume-time substitutions. The browser schema/fixture requires the same DTO on both installed target and plan and includes three equality substitutions plus control and trim hostiles. The independent source oracle command:

```text
bun ./📜️script.ts open-plan-server-check --oracle-only
```

exited 0 on current source and reported descriptor `1`, catalog `2`, receipt `1`, independent codecs `3`, issuer `11`, consume `9`, negative `20`, exchange-negative `5`, redaction `1`, plus production TypeScript parity with `15` plan and `5` exchange rejections. A separate AJV 2020 validation of the browser schema/fixture exited 0 with installed/plan parent equality, five hostile parent vectors and two structural malformed rejections. The production TypeScript parser directly accepted the valid plan, structurally rejected the three malformed parent cases, and deliberately left the syntactically valid standard substitution to the exact installed/catalog equality fence.

After the taxonomy owner repaired the selector-free canonical spellings, the registered root-routed oracle-only target exited 0 with the same counts. The registered framework OS focused test for `browser document open requires exact installed package artifact and surface authority` then exited 0 with one selected/one passed and 236 skipped. Native compile session `44281` exited 0 in 35.70 seconds for `semio-framework-os-kernel --lib` on the retained ticket target, proving the public Rust schema, DirectoryClient retention and store authority construction compile together; it is warnings-only build evidence, not a runtime assertion.

Registered current-source native session `99222` retained the oracle phases above, completed the first library selector build, and then exited 1 before the Hub selectors or any native law ran. The second selector's Hub-binary build stopped in the concurrently edited shared plugin crate with exactly two `E0425` diagnostics: `mounted` was out of scope at `plugin/🦀️.rs:22044`, and `_permit` was bound at `:22512` while `permit` was used at `:22562`. The stored Cargo diagnostic is under the retained ticket target's `.fingerprint/semio-framework-plugin-ea04a964c632e7d9/output-lib-semio_framework_plugin`. This is zero native assertions and does not supersede historical server session `95998`; no parent-dialect-owned compiler error surfaced. Hub20 and all-feature checks were not run from this terminal.

After the shared plugin owner reported its registered check green, warmed session `26716` reran the same registered target. The parent-dialect oracle and production TypeScript phases again passed with the exact counts above, but the first library selector build exited 101 in `semio-s-plugin-stdio` before discovery. The current source produced 52 `E0277` diagnostics; the first four are the HTML5 `any` mutation leaves `set-snapshot`, `set-element-name`, `set-text`, and `set-raw-text` not satisfying `MutationLeaf`, followed by aggregate repetitions. The retained diagnostic is `.fingerprint/semio-s-plugin-stdio-cf88cc8f0df9beac/output-lib-semio_s_plugin_stdio`. This second terminal also contains zero native assertions; Hub20 and all-feature remain unrun.

The future `DocumentExecutionTargetLeaseV1` remains the intended single immutable client comparison boundary documented in `📓️terra-client-execution-target-lease-blueprint.md`; this follow-on does not claim that larger browser/native lease cutover.

## Owned files

- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/📄️document-open-plan-v1.json`
- `🌎️hub/📦️packages/🦀️rust/📦️bin.rs`
- `🌎️hub/🗿️artifact-authority/🗂️trusted-catalog/🦀️.rs`
- `🌎️hub/🗿️artifact-authority/🗂️trusted-catalog/🧬️schema/🔣️bundle.schema.json`
- `🌎️hub/🗿️artifact-authority/🗂️trusted-catalog/🧪️fixtures/🧬️two-package/🔣️.json`
- `🌎️hub/📦️packages/🦀️rust/📜️script.ts`
- `🌎️hub/📦️packages/🦀️rust/📋️project.json`
- `.vscode/🧩️launch.seed.jsonc`
- `.vscode/launch.json` (generated only through the registry owner)
