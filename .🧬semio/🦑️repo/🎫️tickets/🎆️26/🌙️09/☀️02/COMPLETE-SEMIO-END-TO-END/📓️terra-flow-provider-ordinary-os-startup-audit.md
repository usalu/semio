# Flow Provider Ordinary OS Startup Audit

## Verdict

**RED for both ordinary browser and native OS startup.** The live hub can hold a
verified static codec closure only when both trusted-catalog environment inputs are
supplied. Ordinary startup supplies neither, so it publishes no verified catalog.
The React shell therefore creates a bare hub binding and the worker deliberately
stops before its first HTTP request. Native WGPU has a distinct, weaker path: it
can ask the hub for a plan from an unverified local program manifest, but has no
process-local verified catalog/codec bootstrap or full immutable execution-target
comparison.

This audit is source-only. No build, launch, or runtime process was started.

## Current Startup Boundary

`os-hub` creates a static *provider selector*, not an installed catalog:

1. `NativeCodecProviderSetV1::linked()` has two compiled-in selector entries,
   `stdio/semio:stdio` and `gis/semio:gis`, at
   `🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🦀️.rs:24-74`.
   Calling it neither invokes factories nor registers codecs.
2. `configured_artifact_authority` returns `Ok(None)` when both
   `OS_HUB_TRUSTED_CATALOG_BUNDLE` and `OS_HUB_TRUSTED_CATALOG_PROFILE` are absent;
   one without the other is a hard error
   (`🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:380-391`).
3. Ordinary hub boot reads only those two environment variables before deriving
   `artifact_authority_ready`, `open_plan_ready`, and `openable_catalog`
   (`🚀️bin.rs:5315-5359`). Therefore no configured pair means no verified
   installed catalog and `/readyz` has `artifactAuthority=false`, `openPlan=false`.
4. The only current readiness positive manufactures a temporary stdio bundle,
   loads it into a test state, then manually substitutes that state before probing
   `/readyz`; it asserts 26 codecs and one open target
   (`🚀️bin.rs:5659-5691`). It is explicitly not an ordinary launch, browser, or
   native-client journey.

The loader itself is appropriately fail-atomic for the hub process. It bounds and
hashes each component (SHA-256 and BLAKE3), bounds and hashes each descriptor,
checks the selected dependency closure, validates native binding identity/schema/
pack hash and rejects unconsumed extras, then preflights the whole codec assembly
before registration (`🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:342-504`).
It creates a sorted immutable open-target generation. This does **not** publish an
equivalent catalog into a browser or desktop process.

## A Flow Member Is Not Yet a Catalog Provider Row

The current stdio plugin registers `SemioFlowEditor` and `SemioFlowViewer`
(`✏️s/🔌️plugins/🗄️stdio/🦀️.rs:72-73,355-356`). That is an app/member declaration,
not a `NativeCodecBinding` or a trusted-catalog `openTarget`. The current stdio
native receipt closure is fixed at 26 (`native-openable-provider/🦀️.rs:9-11`), and
its native-codec receipt registry contains no Flow receipt. Thus a future verified
public `MemberFactory::begin_open` result cannot itself admit Flow into the
catalog: catalog selection is needed *before* the hub can issue a plan, whereas
member opening is a post-plan consumer.

The correct order is:

1. Produce a Flow-specific native codec receipt/binding and descriptor-owned React
   and WGPU `openTarget` rows for one exact stdio component.
2. Select those exact rows in an immutable bundle/profile and let the hub loader
   atomically publish the complete closure.
3. Derive the client execution target from the same verified installed package
   record, retaining catalog generation and the selected parent dialect.
4. Issue/exchange the plan, establish the socket, then run the public retained
   member-open operation against that selected parent dialect, document scope,
   expected parent/owner reference, and validated checkpoint/history.
5. Only after that operation finishes may the app receive a document pack and
   render/attach its backbone.

Neither a Flow factory receipt/target nor step 3--5 is present as an ordinary
runtime path today.

## Hub Authority Already Enforced

The hub issuer is not the current blocker. It bounds and authenticates the request,
serializes subject admission, revalidates the session/share subject, loads the
descriptor, resolves a unique catalog selection using the requested surface and
author/viewer role, records catalog generation/checkpoint/revalidation state, and
issues a short-lived receipt (`🚀️bin.rs:1970-2083`).

At exchange it rechecks the descriptor, complete selected package/artifact/private
parent dialect/surface/grant tuple, catalog generation and directory membership
revision before consuming the receipt (`🚀️bin.rs:2115-2174`). Socket validation
repeats descriptor, catalog generation, complete selection, revision, and
checkpoint checks (`🚀️bin.rs:2585-2628`). The private authority verifies the
descriptor owner component SHA-256, artifact/schema/pack hash, author-vs-viewer
write grant, and parent dialect (`🚀️bin.rs:993-1040`).

There is an important current wire gap: `parent_dialect` is retained in the private
authority (`🚀️bin.rs:1001`) but omitted by `public_plan`
(`🚀️bin.rs:1045-1060`) and by both Rust and TypeScript `DocumentOpenPlanV1`
schemas (`🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs:686-701`,
`🧬️schema/🟦️.ts:426-440`). A public member-open consumer consequently cannot bind
the selected parent dialect from the server's issued plan yet.

## Browser: Fail-Closed but Unwired

The browser protocol type can carry a complete installed execution target:
package identity with component SHA-256, component BLAKE3, descriptor SHA-256;
artifact kind/schema/pack hash; and exact surface tuple
(`🧰️framework/🛍️products/💻️os/🟦️.ts:557-581`). The worker compares all of those
fields, scope, configured pack hash, requested surface, and `react` renderer before
receipt exchange (`🧵️backbone-worker.ts:481-506`), uses bounded abortable BFF
requests (`:509-546`), and refuses a hub open with no installed target before any
network request (`:515`, `:1659-1663`).

