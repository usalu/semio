# Coordinator Full React R22 — 2026-08-27

Actual full React renderer **628/628 passed**, five files, exit 0, start 20:15:36, duration 64.94 s. Seven captured UI/PluginRuntime files were unchanged, including generic pack, base64, JSON and UiDocumentStore. **ShardClient changed during the captured window** from ffba2728842427fa5de05c35aa0c30b6efbca430642be12aba170b042fd198e7 to b36b197b27a69fe9b644233a2473734d49588c85ba513c16bb1de0d207949b7d. This is an actual full-suite pass, not a stable current-ShardClient certificate; the peer was asked to attribute that delta without reverting source. The UI source hold is released. Strict R31 still has seven tutorial joins. No fresh Wasm, live all-app or timing acceptance is inferred.

## Command

Subsequent peer attribution: the ShardClient delta was a public re-export of the two return-drive codec functions and five canonical return types, not a dispatch/lifecycle/scheduler edit. Coordinator source inspection confirms that export at line 26. This explanation does not retroactively make the captured hashes equal or establish when the importer observed the new module. The peer's later strict run found one test-only boolean-cast diagnostic in the newly exported codec, which is peer-owned; R31's actual seven diagnostics remain the result of its exact run.

```sh
NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false bun x nx run @semio-tech/framework-renderer-react:test-long --skip-nx-cache
```

## Source SHA-256 Before

```text
fe7d70b2ef02d26a8204d61d6800c0a2078423fdfc5a3df0ac811ccd15cf869a  🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️component.ts
ffba2728842427fa5de05c35aa0c30b6efbca430642be12aba170b042fd198e7  🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧵️shard-client.ts
1ec512d5fe007d66622fcee7ffb52dfd1487e9484d8c01ad3fb2a27e29919759  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/UiDocumentStore/🟦️component.tsx
da8313927ce2c79598ec93a6026a73b30c0167eb32e8be337cd9ae99d282704b  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/UiDocumentStore/📥️intake/🟦️component.ts
a52a1665039f01d8872c69a18f237215284c2b59c6ce621622771f5b81f69898  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/PluginRuntime/🟦️component.tsx
0ff0e4666b2bc332cf60b2a032309f2917208d9e3578813f8b4fce8f083b2f4d  🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🧾️typed/🧩️json/🟦️component.ts
8dafbc4a038f8a98f4cca4e4482001c79bcfdab57e22226a8450cbcfd833aad8  🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🧾️typed/🧩️pack/🧾️value/🟦️component.ts
ed218df0bd0aea5e579ad966da31f250625ffdfab752de48dd301fef89343a14  🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🧾️typed/🧩️pack/🔤️base64/🟦️component.ts
```

## Source SHA-256 After

```text
fe7d70b2ef02d26a8204d61d6800c0a2078423fdfc5a3df0ac811ccd15cf869a  🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️component.ts
b36b197b27a69fe9b644233a2473734d49588c85ba513c16bb1de0d207949b7d  🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧵️shard-client.ts
1ec512d5fe007d66622fcee7ffb52dfd1487e9484d8c01ad3fb2a27e29919759  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/UiDocumentStore/🟦️component.tsx
da8313927ce2c79598ec93a6026a73b30c0167eb32e8be337cd9ae99d282704b  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/UiDocumentStore/📥️intake/🟦️component.ts
a52a1665039f01d8872c69a18f237215284c2b59c6ce621622771f5b81f69898  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/PluginRuntime/🟦️component.tsx
0ff0e4666b2bc332cf60b2a032309f2917208d9e3578813f8b4fce8f083b2f4d  🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🧾️typed/🧩️json/🟦️component.ts
8dafbc4a038f8a98f4cca4e4482001c79bcfdab57e22226a8450cbcfd833aad8  🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🧾️typed/🧩️pack/🧾️value/🟦️component.ts
ed218df0bd0aea5e579ad966da31f250625ffdfab752de48dd301fef89343a14  🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🧾️typed/🧩️pack/🔤️base64/🟦️component.ts
```

## Actual Output

```text
> nx run @semio-tech/framework-renderer-react:test-long

> bun ./📜️script.ts test long

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


 Test Files  5 passed (5)
      Tests  628 passed (628)
   Start at  20:15:36
   Duration  64.94s (transform 13.20s, setup 0ms, import 17.23s, tests 65.48s, environment 1.89s)




 NX   Successfully ran target test-long for project @semio-tech/framework-renderer-react
```
