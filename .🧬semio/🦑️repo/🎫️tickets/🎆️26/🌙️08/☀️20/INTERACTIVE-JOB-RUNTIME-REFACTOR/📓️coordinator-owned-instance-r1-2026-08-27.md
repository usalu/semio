# Independent Owned Instance R1 — 2026-08-27

## Result and Scope

The canonical focused React target actually passed **5 tests, 603 skipped, 608 total; one test file passed, four skipped; exit 0**. It started at 18:43:10 and reported 6.34 seconds. The five tests exercise exact activation/guest lifetime identity, stale patch facade refusal, privately minted acknowledgement refusal, terminal host-retirement witness ownership, and bounded/cancelled surface lookup. This is not the later native-source/ACK submission integration, a full React pass, fresh guest execution, or a timing certificate.

The two scoped source hashes matched immediately before and after this run. The executor's sixth native-source integration test was subsequently staged under TDD; this successful five-test run is not evidence for that later source boundary. Nx emitted its historical flaky-task advisory; this execution itself exited 0 and had no failed test.

## Command

```sh
NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false bun x nx run @semio-tech/framework-renderer-react:test-long --skip-nx-cache --args='--run -t OwnedInstance'
```

## Source Snapshot

Before:

```text
2fb231ed1c1f9b75751bc6096fb8e20cddb9a5be65ba18ccf9bdb8a9dc73e992  🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️component.ts
e012cf4769744bdab43a1451a1311170fff156ac89c5b6bfa48ce0207dbbea71  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/UiDocumentStore/🟦️component.tsx
```

After:

```text
2fb231ed1c1f9b75751bc6096fb8e20cddb9a5be65ba18ccf9bdb8a9dc73e992  🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️component.ts
e012cf4769744bdab43a1451a1311170fff156ac89c5b6bfa48ce0207dbbea71  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/UiDocumentStore/🟦️component.tsx
```

## Captured Actual Output

```text

> nx run @semio-tech/framework-renderer-react:test-long --args=--run -t OwnedInstance

> bun ./📜️script.ts test long --run -t OwnedInstance

Warning: The 'NO_COLOR' env is ignored due to the 'FORCE_COLOR' env being set.
      at warnOnDeactivatedColors (internal:tty:33:24)
      at getColorDepth (internal:tty:42:39)
      at shouldColorize (internal:util/colors:14:109)
      at refresh (internal:util/colors:18:31)
      at internal:util/colors (internal:util/colors:24:16)
      at internal:assert/assertion_error (internal:assert/assertion_error:2:187)
      at loadAssertionError (node:assert:28:96)


 RUN  v4.1.10 /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react

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


 Test Files  1 passed | 4 skipped (5)
      Tests  5 passed | 603 skipped (608)
   Start at  18:43:10
   Duration  6.34s (transform 11.61s, setup 0ms, import 15.54s, tests 257ms, environment 3.95s)




 NX   Successfully ran target test-long for project @semio-tech/framework-renderer-react



 NX   Nx detected a flaky task

  @semio-tech/framework-renderer-react:test-long

Flaky tasks can disrupt your CI pipeline. Automatically retry them with Nx Cloud. Learn more at https://nx.dev/ci/features/flaky-tasks


exit_code=0
```

