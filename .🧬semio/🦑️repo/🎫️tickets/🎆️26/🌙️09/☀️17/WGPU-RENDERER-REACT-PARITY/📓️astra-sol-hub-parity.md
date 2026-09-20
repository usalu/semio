# Astra Sol Hub Parity

Source-stable implementation report, 2026-09-20. This packet implements the Hub/session boundary in `📓️astra-terra-host-materials.md` section A against the fresh React reference in `📓️astra-hub-runtime-reference.md`. It does not claim a native or browser runtime verdict; the root agent owns the renderer census, WGPU build, activation, and live interaction probe.

## Neutral projection and oracle

- Added `engine/🧬️schema/🔗️hub-projection/🔣️.json` and `engine/🧫️fixtures/🔗️hub-projection/🔣️.json`. The closed projection contains only exact verified authority generation and bounded per-document remote state. It carries no credential, session capability, or UI-authored signed-in boolean.
- The fixture covers signed-out authority masking live documents, maximum live peer count across multiple documents, connecting over backoff, backoff over detached, document count, and empty verified offline state.
- Added the TypeScript law `engine/🧪️tests/🔗️hub-projection/🟦️.ts`. Ajv validates the shared fixture, an independent fold answers every vector, and the existing React `hubConnectionSummaryV1` is the third-party renderer oracle. Source laws also pin the React workspace composition, localized terminology, real `/hub` route, targetable footer opener, browser status publisher, bounded WGPU wire events, and boot-owned projection bridge.
- Added and mounted `Shell/🧪️tests/🔗️hub-projection-workspace/🦀️.rs`. Its Rust laws consume the same fixture, prove replacement and retirement in one map, and prove the 64-owner bound refuses a sixty-fifth distinct key while still accepting updates to an admitted key.

## Canonical authority and workspace

- `IdentityOutcome` now retains the exact `DirectorySessionAuthorityV1` returned by `/auth/sessions/me`; Shell stores that authority beside the canonical `DirectoryClient`. The footer projection reads its authorization generation and never infers verification from cached identity.
- Sign-in uses the existing session-mint request only to obtain a capability, converts it into the existing owned `LocalHubCredential`, and immediately verifies `/auth/sessions/me`. User mismatch, malformed authority, or transport refusal ends in the closed Hub error reducer. Sign-out and invitation redemption use new methods on the same `DirectoryClient`; no renderer-local duplicate auth protocol remains.
- Shell owns exactly one `HubWorkspaceState`. Its connection book is preference-persisted. Selecting or forgetting a hub clears the verified owner and drafts that depend on it. The retained surface supports close, hub selection, add/forget, email/password sign-in, cancellation, sign-out, spaces refresh/search/open/create, member roster, invitation creation/discard/redeem, and localized English/German labels.
- Every draft input updates on change, the password input retains password semantics, and a selected remote hub exposes a targetable Forget control. The WGPU frame Worker has no clipboard door, so it omits the copy button while retaining the readable one-shot invitation link.
- `/hub` is handled before plugin host resolution, projects verified authority, and refreshes spaces when signed in. It opens the existing retained `framework.hub` document in a modal Shell workspace, matching React's route-level overlay; Hub is not substituted into `defaultDock`. The new `os.openHub` command and the signed-out footer target both route through `/hub`. Opening a space continues through the existing Shell route/document owner, while Close retires overlay authority without modifying the dock.

## Aggregate status ownership

- `ShellHubProjectionV1` owns a bounded `BTreeMap` of attached document keys. The fold uses signed-out authority first, then live with maximum peer count, connecting, reconnecting, and offline. Native `ArtifactEvent::Status` updates the key owned by the current `ArtifactHost` channel and detach retires it.
- Browser `ShellHost` publishes every canonical sync-backbone status and close by runtime document key. `browser-frame-transport` carries lossless `hub-document-status` and `hub-document-close` events; the Worker validates keys, coalesces by key in a fixed 64-entry queue, retries admission on later ticks, and the runtime mailbox projects them into the same Shell map.
- The footer paints `framework.hub.signIn` only without verified authority and registers it as the `/hub` opener. Verified states remain ambient `s-hub-connection` status with no hit authority.
- A native borrow exposed by the first compiler census was repaired by cloning the already-cloneable `ProgramBridgeEntry` before the artifact-event loop. Exact authority expiry now flows from canonical `DirectorySessionAuthorityV1.expires_at: i64` into `HubSessionEvent::Minted.expires_at_ms: Option<i64>` with no unit conversion.

