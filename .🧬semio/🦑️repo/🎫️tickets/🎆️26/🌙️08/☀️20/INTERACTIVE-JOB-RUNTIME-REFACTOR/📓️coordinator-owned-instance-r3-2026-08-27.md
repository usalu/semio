# Independent Owned Instance R3

The coordinator actually ran the released OwnedInstance cohort: **9 passed, 607 skipped (616 discovered), one passed/four skipped files, 8.61seconds, exit0**, start19:10:26. This covers accepted native input, private retirement/submission tokens, exact host-owner publication, cancellation, failed-ACK retry and final host witness in the authored mock-worker integration.

This is not fresh guest/Wasm execution, live PluginRuntime create/destroy adoption, strong raw returned-output retirement, immutable arbitrary wrapper content, full renderer regression, all-app acceptance or an 8ms timing certificate. The native guest aggregate is still in flight. No runtime budget was raised.

## Command

```sh
NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false bun x nx run @semio-tech/framework-renderer-react:test-long --skip-nx-cache --args='--run -t OwnedInstance'
```

## Captured Source Boundary

Before:

```text
b53492500f9558c268475bbd301d24b71fdf0a5ab4f125aa11abf857a6ec1bbd  🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️component.ts
112d1851f1d8d2595759e9f7ab3bd0776d0475b60eeb658f46d883b60275b6a4  🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧵️shard-client.ts
d3b24322b4cb6de707bf4239f802b9a331005fd8e90824cc0b39d3a754150976  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/UiDocumentStore/🟦️component.tsx
```

After:

```text
b53492500f9558c268475bbd301d24b71fdf0a5ab4f125aa11abf857a6ec1bbd  🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️component.ts
112d1851f1d8d2595759e9f7ab3bd0776d0475b60eeb658f46d883b60275b6a4  🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧵️shard-client.ts
d3b24322b4cb6de707bf4239f802b9a331005fd8e90824cc0b39d3a754150976  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/UiDocumentStore/🟦️component.tsx
```

These captures cover three integration files only, not the complete dependency closure. Concurrent source changes remain owned by their authors.

## Actual Output

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
      Tests  9 passed | 607 skipped (616)
   Start at  19:10:26
   Duration  8.61s (transform 13.95s, setup 0ms, import 20.15s, tests 358ms, environment 3.83s)




 NX   Successfully ran target test-long for project @semio-tech/framework-renderer-react



```

