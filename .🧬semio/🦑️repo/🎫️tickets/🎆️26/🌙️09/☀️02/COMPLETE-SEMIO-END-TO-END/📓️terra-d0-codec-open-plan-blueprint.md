# Terra D0 Codec And Complete Open-Plan Blueprint

Date: 2026-09-04  
Scope: current-tree, read-only D0/D1 audit after the in-flight native, MCP, and browser transport work. No product or test source was changed and no build, Cargo target, launcher, or runtime was run for this report. “Source-backed” below means only that the named current bytes contain the behavior; it is not runtime evidence.

## Decision

**D1’s public plan, issuer, receipt exchange, and plan-bound socket path are source-backed. D0 is still RED as a complete executable document-open path.** The first production blocker is deliberately fail-closed: the hub entrypoint’s `linked_native_codec_bindings()` returns `Vec::new()` at [`🌎️hub/📦️packages/🦀️rust/📦️bin.rs:393`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📦️bin.rs:393). A configured trusted catalog therefore has no native codec binding with which to authenticate declared document codecs; without a configured catalog, `open_plan_ready` is false at [`bin.rs:5281-5283`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📦️bin.rs:5281). No implementation should work around that absence by trusting a client-selected schema, plugin, or component.

The smallest safe completion is not another credential route. It is a schema-first **installed execution target** boundary: each renderer must prove that its locally installed codec and executable are the exact `package + artifact + surface` returned by the D1 plan, keyed by the full `(spaceId, documentId)` scope, before it exchanges the plan receipt or mutates a mounted document. The hub must be able to load the same target from a verified bundle with the exact native codec binding. D1’s one-use receipt and SocketGrant then remain the only network authority.

## Current Source Map

| Boundary | Current source-backed behavior | Classification |
| --- | --- | --- |
| Shared D1 DTO | Strict Rust values and validation define IDs (256 UTF-8 bytes), client instance (128), 30 s plan TTL, exact-safe integers, receipt grammar, hashes, role/write relationship, and optional checkpoint frontier at [`📇️directory/🧬️schema/🦀️.rs:490-760`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs:490). TS has a separately implemented strict parser at [`🧬️schema/🟦️.ts:410-492`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts:410). | Source-backed DTO, not cross-runtime proof. |
| Hub issuer/exchange/upgrade | `POST /open-plan` authenticates, revalidates, resolves the immutable catalog, caps request/deadline, derives an actor, and records a short-lived receipt at [`bin.rs:1985-2123`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📦️bin.rs:1985). Exchange revalidates descriptor/catalog/revision and mints one plan-bound SocketGrant at [`bin.rs:2131-2213`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📦️bin.rs:2131). Upgrade rechecks descriptor/catalog/revision/checkpoint before consumption at [`bin.rs:2600-2655`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📦️bin.rs:2600). | Source-backed D1, production unavailable until D0 catalog binding exists. |
| Trusted catalog/codec registry | The catalog loader verifies bounded bundle inputs, exact native bindings, open targets, and atomically preflights/registers codecs at [`🗂️trusted-catalog/🦀️.rs:337-476`](/Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🗂️trusted-catalog/🦀️.rs:337). The registry itself prevents conflicting schema replacement at [`🏪️store/🦀️.rs:9433-9524`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:9433). | Correct fail-closed machinery; no live hub binding producer. |
| Native D1 client/store | `DirectoryClient::admit_document_socket` performs plan then receipt exchange, validates local schema/hash/surface names, uses origin-bound protected requests, and wipes receipt/grant request wrappers at [`📇️directory/🔌️client/🦀️.rs:745-860`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs:745). The store derives a percent-encoded v1 socket path and rechecks returned origin/scope/schema/hash/surface before `SocketHelloV1` at [`🏪️store/🔄️sync/🦀️.rs:1921-2034`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs:1921). | Partial D0; execution identity and scope ownership remain red. |
| Native WGPU mount | WGPU derives a local plugin/package/app/window/surface expectation at [`Shell/🎯targets/🧊wgpu/🦀️.rs:540-596`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Shell/🎯️targets/🧊️wgpu/🦀️.rs:540), but it selects the current session before D1 admission and calls the retired `attach_backbone` stub while opening at [`...wgpu/🦀️.rs:3678-3702`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Shell/🎯️targets/🧊️wgpu/🦀️.rs:3678). That stub always errors at [`ProgramBridge/.../🦀️.rs:281-284`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs:281). | RED: no runnable native document mount. |
| Browser TS fallback | The fallback worker posts D1 open-plan then receipt exchange through the private broker, bounds and clears raw response bytes, and connects credential-free at [`🟦️backbone-worker.ts:383-466`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️backbone-worker.ts:383). | Partial source implementation only. |
| Browser actual worker choice | The Rust host still has only `Open/Close/Send` and no broker/D1 state at [`🏪️store/👷️worker/🦀️.rs:23-89`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🦀️.rs:23), but the current dispatcher explicitly routes every hub-bound `Open/Send/Close` to the TypeScript owner even when that host resolves ([`🟦️backbone-worker.ts:61-112`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️backbone-worker.ts:61)); a focused source law covers the resolved-host case at [`:2360-2378`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️backbone-worker.ts:2360). | Source-closed for the prior Rust-worker D1 bypass; still not a real-hub or full-execution-identity proof. |
| MCP | MCP claims the protected `mcp` credential and builds a native directory client/grant source at [`🌉️mcp/🦀️.rs:655-660`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🦀️.rs:655) and [`workspace/🔗️remote/🦀️.rs:477-505`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🔗️remote/🦀️.rs:477). It opens only its own `os.agent.probe/v1` `ProbeStore` with no exact D1 surface expectation at [`workspace/🦀️.rs:1330-1360`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs:1330), and explicitly says a non-open hub document’s bytes remain unavailable until P4-B at [`:1308-1324`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs:1308). | Not a generic D0 consumer; no current MCP open claim. |
| Tauri | No current OS Tauri entrypoint or consumer was found in the audited tree. | No surface to accept or gate. |

