# Owned Current Intent and Async Custody API Map

Read-only current source2026-10-21. No existing original-controlled owned intent producer was found in reviewed facade; no production edits/native claim.

## Exact existing exports

UI retained/instance OwnedUiInstanceSurface:110 exposes view,subscribeView,subscribeNode,retryNotification,acknowledgeRead,unsubscribeNode and scene readers. It has no buildIntent,dispatchCurrent or event sequence API. retained/read-lease exports OwnedUiReadSource,OwnedUiReadSubscription,OwnedUiNodeReadSnapshot; snapshot.record returns existing typed node under active issued lease. UI renderer owned hook acknowledges exact snapshot only after layout commit and unsubscribes through source-owned retirement.

Typed ordinary component/binding objects are captured/frozen in retained/wire/typed; UiPayloadRetirement:99 clears owner roots/link references progressively,not fields of the ordinary frozen graph. Therefore an arbitrary JS alias to args remains physically readable after lease retirement,but it has escaped original owner accounting. Physical JS reachability is not a retained source authority/paid custody proof. Surface byte views have stronger active-owner access semantics and must never be treated as ordinary components.

UiDocumentStore.buildIntent:532 increments its own per-store bigint seq and creates small intent aliasing binding.action and binding.args;it does not capture/retain payload. emitIntent:581 selects rendered binding without currentnode lookup. Existing Button/Input paths add current checks themselves;an owned dispatch port must centralize current exact snapshot+surfaceversion+key+trigger+disabled check,then assign sequence owned by the actual stable surface lifetime. Reconstructing a fresh store or using numeric equality alone is insufficient.

## Real transport capture boundary

BrowserActorActionMailbox.dispatchIntent calls dispatchRequest synchronously. Its create callback invokes createBrowserActorUiIntentRequestV1:12,which encodes complete intent via encodePackValue and parses/captures closed request before the promise is queued or awaited. Exact surface/revision/owner/actionsequence are bound;queued request carries owned byte array. Thus the ordinary directactor path does not retain borrowed args awaiting mailbox send if this synchronous capture succeeds. This existing synchronous encoder/Array.from/parser is not an original-controlled bounded event capture proof;do not relabel it as one.

Generic ShellHelpers tree-windows uiIntentToActionDescriptor flattens aliases into args,loses source origin and has no lease retention. Reserved staged events should synchronously authenticate current binding and apply bounded source-owned metadata before this path. Any async generic event dispatch must retain issued owner until actual event capture/ack or use a first-party controlled event receiver that owns captured args. No such general retained-intent authority is exposed by the reviewed owned facade.

## Minimum additive contract

A source-owned eventread operation should accept exact current subscription/snapshot,nodekey,trigger,raw input;validate that snapshot is current/active and source publication matches;select authored binding;mint bounded exactu64 sequence;retain/capture only actual event metadata/payload under explicit original admission outside React;return a token/owned event whose retirement/ack ends the lease. Expose no arbitrary replacement binding/path authority. For immediate staged host reduction,borrow fixed bounded binding synchronously and return no borrowed args;host command args already belong to session. Presentation readers remain side-effect-free.

Tests must distinguish physical readable alias from valid retained owner,publication change beforeclick,sourceclose aftercapture but beforeawait,seqoverflow,duplicatekey,disabled,transportrefusal and speculative render. Existing owned hook and directactor mailbox tests provide component pieces;full owned ordinary-control event integration remains open.
