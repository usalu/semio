# Independent Selected Component Mode Source Audit

2026-10-16. Read-only audit; only this Markdown report was written. Root ticket lifecycle remains with parent. No tests, compilers, Rust/Native processes, cache/store operations, browser, GPU, services, leases, network, publication, or physical operations were executed. Read root and applicable Products/OS AGENTS.

## Actionable Finding

P2 — Update the existing trusted bundle Source assertion for the explicit Rust selected-profile argument. `🌎️hub/🧪️tests/🎁️trusted-stdio-gis-bundle-check/🟦️.ts:86` requires the exact old string `record.receipt.binds(&record.plugin_id, &record.package_id, &record.component.sha256, &record.descriptor.sha256)`. The actual private trusted-catalog owner `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧾️descriptor/🦀️.rs:1059` now supplies `directory::os_component_receipt::ComponentProfileV1::Release` first. Its required old substring is absent. Therefore that Source-check branch rejects the corrected owner when reached. This is a static contradiction, not a claim that the check was executed. Update the assertion to the actual explicit-release signature; do not add compatibility API.

## Source Conclusions And Limits

The browser actual publication owners compare receipt.profile to genuine Original mode at lines145/157. Actual model reader `📦️model/👓️reading/🟦️.ts:19` enforces intent.mode equality to authority.original.buildMode before products-store entry at20; receipt checking at22 binds the selected intent. Native entrypoint model verifier :31 binds the genuine mode passed from actual Original. Producer model :78 uses declared.intent.mode, admitted against Original at45.

Receipt TypeScript schema owner :35 requires profile in ComponentReceiptBindingV1; :93–94 binds profile plus all four identity/content fields. Neutral JSON $defs.ComponentReceiptBindingV1 :144 has additionalProperties:false, profile required, and uses the closed two-mode definition. Rust schema :129 takes typed profile and compares it. Shared twelve binding vectors cover both matching profiles, both cross-profile refusals, and four field mutations for each profile. TypeScript binding test reads that exact JSON and uses Ajv plus SQLite; Rust unit :90–106 includes the same JSON and applies binds. Rust laws are authored only, uncompiled/unexecuted here and in the cited closure.

Actual release catalog, bootstrap, actor, document-open and open-plan calls inspected explicitly select release, including the three Rust consumer calls (catalog1059, directory1145, ProgramBridge1780). Facade imports drifted in concurrent work, while componentReceiptBindsV1 remains reexported from the same original private receipt schema owner; current consumer incidence still names selected mode or explicit release. ProgramBridge release-only admission remains a limitation, and physical/default publication remains HOLD. No compatibility or shipped-only restriction found in selected-model binder itself.

The browser mode tests extract actual predicates with TypeScript AST and evaluate them as DATA, including the Original comparison. They do not execute the browser/model owner, Rust, Native provider, or runtime custody path. Retained green4 logs support historical BOTH9/186 only at the frozen epoch; strict logs are empty and cannot independently prove exit0, which is terminal provenance in the parent closure. No current epoch test claim is made.

## Hash Verification

All26 cited source paths and all18 retained log paths were read/hash checked. Before inspection19 source paths matched; seven drifted (including concurrent ticket wrapper). The table below is the authoritative final observation. All18 retained logs match cited byte counts and SHA256. Parent is editing shared sources; these final hashes do not requalify their changed epoch. Core predicate/binding/corpus owners stayed at cited hashes across the audit. Final re-read follows; changed facade paths preserve the original private owner incidence described above.

