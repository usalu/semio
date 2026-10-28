# Ordinary ACK facade lease prereview

Read-only stable ordinary intake/owner flow; new in-progress lease implementation not reviewed or approved here. Paths: renderer elements/PluginRuntime/handle-ui-intake/🟦️.ts installUiPatchRound, acceptOwnedUiPatches, closeOwnedUiInstance; handle-scope releaseInstanceMaps; shared UI contract/retained/instance/🟦️.ts OwnedUiInstanceSurface. No renderer execution claim.

## Existing authority path

acceptOwnedUiPatches requires actual lifecycle lease, OwnedUiInstance, original turn and uiPatchReceipt, then captures per-index real patch authority. Intake reaches original acknowledgement token; installUiPatchRound submits actual issued-ui-acks under lifecycle authority and verifies returned submission count. Each intake accepts its actual corresponding acknowledgement, advances publication/retirement, takes the real surface and installs it into private uiSurfaceByInstance. Publication listeners are scheduled after the full installed round; public BuiltNode projection remains a separate renderer convenience, not an owned facade authority.

OwnedUiInstanceSurface is a stable frozen facade around a mutable live cell. Its view getter reads the current live source; an old facade/map entry can therefore expose a newer revision before that newer intake has completed the ordinary ACK install path. Map membership, matching surface name or a once-issued facade cannot alone prove the current view was acknowledged. Future lease needs an internal exact acknowledged view stamp at real install, plus actual owner/lifecycle/actor/handle/surface identity and read-time stamp equality. Do not let a caller supply that stamp or construct a lease from a copied view/BuiltNode.

## Revocation/reentrancy obligations

Invalidate old ACK authority before replacement intake may change the live view. Reinstall only after genuine ACK+actual takeSurface. Capture/check the exact current view at the same synchronous authority boundary and recheck after callback/notification work; subscribeView can participate in synchronous maintenance notification even though host publication announcement uses the continuation scheduler. A listener can close, replace or edit while being notified; retain only authority that remains current afterwards.

Instance close is serialized with intake, first requires no uiReadsByInstance, closes outstanding intakes, clears installed map and calls owner.beginClose. Lease access/read obligations must revoke and retire before that guard; merely deleting a facade entry does not retire issued read snapshots. releaseInstanceMaps removes actor/lifecycle/owner/surface maps, while handle/activation retirement must also revoke all leased access. Recovery/hot replacement cannot reuse a retired lifetime's acknowledged stamp.

Failed subscribe/read admission must not publish a usable lease. Failed unsubscribe must remain in a live retryable owner rather than be forgotten because public closed=true. Explicit close must prevent later stale notification from restoring readiness; notification callbacks should be isolated so one throwing consumer cannot skip internal revocation. Revoke internal authority before invoking external listeners. Old issued reads may remain physically valid until retirement but are not current publication/intent proof.

## Concrete tests for stable accessor

Use actual first-party retained wire/receipt/ACK fixtures when build policy permits; structural law probes remain labeled as such. Cover old facade after preACK source publication, actual matching ACK install, replacement/missing/wrong receipt, equal revision/hash under different owner or lifetime, real actor/handle close, listener-close/listener-replacement/throwing-listener, denied read, failed unsubscribe retry and old read retained through replacement. Future private auxiliary tree transfer remains outside this existing ACK path until registered: transfer is not ACK. The current accessor implementation must be independently reviewed after High freezes it.
