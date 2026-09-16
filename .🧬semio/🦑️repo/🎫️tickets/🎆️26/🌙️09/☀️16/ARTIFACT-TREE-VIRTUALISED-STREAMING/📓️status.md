# 📓️ Status — Artifact Tree Virtualised Streaming (2026-09-16)

## Goal
No `+N` continuation rows anywhere. Every tree section/group shows its complete entry set: children materialise lazily on expand, only the viewport's rows are materialised, more stream in from the guest as the user scrolls. One framework mechanism for all apps.

## Phase 1 — audit (Sonnet fleet)
- 📓️audit-host-tree-pipeline.md — React side: InterpretedUiNode, UiDocumentStore, Tree element, scroll container, expansion intents, action → guest → re-render round trip.
- 📓️audit-surface-budgets.md — SurfaceReconciler / FlatPresentedNode cost, 8 MiB cap, UiValue arena, UI_BUILT_CHILDREN_MAX.
- 📓️audit-app-panels-a.md / 📓️audit-app-panels-b.md — every plugin panel tree builder and its paging shape.
- 📓️audit-paging-plumbing-and-schema.md — setPanelPage state, action registration, Tree contract schema pipeline (Rust → JSON → TS), wgpu target.
- 📓️audit-paging-tests.md — every test/fixture that pins +N / paging behaviour.

## Phase 1 — done 2026-09-16 (all six audits landed).

## Phase 2 — design
- 📓️design-virtualised-tree.md v1 final (coordinator). Channel: ViewModel.tree_windows; contract TreeWindow + granularity; SDK TreeWindows/tree_window_section/tree_window_item; host observer + host-owned expansion; UI_BUILT_CHILDREN_MAX 32→128, UI_VALUE_PAGE_ROWS 31→128.

## Phase 3 — implementation (Opus fleet)
- Wave 1 launched: P1 contract+ViewModel (📓️p1-contract.md), P3 SDK (📓️p3-sdk.md), P4a Tree element (📓️p4a-tree-element.md), P4b Interpreter+ShellHost (📓️p4b-host-wiring.md), P5 wgpu+reconcile law (📓️p5-wgpu-and-reconcile-law.md).
- Wave 2 launched concurrently (code first, cargo gated on 📓️p1-contract.md + 📓️p3-sdk.md): A1 puzzle3d, A2 puzzle2d/5d, A3 cad, A4 fem, A5 energy, A6 procedural+process3d+block, A7 flow/dag/vcs/note/sequence/writer/animate/imperative/remodel, A8 remaining plugins (shared brief 📓️wave2-app-brief.md; reports 📓️a1…📓️a8).
- Wave 3 browser verification after wave 2.
