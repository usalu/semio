# Terra — Hub Headless Trusted-Artifact Catalog Audit

**Scope.** Read-only snapshot for `🎯r2603`, taken 2026-09-03 while the shared tree was changing. This is an implementation packet, not evidence that a hub can start a catalog today. It covers the production route required before composing `ValidatingCanonicalArtifactAuthority<PluginHostTrustedArtifactCatalog>`, `DbImmutableArtifactBlobStore`, and `HubVerifiedCheckpointPublisher`, then implementing P2-C lag rebootstrap.

## Decision

**Do not compose or advertise a zero-touch headless trusted catalog yet.** The hub has the three useful authority pieces, but `os-hub` does not construct any of them, does not load a plugin graph, and does not register an executable document codec. The generated registry is also not a trusted deployable catalog: only one of 59 expected components is present locally, it depends on a missing `stdio` component/descriptor, and the registry's SHA-256 component assertion is neither available to nor checked by the production loader.

The sharpest trust blocker is the missing **single retained binding** from a vetted registry record to `(pluginId, packageId, exact component bytes, SHA-256, PackageRef/Blake3, decoded descriptor, loaded manifest, registered native codec)`. Existing paths retain fragments of that identity, but no production path validates and preserves the whole tuple to hub startup.

## Evidence and limits

All observations below were reread immediately before this report. Files and source anchors are relative to the repository root.

| Evidence | Result | Meaning |
|---|---:|---|
| Generated registry `…/🔌️plugin/📇️registry/🤖️generated/🟦️plugins.ts` | 33 plugin + 26 extension rows = **59**; 40 hash-bearing | Generated, ignored output; it is a current workspace observation, not committed deployment input. |
| Checked-in static descriptor JSON `cratePath/../../🔣️.json` | **40/59**, all tracked | Inputs parsed by registry generator. |
| Checked-in binary descriptor `cratePath/../../🛂️.descriptor.semio` | **40/59**, all tracked | Production `WasmtimeNodeHost` decodes these, but does not integrity-link them to a component. |
| Expected catalog components in `target/wasm32-wasip2/{debug,wasm-release}` | **1/59** (`process`) | A local build residue, not a committed catalog. `git ls-files -- '*.wasm'` contains only two `jcoprobe` fixture WASMs. |
| Independently computed SHA-256 for that component | `process = adf1fc…934abeb` | It matches the generated `wasmSha256` record. No other expected root component exists to check. |
| Registry `check` | **inconclusive** | `bun nx run @semio-tech/plugin-registry:check` ran silently for two minutes and was interrupted; it must not be represented as passing or as a freshness proof. |
| Static declared artifact kinds | **57 declarations, 48 schema strings** across 40 descriptor JSON files | Declarations, not a live headless manifest/codec catalog. |
| `os-hub` production startup codec registrations | **0 found** | `📦️bin.rs` creates DB/directory/router state only; it never calls `register_document_codec*`. |

`descriptorSha256` in the registry is descriptor metadata from `🔣️.json`, not a raw checksum of `🛂️.descriptor.semio`. Independently hashing all 40 binary descriptors produced **0/40 equal values**. This does **not** prove the descriptors stale; it proves these fields are not usable as binary-descriptor byte attestations. A trusted loader needs an explicit descriptor-byte hash (or a signed envelope containing one) if that file is a trust input.

## Complete generated registry census

`P` below means the exact prefix `✏️s/🔌️plugins/`. `J/B/M` means the owner-root `🔣️.json` / `🛂️.descriptor.semio` / generated hash metadata exists at the snapshot; it says nothing about freshness. `W`, `C`, and `D` are the exact generated `wasmSha256`, `coreWasmSha256`, and `descriptorSha256`; `C=same` means byte-for-byte the stated `W` value. No emitted `PluginBuildTarget` contains an explicit Cargo `packageName` or `packageId`; `cratePath` and `wasmOut` are all this generated row supplies.

### Plugins (33)

