# Native D0 Document Codec/Open Readiness Audit

Current-source audit, 2026-09-04. This is a read-only review of the native WGPU/store/client path after the D1 issuer/exchange work. No Cargo target was started, so this records source/compile reachability only, never a native runtime result.

## Verdict

**RED — native D0 opening is not ready.** The server can now issue a catalog-bound plan and exchange it for a document socket grant, and the current store actor consumes the new `admit_document_socket` interface. That repaired fanout is source-closed. It does not make the native opening authority-safe or executable: the fresh grant can be disclosed to a different binding origin, the native mount is still selected before the plan and never bound to its package/surface target, and every real WGPU open still reaches the retired backbone stub.

## Current path and authority boundary

| Stage | Current source | Classification |
|---|---|---|
| D0 form/codec | `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs:490-761` owns strict intent/plan/exchange forms, 30 s TTL, exact safe integers, receipt grammar, scope/schema/catalog/checkpoint/revalidation checks. | Source-qualified shared authority. |
| Issuer/exchange | `🌎️hub/📦️packages/🦀️rust/📦️bin.rs:1986-2213` authenticates, selects only from a nonempty verified catalog, revalidates descriptor/revision/subject, caps and consumes one receipt, and registers the private plan with the socket grant. Readiness derives both flags from a nonempty catalog at `5277-5283`. | Server source; no native runtime implied. |
| Native client | `📇️directory/🔌️client/🦀️.rs:712-787` percent-encodes the two HTTP path segments, validates plan scope/schema/surface and expiry, exchanges the receipt, and returns receipt-free plan authority plus a grant. | Partial source implementation. |
| Native actor | `🏪️store/🔄️sync/🦀️.rs:1864-1953` uses `admit_document_socket`, rejects expiry/scope/schema/local pack-hash mismatch, closes a late socket, then sends tag-7 `SocketHelloV1`. | Partial source implementation. |
| WGPU opening | `📺️renderer/.../Shell/🎯️targets/🧊️wgpu/🦀️.rs:417-475,3595-3618,3960-3977` accepts caller-selected artifact/schema/plugin/app coordinates and opens/attaches that current session before the actor obtains a plan. | Authority bypass / RED. |
| Plugin execution | `📺️renderer/.../ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs:281-283`, reached by Shell at `3607-3610`, always returns the retired-channel error. | Absolute runtime blocker / RED. |

There is no Tauri native entrypoint in the current tree. The native route is the non-wasm WGPU shell plus `ArtifactHost` actor.

## Material blockers

1. **Grant origin is not bound to the credential origin (security RED).** `DirectoryClient::protected_post` sends the plan and exchange to its credential-bound URL (`📇️directory/🔌️client/🦀️.rs:673-701`). The actor then derives the WebSocket URL from independently caller-supplied `PersistenceBinding::Hub.base_url` (`🏪️store/🔄️sync/🦀️.rs:82-92,1456-1460,1865,1900`) and places the new grant in `Sec-WebSocket-Protocol`. No equality/origin check occurs before the dial or in `finish_connect_hub` (`1916-1928`). A different hub binding can therefore receive a valid one-use grant. Derive the WS origin from the credential/source’s verified hub origin (or a canonical nonsecret origin returned in the admission and checked equal); reject mismatches before issuance or dial.

2. **The server-selected native mount is not the mount authority (security/correctness RED).** The plan explicitly carries `package`, `artifact`, `surface` including `renderer_target`, and effective grant (`📇️directory/🧬️schema/🦀️.rs:531-574`). The actor validates only scope, artifact schema, pack hash and optional surface (`🏪️store/🔄️sync/🦀️.rs:1916-1928`); it never verifies package id/version/component digest, `app_id`, `window_kind_id`, native renderer target, role, or grant against the mounted WGPU program. WGPU accepts those selections from relay input and attaches the already selected plugin first. A React/Wasm or unrelated-package plan can consequently be connected by a native current session. Make a successful native plan select the installed package/app/window/role first, reject every mismatch, and only then construct an immutable actor configuration from receipt-free authority.

3. **The socket URL is not component encoded (availability and boundary drift RED).** HTTP plan/exchange paths use a local percent encoder (`📇️directory/🔌️client/🦀️.rs:715-739`), while `hub_ws_url` interpolates server-authoritative space id, document id and surface straight into path/query (`🏪️store/🔄️sync/🦀️.rs:754-767,1900`). D0 text validation permits punctuation and Unicode. The canonical browser corpus requires encoded Unicode path segments (`🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/📄️browser-document-open-v1.json:45-54`). `?`, `#`, `&`, `/`, percent and non-ASCII therefore do not have one native round-trip representation. Share the byte-wise URI-component encoder for all three fields and add a real native hostile/Unicode WS assertion.

