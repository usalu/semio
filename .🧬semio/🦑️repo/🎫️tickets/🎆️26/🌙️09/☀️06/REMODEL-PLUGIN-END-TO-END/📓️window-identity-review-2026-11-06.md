# Window Identity Contract Review — 2026-11-06

Root independently read the full neutral schema, two fixtures, helper, focused tests and three actual mode-window body wiring paths under the framework OS renderer ShellHost panel-hosting owner.

The metadata helper distinguishes registered app, role, session instance, concrete window instance, window kind and optional real space/document scope. Exact plugin/app/instance ownership selects a document only when one matching open entry exists; missing or ambiguous entries do not borrow another scope. Wiring uses each primary or spawned session rather than the global active navbar chip. No Remodel-specific identity is inserted into the framework.

The current focused React test renders a raw div using the helper. It proves that React can emit these attributes, not that ChromeAwareWindowScrollSurface forwards them or that useModeWindows binds the correct real session at runtime. Root requested actual component-forwarding coverage and multiple-document/same-window-kind isolation.

A further review concern is reactivity: the wiring reads a mutable openDocumentSessionsRef inside useMemo, whose dependency is the ref object. The exact field-write and session-update paths must establish whether scope changes invalidate the memo. If a genuine reactive owner snapshot/revision is required, it must be passed explicitly; fabricated epochs, global fallback identity and private test injection are not acceptable. This is a source concern pending trace, not a proven runtime defect.

Current source review and focused test results do not establish actual editor identity or saved-artifact/window UI behavior. The driver must still observe real matching metadata and visible nonempty content on the prepared host. Full browser acceptance remains unexecuted.

## Current Review Follow-Up

The body-owned DOM attributes are now namespaced data-surface-* so existing dock data-window-id selectors retain their meaning. Focused Node helper checks pass two tests; actual ChromeAwareWindowScrollSurface forwarding coverage is still pending. The driver now requires a visible matching body, genuine document scope and rendered UI node before identity may pass; no live host run has verified this path.

Resolved primary and spawned scopes are computed before useMemo and explicitly included in dependencies. A render observing changed scope invalidates the memo. The mutable map does not itself trigger a render; foreground document commit dispatches a backbone update, while universal background/abort/close notification remains unproven in the bounded source trace. No invented epoch or global identity fallback was added.

The full media contract is being brought into agreement across scenario fixtures, schema and tests, retaining JPEG, corrupt input, repeated append and finite 45-frame AVI requirements rather than reducing them to the initial three-PNG selection skeleton. Twelve was the draft scenario count, not a limit on the requested goal. Actual media/UI results are still unexecuted.

## Current Focused Proof and Entry Repair

Browser owner reports a current scoped tool invocation completed exit 0 with two files/eight tests in 8.12 seconds. Root inspected the retained receipt and source hashes in browser-recovery report, and independently read the real ChromeAwareWindowScrollSurface SSR test and current helper/fixture contracts. It covers two sessions with the same window kind and distinct document/window identities, along with ambiguous and missing scope cases. The forwarding test is now registered in the ordinary renderer element suite. The original output was tool-only, so there is no independent raw redirected log for this run; that limitation is explicit rather than a new execution invented for logging.

Root identified another concrete entry gap: a fresh standalone host page has no authenticated saved-artifact session. The accepted source design reuses existing two-human public UI setup and invokes plugin-owned observations after genuine sign-in, space/artifact creation and A/B opening. A neutral callback receives actual scope, original signal and repo-owned browser test ports implemented privately with Playwright. It must require all fifteen named plugin checks uniquely and refuse missing scope, duplicate checks, callback faults, cancellation or unexecuted scenarios. The draft standalone entry is to be replaced rather than retained as a compatibility mode. No private state injection, new credential path or second serve is required.

That integration and its live same-artifact journey remain unimplemented or unverified at this snapshot. Actual native, descriptor and full Vitest runs are still pending; the renderer SSR/helper results do not substitute for them.
