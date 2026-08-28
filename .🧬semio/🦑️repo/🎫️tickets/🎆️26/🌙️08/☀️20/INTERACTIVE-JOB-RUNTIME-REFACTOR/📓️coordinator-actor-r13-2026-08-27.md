# Independent Actor R13

Actual95/95tests in six files passed,2.19seconds,start19:47:47,exit0. This includes the actual private ShardClient pending-settlement capture and graft-failure tests. Public lifecycle callers still do not supply the new cells; no live strong-output or final-retirement proof is claimed.

Captured hashes were stable before/after. No generated publication, cleanup or native compiler was run by this coordinator check.

## Command

```sh
NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false bun x nx run @semio-tech/framework-actor:test --skip-nx-cache
```

## Hashes Before

```text
922af44c7d06c952e11f2def377359fdeffc0d71afe14d19945a4878e3dd4f36  🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🩹️patch/🟦️component.ts
e159f2fbd11aa31cc0d49557d82351937ce7c093fc5641a108ae4631b50f0a32  🧰️framework/🔨️modules/🎭️actor/🪪️activation/🚪️instance/📥️output/🟦️component.ts
8bf374dbcd0bf29822d8b919c29be0f1c761191fe2aee5326de9aaac4bb6051c  🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧵️shard-client.ts
```

## Hashes After

```text
922af44c7d06c952e11f2def377359fdeffc0d71afe14d19945a4878e3dd4f36  🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🩹️patch/🟦️component.ts
e159f2fbd11aa31cc0d49557d82351937ce7c093fc5641a108ae4631b50f0a32  🧰️framework/🔨️modules/🎭️actor/🪪️activation/🚪️instance/📥️output/🟦️component.ts
8bf374dbcd0bf29822d8b919c29be0f1c761191fe2aee5326de9aaac4bb6051c  🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧵️shard-client.ts
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


 Test Files  6 passed (6)
      Tests  95 passed (95)
   Start at  19:47:47
   Duration  2.19s (transform 3.33s, setup 0ms, import 3.08s, tests 2.38s, environment 1ms)




 NX   Successfully ran target test for project @semio-tech/framework-actor



```

