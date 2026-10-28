# Owned Staged Body Renderer Plan

Source-only map; no production changes or mounted/DOM/native execution claim.

## Reuse boundary

Existing Interpreter/node-tree ContainerView lines94–139 contains reusable presentation but is private and coupled to UiDocumentStore for child keys, recursive UiNodeView, DOM IDs and activation. Extract/offer a narrow presentation component receiving borrowed record, actual control context and already-rendered React children. For this first explicit staged subset admit only Form and Field containers without arbitrary activation bindings; reuse layoutSpecStyle, styleSpecDataAttributes, accessibilityAriaProps and activityBorderClass from existing layout-accessibility/display-views. Do not fake a store for uiChildReactKeys.

An owned body node wrapper should call existing useOwnedRead(owner.reads,id), require exact current source view/snapshot revision, and lend the resident record. Root uses actual source.view.root via useOwnedUiView. Form children are at most17 (one status plus16 fields); Field has exactly one ordinary leaf. Bound those counts and permitted depth/roles, reject cycles/duplicate IDs and unsupported layouts/components at original intake/planner rather than traverse arbitrary trees during React render. Recursive wrapper scope is this bounded page, not a new general renderer.

Status Extension installs proof via existing OwnedStagedStatusView/committed exact read, producing no visible control. Field children render existing OwnedStagedControlView/OrdinaryControlView. No source model query, full tree projection, graph copy or generic Extension host should run. Use actual borrowed child IDs as sibling React keys within the current owner/surface; each child independently reads/checks its authored key and type. Include source lifetime in the outer component key. A replaced publication invalidates old event ports immediately; speculative old read remains owned until layout acknowledgement/paid retirement.

## Accessibility mismatch to solve explicitly

UI elements/Field TSX line31 labels `${id}.control`. Native body names fields stage.field.<address> and leaves stage.control.<address>; ordinary controlDomId derives leaf key/id, so merely wrapping that control with current Field produces a nonmatching htmlFor. Existing leaf aria-label supplies accessible name but does not fix label-click focus. Reuse Field with an explicit child target (or a shared deterministic DOM-id policy that gives Field the exact ordinary target) while keeping both DOM IDs scoped by real surface/owner. Do not derive association from a copied store. Also preserve Field required/error/description and provide described-by/error association where supported; current Field text blocks alone do not establish it.

## Exact files

- Interpreter/node-tree/TSX: ContainerView presentation and current fake-store blockers.
- Interpreter/layout-accessibility/TSX: pure layout/style/ARIA readers.
- Interpreter/stable-dom-ids/TS: controlDomId and uiChildReactKeys (latter is store-dependent, avoid).
- framework UI elements/Field/TSX: Field presentation and htmlFor convention.
- Interpreter/control-port/owned.tsx: exact existing status/ordinary leaf hooks.
- UiDocumentStore/owned/TSX: useOwnedRead commit acknowledgement.
- Native declaration/view/staged-form/body.rs: exact Form/status/Field/leaf builder,16-control cap.

## Minimal authentic fixture

Extend registered UiDocumentStore/tests/owned-staged-controls TSX: publish actual typed-wire Form root→status+Field→Input/Select/Button instead of mounting known leaf IDs directly. Render only actual source.view.root. Assert field required/error/description, label click focuses exact input, actual form role/name, no invisible status DOM, text draft preservation, choice reset and page args identity. Hold a prior active root/leaf through replacement and assert old callback refused; reorder fields/replace IDs without cross-field draft reuse. Unmount/close under StrictMode and verify source retirement witness. Keep existing actual wire offer/ACK/maintenance route. Test is authored/executable target only until sanctioned renderer Vitest policy runs it; no snapshot-loader substitute.