## Material D0 Gaps

### 1. Hub production catalog closure is absent

`VerifiedTrustedCatalog` correctly rejects an open target without a matching native codec binding, but the only production supplier is the empty `linked_native_codec_bindings`. This is not a test-only concern: hub startup calls that function before it configures artifact authority ([`bin.rs:5248-5257`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📦️bin.rs:5248)). A bundle/profile therefore cannot establish `open_plan_ready` with executable native codecs.

Required outcome:

- Generate or statically link `NativeCodecBinding` records from the exact signed/verified package-descriptor receipt, not from plugin IDs, filenames, or a runtime WASM lookup.
- Each binding must include the exact plugin/package/artifact kind/schema and the registered `ArtifactCodec` function pointers; it must be preflighted as one catalog assembly.
- Fail startup/readiness closed for missing, duplicate, extra, zero-hash, or mismatched bindings. Do not expose a plan for an unbound target.

### 2. A client proves names, not installed executable bytes

The public plan includes `componentSha256`, `componentBlake3`, and `descriptorByteSha256`, but outside the schema and test fixtures they are not compared by a native, browser, or MCP execution consumer. Native `DocumentSocketSurfaceExpectationV1` carries plugin/package/version/app/window/renderer but no component or descriptor hashes ([`📇️directory/🔌️client/🦀️.rs:287-349`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs:287)); WGPU derives precisely that reduced tuple. Browser `ArtifactActorConfig` has only document, schema, bindings, actor, and optional pack hash ([`🟦️.ts:559-572`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️.ts:559)), and `documentOpenPlanAuthority` checks schema, optional requested surface, React target, and pack hash only ([`🟦️backbone-worker.ts:418-435`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️backbone-worker.ts:418)).

That means a plan can be structurally valid and network-authorized while the client has not proved that the code it activates is the exact catalog-selected component. This is the core D0 execution-authority gap.

### 3. Runtime ownership discards `spaceId`

D1’s scope is `(spaceId, documentId)`, and the hub’s database key is length-delimited ([`bin.rs:401-408`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📦️bin.rs:401)). Both client actor registries key only on `documentId`:

- Native `ArtifactHostState.documents` and `document_socket_surfaces` are `HashMap<String, …>` ([`🏪️store/🔄️sync/🦀️.rs:1023-1036`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs:1023)); opening removes a pending surface and closes an actor by that ID ([`:1092-1111`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs:1092)).
- Browser state is `artifacts`/`BroadcastChannel` keyed by `config.documentId` ([`🟦️backbone-worker.ts:1530-1565`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️backbone-worker.ts:1530)).
- The MCP `open_probes` map is likewise keyed only by artifact id ([`workspace/🦀️.rs:1165-1175`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs:1165)).

Opening `space-a/doc-x` and `space-b/doc-x` can therefore replace, consume, or cross-notify the wrong local actor before the server can reject the second socket. This is a correctness and authority failure, not merely a display-name collision.

### 4. WGPU opens in the wrong order and cannot finish

