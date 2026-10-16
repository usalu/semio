# Preappend Retirement Source Audit

Read-only Source audit on 2026-10-16 of the last checkpoint30 and implementation report. No Source edit, Git mutation, Native/image/mount/compiler/Cargo/cache/network/runtime operation or test occurred. Hashes below are observations, not locks. The routing process and fixed backend bodies are live peer changes relative to checkpoint30; this audit does not treat them as the stable checkpoint or qualify their new expansion.

## Actionable Finding

### P2 — Retain failed journal reader descriptor closure separately from the body outcome

The newly shared journal reader at native storage `🟦️.ts:97` now consumes `readPhysicalFileV1` for both transaction replay and cleanup. Its descriptor dependency `📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts:5` catches a failed `descriptor.close()` and silently discards that failure whenever the read body already failed. No owner retains that unresolved descriptor or close outcome. Thus a cancellation/identity/read failure (including a directly thrown Error or undefined from the authority callbacks) followed by a close failure exits solely with the original body failure; the actual cleanup cannot distinguish a closed descriptor from an unresolved descriptor. The journal transaction then reaches its lease-release finally, and allocation cleanup also reaches physical-reader close. This newly consumed read path therefore does not establish the claimed descriptor-closure prerequisite under the dual-failure branch. Preserve the original body value as primary DATA while separately retaining/exposing the failed close and its descriptor ownership to the cleanup owner; do not mark physical reader settlement on that branch. This is a static dual-failure branch finding, not a reproduced OS failure or a claim that ordinary success leaks descriptors.

## Verified Corrections And Boundaries

- Both prior P2s are corrected in actual private creator teardown. Final no-image observation requires the captured mount-directory dev/ino/mode. `removeSettledNativeImageV1:62` invokes the empty-root preflight before receipt/backing unlink; the persistent-existing branch passes that same preflight and repeats exact mount identity before settlement. Unknown contents therefore refuse before those unlinks.
- The preflight closes its directory handle, preserves direct Error/undefined using an explicit failed flag, combines distinct close/body failure, and rechecks exact identity after closure. Its ordering is consumed by the actual private creator.
- `storageCustodyV1.transaction` privately captures from the exact real held journal EX with `acceptResourceLeaseCleanupV1`; `acceptNativeStorageCleanupV1` consumes that closure rather than reacquiring a second journal lease. The closure is not exported. Actual issuance/liveness checks reject a delayed capture after lease release.
- The shared reader admits the initial regular/no-link/single-link extent, caps actual reads at that extent and the schema maximum, uses nofollow descriptor opening where exposed, compares descriptor/path identity before and after, and validates UTF-8, complete lines and event replay. The finding above is confined to unresolved closure after a body failure.
- Direct body Error/undefined survives the storage transaction and `withNativeStorageCleanupV1` explicit failed flags. The latter combines distinct physical-close failure. The existing transaction's secondary metadata/release diagnostics intentionally retain the primary; they do not prove success and are not reported as a false success.
- The birth model is finite neutral DATA with separate retained-success and retirement paths. A generated valid sequence cannot retire the parent after child-retained. The model grants no backend authority; no generic peer disposable port is alleged to reach a real-resource consumer here.
- Parent preappend reservation ownership, durable backend attempt, funded constructor, real failed-birth teardown and whole-cache retirement remain explicitly pending. Their absence is not a defect finding against this implemented prerequisite prefix.

## Observed SHA-256

