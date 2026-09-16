# 📓️ Status — Artifact Tree Virtualised Streaming (2026-09-16)

## Goal
No `+N` continuation rows anywhere. Every tree section/group shows its complete entry set: children materialise lazily on expand, only the viewport's rows are materialised, more stream in from the guest as the user scrolls. One framework mechanism for all apps.

## Phase 1 — audit (Sonnet fleet)
- 📓️audit-host-tree-pipeline.md — React side: InterpretedUiNode, UiDocumentStore, Tree element, scroll container, expansion intents, action → guest → re-render round trip.
- 📓️audit-surface-budgets.md — SurfaceReconciler / FlatPresentedNode cost, 8 MiB cap, UiValue arena, UI_BUILT_CHILDREN_MAX.
- 📓️audit-app-panels-a.md / 📓️audit-app-panels-b.md — every plugin panel tree builder and its paging shape.
- 📓️audit-paging-plumbing-and-schema.md — setPanelPage state, action registration, Tree contract schema pipeline (Rust → JSON → TS), wgpu target.
- 📓️audit-paging-tests.md — every test/fixture that pins +N / paging behaviour.

## Phase 2 — design
- 📓️design-virtualised-tree.md (coordinator).

## Phase 3 — implementation (Opus fleet)
- pending.
