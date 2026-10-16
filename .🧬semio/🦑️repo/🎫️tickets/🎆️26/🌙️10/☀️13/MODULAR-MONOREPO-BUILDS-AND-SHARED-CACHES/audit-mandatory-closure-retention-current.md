# Mandatory Graph Closure Retention Audit

2026-10-16. Read-only Source exploration. No Native/SDK/compiler/build/test/network/lease/image/maintenance operation ran. This is a dependency and design report, not deletion authorization or physical qualification. Selected SDK reader seam is corrected at Source level per successor reader audit.

## Actual Dependency Map

Paths below are relative to the library root in the Source roster.

| Producer / retained object | Actual reference / consumer |
| --- | --- |
| inputs/🟨️.mjs lines58–69 publishes exact `{version:1,manifest}` bytes in build-lock-closures/hash.json | cargoLockInput returns SEMIO_CARGO_LOCK_hash env input. It identifies a manifest projection, not the current lock digest. |
| inputs/🟨️.mjs lines72–99 publishes sorted exact source files plus hashing helpers in build-input-closures/hash.json | compactSourceInputs returns SEMIO_SOURCE_CLOSURE_hash env input. Changed source bytes generally reuse inventory identity; changed membership creates a different immutable record. |
| graph/inference lines1810–1811 stores per-project normalized node results in inference/[contexts or selections]/…/np/hash.json; lines2029 outer node result in inference/emoji-project-json.json | observation lines485–518 can return serialized warm result directly after source observations match. Such results can contain compact env tokens, bypassing compactSourceInputs/cargoLockInput publication entirely. Removing closure files while keeping these results is unsound. |
| observation lines533–550 publishes dependency receipt, publication and p-PID producer publication with inference/header/source observations | These records attest warm inference results and may preserve or validate referring state. Full reset must include them, not just optional leaves recognized by the current quota regex. |
| build graph portableInputs lines156–193 walks exact token objects and reads corresponding source/lock record, hashes bytes, expands to ordinary workspace paths | Missing record rejects; it is not a successful cache hit. Graph warm path lines280–286 can reuse canonical/selected JSON+authority after source checks. Source observations may include closure record reads and must be invalidated with the records. |
| graph discovery line239 applies portableInputs before readBuildGraphV1; publication lines304–305 serializes resulting graph | Normal neutral canonical/selected graphs already contain expanded portable paths, so their task input arrays do not inherently require the compact records. Their observation authority and any other compact producer caches still couple them to records. |
| budget/graph artifact lines36–48 serializes readBuildGraphV1(graph), content-addresses snapshots/hash.json and pins Graph image+file SH | Normal snapshot bytes derive from expanded neutral graph. Reuse path graph lines266–270 verifies digest/selection and expands any encountered compact tokens. A reset can discard unheld snapshots to force new graph production; it must not invalidate a live SEMIO_BUILD_GRAPH_PATH capsule. |
| digest helper lines29–63 reads source/lock records synchronously, computes source bytes / selectedCargoLockDigest; lines66–74 rechecks stored source receipt | Patched SDK native task hasher lines1218–1222 calls frozen sourceClosureTaskEnvironments, stores semioSourceInputs; promotion/remote restoration lines1281,1301,1337–1338 recheck via sourceClosureReceiptHolds. A running task may need records again at completion even after initial hashing. |
| selected scope reads source record through bounded descriptor | Actual accepted SDK scope/capture now retain namespace via private reader ledger; this covers those tasks, not every digest helper entry automatically. |

## Smallest Clean Coupled Reset

A schema-first whole-domain cold reset is sufficient in principle. It does not require retaining every historical reference forever. Its closed reset plan must cover all compact-token-bearing inference results, authority/receipt/producer publications, mutable selected/context namespaces, canonical+selected graph pairs/staging, snapshots, and installed Nx graph/task-plan publications that can resurrect compact tokens. Under actual complete reader exclusion, invalidate every referring publication first, then remove validated immutable closure records, then reconcile the optional ledger using the actual owner API and fresh census. Unknown leaves or malformed/foreign objects fail closed. Preserve actual allocator/image/Native journal/bootstrap/guard and lease identities; do not reset the image or Native databases by recursive subtree assumption.

