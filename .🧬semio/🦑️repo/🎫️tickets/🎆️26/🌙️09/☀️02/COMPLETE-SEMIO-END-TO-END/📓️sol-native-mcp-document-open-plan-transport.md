# Sol Report — Native and MCP Document Open Plan Transport

Date: 2026-09-04  
Packet: `implement_socket_s3_resume` / native+MCP D1 cutover

## Boundary

The shared native `ArtifactHost` path now obtains every document connection through `DocumentOpenIntentV1 -> DocumentOpenPlanV1 -> DocumentPlanSocketGrantIntentV1 -> SocketGrantV1 -> /socket/v1`. MCP inherits the same actor and has no direct document-grant shortcut. This packet does not promote ordinary hub readiness: a hub without a verified nonempty openable catalog continues to advertise `openPlan=false` and rejects issuance.

The historical all-features D0 failure in session `45804` remains a separate red qualification and is not superseded by this transport work.

## Implemented source

- The native directory client percent-encodes UTF-8 scope components, caps every protected response at 64 KiB, clamps HTTP deadlines, validates the complete receipt-free plan authority, wipes the plan receipt and SocketGrant/header owners, and exchanges the receipt exactly once.
- Admission binds the credential origin. An arbitrary `PersistenceBinding::Hub.base_url` cannot redirect either the bearer-protected requests or the socket grant, and the actor derives its WebSocket origin from the accepted plan authority.
- The retained authority includes exact descriptor digest, catalog generation, package/plugin/version, artifact kind/schema/pack hash, app/window/surface/role/renderer, grants, checkpoint and revalidation generations. WGPU preclaims its verified program selection before `ArtifactHost::open`; mismatches fail before exchange.
- Root and per-document cancellation are checked before issuance, after the plan, after the receipt exchange, and before/after socket open. A post-exchange cancellation drops the zeroizing SocketGrant owner without exposing it to a URL or protocol header.
- The actor reissues a fresh plan and grant on each connection epoch. Wrong Session, malformed bootstrap, EOF, transport failure and authority expiry clear the socket actor and retained plan authority, requeue pending mutations, and prevent delivery until a fresh matching Session.
- MCP process-entry registration now publishes its structural probe codec synchronously through the same atomic codec-registry barrier; it no longer drops an unpolled registration future.
- The real native process oracle now requires three strict plans, three distinct one-use plan-receipt exchanges, three SocketGrants and three WebSocket epochs. It rejects any command before Session, verifies actor stamping and reconnect delivery, and fails if credential, plan receipt or SocketGrant text reaches stdout/stderr.
- The permanent `secure-local-smoke` gate runs the independent document-open neutral oracle, exact-selects six native Rust laws with a nonzero preflight, builds/directly supervises the WGPU and MCP children, and runs the process oracles.

## Current evidence

- Server catalog/issuer/exchange/consume gate: session `95998`, exit 0, eight exact laws. This is server-only evidence.
- Browser D1 transport is owned and reported separately by the browser lane.
- Unique native target: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🗑️generated/native-socket-grant-sol-target`.
- Session `88236` began before the final codec, actor-authority and process-oracle changes. It is compile-diagnostic evidence only and must not be credited as a current-source runtime terminal.

## Remaining runtime boundary

- A final current-source unique-target native build, the six exact native laws, and the three-epoch WGPU process oracle must complete.
- The normal secure-local hub intentionally has no linked verified openable catalog. Consequently a real-hub MCP D1 success cannot be claimed from that profile; the process must either run against an explicitly verified catalog profile or remain a truthful catalog-unavailable negative. No test-only catalog bypass or readiness overclaim is permitted.
- `ArtifactHost` still indexes open actors and surface preclaims by `document_id` rather than the full `(space_id, document_id)` scope. The current WGPU shell serializes one active document and one MCP workspace owns one space, so this packet does not claim the general cross-space collision resolved. A full-scope host-key migration and hostile duplicate-id law remain required before general multi-space native readiness.

## Owned source

- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Shell/🎯️targets/🧊️wgpu/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs`
- `🌎️hub/📦️packages/🦀️rust/📜️script.ts`
