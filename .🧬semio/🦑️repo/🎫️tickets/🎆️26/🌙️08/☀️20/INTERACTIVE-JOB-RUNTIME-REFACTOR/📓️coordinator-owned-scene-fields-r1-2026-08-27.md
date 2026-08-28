# Independent Owned Scene Fields R1

Actual6passed,621skipped,627discovered across five files (one passed/four skipped);21.66seconds,start19:53:14,exit0. Four captured source hashes were stable before/after.

This selected cohort covers raw owned scenes, retained JSON syntax including private prepared-source admission, and retained base64 page output. It is not a generic-pack parser, typed-host/live renderer cutover, full627test pass, fresh Wasm or timing certificate. New generic-pack TDD was allowed only after this captured run completed. No runtime or test budget was increased.

## Command

```sh
NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false bun x nx run @semio-tech/framework-renderer-react:test-long --skip-nx-cache --args='--run -t OwnedScene'
```

## Hashes Before

```text
8365737e4967937495b9e17e21368f88720a3194a76c03f9bd73408422b16b01  🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🧾️typed/🟦️component.ts
0ff0e4666b2bc332cf60b2a032309f2917208d9e3578813f8b4fce8f083b2f4d  🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🧾️typed/🧩️json/🟦️component.ts
70bcd599d1901bda09eae98df9159089e8b2362721d85e10ae8a72f67b1e734b  🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🧾️typed/🧩️pack/🔤️base64/🟦️component.ts
5c543296a674294c5eb26fc762d3ef79037092e27f1ec9264e1144ae085f0c71  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/UiDocumentStore/🟦️component.tsx
```

## Hashes After

```text
8365737e4967937495b9e17e21368f88720a3194a76c03f9bd73408422b16b01  🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🧾️typed/🟦️component.ts
0ff0e4666b2bc332cf60b2a032309f2917208d9e3578813f8b4fce8f083b2f4d  🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🧾️typed/🧩️json/🟦️component.ts
70bcd599d1901bda09eae98df9159089e8b2362721d85e10ae8a72f67b1e734b  🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🧾️typed/🧩️pack/🔤️base64/🟦️component.ts
5c543296a674294c5eb26fc762d3ef79037092e27f1ec9264e1144ae085f0c71  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/UiDocumentStore/🟦️component.tsx
```

## Actual Output

```text

> nx run @semio-tech/framework-renderer-react:test-long --args=--run -t OwnedScene

> bun ./📜️script.ts test long --run -t OwnedScene

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
      Tests  6 passed | 621 skipped (627)
   Start at  19:53:14
   Duration  21.66s (transform 8.28s, setup 0ms, import 12.12s, tests 19.33s, environment 2.78s)




 NX   Successfully ran target test-long for project @semio-tech/framework-renderer-react



```

