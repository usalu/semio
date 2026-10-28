# Owned Form Renderer Port and Constituent Checkpoint

Read-only source review, 2026-10-21; no production changes, runtime interaction or native execution.

## Constituent SetValue

Shared IFC details work.rs:470 now preserves admitted part/argument for actual SetValue with an existing argument, even when ReferenceReduction returns no special request. Other event families keep their earlier fallback behavior. The condition does not widen reference admission. Work.rs:595–599 still runs DomainWork whenever reference.request is absent, including source enumeration choices and these ordinary constituent SetValue routes. Thus declaration/derived/optional/bounds validation is not skipped by this routing change. Existing parts clone/value staging and SetEntityComplex mutation path remain responsible for actual output and inverse. No new confirmed defect in this narrow condition review; native custody/arity/inverse test sources are unexecuted. The worker's ten TS/fast-json-patch cases are reported evidence, not independently rerun here.

## Exact owned consumer blocker

Renderer element paths below are relative to `C:/git/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements`.

`📃️UiDocumentStore/📖️owned/🟦️.tsx:15 useOwnedUiView` subscribes exact immutable root metadata. `useOwnedUiNode:41` subscribes through source.subscribeNode and acknowledges the exact issued snapshot only in committed layout effect. These are usable owned React ingress hooks, not a full ordinary-control renderer.

The underlying `C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️.ts:110 OwnedUiInstanceSurface` exposes stable view/read subscriptions, acknowledgements and bounded scene readers. It deliberately has no mutation or replacement-root authority.

In contrast, `🗣️Interpreter/🟦️.tsx:79 InterpretedUiNode` requires concrete UiDocumentStore; `🗣️Interpreter/🧱️node-tree/🟦️.tsx:558 UiNodeView` uses useUiNode(store,id), and renderComponent/Input/Select/Button consume UiNodeRecord and store operations. Existing context's store supplies current record lookups and buildIntent; tree/table/container traversal also accesses concrete store state. There is no reviewed production entry that directly renders owned RetainedUiNodeRecord through ordinary Input/Select/Button with exact lease-backed authority.

Therefore feeding owned intake through builtNodeToSnapshot/loadSnapshot or a complete legacy tree projection does not establish original-owned rendering. Do not report the auxiliary form mounted through an owned renderer merely because its source was acquired with ownedUiInstanceSurface.

## Minimal source-owned port

Refactor the ordinary interpreter dependency behind a narrow per-node read/intent interface, with owned implementation using the existing useOwnedUiView/useOwnedUiNode subscriptions and exact original snapshot/lifetime. Keep ordinary component views shared. Resolve children incrementally from admitted records; do not eagerly convert/copy a whole surface into UiDocumentStore. Event dispatch must revalidate current issued resident node/binding/disabled state against the live source. An immutable record preview is not by itself current event authority. Preserve subscription read acknowledgement and retirement instead of manufacturing store revisions.

The optional presentation/intent hook can live at InterpretedUiNode/context and flow through existing renderComponent to InputView/SelectView/ButtonView. Presentation changes only displayed draft values; resident records/bindings stay unchanged. The reserved staged action must be intercepted before intent→ActionDescriptor flattening, with explicit refusal consuming the reserved intent rather than falling through to arbitrary app action dispatch. This hook improves the ordinary interpreter but does not bridge the owned-source type/lifetime boundary by itself.

Required integration coverage: owned node retirement during speculative render; exact committed acknowledgement; control replacement between display/event; disabled control; stale sequence/revision/node key; bounded child traversal; source close during pending refresh; reserved intent refusal cannot fall through; ordinary nonreserved actions and existing blur publication lanes remain unchanged. No such new integration was executed in this audit.
