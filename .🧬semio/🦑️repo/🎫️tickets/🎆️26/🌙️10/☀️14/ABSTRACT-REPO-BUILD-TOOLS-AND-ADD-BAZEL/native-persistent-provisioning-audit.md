# Accepted Persistent Native Provisioning Port Audit

This was a bounded read-only Source audit. No image, store, lease or platform operation was opened; no production Source changed. Core owns the actual retention identity fixture. Its first physical attempt stopped before the identity regression: absent private lease image directory was refused by the current macOS borrowed opener. Core reports the exact private workspace was removed, all stores closed, and no image was created. That attempt does not qualify a persistent identity correction.

## Current Executable Boundary

- [Platform dispatcher](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/🟦️.ts:14) exports openNativePlatformStorageV1 and retireNativePlatformStorageV1. There is no accepted persistent creation, activation or ownership-transfer entry in this dispatcher.
- [macOS borrowed opener](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🟦️.ts:26) admits only an existing correctly receipted, already mounted image. Line31 refuses an absent directory before fresh effects; its EX branch refuses inactive activation rather than attaching. Creating an empty directory cannot repair this boundary: a valid receipt/backing/mount is still required.
- [Temporary macOS creator](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🟦️.ts:74) accepts physical cleanup before effects, retains canonical creator EX, runs accepted create/activate/inspect, and always drains then normally deactivates/removes the created image. Its exported withNativeImageStorageV1 at84 returns only a scoped body without close. It cannot be substituted for a persistent image that must later admit independent SH and EX users. Holding that creator EX while attempting a separate open would contend with itself.
- [Actual cleanup command owner](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/🎛️command/🟦️.ts:30) privately issues only a genuine absent-directory macOS creator. Its action(scope) at36 always supplies persistent:false. It offers no executable receiver acceptance or persistent handoff issuer, and final settle at49 calls removeSettledNativeImageV1.
- [Cleanup data model](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/📋️state/🟦️.ts:56) describes accept-persistent-handoff and persistent-transfer, and schema describes fresh-persistent/published-persistent scenarios. These are explicitly nonauthoritative evidence reductions; no current physical command path sets the actual owner to persistent. A passing model schedule cannot create the missing receiver.
- [Repository partition opener](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts:20) opens the lease partition through platform.open; openRepositoryCachePartitionV1 at34 opens leases first, then the requested image. It has no preceding accepted persistent provision step. Consequently a new macOS repository/private workspace reaches the absent-directory refusal before its requested tools partition can open. Existing mounted main stores can still reopen; that says nothing about fresh zero-touch provisioning.

## Cross-platform Status

[Generic fixed opener](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧱️fixed/🟦️.ts:37) still detects an absent receipt, releases SH, acquires canonical EX, calls platform.prepare, writes the receipt, releases EX and reacquires SH. Linux and Windows map to that generic opener. Their prepare callbacks create a finite loop-backed ext4 image or fixed VHD/NTFS after actual capability and host capacity admission. This is Source behavior, not a physical Linux/Windows qualification. Those paths do not use the macOS accepted temporary creator or its persistent transfer model. The dispatcher therefore currently presents asymmetric fresh-image semantics across hosts.

## Confirmed Gap and Correct Ownership Shape

No existing callable accepted persistent fresh macOS receiver/transfer port was found in the dispatcher, platform contract, macOS creator, command owner, or their production call sites. This is a zero-touch integration gap, not an invitation to bypass the borrowed-open guard. It also applies to activating a receipted but currently inactive macOS backing, where the opener now explicitly requires an accepted activation owner.

A complete future owner must accept finite Original cleanup and the canonical image resource before creation/activation, privately bind the exact physical directory/backing/mapping and finite envelope, then explicitly enroll a persistent receiver before releasing creator custody. Only a successful authenticated receiver acceptance may commit persistent ownership and permit independent later readers/writers. Failure or cancellation before that point must drain accepted work and execute the existing normal bounded physical cleanup, preserving primary failures including undefined. Caller-written JSON or a data-model persistent flag cannot perform the handoff. Receipt publication and EX-to-reader transition need an actual ownership protocol, not a raw mkdir or a retained creator EX.

The repository partition opener must invoke that canonical persistent provisioning/activation operation for fresh/inactive state before it uses the borrowed opener. Private fixtures should use the same operation so they test genuine independent image SH/EX ownership. Existing mounted-image borrowed callers retain their present scope. No quota, floor, path, deadline or platform capability relaxation is required by this finding.

The next concrete proof should begin with the current absent-directory runtime refusal as a setup boundary, then separately test actual creation plus persistent handoff, independent SH/EX contention, cancellation/failed handoff cleanup, changed physical identity refusal, and inactive-image activation under accepted custody. Language-neutral handoff schedules and third-party schema/SQLite lease oracles complement, but do not replace, actual macOS mounted-volume runtime proof. Production changes remain held pending Root assignment; this audit does not implement or qualify that future port.

## Captured Source Bytes

- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/🟦️.ts`: `febee7be7c5af3b2d78ea3f6fb4e001d3f2bc5c366495fb5e19e2f19fed5bf1e`.
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts`: `1093475635b685b2d9ddc5f4f04760146903d32994b31754d1b5d3962bf9d331`.
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🟦️.ts`: `817d3e5a17e07e8cb8e19a7c18d536537e56ae91a819c0f84b9759f5331daeea`.
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧱️fixed/🟦️.ts`: `c0d1abb2f8369daeb0a4f4b314cfb1c680e799fd3aca616b4832fc00cdea79f3`.
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🐧️image/🟦️.ts`: `2e41f95868b26b0c339c7191966444589d4c6000242feb6155517b5b3e691dce`.
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🪟️image/🟦️.ts`: `7f204ec604edc50ffb7975b6a6c1e8e139fe4525e4841c6fdcc6f1429804d383`.
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/🎛️command/🟦️.ts`: `85f2fea266d7b65abc7641a7262901e762682bbe551c59acb0253d3ce3560f64`.
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/📋️state/🟦️.ts`: `8b730f45cc0d1b3c55d1cc6d4cd83a5e57b6216f1f6a4d9adb100c40727a660d`.
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/🧬️schema/🔣️.json`: `b63bfae095e941313218749fc3a7712bc506b8517919d0d034b534647ecc06c2`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts`: `24a4eb5100e9ad2a5b1fa0bdbfcb28ecce54ad25e63de86ca1deb0314950a402`.

## Later Implementation Status

The missing-port finding above describes the audited earlier Source. Root subsequently selected a single canonical opener with a concrete privately owned persistent receiver rather than a separate prepare API. Current implementation and exact focused/physical evidence chronology are retained in persistent-native-implementation.md; the earlier absence finding is not a claim about the later current module.
