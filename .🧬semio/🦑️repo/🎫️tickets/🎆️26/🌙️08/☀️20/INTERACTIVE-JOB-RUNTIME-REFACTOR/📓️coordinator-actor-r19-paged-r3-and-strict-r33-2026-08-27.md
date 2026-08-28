# Actor R19, Paged R3 And Strict R33

## Actual Independent Execution

Root ran the actual registered Bun/Nx targets, independently of their authors:

- Actor R19: **131 passed / 0 failed**, nine files, **4.36 s**, start **22:54:17**, exit 0.
- OwnedPaged R3: **15 passed / 643 skipped / 658 collected**, five collected files, **11.46 s**, start **22:54:19**, exit 0.
- Renderer strict R33: **seven existing tutorial diagnostics**, exit 1. No actor framing, input, paged builder, reader or resident diagnostics were reported. Strict ran after the bounded test holds were released; it is not an atomic source snapshot.

All **31 selected** TS/config/JSON inputs matched before and after both tests. The selection now includes the no-copy framing declaration and all three renamed response-credit files. This is not a complete transitive dependency manifest. All source holds were released immediately after the test results and hash comparison.

## Reviewed Scope

Root read the actual ActorReturnResultFraming implementation and both new tests. It consumes one byte per push and validates the same canonical result grammar, exact scalars, enums, control consistency and padding. A page projection exposes only its receipt and raw payload offset, not a page allocation. The original whole decoder consumes this parser and still explicitly materializes a page for its whole-value caller. Constructor instrumentation and independent Buffer/LEB128 vectors verify the tested no-payload-allocation property; they do not admit parser metadata, establish private source custody, or measure callback time.

Root read the UI reader constructor and separate source-observation phase. The actual reader is installed in the private parent before throwing finalization can occur. A full-grant child is forwarded before a separate 128-byte observation; no completion getter is called in that child turn. The first-fragment transition bound is derived from copy, source and observation phases, not an increased runtime quota.

The credit packet remains a strict-schema/Immer declaration oracle. The neutral shared ledger, registered raw inbox, one-response worker credit, multi-page continuation, native InputAck, live PluginRuntime mount and all-app timing remain unfinished. Seven tutorial errors are genuine pending live capture/restore joins, not optional fields to silence.

## Commands

```text
NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false bun x nx run @semio-tech/framework-actor:test --skip-nx-cache
NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false bun x nx run @semio-tech/framework-renderer-react:test-long --skip-nx-cache --args='--run -t OwnedPaged'
NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false bun x nx run @semio-tech/framework-renderer-react:typecheck --skip-nx-cache
```

## Stable Selected Inputs

