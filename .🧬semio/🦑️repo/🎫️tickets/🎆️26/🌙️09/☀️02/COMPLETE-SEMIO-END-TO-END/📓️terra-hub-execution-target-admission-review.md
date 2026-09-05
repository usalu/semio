# Terra Hub Execution-Target Admission Review

## Verdict

The browser contract intentionally uses **three repeated `DocumentOpenIntentV1` requests, not the plan receipt**, for `manifest`, `component`, and `descriptor`. The receipt is sent once, only after all three verify, in the `DocumentPlanSocketGrantIntentV1` exchange. Reusing the receipt exchange for target reads would be incorrect: the exchange consumes a single-use plan, while the browser requires three independently authenticated target reads.

The newly added Hub target routes have the right outer shape and the right catalog seam. At this review point, their production path is still **RED** for the local relay: it neither admits the three paths nor preserves required `Content-Length`, and its one MiB response ceiling conflicts with the 64 MiB component contract. The target selection helper also needs the plan issuer’s final revalidation/revision fence immediately before it releases an asset.

This was read-only source review; no build or runtime test was run.

## Actual browser wire contract

| Order | Request body and route | Required response | Why it is not a receipt exchange |
| --- | --- | --- | --- |
| 1 | `POST /spaces/{space}/documents/{document}/open-plan` with strict `DocumentOpenIntentV1` | `DocumentOpenPlanV1`, including the opaque `receipt` | Hub selects/authenticates the plan. |
| 2 | `POST …/execution-target/manifest` with the **same intent** | `DocumentExecutionTargetLeaseFieldsV1`, receipt-free | Browser parses it strictly and requires equality with `leaseFieldsFromPlanV1(plan, manifest byte lengths)`. |
| 3 | `POST …/execution-target/component` with the same intent | raw component bytes and exact `Content-Length` | Browser streams exactly the manifest-declared size, then checks SHA-256 and BLAKE3. |
| 4 | `POST …/execution-target/descriptor` with the same intent | raw canonical Pack descriptor and exact `Content-Length` | Browser checks SHA-256, canonical Pack re-encoding, package/app/dialect/window/artifact relations. |
| 5 | `POST …/socket-grants` with `DocumentPlanSocketGrantIntentV1 { planReceipt }` | one socket grant | This alone consumes the plan. |

Evidence is explicit in `🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:568-575,656-735,774-838`. The intent schema uses `deny_unknown_fields` at `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs:1333-1343`; a `planReceipt`, package digest, path, or added selector in a target request is rejected rather than ignored. The plan receipt’s distinct schema and one-use exchange are at `…/schema/🦀️.rs:1462-1469,1602-1609` and `🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:2293-2337`.

This design deliberately makes a rotation/revocation between target requests produce a mismatched/denied body. The browser’s all-field equality and dual-hash checks prevent a mixed triple from becoming a lease. It is not correct to add a receipt to target URLs or bodies: that either makes the receipt a resource selector or consumes it before socket exchange.

## What the target endpoint must verify before every body

The new `document_execution_target_selection` at `🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:2373-2471` already performs most required work:

1. Bounded, JSON-only strict intent parsing; path scope must equal body scope (`:2380-2396`).
2. Re-authentication of the document-scoped session/share, bounded admission gate, and initial subject revalidation (`:2400-2409`).
3. Durable descriptor load, server-side writable role derivation, and exact catalog selection through `assets_for_current_selection` (`:2410-2421`). That accessor is correctly selection-only: `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:322-343` accepts no package, hash, file path, URL, or receipt.
4. Fresh directory revision, descriptor digest, checkpoint, session/share generation, retained byte bounds, and strict lease-field validation (`bin.rs:2422-2471`).

The endpoint should use this selection path—not `issue_document_plan_socket_grant_inner`, not `DocumentOpenPlanLedgerV1::exchange`, and not a direct package lookup. `issue_document_open_plan_inner` remains the appropriate reference for authorization ordering and final consistency (`bin.rs:2191-2285`).

### Required final fence

There is an actionable race in the current target helper: after its initial revalidation it awaits descriptor, revision, and checkpoint reads, then returns bytes without a final revision equality or subject revalidation (`bin.rs:2410-2471`). The plan issuer does both final checks at `bin.rs:2269-2276` before it releases/records a plan.

Immediately after the target helper has gathered its fields and before it returns `VerifiedExecutionTargetAssets`, it must repeat the equivalent fence:

1. `head_seq()` still equals the recorded `directory_revision` (otherwise `stale`);
2. `subject.revalidate(directory, audience, now_ms())` is still active (otherwise `denied`/deadline).

That prevents releasing component or descriptor bytes after a membership/session/share revocation or durable descriptor mutation that raced the first check. It also keeps the target manifest’s revalidation/checkpoint projection meaningful rather than relying only on the later socket exchange to reject a now-stale plan.

The clean bounded refactor is a private `resolve_authenticated_current_document_open` helper factored from `issue_document_open_plan_inner` and `document_execution_target_selection`. It should return the authenticated subject/audience admission, durable descriptor, exact selected catalog record, captured revision, digest, checkpoint and revalidation projection. Plan issue adds actor/receipt construction; target issue adds `assets_for_current_selection`; both call one common final-fence helper. Do not put the plan receipt in this helper’s input or output.