| pluginId | crate path; wasmOut | dependencies | J/B/M | declared hashes W; C; D |
|---|---|---|---|---|
| animate | `P/🎞️animate/📦️packages/🦀️rust`; `semio_s_plugin_animate.wasm` | stdio | yes/yes/yes | `5fff7e3ac148177243275445e12535fd89c433f6fa50316572bcdda9b3d97590`; same; `12a912e82f98d54f405262123150f41035a15234332a1abc971062ac7e973b17` |
| architect | `P/🏛️architect/📦️packages/🦀️rust`; `semio_s_plugin_architect.wasm` | stdio | yes/yes/yes | `2301bc724c96c3f6ea698bc1eba4feb50a0b0b4d1dfdbffa94a912c7e9dab510`; same; `09d0f7320243a4aa38d5c83fa7d0a75ed398756edcb093c848adf515d1c1c4d8` |
| block | `P/🧱️block/📦️packages/🦀️rust`; `semio_s_plugin_block.wasm` | stdio | no/no/no | — |
| cad | `P/📐️cad/📦️packages/🦀️rust`; `semio_s_plugin_cad.wasm` | stdio | yes/yes/yes | `64a36cc37cb80d8d0c122af7c22272e1749730a45e2eb18657e435f6614c8823`; same; `ff3daed49568aaec15d35de6067f2df0956bf988de1db8baa98560f10063b867` |
| dag | `P/🕸️dag/📦️packages/🦀️rust`; `semio_s_plugin_dag.wasm` | stdio | yes/yes/yes | `55c9da9026706dbcd47277335eda53abf66e3ecf19fd848280a95b7a531f51e2`; same; `53d81f2b0927fbc1383cccb1c989a5fe190fd98ea582786bd6ea1846aea5258d` |
| demonstrator | `P/🎪️demonstrator/📦️packages/🦀️rust`; `semio_s_plugin_demonstrator.wasm` | cad, gis, procedural, process, puzzle, sourcing, stdio | yes/yes/yes | `e39095467e06ec3d2fd45543e73bdcfa12d03e4a5a941d9145cd46f570d0ae63`; same; `72e0822284f68c9fd9fa60552db84cd489b1ca9c770adf389dd6e17cb57a2ff3` |
| draw | `P/🖍️draw/📦️packages/🦀️rust`; `semio_s_plugin_draw.wasm` | draw-fsm, stdio | yes/yes/yes | `4bccf647dd64b0d6088e7338a25e7ed1326a412f44660459f0d6c9cab0e79714`; same; `b9d12f23271b085b41da39d7ba395ea78604cab8006b6b00e1ee39aa5265a1bd` |
| energy | `P/🔋️energy/📦️packages/🦀️rust`; `semio_s_plugin_energy.wasm` | stdio | yes/yes/yes | `1c0f620a5d442096c9683acf7095f470375c8b7efa0821076d8e548b8d706f20`; same; `383853b475b0308336f8088fe067d27fa2f525b21349d70b080b07aa86ae2ec1` |
| fem | `P/🏗️fem/📦️packages/🦀️rust`; `semio_s_plugin_fem.wasm` | stdio | yes/yes/yes | `924176ed3c2bd2415f14218d6671a485db3d06931f2b47e67c5170f715661e13`; same; `f0c10888f9dc7101c596b0e8b837fcbd439cb031738dd233e767cc8ad59f6fdb` |
| flow | `P/🌊️flow/📦️packages/🦀️rust`; `semio_s_plugin_flow.wasm` | stdio | yes/yes/yes | `b996f5722473bb19e91f3ab4b38cd67bd95cf1852586684e836a260a642eaed2`; same; `1996bf86c181d869f1d9839d3b4763146ce18c99da5a3c0cc67470398c10f2d4` |
| forms | `P/📋️forms/📦️packages/🦀️rust`; `semio_s_plugin_forms.wasm` | stdio | yes/yes/yes | `a63d0dfc2619a9e7f05ae83c119717989ff8a32667f4771838c5c5599014b152`; same; `8e0b3d00eb48790dd1f31070462adaf925fc00cdc8a664c1865366b6589c0d88` |
| gis | `P/🌍️gis/📦️packages/🦀️rust`; `semio_s_plugin_gis.wasm` | stdio | yes/yes/yes | `78a180b8fcf22d57778a88b5bc93821e832ebf63bf23d90ae36bfd1e756c27eb`; `90a7cd5fc5aa1d0ccb755c7a920ea779d60398e942305ea6bc71478f43d0f15a`; `5be61f0b3aab6dc86dbb992810d5f4cc5632dd32e27a74d966fa47ba033af06b` |
| imperative | `P/📜️imperative/📦️packages/🦀️rust`; `semio_s_plugin_imperative.wasm` | stdio | yes/yes/yes | `32cdff3f114c8390f85c3f7ed928525d25ed52be15b147cbfa58ec64a0e4234f`; same; `7dc6bc0885b16f4a552ecdf5e1757da8d336efebcb81b603c87341ae25a66506` |
| layout | `P/📏️layout/📦️packages/🦀️rust`; `semio_s_plugin_layout.wasm` | stdio | yes/yes/yes | `dfde964f079e83c8f8cc67873cd495448be7a06ac8f6776e8585aef4b4f5b0bc`; same; `66358711ac5cd24af7edebf20ba9e40c3a7d96bb9e28ba19bc9d548b62c026db` |
| lowpoly | `P/💠️lowpoly/📦️packages/🦀️rust`; `semio_s_plugin_lowpoly.wasm` | cad, stdio | yes/yes/yes | `95f9ac4920995ae69e8807c90be68082694a15b2466910d3cf257476a8940c02`; same; `2e2e5e1e43988b270aa356d10fca3608c594faa7b7f6a47b9c1efa93fbb45751` |
| mathematical | `P/➗️mathematical/📦️packages/🦀️rust`; `semio_s_plugin_mathematical.wasm` | stdio | yes/yes/yes | `0b801ea2f23f760c1b8b2b24a7f137af965cc5825da11065cac51cd179b14716`; same; `824b2c80a380ac3cebb2c39ec5ff9b95282fb98e6888f6c91293f85e0263b227` |
| norm | `P/📕️norm/📦️packages/🦀️rust`; `semio_s_plugin_norm.wasm` | fem, stdio | yes/yes/yes | `ee09ede9e0a96f42d31342b2e646edfb17b05f3d63b47315148774eb9f99dbfc`; same; `dbca604de90af12da82cb423792a4ced55422e75c1f1baee863caf898f0295c3` |
| note | `P/🗒️note/📦️packages/🦀️rust`; `semio_s_plugin_note.wasm` | stdio | yes/yes/yes | `a60a593e311b5e4b6e366884638095c8dec2aa0e6bed9792163d6f2cef35a5b7`; same; `1b8c29c800f1fd38f95f6754ec982585b59595a60ddf06fdbbadb6738850a093` |
| playbook | `P/📖️playbook/📦️packages/🦀️rust`; `semio_s_plugin_playbook.wasm` | stdio | no/no/no | — |
| procedural | `P/🌀️procedural/📦️packages/🦀️rust`; `semio_s_plugin_procedural.wasm` | stdio | yes/yes/yes | `42503bf34bf77e69d5e730d75f9b58ce3666f42eadd4b3f810d2d80af69a96bd`; `370310791b85c6cc96ea370aa50e34dea530c256abc8b606139b9049064ee71c`; `932ed0810ec0b2fd27381f847f343bcdacc1e5b683db7c853d8128779b568a79` |
| process | `P/🏭️process/📦️packages/🦀️rust`; `semio_s_plugin_process.wasm` | stdio | yes/yes/yes | `adf1fc2a97ec390e8c2e6f26f474fbcb211e63d5ca7a45e8eb930c6db934abeb`; `63e4f7d59977ac763a86abbc6c3e0e51e0cbb4ee3aa0556685d4f8c04729e7b8`; `5797c6564c2600528f82605534cc116069f5b5c1fffcecd53b46449f3d9526a6` |
| puzzle | `P/🧩️puzzle/📦️packages/🦀️rust`; `semio_s_plugin_puzzle.wasm` | stdio | yes/yes/yes | `9a44199ad3131cd1317895dfe2ae29915c7846cd24a9910c00f6f9357d1be942`; `b273db280223bc136ccf3eca82c648f148cb0ca167f77eda27db5a9b17b79914`; `cbec74394759b25b11065af8bdf0380c51d812d683ebb99af3568b564adfd31d` |
| raster | `P/🖨️raster/📦️packages/🦀️rust`; `semio_s_plugin_raster.wasm` | stdio | yes/yes/yes | `9040c81c6daee99c3d31b9eac685c68ea24d551ac7f33f31cad68fe75487e4e6`; same; `26760a5a3c146b1612a8e8036c877f91a17c13cef425b94a174127df3e33bd94` |
| reasoning-mindmap | `P/💡️reasoning/📦️packages/🦀️rust`; `semio_s_plugin_reasoning_mindmap.wasm` | stdio | yes/yes/yes | `7686a3193c6aeffe74e8e73d76b842112e892e57f9f3aa9ed04d39bc8bc1c2b8`; same; `eb21b2587a19242762803823f748628b1eb1553c783f6281dfee25ac72706f93` |
| remodel | `P/📸️remodel/📦️packages/🦀️rust`; `semio_s_plugin_remodel.wasm` | stdio | yes/yes/yes | `77ef3c98d134f1164cdd388911333b0618bcec94fead7c11ad6fdd24abb125b5`; same; `1e1dded5a4979ce72c0ff11f4e12e8336df93784c89c0f53b0ee573b694fbe62` |
| s | `P/🪐️space/📦️packages/🦀️rust`; `semio_s_plugin_space.wasm` | stdio | yes/yes/yes | `762dad6b1eca109108ff781d0697bdc2114ed8869b692c2cf88cc60ec03209af`; same; `df021b9a83bcb48ab858afe4a8f2c2e30d69f8166850ddebb064421109b3fed6` |
| sequence | `P/🎬️sequence/📦️packages/🦀️rust`; `semio_s_plugin_sequence.wasm` | imperative-control, imperative-effect, imperative-math, imperative-text, stdio | yes/yes/yes | `bbcf24176893beb37e0dcdf36f658f52a62b8a5e48163130cd5f02371b2a6a79`; same; `5c5ee126f62f14b60a81d95575c85186db47ec9b7712d0e56d5ba6b2a032088a` |
| shooting | `P/🎥️shooting/📦️packages/🦀️rust`; `semio_s_plugin_shooting.wasm` | stdio | yes/yes/yes | `2e16eed70a875e078501c439d8f05c162163f1193bcaee4f11b41f0b2f2eed01`; same; `ad86c4d9cf0730ae4b512389898962bb9eefd1f631f8543d7fd8143be3276129` |
| sourcing | `P/🪵️sourcing/📦️packages/🦀️rust`; `semio_s_plugin_sourcing.wasm` | stdio | yes/yes/yes | `c27638455e4eba364a044826adb2e5ad2b679c88601d80757f40e354c4c12298`; `81b04b6396cf37bd2fee9119cd802592bbf51a40ceef05c7af4c710801bc9045`; `fa7ea0be8379f959e0e9b7bbf2c5ae4168a3b29104e8d9d2015c27847632ac28` |
| stdio | `P/🗄️stdio/📦️packages/🦀️rust`; `semio_s_plugin_stdio.wasm` | — | no/no/no | — |
| trinity | `P/🔱️trinity/📦️packages/🦀️rust`; `semio_s_plugin_trinity.wasm` | stdio | no/no/no | — |
| vcs | `P/🌿️vcs/📦️packages/🦀️rust`; `semio_s_plugin_vcs.wasm` | stdio | yes/yes/yes | `74771b987f39e483da63efdb21006a3ce511ad5edd1c3bd0de05543bef00d925`; same; `b702fe11bb1c92bb06226ccce58792ccd37fa01be8313a740e52ea6a48e8329e` |
| writer | `P/✒️writer/📦️packages/🦀️rust`; `semio_s_plugin_writer.wasm` | stdio, trinity | yes/yes/yes | `6507f654884a1c93e633bfc4cd42b5cebb880925ca4e03ca278bc9ccf191c18e`; same; `ced53b5c3f821e2cb2fc847868737e6c695e8e32aa9c2358886df559214e750d` |

