# Coordinator Strict React R32 — 2026-08-27

Actual strict React typecheck exited 1 with exactly seven existing tutorial/local-interaction diagnostics. All 33 captured retained UI, collected renderer, configuration, ShardClient and actor-return source hashes were unchanged across this strict check. No owned Surface, resident pool, page, return, or disposal diagnostic was emitted. This is not a strict pass; tutorial snapshot/restore adoption remains required. No default selection or compatibility field is being introduced to suppress the errors.

## Command

```sh
NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false bun x nx run @semio-tech/framework-renderer-react:typecheck --skip-nx-cache
```

## SHA-256 Before

```text
158af310c62ca4d6b620e3ff4bfaae620974c1d24213dd5501fee818bea5b245  🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🖼️surface/🟦️component.ts
800139cfb8c99af38630e1ab8bcc540c322bb1c31a128826149e628ce73cbcc0  🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🗂️nodes/🟦️component.ts
36d995844ea0eb671798213e96c671ebae72e400f22c5cbf5cd4c5b77dfd096c  🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🛡️validation/🟦️component.ts
be54a59db1c254b1a76e1b2e4d78259ba901689a9d59f9f82db33321f8924a99  🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🛡️validation/🔬️graph/🟦️component.ts
71a34fefe4c3393d7f5948306800a1aa3bd5713445d8517af23a303609fa0801  🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🔢️hash/🟦️component.ts
cccb14e2851cd4a3ba2a83e4b176db256dbe16558bc67a1d5389c46427043788  🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️component.ts
13e4936e461c5f836b33320522ced1ecb1b4eb25d27d15eebfe11d1575ce119b  🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/🟦️component.ts
79d2138edad0e841c8c3131a4994d4cda6ceff0669e4c3dd8e456d57ab4908ac  🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/🟦️component.ts
289527fa0b7851cb7785830288858fbd1ca5c3378dc12c61a7f8759ff43f93d5  🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🟦️component.ts
0754743b5802ccc6e956bf6ded8aa7c4fcaf010795e761ba6ad1a9e9bb1bbe00  🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️component.ts
1d237ad89846103f660ed858ccabeddf27b07e9f286c8b91b2819a8b714eaa71  🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🔒️transport/🟦️component.ts
af27ecc7b6c7f6f5bd676edf86be7e2d73ee38aacba4a196e285403555ed98e9  🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️component.ts
0bdf5eca18c5b347e219321da7eed7bbf9477cc6661147167c2cdfa686f01db2  🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🔢️bytes/🟦️component.ts
3ac49d7eb43a5db72acfea50b58709769f753767f0cd3cce92e0621cde51e30d  🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🟦️component.ts
12d4e5a37c8370b1557a7bbddeaaec461cd3da0bbe185a412b73606b5cb2ab0e  🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/📖️read-lease/🟦️component.ts
ed218df0bd0aea5e579ad966da31f250625ffdfab752de48dd301fef89343a14  🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🧾️typed/🧩️pack/🔤️base64/🟦️component.ts
c5ffb34c562e67f6dfa822c3add1eb32170c54cd1b551ab3dcf329142bdfe25b  🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🧾️typed/🧩️pack/🧾️value/🟦️component.ts
8365737e4967937495b9e17e21368f88720a3194a76c03f9bd73408422b16b01  🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🧾️typed/🟦️component.ts
bdb4aabbd573140f85948ea7c0755c058b39e6d96d810963a9b0e5bac6b7f907  🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🧾️typed/🧩️json/🔤️string/🟦️component.ts
0ff0e4666b2bc332cf60b2a032309f2917208d9e3578813f8b4fce8f083b2f4d  🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🧾️typed/🧩️json/🟦️component.ts
d0399f625aa3bb64a0554f2dd8391679d382266490c147e62131e1034ccbfc9c  🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🧾️typed/🧩️json/🧾️value/🟦️component.ts
3dc1020baadf5742106c61ab2b012bbccfcee7f8987b624669341aa5ed6ab892  🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🟦️component.ts
d37b4c23a29108441f2ab5a38bad2e1f420f30f45b9bb3aaa2b81ddfc8c31785  🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🔗️binding/🟦️component.ts
dcaedbe3a13dd9aab85567cfb052651733faa87beb288276ad86150529abfe05  🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🔗️binding/🗂️index/🟦️component.ts
6dca1d11f1cb9441ad43bb481e18f1b6c5afc3bda559f9828811e75f60a08610  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/UiDocumentStore/🟦️component.tsx
da8313927ce2c79598ec93a6026a73b30c0167eb32e8be337cd9ae99d282704b  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/UiDocumentStore/📥️intake/🟦️component.ts
a52a1665039f01d8872c69a18f237215284c2b59c6ce621622771f5b81f69898  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/PluginRuntime/🟦️component.tsx
125c2f26ceca9513ade80face64bda52ea74a78a4e1b29c066b7eca71e785b71  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Interpreter/🟦️component.tsx
326b526b4ac4e858a4122fa7fb8f3c5b15920c71f3c9ea6dea7ac54e2ee800d8  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️index.test.ts
e0a3b40a20620d710fdb5723a80e1f99e52ed2675b2b2b7e05eddf2df7268c59  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️vitest.config.ts
76f68665728405145b8847778fd1f9509ffc9d9a96bf490451861559289aaed0  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/tsconfig.json
e0a3ef816cbebebe8a76f750c5dd8f4aec5763e7f558c5490e7f97dadec287ea  🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧵️shard-client.ts
836fe0351b67f1a86e953b5c41cb526fb67e1ef99090f377f43c714893751191  🧰️framework/🔨️modules/🎭️actor/📤️return/🟦️component.ts
```

