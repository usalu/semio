# SDK Identity and Vendor Reuse Audit

This is a read-only Source and retained-receipt design audit. No SDK/store opener, Tools custody, graph writer, tests, cache adoption, image operation or Source edit occurred. SDK Source remains held. Proposed boundaries below are not implemented reuse and have no measured performance benefit.

## Actual Attempt

Root's retained diagnostic19472 completed Source-index capture22.688s, derivative clone61.054s over11,701 entries,37 replacement files/622,059 output bytes,152 captured authorizer inputs/353,734 bytes and index verification11.216s before the genuine Original120 failed. No Node preflight, new publication or activation is recorded. These completed reported regions are not a complete or necessarily disjoint elapsed-time partition. Equal counts or byte totals do not prove equal content. Core owns actual CPU attribution; this audit does not infer IO wait from wall minus CPU.

## Six Exact Install Images

[captureNxToolingInputsV1](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/📜️script.ts:119) captures the inputs; [captureNxSdkPublicationInputsV1](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/🎟️capacity/📥️physical/🟦️.ts:15) privately retains their detached buffers before identity, presence and publication. [nxToolingInstallIdentityV1](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/🪪️identity/🟦️.ts:3) hashes the install-v3 tag, platform, architecture and libc, then each path and exact byte image in map iteration order with NUL separators. It does not hash counts or only installer implementation text. The reader reconstructs the same insertion order.

There is currently one authored patch contribution, nx@23.2.0, giving these six images:

| Image | Identity changes when | Current consequence |
| --- | --- | --- |
| package.json | Recipe/dependency/Bun pins or content-addressed patch filename map changes | New final generation; dependency changes also select another pristine base |
| bun.lock | Exact normalized lock image or patch references change | New final generation; dependency lock changes also select another pristine base |
| patches/raw-SHA.patch | Exact authored patch bytes change | New final generation and newly authenticated patched Nx outputs |
| .semio-authorizer-source.json | Runtime entry/mode or any captured Source path/SHA changes | New final generation and current bundle/receipt/preflight; vendor bytes need not have changed |
| .semio-authorizer-compiler.json | Release mode, Bun pin or compiler Source census changes | New final generation even if compilation could produce identical output; current producer provenance is bound |
| .semio-sdk-material.json | Pristine snapshot, sealed Source index or owned-packages material changes | New final generation referencing exactly authenticated base/inodes |

[nxToolingInstallFilesV1](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/📜️script.ts:105) names the raw patch by its SHA and writes that path into both package and lock. That input hash differs from the per-output patched-file hashes later recorded by toolPatchReceipt. The compiled semio-authorizers.mjs output SHA is recorded in the tooling marker; it does not replace the full runtime Source manifest identity.

Reader Source is not a separate seventh install image. It changes install identity when reachable bytes affect the runtime or compiler census. The actual retained before/after snapshots demonstrate that case:

| Census | Before files | After files | Changed existing hashes | Added / removed |
| --- | ---: | ---: | ---: | ---: |
| reader | 100 | 100 | 11 | 0 / 0 |
| runtime | 171 | 173 | 13 | 2 / 0 |
| compiler | 57 | 57 | 10 | 0 / 0 |

Before: tooling-closures-keysets-current.json SHA c25885413e0d746f72bb3bad40583334a8af502954956d8a10e8d0144c36d539. After: tooling-closures-current-live-cpu.json SHA76edc9420c76216c040da1095c40895b53bdf9e08b218b2e4fe9916f183d1dfe. Runtime adds the actual Nx graph operation owner and its schema. Seven expected-retirement runtime/schema files have changed hashes in these relevant censuses. These receipts do not establish old/new raw patch image or installed vendor byte equality; no such equality is claimed.

## Existing Reuse