## Review of the current route edits

The following route work is sound and should be retained:

- Hub exposes only the fixed `POST` names at `🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:6369-6371`; it does not add an arbitrary target suffix or package route.
- `issue_document_execution_target` rejects query strings, bounds body intake to 8 KiB, has a ten-second deadline, and delegates all three assets to the common authenticated selection helper (`bin.rs:2474-2499`).
- Binary target responses set octet-stream, exact `Content-Length`, and `no-store` (`bin.rs:2501-2512`). Those headers match the browser reader, which rejects absent or non-exact lengths at `backbone-worker.ts:623-648`.
- The trusted catalog returns retained bytes only after resolving the durable descriptor and role to one exact selection; it is not a public package file API (`trusted-catalog/🦀️.rs:322-343`).

### Relay blockers to fix with the route work

1. `localRelayUpstreamPath` currently permits only `open-plan` and `socket-grants`, not any execution-target path (`🌎️hub/📦️packages/🦀️rust/📜️script.ts:332-343`). Add exactly the three query-free `POST` expressions. Do not allow a wildcard `execution-target/*`.
2. The relay rebuilds the upstream response without `Content-Length` (`…/📜️script.ts:470-481`). The browser’s component/descriptor reader rejects it before hashing. Return `content-length: String(responseBody.byteLength)` for target bodies (and preserve the matching binary content type).
3. The relay buffers each upstream response with `LOCAL_RELAY_MAX_BODY_BYTES = 1 MiB` (`…/📜️script.ts:110,372-395`), while the public target contract admits a 64 MiB component (`🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts:1157-1176`). Give the three exact paths a 64 MiB response bound; leave the normal relay limit unchanged.
4. The relay upstream deadline is two seconds (`…/📜️script.ts:115,471-478`) while the Hub target endpoint and browser verifier each permit ten seconds (`bin.rs:2341-2342,2485-2499`; `backbone-worker.ts:307,698-700`). Define a route-specific budget with transport slack, or lower the Hub target budget below the relay’s; otherwise a valid bounded target read can become a relay 503 merely through timing.

## First hostile laws

1. `execution_target_body_is_repeated_intent_never_plan_receipt` — all three paths accept the exact intent and reject receipt/unknown selector fields without consuming the already-issued plan; the subsequent socket exchange consumes it exactly once.
2. `execution_target_selection_revalidates_after_checkpoint_read` — hold the target helper after its first revalidation, revoke the session/share or mutate descriptor/membership, then release it; all assets must deny/stale and emit no body.
3. `execution_target_triple_cannot_mix_catalog_or_role_generations` — rotate catalog/descriptor or demote author between manifest/component/descriptor; browser must never mint a lease, and old plan socket exchange must fail stale.
4. `local_relay_preserves_exact_binary_length_and_64mib_ceiling_only_for_targets` — a component over 1 MiB and within 64 MiB arrives with exact `Content-Length`; unrelated relay response bounds remain one MiB.
5. `execution_target_routes_reject_cross_scope_query_and_unselected_surface` — wrong body scope, query, non-POST, missing/duplicate content type, nonmember, viewer-to-editor request, and a package/digest/path field all fail without exposing a target body.

## Current boundary

This admission slice can prove that the plan-selected raw component and descriptor are authenticated and byte-verifiable. It does not yet prove browser component activation: the separately audited JCO bridge/core-WASM closure and private renderer handoff remain required before any Map render claim.

## Follow-up: landed local relay recheck

The relay changes are now present. They resolve the three original relay blockers, subject to the test gaps below:

| Policy | Current implementation | Assessment |
|---|---|---|
| Exact paths | Only `POST`, no query, and precisely `manifest`, `component`, or `descriptor`; decoded space/document ids reject empty, dot, separators, controls, percent and query fragments | Correctly bounded at `🌎️hub/📦️packages/🦀️rust/📜️script.ts:338-369` |
| Request bytes | Target requests are capped at 8 KiB before broker-proof consumption and again while streaming the body | Correct at `📜️script.ts:470-497` |
| Response bytes | Manifest/component/descriptor are separately capped at 8 KiB/64 MiB/4 MiB, with a rebuilt exact `content-length`, preserved content type and `no-store` | Correct at `📜️script.ts:470-508` |
| Heavy capacity | The target asset lane admits at most two in-flight requests, independently of the ordinary 64-request relay lane; both counters release in `finally` | Correct counter/rachet order at `📜️script.ts:465-518` |
| Deadline and abort | Target fetches get 9 seconds, request abort feeds an upstream `AbortController`, and stop aborts all outstanding controllers | Correct relay wiring at `📜️script.ts:492-515`; current Hub budget is still 10 seconds at `🚀️bin.rs:2361-2362`, so it does not yet satisfy the fixture's required 8-second Hub / 9-second relay ordering |