## SHA-256 After

```text
158af310c62ca4d6b620e3ff4bfaae620974c1d24213dd5501fee818bea5b245  🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🖼️surface/🟦️component.ts
800139cfb8c99af38630e1ab8bcc540c322bb1c31a128826149e628ce73cbcc0  🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🗂️nodes/🟦️component.ts
36d995844ea0eb671798213e96c671ebae72e400f22c5cbf5cd4c5b77dfd096c  🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🛡️validation/🟦️component.ts
be54a59db1c254b1a76e1b2e4d78259ba901689a9d59f9f82db33321f8924a99  🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🛡️validation/🔬️graph/🟦️component.ts
71a34fefe4c3393d7f5948306800a1aa3bd5713445d8517af23a303609fa0801  🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🔢️hash/🟦️component.ts
cccb14e2851cd4a3ba2a83e4b176db256dbe16558bc67a1d5389c46427043788  🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️component.ts
13e4936e461c5f836b33320522ced1ecb1b4eb25d27d15eebfe11d1575ce119b  🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/🟦️component.ts
79d2138edad0e841c8c3131a4994d4cda6ceff0669e4c3dd8e456d57ab4908ac  🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/🟦️component.ts
289527fa0b7851cb7785830288858fbd1ca5c3378dc12c61a7f8759ff43f93d5  🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🟦️component.ts
0754743b5802ccc6e956bf6ded8aa7c4fcaf010795e761ba6ad1a9e9bb1bbe00  🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️component.ts
1d237ad89846103f660ed858ccabeddf27b07e9f286c8b91b2819a8b714eaa71  🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🔒️transport/🟦️component.ts
af27ecc7b6c7f6f5bd676edf86be7e2d73ee38aacba4a196e285403555ed98e9  🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️component.ts
0bdf5eca18c5b347e219321da7eed7bbf9477cc6661147167c2cdfa686f01db2  🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🔢️bytes/🟦️component.ts
3ac49d7eb43a5db72acfea50b58709769f753767f0cd3cce92e0621cde51e30d  🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🟦️component.ts
12d4e5a37c8370b1557a7bbddeaaec461cd3da0bbe185a412b73606b5cb2ab0e  🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/📖️read-lease/🟦️component.ts
ed218df0bd0aea5e579ad966da31f250625ffdfab752de48dd301fef89343a14  🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🧾️typed/🧩️pack/🔤️base64/🟦️component.ts
c5ffb34c562e67f6dfa822c3add1eb32170c54cd1b551ab3dcf329142bdfe25b  🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🧾️typed/🧩️pack/🧾️value/🟦️component.ts
8365737e4967937495b9e17e21368f88720a3194a76c03f9bd73408422b16b01  🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🧾️typed/🟦️component.ts
bdb4aabbd573140f85948ea7c0755c058b39e6d96d810963a9b0e5bac6b7f907  🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🧾️typed/🧩️json/🔤️string/🟦️component.ts
0ff0e4666b2bc332cf60b2a032309f2917208d9e3578813f8b4fce8f083b2f4d  🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🧾️typed/🧩️json/🟦️component.ts
d0399f625aa3bb64a0554f2dd8391679d382266490c147e62131e1034ccbfc9c  🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🧾️typed/🧩️json/🧾️value/🟦️component.ts
3dc1020baadf5742106c61ab2b012bbccfcee7f8987b624669341aa5ed6ab892  🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🟦️component.ts
d37b4c23a29108441f2ab5a38bad2e1f420f30f45b9bb3aaa2b81ddfc8c31785  🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🔗️binding/🟦️component.ts
dcaedbe3a13dd9aab85567cfb052651733faa87beb288276ad86150529abfe05  🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🔗️binding/🗂️index/🟦️component.ts
6dca1d11f1cb9441ad43bb481e18f1b6c5afc3bda559f9828811e75f60a08610  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/UiDocumentStore/🟦️component.tsx
da8313927ce2c79598ec93a6026a73b30c0167eb32e8be337cd9ae99d282704b  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/UiDocumentStore/📥️intake/🟦️component.ts
a52a1665039f01d8872c69a18f237215284c2b59c6ce621622771f5b81f69898  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/PluginRuntime/🟦️component.tsx
125c2f26ceca9513ade80face64bda52ea74a78a4e1b29c066b7eca71e785b71  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Interpreter/🟦️component.tsx
326b526b4ac4e858a4122fa7fb8f3c5b15920c71f3c9ea6dea7ac54e2ee800d8  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️index.test.ts
e0a3b40a20620d710fdb5723a80e1f99e52ed2675b2b2b7e05eddf2df7268c59  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️vitest.config.ts
76f68665728405145b8847778fd1f9509ffc9d9a96bf490451861559289aaed0  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/tsconfig.json
e0a3ef816cbebebe8a76f750c5dd8f4aec5763e7f558c5490e7f97dadec287ea  🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧵️shard-client.ts
836fe0351b67f1a86e953b5c41cb526fb67e1ef99090f377f43c714893751191  🧰️framework/🔨️modules/🎭️actor/📤️return/🟦️component.ts
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