### Extensions (26)

| pluginId | crate path; wasmOut | dependencies | J/B/M | declared hashes W; C; D |
|---|---|---|---|---|
| cad-extension-aec-building | `P/📐️cad/🧩️extensions/🏢️aec-building/📦️packages/🦀️rust`; `semio_s_plugin_cad_aec_building.wasm` | cad | yes/yes/yes | `af59b52fd8c7f60d5eb1195406a65d4eaf2de59b471fe54ddddd9dd1ec7d70c0`; same; `4f06e341b211c507f489e3929838512d79015d79a3b8fd97f3c4ef1f3a2ee43e` |
| cad-extension-aec-building-energy | `P/📐️cad/🧩️extensions/🔥️aec-building-energy/📦️packages/🦀️rust`; `semio_s_plugin_cad_aec_building_energy.wasm` | cad | yes/yes/yes | `e5b2ff618804be66178d53f4f5302d9e08974e7f982a94163e812ddd7c315722`; same; `c49d812a1ef6056b2a7a38b886a91d5b2ceb3bce0f498d4e2f7fd5e709b8489f` |
| cad-extension-aec-building-structure | `P/📐️cad/🧩️extensions/🏛️aec-building-structure/📦️packages/🦀️rust`; `semio_s_plugin_cad_aec_building_structure.wasm` | cad | yes/yes/yes | `ec7281c5e733b921760a7a365660b0d105442b20c9064168b28168f92bc97fc9`; same; `71cd360006a6b82ab5e42bbce01580600441234af040753100200e3f2f736dc4` |
| cad-extension-spatial-shape | `P/📐️cad/🧩️extensions/📐️spatial-shape/📦️packages/🦀️rust`; `semio_s_plugin_cad_spatial_shape.wasm` | cad | yes/yes/yes | `d77ec8ebc85fd286e5cdb3f24d037137461e9293524cbf94d7809a7d58fd98ab`; same; `7919165697487d2cdee0c6e4162a25dc8f6e57d0fa751f6239ed5b6b872de5a5` |
| flow-extension-bim | `P/🌊️flow/🧩️extensions/🏗️bim/📦️packages/🦀️rust`; `semio_s_plugin_flow_extension_bim.wasm` | flow | no/no/no | — |
| flow-extension-brep | `P/🌊️flow/🧩️extensions/📐️brep/📦️packages/🦀️rust`; `semio_s_plugin_flow_extension_brep.wasm` | flow, stdio | yes/yes/yes | `a5b648c7575d312ab9c65fad854ef5c33552361e6c72cb61316d006a8694dfc2`; `0d14dfa6d5bb5c69b0a39c7f716b4c327f3de3275139db8d2541cc310328d550`; `da5ccae1e44d4022546d87ea82a30e223a58266c58b0efc9e1635497924251f3` |
| flow-extension-dictionary | `P/🌊️flow/🧩️extensions/📖️dictionary/📦️packages/🦀️rust`; `semio_s_plugin_flow_extension_dictionary.wasm` | flow | yes/yes/yes | `a6c38efadee3b569ceb61a618951c6bfecab568bd3819a69acb0bdfff12ff29d`; `9be9766880776481c47af78498492ba7d71937ce2bea3595900e153124a7643e`; `56207a8c59d586c53d6603c80812c5d7be175d123dde5a3e24c2210191a94543` |
| flow-extension-draw | `P/🌊️flow/🧩️extensions/🖍️draw/📦️packages/🦀️rust`; `semio_s_plugin_flow_extension_draw.wasm` | flow | no/no/no | — |
| flow-extension-list | `P/🌊️flow/🧩️extensions/📃️list/📦️packages/🦀️rust`; `semio_s_plugin_flow_extension_list.wasm` | flow | yes/yes/yes | `b704f249a888288f4d13f4c322371f7eb744dd915545ba7387ce8b24053170ab`; `05963254be2f7b930616e3fab999de8886c123fd21d64ea2efec378b0d923b9e`; `dfe347d736dce2eb6cefe3937b368182fd8fe464c389612d84995bc2ccc7fdfe` |
| flow-extension-logic | `P/🌊️flow/🧩️extensions/🧠️logic/📦️packages/🦀️rust`; `semio_s_plugin_flow_extension_logic.wasm` | flow | yes/yes/yes | `ec11011c12be2573da2aac46df43d20c709df1f06d29ec5c9d34f778b0aeffda`; `c2d98a64566f42468585b89d08dce5a0be8b64b8ee8465678ca520f494de15ba`; `0effe44b7293cbd4029ec14c974c7a702e27c4e9f1510484b63a5b4c19ef2b2d` |
| flow-extension-math | `P/🌊️flow/🧩️extensions/🧮️math/📦️packages/🦀️rust`; `semio_s_plugin_flow_extension_math.wasm` | flow | yes/yes/yes | `db18a550fc3efba8c5f356fb671b8a96f62bcdeb811fa9e2d7580bc7d9ef2379`; `133e8cbf94e80d6172aa3a0c8a1edddb240e9094bc424cb1799410a1f156c54d`; `805d0e505a71c67245b511c475ddcdcd66947c631fd8053b091e221595a5cff8` |
| flow-extension-primitive | `P/🌊️flow/🧩️extensions/🔤️primitive/📦️packages/🦀️rust`; `semio_s_plugin_flow_extension_primitive.wasm` | flow | yes/yes/yes | `0f73a465132b44308438bf12bebee2e8163965d34047d0e088d2b36d71b38424`; `7b448976b669c674f6a800668616cb8112132adae92acfe83a503a08f50fbe8c`; `d0db4f5d79aef0bfd8454b0d76656d421ceff0ccb7d6534624ff9e25c6cd156d` |
| flow-extension-text | `P/🌊️flow/🧩️extensions/📝️text/📦️packages/🦀️rust`; `semio_s_plugin_flow_extension_text.wasm` | flow | yes/yes/yes | `c2d238545722d63e064198cf29bb7ebc927db713bfecf947665f5a328e02c1d7`; `41ac9ea01f64d68575dca620d2380a89496cb4159ed7b42a5879c7008ed8835a`; `26a92210dbafe6b41b306076f3beee34e3f756d7164bc96e20b9f2d0dd360003` |
| imperative-extension-control | `P/📜️imperative/🧩️extensions/🎮️control/📦️packages/🦀️rust`; `semio_s_plugin_imperative_control.wasm` | imperative | no/no/no | — |
| imperative-extension-effect | `P/📜️imperative/🧩️extensions/📣️effect/📦️packages/🦀️rust`; `semio_s_plugin_imperative_effect.wasm` | imperative | no/no/no | — |
| imperative-extension-logic | `P/📜️imperative/🧩️extensions/🧠️logic/📦️packages/🦀️rust`; `semio_s_plugin_imperative_logic.wasm` | imperative | no/no/no | — |
| imperative-extension-math | `P/📜️imperative/🧩️extensions/🧮️math/📦️packages/🦀️rust`; `semio_s_plugin_imperative_math.wasm` | imperative | no/no/no | — |
| imperative-extension-text | `P/📜️imperative/🧩️extensions/📝️text/📦️packages/🦀️rust`; `semio_s_plugin_imperative_text.wasm` | imperative | no/no/no | — |
| playbook-module-procedural | `P/📖️playbook/🧩️extensions/🌀️procedural/📦️packages/🦀️rust`; `semio_s_plugin_playbook_procedural.wasm` | playbook | no/no/no | — |
| process-extension-concrete | `P/🏭️process/🧩️extensions/🧱️concrete/📦️packages/🦀️rust`; `semio_s_plugin_process_concrete.wasm` | process | no/no/no | — |
| process-extension-metal | `P/🏭️process/🧩️extensions/🔩️metal/📦️packages/🦀️rust`; `semio_s_plugin_process_metal.wasm` | process | no/no/no | — |
| process-extension-robotic | `P/🏭️process/🧩️extensions/🤖️robotic/📦️packages/🦀️rust`; `semio_s_plugin_process_robotic.wasm` | process | no/no/no | — |
| process-extension-wood | `P/🏭️process/🧩️extensions/🪵️wood/📦️packages/🦀️rust`; `semio_s_plugin_process_wood.wasm` | process | no/no/no | — |
| sourcing-module-beams | `P/🪵️sourcing/🧩️extensions/🪵️beams/📦️packages/🦀️rust`; `semio_s_plugin_sourcing_beams.wasm` | sourcing | no/no/no | — |
| sourcing-module-slabs | `P/🪵️sourcing/🧩️extensions/🧱️slabs/📦️packages/🦀️rust`; `semio_s_plugin_sourcing_slabs.wasm` | sourcing | no/no/no | — |
| sourcing-module-windows | `P/🪵️sourcing/🧩️extensions/🪟️windows/📦️packages/🦀️rust`; `semio_s_plugin_sourcing_windows.wasm` | sourcing | no/no/no | — |

