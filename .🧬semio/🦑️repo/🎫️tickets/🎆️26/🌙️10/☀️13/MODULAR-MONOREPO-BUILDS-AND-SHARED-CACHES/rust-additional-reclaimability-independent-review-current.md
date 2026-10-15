# Additional Rust Reclaimability Independent Review

Read-only Source and shallow filesystem inspection, 2026-10-15. No Originals, gates, Native commands, mutation, SQLite opens or retirement. Root supplied compact preview 261 units/1.551 GB logical,260 eligible/1.537 GB planned,14.65 MB protected; no physical savings inferred. Exact API findings sent to Nx.

Library paths below are relative to `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/`. Canonical observed Rust root is `.🧬semio/🦑️repo/⚡️cache/rust/fixed/m`.

## Best existing seam: exact old Cargo build units

`⚡️caching/🧹️pruning/🏪️budget/🟦️.ts` `retireInactiveCargoStoreV1` (32–47) already accepts an explicit buildDirectory plus relatedDirectories. Its current CLI (53–54) selects compact mode layout only, but the existing function can operate on an independently authenticated exact old `cargo/prototype/build` or `cargo/release/build` without executing old compiler layout or creating a legacy adapter.

Required actual custody is already concrete: retained Rust enforcing image and global compiler writer, then cache-prune and sorted Cargo store EX (`🧹️pruning/🟦️.ts` 40–43), process refusal, exact physical `.cargo-build-lock` handles, complete descriptor exclusion, scan recensus, and bounded unit deletions. The function scans recognized Cargo build/incremental units, not whole old directories. Before using it on old data, prove those directories belong to the real original compiler store and that old participants use the same exact Cargo store/physical locking identity; raw directory names or ledger owner alone do not prove that. The existing compiler-{mode}.json and storage-events journal bind a persistent compiler allocation, not an arbitrary subtree content ownership receipt.

Shallow observation confirms both old roots exist with ordinary profile/build metadata plus oracles/preparation/tmp/units/provenance directories. Only recognized units passing existing scanner/custody are eligible; unrelated children remain charged. This can extend the candidate inventory beyond compact1.537 GB, but no amount of genuinely reclaimable or physically freed space was measured here. A bounded old-data maintenance caller is the smallest actionable seam, preserving existing refusal rather than whole-root removal.

## Nx ownership is split across images and generations

Current `⚡️caching/💾️storage/🔒️enforcement/🔌️nx/🟦️.ts` 48–50 puts artifacts/native/tmp in Rust16, but graph workspace data in separately funded **graph** storage. `withRepositoryNxStorageV1` (63–65) retains both actual images plus namespace loans. Actual cache operation wrapper at 69–74 takes `nx:fixed-cache:<rust root>` SH/EX and per-output custody, retaining images and namespace. The installed Nx patch invokes that wrapper for task result reads, publication and restore (`🩹️patches/nx@23.2.0.patch` 597/604/1171/1179/1190).

The old Rust root currently has `nx/graphs` (plural), not `nx/graph`; shallow sample toolkit directory was empty. Current graph namespace retirement owner `🔌️nx/📦️graph/🟦️.ts` 56–65 authenticates `.namespace.json`, canonical namespace coordinates and bounded inventory under its actual graph-store allocation; it uses namespace EX with timeout zero before deletion. That owner cannot be pointed at old Rust-resident graph residue by changing a root argument: its allocation/image/root custody is specific. Old namespace receipts and the old publishing protocol must be authenticated first; current graph metadata-as-authority is insufficient. Even empty parents cannot justify deleting unknown descendants wholesale.

## Rust-resident Nx blockers

- `nx/native` shallow contents are version/hash-named `.node` copies. Current root naming and SDK-loaded native-module lifetimes do not provide per-file immutable retirement ownership or a public released-copy cleanup API. An idle PID census cannot establish no loaded mapped module/old native handle. Preserve until exact original SDK/copy receipt plus actual reader exclusion is established.
- `nx/tmp` shallow contents include `.hm` files. No per-temporary ownership receipt or existing authenticated directory-sweep owner was found in the reviewed Source. Current root placement and fixed-cache EX identify a general cache operation boundary, not every old opener's participation. Unknown transient material remains charged.
- `nx/artifacts` contains Nx native DB/lock plus numeric task directories; a shallow sample task directory contained `.cache`. Task hash/output metadata is not by itself a physical allocation retirement receipt. Current fixed-cache/output guards are concrete coordination seams, but no reviewed public API authenticates and retires a whole old cache namespace or its SQLite rows/files. Do not unlink task directories behind the cache DB or manufacture ledger retirement from a task hash.

`⚡️caching/📦️artifacts/🔒️bounded/🟦️.ts` issues real artifact write reservations and commits them; it does not supply a generic existing artifact sweep. `recoverNativeStorageReservationV1` requires an exact owned allocation and actual retirement port; it cannot convert arbitrary Nx files into owned disposable material. Unit-specific `.charge.json` retirement exists in `🦀️cargo/📦️units/📜️script.ts` for replacing that exact output under unit EX, not for sweeping Nx artifacts. The current image native port explicitly refuses generic retirement without original writer/reader custody.

## Practical next step

Inspect/preview recognized units in each exact authenticated old Cargo build root through the existing maintenance API and its unchanged custody, under the real maintenance writer. This is the only concrete additional old-data retirement seam established here. Keep its deletion budget, physical-lock and descriptor refusals; collect actual statfs only under retained writer when executing later. If authenticated candidates still do not fund the initial 4 GiB floor, report unmet physical capacity. Nx roots need a separate exact original publication/reader retirement proof before they can fund admission; their multi-GB totals alone do not authorize deletion.
