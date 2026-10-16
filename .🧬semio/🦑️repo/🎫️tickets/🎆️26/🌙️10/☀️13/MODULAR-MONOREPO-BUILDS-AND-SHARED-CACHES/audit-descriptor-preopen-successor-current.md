# Descriptor Preopen Successor Source Audit

Read-only Source review on 2026-10-16. No Source modifications, tests, probes, builds, Native effects, network or Git operations were performed. This report qualifies only the reviewed Source paths, not Native runtime correctness.

## Prior Finding Status

The descriptor-closure P2 in audit-retirement-preappend-current.md is corrected in the reviewed helper and genuine borrower release paths. Descriptor helper line 15 reserves the finite private record before invoking the opener, retains the actual returned object and captured close method, attempts close once, and removes the record only on success or failed opening. A failed close retains object and error, preserving the exact body error including undefined. There is no public adopter, clear or retry API and no retained numeric-FD retry in this helper. Its cap includes active and unresolved records.

Borrower Source lines 15 and 20 strongly retain authenticated scopes and admit at most the schema owner bound. Normal close line 32 drains accepted work then checks the actual descriptor ledger before lease release, retaining the real scope on refusal; first accepted-work failure remains primary. Cleanup close line 34 and continuation line 38 also check before actual release. Therefore the reviewed refusal does not rely solely on a process-local flag after releasing SH. Mac failed-reader fallback line 122 and Fixed fallback line 104 separately check denial before raw fallback release and retain the receiver on refusal. Successful body plus failed descriptor close rejects with that close error; a subsequent borrower close cannot release its guard through these paths.

Actual Mac retirement lines 38–39, Fixed retirement line 132, accepted cleanup command line 60, private removal line 63 and settlement line 68, Fixed terminal command line 160 and disposable line 211 consume the descriptor denial before their reviewed destructive effects. Existing identity, exact receiver and exclusion prerequisites remain necessary. No additional actionable defect was established in this review. This is not a proof that every Native operation is safe.

## Scope And Evidence Limits

Reference census covered Framework and Hub TS/TSX/JS/MJS/CJS references. No opened-object compatibility overload exists in the actual helper. This review did not independently parse all callers with TypeScript or rerun the reported tests; the implementation report's broader census and runtime claims remain reported evidence. Ordinary FileHandle/SQLite observations in existing logs do not establish actual Native close syscall failure, Native teardown, Windows durability or restart recovery. No synthetic Native provider was accepted or executed here.

Existing test logs were read, not executed. The two preopen test logs contain multiple suite summaries, so their final five-test footer alone is not the claimed aggregate. Empty strict logs contain no standalone exit-status proof. No new claim of passing tests follows from their contents.

## Exact Hash Agreement

All 46 sealed Source paths were hashed before and after Source review. There was no interval Source drift. 45 matched the implementation report exactly. The ticket-local typescript-artifact/📜️script.ts was already b89bb3ad4bb451ae9a7f677b7217d7d8e5bc52793c823805ce9d31ee0f7b0565 at both observations, versus historical reported 877b98de9eac8e6cfb13a675faef11f41e0265e17f8f6f8f0a10e507c57a7093. That is actual preexisting peer drift, not sealed agreement. The complete observed Source list follows (each hash is identical before and after):

