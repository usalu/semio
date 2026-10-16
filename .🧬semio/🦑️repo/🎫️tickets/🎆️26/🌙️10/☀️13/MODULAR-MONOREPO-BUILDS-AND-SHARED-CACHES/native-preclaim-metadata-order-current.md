# Native Creator Metadata Ordering Successor

This corrects the P1 in [Independent Frozen Preclaim Successor Audit](audit-native-funded-preclaim-successor-current.md). It is a Source and ordinary-filesystem checkpoint; no Native provider, image, mount, compiler, cache, Hub, SDK, browser or network operation ran. The actual backend constructor and compiler-volume transaction remain unintegrated. No physical durability/crash recovery or whole-cache retirement qualification follows.

## Correction

The actual creator now persists each newly created directory entry before acknowledging its stage. Under the existing real creator EX, Original/control and accepted cleanup ancestry, private syncCreatedEntryV1 checks the recorded parent and newly observed entry dev/ino/mode, syncs the actual directory containing that entry through the existing descriptor/metadata owner, then checks both original identities again. Only afterward does the real claim append run.

The order is mkdir(child), actual parent-directory sync, directory-stage append; then mkdir(child/m), actual child-directory sync, root-stage append. The existing claim-file fsync and containing-directory sync remain. This also removes the earlier directory-stage acknowledgement window, while correcting the audited missing child-directory barrier before the root stage. No generic path transfer/adoption or provider issuer was introduced.

The original finite request, schema copies, same creator proof, bounded inventory, pending drain, Error/undefined precedence, descriptor ambiguity retention and accepted nonrenewable cleanup remain unchanged. Unknown/partial entries still refuse. Native success still cannot be inferred from this filesystem claim.

## Schema-First Evidence

The claim protocol declares the four metadata events in their required order. Language-neutral corpus rows cover ordered, late-root-sync and missing-root-sync cases. Independent Ajv exact-output and SQLite ordering queries agree.

Genuine RED 83276 ran the actual isolated filesystem creator under a real SQLite writer and carried Original. A dev-only observer wrapped the real system FileHandle.sync method, called it on the actual handle, and recorded actual fstat identity plus the bounded original claim stage. The actual trace was only ["directory-claim","root-claim"]; SQLite correctly refused the missing preceding metadata barriers. No accepted Native provider was replaced or fabricated, and no crash was simulated.

After correction, both final logs retain this actual trace:

    [DEBUG] Actual creator metadata order ["directory-parent-sync","directory-claim","root-parent-sync","root-claim"]

The observer always restores the original method. It observes system calls and their ordering; it does not establish power-loss behavior, Native image recovery or Windows support. The existing POSIX-only preclaim boundary remains explicit.

## Terminal Qualification

| Check | Handle | Generated log | Actual result |
| --- | --- | --- | --- |
| Genuine RED prototype | 83276 | native-preclaim-metadata-red-prototype.log | exit 1, missing metadata barriers |
| Final Source prototype | 77422 | native-preclaim-metadata-test-prototype2.log | exit 0, 33 tests / 848 assertions |
| Final Source release | 50001 | native-preclaim-metadata-test-release.log | exit 0, 33 tests / 848 assertions |
| Expanded strict prototype | 75996 | native-preclaim-metadata-types-prototype.log | exit 0 |
| Expanded strict release | 24052 | native-preclaim-metadata-types-release.log | exit 0 |

Source results comprise retirement 18/570, Native cleanup operation 1/14, fixed command 5/126, persistent contract 1/16, artifact inspection 3/58 and group 5/64. Earlier prototype 1473 was also green at 33/847 before adding the diagnostic trace assertion; final prototype recapture above includes that assertion. All own handles are terminal. Both strict logs are empty, and actual terminal exit statuses were collected.

The existing SEMIO_CACHE_RETIREMENT_ONLY route/selector is reused; all required roots and peer selectors remain. The parent Bun cache-off environment, Original 120-second envelope, existing test timeouts, physical caps/floors and policy values were unchanged. No executable command or launch row was added.

