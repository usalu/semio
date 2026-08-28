# Independent Generated Issued-Patch Receipt R1

Actual result: one passed,60skipped,61discovered; one file,2.52seconds,start19:42:41,exit0. The three captured producer/router/fixture hashes were stable before and after.

The selected test executes the actual generated bridge in Node VM modules with a controlled component namespace, all four shared receipt vectors, exact ACK/Rejected forwarding and cardinality/refusal cases. It is not a fresh Wasm/component, browser, live app or raw-output-retirement proof. The full61test suite was not rerun by the coordinator. This selected fixture does not launch a browser or execute unrelated temporary-directory cleanup cases.

```sh
NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false bun x nx run @semio-tech/framework-os-dev:test-long --skip-nx-cache --args='--run -t issued'
```

## Hashes Before

```text
a246d95516306aa6fdbfb32bcaf8bdf825c685bc20f12eeb09eaa7af5b4c1d5c  🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🌐plugin-web-materialize.ts
9cf27bc33e650ab84ad1d803e2a6ffd0b1d46da98b5077bcff794a9033d1282c  🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts
f2e7e60073b49031a55dfc06e4a8e57401bdd30c955bb358563d85060af0a1ec  🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🩹️patch/🧪️fixture.json
```

## Hashes After

```text
a246d95516306aa6fdbfb32bcaf8bdf825c685bc20f12eeb09eaa7af5b4c1d5c  🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🌐plugin-web-materialize.ts
9cf27bc33e650ab84ad1d803e2a6ffd0b1d46da98b5077bcff794a9033d1282c  🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts
f2e7e60073b49031a55dfc06e4a8e57401bdd30c955bb358563d85060af0a1ec  🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🩹️patch/🧪️fixture.json
```

## Actual Output

```text

> nx run @semio-tech/framework-os-dev:test-long --args=--run -t issued

> bun ./📜️script.ts test long --run -t issued

Warning: The 'NO_COLOR' env is ignored due to the 'FORCE_COLOR' env being set.
      at warnOnDeactivatedColors (internal:tty:33:24)
      at getColorDepth (internal:tty:42:39)
      at shouldColorize (internal:util/colors:14:109)
      at refresh (internal:util/colors:18:31)
      at internal:util/colors (internal:util/colors:24:16)
      at internal:assert/assertion_error (internal:assert/assertion_error:2:187)
      at loadAssertionError (node:assert:28:96)


 RUN  v4.1.10 /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript

Warning: The 'NO_COLOR' env is ignored due to the 'FORCE_COLOR' env being set.
      at warnOnDeactivatedColors (internal:tty:33:24)
      at getColorDepth (internal:tty:42:39)
      at shouldColorize (internal:util/colors:14:109)
      at refresh (internal:util/colors:18:31)
      at internal:util/colors (internal:util/colors:24:16)
      at internal:assert/assertion_error (internal:assert/assertion_error:2:187)
      at loadAssertionError (node:assert:28:96)

(node:83256) ExperimentalWarning: VM Modules is an experimental feature and might change at any time
(Use `node --trace-warnings ...` to show where the warning was created)

 Test Files  1 passed (1)
      Tests  1 passed | 60 skipped (61)
   Start at  19:42:41
   Duration  2.52s (transform 1.49s, setup 0ms, import 1.78s, tests 97ms, environment 527ms)




 NX   Successfully ran target test-long for project @semio-tech/framework-os-dev



```

