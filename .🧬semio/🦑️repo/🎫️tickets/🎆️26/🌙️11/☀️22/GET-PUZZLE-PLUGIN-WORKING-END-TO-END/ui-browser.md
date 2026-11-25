FINAL

# Puzzle Browser Verification

The Linux sandbox blocks local TCP sockets. `repo capacity run --memory=2000 -- timeout 1500 repo tools exec --light node -- node_modules/storybook/dist/bin/dispatcher.js dev -p 6020 --ci --no-open --disable-telemetry` in `puzzle,infinite` scope exited 1 with `uv_interface_addresses` errno 1 before serving. An independent native Node listener on `127.0.0.1:6020` exited 1 with `EPERM`. The default capacity queue path was read-only, so the Storybook attempt used `SEMIO_HOST_CAPACITY_DIRECTORY` under ticket-generated output. The prototype cache contains no activated puzzle sessions or modules. Browser screenshot and interaction assertions require an environment with loopback access and delivered artifacts.

An independent Playwright `chromium.launch` probe used the pinned headless Chromium with `--remote-debugging-pipe` and prepared a fully intercepted HTTP response. The Chromium process exited `SIGTRAP` before creating a page, with `content/browser/sandbox_host_linux.cc:41` reporting `shutdown: Operation not permitted (1)`. The full Chromium channel with single-process launch also exited before creating a page in this sandbox.

Playwright discovery independently failed before browser launch on both puzzle files with `ERR_IMPORT_ATTRIBUTE_MISSING`. Added JSON import attributes to the four puzzle fixture imports and the renderer's terrain fixture import, which the same registered suite loads before the puzzle cases. Strengthened the spatial suite's 5D timeline check to verify that the last checkpoint leaves the expected part, clicking that part selects it, and clicking the background clears both the story and host selection. The existing language-agnostic world fixture now carries the expected part ID from the first Nakagin DSL row.

`repo test-file` routes the Playwright files to Bun's plain runner, which cannot register Playwright `test()` declarations. The registered Playwright runner is `.storybook/🧪️tests/🧪️browser-runner/🟦️.ts`. `repo test-file` does run the board catalogue-drop unit test under Bun, but its Vitest-only `toHaveBeenCalledExactlyOnceWith` matcher threw. The replacement compares the entire mock-call list to the same expected one-call sequence.

The targeted renderer Vitest suite exposed a stale world engagement test double. `useActiveUtilityState` reads `core.interaction.activeUtility` during render, so all three fixture cases threw before dispatching input. The test double now supplies that field both when the cut utility is armed and when select is active. Expected pointer events are unchanged.

| Verification | Exit | Result |
| --- | --- | --- |
| `repo tools exec --light node -- node_modules/@playwright/test/cli.js test -c .storybook/🧪️tests/🧪️browser-runner/🟦️.ts --list -g puzzle` | 0 | 20 puzzle Playwright cases listed, count confirmed with the native output because the wrapper suppressed stdout |
| Playwright `--list` with ticket `p4-pw.config.ts` | 0 | 23 cases listed, including three Infinite cases |
| `repo test-file` on puzzle Storybook index | 0 | 6 passed |
| `repo test-file` on board catalogue drop | 0 for file | 5 passed |
| `repo test-file` on Storybook scope resolution | 0 for file | 13 passed |
| `repo test-file` on the 5D lifecycle plan | 0 | 1 passed |
| `repo run @semio-tech/framework-renderer-react:test -- --maxWorkers 1 --reporter verbose '👆️engagement-pointer' '👻️catalogue-drop'` | 1 | Graph daemon startup lock under `/run/user/1000` is read-only |
| Renderer owner's Vitest config, `SEMIO_TEST_LEVEL=long`, same two files, native Node through `repo capacity run` | 0 | 8 passed, 2 files. The harness emitted a duplicate-Three.js warning |
| `git diff --check` on the six edited source and fixture files | 0 | Clean |
| Ticket `p4-tsconfig.json` under `repo capacity run` | 124 | The native TypeScript invocation reached its 100-second timeout without diagnostics. A prior `repo tools exec` invocation exited 2 without diagnostics |

## Staged Browser WASM Probe

`repo capacity run --memory=2000 -- timeout 100 node <ticket>/ui-wasm-smoke.mjs .cache/b/p/m/browser/prototype/wasm-packages/d30e56a055ea2d5bde08382c` exited 0. The stock browser module dated November 20 initialized in Node without a server. Its 2D parser emitted 180 nodes and 179 edges from Nakagin. The board session accepted the projected snapshot using the same field vocabulary as its Storybook host, then emitted selection and clear-selection events from simulated screen gestures over a single-node snapshot. The 3D parser read Concrete Forest and Nakagin as 1 and 180 objects. The 5D parser matched all three checked-in count pairs (1/0, 180/179, 2880/2864) and verified Nakagin's first part ID against the updated browser fixture.

The probe executed the staged engine WASM exports and board session independently of browser rendering. The live editor, GPU canvas, user input transport, archive restart, and publication receipt remain unverified here.