`Shell::open_document` detaches the current document, selects its already-mounted plugin/session, binds its surface, opens the actor, and only then calls `plugin.attach_backbone` ([`...wgpu/🦀️.rs:3678-3702`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Shell/🎯️targets/🧊️wgpu/🦀️.rs:3678)). It neither chooses the package/app from the returned plan nor retains plan component hashes. The bridge’s only native implementation returns an error, so this path cannot attach the actor to a live app.

Do not “accept” a source-level socket exchange as native document opening. It is only a pre-connect transport component until event-driven backbone attachment and plan-selected mounting exist.

### 5. Browser now has one source-selected hub owner, but no complete execution match

The current dispatcher source-closes the earlier Rust-worker bypass: a hub binding is always owned by the TypeScript D1 transport, even if the Rust module resolved. It still stores that ownership by bare `documentId`, so P1 remains required. The browser test harness also serves a synthetic plan, socket grant, and WebSocket instead of a real hub ([`🌎️hub/📦️packages/🦀️rust/📜️script.ts:1720-1810`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:1720)). Its useful result is D1 owner-routing plus fallback worker behavior, not deployed artifact/catalog activation or real checkpoint/frontier proof.

The fallback keeps raw response `Uint8Array`s bounded/cleared, but decoded receipt strings and `JSON.stringify` exchange body cannot be zeroed by JavaScript. Acceptance must therefore require no receipt/grant in URL, storage, logs, `postMessage`, plugin channels, or UI-visible event values; it cannot claim heap zeroization unless the receipt decoder/exchange body moves to wipeable byte storage.

### 6. MCP is deliberately not a headless open target yet

The D1 renderer enum has only `react`, `wgpu`, and `wasm` ([`🧬schema/🦀️.rs:511-518`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs:511)). MCP must not silently accept a UI target merely because its generic `DocumentSocketExpectationV1.surface` is `None`. P4-B/C provide bounded canonical-pair mechanics, not permission to expose an unscoped raw document resource or to claim currentness beyond their documented checkpoint scope.

A real headless target needs an explicit catalog representation, exact native codec identity, a P4-B verified pair mount tied to the plan checkpoint, and a scoped actor key. Until then `os.agent.probe/v1` is a narrow MCP-owned diagnostic/document path, not general document opening.

## Schema-First Target Contract

Keep D1’s public network values unchanged. Add a local, language-neutral execution projection, generated from the same trusted bundle/descriptor receipt, rather than introducing a client-controlled open request:

```text
DocumentRuntimeKeyV1
  space_id: UTF-8 text
  document_id: UTF-8 text
  canonical_key: v1:<space-byte-length>:<document-byte-length>:<space><document>

InstalledDocumentExecutionTargetV1
  package: DocumentOpenPackageV1               // all three immutable hashes
  artifact: DocumentOpenArtifactV1             // kind, schema, pack-schema hash
  surface: DocumentOpenSurfaceV1               // or explicit Headless variant below
  codec: implementation-private ArtifactCodec handle
  executable: implementation-private verified component/descriptor handle

DocumentOpenMountAuthorityV1
  runtime_key: DocumentRuntimeKeyV1
  plan projection excluding receipt
  exchanged SocketGrantReceiptV1 in wipeable storage
  local exact target match
  cancellation/claim generation
```

`InstalledDocumentExecutionTargetV1` must use equality for every public `package`, `artifact`, and `surface` field; it is not satisfied by a matching schema, plugin ID, or semantic version alone. The executable/codec handles remain implementation-private and never enter the public plan, JSON fixture, WebSocket URL, logs, or MCP resource catalog.

For MCP, either add a first-class `headless` renderer target with no UI `appId/windowKindId`, or define a separate discriminated `DocumentOpenExecutionTargetV1::Headless` in the shared schema. Do not overload `wasm`, `react`, or an absent `surface` to mean headless. The catalog must reject a headless target without its exact native codec and MCP-owned capability policy.

## Small Independent Implementation Partitions

### P0 — Verified Hub Codec Closure (first; blocks actual D1 readiness)

Own:

- [`🌎️hub/📦️packages/🦀️rust/📦️bin.rs`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📦️bin.rs) production binding supplier and generated native binding receipt;
- [`🌎️hub/🗿️artifact-authority/🗂️trusted-catalog/🦀️.rs`](/Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🗂️trusted-catalog/🦀️.rs) only as needed to retain exact bundle-to-binding verification;
- neutral catalog fixture/oracle rows that show exact success plus missing/extra/duplicate/mismatched/zero-hash failure.

Invariant: all selected package codecs and all selected open targets publish together, or no catalog/open-plan readiness is published. A configured bundle must never become “ready” through an empty binding set.