```text
cccb14e2851cd4a3ba2a83e4b176db256dbe16558bc67a1d5389c46427043788  🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️component.ts
d66b7d681ad0c502a203164bd66c0a755248b1b1793bb123efdb460624ac4113  🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/🟦️component.ts
79d2138edad0e841c8c3131a4994d4cda6ceff0669e4c3dd8e456d57ab4908ac  🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/🟦️component.ts
74a80fc305c8c326c51d49d6b1b6f8e5ac4604690bd5ffbd69bc412b4a0568f0  🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️component.ts
fc97151172f65910fad90a4956b907ec4e217e9ac8e0ef6ae052cae680f2b289  🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧵️shard-client.ts
87c7f25b1aed9bbc15bc3916d837bdd518140bec7e93bd04ba3eac1831edd59f  🧰️framework/🔨️modules/🎭️actor/📤️return/🟦️component.ts
c2db1037203c711da2d3af2e7ae600677eb6864de35f05fb0b3f533281124508  🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧪️vitest.config.ts
115d3ca312e424de2ae3fd6c8573f37e5baf056500c279a1d662bd01ed6f68e4  🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/📬️mailbox.ts
a5b6b7d300351971f3c0fa505f62131b4d6931bfbf4ee876c6eb93a1c4cd9097  🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧵️turn-scheduler.ts
cbabad50e7bde94f9734c859cca3e4abe2d945ce86838f897ae91153a527143c  🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/🟦️component.ts
bf625789eb30990459b30228a191cd40dc5e6ee7c8107ba36daa4b5ba6c653b2  🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🟦️component.ts
fe7438c8b67eeb754b14690629605b738816de3a12f3a111e506c3fe41a3145f  🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🧪️fixture.json
93a881938713902df06067b0a23bcdc0526ba3159943b68dcfa51f2b2b342aa5  🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🧪️schema.json
9467594857a08211423647e0afafee6a614e9f0c65eb629710a72212542d43cc  🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🪪️authority/🧪️fixture.json
2b571515757393196cbbc1236c0712609e57339ec8611430b4467761cfd23ba3  🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🪪️authority/🧪️schema.json
806485f5d6e5689aa026b52fbacec759cdd4dc29c656cca08fc209a4b107fded  🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📄️pages/🟦️component.ts
b2cd39dd47650f839df37e87f4863f1aef8954cdef9d81a6a4f5bf43c007ae0e  🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📄️pages/🧬️schema.json
119a0ef8c0a75c30fc8231853117b9b2053ee53100e27291764182b1eb2b3110  🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📄️pages/🧪️fixture.json
bdb108dd406649380511b76f8297148050dbac1b07eb99fb852183bcb6811e7e  🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📄️pages/🧪️schema.json
ed2c9f97b5abdb39963969d13684e68701ea705d03adc1f7823ac0c25c3aa1e7  🧰️framework/🔨️modules/🎭️actor/📤️return/🧪️fixture.json
3328697d8ed6e7e8c3d939c5213ea276d7075aaa029b75e620258869ade72fff  🧰️framework/🔨️modules/🎭️actor/📤️return/🧪️schema.json
06aa8d36e8643c11dbe65e9a89eae0e48d44b450a5d3e19b2041345f6788f515  🧰️framework/🔨️modules/🎭️actor/📄️page/🟦️component.ts
2ef9e04b7ab886af88e45e72874afc381f24b5a5cc129fa5a60f46335bdc858d  🧰️framework/🔨️modules/🎭️actor/📤️return/📨️response/🟦️component.ts
f953fbb3161e3957f6115b74ba2b45640a3823d20c1af40aec2c1c0cd28ce046  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/UiDocumentStore/🟦️component.tsx
a52a1665039f01d8872c69a18f237215284c2b59c6ce621622771f5b81f69898  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/PluginRuntime/🟦️component.tsx
e0a3b40a20620d710fdb5723a80e1f99e52ed2675b2b2b7e05eddf2df7268c59  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️vitest.config.ts
5a3a0d87b73257932d0b1e73f1d5cf1ad0144e6d869f3eddae614e480290fc24  🧰️framework/🔨️modules/🎭️actor/📤️return/📄️framing/🧬️schema.json
ab7e908a44fa04375d16b9a5163d62980c6e7166a04601c99c0a44adf42ed5d5  🧰️framework/🔨️modules/🎭️actor/📤️return/📄️framing/🧪️fixture.json
e903e20204e23ed4194c55716cbb5579559125929f7f76eb12da3a86a6fbb12b  🧰️framework/🔨️modules/🎭️actor/📤️return/📨️response/🎟️credit/🧬️schema.json
8dbcc1b3b5c253dbba8202c5cebec40fe1c9fbce4b169459be1e0e739c1c4cf3  🧰️framework/🔨️modules/🎭️actor/📤️return/📨️response/🎟️credit/🧪️fixture.json
5871b9274d565183a96f05598a7e1ca4569d1601ed7758730ad4def05dd9bf0a  🧰️framework/🔨️modules/🎭️actor/📤️return/📨️response/🎟️credit/🧪️schema.json
```

## Actual Actor Output

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

(node:59405) ExperimentalWarning: Transform Types is an experimental feature and might change at any time
(Use `node --trace-warnings ...` to show where the warning was created)

 Test Files  9 passed (9)
      Tests  131 passed (131)
   Start at  22:54:17
   Duration  4.36s (transform 6.79s, setup 0ms, import 4.35s, tests 12.06s, environment 2ms)




 NX   Successfully ran target test for project @semio-tech/framework-actor


```

## Actual OwnedPaged Output

```text

> nx run @semio-tech/framework-renderer-react:test-long --args=--run -t OwnedPaged

> bun ./📜️script.ts test long --run -t OwnedPaged

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
      Tests  15 passed | 643 skipped (658)
   Start at  22:54:19
   Duration  11.46s (transform 9.87s, setup 0ms, import 14.22s, tests 7.47s, environment 5.90s)




 NX   Successfully ran target test-long for project @semio-tech/framework-renderer-react


```

## Actual Strict Output

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

