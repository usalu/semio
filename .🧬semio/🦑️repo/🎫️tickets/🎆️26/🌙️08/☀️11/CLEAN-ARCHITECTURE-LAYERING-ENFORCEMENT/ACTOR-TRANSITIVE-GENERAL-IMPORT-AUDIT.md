# Actor Reachable General Import Audit

Static TypeScript AST traversal and module resolution only. The compiler library parsed files; no repository module was imported, evaluated, or invoked. No tests, factories, Source capture or native processes. Starts only at current Actor Rust script and follows its literal import/export/dynamic-import/import-type references through General source; stops at first Specific/S/Hub edge. This is not a whole-repository census.

Reachable General source files: 42. First outbound edges: 0. Unresolved relative bindings: 0.

## First Outbound Edges

No literal resolved General-to-Specific Repo/S/Hub edge found in this bounded reachable graph. This does not prove deletion resilience against computed imports, child executable strings, source/catalog data dependencies or runtime discovery. Specific compiler hosting imports into General remain legitimate and were not traversed from the reverse direction.

## Observed File Hashes

- `🧰️framework/🔨️modules/🎭️actor/📦️packages/🦀️rust/📜️script.ts`: `18bb6b74561fe8c0911fa1e2ef4a403b7938e50ed01f8f5efd98aa4813e9f3ec`.
- `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️execution/🟦️.ts`: `a1ffd4c94a562975b07b7cf7638bd388673ab89758a5c7bc98380cc598d398a7`.
- `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts`: `9610713ed696f3ccfc968f21afa3f7df57932a65af9b7c53a12210a5a41189fa`.
- `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts`: `20052ef531cc4060eb1e22b3563ee7ef373d8abc60c0c5f591eb1109204ea759`.
- `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts`: `95b774cc4dd2f563aaf11f670bc4ac340ea0ff57f931459dc250e8bdef6e1ea3`.
- `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🕸️wasm-build/🟦️.ts`: `a9f1d197ff0b050ca20ae284b6ad0b7045c27165ba6e020c9ec71201121e5acb`.
- `🧰️framework/🔨️modules/🎭️actor/🧬️typegen/🏃️execution/🟦️.ts`: `06c547b7d3b0c66ed0588fe671139bcb676ec5781db0aa4a6a1e17d69eecd65d`.
- `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts`: `8c9925f74e43903b15a1a20485be1d90e231264bc5d9204944e86851de54f55b`.
- `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts`: `57c6c0148565d4cf7a59deed5ed90e51ddf90c3f420347d89f3f0c98458bd98f`.
- `🧰️framework/🔨️modules/🏃️process/🧪️testing/🦀️cargo/🏃️execution/🟦️.ts`: `e6006fda02353d74ac97a979736f7257feaef00612f999f93bbb9992dde13e9a`.
- `🧰️framework/🔨️modules/🏃️process/⏱️budget/🟦️.ts`: `01afaba090403b6de22715d69f82d303ef665f6ba2bd633e4eef5ffdb1fc2c37`.
- `🧰️framework/🔨️modules/🏃️process/🏗️build/🎟️capacity/🟦️.ts`: `9bd0b086ec2cdbbffab4be3bd2585df000cc702420b5d7aabbb6c17fc029189b`.
- `🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts`: `90036483e5d8217450e21b8a40cfcf14a3d3384e6c5ade4808ea87af2e2274d1`.
- `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🟦️.ts`: `3c161d5872c8fca2390dc5723a1a437d2b7098ef0f718a4c83fc37caef78dd23`.
- `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts`: `66d001007c289158aeb927fa9ded6c782e33db14f461b10301b53967684f958a`.
- `🧰️framework/🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts`: `06200b3f26dcea04f4d16c00fd2a4a4d89366918e4c23c597cf90eaafef57446`.
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🕸️wasm-build/🟦️.ts`: `a2dd2c5e70df2d33878e8eca0dc40ba75d26737e69052f0dea14a7a5b5a977b4`.
- `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🗂️files/🟦️.ts`: `745d81e573b4e57c9bf90cbc93c5a0405f32d34485992f9eae2c2e6d38bbff3f`.
- `🧰️framework/🔨️modules/🎭️actor/🧬️typegen/📋️plan/🟦️.ts`: `0c842efcfe86c857047397ea0f8e5ccea5f23d7d3509151e80e5160727f1dd95`.
- `🧰️framework/🔨️modules/🎭️actor/🧬️typegen/📤️publication/🟦️.ts`: `e218123ecdf2381957a4822807dfb40ae26153b0b668004c3f22845054eb6639`.
- `🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts`: `023795fe4cc46680b4b41c6b14b1f6602fb03697294cebdc7c8c243638e5cd2a`.
- `🧰️framework/🔨️modules/🏃️process/📥️capture/🟦️.ts`: `89e2f612fdb91c721251d0bf3d1697f0a0384eda4db4ea9eaea2c3c4bbb5f8eb`.
- `🧰️framework/🔨️modules/🏃️process/🪓️termination/🟦️.ts`: `3df81b9a89a5c5e56bb37ecf579c0684dca29beacc50035fa6bfcfe75cb9c7ff`.
- `🧰️framework/🔨️modules/🏃️process/🏗️build/🎚️mode/🟦️.ts`: `9a7e89613025e2ee3d3033bc525be0ae109241cbde4e420db1cad72d49ea06d0`.
- `🧰️framework/🔨️modules/🧬️schema/✅️validator/🟦️.ts`: `4dc0d373fdad671f44e657d4b49c5febcaca06a320125d2049da59ac71a9518f`.
- `🧰️framework/🔨️modules/🏃️process/🔒️leases/🟦️.ts`: `9c30f9d5bb1ff1c0914dee648d0de3a259c68a173d709304dd424d89417202e5`.
- `🧰️framework/🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts`: `22f68a1cdabdcdb7d2f6d1fad86341816ab5d0e9c79ea21fcfa0c1dd10da8aec`.
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🔒️lease/🟦️.ts`: `3d8dd3b1605c9f9a200367dad95160fac74c869a340e4a1e594ad23c73630985`.
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/📤️publication/🟦️.ts`: `32232c0772670d2648f626b37928565518ef0bc184a367c738d394e7df19c7a9`.
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🗂️files/🟦️.ts`: `6364b9ef7ac45fed824f785f5b70b995f9fb739e52af30f597da267eaa8633d0`.
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🎟️admission/🟦️.ts`: `2398b2c3f98a961db0a54d53106d0c620e877b03ddfb04c612d4370ef6fc673a`.
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🟦️.ts`: `5123e07cc873d3b5f242c91aadcd2daaa69b8a50fe74c918c4d190d53e4893df`.
- `🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/📪️outcome/🟦️.ts`: `642be5f8b3f90e9e26ece71d4eca3ed7f3ac7868e942b567519fdade8c3ba66a`.
- `🧰️framework/🔨️modules/🏃️process/🔒️leases/📦️metadata/🟦️.ts`: `b1e61de4a62b575ed3aabb0985775af4ce6b5e929f42ee0712a738c9b61541a5`.
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🔒️lease/🎯️target/🟦️.ts`: `4df04055cf589a30d7e882af7a7c81bc24049825ecc48f9e40459fb298cb2819`.
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🕸️units/📦️package/🌱️environment/📖️references/🟦️.ts`: `de1316b111827afddfb7f3c9d7c3beb2db6d8b41c2bfb0c6cb8d48fd78af6888`.
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts`: `ec72aacb5c390a8578b1278401006763cffad2e02e076d7895831751f1b91559`.
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🕸️units/🟦️.ts`: `7b3c891a8b183ad99e35aef2b3131ebbc3a254861357a2edc3e558c79877d2c1`.
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🕸️units/📥️capture/🟦️.ts`: `df431d45793ff49809cd0d71b89a9f310af9301cf218f26152b8b01b66f1dda6`.
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🎟️admission/📖️read/🟦️.ts`: `9339f4729abacb40857899ed48d241f36176aedcf12846fab0a38902fdc42493`.
- `🧰️framework/🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts`: `c2735ae235567482268051b00dfc54d772258db08c2a95117ee4bd0e8a8eb941`.
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🕸️units/📦️package/🌱️environment/🟦️.ts`: `3ae45293a1255a16fda408466f43f351f1928ee7357594d023b2282a7870c6e1`.

## Unresolved Relative Bindings

[]
