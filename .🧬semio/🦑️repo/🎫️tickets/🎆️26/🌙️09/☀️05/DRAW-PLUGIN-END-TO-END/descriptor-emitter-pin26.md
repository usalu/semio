# Prototype Descriptor Emitter Pin

The normal component door accepts `component --manifest <Cargo.toml> --published-emitter <binarySha256:receiptSha256:indexSha256>` from an issued prototype invocation. The build owner resolves the host platform from the actual process. If the immutable generation is absent, the owner authenticates the matching installed emitter and copies its executable, index, receipt and complete original compiler inventory while the installed publication reader is still held. The managed destination derives its identity from all three digests, mode and platform. A later replacement of the installed emitter leaves the selected generation intact while it remains in the repository products store.

The capability preflight and actual describe stage read that same managed generation. They verify the actual executable, its retained copy, the original selected non-test Cargo binary output and the exact completed receipt and index. Descriptor stage cache keys and remote tool transport still use the verified emitter executable SHA-256. The pinned model publication writes `emitter.json` in the same product as the component, descriptor pair and model receipt. The sidecar binds the component SHA-256 and semantic descriptor SHA-256 to the full emitter generation identity. The retained model reader validates that binding before it returns the model. Unpinned builds retain current-workspace emitter freshness, and release does not admit a pin.

The installed Darwin prototype bundle validated under the production receipt reader and pin check with this selection:

```text
9819ef704a48c6c605a8a0f5c9b06b7756b3943f392e9e5d0c941920e5dde539:dea102640d8fc923b20971bd3b889613773177575987a6898497875796da8598:d25d11baa75e4b3b618eb8269c8b509d5e357f27aa471e001e741e9cbdbdac09
```

The targeted emitter request and freshness suite passed 13 tests. The language-neutral JSON cases agreed with Ajv independently. The lifecycle test proved that an absent generation retains the authenticated installed reader through publication, that a wrong expected emitter cannot write, and that an existing generation avoids an installed refresh. Three related artifact tests passed and independently checked the model reader bounds, single component driver and complete emitter request. The prototype model reader now admits up to seven files, including two existing actor preparation files and one pin binding. Release retains six. With the real installed packet, the production pin check accepted the exact original output and refused a change to each digest and an absent executable blob. The driver and model entry points imported successfully. This verification did not launch Cargo. The full component build is a separate integration check.

## Registered command rejection

A negative integration invoked the actual registered component script through its normal script envelope with an incomplete tool selection. It reached the descriptor-emitter parser and exited 1 with the required three-digest format before compilation.
