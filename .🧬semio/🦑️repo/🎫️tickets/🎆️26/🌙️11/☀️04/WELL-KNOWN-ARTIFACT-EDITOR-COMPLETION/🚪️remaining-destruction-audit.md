# Remaining Destruction and Interface Audit

Current source reviewed on 2026-11-04. Read-only review; no broad TypeScript compilation or browser execution.

## Current Interface Findings

No concrete source-level TypeScript mismatch found in kept recovery: owner.handle is the renderer PluginWasmHandle, identity uses parent_document_id, archive reading is guarded, kept.surface is checked before access, receipt null is refused, and viewState is built from captured ActiveSession. stillKept fences current schema, target, admission timestamp and exact surface before writing, attaching and after attachment. This is source review, not a semantic compiler proof.

Natural-scope and actual callback tests deliberately cast partial mocked cores through unknown. Their passing runtime assertions do not prove complete host interface assignability. The handle contract requires createApp/destroyApp, readHistory, owned UI surface operations and other channel methods omitted by focused mocks. No new public handle method is needed by retirement; exact object identity is the ledger owner.

The standalone surface-switch test imports describe/it from Vitest at line21 and belongs in the Vitest include. Both ledger callsites469–475 and embedded bare-node twin343–345 now supply the stable owner argument (the ledger object). An included filename alone does not prove collection; root reported the queued run resolved only6files,54tests,exit0,17.71seconds.

## Concrete Remaining Scope Gaps

1. Embedded contributor actors are outside captured primary/spawned retirement owners. kernel/extensions retireContributorInstances48–51 directly destroys entries. ensureContributorInstance55–62 accepts arbitrary manifest app IDs, with no viewer/read-only restriction. ShellHost/contributions-refresh232 retires them on session changes;274 retires them for unmounted window bodies. An embedded editor can therefore acquire its own mutable native document and be discarded without persistence or Keep. Existing renderer-types-and-forms tests270–336 cover contributor disposal ordering, not edited-document admission. Suggested neutral regression: editable contributor with non-null document identity, edit, unmount its parent body, choose Keep; require durable archive acknowledgment before destruction, or refusal retaining the body. Do not assume contributors share the parent document.

2. Shell unmount cleanup in registers-and-context-menu339–364 drains then destroys primary and spawned apps and disposes handles without retirement admission. Browser unload cannot reliably await arbitrary async persistence; distinguish explicit host teardown (can await captured-owner admission) from abrupt browser termination (requires earlier durable edits and an honest unload warning). Current final-window close coverage does not establish either.

3. Legacy retireSessionInstance92–105 in update-space-panel resolves a handle by pluginId and destroys in finally even after attachment drain failure. Its current production switch fallback is the documentSource archive-handoff path; ordinary switches use the lease. It is therefore not a demonstrated ordinary Keep bypass, but a handoff failure test should prove transferred archive ownership before invoking this cleanup. Do not reuse this helper for general editable retirement.

Archive import now checks exact captured host, exact loaded handle and plugin operation admission before creation and after load; its direct catch destruction applies to the newly created unpublished failed instance. Natural-file and kept-open catch cleanup similarly concern unpublished candidates. Background space-index and directory-home owners are specialized persistence clients, not proof that arbitrary artifact editors are safe.

## Priority Correction: Authored Reload Path

The contributor API permits mutable editors, but this review has not identified an authored well-known editor contribution route. Treat it as a general framework coverage boundary, not a demonstrated canonical journey failure.

A concrete current path exists for browser reload: host-mode57 enables useBeforeUnloadGuardV1 solely from Object.values(syncStatusByDocumentId). Native palette editors are created and editable before their first Save attaches an actor. They consequently have no sync-status map entry or kept catalog entry. Positive native edits are not checked by the unload guard. Reload can proceed without confirmation, cleanup destroys the unbound actor, and recent files cannot recover it. Suggested actual-hook regression: empty actor status map, spawned artifact editor with positive native history, dispatch cancelable beforeunload, assert defaultPrevented. A robust fix must either retain those native drafts durably before reload or include captured unbound document owners in the guard; async close persistence by itself cannot run reliably at browser unload.