Focused laws: actual production binding list has at least one valid target; every target’s codec is registered; startup with each hostile binding fails closed; source/listing gate selects exactly one law for each case. This packet may proceed independently of renderer work.

### P1 — Shared Runtime Scope And Installed-Target Registry

Own the shared `DocumentRuntimeKeyV1` fixture/parser and the non-network `InstalledDocumentExecutionTargetV1` match API. Migrate native `ArtifactHost` maps, browser `artifacts`/`BroadcastChannel`, and future MCP maps from bare document IDs to this key. Do not perform a compatibility migration: all current callers must provide a full scope for hub bindings.

Invariant: two spaces with the same document ID own independent pending surfaces, actors, events, reconnect state, cache entries, and teardown. A cancellation/close can release only its own key and generation.

Neutral vectors: delimiter/colon/unicode values, same document in two spaces, same scope double-open, stale generation close, and key byte-length disagreement. This partition can run in parallel with P0; P2/P3/P4 depend on it.

### P2 — Native WGPU Plan-Selected Mount And Event Backbone

First replace the retired `ProgramBridge::attach_backbone` stub with the documented event-driven port. Then restructure WGPU’s open flow:

1. retain the existing document until a new request reaches a committed local claim;
2. enumerate only verified installed targets, submit D1 intent with an optional surface preference, and validate the complete returned target including all component/descriptor hashes;
3. claim `(space, document)` once, exchange the receipt only after local exact-match validation, and connect;
4. attach the plan-selected plugin/app/window only after successful receipt exchange and exact `Session` actor confirmation; and
5. on denial, cancellation, expiry, stale catalog/checkpoint, attach failure, or late socket success, close/wipe/release the new claim without dropping the old mounted document.

Focused laws must prove: no plan exchange for a locally mismatched component hash; no attach/hello before matching Session; one close for a late cancelled socket; two same-ID/different-space native opens remain isolated; stale plan/catalog/revision/checkpoint rejects before visible mount; and no raw receipt/grant appears in child arguments, env, URI, diagnostics, or plugin messages.

### P3 — Browser TypeScript Hub Owner With Full Target Match

The source has selected the safe owner: hub requests route to the TypeScript D1 worker, while Rust may serve only non-hub work. Preserve this explicit routing and migrate its ownership map to `DocumentRuntimeKeyV1`; do not reintroduce a fallback that lets Rust open a hub binding without the broker port.

Extend the TypeScript input from schema/pack hash to an immutable installed execution target and require complete plan equality before exchange. The worker owns receipt bytes and the broker port; the renderer/plugin never receives either. Retain the existing broker proof isolation.

Focused laws must execute the resolved-Rust-host hub dispatch as well as the unavailable-host case and prove both use the TypeScript D1 owner. They must use a real D1 hub with P0 bindings, not only a synthetic local responder, and include plus-one response caps, malformed UTF-8/JSON, non-lowercase/zero hash, unknown field, expired receipt, scope collision, plan/package mismatch, cancellation after plan/before exchange/after exchange, stale Session actor, zero/mismatched frontier, and URL/storage/log/worker-message redaction.

### P4 — Explicit MCP Headless Open (after P0, P1, and P4-B materialized pair)

Do not alter MCP’s existing P4-B nonclaims. Add a headless catalog target, a one-use D1 client that supplies the exact headless target, and a scoped mount actor. After D1 validation it must obtain the bounded P4-B pair and compare its authenticated scope, descriptor digest, checkpoint identity/aggregate hash, and baseline frontier with the plan before decoding. It may expose typed derived tool results only; it must not publish raw pack/spr as an arbitrary MCP resource or claim an unauthenticated “latest” value.

Focused laws: no plan for a UI target; no P4-B fetch before plan/target validation; plan/P4-B descriptor/checkpoint/frontier mismatch fails closed; revoke/reconnect invalidates mounted state and retries from a fresh plan; concurrent mounts have one scope-key winner; cancellation closes a late dial and avoids a state refresh; no receipt/grant/raw pair bytes in JSON-RPC output or resource enumeration.

### P5 — Real Cross-Surface Acceptance

After P0–P4, run a single real-process scenario per shipped surface: launch a hub with generated verified bindings, issue a descriptor and checkpoint, open a document, mutate/observe with an authenticated peer, revoke/reopen, and assert final resource teardown. Browser must consume its shipped generated worker; WGPU must run the real launcher/direct child; MCP must use a real headless target and P4-B pair. This is the first point at which “end-to-end document open” is an honest claim.

