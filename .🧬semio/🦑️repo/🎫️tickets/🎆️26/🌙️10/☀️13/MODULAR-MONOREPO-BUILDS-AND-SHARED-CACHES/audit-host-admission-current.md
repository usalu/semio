# Host And Admission Source Audit

Read-only Source audit on 2026-10-16. Only this retained Markdown report was authored. No tests, builds, child processes, Native providers, Hub effects, git mutations or platform probes were run. Existing logs were hashed, not regenerated. Root AGENTS.md was read; no more-specific AGENTS applies to these Source directories.

## Findings

1. **P2 — Start accounting can retain a nonexistent run on path refusal.** `🌎️hub/🚀️local-bootstrap/🏃️execution/🟦️.ts:444–446` increments `run-start` synchronously, then the accepted asynchronous callback computes `runRoot` and throws on its path bound before entering its inner try. The outer try catches synchronous `ledger.run` refusal only; it cannot catch that rejected promise. Consequently `run-close` at455 is skipped. Host final `close` at417 then additionally fails its zero-runs invariant even though no child was born. Move all accepted prebirth work inside the rollback try, including random buffers/path validation, retaining exact Error/undefined. This is source reasoning, not a runtime reproduction.

2. **P2 — Owner closure drains operations before asking live runs to stop.** In the same file414–417 the Host invokes `ledger.drain()` before leaving its issued control and only afterward calls `finishLocalHub` for retained runs. A body that returns while an accepted `run.reader.read()` remains pending therefore cannot reach the close request that ends its pipe until that read times out/fails independently. This contradicts the stated stop-all-then-join owner protocol and can turn ordinary scope exit into deadline failure. Stop host admission, initiate actual run closure, then jointly drain accepted operations and run terminals while retained physical owners remain held. This is source reasoning; no real Hub qualification is claimed.

## Admission And Projection Review

No additional actionable defect was established in canonical admission lines47–54 or projection119–126 within this bounded audit. `track` closes accepting on owner and ancestors on rejection while preserving live authentication for previously accepted drains. Failure arrays distinguish thrown undefined by length, and `settle` separately tracks callback failure presence. Projection walks actual private ControlScope ancestry, clamps ceiling/deadline and retains projected deadlines in WeakMap; capture overrides caller wire immediately at owned spawn. Neither projection helper nor the Host ledger alone issues Native storage authority.

Host private WeakMap factories acquire actual published/captured producer scope plus Diagnostics scratch before minting handles. Candidate construction uses a genuine parent Host or captured Cargo token. No path-only public registrar was found in reviewed constructors. These Source facts do not qualify positive Native image/compiler/cache/runtime ownership.

## Epoch Accounting

Initial and final file reads matched the same values during this bounded audit. The final Hub frozen table has30 Source entries and9 evidence entries: all matched. Earlier prerequisite entries deliberately differ for capture/input/wrapper successors; they are separately retained below and must not be counted as drift in the final30 Source cut. All19 failed-admission entries matched initially and finally. Projection's original routing hash and wrapper hash are superseded by the failed-admission epoch; its other13 entries matched. The current capture remained `0012cf2f899ec87eba9c83f6610ca03bbdcd476a02a6b62e1ca419122c7f9397` during this audit. Root's announced interactive successor is not validated by this report. CPU may subsequently edit cleanup128 onward; that does not retroactively validate a whole-file successor or invalidate the isolated reviewed hunks.

Current canonical routing reviewed hunk hashes (UTF8, LF, final LF):

- Lines 47–54: SHA256 `2ed1fe169a2bb5634bd272c03d25e677b0ef3c0758f1de2c07e5fef2270b2682`.
- Lines 119–126: SHA256 `e47adceaeeb70168523a75ff088f67e1677d1f2d25517fb455553e8b136a618d`.

## hub-e2e-actual-host-current.md

