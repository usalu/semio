# Window Context Ownership

The framework's `ViewModel` already owns utility selection per concrete window. The public action decoder validated that view and then discarded all fields except active mode. Plugins consequently had no canonical utility/window context in their typed command handlers.

`ActionMeta` now carries an optional `ViewModel`. UI action entry points project it to the structurally addressed window and preserve it through both JSON and packed command dispatch. Headless internal operations have no view. The projection validates window identity and kind before dispatch.

Rust `ViewModel::for_window_instance` and TypeScript `windowViewContext` implement the same window projection beside the manifest types. They read utility selection exclusively from the map keyed by concrete window identity. An absent utility entry means no selected utility; it cannot inherit the focused window's utility. Unknown windows produce no projection. OS locale and terminology remain part of the host projection.

The shared four-case JSON fixture covers two windows of the same kind with distinct utilities, another window with no utility, and an unknown window. The TypeScript test failed for the missing implementation before the function was added, then passed through Bun and Nx with `cases=4 isolation=valid preferences=preserved`. The Rust counterpart compares the projection with the independent `serde_json` fixture lookup; Rust execution is pending.

The JSON refresh path now uses the same projection instead of falling back to the focused window's utility. The host agent is applying the TypeScript helper at its target-window boundaries. Utility consumers are being updated by the Sol implementation lane.

Inspection of the active channel found a second loss of context: `surface-visible` carried only a surface identifier, and the reactor rendered every dirty surface using `"{}"`. The WIT record and kernel event now require a separate render body key and packed host `ViewModel`. The renderer selects a concrete surface identity independently from its body key. This distinguishes two windows that render the same body.

Each plugin app instance now retains bounded ephemeral surface contexts. Dirty rerenders and renderer-driven intents read the context for their exact surface. Later host action/view updates reproject all mounted surfaces; hiding a surface releases its context. The language-neutral surface fixture covers sibling windows, preference changes, utility clearing, hiding one surface, and a separate app instance. Its Rust assertions compare against an independent `serde_json` map oracle. Runtime and wire validation are pending; outgoing React and WGPU adapters are owned by the host lane.

## Native Projection And Surface Lifecycle Validation

The isolated Bun/Nx `window-view-native-test` passed: one Rust fixture test, zero failures, 216 unrelated tests filtered. The surface context fixture now covers panel isolation from projected window context, closed/hidden-window cleanup, full-capacity replacement after closing windows, and the actual `plugin_mount_surface` → `plugin_render_surface` → test app renderer path. Its SDK native test is compiling; no result claimed yet. Packed command invalid-context branches were corrected to leave the dispatch block after emitting their fault rather than using an invalid `continue`.
# Body and Surface Identity Follow-Up

The SDK still appended a concrete window ID to the app's declared body key. With real ViewModel delivery, this breaks apps that exact-match their authored body and makes window identity depend on a string convention. Removed that suffix construction; plugin consumers of the obsolete suffix are assigned to the execution lanes and must read `ViewModel.window_id` instead.

Render-plane presence had the same issue: a later drain prefixed an app-instance ID onto a body key, which does not identify two windows of the same kind. `plugin_render_surface` now captures the component tree and its pending presence together under the same mounted surface context, qualifies every presence update with that exact surface ID, and discards the outbox if rendering fails. The reactor publishes that paired result. The former separate presence-drain API has been removed. Native validation remains pending.

## Concrete Surface Presence Binding

Presence now leaves the SDK in the same operation as the component tree and is bound to the concrete mounted surface ID. This prevents two windows sharing one body key from collapsing their presence updates onto a body-key address. The SDK passes the authored body key unchanged and projects the window separately in `ViewModel`. A neutral fixture and real app render regression cover two sibling windows and a panel with actual selection-derived presence, comparing output against serde JSON. Native execution is still pending the SDK compilation repairs; this is not yet a passing-test claim.

## Host Audit Correction

The independent host audit found generic `plugin_refresh_ui` forwarding focused window fields into panel renders. The refresh path now constructs `for_panel()` once for the panel batch. A real-app runtime regression sends a focused right-window request and checks that the panel receives the full neutral panel projection via serde JSON. Execution remains pending the current native retry.