The 19 rows with no J/B/M are: `block`, `playbook`, `stdio`, `trinity`, `flow-extension-bim`, `flow-extension-draw`, the five `imperative-extension-*` rows, `playbook-module-procedural`, the four `process-extension-*` rows, and the three `sourcing-module-*` rows. Hence every normal plugin in the catalog currently depends directly or transitively on an undecorated `stdio`; a whole-catalog strict descriptor load cannot succeed.

## Descriptor, artifact-kind, and codec matrix

`🔣️.json` is parsed at generation time by `readDescriptorJson` in `…/📇️registry/📜️script.ts`. `🛂️.descriptor.semio` is decoded at run time by `WasmtimeNodeHost::read_committed_descriptor` in `…/🏃️run/🦀️.rs`, using `store::pack_rt::decode_wire_value` followed by `PackageDescriptor::from_dsl_value`. Neither code path compares the raw descriptor bytes with a registry digest or component bytes.

The static declarations that have artifact kinds are below; a comma separates `kindId:schema`. These 57 declarations reduce to 48 distinct schemas. The remaining descriptor-bearing rows (`architect`, `energy`, and all descriptor-bearing extensions) declare no artifact kinds.

| pluginId | static kinds |
|---|---|
| animate | `animate.present:animate.present` |
| cad | `3d.cad:cad.scene` |
| dag | `graph.dag:dag.dag` |
| demonstrator | `3d.procedural:procedural.3d`, `3d.cad:cad.scene`, `3d.puzzle:puzzle.3d`, `catalogue.sourcing:sourcing.curation`, `catalogue.kinds:catalogue.kinds`, `kit.catalog:kit.catalog`, `3d.process:process.3d`, `2d.map:gis.map` |
| draw | `2d.drawing:draw.document` |
| fem | `computation.fem2d:computation.fem2d`, `computation.fem3d:computation.fem3d` |
| flow | `computation.flow:flow.artifact` |
| forms | `form.dictionary:form.dictionary` |
| gis | `2d.map:gis.map` twice |
| imperative | `computation.procedure:procedure.document` |
| layout | `2d.layout:layout.layout` |
| lowpoly | `3d.lowpoly:lowpoly.fixture` |
| mathematical | `computation.equation:computation.equation` |
| norm | `computation.norm.din4108:norm.din4108.document`, `…din16798`, `…din18599`, `…en1990`–`…en1999`, `…iso16757`, `…vdi3805` |
| note | `2d.note:note.document` |
| procedural | `2d.generation:generation.2d`, `3d.generation:generation.3d` |
| process | `3d.process:process.3d` |
| puzzle | `2d.puzzle:puzzle.2d`, `3d.puzzle:puzzle.3d`, `5d.puzzle:puzzle.5d` |
| raster | `2d.raster:raster.document`, `2d.image:2d.image` |
| reasoning-mindmap | `graph.wires:reasoning.wires.fixture` |
| remodel | `3d.remodel:remodel.scene` |
| s | `space.sspace:s.space` |
| sequence | `computation.sequence:sequence.sequence` |
| shooting | `2d.shooting:shooting.scene`, `2d.image:2d.image` |
| sourcing | `catalogue.sourcing:sourcing.curation`, `catalogue.kinds:catalogue.kinds`, `kit.catalog:kit.catalog` |
| vcs | `vcs.document:vcs.vcs` |
| writer | `text.document:writer.document` |

