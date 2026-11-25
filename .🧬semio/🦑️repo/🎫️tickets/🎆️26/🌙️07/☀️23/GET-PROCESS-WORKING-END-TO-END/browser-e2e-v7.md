# Browser E2E v7: Concrete Forest switch, with feedback

## Result: GREEN. The full scenario (`process-interactions.json`) now includes switching to Concrete Forest and exits 0 with no console errors: No example, Demo, Concrete Forest, Viewer, Editor, Increase, Undo. Screenshot of the v6 state stays `e2e-process-v6.png`; v6 notes in `browser-e2e-v6.md`.

## Root cause of "picking Concrete Forest fires no command"
It is neither never dispatched nor refused. Instrumented trace (temporary `[DEBUG]`, removed): `dispatchActiveExample` runs, the handoff runs, `handleAction setActiveExample` starts and completes in about 110 console entries, and the shell then waits for the operation to settle. The concrete-forest document takes about 2.5-3 minutes to load in the browser (the earlier 15-40 s waits of the harness simply ended before it). During that time the combobox kept showing `Demo` and the status bar showed `Loading document: 127 of 127` without moving, so a user saw no reaction for the first ~20 s and a stuck-looking counter afterwards. No Work, fuel or authority refusal occurred: the load is slow, not refused (the host-lifetime-fuel exhaustion of the editor test is a test-host construct; the browser host runs the load in its own turns). The speed itself belongs to the record-frame work another agent is doing.

## Fixes in the product
- `ShellHost/🔎️command-bar/🔶️scope/🟦️.tsx` (`dispatchActiveExample`): on every non-empty example pick the user gets an immediate localized notice "Opening example X… Large examples can take a few minutes." (en/de). A refused outcome shows a localized error notice with the example name and the input ledger's localized refusal text; a thrown dispatch shows the same error notice with the message instead of being dropped by `void`. The navbar handler no longer fire-and-forgets an unhandled promise (`void` became an explicit handled chain; the user is told).
- `🎠️kernel/🧭️node-kind-trigger/🟦️.ts`: labels `openingExample` and `exampleRefused` (en, de).
- Law: `📇️registry/🧪️tests/🧭️node-kind-trigger/🟦️.ts` ("the cancelled and unavailable texts equal the language-agnostic fixture") now also pins both new labels in en and de against the fixture `🧫️fixtures/🧭️node-kind-trigger/🔣️.json` (green; the other 22 tests of that file fail in this bun-only runner for environmental reasons and fail identically without my change).

## Harness
`verify-process-firefox-bidi.ts` steps accept `until` (poll an expression every 5 s up to `timeoutMs`, default 300 s, recorded as a failure on timeout). The Concrete Forest step waits until the combobox shows Concrete Forest and no `Loading document` text remains (about 3 minutes).

## Not fixed
- The document-load counter sticks at `N of N` for the whole brep load (no progress while the guest builds the model); a real progress signal needs the loader work in progress by the other agent.
- The initial load of the editor and each example load are slow (35-65 s first canvas, 3 min Concrete Forest).
