# Independent Owned Instance R2 — 2026-08-27

## Result

Canonical focused React test-long executed **7 passed, 603 skipped, 610 total; one file passed, four skipped; exit 0**. Start18:48:57, duration7.81seconds. This exercises actual host-side ShardClient/owned UI publication and ACK integration against mock workers, including post refusal/fault response/retry after operation revocation, exact bound-host rejection and final private witness retry. It is not fresh guest/Wasm execution, the later input-retirement token/container cleanup, a live React/WGPU mount, or a timing certificate.

The instance implementation and ShardClient hashes matched before/after. The test module changed while the subsequent input-token test packet was being staged; the recorded actual run discovered seven matching tests. No claim that the later eight-test source is covered.

## Command

```sh
NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false bun x nx run @semio-tech/framework-renderer-react:test-long --skip-nx-cache --args='--run -t OwnedInstance'
```

## Before Snapshot

```text
0dff7d2ca5f80d8365b9bdedab98d1258aa577421666052ccfe63647976fca59  🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️component.ts
125b66b46dd01703e9369dcdb61b6a8d4eb7056c2b9e63dfe5f155f06a58e014  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/UiDocumentStore/🟦️component.tsx
7210ddad5a9f7d1b8309e1eba507eba598185437978b6e2c0ca62bc0199c317b  🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧵️shard-client.ts
```

## After Snapshot

```text
0dff7d2ca5f80d8365b9bdedab98d1258aa577421666052ccfe63647976fca59  🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️component.ts
af43a3c2cdf92aba51b42c6fec151c8d07b75b8cadd963160694a9151fc03b40  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/UiDocumentStore/🟦️component.tsx
7210ddad5a9f7d1b8309e1eba507eba598185437978b6e2c0ca62bc0199c317b  🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧵️shard-client.ts
```

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
      Tests  7 passed | 603 skipped (610)
   Start at  18:48:57
   Duration  7.81s (transform 12.85s, setup 0ms, import 18.08s, tests 199ms, environment 3.04s)




 NX   Successfully ran target test-long for project @semio-tech/framework-renderer-react



exit_code=0
```