- `e4edb311307c6f081e7faeb6444b06148d65bb718302d259a91030d6ff4bbc9e` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/🧹️retirement/🟦️.ts`
- `54815db155e46e55fb637da1aa8e9602964c95b2119165e5006b67c44c5dc571` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/🧹️retirement/🧬️schema/🔣️.json`
- `5746097831ab26e0d61ad479cb70c8cb9f1094ff384693174e1e8ebbf86d1689` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/🧹️retirement/🧾️journal/🟦️.ts`
- `af65c8feb3721590cfeb6ebb98951972ed02c18108e517e50f7323ac6db689d4` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/🧹️retirement/🧪️tests/🟦️.ts`
- `6d517b6f2a69e43aa0171d19c79ef4e6cc7d44005f7c9f14d42e5cf2682ae28d` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/🧹️retirement/🧪️tests/🔣️.json`
- `76f262bdab0be60cc87314391d9160e849765650601c10e2749ad1ca7ac57645` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/💾️volume/🟦️.ts`
- `1fc2482d1db44cb5aa7b7d0e9cb6ee30154148a974458a8df8c5d5a2d2bbe5c2` — `🧰️framework/🔨️modules/🏃️process/🔒️leases/🟦️.ts`
- `05afde1006332ca3ff978bf066b9b51ff7a4e27bd06cbcea5ac373820f45261e` — `🧰️framework/🔨️modules/🏃️process/🔒️leases/🧹️cleanup/🧬️schema/🔣️.json`
- `1352b5735c177f9ab388849048b8722554a968f079674eca44b6716d4d73d51f` — `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts`
- `cdb3bf110c01fb5b1cbbea00c951d5ce5ffaae8af479a23050db9c6dcbf135d2` — `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🧬️schema/🧹️cleanup.json`
- `8842b932f693871f1e9545e274c7eaf68211e339c73a8b50e156617661e403fd` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📥️borrower/🟦️.ts`
- `9bea270880c93a02a59c65a12ad33f2f83076f7078c735f987feb1fc0423040a` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🟦️.ts`
- `72e80b67358cb9c61ed50f0842919a6e0d9472cbc9e7868d8a4754d1b0f7a365` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧬️schema/🔣️.json`
- `ae1d6460c4a09b719bdddbe8cbc1478a0b81f8a09b5589ceaeea607d60971523` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧱️fixed/🟦️.ts`
- `34a2618d9973bd5e27bd9fa012b9b0ead2c6424ae4e9d50a9941a25583144281` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧱️fixed/🧬️schema/🔣️.json`
- `8cfb2140215f03a1c61751b18842673c88b9bcd090e27825c720046f3f23421b` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧱️fixed/🎛️command/🧪️tests/🟦️.ts`
- `a247fcd8ec6f8c6b66744967a3a65bffdd06339d4cb69e7d1e74c46a0faca74f` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/🎛️command/🟦️.ts`
- `fc9fde6d2210ce57ecb7b77a91b9659a44af3824fb2390d4f8747f984fbc0c84` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/♻️retirement/📋️contract/🟦️.ts`
- `4cd14475e6234196948ac5d25f90faab807f5c4b458d7a70c716096334a43330` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/♻️retirement/🧬️schema/🔣️.json`
- `11dd71b8c2fe63760cf9339eda95f0e12ce094a924bf543bf552dadd19796ca7` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/♻️retirement/💾️durability/🟦️.ts`
- `467ccc19439c45071bd990b81376ac1135194b5e6c073105a4d8194a29568a79` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/🟦️.ts`
- `f4821d91edb5e4b2d9926d3173942a83453e0343658c5846fcfb1e760fb9dd08` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/📂️mount/🟦️.ts`
- `f623f976e1a752bb35d5de251e40190c0690a6440612ecadfabd7a91af2ed26b` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/📂️mount/🧬️schema/🔣️.json`
- `7098baac3a190f61ec9c665f12bd8928f69b1ac3ea7a0ed1fe1f8f2f148c0daa` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🧪️tests/📥️persistent/🟦️.ts`
- `493356524ddbc13ef24a1e4d4a7fe4a17542b420e3e76bf13d03998575c565b2` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🟦️.ts`
- `ad675f6c626de96b6fb0548991ea99cbd6fdb3473fdb472b8cee0f936eecc0b7` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/📥️operation/🧪️tests/🟦️.ts`
- `b89bb3ad4bb451ae9a7f677b7217d7d8e5bc52793c823805ce9d31ee0f7b0565` — `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/typescript-artifact/📜️script.ts` (historical seal differs)
- `9991ba04ed6983c0d385bce480ce7342ff8cbbfff03935895637d982f39022c8` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/📥️operation/🧬️schema/🔣️.json`
- `14b653cb668a2c70cfb190996a725839f47ca06ac5db2c0056b6c8dc01c2104d` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌱️birth/🟦️.ts`
- `011d1fa4b600f74a2f76c050ad544b82d9c991119cb7d5292f0c604cb7fed972` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌱️birth/🧬️schema/🔣️.json`
- `1235600ed5a41fcee181291542282b5032ccf85ca96b9b7935fd64650542a775` — `🧰️framework/🔨️modules/📁️filesystem/🧾️observation/🟦️.ts`
- `cd66522e5157848a49c748e3214401ce29fa319639d9e55e20706de92b681851` — `🧰️framework/🔨️modules/🏃️process/💾️state/🧾️records/🟦️.ts`
- `77cc1d992a271ce7cdb17d72a3428f32c0255ada7e6a9d5214824fd718d43e12` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧾️metadata/🟦️.ts`
- `95e3a99263460d41d6ec92f3849e62b7f0096f98fb0f02f414c590c1fc471aa8` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧾️receipt/📦️retention/📁️copy/🟦️.ts`
- `a29900a188c012f5e92bad644712b28790b51d7c652aea1803029aff5f231dda` — `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️model/🟦️.ts`
- `5469ea8f046566c55b227dbbe6a30c01711e5799ed09265fc807639fab1d3985` — `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📦️distribution/📋️inventory/🟦️.ts`
- `61d87f9359d89d7b3228232cc94d2b98a604ba3bf183684679df980884e844c5` — `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🟦️.ts`
- `dff86ff4e29c240118a0bd6711c1225c4b12c7621254eab049dcaa335bc44495` — `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/👓️reading/📥️inspection/🟦️.ts`
- `262dd0aaa3b53c5a574568d5499b7028aa5c56afc2ba1e38858a5ee73b7fadae` — `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/👓️reading/📥️inspection/🧪️tests/🟦️.ts`
- `138efc710ef295290ecc3a49619bb16972ce3090a09455a93c1640aac5c41eee` — `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🕸️wasm/👓️reading/🟦️.ts`
- `f1df14f945d211da7d5209f57c64db7f1d8d872218a6e537485d7568cfe43006` — `🧰️framework/🛍️products/📓️print/📦️artifacts/📥️material/📤️producer/🟦️.ts`
- `fc64eaaa14e87528bc8af03d3e185b38fe50d8e61acb92134cecbc979ca4623f` — `🌎️hub/🏗️bootstrap/🧾️provenance/📦️cargo/🧫️fixture/📥️reading/🟦️.ts`
- `5b1ac03aeec1c6e0884eecbe1ea1307a0a0c695186eec8378252e6c45ca29e33` — `🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts`
- `4e32ecc41e1d9fc6a7569fcac37444bc839d70be952a85651d170f11f668d2ed` — `🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🧬️schema/🔣️.json`
- `243aabc2e894c3e623ec05cdc8b6d411e17aad6896b7fdf85a66f1276b1d7343` — `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/👓️reading/📚️group/🧪️tests/🟦️.ts`
- `2288bcb947a2dc3d0c377f5213a0b2b7e3656380b2813a2c27a393016d574bef` — `🧰️framework/🔨️modules/📁️filesystem/🧾️observation/🧪️tests/🎛️control/🟦️.ts`

## Existing Log Observations

These exact hashes were checked twice consecutively while preparing this report, with no interval drift; they are observations, not locks or independent execution evidence.

- `df4a7a7114095ac1d18069899f00773e297558597528f35adc91520f2f4166d3` — `descriptor-preopen-test-prototype5.log`
- `d01905d4518d1d9656c93f6b31e67d1abec65be195e7f9ed773a09d6acb2f665` — `descriptor-preopen-test-release5.log`
- `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` — `descriptor-preopen-types-prototype6.log`
- `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` — `descriptor-preopen-types-release6.log`
- `3060ca0ced66571d371262714ae8a29bb3c0a257ac61096f2d49c6f2205a8473` — `descriptor-observer-test-release.log`
- `a192fcdee313cb3697dec33b3a34a49235b838f13ecbf2ccc0de6aa6ffd39a0e` — `descriptor-observer-test-prototype.log`