`PluginHostTrustedArtifactCatalog::load` refuses each kind whose schema has no non-zero-hash `directory::os_store::document_codec`. The process-global registry starts empty (`OnceLock<RwLock<BTreeMap<…>>>` in `…/🏪️store/🦀️.rs`), and `os-hub` startup has no registration call. Thus **0/48 declared schemas are established as production-headless executable-codec coverage**. Native source contains possible app-specific registration helpers and tests; that is not a hub registration plan and must not be mistaken for live coverage.

Existing authority tests prove the adapter's behavior only through synthetic `PluginGraph` construction and hand-made `PackageRef` values. The Block/DAG cases manually construct a one-node manifest and manually register a codec; one test explicitly proves a zero pack-schema hash is rejected. They do not invoke registry discovery, target resolution, component hashing, descriptor load, or `os-hub` startup.

## Identity preservation and where it is lost

| Stage | pluginId | packageId | exact byte identity | Gap |
|---|---|---|---|---|
| Generated TS registry | yes | **no emitted field** | SHA-256 metadata on 40 rows | Produced output is not an authenticated hub input. |
| `run/📦️bin.rs` path resolver | map key | no | local target filename only | Reads generated, ignored `🦀artifacts.rs`/`🔣️plugins.json`; it drops registry hashes. |
| `WasmtimeNodeHost::load_runtime_recursive` | map key | constructs `PackageId(plugin_id)` | reads bytes and makes `PackageHash(framework_hash::hash_bytes(bytes))` | `hash_bytes` is BLAKE3, while registry records SHA-256; no equality check bridges them. Package id is collapsed to plugin id without an independent registry package id. |
| `PackageRef` | no separate plugin field | yes | `PackageHash` retained | Correct carrier, but it is local to compilation. |
| `CompiledHandle` | no | no | `package_hash` only | Definition in `🔌️plugin/🖥️host/🦀️.rs` drops package id. `compiled_for_plugin` is keyed only by plugin id. |
| `PluginGraph` | yes | no | no | Stores `BTreeMap<String, PluginManifest>` and gives topological order only. |
| `LivePluginPackageBinding` | yes | yes | `[u8;32] package_hash` | The desired authority seam exists, but production has no source that derives/passes these bindings. |
| `PluginHostTrustedArtifactCatalog` | yes | yes | package hash copied into `TrustedArtifactIdentity` | Validates exact requested identity only after its caller supplies trusted bindings and executable codecs. |

