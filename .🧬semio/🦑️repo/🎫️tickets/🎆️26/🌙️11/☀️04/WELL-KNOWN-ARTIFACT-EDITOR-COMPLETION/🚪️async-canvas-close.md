# 🚪 Async Canvas Window Close Admission

Scoped UI AGENTS read: ui components remain domain-neutral. No host files changed.

ModeProps.onWindowClose now returns boolean, void, or a Promise of either. Canvas awaits the result before removing the dock window. False, thrown errors, and rejected promises retain layout and active focus; failures are reported. Concurrent requests for the same window share admission, different windows proceed independently, cancellation permits retry, and unmount retires pending commit authority. The hook reads current callback/focus refs after waiting.

New production module: 🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎨️Canvas/🚪️close-admission/🟦️.ts. Updated Canvas/🟦️.tsx and Canvas/🖱️mode-drag-and-window-callbacks/🟦️.tsx.

Schema-first neutral fixture: ui/🧪️tests/🧫️fixtures/🚪️window-close/{🧬️.json,🔣️.json}. Tests: ui/🧪️tests/🚪️window-close/🟦️.ts. Ajv independently validates fixture and allowed-decision oracle. Tests exercise actual production admission function, delayed commitment, duplicate requests, independent windows, retry after Keep, errors and unmount.

Test-first evidence: initial run failed importing the not-yet-created production module. Final command: SEMIO_LANGUAGE=en repo tools exec --light bun -- test ./🧰️framework/🔨️modules/🖱️ui/🧪️tests/🚪️window-close/🟦️.ts. Result: exit 0, 10 pass, 0 fail, 27 assertions, 412 ms. No full UI build/browser run performed. Host must return false (or Promise false) on Keep to retain actual dock window; root owns wiring.
