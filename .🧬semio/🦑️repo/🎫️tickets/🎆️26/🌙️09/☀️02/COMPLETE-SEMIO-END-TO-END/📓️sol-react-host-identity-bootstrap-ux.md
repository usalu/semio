# React Host Identity and Bootstrap UX

## Outcome

This packet removes registry aliases and app ordering from the React host runtime. The live immutable manifest is resolved to the exact Home and Studio editor app objects once per manifest/alias pair. Session creation, direct routes, managed app switching, host presence, directory folding, chrome, and panel gates now consume those canonical objects or IDs. Missing, ambiguous, or identity-colliding required hosts fail explicitly and clear the host session; the host path never falls back to `manifest.apps[0]`.

Artifact restore lifecycle is now a typed worker-to-shell protocol rather than an `ArtifactEvent` cast. Progress carries exact byte and chunk units. Failure diagnostics are UTF-8 bounded to 4,096 bytes. A valid rebootstrap request aborts staging, discards committed pack/SPR/frontier/resume state, marks the transport connecting, and closes the current socket so the existing reconnect loop sends a fresh Hello without stale resume state. ShellHost unmounts the stale active session immediately and republishes it only after the replacement pack has loaded atomically. Progress state is cleared only by replacement or detach. The user-visible Cancel action calls the existing document close path, which aborts the worker actor, detaches the plugin backbone, unregisters routing, and removes the progress record.

Document-wide presence was not reduced: per-document heartbeats remain independent, and their existing canonical dialect-plus-role `surface` telemetry remains attached to hub bindings. This packet does not turn surface telemetry into document presence.

## Schema and independent oracle

- `ShellHost/🧬️contracts/🪪️host-bootstrap/🔣️.schema.json` is the Draft 2020-12 neutral contract for aliases, canonical identities, bounded progress, failure, and rebootstrap records.
- `ShellHost/🧬️contracts/🪪️host-bootstrap/🧪️fixtures/🔣️.json` deliberately places an unrelated app first and includes both Home editor and viewer surfaces.
- AJV independently validates the neutral schema. The existing React Testing Library DOM adapter independently proves the accessible status/alert roles, native progress value/max attributes, exact English/German units, and cancellation callback.
- The TypeScript oracle proves exact object identity, memoized resolution, absent/ambiguous rejection, retained status, and replacement/detach-only clearing.

## Verification

- Red: the focused React quick gate failed before implementation because the new host/bootstrap contract module did not exist.
- Green: `SEMIO_TEST_LEVEL=quick bun nx run @semio-tech/framework-renderer-react:test --skip-nx-cache --verbose` — 1 file passed, 4 tests passed.
- Green: `SEMIO_TEST_LEVEL=long bun nx run @semio-tech/framework-renderer-react:test --skip-nx-cache --verbose --testNamePattern='parses studio and studio\+instance shell paths'` — the complete ShellHost/index module graph transformed successfully; 1 route law passed and 536 unrelated tests were skipped.
- Green: `bun nx run @semio-tech/framework-os:test-quick --skip-nx-cache --verbose --testNamePattern='invalidates the committed session before rebootstrap|artifact bootstrap atomic restore'` — 1 file passed, 2 files skipped, 6 tests passed, 219 skipped.
- Green: `bun nx run @semio-tech/framework-renderer-react:lint --skip-nx-cache --verbose` — renderer region and host-contract lint passed.
- Compile diagnostic: `bun nx run @semio-tech/framework-renderer-react:typecheck --skip-nx-cache --verbose` remains red on concurrent pre-existing manifest/tutorial/icon and generated-schema drift. The packet's initial three JSON literal-widening diagnostics were fixed; the rerun reports no diagnostics in the new contract, fixture test, alias integration, or bootstrap UI integration. Existing unrelated diagnostics remain in `brand.ts`, replication and directory schema code, `index.test.ts`, renderer test support, `PluginRuntime`, `ShellHelpers`, older `ShellHost` tutorial/icon sites, `World3dHost`, and repo-library code. The root OS files also retain unrelated concurrent `DocumentDescriptor` type drift.

## Deliberate residuals

- No hub-authoritative open plan exists in this React packet. Rebootstrap can invalidate local state and force a clean transport Hello, but the server-owned document/open-plan work remains in its separate packet.
- No authentication changes were made. Surface IDs and CLI/browser-local identity are telemetry/policy inputs, never remote authorization; hub session issuance remains governed by the separate auth packet.
- The optional Rust WASM worker package is still not published by the current browser build, so the production browser follows the TypeScript worker path verified here. If that optional module is made loadable, its native `ArtifactEvent::BootstrapProgress` and conflict-based rebootstrap/failure output must be normalized to the same top-level lifecycle contract before enabling it.
- P2-C does not add a new retry button. Retryable lifecycle records remain visible while the existing worker reconnect loop runs; cancellation is the explicit user action.