| Path | Report SHA256 | Final Observed SHA256 | Result |
| --- | --- | --- | --- |
| `🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/📥️capture/🟦️.ts` | `ad1797ae05e9502df0eab2e1f513864ce74fbcdd05c9ebd2d99723b25b2695dd` | `0012cf2f899ec87eba9c83f6610ca03bbdcd476a02a6b62e1ca419122c7f9397` | Successor |
| `🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts` | `bb94fd5bed144b44d50157bff2813524cad6115d7ed9902d63e24ce56a034d65` | `bb94fd5bed144b44d50157bff2813524cad6115d7ed9902d63e24ce56a034d65` | Match |
| `🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/📥️capture/📥️input/🟦️.ts` | `aed03314d45a86b5c64dcabd793bf90c74878ecfa999be1bcd5ebd8f1089a780` | `fa1aa011502a4f3bcd1d669d47e624d3495e2c4a4bd2a0e53a3cf2468d229e13` | Successor |
| `🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/📥️capture/📥️input/🧬️schema/🔣️.json` | `bc5ece39482f949d333f1c91a7d4f5ece7724a10c26a34e44e5bbb4e1a0d8c91` | `25b51ff632f110a78626c329800896226a48c0f4baea2fbb47580b4af28cab83` | Successor |
| `🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/📥️capture/🧪️tests/🟦️.ts` | `1dd4bdd310f2f1411933c2dfb33dc301005631cb8c437f85934b8aea4c2f1797` | `1dd4bdd310f2f1411933c2dfb33dc301005631cb8c437f85934b8aea4c2f1797` | Match |
| `🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/📥️capture/🧪️tests/🔣️.json` | `1362a12313db0d83261a7ed0e33031ec9ba83bb2903488c635d5cd6468b25d05` | `1362a12313db0d83261a7ed0e33031ec9ba83bb2903488c635d5cd6468b25d05` | Match |
| `🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/📥️capture/🧪️tests/📥️input/🟦️.ts` | `0261a26ca910a1a8c24ef01f8c350624394f3c6ddb4fa18f9e19f5276bb7ee2e` | `e2c5e8b78682d0d5451124fe52c0a9e6b75ab94199b6189e2b1d117a664cb58c` | Successor |
| `🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/📥️capture/🧪️tests/📥️input/🔣️.json` | `bdadec20f06463c3811b14295e758de6410e2c01ca430aaabf7943aaaac0aae1` | `8f6e4ac3957f760340aea199c7af01d74ee96a45654bce8aa9edf369494a4957` | Successor |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/typescript-artifact/📜️script.ts` | `f64415539d0b1ccb7f3d24c15d29f83b448ca3432a1b9c5afec98bc9f0a5e3ef` | `572de3b5fc022fe82f75953b2b7ef169a17a4c029060df0fb93c17eeba124934` | Successor |
| `hub-owned-child-final-prototype.log` | `865a4758c631af8658d3cf6319d5a1cc501305cc60f1166f7641b045d43a119c` | `865a4758c631af8658d3cf6319d5a1cc501305cc60f1166f7641b045d43a119c` | Match |
| `hub-owned-child-final-release.log` | `8828a332d4a001e7930da256d26bc6c4c2b5abd5db1b4c71534634d1bc064c94` | `8828a332d4a001e7930da256d26bc6c4c2b5abd5db1b4c71534634d1bc064c94` | Match |
| `hub-owned-child-final-types-prototype.log` | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` | Match |
| `hub-owned-child-final-types-release.log` | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` | Match |
| `hub-owned-child-prototype1.log` | `67f18c7ac43dfdeee2702214bf83d5d368b7e9ffac04da7ea9dcf693bbfa6eb6` | `67f18c7ac43dfdeee2702214bf83d5d368b7e9ffac04da7ea9dcf693bbfa6eb6` | Match |
| `hub-owned-child-prototype2.log` | `fa518014f9776d581aecb8e779c1214b03de49668fd4ec02f848a38baa60d0ab` | `fa518014f9776d581aecb8e779c1214b03de49668fd4ec02f848a38baa60d0ab` | Match |
| `hub-owned-child-prototype3.log` | `663f029e8a42efa7117d8fd7a76c66486859dfea921e494363daa62e3aaef5d0` | `663f029e8a42efa7117d8fd7a76c66486859dfea921e494363daa62e3aaef5d0` | Match |
| `hub-owned-child-prototype4.log` | `141ae02be764fa377326a0eb138b6974905106021e0d2991b8e1d0ba59acabc9` | `141ae02be764fa377326a0eb138b6974905106021e0d2991b8e1d0ba59acabc9` | Match |
| `hub-owned-child-prototype5.log` | `650b7e22a4b5a0e3a9110c992d9b6640a13379bc9b483c9030036b90cdf42ff0` | `650b7e22a4b5a0e3a9110c992d9b6640a13379bc9b483c9030036b90cdf42ff0` | Match |
| `hub-owned-child-prototype6.log` | `9b1aee0a2ab49b9c8687fd62d4167c89e625e401fd7fad3e7fe4ef78009df7cf` | `9b1aee0a2ab49b9c8687fd62d4167c89e625e401fd7fad3e7fe4ef78009df7cf` | Match |
| `hub-owned-child-red.log` | `177db6308bdcc87dd9ddc2a1a8c8c487b136cdb5890932f9362f05ca89ab562c` | `177db6308bdcc87dd9ddc2a1a8c8c487b136cdb5890932f9362f05ca89ab562c` | Match |
| `hub-owned-child-red2.log` | `a20149791709f0cc8895602d652de0fbf4846a30c0867f3144f4e5550b2b16fa` | `a20149791709f0cc8895602d652de0fbf4846a30c0867f3144f4e5550b2b16fa` | Match |
| `hub-owned-child-types-prototype1.log` | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` | Match |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/typescript-artifact/📜️script.ts` | `2676e62509fa44ce59c0f6e64d859bc6aaa4521055e90368cbd9f79ec693e03a` | `572de3b5fc022fe82f75953b2b7ef169a17a4c029060df0fb93c17eeba124934` | Successor |
| `🌎️hub/📦️packages/🟦️typescript/📜️script.ts` | `820a72a992233f8f42c5a3318e5488415874b85a0486a6f1bb3f8d12d252ff71` | `820a72a992233f8f42c5a3318e5488415874b85a0486a6f1bb3f8d12d252ff71` | Match |
| `🌎️hub/🔐️auth/🧪️tests/🤝️live-sign-in/🟦️.ts` | `20129aa76917c0127edba35512a4e9e8f36244fa5f5ccc3135ad456f4c99813f` | `20129aa76917c0127edba35512a4e9e8f36244fa5f5ccc3135ad456f4c99813f` | Match |
| `🌎️hub/🚀️local-bootstrap/🎛️host/🟦️.ts` | `a22b960a949c9a61d108fde9c1b030d890858b6f260573437cc108a7d8f11350` | `a22b960a949c9a61d108fde9c1b030d890858b6f260573437cc108a7d8f11350` | Match |
| `🌎️hub/🚀️local-bootstrap/🎛️host/🧪️tests/🔣️.json` | `257f222c663c742c329e9702aa8dd23a9c8455d754d08f2bc1fdc24d02e771a0` | `257f222c663c742c329e9702aa8dd23a9c8455d754d08f2bc1fdc24d02e771a0` | Match |
| `🌎️hub/🚀️local-bootstrap/🎛️host/🧪️tests/🟦️.ts` | `9d446dcb2702146f8d45ea28107709f466293454a1e75f82e16fc080abd00c28` | `9d446dcb2702146f8d45ea28107709f466293454a1e75f82e16fc080abd00c28` | Match |
| `🌎️hub/🚀️local-bootstrap/🎛️host/🧬️schema/🔣️.json` | `67df83f8c69578b91a61dc091a2df10d5c0f2fa094183f8fbcf1089538841ec6` | `67df83f8c69578b91a61dc091a2df10d5c0f2fa094183f8fbcf1089538841ec6` | Match |
| `🌎️hub/🚀️local-bootstrap/🏃️execution/🟦️.ts` | `b2fd65678554a17adb6d185df8f06e9c0352b265a7f630e0d7c2a092625a7055` | `b2fd65678554a17adb6d185df8f06e9c0352b265a7f630e0d7c2a092625a7055` | Match |
| `🌎️hub/🚀️local-bootstrap/👷️dev-owner/🟦️.ts` | `c34e14cc54876e841f43991093c3b0df3ac9fb5dbb991408a527610c9d554339` | `c34e14cc54876e841f43991093c3b0df3ac9fb5dbb991408a527610c9d554339` | Match |
| `🌎️hub/🚀️local-bootstrap/📡️framing/🟦️.ts` | `f98f07dea3961d228ec8b2def57ffbf133420feeeca7c5237558836552b203d5` | `f98f07dea3961d228ec8b2def57ffbf133420feeeca7c5237558836552b203d5` | Match |
| `🌎️hub/🚀️local-bootstrap/🔐️credential-issuance/🟦️.ts` | `8a7de5a801050a9803ab598a2640fead6704095acb6ce3fa1378f2ac780c3189` | `8a7de5a801050a9803ab598a2640fead6704095acb6ce3fa1378f2ac780c3189` | Match |
| `🌎️hub/🧪️tests/🎚️config/🟦️.ts` | `643246cdc58d8ad03edd409959acdf86865e3967288198ce0c41ad7b4c7c860a` | `643246cdc58d8ad03edd409959acdf86865e3967288198ce0c41ad7b4c7c860a` | Match |
| `🌎️hub/🧪️tests/🎛️run/🟦️.ts` | `4bb69f02fc685826fc7335994c82ef61a5a092fb6b74957e01767523d59014eb` | `4bb69f02fc685826fc7335994c82ef61a5a092fb6b74957e01767523d59014eb` | Match |
| `🌎️hub/🧪️tests/🎛️run/🧬️schema/🔣️.json` | `dec39f7b89694c21d27c10ec8fc06033e988fd9b4454a8be6e125a47016ab57e` | `dec39f7b89694c21d27c10ec8fc06033e988fd9b4454a8be6e125a47016ab57e` | Match |
| `🌎️hub/🧪️tests/💾️backup-restore/🎛️request/🟦️.ts` | `e388a78650a526f7e951ae0b59c7f1ca1b773c708d021366edd7cbf9c24c9595` | `e388a78650a526f7e951ae0b59c7f1ca1b773c708d021366edd7cbf9c24c9595` | Match |
| `🌎️hub/🧪️tests/💾️backup-restore/🟦️.ts` | `130d16cf5ba984362d6b9b529c4774e3cdadba71d76b3425b020332f78acd561` | `130d16cf5ba984362d6b9b529c4774e3cdadba71d76b3425b020332f78acd561` | Match |
| `🌎️hub/🧪️tests/💾️backup-restore/🧬️schema/🔣️.json` | `ac84f29150186f07c436d2b17793bfd54ccc1dbd2e3c9273da78b10a793133ef` | `ac84f29150186f07c436d2b17793bfd54ccc1dbd2e3c9273da78b10a793133ef` | Match |
| `🌎️hub/🧪️tests/📈️document-growth/🟦️.ts` | `69076594fa4251f43ce073c85233ebd68c94931eeec2168db0ff83c7b0642711` | `69076594fa4251f43ce073c85233ebd68c94931eeec2168db0ff83c7b0642711` | Match |
| `🌎️hub/🧪️tests/📺️renderer/🧊️wgpu/📜️script.ts` | `99ed86c9b28db8f09ffa730cec03908fa1d3a0c45023e6fcabcb38535830eec8` | `99ed86c9b28db8f09ffa730cec03908fa1d3a0c45023e6fcabcb38535830eec8` | Match |
| `🌎️hub/🧪️tests/🤝️dev-collaboration/🟦️.ts` | `c7fae733d2a234a20346754ce8ef8c5a3c6ce39e462684f815dab9dd0b4c8e7f` | `c7fae733d2a234a20346754ce8ef8c5a3c6ce39e462684f815dab9dd0b4c8e7f` | Match |
| `🌎️hub/🧪️tests/🤝️two-client-document/🟦️.ts` | `32d8f4505f2974ac1d7f471e9d9ea0ed32cba608d328881ddb3d0fbb63031b97` | `32d8f4505f2974ac1d7f471e9d9ea0ed32cba608d328881ddb3d0fbb63031b97` | Match |
| `🌎️hub/🧪️tests/🧫️fixtures/🧱️foundation-source/🔣️.json` | `da664727efb9cbebd3aed0255a7d45745a9ab6d2439737e33f81b90428a72d5d` | `da664727efb9cbebd3aed0255a7d45745a9ab6d2439737e33f81b90428a72d5d` | Match |
| `🌎️hub/🧪️tests/🧱️foundation-source/🟦️.ts` | `3a73ab6f1c84fe5a6d6eed74dcf85960571fdbaec885ef8350845f3b94a92cf0` | `3a73ab6f1c84fe5a6d6eed74dcf85960571fdbaec885ef8350845f3b94a92cf0` | Match |
| `🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/📥️capture/🧪️tests/📥️input/🟦️.ts` | `e2c5e8b78682d0d5451124fe52c0a9e6b75ab94199b6189e2b1d117a664cb58c` | `e2c5e8b78682d0d5451124fe52c0a9e6b75ab94199b6189e2b1d117a664cb58c` | Match |
| `🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/📥️capture/🧪️tests/🟦️.ts` | `1dd4bdd310f2f1411933c2dfb33dc301005631cb8c437f85934b8aea4c2f1797` | `1dd4bdd310f2f1411933c2dfb33dc301005631cb8c437f85934b8aea4c2f1797` | Match |
| `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/📜️script.ts` | `1f12db9c70147291fac1390eb56fabdcf32c29c497d765faa92855e26b13c7ef` | `1f12db9c70147291fac1390eb56fabdcf32c29c497d765faa92855e26b13c7ef` | Match |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🦀️rust/📜️script.ts` | `15a90d610c9504afe62c32713a654639b57066051073b3545b5a26eb190f3088` | `15a90d610c9504afe62c32713a654639b57066051073b3545b5a26eb190f3088` | Match |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/📦️installation/🟦️.ts` | `de398985023d235ce1869dd81b3aa544b7b87f6245befa4ffbba85e714f92733` | `de398985023d235ce1869dd81b3aa544b7b87f6245befa4ffbba85e714f92733` | Match |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🏗️native-build/📥️published/🧪️tests/🟦️.ts` | `5de9dd1ae57f73bffec317710bb0605f120b4569a46ecc8bfc6d982baff62d01` | `5de9dd1ae57f73bffec317710bb0605f120b4569a46ecc8bfc6d982baff62d01` | Match |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🦀️rust/📜️script.ts` | `2479ad375340a146bbaaaa16cdd033bcd745293f3d5518beafaf1a821776dd88` | `2479ad375340a146bbaaaa16cdd033bcd745293f3d5518beafaf1a821776dd88` | Match |
| `hub-host-freeze-test-prototype.log` | `b5aebbb02c8e438edcda1501ca5278e0746897773af75ec489ff1e5a1ed6ea6f` | `b5aebbb02c8e438edcda1501ca5278e0746897773af75ec489ff1e5a1ed6ea6f` | Match |
| `hub-host-freeze-test-release.log` | `c959c692c10e1e3f723af2d8ff4f00b2c62361884d327c4a0e4c0f4bee882960` | `c959c692c10e1e3f723af2d8ff4f00b2c62361884d327c4a0e4c0f4bee882960` | Match |
| `hub-host-freeze-types-owned-prototype.log` | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` | Match |
| `hub-host-freeze-types-owned-release.log` | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` | Match |
| `hub-host-failed-birth-red.log` | `d5f1cd40b9cc4b9f0cfd147c296c9cdf3374652583198cb2eba1d1d925ae75ca` | `d5f1cd40b9cc4b9f0cfd147c296c9cdf3374652583198cb2eba1d1d925ae75ca` | Match |
| `hub-host-final-failure-types-owned-prototype.log` | `ad4e461722c67a4657cf7db9ffcc7d0c968a0fecf4f83bd51e10c19762f1ea3d` | `ad4e461722c67a4657cf7db9ffcc7d0c968a0fecf4f83bd51e10c19762f1ea3d` | Match |
| `hub-host-final-failure-types-owned-release.log` | `ad4e461722c67a4657cf7db9ffcc7d0c968a0fecf4f83bd51e10c19762f1ea3d` | `ad4e461722c67a4657cf7db9ffcc7d0c968a0fecf4f83bd51e10c19762f1ea3d` | Match |
| `hub-host-foundation-prototype2.log` | `3c03a7d50fa0914dff5f57443feca0b60c28578f051cf1e6178f24218ba62389` | `3c03a7d50fa0914dff5f57443feca0b60c28578f051cf1e6178f24218ba62389` | Match |
| `hub-host-foundation-release.log` | `a19921dea27ce2ffa71836936332b172b892109e43ab25e7e64cc72a9d5248ff` | `a19921dea27ce2ffa71836936332b172b892109e43ab25e7e64cc72a9d5248ff` | Match |

## failed-original-admission-current.md

| Path | Report SHA256 | Final Observed SHA256 | Result |
| --- | --- | --- | --- |
| `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts` | `9f8afc70e3f5efa21d9dcc8a6d7e90ce0057ac9df42dab5645a2eb37e5f0c9be` | `9f8afc70e3f5efa21d9dcc8a6d7e90ce0057ac9df42dab5645a2eb37e5f0c9be` | Match |
| `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🚪️admission/🧬️schema/🔣️.json` | `aef8f7d29b79448262a4476eea5c80acde75188821a1b8f59ce7c38fb1bc6047` | `aef8f7d29b79448262a4476eea5c80acde75188821a1b8f59ce7c38fb1bc6047` | Match |
| `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🚪️admission/🧪️tests/🔣️.json` | `bc4a9c195c54ac04ee990dd986779602d1887fd56b6280c40737c07d711375f3` | `bc4a9c195c54ac04ee990dd986779602d1887fd56b6280c40737c07d711375f3` | Match |
| `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🚪️admission/🧪️tests/🟦️.ts` | `00f26f68c542b594c2fd6ee5fbee71bccd400f6f0d14035c52eaa496a245ece9` | `00f26f68c542b594c2fd6ee5fbee71bccd400f6f0d14035c52eaa496a245ece9` | Match |
| `🧰️framework/🔨️modules/🏃️process/🧪️testing/🦀️cargo/🎯️exact/🟦️.ts` | `ad67c0b02e5e9f86fb3c327fa1863176aefffb180b437e05d14ab26c4325223e` | `ad67c0b02e5e9f86fb3c327fa1863176aefffb180b437e05d14ab26c4325223e` | Match |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/typescript-artifact/📜️script.ts` | `2676e62509fa44ce59c0f6e64d859bc6aaa4521055e90368cbd9f79ec693e03a` | `572de3b5fc022fe82f75953b2b7ef169a17a4c029060df0fb93c17eeba124934` | Successor |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/🗑️generated/failed-original-admission-red1-prototype.log` | `fa84f52729d2b2d0324f1b5357fec704d47ed86b0d535ae539c73b87eeee88d0` | `fa84f52729d2b2d0324f1b5357fec704d47ed86b0d535ae539c73b87eeee88d0` | Match |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/🗑️generated/failed-original-admission-green2-prototype.log` | `c03677d81c643b8881348b2de14a447738490e2a672529f7d3e025b54da64884` | `c03677d81c643b8881348b2de14a447738490e2a672529f7d3e025b54da64884` | Match |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/🗑️generated/failed-original-admission-green2-release.log` | `ae8b1a8f88fc254cc3e56158892b4ef410fdfc802b488ea510664dfd5b9ad9a8` | `ae8b1a8f88fc254cc3e56158892b4ef410fdfc802b488ea510664dfd5b9ad9a8` | Match |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/🗑️generated/failed-original-admission-types2-prototype.log` | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` | Match |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/🗑️generated/failed-original-admission-types2-release.log` | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` | Match |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/🗑️generated/failed-admission-regression-bun_file_cohort3-prototype.log` | `ad03ce10291aa25674f618aab782b31be5078e62564c79509a480e6feea35aa6` | `ad03ce10291aa25674f618aab782b31be5078e62564c79509a480e6feea35aa6` | Match |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/🗑️generated/failed-admission-regression-bun_file_cohort3-release.log` | `87cab03c9654aad3ee4151c312f0af5fd77d6c4fc3c42ef4d66906781da4902c` | `87cab03c9654aad3ee4151c312f0af5fd77d6c4fc3c42ef4d66906781da4902c` | Match |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/🗑️generated/failed-admission-regression-control_child3-prototype.log` | `90126710b6726432684a979ea2487e2bf82d562f6c9ad1322637771a8b16a28c` | `90126710b6726432684a979ea2487e2bf82d562f6c9ad1322637771a8b16a28c` | Match |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/🗑️generated/failed-admission-regression-control_child3-release.log` | `2d466456a8e32390b604d9f548d60e364c88d2b06eeb0047219f673cd950ffb2` | `2d466456a8e32390b604d9f548d60e364c88d2b06eeb0047219f673cd950ffb2` | Match |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/🗑️generated/failed-admission-regression-exact_cargo_output3-prototype.log` | `d1b157f8e7a85c511f0b0c705dcf9307dc503fa4c0c114a7d96d92983434039c` | `d1b157f8e7a85c511f0b0c705dcf9307dc503fa4c0c114a7d96d92983434039c` | Match |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/🗑️generated/failed-admission-regression-exact_cargo_output3-release.log` | `d388c5328236791b2480a685d624b80c7dc78a858571a8a536d60e4dbe5e9c81` | `d388c5328236791b2480a685d624b80c7dc78a858571a8a536d60e4dbe5e9c81` | Match |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/🗑️generated/failed-admission-regression-owned_input3-prototype.log` | `58c055d298cbb6d60e2ec25304c4f9b5f6c733f7759aa6bd0cd69774f91c4e7c` | `58c055d298cbb6d60e2ec25304c4f9b5f6c733f7759aa6bd0cd69774f91c4e7c` | Match |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/🗑️generated/failed-admission-regression-owned_input3-release.log` | `08b794daaae40502f9143f631dee5c0f2941998992ae662c51c6edbfefcf320f` | `08b794daaae40502f9143f631dee5c0f2941998992ae662c51c6edbfefcf320f` | Match |

## control-child-projection-current.md

| Path | Report SHA256 | Final Observed SHA256 | Result |
| --- | --- | --- | --- |
| `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts` | `1b20231752c504075a4ae50859afc735c07145f97765700e59fad85cd5ac8aac` | `9f8afc70e3f5efa21d9dcc8a6d7e90ce0057ac9df42dab5645a2eb37e5f0c9be` | Successor |
| `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/📥️invocation/🏃️process/🟦️.ts` | `2639df5408f60a7f7fe07d1509d47599f7f007c4fce848c109a49e70acd399f8` | `2639df5408f60a7f7fe07d1509d47599f7f007c4fce848c109a49e70acd399f8` | Match |
| `🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/📥️capture/🟦️.ts` | `0012cf2f899ec87eba9c83f6610ca03bbdcd476a02a6b62e1ca419122c7f9397` | `0012cf2f899ec87eba9c83f6610ca03bbdcd476a02a6b62e1ca419122c7f9397` | Match |
| `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/📤️child/🧬️schema/🔣️.json` | `92cd9d0027b5c495750f9eaccdc394fe66997886cc70d27fd537f0d43e206759` | `92cd9d0027b5c495750f9eaccdc394fe66997886cc70d27fd537f0d43e206759` | Match |
| `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/📤️child/🧪️tests/🔣️.json` | `89b6cf1076189d9896e6c0a0d392d5f90a8845b694c74674fda6c4d97d09896d` | `89b6cf1076189d9896e6c0a0d392d5f90a8845b694c74674fda6c4d97d09896d` | Match |
| `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/📤️child/🧪️tests/🟦️.ts` | `fe1aff37ddd1ac61f9e4a32d590cfc90c88f260cf94edd273b2efe281d4b7e72` | `fe1aff37ddd1ac61f9e4a32d590cfc90c88f260cf94edd273b2efe281d4b7e72` | Match |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/typescript-artifact/📜️script.ts` | `f6b5138e6d487605fb322a61a82f449da89bf7217a3ea9b979cc76f799256583` | `572de3b5fc022fe82f75953b2b7ef169a17a4c029060df0fb93c17eeba124934` | Successor |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/🗑️generated/control-child-projection-red1-prototype.log` | `ffbccdaa2ccc3a954df9ee93e99a9fcdbe794fd5342886ac34cc869940d64542` | `ffbccdaa2ccc3a954df9ee93e99a9fcdbe794fd5342886ac34cc869940d64542` | Match |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/🗑️generated/control-child-projection-red2-prototype.log` | `12bb18a24109eab19d3ff648357aa90e5af1d1a97747d348fdab1fdd4b61a554` | `12bb18a24109eab19d3ff648357aa90e5af1d1a97747d348fdab1fdd4b61a554` | Match |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/🗑️generated/control-child-projection-green4-prototype.log` | `f6787df8a148c9e62f47e44865ce69768e9354348d055c48bae7542fbead2159` | `f6787df8a148c9e62f47e44865ce69768e9354348d055c48bae7542fbead2159` | Match |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/🗑️generated/control-child-projection-green4-release.log` | `ca10bc8f038d0bb349c96eb99674190ba11eb0240604adb1c99ce8a845b9311b` | `ca10bc8f038d0bb349c96eb99674190ba11eb0240604adb1c99ce8a845b9311b` | Match |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/🗑️generated/control-child-projection-types4-prototype.log` | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` | Match |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/🗑️generated/control-child-projection-types4-release.log` | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` | Match |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/🗑️generated/owned-binary-input-projected6-prototype.log` | `20fa0dabef9843c25bc019b35c9052f4415bc8c83b1ce05e2b869528420bb6f0` | `20fa0dabef9843c25bc019b35c9052f4415bc8c83b1ce05e2b869528420bb6f0` | Match |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/🗑️generated/owned-binary-input-projected6-release.log` | `91988ec022ef27334bf11b8a33bc64e278fd32466a3e6a34c77c89e90898f505` | `91988ec022ef27334bf11b8a33bc64e278fd32466a3e6a34c77c89e90898f505` | Match |
