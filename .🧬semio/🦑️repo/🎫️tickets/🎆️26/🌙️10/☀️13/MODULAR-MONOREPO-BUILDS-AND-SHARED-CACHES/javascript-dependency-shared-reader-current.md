# JavaScript Dependency Shared Reader

2026-10-16. Source checkpoint; ticket and goal remain active.

## Actual Ownership

`withJavascriptDependencyReadStorageV1(workspace, invocation, phaseControl, scope, body)` authenticates the exact carried Original phase, opens the actual funded dependency partition, and retains SH on the same workspace binding key that the existing installer holds EX. It derives membership through the separate read-only `withCurrentBunWorkspaceOwnershipV1` owner. It cannot publish membership, stage installation inputs, run Bun installation or rebind Source links.

The actual installation projection is now split: `inspectJavascriptDependencyInputsV1` captures current Source and computes the complete locked generation without filesystem writes; `stageJavascriptDependencyInputsV1` writes that captured projection inside actual storage and requires the current callback control. Rechecks also require that actual control. The original installer and export/binding publication remain EX writers and now carry their actual phase.

Before entering the consumer, the reader verifies full generation identity, exact ordered receipt rows, exact current Source links, and physical managed generation ancestry. It uses one bounded no-follow receipt descriptor with before/descriptor/after identity, exact EOF and growth checks. The actual consumer receives a genuine inner phase; its canonical drain settles while the outer reader still retains SH. Source bytes, topology custody, links and the receipt fingerprint are checked again before release. No source path, receipt or generation string becomes authority by itself.

## Bounds and Audit Repairs

The private schema-owned projection policy is4096packages,65536Source entries,8MiB per Source file,64MiB aggregate captured Source,4MiB receipt and64KiB read chunks. Known extents are checked before allocation; Source and receipt growth are refused with descriptor/path checks. Public schema DATA cannot change private copied primitive limits. These are admission/read-memory limits, not new filesystem quotas; the existing fixed dependency partition remains the hard physical boundary.

The independent audit found that the first SH implementation called the publisher and could rewrite stale Source. That branch was replaced by the separate read-only owner, whose BOTH27/242 physical-Source tests and strict proof are in [its report](javascript-dependency-readonly-membership-current.md). The audit also found raw installer receipt reuse; both actual ready reuse and previous binding inspection now use the bounded descriptor owner. Installed package bytes are protected by cooperative writer/reader custody; this feature does not claim a new full node_modules content hash inventory.

## Evidence

| Evidence | Result | Bytes | SHA-256 |
| --- | --- | ---: | --- |
| [dependency-reader-red1-prototype.log](🗑️generated/dependency-reader-red1-prototype.log) | 0 pass, 2 fail, 3 assert | 4419 | `87fb571312f1ee03dfa163733749f14fdd2f317d53d92ffc3b589caedab030c5` |
| [dependency-reader-red2-prototype.log](🗑️generated/dependency-reader-red2-prototype.log) | 2 pass, 1 fail, 26 assert | 6086 | `fbfd893ee957057f9db7fe1a6f2f9f2d5d86db5f10a57150866787a53cba77c4` |
| [dependency-reader-red3-prototype.log](🗑️generated/dependency-reader-red3-prototype.log) | 3 pass, 1 fail, 31 assert | 7865 | `c70cf27dd7875254f00655738e27ad70272d0c4662b041347f67073385ea3390` |
| [dependency-reader-red4-prototype.log](🗑️generated/dependency-reader-red4-prototype.log) | 3 pass, 1 fail, 28 assert | 6497 | `0ef19e13bbf1216796fc1724cef4be4eea8cbd94e09f3a6b7306cc8eb2505bb7` |
| [dependency-reader-red5-prototype.log](🗑️generated/dependency-reader-red5-prototype.log) | 4 pass, 1 fail, 35 assert | 3660 | `75372a2ce0e34473b3fb61fb912179cab30170d70de5a28d5d3f8552ef36d982` |
| [dependency-reader-green6-prototype.log](🗑️generated/dependency-reader-green6-prototype.log) | 5 pass, 0 fail, 36 assert | 1380 | `89a7ba8baba6fefb03ed549c731d85c863c8544e6ae3060594bd2ffc0c6ecc61` |
| [dependency-reader-green6-release.log](🗑️generated/dependency-reader-green6-release.log) | 5 pass, 0 fail, 36 assert | 1378 | `a15eb2f041941eda1d971ec4bda5edad6a26fb94f3e652a14f5dfcb351a210e8` |
| [dependency-reader-types4-prototype.log](🗑️generated/dependency-reader-types4-prototype.log) | strict exit0, empty output | 0 | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| [dependency-reader-types4-release.log](🗑️generated/dependency-reader-types4-release.log) | strict exit0, empty output | 0 | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |

The latest own cohort is BOTH5tests/36assertions; owned strict BOTH exit0. Ajv and SQLite independently check closed receipt/order DATA; the actual owner AST checks read-only projection, SH versus EX and bounded installed receipt reuse. The actual public-schema-mutation law failed before the private primitive-copy fix. No actual managed dependency image, install, SH overlap, Print compiler or worker ran in this cut. The old physical installer fixtures were migrated/strict-checked, not executed. The Print operation now selects this reader; that caller's later wider pipeline epoch is separate.

## Source Roster

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🟦️.ts` — `b0e938f08df10634c17087986bdb0071b091a7107f8b43df1c0db1352ba73e99`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧬️schema/🔣️.json` — `8ee35fbf0819ea52a7de2afdee1d5925d966ee0604f9d3f1c350498c77f88693`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧬️schema/📥️install.json` — `92d3d0e961c747c80bbfb6c8ce04d5fe0613343c1435ff0bddd0c499725206f7`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧬️schema/📥️bindings.json` — `217a34befe2f87c6fd629e32e9784933346ade52c7d4f05776cbb0d3faf4899c`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧪️tests/👓️reading/🟦️.ts` — `5ed51880683a365bd01a88d85c25801b4e6d113f13cc5512cb3a3248ab385765`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧪️tests/👓️reading/🔣️.json` — `6b9fd8a776a06755be5352bf3cd57333ca3eb7df676f1f9dbaccccbf3df08982`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🟦️.ts` — `8f702f78eb68a7f8499bbd079ba4959816e4a1ac384dfa86952128e42ee1fb07`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧪️tests/📥️runtime/🟦️.ts` — `4da329d6d3342256ac766bf83010ba84f286827f8a27a7aa83193b4534f51931`