## Validation

- The first focused TypeScript run was red because the oracle looked for rendered English strings in the headless React components, which correctly consume terminology ids. The law now checks component ids and the canonical `HubConnection` terminology table separately.
- `NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun nx run @semio-tech/framework-renderer-wgpu:test-browser-worker --skip-nx-cache -- '🧪️tests/🔗️hub-projection/🟦️.ts'` passed 9 files and 98 tests.
- Targeted `git diff --check` over the Hub projection, workspace, browser transport, Worker, mailbox, and tests reported no whitespace errors.
- The mounted Rust laws were not executed by this agent because the root agent owns Cargo/native builds. The compiler census supplied two authoritative errors from the current Hub source; both are repaired above and await the root rerun.
- Renderer11's current-React reconciliation updated the command registry, Task Manager/default-dock fixture order, Hub neutral field names, and shortcut-row count. The new Hub workspace law requires absence from every default dock anchor, opens the overlay through `os.openHub`, checks retained document publication, and closes it through the real Hub action. This source was handed to the root-owned renderer compiler; no local Cargo verdict is claimed.

## Review boundary

No new runtime dependency, auth adapter, compatibility path, script, navbar/footer geometry change, or separate workspace authority was added. Credential capabilities stay inside the canonical directory/session client. Runtime authentication, live spaces, and browser interaction remain validation work for the root-owned activated build.

## Follow-up: connection-operation retirement and browser transport pressure

The production audit in `📓️terra-hub-tree-production-audit.md` found that React connection selection retired sign-in, authority and members work but left sign-out and command completions alive. The repair gives every Hub operation an owner containing the selected connection id, origin and monotonically increasing selection generation. Selecting, adding, or forgetting into a different connection advances that generation and aborts sign-in, sign-out, command, authority, member, spaces and invitation-redemption lanes. Every asynchronous completion checks the owner before changing session phase, authority, rows, members, invitation, redemption, connectivity, or initiating a follow-up refresh. Clipboard completion is scoped by the same owner. This extra check is required even with `AbortController`: `runHubSignInV1` deliberately normalizes an aborted request into a `failed/cancelled` outcome.

Three mounted deferred-promise laws now drive the real `useHubConnection` hook. They establish B authority, B rows and a B invitation before resolving A sign-out, A command, or an aborted A sign-in. The assertions pin B identity and phase, B rows, B invitation, absence of B refreshes from A's command, and a null B error after A's normalized failure. The neutral Hub fixture/schema also carries the generation-admission and latest-publication vectors, with an independent fold and the production owner predicate answering the same rows.

Browser Hub status publication now owns a separate 64-document latest-state map. Updates and close replace the pending state for the same document, do not consume keyboard/text/pointer lossless credits, and do not fault the surface when a sixty-fifth distinct pending key is refused. Transfer removes admitted keys and releases capacity; a caller can retry the refused key on the next tick. The production transport law publishes connecting, live and close for one key, fills the other 63 credits, observes refusal for key 65, transfers, retries, drains real Worker batches, and proves the first key delivered only close while the retried key delivered its newest live state. Therefore an update superseded by close cannot be emitted later as a stale resurrection.

Follow-up validation:

- The first mounted hook run was red on all three new A→B laws and exposed a manual-refresh event-argument regression in the initial owner-aware draft. The public refresh callback is again an explicit zero-argument boundary.
- `NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun nx run @semio-tech/framework-renderer-react:hub-sign-in-spaces-check --skip-nx-cache` passed 2 files and 66 tests, including the three deferred production-hook laws.
- Two fundamental-level reruns were killed by the target's 15-second whole-process budget after compiling and starting Vitest; neither reported a red test. `SEMIO_TEST_LEVEL=quick NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun nx run @semio-tech/framework-renderer-wgpu:test-browser-worker --skip-nx-cache -- '🧪️tests/🔗️hub-projection/🟦️.ts'` completed the same target and passed 9 files and 100 tests.
- The broad React typecheck remains red on shared workspace diagnostics. It exposed two local transport declaration gaps: emitted wire events did not declare `timestampMs`, and the ordinary lossless queue still admitted Hub variants. Both declarations are repaired; the focused production suite above compiles and exercises them.
- No Cargo, native renderer, wasm build, or browser runtime was launched by this agent. The root-owned activated runtime remains the authority for live behavior.