## Hostile Law And Neutral Oracle Requirements

Extend the existing neutral fixture [`🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/📄️document-open-plan-v1.json`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/📄️document-open-plan-v1.json), rather than creating one language’s shadow format. The independent oracle must derive results from the schema and canonical byte rules, not import production Rust/TS validation.

It needs vectors for:

- every exact text/hash/receipt bound and unknown/duplicate/missing field; UTF-8 byte length rather than JS code-unit length; lower-case nonzero 32-byte hashes; exact-safe integer edges;
- 8 KiB issuer/exchange request, 64 KiB native/browser response, catalog/open-target/ledger/binding capacity, bounded resolver and dial deadlines;
- full local target equality including component SHA-256, component BLAKE3, descriptor SHA-256, artifact kind/schema/pack hash, and all surface fields;
- scope-key ambiguity (empty/colon/unicode) and two-space same-ID isolation;
- receipt one use, replacement, expiry, concurrent exchange single winner, SocketGrant capacity failure with retryable receipt semantics defined explicitly, revocation during issuer/exchange/upgrade, and restart scope;
- checkpoint optionality stated explicitly. A plan without a checkpoint may establish only D1 transport admission; it must not be accepted as a materialized-current document snapshot. A P4-B mount must bind descriptor digest, checkpoint ID, aggregate hash, document ID, nonzero chain hash, and exact baseline frontier;
- cancellation before issuer, while waiting, after plan before exchange, after exchange before upgrade, and after late WebSocket success; exactly one close and no Hello/mount after cancellation;
- raw-proof/receipt/grant absence from URLs, headers other than the second WebSocket subprotocol offer, persisted stores, diagnostics, child environment/argv, browser ports, plugin messages, and MCP result/resource output.

## Registered Gates And Required Registrations

Current registration exists but is not evidence from this audit:

- `os-hub:open-plan-check` invokes the neutral oracle plus nine exact all-feature Rust selections at [`🌎️hub/📦️packages/🦀️rust/📜️script.ts:2655-2698`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:2655). It is useful for D1 source law selection, but its catalog laws can use test catalogs and do not prove `linked_native_codec_bindings`.
- `os-hub:open-plan-server-check` selects eight exact server/catalog laws, but intentionally labels itself a default-feature subset at [`📜️script.ts:2700-2730`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:2700). It must not substitute for the all-feature target.
- `os-hub:browser-document-open-check` runs a neutral oracle, synthetic-browser-worker runtime, browser tests, and the server subset at [`📜️script.ts:2733-2740`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:2733). It presently does not prove real hub/catalog/Rust-worker parity.
- Launch entries exist at [`.vscode/launch.json:4433`](/Users/ueli/Documents/semio/.vscode/launch.json:4433), [`:4444`](/Users/ueli/Documents/semio/.vscode/launch.json:4444), and [`:4455`](/Users/ueli/Documents/semio/.vscode/launch.json:4455).

Add proposed dedicated targets only through the owning `📜️script.ts`, then register them in the existing launch ordering:

1. `os-hub:d0-codec-open-check` — P0 binding fixture/oracle plus an actual configured hub readiness probe;
2. `os-native:document-open-check` — P1/P2 real native runtime key, complete target match, cancellation, and event-backbone delivery;
3. extend `os-hub:browser-document-open-check` with generated-worker selection and real-hub P0 fixture, rather than adding a duplicate fallback-only check;
4. `os-mcp:headless-document-open-check` — P4 plan/P4-B/headless typed result and redaction law; and
5. one launch-registered cross-surface D0 acceptance target only after those focused gates are green.

Every target must first list and require exactly one fully qualified test per suffix, then exact-run it. A check that skips because its generated worker, trusted bundle, direct child, or real hub is unavailable is an honest failure/inconclusive result, never a pass.

## Acceptance Boundary And Nonclaims

Accept D0 only when P0 has made a real configured hub `open_plan` ready, every shipped surface has one authority-preserving implementation path, each mount matches an installed verified target and complete scope, WGPU can attach its event backbone, MCP has an explicit headless contract and P4-B binding, and the real-process gates above pass.

Until then, do **not** claim:

- that current D1 source provides a runnable production document open;
- that schema/package-name or pack-hash matching authenticates an installed executable;
- that `documentId` alone is a safe cross-space actor/cache key;
- that the browser fallback test proves a resolved Rust worker or a real hub/catalog/checkpoint;
- that MCP opens arbitrary documents, exposes raw bytes, or provides currentness beyond its accepted P4 scope; or
- that source review or registered gates are runtime evidence.
