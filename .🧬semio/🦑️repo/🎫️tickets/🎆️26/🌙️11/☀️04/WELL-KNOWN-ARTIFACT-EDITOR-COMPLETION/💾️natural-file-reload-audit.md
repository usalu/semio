# 💾 Natural File Persistence and Reload Audit

Read-only trace of current production source; no tests/builds run. H = 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost.

## Missing Integration

H/📤️document-transfer-tabs/🔶️scope/🟦️.tsx openNaturalFile imports via createApp/importAppMedia, then commit only calls focusSpawnedProgram. That function writes panelJson in current SET_SESSION viewState with a plugin-instance UI id. It does not attach an actor via openDocument, persist the artifact, or record its identity in a local catalog. saveNaturalFile exports bytes to browser download and emits file-saved; that is export completion, not durable local document acknowledgment.

PluginRuntime/🔗️channel-adapter/🟦️.ts importAppMedia only checks Error and Done frames. It does not apply requested effects or attach persistence. No automatic actor-attachment effect was found for imported spawned instances. Explicit sync-card openSyncTarget reads document identity and calls openDocument; natural-file import never calls it.

Consequently the current acceptance reload proof should fail genuinely unless another explicit keep/attach path ran; it must not reimport bytes silently or accept recreated example windows as restoration.

## Existing Persistence Authorities

H/🗂️local-catalog/🟦️.ts defines localCatalogAdmissionV1 and schema-owned catalog mutation envelopes. H/🛂️directory-route/🔶️catalog/🟦️.ts writeLocalDocumentRef opens folder actor, sends localDocumentArchive and waits for byte-identical readback before success; recordLocalCatalogRef writes admit mutation into local catalog. Replay shell command effect wires those in write-first/record-second order.

However current reload useEffect H/🗄️shell-uri-documents/🔶️scope/🟦️.ts dispatches applyLocalCatalogDocument only to landing Home app. The Home command at ✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/.../🎮️commands/🗃️apply-local-catalog-document/🦀️.rs explicitly parses SpaceSnapshot/SpaceMutation, requires S_SPACE_SCHEMA and rejects non-studio data. This is not generic stdio document rehydration. Blindly admitting a stdio archive through this lane would still fail restore.

A generic document keep path must carry stable document/schema/editor ownership and restore through artifact opening/actor attachment, or persist the imported artifact into an actual space index and reopen by artifact reference. Runtime plugin instance IDs alone cannot be the persistent document identity.

## Test Seams

H/📂️picked-file-opening/🟦️.ts PickedFileFlowPortV1.commit currently returns synchronous boolean; production flow calls it without await then emits opened. Async keep/actor admission there requires Promise-aware commit, cancellation/ownership checks, cleanup before acknowledgment and no optimistic opened notice.

Existing tests: engine/🧪️tests/📂️picked-file-opening/🌊️flow.ts cover routing/import/focus failure and resource cleanup, not durable keep/reload. engine/🧪️tests/💾️media-export-and-natural-files/🟦️.ts covers natural codec helper owner lifecycle. Add real buildScopeState port integration tests requiring opened wait for verified persisted archive, failed/cancelled keep releases created instance without catalog entry, later edits acknowledged, and reconstructed host restores exact stable identity plus edited export content. Generic catalog schema and independent validator fixtures should cover non-studio editor identity.
