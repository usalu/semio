# Retained Page Layer Cache

Implemented in the existing Layout Feature Complete ticket. Layout only.

## Contract and Ownership

The schema and six neutral fixture cases were authored before the implementation. The initial strict TypeScript run failed because the production module did not exist. The helper borrows final layer/frame identifiers from a source with a stable caller registration identity. It preserves layer order and frame order, ignores unknown frame layers, and mirrors the existing cold helper for duplicate generic layer IDs. Actual keyed Layout admission already rejects duplicate layer IDs.

Native preparation reserves sorted ordinals, membership counts and output headers through the incoming original grant. The first pass sorts borrowed layer ordinals with the existing fixed HeapSortCursor, then routes frames by bounded binary-search comparisons. A count pass admits each final object-ID header array before a second routing pass copies final identifiers. Identifier comparisons and real UTF8 copies inspect at most 256 bytes per turn. Every String backing is reserved under the original capacity grant; source text is borrowed and copied incrementally. No Page, Layer or Frame clone and no cold derive_page_layers call occurs. Work is O(L log L + F log L + memberships), multiplied by compared identifier chunks.

The portable implementation retains immutable JavaScript string values by reference, so it does not pretend to allocate or physically own a fresh UTF8 backing. It models numeric and array-reference ownership explicitly; array capacity receipts describe the declared logical backing, not host V8 capacity or garbage collection. Text comparisons advance by at most 64 Unicode scalars per turn. No external runtime dependency was added.

Install requires ApplyCapability natively and swaps exactly one actual Vec<String> header per turn: page.layer_ids followed by each final layer.object_ids. Displaced original owners remain retained. Preparation cancellation, partial installation and normal completion all enter the same original funded close owner. The helper delegates native retirement to RetainedCloneClose and the derived structural retirement owner; there is no synchronous whole-cache destructor.

## Public Native Seam

- PageLayerCacheSource: identity, layer_count/layer_id, frame_count/frame_id/frame_layer_id.
- PageCachePreparation::new(), demand(source)->Result<RetirementDemand,ValueError>, advance(source,grant)->Result<RetainedCloneStep,ValueError>, is_ready().
- install_demand()->Result<RetirementDemand,ValueError>, install(page,grant,ApplyCapability)->Result<RetainedCloneStep,ValueError>. Final header swap returns Complete.
- begin_close(), close_demand(body), close_step(grant), terminal_is_empty().

Source rows remain under the caller's immutable source lease throughout read-only preparation. Registration and row counts are checked every turn; active native text additionally binds pointer and length. Installation occurs after real nested membership/field commits and validates the final layer extent. Interchange owns the production PageWork CacheView and root lifecycle integration; its source getters resolve final sparse provenance in O(1).

## Actual Verification

- Durable runner: page-cache-check.mjs.
- Strict original TypeScript closure: 245 sources, zero diagnostics.
- Six language-neutral cases plus a wide 137-layer/1027-frame page: 7 cases, 27216 funded turns, 82038 single-axis under-grant denials, 11 preparation/partial-installation cancellation cuts, 157 single-header installations.
- Independent test-only Lodash groupBy behind oracle.mjs produces the same complete layerIds/objectIds outputs. Ajv validates each neutral source against the authored schema.
- Cases include empty inputs, unknown layers, duplicate generic layer identifiers, order changes, supplementary/BMP Unicode ordering, empty IDs and long UTF8 identifiers crossing many copy/comparison turns.
- Under-grants for items/copy/capacity/release/depth preserve the cursor checkpoint and page caches with zero receipts. Source-registration substitution refuses. Native paired laws additionally author exact pointer preservation and displaced-cache custody assertions.
- Native production and law files parse with the installed Rust Tree-sitter grammar: two files, zero syntax errors; maximum native path 164 UTF16 code units.

Native tests are authored but have not compiled or executed: the repository runner remains stale and there was no raw Cargo/build workaround. The native laws use the existing first-party thread heap witness to compare physical requested/released bytes with returned original-grant receipts once a current native runner is available. This report does not claim native borrow-checking, macro expansion, linking or runtime success.

## Files

- schema/🔺️diff/🧳️retained/🪡️apply/🗃️page-cache/🔣️.json
- same owner: 🦀️.rs, 🟦️.ts
- same owner/🧪️tests: 🔣️.json, 🦀️.rs, 🟦️.ts, oracle.mjs, oracle.d.mts
- schema/🔺️diff/📦️packages/🦀️rust/Cargo.toml: first-party semio-framework-trace test-only dependency.
- ticket/page-cache-check.mjs and this report.

Generated transpilation output is created under the ticket generated folder and removed by the runner. The authored script, fixture, schema and report remain.