4. **Socket-grant secret lifetime is not wiped (S3/D0 redaction RED).** The plan receipt is wrapped and wiped (`📇️directory/🔌️client/🦀️.rs:330-335,747-760`), but `SocketGrantReceiptV1.grant` is an ordinary `String` (`271-300,700-707`). It is retained in `DocumentSocketAdmissionV1`, copied into `format!(...)` for the WebSocket header (`🏪️store/🔄️sync/🦀️.rs:1900`), and has no wiping owner on success, cancellation, handshake rejection, or drop. Introduce a non-debug/non-clone zeroizing grant owner, move it exactly once into the header/dial seam, zero caller-owned copies on every terminal branch, and prove it with an observer-backed failure/late-cancel law. Existing digest-only probe output is good but does not erase the raw grant.

5. **No native D0 contract/runtime law exists.** The client has no `admit_document_socket` test (`rg` finds only its declaration and implementation); the store’s native focused law at `4617-4695` proves receipt actor/session-epoch cleanup, not plan issuance/exchange/mount. `open-plan-check` runs the shared schema law plus server laws only (`🌎️hub/📦️packages/🦀️rust/📜️script.ts:2632-2674`); `browser-document-open-check` is browser-worker scoped (`2710-2717`). Neither selects or executes this native path.

## Compile and feature state

The immediate stale `issue_document_socket_grant` actor call observed during the live edit is **superseded**: current source calls `admit_document_socket` at `🏪️store/🔄️sync/🦀️.rs:1894-1910`. The test helper was updated separately and no current source-only Rust signature mismatch remains in this path. This is not a compilation claim because no Cargo invocation was run.

The actual WGPU native package enables `semio-framework-os-kernel` with `sync` and `ureq` (`📺️renderer/🧑️‍🎨️engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/Cargo.toml:93-106`), which activates the concrete native directory transport. The kernel exposes that combination at `💻️os/📦️packages/🦀️rust/Cargo.toml:20-38`. Thus the broken D0 authority path is in the intended native feature set, not a disabled optional branch. The existing all-feature `open-plan-check` can still be RED before its laws on unrelated graph compilation; even a green terminal would not qualify native WGPU because its exact target set never contains the client/actor/opening behavior above.

The native transport additionally reads protected responses through a fixed 16 KiB `take` (`📇️directory/🔌️client/🦀️.rs:1215-1229`), while the cross-language D0 fixture fixes the response budget at 64 KiB (`🧫️fixtures/📇️directory/📄️browser-document-open-v1.json:53`). It neither shares that bound nor detects an over-limit body; this needs a single named D0 response limit and a max/max+1 transport law before the native codec can claim corpus parity.

## Smallest truthful production sequence

1. Complete the native admission primitive first: one shared 64 KiB bounded response reader that detects `max+1`, component-encodes HTTP and WS coordinates, derives the WS origin from the credential, and owns/wipes receipt and socket-secret memory. Add deterministic fake transport laws for plan rejection, expiry, scope/schema/surface mismatch, max+1, cancel before/after each phase, origin mismatch with zero dial, and no credential material in URL/log/error.
2. Replace raw `ArtifactActorConfig` hub selection with an immutable receipt-free `DocumentSocketAuthorityV1` configuration. Bind origin, scope, descriptor/hash, package identity/digest, artifact codec, native renderer target, app/window/role, and grant before the socket is offered. Key actor ownership by scope, not bare document id.
3. Make the WGPU operation plan-first: relay input may request only a document scope and non-authoritative surface preference; the verified plan chooses the installed program and mount. Do not attach a plugin until all native plan bindings and the local codec/package checks pass. Provide EN/DE progress/cancel via an operation owner and propagate its generation/deadline rather than using a root cancellation token.
4. Replace `ProgramBridge::attach_backbone` with the production event/effect attachment seam. This is an independent prerequisite: D0 cannot be runtime-accepted while the current explicit error is reached on every open.
5. Register one uncached native D0 gate: it must exact-select a native client/actor law and run an actual hub/native child. Exercise Unicode/reserved coordinates, wrong-origin no-dial/no-leak, wrong package/renderer no mount, one receipt/grant, post-open cancellation closes once, tag-7 plus matching `Session` before delivery, reconnect with a fresh grant, and secret/URL/log redaction. Keep browser/server oracles as parity evidence, not a substitute.

## Acceptance boundary

Current status is **server/client source partial; native codec/open RED; native runtime untested**. Do not promote D0, native OS, or S3 native document transport from the current server, browser, or epoch-cleanup terminals.
