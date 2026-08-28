# Independent Strict TypeScript R30

Actual exit1 with exactly seven known tutorial/local-interaction diagnostics. The two discovery diagnostics and UI nullable fixture/peer push-to-never fixture errors are absent from this captured output. No diagnostic was suppressed; tutorial producer/restore joins remain assigned.

All six captured integration source hashes were identical before and after this run. Later source changes need their own verification. No browser action, native compilation, generated-output publication, cleanup or modifying Git operation was performed by this gate.

## Command

```sh
NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false bun x nx run @semio-tech/framework-renderer-react:typecheck --skip-nx-cache
```

## Hashes Before

```text
fe7d70b2ef02d26a8204d61d6800c0a2078423fdfc5a3df0ac811ccd15cf869a  🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️component.ts
9bc9bf97f18f69721c640845bb5c10e33571a124fde5031e884a672248f8d35d  🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧵️shard-client.ts
1c641b7ea57b1e685cff8051448575b6f2b51f1afa5a3b30dc10d46ca2c2e37f  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/UiDocumentStore/🟦️component.tsx
da8313927ce2c79598ec93a6026a73b30c0167eb32e8be337cd9ae99d282704b  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/UiDocumentStore/📥️intake/🟦️component.ts
a52a1665039f01d8872c69a18f237215284c2b59c6ce621622771f5b81f69898  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/PluginRuntime/🟦️component.tsx
a546a325857a156fded0044c02ecd1c70df83a428c63a1b4a8f6aad361f1b0ba  🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🧾️typed/🧩️json/🟦️component.ts
```

## Hashes After

```text
fe7d70b2ef02d26a8204d61d6800c0a2078423fdfc5a3df0ac811ccd15cf869a  🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️component.ts
9bc9bf97f18f69721c640845bb5c10e33571a124fde5031e884a672248f8d35d  🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧵️shard-client.ts
1c641b7ea57b1e685cff8051448575b6f2b51f1afa5a3b30dc10d46ca2c2e37f  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/UiDocumentStore/🟦️component.tsx
da8313927ce2c79598ec93a6026a73b30c0167eb32e8be337cd9ae99d282704b  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/UiDocumentStore/📥️intake/🟦️component.ts
a52a1665039f01d8872c69a18f237215284c2b59c6ce621622771f5b81f69898  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/PluginRuntime/🟦️component.tsx
a546a325857a156fded0044c02ecd1c70df83a428c63a1b4a8f6aad361f1b0ba  🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🧾️typed/🧩️json/🟦️component.ts
```

## Actual Output

```text

> nx run @semio-tech/framework-renderer-react:typecheck

> bun ./📜️script.ts typecheck

../../../../../../../../../../♻️mit-bestand/🧺️demonstrator/🟦️brand.ts(151,5): error TS2741: Property 'interactionSelection' is missing in type '{ focusedWindowId: string; activeUtilityByWindowId: {}; activePanelTabByGroup: {}; expandedTreeIds: never[]; commandPanelOpen: false; }' but required in type 'TutorialUiSnapshot'.
🧪️index.test.ts(6922,43): error TS2345: Argument of type '{ activeUtilityByWindowId: {}; activePanelTabByGroup: {}; expandedTreeIds: never[]; commandPanelOpen: false; }' is not assignable to parameter of type 'TutorialUiSnapshot'.
  Property 'interactionSelection' is missing in type '{ activeUtilityByWindowId: {}; activePanelTabByGroup: {}; expandedTreeIds: never[]; commandPanelOpen: false; }' but required in type 'TutorialUiSnapshot'.
../../../../🧱️elements/ShellHelpers/🟦️component.tsx(2102,5): error TS2353: Object literal may only specify known properties, and 'selectionJson' does not exist in type 'TutorialUiSnapshot'.
../../../../🧱️elements/ShellHelpers/🟦️component.tsx(2102,39): error TS2339: Property 'selectionJson' does not exist on type 'PluginViewState'.
../../../../🧱️elements/ShellHelpers/🟦️component.tsx(2153,41): error TS2339: Property 'selectionJson' does not exist on type 'TutorialUiSnapshot'.
../../../../🧱️elements/ShellHelpers/🟦️component.tsx(2153,76): error TS2339: Property 'selectionJson' does not exist on type 'PluginViewState'.
../../../../🧱️elements/ShellHelpers/🟦️component.tsx(2197,141): error TS2339: Property 'selectionJson' does not exist on type '{ readonly kind: "selection"; readonly domainId: string; readonly granularity: string; readonly ids: readonly string[]; }'.
Warning: command "bun ./📜️script.ts typecheck" exited with non-zero status code


 NX   Running target typecheck for project @semio-tech/framework-renderer-react failed

Failed tasks:

- @semio-tech/framework-renderer-react:typecheck

Hint: run the command with --verbose for more details.


```

