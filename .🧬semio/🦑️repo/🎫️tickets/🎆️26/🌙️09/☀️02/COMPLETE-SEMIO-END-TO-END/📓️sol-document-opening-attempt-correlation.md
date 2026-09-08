# Shell Document Opening Attempt Correlation

## Scope

This packet implements the Shell-local lifecycle correlation described in `📓️terra-shard-retained-lifecycle-current-audit.md`. It does not authenticate a client, authorize a Hub operation, prove a real GIS component, or prove a mounted Shell replacement while a guest call is pending.

## Contract

- `clientInstanceId` is an outer, UUID-v4-shaped opening-attempt owner.
- Shell document opens mint it before installing session and waiter state.
- `open`, `send`, `close`, document events, bootstrap transitions, socket actor transitions, execution-target status, and the outer browser patch handoff retain it.
- The strict domain-neutral `BrowserActorUiPatchOfferV1` and `BrowserActorUiPatchResultV1` remain unchanged. The backbone codec removes, validates, and reattaches the outer owner.
- A replacement sends an exact close for the prior owner before admitting the successor. Same-owner duplicate open and stale send/close are inert.
- Shell consumers compare both runtime key and opening owner before settling waiters or publishing state after an await.
- Rust wire and wasm-worker entries retain the optional owner. The native artifact actor does not treat this Shell-local value as Hub authority.

## Language-neutral and independent oracle

- Corpus: `🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/🧵️document-opening-attempt-v1.json`
- Schema: `🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/🧬️document-opening-attempt-v1.schema.json`
- Independent comparison: AJV 2020-12 plus `fast-deep-equal`, with 12 state-machine rows and 12 source-hostile substitutions.

## Executed evidence

1. `bun ./📜️script.ts nx run @semio-tech/framework-os:document-opening-attempt-check --skip-nx-cache`
   - Final receipt `dfa48a`, exit 0.
   - `document-opening-attempt-oracle: AJV=1 deep-equal=12 source-hostiles=12`.
   - Four executable TypeScript laws passed: strict request/response/patch wire, A→B worker replacement and stale operations, TypeScript D1 ownership after the Rust host resolves, and one retained token across the lazy EffectBackbone open plus every send.
2. `bun ./📜️script.ts nx run @semio-tech/framework-os:cold-document-pair-browser-check --skip-nx-cache`
   - Receipt `1b311b`, exit 0.
   - Five existing browser-worker/open/cold-pair/patch-handoff laws passed. This is the controlled worker fixture, not a mounted real Shell or GIS acceptance.
3. `bun ./📜️script.ts nx run @semio-tech/framework-renderer-react:typecheck --skip-nx-cache`
   - Receipt `27faec`, exit 1.
   - Renderer-wide typecheck remains globally RED on unrelated shared tutorial, renderer, replication, plugin runtime, flow declaration, repository-library, and pre-existing worker fixture diagnostics. It reports none of the previously observed opening-owner, Shell patch-envelope, presence literal, or cold-pair patch-result owner errors.
4. `bun ./📜️script.ts nx run @semio-tech/plugin-registry:generate --skip-nx-cache`
   - Receipt `cfe638`, exit 0; 59 plugin crates, 60 playgrounds, 45 framework packages and generated launch refreshed.
5. `bun ./📜️script.ts nx run @semio-tech/plugin-registry:check-generated --skip-nx-cache`
   - Receipt `a65f65`, exit 0; generated catalog and launch bytes fresh.
   - This is only the generator/immediate-freshness boundary. It is not the full repository taxonomy audit; Terra's separate full taxonomy run remains RED across 2,696 rows.

## Registered execution

- Source: `@semio-tech/framework-os:document-opening-attempt-check`, launch order `411.14993`.
- Native: `@semio-tech/framework-os-kernel:document-opening-attempt-native-check`, launch order `411.14994`, artifact root `🗑️generated/document-opening-attempt-exact`, one job, 256 MiB test stack, shared `public-member-open-sol-target`, and explicit 24-hour build/orchestration/command budgets.
- The native selector is `os_store::sync::tests::document_opening_attempt_wire_preserves_outer_owner_without_widening_actor_messages` with the `sync` feature. It is registered but not executed in this packet because the coordinated native cache is owned by the Hub GIS and database-capability sequence.

## Remaining acceptance boundary

- The token prevents stale outer completion/publication, but it does not cancel an already admitted `PluginWasmHandle.loadAppDocumentPack` mutation inside a reused plugin instance. A same-instance replacement still needs terminal retirement or a retained cancellable guest-operation owner.
- A mounted real Shell A→B replacement with a genuinely pending guest, real worker process, and exact stale-frame observations remains unexecuted.
- No real GIS WASM, authenticated Hub open, visible Map render, or peer propagation is claimed by this packet.