- `e4edb311307c6f081e7faeb6444b06148d65bb718302d259a91030d6ff4bbc9e` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/🧹️retirement/🟦️.ts` (checkpoint30 match)
- `54815db155e46e55fb637da1aa8e9602964c95b2119165e5006b67c44c5dc571` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/🧹️retirement/🧬️schema/🔣️.json` (checkpoint30 match)
- `96160822fca8c2809103582b6a8f8a3d7643d6bd885306ec1540417dfc49ac4f` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/🧹️retirement/🧾️journal/🟦️.ts` (checkpoint30 match)
- `db898113743d1949672004a505f1af83b6a1703a0f556e464518224beed79770` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/🧹️retirement/🧪️tests/🟦️.ts` (checkpoint30 match)
- `688e72bf886c6826b18d6e8ca8a29e4357f0cba8fafa8b0774132aa26a0ba3ec` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/🧹️retirement/🧪️tests/🔣️.json` (checkpoint30 match)
- `76f262bdab0be60cc87314391d9160e849765650601c10e2749ad1ca7ac57645` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/💾️volume/🟦️.ts` (checkpoint30 match)
- `1fc2482d1db44cb5aa7b7d0e9cb6ee30154148a974458a8df8c5d5a2d2bbe5c2` — `🧰️framework/🔨️modules/🏃️process/🔒️leases/🟦️.ts` (checkpoint30 match)
- `05afde1006332ca3ff978bf066b9b51ff7a4e27bd06cbcea5ac373820f45261e` — `🧰️framework/🔨️modules/🏃️process/🔒️leases/🧹️cleanup/🧬️schema/🔣️.json` (checkpoint30 match)
- `1352b5735c177f9ab388849048b8722554a968f079674eca44b6716d4d73d51f` — `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts` (live moved; checkpoint30 e824abd12449f6c63fc13819f4ea8f8bae85cf604158c535d4985897ce40a90f)
- `cdb3bf110c01fb5b1cbbea00c951d5ce5ffaae8af479a23050db9c6dcbf135d2` — `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🧬️schema/🧹️cleanup.json` (checkpoint30 match)
- `8e459d1ac298a39650827136d0bc2613b6aa57edc4d4251dafcc46a43f807d27` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📥️borrower/🟦️.ts` (checkpoint30 match)
- `9bea270880c93a02a59c65a12ad33f2f83076f7078c735f987feb1fc0423040a` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🟦️.ts` (checkpoint30 match)
- `ec880f2118dabaff237b68c47e5d29eee446ce508a7de4abaeec7d90bd88ec96` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧬️schema/🔣️.json` (checkpoint30 match)
- `eb63fae75e67a91569de86a1419e892108412ac875f111f2cf5e598c285be035` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧱️fixed/🟦️.ts` (live moved; checkpoint30 e643f796f56296f5e5d1700ca76af0522d6544951ce77b0b2c0e4c283f877d88)
- `34a2618d9973bd5e27bd9fa012b9b0ead2c6424ae4e9d50a9941a25583144281` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧱️fixed/🧬️schema/🔣️.json` (checkpoint30 match)
- `8cfb2140215f03a1c61751b18842673c88b9bcd090e27825c720046f3f23421b` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧱️fixed/🎛️command/🧪️tests/🟦️.ts` (checkpoint30 match)
- `d08ecab3bd5696903b698743a9af5303d8e3560ab2e772c52e25165b9a41b75f` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/🎛️command/🟦️.ts` (checkpoint30 match)
- `fc9fde6d2210ce57ecb7b77a91b9659a44af3824fb2390d4f8747f984fbc0c84` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/♻️retirement/📋️contract/🟦️.ts` (checkpoint30 match)
- `4cd14475e6234196948ac5d25f90faab807f5c4b458d7a70c716096334a43330` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/♻️retirement/🧬️schema/🔣️.json` (checkpoint30 match)
- `cbcacb916c1a7a249b49e1b84c322544881ca681ed6acee41606b9ab67b87954` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/♻️retirement/💾️durability/🟦️.ts` (checkpoint30 match)
- `467ccc19439c45071bd990b81376ac1135194b5e6c073105a4d8194a29568a79` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/🟦️.ts` (checkpoint30 match)
- `f4821d91edb5e4b2d9926d3173942a83453e0343658c5846fcfb1e760fb9dd08` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/📂️mount/🟦️.ts` (checkpoint30 match)
- `f623f976e1a752bb35d5de251e40190c0690a6440612ecadfabd7a91af2ed26b` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/📂️mount/🧬️schema/🔣️.json` (checkpoint30 match)
- `7098baac3a190f61ec9c665f12bd8928f69b1ac3ea7a0ed1fe1f8f2f148c0daa` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🧪️tests/📥️persistent/🟦️.ts` (checkpoint30 match)
- `b352958f4f427180f1d985f3f4d96667802e4dee3c3e2685b7a8af1d578421f8` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🟦️.ts` (checkpoint30 match)
- `ad675f6c626de96b6fb0548991ea99cbd6fdb3473fdb472b8cee0f936eecc0b7` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/📥️operation/🧪️tests/🟦️.ts` (checkpoint30 match)
- `7b6668d962ed0180f62fcfd638a2f4fb73f3b625387442857f15ee8aa0acdee5` — `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/typescript-artifact/📜️script.ts` (checkpoint30 match)
- `0b8600e1d9b041c19a095304e9b296a91bcd410624e1c9a3fc6e50a260c441a1` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/📥️operation/🧬️schema/🔣️.json` (checkpoint30 match)
- `14b653cb668a2c70cfb190996a725839f47ca06ac5db2c0056b6c8dc01c2104d` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌱️birth/🟦️.ts` (checkpoint30 match)
- `011d1fa4b600f74a2f76c050ad544b82d9c991119cb7d5292f0c604cb7fed972` — `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌱️birth/🧬️schema/🔣️.json` (checkpoint30 match)
- `6cbd60ac0ee163ee0bf4716a668a246abd9e9e4113ac9c0c09febd34810eeb9b` — `🧰️framework/🔨️modules/📁️filesystem/🧾️observation/🟦️.ts` (additional consumed dependency)
- `8512a4e5e8267214b2f23d9157e25e5c29bb8ac37bfa5e51dca49200249d0800` — `🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts` (additional consumed dependency)
- `81c5ebbc31bcdc2f933c2c16c43d67900aeef34a24b1fd02111c15eb221b60b3` — `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/whole-cache-retirement-execution-current.md` (report read)
- `62e33e980ef0d00b931cf6ff96fc96d729e6fe2c0f7bb7cbf0a7476aa3a8e66b` — `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/native-funded-child-transaction-implementation-current.md` (report read)