The reusable loader is therefore a useful **behavioral** seam, not a reusable trusted-catalog seam as-is. `WasmtimeNodeHost` recursively loads descriptor dependencies before registering routers and has a fixed `NODE_TURN_BUDGET` of fuel 10,000,000, deadline 5,000 ms, 256 effects, 1 MiB patch bytes, and 256 frames. However, its `new` method uses `GuestRuntimes::Owned(OwnedRuntime::new())`, not `WasmtimeRuntime`; it has no `OperationContext`/startup progress/cancellation, keeps its component map private, and cannot return the `PackageRef` values the catalog needs. The code must not imply a verified Wasmtime backend is already available for headless authority startup.

## Ordering, bounds, portability, and P2 seams

* `PluginGraph::register` validates the whole graph and `load_order` is deterministic/topological. The run host compiles a node, decodes its committed descriptor, recursively loads descriptor dependencies, then activates/registers that node. A trusted loader must instead verify the registry-to-byte identity **before** compiling and keep the verified binding through registration.
* Authority bounds are explicit: 16,384 operations, 64 MiB operation bytes, 64 MiB pair bytes. `OperationContext::checkpoint` checks cancellation/deadline; materialization and catalog loading emit bounded progress. `DbImmutableArtifactBlobStore` checks cancellation while its page-based reads/cleanup proceed and bounds diagnostics to 4 KiB.
* **P2-A1 integration blocker (high):** the authority admits a 64 MiB pair, but `DbImmutableArtifactBlobStore` rejects each blob over `496 * 1024` bytes. Two blobs make a practical pair ceiling of at most 992 KiB, not 64 MiB. Headless startup must not imply large artifact support. Choose either a bounded chunk-manifest CAS with ordered SHA-256 chunks, whole-blob/pair identity, full readback before publication, failure-safe cleanup/retention, and atomic publication; or redesign the durable payload ceiling coherently across every backend.
* The current P2-B seam is real: `CheckpointPublicationOrchestrator` stages and exactly reads back pack and SPR before `VerifiedCheckpointPublisher::publish`; `HubVerifiedCheckpointPublisher` delegates to `DirectoryService::publish_verified_artifact_checkpoint`. It is not instantiated by the hub binary.
* P2-C is still a seam, not a composed result. Directory WS handles `broadcast::RecvError::Lagged` with resync through `events_since`, but no authority startup/verified checkpoint path is connected to that replay loop. A rebootstrap cannot trust/replay a checkpoint catalog until the startup catalog and P2-B publication are real.
* No literal P2-A2 source marker was present in the focused hub/OS search. Its visible prerequisite is the above uncomposed loader/catalog/codec boundary; do not infer completed P2-A2 behavior from P2-B adapter tests.
* The repository has a launch entry `🛠️dev🗄️os-hub` in `.vscode/launch.json`, running `bun nx run os-hub:dev` with `OS_HUB_PORT` and `OS_HUB_DATA`. It launches the current DB/directory server, not a catalog loader. Default filesystem target paths and ignored generated registry outputs are not portable zero-touch deployment assets.

