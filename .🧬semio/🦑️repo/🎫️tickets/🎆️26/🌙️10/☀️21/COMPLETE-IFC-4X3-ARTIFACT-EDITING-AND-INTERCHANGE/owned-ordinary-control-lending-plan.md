# Exact Owned Ordinary Control Lending Plan

Read-only actual typed payload/read lease/ordinary views2026-10-21. This refines owned-control-minimum-port.md: no new component projection is necessary for ordinary Input/Select/Button.

## Existing first-party data already usable

UI contract retained/wire/typed/TS:9 defines RetainedUiComponent as ordinary Component excluding surface plus special byte-backed surface variant. RetainedUiNodeRecord is Required<UiNodeRecord> with that component union. Thus ordinary Input/Select/Button component fields,layout,style,accessibility,bindings and children are already captured typed owned payloads. OwnedUiNodeReadSnapshot.record at read-lease:89 returns node.value directly under active exact issued root; it does not recursively decode or copy a record during React render.

Use the existing useOwnedRead subscription/commit acknowledgement hook to lend this exact record. Narrow component.type to the ordinary control subset; TypeScript can then prove the ordinary Component representation without object spread/capture/map. Do not cast special surface payload into ordinary Component or reconstruct a full record merely to satisfy UiDocumentStore typing. Its byteview differs deliberately.

## Concrete narrow interfaces

A shared control context can contain a stable source identity token; exact surface ID accessor for DOM scope; current leased record reader; revision subscription; dispatchCurrent(recordId,key,trigger,rawInput); ordinary action/menu callbacks; localDocumentOwner; optional displayed-draft resolver. Render entry obtains one exact issued node snapshot through existing hook and passes borrowed record to shared views. DOM helper uses existing uiNodeDomId(surface,key,id,scope),requiring no store.getState tree.

Current reader must use the active subscription's latest issued snapshot and publication version,not retain the original render record forever. Button/Input blur recheck id/key/type/disabled against this reader;Select generic dispatch also goes through current reader. Failed/retired lease yields no record/refuses event. Never acknowledge speculative reads;existing useLayoutEffect ack remains exact. Unsubscribe hands outstanding owners to original maintenance close.

Intent creation is its own source-owned operation: verify live exactsurface/publication/node/key/trigger,select current authored binding,and mint monotonic seq. Return/dispatch only after argument custody is retained/captured by actual existing event transport. Borrowed binding args must not outlive its issued owner through an unowned async closure. For staged reserved bindings,the host can synchronously validate/copy bounded fixedmetadata before releasing read;command args stay host-owned session data. Do not turn an ordinary snapshot object into original event authority solely because its fields match.

## Existing shared paths to adapt

InputView current reads getNodeSnapshot (177,183) and subscribeRevision(189) for blur commit publication. ButtonView:55 current lookup+key/type/disabled and stable source identity in pending bucket. action-dispatch:20 emitIntent uses buildIntent. stable-dom-ids:29 currently store.getState.surface but uiNodeDomId already accepts scalar surface. These can delegate through narrow interface without changing actual React Input/Select/Button rendering. Existing concrete store implements the same port;owned source uses exact subscription readers. Container child traversal and tree/table paging need separate bounded selectors,not a whole-tree port.

Tests: direct ordinary controls from actual owned wire intake;no record/component capture during render;speculative render notacked;committed read ack;replacement/retirement between render andclick;current disabled/key/version refuse;event args ownership through asyncsend;blur publication receipt;buttonbusy/sourcechange;select optimisticpage;DOM namespace two surfaces/sibling scopes. Existing owned-intake-and-surface tests exercise hooks;ordinary control tests exercise shared views. Add integration between them with authentic owned fixture rather than convert through builtNodeToSnapshot/loadSnapshot.

No production edits,new runtime library,full browser/native original-funded or mounted form claim. Typed payload admission already occurs outside React;any additional bounded selector must likewise be admitted/quoted outside render and retain its own source owner.
