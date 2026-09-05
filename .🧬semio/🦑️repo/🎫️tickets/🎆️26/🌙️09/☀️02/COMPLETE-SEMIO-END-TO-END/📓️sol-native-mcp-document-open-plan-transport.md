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
- `ArtifactHost` keys hub actors and pending surface claims by the complete `(space_id, document_id)` scope. Local-only documents use a distinct `Local` key, and WGPU, the Rust worker and MCP retain the exact key for subscribe/send/presence/close. The same document id can therefore remain open in two spaces without one actor or surface authority replacing the other.
- MCP preclaims its own exact `os.mcp` probe package/app/window/editor/Wasm surface before the shared actor opens; a server-selected unrelated package or surface cannot attach to the headless probe document.
- The real native process oracle now requires three strict plans, three distinct one-use plan-receipt exchanges, three SocketGrants and three WebSocket epochs. It rejects any command before Session, verifies actor stamping and reconnect delivery, and fails if credential, plan receipt or SocketGrant text reaches stdout/stderr.
- The dedicated permanent `native-document-open-check` gate runs the independent document-open neutral oracle, exact-selects seven native Rust laws and one MCP Rust law with nonzero preflights, builds/directly supervises the WGPU child, and runs the three-epoch process oracle. Its Nx target is generated into launch configuration from the checked-in seed.

## Current evidence

- Server catalog/issuer/exchange/consume gate: session `95998`, exit 0, eight exact laws. This is server-only evidence.
- Browser D1 transport is owned and reported separately by the browser lane.
- Unique native target: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🗑️generated/native-socket-grant-sol-target`.
- Session `88236` began before the final codec, actor-authority and process-oracle changes. It is compile-diagnostic evidence only and must not be credited as a current-source runtime terminal.
- Session `6792` was stopped with exit 130 because the required full-scope host-key migration superseded its source bytes; it is not evidence.
- Launch regeneration session `52801` exited 0 and generated `⚖️gate📄️document-open-native🌎️hub`; generated freshness session `32654` exited 0.
- Current-source exact isolation-law preflight session `8015` exited 101 before selecting the owned law. The external plugin fan-in failed first: `semio-s-plugin-trinity` could not link `semio_framework_ui_contract`; several generated plugin renderers still returned `Result<BuiltNode, PluginAssemblyError>` where `UiNode` was required; and `semio-s-plugin-norm`/`draw` generated mutations lacked `MutationLeaf` implementations. Cargo's terminal was `could not compile semio-s-plugin-norm (lib) due to 816 previous errors`. This is no native D1 law evidence.
- Final scoped `git diff --check HEAD -- <owned files>` exited 0. The source census confirmed the 64 KiB admission cap, the shared `admit_document_socket` entry, full-scope host maps and callers, the dedicated Nx/script/seed/generated-launch registration, and the empty-catalog `openPlan=false` law.

## Remaining runtime boundary

- A final current-source unique-target native build, the seven exact native laws, the MCP authority law, and the three-epoch WGPU process oracle must complete after the unrelated plugin fan-in compiles.
- The normal secure-local hub intentionally has no linked verified openable catalog. Consequently a real-hub MCP D1 success cannot be claimed from that profile; the process must either run against an explicitly verified catalog profile or remain a truthful catalog-unavailable negative. No test-only catalog bypass or readiness overclaim is permitted.
- The host-level cross-space collision is source-closed, but its exact two-space isolation law remains runtime-pending because session `8015` failed in unrelated plugin compilation before test selection. This does not establish the browser lane's separate runtime keying or make an empty production catalog openable.

## Owned source

- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Shell/🎯️targets/🧊️wgpu/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs`
- `🌎️hub/📦️packages/🦀️rust/📜️script.ts`
- `🌎️hub/📦️packages/🦀️rust/📋️project.json`
- `.vscode/🧩️launch.seed.jsonc`
- `.vscode/launch.json`