The proof ratchet ordering is also correct: malformed/oversized and saturated target requests return before consuming a proof; every admitted request advances it before forwarding. A client abort after admission is deliberately terminal for that browser-proof epoch: the worker clears its proof before dispatch and requires rebootstrap whenever its acknowledgement is lost (`🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:355-396`). This prevents replay after a relay-side abort.

### Remaining relay/TDD gaps

The new `execution-target-relay-check` is a real Bun relay process, but its upstream is synthetic. It explicitly accepts body `{}` at `🌎️hub/📦️packages/🦀️rust/📜️script.ts:2571-2581`; that body is rejected by the real Hub, which accepts a strict nonempty `DocumentOpenIntentV1` at `🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:2371-2408`. The production browser does send exactly that repeated intent, not a plan receipt, at `🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:568-575`. Thus the test proves relay transparency, not the Hub-target request contract.

The saturation test holds two component-route requests but gives each a seven-byte response (`📜️script.ts:2614-2635`). It proves target-class capacity and no-proof-consumption on the third request, but not behavior at a substantial component body. More importantly, it has no target-specific client-abort/slot-recovery proof. Generic browser-broker coverage confirms cancel-after-send prevents proof replay, but does not establish that an aborted target request releases this two-slot lane before a replacement request.

The current Hub target helper still does its initial subject revalidation, descriptor/catalog lookup and revision/checkpoint reads, then returns the selected bytes without the final subject/revision fence (`🚀️bin.rs:2400-2474`). The planned final fence is necessary: repeat `subject.revalidate` and require `head_seq()` to equal the captured revision immediately before releasing any selected target body. It must run inside the Hub deadline, not in the relay.

### First bounded follow-up laws

1. `execution_target_relay_forwards_only_canonical_repeated_open_intent` — use the actual canonical `DocumentOpenIntentV1`, verify the relay preserves exact bytes and content type, and have the real Hub target handler reject `{}`, a receipt exchange body, and any added package/digest/path field before bytes.
2. `execution_target_relay_abort_releases_target_slot_and_requires_rebootstrap` — hold one component upstream request, abort its browser client after upstream admission, observe upstream cancellation, and prove a newly bootstrapped broker can fill the released second slot without a third target body slipping through.
3. `execution_target_relay_two_component_class_requests_never_admit_a_third` — retain the existing two-slot test but label it as component-class capacity and include a response safely above the ordinary 1 MiB cap; do not claim a full 64 MiB allocation unless that allocation actually runs.
4. `execution_target_hub_final_auth_and_revision_fence_precedes_body` — gate target selection after its initial checks, revoke/demote or advance the directory revision, then release; all three endpoints must deny/stale without a manifest or byte body.
5. `execution_target_budget_is_hub_8s_then_relay_9s` — tie the language-neutral fixture values to the Hub/relay constants and prove a target request after the ordinary 2-second relay deadline but before the target budget remains eligible, while Hub is bounded first at 8 seconds.

### Exact encoded-path policy mismatch

The fixture schema's `admittedRoute` pattern currently rejects every raw percent escape, while production decodes a captured id and validates the decoded form (`📜️script.ts:338-346`). Consequently production accepts `%3A` for a colon-bearing id—as required by the worker's `encodeURIComponent` path construction—but also accepts noncanonical spellings such as `studi%6f` and lowercase `%3a`. The fixture has no colon/unicode or noncanonical-escape case, so its independent grammar does not model the production policy.

Choose one policy explicitly. The better exact-wire policy is `encodeURIComponent(decoded) === encoded`: it retains canonical `%3A` and UTF-8 encodings needed for legal ids, while rejecting encoded unreserved characters and lowercase escape aliases. Then add positive colon/unicode and negative encoded-unreserved/lowercase-escape vectors. Do not ban all `%` in production, because that would reject the browser's canonical path for otherwise valid ids.

## Final recheck of the subsequent Hub fence edit

The follow-up Hub edit resolves the deadline and final-fence findings in source:

- Hub is now bounded at **8 seconds** (`🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:2360-2362`), leaving one second of relay slack at the independently fixture-checked 9-second relay cap (`🌎️hub/📦️packages/🦀️rust/📜️script.ts:112-119,2562-2567`).
- It captures `head_seq` before loading descriptor/checkpoint, then immediately before returning selected fields/assets re-runs subject revalidation and rejects any head change as `stale` (`🚀️bin.rs:2404-2416,2474-2488`). The fence therefore covers both revoked session/share authority and a directory event after initial selection.
- The native law `execution_target_selection_final_fence_matches_neutral_races` gates immediately before that final fence and drives unchanged, directory-advanced, session-revoked and task-cancelled corpus rows (`🚀️bin.rs:8255-8300`). It also proves no target read mints or consumes a plan receipt.
- The relay check now source-checks those shared limits/fence tokens and, in its `--native` mode, selects both the route and final-fence laws (`📜️script.ts:2555-2567,2652-2666`).

This closes the specific pre-body revalidation gap. The recheck remains read-only and did not execute the live Bun test or native laws. The two outstanding review items are test quality, not a new source correctness finding: use a real strict repeated intent instead of `{}` in the relay upstream, and add the target-specific abort/released-slot plus encoded-id policy laws described above.
