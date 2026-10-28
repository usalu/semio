# Browser Staged Draft and Surface Lifecycle Audit

Read-only source checkpoint, 2026-10-21. No browser interaction or native execution was performed. This is a hookup recommendation for the unmounted guided form, not a regression claim against an already mounted feature.

## Displayed values and resident authority

Source paths below are relative to `C:/git/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements`.

- `🗣️Interpreter/✍️input-views/🟦️.tsx:58` has `useCommitDraft`, but InputView selects its draft only for `commit === "blur"` (152,259,281). Immediate ordinary Input/longText dispatches change while rendering the old published tree value. SelectView (309,321,328) also renders `component.value` directly. Thus waiting for an asynchronous retained refresh is insufficient to preserve immediate typed text/selection. Number/color continuous lanes also need explicit staging display coverage, rather than assuming the blur mechanism handles them.
- The blur mechanism has document publication revisions, an acknowledgement queue, conflict handling, original document lifetime and current record checks. Reusing it as an ephemeral form receipt without an actual publication contract would misstate its authority.
- `📃️UiDocumentStore/🟦️.tsx:532` builds UiIntent with surface/revision/node/nodeKey/trigger and separate input. `🛠️ShellHelpers/🌳️tree-windows/🟦️.tsx:145` flattens this into ActionDescriptor and merges input over binding args. Form interception must precede this flattening. The store record and authored binding remain the authority.

## Minimal display overlay

Add an optional presentation resolver to the existing interpreter context/provider, scoped to the actual active form surface and action/window owner. InputView/SelectView ask it for a displayed value; they still dispatch from the unchanged resident record. This uses the ordinary renderer, not a second form renderer or a fabricated tree.

The resolver must first validate the exact current resident binding and token through the same original-intent guard. Derive field paths, fixed choices and raw-text slot exclusively from that binding. Read the effective value from host-local staged args (or a bounded per-control raw draft registered by that binding). Never accept an event-supplied path, reconstruct fields from the small status Extension, rewrite resident bindings, or treat an optimistic value as a completed native validation result.

Preserve partial strings such as empty, minus, decimal-prefix and incomplete exponent exactly. Displayed text is not parsed numeric output. Choice controls need the resident choice-key to staged value correspondence; use only the current rendered options. Disable Execute while validation/layout is pending. For layout-preserve text edits, keep the current completed binding authority and overlay newer local text. For layout-replace choices, pause old descendants until the replacement tree is accepted, rather than permitting edits through a layout that may no longer exist.

A stale native body must never assign its older args back into host-local drafts. Track request generation independently of tree revision: accept readiness/status only for the active owner/action/token and submitted generation; ignore superseded responses. Only a matching current response may acknowledge the overlay. Reset, source/document replacement, action switch and window retirement invalidate the owner; a late response cannot recreate it. Do not put every keystroke into an Input draftTarget that forces remount/reset.

## Surface retirement route

`🔌️PluginRuntime/🖼️surface-refresh/🟦️.ts:207` emits SurfaceVisible for requested surfaces. Repository-wide renderer search found no production browser SurfaceHidden emission. `🐚️Shell/♻️slice-reducers/🟦️.ts:82` changes expanded action state only; collapse/switch is not an existing hide bridge. Therefore the new auxiliary form consumer needs an explicit retirement effect, not an assumption that removing JSX or omitting a future refresh hides its mounted native context.

The real shared route exists: framework kernel `🔨️modules/🎠️kernel/🦀️.rs:718` declares SurfaceHidden; plugin WIT conversion maps it; plugin `⚛️reactor/🔄️turn/📨️event-dispatch/🦀️.rs:66` receives it and calls `⚛️reactor/🏃️runtime/🧰️ops/🖥️surface-render/🦀️.rs:47 plugin_hide_surface`, which removes the exact surface context. Submit that event through the same actual instance actor/lifecycle turn path used for SurfaceVisible, with the exact previously consumed surface ID. Do not reserve an unused window alias.

Maintain the actually mounted/consumed auxiliary-surface set. Diff it against active forms after collapse, action switch, window close and instance retirement; retire departed surfaces once. Invalidate local generation immediately before awaiting hide. Actor ordering must prevent an already queued older Visible from remounting after hide (cancel/suppress stale refresh before dispatch, and order already submitted work before hide). A hide removes context; it is not itself proof that every retained tree/work owner has been physically retired—preserve existing controlled owner close paths.

## Required executable scenarios

1. Hold refresh pending, type several characters in immediate Input and longText; every keystroke remains visible, partial text remains exact, authoritative resident binding unchanged.
2. Delay response A, submit B, deliver B then A; A cannot overwrite B text, selected key or readiness.
3. Reset during pending refresh; late response does not restore old draft. Repeat document replacement/action switch/window close.
4. Invalid/disabled/read-only/current-node mismatch intent changes neither overlay nor staged args; forged args/path/fixed choice rejected before ActionDescriptor flattening.
5. Layout-preserve edit keeps focus/selection; layout-replace choice pauses stale descendants and accepts only new resident controls after completion.
6. Collapse/switch/close submits exact SurfaceHidden for each consumed form, no synthetic roster entry; queued older Visible cannot remount it; retry/duplicate hide is harmless.
7. Two windows with same action maintain separate draft, request generation and exact real-window context.

These are recommended integration tests, not executed evidence. Native guarded ViewContext ingress and controlled retirement remain separate audit gates.
