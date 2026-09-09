# Final Host Ownership Audit

Date: 2026-09-09  
Scope: read-only audit of the current OS configuration, browser host, native WGPU host, Dock, SDK projection, action, and context-menu paths. This follows `audit-host-ownership.md`, `📓️host-ui-preferences-integration.md`, and `🔬️shared-os-and-surface-ownership-audit.md`.

## Result

The five findings in the preceding host audit no longer reproduce in their reported paths. Two new P1 native action-routing defects remain. They prevent the abstraction-ownership objective from being considered complete.

## Findings

### P1 — Retained native UI actions discard their concrete window instance

`UiCommand::App` deliberately contains both `window_id` and `action` in `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🎯️events.rs:687-692`. The native host discards that `window_id`: `dispatch_ui_event` calls `apply_ui_commands` at `🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:388-393`, whose `App { action, .. }` arm passes only the descriptor at `:419-427`; `publish_retained_action` copies only pre-existing `action.args` at `:449-455`.

The eventual shell dispatch therefore reads no originating target and falls back to `active_window_id` at `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:4191-4208`. Rendering itself is per concrete ID (`render_ui_document_step(..., window.as_str())` at `:10207-10228`), so the target is known when the click occurs but is lost on its way to the action.

Trigger: render two concrete instances of the same kind, leave A active, then click a declarative button in B whose descriptor has no `args.windowId`. The retained UI command is emitted with B, but the plugin invocation is addressed to A.

Fix: preserve the `UiCommand::App.window_id` in the queued descriptor, injecting/overriding `args.windowId` at the native host boundary before the bounded action is published. Add a regression that asserts a B click remains B while A is active.

### P1 — Native context-menu activation loses its correctly resolved concrete target

The menu request correctly finds the body under the pointer (`context_window_instance_id` at `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:3107-3109`) and sends its exact `windowInstanceId` plus the live ViewModel to the plugin at `:6250-6276`. The SDK also projects that exact instance at `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:28101-28109`.

The reply has no retained target. `shell_context_menu_item_from_spec` copies plugin args unchanged and recurses without context at `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:301-317`; `ContextMenuState` has no window field at `:714-724`; activation passes the untouched descriptor straight to `dispatch_action` at `:6853-6861`. If the plugin reply is empty, the fallback chooses actions from `active_window_id`, not the clicked `window_instance_id`, at `:6302-6327`. This then reaches the same active-window fallback at `:4191-4208`.

Trigger: leave A active, right-click B, and have the plugin return an action without `args.windowId` (or no items, taking the fallback). Selecting the item addresses A; the fallback may also show A's action set rather than B's.

Fix: retain the clicked instance ID in menu state and scope every app action, including nested plugin items and fallback rows, immediately before activation. Resolve fallback actions from the clicked instance's kind. Add a regression with active A, clicked B, an empty plugin reply, and a plugin-supplied unscoped item.

## Recheck of Previous Findings

| Prior finding | Current evidence | Result |
| --- | --- | --- |
| Duplicate durable Shell preferences | Canonical `UiPreferences` is owned by OS config and contains the complete preference record in `🎚️config/🧬️schema/🦀️.rs:94-109`; applying mutations is owned there at `:187-191`. The Shell state schema contains only transient `uiDriverDraft` and `uiThemeDraft`, not durable preferences, in `🖥️shell/🧬️schema/🦀️.rs:77-80`. | Fixed |
| Native preference projection lost custom drivers/keybindings | WGPU imports the OS-config mutation API in `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:22-28`, replays canonical mutations at `:12238-12280`, and persists per-ID driver/theme/keybinding changes at `:12328-12385`. | Fixed |
| Dock collapsed instance IDs into kind IDs | `DockStackTab` stores distinct `window_id` and `window_kind_id` at `🛰️Dock/🎯️targets/🧊️wgpu/🦀️.rs:25-46`; live windows retain both at `:566-585`; layout read/write preserves `instance_id` at `:660-689`. | Fixed |
| SDK/window refresh borrowed focused context and panels carried a window | `ViewModel::for_panel` clears window, kind, and utility at `🛂️manifest/🦀️.rs:4337-4352`. Plugin refresh projects every window and panel separately at `🔌️plugin/🦀️.rs:28030-28040`; native refresh does the same at `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:3152-3170`. | Fixed |
| Context-menu request did not convey exact target | Native resolution/request use the concrete ID at `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:6250-6276`, and the plugin wire contract turns it into `for_window_instance` at `🔌️plugin/🦀️.rs:28101-28109`. | Fixed for request construction; activation remains P1 above |

## Browser Cross-check and Verification

The browser host is correctly scoped on the inspected paths: it injects the exact `windowId` into every encoded window action at `🏛️ShellHost/🟦️.tsx:808-833`, projects context-menu requests by concrete window or panel at `:3758-3770`, and loads all canonical custom driver/theme/keybinding projections at `:1720-1742`.

Ran:

```text
NX_DAEMON=false bun nx run @semio-tech/framework-renderer-react:test -- long '../../../../🧪️tests/🔬️engine-contract/🟦️.ts' --run --silent=false --reporter=verbose --testNamePattern='maps context menu specs onto UI items|enriches context menu shortcuts from app keybindings'
```

It passed: one file, three selected tests, 440 skipped. The captured output is `🗑️generated/audit-host-browser-context-menu.log`. This browser check does not exercise either native P1 path; the specified native regression tests are still needed with the fixes.

## Coordinator Fixes and Pending Native Validation

The retained Interpreter now carries UiCommand.window_id into its bounded action publication and overrides any conflicting windowId in the descriptor. The same rule applies to drop payloads. Shell context-menu construction scopes every nested app item after plugin-specific resolution; target identity remains attached through pointer and keyboard activation. Empty-reply fallback resolves the clicked concrete instance and its declared window kind; panel invocations no longer borrow a focused-window fallback menu.

Three native laws share a language-neutral fixture: retained action queue target preservation, nested menu action target preservation, and actual fallback menu construction with another window active. The independent fast-json-patch RFC 6902 oracle passed all three fixture vectors with DEBUG output via Bun/Nx. The native tests are queued behind the WGPU dependency build; they have not passed yet. The existing drop admission law also checks that payload args cannot replace the host target.

Exact new or edited paths: Interpreter native source and wgpu-ui-command-wiring tests; Shell native source and wgpu-command-registry tests; engine/🧪️tests/🔬️window-action-context/🔣️.json and 🟦️.ts; root 📜️script.ts and 📋️project.json; .vscode/🧩️launch.seed.jsonc and .vscode/launch.json; ticket validation/project.json and validation/📜️script.ts.
