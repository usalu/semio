# Renderer Continuations During Document Load

The renderer-level regression calls the actual `applyDispatchActionEffect` and host continuation scheduler with the real per-instance gate and a mocked guest handle. Tests use the renderer's owned Vitest config, its jsdom environment, Node 24.15.0 and one worker under normal repository machine admission. The test otherwise leaves timing, routing, fault checks and deferred publication in production code. The language-neutral data rows cover mutation before and after readiness, effect-owner retirement both parked and in flight, view and interaction requests, archive cancellation, target instances, and the precise retryable framework fault.

The scheduler initially classified an action using the first declaration across all kinds of windows. Two red rows exposed opposite errors for an addressed background window with a shared action identifier. It deferred a view when an earlier kind declared a mutation, and dispatched a mutation while the document was loading when an earlier kind declared a view. The guest route already uses the exact window instance and kind, and the matching Rust and TypeScript manifest operations select the declarations for that kind. The scheduler now uses the existing owned invocation selection against the resolved view and classifies only the addressed kind. App commands retain their declared kind. View, interaction and archive cancellation pass during the load. A retired owner cannot dispatch or publish. Only a typed, retryable framework `document.loading` parks a raced request for one replay.

Before the correction, the real DOM suite failed exactly the two background-window rows and passed the other 30 tests. After the correction, all 32 tests passed, including all existing command-category and dispatch cases. See `🗑️generated/renderer-dispatch-target-{red,green}.log`.

The focused TypeScript check also passed the two touched files after the test's dispatch request, resolved view and mocked guest method were given their real types. The workspace check counted 32 diagnostics outside the selection. A second DOM run passed all 32 cases with the typed fixtures. See `🗑️generated/renderer-dispatch-typecheck-verified.log` and `🗑️generated/renderer-dispatch-dom-verified.log`.

Repository paths:

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/⚡️host-effects/🔁️dispatch-media-and-extension-effects/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/📣️command-categories-and-dispatch/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/📣️command-categories-and-dispatch/🔣️document-load.json`