[nxDependencyInstallFilesV1](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/📥️source/📐️projection/🟦️.ts:4) removes patchedDependencies and projects package/dependency/Bun inputs. [nxSdkSnapshotIdentityV1](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/📥️snapshot/🟦️.ts:17) keys the pristine tools/sdk/digest by the platform tuple and sorted exact dependency images, independently of framework authorizer revisions. [ensureNxPristineSdkV1](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/📥️source/🧊️pristine/🟦️.ts:36) authenticates and reuses that existing base; an authorizer-only change does not by itself reinstall it. Its absent-base fallback may derive a pristine base from an authenticated current snapshot plus registry transform; that is not a separately reusable patched-vendor layer. Prepared Bun, downloads and immutable binary objects are other existing reuse.

[nxSdkDerivativeDispositionV1](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/📥️source/🔗️derivative/🟦️.ts:29) marks every regular node_modules/nx entry as copy and other dependency Source as shared. copyNxSdkDerivativeV1 constructs the complete new namespace, copies Nx using COPYFILE_FICLONE|COPYFILE_EXCL and independent nlink1 inodes, and hardlinks other dependency Source to the exact readonly pristine inodes. Thus the observed clone does not mean every dependency byte was independently copied. No new physical-savings estimate follows from this Source inspection.

If all six exact current images and the environment remain equal, the existing present/installed branch reuses the exact final generation after current reader, patch, binary and Node preflight checks. This is the only existing complete patched-generation reuse found. There is no independently keyed reusable patched-vendor layer.

## Why New Framework Bytes Currently Repeat Nx Copy

A changed runtime/compiler manifest genuinely selects a new final six-image identity. It does not physically require different vendor bytes. The present material protocol nevertheless constructs each final key from pristine. Its current enforcing guards require that route:

- [withNxSdkDerivativeProjectionV1](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/📥️source/🔗️derivative/🟦️.ts:38) retains the exact pristine SH, pinned dependency images and private captured projection. A held SH alone is not a cached proof of physical facts across arbitrary awaits.
- [capture / seal / verify](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/📥️source/🔗️derivative/🔏️index/🟦️.ts:47) binds finite membership, content, modes and contained links. Only actual projection/writer custody may seal readonly base files; unknown links, appeared index or changed bytes refuse.
- [publication owner](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/🎟️capacity/📥️physical/🟦️.ts) admits the real private quote, exact store/control, registry EX and same projection. Empty stage and declared captured-input membership are checked. A caller-supplied quote or retained stage cannot authorize reuse.
- [verifyNxSdkDerivativeNamespaceV1](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/📥️source/🔗️derivative/📥️reader/🟦️.ts:30) requires non-Nx files to be exact known readonly dev/ino with nlink>=2, while Nx files must be independent nlink1 regular files. Cross-generation Nx hardlinks would violate that current rule.
- [provisionNxToolsOwnedV1](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/📜️script.ts:145) requires patch inputs/replacements to be nlink1, then unlinks and writes replacements before verifying actual per-file patch receipts. A shared patched inode cannot enter this mutating stage.
- [withNxSdkDerivativeReaderV1](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/📥️source/🔗️derivative/📥️reader/🟦️.ts:17) authenticates final six inputs and exact base/index, holds the referenced base SH through the callback, verifies namespace inodes and runs final current checks. [nxAuthorizerReceiptHoldsV1](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/📤️receipt/🟦️.ts) binds exact Source manifest and bundle output.
- The patched loadSemioGraphAuthorizersV1 starts from the actual loaded Nx module pathname, requires the final directory basename to equal its full tooling digest, verifies actual patched file hashes and captured Source/bundle receipts, then imports that exact companion. Similar patch counts or an older compiled bundle cannot authorize the current generation.

These distinguish new final namespace/companion requirements from the current policy of copying all Nx files. No input, inode, readonly, current-Source or receipt guard may be bypassed to reuse an old final key.

## Future Boundary, Design Only

A clean future division could keep the existing pristine base, add an authenticated immutable patched-vendor layer, and publish small current framework companions against it. The vendor key would bind exact pristine content/index, exact patch images, captured patch-producer/schema contract and platform tuple. Runtime/compiler changes would still require a new final identity, current companion bundle and preflight. This does not reuse stale framework authority.

