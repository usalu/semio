# Remodel Browser Acceptance on 2026-11-21

The browser runner now requires scoped rendered content on both editors for the identity receipt. The example receipt compares visible stream and camera IDs and frame counts against all three committed examples. Media imports require the expected shared counts and a new stream while retaining the prior streams. The JPEG check additionally needs an added durable archive member. Finalization needs both matching displayed products and a changed snapshot. Mesh and QC downloads are checked against the GLB and report contracts, and the GLB counts must match the finalized mesh shown in the editor.

The browser port now asks Chromium to cancel an actual file-chooser request. It requires a file-input chooser event and restores interception after the interaction. The Remodel receipt also requires the original and peer archives to match before and after cancellation. The browser adapter test requires a trusted DOM cancel event, no change event, and an empty file selection. The sequencing test covers valid and rejected browser input events and asserts interception cleanup in both cases.

The registered test runner was invoked through `repo`. Vitest's esbuild process failed with `EPERM` and its fork pool timed out. The ticket-local `🧪️browser-fixture-runner.ts` preload reruns the authored fixture assertions through Bun’s test API. The Remodel fixture file passed 18 tests with 154 assertions. The shared browser-port fixture file passed 9 tests. Both retain the authored JSON fixture inputs and independent SQLite and AJV checks.

```bash
SEMIO_LANGUAGE=en repo tools exec --light bun -- test --preload './<ticket>/🧪️browser-fixture-runner.ts' './✏️s/🔌️plugins/📸️remodel/🧪️tests/🌐️browser/🧪️tests/🟦️.ts' './🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/👥️two-human/🧩️opened/🧪️tests/🟦️.ts'
SEMIO_LANGUAGE=en repo tools exec --light tsc -- --project '✏️s/🔌️plugins/📸️remodel/📦️packages/🟦️typescript/tsconfig.json' --pretty false
```

The live browser-port test stopped at the loopback HTTP listen with `EPERM`. A direct Playwright launch through the registered Bun tool also stopped before navigation when pinned Chromium's `sandbox_host_linux.cc` returned `shutdown: Operation not permitted`. The repo MCP calls for catalogue, Hub, React host and browser execution returned 300-second timeouts. The native repo acceptance entry hit a read-only host-capacity directory, then a read-only resident-service directory when retried with a writable cache. A direct script invocation correctly refused without a parent invocation. No browser behavior, two-human journey, viewer content, or persisted per-window configuration was verified on this host.

The declared full acceptance route is `repo run @semio-tech/remodel-js:browser-acceptance` with `--hub`, `--serve`, `--locale`, `--kinds 3d.remodeling`, and `--out` under this ticket's generated output directory. The development Hub target is `os-hub:local-hub-owner` on port 8787, and the Remodel React prototype host uses port 6063.
