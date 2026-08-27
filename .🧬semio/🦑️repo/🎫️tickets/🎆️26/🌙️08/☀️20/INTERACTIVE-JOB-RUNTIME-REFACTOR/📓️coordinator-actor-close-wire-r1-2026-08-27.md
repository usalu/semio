# Independent Actor Lifetime Close Wire Check

Command: `NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false bun x nx run @semio-tech/framework-actor:test --skip-nx-cache --args='-t=actor.instance.close'`

Exit code: 0. Two tests passed, 54 skipped, 56 discovered; 495 ms test runtime / 1.46 seconds total, start 13:25:45. The coordinator read the complete codec, both tests and strict neutral fixture. Existing Ajv and an independent LEB128 encoder validate fixed numeric authority, canonical/truncated/overflow rejection and generation-bound receipt matching. This does not execute the native joined close producer, worker-route lease or host cleanup; the fixture's completion participants are obligations, not proof of their implementation. Nx also emitted a flaky-task notice; no claim of flake-free execution is made.

```text

> nx run @semio-tech/framework-actor:test --args=-t=actor.instance.close

> bun ./📜️script.ts test -t=actor.instance.close

Warning: The 'NO_COLOR' env is ignored due to the 'FORCE_COLOR' env being set.
      at warnOnDeactivatedColors (internal:tty:33:24)
      at getColorDepth (internal:tty:42:39)
      at shouldColorize (internal:util/colors:14:109)
      at refresh (internal:util/colors:18:31)
      at internal:util/colors (internal:util/colors:24:16)
      at internal:assert/assertion_error (internal:assert/assertion_error:2:187)
      at loadAssertionError (node:assert:28:96)


 RUN  v4.1.10 /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript

Warning: The 'NO_COLOR' env is ignored due to the 'FORCE_COLOR' env being set.
      at warnOnDeactivatedColors (internal:tty:33:24)
      at getColorDepth (internal:tty:42:39)
      at shouldColorize (internal:util/colors:14:109)
      at refresh (internal:util/colors:18:31)
      at internal:util/colors (internal:util/colors:24:16)
      at internal:assert/assertion_error (internal:assert/assertion_error:2:187)
      at loadAssertionError (node:assert:28:96)

Warning: The 'NO_COLOR' env is ignored due to the 'FORCE_COLOR' env being set.
      at warnOnDeactivatedColors (internal:tty:33:24)
      at getColorDepth (internal:tty:42:39)
      at shouldColorize (internal:util/colors:14:109)
      at refresh (internal:util/colors:18:31)
      at internal:util/colors (internal:util/colors:24:16)
      at internal:assert/assertion_error (internal:assert/assertion_error:2:187)
      at loadAssertionError (node:assert:28:96)

Warning: The 'NO_COLOR' env is ignored due to the 'FORCE_COLOR' env being set.
      at warnOnDeactivatedColors (internal:tty:33:24)
      at getColorDepth (internal:tty:42:39)
      at shouldColorize (internal:util/colors:14:109)
      at refresh (internal:util/colors:18:31)
      at internal:util/colors (internal:util/colors:24:16)
      at internal:assert/assertion_error (internal:assert/assertion_error:2:187)
      at loadAssertionError (node:assert:28:96)

Warning: The 'NO_COLOR' env is ignored due to the 'FORCE_COLOR' env being set.
      at warnOnDeactivatedColors (internal:tty:33:24)
      at getColorDepth (internal:tty:42:39)
      at shouldColorize (internal:util/colors:14:109)
      at refresh (internal:util/colors:18:31)
      at internal:util/colors (internal:util/colors:24:16)
      at internal:assert/assertion_error (internal:assert/assertion_error:2:187)
      at loadAssertionError (node:assert:28:96)


 Test Files  1 passed | 3 skipped (4)
      Tests  2 passed | 54 skipped (56)
   Start at  13:25:45
   Duration  1.46s (transform 1.34s, setup 0ms, import 2.32s, tests 495ms, environment 1ms)




 NX   Successfully ran target test for project @semio-tech/framework-actor



 NX   Nx detected a flaky task

  @semio-tech/framework-actor:test

Flaky tasks can disrupt your CI pipeline. Automatically retry them with Nx Cloud. Learn more at https://nx.dev/ci/features/flaky-tasks


```