| Path | Bytes | Observed SHA256 | Frozen comparison |
|---|---:|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🚀️commands/🟦️.ts` | 20534 | `80145f3c4ab770a1be750fc0f72886911d94cd3a5dcff2d4efab93ea17bd91bf` | matches |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️model/🟦️.ts` | 18165 | `045f149b3dd099cbfab7d94ac8128d515b6db2b1c3ac82f4a0d626c22f98414d` | matches |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️model/👓️reading/🟦️.ts` | 5283 | `b944630986ea0a6cad657026fe56a04797bb85ca26556771edd92d206b6f0332` | matches |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/📦️modules/🟦️.ts` | 10778 | `ec240ec1ebed2adce3f3e61923b81b937b2c8b57086d2bf8592836cba5d07a24` | matches |
| `🌎️hub/🏗️bootstrap/🧾️provenance/🗃️bootstrap-bundle/🟦️.ts` | 41704 | `37ddb576185f6b8761f65b50ab9a811b84a92e29215641fc543bcc4bde82ba21` | drift |
| `🌎️hub/🏗️bootstrap/🎭️actors/🟦️.ts` | 26417 | `8aa6a5596bdda82dc890b22ae245a88768944a23c1fdf4d7ff69d78e5a7ff383` | drift |
| `🌎️hub/🧪️tests/🏛️trusted-catalog-fixtures/🟦️.ts` | 82999 | `5839d89b4daeb9f363c030662ecdc8fa9136a184e241f5c4c38d20fceb911ffa` | drift |
| `🌎️hub/🧪️tests/🗺️open-plan-checks/🟦️.ts` | 41743 | `cb70c54046e6bdb2803b04a0652b770171bdb6fa67a45fa33d25c300144c0711` | drift |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🔏️document-open/🟦️.ts` | 25785 | `11f062e81173a64bdf157238e820b93887c83aae638e3bf38501d84ae3b8d9e8` | drift |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/✅️catalog-verification/🟦️.ts` | 64609 | `741c432f87a63de306c7b413b73191281d9ce3b9131955b8a764f37c46fa54b9` | matches |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏷️component-receipt/🧪️tests/🎚️profile/🟦️.ts` | 4155 | `61a281f53e53cd9909db338b728fce447fd73d26ee44d589847986875fe7aaa8` | matches |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏷️component-receipt/🧬️schema/🟦️.ts` | 7225 | `6a68a9241cfda96adbd8a820445df882540d987480478167f48e1e485b51d656` | matches |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏷️component-receipt/🧬️schema/🦀️.rs` | 8441 | `5a17f8575e158e04773abbdaea402858e7019aaa31230042bfd096b221de0417` | matches |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏷️component-receipt/🧬️schema/🔣️.json` | 5574 | `3eb0572b1083e9412b33916b680abb56d77e10abc1519392f50d9ce278983ecc` | matches |
| `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧾️descriptor/🦀️.rs` | 67217 | `7273c6d65b3f115fbcd027a5e2400c634a4a39581d9e37cdb520ec166fb54d03` | matches |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs` | 116726 | `13e23e0874930dd78f038b8ddb824b302f5a3e2bfe8b027888b800750fa75e2e` | drift |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs` | 82202 | `0237d163ab9cc5afa851679a7642d081f438f0fc2a80386a9355ccfd9620cbe6` | matches |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏷️component-receipt/🧬️schema/🧪️tests/🔬️unit/🦀️.rs` | 7381 | `e8fb993233fac41bd2464e5033fc9b93f7cb8a83f40f2594a389519f5d58f47b` | matches |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🚀️commands/🧪️tests/🎚️mode/🟦️.ts` | 6715 | `dcd7dbf70d126518dcde63ca0a1fb7c1a6b919ed5feb269707808351887c3426` | matches |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🚀️commands/🧪️tests/🎚️mode/🔣️.json` | 1963 | `3c059f5f2a116e09ad3ce5f4837a130c081d90fde905392f9dea5290076753b8` | matches |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🚀️commands/🧪️tests/🎚️mode/🧬️schema/🔣️.json` | 1744 | `efd0467cada32d9e13232176f86487b97a10bbd5df12c77a626a864e4380b775` | matches |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏷️component-receipt/🧪️tests/🔗️binding/🟦️.ts` | 2544 | `6e3c986d215b827c273472a4321bd219b238241adfcc538eb2af263a2a93beea` | matches |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏷️component-receipt/🧪️tests/🔗️binding/🔣️.json` | 1532 | `15500d5e165874391b125e5149a43d215f303c16023108ba035859c29698a0b6` | matches |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏷️component-receipt/🧪️tests/🔗️binding/🧬️schema/🔣️.json` | 1133 | `f271d5dc2f9e04bd22a2d6cbd5928266d3ab8fceb0233bad5f0ca03478c8701a` | matches |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/build-mode/📜️script.ts` | 17778 | `61314446666465f896a6d9053a2a6dad40a5e627ab5fcdfd0fe962a1eb4da79b` | matches |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/typescript-artifact/📜️script.ts` | 80312 | `98906e5f78714bff0c10e0bd7d7290bdea4e462a4168a174524f17b35b0024a9` | drift |

## Retained Logs

- `component-browser-mode-green1-prototype.log`: 1340 bytes; hash matches
   0 fail
- `component-browser-mode-green1-release.log`: 1340 bytes; hash matches
   0 fail
- `component-browser-mode-green2-prototype.log`: 5970 bytes; hash matches
   0 fail
   0 fail
   0 fail
- `component-browser-mode-green2-release.log`: 5971 bytes; hash matches
   0 fail
   0 fail
   0 fail
- `component-browser-mode-green3-prototype.log`: 5942 bytes; hash matches
   0 fail
   0 fail
   0 fail
- `component-browser-mode-green3-release.log`: 5942 bytes; hash matches
   0 fail
   0 fail
   0 fail
- `component-browser-mode-green4-prototype.log`: 5014 bytes; hash matches
   9 pass
   0 fail
   186 expect() calls
- `component-browser-mode-green4-release.log`: 5013 bytes; hash matches
   9 pass
   0 fail
   186 expect() calls
- `component-browser-mode-red1-prototype.log`: 6733 bytes; hash matches
- `component-browser-mode-red2-prototype.log`: 7878 bytes; hash matches
- `component-browser-mode-red3-prototype.log`: 8127 bytes; hash matches
- `component-browser-mode-red4-prototype.log`: 11801 bytes; hash matches
- `component-browser-mode-red5-prototype.log`: 11427 bytes; hash matches
   0 fail
   0 fail
- `component-browser-mode-types1-prototype.log`: 0 bytes; hash matches
- `component-browser-mode-types2-prototype.log`: 4675 bytes; hash matches
- `component-browser-mode-types3-prototype.log`: 5084 bytes; hash matches
- `component-browser-mode-types4-prototype.log`: 0 bytes; hash matches
- `component-browser-mode-types4-release.log`: 0 bytes; hash matches