## Privacy and trust implications

* A trusted catalog must be hub-owned, immutable for the process lifetime, and constructed only from a selected/attested artifact set. Accepting arbitrary `LivePluginPackageBinding` values or target paths would let a caller declare a package identity for bytes that no registry attests.
* Never disclose raw artifact pair bytes to P2-B: the current port correctly gives the publisher only `ArtifactCheckpoint`; the DB adapter returns an opaque BLAKE3 locator while checkpoint integrity uses SHA-256. Preserve that separation in chunk manifests and diagnostics.
* Keep registry/provenance data and private storage locators out of directory/public event projections unless explicitly required. Publish canonical identities/digests, not component paths or pair content.
* The hub's default when `OS_HUB_ADMIN_TOKEN` is absent is loopback-admin and it binds `0.0.0.0`. That is acceptable only as an explicit dev default; a headless trusted authority deployment needs a configured admin token and deployment/network policy before catalog control or rebootstrap controls are exposed.

## Blockers

| Priority | Blocker | Evidence / required resolution |
|---|---|---|
| High | No production composition | `HubState` and `main` in `🌎️hub/📦️packages/🦀️rust/📦️bin.rs` contain no plugin host, graph, catalog, authority, orchestrator, or authority route. Compose only after the following blockers. |
| High | No verified source-to-byte-to-package binding | Registry hash data is ignored by resolver; loader creates BLAKE3 `PackageHash` and package id from plugin id; `CompiledHandle`/`PluginGraph` drop identity. Define and retain a dual-hash trusted record. |
| High | Catalog cannot load as a closure | 19 rows lack both descriptors and metadata; all catalog plugins ultimately need `stdio`, which lacks both. Only `process` component exists locally and its dependency closure is absent. |
| High | No executable native codec catalog | Authority requires native `ArtifactCodec`; headless startup registers none of 48 declared schemas. Decide and implement explicit vetted native codec providers; WASM metadata alone cannot manufacture native function pointers. |
| High | Pair/blob size contradiction | 64 MiB authority pair vs 496 KiB per immutable DB blob; resolve with chunk CAS or a coherent durable-limit change before P2-B/P2-C claims. |
| Medium | Descriptor trust incomplete | Binary descriptors are decoded but have no byte integrity check or component association. Add a descriptor byte digest/attestation and validate manifest/plugin/package/hash correspondence. |
| Medium | Backend/progress mismatch | `WasmtimeNodeHost` is actually backed by `OwnedRuntime` and has its own fixed turn budget; it does not surface startup cancellation/progress or verified package bindings. |
| Medium | Generated/target portability | The registry outputs and target WASMs are not committed deployment inputs. Package an immutable catalog bundle or deterministic build/install output; do not read developer-local `target/`. |
| Low | Launch registration | A hub launch config exists, but there is no headless catalog variant with vetted bundle path, limits, cancellation reporting, or production admin configuration. |