Such a layer needs a new explicit material schema and genuine publication/reader/retirement ownership. A readonly shared Nx layer needs exact known inode/hash/mode membership, layer SH through every dependent user, finite allocation and dependency-aware EX retirement. The mutable nlink1 patch stage must finish before layer sealing; final generations could link only sealed outputs and perform no further in-place patching. Actual module pathname and companion lookup must remain in the final generation namespace. No symlink redirection, environment-only grant, count/version substitute, protocol exception or quota increase is proposed.

An alternative authenticated patched-vendor copy/reflink source could retain nlink1 final inodes. It still constructs the namespace and performs current physical checks; it may avoid repeated patch application but does not prove any reduction of the measured61.054s.

Necessary evidence before any implementation acceptance: actual same-vendor/new-framework identity separation; changed-patch invalidation; late helper/schema refusal; readonly/foreign-link rejection; held-reader retirement exclusion; primary/cancellation/awaited progress; exact peak/funding and original-path/module coherence. Any performance claim requires physical end-to-end warm measurement under unchanged Original and quotas. No implementation or runtime reuse is authorized by this audit.

## Current Source Hashes

- [📜️script.ts](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/📜️script.ts): `0fe0d04e28bdd116beaf48ac84021ece8ecc3b945962fcbc3084b50ceb42278d`
- [📦️sdk/🪪️identity/🟦️.ts](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/🪪️identity/🟦️.ts): `ab11f522acdea8a0d182811e968fa7a1b27c047feed2340ef50a9dd38b0f5c96`
- [📦️sdk/📥️source/📐️projection/🟦️.ts](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/📥️source/📐️projection/🟦️.ts): `5584f09366746be79a61d63f7fad05436a2e579a6a43cba8b3e4a125cf497a39`
- [📦️sdk/📥️snapshot/🟦️.ts](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/📥️snapshot/🟦️.ts): `6ca00e5a17e07f81963f886c71d00ae16dbd1bfb16361bd8655cc85f5e89068c`
- [📦️sdk/📥️source/🧊️pristine/🟦️.ts](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/📥️source/🧊️pristine/🟦️.ts): `f7836354ab0e92aeb337261a5f5c76cc5cac5cbf398d91d00f763e625cf277d6`
- [📦️sdk/📥️source/🔗️derivative/🟦️.ts](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/📥️source/🔗️derivative/🟦️.ts): `c8b9fee47068209cb0a48179c5cafd654281bb701f7b110bac9999364755b0a4`
- [📦️sdk/📥️source/🔗️derivative/📥️reader/🟦️.ts](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/📥️source/🔗️derivative/📥️reader/🟦️.ts): `441ccfb3c54685b8c3d404decd050d95833a0a235946c3fc7c1ecabfde30a456`
- [📦️sdk/📥️source/🔗️derivative/🔏️index/🟦️.ts](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/📥️source/🔗️derivative/🔏️index/🟦️.ts): `01a290658bd36f54b1084dd4b04a61e49b86e73d6531fe9c0489ebd731440e86`
- [📦️sdk/🎟️capacity/📥️physical/🟦️.ts](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/🎟️capacity/📥️physical/🟦️.ts): `bb5b2e5d11b48c0bb8dd416502f30f436f11098fdbc2fa670fdcc91a1c929b82`
- [🔏️authorizers/📤️receipt/🟦️.ts](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/📤️receipt/🟦️.ts): `071fad590764b3cc91beb431b0cc73c54835a8384758c2dac46cda255924755b`


- [🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🩹️patches/nx@23.2.0.patch](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🩹️patches/nx@23.2.0.patch): `cd4974307311ba7fc79c67f954348f91f4331998533ab757c0e4cd4e7767f90e`; raw patch 140647 bytes, 37 diff headers. These current Source facts do not prove equality with an earlier input or the622,059 installed replacement output bytes.
