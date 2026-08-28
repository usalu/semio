# Independent Actor R11 — New Response-Envelope RED

The canonical actor target actually executed92passing tests and one failing test,93total in six files,4.42seconds; exit1, start19:34:35. The only failure is the newly authored isolated `OwnedActorTurnOutput` response-envelope law: `output.captureResponse` was not implemented in the compiled snapshot. This is the peer's new test-first foundation boundary, not a failure of the earlier released92-test checkpoint. A later source read shows an implementation; this run does not verify that later source.

The original output foundation remains unmounted and raw descendant retirement remains open. No fresh component, live lifecycle, immutable-content or terminal retirement claim follows from the92passing tests. Nx's generic flaky-task label does not establish nondeterminism in a changing shared source tree.

## Command

```sh
NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false bun x nx run @semio-tech/framework-actor:test --skip-nx-cache
```

## Source Hashes Before

```text
922af44c7d06c952e11f2def377359fdeffc0d71afe14d19945a4878e3dd4f36  🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🩹️patch/🟦️component.ts
ca6a1b42339372e44c6ac6435ecb86e4e87012182b2304d27d24fc371e910fd6  🧰️framework/🔨️modules/🎭️actor/🪪️activation/🚪️instance/📥️output/🟦️component.ts
1c0bdd0e992198a46c11696ce57ab10454c0822c2f734b7b68bab64119f21fec  🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧵️shard-client.ts
```

## Source Hashes After

```text
922af44c7d06c952e11f2def377359fdeffc0d71afe14d19945a4878e3dd4f36  🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🩹️patch/🟦️component.ts
e159f2fbd11aa31cc0d49557d82351937ce7c093fc5641a108ae4631b50f0a32  🧰️framework/🔨️modules/🎭️actor/🪪️activation/🚪️instance/📥️output/🟦️component.ts
1c0bdd0e992198a46c11696ce57ab10454c0822c2f734b7b68bab64119f21fec  🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧵️shard-client.ts
```

## Actual Output

```text

> nx run @semio-tech/framework-actor:test

> bun ./📜️script.ts test

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

 ❯ |@semio-tech/framework-actor| ../../🪪️activation/🚪️instance/📥️output/🟦️component.ts (4 tests | 1 failed) 418ms
     × captures the original response envelope before settlement or failure extraction can throw 92ms

⎯⎯⎯⎯⎯⎯⎯ Failed Tests 1 ⎯⎯⎯⎯⎯⎯⎯

 FAIL  |@semio-tech/framework-actor| ../../🪪️activation/🚪️instance/📥️output/🟦️component.ts > OwnedActorTurnOutput > captures the original response envelope before settlement or failure extraction can throw
TypeError: output.captureResponse is not a function. (In 'output.captureResponse(raw)', 'output.captureResponse' is undefined)
 ❯ ../../🪪️activation/🚪️instance/📥️output/ð¦ï¸component.ts:143:25
 ❯ run ../../🪪️activation/🚪️instance/📥️output/ð¦ï¸component.ts:26:27
 ❯ ../../🪪️activation/🚪️instance/📥️output/ð¦ï¸component.ts:142:32

⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯[1/1]⎯


 Test Files  1 failed | 5 passed (6)
      Tests  1 failed | 92 passed (93)
   Start at  19:34:35
   Duration  4.42s (transform 3.46s, setup 0ms, import 2.12s, tests 5.18s, environment 2ms)




 NX   Running target test for project @semio-tech/framework-actor failed

Failed tasks:

- @semio-tech/framework-actor:test

Hint: run the command with --verbose for more details.

Warning: command "bun ./📜️script.ts test" exited with non-zero status code
 NX   Nx detected a flaky task

  @semio-tech/framework-actor:test

Flaky tasks can disrupt your CI pipeline. Automatically retry them with Nx Cloud. Learn more at https://nx.dev/ci/features/flaky-tasks


```

