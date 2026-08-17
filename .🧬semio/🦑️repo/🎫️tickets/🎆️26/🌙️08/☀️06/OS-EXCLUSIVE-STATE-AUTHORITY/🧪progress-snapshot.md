# Progress Snapshot — OS Exclusive State Authority

**When:** 2026-08-06

## Green compile gates
- `semio-framework-os-kernel` ✅
- `semio-framework-plugin` ✅ (receiverless DocumentApp + draft_operations + turbofish register)
- `semio-framework-plugin-host` ✅ (DocumentSession + engine-derive/read)
- `semio-s-2d` ✅ (DrawingEngine / EngineCache)
- `semio-s-3d` ✅ (content-addressed GeometryHandle, no seq)

## CHANNEL_VERSION
- bumped to **5**

## Wave 3
- `policyOsStateAuthorityBreaches` + `policyDocumentAppShapeBreaches` in root `📜️script.ts`
- still gated by `SEMIO_OS_STATE_AUTHORITY=1` (~100 breaches until Wave 2 finishes ZST migrations)

## Wave 4 proof
- Engine cache MISS→HIT captured in `🧪w4-engine-runtime-proof.err`
- `[DEBUG]` logs in EngineCache::derive and VcsDocumentApp::dispatch_emit

## Still open
- Host DocumentSession does not yet own DocumentStore/ConfigStore/DraftStore (guest VcsDocumentApp still holds them)
- Guest INSTANCES TLS / ViewState deletion
- Exchange pack rewrite for host-applied Emit
- Mass DocumentApp ZST migration across `✏️s/🔌️plugins`
- Unconditional policy + dep-cruiser/eslint/launch/verify wiring
