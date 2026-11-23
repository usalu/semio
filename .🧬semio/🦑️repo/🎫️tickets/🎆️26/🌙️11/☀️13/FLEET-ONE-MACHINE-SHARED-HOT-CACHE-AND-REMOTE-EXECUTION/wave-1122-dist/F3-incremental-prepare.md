# F3 incremental prepare stage order

Root cause: `bootstrap` in `LIB/🏗️build/📏️incremental/⌨️command/🟦️.ts:86` was a hard-coded list `preflight, tools setup all, dependencies sync --all, tools setup all`. The first `tools setup all` loads workspace packages (`@semio-tech/process`) that only exist after `dependencies sync`, so `repo incremental prepare --fresh` always failed in a fresh clone.

Change:
- `🧬️schema/🔣️.json` Policy.const.bootstrap: declared stages `preflight` -> `dependencies` (command `dependencies sync --all`, requires preflight) -> `tools` (command `tools setup all`, requires dependencies). The redundant first tools setup is dropped.
- `⌨️command/🟦️.ts`: `incrementalBootstrapV1(stages)` derives the command list from the declared data and throws "Bootstrap stage X runs before Y" when a stage precedes a requirement; `bootstrap` is built from it.
- Tests: `🧪️tests/🟦️.ts` new law (declared order, requires relation, reversed order refused); `🏎️bench/🧪️tests/🟦️.ts:738` expectation updated to the 3-stage order.

Verification: no scratch clone built (disk ~94 %, host load ~45, pinned play build running); order proven by the test-level law and the bench test that records executed steps through fake ports.
Tests: see landings.md entry for counts.

## Added scope: fleet state provisioning (coordinator, K5 finding)
- Schema `Policy.const.fleet`: `.cache/f` files `cache-facts.json`, `machine.json`, `identity/` directory, withheld `admin.key` and `origin.key`.
- `incrementalFleetProvisionV1(live,root,signal)` runs in prepare after clone/refresh and before bootstrap (progress stage `fleet`); copies (re-copied on every refresh so facts stay current) with mode 0600; skips absent sources. Copied identity: cache-read, cache-write, submit, worker keys, registration.json, pins-*.json (same machine and user; give the clone the lookup/publish/submit/worker roles). Withheld: `admin.key` (fleet-wide administration; a disposable clone must not hold it), `origin.key` (cache-server origin credential; only the serving host needs it). Re-derivation from the facts server was not chosen: facts are byte-identical on the same machine and a re-fetch needs the network at prepare time.
- Catalogue/help text unchanged (no fixture drift); summary text still accurate.

## Tests (repo test-file, one at a time, taskpolicy -b, host load ~55)
- `🧪️tests/🟦️.ts`: run 1: 47 pass / 4 fail. Fails: 3 real-Bazel tests (130 s timeouts under load, unrelated) and the prepare command test whose expected step log still had the old 4 stages; fixed afterwards (expectation now preflight, dependencies sync, tools setup). New tests (order law, fleet provisioning) passed. Not re-run after the expectation fix (17 min file).
- `🏎️bench/🧪️tests/🟦️.ts`: 48 pass / 1 fail (the 120 s stand-in CLI baseline/compare test timed out under load; unrelated to prepare; the prepare-order test passed).