## Dependency-ordered implementation packet

1. **Freeze a schema-first catalog bundle.** Extend `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts` to emit a Rust-consumable, committed-or-packaged *trust record* per selected package: independent `pluginId` and `packageId`, role, version, topological dependencies, component relative path, component SHA-256, component BLAKE3/`PackageHash`, binary descriptor path and SHA-256, and descriptor-declared identity. Reject all 19 incomplete rows until they are actually described and built. Keep generated outputs derived; never hand-edit them.
2. **Build a bounded trusted loader before the authority.** Add a hub-owned loader module under `🌎️hub/🗿️artifact-authority/` and mount it from `🌎️hub/📦️packages/🦀️rust/🦀️.rs`. It should accept only the bundle, verify path containment and every digest before compile/decode, calculate both hashes from the one byte buffer, validate descriptor-to-record identity, topologically load the selected closure, and retain `pluginId -> PackageRef`/`LivePluginPackageBinding` without going through a lossy `CompiledHandle` or `PluginGraph` lookup.
3. **Make native codec availability explicit.** In the same loader, take a finite native codec-provider registry built into the hub (with corresponding explicit package dependencies), atomically preflight/register its codecs, and reject every manifest kind missing a non-zero `pack_schema_hash`. Do not claim that plugin WASM or static JSON itself provides native `ArtifactCodec` functions.
4. **Close P2-A1 storage semantics.** Extend `🌎️hub/🗿️artifact-authority/🔌️adapters/🦀️.rs` with either the bounded chunk-manifest immutable store described above or a reviewed common payload limit. Test chunk ordering, duplicate content, per-chunk/whole SHA-256, readback corruption, cancellation, deadline, orphan cleanup, and retention. The authority's advertised pair limit must equal what this adapter can publish.
5. **Compose once, at hub startup.** In `🌎️hub/📦️packages/🦀️rust/📦️bin.rs`, load the bundle before opening routes; expose catalog/authority/orchestrator in `HubState`; compose `PluginHostTrustedArtifactCatalog`, `ValidatingCanonicalArtifactAuthority`, `DbImmutableArtifactBlobStore`, and `HubVerifiedCheckpointPublisher`. Fail startup atomically with bounded diagnostics/progress if validation fails. Add configuration for bundle location, selected profile, limits, and a required production admin token.
6. **Only then connect P2-B/P2-C.** Route materialization through the composed authority and verified publisher. Add a rebootstrap flow in the same hub binary/directory integration that replays verified checkpoints after a lag event, proves catalog/package identity still matches, and fails closed on unavailable/mismatched catalog entries.
7. **Register launch/test entry points.** Update the existing ordered `.vscode/launch.json` generation source (not the generated launch file) with a dedicated headless trusted-catalog config that supplies an explicit bundle and non-dev admin token. Retain `🛠️dev🗄️os-hub` as the ordinary server launch.

Required language-agnostic acceptance tests:

* 59-row registry census rejection when any selected row lacks every declared component, descriptor, or both expected hashes; compare SHA-256 values with a third-party implementation in test scope.
* A two-node real dependency fixture proves dependency-first load, exact byte/package binding retention through the catalog, and rejection after one-bit WASM, descriptor, package-id, or codec-schema mutation.
* Headless `os-hub` process test proves startup failure leaves no authority route/state on a missing closure, and proves a valid minimal vetted bundle starts with a non-empty catalog and observable progress/cancellation.
* Codec coverage test establishes every selected manifest kind has exactly one executable codec and rejects zero/mismatched pack schema hashes.
* P2-A1 chunk test exercises a pair over 496 KiB and up to the advertised cap, corrupts every stage, and proves no P2-B call before all whole-object readbacks pass.
* P2-B/P2-C integration test forces a lagged subscription, reboots/rehydrates it from verified checkpoints, and rejects a rebootstrap under a changed component/descriptor identity.

Suggested commands after implementation (none were claimed as passing in this audit):

```sh
bun nx run @semio-tech/plugin-registry:check
bun nx run os-hub:test
bun nx run os-hub:build
```

The final operational test must start the launch configuration with a packaged catalog, not developer-local `target/wasm32-wasip2`, then emit a deliberately identifiable startup progress record and verify an authority checkpoint/rebootstrap round trip.