The ordinary React `ShellHost.openDocument` default supplies only
`{kind:"hub", baseUrl, spaceId, surface}` (`🏛️ShellHost/🟦️.tsx:3442-3476`); the
space-index opener is likewise bare (`:5535-5543`). No ShellHost source derives or
injects `installedTarget` from a verified installed component. Existing browser
tests instead construct a literal target and mocked plan/socket path
(`🧵️backbone-worker.ts:1939-1944,2432-2642`). Therefore the browser has a useful
pre-request denial, but no positive ordinary Flow path.

Remaining browser binding required after immutable package verification:

- retain and compare the catalog generation and public parent dialect too; neither
  exists in `InstalledDocumentExecutionTargetV1` or the worker equality function;
- bind the selected grant to the local document mode before enabling mutation or
  presence behavior; parsing validates its shape but the execution target does not
  retain a grant;
- feed the successful exact selection into public Flow member open rather than
  attaching a backbone to a dev-catalog plugin instance.

## Native/WGPU: Opens Too Early and Knows Too Little

Native `PersistenceBinding::Hub` contains only base URL, space ID, and optional
surface (`🏪️store/🔄️sync/🦀️.rs:82-111`). `ArtifactHost` has a one-shot local
surface preclaim (`:1126-1150`), but it has no installed-target, component digest,
descriptor digest, catalog generation, parent dialect, or grant field.

WGPU derives that preclaim from a `ProgramBridgeEntry` manifest and current app
(`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:550-603`), then opens the actor and attaches the
plugin (`:3686-3715`). It does not call `TrustedCatalogLoader` or
`NativeCodecProviderSetV1`. The only WGPU renderer registration found is a
`NativeSocketProbe` test codec, not Flow. The per-process actor then asks its local
codec registry for `document_codec(schema)` before it can request a grant
(`🏪️store/🔄️sync/🦀️.rs:1990-2021`), so a normal WGPU process has no verified Flow
codec bootstrap shown here.

After the server responds, native checks origin, scope, schema, local pack hash,
surface ID, and the abbreviated manifest surface expectation
(`🏪️store/🔄️sync/🦀️.rs:2045-2075`). `DocumentSocketAuthorityV1` carries complete
package and catalog data, but `matches_surface` compares only plugin/package/
version and surface fields (`📇️directory/🔌️client/🦀️.rs:244-304`). The native
directory client likewise compares no component SHA-256, BLAKE3, descriptor
SHA-256, catalog generation, parent dialect, or local grant capability before it
exchanges the plan (`📇️directory/🔌️client/🦀️.rs:748-826`). This is a native
authorization/identity bypass relative to the browser's full immutable target,
not evidence that the hub itself accepts a forged selection.

Native must gain one verified execution-target lease populated only by a local
trusted catalog load. It must be carried in `PersistenceBinding::Hub`/
`ArtifactActorConfig`, copied into the surface expectation, checked before
`/open-plan`, rechecked after exchange and before `SocketHello`, and invalidated on
catalog-generation turnover/cancellation. Its grant must gate local write/observe
operations; server-side revalidation remains an independent backstop.

## Smallest Honest Process Acceptance

One neutral `flow-open-runtime-v1` fixture should be shared by **two** exact
process laws--browser and native cannot truthfully be collapsed into one mocked
transport test. It contains one authenticated author, one space/document
descriptor, immutable Flow component/descriptor bytes, matching receipt/bundle
profile, parent dialect, Flow editor (write) target, Flow viewer (read) target, and
a bounded checkpoint/history whose public member open reaches a known Flow
component-tree marker.

The hub is launched with the fixture's actual
`OS_HUB_TRUSTED_CATALOG_BUNDLE` and `OS_HUB_TRUSTED_CATALOG_PROFILE`; the law must
observe `/readyz` 503 before the valid configuration and 200 only after the actual
loader completes--not by replacing `HubState` as the present test does. It must
derive both client targets from verified local package bytes, never literals.

The browser law then drives real `ShellHost.openDocument` through the BFF, observes
one open-plan request, one receipt exchange, negotiated socket and a Flow render
marker, and proves close/cancel retires the member/actor. The native law drives the
real WGPU shell/`ArtifactHost` route against the same hub, observes the same
exchange and `SocketHello`, then a public Flow member-open and rendered tree marker.
Both must deny a component digest, descriptor digest, pack hash, catalog generation,
parent dialect, surface role, or grant mismatch before app publication; browser must
also prove absent target causes zero HTTP/WS effects.

Existing registrations are insufficient: `native-openable-catalog-provider-check`
states it makes no client-mount claim (`🌎️hub/📦️packages/🦀️rust/📜️script.ts:2958-2982`),
while `browser-document-open-check` and `native-document-open-check` run neutral/
mocked transport and focused source laws (`:3086-3092`, `:3799-3807`). Add the two
process laws to the same source-owned hub script and seed launch configuration, then
regenerate launch metadata; do not edit generated launch output directly.

## Dependency-Ordered Handoff

1. Finish the public retained MemberFactory open contract and Flow typed decoder;
   this is a consumer boundary, not catalog activation.
2. Add one actual Flow codec receipt and two descriptor `openTarget` rows; enforce
   the exact 1:1 factory/descriptor/package closure in the existing trusted loader.
3. Carry a full immutable target lease (including catalog generation and parent
   dialect) to React and native before either transport starts.
4. Add the two shared-fixture real-hub laws above, including close/cancel and
   hostile no-publication outcomes.

Until all four land, a hub `openPlan` readiness success, a linked static provider,
or a public Flow member decode is **not** evidence of ordinary OS startup, socket
authority, or rendered Flow activation.
