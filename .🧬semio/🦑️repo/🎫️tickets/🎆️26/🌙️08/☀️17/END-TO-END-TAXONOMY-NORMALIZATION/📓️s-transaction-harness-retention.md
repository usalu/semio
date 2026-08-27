# Transaction Harness Evidence Retention

## Safety Correction

Inspection before the next aggregate run found that both the Nx launcher and the transaction tests deleted their completed ticket-local fixture trees. The current instruction requires those temporary assets and evidence to be retained. The launcher now preserves its fixture trees, and the tests replace whole-root removal with exact ticket-parent and fixture-prefix validation. Shared template markers and their empty preparation directory are retained as well. Intentional mutations inside an isolated transaction fixture, such as replacing its test symlink or clearing its cancellation sentinel, remain part of the tested behavior.

The launcher now passes the already absolute compiled test path to Bun. It no longer derives a relative invocation path. No test title, selection filter, concurrency setting, timeout, golden fixture, or transaction operation was changed. In particular, the complete aggregate watchdog remains fourteen seconds.

## Test-First Evidence

A permanent language-neutral vector specifies the launcher, test owner, absolute-path expression, retained-root authority, and accepted/rejected parent cases. The ticket test uses the TypeScript parser to inspect actual invocation and teardown sites. Bun and TypeScript independently compile and execute the real retention function against all four path cases, preserving every payload.

Before implementation: zero passes, three failures, five assertions, 983 milliseconds. The failures identified the relative launch expression, thirty-six whole-root deletion calls, and the missing exact retention function.

After implementation: three passes, zero failures, thirty-one assertions, 2.53 seconds. This is focused harness verification, not a passing complete aggregate or a performance acceptance claim. Actual uncached Nx execution is recorded separately once run.

Retaining shared templates exposed a necessary identity correction: PID-only run names can recur after process reuse and accidentally admit an old fixture or overwrite an old registry. The launcher and standalone test default now use PID plus a fresh UUID; all shards receive the same per-run identity. PID and boundary registries carry that identity too. A new test first failed on the absent unique identity, then the expanded packet passed four tests with zero failures and thirty-five assertions in 440 milliseconds. The frozen transaction golden remains byte-identical: 403,163 bytes, SHA-256 `7b700d79e5474417f0c92ddce61f5ffdd24603af56241d0fbdc3cdd5ba560296`, 63 transaction ledgers, nine workspace ledgers, and 98 boundaries.

The actual uncached filtered target `bun nx run @semio-tech/repo-lib:test-transaction-v2 --skip-nx-cache --args='incomplete plans'` passed one test with six assertions in 1.50 seconds; its shard finished in 1.53 seconds. After process exit, a separate no-follow check found both the shared template and strict-plan fixture under run identity `51506-20039ab0-f516-4a2d-a073-d049fa743e74`, with their source payloads preserved (SHA-256 `5d8f65d2774e206bc9f7a7a4ad39ca2dc563b5c31e46ab57ef4874961237ce29`). The unique PID and boundary registries also remain. This verifies real post-run retention; the full 62-case aggregate and its timing gate remain separate.

## Scope

Changed production test infrastructure: the repo-library TypeScript `📜️script.ts` and `🧪️tests/🧪️transaction-v2/🟦️.test.ts`. New language-neutral vector: `🧫️fixtures/🧪️transaction-harness-retention/🔣️.json`. The test, execution fixtures, and this report are retained inside this ticket. No actual Compose tree or real Git state was modified.