## Remaining Boundary

Independent Source review precedes the broader actual backend attempt/ordinary handoff/deferred failed-birth constructor cut. The current compiler-volume reserve/intent/open path remains unchanged. Real durable success must persist; only genuine failed-birth or explicit-retirement completion may retire parent charge. This successor does not issue such completion or close the full goal.

## Changed Source

Snapshot 2026-10-16T15:40:29.602919+00:00. Four implementation/schema/test/corpus paths changed. Owned hunks are frozen pending independent review.

- `33dec722f2597b1bdd18645bd2a4ea4c7555c473d3611d67dbdcdb2b91f369f0` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌱️birth/🧾️claim/🟦️.ts`
- `be275f9d77d6a666a155bd3c9bdc2afead65c7a7172f26cb6b8e3c512e323ae1` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌱️birth/🧾️claim/🧬️schema/🔣️.json`
- `bd0e18005bc6cd912fdf970a2aeb62d6baf2cf58dc0d4acfaf9ea7e076c4af1e` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/🧹️retirement/🧪️tests/🟦️.ts`
- `e240739ff5d5524cfc65de583f931671a46261827305bd424aa4e6dfd9096591` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/🧹️retirement/🧪️tests/🔣️.json`

## Full Observed Source Closure

The 49 paths below refresh the prior observation. Besides the four changed paths, the shared ticket wrapper changed under Root ownership. Shared entries are exact observations rather than a repository-wide lock; no Source file was copied into the ticket.

