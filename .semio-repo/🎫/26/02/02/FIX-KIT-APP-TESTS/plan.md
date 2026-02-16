# Plan: Fix Kit App Tests

## Problem
The Kit App tests are failing with two primary issues:
1. `ReferenceError: require is not defined` in `test-selection-tools-simple.spec.ts` due to usage of `require("fs")` in an environment that likely expects ESM or doesn't support Node.js built-ins directly in the test execution context if not properly configured.
2. `Error: browserType.launch: Target page, context or browser has been closed` in `selection-tools.spec.ts`. This indicates the browser is crashing or failing to launch properly.

## Goals
1. Fix `ReferenceError` in `test-selection-tools-simple.spec.ts`.
2. Fix browser launch issues in `selection-tools.spec.ts` and ensure tests can run in the devcontainer environment.
3. Ensure all tests in `js/semio/playwright/kit/` pass.

## Steps
1. **Fix `ReferenceError`**:
   - Replace `require("fs")` with `import * as fs from "fs"` if possible, or use Playwright's filesystem capabilities if applicable.
   - Since these seem to be running in Node.js (Playwright runner), `import` should work if the file is a module, or `require` if it's CJS. But since it says `require is not defined`, it's likely treated as ESM. I will change it to dynamic `import()` or static `import` at the top level.

2. **Diagnose and Fix Browser Launch**:
   - The error `exception while trying to kill process: Error: kill ESRCH` suggests the process died before Playwright could connect or manage it.
   - I'll check if dependencies are missing using `npx playwright install-deps`.
   - I'll check if I need to use `--no-sandbox` explicitly, although the logs show it is being used.
   - I will try running a minimal playwright test to see if it's a global issue or specific to these tests.

3. **Verify Tests**:
   - Run the tests again and ensure they pass.

## Todo
- [ ] Fix `test-selection-tools-simple.spec.ts` `require` usage.
- [ ] Diagnose browser launch issue.
- [ ] Verify fix.