Ordinary warm inference after reset must see a declared miss and rerun the authentic producers, recreating records before publishing references. Missing inventory under an otherwise accepted warm result remains an error. A versioned private reset epoch could explicitly make old caches miss, but the epoch must be part of every referring publication's admission contract; introducing an unchecked ambient generation is insufficient. Reset should operate before current operation pins its Graph store and should return to the same Original and inherited phase for recomputation, without a second Original or reconstructed time.

A closed plan needs immutable primitive limits for inventory entries, file/aggregate/physical allocation bytes, path depth/length, parser memory and operation progress/cancellation. Adopt independent neutral schema/oracle laws for full-domain classification, warm-reference invalidation, interruption after invalidation before deletion, exact current live pin exclusion, and successor cold recomputation. No runtime proof is supplied here.

## Current Maintenance Does Not Implement This

inputs/storage optional retirement inventories only quota.optionalPath leaves and removes them under image.withExclusiveMaintenance. Its allowed paths exclude closure records and dependency producer/publication/receipt files. Existing graph headroom lines40–46 authenticates snapshots and current canonical/authority only, omitting selected publications, inference token producers and closure records. Neither is a coupled mandatory-retention policy. Whole inference deletion must also account for open record stores and ledger accounting; the dataset-retire method only retires a specific retained dataset, and does not collect global immutable closures.

## Live Reader and SDK Blockers

Whole outer Graph image EX is sufficient only when every actual process/worker/daemon reader, source receipt rechecker and accepted descendant retains its genuine Graph reader through OS close and callback completion. The corrected selected SDK task ledger meets its local Source ordering requirement; it does not enroll direct raw exported sourceClosureTaskEnvironments/sourceClosureReceiptHolds calls on its own. These synchronous helper readers need enclosing real image custody, including the full task interval between initial hash and later promotion, or an explicit retained reference set for that interval.

The patch visibly wraps SDK main and run-executor in the frozen bundle's withOriginalNxGraphNativeStorageV1. Exact daemon/hash worker paths and every promotion consumer must be traced to their real outer-image owner before reset can be implemented safely. SourceRoot and frozen bundle have separate private issuer/ALS universes: use actual owners within each, with physical shared Graph exclusion providing cross-process coordination; environment/path equality cannot register another bundle's authority. The bootstrap SourceRoot port-factory correction avoids one crossing but does not prove complete global enrollment. Long-lived daemon in-memory graphs and SDK JSON graph caches can revive compact tokens after filesystem inference reset unless the actual validated recomputation/invalidation boundary also covers them.

Thus full reset is a viable clean design only after closed inventory ownership and complete global consumer/daemon invalidation are established. The current Source does not yet establish those prerequisites. Per-key reachability retention is an alternative but requires a complete reference index over the same producer/reader set; age, filename hash and observed unused status cannot replace it.

## Exact Source Roster

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🟨️.mjs` — SHA-256 `589f91e0de595649b830c462ab562988c26f0fad9ef97310319f42367223ca47`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🧮️digest/🟨️.mjs` — SHA-256 `1c25559fb3dc685a1289bd2e5cee8df264b0c91dfadb1184f20d0624404400c9`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🟨️.mjs` — SHA-256 `bc0b39a49a69c8a56bfc1676cc33451cc8645fe1e4a7dff4bc4ed5bb7a6a0bda`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🟨️.mjs` — SHA-256 `425fcec4e001631bf977ca6e30c992c2c5ee02441b645fb6282f7211e9db884a`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🟦️.ts` — SHA-256 `f23de014039ae32829f124cd4c40e09db4aa548608a99416813219e2c8a9994c`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🟨️.mjs` — SHA-256 `d98d6b74252b681e35d64662a4fa726fe30307bdf0942d7751d22ac8215a7a8f`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/🟦️.ts` — SHA-256 `a84b4c7c9aa6ef8e798749b9b2ce0e83749a6a65ba4092664bc5148b1ab54cb4`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🟦️.ts` — SHA-256 `8a8ad95d8ea05f94ecdf49f58997cb9ad37bb9d708f6402c8981142056526ad9`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🩹️patches/nx@23.2.0.patch` — SHA-256 `cd4974307311ba7fc79c67f954348f91f4331998533ab757c0e4cd4e7767f90e`