- `0fc30fde879557b8b42efa7157791c65c8ce3c3cab083b37328e965c33ce0346` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🟦️.ts`
- `0bb49135c534e19d4e885a4b2557f7d73cd1291c9f0986f92827a29007feed1a` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧬️schema/🔣️.json`
- `33dec722f2597b1bdd18645bd2a4ea4c7555c473d3611d67dbdcdb2b91f369f0` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌱️birth/🧾️claim/🟦️.ts`
- `be275f9d77d6a666a155bd3c9bdc2afead65c7a7172f26cb6b8e3c512e323ae1` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌱️birth/🧾️claim/🧬️schema/🔣️.json`
- `bd0e18005bc6cd912fdf970a2aeb62d6baf2cf58dc0d4acfaf9ea7e076c4af1e` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/🧹️retirement/🧪️tests/🟦️.ts`
- `e240739ff5d5524cfc65de583f931671a46261827305bd424aa4e6dfd9096591` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/🧹️retirement/🧪️tests/🔣️.json`
- `037d12f551ca460c8065dc2e4eabc9e04a28458e8b14a625654878a3e61a7a87` — `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts`
- `b0889a857b152189a254acc60d642962d0f7eeaa46f3df379941abdc7f5d8076` — `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🧬️schema/🧹️cleanup.json`
- `d761d7b644ee44c9e809fc3df01bb6630e3440c69c38cd59ad3276963d13ab8e` — `🧰️framework/🔨️modules/🏃️process/🔒️leases/🟦️.ts`
- `f0a5bed40500e74b04bd616cc21c0e184f3c530fcc66031ae3fc818f9826bf9b` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎰️jobserver/🟦️.ts`
- `e4edb311307c6f081e7faeb6444b06148d65bb718302d259a91030d6ff4bbc9e` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/🧹️retirement/🟦️.ts`
- `54815db155e46e55fb637da1aa8e9602964c95b2119165e5006b67c44c5dc571` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/🧹️retirement/🧬️schema/🔣️.json`
- `5746097831ab26e0d61ad479cb70c8cb9f1094ff384693174e1e8ebbf86d1689` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/🧹️retirement/🧾️journal/🟦️.ts`
- `76f262bdab0be60cc87314391d9160e849765650601c10e2749ad1ca7ac57645` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/💾️volume/🟦️.ts`
- `05afde1006332ca3ff978bf066b9b51ff7a4e27bd06cbcea5ac373820f45261e` — `🧰️framework/🔨️modules/🏃️process/🔒️leases/🧹️cleanup/🧬️schema/🔣️.json`
- `8842b932f693871f1e9545e274c7eaf68211e339c73a8b50e156617661e403fd` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📥️borrower/🟦️.ts`
- `47e7753e855eb3790283df998123b43698858b196ea5b062e6869b3ea3e51951` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧱️fixed/🟦️.ts`
- `34a2618d9973bd5e27bd9fa012b9b0ead2c6424ae4e9d50a9941a25583144281` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧱️fixed/🧬️schema/🔣️.json`
- `8cfb2140215f03a1c61751b18842673c88b9bcd090e27825c720046f3f23421b` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧱️fixed/🎛️command/🧪️tests/🟦️.ts`
- `abb955a2debd2241c42d33a430c71facc23e8b3186390826d952915ffa3a17ed` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/🎛️command/🟦️.ts`
- `fc9fde6d2210ce57ecb7b77a91b9659a44af3824fb2390d4f8747f984fbc0c84` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/♻️retirement/📋️contract/🟦️.ts`
- `4cd14475e6234196948ac5d25f90faab807f5c4b458d7a70c716096334a43330` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/♻️retirement/🧬️schema/🔣️.json`
- `11dd71b8c2fe63760cf9339eda95f0e12ce094a924bf543bf552dadd19796ca7` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/♻️retirement/💾️durability/🟦️.ts`
- `467ccc19439c45071bd990b81376ac1135194b5e6c073105a4d8194a29568a79` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/🟦️.ts`
- `86ba794faeb81d713ed674d81b9d631528fe028f435c31f20ec634fcb9167789` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/📂️mount/🟦️.ts`
- `f623f976e1a752bb35d5de251e40190c0690a6440612ecadfabd7a91af2ed26b` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/📂️mount/🧬️schema/🔣️.json`
- `7098baac3a190f61ec9c665f12bd8928f69b1ac3ea7a0ed1fe1f8f2f148c0daa` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🧪️tests/📥️persistent/🟦️.ts`
- `493356524ddbc13ef24a1e4d4a7fe4a17542b420e3e76bf13d03998575c565b2` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🟦️.ts`
- `ad675f6c626de96b6fb0548991ea99cbd6fdb3473fdb472b8cee0f936eecc0b7` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/📥️operation/🧪️tests/🟦️.ts`
- `cbff54d4a42934212145f3a72c3a53cbf8d31f07f865bf976c142c9ff954ac34` — `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/typescript-artifact/📜️script.ts`
- `9991ba04ed6983c0d385bce480ce7342ff8cbbfff03935895637d982f39022c8` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/📥️operation/🧬️schema/🔣️.json`
- `c3911317120432f90966130b0638b32984a44e0a989174880328ec7068d5ca4a` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌱️birth/🟦️.ts`
- `011d1fa4b600f74a2f76c050ad544b82d9c991119cb7d5292f0c604cb7fed972` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌱️birth/🧬️schema/🔣️.json`
- `1235600ed5a41fcee181291542282b5032ccf85ca96b9b7935fd64650542a775` — `🧰️framework/🔨️modules/📁️filesystem/🧾️observation/🟦️.ts`
- `cd66522e5157848a49c748e3214401ce29fa319639d9e55e20706de92b681851` — `🧰️framework/🔨️modules/🏃️process/💾️state/🧾️records/🟦️.ts`
- `77cc1d992a271ce7cdb17d72a3428f32c0255ada7e6a9d5214824fd718d43e12` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧾️metadata/🟦️.ts`
- `95e3a99263460d41d6ec92f3849e62b7f0096f98fb0f02f414c590c1fc471aa8` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧾️receipt/📦️retention/📁️copy/🟦️.ts`
- `045f149b3dd099cbfab7d94ac8128d515b6db2b1c3ac82f4a0d626c22f98414d` — `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️model/🟦️.ts`
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
