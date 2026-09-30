# Final Guest Media Owner Context Refresh

Five actual component guests were rebuilt and materialized after the supplied media-owner SDK, graph-text snapshot retirement, gesture projection, retained BREP extrusion and final mounted retained-fault handoff/completion refusal framing repairs settled. All use the existing `@semio-tech/framework-os-dev:plugin` pipeline, `wasm32-wasip2` and `wasm-dev`, with default owned WASM caches. `NX_DAEMON=false`, `NX_CACHE_PROJECT_GRAPH=false`, and `--skip-nx-cache --output-style=static` were used. Cargo may reuse unchanged compiled objects; Nx was explicitly uncached. No unrelated process was stopped and no geometry source was changed during this final SDK refresh.

SDK root SHA-256 was identical at dispatch and final verification: `44293041f84ffff7c945d73ea472bd1e16b19b3a72f61c7b980771e7144efa3d`.

## Actual Final Producer Receipts

| Filter | Guests | Final Uncached Nx | Log |
| --- | --- | --- | --- |
| `flow-extension-brep` | Flow + BREP, 2/2 | 2m 53s, exit0 | `🗑️generated/guests-owner-context-final/flow-extension-brep-final-fault-handoff.log` |
| `sequence` | Sequence, 1/1 | 37.8s, exit0 | `🗑️generated/guests-owner-context-final/sequence-final-fault-handoff.log` |
| `playbook` | Playbook + procedural, 2/2 | 44.5s, exit0 | `🗑️generated/guests-owner-context-final/playbook-final-fault-handoff.log` |

Earlier SDK and retained-extrusion refreshes also passed, but the table above is authoritative: all three filters were rerun after the parent corrected the generic mounted retained-fault handoff and completion refusal framing following actual app230 RED. These final receipts replace the earlier Flow/BREP2m38s, Sequence35.0s and Playbook53.9s checkpoint.

## Independently Checked Guest Hashes

For each guest, the declared `hashes.wasmSha256` equals SHA-256 of the actual Cargo component file, and `hashes.coreWasmSha256` equals SHA-256 of the browser materialized core. Authored JSON and semio descriptors match their staged bytes exactly. A separate read-only Bun verification decodes each actual semio descriptor, proves canonical re-encoding equals the original bytes, proves JSON and pack forms agree, blanks exactly `hashes.descriptorSha256`, then independently computes SHA-256 of the encoded descriptor and compares it with the declared self-hash. All five passed; receipt `🗑️generated/guests-owner-context-final/descriptor-self-hash-final-fault-handoff.json`, verifier exit0.

| Guest | Actual Component SHA-256 | Actual Extracted Core SHA-256 | Verified Descriptor Self-Hash |
| --- | --- | --- | --- |
| Flow | `fee399586fcb0057de7428bbc5c89c7b1c8668c3a663c8408449fa0b30b80270` | `81ddcfb4d02827d226a79742edc8b6fc19bdacbd42e62576d45821dccdcb8e5a` | `643c83f628fcde4969a84f36383279d364d6e5a8ac8854df32b721a8596a3963` |
| BREP | `398de155ebc8871805ce3df61449bfa302405545ae55d3a22c672fdcbbbe93a2` | `d016abd83e8eaa946f8f0e7b68d5110621a5b93c4535ab2639124129c3b399b4` | `52041240969ec1d6023e438a6239b1ad998d4d3d180ede46db15c2c60d902dcf` |
| Sequence | `dfccea23d6b537859d483b392043054b677c99fdedb5888f7b743fee85849017` | `49717ce677a5883ba2277e50d3c3c17ae9ea3a6fab41228b96303782e5465296` | `01d6be642c2ba4fa1d84bedad5a01e38b3b0ec5c7a03fb9b9c5238eb66a45b3e` |
| Playbook | `e81459e8b8bde271a9dad4a5a2a4b102963f7233c4c85489cf91bcecfac2aca3` | `b365d0ab62810031e4b7ffb45b881a7efae6bf330136c3ff7ee60ffb08055197` | `3c2c4241e732967f35329198cfffd00b2d0948a0bc55223382620b22fb38c2e2` |
| Procedural | `413268acb9aff5e9ad2acee7a2a8575cc57c2477aaa8256280898a598436feb1` | `8fb471d68bda239cce8080a896b5261a319a6cd2e0b1d2f56c9b961bcb1fab8a` | `284d118b0a0acac9e1885a052b4374c08f8dc524bf7b21ac74d7ca9df0a6c16a` |

## Exact Authored Ten-File Manifest

- UPDATED `✏️s/🔌️plugins/🌊️flow/🔣️.json`: `b2945fb0e89ffb8001eecbe94c1c01970ac4c859a63f2de92371f4afec17930d`
- UPDATED `✏️s/🔌️plugins/🌊️flow/🛂️.descriptor.semio`: `7b7d5ce46e9804debd3e7eaa8ea914d815cfa2d6775778033278eec7fdc35a2c`
- UPDATED `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🔣️.json`: `e553ec31793cc6caea947b8cf12ee0777ca1cc05bbb31bd068f3db7ef4ff6f47`
- UPDATED `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🛂️.descriptor.semio`: `dea892e1fc9b7b1b46aedf04caaf2f9d9f2b5fcb908f7320fadbcfb55adb773a`
- UPDATED `✏️s/🔌️plugins/🎬️sequence/🔣️.json`: `32ddb59888d17b48a2d307cff8b2d0d2c4cf2fea682b61a9beb3913130dc9b95`
- UPDATED `✏️s/🔌️plugins/🎬️sequence/🛂️.descriptor.semio`: `b6dff505f6a9da6260b6742727a236aaf5f23997b45bb89881ed128ba4640396`
- UPDATED `✏️s/🔌️plugins/📖️playbook/🔣️.json`: `6f3a8a5f44d2d1e2396137524ecdaab10a4cc68cc1859d627bc6e0305150a329`
- UPDATED `✏️s/🔌️plugins/📖️playbook/🛂️.descriptor.semio`: `f72345ca2f7ab74e616271b5f8f4f73395cd5848da0f7d32f23fe4ab08d52dd2`
- UPDATED `✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🔣️.json`: `e217fb40a6088dad4265ba2433ff93fe68a7d6bdfe79bd0d44026f8f214455af`
- UPDATED `✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🛂️.descriptor.semio`: `b328eeff5fb954568785c31fdc3dde2f8714b37e9c30bedffedb4eba565f47b8`

## Producer Boundary and Law Accounting

The direct plugin producer materializes component descriptors, browser core WASM and bindings and publishes extension bridges. It does not emit activation-owned `.source-content-sha256` or `.source-stat-index.json`; those sidecars belong to later activation/Vite staging. This report therefore makes no false direct-producer source-freshness sidecar claim.

Original composition provenance remains355 =307 moved +48 retained. The separately additive supplied geometry owner app law makes the current composition356 =308 moved +48 retained (concrete app276 plus the other32 moved laws). Read-only current fixture inventory confirms63 suites with308 named laws and17 retained units with48 named laws. Generic plugin13, artifact IO47 and the new BREP extrusion regression are independent inventories and do not rewrite original355 accounting. The kernel repair preserves all original slider/orientation assertions and explicit Session lifetime laws; its exact six-file source manifest and native RED/GREEN receipts are in `🔁️2026-09-30-retained-extrusion-ownership.md`.

These builds confirm native WASM production and browser materialization on macOS; interactive browser behavior is not claimed by this guest receipt.
