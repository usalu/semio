# Round 3: Diagram force test failures

## Root cause
Not an engine regression and not stale mocks. `🧪️tests/🧩️component/🧰️harness/🟦️.ts` installs `vi.mock` of `🔌️Ports/🟦️.tsx` (the `HostReactFlow` capture behind `flowCapture.props`). That mock only takes effect if the harness executes before anything imports the real Ports module.

- Run through the aggregator `🧩️component/🟦️.tsx`, which imports the harness first: all 55 tests pass.
- Run through `🧲️force-tests/🟦️.tsx` alone, the sub-suites import `Diagram` (and so the real Ports) before the harness. `HostReactFlow` is never captured, so `flowCapture.props` is undefined. That produced the `edges.read()`/`nodes.length` undefined and never-called mock symptoms in 5 tests.

## Fix
`🧲️force-tests/🟦️.tsx`: added `import "../🧰️harness/🟦️.ts";` as the first import, making the suite order-independent.

## Result
- `🧲️force-tests/🟦️.tsx`: 15/15 pass (was 10/15).
- `🧩️component/🟦️.tsx` (aggregator): 55/55 pass.

## Note
The four per-registrar files (`🔶️`,`🔸️`,`🟠️`,`🟢️`) export register functions, not suites. The ticket config glob `**/🧪️tests/**/🟦️.{ts,tsx}` also matches them, so running them directly yields "No test suite found". That is a config artefact only, not a defect.
