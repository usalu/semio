# Flow Trusted Codec Bootstrap Audit

Status: **RED for an ordinary Flow OS startup.** This is a current-source, no-build audit. Existing catalog checks are strong for a configured provider, but no configured or linked Flow provider exists, and neither client can bind its local renderer to the catalog's current `wasm` execution target.

## Proven current chain and boundary

1. Flow has a first-party plugin identity, not a provider: [`✏️s/🔌️plugins/🌊️flow/🦀️.rs`] declares package `semio:flow`, and [`✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🦀️.rs`] declares `computation.flow`, schema `flow.fixture`, and a document codec. Its parent dialect is the distinct `s.flow.flow@1/*`; it must **not** be substituted for the `computation.flow` artifact kind.
2. The Flow `describe` command writes a descriptor from a component under Cargo's `wasm32-wasip2/wasm-dev` target output ([`✏️s/🔌️plugins/🌊️flow/📦️packages/🦀️rust/📜️script.ts:55`]). No immutable Flow component/bundle pair is currently present for hub consumption. A generated descriptor alone is therefore not a trust root.
3. The only linked hub providers are `stdio` and `gis` ([`🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🦀️.rs:24-45`]); `🌎️hub/📦️packages/🦀️rust/Cargo.toml` has no Flow dependency. A `flow`/`semio:flow` catalog package fails `preview` before it can publish a codec.
4. A configured catalog does verify raw component SHA-256 and BLAKE3, raw descriptor SHA-256, descriptor/package closure, provider receipt/factory closure, and target codec closure before registering codecs ([`🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:344-504`, `:810-850`]). Its generation hashes all 18 target fields: package plus three digests, artifact tuple, all three parent-dialect fields, surface tuple, renderer, and grant ([`:606-644`]). This is the correct hub-side publication boundary.
5. The hub reads only the paired `OS_HUB_TRUSTED_CATALOG_BUNDLE` and `OS_HUB_TRUSTED_CATALOG_PROFILE` variables; absent values intentionally produce no authority and partial configuration fails closed ([`🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:380-393`]). It exposes an openable catalog only after successful load and marks `openPlan` ready only for a nonempty target set ([`:5320-5364`]). Current launch seed/dev startup prepares neither Flow bytes nor those two variables.

The only close-looking readiness fixture is `native_openable_stdio_bundle()` ([`🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:5576-5683`]). It creates a temporary `b"abc"` component and a **stdio** JSON-viewer record. It is synthetic, removed after the test, and cannot evidence Flow, a shipped bundle, browser mounting, or native mounting.

## Exact current client incompatibilities

The catalog verifier admits document-open targets only where `rendererTarget == "wasm"` and the descriptor app/window/role/dialect are exact ([`🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:538-562`]). Browser worker admission instead requires its locally installed execution target to be `react` and compares its package digests/dialect/surface against the plan ([`🧰️framework/🛍️products/💻️os/🟦️.ts:482-555`]); its current targets are test fixtures, not a Flow catalog materializer. Native `DocumentSocketSurfaceExpectationV1` and `ArtifactHost` retain only partial package/surface fields, not all three digests, parent dialect, catalog generation, or grant; WGPU derives a `wgpu` expectation from a program bridge. Neither can consume a current hub `wasm` target truthfully.

Therefore a server-only Flow bootstrap may prove hub readiness and plan issuance, but must make **no browser/native-open claim** until the separately required immutable client execution-target lease carries the full catalog-selected values and a verified `wasm`-to-local-renderer model exists.

## Smallest honest zero-touch, server-only packet

1. Add a Flow-owned private native receipt/factory closure derived from the existing typed Flow codec declaration. Require exactly the real package version, `computation.flow`, `flow.fixture`, extension, nonzero pack-schema hash, and factory-result equality. Add `semio-s-plugin-flow` and exactly one `flow`/`semio:flow` provider entry to the hub provider set. This is independent of the fixed stdio receipt count.
2. Extend the hub's existing source launch script and **launch seed** (then regenerate `launch.json`) with a server-only materializer. It must invoke Flow's canonical describe/build output into a fresh private run root, copy the exact generated component and descriptor bytes, calculate their raw SHA-256/BLAKE3/lengths, decode the descriptor, and emit `trusted-catalog/v1` profile `local-flow-v1` from those decoded values. Select exactly one descriptor-declared Flow viewer target and derive app/window/surface from it; record renderer `wasm`, not `react` or `wgpu`.
3. Clear inherited trusted-catalog variables for the spawned hub child, supply only the newly materialized bundle/profile, and do not start the child if any preparation/verification step fails. Keep the bundle server-owned and loopback-client-inaccessible. On restart create a new run root and profile; stop the hub before deleting it. The current catalog is static, so rotation is restart-only: a changed target input creates a changed generation, and process restart invalidates in-memory plans/grants.

No client input may name a bundle path, profile, component, descriptor, digest, factory, or codec. Prebuilt artifacts are acceptable only if they are an immutable component+raw-descriptor pair verified by this same materializer; source-tree descriptors and test `b"abc"` bytes are not.

## Required proof before calling this bootable

- Language-neutral Flow bundle corpus: valid descriptor/component bytes plus independent SHA-256 and BLAKE3 framing; tamper each component SHA-256/BLAKE3, raw descriptor SHA-256, descriptor internal component digest, package/version, codec schema/hash, parent dialect fields, surface/app/window/role/renderer, duplicate/missing provider, and extra factory. Every denial leaves codec count and catalog generation unchanged and `openPlan` not ready.
- Process law through the generated launch entry: poisoned inherited catalog variables cannot alter the child; absent Flow bytes, malformed descriptor, selected-profile mismatch, empty target, outside-root/symlink input, and preparation cancellation spawn no hub. A valid materialized Flow run reaches `/readyz` with Flow open-plan readiness only after atomic provider registration.
- Authenticated hub law using a real directory descriptor: issued plan contains exact component SHA-256/BLAKE3, descriptor SHA-256, `computation.flow`/`flow.fixture`, full parent dialect, surface, grant, and generation; exchange is one-use; a restart/rotated bundle invalidates the old plan. Status and logs must not expose receipt secrets.
- Browser and native runtime laws are separate RED work: a real locally verified Flow component must acquire a full immutable lease and be rejected for any one-field digest/dialect/grant/generation/surface mismatch. Existing `s.test`, synthetic stdio, and mock browser targets do not qualify.

## Acceptance and nonclaims

Acceptance for this packet is one verified Flow provider compiled into the hub, one server-owned materialized profile, and one real hub process whose Flow plan readiness and authenticated issuance succeed. It does not activate Flow in React, WGPU, MCP, or a document socket, does not make source outputs a release artifact, and does not turn the static catalog into hot rotation.

