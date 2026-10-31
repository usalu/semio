# Unit Verification After Fixes

2026-10-30. Unit suites for `@semio-tech/course-react` and `@teaching/architecture-course`, run with `repo tools exec vitest -- run` and each package's Vitest config (`--configLoader native`). No Nx graph rebuild.

## @semio-tech/course-react

Config: `framework/products/course/targets/react/tests/config`.

43 files, 1082 tests, passed in 38.86s.

That config includes `tests/navigation`, `tests/home-grid`, `tests/mockups`, and `tests/adaptive-layout`.

## @teaching/architecture-course

Config: `teaching/architecture/course/tests/config`.

First run: 19 files, 823 passed, 3 failed. `tests/course`, `tests/pet-cast`, and `tests/deploy` passed. The failures were in `tests/presence`:

- settles the deadline presence handshake like ws
- settles the parent-cancelled presence handshake like ws
- settles the already-cancelled presence handshake like ws

Each hit the test's 5s limit. The owned handshake (`waitForPresenceSocket` / `presenceUpgradeStatus`) already returned `timed-out` or `aborted`. The `ws` oracle never settled. A bare `require("ws")` inside Vitest is rewritten to a client that stays `CONNECTING` and never applies `handshakeTimeout` or `AbortSignal`. Loading `index.js` next to the resolved `ws/package.json` is the real client: a silent handshake fails in about 30–90ms, and an aborted signal fails as `AbortError`.

Fix: `loadWsOracle` in `teaching/architecture/course/tests/presence`. No production change.

After the fix: 19 files, 826 tests, passed in 11.21s. A follow-up run of the presence file, including swipe progress, passed 6/6.

## Change

- `teaching/architecture/course/tests/presence` — load the installed `ws` package by its package entry so the handshake oracle matches Node's `ws` rather than Vitest's rewritten specifier.
